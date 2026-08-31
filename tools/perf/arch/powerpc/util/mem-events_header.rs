/* SPDX-License-Identifier: GPL-2.0 */

unsafe extern "C" {
    pub static mut perf_mem_events_power: [perf_mem_event; PERF_MEM_EVENTS__MAX];
}
