/* SPDX-License-Identifier: LGPL-2.1-only OR MIT */
/*
 * rseq-ppc-thread-pointer.h
 *
 * (C) Copyright 2021 - Mathieu Desnoyers <mathieu.desnoyers@efficios.com>
 */

// C header guard and extern "C" linkage block omitted in Rust translation.

#[inline]
pub unsafe fn rseq_thread_pointer() -> *mut core::ffi::c_void {
    let __result: *mut core::ffi::c_void;

    #[cfg(target_arch = "powerpc64")]
    {
        core::arch::asm!("", out("r13") __result);
    }

    #[cfg(not(target_arch = "powerpc64"))]
    {
        core::arch::asm!("", out("r2") __result);
    }

    __result
}
