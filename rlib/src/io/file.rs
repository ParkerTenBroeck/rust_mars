use crate::arch::FileDesciptor;

#[cfg(feature = "alloc")]
use crate::alloc_crate::string::String;
#[cfg(feature = "alloc")]
use crate::alloc_crate::vec::Vec;

#[derive(Debug)]
pub struct File {
    descriptor: FileDesciptor,
    flag: FileFlag,
}

pub use crate::arch::FileFlag;
pub use crate::arch::FileIOErrorCode;

pub const STDIN: File = File {
    descriptor: crate::arch::STDIN,
    flag: FileFlag::Read,
};
pub const STDOUT: File = File {
    descriptor: crate::arch::STDOUT,
    flag: FileFlag::Write,
};
pub const STDERR: File = File {
    descriptor: crate::arch::STDERR,
    flag: FileFlag::Write,
};

impl File {
    pub fn new(path: &str, flag: FileFlag) -> Result<Self, FileIOErrorCode> {
        crate::rt::str_to_cstr(path, |c| Self::new_raw(c, flag))
    }

    pub fn read(path: &str) -> Result<Self, FileIOErrorCode> {
        Self::new(path, FileFlag::Read)
    }

    pub fn write(path: &str) -> Result<Self, FileIOErrorCode> {
        Self::new(path, FileFlag::Write)
    }

    pub fn write_create(path: &str) -> Result<Self, FileIOErrorCode> {
        Self::new(path, FileFlag::WriteOnlyWithCreateAppend)
    }

    pub fn new_raw(path: &core::ffi::CStr, flag: FileFlag) -> Result<Self, FileIOErrorCode> {
        Ok(Self {
            descriptor: unsafe { crate::arch::open_file(path, flag) }?,
            flag: flag,
        })
    }

    pub fn read_raw(path: &core::ffi::CStr) -> Result<Self, FileIOErrorCode> {
        Self::new_raw(path, FileFlag::Read)
    }

    pub fn write_raw(path: &core::ffi::CStr) -> Result<Self, FileIOErrorCode> {
        Self::new_raw(path, FileFlag::Write)
    }

    pub fn write_create_raw(path: &core::ffi::CStr) -> Result<Self, FileIOErrorCode> {
        Self::new_raw(path, FileFlag::WriteOnlyWithCreateAppend)
    }

    pub fn write_slice(&mut self, data: &[u8]) -> Result<usize, FileIOErrorCode> {
        unsafe { crate::arch::write_file(&mut self.descriptor, data) }
    }

    #[cfg(feature = "alloc")]
    pub fn read_slice(&self, data: &mut [u8]) -> Result<usize, FileIOErrorCode> {
        unsafe { crate::arch::read_file(&self.descriptor, data) }
    }

    #[cfg(feature = "alloc")]
    pub fn read_to_string(self) -> Result<String, FileIOErrorCode> {
        let vec = self.read_to_vec()?;
        String::from_utf8(vec).map_err(|_|FileIOErrorCode(1))
    }

    pub fn read_to_vec(self) -> Result<Vec<u8>, FileIOErrorCode> {
        let mut data = Vec::with_capacity(32);

        loop {
            let read = unsafe {
                let spare = data.spare_capacity_mut();
                crate::arch::read_file_raw(&self.descriptor, spare.as_mut_ptr().cast(), spare.len())
            }?;

            unsafe{
                let new_len = data.len() + read;
                data.set_len(new_len)
            }

            if data.len() < data.capacity(){
                break;
            }else{
                data.reserve(1);
            }
        }

        Ok(data)
    }

    pub fn flag(&self) -> FileFlag {
        self.flag
    }
}

impl core::fmt::Write for File {
    fn write_str(&mut self, s: &str) -> core::fmt::Result {
        match self.write_slice(s.as_bytes()) {
            Ok(_) => Ok(()),
            Err(_) => Err(core::fmt::Error),
        }
    }
}

impl Drop for File {
    fn drop(&mut self) {
        unsafe { crate::arch::close_file(self.descriptor) }
    }
}
