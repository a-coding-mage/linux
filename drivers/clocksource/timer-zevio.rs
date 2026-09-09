// SPDX-License-Identifier: GPL-2.0-only
/*
 *  linux/drivers/clocksource/zevio-timer.c
 *
 *  Copyright (C) 2013 Daniel Tang <tangrs@tangrs.id.au>
 */

// Linux kernel dependencies are supplied by the surrounding translation.

const IO_CURRENT_VAL: usize = 0x00;
const IO_DIVIDER: usize = 0x04;
const IO_CONTROL: usize = 0x08;

const IO_TIMER1: usize = 0x00;
const IO_TIMER2: usize = 0x0c;

const IO_MATCH_BEGIN: usize = 0x18;
#[inline]
const fn io_match(x: usize) -> usize { IO_MATCH_BEGIN + (x << 2) }

const IO_INTR_STS: usize = 0x00;
const IO_INTR_ACK: usize = 0x00;
const IO_INTR_MSK: usize = 0x04;

const CNTL_STOP_TIMER: u32 = 1 << 4;
const CNTL_RUN_TIMER: u32 = 0 << 4;
const CNTL_INC: u32 = 1 << 3;
const CNTL_DEC: u32 = 0 << 3;
const CNTL_TOZERO: u32 = 0;
#[inline]
const fn cntl_match(x: u32) -> u32 { x + 1 }
const CNTL_FOREVER: u32 = 7;

/* There are 6 match registers but we only use one. */
const TIMER_MATCH: usize = 0;
const TIMER_INTR_MSK: u32 = 1 << TIMER_MATCH;
const TIMER_INTR_ALL: u32 = 0x3f;

#[repr(C)]
pub struct zevio_timer {
    base: *mut u8,
    timer1: *mut u8,
    timer2: *mut u8,
    interrupt_regs: *mut u8,
    clk: *mut clk,
    clkevt: clock_event_device,
    clocksource_name: [core::ffi::c_char; 64],
    clockevent_name: [core::ffi::c_char; 64],
}

#[inline]
unsafe fn reg(base: *mut u8, offset: usize) -> *mut u8 { base.add(offset) }

unsafe fn zevio_timer_set_event(delta: usize, dev: *mut clock_event_device) -> i32 {
    let timer = container_of_clkevt(dev);
    writel(delta as u32, reg((*timer).timer1, IO_CURRENT_VAL));
    writel(CNTL_RUN_TIMER | CNTL_DEC | cntl_match(TIMER_MATCH as u32),
           reg((*timer).timer1, IO_CONTROL));
    0
}

unsafe fn zevio_timer_shutdown(dev: *mut clock_event_device) -> i32 {
    let timer = container_of_clkevt(dev);
    writel(0, reg((*timer).interrupt_regs, IO_INTR_MSK));
    writel(TIMER_INTR_ALL, reg((*timer).interrupt_regs, IO_INTR_ACK));
    writel(CNTL_STOP_TIMER, reg((*timer).timer1, IO_CONTROL));
    0
}

unsafe fn zevio_timer_set_oneshot(dev: *mut clock_event_device) -> i32 {
    let timer = container_of_clkevt(dev);
    writel(TIMER_INTR_MSK, reg((*timer).interrupt_regs, IO_INTR_MSK));
    writel(TIMER_INTR_ALL, reg((*timer).interrupt_regs, IO_INTR_ACK));
    0
}

unsafe fn zevio_timer_interrupt(_irq: i32, dev_id: *mut core::ffi::c_void) -> irqreturn_t {
    let timer = dev_id as *mut zevio_timer;
    let intr = readl(reg((*timer).interrupt_regs, IO_INTR_ACK));
    if intr & TIMER_INTR_MSK == 0 { return IRQ_NONE; }
    writel(TIMER_INTR_MSK, reg((*timer).interrupt_regs, IO_INTR_ACK));
    writel(CNTL_STOP_TIMER, reg((*timer).timer1, IO_CONTROL));
    if let Some(handler) = (*timer).clkevt.event_handler {
        handler(&mut (*timer).clkevt);
    }
    IRQ_HANDLED
}

unsafe fn zevio_timer_add(node: *mut device_node) -> i32 {
    let timer = kzalloc_obj::<zevio_timer>();
    if timer.is_null() { return -12; }
    (*timer).base = of_iomap(node, 0);
    if (*timer).base.is_null() { kfree(timer); return -22; }
    (*timer).timer1 = reg((*timer).base, IO_TIMER1);
    (*timer).timer2 = reg((*timer).base, IO_TIMER2);
    (*timer).clk = of_clk_get(node, 0);
    if is_err((*timer).clk) { let ret = ptr_err((*timer).clk); iounmap((*timer).base); kfree(timer); return ret; }
    (*timer).interrupt_regs = of_iomap(node, 1);
    let irqnr = irq_of_parse_and_map(node, 0);
    let mut res = core::mem::zeroed::<resource>();
    of_address_to_resource(node, 0, &mut res);
    scnprintf((*timer).clocksource_name.as_mut_ptr(), 64, b"%llx.%pOFn_clocksource\0".as_ptr(), res.start, node);
    scnprintf((*timer).clockevent_name.as_mut_ptr(), 64, b"%llx.%pOFn_clockevent\0".as_ptr(), res.start, node);
    if !(*timer).interrupt_regs.is_null() && irqnr != 0 {
        (*timer).clkevt.name = (*timer).clockevent_name.as_mut_ptr();
        (*timer).clkevt.set_next_event = Some(zevio_timer_set_event);
        (*timer).clkevt.set_state_shutdown = Some(zevio_timer_shutdown);
        (*timer).clkevt.set_state_oneshot = Some(zevio_timer_set_oneshot);
        (*timer).clkevt.tick_resume = Some(zevio_timer_set_oneshot);
        (*timer).clkevt.rating = 200;
        (*timer).clkevt.cpumask = cpu_possible_mask;
        (*timer).clkevt.features = CLOCK_EVT_FEAT_ONESHOT;
        (*timer).clkevt.irq = irqnr;
        writel(CNTL_STOP_TIMER, reg((*timer).timer1, IO_CONTROL));
        writel(0, reg((*timer).timer1, IO_DIVIDER));
        writel(0, reg((*timer).interrupt_regs, IO_INTR_MSK));
        writel(TIMER_INTR_ALL, reg((*timer).interrupt_regs, IO_INTR_ACK));
        writel(0, reg((*timer).base, io_match(TIMER_MATCH)));
        request_irq(irqnr, Some(zevio_timer_interrupt), IRQF_TIMER | IRQF_IRQPOLL, (*timer).clockevent_name.as_ptr(), timer as *mut _);
        clockevents_config_and_register(&mut (*timer).clkevt, clk_get_rate((*timer).clk), 1, 0xffff);
    }
    writel(CNTL_STOP_TIMER, reg((*timer).timer2, IO_CONTROL));
    writel(0, reg((*timer).timer2, IO_CURRENT_VAL));
    writel(0, reg((*timer).timer2, IO_DIVIDER));
    writel(CNTL_RUN_TIMER | CNTL_FOREVER | CNTL_INC, reg((*timer).timer2, IO_CONTROL));
    clocksource_mmio_init(reg((*timer).timer2, IO_CURRENT_VAL), (*timer).clocksource_name.as_ptr(), clk_get_rate((*timer).clk), 200, 16, clocksource_mmio_readw_up);
    0
}

unsafe fn zevio_timer_init(node: *mut device_node) -> i32 { zevio_timer_add(node) }

// TIMER_OF_DECLARE(zevio_timer, "lsi,zevio-timer", zevio_timer_init);

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
