// SPDX-License-Identifier: GPL-2.0+
//
// Copyright 2016 Freescale Semiconductor, Inc.
// Copyright 2017 NXP

// Dependencies supplied by the surrounding kernel translation.

const TPM_PARAM: usize = 0x4;
const TPM_PARAM_WIDTH_SHIFT: u32 = 16;
const TPM_PARAM_WIDTH_MASK: u32 = 0xff << 16;
const TPM_SC: usize = 0x10;
const TPM_SC_CMOD_INC_PER_CNT: u32 = 0x1 << 3;
const TPM_SC_CMOD_DIV_DEFAULT: u32 = 0x3;
const TPM_SC_CMOD_DIV_MAX: u32 = 0x7;
const TPM_SC_TOF_MASK: u32 = 0x1 << 7;
const TPM_CNT: usize = 0x14;
const TPM_MOD: usize = 0x18;
const TPM_STATUS: usize = 0x1c;
const TPM_STATUS_CH0F: u32 = 1 << 0;
const TPM_C0SC: usize = 0x20;
const TPM_C0SC_CHIE: u32 = 1 << 6;
const TPM_C0SC_MODE_SHIFT: u32 = 2;
const TPM_C0SC_MODE_MASK: u32 = 0x3c;
const TPM_C0SC_MODE_SW_COMPARE: u32 = 0x4;
const TPM_C0SC_CHF_MASK: u32 = 0x1 << 7;
const TPM_C0V: usize = 0x24;

static mut counter_width: i32 = 0;
static mut timer_base: *mut u8 = core::ptr::null_mut();

#[inline]
unsafe fn tpm_timer_disable() {
    let mut val: u32;

    /* channel disable */
    val = readl(timer_base.add(TPM_C0SC));
    val &= !(TPM_C0SC_MODE_MASK | TPM_C0SC_CHIE);
    writel(val, timer_base.add(TPM_C0SC));
}

#[inline]
unsafe fn tpm_timer_enable() {
    let mut val: u32;

    /* channel enabled in sw compare mode */
    val = readl(timer_base.add(TPM_C0SC));
    val |= (TPM_C0SC_MODE_SW_COMPARE << TPM_C0SC_MODE_SHIFT) | TPM_C0SC_CHIE;
    writel(val, timer_base.add(TPM_C0SC));
}

#[inline]
unsafe fn tpm_irq_acknowledge() {
    writel(TPM_STATUS_CH0F, timer_base.add(TPM_STATUS));
}

#[inline]
unsafe fn tpm_read_counter() -> u32 {
    readl(timer_base.add(TPM_CNT))
}

#[cfg(CONFIG_ARM)]
static mut tpm_delay_timer: delay_timer = delay_timer { read_current_timer: None, freq: 0 };

#[cfg(CONFIG_ARM)]
unsafe fn tpm_read_current_timer() -> u32 {
    tpm_read_counter()
}

#[cfg(CONFIG_ARM)]
unsafe fn tpm_read_sched_clock() -> u64 {
    tpm_read_counter() as u64
}

unsafe fn tpm_set_next_event(delta: u32, _evt: *mut clock_event_device) -> i32 {
    let prev: u32 = tpm_read_counter();
    let next: u32 = prev.wrapping_add(delta);
    writel(next, timer_base.add(TPM_C0V));
    let now: u32 = tpm_read_counter();

    /*
     * Need to wait CNT increase at least 1 cycle to make sure
     * the C0V has been updated into HW.
     */
    if (next & 0xffffffff) != readl(timer_base.add(TPM_C0V)) {
        while now == tpm_read_counter() {}
    }

    /*
     * NOTE: We observed in a very small probability, the bus fabric
     * contention between GPU and A7 may results a few cycles delay
     * of writing CNT registers which may cause the min_delta event got
     * missed, so we need add a ETIME check here in case it happened.
     */
    if now.wrapping_sub(prev) >= delta { -ETIME } else { 0 }
}

unsafe fn tpm_set_state_oneshot(_evt: *mut clock_event_device) -> i32 {
    tpm_timer_enable();
    0
}

unsafe fn tpm_set_state_shutdown(_evt: *mut clock_event_device) -> i32 {
    tpm_timer_disable();
    0
}

unsafe fn tpm_timer_interrupt(irq: i32, dev_id: *mut core::ffi::c_void) -> irqreturn_t {
    let evt: *mut clock_event_device = dev_id as *mut clock_event_device;

    tpm_irq_acknowledge();
    ((*evt).event_handler)(evt);
    IRQ_HANDLED
}

static mut to_tpm: timer_of = timer_of {
    flags: TIMER_OF_IRQ | TIMER_OF_BASE | TIMER_OF_CLOCK,
    clkevt: clock_event_device {
        name: b"i.MX TPM Timer\\0".as_ptr() as *const i8,
        rating: 200,
        features: CLOCK_EVT_FEAT_ONESHOT | CLOCK_EVT_FEAT_DYNIRQ,
        set_state_shutdown: Some(tpm_set_state_shutdown),
        set_state_oneshot: Some(tpm_set_state_oneshot),
        set_next_event: Some(tpm_set_next_event),
        cpumask: unsafe { &raw const cpu_possible_mask },
    },
    of_irq: timer_of_irq {
        handler: Some(tpm_timer_interrupt),
        flags: IRQF_TIMER,
    },
    of_clk: timer_of_clk { name: b"per\\0".as_ptr() as *const i8 },
};

unsafe fn tpm_clocksource_init() -> i32 {
    #[cfg(CONFIG_ARM)]
    {
        tpm_delay_timer.read_current_timer = Some(tpm_read_current_timer);
        tpm_delay_timer.freq = timer_of_rate(&raw const to_tpm) >> 3;
        register_current_timer_delay(&raw mut tpm_delay_timer);

        sched_clock_register(Some(tpm_read_sched_clock), counter_width,
                             timer_of_rate(&raw const to_tpm) >> 3);
    }

    clocksource_mmio_init(timer_base.add(TPM_CNT),
                          b"imx-tpm\\0".as_ptr() as *const i8,
                          timer_of_rate(&raw const to_tpm) >> 3,
                          to_tpm.clkevt.rating,
                          counter_width,
                          Some(clocksource_mmio_readl_up))
}

unsafe fn tpm_clockevent_init() {
    clockevents_config_and_register(&raw mut to_tpm.clkevt,
                                    timer_of_rate(&raw const to_tpm) >> 3,
                                    300,
                                    GENMASK(counter_width - 1, 1));
}

unsafe fn tpm_timer_init(np: *mut device_node) -> i32 {
    let ipg: *mut clk = of_clk_get_by_name(np, b"ipg\\0".as_ptr() as *const i8);
    let mut ret: i32;

    if IS_ERR(ipg) {
        pr_err(b"tpm: failed to get ipg clk\\n\\0".as_ptr() as *const i8);
        return -ENODEV;
    }
    /* enable clk before accessing registers */
    ret = clk_prepare_enable(ipg);
    if ret != 0 {
        pr_err(b"tpm: ipg clock enable failed (%d)\\n\\0".as_ptr() as *const i8, ret);
        clk_put(ipg);
        return ret;
    }

    ret = timer_of_init(np, &raw mut to_tpm);
    if ret != 0 { return ret; }

    timer_base = timer_of_base(&raw const to_tpm);
    counter_width = ((readl(timer_base.add(TPM_PARAM)) & TPM_PARAM_WIDTH_MASK)
        >> TPM_PARAM_WIDTH_SHIFT) as i32;
    /* use rating 200 for 32-bit counter and 150 for 16-bit counter */
    to_tpm.clkevt.rating = if counter_width == 0x20 { 200 } else { 150 };

    /*
     * Initialize tpm module to a known state
     * 1) Counter disabled
     * 2) TPM counter operates in up counting mode
     * 3) Timer Overflow Interrupt disabled
     * 4) Channel0 disabled
     * 5) DMA transfers disabled
     */
    /* make sure counter is disabled */
    writel(0, timer_base.add(TPM_SC));
    /* TOF is W1C */
    writel(TPM_SC_TOF_MASK, timer_base.add(TPM_SC));
    writel(0, timer_base.add(TPM_CNT));
    /* CHF is W1C */
    writel(TPM_C0SC_CHF_MASK, timer_base.add(TPM_C0SC));

    /*
     * increase per cnt,
     * div 8 for 32-bit counter and div 128 for 16-bit counter
     */
    writel(TPM_SC_CMOD_INC_PER_CNT |
        if counter_width == 0x20 { TPM_SC_CMOD_DIV_DEFAULT } else { TPM_SC_CMOD_DIV_MAX },
        timer_base.add(TPM_SC));

    /* set MOD register to maximum for free running mode */
    writel(GENMASK(counter_width - 1, 0), timer_base.add(TPM_MOD));

    tpm_clockevent_init();
    tpm_clocksource_init()
}

TIMER_OF_DECLARE!(imx7ulp, b"fsl,imx7ulp-tpm\\0", tpm_timer_init);

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
