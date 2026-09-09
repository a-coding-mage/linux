// SPDX-License-Identifier: GPL-2.0

use core::ffi::c_void;

// Declarations supplied by the surrounding kernel translation.
extern "C" {
    static mut pgd_cachep: *mut kmem_cache;
    #[cfg(feature = "pagetable_levels_gt_2")]
    static mut pmd_cachep: *mut kmem_cache;

    static mut swapper_pg_dir: *mut pgd_t;

    fn memset(s: *mut c_void, c: i32, n: usize) -> *mut c_void;
    fn memcpy(dest: *mut c_void, src: *const c_void, n: usize) -> *mut c_void;
    fn kmem_cache_create(
        name: *const i8,
        size: usize,
        align: usize,
        flags: u32,
        ctor: Option<unsafe extern "C" fn(*mut c_void)>,
    ) -> *mut kmem_cache;
    fn kmem_cache_alloc(cachep: *mut kmem_cache, flags: u32) -> *mut c_void;
    fn kmem_cache_free(cachep: *mut kmem_cache, objp: *mut c_void);
    fn set_pud(pud: *mut pud_t, p: pud_t);
    fn __pud(value: usize) -> pud_t;
}

// Types and constants are supplied by the surrounding kernel translation.
#[repr(C)]
pub struct kmem_cache {
    _private: [u8; 0],
}

pub type pgd_t = usize;
pub type pud_t = usize;
pub type pmd_t = usize;

#[repr(C)]
pub struct mm_struct {
    _private: [u8; 0],
}

extern "C" {
    static PTRS_PER_PGD: usize;
    static USER_PTRS_PER_PGD: usize;
    static PTRS_PER_PMD: usize;
    static PTE_MAGNITUDE: usize;
    static PAGE_SIZE: usize;
    static SLAB_PANIC: u32;
    static GFP_KERNEL: u32;
    static __GFP_ZERO: u32;
}

unsafe extern "C" fn pgd_ctor(x: *mut c_void) {
    let pgd = x as *mut pgd_t;

    memset(
        pgd as *mut c_void,
        0,
        USER_PTRS_PER_PGD * core::mem::size_of::<pgd_t>(),
    );
    memcpy(
        pgd.add(USER_PTRS_PER_PGD) as *mut c_void,
        swapper_pg_dir.add(USER_PTRS_PER_PGD) as *const c_void,
        (PTRS_PER_PGD - USER_PTRS_PER_PGD) * core::mem::size_of::<pgd_t>(),
    );
}

pub unsafe extern "C" fn pgtable_cache_init() {
    pgd_cachep = kmem_cache_create(
        b"pgd_cache\0".as_ptr() as *const i8,
        PTRS_PER_PGD * (1usize << PTE_MAGNITUDE),
        PAGE_SIZE,
        SLAB_PANIC,
        Some(pgd_ctor),
    );

    // Equivalent of: #if PAGETABLE_LEVELS > 2
    #[cfg(feature = "pagetable_levels_gt_2")]
    {
        pmd_cachep = kmem_cache_create(
            b"pmd_cache\0".as_ptr() as *const i8,
            PTRS_PER_PMD * (1usize << PTE_MAGNITUDE),
            PAGE_SIZE,
            SLAB_PANIC,
            None,
        );
    }
}

pub unsafe extern "C" fn pgd_alloc(_mm: *mut mm_struct) -> *mut pgd_t {
    kmem_cache_alloc(pgd_cachep, GFP_KERNEL) as *mut pgd_t
}

pub unsafe extern "C" fn pgd_free(_mm: *mut mm_struct, pgd: *mut pgd_t) {
    kmem_cache_free(pgd_cachep, pgd as *mut c_void);
}

// Equivalent of: #if PAGETABLE_LEVELS > 2
#[cfg(feature = "pagetable_levels_gt_2")]
pub unsafe extern "C" fn pud_populate(
    _mm: *mut mm_struct,
    pud: *mut pud_t,
    pmd: *mut pmd_t,
) {
    set_pud(pud, __pud(pmd as usize));
}

#[cfg(feature = "pagetable_levels_gt_2")]
pub unsafe extern "C" fn pmd_alloc_one(
    _mm: *mut mm_struct,
    _address: usize,
) -> *mut pmd_t {
    kmem_cache_alloc(pmd_cachep, GFP_KERNEL | __GFP_ZERO) as *mut pmd_t
}

#[cfg(feature = "pagetable_levels_gt_2")]
pub unsafe extern "C" fn pmd_free(_mm: *mut mm_struct, pmd: *mut pmd_t) {
    kmem_cache_free(pmd_cachep, pmd as *mut c_void);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
