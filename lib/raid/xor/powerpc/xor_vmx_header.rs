/* SPDX-License-Identifier: GPL-2.0 */
/*
 * Simple interface to link xor_vmx.c and xor_vmx_glue.c
 *
 * Separating these file ensures that no altivec instructions are run
 * outside of the enable/disable altivec block.
 */

use std::ffi::c_void;
use std::os::raw::c_uint;

unsafe extern "C" {
    pub fn xor_gen_altivec_inner(
        dest: *mut c_void,
        srcs: *mut *mut c_void,
        src_cnt: c_uint,
        bytes: c_uint,
    );
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
