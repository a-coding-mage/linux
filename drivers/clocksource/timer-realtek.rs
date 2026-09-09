// SPDX-License-Identifier: GPL-2.0-only
/*
 * Copyright (c) 2025 Realtek Semiconductor Corp.
 */

// Dependency equivalents supplied by the kernel and timer-of support.
use core::ffi::c_void;

const ENBL: u32 = 1;
const DSBL: u32 = 0;

const SYSTIMER_RATE: u32 = 1_000_000;
const SYSTIMER_MIN_DELTA: u64 = 0x64;
const SYSTIMER_MAX_DELTA: u64 = u64::MAX;

/* SYSTIMER Register Offset (RTK Internal Use) */
const TS_LW_OFST: usize = 0x0;
const TS_HW_OFST: usize = 0x4;
const TS_CMP_VAL_LW_OFST: usize = 0x8;
const TS_CMP_VAL_HW_OFST: usize = 0xC;
const TS_CMP_CTRL_OFST: usize = 0x10;
const TS_CMP_STAT_OFST: usize = 0x14;

/* SYSTIMER CMP CTRL REG Mask */
const TS_CMP_EN_MASK: u32 = 0x1;
const TS_WR_EN0_MASK: u32 = 0x2;

extern "C" {
    static mut cpu_possible_mask: *const c_void;

    fn readl(addr: *mut c_void) -> u32;
    fn writel(value: u32, addr: *mut c_void);
    fn timer_of_init(node: *mut device_node, timer: *mut timer_of) -> i32;
    fn timer_of_base(timer: *mut timer_of) -> *mut c_void;
    fn clockevents_config_and_register(
        clkevt: *mut clock_event_device,
        freq: u32,
        min_delta: u64,
        max_delta: u64,
    );
}

#[repr(C)]
pub struct device_node {
    _private: [u8; 0],
}

#[repr(C)]
pub struct clock_event_device {
    pub name: *const u8,
    pub rating: i32,
    pub cpumask: *const c_void,
    pub features: u32,
    pub set_next_event: Option<unsafe extern "C" fn(unsigned_long: usize, clkevt: *mut clock_event_device) -> i32>,
    pub set_state_oneshot: Option<unsafe extern "C" fn(clkevt: *mut clock_event_device) -> i32>,
    pub set_state_shutdown: Option<unsafe extern "C" fn(clkevt: *mut clock_event_device) -> i32>,
    pub event_handler: unsafe extern "C" fn(clkevt: *mut clock_event_device),
}

#[repr(C)]
pub struct timer_of_irq {
    pub flags: u32,
    pub handler: Option<unsafe extern "C" fn(irq: i32, dev_id: *mut c_void) -> irqreturn_t>,
}

#[repr(C)]
pub struct timer_of {
    pub flags: u32,
    pub clkevt: clock_event_device,
    pub of_irq: timer_of_irq,
}

pub type unsigned_long = usize;
pub type irqreturn_t = i32;

const TIMER_OF_IRQ: u32 = 1 << 0;
const TIMER_OF_BASE: u32 = 1 << 1;
const CLOCK_EVT_FEAT_DYNIRQ: u32 = 1 << 0;
const CLOCK_EVT_FEAT_ONESHOT: u32 = 1 << 1;
const IRQF_TIMER: u32 = 1 << 0;
const IRQF_IRQPOLL: u32 = 1 << 1;
const IRQ_HANDLED: irqreturn_t = 1;

static mut systimer_base: *mut c_void = core::ptr::null_mut();

unsafe fn rtk_ts64_read() -> u64 {
    let low: u32;
    let high: u32;
    let ts: u64;

    /* Caution: Read LSB word (TS_LW_OFST) first then MSB (TS_HW_OFST) */
    low = readl(systimer_base.add(TS_LW_OFST));
    high = readl(systimer_base.add(TS_HW_OFST));
    ts = ((high as u64) << 32) | low as u64;

    ts
}

unsafe fn rtk_cmp_value_write(value: u64) {
    let high: u32 = (value >> 32) as u32;
    let low: u32 = (value & 0xFFFF_FFFF) as u32;

    writel(high, systimer_base.add(TS_CMP_VAL_HW_OFST));
    writel(low, systimer_base.add(TS_CMP_VAL_LW_OFST));
}

#[inline]
unsafe fn rtk_cmp_en_write(cmp_en: bool) {
    let mut val: u32 = TS_WR_EN0_MASK;
    if cmp_en as u32 == ENBL {
        val |= TS_CMP_EN_MASK;
    }

    writel(val, systimer_base.add(TS_CMP_CTRL_OFST));
}

unsafe extern "C" fn rtk_syst_clkevt_next_event(
    cycles: usize,
    _clkevt: *mut clock_event_device,
) -> i32 {
    let cmp_val: u64;

    rtk_cmp_en_write(false);
    cmp_val = rtk_ts64_read();

    /* Set CMP value to current timestamp plus delta_us */
    rtk_cmp_value_write(cmp_val.wrapping_add(cycles as u64));
    rtk_cmp_en_write(true);
    0
}

unsafe extern "C" fn rtk_ts_match_intr_handler(
    _irq: i32,
    dev_id: *mut c_void,
) -> irqreturn_t {
    let clkevt = dev_id as *mut clock_event_device;
    let reg_base: *mut c_void;
    let val: u32;

    /* Disable TS CMP Match */
    rtk_cmp_en_write(false);

    /* Clear TS CMP INTR */
    reg_base = systimer_base.add(TS_CMP_STAT_OFST);
    val = readl(reg_base) & TS_CMP_EN_MASK;
    writel(val | TS_CMP_EN_MASK, reg_base);
    ((*clkevt).event_handler)(clkevt);

    IRQ_HANDLED
}

unsafe extern "C" fn rtk_syst_shutdown(_clkevt: *mut clock_event_device) -> i32 {
    let reg_base: *mut c_void;
    let cmp_val: u64 = 0;

    /* Disable TS CMP Match */
    rtk_cmp_en_write(false);
    /* Set compare value to 0 */
    rtk_cmp_value_write(cmp_val);

    /* Clear TS CMP INTR */
    reg_base = systimer_base.add(TS_CMP_STAT_OFST);
    writel(TS_CMP_EN_MASK, reg_base);
    0
}

static mut rtk_timer_to: timer_of = timer_of {
    flags: TIMER_OF_IRQ | TIMER_OF_BASE,
    clkevt: clock_event_device {
        name: b"rtk-clkevt\0".as_ptr(),
        rating: 300,
        cpumask: core::ptr::null(),
        features: CLOCK_EVT_FEAT_DYNIRQ | CLOCK_EVT_FEAT_ONESHOT,
        set_next_event: Some(rtk_syst_clkevt_next_event),
        set_state_oneshot: Some(rtk_syst_shutdown),
        set_state_shutdown: Some(rtk_syst_shutdown),
        event_handler: timer_event_handler_uninitialized,
    },
    of_irq: timer_of_irq {
        flags: IRQF_TIMER | IRQF_IRQPOLL,
        handler: Some(rtk_ts_match_intr_handler),
    },
};

unsafe extern "C" fn timer_event_handler_uninitialized(_clkevt: *mut clock_event_device) {}

unsafe extern "C" fn rtk_systimer_init(node: *mut device_node) -> i32 {
    let ret: i32;

    ret = timer_of_init(node, &raw mut rtk_timer_to);
    if ret != 0 {
        return ret;
    }

    systimer_base = timer_of_base(&raw mut rtk_timer_to);
    clockevents_config_and_register(
        &raw mut rtk_timer_to.clkevt,
        SYSTIMER_RATE,
        SYSTIMER_MIN_DELTA,
        SYSTIMER_MAX_DELTA,
    );

    0
}

// TIMER_OF_DECLARE(rtk_systimer, "realtek,rtd1625-systimer", rtk_systimer_init);

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
