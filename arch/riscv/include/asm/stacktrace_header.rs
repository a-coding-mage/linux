/* SPDX-License-Identifier: GPL-2.0 */

// Translated from the RISC-V stacktrace header.

use core::ffi::c_void;

#[repr(C)]
pub struct stackframe {
    pub fp: usize,
    pub ra: usize,
}

// Types supplied by the corresponding scheduler and ptrace dependencies.
pub enum task_struct {}
pub enum pt_regs {}

extern "C" {
    pub fn walk_stackframe(
        task: *mut task_struct,
        regs: *mut pt_regs,
        func: Option<unsafe extern "C" fn(*mut c_void, usize) -> bool>,
        arg: *mut c_void,
    );
    pub fn dump_backtrace(
        regs: *mut pt_regs,
        task: *mut task_struct,
        loglvl: *const core::ffi::c_char,
    );
}

// Equivalent to the C inline helper. The current task's stack base,
// current stack pointer, and THREAD_SIZE are supplied by the surrounding
// architecture/runtime definitions.
#[inline]
pub unsafe fn on_thread_stack(
    current_stack: usize,
    current_stack_pointer: usize,
    thread_size: usize,
) -> bool {
    !((current_stack ^ current_stack_pointer) & !(thread_size.wrapping_sub(1)) != 0)
}

// CONFIG_VMAP_STACK conditionally declares this per-CPU overflow stack.
// The declaration is provided by the surrounding per-CPU configuration.


// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
