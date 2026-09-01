/* SPDX-License-Identifier: GPL-2.0 */
/*
 * HiSilicon PCIe Trace and Tuning (PTT) support
 * Copyright (c) 2022 HiSilicon Technologies Co., Ltd.
 */

pub const HISI_PTT_PMU_NAME: &str = "hisi_ptt";
pub const HISI_PTT_AUXTRACE_PRIV_SIZE: usize = core::mem::size_of::<u64>();

unsafe extern "C" {
    pub fn hisi_ptt_recording_init(
        err: *mut ::std::os::raw::c_int,
        hisi_ptt_pmu: *mut perf_pmu,
    ) -> *mut auxtrace_record;

    pub fn hisi_ptt_process_auxtrace_info(
        event: *mut perf_event,
        session: *mut perf_session,
    ) -> ::std::os::raw::c_int;
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
