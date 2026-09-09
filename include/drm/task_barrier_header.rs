/*
 * Copyright 2019 Advanced Micro Devices, Inc.
 *
 * Permission is hereby granted, free of charge, to any person obtaining a
 * copy of this software and associated documentation files (the "Software"),
 * to deal in the Software without restriction, including without limitation
 * the rights to use, copy, modify, merge, publish, distribute, sublicense,
 * and/or sell copies of the Software, and to permit persons to whom the
 * Software is furnished to do so, subject to the following conditions:
 *
 * The above copyright notice and this permission notice shall be included in
 * all copies or substantial portions of the Software.
 *
 * THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
 * IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
 * FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT.  IN NO EVENT SHALL
 * THE COPYRIGHT HOLDER(S) OR AUTHOR(S) BE LIABLE FOR ANY CLAIM, DAMAGES OR
 * OTHER LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE,
 * ARISING FROM, OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR
 * OTHER DEALINGS IN THE SOFTWARE.
 */

/* External Linux semaphore and atomic types/functions are supplied elsewhere. */

/*
 * Reusable 2 PHASE task barrier (rendez-vous point) implementation for N tasks.
 * Based on the Little book of semaphores - https://greenteapress.com/wp/semaphores/
 */

#[repr(C)]
pub struct task_barrier {
    pub n: ::std::os::raw::c_uint,
    pub count: atomic_t,
    pub enter_turnstile: semaphore,
    pub exit_turnstile: semaphore,
}

extern "C" {
    fn up(turnstile: *mut semaphore);
    fn down(turnstile: *mut semaphore);
    fn atomic_set(value: *mut atomic_t, i: ::std::os::raw::c_int);
    fn atomic_inc_return(value: *mut atomic_t) -> ::std::os::raw::c_int;
    fn atomic_dec_return(value: *mut atomic_t) -> ::std::os::raw::c_int;
    fn sema_init(sem: *mut semaphore, val: ::std::os::raw::c_int);
}

/*
 * Represents an instance of a task barrier.
 */
#[inline]
pub unsafe fn task_barrier_signal_turnstile(
    turnstile: *mut semaphore,
    n: ::std::os::raw::c_uint,
) {
    let mut i: ::std::os::raw::c_int = 0;
    while i < n as ::std::os::raw::c_int {
        up(turnstile);
        i += 1;
    }
}

#[inline]
pub unsafe fn task_barrier_init(tb: *mut task_barrier) {
    (*tb).n = 0;
    atomic_set(&mut (*tb).count, 0);
    sema_init(&mut (*tb).enter_turnstile, 0);
    sema_init(&mut (*tb).exit_turnstile, 0);
}

#[inline]
pub unsafe fn task_barrier_add_task(tb: *mut task_barrier) {
    (*tb).n = (*tb).n.wrapping_add(1);
}

#[inline]
pub unsafe fn task_barrier_rem_task(tb: *mut task_barrier) {
    (*tb).n = (*tb).n.wrapping_sub(1);
}

/*
 * Lines up all the threads BEFORE the critical point.
 *
 * When all thread passed this code the entry barrier is back to locked state.
 */
#[inline]
pub unsafe fn task_barrier_enter(tb: *mut task_barrier) {
    if atomic_inc_return(&mut (*tb).count) == (*tb).n as ::std::os::raw::c_int {
        task_barrier_signal_turnstile(&mut (*tb).enter_turnstile, (*tb).n);
    }

    down(&mut (*tb).enter_turnstile);
}

/*
 * Lines up all the threads AFTER the critical point.
 *
 * This function is used to avoid any one thread running ahead if the barrier is
 * used repeatedly .
 */
#[inline]
pub unsafe fn task_barrier_exit(tb: *mut task_barrier) {
    if atomic_dec_return(&mut (*tb).count) == 0 {
        task_barrier_signal_turnstile(&mut (*tb).exit_turnstile, (*tb).n);
    }

    down(&mut (*tb).exit_turnstile);
}

/* Convinieince function when nothing to be done in between entry and exit */
#[inline]
pub unsafe fn task_barrier_full(tb: *mut task_barrier) {
    task_barrier_enter(tb);
    task_barrier_exit(tb);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
