// SPDX-License-Identifier: GPL-2.0
/*
 * Copyright (C) 2018 Socionext Inc.
 */

// External Linux kernel types, functions, constants, and the timer-of
// declaration supplied by the corresponding kernel headers are referenced
// below.

const MLB_TMR_TMCSR_OFS: usize = 0x0;
const MLB_TMR_TMR_OFS: usize = 0x4;
const MLB_TMR_TMRLR1_OFS: usize = 0x8;
const MLB_TMR_TMRLR2_OFS: usize = 0xc;
const MLB_TMR_REGSZPCH: usize = 0x10;

const MLB_TMR_TMCSR_OUTL: u32 = 1 << 5;
const MLB_TMR_TMCSR_RELD: u32 = 1 << 4;
const MLB_TMR_TMCSR_INTE: u32 = 1 << 3;
const MLB_TMR_TMCSR_UF: u32 = 1 << 2;
const MLB_TMR_TMCSR_CNTE: u32 = 1 << 1;
const MLB_TMR_TMCSR_TRG: u32 = 1 << 0;

const MLB_TMR_TMCSR_CSL_DIV2: u32 = 0;
const MLB_TMR_DIV_CNT: usize = 2;

const MLB_TMR_SRC_CH: usize = 1;
const MLB_TMR_EVT_CH: usize = 0;

const MLB_TMR_SRC_CH_OFS: usize = MLB_TMR_REGSZPCH * MLB_TMR_SRC_CH;
const MLB_TMR_EVT_CH_OFS: usize = MLB_TMR_REGSZPCH * MLB_TMR_EVT_CH;

const MLB_TMR_SRC_TMCSR_OFS: usize = MLB_TMR_SRC_CH_OFS + MLB_TMR_TMCSR_OFS;
const MLB_TMR_SRC_TMR_OFS: usize = MLB_TMR_SRC_CH_OFS + MLB_TMR_TMR_OFS;
const MLB_TMR_SRC_TMRLR1_OFS: usize = MLB_TMR_SRC_CH_OFS + MLB_TMR_TMRLR1_OFS;
const MLB_TMR_SRC_TMRLR2_OFS: usize = MLB_TMR_SRC_CH_OFS + MLB_TMR_TMRLR2_OFS;

const MLB_TMR_EVT_TMCSR_OFS: usize = MLB_TMR_EVT_CH_OFS + MLB_TMR_TMCSR_OFS;
const MLB_TMR_EVT_TMR_OFS: usize = MLB_TMR_EVT_CH_OFS + MLB_TMR_TMR_OFS;
const MLB_TMR_EVT_TMRLR1_OFS: usize = MLB_TMR_EVT_CH_OFS + MLB_TMR_TMRLR1_OFS;
const MLB_TMR_EVT_TMRLR2_OFS: usize = MLB_TMR_EVT_CH_OFS + MLB_TMR_TMRLR2_OFS;

const MLB_TIMER_RATING: i32 = 500;
const MLB_TIMER_ONESHOT: bool = false;
const MLB_TIMER_PERIODIC: bool = true;

unsafe fn mlb_timer_interrupt(irq: i32, dev_id: *mut core::ffi::c_void) -> irqreturn_t {
    let clk = dev_id as *mut clock_event_device;
    let to = to_timer_of(clk);
    let mut val: u32;

    val = readl_relaxed(timer_of_base(to).add(MLB_TMR_EVT_TMCSR_OFS));
    val &= !MLB_TMR_TMCSR_UF;
    writel_relaxed(val, timer_of_base(to).add(MLB_TMR_EVT_TMCSR_OFS));

    ((*clk).event_handler)(clk);

    IRQ_HANDLED
}

unsafe fn mlb_evt_timer_start(to: *mut timer_of, periodic: bool) {
    let mut val: u32 = MLB_TMR_TMCSR_CSL_DIV2;

    val |= MLB_TMR_TMCSR_CNTE | MLB_TMR_TMCSR_TRG | MLB_TMR_TMCSR_INTE;
    if periodic {
        val |= MLB_TMR_TMCSR_RELD;
    }
    writel_relaxed(val, timer_of_base(to).add(MLB_TMR_EVT_TMCSR_OFS));
}

unsafe fn mlb_evt_timer_stop(to: *mut timer_of) {
    let mut val: u32 = readl_relaxed(timer_of_base(to).add(MLB_TMR_EVT_TMCSR_OFS));

    val &= !MLB_TMR_TMCSR_CNTE;
    writel_relaxed(val, timer_of_base(to).add(MLB_TMR_EVT_TMCSR_OFS));
}

unsafe fn mlb_evt_timer_register_count(to: *mut timer_of, cnt: c_ulong) {
    writel_relaxed(cnt as u32, timer_of_base(to).add(MLB_TMR_EVT_TMRLR1_OFS));
}

unsafe fn mlb_set_state_periodic(clk: *mut clock_event_device) -> i32 {
    let to = to_timer_of(clk);

    mlb_evt_timer_stop(to);
    mlb_evt_timer_register_count(to, (*to).of_clk.period);
    mlb_evt_timer_start(to, MLB_TIMER_PERIODIC);
    0
}

unsafe fn mlb_set_state_oneshot(clk: *mut clock_event_device) -> i32 {
    let to = to_timer_of(clk);

    mlb_evt_timer_stop(to);
    mlb_evt_timer_start(to, MLB_TIMER_ONESHOT);
    0
}

unsafe fn mlb_set_state_shutdown(clk: *mut clock_event_device) -> i32 {
    let to = to_timer_of(clk);

    mlb_evt_timer_stop(to);
    0
}

unsafe fn mlb_clkevt_next_event(event: c_ulong, clk: *mut clock_event_device) -> i32 {
    let to = to_timer_of(clk);

    mlb_evt_timer_stop(to);
    mlb_evt_timer_register_count(to, event);
    mlb_evt_timer_start(to, MLB_TIMER_ONESHOT);
    0
}

unsafe fn mlb_config_clock_source(to: *mut timer_of) -> i32 {
    let mut val: u32 = MLB_TMR_TMCSR_CSL_DIV2;

    writel_relaxed(val, timer_of_base(to).add(MLB_TMR_SRC_TMCSR_OFS));
    writel_relaxed(!0, timer_of_base(to).add(MLB_TMR_SRC_TMRLR1_OFS));
    writel_relaxed(!0, timer_of_base(to).add(MLB_TMR_SRC_TMRLR2_OFS));
    val |= MLB_TMR_TMCSR_RELD | MLB_TMR_TMCSR_CNTE | MLB_TMR_TMCSR_TRG;
    writel_relaxed(val, timer_of_base(to).add(MLB_TMR_SRC_TMCSR_OFS));
    0
}

unsafe fn mlb_config_clock_event(to: *mut timer_of) -> i32 {
    writel_relaxed(0, timer_of_base(to).add(MLB_TMR_EVT_TMCSR_OFS));
    0
}

static mut TO: timer_of = timer_of {
    flags: TIMER_OF_IRQ | TIMER_OF_BASE | TIMER_OF_CLOCK,
    clkevt: clock_event_device {
        name: "mlb-clkevt",
        rating: MLB_TIMER_RATING,
        cpumask: cpu_possible_mask,
        features: CLOCK_EVT_FEAT_DYNIRQ | CLOCK_EVT_FEAT_ONESHOT,
        set_state_oneshot: Some(mlb_set_state_oneshot),
        set_state_periodic: Some(mlb_set_state_periodic),
        set_state_shutdown: Some(mlb_set_state_shutdown),
        set_next_event: Some(mlb_clkevt_next_event),
    },
    of_irq: timer_of_irq {
        flags: IRQF_TIMER | IRQF_IRQPOLL,
        handler: Some(mlb_timer_interrupt),
    },
};

unsafe fn mlb_timer_sched_read() -> u64 {
    !(readl_relaxed(timer_of_base(&mut TO).add(MLB_TMR_SRC_TMR_OFS)) as u64)
}

unsafe fn mlb_timer_init(node: *mut device_node) -> i32 {
    let ret: i32;
    let rate: c_ulong;

    ret = timer_of_init(node, &mut TO);
    if ret != 0 {
        return ret;
    }

    rate = timer_of_rate(&mut TO) / MLB_TMR_DIV_CNT as c_ulong;
    mlb_config_clock_source(&mut TO);
    clocksource_mmio_init(
        timer_of_base(&mut TO).add(MLB_TMR_SRC_TMR_OFS),
        (*node).name,
        rate,
        MLB_TIMER_RATING,
        32,
        clocksource_mmio_readl_down,
    );
    sched_clock_register(mlb_timer_sched_read, 32, rate);
    mlb_config_clock_event(&mut TO);
    clockevents_config_and_register(&mut TO.clkevt, timer_of_rate(&mut TO), 15, 0xffffffff);
    0
}

TIMER_OF_DECLARE!(mlb_peritimer, "socionext,milbeaut-timer", mlb_timer_init);

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
