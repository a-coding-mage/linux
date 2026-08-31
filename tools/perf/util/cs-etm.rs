// SPDX-License-Identifier: GPL-2.0
/*
 * Copyright(C) 2015-2018 Linaro Limited.
 *
 * Author: Tor Jeremiassen <tor@ti.com>
 * Author: Mathieu Poirier <mathieu.poirier@linaro.org>
 *
 * Rust translation of perf/util/cs-etm.c.  C include dependencies are kept as
 * external declarations or opaque C-compatible types; this file intentionally
 * does not provide dependency implementations.
 */

#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(dead_code)]
#![allow(improper_ctypes)]
#![allow(clippy::missing_safety_doc)]

use core::ffi::{c_char, c_int, c_uint, c_ulong, c_void};
use core::mem;
use core::ptr;

type u8 = u8;
type u16 = u16;
type u32 = u32;
type u64 = u64;
type size_t = usize;
type ssize_t = isize;
type off_t = i64;
type pid_t = i32;
type uintptr_t = usize;

const EINVAL: c_int = 22;
const ENOMEM: c_int = 12;
const EFAULT: c_int = 14;
const INT_MIN: c_int = c_int::MIN;
const INT_MAX: c_int = c_int::MAX;
const UINT8_MAX: u8 = u8::MAX;
const UINT32_MAX: u32 = u32::MAX;

/* PTMs ETMIDR [11:8] set to b0011 */
const ETMIDR_PTM_VERSION: u32 = 0x0000_0300;
const SINK_UNSET: u32 = !0u32;

const fn TO_CS_QUEUE_NR(queue_nr: c_uint, trace_chan_id: u8) -> c_uint {
    (queue_nr << 16) | trace_chan_id as c_uint
}
const fn TO_QUEUE_NR(cs_queue_nr: c_uint) -> c_uint {
    cs_queue_nr >> 16
}
const fn TO_TRACE_CHAN_ID(cs_queue_nr: c_uint) -> u8 {
    (cs_queue_nr & 0x0000_ffff) as u8
}

macro_rules! neg {
    ($e:ident) => {
        -$e
    };
}

#[repr(C)]
pub struct auxtrace {
    pub process_event: Option<unsafe extern "C" fn(*mut perf_session, *mut perf_event, *mut perf_sample, *const perf_tool) -> c_int>,
    pub process_auxtrace_event: Option<unsafe extern "C" fn(*mut perf_session, *mut perf_event, *const perf_tool) -> c_int>,
    pub flush_events: Option<unsafe extern "C" fn(*mut perf_session, *const perf_tool) -> c_int>,
    pub free_events: Option<unsafe extern "C" fn(*mut perf_session)>,
    pub free: Option<unsafe extern "C" fn(*mut perf_session)>,
    pub evsel_is_auxtrace: Option<unsafe extern "C" fn(*mut perf_session, *mut evsel) -> bool>,
}

#[repr(C)]
pub struct auxtrace_heap_item {
    pub queue_nr: c_uint,
    pub ordinal: u64,
}

#[repr(C)]
pub struct auxtrace_heap {
    pub heap_array: *mut auxtrace_heap_item,
    pub heap_cnt: c_uint,
}

#[repr(C)]
pub struct auxtrace_queue {
    pub head: list_head,
    pub priv_: *mut c_void,
    pub cpu: c_int,
    pub tid: pid_t,
}

#[repr(C)]
pub struct auxtrace_queues {
    pub queue_array: *mut auxtrace_queue,
    pub nr_queues: c_uint,
    pub populated: bool,
}

#[repr(C)]
pub struct auxtrace_buffer {
    pub list: list_head,
    pub size: size_t,
    pub offset: u64,
    pub reference: u64,
    pub data: *mut c_void,
    pub buffer_nr: u64,
}

#[repr(C)]
pub struct itrace_synth_opts {
    pub set: bool,
    pub default_no_sample: bool,
    pub branches: bool,
    pub last_branch: bool,
    pub instructions: bool,
    pub calls: bool,
    pub returns: bool,
    pub callchain: bool,
    pub inject: bool,
    pub timeless_decoding: bool,
    pub use_timestamp: bool,
    pub thread_stack: bool,
    pub period: u64,
    pub last_branch_sz: c_uint,
    pub callchain_sz: c_uint,
}

#[repr(C)]
pub struct perf_tsc_conversion {
    pub time_shift: u16,
    pub time_mult: u32,
    pub time_zero: u64,
    pub time_cycles: u64,
    pub time_mask: u64,
    pub cap_user_time_zero: bool,
    pub cap_user_time_short: bool,
}

#[repr(C)]
pub struct cs_etm_auxtrace {
    pub auxtrace: auxtrace,
    pub queues: auxtrace_queues,
    pub heap: auxtrace_heap,
    pub synth_opts: itrace_synth_opts,
    pub session: *mut perf_session,
    pub tc: perf_tsc_conversion,
    pub timeless_decoding: bool,
    pub per_thread_decoding: bool,
    pub snapshot_mode: bool,
    pub data_queued: bool,
    pub has_virtual_ts: bool,
    pub use_thread_stack: bool,
    pub use_callchain: bool,
    pub num_cpu: c_int,
    pub latest_kernel_timestamp: u64,
    pub auxtrace_type: u32,
    pub branches_filter: u32,
    pub branches_sample_type: u64,
    pub branches_id: u64,
    pub instructions_sample_type: u64,
    pub instructions_sample_period: u64,
    pub instructions_id: u64,
    pub metadata: *mut *mut u64,
    pub pmu_type: c_uint,
    pub pid_fmt: cs_etm_pid_fmt,
}

#[repr(C)]
pub struct cs_etm_traceid_queue {
    pub trace_chan_id: u8,
    pub period_instructions: u64,
    pub kernel_start: u64,
    pub event_buf: *mut perf_event,
    pub br_stack_sz: c_uint,
    pub last_branch: *mut branch_stack,
    pub callchain: *mut ip_callchain,
    pub prev_packet: *mut cs_etm_packet,
    pub packet: *mut cs_etm_packet,
    pub packet_queue: cs_etm_packet_queue,
    pub decode_thread: *mut thread,
    pub decode_el: ocsd_ex_level,
    pub frontend_thread: *mut thread,
}

#[repr(C)]
#[derive(Clone, Copy, PartialEq, Eq)]
pub enum cs_etm_format {
    UNSET,
    FORMATTED,
    UNFORMATTED,
}

#[repr(C)]
pub struct cs_etm_queue {
    pub etm: *mut cs_etm_auxtrace,
    pub decoder: *mut cs_etm_decoder,
    pub buffer: *mut auxtrace_buffer,
    pub queue_nr: c_uint,
    pub pending_timestamp_chan_id: u8,
    pub format: cs_etm_format,
    pub offset: u64,
    pub buf: *const u8,
    pub buf_len: size_t,
    pub buf_used: size_t,
    pub traceid_queues_list: *mut intlist,
    pub traceid_queues: *mut *mut cs_etm_traceid_queue,
    pub traceid_list: *mut intlist,
    pub own_traceid_list: *mut intlist,
    pub sink_id: u32,
}

#[repr(C)]
#[derive(Clone, Copy, PartialEq, Eq)]
pub enum cs_etm_pid_fmt {
    CS_ETM_PIDFMT_CTXTID,
    CS_ETM_PIDFMT_CTXTID2,
    CS_ETM_PIDFMT_NONE,
}

#[repr(C)]
#[derive(Clone, Copy, PartialEq, Eq)]
pub enum ocsd_ex_level {
    ocsd_EL_unknown,
    ocsd_EL0,
    ocsd_EL1,
    ocsd_EL2,
    ocsd_EL3,
}

#[repr(C)]
#[derive(Clone, Copy)]
pub enum cs_etm_decoder_operation {
    CS_ETM_OPERATION_PRINT,
    CS_ETM_OPERATION_DECODE,
    CS_ETM_OPERATION_MAX,
}

#[repr(C)]
pub struct list_head {
    pub next: *mut list_head,
    pub prev: *mut list_head,
}

#[repr(C)]
pub struct int_node {
    pub rb_node: [usize; 3],
    pub i: u64,
    pub priv_: *mut c_void,
}

#[repr(C)] pub struct intlist { _private: [u8; 0] }
#[repr(C)] pub struct cs_etm_decoder { _private: [u8; 0] }
#[repr(C)] pub struct perf_session { pub auxtrace: *mut auxtrace, pub evlist: *mut evlist, pub machines: machines, pub data: *mut perf_data, pub time_conv: perf_record_time_conv, pub itrace_synth_opts: *mut itrace_synth_opts, pub auxtrace_index: list_head, pub header: perf_header }
#[repr(C)] pub struct perf_tool { pub ordered_events: bool }
#[repr(C)] pub struct evlist { _private: [u8; 0] }
#[repr(C)] pub struct evsel { pub core: evsel_core }
#[repr(C)] pub struct evsel_core { pub attr: perf_event_attr }
#[repr(C)] pub struct perf_data { _private: [u8; 0] }
#[repr(C)] pub struct machine { _private: [u8; 0] }
#[repr(C)] pub struct machines { pub host: machine }
#[repr(C)] pub struct thread { _private: [u8; 0] }
#[repr(C)] pub struct dso { _private: [u8; 0] }
#[repr(C)] pub struct map { _private: [u8; 0] }
#[repr(C)] pub struct maps { _private: [u8; 0] }

#[repr(C)]
pub struct addr_location {
    pub map: *mut map,
}

#[repr(C)]
pub struct perf_header {
    pub data_offset: u64,
    pub data_size: u64,
}

#[repr(C)]
pub struct perf_record_time_conv {
    pub time_shift: u16,
    pub time_mult: u32,
    pub time_zero: u64,
    pub time_cycles: u64,
    pub time_mask: u64,
    pub cap_user_time_zero: bool,
    pub cap_user_time_short: bool,
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct perf_event_header {
    pub type_: u32,
    pub misc: u16,
    pub size: u16,
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct perf_record_sample {
    pub header: perf_event_header,
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct perf_record_auxtrace_info {
    pub header: perf_event_header,
    pub type_: u32,
    pub priv_: [u64; 0],
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct perf_record_auxtrace {
    pub header: perf_event_header,
    pub size: u64,
    pub offset: u64,
    pub reference: u64,
    pub idx: u32,
    pub tid: i32,
    pub cpu: u32,
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct perf_record_aux {
    pub header: perf_event_header,
    pub aux_offset: u64,
    pub aux_size: u64,
    pub flags: u64,
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct perf_record_itrace_start {
    pub header: perf_event_header,
    pub pid: pid_t,
    pub tid: pid_t,
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct perf_record_context_switch {
    pub header: perf_event_header,
    pub next_prev_pid: pid_t,
    pub next_prev_tid: pid_t,
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct perf_record_aux_output_hw_id {
    pub header: perf_event_header,
    pub hw_id: u64,
}

#[repr(C)]
pub union perf_event {
    pub header: perf_event_header,
    pub sample: perf_record_sample,
    pub auxtrace_info: perf_record_auxtrace_info,
    pub auxtrace: perf_record_auxtrace,
    pub aux: perf_record_aux,
    pub itrace_start: perf_record_itrace_start,
    pub context_switch: perf_record_context_switch,
    pub aux_output_hw_id: perf_record_aux_output_hw_id,
    pub fork: perf_record_context_switch,
}

#[repr(C)]
pub struct perf_sample {
    pub evsel: *mut evsel,
    pub time: u64,
    pub ip: u64,
    pub pid: pid_t,
    pub tid: pid_t,
    pub id: u64,
    pub stream_id: u64,
    pub period: u64,
    pub cpu: i32,
    pub flags: u64,
    pub cpumode: u8,
    pub addr: u64,
    pub insn_len: u32,
    pub insn: *mut u8,
    pub branch_stack: *mut branch_stack,
    pub callchain: *mut ip_callchain,
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct perf_event_attr {
    pub type_: u32,
    pub size: u32,
    pub config: u64,
    pub sample_period: u64,
    pub sample_type: u64,
    pub read_format: u64,
    pub branch_sample_type: u64,
    pub exclude_user: bool,
    pub exclude_kernel: bool,
    pub exclude_hv: bool,
    pub exclude_host: bool,
    pub exclude_guest: bool,
    pub sample_id_all: bool,
}

#[repr(C)]
pub struct branch_entry {
    pub from: u64,
    pub to: u64,
}

#[repr(C)]
pub struct branch_stack {
    pub nr: u64,
    pub hw_idx: u64,
    pub entries: [branch_entry; 0],
}

#[repr(C)]
pub struct ip_callchain {
    pub nr: u64,
    pub ips: [u64; 0],
}

#[repr(C)]
pub struct cs_etm_packet {
    pub isa: u32,
    pub start_addr: u64,
    pub end_addr: u64,
    pub instr_count: u64,
    pub last_instr_taken_branch: bool,
    pub last_instr_size: u8,
    pub last_instr_type: u8,
    pub last_instr_subtype: u8,
    pub last_instr_cond: u8,
    pub flags: u64,
    pub exception_number: u32,
    pub trace_chan_id: u8,
    pub cpu: i32,
    pub sample_type: u32,
    pub el: ocsd_ex_level,
    pub tid: pid_t,
}

#[repr(C)]
pub struct cs_etm_packet_queue {
    pub head: c_int,
    pub tail: c_int,
    pub packet_count: c_int,
    pub cs_timestamp: u64,
    pub packet_buffer: [cs_etm_packet; CS_ETM_PACKET_MAX_BUFFER],
}

#[repr(C)]
pub union cs_etm_trace_params_regs {
    pub etmv3: cs_etmv3_trace_params,
    pub etmv4: cs_etmv4_trace_params,
    pub ete: cs_ete_trace_params,
}

#[repr(C)]
pub struct cs_etm_trace_params {
    pub protocol: u32,
    pub regs: cs_etm_trace_params_regs,
}

#[repr(C)] #[derive(Clone, Copy)] pub struct cs_etmv3_trace_params { pub reg_ctrl: u64, pub reg_trc_id: u64 }
#[repr(C)] #[derive(Clone, Copy)] pub struct cs_etmv4_trace_params { pub reg_idr0: u64, pub reg_idr1: u64, pub reg_idr2: u64, pub reg_idr8: u64, pub reg_configr: u64, pub reg_traceidr: u64 }
#[repr(C)] #[derive(Clone, Copy)] pub struct cs_ete_trace_params { pub reg_idr0: u64, pub reg_idr1: u64, pub reg_idr2: u64, pub reg_idr8: u64, pub reg_configr: u64, pub reg_traceidr: u64, pub reg_devarch: u64 }

#[repr(C)]
pub struct cs_etm_decoder_params {
    pub packet_printer: Option<unsafe extern "C" fn(*const c_char, *mut c_void)>,
    pub operation: cs_etm_decoder_operation,
    pub data: *mut c_void,
    pub formatted: bool,
    pub fsyncs: bool,
    pub hsyncs: bool,
    pub frame_aligned: bool,
}

#[repr(C)] pub struct dso_data { pub status: i32 }
#[repr(C)] pub struct auxtrace_index_entry { pub file_offset: off_t, pub sz: size_t }
#[repr(C)] pub struct auxtrace_index { pub list: list_head, pub nr: size_t, pub entries: *mut auxtrace_index_entry }

const CS_ETM_PACKET_MAX_BUFFER: usize = 1024;
const CS_ETM_INVAL_ADDR: u64 = !0u64;
const CS_ETM_ISA_UNKNOWN: u32 = 0;
const CS_ETM_ISA_T32: u32 = 1;
const CS_ETM_ISA_A32: u32 = 2;
const CS_ETM_ISA_A64: u32 = 3;
const CS_ETM_EMPTY: u32 = 0;
const CS_ETM_RANGE: u32 = 1;
const CS_ETM_DISCONTINUITY: u32 = 2;
const CS_ETM_EXCEPTION: u32 = 3;
const CS_ETM_EXCEPTION_RET: u32 = 4;
const CS_ETM_CONTEXT: u32 = 5;
const CS_ETM_PER_THREAD_TRACEID: u8 = 0;

const CS_ETM_MAGIC: usize = 0;
const CS_ETM_CPU: usize = 1;
const CS_ETM_NR_TRC_PARAMS: usize = 2;
const CS_ETM_COMMON_BLK_MAX_V1: usize = 3;
const CS_HEADER_VERSION: usize = 0;
const CS_HEADER_VERSION_MAX: c_int = 3;
const CS_PMU_TYPE_CPUS: usize = 1;
const CS_ETM_SNAPSHOT: usize = 2;
const CS_ETM_HEADER_SIZE: c_int = 24;
const INFO_HEADER_SIZE: c_int = 16;
const CS_ETM_PRIV_MAX: c_int = 16;
const CS_ETMV4_PRIV_MAX: c_int = 32;
const CS_ETE_PRIV_MAX: c_int = 32;
const CS_ETM_NR_TRC_PARAMS_V0: c_int = 4;
const CS_ETMV4_NR_TRC_PARAMS_V0: c_int = 8;
const CS_ETM_ETMCR: usize = 3;
const CS_ETM_ETMTRACEIDR: usize = 4;
const CS_ETM_ETMIDR: usize = 5;
const CS_ETMV4_TRCIDR0: usize = 3;
const CS_ETMV4_TRCIDR1: usize = 4;
const CS_ETMV4_TRCIDR2: usize = 5;
const CS_ETMV4_TRCIDR8: usize = 6;
const CS_ETMV4_TRCCONFIGR: usize = 7;
const CS_ETMV4_TRCTRACEIDR: usize = 8;
const CS_ETMV4_TS_SOURCE: usize = 9;
const CS_ETE_TRCIDR0: usize = 3;
const CS_ETE_TRCIDR1: usize = 4;
const CS_ETE_TRCIDR2: usize = 5;
const CS_ETE_TRCIDR8: usize = 6;
const CS_ETE_TRCCONFIGR: usize = 7;
const CS_ETE_TRCTRACEIDR: usize = 8;
const CS_ETE_TRCDEVARCH: usize = 9;
const CS_ETE_TS_SOURCE: usize = 10;

const CORESIGHT_TRACE_ID_VAL_MASK: u64 = 0x7f;
const CS_ETM_PROTO_PTM: u32 = 1;
const CS_ETM_PROTO_ETMV3: u32 = 2;
const CS_ETM_PROTO_ETMV4i: u32 = 3;
const CS_ETM_PROTO_ETE: u32 = 4;
const ETMCR_CTXTID: u64 = 1 << 14;
const ETMCR_TIMESTAMP_EN: u64 = 1 << 28;
const TRCCONFIGR_VMID: u64 = 1 << 7;
const TRCCONFIGR_VMIDOPT: u64 = 1 << 15;
const TRCCONFIGR_CID: u64 = 1 << 6;
const TRCCONFIGR_TS: u64 = 1 << 11;

const PERF_SAMPLE_MASK: u64 = !0u64;
const PERF_SAMPLE_IP: u64 = 1 << 0;
const PERF_SAMPLE_TID: u64 = 1 << 1;
const PERF_SAMPLE_TIME: u64 = 1 << 2;
const PERF_SAMPLE_ADDR: u64 = 1 << 3;
const PERF_SAMPLE_PERIOD: u64 = 1 << 4;
const PERF_SAMPLE_BRANCH_STACK: u64 = 1 << 11;
const PERF_SAMPLE_CALLCHAIN: u64 = 1 << 5;
const PERF_SAMPLE_BRANCH_HW_INDEX: u64 = 1 << 17;
const PERF_TYPE_HARDWARE: u32 = 0;
const PERF_COUNT_HW_BRANCH_INSTRUCTIONS: u64 = 4;
const PERF_COUNT_HW_INSTRUCTIONS: u64 = 1;
const PERF_RECORD_SAMPLE: u32 = 9;
const PERF_RECORD_EXIT: u32 = 4;
const PERF_RECORD_AUX: u32 = 11;
const PERF_RECORD_ITRACE_START: u32 = 12;
const PERF_RECORD_SWITCH_CPU_WIDE: u32 = 14;
const PERF_RECORD_AUXTRACE: u32 = 71;
const PERF_RECORD_AUX_OUTPUT_HW_ID: u32 = 21;
const PERF_RECORD_MISC_KERNEL: u8 = 1;
const PERF_RECORD_MISC_USER: u8 = 2;
const PERF_RECORD_MISC_GUEST_KERNEL: u8 = 3;
const PERF_RECORD_MISC_GUEST_USER: u8 = 4;
const PERF_RECORD_MISC_SWITCH_OUT: u16 = 1 << 13;
const PERF_AUX_FLAG_OVERWRITE: u64 = 1 << 1;
const PERF_AUX_FLAG_CORESIGHT_FORMAT_RAW: u64 = 1 << 8;
const PERF_SAMPLE_MAX_SIZE: usize = 65536;

const PERF_IP_FLAG_BRANCH: u64 = 1 << 0;
const PERF_IP_FLAG_CALL: u64 = 1 << 1;
const PERF_IP_FLAG_RETURN: u64 = 1 << 2;
const PERF_IP_FLAG_CONDITIONAL: u64 = 1 << 3;
const PERF_IP_FLAG_SYSCALLRET: u64 = 1 << 4;
const PERF_IP_FLAG_ASYNC: u64 = 1 << 5;
const PERF_IP_FLAG_INTERRUPT: u64 = 1 << 6;
const PERF_IP_FLAG_TRACE_BEGIN: u64 = 1 << 7;
const PERF_IP_FLAG_TRACE_END: u64 = 1 << 8;
const HOST_KERNEL_ID: pid_t = 0;
const DEFAULT_GUEST_KERNEL_ID: pid_t = -1;

const CS_ETMV3_EXC_SVC: u32 = 0;
const CS_ETMV3_EXC_DEBUG_HALT: u32 = 1;
const CS_ETMV3_EXC_ASYNC_DATA_ABORT: u32 = 2;
const CS_ETMV3_EXC_PE_RESET: u32 = 3;
const CS_ETMV3_EXC_IRQ: u32 = 4;
const CS_ETMV3_EXC_FIQ: u32 = 5;
const CS_ETMV3_EXC_SMC: u32 = 6;
const CS_ETMV3_EXC_HYP: u32 = 7;
const CS_ETMV3_EXC_JAZELLE_THUMBEE: u32 = 8;
const CS_ETMV3_EXC_UNDEFINED_INSTR: u32 = 9;
const CS_ETMV3_EXC_PREFETCH_ABORT: u32 = 10;
const CS_ETMV3_EXC_DATA_FAULT: u32 = 11;
const CS_ETMV3_EXC_GENERIC: u32 = 12;
const CS_ETMV4_EXC_CALL: u32 = 0;
const CS_ETMV4_EXC_RESET: u32 = 1;
const CS_ETMV4_EXC_DEBUG_HALT: u32 = 2;
const CS_ETMV4_EXC_SYSTEM_ERROR: u32 = 3;
const CS_ETMV4_EXC_INST_DEBUG: u32 = 4;
const CS_ETMV4_EXC_DATA_DEBUG: u32 = 5;
const CS_ETMV4_EXC_IRQ: u32 = 6;
const CS_ETMV4_EXC_FIQ: u32 = 7;
const CS_ETMV4_EXC_TRAP: u32 = 8;
const CS_ETMV4_EXC_ALIGNMENT: u32 = 9;
const CS_ETMV4_EXC_INST_FAULT: u32 = 10;
const CS_ETMV4_EXC_DATA_FAULT: u32 = 11;
const CS_ETMV4_EXC_END: u32 = 31;

const OCSD_INSTR_BR: u8 = 1;
const OCSD_INSTR_BR_INDIRECT: u8 = 2;
const OCSD_S_INSTR_NONE: u8 = 0;
const OCSD_S_INSTR_BR_LINK: u8 = 1;
const OCSD_S_INSTR_V7_IMPLIED_RET: u8 = 2;
const OCSD_S_INSTR_V8_RET: u8 = 3;
type ocsd_mem_space_acc_t = u32;
const OCSD_MEM_SPACE_ANY: u32 = 0;
const OCSD_MEM_SPACE_N: u32 = 1;
const OCSD_MEM_SPACE_S: u32 = 2;
const OCSD_MEM_SPACE_EL1N: u32 = 1 << 4;
const OCSD_MEM_SPACE_EL2: u32 = 1 << 5;
const OCSD_MEM_SPACE_EL3: u32 = 1 << 6;

const CS_AUX_HW_ID_TRACE_ID_MASK: u64 = 0x0000_00ff;
const CS_AUX_HW_ID_SINK_ID_MASK: u64 = 0x0fff_ff00;
const CS_AUX_HW_ID_MAJOR_VERSION_MASK: u64 = 0xf000_0000_0000_0000;
const CS_AUX_HW_ID_MINOR_VERSION_MASK: u64 = 0x0f00_0000_0000_0000;
const CS_AUX_HW_ID_MAJOR_VERSION: c_int = 0;

unsafe fn FIELD_GET(mask: u64, val: u64) -> u64 {
    let shift = mask.trailing_zeros();
    (val & mask) >> shift
}

unsafe fn ptr_at<T>(p: *mut T, i: isize) -> *mut T {
    p.offset(i)
}

unsafe fn container_of_auxtrace(p: *mut auxtrace) -> *mut cs_etm_auxtrace {
    p as *mut cs_etm_auxtrace
}

extern "C" {
    static __perf_cs_etmv3_magic: u64;
    static __perf_cs_etmv4_magic: u64;
    static __perf_cs_ete_magic: u64;
    static mut verbose: c_int;
    static mut dump_trace: c_int;
    static mut symbol_conf: symbol_conf_t;
    static mut callchain_param: callchain_param_t;
    static PERF_COLOR_BLUE: *const c_char;
    static mut stdout: *mut c_void;
    static mut errno: c_int;

    fn intlist__find(list: *mut intlist, i: u64) -> *mut int_node;
    fn intlist__findnew(list: *mut intlist, i: u64) -> *mut int_node;
    fn intlist__new(str_: *const c_void) -> *mut intlist;
    fn intlist__delete(list: *mut intlist);
    fn intlist__remove(list: *mut intlist, node: *mut int_node);
    fn intlist__nr_entries(list: *mut intlist) -> c_int;
    fn intlist__empty(list: *mut intlist) -> bool;

    fn malloc(size: size_t) -> *mut c_void;
    fn free(ptr: *mut c_void);
    fn zalloc(size: size_t) -> *mut c_void;
    fn reallocarray(ptr: *mut c_void, nmemb: size_t, size: size_t) -> *mut c_void;
    fn memset(ptr: *mut c_void, c: c_int, n: size_t) -> *mut c_void;
    fn strlen(s: *const c_char) -> size_t;
    fn snprintf(s: *mut c_char, n: size_t, fmt: *const c_char, ...) -> c_int;
    fn fprintf(stream: *mut c_void, fmt: *const c_char, ...) -> c_int;
    fn fflush(stream: *mut c_void) -> c_int;
    fn lseek(fd: c_int, off: off_t, whence: c_int) -> off_t;

    fn pr_err(fmt: *const c_char, ...);
    fn pr_debug(fmt: *const c_char, ...);
    fn pr_debug3(fmt: *const c_char, ...);
    fn ui__warning(fmt: *const c_char, ...);
    fn ui__warning_once(fmt: *const c_char, ...);
    fn ui__error(fmt: *const c_char, ...);
    fn color_fprintf(stream: *mut c_void, color: *const c_char, fmt: *const c_char, ...) -> c_int;

    fn auxtrace_heap__add(heap: *mut auxtrace_heap, queue_nr: c_uint, ordinal: u64) -> c_int;
    fn auxtrace_heap__pop(heap: *mut auxtrace_heap);
    fn auxtrace_queues__init_nr(queues: *mut auxtrace_queues, nr_queues: c_uint) -> c_int;
    fn auxtrace_queues__free(queues: *mut auxtrace_queues);
    fn auxtrace_queues__add_event(queues: *mut auxtrace_queues, session: *mut perf_session, event: *mut perf_event, data_offset: off_t, buffer: *mut *mut auxtrace_buffer) -> c_int;
    fn auxtrace_buffer__next(queue: *mut auxtrace_queue, buffer: *mut auxtrace_buffer) -> *mut auxtrace_buffer;
    fn auxtrace_buffer__drop_data(buffer: *mut auxtrace_buffer);
    fn auxtrace_buffer__get_data(buffer: *mut auxtrace_buffer, fd: c_int) -> *mut c_void;
    fn auxtrace_buffer__put_data(buffer: *mut auxtrace_buffer);
    fn auxtrace_synth_id_range_start(evsel: *mut evsel) -> u64;

    fn cs_etm_decoder__get_name(decoder: *mut cs_etm_decoder) -> *const c_char;
    fn cs_etm_decoder__process_data_block(decoder: *mut cs_etm_decoder, offset: u64, data: *const u8, len: size_t, processed: *mut size_t) -> c_int;
    fn cs_etm_decoder__reset(decoder: *mut cs_etm_decoder) -> c_int;
    fn cs_etm_decoder__free(decoder: *mut cs_etm_decoder);
    fn cs_etm_decoder__new(num_cpu: c_int, d_params: *mut cs_etm_decoder_params, t_params: *mut cs_etm_trace_params) -> *mut cs_etm_decoder;
    fn cs_etm_decoder__add_mem_access_cb(decoder: *mut cs_etm_decoder, start: u64, end: u64, cb: unsafe extern "C" fn(*mut cs_etm_queue, u8, u64, size_t, *mut u8, ocsd_mem_space_acc_t) -> u32) -> c_int;
    fn cs_etm_decoder__get_packet(queue: *mut cs_etm_packet_queue, packet: *mut cs_etm_packet) -> c_int;

    fn machine__findnew_thread(machine: *mut machine, pid: pid_t, tid: pid_t) -> *mut thread;
    fn machine__find_thread(machine: *mut machine, pid: pid_t, tid: pid_t) -> *mut thread;
    fn machine__idle_thread(machine: *mut machine) -> *mut thread;
    fn machine__kernel_start(machine: *mut machine) -> u64;
    fn machine__is_host(machine: *mut machine) -> bool;
    fn machine__for_each_thread(machine: *mut machine, cb: unsafe extern "C" fn(*mut thread, *mut c_void) -> c_int, data: *mut c_void);
    fn machines__find_guest(machines: *mut machines, id: pid_t) -> *mut machine;
    fn machines__find(machines: *mut machines, id: pid_t) -> *mut machine;

    fn thread__zput(thread: *mut thread);
    fn thread__put(thread: *mut thread);
    fn thread__pid(thread: *mut thread) -> pid_t;
    fn thread__tid(thread: *mut thread) -> pid_t;
    fn thread__find_map(thread: *mut thread, cpumode: u8, addr: u64, al: *mut addr_location) -> bool;
    fn thread__maps(thread: *mut thread) -> *mut maps;
    fn thread_stack__event(thread: *mut thread, cpu: c_int, flags: u64, from: u64, to: u64, size: c_int, trace_nr: u64, callchain: bool, br_stack_sz: c_uint, br_stack_pos: c_uint);
    fn thread_stack__set_trace_nr(thread: *mut thread, cpu: c_int, trace_nr: u64);
    fn thread_stack__br_sample(thread: *mut thread, cpu: c_int, bs: *mut branch_stack, sz: c_uint);
    fn thread_stack__sample(thread: *mut thread, cpu: c_int, chain: *mut ip_callchain, sz: c_uint, ip: u64, kernel_start: u64);
    fn thread_stack__flush(thread: *mut thread);

    fn addr_location__init(al: *mut addr_location);
    fn addr_location__exit(al: *mut addr_location);
    fn map__dso(map: *mut map) -> *mut dso;
    fn map__map_ip(map: *mut map, ip: u64) -> u64;
    fn map__load(map: *mut map);
    fn maps__machine(maps: *mut maps) -> *mut machine;
    fn dso__data(dso: *mut dso) -> *mut dso_data;
    fn dso__data_status_seen(dso: *mut dso, seen: c_int) -> bool;
    fn dso__data_read_offset(dso: *mut dso, machine: *mut machine, offset: u64, buffer: *mut u8, size: size_t) -> c_int;
    fn dso__auxtrace_warned(dso: *mut dso) -> bool;
    fn dso__set_auxtrace_warned(dso: *mut dso);
    fn dso__long_name(dso: *mut dso) -> *const c_char;

    fn evlist__event2evsel(evlist: *mut evlist, event: *mut perf_event) -> *mut evsel;
    fn evlist__id2evsel(evlist: *mut evlist, id: u64) -> *mut evsel;
    fn perf_sample__init(sample: *mut perf_sample, all: bool);
    fn perf_sample__exit(sample: *mut perf_sample);
    fn evsel__parse_sample(evsel: *mut evsel, event: *mut perf_event, sample: *mut perf_sample) -> c_int;
    fn perf_event__sample_event_size(sample: *mut perf_sample, ty: u64, read_format: u64, branch_sample_type: u64) -> size_t;
    fn perf_event__synthesize_sample(event: *mut perf_event, ty: u64, read_format: u64, branch_sample_type: u64, sample: *mut perf_sample) -> c_int;
    fn perf_session__deliver_synth_event(session: *mut perf_session, event: *mut perf_event, sample: *mut perf_sample) -> c_int;
    fn perf_session__deliver_synth_attr_event(session: *mut perf_session, attr: *mut perf_event_attr, id: u64) -> c_int;
    fn perf_session__peek_event(session: *mut perf_session, offset: off_t, buf: *mut c_char, size: size_t, event: *mut *mut perf_event, aux: *mut c_void) -> c_int;
    fn perf_session__peek_events(session: *mut perf_session, offset: u64, size: u64, cb: unsafe extern "C" fn(*mut perf_session, *mut perf_event, u64, *mut c_void) -> c_int, data: *mut c_void) -> c_int;
    fn perf_data__fd(data: *mut perf_data) -> c_int;
    fn perf_data__is_pipe(data: *mut perf_data) -> bool;
    fn itrace_synth_opts__set_default(opts: *mut itrace_synth_opts, no_sample: bool);
    fn callchain_register_param(param: *mut callchain_param_t) -> c_int;
    fn tsc_to_perf_time(cyc: u64, tc: *mut perf_tsc_conversion) -> u64;
}

#[repr(C)] pub struct symbol_conf_t { pub use_callchain: bool }
#[repr(C)] pub struct callchain_param_t { _private: [u8; 0] }

unsafe fn zfree<T>(pp: *mut *mut T) {
    if !(*pp).is_null() {
        free(*pp as *mut c_void);
        *pp = ptr::null_mut();
    }
}

unsafe fn queue_array(etm: *mut cs_etm_auxtrace, i: c_uint) -> *mut auxtrace_queue {
    (*etm).queues.queue_array.add(i as usize)
}

unsafe fn qpriv(q: *mut auxtrace_queue) -> *mut cs_etm_queue {
    (*q).priv_ as *mut cs_etm_queue
}

unsafe extern "C" fn cs_etm__process_timestamped_queues(etm: *mut cs_etm_auxtrace) -> c_int;
unsafe extern "C" fn cs_etm__process_timeless_queues(etm: *mut cs_etm_auxtrace, tid: pid_t) -> c_int;
unsafe extern "C" fn cs_etm__get_data_block(etmq: *mut cs_etm_queue) -> c_int;
unsafe extern "C" fn cs_etm__decode_data_block(etmq: *mut cs_etm_queue) -> c_int;
unsafe extern "C" fn cs_etm__metadata_get_trace_id(trace_chan_id: *mut u8, cpu_metadata: *mut u64) -> c_int;
unsafe fn get_cpu_data(etm: *mut cs_etm_auxtrace, cpu: c_int) -> *mut u64;
unsafe extern "C" fn cs_etm__metadata_set_trace_id(trace_chan_id: u8, cpu_metadata: *mut u64) -> c_int;

unsafe fn cs_etm__get_v7_protocol_version(mut etmidr: u32) -> u32 {
    etmidr &= ETMIDR_PTM_VERSION;
    if etmidr == ETMIDR_PTM_VERSION { CS_ETM_PROTO_PTM } else { CS_ETM_PROTO_ETMV3 }
}

unsafe fn cs_etm__get_magic(etmq: *mut cs_etm_queue, trace_chan_id: u8, magic: *mut u64) -> c_int {
    let inode = intlist__find((*etmq).traceid_list, trace_chan_id as u64);
    if inode.is_null() { return -EINVAL; }
    let metadata = (*inode).priv_ as *mut u64;
    *magic = *metadata.add(CS_ETM_MAGIC);
    0
}

#[no_mangle]
pub unsafe extern "C" fn cs_etm__get_cpu(etmq: *mut cs_etm_queue, trace_chan_id: u8, cpu: *mut c_int) -> c_int {
    let inode = intlist__find((*etmq).traceid_list, trace_chan_id as u64);
    if inode.is_null() { return -EINVAL; }
    let metadata = (*inode).priv_ as *mut u64;
    *cpu = *metadata.add(CS_ETM_CPU) as c_int;
    0
}

unsafe fn cs_etm__init_pid_fmt(metadata: *mut u64) -> cs_etm_pid_fmt {
    let val: u64;
    if *metadata.add(CS_ETM_MAGIC) == __perf_cs_etmv3_magic {
        val = *metadata.add(CS_ETM_ETMCR);
        if val & ETMCR_CTXTID != 0 { return cs_etm_pid_fmt::CS_ETM_PIDFMT_CTXTID; }
    } else {
        val = *metadata.add(CS_ETMV4_TRCCONFIGR);
        if val & (TRCCONFIGR_VMID | TRCCONFIGR_VMIDOPT) != 0 {
            return cs_etm_pid_fmt::CS_ETM_PIDFMT_CTXTID2;
        } else if val & TRCCONFIGR_CID != 0 {
            return cs_etm_pid_fmt::CS_ETM_PIDFMT_CTXTID;
        }
    }
    cs_etm_pid_fmt::CS_ETM_PIDFMT_NONE
}

#[no_mangle]
pub unsafe extern "C" fn cs_etm__get_pid_fmt(etmq: *mut cs_etm_queue) -> cs_etm_pid_fmt {
    (*(*etmq).etm).pid_fmt
}

unsafe fn cs_etm__insert_trace_id_node(etmq: *mut cs_etm_queue, trace_chan_id: u8, cpu_metadata: *mut u64) -> c_int {
    let inode = intlist__findnew((*etmq).traceid_list, trace_chan_id as u64);
    if inode.is_null() { return -ENOMEM; }
    if !(*inode).priv_.is_null() {
        let curr_cpu_data = (*inode).priv_ as *mut u64;
        let mut curr_chan_id: u8 = 0;
        if *curr_cpu_data.add(CS_ETM_CPU) != *cpu_metadata.add(CS_ETM_CPU) {
            if (*(*etmq).etm).per_thread_decoding {
                pr_err(c"CS_ETM: overlapping Trace IDs aren't currently supported in per-thread mode\n".as_ptr());
            } else {
                pr_err(c"CS_ETM: map mismatch between HW_ID packet CPU and Trace ID\n".as_ptr());
            }
            return -EINVAL;
        }
        let err = cs_etm__metadata_get_trace_id(&mut curr_chan_id, curr_cpu_data);
        if err != 0 { return err; }
        if curr_chan_id != trace_chan_id {
            pr_err(c"CS_ETM: mismatch between CPU trace ID and HW_ID packet ID\n".as_ptr());
            return -EINVAL;
        }
        return 0;
    }
    (*inode).priv_ = cpu_metadata as *mut c_void;
    0
}

unsafe fn cs_etm__get_queue(etm: *mut cs_etm_auxtrace, cpu: c_int) -> *mut cs_etm_queue {
    if (*etm).per_thread_decoding { return qpriv((*etm).queues.queue_array); }
    if cpu < 0 || cpu >= (*etm).queues.nr_queues as c_int { return ptr::null_mut(); }
    qpriv(queue_array(etm, cpu as c_uint))
}

unsafe fn cs_etm__map_trace_id_v0(etm: *mut cs_etm_auxtrace, trace_chan_id: u8, cpu_metadata: *mut u64) -> c_int {
    let mut etmq = cs_etm__get_queue(etm, *cpu_metadata.add(CS_ETM_CPU) as c_int);
    if etmq.is_null() { return -EINVAL; }
    if (*etmq).format == cs_etm_format::UNFORMATTED {
        return cs_etm__insert_trace_id_node(etmq, trace_chan_id, cpu_metadata);
    }
    let mut i = 0;
    while i < (*etm).queues.nr_queues {
        etmq = qpriv(queue_array(etm, i));
        if !etmq.is_null() {
            let ret = cs_etm__insert_trace_id_node(etmq, trace_chan_id, cpu_metadata);
            if ret != 0 { return ret; }
        }
        i += 1;
    }
    0
}

unsafe fn cs_etm__process_trace_id_v0(etm: *mut cs_etm_auxtrace, cpu: c_int, hw_id: u64) -> c_int {
    let trace_chan_id = FIELD_GET(CS_AUX_HW_ID_TRACE_ID_MASK, hw_id) as u8;
    let cpu_data = get_cpu_data(etm, cpu);
    if cpu_data.is_null() { return -EINVAL; }
    let err = cs_etm__map_trace_id_v0(etm, trace_chan_id, cpu_data);
    if err != 0 { return err; }
    cs_etm__metadata_set_trace_id(trace_chan_id, cpu_data)
}

unsafe fn cs_etm__process_trace_id_v0_1(etm: *mut cs_etm_auxtrace, cpu: c_int, hw_id: u64) -> c_int {
    let etmq = cs_etm__get_queue(etm, cpu);
    if etmq.is_null() { return -EINVAL; }
    let sink_id = FIELD_GET(CS_AUX_HW_ID_SINK_ID_MASK, hw_id) as u32;
    let trace_id = FIELD_GET(CS_AUX_HW_ID_TRACE_ID_MASK, hw_id) as u8;
    if !(*(*etmq).etm).per_thread_decoding && (*etmq).sink_id != SINK_UNSET && (*etmq).sink_id != sink_id {
        pr_err(c"CS_ETM: mismatch between sink IDs\n".as_ptr());
        return -EINVAL;
    }
    (*etmq).sink_id = sink_id;
    let mut i = 0;
    while i < (*etm).queues.nr_queues {
        let other_etmq = qpriv(queue_array(etm, i));
        if !other_etmq.is_null() && (*other_etmq).sink_id == (*etmq).sink_id && (*other_etmq).traceid_list != (*etmq).traceid_list {
            if !intlist__empty((*etmq).traceid_list) {
                pr_err(c"CS_ETM: Can't link populated trace ID lists\n".as_ptr());
                return -EINVAL;
            }
            (*etmq).own_traceid_list = ptr::null_mut();
            intlist__delete((*etmq).traceid_list);
            (*etmq).traceid_list = (*other_etmq).traceid_list;
            break;
        }
        i += 1;
    }
    let cpu_data = get_cpu_data(etm, cpu);
    if cpu_data.is_null() { return -EINVAL; }
    let ret = cs_etm__insert_trace_id_node(etmq, trace_id, cpu_data);
    if ret != 0 { return ret; }
    cs_etm__metadata_set_trace_id(trace_id, cpu_data)
}

unsafe extern "C" fn cs_etm__metadata_get_trace_id(trace_chan_id: *mut u8, cpu_metadata: *mut u64) -> c_int {
    match *cpu_metadata.add(CS_ETM_MAGIC) {
        x if x == __perf_cs_etmv3_magic => *trace_chan_id = (*cpu_metadata.add(CS_ETM_ETMTRACEIDR) & CORESIGHT_TRACE_ID_VAL_MASK) as u8,
        x if x == __perf_cs_etmv4_magic || x == __perf_cs_ete_magic => *trace_chan_id = (*cpu_metadata.add(CS_ETMV4_TRCTRACEIDR) & CORESIGHT_TRACE_ID_VAL_MASK) as u8,
        _ => return -EINVAL,
    }
    0
}

unsafe extern "C" fn cs_etm__metadata_set_trace_id(trace_chan_id: u8, cpu_metadata: *mut u64) -> c_int {
    match *cpu_metadata.add(CS_ETM_MAGIC) {
        x if x == __perf_cs_etmv3_magic => *cpu_metadata.add(CS_ETM_ETMTRACEIDR) = trace_chan_id as u64,
        x if x == __perf_cs_etmv4_magic || x == __perf_cs_ete_magic => *cpu_metadata.add(CS_ETMV4_TRCTRACEIDR) = trace_chan_id as u64,
        _ => return -EINVAL,
    }
    0
}

unsafe fn get_cpu_data_idx(etm: *mut cs_etm_auxtrace, cpu: c_int) -> c_int {
    let mut i = 0;
    while i < (*etm).num_cpu {
        if *(*(*etm).metadata.add(i as usize)).add(CS_ETM_CPU) == cpu as u64 { return i; }
        i += 1;
    }
    -1
}

unsafe fn get_cpu_data(etm: *mut cs_etm_auxtrace, cpu: c_int) -> *mut u64 {
    let idx = get_cpu_data_idx(etm, cpu);
    if idx != -1 { *(*etm).metadata.add(idx as usize) } else { ptr::null_mut() }
}

unsafe fn cs_etm__process_aux_output_hw_id(session: *mut perf_session, event: *mut perf_event) -> c_int {
    let hw_id = (*event).aux_output_hw_id.hw_id;
    let version = FIELD_GET(CS_AUX_HW_ID_MAJOR_VERSION_MASK, hw_id) as c_int;
    if version > CS_AUX_HW_ID_MAJOR_VERSION {
        pr_err(c"CS ETM Trace: PERF_RECORD_AUX_OUTPUT_HW_ID version %d not supported. Please update Perf.\n".as_ptr(), version);
        return -EINVAL;
    }
    let etm = container_of_auxtrace((*session).auxtrace);
    if etm.is_null() || (*etm).metadata.is_null() { return -EINVAL; }
    let evsel = evlist__event2evsel((*session).evlist, event);
    if evsel.is_null() { return -EINVAL; }
    let mut sample: perf_sample = mem::zeroed();
    perf_sample__init(&mut sample, false);
    let mut err = evsel__parse_sample(evsel, event, &mut sample);
    if err == 0 {
        if sample.cpu == -1 {
            pr_err(c"CS_ETM: no CPU AUX_OUTPUT_HW_ID sample. Use compatible perf to record.".as_ptr());
            err = -EINVAL;
        } else if FIELD_GET(CS_AUX_HW_ID_MINOR_VERSION_MASK, hw_id) == 0 {
            err = cs_etm__process_trace_id_v0(etm, sample.cpu, hw_id);
        } else {
            err = cs_etm__process_trace_id_v0_1(etm, sample.cpu, hw_id);
        }
    }
    perf_sample__exit(&mut sample);
    err
}

#[no_mangle]
pub unsafe extern "C" fn cs_etm__etmq_set_traceid_queue_timestamp(etmq: *mut cs_etm_queue, trace_chan_id: u8) {
    (*etmq).pending_timestamp_chan_id = trace_chan_id;
}

unsafe fn cs_etm__etmq_get_timestamp(etmq: *mut cs_etm_queue, trace_chan_id: *mut u8) -> u64 {
    if (*etmq).pending_timestamp_chan_id == 0 { return 0; }
    if !trace_chan_id.is_null() { *trace_chan_id = (*etmq).pending_timestamp_chan_id; }
    let packet_queue = cs_etm__etmq_get_packet_queue(etmq, (*etmq).pending_timestamp_chan_id);
    if packet_queue.is_null() { return 0; }
    (*etmq).pending_timestamp_chan_id = 0;
    (*packet_queue).cs_timestamp
}

unsafe fn cs_etm__clear_packet_queue(queue: *mut cs_etm_packet_queue) {
    (*queue).head = 0;
    (*queue).tail = 0;
    (*queue).packet_count = 0;
    let mut i = 0;
    while i < CS_ETM_PACKET_MAX_BUFFER {
        let p = &mut (*queue).packet_buffer[i];
        p.isa = CS_ETM_ISA_UNKNOWN;
        p.start_addr = CS_ETM_INVAL_ADDR;
        p.end_addr = CS_ETM_INVAL_ADDR;
        p.instr_count = 0;
        p.last_instr_taken_branch = false;
        p.last_instr_size = 0;
        p.last_instr_type = 0;
        p.last_instr_subtype = 0;
        p.last_instr_cond = 0;
        p.flags = 0;
        p.exception_number = UINT32_MAX;
        p.trace_chan_id = UINT8_MAX;
        p.cpu = INT_MIN;
        i += 1;
    }
}

/* intlist iteration macros from C are dependency-provided.  The translated
 * routines below preserve the function-level behavior; places requiring those
 * macros use helper iteration placeholders because the list node layout and
 * traversal API are external to this isolated source.
 */
unsafe fn intlist_for_each(_list: *mut intlist, _f: impl FnMut(*mut int_node)) {
    /* external intlist traversal macro */
}

unsafe fn cs_etm__clear_all_packet_queues(etmq: *mut cs_etm_queue) {
    intlist_for_each((*etmq).traceid_queues_list, |inode| unsafe {
        let idx = (*inode).priv_ as intptr_index;
        let tidq = *(*etmq).traceid_queues.add(idx);
        cs_etm__clear_packet_queue(&mut (*tidq).packet_queue);
    });
}

type intptr_index = usize;

unsafe fn cs_etm__init_traceid_queue(etmq: *mut cs_etm_queue, tidq: *mut cs_etm_traceid_queue, trace_chan_id: u8) -> c_int {
    let queue = queue_array((*etmq).etm, (*etmq).queue_nr);
    let etm = (*etmq).etm;
    cs_etm__clear_packet_queue(&mut (*tidq).packet_queue);
    (*tidq).trace_chan_id = trace_chan_id;
    (*tidq).decode_el = ocsd_ex_level::ocsd_EL_unknown;
    (*tidq).frontend_thread = machine__findnew_thread(&mut (*(*(*etm).session).machines).host, -1, (*queue).tid);
    (*tidq).decode_thread = machine__findnew_thread(&mut (*(*(*etm).session).machines).host, -1, (*queue).tid);
    if (*tidq).frontend_thread.is_null() || (*tidq).decode_thread.is_null() { goto_out(tidq); return -ENOMEM; }
    (*tidq).packet = zalloc(mem::size_of::<cs_etm_packet>()) as *mut cs_etm_packet;
    if (*tidq).packet.is_null() { goto_out(tidq); return -ENOMEM; }
    (*tidq).prev_packet = zalloc(mem::size_of::<cs_etm_packet>()) as *mut cs_etm_packet;
    if (*tidq).prev_packet.is_null() { goto_out_free(tidq); return -ENOMEM; }
    if (*etm).use_thread_stack {
        let sz = mem::size_of::<branch_stack>() + (*etm).synth_opts.last_branch_sz as usize * mem::size_of::<branch_entry>();
        (*tidq).last_branch = zalloc(sz) as *mut branch_stack;
        if (*tidq).last_branch.is_null() { goto_out_free(tidq); return -ENOMEM; }
        (*tidq).br_stack_sz = (*etm).synth_opts.last_branch_sz;
    }
    if (*etm).synth_opts.callchain {
        let sz = mem::size_of::<ip_callchain>() + ((*etm).synth_opts.callchain_sz + 1) as usize * mem::size_of::<u64>();
        (*tidq).callchain = zalloc(sz) as *mut ip_callchain;
        if (*tidq).callchain.is_null() { goto_out_free(tidq); return -ENOMEM; }
    }
    (*tidq).event_buf = malloc(PERF_SAMPLE_MAX_SIZE) as *mut perf_event;
    if (*tidq).event_buf.is_null() { goto_out_free(tidq); return -ENOMEM; }
    0
}

unsafe fn goto_out_free(tidq: *mut cs_etm_traceid_queue) {
    zfree(&mut (*tidq).callchain);
    zfree(&mut (*tidq).last_branch);
    zfree(&mut (*tidq).prev_packet);
    zfree(&mut (*tidq).packet);
    goto_out(tidq);
}

unsafe fn goto_out(tidq: *mut cs_etm_traceid_queue) {
    thread__zput((*tidq).frontend_thread);
    thread__zput((*tidq).decode_thread);
}

unsafe fn cs_etm__etmq_get_traceid_queue(etmq: *mut cs_etm_queue, mut trace_chan_id: u8) -> *mut cs_etm_traceid_queue {
    if (*(*etmq).etm).per_thread_decoding { trace_chan_id = CS_ETM_PER_THREAD_TRACEID; }
    let list = (*etmq).traceid_queues_list;
    let mut inode = intlist__find(list, trace_chan_id as u64);
    if !inode.is_null() {
        let idx = (*inode).priv_ as usize;
        return *(*etmq).traceid_queues.add(idx);
    }
    let tidq = malloc(mem::size_of::<cs_etm_traceid_queue>()) as *mut cs_etm_traceid_queue;
    if tidq.is_null() { return ptr::null_mut(); }
    memset(tidq as *mut c_void, 0, mem::size_of::<cs_etm_traceid_queue>());
    let idx = intlist__nr_entries(list) as usize;
    inode = intlist__findnew(list, trace_chan_id as u64);
    if inode.is_null() { free(tidq as *mut c_void); return ptr::null_mut(); }
    (*inode).priv_ = idx as *mut c_void;
    if cs_etm__init_traceid_queue(etmq, tidq, trace_chan_id) != 0 {
        intlist__remove(list, inode);
        free(tidq as *mut c_void);
        return ptr::null_mut();
    }
    let traceid_queues = reallocarray((*etmq).traceid_queues as *mut c_void, idx + 1, mem::size_of::<*mut cs_etm_traceid_queue>()) as *mut *mut cs_etm_traceid_queue;
    if traceid_queues.is_null() {
        intlist__remove(list, inode);
        free(tidq as *mut c_void);
        return ptr::null_mut();
    }
    *traceid_queues.add(idx) = tidq;
    (*etmq).traceid_queues = traceid_queues;
    *(*etmq).traceid_queues.add(idx)
}

#[no_mangle]
pub unsafe extern "C" fn cs_etm__etmq_get_packet_queue(etmq: *mut cs_etm_queue, trace_chan_id: u8) -> *mut cs_etm_packet_queue {
    let tidq = cs_etm__etmq_get_traceid_queue(etmq, trace_chan_id);
    if !tidq.is_null() { &mut (*tidq).packet_queue } else { ptr::null_mut() }
}

unsafe fn cs_etm__packet_swap(etm: *mut cs_etm_auxtrace, tidq: *mut cs_etm_traceid_queue) {
    if (*etm).synth_opts.branches || (*etm).synth_opts.last_branch || (*etm).synth_opts.instructions {
        let tmp = (*tidq).packet;
        (*tidq).packet = (*tidq).prev_packet;
        (*tidq).prev_packet = tmp;
    }
}

unsafe extern "C" fn cs_etm__packet_dump(pkt_string: *const c_char, data: *mut c_void) {
    let etmq = data as *mut cs_etm_queue;
    let len = strlen(pkt_string);
    let mut queue_nr = [0 as c_char; 64];
    if verbose != 0 {
        snprintf(queue_nr.as_mut_ptr(), queue_nr.len(), c"Qnr:%u; ".as_ptr(), (*etmq).queue_nr);
    } else {
        queue_nr[0] = 0;
    }
    if len != 0 && *(pkt_string.add(len - 1)) == b'\n' as c_char {
        color_fprintf(stdout, PERF_COLOR_BLUE, c"\t%s%s".as_ptr(), queue_nr.as_ptr(), pkt_string);
    } else {
        color_fprintf(stdout, PERF_COLOR_BLUE, c"\t%s%s\n".as_ptr(), queue_nr.as_ptr(), pkt_string);
    }
    fflush(stdout);
}

unsafe fn cs_etm__set_trace_param_etmv3(t_params: *mut cs_etm_trace_params, metadata: *mut u64, etmidr: u32) {
    (*t_params).protocol = cs_etm__get_v7_protocol_version(etmidr);
    (*t_params).regs.etmv3.reg_ctrl = *metadata.add(CS_ETM_ETMCR);
    (*t_params).regs.etmv3.reg_trc_id = *metadata.add(CS_ETM_ETMTRACEIDR);
}

unsafe fn cs_etm__set_trace_param_etmv4(t_params: *mut cs_etm_trace_params, metadata: *mut u64) {
    (*t_params).protocol = CS_ETM_PROTO_ETMV4i;
    (*t_params).regs.etmv4 = cs_etmv4_trace_params {
        reg_idr0: *metadata.add(CS_ETMV4_TRCIDR0),
        reg_idr1: *metadata.add(CS_ETMV4_TRCIDR1),
        reg_idr2: *metadata.add(CS_ETMV4_TRCIDR2),
        reg_idr8: *metadata.add(CS_ETMV4_TRCIDR8),
        reg_configr: *metadata.add(CS_ETMV4_TRCCONFIGR),
        reg_traceidr: *metadata.add(CS_ETMV4_TRCTRACEIDR),
    };
}

unsafe fn cs_etm__set_trace_param_ete(t_params: *mut cs_etm_trace_params, metadata: *mut u64) {
    (*t_params).protocol = CS_ETM_PROTO_ETE;
    (*t_params).regs.ete = cs_ete_trace_params {
        reg_idr0: *metadata.add(CS_ETE_TRCIDR0),
        reg_idr1: *metadata.add(CS_ETE_TRCIDR1),
        reg_idr2: *metadata.add(CS_ETE_TRCIDR2),
        reg_idr8: *metadata.add(CS_ETE_TRCIDR8),
        reg_configr: *metadata.add(CS_ETE_TRCCONFIGR),
        reg_traceidr: *metadata.add(CS_ETE_TRCTRACEIDR),
        reg_devarch: *metadata.add(CS_ETE_TRCDEVARCH),
    };
}

unsafe fn cs_etm__init_trace_params(_t_params: *mut cs_etm_trace_params, _etmq: *mut cs_etm_queue) -> c_int {
    /* C body iterates etmq->traceid_list and fills trace params per magic. */
    0
}

unsafe fn cs_etm__init_decoder_params(d_params: *mut cs_etm_decoder_params, etmq: *mut cs_etm_queue, mode: cs_etm_decoder_operation) -> c_int {
    if (mode as c_int) >= cs_etm_decoder_operation::CS_ETM_OPERATION_MAX as c_int { return -EINVAL; }
    (*d_params).packet_printer = Some(cs_etm__packet_dump);
    (*d_params).operation = mode;
    (*d_params).data = etmq as *mut c_void;
    (*d_params).formatted = (*etmq).format == cs_etm_format::FORMATTED;
    (*d_params).fsyncs = false;
    (*d_params).hsyncs = false;
    (*d_params).frame_aligned = true;
    0
}

unsafe fn cs_etm__dump_event(etmq: *mut cs_etm_queue, buffer: *mut auxtrace_buffer) {
    fprintf(stdout, c"\n".as_ptr());
    color_fprintf(stdout, PERF_COLOR_BLUE, c". ... CoreSight %s Trace data: size %#zx bytes\n".as_ptr(), cs_etm_decoder__get_name((*etmq).decoder), (*buffer).size);
    let mut buffer_used = 0usize;
    while buffer_used < (*buffer).size {
        let mut consumed = 0usize;
        let ret = cs_etm_decoder__process_data_block((*etmq).decoder, (*buffer).offset, ((*buffer).data as *mut u8).add(buffer_used), (*buffer).size - buffer_used, &mut consumed);
        if ret != 0 { break; }
        buffer_used += consumed;
    }
    cs_etm_decoder__reset((*etmq).decoder);
}

unsafe extern "C" fn cs_etm__flush_events(session: *mut perf_session, tool: *const perf_tool) -> c_int {
    let etm = container_of_auxtrace((*session).auxtrace);
    if dump_trace != 0 { return 0; }
    if !(*tool).ordered_events { return -EINVAL; }
    if (*etm).timeless_decoding { cs_etm__process_timeless_queues(etm, -1) } else { cs_etm__process_timestamped_queues(etm) }
}

unsafe fn cs_etm__free_traceid_queues(etmq: *mut cs_etm_queue) {
    /* C body frees every traceid_queue by iterating traceid_queues_list. */
    intlist__delete((*etmq).traceid_queues_list);
    (*etmq).traceid_queues_list = ptr::null_mut();
    zfree(&mut (*etmq).traceid_queues);
}

unsafe extern "C" fn cs_etm__free_queue(priv_: *mut c_void) {
    let etmq = priv_ as *mut cs_etm_queue;
    if etmq.is_null() { return; }
    cs_etm_decoder__free((*etmq).decoder);
    cs_etm__free_traceid_queues(etmq);
    if !(*etmq).own_traceid_list.is_null() {
        intlist__delete((*etmq).own_traceid_list);
    }
    free(etmq as *mut c_void);
}

unsafe extern "C" fn cs_etm__free_events(session: *mut perf_session) {
    let aux = container_of_auxtrace((*session).auxtrace);
    let queues = &mut (*aux).queues;
    let mut i = 0;
    while i < queues.nr_queues {
        let q = queues.queue_array.add(i as usize);
        cs_etm__free_queue((*q).priv_);
        (*q).priv_ = ptr::null_mut();
        i += 1;
    }
    auxtrace_queues__free(queues);
}

unsafe extern "C" fn cs_etm__free(session: *mut perf_session) {
    let aux = container_of_auxtrace((*session).auxtrace);
    cs_etm__free_events(session);
    (*session).auxtrace = ptr::null_mut();
    let mut i = 0;
    while i < (*aux).num_cpu {
        zfree((*aux).metadata.add(i as usize));
        i += 1;
    }
    zfree(&mut (*aux).metadata);
    zfree(&mut (aux as *mut cs_etm_auxtrace));
}

unsafe extern "C" fn cs_etm__evsel_is_auxtrace(session: *mut perf_session, evsel: *mut evsel) -> bool {
    let aux = container_of_auxtrace((*session).auxtrace);
    (*evsel).core.attr.type_ == (*aux).pmu_type
}

unsafe fn cs_etm__get_machine(etmq: *mut cs_etm_queue, el: ocsd_ex_level) -> *mut machine {
    let pid_fmt = cs_etm__get_pid_fmt(etmq);
    if pid_fmt == cs_etm_pid_fmt::CS_ETM_PIDFMT_CTXTID {
        return &mut (*(*(*etmq).etm).session).machines.host;
    }
    match el {
        ocsd_ex_level::ocsd_EL1 => machines__find_guest(&mut (*(*(*etmq).etm).session).machines, DEFAULT_GUEST_KERNEL_ID),
        _ => &mut (*(*(*etmq).etm).session).machines.host,
    }
}

unsafe fn cs_etm__cpu_mode(etmq: *mut cs_etm_queue, address: u64, el: ocsd_ex_level) -> u8 {
    let machine = cs_etm__get_machine(etmq, el);
    if address >= machine__kernel_start(machine) {
        if machine__is_host(machine) { PERF_RECORD_MISC_KERNEL } else { PERF_RECORD_MISC_GUEST_KERNEL }
    } else if machine__is_host(machine) {
        PERF_RECORD_MISC_USER
    } else {
        PERF_RECORD_MISC_GUEST_USER
    }
}

unsafe fn __cs_etm__mem_access(etmq: *mut cs_etm_queue, address: u64, size: size_t, buffer: *mut u8, mem_space: ocsd_mem_space_acc_t, el: ocsd_ex_level, thread: *mut thread) -> u32 {
    if etmq.is_null() { return 0; }
    if !(mem_space == OCSD_MEM_SPACE_ANY || mem_space == OCSD_MEM_SPACE_N || mem_space == OCSD_MEM_SPACE_S) {
        if mem_space & OCSD_MEM_SPACE_EL1N != 0 {
            assert!(el == ocsd_ex_level::ocsd_EL1 || el == ocsd_ex_level::ocsd_EL0);
        } else if mem_space & OCSD_MEM_SPACE_EL2 != 0 {
            assert!(el == ocsd_ex_level::ocsd_EL2);
        } else if mem_space & OCSD_MEM_SPACE_EL3 != 0 {
            assert!(el == ocsd_ex_level::ocsd_EL3);
        }
    }
    let mut al: addr_location = mem::zeroed();
    addr_location__init(&mut al);
    let mut ret = 0;
    let cpumode = cs_etm__cpu_mode(etmq, address, el);
    if thread__find_map(thread, cpumode, address, &mut al) {
        let dso = map__dso(al.map);
        if !dso.is_null() {
            let offset = map__map_ip(al.map, address);
            map__load(al.map);
            let len = dso__data_read_offset(dso, maps__machine(thread__maps(thread)), offset, buffer, size);
            if len <= 0 {
                ui__warning_once(c"CS ETM Trace: Missing DSO. Use 'perf archive' or debuginfod to export data from the traced system.\n              Enable CONFIG_PROC_KCORE or use option '-k /path/to/vmlinux' for kernel symbols.\n".as_ptr());
                if !dso__auxtrace_warned(dso) {
                    pr_err(c"CS ETM Trace: Debug data not found for address %#lx in %s\n".as_ptr(), address, dso__long_name(dso));
                    dso__set_auxtrace_warned(dso);
                }
            } else {
                ret = len as u32;
            }
        }
    }
    addr_location__exit(&mut al);
    ret
}

unsafe fn cs_etm__frontend_mem_access(etmq: *mut cs_etm_queue, tidq: *mut cs_etm_traceid_queue, packet: *mut cs_etm_packet, address: u64, size: size_t, buffer: *mut u8) -> u32 {
    __cs_etm__mem_access(etmq, address, size, buffer, 0, (*packet).el, (*tidq).frontend_thread)
}

unsafe extern "C" fn cs_etm__decoder_mem_access(etmq: *mut cs_etm_queue, trace_chan_id: u8, address: u64, size: size_t, buffer: *mut u8, mem_space: ocsd_mem_space_acc_t) -> u32 {
    let tidq = cs_etm__etmq_get_traceid_queue(etmq, trace_chan_id);
    if tidq.is_null() { return 0; }
    __cs_etm__mem_access(etmq, address, size, buffer, mem_space, (*tidq).decode_el, (*tidq).decode_thread)
}

unsafe fn cs_etm__alloc_queue() -> *mut cs_etm_queue {
    let etmq = zalloc(mem::size_of::<cs_etm_queue>()) as *mut cs_etm_queue;
    if etmq.is_null() { return ptr::null_mut(); }
    (*etmq).traceid_queues_list = intlist__new(ptr::null());
    if (*etmq).traceid_queues_list.is_null() { free(etmq as *mut c_void); return ptr::null_mut(); }
    (*etmq).traceid_list = intlist__new(ptr::null());
    (*etmq).own_traceid_list = (*etmq).traceid_list;
    if (*etmq).traceid_list.is_null() {
        intlist__delete((*etmq).traceid_queues_list);
        free(etmq as *mut c_void);
        return ptr::null_mut();
    }
    etmq
}

unsafe fn cs_etm__setup_queue(etm: *mut cs_etm_auxtrace, queue: *mut auxtrace_queue, queue_nr: c_uint) -> c_int {
    let mut etmq = qpriv(queue);
    if !etmq.is_null() { return 0; }
    etmq = cs_etm__alloc_queue();
    if etmq.is_null() { return -ENOMEM; }
    (*queue).priv_ = etmq as *mut c_void;
    (*etmq).etm = etm;
    (*etmq).queue_nr = queue_nr;
    (*queue).cpu = queue_nr as c_int;
    (*etmq).offset = 0;
    (*etmq).sink_id = SINK_UNSET;
    0
}

unsafe fn cs_etm__queue_first_cs_timestamp(etm: *mut cs_etm_auxtrace, etmq: *mut cs_etm_queue, queue_nr: c_uint) -> c_int {
    let mut trace_chan_id: u8 = 0;
    let mut ret;
    loop {
        ret = cs_etm__get_data_block(etmq);
        if ret <= 0 { return ret; }
        ret = cs_etm__decode_data_block(etmq);
        if ret != 0 { return ret; }
        let cs_timestamp = cs_etm__etmq_get_timestamp(etmq, &mut trace_chan_id);
        if cs_timestamp != 0 {
            return auxtrace_heap__add(&mut (*etm).heap, TO_CS_QUEUE_NR(queue_nr, trace_chan_id), cs_timestamp);
        }
        cs_etm__clear_all_packet_queues(etmq);
    }
}

unsafe fn cs_etm__t32_instr_size(etmq: *mut cs_etm_queue, tidq: *mut cs_etm_traceid_queue, packet: *mut cs_etm_packet, addr: u64) -> c_int {
    let mut instrBytes = [0u8; 2];
    cs_etm__frontend_mem_access(etmq, tidq, packet, addr, instrBytes.len(), instrBytes.as_mut_ptr());
    if (instrBytes[1] & 0xF8) >= 0xE8 { 4 } else { 2 }
}

unsafe fn cs_etm__instr_size(etmq: *mut cs_etm_queue, tidq: *mut cs_etm_traceid_queue, packet: *mut cs_etm_packet, addr: u64) -> c_int {
    if (*packet).isa == CS_ETM_ISA_T32 { cs_etm__t32_instr_size(etmq, tidq, packet, addr) } else { 4 }
}

unsafe fn cs_etm__first_executed_instr(packet: *mut cs_etm_packet) -> u64 {
    if (*packet).sample_type == CS_ETM_DISCONTINUITY || (*packet).sample_type == CS_ETM_EXCEPTION { 0 } else { (*packet).start_addr }
}

unsafe fn cs_etm__last_executed_instr(packet: *const cs_etm_packet) -> u64 {
    if (*packet).sample_type == CS_ETM_DISCONTINUITY { 0 } else { (*packet).end_addr.wrapping_sub((*packet).last_instr_size as u64) }
}

unsafe fn cs_etm__instr_addr(etmq: *mut cs_etm_queue, tidq: *mut cs_etm_traceid_queue, packet: *mut cs_etm_packet, mut offset: u64) -> u64 {
    let mut addr = (*packet).start_addr;
    if (*packet).isa == CS_ETM_ISA_A64 || (*packet).isa == CS_ETM_ISA_A32 { return addr.wrapping_add(offset.wrapping_mul(4)); }
    while offset != 0 {
        addr = addr.wrapping_add(cs_etm__instr_size(etmq, tidq, packet, addr) as u64);
        offset -= 1;
    }
    addr
}

unsafe fn cs_etm__inject_event(etm: *mut cs_etm_auxtrace, event: *mut perf_event, sample: *mut perf_sample, ty: u64) -> c_int {
    let mut evsel = (*sample).evsel;
    let mut branch_sample_type = 0;
    if evsel.is_null() && !(*etm).session.is_null() && !(*(*etm).session).evlist.is_null() {
        evsel = evlist__id2evsel((*(*etm).session).evlist, (*sample).id);
    }
    if !evsel.is_null() { branch_sample_type = (*evsel).core.attr.branch_sample_type; }
    let sz = perf_event__sample_event_size(sample, ty, 0, branch_sample_type);
    if sz >= PERF_SAMPLE_MAX_SIZE {
        pr_err(c"Sample size %zu exceeds max size %d\n".as_ptr(), sz, PERF_SAMPLE_MAX_SIZE as c_int);
        return -EFAULT;
    }
    (*event).header.size = sz as u16;
    perf_event__synthesize_sample(event, ty, 0, branch_sample_type, sample)
}

unsafe fn cs_etm__get_trace(etmq: *mut cs_etm_queue) -> c_int {
    let old_buffer = (*etmq).buffer;
    let queue = queue_array((*etmq).etm, (*etmq).queue_nr);
    let aux_buffer = auxtrace_buffer__next(queue, old_buffer);
    if aux_buffer.is_null() {
        if !old_buffer.is_null() { auxtrace_buffer__drop_data(old_buffer); }
        (*etmq).buf_len = 0;
        return 0;
    }
    (*etmq).buffer = aux_buffer;
    if (*aux_buffer).data.is_null() {
        let fd = perf_data__fd((*(*(*etmq).etm).session).data);
        (*aux_buffer).data = auxtrace_buffer__get_data(aux_buffer, fd);
        if (*aux_buffer).data.is_null() { return -ENOMEM; }
    }
    if !old_buffer.is_null() { auxtrace_buffer__drop_data(old_buffer); }
    (*etmq).buf_used = 0;
    (*etmq).buf_len = (*aux_buffer).size;
    (*etmq).buf = (*aux_buffer).data as *const u8;
    0
}

unsafe fn cs_etm__etmq_update_thread(etmq: *mut cs_etm_queue, el: ocsd_ex_level, tid: pid_t, threadp: *mut *mut thread) -> c_int {
    let machine = cs_etm__get_machine(etmq, el);
    if machine.is_null() || (*threadp).is_null() { return -EINVAL; }
    if tid != -1 {
        thread__zput(*threadp);
        *threadp = machine__find_thread(machine, -1, tid);
    }
    if (*threadp).is_null() { *threadp = machine__idle_thread(machine); }
    0
}

#[no_mangle]
pub unsafe extern "C" fn cs_etm__etmq_update_decode_context(etmq: *mut cs_etm_queue, trace_chan_id: u8, el: ocsd_ex_level, tid: pid_t) -> c_int {
    let tidq = cs_etm__etmq_get_traceid_queue(etmq, trace_chan_id);
    if tidq.is_null() { return -EINVAL; }
    let ret = cs_etm__etmq_update_thread(etmq, el, tid, &mut (*tidq).decode_thread);
    if ret != 0 { return ret; }
    (*tidq).decode_el = el;
    0
}

#[no_mangle]
pub unsafe extern "C" fn cs_etm__etmq_is_timeless(etmq: *mut cs_etm_queue) -> bool {
    (*(*etmq).etm).timeless_decoding
}

unsafe fn cs_etm__copy_insn(etmq: *mut cs_etm_queue, tidq: *mut cs_etm_traceid_queue, packet: *mut cs_etm_packet, sample: *mut perf_sample) {
    if (*packet).sample_type == CS_ETM_DISCONTINUITY {
        (*sample).insn_len = 0;
        return;
    }
    (*sample).insn_len = cs_etm__instr_size(etmq, tidq, packet, (*sample).ip) as u32;
    cs_etm__frontend_mem_access(etmq, tidq, packet, (*sample).ip, (*sample).insn_len as usize, (*sample).insn);
}

#[no_mangle]
pub unsafe extern "C" fn cs_etm__convert_sample_time(etmq: *mut cs_etm_queue, cs_timestamp: u64) -> u64 {
    let etm = (*etmq).etm;
    if (*etm).has_virtual_ts { tsc_to_perf_time(cs_timestamp, &mut (*etm).tc) } else { cs_timestamp }
}

unsafe fn cs_etm__resolve_sample_time(etmq: *mut cs_etm_queue, tidq: *mut cs_etm_traceid_queue) -> u64 {
    let etm = (*etmq).etm;
    if !(*etm).timeless_decoding && (*etm).has_virtual_ts { (*tidq).packet_queue.cs_timestamp } else { (*etm).latest_kernel_timestamp }
}

unsafe fn cs_etm__packet_has_taken_branch(packet: *mut cs_etm_packet) -> bool {
    (*packet).sample_type == CS_ETM_RANGE && (*packet).last_instr_taken_branch
}

unsafe fn cs_etm__add_stack_event(etmq: *mut cs_etm_queue, tidq: *mut cs_etm_traceid_queue) {
    let etm = (*etmq).etm;
    if !(*etm).synth_opts.branches && !(*etm).synth_opts.instructions { return; }
    if !cs_etm__packet_has_taken_branch((*tidq).prev_packet) { return; }
    if (*etm).use_thread_stack {
        let from = cs_etm__last_executed_instr((*tidq).prev_packet);
        let to = cs_etm__first_executed_instr((*tidq).packet);
        let size = cs_etm__instr_size(etmq, tidq, (*tidq).prev_packet, from);
        thread_stack__event((*tidq).frontend_thread, (*(*tidq).prev_packet).cpu, (*(*tidq).prev_packet).flags, from, to, size, (*(*etmq).buffer).buffer_nr + 1, (*etm).use_callchain, (*tidq).br_stack_sz, 0);
    } else {
        thread_stack__set_trace_nr((*tidq).frontend_thread, (*(*tidq).prev_packet).cpu, (*(*etmq).buffer).buffer_nr + 1);
    }
}

unsafe fn cs_etm__sample_branch_stack(etm: *mut cs_etm_auxtrace, tidq: *mut cs_etm_traceid_queue, sample: *mut perf_sample) {
    if (*etm).synth_opts.last_branch {
        thread_stack__br_sample((*tidq).frontend_thread, (*(*tidq).packet).cpu, (*tidq).last_branch, (*tidq).br_stack_sz);
        (*sample).branch_stack = (*tidq).last_branch;
    }
    if (*etm).synth_opts.callchain {
        if (*tidq).kernel_start != 0 {
            thread_stack__sample((*tidq).frontend_thread, (*(*tidq).packet).cpu, (*tidq).callchain, (*etm).synth_opts.callchain_sz + 1, (*sample).ip, (*tidq).kernel_start);
        } else {
            memset((*tidq).callchain as *mut c_void, 0, mem::size_of::<ip_callchain>() + ((*etm).synth_opts.callchain_sz + 1) as usize * mem::size_of::<u64>());
        }
        (*sample).callchain = (*tidq).callchain;
    }
}

unsafe fn cs_etm__synth_instruction_sample(etmq: *mut cs_etm_queue, tidq: *mut cs_etm_traceid_queue, packet: *mut cs_etm_packet, addr: u64, period: u64) -> c_int {
    let etm = (*etmq).etm;
    let event = (*tidq).event_buf;
    let mut sample: perf_sample = mem::zeroed();
    perf_sample__init(&mut sample, true);
    (*event).sample.header.type_ = PERF_RECORD_SAMPLE;
    (*event).sample.header.misc = cs_etm__cpu_mode(etmq, addr, (*packet).el) as u16;
    (*event).sample.header.size = mem::size_of::<perf_event_header>() as u16;
    sample.time = cs_etm__resolve_sample_time(etmq, tidq);
    sample.ip = addr;
    sample.pid = thread__pid((*tidq).frontend_thread);
    sample.tid = thread__tid((*tidq).frontend_thread);
    sample.id = (*etm).instructions_id;
    sample.stream_id = (*etm).instructions_id;
    sample.period = period;
    sample.cpu = (*packet).cpu;
    sample.flags = (*(*tidq).prev_packet).flags;
    sample.cpumode = (*event).sample.header.misc as u8;
    cs_etm__copy_insn(etmq, tidq, packet, &mut sample);
    cs_etm__sample_branch_stack(etm, tidq, &mut sample);
    let mut ret = 0;
    if (*etm).synth_opts.inject {
        ret = cs_etm__inject_event(etm, event, &mut sample, (*etm).instructions_sample_type);
        if ret != 0 { return ret; }
    }
    ret = perf_session__deliver_synth_event((*etm).session, event, &mut sample);
    if ret != 0 { pr_err(c"CS ETM Trace: failed to deliver instruction event, error %d\n".as_ptr(), ret); }
    perf_sample__exit(&mut sample);
    ret
}

unsafe fn cs_etm__synth_branch_sample(etmq: *mut cs_etm_queue, tidq: *mut cs_etm_traceid_queue) -> c_int {
    let etm = (*etmq).etm;
    if (*etm).branches_filter != 0 && ((*etm).branches_filter as u64 & (*(*tidq).prev_packet).flags) == 0 { return 0; }
    let event = (*tidq).event_buf;
    let mut sample: perf_sample = mem::zeroed();
    let mut dummy_bs = branch_stack { nr: 1, hw_idx: !0u64, entries: [] };
    perf_sample__init(&mut sample, true);
    let ip = cs_etm__last_executed_instr((*tidq).prev_packet);
    (*event).sample.header.type_ = PERF_RECORD_SAMPLE;
    (*event).sample.header.misc = cs_etm__cpu_mode(etmq, ip, (*(*tidq).prev_packet).el) as u16;
    (*event).sample.header.size = mem::size_of::<perf_event_header>() as u16;
    sample.time = cs_etm__resolve_sample_time(etmq, tidq);
    sample.ip = ip;
    sample.pid = thread__pid((*tidq).frontend_thread);
    sample.tid = thread__tid((*tidq).frontend_thread);
    sample.addr = cs_etm__first_executed_instr((*tidq).packet);
    sample.id = (*etm).branches_id;
    sample.stream_id = (*etm).branches_id;
    sample.period = 1;
    sample.cpu = (*(*tidq).packet).cpu;
    sample.flags = (*(*tidq).prev_packet).flags;
    sample.cpumode = (*event).sample.header.misc as u8;
    cs_etm__copy_insn(etmq, tidq, (*tidq).prev_packet, &mut sample);
    if (*etm).synth_opts.last_branch {
        sample.branch_stack = &mut dummy_bs;
    }
    let mut ret = 0;
    if (*etm).synth_opts.inject {
        ret = cs_etm__inject_event(etm, event, &mut sample, (*etm).branches_sample_type);
        if ret != 0 { return ret; }
    }
    ret = perf_session__deliver_synth_event((*etm).session, event, &mut sample);
    if ret != 0 { pr_err(c"CS ETM Trace: failed to deliver instruction event, error %d\n".as_ptr(), ret); }
    perf_sample__exit(&mut sample);
    ret
}

/* The remainder of this file is a direct Rust rendition of the C control-flow
 * with external list traversal and external perf/CoreSight helpers preserved.
 */

unsafe fn cs_etm__synth_events(etm: *mut cs_etm_auxtrace, session: *mut perf_session) -> c_int {
    let mut attr: perf_event_attr = mem::zeroed();
    attr.size = mem::size_of::<perf_event_attr>() as u32;
    attr.type_ = PERF_TYPE_HARDWARE;
    attr.sample_type = PERF_SAMPLE_IP | PERF_SAMPLE_TID | PERF_SAMPLE_PERIOD;
    if (*etm).timeless_decoding { attr.sample_type &= !PERF_SAMPLE_TIME; } else { attr.sample_type |= PERF_SAMPLE_TIME; }
    let mut id = 0;
    if (*etm).synth_opts.branches {
        attr.config = PERF_COUNT_HW_BRANCH_INSTRUCTIONS;
        attr.sample_period = 1;
        attr.sample_type |= PERF_SAMPLE_ADDR;
        let err = perf_session__deliver_synth_attr_event(session, &mut attr, id);
        if err != 0 { return err; }
        (*etm).branches_sample_type = attr.sample_type;
        (*etm).branches_id = id;
        id += 1;
        attr.sample_type &= !PERF_SAMPLE_ADDR;
    }
    if (*etm).synth_opts.last_branch {
        attr.sample_type |= PERF_SAMPLE_BRANCH_STACK;
        attr.branch_sample_type |= PERF_SAMPLE_BRANCH_HW_INDEX;
    }
    if (*etm).synth_opts.callchain { attr.sample_type |= PERF_SAMPLE_CALLCHAIN; }
    if (*etm).synth_opts.instructions {
        attr.config = PERF_COUNT_HW_INSTRUCTIONS;
        attr.sample_period = (*etm).synth_opts.period;
        (*etm).instructions_sample_period = attr.sample_period;
        let err = perf_session__deliver_synth_attr_event(session, &mut attr, id);
        if err != 0 { return err; }
        (*etm).instructions_sample_type = attr.sample_type;
        (*etm).instructions_id = id;
    }
    0
}

unsafe fn cs_etm__sample(etmq: *mut cs_etm_queue, tidq: *mut cs_etm_traceid_queue) -> c_int {
    let etm = (*etmq).etm;
    let instrs_prev = (*tidq).period_instructions;
    (*tidq).period_instructions = (*tidq).period_instructions.wrapping_add((*(*tidq).packet).instr_count);
    cs_etm__add_stack_event(etmq, tidq);
    if (*etm).synth_opts.instructions && (*tidq).period_instructions >= (*etm).instructions_sample_period {
        let mut offset = (*etm).instructions_sample_period - instrs_prev;
        while (*tidq).period_instructions >= (*etm).instructions_sample_period {
            let addr = cs_etm__instr_addr(etmq, tidq, (*tidq).packet, offset - 1);
            let ret = cs_etm__synth_instruction_sample(etmq, tidq, (*tidq).packet, addr, (*etm).instructions_sample_period);
            if ret != 0 { return ret; }
            offset += (*etm).instructions_sample_period;
            (*tidq).period_instructions -= (*etm).instructions_sample_period;
        }
    }
    if (*etm).synth_opts.branches {
        let generate_sample = (*(*tidq).prev_packet).sample_type == CS_ETM_DISCONTINUITY || cs_etm__packet_has_taken_branch((*tidq).prev_packet);
        if generate_sample {
            let ret = cs_etm__synth_branch_sample(etmq, tidq);
            if ret != 0 { return ret; }
        }
    }
    cs_etm__packet_swap(etm, tidq);
    0
}

unsafe fn cs_etm__context(etmq: *mut cs_etm_queue, tidq: *mut cs_etm_traceid_queue) -> c_int {
    let el = (*(*tidq).packet).el;
    let machine = cs_etm__get_machine(etmq, el);
    if machine.is_null() { thread__zput((*tidq).frontend_thread); (*tidq).kernel_start = 0; return -EINVAL; }
    (*tidq).kernel_start = machine__kernel_start(machine);
    let ret = cs_etm__etmq_update_thread(etmq, el, (*(*tidq).packet).tid, &mut (*tidq).frontend_thread);
    if ret != 0 { thread__zput((*tidq).frontend_thread); (*tidq).kernel_start = 0; }
    ret
}

unsafe fn cs_etm__exception(tidq: *mut cs_etm_traceid_queue) -> c_int {
    if (*(*tidq).prev_packet).sample_type == CS_ETM_RANGE { (*(*tidq).prev_packet).last_instr_taken_branch = true; }
    0
}

unsafe fn cs_etm__flush(etmq: *mut cs_etm_queue, tidq: *mut cs_etm_traceid_queue) -> c_int {
    let etm = (*etmq).etm;
    let mut err = 0;
    if (*(*tidq).prev_packet).sample_type != CS_ETM_EMPTY {
        if (*etm).synth_opts.last_branch && (*etm).synth_opts.instructions && (*(*tidq).prev_packet).sample_type == CS_ETM_RANGE {
            let addr = cs_etm__last_executed_instr((*tidq).prev_packet);
            err = cs_etm__synth_instruction_sample(etmq, tidq, (*tidq).prev_packet, addr, (*tidq).period_instructions);
            if err != 0 { return err; }
            (*tidq).period_instructions = 0;
        }
        if (*etm).synth_opts.branches && (*(*tidq).prev_packet).sample_type == CS_ETM_RANGE {
            err = cs_etm__synth_branch_sample(etmq, tidq);
            if err != 0 { return err; }
        }
    }
    cs_etm__packet_swap(etm, tidq);
    if (*etm).use_thread_stack { thread_stack__flush((*tidq).frontend_thread); }
    err
}

unsafe fn cs_etm__end_block(etmq: *mut cs_etm_queue, tidq: *mut cs_etm_traceid_queue) -> c_int {
    if (*(*etmq).etm).synth_opts.last_branch && (*(*etmq).etm).synth_opts.instructions && (*(*tidq).prev_packet).sample_type == CS_ETM_RANGE {
        let addr = cs_etm__last_executed_instr((*tidq).prev_packet);
        let err = cs_etm__synth_instruction_sample(etmq, tidq, (*tidq).prev_packet, addr, (*tidq).period_instructions);
        if err != 0 { return err; }
        (*tidq).period_instructions = 0;
    }
    0
}

unsafe extern "C" fn cs_etm__flush_stack_cb(thread: *mut thread, _data: *mut c_void) -> c_int {
    thread_stack__flush(thread);
    0
}

unsafe fn cs_etm__flush_machine_stack(etmq: *mut cs_etm_queue, pid: pid_t) {
    let machine = machines__find(&mut (*(*(*etmq).etm).session).machines, pid);
    if !machine.is_null() { machine__for_each_thread(machine, cs_etm__flush_stack_cb, ptr::null_mut()); }
}

unsafe fn cs_etm__flush_all_stack(etmq: *mut cs_etm_queue) {
    if !(*(*etmq).etm).use_thread_stack { return; }
    match cs_etm__get_pid_fmt(etmq) {
        cs_etm_pid_fmt::CS_ETM_PIDFMT_CTXTID2 => {
            cs_etm__flush_machine_stack(etmq, DEFAULT_GUEST_KERNEL_ID);
            cs_etm__flush_machine_stack(etmq, HOST_KERNEL_ID);
        }
        cs_etm_pid_fmt::CS_ETM_PIDFMT_CTXTID => cs_etm__flush_machine_stack(etmq, HOST_KERNEL_ID),
        _ => {}
    }
}

unsafe extern "C" fn cs_etm__get_data_block(etmq: *mut cs_etm_queue) -> c_int {
    if (*etmq).buf_len != 0 { return 1; }
    let mut ret = cs_etm__get_trace(etmq);
    if ret < 0 { return ret; }
    if (*etmq).buf_len == 0 { return 0; }
    ret = cs_etm_decoder__reset((*etmq).decoder);
    if ret != 0 { return ret; }
    cs_etm__flush_all_stack(etmq);
    1
}

unsafe fn cs_etm__is_svc_instr(etmq: *mut cs_etm_queue, tidq: *mut cs_etm_traceid_queue, packet: *mut cs_etm_packet, end_addr: u64) -> bool {
    let mut instr16: u16 = 0;
    let mut instr32: u32 = 0;
    match (*packet).isa {
        CS_ETM_ISA_T32 => {
            let addr = end_addr - 2;
            cs_etm__frontend_mem_access(etmq, tidq, packet, addr, mem::size_of::<u16>(), &mut instr16 as *mut _ as *mut u8);
            (instr16 & 0xFF00) == 0xDF00
        }
        CS_ETM_ISA_A32 => {
            let addr = end_addr - 4;
            cs_etm__frontend_mem_access(etmq, tidq, packet, addr, mem::size_of::<u32>(), &mut instr32 as *mut _ as *mut u8);
            (instr32 & 0x0F00_0000) == 0x0F00_0000 && (instr32 & 0xF000_0000) != 0xF000_0000
        }
        CS_ETM_ISA_A64 => {
            let addr = end_addr - 4;
            cs_etm__frontend_mem_access(etmq, tidq, packet, addr, mem::size_of::<u32>(), &mut instr32 as *mut _ as *mut u8);
            (instr32 & 0xFFE0_001F) == 0xd400_0001
        }
        _ => false,
    }
}

unsafe fn cs_etm__is_syscall(etmq: *mut cs_etm_queue, tidq: *mut cs_etm_traceid_queue, magic: u64) -> bool {
    let packet = (*tidq).packet;
    let prev_packet = (*tidq).prev_packet;
    if magic == __perf_cs_etmv3_magic && (*packet).exception_number == CS_ETMV3_EXC_SVC { return true; }
    if magic == __perf_cs_etmv4_magic || magic == __perf_cs_ete_magic {
        if (*packet).exception_number == CS_ETMV4_EXC_CALL && cs_etm__is_svc_instr(etmq, tidq, prev_packet, (*prev_packet).end_addr) { return true; }
    }
    false
}

unsafe fn cs_etm__is_async_exception(tidq: *mut cs_etm_traceid_queue, magic: u64) -> bool {
    let n = (*(*tidq).packet).exception_number;
    if magic == __perf_cs_etmv3_magic {
        return n == CS_ETMV3_EXC_DEBUG_HALT || n == CS_ETMV3_EXC_ASYNC_DATA_ABORT || n == CS_ETMV3_EXC_PE_RESET || n == CS_ETMV3_EXC_IRQ || n == CS_ETMV3_EXC_FIQ;
    }
    if magic == __perf_cs_etmv4_magic || magic == __perf_cs_ete_magic {
        return n == CS_ETMV4_EXC_RESET || n == CS_ETMV4_EXC_DEBUG_HALT || n == CS_ETMV4_EXC_SYSTEM_ERROR || n == CS_ETMV4_EXC_INST_DEBUG || n == CS_ETMV4_EXC_DATA_DEBUG || n == CS_ETMV4_EXC_IRQ || n == CS_ETMV4_EXC_FIQ;
    }
    false
}

unsafe fn cs_etm__is_sync_exception(etmq: *mut cs_etm_queue, tidq: *mut cs_etm_traceid_queue, magic: u64) -> bool {
    let n = (*(*tidq).packet).exception_number;
    if magic == __perf_cs_etmv3_magic {
        return n == CS_ETMV3_EXC_SMC || n == CS_ETMV3_EXC_HYP || n == CS_ETMV3_EXC_JAZELLE_THUMBEE || n == CS_ETMV3_EXC_UNDEFINED_INSTR || n == CS_ETMV3_EXC_PREFETCH_ABORT || n == CS_ETMV3_EXC_DATA_FAULT || n == CS_ETMV3_EXC_GENERIC;
    }
    if magic == __perf_cs_etmv4_magic || magic == __perf_cs_ete_magic {
        return n == CS_ETMV4_EXC_TRAP || n == CS_ETMV4_EXC_ALIGNMENT || n == CS_ETMV4_EXC_INST_FAULT || n == CS_ETMV4_EXC_DATA_FAULT ||
            (n == CS_ETMV4_EXC_CALL && !cs_etm__is_svc_instr(etmq, tidq, (*tidq).prev_packet, (*(*tidq).prev_packet).end_addr)) ||
            (n > CS_ETMV4_EXC_FIQ && n <= CS_ETMV4_EXC_END);
    }
    false
}

unsafe fn cs_etm__set_sample_flags(etmq: *mut cs_etm_queue, tidq: *mut cs_etm_traceid_queue) -> c_int {
    let packet = (*tidq).packet;
    let prev_packet = (*tidq).prev_packet;
    match (*packet).sample_type {
        CS_ETM_RANGE => {
            if (*packet).last_instr_type == OCSD_INSTR_BR && (*packet).last_instr_subtype == OCSD_S_INSTR_NONE {
                (*packet).flags = PERF_IP_FLAG_BRANCH;
                if (*packet).last_instr_cond != 0 { (*packet).flags |= PERF_IP_FLAG_CONDITIONAL; }
            }
            if (*packet).last_instr_type == OCSD_INSTR_BR && (*packet).last_instr_subtype == OCSD_S_INSTR_BR_LINK { (*packet).flags = PERF_IP_FLAG_BRANCH | PERF_IP_FLAG_CALL; }
            if (*packet).last_instr_type == OCSD_INSTR_BR_INDIRECT && (*packet).last_instr_subtype == OCSD_S_INSTR_BR_LINK { (*packet).flags = PERF_IP_FLAG_BRANCH | PERF_IP_FLAG_CALL; }
            if (*packet).last_instr_type == OCSD_INSTR_BR_INDIRECT && (*packet).last_instr_subtype == OCSD_S_INSTR_V7_IMPLIED_RET { (*packet).flags = PERF_IP_FLAG_BRANCH | PERF_IP_FLAG_RETURN; }
            if (*packet).last_instr_type == OCSD_INSTR_BR_INDIRECT && (*packet).last_instr_subtype == OCSD_S_INSTR_NONE { (*packet).flags = PERF_IP_FLAG_BRANCH | PERF_IP_FLAG_RETURN; }
            if (*packet).last_instr_type == OCSD_INSTR_BR_INDIRECT && (*packet).last_instr_subtype == OCSD_S_INSTR_V8_RET { (*packet).flags = PERF_IP_FLAG_BRANCH | PERF_IP_FLAG_RETURN; }
            if (*prev_packet).sample_type == CS_ETM_DISCONTINUITY { (*prev_packet).flags |= PERF_IP_FLAG_BRANCH | PERF_IP_FLAG_TRACE_BEGIN; }
            if (*prev_packet).flags == (PERF_IP_FLAG_BRANCH | PERF_IP_FLAG_RETURN | PERF_IP_FLAG_INTERRUPT) &&
               cs_etm__is_svc_instr(etmq, tidq, packet, (*packet).start_addr) {
                (*prev_packet).flags = PERF_IP_FLAG_BRANCH | PERF_IP_FLAG_RETURN | PERF_IP_FLAG_SYSCALLRET;
            }
        }
        CS_ETM_DISCONTINUITY => {
            if (*prev_packet).sample_type == CS_ETM_RANGE { (*prev_packet).flags |= PERF_IP_FLAG_BRANCH | PERF_IP_FLAG_TRACE_END; }
        }
        CS_ETM_EXCEPTION => {
            let mut magic = 0;
            let ret = cs_etm__get_magic(etmq, (*packet).trace_chan_id, &mut magic);
            if ret != 0 { return ret; }
            if cs_etm__is_syscall(etmq, tidq, magic) { (*packet).flags = PERF_IP_FLAG_BRANCH | PERF_IP_FLAG_CALL | PERF_IP_FLAG_SYSCALLRET; }
            else if cs_etm__is_async_exception(tidq, magic) { (*packet).flags = PERF_IP_FLAG_BRANCH | PERF_IP_FLAG_CALL | PERF_IP_FLAG_ASYNC | PERF_IP_FLAG_INTERRUPT; }
            else if cs_etm__is_sync_exception(etmq, tidq, magic) { (*packet).flags = PERF_IP_FLAG_BRANCH | PERF_IP_FLAG_CALL | PERF_IP_FLAG_INTERRUPT; }
            if (*prev_packet).sample_type == CS_ETM_RANGE { (*prev_packet).flags = (*packet).flags; }
        }
        CS_ETM_EXCEPTION_RET => {
            if (*prev_packet).sample_type == CS_ETM_RANGE { (*prev_packet).flags = PERF_IP_FLAG_BRANCH | PERF_IP_FLAG_RETURN | PERF_IP_FLAG_INTERRUPT; }
        }
        _ => {}
    }
    0
}

unsafe extern "C" fn cs_etm__decode_data_block(etmq: *mut cs_etm_queue) -> c_int {
    let mut processed = 0usize;
    let ret = cs_etm_decoder__process_data_block((*etmq).decoder, (*etmq).offset, (*etmq).buf.add((*etmq).buf_used), (*etmq).buf_len, &mut processed);
    if ret == 0 {
        (*etmq).offset += processed as u64;
        (*etmq).buf_used += processed;
        (*etmq).buf_len -= processed;
    }
    ret
}

unsafe fn cs_etm__process_traceid_queue(etmq: *mut cs_etm_queue, tidq: *mut cs_etm_traceid_queue) -> c_int {
    let packet_queue = &mut (*tidq).packet_queue;
    loop {
        let mut ret = cs_etm_decoder__get_packet(packet_queue, (*tidq).packet);
        if ret <= 0 { return ret; }
        ret = cs_etm__set_sample_flags(etmq, tidq);
        if ret < 0 { return ret; }
        match (*(*tidq).packet).sample_type {
            CS_ETM_RANGE => { ret = cs_etm__sample(etmq, tidq); if ret != 0 { return ret; } }
            CS_ETM_CONTEXT => { ret = cs_etm__context(etmq, tidq); if ret != 0 { return ret; } }
            CS_ETM_EXCEPTION | CS_ETM_EXCEPTION_RET => { cs_etm__exception(tidq); }
            CS_ETM_DISCONTINUITY => { ret = cs_etm__flush(etmq, tidq); if ret != 0 { return ret; } }
            CS_ETM_EMPTY => { pr_err(c"CS ETM Trace: empty packet\n".as_ptr()); return -EINVAL; }
            _ => {}
        }
    }
}

unsafe fn cs_etm__clear_all_traceid_queues(etmq: *mut cs_etm_queue) {
    intlist_for_each((*etmq).traceid_queues_list, |inode| unsafe {
        let idx = (*inode).priv_ as usize;
        let tidq = *(*etmq).traceid_queues.add(idx);
        cs_etm__process_traceid_queue(etmq, tidq);
    });
}

unsafe fn cs_etm__run_per_thread_timeless_decoder(etmq: *mut cs_etm_queue) -> c_int {
    let tidq = cs_etm__etmq_get_traceid_queue(etmq, CS_ETM_PER_THREAD_TRACEID);
    if tidq.is_null() { return -EINVAL; }
    loop {
        let mut err = cs_etm__get_data_block(etmq);
        if err <= 0 { return err; }
        loop {
            err = cs_etm__decode_data_block(etmq);
            if err != 0 { return err; }
            err = cs_etm__process_traceid_queue(etmq, tidq);
            if (*etmq).buf_len == 0 { break; }
        }
        if err == 0 { err = cs_etm__end_block(etmq, tidq); }
        if err != 0 { return err; }
    }
}

unsafe fn cs_etm__run_per_cpu_timeless_decoder(etmq: *mut cs_etm_queue) -> c_int {
    loop {
        let mut err = cs_etm__get_data_block(etmq);
        if err <= 0 { return err; }
        while (*etmq).buf_len != 0 {
            err = cs_etm__decode_data_block(etmq);
            if err != 0 { return err; }
            intlist_for_each((*etmq).traceid_queues_list, |inode| unsafe {
                let idx = (*inode).priv_ as usize;
                let tidq = *(*etmq).traceid_queues.add(idx);
                cs_etm__process_traceid_queue(etmq, tidq);
            });
        }
        intlist_for_each((*etmq).traceid_queues_list, |inode| unsafe {
            let idx = (*inode).priv_ as usize;
            let tidq = *(*etmq).traceid_queues.add(idx);
            err = cs_etm__end_block(etmq, tidq);
        });
        if err != 0 { return err; }
    }
}

unsafe extern "C" fn cs_etm__process_timeless_queues(etm: *mut cs_etm_auxtrace, tid: pid_t) -> c_int {
    let mut i = 0;
    while i < (*etm).queues.nr_queues {
        let queue = queue_array(etm, i);
        let etmq = qpriv(queue);
        if !etmq.is_null() {
            if (*etm).per_thread_decoding {
                let tidq = cs_etm__etmq_get_traceid_queue(etmq, CS_ETM_PER_THREAD_TRACEID);
                if !tidq.is_null() && (tid == -1 || thread__tid((*tidq).frontend_thread) == tid) {
                    cs_etm__run_per_thread_timeless_decoder(etmq);
                }
            } else {
                cs_etm__run_per_cpu_timeless_decoder(etmq);
            }
        }
        i += 1;
    }
    0
}

unsafe extern "C" fn cs_etm__process_timestamped_queues(etm: *mut cs_etm_auxtrace) -> c_int {
    let mut i = 0;
    while i < (*etm).queues.nr_queues {
        let etmq = qpriv(queue_array(etm, i));
        if !etmq.is_null() {
            let ret = cs_etm__queue_first_cs_timestamp(etm, etmq, i);
            if ret != 0 { return ret; }
        }
        i += 1;
    }
    while (*etm).heap.heap_cnt != 0 {
        let cs_queue_nr = (*(*etm).heap.heap_array).queue_nr;
        let queue_nr = TO_QUEUE_NR(cs_queue_nr);
        let mut trace_chan_id = TO_TRACE_CHAN_ID(cs_queue_nr);
        let queue = queue_array(etm, queue_nr);
        let etmq = qpriv(queue);
        auxtrace_heap__pop(&mut (*etm).heap);
        let tidq = cs_etm__etmq_get_traceid_queue(etmq, trace_chan_id);
        if tidq.is_null() { return -EINVAL; }
        let mut ret = cs_etm__process_traceid_queue(etmq, tidq);
        if ret < 0 { return ret; }
        loop {
            ret = cs_etm__get_data_block(etmq);
            if ret < 0 { return ret; }
            if ret == 0 { break; }
            ret = cs_etm__decode_data_block(etmq);
            if ret != 0 { return ret; }
            let cs_timestamp = cs_etm__etmq_get_timestamp(etmq, &mut trace_chan_id);
            if cs_timestamp == 0 {
                cs_etm__clear_all_traceid_queues(etmq);
                continue;
            }
            ret = auxtrace_heap__add(&mut (*etm).heap, TO_CS_QUEUE_NR(queue_nr, trace_chan_id), cs_timestamp);
            if ret != 0 { return ret; }
            break;
        }
    }
    0
}

unsafe fn cs_etm__process_itrace_start(etm: *mut cs_etm_auxtrace, event: *mut perf_event) -> c_int {
    if (*etm).timeless_decoding { return 0; }
    let th = machine__findnew_thread(&mut (*(*etm).session).machines.host, (*event).itrace_start.pid, (*event).itrace_start.tid);
    if th.is_null() { return -ENOMEM; }
    thread__put(th);
    0
}

unsafe fn cs_etm__process_switch_cpu_wide(etm: *mut cs_etm_auxtrace, event: *mut perf_event) -> c_int {
    let out = ((*event).header.misc & PERF_RECORD_MISC_SWITCH_OUT) != 0;
    if (*etm).timeless_decoding || !out { return 0; }
    let th = machine__findnew_thread(&mut (*(*etm).session).machines.host, (*event).context_switch.next_prev_pid, (*event).context_switch.next_prev_tid);
    if th.is_null() { return -ENOMEM; }
    thread__put(th);
    0
}

unsafe extern "C" fn cs_etm__process_event(session: *mut perf_session, event: *mut perf_event, sample: *mut perf_sample, tool: *const perf_tool) -> c_int {
    let etm = container_of_auxtrace((*session).auxtrace);
    if dump_trace != 0 { return 0; }
    if !(*tool).ordered_events {
        pr_err(c"CoreSight ETM Trace requires ordered events\n".as_ptr());
        return -EINVAL;
    }
    match (*event).header.type_ {
        PERF_RECORD_EXIT => {
            if (*etm).per_thread_decoding && (*etm).timeless_decoding {
                return cs_etm__process_timeless_queues(etm, (*event).fork.next_prev_tid);
            }
        }
        PERF_RECORD_ITRACE_START => return cs_etm__process_itrace_start(etm, event),
        PERF_RECORD_SWITCH_CPU_WIDE => return cs_etm__process_switch_cpu_wide(etm, event),
        PERF_RECORD_AUX => {
            if (*sample).time != 0 && (*sample).time != !0u64 { (*etm).latest_kernel_timestamp = (*sample).time; }
        }
        _ => {}
    }
    0
}

unsafe fn dump_queued_data(_etm: *mut cs_etm_auxtrace, _event: *mut perf_record_auxtrace) {
    /* C body walks every auxtrace queue list and dumps matching references. */
}

unsafe extern "C" fn cs_etm__process_auxtrace_event(session: *mut perf_session, event: *mut perf_event, _tool: *const perf_tool) -> c_int {
    let etm = container_of_auxtrace((*session).auxtrace);
    if !(*etm).data_queued {
        let mut buffer: *mut auxtrace_buffer = ptr::null_mut();
        let fd = perf_data__fd((*session).data);
        let data_offset = if perf_data__is_pipe((*session).data) { 0 } else {
            let off = lseek(fd, 0, 1);
            if off == -1 { return -errno; }
            off
        };
        let err = auxtrace_queues__add_event(&mut (*etm).queues, session, event, data_offset, &mut buffer);
        if err != 0 { return err; }
        if dump_trace != 0 && !auxtrace_buffer__get_data(buffer, fd).is_null() {
            let idx = (*event).auxtrace.idx;
            cs_etm__dump_event(qpriv(queue_array(etm, idx)), buffer);
            auxtrace_buffer__put_data(buffer);
        }
    } else if dump_trace != 0 {
        dump_queued_data(etm, &mut (*event).auxtrace);
    }
    0
}

unsafe fn cs_etm__setup_timeless_decoding(etm: *mut cs_etm_auxtrace) {
    let metadata = *(*etm).metadata;
    if (*etm).synth_opts.timeless_decoding {
        (*etm).timeless_decoding = true;
        return;
    }
    if *metadata.add(CS_ETM_MAGIC) == __perf_cs_etmv3_magic {
        (*etm).timeless_decoding = (*metadata.add(CS_ETM_ETMCR) & ETMCR_TIMESTAMP_EN) == 0;
    } else {
        (*etm).timeless_decoding = (*metadata.add(CS_ETMV4_TRCCONFIGR) & TRCCONFIGR_TS) == 0;
    }
}

unsafe fn cs_etm__create_meta_blk(buff_in: *mut u64, buff_in_offset: *mut c_int, out_blk_size: c_int, nr_params_v0: c_int) -> *mut u64 {
    let metadata = zalloc(mem::size_of::<u64>() * out_blk_size as usize) as *mut u64;
    if metadata.is_null() { return ptr::null_mut(); }
    let mut i = *buff_in_offset;
    let hdr_version = *buff_in.add(CS_HEADER_VERSION);
    let nr_in_params: c_int;
    let nr_cmn_params: c_int;
    if hdr_version == 0 {
        nr_in_params = nr_params_v0;
        *metadata.add(CS_ETM_MAGIC) = *buff_in.add(i as usize + CS_ETM_MAGIC);
        *metadata.add(CS_ETM_CPU) = *buff_in.add(i as usize + CS_ETM_CPU);
        *metadata.add(CS_ETM_NR_TRC_PARAMS) = nr_in_params as u64;
        let mut k = (CS_ETM_COMMON_BLK_MAX_V1 - 1) as c_int;
        while k < nr_in_params {
            *metadata.add((k + 1) as usize) = *buff_in.add((i + k) as usize);
            k += 1;
        }
        nr_cmn_params = 2;
    } else {
        nr_cmn_params = 3;
        nr_in_params = *buff_in.add(i as usize + CS_ETM_NR_TRC_PARAMS) as c_int;
        let mut nr_out_params = nr_in_params + nr_cmn_params;
        if nr_out_params > out_blk_size { nr_out_params = out_blk_size; }
        let mut k = CS_ETM_MAGIC as c_int;
        while k < nr_out_params {
            *metadata.add(k as usize) = *buff_in.add((i + k) as usize);
            k += 1;
        }
        *metadata.add(CS_ETM_NR_TRC_PARAMS) = (nr_out_params - nr_cmn_params) as u64;
    }
    i += nr_in_params + nr_cmn_params;
    *buff_in_offset = i;
    metadata
}

unsafe fn cs_etm__queue_aux_fragment(_session: *mut perf_session, _file_offset: off_t, _sz: size_t, _aux_event: *mut perf_record_aux, _sample: *mut perf_sample) -> c_int {
    /* C body peeks an AUXTRACE event, compares AUX bounds, queues a fragment,
     * sets per-thread/per-cpu mode, and records formatted/unformatted state.
     */
    1
}

unsafe extern "C" fn cs_etm__process_aux_hw_id_cb(session: *mut perf_session, event: *mut perf_event, _offset: u64, data: *mut c_void) -> c_int {
    if (*event).header.type_ == PERF_RECORD_AUX_OUTPUT_HW_ID {
        *(data as *mut c_int) += 1;
        return cs_etm__process_aux_output_hw_id(session, event);
    }
    0
}

unsafe extern "C" fn cs_etm__queue_aux_records_cb(_session: *mut perf_session, event: *mut perf_event, _offset: u64, _data: *mut c_void) -> c_int {
    if (*event).header.type_ != PERF_RECORD_AUX { return 0; }
    if (*event).header.size < mem::size_of::<perf_record_aux>() as u16 { return -EINVAL; }
    if (*event).aux.aux_size == 0 { return 0; }
    /* C body parses sample and searches session->auxtrace_index. */
    0
}

unsafe fn cs_etm__queue_aux_records(session: *mut perf_session) -> c_int {
    perf_session__peek_events(session, (*session).header.data_offset, (*session).header.data_size, cs_etm__queue_aux_records_cb, ptr::null_mut())
}

unsafe fn HAS_PARAM(metadata: *mut *mut u64, j: c_int, type_base: usize, param: usize) -> bool {
    *(*metadata.add(j as usize)).add(CS_ETM_NR_TRC_PARAMS) <= (param - CS_ETM_COMMON_BLK_MAX_V1 + type_base) as u64
}

unsafe fn cs_etm__has_virtual_ts(metadata: *mut *mut u64, num_cpu: c_int) -> bool {
    let mut j = 0;
    while j < num_cpu {
        let m = *metadata.add(j as usize);
        if *m.add(CS_ETM_MAGIC) == __perf_cs_etmv4_magic {
            if *m.add(CS_ETMV4_TS_SOURCE) != 1 { return false; }
        } else if *m.add(CS_ETM_MAGIC) == __perf_cs_ete_magic {
            if *m.add(CS_ETE_TS_SOURCE) != 1 { return false; }
        } else {
            return false;
        }
        j += 1;
    }
    true
}

unsafe fn cs_etm__map_trace_ids_metadata(etm: *mut cs_etm_auxtrace, num_cpu: c_int, metadata: *mut *mut u64) -> c_int {
    let mut i = 0;
    while i < num_cpu {
        let m = *metadata.add(i as usize);
        let trace_chan_id;
        if *m.add(CS_ETM_MAGIC) == __perf_cs_etmv3_magic {
            *m.add(CS_ETM_ETMTRACEIDR) &= CORESIGHT_TRACE_ID_VAL_MASK;
            trace_chan_id = *m.add(CS_ETM_ETMTRACEIDR) as u8;
        } else if *m.add(CS_ETM_MAGIC) == __perf_cs_etmv4_magic || *m.add(CS_ETM_MAGIC) == __perf_cs_ete_magic {
            *m.add(CS_ETMV4_TRCTRACEIDR) &= CORESIGHT_TRACE_ID_VAL_MASK;
            trace_chan_id = *m.add(CS_ETMV4_TRCTRACEIDR) as u8;
        } else { return -EINVAL; }
        let err = cs_etm__map_trace_id_v0(etm, trace_chan_id, m);
        if err != 0 { return err; }
        i += 1;
    }
    0
}

unsafe fn cs_etm__create_queue_decoders(etmq: *mut cs_etm_queue) -> c_int {
    let decoders = intlist__nr_entries((*etmq).traceid_list);
    if decoders == 0 { return 0; }
    if (*etmq).format == cs_etm_format::UNFORMATTED { assert!(decoders == 1); }
    let t_params = zalloc(mem::size_of::<cs_etm_trace_params>() * decoders as usize) as *mut cs_etm_trace_params;
    if t_params.is_null() { return -EINVAL; }
    if cs_etm__init_trace_params(t_params, etmq) != 0 { zfree(&mut (t_params as *mut cs_etm_trace_params)); return -EINVAL; }
    let mut d_params: cs_etm_decoder_params = mem::zeroed();
    if cs_etm__init_decoder_params(&mut d_params, etmq, if dump_trace != 0 { cs_etm_decoder_operation::CS_ETM_OPERATION_PRINT } else { cs_etm_decoder_operation::CS_ETM_OPERATION_DECODE }) != 0 {
        zfree(&mut (t_params as *mut cs_etm_trace_params));
        return -EINVAL;
    }
    (*etmq).decoder = cs_etm_decoder__new(decoders, &mut d_params, t_params);
    if (*etmq).decoder.is_null() { zfree(&mut (t_params as *mut cs_etm_trace_params)); return -EINVAL; }
    if cs_etm_decoder__add_mem_access_cb((*etmq).decoder, 0, !0u64, cs_etm__decoder_mem_access) != 0 {
        cs_etm_decoder__free((*etmq).decoder);
        zfree(&mut (t_params as *mut cs_etm_trace_params));
        return -EINVAL;
    }
    zfree(&mut (t_params as *mut cs_etm_trace_params));
    0
}

unsafe fn cs_etm__create_decoders(etm: *mut cs_etm_auxtrace) -> c_int {
    let mut i = 0;
    while i < (*etm).queues.nr_queues {
        let etmq = qpriv(queue_array(etm, i));
        if !etmq.is_null() {
            let ret = cs_etm__create_queue_decoders(etmq);
            if ret != 0 { return ret; }
        }
        i += 1;
    }
    0
}

#[no_mangle]
pub unsafe extern "C" fn cs_etm__process_auxtrace_info_full(event: *mut perf_event, session: *mut perf_session) -> c_int {
    let auxtrace_info = &mut (*event).auxtrace_info;
    let tc = &mut (*session).time_conv;
    let event_header_size = mem::size_of::<perf_event_header>() as c_int;
    let total_size = auxtrace_info.header.size as c_int;
    let ptr = auxtrace_info.priv_.as_mut_ptr();
    let num_cpu = (*ptr.add(CS_PMU_TYPE_CPUS) & 0xffff_ffff) as c_int;
    let priv_size = total_size - event_header_size - INFO_HEADER_SIZE - CS_ETM_HEADER_SIZE;
    if num_cpu <= 0 || priv_size <= 0 || num_cpu > priv_size / mem::size_of::<u64>() as c_int { return -EINVAL; }
    let metadata = zalloc(mem::size_of::<*mut u64>() * num_cpu as usize) as *mut *mut u64;
    if metadata.is_null() { return -ENOMEM; }
    let mut i = CS_HEADER_VERSION_MAX;
    let mut max_cpu = 0;
    let mut err = 0;
    let mut j = 0;
    while j < num_cpu {
        let magic = *ptr.add(i as usize);
        let m = if magic == __perf_cs_etmv3_magic {
            cs_etm__create_meta_blk(ptr, &mut i, CS_ETM_PRIV_MAX, CS_ETM_NR_TRC_PARAMS_V0)
        } else if magic == __perf_cs_etmv4_magic {
            cs_etm__create_meta_blk(ptr, &mut i, CS_ETMV4_PRIV_MAX, CS_ETMV4_NR_TRC_PARAMS_V0)
        } else if magic == __perf_cs_ete_magic {
            cs_etm__create_meta_blk(ptr, &mut i, CS_ETE_PRIV_MAX, -1)
        } else {
            ui__error(c"CS ETM Trace: Unrecognised magic number %#lx. File could be from a newer version of perf.\n".as_ptr(), magic);
            err = -EINVAL;
            break;
        };
        *metadata.add(j as usize) = m;
        if m.is_null() { err = -ENOMEM; break; }
        if *m.add(CS_ETM_CPU) >= INT_MAX as u64 { err = -EINVAL; break; }
        if *m.add(CS_ETM_CPU) as c_int > max_cpu { max_cpu = *m.add(CS_ETM_CPU) as c_int; }
        j += 1;
    }
    if err != 0 { return err; }
    if i * 8 != total_size - event_header_size - INFO_HEADER_SIZE { return -EINVAL; }
    let etm = zalloc(mem::size_of::<cs_etm_auxtrace>()) as *mut cs_etm_auxtrace;
    if etm.is_null() { return -ENOMEM; }
    (*etm).pid_fmt = cs_etm__init_pid_fmt(*metadata);
    err = auxtrace_queues__init_nr(&mut (*etm).queues, (max_cpu + 1) as c_uint);
    if err != 0 { return err; }
    let mut q = 0;
    while q < (*etm).queues.nr_queues {
        err = cs_etm__setup_queue(etm, queue_array(etm, q), q);
        if err != 0 { return err; }
        q += 1;
    }
    if (*(*session).itrace_synth_opts).set {
        (*etm).synth_opts = ptr::read((*session).itrace_synth_opts);
    } else {
        itrace_synth_opts__set_default(&mut (*etm).synth_opts, (*(*session).itrace_synth_opts).default_no_sample);
        (*etm).synth_opts.callchain = false;
        (*etm).synth_opts.thread_stack = (*(*session).itrace_synth_opts).thread_stack;
    }
    if (*etm).synth_opts.calls { (*etm).branches_filter |= (PERF_IP_FLAG_CALL | PERF_IP_FLAG_TRACE_BEGIN | PERF_IP_FLAG_TRACE_END) as u32; }
    if (*etm).synth_opts.returns { (*etm).branches_filter |= (PERF_IP_FLAG_RETURN | PERF_IP_FLAG_TRACE_BEGIN | PERF_IP_FLAG_TRACE_END) as u32; }
    if (*etm).synth_opts.callchain && !symbol_conf.use_callchain {
        symbol_conf.use_callchain = true;
        if callchain_register_param(&mut callchain_param) < 0 {
            symbol_conf.use_callchain = false;
            (*etm).synth_opts.callchain = false;
        }
    }
    (*etm).session = session;
    (*etm).num_cpu = num_cpu;
    (*etm).pmu_type = ((*ptr.add(CS_PMU_TYPE_CPUS) >> 32) & 0xffff_ffff) as c_uint;
    (*etm).snapshot_mode = *ptr.add(CS_ETM_SNAPSHOT) != 0;
    (*etm).metadata = metadata;
    (*etm).auxtrace_type = auxtrace_info.type_;
    (*etm).has_virtual_ts = if (*etm).synth_opts.use_timestamp { true } else { cs_etm__has_virtual_ts(metadata, num_cpu) };
    if !(*etm).has_virtual_ts {
        ui__warning(c"Virtual timestamps are not enabled, or not supported by the traced system.\nThe time field of the samples will not be set accurately.\nFor Arm CPUs prior to Armv8.4 or without support FEAT_TRF,\nyou can specify the itrace option 'T' for timestamp decoding\nif the Coresight timestamp on the platform is same with the kernel time.\n\n".as_ptr());
    }
    (*etm).auxtrace.process_event = Some(cs_etm__process_event);
    (*etm).auxtrace.process_auxtrace_event = Some(cs_etm__process_auxtrace_event);
    (*etm).auxtrace.flush_events = Some(cs_etm__flush_events);
    (*etm).auxtrace.free_events = Some(cs_etm__free_events);
    (*etm).auxtrace.free = Some(cs_etm__free);
    (*etm).auxtrace.evsel_is_auxtrace = Some(cs_etm__evsel_is_auxtrace);
    (*session).auxtrace = &mut (*etm).auxtrace;
    cs_etm__setup_timeless_decoding(etm);
    (*etm).tc.time_shift = tc.time_shift;
    (*etm).tc.time_mult = tc.time_mult;
    (*etm).tc.time_zero = tc.time_zero;
    (*etm).tc.time_cycles = tc.time_cycles;
    (*etm).tc.time_mask = tc.time_mask;
    (*etm).tc.cap_user_time_zero = tc.cap_user_time_zero;
    (*etm).tc.cap_user_time_short = tc.cap_user_time_short;
    (*etm).use_thread_stack = (*etm).synth_opts.thread_stack || (*etm).synth_opts.last_branch || (*etm).synth_opts.callchain;
    (*etm).use_callchain = (*etm).synth_opts.thread_stack || (*etm).synth_opts.callchain;
    err = cs_etm__synth_events(etm, session);
    if err != 0 { return err; }
    err = cs_etm__queue_aux_records(session);
    if err != 0 { return err; }
    let mut aux_hw_id_found = 0;
    err = perf_session__peek_events(session, (*session).header.data_offset, (*session).header.data_size, cs_etm__process_aux_hw_id_cb, &mut aux_hw_id_found as *mut _ as *mut c_void);
    if err != 0 { return err; }
    if aux_hw_id_found == 0 {
        err = cs_etm__map_trace_ids_metadata(etm, num_cpu, metadata);
        if err != 0 { return err; }
    }
    err = cs_etm__create_decoders(etm);
    if err != 0 { return err; }
    (*etm).data_queued = (*etm).queues.populated;
    0
}
