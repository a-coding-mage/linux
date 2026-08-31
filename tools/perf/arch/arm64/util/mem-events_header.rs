/* SPDX-License-Identifier: GPL-2.0 */
/* Header guard _ARM64_MEM_EVENTS_H omitted in Rust. */

unsafe extern "C" {
    pub static mut perf_mem_events_arm: [perf_mem_event; PERF_MEM_EVENTS__MAX];
}
