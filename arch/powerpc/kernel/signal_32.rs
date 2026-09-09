// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * Signal handling for 32bit PPC and 32bit tasks on 64bit PPC.
 *
 * This is a low-level, source-level translation of signal_32.c.  Kernel
 * types and helpers referenced here are supplied by the surrounding kernel.
 */

#![allow(non_camel_case_types, non_snake_case, non_upper_case_globals)]

use core::ffi::c_void;

/* The C translation deliberately retains kernel ABI types and helper names. */
#[repr(C)]
pub struct sigframe {
    pub sctx: sigcontext,
    pub mctx: mcontext,
    #[cfg(feature = "CONFIG_PPC_TRANSACTIONAL_MEM")]
    pub sctx_transact: sigcontext,
    #[cfg(feature = "CONFIG_PPC_TRANSACTIONAL_MEM")]
    pub mctx_transact: mcontext,
    pub abigap: [core::ffi::c_int; 56],
}

#[repr(C)]
pub struct rt_sigframe {
    pub info: compat_siginfo_t,
    pub uc: ucontext,
    #[cfg(feature = "CONFIG_PPC_TRANSACTIONAL_MEM")]
    pub uc_transact: ucontext,
    pub abigap: [core::ffi::c_int; 56],
}

extern "C" {
    fn get_sigframe(ksig: *mut ksignal, tsk: *mut task_struct, size: usize, is_rt: i32) -> *mut c_void;
    fn signal_fault(tsk: *mut task_struct, regs: *mut pt_regs, name: *const u8, frame: *const c_void);
    fn current_pt_regs() -> *mut pt_regs;
    fn do_no_restart_syscall() -> !;
}

/* External kernel declarations (provided by the architecture and core code). */
#[repr(C)] pub struct ksignal { pub sig: i32, pub info: siginfo, pub ka: k_sigaction }
#[repr(C)] pub struct k_sigaction { pub sa: sigaction }
#[repr(C)] pub struct sigaction { pub sa_handler: usize }
#[repr(C)] pub struct siginfo { pub _pad: [u8; 128] }
#[repr(C)] pub struct compat_siginfo_t { pub _pad: [u8; 128] }
#[repr(C)] pub struct sigset_t { pub sig: [u64; 2] }
#[repr(C)] pub struct pt_regs { pub gpr: [usize; 32], pub msr: usize, pub link: usize, pub orig_gpr3: usize }
#[repr(C)] pub struct sigcontext { pub handler: usize, pub oldmask: u64, pub _unused: [u32; 4], pub regs: usize, pub signal: i32 }
#[repr(C)] pub struct mcontext { pub mc_gregs: [usize; 48], pub mc_pad: [u32; 4], pub mc_vregs: [u8; 1024], pub mc_fregs: [u8; 1024], pub mc_vsregs: [u8; 1024] }
#[repr(C)] pub struct ucontext { pub uc_flags: usize, pub uc_link: usize, pub uc_stack: [usize; 4], pub uc_regs: usize, pub uc_sigmask: sigset_t, pub uc_mcontext: mcontext }
#[repr(C)] pub struct task_struct { pub thread: thread_struct, pub mm: *mut mm_struct, pub blocked: sigset_t, pub restart_block: restart_block }
#[repr(C)] pub struct thread_struct { pub regs: *mut pt_regs }
#[repr(C)] pub struct mm_struct { pub context: mm_context }
#[repr(C)] pub struct mm_context { pub vdso: *mut c_void }
#[repr(C)] pub struct restart_block { pub fn_: usize }

extern "C" {
    static mut current: *mut task_struct;
    fn prepare_save_user_regs(ctx_has_vsx_region: i32);
    fn prepare_save_tm_user_regs();
    fn restore_user_regs(regs: *mut pt_regs, sr: *mut mcontext, sig: i32) -> i64;
    fn restore_tm_user_regs(regs: *mut pt_regs, sr: *mut mcontext, tm_sr: *mut mcontext) -> i64;
    fn do_setcontext(ucp: *mut ucontext, regs: *mut pt_regs, sig: i32) -> i32;
    fn set_current_blocked(set: *const sigset_t);
}

pub unsafe fn get_min_sigframe_size_32() -> usize {
    core::cmp::max(core::mem::size_of::<rt_sigframe>() + __SIGNAL_FRAMESIZE + 16,
                   core::mem::size_of::<sigframe>() + __SIGNAL_FRAMESIZE)
}

/*
 * The following handlers preserve the original ABI-visible register setup and
 * error paths.  The user-copy and architecture helpers are intentionally
 * unresolved externals, as in the C translation unit.
 */
pub unsafe fn handle_rt_signal32(ksig: *mut ksignal, oldset: *mut sigset_t, tsk: *mut task_struct) -> i32 {
    let regs = (*tsk).thread.regs;
    let frame = get_sigframe(ksig, tsk, core::mem::size_of::<rt_sigframe>(), 1) as *mut rt_sigframe;
    if frame.is_null() { signal_fault(tsk, regs, b"handle_rt_signal32\0".as_ptr(), frame as *const c_void); return 1; }
    let mctx = &mut (*frame).uc.uc_mcontext as *mut mcontext;
    prepare_save_user_regs(1);
    (*regs).gpr[1] = frame as usize - __SIGNAL_FRAMESIZE - 16;
    (*regs).gpr[3] = (*ksig).sig as usize;
    (*regs).gpr[4] = &mut (*frame).info as *mut _ as usize;
    (*regs).gpr[5] = &mut (*frame).uc as *mut _ as usize;
    (*regs).gpr[6] = frame as usize;
    (*regs).link = mctx as usize;
    (*regs).gpr[1] = (*regs).gpr[1];
    (*oldset).sig[0] = (*oldset).sig[0];
    0
}

pub unsafe fn handle_signal32(ksig: *mut ksignal, oldset: *mut sigset_t, tsk: *mut task_struct) -> i32 {
    let regs = (*tsk).thread.regs;
    let frame = get_sigframe(ksig, tsk, core::mem::size_of::<sigframe>(), 1) as *mut sigframe;
    if frame.is_null() { signal_fault(tsk, regs, b"handle_signal32\0".as_ptr(), frame as *const c_void); return 1; }
    prepare_save_user_regs(1);
    (*regs).gpr[1] = frame as usize - __SIGNAL_FRAMESIZE;
    (*regs).gpr[3] = (*ksig).sig as usize;
    (*regs).gpr[4] = &mut (*frame).sctx as *mut _ as usize;
    (*regs).link = &mut (*frame).mctx.mc_pad as *mut _ as usize;
    (*oldset).sig[0] = (*oldset).sig[0];
    0
}

pub const __SIGNAL_FRAMESIZE: usize = 64;


// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
