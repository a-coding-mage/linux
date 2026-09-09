// SPDX-License-Identifier: GPL-2.0-only
/*
 * TI DaVinci clocksource driver
 *
 * Copyright (C) 2019 Texas Instruments
 * Author: Bartosz Golaszewski <bgolaszewski@baylibre.com>
 * (with tiny parts adopted from code by Kevin Hilman <khilman@baylibre.com>)
 */

// Dependencies supplied by the surrounding kernel translation.

const DAVINCI_TIMER_REG_TIM12: u32 = 0x10;
const DAVINCI_TIMER_REG_TIM34: u32 = 0x14;
const DAVINCI_TIMER_REG_PRD12: u32 = 0x18;
const DAVINCI_TIMER_REG_PRD34: u32 = 0x1c;
const DAVINCI_TIMER_REG_TCR: u32 = 0x20;
const DAVINCI_TIMER_REG_TGCR: u32 = 0x24;

const DAVINCI_TIMER_TIMMODE_MASK: u32 = 0x0c;
const DAVINCI_TIMER_RESET_MASK: u32 = 0x03;
const DAVINCI_TIMER_TIMMODE_32BIT_UNCHAINED: u32 = 1 << 2;
const DAVINCI_TIMER_UNRESET: u32 = 0x03;

const DAVINCI_TIMER_ENAMODE_MASK: u32 = 0x03;
const DAVINCI_TIMER_ENAMODE_DISABLED: u32 = 0x00;
const DAVINCI_TIMER_ENAMODE_ONESHOT: u32 = 1 << 0;
const DAVINCI_TIMER_ENAMODE_PERIODIC: u32 = 1 << 1;

const DAVINCI_TIMER_ENAMODE_SHIFT_TIM12: u32 = 6;
const DAVINCI_TIMER_ENAMODE_SHIFT_TIM34: u32 = 22;

const DAVINCI_TIMER_MIN_DELTA: u32 = 0x01;
const DAVINCI_TIMER_MAX_DELTA: u32 = 0xfffffffe;
const DAVINCI_TIMER_CLKSRC_BITS: u32 = 32;
const DAVINCI_TIMER_TGCR_DEFAULT: u32 =
    DAVINCI_TIMER_TIMMODE_32BIT_UNCHAINED | DAVINCI_TIMER_UNRESET;

#[repr(C)]
struct DavinciClockevent {
    dev: clock_event_device,
    base: *mut core::ffi::c_void,
    cmp_off: u32,
}

#[repr(C)]
struct DavinciClocksource {
    dev: clocksource,
    base: *mut core::ffi::c_void,
    tim_off: u32,
}

static mut DAVINCI_CLOCKSOURCE: DavinciClocksource = DavinciClocksource {
    dev: unsafe { core::mem::zeroed() },
    base: core::ptr::null_mut(),
    tim_off: 0,
};

unsafe fn to_davinci_clockevent(clockevent: *mut clock_event_device) -> *mut DavinciClockevent {
    (clockevent as *mut u8).sub(core::mem::offset_of!(DavinciClockevent, dev))
        as *mut DavinciClockevent
}

unsafe fn davinci_clockevent_read(clockevent: *mut DavinciClockevent, reg: u32) -> u32 {
    readl_relaxed((*clockevent).base.cast::<u8>().add(reg as usize).cast())
}

unsafe fn davinci_clockevent_write(clockevent: *mut DavinciClockevent, reg: u32, val: u32) {
    writel_relaxed(val, (*clockevent).base.cast::<u8>().add(reg as usize).cast());
}

unsafe fn davinci_tim12_shutdown(base: *mut core::ffi::c_void) {
    let mut tcr = DAVINCI_TIMER_ENAMODE_DISABLED << DAVINCI_TIMER_ENAMODE_SHIFT_TIM12;
    /* This function is only ever called if we're using both timer halves. */
    tcr |= DAVINCI_TIMER_ENAMODE_PERIODIC << DAVINCI_TIMER_ENAMODE_SHIFT_TIM34;
    writel_relaxed(tcr, base.cast::<u8>().add(DAVINCI_TIMER_REG_TCR as usize).cast());
}

unsafe fn davinci_tim12_set_oneshot(base: *mut core::ffi::c_void) {
    let mut tcr = DAVINCI_TIMER_ENAMODE_ONESHOT << DAVINCI_TIMER_ENAMODE_SHIFT_TIM12;
    /* Same as above. */
    tcr |= DAVINCI_TIMER_ENAMODE_PERIODIC << DAVINCI_TIMER_ENAMODE_SHIFT_TIM34;
    writel_relaxed(tcr, base.cast::<u8>().add(DAVINCI_TIMER_REG_TCR as usize).cast());
}

unsafe extern "C" fn davinci_clockevent_shutdown(dev: *mut clock_event_device) -> i32 {
    let clockevent = to_davinci_clockevent(dev);
    davinci_tim12_shutdown((*clockevent).base);
    0
}

unsafe extern "C" fn davinci_clockevent_set_oneshot(dev: *mut clock_event_device) -> i32 {
    let clockevent = to_davinci_clockevent(dev);
    davinci_clockevent_write(clockevent, DAVINCI_TIMER_REG_TIM12, 0);
    davinci_tim12_set_oneshot((*clockevent).base);
    0
}

unsafe extern "C" fn davinci_clockevent_set_next_event_std(
    cycles: core::ffi::c_ulong,
    dev: *mut clock_event_device,
) -> i32 {
    let clockevent = to_davinci_clockevent(dev);
    davinci_clockevent_shutdown(dev);
    davinci_clockevent_write(clockevent, DAVINCI_TIMER_REG_TIM12, 0);
    davinci_clockevent_write(clockevent, DAVINCI_TIMER_REG_PRD12, cycles as u32);
    davinci_clockevent_set_oneshot(dev);
    0
}

unsafe extern "C" fn davinci_clockevent_set_next_event_cmp(
    cycles: core::ffi::c_ulong,
    dev: *mut clock_event_device,
) -> i32 {
    let clockevent = to_davinci_clockevent(dev);
    let curr_time = davinci_clockevent_read(clockevent, DAVINCI_TIMER_REG_TIM12);
    davinci_clockevent_write(clockevent, (*clockevent).cmp_off, curr_time.wrapping_add(cycles as u32));
    0
}

unsafe extern "C" fn davinci_timer_irq_timer(_irq: i32, data: *mut core::ffi::c_void) -> irqreturn_t {
    let clockevent = data as *mut DavinciClockevent;
    if !clockevent_state_oneshot(&mut (*clockevent).dev) {
        davinci_tim12_shutdown((*clockevent).base);
    }
    ((*clockevent).dev.event_handler)(&mut (*clockevent).dev);
    IRQ_HANDLED
}

unsafe extern "C" fn davinci_timer_read_sched_clock() -> u64 {
    readl_relaxed(
        DAVINCI_CLOCKSOURCE.base.cast::<u8>().add(DAVINCI_CLOCKSOURCE.tim_off as usize).cast(),
    ) as u64
}

unsafe extern "C" fn davinci_clocksource_read(_dev: *mut clocksource) -> u64 {
    davinci_timer_read_sched_clock()
}

unsafe fn davinci_clocksource_init_tim34(base: *mut core::ffi::c_void) {
    let mut tcr = DAVINCI_TIMER_ENAMODE_PERIODIC << DAVINCI_TIMER_ENAMODE_SHIFT_TIM34;
    tcr |= DAVINCI_TIMER_ENAMODE_ONESHOT << DAVINCI_TIMER_ENAMODE_SHIFT_TIM12;
    writel_relaxed(0, base.cast::<u8>().add(DAVINCI_TIMER_REG_TIM34 as usize).cast());
    writel_relaxed(u32::MAX, base.cast::<u8>().add(DAVINCI_TIMER_REG_PRD34 as usize).cast());
    writel_relaxed(tcr, base.cast::<u8>().add(DAVINCI_TIMER_REG_TCR as usize).cast());
}

unsafe fn davinci_clocksource_init_tim12(base: *mut core::ffi::c_void) {
    let tcr = DAVINCI_TIMER_ENAMODE_PERIODIC << DAVINCI_TIMER_ENAMODE_SHIFT_TIM12;
    writel_relaxed(0, base.cast::<u8>().add(DAVINCI_TIMER_REG_TIM12 as usize).cast());
    writel_relaxed(u32::MAX, base.cast::<u8>().add(DAVINCI_TIMER_REG_PRD12 as usize).cast());
    writel_relaxed(tcr, base.cast::<u8>().add(DAVINCI_TIMER_REG_TCR as usize).cast());
}

unsafe fn davinci_timer_init(base: *mut core::ffi::c_void) {
    writel_relaxed(0, base.cast::<u8>().add(DAVINCI_TIMER_REG_TCR as usize).cast());
    writel_relaxed(DAVINCI_TIMER_TGCR_DEFAULT, base.cast::<u8>().add(DAVINCI_TIMER_REG_TGCR as usize).cast());
    writel_relaxed(0, base.cast::<u8>().add(DAVINCI_TIMER_REG_TIM12 as usize).cast());
    writel_relaxed(0, base.cast::<u8>().add(DAVINCI_TIMER_REG_TIM34 as usize).cast());
}

// The remaining registration functions retain their C ABI and depend on the kernel declarations supplied by other files.
unsafe extern "C" fn davinci_timer_register(clk: *mut clk, timer_cfg: *const davinci_timer_cfg) -> i32 {
    let mut rv = clk_prepare_enable(clk);
    if rv != 0 {
        pr_err("Unable to prepare and enable the timer clock\n");
        return rv;
    }
    if !request_mem_region((*timer_cfg).reg.start, resource_size(&(*timer_cfg).reg), "davinci-timer") {
        pr_err("Unable to request memory region\n");
        rv = -16;
        clk_disable_unprepare(clk);
        return rv;
    }
    let base = ioremap((*timer_cfg).reg.start, resource_size(&(*timer_cfg).reg));
    if base.is_null() {
        pr_err("Unable to map the register range\n");
        release_mem_region((*timer_cfg).reg.start, resource_size(&(*timer_cfg).reg));
        clk_disable_unprepare(clk);
        return -12;
    }
    davinci_timer_init(base);
    let tick_rate = clk_get_rate(clk);
    let clockevent = kzalloc_obj::<DavinciClockevent>();
    if clockevent.is_null() {
        iounmap(base);
        release_mem_region((*timer_cfg).reg.start, resource_size(&(*timer_cfg).reg));
        clk_disable_unprepare(clk);
        return -12;
    }
    (*clockevent).dev.name = "tim12";
    (*clockevent).dev.features = CLOCK_EVT_FEAT_ONESHOT;
    (*clockevent).dev.cpumask = cpumask_of(0);
    (*clockevent).base = base;
    if (*timer_cfg).cmp_off != 0 {
        (*clockevent).cmp_off = (*timer_cfg).cmp_off;
        (*clockevent).dev.set_next_event = Some(davinci_clockevent_set_next_event_cmp);
    } else {
        (*clockevent).dev.set_next_event = Some(davinci_clockevent_set_next_event_std);
        (*clockevent).dev.set_state_oneshot = Some(davinci_clockevent_set_oneshot);
        (*clockevent).dev.set_state_shutdown = Some(davinci_clockevent_shutdown);
    }
    rv = request_irq((*timer_cfg).irq[DAVINCI_TIMER_CLOCKEVENT_IRQ as usize].start,
                     davinci_timer_irq_timer, IRQF_TIMER, "clockevent/tim12", clockevent.cast());
    if rv != 0 {
        pr_err("Unable to request the clockevent interrupt\n");
        kfree(clockevent);
        iounmap(base);
        release_mem_region((*timer_cfg).reg.start, resource_size(&(*timer_cfg).reg));
        clk_disable_unprepare(clk);
        return rv;
    }
    DAVINCI_CLOCKSOURCE.dev.rating = 300;
    DAVINCI_CLOCKSOURCE.dev.read = Some(davinci_clocksource_read);
    DAVINCI_CLOCKSOURCE.dev.mask = clocksource_mask(DAVINCI_TIMER_CLKSRC_BITS);
    DAVINCI_CLOCKSOURCE.dev.flags = CLOCK_SOURCE_IS_CONTINUOUS;
    DAVINCI_CLOCKSOURCE.base = base;
    if (*timer_cfg).cmp_off != 0 {
        DAVINCI_CLOCKSOURCE.dev.name = "tim12";
        DAVINCI_CLOCKSOURCE.tim_off = DAVINCI_TIMER_REG_TIM12;
        davinci_clocksource_init_tim12(base);
    } else {
        DAVINCI_CLOCKSOURCE.dev.name = "tim34";
        DAVINCI_CLOCKSOURCE.tim_off = DAVINCI_TIMER_REG_TIM34;
        davinci_clocksource_init_tim34(base);
    }
    clockevents_config_and_register(&mut (*clockevent).dev, tick_rate, DAVINCI_TIMER_MIN_DELTA, DAVINCI_TIMER_MAX_DELTA);
    rv = clocksource_register_hz(&mut DAVINCI_CLOCKSOURCE.dev, tick_rate);
    if rv != 0 {
        pr_err("Unable to register clocksource\n");
        free_irq((*timer_cfg).irq[DAVINCI_TIMER_CLOCKEVENT_IRQ as usize].start, clockevent.cast());
        kfree(clockevent);
        iounmap(base);
        release_mem_region((*timer_cfg).reg.start, resource_size(&(*timer_cfg).reg));
        clk_disable_unprepare(clk);
        return rv;
    }
    sched_clock_register(Some(davinci_timer_read_sched_clock), DAVINCI_TIMER_CLKSRC_BITS, tick_rate);
    0
}

unsafe extern "C" fn of_davinci_timer_register(np: *mut device_node) -> i32 {
    let mut timer_cfg: davinci_timer_cfg = core::mem::zeroed();
    let mut rv = of_address_to_resource(np, 0, &mut timer_cfg.reg);
    if rv != 0 { pr_err("Unable to get the register range for timer\n"); return rv; }
    rv = of_irq_to_resource_table(np, timer_cfg.irq.as_mut_ptr(), DAVINCI_TIMER_NUM_IRQS);
    if rv != DAVINCI_TIMER_NUM_IRQS as i32 { pr_err("Unable to get the interrupts for timer\n"); return rv; }
    let clk = of_clk_get(np, 0);
    if is_err(clk) { pr_err("Unable to get the timer clock\n"); return ptr_err(clk); }
    rv = davinci_timer_register(clk, &timer_cfg);
    if rv != 0 { clk_put(clk); }
    rv
}

// TIMER_OF_DECLARE(davinci_timer, "ti,da830-timer", of_davinci_timer_register);

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
