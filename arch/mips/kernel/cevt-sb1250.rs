// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * Copyright (C) 2000, 2001 Broadcom Corporation
 */

// The declarations supplied by the Linux and MIPS headers are intentionally
// left as external dependencies of this translation.

use core::ffi::c_void;

const IMR_IP2_VAL: u64 = K_INT_MAP_I0;
const IMR_IP3_VAL: u64 = K_INT_MAP_I1;
const IMR_IP4_VAL: u64 = K_INT_MAP_I2;

extern "C" {
    static K_INT_MAP_I0: u64;
    static K_INT_MAP_I1: u64;
    static K_INT_MAP_I2: u64;
    static V_SCD_TIMER_FREQ: u64;
    static HZ: u64;
    static M_SCD_TIMER_ENABLE: u64;
    static M_SCD_TIMER_MODE_CONTINUOUS: u64;

    fn smp_processor_id() -> u32;
    fn IOADDR(address: u64) -> *mut c_void;
    fn A_SCD_TIMER_REGISTER(cpu: u32, reg: u64) -> u64;
    fn A_IMR_REGISTER(cpu: u32, reg: u64) -> u64;
    static R_SCD_TIMER_CFG: u64;
    static R_SCD_TIMER_INIT: u64;
    static R_IMR_INTERRUPT_MAP_BASE: u64;

    fn __raw_writeq(value: u64, address: *mut c_void);
    fn ____raw_writeq(value: u64, address: *mut c_void);
    fn clockevent_state_periodic(evt: *mut clock_event_device) -> bool;
    fn clockevent_set_clock(evt: *mut clock_event_device, frequency: u64);
    fn clockevent_delta2ns(delta: u64, evt: *mut clock_event_device) -> u64;
    fn clockevents_register_device(evt: *mut clock_event_device);
    fn sb1250_mask_irq(cpu: u32, irq: u32);
    fn sb1250_unmask_irq(cpu: u32, irq: u32);
    fn cpumask_of(cpu: u32) -> *mut c_void;
    fn irq_set_affinity(irq: u32, mask: *mut c_void);
    fn request_irq(irq: u32, handler: unsafe extern "C" fn(i32, *mut c_void) -> irqreturn_t,
                   flags: u64, name: *mut u8, dev_id: *mut c_void) -> i32;
    fn pr_err(format: *const u8, ...);
    fn sprintf(buffer: *mut u8, format: *const u8, ...);
}

type irqreturn_t = i32;
const IRQ_HANDLED: irqreturn_t = 1;
const IRQF_PERCPU: u64 = 0;
const IRQF_TIMER: u64 = 0;
const CLOCK_EVT_FEAT_PERIODIC: u64 = 1;
const CLOCK_EVT_FEAT_ONESHOT: u64 = 2;
const K_INT_TIMER_0: u32 = 0;

#[repr(C)]
pub struct clock_event_device {
    pub name: *mut u8,
    pub features: u64,
    pub max_delta_ns: u64,
    pub max_delta_ticks: u64,
    pub min_delta_ns: u64,
    pub min_delta_ticks: u64,
    pub rating: i32,
    pub irq: u32,
    pub cpumask: *mut c_void,
    pub set_next_event: Option<unsafe extern "C" fn(u64, *mut clock_event_device) -> i32>,
    pub set_state_shutdown: Option<unsafe extern "C" fn(*mut clock_event_device) -> i32>,
    pub set_state_periodic: Option<unsafe extern "C" fn(*mut clock_event_device) -> i32>,
    pub set_state_oneshot: Option<unsafe extern "C" fn(*mut clock_event_device) -> i32>,
    pub event_handler: unsafe extern "C" fn(*mut clock_event_device),
}

static mut SIBYTE_HPT_CLOCKEVENT: [clock_event_device; 4] = unsafe { core::mem::zeroed() };
static mut SIBYTE_HPT_NAME: [[u8; 18]; 4] = [[0; 18]; 4];

unsafe extern "C" fn sibyte_shutdown(_evt: *mut clock_event_device) -> i32 {
    let cfg = IOADDR(A_SCD_TIMER_REGISTER(smp_processor_id(), R_SCD_TIMER_CFG));
    __raw_writeq(0, cfg);
    0
}

unsafe extern "C" fn sibyte_set_periodic(_evt: *mut clock_event_device) -> i32 {
    let cpu = smp_processor_id();
    let cfg = IOADDR(A_SCD_TIMER_REGISTER(cpu, R_SCD_TIMER_CFG));
    let init = IOADDR(A_SCD_TIMER_REGISTER(cpu, R_SCD_TIMER_INIT));
    __raw_writeq(0, cfg);
    __raw_writeq((V_SCD_TIMER_FREQ / HZ) - 1, init);
    __raw_writeq(M_SCD_TIMER_ENABLE | M_SCD_TIMER_MODE_CONTINUOUS, cfg);
    0
}

unsafe extern "C" fn sibyte_next_event(delta: u64, _cd: *mut clock_event_device) -> i32 {
    let cpu = smp_processor_id();
    let cfg = IOADDR(A_SCD_TIMER_REGISTER(cpu, R_SCD_TIMER_CFG));
    let init = IOADDR(A_SCD_TIMER_REGISTER(cpu, R_SCD_TIMER_INIT));
    __raw_writeq(0, cfg);
    __raw_writeq(delta - 1, init);
    __raw_writeq(M_SCD_TIMER_ENABLE, cfg);
    0
}

unsafe extern "C" fn sibyte_counter_handler(_irq: i32, dev_id: *mut c_void) -> irqreturn_t {
    let cpu = smp_processor_id();
    let cd = dev_id as *mut clock_event_device;
    let tmode = if clockevent_state_periodic(cd) {
        M_SCD_TIMER_ENABLE | M_SCD_TIMER_MODE_CONTINUOUS
    } else { 0 };
    let cfg = IOADDR(A_SCD_TIMER_REGISTER(cpu, R_SCD_TIMER_CFG));
    ____raw_writeq(tmode, cfg);
    ((*cd).event_handler)(cd);
    IRQ_HANDLED
}

pub unsafe extern "C" fn sb1250_clockevent_init() {
    let cpu = smp_processor_id();
    let irq = K_INT_TIMER_0 + cpu;
    let cd = &mut SIBYTE_HPT_CLOCKEVENT[cpu as usize] as *mut clock_event_device;
    let name = SIBYTE_HPT_NAME[cpu as usize].as_mut_ptr();
    let flags = IRQF_PERCPU | IRQF_TIMER;
    assert!(cpu <= 2);
    sprintf(name, b"sb1250-counter-%d\0".as_ptr(), cpu);
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
    sb1250_mask_irq(cpu, irq);
    __raw_writeq(IMR_IP4_VAL, IOADDR(A_IMR_REGISTER(cpu, R_IMR_INTERRUPT_MAP_BASE) + ((irq as u64) << 3)));
    sb1250_unmask_irq(cpu, irq);
    irq_set_affinity(irq, cpumask_of(cpu));
    if request_irq(irq, sibyte_counter_handler, flags, name, cd as *mut c_void) != 0 {
        pr_err(b"Failed to request irq %d (%s)\n\0".as_ptr(), irq, name);
    }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
