// SPDX-License-Identifier: GPL-2.0

// Dependencies supplied by the surrounding kernel translation.

const ACE_PRESENT: u32 = 1 << 6;
const ACE_ENABLED: u32 = 1 << 7;
const ACE_FCR: u64 = 1 << 28; // MSR_VIA_FCR

const RNG_PRESENT: u32 = 1 << 2;
const RNG_ENABLED: u32 = 1 << 3;
const RNG_ENABLE: u64 = 1 << 6; // MSR_VIA_RNG

unsafe fn init_c3(c: *mut cpuinfo_x86) {
    let mut msr: u64;

    // Test for Centaur Extended Feature Flags presence
    if cpuid_eax(0xC0000000) >= 0xC0000001 {
        let tmp = cpuid_edx(0xC0000001);

        // enable ACE unit, if present and disabled
        if (tmp & (ACE_PRESENT | ACE_ENABLED)) == ACE_PRESENT {
            rdmsrq(MSR_VIA_FCR, &mut msr);
            // enable ACE unit
            wrmsrq(MSR_VIA_FCR, msr | ACE_FCR);
            pr_info!("CPU: Enabled ACE h/w crypto\n");
        }

        // enable RNG unit, if present and disabled
        if (tmp & (RNG_PRESENT | RNG_ENABLED)) == RNG_PRESENT {
            rdmsrq(MSR_VIA_RNG, &mut msr);
            // enable RNG unit
            wrmsrq(MSR_VIA_RNG, msr | RNG_ENABLE);
            pr_info!("CPU: Enabled h/w RNG\n");
        }

        // store Centaur Extended Feature Flags as word 5 of the CPU capability bit array
        (*c).x86_capability[CPUID_C000_0001_EDX] = cpuid_edx(0xC0000001);
    }
    #[cfg(CONFIG_X86_32)]
    {
        // Cyrix III family needs CX8 & PGE explicitly enabled.
        if (*c).x86_model >= 6 && (*c).x86_model <= 13 {
            rdmsrq(MSR_VIA_FCR, &mut msr);
            wrmsrq(MSR_VIA_FCR, msr | (1 << 1 | 1 << 7));
            set_cpu_cap(c, X86_FEATURE_CX8);
        }

        // Before Nehemiah, the C3's had 3dNOW!
        if (*c).x86_model >= 6 && (*c).x86_model < 9 {
            set_cpu_cap(c, X86_FEATURE_3DNOW);
        }
    }
    if (*c).x86 == 0x6 && (*c).x86_model >= 0xf {
        (*c).x86_cache_alignment = (*c).x86_clflush_size * 2;
        set_cpu_cap(c, X86_FEATURE_REP_GOOD);
    }

    if (*c).x86 >= 7 {
        set_cpu_cap(c, X86_FEATURE_REP_GOOD);
    }
}

const ECX8: u32 = 1 << 1;
const EIERRINT: u32 = 1 << 2;
const DPM: u32 = 1 << 3;
const DMCE: u32 = 1 << 4;
const DSTPCLK: u32 = 1 << 5;
const ELINEAR: u32 = 1 << 6;
const DSMC: u32 = 1 << 7;
const DTLOCK: u32 = 1 << 8;
const EDCTLB: u32 = 1 << 8;
const EMMX: u32 = 1 << 9;
const DPDC: u32 = 1 << 11;
const EBRPRED: u32 = 1 << 12;
const DIC: u32 = 1 << 13;
const DDC: u32 = 1 << 14;
const DNA: u32 = 1 << 15;
const ERETSTK: u32 = 1 << 16;
const E2MMX: u32 = 1 << 19;
const EAMD3D: u32 = 1 << 20;

unsafe fn early_init_centaur(c: *mut cpuinfo_x86) {
    #[cfg(CONFIG_X86_32)]
    if (*c).x86 == 5 {
        set_cpu_cap(c, X86_FEATURE_CENTAUR_MCR);
    }
    if ((*c).x86 == 6 && (*c).x86_model >= 0xf) || (*c).x86 >= 7 {
        set_cpu_cap(c, X86_FEATURE_CONSTANT_TSC);
    }
    if (*c).x86_power & (1 << 8) != 0 {
        set_cpu_cap(c, X86_FEATURE_CONSTANT_TSC);
        set_cpu_cap(c, X86_FEATURE_NONSTOP_TSC);
    }
}

unsafe fn init_centaur(c: *mut cpuinfo_x86) {
    early_init_centaur(c);
    init_intel_cacheinfo(c);

    if (*c).cpuid_level > 9 {
        let eax = cpuid_eax(10);
        if (eax & 0xff) != 0 && ((eax >> 8) & 0xff) > 1 {
            set_cpu_cap(c, X86_FEATURE_ARCH_PERFMON);
        }
    }

    #[cfg(CONFIG_X86_32)]
    if (*c).x86 == 5 {
        let mut name: *const core::ffi::c_char;
        let mut fcr_set: u32 = 0;
        let mut fcr_clr: u32 = 0;
        let mut newlo: u32;
        let (mut aa, mut bb, mut cc, mut dd): (u32, u32, u32, u32);
        let mut val: msr;
        match (*c).x86_model {
            4 => {
                name = c"C6".as_ptr();
                fcr_set = ECX8 | DSMC | EDCTLB | EMMX | ERETSTK;
                fcr_clr = DPDC;
                pr_notice!("Disabling bugged TSC.\n");
                clear_cpu_cap(c, X86_FEATURE_TSC);
            }
            8 => {
                name = match (*c).x86_stepping {
                    7..=9 => c"2A".as_ptr(),
                    10..=15 => c"2B".as_ptr(),
                    _ => c"2".as_ptr(),
                };
                fcr_set = ECX8 | DSMC | DTLOCK | EMMX | EBRPRED | ERETSTK | E2MMX | EAMD3D;
                fcr_clr = DPDC;
            }
            9 => {
                name = c"3".as_ptr();
                fcr_set = ECX8 | DSMC | DTLOCK | EMMX | EBRPRED | ERETSTK | E2MMX | EAMD3D;
                fcr_clr = DPDC;
            }
            _ => name = c"??".as_ptr(),
        }
        rdmsrq(MSR_IDT_FCR1, &mut val.q);
        newlo = (val.l | fcr_set) & !fcr_clr;
        if newlo != val.l {
            pr_info!("Centaur FCR was 0x%X now 0x%X\n", val.l, newlo);
            val.l = newlo;
            wrmsrq(MSR_IDT_FCR1, val.q);
        } else {
            pr_info!("Centaur FCR is 0x%X\n", val.l);
        }
        set_cpu_cap(c, X86_FEATURE_CENTAUR_MCR);
        set_cpu_cap(c, X86_FEATURE_CX8);
        if (*c).x86_model >= 8 { set_cpu_cap(c, X86_FEATURE_3DNOW); }
        if cpuid_eax(0x80000000) >= 0x80000005 {
            cpuid(0x80000005, &mut aa, &mut bb, &mut cc, &mut dd);
            (*c).x86_cache_size = (cc >> 24) + (dd >> 24);
        }
        sprintf((*c).x86_model_id.as_mut_ptr(), c"WinChip %s".as_ptr(), name);
    }
    if (*c).x86 == 6 || (*c).x86 >= 7 { init_c3(c); }
    #[cfg(CONFIG_X86_64)]
    set_cpu_cap(c, X86_FEATURE_LFENCE_RDTSC);
    init_ia32_feat_ctl(c);
}

#[cfg(CONFIG_X86_32)]
unsafe fn centaur_size_cache(c: *mut cpuinfo_x86, mut size: u32) -> u32 {
    if (*c).x86 == 6 && ((*c).x86_model == 7 || (*c).x86_model == 8) { size >>= 8; }
    if (*c).x86 == 6 && (*c).x86_model == 9 && (*c).x86_stepping == 1 && size == 65 { size -= 1; }
    size
}

static centaur_cpu_dev: cpu_dev = cpu_dev {
    c_vendor: c"Centaur".as_ptr(),
    c_ident: [c"CentaurHauls".as_ptr()],
    c_early_init: Some(early_init_centaur),
    c_init: Some(init_centaur),
    #[cfg(CONFIG_X86_32)]
    legacy_cache_size: Some(centaur_size_cache),
    c_x86_vendor: X86_VENDOR_CENTAUR,
};

cpu_dev_register(centaur_cpu_dev);

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
