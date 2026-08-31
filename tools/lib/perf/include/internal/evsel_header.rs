/* SPDX-License-Identifier: GPL-2.0 */

/* C dependencies translated from:
 * <linux/types.h>, <linux/perf_event.h>, <stdbool.h>, <sys/types.h>,
 * and <internal/cpumap.h>.
 *
 * The concrete definitions of list_head, hlist_node, hlist_head,
 * perf_event_attr, perf_cpu_map, perf_cpu, perf_thread_map, and xyarray are
 * supplied by other translated headers.
 */

pub type pid_t = i32;

#[repr(C)]
pub struct perf_thread_map {
    _unused: [u8; 0],
}

#[repr(C)]
pub struct xyarray {
    _unused: [u8; 0],
}

/**
 * The per-thread accumulated period storage node.
 */
#[repr(C)]
pub struct perf_sample_id_period {
    pub node: list_head,
    pub hnode: hlist_node,
    /* Holds total ID period value for PERF_SAMPLE_READ processing. */
    pub period: u64,
    /* The TID that the values belongs to */
    pub tid: u32,
}

/**
 * perf_evsel_for_each_per_thread_period_safe - safely iterate thru all the
 * per_stream_periods
 * @evlist:perf_evsel instance to iterate
 * @item: struct perf_sample_id_period iterator
 * @tmp: struct perf_sample_id_period temp iterator
 *
 * C macro:
 * list_for_each_entry_safe(item, tmp, &(evsel)->per_stream_periods, node)
 */

pub const PERF_SAMPLE_ID__HLIST_BITS: usize = 4;
pub const PERF_SAMPLE_ID__HLIST_SIZE: usize = 1 << PERF_SAMPLE_ID__HLIST_BITS;

/*
 * Per fd, to map back from PERF_SAMPLE_ID to evsel, only used when there are
 * more than one entry in the evlist.
 */
#[repr(C)]
pub struct perf_sample_id {
    pub node: hlist_node,
    pub id: u64,
    pub evsel: *mut perf_evsel,
    /*
     * 'idx' will be used for AUX area sampling. A sample will have AUX area
     * data that will be queued for decoding, where there are separate
     * queues for each CPU (per-cpu tracing) or task (per-thread tracing).
     * The sample ID can be used to lookup 'idx' which is effectively the
     * queue number.
     */
    pub idx: i32,
    pub cpu: perf_cpu,
    pub tid: pid_t,

    /* Guest machine pid and VCPU, valid only if machine_pid is non-zero */
    pub machine_pid: pid_t,
    pub vcpu: perf_cpu,

    /*
     * Per-thread, and global event counts are mutually exclusive:
     * Whilst it is possible to combine events into a group with differing
     * values of PERF_SAMPLE_READ, it is not valid to have inconsistent
     * values for `inherit`. Therefore it is not possible to have a
     * situation where a per-thread event is sampled as a global event;
     * all !inherit groups are global, and all groups where the sampling
     * event is inherit + PERF_SAMPLE_READ will be per-thread. Any event
     * that is part of such a group that is inherit but not PERF_SAMPLE_READ
     * will be read as per-thread. If such an event can also trigger a
     * sample (such as with sample_period > 0) then it will not cause
     * `read_format` to be included in its PERF_RECORD_SAMPLE, and
     * therefore will not expose the per-thread group members as global.
     */
    pub u: perf_sample_id__bindgen_ty_1,
}

#[repr(C)]
pub union perf_sample_id__bindgen_ty_1 {
    /*
     * Holds total ID period value for PERF_SAMPLE_READ processing
     * (when period is not per-thread).
     */
    pub period: u64,
    /*
     * Holds total ID period value for PERF_SAMPLE_READ processing
     * (when period is per-thread).
     */
    pub periods: [hlist_head; PERF_SAMPLE_ID__HLIST_SIZE],
}

#[repr(C)]
pub struct perf_evsel {
    pub node: list_head,
    pub attr: perf_event_attr,
    /** The commonly used cpu map of CPUs the event should be opened upon, etc. */
    pub cpus: *mut perf_cpu_map,
    /**
     * The cpu map read from the PMU. For core PMUs this is the list of all
     * CPUs the event can be opened upon. For other PMUs this is the default
     * cpu map for opening the event on, for example, the first CPU on a
     * socket for an uncore event.
     */
    pub pmu_cpus: *mut perf_cpu_map,
    pub threads: *mut perf_thread_map,
    pub fd: *mut xyarray,
    pub mmap: *mut xyarray,
    pub sample_id: *mut xyarray,
    pub id: *mut u64,
    pub ids: u32,
    pub leader: *mut perf_evsel,

    /* For events where the read_format value is per-thread rather than
     * global, stores the per-thread cumulative period */
    pub per_stream_periods: list_head,

    /* parse modifier helper */
    pub nr_members: i32,
    /*
     * system_wide is for events that need to be on every CPU, irrespective
     * of user requested CPUs or threads. Tha main example of this is the
     * dummy event. Map propagation will set cpus for this event to all CPUs
     * as software PMU events like dummy, have a CPU map that is empty.
     */
    pub system_wide: bool,
    /*
     * Some events, for example uncore events, require a CPU.
     * i.e. it cannot be the 'any CPU' value of -1.
     */
    pub requires_cpu: bool,
    /** Is the PMU for the event a core one? Effects the handling of own_cpus. */
    pub is_pmu_core: bool,
    /** Does the evsel on read on the first CPU index such as tool time events? */
    pub reads_only_on_cpu_idx0: bool,
    pub idx: i32,
}

unsafe extern "C" {
    pub fn perf_evsel__init(evsel: *mut perf_evsel, attr: *mut perf_event_attr, idx: i32);
    pub fn perf_evsel__exit(evsel: *mut perf_evsel);
    pub fn perf_evsel__alloc_fd(evsel: *mut perf_evsel, ncpus: i32, nthreads: i32) -> i32;
    pub fn perf_evsel__close_fd(evsel: *mut perf_evsel);
    pub fn perf_evsel__free_fd(evsel: *mut perf_evsel);
    pub fn perf_evsel__read_size(evsel: *mut perf_evsel) -> i32;
    pub fn perf_evsel__apply_filter(
        evsel: *mut perf_evsel,
        filter: *const ::std::os::raw::c_char,
    ) -> i32;

    pub fn perf_evsel__alloc_id(evsel: *mut perf_evsel, ncpus: i32, nthreads: i32) -> i32;
    pub fn perf_evsel__free_id(evsel: *mut perf_evsel);

    pub fn perf_evsel__attr_has_per_thread_sample_period(evsel: *mut perf_evsel) -> bool;

    pub fn perf_sample_id__get_period_storage(
        sid: *mut perf_sample_id,
        tid: u32,
        per_thread: bool,
    ) -> *mut u64;
}
