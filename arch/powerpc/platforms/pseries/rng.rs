// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * Copyright 2013, Michael Ellerman, IBM Corporation.
 */

// #define pr_fmt(fmt) "pseries-rng: " fmt
// C dependencies supplied by other files are referenced here as external items.

use core::ffi::c_void;

extern "C" {
    fn plpar_hcall(token: u64, retbuf: *mut c_ulong) -> c_long;
    fn of_find_compatible_node(
        from: *mut device_node,
        type_: *const c_char,
        compatible: *const c_char,
    ) -> *mut device_node;
    fn of_node_put(node: *mut device_node);
}

type c_ulong = usize;
type c_long = isize;
type c_char = i8;

#[repr(C)]
pub struct device_node {
    _private: [u8; 0],
}

#[repr(C)]
pub struct machdep_calls {
    pub get_random_seed: Option<unsafe extern "C" fn(*mut c_ulong) -> c_int>,
}

type c_int = i32;

extern "C" {
    static mut ppc_md: machdep_calls;
}

const PLPAR_HCALL_BUFSIZE: usize = 4;
const H_RANDOM: u64 = 0;
const H_SUCCESS: c_long = 0;

unsafe extern "C" fn pseries_get_random_long(v: *mut c_ulong) -> c_int {
    let mut retbuf: [c_ulong; PLPAR_HCALL_BUFSIZE] = [0; PLPAR_HCALL_BUFSIZE];

    if plpar_hcall(H_RANDOM, retbuf.as_mut_ptr()) == H_SUCCESS {
        *v = retbuf[0];
        return 1;
    }

    0
}

pub unsafe extern "C" fn pseries_rng_init() {
    let dn: *mut device_node;

    dn = of_find_compatible_node(
        core::ptr::null_mut(),
        core::ptr::null(),
        b"ibm,random\0".as_ptr() as *const c_char,
    );
    if dn.is_null() {
        return;
    }
    ppc_md.get_random_seed = Some(pseries_get_random_long);
    of_node_put(dn);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
