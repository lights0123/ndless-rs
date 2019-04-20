use core::fmt::{Error, Write};

pub fn print(msg: &str) {
    unsafe {
        // Instead of allocating a C-style string, we tell printf the length to output
        ndless_sys::printf("%.*s\0".as_ptr() as *const cty::c_char, msg.len(), msg);
    }
}

pub fn println(msg: &str) {
    unsafe {
        // Instead of allocating a C-style string, we tell printf the length to output
        ndless_sys::printf("%.*s\n\0".as_ptr() as *const cty::c_char, msg.len(), msg);
    }
}

pub struct STDOut {}

impl Write for STDOut {
    fn write_str(&mut self, s: &str) -> Result<(), Error> {
        print(s);
        Ok(())
    }
}
