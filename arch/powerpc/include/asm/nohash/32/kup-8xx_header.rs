/* SPDX-License-Identifier: GPL-2.0 */

// Translated from the C header. The original declarations are active only
// when CONFIG_PPC_KUAP is enabled and, except for assembler builds, use the
// architecture-provided pt_regs, SPR, MMU, and warning definitions.

#[cfg(feature = "CONFIG_PPC_KUAP")]
#[inline(always)]
pub unsafe fn __kuap_save_and_lock(regs: *mut pt_regs) {
    (*regs).kuap = mfspr(SPRN_MD_AP);
    mtspr(SPRN_MD_AP, MD_APG_KUAP);
}

#[cfg(feature = "CONFIG_PPC_KUAP")]
#[inline(always)]
pub unsafe fn kuap_user_restore(_regs: *mut pt_regs) {
}

#[cfg(feature = "CONFIG_PPC_KUAP")]
#[inline(always)]
pub unsafe fn __kuap_kernel_restore(regs: *mut pt_regs, _kuap: ::core::ffi::c_ulong) {
    mtspr(SPRN_MD_AP, (*regs).kuap);
}

#[cfg(all(feature = "CONFIG_PPC_KUAP", feature = "CONFIG_PPC_KUAP_DEBUG"))]
#[inline(always)]
pub unsafe fn __kuap_get_and_assert_locked() -> ::core::ffi::c_ulong {
    WARN_ON_ONCE((mfspr(SPRN_MD_AP) >> 16) != (MD_APG_KUAP >> 16));
    0
}

#[cfg(feature = "CONFIG_PPC_KUAP")]
#[inline(always)]
pub unsafe fn uaccess_begin_8xx(val: ::core::ffi::c_ulong) {
    // ASM_MMU_FTR_IFSET("mtspr %0, %1", "", MMU_FTR_KUAP), with a memory
    // clobber, is architecture-specific and must be supplied by the target.
    asm_mmu_ftr_ifset_mtspr(SPRN_MD_AP, val, MMU_FTR_KUAP);
}

#[cfg(feature = "CONFIG_PPC_KUAP")]
#[inline(always)]
pub unsafe fn uaccess_end_8xx() {
    // ASM_MMU_FTR_IFSET("mtspr %0, %1", "", MMU_FTR_KUAP), with a memory
    // clobber, is architecture-specific and must be supplied by the target.
    asm_mmu_ftr_ifset_mtspr(SPRN_MD_AP, MD_APG_KUAP, MMU_FTR_KUAP);
}

#[cfg(feature = "CONFIG_PPC_KUAP")]
#[inline(always)]
pub unsafe fn allow_user_access(_to: *mut ::core::ffi::c_void, _dir: ::core::ffi::c_ulong) {
    uaccess_begin_8xx(MD_APG_INIT);
}

#[cfg(feature = "CONFIG_PPC_KUAP")]
#[inline(always)]
pub unsafe fn prevent_user_access(_dir: ::core::ffi::c_ulong) {
    uaccess_end_8xx();
}

#[cfg(feature = "CONFIG_PPC_KUAP")]
#[inline(always)]
pub unsafe fn prevent_user_access_return() -> ::core::ffi::c_ulong {
    let flags = mfspr(SPRN_MD_AP);
    uaccess_end_8xx();
    flags
}

#[cfg(feature = "CONFIG_PPC_KUAP")]
#[inline(always)]
pub unsafe fn restore_user_access(flags: ::core::ffi::c_ulong) {
    uaccess_begin_8xx(flags);
}

#[cfg(feature = "CONFIG_PPC_KUAP")]
#[inline(always)]
pub unsafe fn __bad_kuap_fault(
    regs: *mut pt_regs,
    _address: ::core::ffi::c_ulong,
    _is_write: bool,
) -> bool {
    !(((*regs).kuap ^ MD_APG_KUAP) & 0xff00_0000 != 0)
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
