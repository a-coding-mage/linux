// SPDX-License-Identifier: GPL-2.0
/*
 * Common EFI memory map functions.
 */

// C dependency intent: declarations supplied by Linux EFI, memory mapping,
// initialization, I/O, memblock, slab, and architecture-specific headers.

/**
 * __efi_memmap_init - Common code for mapping the EFI memory map
 * @data: EFI memory map data
 *
 * This function takes care of figuring out which function to use to
 * map the EFI memory map in efi.memmap based on how far into the boot
 * we are.
 *
 * During bootup EFI_MEMMAP_LATE in data->flags should be clear since we
 * only have access to the early_memremap*() functions as the vmalloc
 * space isn't setup.  Once the kernel is fully booted we can fallback
 * to the more robust memremap*() API.
 *
 * Returns: zero on success, a negative error code on failure.
 */
pub unsafe fn __efi_memmap_init(data: *mut efi_memory_map_data) -> i32 {
    let mut map: efi_memory_map = core::mem::zeroed();
    let phys_map: phys_addr_t = (*data).phys_map;

    if ((*data).flags & EFI_MEMMAP_LATE) != 0 {
        map.map = memremap(phys_map, (*data).size, MEMREMAP_WB);
    } else {
        map.map = early_memremap(phys_map, (*data).size);
    }

    if map.map.is_null() {
        pr_err!("Could not map the memory map! phys_map=%pa, size=0x%lx\n", &phys_map, (*data).size);
        return -ENOMEM;
    }

    map.phys_map = (*data).phys_map;
    map.nr_map = (*data).size / (*data).desc_size;
    map.map_end = map.map.add((*data).size as usize);

    map.desc_version = (*data).desc_version;
    map.desc_size = (*data).desc_size;
    map.flags = (*data).flags;

    set_bit(EFI_MEMMAP, &mut efi.flags);

    efi.memmap = map;

    0
}

/**
 * efi_memmap_init_early - Map the EFI memory map data structure
 * @data: EFI memory map data
 *
 * Use early_memremap() to map the passed in EFI memory map and assign
 * it to efi.memmap.
 *
 * Returns: zero on success, a negative error code on failure.
 */
pub unsafe fn efi_memmap_init_early(data: *mut efi_memory_map_data) -> i32 {
    /* Cannot go backwards */
    WARN_ON!((efi.memmap.flags & EFI_MEMMAP_LATE) != 0);

    (*data).flags = 0;
    __efi_memmap_init(data)
}

pub unsafe fn efi_memmap_unmap() {
    if !efi_enabled(EFI_MEMMAP) {
        return;
    }

    if (efi.memmap.flags & EFI_MEMMAP_LATE) == 0 {
        let size: usize;

        size = (efi.memmap.desc_size * efi.memmap.nr_map) as usize;
        early_memunmap(efi.memmap.map, size);
    } else {
        memunmap(efi.memmap.map);
    }

    efi.memmap.map = core::ptr::null_mut();
    clear_bit(EFI_MEMMAP, &mut efi.flags);
}

/**
 * efi_memmap_init_late - Map efi.memmap with memremap()
 * @addr: Physical address of the new EFI memory map
 * @size: Size in bytes of the new EFI memory map
 *
 * Setup a mapping of the EFI memory map using ioremap_cache(). This
 * function should only be called once the vmalloc space has been
 * setup and is therefore not suitable for calling during early EFI
 * initialise, e.g. in efi_init(). Additionally, it expects
 * efi_memmap_init_early() to have already been called.
 *
 * The reason there are two EFI memmap initialisation
 * (efi_memmap_init_early() and this late version) is because the
 * early EFI memmap should be explicitly unmapped once EFI
 * initialisation is complete as the fixmap space used to map the EFI
 * memmap (via early_memremap()) is a scarce resource.
 *
 * This late mapping is intended to persist for the duration of
 * runtime so that things like efi_mem_desc_lookup() and
 * efi_mem_attributes() always work.
 *
 * Returns: zero on success, a negative error code on failure.
 */
pub unsafe fn efi_memmap_init_late(addr: phys_addr_t, size: usize) -> i32 {
    let mut data: efi_memory_map_data = efi_memory_map_data {
        phys_map: addr,
        size,
        flags: EFI_MEMMAP_LATE,
        ..core::mem::zeroed()
    };

    /* Did we forget to unmap the early EFI memmap? */
    WARN_ON!(!efi.memmap.map.is_null());

    /* Were we already called? */
    WARN_ON!((efi.memmap.flags & EFI_MEMMAP_LATE) != 0);

    /*
     * It makes no sense to allow callers to register different
     * values for the following fields. Copy them out of the
     * existing early EFI memmap.
     */
    data.desc_version = efi.memmap.desc_version;
    data.desc_size = efi.memmap.desc_size;

    __efi_memmap_init(&mut data)
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
