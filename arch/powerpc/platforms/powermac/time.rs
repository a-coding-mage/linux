// SPDX-License-Identifier: GPL-2.0
/*
 * Support for periodic interrupts (100 per second) and for getting
 * the current time from the RTC on Power Macintoshes.
 *
 * We use the decrementer register for our periodic interrupts.
 *
 * Paul Mackerras August 1996.
 * Copyright (C) 1996 Paul Mackerras.
 * Copyright (C) 2003-2005 Benjamin Herrenschmidt.
 */

// C headers provide the declarations used below.

// #undef DEBUG
#[cfg(feature = "DEBUG")]
macro_rules! DBG { ($($x:tt)*) => { printk!($($x)*); } }
#[cfg(not(feature = "DEBUG"))]
macro_rules! DBG { ($($x:tt)*) => {}; }

/* Calibrate the decrementer frequency with the VIA timer 1. */
const VIA_TIMER_FREQ_6: i32 = 4700000;

/* VIA registers */
const RS: usize = 0x200;
const T1CL: usize = 4 * RS;
const T1CH: usize = 5 * RS;
const T1LL: usize = 6 * RS;
const T1LH: usize = 7 * RS;
const ACR: usize = 11 * RS;
const IFR: usize = 13 * RS;

/* Bits in ACR */
const T1MODE: u8 = 0xc0;
const T1MODE_CONT: u8 = 0x40;

/* Bits in IFR and IER */
const T1_INT: u8 = 0x40;

pub unsafe fn pmac_time_init() -> i32 {
    let mut delta: i32 = 0;
    // Preserved build-time condition: CONFIG_NVRAM && CONFIG_PPC32.
    #[cfg(all(feature = "CONFIG_NVRAM", feature = "CONFIG_PPC32"))]
    {
        let dst: i32;
        delta = (pmac_xpram_read(PMAC_XPRAM_MACHINE_LOC + 0x9) as i32) << 16;
        delta |= (pmac_xpram_read(PMAC_XPRAM_MACHINE_LOC + 0xa) as i32) << 8;
        delta |= pmac_xpram_read(PMAC_XPRAM_MACHINE_LOC + 0xb) as i32;
        if (delta & 0x00800000u32 as i32) != 0 {
            delta |= 0xFF000000u32 as i32;
        }
        dst = if (pmac_xpram_read(PMAC_XPRAM_MACHINE_LOC + 0x8) & 0x80) != 0 { 1 } else { 0 };
        printk!("GMT Delta read from XPRAM: {} minutes, DST: {}\n", delta / 60, str_on_off(dst));
    }
    delta
}

#[cfg(feature = "CONFIG_PMAC_SMU")]
unsafe fn smu_get_time() -> time64_t {
    let mut tm: rtc_time = core::mem::zeroed();
    if smu_get_rtc_time(&mut tm, 1) != 0 { return 0; }
    rtc_tm_to_time64(&tm)
}

/* Can't be __init, it's called when suspending and resuming */
pub unsafe fn pmac_get_boot_time() -> time64_t {
    /* Get the time from the RTC, used only at boot time */
    match sys_ctrler {
        #[cfg(feature = "CONFIG_ADB_CUDA")]
        SYS_CTRLER_CUDA => cuda_get_time(),
        #[cfg(feature = "CONFIG_ADB_PMU")]
        SYS_CTRLER_PMU => pmu_get_time(),
        #[cfg(feature = "CONFIG_PMAC_SMU")]
        SYS_CTRLER_SMU => smu_get_time(),
        _ => 0,
    }
}

pub unsafe fn pmac_get_rtc_time(tm: *mut rtc_time) {
    match sys_ctrler {
        #[cfg(feature = "CONFIG_ADB_CUDA")]
        SYS_CTRLER_CUDA => rtc_time64_to_tm(cuda_get_time(), tm),
        #[cfg(feature = "CONFIG_ADB_PMU")]
        SYS_CTRLER_PMU => rtc_time64_to_tm(pmu_get_time(), tm),
        #[cfg(feature = "CONFIG_PMAC_SMU")]
        SYS_CTRLER_SMU => { smu_get_rtc_time(tm, 1); },
        _ => {}
    }
}

pub unsafe fn pmac_set_rtc_time(tm: *mut rtc_time) -> i32 {
    match sys_ctrler {
        #[cfg(feature = "CONFIG_ADB_CUDA")]
        SYS_CTRLER_CUDA => cuda_set_rtc_time(tm),
        #[cfg(feature = "CONFIG_ADB_PMU")]
        SYS_CTRLER_PMU => pmu_set_rtc_time(tm),
        #[cfg(feature = "CONFIG_PMAC_SMU")]
        SYS_CTRLER_SMU => smu_set_rtc_time(tm, 1),
        _ => -ENODEV,
    }
}

// Preserved build-time condition: CONFIG_PPC32.
#[cfg(feature = "CONFIG_PPC32")]
unsafe fn via_calibrate_decr() -> i32 {
    let mut vias: *mut device_node;
    let mut via: *mut u8;
    let count: i32 = VIA_TIMER_FREQ_6 / 100;
    let mut dstart: u32;
    let mut dend: u32;
    let mut rsrc: resource = core::mem::zeroed();

    vias = of_find_node_by_name(core::ptr::null_mut(), "via-cuda\0".as_ptr() as *const _);
    if vias.is_null() { vias = of_find_node_by_name(core::ptr::null_mut(), "via-pmu\0".as_ptr() as *const _); }
    if vias.is_null() { vias = of_find_node_by_name(core::ptr::null_mut(), "via\0".as_ptr() as *const _); }
    if vias.is_null() || of_address_to_resource(vias, 0, &mut rsrc) != 0 {
        of_node_put(vias);
        return 0;
    }
    of_node_put(vias);
    via = early_ioremap(rsrc.start, resource_size(&rsrc));
    if via.is_null() { printk!("Failed to map VIA for timer calibration !\n"); return 0; }

    out_8(via.add(ACR), (in_8(via.add(ACR)) & !T1MODE) | T1MODE_CONT);
    out_8(via.add(T1CH), 2);
    out_8(via.add(T1LL), count as u8);
    out_8(via.add(T1LH), (count >> 8) as u8);
    while (in_8(via.add(IFR)) & T1_INT == 0) {}
    dstart = get_dec();
    in_8(via.add(T1CL));
    while (in_8(via.add(IFR)) & T1_INT == 0) {}
    dend = get_dec();
    ppc_tb_freq = (dstart.wrapping_sub(dend) * 100) / 6;
    early_iounmap(via as *mut core::ffi::c_void, resource_size(&rsrc));
    1
}

pub unsafe fn pmac_calibrate_decr() {
    generic_calibrate_decr();
    #[cfg(feature = "CONFIG_PPC32")]
    {
        if !of_machine_is_compatible("MacRISC2\0".as_ptr() as *const _) &&
           !of_machine_is_compatible("MacRISC3\0".as_ptr() as *const _) &&
           !of_machine_is_compatible("MacRISC4\0".as_ptr() as *const _) {
            if via_calibrate_decr() != 0 { return; }
        }
        if of_machine_is_compatible("PowerMac3,5\0".as_ptr() as *const _) && via_calibrate_decr() != 0 { return; }
    }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
