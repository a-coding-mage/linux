// SPDX-License-Identifier: GPL-2.0
/*
 * VPA DTL PMU support
 */

use core::ffi::{c_char, c_int, c_uint, c_ulong, c_void};

type u32 = u32;
type u64 = u64;
type size_t = usize;
type pid_t = c_int;
type off_t = i64;
type bool_ = bool;
type __u64 = u64;

const PERF_COLOR_BLUE: *const c_char = b"blue\0".as_ptr() as *const c_char;
const PERF_RECORD_MISC_KERNEL: u16 = 1 << 0;
const PERF_RECORD_SAMPLE: u32 = 9;
const PERF_TYPE_SYNTH: u32 = 1;
const PERF_SYNTH_POWERPC_VPA_DTL: u64 = 0;
const POWERPC_VPADTL_TYPE: usize = 0;
const ENOMEM: c_int = 12;
const EINVAL: c_int = 22;
const SEEK_CUR: c_int = 1;

#[repr(C)]
pub struct list_head {
    _private: [u8; 0],
}

#[repr(C)]
pub struct auxtrace {
    process_event: Option<
        unsafe extern "C" fn(
            *mut perf_session,
            *mut perf_event,
            *mut perf_sample,
            *const perf_tool,
        ) -> c_int,
    >,
    process_auxtrace_event:
        Option<unsafe extern "C" fn(*mut perf_session, *mut perf_event, *const perf_tool) -> c_int>,
    flush_events: Option<unsafe extern "C" fn(*mut perf_session, *const perf_tool) -> c_int>,
    free_events: Option<unsafe extern "C" fn(*mut perf_session)>,
    free: Option<unsafe extern "C" fn(*mut perf_session)>,
}

#[repr(C)]
pub struct auxtrace_queues {
    queue_array: *mut auxtrace_queue,
    nr_queues: c_uint,
    new_data: bool,
}

#[repr(C)]
pub struct auxtrace_heap {
    heap_array: *mut auxtrace_heap_item,
    heap_cnt: c_uint,
}

#[repr(C)]
pub struct auxtrace_heap_item {
    ordinal: u64,
    queue_nr: c_uint,
}

#[repr(C)]
pub struct perf_session {
    auxtrace: *mut auxtrace,
    data: *mut perf_data,
    evlist: *mut evlist,
    machines: machines,
}

#[repr(C)]
pub struct machines {
    host: machine,
}

#[repr(C)]
pub struct perf_data {
    _private: [u8; 0],
}

#[repr(C)]
pub struct machine {
    _private: [u8; 0],
}

#[repr(C)]
pub struct auxtrace_buffer {
    data: *mut c_void,
    size: size_t,
    buffer_nr: c_uint,
}

#[repr(C)]
pub struct auxtrace_queue {
    head: list_head,
    priv_: *mut c_void,
    cpu: c_int,
}

#[repr(C)]
pub struct thread {
    _private: [u8; 0],
}

#[repr(C)]
pub struct powerpc_vpadtl_entry {
    timebase: u64,
    enqueue_to_dispatch_time: u32,
    ready_to_enqueue_time: u32,
    waiting_to_ready_time: u32,
    dispatch_reason: usize,
    preempt_reason: usize,
    srr0: u64,
}

/*
 * Structure to save the auxtrace queue
 */
#[repr(C)]
pub struct powerpc_vpadtl {
    auxtrace: auxtrace,
    queues: auxtrace_queues,
    heap: auxtrace_heap,
    auxtrace_type: u32,
    session: *mut perf_session,
    machine: *mut machine,
    pmu_type: u32,
    sample_id: u64,
}

#[repr(C)]
pub struct boottb_freq {
    boot_tb: u64,
    tb_freq: u64,
    timebase: u64,
    padded: [u64; 3],
}

#[repr(C)]
pub struct powerpc_vpadtl_queue {
    vpa: *mut powerpc_vpadtl,
    queue_nr: c_uint,
    buffer: *mut auxtrace_buffer,
    thread: *mut thread,
    on_heap: bool,
    dtl: *mut powerpc_vpadtl_entry,
    timestamp: u64,
    pkt_len: c_ulong,
    buf_len: c_ulong,
    boot_tb: u64,
    tb_freq: u64,
    tb_buffer: c_uint,
    size: c_uint,
    done: bool,
    pid: pid_t,
    tid: pid_t,
    cpu: c_int,
}

static dispatch_reasons: [*const c_char; 11] = [
    b"external_interrupt\0".as_ptr() as *const c_char,
    b"firmware_internal_event\0".as_ptr() as *const c_char,
    b"H_PROD\0".as_ptr() as *const c_char,
    b"decrementer_interrupt\0".as_ptr() as *const c_char,
    b"system_reset\0".as_ptr() as *const c_char,
    b"firmware_internal_event\0".as_ptr() as *const c_char,
    b"conferred_cycles\0".as_ptr() as *const c_char,
    b"time_slice\0".as_ptr() as *const c_char,
    b"virtual_memory_page_fault\0".as_ptr() as *const c_char,
    b"expropriated_adjunct\0".as_ptr() as *const c_char,
    b"priv_doorbell\0".as_ptr() as *const c_char,
];

static preempt_reasons: [*const c_char; 10] = [
    b"unused\0".as_ptr() as *const c_char,
    b"firmware_internal_event\0".as_ptr() as *const c_char,
    b"H_CEDE\0".as_ptr() as *const c_char,
    b"H_CONFER\0".as_ptr() as *const c_char,
    b"time_slice\0".as_ptr() as *const c_char,
    b"migration_hibernation_page_fault\0".as_ptr() as *const c_char,
    b"virtual_memory_page_fault\0".as_ptr() as *const c_char,
    b"H_CONFER_ADJUNCT\0".as_ptr() as *const c_char,
    b"hcall_adjunct\0".as_ptr() as *const c_char,
    b"HDEC_adjunct\0".as_ptr() as *const c_char,
];

const dtl_entry_size: usize = core::mem::size_of::<powerpc_vpadtl_entry>();

#[repr(C)]
pub struct perf_event_header {
    type_: u32,
    misc: u16,
    size: u16,
}

#[repr(C)]
pub struct perf_event_sample {
    header: perf_event_header,
}

#[repr(C)]
pub struct perf_record_auxtrace_info {
    header: perf_event_header,
    type_: u32,
    priv_: [u64; 1],
}

#[repr(C)]
pub union perf_event {
    sample: core::mem::ManuallyDrop<perf_event_sample>,
    auxtrace_info: core::mem::ManuallyDrop<perf_record_auxtrace_info>,
}

#[repr(C)]
pub struct perf_sample {
    ip: u64,
    period: u64,
    cpu: c_int,
    id: u64,
    callchain: *mut c_void,
    branch_stack: *mut c_void,
    cpumode: u16,
    time: u64,
    raw_data: *mut c_void,
    raw_size: u32,
}

#[repr(C)]
pub struct perf_tool {
    ordered_events: bool,
}

#[repr(C)]
pub struct evlist {
    _private: [u8; 0],
}

#[repr(C)]
pub struct evsel {
    core: evsel_core,
    name: *mut c_char,
}

#[repr(C)]
pub struct evsel_core {
    id: *mut u64,
    attr: perf_event_attr,
}

#[repr(C)]
pub struct perf_event_attr {
    size: u32,
    sample_type: u64,
    sample_id_all: u32,
    type_: u32,
    config: u64,
}

extern "C" {
    static mut stdout: *mut c_void;
    static mut errno: c_int;
    static mut dump_trace: bool;

    fn color_fprintf(stream: *mut c_void, color: *const c_char, fmt: *const c_char, ...) -> c_int;
    fn printf(fmt: *const c_char, ...) -> c_int;
    fn fprintf(stream: *mut c_void, fmt: *const c_char, ...) -> c_int;
    fn memset(s: *mut c_void, c: c_int, n: usize) -> *mut c_void;
    fn strdup(s: *const c_char) -> *mut c_char;
    fn free(ptr: *mut c_void);
    fn lseek(fd: c_int, offset: off_t, whence: c_int) -> off_t;
    fn zalloc(size: usize) -> *mut c_void;
    fn zfree(ptr: *mut *mut c_void);
    fn be32_to_cpu(x: u32) -> u32;
    fn be64_to_cpu(x: u64) -> u64;
    fn perf_sample__init(sample: *mut perf_sample, all: bool);
    fn perf_sample__exit(sample: *mut perf_sample);
    fn perf_session__deliver_synth_event(
        session: *mut perf_session,
        event: *mut perf_event,
        sample: *mut perf_sample,
    ) -> c_int;
    fn perf_data__fd(data: *mut perf_data) -> c_int;
    fn perf_data__is_pipe(data: *mut perf_data) -> bool;
    fn auxtrace_buffer__next(
        queue: *mut auxtrace_queue,
        buffer: *mut auxtrace_buffer,
    ) -> *mut auxtrace_buffer;
    fn auxtrace_buffer__get_data(buffer: *mut auxtrace_buffer, fd: c_int) -> *mut c_void;
    fn auxtrace_buffer__put_data(buffer: *mut auxtrace_buffer);
    fn auxtrace_queues__add_event(
        queues: *mut auxtrace_queues,
        session: *mut perf_session,
        event: *mut perf_event,
        data_offset: off_t,
        buffer: *mut *mut auxtrace_buffer,
    ) -> c_int;
    fn auxtrace_queues__init(queues: *mut auxtrace_queues) -> c_int;
    fn auxtrace_queues__free(queues: *mut auxtrace_queues);
    fn auxtrace_queues__process_index(queues: *mut auxtrace_queues, session: *mut perf_session)
        -> c_int;
    fn auxtrace_heap__add(heap: *mut auxtrace_heap, queue_nr: c_uint, ordinal: u64) -> c_int;
    fn auxtrace_heap__pop(heap: *mut auxtrace_heap);
    fn auxtrace_heap__free(heap: *mut auxtrace_heap);
    fn list_empty(head: *const list_head) -> bool;
    fn pr_debug(fmt: *const c_char, ...);
    fn pr_err(fmt: *const c_char, ...);
    fn strstarts(str_: *const c_char, prefix: *const c_char) -> bool;
    fn auxtrace_synth_id_range_start(evsel: *mut evsel) -> u64;
    fn perf_session__deliver_synth_attr_event(
        session: *mut perf_session,
        attr: *mut perf_event_attr,
        id: u64,
    ) -> c_int;
    fn evlist__first(evlist: *mut evlist) -> *mut evsel;
    fn evlist__next(evsel: *mut evsel) -> *mut evsel;
}

unsafe fn session_to_vpa(session: *mut perf_session) -> *mut powerpc_vpadtl {
    (*session).auxtrace as *mut powerpc_vpadtl
}

/*
 * Function to dump the dispatch trace data when perf report
 * is invoked with -D
 */
unsafe fn powerpc_vpadtl_dump(
    _vpa: *mut powerpc_vpadtl,
    mut buf: *mut u8,
    mut len: size_t,
) {
    let mut dtl: *mut powerpc_vpadtl_entry;
    let mut pkt_len: c_int;
    let mut pos: c_int = 0;
    let color = PERF_COLOR_BLUE;

    color_fprintf(
        stdout,
        color,
        b". ... VPA DTL PMU data: size %zu bytes, entries is %zu\n\0".as_ptr() as *const c_char,
        len,
        len / dtl_entry_size,
    );

    if len % dtl_entry_size != 0 {
        len -= len % dtl_entry_size;
    }

    while len != 0 {
        pkt_len = dtl_entry_size as c_int;
        printf(b".\0".as_ptr() as *const c_char);
        color_fprintf(stdout, color, b"  %08x: \0".as_ptr() as *const c_char, pos);
        dtl = buf as *mut powerpc_vpadtl_entry;
        if (*dtl).timebase != 0 {
            printf(
                b"dispatch_reason:%s, preempt_reason:%s, enqueue_to_dispatch_time:%d, ready_to_enqueue_time:%d, waiting_to_ready_time:%d\n\0".as_ptr() as *const c_char,
                dispatch_reasons[(*dtl).dispatch_reason],
                preempt_reasons[(*dtl).preempt_reason],
                be32_to_cpu((*dtl).enqueue_to_dispatch_time),
                be32_to_cpu((*dtl).ready_to_enqueue_time),
                be32_to_cpu((*dtl).waiting_to_ready_time),
            );
        } else {
            let boot_tb = buf as *mut boottb_freq;

            printf(
                b"boot_tb: %llu, tb_freq: %llu\n\0".as_ptr() as *const c_char,
                (*boot_tb).boot_tb,
                (*boot_tb).tb_freq,
            );
        }

        pos += pkt_len;
        buf = buf.add(pkt_len as usize);
        len -= pkt_len as usize;
    }
}

unsafe fn powerpc_vpadtl_timestamp(vpaq: *mut powerpc_vpadtl_queue) -> u64 {
    let record = (*vpaq).dtl;
    let mut timestamp: u64 = 0;
    let boot_tb: u64;
    let diff: u64;
    let result: f64;
    let div: f64;
    let boot_freq: f64;
    /*
     * Formula used to get timestamp that can be co-related with
     * other perf events:
     * ((timbase from DTL entry - boot time) / frequency) * 1000000000
     */
    if (*record).timebase != 0 {
        boot_tb = (*vpaq).boot_tb;
        boot_freq = (*vpaq).tb_freq as f64;
        diff = be64_to_cpu((*record).timebase).wrapping_sub(boot_tb);
        div = (diff as f64) / boot_freq;
        result = div * 1000000000.0;
        timestamp = result as u64;
    }

    timestamp
}

unsafe fn powerpc_vpadtl_dump_event(vpa: *mut powerpc_vpadtl, buf: *mut u8, len: size_t) {
    printf(b".\n\0".as_ptr() as *const c_char);
    powerpc_vpadtl_dump(vpa, buf, len);
}

/*
 * Generate perf sample for each entry in the dispatch trace log.
 */
unsafe fn powerpc_vpadtl_sample(
    record: *mut powerpc_vpadtl_entry,
    vpa: *mut powerpc_vpadtl,
    save: u64,
    cpu: c_int,
) -> c_int {
    let mut sample: perf_sample = core::mem::zeroed();
    let mut event: perf_event = core::mem::zeroed();
    let ret: c_int;

    perf_sample__init(&mut sample, true);
    sample.ip = be64_to_cpu((*record).srr0);
    sample.period = 1;
    sample.cpu = cpu;
    sample.id = (*vpa).sample_id;
    sample.callchain = core::ptr::null_mut();
    sample.branch_stack = core::ptr::null_mut();
    memset(&mut event as *mut _ as *mut c_void, 0, core::mem::size_of::<perf_event>());
    sample.cpumode = PERF_RECORD_MISC_KERNEL;
    sample.time = save;
    sample.raw_data = record as *mut c_void;
    sample.raw_size = core::mem::size_of_val(&record) as u32;
    (*event.sample).header.type_ = PERF_RECORD_SAMPLE;
    (*event.sample).header.misc = sample.cpumode;
    (*event.sample).header.size = core::mem::size_of::<perf_event_header>() as u16;

    ret = perf_session__deliver_synth_event((*vpa).session, &mut event, &mut sample);
    if ret != 0 {
        pr_debug(b"Failed to create sample for dtl entry\n\0".as_ptr() as *const c_char);
    }

    perf_sample__exit(&mut sample);
    ret
}

unsafe fn powerpc_vpadtl_get_buffer(vpaq: *mut powerpc_vpadtl_queue) -> c_int {
    let mut buffer = (*vpaq).buffer;
    let queues = &mut (*(*vpaq).vpa).queues as *mut auxtrace_queues;
    let queue = (*queues).queue_array.add((*vpaq).queue_nr as usize);
    buffer = auxtrace_buffer__next(queue, buffer);

    if buffer.is_null() {
        return 0;
    }

    (*vpaq).buffer = buffer;
    (*vpaq).size = (*buffer).size as c_uint;

    /* If the aux_buffer doesn't have data associated, try to load it */
    if (*buffer).data.is_null() {
        /* get the file desc associated with the perf data file */
        let fd = perf_data__fd((*(*vpaq).vpa).session.as_ref().unwrap().data);

        (*buffer).data = auxtrace_buffer__get_data(buffer, fd);
        if (*buffer).data.is_null() {
            return -ENOMEM;
        }
    }

    (*vpaq).buf_len = (*buffer).size as c_ulong;

    if (*buffer).size % dtl_entry_size != 0 {
        (*vpaq).buf_len = ((*buffer).size - ((*buffer).size % dtl_entry_size)) as c_ulong;
    }

    if (*vpaq).tb_buffer != (*buffer).buffer_nr {
        (*vpaq).pkt_len = 0;
        (*vpaq).tb_buffer = 0;
    }

    1
}

/*
 * The first entry in the queue for VPA DTL PMU has the boot timebase,
 * frequency details which are needed to get timestamp which is required to
 * correlate with other events. Save the boot_tb and tb_freq as part of
 * powerpc_vpadtl_queue. The very next entry is the actual trace data to
 * be returned.
 */
unsafe fn powerpc_vpadtl_decode(vpaq: *mut powerpc_vpadtl_queue) -> c_int {
    let ret = powerpc_vpadtl_get_buffer(vpaq);
    if ret <= 0 {
        return ret;
    }

    let boottb = (*(*vpaq).buffer).data as *mut boottb_freq;
    if (*boottb).timebase == 0 {
        (*vpaq).boot_tb = (*boottb).boot_tb;
        (*vpaq).tb_freq = (*boottb).tb_freq;
        (*vpaq).pkt_len += dtl_entry_size as c_ulong;
    }

    let mut buf = (*(*vpaq).buffer).data as *mut u8;
    buf = buf.add((*vpaq).pkt_len as usize);
    (*vpaq).dtl = buf as *mut powerpc_vpadtl_entry;

    (*vpaq).tb_buffer = (*(*vpaq).buffer).buffer_nr;
    (*vpaq).buffer = core::ptr::null_mut();
    (*vpaq).buf_len = 0;

    1
}

unsafe fn powerpc_vpadtl_decode_all(vpaq: *mut powerpc_vpadtl_queue) -> c_int {
    if (*vpaq).buf_len == 0 || (*vpaq).pkt_len == (*vpaq).size as c_ulong {
        let ret = powerpc_vpadtl_get_buffer(vpaq);
        if ret <= 0 {
            return ret;
        }
    }

    if !(*vpaq).buffer.is_null() {
        let mut buf = (*(*vpaq).buffer).data as *mut u8;
        buf = buf.add((*vpaq).pkt_len as usize);
        (*vpaq).dtl = buf as *mut powerpc_vpadtl_entry;
        if (be64_to_cpu((*(*vpaq).dtl).timebase) as i64) <= 0 {
            if (*vpaq).pkt_len != dtl_entry_size as c_ulong && (*vpaq).buf_len != 0 {
                (*vpaq).pkt_len += dtl_entry_size as c_ulong;
                (*vpaq).buf_len -= dtl_entry_size as c_ulong;
            }
            return -1;
        }
        (*vpaq).pkt_len += dtl_entry_size as c_ulong;
        (*vpaq).buf_len -= dtl_entry_size as c_ulong;
    } else {
        return 0;
    }

    1
}

unsafe fn powerpc_vpadtl_run_decoder(vpaq: *mut powerpc_vpadtl_queue, timestamp: *mut u64) -> c_int {
    let vpa = (*vpaq).vpa;

    loop {
        let mut ret = powerpc_vpadtl_decode_all(vpaq);
        if ret == 0 {
            pr_debug(b"All data in the queue has been processed.\n\0".as_ptr() as *const c_char);
            return 1;
        }

        /*
         * Error is detected when decoding VPA PMU trace. Continue to
         * the next trace data and find out more dtl entries.
         */
        if ret < 0 {
            continue;
        }

        let record = (*vpaq).dtl;
        let vpaq_timestamp = powerpc_vpadtl_timestamp(vpaq);

        /* Update timestamp for the last record */
        if vpaq_timestamp > (*vpaq).timestamp {
            (*vpaq).timestamp = vpaq_timestamp;
        }

        /*
         * If the timestamp of the queue is later than timestamp of the
         * coming perf event, bail out so can allow the perf event to
         * be processed ahead.
         */
        if (*vpaq).timestamp >= *timestamp {
            *timestamp = (*vpaq).timestamp;
            (*vpaq).pkt_len -= dtl_entry_size as c_ulong;
            (*vpaq).buf_len += dtl_entry_size as c_ulong;
            return 0;
        }

        ret = powerpc_vpadtl_sample(record, vpa, vpaq_timestamp, (*vpaq).cpu);
        if ret != 0 {
            continue;
        }
    }
}

/*
 * For each of the PERF_RECORD_XX record, compare the timestamp
 * of perf record with timestamp of top element in the auxtrace heap.
 * Process the auxtrace queue if the timestamp of element from heap is
 * lower than timestamp from entry in perf record.
 */
unsafe fn powerpc_vpadtl_process_queues(vpa: *mut powerpc_vpadtl, timestamp: u64) -> c_int {
    loop {
        if (*vpa).heap.heap_cnt == 0 {
            return 0;
        }

        if (*(*vpa).heap.heap_array).ordinal >= timestamp {
            return 0;
        }

        let queue_nr = (*(*vpa).heap.heap_array).queue_nr;
        let queue = (*vpa).queues.queue_array.add(queue_nr as usize);
        let vpaq = (*queue).priv_ as *mut powerpc_vpadtl_queue;

        auxtrace_heap__pop(&mut (*vpa).heap);

        let mut ts = if (*vpa).heap.heap_cnt != 0 {
            let mut ts = (*(*vpa).heap.heap_array).ordinal + 1;
            if ts > timestamp {
                ts = timestamp;
            }
            ts
        } else {
            timestamp
        };

        let ret = powerpc_vpadtl_run_decoder(vpaq, &mut ts);
        if ret < 0 {
            auxtrace_heap__add(&mut (*vpa).heap, queue_nr, ts);
            return ret;
        }

        if ret == 0 {
            let add_ret = auxtrace_heap__add(&mut (*vpa).heap, queue_nr, ts);
            if add_ret < 0 {
                return add_ret;
            }
        } else {
            (*vpaq).on_heap = false;
        }
    }
}

unsafe fn powerpc_vpadtl__alloc_queue(
    vpa: *mut powerpc_vpadtl,
    queue_nr: c_uint,
) -> *mut powerpc_vpadtl_queue {
    let vpaq = zalloc(core::mem::size_of::<powerpc_vpadtl_queue>()) as *mut powerpc_vpadtl_queue;
    if vpaq.is_null() {
        return core::ptr::null_mut();
    }

    (*vpaq).vpa = vpa;
    (*vpaq).queue_nr = queue_nr;

    vpaq
}

/*
 * When the Dispatch Trace Log data is collected along with other events
 * like sched tracepoint events, it needs to be correlated and present
 * interleaved along with these events.
 */
unsafe fn powerpc_vpadtl__setup_queue(
    vpa: *mut powerpc_vpadtl,
    queue: *mut auxtrace_queue,
    queue_nr: c_uint,
) -> c_int {
    let mut vpaq = (*queue).priv_ as *mut powerpc_vpadtl_queue;

    if list_empty(&(*queue).head) || !vpaq.is_null() {
        return 0;
    }

    vpaq = powerpc_vpadtl__alloc_queue(vpa, queue_nr);
    if vpaq.is_null() {
        return -ENOMEM;
    }

    (*queue).priv_ = vpaq as *mut c_void;

    if (*queue).cpu != -1 {
        (*vpaq).cpu = (*queue).cpu;
    }

    if !(*vpaq).on_heap {
        loop {
            let ret = powerpc_vpadtl_decode(vpaq);
            if ret == 0 {
                return 0;
            }

            if ret >= 0 {
                (*vpaq).timestamp = powerpc_vpadtl_timestamp(vpaq);

                let add_ret = auxtrace_heap__add(&mut (*vpa).heap, queue_nr, (*vpaq).timestamp);
                if add_ret != 0 {
                    return add_ret;
                }
                (*vpaq).on_heap = true;
                break;
            }
        }
    }

    0
}

unsafe fn powerpc_vpadtl__setup_queues(vpa: *mut powerpc_vpadtl) -> c_int {
    let mut i: c_uint = 0;

    while i < (*vpa).queues.nr_queues {
        let ret = powerpc_vpadtl__setup_queue(vpa, (*vpa).queues.queue_array.add(i as usize), i);
        if ret != 0 {
            return ret;
        }
        i += 1;
    }

    0
}

unsafe fn powerpc_vpadtl__update_queues(vpa: *mut powerpc_vpadtl) -> c_int {
    if (*vpa).queues.new_data {
        (*vpa).queues.new_data = false;
        return powerpc_vpadtl__setup_queues(vpa);
    }

    0
}

unsafe extern "C" fn powerpc_vpadtl_process_event(
    session: *mut perf_session,
    _event: *mut perf_event,
    sample: *mut perf_sample,
    tool: *const perf_tool,
) -> c_int {
    let vpa = session_to_vpa(session);
    let mut err: c_int = 0;

    if dump_trace {
        return 0;
    }

    if !(*tool).ordered_events {
        pr_err(b"VPA requires ordered events\n\0".as_ptr() as *const c_char);
        return -EINVAL;
    }

    if (*sample).time != 0 {
        err = powerpc_vpadtl__update_queues(vpa);
        if err != 0 {
            return err;
        }

        err = powerpc_vpadtl_process_queues(vpa, (*sample).time);
    }

    err
}

/*
 * Process PERF_RECORD_AUXTRACE records
 */
unsafe extern "C" fn powerpc_vpadtl_process_auxtrace_event(
    session: *mut perf_session,
    event: *mut perf_event,
    _tool: *const perf_tool,
) -> c_int {
    let vpa = session_to_vpa(session);
    let mut buffer: *mut auxtrace_buffer = core::ptr::null_mut();
    let fd = perf_data__fd((*session).data);
    let data_offset: off_t;

    if !dump_trace {
        return 0;
    }

    if perf_data__is_pipe((*session).data) {
        data_offset = 0;
    } else {
        data_offset = lseek(fd, 0, SEEK_CUR);
        if data_offset == -1 {
            return -errno;
        }
    }

    let err = auxtrace_queues__add_event(
        &mut (*vpa).queues,
        session,
        event,
        data_offset,
        &mut buffer,
    );

    if err != 0 {
        return err;
    }

    /* Dump here now we have copied a piped trace out of the pipe */
    if !auxtrace_buffer__get_data(buffer, fd).is_null() {
        powerpc_vpadtl_dump_event(vpa, (*buffer).data as *mut u8, (*buffer).size);
        auxtrace_buffer__put_data(buffer);
    }

    0
}

unsafe extern "C" fn powerpc_vpadtl_flush(
    _session: *mut perf_session,
    _tool: *const perf_tool,
) -> c_int {
    0
}

unsafe extern "C" fn powerpc_vpadtl_free_events(session: *mut perf_session) {
    let vpa = session_to_vpa(session);
    let queues = &mut (*vpa).queues as *mut auxtrace_queues;

    let mut i: c_uint = 0;
    while i < (*queues).nr_queues {
        let priv_ptr = &mut (*(*queues).queue_array.add(i as usize)).priv_ as *mut *mut c_void;
        zfree(priv_ptr);
        i += 1;
    }

    auxtrace_queues__free(queues);
}

unsafe extern "C" fn powerpc_vpadtl_free(session: *mut perf_session) {
    let vpa = session_to_vpa(session);

    auxtrace_heap__free(&mut (*vpa).heap);
    powerpc_vpadtl_free_events(session);
    (*session).auxtrace = core::ptr::null_mut();
    free(vpa as *mut c_void);
}

static powerpc_vpadtl_info_fmts: [*const c_char; 1] = [
    b"  PMU Type           %lld\n\0".as_ptr() as *const c_char,
];

unsafe fn powerpc_vpadtl_print_info(arr: *mut __u64) {
    if !dump_trace {
        return;
    }

    fprintf(stdout, powerpc_vpadtl_info_fmts[POWERPC_VPADTL_TYPE], *arr.add(POWERPC_VPADTL_TYPE));
}

unsafe fn set_event_name(evlist: *mut evlist, id: u64, name: *const c_char) {
    let mut evsel = evlist__first(evlist);

    while !evsel.is_null() {
        if !(*evsel).core.id.is_null() && *(*evsel).core.id == id {
            if !(*evsel).name.is_null() {
                zfree(&mut (*evsel).name as *mut *mut c_char as *mut *mut c_void);
            }
            (*evsel).name = strdup(name);
            break;
        }
        evsel = evlist__next(evsel);
    }
}

unsafe fn powerpc_vpadtl_synth_events(
    vpa: *mut powerpc_vpadtl,
    session: *mut perf_session,
) -> c_int {
    let evlist = (*session).evlist;
    let mut evsel = evlist__first(evlist);
    let mut found = false;

    while !evsel.is_null() {
        if strstarts((*evsel).name, b"vpa_dtl\0".as_ptr() as *const c_char) {
            found = true;
            break;
        }
        evsel = evlist__next(evsel);
    }

    if !found {
        pr_debug(b"No selected events with VPA trace data\n\0".as_ptr() as *const c_char);
        return 0;
    }

    let mut attr: perf_event_attr = core::mem::zeroed();
    memset(
        &mut attr as *mut _ as *mut c_void,
        0,
        core::mem::size_of::<perf_event_attr>(),
    );
    attr.size = core::mem::size_of::<perf_event_attr>() as u32;
    attr.sample_type = (*evsel).core.attr.sample_type;
    attr.sample_id_all = (*evsel).core.attr.sample_id_all;
    attr.type_ = PERF_TYPE_SYNTH;
    attr.config = PERF_SYNTH_POWERPC_VPA_DTL;

    /* create new id val to be a fixed offset from evsel id */
    let id = auxtrace_synth_id_range_start(evsel);

    let err = perf_session__deliver_synth_attr_event(session, &mut attr, id);
    if err != 0 {
        return err;
    }

    (*vpa).sample_id = id;
    set_event_name(evlist, id, b"vpa-dtl\0".as_ptr() as *const c_char);

    0
}

/*
 * Process the PERF_RECORD_AUXTRACE_INFO records and setup
 * the infrastructure to process auxtrace events. PERF_RECORD_AUXTRACE_INFO
 * is processed first since it is of type perf_user_event_type.
 * Initialise the aux buffer queues using auxtrace_queues__init().
 * auxtrace_queue is created for each CPU.
 */
#[no_mangle]
pub unsafe extern "C" fn powerpc_vpadtl_process_auxtrace_info(
    event: *mut perf_event,
    session: *mut perf_session,
) -> c_int {
    let auxtrace_info = &mut *(*event).auxtrace_info;
    let min_sz = core::mem::size_of::<u64>() * POWERPC_VPADTL_TYPE;

    if (auxtrace_info.header.size as usize)
        < core::mem::size_of::<perf_record_auxtrace_info>() + min_sz
    {
        return -EINVAL;
    }

    let vpa = zalloc(core::mem::size_of::<powerpc_vpadtl>()) as *mut powerpc_vpadtl;
    if vpa.is_null() {
        return -ENOMEM;
    }

    let mut err = auxtrace_queues__init(&mut (*vpa).queues);
    if err != 0 {
        free(vpa as *mut c_void);
        return err;
    }

    (*vpa).session = session;
    (*vpa).machine = &mut (*session).machines.host;
    (*vpa).auxtrace_type = auxtrace_info.type_;
    (*vpa).pmu_type = auxtrace_info.priv_[POWERPC_VPADTL_TYPE] as u32;

    (*vpa).auxtrace.process_event = Some(powerpc_vpadtl_process_event);
    (*vpa).auxtrace.process_auxtrace_event = Some(powerpc_vpadtl_process_auxtrace_event);
    (*vpa).auxtrace.flush_events = Some(powerpc_vpadtl_flush);
    (*vpa).auxtrace.free_events = Some(powerpc_vpadtl_free_events);
    (*vpa).auxtrace.free = Some(powerpc_vpadtl_free);
    (*session).auxtrace = &mut (*vpa).auxtrace;

    powerpc_vpadtl_print_info(auxtrace_info.priv_.as_mut_ptr());

    if dump_trace {
        return 0;
    }

    err = powerpc_vpadtl_synth_events(vpa, session);
    if err != 0 {
        auxtrace_queues__free(&mut (*vpa).queues);
        (*session).auxtrace = core::ptr::null_mut();
        free(vpa as *mut c_void);
        return err;
    }

    err = auxtrace_queues__process_index(&mut (*vpa).queues, session);
    if err != 0 {
        auxtrace_queues__free(&mut (*vpa).queues);
        (*session).auxtrace = core::ptr::null_mut();
        free(vpa as *mut c_void);
        return err;
    }

    0
}
