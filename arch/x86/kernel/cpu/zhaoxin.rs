// SPDX-License-Identifier: GPL-2.0
// Dependencies supplied by the included Linux and architecture headers.

const MSR_ZHAOXIN_FCR57: u32 = 0x0000_1257;

const ACE_PRESENT: u32 = 1 << 6;
const ACE_ENABLED: u32 = 1 << 7;
const ACE_FCR: u32 = 1 << 7; // MSR_ZHAOXIN_FCR

const RNG_PRESENT: u32 = 1 << 2;
const RNG_ENABLED: u32 = 1 << 3;
const RNG_ENABLE: u32 = 1 << 8; // MSR_ZHAOXIN_RNG

unsafe fn init_zhaoxin_cap(c: *mut cpuinfo_x86) {
    let mut msr: u64;

    // Test for Extended Feature Flags presence
    if cpuid_eax(0xC000_0000) >= 0xC000_0001 {
        let tmp: u32 = cpuid_edx(0xC000_0001);

        // Enable ACE unit, if present and disabled
        if (tmp & (ACE_PRESENT | ACE_ENABLED)) == ACE_PRESENT {
            rdmsrq(MSR_ZHAOXIN_FCR57, &mut msr);
            // Enable ACE unit
            wrmsrq(MSR_ZHAOXIN_FCR57, msr | ACE_FCR);
            pr_info("CPU: Enabled ACE h/w crypto\0");
        }

        // Enable RNG unit, if present and disabled
        if (tmp & (RNG_PRESENT | RNG_ENABLED)) == RNG_PRESENT {
            rdmsrq(MSR_ZHAOXIN_FCR57, &mut msr);
            // Enable RNG unit
            wrmsrq(MSR_ZHAOXIN_FCR57, msr | RNG_ENABLE);
            pr_info("CPU: Enabled h/w RNG\0");
        }

        /*
         * Store Extended Feature Flags as word 5 of the CPU
         * capability bit array
         */
        (*c).x86_capability[CPUID_C000_0001_EDX] = cpuid_edx(0xC000_0001);
    }

    if (*c).x86 >= 0x6 {
        set_cpu_cap(c, X86_FEATURE_REP_GOOD);
    }
}

unsafe fn early_init_zhaoxin(c: *mut cpuinfo_x86) {
    if (*c).x86 >= 0x6 {
        set_cpu_cap(c, X86_FEATURE_CONSTANT_TSC);
    }

    if (*c).x86_power & (1 << 8) != 0 {
        set_cpu_cap(c, X86_FEATURE_CONSTANT_TSC);
        set_cpu_cap(c, X86_FEATURE_NONSTOP_TSC);
    }
}

unsafe fn init_zhaoxin(c: *mut cpuinfo_x86) {
    early_init_zhaoxin(c);
    init_intel_cacheinfo(c);

    if (*c).cpuid_level > 9 {
        let eax: u32 = cpuid_eax(10);

        /*
         * Check for version and the number of counters
         * Version(eax[7:0]) can't be 0;
         * Counters(eax[15:8]) should be greater than 1;
         */
        if (eax & 0xff) != 0 && ((eax >> 8) & 0xff) > 1 {
            set_cpu_cap(c, X86_FEATURE_ARCH_PERFMON);
        }
    }

    if (*c).x86 >= 0x6 {
        init_zhaoxin_cap(c);
    }

    // CONFIG_X86_64
    #[cfg(target_pointer_width = "64")]
    set_cpu_cap(c, X86_FEATURE_LFENCE_RDTSC);

    init_ia32_feat_ctl(c);
}

// CONFIG_X86_32
#[cfg(target_pointer_width = "32")]
unsafe fn zhaoxin_size_cache(_c: *mut cpuinfo_x86, size: u32) -> u32 {
    size
}

static ZHAOXIN_CPU_DEV: cpu_dev = cpu_dev {
    c_vendor: "zhaoxin",
    c_ident: ["  Shanghai  "],
    c_early_init: Some(early_init_zhaoxin),
    c_init: Some(init_zhaoxin),
    // CONFIG_X86_32
    #[cfg(target_pointer_width = "32")]
    legacy_cache_size: Some(zhaoxin_size_cache),
    c_x86_vendor: X86_VENDOR_ZHAOXIN,
};

cpu_dev_register(ZHAOXIN_CPU_DEV);

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
