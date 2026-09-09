// SPDX-License-Identifier: GPL-2.0-only
/*
 * Based on arch/arm/mm/mmap.c
 *
 * Copyright (C) 2012 ARM Ltd.
 */

// Linux kernel dependencies supplied by other translation units.

static mut protection_map: [pgprot_t; 16] = [
    PAGE_NONE,
    PAGE_READONLY,
    PAGE_READONLY,
    PAGE_READONLY,
    PAGE_READONLY_EXEC,
    PAGE_READONLY_EXEC,
    PAGE_READONLY_EXEC,
    PAGE_READONLY_EXEC,
    PAGE_NONE,
    PAGE_READONLY,
    PAGE_SHARED,
    PAGE_SHARED,
    PAGE_READONLY_EXEC,
    PAGE_READONLY_EXEC,
    PAGE_SHARED_EXEC,
    PAGE_SHARED_EXEC,
];

static mut gcs_page_prot: ptval_t = _PAGE_GCS_RO;

extern "C" {
    fn memblock_is_region_memory(addr: phys_addr_t, size: usize) -> bool;
    fn memblock_is_map_memory(addr: phys_addr_t) -> bool;
    fn cpus_have_cap(cap: i32) -> bool;
    fn lpa2_is_enabled() -> bool;
    fn system_supports_gcs() -> bool;
    fn system_supports_poe() -> bool;
}

/*
 * You really shouldn't be using read() or write() on /dev/mem.  This might go
 * away in the future.
 */
pub unsafe fn valid_phys_addr_range(addr: phys_addr_t, size: usize) -> i32 {
    /*
     * Check whether addr is covered by a memory region without the
     * MEMBLOCK_NOMAP attribute, and whether that region covers the
     * entire range. In theory, this could lead to false negatives
     * if the range is covered by distinct but adjacent memory regions
     * that only differ in other attributes. However, few of such
     * attributes have been defined, and it is debatable whether it
     * follows that /dev/mem read() calls should be able traverse
     * such boundaries.
     */
    (memblock_is_region_memory(addr, size) && memblock_is_map_memory(addr)) as i32
}

/*
 * Do not allow /dev/mem mappings beyond the supported physical range.
 */
pub unsafe fn valid_mmap_phys_addr_range(pfn: c_ulong, size: usize) -> i32 {
    (!(((pfn << PAGE_SHIFT) + size as c_ulong) & !PHYS_MASK) != 0) as i32
}

unsafe fn adjust_protection_map() -> i32 {
    /*
     * With Enhanced PAN we can honour the execute-only permissions as
     * there is no PAN override with such mappings.
     */
    if cpus_have_cap(ARM64_HAS_EPAN) {
        protection_map[VM_EXEC as usize] = PAGE_EXECONLY;
        protection_map[(VM_EXEC | VM_SHARED) as usize] = PAGE_EXECONLY;
    }

    if lpa2_is_enabled() {
        let mut i = 0;
        while i < protection_map.len() {
            pgprot_val(&mut protection_map[i]) &= !PTE_SHARED;
            i += 1;
        }
        gcs_page_prot &= !PTE_SHARED;
    }

    0
}

// arch_initcall(adjust_protection_map);

pub unsafe fn vm_get_page_prot(vm_flags: vm_flags_t) -> pgprot_t {
    let prot: ptval_t;

    /* Short circuit GCS to avoid bloating the table. */
    if system_supports_gcs() && (vm_flags & VM_SHADOW_STACK) != 0 {
        /* Honour mprotect(PROT_NONE) on shadow stack mappings */
        if (vm_flags & VM_ACCESS_FLAGS) != 0 {
            prot = gcs_page_prot;
        } else {
            prot = pgprot_val(&protection_map[VM_NONE as usize]);
        }
    } else {
        prot = pgprot_val(&protection_map[(vm_flags &
            (VM_READ | VM_WRITE | VM_EXEC | VM_SHARED)) as usize]);
    }

    if (vm_flags & VM_ARM64_BTI) != 0 {
        prot |= PTE_GP;
    }

    /*
     * There are two conditions required for returning a Normal Tagged
     * memory type: (1) the user requested it via PROT_MTE passed to
     * mmap() or mprotect() and (2) the corresponding vma supports MTE. We
     * register (1) as VM_MTE in the vma->vm_flags and (2) as
     * VM_MTE_ALLOWED. Note that the latter can only be set during the
     * mmap() call since mprotect() does not accept MAP_* flags.
     * Checking for VM_MTE only is sufficient since arch_validate_flags()
     * does not permit (VM_MTE & !VM_MTE_ALLOWED).
     */
    if (vm_flags & VM_MTE) != 0 {
        prot |= PTE_ATTRINDX(MT_NORMAL_TAGGED);
    }

    // #ifdef CONFIG_ARCH_HAS_PKEYS
    if system_supports_poe() {
        if (vm_flags & VM_PKEY_BIT0) != 0 {
            prot |= PTE_PO_IDX_0;
        }
        if (vm_flags & VM_PKEY_BIT1) != 0 {
            prot |= PTE_PO_IDX_1;
        }
        if (vm_flags & VM_PKEY_BIT2) != 0 {
            prot |= PTE_PO_IDX_2;
        }
    }
    // #endif

    __pgprot(prot)
}

// EXPORT_SYMBOL(vm_get_page_prot);

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
