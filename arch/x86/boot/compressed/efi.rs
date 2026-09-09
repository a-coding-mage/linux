// SPDX-License-Identifier: GPL-2.0
/*
 * Helpers for early access to EFI configuration table.
 *
 * Originally derived from arch/x86/boot/compressed/acpi.c
 */

// Dependencies supplied by the surrounding translation unit.

/**
 * efi_get_type - Given a pointer to boot_params, determine the type of EFI environment.
 *
 * @bp:         pointer to boot_params
 *
 * Return: EFI_TYPE_{32,64} for valid EFI environments, EFI_TYPE_NONE otherwise.
 */
pub unsafe fn efi_get_type(bp: *mut boot_params) -> efi_type {
    let ei: *mut efi_info = &mut (*bp).efi_info;
    let sig: *const core::ffi::c_char = &(*ei).efi_loader_signature as *const _ as *const core::ffi::c_char;

    let et;
    if strncmp(sig, EFI64_LOADER_SIGNATURE, 4) == 0 {
        et = EFI_TYPE_64;
    } else if strncmp(sig, EFI32_LOADER_SIGNATURE, 4) == 0 {
        et = EFI_TYPE_32;
    } else {
        debug_putstr("No EFI environment detected.\0".as_ptr() as *const core::ffi::c_char);
        et = EFI_TYPE_NONE;
    }

    // On non-x86_64 builds, preserve the CONFIG_X86_64 conditional behavior.
    #[cfg(not(target_arch = "x86_64"))]
    {
        /*
         * Existing callers like acpi.c treat this case as an indicator to
         * fall-through to non-EFI, rather than an error, so maintain that
         * functionality here as well.
         */
        if (*ei).efi_systab_hi != 0 || (*ei).efi_memmap_hi != 0 {
            debug_putstr("EFI system table is located above 4GB and cannot be accessed.\0".as_ptr() as *const core::ffi::c_char);
            return EFI_TYPE_NONE;
        }
    }

    et
}

/**
 * efi_get_system_table - Given a pointer to boot_params, retrieve the physical address
 *                        of the EFI system table.
 *
 * @bp:         pointer to boot_params
 *
 * Return: EFI system table address on success. On error, return 0.
 */
pub unsafe fn efi_get_system_table(bp: *mut boot_params) -> c_ulong {
    let ei: *mut efi_info = &mut (*bp).efi_info;
    // Get systab from boot params.
    let sys_tbl_pa: c_ulong = if cfg!(target_arch = "x86_64") {
        ((*ei).efi_systab as u64 | ((*ei).efi_systab_hi as u64) << 32) as c_ulong
    } else {
        (*ei).efi_systab as c_ulong
    };
    if sys_tbl_pa == 0 {
        debug_putstr("EFI system table not found.\0".as_ptr() as *const core::ffi::c_char);
        return 0;
    }
    sys_tbl_pa
}

/*
 * EFI config table address changes to virtual address after boot, which may
 * not be accessible for the kexec'd kernel. To address this, kexec provides
 * the initial physical address via a struct setup_data entry, which is
 * checked for here, along with some sanity checks.
 */
unsafe fn get_kexec_setup_data(bp: *mut boot_params, _et: efi_type) -> *mut efi_setup_data {
    #[cfg(target_arch = "x86_64")]
    {
        let mut esd: *mut efi_setup_data = core::ptr::null_mut();
        let mut pa_data = (*bp).hdr.setup_data as u64;
        while pa_data != 0 {
            let data = pa_data as *mut setup_data;
            if (*data).type_ == SETUP_EFI {
                esd = (pa_data + core::mem::size_of::<setup_data>() as u64) as *mut efi_setup_data;
                break;
            }
            pa_data = (*data).next;
        }
        if !esd.is_null() && (*esd).tables == 0 {
            debug_putstr("kexec EFI environment missing valid configuration table.\0".as_ptr() as *const core::ffi::c_char);
            return core::ptr::null_mut();
        }
        return esd;
    }
    core::ptr::null_mut()
}

/**
 * efi_get_conf_table - Given a pointer to boot_params, locate and return the physical
 *                      address of EFI configuration table.
 */
pub unsafe fn efi_get_conf_table(bp: *mut boot_params, cfg_tbl_pa: *mut c_ulong, cfg_tbl_len: *mut c_uint) -> c_int {
    if cfg_tbl_pa.is_null() || cfg_tbl_len.is_null() { return -EINVAL; }
    let sys_tbl_pa = efi_get_system_table(bp);
    if sys_tbl_pa == 0 { return -EINVAL; }
    let et = efi_get_type(bp);
    if et == EFI_TYPE_64 {
        let stbl = sys_tbl_pa as *mut efi_system_table_64_t;
        let esd = get_kexec_setup_data(bp, et);
        *cfg_tbl_pa = if !esd.is_null() { (*esd).tables as c_ulong } else { (*stbl).tables as c_ulong };
        *cfg_tbl_len = (*stbl).nr_tables;
    } else if et == EFI_TYPE_32 {
        let stbl = sys_tbl_pa as *mut efi_system_table_32_t;
        *cfg_tbl_pa = (*stbl).tables as c_ulong;
        *cfg_tbl_len = (*stbl).nr_tables;
    } else { return -EINVAL; }
    0
}

/* Get vendor table address/guid from EFI config table at the given index */
unsafe fn get_vendor_table(cfg_tbl: *mut core::ffi::c_void, idx: c_uint, vendor_tbl_pa: *mut c_ulong, vendor_tbl_guid: *mut efi_guid_t, et: efi_type) -> c_int {
    if et == EFI_TYPE_64 {
        let entry = (cfg_tbl as *mut efi_config_table_64_t).add(idx as usize);
        if cfg!(not(target_arch = "x86_64")) && (*entry).table >> 32 != 0 { return -EINVAL; }
        *vendor_tbl_pa = (*entry).table as c_ulong;
        *vendor_tbl_guid = (*entry).guid;
    } else if et == EFI_TYPE_32 {
        let entry = (cfg_tbl as *mut efi_config_table_32_t).add(idx as usize);
        *vendor_tbl_pa = (*entry).table as c_ulong;
        *vendor_tbl_guid = (*entry).guid;
    } else { return -EINVAL; }
    0
}

pub unsafe fn efi_find_vendor_table(bp: *mut boot_params, cfg_tbl_pa: c_ulong, cfg_tbl_len: c_uint, guid: efi_guid_t) -> c_ulong {
    let et = efi_get_type(bp);
    if et == EFI_TYPE_NONE { return 0; }
    for i in 0..cfg_tbl_len {
        let mut vendor_tbl_pa = 0;
        let mut vendor_tbl_guid: efi_guid_t = core::mem::zeroed();
        if get_vendor_table(cfg_tbl_pa as *mut core::ffi::c_void, i, &mut vendor_tbl_pa, &mut vendor_tbl_guid, et) != 0 { return 0; }
        if efi_guidcmp(guid, vendor_tbl_guid) == 0 { return vendor_tbl_pa; }
    }
    0
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
