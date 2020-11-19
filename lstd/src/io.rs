use c::{
    stdio,
    types::*
};
use core::{
    fmt::{Write, Error},
    ptr::null_mut,
    ops::Drop
};

const STRING_BUFFER_SIZE: usize = 1024;
pub static mut STRING_BUFFER: StringWriter =
    StringWriter { buf: [0_u8; STRING_BUFFER_SIZE], cursor: 0 };

pub fn print(s: &str) {
    unsafe {
        c::stdio::write(c::stdio::STDOUT, s.as_ptr() as *const _, s.len());
    }
}

pub struct StringWriter {
    pub buf: [u8; STRING_BUFFER_SIZE],
    pub cursor: usize
}

impl StringWriter {
    pub fn reset_cursor(&mut self) {
        self.cursor = 0;
    }

    pub fn add_terminator(&mut self) {
        self.buf[self.cursor] = b'\0';
        self.cursor += 1;
    }
}

impl Write for StringWriter {
    fn write_str(&mut self, s: &str) -> Result<(), Error> {
        if self.cursor + s.len() < STRING_BUFFER_SIZE {
            for (i, ch) in s.as_bytes().iter().enumerate() {
                self.buf[self.cursor + i] = *ch;
            }
            self.cursor += s.len();
        }
        else {
            panic!()
        }

        Ok(())
    }
}

#[macro_export]
macro_rules! print {
    ($s:expr) => (print!($s,));

    ($fmt:expr, $($expr:expr),*) => {{
        use $crate::io::{STRING_BUFFER, StringWriter, print};
        use core::{
            fmt::{write, Write},
            str::from_utf8_unchecked
        };

        unsafe {
            STRING_BUFFER.reset_cursor();
            let _ = write(
                &mut STRING_BUFFER,
                format_args!($fmt, $($expr),*)
            );
            STRING_BUFFER.add_terminator();

            print(from_utf8_unchecked(&STRING_BUFFER.buf[0 .. STRING_BUFFER.cursor]));
        }

    }}
}

#[macro_export]
macro_rules! println {
    ($s:expr) => (println!($s,));

    ($fmt:expr, $($expr:expr),*) => {{
        $crate::print!(concat!($fmt, "\n"), $($expr),*);
    }}
}

pub enum FileMode {
    Read,
    Write,
    Append
}

pub enum FileSeek {
    Set,
    Cur,
    End
}

pub struct File(*mut c_void, FileMode);

impl File {
    pub unsafe fn open(filename: &str, mode: FileMode) -> Option<File> {
        let handle = {
            let mode = match mode {
                FileMode::Read   => b"rb\0",
                FileMode::Write  => b"wb\0",
                FileMode::Append => b"ab\0"
            };
            stdio::fopen(
                filename.as_ptr() as *const _,
                mode.as_ptr() as *mut _
            )
        };
        if handle == null_mut() {
            None
        } else {
            Some(File(handle, mode))
        }
    }

    pub fn read<'a>(&self, buf: &'a mut [u8]) -> Option<&'a [u8]> {
        match self.1 {
            FileMode::Write => return None,
            _ => {}
        };

        let read = unsafe {
            stdio::fread(
                buf.as_mut_ptr() as *mut _,
                1,
                buf.len(),
                self.0
            )
        };

        if read == 0 {
            None
        } else {
            Some(&buf[0..read])
        }
    }

    pub fn write(&self, buf: &[u8]) -> usize {
        match self.1 {
            FileMode::Read => return 0,
            _ => {}
        };

        unsafe {
            stdio::fwrite(
                buf.as_ptr() as *mut _,
                1,
                buf.len(),
                self.0
            )
        }
    }
}

impl Drop for File {
    fn drop(&mut self) {
        unsafe { stdio::fclose(self.0); }
    }
}

