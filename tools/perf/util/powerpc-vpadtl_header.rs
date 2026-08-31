/* SPDX-License-Identifier: GPL-2.0 */
/*
 * VPA DTL PMU Support
 */

// Header guard and C includes are omitted in Rust. This header depends on the
// external C definitions of u64, union perf_event, struct perf_session, and
// struct perf_pmu.

pub const POWERPC_VPADTL_TYPE: ::std::os::raw::c_uint = 0;
pub const VPADTL_AUXTRACE_PRIV_MAX: ::std::os::raw::c_uint = 1;

pub const VPADTL_AUXTRACE_PRIV_SIZE: usize =
    (VPADTL_AUXTRACE_PRIV_MAX as usize) * ::std::mem::size_of::<u64>();

unsafe extern "C" {
    pub type perf_event;
    pub type perf_session;
    pub type perf_pmu;

    pub fn powerpc_vpadtl_process_auxtrace_info(
        event: *mut perf_event,
        session: *mut perf_session,
    ) -> ::std::os::raw::c_int;
}
