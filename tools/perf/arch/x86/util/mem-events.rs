// SPDX-License-Identifier: GPL-2.0
// Translated from C implementation source:
// includes: "linux/string.h", "util/map_symbol.h", "util/mem-events.h", "mem-events.h"

pub const MEM_LOADS_AUX: u64 = 0x8203;

// Equivalent of C initializer macro:
// #define E(t, n, s, l, a) { .tag = t, .name = n, .event_name = s, .ldlat = l, .aux_event = a }

#[no_mangle]
pub static mut perf_mem_events_intel: [perf_mem_event; PERF_MEM_EVENTS__MAX as usize] = [
    perf_mem_event {
        tag: c"ldlat-loads".as_ptr(),
        name: c"%s/mem-loads,ldlat=%u/P".as_ptr(),
        event_name: c"mem-loads".as_ptr(),
        ldlat: true,
        aux_event: 0,
    },
    perf_mem_event {
        tag: c"ldlat-stores".as_ptr(),
        name: c"%s/mem-stores/P".as_ptr(),
        event_name: c"mem-stores".as_ptr(),
        ldlat: false,
        aux_event: 0,
    },
    perf_mem_event {
        tag: core::ptr::null(),
        name: core::ptr::null(),
        event_name: core::ptr::null(),
        ldlat: false,
        aux_event: 0,
    },
];

#[no_mangle]
pub static mut perf_mem_events_intel_aux: [perf_mem_event; PERF_MEM_EVENTS__MAX as usize] = [
    perf_mem_event {
        tag: c"ldlat-loads".as_ptr(),
        name: c"{%s/mem-loads-aux/,%s/mem-loads,ldlat=%u/}:P".as_ptr(),
        event_name: c"mem-loads".as_ptr(),
        ldlat: true,
        aux_event: MEM_LOADS_AUX,
    },
    perf_mem_event {
        tag: c"ldlat-stores".as_ptr(),
        name: c"%s/mem-stores/P".as_ptr(),
        event_name: c"mem-stores".as_ptr(),
        ldlat: false,
        aux_event: 0,
    },
    perf_mem_event {
        tag: core::ptr::null(),
        name: core::ptr::null(),
        event_name: core::ptr::null(),
        ldlat: false,
        aux_event: 0,
    },
];

#[no_mangle]
pub static mut perf_mem_events_amd: [perf_mem_event; PERF_MEM_EVENTS__MAX as usize] = [
    perf_mem_event {
        tag: core::ptr::null(),
        name: core::ptr::null(),
        event_name: core::ptr::null(),
        ldlat: false,
        aux_event: 0,
    },
    perf_mem_event {
        tag: core::ptr::null(),
        name: core::ptr::null(),
        event_name: core::ptr::null(),
        ldlat: false,
        aux_event: 0,
    },
    perf_mem_event {
        tag: c"mem-ldst".as_ptr(),
        name: c"%s//".as_ptr(),
        event_name: core::ptr::null(),
        ldlat: false,
        aux_event: 0,
    },
];

#[no_mangle]
pub static mut perf_mem_events_amd_ldlat: [perf_mem_event; PERF_MEM_EVENTS__MAX as usize] = [
    perf_mem_event {
        tag: core::ptr::null(),
        name: core::ptr::null(),
        event_name: core::ptr::null(),
        ldlat: false,
        aux_event: 0,
    },
    perf_mem_event {
        tag: core::ptr::null(),
        name: core::ptr::null(),
        event_name: core::ptr::null(),
        ldlat: false,
        aux_event: 0,
    },
    perf_mem_event {
        tag: c"mem-ldst".as_ptr(),
        name: c"%s/ldlat=%u/".as_ptr(),
        event_name: core::ptr::null(),
        ldlat: true,
        aux_event: 0,
    },
];

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
