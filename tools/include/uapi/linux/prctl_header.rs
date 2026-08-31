/* SPDX-License-Identifier: GPL-2.0 WITH Linux-syscall-note */

/* Depends on Linux UAPI integer types from <linux/types.h>. */

/* Values to pass as first argument to prctl() */

pub const PR_SET_PDEATHSIG: u32 = 1; /* Second arg is a signal */
pub const PR_GET_PDEATHSIG: u32 = 2; /* Second arg is a ptr to return the signal */

/* Get/set current->mm->dumpable */
pub const PR_GET_DUMPABLE: u32 = 3;
pub const PR_SET_DUMPABLE: u32 = 4;

/* Get/set unaligned access control bits (if meaningful) */
pub const PR_GET_UNALIGN: u32 = 5;
pub const PR_SET_UNALIGN: u32 = 6;
pub const PR_UNALIGN_NOPRINT: u32 = 1; /* silently fix up unaligned user accesses */
pub const PR_UNALIGN_SIGBUS: u32 = 2; /* generate SIGBUS on unaligned user access */

/* Get/set whether or not to drop capabilities on setuid() away from
 * uid 0 (as per security/commoncap.c) */
pub const PR_GET_KEEPCAPS: u32 = 7;
pub const PR_SET_KEEPCAPS: u32 = 8;

/* Get/set floating-point emulation control bits (if meaningful) */
pub const PR_GET_FPEMU: u32 = 9;
pub const PR_SET_FPEMU: u32 = 10;
pub const PR_FPEMU_NOPRINT: u32 = 1; /* silently emulate fp operations accesses */
pub const PR_FPEMU_SIGFPE: u32 = 2; /* don't emulate fp operations, send SIGFPE instead */

/* Get/set floating-point exception mode (if meaningful) */
pub const PR_GET_FPEXC: u32 = 11;
pub const PR_SET_FPEXC: u32 = 12;
pub const PR_FP_EXC_SW_ENABLE: u32 = 0x80; /* Use FPEXC for FP exception enables */
pub const PR_FP_EXC_DIV: u32 = 0x010000; /* floating point divide by zero */
pub const PR_FP_EXC_OVF: u32 = 0x020000; /* floating point overflow */
pub const PR_FP_EXC_UND: u32 = 0x040000; /* floating point underflow */
pub const PR_FP_EXC_RES: u32 = 0x080000; /* floating point inexact result */
pub const PR_FP_EXC_INV: u32 = 0x100000; /* floating point invalid operation */
pub const PR_FP_EXC_DISABLED: u32 = 0; /* FP exceptions disabled */
pub const PR_FP_EXC_NONRECOV: u32 = 1; /* async non-recoverable exc. mode */
pub const PR_FP_EXC_ASYNC: u32 = 2; /* async recoverable exception mode */
pub const PR_FP_EXC_PRECISE: u32 = 3; /* precise exception mode */

/* Get/set whether we use statistical process timing or accurate timestamp
 * based process timing */
pub const PR_GET_TIMING: u32 = 13;
pub const PR_SET_TIMING: u32 = 14;
pub const PR_TIMING_STATISTICAL: u32 = 0; /* Normal, traditional,
                                                   statistical process timing */
pub const PR_TIMING_TIMESTAMP: u32 = 1; /* Accurate timestamp based
                                                   process timing */

pub const PR_SET_NAME: u32 = 15; /* Set process name */
pub const PR_GET_NAME: u32 = 16; /* Get process name */

/* Get/set process endian */
pub const PR_GET_ENDIAN: u32 = 19;
pub const PR_SET_ENDIAN: u32 = 20;
pub const PR_ENDIAN_BIG: u32 = 0;
pub const PR_ENDIAN_LITTLE: u32 = 1; /* True little endian mode */
pub const PR_ENDIAN_PPC_LITTLE: u32 = 2; /* "PowerPC" pseudo little endian */

/* Get/set process seccomp mode */
pub const PR_GET_SECCOMP: u32 = 21;
pub const PR_SET_SECCOMP: u32 = 22;

/* Get/set the capability bounding set (as per security/commoncap.c) */
pub const PR_CAPBSET_READ: u32 = 23;
pub const PR_CAPBSET_DROP: u32 = 24;

/* Get/set the process' ability to use the timestamp counter instruction */
pub const PR_GET_TSC: u32 = 25;
pub const PR_SET_TSC: u32 = 26;
pub const PR_TSC_ENABLE: u32 = 1; /* allow the use of the timestamp counter */
pub const PR_TSC_SIGSEGV: u32 = 2; /* throw a SIGSEGV instead of reading the TSC */

/* Get/set securebits (as per security/commoncap.c) */
pub const PR_GET_SECUREBITS: u32 = 27;
pub const PR_SET_SECUREBITS: u32 = 28;

/*
 * Get/set the timerslack as used by poll/select/nanosleep
 * A value of 0 means "use default"
 */
pub const PR_SET_TIMERSLACK: u32 = 29;
pub const PR_GET_TIMERSLACK: u32 = 30;

pub const PR_TASK_PERF_EVENTS_DISABLE: u32 = 31;
pub const PR_TASK_PERF_EVENTS_ENABLE: u32 = 32;

/*
 * Set early/late kill mode for hwpoison memory corruption.
 * This influences when the process gets killed on a memory corruption.
 */
pub const PR_MCE_KILL: u32 = 33;
pub const PR_MCE_KILL_CLEAR: u32 = 0;
pub const PR_MCE_KILL_SET: u32 = 1;

pub const PR_MCE_KILL_LATE: u32 = 0;
pub const PR_MCE_KILL_EARLY: u32 = 1;
pub const PR_MCE_KILL_DEFAULT: u32 = 2;

pub const PR_MCE_KILL_GET: u32 = 34;

/*
 * Tune up process memory map specifics.
 */
pub const PR_SET_MM: u32 = 35;
pub const PR_SET_MM_START_CODE: u32 = 1;
pub const PR_SET_MM_END_CODE: u32 = 2;
pub const PR_SET_MM_START_DATA: u32 = 3;
pub const PR_SET_MM_END_DATA: u32 = 4;
pub const PR_SET_MM_START_STACK: u32 = 5;
pub const PR_SET_MM_START_BRK: u32 = 6;
pub const PR_SET_MM_BRK: u32 = 7;
pub const PR_SET_MM_ARG_START: u32 = 8;
pub const PR_SET_MM_ARG_END: u32 = 9;
pub const PR_SET_MM_ENV_START: u32 = 10;
pub const PR_SET_MM_ENV_END: u32 = 11;
pub const PR_SET_MM_AUXV: u32 = 12;
pub const PR_SET_MM_EXE_FILE: u32 = 13;
pub const PR_SET_MM_MAP: u32 = 14;
pub const PR_SET_MM_MAP_SIZE: u32 = 15;

/*
 * This structure provides new memory descriptor
 * map which mostly modifies /proc/pid/stat[m]
 * output for a task. This mostly done in a
 * sake of checkpoint/restore functionality.
 */
#[repr(C)]
pub struct prctl_mm_map {
    pub start_code: __u64, /* code section bounds */
    pub end_code: __u64,
    pub start_data: __u64, /* data section bounds */
    pub end_data: __u64,
    pub start_brk: __u64, /* heap for brk() syscall */
    pub brk: __u64,
    pub start_stack: __u64, /* stack starts at */
    pub arg_start: __u64, /* command line arguments bounds */
    pub arg_end: __u64,
    pub env_start: __u64, /* environment variables bounds */
    pub env_end: __u64,
    pub auxv: *mut __u64, /* auxiliary vector */
    pub auxv_size: __u32, /* vector size */
    pub exe_fd: __u32, /* /proc/$pid/exe link file */
}

/*
 * Set specific pid that is allowed to ptrace the current task.
 * A value of 0 mean "no process".
 */
pub const PR_SET_PTRACER: u32 = 0x59616d61;
pub const PR_SET_PTRACER_ANY: usize = usize::MAX;

pub const PR_SET_CHILD_SUBREAPER: u32 = 36;
pub const PR_GET_CHILD_SUBREAPER: u32 = 37;

/*
 * If no_new_privs is set, then operations that grant new privileges (i.e.
 * execve) will either fail or not grant them.  This affects suid/sgid,
 * file capabilities, and LSMs.
 *
 * Operations that merely manipulate or drop existing privileges (setresuid,
 * capset, etc.) will still work.  Drop those privileges if you want them gone.
 *
 * Changing LSM security domain is considered a new privilege.  So, for example,
 * asking selinux for a specific new context (e.g. with runcon) will result
 * in execve returning -EPERM.
 *
 * See Documentation/userspace-api/no_new_privs.rst for more details.
 */
pub const PR_SET_NO_NEW_PRIVS: u32 = 38;
pub const PR_GET_NO_NEW_PRIVS: u32 = 39;

pub const PR_GET_TID_ADDRESS: u32 = 40;

pub const PR_SET_THP_DISABLE: u32 = 41;
pub const PR_GET_THP_DISABLE: u32 = 42;

/*
 * No longer implemented, but left here to ensure the numbers stay reserved:
 */
pub const PR_MPX_ENABLE_MANAGEMENT: u32 = 43;
pub const PR_MPX_DISABLE_MANAGEMENT: u32 = 44;

pub const PR_SET_FP_MODE: u32 = 45;
pub const PR_GET_FP_MODE: u32 = 46;
pub const PR_FP_MODE_FR: u32 = 1 << 0; /* 64b FP registers */
pub const PR_FP_MODE_FRE: u32 = 1 << 1; /* 32b compatibility */

/* Control the ambient capability set */
pub const PR_CAP_AMBIENT: u32 = 47;
pub const PR_CAP_AMBIENT_IS_SET: u32 = 1;
pub const PR_CAP_AMBIENT_RAISE: u32 = 2;
pub const PR_CAP_AMBIENT_LOWER: u32 = 3;
pub const PR_CAP_AMBIENT_CLEAR_ALL: u32 = 4;

/* arm64 Scalable Vector Extension controls */
/* Flag values must be kept in sync with ptrace NT_ARM_SVE interface */
pub const PR_SVE_SET_VL: u32 = 50; /* set task vector length */
pub const PR_SVE_SET_VL_ONEXEC: u32 = 1 << 18; /* defer effect until exec */
pub const PR_SVE_GET_VL: u32 = 51; /* get task vector length */
/* Bits common to PR_SVE_SET_VL and PR_SVE_GET_VL */
pub const PR_SVE_VL_LEN_MASK: u32 = 0xffff;
pub const PR_SVE_VL_INHERIT: u32 = 1 << 17; /* inherit across exec */

/* Per task speculation control */
pub const PR_GET_SPECULATION_CTRL: u32 = 52;
pub const PR_SET_SPECULATION_CTRL: u32 = 53;
/* Speculation control variants */
pub const PR_SPEC_STORE_BYPASS: u32 = 0;
pub const PR_SPEC_INDIRECT_BRANCH: u32 = 1;
pub const PR_SPEC_L1D_FLUSH: u32 = 2;
/* Return and control values for PR_SET/GET_SPECULATION_CTRL */
pub const PR_SPEC_NOT_AFFECTED: usize = 0;
pub const PR_SPEC_PRCTL: usize = 1usize << 0;
pub const PR_SPEC_ENABLE: usize = 1usize << 1;
pub const PR_SPEC_DISABLE: usize = 1usize << 2;
pub const PR_SPEC_FORCE_DISABLE: usize = 1usize << 3;
pub const PR_SPEC_DISABLE_NOEXEC: usize = 1usize << 4;

/* Reset arm64 pointer authentication keys */
pub const PR_PAC_RESET_KEYS: u32 = 54;
pub const PR_PAC_APIAKEY: usize = 1usize << 0;
pub const PR_PAC_APIBKEY: usize = 1usize << 1;
pub const PR_PAC_APDAKEY: usize = 1usize << 2;
pub const PR_PAC_APDBKEY: usize = 1usize << 3;
pub const PR_PAC_APGAKEY: usize = 1usize << 4;

/* Tagged user address controls for arm64 and RISC-V */
pub const PR_SET_TAGGED_ADDR_CTRL: u32 = 55;
pub const PR_GET_TAGGED_ADDR_CTRL: u32 = 56;
pub const PR_TAGGED_ADDR_ENABLE: usize = 1usize << 0;
/* MTE tag check fault modes */
pub const PR_MTE_TCF_NONE: usize = 0usize;
pub const PR_MTE_TCF_SYNC: usize = 1usize << 1;
pub const PR_MTE_TCF_ASYNC: usize = 1usize << 2;
pub const PR_MTE_TCF_MASK: usize = PR_MTE_TCF_SYNC | PR_MTE_TCF_ASYNC;
/* MTE tag inclusion mask */
pub const PR_MTE_TAG_SHIFT: usize = 3;
pub const PR_MTE_TAG_MASK: usize = 0xffffusize << PR_MTE_TAG_SHIFT;
/* Unused; kept only for source compatibility */
pub const PR_MTE_TCF_SHIFT: usize = 1;
/* RISC-V pointer masking tag length */
pub const PR_PMLEN_SHIFT: usize = 24;
pub const PR_PMLEN_MASK: usize = 0x7fusize << PR_PMLEN_SHIFT;

/* Control reclaim behavior when allocating memory */
pub const PR_SET_IO_FLUSHER: u32 = 57;
pub const PR_GET_IO_FLUSHER: u32 = 58;

/* Dispatch syscalls to a userspace handler */
pub const PR_SET_SYSCALL_USER_DISPATCH: u32 = 59;
pub const PR_SYS_DISPATCH_OFF: u32 = 0;
/* Enable dispatch except for the specified range */
pub const PR_SYS_DISPATCH_EXCLUSIVE_ON: u32 = 1;
/* Enable dispatch for the specified range */
pub const PR_SYS_DISPATCH_INCLUSIVE_ON: u32 = 2;
/* Legacy name for backwards compatibility */
pub const PR_SYS_DISPATCH_ON: u32 = PR_SYS_DISPATCH_EXCLUSIVE_ON;
/* The control values for the user space selector when dispatch is enabled */
pub const SYSCALL_DISPATCH_FILTER_ALLOW: u32 = 0;
pub const SYSCALL_DISPATCH_FILTER_BLOCK: u32 = 1;

/* Set/get enabled arm64 pointer authentication keys */
pub const PR_PAC_SET_ENABLED_KEYS: u32 = 60;
pub const PR_PAC_GET_ENABLED_KEYS: u32 = 61;

/* Request the scheduler to share a core */
pub const PR_SCHED_CORE: u32 = 62;
pub const PR_SCHED_CORE_GET: u32 = 0;
pub const PR_SCHED_CORE_CREATE: u32 = 1; /* create unique core_sched cookie */
pub const PR_SCHED_CORE_SHARE_TO: u32 = 2; /* push core_sched cookie to pid */
pub const PR_SCHED_CORE_SHARE_FROM: u32 = 3; /* pull core_sched cookie to pid */
pub const PR_SCHED_CORE_MAX: u32 = 4;
pub const PR_SCHED_CORE_SCOPE_THREAD: u32 = 0;
pub const PR_SCHED_CORE_SCOPE_THREAD_GROUP: u32 = 1;
pub const PR_SCHED_CORE_SCOPE_PROCESS_GROUP: u32 = 2;

/* arm64 Scalable Matrix Extension controls */
/* Flag values must be in sync with SVE versions */
pub const PR_SME_SET_VL: u32 = 63; /* set task vector length */
pub const PR_SME_SET_VL_ONEXEC: u32 = 1 << 18; /* defer effect until exec */
pub const PR_SME_GET_VL: u32 = 64; /* get task vector length */
/* Bits common to PR_SME_SET_VL and PR_SME_GET_VL */
pub const PR_SME_VL_LEN_MASK: u32 = 0xffff;
pub const PR_SME_VL_INHERIT: u32 = 1 << 17; /* inherit across exec */

/* Memory deny write / execute */
pub const PR_SET_MDWE: u32 = 65;
pub const PR_MDWE_REFUSE_EXEC_GAIN: usize = 1usize << 0;
pub const PR_MDWE_NO_INHERIT: usize = 1usize << 1;

pub const PR_GET_MDWE: u32 = 66;

pub const PR_SET_VMA: u32 = 0x53564d41;
pub const PR_SET_VMA_ANON_NAME: u32 = 0;

pub const PR_GET_AUXV: u32 = 0x41555856;

pub const PR_SET_MEMORY_MERGE: u32 = 67;
pub const PR_GET_MEMORY_MERGE: u32 = 68;

pub const PR_RISCV_V_SET_CONTROL: u32 = 69;
pub const PR_RISCV_V_GET_CONTROL: u32 = 70;
pub const PR_RISCV_V_VSTATE_CTRL_DEFAULT: u32 = 0;
pub const PR_RISCV_V_VSTATE_CTRL_OFF: u32 = 1;
pub const PR_RISCV_V_VSTATE_CTRL_ON: u32 = 2;
pub const PR_RISCV_V_VSTATE_CTRL_INHERIT: u32 = 1 << 4;
pub const PR_RISCV_V_VSTATE_CTRL_CUR_MASK: u32 = 0x3;
pub const PR_RISCV_V_VSTATE_CTRL_NEXT_MASK: u32 = 0xc;
pub const PR_RISCV_V_VSTATE_CTRL_MASK: u32 = 0x1f;

pub const PR_RISCV_SET_ICACHE_FLUSH_CTX: u32 = 71;
pub const PR_RISCV_CTX_SW_FENCEI_ON: u32 = 0;
pub const PR_RISCV_CTX_SW_FENCEI_OFF: u32 = 1;
pub const PR_RISCV_SCOPE_PER_PROCESS: u32 = 0;
pub const PR_RISCV_SCOPE_PER_THREAD: u32 = 1;

/* PowerPC Dynamic Execution Control Register (DEXCR) controls */
pub const PR_PPC_GET_DEXCR: u32 = 72;
pub const PR_PPC_SET_DEXCR: u32 = 73;
/* DEXCR aspect to act on */
pub const PR_PPC_DEXCR_SBHE: u32 = 0; /* Speculative branch hint enable */
pub const PR_PPC_DEXCR_IBRTPD: u32 = 1; /* Indirect branch recurrent target prediction disable */
pub const PR_PPC_DEXCR_SRAPD: u32 = 2; /* Subroutine return address prediction disable */
pub const PR_PPC_DEXCR_NPHIE: u32 = 3; /* Non-privileged hash instruction enable */
/* Action to apply / return */
pub const PR_PPC_DEXCR_CTRL_EDITABLE: u32 = 0x1; /* Aspect can be modified with PR_PPC_SET_DEXCR */
pub const PR_PPC_DEXCR_CTRL_SET: u32 = 0x2; /* Set the aspect for this process */
pub const PR_PPC_DEXCR_CTRL_CLEAR: u32 = 0x4; /* Clear the aspect for this process */
pub const PR_PPC_DEXCR_CTRL_SET_ONEXEC: u32 = 0x8; /* Set the aspect on exec */
pub const PR_PPC_DEXCR_CTRL_CLEAR_ONEXEC: u32 = 0x10; /* Clear the aspect on exec */
pub const PR_PPC_DEXCR_CTRL_MASK: u32 = 0x1f;

/*
 * Get the current shadow stack configuration for the current thread,
 * this will be the value configured via PR_SET_SHADOW_STACK_STATUS.
 */
pub const PR_GET_SHADOW_STACK_STATUS: u32 = 74;

/*
 * Set the current shadow stack configuration.  Enabling the shadow
 * stack will cause a shadow stack to be allocated for the thread.
 */
pub const PR_SET_SHADOW_STACK_STATUS: u32 = 75;
pub const PR_SHADOW_STACK_ENABLE: usize = 1usize << 0;
pub const PR_SHADOW_STACK_WRITE: usize = 1usize << 1;
pub const PR_SHADOW_STACK_PUSH: usize = 1usize << 2;

/*
 * Prevent further changes to the specified shadow stack
 * configuration.  All bits may be locked via this call, including
 * undefined bits.
 */
pub const PR_LOCK_SHADOW_STACK_STATUS: u32 = 76;

/*
 * Controls the mode of timer_create() for CRIU restore operations.
 * Enabling this allows CRIU to restore timers with explicit IDs.
 *
 * Don't use for normal operations as the result might be undefined.
 */
pub const PR_TIMER_CREATE_RESTORE_IDS: u32 = 77;
pub const PR_TIMER_CREATE_RESTORE_IDS_OFF: u32 = 0;
pub const PR_TIMER_CREATE_RESTORE_IDS_ON: u32 = 1;
pub const PR_TIMER_CREATE_RESTORE_IDS_GET: u32 = 2;

/* FUTEX hash management */
pub const PR_FUTEX_HASH: u32 = 78;
pub const PR_FUTEX_HASH_SET_SLOTS: u32 = 1;
pub const PR_FUTEX_HASH_GET_SLOTS: u32 = 2;
