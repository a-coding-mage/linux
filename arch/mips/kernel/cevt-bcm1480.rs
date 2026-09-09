// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * Copyright (C) 2000,2001,2004 Broadcom Corporation
 */

// Linux and MIPS headers supplying the following types, constants, and functions
// are external dependencies of this translation.

use core::ffi::c_void;

type CInt = i32;
type UInt = u32;
type ULong = usize;
type IrqReturnT = CInt;

const IRQ_HANDLED: IrqReturnT = 1;
const IMR_IP2_VAL: u64 = K_BCM1480_INT_MAP_I0;
const IMR_IP3_VAL: u64 = K_BCM1480_INT_MAP_I1;
const IMR_IP4_VAL: u64 = K_BCM1480_INT_MAP_I2;

#[repr(C)]
pub struct ClockEventDevice {
    pub name: *mut u8,
    pub features: UInt,
    pub max_delta_ns: u64,
    pub max_delta_ticks: u64,
    pub min_delta_ns: u64,
    pub min_delta_ticks: u64,
    pub rating: CInt,
    pub irq: UInt,
    pub cpumask: *const c_void,
    pub set_next_event: Option<unsafe extern "C" fn(ULong, *mut ClockEventDevice) -> CInt>,
    pub set_state_shutdown: Option<unsafe extern "C" fn(*mut ClockEventDevice) -> CInt>,
    pub set_state_periodic: Option<unsafe extern "C" fn(*mut ClockEventDevice) -> CInt>,
    pub set_state_oneshot: Option<unsafe extern "C" fn(*mut ClockEventDevice) -> CInt>,
    pub event_handler: Option<unsafe extern "C" fn(*mut ClockEventDevice)>,
}

extern "C" {
    static mut sibyte_hpt_clockevent: [ClockEventDevice; 4];
    static mut sibyte_hpt_name: [[u8; 18]; 4];
    fn smp_processor_id() -> UInt;
    fn ioaddr(addr: ULong) -> *mut c_void;
    fn __raw_writeq(value: u64, addr: *mut c_void);
    fn ____raw_writeq(value: u64, addr: *mut c_void);
    fn clockevent_state_periodic(cd: *mut ClockEventDevice) -> bool;
    fn clockevent_set_clock(cd: *mut ClockEventDevice, freq: u64);
    fn clockevent_delta2ns(delta: u64, cd: *mut ClockEventDevice) -> u64;
    fn clockevents_register_device(cd: *mut ClockEventDevice);
    fn cpumask_of(cpu: UInt) -> *const c_void;
    fn bcm1480_mask_irq(cpu: UInt, irq: UInt);
    fn bcm1480_unmask_irq(cpu: UInt, irq: UInt);
    fn irq_set_affinity(irq: UInt, mask: *const c_void);
    fn request_irq(irq: UInt, handler: unsafe extern "C" fn(CInt, *mut c_void) -> IrqReturnT,
                   flags: ULong, name: *const u8, dev_id: *mut c_void) -> CInt;
    fn pr_err(fmt: *const u8, ...);
}

unsafe extern "C" fn sibyte_set_periodic(_evt: *mut ClockEventDevice) -> CInt {
    let cpu = smp_processor_id();
    let cfg = ioaddr(a_scd_timer_register(cpu, R_SCD_TIMER_CFG));
    let init = ioaddr(a_scd_timer_register(cpu, R_SCD_TIMER_INIT));
    __raw_writeq(0, cfg);
    __raw_writeq((V_SCD_TIMER_FREQ / HZ) - 1, init);
    __raw_writeq(M_SCD_TIMER_ENABLE | M_SCD_TIMER_MODE_CONTINUOUS, cfg);
    0
}

unsafe extern "C" fn sibyte_shutdown(_evt: *mut ClockEventDevice) -> CInt {
    let cpu = smp_processor_id();
    let cfg = ioaddr(a_scd_timer_register(cpu, R_SCD_TIMER_CFG));
    // Stop the timer until we actually program a shot
    __raw_writeq(0, cfg);
    0
}

unsafe extern "C" fn sibyte_next_event(delta: ULong, _cd: *mut ClockEventDevice) -> CInt {
    let cpu = smp_processor_id();
    let cfg = ioaddr(a_scd_timer_register(cpu, R_SCD_TIMER_CFG));
    let init = ioaddr(a_scd_timer_register(cpu, R_SCD_TIMER_INIT));
    __raw_writeq(0, cfg);
    __raw_writeq((delta as u64).wrapping_sub(1), init);
    __raw_writeq(M_SCD_TIMER_ENABLE, cfg);
    0
}

unsafe extern "C" fn sibyte_counter_handler(irq: CInt, dev_id: *mut c_void) -> IrqReturnT {
    let _ = irq;
    let cpu = smp_processor_id();
    let cd = dev_id as *mut ClockEventDevice;
    let tmode = if clockevent_state_periodic(cd) {
        M_SCD_TIMER_ENABLE | M_SCD_TIMER_MODE_CONTINUOUS
    } else { 0 };
    // ACK interrupt
    let cfg = ioaddr(a_scd_timer_register(cpu, R_SCD_TIMER_CFG));
    ____raw_writeq(tmode, cfg);
    if let Some(handler) = (*cd).event_handler { handler(cd); }
    IRQ_HANDLED
}

#[no_mangle]
pub unsafe extern "C" fn sb1480_clockevent_init() {
    let cpu = smp_processor_id();
    let irq = K_BCM1480_INT_TIMER_0 + cpu;
    let cd = &mut sibyte_hpt_clockevent[cpu as usize] as *mut ClockEventDevice;
    let name = sibyte_hpt_name[cpu as usize].as_mut_ptr();
    let flags: ULong = IRQF_PERCPU | IRQF_TIMER;
    if cpu > 3 { panic!("BUG_ON(cpu > 3)"); }
    let prefix = b"bcm1480-counter-";
    name[..prefix.len()].copy_from_slice(prefix);
    name[prefix.len()] = b'0' + (cpu as u8);
    name[prefix.len() + 1] = 0;
    (*cd).name = name;
    (*cd).features = CLOCK_EVT_FEAT_PERIODIC | CLOCK_EVT_FEAT_ONESHOT;
    clockevent_set_clock(cd, V_SCD_TIMER_FREQ);
    (*cd).max_delta_ns = clockevent_delta2ns(0x7fffff, cd);
    (*cd).max_delta_ticks = 0x7fffff;
    (*cd).min_delta_ns = clockevent_delta2ns(2, cd);
    (*cd).min_delta_ticks = 2;
    (*cd).rating = 200;
    (*cd).irq = irq;
    (*cd).cpumask = cpumask_of(cpu);
    (*cd).set_next_event = Some(sibyte_next_event);
    (*cd).set_state_shutdown = Some(sibyte_shutdown);
    (*cd).set_state_periodic = Some(sibyte_set_periodic);
    (*cd).set_state_oneshot = Some(sibyte_shutdown);
    clockevents_register_device(cd);
    bcm1480_mask_irq(cpu, irq);
    __raw_writeq(IMR_IP4_VAL, ioaddr(a_bcm1480_imr_register(cpu, R_BCM1480_IMR_INTERRUPT_MAP_BASE_H) + (irq as ULong * 8)));
    bcm1480_unmask_irq(cpu, irq);
    irq_set_affinity(irq, cpumask_of(cpu));
    if request_irq(irq, sibyte_counter_handler, flags, name, cd as *mut c_void) != 0 {
        pr_err(b"Failed to request irq %d (%s)\0".as_ptr(), irq as CInt, name);
    }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
