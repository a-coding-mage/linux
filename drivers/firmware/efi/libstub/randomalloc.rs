// SPDX-License-Identifier: GPL-2.0
/*
 * Copyright (C) 2016 Linaro Ltd;  <ard.biesheuvel@linaro.org>
 */

/* Dependencies are supplied by the surrounding EFI and kernel environment. */

/*
 * Return the number of slots covered by this entry, i.e., the number of
 * addresses it covers that are suitably aligned and supply enough room
 * for the allocation.
 */
unsafe fn get_entry_num_slots(
    md: *mut efi_memory_desc_t,
    size: c_ulong,
    align_shift: c_ulong,
    alloc_min: u64,
    alloc_max: u64,
) -> c_ulong {
    let align: c_ulong = 1 << align_shift;
    let first_slot: u64;
    let last_slot: u64;
    let region_end: u64;

    if (*md).type_ != EFI_CONVENTIONAL_MEMORY {
        return 0;
    }

    if (*md).attribute & EFI_MEMORY_HOT_PLUGGABLE != 0 {
        return 0;
    }

    if efi_soft_reserve_enabled() && ((*md).attribute & EFI_MEMORY_SP != 0) {
        return 0;
    }

    region_end = core::cmp::min(
        (*md).phys_addr + (*md).num_pages * EFI_PAGE_SIZE - 1,
        alloc_max,
    );
    if region_end < size as u64 {
        return 0;
    }

    first_slot = round_up(core::cmp::max((*md).phys_addr, alloc_min), align);
    last_slot = round_down(region_end - size as u64 + 1, align);

    if first_slot > last_slot {
        return 0;
    }

    (((last_slot - first_slot) as c_ulong) >> align_shift) + 1
}

pub unsafe fn efi_random_alloc(
    size: c_ulong,
    mut align: c_ulong,
    addr: *mut c_ulong,
    random_seed: c_ulong,
    memory_type: c_int,
    mut alloc_min: c_ulong,
    alloc_max: c_ulong,
) -> efi_status_t {
    // C source: struct efi_boot_memmap *map __free(efi_pool) = NULL;
    let mut map: *mut efi_boot_memmap = core::ptr::null_mut();
    let mut total_slots: c_ulong = 0;
    let mut target_slot: c_ulong;
    let mut total_mirrored_slots: c_ulong = 0;
    let mut status: efi_status_t;
    let mut map_offset: c_int;

    status = efi_get_memory_map(&mut map, false);
    if status != EFI_SUCCESS {
        return status;
    }

    if align < EFI_ALLOC_ALIGN {
        align = EFI_ALLOC_ALIGN;
    }

    /* Avoid address 0x0, as it can be mistaken for NULL */
    if alloc_min == 0 {
        alloc_min = align;
    }

    let size = round_up(size, EFI_ALLOC_ALIGN);

    /* count the suitable slots in each memory map entry */
    map_offset = 0;
    while map_offset < (*map).map_size {
        let md = ((*map).map as *mut u8).add(map_offset as usize)
            as *mut efi_memory_desc_t;
        let slots = get_entry_num_slots(md, size, ilog2(align), alloc_min as u64, alloc_max as u64);

        (*md).virt_addr = slots as u64;
        total_slots += slots;
        if (*md).attribute & EFI_MEMORY_MORE_RELIABLE != 0 {
            total_mirrored_slots += slots;
        }
        map_offset += (*map).desc_size;
    }

    /* consider only mirrored slots for randomization if any exist */
    if total_mirrored_slots > 0 {
        total_slots = total_mirrored_slots;
    }

    /* find a random number between 0 and total_slots */
    target_slot = ((total_slots as u64 * (random_seed & U32_MAX as c_ulong) as u64) >> 32)
        as c_ulong;

    /*
     * target_slot is now a value in the range [0, total_slots), and so
     * it corresponds with exactly one of the suitable slots we recorded
     * when iterating over the memory map the first time around.
     *
     * So iterate over the memory map again, subtracting the number of
     * slots of each entry at each iteration, until we have found the entry
     * that covers our chosen slot. Use the residual value of target_slot
     * to calculate the randomly chosen address, and allocate it directly
     * using EFI_ALLOCATE_ADDRESS.
     */
    status = EFI_OUT_OF_RESOURCES;
    map_offset = 0;
    while map_offset < (*map).map_size {
        let md = ((*map).map as *mut u8).add(map_offset as usize)
            as *mut efi_memory_desc_t;
        let mut target: efi_physical_addr_t;
        let pages: c_ulong;

        if total_mirrored_slots > 0 && (*md).attribute & EFI_MEMORY_MORE_RELIABLE == 0 {
            map_offset += (*map).desc_size;
            continue;
        }

        let slots = (*md).virt_addr as c_ulong;
        if target_slot >= slots {
            target_slot -= slots;
            map_offset += (*map).desc_size;
            continue;
        }

        target = round_up(core::cmp::max((*md).phys_addr, alloc_min as u64), align as u64)
            + target_slot as u64 * align as u64;
        pages = size / EFI_PAGE_SIZE;

        status = efi_bs_call(allocate_pages, EFI_ALLOCATE_ADDRESS, memory_type, pages, &mut target);
        if status == EFI_SUCCESS {
            *addr = target as c_ulong;
        }
        break;
    }

    status
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
