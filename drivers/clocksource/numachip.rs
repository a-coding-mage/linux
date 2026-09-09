// SPDX-License-Identifier: GPL-2.0-only
/*
 *
 * Copyright (C) 2015 Numascale AS. All rights reserved.
 */

// Dependencies supplied by the Linux kernel and architecture headers are
// intentionally left external to this translation unit.

use core::ffi::c_void;

type CyclesT = u64;

#[repr(C)]
pub struct ClockSource {
    pub name: *const u8,
    pub rating: i32,
    pub read: Option<unsafe extern "C" fn(*mut ClockSource) -> CyclesT>,
    pub mask: u64,
    pub flags: u32,
    pub mult: u32,
    pub shift: u32,
}

#[repr(C)]
pub struct ClockEventDevice {
    pub name: *const u8,
    pub rating: i32,
    pub set_next_event: Option<unsafe extern "C" fn(usize, *mut ClockEventDevice) -> i32>,
    pub event_handler: Option<unsafe extern "C" fn(*mut ClockEventDevice)>,
    pub features: u32,
    pub mult: u32,
    pub shift: u32,
    pub min_delta_ns: u64,
    pub min_delta_ticks: u64,
    pub max_delta_ns: i64,
    pub max_delta_ticks: i64,
    pub cpumask: *const c_void,
}

#[repr(C)]
pub struct WorkStruct {
    _private: [u8; 0],
}

extern "C" {
    static mut numachip_system: i32;
    static mut x86_platform_ipi_callback: Option<unsafe extern "C" fn()>;

    fn numachip2_read64_lcsr(reg: u64) -> u64;
    fn numachip2_write64_lcsr(reg: u64, value: u64);
    fn numachip2_timer() -> u64;
    fn this_cpu_ptr<T>(ptr: *mut T) -> *mut T;
    fn __this_cpu_read<T>(value: T) -> T;
    fn cpumask_of(cpu: i32) -> *const c_void;
    fn smp_processor_id() -> i32;
    fn clockevents_register_device(ced: *mut ClockEventDevice);
    fn clocksource_register_hz(cs: *mut ClockSource, hz: u64) -> i32;
    fn schedule_on_each_cpu(work: Option<unsafe extern "C" fn(*mut WorkStruct)>) -> i32;
}

const NUMACHIP2_TIMER_NOW: u64 = 0; // Supplied by asm/numachip/numachip_csr.h
const NUMACHIP2_TIMER_DEADLINE: u64 = 0; // Supplied by asm/numachip/numachip_csr.h
const NUMACHIP2_TIMER_INT: u64 = 0; // Supplied by asm/numachip/numachip_csr.h
const NUMACHIP2_TIMER_RESET: u64 = 0; // Supplied by asm/numachip/numachip_csr.h
const CLOCK_SOURCE_IS_CONTINUOUS: u32 = 1 << 0;
const CLOCK_EVT_FEAT_ONESHOT: u32 = 1 << 0;
const X86_PLATFORM_IPI_VECTOR: u64 = 0; // Supplied by asm/irq.h
const NSEC_PER_SEC: u64 = 1_000_000_000;
const LONG_MAX: i64 = i64::MAX;
const ENODEV: i32 = 19;

static mut NUMACHIP2_CED: ClockEventDevice = ClockEventDevice {
    name: core::ptr::null(),
    rating: 0,
    set_next_event: None,
    event_handler: None,
    features: 0,
    mult: 0,
    shift: 0,
    min_delta_ns: 0,
    min_delta_ticks: 0,
    max_delta_ns: 0,
    max_delta_ticks: 0,
    cpumask: core::ptr::null(),
};

unsafe extern "C" fn numachip2_timer_read(_cs: *mut ClockSource) -> CyclesT {
    numachip2_read64_lcsr(NUMACHIP2_TIMER_NOW)
}

static mut NUMACHIP2_CLOCKSOURCE: ClockSource = ClockSource {
    name: b"numachip2\0".as_ptr(),
    rating: 295,
    read: Some(numachip2_timer_read),
    mask: u64::MAX,
    flags: CLOCK_SOURCE_IS_CONTINUOUS,
    mult: 1,
    shift: 0,
};

unsafe extern "C" fn numachip2_set_next_event(delta: usize, _ced: *mut ClockEventDevice) -> i32 {
    numachip2_write64_lcsr(
        NUMACHIP2_TIMER_DEADLINE.wrapping_add(numachip2_timer()),
        delta as u64,
    );
    0
}

static NUMACHIP2_CLOCKEVENT: ClockEventDevice = ClockEventDevice {
    name: b"numachip2\0".as_ptr(),
    rating: 400,
    set_next_event: Some(numachip2_set_next_event),
    event_handler: None,
    features: CLOCK_EVT_FEAT_ONESHOT,
    mult: 1,
    shift: 0,
    min_delta_ns: 1250,
    min_delta_ticks: 1250,
    max_delta_ns: LONG_MAX,
    max_delta_ticks: LONG_MAX,
    cpumask: core::ptr::null(),
};

unsafe extern "C" fn numachip_timer_interrupt() {
    let ced = this_cpu_ptr(&raw mut NUMACHIP2_CED);
    if let Some(handler) = (*ced).event_handler {
        handler(ced);
    }
}

unsafe extern "C" fn numachip_timer_each(_work: *mut WorkStruct) {
    let local_apicid = (__this_cpu_read(0u32) & 0xff) as u64;
    let ced = this_cpu_ptr(&raw mut NUMACHIP2_CED);

    /* Setup IPI vector to local core and relative timing mode */
    numachip2_write64_lcsr(
        NUMACHIP2_TIMER_INT.wrapping_add(numachip2_timer()),
        (3u64 << 22) | (X86_PLATFORM_IPI_VECTOR << 14) | (local_apicid << 6),
    );

    *ced = NUMACHIP2_CLOCKEVENT;
    (*ced).cpumask = cpumask_of(smp_processor_id());
    clockevents_register_device(ced);
}

unsafe extern "C" fn numachip_timer_init() -> i32 {
    if numachip_system != 2 {
        return -ENODEV;
    }

    /* Reset timer */
    numachip2_write64_lcsr(NUMACHIP2_TIMER_RESET, 0);
    clocksource_register_hz(&raw mut NUMACHIP2_CLOCKSOURCE, NSEC_PER_SEC);

    /* Setup per-cpu clockevents */
    x86_platform_ipi_callback = Some(numachip_timer_interrupt);
    schedule_on_each_cpu(Some(numachip_timer_each));

    0
}

// arch_initcall(numachip_timer_init);

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
