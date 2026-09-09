// SPDX-License-Identifier: GPL-2.0-only
/*
 * Copyright (C) 2020 Western Digital Corporation or its affiliates.
 * Adapted from arch/arm64/kernel/efi.c
 */

// Dependencies supplied by the surrounding kernel translation.

/*
 * Only regions of type EFI_RUNTIME_SERVICES_CODE need to be
 * executable, everything else can be mapped with the XN bits
 * set. Also take the new (optional) RO/XP bits into account.
 */
unsafe fn efimem_to_pgprot_map(md: *mut efi_memory_desc_t) -> pgprot_t {
    let attr: u64 = (*md).attribute;
    let type_: u32 = (*md).type_;

    if type_ == EFI_MEMORY_MAPPED_IO {
        return PAGE_KERNEL;
    }

    /* R-- */
    if (attr & (EFI_MEMORY_XP | EFI_MEMORY_RO)) == (EFI_MEMORY_XP | EFI_MEMORY_RO) {
        return PAGE_KERNEL_READ;
    }

    /* R-X */
    if (attr & EFI_MEMORY_RO) != 0 {
        return PAGE_KERNEL_READ_EXEC;
    }

    /* RW- */
    if ((attr & (EFI_MEMORY_RP | EFI_MEMORY_WP | EFI_MEMORY_XP)) == EFI_MEMORY_XP
        || type_ != EFI_RUNTIME_SERVICES_CODE)
    {
        return PAGE_KERNEL;
    }

    /* RWX */
    PAGE_KERNEL_EXEC
}

unsafe fn efi_create_mapping(mm: *mut mm_struct, md: *mut efi_memory_desc_t) -> i32 {
    let prot: pgprot_t = __pgprot(pgprot_val(efimem_to_pgprot_map(md)) & !(_PAGE_GLOBAL));
    let mut i = 0;

    /* RISC-V maps one page at a time */
    while i < (*md).num_pages {
        create_pgd_mapping(
            (*mm).pgd,
            (*md).virt_addr + i * PAGE_SIZE,
            (*md).phys_addr + i * PAGE_SIZE,
            PAGE_SIZE,
            prot,
        );
        i += 1;
    }
    0
}

unsafe fn set_permissions(ptep: *mut pte_t, _addr: c_ulong, data: *mut c_void) -> i32 {
    let md: *mut efi_memory_desc_t = data as *mut efi_memory_desc_t;
    let mut pte: pte_t = ptep_get(ptep);
    let mut val: c_ulong;

    if (*md).attribute & EFI_MEMORY_RO != 0 {
        val = pte_val(pte) & !(_PAGE_WRITE);
        val |= _PAGE_READ;
        pte = __pte(val);
    }
    if (*md).attribute & EFI_MEMORY_XP != 0 {
        val = pte_val(pte) & !(_PAGE_EXEC);
        pte = __pte(val);
    }
    set_pte(ptep, pte);

    0
}

unsafe fn efi_set_mapping_permissions(
    mm: *mut mm_struct,
    md: *mut efi_memory_desc_t,
    _ignored: bool,
) -> i32 {
    BUG_ON(
        (*md).type_ != EFI_RUNTIME_SERVICES_CODE
            && (*md).type_ != EFI_RUNTIME_SERVICES_DATA,
    );

    /*
     * Calling apply_to_page_range() is only safe on regions that are
     * guaranteed to be mapped down to pages. Since we are only called
     * for regions that have been mapped using efi_create_mapping() above
     * (and this is checked by the generic Memory Attributes table parsing
     * routines), there is no need to check that again here.
     */
    apply_to_page_range(
        mm,
        (*md).virt_addr,
        (*md).num_pages << EFI_PAGE_SHIFT,
        set_permissions,
        md as *mut c_void,
    )
}

fn efi_poweroff_required() -> bool {
    efi_enabled(EFI_RUNTIME_SERVICES)
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
