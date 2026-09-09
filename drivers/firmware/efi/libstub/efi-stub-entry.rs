// SPDX-License-Identifier: GPL-2.0-only

// Dependencies supplied by the surrounding EFI stub and kernel interfaces.

static mut KERNEL_IMAGE_OFFSET: ::core::primitive::c_ulong = 0;

unsafe fn kernel_image_addr(addr: *mut ::core::ffi::c_void) -> *mut ::core::ffi::c_void {
    (addr as *mut u8).add(KERNEL_IMAGE_OFFSET as usize) as *mut ::core::ffi::c_void
}

pub unsafe fn alloc_primary_display() -> *mut sysfb_display_info {
    // Equivalent of IS_ENABLED(CONFIG_ARM).
    #[cfg(CONFIG_ARM)]
    {
        return __alloc_primary_display();
    }

    // Equivalent of IS_ENABLED(CONFIG_X86) ||
    // IS_ENABLED(CONFIG_EFI_EARLYCON) || IS_ENABLED(CONFIG_SYSFB).
    #[cfg(any(CONFIG_X86, CONFIG_EFI_EARLYCON, CONFIG_SYSFB))]
    {
        return kernel_image_addr(
            &raw mut sysfb_primary_display as *mut sysfb_display_info
                as *mut ::core::ffi::c_void,
        ) as *mut sysfb_display_info;
    }

    ::core::ptr::null_mut()
}

/*
 * EFI entry point for the generic EFI stub used by ARM, arm64, RISC-V and
 * LoongArch. This is the entrypoint that is described in the PE/COFF header
 * of the core kernel.
 */
pub unsafe extern "efiapi" fn efi_pe_entry(
    handle: efi_handle_t,
    systab: *mut efi_system_table_t,
) -> efi_status_t {
    let mut image: *mut efi_loaded_image_t = ::core::ptr::null_mut();
    let mut status: efi_status_t;
    let mut image_addr: ::core::primitive::c_ulong;
    let mut image_size: ::core::primitive::c_ulong = 0;
    /* addr/point and size pairs for memory management*/
    let mut cmdline_ptr: *mut ::core::ffi::c_char = ::core::ptr::null_mut();
    let mut loaded_image_proto: efi_guid_t = LOADED_IMAGE_PROTOCOL_GUID;
    let mut reserve_addr: ::core::primitive::c_ulong = 0;
    let mut reserve_size: ::core::primitive::c_ulong = 0;

    // WRITE_ONCE(efi_system_table, systab)
    ::core::ptr::write_volatile(&raw mut efi_system_table, systab);

    /* Check if we were booted by the EFI firmware */
    if (*efi_system_table).hdr.signature != EFI_SYSTEM_TABLE_SIGNATURE {
        return EFI_INVALID_PARAMETER;
    }

    /*
     * Get a handle to the loaded image protocol.  This is used to get
     * information about the running image, such as size and the command
     * line.
     */
    status = efi_bs_call(
        handle_protocol,
        handle,
        &mut loaded_image_proto,
        &mut image as *mut *mut efi_loaded_image_t as *mut ::core::ffi::c_void,
    );
    if status != EFI_SUCCESS {
        efi_err("Failed to get loaded image protocol\n");
        return status;
    }

    status = efi_handle_cmdline(image, &mut cmdline_ptr);
    if status != EFI_SUCCESS {
        return status;
    }

    efi_info("Booting Linux Kernel...\n");

    status = handle_kernel_image(
        &mut image_addr,
        &mut image_size,
        &mut reserve_addr,
        &mut reserve_size,
        image,
        handle,
    );
    if status != EFI_SUCCESS {
        efi_err("Failed to relocate kernel\n");
        return status;
    }

    KERNEL_IMAGE_OFFSET = image_addr - (*image).image_base as ::core::primitive::c_ulong;

    status = efi_stub_common(handle, image, image_addr, cmdline_ptr);

    efi_free(image_size, image_addr);
    efi_free(reserve_size, reserve_addr);

    status
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
