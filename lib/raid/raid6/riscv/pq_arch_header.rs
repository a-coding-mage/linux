/* SPDX-License-Identifier: GPL-2.0-or-later */

// Dependency supplied by <asm/vector.h>.
unsafe extern "C" {
    pub fn has_vector() -> bool;
}

// Opaque types supplied by the surrounding raid6 implementation.
#[repr(C)]
pub struct raid6_calls {
    _private: [u8; 0],
}

#[repr(C)]
pub struct raid6_recov_calls {
    _private: [u8; 0],
}

unsafe extern "C" {
    pub static raid6_rvvx1: raid6_calls;
    pub static raid6_rvvx2: raid6_calls;
    pub static raid6_rvvx4: raid6_calls;
    pub static raid6_rvvx8: raid6_calls;
    pub static raid6_recov_rvv: raid6_recov_calls;

    pub fn raid6_algo_add_default();
    pub fn raid6_algo_add(algo: *const raid6_calls);
    pub fn raid6_recov_algo_add(algo: *const raid6_recov_calls);
}

// C: static __always_inline void __init arch_raid6_init(void)
#[inline(always)]
pub unsafe fn arch_raid6_init() {
    raid6_algo_add_default();
    if has_vector() {
        raid6_algo_add(&raid6_rvvx1);
        raid6_algo_add(&raid6_rvvx2);
        raid6_algo_add(&raid6_rvvx4);
        raid6_algo_add(&raid6_rvvx8);
        raid6_recov_algo_add(&raid6_recov_rvv);
    }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
