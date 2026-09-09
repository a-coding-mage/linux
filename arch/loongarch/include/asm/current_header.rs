/* SPDX-License-Identifier: GPL-2.0 */

// C header dependencies: <linux/compiler.h>, <asm/percpu.h>

#[repr(C)]
pub struct task_struct;

// DECLARE_PER_CPU(struct task_struct *, cpu_tasks);
// The per-CPU declaration is supplied by the target environment.

// register struct task_struct *current_thread_pointer __asm__("$tp");
// The register binding is target-specific and is preserved by this declaration.
extern "C" {
    pub static mut current_thread_pointer: *mut task_struct;
}

#[inline(always)]
pub unsafe fn get_current() -> *mut task_struct {
    current_thread_pointer
}

#[inline(always)]
pub unsafe fn current() -> *mut task_struct {
    get_current()
}

#[inline(always)]
pub unsafe fn set_current(task: *mut task_struct) {
    __this_cpu_write!(cpu_tasks, task);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
