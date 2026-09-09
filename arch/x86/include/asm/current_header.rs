/* SPDX-License-Identifier: GPL-2.0 */

/*
 * The C header guard `_ASM_X86_CURRENT_H` is represented by Rust's module
 * item uniqueness.  The included Linux declarations are supplied by other
 * translated files.
 */

#[repr(C)]
pub struct task_struct {
    _private: [u8; 0],
}

/* DECLARE_PER_CPU_CACHE_HOT(struct task_struct *, current_task); */
extern "C" {
    pub static mut current_task: *mut task_struct;
}

/* const-qualified alias provided by the linker. */
/* DECLARE_PER_CPU_CACHE_HOT(struct task_struct * const __percpu_seg_override,
 *                           const_current_task); */
extern "C" {
    pub static const_current_task: *const task_struct;
}

extern "C" {
    pub fn this_cpu_read_const(value: *const task_struct) -> *mut task_struct;
    pub fn this_cpu_read_stable(value: *mut task_struct) -> *mut task_struct;
}

#[inline(always)]
pub unsafe fn get_current() -> *mut task_struct {
    /* IS_ENABLED(CONFIG_USE_X86_SEG_SUPPORT) */
    if cfg!(feature = "CONFIG_USE_X86_SEG_SUPPORT") {
        this_cpu_read_const(const_current_task)
    } else {
        this_cpu_read_stable(current_task)
    }
}

/* #define current get_current() */
#[inline(always)]
pub unsafe fn current() -> *mut task_struct {
    get_current()
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
