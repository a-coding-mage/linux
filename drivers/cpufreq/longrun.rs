// SPDX-License-Identifier: GPL-2.0-only
/*
 * (C) 2002 - 2003  Dominik Brodowski <linux@brodo.de>
 *
 *  BIG FAT DISCLAIMER: Work in progress code. Possibly *dangerous*
 */

// Kernel and architecture dependencies supplied by other translation units.

static mut longrun_driver: cpufreq_driver = cpufreq_driver::default();

/**
 * longrun_{low,high}_freq is needed for the conversion of cpufreq kHz
 * values into per cent values. In TMTA microcode, the following is valid:
 * performance_pctg = (current_freq - low_freq)/(high_freq - low_freq)
 */
static mut longrun_low_freq: u32 = 0;
static mut longrun_high_freq: u32 = 0;

/** longrun_get_policy - get the current LongRun policy */
unsafe fn longrun_get_policy(policy: *mut cpufreq_policy) {
    let mut msr: msr = core::mem::zeroed();

    rdmsrq(MSR_TMTA_LONGRUN_FLAGS, &mut msr.q);
    pr_debug!("longrun flags are {:x} - {:x}\n", msr.l, msr.h);
    if msr.l & 0x01 != 0 {
        (*policy).policy = CPUFREQ_POLICY_PERFORMANCE;
    } else {
        (*policy).policy = CPUFREQ_POLICY_POWERSAVE;
    }

    rdmsrq(MSR_TMTA_LONGRUN_CTRL, &mut msr.q);
    pr_debug!("longrun ctrl is {:x} - {:x}\n", msr.l, msr.h);
    msr.l &= 0x0000007F;
    msr.h &= 0x0000007F;

    if longrun_high_freq <= longrun_low_freq {
        (*policy).min = longrun_high_freq;
        (*policy).max = longrun_high_freq;
    } else {
        (*policy).min = longrun_low_freq + msr.l * ((longrun_high_freq - longrun_low_freq) / 100);
        (*policy).max = longrun_low_freq + msr.h * ((longrun_high_freq - longrun_low_freq) / 100);
    }
    (*policy).cpu = 0;
}

/** longrun_set_policy - sets a new CPUFreq policy */
unsafe fn longrun_set_policy(policy: *mut cpufreq_policy) -> i32 {
    let mut msr: msr = core::mem::zeroed();
    let (mut pctg_lo, mut pctg_hi): (u32, u32);

    if policy.is_null() { return -EINVAL; }
    if longrun_high_freq <= longrun_low_freq {
        pctg_lo = 100; pctg_hi = 100;
    } else {
        pctg_lo = ((*policy).min - longrun_low_freq) / ((longrun_high_freq - longrun_low_freq) / 100);
        pctg_hi = ((*policy).max - longrun_low_freq) / ((longrun_high_freq - longrun_low_freq) / 100);
    }
    if pctg_hi > 100 { pctg_hi = 100; }
    if pctg_lo > pctg_hi { pctg_lo = pctg_hi; }

    rdmsrq(MSR_TMTA_LONGRUN_FLAGS, &mut msr.q);
    msr.l &= 0xFFFFFFFE;
    match (*policy).policy {
        CPUFREQ_POLICY_PERFORMANCE => msr.l |= 0x00000001,
        CPUFREQ_POLICY_POWERSAVE => {},
        _ => {},
    }
    wrmsrq(MSR_TMTA_LONGRUN_FLAGS, msr.q);

    rdmsrq(MSR_TMTA_LONGRUN_CTRL, &mut msr.q);
    msr.l &= 0xFFFFFF80;
    msr.h &= 0xFFFFFF80;
    msr.l |= pctg_lo;
    msr.h |= pctg_hi;
    wrmsrq(MSR_TMTA_LONGRUN_CTRL, msr.q);
    0
}

/** longrun_verify_policy - verifies a new CPUFreq policy */
unsafe fn longrun_verify_policy(policy: *mut cpufreq_policy_data) -> i32 {
    if policy.is_null() { return -EINVAL; }
    (*policy).cpu = 0;
    cpufreq_verify_within_cpu_limits(policy);
    0
}

unsafe fn longrun_get(cpu: u32) -> u32 {
    let (mut eax, mut ebx, mut ecx, mut edx) = (0u32, 0u32, 0u32, 0u32);
    if cpu != 0 { return 0; }
    cpuid(0x80860007, &mut eax, &mut ebx, &mut ecx, &mut edx);
    pr_debug!("cpuid eax is {}\n", eax);
    eax * 1000
}

/** longrun_determine_freqs - determines the lowest and highest possible core frequency */
unsafe fn longrun_determine_freqs(low_freq: *mut u32, high_freq: *mut u32) -> i32 {
    let (mut msr, mut save): (msr, msr) = (core::mem::zeroed(), core::mem::zeroed());
    let (mut eax, mut ebx, mut ecx, mut edx) = (0u32, 0u32, 0u32, 0u32);
    let mut try_hi: u32;
    let c = &mut cpu_data(0);
    if low_freq.is_null() || high_freq.is_null() { return -EINVAL; }

    if cpu_has(c, X86_FEATURE_LRTI) {
        rdmsrq(MSR_TMTA_LRTI_READOUT, &mut msr.q);
        msr.l = msr.h;
        wrmsrq(MSR_TMTA_LRTI_READOUT, msr.q);
        rdmsrq(MSR_TMTA_LRTI_VOLT_MHZ, &mut msr.q);
        *low_freq = msr.l * 1000;
        msr.l = 0;
        wrmsrq(MSR_TMTA_LRTI_READOUT, msr.q);
        rdmsrq(MSR_TMTA_LRTI_VOLT_MHZ, &mut msr.q);
        *high_freq = msr.l * 1000;
        pr_debug!("longrun table interface told {} - {} kHz\n", *low_freq, *high_freq);
        if *low_freq > *high_freq { *low_freq = *high_freq; }
        return 0;
    }

    *high_freq = cpu_khz / 1000;
    *high_freq = *high_freq * 1000;
    pr_debug!("high frequency is {} kHz\n", *high_freq);
    rdmsrq(MSR_TMTA_LONGRUN_CTRL, &mut msr.q);
    save.l = msr.l & 0x0000007F;
    save.h = msr.h & 0x0000007F;
    cpuid(0x80860007, &mut eax, &mut ebx, &mut ecx, &mut edx);
    for try_hi in (1..=80).rev().step_by(10) {
        if ecx <= 90 { break; }
        msr.l &= 0xFFFFFF80;
        msr.h &= 0xFFFFFF80;
        msr.h |= try_hi;
        wrmsrq(MSR_TMTA_LONGRUN_CTRL, msr.q);
        cpuid(0x80860007, &mut eax, &mut ebx, &mut ecx, &mut edx);
        wrmsrq(MSR_TMTA_LONGRUN_CTRL, save.q);
    }
    pr_debug!("percentage is {} %%, freq is {} MHz\n", ecx, eax);
    ebx = ((cpu_khz / 1000) * ecx) / 100;
    if ecx > 95 || ecx == 0 || eax < ebx { return -EIO; }
    edx = ((eax - ebx) * 100) / (100 - ecx);
    *low_freq = edx * 1000;
    pr_debug!("low frequency is {} kHz\n", *low_freq);
    if *low_freq > *high_freq { *low_freq = *high_freq; }
    0
}

unsafe fn longrun_cpu_init(policy: *mut cpufreq_policy) -> i32 {
    if (*policy).cpu != 0 { return -ENODEV; }
    let result = longrun_determine_freqs(&mut longrun_low_freq, &mut longrun_high_freq);
    if result != 0 { return result; }
    (*policy).cpuinfo.min_freq = longrun_low_freq;
    (*policy).cpuinfo.max_freq = longrun_high_freq;
    longrun_get_policy(policy);
    0
}

static mut longrun_ids: [x86_cpu_id; 2] = [
    X86_MATCH_VENDOR_FEATURE(TRANSMETA, X86_FEATURE_LONGRUN, core::ptr::null()),
    x86_cpu_id::default(),
];

unsafe fn longrun_init() -> i32 {
    if !x86_match_cpu(longrun_ids.as_ptr()) { return -ENODEV; }
    cpufreq_register_driver(&mut longrun_driver)
}

unsafe fn longrun_exit() {
    cpufreq_unregister_driver(&mut longrun_driver);
}


// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
