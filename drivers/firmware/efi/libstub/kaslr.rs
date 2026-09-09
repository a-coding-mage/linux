// SPDX-License-Identifier: GPL-2.0
/*
 * Helper functions used by the EFI stub on multiple
 * architectures to deal with physical address space randomization.
 */

// Dependencies supplied by the surrounding EFI stub implementation.

/// efi_kaslr_get_phys_seed() - Get random seed for physical kernel KASLR
/// @image_handle: Handle to the image
//
// If KASLR is not disabled, obtain a random seed using EFI_RNG_PROTOCOL
// that will be used to move the kernel physical mapping.
//
// Return: the random seed
pub unsafe fn efi_kaslr_get_phys_seed(image_handle: efi_handle_t) -> u32 {
    let li_fixed_proto: efi_guid_t = LINUX_EFI_LOADED_IMAGE_FIXED_GUID;
    let mut p: *mut core::ffi::c_void = core::ptr::null_mut();

    if !IS_ENABLED(CONFIG_RANDOMIZE_BASE) {
        return 0;
    }

    if efi_nokaslr {
        efi_info!("KASLR disabled on kernel command line\n");
    } else if efi_bs_call!(handle_protocol, image_handle, &li_fixed_proto, &mut p)
        == EFI_SUCCESS
    {
        efi_info!("Image placement fixed by loader\n");
    } else {
        let status: efi_status_t;
        let mut phys_seed: u32 = 0;

        status = efi_get_random_bytes(
            core::mem::size_of::<u32>(),
            &mut phys_seed as *mut u32 as *mut u8,
        );
        if status == EFI_SUCCESS {
            return phys_seed;
        }

        if status == EFI_NOT_FOUND {
            efi_info!("EFI_RNG_PROTOCOL unavailable\n");
        } else {
            efi_err!("efi_get_random_bytes() failed (0x%lx)\n", status);
        }

        efi_nokaslr = true;
    }

    0
}

/*
 * Distro versions of GRUB may ignore the BSS allocation entirely (i.e., fail
 * to provide space, and fail to zero it). Check for this condition by double
 * checking that the first and the last byte of the image are covered by the
 * same EFI memory map entry.
 */
unsafe fn check_image_region(base: u64, size: u64) -> bool {
    let mut map: *mut efi_boot_memmap = core::ptr::null_mut();
    let status: efi_status_t;
    let mut ret = false;
    let mut map_offset: i32 = 0;

    status = efi_get_memory_map(&mut map, false);
    if status != EFI_SUCCESS {
        return false;
    }

    while map_offset < (*map).map_size {
        let md = ((*map).map as *mut u8).add(map_offset as usize)
            as *mut efi_memory_desc_t;
        let end = (*md).phys_addr + (*md).num_pages * EFI_PAGE_SIZE;

        /*
         * Find the region that covers base, and return whether
         * it covers base+size bytes.
         */
        if base >= (*md).phys_addr && base < end {
            ret = base + size <= end;
            break;
        }

        map_offset += (*map).desc_size;
    }

    ret
}

/// efi_kaslr_relocate_kernel() - Relocate the kernel (random if KASLR enabled)
/// @image_addr: Pointer to the current kernel location
/// @reserve_addr: Pointer to the relocated kernel location
/// @reserve_size: Size of the relocated kernel
/// @kernel_size: Size of the text + data
/// @kernel_codesize: Size of the text
/// @kernel_memsize: Size of the text + data + bss
/// @phys_seed: Random seed used for the relocation
//
// If KASLR is not enabled, this function relocates the kernel to a fixed
// address (or leave it as its current location). If KASLR is enabled, the
// kernel physical location is randomized using the seed in parameter.
//
// Return: status code, EFI_SUCCESS if relocation is successful
pub unsafe fn efi_kaslr_relocate_kernel(
    image_addr: *mut c_ulong,
    reserve_addr: *mut c_ulong,
    reserve_size: *mut c_ulong,
    kernel_size: c_ulong,
    kernel_codesize: c_ulong,
    kernel_memsize: c_ulong,
    phys_seed: u32,
) -> efi_status_t {
    let mut status: efi_status_t;
    let min_kimg_align: u64 = efi_get_kimg_min_align();

    if IS_ENABLED(CONFIG_RANDOMIZE_BASE) && phys_seed != 0 {
        /*
         * If KASLR is enabled, and we have some randomness available,
         * locate the kernel at a randomized offset in physical memory.
         */
        status = efi_random_alloc(
            *reserve_size,
            min_kimg_align,
            reserve_addr,
            phys_seed,
            EFI_LOADER_CODE,
            0,
            EFI_ALLOC_LIMIT,
        );
        if status != EFI_SUCCESS {
            efi_warn!("efi_random_alloc() failed: 0x%lx\n", status);
        }
    } else {
        status = EFI_OUT_OF_RESOURCES;
    }

    if status != EFI_SUCCESS {
        if !check_image_region(*image_addr as u64, kernel_memsize as u64) {
            efi_err!("FIRMWARE BUG: Image BSS overlaps adjacent EFI memory region\n");
        } else if IS_ALIGNED!(*image_addr, min_kimg_align)
            && (*_end as c_ulong) < EFI_ALLOC_LIMIT
        {
            /*
             * Just execute from wherever we were loaded by the
             * UEFI PE/COFF loader if the placement is suitable.
             */
            *reserve_size = 0;
            return EFI_SUCCESS;
        }

        status = efi_allocate_pages_aligned(
            *reserve_size,
            reserve_addr,
            ULONG_MAX,
            min_kimg_align,
            EFI_LOADER_CODE,
        );

        if status != EFI_SUCCESS {
            efi_err!("Failed to relocate kernel\n");
            *reserve_size = 0;
            return status;
        }
    }

    core::ptr::copy_nonoverlapping(
        *image_addr as *const u8,
        *reserve_addr as *mut u8,
        kernel_size as usize,
    );
    *image_addr = *reserve_addr;
    efi_icache_sync(*image_addr, *image_addr + kernel_codesize);
    efi_remap_image(*image_addr, *reserve_size, kernel_codesize);

    status
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
