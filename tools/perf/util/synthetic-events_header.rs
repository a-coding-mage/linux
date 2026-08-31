/* SPDX-License-Identifier: GPL-2.0 */

use core::ffi::{c_char, c_int, c_uint, c_ulong, c_void};

pub type pid_t = c_int;
pub type size_t = c_ulong;
pub type u32 = u32;
pub type u64 = u64;
pub type __u16 = u16;
pub type __u32 = u32;
pub type __u64 = u64;

#[repr(C)]
pub struct auxtrace_record {
    _private: [u8; 0],
}

#[repr(C)]
pub struct build_id {
    _private: [u8; 0],
}

#[repr(C)]
pub struct dso {
    _private: [u8; 0],
}

#[repr(C)]
pub struct evlist {
    _private: [u8; 0],
}

#[repr(C)]
pub struct evsel {
    _private: [u8; 0],
}

#[repr(C)]
pub struct machine {
    _private: [u8; 0],
}

#[repr(C)]
pub struct perf_counts_values {
    _private: [u8; 0],
}

#[repr(C)]
pub struct perf_cpu_map {
    _private: [u8; 0],
}

#[repr(C)]
pub struct perf_data {
    _private: [u8; 0],
}

#[repr(C)]
pub struct perf_event_attr {
    _private: [u8; 0],
}

#[repr(C)]
pub struct perf_event_mmap_page {
    _private: [u8; 0],
}

#[repr(C)]
pub struct perf_sample {
    _private: [u8; 0],
}

#[repr(C)]
pub struct perf_session {
    _private: [u8; 0],
}

#[repr(C)]
pub struct perf_stat_config {
    _private: [u8; 0],
}

#[repr(C)]
pub struct perf_thread_map {
    _private: [u8; 0],
}

#[repr(C)]
pub struct perf_tool {
    _private: [u8; 0],
}

#[repr(C)]
pub struct record_opts {
    _private: [u8; 0],
}

#[repr(C)]
pub struct target {
    _private: [u8; 0],
}

#[repr(C)]
pub struct perf_cpu {
    _private: [u8; 0],
}

#[repr(C)]
pub union perf_event {
    _bindgen_union_align: [u64; 0],
}

pub const PERF_SYNTH_TASK: c_uint = 1 << 0;
pub const PERF_SYNTH_MMAP: c_uint = 1 << 1;
pub const PERF_SYNTH_CGROUP: c_uint = 1 << 2;

/* last element */
pub const PERF_SYNTH_MAX: c_uint = 1 << 3;
pub const PERF_SYNTH_ALL: c_uint = PERF_SYNTH_MAX - 1;

pub type perf_event__handler_t = Option<
    unsafe extern "C" fn(
        tool: *const perf_tool,
        event: *mut perf_event,
        sample: *mut perf_sample,
        machine: *mut machine,
    ) -> c_int,
>;

unsafe extern "C" {
    pub fn parse_synth_opt(str_: *mut c_char) -> c_int;

    pub fn perf_event__synthesize_attrs(
        tool: *const perf_tool,
        evlist: *mut evlist,
        process: perf_event__handler_t,
    ) -> c_int;
    pub fn perf_event__synthesize_attr(
        tool: *const perf_tool,
        attr: *mut perf_event_attr,
        ids: u32,
        id: *mut u64,
        process: perf_event__handler_t,
    ) -> c_int;
    pub fn perf_event__synthesize_build_id(
        tool: *const perf_tool,
        sample: *mut perf_sample,
        machine: *mut machine,
        process: perf_event__handler_t,
        misc: __u16,
        bid: *const build_id,
        filename: *const c_char,
    ) -> c_int;
    pub fn perf_event__synthesize_mmap2_build_id(
        tool: *const perf_tool,
        sample: *mut perf_sample,
        machine: *mut machine,
        process: perf_event__handler_t,
        misc: __u16,
        pid: __u32,
        tid: __u32,
        start: __u64,
        len: __u64,
        pgoff: __u64,
        bid: *const build_id,
        prot: __u32,
        flags: __u32,
        filename: *const c_char,
    ) -> c_int;
    pub fn perf_event__synthesize_cpu_map(
        tool: *const perf_tool,
        cpus: *const perf_cpu_map,
        process: perf_event__handler_t,
        machine: *mut machine,
    ) -> c_int;
    pub fn perf_event__synthesize_event_update_cpus(
        tool: *const perf_tool,
        evsel: *mut evsel,
        process: perf_event__handler_t,
    ) -> c_int;
    pub fn perf_event__synthesize_event_update_name(
        tool: *const perf_tool,
        evsel: *mut evsel,
        process: perf_event__handler_t,
    ) -> c_int;
    pub fn perf_event__synthesize_event_update_scale(
        tool: *const perf_tool,
        evsel: *mut evsel,
        process: perf_event__handler_t,
    ) -> c_int;
    pub fn perf_event__synthesize_event_update_unit(
        tool: *const perf_tool,
        evsel: *mut evsel,
        process: perf_event__handler_t,
    ) -> c_int;
    pub fn perf_event__synthesize_extra_attr(
        tool: *const perf_tool,
        evsel_list: *mut evlist,
        process: perf_event__handler_t,
        is_pipe: bool,
    ) -> c_int;
    pub fn perf_event__synthesize_extra_kmaps(
        tool: *const perf_tool,
        process: perf_event__handler_t,
        machine: *mut machine,
    ) -> c_int;
    pub fn perf_event__synthesize_features(
        tool: *const perf_tool,
        session: *mut perf_session,
        evlist: *mut evlist,
        process: perf_event__handler_t,
    ) -> c_int;
    pub fn perf_event__synthesize_id_index(
        tool: *const perf_tool,
        process: perf_event__handler_t,
        evlist: *mut evlist,
        machine: *mut machine,
    ) -> c_int;
    pub fn __perf_event__synthesize_id_index(
        tool: *const perf_tool,
        process: perf_event__handler_t,
        evlist: *mut evlist,
        machine: *mut machine,
        from: size_t,
    ) -> c_int;
    pub fn perf_event__synthesize_id_sample(
        array: *mut __u64,
        type_: u64,
        sample: *const perf_sample,
    ) -> c_int;
    pub fn perf_event__synthesize_kernel_mmap(
        tool: *const perf_tool,
        process: perf_event__handler_t,
        machine: *mut machine,
    ) -> c_int;
    pub fn perf_event__synthesize_mmap_events(
        tool: *const perf_tool,
        event: *mut perf_event,
        pid: pid_t,
        tgid: pid_t,
        process: perf_event__handler_t,
        machine: *mut machine,
        mmap_data: bool,
    ) -> c_int;
    pub fn perf_event__synthesize_modules(
        tool: *const perf_tool,
        process: perf_event__handler_t,
        machine: *mut machine,
    ) -> c_int;
    pub fn perf_event__synthesize_namespaces(
        tool: *const perf_tool,
        event: *mut perf_event,
        pid: pid_t,
        tgid: pid_t,
        process: perf_event__handler_t,
        machine: *mut machine,
    ) -> c_int;
    pub fn perf_event__synthesize_cgroups(
        tool: *const perf_tool,
        process: perf_event__handler_t,
        machine: *mut machine,
    ) -> c_int;
    pub fn perf_event__synthesize_sample(
        event: *mut perf_event,
        type_: u64,
        read_format: u64,
        branch_sample_type: u64,
        sample: *const perf_sample,
    ) -> c_int;
    pub fn perf_event__synthesize_stat_config(
        tool: *const perf_tool,
        config: *mut perf_stat_config,
        process: perf_event__handler_t,
        machine: *mut machine,
    ) -> c_int;
    pub fn perf_event__synthesize_stat_events(
        config: *mut perf_stat_config,
        tool: *const perf_tool,
        evlist: *mut evlist,
        process: perf_event__handler_t,
        attrs: bool,
    ) -> c_int;
    pub fn perf_event__synthesize_stat_round(
        tool: *const perf_tool,
        time: u64,
        type_: u64,
        process: perf_event__handler_t,
        machine: *mut machine,
    ) -> c_int;
    pub fn perf_event__synthesize_stat(
        tool: *const perf_tool,
        cpu: perf_cpu,
        thread: u32,
        id: u64,
        count: *mut perf_counts_values,
        process: perf_event__handler_t,
        machine: *mut machine,
    ) -> c_int;
    pub fn perf_event__synthesize_thread_map2(
        tool: *const perf_tool,
        threads: *mut perf_thread_map,
        process: perf_event__handler_t,
        machine: *mut machine,
    ) -> c_int;
    pub fn perf_event__synthesize_thread_map(
        tool: *const perf_tool,
        threads: *mut perf_thread_map,
        process: perf_event__handler_t,
        machine: *mut machine,
        needs_mmap: bool,
        mmap_data: bool,
    ) -> c_int;
    pub fn perf_event__synthesize_threads(
        tool: *const perf_tool,
        process: perf_event__handler_t,
        machine: *mut machine,
        needs_mmap: bool,
        mmap_data: bool,
        nr_threads_synthesize: c_uint,
    ) -> c_int;
    pub fn perf_event__synthesize_tracing_data(
        tool: *const perf_tool,
        fd: c_int,
        evlist: *mut evlist,
        process: perf_event__handler_t,
    ) -> c_int;
    pub fn perf_event__synth_time_conv(
        pc: *const perf_event_mmap_page,
        tool: *const perf_tool,
        process: perf_event__handler_t,
        machine: *mut machine,
    ) -> c_int;
    pub fn perf_event__synthesize_comm(
        tool: *const perf_tool,
        event: *mut perf_event,
        pid: pid_t,
        process: perf_event__handler_t,
        machine: *mut machine,
    ) -> pid_t;
    pub fn perf_event__synthesize_final_bpf_metadata(
        session: *mut perf_session,
        process: perf_event__handler_t,
    );

    pub fn perf_tool__process_synth_event(
        tool: *const perf_tool,
        event: *mut perf_event,
        machine: *mut machine,
        process: perf_event__handler_t,
    ) -> c_int;

    pub fn perf_event__sample_event_size(
        sample: *const perf_sample,
        type_: u64,
        read_format: u64,
        branch_sample_type: u64,
    ) -> size_t;

    pub fn __machine__synthesize_threads(
        machine: *mut machine,
        tool: *const perf_tool,
        target: *mut target,
        threads: *mut perf_thread_map,
        process: perf_event__handler_t,
        needs_mmap: bool,
        data_mmap: bool,
        nr_threads_synthesize: c_uint,
    ) -> c_int;
    pub fn machine__synthesize_threads(
        machine: *mut machine,
        target: *mut target,
        threads: *mut perf_thread_map,
        needs_mmap: bool,
        data_mmap: bool,
        nr_threads_synthesize: c_uint,
    ) -> c_int;

    pub fn perf_event__synthesize_auxtrace_info(
        itr: *mut auxtrace_record,
        tool: *const perf_tool,
        session: *mut perf_session,
        process: perf_event__handler_t,
    ) -> c_int;

    /* HAVE_LIBBPF_SUPPORT declaration. */
    pub fn perf_event__synthesize_bpf_events(
        session: *mut perf_session,
        process: perf_event__handler_t,
        machine: *mut machine,
        opts: *mut record_opts,
    ) -> c_int;

    pub fn perf_event__synthesize_for_pipe(
        tool: *const perf_tool,
        session: *mut perf_session,
        data: *mut perf_data,
        process: perf_event__handler_t,
    ) -> c_int;

    pub fn perf_event__synthesize_schedstat(
        tool: *const perf_tool,
        process: perf_event__handler_t,
        user_requested_cpu: *mut perf_cpu_map,
    ) -> c_int;
}

/* Without HAVE_LIBBPF_SUPPORT, the C header provides a static inline fallback
 * for perf_event__synthesize_bpf_events that ignores all parameters and returns 0.
 */
#[inline]
pub unsafe fn perf_event__synthesize_bpf_events_no_libbpf(
    _session: *mut perf_session,
    _process: perf_event__handler_t,
    _machine: *mut machine,
    _opts: *mut record_opts,
) -> c_int {
    0
}
