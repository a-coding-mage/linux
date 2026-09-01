/* SPDX-License-Identifier: GPL-2.0 */

use std::os::raw::{c_char, c_int, c_uint, c_ulong, c_void};

pub type size_t = usize;
pub type off_t = i64;
pub type u8 = std::primitive::u8;
pub type u16 = std::primitive::u16;
pub type u32 = std::primitive::u32;
pub type u64 = std::primitive::u64;
pub type __u64 = std::primitive::u64;
pub type uint16_t = std::primitive::u16;
pub type uint32_t = std::primitive::u32;
pub type refcount_t = c_int;

#[repr(C)]
pub struct bperf_follower_bpf {
    _unused: [u8; 0],
}
#[repr(C)]
pub struct bperf_leader_bpf {
    _unused: [u8; 0],
}
#[repr(C)]
pub struct bpf_counter_ops {
    _unused: [u8; 0],
}
#[repr(C)]
pub struct bpf_object {
    _unused: [u8; 0],
}
#[repr(C)]
pub struct cgroup {
    _unused: [u8; 0],
}
#[repr(C)]
pub struct hashmap {
    _unused: [u8; 0],
}
#[repr(C)]
pub struct perf_counts {
    _unused: [u8; 0],
}
#[repr(C)]
pub struct perf_pmu {
    _unused: [u8; 0],
}
#[repr(C)]
pub struct perf_stat_config {
    _unused: [u8; 0],
}
#[repr(C)]
pub struct perf_stat_evsel {
    _unused: [u8; 0],
}
#[repr(C)]
pub struct target {
    _unused: [u8; 0],
}
#[repr(C)]
pub union perf_event {
    _unused: [u8; 0],
}
#[repr(C)]
pub struct perf_cpu_map {
    _unused: [u8; 0],
}
#[repr(C)]
pub struct perf_thread_map {
    _unused: [u8; 0],
}
#[repr(C)]
pub struct thread_map {
    _unused: [u8; 0],
}
#[repr(C)]
pub struct record_opts {
    _unused: [u8; 0],
}
#[repr(C)]
pub struct callchain_param {
    _unused: [u8; 0],
}
#[repr(C)]
pub struct perf_sample {
    _unused: [u8; 0],
}
#[repr(C)]
pub struct tep_format_field {
    _unused: [u8; 0],
}
#[repr(C)]
pub struct tep_event {
    _unused: [u8; 0],
}
#[repr(C)]
pub struct perf_session {
    _unused: [u8; 0],
}
#[repr(C)]
pub struct perf_env {
    _unused: [u8; 0],
}
#[repr(C)]
pub struct xyarray {
    _unused: [u8; 0],
}

pub type evsel__sb_cb_t =
    Option<unsafe extern "C" fn(event: *mut perf_event, data: *mut c_void) -> c_int>;

#[repr(C)]
pub struct list_head {
    pub next: *mut list_head,
    pub prev: *mut list_head,
}

#[repr(C)]
pub struct perf_event_attr {
    pub type_: u32,
    pub config: u64,
    pub sample_type: u64,
    pub branch_sample_type: u64,
}

#[repr(C)]
pub struct perf_evsel {
    pub node: list_head,
    pub leader: *mut perf_evsel,
    pub nr_members: c_int,
    pub idx: c_int,
    pub attr: perf_event_attr,
}

#[repr(C)]
pub struct evlist {
    _unused: [u8; 0],
}

#[repr(C)]
pub struct perf_counts_values {
    _unused: [u8; 0],
}

#[repr(C)]
pub struct evsel_parse_fields {
    pub name: *mut c_char,
    pub group_name: *mut c_char,
    pub group_pmu_name: *const c_char,
    /* HAVE_LIBTRACEEVENT fields in C:
     * char *tp_sys;
     * char *tp_name;
     * struct tep_event *tp_format;
     */
    pub filter: *mut c_char,
    pub max_events: c_ulong,
    pub scale: f64,
    pub unit: *const c_char,
    pub cgrp: *mut cgroup,
    pub metric_id: *const c_char,

    /* The PMU the event is from. Used for missing_features, PMU name, etc. */
    pub pmu: *mut perf_pmu,

    /*
     * This point to the first evsel with the same name, intended to store the
     * aggregated counts in aggregation mode.
     */
    pub first_wildcard_match: *mut evsel,
    /* parse modifier helper */
    pub exclude_GH: c_int,
    pub sample_read: c_int,
    pub snapshot: bool,
    pub per_pkg: bool,
    pub percore: bool,
    pub precise_max: bool,
    pub is_libpfm_event: bool,
    pub collect_stat: bool,
    pub weak_group: bool,
    pub bpf_counter: bool,
    pub use_config_name: bool,
    pub skippable: bool,
    pub retire_lat: bool,
    pub dont_regroup: bool,
    pub default_metricgroup: bool, /* A member of the Default metricgroup */
    pub default_show_events: bool, /* If a default group member, show the event */
    pub config_terms: list_head,
    pub alternate_hw_config: u64,
}

#[repr(C)]
pub struct evsel_side_band {
    pub cb: evsel__sb_cb_t,
    pub data: *mut c_void,
}

#[repr(C)]
pub union evsel_bpf_skel {
    pub leader_skel: *mut bperf_leader_bpf,
    pub follower_skel: *mut bperf_follower_bpf,
    pub bpf_skel: *mut c_void,
}

#[repr(C)]
pub struct _retirement_latency {
    pub mean: f64,
    pub min: f64,
    pub max: f64,
}

#[repr(C)]
pub struct evsel_duration_time {
    pub start_time: __u64,
    pub accumulated_time: __u64,
}

#[repr(C)]
pub struct evsel_process_time {
    pub start_times: *mut xyarray,
    pub accumulated_times: *mut xyarray,
}

#[repr(C)]
pub union evsel_tool_time {
    /* Defaults for retirement latency events. */
    pub retirement_latency: _retirement_latency,
    /* duration_time is a single global time. */
    pub duration_time: evsel_duration_time,
    /*
     * user_time and system_time read an initial value potentially
     * per-CPU or per-pid.
     */
    pub process_time: evsel_process_time,
}

/** struct evsel - event selector
 *
 * @evlist - evlist this evsel is in, if it is in one.
 * @core - libperf evsel object
 * @name - Can be set to retain the original event name passed by the user,
 *         so that when showing results in tools such as 'perf stat', we
 *         show the name used, not some alias.
 * @id_pos: the position of the event id (PERF_SAMPLE_ID or
 *          PERF_SAMPLE_IDENTIFIER) in a sample event i.e. in the array of
 *          struct perf_record_sample
 * @is_pos: the position (counting backwards) of the event id (PERF_SAMPLE_ID or
 *          PERF_SAMPLE_IDENTIFIER) in a non-sample event i.e. if sample_id_all
 *          is used there is an id sample appended to non-sample events
 * @priv:   And what is in its containing unnamed union are tool specific
 */
#[repr(C)]
pub struct evsel {
    pub core: perf_evsel,
    pub evlist: *mut evlist,
    pub refcnt: refcount_t,
    pub id_offset: off_t,
    pub id_pos: c_int,
    pub is_pos: c_int,
    pub sample_size: c_uint,

    /*
     * These fields can be set in the parse-events code or similar.
     * Please check evsel__clone() to copy them properly so that
     * they can be released properly.
     */
    pub parse_fields: evsel_parse_fields,

    /*
     * metric fields are similar, but needs more care as they can have
     * references to other metric (evsel).
     */
    pub metric_leader: *mut evsel,

    pub handler: *mut c_void,
    pub counts: *mut perf_counts,
    pub prev_raw_counts: *mut perf_counts,
    pub nr_events_printed: c_ulong,
    pub stats: *mut perf_stat_evsel,
    pub priv_: *mut c_void,
    pub db_id: u64,
    pub uniquified_name: bool,
    pub supported: bool,
    pub needs_swap: bool,
    pub disabled: bool,
    pub no_aux_samples: bool,
    pub immediate: bool,
    pub tracking: bool,
    pub ignore_missing_thread: bool,
    pub forced_leader: bool,
    pub cmdline_group_boundary: bool,
    pub reset_group: bool,
    pub needs_auxtrace_mmap: bool,
    pub needs_uniquify: bool,
    pub fallenback_eacces: bool,
    pub fallenback_eopnotsupp: bool,
    pub probe_type: u8, /* C bit-field: u8 probe_type:3 */
    pub per_pkg_mask: *mut hashmap,
    pub err: c_int,
    pub script_output_type: c_int,
    pub side_band: evsel_side_band,
    /*
     * For reporting purposes, an evsel sample can have a callchain
     * synthesized from AUX area data. Keep track of synthesized sample
     * types here. Note, the recorded sample_type cannot be changed because
     * it is needed to continue to parse events.
     * See also evsel__has_callchain().
     */
    pub synth_sample_type: __u64,

    /*
     * Store the branch counter related information.
     * br_cntr_idx: The idx of the branch counter event in the evlist
     * br_cntr_nr:  The number of the branch counter event in the group
     *              (Only available for the leader event)
     * abbr_name:   The abbreviation name assigned to an event which is
     *              logged by the branch counter.
     *              The abbr name is from A to Z9. NA is applied if out
     *              of the range.
     */
    pub br_cntr_idx: c_int,
    pub br_cntr_nr: c_int,
    pub abbr_name: [c_char; 3],

    /*
     * bpf_counter_ops serves two use cases:
     *   1. perf-stat -b          counting events used byBPF programs
     *   2. perf-stat --use-bpf   use BPF programs to aggregate counts
     */
    pub bpf_counter_ops: *mut bpf_counter_ops,

    pub bpf_counter_list: list_head, /* for perf-stat -b */
    pub bpf_filters: list_head,      /* for perf-record --filter */

    /* for perf-stat --use-bpf */
    pub bperf_leader_prog_fd: c_int,
    pub bperf_leader_link_fd: c_int,
    pub bpf_skel_union: evsel_bpf_skel,
    pub open_flags: c_ulong,
    pub precise_ip_original: c_int,

    /* For tool events */
    /* Beginning time subtracted when the counter is read. */
    pub tool_time: evsel_tool_time,
    /* Is the tool's fd for /proc/pid/stat or /proc/stat. */
    pub pid_stat: bool,
}

#[repr(C)]
pub struct perf_missing_features {
    pub sample_id_all: bool,
    pub exclude_guest: bool,
    pub mmap2: bool,
    pub cloexec: bool,
    pub clockid: bool,
    pub clockid_wrong: bool,
    pub lbr_flags: bool,
    pub write_backward: bool,
    pub group_read: bool,
    pub ksymbol: bool,
    pub bpf: bool,
    pub aux_output: bool,
    pub branch_hw_idx: bool,
    pub cgroup: bool,
    pub data_page_size: bool,
    pub code_page_size: bool,
    pub weight_struct: bool,
    pub read_lost: bool,
    pub branch_counters: bool,
    pub aux_action: bool,
    pub inherit_sample_read: bool,
    pub defer_callchain: bool,
}

pub const EVSEL__MAX_ALIASES: usize = 8;

pub type perf_event_sample_format = c_int;

unsafe extern "C" {
    pub static mut perf_missing_features: perf_missing_features;

    pub fn perf_evsel__cpus(evsel: *mut perf_evsel) -> *mut perf_cpu_map;
    pub fn perf_cpu_map__nr(cpus: *mut perf_cpu_map) -> c_int;
    pub fn list_empty(head: *const list_head) -> c_int;

    pub fn evsel__compute_deltas(
        evsel: *mut evsel,
        cpu: c_int,
        thread: c_int,
        count: *mut perf_counts_values,
    );
    pub fn evsel__object_config(
        object_size: size_t,
        init: Option<unsafe extern "C" fn(evsel: *mut evsel) -> c_int>,
        fini: Option<unsafe extern "C" fn(evsel: *mut evsel)>,
    ) -> c_int;
    pub fn evsel__find_pmu(evsel: *const evsel) -> *mut perf_pmu;
    pub fn evsel__pmu_name(evsel: *const evsel) -> *const c_char;
    pub fn evsel__is_aux_event(evsel: *const evsel) -> bool;
    pub fn evsel__is_probe(evsel: *mut evsel) -> bool;
    pub fn evsel__is_kprobe(evsel: *mut evsel) -> bool;
    pub fn evsel__is_uprobe(evsel: *mut evsel) -> bool;
    pub fn evsel__new_idx(attr: *mut perf_event_attr, idx: c_int) -> *mut evsel;
    pub fn evsel__clone(orig: *mut evsel) -> *mut evsel;
    pub fn copy_config_terms(dst: *mut list_head, src: *mut list_head) -> c_int;
    pub fn free_config_terms(config_terms: *mut list_head);
    /*
     * Returns pointer with encoded error via <linux/err.h> interface.
     */
    pub fn evsel__newtp_idx(
        sys: *const c_char,
        name: *const c_char,
        idx: c_int,
        format: bool,
    ) -> *mut evsel;
    pub fn evsel__get(evsel: *mut evsel) -> *mut evsel;
    pub fn evsel__put(evsel: *mut evsel);
    /* HAVE_LIBTRACEEVENT: pub fn evsel__tp_format(evsel: *mut evsel) -> *mut tep_event; */
    pub fn evsel__set_priv_destructor(destructor: Option<unsafe extern "C" fn(priv_: *mut c_void)>);
    pub fn evsel__config(
        evsel: *mut evsel,
        opts: *const record_opts,
        callchain: *const callchain_param,
    );
    pub fn evsel__config_callchain(
        evsel: *mut evsel,
        opts: *const record_opts,
        callchain: *const callchain_param,
    );
    pub fn __evsel__sample_size(sample_type: u64) -> c_int;
    pub fn evsel__calc_id_pos(evsel: *mut evsel);
    pub fn evsel__is_cache_op_valid(type_: u8, op: u8) -> bool;

    pub static evsel__hw_cache: [[*const c_char; EVSEL__MAX_ALIASES]; PERF_COUNT_HW_CACHE_MAX];
    pub static evsel__hw_cache_op: [[*const c_char; EVSEL__MAX_ALIASES]; PERF_COUNT_HW_CACHE_OP_MAX];
    pub static evsel__hw_cache_result: [[*const c_char; EVSEL__MAX_ALIASES]; PERF_COUNT_HW_CACHE_RESULT_MAX];
    pub static evsel__hw_names: [*const c_char; PERF_COUNT_HW_MAX];
    pub static evsel__sw_names: [*const c_char; PERF_COUNT_SW_MAX];
    pub static mut evsel__bpf_counter_events: *mut c_char;
    pub fn evsel__match_bpf_counter_events(name: *const c_char) -> bool;
    pub fn arch_evsel__hw_name(evsel: *mut evsel, bf: *mut c_char, size: size_t) -> c_int;
    pub fn __evsel__hw_cache_type_op_res_name(
        type_: u8,
        op: u8,
        result: u8,
        bf: *mut c_char,
        size: size_t,
    ) -> c_int;
    pub fn evsel__name(evsel: *mut evsel) -> *const c_char;
    pub fn evsel__name_is(evsel: *mut evsel, name: *const c_char) -> bool;
    pub fn evsel__metric_id(evsel: *const evsel) -> *const c_char;
    pub fn evsel__group_name(evsel: *mut evsel) -> *const c_char;
    pub fn evsel__group_desc(evsel: *mut evsel, buf: *mut c_char, size: size_t) -> c_int;
    pub fn __evsel__set_sample_bit(evsel: *mut evsel, bit: perf_event_sample_format);
    pub fn __evsel__reset_sample_bit(evsel: *mut evsel, bit: perf_event_sample_format);
    pub fn evsel__set_sample_id(evsel: *mut evsel, use_sample_identifier: bool);
    pub fn arch_evsel__set_sample_weight(evsel: *mut evsel);
    pub fn arch__post_evsel_config(evsel: *mut evsel, attr: *mut perf_event_attr);
    pub fn arch_evsel__open_strerror(
        evsel: *mut evsel,
        err: c_int,
        msg: *mut c_char,
        size: size_t,
    ) -> c_int;
    pub fn arch_evsel__apply_ratio_to_prev(evsel: *mut evsel, attr: *mut perf_event_attr);
    pub fn evsel__set_filter(evsel: *mut evsel, filter: *const c_char) -> c_int;
    pub fn evsel__append_tp_filter(evsel: *mut evsel, filter: *const c_char) -> c_int;
    pub fn evsel__append_addr_filter(evsel: *mut evsel, filter: *const c_char) -> c_int;
    pub fn evsel__enable_cpu(evsel: *mut evsel, cpu_map_idx: c_int) -> c_int;
    pub fn evsel__enable(evsel: *mut evsel) -> c_int;
    pub fn evsel__disable(evsel: *mut evsel) -> c_int;
    pub fn evsel__disable_cpu(evsel: *mut evsel, cpu_map_idx: c_int) -> c_int;
    pub fn evsel__open_per_cpu_and_thread(
        evsel: *mut evsel,
        cpus: *mut perf_cpu_map,
        cpu_map_idx: c_int,
        threads: *mut perf_thread_map,
    ) -> c_int;
    pub fn evsel__open_per_cpu(
        evsel: *mut evsel,
        cpus: *mut perf_cpu_map,
        cpu_map_idx: c_int,
    ) -> c_int;
    pub fn evsel__open_per_thread(evsel: *mut evsel, threads: *mut perf_thread_map) -> c_int;
    pub fn evsel__open(
        evsel: *mut evsel,
        cpus: *mut perf_cpu_map,
        threads: *mut perf_thread_map,
    ) -> c_int;
    pub fn evsel__close(evsel: *mut evsel);
    pub fn evsel__prepare_open(
        evsel: *mut evsel,
        cpus: *mut perf_cpu_map,
        threads: *mut perf_thread_map,
    ) -> c_int;
    pub fn evsel__precise_ip_fallback(evsel: *mut evsel) -> bool;
    /* HAVE_LIBTRACEEVENT sample helpers:
     * pub fn perf_sample__rawptr(sample: *mut perf_sample, name: *const c_char) -> *mut c_void;
     * pub fn perf_sample__intval(sample: *mut perf_sample, name: *const c_char) -> u64;
     * pub fn perf_sample__intval_common(sample: *mut perf_sample, name: *const c_char) -> u64;
     * pub fn perf_sample__taskstate(sample: *mut perf_sample, name: *const c_char) -> c_char;
     */
    pub fn format_field__get_raw_data(
        field: *mut tep_format_field,
        sample: *mut perf_sample,
        needs_swap: bool,
        len_out: *mut u16,
    ) -> *mut c_void;
    pub fn format_field__get_cpumask(
        field: *mut tep_format_field,
        sample: *mut perf_sample,
        needs_swap: bool,
        len_out: *mut u16,
    ) -> *mut c_ulong;
    pub fn format_field__intval(
        field: *mut tep_format_field,
        sample: *mut perf_sample,
        needs_swap: bool,
    ) -> u64;
    /* HAVE_LIBTRACEEVENT:
     * pub fn evsel__field(evsel: *mut evsel, name: *const c_char) -> *mut tep_format_field;
     * pub fn evsel__common_field(evsel: *mut evsel, name: *const c_char) -> *mut tep_format_field;
     */
    pub fn __evsel__match(evsel: *const evsel, type_: u32, config: u64) -> bool;
    pub fn evsel__read_counter(evsel: *mut evsel, cpu_map_idx: c_int, thread: c_int) -> c_int;
    pub fn __evsel__read_on_cpu(
        evsel: *mut evsel,
        cpu_map_idx: c_int,
        thread: c_int,
        scale: bool,
    ) -> c_int;
    pub fn __evsel__parse_sample(
        evsel: *mut evsel,
        event: *mut perf_event,
        data: *mut perf_sample,
        needs_swap: bool,
    ) -> c_int;
    pub fn evsel__parse_sample_timestamp(
        evsel: *mut evsel,
        event: *mut perf_event,
        timestamp: *mut u64,
    ) -> c_int;
    pub fn evsel__id_hdr_size(evsel: *const evsel) -> u16;
    pub fn evsel__is_non_perf_event_open_pmu(evsel: *const evsel) -> bool;
    pub fn evsel__is_function_event(evsel: *mut evsel) -> bool;
    pub fn evsel__fallback(
        evsel: *mut evsel,
        target: *mut target,
        err: c_int,
        msg: *mut c_char,
        msgsize: size_t,
    ) -> bool;
    pub fn evsel__open_strerror(
        evsel: *mut evsel,
        target: *mut target,
        err: c_int,
        msg: *mut c_char,
        size: size_t,
    ) -> c_int;
    pub fn evsel__session(evsel: *mut evsel) -> *mut perf_session;
    pub fn evsel__env(evsel: *mut evsel) -> *mut perf_env;
    pub fn evsel__e_machine(evsel: *mut evsel, e_flags: *mut uint32_t) -> uint16_t;
    pub fn evsel__store_ids(evsel: *mut evsel, evlist: *mut evlist) -> c_int;
    pub fn evsel__zero_per_pkg(evsel: *mut evsel);
    pub fn evsel__is_hybrid(evsel: *const evsel) -> bool;
    pub fn evsel__leader(evsel: *const evsel) -> *mut evsel;
    pub fn evsel__has_leader(evsel: *mut evsel, leader: *mut evsel) -> bool;
    pub fn evsel__is_leader(evsel: *mut evsel) -> bool;
    pub fn evsel__set_leader(evsel: *mut evsel, leader: *mut evsel);
    pub fn evsel__source_count(evsel: *const evsel) -> c_int;
    pub fn evsel__remove_from_group(evsel: *mut evsel, leader: *mut evsel);
    pub fn arch_evsel__must_be_in_group(evsel: *const evsel) -> bool;
    pub fn evsel__set_needs_uniquify(
        counter: *mut evsel,
        config: *const perf_stat_config,
    ) -> bool;
    pub fn evsel__uniquify_counter(counter: *mut evsel);
    pub fn evsel__bitfield_swap_branch_flags(value: u64) -> u64;
    pub fn evsel__config_exists(evsel: *const evsel, config_name: *const c_char) -> bool;
    pub fn evsel__get_config_val(
        evsel: *const evsel,
        config_name: *const c_char,
        val: *mut u64,
    ) -> c_int;
    pub fn evsel__set_config_if_unset(
        evsel: *mut evsel,
        config_name: *const c_char,
        val: u64,
    );
    pub fn evsel__is_offcpu_event(evsel: *mut evsel) -> bool;
    pub fn evsel__warn_user_requested_cpus(
        evsel: *mut evsel,
        user_requested_cpus: *mut perf_cpu_map,
    );
}

pub const PERF_COUNT_HW_CACHE_MAX: usize = 0; /* external Linux perf constant */
pub const PERF_COUNT_HW_CACHE_OP_MAX: usize = 0; /* external Linux perf constant */
pub const PERF_COUNT_HW_CACHE_RESULT_MAX: usize = 0; /* external Linux perf constant */
pub const PERF_COUNT_HW_MAX: usize = 0; /* external Linux perf constant */
pub const PERF_COUNT_SW_MAX: usize = 0; /* external Linux perf constant */
pub const PERF_SAMPLE_BRANCH_CALL_STACK: u64 = 0; /* external Linux perf constant */
pub const PERF_SAMPLE_BRANCH_HW_INDEX: u64 = 0; /* external Linux perf constant */
pub const PERF_SAMPLE_CALLCHAIN: u64 = 0; /* external Linux perf constant */
pub const PERF_SAMPLE_BRANCH_STACK: u64 = 0; /* external Linux perf constant */
pub const PERF_TYPE_SOFTWARE: u32 = 0; /* external Linux perf constant */
pub const PERF_COUNT_SW_DUMMY: u64 = 0; /* external Linux perf constant */
pub const PERF_COUNT_SW_BPF_OUTPUT: u64 = 0; /* external Linux perf constant */
pub const PERF_COUNT_SW_CPU_CLOCK: u64 = 0; /* external Linux perf constant */
pub const PERF_COUNT_SW_TASK_CLOCK: u64 = 0; /* external Linux perf constant */

unsafe extern "C" {
    pub static symbol_conf: symbol_conf;
}

#[repr(C)]
pub struct symbol_conf {
    pub event_group: bool,
}

#[inline]
pub unsafe fn evsel__cpus(evsel: *mut evsel) -> *mut perf_cpu_map {
    unsafe { perf_evsel__cpus(&mut (*evsel).core) }
}

#[inline]
pub unsafe fn evsel__nr_cpus(evsel: *mut evsel) -> c_int {
    unsafe { perf_cpu_map__nr(evsel__cpus(evsel)) }
}

#[inline]
pub unsafe fn evsel__new(attr: *mut perf_event_attr) -> *mut evsel {
    unsafe { evsel__new_idx(attr, 0) }
}

#[inline]
pub unsafe fn evsel__newtp(sys: *const c_char, name: *const c_char) -> *mut evsel {
    unsafe { evsel__newtp_idx(sys, name, 0, true) }
}

#[inline]
pub unsafe fn evsel__is_bpf(evsel: *mut evsel) -> bool {
    unsafe { !(*evsel).bpf_counter_ops.is_null() }
}

#[inline]
pub unsafe fn evsel__is_bperf(evsel: *mut evsel) -> bool {
    unsafe { !(*evsel).bpf_counter_ops.is_null() && list_empty(&(*evsel).bpf_counter_list) != 0 }
}

/* C token-pasting macros translated by calling the underlying functions with resolved constants:
 * evsel__set_sample_bit(evsel, bit) => __evsel__set_sample_bit(evsel, PERF_SAMPLE_##bit)
 * evsel__reset_sample_bit(evsel, bit) => __evsel__reset_sample_bit(evsel, PERF_SAMPLE_##bit)
 */

#[inline]
pub unsafe fn evsel__is_retire_lat(evsel: *const evsel) -> bool {
    unsafe { (*evsel).parse_fields.retire_lat }
}

#[cfg(not(HAVE_LIBTRACEEVENT))]
#[inline]
pub unsafe fn evsel__field(
    _evsel: *mut evsel,
    _name: *const c_char,
) -> *mut tep_format_field {
    std::ptr::null_mut()
}

#[cfg(not(HAVE_LIBTRACEEVENT))]
#[inline]
pub unsafe fn evsel__common_field(
    _evsel: *mut evsel,
    _name: *const c_char,
) -> *mut tep_format_field {
    std::ptr::null_mut()
}

/* evsel__match(evsel, t, c) => __evsel__match(evsel, PERF_TYPE_##t, PERF_COUNT_##c) */

/**
 * evsel__read_on_cpu - Read out the results on a CPU and thread
 *
 * @evsel - event selector to read value
 * @cpu_map_idx - CPU of interest
 * @thread - thread of interest
 */
#[inline]
pub unsafe fn evsel__read_on_cpu(evsel: *mut evsel, cpu_map_idx: c_int, thread: c_int) -> c_int {
    unsafe { __evsel__read_on_cpu(evsel, cpu_map_idx, thread, false) }
}

/**
 * evsel__read_on_cpu_scaled - Read out the results on a CPU and thread, scaled
 *
 * @evsel - event selector to read value
 * @cpu_map_idx - CPU of interest
 * @thread - thread of interest
 */
#[inline]
pub unsafe fn evsel__read_on_cpu_scaled(
    evsel: *mut evsel,
    cpu_map_idx: c_int,
    thread: c_int,
) -> c_int {
    unsafe { __evsel__read_on_cpu(evsel, cpu_map_idx, thread, true) }
}

#[inline]
pub unsafe fn evsel__parse_sample(
    evsel: *mut evsel,
    event: *mut perf_event,
    data: *mut perf_sample,
) -> c_int {
    unsafe { __evsel__parse_sample(evsel, event, data, (*evsel).needs_swap) }
}

#[inline]
pub unsafe fn evsel__next(evsel: *mut evsel) -> *mut evsel {
    unsafe { list_entry!((*evsel).core.node.next, evsel, core.node) }
}

#[inline]
pub unsafe fn evsel__prev(evsel: *mut evsel) -> *mut evsel {
    unsafe { list_entry!((*evsel).core.node.prev, evsel, core.node) }
}

/**
 * evsel__is_group_leader - Return whether given evsel is a leader event
 *
 * @evsel - evsel selector to be tested
 *
 * Return %true if @evsel is a group leader or a stand-alone event
 */
#[inline]
pub unsafe fn evsel__is_group_leader(evsel: *const evsel) -> bool {
    unsafe { (*evsel).core.leader == &(*evsel).core as *const perf_evsel as *mut perf_evsel }
}

/**
 * evsel__is_group_event - Return whether given evsel is a group event
 *
 * @evsel - evsel selector to be tested
 *
 * Return %true iff event group view is enabled and @evsel is a actual group
 * leader which has other members in the group
 */
#[inline]
pub unsafe fn evsel__is_group_event(evsel: *mut evsel) -> bool {
    unsafe {
        if !symbol_conf.event_group {
            return false;
        }

        evsel__is_group_leader(evsel) && (*evsel).core.nr_members > 1
    }
}

#[inline]
pub unsafe fn evsel__is_bpf_output(evsel: *mut evsel) -> bool {
    unsafe { __evsel__match(evsel, PERF_TYPE_SOFTWARE, PERF_COUNT_SW_BPF_OUTPUT) }
}

#[inline]
pub unsafe fn evsel__is_clock(evsel: *const evsel) -> bool {
    unsafe {
        __evsel__match(evsel, PERF_TYPE_SOFTWARE, PERF_COUNT_SW_CPU_CLOCK)
            || __evsel__match(evsel, PERF_TYPE_SOFTWARE, PERF_COUNT_SW_TASK_CLOCK)
    }
}

#[inline]
pub unsafe fn evsel__group_idx(evsel: *mut evsel) -> c_int {
    unsafe { (*evsel).core.idx - (*(*evsel).core.leader).idx }
}

/* Iterates group WITHOUT the leader.
 * for_each_group_member_head(_evsel, _leader, _head)
 * for_each_group_member(_evsel, _leader)
 *
 * Iterates group WITH the leader.
 * for_each_group_evsel_head(_evsel, _leader, _head)
 * for_each_group_evsel(_evsel, _leader)
 */

#[inline]
pub unsafe fn evsel__has_branch_callstack(evsel: *const evsel) -> bool {
    unsafe { ((*evsel).core.attr.branch_sample_type & PERF_SAMPLE_BRANCH_CALL_STACK) != 0 }
}

#[inline]
pub unsafe fn evsel__has_branch_hw_idx(evsel: *const evsel) -> bool {
    unsafe { ((*evsel).core.attr.branch_sample_type & PERF_SAMPLE_BRANCH_HW_INDEX) != 0 }
}

#[inline]
pub unsafe fn evsel__has_callchain(evsel: *const evsel) -> bool {
    /*
     * For reporting purposes, an evsel sample can have a recorded callchain
     * or a callchain synthesized from AUX area data.
     */
    unsafe {
        ((*evsel).core.attr.sample_type & PERF_SAMPLE_CALLCHAIN) != 0
            || ((*evsel).synth_sample_type & PERF_SAMPLE_CALLCHAIN) != 0
    }
}

#[inline]
pub unsafe fn evsel__has_br_stack(evsel: *const evsel) -> bool {
    /*
     * For reporting purposes, an evsel sample can have a recorded branch
     * stack or a branch stack synthesized from AUX area data.
     */
    unsafe {
        ((*evsel).core.attr.sample_type & PERF_SAMPLE_BRANCH_STACK) != 0
            || ((*evsel).synth_sample_type & PERF_SAMPLE_BRANCH_STACK) != 0
    }
}

#[inline]
pub unsafe fn evsel__is_dummy_event(evsel: *mut evsel) -> bool {
    unsafe {
        ((*evsel).core.attr.type_ == PERF_TYPE_SOFTWARE)
            && ((*evsel).core.attr.config == PERF_COUNT_SW_DUMMY)
    }
}

/*
 * Macro to swap the bit-field postition and size.
 * Used when,
 * - dont need to swap the entire u64 &&
 * - when u64 has variable bit-field sizes &&
 * - when presented in a host endian which is different
 *   than the source endian of the perf.data file
 */
#[inline]
pub fn bitfield_swap(src: u64, pos: u64, size: u64) -> u64 {
    (((src >> pos) & ((1u64 << size) - 1)) << (63 - (pos + size - 1)))
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
