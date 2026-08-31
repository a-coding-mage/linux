/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * auxtrace.h: AUX area trace support
 * Copyright (c) 2013-2015, Intel Corporation.
 */

/* Dependencies from the C header:
 * <sys/types.h>, <stdio.h>, <linux/perf_event.h>, <linux/types.h>,
 * <asm/barrier.h>, <perf/cpumap.h>
 */

use core::ffi::{c_char, c_int, c_uint, c_void};

/* Forward declarations supplied by other translation units/headers. */
pub enum perf_event {}
pub enum perf_session {}
pub enum evlist {}
pub enum evsel {}
pub enum perf_env {}
pub enum perf_tool {}
pub enum mmap {}
pub enum perf_sample {}
pub enum option {}
pub enum record_opts {}
pub enum perf_record_auxtrace_error {}
pub enum perf_record_auxtrace_info {}
pub enum events_stats {}
pub enum perf_pmu {}
pub enum auxtrace_cache {}
pub enum FILE {}

#[repr(C)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum auxtrace_error_type {
    PERF_AUXTRACE_ERROR_ITRACE = 1,
    PERF_AUXTRACE_ERROR_MAX,
}

/* Auxtrace records must have the same alignment as perf event records */
pub const PERF_AUXTRACE_RECORD_ALIGNMENT: c_int = 8;

#[repr(C)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum auxtrace_type {
    PERF_AUXTRACE_UNKNOWN,
    PERF_AUXTRACE_INTEL_PT,
    PERF_AUXTRACE_INTEL_BTS,
    PERF_AUXTRACE_CS_ETM,
    PERF_AUXTRACE_ARM_SPE,
    PERF_AUXTRACE_S390_CPUMSF,
    PERF_AUXTRACE_HISI_PTT,
    PERF_AUXTRACE_VPA_DTL,
}

#[repr(C)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum itrace_period_type {
    PERF_ITRACE_PERIOD_INSTRUCTIONS,
    PERF_ITRACE_PERIOD_TICKS,
    PERF_ITRACE_PERIOD_NANOSECS,
}

pub const AUXTRACE_ERR_FLG_OVERFLOW: c_uint = 1u32 << (b'o' - b'a');
pub const AUXTRACE_ERR_FLG_DATA_LOST: c_uint = 1u32 << (b'l' - b'a');

pub const AUXTRACE_LOG_FLG_ALL_PERF_EVTS: c_uint = 1u32 << (b'a' - b'a');
pub const AUXTRACE_LOG_FLG_ON_ERROR: c_uint = 1u32 << (b'e' - b'a');
pub const AUXTRACE_LOG_FLG_USE_STDOUT: c_uint = 1u32 << (b'o' - b'a');

/**
 * struct itrace_synth_opts - AUX area tracing synthesis options.
 * @set: indicates whether or not options have been set
 * @default_no_sample: Default to no sampling.
 * @inject: indicates the event (not just the sample) must be fully synthesized
 *          because 'perf inject' will write it out
 * @instructions: whether to synthesize 'instructions' events
 * @cycles: whether to synthesize 'cycles' events
 *          (not fully accurate, since CYC packets are only emitted
 *          together with other events, such as branches)
 * @branches: whether to synthesize 'branches' events
 * @transactions: whether to synthesize events for transactions
 * @ptwrites: whether to synthesize events for ptwrites
 * @pwr_events: whether to synthesize power events
 * @other_events: whether to synthesize other events recorded due to the use of
 *                aux_output
 * @intr_events: whether to synthesize interrupt events
 * @errors: whether to synthesize decoder error events
 * @dont_decode: whether to skip decoding entirely
 * @log: write a decoding log
 * @calls: limit branch samples to calls (can be combined with @returns)
 * @returns: limit branch samples to returns (can be combined with @calls)
 * @callchain: add callchain to 'instructions' events
 * @add_callchain: add callchain to existing event records
 * @thread_stack: feed branches to the thread_stack
 * @last_branch: add branch context to 'instruction' events
 * @add_last_branch: add branch context to existing event records
 * @approx_ipc: approximate IPC
 * @flc: whether to synthesize first level cache events
 * @llc: whether to synthesize last level cache events
 * @tlb: whether to synthesize TLB events
 * @remote_access: whether to synthesize remote access events
 * @mem: whether to synthesize memory events
 * @timeless_decoding: prefer "timeless" decoding i.e. ignore timestamps
 * @use_timestamp: use the timestamp trace as kernel time
 * @vm_time_correlation: perform VM Time Correlation
 * @vm_tm_corr_dry_run: VM Time Correlation dry-run
 * @vm_tm_corr_args:  VM Time Correlation implementation-specific arguments
 * @callchain_sz: maximum callchain size
 * @last_branch_sz: branch context size
 * @period: 'instructions' events period
 * @period_type: 'instructions' events period type
 * @initial_skip: skip N events at the beginning.
 * @cpu_bitmap: CPUs for which to synthesize events, or NULL for all
 * @ptime_range: time intervals to trace or NULL
 * @range_num: number of time intervals to trace
 * @error_plus_flags: flags to affect what errors are reported
 * @error_minus_flags: flags to affect what errors are reported
 * @log_plus_flags: flags to affect what is logged
 * @log_minus_flags: flags to affect what is logged
 * @quick: quicker (less detailed) decoding
 * @log_on_error_size: size of log to keep for outputting log only on errors
 */
#[repr(C)]
pub struct itrace_synth_opts {
    pub set: bool,
    pub default_no_sample: bool,
    pub inject: bool,
    pub instructions: bool,
    pub cycles: bool,
    pub branches: bool,
    pub transactions: bool,
    pub ptwrites: bool,
    pub pwr_events: bool,
    pub other_events: bool,
    pub intr_events: bool,
    pub errors: bool,
    pub dont_decode: bool,
    pub log: bool,
    pub calls: bool,
    pub returns: bool,
    pub callchain: bool,
    pub add_callchain: bool,
    pub thread_stack: bool,
    pub last_branch: bool,
    pub add_last_branch: bool,
    pub approx_ipc: bool,
    pub flc: bool,
    pub llc: bool,
    pub tlb: bool,
    pub remote_access: bool,
    pub mem: bool,
    pub timeless_decoding: bool,
    pub use_timestamp: bool,
    pub vm_time_correlation: bool,
    pub vm_tm_corr_dry_run: bool,
    pub vm_tm_corr_args: *mut c_char,
    pub callchain_sz: c_uint,
    pub last_branch_sz: c_uint,
    pub period: u64,
    pub period_type: itrace_period_type,
    pub initial_skip: usize,
    pub cpu_bitmap: *mut usize,
    pub ptime_range: *mut perf_time_interval,
    pub range_num: c_int,
    pub error_plus_flags: c_uint,
    pub error_minus_flags: c_uint,
    pub log_plus_flags: c_uint,
    pub log_minus_flags: c_uint,
    pub quick: c_uint,
    pub log_on_error_size: c_uint,
}

/**
 * struct auxtrace_index_entry - indexes a AUX area tracing event within a
 *                               perf.data file.
 * @file_offset: offset within the perf.data file
 * @sz: size of the event
 */
#[repr(C)]
pub struct auxtrace_index_entry {
    pub file_offset: u64,
    pub sz: u64,
}

pub const PERF_AUXTRACE_INDEX_ENTRY_COUNT: usize = 256;

/**
 * struct auxtrace_index - index of AUX area tracing events within a perf.data
 *                         file.
 * @list: linking a number of arrays of entries
 * @nr: number of entries
 * @entries: array of entries
 */
#[repr(C)]
pub struct auxtrace_index {
    pub list: list_head,
    pub nr: usize,
    pub entries: [auxtrace_index_entry; PERF_AUXTRACE_INDEX_ENTRY_COUNT],
}

/**
 * struct auxtrace - session callbacks to allow AUX area data decoding.
 * @process_event: lets the decoder see all session events
 * @process_auxtrace_event: process a PERF_RECORD_AUXTRACE event
 * @queue_data: queue an AUX sample or PERF_RECORD_AUXTRACE event for later
 *              processing
 * @dump_auxtrace_sample: dump AUX area sample data
 * @flush_events: process any remaining data
 * @free_events: free resources associated with event processing
 * @free: free resources associated with the session
 */
#[repr(C)]
pub struct auxtrace {
    pub process_event: Option<unsafe extern "C" fn(*mut perf_session, *mut perf_event, *mut perf_sample, *const perf_tool) -> c_int>,
    pub process_auxtrace_event: Option<unsafe extern "C" fn(*mut perf_session, *mut perf_event, *const perf_tool) -> c_int>,
    pub queue_data: Option<unsafe extern "C" fn(*mut perf_session, *mut perf_sample, *mut perf_event, u64) -> c_int>,
    pub dump_auxtrace_sample: Option<unsafe extern "C" fn(*mut perf_session, *mut perf_sample)>,
    pub flush_events: Option<unsafe extern "C" fn(*mut perf_session, *const perf_tool) -> c_int>,
    pub free_events: Option<unsafe extern "C" fn(*mut perf_session)>,
    pub free: Option<unsafe extern "C" fn(*mut perf_session)>,
    pub evsel_is_auxtrace: Option<unsafe extern "C" fn(*mut perf_session, *mut evsel) -> bool>,
}

/**
 * struct auxtrace_buffer - a buffer containing AUX area tracing data.
 * @list: buffers are queued in a list held by struct auxtrace_queue
 * @size: size of the buffer in bytes
 * @pid: in per-thread mode, the pid this buffer is associated with
 * @tid: in per-thread mode, the tid this buffer is associated with
 * @cpu: in per-cpu mode, the cpu this buffer is associated with
 * @data: actual buffer data (can be null if the data has not been loaded)
 * @data_offset: file offset at which the buffer can be read
 * @mmap_addr: mmap address at which the buffer can be read
 * @mmap_size: size of the mmap at @mmap_addr
 * @data_needs_freeing: @data was malloc'd so free it when it is no longer
 *                      needed
 * @consecutive: the original data was split up and this buffer is consecutive
 *               to the previous buffer
 * @offset: offset as determined by aux_head / aux_tail members of struct
 *          perf_event_mmap_page
 * @reference: an implementation-specific reference determined when the data is
 *             recorded
 * @buffer_nr: used to number each buffer
 * @use_size: implementation actually only uses this number of bytes
 * @use_data: implementation actually only uses data starting at this address
 */
#[repr(C)]
pub struct auxtrace_buffer {
    pub list: list_head,
    pub size: usize,
    pub pid: pid_t,
    pub tid: pid_t,
    pub cpu: perf_cpu,
    pub data: *mut c_void,
    pub data_offset: off_t,
    pub mmap_addr: *mut c_void,
    pub mmap_size: usize,
    pub data_needs_freeing: bool,
    pub consecutive: bool,
    pub offset: u64,
    pub reference: u64,
    pub buffer_nr: u64,
    pub use_size: usize,
    pub use_data: *mut c_void,
}

/**
 * struct auxtrace_queue - a queue of AUX area tracing data buffers.
 * @head: head of buffer list
 * @tid: in per-thread mode, the tid this queue is associated with
 * @cpu: in per-cpu mode, the cpu this queue is associated with
 * @set: %true once this queue has been dedicated to a specific thread or cpu
 * @priv: implementation-specific data
 */
#[repr(C)]
pub struct auxtrace_queue {
    pub head: list_head,
    pub tid: pid_t,
    pub cpu: c_int,
    pub set: bool,
    pub priv_: *mut c_void,
}

/**
 * struct auxtrace_queues - an array of AUX area tracing queues.
 * @queue_array: array of queues
 * @nr_queues: number of queues
 * @new_data: set whenever new data is queued
 * @populated: queues have been fully populated using the auxtrace_index
 * @next_buffer_nr: used to number each buffer
 */
#[repr(C)]
pub struct auxtrace_queues {
    pub queue_array: *mut auxtrace_queue,
    pub nr_queues: c_uint,
    pub new_data: bool,
    pub populated: bool,
    pub next_buffer_nr: u64,
}

/**
 * struct auxtrace_heap_item - element of struct auxtrace_heap.
 * @queue_nr: queue number
 * @ordinal: value used for sorting (lowest ordinal is top of the heap) expected
 *           to be a timestamp
 */
#[repr(C)]
pub struct auxtrace_heap_item {
    pub queue_nr: c_uint,
    pub ordinal: u64,
}

/**
 * struct auxtrace_heap - a heap suitable for sorting AUX area tracing queues.
 * @heap_array: the heap
 * @heap_cnt: the number of elements in the heap
 * @heap_sz: maximum number of elements (grows as needed)
 */
#[repr(C)]
pub struct auxtrace_heap {
    pub heap_array: *mut auxtrace_heap_item,
    pub heap_cnt: c_uint,
    pub heap_sz: c_uint,
}

/**
 * struct auxtrace_mmap - records an mmap of the auxtrace buffer.
 * @base: address of mapped area
 * @userpg: pointer to buffer's perf_event_mmap_page
 * @mask: %0 if @len is not a power of two, otherwise (@len - %1)
 * @len: size of mapped area
 * @prev: previous aux_head
 * @idx: index of this mmap
 * @tid: tid for a per-thread mmap (also set if there is only 1 tid on a per-cpu
 *       mmap) otherwise %0
 * @cpu: cpu number for a per-cpu mmap otherwise %-1
 */
#[repr(C)]
pub struct auxtrace_mmap {
    pub base: *mut c_void,
    pub userpg: *mut c_void,
    pub mask: usize,
    pub len: usize,
    pub prev: u64,
    pub idx: c_int,
    pub tid: pid_t,
    pub cpu: c_int,
}

/**
 * struct auxtrace_mmap_params - parameters to set up struct auxtrace_mmap.
 * @mask: %0 if @len is not a power of two, otherwise (@len - %1)
 * @offset: file offset of mapped area
 * @len: size of mapped area
 * @prot: mmap memory protection
 * @idx: index of this mmap
 * @tid: tid for a per-thread mmap (also set if there is only 1 tid on a per-cpu
 *       mmap) otherwise %0
 * @mmap_needed: set to %false for non-auxtrace events. This is needed because
 *               auxtrace mmapping is done in the same code path as non-auxtrace
 *               mmapping but not every evsel that needs non-auxtrace mmapping
 *               also needs auxtrace mmapping.
 * @cpu: cpu number for a per-cpu mmap otherwise %-1
 */
#[repr(C)]
pub struct auxtrace_mmap_params {
    pub mask: usize,
    pub offset: off_t,
    pub len: usize,
    pub prot: c_int,
    pub idx: c_int,
    pub tid: pid_t,
    pub mmap_needed: bool,
    pub cpu: perf_cpu,
}

/**
 * struct auxtrace_record - callbacks for recording AUX area data.
 * @recording_options: validate and process recording options
 * @info_priv_size: return the size of the private data in auxtrace_info_event
 * @info_fill: fill-in the private data in auxtrace_info_event
 * @free: free this auxtrace record structure
 * @snapshot_start: starting a snapshot
 * @snapshot_finish: finishing a snapshot
 * @find_snapshot: find data to snapshot within auxtrace mmap
 * @parse_snapshot_options: parse snapshot options
 * @reference: provide a 64-bit reference number for auxtrace_event
 * @read_finish: called after reading from an auxtrace mmap
 * @alignment: alignment (if any) for AUX area data
 * @default_aux_sample_size: default sample size for --aux sample option
 * @pmu: associated pmu
 * @evlist: selected events list
 */
#[repr(C)]
pub struct auxtrace_record {
    pub recording_options: Option<unsafe extern "C" fn(*mut auxtrace_record, *mut evlist, *mut record_opts) -> c_int>,
    pub info_priv_size: Option<unsafe extern "C" fn(*mut auxtrace_record, *mut evlist) -> usize>,
    pub info_fill: Option<unsafe extern "C" fn(*mut auxtrace_record, *mut perf_session, *mut perf_record_auxtrace_info, usize) -> c_int>,
    pub free: Option<unsafe extern "C" fn(*mut auxtrace_record)>,
    pub snapshot_start: Option<unsafe extern "C" fn(*mut auxtrace_record) -> c_int>,
    pub snapshot_finish: Option<unsafe extern "C" fn(*mut auxtrace_record) -> c_int>,
    pub find_snapshot: Option<unsafe extern "C" fn(*mut auxtrace_record, c_int, *mut auxtrace_mmap, *mut u8, *mut u64, *mut u64) -> c_int>,
    pub parse_snapshot_options: Option<unsafe extern "C" fn(*mut auxtrace_record, *mut record_opts, *const c_char) -> c_int>,
    pub reference: Option<unsafe extern "C" fn(*mut auxtrace_record) -> u64>,
    pub read_finish: Option<unsafe extern "C" fn(*mut auxtrace_record, c_int) -> c_int>,
    pub alignment: c_uint,
    pub default_aux_sample_size: c_uint,
    pub evlist: *mut evlist,
}

/**
 * struct addr_filter - address filter.
 * @list: list node
 * @range: true if it is a range filter
 * @start: true if action is 'filter' or 'start'
 * @action: 'filter', 'start' or 'stop' ('tracestop' is accepted but converted
 *          to 'stop')
 * @sym_from: symbol name for the filter address
 * @sym_to: symbol name that determines the filter size
 * @sym_from_idx: selects n'th from symbols with the same name (0 means global
 *                and less than 0 means symbol must be unique)
 * @sym_to_idx: same as @sym_from_idx but for @sym_to
 * @addr: filter address
 * @size: filter region size (for range filters)
 * @filename: DSO file name or NULL for the kernel
 * @str: allocated string that contains the other string members
 */
#[repr(C)]
pub struct addr_filter {
    pub list: list_head,
    pub range: bool,
    pub start: bool,
    pub action: *const c_char,
    pub sym_from: *const c_char,
    pub sym_to: *const c_char,
    pub sym_from_idx: c_int,
    pub sym_to_idx: c_int,
    pub addr: u64,
    pub size: u64,
    pub filename: *const c_char,
    pub str_: *mut c_char,
}

/**
 * struct addr_filters - list of address filters.
 * @head: list of address filters
 * @cnt: number of address filters
 */
#[repr(C)]
pub struct addr_filters {
    pub head: list_head,
    pub cnt: c_int,
}

unsafe extern "C" {
    pub fn compat_auxtrace_mmap__read_head(mm: *mut auxtrace_mmap) -> u64;
    pub fn compat_auxtrace_mmap__write_tail(mm: *mut auxtrace_mmap, tail: u64) -> c_int;
}

#[inline]
pub unsafe fn auxtrace_mmap__read_head(mm: *mut auxtrace_mmap, kernel_is_64_bit: c_int) -> u64 {
    let pc = unsafe { (*mm).userpg as *mut perf_event_mmap_page };
    let head: u64;

    /* Original C condition: #if BITS_PER_LONG == 32 */
    #[cfg(target_pointer_width = "32")]
    {
        if kernel_is_64_bit != 0 {
            return unsafe { compat_auxtrace_mmap__read_head(mm) };
        }
    }
    #[cfg(not(target_pointer_width = "32"))]
    {
        let _ = kernel_is_64_bit;
    }

    head = unsafe { core::ptr::read_volatile(core::ptr::addr_of!((*pc).aux_head)) };

    /* Ensure all reads are done after we read the head */
    core::sync::atomic::fence(core::sync::atomic::Ordering::Acquire);
    head
}

#[inline]
pub unsafe fn auxtrace_mmap__write_tail(
    mm: *mut auxtrace_mmap,
    tail: u64,
    kernel_is_64_bit: c_int,
) -> c_int {
    let pc = unsafe { (*mm).userpg as *mut perf_event_mmap_page };

    /* Original C condition: #if BITS_PER_LONG == 32 */
    #[cfg(target_pointer_width = "32")]
    {
        if kernel_is_64_bit != 0 {
            return unsafe { compat_auxtrace_mmap__write_tail(mm, tail) };
        }
    }
    #[cfg(not(target_pointer_width = "32"))]
    {
        let _ = kernel_is_64_bit;
    }

    /* Ensure all reads are done before we write the tail out */
    core::sync::atomic::fence(core::sync::atomic::Ordering::SeqCst);
    unsafe { core::ptr::write_volatile(core::ptr::addr_of_mut!((*pc).aux_tail), tail) };
    0
}

pub type process_auxtrace_t = Option<
    unsafe extern "C" fn(
        *const perf_tool,
        *mut mmap,
        *mut perf_event,
        *mut c_void,
        usize,
        *mut c_void,
        usize,
    ) -> c_int,
>;

unsafe extern "C" {
    pub fn auxtrace_mmap__mmap(
        mm: *mut auxtrace_mmap,
        mp: *mut auxtrace_mmap_params,
        userpg: *mut c_void,
        fd: c_int,
    ) -> c_int;
    pub fn auxtrace_mmap__munmap(mm: *mut auxtrace_mmap);
    pub fn auxtrace_mmap_params__init(
        mp: *mut auxtrace_mmap_params,
        auxtrace_offset: off_t,
        auxtrace_pages: c_uint,
        auxtrace_overwrite: bool,
    );
    pub fn auxtrace_mmap_params__set_idx(
        mp: *mut auxtrace_mmap_params,
        evlist: *mut evlist,
        evsel: *mut evsel,
        idx: c_int,
    );
    pub fn auxtrace_mmap__read(
        map: *mut mmap,
        itr: *mut auxtrace_record,
        env: *mut perf_env,
        tool: *const perf_tool,
        fn_: process_auxtrace_t,
    ) -> c_int;
    pub fn auxtrace_mmap__read_snapshot(
        map: *mut mmap,
        itr: *mut auxtrace_record,
        env: *mut perf_env,
        tool: *const perf_tool,
        fn_: process_auxtrace_t,
        snapshot_size: usize,
    ) -> c_int;
    pub fn auxtrace_queues__init_nr(queues: *mut auxtrace_queues, nr_queues: c_int) -> c_int;
    pub fn auxtrace_queues__init(queues: *mut auxtrace_queues) -> c_int;
    pub fn auxtrace_queues__add_event(
        queues: *mut auxtrace_queues,
        session: *mut perf_session,
        event: *mut perf_event,
        data_offset: off_t,
        buffer_ptr: *mut *mut auxtrace_buffer,
    ) -> c_int;
    pub fn auxtrace_queues__sample_queue(
        queues: *mut auxtrace_queues,
        sample: *mut perf_sample,
        session: *mut perf_session,
    ) -> *mut auxtrace_queue;
    pub fn auxtrace_queues__add_sample(
        queues: *mut auxtrace_queues,
        session: *mut perf_session,
        sample: *mut perf_sample,
        data_offset: u64,
        reference: u64,
    ) -> c_int;
    pub fn auxtrace_queues__free(queues: *mut auxtrace_queues);
    pub fn auxtrace_queues__process_index(
        queues: *mut auxtrace_queues,
        session: *mut perf_session,
    ) -> c_int;
    pub fn auxtrace_queue_data(session: *mut perf_session, samples: bool, events: bool) -> c_int;
    pub fn auxtrace_buffer__next(
        queue: *mut auxtrace_queue,
        buffer: *mut auxtrace_buffer,
    ) -> *mut auxtrace_buffer;
    pub fn auxtrace_buffer__get_data_rw(
        buffer: *mut auxtrace_buffer,
        fd: c_int,
        rw: bool,
    ) -> *mut c_void;
}

#[inline]
pub unsafe fn auxtrace_buffer__get_data(buffer: *mut auxtrace_buffer, fd: c_int) -> *mut c_void {
    unsafe { auxtrace_buffer__get_data_rw(buffer, fd, false) }
}

unsafe extern "C" {
    pub fn auxtrace_buffer__put_data(buffer: *mut auxtrace_buffer);
    pub fn auxtrace_buffer__drop_data(buffer: *mut auxtrace_buffer);
    pub fn auxtrace_buffer__free(buffer: *mut auxtrace_buffer);

    pub fn auxtrace_heap__add(heap: *mut auxtrace_heap, queue_nr: c_uint, ordinal: u64) -> c_int;
    pub fn auxtrace_heap__pop(heap: *mut auxtrace_heap);
    pub fn auxtrace_heap__free(heap: *mut auxtrace_heap);
}

#[repr(C)]
pub struct auxtrace_cache_entry {
    pub hash: hlist_node,
    pub key: u32,
}

unsafe extern "C" {
    pub fn auxtrace_cache__new(
        bits: c_uint,
        entry_size: usize,
        limit_percent: c_uint,
    ) -> *mut auxtrace_cache;
    pub fn auxtrace_cache__free(auxtrace_cache: *mut auxtrace_cache);
    pub fn auxtrace_cache__alloc_entry(c: *mut auxtrace_cache) -> *mut c_void;
    pub fn auxtrace_cache__free_entry(c: *mut auxtrace_cache, entry: *mut c_void);
    pub fn auxtrace_cache__add(
        c: *mut auxtrace_cache,
        key: u32,
        entry: *mut auxtrace_cache_entry,
    ) -> c_int;
    pub fn auxtrace_cache__remove(c: *mut auxtrace_cache, key: u32);
    pub fn auxtrace_cache__lookup(c: *mut auxtrace_cache, key: u32) -> *mut c_void;

    pub fn auxtrace_record__init(evlist: *mut evlist, err: *mut c_int) -> *mut auxtrace_record;

    pub fn auxtrace_parse_snapshot_options(
        itr: *mut auxtrace_record,
        opts: *mut record_opts,
        str_: *const c_char,
    ) -> c_int;
    pub fn auxtrace_parse_sample_options(
        itr: *mut auxtrace_record,
        evlist: *mut evlist,
        opts: *mut record_opts,
        str_: *const c_char,
    ) -> c_int;
    pub fn auxtrace_parse_aux_action(evlist: *mut evlist) -> c_int;
    pub fn auxtrace_record__options(
        itr: *mut auxtrace_record,
        evlist: *mut evlist,
        opts: *mut record_opts,
    ) -> c_int;
    pub fn auxtrace_record__info_priv_size(itr: *mut auxtrace_record, evlist: *mut evlist) -> usize;
    pub fn auxtrace_record__info_fill(
        itr: *mut auxtrace_record,
        session: *mut perf_session,
        auxtrace_info: *mut perf_record_auxtrace_info,
        priv_size: usize,
    ) -> c_int;
    pub fn auxtrace_record__free(itr: *mut auxtrace_record);
    pub fn auxtrace_record__snapshot_start(itr: *mut auxtrace_record) -> c_int;
    pub fn auxtrace_record__snapshot_finish(itr: *mut auxtrace_record, on_exit: bool) -> c_int;
    pub fn auxtrace_record__find_snapshot(
        itr: *mut auxtrace_record,
        idx: c_int,
        mm: *mut auxtrace_mmap,
        data: *mut u8,
        head: *mut u64,
        old: *mut u64,
    ) -> c_int;
    pub fn auxtrace_record__reference(itr: *mut auxtrace_record) -> u64;
    pub fn auxtrace_record__read_finish(itr: *mut auxtrace_record, idx: c_int) -> c_int;

    pub fn auxtrace_index__auxtrace_event(
        head: *mut list_head,
        event: *mut perf_event,
        file_offset: off_t,
    ) -> c_int;
    pub fn auxtrace_index__write(fd: c_int, head: *mut list_head) -> c_int;
    pub fn auxtrace_index__process(
        fd: c_int,
        size: u64,
        session: *mut perf_session,
        needs_swap: bool,
    ) -> c_int;
    pub fn auxtrace_index__free(head: *mut list_head);

    pub fn auxtrace_synth_guest_error(
        auxtrace_error: *mut perf_record_auxtrace_error,
        type_: c_int,
        code: c_int,
        cpu: c_int,
        pid: pid_t,
        tid: pid_t,
        ip: u64,
        msg: *const c_char,
        timestamp: u64,
        machine_pid: pid_t,
        vcpu: c_int,
    );
    pub fn auxtrace_synth_error(
        auxtrace_error: *mut perf_record_auxtrace_error,
        type_: c_int,
        code: c_int,
        cpu: c_int,
        pid: pid_t,
        tid: pid_t,
        ip: u64,
        msg: *const c_char,
        timestamp: u64,
    );

    pub fn perf_event__process_auxtrace_info(
        tool: *const perf_tool,
        session: *mut perf_session,
        event: *mut perf_event,
    ) -> c_int;
    pub fn perf_event__process_auxtrace(
        tool: *const perf_tool,
        session: *mut perf_session,
        event: *mut perf_event,
    ) -> s64;
    pub fn perf_event__process_auxtrace_error(
        tool: *const perf_tool,
        session: *mut perf_session,
        event: *mut perf_event,
    ) -> c_int;
    pub fn itrace_do_parse_synth_opts(
        synth_opts: *mut itrace_synth_opts,
        str_: *const c_char,
        unset: c_int,
    ) -> c_int;
    pub fn itrace_parse_synth_opts(opt: *const option, str_: *const c_char, unset: c_int) -> c_int;
    pub fn itrace_synth_opts__set_default(synth_opts: *mut itrace_synth_opts, no_sample: bool);

    pub fn perf_event__fprintf_auxtrace_error(event: *mut perf_event, fp: *mut FILE) -> usize;
    pub fn perf_session__auxtrace_error_inc(session: *mut perf_session, event: *mut perf_event);
    pub fn events_stats__auxtrace_error_warn(stats: *const events_stats);

    pub fn addr_filters__init(filts: *mut addr_filters);
    pub fn addr_filters__exit(filts: *mut addr_filters);
    pub fn addr_filters__parse_bare_filter(
        filts: *mut addr_filters,
        filter: *const c_char,
    ) -> c_int;
    pub fn auxtrace_parse_filters(evlist: *mut evlist) -> c_int;

    pub fn auxtrace__process_event(
        session: *mut perf_session,
        event: *mut perf_event,
        sample: *mut perf_sample,
        tool: *const perf_tool,
    ) -> c_int;
    pub fn auxtrace__dump_auxtrace_sample(session: *mut perf_session, sample: *mut perf_sample);
    pub fn auxtrace__flush_events(session: *mut perf_session, tool: *const perf_tool) -> c_int;
    pub fn auxtrace__free_events(session: *mut perf_session);
    pub fn auxtrace__free(session: *mut perf_session);
    pub fn auxtrace__evsel_is_auxtrace(session: *mut perf_session, evsel: *mut evsel) -> bool;
    pub fn auxtrace_synth_id_range_start(evsel: *mut evsel) -> u64;
}

pub const ITRACE_HELP: &str =
    "\t\t\t\ti[period]:    \t\tsynthesize instructions events\n\
\t\t\t\ty[period]:    \t\tsynthesize cycles events (same period as i)\n\
\t\t\t\tb:\t    \t\tsynthesize branches events\n\
\t\t\t\tc:\t    \t\tsynthesize branches events (calls only)\n\
\t\t\t\tr:\t    \t\tsynthesize branches events (returns only)\n\
\t\t\t\tx:\t    \t\tsynthesize transactions events\n\
\t\t\t\tw:\t    \t\tsynthesize ptwrite events\n\
\t\t\t\tp:\t    \t\tsynthesize power events\n\
\t\t\t\to:\t\t\tsynthesize other events recorded due to the use\n\
\t\t\t\t\t\t\tof aux-output (refer to perf record)\n\
\t\t\t\tI:\t\t\tsynthesize interrupt or similar (asynchronous) events\n\
\t\t\t\t\t\t\t(e.g. Intel PT Event Trace)\n\
\t\t\t\te[flags]:\t\tsynthesize error events\n\
\t\t\t\t\t\t\teach flag must be preceded by + or -\n\
\t\t\t\t\t\t\terror flags are: o (overflow)\n\
\t\t\t\t\t\t\t\t\t l (data lost)\n\
\t\t\t\td[flags]:\t\tcreate a debug log\n\
\t\t\t\t\t\t\teach flag must be preceded by + or -\n\
\t\t\t\t\t\t\tlog flags are: a (all perf events)\n\
\t\t\t\t\t\t\t               o (output to stdout)\n\
\t\t\t\tf:\t    \t\tsynthesize first level cache events\n\
\t\t\t\tm:\t    \t\tsynthesize last level cache events\n\
\t\t\t\tt:\t    \t\tsynthesize TLB events\n\
\t\t\t\ta:\t    \t\tsynthesize remote access events\n\
\t\t\t\tg[len]:     \t\tsynthesize a call chain (use with i or x)\n\
\t\t\t\tG[len]:\t\t\tsynthesize a call chain on existing event records\n\
\t\t\t\tl[len]:     \t\tsynthesize last branch entries (use with i or x)\n\
\t\t\t\tL[len]:\t\t\tsynthesize last branch entries on existing event records\n\
\t\t\t\tsNUMBER:    \t\tskip initial number of events\n\
\t\t\t\tq:\t\t\tquicker (less detailed) decoding\n\
\t\t\t\tA:\t\t\tapproximate IPC\n\
\t\t\t\tZ:\t\t\tprefer to ignore timestamps (so-called \"timeless\" decoding)\n\
\t\t\t\tT:\t\t\tuse the timestamp trace as kernel time\n\
\t\t\t\tPERIOD[ns|us|ms|i|t]:   specify period to sample stream\n\
\t\t\t\tconcatenate multiple options. Default is iybxwpe or cewp\n";

#[inline]
pub unsafe fn itrace_synth_opts__set_time_range(
    opts: *mut itrace_synth_opts,
    ptime_range: *mut perf_time_interval,
    range_num: c_int,
) {
    unsafe {
        (*opts).ptime_range = ptime_range;
        (*opts).range_num = range_num;
    }
}

#[inline]
pub unsafe fn itrace_synth_opts__clear_time_range(opts: *mut itrace_synth_opts) {
    unsafe {
        (*opts).ptime_range = core::ptr::null_mut();
        (*opts).range_num = 0;
    }
}
