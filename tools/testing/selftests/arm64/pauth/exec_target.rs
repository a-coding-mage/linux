// SPDX-License-Identifier: GPL-2.0
// Copyright (C) 2020 ARM Limited

use core::ffi::{c_int, c_ulong, c_void};

type size_t = usize;

const EXIT_FAILURE: c_int = 1;

// From <sys/auxv.h> / kernel hwcap definitions.
const AT_HWCAP: c_ulong = 16;
const HWCAP_PACA: c_ulong = 1 << 30;
const HWCAP_PACG: c_ulong = 1 << 31;

#[repr(C)]
struct FILE {
    _private: [u8; 0],
}

#[repr(C)]
struct signatures {
    keyia: size_t,
    keyib: size_t,
    keyda: size_t,
    keydb: size_t,
    keyg: size_t,
}

unsafe extern "C" {
    static mut stdin: *mut FILE;
    static mut stdout: *mut FILE;
    static mut stderr: *mut FILE;

    fn fread(ptr: *mut c_void, size: size_t, nmemb: size_t, stream: *mut FILE) -> size_t;
    fn fwrite(ptr: *const c_void, size: size_t, nmemb: size_t, stream: *mut FILE) -> size_t;
    fn fprintf(stream: *mut FILE, format: *const u8, ...) -> c_int;
    fn getauxval(type_: c_ulong) -> c_ulong;

    // Declarations supplied by helper.h.
    fn keyia_sign(val: size_t) -> size_t;
    fn keyib_sign(val: size_t) -> size_t;
    fn keyda_sign(val: size_t) -> size_t;
    fn keydb_sign(val: size_t) -> size_t;
    fn keyg_sign(val: size_t) -> size_t;
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn main() -> c_int {
    let mut signed_vals: signatures = core::mem::zeroed();
    let mut hwcaps: c_ulong;
    let mut val: size_t = 0;

    let size: size_t = fread(
        &mut val as *mut size_t as *mut c_void,
        core::mem::size_of::<size_t>(),
        1,
        stdin,
    );

    if size != 1 {
        fprintf(
            stderr,
            c"Could not read input from stdin\n".as_ptr() as *const u8,
        );
        return EXIT_FAILURE;
    }

    /* don't try to execute illegal (unimplemented) instructions) caller
     * should have checked this and keep worker simple
     */
    hwcaps = getauxval(AT_HWCAP);

    if (hwcaps & HWCAP_PACA) != 0 {
        signed_vals.keyia = keyia_sign(val);
        signed_vals.keyib = keyib_sign(val);
        signed_vals.keyda = keyda_sign(val);
        signed_vals.keydb = keydb_sign(val);
    }
    signed_vals.keyg = if (hwcaps & HWCAP_PACG) != 0 {
        keyg_sign(val)
    } else {
        0
    };

    fwrite(
        &signed_vals as *const signatures as *const c_void,
        core::mem::size_of::<signatures>(),
        1,
        stdout,
    );

    return 0;
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
