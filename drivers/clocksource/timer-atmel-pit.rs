// SPDX-License-Identifier: GPL-2.0-only
/*
 * at91sam926x_time.c - Periodic Interval Timer (PIT) for at91sam926x
 *
 * Copyright (C) 2005-2006 M. Amine SAYA, ATMEL Rousset, France
 * Revision\t 2005 M. Nicolas Diremdjian, ATMEL Rousset, France
 * Converted to ClockSource/ClockEvents by David Brownell.
 */

// pr_fmt(fmt) = "AT91: PIT: " fmt
// External kernel headers and symbols are supplied by the surrounding tree.

const AT91_PIT_MR: u32 = 0x00; // Mode Register
const AT91_PIT_PITIEN: u32 = 1u32 << 25; // Timer Interrupt Enable
const AT91_PIT_PITEN: u32 = 1u32 << 24; // Timer Enabled
const AT91_PIT_PIV: u32 = (1u32 << 20) - 1; // Periodic Interval Value

const AT91_PIT_SR: u32 = 0x04; // Status Register
const AT91_PIT_PITS: u32 = 1u32 << 0; // Timer Status

const AT91_PIT_PIVR: u32 = 0x08; // Periodic Interval Value Register
const AT91_PIT_PIIR: u32 = 0x0c; // Periodic Interval Image Register
const AT91_PIT_PICNT: u32 = 0xfffu32 << 20; // Interval Counter
const AT91_PIT_CPIV: u32 = (1u32 << 20) - 1; // Inverval Value

#[inline]
fn PIT_CPIV(x: u32) -> u32 { x & AT91_PIT_CPIV }
#[inline]
fn PIT_PICNT(x: u32) -> u32 { (x & AT91_PIT_PICNT) >> 20 }

#[repr(C)]
struct pit_data {
    clkevt: clock_event_device,
    clksrc: clocksource,
    base: *mut core::ffi::c_void,
    cycle: u32,
    cnt: u32,
    irq: u32,
    mck: *mut clk,
}

#[inline]
unsafe fn clksrc_to_pit_data(clksrc: *mut clocksource) -> *mut pit_data {
    container_of!(clksrc, pit_data, clksrc)
}

#[inline]
unsafe fn clkevt_to_pit_data(clkevt: *mut clock_event_device) -> *mut pit_data {
    container_of!(clkevt, pit_data, clkevt)
}

#[inline]
unsafe fn pit_read(base: *mut core::ffi::c_void, reg_offset: u32) -> u32 {
    readl_relaxed(base.add(reg_offset as usize) as *const core::ffi::c_void)
}

#[inline]
unsafe fn pit_write(base: *mut core::ffi::c_void, reg_offset: u32, value: usize) {
    writel_relaxed(value, base.add(reg_offset as usize));
}

/*
 * Clocksource:  just a monotonic counter of MCK/16 cycles.
 * We don't care whether or not PIT irqs are enabled.
 */
unsafe fn read_pit_clk(cs: *mut clocksource) -> u64 {
    let data = clksrc_to_pit_data(cs);
    let mut flags: usize = 0;
    raw_local_irq_save(&mut flags);
    let mut elapsed = (*data).cnt;
    let t = pit_read((*data).base, AT91_PIT_PIIR);
    raw_local_irq_restore(flags);
    elapsed = elapsed.wrapping_add(PIT_PICNT(t).wrapping_mul((*data).cycle));
    elapsed = elapsed.wrapping_add(PIT_CPIV(t));
    elapsed as u64
}

unsafe fn pit_clkevt_shutdown(dev: *mut clock_event_device) -> i32 {
    let data = clkevt_to_pit_data(dev);
    pit_write((*data).base, AT91_PIT_MR,
        ((*data).cycle.wrapping_sub(1) | AT91_PIT_PITEN) as usize);
    0
}

/* Clockevent device: interrupts every 1/HZ (== pit_cycles * MCK/16) */
unsafe fn pit_clkevt_set_periodic(dev: *mut clock_event_device) -> i32 {
    let data = clkevt_to_pit_data(dev);
    (*data).cnt = (*data).cnt.wrapping_add(
        (*data).cycle.wrapping_mul(PIT_PICNT(pit_read((*data).base, AT91_PIT_PIVR))));
    pit_write((*data).base, AT91_PIT_MR,
        ((*data).cycle.wrapping_sub(1) | AT91_PIT_PITEN | AT91_PIT_PITIEN) as usize);
    0
}

unsafe fn at91sam926x_pit_suspend(cedev: *mut clock_event_device) {
    let data = clkevt_to_pit_data(cedev);
    pit_write((*data).base, AT91_PIT_MR, 0);
}

unsafe fn at91sam926x_pit_reset(data: *mut pit_data) {
    pit_write((*data).base, AT91_PIT_MR, 0);
    while PIT_CPIV(pit_read((*data).base, AT91_PIT_PIVR)) != 0 { cpu_relax(); }
    pit_write((*data).base, AT91_PIT_MR,
        ((*data).cycle.wrapping_sub(1) | AT91_PIT_PITEN) as usize);
}

unsafe fn at91sam926x_pit_resume(cedev: *mut clock_event_device) {
    at91sam926x_pit_reset(clkevt_to_pit_data(cedev));
}

/* IRQ handler for the timer. */
unsafe fn at91sam926x_pit_interrupt(_irq: i32, dev_id: *mut core::ffi::c_void) -> irqreturn_t {
    let data = dev_id as *mut pit_data;
    if clockevent_state_periodic(&mut (*data).clkevt) &&
       (pit_read((*data).base, AT91_PIT_SR) & AT91_PIT_PITS) != 0 {
        (*data).cnt = (*data).cnt.wrapping_add(
            (*data).cycle.wrapping_mul(PIT_PICNT(pit_read((*data).base, AT91_PIT_PIVR))));
        ((*data).clkevt.event_handler)(&mut (*data).clkevt);
        return IRQ_HANDLED;
    }
    IRQ_NONE
}

/* Set up both clocksource and clockevent support. */
unsafe fn at91sam926x_pit_dt_init(node: *mut device_node) -> i32 {
    let mut pit_rate: usize;
    let bits: u32;
    let mut ret: i32;
    let data = kzalloc_obj::<pit_data>();
    if data.is_null() { return -ENOMEM; }
    (*data).base = of_iomap(node, 0);
    if (*data).base.is_null() { pr_err!("Could not map PIT address\n"); ret = -ENXIO; return { kfree(data); ret }; }
    (*data).mck = of_clk_get(node, 0);
    if IS_ERR!((*data).mck) { pr_err!("Unable to get mck clk\n"); ret = PTR_ERR!((*data).mck); return { kfree(data); ret }; }
    ret = clk_prepare_enable((*data).mck); if ret != 0 { pr_err!("Unable to enable mck\n"); return { kfree(data); ret }; }
    (*data).irq = irq_of_parse_and_map(node, 0);
    if (*data).irq == 0 { pr_err!("Unable to get IRQ from DT\n"); ret = -EINVAL; return { kfree(data); ret }; }
    pit_rate = clk_get_rate((*data).mck) / 16;
    (*data).cycle = DIV_ROUND_CLOSEST!(pit_rate, HZ);
    WARN_ON!((((*data).cycle.wrapping_sub(1)) & !AT91_PIT_PIV) != 0);
    at91sam926x_pit_reset(data);
    bits = 12 + ilog2((*data).cycle);
    (*data).clksrc.mask = CLOCKSOURCE_MASK!(bits);
    (*data).clksrc.name = "pit"; (*data).clksrc.rating = 175;
    (*data).clksrc.read = Some(read_pit_clk);
    (*data).clksrc.flags = CLOCK_SOURCE_IS_CONTINUOUS;
    ret = clocksource_register_hz(&mut (*data).clksrc, pit_rate);
    if ret != 0 { pr_err!("Failed to register clocksource\n"); return { kfree(data); ret }; }
    ret = request_irq((*data).irq, Some(at91sam926x_pit_interrupt), IRQF_SHARED | IRQF_TIMER | IRQF_IRQPOLL, "at91_tick", data as *mut _);
    if ret != 0 { pr_err!("Unable to setup IRQ\n"); clocksource_unregister(&mut (*data).clksrc); return { kfree(data); ret }; }
    (*data).clkevt.name = "pit"; (*data).clkevt.features = CLOCK_EVT_FEAT_PERIODIC;
    (*data).clkevt.shift = 32; (*data).clkevt.mult = div_sc(pit_rate, NSEC_PER_SEC, (*data).clkevt.shift);
    (*data).clkevt.rating = 100; (*data).clkevt.cpumask = cpumask_of(0);
    (*data).clkevt.set_state_shutdown = Some(pit_clkevt_shutdown);
    (*data).clkevt.set_state_periodic = Some(pit_clkevt_set_periodic);
    (*data).clkevt.resume = Some(at91sam926x_pit_resume); (*data).clkevt.suspend = Some(at91sam926x_pit_suspend);
    clockevents_register_device(&mut (*data).clkevt);
    0
}

// TIMER_OF_DECLARE(at91sam926x_pit, "atmel,at91sam9260-pit", at91sam926x_pit_dt_init);

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
