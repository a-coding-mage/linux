// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * Freescale FlexTimer Module (FTM) timer driver.
 *
 * Copyright 2014 Freescale Semiconductor, Inc.
 */

// Kernel dependencies supplied by other translation units.

const FTM_SC_CLK_MASK_SHIFT: u32 = 3;
const FTM_SC_PS_MASK: u32 = 0x07;
const FTM_SC_CLK_MASK: u32 = 0x18;
const FTM_SC: usize = 0x00;
const FTM_CNT: usize = 0x04;
const FTM_MOD: usize = 0x08;
const FTM_CNTIN: usize = 0x4c;
const FTM_SC_TOF: u32 = 1 << 7;
const FTM_SC_TOIE: u32 = 1 << 6;
const FTM_PS_MAX: usize = 7;
const HZ: usize = 100;

type U32 = u32;
type U64 = u64;
type CInt = i32;
type CLong = isize;
type ULong = usize;
type Irqreturn = CInt;

#[repr(C)]
pub struct DeviceNode {
    _private: [u8; 0],
}

#[repr(C)]
pub struct Clk {
    _private: [u8; 0],
}

#[repr(C)]
pub struct ClockEventDevice {
    pub name: *const i8,
    pub features: ULong,
    pub set_state_periodic: Option<unsafe fn(*mut ClockEventDevice) -> CInt>,
    pub set_state_oneshot: Option<unsafe fn(*mut ClockEventDevice) -> CInt>,
    pub set_next_event: Option<unsafe fn(ULong, *mut ClockEventDevice) -> CInt>,
    pub rating: CInt,
    pub cpumask: *const core::ffi::c_void,
    pub irq: CInt,
    pub event_handler: unsafe fn(*mut ClockEventDevice),
}

#[repr(C)]
pub struct FtmClockDevice {
    clksrc_base: *mut u8,
    clkevt_base: *mut u8,
    periodic_cyc: ULong,
    ps: ULong,
    big_endian: bool,
}

static mut PRIV: *mut FtmClockDevice = core::ptr::null_mut();

const fn ftm_sc_clk(c: U32) -> U32 {
    c << FTM_SC_CLK_MASK_SHIFT
}

unsafe extern "C" {
    fn ioread32(addr: *mut u8) -> U32;
    fn ioread32be(addr: *mut u8) -> U32;
    fn iowrite32(val: U32, addr: *mut u8);
    fn iowrite32be(val: U32, addr: *mut u8);
    fn request_irq(irq: CInt, handler: unsafe extern "C" fn(CInt, *mut core::ffi::c_void) -> Irqreturn, flags: ULong, name: *const i8, dev_id: *mut core::ffi::c_void) -> CInt;
    fn clockevent_state_oneshot(evt: *mut ClockEventDevice) -> bool;
    fn clockevents_config_and_register(evt: *mut ClockEventDevice, freq: ULong, min_delta: ULong, max_delta: ULong);
    fn cpumask_of(cpu: ULong) -> *const core::ffi::c_void;
    fn clocksource_mmio_init(base: *mut u8, name: *const i8, freq: ULong, rating: CInt, bits: CInt, read: *const core::ffi::c_void) -> CInt;
    fn sched_clock_register(read: unsafe fn() -> U64, bits: CInt, freq: ULong);
    fn of_clk_get_by_name(np: *mut DeviceNode, name: *mut i8) -> *mut Clk;
    fn clk_prepare_enable(clk: *mut Clk) -> CInt;
    fn clk_get_rate(clk: *mut Clk) -> ULong;
    fn of_iomap(np: *mut DeviceNode, index: CInt) -> *mut u8;
    fn irq_of_parse_and_map(np: *mut DeviceNode, index: CInt) -> CInt;
    fn of_property_read_bool(np: *mut DeviceNode, name: *const i8) -> bool;
    fn iounmap(addr: *mut u8);
    fn kzalloc<T>() -> *mut T;
    fn kfree(ptr: *mut core::ffi::c_void);
}

unsafe fn ftm_readl(addr: *mut u8) -> U32 {
    if (*PRIV).big_endian { ioread32be(addr) } else { ioread32(addr) }
}

unsafe fn ftm_writel(val: U32, addr: *mut u8) {
    if (*PRIV).big_endian { iowrite32be(val, addr) } else { iowrite32(val, addr) }
}

unsafe fn ftm_counter_enable(base: *mut u8) {
    let mut val = ftm_readl(base.add(FTM_SC));
    val &= !(FTM_SC_PS_MASK | FTM_SC_CLK_MASK);
    val |= (*PRIV).ps as U32 | ftm_sc_clk(1);
    ftm_writel(val, base.add(FTM_SC));
}

unsafe fn ftm_counter_disable(base: *mut u8) {
    let mut val = ftm_readl(base.add(FTM_SC));
    val &= !(FTM_SC_PS_MASK | FTM_SC_CLK_MASK);
    ftm_writel(val, base.add(FTM_SC));
}

unsafe fn ftm_irq_acknowledge(base: *mut u8) {
    let mut val = ftm_readl(base.add(FTM_SC));
    val &= !FTM_SC_TOF;
    ftm_writel(val, base.add(FTM_SC));
}

unsafe fn ftm_irq_enable(base: *mut u8) {
    let mut val = ftm_readl(base.add(FTM_SC));
    val |= FTM_SC_TOIE;
    ftm_writel(val, base.add(FTM_SC));
}

unsafe fn ftm_irq_disable(base: *mut u8) {
    let mut val = ftm_readl(base.add(FTM_SC));
    val &= !FTM_SC_TOIE;
    ftm_writel(val, base.add(FTM_SC));
}

unsafe fn ftm_reset_counter(base: *mut u8) {
    /*
     * The CNT register contains the FTM counter value.
     * Reset clears the CNT register. Writing any value to COUNT
     * updates the counter with its initial value, CNTIN.
     */
    ftm_writel(0x00, base.add(FTM_CNT));
}

unsafe fn ftm_read_sched_clock() -> U64 {
    ftm_readl((*PRIV).clksrc_base.add(FTM_CNT)) as U64
}

unsafe fn ftm_set_next_event(delta: ULong, _unused: *mut ClockEventDevice) -> CInt {
    ftm_counter_disable((*PRIV).clkevt_base);
    ftm_reset_counter((*PRIV).clkevt_base);
    ftm_writel((delta.wrapping_sub(1)) as U32, (*PRIV).clkevt_base.add(FTM_MOD));
    ftm_counter_enable((*PRIV).clkevt_base);
    ftm_irq_enable((*PRIV).clkevt_base);
    0
}

unsafe fn ftm_set_oneshot(_evt: *mut ClockEventDevice) -> CInt {
    ftm_counter_disable((*PRIV).clkevt_base);
    0
}

unsafe fn ftm_set_periodic(evt: *mut ClockEventDevice) -> CInt {
    ftm_set_next_event((*PRIV).periodic_cyc, evt);
    0
}

unsafe extern "C" fn ftm_evt_interrupt(_irq: CInt, dev_id: *mut core::ffi::c_void) -> Irqreturn {
    let evt = dev_id as *mut ClockEventDevice;
    ftm_irq_acknowledge((*PRIV).clkevt_base);
    if clockevent_state_oneshot(evt) {
        ftm_irq_disable((*PRIV).clkevt_base);
        ftm_counter_disable((*PRIV).clkevt_base);
    }
    ((*evt).event_handler)(evt);
    1
}

static mut FTM_CLOCKEVENT: ClockEventDevice = ClockEventDevice {
    name: b"Freescale ftm timer\0".as_ptr() as *const i8,
    features: 0x01 | 0x02,
    set_state_periodic: Some(ftm_set_periodic),
    set_state_oneshot: Some(ftm_set_oneshot),
    set_next_event: Some(ftm_set_next_event),
    rating: 300,
    cpumask: core::ptr::null(),
    irq: 0,
    event_handler: event_handler_stub,
};

unsafe fn event_handler_stub(_evt: *mut ClockEventDevice) {}

unsafe fn ftm_clockevent_init(freq: ULong, irq: CInt) -> CInt {
    ftm_writel(0x00, (*PRIV).clkevt_base.add(FTM_CNTIN));
    ftm_writel(!0u32, (*PRIV).clkevt_base.add(FTM_MOD));
    ftm_reset_counter((*PRIV).clkevt_base);
    let err = request_irq(irq, ftm_evt_interrupt, 0, b"Freescale ftm timer\0".as_ptr() as *const i8, &raw mut FTM_CLOCKEVENT as *mut _ as *mut core::ffi::c_void);
    if err != 0 { return err; }
    FTM_CLOCKEVENT.cpumask = cpumask_of(0);
    FTM_CLOCKEVENT.irq = irq;
    clockevents_config_and_register(&raw mut FTM_CLOCKEVENT, freq / (1usize << (*PRIV).ps), 1, 0xffff);
    ftm_counter_enable((*PRIV).clkevt_base);
    0
}

unsafe fn ftm_clocksource_init(freq: ULong) -> CInt {
    ftm_writel(0x00, (*PRIV).clksrc_base.add(FTM_CNTIN));
    ftm_writel(!0u32, (*PRIV).clksrc_base.add(FTM_MOD));
    ftm_reset_counter((*PRIV).clksrc_base);
    sched_clock_register(ftm_read_sched_clock, 16, freq / (1usize << (*PRIV).ps));
    let err = clocksource_mmio_init((*PRIV).clksrc_base.add(FTM_CNT), b"fsl-ftm\0".as_ptr() as *const i8, freq / (1usize << (*PRIV).ps), 300, 16, core::ptr::null());
    if err != 0 { return err; }
    ftm_counter_enable((*PRIV).clksrc_base);
    0
}

unsafe fn __ftm_clk_init(np: *mut DeviceNode, cnt_name: *mut i8, ftm_name: *mut i8) -> CLong {
    let clk = of_clk_get_by_name(np, cnt_name);
    if clk.is_null() { return -19; }
    let mut err = clk_prepare_enable(clk);
    if err != 0 { return err as CLong; }
    let clk = of_clk_get_by_name(np, ftm_name);
    if clk.is_null() { return -19; }
    err = clk_prepare_enable(clk);
    if err != 0 { return err as CLong; }
    clk_get_rate(clk) as CLong
}

unsafe fn ftm_clk_init(np: *mut DeviceNode) -> ULong {
    let mut freq = __ftm_clk_init(np, b"ftm-evt-counter-en\0".as_ptr() as *mut i8, b"ftm-evt\0".as_ptr() as *mut i8);
    if freq <= 0 { return 0; }
    freq = __ftm_clk_init(np, b"ftm-src-counter-en\0".as_ptr() as *mut i8, b"ftm-src\0".as_ptr() as *mut i8);
    if freq <= 0 { return 0; }
    freq as ULong
}

unsafe fn ftm_calc_closest_round_cyc(freq: ULong) -> CInt {
    (*PRIV).ps = 0;
    loop {
        (*PRIV).ps = (*PRIV).ps.wrapping_add(1);
        (*PRIV).periodic_cyc = (freq + (HZ * (1usize << (*PRIV).ps)) / 2) / (HZ * (1usize << (*PRIV).ps));
        if (*PRIV).periodic_cyc <= 0xffff { break; }
    }
    if (*PRIV).ps > FTM_PS_MAX { return -22; }
    0
}

unsafe fn ftm_timer_init(np: *mut DeviceNode) -> CInt {
    PRIV = kzalloc::<FtmClockDevice>();
    if PRIV.is_null() { return -12; }
    let mut ret = -6;
    (*PRIV).clkevt_base = of_iomap(np, 0);
    if (*PRIV).clkevt_base.is_null() { ret = -6; goto_err(); return ret; }
    (*PRIV).clksrc_base = of_iomap(np, 1);
    if (*PRIV).clksrc_base.is_null() { iounmap((*PRIV).clkevt_base); return ret; }
    let irq = irq_of_parse_and_map(np, 0);
    if irq <= 0 { iounmap((*PRIV).clksrc_base); iounmap((*PRIV).clkevt_base); return -22; }
    (*PRIV).big_endian = of_property_read_bool(np, b"big-endian\0".as_ptr() as *const i8);
    let freq = ftm_clk_init(np);
    if freq == 0 { iounmap((*PRIV).clksrc_base); iounmap((*PRIV).clkevt_base); return ret; }
    ret = ftm_calc_closest_round_cyc(freq);
    if ret != 0 { iounmap((*PRIV).clksrc_base); iounmap((*PRIV).clkevt_base); return ret; }
    ret = ftm_clocksource_init(freq);
    if ret != 0 { iounmap((*PRIV).clksrc_base); iounmap((*PRIV).clkevt_base); return ret; }
    ret = ftm_clockevent_init(freq, irq);
    if ret != 0 { iounmap((*PRIV).clksrc_base); iounmap((*PRIV).clkevt_base); return ret; }
    0
}

unsafe fn goto_err() {}

// TIMER_OF_DECLARE(flextimer, "fsl,ftm-timer", ftm_timer_init);

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
