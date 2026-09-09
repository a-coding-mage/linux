/* SPDX-License-Identifier: GPL-2.0 */

// Dependency intent preserved from <uapi/asm/processor-flags.h> and
// <linux/mem_encrypt.h>.

#[cfg(feature = "CONFIG_VM86")]
pub const X86_VM_MASK: u64 = X86_EFLAGS_VM;

#[cfg(not(feature = "CONFIG_VM86"))]
pub const X86_VM_MASK: u64 = 0; // No VM86 support

/*
 * CR3's layout varies depending on several things.
 *
 * If CR4.PCIDE is set (64-bit only), then CR3[11:0] is the address space ID.
 * If PAE is enabled, then CR3[11:5] is part of the PDPT address
 * (i.e. it's 32-byte aligned, not page-aligned) and CR3[4:0] is ignored.
 * Otherwise (non-PAE, non-PCID), CR3[3] is PWT, CR3[4] is PCD, and
 * CR3[2:0] and CR3[11:5] are ignored.
 *
 * In all cases, Linux puts zeros in the low ignored bits and in PWT and PCD.
 *
 * CR3[63] is always read as zero.  If CR4.PCIDE is set, then CR3[63] may be
 * written as 1 to prevent the write to CR3 from flushing the TLB.
 *
 * On systems with SME, one bit (in a variable position!) is stolen to indicate
 * that the top-level paging structure is encrypted.
 *
 * On systemms with LAM, bits 61 and 62 are used to indicate LAM mode.
 *
 * All of the remaining bits indicate the physical address of the top-level
 * paging structure.
 *
 * CR3_ADDR_MASK is the mask used by read_cr3_pa().
 */
#[cfg(feature = "CONFIG_X86_64")]
pub const CR3_ADDR_MASK: u64 = __sme_clr!(PHYSICAL_PAGE_MASK);

#[cfg(feature = "CONFIG_X86_64")]
pub const CR3_PCID_MASK: u64 = 0xFFFull;

#[cfg(feature = "CONFIG_X86_64")]
pub const CR3_NOFLUSH: u64 = 1u64 << 63;

/*
 * CR3_ADDR_MASK needs at least bits 31:5 set on PAE systems, and we save
 * a tiny bit of code size by setting all the bits.
 */
#[cfg(not(feature = "CONFIG_X86_64"))]
pub const CR3_ADDR_MASK: u64 = 0xFFFFFFFFu64;

#[cfg(not(feature = "CONFIG_X86_64"))]
pub const CR3_PCID_MASK: u64 = 0u64;

#[cfg(not(feature = "CONFIG_X86_64"))]
pub const CR3_NOFLUSH: u64 = 0;

#[cfg(feature = "CONFIG_MITIGATION_PAGE_TABLE_ISOLATION")]
pub const X86_CR3_PTI_PCID_USER_BIT: u32 = 11;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
