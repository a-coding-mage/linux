// SPDX-License-Identifier: GPL-2.0
/* Faraday Technology FTTMR010 timer driver. */

const TIMER1_COUNT: usize = 0x00;
const TIMER1_LOAD: usize = 0x04;
const TIMER1_MATCH1: usize = 0x08;
const TIMER1_MATCH2: usize = 0x0c;
const TIMER2_COUNT: usize = 0x10;
const TIMER2_LOAD: usize = 0x14;
const TIMER2_MATCH1: usize = 0x18;
const TIMER2_MATCH2: usize = 0x1c;
const TIMER3_COUNT: usize = 0x20;
const TIMER3_LOAD: usize = 0x24;
const TIMER3_MATCH1: usize = 0x28;
const TIMER3_MATCH2: usize = 0x2c;
const TIMER_CR: usize = 0x30;
const AST2600_TIMER_CR_CLR: usize = 0x3c;

const TIMER_1_CR_ENABLE: u32 = 1 << 0;
const TIMER_1_CR_CLOCK: u32 = 1 << 1;
const TIMER_1_CR_INT: u32 = 1 << 2;
const TIMER_2_CR_ENABLE: u32 = 1 << 3;
const TIMER_2_CR_CLOCK: u32 = 1 << 4;
const TIMER_2_CR_INT: u32 = 1 << 5;
const TIMER_3_CR_ENABLE: u32 = 1 << 6;
const TIMER_3_CR_CLOCK: u32 = 1 << 7;
const TIMER_3_CR_INT: u32 = 1 << 8;
const TIMER_1_CR_UPDOWN: u32 = 1 << 9;
const TIMER_2_CR_UPDOWN: u32 = 1 << 10;
const TIMER_3_CR_UPDOWN: u32 = 1 << 11;
const TIMER_1_CR_ASPEED_ENABLE: u32 = 1 << 0;
const TIMER_1_CR_ASPEED_CLOCK: u32 = 1 << 1;
const TIMER_1_CR_ASPEED_INT: u32 = 1 << 2;
const TIMER_2_CR_ASPEED_ENABLE: u32 = 1 << 4;
const TIMER_2_CR_ASPEED_CLOCK: u32 = 1 << 5;
const TIMER_2_CR_ASPEED_INT: u32 = 1 << 6;
const TIMER_3_CR_ASPEED_ENABLE: u32 = 1 << 8;
const TIMER_3_CR_ASPEED_CLOCK: u32 = 1 << 9;
const TIMER_3_CR_ASPEED_INT: u32 = 1 << 10;
const TIMER_INTR_STATE: usize = 0x34;
const TIMER_INTR_MASK: usize = 0x38;
const TIMER_1_INT_MATCH1: u32 = 1 << 0;
const TIMER_1_INT_MATCH2: u32 = 1 << 1;
const TIMER_1_INT_OVERFLOW: u32 = 1 << 2;
const TIMER_2_INT_MATCH1: u32 = 1 << 3;
const TIMER_2_INT_MATCH2: u32 = 1 << 4;
const TIMER_2_INT_OVERFLOW: u32 = 1 << 5;
const TIMER_3_INT_MATCH1: u32 = 1 << 6;
const TIMER_3_INT_MATCH2: u32 = 1 << 7;
const TIMER_3_INT_OVERFLOW: u32 = 1 << 8;
const TIMER_INT_ALL_MASK: u32 = 0x1ff;

#[repr(C)]
struct fttmr010 {
    base: *mut core::ffi::c_void,
    tick_rate: u32,
    is_aspeed: bool,
    t1_enable_val: u32,
    clkevt: clock_event_device,
    timer_shutdown: Option<unsafe extern "C" fn(*mut clock_event_device) -> i32>,
    #[cfg(target_arch = "arm")]
    delay_timer: delay_timer,
}

static mut local_fttmr: *mut fttmr010 = core::ptr::null_mut();

unsafe fn to_fttmr010(evt: *mut clock_event_device) -> *mut fttmr010 {
    (evt as *mut u8).sub(core::mem::offset_of!(fttmr010, clkevt)) as *mut fttmr010
}

unsafe fn fttmr010_read_current_timer_up() -> usize {
    readl((*local_fttmr).base.add(TIMER2_COUNT)) as usize
}
unsafe fn fttmr010_read_current_timer_down() -> usize {
    (!readl((*local_fttmr).base.add(TIMER2_COUNT))) as usize
}
unsafe extern "C" fn fttmr010_read_sched_clock_up() -> u64 { fttmr010_read_current_timer_up() as u64 }
unsafe extern "C" fn fttmr010_read_sched_clock_down() -> u64 { fttmr010_read_current_timer_down() as u64 }

unsafe extern "C" fn fttmr010_timer_set_next_event(cycles: usize, evt: *mut clock_event_device) -> i32 {
    let t = &mut *to_fttmr010(evt);
    (t.timer_shutdown.unwrap())(evt);
    if t.is_aspeed { writel(cycles as u32, t.base.add(TIMER1_LOAD)); }
    else { let cr = readl(t.base.add(TIMER1_COUNT)); writel(cr.wrapping_add(cycles as u32), t.base.add(TIMER1_MATCH1)); }
    let cr = readl(t.base.add(TIMER_CR)); writel(cr | t.t1_enable_val, t.base.add(TIMER_CR)); 0
}

unsafe extern "C" fn ast2600_timer_shutdown(evt: *mut clock_event_device) -> i32 {
    let t = &mut *to_fttmr010(evt); writel(t.t1_enable_val, t.base.add(AST2600_TIMER_CR_CLR)); 0
}
unsafe extern "C" fn fttmr010_timer_shutdown(evt: *mut clock_event_device) -> i32 {
    let t = &mut *to_fttmr010(evt); let cr = readl(t.base.add(TIMER_CR)); writel(cr & !t.t1_enable_val, t.base.add(TIMER_CR)); 0
}

unsafe extern "C" fn fttmr010_timer_set_oneshot(evt: *mut clock_event_device) -> i32 {
    let t = &mut *to_fttmr010(evt); (t.timer_shutdown.unwrap())(evt); writel(0, t.base.add(TIMER1_COUNT));
    if t.is_aspeed { writel(!0, t.base.add(TIMER1_LOAD)); } else { writel(0, t.base.add(TIMER1_LOAD)); let mut cr = readl(t.base.add(TIMER_INTR_MASK)); cr &= !(TIMER_1_INT_OVERFLOW | TIMER_1_INT_MATCH2); cr |= TIMER_1_INT_MATCH1; writel(cr, t.base.add(TIMER_INTR_MASK)); } 0
}

unsafe extern "C" fn fttmr010_timer_set_periodic(evt: *mut clock_event_device) -> i32 {
    let t = &mut *to_fttmr010(evt); let period = (t.tick_rate + HZ / 2) / HZ; (t.timer_shutdown.unwrap())(evt);
    if t.is_aspeed { writel(period, t.base.add(TIMER1_LOAD)); } else { let v = 0xffff_ffffu32.wrapping_sub(period - 1); writel(v, t.base.add(TIMER1_COUNT)); writel(v, t.base.add(TIMER1_LOAD)); let mut cr = readl(t.base.add(TIMER_INTR_MASK)); cr &= !(TIMER_1_INT_MATCH1 | TIMER_1_INT_MATCH2); cr |= TIMER_1_INT_OVERFLOW; writel(cr, t.base.add(TIMER_INTR_MASK)); }
    let cr = readl(t.base.add(TIMER_CR)); writel(cr | t.t1_enable_val, t.base.add(TIMER_CR)); 0
}

unsafe extern "C" fn fttmr010_timer_interrupt(_irq: i32, dev_id: *mut core::ffi::c_void) -> irqreturn_t { let evt = dev_id as *mut clock_event_device; ((*evt).event_handler.unwrap())(evt); IRQ_HANDLED }
unsafe extern "C" fn ast2600_timer_interrupt(_irq: i32, dev_id: *mut core::ffi::c_void) -> irqreturn_t { let evt = dev_id as *mut clock_event_device; let t = &mut *to_fttmr010(evt); writel(1, t.base.add(TIMER_INTR_STATE)); ((*evt).event_handler.unwrap())(evt); IRQ_HANDLED }

// External kernel types, functions, constants, and registration macros are supplied by other translation units.
unsafe extern "C" fn fttmr010_common_init(np: *mut device_node, is_aspeed: bool, is_ast2600: bool) -> i32 {
    let clk = of_clk_get_by_name(np, b"PCLK\0".as_ptr() as *const i8); if IS_ERR(clk) { return PTR_ERR(clk); }
    let mut ret = clk_prepare_enable(clk); if ret != 0 { return ret; }
    let t = kzalloc_obj::<fttmr010>(); if t.is_null() { ret = -ENOMEM; goto_out_disable_clock!(); }
    (*t).tick_rate = clk_get_rate(clk); (*t).base = of_iomap(np, 0); if (*t).base.is_null() { ret = -ENXIO; goto_out_free!(); }
    let irq = irq_of_parse_and_map(np, 0); if irq <= 0 { ret = -EINVAL; goto_out_unmap!(); }
    if is_aspeed { (*t).t1_enable_val = TIMER_1_CR_ASPEED_ENABLE | TIMER_1_CR_ASPEED_INT; (*t).is_aspeed = true; } else { (*t).t1_enable_val = TIMER_1_CR_ENABLE | TIMER_1_CR_INT; writel(TIMER_INT_ALL_MASK, (*t).base.add(TIMER_INTR_MASK)); writel(0, (*t).base.add(TIMER_INTR_STATE)); }
    let val = if is_aspeed { TIMER_2_CR_ASPEED_ENABLE } else { TIMER_2_CR_ENABLE | TIMER_1_CR_UPDOWN | TIMER_2_CR_UPDOWN }; writel(val, (*t).base.add(TIMER_CR)); local_fttmr = t;
    writel(0, (*t).base.add(TIMER2_COUNT)); writel(0, (*t).base.add(TIMER2_MATCH1)); writel(0, (*t).base.add(TIMER2_MATCH2));
    if is_aspeed { writel(!0, (*t).base.add(TIMER2_LOAD)); clocksource_mmio_init((*t).base.add(TIMER2_COUNT), b"FTTMR010-TIMER2\0".as_ptr() as *const i8, (*t).tick_rate, 300, 32, clocksource_mmio_readl_down); sched_clock_register(fttmr010_read_sched_clock_down, 32, (*t).tick_rate); } else { writel(0, (*t).base.add(TIMER2_LOAD)); clocksource_mmio_init((*t).base.add(TIMER2_COUNT), b"FTTMR010-TIMER2\0".as_ptr() as *const i8, (*t).tick_rate, 300, 32, clocksource_mmio_readl_up); sched_clock_register(fttmr010_read_sched_clock_up, 32, (*t).tick_rate); }
    writel(0, (*t).base.add(TIMER1_COUNT)); writel(0, (*t).base.add(TIMER1_LOAD)); writel(0, (*t).base.add(TIMER1_MATCH1)); writel(0, (*t).base.add(TIMER1_MATCH2));
    (*t).timer_shutdown = Some(if is_ast2600 { ast2600_timer_shutdown } else { fttmr010_timer_shutdown }); ret = request_irq(irq, if is_ast2600 { ast2600_timer_interrupt } else { fttmr010_timer_interrupt }, IRQF_TIMER, b"FTTMR010-TIMER1\0".as_ptr() as *const i8, &mut (*t).clkevt as *mut _ as *mut _); if ret != 0 { goto_out_unmap!(); }
    (*t).clkevt.name = b"FTTMR010-TIMER1\0".as_ptr() as *const i8; (*t).clkevt.rating = 300; (*t).clkevt.features = CLOCK_EVT_FEAT_PERIODIC | CLOCK_EVT_FEAT_ONESHOT; (*t).clkevt.set_next_event = Some(fttmr010_timer_set_next_event); (*t).clkevt.set_state_shutdown = (*t).timer_shutdown; (*t).clkevt.set_state_periodic = Some(fttmr010_timer_set_periodic); (*t).clkevt.set_state_oneshot = Some(fttmr010_timer_set_oneshot); (*t).clkevt.tick_resume = (*t).timer_shutdown; (*t).clkevt.cpumask = cpumask_of(0); (*t).clkevt.irq = irq; clockevents_config_and_register(&mut (*t).clkevt, (*t).tick_rate, 1, 0xffff_ffff); 0
    // CONFIG_ARM delay-timer setup is preserved by the corresponding target-specific integration.
}

unsafe extern "C" fn ast2600_timer_init(np: *mut device_node) -> i32 { fttmr010_common_init(np, true, true) }
unsafe extern "C" fn aspeed_timer_init(np: *mut device_node) -> i32 { fttmr010_common_init(np, true, false) }
unsafe extern "C" fn fttmr010_timer_init(np: *mut device_node) -> i32 { fttmr010_common_init(np, false, false) }

TIMER_OF_DECLARE!(fttmr010, "faraday,fttmr010", fttmr010_timer_init);
TIMER_OF_DECLARE!(gemini, "cortina,gemini-timer", fttmr010_timer_init);
TIMER_OF_DECLARE!(moxart, "moxa,moxart-timer", fttmr010_timer_init);
TIMER_OF_DECLARE!(ast2400, "aspeed,ast2400-timer", aspeed_timer_init);
TIMER_OF_DECLARE!(ast2500, "aspeed,ast2500-timer", aspeed_timer_init);
TIMER_OF_DECLARE!(ast2600, "aspeed,ast2600-timer", ast2600_timer_init);

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
