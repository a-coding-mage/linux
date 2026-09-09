/* SPDX-License-Identifier: GPL-2.0 */
/* C header guard and __KERNEL__ conditional omitted; declarations preserved below. */

/*
 * Definitions for PPC440
 *
 * Because of the 3 word TLB entries to support 36-bit addressing,
 * the attribute are difficult to map in such a fashion that they
 * are easily loaded during exception processing.  I decided to
 * organize the entry so the ERPN is the only portion in the
 * upper word of the PTE and the attribute bits below are packed
 * in as sensibly as they can be in the area below a 4KB page size
 * oriented RPN.  This at least makes it easy to load the RPN and
 * ERPN fields in the TLB. -Matt
 *
 * This isn't entirely true anymore, at least some bits are now
 * easier to move into the TLB from the PTE. -BenH.
 *
 * Note that these bits preclude future use of a page size
 * less than 4KB.
 *
 *
 * PPC 440 core has following TLB attribute fields;
 *
 *   TLB1:
 *   0  1  2  3  4  ... 16 17 18 19 20 21 22 23 24 25 26 27 28 29 30 31
 *   RPN.................................  -  -  -  -  -  - ERPN.......
 *
 *   TLB2:
 *   0  1  2  3  4  ... 16 17 18 19 20 21 22 23 24 25 26 27 28 29 30 31
 *   -  -  -  -  -    - U0 U1 U2 U3 W  I  M  G  E   - UX UW UR SX SW SR
 *
 * Newer 440 cores (440x6 as used on AMCC 460EX/460GT) have additional
 * TLB2 storage attribute fields. Those are:
 *
 *   TLB2:
 *   0...10    11   12   13   14   15   16...31
 *   no change WL1  IL1I IL1D IL2I IL2D no change
 *
 * There are some constrains and options, to decide mapping software bits
 * into TLB entry.
 *
 *   - PRESENT *must* be in the bottom three bits because swap cache
 *     entries use the top 29 bits for TLB2.
 *
 *   - CACHE COHERENT bit (M) has no effect on original PPC440 cores,
 *     because it doesn't support SMP. However, some later 460 variants
 *     have -some- form of SMP support and so I keep the bit there for
 *     future use
 *
 * With the PPC 44x Linux implementation, the 0-11th LSBs of the PTE are used
 * for memory protection related functions (see PTE structure in
 * include/asm-ppc/mmu.h).  The _PAGE_XXX definitions in this file map to the
 * above bits.  Note that the bit values are CPU specific, not architecture
 * specific.
 *
 * The kernel PTE entry can be an ordinary PTE mapping a page or a special swap
 * PTE. In case of a swap PTE, LSB 2-24 are used to store information regarding
 * the swap entry. However LSB 0-1 still hold protection values, for example,
 * to distinguish swap PTEs from ordinary PTEs, and must be used with care.
 */

pub const _PAGE_PRESENT: u32 = 0x00000001; /* S: PTE valid */
pub const _PAGE_WRITE: u32 = 0x00000002; /* S: Write permission */
pub const _PAGE_EXEC: u32 = 0x00000004; /* H: Execute permission */
pub const _PAGE_READ: u32 = 0x00000008; /* S: Read permission */
pub const _PAGE_DIRTY: u32 = 0x00000010; /* S: Page dirty */
pub const _PAGE_SPECIAL: u32 = 0x00000020; /* S: Special page */
pub const _PAGE_ACCESSED: u32 = 0x00000040; /* S: Page referenced */
pub const _PAGE_ENDIAN: u32 = 0x00000080; /* H: E bit */
pub const _PAGE_GUARDED: u32 = 0x00000100; /* H: G bit */
pub const _PAGE_COHERENT: u32 = 0x00000200; /* H: M bit */
pub const _PAGE_NO_CACHE: u32 = 0x00000400; /* H: I bit */
pub const _PAGE_WRITETHRU: u32 = 0x00000800; /* H: W bit */

/* TODO: Add large page lowmem mapping support */
pub const _PMD_PRESENT: u32 = 0;
pub const _PMD_PRESENT_MASK: u32 = PAGE_MASK;
pub const _PMD_BAD: u32 = !PAGE_MASK;
pub const _PMD_USER: u32 = 0;

/* ERPN in a PTE never gets cleared, ignore it */
pub const _PTE_NONE_MASK: u64 = 0xffffffff00000000u64;

/*
 * We define 2 sets of base prot bits, one for basic pages (ie,
 * cacheable kernel and user pages) and one for non cacheable
 * pages. We always set _PAGE_COHERENT when SMP is enabled or
 * the processor might need it for DMA coherency.
 */
pub const _PAGE_BASE_NC: u32 = _PAGE_PRESENT | _PAGE_ACCESSED;
/* CONFIG_SMP is a build-time configuration condition from the C source. */
#[cfg(CONFIG_SMP)]
pub const _PAGE_BASE: u32 = _PAGE_BASE_NC | _PAGE_COHERENT;
#[cfg(not(CONFIG_SMP))]
pub const _PAGE_BASE: u32 = _PAGE_BASE_NC;

/* Dependency supplied by asm/pgtable-masks.h in the original header. */

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
