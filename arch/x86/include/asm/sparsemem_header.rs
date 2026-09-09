/* SPDX-License-Identifier: GPL-2.0 */

// Translated from the x86 sparsemem header.
// The declarations below are active only when CONFIG_SPARSEMEM is enabled.

#[cfg(feature = "CONFIG_SPARSEMEM")]
pub mod sparsemem {
    /*
     * Generic non-linear memory support:
     *
     * We will not split memory into more chunks than will fit into the flags
     * field of struct page.
     *
     * SECTION_SIZE_BITS: 2^n, size of each section
     * MAX_PHYSMEM_BITS: 2^n, maximum size of the physical address space
     */

    // CONFIG_X86_32 and CONFIG_X86_PAE are build-time configuration
    // conditions from the original header.
    #[cfg(all(feature = "CONFIG_X86_32", feature = "CONFIG_X86_PAE"))]
    pub const SECTION_SIZE_BITS: u32 = 29;

    #[cfg(all(feature = "CONFIG_X86_32", feature = "CONFIG_X86_PAE"))]
    pub const MAX_PHYSMEM_BITS: u32 = 36;

    #[cfg(all(feature = "CONFIG_X86_32", not(feature = "CONFIG_X86_PAE")))]
    pub const SECTION_SIZE_BITS: u32 = 26;

    #[cfg(all(feature = "CONFIG_X86_32", not(feature = "CONFIG_X86_PAE")))]
    pub const MAX_PHYSMEM_BITS: u32 = 32;

    #[cfg(not(feature = "CONFIG_X86_32"))]
    pub const SECTION_SIZE_BITS: u32 = 27; // matt - 128 is convenient right now

    // The original macro evaluates pgtable_l5_enabled() at use time.
    #[cfg(not(feature = "CONFIG_X86_32"))]
    pub unsafe fn MAX_PHYSMEM_BITS() -> u32 {
        if pgtable_l5_enabled() != 0 { 52 } else { 46 }
    }

    #[cfg(not(feature = "CONFIG_X86_32"))]
    unsafe extern "C" {
        pub fn pgtable_l5_enabled() -> i32;
    }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
