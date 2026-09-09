/* SPDX-License-Identifier: GPL-2.0 */

/* Dependencies supplied by the surrounding kernel translation. */

#[cfg(feature = "CONFIG_UNWIND_USER")]
pub const UNWIND_PENDING_BIT: u32 = 0;
#[cfg(feature = "CONFIG_UNWIND_USER")]
pub const UNWIND_USED_BIT: u32 = 1;

#[cfg(feature = "CONFIG_UNWIND_USER")]
pub const UNWIND_PENDING: usize = 1usize << UNWIND_PENDING_BIT;

#[cfg(feature = "CONFIG_UNWIND_USER")]
/* Set if the unwinding was used (directly or deferred) */
pub const UNWIND_USED: usize = 1usize << UNWIND_USED_BIT;

#[cfg(feature = "CONFIG_UNWIND_USER")]
extern "C" {
    pub fn unwind_task_init(task: *mut task_struct);
    pub fn unwind_task_free(task: *mut task_struct);

    pub fn unwind_user_faultable(trace: *mut unwind_stacktrace) -> i32;

    pub fn unwind_deferred_init(work: *mut unwind_work, func: unwind_callback_t) -> i32;
    pub fn unwind_deferred_request(work: *mut unwind_work, cookie: *mut u64) -> i32;
    pub fn unwind_deferred_cancel(work: *mut unwind_work);

    pub fn unwind_deferred_task_exit(task: *mut task_struct);
}

#[cfg(feature = "CONFIG_UNWIND_USER")]
#[inline(always)]
pub unsafe fn unwind_reset_info() {
    let info: *mut unwind_task_info = &mut (*current).unwind_info;
    let mut bits: usize = (*info).unwind_mask.load(std::sync::atomic::Ordering::Relaxed);

    /* Was there any unwinding? */
    if bits == 0 {
        return;
    }

    loop {
        /* Is a task_work going to run again before going back */
        if bits & UNWIND_PENDING != 0 {
            return;
        }
        match (*info).unwind_mask.compare_exchange_weak(
            bits,
            0usize,
            std::sync::atomic::Ordering::SeqCst,
            std::sync::atomic::Ordering::SeqCst,
        ) {
            Ok(_) => break,
            Err(new_bits) => bits = new_bits,
        }
    }
    (*current).unwind_info.id.id = 0;

    if !(*info).cache.is_null() {
        (*(*info).cache).nr_entries = 0;
        (*(*info).cache).unwind_completed = 0;
    }
}

#[cfg(not(feature = "CONFIG_UNWIND_USER"))]
#[inline]
pub unsafe fn unwind_task_init(_task: *mut task_struct) {}

#[cfg(not(feature = "CONFIG_UNWIND_USER"))]
#[inline]
pub unsafe fn unwind_task_free(_task: *mut task_struct) {}

#[cfg(not(feature = "CONFIG_UNWIND_USER"))]
#[inline]
pub unsafe fn unwind_user_faultable(_trace: *mut unwind_stacktrace) -> i32 {
    -38
}

#[cfg(not(feature = "CONFIG_UNWIND_USER"))]
#[inline]
pub unsafe fn unwind_deferred_init(
    _work: *mut unwind_work,
    _func: unwind_callback_t,
) -> i32 {
    -38
}

#[cfg(not(feature = "CONFIG_UNWIND_USER"))]
#[inline]
pub unsafe fn unwind_deferred_request(_work: *mut unwind_work, _timestamp: *mut u64) -> i32 {
    -38
}

#[cfg(not(feature = "CONFIG_UNWIND_USER"))]
#[inline]
pub unsafe fn unwind_deferred_cancel(_work: *mut unwind_work) {}

#[cfg(not(feature = "CONFIG_UNWIND_USER"))]
#[inline]
pub unsafe fn unwind_deferred_task_exit(_task: *mut task_struct) {}

#[cfg(not(feature = "CONFIG_UNWIND_USER"))]
#[inline]
pub unsafe fn unwind_reset_info() {}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
