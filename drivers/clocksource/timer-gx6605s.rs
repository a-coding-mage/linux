// SPDX-License-Identifier: GPL-2.0
// Copyright (C) 2018 Hangzhou C-SKY Microsystems co.,ltd.

// External kernel declarations supplied by the surrounding build.
use core::ffi::c_void;

const CLKSRC_OFFSET: usize = 0x40;

const TIMER_STATUS: usize = 0x00;
const TIMER_VALUE: usize = 0x04;
const TIMER_CONTRL: usize = 0x10;
const TIMER_CONFIG: usize = 0x20;
const TIMER_DIV: usize = 0x24;
const TIMER_INI: usize = 0x28;

const GX6605S_STATUS_CLR: u32 = 1 << 0;
const GX6605S_CONTRL_RST: u32 = 1 << 0;
const GX6605S_CONTRL_START: u32 = 1 << 1;
const GX6605S_CONFIG_EN: u32 = 1 << 0;
const GX6605S_CONFIG_IRQ_EN: u32 = 1 << 1;

type Irqreturn = isize;
type DeviceNode = c_void;
type Iomem = c_void;

#[repr(C)]
struct ClockEventDevice {
    rating: i32,
    features: u32,
    set_state_shutdown: Option<unsafe extern "C" fn(*mut ClockEventDevice) -> i32>,
    set_state_oneshot: Option<unsafe extern "C" fn(*mut ClockEventDevice) -> i32>,
    set_next_event: Option<unsafe extern "C" fn(usize, *mut ClockEventDevice) -> i32>,
    cpumask: *const c_void,
    event_handler: Option<unsafe extern "C" fn(*mut ClockEventDevice)>,
}

#[repr(C)]
struct TimerOfIrq {
    handler: Option<unsafe extern "C" fn(i32, *mut c_void) -> Irqreturn>,
    flags: u32,
}

#[repr(C)]
struct TimerOf {
    flags: u32,
    clkevt: ClockEventDevice,
    of_irq: TimerOfIrq,
}

extern "C" {
    static mut cpu_possible_mask: c_void;
    fn timer_of_base(to: *const TimerOf) -> *mut Iomem;
    fn to_timer_of(ce: *mut ClockEventDevice) -> *const TimerOf;
    fn timer_of_rate(to: *const TimerOf) -> u32;
    fn writel_relaxed(value: u32, address: *mut Iomem);
    fn readl_relaxed(address: *mut Iomem) -> u32;
    fn clockevents_config_and_register(
        ce: *mut ClockEventDevice,
        freq: u32,
        min_delta: u32,
        max_delta: usize,
    );
    fn sched_clock_register(read: unsafe extern "C" fn() -> u64, bits: u32, rate: u32);
    fn clocksource_mmio_init(
        base: *mut Iomem,
        name: *const u8,
        freq: u32,
        rating: u32,
        bits: u32,
        read: *const c_void,
    ) -> i32;
    fn clocksource_mmio_readl_up() -> u64;
    fn timer_of_init(np: *mut DeviceNode, to: *mut TimerOf) -> i32;
}

const TIMER_OF_IRQ: u32 = 1 << 0;
const TIMER_OF_BASE: u32 = 1 << 1;
const TIMER_OF_CLOCK: u32 = 1 << 2;
const CLOCK_EVT_FEAT_DYNIRQ: u32 = 1 << 0;
const CLOCK_EVT_FEAT_ONESHOT: u32 = 1 << 1;
const IRQF_TIMER: u32 = 1 << 0;
const IRQF_IRQPOLL: u32 = 1 << 1;

unsafe extern "C" fn gx6605s_timer_interrupt(_irq: i32, dev: *mut c_void) -> Irqreturn {
    let ce = dev as *mut ClockEventDevice;
    let base = timer_of_base(to_timer_of(ce));

    writel_relaxed(GX6605S_STATUS_CLR, base.add(TIMER_STATUS));
    writel_relaxed(0, base.add(TIMER_INI));
    if let Some(handler) = (*ce).event_handler {
        handler(ce);
    }
    1
}

unsafe extern "C" fn gx6605s_timer_set_oneshot(ce: *mut ClockEventDevice) -> i32 {
    let base = timer_of_base(to_timer_of(ce));
    // reset and stop counter
    writel_relaxed(GX6605S_CONTRL_RST, base.add(TIMER_CONTRL));
    // enable with irq and start
    writel_relaxed(GX6605S_CONFIG_EN | GX6605S_CONFIG_IRQ_EN, base.add(TIMER_CONFIG));
    0
}

unsafe extern "C" fn gx6605s_timer_set_next_event(delta: usize, ce: *mut ClockEventDevice) -> i32 {
    let base = timer_of_base(to_timer_of(ce));
    // use reset to pause timer
    writel_relaxed(GX6605S_CONTRL_RST, base.add(TIMER_CONTRL));
    // config next timeout value
    writel_relaxed((usize::MAX - delta) as u32, base.add(TIMER_INI));
    writel_relaxed(GX6605S_CONTRL_START, base.add(TIMER_CONTRL));
    0
}

unsafe extern "C" fn gx6605s_timer_shutdown(ce: *mut ClockEventDevice) -> i32 {
    let base = timer_of_base(to_timer_of(ce));
    writel_relaxed(0, base.add(TIMER_CONTRL));
    writel_relaxed(0, base.add(TIMER_CONFIG));
    0
}

static mut TO: TimerOf = TimerOf {
    flags: TIMER_OF_IRQ | TIMER_OF_BASE | TIMER_OF_CLOCK,
    clkevt: ClockEventDevice {
        rating: 300,
        features: CLOCK_EVT_FEAT_DYNIRQ | CLOCK_EVT_FEAT_ONESHOT,
        set_state_shutdown: Some(gx6605s_timer_shutdown),
        set_state_oneshot: Some(gx6605s_timer_set_oneshot),
        set_next_event: Some(gx6605s_timer_set_next_event),
        cpumask: unsafe { &cpu_possible_mask },
        event_handler: None,
    },
    of_irq: TimerOfIrq {
        handler: Some(gx6605s_timer_interrupt),
        flags: IRQF_TIMER | IRQF_IRQPOLL,
    },
};

unsafe extern "C" fn gx6605s_sched_clock_read() -> u64 {
    let base = timer_of_base(&TO).add(CLKSRC_OFFSET);
    readl_relaxed(base.add(TIMER_VALUE)) as u64
}

unsafe fn gx6605s_clkevt_init(base: *mut Iomem) {
    writel_relaxed(0, base.add(TIMER_DIV));
    writel_relaxed(0, base.add(TIMER_CONFIG));
    clockevents_config_and_register(&mut TO.clkevt, timer_of_rate(&TO), 2, usize::MAX);
}

unsafe fn gx6605s_clksrc_init(base: *mut Iomem) -> i32 {
    writel_relaxed(0, base.add(TIMER_DIV));
    writel_relaxed(0, base.add(TIMER_INI));
    writel_relaxed(GX6605S_CONTRL_RST, base.add(TIMER_CONTRL));
    writel_relaxed(GX6605S_CONFIG_EN, base.add(TIMER_CONFIG));
    writel_relaxed(GX6605S_CONTRL_START, base.add(TIMER_CONTRL));
    sched_clock_register(gx6605s_sched_clock_read, 32, timer_of_rate(&TO));
    clocksource_mmio_init(
        base.add(TIMER_VALUE), b"gx6605s\0".as_ptr(), timer_of_rate(&TO), 200, 32,
        clocksource_mmio_readl_up as *const c_void,
    )
}

unsafe extern "C" fn gx6605s_timer_init(np: *mut DeviceNode) -> i32 {
    let ret = timer_of_init(np, &mut TO);
    if ret != 0 {
        return ret;
    }
    gx6605s_clkevt_init(timer_of_base(&TO));
    gx6605s_clksrc_init(timer_of_base(&TO).add(CLKSRC_OFFSET))
}

// TIMER_OF_DECLARE(csky_gx6605s_timer, "csky,gx6605s-timer", gx6605s_timer_init)

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
