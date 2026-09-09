/* SPDX-License-Identifier: GPL-2.0 */

// C header guard: _ASM_X86_FPU_XCR_H

pub const XCR_XFEATURE_ENABLED_MASK: u32 = 0x00000000;
pub const XCR_XFEATURE_IN_USE_MASK: u32 = 0x00000001;

#[inline(always)]
pub unsafe fn xgetbv(index: u32) -> u64 {
    let eax: u32;
    let edx: u32;

    core::arch::asm!(
        "xgetbv",
        out("eax") eax,
        out("edx") edx,
        in("ecx") index,
    );
    (eax as u64).wrapping_add((edx as u64) << 32)
}

#[inline]
pub unsafe fn xsetbv(index: u32, value: u64) {
    let eax = value as u32;
    let edx = (value >> 32) as u32;

    core::arch::asm!(
        "xsetbv",
        in("eax") eax,
        in("edx") edx,
        in("ecx") index,
    );
}

/*
 * Return a mask of xfeatures which are currently being tracked
 * by the processor as being not in the initial configuration.
 *
 * Callers should check X86_FEATURE_XGETBV1.
 */
#[inline(always)]
pub unsafe fn xfeatures_in_use() -> u64 {
    xgetbv(XCR_XFEATURE_IN_USE_MASK)
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
