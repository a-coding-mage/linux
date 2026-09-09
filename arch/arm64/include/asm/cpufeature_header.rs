/* SPDX-License-Identifier: GPL-2.0-only */
/* Translated from asm/cpufeature.h. External kernel symbols are dependencies. */

pub const MAX_CPU_FEATURES: usize = 192;
pub const ARM64_SW_FEATURE_OVERRIDE_NOKASLR: u32 = 0;
pub const ARM64_SW_FEATURE_OVERRIDE_HVHE: u32 = 4;
pub const ARM64_SW_FEATURE_OVERRIDE_RODATA_OFF: u32 = 8;

#[repr(C)]
#[derive(Copy, Clone)]
pub enum ftr_type { FTR_EXACT, FTR_LOWER_SAFE, FTR_HIGHER_SAFE, FTR_HIGHER_OR_ZERO_SAFE }

pub const FTR_STRICT: bool = true;
pub const FTR_NONSTRICT: bool = false;
pub const FTR_SIGNED: bool = true;
pub const FTR_UNSIGNED: bool = false;
pub const FTR_VISIBLE: bool = true;
pub const FTR_HIDDEN: bool = false;

#[repr(C)]
pub struct arm64_ftr_bits {
    pub sign: bool, pub visible: bool, pub strict: bool, pub type_: ftr_type,
    pub shift: u8, pub width: u8, pub safe_val: i64,
}
#[repr(C)] pub struct arm64_ftr_override { pub val: u64, pub mask: u64 }
#[repr(C)]
pub struct arm64_ftr_reg {
    pub name: *const core::ffi::c_char, pub strict_mask: u64, pub user_mask: u64,
    pub sys_val: u64, pub user_val: u64, pub override_: *mut arm64_ftr_override,
    pub ftr_bits: *const arm64_ftr_bits,
}
extern "C" { pub static mut arm64_ftr_reg_ctrel0: arm64_ftr_reg; }

#[repr(C)]
pub struct arm64_cpu_capabilities {
    pub desc: *const core::ffi::c_char, pub capability: u16, pub type_: u16,
    pub matches: Option<unsafe extern "C" fn(*const arm64_cpu_capabilities, i32) -> bool>,
    pub cpu_enable: Option<unsafe extern "C" fn(*const arm64_cpu_capabilities)>,
    pub data: arm64_cpu_capability_data,
    pub match_list: *const arm64_cpu_capabilities, pub cpus: *const cpumask,
}
#[repr(C)] pub union arm64_cpu_capability_data {
    pub erratum: arm64_cpu_capability_erratum,
    pub midr_range_list: *const midr_range,
    pub feature: arm64_cpu_capability_feature,
}
#[repr(C)] pub struct arm64_cpu_capability_erratum {
    pub midr_range: midr_range,
    pub fixed_revs: *const arm64_midr_revidr,
}
#[repr(C)] pub struct arm64_midr_revidr { pub midr_rv: u32, pub revidr_mask: u32 }
#[repr(C)] pub struct arm64_cpu_capability_feature {
    pub sys_reg: u32, pub field_pos: u8, pub field_width: u8, pub min_field_value: u8,
    pub max_field_value: u8, pub hwcap_type: u8, pub sign: bool, pub hwcap: usize,
}

pub const ARM64_CPUCAP_SCOPE_LOCAL_CPU: u16 = 1 << 0;
pub const ARM64_CPUCAP_SCOPE_SYSTEM: u16 = 1 << 1;
pub const ARM64_CPUCAP_SCOPE_BOOT_CPU: u16 = 1 << 2;
pub const ARM64_CPUCAP_SCOPE_MASK: u16 = ARM64_CPUCAP_SCOPE_SYSTEM | ARM64_CPUCAP_SCOPE_LOCAL_CPU | ARM64_CPUCAP_SCOPE_BOOT_CPU;
pub const SCOPE_SYSTEM: u16 = ARM64_CPUCAP_SCOPE_SYSTEM;
pub const SCOPE_LOCAL_CPU: u16 = ARM64_CPUCAP_SCOPE_LOCAL_CPU;
pub const SCOPE_BOOT_CPU: u16 = ARM64_CPUCAP_SCOPE_BOOT_CPU;
pub const SCOPE_ALL: u16 = ARM64_CPUCAP_SCOPE_MASK;
pub const ARM64_CPUCAP_PERMITTED_FOR_LATE_CPU: u16 = 1 << 4;
pub const ARM64_CPUCAP_OPTIONAL_FOR_LATE_CPU: u16 = 1 << 5;
pub const ARM64_CPUCAP_PANIC_ON_CONFLICT: u16 = 1 << 6;
pub const ARM64_CPUCAP_MATCH_ALL_EARLY_CPUS: u16 = 1 << 7;
pub const ARM64_CPUCAP_LOCAL_CPU_ERRATUM: u16 = ARM64_CPUCAP_SCOPE_LOCAL_CPU | ARM64_CPUCAP_OPTIONAL_FOR_LATE_CPU;
pub const ARM64_CPUCAP_SYSTEM_FEATURE: u16 = ARM64_CPUCAP_SCOPE_SYSTEM | ARM64_CPUCAP_PERMITTED_FOR_LATE_CPU;
pub const ARM64_CPUCAP_WEAK_LOCAL_CPU_FEATURE: u16 = ARM64_CPUCAP_SCOPE_LOCAL_CPU | ARM64_CPUCAP_OPTIONAL_FOR_LATE_CPU | ARM64_CPUCAP_PERMITTED_FOR_LATE_CPU;
pub const ARM64_CPUCAP_EARLY_LOCAL_CPU_FEATURE: u16 = ARM64_CPUCAP_SCOPE_LOCAL_CPU | ARM64_CPUCAP_PERMITTED_FOR_LATE_CPU | ARM64_CPUCAP_MATCH_ALL_EARLY_CPUS;
pub const ARM64_CPUCAP_BOOT_RESTRICTED_CPU_LOCAL_FEATURE: u16 = ARM64_CPUCAP_SCOPE_LOCAL_CPU | ARM64_CPUCAP_OPTIONAL_FOR_LATE_CPU;
pub const ARM64_CPUCAP_STRICT_BOOT_CPU_FEATURE: u16 = ARM64_CPUCAP_SCOPE_BOOT_CPU | ARM64_CPUCAP_PANIC_ON_CONFLICT;
pub const ARM64_CPUCAP_BOOT_CPU_FEATURE: u16 = ARM64_CPUCAP_SCOPE_BOOT_CPU | ARM64_CPUCAP_PERMITTED_FOR_LATE_CPU;

extern "C" {
    pub static mut system_cpucaps: [usize; 0];
    pub static mut boot_cpucaps: [usize; 0];
    pub fn this_cpu_has_cap(cap: u32) -> bool;
    pub fn cpu_set_feature(num: u32); pub fn cpu_have_feature(num: u32) -> bool;
    pub fn cpu_get_elf_hwcap() -> usize; pub fn cpu_get_elf_hwcap2() -> usize; pub fn cpu_get_elf_hwcap3() -> usize;
    pub fn setup_boot_cpu_features(); pub fn setup_system_features(); pub fn setup_user_features();
    pub fn check_local_cpu_capabilities();
    pub fn read_sanitised_ftr_reg(id: u32) -> u64; pub fn __read_sysreg_by_encoding(sys_id: u32) -> u64;
    pub fn cpu_supports_bbml3() -> bool;
    pub fn do_emulate_mrs(regs: *mut pt_regs, sys_reg: u32, rt: u32) -> i32;
    pub fn try_emulate_mrs(regs: *mut pt_regs, isn: u32) -> bool;
    pub fn get_cpu_with_amu_feat() -> i32;
    pub fn arm64_ftr_safe_value(ftrp: *const arm64_ftr_bits, new_: i64, cur: i64) -> i64;
    pub fn get_arm64_ftr_reg(sys_id: u32) -> *mut arm64_ftr_reg;
}

// External kernel types and operations referenced by this header.
#[repr(C)] pub struct cpumask { _opaque: [u8; 0] }
#[repr(C)] pub struct midr_range { _opaque: [u8; 0] }
#[repr(C)] pub struct pt_regs { _opaque: [u8; 0] }

#[inline] pub unsafe fn cpucap_default_scope(cap: *const arm64_cpu_capabilities) -> i32 { ((*cap).type_ & ARM64_CPUCAP_SCOPE_MASK) as i32 }
#[inline] pub unsafe fn cpucap_match_all_early_cpus(cap: *const arm64_cpu_capabilities) -> bool { ((*cap).type_ & ARM64_CPUCAP_MATCH_ALL_EARLY_CPUS) != 0 }
#[inline] pub unsafe fn cpucap_multi_entry_cap_matches(entry: *const arm64_cpu_capabilities, scope: i32) -> bool {
    let mut caps = (*entry).match_list;
    while !caps.is_null() { if let Some(f) = (*caps).matches { if f(caps, scope) { return true; } } else { break; } caps = caps.add(1); }
    false
}
#[inline] pub fn is_vhe_hyp_code() -> bool { false }
#[inline] pub fn is_nvhe_hyp_code() -> bool { false }
#[inline] pub fn is_hyp_code() -> bool { is_vhe_hyp_code() || is_nvhe_hyp_code() }

#[inline] pub const fn cpuid_feature_extract_signed_field_width(features: u64, field: i32, width: i32) -> i32 { ((features << (64 - width - field)) as i64 >> (64 - width)) as i32 }
#[inline] pub const fn cpuid_feature_extract_signed_field(features: u64, field: i32) -> i32 { cpuid_feature_extract_signed_field_width(features, field, 4) }
#[inline] pub const fn cpuid_feature_extract_unsigned_field_width(features: u64, field: i32, width: i32) -> u32 { (features << (64 - width - field) >> (64 - width)) as u32 }
#[inline] pub const fn cpuid_feature_extract_unsigned_field(features: u64, field: i32) -> u32 { cpuid_feature_extract_unsigned_field_width(features, field, 4) }
#[inline] pub unsafe fn arm64_ftr_mask(f: *const arm64_ftr_bits) -> u64 { (((1u64 << ((*f).width as u32)) - 1) << (*f).shift) }
#[inline] pub unsafe fn arm64_ftr_reg_user_value(r: *const arm64_ftr_reg) -> u64 { (*r).user_val | ((*r).sys_val & (*r).user_mask) }
#[inline] pub unsafe fn cpuid_feature_extract_field_width(features: u64, field: i32, width: i32, sign: bool) -> i32 { if sign { cpuid_feature_extract_signed_field_width(features, field, width) } else { cpuid_feature_extract_unsigned_field_width(features, field, width) as i32 } }
#[inline] pub unsafe fn cpuid_feature_extract_field(features: u64, field: i32, sign: bool) -> i32 { cpuid_feature_extract_field_width(features, field, 4, sign) }
#[inline] pub unsafe fn arm64_ftr_value(f: *const arm64_ftr_bits, val: u64) -> i64 { cpuid_feature_extract_field_width(val, (*f).shift as i32, (*f).width as i32, (*f).sign) as i64 }

#[inline] pub fn id_aa64mmfr0_mixed_endian_el0(mmfr0: u64) -> bool { cpuid_feature_extract_unsigned_field(mmfr0, ID_AA64MMFR0_EL1_BIGEND_SHIFT) == 1 || cpuid_feature_extract_unsigned_field(mmfr0, ID_AA64MMFR0_EL1_BIGENDEL0_SHIFT) == 1 }
#[inline] pub fn id_aa64pfr0_32bit_el1(pfr0: u64) -> bool { cpuid_feature_extract_unsigned_field(pfr0, ID_AA64PFR0_EL1_EL1_SHIFT) == ID_AA64PFR0_EL1_EL1_AARCH32 }
#[inline] pub fn id_aa64pfr0_32bit_el0(pfr0: u64) -> bool { cpuid_feature_extract_unsigned_field(pfr0, ID_AA64PFR0_EL1_EL0_SHIFT) == ID_AA64PFR0_EL1_EL0_AARCH32 }
#[inline] pub fn id_aa64pfr0_sve(pfr0: u64) -> bool { cpuid_feature_extract_unsigned_field(pfr0, ID_AA64PFR0_EL1_SVE_SHIFT) > 0 }
#[inline] pub fn id_aa64pfr1_sme(pfr1: u64) -> bool { cpuid_feature_extract_unsigned_field(pfr1, ID_AA64PFR1_EL1_SME_SHIFT) > 0 }
#[inline] pub fn id_aa64pfr0_mpam(pfr0: u64) -> bool { cpuid_feature_extract_unsigned_field(pfr0, ID_AA64PFR0_EL1_MPAM_SHIFT) > 0 }
#[inline] pub fn id_aa64pfr1_mpamfrac(pfr1: u64) -> bool { cpuid_feature_extract_unsigned_field(pfr1, ID_AA64PFR1_EL1_MPAM_frac_SHIFT) > 0 }
#[inline] pub fn id_aa64pfr1_mte(pfr1: u64) -> bool { cpuid_feature_extract_unsigned_field(pfr1, ID_AA64PFR1_EL1_MTE_SHIFT) >= ID_AA64PFR1_EL1_MTE_MTE2 }

#[inline] pub fn id_aa64mmfr0_parange_to_phys_shift(parange: i32) -> u32 { match parange { ID_AA64MMFR0_EL1_PARANGE_32 => 32, ID_AA64MMFR0_EL1_PARANGE_36 => 36, ID_AA64MMFR0_EL1_PARANGE_40 => 40, ID_AA64MMFR0_EL1_PARANGE_42 => 42, ID_AA64MMFR0_EL1_PARANGE_44 => 44, ID_AA64MMFR0_EL1_PARANGE_48 => 48, ID_AA64MMFR0_EL1_PARANGE_52 => 52, _ => CONFIG_ARM64_PA_BITS } }
#[inline] pub fn get_vmid_bits(mmfr1: u64) -> u32 { if cpuid_feature_extract_unsigned_field(mmfr1, ID_AA64MMFR1_EL1_VMIDBits_SHIFT) == ID_AA64MMFR1_EL1_VMIDBits_16 { 16 } else { 8 } }

extern "C" {
    pub static mut id_aa64mmfr0_override: arm64_ftr_override; pub static mut id_aa64mmfr1_override: arm64_ftr_override; pub static mut id_aa64mmfr2_override: arm64_ftr_override; pub static mut id_aa64mmfr4_override: arm64_ftr_override;
    pub static mut id_aa64pfr0_override: arm64_ftr_override; pub static mut id_aa64pfr1_override: arm64_ftr_override; pub static mut id_aa64zfr0_override: arm64_ftr_override; pub static mut id_aa64smfr0_override: arm64_ftr_override;
    pub static mut id_aa64isar1_override: arm64_ftr_override; pub static mut id_aa64isar2_override: arm64_ftr_override; pub static mut arm64_sw_feature_override: arm64_ftr_override;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
