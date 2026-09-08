// SPDX-License-Identifier: GPL-2.0
/*
 * Machine keyring routines.
 *
 * Copyright (c) 2021, Oracle and/or its affiliates.
 */

// Dependencies from <linux/efi.h> and "../integrity.h" are expected to be
// supplied by the surrounding translated kernel tree.

pub type size_t = usize;
pub type key_perm_t = u32;

#[repr(C)]
pub struct efi_mokvar_table_entry {
    _private: [u8; 0],
}

extern "C" {
    fn integrity_init_keyring(id: i32) -> i32;
    fn integrity_load_cert(
        id: i32,
        source: *const core::ffi::c_char,
        data: *const core::ffi::c_void,
        len: size_t,
        perm: key_perm_t,
    ) -> i32;
    fn efi_enabled(feature: i32) -> bool;
    fn efi_mokvar_entry_find(name: *const core::ffi::c_char) -> *mut efi_mokvar_table_entry;
    fn pr_notice(fmt: *const core::ffi::c_char, ...);
    fn pr_info(fmt: *const core::ffi::c_char, ...);
}

// Constants/macros provided by the surrounding kernel translation:
// INTEGRITY_KEYRING_MACHINE, INTEGRITY_KEYRING_PLATFORM, KEY_POS_ALL,
// KEY_POS_SETATTR, KEY_USR_VIEW, EFI_BOOT, CONFIG_INTEGRITY_PLATFORM_KEYRING,
// IS_ENABLED(), __init, device_initcall().

unsafe fn machine_keyring_init() -> i32 {
    let rc: i32;

    rc = integrity_init_keyring(INTEGRITY_KEYRING_MACHINE);
    if rc != 0 {
        return rc;
    }

    pr_notice(c"Machine keyring initialized\n".as_ptr());
    return 0;
}

// device_initcall(machine_keyring_init);

pub unsafe fn add_to_machine_keyring(
    source: *const core::ffi::c_char,
    data: *const core::ffi::c_void,
    len: size_t,
) {
    let perm: key_perm_t;
    let mut rc: i32;

    perm = (KEY_POS_ALL & !KEY_POS_SETATTR) | KEY_USR_VIEW;
    rc = integrity_load_cert(INTEGRITY_KEYRING_MACHINE, source, data, len, perm);

    /*
     * Some MOKList keys may not pass the machine keyring restrictions.
     * If the restriction check does not pass and the platform keyring
     * is configured, try to add it into that keyring instead.
     */
    if rc != 0 && efi_enabled(EFI_BOOT) && IS_ENABLED(CONFIG_INTEGRITY_PLATFORM_KEYRING) {
        rc = integrity_load_cert(INTEGRITY_KEYRING_PLATFORM, source, data, len, perm);
    }

    if rc != 0 {
        pr_info(c"Error adding keys to machine keyring %s\n".as_ptr(), source);
    }
}

/*
 * Try to load the MokListTrustedRT MOK variable to see if we should trust
 * the MOK keys within the kernel. It is not an error if this variable
 * does not exist.  If it does not exist, MOK keys should not be trusted
 * within the machine keyring.
 */
unsafe fn uefi_check_trust_mok_keys() -> bool {
    let mokvar_entry: *mut efi_mokvar_table_entry;

    mokvar_entry = efi_mokvar_entry_find(c"MokListTrustedRT".as_ptr());

    if !mokvar_entry.is_null() {
        return true;
    }

    return false;
}

unsafe fn trust_moklist() -> bool {
    static mut INITIALIZED: bool = false;
    static mut TRUST_MOK: bool = false;

    if !INITIALIZED {
        INITIALIZED = true;
        TRUST_MOK = false;

        if uefi_check_trust_mok_keys() {
            TRUST_MOK = true;
        }
    }

    return TRUST_MOK;
}

/*
 * Provides platform specific check for trusting imputed keys before loading
 * on .machine keyring. UEFI systems enable this trust based on a variable,
 * and for other platforms, it is always enabled.
 */
pub unsafe fn imputed_trust_enabled() -> bool {
    if efi_enabled(EFI_BOOT) {
        return trust_moklist();
    }

    return true;
}


// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
