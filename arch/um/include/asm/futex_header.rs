/* SPDX-License-Identifier: GPL-2.0 */

// Dependencies supplied by the corresponding Linux and architecture headers:
// <linux/futex.h>, <linux/uaccess.h>, and <asm/errno.h>.

unsafe extern "C" {
    pub fn arch_futex_atomic_op_inuser(
        op: ::core::ffi::c_int,
        oparg: u32,
        oval: *mut ::core::ffi::c_int,
        uaddr: *mut u32,
    ) -> ::core::ffi::c_int;

    pub fn futex_atomic_cmpxchg_inatomic(
        uval: *mut u32,
        uaddr: *mut u32,
        oldval: u32,
        newval: u32,
    ) -> ::core::ffi::c_int;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
