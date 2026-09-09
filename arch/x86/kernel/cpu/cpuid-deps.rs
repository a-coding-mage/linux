/* Declare dependencies between CPUIDs */

#[repr(C)]
struct CpuidDep {
    feature: u32,
    depends: u32,
}

/*
 * Table of CPUID features that depend on others.
 *
 * This only includes dependencies that can be usefully disabled, not
 * features part of the base set (like FPU).
 *
 * Note this all is not __init / __initdata because it can be
 * called from cpu hotplug. It shouldn't do anything in this case,
 * but it's difficult to tell that to the init reference checker.
 */
static CPUID_DEPS: &[CpuidDep] = &[
    CpuidDep { feature: X86_FEATURE_FXSR, depends: X86_FEATURE_FPU },
    CpuidDep { feature: X86_FEATURE_XSAVEOPT, depends: X86_FEATURE_XSAVE },
    CpuidDep { feature: X86_FEATURE_XSAVEC, depends: X86_FEATURE_XSAVE },
    CpuidDep { feature: X86_FEATURE_XSAVES, depends: X86_FEATURE_XSAVE },
    CpuidDep { feature: X86_FEATURE_AVX, depends: X86_FEATURE_XSAVE },
    CpuidDep { feature: X86_FEATURE_PKU, depends: X86_FEATURE_XSAVE },
    CpuidDep { feature: X86_FEATURE_MPX, depends: X86_FEATURE_XSAVE },
    CpuidDep { feature: X86_FEATURE_XGETBV1, depends: X86_FEATURE_XSAVE },
    CpuidDep { feature: X86_FEATURE_APX, depends: X86_FEATURE_XSAVE },
    CpuidDep { feature: X86_FEATURE_CMOV, depends: X86_FEATURE_FXSR },
    CpuidDep { feature: X86_FEATURE_MMX, depends: X86_FEATURE_FXSR },
    CpuidDep { feature: X86_FEATURE_MMXEXT, depends: X86_FEATURE_MMX },
    CpuidDep { feature: X86_FEATURE_FXSR_OPT, depends: X86_FEATURE_FXSR },
    CpuidDep { feature: X86_FEATURE_XSAVE, depends: X86_FEATURE_FXSR },
    CpuidDep { feature: X86_FEATURE_XMM, depends: X86_FEATURE_FXSR },
    CpuidDep { feature: X86_FEATURE_XMM2, depends: X86_FEATURE_XMM },
    CpuidDep { feature: X86_FEATURE_XMM3, depends: X86_FEATURE_XMM2 },
    CpuidDep { feature: X86_FEATURE_XMM4_1, depends: X86_FEATURE_XMM2 },
    CpuidDep { feature: X86_FEATURE_XMM4_2, depends: X86_FEATURE_XMM2 },
    CpuidDep { feature: X86_FEATURE_XMM3, depends: X86_FEATURE_XMM2 },
    CpuidDep { feature: X86_FEATURE_PCLMULQDQ, depends: X86_FEATURE_XMM2 },
    CpuidDep { feature: X86_FEATURE_SSSE3, depends: X86_FEATURE_XMM2 },
    CpuidDep { feature: X86_FEATURE_F16C, depends: X86_FEATURE_XMM2 },
    CpuidDep { feature: X86_FEATURE_AES, depends: X86_FEATURE_XMM2 },
    CpuidDep { feature: X86_FEATURE_SHA_NI, depends: X86_FEATURE_XMM2 },
    CpuidDep { feature: X86_FEATURE_GFNI, depends: X86_FEATURE_XMM2 },
    CpuidDep { feature: X86_FEATURE_AVX_VNNI, depends: X86_FEATURE_AVX },
    CpuidDep { feature: X86_FEATURE_FMA, depends: X86_FEATURE_AVX },
    CpuidDep { feature: X86_FEATURE_VAES, depends: X86_FEATURE_AVX },
    CpuidDep { feature: X86_FEATURE_VPCLMULQDQ, depends: X86_FEATURE_AVX },
    CpuidDep { feature: X86_FEATURE_AVX2, depends: X86_FEATURE_AVX },
    CpuidDep { feature: X86_FEATURE_AVX512F, depends: X86_FEATURE_AVX },
    CpuidDep { feature: X86_FEATURE_AVX512IFMA, depends: X86_FEATURE_AVX512F },
    CpuidDep { feature: X86_FEATURE_AVX512PF, depends: X86_FEATURE_AVX512F },
    CpuidDep { feature: X86_FEATURE_AVX512ER, depends: X86_FEATURE_AVX512F },
    CpuidDep { feature: X86_FEATURE_AVX512CD, depends: X86_FEATURE_AVX512F },
    CpuidDep { feature: X86_FEATURE_AVX512DQ, depends: X86_FEATURE_AVX512F },
    CpuidDep { feature: X86_FEATURE_AVX512BW, depends: X86_FEATURE_AVX512F },
    CpuidDep { feature: X86_FEATURE_AVX512VL, depends: X86_FEATURE_AVX512F },
    CpuidDep { feature: X86_FEATURE_AVX512VBMI, depends: X86_FEATURE_AVX512F },
    CpuidDep { feature: X86_FEATURE_AVX512_VBMI2, depends: X86_FEATURE_AVX512VL },
    CpuidDep { feature: X86_FEATURE_AVX512_VNNI, depends: X86_FEATURE_AVX512VL },
    CpuidDep { feature: X86_FEATURE_AVX512_BITALG, depends: X86_FEATURE_AVX512VL },
    CpuidDep { feature: X86_FEATURE_AVX512_4VNNIW, depends: X86_FEATURE_AVX512F },
    CpuidDep { feature: X86_FEATURE_AVX512_4FMAPS, depends: X86_FEATURE_AVX512F },
    CpuidDep { feature: X86_FEATURE_AVX512_VPOPCNTDQ, depends: X86_FEATURE_AVX512F },
    CpuidDep { feature: X86_FEATURE_AVX512_VP2INTERSECT, depends: X86_FEATURE_AVX512VL },
    CpuidDep { feature: X86_FEATURE_CQM_OCCUP_LLC, depends: X86_FEATURE_CQM_LLC },
    CpuidDep { feature: X86_FEATURE_CQM_MBM_TOTAL, depends: X86_FEATURE_CQM_LLC },
    CpuidDep { feature: X86_FEATURE_CQM_MBM_LOCAL, depends: X86_FEATURE_CQM_LLC },
    CpuidDep { feature: X86_FEATURE_BMEC, depends: X86_FEATURE_CQM_MBM_TOTAL },
    CpuidDep { feature: X86_FEATURE_BMEC, depends: X86_FEATURE_CQM_MBM_LOCAL },
    CpuidDep { feature: X86_FEATURE_SDCIAE, depends: X86_FEATURE_CAT_L3 },
    CpuidDep { feature: X86_FEATURE_AVX512_BF16, depends: X86_FEATURE_AVX512VL },
    CpuidDep { feature: X86_FEATURE_AVX512_FP16, depends: X86_FEATURE_AVX512BW },
    CpuidDep { feature: X86_FEATURE_ENQCMD, depends: X86_FEATURE_XSAVES },
    CpuidDep { feature: X86_FEATURE_PER_THREAD_MBA, depends: X86_FEATURE_MBA },
    CpuidDep { feature: X86_FEATURE_SGX_LC, depends: X86_FEATURE_SGX },
    CpuidDep { feature: X86_FEATURE_SGX1, depends: X86_FEATURE_SGX },
    CpuidDep { feature: X86_FEATURE_SGX2, depends: X86_FEATURE_SGX1 },
    CpuidDep { feature: X86_FEATURE_SGX_EUPDATESVN, depends: X86_FEATURE_SGX1 },
    CpuidDep { feature: X86_FEATURE_SGX_EDECCSSA, depends: X86_FEATURE_SGX1 },
    CpuidDep { feature: X86_FEATURE_XFD, depends: X86_FEATURE_XSAVES },
    CpuidDep { feature: X86_FEATURE_XFD, depends: X86_FEATURE_XGETBV1 },
    CpuidDep { feature: X86_FEATURE_AMX_TILE, depends: X86_FEATURE_XFD },
    CpuidDep { feature: X86_FEATURE_AMX_FP16, depends: X86_FEATURE_AMX_TILE },
    CpuidDep { feature: X86_FEATURE_AMX_BF16, depends: X86_FEATURE_AMX_TILE },
    CpuidDep { feature: X86_FEATURE_AMX_INT8, depends: X86_FEATURE_AMX_TILE },
    CpuidDep { feature: X86_FEATURE_SHSTK, depends: X86_FEATURE_XSAVES },
    CpuidDep { feature: X86_FEATURE_FRED, depends: X86_FEATURE_LKGS },
    CpuidDep { feature: X86_FEATURE_SPEC_CTRL_SSBD, depends: X86_FEATURE_SPEC_CTRL },
    CpuidDep { feature: X86_FEATURE_LASS, depends: X86_FEATURE_SMAP },
    CpuidDep { feature: X86_FEATURE_INVLPGB, depends: X86_FEATURE_PCID },
    CpuidDep { feature: 0, depends: 0 },
];

unsafe fn clear_feature(c: *mut cpuinfo_x86, feature: u32) {
    /*
     * Note: This could use the non atomic __*_bit() variants, but the
     * rest of the cpufeature code uses atomics as well, so keep it for
     * consistency. Cleanup all of it separately.
     */
    if c.is_null() {
        clear_cpu_cap(&mut boot_cpu_data, feature);
        set_bit(feature, cpu_caps_cleared as *mut usize);
    } else {
        clear_bit(feature, (*c).x86_capability.as_mut_ptr() as *mut usize);
    }
}

/* Take the capabilities and the BUG bits into account */
const MAX_FEATURE_BITS: usize = (NCAPINTS + NBUGINTS) * core::mem::size_of::<u32>() * 8;

unsafe fn do_clear_cpu_cap(c: *mut cpuinfo_x86, feature: u32) {
    let mut disable = [0usize; (MAX_FEATURE_BITS + (usize::BITS as usize) - 1) / (usize::BITS as usize)];

    if WARN_ON(feature >= MAX_FEATURE_BITS as u32) {
        return;
    }

    if boot_cpu_has(feature) {
        WARN_ON(alternatives_patched);
    }

    clear_feature(c, feature);

    /* Collect all features to disable, handling dependencies */
    disable.fill(0);
    __set_bit(feature, disable.as_mut_ptr());

    /* Loop until we get a stable state. */
    loop {
        let mut changed = false;
        for d in CPUID_DEPS {
            if d.feature == 0 {
                break;
            }
            if !test_bit(d.depends, disable.as_ptr()) {
                continue;
            }
            if __test_and_set_bit(d.feature, disable.as_mut_ptr()) {
                continue;
            }

            changed = true;
            clear_feature(c, d.feature);
        }
        if !changed {
            break;
        }
    }
}

pub unsafe fn clear_cpu_cap(c: *mut cpuinfo_x86, feature: u32) {
    do_clear_cpu_cap(c, feature);
}

pub unsafe fn setup_clear_cpu_cap(feature: u32) {
    do_clear_cpu_cap(core::ptr::null_mut(), feature);
}

/*
 * Return the feature "name" if available, otherwise return
 * the X86_FEATURE_* numerals to make it easier to identify the
 * feature.
 */
unsafe fn x86_feature_name(feature: u32, buf: *mut i8) -> *const i8 {
    if !x86_cap_flags[feature as usize].is_null() {
        return x86_cap_flags[feature as usize];
    }

    snprintf(buf, 16, c"%d*32+%2d".as_ptr(), feature / 32, feature % 32);
    buf
}

pub unsafe fn check_cpufeature_deps(c: *mut cpuinfo_x86) {
    let mut feature_buf = [0i8; 16];
    let mut depends_buf = [0i8; 16];

    for d in CPUID_DEPS {
        if d.feature == 0 {
            break;
        }
        if cpu_has(c, d.feature) && !cpu_has(c, d.depends) {
            /*
             * Only warn about the first unmet dependency on the
             * first CPU where it is encountered to avoid spamming
             * the kernel log.
             */
            pr_warn_once(
                c"x86 CPU feature dependency check failure: CPU%d has '%s' enabled but '%s' disabled. Kernel might be fine, but no guarantees.\n".as_ptr(),
                smp_processor_id(),
                x86_feature_name(d.feature, feature_buf.as_mut_ptr()),
                x86_feature_name(d.depends, depends_buf.as_mut_ptr()),
            );
        }
    }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
