/* SPDX-License-Identifier: GPL-2.0 */

// Dependencies supplied by the surrounding kernel translation:
// linux/llist.h, asm/ptrace.h, and asm/sections.h.

#[repr(C)]
pub struct stackframe {
    /*
     * FP member should hold R7 when CONFIG_THUMB2_KERNEL is enabled
     * and R11 otherwise.
     */
    pub fp: ::core::ffi::c_ulong,
    pub sp: ::core::ffi::c_ulong,
    pub lr: ::core::ffi::c_ulong,
    pub pc: ::core::ffi::c_ulong,

    /* address of the LR value on the stack */
    pub lr_addr: *mut ::core::ffi::c_ulong,
    #[cfg(CONFIG_KRETPROBES)]
    pub kr_cur: *mut llist_node,
    #[cfg(CONFIG_KRETPROBES)]
    pub tsk: *mut task_struct,
    #[cfg(CONFIG_UNWINDER_FRAME_POINTER)]
    pub ex_frame: bool,
}

pub unsafe fn on_thread_stack() -> bool {
    let delta: ::core::ffi::c_ulong = current_stack_pointer ^ (current as *mut task_struct as ::core::ffi::c_ulong);

    delta < THREAD_SIZE
}

#[inline(always)]
pub unsafe fn arm_get_current_stackframe(regs: *mut pt_regs, frame: *mut stackframe) {
    (*frame).fp = frame_pointer(regs);
    (*frame).sp = (*regs).ARM_sp;
    (*frame).lr = (*regs).ARM_lr;
    (*frame).pc = (*regs).ARM_pc;
    #[cfg(CONFIG_KRETPROBES)]
    {
        (*frame).kr_cur = core::ptr::null_mut();
        (*frame).tsk = current;
    }
    #[cfg(CONFIG_UNWINDER_FRAME_POINTER)]
    {
        (*frame).ex_frame = in_entry_text((*frame).pc);
    }
}

extern "C" {
    pub fn unwind_frame(frame: *mut stackframe) -> ::core::ffi::c_int;
    pub fn walk_stackframe(
        frame: *mut stackframe,
        fn_: Option<unsafe extern "C" fn(*mut ::core::ffi::c_void, ::core::ffi::c_ulong) -> bool>,
        data: *mut ::core::ffi::c_void,
    );
    pub fn dump_mem(
        lvl: *const ::core::ffi::c_char,
        str_: *const ::core::ffi::c_char,
        bottom: ::core::ffi::c_ulong,
        top: ::core::ffi::c_ulong,
    );
    pub fn dump_backtrace(
        regs: *mut pt_regs,
        tsk: *mut task_struct,
        loglvl: *const ::core::ffi::c_char,
    );
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
