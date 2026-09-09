/* SPDX-License-Identifier: GPL-2.0 */

// C dependencies: linux/linkage.h and linux/list.h.

use core::ffi::c_void;

pub struct pt_regs;
pub struct task_struct;

#[repr(C)]
pub struct undef_hook {
    pub node: list_head,
    pub instr_mask: u32,
    pub instr_val: u32,
    pub cpsr_mask: u32,
    pub cpsr_val: u32,
    pub fn_: Option<unsafe extern "C" fn(regs: *mut pt_regs, instr: u32) -> i32>,
}

extern "C" {
    pub fn register_undef_hook(hook: *mut undef_hook);
    pub fn unregister_undef_hook(hook: *mut undef_hook);
}

#[inline]
pub unsafe fn __in_irqentry_text(ptr: c_ulong) -> i32 {
    extern "C" {
        static __irqentry_text_start: u8;
        static __irqentry_text_end: u8;
    }

    if ptr >= (&__irqentry_text_start as *const u8 as c_ulong)
        && ptr < (&__irqentry_text_end as *const u8 as c_ulong)
    {
        1
    } else {
        0
    }
}

extern "C" {
    pub fn early_trap_init(arg: *mut c_void);
    pub fn dump_backtrace_entry(
        where_: c_ulong,
        from: c_ulong,
        frame: c_ulong,
        loglvl: *const i8,
    );
    pub fn ptrace_break(regs: *mut pt_regs);

    pub static mut vectors_page: *mut c_void;

    pub fn dump_backtrace_stm(stack: *mut u32, instruction: u32, loglvl: *const i8);
    pub fn do_undefinstr(regs: *mut pt_regs);
    pub fn handle_fiq_as_nmi(regs: *mut pt_regs);
    pub fn bad_mode(regs: *mut pt_regs, reason: i32);
    pub fn arm_syscall(no: i32, regs: *mut pt_regs) -> i32;
    pub fn baddataabort(code: i32, instr: c_ulong, regs: *mut pt_regs);
    pub fn __div0();
    pub fn handle_bad_stack(regs: *mut pt_regs);
}

// External dependency supplied by linux/list.h.
pub type list_head = crate::list_head;
pub type c_ulong = usize;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
