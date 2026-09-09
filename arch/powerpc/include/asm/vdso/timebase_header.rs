/* SPDX-License-Identifier: GPL-2.0-or-later */
/*
 * Common timebase prototypes and such for all ppc machines.
 *
 * Translated from the C header.  Configuration-specific assembly is retained
 * through Rust cfg attributes; symbols such as SPRN_TBRL are supplied by the
 * surrounding PowerPC headers.
 */

#[cfg(all(target_arch = "powerpc64", any(feature = "CONFIG_PPC_CELL", feature = "CONFIG_PPC_E500")))]
#[inline(always)]
pub unsafe fn mftb() -> usize {
    let mut rval: usize;
    core::arch::asm!(
        "90: mfspr {r}, {spr};",
        "cmpwi {r}, 0;",
        "beq- 90b;",
        r = out(reg) rval,
        spr = const SPRN_TBRL,
        options(nostack)
    );
    rval
}

#[cfg(feature = "CONFIG_PPC_8xx")]
#[inline(always)]
pub unsafe fn mftb() -> usize {
    let mut rval: usize;
    core::arch::asm!("mftbl {r}", r = out(reg) rval, options(nostack));
    rval
}

#[cfg(not(any(
    all(target_arch = "powerpc64", any(feature = "CONFIG_PPC_CELL", feature = "CONFIG_PPC_E500")),
    feature = "CONFIG_PPC_8xx"
)))]
#[inline(always)]
pub unsafe fn mftb() -> usize {
    let mut rval: usize;
    core::arch::asm!("mfspr {r}, {spr}", r = out(reg) rval, spr = const SPRN_TBRL, options(nostack));
    rval
}

#[cfg(feature = "CONFIG_PPC_8xx")]
#[inline(always)]
pub unsafe fn mftbu() -> usize {
    let mut rval: usize;
    core::arch::asm!("mftbu {r}", r = out(reg) rval, options(nostack));
    rval
}

#[cfg(not(feature = "CONFIG_PPC_8xx"))]
#[inline(always)]
pub unsafe fn mftbu() -> usize {
    let mut rval: usize;
    core::arch::asm!("mfspr {r}, {spr}", r = out(reg) rval, spr = const SPRN_TBRU, options(nostack));
    rval
}

#[inline(always)]
pub unsafe fn mttbl(v: usize) {
    core::arch::asm!("mttbl {value}", value = in(reg) v, options(nostack));
}

#[inline(always)]
pub unsafe fn mttbu(v: usize) {
    core::arch::asm!("mttbu {value}", value = in(reg) v, options(nostack));
}

#[inline(always)]
pub unsafe fn get_tb() -> u64 {
    /* __powerpc64__ selects the 64-bit VDSO implementation, including the
     * deliberate distinction from CONFIG_PPC64 used by the original header.
     */
    #[cfg(target_arch = "powerpc64")]
    {
        return mftb() as u64;
    }

    let mut tbhi: u32;
    let mut tblo: u32;
    let mut tbhi2: u32;
    loop {
        tbhi = mftbu() as u32;
        tblo = mftb() as u32;
        tbhi2 = mftbu() as u32;
        if tbhi == tbhi2 {
            break;
        }
    }

    ((tbhi as u64) << 32) | (tblo as u64)
}

#[inline]
pub unsafe fn set_tb(upper: u32, lower: u32) {
    mtspr(SPRN_TBWL, 0);
    mtspr(SPRN_TBWU, upper);
    mtspr(SPRN_TBWL, lower);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
