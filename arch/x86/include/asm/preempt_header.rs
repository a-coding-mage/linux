/* SPDX-License-Identifier: GPL-2.0 */
// Rust translation of x86/include/asm/preempt.h.
// The included percpu, RMW, static-call, and configuration facilities are
// supplied by the surrounding kernel translation.

use core::ffi::c_void;

// DECLARE_PER_CPU_CACHE_HOT(unsigned long, __preempt_count);
extern "C" {
    static mut __preempt_count: usize;
}

/* We use the MSB for PREEMPT_NEED_RESCHED mostly because it is available. */
pub const PREEMPT_NEED_RESCHED: usize = !(usize::MAX >> 1);

#[cfg(feature = "CONFIG_HAS_SEPARATE_PREEMPT_RESCHED_BITS")]
const __PC_DEC: &str = "decq";
#[cfg(not(feature = "CONFIG_HAS_SEPARATE_PREEMPT_RESCHED_BITS"))]
const __PC_DEC: &str = "decl";

// The raw_cpu_* operations and per-CPU addressing are external kernel facilities.
extern "C" {
    fn raw_cpu_read_4(var: *mut usize) -> usize;
    fn raw_cpu_read_8(var: *mut usize) -> usize;
    fn raw_cpu_and_4(var: *mut usize, value: usize);
    fn raw_cpu_and_8(var: *mut usize, value: usize);
    fn raw_cpu_or_4(var: *mut usize, value: usize);
    fn raw_cpu_or_8(var: *mut usize, value: usize);
    fn raw_cpu_add_4(var: *mut usize, value: i32);
    fn raw_cpu_add_8(var: *mut usize, value: i32);
    fn raw_cpu_add_return_4(var: *mut usize, value: i32) -> i32;
    fn raw_cpu_add_return_8(var: *mut usize, value: i32) -> i32;
    fn raw_cpu_try_cmpxchg_4(var: *mut usize, old: *mut usize, new: usize) -> bool;
    fn raw_cpu_try_cmpxchg_8(var: *mut usize, old: *mut usize, new: usize) -> bool;
    fn gen_unary_rmwcc_dec(var: *mut usize) -> bool;
    fn unlikely(value: bool) -> bool;
    fn preempt_schedule();
    fn preempt_schedule_thunk();
    fn preempt_schedule_notrace();
    fn preempt_schedule_notrace_thunk();
}

#[inline(always)]
pub unsafe fn preempt_count() -> i32 {
    (raw_cpu_read_4(&mut __preempt_count) & !PREEMPT_NEED_RESCHED) as i32
}

#[inline(always)]
pub unsafe fn preempt_count_set(pc: usize) {
    let mut old = raw_cpu_read_4(&mut __preempt_count);
    loop {
        let new = (old & PREEMPT_NEED_RESCHED) | (pc & !PREEMPT_NEED_RESCHED);
        if raw_cpu_try_cmpxchg_4(&mut __preempt_count, &mut old, new) { break; }
    }
}

// #define init_task_preempt_count(p) do { } while (0)
#[inline(always)] pub fn init_task_preempt_count<T>(_p: *mut T) {}

// PREEMPT_DISABLED is supplied by the surrounding scheduler translation.
#[inline(always)]
pub unsafe fn init_idle_preempt_count<T>(_p: *mut T, _cpu: usize) {
    // per_cpu(__preempt_count, cpu) = PREEMPT_DISABLED;
}

#[inline(always)] pub unsafe fn set_preempt_need_resched() { raw_cpu_and_4(&mut __preempt_count, !PREEMPT_NEED_RESCHED); }
#[inline(always)] pub unsafe fn clear_preempt_need_resched() { raw_cpu_or_4(&mut __preempt_count, PREEMPT_NEED_RESCHED); }
#[inline(always)] pub unsafe fn test_preempt_need_resched() -> bool { (raw_cpu_read_4(&mut __preempt_count) & PREEMPT_NEED_RESCHED) == 0 }

#[inline(always)] pub unsafe fn __preempt_count_add(val: i32) { raw_cpu_add_4(&mut __preempt_count, val); }
#[inline(always)] pub unsafe fn __preempt_count_sub(val: i32) { raw_cpu_add_4(&mut __preempt_count, -val); }
#[inline(always)] pub unsafe fn __preempt_count_add_return(val: i32) -> i32 { raw_cpu_add_return_4(&mut __preempt_count, val) }
#[inline(always)] pub unsafe fn __preempt_count_sub_return(val: i32) -> i32 { raw_cpu_add_return_4(&mut __preempt_count, -val) }
#[inline(always)] pub unsafe fn __preempt_count_dec_and_test() -> bool { gen_unary_rmwcc_dec(&mut __preempt_count) }

#[inline(always)]
pub unsafe fn should_resched(preempt_offset: i32) -> bool {
    unlikely(preempt_count() == preempt_offset)
}

// CONFIG_PREEMPTION declarations and CONFIG_PREEMPT_DYNAMIC static-call/asm
// macros are preserved by the external schedule symbols above; their exact
// dispatch is a build-time kernel configuration concern.

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
