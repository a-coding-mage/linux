// SPDX-License-Identifier: GPL-2.0

// Dependencies supplied by the surrounding EFI stub and kernel sources.

/*
 * There are two ways of populating the core kernel's sysfb_primary_display
 * via the stub:
 *
 *   - using a configuration table, which relies on the EFI init code to
 *     locate the table and copy the contents; or
 *
 *   - by linking directly to the core kernel's copy of the global symbol.
 *
 * The latter is preferred because it makes the EFIFB earlycon available very
 * early, but it only works if the EFI stub is part of the core kernel image
 * itself. The zboot decompressor can only use the configuration table
 * approach.
 */

static mut primary_display_guid: efi_guid_t = LINUX_EFI_PRIMARY_DISPLAY_TABLE_GUID;

pub unsafe fn __alloc_primary_display() -> *mut sysfb_display_info {
    let mut dpy: *mut sysfb_display_info = core::ptr::null_mut();
    let mut status: efi_status_t;

    status = efi_bs_call!(
        allocate_pool,
        EFI_ACPI_RECLAIM_MEMORY,
        core::mem::size_of::<sysfb_display_info>(),
        &mut dpy as *mut *mut sysfb_display_info as *mut *mut core::ffi::c_void
    );

    if status != EFI_SUCCESS {
        return core::ptr::null_mut();
    }

    core::ptr::write_bytes(
        dpy as *mut u8,
        0,
        core::mem::size_of::<sysfb_display_info>(),
    );

    status = efi_bs_call!(
        install_configuration_table,
        &raw mut primary_display_guid,
        dpy,
    );
    if status == EFI_SUCCESS {
        return dpy;
    }

    efi_bs_call!(free_pool, dpy);
    core::ptr::null_mut()
}

pub unsafe fn free_primary_display(dpy: *mut sysfb_display_info) {
    if dpy.is_null() {
        return;
    }

    efi_bs_call!(install_configuration_table, &raw mut primary_display_guid, core::ptr::null_mut());
    efi_bs_call!(free_pool, dpy);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
