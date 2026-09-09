/* SPDX-License-Identifier: GPL-2.0 */

/* The C header selects this declaration when CONFIG_MMU is enabled. */
#[cfg(CONFIG_MMU)]
pub static mut current: *mut task_struct = core::ptr::null_mut();

pub struct task_struct;

#[cfg(not(CONFIG_MMU))]
#[repr(C)]
pub struct thread_info {
    pub task: *mut task_struct,
}

#[cfg(not(CONFIG_MMU))]
unsafe extern "C" {
    pub fn current_thread_info() -> *mut thread_info;
}

#[cfg(not(CONFIG_MMU))]
#[inline(always)]
pub unsafe fn get_current() -> *mut task_struct {
    /* Equivalent to current_thread_info()->task. */
    (*current_thread_info()).task
}

/* In the non-MMU configuration, the C macro `current` expands to get_current(). */
#[cfg(not(CONFIG_MMU))]
#[inline(always)]
pub unsafe fn current() -> *mut task_struct {
    get_current()
}

/* The C declaration binds this variable to the m68k stack-pointer register. */
#[cfg(target_arch = "m68k")]
pub static mut current_stack_pointer: usize = 0;

#[cfg(not(target_arch = "m68k"))]
pub static mut current_stack_pointer: usize = 0;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
