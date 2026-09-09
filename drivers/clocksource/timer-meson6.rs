// SPDX-License-Identifier: GPL-2.0
/*
 * Amlogic Meson6 SoCs timer handling.
 *
 * Copyright (C) 2014 Carlo Caione <carlo@caione.org>
 *
 * Based on code from Amlogic, Inc
 */

// Linux header dependencies are supplied by other translated files.

const MESON_ISA_TIMER_MUX: usize = 0x00;
const MESON_ISA_TIMER_MUX_TIMERD_EN: u32 = 1 << 19;
const MESON_ISA_TIMER_MUX_TIMERC_EN: u32 = 1 << 18;
const MESON_ISA_TIMER_MUX_TIMERB_EN: u32 = 1 << 17;
const MESON_ISA_TIMER_MUX_TIMERA_EN: u32 = 1 << 16;
const MESON_ISA_TIMER_MUX_TIMERD_MODE: u32 = 1 << 15;
const MESON_ISA_TIMER_MUX_TIMERC_MODE: u32 = 1 << 14;
const MESON_ISA_TIMER_MUX_TIMERB_MODE: u32 = 1 << 13;
const MESON_ISA_TIMER_MUX_TIMERA_MODE: u32 = 1 << 12;
const MESON_ISA_TIMER_MUX_TIMERE_INPUT_CLOCK_MASK: u32 = 0x7 << 8;
const MESON_ISA_TIMER_MUX_TIMERE_INPUT_CLOCK_SYSTEM_CLOCK: u32 = 0x0;
const MESON_ISA_TIMER_MUX_TIMERE_INPUT_CLOCK_1US: u32 = 0x1;
const MESON_ISA_TIMER_MUX_TIMERE_INPUT_CLOCK_10US: u32 = 0x2;
const MESON_ISA_TIMER_MUX_TIMERE_INPUT_CLOCK_100US: u32 = 0x3;
const MESON_ISA_TIMER_MUX_TIMERE_INPUT_CLOCK_1MS: u32 = 0x4;
const MESON_ISA_TIMER_MUX_TIMERD_INPUT_CLOCK_MASK: u32 = 0x3 << 6;
const MESON_ISA_TIMER_MUX_TIMERC_INPUT_CLOCK_MASK: u32 = 0x3 << 4;
const MESON_ISA_TIMER_MUX_TIMERB_INPUT_CLOCK_MASK: u32 = 0x3 << 2;
const MESON_ISA_TIMER_MUX_TIMERA_INPUT_CLOCK_MASK: u32 = 0x3;
const MESON_ISA_TIMER_MUX_TIMERABCD_INPUT_CLOCK_1US: u32 = 0x0;
const MESON_ISA_TIMER_MUX_TIMERABCD_INPUT_CLOCK_10US: u32 = 0x1;
const MESON_ISA_TIMER_MUX_TIMERABCD_INPUT_CLOCK_100US: u32 = 0x0;
const MESON_ISA_TIMER_MUX_TIMERABCD_INPUT_CLOCK_1MS: u32 = 0x3;

const MESON_ISA_TIMERA: usize = 0x04;
const MESON_ISA_TIMERB: usize = 0x08;
const MESON_ISA_TIMERC: usize = 0x0c;
const MESON_ISA_TIMERD: usize = 0x10;
const MESON_ISA_TIMERE: usize = 0x14;

static mut timer_base: *mut u8 = core::ptr::null_mut();

#[cfg(CONFIG_ARM)]
unsafe extern "C" {
    fn readl_relaxed(addr: *mut u8) -> u32;
    fn register_current_timer_delay(timer: *mut delay_timer);
}

#[cfg(CONFIG_ARM)]
static mut meson6_delay_timer: delay_timer = delay_timer {
    read_current_timer: Some(meson6_read_current_timer),
    freq: 1000 * 1000,
};

#[cfg(CONFIG_ARM)]
unsafe fn meson6_read_current_timer() -> usize {
    readl_relaxed(timer_base.add(MESON_ISA_TIMERE)) as usize
}

unsafe fn meson6_timer_sched_read() -> u64 {
    readl(timer_base.add(MESON_ISA_TIMERE)) as u64
}

unsafe fn meson6_clkevt_time_stop() {
    let val = readl(timer_base.add(MESON_ISA_TIMER_MUX));
    writel(val & !MESON_ISA_TIMER_MUX_TIMERA_EN, timer_base.add(MESON_ISA_TIMER_MUX));
}

unsafe fn meson6_clkevt_time_setup(delay: usize) {
    writel(delay as u32, timer_base.add(MESON_ISA_TIMERA));
}

unsafe fn meson6_clkevt_time_start(periodic: bool) {
    let mut val = readl(timer_base.add(MESON_ISA_TIMER_MUX));
    if periodic { val |= MESON_ISA_TIMER_MUX_TIMERA_MODE; }
    else { val &= !MESON_ISA_TIMER_MUX_TIMERA_MODE; }
    writel(val | MESON_ISA_TIMER_MUX_TIMERA_EN, timer_base.add(MESON_ISA_TIMER_MUX));
}

unsafe fn meson6_shutdown(_evt: *mut clock_event_device) -> i32 { meson6_clkevt_time_stop(); 0 }
unsafe fn meson6_set_oneshot(_evt: *mut clock_event_device) -> i32 {
    meson6_clkevt_time_stop(); meson6_clkevt_time_start(false); 0
}
unsafe fn meson6_set_periodic(_evt: *mut clock_event_device) -> i32 {
    meson6_clkevt_time_stop(); meson6_clkevt_time_setup(USEC_PER_SEC / HZ - 1); meson6_clkevt_time_start(true); 0
}
unsafe fn meson6_clkevt_next_event(evt: usize, _unused: *mut clock_event_device) -> i32 {
    meson6_clkevt_time_stop(); meson6_clkevt_time_setup(evt); meson6_clkevt_time_start(false); 0
}

static mut meson6_clockevent: clock_event_device = clock_event_device {
    name: c"meson6_tick".as_ptr(), rating: 400,
    features: CLOCK_EVT_FEAT_PERIODIC | CLOCK_EVT_FEAT_ONESHOT,
    set_state_shutdown: Some(meson6_shutdown), set_state_periodic: Some(meson6_set_periodic),
    set_state_oneshot: Some(meson6_set_oneshot), tick_resume: Some(meson6_shutdown),
    set_next_event: Some(meson6_clkevt_next_event),
};

unsafe fn meson6_timer_interrupt(_irq: i32, dev_id: *mut core::ffi::c_void) -> irqreturn_t {
    let evt = dev_id as *mut clock_event_device;
    ((*evt).event_handler)(evt); IRQ_HANDLED
}

unsafe fn meson6_timer_init(node: *mut device_node) -> i32 {
    let mut val: u32;
    let (mut ret, mut irq): (i32, i32);
    timer_base = of_io_request_and_map(node, 0, c"meson6-timer".as_ptr());
    if IS_ERR(timer_base) { pr_err(c"Can't map registers\n"); return -ENXIO; }
    irq = irq_of_parse_and_map(node, 0);
    if irq <= 0 { pr_err(c"Can't parse IRQ\n"); return -EINVAL; }
    val = readl(timer_base.add(MESON_ISA_TIMER_MUX));
    val &= !MESON_ISA_TIMER_MUX_TIMERE_INPUT_CLOCK_MASK;
    val |= (MESON_ISA_TIMER_MUX_TIMERE_INPUT_CLOCK_1US << 8) & MESON_ISA_TIMER_MUX_TIMERE_INPUT_CLOCK_MASK;
    writel(val, timer_base.add(MESON_ISA_TIMER_MUX));
    sched_clock_register(meson6_timer_sched_read, 32, USEC_PER_SEC);
    clocksource_mmio_init(timer_base.add(MESON_ISA_TIMERE), (*node).name, 1000 * 1000, 300, 32, clocksource_mmio_readl_up);
    val &= !MESON_ISA_TIMER_MUX_TIMERA_INPUT_CLOCK_MASK;
    val |= MESON_ISA_TIMER_MUX_TIMERABCD_INPUT_CLOCK_1US;
    writel(val, timer_base.add(MESON_ISA_TIMER_MUX));
    meson6_clkevt_time_stop();
    ret = request_irq(irq, Some(meson6_timer_interrupt), IRQF_TIMER | IRQF_IRQPOLL, c"meson6_timer".as_ptr(), &raw mut meson6_clockevent as *mut _ as *mut core::ffi::c_void);
    if ret != 0 { pr_warn(c"failed to setup irq %d\n", irq); return ret; }
    meson6_clockevent.cpumask = cpu_possible_mask; meson6_clockevent.irq = irq;
    clockevents_config_and_register(&raw mut meson6_clockevent, USEC_PER_SEC, 1, 0xfffe);
    #[cfg(CONFIG_ARM)] register_current_timer_delay(&raw mut meson6_delay_timer);
    0
}

TIMER_OF_DECLARE!(meson6, c"amlogic,meson6-timer", meson6_timer_init);

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
