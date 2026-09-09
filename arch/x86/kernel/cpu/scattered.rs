/*
 *	Routines to identify additional cpu features that are scattered in
 *	cpuid space.
 */

// Dependencies supplied by the surrounding kernel translation.

#[repr(C)]
struct CpuidBit {
    feature: u16,
    reg: u8,
    bit: u8,
    level: u32,
    sub_leaf: u32,
}

/*
 * Please keep the leaf sorted by cpuid_bit.level for faster search.
 * X86_FEATURE_MBA is supported by both Intel and AMD. But the CPUID
 * levels are different and there is a separate entry for each.
 */
static CPUID_BITS: &[CpuidBit] = &[
    CpuidBit { feature: X86_FEATURE_APERFMPERF, reg: CPUID_ECX, bit: 0, level: 0x00000006, sub_leaf: 0 },
    CpuidBit { feature: X86_FEATURE_EPB, reg: CPUID_ECX, bit: 3, level: 0x00000006, sub_leaf: 0 },
    CpuidBit { feature: X86_FEATURE_INTEL_PPIN, reg: CPUID_EBX, bit: 0, level: 0x00000007, sub_leaf: 1 },
    CpuidBit { feature: X86_FEATURE_MSR_IMM, reg: CPUID_ECX, bit: 5, level: 0x00000007, sub_leaf: 1 },
    CpuidBit { feature: X86_FEATURE_APX, reg: CPUID_EDX, bit: 21, level: 0x00000007, sub_leaf: 1 },
    CpuidBit { feature: X86_FEATURE_RRSBA_CTRL, reg: CPUID_EDX, bit: 2, level: 0x00000007, sub_leaf: 2 },
    CpuidBit { feature: X86_FEATURE_BHI_CTRL, reg: CPUID_EDX, bit: 4, level: 0x00000007, sub_leaf: 2 },
    CpuidBit { feature: X86_FEATURE_CQM_LLC, reg: CPUID_EDX, bit: 1, level: 0x0000000f, sub_leaf: 0 },
    CpuidBit { feature: X86_FEATURE_CQM_OCCUP_LLC, reg: CPUID_EDX, bit: 0, level: 0x0000000f, sub_leaf: 1 },
    CpuidBit { feature: X86_FEATURE_CQM_MBM_TOTAL, reg: CPUID_EDX, bit: 1, level: 0x0000000f, sub_leaf: 1 },
    CpuidBit { feature: X86_FEATURE_CQM_MBM_LOCAL, reg: CPUID_EDX, bit: 2, level: 0x0000000f, sub_leaf: 1 },
    CpuidBit { feature: X86_FEATURE_CAT_L3, reg: CPUID_EBX, bit: 1, level: 0x00000010, sub_leaf: 0 },
    CpuidBit { feature: X86_FEATURE_CAT_L2, reg: CPUID_EBX, bit: 2, level: 0x00000010, sub_leaf: 0 },
    CpuidBit { feature: X86_FEATURE_MBA, reg: CPUID_EBX, bit: 3, level: 0x00000010, sub_leaf: 0 },
    CpuidBit { feature: X86_FEATURE_CDP_L3, reg: CPUID_ECX, bit: 2, level: 0x00000010, sub_leaf: 1 },
    CpuidBit { feature: X86_FEATURE_CDP_L2, reg: CPUID_ECX, bit: 2, level: 0x00000010, sub_leaf: 2 },
    CpuidBit { feature: X86_FEATURE_PER_THREAD_MBA, reg: CPUID_ECX, bit: 0, level: 0x00000010, sub_leaf: 3 },
    CpuidBit { feature: X86_FEATURE_SGX1, reg: CPUID_EAX, bit: 0, level: 0x00000012, sub_leaf: 0 },
    CpuidBit { feature: X86_FEATURE_SGX2, reg: CPUID_EAX, bit: 1, level: 0x00000012, sub_leaf: 0 },
    CpuidBit { feature: X86_FEATURE_SGX_EUPDATESVN, reg: CPUID_EAX, bit: 10, level: 0x00000012, sub_leaf: 0 },
    CpuidBit { feature: X86_FEATURE_SGX_EDECCSSA, reg: CPUID_EAX, bit: 11, level: 0x00000012, sub_leaf: 0 },
    CpuidBit { feature: X86_FEATURE_OVERFLOW_RECOV, reg: CPUID_EBX, bit: 0, level: 0x80000007, sub_leaf: 0 },
    CpuidBit { feature: X86_FEATURE_SUCCOR, reg: CPUID_EBX, bit: 1, level: 0x80000007, sub_leaf: 0 },
    CpuidBit { feature: X86_FEATURE_SMCA, reg: CPUID_EBX, bit: 3, level: 0x80000007, sub_leaf: 0 },
    CpuidBit { feature: X86_FEATURE_HW_PSTATE, reg: CPUID_EDX, bit: 7, level: 0x80000007, sub_leaf: 0 },
    CpuidBit { feature: X86_FEATURE_CPB, reg: CPUID_EDX, bit: 9, level: 0x80000007, sub_leaf: 0 },
    CpuidBit { feature: X86_FEATURE_PROC_FEEDBACK, reg: CPUID_EDX, bit: 11, level: 0x80000007, sub_leaf: 0 },
    CpuidBit { feature: X86_FEATURE_AMD_FAST_CPPC, reg: CPUID_EDX, bit: 15, level: 0x80000007, sub_leaf: 0 },
    CpuidBit { feature: X86_FEATURE_CPPC_PERF_PRIO, reg: CPUID_EDX, bit: 16, level: 0x80000007, sub_leaf: 0 },
    CpuidBit { feature: X86_FEATURE_MBA, reg: CPUID_EBX, bit: 6, level: 0x80000008, sub_leaf: 0 },
    CpuidBit { feature: X86_FEATURE_X2AVIC_EXT, reg: CPUID_ECX, bit: 6, level: 0x8000000a, sub_leaf: 0 },
    CpuidBit { feature: X86_FEATURE_COHERENCY_SFW_NO, reg: CPUID_EBX, bit: 31, level: 0x8000001f, sub_leaf: 0 },
    CpuidBit { feature: X86_FEATURE_SMBA, reg: CPUID_EBX, bit: 2, level: 0x80000020, sub_leaf: 0 },
    CpuidBit { feature: X86_FEATURE_BMEC, reg: CPUID_EBX, bit: 3, level: 0x80000020, sub_leaf: 0 },
    CpuidBit { feature: X86_FEATURE_ABMC, reg: CPUID_EBX, bit: 5, level: 0x80000020, sub_leaf: 0 },
    CpuidBit { feature: X86_FEATURE_SDCIAE, reg: CPUID_EBX, bit: 6, level: 0x80000020, sub_leaf: 0 },
    CpuidBit { feature: X86_FEATURE_AMD_WORKLOAD_CLASS, reg: CPUID_EAX, bit: 22, level: 0x80000021, sub_leaf: 0 },
    CpuidBit { feature: X86_FEATURE_TSA_SQ_NO, reg: CPUID_ECX, bit: 1, level: 0x80000021, sub_leaf: 0 },
    CpuidBit { feature: X86_FEATURE_TSA_L1_NO, reg: CPUID_ECX, bit: 2, level: 0x80000021, sub_leaf: 0 },
    CpuidBit { feature: X86_FEATURE_PERFMON_V2, reg: CPUID_EAX, bit: 0, level: 0x80000022, sub_leaf: 0 },
    CpuidBit { feature: X86_FEATURE_AMD_LBR_V2, reg: CPUID_EAX, bit: 1, level: 0x80000022, sub_leaf: 0 },
    CpuidBit { feature: X86_FEATURE_AMD_LBR_PMC_FREEZE, reg: CPUID_EAX, bit: 2, level: 0x80000022, sub_leaf: 0 },
    CpuidBit { feature: X86_FEATURE_AMD_HTR_CORES, reg: CPUID_EAX, bit: 30, level: 0x80000026, sub_leaf: 0 },
    CpuidBit { feature: 0, reg: 0, bit: 0, level: 0, sub_leaf: 0 },
];

pub unsafe fn init_scattered_cpuid_features(c: *mut cpuinfo_x86) {
    let mut regs = [0u32; 4];

    for cb in CPUID_BITS.iter() {
        if cb.feature == 0 {
            break;
        }

        /* Verify that the level is valid */
        let max_level = cpuid_eax(cb.level & 0xffff0000);
        if max_level < cb.level || max_level > (cb.level | 0xffff) {
            continue;
        }

        cpuid_count(cb.level, cb.sub_leaf, &mut regs[CPUID_EAX as usize],
            &mut regs[CPUID_EBX as usize], &mut regs[CPUID_ECX as usize],
            &mut regs[CPUID_EDX as usize]);

        if regs[cb.reg as usize] & (1u32 << cb.bit) != 0 {
            set_cpu_cap(c, cb.feature);
        }
    }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
