/* SPDX-License-Identifier: GPL-2.0-or-later */

// Dependency supplied by the PowerPC architecture headers:
// #include <asm/cputable.h>

#[repr(C)]
pub struct raid6_calls {
    _private: [u8; 0],
}

unsafe extern "C" {
    pub static raid6_altivec1: raid6_calls;
    pub static raid6_altivec2: raid6_calls;
    pub static raid6_altivec4: raid6_calls;
    pub static raid6_altivec8: raid6_calls;
    pub static raid6_vpermxor1: raid6_calls;
    pub static raid6_vpermxor2: raid6_calls;
    pub static raid6_vpermxor4: raid6_calls;
    pub static raid6_vpermxor8: raid6_calls;

    fn raid6_algo_add_default();
    fn raid6_algo_add(algo: *const raid6_calls);
    fn cpu_has_feature(feature: u64) -> bool;
}

// CPU feature constants are provided by the PowerPC architecture headers.
extern "C" {
    static CPU_FTR_ALTIVEC: u64;
    static CPU_FTR_ALTIVEC_COMP: u64;
    static CPU_FTR_ARCH_207S: u64;
}

#[inline(always)]
pub unsafe fn arch_raid6_init() {
    raid6_algo_add_default();

    /* This assumes either all CPUs have Altivec or none does */
    if cpu_has_feature(CPU_FTR_ALTIVEC) {
        raid6_algo_add(&raid6_altivec1);
        raid6_algo_add(&raid6_altivec2);
        raid6_algo_add(&raid6_altivec4);
        raid6_algo_add(&raid6_altivec8);
    }
    if cpu_has_feature(CPU_FTR_ALTIVEC_COMP)
        && cpu_has_feature(CPU_FTR_ARCH_207S)
    {
        raid6_algo_add(&raid6_vpermxor1);
        raid6_algo_add(&raid6_vpermxor2);
        raid6_algo_add(&raid6_vpermxor4);
        raid6_algo_add(&raid6_vpermxor8);
    }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
