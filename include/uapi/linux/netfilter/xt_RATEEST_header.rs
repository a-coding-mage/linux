/* SPDX-License-Identifier: GPL-2.0 WITH Linux-syscall-note */

// Dependency equivalents supplied by the surrounding Linux headers:
// linux/types.h and linux/if.h.

use core::ffi::c_char;

// Opaque type declared by the netfilter rate-estimator implementation.
#[repr(C)]
pub struct xt_rateest {
    _private: [u8; 0],
}

// IFNAMSIZ is supplied by linux/if.h.
extern "C" {
    pub static IFNAMSIZ: usize;
}

#[repr(C, align(8))]
pub struct xt_rateest_target_info {
    pub name: [c_char; 16],
    pub interval: i8,
    pub ewma_log: u8,

    /* Used internally by the kernel */
    pub est: *mut xt_rateest,
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
