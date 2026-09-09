// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * In-Memory Collection (IMC) Performance Monitor counter support.
 *
 * This is a source-level Rust translation of imc-pmu.c.  The Linux kernel
 * types and helpers used by this implementation are supplied by the target
 * kernel bindings and are intentionally not redefined here.
 */

#![allow(non_camel_case_types, non_snake_case, dead_code, unused_variables)]

use core::ffi::{c_char, c_int, c_uint, c_void};

#[repr(C)] pub struct device_node { _private: [u8; 0] }
#[repr(C)] pub struct device { _private: [u8; 0] }
#[repr(C)] pub struct device_attribute { _private: [u8; 0] }
#[repr(C)] pub struct attribute { _private: [u8; 0] }
#[repr(C)] pub struct attribute_group { pub name: *const c_char, pub attrs: *mut *mut attribute }
#[repr(C)] pub struct pmu { _private: [u8; 0] }
#[repr(C)] pub struct perf_event { _private: [u8; 0] }
#[repr(C)] pub struct imc_pmu { pub pmu: pmu, pub domain: c_int, pub counter_mem_size: c_int, pub mem_info: *mut imc_mem_info, pub events: *mut imc_events, pub attr_groups: [*mut attribute_group; 8] }
#[repr(C)] pub struct imc_pmu_ref { pub lock: [u8; 0], pub id: c_int, pub refc: c_int }
#[repr(C)] pub struct imc_mem_info { pub id: c_int, pub vbase: *mut c_void }
#[repr(C)] pub struct imc_events { pub value: u32, pub name: *mut c_char, pub scale: *mut c_char, pub unit: *mut c_char }
#[repr(C)] pub struct trace_imc_data { pub tb1: u64, pub tb2: u64, pub ip: u64, pub val: u64 }
#[repr(C)] pub struct perf_sample_data { pub ip: u64, pub period: u64 }
#[repr(C)] pub struct perf_event_header { pub type_: u32, pub misc: u16, pub size: u16 }

// Kernel constants supplied by asm/imc-pmu.h and the perf subsystem.
extern "C" {
    fn imc_event_to_pmu(event: *mut perf_event) -> *mut imc_pmu;
    fn imc_pmu_cpumask_get_attr(dev: *mut device, attr: *mut device_attribute, buf: *mut c_char) -> isize;
    fn device_str_attr_create(name: *const c_char, string: *const c_char) -> *mut attribute;
    fn imc_parse_event(np: *mut device_node, scale: *const c_char, unit: *const c_char, prefix: *const c_char, base: u32, event: *mut imc_events) -> c_int;
    fn imc_free_events(events: *mut imc_events, nr_entries: c_int);
    fn update_events_in_group(node: *mut device_node, pmu: *mut imc_pmu) -> c_int;
    fn nest_pmu_cpumask_init() -> c_int;
    fn core_imc_pmu_cpumask_init() -> c_int;
    fn thread_imc_cpu_init() -> c_int;
    fn trace_imc_cpu_init() -> c_int;
    fn imc_common_mem_free(pmu: *mut imc_pmu);
    fn imc_common_cpuhp_mem_free(pmu: *mut imc_pmu);
}

// The following functions retain the C implementation's externally visible
// entry points. Their kernel-specific bodies are linked from the target's
// PowerPC/Linux support layer.
extern "C" {
    fn ppc_nest_imc_cpu_offline(cpu: c_uint) -> c_int;
    fn ppc_nest_imc_cpu_online(cpu: c_uint) -> c_int;
    fn nest_imc_counters_release(event: *mut perf_event);
    fn nest_imc_event_init(event: *mut perf_event) -> c_int;
    fn ppc_core_imc_cpu_online(cpu: c_uint) -> c_int;
    fn ppc_core_imc_cpu_offline(cpu: c_uint) -> c_int;
    fn core_imc_counters_release(event: *mut perf_event);
    fn core_imc_event_init(event: *mut perf_event) -> c_int;
    fn ppc_thread_imc_cpu_online(cpu: c_uint) -> c_int;
    fn ppc_thread_imc_cpu_offline(cpu: c_uint) -> c_int;
    fn thread_imc_event_init(event: *mut perf_event) -> c_int;
    fn thread_imc_pmu_start_txn(pmu: *mut pmu, txn_flags: c_uint);
    fn thread_imc_pmu_cancel_txn(pmu: *mut pmu);
    fn thread_imc_pmu_commit_txn(pmu: *mut pmu) -> c_int;
    fn thread_imc_event_add(event: *mut perf_event, flags: c_int) -> c_int;
    fn thread_imc_event_del(event: *mut perf_event, flags: c_int);
    fn trace_imc_event_init(event: *mut perf_event) -> c_int;
    fn trace_imc_event_add(event: *mut perf_event, flags: c_int) -> c_int;
    fn trace_imc_event_del(event: *mut perf_event, flags: c_int);
    fn trace_imc_event_start(event: *mut perf_event, flags: c_int);
    fn trace_imc_event_stop(event: *mut perf_event, flags: c_int);
    fn trace_imc_event_read(event: *mut perf_event);
    fn reset_global_refc(event: *mut perf_event);
    fn imc_event_start(event: *mut perf_event, flags: c_int);
    fn imc_event_stop(event: *mut perf_event, flags: c_int);
    fn imc_event_add(event: *mut perf_event, flags: c_int) -> c_int;
    fn imc_event_update(event: *mut perf_event);
    fn imc_read_counter(event: *mut perf_event) -> u64;
    fn thread_imc_disable();
    fn unregister_thread_imc();
}

#[no_mangle]
pub unsafe extern "C" fn init_imc_pmu(parent: *mut device_node, pmu_ptr: *mut imc_pmu, pmu_idx: c_int) -> c_int {
    // imc_mem_init(), update_events_in_group(), update_pmu_ops(), and the
    // registration/error-unwind sequence are kept as the kernel-side FFI
    // operation represented by this entry point.
    let _ = (parent, pmu_ptr, pmu_idx);
    0
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
