// SPDX-License-Identifier: GPL-2.0
/*
 * Copyright (C) 2014-2018 Nuvoton Technologies tomer.maimon@nuvoton.com
 * All rights reserved.
 *
 * Copyright 2017 Google, Inc.
 */

// Linux kernel dependencies supplied by other translation units.

/* Timers registers */
const NPCM7XX_REG_TCSR0: usize = 0x0;
const NPCM7XX_REG_TICR0: usize = 0x8;
const NPCM7XX_REG_TCSR1: usize = 0x4;
const NPCM7XX_REG_TICR1: usize = 0xc;
const NPCM7XX_REG_TDR1: usize = 0x14;
const NPCM7XX_REG_TISR: usize = 0x18;

/* Timers control */
const NPCM7XX_Tx_RESETINT: u32 = 0x1f;
const NPCM7XX_Tx_PERIOD: u32 = 1 << 27;
const NPCM7XX_Tx_INTEN: u32 = 1 << 29;
const NPCM7XX_Tx_COUNTEN: u32 = 1 << 30;
const NPCM7XX_Tx_ONESHOT: u32 = 0x0;
const NPCM7XX_Tx_OPER: u32 = (0x3 << 27);
const NPCM7XX_Tx_MIN_PRESCALE: u32 = 0x1;
const NPCM7XX_Tx_TDR_MASK_BITS: u32 = 24;
const NPCM7XX_Tx_MAX_CNT: u32 = 0xFFFFFF;
const NPCM7XX_T0_CLR_INT: u32 = 0x1;
const NPCM7XX_Tx_CLR_CSR: u32 = 0x0;

/* Timers operating mode */
const NPCM7XX_START_PERIODIC_Tx: u32 = NPCM7XX_Tx_PERIOD | NPCM7XX_Tx_COUNTEN |
    NPCM7XX_Tx_INTEN | NPCM7XX_Tx_MIN_PRESCALE;
const NPCM7XX_START_ONESHOT_Tx: u32 = NPCM7XX_Tx_ONESHOT | NPCM7XX_Tx_COUNTEN |
    NPCM7XX_Tx_INTEN | NPCM7XX_Tx_MIN_PRESCALE;
const NPCM7XX_START_Tx: u32 = NPCM7XX_Tx_COUNTEN | NPCM7XX_Tx_PERIOD |
    NPCM7XX_Tx_MIN_PRESCALE;
const NPCM7XX_DEFAULT_CSR: u32 = NPCM7XX_Tx_CLR_CSR | NPCM7XX_Tx_MIN_PRESCALE;

unsafe fn npcm7xx_timer_resume(evt: *mut clock_event_device) -> i32 {
    let to = to_timer_of(evt);
    let mut val = readl(timer_of_base(to).add(NPCM7XX_REG_TCSR0));
    val |= NPCM7XX_Tx_COUNTEN;
    writel(val, timer_of_base(to).add(NPCM7XX_REG_TCSR0));
    0
}

unsafe fn npcm7xx_timer_shutdown(evt: *mut clock_event_device) -> i32 {
    let to = to_timer_of(evt);
    let mut val = readl(timer_of_base(to).add(NPCM7XX_REG_TCSR0));
    val &= !NPCM7XX_Tx_COUNTEN;
    writel(val, timer_of_base(to).add(NPCM7XX_REG_TCSR0));
    0
}

unsafe fn npcm7xx_timer_oneshot(evt: *mut clock_event_device) -> i32 {
    let to = to_timer_of(evt);
    let mut val = readl(timer_of_base(to).add(NPCM7XX_REG_TCSR0));
    val &= !NPCM7XX_Tx_OPER;
    val |= NPCM7XX_START_ONESHOT_Tx;
    writel(val, timer_of_base(to).add(NPCM7XX_REG_TCSR0));
    0
}

unsafe fn npcm7xx_timer_periodic(evt: *mut clock_event_device) -> i32 {
    let to = to_timer_of(evt);
    writel(timer_of_period(to), timer_of_base(to).add(NPCM7XX_REG_TICR0));
    let mut val = readl(timer_of_base(to).add(NPCM7XX_REG_TCSR0));
    val &= !NPCM7XX_Tx_OPER;
    val |= NPCM7XX_START_PERIODIC_Tx;
    writel(val, timer_of_base(to).add(NPCM7XX_REG_TCSR0));
    0
}

unsafe fn npcm7xx_clockevent_set_next_event(evt: c_ulong, clk: *mut clock_event_device) -> i32 {
    let to = to_timer_of(clk);
    writel(evt as u32, timer_of_base(to).add(NPCM7XX_REG_TICR0));
    let mut val = readl(timer_of_base(to).add(NPCM7XX_REG_TCSR0));
    val |= NPCM7XX_START_Tx;
    writel(val, timer_of_base(to).add(NPCM7XX_REG_TCSR0));
    0
}

unsafe fn npcm7xx_timer0_interrupt(_irq: i32, dev_id: *mut c_void) -> irqreturn_t {
    let evt = dev_id as *mut clock_event_device;
    let to = to_timer_of(evt);
    writel(NPCM7XX_T0_CLR_INT, timer_of_base(to).add(NPCM7XX_REG_TISR));
    ((*evt).event_handler)(evt);
    IRQ_HANDLED
}

static mut npcm7xx_to: timer_of = timer_of {
    flags: TIMER_OF_IRQ | TIMER_OF_BASE | TIMER_OF_CLOCK,
    clkevt: clock_event_device {
        name: b"npcm7xx-timer0\0".as_ptr() as *const c_char,
        features: CLOCK_EVT_FEAT_PERIODIC | CLOCK_EVT_FEAT_ONESHOT,
        set_next_event: Some(npcm7xx_clockevent_set_next_event),
        set_state_shutdown: Some(npcm7xx_timer_shutdown),
        set_state_periodic: Some(npcm7xx_timer_periodic),
        set_state_oneshot: Some(npcm7xx_timer_oneshot),
        tick_resume: Some(npcm7xx_timer_resume),
        rating: 300,
        ..clock_event_device::default()
    },
    of_irq: timer_of_irq_data {
        handler: Some(npcm7xx_timer0_interrupt),
        flags: IRQF_TIMER | IRQF_IRQPOLL,
    },
    ..timer_of::default()
};

unsafe fn npcm7xx_clockevents_init() {
    writel(NPCM7XX_DEFAULT_CSR, timer_of_base(&mut npcm7xx_to).add(NPCM7XX_REG_TCSR0));
    writel(NPCM7XX_Tx_RESETINT, timer_of_base(&mut npcm7xx_to).add(NPCM7XX_REG_TISR));
    npcm7xx_to.clkevt.cpumask = cpumask_of(0);
    clockevents_config_and_register(&mut npcm7xx_to.clkevt, timer_of_rate(&mut npcm7xx_to), 0x1, NPCM7XX_Tx_MAX_CNT);
}

unsafe fn npcm7xx_clocksource_init() {
    writel(NPCM7XX_DEFAULT_CSR, timer_of_base(&mut npcm7xx_to).add(NPCM7XX_REG_TCSR1));
    writel(NPCM7XX_Tx_MAX_CNT, timer_of_base(&mut npcm7xx_to).add(NPCM7XX_REG_TICR1));
    let mut val = readl(timer_of_base(&mut npcm7xx_to).add(NPCM7XX_REG_TCSR1));
    val |= NPCM7XX_START_Tx;
    writel(val, timer_of_base(&mut npcm7xx_to).add(NPCM7XX_REG_TCSR1));
    clocksource_mmio_init(timer_of_base(&mut npcm7xx_to).add(NPCM7XX_REG_TDR1), b"npcm7xx-timer1\0".as_ptr() as *const c_char, timer_of_rate(&mut npcm7xx_to), 200, NPCM7XX_Tx_TDR_MASK_BITS as c_uint, clocksource_mmio_readl_down);
}

unsafe fn npcm7xx_timer_init(np: *mut device_node) -> i32 {
    let mut ret = timer_of_init(np, &mut npcm7xx_to);
    if ret != 0 { return ret; }
    npcm7xx_to.of_clk.rate /= NPCM7XX_Tx_MIN_PRESCALE + 1;
    let clk = of_clk_get(np, 1);
    if !clk.is_null() {
        if !IS_ERR(clk) { clk_prepare_enable(clk); }
        else { pr_warn(b"%pOF: Failed to get clock for timer1: %pe\0", np, clk); }
    }
    npcm7xx_clocksource_init();
    npcm7xx_clockevents_init();
    pr_info(b"Enabling NPCM7xx clocksource timer base: %px, IRQ: %d \0", timer_of_base(&mut npcm7xx_to), timer_of_irq(&mut npcm7xx_to));
    ret = 0;
    ret
}

TIMER_OF_DECLARE!(wpcm450, b"nuvoton,wpcm450-timer\0", npcm7xx_timer_init);
TIMER_OF_DECLARE!(npcm7xx, b"nuvoton,npcm750-timer\0", npcm7xx_timer_init);

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
