// SPDX-License-Identifier: GPL-2.0

/* The linker tells us where the image is. */
unsafe extern "C" {
    pub static mut __image_begin: [u8; 0];
    pub static mut __image_end: [u8; 0];
}

/* debug interfaces  */
#[cfg(feature = "CONFIG_DEBUG_ZBOOT")]
unsafe extern "C" {
    pub fn putc(c: core::ffi::c_char);
    pub fn puts(s: *const core::ffi::c_char);
    pub fn puthex(val: u64);
}

#[cfg(not(feature = "CONFIG_DEBUG_ZBOOT"))]
macro_rules! putc {
    ($s:expr) => {{
        let _ = &$s;
    }};
}

#[cfg(not(feature = "CONFIG_DEBUG_ZBOOT"))]
macro_rules! puts {
    ($s:expr) => {{
        let _ = &$s;
    }};
}

#[cfg(not(feature = "CONFIG_DEBUG_ZBOOT"))]
macro_rules! puthex {
    ($val:expr) => {{
        let _ = &$val;
    }};
}

unsafe extern "C" {
    pub static mut __appended_dtb: [core::ffi::c_char; 0];

    pub fn error(x: *mut core::ffi::c_char);
    pub fn decompress_kernel(boot_heap_start: core::ffi::c_ulong);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
