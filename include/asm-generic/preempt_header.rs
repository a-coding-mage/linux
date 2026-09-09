/* SPDX-License-Identifier: GPL-2.0 */

// Dependency supplied by the surrounding kernel translation:
// #include <linux/thread_info.h>

pub const PREEMPT_ENABLED: i32 = 0;

#[inline(always)]
pub unsafe fn preempt_count() -> i32 {
    // READ_ONCE(current_thread_info()->preempt_count)
    core::ptr::read_volatile(&(*crate::current_thread_info()).preempt_count)
}

#[inline(always)]
pub unsafe fn preempt_count_ptr() -> *mut i32 {
    &mut (*crate::current_thread_info()).preempt_count
}

#[inline(always)]
pub unsafe fn preempt_count_set(pc: i32) {
    *preempt_count_ptr() = pc;
}

// Must be macros in C to avoid header recursion hell.
#[inline(always)]
pub unsafe fn init_task_preempt_count(p: *mut core::ffi::c_void) {
    (*crate::task_thread_info(p)).preempt_count = crate::FORK_PREEMPT_COUNT;
}

#[inline(always)]
pub unsafe fn init_idle_preempt_count(
    p: *mut core::ffi::c_void,
    _cpu: i32,
) {
    (*crate::task_thread_info(p)).preempt_count = crate::PREEMPT_DISABLED;
}

#[inline(always)]
pub fn set_preempt_need_resched() {}

#[inline(always)]
pub fn clear_preempt_need_resched() {}

#[inline(always)]
pub fn test_preempt_need_resched() -> bool {
    false
}

/* The various preempt_count add/sub methods. */

#[inline(always)]
pub unsafe fn __preempt_count_add(val: i32) {
    *preempt_count_ptr() += val;
}

#[inline(always)]
pub unsafe fn __preempt_count_sub(val: i32) {
    *preempt_count_ptr() -= val;
}

#[inline(always)]
pub unsafe fn __preempt_count_add_return(val: i32) -> i32 {
    *preempt_count_ptr() += val;
    *preempt_count_ptr()
}

#[inline(always)]
pub unsafe fn __preempt_count_sub_return(val: i32) -> i32 {
    *preempt_count_ptr() -= val;
    *preempt_count_ptr()
}

#[inline(always)]
pub unsafe fn __preempt_count_dec_and_test() -> bool {
    /*
     * Because load-store architectures cannot do per-cpu atomic
     * operations; we cannot use PREEMPT_NEED_RESCHED because it might get
     * lost.
     */
    *preempt_count_ptr() -= 1;
    *preempt_count_ptr() == 0 && crate::tif_need_resched()
}

/* Returns true when we need to resched and can (barring IRQ state). */
#[inline(always)]
pub unsafe fn should_resched(preempt_offset: i32) -> bool {
    crate::unlikely(preempt_count() == preempt_offset && crate::tif_need_resched())
}

// CONFIG_PREEMPTION declarations and macro mappings are conditional on the
// corresponding build-time configuration and are supplied by the build.
#[cfg(feature = "CONFIG_PREEMPTION")]
extern "C" {
    pub fn preempt_schedule();
    pub fn preempt_schedule_notrace();
}

#[cfg(all(feature = "CONFIG_PREEMPTION", feature = "CONFIG_PREEMPT_DYNAMIC", feature = "CONFIG_HAVE_PREEMPT_DYNAMIC_KEY"))]
extern "C" {
    pub fn dynamic_preempt_schedule();
    pub fn dynamic_preempt_schedule_notrace();
}

#[cfg(all(feature = "CONFIG_PREEMPTION", feature = "CONFIG_PREEMPT_DYNAMIC", feature = "CONFIG_HAVE_PREEMPT_DYNAMIC_KEY"))]
#[inline(always)]
pub unsafe fn __preempt_schedule() {
    dynamic_preempt_schedule();
}

#[cfg(all(feature = "CONFIG_PREEMPTION", feature = "CONFIG_PREEMPT_DYNAMIC", feature = "CONFIG_HAVE_PREEMPT_DYNAMIC_KEY"))]
#[inline(always)]
pub unsafe fn __preempt_schedule_notrace() {
    dynamic_preempt_schedule_notrace();
}

#[cfg(all(feature = "CONFIG_PREEMPTION", not(all(feature = "CONFIG_PREEMPT_DYNAMIC", feature = "CONFIG_HAVE_PREEMPT_DYNAMIC_KEY"))))]
#[inline(always)]
pub unsafe fn __preempt_schedule() {
    preempt_schedule();
}

#[cfg(all(feature = "CONFIG_PREEMPTION", not(all(feature = "CONFIG_PREEMPT_DYNAMIC", feature = "CONFIG_HAVE_PREEMPT_DYNAMIC_KEY"))))]
#[inline(always)]
pub unsafe fn __preempt_schedule_notrace() {
    preempt_schedule_notrace();
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
