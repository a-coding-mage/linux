/*
 * This file is subject to the terms and conditions of the GNU General Public
 * License.  See the file "COPYING" in the main directory of this archive
 * for more details.
 *
 * Copyright (C) 1994 - 2002 by Ralf Baechle
 * Copyright (C) 1999, 2000, 2001 Silicon Graphics, Inc.
 * Copyright (C) 2002  Maciej W. Rozycki
 */

/* Configuration branches below mirror the original C preprocessor conditions. */

#[cfg(CONFIG_XPA)]
#[repr(isize)]
pub enum pgtable_bits {
    /* Used by TLB hardware (placed in EntryLo*) */
    _PAGE_NO_EXEC_SHIFT,
    _PAGE_NO_READ_SHIFT,
    _PAGE_GLOBAL_SHIFT,
    _PAGE_VALID_SHIFT,
    _PAGE_DIRTY_SHIFT,
    _CACHE_SHIFT,

    /* Used only by software (masked out before writing EntryLo*) */
    _PAGE_PRESENT_SHIFT = 24,
    _PAGE_WRITE_SHIFT,
    _PAGE_ACCESSED_SHIFT,
    _PAGE_MODIFIED_SHIFT,
    #[cfg(CONFIG_ARCH_HAS_PTE_SPECIAL)]
    _PAGE_SPECIAL_SHIFT,
    #[cfg(CONFIG_HAVE_ARCH_SOFT_DIRTY)]
    _PAGE_SOFT_DIRTY_SHIFT,
}

#[cfg(CONFIG_XPA)]
pub const _PFNX_MASK: u64 = 0xffffff;

#[cfg(all(CONFIG_PHYS_ADDR_T_64BIT, CONFIG_CPU_MIPS32, not(CONFIG_XPA)))]
#[repr(isize)]
pub enum pgtable_bits {
    /* Used by TLB hardware (placed in EntryLo*) */
    _PAGE_GLOBAL_SHIFT,
    _PAGE_VALID_SHIFT,
    _PAGE_DIRTY_SHIFT,
    _CACHE_SHIFT,

    /* Used only by software (masked out before writing EntryLo*) */
    _PAGE_PRESENT_SHIFT = _CACHE_SHIFT as isize + 3,
    _PAGE_NO_READ_SHIFT,
    _PAGE_WRITE_SHIFT,
    _PAGE_ACCESSED_SHIFT,
    _PAGE_MODIFIED_SHIFT,
    #[cfg(CONFIG_ARCH_HAS_PTE_SPECIAL)]
    _PAGE_SPECIAL_SHIFT,
    #[cfg(CONFIG_HAVE_ARCH_SOFT_DIRTY)]
    _PAGE_SOFT_DIRTY_SHIFT,
}

#[cfg(all(CONFIG_CPU_R3K_TLB, not(CONFIG_XPA), not(all(CONFIG_PHYS_ADDR_T_64BIT, CONFIG_CPU_MIPS32))))]
#[repr(isize)]
pub enum pgtable_bits {
    /* Used only by software (writes to EntryLo ignored) */
    _PAGE_PRESENT_SHIFT,
    _PAGE_NO_READ_SHIFT,
    _PAGE_WRITE_SHIFT,
    _PAGE_ACCESSED_SHIFT,
    _PAGE_MODIFIED_SHIFT,
    #[cfg(CONFIG_ARCH_HAS_PTE_SPECIAL)]
    _PAGE_SPECIAL_SHIFT,
    #[cfg(CONFIG_HAVE_ARCH_SOFT_DIRTY)]
    _PAGE_SOFT_DIRTY_SHIFT,

    /* Used by TLB hardware (placed in EntryLo) */
    _PAGE_GLOBAL_SHIFT = 8,
    _PAGE_VALID_SHIFT,
    _PAGE_DIRTY_SHIFT,
    _CACHE_UNCACHED_SHIFT,
}

#[cfg(all(not(CONFIG_XPA), not(all(CONFIG_PHYS_ADDR_T_64BIT, CONFIG_CPU_MIPS32)), not(CONFIG_CPU_R3K_TLB)))]
#[repr(isize)]
pub enum pgtable_bits {
    /* Used only by software (masked out before writing EntryLo*) */
    _PAGE_PRESENT_SHIFT,
    #[cfg(not(CONFIG_CPU_HAS_RIXI))]
    _PAGE_NO_READ_SHIFT,
    _PAGE_WRITE_SHIFT,
    _PAGE_ACCESSED_SHIFT,
    _PAGE_MODIFIED_SHIFT,
    #[cfg(CONFIG_MIPS_HUGE_TLB_SUPPORT)]
    _PAGE_HUGE_SHIFT,
    #[cfg(CONFIG_ARCH_HAS_PTE_SPECIAL)]
    _PAGE_SPECIAL_SHIFT,
    #[cfg(CONFIG_HAVE_ARCH_SOFT_DIRTY)]
    _PAGE_SOFT_DIRTY_SHIFT,
    /* Used by TLB hardware (placed in EntryLo*) */
    #[cfg(CONFIG_CPU_HAS_RIXI)]
    _PAGE_NO_EXEC_SHIFT,
    #[cfg(CONFIG_CPU_HAS_RIXI)]
    _PAGE_NO_READ_SHIFT,
    _PAGE_GLOBAL_SHIFT,
    _PAGE_VALID_SHIFT,
    _PAGE_DIRTY_SHIFT,
    _CACHE_SHIFT,
}

/* Used only by software */
pub const _PAGE_PRESENT: u64 = 1u64 << (_PAGE_PRESENT_SHIFT as u32);
pub const _PAGE_WRITE: u64 = 1u64 << (_PAGE_WRITE_SHIFT as u32);
pub const _PAGE_ACCESSED: u64 = 1u64 << (_PAGE_ACCESSED_SHIFT as u32);
pub const _PAGE_MODIFIED: u64 = 1u64 << (_PAGE_MODIFIED_SHIFT as u32);
#[cfg(CONFIG_MIPS_HUGE_TLB_SUPPORT)]
pub const _PAGE_HUGE: u64 = 1u64 << (_PAGE_HUGE_SHIFT as u32);
#[cfg(CONFIG_ARCH_HAS_PTE_SPECIAL)]
pub const _PAGE_SPECIAL: u64 = 1u64 << (_PAGE_SPECIAL_SHIFT as u32);
#[cfg(not(CONFIG_ARCH_HAS_PTE_SPECIAL))]
pub const _PAGE_SPECIAL: u64 = 0;
#[cfg(CONFIG_HAVE_ARCH_SOFT_DIRTY)]
pub const _PAGE_SOFT_DIRTY: u64 = 1u64 << (_PAGE_SOFT_DIRTY_SHIFT as u32);
#[cfg(not(CONFIG_HAVE_ARCH_SOFT_DIRTY))]
pub const _PAGE_SOFT_DIRTY: u64 = 0;

/* Used by TLB hardware (placed in EntryLo*) */
#[cfg(CONFIG_XPA)]
pub const _PAGE_NO_EXEC: u64 = 1u64 << (_PAGE_NO_EXEC_SHIFT as u32);
#[cfg(all(not(CONFIG_XPA), CONFIG_CPU_HAS_RIXI))]
pub const _PAGE_NO_EXEC: u64 = if cpu_has_rixi { 1u64 << (_PAGE_NO_EXEC_SHIFT as u32) } else { 0 };
pub const _PAGE_NO_READ: u64 = 1u64 << (_PAGE_NO_READ_SHIFT as u32);
pub const _PAGE_GLOBAL: u64 = 1u64 << (_PAGE_GLOBAL_SHIFT as u32);
pub const _PAGE_VALID: u64 = 1u64 << (_PAGE_VALID_SHIFT as u32);
pub const _PAGE_DIRTY: u64 = 1u64 << (_PAGE_DIRTY_SHIFT as u32);
#[cfg(CONFIG_CPU_R3K_TLB)]
pub const _CACHE_UNCACHED: u64 = 1u64 << (_CACHE_UNCACHED_SHIFT as u32);
#[cfg(CONFIG_CPU_R3K_TLB)]
pub const _CACHE_MASK: u64 = _CACHE_UNCACHED;
#[cfg(CONFIG_CPU_R3K_TLB)]
pub const PFN_PTE_SHIFT: u32 = PAGE_SHIFT;
#[cfg(not(CONFIG_CPU_R3K_TLB))]
pub const _CACHE_MASK: u64 = 7u64 << (_CACHE_SHIFT as u32);
#[cfg(not(CONFIG_CPU_R3K_TLB))]
pub const PFN_PTE_SHIFT: u32 = PAGE_SHIFT - 12 + _CACHE_SHIFT as u32 + 3;
#[cfg(not(any(CONFIG_XPA, CONFIG_CPU_HAS_RIXI)))]
pub const _PAGE_NO_EXEC: u64 = 0;

pub const _PAGE_SILENT_READ: u64 = _PAGE_VALID;
pub const _PAGE_SILENT_WRITE: u64 = _PAGE_DIRTY;
pub const _PFN_MASK: u64 = !((1u64 << PFN_PTE_SHIFT) - 1);

/*
 * The final layouts of the PTE bits are:
 *
 *   64-bit, R1 or earlier:     CCC D V G [S H] M A W R P
 *   32-bit, R1 or earlier:     CCC D V G M A W R P
 *   64-bit, R2 or later:       CCC D V G RI/R XI [S H] M A W P
 *   32-bit, R2 or later:       CCC D V G RI/R XI M A W P
 */

/*
 * pte_to_entrylo converts a page table entry (PTE) into a Mips
 * entrylo0/1 value.
 */
#[inline]
pub unsafe fn pte_to_entrylo(pte_val: libc::c_ulong) -> u64 {
    #[cfg(CONFIG_CPU_HAS_RIXI)]
    if cpu_has_rixi {
        #[cfg(CONFIG_32BIT)]
        let sa: u32 = 31 - _PAGE_NO_READ_SHIFT as u32;
        #[cfg(not(CONFIG_32BIT))]
        let sa: u32 = 63 - _PAGE_NO_READ_SHIFT as u32;
        return ((pte_val as u64) >> (_PAGE_GLOBAL_SHIFT as u32))
            | (((pte_val as u64) & (_PAGE_NO_EXEC | _PAGE_NO_READ)) << sa);
    }

    (pte_val as u64) >> (_PAGE_GLOBAL_SHIFT as u32)
}

/* Cache attributes */
#[cfg(CONFIG_CPU_R3K_TLB)]
pub const _CACHE_CACHABLE_NONCOHERENT: u64 = 0;
#[cfg(CONFIG_CPU_R3K_TLB)]
pub const _CACHE_UNCACHED_ACCELERATED: u64 = _CACHE_UNCACHED;
#[cfg(CONFIG_CPU_SB1)]
pub const _CACHE_CACHABLE_NONCOHERENT: u64 = 5u64 << (_CACHE_SHIFT as u32);

#[cfg(not(any(CONFIG_CPU_R3K_TLB, CONFIG_CPU_SB1)))]
pub const _CACHE_CACHABLE_NONCOHERENT: u64 = 3u64 << (_CACHE_SHIFT as u32);
pub const _CACHE_CACHABLE_NO_WA: u64 = 0u64 << (_CACHE_SHIFT as u32);
pub const _CACHE_CACHABLE_WA: u64 = 1u64 << (_CACHE_SHIFT as u32);
#[cfg(not(CONFIG_CPU_R3K_TLB))]
pub const _CACHE_UNCACHED: u64 = 2u64 << (_CACHE_SHIFT as u32);
pub const _CACHE_CACHABLE_CE: u64 = 4u64 << (_CACHE_SHIFT as u32);
pub const _CACHE_CACHABLE_COW: u64 = 5u64 << (_CACHE_SHIFT as u32);
pub const _CACHE_CACHABLE_CUW: u64 = 6u64 << (_CACHE_SHIFT as u32);
#[cfg(not(CONFIG_CPU_R3K_TLB))]
pub const _CACHE_UNCACHED_ACCELERATED: u64 = 7u64 << (_CACHE_SHIFT as u32);

pub const __READABLE: u64 = _PAGE_SILENT_READ | _PAGE_ACCESSED;
pub const __WRITEABLE: u64 = _PAGE_SILENT_WRITE | _PAGE_WRITE | _PAGE_MODIFIED;
pub const _PAGE_CHG_MASK: u64 = _PAGE_ACCESSED | _PAGE_MODIFIED |
    _PAGE_SOFT_DIRTY | _PFN_MASK | _CACHE_MASK | _PAGE_SPECIAL;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
