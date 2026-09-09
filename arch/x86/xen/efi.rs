// SPDX-License-Identifier: GPL-2.0
/*
 * Copyright (c) 2014 Oracle Co., Daniel Kiper
 */

// C dependencies are supplied by the surrounding kernel/Xen translation unit.

static mut VENDOR: [efi_char16_t; 100] = [0; 100];

static mut EFI_SYSTAB_XEN: efi_system_table_t = efi_system_table_t {
    hdr: efi_table_hdr_t {
        signature: EFI_SYSTEM_TABLE_SIGNATURE,
        revision: 0, // Initialized later.
        headersize: 0, // Ignored by Linux Kernel.
        crc32: 0, // Ignored by Linux Kernel.
        reserved: 0,
    },
    fw_vendor: EFI_INVALID_TABLE_ADDR, // Initialized later.
    fw_revision: 0, // Initialized later.
    con_in_handle: EFI_INVALID_TABLE_ADDR, // Not used under Xen.
    con_in: core::ptr::null_mut(), // Not used under Xen.
    con_out_handle: EFI_INVALID_TABLE_ADDR, // Not used under Xen.
    con_out: core::ptr::null_mut(), // Not used under Xen.
    stderr_handle: EFI_INVALID_TABLE_ADDR, // Not used under Xen.
    stderr: EFI_INVALID_TABLE_ADDR, // Not used under Xen.
    runtime: EFI_INVALID_TABLE_ADDR as *mut efi_runtime_services_t, // Not used under Xen.
    boottime: EFI_INVALID_TABLE_ADDR as *mut efi_boot_services_t, // Not used under Xen.
    nr_tables: 0, // Initialized later.
    tables: EFI_INVALID_TABLE_ADDR, // Initialized later.
};

unsafe fn xen_efi_probe() -> *mut efi_system_table_t {
    let mut op = xen_platform_op {
        cmd: XENPF_firmware_info,
        u: xen_platform_op_union {
            firmware_info: xenpf_firmware_info {
                type_: XEN_FW_EFI_INFO,
                index: XEN_FW_EFI_CONFIG_TABLE,
                u: core::mem::zeroed(),
            },
        },
    };
    let info = &mut (*(&mut op.u.firmware_info as *mut _)).u.efi_info;

    if !xen_initial_domain() || HYPERVISOR_platform_op(&mut op) < 0 {
        return core::ptr::null_mut();
    }

    // Here we know that Xen runs on EFI platform.
    xen_efi_runtime_setup();

    EFI_SYSTAB_XEN.tables = (*info).cfg.addr;
    EFI_SYSTAB_XEN.nr_tables = (*info).cfg.nent;

    op.cmd = XENPF_firmware_info;
    op.u.firmware_info.type_ = XEN_FW_EFI_INFO;
    op.u.firmware_info.index = XEN_FW_EFI_VENDOR;
    (*info).vendor.bufsz = core::mem::size_of_val(&VENDOR);
    set_xen_guest_handle((*info).vendor.name, VENDOR.as_mut_ptr());

    if HYPERVISOR_platform_op(&mut op) == 0 {
        EFI_SYSTAB_XEN.fw_vendor = __pa_symbol(VENDOR.as_ptr());
        EFI_SYSTAB_XEN.fw_revision = (*info).vendor.revision;
    } else {
        EFI_SYSTAB_XEN.fw_vendor = __pa_symbol(L"UNKNOWN".as_ptr());
    }

    op.cmd = XENPF_firmware_info;
    op.u.firmware_info.type_ = XEN_FW_EFI_INFO;
    op.u.firmware_info.index = XEN_FW_EFI_VERSION;

    if HYPERVISOR_platform_op(&mut op) == 0 {
        EFI_SYSTAB_XEN.hdr.revision = (*info).version;
    }

    op.cmd = XENPF_firmware_info;
    op.u.firmware_info.type_ = XEN_FW_EFI_INFO;
    op.u.firmware_info.index = XEN_FW_EFI_RT_VERSION;

    if HYPERVISOR_platform_op(&mut op) == 0 {
        efi.runtime_version = (*info).version;
    }

    &mut EFI_SYSTAB_XEN
}

/*
 * Determine whether we're in secure boot mode.
 */
unsafe fn xen_efi_get_secureboot() -> efi_secureboot_mode {
    static mut SHIM_GUID: efi_guid_t = EFI_SHIM_LOCK_GUID;
    let mut mode: efi_secureboot_mode;
    let status: efi_status_t;
    let mut moksbstate: u8 = 0;
    let mut size: usize;

    mode = efi_get_secureboot_mode(efi.get_variable);
    if mode == efi_secureboot_mode_unknown {
        pr_err!("Could not determine UEFI Secure Boot status.\n");
        return efi_secureboot_mode_unknown;
    }
    if mode != efi_secureboot_mode_enabled {
        return mode;
    }

    // See if a user has put the shim into insecure mode.
    size = core::mem::size_of_val(&moksbstate);
    status = (efi.get_variable)(
        L"MokSBStateRT".as_ptr(),
        &mut SHIM_GUID,
        core::ptr::null_mut(),
        &mut size,
        &mut moksbstate as *mut u8 as *mut core::ffi::c_void,
    );

    // If it fails, we don't care why. Default to secure.
    if status == EFI_SUCCESS && moksbstate == 1 {
        return efi_secureboot_mode_disabled;
    }

    pr_info!("UEFI Secure Boot is enabled.\n");
    efi_secureboot_mode_enabled
}

pub unsafe fn xen_efi_init(boot_params: *mut boot_params) {
    let efi_systab_xen = xen_efi_probe();

    if efi_systab_xen.is_null() {
        return;
    }

    strscpy(
        (*boot_params).efi_info.efi_loader_signature.as_mut_ptr() as *mut i8,
        b"Xen\0".as_ptr() as *const i8,
        core::mem::size_of_val(&(*boot_params).efi_info.efi_loader_signature),
    );
    (*boot_params).efi_info.efi_systab = __pa(efi_systab_xen) as u32;
    (*boot_params).efi_info.efi_systab_hi = (__pa(efi_systab_xen) >> 32) as u32;

    (*boot_params).secure_boot = xen_efi_get_secureboot();

    set_bit(EFI_BOOT, &mut efi.flags);
    set_bit(EFI_PARAVIRT, &mut efi.flags);
    set_bit(EFI_64BIT, &mut efi.flags);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
