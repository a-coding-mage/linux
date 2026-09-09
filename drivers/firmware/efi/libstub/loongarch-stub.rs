// SPDX-License-Identifier: GPL-2.0
/*
 * Author: Yun Liu <liuyun@loongson.cn>
 *         Huacai Chen <chenhuacai@loongson.cn>
 * Copyright (C) 2020-2022 Loongson Technology Corporation Limited
 */

// Dependencies supplied by the surrounding EFI stub and LoongArch headers.

extern "C" {
    static mut kernel_asize: ::core::ffi::c_int;
    static mut kernel_fsize: ::core::ffi::c_int;
    static mut kernel_entry: ::core::ffi::c_int;
}

/**
 * efi_relocate_kernel() - copy memory area
 * @image_addr:        pointer to address of memory area to copy
 * @image_size:        size of memory area to copy
 * @alloc_size:        minimum size of memory to allocate, must be greater or
 *                     equal to image_size
 * @preferred_addr:    preferred target address
 * @alignment:         minimum alignment of the allocated memory area. It
 *                     should be a power of two.
 * @min_addr:          minimum target address
 *
 * Copy a memory area to a newly allocated memory area aligned according
 * to @alignment but at least EFI_ALLOC_ALIGN. If the preferred address
 * is not available, the allocated address will not be below @min_addr.
 * On exit, @image_addr is updated to the target copy address that was used.
 *
 * This function is used to copy the Linux kernel verbatim. It does not apply
 * any relocation changes.
 *
 * Return:             status code
 */
unsafe fn efi_relocate_kernel(
    image_addr: *mut ::core::ffi::c_ulong,
    image_size: ::core::ffi::c_ulong,
    alloc_size: ::core::ffi::c_ulong,
    preferred_addr: ::core::ffi::c_ulong,
    alignment: ::core::ffi::c_ulong,
    min_addr: ::core::ffi::c_ulong,
) -> efi_status_t {
    let cur_image_addr: ::core::ffi::c_ulong;
    let mut new_addr: ::core::ffi::c_ulong = 0;
    let status: efi_status_t;
    let nr_pages: ::core::ffi::c_ulong;
    let mut efi_addr: efi_physical_addr_t = preferred_addr;

    if image_addr.is_null() || image_size == 0 || alloc_size == 0 {
        return EFI_INVALID_PARAMETER;
    }
    if alloc_size < image_size {
        return EFI_INVALID_PARAMETER;
    }

    cur_image_addr = *image_addr;

    /*
     * The EFI firmware loader could have placed the kernel image
     * anywhere in memory, but the kernel has restrictions on the
     * max physical address it can run at.  Some architectures
     * also have a preferred address, so first try to relocate
     * to the preferred address.  If that fails, allocate as low
     * as possible while respecting the required alignment.
     */
    nr_pages = round_up(alloc_size, EFI_ALLOC_ALIGN) / EFI_PAGE_SIZE;
    status = efi_bs_call!(allocate_pages, EFI_ALLOCATE_ADDRESS,
                          EFI_LOADER_DATA, nr_pages, &mut efi_addr);
    new_addr = efi_addr;
    /*
     * If preferred address allocation failed allocate as low as
     * possible.
     */
    if status != EFI_SUCCESS {
        status = efi_low_alloc_above(alloc_size, alignment, &mut new_addr,
                                     min_addr);
    }
    if status != EFI_SUCCESS {
        efi_err!("Failed to allocate usable memory for kernel.\n");
        return status;
    }

    /*
     * We know source/dest won't overlap since both memory ranges
     * have been allocated by UEFI, so we can safely use memcpy.
     */
    memcpy(new_addr as *mut ::core::ffi::c_void,
           cur_image_addr as *const ::core::ffi::c_void, image_size);
    efi_cache_sync_image(new_addr, image_size);

    /* Return the new address of the relocated image. */
    *image_addr = new_addr;

    status
}

unsafe fn handle_kernel_image(
    image_addr: *mut ::core::ffi::c_ulong,
    image_size: *mut ::core::ffi::c_ulong,
    reserve_addr: *mut ::core::ffi::c_ulong,
    reserve_size: *mut ::core::ffi::c_ulong,
    image: *mut efi_loaded_image_t,
    image_handle: efi_handle_t,
) -> efi_status_t {
    let status: efi_status_t;
    let mut kernel_addr: ::core::ffi::c_ulong = 0;

    kernel_addr = (*image).image_base as ::core::ffi::c_ulong;

    status = efi_relocate_kernel(&mut kernel_addr, kernel_fsize as _, kernel_asize as _,
                                 EFI_KIMG_PREFERRED_ADDRESS, efi_get_kimg_min_align(), 0x0);

    *image_addr = kernel_addr;
    *image_size = kernel_asize as _;

    status
}

unsafe fn kernel_entry_address(
    kernel_addr: ::core::ffi::c_ulong,
    image: *mut efi_loaded_image_t,
) -> ::core::ffi::c_ulong {
    let base: ::core::ffi::c_ulong = (*image).image_base as ::core::ffi::c_ulong;

    (&kernel_entry as *const _ as ::core::ffi::c_ulong) - base + kernel_addr
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
