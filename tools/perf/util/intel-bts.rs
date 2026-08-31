// SPDX-License-Identifier: GPL-2.0-only
/*
 * intel-bts.c: Intel Processor Trace support
 * Copyright (c) 2013-2015, Intel Corporation.
 */

#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(dead_code)]
#![allow(improper_ctypes)]

use core::ffi::{c_char, c_int, c_long, c_uint, c_ulong, c_void};
use core::mem::{size_of, zeroed};
use core::ptr;

type u8 = u8;
type u32 = u32;
type u64 = u64;
type size_t = usize;
type ssize_t = isize;
type pid_t = c_int;
type off_t = c_long;
type bool_ = bool;
type FILE = c_void;

const MAX_TIMESTAMP: u64 = !0u64;

const INTEL_BTS_ERR_NOINSN: c_int = 5;
const INTEL_BTS_ERR_LOST: c_int = 9;

const ENOMEM: c_int = 12;
const EINVAL: c_int = 22;
const SEEK_CUR: c_int = 1;

const PERF_COLOR_BLUE: *const c_char = b"blue\0".as_ptr() as *const c_char;
const PERF_AUXTRACE_ERROR_ITRACE: u32 = 1;
const PERF_RECORD_SAMPLE: u32 = 9;
const PERF_RECORD_EXIT: u32 = 4;
const PERF_RECORD_AUX: u32 = 11;
const PERF_AUX_FLAG_TRUNCATED: u64 = 1;
const PERF_RECORD_MISC_KERNEL: u8 = 1;
const PERF_RECORD_MISC_USER: u8 = 2;
const PERF_TYPE_HARDWARE: u32 = 0;
const PERF_COUNT_HW_BRANCH_INSTRUCTIONS: u64 = 4;
const PERF_SAMPLE_IP: u64 = 1 << 0;
const PERF_SAMPLE_TID: u64 = 1 << 1;
const PERF_SAMPLE_TIME: u64 = 1 << 2;
const PERF_SAMPLE_ADDR: u64 = 1 << 3;
const PERF_SAMPLE_CPU: u64 = 1 << 7;
const PERF_SAMPLE_PERIOD: u64 = 1 << 8;
const PERF_SAMPLE_MASK: u64 = !0u64;
const PERF_IP_FLAG_BRANCH: u32 = 1 << 0;
const PERF_IP_FLAG_CALL: u32 = 1 << 1;
const PERF_IP_FLAG_RETURN: u32 = 1 << 2;
const PERF_IP_FLAG_CONDITIONAL: u32 = 1 << 3;
const PERF_IP_FLAG_SYSCALLRET: u32 = 1 << 4;
const PERF_IP_FLAG_ASYNC: u32 = 1 << 5;
const PERF_IP_FLAG_INTERRUPT: u32 = 1 << 6;
const PERF_IP_FLAG_TRACE_BEGIN: u32 = 1 << 7;
const PERF_IP_FLAG_TRACE_END: u32 = 1 << 8;
const INTEL_PT_INSN_BUF_SZ: usize = 16;

const INTEL_BTS_PMU_TYPE: usize = 0;
const INTEL_BTS_TIME_SHIFT: usize = 1;
const INTEL_BTS_TIME_MULT: usize = 2;
const INTEL_BTS_TIME_ZERO: usize = 3;
const INTEL_BTS_CAP_USER_TIME_ZERO: usize = 4;
const INTEL_BTS_SNAPSHOT_MODE: usize = 5;

#[repr(C)]
struct list_head {
    next: *mut list_head,
    prev: *mut list_head,
}

#[repr(C)]
struct auxtrace {
    process_event: Option<unsafe extern "C" fn(*mut perf_session, *mut perf_event, *mut perf_sample, *const perf_tool) -> c_int>,
    process_auxtrace_event: Option<unsafe extern "C" fn(*mut perf_session, *mut perf_event, *const perf_tool) -> c_int>,
    flush_events: Option<unsafe extern "C" fn(*mut perf_session, *const perf_tool) -> c_int>,
    free_events: Option<unsafe extern "C" fn(*mut perf_session)>,
    free: Option<unsafe extern "C" fn(*mut perf_session)>,
    evsel_is_auxtrace: Option<unsafe extern "C" fn(*mut perf_session, *mut evsel) -> bool>,
}

#[repr(C)]
struct auxtrace_queues {
    nr_queues: c_uint,
    queue_array: *mut auxtrace_queue,
    new_data: bool,
    populated: bool,
}

#[repr(C)]
struct auxtrace_queue {
    head: list_head,
    priv_: *mut c_void,
    cpu: c_int,
    tid: pid_t,
}

#[repr(C)]
struct auxtrace_heap {
    heap_cnt: c_uint,
    heap_array: *mut auxtrace_heap_item,
}

#[repr(C)]
struct auxtrace_heap_item {
    queue_nr: c_uint,
    ordinal: u64,
}

#[repr(C)]
struct auxtrace_buffer {
    list: list_head,
    data: *mut u8,
    size: size_t,
    use_data: *mut u8,
    use_size: size_t,
    reference: u64,
    consecutive: bool,
    buffer_nr: u64,
}

#[repr(C)]
struct perf_tsc_conversion {
    time_shift: u16,
    time_mult: u32,
    time_zero: u64,
}

type u16 = u16;

#[repr(C)]
#[derive(Copy, Clone)]
struct itrace_synth_opts {
    set: bool,
    default_no_sample: bool,
    thread_stack: bool,
    callchain: bool,
    inject: bool,
    errors: bool,
    branches: bool,
    calls: bool,
    returns: bool,
    initial_skip: u64,
}

#[repr(C)]
struct intel_pt_insn {
    buf: [u8; INTEL_PT_INSN_BUF_SZ],
    length: u8,
    op: c_int,
}

#[repr(C)]
struct intel_bts {
    auxtrace: auxtrace,
    queues: auxtrace_queues,
    heap: auxtrace_heap,
    auxtrace_type: u32,
    session: *mut perf_session,
    machine: *mut machine,
    sampling_mode: bool,
    snapshot_mode: bool,
    data_queued: bool,
    pmu_type: u32,
    tc: perf_tsc_conversion,
    cap_user_time_zero: bool,
    synth_opts: itrace_synth_opts,
    sample_branches: bool,
    branches_filter: u32,
    branches_sample_type: u64,
    branches_id: u64,
    branches_event_size: size_t,
    num_events: c_ulong,
}

#[repr(C)]
struct intel_bts_queue {
    bts: *mut intel_bts,
    queue_nr: c_uint,
    buffer: *mut auxtrace_buffer,
    on_heap: bool,
    done: bool,
    pid: pid_t,
    tid: pid_t,
    cpu: c_int,
    time: u64,
    intel_pt_insn: intel_pt_insn,
    sample_flags: u32,
}

#[repr(C)]
struct branch {
    from: u64,
    to: u64,
    misc: u64,
}

#[repr(C)]
struct perf_event_header {
    type_: u32,
    misc: u16,
    size: u16,
}

#[repr(C)]
struct perf_record_sample {
    header: perf_event_header,
}

#[repr(C)]
struct perf_record_aux {
    header: perf_event_header,
    flags: u64,
}

#[repr(C)]
struct perf_record_fork {
    header: perf_event_header,
    tid: pid_t,
}

#[repr(C)]
struct perf_record_auxtrace_error {
    header: perf_event_header,
}

#[repr(C)]
struct perf_record_auxtrace_info {
    header: perf_event_header,
    type_: u32,
    priv_: [u64; 0],
}

#[repr(C)]
union perf_event {
    header: perf_event_header,
    sample: core::mem::ManuallyDrop<perf_record_sample>,
    aux: core::mem::ManuallyDrop<perf_record_aux>,
    fork: core::mem::ManuallyDrop<perf_record_fork>,
    auxtrace_error: core::mem::ManuallyDrop<perf_record_auxtrace_error>,
    auxtrace_info: core::mem::ManuallyDrop<perf_record_auxtrace_info>,
}

#[repr(C)]
struct perf_sample {
    ip: u64,
    cpumode: u8,
    pid: pid_t,
    tid: pid_t,
    addr: u64,
    id: u64,
    stream_id: u64,
    period: u64,
    cpu: c_int,
    flags: u32,
    insn_len: u8,
    insn: [u8; INTEL_PT_INSN_BUF_SZ],
    time: u64,
}

#[repr(C)]
struct perf_event_attr {
    type_: u32,
    size: u32,
    config: u64,
    sample_period: u64,
    sample_type: u64,
    read_format: u64,
    exclude_user: bool,
    exclude_kernel: bool,
    exclude_hv: bool,
    exclude_host: bool,
    exclude_guest: bool,
    sample_id_all: bool,
}

#[repr(C)]
struct perf_tool {
    ordered_events: bool,
}

#[repr(C)]
struct perf_session {
    auxtrace: *mut auxtrace,
    machines: machines,
    evlist: *mut evlist,
    itrace_synth_opts: *mut itrace_synth_opts,
    data: *mut perf_data,
}

#[repr(C)]
struct machines {
    host: machine,
}

#[repr(C)]
struct evsel_core {
    attr: perf_event_attr,
    ids: *mut c_void,
}

#[repr(C)]
struct evsel {
    core: evsel_core,
}

#[repr(C)]
struct machine {
    _private: [u8; 0],
}

#[repr(C)]
struct thread {
    _private: [u8; 0],
}

#[repr(C)]
struct evlist {
    _private: [u8; 0],
}

#[repr(C)]
struct perf_data {
    _private: [u8; 0],
}

unsafe extern "C" {
    static mut stdout: *mut FILE;
    static mut dump_trace: bool;
    static mut errno: c_int;

    fn printf(fmt: *const c_char, ...) -> c_int;
    fn fprintf(stream: *mut FILE, fmt: *const c_char, ...) -> c_int;
    fn color_fprintf(stream: *mut FILE, color: *const c_char, fmt: *const c_char, ...) -> c_int;
    fn pr_err(fmt: *const c_char, ...) -> c_int;
    fn pr_debug(fmt: *const c_char, ...) -> c_int;
    fn zalloc(size: size_t) -> *mut c_void;
    fn free(ptr: *mut c_void);
    fn memcmp(a: *const c_void, b: *const c_void, n: size_t) -> c_int;
    fn memcpy(dst: *mut c_void, src: *const c_void, n: size_t) -> *mut c_void;
    fn memset(dst: *mut c_void, c: c_int, n: size_t) -> *mut c_void;
    fn lseek(fd: c_int, offset: off_t, whence: c_int) -> off_t;

    fn auxtrace_synth_error(err: *mut perf_record_auxtrace_error, typ: u32, code: c_int, cpu: c_int, pid: pid_t, tid: pid_t, ip: u64, msg: *const c_char, time: u64);
    fn perf_session__deliver_synth_event(session: *mut perf_session, event: *mut perf_event, sample: *mut perf_sample) -> c_int;
    fn auxtrace_buffer__next(queue: *mut auxtrace_queue, buffer: *mut auxtrace_buffer) -> *mut auxtrace_buffer;
    fn auxtrace_heap__add(heap: *mut auxtrace_heap, queue_nr: c_uint, ordinal: u64) -> c_int;
    fn auxtrace_heap__pop(heap: *mut auxtrace_heap);
    fn auxtrace_heap__free(heap: *mut auxtrace_heap);
    fn machine__kernel_ip(machine: *mut machine, ip: u64) -> bool;
    fn perf_sample__init(sample: *mut perf_sample, all: bool);
    fn perf_sample__exit(sample: *mut perf_sample);
    fn perf_event__synthesize_sample(event: *mut perf_event, sample_type: u64, read_format: u64, branch_sample_type: u64, sample: *mut perf_sample) -> c_int;
    fn machine__find_thread(machine: *mut machine, pid: pid_t, tid: pid_t) -> *mut thread;
    fn machine__findnew_thread(machine: *mut machine, pid: pid_t, tid: pid_t) -> *mut thread;
    fn thread__memcpy(thread: *mut thread, machine: *mut machine, buf: *mut u8, ip: u64, len: size_t, x86_64: *mut bool) -> ssize_t;
    fn thread__put(thread: *mut thread);
    fn thread__pid(thread: *mut thread) -> pid_t;
    fn intel_pt_get_insn(buf: *mut u8, len: ssize_t, x86_64: bool, insn: *mut intel_pt_insn) -> c_int;
    fn intel_pt_insn_type(op: c_int) -> u32;
    fn thread_stack__event(thread: *mut thread, cpu: c_int, flags: u32, from: u64, to: u64, insn_len: u8, trace_nr: u64, callchain: bool, a: u64, b: u64);
    fn thread_stack__set_trace_nr(thread: *mut thread, cpu: c_int, trace_nr: u64);
    fn perf_data__fd(data: *mut perf_data) -> c_int;
    fn auxtrace_buffer__get_data(buffer: *mut auxtrace_buffer, fd: c_int) -> *mut u8;
    fn auxtrace_buffer__drop_data(buffer: *mut auxtrace_buffer);
    fn auxtrace_buffer__put_data(buffer: *mut auxtrace_buffer);
    fn perf_time_to_tsc(time: u64, tc: *mut perf_tsc_conversion) -> u64;
    fn perf_data__is_pipe(data: *mut perf_data) -> bool;
    fn auxtrace_queues__add_event(queues: *mut auxtrace_queues, session: *mut perf_session, event: *mut perf_event, data_offset: off_t, buffer: *mut *mut auxtrace_buffer) -> c_int;
    fn auxtrace_queues__free(queues: *mut auxtrace_queues);
    fn auxtrace_queues__init(queues: *mut auxtrace_queues) -> c_int;
    fn auxtrace_queues__process_index(queues: *mut auxtrace_queues, session: *mut perf_session) -> c_int;
    fn auxtrace_synth_id_range_start(evsel: *mut evsel) -> u64;
    fn perf_session__deliver_synth_attr_event(session: *mut perf_session, attr: *mut perf_event_attr, id: u64) -> c_int;
    fn __evsel__sample_size(sample_type: u64) -> size_t;
    fn itrace_synth_opts__set_default(opts: *mut itrace_synth_opts, default_no_sample: bool);
}

#[inline]
unsafe fn le64_to_cpu(x: u64) -> u64 {
    u64::from_le(x)
}

#[inline]
unsafe fn list_empty(head: *const list_head) -> bool {
    (*head).next == head as *mut list_head
}

#[inline]
unsafe fn container_of_auxtrace(ptr: *mut auxtrace) -> *mut intel_bts {
    ptr as *mut intel_bts
}

#[inline]
unsafe fn list_entry_auxtrace_buffer_list(ptr: *mut list_head) -> *mut auxtrace_buffer {
    (ptr as *mut u8).sub(core::mem::offset_of!(auxtrace_buffer, list)) as *mut auxtrace_buffer
}

unsafe fn intel_bts_dump(_bts: *mut intel_bts, mut buf: *mut u8, mut len: size_t) {
    let mut branch: *mut branch;
    let mut i: size_t;
    let mut pos: size_t = 0;
    let br_sz: size_t = size_of::<branch>();
    let mut sz: size_t;
    let color = PERF_COLOR_BLUE;

    color_fprintf(stdout, color, b". ... Intel BTS data: size %zu bytes\n\0".as_ptr() as *const c_char, len);

    while len != 0 {
        if len >= br_sz {
            sz = br_sz;
        } else {
            sz = len;
        }
        printf(b".\0".as_ptr() as *const c_char);
        color_fprintf(stdout, color, b"  %08zx: \0".as_ptr() as *const c_char, pos);
        i = 0;
        while i < sz {
            color_fprintf(stdout, color, b" %02x\0".as_ptr() as *const c_char, *buf.add(i) as c_int);
            i += 1;
        }
        while i < br_sz {
            color_fprintf(stdout, color, b"   \0".as_ptr() as *const c_char);
            i += 1;
        }
        if len >= br_sz {
            branch = buf as *mut branch;
            color_fprintf(
                stdout,
                color,
                b" %lx -> %lx %s\n\0".as_ptr() as *const c_char,
                le64_to_cpu((*branch).from),
                le64_to_cpu((*branch).to),
                if (le64_to_cpu((*branch).misc) & 0x10) != 0 {
                    b"pred\0".as_ptr() as *const c_char
                } else {
                    b"miss\0".as_ptr() as *const c_char
                },
            );
        } else {
            color_fprintf(stdout, color, b" Bad record!\n\0".as_ptr() as *const c_char);
        }
        pos += sz;
        buf = buf.add(sz);
        len -= sz;
    }
}

unsafe extern "C" fn intel_bts_dump_event(bts: *mut intel_bts, buf: *mut u8, len: size_t) {
    printf(b".\n\0".as_ptr() as *const c_char);
    intel_bts_dump(bts, buf, len);
}

unsafe fn intel_bts_lost(bts: *mut intel_bts, sample: *mut perf_sample) -> c_int {
    let mut event: perf_event = zeroed();
    let mut err: c_int;

    auxtrace_synth_error(&mut event.auxtrace_error as *mut _ as *mut perf_record_auxtrace_error, PERF_AUXTRACE_ERROR_ITRACE,
                         INTEL_BTS_ERR_LOST, (*sample).cpu, (*sample).pid,
                         (*sample).tid, 0, b"Lost trace data\0".as_ptr() as *const c_char, (*sample).time);

    err = perf_session__deliver_synth_event((*bts).session, &mut event, ptr::null_mut());
    if err != 0 {
        pr_err(b"Intel BTS: failed to deliver error event, error %d\n\0".as_ptr() as *const c_char, err);
    }

    err
}

unsafe fn intel_bts_alloc_queue(bts: *mut intel_bts, queue_nr: c_uint) -> *mut intel_bts_queue {
    let btsq = zalloc(size_of::<intel_bts_queue>()) as *mut intel_bts_queue;
    if btsq.is_null() {
        return ptr::null_mut();
    }

    (*btsq).bts = bts;
    (*btsq).queue_nr = queue_nr;
    (*btsq).pid = -1;
    (*btsq).tid = -1;
    (*btsq).cpu = -1;

    btsq
}

unsafe fn intel_bts_setup_queue(bts: *mut intel_bts, queue: *mut auxtrace_queue, queue_nr: c_uint) -> c_int {
    let mut btsq = (*queue).priv_ as *mut intel_bts_queue;

    if list_empty(&(*queue).head) {
        return 0;
    }

    if btsq.is_null() {
        btsq = intel_bts_alloc_queue(bts, queue_nr);
        if btsq.is_null() {
            return -ENOMEM;
        }
        (*queue).priv_ = btsq as *mut c_void;

        if (*queue).cpu != -1 {
            (*btsq).cpu = (*queue).cpu;
        }
        (*btsq).tid = (*queue).tid;
    }

    if (*bts).sampling_mode {
        return 0;
    }

    if !(*btsq).on_heap && (*btsq).buffer.is_null() {
        let ret: c_int;

        (*btsq).buffer = auxtrace_buffer__next(queue, ptr::null_mut());
        if (*btsq).buffer.is_null() {
            return 0;
        }

        ret = auxtrace_heap__add(&mut (*bts).heap, queue_nr, (*(*btsq).buffer).reference);
        if ret != 0 {
            return ret;
        }
        (*btsq).on_heap = true;
    }

    0
}

unsafe fn intel_bts_setup_queues(bts: *mut intel_bts) -> c_int {
    let mut i: c_uint = 0;
    while i < (*bts).queues.nr_queues {
        let ret = intel_bts_setup_queue(bts, (*bts).queues.queue_array.add(i as usize), i);
        if ret != 0 {
            return ret;
        }
        i += 1;
    }
    0
}

#[inline]
unsafe fn intel_bts_update_queues(bts: *mut intel_bts) -> c_int {
    if (*bts).queues.new_data {
        (*bts).queues.new_data = false;
        return intel_bts_setup_queues(bts);
    }
    0
}

unsafe fn intel_bts_find_overlap(buf_a: *mut u8, len_a: size_t, buf_b: *mut u8, len_b: size_t) -> *mut u8 {
    let mut offs: size_t;
    let mut len: size_t;

    if len_a > len_b {
        offs = len_a - len_b;
    } else {
        offs = 0;
    }

    while offs < len_a {
        len = len_a - offs;
        if memcmp(buf_a.add(offs) as *const c_void, buf_b as *const c_void, len) == 0 {
            return buf_b.add(len);
        }
        offs += size_of::<branch>();
    }

    buf_b
}

unsafe fn intel_bts_do_fix_overlap(queue: *mut auxtrace_queue, b: *mut auxtrace_buffer) -> c_int {
    let a: *mut auxtrace_buffer;
    let start: *mut u8;

    if (*b).list.prev == &mut (*queue).head {
        return 0;
    }
    a = list_entry_auxtrace_buffer_list((*b).list.prev);
    start = intel_bts_find_overlap((*a).data, (*a).size, (*b).data, (*b).size);
    if start.is_null() {
        return -EINVAL;
    }
    (*b).use_size = (*b).data.add((*b).size).offset_from(start) as size_t;
    (*b).use_data = start;
    0
}

#[inline]
unsafe fn intel_bts_cpumode(bts: *mut intel_bts, ip: u64) -> u8 {
    if machine__kernel_ip((*bts).machine, ip) {
        PERF_RECORD_MISC_KERNEL
    } else {
        PERF_RECORD_MISC_USER
    }
}

unsafe fn intel_bts_synth_branch_sample(btsq: *mut intel_bts_queue, branch: *mut branch) -> c_int {
    let mut ret: c_int;
    let bts = (*btsq).bts;
    let mut event: perf_event = zeroed();
    let mut sample: perf_sample = zeroed();

    if (*bts).synth_opts.initial_skip != 0 && {
        let old = (*bts).num_events;
        (*bts).num_events = (*bts).num_events.wrapping_add(1);
        old <= (*bts).synth_opts.initial_skip as c_ulong
    } {
        return 0;
    }

    perf_sample__init(&mut sample, true);
    sample.ip = le64_to_cpu((*branch).from);
    sample.cpumode = intel_bts_cpumode(bts, sample.ip);
    sample.pid = (*btsq).pid;
    sample.tid = (*btsq).tid;
    sample.addr = le64_to_cpu((*branch).to);
    sample.id = (*(*btsq).bts).branches_id;
    sample.stream_id = (*(*btsq).bts).branches_id;
    sample.period = 1;
    sample.cpu = (*btsq).cpu;
    sample.flags = (*btsq).sample_flags;
    sample.insn_len = (*btsq).intel_pt_insn.length;
    memcpy(sample.insn.as_mut_ptr() as *mut c_void, (*btsq).intel_pt_insn.buf.as_ptr() as *const c_void, INTEL_PT_INSN_BUF_SZ);

    event.sample.header.type_ = PERF_RECORD_SAMPLE;
    event.sample.header.misc = sample.cpumode as u16;
    event.sample.header.size = size_of::<perf_event_header>() as u16;

    if (*bts).synth_opts.inject {
        event.sample.header.size = (*bts).branches_event_size as u16;
        ret = perf_event__synthesize_sample(&mut event, (*bts).branches_sample_type, 0, 0, &mut sample);
        if ret != 0 {
            return ret;
        }
    }

    ret = perf_session__deliver_synth_event((*bts).session, &mut event, &mut sample);
    if ret != 0 {
        pr_err(b"Intel BTS: failed to deliver branch event, error %d\n\0".as_ptr() as *const c_char, ret);
    }

    perf_sample__exit(&mut sample);
    ret
}

unsafe fn intel_bts_get_next_insn(btsq: *mut intel_bts_queue, ip: u64) -> c_int {
    let machine = (*(*btsq).bts).machine;
    let thread: *mut thread;
    let mut buf = [0u8; INTEL_PT_INSN_BUF_SZ];
    let len: ssize_t;
    let mut x86_64 = false;
    let mut err: c_int = -1;

    thread = machine__find_thread(machine, -1, (*btsq).tid);
    if thread.is_null() {
        return -1;
    }

    len = thread__memcpy(thread, machine, buf.as_mut_ptr(), ip, INTEL_PT_INSN_BUF_SZ, &mut x86_64);
    if len <= 0 {
        thread__put(thread);
        return err;
    }

    if intel_pt_get_insn(buf.as_mut_ptr(), len, x86_64, &mut (*btsq).intel_pt_insn) != 0 {
        thread__put(thread);
        return err;
    }

    err = 0;
    thread__put(thread);
    err
}

unsafe fn intel_bts_synth_error(bts: *mut intel_bts, cpu: c_int, pid: pid_t, tid: pid_t, ip: u64) -> c_int {
    let mut event: perf_event = zeroed();
    let mut err: c_int;

    auxtrace_synth_error(&mut event.auxtrace_error as *mut _ as *mut perf_record_auxtrace_error, PERF_AUXTRACE_ERROR_ITRACE,
                         INTEL_BTS_ERR_NOINSN, cpu, pid, tid, ip,
                         b"Failed to get instruction\0".as_ptr() as *const c_char, 0);

    err = perf_session__deliver_synth_event((*bts).session, &mut event, ptr::null_mut());
    if err != 0 {
        pr_err(b"Intel BTS: failed to deliver error event, error %d\n\0".as_ptr() as *const c_char, err);
    }

    err
}

unsafe fn intel_bts_get_branch_type(btsq: *mut intel_bts_queue, branch: *mut branch) -> c_int {
    let mut err: c_int;

    if (*branch).from == 0 {
        if (*branch).to != 0 {
            (*btsq).sample_flags = PERF_IP_FLAG_BRANCH | PERF_IP_FLAG_TRACE_BEGIN;
        } else {
            (*btsq).sample_flags = 0;
        }
        (*btsq).intel_pt_insn.length = 0;
    } else if (*branch).to == 0 {
        (*btsq).sample_flags = PERF_IP_FLAG_BRANCH | PERF_IP_FLAG_TRACE_END;
        (*btsq).intel_pt_insn.length = 0;
    } else {
        err = intel_bts_get_next_insn(btsq, (*branch).from);
        if err != 0 {
            (*btsq).sample_flags = 0;
            (*btsq).intel_pt_insn.length = 0;
            if !(*(*btsq).bts).synth_opts.errors {
                return 0;
            }
            err = intel_bts_synth_error((*btsq).bts, (*btsq).cpu, (*btsq).pid, (*btsq).tid, (*branch).from);
            return err;
        }
        (*btsq).sample_flags = intel_pt_insn_type((*btsq).intel_pt_insn.op);
        /* Check for an async branch into the kernel */
        if !machine__kernel_ip((*(*btsq).bts).machine, (*branch).from)
            && machine__kernel_ip((*(*btsq).bts).machine, (*branch).to)
            && (*btsq).sample_flags != (PERF_IP_FLAG_BRANCH | PERF_IP_FLAG_CALL | PERF_IP_FLAG_SYSCALLRET)
        {
            (*btsq).sample_flags = PERF_IP_FLAG_BRANCH | PERF_IP_FLAG_CALL | PERF_IP_FLAG_ASYNC | PERF_IP_FLAG_INTERRUPT;
        }
    }

    0
}

unsafe fn intel_bts_process_buffer(btsq: *mut intel_bts_queue, buffer: *mut auxtrace_buffer, thread: *mut thread) -> c_int {
    let mut branch_: *mut branch;
    let mut sz: size_t;
    let bsz: size_t = size_of::<branch>();
    let filter: u32 = (*(*btsq).bts).branches_filter;
    let mut err: c_int = 0;

    if !(*buffer).use_data.is_null() {
        sz = (*buffer).use_size;
        branch_ = (*buffer).use_data as *mut branch;
    } else {
        sz = (*buffer).size;
        branch_ = (*buffer).data as *mut branch;
    }

    if !(*(*btsq).bts).sample_branches {
        return 0;
    }

    while sz > bsz {
        if (*branch_).from == 0 && (*branch_).to == 0 {
            branch_ = branch_.add(1);
            sz -= bsz;
            continue;
        }
        intel_bts_get_branch_type(btsq, branch_);
        if (*(*btsq).bts).synth_opts.thread_stack {
            thread_stack__event(thread, (*btsq).cpu, (*btsq).sample_flags,
                                le64_to_cpu((*branch_).from),
                                le64_to_cpu((*branch_).to),
                                (*btsq).intel_pt_insn.length,
                                (*buffer).buffer_nr + 1, true, 0, 0);
        }
        if filter != 0 && (filter & (*btsq).sample_flags) == 0 {
            branch_ = branch_.add(1);
            sz -= bsz;
            continue;
        }
        err = intel_bts_synth_branch_sample(btsq, branch_);
        if err != 0 {
            break;
        }
        branch_ = branch_.add(1);
        sz -= bsz;
    }
    err
}

unsafe fn intel_bts_process_queue(btsq: *mut intel_bts_queue, timestamp: *mut u64) -> c_int {
    let mut buffer = (*btsq).buffer;
    let old_buffer = buffer;
    let queue: *mut auxtrace_queue;
    let thread: *mut thread;
    let mut err: c_int;

    if (*btsq).done {
        return 1;
    }

    if (*btsq).pid == -1 {
        thread = machine__find_thread((*(*btsq).bts).machine, -1, (*btsq).tid);
        if !thread.is_null() {
            (*btsq).pid = thread__pid(thread);
        }
    } else {
        thread = machine__findnew_thread((*(*btsq).bts).machine, (*btsq).pid, (*btsq).tid);
    }

    queue = (*(*btsq).bts).queues.queue_array.add((*btsq).queue_nr as usize);

    if buffer.is_null() {
        buffer = auxtrace_buffer__next(queue, ptr::null_mut());
    }

    if buffer.is_null() {
        if !(*(*btsq).bts).sampling_mode {
            (*btsq).done = true;
        }
        err = 1;
        thread__put(thread);
        return err;
    }

    /* Currently there is no support for split buffers */
    if (*buffer).consecutive {
        err = -EINVAL;
        thread__put(thread);
        return err;
    }

    if (*buffer).data.is_null() {
        let fd = perf_data__fd((*(*(*btsq).bts).session).data);

        (*buffer).data = auxtrace_buffer__get_data(buffer, fd);
        if (*buffer).data.is_null() {
            err = -ENOMEM;
            thread__put(thread);
            return err;
        }
    }

    if (*(*btsq).bts).snapshot_mode && !(*buffer).consecutive
        && intel_bts_do_fix_overlap(queue, buffer) != 0
    {
        err = -ENOMEM;
        thread__put(thread);
        return err;
    }

    if !(*(*btsq).bts).synth_opts.callchain
        && !(*(*btsq).bts).synth_opts.thread_stack
        && !thread.is_null()
        && (old_buffer.is_null()
            || (*(*btsq).bts).sampling_mode
            || ((*(*btsq).bts).snapshot_mode && !(*buffer).consecutive))
    {
        thread_stack__set_trace_nr(thread, (*btsq).cpu, (*buffer).buffer_nr + 1);
    }

    err = intel_bts_process_buffer(btsq, buffer, thread);

    auxtrace_buffer__drop_data(buffer);

    (*btsq).buffer = auxtrace_buffer__next(queue, buffer);
    if !(*btsq).buffer.is_null() {
        if !timestamp.is_null() {
            *timestamp = (*(*btsq).buffer).reference;
        }
    } else if !(*(*btsq).bts).sampling_mode {
        (*btsq).done = true;
    }
    thread__put(thread);
    err
}

unsafe fn intel_bts_flush_queue(btsq: *mut intel_bts_queue) -> c_int {
    let mut ts: u64 = 0;
    let mut ret: c_int;

    loop {
        ret = intel_bts_process_queue(btsq, &mut ts);
        if ret < 0 {
            return ret;
        }
        if ret != 0 {
            break;
        }
    }
    0
}

unsafe fn intel_bts_process_tid_exit(bts: *mut intel_bts, tid: pid_t) -> c_int {
    let queues = &mut (*bts).queues as *mut auxtrace_queues;
    let mut i: c_uint = 0;

    while i < (*queues).nr_queues {
        let queue = (*bts).queues.queue_array.add(i as usize);
        let btsq = (*queue).priv_ as *mut intel_bts_queue;

        if !btsq.is_null() && (*btsq).tid == tid {
            return intel_bts_flush_queue(btsq);
        }
        i += 1;
    }
    0
}

unsafe fn intel_bts_process_queues(bts: *mut intel_bts, timestamp: u64) -> c_int {
    loop {
        let queue_nr: c_uint;
        let queue: *mut auxtrace_queue;
        let btsq: *mut intel_bts_queue;
        let mut ts: u64 = 0;
        let mut ret: c_int;

        if (*bts).heap.heap_cnt == 0 {
            return 0;
        }

        if (*(*bts).heap.heap_array.add(0)).ordinal > timestamp {
            return 0;
        }

        queue_nr = (*(*bts).heap.heap_array.add(0)).queue_nr;
        queue = (*bts).queues.queue_array.add(queue_nr as usize);
        btsq = (*queue).priv_ as *mut intel_bts_queue;

        auxtrace_heap__pop(&mut (*bts).heap);

        ret = intel_bts_process_queue(btsq, &mut ts);
        if ret < 0 {
            auxtrace_heap__add(&mut (*bts).heap, queue_nr, ts);
            return ret;
        }

        if ret == 0 {
            ret = auxtrace_heap__add(&mut (*bts).heap, queue_nr, ts);
            if ret < 0 {
                return ret;
            }
        } else {
            (*btsq).on_heap = false;
        }
    }
}

unsafe extern "C" fn intel_bts_process_event(session: *mut perf_session, event: *mut perf_event, sample: *mut perf_sample, tool: *const perf_tool) -> c_int {
    let bts = container_of_auxtrace((*session).auxtrace);
    let timestamp: u64;
    let mut err: c_int;

    if dump_trace {
        return 0;
    }

    if !(*tool).ordered_events {
        pr_err(b"Intel BTS requires ordered events\n\0".as_ptr() as *const c_char);
        return -EINVAL;
    }

    if (*sample).time != 0 && (*sample).time != !0u64 {
        timestamp = perf_time_to_tsc((*sample).time, &mut (*bts).tc);
    } else {
        timestamp = 0;
    }

    err = intel_bts_update_queues(bts);
    if err != 0 {
        return err;
    }

    err = intel_bts_process_queues(bts, timestamp);
    if err != 0 {
        return err;
    }
    if (*event).header.type_ == PERF_RECORD_EXIT {
        err = intel_bts_process_tid_exit(bts, (*event).fork.tid);
        if err != 0 {
            return err;
        }
    }

    if (*event).header.type_ == PERF_RECORD_AUX
        && ((*event).aux.flags & PERF_AUX_FLAG_TRUNCATED) != 0
        && (*bts).synth_opts.errors
    {
        err = intel_bts_lost(bts, sample);
    }

    err
}

unsafe extern "C" fn intel_bts_process_auxtrace_event(session: *mut perf_session, event: *mut perf_event, _tool: *const perf_tool) -> c_int {
    let bts = container_of_auxtrace((*session).auxtrace);

    if (*bts).sampling_mode {
        return 0;
    }

    if !(*bts).data_queued {
        let mut buffer: *mut auxtrace_buffer = ptr::null_mut();
        let data_offset: off_t;
        let fd = perf_data__fd((*session).data);
        let err: c_int;

        if perf_data__is_pipe((*session).data) {
            data_offset = 0;
        } else {
            data_offset = lseek(fd, 0, SEEK_CUR);
            if data_offset == -1 {
                return -errno;
            }
        }

        err = auxtrace_queues__add_event(&mut (*bts).queues, session, event, data_offset, &mut buffer);
        if err != 0 {
            return err;
        }

        /* Dump here now we have copied a piped trace out of the pipe */
        if dump_trace {
            if !auxtrace_buffer__get_data(buffer, fd).is_null() {
                intel_bts_dump_event(bts, (*buffer).data, (*buffer).size);
                auxtrace_buffer__put_data(buffer);
            }
        }
    }

    0
}

unsafe extern "C" fn intel_bts_flush(session: *mut perf_session, tool: *const perf_tool) -> c_int {
    let bts = container_of_auxtrace((*session).auxtrace);
    let ret: c_int;

    if dump_trace || (*bts).sampling_mode {
        return 0;
    }

    if !(*tool).ordered_events {
        return -EINVAL;
    }

    ret = intel_bts_update_queues(bts);
    if ret < 0 {
        return ret;
    }

    intel_bts_process_queues(bts, MAX_TIMESTAMP)
}

unsafe extern "C" fn intel_bts_free_queue(priv_: *mut c_void) {
    let btsq = priv_ as *mut intel_bts_queue;

    if btsq.is_null() {
        return;
    }
    free(btsq as *mut c_void);
}

unsafe extern "C" fn intel_bts_free_events(session: *mut perf_session) {
    let bts = container_of_auxtrace((*session).auxtrace);
    let queues = &mut (*bts).queues as *mut auxtrace_queues;
    let mut i: c_uint = 0;

    while i < (*queues).nr_queues {
        let q = (*queues).queue_array.add(i as usize);
        intel_bts_free_queue((*q).priv_);
        (*q).priv_ = ptr::null_mut();
        i += 1;
    }
    auxtrace_queues__free(queues);
}

unsafe extern "C" fn intel_bts_free(session: *mut perf_session) {
    let bts = container_of_auxtrace((*session).auxtrace);

    auxtrace_heap__free(&mut (*bts).heap);
    intel_bts_free_events(session);
    (*session).auxtrace = ptr::null_mut();
    free(bts as *mut c_void);
}

unsafe extern "C" fn intel_bts_evsel_is_auxtrace(session: *mut perf_session, evsel: *mut evsel) -> bool {
    let bts = container_of_auxtrace((*session).auxtrace);

    (*evsel).core.attr.type_ == (*bts).pmu_type
}

unsafe fn intel_bts_synth_events(bts: *mut intel_bts, session: *mut perf_session) -> c_int {
    let evlist = (*session).evlist;
    let mut evsel: *mut evsel = ptr::null_mut();
    let mut attr: perf_event_attr = zeroed();
    let mut found = false;
    let id: u64;
    let err: c_int;

    /* evlist__for_each_entry(evlist, evsel) */
    unsafe extern "C" {
        fn evlist__first(evlist: *mut evlist) -> *mut evsel;
        fn evlist__next(evlist: *mut evlist, evsel: *mut evsel) -> *mut evsel;
    }
    evsel = evlist__first(evlist);
    while !evsel.is_null() {
        if (*evsel).core.attr.type_ == (*bts).pmu_type && !(*evsel).core.ids.is_null() {
            found = true;
            break;
        }
        evsel = evlist__next(evlist, evsel);
    }

    if !found {
        pr_debug(b"There are no selected events with Intel BTS data\n\0".as_ptr() as *const c_char);
        return 0;
    }

    memset(&mut attr as *mut _ as *mut c_void, 0, size_of::<perf_event_attr>());
    attr.size = size_of::<perf_event_attr>() as u32;
    attr.type_ = PERF_TYPE_HARDWARE;
    attr.sample_type = (*evsel).core.attr.sample_type & PERF_SAMPLE_MASK;
    attr.sample_type |= PERF_SAMPLE_IP | PERF_SAMPLE_TID | PERF_SAMPLE_PERIOD;
    attr.sample_type &= !(PERF_SAMPLE_TIME as u64);
    attr.sample_type &= !(PERF_SAMPLE_CPU as u64);
    attr.exclude_user = (*evsel).core.attr.exclude_user;
    attr.exclude_kernel = (*evsel).core.attr.exclude_kernel;
    attr.exclude_hv = (*evsel).core.attr.exclude_hv;
    attr.exclude_host = (*evsel).core.attr.exclude_host;
    attr.exclude_guest = (*evsel).core.attr.exclude_guest;
    attr.sample_id_all = (*evsel).core.attr.sample_id_all;
    attr.read_format = (*evsel).core.attr.read_format;

    id = auxtrace_synth_id_range_start(evsel);

    if (*bts).synth_opts.branches {
        attr.config = PERF_COUNT_HW_BRANCH_INSTRUCTIONS;
        attr.sample_period = 1;
        attr.sample_type |= PERF_SAMPLE_ADDR;
        pr_debug(b"Synthesizing 'branches' event with id %lu sample type %#lx\n\0".as_ptr() as *const c_char, id, attr.sample_type);
        let e = perf_session__deliver_synth_attr_event(session, &mut attr, id);
        if e != 0 {
            pr_err(b"%s: failed to synthesize 'branches' event type\n\0".as_ptr() as *const c_char, b"intel_bts_synth_events\0".as_ptr() as *const c_char);
            return e;
        }
        (*bts).sample_branches = true;
        (*bts).branches_sample_type = attr.sample_type;
        (*bts).branches_id = id;
        /*
         * We only use sample types from PERF_SAMPLE_MASK so we can use
         * __evsel__sample_size() here.
         */
        (*bts).branches_event_size = size_of::<perf_record_sample>() + __evsel__sample_size(attr.sample_type);
    }

    0
}

const intel_bts_info_fmts: [*const c_char; 6] = [
    b"  PMU Type           %ld\n\0".as_ptr() as *const c_char,
    b"  Time Shift         %lu\n\0".as_ptr() as *const c_char,
    b"  Time Multiplier    %lu\n\0".as_ptr() as *const c_char,
    b"  Time Zero          %lu\n\0".as_ptr() as *const c_char,
    b"  Cap Time Zero      %ld\n\0".as_ptr() as *const c_char,
    b"  Snapshot mode      %ld\n\0".as_ptr() as *const c_char,
];

unsafe fn intel_bts_print_info(arr: *mut u64, start: c_int, finish: c_int) {
    let mut i = start;

    if !dump_trace {
        return;
    }

    while i <= finish {
        fprintf(stdout, intel_bts_info_fmts[i as usize], *arr.add(i as usize));
        i += 1;
    }
}

#[no_mangle]
pub unsafe extern "C" fn intel_bts_process_auxtrace_info(event: *mut perf_event, session: *mut perf_session) -> c_int {
    let auxtrace_info = &mut (*event).auxtrace_info as *mut _ as *mut perf_record_auxtrace_info;
    let min_sz: size_t = size_of::<u64>() * (INTEL_BTS_SNAPSHOT_MODE + 1);
    let bts: *mut intel_bts;
    let mut err: c_int;

    if (*auxtrace_info).header.size as size_t
        < size_of::<perf_record_auxtrace_info>() + min_sz
    {
        return -EINVAL;
    }

    bts = zalloc(size_of::<intel_bts>()) as *mut intel_bts;
    if bts.is_null() {
        return -ENOMEM;
    }

    err = auxtrace_queues__init(&mut (*bts).queues);
    if err != 0 {
        free(bts as *mut c_void);
        return err;
    }

    (*bts).session = session;
    (*bts).machine = &mut (*session).machines.host; /* No kvm support */
    (*bts).auxtrace_type = (*auxtrace_info).type_;
    let priv_arr = (*auxtrace_info).priv_.as_ptr() as *mut u64;
    (*bts).pmu_type = *priv_arr.add(INTEL_BTS_PMU_TYPE) as u32;
    (*bts).tc.time_shift = *priv_arr.add(INTEL_BTS_TIME_SHIFT) as u16;
    (*bts).tc.time_mult = *priv_arr.add(INTEL_BTS_TIME_MULT) as u32;
    (*bts).tc.time_zero = *priv_arr.add(INTEL_BTS_TIME_ZERO);
    (*bts).cap_user_time_zero = *priv_arr.add(INTEL_BTS_CAP_USER_TIME_ZERO) != 0;
    (*bts).snapshot_mode = *priv_arr.add(INTEL_BTS_SNAPSHOT_MODE) != 0;

    (*bts).sampling_mode = false;

    (*bts).auxtrace.process_event = Some(intel_bts_process_event);
    (*bts).auxtrace.process_auxtrace_event = Some(intel_bts_process_auxtrace_event);
    (*bts).auxtrace.flush_events = Some(intel_bts_flush);
    (*bts).auxtrace.free_events = Some(intel_bts_free_events);
    (*bts).auxtrace.free = Some(intel_bts_free);
    (*bts).auxtrace.evsel_is_auxtrace = Some(intel_bts_evsel_is_auxtrace);
    (*session).auxtrace = &mut (*bts).auxtrace;

    intel_bts_print_info(priv_arr, INTEL_BTS_PMU_TYPE as c_int, INTEL_BTS_SNAPSHOT_MODE as c_int);

    if dump_trace {
        return 0;
    }

    if (*(*session).itrace_synth_opts).set {
        (*bts).synth_opts = *(*session).itrace_synth_opts;
    } else {
        itrace_synth_opts__set_default(&mut (*bts).synth_opts, (*(*session).itrace_synth_opts).default_no_sample);
        (*bts).synth_opts.thread_stack = (*(*session).itrace_synth_opts).thread_stack;
    }

    if (*bts).synth_opts.calls {
        (*bts).branches_filter |= PERF_IP_FLAG_CALL | PERF_IP_FLAG_ASYNC | PERF_IP_FLAG_TRACE_END;
    }
    if (*bts).synth_opts.returns {
        (*bts).branches_filter |= PERF_IP_FLAG_RETURN | PERF_IP_FLAG_TRACE_BEGIN;
    }

    err = intel_bts_synth_events(bts, session);
    if err != 0 {
        auxtrace_queues__free(&mut (*bts).queues);
        (*session).auxtrace = ptr::null_mut();
        free(bts as *mut c_void);
        return err;
    }

    err = auxtrace_queues__process_index(&mut (*bts).queues, session);
    if err != 0 {
        auxtrace_queues__free(&mut (*bts).queues);
        (*session).auxtrace = ptr::null_mut();
        free(bts as *mut c_void);
        return err;
    }

    if (*bts).queues.populated {
        (*bts).data_queued = true;
    }

    0
}
