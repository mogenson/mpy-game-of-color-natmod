use crate::mpy_types::MpPrintT;
use core::ffi::c_char;
use ufmt::uWrite;

impl uWrite for MpPrintT {
    type Error = core::convert::Infallible;

    fn write_str(&mut self, s: &str) -> Result<(), Self::Error> {
        let ptr = s.as_ptr() as *const c_char;
        let len = s.len();
        unsafe {
            (self.print_strn)(self.env, ptr, len);
        }
        Ok(())
    }
}

#[macro_export]
macro_rules! mpy_print {
    ($($arg:tt)*) => {{
        unsafe {
            let p = $crate::mpy_types::mpy_get_plat_print() as *mut $crate::mpy_types::MpPrintT;
            let _ = ufmt::uwrite!(&mut *p, $($arg)*);
        }
    }};
}

#[macro_export]
macro_rules! mpy_println {
    ($($arg:tt)*) => {{
        unsafe {
            let p = $crate::mpy_types::mpy_get_plat_print() as *mut $crate::mpy_types::MpPrintT;
            let _ = ufmt::uwriteln!(&mut *p, $($arg)*);
        }
    }};
}
