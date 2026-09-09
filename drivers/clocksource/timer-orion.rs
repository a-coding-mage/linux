// SPDX-License-Identifier: GPL-2.0
/*
 * Marvell Orion SoC timer handling.
 *
 * Sebastian Hesselbarth <sebastian.hesselbarth@gmail.com>
 *
 * Timer 0 is used as free-running clocksource, while timer 1 is
 * used as clock_event_device.
 */

// Linux kernel dependencies supplied by other translation units.

const TIMER_CTRL: usize = 0x00;
const TIMER0_EN: u32 = 1 << 0;
const TIMER0_RELOAD_EN: u32 = 1 << 1;
const TIMER1_EN: u32 = 1 << 2;
const TIMER1_RELOAD_EN: u32 = 1 << 3;
const TIMER0_RELOAD: usize = 0x10;
const TIMER0_VAL: usize = 0x14;
const TIMER1_RELOAD: usize = 0x18;
const TIMER1_VAL: usize = 0x1c;

const ORION_ONESHOT_MIN: u64 = 1;
const ORION_ONESHOT_MAX: u64 = 0xfffffffe;

static mut timer_base: *mut core::ffi::c_void = core::ptr::null_mut();

unsafe extern "C" {
    fn readl(addr: *mut core::ffi::c_void) -> u32;
    fn writel(value: u32, addr: *mut core::ffi::c_void);
    fn atomic_io_modify(addr: *mut core::ffi::c_void, mask: u32, value: u32);
}

unsafe fn orion_read_timer() -> usize {
    (!readl(timer_base.add(TIMER0_VAL))) as usize
}

#[repr(C)]
struct delay_timer {
    read_current_timer: Option<unsafe fn() -> usize>,
    freq: usize,
}

static mut orion_delay_timer: delay_timer = delay_timer {
    read_current_timer: Some(orion_read_timer),
    freq: 0,
};

unsafe extern "C" {
    fn register_current_timer_delay(timer: *mut delay_timer);
}

unsafe fn orion_delay_timer_init(rate: usize) {
    orion_delay_timer.freq = rate;
    register_current_timer_delay(&raw mut orion_delay_timer);
}

/*
 * Free-running clocksource handling.
 */
unsafe fn orion_read_sched_clock() -> u64 {
    (!readl(timer_base.add(TIMER0_VAL))) as u64
}

/*
 * Clockevent handling.
 */
static mut ticks_per_jiffy: u32 = 0;

unsafe fn orion_clkevt_next_event(delta: usize, _dev: *mut clock_event_device) -> i32 {
    /* setup and enable one-shot timer */
    writel(delta as u32, timer_base.add(TIMER1_VAL));
    atomic_io_modify(
        timer_base.add(TIMER_CTRL),
        TIMER1_RELOAD_EN | TIMER1_EN,
        TIMER1_EN,
    );

    0
}

unsafe fn orion_clkevt_shutdown(_dev: *mut clock_event_device) -> i32 {
    /* disable timer */
    atomic_io_modify(timer_base.add(TIMER_CTRL), TIMER1_RELOAD_EN | TIMER1_EN, 0);
    0
}

unsafe fn orion_clkevt_set_periodic(_dev: *mut clock_event_device) -> i32 {
    /* setup and enable periodic timer at 1/HZ intervals */
    writel(ticks_per_jiffy - 1, timer_base.add(TIMER1_RELOAD));
    writel(ticks_per_jiffy - 1, timer_base.add(TIMER1_VAL));
    atomic_io_modify(
        timer_base.add(TIMER_CTRL),
        TIMER1_RELOAD_EN | TIMER1_EN,
        TIMER1_RELOAD_EN | TIMER1_EN,
    );
    0
}

#[repr(C)]
struct clock_event_device {
    name: *const u8,
    features: u32,
    shift: u32,
    rating: u32,
    set_next_event: Option<unsafe fn(usize, *mut clock_event_device) -> i32>,
    set_state_shutdown: Option<unsafe fn(*mut clock_event_device) -> i32>,
    set_state_periodic: Option<unsafe fn(*mut clock_event_device) -> i32>,
    set_state_oneshot: Option<unsafe fn(*mut clock_event_device) -> i32>,
    tick_resume: Option<unsafe fn(*mut clock_event_device) -> i32>,
    event_handler: Option<unsafe fn(*mut clock_event_device)>,
    cpumask: *const core::ffi::c_void,
    irq: i32,
}

const CLOCK_EVT_FEAT_ONESHOT: u32 = 1 << 0;
const CLOCK_EVT_FEAT_PERIODIC: u32 = 1 << 1;

static mut orion_clkevt: clock_event_device = clock_event_device {
    name: b"orion_event\0".as_ptr(),
    features: CLOCK_EVT_FEAT_ONESHOT | CLOCK_EVT_FEAT_PERIODIC,
    shift: 32,
    rating: 300,
    set_next_event: Some(orion_clkevt_next_event),
    set_state_shutdown: Some(orion_clkevt_shutdown),
    set_state_periodic: Some(orion_clkevt_set_periodic),
    set_state_oneshot: Some(orion_clkevt_shutdown),
    tick_resume: Some(orion_clkevt_shutdown),
    event_handler: None,
    cpumask: core::ptr::null(),
    irq: 0,
};

const IRQ_HANDLED: i32 = 1;

unsafe extern "C" fn orion_clkevt_irq_handler(_irq: i32, _dev_id: *mut core::ffi::c_void) -> i32 {
    if let Some(handler) = orion_clkevt.event_handler {
        handler(&raw mut orion_clkevt);
    }
    IRQ_HANDLED
}

#[repr(C)]
struct device_node;
#[repr(C)]
struct clk;

unsafe extern "C" {
    fn of_iomap(np: *mut device_node, index: i32) -> *mut core::ffi::c_void;
    fn of_clk_get(np: *mut device_node, index: i32) -> *mut clk;
    fn clk_prepare_enable(clk: *mut clk) -> i32;
    fn clk_disable_unprepare(clk: *mut clk);
    fn clk_get_rate(clk: *mut clk) -> usize;
    fn irq_of_parse_and_map(np: *mut device_node, index: i32) -> i32;
    fn clocksource_mmio_init(addr: *mut core::ffi::c_void, name: *const u8, rate: usize, rating: i32, bits: u32, read: unsafe extern "C" fn(*mut core::ffi::c_void) -> u32) -> i32;
    fn clocksource_mmio_readl_down(addr: *mut core::ffi::c_void) -> u32;
    fn sched_clock_register(read: unsafe fn() -> u64, bits: u32, rate: usize);
    fn request_irq(irq: i32, handler: unsafe extern "C" fn(i32, *mut core::ffi::c_void) -> i32, flags: u32, name: *const u8, dev: *mut core::ffi::c_void) -> i32;
    fn cpumask_of(cpu: u32) -> *const core::ffi::c_void;
    fn clockevents_config_and_register(dev: *mut clock_event_device, rate: usize, min: u64, max: u64);
}

const IRQF_TIMER: u32 = 0x00000080;
const HZ: usize = 100;

unsafe extern "C" fn orion_timer_init(np: *mut device_node) -> i32 {
    let mut rate: usize;
    let clk: *mut clk;
    let irq: i32;
    let mut ret: i32;

    /* timer registers are shared with watchdog timer */
    timer_base = of_iomap(np, 0);
    if timer_base.is_null() {
        return -6;
    }

    clk = of_clk_get(np, 0);
    if clk.is_null() {
        return -1;
    }

    ret = clk_prepare_enable(clk);
    if ret != 0 {
        return ret;
    }

    /* we are only interested in timer1 irq */
    irq = irq_of_parse_and_map(np, 1);
    if irq <= 0 {
        ret = -22;
        clk_disable_unprepare(clk);
        return ret;
    }

    rate = clk_get_rate(clk);

    /* setup timer0 as free-running clocksource */
    writel(!0, timer_base.add(TIMER0_VAL));
    writel(!0, timer_base.add(TIMER0_RELOAD));
    atomic_io_modify(
        timer_base.add(TIMER_CTRL),
        TIMER0_RELOAD_EN | TIMER0_EN,
        TIMER0_RELOAD_EN | TIMER0_EN,
    );

    ret = clocksource_mmio_init(
        timer_base.add(TIMER0_VAL),
        b"orion_clocksource\0".as_ptr(),
        rate,
        300,
        32,
        clocksource_mmio_readl_down,
    );
    if ret != 0 {
        clk_disable_unprepare(clk);
        return ret;
    }

    sched_clock_register(orion_read_sched_clock, 32, rate);

    /* setup timer1 as clockevent timer */
    ret = request_irq(irq, orion_clkevt_irq_handler, IRQF_TIMER, b"orion_event\0".as_ptr(), core::ptr::null_mut());
    if ret != 0 {
        clk_disable_unprepare(clk);
        return ret;
    }

    ticks_per_jiffy = ((clk_get_rate(clk) + HZ / 2) / HZ) as u32;
    orion_clkevt.cpumask = cpumask_of(0);
    orion_clkevt.irq = irq;
    clockevents_config_and_register(&raw mut orion_clkevt, rate, ORION_ONESHOT_MIN, ORION_ONESHOT_MAX);

    orion_delay_timer_init(rate);
    0
}

// TIMER_OF_DECLARE(orion_timer, "marvell,orion-timer", orion_timer_init);

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
