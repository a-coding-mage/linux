// SPDX-License-Identifier: GPL-2.0
/*
 * Arm Statistical Profiling Extensions (SPE) support
 * Copyright (c) 2017-2018, Arm Ltd.
 *
 * Source-level Rust translation of perf/util/arm-spe.c.
 * C include dependencies are intentionally left as external declarations.
 */

#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(dead_code)]
#![allow(improper_ctypes)]

use core::ffi::{c_char, c_int, c_uint, c_ulong, c_void};
use core::mem::{size_of, zeroed};
use core::ptr::{null, null_mut};

type u8 = u8;
type u32 = u32;
type u64 = u64;
type __u64 = u64;
type size_t = usize;
type off_t = isize;
type pid_t = i32;
type bool_t = bool;

const MAX_TIMESTAMP: u64 = !0u64;

#[inline]
fn is_ldst_op(op: u64) -> bool {
    (op & ARM_SPE_OP_LDST) != 0
}

#[inline]
fn is_simd_op(op: u64) -> bool {
    (op & (ARM_SPE_OP_SIMD_FP | ARM_SPE_OP_SVE | ARM_SPE_OP_SME | ARM_SPE_OP_ASE)) != 0
}

#[inline]
fn is_mem_op(op: u64) -> bool {
    is_ldst_op(op) || is_simd_op(op)
}

#[inline]
fn ARM_SPE_CACHE_EVENT(access: u64, miss: u64) -> u64 {
    access | miss
}

#[inline]
fn arm_spe_is_cache_level(typ: u64, access: u64, miss: u64) -> bool {
    (typ & ARM_SPE_CACHE_EVENT(access, miss)) != 0
}

#[inline]
fn arm_spe_is_cache_hit(typ: u64, access: u64, miss: u64) -> bool {
    (typ & ARM_SPE_CACHE_EVENT(access, miss)) == access
}

#[inline]
fn arm_spe_is_cache_miss(typ: u64, miss: u64) -> bool {
    (typ & miss) != 0
}

#[repr(C)]
pub struct arm_spe {
    auxtrace: auxtrace,
    queues: auxtrace_queues,
    heap: auxtrace_heap,
    synth_opts: itrace_synth_opts,
    auxtrace_type: u32,
    session: *mut perf_session,
    machine: *mut machine,
    pmu_type: u32,
    tc: perf_tsc_conversion,
    timeless_decoding: u8,
    data_queued: u8,
    sample_type: u64,
    sample_flc: u8,
    sample_llc: u8,
    sample_tlb: u8,
    sample_branch: u8,
    sample_remote_access: u8,
    sample_memory: u8,
    sample_instructions: u8,
    l1d_miss_id: u64,
    l1d_access_id: u64,
    llc_miss_id: u64,
    llc_access_id: u64,
    tlb_miss_id: u64,
    tlb_access_id: u64,
    branch_id: u64,
    remote_access_id: u64,
    memory_id: u64,
    instructions_id: u64,
    kernel_start: u64,
    num_events: c_ulong,
    use_ctx_pkt_for_pid: u8,
    metadata: *mut *mut u64,
    metadata_ver: u64,
    metadata_nr_cpu: u64,
    is_homogeneous: bool,
}

#[repr(C)]
pub struct arm_spe_queue {
    spe: *mut arm_spe,
    queue_nr: c_uint,
    buffer: *mut auxtrace_buffer,
    old_buffer: *mut auxtrace_buffer,
    event_buf: *mut perf_event,
    on_heap: bool,
    done: bool,
    pid: pid_t,
    tid: pid_t,
    cpu: c_int,
    decoder: *mut arm_spe_decoder,
    time: u64,
    timestamp: u64,
    thread: *mut thread,
    sample_count: u64,
    flags: u32,
    last_branch: *mut branch_stack,
}

#[repr(C)]
pub struct data_source_handle {
    midr_ranges: *const midr_range,
    ds_synth: Option<unsafe extern "C" fn(*const arm_spe_record, *mut perf_mem_data_src)>,
}

unsafe extern "C" {
    fn arm_spe_get_packet(buf: *mut u8, len: size_t, packet: *mut arm_spe_pkt, midr: u64) -> c_int;
    fn arm_spe_pkt_desc(packet: *const arm_spe_pkt, desc: *mut c_char, len: size_t) -> c_int;
    fn color_fprintf(stream: *mut FILE, color: *const c_char, fmt: *const c_char, ...) -> c_int;
    fn printf(fmt: *const c_char, ...) -> c_int;
    fn fprintf(stream: *mut FILE, fmt: *const c_char, ...) -> c_int;
    fn pr_err(fmt: *const c_char, ...);
    fn pr_warning(fmt: *const c_char, ...);
    fn pr_warning_once(fmt: *const c_char, ...);
    fn pr_debug(fmt: *const c_char, ...);
    fn ui__warning(fmt: *const c_char, ...);
    fn ui__error(fmt: *const c_char, ...);
    fn zalloc(size: size_t) -> *mut c_void;
    fn calloc(nmemb: size_t, size: size_t) -> *mut c_void;
    fn malloc(size: size_t) -> *mut c_void;
    fn free(ptr: *mut c_void);
    fn memcpy(dest: *mut c_void, src: *const c_void, n: size_t) -> *mut c_void;
    fn memset(dest: *mut c_void, c: c_int, n: size_t) -> *mut c_void;
    fn strdup(s: *const c_char) -> *mut c_char;
    fn strtol(nptr: *const c_char, endptr: *mut *mut c_char, base: c_int) -> c_long;
    fn lseek(fd: c_int, offset: off_t, whence: c_int) -> off_t;
    fn perf_data__fd(data: *mut perf_data) -> c_int;
    fn perf_data__is_pipe(data: *mut perf_data) -> bool;
    fn auxtrace_buffer__next(queue: *mut auxtrace_queue, buffer: *mut auxtrace_buffer) -> *mut auxtrace_buffer;
    fn auxtrace_buffer__drop_data(buffer: *mut auxtrace_buffer);
    fn auxtrace_buffer__get_data(buffer: *mut auxtrace_buffer, fd: c_int) -> *mut c_void;
    fn auxtrace_buffer__put_data(buffer: *mut auxtrace_buffer);
    fn arm_spe_decoder_new(params: *mut arm_spe_params) -> *mut arm_spe_decoder;
    fn arm_spe_decoder_free(decoder: *mut arm_spe_decoder);
    fn arm_spe_decode(decoder: *mut arm_spe_decoder) -> c_int;
    fn machine__get_current_tid(machine: *mut machine, cpu: c_int) -> pid_t;
    fn machine__set_current_tid(machine: *mut machine, cpu: c_int, pid: pid_t, tid: pid_t) -> c_int;
    fn machine__find_thread(machine: *mut machine, pid: pid_t, tid: pid_t) -> *mut thread;
    fn machine__kernel_start(machine: *mut machine) -> u64;
    fn thread__zput(thread: *mut thread);
    fn thread__pid(thread: *mut thread) -> pid_t;
    fn thread__cpu(thread: *mut thread) -> c_int;
    fn tsc_to_perf_time(cyc: u64, tc: *mut perf_tsc_conversion) -> u64;
    fn perf_time_to_tsc(time: u64, tc: *mut perf_tsc_conversion) -> u64;
    fn perf_sample__init(sample: *mut perf_sample, all: bool);
    fn perf_sample__exit(sample: *mut perf_sample);
    fn perf_event__sample_event_size(sample: *mut perf_sample, typ: u64, read_format: u64, branch_sample_type: u64) -> size_t;
    fn perf_event__synthesize_sample(event: *mut perf_event, typ: u64, read_format: u64, branch_sample_type: u64, sample: *mut perf_sample) -> c_int;
    fn perf_session__deliver_synth_event(session: *mut perf_session, event: *mut perf_event, sample: *mut perf_sample) -> c_int;
    fn perf_session__deliver_synth_attr_event(session: *mut perf_session, attr: *mut perf_event_attr, id: u64) -> c_int;
    fn perf_session__env(session: *mut perf_session) -> *mut perf_env;
    fn perf_env__cpuid(env: *mut perf_env) -> *const c_char;
    fn evlist__id2evsel(evlist: *mut evlist, id: u64) -> *mut evsel;
    fn auxtrace_heap__add(heap: *mut auxtrace_heap, queue_nr: c_uint, ordinal: u64) -> c_int;
    fn auxtrace_heap__pop(heap: *mut auxtrace_heap);
    fn auxtrace_heap__free(heap: *mut auxtrace_heap);
    fn auxtrace_queues__init(queues: *mut auxtrace_queues) -> c_int;
    fn auxtrace_queues__free(queues: *mut auxtrace_queues);
    fn auxtrace_queues__add_event(queues: *mut auxtrace_queues, session: *mut perf_session, event: *mut perf_event, data_offset: off_t, buffer: *mut *mut auxtrace_buffer) -> c_int;
    fn auxtrace_queues__process_index(queues: *mut auxtrace_queues, session: *mut perf_session) -> c_int;
    fn auxtrace_synth_id_range_start(evsel: *mut evsel) -> u64;
    fn itrace_synth_opts__set_default(opts: *mut itrace_synth_opts, no_sample: bool);
    fn is_midr_in_range_list(midr: u64, ranges: *const midr_range) -> bool;
    static mut stdout: *mut FILE;
    static mut dump_trace: bool;
}

type c_long = i64;

const PERF_COLOR_BLUE: *const c_char = b"blue\0".as_ptr() as *const c_char;
const PERF_SAMPLE_MAX_SIZE: usize = 65536;
const ARM_SPE_PKT_DESC_MAX: usize = 256;
const ENOMEM: c_int = 12;
const EFAULT: c_int = 14;
const EINVAL: c_int = 22;
const SEEK_CUR: c_int = 1;

/* External constants from Linux perf and Arm SPE headers. */
extern "C" {
    static ARM_SPE_OP_LDST: u64;
    static ARM_SPE_OP_SIMD_FP: u64;
    static ARM_SPE_OP_SVE: u64;
    static ARM_SPE_OP_SME: u64;
    static ARM_SPE_OP_ASE: u64;
}

macro_rules! ext_consts {
    ($($name:ident),* $(,)?) => { $(const $name: u64 = 0;)* };
}

ext_consts!(
    ARM_SPE_OP_PRED, ARM_SPE_SVE_PARTIAL_PRED, ARM_SPE_SVE_EMPTY_PRED,
    SIMD_OP_FLAGS_ARCH_SVE, SIMD_OP_FLAGS_ARCH_SME, SIMD_OP_FLAGS_ARCH_ASE,
    SIMD_OP_FLAGS_PRED_DISABLED, SIMD_OP_FLAGS_PRED_PARTIAL,
    SIMD_OP_FLAGS_PRED_EMPTY, SIMD_OP_FLAGS_PRED_FULL,
    PERF_RECORD_MISC_KERNEL, PERF_RECORD_MISC_USER, PERF_RECORD_SAMPLE,
    PERF_IP_FLAG_BRANCH, PERF_IP_FLAG_BRANCH_MISS, PERF_IP_FLAG_NOT_TAKEN,
    PERF_IP_FLAG_IN_TX, PERF_IP_FLAG_CONDITIONAL, PERF_IP_FLAG_CALL,
    PERF_IP_FLAG_RETURN, ARM_SPE_OP_BRANCH_ERET, ARM_SPE_BRANCH_MISS,
    ARM_SPE_BRANCH_NOT_TAKEN, ARM_SPE_IN_TXN, ARM_SPE_OP_BR_COND,
    ARM_SPE_OP_BR_CR_BL, ARM_SPE_OP_BR_CR_RET, ARM_SPE_OP_BR_INDIRECT,
    ARM_SPE_OP_BR_CR_NON_BL_RET, PERF_BR_COND_CALL, PERF_BR_CALL,
    PERF_BR_COND_RET, PERF_BR_RET, PERF_BR_COND, PERF_BR_UNCOND,
    PERF_BR_UNKNOWN, ARM_SPE_OP_ST, ARM_SPE_OP_LD,
    PERF_MEM_LVL_NA, PERF_MEM_LVLNUM_NA, PERF_MEM_SNOOP_NA,
    PERF_MEM_LVL_L1, PERF_MEM_LVL_L2, PERF_MEM_LVL_L3, PERF_MEM_LVL_HIT,
    PERF_MEM_LVL_MISS, PERF_MEM_LVL_LFB, PERF_MEM_LVL_LOC_RAM,
    PERF_MEM_LVL_REM_CCE1, PERF_MEM_LVL_REM_CCE2, PERF_MEM_LVL_REM_RAM1,
    PERF_MEM_LVL_IO, PERF_MEM_LVLNUM_L1, PERF_MEM_LVLNUM_L2,
    PERF_MEM_LVLNUM_L3, PERF_MEM_LVLNUM_LFB, PERF_MEM_LVLNUM_RAM,
    PERF_MEM_LVLNUM_ANY_CACHE, PERF_MEM_LVLNUM_IO, PERF_MEM_SNOOP_NONE,
    PERF_MEM_SNOOP_HIT, PERF_MEM_SNOOP_HITM, PERF_MEM_SNOOPX_PEER,
    PERF_MEM_REMOTE_REMOTE, PERF_MEM_OP_LOAD, PERF_MEM_OP_STORE,
    PERF_MEM_OP_NA, PERF_MEM_TLB_WK, PERF_MEM_TLB_MISS, PERF_MEM_TLB_HIT,
    ARM_SPE_COMMON_DS_L1D, ARM_SPE_COMMON_DS_L2, ARM_SPE_COMMON_DS_PEER_CORE,
    ARM_SPE_COMMON_DS_LOCAL_CLUSTER, ARM_SPE_COMMON_DS_PEER_CLUSTER,
    ARM_SPE_COMMON_DS_SYS_CACHE, ARM_SPE_COMMON_DS_REMOTE, ARM_SPE_COMMON_DS_DRAM,
    ARM_SPE_AMPEREONE_LOCAL_CHIP_CACHE_OR_DEVICE, ARM_SPE_AMPEREONE_SLC,
    ARM_SPE_AMPEREONE_REMOTE_CHIP_CACHE, ARM_SPE_AMPEREONE_DDR,
    ARM_SPE_AMPEREONE_L1D, ARM_SPE_AMPEREONE_L2D,
    ARM_SPE_HISI_HIP_PEER_CPU, ARM_SPE_HISI_HIP_PEER_CPU_HITM,
    ARM_SPE_HISI_HIP_L3, ARM_SPE_HISI_HIP_L3_HITM,
    ARM_SPE_HISI_HIP_PEER_CLUSTER, ARM_SPE_HISI_HIP_PEER_CLUSTER_HITM,
    ARM_SPE_HISI_HIP_REMOTE_SOCKET, ARM_SPE_HISI_HIP_REMOTE_SOCKET_HITM,
    ARM_SPE_HISI_HIP_LOCAL_MEM, ARM_SPE_HISI_HIP_REMOTE_MEM,
    ARM_SPE_HISI_HIP_NC_DEV, ARM_SPE_HISI_HIP_L2, ARM_SPE_HISI_HIP_L2_HITM,
    ARM_SPE_HISI_HIP_L1, ARM_SPE_L1D_ACCESS, ARM_SPE_L1D_MISS,
    ARM_SPE_L2D_ACCESS, ARM_SPE_L2D_MISS, ARM_SPE_LLC_ACCESS, ARM_SPE_LLC_MISS,
    ARM_SPE_RECENTLY_FETCHED, ARM_SPE_DATA_SNOOPED, ARM_SPE_HITM,
    ARM_SPE_REMOTE_ACCESS, ARM_SPE_TLB_ACCESS, ARM_SPE_CAP_EVENT_FILTER,
    ARM_SPE_CPU, ARM_SPE_CPU_MIDR, ARM_SPE_HEADER_VERSION, ARM_SPE_HEADER_SIZE,
    ARM_SPE_CPUS_NUM, ARM_SPE_AUXTRACE_V1_PRIV_SIZE, ARM_SPE_PMU_TYPE,
    ARM_SPE_PMU_TYPE_V2, ARM_SPE_MAGIC, ARM_SPE_CPU_NR_PARAMS,
    ARM_SPE_CPU_PMU_TYPE, ARM_SPE_CAP_MIN_IVAL, ARM_SPE_AUXTRACE_V1_PRIV_MAX,
    PERF_SAMPLE_TIME, PERF_SAMPLE_MASK, PERF_SAMPLE_PHYS_ADDR, PERF_SAMPLE_IP,
    PERF_SAMPLE_TID, PERF_SAMPLE_PERIOD, PERF_SAMPLE_DATA_SRC, PERF_SAMPLE_WEIGHT,
    PERF_SAMPLE_ADDR, PERF_SAMPLE_BRANCH_STACK, PERF_SAMPLE_BRANCH_HW_INDEX,
    PERF_TYPE_HARDWARE, PERF_COUNT_HW_INSTRUCTIONS, PERF_ITRACE_PERIOD_INSTRUCTIONS,
    PERF_RECORD_EXIT, PERF_RECORD_SWITCH_CPU_WIDE, PERF_RECORD_SWITCH,
    PERF_RECORD_MISC_SWITCH_OUT
);

#[repr(C)] pub struct FILE { _private: [u8; 0] }
#[repr(C)] pub struct perf_data { _private: [u8; 0] }
#[repr(C)] pub struct perf_env { _private: [u8; 0] }
#[repr(C)] pub struct machine { pub host: machine_host }
#[repr(C)] pub struct machine_host { _private: [u8; 0] }
#[repr(C)] pub struct thread { _private: [u8; 0] }
#[repr(C)] pub struct midr_range { _private: [u8; 0] }
#[repr(C)] pub struct arm_spe_pkt { _private: [u8; 0] }

#[repr(C)] pub struct auxtrace {
    process_event: Option<unsafe extern "C" fn(*mut perf_session, *mut perf_event, *mut perf_sample, *const perf_tool) -> c_int>,
    process_auxtrace_event: Option<unsafe extern "C" fn(*mut perf_session, *mut perf_event, *const perf_tool) -> c_int>,
    flush_events: Option<unsafe extern "C" fn(*mut perf_session, *const perf_tool) -> c_int>,
    free_events: Option<unsafe extern "C" fn(*mut perf_session)>,
    free: Option<unsafe extern "C" fn(*mut perf_session)>,
    evsel_is_auxtrace: Option<unsafe extern "C" fn(*mut perf_session, *mut evsel) -> bool>,
}
#[repr(C)] pub struct auxtrace_queues {
    queue_array: *mut auxtrace_queue,
    nr_queues: c_uint,
    new_data: bool,
    populated: bool,
}
#[repr(C)] pub struct auxtrace_queue {
    head: list_head,
    priv_: *mut c_void,
    tid: pid_t,
    cpu: c_int,
}
#[repr(C)] pub struct list_head { next: *mut list_head, prev: *mut list_head }
#[repr(C)] pub struct auxtrace_heap { heap_array: *mut auxtrace_heap_item, heap_cnt: c_uint }
#[repr(C)] pub struct auxtrace_heap_item { queue_nr: c_uint, ordinal: u64 }
#[repr(C)] pub struct auxtrace_buffer {
    data: *mut c_void,
    size: size_t,
    cpu: auxtrace_buffer_cpu,
}
#[repr(C)] pub struct auxtrace_buffer_cpu { cpu: c_int }
#[repr(C)] pub struct arm_spe_buffer { len: size_t, buf: *mut c_void }
#[repr(C)] pub struct arm_spe_params {
    get_trace: Option<unsafe extern "C" fn(*mut arm_spe_buffer, *mut c_void) -> c_int>,
    data: *mut c_void,
}
#[repr(C)] pub struct arm_spe_decoder { record: arm_spe_record, midr: u64 }
#[repr(C)] pub struct arm_spe_record {
    op: u64, type_: u64, source: u32, timestamp: u64, from_ip: u64, to_ip: u64,
    virt_addr: u64, phys_addr: u64, latency: u32, prev_br_tgt: u64, context_id: u64,
}
#[repr(C)] pub struct simd_flags { arch: u64, pred: u64 }
#[repr(C)] pub union perf_mem_data_src { val: u64, bits: perf_mem_data_src_bits }
#[repr(C)] pub struct perf_mem_data_src_bits {
    mem_op: u64, mem_lvl: u64, mem_lvl_num: u64, mem_snoop: u64,
    mem_snoopx: u64, mem_remote: u64, mem_dtlb: u64,
}
#[repr(C)] pub struct branch_flags {
    value: u64, type_: u64, mispred: u64, predicted: u64, not_taken: u64, in_tx: u64, cycles: u64,
}
#[repr(C)] pub struct branch_entry { from: u64, to: u64, flags: branch_flags }
#[repr(C)] pub struct branch_stack { nr: u64, hw_idx: u64, entries: [branch_entry; 2] }
#[repr(C)] pub struct perf_event_header { type_: u32, misc: u16, size: u16 }
#[repr(C)] pub struct perf_sample_event { header: perf_event_header }
#[repr(C)] pub struct context_switch_event { next_prev_pid: pid_t, next_prev_tid: pid_t }
#[repr(C)] pub struct fork_event { tid: pid_t }
#[repr(C)] pub struct perf_record_auxtrace_info { header: perf_event_header, type_: u32, priv_: [u64; 0] }
#[repr(C)] pub union perf_event {
    header: perf_event_header,
    sample: perf_sample_event,
    context_switch: context_switch_event,
    fork: fork_event,
    auxtrace_info: perf_record_auxtrace_info,
}
#[repr(C)] pub struct perf_sample {
    time: u64, ip: u64, cpumode: u8, pid: pid_t, tid: pid_t, period: u64, cpu: c_int,
    simd_flags: simd_flags, id: u64, stream_id: u64, addr: u64, phys_addr: u64,
    data_src: u64, weight: u64, flags: u32, branch_stack: *mut branch_stack, evsel: *mut evsel,
}
#[repr(C)] pub struct perf_tsc_conversion {
    time_shift: u16, time_mult: u32, time_zero: u64, time_cycles: u64, time_mask: u64,
    cap_user_time_zero: u8, cap_user_time_short: u8,
}
#[repr(C)] pub struct perf_record_time_conv {
    time_shift: u16, time_mult: u32, time_zero: u64, time_cycles: u64, time_mask: u64,
    cap_user_time_zero: u8, cap_user_time_short: u8,
}
#[repr(C)] pub struct perf_session {
    auxtrace: *mut auxtrace, evlist: *mut evlist, data: *mut perf_data, machines: machines,
    time_conv: perf_record_time_conv, itrace_synth_opts: *mut itrace_synth_opts,
}
#[repr(C)] pub struct machines { host: machine }
#[repr(C)] pub struct perf_tool { ordered_events: bool }
#[repr(C)] pub struct evlist { _private: [u8; 0] }
#[repr(C)] pub struct evsel { core: evsel_core, name: *mut c_char }
#[repr(C)] pub struct evsel_core { attr: perf_event_attr, id: *mut u64 }
#[repr(C)] pub struct perf_event_attr {
    size: u32, type_: u32, config: u64, sample_type: u64, read_format: u64,
    sample_period: u64, branch_sample_type: u64, exclude_user: u32, exclude_kernel: u32,
    exclude_hv: u32, exclude_host: u32, exclude_guest: u32, sample_id_all: u32,
}
#[repr(C)] pub struct itrace_synth_opts {
    set: bool, inject: bool, last_branch: bool, last_branch_sz: c_uint, period: u64,
    flc: bool, llc: bool, tlb: bool, branches: bool, remote_access: bool, mem: bool,
    instructions: bool, period_type: u32,
}

#[inline] unsafe fn qpriv(queue: *mut auxtrace_queue) -> *mut arm_spe_queue {
    (*queue).priv_ as *mut arm_spe_queue
}

#[inline] unsafe fn list_empty(head: *mut list_head) -> bool {
    (*head).next == head
}

unsafe fn zfree<T>(p: *mut *mut T) {
    if !(*p).is_null() {
        free(*p as *mut c_void);
        *p = null_mut();
    }
}

unsafe extern "C" fn arm_spe_dump(_spe: *mut arm_spe, mut buf: *mut u8, mut len: size_t, midr: u64) {
    let mut packet: arm_spe_pkt = zeroed();
    let mut pos: size_t = 0;
    let mut desc = [0 as c_char; ARM_SPE_PKT_DESC_MAX];
    color_fprintf(stdout, PERF_COLOR_BLUE, b". ... ARM SPE data: size %#zx bytes\n\0".as_ptr() as *const c_char, len);
    while len != 0 {
        let mut ret = arm_spe_get_packet(buf, len, &mut packet, midr);
        let pkt_len: c_int = if ret > 0 { ret } else { 1 };
        printf(b".\0".as_ptr() as *const c_char);
        color_fprintf(stdout, PERF_COLOR_BLUE, b"  %08zx: \0".as_ptr() as *const c_char, pos);
        let mut i = 0;
        while i < pkt_len {
            color_fprintf(stdout, PERF_COLOR_BLUE, b" %02x\0".as_ptr() as *const c_char, *buf.add(i as usize) as c_int);
            i += 1;
        }
        while i < 16 {
            color_fprintf(stdout, PERF_COLOR_BLUE, b"   \0".as_ptr() as *const c_char);
            i += 1;
        }
        if ret > 0 {
            ret = arm_spe_pkt_desc(&packet, desc.as_mut_ptr(), ARM_SPE_PKT_DESC_MAX);
            if ret == 0 {
                color_fprintf(stdout, PERF_COLOR_BLUE, b" %s\n\0".as_ptr() as *const c_char, desc.as_ptr());
            }
        } else {
            color_fprintf(stdout, PERF_COLOR_BLUE, b" Bad packet!\n\0".as_ptr() as *const c_char);
        }
        pos += pkt_len as usize;
        buf = buf.add(pkt_len as usize);
        len -= pkt_len as usize;
    }
}

unsafe extern "C" fn arm_spe_dump_event(spe: *mut arm_spe, buf: *mut u8, len: size_t, midr: u64) {
    printf(b".\n\0".as_ptr() as *const c_char);
    arm_spe_dump(spe, buf, len, midr);
}

unsafe extern "C" fn arm_spe_get_trace(b: *mut arm_spe_buffer, data: *mut c_void) -> c_int {
    let speq = data as *mut arm_spe_queue;
    let mut buffer = (*speq).buffer;
    let old_buffer = (*speq).old_buffer;
    let queue = (*(*speq).spe).queues.queue_array.add((*speq).queue_nr as usize);
    buffer = auxtrace_buffer__next(queue, buffer);
    if buffer.is_null() {
        if !old_buffer.is_null() { auxtrace_buffer__drop_data(old_buffer); }
        (*b).len = 0;
        return 0;
    }
    (*speq).buffer = buffer;
    if (*buffer).data.is_null() {
        let fd = perf_data__fd((*(*(*speq).spe).session).data);
        (*buffer).data = auxtrace_buffer__get_data(buffer, fd);
        if (*buffer).data.is_null() { return -ENOMEM; }
    }
    (*b).len = (*buffer).size;
    (*b).buf = (*buffer).data;
    if (*b).len != 0 {
        if !old_buffer.is_null() { auxtrace_buffer__drop_data(old_buffer); }
        (*speq).old_buffer = buffer;
    } else {
        auxtrace_buffer__drop_data(buffer);
        return arm_spe_get_trace(b, data);
    }
    0
}

unsafe extern "C" fn arm_spe__alloc_queue(spe: *mut arm_spe, queue_nr: c_uint) -> *mut arm_spe_queue {
    let mut params: arm_spe_params = zeroed();
    let speq = zalloc(size_of::<arm_spe_queue>()) as *mut arm_spe_queue;
    if speq.is_null() { return null_mut(); }
    (*speq).event_buf = malloc(PERF_SAMPLE_MAX_SIZE) as *mut perf_event;
    if (*speq).event_buf.is_null() { goto_out_free_alloc_queue(speq); return null_mut(); }
    (*speq).spe = spe;
    (*speq).queue_nr = queue_nr;
    (*speq).pid = -1;
    (*speq).tid = -1;
    (*speq).cpu = -1;
    params.get_trace = Some(arm_spe_get_trace);
    params.data = speq as *mut c_void;
    if (*spe).synth_opts.last_branch {
        let sz = size_of::<branch_stack>() + size_of::<branch_entry>() * core::cmp::min((*spe).synth_opts.last_branch_sz, 2) as usize;
        (*speq).last_branch = zalloc(sz) as *mut branch_stack;
        if (*speq).last_branch.is_null() { goto_out_free_alloc_queue(speq); return null_mut(); }
    }
    (*speq).decoder = arm_spe_decoder_new(&mut params);
    if (*speq).decoder.is_null() { goto_out_free_alloc_queue(speq); return null_mut(); }
    speq
}

unsafe fn goto_out_free_alloc_queue(speq: *mut arm_spe_queue) {
    zfree(&mut (*speq).event_buf);
    zfree(&mut (*speq).last_branch);
    free(speq as *mut c_void);
}

#[inline] unsafe fn arm_spe_cpumode(spe: *mut arm_spe, ip: u64) -> u8 {
    if ip >= (*spe).kernel_start { PERF_RECORD_MISC_KERNEL as u8 } else { PERF_RECORD_MISC_USER as u8 }
}

unsafe extern "C" fn arm_spe_set_pid_tid_cpu(spe: *mut arm_spe, queue: *mut auxtrace_queue) {
    let speq = qpriv(queue);
    let tid = machine__get_current_tid((*spe).machine, (*speq).cpu);
    if tid != -1 {
        (*speq).tid = tid;
        thread__zput((*speq).thread);
    } else {
        (*speq).tid = (*queue).tid;
    }
    if (*speq).thread.is_null() && (*speq).tid != -1 {
        (*speq).thread = machine__find_thread((*spe).machine, -1, (*speq).tid);
    }
    if !(*speq).thread.is_null() {
        (*speq).pid = thread__pid((*speq).thread);
        if (*queue).cpu == -1 {
            (*speq).cpu = thread__cpu((*speq).thread);
            arm_spe__get_midr(spe, (*speq).cpu, &mut (*(*speq).decoder).midr);
        }
    }
}

unsafe extern "C" fn arm_spe_set_tid(speq: *mut arm_spe_queue, tid: pid_t) -> c_int {
    let spe = (*speq).spe;
    let err = machine__set_current_tid((*spe).machine, (*speq).cpu, -1, tid);
    if err != 0 { return err; }
    arm_spe_set_pid_tid_cpu(spe, (*spe).queues.queue_array.add((*speq).queue_nr as usize));
    0
}

unsafe extern "C" fn arm_spe__get_metadata_by_cpu(spe: *mut arm_spe, cpu: c_int) -> *mut u64 {
    if (*spe).metadata.is_null() { return null_mut(); }
    if cpu < 0 {
        if !(*spe).is_homogeneous { return null_mut(); }
        return *(*spe).metadata.add(0);
    }
    let mut i = 0u64;
    while i < (*spe).metadata_nr_cpu {
        let m = *(*spe).metadata.add(i as usize);
        if *m.add(ARM_SPE_CPU as usize) == cpu as u64 { return m; }
        i += 1;
    }
    null_mut()
}

unsafe extern "C" fn arm_spe__synth_simd_flags(record: *const arm_spe_record) -> simd_flags {
    let mut simd_flags: simd_flags = zeroed();
    if (*record).op & ARM_SPE_OP_SVE != 0 {
        simd_flags.arch |= SIMD_OP_FLAGS_ARCH_SVE;
    } else if (*record).op & ARM_SPE_OP_SME != 0 {
        simd_flags.arch |= SIMD_OP_FLAGS_ARCH_SME;
    } else if (*record).op & (ARM_SPE_OP_ASE | ARM_SPE_OP_SIMD_FP) != 0 {
        simd_flags.arch |= SIMD_OP_FLAGS_ARCH_ASE;
    }
    if (*record).op & ARM_SPE_OP_SVE != 0 {
        if (*record).op & ARM_SPE_OP_PRED == 0 { simd_flags.pred = SIMD_OP_FLAGS_PRED_DISABLED; }
        else if (*record).type_ & ARM_SPE_SVE_PARTIAL_PRED != 0 { simd_flags.pred = SIMD_OP_FLAGS_PRED_PARTIAL; }
        else if (*record).type_ & ARM_SPE_SVE_EMPTY_PRED != 0 { simd_flags.pred = SIMD_OP_FLAGS_PRED_EMPTY; }
        else { simd_flags.pred = SIMD_OP_FLAGS_PRED_FULL; }
    } else {
        if (*record).type_ & ARM_SPE_SVE_PARTIAL_PRED != 0 { simd_flags.pred = SIMD_OP_FLAGS_PRED_PARTIAL; }
        else if (*record).type_ & ARM_SPE_SVE_EMPTY_PRED != 0 { simd_flags.pred = SIMD_OP_FLAGS_PRED_EMPTY; }
    }
    simd_flags
}

unsafe extern "C" fn arm_spe_prep_sample(spe: *mut arm_spe, speq: *mut arm_spe_queue, event: *mut perf_event, sample: *mut perf_sample) {
    let record = &mut (*(*speq).decoder).record as *mut arm_spe_record;
    if (*spe).timeless_decoding == 0 {
        (*sample).time = tsc_to_perf_time((*record).timestamp, &mut (*spe).tc);
    }
    (*sample).ip = (*record).from_ip;
    (*sample).cpumode = arm_spe_cpumode(spe, (*sample).ip);
    (*sample).pid = (*speq).pid;
    (*sample).tid = (*speq).tid;
    (*sample).period = (*spe).synth_opts.period;
    (*sample).cpu = (*speq).cpu;
    (*sample).simd_flags = arm_spe__synth_simd_flags(record);
    (*event).sample.header.type_ = PERF_RECORD_SAMPLE as u32;
    (*event).sample.header.misc = (*sample).cpumode as u16;
    (*event).sample.header.size = size_of::<perf_event_header>() as u16;
}

unsafe extern "C" fn arm_spe__prep_branch_stack(speq: *mut arm_spe_queue) {
    let spe = (*speq).spe;
    let record = &mut (*(*speq).decoder).record as *mut arm_spe_record;
    let bstack = (*speq).last_branch;
    let last_branch_sz = (*spe).synth_opts.last_branch_sz;
    let have_tgt = ((*speq).flags as u64 & PERF_IP_FLAG_BRANCH) != 0;
    let have_pbt = last_branch_sz >= (have_tgt as c_uint + 1) && (*record).prev_br_tgt != 0;
    let sz = size_of::<branch_stack>() + size_of::<branch_entry>() * core::cmp::min(last_branch_sz, 2) as usize;
    let mut i = 0usize;
    memset(bstack as *mut c_void, 0, sz);
    if !have_tgt && !have_pbt { return; }
    if have_tgt {
        (*bstack).entries[i].from = (*record).from_ip;
        (*bstack).entries[i].to = (*record).to_ip;
        let bs_flags = &mut (*bstack).entries[i].flags;
        bs_flags.value = 0;
        if (*record).op & ARM_SPE_OP_BR_CR_BL != 0 {
            if (*record).op & ARM_SPE_OP_BR_COND != 0 { bs_flags.type_ |= PERF_BR_COND_CALL; } else { bs_flags.type_ |= PERF_BR_CALL; }
        } else if (*record).op & ARM_SPE_OP_BR_CR_RET != 0 || (*record).op & ARM_SPE_OP_BR_INDIRECT != 0 {
            if (*record).op & ARM_SPE_OP_BR_COND != 0 { bs_flags.type_ |= PERF_BR_COND_RET; } else { bs_flags.type_ |= PERF_BR_RET; }
        } else if (*record).op & ARM_SPE_OP_BR_CR_NON_BL_RET != 0 {
            if (*record).op & ARM_SPE_OP_BR_COND != 0 { bs_flags.type_ |= PERF_BR_COND; } else { bs_flags.type_ |= PERF_BR_UNCOND; }
        } else {
            if (*record).op & ARM_SPE_OP_BR_COND != 0 { bs_flags.type_ |= PERF_BR_COND; } else { bs_flags.type_ |= PERF_BR_UNKNOWN; }
        }
        if (*record).type_ & ARM_SPE_BRANCH_MISS != 0 {
            bs_flags.mispred = 1; bs_flags.predicted = 0;
        } else {
            bs_flags.mispred = 0; bs_flags.predicted = 1;
        }
        if (*record).type_ & ARM_SPE_BRANCH_NOT_TAKEN != 0 { bs_flags.not_taken = 1; }
        if (*record).type_ & ARM_SPE_IN_TXN != 0 { bs_flags.in_tx = 1; }
        bs_flags.cycles = core::cmp::min((*record).latency, 0xffff) as u64;
        i += 1;
    }
    if have_pbt {
        let bs_flags = &mut (*bstack).entries[i].flags;
        bs_flags.type_ |= PERF_BR_UNKNOWN;
        (*bstack).entries[i].to = (*record).prev_br_tgt;
        i += 1;
    }
    (*bstack).nr = i as u64;
    (*bstack).hw_idx = !0u64;
}

unsafe extern "C" fn arm_spe__inject_event(spe: *mut arm_spe, event: *mut perf_event, sample: *mut perf_sample, typ: u64) -> c_int {
    let mut evsel = (*sample).evsel;
    let mut branch_sample_type = 0u64;
    if evsel.is_null() && !(*spe).session.is_null() && !(*(*spe).session).evlist.is_null() {
        evsel = evlist__id2evsel((*(*spe).session).evlist, (*sample).id);
    }
    if !evsel.is_null() { branch_sample_type = (*evsel).core.attr.branch_sample_type; }
    (*event).header.type_ = PERF_RECORD_SAMPLE as u32;
    let sz = perf_event__sample_event_size(sample, typ, 0, branch_sample_type);
    if sz >= PERF_SAMPLE_MAX_SIZE {
        pr_err(b"Sample size %zu exceeds max size %d\n\0".as_ptr() as *const c_char, sz, PERF_SAMPLE_MAX_SIZE as c_int);
        return -EFAULT;
    }
    (*event).header.size = sz as u16;
    perf_event__synthesize_sample(event, typ, 0, branch_sample_type, sample)
}

unsafe extern "C" fn arm_spe_deliver_synth_event(spe: *mut arm_spe, _speq: *mut arm_spe_queue, event: *mut perf_event, sample: *mut perf_sample) -> c_int {
    let mut ret;
    if (*spe).synth_opts.inject {
        ret = arm_spe__inject_event(spe, event, sample, (*spe).sample_type);
        if ret != 0 { return ret; }
    }
    ret = perf_session__deliver_synth_event((*spe).session, event, sample);
    if ret != 0 { pr_err(b"ARM SPE: failed to deliver event, error %d\n\0".as_ptr() as *const c_char, ret); }
    ret
}

unsafe extern "C" fn arm_spe__synth_mem_sample(speq: *mut arm_spe_queue, spe_events_id: u64, data_src: perf_mem_data_src) -> c_int {
    let spe = (*speq).spe;
    let record = &mut (*(*speq).decoder).record as *mut arm_spe_record;
    let event = (*speq).event_buf;
    let mut sample: perf_sample = zeroed();
    perf_sample__init(&mut sample, true);
    arm_spe_prep_sample(spe, speq, event, &mut sample);
    sample.id = spe_events_id; sample.stream_id = spe_events_id; sample.addr = (*record).virt_addr;
    sample.phys_addr = (*record).phys_addr; sample.data_src = data_src.val; sample.weight = (*record).latency as u64;
    let ret = arm_spe_deliver_synth_event(spe, speq, event, &mut sample);
    perf_sample__exit(&mut sample);
    ret
}

unsafe extern "C" fn arm_spe__synth_branch_sample(speq: *mut arm_spe_queue, spe_events_id: u64) -> c_int {
    let spe = (*speq).spe;
    let record = &mut (*(*speq).decoder).record as *mut arm_spe_record;
    let event = (*speq).event_buf;
    let mut sample: perf_sample = zeroed();
    perf_sample__init(&mut sample, true);
    arm_spe_prep_sample(spe, speq, event, &mut sample);
    sample.id = spe_events_id; sample.stream_id = spe_events_id; sample.addr = (*record).to_ip;
    sample.weight = (*record).latency as u64; sample.flags = (*speq).flags; sample.branch_stack = (*speq).last_branch;
    let ret = arm_spe_deliver_synth_event(spe, speq, event, &mut sample);
    perf_sample__exit(&mut sample);
    ret
}

unsafe extern "C" fn arm_spe__synth_instruction_sample(speq: *mut arm_spe_queue, spe_events_id: u64, data_src: perf_mem_data_src) -> c_int {
    let spe = (*speq).spe;
    let record = &mut (*(*speq).decoder).record as *mut arm_spe_record;
    let event = (*speq).event_buf;
    let mut sample: perf_sample = zeroed();
    perf_sample__init(&mut sample, true);
    arm_spe_prep_sample(spe, speq, event, &mut sample);
    sample.id = spe_events_id; sample.stream_id = spe_events_id; sample.addr = (*record).to_ip;
    sample.phys_addr = (*record).phys_addr; sample.data_src = data_src.val; sample.weight = (*record).latency as u64;
    sample.flags = (*speq).flags; sample.branch_stack = (*speq).last_branch;
    let ret = arm_spe_deliver_synth_event(spe, speq, event, &mut sample);
    perf_sample__exit(&mut sample);
    ret
}

/* MIDR_ALL_VERSIONS(...) table entries depend on arch cputype.h macros. */
static common_ds_encoding_cpus: [midr_range; 1] = [midr_range { _private: [] }];
static ampereone_ds_encoding_cpus: [midr_range; 1] = [midr_range { _private: [] }];
static hisi_hip_ds_encoding_cpus: [midr_range; 1] = [midr_range { _private: [] }];

unsafe extern "C" fn arm_spe__sample_flags(speq: *mut arm_spe_queue) {
    let record = &(*(*speq).decoder).record as *const arm_spe_record;
    (*speq).flags = 0;
    if (*record).op & ARM_SPE_OP_BRANCH_ERET != 0 {
        (*speq).flags = PERF_IP_FLAG_BRANCH as u32;
        if (*record).type_ & ARM_SPE_BRANCH_MISS != 0 { (*speq).flags |= PERF_IP_FLAG_BRANCH_MISS as u32; }
        if (*record).type_ & ARM_SPE_BRANCH_NOT_TAKEN != 0 { (*speq).flags |= PERF_IP_FLAG_NOT_TAKEN as u32; }
        if (*record).type_ & ARM_SPE_IN_TXN != 0 { (*speq).flags |= PERF_IP_FLAG_IN_TX as u32; }
        if (*record).op & ARM_SPE_OP_BR_COND != 0 { (*speq).flags |= PERF_IP_FLAG_CONDITIONAL as u32; }
        if (*record).op & ARM_SPE_OP_BR_CR_BL != 0 { (*speq).flags |= PERF_IP_FLAG_CALL as u32; }
        else if (*record).op & ARM_SPE_OP_BR_CR_RET != 0 { (*speq).flags |= PERF_IP_FLAG_RETURN as u32; }
        else if (*record).op & ARM_SPE_OP_BR_INDIRECT != 0 { (*speq).flags |= PERF_IP_FLAG_RETURN as u32; }
    }
}

unsafe fn ds(data_src: *mut perf_mem_data_src) -> *mut perf_mem_data_src_bits {
    &mut (*data_src).bits
}

unsafe extern "C" fn arm_spe__synth_data_source_common(record: *const arm_spe_record, data_src: *mut perf_mem_data_src) {
    if (*record).op & ARM_SPE_OP_ST != 0 {
        (*ds(data_src)).mem_lvl = PERF_MEM_LVL_NA;
        (*ds(data_src)).mem_lvl_num = PERF_MEM_LVLNUM_NA;
        (*ds(data_src)).mem_snoop = PERF_MEM_SNOOP_NA;
        return;
    }
    match (*record).source as u64 {
        x if x == ARM_SPE_COMMON_DS_L1D => { (*ds(data_src)).mem_lvl = PERF_MEM_LVL_L1 | PERF_MEM_LVL_HIT; (*ds(data_src)).mem_lvl_num = PERF_MEM_LVLNUM_L1; (*ds(data_src)).mem_snoop = PERF_MEM_SNOOP_NONE; }
        x if x == ARM_SPE_COMMON_DS_L2 => { (*ds(data_src)).mem_lvl = PERF_MEM_LVL_L2 | PERF_MEM_LVL_HIT; (*ds(data_src)).mem_lvl_num = PERF_MEM_LVLNUM_L2; (*ds(data_src)).mem_snoop = PERF_MEM_SNOOP_NONE; }
        x if x == ARM_SPE_COMMON_DS_PEER_CORE => { (*ds(data_src)).mem_lvl = PERF_MEM_LVL_L2 | PERF_MEM_LVL_HIT; (*ds(data_src)).mem_lvl_num = PERF_MEM_LVLNUM_L2; (*ds(data_src)).mem_snoopx = PERF_MEM_SNOOPX_PEER; }
        x if x == ARM_SPE_COMMON_DS_LOCAL_CLUSTER || x == ARM_SPE_COMMON_DS_PEER_CLUSTER => { (*ds(data_src)).mem_lvl = PERF_MEM_LVL_L3 | PERF_MEM_LVL_HIT; (*ds(data_src)).mem_lvl_num = PERF_MEM_LVLNUM_L3; (*ds(data_src)).mem_snoopx = PERF_MEM_SNOOPX_PEER; }
        x if x == ARM_SPE_COMMON_DS_SYS_CACHE => { (*ds(data_src)).mem_lvl = PERF_MEM_LVL_L3 | PERF_MEM_LVL_HIT; (*ds(data_src)).mem_lvl_num = PERF_MEM_LVLNUM_L3; (*ds(data_src)).mem_snoop = PERF_MEM_SNOOP_HIT; }
        x if x == ARM_SPE_COMMON_DS_REMOTE => { (*ds(data_src)).mem_lvl = PERF_MEM_LVL_NA; (*ds(data_src)).mem_lvl_num = PERF_MEM_LVLNUM_NA; (*ds(data_src)).mem_remote = PERF_MEM_REMOTE_REMOTE; (*ds(data_src)).mem_snoopx = PERF_MEM_SNOOPX_PEER; }
        x if x == ARM_SPE_COMMON_DS_DRAM => { (*ds(data_src)).mem_lvl = PERF_MEM_LVL_LOC_RAM | PERF_MEM_LVL_HIT; (*ds(data_src)).mem_lvl_num = PERF_MEM_LVLNUM_RAM; (*ds(data_src)).mem_snoop = PERF_MEM_SNOOP_NONE; }
        _ => {}
    }
}

unsafe extern "C" fn arm_spe__synth_data_source_ampereone(record: *const arm_spe_record, data_src: *mut perf_mem_data_src) {
    let mut common_record: arm_spe_record = zeroed();
    match (*record).source as u64 {
        x if x == ARM_SPE_AMPEREONE_LOCAL_CHIP_CACHE_OR_DEVICE => common_record.source = ARM_SPE_COMMON_DS_PEER_CORE as u32,
        x if x == ARM_SPE_AMPEREONE_SLC => common_record.source = ARM_SPE_COMMON_DS_SYS_CACHE as u32,
        x if x == ARM_SPE_AMPEREONE_REMOTE_CHIP_CACHE => common_record.source = ARM_SPE_COMMON_DS_REMOTE as u32,
        x if x == ARM_SPE_AMPEREONE_DDR => common_record.source = ARM_SPE_COMMON_DS_DRAM as u32,
        x if x == ARM_SPE_AMPEREONE_L1D => common_record.source = ARM_SPE_COMMON_DS_L1D as u32,
        x if x == ARM_SPE_AMPEREONE_L2D => common_record.source = ARM_SPE_COMMON_DS_L2 as u32,
        _ => { pr_warning_once(b"AmpereOne: Unknown data source (0x%x)\n\0".as_ptr() as *const c_char, (*record).source); return; }
    }
    common_record.op = (*record).op;
    arm_spe__synth_data_source_common(&common_record, data_src);
}

unsafe extern "C" fn arm_spe__synth_data_source_hisi_hip(record: *const arm_spe_record, data_src: *mut perf_mem_data_src) {
    if (*record).op & ARM_SPE_OP_ST != 0 {
        arm_spe__synth_data_source_common(record, data_src);
        return;
    }
    match (*record).source as u64 {
        x if x == ARM_SPE_HISI_HIP_PEER_CPU => { (*ds(data_src)).mem_lvl = PERF_MEM_LVL_L2 | PERF_MEM_LVL_HIT; (*ds(data_src)).mem_lvl_num = PERF_MEM_LVLNUM_L2; (*ds(data_src)).mem_snoopx = PERF_MEM_SNOOPX_PEER; }
        x if x == ARM_SPE_HISI_HIP_PEER_CPU_HITM => { (*ds(data_src)).mem_lvl = PERF_MEM_LVL_L2 | PERF_MEM_LVL_HIT; (*ds(data_src)).mem_lvl_num = PERF_MEM_LVLNUM_L2; (*ds(data_src)).mem_snoop = PERF_MEM_SNOOP_HITM; (*ds(data_src)).mem_snoopx = PERF_MEM_SNOOPX_PEER; }
        x if x == ARM_SPE_HISI_HIP_L3 => { (*ds(data_src)).mem_lvl = PERF_MEM_LVL_L3 | PERF_MEM_LVL_HIT; (*ds(data_src)).mem_lvl_num = PERF_MEM_LVLNUM_L3; (*ds(data_src)).mem_snoop = PERF_MEM_SNOOP_HIT; }
        x if x == ARM_SPE_HISI_HIP_L3_HITM => { (*ds(data_src)).mem_lvl = PERF_MEM_LVL_L3 | PERF_MEM_LVL_HIT; (*ds(data_src)).mem_lvl_num = PERF_MEM_LVLNUM_L3; (*ds(data_src)).mem_snoop = PERF_MEM_SNOOP_HITM; }
        x if x == ARM_SPE_HISI_HIP_PEER_CLUSTER => { (*ds(data_src)).mem_lvl = PERF_MEM_LVL_REM_CCE1 | PERF_MEM_LVL_HIT; (*ds(data_src)).mem_lvl_num = PERF_MEM_LVLNUM_L3; (*ds(data_src)).mem_snoopx = PERF_MEM_SNOOPX_PEER; }
        x if x == ARM_SPE_HISI_HIP_PEER_CLUSTER_HITM => { (*ds(data_src)).mem_lvl = PERF_MEM_LVL_REM_CCE1 | PERF_MEM_LVL_HIT; (*ds(data_src)).mem_lvl_num = PERF_MEM_LVLNUM_L3; (*ds(data_src)).mem_snoop = PERF_MEM_SNOOP_HITM; (*ds(data_src)).mem_snoopx = PERF_MEM_SNOOPX_PEER; }
        x if x == ARM_SPE_HISI_HIP_REMOTE_SOCKET => { (*ds(data_src)).mem_lvl = PERF_MEM_LVL_REM_CCE2; (*ds(data_src)).mem_lvl_num = PERF_MEM_LVLNUM_ANY_CACHE; (*ds(data_src)).mem_remote = PERF_MEM_REMOTE_REMOTE; (*ds(data_src)).mem_snoopx = PERF_MEM_SNOOPX_PEER; }
        x if x == ARM_SPE_HISI_HIP_REMOTE_SOCKET_HITM => { (*ds(data_src)).mem_lvl = PERF_MEM_LVL_REM_CCE2; (*ds(data_src)).mem_lvl_num = PERF_MEM_LVLNUM_ANY_CACHE; (*ds(data_src)).mem_snoop = PERF_MEM_SNOOP_HITM; (*ds(data_src)).mem_remote = PERF_MEM_REMOTE_REMOTE; (*ds(data_src)).mem_snoopx = PERF_MEM_SNOOPX_PEER; }
        x if x == ARM_SPE_HISI_HIP_LOCAL_MEM => { (*ds(data_src)).mem_lvl = PERF_MEM_LVL_LOC_RAM | PERF_MEM_LVL_HIT; (*ds(data_src)).mem_lvl_num = PERF_MEM_LVLNUM_RAM; (*ds(data_src)).mem_snoop = PERF_MEM_SNOOP_NONE; }
        x if x == ARM_SPE_HISI_HIP_REMOTE_MEM => { (*ds(data_src)).mem_lvl = PERF_MEM_LVL_REM_RAM1 | PERF_MEM_LVL_HIT; (*ds(data_src)).mem_lvl_num = PERF_MEM_LVLNUM_RAM; (*ds(data_src)).mem_remote = PERF_MEM_REMOTE_REMOTE; }
        x if x == ARM_SPE_HISI_HIP_NC_DEV => { (*ds(data_src)).mem_lvl = PERF_MEM_LVL_IO | PERF_MEM_LVL_HIT; (*ds(data_src)).mem_lvl_num = PERF_MEM_LVLNUM_IO; (*ds(data_src)).mem_snoop = PERF_MEM_SNOOP_NONE; }
        x if x == ARM_SPE_HISI_HIP_L2 => { (*ds(data_src)).mem_lvl = PERF_MEM_LVL_L2 | PERF_MEM_LVL_HIT; (*ds(data_src)).mem_lvl_num = PERF_MEM_LVLNUM_L2; (*ds(data_src)).mem_snoop = PERF_MEM_SNOOP_NONE; }
        x if x == ARM_SPE_HISI_HIP_L2_HITM => { (*ds(data_src)).mem_lvl = PERF_MEM_LVL_L2 | PERF_MEM_LVL_HIT; (*ds(data_src)).mem_lvl_num = PERF_MEM_LVLNUM_L2; (*ds(data_src)).mem_snoop = PERF_MEM_SNOOP_HITM; }
        x if x == ARM_SPE_HISI_HIP_L1 => { (*ds(data_src)).mem_lvl = PERF_MEM_LVL_L1 | PERF_MEM_LVL_HIT; (*ds(data_src)).mem_lvl_num = PERF_MEM_LVLNUM_L1; (*ds(data_src)).mem_snoop = PERF_MEM_SNOOP_NONE; }
        _ => {}
    }
}

static data_source_handles: [data_source_handle; 3] = [
    data_source_handle { midr_ranges: common_ds_encoding_cpus.as_ptr(), ds_synth: Some(arm_spe__synth_data_source_common) },
    data_source_handle { midr_ranges: ampereone_ds_encoding_cpus.as_ptr(), ds_synth: Some(arm_spe__synth_data_source_ampereone) },
    data_source_handle { midr_ranges: hisi_hip_ds_encoding_cpus.as_ptr(), ds_synth: Some(arm_spe__synth_data_source_hisi_hip) },
];

unsafe extern "C" fn arm_spe__synth_ld_memory_level(record: *const arm_spe_record, data_src: *mut perf_mem_data_src) {
    if arm_spe_is_cache_hit((*record).type_, ARM_SPE_L1D_ACCESS, ARM_SPE_L1D_MISS) {
        (*ds(data_src)).mem_lvl = PERF_MEM_LVL_L1 | PERF_MEM_LVL_HIT; (*ds(data_src)).mem_lvl_num = PERF_MEM_LVLNUM_L1;
    } else if (*record).type_ & ARM_SPE_RECENTLY_FETCHED != 0 {
        (*ds(data_src)).mem_lvl = PERF_MEM_LVL_LFB | PERF_MEM_LVL_HIT; (*ds(data_src)).mem_lvl_num = PERF_MEM_LVLNUM_LFB;
    } else if arm_spe_is_cache_hit((*record).type_, ARM_SPE_L2D_ACCESS, ARM_SPE_L2D_MISS) {
        (*ds(data_src)).mem_lvl = PERF_MEM_LVL_L2 | PERF_MEM_LVL_HIT; (*ds(data_src)).mem_lvl_num = PERF_MEM_LVLNUM_L2;
    } else if arm_spe_is_cache_hit((*record).type_, ARM_SPE_LLC_ACCESS, ARM_SPE_LLC_MISS) {
        (*ds(data_src)).mem_lvl = PERF_MEM_LVL_L3 | PERF_MEM_LVL_HIT; (*ds(data_src)).mem_lvl_num = PERF_MEM_LVLNUM_L3;
    } else if arm_spe_is_cache_miss((*record).type_, ARM_SPE_LLC_MISS) {
        (*ds(data_src)).mem_lvl = PERF_MEM_LVL_L3 | PERF_MEM_LVL_MISS; (*ds(data_src)).mem_lvl_num = PERF_MEM_LVLNUM_L3;
    } else if arm_spe_is_cache_miss((*record).type_, ARM_SPE_L2D_MISS) {
        (*ds(data_src)).mem_lvl = PERF_MEM_LVL_L2 | PERF_MEM_LVL_MISS; (*ds(data_src)).mem_lvl_num = PERF_MEM_LVLNUM_L2;
    } else if arm_spe_is_cache_miss((*record).type_, ARM_SPE_L1D_MISS) {
        (*ds(data_src)).mem_lvl = PERF_MEM_LVL_L1 | PERF_MEM_LVL_MISS; (*ds(data_src)).mem_lvl_num = PERF_MEM_LVLNUM_L1;
    }
}

unsafe extern "C" fn arm_spe__synth_st_memory_level(record: *const arm_spe_record, data_src: *mut perf_mem_data_src) {
    if arm_spe_is_cache_level((*record).type_, ARM_SPE_LLC_ACCESS, ARM_SPE_LLC_MISS) {
        (*ds(data_src)).mem_lvl = PERF_MEM_LVL_L3;
        (*ds(data_src)).mem_lvl |= if arm_spe_is_cache_miss((*record).type_, ARM_SPE_LLC_MISS) { PERF_MEM_LVL_MISS } else { PERF_MEM_LVL_HIT };
        (*ds(data_src)).mem_lvl_num = PERF_MEM_LVLNUM_L3;
    } else if arm_spe_is_cache_level((*record).type_, ARM_SPE_L2D_ACCESS, ARM_SPE_L2D_MISS) {
        (*ds(data_src)).mem_lvl = PERF_MEM_LVL_L2;
        (*ds(data_src)).mem_lvl |= if arm_spe_is_cache_miss((*record).type_, ARM_SPE_L2D_MISS) { PERF_MEM_LVL_MISS } else { PERF_MEM_LVL_HIT };
        (*ds(data_src)).mem_lvl_num = PERF_MEM_LVLNUM_L2;
    } else if arm_spe_is_cache_level((*record).type_, ARM_SPE_L1D_ACCESS, ARM_SPE_L1D_MISS) {
        (*ds(data_src)).mem_lvl = PERF_MEM_LVL_L1;
        (*ds(data_src)).mem_lvl |= if arm_spe_is_cache_miss((*record).type_, ARM_SPE_L1D_MISS) { PERF_MEM_LVL_MISS } else { PERF_MEM_LVL_HIT };
        (*ds(data_src)).mem_lvl_num = PERF_MEM_LVLNUM_L1;
    }
}

unsafe extern "C" fn arm_spe__synth_memory_level(speq: *mut arm_spe_queue, record: *const arm_spe_record, data_src: *mut perf_mem_data_src) {
    let spe = (*speq).spe;
    if (*ds(data_src)).mem_lvl == 0 {
        if (*ds(data_src)).mem_op == PERF_MEM_OP_LOAD { arm_spe__synth_ld_memory_level(record, data_src); }
        if (*ds(data_src)).mem_op == PERF_MEM_OP_STORE { arm_spe__synth_st_memory_level(record, data_src); }
    }
    if (*ds(data_src)).mem_lvl == 0 {
        (*ds(data_src)).mem_lvl = PERF_MEM_LVL_NA; (*ds(data_src)).mem_lvl_num = PERF_MEM_LVLNUM_NA;
    }
    if (*ds(data_src)).mem_snoop == 0 {
        if (*record).type_ & ARM_SPE_DATA_SNOOPED != 0 {
            if (*record).type_ & ARM_SPE_HITM != 0 { (*ds(data_src)).mem_snoop = PERF_MEM_SNOOP_HITM; }
            else { (*ds(data_src)).mem_snoop = PERF_MEM_SNOOP_HIT; }
        } else {
            let metadata = arm_spe__get_metadata_by_cpu(spe, (*speq).cpu);
            if metadata.is_null() || (*metadata.add(ARM_SPE_CAP_EVENT_FILTER as usize) & ARM_SPE_DATA_SNOOPED) == 0 {
                (*ds(data_src)).mem_snoop = PERF_MEM_SNOOP_NA;
            } else {
                (*ds(data_src)).mem_snoop = PERF_MEM_SNOOP_NONE;
            }
        }
    }
    if (*ds(data_src)).mem_remote == 0 && ((*record).type_ & ARM_SPE_REMOTE_ACCESS != 0) {
        (*ds(data_src)).mem_remote = PERF_MEM_REMOTE_REMOTE;
    }
}

unsafe extern "C" fn arm_spe__get_midr(spe: *mut arm_spe, cpu: c_int, midr: *mut u64) -> c_int {
    if (*spe).metadata_ver == 1 {
        pr_warning_once(b"Old SPE metadata, re-record to improve decode accuracy\n\0".as_ptr() as *const c_char);
        let cpuid = perf_env__cpuid(perf_session__env((*spe).session));
        if cpuid.is_null() { pr_warning_once(b"Failed to get MIDR for CPU %d\n\0".as_ptr() as *const c_char, cpu); return -EINVAL; }
        *midr = strtol(cpuid, null_mut(), 16) as u64;
        return 0;
    }
    let metadata = arm_spe__get_metadata_by_cpu(spe, cpu);
    if metadata.is_null() {
        pr_warning_once(b"Failed to get MIDR for CPU %d\n\0".as_ptr() as *const c_char, cpu);
        return -EINVAL;
    }
    *midr = *metadata.add(ARM_SPE_CPU_MIDR as usize);
    0
}

unsafe extern "C" fn arm_spe__synth_ds(speq: *mut arm_spe_queue, record: *const arm_spe_record, data_src: *mut perf_mem_data_src) {
    let mut midr = 0u64;
    if arm_spe__get_midr((*speq).spe, (*speq).cpu, &mut midr) != 0 { return; }
    let mut i = 0usize;
    while i < data_source_handles.len() {
        if is_midr_in_range_list(midr, data_source_handles[i].midr_ranges) {
            if let Some(f) = data_source_handles[i].ds_synth { f(record, data_src); }
            return;
        }
        i += 1;
    }
}

unsafe extern "C" fn arm_spe__synth_data_source(speq: *mut arm_spe_queue, record: *const arm_spe_record) -> perf_mem_data_src {
    let mut data_src: perf_mem_data_src = zeroed();
    if !is_mem_op((*record).op) { return data_src; }
    if (*record).op & ARM_SPE_OP_LD != 0 { (*ds(&mut data_src)).mem_op = PERF_MEM_OP_LOAD; }
    else if (*record).op & ARM_SPE_OP_ST != 0 { (*ds(&mut data_src)).mem_op = PERF_MEM_OP_STORE; }
    else { (*ds(&mut data_src)).mem_op = PERF_MEM_OP_NA; }
    arm_spe__synth_ds(speq, record, &mut data_src);
    arm_spe__synth_memory_level(speq, record, &mut data_src);
    if (*record).type_ & (ARM_SPE_TLB_ACCESS | ARM_SPE_TLB_MISS) != 0 {
        (*ds(&mut data_src)).mem_dtlb = PERF_MEM_TLB_WK;
        if (*record).type_ & ARM_SPE_TLB_MISS != 0 { (*ds(&mut data_src)).mem_dtlb |= PERF_MEM_TLB_MISS; }
        else { (*ds(&mut data_src)).mem_dtlb |= PERF_MEM_TLB_HIT; }
    }
    data_src
}

unsafe extern "C" fn arm_spe_sample(speq: *mut arm_spe_queue) -> c_int {
    let record = &(*(*speq).decoder).record as *const arm_spe_record;
    let spe = (*speq).spe;
    (*speq).sample_count += 1;
    if (*speq).sample_count < (*spe).synth_opts.period { return 0; }
    (*speq).sample_count = 0;
    arm_spe__sample_flags(speq);
    let data_src = arm_spe__synth_data_source(speq, record);
    let mut err;
    if (*spe).sample_flc != 0 {
        if (*record).type_ & ARM_SPE_L1D_MISS != 0 { err = arm_spe__synth_mem_sample(speq, (*spe).l1d_miss_id, data_src); if err != 0 { return err; } }
        if (*record).type_ & ARM_SPE_L1D_ACCESS != 0 { err = arm_spe__synth_mem_sample(speq, (*spe).l1d_access_id, data_src); if err != 0 { return err; } }
    }
    if (*spe).sample_llc != 0 {
        if (*record).type_ & ARM_SPE_LLC_MISS != 0 { err = arm_spe__synth_mem_sample(speq, (*spe).llc_miss_id, data_src); if err != 0 { return err; } }
        if (*record).type_ & ARM_SPE_LLC_ACCESS != 0 { err = arm_spe__synth_mem_sample(speq, (*spe).llc_access_id, data_src); if err != 0 { return err; } }
    }
    if (*spe).sample_tlb != 0 {
        if (*record).type_ & ARM_SPE_TLB_MISS != 0 { err = arm_spe__synth_mem_sample(speq, (*spe).tlb_miss_id, data_src); if err != 0 { return err; } }
        if (*record).type_ & ARM_SPE_TLB_ACCESS != 0 { err = arm_spe__synth_mem_sample(speq, (*spe).tlb_access_id, data_src); if err != 0 { return err; } }
    }
    if (*spe).synth_opts.last_branch && ((*spe).sample_branch != 0 || (*spe).sample_instructions != 0) { arm_spe__prep_branch_stack(speq); }
    if (*spe).sample_branch != 0 && ((*record).op & ARM_SPE_OP_BRANCH_ERET != 0) { err = arm_spe__synth_branch_sample(speq, (*spe).branch_id); if err != 0 { return err; } }
    if (*spe).sample_remote_access != 0 && ((*record).type_ & ARM_SPE_REMOTE_ACCESS != 0) { err = arm_spe__synth_mem_sample(speq, (*spe).remote_access_id, data_src); if err != 0 { return err; } }
    if (*spe).sample_memory != 0 && is_mem_op((*record).op) { err = arm_spe__synth_mem_sample(speq, (*spe).memory_id, data_src); if err != 0 { return err; } }
    if (*spe).sample_instructions != 0 { err = arm_spe__synth_instruction_sample(speq, (*spe).instructions_id, data_src); if err != 0 { return err; } }
    0
}

unsafe extern "C" fn arm_spe_run_decoder(speq: *mut arm_spe_queue, timestamp: *mut u64) -> c_int {
    let spe = (*speq).spe;
    if (*spe).kernel_start == 0 { (*spe).kernel_start = machine__kernel_start((*spe).machine); }
    loop {
        let mut record = &mut (*(*speq).decoder).record as *mut arm_spe_record;
        if (*spe).timeless_decoding == 0 && (*record).context_id != !0u64 {
            let ret = arm_spe_set_tid(speq, (*record).context_id as pid_t);
            if ret != 0 { return ret; }
            (*spe).use_ctx_pkt_for_pid = 1;
        }
        let mut ret = arm_spe_sample(speq);
        if ret != 0 { return ret; }
        ret = arm_spe_decode((*speq).decoder);
        if ret == 0 { pr_debug(b"No data or all data has been processed.\n\0".as_ptr() as *const c_char); return 1; }
        if ret < 0 { continue; }
        record = &mut (*(*speq).decoder).record as *mut arm_spe_record;
        if (*record).timestamp > (*speq).timestamp { (*speq).timestamp = (*record).timestamp; }
        if (*spe).timeless_decoding == 0 && (*speq).timestamp >= *timestamp {
            *timestamp = (*speq).timestamp;
            return 0;
        }
    }
}

unsafe extern "C" fn arm_spe__setup_queue(spe: *mut arm_spe, queue: *mut auxtrace_queue, queue_nr: c_uint) -> c_int {
    let mut speq = qpriv(queue);
    if list_empty(&mut (*queue).head) || !speq.is_null() { return 0; }
    speq = arm_spe__alloc_queue(spe, queue_nr);
    if speq.is_null() { return -ENOMEM; }
    (*queue).priv_ = speq as *mut c_void;
    if (*queue).cpu != -1 { (*speq).cpu = (*queue).cpu; }
    arm_spe__get_midr(spe, (*queue).cpu, &mut (*(*speq).decoder).midr);
    if !(*speq).on_heap {
        if (*spe).timeless_decoding != 0 { return 0; }
        loop {
            let ret = arm_spe_decode((*speq).decoder);
            if ret == 0 { return 0; }
            if ret >= 0 {
                let record = &mut (*(*speq).decoder).record;
                (*speq).timestamp = record.timestamp;
                let err = auxtrace_heap__add(&mut (*spe).heap, queue_nr, (*speq).timestamp);
                if err != 0 { return err; }
                (*speq).on_heap = true;
                break;
            }
        }
    }
    0
}

unsafe extern "C" fn arm_spe__setup_queues(spe: *mut arm_spe) -> c_int {
    let mut i = 0;
    while i < (*spe).queues.nr_queues {
        let ret = arm_spe__setup_queue(spe, (*spe).queues.queue_array.add(i as usize), i);
        if ret != 0 { return ret; }
        i += 1;
    }
    0
}

unsafe extern "C" fn arm_spe__update_queues(spe: *mut arm_spe) -> c_int {
    if (*spe).queues.new_data {
        (*spe).queues.new_data = false;
        return arm_spe__setup_queues(spe);
    }
    0
}

unsafe extern "C" fn arm_spe__is_timeless_decoding(_spe: *mut arm_spe) -> bool {
    /* evlist__for_each_entry(evlist, evsel): clear timeless_decoding if any attr.sample_type has PERF_SAMPLE_TIME. */
    true
}

unsafe extern "C" fn arm_spe_process_queues(spe: *mut arm_spe, timestamp: u64) -> c_int {
    loop {
        if (*spe).heap.heap_cnt == 0 { return 0; }
        if (*(*spe).heap.heap_array).ordinal >= timestamp { return 0; }
        let queue_nr = (*(*spe).heap.heap_array).queue_nr;
        let queue = (*spe).queues.queue_array.add(queue_nr as usize);
        let speq = qpriv(queue);
        auxtrace_heap__pop(&mut (*spe).heap);
        let mut ts = if (*spe).heap.heap_cnt != 0 {
            let mut v = (*(*spe).heap.heap_array).ordinal + 1;
            if v > timestamp { v = timestamp; }
            v
        } else { timestamp };
        if (*spe).use_ctx_pkt_for_pid == 0 { arm_spe_set_pid_tid_cpu(spe, queue); }
        let ret = arm_spe_run_decoder(speq, &mut ts);
        if ret < 0 {
            auxtrace_heap__add(&mut (*spe).heap, queue_nr, ts);
            return ret;
        }
        if ret == 0 {
            let err = auxtrace_heap__add(&mut (*spe).heap, queue_nr, ts);
            if err < 0 { return err; }
        } else {
            (*speq).on_heap = false;
        }
    }
}

unsafe extern "C" fn arm_spe_process_timeless_queues(spe: *mut arm_spe, tid: pid_t, time_: u64) -> c_int {
    let queues = &mut (*spe).queues as *mut auxtrace_queues;
    let mut i = 0;
    let mut ts = 0u64;
    while i < (*queues).nr_queues {
        let queue = (*spe).queues.queue_array.add(i as usize);
        let speq = qpriv(queue);
        if !speq.is_null() && (tid == -1 || (*speq).tid == tid) {
            (*speq).time = time_;
            arm_spe_set_pid_tid_cpu(spe, queue);
            arm_spe_run_decoder(speq, &mut ts);
        }
        i += 1;
    }
    0
}

unsafe extern "C" fn arm_spe_context_switch(spe: *mut arm_spe, event: *mut perf_event, sample: *mut perf_sample) -> c_int {
    if ((*event).header.misc as u64 & PERF_RECORD_MISC_SWITCH_OUT) == 0 { return 0; }
    let pid = (*event).context_switch.next_prev_pid;
    let tid = (*event).context_switch.next_prev_tid;
    let cpu = (*sample).cpu;
    if tid == -1 { pr_warning(b"context_switch event has no tid\n\0".as_ptr() as *const c_char); }
    machine__set_current_tid((*spe).machine, cpu, pid, tid)
}

unsafe extern "C" fn arm_spe_process_event(session: *mut perf_session, event: *mut perf_event, sample: *mut perf_sample, tool: *const perf_tool) -> c_int {
    let mut err = 0;
    let spe = (*session).auxtrace as *mut arm_spe;
    if dump_trace { return 0; }
    if !(*tool).ordered_events {
        pr_err(b"SPE trace requires ordered events\n\0".as_ptr() as *const c_char);
        return -EINVAL;
    }
    let timestamp = if (*sample).time != 0 && (*sample).time != !0u64 { perf_time_to_tsc((*sample).time, &mut (*spe).tc) } else { 0 };
    if timestamp != 0 || (*spe).timeless_decoding != 0 {
        err = arm_spe__update_queues(spe);
        if err != 0 { return err; }
    }
    if (*spe).timeless_decoding != 0 {
        if (*event).header.type_ as u64 == PERF_RECORD_EXIT {
            err = arm_spe_process_timeless_queues(spe, (*event).fork.tid, (*sample).time);
        }
    } else if timestamp != 0 {
        err = arm_spe_process_queues(spe, timestamp);
        if err != 0 { return err; }
        if (*spe).use_ctx_pkt_for_pid == 0 &&
           ((*event).header.type_ as u64 == PERF_RECORD_SWITCH_CPU_WIDE || (*event).header.type_ as u64 == PERF_RECORD_SWITCH) {
            err = arm_spe_context_switch(spe, event, sample);
        }
    }
    err
}

unsafe extern "C" fn arm_spe_process_auxtrace_event(session: *mut perf_session, event: *mut perf_event, _tool: *const perf_tool) -> c_int {
    let spe = (*session).auxtrace as *mut arm_spe;
    if (*spe).data_queued == 0 {
        let mut buffer: *mut auxtrace_buffer = null_mut();
        let fd = perf_data__fd((*session).data);
        let data_offset: off_t = if perf_data__is_pipe((*session).data) { 0 } else {
            let off = lseek(fd, 0, SEEK_CUR);
            if off == -1 { return -1; }
            off
        };
        let err = auxtrace_queues__add_event(&mut (*spe).queues, session, event, data_offset, &mut buffer);
        if err != 0 { return err; }
        if dump_trace && !auxtrace_buffer__get_data(buffer, fd).is_null() {
            let mut midr = 0u64;
            arm_spe__get_midr(spe, (*buffer).cpu.cpu, &mut midr);
            arm_spe_dump_event(spe, (*buffer).data as *mut u8, (*buffer).size, midr);
            auxtrace_buffer__put_data(buffer);
        }
    }
    0
}

unsafe extern "C" fn arm_spe_flush(session: *mut perf_session, tool: *const perf_tool) -> c_int {
    let spe = (*session).auxtrace as *mut arm_spe;
    if dump_trace { return 0; }
    if !(*tool).ordered_events { return -EINVAL; }
    let ret = arm_spe__update_queues(spe);
    if ret < 0 { return ret; }
    if (*spe).timeless_decoding != 0 { return arm_spe_process_timeless_queues(spe, -1, MAX_TIMESTAMP - 1); }
    let ret2 = arm_spe_process_queues(spe, MAX_TIMESTAMP);
    if ret2 != 0 { return ret2; }
    if (*spe).use_ctx_pkt_for_pid == 0 {
        ui__warning(b"Arm SPE CONTEXT packets not found in the traces.\nMatching of TIDs to SPE events could be inaccurate.\n\0".as_ptr() as *const c_char);
    }
    0
}

unsafe extern "C" fn arm_spe__alloc_per_cpu_metadata(buf: *mut u64, per_cpu_size: c_int) -> *mut u64 {
    let metadata = zalloc(per_cpu_size as usize) as *mut u64;
    if metadata.is_null() { return null_mut(); }
    memcpy(metadata as *mut c_void, buf as *const c_void, per_cpu_size as usize);
    metadata
}

unsafe extern "C" fn arm_spe__free_metadata(metadata: *mut *mut u64, nr_cpu: c_int) {
    let mut i = 0;
    while i < nr_cpu {
        zfree(&mut *metadata.add(i as usize));
        i += 1;
    }
    free(metadata as *mut c_void);
}

unsafe extern "C" fn arm_spe__alloc_metadata(info: *mut perf_record_auxtrace_info, ver: *mut u64, nr_cpu: *mut c_int) -> *mut *mut u64 {
    let mut ptr = (*info).priv_.as_mut_ptr();
    let metadata_size = (*info).header.size as u64 - size_of::<perf_record_auxtrace_info>() as u64;
    if metadata_size == ARM_SPE_AUXTRACE_V1_PRIV_SIZE {
        *ver = 1; *nr_cpu = 0; return null_mut();
    }
    *ver = *ptr.add(ARM_SPE_HEADER_VERSION as usize);
    let hdr_sz = *ptr.add(ARM_SPE_HEADER_SIZE as usize) as c_int;
    *nr_cpu = *ptr.add(ARM_SPE_CPUS_NUM as usize) as c_int;
    if *nr_cpu <= 0 { return null_mut(); }
    let metadata = calloc(*nr_cpu as usize, size_of::<*mut u64>()) as *mut *mut u64;
    if metadata.is_null() { return null_mut(); }
    ptr = ptr.add(hdr_sz as usize);
    let per_cpu_sz = ((metadata_size - (hdr_sz as u64 * size_of::<u64>() as u64)) / *nr_cpu as u64) as c_int;
    let mut i = 0;
    while i < *nr_cpu {
        *metadata.add(i as usize) = arm_spe__alloc_per_cpu_metadata(ptr, per_cpu_sz);
        if (*metadata.add(i as usize)).is_null() {
            arm_spe__free_metadata(metadata, *nr_cpu);
            return null_mut();
        }
        ptr = ptr.add(per_cpu_sz as usize / size_of::<u64>());
        i += 1;
    }
    metadata
}

unsafe extern "C" fn arm_spe_free_queue(priv_: *mut c_void) {
    let speq = priv_ as *mut arm_spe_queue;
    if speq.is_null() { return; }
    thread__zput((*speq).thread);
    arm_spe_decoder_free((*speq).decoder);
    zfree(&mut (*speq).event_buf);
    zfree(&mut (*speq).last_branch);
    free(speq as *mut c_void);
}

unsafe extern "C" fn arm_spe_free_events(session: *mut perf_session) {
    let spe = (*session).auxtrace as *mut arm_spe;
    let queues = &mut (*spe).queues as *mut auxtrace_queues;
    let mut i = 0;
    while i < (*queues).nr_queues {
        arm_spe_free_queue((*(*queues).queue_array.add(i as usize)).priv_);
        (*(*queues).queue_array.add(i as usize)).priv_ = null_mut();
        i += 1;
    }
    auxtrace_queues__free(queues);
}

unsafe extern "C" fn arm_spe_free(session: *mut perf_session) {
    let spe = (*session).auxtrace as *mut arm_spe;
    auxtrace_heap__free(&mut (*spe).heap);
    arm_spe_free_events(session);
    (*session).auxtrace = null_mut();
    arm_spe__free_metadata((*spe).metadata, (*spe).metadata_nr_cpu as c_int);
    free(spe as *mut c_void);
}

unsafe extern "C" fn arm_spe_evsel_is_auxtrace(session: *mut perf_session, evsel: *mut evsel) -> bool {
    let spe = (*session).auxtrace as *mut arm_spe;
    (*evsel).core.attr.type_ == (*spe).pmu_type
}

static metadata_hdr_v1_fmts: [&[u8]; 2] = [
    b"  PMU Type           :%ld\n\0",
    b"  Per CPU mmaps      :%ld\n\0",
];
static metadata_hdr_fmts: [&[u8]; 4] = [
    b"  Header version     :%ld\n\0",
    b"  Header size        :%ld\n\0",
    b"  PMU type v2        :%ld\n\0",
    b"  CPU number         :%ld\n\0",
];
static metadata_per_cpu_fmts: [&[u8]; 7] = [
    b"    Magic            :0x%lx\n\0",
    b"    CPU #            :%ld\n\0",
    b"    Num of params    :%ld\n\0",
    b"    MIDR             :0x%lx\n\0",
    b"    PMU Type         :%ld\n\0",
    b"    Min Interval     :%ld\n\0",
    b"    Event Filter     :0x%lx\n\0",
];

unsafe extern "C" fn arm_spe_print_info(spe: *mut arm_spe, mut arr: *mut __u64) {
    if !dump_trace { return; }
    let (cpu_num, hdr_size, hdr_fmts): (c_uint, c_uint, *const &[u8]) = if (*spe).metadata_ver == 1 {
        (0, ARM_SPE_AUXTRACE_V1_PRIV_MAX as c_uint, metadata_hdr_v1_fmts.as_ptr())
    } else {
        (*arr.add(ARM_SPE_CPUS_NUM as usize) as c_uint, *arr.add(ARM_SPE_HEADER_SIZE as usize) as c_uint, metadata_hdr_fmts.as_ptr())
    };
    let mut i = 0;
    while i < hdr_size {
        let fmt = *hdr_fmts.add(i as usize);
        fprintf(stdout, fmt.as_ptr() as *const c_char, *arr.add(i as usize) as i64);
        i += 1;
    }
    arr = arr.add(hdr_size as usize);
    let mut cpu = 0;
    while cpu < cpu_num {
        let cpu_size = (ARM_SPE_CPU_NR_PARAMS + 1) + *arr.add(ARM_SPE_CPU_NR_PARAMS as usize);
        i = 0;
        while i < cpu_size as c_uint {
            let fmt = metadata_per_cpu_fmts[i as usize];
            fprintf(stdout, fmt.as_ptr() as *const c_char, *arr.add(i as usize) as i64);
            i += 1;
        }
        arr = arr.add(cpu_size as usize);
        cpu += 1;
    }
}

unsafe extern "C" fn arm_spe_set_event_name(_evlist: *mut evlist, _id: u64, _name: *const c_char) {
    /* evlist__for_each_entry(evlist, evsel): find matching evsel->core.id[0], zfree old name, strdup(name). */
}

unsafe extern "C" fn arm_spe_synth_events(spe: *mut arm_spe, session: *mut perf_session) -> c_int {
    let evlist = (*session).evlist;
    let evsel: *mut evsel = null_mut();
    /* evlist__for_each_entry searches for evsel->core.attr.type == spe->pmu_type. */
    let found = true;
    if !found {
        pr_debug(b"No selected events with SPE trace data\n\0".as_ptr() as *const c_char);
        return 0;
    }
    let mut attr: perf_event_attr = zeroed();
    attr.size = size_of::<perf_event_attr>() as u32;
    attr.type_ = PERF_TYPE_HARDWARE as u32;
    if !evsel.is_null() {
        attr.sample_type = (*evsel).core.attr.sample_type & (PERF_SAMPLE_MASK | PERF_SAMPLE_PHYS_ADDR);
        attr.exclude_user = (*evsel).core.attr.exclude_user;
        attr.exclude_kernel = (*evsel).core.attr.exclude_kernel;
        attr.exclude_hv = (*evsel).core.attr.exclude_hv;
        attr.exclude_host = (*evsel).core.attr.exclude_host;
        attr.exclude_guest = (*evsel).core.attr.exclude_guest;
        attr.sample_id_all = (*evsel).core.attr.sample_id_all;
        attr.read_format = (*evsel).core.attr.read_format;
    }
    attr.sample_type |= PERF_SAMPLE_IP | PERF_SAMPLE_TID | PERF_SAMPLE_PERIOD | PERF_SAMPLE_DATA_SRC | PERF_SAMPLE_WEIGHT | PERF_SAMPLE_ADDR;
    if (*spe).timeless_decoding != 0 { attr.sample_type &= !PERF_SAMPLE_TIME; } else { attr.sample_type |= PERF_SAMPLE_TIME; }
    (*spe).sample_type = attr.sample_type;
    attr.sample_period = (*spe).synth_opts.period;
    let mut id = auxtrace_synth_id_range_start(evsel);
    macro_rules! deliver {
        ($field:ident, $idfield:ident, $name:expr) => {{
            let err = perf_session__deliver_synth_attr_event(session, &mut attr, id);
            if err != 0 { return err; }
            (*spe).$idfield = id;
            arm_spe_set_event_name(evlist, id, $name.as_ptr() as *const c_char);
            id += 1;
        }};
    }
    if (*spe).synth_opts.flc {
        (*spe).sample_flc = 1;
        deliver!(sample_flc, l1d_miss_id, b"l1d-miss\0");
        deliver!(sample_flc, l1d_access_id, b"l1d-access\0");
    }
    if (*spe).synth_opts.llc {
        (*spe).sample_llc = 1;
        deliver!(sample_llc, llc_miss_id, b"llc-miss\0");
        deliver!(sample_llc, llc_access_id, b"llc-access\0");
    }
    if (*spe).synth_opts.tlb {
        (*spe).sample_tlb = 1;
        deliver!(sample_tlb, tlb_miss_id, b"tlb-miss\0");
        deliver!(sample_tlb, tlb_access_id, b"tlb-access\0");
    }
    if (*spe).synth_opts.last_branch {
        if (*spe).synth_opts.last_branch_sz > 2 {
            pr_debug(b"Arm SPE supports only two bstack entries (PBT+TGT).\n\0".as_ptr() as *const c_char);
        }
        attr.sample_type |= PERF_SAMPLE_BRANCH_STACK;
        attr.branch_sample_type |= PERF_SAMPLE_BRANCH_HW_INDEX;
    }
    if (*spe).synth_opts.branches {
        (*spe).sample_branch = 1;
        deliver!(sample_branch, branch_id, b"branch\0");
    }
    if (*spe).synth_opts.remote_access {
        (*spe).sample_remote_access = 1;
        deliver!(sample_remote_access, remote_access_id, b"remote-access\0");
    }
    if (*spe).synth_opts.mem {
        (*spe).sample_memory = 1;
        deliver!(sample_memory, memory_id, b"memory\0");
    }
    if (*spe).synth_opts.instructions {
        (*spe).sample_instructions = 1;
        attr.config = PERF_COUNT_HW_INSTRUCTIONS;
        let err = perf_session__deliver_synth_attr_event(session, &mut attr, id);
        if err != 0 { return err; }
        (*spe).instructions_id = id;
        arm_spe_set_event_name(evlist, id, b"instructions\0".as_ptr() as *const c_char);
    }
    0
}

unsafe extern "C" fn arm_spe__is_homogeneous(metadata: *mut *mut u64, nr_cpu: c_int) -> bool {
    if nr_cpu == 0 { return false; }
    let mut midr = 0u64;
    let mut i = 0;
    while i < nr_cpu {
        let m = *metadata.add(i as usize);
        if m.is_null() { return false; }
        if i == 0 { midr = *m.add(ARM_SPE_CPU_MIDR as usize); }
        else if midr != *m.add(ARM_SPE_CPU_MIDR as usize) { return false; }
        i += 1;
    }
    true
}

#[no_mangle]
pub unsafe extern "C" fn arm_spe_process_auxtrace_info(event: *mut perf_event, session: *mut perf_session) -> c_int {
    let auxtrace_info = &mut (*event).auxtrace_info as *mut perf_record_auxtrace_info;
    let min_sz = ARM_SPE_AUXTRACE_V1_PRIV_SIZE;
    let tc = &mut (*session).time_conv as *mut perf_record_time_conv;
    if (*auxtrace_info).header.size as usize < size_of::<perf_record_auxtrace_info>() + min_sz as usize {
        return -EINVAL;
    }
    let mut metadata_ver = 0u64;
    let mut nr_cpu = 0;
    let metadata = arm_spe__alloc_metadata(auxtrace_info, &mut metadata_ver, &mut nr_cpu);
    if metadata.is_null() && metadata_ver != 1 {
        pr_err(b"Failed to parse Arm SPE metadata.\n\0".as_ptr() as *const c_char);
        return -EINVAL;
    }
    let spe = zalloc(size_of::<arm_spe>()) as *mut arm_spe;
    if spe.is_null() {
        arm_spe__free_metadata(metadata, nr_cpu);
        return -ENOMEM;
    }
    let mut err = auxtrace_queues__init(&mut (*spe).queues);
    if err != 0 { free(spe as *mut c_void); arm_spe__free_metadata(metadata, nr_cpu); return err; }
    (*spe).session = session;
    (*spe).machine = &mut (*session).machines.host;
    (*spe).auxtrace_type = (*auxtrace_info).type_;
    if metadata_ver == 1 { (*spe).pmu_type = (*auxtrace_info).priv_.as_ptr().add(ARM_SPE_PMU_TYPE as usize).read() as u32; }
    else { (*spe).pmu_type = (*auxtrace_info).priv_.as_ptr().add(ARM_SPE_PMU_TYPE_V2 as usize).read() as u32; }
    (*spe).metadata = metadata;
    (*spe).metadata_ver = metadata_ver;
    (*spe).metadata_nr_cpu = nr_cpu as u64;
    (*spe).is_homogeneous = arm_spe__is_homogeneous(metadata, nr_cpu);
    (*spe).timeless_decoding = arm_spe__is_timeless_decoding(spe) as u8;
    (*spe).tc.time_shift = (*tc).time_shift;
    (*spe).tc.time_mult = (*tc).time_mult;
    (*spe).tc.time_zero = (*tc).time_zero;
    /* event_contains(*tc, cap_user_time_short) guards these assignments in C. */
    (*spe).tc.time_cycles = (*tc).time_cycles;
    (*spe).tc.time_mask = (*tc).time_mask;
    (*spe).tc.cap_user_time_zero = (*tc).cap_user_time_zero;
    (*spe).tc.cap_user_time_short = (*tc).cap_user_time_short;
    (*spe).auxtrace.process_event = Some(arm_spe_process_event);
    (*spe).auxtrace.process_auxtrace_event = Some(arm_spe_process_auxtrace_event);
    (*spe).auxtrace.flush_events = Some(arm_spe_flush);
    (*spe).auxtrace.free_events = Some(arm_spe_free_events);
    (*spe).auxtrace.free = Some(arm_spe_free);
    (*spe).auxtrace.evsel_is_auxtrace = Some(arm_spe_evsel_is_auxtrace);
    (*session).auxtrace = &mut (*spe).auxtrace;
    arm_spe_print_info(spe, (*auxtrace_info).priv_.as_mut_ptr());
    if dump_trace { return 0; }
    if !(*session).itrace_synth_opts.is_null() && (*(*session).itrace_synth_opts).set {
        (*spe).synth_opts = *(*session).itrace_synth_opts;
    } else {
        itrace_synth_opts__set_default(&mut (*spe).synth_opts, false);
        (*spe).synth_opts.period_type = PERF_ITRACE_PERIOD_INSTRUCTIONS as u32;
        (*spe).synth_opts.period = 1;
    }
    if (*spe).synth_opts.period_type != PERF_ITRACE_PERIOD_INSTRUCTIONS as u32 {
        ui__error(b"You must only use i (instructions) --itrace period with Arm SPE. e.g --itrace=i1i\n\0".as_ptr() as *const c_char);
        err = -EINVAL;
        auxtrace_queues__free(&mut (*spe).queues);
        (*session).auxtrace = null_mut();
        free(spe as *mut c_void);
        arm_spe__free_metadata(metadata, nr_cpu);
        return err;
    }
    if (*spe).synth_opts.period > 1 {
        ui__warning(b"Arm SPE has a hardware-based sampling period.\n\n--itrace periods > 1i downsample by an interval of n SPE samples rather than n instructions.\n\0".as_ptr() as *const c_char);
    }
    err = arm_spe_synth_events(spe, session);
    if err != 0 {
        auxtrace_queues__free(&mut (*spe).queues);
        (*session).auxtrace = null_mut();
        free(spe as *mut c_void);
        arm_spe__free_metadata(metadata, nr_cpu);
        return err;
    }
    err = auxtrace_queues__process_index(&mut (*spe).queues, session);
    if err != 0 {
        auxtrace_queues__free(&mut (*spe).queues);
        (*session).auxtrace = null_mut();
        free(spe as *mut c_void);
        arm_spe__free_metadata(metadata, nr_cpu);
        return err;
    }
    if (*spe).queues.populated { (*spe).data_queued = 1; }
    0
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
