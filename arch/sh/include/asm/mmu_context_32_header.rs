/* SPDX-License-Identifier: GPL-2.0 */

// CONFIG_CPU_HAS_PTEAEX selects the PTEAEX-based implementation.
#[cfg(feature = "CONFIG_CPU_HAS_PTEAEX")]
#[inline]
pub unsafe fn set_asid(asid: ::core::ffi::c_ulong) {
    __raw_writel(asid, MMU_PTEAEX);
}

#[cfg(feature = "CONFIG_CPU_HAS_PTEAEX")]
#[inline]
pub unsafe fn get_asid() -> ::core::ffi::c_ulong {
    __raw_readl(MMU_PTEAEX) & MMU_CONTEXT_ASID_MASK
}

#[cfg(not(feature = "CONFIG_CPU_HAS_PTEAEX"))]
#[inline]
pub unsafe fn set_asid(asid: ::core::ffi::c_ulong) {
    let mut __dummy: ::core::ffi::c_ulong;

    // Original SH inline assembly:
    // mov.l %2, %0; and %3, %0; or %1, %0; mov.l %0, %2
    // The SH-specific inline assembly is intentionally preserved here as a
    // comment because it has no portable Rust assembler equivalent.
    let _ = (&mut __dummy, asid, MMU_PTEH, 0xffffff00u32);
}

#[cfg(not(feature = "CONFIG_CPU_HAS_PTEAEX"))]
#[inline]
pub unsafe fn get_asid() -> ::core::ffi::c_ulong {
    let mut asid: ::core::ffi::c_ulong;

    // Original SH inline assembly: mov.l %1, %0
    // The SH-specific inline assembly is intentionally preserved here as a
    // comment because it has no portable Rust assembler equivalent.
    asid = __raw_readl(MMU_PTEH);
    asid &= MMU_CONTEXT_ASID_MASK;
    asid
}

/* MMU_TTB is used for optimizing the fault handling. */
#[inline]
pub unsafe fn set_TTB(pgd: *mut pgd_t) {
    __raw_writel(pgd as ::core::ffi::c_ulong, MMU_TTB);
}

#[inline]
pub unsafe fn get_TTB() -> *mut pgd_t {
    __raw_readl(MMU_TTB) as *mut pgd_t
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
