// SPDX-License-Identifier: GPL-2.0
/*
 * Secure boot handling.
 *
 * Copyright (C) 2013,2014 Linaro Limited
 *     Roy Franz <roy.franz@linaro.org
 * Copyright (C) 2013 Red Hat, Inc.
 *     Mark Salter <msalter@redhat.com>
 */

// Dependencies supplied by the surrounding EFI stub environment:
// linux/efi.h, asm/efi.h, and efistub.h.

/* SHIM variables */
static const shim_guid: efi_guid_t = EFI_SHIM_LOCK_GUID;
static const shim_MokSBState_name: [efi_char16_t; 13] = [
    'M' as efi_char16_t,
    'o' as efi_char16_t,
    'k' as efi_char16_t,
    'S' as efi_char16_t,
    'B' as efi_char16_t,
    'S' as efi_char16_t,
    't' as efi_char16_t,
    'a' as efi_char16_t,
    't' as efi_char16_t,
    'e' as efi_char16_t,
    'R' as efi_char16_t,
    'T' as efi_char16_t,
    0,
];

unsafe extern "C" {
    fn get_efi_var(
        name: *mut efi_char16_t,
        vendor: *mut efi_guid_t,
        attr: *mut u32,
        data_size: *mut core::ffi::c_ulong,
        data: *mut core::ffi::c_void,
    ) -> efi_status_t;

    fn efi_get_secureboot_mode(
        get_var: unsafe fn(
            *mut efi_char16_t,
            *mut efi_guid_t,
            *mut u32,
            *mut core::ffi::c_ulong,
            *mut core::ffi::c_void,
        ) -> efi_status_t,
    ) -> efi_secureboot_mode;

    fn efi_err(format: *const core::ffi::c_char, ...);
    fn efi_info(format: *const core::ffi::c_char, ...);
}

type efi_char16_t = u16;
type efi_status_t = usize;
type efi_guid_t = [u8; 16];

#[derive(Clone, Copy)]
#[allow(non_camel_case_types)]
#[repr(C)]
enum efi_secureboot_mode {
    efi_secureboot_mode_unknown,
    efi_secureboot_mode_disabled,
    efi_secureboot_mode_enabled,
}

const EFI_SHIM_LOCK_GUID: efi_guid_t = [0; 16];
const EFI_SUCCESS: efi_status_t = 0;
const EFI_VARIABLE_NON_VOLATILE: u32 = 0x00000001;

unsafe fn get_var(
    name: *mut efi_char16_t,
    vendor: *mut efi_guid_t,
    attr: *mut u32,
    data_size: *mut core::ffi::c_ulong,
    data: *mut core::ffi::c_void,
) -> efi_status_t {
    get_efi_var(name, vendor, attr, data_size, data)
}

/*
 * Determine whether we're in secure boot mode.
 */
pub unsafe fn efi_getsecureboot() -> efi_secureboot_mode {
    let mut attr: u32;
    let mut size: core::ffi::c_ulong;
    let mode: efi_secureboot_mode;
    let status: efi_status_t;
    let mut moksbstate: u8;

    mode = efi_get_secureboot_mode(get_var);
    if matches!(mode, efi_secureboot_mode::efi_secureboot_mode_unknown) {
        efi_err(c"Could not determine UEFI Secure Boot status.\n".as_ptr());
        return efi_secureboot_mode::efi_secureboot_mode_unknown;
    }
    if !matches!(mode, efi_secureboot_mode::efi_secureboot_mode_enabled) {
        return mode;
    }

    /*
     * See if a user has put the shim into insecure mode. If so, and if the
     * variable doesn't have the non-volatile attribute set, we might as
     * well honor that.
     */
    size = core::mem::size_of::<u8>() as core::ffi::c_ulong;
    status = get_efi_var(
        shim_MokSBState_name.as_ptr() as *mut efi_char16_t,
        &shim_guid as *const efi_guid_t as *mut efi_guid_t,
        &mut attr,
        &mut size,
        &mut moksbstate as *mut u8 as *mut core::ffi::c_void,
    );

    /* If it fails, we don't care why. Default to secure */
    if status != EFI_SUCCESS {
        efi_info(c"UEFI Secure Boot is enabled.\n".as_ptr());
        return efi_secureboot_mode::efi_secureboot_mode_enabled;
    }
    if (attr & EFI_VARIABLE_NON_VOLATILE) == 0 && moksbstate == 1 {
        return efi_secureboot_mode::efi_secureboot_mode_disabled;
    }

    efi_info(c"UEFI Secure Boot is enabled.\n".as_ptr());
    efi_secureboot_mode::efi_secureboot_mode_enabled
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
