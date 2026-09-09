// SPDX-License-Identifier: GPL-2.0

// Dependencies supplied by the surrounding EFI/kernel translation unit.

unsafe fn alloc_preferred_address(alloc_size: c_ulong) -> c_ulong {
    // EFI_KIMG_PREFERRED_ADDRESS is a build-time configuration constant.
    #[cfg(EFI_KIMG_PREFERRED_ADDRESS)]
    {
        let mut efi_addr: efi_physical_addr_t = EFI_KIMG_PREFERRED_ADDRESS;

        if efi_bs_call(
            allocate_pages,
            EFI_ALLOCATE_ADDRESS,
            EFI_LOADER_DATA,
            alloc_size / EFI_PAGE_SIZE,
            &mut efi_addr,
        ) == EFI_SUCCESS
        {
            return efi_addr;
        }
    }
    ULONG_MAX
}

pub unsafe fn efi_cache_sync_image(_image_base: c_ulong, _alloc_size: c_ulong) {
    // Provided by the arch to perform the cache maintenance necessary for
    // executable code loaded into memory to be safe for execution.
}

pub unsafe fn alloc_primary_display() -> *mut sysfb_display_info {
    __alloc_primary_display()
}

pub unsafe extern "C" fn efi_zboot_entry(
    handle: efi_handle_t,
    systab: *mut efi_system_table_t,
) -> efi_status_t {
    // C's __free(efi_pool) cleanup annotation is represented by the owning
    // EFI cleanup convention of the surrounding translation unit.
    let mut cmdline_ptr: *mut c_char = core::ptr::null_mut();
    let mut image_base: c_ulong;
    let mut alloc_size: c_ulong = 0;
    let mut image: *mut efi_loaded_image_t = core::ptr::null_mut();
    let mut status: efi_status_t;

    WRITE_ONCE(efi_system_table, systab);

    status = efi_bs_call(
        handle_protocol,
        handle,
        &LOADED_IMAGE_PROTOCOL_GUID,
        &mut image as *mut *mut efi_loaded_image_t as *mut *mut c_void,
    );
    if status != EFI_SUCCESS {
        efi_err("Failed to locate parent's loaded image protocol\n");
        return status;
    }

    status = efi_handle_cmdline(image, &mut cmdline_ptr);
    if status != EFI_SUCCESS {
        return status;
    }

    efi_info("Decompressing Linux Kernel...\n");

    status = efi_zboot_decompress_init(&mut alloc_size);
    if status != EFI_SUCCESS {
        return status;
    }

    // If the architecture has a preferred address for the image,
    // try that first.
    image_base = alloc_preferred_address(alloc_size);
    let image_base = if image_base == ULONG_MAX {
        let min_kimg_align: c_ulong = efi_get_kimg_min_align();
        let mut seed: u32 = U32_MAX;

        if !IS_ENABLED(CONFIG_RANDOMIZE_BASE) {
            // Setting the random seed to 0x0 is the same as
            // allocating as low as possible
            seed = 0;
        } else if efi_nokaslr {
            efi_info("KASLR disabled on kernel command line\n");
        } else {
            status = efi_get_random_bytes(
                core::mem::size_of::<u32>(),
                &mut seed as *mut u32 as *mut u8,
            );
            if status == EFI_NOT_FOUND {
                efi_info("EFI_RNG_PROTOCOL unavailable\n");
                efi_nokaslr = true;
            } else if status != EFI_SUCCESS {
                efi_err("efi_get_random_bytes() failed (0x%lx)\n", status);
                efi_nokaslr = true;
            }
        }

        status = efi_random_alloc(
            alloc_size,
            min_kimg_align,
            &mut image_base as *mut c_ulong,
            seed,
            EFI_LOADER_CODE,
            0,
            EFI_ALLOC_LIMIT,
        );
        if status != EFI_SUCCESS {
            efi_err("Failed to allocate memory\n");
            return status;
        }
        image_base
    } else {
        image_base
    };

    // Decompress the payload into the newly allocated buffer
    let decompression_status = efi_zboot_decompress(image_base as *mut c_void, alloc_size);
    status = if decompression_status != 0 {
        decompression_status
    } else {
        efi_stub_common(handle, image, image_base, cmdline_ptr)
    };

    efi_free(alloc_size, image_base);
    status
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
