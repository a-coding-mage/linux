/* SPDX-License-Identifier: GPL-2.0-or-later */

// Dependency supplied by the Linux CPU feature headers: cpu_has_vx().

// Opaque declarations supplied by the RAID6 implementation.
#[repr(C)]
pub struct raid6_calls {
    _private: [u8; 0],
}

#[repr(C)]
pub struct raid6_recov_calls {
    _private: [u8; 0],
}

extern "C" {
    pub static raid6_s390vx8: raid6_calls;
    pub static raid6_recov_s390xc: raid6_recov_calls;

    pub fn cpu_has_vx() -> bool;
    pub fn raid6_algo_add(alg: *const raid6_calls);
    pub fn raid6_algo_add_default();
    pub fn raid6_recov_algo_add(alg: *const raid6_recov_calls);
}

#[inline(always)]
// Corresponds to the kernel's __init annotation.
unsafe fn arch_raid6_init() {
    if cpu_has_vx() {
        raid6_algo_add(&raid6_s390vx8 as *const raid6_calls);
    } else {
        raid6_algo_add_default();
    }
    raid6_recov_algo_add(&raid6_recov_s390xc as *const raid6_recov_calls);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
