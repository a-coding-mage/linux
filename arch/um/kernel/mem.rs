// SPDX-License-Identifier: GPL-2.0
/*
 * Copyright (C) 2000 - 2007 Jeff Dike (jdike@{addtoit,linux.intel}.com)
 */

// Declarations and constants supplied by the kernel and UML dependencies.

#[cfg(CONFIG_KASAN)]
#[no_mangle]
pub unsafe extern "C" fn kasan_init() {
    /*
     * kasan_map_memory will map all of the required address space and
     * the host machine will allocate physical memory as necessary.
     */
    kasan_map_memory(KASAN_SHADOW_START as *mut core::ffi::c_void, KASAN_SHADOW_SIZE);
    init_task.kasan_depth = 0;
    /*
     * Since kasan_init() is called before main(),
     * KASAN is initialized but the enablement is deferred after
     * jump_label_init(). See arch_mm_preinit().
     */
}

#[cfg(CONFIG_KASAN)]
#[used]
#[link_section = ".kasan_init"]
static mut KASAN_INIT_PTR: Option<unsafe extern "C" fn()> = Some(kasan_init);

/*
 * Initialized during boot, and readonly for initializing page tables
 * afterwards
 */
#[no_mangle]
pub static mut swapper_pg_dir: [pgd_t; PTRS_PER_PGD] = [unsafe { core::mem::zeroed() }; PTRS_PER_PGD];

/* Initialized at boot time, and readonly after that */
#[no_mangle]
pub static mut kmalloc_ok: core::ffi::c_int = 0;

/* Used during early boot */
static mut brk_end: core::ffi::c_ulong = 0;

pub unsafe extern "C" fn arch_mm_preinit() {
    /* Safe to call after jump_label_init(). Enables KASAN. */
    kasan_init_generic();

    /* Map in the area just after the brk now that kmalloc is about
     * to be turned on.
     */
    brk_end = PAGE_ALIGN(sbrk(0) as core::ffi::c_ulong);
    map_memory(
        brk_end,
        __pa(brk_end),
        uml_reserved - brk_end,
        1,
        1,
        0,
    );
    memblock_free(brk_end as *mut core::ffi::c_void, uml_reserved - brk_end);
    uml_reserved = brk_end;
    min_low_pfn = PFN_UP(__pa(uml_reserved));
    max_pfn = max_low_pfn;
}

pub unsafe extern "C" fn mem_init() {
    kmalloc_ok = 1;
}

pub unsafe extern "C" fn arch_zone_limits_init(max_zone_pfns: *mut core::ffi::c_ulong) {
    *max_zone_pfns.add(ZONE_NORMAL as usize) = high_physmem >> PAGE_SHIFT;
}

/*
 * This can't do anything because nothing in the kernel image can be freed
 * since it's not in kernel physical memory.
 */

pub unsafe extern "C" fn free_initmem() {}

/* Allocate and free page tables. */

pub unsafe extern "C" fn pgd_alloc(mm: *mut mm_struct) -> *mut pgd_t {
    let pgd = __pgd_alloc(mm, 0);

    if !pgd.is_null() {
        memcpy(
            pgd.add(USER_PTRS_PER_PGD as usize),
            swapper_pg_dir.as_ptr().add(USER_PTRS_PER_PGD as usize),
            ((PTRS_PER_PGD - USER_PTRS_PER_PGD) as usize) * core::mem::size_of::<pgd_t>(),
        );
    }

    pgd
}

pub unsafe extern "C" fn uml_kmalloc(
    size: core::ffi::c_int,
    flags: core::ffi::c_int,
) -> *mut core::ffi::c_void {
    kmalloc(size, flags)
}

static protection_map: [pgprot_t; 16] = [
    PAGE_NONE,
    PAGE_READONLY,
    PAGE_COPY,
    PAGE_COPY,
    PAGE_READONLY,
    PAGE_READONLY,
    PAGE_COPY,
    PAGE_COPY,
    PAGE_NONE,
    PAGE_READONLY,
    PAGE_SHARED,
    PAGE_SHARED,
    PAGE_READONLY,
    PAGE_READONLY,
    PAGE_SHARED,
    PAGE_SHARED,
];

DECLARE_VM_GET_PAGE_PROT

pub unsafe extern "C" fn mark_rodata_ro() {
    let rodata_start = PFN_ALIGN(__start_rodata);
    let rodata_end = PFN_ALIGN(__end_rodata);

    os_protect_memory(
        rodata_start as *mut core::ffi::c_void,
        rodata_end - rodata_start,
        1,
        0,
        0,
    );
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
