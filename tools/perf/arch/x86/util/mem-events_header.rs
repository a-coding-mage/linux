/* SPDX-License-Identifier: GPL-2.0 */

/* Dependency intent from C header:
 * extern struct perf_mem_event ...[PERF_MEM_EVENTS__MAX];
 */

unsafe extern "C" {
    pub static mut perf_mem_events_intel: [perf_mem_event; PERF_MEM_EVENTS__MAX];
    pub static mut perf_mem_events_intel_aux: [perf_mem_event; PERF_MEM_EVENTS__MAX];

    pub static mut perf_mem_events_amd: [perf_mem_event; PERF_MEM_EVENTS__MAX];
    pub static mut perf_mem_events_amd_ldlat: [perf_mem_event; PERF_MEM_EVENTS__MAX];
}
