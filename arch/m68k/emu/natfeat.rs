/*
 * natfeat.c - ARAnyM hardware support via Native Features (natfeats)
 *
 * Copyright (c) 2005 Petr Stehlik of ARAnyM dev team
 *
 * Reworked for Linux by Roman Zippel <zippel@linux-m68k.org>
 *
 * This software may be used and distributed according to the terms of
 * the GNU General Public License (GPL), incorporated herein by reference.
 */

use core::ffi::{c_char, c_int, c_long, c_ulong, c_void};

// Declarations supplied by the Linux/m68k environment.
unsafe extern "C" {
    fn nf_get_id_phys(feature_name: c_ulong) -> c_long;
    fn nf_call(id: c_long, ...) -> c_long;
    fn strscpy(dest: *mut c_char, src: *const c_char, count: usize) -> isize;
    fn virt_to_phys(address: *const c_void) -> c_ulong;
    fn register_platform_power_off(callback: unsafe extern "C" fn());
    fn pr_info(format: *const c_char, ...);
}

// EXPORT_SYMBOL_GPL(nf_call);

pub unsafe extern "C" fn nf_get_id(feature_name: *const c_char) -> c_long {
    /* feature_name may be in vmalloc()ed memory, so make a copy */
    let mut name_copy = [0 as c_char; 32];
    let n = unsafe { strscpy(name_copy.as_mut_ptr(), feature_name, name_copy.len()) };
    if n < 0 {
        return 0;
    }

    unsafe { nf_get_id_phys(virt_to_phys(name_copy.as_ptr() as *const c_void)) }
}

// EXPORT_SYMBOL_GPL(nf_get_id);

pub unsafe extern "C" fn nfprint(fmt: *const c_char, ...) {
    static mut BUF: [c_char; 256] = [0; 256];

    // The C implementation forwards its va_list to vsnprintf.  The
    // platform's variadic ABI supplies this operation outside this file.
    let _ = fmt;
    unsafe {
        nf_call(
            nf_get_id(c"NF_STDERR".as_ptr()),
            virt_to_phys(BUF.as_ptr() as *const c_void),
        );
    }
}

unsafe extern "C" fn nf_poweroff() {
    let id = unsafe { nf_get_id(c"NF_SHUTDOWN".as_ptr()) };

    if id != 0 {
        unsafe { nf_call(id) };
    }
}

pub unsafe extern "C" fn nf_init() {
    let mut id: c_ulong;
    let version: c_ulong;
    let mut buf = [0 as c_char; 256];

    id = unsafe { nf_get_id(c"NF_VERSION".as_ptr()) as c_ulong };
    if id == 0 {
        return;
    }
    version = unsafe { nf_call(id as c_long) as c_ulong };

    id = unsafe { nf_get_id(c"NF_NAME".as_ptr()) as c_ulong };
    if id == 0 {
        return;
    }
    unsafe {
        nf_call(
            id as c_long,
            virt_to_phys(buf.as_mut_ptr() as *mut c_void),
            256 as c_int,
        );
    }
    buf[255] = 0;

    unsafe {
        pr_info(
            c"NatFeats found (%s, %lu.%lu)\n".as_ptr(),
            buf.as_ptr(),
            version >> 16,
            version & 0xffff,
        );
        register_platform_power_off(nf_poweroff);
    }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
