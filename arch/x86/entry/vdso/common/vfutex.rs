// SPDX-License-Identifier: GPL-2.0-only

/*
 * Assembly template for the try unlock functions. The basic functionality is:
 *
 *		mov		esi, %eax	Move the TID into EAX
 *		xor		%ecx, %ecx	Clear ECX
 *		lock_cmpxchgl	%ecx, (%rdi)	Attempt the TID -> 0 transition
 * .Lcs_start:					Start of the critical section
 *		jnz		.Lcs_end	If cmpxchl failed jump to the end
 * .Lcs_success:				Start of the success section
 *		movq		%rcx, (%rdx)	Set the pending op pointer to 0
 * .Lcs_end:					End of the critical section
 *
 * .Lcs_start and .Lcs_end establish the critical section range. .Lcs_success is
 * technically not required, but there for illustration, debugging and testing.
 *
 * When CONFIG_COMPAT is enabled then the 64-bit VDSO provides two functions.
 * One for the regular 64-bit sized pending operation pointer and one for a
 * 32-bit sized pointer to support gaming emulators.
 *
 * The 32-bit VDSO provides only the one for 32-bit sized pointers.
 */

#[inline(always)]
unsafe fn futex_robust_try_unlock<P>(lock: *mut u32, tid: u32, pop: *mut P) -> u32 {
    // Corresponds to `lock cmpxchgl` with the TID in EAX and zero as the value.
    let lock_atomic = &*(lock as *const core::sync::atomic::AtomicU32);
    match lock_atomic.compare_exchange(
        tid,
        0,
        core::sync::atomic::Ordering::SeqCst,
        core::sync::atomic::Ordering::SeqCst,
    ) {
        Ok(previous) => {
            // Critical-section success path: clear the pending operation pointer.
            core::ptr::write_volatile(pop, unsafe { core::mem::zeroed() });
            previous
        }
        Err(previous) => previous,
    }
}

#[cfg(target_arch = "x86_64")]
pub unsafe extern "C" fn __vdso_futex_robust_list64_try_unlock(
    lock: *mut u32,
    tid: u32,
    pop: *mut u64,
) -> u32 {
    futex_robust_try_unlock(lock, tid, pop)
}

// CONFIG_X86_32 || CONFIG_COMPAT: preserved as a build-time condition.
#[cfg(any(target_arch = "x86", feature = "CONFIG_COMPAT"))]
pub unsafe extern "C" fn __vdso_futex_robust_list32_try_unlock(
    lock: *mut u32,
    tid: u32,
    pop: *mut u32,
) -> u32 {
    futex_robust_try_unlock(lock, tid, pop)
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
