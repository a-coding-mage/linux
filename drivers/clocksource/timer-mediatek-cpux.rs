// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * MediaTek SoCs CPUX General Purpose Timer handling
 *
 * Based on timer-mediatek.c:
 * Copyright (C) 2014 Matthias Brugger <matthias.bgg@gmail.com>
 *
 * Copyright (C) 2022 Collabora Ltd.
 *                    AngeloGioacchino Del Regno <angelogioacchino.delregno@collabora.com>
 */

// pr_fmt(fmt) KBUILD_MODNAME ": " fmt

// Linux kernel dependencies supplied by other translation units.
use core::ffi::{c_char, c_int, c_ulong};

const TIMER_SYNC_TICKS: u32 = 3;

/* cpux mcusys wrapper */
const CPUX_CON_REG: u32 = 0x0;
const CPUX_IDX_REG: u32 = 0x4;

/* cpux */
const CPUX_IDX_GLOBAL_CTRL: u32 = 0x0;
const CPUX_ENABLE: u32 = 1 << 0;
const CPUX_CLK_DIV_MASK: u32 = 0x7 << 8;
const CPUX_CLK_DIV1: u32 = 1 << 8;
const CPUX_CLK_DIV2: u32 = 1 << 9;
const CPUX_CLK_DIV4: u32 = 1 << 10;
const CPUX_IDX_GLOBAL_IRQ: u32 = 0x30;

#[repr(C)]
pub struct device_node;

#[repr(C)]
pub struct clock_event_device {
    pub name: *const c_char,
    pub cpumask: *const cpumask,
    pub rating: c_int,
    pub set_state_shutdown: Option<unsafe extern "C" fn(*mut clock_event_device) -> c_int>,
    pub tick_resume: Option<unsafe extern "C" fn(*mut clock_event_device) -> c_int>,
}

#[repr(C)]
pub struct timer_of {
    pub flags: u32,
    pub clkevt: clock_event_device,
}

#[repr(C)]
pub struct cpumask {
    _private: [u8; 0],
}

extern "C" {
    static cpu_possible_mask: *const cpumask;
    fn timer_of_base(to: *mut timer_of) -> *mut u8;
    fn readl(addr: *mut u8) -> u32;
    fn writel(value: u32, addr: *mut u8);
    fn to_timer_of(clkevt: *mut clock_event_device) -> *mut timer_of;
    fn cpumask_bits(mask: *const cpumask) -> *const c_ulong;
    fn timer_of_init(node: *mut device_node, to: *mut timer_of) -> c_int;
    fn timer_of_rate(to: *mut timer_of) -> u32;
    fn clockevents_config_and_register(
        clkevt: *mut clock_event_device,
        freq: u32,
        min_delta: u32,
        max_delta: u32,
    );
    fn warn(condition: c_int, format: *const c_char, ...);
}

const TIMER_OF_BASE: u32 = 1 << 0;
const TIMER_OF_CLOCK: u32 = 1 << 1;

unsafe fn mtk_cpux_readl(reg_idx: u32, to: *mut timer_of) -> u32 {
    let base = timer_of_base(to);
    writel(reg_idx, base.add(CPUX_IDX_REG as usize));
    readl(base.add(CPUX_CON_REG as usize))
}

unsafe fn mtk_cpux_writel(val: u32, reg_idx: u32, to: *mut timer_of) {
    let base = timer_of_base(to);
    writel(reg_idx, base.add(CPUX_IDX_REG as usize));
    writel(val, base.add(CPUX_CON_REG as usize));
}

unsafe fn mtk_cpux_set_irq(to: *mut timer_of, enable: bool) {
    let irq_mask = cpumask_bits(cpu_possible_mask);
    let mut val = mtk_cpux_readl(CPUX_IDX_GLOBAL_IRQ, to);

    if enable {
        val |= *irq_mask as u32;
    } else {
        val &= !(*irq_mask as u32);
    }

    mtk_cpux_writel(val, CPUX_IDX_GLOBAL_IRQ, to);
}

unsafe extern "C" fn mtk_cpux_clkevt_shutdown(clkevt: *mut clock_event_device) -> c_int {
    /* Clear any irq */
    mtk_cpux_set_irq(to_timer_of(clkevt), false);

    /*
     * Disabling CPUXGPT timer will crash the platform, especially
     * if Trusted Firmware is using it (usually, for sleep states),
     * so we only mask the IRQ and call it a day.
     */
    0
}

unsafe extern "C" fn mtk_cpux_clkevt_resume(clkevt: *mut clock_event_device) -> c_int {
    mtk_cpux_set_irq(to_timer_of(clkevt), true);
    0
}

static mut TO: timer_of = timer_of {
    /*
     * There are per-cpu interrupts for the CPUX General Purpose Timer
     * but since this timer feeds the AArch64 System Timer we can rely
     * on the CPU timer PPIs as well, so we don't declare TIMER_OF_IRQ.
     */
    flags: TIMER_OF_BASE | TIMER_OF_CLOCK,
    clkevt: clock_event_device {
        name: b"mtk-cpuxgpt\0".as_ptr() as *const c_char,
        cpumask: core::ptr::null(),
        rating: 10,
        set_state_shutdown: Some(mtk_cpux_clkevt_shutdown),
        tick_resume: Some(mtk_cpux_clkevt_resume),
    },
};

unsafe extern "C" fn mtk_cpux_init(node: *mut device_node) -> c_int {
    let mut freq: u32;
    let mut val: u32;
    let ret: c_int;

    /* If this fails, bad things are about to happen... */
    ret = timer_of_init(node, &raw mut TO);
    if ret != 0 {
        warn(1, b"Cannot start CPUX timers.\n\0".as_ptr() as *const c_char);
        return ret;
    }

    /*
     * Check if we're given a clock with the right frequency for this
     * timer, otherwise warn but keep going with the setup anyway, as
     * that makes it possible to still boot the kernel, even though
     * it may not work correctly (random lockups, etc).
     * The reason behind this is that having an early UART may not be
     * possible for everyone and this gives a chance to retrieve kmsg
     * for eventual debugging even on consumer devices.
     */
    freq = timer_of_rate(&raw mut TO);
    if freq > 13000000 {
        warn(1, b"Requested unsupported timer frequency %u\n\0".as_ptr() as *const c_char, freq);
    }

    /* Clock input is 26MHz, set DIV2 to achieve 13MHz clock */
    val = mtk_cpux_readl(CPUX_IDX_GLOBAL_CTRL, &raw mut TO);
    val &= !CPUX_CLK_DIV_MASK;
    val |= CPUX_CLK_DIV2;
    mtk_cpux_writel(val, CPUX_IDX_GLOBAL_CTRL, &raw mut TO);

    /* Enable all CPUXGPT timers */
    val = mtk_cpux_readl(CPUX_IDX_GLOBAL_CTRL, &raw mut TO);
    mtk_cpux_writel(val | CPUX_ENABLE, CPUX_IDX_GLOBAL_CTRL, &raw mut TO);

    clockevents_config_and_register(
        &raw mut TO.clkevt,
        timer_of_rate(&raw mut TO),
        TIMER_SYNC_TICKS,
        0xffff_ffff,
    );

    0
}

// TIMER_OF_DECLARE(mtk_mt6795, "mediatek,mt6795-systimer", mtk_cpux_init);

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
