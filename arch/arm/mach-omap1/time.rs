/*
 * linux/arch/arm/mach-omap1/time.c
 *
 * OMAP Timers
 *
 * Copyright (C) 2004 Nokia Corporation
 * Partial timer rewrite and additional dynamic tick timer support by
 * Tony Lindgen <tony@atomide.com> and
 * Tuukka Tikkanen <tuukka.tikkanen@elektrobit.com>
 *
 * MPU timer code based on the older MPU timer code for OMAP
 * Copyright (C) 2000 RidgeRun, Inc.
 * Author: Greg Lonnon <glonnon@ridgerun.com>
 */

/* Linux headers and local headers are supplied by the surrounding translation. */

#[cfg(CONFIG_OMAP_MPU_TIMER)]
const OMAP_MPU_TIMER_BASE: usize = OMAP_MPU_TIMER1_BASE;
#[cfg(CONFIG_OMAP_MPU_TIMER)]
const OMAP_MPU_TIMER_OFFSET: usize = 0x100;

#[cfg(CONFIG_OMAP_MPU_TIMER)]
#[repr(C)]
struct omap_mpu_timer_regs_t {
    cntl: u32,    /* CNTL_TIMER, R/W */
    load_tim: u32, /* LOAD_TIM, W */
    read_tim: u32, /* READ_TIM, R */
}

#[cfg(CONFIG_OMAP_MPU_TIMER)]
#[inline]
unsafe fn omap_mpu_timer_base(n: i32) -> *mut omap_mpu_timer_regs_t {
    OMAP1_IO_ADDRESS(OMAP_MPU_TIMER_BASE + (n as usize) * OMAP_MPU_TIMER_OFFSET)
        as *mut omap_mpu_timer_regs_t
}

#[cfg(CONFIG_OMAP_MPU_TIMER)]
#[inline]
unsafe fn omap_mpu_timer_read(nr: i32) -> ::core::ffi::c_ulong {
    let timer = omap_mpu_timer_base(nr);
    readl(&(*timer).read_tim)
}

#[cfg(CONFIG_OMAP_MPU_TIMER)]
#[inline]
unsafe fn omap_mpu_set_autoreset(nr: i32) {
    let timer = omap_mpu_timer_base(nr);
    writel(readl(&(*timer).cntl) | MPU_TIMER_AR, &mut (*timer).cntl);
}

#[cfg(CONFIG_OMAP_MPU_TIMER)]
#[inline]
unsafe fn omap_mpu_remove_autoreset(nr: i32) {
    let timer = omap_mpu_timer_base(nr);
    writel(readl(&(*timer).cntl) & !MPU_TIMER_AR, &mut (*timer).cntl);
}

#[cfg(CONFIG_OMAP_MPU_TIMER)]
#[inline]
unsafe fn omap_mpu_timer_start(nr: i32, load_val: ::core::ffi::c_ulong, autoreset: i32) {
    let timer = omap_mpu_timer_base(nr);
    let mut timerflags: u32 = MPU_TIMER_CLOCK_ENABLE | MPU_TIMER_ST;

    if autoreset != 0 {
        timerflags |= MPU_TIMER_AR;
    }

    writel(MPU_TIMER_CLOCK_ENABLE, &mut (*timer).cntl);
    udelay(1);
    writel(load_val as u32, &mut (*timer).load_tim);
    udelay(1);
    writel(timerflags, &mut (*timer).cntl);
}

#[cfg(CONFIG_OMAP_MPU_TIMER)]
#[inline]
unsafe fn omap_mpu_timer_stop(nr: i32) {
    let timer = omap_mpu_timer_base(nr);
    writel(readl(&(*timer).cntl) & !MPU_TIMER_ST, &mut (*timer).cntl);
}

#[cfg(CONFIG_OMAP_MPU_TIMER)]
unsafe fn omap_mpu_set_next_event(cycles: ::core::ffi::c_ulong, _evt: *mut clock_event_device) -> i32 {
    omap_mpu_timer_start(0, cycles, 0);
    0
}

#[cfg(CONFIG_OMAP_MPU_TIMER)]
unsafe fn omap_mpu_set_oneshot(_evt: *mut clock_event_device) -> i32 {
    omap_mpu_timer_stop(0);
    omap_mpu_remove_autoreset(0);
    0
}

#[cfg(CONFIG_OMAP_MPU_TIMER)]
unsafe fn omap_mpu_set_periodic(_evt: *mut clock_event_device) -> i32 {
    omap_mpu_set_autoreset(0);
    0
}

#[cfg(CONFIG_OMAP_MPU_TIMER)]
static mut clockevent_mpu_timer1: clock_event_device = clock_event_device {
    name: "mpu_timer1" as *const u8,
    features: CLOCK_EVT_FEAT_PERIODIC | CLOCK_EVT_FEAT_ONESHOT,
    set_next_event: Some(omap_mpu_set_next_event),
    set_state_periodic: Some(omap_mpu_set_periodic),
    set_state_oneshot: Some(omap_mpu_set_oneshot),
};

#[cfg(CONFIG_OMAP_MPU_TIMER)]
unsafe fn omap_mpu_timer1_interrupt(_irq: i32, _dev_id: *mut ::core::ffi::c_void) -> irqreturn_t {
    let evt = &mut clockevent_mpu_timer1;
    ((*evt).event_handler)(evt);
    IRQ_HANDLED
}

#[cfg(CONFIG_OMAP_MPU_TIMER)]
unsafe fn omap_init_mpu_timer(rate: ::core::ffi::c_ulong) {
    if request_irq(INT_TIMER1, Some(omap_mpu_timer1_interrupt), IRQF_TIMER | IRQF_IRQPOLL,
                   "mpu_timer1", core::ptr::null_mut()) != 0 {
        pr_err!("Failed to request irq {} (mpu_timer1)\n", INT_TIMER1);
    }
    omap_mpu_timer_start(0, rate / HZ - 1, 1);
    clockevent_mpu_timer1.cpumask = cpumask_of(0);
    clockevents_config_and_register(&mut clockevent_mpu_timer1, rate, 1, -1);
}

#[cfg(CONFIG_OMAP_MPU_TIMER)]
unsafe fn omap_mpu_read_sched_clock() -> u64 {
    !(omap_mpu_timer_read(1) as u64)
}

#[cfg(CONFIG_OMAP_MPU_TIMER)]
unsafe fn omap_init_clocksource(rate: ::core::ffi::c_ulong) {
    let timer = omap_mpu_timer_base(1);
    omap_mpu_timer_start(1, !0, 1);
    sched_clock_register(Some(omap_mpu_read_sched_clock), 32, rate);
    if clocksource_mmio_init(&(*timer).read_tim, "mpu_timer2", rate, 300, 32,
                             Some(clocksource_mmio_readl_down)) != 0 {
        printk!(KERN_ERR "{}: can't register clocksource!\n", "mpu_timer2");
    }
}

#[cfg(CONFIG_OMAP_MPU_TIMER)]
unsafe fn omap_mpu_timer_init() {
    let ck_ref = clk_get(core::ptr::null(), "ck_ref");
    BUG_ON(IS_ERR(ck_ref));
    let mut rate = clk_get_rate(ck_ref);
    clk_put(ck_ref);
    rate /= 2; /* PTV = 0 */
    omap_init_mpu_timer(rate);
    omap_init_clocksource(rate);
}

#[cfg(not(CONFIG_OMAP_MPU_TIMER))]
#[inline]
unsafe fn omap_mpu_timer_init() {
    pr_err!("Bogus timer, should not happen\n");
}

pub unsafe fn omap1_timer_init() {
    omap1_clk_init();
    omap1_mux_init();
    if omap_32k_timer_init() != 0 {
        omap_mpu_timer_init();
    }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
