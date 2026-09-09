/* SPDX-License-Identifier: GPL-2.0 */

// Dependencies supplied by the surrounding kernel translation.

/*
 * Define a KVM-only feature flag.
 *
 * For features that are scattered by cpufeatures.h, __feature_translate() also
 * needs to be updated to translate the kernel-defined feature into the
 * KVM-defined feature.
 */
#[inline(always)]
pub const fn kvm_x86_feature(w: u32, f: u32) -> u32 { w * 32 + f }

pub const KVM_X86_FEATURE_SGX1: u32 = kvm_x86_feature(CPUID_12_EAX, 0);
pub const KVM_X86_FEATURE_SGX2: u32 = kvm_x86_feature(CPUID_12_EAX, 1);
pub const KVM_X86_FEATURE_SGX_EDECCSSA: u32 = kvm_x86_feature(CPUID_12_EAX, 11);
pub const KVM_X86_FEATURE_MSR_IMM: u32 = kvm_x86_feature(CPUID_7_1_ECX, 5);

pub const X86_FEATURE_AVX_VNNI_INT8: u32 = kvm_x86_feature(CPUID_7_1_EDX, 4);
pub const X86_FEATURE_AVX_NE_CONVERT: u32 = kvm_x86_feature(CPUID_7_1_EDX, 5);
pub const X86_FEATURE_AMX_COMPLEX: u32 = kvm_x86_feature(CPUID_7_1_EDX, 8);
pub const X86_FEATURE_AVX_VNNI_INT16: u32 = kvm_x86_feature(CPUID_7_1_EDX, 10);
pub const X86_FEATURE_PREFETCHITI: u32 = kvm_x86_feature(CPUID_7_1_EDX, 14);
pub const X86_FEATURE_AVX10: u32 = kvm_x86_feature(CPUID_7_1_EDX, 19);
pub const X86_FEATURE_INTEL_PSFD: u32 = kvm_x86_feature(CPUID_7_2_EDX, 0);
pub const X86_FEATURE_IPRED_CTRL: u32 = kvm_x86_feature(CPUID_7_2_EDX, 1);
pub const KVM_X86_FEATURE_RRSBA_CTRL: u32 = kvm_x86_feature(CPUID_7_2_EDX, 2);
pub const X86_FEATURE_DDPD_U: u32 = kvm_x86_feature(CPUID_7_2_EDX, 3);
pub const KVM_X86_FEATURE_BHI_CTRL: u32 = kvm_x86_feature(CPUID_7_2_EDX, 4);
pub const X86_FEATURE_MCDT_NO: u32 = kvm_x86_feature(CPUID_7_2_EDX, 5);

pub const X86_FEATURE_AMX_INT8_ALIAS: u32 = kvm_x86_feature(CPUID_1E_1_EAX, 0);
pub const X86_FEATURE_AMX_BF16_ALIAS: u32 = kvm_x86_feature(CPUID_1E_1_EAX, 1);
pub const X86_FEATURE_AMX_COMPLEX_ALIAS: u32 = kvm_x86_feature(CPUID_1E_1_EAX, 2);
pub const X86_FEATURE_AMX_FP16_ALIAS: u32 = kvm_x86_feature(CPUID_1E_1_EAX, 3);
pub const X86_FEATURE_AMX_FP8: u32 = kvm_x86_feature(CPUID_1E_1_EAX, 4);
pub const X86_FEATURE_AMX_TF32: u32 = kvm_x86_feature(CPUID_1E_1_EAX, 6);
pub const X86_FEATURE_AMX_AVX512: u32 = kvm_x86_feature(CPUID_1E_1_EAX, 7);
pub const X86_FEATURE_AMX_MOVRS: u32 = kvm_x86_feature(CPUID_1E_1_EAX, 8);
pub const X86_FEATURE_AVX10_128: u32 = kvm_x86_feature(CPUID_24_0_EBX, 16);
pub const X86_FEATURE_AVX10_256: u32 = kvm_x86_feature(CPUID_24_0_EBX, 17);
pub const X86_FEATURE_AVX10_512: u32 = kvm_x86_feature(CPUID_24_0_EBX, 18);
pub const X86_FEATURE_AVX10_VNNI_INT: u32 = kvm_x86_feature(CPUID_24_1_ECX, 2);
pub const KVM_X86_FEATURE_CONSTANT_TSC: u32 = kvm_x86_feature(CPUID_8000_0007_EDX, 8);
pub const KVM_X86_FEATURE_PERFMON_V2: u32 = kvm_x86_feature(CPUID_8000_0022_EAX, 0);
pub const KVM_X86_FEATURE_TSA_SQ_NO: u32 = kvm_x86_feature(CPUID_8000_0021_ECX, 1);
pub const KVM_X86_FEATURE_TSA_L1_NO: u32 = kvm_x86_feature(CPUID_8000_0021_ECX, 2);

#[repr(C)]
#[derive(Copy, Clone)]
pub struct cpuid_reg { pub function: u32, pub index: u32, pub reg: i32 }

// C designated initializers are represented by the lookup function below.
#[inline(always)]
pub fn reverse_cpuid(x86_leaf: usize) -> cpuid_reg {
    match x86_leaf {
        CPUID_1_EDX => cpuid_reg { function: 1, index: 0, reg: CPUID_EDX },
        CPUID_8000_0001_EDX => cpuid_reg { function: 0x80000001, index: 0, reg: CPUID_EDX },
        CPUID_8086_0001_EDX => cpuid_reg { function: 0x80860001, index: 0, reg: CPUID_EDX },
        CPUID_1_ECX => cpuid_reg { function: 1, index: 0, reg: CPUID_ECX },
        CPUID_C000_0001_EDX => cpuid_reg { function: 0xc0000001, index: 0, reg: CPUID_EDX },
        CPUID_8000_0001_ECX => cpuid_reg { function: 0x80000001, index: 0, reg: CPUID_ECX },
        CPUID_7_0_EBX => cpuid_reg { function: 7, index: 0, reg: CPUID_EBX },
        CPUID_D_1_EAX => cpuid_reg { function: 0xd, index: 1, reg: CPUID_EAX },
        CPUID_8000_0008_EBX => cpuid_reg { function: 0x80000008, index: 0, reg: CPUID_EBX },
        CPUID_6_EAX => cpuid_reg { function: 6, index: 0, reg: CPUID_EAX },
        CPUID_8000_000A_EDX => cpuid_reg { function: 0x8000000a, index: 0, reg: CPUID_EDX },
        CPUID_7_ECX => cpuid_reg { function: 7, index: 0, reg: CPUID_ECX },
        CPUID_7_EDX => cpuid_reg { function: 7, index: 0, reg: CPUID_EDX },
        CPUID_7_1_EAX => cpuid_reg { function: 7, index: 1, reg: CPUID_EAX },
        CPUID_12_EAX => cpuid_reg { function: 0x12, index: 0, reg: CPUID_EAX },
        CPUID_8000_001F_EAX => cpuid_reg { function: 0x8000001f, index: 0, reg: CPUID_EAX },
        CPUID_7_1_EDX => cpuid_reg { function: 7, index: 1, reg: CPUID_EDX },
        CPUID_8000_0007_EDX => cpuid_reg { function: 0x80000007, index: 0, reg: CPUID_EDX },
        CPUID_8000_0021_EAX => cpuid_reg { function: 0x80000021, index: 0, reg: CPUID_EAX },
        CPUID_8000_0022_EAX => cpuid_reg { function: 0x80000022, index: 0, reg: CPUID_EAX },
        CPUID_7_2_EDX => cpuid_reg { function: 7, index: 2, reg: CPUID_EDX },
        CPUID_24_0_EBX => cpuid_reg { function: 0x24, index: 0, reg: CPUID_EBX },
        CPUID_8000_0021_ECX => cpuid_reg { function: 0x80000021, index: 0, reg: CPUID_ECX },
        CPUID_7_1_ECX => cpuid_reg { function: 7, index: 1, reg: CPUID_ECX },
        CPUID_1E_1_EAX => cpuid_reg { function: 0x1e, index: 1, reg: CPUID_EAX },
        CPUID_24_1_ECX => cpuid_reg { function: 0x24, index: 1, reg: CPUID_ECX },
        _ => cpuid_reg { function: 0, index: 0, reg: 0 },
    }
}

#[inline(always)]
pub fn reverse_cpuid_check(x86_leaf: usize) {
    // BUILD_BUG_ON(NR_CPUID_WORDS != NCAPINTS) and the Linux-defined-word
    // exclusions are compile-time conditions in the C implementation.
    assert!(x86_leaf < ARRAY_SIZE_REVERSE_CPUID);
    assert!(reverse_cpuid(x86_leaf).function != 0);
}

#[inline(always)]
pub fn __feature_translate(x86_feature: i32) -> u32 {
    match x86_feature {
        X86_FEATURE_SGX1 => KVM_X86_FEATURE_SGX1,
        X86_FEATURE_SGX2 => KVM_X86_FEATURE_SGX2,
        X86_FEATURE_SGX_EDECCSSA => KVM_X86_FEATURE_SGX_EDECCSSA,
        X86_FEATURE_CONSTANT_TSC => KVM_X86_FEATURE_CONSTANT_TSC,
        X86_FEATURE_PERFMON_V2 => KVM_X86_FEATURE_PERFMON_V2,
        X86_FEATURE_RRSBA_CTRL => KVM_X86_FEATURE_RRSBA_CTRL,
        X86_FEATURE_BHI_CTRL => KVM_X86_FEATURE_BHI_CTRL,
        X86_FEATURE_TSA_SQ_NO => KVM_X86_FEATURE_TSA_SQ_NO,
        X86_FEATURE_TSA_L1_NO => KVM_X86_FEATURE_TSA_L1_NO,
        X86_FEATURE_MSR_IMM => KVM_X86_FEATURE_MSR_IMM,
        _ => x86_feature as u32,
    }
}

#[inline(always)] pub fn __feature_leaf(x: i32) -> usize { (__feature_translate(x) / 32) as usize }
#[inline(always)] pub fn __feature_bit(x: i32) -> u32 { 1u32 << (__feature_translate(x) & 31) }
#[inline(always)] pub fn feature_bit(name: i32) -> u32 { __feature_bit(name) }
#[inline(always)] pub fn x86_feature_cpuid(x: u32) -> cpuid_reg { reverse_cpuid(__feature_leaf(x as i32)) }

#[inline(always)]
pub unsafe fn __cpuid_entry_get_reg(entry: *mut kvm_cpuid_entry2, reg: u32) -> *mut u32 {
    match reg {
        CPUID_EAX => &mut (*entry).eax,
        CPUID_EBX => &mut (*entry).ebx,
        CPUID_ECX => &mut (*entry).ecx,
        CPUID_EDX => &mut (*entry).edx,
        _ => core::ptr::null_mut(),
    }
}

#[inline(always)]
pub unsafe fn cpuid_entry_get_reg(entry: *mut kvm_cpuid_entry2, feature: u32) -> *mut u32 {
    __cpuid_entry_get_reg(entry, reverse_cpuid(__feature_leaf(feature as i32)).reg as u32)
}

#[inline(always)] pub unsafe fn cpuid_entry_get(e: *mut kvm_cpuid_entry2, f: u32) -> u32 { *cpuid_entry_get_reg(e, f) & __feature_bit(f as i32) }
#[inline(always)] pub unsafe fn cpuid_entry_has(e: *mut kvm_cpuid_entry2, f: u32) -> bool { cpuid_entry_get(e, f) != 0 }
#[inline(always)] pub unsafe fn cpuid_entry_clear(e: *mut kvm_cpuid_entry2, f: u32) { *cpuid_entry_get_reg(e, f) &= !__feature_bit(f as i32); }
#[inline(always)] pub unsafe fn cpuid_entry_set(e: *mut kvm_cpuid_entry2, f: u32) { *cpuid_entry_get_reg(e, f) |= __feature_bit(f as i32); }
#[inline(always)] pub unsafe fn cpuid_entry_change(e: *mut kvm_cpuid_entry2, f: u32, set: bool) { if set { cpuid_entry_set(e, f) } else { cpuid_entry_clear(e, f) } }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
