#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(dead_code)]

mod lib {
    use std::ffi::c_void;
    use std::slice;

    include!(concat!(env!("OUT_DIR"), "/bindings.rs"));

    pub fn create_callback<F: Fn(f64, &[u8])>(
        f: F,
    ) -> (
        unsafe extern "C" fn(f64, *const u8, usize, *mut c_void),
        *mut F,
    ) {
        unsafe extern "C" fn trampoline<F: Fn(f64, &[u8])>(
            timestamp: f64,
            data: *const u8,
            size: usize,
            func: *mut c_void,
        ) {
            let messages = slice::from_raw_parts(data, size);
            (*(func as *mut F))(timestamp, messages)
        }
        (trampoline::<F>, Box::into_raw(Box::new(f)))
    }
}

pub use lib::*;
