use crate::sync::{Mutex, MutexGuard};

pub mod file;
pub mod midi;

struct StdOutRaw;

impl StdOutRaw {
    fn write_str(&mut self, s: &str) {
        for char in s.chars() {
            crate::arch::print_char(char)
        }
    }

    fn flush(&mut self) {
        // crate::arch::flush();
    }
}

pub struct StdOut {
    lock: MutexGuard<'static, StdOutRaw>,
    // inner: StdOutRaw,
}

impl Drop for StdOut {
    fn drop(&mut self) {
        // self.inner.flush()
        self.lock.flush()
    }
}

static STDOUT: Mutex<StdOutRaw> = Mutex::new(StdOutRaw);

pub fn stdout() -> StdOut {
    // StdOut { inner: StdOutRaw }
    StdOut {
        lock: STDOUT.lock(),
    }
}

impl core::fmt::Write for StdOut {
    fn write_str(&mut self, s: &str) -> core::fmt::Result {
        // self.inner.write_str(s);
        self.lock.write_str(s);
        Ok(())
    }
}
