/* SPDX-License-Identifier: GPL-2.0 */
/* thread_info.h: sparc64 low-level thread information
 *
 * Copyright (C) 2002  David S. Miller (davem@redhat.com)
 */

/* C header guard and __KERNEL__ conditional retained conceptually. */

pub const NSWINS: usize = 7;

pub const TI_FLAG_BYTE_FAULT_CODE: usize = 0;
pub const TI_FLAG_FAULT_CODE_SHIFT: usize = 56;
pub const TI_FLAG_BYTE_WSTATE: usize = 1;
pub const TI_FLAG_WSTATE_SHIFT: usize = 48;
pub const TI_FLAG_BYTE_NOERROR: usize = 2;
pub const TI_FLAG_NOERROR_SHIFT: usize = 40;
pub const TI_FLAG_BYTE_FPDEPTH: usize = 3;
pub const TI_FLAG_FPDEPTH_SHIFT: usize = 32;
pub const TI_FLAG_BYTE_CWP: usize = 4;
pub const TI_FLAG_CWP_SHIFT: usize = 24;
pub const TI_FLAG_BYTE_WSAVED: usize = 5;
pub const TI_FLAG_WSAVED_SHIFT: usize = 16;

/* External types supplied by the surrounding translation unit. */
#[repr(C)]
pub struct thread_info {
    /* D$ line 1 */
    pub task: *mut task_struct,
    pub flags: ::core::ffi::c_ulong,
    pub fpsaved: [u8; 7],
    pub status: u8,
    pub ksp: ::core::ffi::c_ulong,

    /* D$ line 2 */
    pub fault_address: ::core::ffi::c_ulong,
    pub kregs: *mut pt_regs,
    pub preempt_count: ::core::ffi::c_int,
    pub new_child: u8,
    pub __pad: u8,
    pub cpu: u16,

    pub utraps: *mut ::core::ffi::c_ulong,
    pub reg_window: [reg_window; NSWINS],
    pub rwbuf_stkptrs: [::core::ffi::c_ulong; NSWINS],
    pub gsr: [::core::ffi::c_ulong; 7],
    pub xfsr: [::core::ffi::c_ulong; 7],
    pub kern_una_regs: *mut pt_regs,
    pub kern_una_insn: ::core::ffi::c_uint,
    #[repr(align(64))]
    pub fpregs: [::core::ffi::c_ulong; (7 * 256) / ::core::mem::size_of::<::core::ffi::c_ulong>()],
}

pub const TI_TASK: usize = 0x00000000;
pub const TI_FLAGS: usize = 0x00000008;
pub const TI_FAULT_CODE: usize = TI_FLAGS + TI_FLAG_BYTE_FAULT_CODE;
pub const TI_WSTATE: usize = TI_FLAGS + TI_FLAG_BYTE_WSTATE;
pub const TI_CWP: usize = TI_FLAGS + TI_FLAG_BYTE_CWP;
pub const TI_FPDEPTH: usize = TI_FLAGS + TI_FLAG_BYTE_FPDEPTH;
pub const TI_WSAVED: usize = TI_FLAGS + TI_FLAG_BYTE_WSAVED;
pub const TI_SYS_NOERROR: usize = TI_FLAGS + TI_FLAG_BYTE_NOERROR;
pub const TI_FPSAVED: usize = 0x00000010;
pub const TI_KSP: usize = 0x00000018;
pub const TI_FAULT_ADDR: usize = 0x00000020;
pub const TI_KREGS: usize = 0x00000028;
pub const TI_PRE_COUNT: usize = 0x00000030;
pub const TI_NEW_CHILD: usize = 0x00000034;
pub const TI_CPU: usize = 0x00000036;
pub const TI_UTRAPS: usize = 0x00000038;
pub const TI_REG_WINDOW: usize = 0x00000040;
pub const TI_RWIN_SPTRS: usize = 0x000003c0;
pub const TI_GSR: usize = 0x000003f8;
pub const TI_XFSR: usize = 0x00000430;
pub const TI_KUNA_REGS: usize = 0x00000468;
pub const TI_KUNA_INSN: usize = 0x00000470;
pub const TI_FPREGS: usize = 0x00000480;

pub const FAULT_CODE_WRITE: u8 = 0x01;
pub const FAULT_CODE_DTLB: u8 = 0x02;
pub const FAULT_CODE_ITLB: u8 = 0x04;
pub const FAULT_CODE_WINFIXUP: u8 = 0x08;
pub const FAULT_CODE_BLKCOMMIT: u8 = 0x10;
pub const FAULT_CODE_BAD_RA: u8 = 0x20;

/* PAGE_SHIFT is a build-time configuration supplied externally. */
#[cfg(PAGE_SHIFT_13)]
pub const THREAD_SIZE_ORDER: usize = 1;
#[cfg(not(PAGE_SHIFT_13))]
pub const THREAD_SIZE_ORDER: usize = 0;

pub const TIF_SYSCALL_TRACE: usize = 0;
pub const TIF_NOTIFY_RESUME: usize = 1;
pub const TIF_SIGPENDING: usize = 2;
pub const TIF_NEED_RESCHED: usize = 3;
pub const TIF_NOTIFY_SIGNAL: usize = 4;
pub const TIF_UNALIGNED: usize = 5;
pub const TIF_UPROBE: usize = 6;
pub const TIF_32BIT: usize = 7;
pub const TIF_NOHZ: usize = 8;
pub const TIF_SECCOMP: usize = 9;
pub const TIF_SYSCALL_AUDIT: usize = 10;
pub const TIF_SYSCALL_TRACEPOINT: usize = 11;
pub const TIF_MCDPER: usize = 12;
pub const TIF_MEMDIE: usize = 13;
pub const TIF_POLLING_NRFLAG: usize = 14;

pub const _TIF_SYSCALL_TRACE: usize = 1 << TIF_SYSCALL_TRACE;
pub const _TIF_NOTIFY_RESUME: usize = 1 << TIF_NOTIFY_RESUME;
pub const _TIF_SIGPENDING: usize = 1 << TIF_SIGPENDING;
pub const _TIF_NEED_RESCHED: usize = 1 << TIF_NEED_RESCHED;
pub const _TIF_NOTIFY_SIGNAL: usize = 1 << TIF_NOTIFY_SIGNAL;
pub const _TIF_UNALIGNED: usize = 1 << TIF_UNALIGNED;
pub const _TIF_UPROBE: usize = 1 << TIF_UPROBE;
pub const _TIF_32BIT: usize = 1 << TIF_32BIT;
pub const _TIF_NOHZ: usize = 1 << TIF_NOHZ;
pub const _TIF_SECCOMP: usize = 1 << TIF_SECCOMP;
pub const _TIF_SYSCALL_AUDIT: usize = 1 << TIF_SYSCALL_AUDIT;
pub const _TIF_SYSCALL_TRACEPOINT: usize = 1 << TIF_SYSCALL_TRACEPOINT;
pub const _TIF_POLLING_NRFLAG: usize = 1 << TIF_POLLING_NRFLAG;
pub const _TIF_DO_NOTIFY_RESUME_MASK: usize = _TIF_NOTIFY_RESUME | _TIF_SIGPENDING | _TIF_UPROBE | _TIF_NOTIFY_SIGNAL;
pub const _TIF_USER_WORK_MASK: usize = (0xffusize << TI_FLAG_WSAVED_SHIFT) | _TIF_DO_NOTIFY_RESUME_MASK | _TIF_NEED_RESCHED;

/* External declarations and macros retained as Rust interfaces. */
unsafe extern "C" {
    pub fn current_thread_info() -> *mut thread_info;
    pub fn test_thread_flag(flag: usize) -> bool;
}

#[inline]
pub unsafe fn is_32bit_task() -> bool { test_thread_flag(TIF_32BIT) }

#[inline]
pub const fn thread32_stack_is_64bit(sp: ::core::ffi::c_ulong) -> bool { (sp & 0x1) != 0 }

#[inline]
pub unsafe fn test_thread_64bit_stack(sp: ::core::ffi::c_ulong) -> bool {
    if test_thread_flag(TIF_32BIT) && !thread32_stack_is_64bit(sp) { false } else { true }
}

#[inline]
pub unsafe fn thread_flag_byte_ptr(ti: *mut thread_info) -> *mut u8 {
    (&mut (*ti).flags as *mut _ as *mut u8)
}

/* The following accessors preserve the C byte-offset operations. */
#[inline] pub unsafe fn get_thread_fault_code() -> u8 { *thread_flag_byte_ptr(current_thread_info()).add(TI_FLAG_BYTE_FAULT_CODE) }
#[inline] pub unsafe fn set_thread_fault_code(val: u8) { *thread_flag_byte_ptr(current_thread_info()).add(TI_FLAG_BYTE_FAULT_CODE) = val; }
#[inline] pub unsafe fn get_thread_wstate() -> u8 { *thread_flag_byte_ptr(current_thread_info()).add(TI_FLAG_BYTE_WSTATE) }
#[inline] pub unsafe fn set_thread_wstate(val: u8) { *thread_flag_byte_ptr(current_thread_info()).add(TI_FLAG_BYTE_WSTATE) = val; }
#[inline] pub unsafe fn get_thread_cwp() -> u8 { *thread_flag_byte_ptr(current_thread_info()).add(TI_FLAG_BYTE_CWP) }
#[inline] pub unsafe fn set_thread_cwp(val: u8) { *thread_flag_byte_ptr(current_thread_info()).add(TI_FLAG_BYTE_CWP) = val; }
#[inline] pub unsafe fn get_thread_noerror() -> u8 { *thread_flag_byte_ptr(current_thread_info()).add(TI_FLAG_BYTE_NOERROR) }
#[inline] pub unsafe fn set_thread_noerror(val: u8) { *thread_flag_byte_ptr(current_thread_info()).add(TI_FLAG_BYTE_NOERROR) = val; }
#[inline] pub unsafe fn get_thread_fpdepth() -> u8 { *thread_flag_byte_ptr(current_thread_info()).add(TI_FLAG_BYTE_FPDEPTH) }
#[inline] pub unsafe fn set_thread_fpdepth(val: u8) { *thread_flag_byte_ptr(current_thread_info()).add(TI_FLAG_BYTE_FPDEPTH) = val; }
#[inline] pub unsafe fn get_thread_wsaved() -> u8 { *thread_flag_byte_ptr(current_thread_info()).add(TI_FLAG_BYTE_WSAVED) }
#[inline] pub unsafe fn set_thread_wsaved(val: u8) { *thread_flag_byte_ptr(current_thread_info()).add(TI_FLAG_BYTE_WSAVED) = val; }

/* External types referenced by the C header. */
pub enum task_struct {}
pub enum pt_regs {}
pub enum reg_window {}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
