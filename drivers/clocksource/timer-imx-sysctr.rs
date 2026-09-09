// SPDX-License-Identifier: GPL-2.0+
//
// Copyright 2017-2019 NXP

// Dependencies supplied by the surrounding kernel translation.

const CMP_OFFSET: u32 = 0x10000;
const RD_OFFSET: u32 = 0x20000;

const CNTCV_LO: u32 = 0x8;
const CNTCV_HI: u32 = 0xc;
const CMPCV_LO: u32 = CMP_OFFSET + 0x20;
const CMPCV_HI: u32 = CMP_OFFSET + 0x24;
const CMPCR: u32 = CMP_OFFSET + 0x2c;
const CNTCV_LO_IMX95: u32 = RD_OFFSET + 0x8;
const CNTCV_HI_IMX95: u32 = RD_OFFSET + 0xc;

const SYS_CTR_EN: u32 = 0x1;
const SYS_CTR_IRQ_MASK: u32 = 0x2;

const SYS_CTR_CLK_DIV: u32 = 0x3;

#[repr(C)]
struct sysctr_private {
    cmpcr: u32,
    lo_off: u32,
    hi_off: u32,
}

unsafe fn sysctr_timer_enable(evt: *mut clock_event_device, enable: bool) {
    let to = to_timer_of(evt);
    let priv_ = (*to).private_data as *mut sysctr_private;
    let base = timer_of_base(to);

    writel(
        if enable { (*priv_).cmpcr | SYS_CTR_EN } else { (*priv_).cmpcr },
        base.add(CMPCR as usize),
    );
}

unsafe fn sysctr_irq_acknowledge(evt: *mut clock_event_device) {
    /*
     * clear the enable bit(EN =0) will clear
     * the status bit(ISTAT = 0), then the interrupt
     * signal will be negated(acknowledged).
     */
    sysctr_timer_enable(evt, false);
}

#[inline]
unsafe fn sysctr_read_counter(evt: *mut clock_event_device) -> u64 {
    let to = to_timer_of(evt);
    let priv_ = (*to).private_data as *mut sysctr_private;
    let base = timer_of_base(to);
    let (mut cnt_hi, mut tmp_hi, mut cnt_lo): (u32, u32, u32);

    loop {
        cnt_hi = readl_relaxed(base.add((*priv_).hi_off as usize));
        cnt_lo = readl_relaxed(base.add((*priv_).lo_off as usize));
        tmp_hi = readl_relaxed(base.add((*priv_).hi_off as usize));
        if tmp_hi == cnt_hi {
            break;
        }
    }

    ((cnt_hi as u64) << 32) | cnt_lo as u64
}

unsafe fn sysctr_set_next_event(delta: c_ulong, evt: *mut clock_event_device) -> c_int {
    let to = to_timer_of(evt);
    let base = timer_of_base(to);
    let cmp_hi: u32;
    let cmp_lo: u32;
    let mut next: u64;

    sysctr_timer_enable(evt, false);

    next = sysctr_read_counter(evt);
    next = next.wrapping_add(delta as u64);

    cmp_hi = ((next >> 32) & 0x00fffff) as u32;
    cmp_lo = (next & 0xffffffff) as u32;

    writel_relaxed(cmp_hi, base.add(CMPCV_HI as usize));
    writel_relaxed(cmp_lo, base.add(CMPCV_LO as usize));

    sysctr_timer_enable(evt, true);

    0
}

unsafe fn sysctr_set_state_oneshot(_evt: *mut clock_event_device) -> c_int {
    0
}

unsafe fn sysctr_set_state_shutdown(evt: *mut clock_event_device) -> c_int {
    sysctr_timer_enable(evt, false);
    0
}

unsafe fn sysctr_timer_interrupt(irq: c_int, dev_id: *mut c_void) -> irqreturn_t {
    let evt = dev_id as *mut clock_event_device;

    sysctr_irq_acknowledge(evt);
    ((*evt).event_handler)(evt);

    IRQ_HANDLED
}

static mut to_sysctr: timer_of = timer_of {
    flags: TIMER_OF_IRQ | TIMER_OF_CLOCK | TIMER_OF_BASE,
    clkevt: clock_event_device {
        name: "i.MX system counter timer",
        features: CLOCK_EVT_FEAT_ONESHOT | CLOCK_EVT_FEAT_DYNIRQ,
        set_state_oneshot: Some(sysctr_set_state_oneshot),
        set_next_event: Some(sysctr_set_next_event),
        set_state_shutdown: Some(sysctr_set_state_shutdown),
        rating: 200,
    },
    of_irq: timer_of_irq {
        handler: Some(sysctr_timer_interrupt),
        flags: IRQF_TIMER,
    },
    of_clk: timer_of_clk { name: "per" },
};

unsafe fn __sysctr_timer_init(np: *mut device_node) -> c_int {
    let priv_: *mut sysctr_private;
    let base: *mut u8;
    let ret: c_int;

    priv_ = kzalloc_obj::<sysctr_private>();
    if priv_.is_null() {
        return -ENOMEM;
    }

    ret = timer_of_init(np, &raw mut to_sysctr);
    if ret != 0 {
        kfree(priv_ as *mut c_void);
        return ret;
    }

    if !of_property_read_bool(np, "nxp,no-divider") {
        /* system counter clock is divided by 3 internally */
        (*to_sysctr.of_clk).rate /= SYS_CTR_CLK_DIV;
    }

    to_sysctr.clkevt.cpumask = cpu_possible_mask;
    to_sysctr.private_data = priv_ as *mut c_void;

    base = timer_of_base(&raw mut to_sysctr);
    (*priv_).cmpcr = readl(base.add(CMPCR as usize)) & !SYS_CTR_EN;

    0
}

unsafe fn sysctr_timer_init(np: *mut device_node) -> c_int {
    let priv_: *mut sysctr_private;
    let ret = __sysctr_timer_init(np);
    if ret != 0 {
        return ret;
    }

    priv_ = to_sysctr.private_data as *mut sysctr_private;
    (*priv_).lo_off = CNTCV_LO;
    (*priv_).hi_off = CNTCV_HI;

    clockevents_config_and_register(&raw mut to_sysctr.clkevt,
                                    timer_of_rate(&raw mut to_sysctr),
                                    0xff, 0x7fffffff);
    0
}

unsafe fn sysctr_timer_imx95_init(np: *mut device_node) -> c_int {
    let priv_: *mut sysctr_private;
    let ret = __sysctr_timer_init(np);
    if ret != 0 {
        return ret;
    }

    priv_ = to_sysctr.private_data as *mut sysctr_private;
    (*priv_).lo_off = CNTCV_LO_IMX95;
    (*priv_).hi_off = CNTCV_HI_IMX95;

    clockevents_config_and_register(&raw mut to_sysctr.clkevt,
                                    timer_of_rate(&raw mut to_sysctr),
                                    0xff, 0x7fffffff);
    0
}

TIMER_OF_DECLARE!(sysctr_timer, "nxp,sysctr-timer", sysctr_timer_init);
TIMER_OF_DECLARE!(sysctr_timer_imx95, "nxp,imx95-sysctr-timer", sysctr_timer_imx95_init);

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
