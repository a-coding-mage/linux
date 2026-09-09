// SPDX-License-Identifier: GPL-2.0
// Dependencies supplied by the surrounding kernel build:
// linux/sched/signal.h, linux/sched/task.h, linux/sched/task_stack.h,
// linux/slab.h, asm/processor.h, asm/fpu.h, asm/traps.h, asm/ptrace.h

use core::ffi::c_void;

#[repr(C)]
pub struct task_struct {
    pub thread: thread_struct,
}

#[repr(C)]
pub struct thread_struct {
    pub xstate: *mut xstate_struct,
    pub fpu_counter: i32,
}

#[repr(C)]
pub struct xstate_struct {
    pub hardfpu: sh_fpu_hard_struct,
    pub softfpu: sh_fpu_soft_struct,
}

#[repr(C)]
pub struct sh_fpu_hard_struct {
    pub fpscr: u32,
}

#[repr(C)]
pub struct sh_fpu_soft_struct {
    pub fpscr: u32,
}

#[repr(C)]
pub struct pt_regs {
    _private: [u8; 0],
}

#[repr(C)]
pub struct thread_info {
    pub status: usize,
}

#[repr(C)]
pub struct cpuinfo_sh {
    pub flags: u32,
}

extern "C" {
    static mut boot_cpu_data: cpuinfo_sh;
    static mut current: *mut task_struct;
    static mut task_xstate_cachep: *mut c_void;
    static mut xstate_size: usize;

    fn tsk_used_math(tsk: *mut task_struct) -> bool;
    fn unlazy_fpu(tsk: *mut task_struct, regs: *mut pt_regs);
    fn task_pt_regs(tsk: *mut task_struct) -> *mut pt_regs;
    fn kmem_cache_alloc(cachep: *mut c_void, flags: u32) -> *mut xstate_struct;
    fn memset(s: *mut c_void, c: i32, n: usize) -> *mut c_void;
    fn set_stopped_child_used_math(tsk: *mut task_struct);
    fn restore_fpu(tsk: *mut task_struct);
    fn task_thread_info(tsk: *mut task_struct) -> *mut thread_info;
    fn user_mode(regs: *mut pt_regs) -> bool;
    fn printk(level: *const u8, fmt: *const u8);
    fn bug();
    fn local_irq_enable();
    fn local_irq_disable();
    fn force_sig(sig: i32);
    fn grab_fpu(regs: *mut pt_regs);
}

const CPU_HAS_FPU: u32 = 1 << 0;
const FPSCR_INIT: u32 = 0;
const GFP_KERNEL: u32 = 0;
const ENOMEM: i32 = 12;
const SIGKILL: i32 = 9;
const TS_USEDFPU: usize = 1;

#[inline]
pub unsafe fn init_fpu(tsk: *mut task_struct) -> i32 {
    if tsk_used_math(tsk) {
        if (boot_cpu_data.flags & CPU_HAS_FPU) != 0 && tsk == current {
            unlazy_fpu(tsk, task_pt_regs(tsk));
        }
        return 0;
    }

    /*
     * Memory allocation at the first usage of the FPU and other state.
     */
    if (*tsk).thread.xstate.is_null() {
        (*tsk).thread.xstate = kmem_cache_alloc(task_xstate_cachep, GFP_KERNEL);
        if (*tsk).thread.xstate.is_null() {
            return -ENOMEM;
        }
    }

    if (boot_cpu_data.flags & CPU_HAS_FPU != 0) {
        let fp: *mut sh_fpu_hard_struct = &mut (*(*tsk).thread.xstate).hardfpu;
        memset(fp.cast(), 0, xstate_size);
        (*fp).fpscr = FPSCR_INIT;
    } else {
        let fp: *mut sh_fpu_soft_struct = &mut (*(*tsk).thread.xstate).softfpu;
        memset(fp.cast(), 0, xstate_size);
        (*fp).fpscr = FPSCR_INIT;
    }

    set_stopped_child_used_math(tsk);
    0
}

#[cfg(CONFIG_SH_FPU)]
pub unsafe fn __fpu_state_restore() {
    let tsk: *mut task_struct = current;

    restore_fpu(tsk);

    (*task_thread_info(tsk)).status |= TS_USEDFPU;
    (*tsk).thread.fpu_counter += 1;
}

#[cfg(CONFIG_SH_FPU)]
pub unsafe fn fpu_state_restore(regs: *mut pt_regs) {
    let tsk: *mut task_struct = current;

    if !user_mode(regs) {
        printk(b"BUG: FPU is used in kernel mode.\0".as_ptr(), b"\0".as_ptr());
        bug();
        return;
    }

    if !tsk_used_math(tsk) {
        let ret: i32;
        /*
         * does a slab alloc which can sleep
         */
        local_irq_enable();
        ret = init_fpu(tsk);
        local_irq_disable();
        if ret != 0 {
            /*
             * ran out of memory!
             */
            force_sig(SIGKILL);
            return;
        }
    }

    grab_fpu(regs);

    __fpu_state_restore();
}

#[cfg(CONFIG_SH_FPU)]
pub unsafe fn fpu_state_restore_trap(regs: *mut pt_regs) {
    fpu_state_restore(regs);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
