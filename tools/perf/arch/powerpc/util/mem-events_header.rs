/* SPDX-License-Identifier: GPL-2.0 */

unsafe extern "C" {
    pub static mut perf_mem_events_power: [perf_mem_event; PERF_MEM_EVENTS__MAX];
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
