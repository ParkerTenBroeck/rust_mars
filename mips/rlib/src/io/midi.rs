use core::marker::PhantomData;

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Note {
    Cs,
    Db,
    D,
    Ds,
    Eb,
    E,
    Fb,
    Es,
    F,
    Fs,
    Gb,
    G,
    Gs,
    Ab,
    A,
    As,
    Bb,
    B,
    Cb,
    Bs,
    C,
}

impl Note {
    #[inline(always)]
    pub const fn to_num(&self) -> u8 {
        match self {
            Note::Cs => 1,
            Note::Db => 1,
            Note::D => 2,
            Note::Ds => 3,
            Note::Eb => 3,
            Note::E => 4,
            Note::Fb => 4,
            Note::Es => 5,
            Note::F => 5,
            Note::Fs => 6,
            Note::Gb => 6,
            Note::G => 7,
            Note::Gs => 8,
            Note::Ab => 8,
            Note::A => 9,
            Note::As => 10,
            Note::Bb => 10,
            Note::B => 11,
            Note::Cb => 11,
            Note::Bs => 12,
            Note::C => 12,
        }
    }

    #[inline(always)]
    pub const fn from_num(val: u8) -> Option<Self> {
        Some(match val {
            1 => Note::Cs,
            2 => Note::D,
            3 => Note::Ds,
            4 => Note::E,
            5 => Note::F,
            6 => Note::Fs,
            7 => Note::G,
            8 => Note::Gs,
            9 => Note::A,
            10 => Note::As,
            11 => Note::B,
            12 => Note::C,
            _ => return None,
        })
    }

    pub const fn major_nodes() -> [Self; 12] {
        [
            Self::Cs,
            Self::D,
            Self::Ds,
            Self::E,
            Self::F,
            Self::Fs,
            Self::G,
            Self::Gs,
            Self::A,
            Self::As,
            Self::B,
            Self::C,
        ]
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Pitch {
    pitch: u8,
}

macro_rules! from_u8 {
    ($(#[$meta:meta])* $vis:vis enum $enum_name:ident { $($vari_name:ident = $num:expr $(,)?)* }) => {

        $(#[$meta])*
        $vis enum $enum_name {
            $($vari_name = $num, )*
        }

        impl $enum_name{
            #[inline(always)]
            pub fn from_num(val: u8) -> Result<Self, ()>{
                match val{
                    $($num => Ok(Self::$vari_name), )*
                    _ => Err(())
                }
            }

            pub fn name(&self) -> &'static str{
                match self{
                    $(Self::$vari_name => stringify!($vari_name), )*
                }
            }
        }
    };
}
from_u8! {
    #[repr(u8)]
    #[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, PartialOrd, Ord)]
    pub enum Insturment{
        AcousticGrandPiano=0,
        BrightAcousticPiano=1,
        ElectricGrandPiano=2,
        HonkyTonkPiano=3,
        RhodesPiano=4,
        ChorusedPiano=5,
        Harpsichord=6,
        Clavinet=7,
        Celesta=8,
        Glockenspiel=9,
        Musicbox=10,
        Vibraphone=11,
        Marimba=12,
        Xylophone=13,
        TubularBells=14,
        Dulcimer=15,
        HammondOrgan=16,
        PercussiveOrgan=17,
        RockOrgan=18,
        ChurchOrgan=19,
        ReedOrgan=20,
        Accordion=21,
        Harmonica=22,
        TangoAccordion=23,
        AcousticGuitarNylon=24,
        AcousticGuitarSteel=25,
        ElectricGuitarJazz=26,
        ElectricGuitarClean=27,
        ElectricGuitarMuted=28,
        OverdrivenGuitar=29,
        DistortionGuitar=30,
        GuitarHarmonics=31,
        AcousticBass=32,
        ElectricBassFinger=33,
        ElectricBassPick=34,
        FretlessBass=35,
        SlapBass1=36,
        SlapBass2=37,
        SynthBass1=38,
        SynthBass2=39,
        Violin=40,
        Viola=41,
        Cello=42,
        Contrabass=43,
        TremoloStrings=44,
        PizzicatoStrings=45,
        OrchestralHarp=46,
        Timpani=47,
        StringEnsemble1=48,
        StringEnsemble2=49,
        SynthStrings1=50,
        SynthStrings2=51,
        ChoirAahs=52,
        VoiceOohs=53,
        SynthVoice=54,
        OrchestraHit=55,
        Trumpet=56,
        Trombone=57,
        Tuba=58,
        MutedTrumpet=59,
        FrenchHorn=60,
        BrassSection=61,
        SynthBrass1=62,
        SynthBrass2=63,
        SopranoSax=64,
        AltoSax=65,
        TenorSax=66,
        BaritoneSax=67,
        Oboe=68,
        EnglishHorn=69,
        Bassoon=70,
        Clarinet=71,
        Piccolo=72,
        Flute=73,
        Recorder=74,
        PanFlute=75,
        BottleBlow=76,
        Shakuhachi=77,
        Whistle=78,
        Ocarina=79,
        Lead1square=80,
        Lead2sawtooth=81,
        Lead3calliopelead=82,
        Lead4chifferlead=83,
        Lead5charang=84,
        Lead6voice=85,
        Lead7fifths=86,
        Lead8brassLead=87,
        Pad1newage=88,
        Pad2warm=89,
        Pad3polysynth=90,
        Pad4choir=91,
        Pad5bowed=92,
        Pad6metallic=93,
        Pad7halo=94,
        Pad8sweep=95,
        FX1rain=96,
        FX2soundtrack=97,
        FX3crystal=98,
        FX4atmosphere=99,
        FX5brightness=100,
        FX6goblins=101,
        FX7echoes=102,
        FX8sciFi=103,
        Sitar=104,
        Banjo=105,
        Shamisen=106,
        Koto=107,
        Kalimba=108,
        Bagpipe=109,
        Fiddle=110,
        Shana=111,
        TinkleBell=112,
        Agogo=113,
        SteelDrums=114,
        Woodblock=115,
        TaikoDrum=116,
        MelodicTom=117,
        SynthDrum=118,
        ReverseCymbal=119,
        GuitarFretNoise=120,
        BreathNoise=121,
        Seashore=122,
        BirdTweet=123,
        TelephoneRing=124,
        Helicopter=125,
        Applause=126,
        Gunshot=127,
    }
}

impl From<InsturmentClass> for Insturment {
    #[inline(always)]
    fn from(value: InsturmentClass) -> Self {
        match Self::from_num(value.to_num()) {
            Ok(ok) => ok,
            Err(_) => unreachable!(),
        }
    }
}

#[repr(u8)]
#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub enum Class {
    C1 = 0,
    C2 = 1,
    C3 = 2,
    C4 = 3,
    C5 = 4,
    C6 = 5,
    C7 = 6,
    C8 = 7,
}

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub enum InsturmentClass {
    Piano(Class),
    ChromaticPercussion(Class),
    Organ(Class),
    Guitar(Class),
    Bass(Class),
    Strings(Class),
    Strings2(Class),
    Brass(Class),
    Reed(Class),
    Pipe(Class),
    SynthLead(Class),
    SynthPad(Class),
    SynthEffects(Class),
    Ethnic(Class),
    Percusive(Class),
    SoundEffect(Class),
}

impl InsturmentClass {
    #[inline(always)]
    pub const fn to_num(&self) -> u8 {
        match self {
            InsturmentClass::Piano(c) => 0 + (*c as u8),
            InsturmentClass::ChromaticPercussion(c) => 8 + (*c as u8),
            InsturmentClass::Organ(c) => 16 + (*c as u8),
            InsturmentClass::Guitar(c) => 24 + (*c as u8),
            InsturmentClass::Bass(c) => 32 + (*c as u8),
            InsturmentClass::Strings(c) => 40 + (*c as u8),
            InsturmentClass::Strings2(c) => 48 + (*c as u8),
            InsturmentClass::Brass(c) => 56 + (*c as u8),
            InsturmentClass::Reed(c) => 64 + (*c as u8),
            InsturmentClass::Pipe(c) => 72 + (*c as u8),
            InsturmentClass::SynthLead(c) => 80 + (*c as u8),
            InsturmentClass::SynthPad(c) => 88 + (*c as u8),
            InsturmentClass::SynthEffects(c) => 96 + (*c as u8),
            InsturmentClass::Ethnic(c) => 104 + (*c as u8),
            InsturmentClass::Percusive(c) => 112 + (*c as u8),
            InsturmentClass::SoundEffect(c) => 120 + (*c as u8),
        }
    }
}

impl Pitch {
    #[inline(always)]
    pub const fn get_pitch(&self) -> u8 {
        self.pitch
    }

    #[inline(always)]
    pub const fn note(mut octave: u8, note: Note) -> Self {
        if octave > 10 {
            octave = 10;
        }
        octave *= 12;
        let pitch = octave + note.to_num();
    
        Self { pitch }
    }

    #[inline(always)]
    pub const fn raw(pitch: u8) -> Self {
        if pitch > 127 {
            Self { pitch: 127 }
        } else {
            Self { pitch }
        }
    }
}

#[inline(always)]
pub fn get_midi() -> Midi {
    Midi(PhantomData)
}

pub struct Midi(PhantomData<()>);

impl Midi {
    #[inline(always)]
    pub fn out(
        &mut self,
        pitch: Pitch,
        duration_ms: u32,
        instrument: impl Into<Insturment>,
        volume: u8,
    ) {
        unsafe { crate::arch::midi_out(pitch.pitch, duration_ms, instrument.into() as u8, volume) }
    }

    #[inline(always)]
    pub fn out_sync(
        &mut self,
        pitch: Pitch,
        duration_ms: u32,
        instrument: impl Into<Insturment>,
        volume: u8,
    ) {
        unsafe {
            crate::arch::midi_out_sync(pitch.pitch, duration_ms, instrument.into() as u8, volume)
        }
    }
}
