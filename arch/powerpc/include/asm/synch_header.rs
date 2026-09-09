/* SPDX-License-Identifier: GPL-2.0 */

// Kernel-only header translation. C preprocessor configuration symbols and
// assembly helper macros are represented with Rust cfgs/comments as applicable.

#[cfg(not(feature = "assembler"))]
extern "C" {
    pub static mut __start___lwsync_fixup: ::core::ffi::c_uint;
    pub static mut __stop___lwsync_fixup: ::core::ffi::c_uint;
    pub fn do_lwsync_fixups(
        value: ::core::ffi::c_ulong,
        fixup_start: *mut ::core::ffi::c_void,
        fixup_end: *mut ::core::ffi::c_void,
    );

    #[inline]
    pub unsafe fn eieio() {
        // CONFIG_BOOKE selects `mbar`; other PowerPC targets use `eieio`.
        #[cfg(feature = "booke")]
        ::core::arch::asm!("mbar", options(nostack, preserves_flags));
        #[cfg(not(feature = "booke"))]
        ::core::arch::asm!("eieio", options(nostack, preserves_flags));
    }

    #[inline]
    pub unsafe fn isync() {
        ::core::arch::asm!("isync", options(nostack, preserves_flags));
    }

    #[inline]
    pub unsafe fn ppc_after_tlbiel_barrier() {
        ::core::arch::asm!("ptesync", options(nostack, preserves_flags));
        /*
         * POWER9, POWER10 need a cp_abort after tlbiel to ensure the copy is
         * invalidated correctly. If this is not done, the paste can take data
         * from the physical address that was translated at copy time.
         *
         * POWER9 in practice does not need this, because address spaces with
         * accelerators mapped will use tlbie (which does invalidate the copy)
         * to invalidate translations. It's not possible to limit POWER10 this
         * way due to local copy-paste.
         *
         * POWER12 does not need it.
         */
        // ASM_FTR_IF(PPC_CP_ABORT, "", CPU_FTR_ARCH_31|CPU_FTR_ARCH_32,
        //            CPU_FTR_ARCH_31), preserving the source feature-fixup
        // intent for the external assembly macro definitions.
    }
}

// LWSYNC:
//   __powerpc64__: lwsync
//   CONFIG_PPC_E500: START_LWSYNC_SECTION(96); sync;
//                   MAKE_LWSYNC_SECTION_ENTRY(96, __lwsync_fixup)
//   otherwise: sync
#[cfg(target_arch = "powerpc64")]
pub const LWSYNC: &str = "lwsync";
#[cfg(all(not(target_arch = "powerpc64"), feature = "ppc_e500"))]
pub const LWSYNC: &str =
    "START_LWSYNC_SECTION(96); sync; MAKE_LWSYNC_SECTION_ENTRY(96, __lwsync_fixup);";
#[cfg(all(not(target_arch = "powerpc64"), not(feature = "ppc_e500")))]
pub const LWSYNC: &str = "sync";

#[cfg(feature = "smp")]
pub const PPC_ACQUIRE_BARRIER: &str =
    "\nSTART_LWSYNC_SECTION(97); isync; MAKE_LWSYNC_SECTION_ENTRY(97, __lwsync_fixup);";
#[cfg(feature = "smp")]
pub const PPC_RELEASE_BARRIER: &str = "LWSYNC\n";
#[cfg(feature = "smp")]
pub const PPC_ATOMIC_ENTRY_BARRIER: &str = "\nsync\n";
#[cfg(feature = "smp")]
pub const PPC_ATOMIC_EXIT_BARRIER: &str = "\nsync\n";

#[cfg(not(feature = "smp"))]
pub const PPC_ACQUIRE_BARRIER: &str = "";
#[cfg(not(feature = "smp"))]
pub const PPC_RELEASE_BARRIER: &str = "";
#[cfg(not(feature = "smp"))]
pub const PPC_ATOMIC_ENTRY_BARRIER: &str = "";
#[cfg(not(feature = "smp"))]
pub const PPC_ATOMIC_EXIT_BARRIER: &str = "";

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
