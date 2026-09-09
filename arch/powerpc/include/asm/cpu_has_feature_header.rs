/* SPDX-License-Identifier: GPL-2.0 */

// The C header guard and assembler-only condition have no executable Rust
// equivalent.  This file corresponds to the non-assembler section.

use core::ffi::c_ulong;

// Supplied by the corresponding architecture and kernel dependencies.
extern "C" {
    pub static cur_cpu_spec: *const CpuSpec;
    pub static cpu_feature_keys: [static_key_true; NUM_CPU_FTR_KEYS];
    pub static static_key_feature_checks_initialized: bool;

    pub fn printk(format: *const u8, ...);
    pub fn dump_stack();
    pub fn static_branch_likely(key: *const static_key_true) -> bool;
}

#[repr(C)]
pub struct CpuSpec {
    pub cpu_features: c_ulong,
}

#[repr(C)]
pub struct static_key_true {
    _private: [u8; 0],
}

// These names are supplied by asm/cputable.h.
extern "C" {
    static CPU_FTRS_ALWAYS: c_ulong;
    static CPU_FTRS_POSSIBLE: c_ulong;
}

pub const NUM_CPU_FTR_KEYS: usize = (c_ulong::BITS as usize);

#[inline(always)]
pub unsafe fn early_cpu_has_feature(feature: c_ulong) -> bool {
    (CPU_FTRS_ALWAYS & feature != 0)
        || (CPU_FTRS_POSSIBLE & (*cur_cpu_spec).cpu_features & feature != 0)
}

#[cfg(feature = "CONFIG_JUMP_LABEL_FEATURE_CHECKS")]
#[inline(always)]
pub unsafe fn cpu_has_feature(feature: c_ulong) -> bool {
    // BUILD_BUG_ON(!__builtin_constant_p(feature));
    // BUILD_BUG_ON(__builtin_popcountl(feature) > 1);

    #[cfg(feature = "CONFIG_JUMP_LABEL_FEATURE_CHECK_DEBUG")]
    {
        if !static_key_feature_checks_initialized {
            let warning = b"Warning! cpu_has_feature() used prior to jump label init!\n\0";
            printk(warning.as_ptr());
            dump_stack();
            return early_cpu_has_feature(feature);
        }
    }

    if CPU_FTRS_ALWAYS & feature != 0 {
        return true;
    }

    if CPU_FTRS_POSSIBLE & feature == 0 {
        return false;
    }

    let i = feature.trailing_zeros() as usize;
    static_branch_likely(&cpu_feature_keys[i])
}

#[cfg(not(feature = "CONFIG_JUMP_LABEL_FEATURE_CHECKS"))]
#[inline(always)]
pub unsafe fn cpu_has_feature(feature: c_ulong) -> bool {
    early_cpu_has_feature(feature)
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
