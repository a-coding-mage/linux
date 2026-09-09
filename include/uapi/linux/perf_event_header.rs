/* SPDX-License-Identifier: GPL-2.0 WITH Linux-syscall-note */
/* Rust translation of uapi/linux/perf_event.h. */

/* External Linux ABI integer types are supplied by the surrounding bindings. */
pub type __u8 = u8;
pub type __u16 = u16;
pub type __u32 = u32;
pub type __u64 = u64;
pub type __s32 = i32;
pub type __s64 = i64;

pub const PERF_TYPE_HARDWARE: u32 = 0;
pub const PERF_TYPE_SOFTWARE: u32 = 1;
pub const PERF_TYPE_TRACEPOINT: u32 = 2;
pub const PERF_TYPE_HW_CACHE: u32 = 3;
pub const PERF_TYPE_RAW: u32 = 4;
pub const PERF_TYPE_BREAKPOINT: u32 = 5;
pub const PERF_TYPE_MAX: u32 = 6;
pub const PERF_PMU_TYPE_SHIFT: u32 = 32;
pub const PERF_HW_EVENT_MASK: u64 = 0xffff_ffff;

pub const PERF_COUNT_HW_CPU_CYCLES: u32 = 0;
pub const PERF_COUNT_HW_INSTRUCTIONS: u32 = 1;
pub const PERF_COUNT_HW_CACHE_REFERENCES: u32 = 2;
pub const PERF_COUNT_HW_CACHE_MISSES: u32 = 3;
pub const PERF_COUNT_HW_BRANCH_INSTRUCTIONS: u32 = 4;
pub const PERF_COUNT_HW_BRANCH_MISSES: u32 = 5;
pub const PERF_COUNT_HW_BUS_CYCLES: u32 = 6;
pub const PERF_COUNT_HW_STALLED_CYCLES_FRONTEND: u32 = 7;
pub const PERF_COUNT_HW_STALLED_CYCLES_BACKEND: u32 = 8;
pub const PERF_COUNT_HW_REF_CPU_CYCLES: u32 = 9;
pub const PERF_COUNT_HW_MAX: u32 = 10;
pub const PERF_COUNT_HW_CACHE_L1D: u32 = 0;
pub const PERF_COUNT_HW_CACHE_L1I: u32 = 1;
pub const PERF_COUNT_HW_CACHE_LL: u32 = 2;
pub const PERF_COUNT_HW_CACHE_DTLB: u32 = 3;
pub const PERF_COUNT_HW_CACHE_ITLB: u32 = 4;
pub const PERF_COUNT_HW_CACHE_BPU: u32 = 5;
pub const PERF_COUNT_HW_CACHE_NODE: u32 = 6;
pub const PERF_COUNT_HW_CACHE_MAX: u32 = 7;
pub const PERF_COUNT_HW_CACHE_OP_READ: u32 = 0;
pub const PERF_COUNT_HW_CACHE_OP_WRITE: u32 = 1;
pub const PERF_COUNT_HW_CACHE_OP_PREFETCH: u32 = 2;
pub const PERF_COUNT_HW_CACHE_OP_MAX: u32 = 3;
pub const PERF_COUNT_HW_CACHE_RESULT_ACCESS: u32 = 0;
pub const PERF_COUNT_HW_CACHE_RESULT_MISS: u32 = 1;
pub const PERF_COUNT_HW_CACHE_RESULT_MAX: u32 = 2;

pub const PERF_COUNT_SW_CPU_CLOCK: u32 = 0;
pub const PERF_COUNT_SW_TASK_CLOCK: u32 = 1;
pub const PERF_COUNT_SW_PAGE_FAULTS: u32 = 2;
pub const PERF_COUNT_SW_CONTEXT_SWITCHES: u32 = 3;
pub const PERF_COUNT_SW_CPU_MIGRATIONS: u32 = 4;
pub const PERF_COUNT_SW_PAGE_FAULTS_MIN: u32 = 5;
pub const PERF_COUNT_SW_PAGE_FAULTS_MAJ: u32 = 6;
pub const PERF_COUNT_SW_ALIGNMENT_FAULTS: u32 = 7;
pub const PERF_COUNT_SW_EMULATION_FAULTS: u32 = 8;
pub const PERF_COUNT_SW_DUMMY: u32 = 9;
pub const PERF_COUNT_SW_BPF_OUTPUT: u32 = 10;
pub const PERF_COUNT_SW_CGROUP_SWITCHES: u32 = 11;
pub const PERF_COUNT_SW_MAX: u32 = 12;

pub const PERF_SAMPLE_IP: u64 = 1 << 0; pub const PERF_SAMPLE_TID: u64 = 1 << 1;
pub const PERF_SAMPLE_TIME: u64 = 1 << 2; pub const PERF_SAMPLE_ADDR: u64 = 1 << 3;
pub const PERF_SAMPLE_READ: u64 = 1 << 4; pub const PERF_SAMPLE_CALLCHAIN: u64 = 1 << 5;
pub const PERF_SAMPLE_ID: u64 = 1 << 6; pub const PERF_SAMPLE_CPU: u64 = 1 << 7;
pub const PERF_SAMPLE_PERIOD: u64 = 1 << 8; pub const PERF_SAMPLE_STREAM_ID: u64 = 1 << 9;
pub const PERF_SAMPLE_RAW: u64 = 1 << 10; pub const PERF_SAMPLE_BRANCH_STACK: u64 = 1 << 11;
pub const PERF_SAMPLE_REGS_USER: u64 = 1 << 12; pub const PERF_SAMPLE_STACK_USER: u64 = 1 << 13;
pub const PERF_SAMPLE_WEIGHT: u64 = 1 << 14; pub const PERF_SAMPLE_DATA_SRC: u64 = 1 << 15;
pub const PERF_SAMPLE_IDENTIFIER: u64 = 1 << 16; pub const PERF_SAMPLE_TRANSACTION: u64 = 1 << 17;
pub const PERF_SAMPLE_REGS_INTR: u64 = 1 << 18; pub const PERF_SAMPLE_PHYS_ADDR: u64 = 1 << 19;
pub const PERF_SAMPLE_AUX: u64 = 1 << 20; pub const PERF_SAMPLE_CGROUP: u64 = 1 << 21;
pub const PERF_SAMPLE_DATA_PAGE_SIZE: u64 = 1 << 22; pub const PERF_SAMPLE_CODE_PAGE_SIZE: u64 = 1 << 23;
pub const PERF_SAMPLE_WEIGHT_STRUCT: u64 = 1 << 24; pub const PERF_SAMPLE_MAX: u64 = 1 << 25;
pub const PERF_SAMPLE_WEIGHT_TYPE: u64 = PERF_SAMPLE_WEIGHT | PERF_SAMPLE_WEIGHT_STRUCT;

pub const PERF_SAMPLE_BRANCH_USER_SHIFT: u32 = 0; pub const PERF_SAMPLE_BRANCH_KERNEL_SHIFT: u32 = 1;
pub const PERF_SAMPLE_BRANCH_HV_SHIFT: u32 = 2; pub const PERF_SAMPLE_BRANCH_ANY_SHIFT: u32 = 3;
pub const PERF_SAMPLE_BRANCH_ANY_CALL_SHIFT: u32 = 4; pub const PERF_SAMPLE_BRANCH_ANY_RETURN_SHIFT: u32 = 5;
pub const PERF_SAMPLE_BRANCH_IND_CALL_SHIFT: u32 = 6; pub const PERF_SAMPLE_BRANCH_ABORT_TX_SHIFT: u32 = 7;
pub const PERF_SAMPLE_BRANCH_IN_TX_SHIFT: u32 = 8; pub const PERF_SAMPLE_BRANCH_NO_TX_SHIFT: u32 = 9;
pub const PERF_SAMPLE_BRANCH_COND_SHIFT: u32 = 10; pub const PERF_SAMPLE_BRANCH_CALL_STACK_SHIFT: u32 = 11;
pub const PERF_SAMPLE_BRANCH_IND_JUMP_SHIFT: u32 = 12; pub const PERF_SAMPLE_BRANCH_CALL_SHIFT: u32 = 13;
pub const PERF_SAMPLE_BRANCH_NO_FLAGS_SHIFT: u32 = 14; pub const PERF_SAMPLE_BRANCH_NO_CYCLES_SHIFT: u32 = 15;
pub const PERF_SAMPLE_BRANCH_TYPE_SAVE_SHIFT: u32 = 16; pub const PERF_SAMPLE_BRANCH_HW_INDEX_SHIFT: u32 = 17;
pub const PERF_SAMPLE_BRANCH_PRIV_SAVE_SHIFT: u32 = 18; pub const PERF_SAMPLE_BRANCH_COUNTERS_SHIFT: u32 = 19;
pub const PERF_SAMPLE_BRANCH_MAX_SHIFT: u32 = 20;
pub const PERF_SAMPLE_BRANCH_USER: u64 = 1 << 0; pub const PERF_SAMPLE_BRANCH_KERNEL: u64 = 1 << 1;
pub const PERF_SAMPLE_BRANCH_HV: u64 = 1 << 2; pub const PERF_SAMPLE_BRANCH_ANY: u64 = 1 << 3;
pub const PERF_SAMPLE_BRANCH_ANY_CALL: u64 = 1 << 4; pub const PERF_SAMPLE_BRANCH_ANY_RETURN: u64 = 1 << 5;
pub const PERF_SAMPLE_BRANCH_IND_CALL: u64 = 1 << 6; pub const PERF_SAMPLE_BRANCH_ABORT_TX: u64 = 1 << 7;
pub const PERF_SAMPLE_BRANCH_IN_TX: u64 = 1 << 8; pub const PERF_SAMPLE_BRANCH_NO_TX: u64 = 1 << 9;
pub const PERF_SAMPLE_BRANCH_COND: u64 = 1 << 10; pub const PERF_SAMPLE_BRANCH_CALL_STACK: u64 = 1 << 11;
pub const PERF_SAMPLE_BRANCH_IND_JUMP: u64 = 1 << 12; pub const PERF_SAMPLE_BRANCH_CALL: u64 = 1 << 13;
pub const PERF_SAMPLE_BRANCH_NO_FLAGS: u64 = 1 << 14; pub const PERF_SAMPLE_BRANCH_NO_CYCLES: u64 = 1 << 15;
pub const PERF_SAMPLE_BRANCH_TYPE_SAVE: u64 = 1 << 16; pub const PERF_SAMPLE_BRANCH_HW_INDEX: u64 = 1 << 17;
pub const PERF_SAMPLE_BRANCH_PRIV_SAVE: u64 = 1 << 18; pub const PERF_SAMPLE_BRANCH_COUNTERS: u64 = 1 << 19;
pub const PERF_SAMPLE_BRANCH_MAX: u64 = 1 << 20;
pub const PERF_SAMPLE_BRANCH_PLM_ALL: u64 = PERF_SAMPLE_BRANCH_USER | PERF_SAMPLE_BRANCH_KERNEL | PERF_SAMPLE_BRANCH_HV;

pub const PERF_BR_UNKNOWN: u32 = 0; pub const PERF_BR_COND: u32 = 1; pub const PERF_BR_UNCOND: u32 = 2;
pub const PERF_BR_IND: u32 = 3; pub const PERF_BR_CALL: u32 = 4; pub const PERF_BR_IND_CALL: u32 = 5;
pub const PERF_BR_RET: u32 = 6; pub const PERF_BR_SYSCALL: u32 = 7; pub const PERF_BR_SYSRET: u32 = 8;
pub const PERF_BR_COND_CALL: u32 = 9; pub const PERF_BR_COND_RET: u32 = 10; pub const PERF_BR_ERET: u32 = 11;
pub const PERF_BR_IRQ: u32 = 12; pub const PERF_BR_SERROR: u32 = 13; pub const PERF_BR_NO_TX: u32 = 14;
pub const PERF_BR_EXTEND_ABI: u32 = 15; pub const PERF_BR_MAX: u32 = 16;
pub const PERF_BR_SPEC_NA: u32 = 0; pub const PERF_BR_SPEC_WRONG_PATH: u32 = 1;
pub const PERF_BR_NON_SPEC_CORRECT_PATH: u32 = 2; pub const PERF_BR_SPEC_CORRECT_PATH: u32 = 3; pub const PERF_BR_SPEC_MAX: u32 = 4;
pub const PERF_BR_NEW_FAULT_ALGN: u32 = 0; pub const PERF_BR_NEW_FAULT_DATA: u32 = 1; pub const PERF_BR_NEW_FAULT_INST: u32 = 2;
pub const PERF_BR_NEW_ARCH_1: u32 = 3; pub const PERF_BR_NEW_ARCH_2: u32 = 4; pub const PERF_BR_NEW_ARCH_3: u32 = 5;
pub const PERF_BR_NEW_ARCH_4: u32 = 6; pub const PERF_BR_NEW_ARCH_5: u32 = 7; pub const PERF_BR_NEW_MAX: u32 = 8;
pub const PERF_BR_PRIV_UNKNOWN: u32 = 0; pub const PERF_BR_PRIV_USER: u32 = 1; pub const PERF_BR_PRIV_KERNEL: u32 = 2; pub const PERF_BR_PRIV_HV: u32 = 3;
pub const PERF_BR_ARM64_FIQ: u32 = PERF_BR_NEW_ARCH_1; pub const PERF_BR_ARM64_DEBUG_HALT: u32 = PERF_BR_NEW_ARCH_2;
pub const PERF_BR_ARM64_DEBUG_EXIT: u32 = PERF_BR_NEW_ARCH_3; pub const PERF_BR_ARM64_DEBUG_INST: u32 = PERF_BR_NEW_ARCH_4; pub const PERF_BR_ARM64_DEBUG_DATA: u32 = PERF_BR_NEW_ARCH_5;
pub const PERF_SAMPLE_REGS_ABI_NONE: u32 = 0; pub const PERF_SAMPLE_REGS_ABI_32: u32 = 1; pub const PERF_SAMPLE_REGS_ABI_64: u32 = 2;
pub const PERF_TXN_ELISION: u64 = 1 << 0; pub const PERF_TXN_TRANSACTION: u64 = 1 << 1; pub const PERF_TXN_SYNC: u64 = 1 << 2;
pub const PERF_TXN_ASYNC: u64 = 1 << 3; pub const PERF_TXN_RETRY: u64 = 1 << 4; pub const PERF_TXN_CONFLICT: u64 = 1 << 5;
pub const PERF_TXN_CAPACITY_WRITE: u64 = 1 << 6; pub const PERF_TXN_CAPACITY_READ: u64 = 1 << 7; pub const PERF_TXN_MAX: u64 = 1 << 8;
pub const PERF_TXN_ABORT_MASK: u64 = 0xffff_ffffu64 << 32; pub const PERF_TXN_ABORT_SHIFT: u32 = 32;

pub const PERF_FORMAT_TOTAL_TIME_ENABLED: u64 = 1 << 0; pub const PERF_FORMAT_TOTAL_TIME_RUNNING: u64 = 1 << 1;
pub const PERF_FORMAT_ID: u64 = 1 << 2; pub const PERF_FORMAT_GROUP: u64 = 1 << 3;
pub const PERF_FORMAT_LOST: u64 = 1 << 4; pub const PERF_FORMAT_MAX: u64 = 1 << 5;

/* ioctl encodings are supplied by the Linux ioctl ABI helpers. */
pub const PERF_EVENT_IOC_ENABLE: u32 = _IO(b'$', 0);
pub const PERF_EVENT_IOC_DISABLE: u32 = _IO(b'$', 1);
pub const PERF_EVENT_IOC_REFRESH: u32 = _IO(b'$', 2);
pub const PERF_EVENT_IOC_RESET: u32 = _IO(b'$', 3);
pub const PERF_EVENT_IOC_PERIOD: u32 = _IOW(b'$', 4, __u64);
pub const PERF_EVENT_IOC_SET_OUTPUT: u32 = _IO(b'$', 5);
pub const PERF_EVENT_IOC_SET_FILTER: u32 = _IOW(b'$', 6, *mut core::ffi::c_char);
pub const PERF_EVENT_IOC_ID: u32 = _IOR(b'$', 7, *mut __u64);
pub const PERF_EVENT_IOC_SET_BPF: u32 = _IOW(b'$', 8, __u32);
pub const PERF_EVENT_IOC_PAUSE_OUTPUT: u32 = _IOW(b'$', 9, __u32);
pub const PERF_EVENT_IOC_QUERY_BPF: u32 = _IOWR(b'$', 10, *mut perf_event_query_bpf);
pub const PERF_EVENT_IOC_MODIFY_ATTRIBUTES: u32 = _IOW(b'$', 11, *mut perf_event_attr);
pub const PERF_ATTR_SIZE_VER0: usize = 64; pub const PERF_ATTR_SIZE_VER1: usize = 72;
pub const PERF_ATTR_SIZE_VER2: usize = 80; pub const PERF_ATTR_SIZE_VER3: usize = 96;
pub const PERF_ATTR_SIZE_VER4: usize = 104; pub const PERF_ATTR_SIZE_VER5: usize = 112;
pub const PERF_ATTR_SIZE_VER6: usize = 120; pub const PERF_ATTR_SIZE_VER7: usize = 128;
pub const PERF_ATTR_SIZE_VER8: usize = 136; pub const PERF_ATTR_SIZE_VER9: usize = 144;

#[repr(C)]
pub union perf_event_attr_sample_period { pub sample_period: __u64, pub sample_freq: __u64 }
#[repr(C)] pub union perf_event_attr_wakeup { pub wakeup_events: __u32, pub wakeup_watermark: __u32 }
#[repr(C)] pub union perf_event_attr_bp_addr { pub bp_addr: __u64, pub kprobe_func: __u64, pub uprobe_path: __u64, pub config1: __u64 }
#[repr(C)] pub union perf_event_attr_bp_len { pub bp_len: __u64, pub kprobe_addr: __u64, pub probe_offset: __u64, pub config2: __u64 }
#[repr(C)] pub union perf_event_attr_aux { pub aux_action: __u32, pub bits: __u32 }

#[repr(C)]
pub struct perf_event_attr {
    pub type_: __u32, pub size: __u32, pub config: __u64,
    pub sample_period: perf_event_attr_sample_period, pub sample_type: __u64, pub read_format: __u64,
    /* C bitfields are represented by their ABI storage word. */ pub flags: __u64,
    pub wakeup: perf_event_attr_wakeup, pub bp_type: __u32,
    pub bp_addr: perf_event_attr_bp_addr, pub bp_len: perf_event_attr_bp_len,
    pub branch_sample_type: __u64, pub sample_regs_user: __u64, pub sample_stack_user: __u32,
    pub clockid: __s32, pub sample_regs_intr: __u64, pub aux_watermark: __u32,
    pub sample_max_stack: __u16, pub __reserved_2: __u16, pub aux_sample_size: __u32,
    pub aux: perf_event_attr_aux, pub sig_data: __u64, pub config3: __u64, pub config4: __u64,
}

#[repr(C)] pub struct perf_event_query_bpf { pub ids_len: __u32, pub prog_cnt: __u32, pub ids: [__u32; 0] }
pub const PERF_IOC_FLAG_GROUP: u32 = 1 << 0;

#[repr(C)] pub struct perf_event_mmap_page {
    pub version: __u32, pub compat_version: __u32, pub lock: __u32, pub index: __u32,
    pub offset: __s64, pub time_enabled: __u64, pub time_running: __u64, pub capabilities: __u64,
    pub pmc_width: __u16, pub time_shift: __u16, pub time_mult: __u32, pub time_offset: __u64,
    pub time_zero: __u64, pub size: __u32, pub __reserved_1: __u32, pub time_cycles: __u64,
    pub time_mask: __u64, pub __reserved: [__u8; 116 * 8], pub data_head: __u64, pub data_tail: __u64,
    pub data_offset: __u64, pub data_size: __u64, pub aux_head: __u64, pub aux_tail: __u64,
    pub aux_offset: __u64, pub aux_size: __u64,
}

pub const PERF_RECORD_MISC_CPUMODE_MASK: u16 = 7; pub const PERF_RECORD_MISC_CPUMODE_UNKNOWN: u16 = 0;
pub const PERF_RECORD_MISC_KERNEL: u16 = 1; pub const PERF_RECORD_MISC_USER: u16 = 2;
pub const PERF_RECORD_MISC_HYPERVISOR: u16 = 3; pub const PERF_RECORD_MISC_GUEST_KERNEL: u16 = 4;
pub const PERF_RECORD_MISC_GUEST_USER: u16 = 5; pub const PERF_RECORD_MISC_PROC_MAP_PARSE_TIMEOUT: u16 = 1 << 12;
pub const PERF_RECORD_MISC_MMAP_DATA: u16 = 1 << 13; pub const PERF_RECORD_MISC_COMM_EXEC: u16 = 1 << 13;
pub const PERF_RECORD_MISC_FORK_EXEC: u16 = 1 << 13; pub const PERF_RECORD_MISC_SWITCH_OUT: u16 = 1 << 13;
pub const PERF_RECORD_MISC_EXACT_IP: u16 = 1 << 14; pub const PERF_RECORD_MISC_SWITCH_OUT_PREEMPT: u16 = 1 << 14;
pub const PERF_RECORD_MISC_MMAP_BUILD_ID: u16 = 1 << 14; pub const PERF_RECORD_MISC_EXT_RESERVED: u16 = 1 << 15;
#[repr(C)] pub struct perf_event_header { pub type_: __u32, pub misc: __u16, pub size: __u16 }
#[repr(C)] pub struct perf_ns_link_info { pub dev: __u64, pub ino: __u64 }
pub const NET_NS_INDEX: u32 = 0; pub const UTS_NS_INDEX: u32 = 1; pub const IPC_NS_INDEX: u32 = 2;
pub const PID_NS_INDEX: u32 = 3; pub const USER_NS_INDEX: u32 = 4; pub const MNT_NS_INDEX: u32 = 5;
pub const CGROUP_NS_INDEX: u32 = 6; pub const NR_NAMESPACES: u32 = 7;

pub const PERF_RECORD_MMAP: u32 = 1; pub const PERF_RECORD_LOST: u32 = 2; pub const PERF_RECORD_COMM: u32 = 3;
pub const PERF_RECORD_EXIT: u32 = 4; pub const PERF_RECORD_THROTTLE: u32 = 5; pub const PERF_RECORD_UNTHROTTLE: u32 = 6;
pub const PERF_RECORD_FORK: u32 = 7; pub const PERF_RECORD_READ: u32 = 8; pub const PERF_RECORD_SAMPLE: u32 = 9;
pub const PERF_RECORD_MMAP2: u32 = 10; pub const PERF_RECORD_AUX: u32 = 11; pub const PERF_RECORD_ITRACE_START: u32 = 12;
pub const PERF_RECORD_LOST_SAMPLES: u32 = 13; pub const PERF_RECORD_SWITCH: u32 = 14; pub const PERF_RECORD_SWITCH_CPU_WIDE: u32 = 15;
pub const PERF_RECORD_NAMESPACES: u32 = 16; pub const PERF_RECORD_KSYMBOL: u32 = 17; pub const PERF_RECORD_BPF_EVENT: u32 = 18;
pub const PERF_RECORD_CGROUP: u32 = 19; pub const PERF_RECORD_TEXT_POKE: u32 = 20; pub const PERF_RECORD_AUX_OUTPUT_HW_ID: u32 = 21;
pub const PERF_RECORD_CALLCHAIN_DEFERRED: u32 = 22; pub const PERF_RECORD_MAX: u32 = 23;
pub const PERF_RECORD_KSYMBOL_TYPE_UNKNOWN: u32 = 0; pub const PERF_RECORD_KSYMBOL_TYPE_BPF: u32 = 1;
pub const PERF_RECORD_KSYMBOL_TYPE_OOL: u32 = 2; pub const PERF_RECORD_KSYMBOL_TYPE_MAX: u32 = 3;
pub const PERF_RECORD_KSYMBOL_FLAGS_UNREGISTER: u32 = 1; pub const PERF_BPF_EVENT_UNKNOWN: u32 = 0;
pub const PERF_BPF_EVENT_PROG_LOAD: u32 = 1; pub const PERF_BPF_EVENT_PROG_UNLOAD: u32 = 2; pub const PERF_BPF_EVENT_MAX: u32 = 3;
pub const PERF_MAX_STACK_DEPTH: u32 = 127; pub const PERF_MAX_CONTEXTS_PER_STACK: u32 = 8;
pub const PERF_CONTEXT_HV: u64 = (-32i64) as u64; pub const PERF_CONTEXT_KERNEL: u64 = (-128i64) as u64;
pub const PERF_CONTEXT_USER: u64 = (-512i64) as u64; pub const PERF_CONTEXT_USER_DEFERRED: u64 = (-640i64) as u64;
pub const PERF_CONTEXT_GUEST: u64 = (-2048i64) as u64; pub const PERF_CONTEXT_GUEST_KERNEL: u64 = (-2176i64) as u64;
pub const PERF_CONTEXT_GUEST_USER: u64 = (-2560i64) as u64; pub const PERF_CONTEXT_MAX: u64 = (-4095i64) as u64;

pub const PERF_AUX_FLAG_TRUNCATED: u32 = 0x0001; pub const PERF_AUX_FLAG_OVERWRITE: u32 = 0x0002;
pub const PERF_AUX_FLAG_PARTIAL: u32 = 0x0004; pub const PERF_AUX_FLAG_COLLISION: u32 = 0x0008;
pub const PERF_AUX_FLAG_PMU_FORMAT_TYPE_MASK: u32 = 0xff00; pub const PERF_AUX_FLAG_CORESIGHT_FORMAT_CORESIGHT: u32 = 0;
pub const PERF_AUX_FLAG_CORESIGHT_FORMAT_RAW: u32 = 0x0100; pub const PERF_FLAG_FD_NO_GROUP: usize = 1 << 0;
pub const PERF_FLAG_FD_OUTPUT: usize = 1 << 1; pub const PERF_FLAG_PID_CGROUP: usize = 1 << 2; pub const PERF_FLAG_FD_CLOEXEC: usize = 1 << 3;

#[repr(C)] pub union perf_mem_data_src { pub val: __u64, pub bits: __u64 }
pub const PERF_MEM_OP_NA: u64 = 1; pub const PERF_MEM_OP_LOAD: u64 = 2; pub const PERF_MEM_OP_STORE: u64 = 4;
pub const PERF_MEM_OP_PFETCH: u64 = 8; pub const PERF_MEM_OP_EXEC: u64 = 16; pub const PERF_MEM_OP_SHIFT: u32 = 0;
pub const PERF_MEM_LVL_NA: u64 = 1; pub const PERF_MEM_LVL_HIT: u64 = 2; pub const PERF_MEM_LVL_MISS: u64 = 4;
pub const PERF_MEM_LVL_L1: u64 = 8; pub const PERF_MEM_LVL_LFB: u64 = 16; pub const PERF_MEM_LVL_L2: u64 = 32;
pub const PERF_MEM_LVL_L3: u64 = 64; pub const PERF_MEM_LVL_LOC_RAM: u64 = 128; pub const PERF_MEM_LVL_REM_RAM1: u64 = 0x100;
pub const PERF_MEM_LVL_REM_RAM2: u64 = 0x200; pub const PERF_MEM_LVL_REM_CCE1: u64 = 0x400; pub const PERF_MEM_LVL_REM_CCE2: u64 = 0x800;
pub const PERF_MEM_LVL_IO: u64 = 0x1000; pub const PERF_MEM_LVL_UNC: u64 = 0x2000; pub const PERF_MEM_LVL_SHIFT: u32 = 5;
pub const PERF_MEM_REMOTE_REMOTE: u64 = 1; pub const PERF_MEM_REMOTE_SHIFT: u32 = 37;
pub const PERF_MEM_SNOOP_NA: u64 = 1; pub const PERF_MEM_SNOOP_NONE: u64 = 2; pub const PERF_MEM_SNOOP_HIT: u64 = 4;
pub const PERF_MEM_SNOOP_MISS: u64 = 8; pub const PERF_MEM_SNOOP_HITM: u64 = 16; pub const PERF_MEM_SNOOP_SHIFT: u32 = 19;
pub const PERF_MEM_SNOOPX_FWD: u64 = 1; pub const PERF_MEM_SNOOPX_PEER: u64 = 2; pub const PERF_MEM_SNOOPX_SHIFT: u32 = 38;
pub const PERF_MEM_LOCK_NA: u64 = 1; pub const PERF_MEM_LOCK_LOCKED: u64 = 2; pub const PERF_MEM_LOCK_SHIFT: u32 = 24;
pub const PERF_MEM_TLB_NA: u64 = 1; pub const PERF_MEM_TLB_HIT: u64 = 2; pub const PERF_MEM_TLB_MISS: u64 = 4;
pub const PERF_MEM_TLB_L1: u64 = 8; pub const PERF_MEM_TLB_L2: u64 = 16; pub const PERF_MEM_TLB_WK: u64 = 32;
pub const PERF_MEM_TLB_OS: u64 = 64; pub const PERF_MEM_TLB_SHIFT: u32 = 26;
pub const PERF_MEM_BLK_NA: u64 = 1; pub const PERF_MEM_BLK_DATA: u64 = 2; pub const PERF_MEM_BLK_ADDR: u64 = 4; pub const PERF_MEM_BLK_SHIFT: u32 = 40;
pub const PERF_MEM_HOPS_0: u64 = 1; pub const PERF_MEM_HOPS_1: u64 = 2; pub const PERF_MEM_HOPS_2: u64 = 3; pub const PERF_MEM_HOPS_3: u64 = 4; pub const PERF_MEM_HOPS_SHIFT: u32 = 43;
pub const PERF_MEM_REGION_NA: u64 = 0; pub const PERF_MEM_REGION_RSVD: u64 = 1; pub const PERF_MEM_REGION_L_SHARE: u64 = 2; pub const PERF_MEM_REGION_L_NON_SHARE: u64 = 3;
pub const PERF_MEM_REGION_O_IO: u64 = 4; pub const PERF_MEM_REGION_O_SHARE: u64 = 5; pub const PERF_MEM_REGION_O_NON_SHARE: u64 = 6; pub const PERF_MEM_REGION_MMIO: u64 = 7;
pub const PERF_MEM_REGION_MEM0: u64 = 8; pub const PERF_MEM_REGION_MEM1: u64 = 9; pub const PERF_MEM_REGION_MEM2: u64 = 10; pub const PERF_MEM_REGION_MEM3: u64 = 11;
pub const PERF_MEM_REGION_MEM4: u64 = 12; pub const PERF_MEM_REGION_MEM5: u64 = 13; pub const PERF_MEM_REGION_MEM6: u64 = 14; pub const PERF_MEM_REGION_MEM7: u64 = 15; pub const PERF_MEM_REGION_SHIFT: u32 = 46;

#[repr(C)] pub struct perf_branch_entry { pub from: __u64, pub to: __u64, pub info: __u64 }
pub const PERF_BRANCH_ENTRY_INFO_BITS_MAX: u32 = 33;
#[repr(C)] pub union perf_sample_weight { pub full: __u64, pub bits: [__u8; 8] }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
