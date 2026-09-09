/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * Supervisor Mode Access Prevention support
 *
 * Translated from the C header smap.h.
 * The original alternative() machinery and feature constants are supplied by
 * other headers/build configuration.
 */

/* __ASSEMBLER__ alternatives:
 * ASM_CLAC: ALTERNATIVE("", "clac", X86_FEATURE_SMAP)
 * ASM_STAC: ALTERNATIVE("", "stac", X86_FEATURE_SMAP)
 */

/*
 * The CLAC/STAC instructions toggle SMAP/LASS enforcement.  The original
 * implementation uses alternative() so the instruction is emitted only when
 * the corresponding CPU feature is available.
 */
#[inline(always)]
pub unsafe fn clac() {
    core::arch::asm!("clac", options(nostack, preserves_flags));
}

#[inline(always)]
pub unsafe fn stac() {
    core::arch::asm!("stac", options(nostack, preserves_flags));
}

#[inline(always)]
pub unsafe fn lass_clac() {
    core::arch::asm!("clac", options(nostack, preserves_flags));
}

#[inline(always)]
pub unsafe fn lass_stac() {
    core::arch::asm!("stac", options(nostack, preserves_flags));
}

#[inline(always)]
pub unsafe fn smap_save() -> usize {
    let flags: usize;
    core::arch::asm!(
        "pushf",
        "pop {flags}",
        "clac",
        flags = lateout(reg) flags,
        options(preserves_flags),
    );
    flags
}

#[inline(always)]
pub unsafe fn smap_restore(flags: usize) {
    core::arch::asm!(
        "push {flags}",
        "popf",
        flags = in(reg) flags,
        options(preserves_flags),
    );
}

/* These macros can be used in asm! statements. */
#[macro_export]
macro_rules! ASM_CLAC {
    () => { "clac" };
}

#[macro_export]
macro_rules! ASM_STAC {
    () => { "stac" };
}

#[macro_export]
macro_rules! ASM_CLAC_UNSAFE {
    () => { "clac" };
}

#[macro_export]
macro_rules! ASM_STAC_UNSAFE {
    () => { "stac" };
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
