// SPDX-License-Identifier: GPL-2.0
//
// Translated from perf/util/tool.c. C include dependencies are expected to be
// provided by surrounding translated units.

use core::ffi::{c_char, c_int, c_long, c_void};
use core::mem::size_of;
use core::ptr;

pub type u64 = u64;
pub type s64 = i64;
pub type size_t = usize;
pub type ssize_t = isize;
pub type off_t = c_long;

pub const SHOW_FEAT_NO_HEADER: c_int = 0;
pub const PERF_RECORD_COMPRESSED: u32 = 81;
pub const PERF_RECORD_COMPRESSED2: u32 = 82;
pub const PROT_READ: c_int = 0x1;
pub const PROT_WRITE: c_int = 0x2;
pub const MAP_PRIVATE: c_int = 0x02;
pub const MAP_ANONYMOUS: c_int = 0x20;
pub const MAP_FAILED: *mut c_void = !0usize as *mut c_void;

#[repr(C)]
pub struct perf_tool {
    pub ordered_events: bool,
    pub ordering_requires_timestamps: bool,
    pub namespace_events: bool,
    pub cgroup_events: bool,
    pub no_warn: bool,
    pub show_feat_hdr: c_int,
    pub merge_deferred_callchains: bool,
    pub dont_split_sample_group: bool,
    pub sample: SampleFn,
    pub mmap: EventFn,
    pub mmap2: EventFn,
    pub comm: EventFn,
    pub namespaces: EventFn,
    pub cgroup: EventFn,
    pub fork: EventFn,
    pub exit: EventFn,
    pub lost: EventFn,
    pub lost_samples: EventFn,
    pub aux: EventFn,
    pub itrace_start: EventFn,
    pub context_switch: EventFn,
    pub ksymbol: EventFn,
    pub bpf: EventFn,
    pub text_poke: EventFn,
    pub aux_output_hw_id: EventFn,
    pub read: SampleFn,
    pub throttle: EventFn,
    pub unthrottle: EventFn,
    pub callchain_deferred: SampleFn,
    pub attr: AttrFn,
    pub event_update: AttrFn,
    pub tracing_data: TracingDataFn,
    pub build_id: Op2Fn,
    pub finished_round: FinishedRoundFn,
    pub id_index: Op2Fn,
    pub auxtrace_info: Op2Fn,
    pub auxtrace: AuxtraceFn,
    pub auxtrace_error: Op2Fn,
    pub thread_map: Op2Fn,
    pub cpu_map: Op2Fn,
    pub stat_config: Op2Fn,
    pub stat: Op2Fn,
    pub stat_round: Op2Fn,
    pub time_conv: Op2Fn,
    pub feature: Op2Fn,
    pub compressed: CompressedFn,
    pub finished_init: Op2Fn,
    pub bpf_metadata: Op2Fn,
    pub schedstat_cpu: Op2Fn,
    pub schedstat_domain: Op2Fn,
}

#[repr(C)]
pub struct delegate_tool {
    pub tool: perf_tool,
    pub delegate: *mut perf_tool,
}

#[repr(C)]
pub struct perf_session {
    pub data: *mut perf_data,
    pub active_decomp: *mut active_decomp,
}

#[repr(C)]
pub struct active_decomp {
    pub zstd_decomp: *mut c_void,
    pub decomp: *mut decomp,
    pub decomp_last: *mut decomp,
}

#[repr(C)]
pub struct perf_env {
    pub comp_mmap_len: size_t,
}

#[repr(C)]
pub struct decomp {
    pub file_pos: u64,
    pub file_path: *const c_char,
    pub mmap_len: size_t,
    pub head: u64,
    pub size: u64,
    pub next: *mut decomp,
    pub data: [u8; 0],
}

#[repr(C)]
pub struct perf_event_header {
    pub type_: u32,
    pub misc: u16,
    pub size: u16,
}

#[repr(C)]
pub struct perf_record_compressed {
    pub header: perf_event_header,
}

#[repr(C)]
pub struct perf_record_compressed2 {
    pub header: perf_event_header,
    pub data_size: size_t,
}

#[repr(C)]
pub struct perf_record_auxtrace {
    pub header: perf_event_header,
    pub size: u64,
}

#[repr(C)]
pub union perf_event {
    pub header: perf_event_header,
    pub pack: core::mem::ManuallyDrop<perf_record_compressed>,
    pub pack2: core::mem::ManuallyDrop<perf_record_compressed2>,
    pub auxtrace: core::mem::ManuallyDrop<perf_record_auxtrace>,
}

#[repr(C)]
pub struct perf_sample {
    _unused: [u8; 0],
}
#[repr(C)]
pub struct machine {
    _unused: [u8; 0],
}
#[repr(C)]
pub struct evlist {
    _unused: [u8; 0],
}
#[repr(C)]
pub struct ordered_events {
    _unused: [u8; 0],
}
#[repr(C)]
pub struct perf_data {
    _unused: [u8; 0],
}

pub type SampleFn = unsafe extern "C" fn(
    *const perf_tool,
    *mut perf_event,
    *mut perf_sample,
    *mut machine,
) -> c_int;
pub type EventFn = unsafe extern "C" fn(
    *const perf_tool,
    *mut perf_event,
    *mut perf_sample,
    *mut machine,
) -> c_int;
pub type AttrFn =
    unsafe extern "C" fn(*const perf_tool, *mut perf_event, *mut *mut evlist) -> c_int;
pub type TracingDataFn =
    unsafe extern "C" fn(*const perf_tool, *mut perf_session, *mut perf_event) -> c_int;
pub type FinishedRoundFn =
    unsafe extern "C" fn(*const perf_tool, *mut perf_event, *mut ordered_events) -> c_int;
pub type Op2Fn =
    unsafe extern "C" fn(*const perf_tool, *mut perf_session, *mut perf_event) -> c_int;
pub type AuxtraceFn =
    unsafe extern "C" fn(*const perf_tool, *mut perf_session, *mut perf_event) -> s64;
pub type CompressedFn = unsafe extern "C" fn(
    *const perf_tool,
    *mut perf_session,
    *mut perf_event,
    u64,
    *const c_char,
) -> c_int;

unsafe extern "C" {
    static mut dump_trace: bool;
    static mut stdout: *mut c_void;

    fn dump_printf(fmt: *const c_char, ...);
    fn pr_err(fmt: *const c_char, ...);
    fn pr_debug(fmt: *const c_char, ...);
    fn read(fd: c_int, buf: *mut c_void, count: size_t) -> ssize_t;
    fn mmap(
        addr: *mut c_void,
        length: size_t,
        prot: c_int,
        flags: c_int,
        fd: c_int,
        offset: off_t,
    ) -> *mut c_void;
    fn munmap(addr: *mut c_void, length: size_t) -> c_int;
    fn memcpy(dest: *mut c_void, src: *const c_void, n: size_t) -> *mut c_void;

    fn perf_session__env(session: *mut perf_session) -> *mut perf_env;
    fn zstd_decompress_stream(
        decomp: *mut c_void,
        src: *mut c_void,
        src_size: size_t,
        dst: *mut u8,
        dst_size: size_t,
    ) -> size_t;
    fn perf_data__is_pipe(data: *mut perf_data) -> bool;
    fn perf_data__fd(data: *mut perf_data) -> c_int;

    fn perf_event__fprintf_event_update(event: *mut perf_event, fp: *mut c_void);
    fn perf_event__fprintf_thread_map(event: *mut perf_event, fp: *mut c_void);
    fn perf_event__fprintf_cpu_map(event: *mut perf_event, fp: *mut c_void);
    fn perf_event__fprintf_stat_config(event: *mut perf_event, fp: *mut c_void);
    fn perf_event__fprintf_stat(event: *mut perf_event, fp: *mut c_void);
    fn perf_event__fprintf_stat_round(event: *mut perf_event, fp: *mut c_void);
    fn perf_event__fprintf_time_conv(event: *mut perf_event, fp: *mut c_void);
    fn perf_event__fprintf_bpf_metadata(event: *mut perf_event, fp: *mut c_void);
    fn perf_event__fprintf_schedstat_cpu(event: *mut perf_event, fp: *mut c_void);
    fn perf_event__fprintf_schedstat_domain(event: *mut perf_event, fp: *mut c_void);

    fn perf_event__process_lost(
        tool: *const perf_tool,
        event: *mut perf_event,
        sample: *mut perf_sample,
        machine: *mut machine,
    ) -> c_int;
    fn perf_event__process_lost_samples(
        tool: *const perf_tool,
        event: *mut perf_event,
        sample: *mut perf_sample,
        machine: *mut machine,
    ) -> c_int;
    fn perf_event__process_aux(
        tool: *const perf_tool,
        event: *mut perf_event,
        sample: *mut perf_sample,
        machine: *mut machine,
    ) -> c_int;
    fn perf_event__process_itrace_start(
        tool: *const perf_tool,
        event: *mut perf_event,
        sample: *mut perf_sample,
        machine: *mut machine,
    ) -> c_int;
    fn perf_event__process_switch(
        tool: *const perf_tool,
        event: *mut perf_event,
        sample: *mut perf_sample,
        machine: *mut machine,
    ) -> c_int;
    fn perf_event__process_ksymbol(
        tool: *const perf_tool,
        event: *mut perf_event,
        sample: *mut perf_sample,
        machine: *mut machine,
    ) -> c_int;
    fn perf_event__process_bpf(
        tool: *const perf_tool,
        event: *mut perf_event,
        sample: *mut perf_sample,
        machine: *mut machine,
    ) -> c_int;
    fn perf_event__process_text_poke(
        tool: *const perf_tool,
        event: *mut perf_event,
        sample: *mut perf_sample,
        machine: *mut machine,
    ) -> c_int;
    fn perf_event__process_aux_output_hw_id(
        tool: *const perf_tool,
        event: *mut perf_event,
        sample: *mut perf_sample,
        machine: *mut machine,
    ) -> c_int;
    fn perf_event__process_finished_round(
        tool: *const perf_tool,
        event: *mut perf_event,
        oe: *mut ordered_events,
    ) -> c_int;
}

#[cfg(feature = "zstd")]
unsafe extern "C" fn perf_session__process_compressed_event(
    _tool: *const perf_tool,
    session: *mut perf_session,
    event: *mut perf_event,
    file_offset: u64,
    file_path: *const c_char,
) -> c_int {
    let src: *mut c_void;
    let src_size: size_t;
    let mut decomp_last_rem: u64 = 0;
    let mut decomp_len: size_t = (*perf_session__env(session)).comp_mmap_len;
    let decomp_last: *mut decomp = (*(*session).active_decomp).decomp_last;

    if decomp_len == 0 {
        pr_err(c"Compressed events found but HEADER_COMPRESSED not set\n".as_ptr());
        return -1;
    }

    if !decomp_last.is_null() {
        /* Prevent u64 underflow in decomp_last_rem */
        if (*decomp_last).head > (*decomp_last).size {
            return -1;
        }
        decomp_last_rem = (*decomp_last).size - (*decomp_last).head;
        /*
         * Check before adding: on 32-bit, size_t += u64
         * silently truncates, bypassing the overflow check
         * below and producing an undersized buffer.
         */
        if decomp_last_rem as usize > usize::MAX - decomp_len - size_of::<decomp>() {
            pr_err(c"Decompression buffer size overflow\n".as_ptr());
            return -1;
        }
        decomp_len += decomp_last_rem as size_t;
    }

    if decomp_len > usize::MAX - size_of::<decomp>() {
        pr_err(c"Decompression buffer size overflow\n".as_ptr());
        return -1;
    }
    let mmap_len: size_t = size_of::<decomp>() + decomp_len;
    let decomp_ptr = mmap(
        ptr::null_mut(),
        mmap_len,
        PROT_READ | PROT_WRITE,
        MAP_ANONYMOUS | MAP_PRIVATE,
        -1,
        0,
    ) as *mut decomp;
    if decomp_ptr as *mut c_void == MAP_FAILED {
        pr_err(c"Couldn't allocate memory for decompression\n".as_ptr());
        return -1;
    }

    (*decomp_ptr).file_pos = file_offset;
    (*decomp_ptr).file_path = file_path;
    (*decomp_ptr).mmap_len = mmap_len;
    (*decomp_ptr).head = 0;

    if decomp_last_rem != 0 {
        memcpy(
            (*decomp_ptr).data.as_mut_ptr() as *mut c_void,
            (*decomp_last).data.as_ptr().add((*decomp_last).head as usize) as *const c_void,
            decomp_last_rem as size_t,
        );
        (*decomp_ptr).size = decomp_last_rem;
    }

    /*
     * Events are read directly from the mmap'd file; fields could
     * theoretically change via a FUSE-backed file, but that applies
     * to the entire event processing pipeline, not just here.
     */
    if (*event).header.type_ == PERF_RECORD_COMPRESSED {
        if ((*event).header.size as usize) < size_of::<perf_record_compressed>() {
            munmap(decomp_ptr as *mut c_void, mmap_len);
            pr_err(c"Couldn't decompress data\n".as_ptr());
            return -1;
        }
        src = (event as *mut u8).add(size_of::<perf_record_compressed>()) as *mut c_void;
        src_size = (*event).pack.header.size as size_t - size_of::<perf_record_compressed>();
    } else if (*event).header.type_ == PERF_RECORD_COMPRESSED2 {
        /*
         * prefetch_event() only guarantees that the 8-byte
         * event header fits; validate that header.size covers
         * the data_size field before accessing it, otherwise a
         * crafted event reads data_size from adjacent memory.
         */
        if ((*event).header.size as usize) < size_of::<perf_record_compressed2>() {
            munmap(decomp_ptr as *mut c_void, mmap_len);
            pr_err(c"Couldn't decompress data\n".as_ptr());
            return -1;
        }
        src = (event as *mut u8).add(size_of::<perf_record_compressed2>()) as *mut c_void;
        src_size = (*event).pack2.data_size;
        /*
         * data_size is independent of header.size (which
         * includes padding); verify it doesn't exceed the
         * actual payload to prevent out-of-bounds reads in
         * zstd_decompress_stream().
         */
        if src_size > (*event).header.size as usize - size_of::<perf_record_compressed2>() {
            munmap(decomp_ptr as *mut c_void, mmap_len);
            pr_err(c"Couldn't decompress data\n".as_ptr());
            return -1;
        }
    } else {
        munmap(decomp_ptr as *mut c_void, mmap_len);
        pr_err(c"Couldn't decompress data\n".as_ptr());
        return -1;
    }

    let decomp_size = zstd_decompress_stream(
        (*(*session).active_decomp).zstd_decomp,
        src,
        src_size,
        (*decomp_ptr).data.as_mut_ptr().add(decomp_last_rem as usize),
        decomp_len - decomp_last_rem as size_t,
    );
    if decomp_size == 0 {
        munmap(decomp_ptr as *mut c_void, mmap_len);
        pr_err(c"Couldn't decompress data\n".as_ptr());
        return -1;
    }

    (*decomp_ptr).size += decomp_size as u64;

    if (*(*session).active_decomp).decomp.is_null() {
        (*(*session).active_decomp).decomp = decomp_ptr;
    } else {
        (*(*(*session).active_decomp).decomp_last).next = decomp_ptr;
    }

    (*(*session).active_decomp).decomp_last = decomp_ptr;

    pr_debug(c"decomp (B): %zd to %zd\n".as_ptr(), src_size, decomp_size);

    0
}

unsafe extern "C" fn process_event_synth_tracing_data_stub(
    _tool: *const perf_tool,
    _session: *mut perf_session,
    _event: *mut perf_event,
) -> c_int {
    dump_printf(c": unhandled!\n".as_ptr());
    0
}

unsafe extern "C" fn process_event_synth_attr_stub(
    _tool: *const perf_tool,
    _event: *mut perf_event,
    _pevlist: *mut *mut evlist,
) -> c_int {
    dump_printf(c": unhandled!\n".as_ptr());
    0
}

unsafe extern "C" fn process_event_synth_event_update_stub(
    _tool: *const perf_tool,
    event: *mut perf_event,
    _pevlist: *mut *mut evlist,
) -> c_int {
    if dump_trace {
        perf_event__fprintf_event_update(event, stdout);
    }

    dump_printf(c": unhandled!\n".as_ptr());
    0
}

#[no_mangle]
pub unsafe extern "C" fn process_event_sample_stub(
    _tool: *const perf_tool,
    _event: *mut perf_event,
    _sample: *mut perf_sample,
    _machine: *mut machine,
) -> c_int {
    dump_printf(c": unhandled!\n".as_ptr());
    0
}

unsafe extern "C" fn process_event_stub(
    _tool: *const perf_tool,
    _event: *mut perf_event,
    _sample: *mut perf_sample,
    _machine: *mut machine,
) -> c_int {
    dump_printf(c": unhandled!\n".as_ptr());
    0
}

unsafe extern "C" fn process_finished_round_stub(
    _tool: *const perf_tool,
    _event: *mut perf_event,
    _oe: *mut ordered_events,
) -> c_int {
    dump_printf(c": unhandled!\n".as_ptr());
    0
}

unsafe extern "C" fn skipn(fd: c_int, mut n: off_t) -> c_int {
    let mut buf = [0 as c_char; 4096];
    let mut ret: ssize_t;

    while n > 0 {
        let count = if n < buf.len() as off_t {
            n as size_t
        } else {
            buf.len()
        };
        ret = read(fd, buf.as_mut_ptr() as *mut c_void, count);
        if ret <= 0 {
            return ret as c_int;
        }
        n -= ret as off_t;
    }

    0
}

unsafe extern "C" fn process_event_auxtrace_stub(
    _tool: *const perf_tool,
    session: *mut perf_session,
    event: *mut perf_event,
) -> s64 {
    dump_printf(c": unhandled!\n".as_ptr());
    if perf_data__is_pipe((*session).data) {
        skipn(perf_data__fd((*session).data), (*event).auxtrace.size as off_t);
    }
    (*event).auxtrace.size as s64
}

unsafe extern "C" fn process_event_op2_stub(
    _tool: *const perf_tool,
    _session: *mut perf_session,
    _event: *mut perf_event,
) -> c_int {
    dump_printf(c": unhandled!\n".as_ptr());
    0
}

unsafe extern "C" fn process_event_thread_map_stub(
    _tool: *const perf_tool,
    _session: *mut perf_session,
    event: *mut perf_event,
) -> c_int {
    if dump_trace {
        perf_event__fprintf_thread_map(event, stdout);
    }

    dump_printf(c": unhandled!\n".as_ptr());
    0
}

unsafe extern "C" fn process_event_cpu_map_stub(
    _tool: *const perf_tool,
    _session: *mut perf_session,
    event: *mut perf_event,
) -> c_int {
    if dump_trace {
        perf_event__fprintf_cpu_map(event, stdout);
    }

    dump_printf(c": unhandled!\n".as_ptr());
    0
}

unsafe extern "C" fn process_event_stat_config_stub(
    _tool: *const perf_tool,
    _session: *mut perf_session,
    event: *mut perf_event,
) -> c_int {
    if dump_trace {
        perf_event__fprintf_stat_config(event, stdout);
    }

    dump_printf(c": unhandled!\n".as_ptr());
    0
}

unsafe extern "C" fn process_stat_stub(
    _tool: *const perf_tool,
    _perf_session: *mut perf_session,
    event: *mut perf_event,
) -> c_int {
    if dump_trace {
        perf_event__fprintf_stat(event, stdout);
    }

    dump_printf(c": unhandled!\n".as_ptr());
    0
}

unsafe extern "C" fn process_stat_round_stub(
    _tool: *const perf_tool,
    _perf_session: *mut perf_session,
    event: *mut perf_event,
) -> c_int {
    if dump_trace {
        perf_event__fprintf_stat_round(event, stdout);
    }

    dump_printf(c": unhandled!\n".as_ptr());
    0
}

unsafe extern "C" fn process_event_time_conv_stub(
    _tool: *const perf_tool,
    _perf_session: *mut perf_session,
    event: *mut perf_event,
) -> c_int {
    if dump_trace {
        perf_event__fprintf_time_conv(event, stdout);
    }

    dump_printf(c": unhandled!\n".as_ptr());
    0
}

unsafe extern "C" fn perf_session__process_compressed_event_stub(
    _tool: *const perf_tool,
    _session: *mut perf_session,
    _event: *mut perf_event,
    _file_offset: u64,
    _file_path: *const c_char,
) -> c_int {
    dump_printf(c": unhandled!\n".as_ptr());
    0
}

unsafe extern "C" fn perf_event__process_bpf_metadata_stub(
    _tool: *const perf_tool,
    _perf_session: *mut perf_session,
    event: *mut perf_event,
) -> c_int {
    if dump_trace {
        perf_event__fprintf_bpf_metadata(event, stdout);
    }
    dump_printf(c": unhandled!\n".as_ptr());
    0
}

unsafe extern "C" fn process_schedstat_cpu_stub(
    _tool: *const perf_tool,
    _perf_session: *mut perf_session,
    event: *mut perf_event,
) -> c_int {
    if dump_trace {
        perf_event__fprintf_schedstat_cpu(event, stdout);
    }
    dump_printf(c": unhandled!\n".as_ptr());
    0
}

unsafe extern "C" fn process_schedstat_domain_stub(
    _tool: *const perf_tool,
    _perf_session: *mut perf_session,
    event: *mut perf_event,
) -> c_int {
    if dump_trace {
        perf_event__fprintf_schedstat_domain(event, stdout);
    }
    dump_printf(c": unhandled!\n".as_ptr());
    0
}

#[no_mangle]
pub unsafe extern "C" fn perf_tool__init(tool: *mut perf_tool, ordered_events: bool) {
    (*tool).ordered_events = ordered_events;
    (*tool).ordering_requires_timestamps = false;
    (*tool).namespace_events = false;
    (*tool).cgroup_events = false;
    (*tool).no_warn = false;
    (*tool).show_feat_hdr = SHOW_FEAT_NO_HEADER;
    (*tool).merge_deferred_callchains = true;
    (*tool).dont_split_sample_group = false;

    (*tool).sample = process_event_sample_stub;
    (*tool).mmap = process_event_stub;
    (*tool).mmap2 = process_event_stub;
    (*tool).comm = process_event_stub;
    (*tool).namespaces = process_event_stub;
    (*tool).cgroup = process_event_stub;
    (*tool).fork = process_event_stub;
    (*tool).exit = process_event_stub;
    (*tool).lost = perf_event__process_lost;
    (*tool).lost_samples = perf_event__process_lost_samples;
    (*tool).aux = perf_event__process_aux;
    (*tool).itrace_start = perf_event__process_itrace_start;
    (*tool).context_switch = perf_event__process_switch;
    (*tool).ksymbol = perf_event__process_ksymbol;
    (*tool).bpf = perf_event__process_bpf;
    (*tool).text_poke = perf_event__process_text_poke;
    (*tool).aux_output_hw_id = perf_event__process_aux_output_hw_id;
    (*tool).read = process_event_sample_stub;
    (*tool).throttle = process_event_stub;
    (*tool).unthrottle = process_event_stub;
    (*tool).callchain_deferred = process_event_sample_stub;
    (*tool).attr = process_event_synth_attr_stub;
    (*tool).event_update = process_event_synth_event_update_stub;
    (*tool).tracing_data = process_event_synth_tracing_data_stub;
    (*tool).build_id = process_event_op2_stub;

    if ordered_events {
        (*tool).finished_round = perf_event__process_finished_round;
    } else {
        (*tool).finished_round = process_finished_round_stub;
    }

    (*tool).id_index = process_event_op2_stub;
    (*tool).auxtrace_info = process_event_op2_stub;
    (*tool).auxtrace = process_event_auxtrace_stub;
    (*tool).auxtrace_error = process_event_op2_stub;
    (*tool).thread_map = process_event_thread_map_stub;
    (*tool).cpu_map = process_event_cpu_map_stub;
    (*tool).stat_config = process_event_stat_config_stub;
    (*tool).stat = process_stat_stub;
    (*tool).stat_round = process_stat_round_stub;
    (*tool).time_conv = process_event_time_conv_stub;
    (*tool).feature = process_event_op2_stub;
    #[cfg(feature = "zstd")]
    {
        (*tool).compressed = perf_session__process_compressed_event;
    }
    #[cfg(not(feature = "zstd"))]
    {
        (*tool).compressed = perf_session__process_compressed_event_stub;
    }
    (*tool).finished_init = process_event_op2_stub;
    (*tool).bpf_metadata = perf_event__process_bpf_metadata_stub;
    (*tool).schedstat_cpu = process_schedstat_cpu_stub;
    (*tool).schedstat_domain = process_schedstat_domain_stub;
}

#[no_mangle]
pub unsafe extern "C" fn perf_tool__compressed_is_stub(tool: *const perf_tool) -> bool {
    (*tool).compressed as usize == perf_session__process_compressed_event_stub as usize
}

unsafe fn delegate_tool_from_tool(tool: *const perf_tool) -> *mut delegate_tool {
    tool as *mut delegate_tool
}

macro_rules! create_delegate_sample {
    ($name:ident, $func:ident) => {
        unsafe extern "C" fn $func(
            tool: *const perf_tool,
            event: *mut perf_event,
            sample: *mut perf_sample,
            machine: *mut machine,
        ) -> c_int {
            let del_tool = delegate_tool_from_tool(tool);
            let delegate = (*del_tool).delegate;
            ((*delegate).$name)(delegate, event, sample, machine)
        }
    };
}
create_delegate_sample!(read, delegate_read);
create_delegate_sample!(sample, delegate_sample);
create_delegate_sample!(callchain_deferred, delegate_callchain_deferred);

macro_rules! create_delegate_attr {
    ($name:ident, $func:ident) => {
        unsafe extern "C" fn $func(
            tool: *const perf_tool,
            event: *mut perf_event,
            pevlist: *mut *mut evlist,
        ) -> c_int {
            let del_tool = delegate_tool_from_tool(tool);
            let delegate = (*del_tool).delegate;
            ((*delegate).$name)(delegate, event, pevlist)
        }
    };
}
create_delegate_attr!(attr, delegate_attr);
create_delegate_attr!(event_update, delegate_event_update);

macro_rules! create_delegate_oe {
    ($name:ident, $func:ident) => {
        unsafe extern "C" fn $func(
            tool: *const perf_tool,
            event: *mut perf_event,
            oe: *mut ordered_events,
        ) -> c_int {
            let del_tool = delegate_tool_from_tool(tool);
            let delegate = (*del_tool).delegate;
            ((*delegate).$name)(delegate, event, oe)
        }
    };
}
create_delegate_oe!(finished_round, delegate_finished_round);

macro_rules! create_delegate_op {
    ($name:ident, $func:ident) => {
        unsafe extern "C" fn $func(
            tool: *const perf_tool,
            event: *mut perf_event,
            sample: *mut perf_sample,
            machine: *mut machine,
        ) -> c_int {
            let del_tool = delegate_tool_from_tool(tool);
            let delegate = (*del_tool).delegate;
            ((*delegate).$name)(delegate, event, sample, machine)
        }
    };
}
create_delegate_op!(aux, delegate_aux);
create_delegate_op!(aux_output_hw_id, delegate_aux_output_hw_id);
create_delegate_op!(bpf, delegate_bpf);
create_delegate_op!(cgroup, delegate_cgroup);
create_delegate_op!(comm, delegate_comm);
create_delegate_op!(context_switch, delegate_context_switch);
create_delegate_op!(exit, delegate_exit);
create_delegate_op!(fork, delegate_fork);
create_delegate_op!(itrace_start, delegate_itrace_start);
create_delegate_op!(ksymbol, delegate_ksymbol);
create_delegate_op!(lost, delegate_lost);
create_delegate_op!(lost_samples, delegate_lost_samples);
create_delegate_op!(mmap, delegate_mmap);
create_delegate_op!(mmap2, delegate_mmap2);
create_delegate_op!(namespaces, delegate_namespaces);
create_delegate_op!(text_poke, delegate_text_poke);
create_delegate_op!(throttle, delegate_throttle);
create_delegate_op!(unthrottle, delegate_unthrottle);

macro_rules! create_delegate_op2 {
    ($name:ident, $func:ident) => {
        unsafe extern "C" fn $func(
            tool: *const perf_tool,
            session: *mut perf_session,
            event: *mut perf_event,
        ) -> c_int {
            let del_tool = delegate_tool_from_tool(tool);
            let delegate = (*del_tool).delegate;
            ((*delegate).$name)(delegate, session, event)
        }
    };
}
create_delegate_op2!(auxtrace_error, delegate_auxtrace_error);
create_delegate_op2!(auxtrace_info, delegate_auxtrace_info);
create_delegate_op2!(bpf_metadata, delegate_bpf_metadata);
create_delegate_op2!(build_id, delegate_build_id);
create_delegate_op2!(cpu_map, delegate_cpu_map);
create_delegate_op2!(feature, delegate_feature);
create_delegate_op2!(finished_init, delegate_finished_init);
create_delegate_op2!(id_index, delegate_id_index);
create_delegate_op2!(stat, delegate_stat);
create_delegate_op2!(stat_config, delegate_stat_config);
create_delegate_op2!(stat_round, delegate_stat_round);
create_delegate_op2!(thread_map, delegate_thread_map);
create_delegate_op2!(time_conv, delegate_time_conv);
create_delegate_op2!(schedstat_cpu, delegate_schedstat_cpu);
create_delegate_op2!(schedstat_domain, delegate_schedstat_domain);
create_delegate_op2!(tracing_data, delegate_tracing_data);

macro_rules! create_delegate_op3 {
    ($name:ident, $func:ident) => {
        unsafe extern "C" fn $func(
            tool: *const perf_tool,
            session: *mut perf_session,
            event: *mut perf_event,
        ) -> s64 {
            let del_tool = delegate_tool_from_tool(tool);
            let delegate = (*del_tool).delegate;
            ((*delegate).$name)(delegate, session, event)
        }
    };
}
create_delegate_op3!(auxtrace, delegate_auxtrace);

macro_rules! create_delegate_op4 {
    ($name:ident, $func:ident) => {
        unsafe extern "C" fn $func(
            tool: *const perf_tool,
            session: *mut perf_session,
            event: *mut perf_event,
            data: u64,
            str_: *const c_char,
        ) -> c_int {
            let del_tool = delegate_tool_from_tool(tool);
            let delegate = (*del_tool).delegate;
            ((*delegate).$name)(delegate, session, event, data, str_)
        }
    };
}
create_delegate_op4!(compressed, delegate_compressed);

#[no_mangle]
pub unsafe extern "C" fn delegate_tool__init(tool: *mut delegate_tool, delegate: *mut perf_tool) {
    (*tool).delegate = delegate;

    (*tool).tool.ordered_events = (*delegate).ordered_events;
    (*tool).tool.ordering_requires_timestamps = (*delegate).ordering_requires_timestamps;
    (*tool).tool.namespace_events = (*delegate).namespace_events;
    (*tool).tool.cgroup_events = (*delegate).cgroup_events;
    (*tool).tool.no_warn = (*delegate).no_warn;
    (*tool).tool.show_feat_hdr = (*delegate).show_feat_hdr;
    (*tool).tool.merge_deferred_callchains = (*delegate).merge_deferred_callchains;
    (*tool).tool.dont_split_sample_group = (*delegate).dont_split_sample_group;

    (*tool).tool.sample = delegate_sample;
    (*tool).tool.read = delegate_read;

    (*tool).tool.mmap = delegate_mmap;
    (*tool).tool.mmap2 = delegate_mmap2;
    (*tool).tool.comm = delegate_comm;
    (*tool).tool.namespaces = delegate_namespaces;
    (*tool).tool.cgroup = delegate_cgroup;
    (*tool).tool.fork = delegate_fork;
    (*tool).tool.exit = delegate_exit;
    (*tool).tool.lost = delegate_lost;
    (*tool).tool.lost_samples = delegate_lost_samples;
    (*tool).tool.aux = delegate_aux;
    (*tool).tool.itrace_start = delegate_itrace_start;
    (*tool).tool.aux_output_hw_id = delegate_aux_output_hw_id;
    (*tool).tool.context_switch = delegate_context_switch;
    (*tool).tool.throttle = delegate_throttle;
    (*tool).tool.unthrottle = delegate_unthrottle;
    (*tool).tool.ksymbol = delegate_ksymbol;
    (*tool).tool.bpf = delegate_bpf;
    (*tool).tool.text_poke = delegate_text_poke;
    (*tool).tool.callchain_deferred = delegate_callchain_deferred;

    (*tool).tool.attr = delegate_attr;
    (*tool).tool.event_update = delegate_event_update;

    (*tool).tool.tracing_data = delegate_tracing_data;

    (*tool).tool.finished_round = delegate_finished_round;

    (*tool).tool.build_id = delegate_build_id;
    (*tool).tool.id_index = delegate_id_index;
    (*tool).tool.auxtrace_info = delegate_auxtrace_info;
    (*tool).tool.auxtrace_error = delegate_auxtrace_error;
    (*tool).tool.time_conv = delegate_time_conv;
    (*tool).tool.thread_map = delegate_thread_map;
    (*tool).tool.cpu_map = delegate_cpu_map;
    (*tool).tool.stat_config = delegate_stat_config;
    (*tool).tool.stat = delegate_stat;
    (*tool).tool.stat_round = delegate_stat_round;
    (*tool).tool.feature = delegate_feature;
    (*tool).tool.finished_init = delegate_finished_init;
    (*tool).tool.bpf_metadata = delegate_bpf_metadata;
    (*tool).tool.compressed = delegate_compressed;
    (*tool).tool.auxtrace = delegate_auxtrace;
    (*tool).tool.schedstat_cpu = delegate_schedstat_cpu;
    (*tool).tool.schedstat_domain = delegate_schedstat_domain;
}
