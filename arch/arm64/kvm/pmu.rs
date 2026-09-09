// SPDX-License-Identifier: GPL-2.0
/*
 * Copyright 2019 Arm Limited
 * Author: Andrew Murray <Andrew.Murray@arm.com>
 */

use core::ffi::c_int;

// Declarations supplied by the surrounding kernel translation.
#[repr(C)]
pub struct perf_event_attr {
    pub exclude_user: bool,
    pub exclude_host: bool,
    pub exclude_guest: bool,
}
#[repr(C)]
pub struct kvm_pmu_events { pub events_host: u64, pub events_guest: u64 }
#[repr(C)] pub struct kvm_vcpu;
#[repr(C)] pub struct kvm_cpu_context;
#[repr(C)] pub struct host_ctxt;

extern "C" {
    fn has_vhe() -> bool;
    fn system_supports_pmuv3() -> bool;
    fn this_cpu_ptr(events: *mut kvm_pmu_events) -> *mut kvm_pmu_events;
    fn preempt_disable();
    fn preempt_enable();
    fn read_pmccfiltr() -> u64;
    fn read_pmicfiltr() -> u64;
    fn read_pmevtypern(idx: c_int) -> u64;
    fn write_pmccfiltr(val: u32);
    fn write_pmicfiltr(val: u32);
    fn write_pmevtypern(idx: c_int, val: u32);
    fn kvm_get_running_vcpu() -> *mut kvm_vcpu;
    fn vcpu_get_flag(vcpu: *mut kvm_vcpu, flag: c_int) -> bool;
    fn host_data_ptr(host_ctxt: *mut host_ctxt) -> *mut kvm_cpu_context;
    fn ctxt_sys_reg(ctxt: *mut kvm_cpu_context, reg: c_int) -> *mut u64;
    fn in_interrupt() -> bool;
    fn kvm_make_request(req: c_int, vcpu: *mut kvm_vcpu);
}

extern "C" {
    static mut host_ctxt: *mut host_ctxt;
}

const ARMV8_PMU_CYCLE_IDX: c_int = 31;
const ARMV8_PMU_INSTR_IDX: c_int = 32;
const ARMPMU_MAX_HWEVENTS: u32 = 33;
const ARMV8_PMU_EXCLUDE_EL0: u64 = 1 << 30;
const PMUSERENR_ON_CPU: c_int = 0;
const PMUSERENR_EL0: c_int = 0;
const KVM_REQ_RESYNC_PMU_EL0: c_int = 0;

static mut kvm_pmu_events: kvm_pmu_events = kvm_pmu_events { events_host: 0, events_guest: 0 };

/* Given the perf event attributes and system type, determine
 * if we are going to need to switch counters at guest entry/exit. */
unsafe fn kvm_pmu_switch_needed(attr: *mut perf_event_attr) -> bool {
    /* With VHE the guest kernel runs at EL1 and the host at EL2,
     * where user (EL0) is excluded then we have no reason to switch counters. */
    if has_vhe() && (*attr).exclude_user { return false; }
    (*attr).exclude_host != (*attr).exclude_guest
}

pub unsafe fn kvm_get_pmu_events() -> *mut kvm_pmu_events {
    this_cpu_ptr(&mut kvm_pmu_events)
}

pub unsafe fn kvm_set_pmu_events(set: u64, attr: *mut perf_event_attr) {
    let pmu = kvm_get_pmu_events();
    if !system_supports_pmuv3() || !kvm_pmu_switch_needed(attr) { return; }
    if !(*attr).exclude_host { (*pmu).events_host |= set; }
    if !(*attr).exclude_guest { (*pmu).events_guest |= set; }
}

pub unsafe fn kvm_clr_pmu_events(clr: u64) {
    let pmu = kvm_get_pmu_events();
    if !system_supports_pmuv3() { return; }
    (*pmu).events_host &= !clr;
    (*pmu).events_guest &= !clr;
}

unsafe fn kvm_vcpu_pmu_read_evtype_direct(idx: c_int) -> u64 {
    if idx == ARMV8_PMU_CYCLE_IDX { read_pmccfiltr() }
    else if idx == ARMV8_PMU_INSTR_IDX { read_pmicfiltr() }
    else { read_pmevtypern(idx) }
}

unsafe fn kvm_vcpu_pmu_write_evtype_direct(idx: c_int, val: u32) {
    if idx == ARMV8_PMU_CYCLE_IDX { write_pmccfiltr(val); }
    else if idx == ARMV8_PMU_INSTR_IDX { write_pmicfiltr(val); }
    else { write_pmevtypern(idx, val); }
}

unsafe fn kvm_vcpu_pmu_enable_el0(mut events: u64) {
    for counter in 0..ARMPMU_MAX_HWEVENTS {
        if events & (1u64 << counter) != 0 {
            let mut typer = kvm_vcpu_pmu_read_evtype_direct(counter as c_int);
            typer &= !ARMV8_PMU_EXCLUDE_EL0;
            kvm_vcpu_pmu_write_evtype_direct(counter as c_int, typer as u32);
        }
    }
}

unsafe fn kvm_vcpu_pmu_disable_el0(events: u64) {
    for counter in 0..ARMPMU_MAX_HWEVENTS {
        if events & (1u64 << counter) != 0 {
            let mut typer = kvm_vcpu_pmu_read_evtype_direct(counter as c_int);
            typer |= ARMV8_PMU_EXCLUDE_EL0;
            kvm_vcpu_pmu_write_evtype_direct(counter as c_int, typer as u32);
        }
    }
}

pub unsafe fn kvm_vcpu_pmu_restore_guest(_vcpu: *mut kvm_vcpu) {
    if !system_supports_pmuv3() || !has_vhe() { return; }
    preempt_disable();
    let pmu = kvm_get_pmu_events();
    let events_guest = (*pmu).events_guest;
    let events_host = (*pmu).events_host;
    kvm_vcpu_pmu_enable_el0(events_guest);
    kvm_vcpu_pmu_disable_el0(events_host);
    preempt_enable();
}

pub unsafe fn kvm_vcpu_pmu_restore_host(_vcpu: *mut kvm_vcpu) {
    if !system_supports_pmuv3() || !has_vhe() { return; }
    let pmu = kvm_get_pmu_events();
    let events_guest = (*pmu).events_guest;
    let events_host = (*pmu).events_host;
    kvm_vcpu_pmu_enable_el0(events_host);
    kvm_vcpu_pmu_disable_el0(events_guest);
}

pub unsafe fn kvm_set_pmuserenr(val: u64) -> bool {
    if !system_supports_pmuv3() || !has_vhe() { return false; }
    let vcpu = kvm_get_running_vcpu();
    if vcpu.is_null() || !vcpu_get_flag(vcpu, PMUSERENR_ON_CPU) { return false; }
    let hctxt = host_data_ptr(host_ctxt);
    *ctxt_sys_reg(hctxt, PMUSERENR_EL0) = val;
    true
}

pub unsafe fn kvm_vcpu_pmu_resync_el0() {
    if !has_vhe() || !in_interrupt() { return; }
    let vcpu = kvm_get_running_vcpu();
    if vcpu.is_null() { return; }
    kvm_make_request(KVM_REQ_RESYNC_PMU_EL0, vcpu);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
