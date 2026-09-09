/* SPDX-License-Identifier: GPL-2.0 */

// Dependencies supplied by the surrounding PowerPC kernel translation:
// asm/bug.h, asm/book3s/32/mmu-hash.h, asm/mmu.h, asm/synch.h, linux/sched.h.
// The following items are intentionally referenced from those dependencies.

#[cfg(feature = "CONFIG_PPC_KUAP")]
pub const KUAP_NONE: usize = usize::MAX;

#[cfg(feature = "CONFIG_PPC_KUAP")]
#[inline(always)]
pub unsafe fn kuap_lock_one(addr: usize) {
    mtsr(mfsr(addr) | SR_KS, addr);
    isync(); // Context sync required after mtsr()
}

#[cfg(feature = "CONFIG_PPC_KUAP")]
#[inline(always)]
pub unsafe fn kuap_unlock_one(addr: usize) {
    mtsr(mfsr(addr) & !SR_KS, addr);
    isync(); // Context sync required after mtsr()
}

#[cfg(feature = "CONFIG_PPC_KUAP")]
#[inline(always)]
pub unsafe fn uaccess_begin_32s(addr: usize) {
    // ASM_MMU_FTR_IFSET("mfsrin; rlwinm; mtsrin; isync", "", MMU_FTR_KUAP)
    // is emitted by the target PowerPC backend here.
    let _ = addr;
}

#[cfg(feature = "CONFIG_PPC_KUAP")]
#[inline(always)]
pub unsafe fn uaccess_end_32s(addr: usize) {
    // ASM_MMU_FTR_IFSET("mfsrin; oris; mtsrin; isync", "", MMU_FTR_KUAP)
    // is emitted by the target PowerPC backend here.
    let _ = addr;
}

#[cfg(feature = "CONFIG_PPC_KUAP")]
#[inline(always)]
pub unsafe fn __kuap_save_and_lock(regs: *mut pt_regs) {
    let kuap = (*current).thread.kuap;

    (*regs).kuap = kuap;
    if unlikely(kuap == KUAP_NONE) {
        return;
    }

    (*current).thread.kuap = KUAP_NONE;
    kuap_lock_one(kuap);
}

#[cfg(feature = "CONFIG_PPC_KUAP")]
#[inline(always)]
pub unsafe fn kuap_user_restore(_regs: *mut pt_regs) {}

#[cfg(feature = "CONFIG_PPC_KUAP")]
#[inline(always)]
pub unsafe fn __kuap_kernel_restore(regs: *mut pt_regs, kuap: usize) {
    if unlikely(kuap != KUAP_NONE) {
        (*current).thread.kuap = KUAP_NONE;
        kuap_lock_one(kuap);
    }

    if likely((*regs).kuap == KUAP_NONE) {
        return;
    }

    (*current).thread.kuap = (*regs).kuap;
    kuap_unlock_one((*regs).kuap);
}

#[cfg(feature = "CONFIG_PPC_KUAP")]
#[inline(always)]
pub unsafe fn __kuap_get_and_assert_locked() -> usize {
    let kuap = (*current).thread.kuap;
    WARN_ON_ONCE(IS_ENABLED_CONFIG_PPC_KUAP_DEBUG && kuap != KUAP_NONE);
    kuap
}

#[cfg(feature = "CONFIG_PPC_KUAP")]
#[inline(always)]
pub unsafe fn allow_user_access(to: *mut core::ffi::c_void, dir: usize) {
    BUILD_BUG_ON(!__builtin_constant_p(dir));

    if (dir & KUAP_WRITE) == 0 {
        return;
    }

    (*current).thread.kuap = to as u32 as usize;
    uaccess_begin_32s(to as u32 as usize);
}

#[cfg(feature = "CONFIG_PPC_KUAP")]
#[inline(always)]
pub unsafe fn prevent_user_access(dir: usize) {
    let kuap = (*current).thread.kuap;

    BUILD_BUG_ON(!__builtin_constant_p(dir));
    if (dir & KUAP_WRITE) == 0 {
        return;
    }

    (*current).thread.kuap = KUAP_NONE;
    uaccess_end_32s(kuap);
}

#[cfg(feature = "CONFIG_PPC_KUAP")]
#[inline(always)]
pub unsafe fn prevent_user_access_return() -> usize {
    let flags = (*current).thread.kuap;

    if flags != KUAP_NONE {
        (*current).thread.kuap = KUAP_NONE;
        uaccess_end_32s(flags);
    }
    flags
}

#[cfg(feature = "CONFIG_PPC_KUAP")]
#[inline(always)]
pub unsafe fn restore_user_access(flags: usize) {
    if flags != KUAP_NONE {
        (*current).thread.kuap = flags;
        uaccess_begin_32s(flags);
    }
}

#[cfg(feature = "CONFIG_PPC_KUAP")]
#[inline(always)]
pub unsafe fn __bad_kuap_fault(regs: *mut pt_regs, address: usize, is_write: bool) -> bool {
    let kuap = (*regs).kuap;

    if !is_write {
        return false;
    }
    if kuap == KUAP_NONE {
        return true;
    }

    // If faulting address doesn't match unlocked segment, change segment.
    // In case of unaligned store crossing two segments, emulate store.
    if ((kuap ^ address) & 0xf0000000) != 0 {
        if (kuap & 0x0fffffff) == 0 && address > kuap.wrapping_sub(4) && fix_alignment(regs) {
            regs_add_return_ip(regs, 4);
            emulate_single_step(regs);
        } else {
            (*regs).kuap = address;
        }
    }

    false
}


// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
