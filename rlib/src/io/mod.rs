use crate::sync::{Mutex, MutexGuard};



struct StdOutRaw;

impl StdOutRaw {
    fn write_str(&mut self, s: &str) {
        for char in s.chars(){
            crate::arch::print_char(char)
        }
    }

    fn flush(&mut self) {
        // crate::arch::flush();
    }
}

pub struct StdOut {
    lock: MutexGuard<'static, StdOutRaw>,
}

impl Drop for StdOut {
    fn drop(&mut self) {
        self.lock.flush()
    }
}

static STDOUT: Mutex<StdOutRaw> = Mutex::new(StdOutRaw);

pub fn stdout() -> StdOut {
    StdOut {
        lock: STDOUT.lock(),
    }
}

impl core::fmt::Write for StdOut {
    fn write_str(&mut self, s: &str) -> core::fmt::Result {
        self.lock.write_str(s);
        Ok(())
    }
}
