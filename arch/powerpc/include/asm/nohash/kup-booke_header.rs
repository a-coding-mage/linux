/* SPDX-License-Identifier: GPL-2.0 */

/* Translated from kup-booke.h.  Required kernel and architecture symbols are
 * supplied by the surrounding translation unit. */

#[cfg(feature = "CONFIG_PPC_KUAP")]
#[cfg(not(feature = "__ASSEMBLER__"))]
use core::ffi::{c_ulong, c_void};

/* Under __ASSEMBLER__, kuap_check_amr(gpr1, gpr2) expands to no instructions. */

#[cfg(feature = "CONFIG_PPC_KUAP")]
#[cfg(not(feature = "__ASSEMBLER__"))]
#[inline(always)]
pub unsafe fn __kuap_lock() {
    mtspr(SPRN_PID, 0);
    isync();
}

#[cfg(feature = "CONFIG_PPC_KUAP")]
#[cfg(not(feature = "__ASSEMBLER__"))]
#[inline(always)]
pub unsafe fn __kuap_save_and_lock(regs: *mut pt_regs) {
    (*regs).kuap = mfspr(SPRN_PID);
    mtspr(SPRN_PID, 0);
    isync();
}

#[cfg(feature = "CONFIG_PPC_KUAP")]
#[cfg(not(feature = "__ASSEMBLER__"))]
#[inline(always)]
pub unsafe fn kuap_user_restore(regs: *mut pt_regs) {
    if kuap_is_disabled() {
        return;
    }

    mtspr(SPRN_PID, (*current).thread.pid);

    /* Context synchronisation is performed by rfi. */
    let _ = regs;
}

#[cfg(feature = "CONFIG_PPC_KUAP")]
#[cfg(not(feature = "__ASSEMBLER__"))]
#[inline(always)]
pub unsafe fn __kuap_kernel_restore(regs: *mut pt_regs, kuap: c_ulong) {
    if (*regs).kuap != 0 {
        mtspr(SPRN_PID, (*current).thread.pid);
    }

    /* Context synchronisation is performed by rfi. */
    let _ = kuap;
}

#[cfg(all(feature = "CONFIG_PPC_KUAP", feature = "CONFIG_PPC_KUAP_DEBUG"))]
#[cfg(not(feature = "__ASSEMBLER__"))]
#[inline(always)]
pub unsafe fn __kuap_get_and_assert_locked() -> c_ulong {
    WARN_ON_ONCE(mfspr(SPRN_PID));
    0
}

#[cfg(feature = "CONFIG_PPC_KUAP")]
#[cfg(not(feature = "__ASSEMBLER__"))]
#[inline(always)]
pub unsafe fn uaccess_begin_booke(val: c_ulong) {
    /* ASM_MMU_FTR_IFSET("mtspr %0, %1; isync", "", MMU_FTR_KUAP). */
    mtspr(SPRN_PID, val);
    isync();
}

#[cfg(feature = "CONFIG_PPC_KUAP")]
#[cfg(not(feature = "__ASSEMBLER__"))]
#[inline(always)]
pub unsafe fn uaccess_end_booke() {
    /* ASM_MMU_FTR_IFSET("mtspr %0, %1; isync", "", MMU_FTR_KUAP). */
    mtspr(SPRN_PID, 0);
    isync();
}

#[cfg(feature = "CONFIG_PPC_KUAP")]
#[cfg(not(feature = "__ASSEMBLER__"))]
#[inline(always)]
pub unsafe fn allow_user_access(to: *mut c_void, dir: c_ulong) {
    uaccess_begin_booke((*current).thread.pid);
    let _ = (to, dir);
}

#[cfg(feature = "CONFIG_PPC_KUAP")]
#[cfg(not(feature = "__ASSEMBLER__"))]
#[inline(always)]
pub unsafe fn prevent_user_access(dir: c_ulong) {
    uaccess_end_booke();
    let _ = dir;
}

#[cfg(feature = "CONFIG_PPC_KUAP")]
#[cfg(not(feature = "__ASSEMBLER__"))]
#[inline(always)]
pub unsafe fn prevent_user_access_return() -> c_ulong {
    let flags = mfspr(SPRN_PID);
    uaccess_end_booke();
    flags
}

#[cfg(feature = "CONFIG_PPC_KUAP")]
#[cfg(not(feature = "__ASSEMBLER__"))]
#[inline(always)]
pub unsafe fn restore_user_access(flags: c_ulong) {
    if flags != 0 {
        uaccess_begin_booke((*current).thread.pid);
    }
}

#[cfg(feature = "CONFIG_PPC_KUAP")]
#[cfg(not(feature = "__ASSEMBLER__"))]
#[inline(always)]
pub unsafe fn __bad_kuap_fault(
    regs: *mut pt_regs,
    address: c_ulong,
    is_write: bool,
) -> bool {
    let _ = (address, is_write);
    (*regs).kuap == 0
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
