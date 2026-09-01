// SPDX-License-Identifier: GPL-2.0
//
// Translated from C implementation source. Original includes:
// <sys/types.h>, <errno.h>, <unistd.h>, <stdio.h>, <stdlib.h>,
// <string.h>, <regex.h>, "../../../util/debug.h",
// "../../../util/header.h", "cpuid.h"

use libc::{
    c_char, c_int, c_uint, c_void, free, malloc, regcomp, regexec, regfree, regmatch_t, regex_t,
    strchr, strlen, strncpy, strrchr, size_t, ENOBUFS, REG_EXTENDED,
};

#[repr(C)]
pub struct perf_cpu {
    _private: [u8; 0],
}

extern "C" {
    fn cpuid(
        op: c_uint,
        count: c_uint,
        eax: *mut c_uint,
        ebx: *mut c_uint,
        ecx: *mut c_uint,
        edx: *mut c_uint,
    );

    fn scnprintf(buf: *mut c_char, size: size_t, fmt: *const c_char, ...) -> c_int;
    fn pr_info(fmt: *const c_char, ...) -> c_int;
}

#[no_mangle]
pub unsafe extern "C" fn get_cpuid_0(vendor: *mut c_char, lvl: *mut c_uint) {
    let mut b: c_uint = 0;
    let mut c: c_uint = 0;
    let mut d: c_uint = 0;

    cpuid(0, 0, lvl, &mut b, &mut c, &mut d);
    strncpy(vendor.add(0), (&mut b as *mut c_uint).cast::<c_char>(), 4);
    strncpy(vendor.add(4), (&mut d as *mut c_uint).cast::<c_char>(), 4);
    strncpy(vendor.add(8), (&mut c as *mut c_uint).cast::<c_char>(), 4);
    *vendor.add(12) = b'\0' as c_char;
}

unsafe fn __get_cpuid(buffer: *mut c_char, sz: size_t, fmt: *const c_char) -> c_int {
    let mut a: c_uint = 0;
    let mut b: c_uint = 0;
    let mut c: c_uint = 0;
    let mut d: c_uint = 0;
    let mut lvl: c_uint = 0;
    let mut family: c_int = -1;
    let mut model: c_int = -1;
    let mut step: c_int = -1;
    let nb: c_int;
    let mut vendor = [0 as c_char; 16];

    get_cpuid_0(vendor.as_mut_ptr(), &mut lvl);

    if lvl >= 1 {
        cpuid(1, 0, &mut a, &mut b, &mut c, &mut d);

        family = ((a >> 8) & 0xf) as c_int; /* bits 11 - 8 */
        model = ((a >> 4) & 0xf) as c_int; /* Bits  7 - 4 */
        step = (a & 0xf) as c_int;

        /* extended family */
        if family == 0xf {
            family += ((a >> 20) & 0xff) as c_int;
        }

        /* extended model */
        if family >= 0x6 {
            model += (((a >> 16) & 0xf) << 4) as c_int;
        }
    }
    nb = scnprintf(
        buffer,
        sz,
        fmt,
        vendor.as_ptr(),
        family as c_uint,
        model as c_uint,
        step as c_uint,
    );

    /* look for end marker to ensure the entire data fit */
    if !strchr(buffer, b'$' as c_int).is_null() {
        *buffer.add((nb - 1) as usize) = b'\0' as c_char;
        return 0;
    }
    ENOBUFS
}

#[no_mangle]
pub unsafe extern "C" fn get_cpuid(
    buffer: *mut c_char,
    sz: size_t,
    cpu: perf_cpu, /* __maybe_unused */
) -> c_int {
    let _ = cpu;
    __get_cpuid(buffer, sz, b"%s,%u,%u,%u$\0".as_ptr().cast::<c_char>())
}

#[no_mangle]
pub unsafe extern "C" fn get_cpuid_str(cpu: perf_cpu /* __maybe_unused */) -> *mut c_char {
    let _ = cpu;
    let buf = malloc(128).cast::<c_char>();

    if !buf.is_null()
        && __get_cpuid(buf, 128, b"%s-%u-%X-%X$\0".as_ptr().cast::<c_char>()) < 0
    {
        free(buf.cast::<c_void>());
        return core::ptr::null_mut();
    }
    buf
}

/* Full CPUID format for x86 is vendor-family-model-stepping */
unsafe fn is_full_cpuid(id: *const c_char) -> bool {
    let mut tmp = id;
    let mut count: c_int = 0;

    loop {
        tmp = strchr(tmp, b'-' as c_int);
        if tmp.is_null() {
            break;
        }
        count += 1;
        tmp = tmp.add(1);
    }

    if count == 3 {
        return true;
    }

    false
}

#[no_mangle]
pub unsafe extern "C" fn strcmp_cpuid_str(
    mapcpuid: *const c_char,
    id: *const c_char,
) -> c_int {
    let mut re = core::mem::MaybeUninit::<regex_t>::uninit();
    let mut pmatch = [core::mem::MaybeUninit::<regmatch_t>::uninit(); 1];
    let match_: c_int;
    let full_mapcpuid: bool = is_full_cpuid(mapcpuid);
    let full_cpuid: bool = is_full_cpuid(id);

    /*
     * Full CPUID format is required to identify a platform.
     * Error out if the cpuid string is incomplete.
     */
    if full_mapcpuid && !full_cpuid {
        pr_info(
            b"Invalid CPUID %s. Full CPUID is required, vendor-family-model-stepping\n\0"
                .as_ptr()
                .cast::<c_char>(),
            id,
        );
        return 1;
    }

    if regcomp(re.as_mut_ptr(), mapcpuid, REG_EXTENDED) != 0 {
        /* Warn unable to generate match particular string. */
        pr_info(
            b"Invalid regular expression %s\n\0"
                .as_ptr()
                .cast::<c_char>(),
            mapcpuid,
        );
        return 1;
    }

    match_ = (regexec(re.as_ptr(), id, 1, pmatch.as_mut_ptr().cast::<regmatch_t>(), 0) == 0)
        as c_int;
    regfree(re.as_mut_ptr());
    if match_ != 0 {
        let pmatch0 = pmatch[0].assume_init();
        let match_len: size_t = (pmatch0.rm_eo - pmatch0.rm_so) as size_t;
        let cpuid_len: size_t;

        /* If the full CPUID format isn't required,
         * ignoring the stepping.
         */
        if !full_mapcpuid && full_cpuid {
            cpuid_len = strrchr(id, b'-' as c_int).offset_from(id) as size_t;
        } else {
            cpuid_len = strlen(id);
        }

        /* Verify the entire string matched. */
        if match_len == cpuid_len {
            return 0;
        }
    }

    1
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
