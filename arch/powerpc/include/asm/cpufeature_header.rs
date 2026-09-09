/* SPDX-License-Identifier: GPL-2.0-or-later */
/*
 * CPU feature definitions for module loading, used by
 * module_cpu_feature_match(), see asm/cputable.h for powerpc CPU features.
 *
 * Copyright 2016 Alastair D'Silva, IBM Corporation.
 */

// Dependency intent: the declarations below correspond to symbols supplied
// by asm/cputable.h and the surrounding PowerPC implementation.

/* Keep these in step with powerpc/include/asm/cputable.h */
pub const MAX_CPU_FEATURES: u32 = 2 * 32;

/*
 * Currently we don't have a need for any of the feature bits defined in
 * cpu_user_features. When we do, they should be defined such as:
 *
 * #define PPC_MODULE_FEATURE_32 (ilog2(PPC_FEATURE_32))
 */

unsafe extern "C" {
    pub fn ilog2(value: u64) -> u32;
}

pub const PPC_MODULE_FEATURE_VEC_CRYPTO: u32 = unsafe {
    32 + ilog2(PPC_FEATURE2_VEC_CRYPTO as u64)
};
pub const PPC_MODULE_FEATURE_P10: u32 = unsafe {
    32 + ilog2(PPC_FEATURE2_ARCH_3_1 as u64)
};

pub const fn cpu_feature<T>(x: T) -> T {
    x
}

#[repr(C)]
pub struct CpuSpec {
    pub cpu_user_features: u64,
    pub cpu_user_features2: u64,
}

unsafe extern "C" {
    pub static cur_cpu_spec: *const CpuSpec;
}

pub unsafe fn cpu_have_feature(num: u32) -> bool {
    if num < 32 {
        ((*cur_cpu_spec).cpu_user_features & (1u64 << num)) != 0
    } else {
        ((*cur_cpu_spec).cpu_user_features2 & (1u64 << (num - 32))) != 0
    }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
