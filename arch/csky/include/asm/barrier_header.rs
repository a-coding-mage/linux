/* SPDX-License-Identifier: GPL-2.0 */

/* C header guard: __ASM_CSKY_BARRIER_H */
/* C __ASSEMBLER__ exclusion: these declarations apply to non-assembler builds. */

#[inline(always)]
pub unsafe fn nop() {
    core::arch::asm!("nop", options(nostack));
}

/* CONFIG_SMP conditional declarations. */
#[cfg(feature = "CONFIG_SMP")]
pub const FULL_FENCE: &str = ".long 0x842fc000\n";
#[cfg(feature = "CONFIG_SMP")]
pub const ACQUIRE_FENCE: &str = ".long 0x8427c000\n";
#[cfg(feature = "CONFIG_SMP")]
pub const RELEASE_FENCE: &str = ".long 0x842ec000\n";

#[cfg(feature = "CONFIG_SMP")]
#[inline(always)]
pub unsafe fn __bar_brw() { core::arch::asm!(".long 0x842cc000", options(nostack)); }
#[cfg(feature = "CONFIG_SMP")]
#[inline(always)]
pub unsafe fn __bar_br() { core::arch::asm!(".long 0x8424c000", options(nostack)); }
#[cfg(feature = "CONFIG_SMP")]
#[inline(always)]
pub unsafe fn __bar_bw() { core::arch::asm!(".long 0x8428c000", options(nostack)); }
#[cfg(feature = "CONFIG_SMP")]
#[inline(always)]
pub unsafe fn __bar_arw() { core::arch::asm!(".long 0x8423c000", options(nostack)); }
#[cfg(feature = "CONFIG_SMP")]
#[inline(always)]
pub unsafe fn __bar_ar() { core::arch::asm!(".long 0x8421c000", options(nostack)); }
#[cfg(feature = "CONFIG_SMP")]
#[inline(always)]
pub unsafe fn __bar_aw() { core::arch::asm!(".long 0x8422c000", options(nostack)); }
#[cfg(feature = "CONFIG_SMP")]
#[inline(always)]
pub unsafe fn __bar_brwarw() { core::arch::asm!(".long 0x842fc000", options(nostack)); }
#[cfg(feature = "CONFIG_SMP")]
#[inline(always)]
pub unsafe fn __bar_brarw() { core::arch::asm!(".long 0x8427c000", options(nostack)); }
#[cfg(feature = "CONFIG_SMP")]
#[inline(always)]
pub unsafe fn __bar_bwarw() { core::arch::asm!(".long 0x842bc000", options(nostack)); }
#[cfg(feature = "CONFIG_SMP")]
#[inline(always)]
pub unsafe fn __bar_brwar() { core::arch::asm!(".long 0x842dc000", options(nostack)); }
#[cfg(feature = "CONFIG_SMP")]
#[inline(always)]
pub unsafe fn __bar_brwaw() { core::arch::asm!(".long 0x842ec000", options(nostack)); }
#[cfg(feature = "CONFIG_SMP")]
#[inline(always)]
pub unsafe fn __bar_brar() { core::arch::asm!(".long 0x8425c000", options(nostack)); }
/* The source defines __bar_brar twice; the repeated definition is identical. */
#[cfg(feature = "CONFIG_SMP")]
#[inline(always)]
pub unsafe fn __bar_bwaw() { core::arch::asm!(".long 0x842ac000", options(nostack)); }

#[cfg(feature = "CONFIG_SMP")]
#[inline(always)]
pub unsafe fn __smp_mb() { __bar_brwarw(); }
#[cfg(feature = "CONFIG_SMP")]
#[inline(always)]
pub unsafe fn __smp_rmb() { __bar_brar(); }
#[cfg(feature = "CONFIG_SMP")]
#[inline(always)]
pub unsafe fn __smp_wmb() { __bar_bwaw(); }
#[cfg(feature = "CONFIG_SMP")]
#[inline(always)]
pub unsafe fn __smp_acquire_fence() { __bar_brarw(); }
#[cfg(feature = "CONFIG_SMP")]
#[inline(always)]
pub unsafe fn __smp_release_fence() { __bar_brwaw(); }

#[inline(always)]
pub unsafe fn mb() {
    core::arch::asm!("sync", options(nostack));
}

#[cfg(feature = "CONFIG_CPU_HAS_CACHEV2")]
#[inline(always)]
pub unsafe fn sync_is() {
    core::arch::asm!("sync.is\nsync.is\nsync.is", options(nostack));
}

/* Dependency: <asm-generic/barrier.h> */

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
