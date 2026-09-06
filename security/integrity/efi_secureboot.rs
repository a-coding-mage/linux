// SPDX-License-Identifier: GPL-1.0+
/*
 * Copyright (C) 2018 IBM Corporation
 */

// Conditional compilation: if arch_efi_boot_mode is not defined elsewhere,
// use efi_secureboot_mode_unset as the default
#[allow(non_upper_case_globals)]
const arch_efi_boot_mode: efi_secureboot_mode = efi_secureboot_mode::efi_secureboot_mode_unset;

extern "C" {
    fn efi_rt_services_supported(services: u32) -> bool;
    fn efi_get_secureboot_mode(get_variable: *const ()) -> efi_secureboot_mode;
    fn efi_enabled(flag: u32) -> bool;
    fn pr_info(fmt: *const u8, ...);

    static efi: efi_t;
}

#[repr(C)]
pub struct efi_t {
    pub get_variable: *const (),
}

#[repr(C)]
#[derive(Clone, Copy, PartialEq, Eq)]
pub enum efi_secureboot_mode {
    efi_secureboot_mode_unset,
    efi_secureboot_mode_disabled,
    efi_secureboot_mode_unknown,
    efi_secureboot_mode_enabled,
}

const EFI_RT_SUPPORTED_GET_VARIABLE: u32 = 1 << 0;
const EFI_BOOT: u32 = 1 << 0;

fn get_sb_mode() -> efi_secureboot_mode {
    let mode: efi_secureboot_mode;

    unsafe {
        if !efi_rt_services_supported(EFI_RT_SUPPORTED_GET_VARIABLE) {
            pr_info(
                "integrity: secureboot mode unknown, no efi\n\0".as_ptr(),
            );
            return efi_secureboot_mode::efi_secureboot_mode_unknown;
        }

        mode = efi_get_secureboot_mode(efi.get_variable);
        if mode == efi_secureboot_mode::efi_secureboot_mode_disabled {
            pr_info(
                "integrity: secureboot mode disabled\n\0".as_ptr(),
            );
        } else if mode == efi_secureboot_mode::efi_secureboot_mode_unknown {
            pr_info(
                "integrity: secureboot mode unknown\n\0".as_ptr(),
            );
        } else {
            pr_info(
                "integrity: secureboot mode enabled\n\0".as_ptr(),
            );
        }
    }
    mode
}

/*
 * Query secure boot status
 *
 * Note don't call this function too early e.g. in __setup hook otherwise the
 * kernel may hang when calling efi_get_secureboot_mode.
 *
 */
pub fn arch_get_secureboot() -> bool {
    static mut sb_mode: efi_secureboot_mode = efi_secureboot_mode::efi_secureboot_mode_unset;
    static mut initialized: bool = false;

    unsafe {
        if !initialized && efi_enabled(EFI_BOOT) {
            sb_mode = arch_efi_boot_mode;

            if sb_mode == efi_secureboot_mode::efi_secureboot_mode_unset {
                sb_mode = get_sb_mode();
            }
            initialized = true;
        }

        if sb_mode == efi_secureboot_mode::efi_secureboot_mode_enabled {
            true
        } else {
            false
        }
    }
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
