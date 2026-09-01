/* SPDX-License-Identifier: GPL-2.0 */

/* Translated from perf/util/perf_api_probe.h. */

unsafe extern "C" {
    pub fn perf_can_aux_sample() -> bool;
    pub fn perf_can_comm_exec() -> bool;
    pub fn perf_can_record_cpu_wide() -> bool;
    pub fn perf_can_record_switch_events() -> bool;
    pub fn perf_can_record_text_poke_events() -> bool;
    pub fn perf_can_sample_identifier() -> bool;
    pub fn perf_can_record_build_id() -> bool;
    pub fn perf_can_record_cgroup() -> bool;
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
