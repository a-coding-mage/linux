/* SPDX-License-Identifier: GPL-2.0 */

use core::ffi::c_int;

unsafe extern "C" {
    pub static mut preempt_count: c_int;

    pub fn uatomic_inc(v: *mut c_int);
    pub fn uatomic_dec(v: *mut c_int);
}

pub unsafe fn preempt_disable() {
    unsafe {
        uatomic_inc(&raw mut preempt_count);
    }
}

pub unsafe fn preempt_enable() {
    unsafe {
        uatomic_dec(&raw mut preempt_count);
    }
}

#[inline]
pub fn in_interrupt() -> c_int {
    0
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
