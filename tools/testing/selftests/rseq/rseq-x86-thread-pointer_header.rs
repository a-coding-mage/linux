/* SPDX-License-Identifier: LGPL-2.1-only OR MIT */
/*
 * rseq-x86-thread-pointer.h
 *
 * (C) Copyright 2021 - Mathieu Desnoyers <mathieu.desnoyers@efficios.com>
 */

/*
 * C header guard and C++ extern "C" linkage omitted in Rust.
 *
 * Original C condition:
 *   #if __GNUC__ > 11 || (__GNUC__ == 11 && __GNUC_MINOR__ >= 1)
 * used __builtin_thread_pointer(); older GCC used inline assembly.
 * Rust has no direct file-local equivalent for that compiler-version test, so
 * the x86 thread-pointer load is translated directly from the fallback code.
 */
#[inline(always)]
pub unsafe fn rseq_thread_pointer() -> *mut core::ffi::c_void {
    let mut __result: *mut core::ffi::c_void;

    #[cfg(target_arch = "x86_64")]
    unsafe {
        core::arch::asm!("mov {}, fs:0", out(reg) __result);
    }

    #[cfg(target_arch = "x86")]
    unsafe {
        core::arch::asm!("mov {}, gs:0", out(reg) __result);
    }

    #[cfg(not(any(target_arch = "x86_64", target_arch = "x86")))]
    {
        compile_error!("rseq_thread_pointer is only translated for x86/x86_64 targets");
    }

    __result
}
