// SPDX-License-Identifier: GPL-2.0-only
/*
 * (C) 2002 - 2003 Dominik Brodowski <linux@brodo.de>
 *
 *  Library for common functions for Intel SpeedStep v.1 and v.2 support
 *
 *  BIG FAT DISCLAIMER: Work in progress code. Possibly *dangerous*
 */

// Kernel headers and speedstep-lib.h provide the external types, constants,
// functions, and globals referenced below.

#[cfg(feature = "config_x86_speedstep_relaxed_cap_check")]
static mut relaxed_check: i32 = 0;
#[cfg(not(feature = "config_x86_speedstep_relaxed_cap_check"))]
const relaxed_check: i32 = 0;

unsafe fn pentium3_get_frequency(processor: speedstep_processor) -> u32 {
    /* See table 14 of p3_ds.pdf and table 22 of 29834003.pdf */
    const msr_decode_mult: [(u32, u8); 15] = [
        (30, 0x01), (35, 0x05), (40, 0x02), (45, 0x06), (50, 0x00),
        (55, 0x04), (60, 0x0b), (65, 0x0f), (70, 0x09), (75, 0x0d),
        (80, 0x0a), (85, 0x26), (90, 0x20), (100, 0x2b), (0, 0xff),
    ];
    /* PIII(-M) FSB settings: see table b1-b of 24547206.pdf */
    const msr_decode_fsb: [(u32, u8); 4] = [(66, 0x0), (100, 0x2), (133, 0x1), (0, 0xff)];

    let mut msr = msr { q: 0 };
    let mut msr_lo: u32;
    let mut msr_tmp: u32;
    let mut i = 0usize;
    let mut j = 0usize;

    rdmsrq(MSR_IA32_EBL_CR_POWERON, &mut msr.q);
    pr_debug!("P3 - MSR_IA32_EBL_CR_POWERON: 0x%x 0x%x\n", msr.l, msr.h);
    msr_tmp = msr_lo = msr.l;
    msr_tmp &= 0x00c0000;
    msr_tmp >>= 18;
    while msr_tmp != msr_decode_fsb[i].1 {
        if msr_decode_fsb[i].1 == 0xff { return 0; }
        i += 1;
    }
    if processor == SPEEDSTEP_CPU_PIII_C_EARLY {
        pr_debug!("workaround for early PIIIs\n");
        msr_lo &= 0x03c00000;
    } else { msr_lo &= 0x0bc00000; }
    msr_lo >>= 22;
    while msr_lo != msr_decode_mult[j].1 {
        if msr_decode_mult[j].1 == 0xff { return 0; }
        j += 1;
    }
    pr_debug!("speed is %u\n", msr_decode_mult[j].0 * msr_decode_fsb[i].0 * 100);
    msr_decode_mult[j].0 * msr_decode_fsb[i].0 * 100
}

unsafe fn pentiumM_get_frequency() -> u32 {
    let mut msr = msr { q: 0 };
    rdmsrq(MSR_IA32_EBL_CR_POWERON, &mut msr.q);
    pr_debug!("PM - MSR_IA32_EBL_CR_POWERON: 0x%x 0x%x\n", msr.l, msr.h);
    if msr.l & 0x00040000 != 0 {
        printk!(KERN_DEBUG, "speedstep-lib: PM - invalid FSB: 0x%x 0x%x\n", msr.l, msr.h);
        return 0;
    }
    let msr_tmp = (msr.l >> 22) & 0x1f;
    pr_debug!("bits 22-26 are 0x%x, speed is %u\n", msr_tmp, msr_tmp * 100 * 1000);
    msr_tmp * 100 * 1000
}

unsafe fn pentium_core_get_frequency() -> u32 {
    let mut msr = msr { q: 0 };
    let fsb: u32 = match { rdmsrq(MSR_FSB_FREQ, &mut msr.q); msr.l & 0x07 } {
        5 => 100000, 1 => 133333, 3 => 166667, 2 => 200000, 0 => 266667, 4 => 333333,
        _ => { pr_err!("PCORE - MSR_FSB_FREQ undefined value\n"); 0 }
    };
    rdmsrq(MSR_IA32_EBL_CR_POWERON, &mut msr.q);
    pr_debug!("PCORE - MSR_IA32_EBL_CR_POWERON: 0x%x 0x%x\n", msr.l, msr.h);
    let msr_tmp = (msr.l >> 22) & 0x1f;
    pr_debug!("bits 22-26 are 0x%x, speed is %u\n", msr_tmp, msr_tmp * fsb);
    msr_tmp * fsb
}

unsafe fn pentium4_get_frequency() -> u32 {
    let c = &boot_cpu_data;
    if c.x86_model < 2 { return cpu_khz; }
    let mut msr = msr { q: 0 };
    rdmsrq(0x2c, &mut msr.q);
    pr_debug!("P4 - MSR_EBC_FREQUENCY_ID: 0x%x 0x%x\n", msr.l, msr.h);
    let fsb = match ((msr.l >> 16) & 0x7) as u8 {
        0 => 100 * 1000, 1 => 13333 * 10, 2 => 200 * 1000, _ => 0,
    };
    if fsb == 0 { printk!(KERN_DEBUG, "speedstep-lib: couldn't detect FSB speed. Please send an e-mail to <linux@brodo.de>\n"); }
    let mult = msr.l >> 24;
    pr_debug!("P4 - FSB %u kHz; Multiplier %u; Speed %u kHz\n", fsb, mult, fsb * mult);
    fsb * mult
}

pub unsafe fn speedstep_get_frequency(processor: speedstep_processor) -> u32 {
    match processor {
        SPEEDSTEP_CPU_PCORE => pentium_core_get_frequency(),
        SPEEDSTEP_CPU_PM => pentiumM_get_frequency(),
        SPEEDSTEP_CPU_P4D | SPEEDSTEP_CPU_P4M => pentium4_get_frequency(),
        SPEEDSTEP_CPU_PIII_T | SPEEDSTEP_CPU_PIII_C | SPEEDSTEP_CPU_PIII_C_EARLY => pentium3_get_frequency(processor),
        _ => 0,
    }
}

pub unsafe fn speedstep_detect_processor() -> speedstep_processor {
    let c = &cpu_data(0);
    let mut msr = msr { q: 0 };
    pr_debug!("x86: %x, model: %x\n", c.x86, c.x86_model);
    if c.x86_vendor != X86_VENDOR_INTEL || (c.x86 != 6 && c.x86 != 0xF) { return 0; }
    if c.x86 == 0xF {
        if c.x86_model != 2 { return 0; }
        let ebx = cpuid_ebx(0x00000001) & 0xff;
        pr_debug!("ebx value is %x, x86_stepping is %x\n", ebx, c.x86_stepping);
        match c.x86_stepping {
            4 if ebx == 0x0e || ebx == 0x0f => return SPEEDSTEP_CPU_P4M,
            7 if ebx == 0x0e => return SPEEDSTEP_CPU_P4M,
            9 if ebx == 0x0e || strstr(c.x86_model_id, "Mobile Intel(R) Pentium(R) 4") => return SPEEDSTEP_CPU_P4M,
            _ => {}
        }
        return 0;
    }
    match c.x86_model {
        0x0B => { let ebx = cpuid_ebx(1) & 0xff; if ebx == 0x06 { SPEEDSTEP_CPU_PIII_T } else { 0 } }
        0x08 => {
            rdmsrq(MSR_IA32_EBL_CR_POWERON, &mut msr.q);
            msr.l &= 0x00c0000;
            if msr.l != 0x0080000 { return 0; }
            rdmsrq(MSR_IA32_PLATFORM_ID, &mut msr.q);
            if (msr.h & (1 << 18)) != 0 && (relaxed_check != 0 || (msr.h & (3 << 24)) != 0) {
                if c.x86_stepping == 0x01 { SPEEDSTEP_CPU_PIII_C_EARLY } else { SPEEDSTEP_CPU_PIII_C }
            } else { 0 }
        }
        _ => 0,
    }
}

pub unsafe fn speedstep_get_freqs(processor: speedstep_processor, low_speed: *mut u32, high_speed: *mut u32, transition_latency: *mut u32, set_state: Option<unsafe extern "C" fn(u32)>) -> i32 {
    if processor == 0 || low_speed.is_null() || high_speed.is_null() || set_state.is_none() { return -22; }
    let prev_speed = speedstep_get_frequency(processor);
    if prev_speed == 0 { return -5; }
    preempt_disable();
    let mut flags = 0ul;
    local_irq_save(&mut flags);
    set_state.unwrap()(SPEEDSTEP_LOW);
    *low_speed = speedstep_get_frequency(processor);
    if *low_speed == 0 { local_irq_restore(flags); preempt_enable(); return -5; }
    let tv1 = if !transition_latency.is_null() { ktime_get() } else { 0 };
    set_state.unwrap()(SPEEDSTEP_HIGH);
    let tv2 = if !transition_latency.is_null() { ktime_get() } else { 0 };
    *high_speed = speedstep_get_frequency(processor);
    let mut ret = 0;
    if *high_speed == 0 { ret = -5; }
    else if *low_speed == *high_speed { ret = -19; }
    else {
        if *high_speed != prev_speed { set_state.unwrap()(SPEEDSTEP_LOW); }
        if !transition_latency.is_null() {
            *transition_latency = ktime_to_us(ktime_sub(tv2, tv1));
            *transition_latency *= 1200;
            if *transition_latency > 10000000 || *transition_latency < 50000 { *transition_latency = 500000; }
        }
    }
    local_irq_restore(flags);
    preempt_enable();
    ret
}


// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
