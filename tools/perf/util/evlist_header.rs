/* SPDX-License-Identifier: GPL-2.0 */

// Translated from perf/util/evlist.h. C include dependencies are intentionally
// left as external Rust names supplied by the surrounding translation.

pub type pid_t = i32;
pub type pthread_t = libc::pthread_t;
pub type size_t = usize;
pub type u64 = u64;
pub type u16 = u16;

#[repr(C)]
pub struct perf_cpu_map {
    _unused: [u8; 0],
}

#[repr(C)]
pub struct perf_stat_config {
    _unused: [u8; 0],
}

#[repr(C)]
pub struct pollfd {
    _unused: [u8; 0],
}

#[repr(C)]
pub struct record_opts {
    _unused: [u8; 0],
}

#[repr(C)]
pub struct strbuf {
    _unused: [u8; 0],
}

#[repr(C)]
pub struct target {
    _unused: [u8; 0],
}

#[repr(C)]
pub struct thread_map {
    _unused: [u8; 0],
}

#[repr(C)]
pub struct perf_evlist {
    pub entries: list_head,
    pub nr_entries: i32,
}

#[repr(C)]
pub struct refcount_t {
    _unused: [u8; 0],
}

#[repr(C)]
pub struct mmap {
    _unused: [u8; 0],
}

#[repr(C)]
pub struct evsel {
    pub core: perf_evsel,
}

#[repr(C)]
pub struct perf_evsel {
    pub node: list_head,
}

#[repr(C)]
pub struct events_stats {
    _unused: [u8; 0],
}

#[repr(C)]
pub struct perf_session {
    _unused: [u8; 0],
}

#[repr(C)]
pub union perf_event {
    _bindgen_union_align: u64,
}

#[repr(C)]
pub struct perf_sample {
    _unused: [u8; 0],
}

#[repr(C)]
pub struct rblist {
    _unused: [u8; 0],
}

#[repr(C)]
pub struct list_head {
    pub next: *mut list_head,
    pub prev: *mut list_head,
}

#[repr(C)]
pub struct perf_sample_id {
    _unused: [u8; 0],
}

#[repr(C)]
pub struct perf_event_attr {
    _unused: [u8; 0],
}

#[repr(C)]
pub struct callchain_param {
    _unused: [u8; 0],
}

#[repr(C)]
pub struct option {
    _unused: [u8; 0],
}

#[repr(C)]
pub struct siginfo_t {
    _unused: [u8; 0],
}

#[repr(C)]
pub struct perf_cpu {
    _unused: [u8; 0],
}

#[repr(C)]
pub struct affinity {
    _unused: [u8; 0],
}

pub type evsel__sb_cb_t = Option<unsafe extern "C" fn(*mut evsel, *mut perf_sample, *mut libc::c_void)>;

/*
 * State machine of bkw_mmap_state:
 *
 *                     .________________(forbid)_____________.
 *                     |                                     V
 * NOTREADY --(0)--> RUNNING --(1)--> DATA_PENDING --(2)--> EMPTY
 *                     ^  ^              |   ^               |
 *                     |  |__(forbid)____/   |___(forbid)___/|
 *                     |                                     |
 *                      \_________________(3)_______________/
 *
 * NOTREADY     : Backward ring buffers are not ready
 * RUNNING      : Backward ring buffers are recording
 * DATA_PENDING : We are required to collect data from backward ring buffers
 * EMPTY        : We have collected data from backward ring buffers.
 *
 * (0): Setup backward ring buffer
 * (1): Pause ring buffers for reading
 * (2): Read from ring buffers
 * (3): Resume ring buffers for recording
 */
#[repr(C)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum bkw_mmap_state {
    BKW_MMAP_NOTREADY,
    BKW_MMAP_RUNNING,
    BKW_MMAP_DATA_PENDING,
    BKW_MMAP_EMPTY,
}

#[repr(C)]
pub struct event_enable_timer {
    _unused: [u8; 0],
}

#[repr(C)]
pub struct evlist_workload {
    pub cork_fd: i32,
    pub pid: pid_t,
}

#[repr(C)]
pub struct evlist_sb_thread {
    pub th: pthread_t,
    pub done: i32,
}

#[repr(C)]
pub struct evlist_ctl_fd {
    pub fd: i32,  /* control file descriptor */
    pub ack: i32, /* ack file descriptor for control commands */
    pub pos: i32, /* index at evlist core object to check signals */
}

#[repr(C)]
pub struct evlist {
    pub core: perf_evlist,
    pub refcnt: refcount_t,
    pub enabled: bool,
    pub no_affinity: bool,
    pub id_pos: i32,
    pub is_pos: i32,
    pub nr_br_cntr: i32,
    pub combined_sample_type: u64,
    pub bkw_mmap_state: bkw_mmap_state,
    pub workload: evlist_workload,
    pub mmap: *mut mmap,
    pub overwrite_mmap: *mut mmap,
    pub selected: *mut evsel,
    pub stats: events_stats,
    pub session: *mut perf_session,
    pub trace_event_sample_raw:
        Option<unsafe extern "C" fn(*mut evlist, *mut perf_event, *mut perf_sample)>,
    pub first_sample_time: u64,
    pub last_sample_time: u64,
    pub sb_thread: evlist_sb_thread,
    pub ctl_fd: evlist_ctl_fd,
    pub eet: *mut event_enable_timer,
    /**
     * @metric_events: A list of struct metric_event which each have a list
     * of struct metric_expr.
     */
    pub metric_events: rblist,
    /* samples with deferred_callchain would wait here. */
    pub deferred_samples: list_head,
}

#[repr(C)]
pub struct evsel_str_handler {
    pub name: *const libc::c_char,
    pub handler: *mut libc::c_void,
}

#[inline]
pub unsafe fn evlist__core(evlist: *mut evlist) -> *mut perf_evlist {
    unsafe { &mut (*evlist).core }
}

#[inline]
pub unsafe fn evlist__const_core(evlist: *const evlist) -> *const perf_evlist {
    unsafe { &(*evlist).core }
}

#[inline]
pub unsafe fn evlist__nr_entries(evlist: *const evlist) -> i32 {
    unsafe { (*evlist__const_core(evlist)).nr_entries }
}

#[inline]
pub unsafe fn evlist__enabled(evlist: *const evlist) -> bool {
    unsafe { (*evlist).enabled }
}

#[inline]
pub unsafe fn evlist__set_enabled(evlist: *mut evlist, enabled: bool) {
    unsafe { (*evlist).enabled = enabled };
}

#[inline]
pub unsafe fn evlist__no_affinity(evlist: *const evlist) -> bool {
    unsafe { (*evlist).no_affinity }
}

#[inline]
pub unsafe fn evlist__set_no_affinity(evlist: *mut evlist, no_affinity: bool) {
    unsafe { (*evlist).no_affinity = no_affinity };
}

#[inline]
pub unsafe fn evlist__sb_thread_done(evlist: *const evlist) -> i32 {
    unsafe { core::ptr::read_volatile(&(*evlist).sb_thread.done) }
}

#[inline]
pub unsafe fn evlist__set_sb_thread_done(evlist: *mut evlist, done: i32) {
    unsafe { core::ptr::write_volatile(&mut (*evlist).sb_thread.done, done) };
}

#[inline]
pub unsafe fn evlist__sb_thread_th(evlist: *mut evlist) -> *mut pthread_t {
    unsafe { &mut (*evlist).sb_thread.th }
}

#[inline]
pub unsafe fn evlist__id_pos(evlist: *const evlist) -> i32 {
    unsafe { (*evlist).id_pos }
}

#[inline]
pub unsafe fn evlist__is_pos(evlist: *const evlist) -> i32 {
    unsafe { (*evlist).is_pos }
}

#[inline]
pub unsafe fn evlist__event_enable_timer(evlist: *mut evlist) -> *mut event_enable_timer {
    unsafe { (*evlist).eet }
}

#[inline]
pub unsafe fn evlist__bkw_mmap_state(evlist: *const evlist) -> bkw_mmap_state {
    unsafe { (*evlist).bkw_mmap_state }
}

#[inline]
pub unsafe fn evlist__set_bkw_mmap_state(evlist: *mut evlist, state: bkw_mmap_state) {
    unsafe { (*evlist).bkw_mmap_state = state };
}

#[inline]
pub unsafe fn evlist__mmap(evlist: *mut evlist) -> *mut mmap {
    unsafe { (*evlist).mmap }
}

#[inline]
pub unsafe fn evlist__overwrite_mmap(evlist: *mut evlist) -> *mut mmap {
    unsafe { (*evlist).overwrite_mmap }
}

#[inline]
pub unsafe fn evlist__stats(evlist: *mut evlist) -> *mut events_stats {
    unsafe { &mut (*evlist).stats }
}

#[inline]
pub unsafe fn evlist__first_sample_time(evlist: *const evlist) -> u64 {
    unsafe { (*evlist).first_sample_time }
}

#[inline]
pub unsafe fn evlist__set_first_sample_time(evlist: *mut evlist, first: u64) {
    unsafe { (*evlist).first_sample_time = first };
}

#[inline]
pub unsafe fn evlist__last_sample_time(evlist: *const evlist) -> u64 {
    unsafe { (*evlist).last_sample_time }
}

#[inline]
pub unsafe fn evlist__set_last_sample_time(evlist: *mut evlist, last: u64) {
    unsafe { (*evlist).last_sample_time = last };
}

#[inline]
pub unsafe fn evlist__nr_br_cntr(evlist: *const evlist) -> i32 {
    unsafe { (*evlist).nr_br_cntr }
}

#[inline]
pub unsafe fn evlist__set_nr_br_cntr(evlist: *mut evlist, nr: i32) {
    unsafe { (*evlist).nr_br_cntr = nr };
}

#[inline]
pub unsafe fn evlist__session(evlist: *mut evlist) -> *mut perf_session {
    unsafe { (*evlist).session }
}

#[inline]
pub unsafe fn evlist__set_session(evlist: *mut evlist, session: *mut perf_session) {
    unsafe { (*evlist).session = session };
}

#[inline]
pub unsafe fn evlist__trace_event_sample_raw(
    evlist: *mut evlist,
) -> Option<unsafe extern "C" fn(*mut evlist, *mut perf_event, *mut perf_sample)> {
    unsafe { (*evlist).trace_event_sample_raw }
}

#[inline]
pub unsafe fn evlist__set_trace_event_sample_raw(
    evlist: *mut evlist,
    fun: Option<unsafe extern "C" fn(*mut evlist, *mut perf_event, *mut perf_sample)>,
) {
    unsafe { (*evlist).trace_event_sample_raw = fun };
}

#[inline]
pub unsafe fn evlist__workload_pid(evlist: *const evlist) -> pid_t {
    unsafe { (*evlist).workload.pid }
}

#[inline]
pub unsafe fn evlist__set_workload_pid(evlist: *mut evlist, pid: pid_t) {
    unsafe { (*evlist).workload.pid = pid };
}

#[inline]
pub unsafe fn evlist__workload_cork_fd(evlist: *const evlist) -> i32 {
    unsafe { (*evlist).workload.cork_fd }
}

#[inline]
pub unsafe fn evlist__set_workload_cork_fd(evlist: *mut evlist, cork_fd: i32) {
    unsafe { (*evlist).workload.cork_fd = cork_fd };
}

#[inline]
pub unsafe fn evlist__ctl_fd_fd(evlist: *const evlist) -> i32 {
    unsafe { (*evlist).ctl_fd.fd }
}

#[inline]
pub unsafe fn evlist__set_ctl_fd_fd(evlist: *mut evlist, fd: i32) {
    unsafe { (*evlist).ctl_fd.fd = fd };
}

#[inline]
pub unsafe fn evlist__ctl_fd_ack(evlist: *const evlist) -> i32 {
    unsafe { (*evlist).ctl_fd.ack }
}

#[inline]
pub unsafe fn evlist__set_ctl_fd_ack(evlist: *mut evlist, ack: i32) {
    unsafe { (*evlist).ctl_fd.ack = ack };
}

#[inline]
pub unsafe fn evlist__ctl_fd_pos(evlist: *const evlist) -> i32 {
    unsafe { (*evlist).ctl_fd.pos }
}

#[inline]
pub unsafe fn evlist__set_ctl_fd_pos(evlist: *mut evlist, pos: i32) {
    unsafe { (*evlist).ctl_fd.pos = pos };
}

#[inline]
pub unsafe fn evlist__refcnt(evlist: *mut evlist) -> *mut refcount_t {
    unsafe { &mut (*evlist).refcnt }
}

#[inline]
pub unsafe fn evlist__metric_events(evlist: *mut evlist) -> *mut rblist {
    unsafe { &mut (*evlist).metric_events }
}

#[inline]
pub unsafe fn evlist__deferred_samples(evlist: *mut evlist) -> *mut list_head {
    unsafe { &mut (*evlist).deferred_samples }
}

#[inline]
pub unsafe fn evlist__selected(evlist: *mut evlist) -> *mut evsel {
    unsafe { (*evlist).selected }
}

#[inline]
pub unsafe fn evlist__set_selected(evlist: *mut evlist, evsel: *mut evsel) {
    unsafe { (*evlist).selected = evsel };
}

unsafe extern "C" {
    pub fn evlist__new() -> *mut evlist;
    pub fn evlist__new_default(target: *const target, sample_callchains: bool) -> *mut evlist;
    pub fn evlist__new_dummy() -> *mut evlist;
    pub fn evlist__get(evlist: *mut evlist) -> *mut evlist;
    pub fn evlist__put(evlist: *mut evlist);
    pub fn evlist__add(evlist: *mut evlist, entry: *mut evsel);
    pub fn evlist__remove(evlist: *mut evlist, evsel: *mut evsel);
    pub fn arch_evlist__cmp(lhs: *const evsel, rhs: *const evsel) -> i32;
    pub fn arch_evlist__add_required_events(list: *mut list_head) -> i32;
    pub fn evlist__add_dummy(evlist: *mut evlist) -> i32;
    pub fn evlist__add_aux_dummy(evlist: *mut evlist, system_wide: bool) -> *mut evsel;

    // Present in C only when HAVE_LIBTRACEEVENT is defined.
    pub fn evlist__add_sched_switch(evlist: *mut evlist, system_wide: bool) -> *mut evsel;

    pub fn evlist__add_sb_event(
        evlist: *mut evlist,
        attr: *mut perf_event_attr,
        cb: evsel__sb_cb_t,
        data: *mut libc::c_void,
    ) -> i32;
    pub fn evlist__set_cb(evlist: *mut evlist, cb: evsel__sb_cb_t, data: *mut libc::c_void);
    pub fn evlist__start_sb_thread(evlist: *mut evlist, target: *mut target) -> i32;
    pub fn evlist__stop_sb_thread(evlist: *mut evlist);

    // Present in C only when HAVE_LIBTRACEEVENT is defined.
    pub fn evlist__add_newtp(
        evlist: *mut evlist,
        sys: *const libc::c_char,
        name: *const libc::c_char,
        handler: *mut libc::c_void,
    ) -> i32;

    pub fn __evlist__set_tracepoints_handlers(
        evlist: *mut evlist,
        assocs: *const evsel_str_handler,
        nr_assocs: size_t,
    ) -> i32;
    pub fn evlist__set_tp_filter(evlist: *mut evlist, filter: *const libc::c_char) -> i32;
    pub fn evlist__set_tp_filter_pids(evlist: *mut evlist, npids: size_t, pids: *mut pid_t) -> i32;
    pub fn evlist__append_tp_filter(evlist: *mut evlist, filter: *const libc::c_char) -> i32;
    pub fn evlist__append_tp_filter_pid(evlist: *mut evlist, pid: pid_t) -> i32;
    pub fn evlist__append_tp_filter_pids(evlist: *mut evlist, npids: size_t, pids: *mut pid_t) -> i32;
    pub fn evlist__find_tracepoint_by_name(evlist: *mut evlist, name: *const libc::c_char) -> *mut evsel;
    pub fn evlist__add_pollfd(evlist: *mut evlist, fd: i32) -> i32;
    pub fn evlist__filter_pollfd(evlist: *mut evlist, revents_and_mask: i16) -> i32;

    // Present in C only when HAVE_EVENTFD_SUPPORT is defined.
    pub fn evlist__add_wakeup_eventfd(evlist: *mut evlist, fd: i32) -> i32;

    pub fn evlist__poll(evlist: *mut evlist, timeout: i32) -> i32;
    pub fn evlist__id2evsel(evlist: *mut evlist, id: u64) -> *mut evsel;
    pub fn evlist__id2evsel_strict(evlist: *mut evlist, id: u64) -> *mut evsel;
    pub fn evlist__id2sid(evlist: *mut evlist, id: u64) -> *mut perf_sample_id;
    pub fn evlist__toggle_bkw_mmap(evlist: *mut evlist, state: bkw_mmap_state);
    pub fn evlist__mmap_consume(evlist: *mut evlist, idx: i32);
    pub fn evlist__open(evlist: *mut evlist) -> i32;
    pub fn evlist__close(evlist: *mut evlist);
    pub fn evlist__set_id_pos(evlist: *mut evlist);
    pub fn evlist__config(evlist: *mut evlist, opts: *mut record_opts, callchain: *mut callchain_param);
    pub fn record_opts__config(opts: *mut record_opts) -> i32;
    pub fn evlist__prepare_workload(
        evlist: *mut evlist,
        target: *mut target,
        argv: *const *const libc::c_char,
        pipe_output: bool,
        exec_error: Option<unsafe extern "C" fn(i32, *mut siginfo_t, *mut libc::c_void)>,
    ) -> i32;
    pub fn evlist__start_workload(evlist: *mut evlist) -> i32;
    pub fn evlist__cancel_workload(evlist: *mut evlist);
    pub fn __evlist__parse_mmap_pages(mmap_pages: *mut u32, str: *const libc::c_char) -> i32;
    pub fn evlist__parse_mmap_pages(opt: *const option, str: *const libc::c_char, unset: i32) -> i32;
    pub fn perf_event_mlock_kb_in_pages() -> libc::c_ulong;
    pub fn evlist__mmap_ex(
        evlist: *mut evlist,
        pages: u32,
        auxtrace_pages: u32,
        auxtrace_overwrite: bool,
        nr_cblocks: i32,
        affinity: i32,
        flush: i32,
        comp_level: i32,
    ) -> i32;
    pub fn evlist__do_mmap(evlist: *mut evlist, pages: u32) -> i32;
    pub fn evlist__do_munmap(evlist: *mut evlist);
    pub fn evlist__mmap_size(pages: libc::c_ulong) -> size_t;
    pub fn evlist__disable(evlist: *mut evlist);
    pub fn evlist__enable(evlist: *mut evlist);
    pub fn evlist__toggle_enable(evlist: *mut evlist);
    pub fn evlist__disable_evsel(evlist: *mut evlist, evsel_name: *mut libc::c_char);
    pub fn evlist__enable_evsel(evlist: *mut evlist, evsel_name: *mut libc::c_char);
    pub fn evlist__disable_non_dummy(evlist: *mut evlist);
    pub fn evlist__enable_non_dummy(evlist: *mut evlist);
    pub fn evlist__create_maps(evlist: *mut evlist, target: *mut target) -> i32;
    pub fn evlist__apply_filters(evlist: *mut evlist, err_evsel: *mut *mut evsel, target: *mut target) -> i32;
    pub fn __evlist__combined_sample_type(evlist: *mut evlist) -> u64;
    pub fn evlist__combined_sample_type(evlist: *mut evlist) -> u64;
    pub fn evlist__combined_branch_type(evlist: *mut evlist) -> u64;
    pub fn evlist__update_br_cntr(evlist: *mut evlist);
    pub fn evlist__sample_id_all(evlist: *mut evlist) -> bool;
    pub fn evlist__id_hdr_size(evlist: *mut evlist) -> u16;
    pub fn evlist__parse_sample(evlist: *mut evlist, event: *mut perf_event, sample: *mut perf_sample) -> i32;
    pub fn evlist__parse_sample_timestamp(evlist: *mut evlist, event: *mut perf_event, timestamp: *mut u64) -> i32;
    pub fn evlist__valid_sample_type(evlist: *mut evlist) -> bool;
    pub fn evlist__valid_sample_id_all(evlist: *mut evlist) -> bool;
    pub fn evlist__valid_read_format(evlist: *mut evlist) -> bool;
    pub fn evlist__splice_list_tail(evlist: *mut evlist, list: *mut list_head);
    pub fn evlist__strerror_open(evlist: *mut evlist, err: i32, buf: *mut libc::c_char, size: size_t) -> i32;
    pub fn evlist__strerror_mmap(evlist: *mut evlist, err: i32, buf: *mut libc::c_char, size: size_t) -> i32;
    pub fn evlist__can_select_event(evlist: *mut evlist, str: *const libc::c_char) -> bool;
    pub fn evlist__to_front(evlist: *mut evlist, move_evsel: *mut evsel);
}

#[inline]
pub unsafe fn evlist__add_dummy_on_all_cpus(evlist: *mut evlist) -> *mut evsel {
    unsafe { evlist__add_aux_dummy(evlist, true) }
}

// C macro equivalent:
// evlist__set_tracepoints_handlers(evlist, array) calls
// __evlist__set_tracepoints_handlers(evlist, array, ARRAY_SIZE(array)).

unsafe extern "C" {
    pub fn list_empty(head: *const list_head) -> bool;
    pub fn perf_evlist__first(evlist: *mut perf_evlist) -> *mut perf_evsel;
    pub fn perf_evlist__last(evlist: *mut perf_evlist) -> *mut perf_evsel;
    pub fn perf_evlist__nr_groups(evlist: *mut perf_evlist) -> i32;
}

#[inline]
pub unsafe fn evlist__empty(evlist: *mut evlist) -> bool {
    unsafe { list_empty(&mut (*evlist__core(evlist)).entries) }
}

#[inline]
pub unsafe fn evlist__first(evlist: *mut evlist) -> *mut evsel {
    unsafe { perf_evlist__first(evlist__core(evlist)) as *mut evsel }
}

#[inline]
pub unsafe fn evlist__last(evlist: *mut evlist) -> *mut evsel {
    unsafe { perf_evlist__last(evlist__core(evlist)) as *mut evsel }
}

#[inline]
pub unsafe fn evlist__nr_groups(evlist: *mut evlist) -> i32 {
    unsafe { perf_evlist__nr_groups(evlist__core(evlist)) }
}

/*
 * __evlist__for_each_entry(list, evsel):
 *     list_for_each_entry(evsel, list, core.node)
 * evlist__for_each_entry(evlist, evsel):
 *     __evlist__for_each_entry(&evlist__core(evlist)->entries, evsel)
 *
 * __evlist__for_each_entry_continue(list, evsel):
 *     list_for_each_entry_continue(evsel, list, core.node)
 * evlist__for_each_entry_continue(evlist, evsel):
 *     __evlist__for_each_entry_continue(&evlist__core(evlist)->entries, evsel)
 *
 * __evlist__for_each_entry_from(list, evsel):
 *     list_for_each_entry_from(evsel, list, core.node)
 * evlist__for_each_entry_from(evlist, evsel):
 *     __evlist__for_each_entry_from(&evlist__core(evlist)->entries, evsel)
 *
 * __evlist__for_each_entry_reverse(list, evsel):
 *     list_for_each_entry_reverse(evsel, list, core.node)
 * evlist__for_each_entry_reverse(evlist, evsel):
 *     __evlist__for_each_entry_reverse(&evlist__core(evlist)->entries, evsel)
 *
 * __evlist__for_each_entry_safe(list, tmp, evsel):
 *     list_for_each_entry_safe(evsel, tmp, list, core.node)
 * evlist__for_each_entry_safe(evlist, tmp, evsel):
 *     __evlist__for_each_entry_safe(&evlist__core(evlist)->entries, tmp, evsel)
 */

/** Iterator state for evlist__for_each_cpu */
#[repr(C)]
pub struct evlist_cpu_iterator {
    /** The list being iterated through. */
    pub container: *mut evlist,
    /** The current evsel of the iterator. */
    pub evsel: *mut evsel,
    /** The CPU map index corresponding to the evsel->core.cpus for the current CPU. */
    pub cpu_map_idx: i32,
    /**
     * The CPU map index corresponding to evlist->core.all_cpus for the
     * current CPU.  Distinct from cpu_map_idx as the evsel's cpu map may
     * contain fewer entries.
     */
    pub evlist_cpu_map_idx: i32,
    /** The number of CPU map entries in evlist->core.all_cpus. */
    pub evlist_cpu_map_nr: i32,
    /** The current CPU of the iterator. */
    pub cpu: perf_cpu,
    /** If present, used to set the affinity when switching between CPUs. */
    pub affinity: *mut affinity,
    /** Maybe be used to hold affinity state prior to iterating. */
    pub saved_affinity: affinity,
}

/*
 * evlist__for_each_cpu(evlist_cpu_itr, evlist):
 * for (evlist_cpu_iterator__init(&(evlist_cpu_itr), evlist);
 *      !evlist_cpu_iterator__end(&evlist_cpu_itr);
 *      evlist_cpu_iterator__next(&evlist_cpu_itr))
 */

unsafe extern "C" {
    /** Setup an iterator set to the first CPU/evsel of evlist. */
    pub fn evlist_cpu_iterator__init(itr: *mut evlist_cpu_iterator, evlist: *mut evlist);
    /**
     * Cleans up the iterator, automatically done by evlist_cpu_iterator__next when
     * the end of the list is reached. Multiple calls are safe.
     */
    pub fn evlist_cpu_iterator__exit(itr: *mut evlist_cpu_iterator);
    /** Move to next element in iterator, updating CPU, evsel and the affinity. */
    pub fn evlist_cpu_iterator__next(evlist_cpu_itr: *mut evlist_cpu_iterator);
}

#[inline]
pub unsafe fn evlist_cpu_iterator__end(evlist_cpu_itr: *const evlist_cpu_iterator) -> bool {
    unsafe { (*evlist_cpu_itr).evlist_cpu_map_idx >= (*evlist_cpu_itr).evlist_cpu_map_nr }
}

unsafe extern "C" {
    pub fn evlist__get_tracking_event(evlist: *mut evlist) -> *mut evsel;
    pub fn evlist__set_tracking_event(evlist: *mut evlist, tracking_evsel: *mut evsel);
    pub fn evlist__findnew_tracking_event(evlist: *mut evlist, system_wide: bool) -> *mut evsel;
    pub fn evlist__find_evsel_by_str(evlist: *mut evlist, str: *const libc::c_char) -> *mut evsel;
    pub fn evlist__event2evsel(evlist: *mut evlist, event: *mut perf_event) -> *mut evsel;
    pub fn evlist__exclude_kernel(evlist: *mut evlist) -> bool;
    pub fn evlist__force_leader(evlist: *mut evlist);
    pub fn evlist__reset_weak_group(evlist: *mut evlist, evsel: *mut evsel, close: bool) -> *mut evsel;
}

pub const EVLIST_CTL_CMD_ENABLE_TAG: &[u8] = b"enable\0";
pub const EVLIST_CTL_CMD_DISABLE_TAG: &[u8] = b"disable\0";
pub const EVLIST_CTL_CMD_ACK_TAG: &[u8] = b"ack\n\0";
pub const EVLIST_CTL_CMD_SNAPSHOT_TAG: &[u8] = b"snapshot\0";
pub const EVLIST_CTL_CMD_EVLIST_TAG: &[u8] = b"evlist\0";
pub const EVLIST_CTL_CMD_STOP_TAG: &[u8] = b"stop\0";
pub const EVLIST_CTL_CMD_PING_TAG: &[u8] = b"ping\0";

pub const EVLIST_CTL_CMD_MAX_LEN: usize = 64;

#[repr(C)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum evlist_ctl_cmd {
    EVLIST_CTL_CMD_UNSUPPORTED = 0,
    EVLIST_CTL_CMD_ENABLE,
    EVLIST_CTL_CMD_DISABLE,
    EVLIST_CTL_CMD_ACK,
    EVLIST_CTL_CMD_SNAPSHOT,
    EVLIST_CTL_CMD_EVLIST,
    EVLIST_CTL_CMD_STOP,
    EVLIST_CTL_CMD_PING,
}

unsafe extern "C" {
    pub fn evlist__parse_control(
        str: *const libc::c_char,
        ctl_fd: *mut i32,
        ctl_fd_ack: *mut i32,
        ctl_fd_close: *mut bool,
    ) -> i32;
    pub fn evlist__close_control(ctl_fd: i32, ctl_fd_ack: i32, ctl_fd_close: *mut bool);
    pub fn evlist__initialize_ctlfd(evlist: *mut evlist, ctl_fd: i32, ctl_fd_ack: i32) -> i32;
    pub fn evlist__finalize_ctlfd(evlist: *mut evlist) -> i32;
    pub fn evlist__ctlfd_initialized(evlist: *mut evlist) -> bool;
    pub fn evlist__ctlfd_process(evlist: *mut evlist, cmd: *mut evlist_ctl_cmd) -> i32;
    pub fn evlist__ctlfd_ack(evlist: *mut evlist) -> i32;
}

pub const EVLIST_ENABLED_MSG: &[u8] = b"Events enabled\n\0";
pub const EVLIST_DISABLED_MSG: &[u8] = b"Events disabled\n\0";

unsafe extern "C" {
    pub fn evlist__parse_event_enable_time(
        evlist: *mut evlist,
        opts: *mut record_opts,
        str: *const libc::c_char,
        unset: i32,
    ) -> i32;
    pub fn event_enable_timer__start(eet: *mut event_enable_timer) -> i32;
    pub fn event_enable_timer__process(eet: *mut event_enable_timer) -> i32;
    pub fn evlist__find_evsel(evlist: *mut evlist, idx: i32) -> *mut evsel;
    pub fn evlist__format_evsels(evlist: *mut evlist, sb: *mut strbuf, max_length: size_t);
    pub fn evlist__check_mem_load_aux(evlist: *mut evlist);
    pub fn evlist__warn_user_requested_cpus(evlist: *mut evlist, cpu_list: *const libc::c_char);
    pub fn evlist__uniquify_evsel_names(evlist: *mut evlist, config: *const perf_stat_config);
    pub fn evlist__has_bpf_output(evlist: *mut evlist) -> bool;
    pub fn evlist__needs_bpf_sb_event(evlist: *mut evlist) -> bool;
}
