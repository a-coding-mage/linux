/* SPDX-License-Identifier: GPL-2.0 */

#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum summary_field {
    SUMMARY_CURRENT,
    SUMMARY_MIN,
    SUMMARY_MAX,
    SUMMARY_COUNT,
    SUMMARY_SUM,
    SUMMARY_OVERFLOW,
    SUMMARY_FIELD_N,
}

/*
 * C preprocessor intent:
 *   #ifndef __bpf__
 *   #ifdef HAVE_BPF_SKEL
 *
 * The declarations below are exposed for non-BPF builds when BPF skeleton
 * support is available. The fallback inline definitions preserve the no-libbpf
 * behavior from the header.
 */

#[cfg(all(not(__bpf__), HAVE_BPF_SKEL))]
extern "C" {
    pub fn timerlat_bpf_init(params: *mut timerlat_params) -> ::std::os::raw::c_int;
    pub fn timerlat_bpf_attach() -> ::std::os::raw::c_int;
    pub fn timerlat_bpf_detach();
    pub fn timerlat_bpf_destroy();
    pub fn timerlat_bpf_wait(timeout: ::std::os::raw::c_int) -> ::std::os::raw::c_int;
    pub fn timerlat_bpf_restart_tracing() -> ::std::os::raw::c_int;
    pub fn timerlat_bpf_get_hist_value(
        key: ::std::os::raw::c_int,
        value_irq: *mut ::std::os::raw::c_longlong,
        value_thread: *mut ::std::os::raw::c_longlong,
        value_user: *mut ::std::os::raw::c_longlong,
    ) -> ::std::os::raw::c_int;
    pub fn timerlat_bpf_get_summary_value(
        key: summary_field,
        value_irq: *mut ::std::os::raw::c_longlong,
        value_thread: *mut ::std::os::raw::c_longlong,
        value_user: *mut ::std::os::raw::c_longlong,
    ) -> ::std::os::raw::c_int;
    pub fn timerlat_load_bpf_action_program(
        program_path: *const ::std::os::raw::c_char,
    ) -> ::std::os::raw::c_int;
}

#[cfg(all(not(__bpf__), HAVE_BPF_SKEL))]
#[inline]
pub fn have_libbpf_support() -> ::std::os::raw::c_int {
    1
}

#[cfg(all(not(__bpf__), not(HAVE_BPF_SKEL)))]
#[inline]
pub unsafe fn timerlat_bpf_init(_params: *mut timerlat_params) -> ::std::os::raw::c_int {
    -1
}

#[cfg(all(not(__bpf__), not(HAVE_BPF_SKEL)))]
#[inline]
pub fn timerlat_bpf_attach() -> ::std::os::raw::c_int {
    -1
}

#[cfg(all(not(__bpf__), not(HAVE_BPF_SKEL)))]
#[inline]
pub fn timerlat_bpf_detach() {}

#[cfg(all(not(__bpf__), not(HAVE_BPF_SKEL)))]
#[inline]
pub fn timerlat_bpf_destroy() {}

#[cfg(all(not(__bpf__), not(HAVE_BPF_SKEL)))]
#[inline]
pub fn timerlat_bpf_wait(_timeout: ::std::os::raw::c_int) -> ::std::os::raw::c_int {
    -1
}

#[cfg(all(not(__bpf__), not(HAVE_BPF_SKEL)))]
#[inline]
pub fn timerlat_bpf_restart_tracing() -> ::std::os::raw::c_int {
    -1
}

#[cfg(all(not(__bpf__), not(HAVE_BPF_SKEL)))]
#[inline]
pub unsafe fn timerlat_bpf_get_hist_value(
    _key: ::std::os::raw::c_int,
    _value_irq: *mut ::std::os::raw::c_longlong,
    _value_thread: *mut ::std::os::raw::c_longlong,
    _value_user: *mut ::std::os::raw::c_longlong,
) -> ::std::os::raw::c_int {
    -1
}

#[cfg(all(not(__bpf__), not(HAVE_BPF_SKEL)))]
#[inline]
pub unsafe fn timerlat_bpf_get_summary_value(
    _key: summary_field,
    _value_irq: *mut ::std::os::raw::c_longlong,
    _value_thread: *mut ::std::os::raw::c_longlong,
    _value_user: *mut ::std::os::raw::c_longlong,
) -> ::std::os::raw::c_int {
    -1
}

#[cfg(all(not(__bpf__), not(HAVE_BPF_SKEL)))]
#[inline]
pub unsafe fn timerlat_load_bpf_action_program(
    _program_path: *const ::std::os::raw::c_char,
) -> ::std::os::raw::c_int {
    -1
}

#[cfg(all(not(__bpf__), not(HAVE_BPF_SKEL)))]
#[inline]
pub fn have_libbpf_support() -> ::std::os::raw::c_int {
    0
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
