/* SPDX-License-Identifier: GPL-2.0 WITH Linux-syscall-note */
/* ptrace.h */
/* structs and defines to help the user use the ptrace system call. */

/* has the defines to get at the registers. */

pub const PTRACE_TRACEME: u32 = 0;
pub const PTRACE_PEEKTEXT: u32 = 1;
pub const PTRACE_PEEKDATA: u32 = 2;
pub const PTRACE_PEEKUSR: u32 = 3;
pub const PTRACE_POKETEXT: u32 = 4;
pub const PTRACE_POKEDATA: u32 = 5;
pub const PTRACE_POKEUSR: u32 = 6;
pub const PTRACE_CONT: u32 = 7;
pub const PTRACE_KILL: u32 = 8;
pub const PTRACE_SINGLESTEP: u32 = 9;

pub const PTRACE_ATTACH: u32 = 16;
pub const PTRACE_DETACH: u32 = 17;
pub const PTRACE_SYSCALL: u32 = 24;

pub const PTRACE_SETOPTIONS: u32 = 0x4200;
pub const PTRACE_GETEVENTMSG: u32 = 0x4201;
pub const PTRACE_GETSIGINFO: u32 = 0x4202;
pub const PTRACE_SETSIGINFO: u32 = 0x4203;
pub const PTRACE_GETREGSET: u32 = 0x4204;
pub const PTRACE_SETREGSET: u32 = 0x4205;
pub const PTRACE_SEIZE: u32 = 0x4206;
pub const PTRACE_INTERRUPT: u32 = 0x4207;
pub const PTRACE_LISTEN: u32 = 0x4208;
pub const PTRACE_PEEKSIGINFO: u32 = 0x4209;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ptrace_peeksiginfo_args {
    pub off: u64, /* from which siginfo to start */
    pub flags: u32,
    pub nr: i32, /* how may siginfos to take */
}

pub const PTRACE_GETSIGMASK: u32 = 0x420a;
pub const PTRACE_SETSIGMASK: u32 = 0x420b;
pub const PTRACE_SECCOMP_GET_FILTER: u32 = 0x420c;
pub const PTRACE_SECCOMP_GET_METADATA: u32 = 0x420d;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct seccomp_metadata {
    pub filter_off: u64, /* Input: which filter */
    pub flags: u64, /* Output: filter's flags */
}

pub const PTRACE_GET_SYSCALL_INFO: u32 = 0x420e;
pub const PTRACE_SET_SYSCALL_INFO: u32 = 0x4212;
pub const PTRACE_SYSCALL_INFO_NONE: u32 = 0;
pub const PTRACE_SYSCALL_INFO_ENTRY: u32 = 1;
pub const PTRACE_SYSCALL_INFO_EXIT: u32 = 2;
pub const PTRACE_SYSCALL_INFO_SECCOMP: u32 = 3;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ptrace_syscall_info_entry {
    pub nr: u64,
    pub args: [u64; 6],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ptrace_syscall_info_exit {
    pub rval: i64,
    pub is_error: u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ptrace_syscall_info_seccomp {
    pub nr: u64,
    pub args: [u64; 6],
    pub ret_data: u32,
    pub reserved2: u32,
}

#[repr(C)]
pub union ptrace_syscall_info_union {
    pub entry: ptrace_syscall_info_entry,
    pub exit: ptrace_syscall_info_exit,
    pub seccomp: ptrace_syscall_info_seccomp,
}

#[repr(C)]
pub struct ptrace_syscall_info {
    pub op: u8, /* PTRACE_SYSCALL_INFO_* */
    pub reserved: u8,
    pub flags: u16,
    pub arch: u32,
    pub instruction_pointer: u64,
    pub stack_pointer: u64,
    pub __bindgen_anon_1: ptrace_syscall_info_union,
}

pub const PTRACE_GET_RSEQ_CONFIGURATION: u32 = 0x420f;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ptrace_rseq_configuration {
    pub rseq_abi_pointer: u64,
    pub rseq_abi_size: u32,
    pub signature: u32,
    pub flags: u32,
    pub pad: u32,
}

pub const PTRACE_SET_SYSCALL_USER_DISPATCH_CONFIG: u32 = 0x4210;
pub const PTRACE_GET_SYSCALL_USER_DISPATCH_CONFIG: u32 = 0x4211;

/*
 * struct ptrace_sud_config - Per-task configuration for Syscall User Dispatch
 * @mode: One of PR_SYS_DISPATCH_ON or PR_SYS_DISPATCH_OFF
 * @selector: Tracees user virtual address of SUD selector
 * @offset: SUD exclusion area (virtual address)
 * @len: Length of SUD exclusion area
 */
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ptrace_sud_config {
    pub mode: u64,
    pub selector: u64,
    pub offset: u64,
    pub len: u64,
}

/* 0x4212 is PTRACE_SET_SYSCALL_INFO */

/* These values are stored in task->ptrace_message by ptrace_stop. */
pub const PTRACE_EVENTMSG_SYSCALL_ENTRY: u32 = 1;
pub const PTRACE_EVENTMSG_SYSCALL_EXIT: u32 = 2;

pub const PTRACE_PEEKSIGINFO_SHARED: u32 = 1 << 0;
pub const PTRACE_EVENT_FORK: u32 = 1;
pub const PTRACE_EVENT_VFORK: u32 = 2;
pub const PTRACE_EVENT_CLONE: u32 = 3;
pub const PTRACE_EVENT_EXEC: u32 = 4;
pub const PTRACE_EVENT_VFORK_DONE: u32 = 5;
pub const PTRACE_EVENT_EXIT: u32 = 6;
pub const PTRACE_EVENT_SECCOMP: u32 = 7;
pub const PTRACE_EVENT_STOP: u32 = 128;

pub const PTRACE_O_TRACESYSGOOD: u32 = 1;
pub const PTRACE_O_TRACEFORK: u32 = 1 << PTRACE_EVENT_FORK;
pub const PTRACE_O_TRACEVFORK: u32 = 1 << PTRACE_EVENT_VFORK;
pub const PTRACE_O_TRACECLONE: u32 = 1 << PTRACE_EVENT_CLONE;
pub const PTRACE_O_TRACEEXEC: u32 = 1 << PTRACE_EVENT_EXEC;
pub const PTRACE_O_TRACEVFORKDONE: u32 = 1 << PTRACE_EVENT_VFORK_DONE;
pub const PTRACE_O_TRACEEXIT: u32 = 1 << PTRACE_EVENT_EXIT;
pub const PTRACE_O_TRACESECCOMP: u32 = 1 << PTRACE_EVENT_SECCOMP;
pub const PTRACE_O_EXITKILL: u32 = 1 << 20;
pub const PTRACE_O_SUSPEND_SECCOMP: u32 = 1 << 21;
pub const PTRACE_O_MASK: u32 = 0x000000ff | PTRACE_O_EXITKILL | PTRACE_O_SUSPEND_SECCOMP;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
