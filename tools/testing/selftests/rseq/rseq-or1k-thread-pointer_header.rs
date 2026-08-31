/* SPDX-License-Identifier: LGPL-2.1-only OR MIT */

use core::arch::asm;
use core::ffi::c_void;

#[inline]
pub unsafe fn rseq_thread_pointer() -> *mut c_void {
    let __thread_register: *mut c_void;

    unsafe {
        asm!("l.or {0}, r10, r0", out(reg) __thread_register);
    }
    __thread_register
}
