// SPDX-License-Identifier: GPL-2.0
//
// Dependencies supplied by the surrounding kernel translation.

unsafe fn early_init_transmeta(c: *mut cpuinfo_x86) {
    let mut xlvl: u32;

    /* Transmeta-defined flags: level 0x80860001 */
    xlvl = cpuid_eax(0x80860000);
    if (xlvl & 0xffff0000) == 0x80860000 {
        if xlvl >= 0x80860001 {
            (*c).x86_capability[CPUID_8086_0001_EDX as usize] = cpuid_edx(0x80860001);
        }
    }
}

unsafe fn init_transmeta(c: *mut cpuinfo_x86) {
    let mut msr: u64;
    let mut max: u32;
    let mut dummy: u32;
    let mut cms_rev1: u32;
    let mut cms_rev2: u32;
    let mut cpu_rev: u32;
    let mut cpu_freq: u32 = 0;
    let mut cpu_flags: u32;
    let mut new_cpu_rev: u32;
    let mut cpu_info = [0i8; 65];

    early_init_transmeta(c);

    cpu_detect_cache_sizes(c);

    /* Print CMS and CPU revision */
    max = cpuid_eax(0x80860000);
    cpu_rev = 0;
    if max >= 0x80860001 {
        cpuid(0x80860001, &mut dummy, &mut cpu_rev, &mut cpu_freq, &mut cpu_flags);
        if cpu_rev != 0x02000000 {
            pr_info!(
                "CPU: Processor revision {}.{}.{}.{}, {} MHz\n",
                (cpu_rev >> 24) & 0xff,
                (cpu_rev >> 16) & 0xff,
                (cpu_rev >> 8) & 0xff,
                cpu_rev & 0xff,
                cpu_freq
            );
        }
    }
    if max >= 0x80860002 {
        cpuid(0x80860002, &mut new_cpu_rev, &mut cms_rev1, &mut cms_rev2, &mut dummy);
        if cpu_rev == 0x02000000 {
            pr_info!("CPU: Processor revision {:08X}, {} MHz\n", new_cpu_rev, cpu_freq);
        }
        pr_info!(
            "CPU: Code Morphing Software revision {}.{}.{}-{}-{}\n",
            (cms_rev1 >> 24) & 0xff,
            (cms_rev1 >> 16) & 0xff,
            (cms_rev1 >> 8) & 0xff,
            cms_rev1 & 0xff,
            cms_rev2
        );
    }
    if max >= 0x80860006 {
        cpuid(
            0x80860003,
            (&mut cpu_info[0] as *mut i8).cast::<u32>(),
            (&mut cpu_info[4] as *mut i8).cast::<u32>(),
            (&mut cpu_info[8] as *mut i8).cast::<u32>(),
            (&mut cpu_info[12] as *mut i8).cast::<u32>(),
        );
        cpuid(
            0x80860004,
            (&mut cpu_info[16] as *mut i8).cast::<u32>(),
            (&mut cpu_info[20] as *mut i8).cast::<u32>(),
            (&mut cpu_info[24] as *mut i8).cast::<u32>(),
            (&mut cpu_info[28] as *mut i8).cast::<u32>(),
        );
        cpuid(
            0x80860005,
            (&mut cpu_info[32] as *mut i8).cast::<u32>(),
            (&mut cpu_info[36] as *mut i8).cast::<u32>(),
            (&mut cpu_info[40] as *mut i8).cast::<u32>(),
            (&mut cpu_info[44] as *mut i8).cast::<u32>(),
        );
        cpuid(
            0x80860006,
            (&mut cpu_info[48] as *mut i8).cast::<u32>(),
            (&mut cpu_info[52] as *mut i8).cast::<u32>(),
            (&mut cpu_info[56] as *mut i8).cast::<u32>(),
            (&mut cpu_info[60] as *mut i8).cast::<u32>(),
        );
        cpu_info[64] = 0;
        pr_info!("CPU: %s\n", cpu_info.as_ptr());
    }

    /* Unhide possibly hidden capability flags */
    rdmsrq(0x80860004, &mut msr);
    wrmsrq(0x80860004, msr | !0u32 as u64);
    cpuid_refresh_leaf(c, 0x1);
    (*c).x86_capability[CPUID_1_EDX as usize] = cpuid_edx(0x00000001);
    wrmsrq(0x80860004, msr);

    /* All Transmeta CPUs have a constant TSC */
    set_cpu_cap(c, X86_FEATURE_CONSTANT_TSC);

    // CONFIG_SYSCTL: randomize_va_space slows us down enormously;
    // it probably triggers retranslation of x86->native bytecode.
}

static transmeta_cpu_dev: cpu_dev = cpu_dev {
    c_vendor: "Transmeta",
    c_ident: ["GenuineTMx86", "TransmetaCPU"],
    c_early_init: Some(early_init_transmeta),
    c_init: Some(init_transmeta),
    c_x86_vendor: X86_VENDOR_TRANSMETA,
};

cpu_dev_register(transmeta_cpu_dev);

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
