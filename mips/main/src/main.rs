#![no_std]
#![no_main]
// #![feature(allocator_api)]

// extern crate alloc;

use arch::{print_byte, print_cstr, print_i32};
use io::midi::{Insturment, Pitch};
use midly::{EventIter, Format, Header, Timing, TrackIter};
pub use rlib::*;

struct TimingState {
    header_info: Timing,
    tempo_us_per_qn: u32,
    sig_top: u8,
    sig_bottom: u8,
    n32_per_midi_qn: u8,
}

impl TimingState {
    pub fn get_upt(&self) -> u32 {
        match self.header_info {
            midly::Timing::Metrical(t_per_qn) => self.tempo_us_per_qn / t_per_qn.as_int() as u32,
            midly::Timing::Timecode(fps, subpf) => 1_000_000 / fps.as_int() as u32 / subpf as u32,
        }
    }
}

struct MidiState<'a> {
    tracks: [Option<TrackState<'a>>; 16],
    format: Format,
    timing: TimingState,
}

struct TrackState<'a> {
    iter: EventIter<'a>,
    channels: [ChannelState; 16],

    tick: u32,
    us: u32,
    peek: Option<PlayEvent>,
    track_idx: u8,
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
struct PlayEvent {
    pitch: Pitch,
    start_ms: u32,
    duration_ms: u32,
    insturment: Insturment,
    volume: u8,
    track_idx: u8,
}

#[derive(Clone, Copy)]
struct ChannelState {
    instrument: Insturment,
}

impl<'a> TrackState<'a> {
    pub fn new(track_idx: u8, track: EventIter<'a>) -> Self {
        Self {
            iter: track,
            track_idx,

            tick: 0,
            us: 0,

            channels: [ChannelState {
                instrument: if track_idx == 1 {
                    Insturment::BrightAcousticPiano
                } else {
                    Insturment::AcousticGrandPiano
                },
            }; 16],
            peek: None,
        }
    }

    pub fn next(&mut self, timing: &mut TimingState) -> Option<PlayEvent> {
        if self.peek.is_some() {
            return self.peek.take();
        }
        use midly::MidiMessage as MM;
        use midly::TrackEventKind as TEK;
        loop {
            let event = self.iter.next()?.ok()?;
            self.tick += event.delta.as_int();
            let upd = timing.get_upt();
            self.us = self.tick * upd;

            match event.kind {
                TEK::Midi { channel, message } => {
                    let chan = &mut self.channels[channel.as_int() as usize];
                    match message {
                        MM::NoteOn { key, vel } => {
                            let mut len = 0;
                            for event in self.iter.clone().flatten() {
                                len += event.delta.as_int();
                                match event.kind {
                                    TEK::Midi {
                                        channel: off_channel,
                                        message:
                                            MM::NoteOff { key: off_key, .. }
                                            | MM::NoteOn { key: off_key, .. },
                                    } if off_channel == channel && key == off_key => break,
                                    _ => {}
                                }
                            }
                            return Some(PlayEvent {
                                pitch: Pitch::raw(key.as_int()),
                                start_ms: (self.us / 1000),
                                duration_ms: len * upd / 1000,
                                insturment: chan.instrument,
                                volume: vel.as_int(),
                                track_idx: self.track_idx,
                            });
                        }
                        MM::ProgramChange { program } => {
                            chan.instrument = Insturment::from_num(program.as_int())
                                .unwrap_or(Insturment::AcousticGrandPiano)
                        }
                        MM::NoteOff { .. } => {}
                        MM::ChannelAftertouch { .. } => {}
                        MM::Aftertouch { .. } => {}
                        MM::PitchBend { .. } => {}
                        MM::Controller { .. } => {}
                    }
                }
                TEK::SysEx(_) => {}
                TEK::Escape(_) => {}
                TEK::Meta(meta_message) => {
                    match meta_message {
                        midly::MetaMessage::Tempo(temp) => timing.tempo_us_per_qn = temp.as_int(),
                        midly::MetaMessage::TimeSignature(top, bottom, _, n32_per_midi_qn) => {
                            timing.sig_top = top;
                            timing.sig_bottom = bottom;
                            timing.n32_per_midi_qn = n32_per_midi_qn;
                        }

                        // midly::MetaMessage::TrackNumber(_) => todo!(),
                        midly::MetaMessage::Text(text) => {
                            text.iter().copied().for_each(rlib::arch::print_byte)
                        }
                        // midly::MetaMessage::Copyright(_) => todo!(),
                        midly::MetaMessage::TrackName(text) => {
                            text.iter().copied().for_each(rlib::arch::print_byte)
                        }
                        // midly::MetaMessage::InstrumentName(_) => todo!(),
                        midly::MetaMessage::Lyric(text) => {
                            text.iter().copied().for_each(rlib::arch::print_byte)
                        }
                        midly::MetaMessage::Marker(text) => {
                            text.iter().copied().for_each(rlib::arch::print_byte)
                        }
                        midly::MetaMessage::CuePoint(text) => {
                            text.iter().copied().for_each(rlib::arch::print_byte)
                        }
                        midly::MetaMessage::ProgramName(text) => {
                            text.iter().copied().for_each(rlib::arch::print_byte)
                        }
                        midly::MetaMessage::DeviceName(text) => {
                            text.iter().copied().for_each(rlib::arch::print_byte)
                        }
                        midly::MetaMessage::EndOfTrack => return None,
                        // midly::MetaMessage::MidiChannel(u4) => todo!(),
                        // midly::MetaMessage::MidiPort(u7) => todo!(),
                        // midly::MetaMessage::SmpteOffset(smpte_time) => todo!(),
                        // midly::MetaMessage::SequencerSpecific(_) => todo!(),
                        // midly::MetaMessage::KeySignature(_, _) => todo!(),
                        // midly::MetaMessage::Unknown(_, _) => todo!(),
                        _ => {}
                    }
                }
            }
        }
    }

    fn peek(&mut self, timing: &mut TimingState) -> Option<&PlayEvent> {
        if self.peek.is_none() {
            self.peek = self.next(timing);
        }
        self.peek.as_ref()
    }
}

impl<'a> MidiState<'a> {
    pub fn new(header: Header, tracks: TrackIter<'a>) -> Self {
        let mut myself = Self {
            tracks: [const { None }; 16],

            format: header.format,

            timing: TimingState {
                tempo_us_per_qn: 1_000_000 / 120,
                sig_top: 4,
                sig_bottom: 2,
                n32_per_midi_qn: 32 / 4,
                header_info: header.timing,
            },
        };
        for (i, (track, place)) in tracks.flatten().zip(myself.tracks.iter_mut()).enumerate() {
            *place = Some(TrackState::new(i as u8, track))
        }
        myself
    }

    pub fn next(&mut self) -> Option<PlayEvent> {
        match self.format {
            Format::SingleTrack => self.tracks[0].as_mut()?.next(&mut self.timing),
            Format::Parallel => self
                .tracks
                .iter_mut()
                .flatten()
                .map(|v| {
                    (
                        v.peek(&mut self.timing)
                            .map(|v| v.start_ms)
                            .unwrap_or(u32::MAX),
                        v,
                    )
                })
                .min_by(|(lhs, _), (rhs, _)| lhs.cmp(rhs))?
                .1
                .next(&mut self.timing),
            Format::Sequential => {
                self.tracks.iter_mut().flatten().next().and_then(|v|v.next(&mut self.timing))
            }
        }
    }
}

impl core::iter::Iterator for MidiState<'_>{
    type Item = PlayEvent;

    fn next(&mut self) -> Option<Self::Item> {
        self.next()
    }
}

#[no_mangle]
pub fn main() {
    let mut midi = rlib::io::midi::get_midi();
    
    let path = c"/home/may/Downloads/good_ones/Never_Gonna_Give_You_Up.mid";
    let file = rlib::io::file::File::read_raw(path).ok().unwrap();
    let data = rlib::arch::sbrk(1<<20);
    let size = file.read_slice(data).ok().unwrap();
    let data = &data[..size];

    let (header, tracks) = midly::parse(data).unwrap();

    let state = MidiState::new(header, tracks);

    let start_real = rlib::arch::systime();
    for event in state {
        if event.volume == 0 {
            continue;
        }
        print_cstr(c"\npitch: ");
        print_i32(event.pitch.get_pitch() as i32);
        print_cstr(c" duration: ");
        print_i32(event.duration_ms as i32);
        print_cstr(c" volume: ");
        print_i32(event.volume as i32);
        print_cstr(c" track: ");
        print_i32(event.track_idx as i32);
        print_cstr(c" insturment: ");
        event.insturment.name().bytes().for_each(print_byte);

        rlib::arch::sleep_ms((event.start_ms as i32-(rlib::arch::systime()-start_real) as i32).max(0));
        midi.out(
            event.pitch,
            event.duration_ms,
            event.insturment,
            event.volume,
        );
    }

    rlib::arch::sleep_ms(500);
}
