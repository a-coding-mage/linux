// Translated from linux/platform_data/tda9950.h.

use core::ffi::{c_int, c_ulong, c_void};

#[repr(C)]
pub struct device {
    _private: [u8; 0],
}

#[repr(C)]
pub struct tda9950_glue {
    pub parent: *mut device,
    pub irq_flags: c_ulong,
    pub data: *mut c_void,
    pub init: Option<unsafe extern "C" fn(*mut c_void) -> c_int>,
    pub exit: Option<unsafe extern "C" fn(*mut c_void)>,
    pub open: Option<unsafe extern "C" fn(*mut c_void) -> c_int>,
    pub release: Option<unsafe extern "C" fn(*mut c_void)>,
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
