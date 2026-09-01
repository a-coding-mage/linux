// SPDX-License-Identifier: GPL-2.0
// C dependencies: "util/map_symbol.h", "util/mem-events.h", "mem-events.h"
// C initializer macro:
// #define E(t, n, s, l, a) { .tag = t, .name = n, .event_name = s, .ldlat = l, .aux_event = a }

pub static mut perf_mem_events_arm: [perf_mem_event; PERF_MEM_EVENTS__MAX as usize] = [
    perf_mem_event {
        tag: c"spe-load".as_ptr(),
        name: c"%s/ts_enable=1,pa_enable=1,load_filter=1,min_latency=%u/".as_ptr(),
        event_name: core::ptr::null(),
        ldlat: true,
        aux_event: 0,
    },
    perf_mem_event {
        tag: c"spe-store".as_ptr(),
        name: c"%s/ts_enable=1,pa_enable=1,store_filter=1/".as_ptr(),
        event_name: core::ptr::null(),
        ldlat: false,
        aux_event: 0,
    },
    perf_mem_event {
        tag: c"spe-ldst".as_ptr(),
        name: c"%s/ts_enable=1,pa_enable=1,load_filter=1,store_filter=1,min_latency=%u/".as_ptr(),
        event_name: core::ptr::null(),
        ldlat: true,
        aux_event: 0,
    },
];

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
