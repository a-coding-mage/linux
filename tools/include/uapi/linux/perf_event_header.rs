/* SPDX-License-Identifier: GPL-2.0 WITH Linux-syscall-note */
/*
 * Performance events:
 *
 *    Copyright (C) 2008-2009, Linutronix GmbH, Thomas Gleixner <tglx@kernel.org>
 *    Copyright (C) 2008-2011, Red Hat, Inc., Ingo Molnar
 *    Copyright (C) 2008-2011, Red Hat, Inc., Peter Zijlstra
 *
 * Data type definitions, declarations, prototypes.
 *
 *    Started by: Thomas Gleixner and Ingo Molnar
 *
 * For licencing details see kernel-base/COPYING
 */

/* C dependencies: <linux/types.h>, <linux/ioctl.h>, <asm/byteorder.h>. */
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]

pub type __u8 = u8;
pub type __u16 = u16;
pub type __u32 = u32;
pub type __u64 = u64;
pub type __s32 = i32;
pub type __s64 = i64;

/*
 * User-space ABI bits:
 */

/*
 * attr.type
 */
pub const PERF_TYPE_HARDWARE: __u32 = 0;
pub const PERF_TYPE_SOFTWARE: __u32 = 1;
pub const PERF_TYPE_TRACEPOINT: __u32 = 2;
pub const PERF_TYPE_HW_CACHE: __u32 = 3;
pub const PERF_TYPE_RAW: __u32 = 4;
pub const PERF_TYPE_BREAKPOINT: __u32 = 5;
pub const PERF_TYPE_MAX: __u32 = 6; /* non-ABI */

/*
 * attr.config layout for type PERF_TYPE_HARDWARE and PERF_TYPE_HW_CACHE
 *
 * PERF_TYPE_HARDWARE:          0xEEEEEEEE000000AA
 *                              AA: hardware event ID
 *                              EEEEEEEE: PMU type ID
 *
 * PERF_TYPE_HW_CACHE:          0xEEEEEEEE00DDCCBB
 *                              BB: hardware cache ID
 *                              CC: hardware cache op ID
 *                              DD: hardware cache op result ID
 *                              EEEEEEEE: PMU type ID
 *
 * If the PMU type ID is 0, PERF_TYPE_RAW will be applied.
 */
pub const PERF_PMU_TYPE_SHIFT: __u32 = 32;
pub const PERF_HW_EVENT_MASK: __u32 = 0xffffffff;

/*
 * Generalized performance event event_id types, used by the
 * attr.event_id parameter of the sys_perf_event_open()
 * syscall:
 */
pub const PERF_COUNT_HW_CPU_CYCLES: __u32 = 0;
pub const PERF_COUNT_HW_INSTRUCTIONS: __u32 = 1;
pub const PERF_COUNT_HW_CACHE_REFERENCES: __u32 = 2;
pub const PERF_COUNT_HW_CACHE_MISSES: __u32 = 3;
pub const PERF_COUNT_HW_BRANCH_INSTRUCTIONS: __u32 = 4;
pub const PERF_COUNT_HW_BRANCH_MISSES: __u32 = 5;
pub const PERF_COUNT_HW_BUS_CYCLES: __u32 = 6;
pub const PERF_COUNT_HW_STALLED_CYCLES_FRONTEND: __u32 = 7;
pub const PERF_COUNT_HW_STALLED_CYCLES_BACKEND: __u32 = 8;
pub const PERF_COUNT_HW_REF_CPU_CYCLES: __u32 = 9;
pub const PERF_COUNT_HW_MAX: __u32 = 10; /* non-ABI */

/*
 * Generalized hardware cache events:
 *
 *       { L1-D, L1-I, LLC, ITLB, DTLB, BPU, NODE } x
 *       { read, write, prefetch } x
 *       { accesses, misses }
 */
pub const PERF_COUNT_HW_CACHE_L1D: __u32 = 0;
pub const PERF_COUNT_HW_CACHE_L1I: __u32 = 1;
pub const PERF_COUNT_HW_CACHE_LL: __u32 = 2;
pub const PERF_COUNT_HW_CACHE_DTLB: __u32 = 3;
pub const PERF_COUNT_HW_CACHE_ITLB: __u32 = 4;
pub const PERF_COUNT_HW_CACHE_BPU: __u32 = 5;
pub const PERF_COUNT_HW_CACHE_NODE: __u32 = 6;
pub const PERF_COUNT_HW_CACHE_MAX: __u32 = 7; /* non-ABI */

pub const PERF_COUNT_HW_CACHE_OP_READ: __u32 = 0;
pub const PERF_COUNT_HW_CACHE_OP_WRITE: __u32 = 1;
pub const PERF_COUNT_HW_CACHE_OP_PREFETCH: __u32 = 2;
pub const PERF_COUNT_HW_CACHE_OP_MAX: __u32 = 3; /* non-ABI */

pub const PERF_COUNT_HW_CACHE_RESULT_ACCESS: __u32 = 0;
pub const PERF_COUNT_HW_CACHE_RESULT_MISS: __u32 = 1;
pub const PERF_COUNT_HW_CACHE_RESULT_MAX: __u32 = 2; /* non-ABI */

/*
 * Special "software" events provided by the kernel, even if the hardware
 * does not support performance events. These events measure various
 * physical and SW events of the kernel (and allow the profiling of them as
 * well):
 */
pub const PERF_COUNT_SW_CPU_CLOCK: __u32 = 0;
pub const PERF_COUNT_SW_TASK_CLOCK: __u32 = 1;
pub const PERF_COUNT_SW_PAGE_FAULTS: __u32 = 2;
pub const PERF_COUNT_SW_CONTEXT_SWITCHES: __u32 = 3;
pub const PERF_COUNT_SW_CPU_MIGRATIONS: __u32 = 4;
pub const PERF_COUNT_SW_PAGE_FAULTS_MIN: __u32 = 5;
pub const PERF_COUNT_SW_PAGE_FAULTS_MAJ: __u32 = 6;
pub const PERF_COUNT_SW_ALIGNMENT_FAULTS: __u32 = 7;
pub const PERF_COUNT_SW_EMULATION_FAULTS: __u32 = 8;
pub const PERF_COUNT_SW_DUMMY: __u32 = 9;
pub const PERF_COUNT_SW_BPF_OUTPUT: __u32 = 10;
pub const PERF_COUNT_SW_CGROUP_SWITCHES: __u32 = 11;
pub const PERF_COUNT_SW_MAX: __u32 = 12; /* non-ABI */

/*
 * Bits that can be set in attr.sample_type to request information
 * in the overflow packets.
 */
pub const PERF_SAMPLE_IP: __u64 = 1u64 << 0;
pub const PERF_SAMPLE_TID: __u64 = 1u64 << 1;
pub const PERF_SAMPLE_TIME: __u64 = 1u64 << 2;
pub const PERF_SAMPLE_ADDR: __u64 = 1u64 << 3;
pub const PERF_SAMPLE_READ: __u64 = 1u64 << 4;
pub const PERF_SAMPLE_CALLCHAIN: __u64 = 1u64 << 5;
pub const PERF_SAMPLE_ID: __u64 = 1u64 << 6;
pub const PERF_SAMPLE_CPU: __u64 = 1u64 << 7;
pub const PERF_SAMPLE_PERIOD: __u64 = 1u64 << 8;
pub const PERF_SAMPLE_STREAM_ID: __u64 = 1u64 << 9;
pub const PERF_SAMPLE_RAW: __u64 = 1u64 << 10;
pub const PERF_SAMPLE_BRANCH_STACK: __u64 = 1u64 << 11;
pub const PERF_SAMPLE_REGS_USER: __u64 = 1u64 << 12;
pub const PERF_SAMPLE_STACK_USER: __u64 = 1u64 << 13;
pub const PERF_SAMPLE_WEIGHT: __u64 = 1u64 << 14;
pub const PERF_SAMPLE_DATA_SRC: __u64 = 1u64 << 15;
pub const PERF_SAMPLE_IDENTIFIER: __u64 = 1u64 << 16;
pub const PERF_SAMPLE_TRANSACTION: __u64 = 1u64 << 17;
pub const PERF_SAMPLE_REGS_INTR: __u64 = 1u64 << 18;
pub const PERF_SAMPLE_PHYS_ADDR: __u64 = 1u64 << 19;
pub const PERF_SAMPLE_AUX: __u64 = 1u64 << 20;
pub const PERF_SAMPLE_CGROUP: __u64 = 1u64 << 21;
pub const PERF_SAMPLE_DATA_PAGE_SIZE: __u64 = 1u64 << 22;
pub const PERF_SAMPLE_CODE_PAGE_SIZE: __u64 = 1u64 << 23;
pub const PERF_SAMPLE_WEIGHT_STRUCT: __u64 = 1u64 << 24;
pub const PERF_SAMPLE_MAX: __u64 = 1u64 << 25; /* non-ABI */
pub const PERF_SAMPLE_WEIGHT_TYPE: __u64 = PERF_SAMPLE_WEIGHT | PERF_SAMPLE_WEIGHT_STRUCT;

/*
 * Values to program into branch_sample_type when PERF_SAMPLE_BRANCH is set.
 *
 * If the user does not pass priv level information via branch_sample_type,
 * the kernel uses the event's priv level. Branch and event priv levels do
 * not have to match. Branch priv level is checked for permissions.
 *
 * The branch types can be combined, however BRANCH_ANY covers all types
 * of branches and therefore it supersedes all the other types.
 */
pub const PERF_SAMPLE_BRANCH_USER_SHIFT: __u32 = 0;
pub const PERF_SAMPLE_BRANCH_KERNEL_SHIFT: __u32 = 1;
pub const PERF_SAMPLE_BRANCH_HV_SHIFT: __u32 = 2;
pub const PERF_SAMPLE_BRANCH_ANY_SHIFT: __u32 = 3;
pub const PERF_SAMPLE_BRANCH_ANY_CALL_SHIFT: __u32 = 4;
pub const PERF_SAMPLE_BRANCH_ANY_RETURN_SHIFT: __u32 = 5;
pub const PERF_SAMPLE_BRANCH_IND_CALL_SHIFT: __u32 = 6;
pub const PERF_SAMPLE_BRANCH_ABORT_TX_SHIFT: __u32 = 7;
pub const PERF_SAMPLE_BRANCH_IN_TX_SHIFT: __u32 = 8;
pub const PERF_SAMPLE_BRANCH_NO_TX_SHIFT: __u32 = 9;
pub const PERF_SAMPLE_BRANCH_COND_SHIFT: __u32 = 10;
pub const PERF_SAMPLE_BRANCH_CALL_STACK_SHIFT: __u32 = 11;
pub const PERF_SAMPLE_BRANCH_IND_JUMP_SHIFT: __u32 = 12;
pub const PERF_SAMPLE_BRANCH_CALL_SHIFT: __u32 = 13;
pub const PERF_SAMPLE_BRANCH_NO_FLAGS_SHIFT: __u32 = 14;
pub const PERF_SAMPLE_BRANCH_NO_CYCLES_SHIFT: __u32 = 15;
pub const PERF_SAMPLE_BRANCH_TYPE_SAVE_SHIFT: __u32 = 16;
pub const PERF_SAMPLE_BRANCH_HW_INDEX_SHIFT: __u32 = 17;
pub const PERF_SAMPLE_BRANCH_PRIV_SAVE_SHIFT: __u32 = 18;
pub const PERF_SAMPLE_BRANCH_COUNTERS_SHIFT: __u32 = 19;
pub const PERF_SAMPLE_BRANCH_MAX_SHIFT: __u32 = 20; /* non-ABI */

pub const PERF_SAMPLE_BRANCH_USER: __u64 = 1u64 << PERF_SAMPLE_BRANCH_USER_SHIFT;
pub const PERF_SAMPLE_BRANCH_KERNEL: __u64 = 1u64 << PERF_SAMPLE_BRANCH_KERNEL_SHIFT;
pub const PERF_SAMPLE_BRANCH_HV: __u64 = 1u64 << PERF_SAMPLE_BRANCH_HV_SHIFT;
pub const PERF_SAMPLE_BRANCH_ANY: __u64 = 1u64 << PERF_SAMPLE_BRANCH_ANY_SHIFT;
pub const PERF_SAMPLE_BRANCH_ANY_CALL: __u64 = 1u64 << PERF_SAMPLE_BRANCH_ANY_CALL_SHIFT;
pub const PERF_SAMPLE_BRANCH_ANY_RETURN: __u64 = 1u64 << PERF_SAMPLE_BRANCH_ANY_RETURN_SHIFT;
pub const PERF_SAMPLE_BRANCH_IND_CALL: __u64 = 1u64 << PERF_SAMPLE_BRANCH_IND_CALL_SHIFT;
pub const PERF_SAMPLE_BRANCH_ABORT_TX: __u64 = 1u64 << PERF_SAMPLE_BRANCH_ABORT_TX_SHIFT;
pub const PERF_SAMPLE_BRANCH_IN_TX: __u64 = 1u64 << PERF_SAMPLE_BRANCH_IN_TX_SHIFT;
pub const PERF_SAMPLE_BRANCH_NO_TX: __u64 = 1u64 << PERF_SAMPLE_BRANCH_NO_TX_SHIFT;
pub const PERF_SAMPLE_BRANCH_COND: __u64 = 1u64 << PERF_SAMPLE_BRANCH_COND_SHIFT;
pub const PERF_SAMPLE_BRANCH_CALL_STACK: __u64 = 1u64 << PERF_SAMPLE_BRANCH_CALL_STACK_SHIFT;
pub const PERF_SAMPLE_BRANCH_IND_JUMP: __u64 = 1u64 << PERF_SAMPLE_BRANCH_IND_JUMP_SHIFT;
pub const PERF_SAMPLE_BRANCH_CALL: __u64 = 1u64 << PERF_SAMPLE_BRANCH_CALL_SHIFT;
pub const PERF_SAMPLE_BRANCH_NO_FLAGS: __u64 = 1u64 << PERF_SAMPLE_BRANCH_NO_FLAGS_SHIFT;
pub const PERF_SAMPLE_BRANCH_NO_CYCLES: __u64 = 1u64 << PERF_SAMPLE_BRANCH_NO_CYCLES_SHIFT;
pub const PERF_SAMPLE_BRANCH_TYPE_SAVE: __u64 = 1u64 << PERF_SAMPLE_BRANCH_TYPE_SAVE_SHIFT;
pub const PERF_SAMPLE_BRANCH_HW_INDEX: __u64 = 1u64 << PERF_SAMPLE_BRANCH_HW_INDEX_SHIFT;
pub const PERF_SAMPLE_BRANCH_PRIV_SAVE: __u64 = 1u64 << PERF_SAMPLE_BRANCH_PRIV_SAVE_SHIFT;
pub const PERF_SAMPLE_BRANCH_COUNTERS: __u64 = 1u64 << PERF_SAMPLE_BRANCH_COUNTERS_SHIFT;
pub const PERF_SAMPLE_BRANCH_MAX: __u64 = 1u64 << PERF_SAMPLE_BRANCH_MAX_SHIFT;

/*
 * Common control flow change classifications:
 */
pub const PERF_BR_UNKNOWN: __u32 = 0;
pub const PERF_BR_COND: __u32 = 1;
pub const PERF_BR_UNCOND: __u32 = 2;
pub const PERF_BR_IND: __u32 = 3;
pub const PERF_BR_CALL: __u32 = 4;
pub const PERF_BR_IND_CALL: __u32 = 5;
pub const PERF_BR_RET: __u32 = 6;
pub const PERF_BR_SYSCALL: __u32 = 7;
pub const PERF_BR_SYSRET: __u32 = 8;
pub const PERF_BR_COND_CALL: __u32 = 9;
pub const PERF_BR_COND_RET: __u32 = 10;
pub const PERF_BR_ERET: __u32 = 11;
pub const PERF_BR_IRQ: __u32 = 12;
pub const PERF_BR_SERROR: __u32 = 13;
pub const PERF_BR_NO_TX: __u32 = 14;
pub const PERF_BR_EXTEND_ABI: __u32 = 15;
pub const PERF_BR_MAX: __u32 = 16;

/*
 * Common branch speculation outcome classifications:
 */
pub const PERF_BR_SPEC_NA: __u32 = 0;
pub const PERF_BR_SPEC_WRONG_PATH: __u32 = 1;
pub const PERF_BR_NON_SPEC_CORRECT_PATH: __u32 = 2;
pub const PERF_BR_SPEC_CORRECT_PATH: __u32 = 3;
pub const PERF_BR_SPEC_MAX: __u32 = 4;

pub const PERF_BR_NEW_FAULT_ALGN: __u32 = 0;
pub const PERF_BR_NEW_FAULT_DATA: __u32 = 1;
pub const PERF_BR_NEW_FAULT_INST: __u32 = 2;
pub const PERF_BR_NEW_ARCH_1: __u32 = 3;
pub const PERF_BR_NEW_ARCH_2: __u32 = 4;
pub const PERF_BR_NEW_ARCH_3: __u32 = 5;
pub const PERF_BR_NEW_ARCH_4: __u32 = 6;
pub const PERF_BR_NEW_ARCH_5: __u32 = 7;
pub const PERF_BR_NEW_MAX: __u32 = 8;

pub const PERF_BR_PRIV_UNKNOWN: __u32 = 0;
pub const PERF_BR_PRIV_USER: __u32 = 1;
pub const PERF_BR_PRIV_KERNEL: __u32 = 2;
pub const PERF_BR_PRIV_HV: __u32 = 3;

pub const PERF_BR_ARM64_FIQ: __u32 = PERF_BR_NEW_ARCH_1;
pub const PERF_BR_ARM64_DEBUG_HALT: __u32 = PERF_BR_NEW_ARCH_2;
pub const PERF_BR_ARM64_DEBUG_EXIT: __u32 = PERF_BR_NEW_ARCH_3;
pub const PERF_BR_ARM64_DEBUG_INST: __u32 = PERF_BR_NEW_ARCH_4;
pub const PERF_BR_ARM64_DEBUG_DATA: __u32 = PERF_BR_NEW_ARCH_5;

pub const PERF_SAMPLE_BRANCH_PLM_ALL: __u64 =
    PERF_SAMPLE_BRANCH_USER | PERF_SAMPLE_BRANCH_KERNEL | PERF_SAMPLE_BRANCH_HV;

/*
 * Values to determine ABI of the registers dump.
 */
pub const PERF_SAMPLE_REGS_ABI_NONE: __u32 = 0;
pub const PERF_SAMPLE_REGS_ABI_32: __u32 = 1;
pub const PERF_SAMPLE_REGS_ABI_64: __u32 = 2;

/*
 * Values for the memory transaction event qualifier, mostly for
 * abort events. Multiple bits can be set.
 */
pub const PERF_TXN_ELISION: __u64 = 1u64 << 0;
pub const PERF_TXN_TRANSACTION: __u64 = 1u64 << 1;
pub const PERF_TXN_SYNC: __u64 = 1u64 << 2;
pub const PERF_TXN_ASYNC: __u64 = 1u64 << 3;
pub const PERF_TXN_RETRY: __u64 = 1u64 << 4;
pub const PERF_TXN_CONFLICT: __u64 = 1u64 << 5;
pub const PERF_TXN_CAPACITY_WRITE: __u64 = 1u64 << 6;
pub const PERF_TXN_CAPACITY_READ: __u64 = 1u64 << 7;
pub const PERF_TXN_MAX: __u64 = 1u64 << 8; /* non-ABI */
/* Bits 32..63 are reserved for the abort code */
pub const PERF_TXN_ABORT_MASK: __u64 = 0xffffffffu64 << 32;
pub const PERF_TXN_ABORT_SHIFT: __u32 = 32;

/*
 * The format of the data returned by read() on a perf event fd,
 * as specified by attr.read_format.
 */
pub const PERF_FORMAT_TOTAL_TIME_ENABLED: __u64 = 1u64 << 0;
pub const PERF_FORMAT_TOTAL_TIME_RUNNING: __u64 = 1u64 << 1;
pub const PERF_FORMAT_ID: __u64 = 1u64 << 2;
pub const PERF_FORMAT_GROUP: __u64 = 1u64 << 3;
pub const PERF_FORMAT_LOST: __u64 = 1u64 << 4;
pub const PERF_FORMAT_MAX: __u64 = 1u64 << 5; /* non-ABI */

pub const PERF_ATTR_SIZE_VER0: __u32 = 64; /* Size of first published 'struct perf_event_attr' */
pub const PERF_ATTR_SIZE_VER1: __u32 = 72; /* Add: config2 */
pub const PERF_ATTR_SIZE_VER2: __u32 = 80; /* Add: branch_sample_type */
pub const PERF_ATTR_SIZE_VER3: __u32 = 96; /* Add: sample_regs_user; sample_stack_user */
pub const PERF_ATTR_SIZE_VER4: __u32 = 104; /* Add: sample_regs_intr */
pub const PERF_ATTR_SIZE_VER5: __u32 = 112; /* Add: aux_watermark */
pub const PERF_ATTR_SIZE_VER6: __u32 = 120; /* Add: aux_sample_size */
pub const PERF_ATTR_SIZE_VER7: __u32 = 128; /* Add: sig_data */
pub const PERF_ATTR_SIZE_VER8: __u32 = 136; /* Add: config3 */
pub const PERF_ATTR_SIZE_VER9: __u32 = 144; /* add: config4 */

#[repr(C)]
#[derive(Copy, Clone)]
pub union perf_event_attr_sample {
    pub sample_period: __u64,
    pub sample_freq: __u64,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub union perf_event_attr_wakeup {
    pub wakeup_events: __u32,
    pub wakeup_watermark: __u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub union perf_event_attr_bp_addr {
    pub bp_addr: __u64,
    pub kprobe_func: __u64,
    pub uprobe_path: __u64,
    pub config1: __u64,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub union perf_event_attr_bp_len {
    pub bp_len: __u64,
    pub kprobe_addr: __u64,
    pub probe_offset: __u64,
    pub config2: __u64,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub union perf_event_attr_aux {
    pub aux_action: __u32,
    /*
     * Bit layout for aux_action:
     * aux_start_paused:1, aux_pause:1, aux_resume:1, __reserved_3:29.
     */
    pub bits: __u32,
}

/*
 * 'struct perf_event_attr' contains various attributes that define
 * a performance event - most of them hardware related configuration
 * details, but also a lot of behavioral switches and values implemented
 * by the kernel.
 */
#[repr(C)]
#[derive(Copy, Clone)]
pub struct perf_event_attr {
    pub type_: __u32,
    pub size: __u32,
    pub config: __u64,
    pub sample: perf_event_attr_sample,
    pub sample_type: __u64,
    pub read_format: __u64,
    /*
     * C bitfield storage, in declaration order:
     * disabled:1, inherit:1, pinned:1, exclusive:1, exclude_user:1,
     * exclude_kernel:1, exclude_hv:1, exclude_idle:1, mmap:1, comm:1,
     * freq:1, inherit_stat:1, enable_on_exec:1, task:1, watermark:1,
     * precise_ip:2, mmap_data:1, sample_id_all:1, exclude_host:1,
     * exclude_guest:1, exclude_callchain_kernel:1, exclude_callchain_user:1,
     * mmap2:1, comm_exec:1, use_clockid:1, context_switch:1,
     * write_backward:1, namespaces:1, ksymbol:1, bpf_event:1,
     * aux_output:1, cgroup:1, text_poke:1, build_id:1,
     * inherit_thread:1, remove_on_exec:1, sigtrap:1, defer_callchain:1,
     * defer_output:1, __reserved_1:24.
     */
    pub flags: __u64,
    pub wakeup: perf_event_attr_wakeup,
    pub bp_type: __u32,
    pub bp_addr: perf_event_attr_bp_addr,
    pub bp_len: perf_event_attr_bp_len,
    pub branch_sample_type: __u64,
    pub sample_regs_user: __u64,
    pub sample_stack_user: __u32,
    pub clockid: __s32,
    pub sample_regs_intr: __u64,
    pub aux_watermark: __u32,
    pub sample_max_stack: __u16,
    pub __reserved_2: __u16,
    pub aux_sample_size: __u32,
    pub aux: perf_event_attr_aux,
    pub sig_data: __u64,
    pub config3: __u64,
    pub config4: __u64,
}

/*
 * Structure used by below PERF_EVENT_IOC_QUERY_BPF command
 * to query BPF programs attached to the same perf tracepoint
 * as the given perf event.
 */
#[repr(C)]
pub struct perf_event_query_bpf {
    pub ids_len: __u32,
    pub prog_cnt: __u32,
    /* Flexible array member: __u32 ids[]; */
    pub ids: [__u32; 0],
}

/*
 * Ioctls that can be done on a perf event fd:
 *
 * C definitions used _IO/_IOW/_IOR/_IOWR from <linux/ioctl.h>. The Rust
 * expressions below preserve the Linux ioctl encoding for this header.
 */
pub const IOC_NRBITS: __u32 = 8;
pub const IOC_TYPEBITS: __u32 = 8;
pub const IOC_SIZEBITS: __u32 = 14;
pub const IOC_DIRBITS: __u32 = 2;
pub const IOC_NRMASK: __u32 = (1 << IOC_NRBITS) - 1;
pub const IOC_TYPEMASK: __u32 = (1 << IOC_TYPEBITS) - 1;
pub const IOC_SIZEMASK: __u32 = (1 << IOC_SIZEBITS) - 1;
pub const IOC_DIRMASK: __u32 = (1 << IOC_DIRBITS) - 1;
pub const IOC_NRSHIFT: __u32 = 0;
pub const IOC_TYPESHIFT: __u32 = IOC_NRSHIFT + IOC_NRBITS;
pub const IOC_SIZESHIFT: __u32 = IOC_TYPESHIFT + IOC_TYPEBITS;
pub const IOC_DIRSHIFT: __u32 = IOC_SIZESHIFT + IOC_SIZEBITS;
pub const IOC_NONE: __u32 = 0;
pub const IOC_WRITE: __u32 = 1;
pub const IOC_READ: __u32 = 2;
pub const fn _IOC(dir: __u32, type_: __u32, nr: __u32, size: __u32) -> __u64 {
    ((dir as __u64) << IOC_DIRSHIFT)
        | ((type_ as __u64) << IOC_TYPESHIFT)
        | ((nr as __u64) << IOC_NRSHIFT)
        | ((size as __u64) << IOC_SIZESHIFT)
}
pub const fn _IO(type_: __u32, nr: __u32) -> __u64 { _IOC(IOC_NONE, type_, nr, 0) }
pub const fn _IOR<T>(type_: __u32, nr: __u32) -> __u64 {
    _IOC(IOC_READ, type_, nr, core::mem::size_of::<T>() as __u32)
}
pub const fn _IOW<T>(type_: __u32, nr: __u32) -> __u64 {
    _IOC(IOC_WRITE, type_, nr, core::mem::size_of::<T>() as __u32)
}
pub const fn _IOWR<T>(type_: __u32, nr: __u32) -> __u64 {
    _IOC(IOC_READ | IOC_WRITE, type_, nr, core::mem::size_of::<T>() as __u32)
}

pub const PERF_EVENT_IOC_ENABLE: __u64 = _IO(b'$' as __u32, 0);
pub const PERF_EVENT_IOC_DISABLE: __u64 = _IO(b'$' as __u32, 1);
pub const PERF_EVENT_IOC_REFRESH: __u64 = _IO(b'$' as __u32, 2);
pub const PERF_EVENT_IOC_RESET: __u64 = _IO(b'$' as __u32, 3);
pub const PERF_EVENT_IOC_PERIOD: __u64 = _IOW::<__u64>(b'$' as __u32, 4);
pub const PERF_EVENT_IOC_SET_OUTPUT: __u64 = _IO(b'$' as __u32, 5);
pub const PERF_EVENT_IOC_SET_FILTER: __u64 = _IOW::<*mut core::ffi::c_char>(b'$' as __u32, 6);
pub const PERF_EVENT_IOC_ID: __u64 = _IOR::<*mut __u64>(b'$' as __u32, 7);
pub const PERF_EVENT_IOC_SET_BPF: __u64 = _IOW::<__u32>(b'$' as __u32, 8);
pub const PERF_EVENT_IOC_PAUSE_OUTPUT: __u64 = _IOW::<__u32>(b'$' as __u32, 9);
pub const PERF_EVENT_IOC_QUERY_BPF: __u64 = _IOWR::<*mut perf_event_query_bpf>(b'$' as __u32, 10);
pub const PERF_EVENT_IOC_MODIFY_ATTRIBUTES: __u64 = _IOW::<*mut perf_event_attr>(b'$' as __u32, 11);

pub const PERF_IOC_FLAG_GROUP: __u32 = 1u32 << 0;

#[repr(C)]
#[derive(Copy, Clone)]
pub union perf_event_mmap_page_cap {
    pub capabilities: __u64,
    /*
     * Bit layout:
     * cap_bit0:1, cap_bit0_is_deprecated:1, cap_user_rdpmc:1,
     * cap_user_time:1, cap_user_time_zero:1, cap_user_time_short:1,
     * cap_____res:58.
     */
    pub bits: __u64,
}

/*
 * Structure of the page that can be mapped via mmap
 */
#[repr(C)]
#[derive(Copy, Clone)]
pub struct perf_event_mmap_page {
    pub version: __u32,
    pub compat_version: __u32,
    pub lock: __u32,
    pub index: __u32,
    pub offset: __s64,
    pub time_enabled: __u64,
    pub time_running: __u64,
    pub cap: perf_event_mmap_page_cap,
    pub pmc_width: __u16,
    pub time_shift: __u16,
    pub time_mult: __u32,
    pub time_offset: __u64,
    pub time_zero: __u64,
    pub size: __u32,
    pub __reserved_1: __u32,
    pub time_cycles: __u64,
    pub time_mask: __u64,
    /* Hole for extension of the self monitor capabilities; align to 1k. */
    pub __reserved: [__u8; 116 * 8],
    pub data_head: __u64,
    pub data_tail: __u64,
    pub data_offset: __u64,
    pub data_size: __u64,
    pub aux_head: __u64,
    pub aux_tail: __u64,
    pub aux_offset: __u64,
    pub aux_size: __u64,
}

/*
 * The current state of perf_event_header::misc bits usage:
 * ('|' used bit, '-' unused bit)
 *
 *  012         CDEF
 *  |||---------||||
 *
 *  Where:
 *    0-2     CPUMODE_MASK
 *
 *    C       PROC_MAP_PARSE_TIMEOUT
 *    D       MMAP_DATA / COMM_EXEC / FORK_EXEC / SWITCH_OUT
 *    E       MMAP_BUILD_ID / EXACT_IP / SCHED_OUT_PREEMPT
 *    F       (reserved)
 */
pub const PERF_RECORD_MISC_CPUMODE_MASK: __u16 = 7 << 0;
pub const PERF_RECORD_MISC_CPUMODE_UNKNOWN: __u16 = 0 << 0;
pub const PERF_RECORD_MISC_KERNEL: __u16 = 1 << 0;
pub const PERF_RECORD_MISC_USER: __u16 = 2 << 0;
pub const PERF_RECORD_MISC_HYPERVISOR: __u16 = 3 << 0;
pub const PERF_RECORD_MISC_GUEST_KERNEL: __u16 = 4 << 0;
pub const PERF_RECORD_MISC_GUEST_USER: __u16 = 5 << 0;
pub const PERF_RECORD_MISC_PROC_MAP_PARSE_TIMEOUT: __u16 = 1 << 12;
pub const PERF_RECORD_MISC_MMAP_DATA: __u16 = 1 << 13;
pub const PERF_RECORD_MISC_COMM_EXEC: __u16 = 1 << 13;
pub const PERF_RECORD_MISC_FORK_EXEC: __u16 = 1 << 13;
pub const PERF_RECORD_MISC_SWITCH_OUT: __u16 = 1 << 13;
pub const PERF_RECORD_MISC_EXACT_IP: __u16 = 1 << 14;
pub const PERF_RECORD_MISC_SWITCH_OUT_PREEMPT: __u16 = 1 << 14;
pub const PERF_RECORD_MISC_MMAP_BUILD_ID: __u16 = 1 << 14;
pub const PERF_RECORD_MISC_EXT_RESERVED: __u16 = 1 << 15;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct perf_event_header {
    pub type_: __u32,
    pub misc: __u16,
    pub size: __u16,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct perf_ns_link_info {
    pub dev: __u64,
    pub ino: __u64,
}

pub const NET_NS_INDEX: __u32 = 0;
pub const UTS_NS_INDEX: __u32 = 1;
pub const IPC_NS_INDEX: __u32 = 2;
pub const PID_NS_INDEX: __u32 = 3;
pub const USER_NS_INDEX: __u32 = 4;
pub const MNT_NS_INDEX: __u32 = 5;
pub const CGROUP_NS_INDEX: __u32 = 6;
pub const NR_NAMESPACES: __u32 = 7; /* number of available namespaces */

/*
 * perf_event_type record payload layouts are described in the original C
 * comments and remain variable-length ABI records following perf_event_header.
 */
pub const PERF_RECORD_MMAP: __u32 = 1;
pub const PERF_RECORD_LOST: __u32 = 2;
pub const PERF_RECORD_COMM: __u32 = 3;
pub const PERF_RECORD_EXIT: __u32 = 4;
pub const PERF_RECORD_THROTTLE: __u32 = 5;
pub const PERF_RECORD_UNTHROTTLE: __u32 = 6;
pub const PERF_RECORD_FORK: __u32 = 7;
pub const PERF_RECORD_READ: __u32 = 8;
pub const PERF_RECORD_SAMPLE: __u32 = 9;
pub const PERF_RECORD_MMAP2: __u32 = 10;
pub const PERF_RECORD_AUX: __u32 = 11;
pub const PERF_RECORD_ITRACE_START: __u32 = 12;
pub const PERF_RECORD_LOST_SAMPLES: __u32 = 13;
pub const PERF_RECORD_SWITCH: __u32 = 14;
pub const PERF_RECORD_SWITCH_CPU_WIDE: __u32 = 15;
pub const PERF_RECORD_NAMESPACES: __u32 = 16;
pub const PERF_RECORD_KSYMBOL: __u32 = 17;
pub const PERF_RECORD_BPF_EVENT: __u32 = 18;
pub const PERF_RECORD_CGROUP: __u32 = 19;
pub const PERF_RECORD_TEXT_POKE: __u32 = 20;
pub const PERF_RECORD_AUX_OUTPUT_HW_ID: __u32 = 21;
pub const PERF_RECORD_CALLCHAIN_DEFERRED: __u32 = 22;
pub const PERF_RECORD_MAX: __u32 = 23; /* non-ABI */

pub const PERF_RECORD_KSYMBOL_TYPE_UNKNOWN: __u32 = 0;
pub const PERF_RECORD_KSYMBOL_TYPE_BPF: __u32 = 1;
/* Out of line code such as kprobe-replaced instructions or optimized kprobes or ftrace trampolines. */
pub const PERF_RECORD_KSYMBOL_TYPE_OOL: __u32 = 2;
pub const PERF_RECORD_KSYMBOL_TYPE_MAX: __u32 = 3; /* non-ABI */
pub const PERF_RECORD_KSYMBOL_FLAGS_UNREGISTER: __u32 = 1 << 0;

pub const PERF_BPF_EVENT_UNKNOWN: __u32 = 0;
pub const PERF_BPF_EVENT_PROG_LOAD: __u32 = 1;
pub const PERF_BPF_EVENT_PROG_UNLOAD: __u32 = 2;
pub const PERF_BPF_EVENT_MAX: __u32 = 3; /* non-ABI */

pub const PERF_MAX_STACK_DEPTH: __u32 = 127;
pub const PERF_MAX_CONTEXTS_PER_STACK: __u32 = 8;

pub const PERF_CONTEXT_HV: __u64 = (-32i64) as __u64;
pub const PERF_CONTEXT_KERNEL: __u64 = (-128i64) as __u64;
pub const PERF_CONTEXT_USER: __u64 = (-512i64) as __u64;
pub const PERF_CONTEXT_USER_DEFERRED: __u64 = (-640i64) as __u64;
pub const PERF_CONTEXT_GUEST: __u64 = (-2048i64) as __u64;
pub const PERF_CONTEXT_GUEST_KERNEL: __u64 = (-2176i64) as __u64;
pub const PERF_CONTEXT_GUEST_USER: __u64 = (-2560i64) as __u64;
pub const PERF_CONTEXT_MAX: __u64 = (-4095i64) as __u64;

/**
 * PERF_RECORD_AUX::flags bits
 */
pub const PERF_AUX_FLAG_TRUNCATED: __u32 = 0x0001;
pub const PERF_AUX_FLAG_OVERWRITE: __u32 = 0x0002;
pub const PERF_AUX_FLAG_PARTIAL: __u32 = 0x0004;
pub const PERF_AUX_FLAG_COLLISION: __u32 = 0x0008;
pub const PERF_AUX_FLAG_PMU_FORMAT_TYPE_MASK: __u32 = 0xff00;
/* CoreSight PMU AUX buffer formats */
pub const PERF_AUX_FLAG_CORESIGHT_FORMAT_CORESIGHT: __u32 = 0x0000;
pub const PERF_AUX_FLAG_CORESIGHT_FORMAT_RAW: __u32 = 0x0100;

pub const PERF_FLAG_FD_NO_GROUP: usize = 1usize << 0;
pub const PERF_FLAG_FD_OUTPUT: usize = 1usize << 1;
pub const PERF_FLAG_PID_CGROUP: usize = 1usize << 2; /* pid=cgroup ID, per-CPU mode only */
pub const PERF_FLAG_FD_CLOEXEC: usize = 1usize << 3; /* O_CLOEXEC */

#[repr(C)]
#[derive(Copy, Clone)]
pub union perf_mem_data_src {
    pub val: __u64,
    /*
     * Endian-dependent C bitfield view:
     * little-endian order:
     * mem_op:5, mem_lvl:14, mem_snoop:5, mem_lock:2, mem_dtlb:7,
     * mem_lvl_num:4, mem_remote:1, mem_snoopx:2, mem_blk:3,
     * mem_hops:3, mem_region:5, mem_rsvd:13.
     * big-endian order is the reverse declaration order from the C header.
     */
    pub bits: __u64,
}

/* Type of memory opcode: */
pub const PERF_MEM_OP_NA: __u64 = 0x0001;
pub const PERF_MEM_OP_LOAD: __u64 = 0x0002;
pub const PERF_MEM_OP_STORE: __u64 = 0x0004;
pub const PERF_MEM_OP_PFETCH: __u64 = 0x0008;
pub const PERF_MEM_OP_EXEC: __u64 = 0x0010;
pub const PERF_MEM_OP_SHIFT: __u32 = 0;

/*
 * The PERF_MEM_LVL_* namespace is being deprecated to some extent in
 * favour of newer composite PERF_MEM_{LVLNUM_,REMOTE_,SNOOPX_} fields.
 * We support this namespace in order to not break defined ABIs.
 *
 * Memory hierarchy (memory level, hit or miss)
 */
pub const PERF_MEM_LVL_NA: __u64 = 0x0001;
pub const PERF_MEM_LVL_HIT: __u64 = 0x0002;
pub const PERF_MEM_LVL_MISS: __u64 = 0x0004;
pub const PERF_MEM_LVL_L1: __u64 = 0x0008;
pub const PERF_MEM_LVL_LFB: __u64 = 0x0010;
pub const PERF_MEM_LVL_L2: __u64 = 0x0020;
pub const PERF_MEM_LVL_L3: __u64 = 0x0040;
pub const PERF_MEM_LVL_LOC_RAM: __u64 = 0x0080;
pub const PERF_MEM_LVL_REM_RAM1: __u64 = 0x0100;
pub const PERF_MEM_LVL_REM_RAM2: __u64 = 0x0200;
pub const PERF_MEM_LVL_REM_CCE1: __u64 = 0x0400;
pub const PERF_MEM_LVL_REM_CCE2: __u64 = 0x0800;
pub const PERF_MEM_LVL_IO: __u64 = 0x1000;
pub const PERF_MEM_LVL_UNC: __u64 = 0x2000;
pub const PERF_MEM_LVL_SHIFT: __u32 = 5;

pub const PERF_MEM_REMOTE_REMOTE: __u64 = 0x0001;
pub const PERF_MEM_REMOTE_SHIFT: __u32 = 37;

pub const PERF_MEM_LVLNUM_L1: __u64 = 0x0001;
pub const PERF_MEM_LVLNUM_L2: __u64 = 0x0002;
pub const PERF_MEM_LVLNUM_L3: __u64 = 0x0003;
pub const PERF_MEM_LVLNUM_L4: __u64 = 0x0004;
pub const PERF_MEM_LVLNUM_L2_MHB: __u64 = 0x0005;
pub const PERF_MEM_LVLNUM_MSC: __u64 = 0x0006;
pub const PERF_MEM_LVLNUM_L0: __u64 = 0x0007;
pub const PERF_MEM_LVLNUM_UNC: __u64 = 0x0008;
pub const PERF_MEM_LVLNUM_CXL: __u64 = 0x0009;
pub const PERF_MEM_LVLNUM_IO: __u64 = 0x000a;
pub const PERF_MEM_LVLNUM_ANY_CACHE: __u64 = 0x000b;
pub const PERF_MEM_LVLNUM_LFB: __u64 = 0x000c;
pub const PERF_MEM_LVLNUM_RAM: __u64 = 0x000d;
pub const PERF_MEM_LVLNUM_PMEM: __u64 = 0x000e;
pub const PERF_MEM_LVLNUM_NA: __u64 = 0x000f;
pub const PERF_MEM_LVLNUM_SHIFT: __u32 = 33;

/* Snoop mode */
pub const PERF_MEM_SNOOP_NA: __u64 = 0x0001;
pub const PERF_MEM_SNOOP_NONE: __u64 = 0x0002;
pub const PERF_MEM_SNOOP_HIT: __u64 = 0x0004;
pub const PERF_MEM_SNOOP_MISS: __u64 = 0x0008;
pub const PERF_MEM_SNOOP_HITM: __u64 = 0x0010;
pub const PERF_MEM_SNOOP_SHIFT: __u32 = 19;

pub const PERF_MEM_SNOOPX_FWD: __u64 = 0x0001;
pub const PERF_MEM_SNOOPX_PEER: __u64 = 0x0002;
pub const PERF_MEM_SNOOPX_SHIFT: __u32 = 38;

/* Locked instruction */
pub const PERF_MEM_LOCK_NA: __u64 = 0x0001;
pub const PERF_MEM_LOCK_LOCKED: __u64 = 0x0002;
pub const PERF_MEM_LOCK_SHIFT: __u32 = 24;

/* TLB access */
pub const PERF_MEM_TLB_NA: __u64 = 0x0001;
pub const PERF_MEM_TLB_HIT: __u64 = 0x0002;
pub const PERF_MEM_TLB_MISS: __u64 = 0x0004;
pub const PERF_MEM_TLB_L1: __u64 = 0x0008;
pub const PERF_MEM_TLB_L2: __u64 = 0x0010;
pub const PERF_MEM_TLB_WK: __u64 = 0x0020;
pub const PERF_MEM_TLB_OS: __u64 = 0x0040;
pub const PERF_MEM_TLB_SHIFT: __u32 = 26;

/* Access blocked */
pub const PERF_MEM_BLK_NA: __u64 = 0x0001;
pub const PERF_MEM_BLK_DATA: __u64 = 0x0002;
pub const PERF_MEM_BLK_ADDR: __u64 = 0x0004;
pub const PERF_MEM_BLK_SHIFT: __u32 = 40;

/* Hop level */
pub const PERF_MEM_HOPS_0: __u64 = 0x0001;
pub const PERF_MEM_HOPS_1: __u64 = 0x0002;
pub const PERF_MEM_HOPS_2: __u64 = 0x0003;
pub const PERF_MEM_HOPS_3: __u64 = 0x0004;
/* 5-7 available */
pub const PERF_MEM_HOPS_SHIFT: __u32 = 43;

/* Cache/Memory region */
pub const PERF_MEM_REGION_NA: __u64 = 0x0;
pub const PERF_MEM_REGION_RSVD: __u64 = 0x01;
pub const PERF_MEM_REGION_L_SHARE: __u64 = 0x02;
pub const PERF_MEM_REGION_L_NON_SHARE: __u64 = 0x03;
pub const PERF_MEM_REGION_O_IO: __u64 = 0x04;
pub const PERF_MEM_REGION_O_SHARE: __u64 = 0x05;
pub const PERF_MEM_REGION_O_NON_SHARE: __u64 = 0x06;
pub const PERF_MEM_REGION_MMIO: __u64 = 0x07;
pub const PERF_MEM_REGION_MEM0: __u64 = 0x08;
pub const PERF_MEM_REGION_MEM1: __u64 = 0x09;
pub const PERF_MEM_REGION_MEM2: __u64 = 0x0a;
pub const PERF_MEM_REGION_MEM3: __u64 = 0x0b;
pub const PERF_MEM_REGION_MEM4: __u64 = 0x0c;
pub const PERF_MEM_REGION_MEM5: __u64 = 0x0d;
pub const PERF_MEM_REGION_MEM6: __u64 = 0x0e;
pub const PERF_MEM_REGION_MEM7: __u64 = 0x0f;
pub const PERF_MEM_REGION_SHIFT: __u32 = 46;

pub const fn PERF_MEM_S(value: __u64, shift: __u32) -> __u64 {
    value << shift
}

/*
 * Layout of single taken branch records:
 *
 *      from: source instruction (may not always be a branch insn)
 *        to: branch target
 *   mispred: branch target was mispredicted
 * predicted: branch target was predicted
 *
 * support for mispred, predicted is optional. In case it
 * is not supported mispred = predicted = 0.
 *
 *     in_tx: running in a hardware transaction
 *     abort: aborting a hardware transaction
 *    cycles: cycles from last branch (or 0 if not supported)
 *      type: branch type
 *      spec: branch speculation info (or 0 if not supported)
 */
#[repr(C)]
#[derive(Copy, Clone)]
pub struct perf_branch_entry {
    pub from: __u64,
    pub to: __u64,
    /*
     * C bitfield storage:
     * mispred:1, predicted:1, in_tx:1, abort:1, cycles:16, type:4,
     * spec:2, new_type:4, priv:3, reserved:31.
     */
    pub flags: __u64,
}

/* Size of used info bits in struct perf_branch_entry */
pub const PERF_BRANCH_ENTRY_INFO_BITS_MAX: __u32 = 33;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct perf_sample_weight_words {
    /*
     * The C header declares var1_dw/var2_w/var3_w in different order for
     * __LITTLE_ENDIAN_BITFIELD and __BIG_ENDIAN_BITFIELD.
     */
    #[cfg(target_endian = "little")]
    pub var1_dw: __u32,
    #[cfg(target_endian = "little")]
    pub var2_w: __u16,
    #[cfg(target_endian = "little")]
    pub var3_w: __u16,
    #[cfg(target_endian = "big")]
    pub var3_w: __u16,
    #[cfg(target_endian = "big")]
    pub var2_w: __u16,
    #[cfg(target_endian = "big")]
    pub var1_dw: __u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub union perf_sample_weight {
    pub full: __u64,
    pub words: perf_sample_weight_words,
}
