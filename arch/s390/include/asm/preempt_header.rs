/* SPDX-License-Identifier: GPL-2.0 */

// Dependencies supplied by the surrounding kernel translation unit:
// asm/current.h, linux/thread_info.h, asm/atomic_ops.h, asm/cmpxchg.h,
// and asm/march.h.

pub const PREEMPT_NEED_RESCHED: usize = 0x8000_0000_0000_0000usize;
pub const PREEMPT_ENABLED: usize = 0usize + PREEMPT_NEED_RESCHED;

extern "C" {
    pub fn get_lowcore() -> *mut Lowcore;
    pub fn arch_try_cmpxchg(ptr: *mut usize, old: *mut usize, new: usize) -> bool;
    pub fn __atomic64_and(value: usize, ptr: *mut isize);
    pub fn __atomic64_or(value: usize, ptr: *mut isize);
    pub fn __atomic64_add(value: i32, ptr: *mut isize) -> i32;
    pub fn __atomic64_add_const_and_test(value: i32, ptr: *mut isize) -> bool;
    pub fn unlikely(value: bool) -> bool;
}

#[repr(C)]
pub struct LowcorePreempt {
    pub count: i32,
}

#[repr(C)]
pub struct Lowcore {
    pub preempt: LowcorePreempt,
    pub preempt_count: usize,
}

#[inline(always)]
pub unsafe fn preempt_count() -> i32 {
    // The original uses s390 inline assembly with ALTERNATIVE(MFEATURE_LOWCORE)
    // to perform READ_ONCE(get_lowcore()->preempt.count).
    core::ptr::read_volatile(&(*get_lowcore()).preempt.count)
}

#[inline(always)]
pub unsafe fn preempt_count_set(pc: usize) {
    let mut old = core::ptr::read_volatile(&(*get_lowcore()).preempt_count);
    loop {
        let new = (old & PREEMPT_NEED_RESCHED) | (pc & !PREEMPT_NEED_RESCHED);
        if arch_try_cmpxchg(&mut (*get_lowcore()).preempt_count, &mut old, new) {
            break;
        }
    }
}

#[inline(always)]
pub unsafe fn set_preempt_need_resched() {
    __atomic64_and(!PREEMPT_NEED_RESCHED, &mut (*get_lowcore()).preempt_count as *mut usize as *mut isize);
}

#[inline(always)]
pub unsafe fn clear_preempt_need_resched() {
    __atomic64_or(PREEMPT_NEED_RESCHED, &mut (*get_lowcore()).preempt_count as *mut usize as *mut isize);
}

#[inline(always)]
pub unsafe fn test_preempt_need_resched() -> bool {
    !(core::ptr::read_volatile(&(*get_lowcore()).preempt_count) & PREEMPT_NEED_RESCHED != 0)
}

#[inline(always)]
pub unsafe fn __preempt_count_add(val: i32) {
    // The short s390 AGSI inline-assembly path is retained as conditional intent.
    // CONFIG_PROFILE_ALL_BRANCHES and __builtin_constant_p(val) are C build-time
    // conditions and cannot be evaluated from this isolated header.
    __atomic64_add(val, &mut (*get_lowcore()).preempt_count as *mut usize as *mut isize);
}

#[inline(always)]
pub unsafe fn __preempt_count_sub(val: i32) {
    __preempt_count_add(-val);
}

#[inline(always)]
pub unsafe fn __preempt_count_dec_and_test() -> bool {
    // The s390 ALGS(I) flag-output path is architecture-specific; this is its
    // equivalent atomic fallback.
    __atomic64_add_const_and_test(-1, &mut (*get_lowcore()).preempt_count as *mut usize as *mut isize)
}

#[inline(always)]
pub unsafe fn should_resched(preempt_offset: i32) -> bool {
    unlikely(core::ptr::read_volatile(&(*get_lowcore()).preempt_count) == preempt_offset as usize)
}

#[inline(always)]
pub unsafe fn __preempt_count_add_return(val: i32) -> i32 {
    val + __atomic64_add(val, &mut (*get_lowcore()).preempt_count as *mut usize as *mut isize)
}

#[inline(always)]
pub unsafe fn __preempt_count_sub_return(val: i32) -> i32 {
    __preempt_count_add_return(-val)
}

#[inline(always)]
pub fn init_task_preempt_count<T>(_p: *mut T) {}

// Deferred to CPU bringup time.
#[inline(always)]
pub fn init_idle_preempt_count<T>(_p: *mut T, _cpu: i32) {}

// CONFIG_PREEMPTION declarations.
extern "C" {
    pub fn preempt_schedule();
    pub fn preempt_schedule_notrace();
}

// CONFIG_PREEMPT_DYNAMIC declarations and selection macros.
extern "C" {
    pub fn dynamic_preempt_schedule();
    pub fn dynamic_preempt_schedule_notrace();
}

#[inline(always)]
pub unsafe fn __preempt_schedule() {
    dynamic_preempt_schedule();
}

#[inline(always)]
pub unsafe fn __preempt_schedule_notrace() {
    dynamic_preempt_schedule_notrace();
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
