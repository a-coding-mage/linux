// SPDX-License-Identifier: GPL-2.0
//
// C source included:
//   "util/map_symbol.h"
//   "util/mem-events.h"
//   "mem-events.h"

use core::ffi::c_char;
use core::ptr;

pub const PERF_MEM_EVENTS__MAX: usize = 3;

#[repr(C)]
pub struct perf_mem_event {
    pub tag: *const c_char,
    pub name: *const c_char,
    pub event_name: *const c_char,
    pub ldlat: bool,
    pub aux_event: *const c_char,
}

// C macro:
// #define E(t, n, s, l, a) { .tag = t, .name = n, .event_name = s, .ldlat = l, .aux_event = a }

#[no_mangle]
pub static mut perf_mem_events_power: [perf_mem_event; PERF_MEM_EVENTS__MAX] = [
    perf_mem_event {
        tag: b"ldlat-loads\0".as_ptr() as *const c_char,
        name: b"%s/mem-loads/\0".as_ptr() as *const c_char,
        event_name: b"mem-loads\0".as_ptr() as *const c_char,
        ldlat: false,
        aux_event: ptr::null(),
    },
    perf_mem_event {
        tag: b"ldlat-stores\0".as_ptr() as *const c_char,
        name: b"%s/mem-stores/\0".as_ptr() as *const c_char,
        event_name: b"mem-stores\0".as_ptr() as *const c_char,
        ldlat: false,
        aux_event: ptr::null(),
    },
    perf_mem_event {
        tag: ptr::null(),
        name: ptr::null(),
        event_name: ptr::null(),
        ldlat: false,
        aux_event: ptr::null(),
    },
];
