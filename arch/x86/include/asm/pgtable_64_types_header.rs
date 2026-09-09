/* SPDX-License-Identifier: GPL-2.0 */

// Dependency supplied by the surrounding translation: asm/sparsemem.h

// These are used to make use of C type-checking.
pub type PtevalT = ::core::ffi::c_ulong;
pub type PmdvalT = ::core::ffi::c_ulong;
pub type PudvalT = ::core::ffi::c_ulong;
pub type P4dvalT = ::core::ffi::c_ulong;
pub type PgdvalT = ::core::ffi::c_ulong;
pub type PgprotvalT = ::core::ffi::c_ulong;

#[repr(C)]
pub struct PteT {
    pub pte: PtevalT,
}

#[repr(C)]
pub struct PmdT {
    pub pmd: PmdvalT,
}

pub static mut __pgtable_l5_enabled: ::core::ffi::c_uint = 0;

// Under USE_EARLY_PGTABLE_L5, cpu_feature_enabled() is unavailable in early
// boot code and the variable above is used instead. Otherwise this maps to
// cpu_feature_enabled(X86_FEATURE_LA57).
#[inline]
pub unsafe fn pgtable_l5_enabled() -> bool {
    __pgtable_l5_enabled != 0
}

pub const ARCH_PAGE_TABLE_SYNC_MASK: ::core::ffi::c_ulong =
    if cfg!(USE_EARLY_PGTABLE_L5) {
        // PGTBL_PGD_MODIFIED when the early-page-table configuration applies.
        PGTBL_PGD_MODIFIED
    } else {
        // The non-early configuration uses cpu_feature_enabled(X86_FEATURE_LA57)
        // to select between these values.
        PGTBL_P4D_MODIFIED
    };

pub static mut pgdir_shift: ::core::ffi::c_uint = 0;
pub static mut ptrs_per_p4d: ::core::ffi::c_uint = 0;

/* PGDIR_SHIFT determines what a top-level page table entry can map. */
pub const PTRS_PER_PGD: ::core::ffi::c_ulong = 512;

/* 4th level page in 5-level paging case. */
pub const P4D_SHIFT: u32 = 39;
pub const MAX_PTRS_PER_P4D: ::core::ffi::c_ulong = 512;
#[inline]
pub unsafe fn P4D_SIZE() -> ::core::ffi::c_ulong { 1u64 as ::core::ffi::c_ulong << P4D_SHIFT }
#[inline]
pub unsafe fn P4D_MASK() -> ::core::ffi::c_ulong { !(P4D_SIZE() - 1) }

/* 3rd level page. */
pub const PUD_SHIFT: u32 = 30;
pub const PTRS_PER_PUD: ::core::ffi::c_ulong = 512;

/* PMD_SHIFT determines the size of the area a middle-level page table can map. */
pub const PMD_SHIFT: u32 = 21;
pub const PTRS_PER_PMD: ::core::ffi::c_ulong = 512;

/* entries per page directory level */
pub const PTRS_PER_PTE: ::core::ffi::c_ulong = 512;

#[inline]
pub unsafe fn PMD_SIZE() -> ::core::ffi::c_ulong { 1u64 as ::core::ffi::c_ulong << PMD_SHIFT }
#[inline]
pub unsafe fn PMD_MASK() -> ::core::ffi::c_ulong { !(PMD_SIZE() - 1) }
#[inline]
pub unsafe fn PUD_SIZE() -> ::core::ffi::c_ulong { 1u64 as ::core::ffi::c_ulong << PUD_SHIFT }
#[inline]
pub unsafe fn PUD_MASK() -> ::core::ffi::c_ulong { !(PUD_SIZE() - 1) }
#[inline]
pub unsafe fn PGDIR_SHIFT() -> ::core::ffi::c_uint { pgdir_shift }
#[inline]
pub unsafe fn PGDIR_SIZE() -> ::core::ffi::c_ulong { 1u64 as ::core::ffi::c_ulong << PGDIR_SHIFT() }
#[inline]
pub unsafe fn PGDIR_MASK() -> ::core::ffi::c_ulong { !(PGDIR_SIZE() - 1) }

// Build-time symbols and configuration-dependent macros are preserved as
// expressions; their definitions are supplied by other translated headers.
pub const MAX_POSSIBLE_PHYSMEM_BITS: u32 = 52;
pub const MAXMEM: ::core::ffi::c_ulong = 1u64 as ::core::ffi::c_ulong << MAX_PHYSMEM_BITS;

pub const GUARD_HOLE_PGD_ENTRY: ::core::ffi::c_ulong = (-256i64) as ::core::ffi::c_ulong;
pub const GUARD_HOLE_SIZE: ::core::ffi::c_ulong = 16u64 as ::core::ffi::c_ulong << unsafe { pgdir_shift };
pub const GUARD_HOLE_BASE_ADDR: ::core::ffi::c_ulong = GUARD_HOLE_PGD_ENTRY << unsafe { pgdir_shift };
pub const GUARD_HOLE_END_ADDR: ::core::ffi::c_ulong = GUARD_HOLE_BASE_ADDR + GUARD_HOLE_SIZE;

pub const LDT_PGD_ENTRY: ::core::ffi::c_ulong = (-240i64) as ::core::ffi::c_ulong;
pub const LDT_BASE_ADDR: ::core::ffi::c_ulong = LDT_PGD_ENTRY << unsafe { pgdir_shift };
pub const LDT_END_ADDR: ::core::ffi::c_ulong = LDT_BASE_ADDR + (1u64 as ::core::ffi::c_ulong << unsafe { pgdir_shift });

pub const __VMALLOC_BASE_L4: ::core::ffi::c_ulong = 0xffffc90000000000;
pub const __VMALLOC_BASE_L5: ::core::ffi::c_ulong = 0xffa0000000000000;
pub const VMALLOC_SIZE_TB_L4: ::core::ffi::c_ulong = 32;
pub const VMALLOC_SIZE_TB_L5: ::core::ffi::c_ulong = 12800;
pub const __VMEMMAP_BASE_L4: ::core::ffi::c_ulong = 0xffffea0000000000;
pub const __VMEMMAP_BASE_L5: ::core::ffi::c_ulong = 0xffd4000000000000;

// VMALLOC_START = vmalloc_base; VMEMMAP_START = vmemmap_base.
// VMALLOC_SIZE_TB selects L5 or L4 according to pgtable_l5_enabled().
#[inline]
pub unsafe fn VMALLOC_SIZE_TB() -> ::core::ffi::c_ulong {
    if pgtable_l5_enabled() { VMALLOC_SIZE_TB_L5 } else { VMALLOC_SIZE_TB_L4 }
}
// VMALLOC_START is the external vmalloc_base symbol; VMEMMAP_START is the
// external vmemmap_base symbol.
// CONFIG_RANDOMIZE_MEMORY additionally defines DIRECT_MAP_PHYSMEM_END as
// direct_map_physmem_end.

// See the source header comments for the memory-map and KASLR constraints.
// VMEMORY_END = VMALLOC_START + (VMALLOC_SIZE_TB << 40) - 1.
// CONFIG_KMSAN changes VMALLOC_END and defines the KMSAN metadata offsets and
// starts as described by the original conditional block.

// MODULES_VADDR = __START_KERNEL_map + KERNEL_IMAGE_SIZE.
// MODULES_END is 0xffffffffff000000, or 0xfffffffffe000000 when
// CONFIG_DEBUG_KMAP_LOCAL_FORCE_MAP is enabled.
// MODULES_LEN = MODULES_END - MODULES_VADDR.

pub const EARLY_DYNAMIC_PAGE_TABLES: ::core::ffi::c_ulong = 64;

pub const ESPFIX_PGD_ENTRY: ::core::ffi::c_ulong = (-2i64) as ::core::ffi::c_ulong;
pub const ESPFIX_BASE_ADDR: ::core::ffi::c_ulong = ESPFIX_PGD_ENTRY << P4D_SHIFT;
pub const CPU_ENTRY_AREA_PGD: ::core::ffi::c_ulong = (-4i64) as ::core::ffi::c_ulong;
pub const CPU_ENTRY_AREA_BASE: ::core::ffi::c_ulong = CPU_ENTRY_AREA_PGD << P4D_SHIFT;

pub const EFI_VA_START: i64 = -4 * (1i64 << 30);
pub const EFI_VA_END: i64 = -68 * (1i64 << 30);
pub const PGD_KERNEL_START: usize = (PAGE_SIZE / 2) / ::core::mem::size_of::<PgdT>();

// We borrow bit 3 to remember PG_anon_exclusive.
pub const _PAGE_SWP_EXCLUSIVE: ::core::ffi::c_ulong = _PAGE_PWT;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
