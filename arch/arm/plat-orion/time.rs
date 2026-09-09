/*
 * arch/arm/plat-orion/time.c
 *
 * Marvell Orion SoC timer handling.
 *
 * This file is licensed under the terms of the GNU General Public
 * License version 2.  This program is licensed "as is" without any
 * warranty of any kind, whether express or implied.
 *
 * Timer 0 is used as free-running clocksource, while timer 1 is
 * used as clock_event_device.
 */

// Linux and platform dependencies are supplied by the surrounding kernel.

/*
 * MBus bridge block registers.
 */
const BRIDGE_CAUSE_OFF: usize = 0x0110;
const BRIDGE_MASK_OFF: usize = 0x0114;
const BRIDGE_INT_TIMER0: u32 = 0x0002;
const BRIDGE_INT_TIMER1: u32 = 0x0004;

/*
 * Timer block registers.
 */
const TIMER_CTRL_OFF: usize = 0x0000;
const TIMER0_EN: u32 = 0x0001;
const TIMER0_RELOAD_EN: u32 = 0x0002;
const TIMER1_EN: u32 = 0x0004;
const TIMER1_RELOAD_EN: u32 = 0x0008;
const TIMER0_RELOAD_OFF: usize = 0x0010;
const TIMER0_VAL_OFF: usize = 0x0014;
const TIMER1_RELOAD_OFF: usize = 0x0018;
const TIMER1_VAL_OFF: usize = 0x001c;

/*
 * SoC-specific data.
 */
static mut bridge_base: *mut core::ffi::c_void = core::ptr::null_mut();
static mut bridge_timer1_clr_mask: u32 = 0;
static mut timer_base: *mut core::ffi::c_void = core::ptr::null_mut();

/*
 * Number of timer ticks per jiffy.
 */
static mut ticks_per_jiffy: u32 = 0;

/*
 * Orion's sched_clock implementation. It has a resolution of
 * at least 7.5ns (133MHz TCLK).
 */
unsafe fn orion_read_sched_clock() -> u64 {
    (!readl(timer_base.byte_add(TIMER0_VAL_OFF))) as u64
}

/*
 * Clockevent handling.
 */
unsafe fn orion_clkevt_next_event(delta: usize, _dev: *mut clock_event_device) -> i32 {
    let mut flags: usize = 0;
    let mut u: u32;

    if delta == 0 {
        return -ETIME;
    }

    local_irq_save(&mut flags);

    /* Clear and enable clockevent timer interrupt. */
    writel(bridge_timer1_clr_mask, bridge_base.byte_add(BRIDGE_CAUSE_OFF));

    u = readl(bridge_base.byte_add(BRIDGE_MASK_OFF));
    u |= BRIDGE_INT_TIMER1;
    writel(u, bridge_base.byte_add(BRIDGE_MASK_OFF));

    /* Setup new clockevent timer value. */
    writel(delta as u32, timer_base.byte_add(TIMER1_VAL_OFF));

    /* Enable the timer. */
    u = readl(timer_base.byte_add(TIMER_CTRL_OFF));
    u = (u & !TIMER1_RELOAD_EN) | TIMER1_EN;
    writel(u, timer_base.byte_add(TIMER_CTRL_OFF));

    local_irq_restore(flags);

    0
}

unsafe fn orion_clkevt_shutdown(_evt: *mut clock_event_device) -> i32 {
    let mut flags: usize = 0;
    let mut u: u32;

    local_irq_save(&mut flags);

    /* Disable timer */
    u = readl(timer_base.byte_add(TIMER_CTRL_OFF));
    writel(u & !TIMER1_EN, timer_base.byte_add(TIMER_CTRL_OFF));

    /* Disable timer interrupt */
    u = readl(bridge_base.byte_add(BRIDGE_MASK_OFF));
    writel(u & !BRIDGE_INT_TIMER1, bridge_base.byte_add(BRIDGE_MASK_OFF));

    /* ACK pending timer interrupt */
    writel(bridge_timer1_clr_mask, bridge_base.byte_add(BRIDGE_CAUSE_OFF));

    local_irq_restore(flags);

    0
}

unsafe fn orion_clkevt_set_periodic(_evt: *mut clock_event_device) -> i32 {
    let mut flags: usize = 0;
    let mut u: u32;

    local_irq_save(&mut flags);

    /* Setup timer to fire at 1/HZ intervals */
    writel(ticks_per_jiffy - 1, timer_base.byte_add(TIMER1_RELOAD_OFF));
    writel(ticks_per_jiffy - 1, timer_base.byte_add(TIMER1_VAL_OFF));

    /* Enable timer interrupt */
    u = readl(bridge_base.byte_add(BRIDGE_MASK_OFF));
    writel(u | BRIDGE_INT_TIMER1, bridge_base.byte_add(BRIDGE_MASK_OFF));

    /* Enable timer */
    u = readl(timer_base.byte_add(TIMER_CTRL_OFF));
    writel(u | TIMER1_EN | TIMER1_RELOAD_EN, timer_base.byte_add(TIMER_CTRL_OFF));

    local_irq_restore(flags);

    0
}

static mut orion_clkevt: clock_event_device = clock_event_device {
    name: b"orion_tick\0".as_ptr() as *const i8,
    features: CLOCK_EVT_FEAT_ONESHOT | CLOCK_EVT_FEAT_PERIODIC,
    rating: 300,
    set_next_event: Some(orion_clkevt_next_event),
    set_state_shutdown: Some(orion_clkevt_shutdown),
    set_state_periodic: Some(orion_clkevt_set_periodic),
    set_state_oneshot: Some(orion_clkevt_shutdown),
    tick_resume: Some(orion_clkevt_shutdown),
    ..clock_event_device::default()
};

unsafe fn orion_timer_interrupt(_irq: i32, _dev_id: *mut core::ffi::c_void) -> irqreturn_t {
    /* ACK timer interrupt and call event handler. */
    writel(bridge_timer1_clr_mask, bridge_base.byte_add(BRIDGE_CAUSE_OFF));
    ((*orion_clkevt).event_handler)(&mut orion_clkevt);
    IRQ_HANDLED
}

pub unsafe fn orion_time_set_base(_timer_base: *mut core::ffi::c_void) {
    timer_base = _timer_base;
}

unsafe fn orion_delay_timer_read() -> usize {
    (!readl(timer_base.byte_add(TIMER0_VAL_OFF))) as usize
}

static mut orion_delay_timer: delay_timer = delay_timer {
    read_current_timer: Some(orion_delay_timer_read),
    ..delay_timer::default()
};

pub unsafe fn orion_time_init(
    _bridge_base: *mut core::ffi::c_void,
    _bridge_timer1_clr_mask: u32,
    irq: u32,
    tclk: u32,
) {
    let mut u: u32;

    /* Set SoC-specific data. */
    bridge_base = _bridge_base;
    bridge_timer1_clr_mask = _bridge_timer1_clr_mask;

    ticks_per_jiffy = (tclk + HZ / 2) / HZ;

    orion_delay_timer.freq = tclk;
    register_current_timer_delay(&mut orion_delay_timer);

    /* Set scale and timer for sched_clock. */
    sched_clock_register(Some(orion_read_sched_clock), 32, tclk);

    /* Setup free-running clocksource timer (interrupts disabled). */
    writel(0xffffffff, timer_base.byte_add(TIMER0_VAL_OFF));
    writel(0xffffffff, timer_base.byte_add(TIMER0_RELOAD_OFF));
    u = readl(bridge_base.byte_add(BRIDGE_MASK_OFF));
    writel(u & !BRIDGE_INT_TIMER0, bridge_base.byte_add(BRIDGE_MASK_OFF));
    u = readl(timer_base.byte_add(TIMER_CTRL_OFF));
    writel(u | TIMER0_EN | TIMER0_RELOAD_EN, timer_base.byte_add(TIMER_CTRL_OFF));
    clocksource_mmio_init(
        timer_base.byte_add(TIMER0_VAL_OFF), b"orion_clocksource\0".as_ptr() as *const i8,
        tclk, 300, 32, Some(clocksource_mmio_readl_down),
    );

    /* Setup clockevent timer (interrupt-driven). */
    if request_irq(irq, Some(orion_timer_interrupt), IRQF_TIMER, b"orion_tick\0".as_ptr() as *const i8, core::ptr::null_mut()) != 0 {
        pr_err!("Failed to request irq {} (orion_tick)\n", irq);
    }
    orion_clkevt.cpumask = cpumask_of(0);
    clockevents_config_and_register(&mut orion_clkevt, tclk, 1, 0xfffffffe);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
