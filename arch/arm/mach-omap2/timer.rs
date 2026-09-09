/*
 * linux/arch/arm/mach-omap2/timer.c
 *
 * OMAP2 GP timer support.
 *
 * Copyright (C) 2009 Nokia Corporation
 *
 * Update to use new clocksource/clockevent layers
 * Author: Kevin Hilman, MontaVista Software, Inc. <source@mvista.com>
 * Copyright (C) 2007 MontaVista Software, Inc.
 *
 * Original driver:
 * Copyright (C) 2005 Nokia Corporation
 * Author: Paul Mundt <paul.mundt@nokia.com>
 *         Juha Yrjölä <juha.yrjola@nokia.com>
 * OMAP Dual-mode timer framework support by Timo Teras
 *
 * Some parts based off of TI's 24xx code:
 *
 * Copyright (C) 2004-2009 Texas Instruments, Inc.
 *
 * Roughly modelled after the OMAP1 MPU timer code.
 * Added OMAP4 support - Santosh Shilimkar <santosh.shilimkar@ti.com>
 *
 * This file is subject to the terms and conditions of the GNU General Public
 * License. See the file "COPYING" in the main directory of this archive
 * for more details.
 */

const REALTIME_COUNTER_BASE: usize = 0x48243200;
const INCREMENTER_NUMERATOR_OFFSET: usize = 0x10;
const INCREMENTER_DENUMERATOR_RELOAD_OFFSET: usize = 0x14;
const NUMERATOR_DENUMERATOR_MASK: u32 = 0xfffff000;

static mut ARCH_TIMER_FREQ: libc::c_ulong = 0;

extern "C" {
    fn omap_smc1(index: libc::c_ulong, value: libc::c_ulong);
    fn ioremap(addr: usize, size: usize) -> *mut core::ffi::c_void;
    fn iounmap(addr: *mut core::ffi::c_void);
    fn pr_err(fmt: *const libc::c_char, ...);
    fn clk_get(dev: *mut core::ffi::c_void, id: *const libc::c_char) -> *mut clk;
    fn clk_get_rate(clk: *mut clk) -> libc::c_ulong;
    fn clk_put(clk: *mut clk);
    fn IS_ERR(ptr: *mut clk) -> bool;
    fn soc_is_dra7xx() -> bool;
    fn omap_ctrl_readl(reg: libc::c_ulong) -> u32;
    fn readl_relaxed(addr: *mut u32) -> u32;
    fn writel_relaxed(value: u32, addr: *mut u32);
    fn omap_clk_init();
    fn timer_probe();
}

#[repr(C)]
struct clk {
    _private: [u8; 0],
}

extern "C" {
    static OMAP5_DRA7_MON_SET_CNTFRQ_INDEX: libc::c_ulong;
    static DRA7_CTRL_CORE_BOOTSTRAP: libc::c_ulong;
    static DRA7_SPEEDSELECT_MASK: u32;
}

pub unsafe fn set_cntfreq() {
    omap_smc1(OMAP5_DRA7_MON_SET_CNTFRQ_INDEX, ARCH_TIMER_FREQ);
}

/*
 * The realtime counter also called master counter, is a free-running
 * counter, which is related to real time. It produces the count used
 * by the CPU local timer peripherals in the MPU cluster. The timer counts
 * at a rate of 6.144 MHz. Because the device operates on different clocks
 * in different power modes, the master counter shifts operation between
 * clocks, adjusting the increment per clock in hardware accordingly to
 * maintain a constant count rate.
 */
unsafe fn realtime_counter_init() {
    let base = ioremap(REALTIME_COUNTER_BASE, 32);
    if base.is_null() {
        pr_err(b"%s: ioremap failed\0".as_ptr() as *const libc::c_char);
        return;
    }

    let sys_clk = clk_get(core::ptr::null_mut(), b"sys_clkin\0".as_ptr() as *const libc::c_char);
    if IS_ERR(sys_clk) {
        pr_err(b"%s: failed to get system clock handle\0".as_ptr() as *const libc::c_char);
        iounmap(base);
        return;
    }

    let rate = clk_get_rate(sys_clk);
    clk_put(sys_clk);

    let (num, den): (u64, u64);
    if soc_is_dra7xx() {
        let reg = omap_ctrl_readl(DRA7_CTRL_CORE_BOOTSTRAP);
        if reg & DRA7_SPEEDSELECT_MASK != 0 {
            num = 75;
            den = 244;
            goto_sysclk1_based(base, rate, num, den);
            return;
        }
    }

    /* Numerator/denumerator values refer TRM Realtime Counter section */
    (num, den) = match rate {
        12000000 => (64, 125),
        13000000 => (768, 1625),
        19200000 => (8, 25),
        20000000 => (192, 625),
        26000000 => (384, 1625),
        27000000 => (256, 1125),
        38400000 => (4, 25),
        _ => (4, 25),
    };

    goto_sysclk1_based(base, rate, num, den);
}

unsafe fn goto_sysclk1_based(base: *mut core::ffi::c_void, rate: libc::c_ulong, num: u64, den: u64) {
    /* Program numerator and denumerator registers */
    let numerator = (base as *mut u8).add(INCREMENTER_NUMERATOR_OFFSET) as *mut u32;
    let mut reg = readl_relaxed(numerator) & NUMERATOR_DENUMERATOR_MASK;
    reg |= num as u32;
    writel_relaxed(reg, numerator);

    let denominator = (base as *mut u8).add(INCREMENTER_DENUMERATOR_RELOAD_OFFSET) as *mut u32;
    reg = readl_relaxed(denominator) & NUMERATOR_DENUMERATOR_MASK;
    reg |= den as u32;
    writel_relaxed(reg, denominator);

    ARCH_TIMER_FREQ = (((rate as u128) * (num as u128) + den as u128 - 1) / den as u128) as libc::c_ulong;
    set_cntfreq();

    iounmap(base);
}

pub unsafe fn omap5_realtime_timer_init() {
    omap_clk_init();
    realtime_counter_init();

    timer_probe();
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
