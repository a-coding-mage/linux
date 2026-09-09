// SPDX-License-Identifier: GPL-2.0
// Copyright (C) 2012 Broadcom Corporation

// Linux kernel dependencies are supplied by the surrounding translation unit.

const KONA_GPTIMER_STCS_OFFSET: usize = 0x00000000;
const KONA_GPTIMER_STCLO_OFFSET: usize = 0x00000004;
const KONA_GPTIMER_STCHI_OFFSET: usize = 0x00000008;
const KONA_GPTIMER_STCM0_OFFSET: usize = 0x0000000C;

const KONA_GPTIMER_STCS_TIMER_MATCH_SHIFT: u32 = 0;
const KONA_GPTIMER_STCS_COMPARE_ENABLE_SHIFT: u32 = 4;

#[repr(C)]
struct kona_bcm_timers {
    tmr_irq: i32,
    tmr_regs: *mut core::ffi::c_void,
}

static mut timers: kona_bcm_timers = kona_bcm_timers {
    tmr_irq: 0,
    tmr_regs: core::ptr::null_mut(),
};

static mut arch_timer_rate: u32 = 0;

extern "C" {
    fn readl(addr: *mut core::ffi::c_void) -> u32;
    fn writel(value: u32, addr: *mut core::ffi::c_void);
    fn pr_err(fmt: *const core::ffi::c_char, ...);
}

/*
 * We use the peripheral timers for system tick, the cpu global timer for
 * profile tick
 */
unsafe fn kona_timer_disable_and_clear(base: *mut core::ffi::c_void) {
    let mut reg: u32;

    /*
     * clear and disable interrupts
     * We are using compare/match register 0 for our system interrupts
     */
    reg = readl(base.add(KONA_GPTIMER_STCS_OFFSET));

    /* Clear compare (0) interrupt */
    reg |= 1u32 << KONA_GPTIMER_STCS_TIMER_MATCH_SHIFT;
    /* disable compare */
    reg &= !(1u32 << KONA_GPTIMER_STCS_COMPARE_ENABLE_SHIFT);

    writel(reg, base.add(KONA_GPTIMER_STCS_OFFSET));
}

unsafe fn kona_timer_get_counter(
    timer_base: *mut core::ffi::c_void,
    msw: *mut u32,
    lsw: *mut u32,
) -> i32 {
    let mut loop_limit = 3;

    /*
     * Read 64-bit free running counter
     * 1. Read hi-word
     * 2. Read low-word
     * 3. Read hi-word again
     * 4.1
     *      if new hi-word is not equal to previously read hi-word, then
     *      start from #1
     * 4.2
     *      if new hi-word is equal to previously read hi-word then stop.
     */
    loop {
        *msw = readl(timer_base.add(KONA_GPTIMER_STCHI_OFFSET));
        *lsw = readl(timer_base.add(KONA_GPTIMER_STCLO_OFFSET));
        if *msw == readl(timer_base.add(KONA_GPTIMER_STCHI_OFFSET)) {
            break;
        }
        loop_limit -= 1;
        if loop_limit == 0 {
            break;
        }
    }
    if loop_limit == 0 {
        pr_err(b"bcm_kona_timer: getting counter failed.\0".as_ptr() as *const _);
        pr_err(b" Timer will be impacted\n\0".as_ptr() as *const _);
        return -110; // -ETIMEDOUT
    }

    0
}

unsafe fn kona_timer_set_next_event(clc: usize, _unused: *mut clock_event_device) -> i32 {
    /*
     * timer (0) is disabled by the timer interrupt already
     * so, here we reload the next event value and re-enable
     * the timer.
     *
     * This way, we are potentially losing the time between
     * timer-interrupt->set_next_event. CPU local timers, when
     * they come in should get rid of skew.
     */
    let mut lsw: u32 = 0;
    let mut msw: u32 = 0;
    let mut reg: u32;

    let ret = kona_timer_get_counter(timers.tmr_regs, &mut msw, &mut lsw);
    if ret != 0 {
        return ret;
    }

    /* Load the "next" event tick value */
    writel(lsw.wrapping_add(clc as u32), timers.tmr_regs.add(KONA_GPTIMER_STCM0_OFFSET));

    /* Enable compare */
    reg = readl(timers.tmr_regs.add(KONA_GPTIMER_STCS_OFFSET));
    reg |= 1u32 << KONA_GPTIMER_STCS_COMPARE_ENABLE_SHIFT;
    writel(reg, timers.tmr_regs.add(KONA_GPTIMER_STCS_OFFSET));

    0
}

unsafe fn kona_timer_shutdown(_evt: *mut clock_event_device) -> i32 {
    kona_timer_disable_and_clear(timers.tmr_regs);
    0
}

#[repr(C)]
struct clock_event_device {
    name: *const core::ffi::c_char,
    features: u32,
    set_next_event: Option<unsafe fn(usize, *mut clock_event_device) -> i32>,
    set_state_shutdown: Option<unsafe fn(*mut clock_event_device) -> i32>,
    tick_resume: Option<unsafe fn(*mut clock_event_device) -> i32>,
    cpumask: *mut core::ffi::c_void,
    event_handler: Option<unsafe fn(*mut clock_event_device)>,
}

static mut kona_clockevent_timer: clock_event_device = clock_event_device {
    name: b"timer 1\0".as_ptr() as *const _,
    features: 1, // CLOCK_EVT_FEAT_ONESHOT
    set_next_event: Some(kona_timer_set_next_event),
    set_state_shutdown: Some(kona_timer_shutdown),
    tick_resume: Some(kona_timer_shutdown),
    cpumask: core::ptr::null_mut(),
    event_handler: None,
};

extern "C" {
    fn cpumask_of(cpu: u32) -> *mut core::ffi::c_void;
    fn clockevents_config_and_register(dev: *mut clock_event_device, freq: u32, min_delta: u32, max_delta: u32);
}

unsafe fn kona_timer_clockevents_init() {
    kona_clockevent_timer.cpumask = cpumask_of(0);
    clockevents_config_and_register(&mut kona_clockevent_timer, arch_timer_rate, 6, 0xffff_ffff);
}

unsafe fn kona_timer_interrupt(_irq: i32, _dev_id: *mut core::ffi::c_void) -> i32 {
    let evt = &mut kona_clockevent_timer;

    kona_timer_disable_and_clear(timers.tmr_regs);
    if let Some(handler) = evt.event_handler {
        handler(evt);
    }
    1 // IRQ_HANDLED
}

#[repr(C)]
struct device_node {
    _opaque: [u8; 0],
}

extern "C" {
    fn of_clk_get_by_name(node: *mut device_node, name: *const core::ffi::c_char) -> *mut core::ffi::c_void;
    fn clk_get_rate(clk: *mut core::ffi::c_void) -> u32;
    fn clk_prepare_enable(clk: *mut core::ffi::c_void) -> i32;
    fn of_property_read_u32(node: *mut device_node, propname: *const core::ffi::c_char, out: *mut u32) -> i32;
    fn irq_of_parse_and_map(node: *mut device_node, index: u32) -> i32;
    fn of_iomap(node: *mut device_node, index: u32) -> *mut core::ffi::c_void;
    fn request_irq(irq: i32, handler: unsafe fn(i32, *mut core::ffi::c_void) -> i32, flags: u32, name: *const core::ffi::c_char, dev: *mut core::ffi::c_void) -> i32;
    static HZ: u32;
}

unsafe fn kona_timer_init(node: *mut device_node) -> i32 {
    let mut freq: u32 = 0;
    let external_clk = of_clk_get_by_name(node, core::ptr::null());

    if !external_clk.is_null() {
        arch_timer_rate = clk_get_rate(external_clk);
        clk_prepare_enable(external_clk);
    } else if of_property_read_u32(node, b"clock-frequency\0".as_ptr() as *const _, &mut freq) == 0 {
        arch_timer_rate = freq;
    } else {
        pr_err(b"Kona Timer v1 unable to determine clock-frequency\n\0".as_ptr() as *const _);
        return -22; // -EINVAL
    }

    /* Setup IRQ numbers */
    timers.tmr_irq = irq_of_parse_and_map(node, 0);

    /* Setup IO addresses */
    timers.tmr_regs = of_iomap(node, 0);

    kona_timer_disable_and_clear(timers.tmr_regs);

    kona_timer_clockevents_init();
    if request_irq(timers.tmr_irq, kona_timer_interrupt, 0x0000_0080, b"Kona Timer Tick\0".as_ptr() as *const _, core::ptr::null_mut()) != 0 {
        pr_err(b"%s: request_irq() failed\n\0".as_ptr() as *const _, b"Kona Timer Tick\0".as_ptr());
    }
    kona_timer_set_next_event((arch_timer_rate / HZ) as usize, core::ptr::null_mut());

    0
}

// Device-tree registration declarations are provided by the kernel integration.
// TIMER_OF_DECLARE(brcm_kona, "brcm,kona-timer", kona_timer_init);
// TIMER_OF_DECLARE(bcm_kona, "bcm,kona-timer", kona_timer_init);

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
