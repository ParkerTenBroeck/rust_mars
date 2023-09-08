use crate::arch::FileDesciptor;



#[derive(Debug)]
pub struct File{
    descriptor: FileDesciptor,
    flag: FileFlag,
}

pub use crate::arch::FileFlag;
pub use crate::arch::FileIOErrorCode;

pub const STDIN: File = File{
    descriptor: crate::arch::STDIN,
    flag: FileFlag::Read,
};
pub const STDOUT: File = File{
    descriptor: crate::arch::STDOUT,
    flag: FileFlag::Write,
};
pub const STDERR: File = File{
    descriptor: crate::arch::STDERR,
    flag: FileFlag::Write,
};

impl File{
    pub fn new(path: &str, flag: FileFlag) -> Result<Self, FileIOErrorCode>{
        crate::rt::str_to_cstr(path, |c|{
            Self::new_raw(c, flag)
        })
    }

    pub fn new_raw(path: &core::ffi::CStr, flag: FileFlag) -> Result<Self, FileIOErrorCode>{
        Ok(Self{
            descriptor: unsafe{crate::arch::open_file(path, flag)}?,
            flag: flag,
        })
    }

    pub fn write(&mut self, data: &[u8]) -> Result<usize, FileIOErrorCode>{
        unsafe{
            crate::arch::write_file(&mut self.descriptor, data)
        }
    }

    pub fn read(&self, data: &mut[u8]) -> Result<usize, FileIOErrorCode>{
        unsafe{
            crate::arch::read_file(&self.descriptor, data)
        }
    }
}

impl core::fmt::Write for File{
    fn write_str(&mut self, s: &str) -> core::fmt::Result {
        match self.write(s.as_bytes()){
            Ok(_) => Ok(()),
            Err(_) => Err(core::fmt::Error)
        }
    }
}

impl Drop for File{
    fn drop(&mut self) {
        unsafe{
            crate::arch::close_file(self.descriptor)
        }
    }
}