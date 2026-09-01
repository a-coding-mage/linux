/* SPDX-License-Identifier: GPL-2.0 */

/* Depends on linux/list.h, bpf_skel/sample-filter.h, util/debug.h, and errno.h. */

#[repr(C)]
pub struct perf_bpf_filter_expr {
    pub list: list_head,
    pub groups: list_head,
    pub op: perf_bpf_filter_op,
    pub part: ::std::os::raw::c_int,
    pub term: perf_bpf_filter_term,
    pub val: ::std::os::raw::c_ulong,
}

/* Forward declarations from the C header:
 * struct evsel;
 * struct target;
 */

/* path in BPF-fs for the pinned program and maps */
pub const PERF_BPF_FILTER_PIN_PATH: &[u8; 12] = b"perf_filter\0";

/* C conditional: HAVE_BPF_SKEL */
#[cfg(HAVE_BPF_SKEL)]
unsafe extern "C" {
    pub fn perf_bpf_filter_expr__new(
        term: perf_bpf_filter_term,
        part: ::std::os::raw::c_int,
        op: perf_bpf_filter_op,
        val: ::std::os::raw::c_ulong,
    ) -> *mut perf_bpf_filter_expr;
    pub fn perf_bpf_filter__parse(
        expr_head: *mut list_head,
        str: *const ::std::os::raw::c_char,
    ) -> ::std::os::raw::c_int;
    pub fn perf_bpf_filter__prepare(
        evsel: *mut evsel,
        target: *mut target,
    ) -> ::std::os::raw::c_int;
    pub fn perf_bpf_filter__destroy(evsel: *mut evsel) -> ::std::os::raw::c_int;
    pub fn perf_bpf_filter__lost_count(evsel: *mut evsel) -> u64;
    pub fn perf_bpf_filter__pin() -> ::std::os::raw::c_int;
    pub fn perf_bpf_filter__unpin() -> ::std::os::raw::c_int;
}

/* C conditional: !HAVE_BPF_SKEL */
#[cfg(not(HAVE_BPF_SKEL))]
#[inline]
pub unsafe extern "C" fn perf_bpf_filter__parse(
    _expr_head: *mut list_head,
    _str: *const ::std::os::raw::c_char,
) -> ::std::os::raw::c_int {
    pr_err!(
        "Error: BPF filter is requested but perf is not built with BPF.\n\
        \tPlease make sure to build with libbpf and BPF skeleton.\n"
    );
    -EOPNOTSUPP
}

#[cfg(not(HAVE_BPF_SKEL))]
#[inline]
pub unsafe extern "C" fn perf_bpf_filter__prepare(
    _evsel: *mut evsel,
    _target: *mut target,
) -> ::std::os::raw::c_int {
    -EOPNOTSUPP
}

#[cfg(not(HAVE_BPF_SKEL))]
#[inline]
pub unsafe extern "C" fn perf_bpf_filter__destroy(
    _evsel: *mut evsel,
) -> ::std::os::raw::c_int {
    -EOPNOTSUPP
}

#[cfg(not(HAVE_BPF_SKEL))]
#[inline]
pub unsafe extern "C" fn perf_bpf_filter__lost_count(_evsel: *mut evsel) -> u64 {
    0
}

#[cfg(not(HAVE_BPF_SKEL))]
#[inline]
pub unsafe extern "C" fn perf_bpf_filter__pin() -> ::std::os::raw::c_int {
    -EOPNOTSUPP
}

#[cfg(not(HAVE_BPF_SKEL))]
#[inline]
pub unsafe extern "C" fn perf_bpf_filter__unpin() -> ::std::os::raw::c_int {
    -EOPNOTSUPP
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
