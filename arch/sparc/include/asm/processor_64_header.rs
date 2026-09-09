/* SPDX-License-Identifier: GPL-2.0 */
/*
 * include/asm/processor.h
 *
 * Copyright (C) 1996 David S. Miller (davem@caip.rutgers.edu)
 */

// Translated from the C header.  The original include dependencies and
// build-time assembler/kernel conditions are supplied by the surrounding
// translation unit.

pub const VA_BITS: usize = 44;
pub const VPTE_SIZE: usize = 1usize << (VA_BITS - PAGE_SHIFT + 3);

#[inline]
pub unsafe fn task_size_of<T>(tsk: *mut T) -> usize {
    if test_tsk_thread_flag(tsk, TIF_32BIT) {
        1usize << 32
    } else {
        (-(VPTE_SIZE as isize)) as usize
    }
}

#[inline]
pub unsafe fn task_size() -> usize {
    if test_thread_flag(TIF_32BIT) {
        1usize << 32
    } else {
        (-(VPTE_SIZE as isize)) as usize
    }
}

pub const STACK_TOP32: usize = (1usize << 32) - PAGE_SIZE;
pub const STACK_TOP64: usize = 0x0000_0800_0000_0000usize - (1usize << 32);

#[inline]
pub unsafe fn stack_top() -> usize {
    if test_thread_flag(TIF_32BIT) { STACK_TOP32 } else { STACK_TOP64 }
}

pub const STACK_TOP_MAX: usize = STACK_TOP64;

/* The Sparc processor specific thread struct. */
/* XXX This should die, everything can go into thread_info now. */
#[repr(C)]
pub struct thread_struct {
    #[cfg(CONFIG_DEBUG_SPINLOCK)]
    /* How many spinlocks held by this thread. */
    pub smp_lock_count: i32,
    #[cfg(CONFIG_DEBUG_SPINLOCK)]
    pub smp_lock_pc: u32,
    #[cfg(not(CONFIG_DEBUG_SPINLOCK))]
    pub dummy: i32, /* f'in gcc bug... */
}

/* INIT_THREAD is { 0 } without CONFIG_DEBUG_SPINLOCK, and { 0, 0 } with it. */
#[cfg(not(CONFIG_DEBUG_SPINLOCK))]
pub const INIT_THREAD: thread_struct = thread_struct { dummy: 0 };
#[cfg(CONFIG_DEBUG_SPINLOCK)]
pub const INIT_THREAD: thread_struct = thread_struct { smp_lock_count: 0, smp_lock_pc: 0 };

/* On Uniprocessor, even in RMO processes see TSO semantics. */
#[cfg(CONFIG_SMP)]
pub const TSTATE_INITIAL_MM: usize = TSTATE_TSO;
#[cfg(not(CONFIG_SMP))]
pub const TSTATE_INITIAL_MM: usize = TSTATE_RMO;

/* Do necessary setup to start up a newly executed thread. */
#[macro_export]
macro_rules! start_thread {
    ($regs:expr, $pc:expr, $sp:expr) => {{
        let mut __asi: usize = ASI_PNF;
        $regs.tstate = ($regs.tstate & TSTATE_CWP) | (TSTATE_INITIAL_MM | TSTATE_IE) | (__asi << 24);
        $regs.tpc = (($pc & !3usize).wrapping_sub(4));
        $regs.tnpc = $regs.tpc.wrapping_add(4);
        $regs.y = 0;
        set_thread_wstate(1 << 3);
        if !current_thread_info().utraps.is_null() {
            if *current_thread_info().utraps < 2 { kfree(current_thread_info().utraps); }
            else { *current_thread_info().utraps -= 1; }
            current_thread_info().utraps = core::ptr::null_mut();
        }
        unsafe { core::arch::asm!("stx %g0, [{0} + {2} + 0x00]", in(reg) $regs, in(reg) ($sp - core::mem::size_of::<reg_window>() - STACK_BIAS), const 0); }
        fprs_write(0);
        current_thread_info().xfsr[0] = 0;
        current_thread_info().fpsaved[0] = 0;
        $regs.tstate &= !TSTATE_PEF;
    }};
}

#[macro_export]
macro_rules! start_thread32 {
    ($regs:expr, $pc:expr, $sp:expr) => {{
        let __asi: usize = ASI_PNF;
        let pc = $pc & 0x0000_0000_ffff_ffffusize;
        let sp = $sp & 0x0000_0000_ffff_ffffusize;
        $regs.tstate = ($regs.tstate & TSTATE_CWP) | (TSTATE_INITIAL_MM | TSTATE_IE | TSTATE_AM) | (__asi << 24);
        $regs.tpc = ((pc & !3usize).wrapping_sub(4));
        $regs.tnpc = $regs.tpc.wrapping_add(4);
        $regs.y = 0;
        set_thread_wstate(2 << 3);
        if !current_thread_info().utraps.is_null() {
            if *current_thread_info().utraps < 2 { kfree(current_thread_info().utraps); }
            else { *current_thread_info().utraps -= 1; }
            current_thread_info().utraps = core::ptr::null_mut();
        }
        unsafe { core::arch::asm!("stx %g0, [{0} + {2} + 0x00]", in(reg) $regs, in(reg) (sp - core::mem::size_of::<reg_window32>()), const 0); }
        fprs_write(0);
        current_thread_info().xfsr[0] = 0;
        current_thread_info().fpsaved[0] = 0;
        $regs.tstate &= !TSTATE_PEF;
    }};
}

extern "C" {
    pub fn __get_wchan(task: *mut task_struct) -> usize;
    pub fn do_mathemu(regs: *mut pt_regs, f: *mut fpustate, illegal_insn_trap: bool) -> i32;
}

/* Equivalent accessors for the original task_pt_regs, KSTK_EIP, and KSTK_ESP macros. */
#[inline] pub unsafe fn task_pt_regs<T>(tsk: *mut T) -> *mut pt_regs { task_thread_info(tsk).kregs }
#[inline] pub unsafe fn kstk_eip<T>(tsk: *mut T) -> usize { (*task_pt_regs(tsk)).tpc }
#[inline] pub unsafe fn kstk_esp<T>(tsk: *mut T) -> usize { (*task_pt_regs(tsk)).u_regs[UREG_FP] }

/* Prefetch support.  This is tuned for UltraSPARC-III and later. */
pub const ARCH_HAS_PREFETCH: bool = true;
pub const ARCH_HAS_PREFETCHW: bool = true;

#[inline]
pub unsafe fn prefetch(x: *const core::ffi::c_void) {
    core::arch::asm!("prefetch [{0}], #one_write", in(reg) x);
}

#[inline]
pub unsafe fn prefetchw(x: *const core::ffi::c_void) {
    core::arch::asm!("prefetch [{0}], #n_writes", in(reg) x);
}

pub const HAVE_ARCH_PICK_MMAP_LAYOUT: bool = true;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
