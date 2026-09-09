/* SPDX-License-Identifier: GPL-2.0 */

// Dependency supplied by the surrounding kernel translation:
// linux/thread_info.h

pub const PREEMPT_NEED_RESCHED: u64 = 1u64 << 32;
pub const PREEMPT_ENABLED: u64 = PREEMPT_NEED_RESCHED;

#[inline]
pub unsafe fn preempt_count() -> i32 {
    READ_ONCE((*current_thread_info()).preempt.count)
}

#[inline]
pub unsafe fn preempt_count_set(pc: u64) {
    // Preserve existing value of PREEMPT_NEED_RESCHED
    WRITE_ONCE((*current_thread_info()).preempt.count, pc);
}

#[macro_export]
macro_rules! init_task_preempt_count {
    ($p:expr) => {{
        task_thread_info!($p).preempt_count = FORK_PREEMPT_COUNT;
    }};
}

#[macro_export]
macro_rules! init_idle_preempt_count {
    ($p:expr, $cpu:expr) => {{
        task_thread_info!($p).preempt_count = PREEMPT_DISABLED;
    }};
}

#[inline]
pub unsafe fn set_preempt_need_resched() {
    (*current_thread_info()).preempt.need_resched = 0;
}

#[inline]
pub unsafe fn clear_preempt_need_resched() {
    (*current_thread_info()).preempt.need_resched = 1;
}

#[inline]
pub unsafe fn test_preempt_need_resched() -> bool {
    !(*current_thread_info()).preempt.need_resched
}

#[inline]
pub unsafe fn __preempt_count_add(val: i32) {
    let mut pc: u32 = READ_ONCE((*current_thread_info()).preempt.count);
    pc = pc.wrapping_add(val as u32);
    WRITE_ONCE((*current_thread_info()).preempt.count, pc);
}

#[inline]
pub unsafe fn __preempt_count_sub(val: i32) {
    let mut pc: u32 = READ_ONCE((*current_thread_info()).preempt.count);
    pc = pc.wrapping_sub(val as u32);
    WRITE_ONCE((*current_thread_info()).preempt.count, pc);
}

#[inline]
pub unsafe fn __preempt_count_add_return(val: i32) -> i32 {
    let mut pc: u32 = READ_ONCE((*current_thread_info()).preempt.count);
    pc = pc.wrapping_add(val as u32);
    WRITE_ONCE((*current_thread_info()).preempt.count, pc);
    pc as i32
}

#[inline]
pub unsafe fn __preempt_count_sub_return(val: i32) -> i32 {
    let mut pc: u32 = READ_ONCE((*current_thread_info()).preempt.count);
    pc = pc.wrapping_sub(val as u32);
    WRITE_ONCE((*current_thread_info()).preempt.count, pc);
    pc as i32
}

#[inline]
pub unsafe fn __preempt_count_dec_and_test() -> bool {
    let ti = current_thread_info();
    let mut pc: u64 = READ_ONCE((*ti).preempt_count);

    // Update only the count field, leaving need_resched unchanged
    pc = pc.wrapping_sub(1);
    WRITE_ONCE((*ti).preempt.count, pc);

    /*
     * If we wrote back all zeroes, then we're preemptible and in
     * need of a reschedule. Otherwise, we need to reload the
     * preempt_count in case the need_resched flag was cleared by an
     * interrupt occurring between the non-atomic READ_ONCE/WRITE_ONCE
     * pair.
     */
    !pc || !READ_ONCE((*ti).preempt_count)
}

#[inline]
pub unsafe fn should_resched(preempt_offset: i32) -> bool {
    let pc: u64 = READ_ONCE((*current_thread_info()).preempt_count);
    pc == preempt_offset as u64
}

// CONFIG_PREEMPTION declarations are retained conditionally by the build.
#[cfg(CONFIG_PREEMPTION)]
extern "C" {
    pub fn preempt_schedule();
    pub fn preempt_schedule_notrace();
}

#[cfg(all(CONFIG_PREEMPTION, CONFIG_PREEMPT_DYNAMIC))]
extern "C" {
    pub fn dynamic_preempt_schedule();
    pub fn dynamic_preempt_schedule_notrace();
}

#[cfg(all(CONFIG_PREEMPTION, CONFIG_PREEMPT_DYNAMIC))]
#[inline]
pub unsafe fn __preempt_schedule() {
    dynamic_preempt_schedule();
}

#[cfg(all(CONFIG_PREEMPTION, CONFIG_PREEMPT_DYNAMIC))]
#[inline]
pub unsafe fn __preempt_schedule_notrace() {
    dynamic_preempt_schedule_notrace();
}

#[cfg(all(CONFIG_PREEMPTION, not(CONFIG_PREEMPT_DYNAMIC)))]
#[inline]
pub unsafe fn __preempt_schedule() {
    preempt_schedule();
}

#[cfg(all(CONFIG_PREEMPTION, not(CONFIG_PREEMPT_DYNAMIC)))]
#[inline]
pub unsafe fn __preempt_schedule_notrace() {
    preempt_schedule_notrace();
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
