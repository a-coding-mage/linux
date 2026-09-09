/* SPDX-License-Identifier: GPL-2.0 */

/*
 * Can't use <linux/bitfield.h> because it generates expressions that
 * cannot be used in structure initializers. Bitfield construction
 * here must match the union in struct cpuinfo_86.
 *
 * The included kernel definitions (x86_cpu_id, vendor/family/model,
 * feature, stepping, and CPU type constants) are supplied externally.
 */

pub const VFM_MODEL_BIT: u32 = 0;
pub const VFM_FAMILY_BIT: u32 = 8;
pub const VFM_VENDOR_BIT: u32 = 16;
pub const VFM_RSVD_BIT: u32 = 24;

pub const VFM_MODEL_MASK: u32 = ((1u32 << (VFM_FAMILY_BIT - VFM_MODEL_BIT)) - 1) << VFM_MODEL_BIT;
pub const VFM_FAMILY_MASK: u32 = ((1u32 << (VFM_VENDOR_BIT - VFM_FAMILY_BIT)) - 1) << VFM_FAMILY_BIT;
pub const VFM_VENDOR_MASK: u32 = ((1u32 << (VFM_RSVD_BIT - VFM_VENDOR_BIT)) - 1) << VFM_VENDOR_BIT;

#[macro_export]
macro_rules! VFM_MODEL {
    ($vfm:expr) => { (($vfm & $crate::VFM_MODEL_MASK) >> $crate::VFM_MODEL_BIT) };
}
#[macro_export]
macro_rules! VFM_FAMILY {
    ($vfm:expr) => { (($vfm & $crate::VFM_FAMILY_MASK) >> $crate::VFM_FAMILY_BIT) };
}
#[macro_export]
macro_rules! VFM_VENDOR {
    ($vfm:expr) => { (($vfm & $crate::VFM_VENDOR_MASK) >> $crate::VFM_VENDOR_BIT) };
}
#[macro_export]
macro_rules! VFM_MAKE {
    ($vendor:expr, $family:expr, $model:expr) => {
        (($model << $crate::VFM_MODEL_BIT) |
         ($family << $crate::VFM_FAMILY_BIT) |
         ($vendor << $crate::VFM_VENDOR_BIT))
    };
}

/* Centaur FAM6 models */
pub const X86_CENTAUR_FAM6_C7_A: u32 = 0xa;
pub const X86_CENTAUR_FAM6_C7_D: u32 = 0xd;
pub const X86_CENTAUR_FAM6_NANO: u32 = 0xf;

/* x86_cpu_id::flags */
pub const X86_CPU_ID_FLAG_ENTRY_VALID: u32 = 1 << 0;

#[macro_export]
macro_rules! X86_MATCH_CPU {
    ($vendor:expr, $family:expr, $model:expr, $steppings:expr, $feature:expr, $type:expr, $data:expr) => {
        x86_cpu_id {
            vendor: $vendor,
            family: $family,
            model: $model,
            steppings: $steppings,
            feature: $feature,
            flags: $crate::X86_CPU_ID_FLAG_ENTRY_VALID,
            type_: $type,
            driver_data: $data as ::core::ffi::c_ulong,
        }
    };
}

#[macro_export]
macro_rules! X86_MATCH_VENDOR_FAM_FEATURE {
    ($vendor:expr, $family:expr, $feature:expr, $data:expr) => {
        $crate::X86_MATCH_CPU!($vendor, $family, X86_MODEL_ANY, X86_STEPPING_ANY,
                               $feature, X86_CPU_TYPE_ANY, $data)
    };
}
#[macro_export]
macro_rules! X86_MATCH_VENDOR_FEATURE {
    ($vendor:expr, $feature:expr, $data:expr) => {
        $crate::X86_MATCH_CPU!($vendor, X86_FAMILY_ANY, X86_MODEL_ANY,
                               X86_STEPPING_ANY, $feature, X86_CPU_TYPE_ANY, $data)
    };
}
#[macro_export]
macro_rules! X86_MATCH_FEATURE {
    ($feature:expr, $data:expr) => {
        $crate::X86_MATCH_CPU!(X86_VENDOR_ANY, X86_FAMILY_ANY, X86_MODEL_ANY,
                               X86_STEPPING_ANY, $feature, X86_CPU_TYPE_ANY, $data)
    };
}
#[macro_export]
macro_rules! X86_MATCH_VENDOR_FAM_MODEL {
    ($vendor:expr, $family:expr, $model:expr, $data:expr) => {
        $crate::X86_MATCH_CPU!($vendor, $family, $model, X86_STEPPING_ANY,
                               X86_FEATURE_ANY, X86_CPU_TYPE_ANY, $data)
    };
}
#[macro_export]
macro_rules! X86_MATCH_VENDOR_FAM {
    ($vendor:expr, $family:expr, $data:expr) => {
        $crate::X86_MATCH_CPU!($vendor, $family, X86_MODEL_ANY, X86_STEPPING_ANY,
                               X86_FEATURE_ANY, X86_CPU_TYPE_ANY, $data)
    };
}
#[macro_export]
macro_rules! X86_MATCH_VFM {
    ($vfm:expr, $data:expr) => {
        $crate::X86_MATCH_CPU!(VFM_VENDOR!($vfm), VFM_FAMILY!($vfm), VFM_MODEL!($vfm),
                               X86_STEPPING_ANY, X86_FEATURE_ANY, X86_CPU_TYPE_ANY, $data)
    };
}

#[macro_export]
macro_rules! __X86_STEPPINGS {
    ($mins:expr, $maxs:expr) => { (((1u32 << ($maxs - $mins + 1)) - 1) << $mins) };
}
#[macro_export]
macro_rules! X86_MATCH_VFM_STEPS {
    ($vfm:expr, $min_step:expr, $max_step:expr, $data:expr) => {
        $crate::X86_MATCH_CPU!(VFM_VENDOR!($vfm), VFM_FAMILY!($vfm), VFM_MODEL!($vfm),
                               $crate::__X86_STEPPINGS!($min_step, $max_step),
                               X86_FEATURE_ANY, X86_CPU_TYPE_ANY, $data)
    };
}
#[macro_export]
macro_rules! X86_MATCH_VFM_FEATURE {
    ($vfm:expr, $feature:expr, $data:expr) => {
        $crate::X86_MATCH_CPU!(VFM_VENDOR!($vfm), VFM_FAMILY!($vfm), VFM_MODEL!($vfm),
                               X86_STEPPING_ANY, $feature, X86_CPU_TYPE_ANY, $data)
    };
}
#[macro_export]
macro_rules! X86_MATCH_VFM_CPU_TYPE {
    ($vfm:expr, $type:expr, $data:expr) => {
        $crate::X86_MATCH_CPU!(VFM_VENDOR!($vfm), VFM_FAMILY!($vfm), VFM_MODEL!($vfm),
                               X86_STEPPING_ANY, X86_FEATURE_ANY, $type, $data)
    };
}

extern "C" {
    pub fn x86_match_cpu(match_: *const x86_cpu_id) -> *const x86_cpu_id;
    pub fn x86_match_min_microcode_rev(table: *const x86_cpu_id) -> bool;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
