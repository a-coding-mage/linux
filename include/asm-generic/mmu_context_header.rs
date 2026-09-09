/* SPDX-License-Identifier: GPL-2.0 */

/*
 * Generic hooks to implement no-op functionality.
 *
 * The C header guard and preprocessor conditionals are represented as
 * comments because their configuration is supplied by the surrounding build.
 */

#[repr(C)]
pub struct task_struct {
    _private: [u8; 0],
}

#[repr(C)]
pub struct mm_struct {
    _private: [u8; 0],
}

/*
 * enter_lazy_tlb - Called when "tsk" is about to enter lazy TLB mode.
 *
 * @mm:  the currently active mm context which is becoming lazy
 * @tsk: task which is entering lazy tlb
 *
 * tsk->mm will be NULL
 *
 * C conditional: defined only when enter_lazy_tlb is not supplied elsewhere.
 */
#[inline]
pub unsafe fn enter_lazy_tlb(_mm: *mut mm_struct, _tsk: *mut task_struct) {}

/**
 * init_new_context - Initialize context of a new mm_struct.
 * @tsk: task struct for the mm
 * @mm:  the new mm struct
 * @return: 0 on success, -errno on failure
 *
 * C conditional: defined only when init_new_context is not supplied elsewhere.
 */
#[inline]
pub unsafe fn init_new_context(_tsk: *mut task_struct, _mm: *mut mm_struct) -> i32 {
    0
}

/**
 * destroy_context - Undo init_new_context when the mm is going away
 * @mm: old mm struct
 *
 * C conditional: defined only when destroy_context is not supplied elsewhere.
 */
#[inline]
pub unsafe fn destroy_context(_mm: *mut mm_struct) {}

/**
 * activate_mm - called after exec switches the current task to a new mm, to switch to it
 * @prev_mm: previous mm of this task
 * @next_mm: new mm
 *
 * C conditional: defined only when activate_mm is not supplied elsewhere.
 */
#[inline]
pub unsafe fn activate_mm(prev_mm: *mut mm_struct, next_mm: *mut mm_struct) {
    switch_mm(prev_mm, next_mm, current);
}

/**
 * dectivate_mm - called when an mm is released after exit or exec switches away from it
 * @tsk: the task
 * @mm:  the old mm
 *
 * C conditional: defined only when deactivate_mm is not supplied elsewhere.
 */
#[inline]
pub unsafe fn deactivate_mm(_tsk: *mut task_struct, _mm: *mut mm_struct) {}

/* External dependency supplied by the surrounding translation unit. */
extern "C" {
    fn switch_mm(
        prev_mm: *mut mm_struct,
        next_mm: *mut mm_struct,
        current: *mut task_struct,
    );
}

/* `current` is supplied by the surrounding translation unit. */
extern "C" {
    static mut current: *mut task_struct;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
