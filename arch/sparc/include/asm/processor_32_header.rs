/* SPDX-License-Identifier: GPL-2.0 */
/* include/asm/processor.h
 *
 * Copyright (C) 1994 David S. Miller (davem@caip.rutgers.edu)
 */

// Dependencies supplied by the surrounding kernel translation are intentionally
// referenced here rather than reimplemented.

/* Whee, this is STACK_TOP + PAGE_SIZE and the lowest kernel address too...
 * That one page is used to protect kernel from intruders, so that
 * we can make our access_ok test faster
 */
pub const TASK_SIZE: usize = PAGE_OFFSET;

// In the original header these declarations are enabled by __KERNEL__.
#[cfg(feature = "__KERNEL__")]
pub const STACK_TOP: usize = PAGE_OFFSET - PAGE_SIZE;
#[cfg(feature = "__KERNEL__")]
pub const STACK_TOP_MAX: usize = STACK_TOP;

pub struct task_struct;

#[cfg(feature = "__KERNEL__")]
#[repr(C)]
pub struct fpq {
    pub insn_addr: *mut c_ulong,
    pub insn: c_ulong,
}

/* The Sparc processor specific thread struct. */
#[repr(C, align(8))]
pub struct thread_struct {
    pub kregs: *mut pt_regs,
    pub _pad1: c_uint,

    /* Special child fork kpsr/kwim values. */
    pub fork_kpsr: c_ulong,
    pub fork_kwim: c_ulong,

    /* Floating point regs */
    pub float_regs: [c_ulong; 32],
    pub fsr: c_ulong,
    pub fpqdepth: c_ulong,
    pub fpqueue: [fpq; 16],
}

#[cfg(feature = "__KERNEL__")]
pub const INIT_THREAD: thread_struct = thread_struct {
    kregs: unsafe { (init_stack.add(THREAD_SIZE) as *mut pt_regs).sub(1) },
    _pad1: 0,
    fork_kpsr: 0,
    fork_kwim: 0,
    float_regs: [0; 32],
    fsr: 0,
    fpqdepth: 0,
    fpqueue: [const { fpq { insn_addr: core::ptr::null_mut(), insn: 0 } }; 16],
};

/* Do necessary setup to start up a newly executed thread. */
#[inline]
pub unsafe fn start_thread(regs: *mut pt_regs, pc: c_ulong, sp: c_ulong) {
    (*regs).psr = ((*regs).psr & PSR_CWP) | PSR_S;
    (*regs).pc = (pc & !3).wrapping_sub(4);
    (*regs).npc = (*regs).pc.wrapping_add(4);
    (*regs).y = 0;
    // The original uses SPARC inline assembly to clear the new register window.
    // Preserve that target-specific operation as an explicit dependency.
    sparc_clear_register_window(
        regs,
        sp.wrapping_sub(core::mem::size_of::<reg_window32>() as c_ulong),
        &mut 0,
        core::mem::offset_of!(pt_regs, u_regs),
    );
}

unsafe extern "C" {
    pub fn sparc_clear_register_window(
        regs: *mut pt_regs,
        window: c_ulong,
        zero: *mut c_ulong,
        u_regs_offset: usize,
    );
}

pub unsafe extern "C" fn __get_wchan(_: *mut task_struct) -> c_ulong;

#[inline]
pub unsafe fn task_pt_regs(tsk: *mut task_struct) -> *mut pt_regs {
    (*tsk).thread.kregs
}

#[inline]
pub unsafe fn KSTK_EIP(tsk: *mut task_struct) -> c_ulong {
    (*(*tsk).thread.kregs).pc
}

#[inline]
pub unsafe fn KSTK_ESP(tsk: *mut task_struct) -> c_ulong {
    (*(*tsk).thread.kregs).u_regs[UREG_FP]
}

#[cfg(feature = "__KERNEL__")]
unsafe extern "C" {
    pub static mut last_task_used_math: *mut task_struct;
    pub fn do_mathemu(regs: *mut pt_regs, fpt: *mut task_struct) -> c_int;
    pub static mut sparc_idle: Option<unsafe extern "C" fn()>;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
