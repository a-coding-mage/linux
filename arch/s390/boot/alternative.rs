// SPDX-License-Identifier: GPL-2.0
// boot_fmt(fmt) expands to "alt: " fmt; the boot headers provide the
// remaining declarations used by the included alternative implementation.

use core::ffi::{c_char, c_ulong};

// Declarations supplied by the surrounding s390 alternative implementation.
// The exact layout and constants are provided by the corresponding headers.
extern "C" {
    static mut alt_debug: AltDebug;
    fn __clear_facility(nr: c_uint, facilities: *mut c_ulong);
    fn __set_facility(nr: c_uint, facilities: *mut c_ulong);
    fn __clear_machine_feature(nr: c_uint, mfeatures: *mut c_ulong);
    fn __set_machine_feature(nr: c_uint, mfeatures: *mut c_ulong);
    fn simple_strtoull(s: *const c_char, endp: *mut *mut c_char, base: c_uint) -> c_ulong;
}

type c_uint = u32;

#[repr(C)]
struct AltDebug {
    facilities: [c_ulong; 4],
    mfeatures: [c_ulong; 2],
    spec: bool,
}

const ALT_TYPE_FACILITY: c_uint = 0;
const ALT_TYPE_FEATURE: c_uint = 1;
const ALT_TYPE_SPEC: c_uint = 2;

unsafe fn alt_debug_all(type_: c_uint) {
    match type_ {
        ALT_TYPE_FACILITY => {
            let mut i = 0;
            while i < (*core::ptr::addr_of_mut!(alt_debug)).facilities.len() {
                (*core::ptr::addr_of_mut!(alt_debug)).facilities[i] = !0;
                i += 1;
            }
        }
        ALT_TYPE_FEATURE => {
            let mut i = 0;
            while i < (*core::ptr::addr_of_mut!(alt_debug)).mfeatures.len() {
                (*core::ptr::addr_of_mut!(alt_debug)).mfeatures[i] = !0;
                i += 1;
            }
        }
        ALT_TYPE_SPEC => (*core::ptr::addr_of_mut!(alt_debug)).spec = true,
        _ => {}
    }
}

unsafe fn alt_debug_modify(type_: c_uint, nr: c_uint, clear: bool) {
    match type_ {
        ALT_TYPE_FACILITY => {
            if clear {
                __clear_facility(nr, (*core::ptr::addr_of_mut!(alt_debug)).facilities.as_mut_ptr());
            } else {
                __set_facility(nr, (*core::ptr::addr_of_mut!(alt_debug)).facilities.as_mut_ptr());
            }
        }
        ALT_TYPE_FEATURE => {
            if clear {
                __clear_machine_feature(nr, (*core::ptr::addr_of_mut!(alt_debug)).mfeatures.as_mut_ptr());
            } else {
                __set_machine_feature(nr, (*core::ptr::addr_of_mut!(alt_debug)).mfeatures.as_mut_ptr());
            }
        }
        _ => {}
    }
}

unsafe fn alt_debug_parse(type_: c_uint, mut str_: *mut c_char) -> *mut c_char {
    if *str_ == b':' as c_char {
        str_ = str_.add(1);
    } else {
        alt_debug_all(type_);
        return str_;
    }
    let mut clear = false;
    if *str_ == b'!' as c_char {
        alt_debug_all(type_);
        clear = true;
        str_ = str_.add(1);
    }
    while *str_ != 0 {
        let mut endp: *mut c_char = core::ptr::null_mut();
        let mut val = simple_strtoull(str_, &mut endp, 0);
        if str_ == endp { break; }
        str_ = endp;
        if *str_ == b'-' as c_char {
            str_ = str_.add(1);
            let endval = simple_strtoull(str_, &mut endp, 0);
            if str_ == endp { break; }
            str_ = endp;
            while val <= endval {
                alt_debug_modify(type_, val as c_uint, clear);
                val = val.wrapping_add(1);
            }
        } else {
            alt_debug_modify(type_, val as c_uint, clear);
        }
        if *str_ != b',' as c_char { break; }
        str_ = str_.add(1);
    }
    str_
}

pub unsafe extern "C" fn alt_debug_setup(mut str_: *mut c_char) {
    if str_.is_null() {
        alt_debug_all(ALT_TYPE_FACILITY);
        alt_debug_all(ALT_TYPE_FEATURE);
        alt_debug_all(ALT_TYPE_SPEC);
        return;
    }
    while *str_ != 0 {
        let mut endp: *mut c_char = core::ptr::null_mut();
        let type_ = simple_strtoull(str_, &mut endp, 0) as c_uint;
        if str_ == endp { break; }
        str_ = endp;
        match type_ {
            ALT_TYPE_FACILITY | ALT_TYPE_FEATURE => str_ = alt_debug_parse(type_, str_),
            ALT_TYPE_SPEC => alt_debug_all(ALT_TYPE_SPEC),
            _ => {}
        }
        if *str_ != b';' as c_char { break; }
        str_ = str_.add(1);
    }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
