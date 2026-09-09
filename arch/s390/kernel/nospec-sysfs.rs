// SPDX-License-Identifier: GPL-2.0
//
// Dependencies supplied by the corresponding Linux headers are represented
// here as external declarations.

use core::ffi::c_char;

#[repr(C)]
pub struct device {
    _private: [u8; 0],
}

#[repr(C)]
pub struct device_attribute {
    _private: [u8; 0],
}

unsafe extern "C" {
    fn sysfs_emit(buf: *mut c_char, fmt: *const c_char, ...) -> isize;
    fn test_facility(facility: i32) -> bool;
    fn nospec_uses_trampoline() -> bool;
    fn nobp_enabled() -> bool;
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn cpu_show_spectre_v1(
    _dev: *mut device,
    _attr: *mut device_attribute,
    buf: *mut c_char,
) -> isize {
    unsafe { sysfs_emit(buf, c"Mitigation: __user pointer sanitization\n".as_ptr()) }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn cpu_show_spectre_v2(
    _dev: *mut device,
    _attr: *mut device_attribute,
    buf: *mut c_char,
) -> isize {
    if unsafe { test_facility(156) } {
        return unsafe { sysfs_emit(buf, c"Mitigation: etokens\n".as_ptr()) };
    }
    if unsafe { nospec_uses_trampoline() } {
        return unsafe { sysfs_emit(buf, c"Mitigation: execute trampolines\n".as_ptr()) };
    }
    if unsafe { nobp_enabled() } {
        return unsafe { sysfs_emit(buf, c"Mitigation: limited branch prediction\n".as_ptr()) };
    }
    unsafe { sysfs_emit(buf, c"Vulnerable\n".as_ptr()) }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
