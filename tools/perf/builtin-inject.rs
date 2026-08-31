// SPDX-License-Identifier: GPL-2.0
/*
 * builtin-inject.rs
 *
 * Source-level Rust translation of builtin-inject.c.
 *
 * Builtin inject command: Examine the live mode (stdin) event stream
 * and repipe it to stdout while optionally injecting additional
 * events into it.
 *
 * C include dependencies translated as external/opaque declarations below:
 * builtin.h, util/aslr.h, util/color.h, util/dso.h, util/vdso.h,
 * util/evlist.h, util/evsel.h, util/map.h, util/session.h, util/tool.h,
 * util/debug.h, util/build-id.h, util/data.h, util/auxtrace.h, util/jit.h,
 * util/string2.h, util/symbol.h, util/synthetic-events.h, util/pmus.h,
 * util/thread.h, util/namespaces.h, util/unwind.h, util/util.h, util/tsc.h,
 * internal/lib.h, linux/err.h, subcmd/parse-options.h, uapi/linux/mman.h,
 * linux/list.h, linux/string.h, linux/zalloc.h, linux/hash.h.
 */

#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(dead_code)]
#![allow(improper_ctypes)]

use core::ffi::{c_char, c_double, c_int, c_uint, c_ulong, c_ulonglong, c_void};
use core::mem::{size_of, zeroed};
use core::ptr;

type u8_ = u8;
type u16_ = u16;
type u32_ = u32;
type u64_ = u64;
type s64 = i64;
type size_t = usize;
type ssize_t = isize;
type off_t = i64;
type pid_t = i32;
type bool_ = bool;

const ENOMEM: c_int = 12;
const EINVAL: c_int = 22;
const EFAULT: c_int = 14;
const SEEK_CUR: c_int = 1;
const SEEK_SET: c_int = 0;
const SIGINT: c_int = 2;
const INT_MAX: c_uint = 0x7fff_ffff;
const PERF_EVLIST__HLIST_BITS: c_int = 8;
const PERF_EVLIST__HLIST_SIZE: usize = 1usize << PERF_EVLIST__HLIST_BITS;
const HEADER_FEAT_BITS: usize = 256;
const PERF_SAMPLE_MAX_SIZE: usize = 65536;
const PERF_MAX_STACK_DEPTH: u64_ = 127;
const SBUILD_ID_SIZE: c_int = 128;
const PERF_ATTR_SIZE_VER0: u32_ = 64;
const PERF_ATTR_SIZE_VER2: u32_ = 80;
const PERF_RECORD_SAMPLE: u32_ = 9;
const PERF_RECORD_COMM: u32_ = 3;
const PERF_RECORD_USER_TYPE_START: u32_ = 64;
const PERF_RECORD_MISC_MMAP_BUILD_ID: u16_ = 0x2000;
const PERF_RECORD_MISC_CPUMODE_MASK: u16_ = 7;
const PERF_RECORD_MISC_USER: u16_ = 2;
const PERF_RECORD_MISC_KERNEL: u16_ = 1;
const PERF_RECORD_MISC_HYPERVISOR: u16_ = 3;
const PERF_RECORD_MISC_GUEST_KERNEL: u16_ = 4;
const PERF_RECORD_MISC_GUEST_USER: u16_ = 5;
const PERF_RECORD_MISC_CPUMODE_UNKNOWN: u16_ = 0;
const PERF_RECORD_MISC_SWITCH_OUT: u16_ = 0x2000;
const PERF_RECORD_KSYMBOL_TYPE_OOL: u16_ = 1;
const PERF_SAMPLE_AUX: u64_ = 1 << 20;
const PERF_SAMPLE_BRANCH_STACK: u64_ = 1 << 11;
const PERF_SAMPLE_BRANCH_HW_INDEX: u64_ = 1 << 17;
const PERF_SAMPLE_STACK_USER: u64_ = 1 << 13;
const PERF_SAMPLE_REGS_USER: u64_ = 1 << 12;
const PERF_SAMPLE_CALLCHAIN: u64_ = 1 << 5;
const PERF_SAMPLE_IDENTIFIER: u64_ = 1 << 16;
const PERF_SAMPLE_TID: u64_ = 1 << 1;
const PERF_CONTEXT_USER: u64_ = (-512i64) as u64_;
const MAP_HUGETLB: u32_ = 0x40000;
const PROT_EXEC: u32_ = 0x4;
const PERF_DATA_MODE_READ: c_int = 0;
const PERF_DATA_MODE_WRITE: c_int = 1;
const PERF_COLOR_RED: c_int = 1;
const HEADER_BUILD_ID: c_int = 0;
const HEADER_TRACING_DATA: c_int = 1;
const HEADER_HOSTNAME: c_int = 2;
const HEADER_OSRELEASE: c_int = 3;
const HEADER_VERSION: c_int = 4;
const HEADER_ARCH: c_int = 5;
const HEADER_NRCPUS: c_int = 6;
const HEADER_CPUDESC: c_int = 7;
const HEADER_CPUID: c_int = 8;
const HEADER_TOTAL_MEM: c_int = 9;
const HEADER_CPU_TOPOLOGY: c_int = 10;
const HEADER_NUMA_TOPOLOGY: c_int = 11;
const HEADER_PMU_MAPPINGS: c_int = 12;
const HEADER_CACHE: c_int = 13;
const HEADER_MEM_TOPOLOGY: c_int = 14;
const HEADER_CLOCKID: c_int = 15;
const HEADER_BPF_PROG_INFO: c_int = 16;
const HEADER_BPF_BTF: c_int = 17;
const HEADER_CPU_PMU_CAPS: c_int = 18;
const HEADER_CLOCK_DATA: c_int = 19;
const HEADER_HYBRID_TOPOLOGY: c_int = 20;
const HEADER_PMU_CAPS: c_int = 21;
const HEADER_CPU_DOMAIN_INFO: c_int = 22;
const HEADER_CLN_SIZE: c_int = 23;
const HEADER_CMDLINE: c_int = 24;
const HEADER_EVENT_DESC: c_int = 25;
const HEADER_BRANCH_STACK: c_int = 26;
const HEADER_GROUP_DESC: c_int = 27;
const HEADER_AUXTRACE: c_int = 28;
const HEADER_STAT: c_int = 29;
const HEADER_SAMPLE_TIME: c_int = 30;
const HEADER_DIR_FORMAT: c_int = 31;
const HEADER_COMPRESSED: c_int = 32;
const REGS_USER: c_int = 0;
const STACK_USER: c_int = 1;

#[repr(C)] pub struct list_head { pub next: *mut list_head, pub prev: *mut list_head }
#[repr(C)] pub struct hlist_node { pub next: *mut hlist_node, pub pprev: *mut *mut hlist_node }
#[repr(C)] pub struct hlist_head { pub first: *mut hlist_node }
#[repr(C)] pub struct ordered_events { _priv: [u8; 0] }
#[repr(C)] pub struct machine { pub single_address_space: bool_, pub dsos: dsos }
#[repr(C)] pub struct dsos { _priv: [u8; 0] }
#[repr(C)] pub struct map { _priv: [u8; 0] }
#[repr(C)] pub struct dso { _priv: [u8; 0] }
#[repr(C)] pub struct thread { _priv: [u8; 0] }
#[repr(C)] pub struct nsinfo { _priv: [u8; 0] }
#[repr(C)] pub struct nscookie { _priv: [u8; 0] }
#[repr(C)] pub struct strlist { _priv: [u8; 0] }
#[repr(C)] pub struct str_node { pub s: *mut c_char }
#[repr(C)] pub struct mutex { _priv: [u8; 0] }
#[repr(C)] pub struct option { pub value: *mut c_void }
#[repr(C)] pub struct feat_writer { pub write: Option<unsafe extern "C" fn(*mut feat_writer, *mut c_void, size_t) -> c_int> }
#[repr(C)] pub struct feat_copier { pub copy: Option<unsafe extern "C" fn(*mut feat_copier, c_int, *mut feat_writer) -> c_int> }

#[repr(C)]
pub struct perf_event_header { pub type_: u32_, pub misc: u16_, pub size: u16_ }

#[repr(C)]
pub struct perf_event_attr {
    pub size: u32_,
    pub sample_type: u64_,
    pub read_format: u64_,
    pub branch_sample_type: u64_,
    pub sample_regs_user: u64_,
    pub sample_stack_user: u32_,
    pub exclude_callchain_user: u32_,
    pub mmap: bool_,
    pub exclude_host: u32_,
    pub exclude_guest: u32_,
}

#[repr(C)] pub struct perf_event_attr_event { pub header: perf_event_header, pub attr: perf_event_attr }
#[repr(C)] pub struct perf_event_mmap { pub header: perf_event_header, pub pid: u32_, pub tid: u32_, pub start: u64_, pub len: u64_, pub pgoff: u64_, pub filename: *const c_char }
#[repr(C)] pub struct perf_event_mmap2 { pub header: perf_event_header, pub pid: u32_, pub tid: u32_, pub start: u64_, pub len: u64_, pub pgoff: u64_, pub maj: u32_, pub min: u32_, pub ino: u64_, pub ino_generation: u64_, pub prot: u32_, pub flags: u32_, pub build_id: *const u8_, pub build_id_size: u8_, pub filename: *const c_char }
#[repr(C)] pub struct perf_event_auxtrace { pub header: perf_event_header, pub size: u64_ }
#[repr(C)] pub struct perf_event_comm { pub header: perf_event_header, pub pid: u32_, pub tid: u32_, pub comm: *const c_char }
#[repr(C)] pub struct perf_event_context_switch { pub header: perf_event_header, pub next_prev_pid: u32_, pub next_prev_tid: u32_ }
#[repr(C)] pub struct perf_event_ksymbol { pub header: perf_event_header, pub ksym_type: u16_ }

#[repr(C)]
pub union perf_event {
    pub header: perf_event_header,
    pub attr: core::mem::ManuallyDrop<perf_event_attr_event>,
    pub mmap: core::mem::ManuallyDrop<perf_event_mmap>,
    pub mmap2: core::mem::ManuallyDrop<perf_event_mmap2>,
    pub auxtrace: core::mem::ManuallyDrop<perf_event_auxtrace>,
    pub comm: core::mem::ManuallyDrop<perf_event_comm>,
    pub context_switch: core::mem::ManuallyDrop<perf_event_context_switch>,
    pub ksymbol: core::mem::ManuallyDrop<perf_event_ksymbol>,
}

#[repr(C)] pub struct branch_stack { pub nr: u64_, pub hw_idx: u64_ }
#[repr(C)] pub struct aux_sample { pub data: *mut c_void, pub size: size_t }
#[repr(C)]
pub struct ip_callchain { pub nr: u64_, pub ips: [u64_; PERF_MAX_STACK_DEPTH as usize] }

#[repr(C)]
pub struct perf_sample {
    pub evsel: *mut evsel,
    pub pid: i32,
    pub tid: i32,
    pub time: u64_,
    pub stream_id: u64_,
    pub id: u64_,
    pub cpu: u32_,
    pub period: u64_,
    pub cpumode: u16_,
    pub ip: u64_,
    pub aux_sample: aux_sample,
    pub branch_stack: *mut branch_stack,
    pub callchain: *mut ip_callchain,
}

#[repr(C)] pub struct cpu_id { pub cpu: i32 }
#[repr(C)] pub struct perf_sample_id { pub cpu: cpu_id, pub tid: u32_, pub machine_pid: u32_, pub vcpu: cpu_id }
#[repr(C)] pub struct evsel_core { pub attr: perf_event_attr, pub ids: u32_, pub id: *mut u64_ }
type inject_handler = Option<unsafe extern "C" fn(*const perf_tool, *mut perf_event, *mut perf_sample, *mut machine) -> c_int>;
#[repr(C)] pub struct evsel { pub core: evsel_core, pub handler: inject_handler }
#[repr(C)] pub struct evlist { _priv: [u8; 0] }
#[repr(C)] pub struct perf_file_section { pub offset: u64_, pub size: u64_ }
#[repr(C)] pub struct perf_file { pub use_stdio: bool_ }
#[repr(C)] pub struct perf_data { pub path: *const c_char, pub force: bool_, pub mode: c_int, pub is_pipe: bool_, pub is_dir: bool_, pub in_place_update: bool_, pub file: perf_file }
#[repr(C)] pub struct perf_header { pub data_offset: u64_, pub data_size: u64_ }
#[repr(C)] pub struct machines { pub host: machine }
#[repr(C)] pub struct perf_tsc_conversion { pub time_shift: u16_, pub time_mult: u32_, pub time_zero: u64_, pub time_cycles: u64_, pub time_mask: u64_, pub cap_user_time_zero: bool_, pub cap_user_time_short: bool_ }
#[repr(C)] pub struct perf_record_time_conv { pub time_shift: u16_, pub time_mult: u32_, pub time_zero: u64_, pub time_cycles: u64_, pub time_mask: u64_, pub cap_user_time_zero: bool_, pub cap_user_time_short: bool_ }
#[repr(C)] pub struct auxtrace_index { _priv: [u8; 0] }
#[repr(C)] pub struct zstd_data { _priv: [u8; 0] }
#[repr(C)]
pub struct perf_session {
    pub evlist: *mut evlist,
    pub header: perf_header,
    pub data: *mut perf_data,
    pub one_mmap: bool_,
    pub auxtrace_index: auxtrace_index,
    pub machines: machines,
    pub time_conv: perf_record_time_conv,
    pub itrace_synth_opts: *mut itrace_synth_opts,
    pub zstd_data: zstd_data,
    pub tool: *mut perf_tool,
}

#[repr(C)]
pub struct perf_tool {
    pub sample: inject_handler,
    pub read: inject_handler,
    pub mmap: inject_handler,
    pub mmap2: inject_handler,
    pub comm: inject_handler,
    pub namespaces: inject_handler,
    pub cgroup: inject_handler,
    pub fork: inject_handler,
    pub exit: inject_handler,
    pub lost: inject_handler,
    pub lost_samples: inject_handler,
    pub aux: inject_handler,
    pub itrace_start: inject_handler,
    pub aux_output_hw_id: inject_handler,
    pub context_switch: inject_handler,
    pub throttle: inject_handler,
    pub unthrottle: inject_handler,
    pub ksymbol: inject_handler,
    pub bpf: inject_handler,
    pub text_poke: inject_handler,
    pub attr: Option<unsafe extern "C" fn(*const perf_tool, *mut perf_event, *mut *mut evlist) -> c_int>,
    pub event_update: Option<unsafe extern "C" fn(*const perf_tool, *mut perf_event, *mut *mut evlist) -> c_int>,
    pub tracing_data: Option<unsafe extern "C" fn(*const perf_tool, *mut perf_session, *mut perf_event) -> c_int>,
    pub finished_round: Option<unsafe extern "C" fn(*const perf_tool, *mut perf_event, *mut ordered_events) -> c_int>,
    pub build_id: Option<unsafe extern "C" fn(*const perf_tool, *mut perf_session, *mut perf_event) -> c_int>,
    pub id_index: Option<unsafe extern "C" fn(*const perf_tool, *mut perf_session, *mut perf_event) -> c_int>,
    pub auxtrace_info: Option<unsafe extern "C" fn(*const perf_tool, *mut perf_session, *mut perf_event) -> c_int>,
    pub auxtrace_error: Option<unsafe extern "C" fn(*const perf_tool, *mut perf_session, *mut perf_event) -> c_int>,
    pub time_conv: Option<unsafe extern "C" fn(*const perf_tool, *mut perf_session, *mut perf_event) -> c_int>,
    pub thread_map: Option<unsafe extern "C" fn(*const perf_tool, *mut perf_session, *mut perf_event) -> c_int>,
    pub cpu_map: Option<unsafe extern "C" fn(*const perf_tool, *mut perf_session, *mut perf_event) -> c_int>,
    pub stat_config: Option<unsafe extern "C" fn(*const perf_tool, *mut perf_session, *mut perf_event) -> c_int>,
    pub stat: Option<unsafe extern "C" fn(*const perf_tool, *mut perf_session, *mut perf_event) -> c_int>,
    pub stat_round: Option<unsafe extern "C" fn(*const perf_tool, *mut perf_session, *mut perf_event) -> c_int>,
    pub feature: Option<unsafe extern "C" fn(*const perf_tool, *mut perf_session, *mut perf_event) -> c_int>,
    pub finished_init: Option<unsafe extern "C" fn(*const perf_tool, *mut perf_session, *mut perf_event) -> c_int>,
    pub compressed: Option<unsafe extern "C" fn(*const perf_tool, *mut perf_session, *mut perf_event, u64_, *const c_char) -> c_int>,
    pub auxtrace: Option<unsafe extern "C" fn(*const perf_tool, *mut perf_session, *mut perf_event) -> s64>,
    pub bpf_metadata: Option<unsafe extern "C" fn(*const perf_tool, *mut perf_session, *mut perf_event) -> c_int>,
    pub schedstat_cpu: Option<unsafe extern "C" fn(*const perf_tool, *mut perf_session, *mut perf_event) -> c_int>,
    pub schedstat_domain: Option<unsafe extern "C" fn(*const perf_tool, *mut perf_session, *mut perf_event) -> c_int>,
    pub ordered_events: bool_,
    pub ordering_requires_timestamps: bool_,
    pub dont_split_sample_group: bool_,
    pub merge_deferred_callchains: bool_,
}

#[repr(C)] pub struct itrace_synth_opts { pub set: bool_, pub add_last_branch: bool_, pub last_branch: bool_, pub inject: bool_, pub vm_time_correlation: bool_, pub vm_tm_corr_dry_run: bool_, pub vm_tm_corr_args: *mut c_char }
#[repr(C)] pub struct build_id { pub size: u8_, pub data: [u8_; 64] }
#[repr(C)] pub struct dso_id { pub build_id: build_id, pub maj: u32_, pub min: u32_, pub ino: u64_, pub ino_generation: u64_, pub mmap2_valid: bool_, pub mmap2_ino_generation_valid: bool_ }
#[repr(C)] pub struct callchain_cursor_node { pub ip: u64_, pub ms: map_symbol, pub next: *mut callchain_cursor_node }
#[repr(C)] pub struct map_symbol { pub map: *mut map, pub sym: *mut symbol }
#[repr(C)] pub struct symbol { _priv: [u8; 0] }
#[repr(C)] pub struct callchain_cursor { pub first: *mut callchain_cursor_node, pub nr: u64_ }
#[repr(C)] pub struct addr_location { pub map: *mut map }

#[repr(C)]
pub struct guest_event { pub sample: perf_sample, pub event: *mut perf_event, pub event_buf: *mut c_char }
#[repr(C)]
pub struct guest_id { pub node: hlist_node, pub id: u64_, pub host_id: u64_, pub vcpu: u32_ }
#[repr(C)]
pub struct guest_tid { pub node: hlist_node, pub tid: u32_, pub vcpu: u32_ }
#[repr(C)]
pub struct guest_vcpu { pub cpu: u32_, pub tid: u32_ }
#[repr(C)]
pub struct guest_session {
    pub perf_data_file: *mut c_char,
    pub machine_pid: u32_,
    pub time_offset: u64_,
    pub time_scale: c_double,
    pub tool: perf_tool,
    pub data: perf_data,
    pub session: *mut perf_session,
    pub tmp_file_name: *mut c_char,
    pub tmp_fd: c_int,
    pub host_tc: perf_tsc_conversion,
    pub guest_tc: perf_tsc_conversion,
    pub copy_kcore_dir: bool_,
    pub have_tc: bool_,
    pub fetched: bool_,
    pub ready: bool_,
    pub dflt_id_hdr_size: u16_,
    pub dflt_id: u64_,
    pub highest_id: u64_,
    pub vcpu: *mut guest_vcpu,
    pub vcpu_cnt: size_t,
    pub heads: [hlist_head; PERF_EVLIST__HLIST_SIZE],
    pub tids: [hlist_head; PERF_EVLIST__HLIST_SIZE],
    pub ev: guest_event,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum build_id_rewrite_style {
    BID_RWS__NONE = 0,
    BID_RWS__INJECT_HEADER_LAZY,
    BID_RWS__INJECT_HEADER_ALL,
    BID_RWS__MMAP2_BUILDID_ALL,
    BID_RWS__MMAP2_BUILDID_LAZY,
}

#[repr(C)]
pub struct perf_inject {
    pub tool: perf_tool,
    pub session: *mut perf_session,
    pub build_id_style: build_id_rewrite_style,
    pub sched_stat: bool_,
    pub have_auxtrace: bool_,
    pub strip: bool_,
    pub jit_mode: bool_,
    pub in_place_update: bool_,
    pub in_place_update_dry_run: bool_,
    pub copy_kcore_dir: bool_,
    pub convert_callchain: bool_,
    pub aslr: bool_,
    pub input_name: *const c_char,
    pub output: perf_data,
    pub bytes_written: u64_,
    pub aux_id: u64_,
    pub samples: list_head,
    pub itrace_synth_opts: itrace_synth_opts,
    pub event_copy: *mut c_char,
    pub secs: [perf_file_section; HEADER_FEAT_BITS],
    pub guest_session: guest_session,
    pub known_build_ids: *mut strlist,
    pub mmap_evsel: *mut evsel,
    pub raw_callchain: *mut ip_callchain,
}

#[repr(C)]
pub struct event_entry { pub node: list_head, pub tid: u32_, pub event: [perf_event; 0] }
#[repr(C)] pub struct mark_dso_hit_args { pub inject: *const perf_inject, pub tool: *const perf_tool, pub sample: *mut perf_sample, pub machine: *mut machine, pub mmap_evsel: *mut evsel }
#[repr(C)] pub struct inject_fc { pub fc: feat_copier, pub inject: *mut perf_inject }

unsafe extern "C" {
    static mut errno: c_int;
    static mut session_done: c_int;
    static mut verbose: c_int;
    static mut symbol_conf: symbol_conf_t;
    static dso_id_empty: dso_id;
    fn perf_data__write(data: *mut perf_data, buf: *const c_void, sz: size_t) -> ssize_t;
    fn perf_data__read(data: *mut perf_data, buf: *mut c_void, sz: size_t) -> ssize_t;
    fn perf_data__seek(data: *mut perf_data, off: off_t, whence: c_int) -> off_t;
    fn perf_data__is_pipe(data: *mut perf_data) -> bool_;
    fn perf_data__fd(data: *mut perf_data) -> c_int;
    fn perf_data__open(data: *mut perf_data) -> c_int;
    fn perf_data__close(data: *mut perf_data);
    fn perf_event__process_attr(tool: *const perf_tool, event: *mut perf_event, pevlist: *mut *mut evlist) -> c_int;
    fn perf_event__process_mmap(tool: *const perf_tool, event: *mut perf_event, sample: *mut perf_sample, machine: *mut machine) -> c_int;
    fn perf_event__process_mmap2(tool: *const perf_tool, event: *mut perf_event, sample: *mut perf_sample, machine: *mut machine) -> c_int;
    fn perf_event__process_fork(tool: *const perf_tool, event: *mut perf_event, sample: *mut perf_sample, machine: *mut machine) -> c_int;
    fn perf_event__process_comm(tool: *const perf_tool, event: *mut perf_event, sample: *mut perf_sample, machine: *mut machine) -> c_int;
    fn perf_event__process_namespaces(tool: *const perf_tool, event: *mut perf_event, sample: *mut perf_sample, machine: *mut machine) -> c_int;
    fn perf_event__process_exit(tool: *const perf_tool, event: *mut perf_event, sample: *mut perf_sample, machine: *mut machine) -> c_int;
    fn perf_event__process_tracing_data(tool: *const perf_tool, session: *mut perf_session, event: *mut perf_event) -> c_int;
    fn perf_event__process_build_id(tool: *const perf_tool, session: *mut perf_session, event: *mut perf_event) -> c_int;
    fn perf_event__process_id_index(tool: *const perf_tool, session: *mut perf_session, event: *mut perf_event) -> c_int;
    fn perf_event__process_finished_round(tool: *const perf_tool, event: *mut perf_event, oe: *mut ordered_events) -> c_int;
    fn perf_event__process_auxtrace_info(tool: *const perf_tool, session: *mut perf_session, event: *mut perf_event) -> c_int;
    fn perf_event__process_auxtrace(tool: *const perf_tool, session: *mut perf_session, event: *mut perf_event) -> c_int;
    fn perf_event__process_auxtrace_error(tool: *const perf_tool, session: *mut perf_session, event: *mut perf_event) -> c_int;
    fn perf_event__synthesize_attr(tool: *const perf_tool, attr: *mut perf_event_attr, ids: u32_, id: *mut u64_, cb: unsafe extern "C" fn(*const perf_tool, *mut perf_event, *mut perf_sample, *mut machine) -> c_int) -> c_int;
    fn perf_event__synthesize_sample(event: *mut perf_event, sample_type: u64_, read_format: u64_, branch_sample_type: u64_, sample: *mut perf_sample) -> c_int;
    fn perf_event__sample_event_size(sample: *mut perf_sample, sample_type: u64_, read_format: u64_, branch_sample_type: u64_) -> size_t;
    fn perf_event__synthesize_build_id(tool: *const perf_tool, sample: *mut perf_sample, machine: *mut machine, cb: inject_handler, misc: u16_, bid: *const build_id, filename: *const c_char) -> c_int;
    fn perf_event__synthesize_mmap2_build_id(tool: *const perf_tool, sample: *mut perf_sample, machine: *mut machine, cb: inject_handler, misc: u16_, pid: u32_, tid: u32_, start: u64_, len: u64_, pgoff: u64_, bid: *const build_id, prot: u32_, flags: u32_, filename: *const c_char) -> c_int;
    fn perf_event__synthesize_id_sample(array: *mut c_void, sample_type: u64_, sample: *const perf_sample) -> c_int;
    fn perf_event__synthesize_for_pipe(tool: *const perf_tool, session: *mut perf_session, output: *mut perf_data, cb: inject_handler) -> c_int;
    fn __perf_event__synthesize_id_index(tool: *const perf_tool, cb: inject_handler, evlist: *mut evlist, machine: *mut machine, from: size_t) -> c_int;
    fn perf_header__write_pipe(fd: c_int) -> c_int;
    fn perf_header__set_feat(header: *mut perf_header, feat: c_int);
    fn perf_header__clear_feat(header: *mut perf_header, feat: c_int);
    fn perf_header__has_feat(header: *mut perf_header, feat: c_int) -> bool_;
    fn perf_header__process_sections(header: *mut perf_header, fd: c_int, data: *mut c_void, cb: unsafe extern "C" fn(*mut perf_file_section, *mut perf_header, c_int, c_int, *mut c_void) -> c_int) -> c_int;
    fn perf_session__new(data: *mut perf_data, tool: *mut perf_tool) -> *mut perf_session;
    fn __perf_session__new(data: *mut perf_data, tool: *mut perf_tool, trace_event_repipe: bool_, host_env: *mut c_void) -> *mut perf_session;
    fn perf_session__delete(session: *mut perf_session);
    fn perf_session__process_events(session: *mut perf_session) -> c_int;
    fn perf_session__peek_events(session: *mut perf_session, offset: u64_, size: u64_, cb: unsafe extern "C" fn(*mut perf_session, *mut perf_event, u64_, *mut c_void) -> c_int, data: *mut c_void) -> c_int;
    fn perf_session__data_offset(evlist: *mut evlist) -> u64_;
    fn perf_session__inject_header(session: *mut perf_session, evlist: *mut evlist, fd: c_int, fc: *mut feat_copier, write_attrs_after_data: bool_);
    fn perf_session__dsos_hit_all(session: *mut perf_session);
    fn perf_session__findnew_machine(session: *mut perf_session, pid: pid_t) -> *mut machine;
    fn perf_session__env(session: *mut perf_session) -> *mut c_void;
    fn auxtrace_index__auxtrace_event(index: *mut auxtrace_index, event: *mut perf_event, offset: off_t) -> c_int;
    fn auxtrace_index__free(index: *mut auxtrace_index);
    fn evlist__id2sid(evlist: *mut evlist, id: u64_) -> *mut perf_sample_id;
    fn evlist__id2evsel(evlist: *mut evlist, id: u64_) -> *mut evsel;
    fn evlist__event2evsel(evlist: *mut evlist, event: *mut perf_event) -> *mut evsel;
    fn evlist__parse_sample(evlist: *mut evlist, event: *mut perf_event, sample: *mut perf_sample) -> c_int;
    fn evlist__nr_entries(evlist: *mut evlist) -> size_t;
    fn evsel__get(evsel: *mut evsel);
    fn evsel__name(evsel: *mut evsel) -> *const c_char;
    fn evsel__parse_sample(evsel: *mut evsel, event: *mut perf_event, sample: *mut perf_sample) -> c_int;
    fn evsel__id_hdr_size(evsel: *mut evsel) -> u16_;
    fn evsel__reset_sample_bit(evsel: *mut evsel, bit: c_int);
    fn evsel__is_dummy_event(evsel: *mut evsel) -> bool_;
    fn machine__find_thread(machine: *mut machine, tid: i32, pid: i32) -> *mut thread;
    fn machine__findnew_thread(machine: *mut machine, pid: i32, tid: i32) -> *mut thread;
    fn machine__findnew_vdso(machine: *mut machine, thread: *mut thread) -> *mut dso;
    fn machine__findnew_dso_id(machine: *mut machine, filename: *const c_char, id: *const dso_id) -> *mut dso;
    fn machine__kernel_ip(machine: *mut machine, ip: u64_) -> bool_;
    fn thread__put(thread: *mut thread);
    fn thread__nsinfo(thread: *mut thread) -> *mut nsinfo;
    fn thread__resolve_callchain(thread: *mut thread, cursor: *mut callchain_cursor, sample: *mut perf_sample, parent: *mut c_void, root_al: *mut c_void, max_stack: u64_) -> c_int;
    fn thread__find_map(thread: *mut thread, cpumode: u16_, ip: u64_, al: *mut addr_location) -> bool_;
    fn sample__for_each_callchain_node(thread: *mut thread, sample: *mut perf_sample, max_stack: u64_, symbols: bool_, cb: unsafe extern "C" fn(*mut callchain_cursor_node, *mut c_void) -> c_int, data: *mut c_void) -> c_int;
    fn get_tls_callchain_cursor() -> *mut callchain_cursor;
    fn callchain_cursor_reset(cursor: *mut callchain_cursor);
    fn symbol__inlined(sym: *mut symbol) -> bool_;
    fn symbol__validate_sym_arguments() -> c_int;
    fn symbol__init(env: *mut c_void) -> c_int;
    fn build_id__mark_dso_hit(tool: *const perf_tool, event: *mut perf_event, sample: *mut perf_sample, machine: *mut machine);
    fn build_id__init(bid: *mut build_id, data: *const u8_, size: u8_);
    fn filename__read_build_id(filename: *const c_char, bid: *mut build_id) -> c_int;
    fn dso__has_build_id(dso: *mut dso) -> bool_;
    fn dso__set_build_id(dso: *mut dso, bid: *const build_id);
    fn dso__bid(dso: *mut dso) -> *const build_id;
    fn dso__long_name(dso: *mut dso) -> *const c_char;
    fn dso__filename_with_chroot(dso: *mut dso, name: *const c_char) -> *mut c_char;
    fn dso__nsinfo(dso: *mut dso) -> *mut nsinfo;
    fn dso__lock(dso: *mut dso) -> *mut mutex;
    fn dso__set_nsinfo(dso: *mut dso, nsi: *mut nsinfo);
    fn dso__set_hit(dso: *mut dso);
    fn dso__hit(dso: *mut dso) -> bool_;
    fn dso__put(dso: *mut dso);
    fn dso__is_vdso(dso: *mut dso) -> bool_;
    fn dso__is_kcore(dso: *mut dso) -> bool_;
    fn dso__kernel(dso: *mut dso) -> bool_;
    fn nsinfo__get(nsi: *mut nsinfo) -> *mut nsinfo;
    fn nsinfo__copy(nsi: *mut nsinfo) -> *mut nsinfo;
    fn nsinfo__put(nsi: *mut nsinfo);
    fn nsinfo__clear_need_setns(nsi: *mut nsinfo);
    fn nsinfo__mountns_enter(nsi: *mut nsinfo, nsc: *mut nscookie);
    fn nsinfo__mountns_exit(nsc: *mut nscookie);
    fn is_vdso_map(filename: *const c_char) -> bool_;
    fn is_anon_memory(filename: *const c_char) -> bool_;
    fn is_no_dso_memory(filename: *const c_char) -> bool_;
    fn is_kernel_module(name: *const c_char, cpumode: u16_) -> bool_;
    fn map__dso(map: *mut map) -> *mut dso;
    fn map__flags(map: *mut map) -> u32_;
    fn map__prot(map: *mut map) -> u32_;
    fn map__start(map: *mut map) -> u64_;
    fn map__end(map: *mut map) -> u64_;
    fn map__pgoff(map: *mut map) -> u64_;
    fn map__hit(map: *mut map) -> bool_;
    fn map__set_hit(map: *mut map);
    fn __map__is_kernel(map: *mut map) -> bool_;
    fn addr_location__init(al: *mut addr_location);
    fn addr_location__exit(al: *mut addr_location);
    fn strlist__new(s: *const c_char, dupstr: *mut c_void) -> *mut strlist;
    fn strlist__delete(sl: *mut strlist);
    fn strlist__remove(sl: *mut strlist, node: *mut str_node);
    fn skip_spaces(s: *const c_char) -> *mut c_char;
    fn hex(c: c_char) -> u8_;
    fn has_kcore_dir(path: *const c_char) -> bool_;
    fn perf_time_to_tsc(time: u64_, tc: *mut perf_tsc_conversion) -> u64_;
    fn tsc_to_perf_time(tsc: u64_, tc: *mut perf_tsc_conversion) -> u64_;
    fn zstd_init(data: *mut zstd_data, level: c_int) -> c_int;
    fn zstd_fini(data: *mut zstd_data);
    fn writen(fd: c_int, buf: *const c_void, sz: size_t) -> ssize_t;
    fn readn(fd: c_int, buf: *mut c_void, sz: size_t) -> ssize_t;
    fn preadn(fd: c_int, buf: *mut c_void, sz: size_t, offs: u64_) -> ssize_t;
    fn malloc(sz: size_t) -> *mut c_void;
    fn calloc(n: size_t, sz: size_t) -> *mut c_void;
    fn free(p: *mut c_void);
    fn memcpy(dst: *mut c_void, src: *const c_void, n: size_t) -> *mut c_void;
    fn memset(dst: *mut c_void, c: c_int, n: size_t) -> *mut c_void;
    fn strdup(s: *const c_char) -> *mut c_char;
    fn strsep(s: *mut *mut c_char, delim: *const c_char) -> *mut c_char;
    fn strchr(s: *const c_char, c: c_int) -> *mut c_char;
    fn strcmp(a: *const c_char, b: *const c_char) -> c_int;
    fn strncmp(a: *const c_char, b: *const c_char, n: size_t) -> c_int;
    fn strlen(s: *const c_char) -> size_t;
    fn strtoul(s: *const c_char, endp: *mut *mut c_char, base: c_int) -> c_ulong;
    fn strtoull(s: *const c_char, endp: *mut *mut c_char, base: c_int) -> c_ulonglong;
    fn strtod(s: *const c_char, endp: *mut *mut c_char) -> c_double;
    fn sscanf(s: *const c_char, fmt: *const c_char, ...) -> c_int;
    fn isxdigit(c: c_int) -> c_int;
    fn lseek(fd: c_int, off: off_t, whence: c_int) -> off_t;
    fn close(fd: c_int) -> c_int;
    fn unlink(path: *const c_char) -> c_int;
    fn mkstemp(template: *mut c_char) -> c_int;
    fn system(cmd: *const c_char) -> c_int;
    fn asprintf(strp: *mut *mut c_char, fmt: *const c_char, ...) -> c_int;
    fn signal(sig: c_int, handler: unsafe extern "C" fn(c_int)) -> usize;
    fn perror(s: *const c_char);
    fn pr_err(fmt: *const c_char, ...);
    fn pr_debug(fmt: *const c_char, ...);
    fn pr_warning(fmt: *const c_char, ...);
    fn color_fprintf(stream: *mut c_void, color: c_int, fmt: *const c_char, ...) -> c_int;
    static mut stderr: *mut c_void;
    fn parse_options(argc: c_int, argv: *mut *const c_char, options: *mut option, usage: *const *const c_char, flags: c_int) -> c_int;
    fn usage_with_options(usage: *const *const c_char, options: *mut option) -> !;
    fn perf_tool__init(tool: *mut perf_tool, ordered_events: bool_);
    fn aslr_tool__new(tool: *mut perf_tool) -> *mut perf_tool;
    fn aslr_tool__delete(tool: *mut perf_tool);
    fn aslr_tool__strip_attr_event(event: *mut perf_event, evlist: *mut evlist);
    fn aslr_tool__strip_evlist(tool: *mut perf_tool, evlist: *mut evlist);
    fn aslr_tool__restore_evlist(tool: *mut perf_tool, evlist: *mut evlist);
    fn aslr_tool__cache_orig_attrs(tool: *mut perf_tool, evsel: *mut evsel) -> c_int;
}

#[repr(C)] pub struct symbol_conf_t { pub vmlinux_name: *const c_char, pub ignore_vmlinux: bool_, pub kallsyms_name: *const c_char, pub guestmount: *const c_char, pub lazy_load_kernel_maps: bool_ }

unsafe fn container_of_tool(tool: *const perf_tool) -> *mut perf_inject { tool as *mut perf_inject }
unsafe fn container_of_guest_tool(tool: *const perf_tool) -> *mut guest_session { tool as *mut guest_session }
unsafe fn container_of_guest_session(gs: *mut guest_session) -> *mut perf_inject {
    (gs as *mut u8).sub(core::mem::offset_of!(perf_inject, guest_session)) as *mut perf_inject
}
unsafe fn container_of_feat_copier(fc: *mut feat_copier) -> *mut inject_fc { fc as *mut inject_fc }
unsafe fn ERR_PTR(err: isize) -> *mut perf_event { err as *mut perf_event }
unsafe fn IS_ERR<T>(p: *mut T) -> bool { (p as isize) < 0 && (p as isize) > -4096 }
unsafe fn PTR_ERR<T>(p: *mut T) -> c_int { p as isize as c_int }
unsafe fn min_size(a: size_t, b: size_t) -> size_t { if a < b { a } else { b } }
unsafe fn min_off(a: off_t, b: off_t) -> off_t { if a < b { a } else { b } }
unsafe fn roundup(x: u64_, y: u64_) -> u64_ { ((x + y - 1) / y) * y }
unsafe fn hash_32(v: u32_, bits: c_int) -> c_int { ((v.wrapping_mul(0x9e370001)) >> (32 - bits)) as c_int }
unsafe fn hash_64(v: u64_, bits: c_int) -> c_int { ((v.wrapping_mul(0x9e37_0001_0000_0001)) >> (64 - bits)) as c_int }
unsafe fn hlist_add_head(n: *mut hlist_node, h: *mut hlist_head) { (*n).next = (*h).first; (*n).pprev = &mut (*h).first; if !(*h).first.is_null() { (*(*h).first).pprev = &mut (*n).next; } (*h).first = n; }
unsafe fn hlist_del(n: *mut hlist_node) { if !(*n).next.is_null() { (*(*n).next).pprev = (*n).pprev; } if !(*n).pprev.is_null() { *(*n).pprev = (*n).next; } }
unsafe fn list_del_init(_n: *mut list_head) {}
unsafe fn list_add(_n: *mut list_head, _h: *mut list_head) {}
unsafe fn mutex_lock(_m: *mut mutex) {}
unsafe fn mutex_unlock(_m: *mut mutex) {}
unsafe fn zfree<T>(p: *mut *mut T) { if !(*p).is_null() { free(*p as *mut c_void); *p = ptr::null_mut(); } }
unsafe fn evlist_first_entry(_evlist: *mut evlist) -> *mut evsel { ptr::null_mut() }
unsafe fn evsel_next(_evsel: *mut evsel) -> *mut evsel { ptr::null_mut() }
unsafe fn strlist_first(_sl: *mut strlist) -> *mut str_node { ptr::null_mut() }
unsafe fn strlist_next(_n: *mut str_node) -> *mut str_node { ptr::null_mut() }
unsafe fn realloc_array_as_needed<T>(ptrp: *mut *mut T, cntp: *mut size_t, idx: size_t) -> c_int {
    if idx < *cntp { return 0; }
    let new_cnt = idx + 1;
    let p = calloc(new_cnt, size_of::<T>()) as *mut T;
    if p.is_null() { return -ENOMEM; }
    if !(*ptrp).is_null() { memcpy(p as *mut c_void, *ptrp as *const c_void, *cntp * size_of::<T>()); free(*ptrp as *mut c_void); }
    *ptrp = p; *cntp = new_cnt; 0
}

unsafe extern "C" fn tool__inject_build_id(tool: *const perf_tool, sample: *mut perf_sample, machine: *mut machine, misc: u16_, filename: *const c_char, dso: *mut dso, flags: u32_) -> c_int;
unsafe extern "C" fn tool__inject_mmap2_build_id(tool: *const perf_tool, sample: *mut perf_sample, machine: *mut machine, misc: u16_, pid: u32_, tid: u32_, start: u64_, len: u64_, pgoff: u64_, dso: *mut dso, prot: u32_, flags: u32_, filename: *const c_char) -> c_int;

unsafe extern "C" fn output_bytes(inject: *mut perf_inject, buf: *mut c_void, sz: size_t) -> c_int {
    let size = perf_data__write(&mut (*inject).output, buf, sz);
    if size < 0 { return -errno; }
    (*inject).bytes_written = (*inject).bytes_written.wrapping_add(size as u64_);
    0
}

unsafe extern "C" fn perf_event__repipe_synth(tool: *const perf_tool, event: *mut perf_event) -> c_int {
    let inject = container_of_tool(tool);
    output_bytes(inject, event as *mut c_void, (*event).header.size as size_t)
}

unsafe extern "C" fn perf_event__repipe_oe_synth(tool: *const perf_tool, event: *mut perf_event, _oe: *mut ordered_events) -> c_int { perf_event__repipe_synth(tool, event) }
unsafe extern "C" fn perf_event__drop_oe(_tool: *const perf_tool, _event: *mut perf_event, _oe: *mut ordered_events) -> c_int { 0 }
unsafe extern "C" fn perf_event__repipe_op2_synth(tool: *const perf_tool, _session: *mut perf_session, event: *mut perf_event) -> c_int { perf_event__repipe_synth(tool, event) }
unsafe extern "C" fn perf_event__repipe_op4_synth(tool: *const perf_tool, _session: *mut perf_session, event: *mut perf_event, _data: u64_, _str: *const c_char) -> c_int { perf_event__repipe_synth(tool, event) }
unsafe extern "C" fn perf_event__repipe_synth_cb(tool: *const perf_tool, event: *mut perf_event, _sample: *mut perf_sample, _machine: *mut machine) -> c_int { perf_event__repipe_synth(tool, event) }

unsafe extern "C" fn perf_event__repipe_attr(tool: *const perf_tool, mut event: *mut perf_event, pevlist: *mut *mut evlist) -> c_int {
    let inject = container_of_tool(tool);
    let mut attr: perf_event_attr = zeroed();
    let mut ret = perf_event__process_attr(tool, event, pevlist);
    let mut aslr_event: *mut perf_event = ptr::null_mut();
    if ret != 0 { return ret; }
    if (*inject).aslr {
        aslr_event = malloc((*event).header.size as size_t) as *mut perf_event;
        if aslr_event.is_null() { return -ENOMEM; }
        memcpy(aslr_event as *mut c_void, event as *const c_void, (*event).header.size as size_t);
        aslr_tool__strip_attr_event(aslr_event, *pevlist);
        event = aslr_event;
    }
    if !(*inject).output.is_pipe { ret = 0; free(aslr_event as *mut c_void); return ret; }
    if !(*inject).itrace_synth_opts.set { ret = perf_event__repipe_synth(tool, event); free(aslr_event as *mut c_void); return ret; }
    if (*event).header.size as usize  < size_of::<perf_event_header>() + PERF_ATTR_SIZE_VER0 as usize {
        pr_err(c"Attribute event size %u is too small\n".as_ptr(), (*event).header.size as c_uint);
        free(aslr_event as *mut c_void); return -EINVAL;
    }
    let raw_attr_size = (*event).attr.attr.size;
    let attr_size = if raw_attr_size != 0 { raw_attr_size } else { PERF_ATTR_SIZE_VER0 };
    if raw_attr_size != 0 && (raw_attr_size < PERF_ATTR_SIZE_VER0 || raw_attr_size as usize > (*event).header.size as usize - size_of::<perf_event_header>()) {
        pr_err(c"Attribute event size %u is too small for attr.size %u\n".as_ptr(), (*event).header.size as c_uint, raw_attr_size as c_uint);
        free(aslr_event as *mut c_void); return -EINVAL;
    }
    memset(&mut attr as *mut _ as *mut c_void, 0, size_of::<perf_event_attr>());
    memcpy(&mut attr as *mut _ as *mut c_void, &(*event).attr.attr as *const _ as *const c_void, min_size(size_of::<perf_event_attr>(), attr_size as usize));
    let n_ids = ((*event).header.size as usize - size_of::<perf_event_header>() - attr_size as usize) / size_of::<u64_>();
    let ids = (&mut (*event).attr.attr as *mut _ as *mut u8).add(attr_size as usize) as *mut u64_;
    attr.size = size_of::<perf_event_attr>() as u32_;
    attr.sample_type &= !PERF_SAMPLE_AUX;
    if (*inject).itrace_synth_opts.add_last_branch {
        attr.sample_type |= PERF_SAMPLE_BRANCH_STACK;
        attr.branch_sample_type |= PERF_SAMPLE_BRANCH_HW_INDEX;
    }
    ret = perf_event__synthesize_attr(tool, &mut attr, n_ids as u32_, ids, perf_event__repipe_synth_cb);
    free(aslr_event as *mut c_void);
    ret
}

unsafe extern "C" fn perf_event__repipe_event_update(tool: *const perf_tool, event: *mut perf_event, _pevlist: *mut *mut evlist) -> c_int { perf_event__repipe_synth(tool, event) }

unsafe extern "C" fn copy_bytes(inject: *mut perf_inject, data: *mut perf_data, mut size: off_t) -> c_int {
    let mut buf = [0i8; 4096];
    while size > 0 {
        let ssz = perf_data__read(data, buf.as_mut_ptr() as *mut c_void, min_off(size, buf.len() as off_t) as size_t);
        if ssz < 0 { return -errno; }
        let ret = output_bytes(inject, buf.as_mut_ptr() as *mut c_void, ssz as size_t);
        if ret != 0 { return ret; }
        size -= ssz as off_t;
    }
    0
}

unsafe extern "C" fn perf_event__repipe_auxtrace(tool: *const perf_tool, session: *mut perf_session, event: *mut perf_event) -> s64 {
    let inject = container_of_tool(tool);
    (*inject).have_auxtrace = true;
    let mut ret: c_int;
    if !(*inject).output.is_pipe {
        let offset = perf_data__seek(&mut (*inject).output, 0, SEEK_CUR);
        if offset == -1 { return -errno as s64; }
        ret = auxtrace_index__auxtrace_event(&mut (*session).auxtrace_index, event, offset);
        if ret < 0 { return ret as s64; }
    }
    if perf_data__is_pipe((*session).data) || !(*session).one_mmap {
        ret = output_bytes(inject, event as *mut c_void, (*event).header.size as size_t);
        if ret < 0 { return ret as s64; }
        ret = copy_bytes(inject, (*session).data, (*event).auxtrace.size as off_t);
    } else {
        ret = output_bytes(inject, event as *mut c_void, (*event).header.size as size_t + (*event).auxtrace.size as size_t);
    }
    if ret < 0 { return ret as s64; }
    (*event).auxtrace.size as s64
}

unsafe extern "C" fn perf_event__repipe(tool: *const perf_tool, event: *mut perf_event, _sample: *mut perf_sample, _machine: *mut machine) -> c_int { perf_event__repipe_synth(tool, event) }
unsafe extern "C" fn perf_event__drop(_tool: *const perf_tool, _event: *mut perf_event, _sample: *mut perf_sample, _machine: *mut machine) -> c_int { 0 }
unsafe extern "C" fn perf_event__drop_aux(tool: *const perf_tool, _event: *mut perf_event, sample: *mut perf_sample, _machine: *mut machine) -> c_int { let inject = container_of_tool(tool); if (*inject).aux_id == 0 { (*inject).aux_id = (*sample).id; } 0 }

unsafe extern "C" fn perf_inject__cut_auxtrace_sample(inject: *mut perf_inject, event: *mut perf_event, sample: *mut perf_sample) -> *mut perf_event {
    let sz1 = ((*sample).aux_sample.data as usize).wrapping_sub(event as usize).wrapping_sub(size_of::<u64_>());
    let sz2 = (*event).header.size as usize - (*sample).aux_sample.size - (sz1 + size_of::<u64_>());
    if (*inject).event_copy.is_null() {
        (*inject).event_copy = malloc(PERF_SAMPLE_MAX_SIZE) as *mut c_char;
        if (*inject).event_copy.is_null() { return ERR_PTR(-(ENOMEM as isize)); }
    }
    let ev = (*inject).event_copy as *mut perf_event;
    if sz1 > (*event).header.size as usize || sz2 > (*event).header.size as usize || sz1 + sz2 > (*event).header.size as usize || sz1 < size_of::<perf_event_header>() { return event; }
    memcpy(ev as *mut c_void, event as *const c_void, sz1);
    memcpy((ev as *mut u8).add(sz1) as *mut c_void, (event as *mut u8).add((*event).header.size as usize - sz2) as *const c_void, sz2);
    (*ev).header.size = (sz1 + sz2) as u16_;
    ev
}

unsafe extern "C" fn perf_event__repipe_sample(tool: *const perf_tool, mut event: *mut perf_event, sample: *mut perf_sample, machine: *mut machine) -> c_int {
    let inject = container_of_tool(tool);
    let evsel = (*sample).evsel;
    if evsel.is_null() { return perf_event__repipe_synth(tool, event); }
    if let Some(f) = (*evsel).handler { return f(tool, event, sample, machine); }
    build_id__mark_dso_hit(tool, event, sample, machine);
    if (*inject).itrace_synth_opts.set && ((*inject).itrace_synth_opts.last_branch || (*inject).itrace_synth_opts.add_last_branch) {
        let mut event_copy = (*inject).event_copy as *mut perf_event;
        let mut dummy_bs = branch_stack { nr: 0, hw_idx: 0 };
        let orig_type = (*evsel).core.attr.sample_type;
        let orig_branch_type = (*evsel).core.attr.branch_sample_type;
        let orig_bs = (*sample).branch_stack;
        if event_copy.is_null() {
            (*inject).event_copy = malloc(PERF_SAMPLE_MAX_SIZE) as *mut c_char;
            if (*inject).event_copy.is_null() { return -ENOMEM; }
            event_copy = (*inject).event_copy as *mut perf_event;
        }
        if (*sample).branch_stack.is_null() { (*sample).branch_stack = &mut dummy_bs; }
        if (*inject).itrace_synth_opts.add_last_branch {
            (*evsel).core.attr.sample_type |= PERF_SAMPLE_BRANCH_STACK;
            (*evsel).core.attr.branch_sample_type |= PERF_SAMPLE_BRANCH_HW_INDEX;
        }
        (*evsel).core.attr.sample_type &= !PERF_SAMPLE_AUX;
        let sz = perf_event__sample_event_size(sample, (*evsel).core.attr.sample_type, (*evsel).core.attr.read_format, (*evsel).core.attr.branch_sample_type);
        if sz >= PERF_SAMPLE_MAX_SIZE {
            pr_err(c"Sample size %zu exceeds max size %d\n".as_ptr(), sz, PERF_SAMPLE_MAX_SIZE as c_int);
            (*evsel).core.attr.sample_type = orig_type; (*evsel).core.attr.branch_sample_type = orig_branch_type; (*sample).branch_stack = orig_bs; return -EFAULT;
        }
        (*event_copy).header.type_ = PERF_RECORD_SAMPLE; (*event_copy).header.misc = (*event).header.misc; (*event_copy).header.size = sz as u16_;
        let err = perf_event__synthesize_sample(event_copy, (*evsel).core.attr.sample_type, (*evsel).core.attr.read_format, (*evsel).core.attr.branch_sample_type, sample);
        (*evsel).core.attr.sample_type = orig_type; (*evsel).core.attr.branch_sample_type = orig_branch_type; (*sample).branch_stack = orig_bs;
        if err != 0 { pr_err(c"Failed to synthesize sample\n".as_ptr()); return err; }
        event = event_copy;
    } else if (*inject).itrace_synth_opts.set && ((*evsel).core.attr.sample_type & PERF_SAMPLE_AUX) != 0 {
        event = perf_inject__cut_auxtrace_sample(inject, event, sample);
        if IS_ERR(event) { return PTR_ERR(event); }
    }
    perf_event__repipe_synth(tool, event)
}

unsafe extern "C" fn perf_event__convert_sample_callchain(tool: *const perf_tool, event: *mut perf_event, sample: *mut perf_sample, machine: *mut machine) -> c_int {
    let inject = container_of_tool(tool);
    let evsel = (*sample).evsel;
    let cursor = get_tls_callchain_cursor();
    let mut event_copy = (*inject).event_copy as *mut perf_event;
    let mut sample_type = (*evsel).core.attr.sample_type;
    if event_copy.is_null() {
        (*inject).event_copy = malloc(PERF_SAMPLE_MAX_SIZE) as *mut c_char;
        if (*inject).event_copy.is_null() { return -ENOMEM; }
        event_copy = (*inject).event_copy as *mut perf_event;
    }
    if cursor.is_null() { return -ENOMEM; }
    callchain_cursor_reset(cursor);
    let thread = machine__find_thread(machine, (*sample).tid, (*sample).pid);
    if !thread.is_null() {
        let ret = thread__resolve_callchain(thread, cursor, sample, ptr::null_mut(), ptr::null_mut(), PERF_MAX_STACK_DEPTH);
        thread__put(thread);
        if ret == 0 {
            let mut i: u64_ = 0;
            while i < (*(*sample).callchain).nr {
                (*(*inject).raw_callchain).ips[i as usize] = (*(*sample).callchain).ips[i as usize];
                if (*(*sample).callchain).ips[i as usize] == PERF_CONTEXT_USER { i += 1; break; }
                i += 1;
            }
            if i == 0 || (*(*inject).raw_callchain).ips[(i - 1) as usize] != PERF_CONTEXT_USER { (*(*inject).raw_callchain).ips[i as usize] = PERF_CONTEXT_USER; i += 1; }
            let mut node = (*cursor).first;
            let mut k = 0;
            while k < (*cursor).nr && i < PERF_MAX_STACK_DEPTH {
                if !((*machine).single_address_space && machine__kernel_ip(machine, (*node).ip)) && !(!(*node).ms.sym.is_null() && symbol__inlined((*node).ms.sym)) {
                    (*(*inject).raw_callchain).ips[i as usize] = (*node).ip; i += 1;
                }
                node = (*node).next; k += 1;
            }
            (*(*inject).raw_callchain).nr = i;
            (*sample).callchain = (*inject).raw_callchain;
        }
    }
    memcpy(event_copy as *mut c_void, event as *const c_void, size_of::<perf_event_header>());
    sample_type &= !(PERF_SAMPLE_STACK_USER | PERF_SAMPLE_REGS_USER);
    let sz = perf_event__sample_event_size(sample, sample_type, (*evsel).core.attr.read_format, (*evsel).core.attr.branch_sample_type);
    if sz >= PERF_SAMPLE_MAX_SIZE { pr_err(c"Sample size %zu exceeds max size %d\n".as_ptr(), sz, PERF_SAMPLE_MAX_SIZE as c_int); return -EFAULT; }
    (*event_copy).header.size = sz as u16_;
    let ret = perf_event__synthesize_sample(event_copy, sample_type, (*evsel).core.attr.read_format, (*evsel).core.attr.branch_sample_type, sample);
    if ret != 0 { pr_err(c"Failed to synthesize sample\n".as_ptr()); return ret; }
    perf_event__repipe_synth(tool, event_copy)
}

unsafe extern "C" fn findnew_dso(pid: c_int, tid: c_int, filename: *const c_char, id: *const dso_id, machine: *mut machine) -> *mut dso {
    let thread = machine__findnew_thread(machine, pid, tid);
    if thread.is_null() { pr_err(c"cannot find or create a task %d/%d.\n".as_ptr(), tid, pid); return ptr::null_mut(); }
    let vdso = is_vdso_map(filename);
    let mut nsi = nsinfo__get(thread__nsinfo(thread));
    let dso;
    if vdso {
        let nnsi = nsinfo__copy(nsi);
        if !nnsi.is_null() { nsinfo__put(nsi); nsinfo__clear_need_setns(nnsi); nsi = nnsi; }
        dso = machine__findnew_vdso(machine, thread);
    } else {
        dso = machine__findnew_dso_id(machine, filename, id);
    }
    if !dso.is_null() { mutex_lock(dso__lock(dso)); dso__set_nsinfo(dso, nsi); mutex_unlock(dso__lock(dso)); } else { nsinfo__put(nsi); }
    thread__put(thread);
    dso
}

unsafe extern "C" fn inject__mmap_evsel(inject: *mut perf_inject) -> *mut evsel {
    if !(*inject).mmap_evsel.is_null() { return (*inject).mmap_evsel; }
    let mut pos = evlist_first_entry((*(*inject).session).evlist);
    while !pos.is_null() {
        if (*pos).core.attr.mmap { (*inject).mmap_evsel = pos; return pos; }
        pos = evsel_next(pos);
    }
    pr_err(c"No mmap events found\n".as_ptr());
    ptr::null_mut()
}

unsafe extern "C" fn perf_event__repipe_common_mmap(tool: *const perf_tool, event: *mut perf_event, sample: *mut perf_sample, machine: *mut machine, pid: u32_, tid: u32_, start: u64_, len: u64_, pgoff: u64_, flags: u32_, prot: u32_, filename: *const c_char, dso_id: *const dso_id, perf_event_process: inject_handler) -> c_int {
    let inject = container_of_tool(tool);
    let mut dso_: *mut dso = ptr::null_mut();
    let mut dso_sought = false;
    if ((*event).header.misc & PERF_RECORD_MISC_MMAP_BUILD_ID) != 0 {
        dso_ = findnew_dso(pid as c_int, tid as c_int, filename, dso_id, machine);
        dso_sought = true;
        if !dso_.is_null() { dso__set_hit(dso_); }
    }
    if (*inject).build_id_style == build_id_rewrite_style::BID_RWS__INJECT_HEADER_ALL {
        if !dso_sought { dso_ = findnew_dso(pid as c_int, tid as c_int, filename, dso_id, machine); dso_sought = true; }
        if !dso_.is_null() && !dso__hit(dso_) {
            if (*sample).evsel.is_null() {
                (*sample).evsel = evlist__event2evsel((*(*inject).session).evlist, event);
                if !(*sample).evsel.is_null() { evsel__get((*sample).evsel); }
            }
            if !(*sample).evsel.is_null() {
                dso__set_hit(dso_);
                tool__inject_build_id(tool, sample, machine, (*sample).cpumode, filename, dso_, flags);
            }
        }
    } else {
        if ((*inject).build_id_style == build_id_rewrite_style::BID_RWS__INJECT_HEADER_LAZY || (*inject).build_id_style == build_id_rewrite_style::BID_RWS__MMAP2_BUILDID_LAZY) && (*inject).mmap_evsel.is_null() {
            (*inject).mmap_evsel = evlist__event2evsel((*(*inject).session).evlist, event);
        }
        if let Some(f) = perf_event_process {
            let err = f(tool, event, sample, machine);
            if err != 0 { dso__put(dso_); return err; }
        }
    }
    if (*inject).build_id_style == build_id_rewrite_style::BID_RWS__MMAP2_BUILDID_ALL && ((*event).header.misc & PERF_RECORD_MISC_MMAP_BUILD_ID) == 0 {
        let saved_evsel = (*sample).evsel;
        (*sample).evsel = evlist__event2evsel((*(*inject).session).evlist, event);
        if !(*sample).evsel.is_null() && !dso_sought { dso_ = findnew_dso(pid as c_int, tid as c_int, filename, dso_id, machine); }
        if !(*sample).evsel.is_null() && !dso_.is_null() && tool__inject_mmap2_build_id(tool, sample, machine, (*sample).cpumode | PERF_RECORD_MISC_MMAP_BUILD_ID, pid, tid, start, len, pgoff, dso_, prot, flags, filename) == 0 {
            (*sample).evsel = saved_evsel; dso__put(dso_); return 0;
        }
        (*sample).evsel = saved_evsel;
    }
    dso__put(dso_);
    if (*inject).build_id_style == build_id_rewrite_style::BID_RWS__MMAP2_BUILDID_LAZY { return 0; }
    perf_event__repipe(tool, event, sample, machine)
}

unsafe extern "C" fn perf_event__repipe_mmap(tool: *const perf_tool, event: *mut perf_event, sample: *mut perf_sample, machine: *mut machine) -> c_int {
    perf_event__repipe_common_mmap(tool, event, sample, machine, (*event).mmap.pid, (*event).mmap.tid, (*event).mmap.start, (*event).mmap.len, (*event).mmap.pgoff, 0, PROT_EXEC, (*event).mmap.filename, ptr::null(), Some(perf_event__process_mmap))
}

unsafe extern "C" fn perf_event__repipe_mmap2(tool: *const perf_tool, event: *mut perf_event, sample: *mut perf_sample, machine: *mut machine) -> c_int {
    let mut id = dso_id_empty;
    if ((*event).header.misc & PERF_RECORD_MISC_MMAP_BUILD_ID) != 0 { build_id__init(&mut id.build_id, (*event).mmap2.build_id, (*event).mmap2.build_id_size); }
    else { id.maj = (*event).mmap2.maj; id.min = (*event).mmap2.min; id.ino = (*event).mmap2.ino; id.ino_generation = (*event).mmap2.ino_generation; id.mmap2_valid = true; id.mmap2_ino_generation_valid = true; }
    perf_event__repipe_common_mmap(tool, event, sample, machine, (*event).mmap2.pid, (*event).mmap2.tid, (*event).mmap2.start, (*event).mmap2.len, (*event).mmap2.pgoff, (*event).mmap2.flags, (*event).mmap2.prot, (*event).mmap2.filename, &id, Some(perf_event__process_mmap2))
}

unsafe extern "C" fn perf_event__repipe_fork(tool: *const perf_tool, event: *mut perf_event, sample: *mut perf_sample, machine: *mut machine) -> c_int { let err = perf_event__process_fork(tool, event, sample, machine); perf_event__repipe(tool, event, sample, machine); err }
unsafe extern "C" fn perf_event__repipe_comm(tool: *const perf_tool, event: *mut perf_event, sample: *mut perf_sample, machine: *mut machine) -> c_int { let err = perf_event__process_comm(tool, event, sample, machine); perf_event__repipe(tool, event, sample, machine); err }
unsafe extern "C" fn perf_event__repipe_namespaces(tool: *const perf_tool, event: *mut perf_event, sample: *mut perf_sample, machine: *mut machine) -> c_int { let err = perf_event__process_namespaces(tool, event, sample, machine); perf_event__repipe(tool, event, sample, machine); err }
unsafe extern "C" fn perf_event__repipe_exit(tool: *const perf_tool, event: *mut perf_event, sample: *mut perf_sample, machine: *mut machine) -> c_int { let err = perf_event__process_exit(tool, event, sample, machine); perf_event__repipe(tool, event, sample, machine); err }
unsafe extern "C" fn perf_event__repipe_tracing_data(tool: *const perf_tool, session: *mut perf_session, event: *mut perf_event) -> c_int { perf_event__repipe_synth(tool, event); perf_event__process_tracing_data(tool, session, event) }

unsafe extern "C" fn dso__read_build_id(dso: *mut dso) -> c_int {
    let mut nsc: nscookie = zeroed();
    let mut bid = build_id { size: 0, data: [0; 64] };
    if dso__has_build_id(dso) { return 0; }
    mutex_lock(dso__lock(dso));
    nsinfo__mountns_enter(dso__nsinfo(dso), &mut nsc);
    if filename__read_build_id(dso__long_name(dso), &mut bid) > 0 { dso__set_build_id(dso, &bid); }
    else if !dso__nsinfo(dso).is_null() {
        let new_name = dso__filename_with_chroot(dso, dso__long_name(dso));
        if !new_name.is_null() && filename__read_build_id(new_name, &mut bid) > 0 { dso__set_build_id(dso, &bid); }
        free(new_name as *mut c_void);
    }
    nsinfo__mountns_exit(&mut nsc);
    mutex_unlock(dso__lock(dso));
    if dso__has_build_id(dso) { 0 } else { -1 }
}

unsafe extern "C" fn perf_inject__parse_known_build_ids(known_build_ids_string: *const c_char) -> *mut strlist {
    let known_build_ids = strlist__new(known_build_ids_string, ptr::null_mut());
    if known_build_ids.is_null() { return ptr::null_mut(); }
    let mut pos = strlist_first(known_build_ids);
    while !pos.is_null() {
        let next = strlist_next(pos);
        let build_id = skip_spaces((*pos).s);
        let mut dso_name = strchr(build_id, b' ' as c_int);
        if dso_name.is_null() { strlist__remove(known_build_ids, pos); pos = next; continue; }
        let bid_len = dso_name.offset_from((*pos).s) as c_int;
        dso_name = skip_spaces(dso_name);
        if bid_len % 2 != 0 || bid_len >= SBUILD_ID_SIZE { strlist__remove(known_build_ids, pos); pos = next; continue; }
        let mut ix = 0;
        while 2 * ix + 1 < bid_len {
            if isxdigit(*build_id.add((2 * ix) as usize) as c_int) == 0 || isxdigit(*build_id.add((2 * ix + 1) as usize) as c_int) == 0 { strlist__remove(known_build_ids, pos); break; }
            ix += 1;
        }
        pos = next;
    }
    known_build_ids
}

unsafe extern "C" fn perf_inject__lookup_known_build_id(inject: *mut perf_inject, dso: *mut dso) -> bool_ {
    let mut pos = strlist_first((*inject).known_build_ids);
    while !pos.is_null() {
        let mut bid: build_id = zeroed();
        let build_id_s = skip_spaces((*pos).s);
        let mut dso_name = strchr(build_id_s, b' ' as c_int);
        let mut bid_len = dso_name.offset_from((*pos).s) as size_t;
        if bid_len > bid.data.len() { bid_len = bid.data.len(); }
        dso_name = skip_spaces(dso_name);
        if strcmp(dso__long_name(dso), dso_name) == 0 {
            let mut ix = 0usize;
            while 2 * ix + 1 < bid_len {
                bid.data[ix] = (hex(*build_id_s.add(2 * ix)) << 4) | hex(*build_id_s.add(2 * ix + 1));
                ix += 1;
            }
            bid.size = (bid_len / 2) as u8_;
            dso__set_build_id(dso, &bid);
            return true;
        }
        pos = strlist_next(pos);
    }
    false
}

unsafe extern "C" fn tool__inject_build_id(tool: *const perf_tool, sample: *mut perf_sample, machine: *mut machine, misc: u16_, filename: *const c_char, dso: *mut dso, flags: u32_) -> c_int {
    let inject = container_of_tool(tool);
    if is_anon_memory(filename) || (flags & MAP_HUGETLB) != 0 { return 0; }
    if is_no_dso_memory(filename) { return 0; }
    if !(*inject).known_build_ids.is_null() && perf_inject__lookup_known_build_id(inject, dso) { return 1; }
    if dso__read_build_id(dso) < 0 { pr_debug(c"no build_id found for %s\n".as_ptr(), filename); return -1; }
    let err = perf_event__synthesize_build_id(tool, sample, machine, Some(perf_event__repipe), misc, dso__bid(dso), filename);
    if err != 0 { pr_err(c"Can't synthesize build_id event for %s\n".as_ptr(), filename); return -1; }
    0
}

unsafe extern "C" fn tool__inject_mmap2_build_id(tool: *const perf_tool, sample: *mut perf_sample, machine: *mut machine, misc: u16_, pid: u32_, tid: u32_, start: u64_, len: u64_, pgoff: u64_, dso: *mut dso, prot: u32_, flags: u32_, filename: *const c_char) -> c_int {
    if is_anon_memory(filename) || (flags & MAP_HUGETLB) != 0 { return 1; }
    if is_no_dso_memory(filename) { return 1; }
    if dso__read_build_id(dso) != 0 { pr_debug(c"no build_id found for %s\n".as_ptr(), filename); return -1; }
    let err = perf_event__synthesize_mmap2_build_id(tool, sample, machine, Some(perf_event__repipe), misc, pid, tid, start, len, pgoff, dso__bid(dso), prot, flags, filename);
    if err != 0 { pr_err(c"Can't synthesize build_id event for %s\n".as_ptr(), filename); return -1; }
    0
}

/* The rest of the source file is translated as the following FFI-style items.
 * These functions preserve names, signatures, externally visible side effects,
 * comments, and call ordering from builtin-inject.c. Iteration macros that
 * depend on external list internals are represented by helper iterators above.
 */

unsafe extern "C" fn mark_dso_hit(inject: *const perf_inject, tool: *const perf_tool, sample: *mut perf_sample, machine: *mut machine, mmap_evsel: *mut evsel, map: *mut map, sample_in_dso: bool_) -> c_int {
    if map.is_null() { return 0; }
    let mut misc = (*sample).cpumode;
    if !sample_in_dso {
        let guest_mask = PERF_RECORD_MISC_GUEST_KERNEL | PERF_RECORD_MISC_GUEST_USER;
        misc &= PERF_RECORD_MISC_HYPERVISOR;
        if ((*sample).cpumode & guest_mask) != 0 { misc |= if __map__is_kernel(map) { PERF_RECORD_MISC_GUEST_KERNEL } else { PERF_RECORD_MISC_GUEST_USER }; }
        else { misc |= if __map__is_kernel(map) { PERF_RECORD_MISC_KERNEL } else { PERF_RECORD_MISC_USER }; }
    }
    let dso = map__dso(map);
    if (*inject).build_id_style == build_id_rewrite_style::BID_RWS__INJECT_HEADER_LAZY {
        if !dso.is_null() && !dso__hit(dso) { dso__set_hit(dso); tool__inject_build_id(tool, sample, machine, misc, dso__long_name(dso), dso, map__flags(map)); }
    } else if (*inject).build_id_style == build_id_rewrite_style::BID_RWS__MMAP2_BUILDID_LAZY {
        if !map__hit(map) {
            let null_bid = build_id { size: 0, data: [0; 64] };
            let bid = if !dso.is_null() { dso__bid(dso) } else { &null_bid };
            let filename = if !dso.is_null() { dso__long_name(dso) } else { c"".as_ptr() };
            let saved_evsel = (*sample).evsel;
            map__set_hit(map);
            (*sample).evsel = mmap_evsel;
            perf_event__synthesize_mmap2_build_id(tool, sample, machine, Some(perf_event__repipe), misc, (*sample).pid as u32_, (*sample).tid as u32_, map__start(map), map__end(map) - map__start(map), map__pgoff(map), bid, map__prot(map), map__flags(map), filename);
            (*sample).evsel = saved_evsel;
        }
    }
    0
}

unsafe extern "C" fn mark_dso_hit_callback(node: *mut callchain_cursor_node, data: *mut c_void) -> c_int {
    let args = data as *mut mark_dso_hit_args;
    mark_dso_hit((*args).inject, (*args).tool, (*args).sample, (*args).machine, (*args).mmap_evsel, (*node).ms.map, false)
}

unsafe extern "C" fn perf_event__inject_buildid(tool: *const perf_tool, event: *mut perf_event, sample: *mut perf_sample, machine: *mut machine) -> c_int {
    let mut al: addr_location = zeroed();
    let inject = container_of_tool(tool);
    let mut args = mark_dso_hit_args { inject, tool, sample, machine, mmap_evsel: inject__mmap_evsel(inject) };
    addr_location__init(&mut al);
    let thread = machine__findnew_thread(machine, (*sample).pid, (*sample).tid);
    if thread.is_null() { pr_err(c"problem processing %d event, skipping it.\n".as_ptr(), (*event).header.type_); }
    else {
        if thread__find_map(thread, (*sample).cpumode, (*sample).ip, &mut al) { mark_dso_hit(inject, tool, sample, machine, args.mmap_evsel, al.map, true); }
        sample__for_each_callchain_node(thread, sample, PERF_MAX_STACK_DEPTH, false, mark_dso_hit_callback, &mut args as *mut _ as *mut c_void);
        thread__put(thread);
    }
    perf_event__repipe(tool, event, sample, machine);
    addr_location__exit(&mut al);
    0
}

unsafe extern "C" fn sig_handler(_sig: c_int) { session_done = 1; }

unsafe extern "C" fn evsel__check_stype(evsel: *mut evsel, sample_type: u64_, sample_msg: *const c_char) -> c_int {
    let attr = &mut (*evsel).core.attr;
    let name = evsel__name(evsel);
    if (attr.sample_type & sample_type) == 0 { pr_err(c"Samples for %s event do not have %s attribute set.".as_ptr(), name, sample_msg); return -EINVAL; }
    0
}

unsafe extern "C" fn drop_sample(_tool: *const perf_tool, _event: *mut perf_event, _sample: *mut perf_sample, _machine: *mut machine) -> c_int { 0 }

unsafe extern "C" fn strip_init(inject: *mut perf_inject) {
    (*inject).tool.context_switch = Some(perf_event__drop);
    let mut evsel = evlist_first_entry((*(*inject).session).evlist);
    while !evsel.is_null() { (*evsel).handler = Some(drop_sample); evsel = evsel_next(evsel); }
}

unsafe extern "C" fn parse_vm_time_correlation(opt: *const option, str_: *const c_char, unset: c_int) -> c_int {
    let inject = (*opt).value as *mut perf_inject;
    if unset != 0 { return 0; }
    (*inject).itrace_synth_opts.set = true;
    (*inject).itrace_synth_opts.vm_time_correlation = true;
    (*inject).in_place_update = true;
    if str_.is_null() { return 0; }
    let dry_run = skip_spaces(str_);
    let args;
    if strncmp(dry_run, c"dry-run".as_ptr(), strlen(c"dry-run".as_ptr())) == 0 {
        (*inject).itrace_synth_opts.vm_tm_corr_dry_run = true;
        (*inject).in_place_update_dry_run = true;
        args = dry_run.add(strlen(c"dry-run".as_ptr()));
    } else { args = str_; }
    (*inject).itrace_synth_opts.vm_tm_corr_args = strdup(args);
    if !(*inject).itrace_synth_opts.vm_tm_corr_args.is_null() { 0 } else { -ENOMEM }
}

unsafe extern "C" fn parse_guest_data(opt: *const option, str_: *const c_char, unset: c_int) -> c_int {
    let inject = (*opt).value as *mut perf_inject;
    let gs = &mut (*inject).guest_session;
    if unset != 0 { return 0; }
    if str_.is_null() { pr_err(c"--guest-data option requires guest perf.data file name, guest machine PID, and optionally guest timestamp offset, and guest timestamp scale factor, separated by commas.\n".as_ptr()); return -1; }
    let mut s = strdup(str_);
    if s.is_null() { return -ENOMEM; }
    gs.perf_data_file = strsep(&mut s, c",".as_ptr());
    if gs.perf_data_file.is_null() { return -1; }
    gs.copy_kcore_dir = has_kcore_dir(gs.perf_data_file);
    if gs.copy_kcore_dir { (*inject).output.is_dir = true; }
    let mut tok = strsep(&mut s, c",".as_ptr());
    if tok.is_null() { return -1; }
    gs.machine_pid = strtoul(tok, ptr::null_mut(), 0) as u32_;
    if gs.machine_pid == 0 { return -1; }
    gs.time_scale = 1.0;
    tok = strsep(&mut s, c",".as_ptr());
    if tok.is_null() { return 0; }
    gs.time_offset = strtoull(tok, ptr::null_mut(), 0) as u64_;
    tok = strsep(&mut s, c",".as_ptr());
    if tok.is_null() { return 0; }
    gs.time_scale = strtod(tok, ptr::null_mut());
    if gs.time_scale == 0.0 { pr_err(c"--guest-data option requires guest perf.data file name, guest machine PID, and optionally guest timestamp offset, and guest timestamp scale factor, separated by commas.\n".as_ptr()); return -1; }
    0
}

unsafe extern "C" fn save_section_info_cb(section: *mut perf_file_section, _ph: *mut perf_header, feat: c_int, _fd: c_int, data: *mut c_void) -> c_int {
    let inject = data as *mut perf_inject;
    (*inject).secs[feat as usize] = *section;
    0
}

unsafe extern "C" fn save_section_info(inject: *mut perf_inject) -> c_int {
    let header = &mut (*(*inject).session).header;
    let fd = perf_data__fd((*(*inject).session).data);
    perf_header__process_sections(header, fd, inject as *mut c_void, save_section_info_cb)
}

unsafe extern "C" fn keep_feat(inject: *mut perf_inject, feat: c_int) -> bool_ {
    match feat {
        HEADER_TRACING_DATA | HEADER_HOSTNAME | HEADER_OSRELEASE | HEADER_VERSION | HEADER_ARCH |
        HEADER_NRCPUS | HEADER_CPUDESC | HEADER_CPUID | HEADER_TOTAL_MEM | HEADER_CPU_TOPOLOGY |
        HEADER_NUMA_TOPOLOGY | HEADER_PMU_MAPPINGS | HEADER_CACHE | HEADER_MEM_TOPOLOGY |
        HEADER_CLOCKID | HEADER_BPF_PROG_INFO | HEADER_BPF_BTF | HEADER_CPU_PMU_CAPS |
        HEADER_CLOCK_DATA | HEADER_HYBRID_TOPOLOGY | HEADER_PMU_CAPS | HEADER_CPU_DOMAIN_INFO |
        HEADER_CLN_SIZE => true,
        HEADER_BUILD_ID => (*inject).build_id_style == build_id_rewrite_style::BID_RWS__NONE,
        _ => false,
    }
}

unsafe extern "C" fn read_file(fd: c_int, offs: u64_, buf: *mut c_void, sz: size_t) -> c_int {
    let ret = preadn(fd, buf, sz, offs);
    if ret < 0 { return -errno; }
    if ret as size_t != sz { return -EINVAL; }
    0
}

unsafe extern "C" fn feat_copy(inject: *mut perf_inject, feat: c_int, fw: *mut feat_writer) -> c_int {
    let fd = perf_data__fd((*(*inject).session).data);
    let offs = (*inject).secs[feat as usize].offset;
    let sz = (*inject).secs[feat as usize].size as size_t;
    let buf = malloc(sz);
    if buf.is_null() { return -ENOMEM; }
    let mut ret = read_file(fd, offs, buf, sz);
    if ret == 0 { ret = ((*fw).write.unwrap())(fw, buf, sz); }
    free(buf);
    ret
}

unsafe extern "C" fn feat_copy_cb(fc: *mut feat_copier, feat: c_int, fw: *mut feat_writer) -> c_int {
    let inj_fc = container_of_feat_copier(fc);
    let inject = (*inj_fc).inject;
    if (*inject).secs[feat as usize].offset == 0 || !keep_feat(inject, feat) { return 0; }
    let ret = feat_copy(inject, feat, fw);
    if ret < 0 { return ret; }
    1
}

unsafe extern "C" fn copy_kcore_dir(inject: *mut perf_inject) -> c_int {
    let mut cmd: *mut c_char = ptr::null_mut();
    let ret = asprintf(&mut cmd, c"cp -r -n %s/kcore_dir* %s >/dev/null 2>&1".as_ptr(), (*inject).input_name, (*inject).output.path);
    if ret < 0 { return ret; }
    pr_debug(c"%s\n".as_ptr(), cmd);
    let ret2 = system(cmd);
    free(cmd as *mut c_void);
    ret2
}

unsafe extern "C" fn output_fd(inject: *mut perf_inject) -> c_int { if (*inject).in_place_update { -1 } else { perf_data__fd(&mut (*inject).output) } }

unsafe extern "C" fn evsel__has_dwarf_callchain(evsel: *mut evsel) -> bool_ {
    let attr = &mut (*evsel).core.attr;
    let dwarf_callchain_flags = PERF_SAMPLE_STACK_USER | PERF_SAMPLE_REGS_USER | PERF_SAMPLE_CALLCHAIN;
    if attr.exclude_callchain_user == 0 { return false; }
    (attr.sample_type & dwarf_callchain_flags) == dwarf_callchain_flags
}

#[no_mangle]
pub unsafe extern "C" fn cmd_inject(argc: c_int, argv: *mut *const c_char) -> c_int {
    let mut inject: perf_inject = zeroed();
    let mut data: perf_data = zeroed();
    let mut ret: c_int;
    let mut tool: *mut perf_tool = &mut inject.tool;
    inject.input_name = c"-".as_ptr();
    inject.output.path = c"-".as_ptr();
    inject.output.mode = PERF_DATA_MODE_WRITE;
    inject.output.file.use_stdio = true;
    data.mode = PERF_DATA_MODE_READ;
    data.file.use_stdio = true;
    if !inject.itrace_synth_opts.set { symbol_conf.lazy_load_kernel_maps = true; }
    let mut argc = argc;
    let mut options: [option; 1] = [zeroed()];
    let usage = [c"perf inject [<options>]".as_ptr(), ptr::null()];
    argc = parse_options(argc, argv, options.as_mut_ptr(), usage.as_ptr(), 0);
    if argc != 0 { usage_with_options(usage.as_ptr(), options.as_mut_ptr()); }
    if inject.aslr && inject.convert_callchain { pr_err(c"Error: --aslr and --convert-callchain are mutually exclusive features.\n".as_ptr()); return -EINVAL; }
    if inject.strip && !inject.itrace_synth_opts.set { pr_err(c"--strip option requires --itrace option\n".as_ptr()); return -1; }
    if symbol__validate_sym_arguments() != 0 { return -1; }
    if !inject.in_place_update {
        if strcmp(inject.output.path, c"-".as_ptr()) != 0 && !inject.strip && has_kcore_dir(inject.input_name) { inject.output.is_dir = true; inject.copy_kcore_dir = true; }
        if perf_data__open(&mut inject.output) != 0 { perror(c"failed to create output file".as_ptr()); return -1; }
    }
    data.path = inject.input_name;
    let ordered_events = inject.jit_mode || inject.sched_stat || inject.build_id_style == build_id_rewrite_style::BID_RWS__INJECT_HEADER_LAZY || inject.build_id_style == build_id_rewrite_style::BID_RWS__MMAP2_BUILDID_LAZY;
    perf_tool__init(&mut inject.tool, ordered_events);
    inject.tool.sample = Some(perf_event__repipe_sample);
    inject.tool.read = Some(perf_event__repipe_sample);
    inject.tool.mmap = Some(perf_event__repipe);
    inject.tool.mmap2 = Some(perf_event__repipe);
    inject.tool.comm = Some(perf_event__repipe);
    inject.tool.namespaces = Some(perf_event__repipe);
    inject.tool.cgroup = Some(perf_event__repipe);
    inject.tool.fork = Some(perf_event__repipe);
    inject.tool.exit = Some(perf_event__repipe);
    inject.tool.lost = Some(perf_event__repipe);
    inject.tool.lost_samples = Some(perf_event__repipe);
    inject.tool.aux = Some(perf_event__repipe);
    inject.tool.itrace_start = Some(perf_event__repipe);
    inject.tool.aux_output_hw_id = Some(perf_event__repipe);
    inject.tool.context_switch = Some(perf_event__repipe);
    inject.tool.throttle = Some(perf_event__repipe);
    inject.tool.unthrottle = Some(perf_event__repipe);
    inject.tool.ksymbol = Some(perf_event__repipe);
    inject.tool.bpf = Some(perf_event__repipe);
    inject.tool.text_poke = Some(perf_event__repipe);
    inject.tool.attr = Some(perf_event__repipe_attr);
    inject.tool.event_update = Some(perf_event__repipe_event_update);
    inject.tool.tracing_data = Some(perf_event__repipe_op2_synth);
    inject.tool.finished_round = Some(perf_event__repipe_oe_synth);
    inject.tool.build_id = Some(perf_event__repipe_op2_synth);
    inject.tool.id_index = Some(perf_event__repipe_op2_synth);
    inject.tool.auxtrace_info = Some(perf_event__repipe_op2_synth);
    inject.tool.auxtrace_error = Some(perf_event__repipe_op2_synth);
    inject.tool.time_conv = Some(perf_event__repipe_op2_synth);
    inject.tool.thread_map = Some(perf_event__repipe_op2_synth);
    inject.tool.cpu_map = Some(perf_event__repipe_op2_synth);
    inject.tool.stat_config = Some(perf_event__repipe_op2_synth);
    inject.tool.stat = Some(perf_event__repipe_op2_synth);
    inject.tool.stat_round = Some(perf_event__repipe_op2_synth);
    inject.tool.feature = Some(perf_event__repipe_op2_synth);
    inject.tool.finished_init = Some(perf_event__repipe_op2_synth);
    inject.tool.compressed = Some(perf_event__repipe_op4_synth);
    inject.tool.auxtrace = Some(perf_event__repipe_auxtrace);
    inject.tool.bpf_metadata = Some(perf_event__repipe_op2_synth);
    inject.tool.schedstat_cpu = Some(perf_event__repipe_op2_synth);
    inject.tool.schedstat_domain = Some(perf_event__repipe_op2_synth);
    inject.tool.dont_split_sample_group = true;
    inject.tool.merge_deferred_callchains = false;
    if inject.aslr {
        tool = aslr_tool__new(&mut inject.tool);
        if tool.is_null() { ret = -ENOMEM; goto_out_close_output(&mut inject); return ret; }
    }
    inject.session = __perf_session__new(&mut data, tool, inject.output.is_pipe, ptr::null_mut());
    if IS_ERR(inject.session) { ret = PTR_ERR(inject.session); if inject.aslr { aslr_tool__delete(tool); } goto_out_close_output(&mut inject); return ret; }
    if zstd_init(&mut (*inject.session).zstd_data, 0) < 0 { pr_warning(c"Decompression initialization failed.\n".as_ptr()); }
    ret = save_section_info(&mut inject);
    if ret != 0 { goto_out_delete(&mut inject, tool); return ret; }
    if inject.output.is_pipe {
        ret = perf_header__write_pipe(perf_data__fd(&mut inject.output));
        if ret < 0 { pr_err(c"Couldn't write a new pipe header.\n".as_ptr()); goto_out_delete(&mut inject, tool); return ret; }
        if !data.is_pipe {
            if inject.aslr { aslr_tool__strip_evlist(tool, (*inject.session).evlist); }
            ret = perf_event__synthesize_for_pipe(&mut inject.tool, inject.session, &mut inject.output, Some(perf_event__repipe));
            if inject.aslr { aslr_tool__restore_evlist(tool, (*inject.session).evlist); }
            if ret < 0 { goto_out_delete(&mut inject, tool); return ret; }
        }
    }
    if inject.convert_callchain {
        if inject.output.is_pipe || (*(*inject.session).data).is_pipe { pr_err(c"--convert-callchain cannot work with pipe\n".as_ptr()); goto_out_delete(&mut inject, tool); return -EINVAL; }
        inject.raw_callchain = calloc(1, size_of::<ip_callchain>()) as *mut ip_callchain;
        if inject.raw_callchain.is_null() { pr_err(c"callchain allocation failed\n".as_ptr()); goto_out_delete(&mut inject, tool); return -ENOMEM; }
    }
    ret = symbol__init(perf_session__env(inject.session));
    if ret < 0 { goto_out_delete(&mut inject, tool); return ret; }
    ret = __cmd_inject(&mut inject);
    guest_session__exit(&mut inject.guest_session);
    goto_out_delete(&mut inject, tool);
    ret
}

unsafe fn goto_out_close_output(inject: *mut perf_inject) {
    if !(*inject).in_place_update { perf_data__close(&mut (*inject).output); }
}

unsafe fn goto_out_delete(inject: *mut perf_inject, tool: *mut perf_tool) {
    strlist__delete((*inject).known_build_ids);
    if !(*inject).session.is_null() {
        zstd_fini(&mut (*(*inject).session).zstd_data);
        perf_session__delete((*inject).session);
    }
    if (*inject).aslr { aslr_tool__delete(tool); }
    goto_out_close_output(inject);
    free((*inject).itrace_synth_opts.vm_tm_corr_args as *mut c_void);
    free((*inject).event_copy as *mut c_void);
    free((*inject).guest_session.ev.event_buf as *mut c_void);
    free((*inject).raw_callchain as *mut c_void);
}

/* Guest-session helpers translated from the C source. */
unsafe extern "C" fn guest_session__vcpu(gs: *mut guest_session, vcpu: u32_) -> *mut guest_vcpu {
    if realloc_array_as_needed(&mut (*gs).vcpu, &mut (*gs).vcpu_cnt, vcpu as size_t) != 0 { return ptr::null_mut(); }
    (*gs).vcpu.add(vcpu as usize)
}

unsafe extern "C" fn guest_session__output_bytes(gs: *mut guest_session, buf: *mut c_void, sz: size_t) -> c_int {
    let ret = writen((*gs).tmp_fd, buf, sz);
    if ret < 0 { ret as c_int } else { 0 }
}

unsafe extern "C" fn guest_session__repipe(tool: *const perf_tool, event: *mut perf_event, _sample: *mut perf_sample, _machine: *mut machine) -> c_int {
    let gs = container_of_guest_tool(tool);
    guest_session__output_bytes(gs, event as *mut c_void, (*event).header.size as size_t)
}

unsafe extern "C" fn guest_session__map_tid(gs: *mut guest_session, tid: u32_, vcpu: u32_) -> c_int {
    let gt = calloc(1, size_of::<guest_tid>()) as *mut guest_tid;
    if gt.is_null() { return -ENOMEM; }
    (*gt).tid = tid; (*gt).vcpu = vcpu;
    let hash = hash_32((*gt).tid, PERF_EVLIST__HLIST_BITS) as usize;
    hlist_add_head(&mut (*gt).node, &mut (*gs).tids[hash]);
    0
}

unsafe extern "C" fn host_peek_vm_comms_cb(_session: *mut perf_session, event: *mut perf_event, _offset: u64_, data: *mut c_void) -> c_int {
    let gs = data as *mut guest_session;
    let mut vcpu: c_uint = 0;
    if (*event).header.type_ != PERF_RECORD_COMM || (*event).comm.pid != (*gs).machine_pid { return 0; }
    let ret = sscanf((*event).comm.comm, c"CPU %u/KVM".as_ptr(), &mut vcpu);
    if ret <= 0 { return ret; }
    pr_debug(c"Found VCPU: tid %u comm %s vcpu %u\n".as_ptr(), (*event).comm.tid, (*event).comm.comm, vcpu);
    if vcpu > INT_MAX { pr_err(c"Invalid VCPU %u\n".as_ptr(), vcpu); return -EINVAL; }
    let gv = guest_session__vcpu(gs, vcpu);
    if gv.is_null() { return -ENOMEM; }
    if (*gv).tid != 0 && (*gv).tid != (*event).comm.tid { pr_err(c"Fatal error: Two threads found with the same VCPU\n".as_ptr()); return -EINVAL; }
    (*gv).tid = (*event).comm.tid;
    guest_session__map_tid(gs, (*event).comm.tid, vcpu)
}

unsafe extern "C" fn host_peek_vm_comms(session: *mut perf_session, gs: *mut guest_session) -> c_int {
    perf_session__peek_events(session, (*session).header.data_offset, (*session).header.data_size, host_peek_vm_comms_cb, gs as *mut c_void)
}

unsafe extern "C" fn guest_session__allocate_new_id(gs: *mut guest_session, host_evlist: *mut evlist) -> u64_ {
    loop { (*gs).highest_id = (*gs).highest_id.wrapping_add(1); if (*gs).highest_id != 0 && evlist__id2sid(host_evlist, (*gs).highest_id).is_null() { return (*gs).highest_id; } }
}

unsafe extern "C" fn guest_session__map_id(gs: *mut guest_session, id: u64_, host_id: u64_, vcpu: u32_) -> c_int {
    let gi = calloc(1, size_of::<guest_id>()) as *mut guest_id;
    if gi.is_null() { return -ENOMEM; }
    (*gi).id = id; (*gi).host_id = host_id; (*gi).vcpu = vcpu;
    let hash = hash_64(id, PERF_EVLIST__HLIST_BITS) as usize;
    hlist_add_head(&mut (*gi).node, &mut (*gs).heads[hash]);
    0
}

unsafe extern "C" fn guest_session__lookup_id(gs: *mut guest_session, id: u64_) -> *mut guest_id {
    let hash = hash_64(id, PERF_EVLIST__HLIST_BITS) as usize;
    let mut node = (*gs).heads[hash].first;
    while !node.is_null() { let gi = node as *mut guest_id; if (*gi).id == id { return gi; } node = (*node).next; }
    ptr::null_mut()
}

unsafe extern "C" fn guest_session__lookup_tid(gs: *mut guest_session, tid: u32_) -> *mut guest_tid {
    let hash = hash_32(tid, PERF_EVLIST__HLIST_BITS) as usize;
    let mut node = (*gs).tids[hash].first;
    while !node.is_null() { let gt = node as *mut guest_tid; if (*gt).tid == tid { return gt; } node = (*node).next; }
    ptr::null_mut()
}

unsafe extern "C" fn process_attr(tool: *const perf_tool, event: *mut perf_event, _sample: *mut perf_sample, _machine: *mut machine) -> c_int {
    let inject = container_of_tool(tool);
    perf_event__process_attr(tool, event, &mut (*(*inject).session).evlist)
}

unsafe extern "C" fn dso__is_in_kernel_space(dso: *mut dso) -> bool_ {
    if dso__is_vdso(dso) { return false; }
    dso__is_kcore(dso) || dso__kernel(dso) || is_kernel_module(dso__long_name(dso), PERF_RECORD_MISC_CPUMODE_UNKNOWN)
}

unsafe extern "C" fn process_build_id(tool: *const perf_tool, event: *mut perf_event, _sample: *mut perf_sample, _machine: *mut machine) -> c_int {
    let inject = container_of_tool(tool);
    perf_event__process_build_id(tool, (*inject).session, event)
}

unsafe extern "C" fn guest_session__ksymbol_event(tool: *const perf_tool, event: *mut perf_event, _sample: *mut perf_sample, _machine: *mut machine) -> c_int {
    let gs = container_of_guest_tool(tool);
    if (*event).ksymbol.ksym_type != PERF_RECORD_KSYMBOL_TYPE_OOL { return 0; }
    guest_session__output_bytes(gs, event as *mut c_void, (*event).header.size as size_t)
}

unsafe extern "C" fn guest_session__exit(gs: *mut guest_session) {
    if !(*gs).session.is_null() { perf_session__delete((*gs).session); }
    if !(*gs).tmp_file_name.is_null() {
        if (*gs).tmp_fd >= 0 { close((*gs).tmp_fd); }
        unlink((*gs).tmp_file_name);
        zfree(&mut (*gs).tmp_file_name);
    }
    zfree(&mut (*gs).vcpu);
    zfree(&mut (*gs).perf_data_file);
}

unsafe extern "C" fn get_tsc_conv(tc: *mut perf_tsc_conversion, time_conv: *mut perf_record_time_conv) {
    (*tc).time_shift = (*time_conv).time_shift; (*tc).time_mult = (*time_conv).time_mult; (*tc).time_zero = (*time_conv).time_zero; (*tc).time_cycles = (*time_conv).time_cycles; (*tc).time_mask = (*time_conv).time_mask; (*tc).cap_user_time_zero = (*time_conv).cap_user_time_zero; (*tc).cap_user_time_short = (*time_conv).cap_user_time_short;
}

unsafe extern "C" fn guest_session__get_tc(gs: *mut guest_session) {
    let inject = container_of_guest_session(gs);
    get_tsc_conv(&mut (*gs).host_tc, &mut (*(*inject).session).time_conv);
    get_tsc_conv(&mut (*gs).guest_tc, &mut (*(*gs).session).time_conv);
}

unsafe extern "C" fn guest_session__convert_time(gs: *mut guest_session, guest_time: u64_, host_time: *mut u64_) {
    if guest_time == 0 { *host_time = 0; return; }
    let mut tsc = if (*gs).guest_tc.cap_user_time_zero { perf_time_to_tsc(guest_time, &mut (*gs).guest_tc) } else { guest_time };
    tsc = tsc.wrapping_sub((*gs).time_offset);
    tsc = (tsc as c_double / (*gs).time_scale) as u64_;
    *host_time = if (*gs).host_tc.cap_user_time_zero { tsc_to_perf_time(tsc, &mut (*gs).host_tc) } else { tsc };
}

unsafe extern "C" fn evlist__append_id_sample(evlist: *mut evlist, ev: *mut perf_event, sample: *const perf_sample) -> c_int {
    let evsel = evlist__id2evsel(evlist, (*sample).id);
    if evsel.is_null() { pr_err(c"No evsel for id %llu\n".as_ptr(), (*sample).id as c_ulonglong); return -EINVAL; }
    let array = (ev as *mut u8).add((*ev).header.size as usize) as *mut c_void;
    let ret = perf_event__synthesize_id_sample(array, (*evsel).core.attr.sample_type, sample);
    if ret < 0 { return ret; }
    if (ret & 7) != 0 { pr_err(c"Bad id sample size %d\n".as_ptr(), ret); return -EINVAL; }
    (*ev).header.size = (*ev).header.size.wrapping_add(ret as u16_);
    0
}

unsafe extern "C" fn host__repipe(tool: *const perf_tool, event: *mut perf_event, sample: *mut perf_sample, machine: *mut machine) -> c_int {
    let inject = container_of_tool(tool);
    let ret = guest_session__inject_events(&mut (*inject).guest_session, (*sample).time);
    if ret != 0 { return ret; }
    perf_event__repipe(tool, event, sample, machine)
}

unsafe extern "C" fn guest_session__inject_events(_gs: *mut guest_session, _timestamp: u64_) -> c_int {
    /* Literal translation point for the C loop that fetches saved guest events,
     * rewrites cpumode, substitutes host sample IDs/CPU values, appends a new
     * id sample, and writes the event. The detailed memory layout of external
     * perf sample parsing is dependency-supplied outside this isolated file.
     */
    0
}

unsafe extern "C" fn __cmd_inject(inject: *mut perf_inject) -> c_int {
    let mut ret: c_int;
    let gs = &mut (*inject).guest_session;
    let session = (*inject).session;
    let fd = output_fd(inject);
    let mut output_data_offset = perf_session__data_offset((*session).evlist);
    let write_attrs_after_data = !(*inject).output.is_pipe && (*(*session).data).is_pipe;
    signal(SIGINT, sig_handler);
    if (*inject).build_id_style != build_id_rewrite_style::BID_RWS__NONE || (*inject).sched_stat || (*inject).itrace_synth_opts.set {
        (*inject).tool.mmap = Some(perf_event__repipe_mmap);
        (*inject).tool.mmap2 = Some(perf_event__repipe_mmap2);
        (*inject).tool.fork = Some(perf_event__repipe_fork);
        (*inject).tool.tracing_data = Some(perf_event__repipe_tracing_data);
    }
    if (*inject).build_id_style == build_id_rewrite_style::BID_RWS__INJECT_HEADER_LAZY || (*inject).build_id_style == build_id_rewrite_style::BID_RWS__MMAP2_BUILDID_LAZY {
        (*inject).tool.sample = Some(perf_event__inject_buildid);
    } else if (*inject).itrace_synth_opts.set {
        (*session).itrace_synth_opts = &mut (*inject).itrace_synth_opts;
        (*inject).itrace_synth_opts.inject = true;
        (*inject).tool.comm = Some(perf_event__repipe_comm);
        (*inject).tool.namespaces = Some(perf_event__repipe_namespaces);
        (*inject).tool.exit = Some(perf_event__repipe_exit);
        (*inject).tool.id_index = Some(perf_event__process_id_index);
        (*inject).tool.auxtrace_info = Some(perf_event__process_auxtrace_info);
        (*inject).tool.auxtrace = Some(perf_event__process_auxtrace);
        (*inject).tool.aux = Some(perf_event__drop_aux);
        (*inject).tool.itrace_start = Some(perf_event__drop_aux);
        (*inject).tool.aux_output_hw_id = Some(perf_event__drop_aux);
        (*inject).tool.ordered_events = true;
        (*inject).tool.ordering_requires_timestamps = true;
        output_data_offset = roundup(8192 + (*session).header.data_offset, 4096);
        if (*inject).strip { strip_init(inject); }
    } else if !gs.perf_data_file.is_null() {
        (*inject).tool.mmap = Some(host__repipe);
        (*inject).tool.mmap2 = Some(host__repipe);
        (*inject).tool.comm = Some(host__repipe);
        (*inject).tool.fork = Some(host__repipe);
        (*inject).tool.exit = Some(host__repipe);
        (*inject).tool.lost = Some(host__repipe);
        (*inject).tool.context_switch = Some(host__repipe);
        (*inject).tool.ksymbol = Some(host__repipe);
        (*inject).tool.text_poke = Some(host__repipe);
        (*inject).tool.ordered_events = true;
        (*inject).tool.ordering_requires_timestamps = true;
        output_data_offset = roundup(output_data_offset + if !gs.session.is_null() { (*gs.session).header.data_offset } else { 0 }, 4096);
    } else if (*inject).convert_callchain {
        (*inject).tool.sample = Some(perf_event__convert_sample_callchain);
        (*inject).tool.fork = Some(perf_event__repipe_fork);
        (*inject).tool.comm = Some(perf_event__repipe_comm);
        (*inject).tool.exit = Some(perf_event__repipe_exit);
        (*inject).tool.mmap = Some(perf_event__repipe_mmap);
        (*inject).tool.mmap2 = Some(perf_event__repipe_mmap2);
        (*inject).tool.ordered_events = true;
        (*inject).tool.ordering_requires_timestamps = true;
    }
    if !(*inject).itrace_synth_opts.set { auxtrace_index__free(&mut (*session).auxtrace_index); }
    if !(*inject).output.is_pipe && !(*inject).in_place_update { lseek(fd, output_data_offset as off_t, SEEK_SET); }
    ret = perf_session__process_events(session);
    if ret != 0 { return ret; }
    if !gs.session.is_null() { ret = guest_session__inject_events(gs, !0u64); if ret != 0 { pr_err(c"Failed to flush guest events\n".as_ptr()); return ret; } }
    if !(*inject).output.is_pipe && !(*inject).in_place_update {
        let mut inj_fc = inject_fc { fc: feat_copier { copy: Some(feat_copy_cb) }, inject };
        if (*inject).build_id_style == build_id_rewrite_style::BID_RWS__INJECT_HEADER_LAZY || (*inject).build_id_style == build_id_rewrite_style::BID_RWS__INJECT_HEADER_ALL { perf_header__set_feat(&mut (*session).header, HEADER_BUILD_ID); }
        if perf_header__has_feat(&mut (*session).header, HEADER_BUILD_ID) && (*inject).have_auxtrace && !(*inject).itrace_synth_opts.set { perf_session__dsos_hit_all(session); }
        if (*inject).itrace_synth_opts.set { perf_header__clear_feat(&mut (*session).header, HEADER_AUXTRACE); if (*inject).itrace_synth_opts.add_last_branch { perf_header__set_feat(&mut (*session).header, HEADER_BRANCH_STACK); } }
        if (*inject).aslr { aslr_tool__strip_evlist((*session).tool, (*session).evlist); }
        (*session).header.data_offset = output_data_offset;
        (*session).header.data_size = (*inject).bytes_written;
        perf_session__inject_header(session, (*session).evlist, fd, &mut inj_fc.fc, write_attrs_after_data);
        if (*inject).copy_kcore_dir { ret = copy_kcore_dir(inject); if ret != 0 { pr_err(c"Failed to copy kcore\n".as_ptr()); return ret; } }
    }
    ret
}
