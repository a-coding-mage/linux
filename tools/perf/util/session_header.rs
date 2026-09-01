/* SPDX-License-Identifier: GPL-2.0 */

/* Translated from perf/util/session.h. */
/* Dependencies from the original includes:
 * trace-event.h, event.h, header.h, machine.h, data.h, ordered-events.h,
 * util/compress.h, linux/kernel.h, linux/rbtree.h, linux/perf_event.h.
 */

use core::ffi::{c_char, c_int, c_uint, c_ulong, c_void};

pub type u32 = u32;
pub type u64 = u64;
pub type pid_t = c_int;
pub type off_t = i64;
pub type size_t = usize;
pub type uint16_t = u16;
pub type uint32_t = u32;

/* Forward declarations from this header and included headers. */
pub enum ip_callchain {}
pub enum symbol {}
pub enum thread {}
pub enum auxtrace {}
pub enum itrace_synth_opts {}
pub enum perf_tool {}
pub enum perf_env {}
pub enum evsel {}
pub enum perf_sample {}
pub enum perf_event {}
pub enum perf_event_header {}
pub enum perf_event_attr {}
pub enum dso {}
pub enum FILE {}

#[repr(C)]
pub struct decomp_data {
    pub decomp: *mut decomp,
    pub decomp_last: *mut decomp,
    pub zstd_decomp: *mut zstd_data,
}

/**
 * struct perf_session- A Perf session holds the main state when the program is
 * working with live perf events or reading data from an input file.
 *
 * The rough organization of a perf_session is:
 * ```
 * +--------------+           +-----------+           +------------+
 * |   Session    |1..* ----->|  Machine  |1..* ----->|   Thread   |
 * +--------------+           +-----------+           +------------+
 * ```
 */
#[repr(C)]
pub struct perf_session {
    /**
     * @header: The read version of a perf_file_header, or captures global
     * information from a live session.
     */
    pub header: perf_header,
    /** @machines: Machines within the session a host and 0 or more guests. */
    pub machines: machines,
    /** @evlist: List of evsels/events of the session. */
    pub evlist: *mut evlist,
    /** @auxtrace: callbacks to allow AUX area data decoding. */
    pub auxtrace: *const auxtrace,
    /** @itrace_synth_opts: AUX area tracing synthesis options. */
    pub itrace_synth_opts: *mut itrace_synth_opts,
    /** @auxtrace_index: index of AUX area tracing events within a perf.data file. */
    pub auxtrace_index: list_head,
    /* Original C condition: #ifdef HAVE_LIBTRACEEVENT. */
    #[cfg(HAVE_LIBTRACEEVENT)]
    /** @tevent: handles for libtraceevent and plugins. */
    pub tevent: trace_event,
    /** @time_conv: Holds contents of last PERF_RECORD_TIME_CONV event. */
    pub time_conv: perf_record_time_conv,
    /** @trace_event_repipe: When set causes read trace events to be written to stdout. */
    pub trace_event_repipe: bool,
    /**
     * @one_mmap: The reader will use a single mmap by default. There may be
     * multiple data files in particular for aux events. If this is true
     * then the single big mmap for the data file can be assumed.
     */
    pub one_mmap: bool,
    /** @one_mmap_addr: Address of initial perf data file reader mmap. */
    pub one_mmap_addr: *mut c_void,
    /** @one_mmap_offset: File offset in perf.data file when mapped. */
    pub one_mmap_offset: u64,
    /** @one_mmap_size: Size of the single mmap in bytes. */
    pub one_mmap_size: u64,
    /** @ordered_events: Used to turn unordered events into ordered ones. */
    pub ordered_events: ordered_events,
    /** @data: Optional perf data file being read from. */
    pub data: *mut perf_data,
    /** @tool: callbacks for event handling. */
    pub tool: *const perf_tool,
    /**
     * @bytes_transferred: Used by perf record to count written bytes before
     * compression.
     */
    pub bytes_transferred: u64,
    /**
     * @bytes_compressed: Used by perf record to count written bytes after
     * compression.
     */
    pub bytes_compressed: u64,
    /** @zstd_data: Owner of global compression state, buffers, etc. */
    pub zstd_data: zstd_data,
    pub decomp_data: decomp_data,
    pub active_decomp: *mut decomp_data,
}

#[repr(C)]
pub struct decomp {
    pub next: *mut decomp,
    pub file_pos: u64,
    pub file_path: *const c_char,
    pub mmap_len: size_t,
    pub head: u64,
    pub size: size_t,
    /* Flexible array member: char data[]; */
    pub data: [c_char; 0],
}

pub type peek_events_cb_t = Option<
    unsafe extern "C" fn(
        session: *mut perf_session,
        event: *mut perf_event,
        offset: u64,
        data: *mut c_void,
    ) -> c_int,
>;

pub type perf_session_dso_buildid_fn_t =
    Option<unsafe extern "C" fn(dso: *mut dso, parm: c_int) -> bool>;

unsafe extern "C" {
    pub fn __perf_session__new(
        data: *mut perf_data,
        tool: *mut perf_tool,
        trace_event_repipe: bool,
        host_env: *mut perf_env,
    ) -> *mut perf_session;

    pub fn perf_session__delete(session: *mut perf_session);

    pub fn perf_event_header__bswap(hdr: *mut perf_event_header);

    pub fn perf_event__too_small(event: *const perf_event, min: *mut u32) -> bool;

    pub fn perf_session__peek_event(
        session: *mut perf_session,
        file_offset: off_t,
        buf: *mut c_void,
        buf_sz: size_t,
        event_ptr: *mut *mut perf_event,
        sample: *mut perf_sample,
    ) -> c_int;

    pub fn perf_session__peek_events(
        session: *mut perf_session,
        offset: u64,
        size: u64,
        cb: peek_events_cb_t,
        data: *mut c_void,
    ) -> c_int;

    pub fn perf_session__process_events(session: *mut perf_session) -> c_int;

    pub fn perf_session__queue_event(
        s: *mut perf_session,
        event: *mut perf_event,
        timestamp: u64,
        file_offset: u64,
        file_path: *const c_char,
    ) -> c_int;

    pub fn perf_session__resolve_callchain(
        session: *mut perf_session,
        evsel: *mut evsel,
        thread: *mut thread,
        chain: *mut ip_callchain,
        parent: *mut *mut symbol,
    ) -> c_int;

    pub fn perf_session__has_traces(session: *mut perf_session, msg: *const c_char) -> bool;
    pub fn perf_session__has_switch_events(session: *mut perf_session) -> bool;

    pub fn perf_event__attr_swap(attr: *mut perf_event_attr);

    pub fn perf_session__create_kernel_maps(session: *mut perf_session) -> c_int;

    pub fn perf_session__set_id_hdr_size(session: *mut perf_session);

    pub fn machines__find(machines: *mut machines, pid: pid_t) -> *mut machine;
    pub fn machines__findnew(machines: *mut machines, pid: pid_t) -> *mut machine;

    pub fn perf_session__findnew(session: *mut perf_session, pid: pid_t) -> *mut thread;
    pub fn perf_session__register_idle_thread(session: *mut perf_session) -> c_int;

    pub fn perf_session__fprintf(session: *mut perf_session, fp: *mut FILE) -> size_t;

    pub fn perf_session__fprintf_dsos(session: *mut perf_session, fp: *mut FILE) -> size_t;

    pub fn perf_session__fprintf_dsos_buildid(
        session: *mut perf_session,
        fp: *mut FILE,
        fn_: perf_session_dso_buildid_fn_t,
        parm: c_int,
    ) -> size_t;

    pub fn perf_session__fprintf_nr_events(session: *mut perf_session, fp: *mut FILE) -> size_t;

    pub fn perf_session__dump_kmaps(session: *mut perf_session);

    pub fn perf_session__find_first_evtype(
        session: *mut perf_session,
        type_: c_uint,
    ) -> *mut evsel;

    pub fn perf_session__cpu_bitmap(
        session: *mut perf_session,
        cpu_list: *const c_char,
        cpu_bitmap: *mut c_ulong,
    ) -> c_int;

    pub fn perf_session__fprintf_info(s: *mut perf_session, fp: *mut FILE, full: bool);

    pub fn __evlist__set_tracepoints_handlers(
        evlist: *mut evlist,
        array: *mut evsel_str_handler,
        nr_handlers: size_t,
    ) -> c_int;

    pub static mut session_done: c_int;

    pub fn perf_session__deliver_synth_event(
        session: *mut perf_session,
        event: *mut perf_event,
        sample: *mut perf_sample,
    ) -> c_int;

    pub fn perf_session__deliver_synth_attr_event(
        session: *mut perf_session,
        attr: *const perf_event_attr,
        id: u64,
    ) -> c_int;

    pub fn perf_session__dsos_hit_all(session: *mut perf_session) -> c_int;

    pub fn perf_event__process_id_index(
        tool: *const perf_tool,
        session: *mut perf_session,
        event: *mut perf_event,
    ) -> c_int;

    pub fn perf_event__process_finished_round(
        tool: *const perf_tool,
        event: *mut perf_event,
        oe: *mut ordered_events,
    ) -> c_int;

    pub fn perf_session__env(session: *mut perf_session) -> *mut perf_env;
    pub fn perf_session__e_machine(session: *mut perf_session, e_flags: *mut uint32_t) -> uint16_t;
}

#[inline]
pub unsafe fn perf_session__new(
    data: *mut perf_data,
    tool: *mut perf_tool,
) -> *mut perf_session {
    unsafe { __perf_session__new(data, tool, false, core::ptr::null_mut()) }
}

#[inline]
pub unsafe fn perf_session__find_machine(
    session: *mut perf_session,
    pid: pid_t,
) -> *mut machine {
    unsafe { machines__find(core::ptr::addr_of_mut!((*session).machines), pid) }
}

#[inline]
pub unsafe fn perf_session__findnew_machine(
    session: *mut perf_session,
    pid: pid_t,
) -> *mut machine {
    unsafe { machines__findnew(core::ptr::addr_of_mut!((*session).machines), pid) }
}

/* Macro translation:
 * #define perf_session__set_tracepoints_handlers(session, array) \
 *         __evlist__set_tracepoints_handlers(session->evlist, array, ARRAY_SIZE(array))
 */
#[inline]
pub unsafe fn perf_session__set_tracepoints_handlers<const N: usize>(
    session: *mut perf_session,
    array: *mut [evsel_str_handler; N],
) -> c_int {
    unsafe {
        __evlist__set_tracepoints_handlers(
            (*session).evlist,
            (*array).as_mut_ptr(),
            N as size_t,
        )
    }
}

/* Macro translation of READ_ONCE(session_done). */
#[inline]
pub unsafe fn session_done_read_once() -> c_int {
    unsafe { core::ptr::read_volatile(core::ptr::addr_of!(session_done)) }
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
