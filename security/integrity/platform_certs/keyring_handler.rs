// SPDX-License-Identifier: GPL-2.0

use core::ffi::{c_char, c_int, c_void};

pub type size_t = usize;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct efi_guid_t {
    pub b: [u8; 16],
}

pub type efi_element_handler_t =
    Option<unsafe extern "C" fn(source: *const c_char, data: *const c_void, len: size_t)>;

pub const BLACKLIST_HASH_X509_TBS: c_int = 0;
pub const BLACKLIST_HASH_BINARY: c_int = 1;

extern "C" {
    static EFI_CERT_X509_GUID: efi_guid_t;
    static EFI_CERT_X509_SHA256_GUID: efi_guid_t;
    static EFI_CERT_SHA256_GUID: efi_guid_t;

    fn efi_guidcmp(left: efi_guid_t, right: efi_guid_t) -> c_int;
    fn mark_hash_blacklisted(data: *const c_void, len: size_t, hash_type: c_int);
    fn add_key_to_revocation_list(data: *const c_void, len: size_t);
    fn add_to_platform_keyring(source: *const c_char, data: *const c_void, len: size_t);
    fn add_to_machine_keyring(source: *const c_char, data: *const c_void, len: size_t);
    fn add_to_secondary_keyring(source: *const c_char, data: *const c_void, len: size_t);
    fn imputed_trust_enabled() -> bool;
}

// C __initdata statics initialized from EFI_CERT_*_GUID.
static mut efi_cert_x509_guid: efi_guid_t = unsafe { EFI_CERT_X509_GUID };
static mut efi_cert_x509_sha256_guid: efi_guid_t = unsafe { EFI_CERT_X509_SHA256_GUID };
static mut efi_cert_sha256_guid: efi_guid_t = unsafe { EFI_CERT_SHA256_GUID };

/*
 * Blacklist an X509 TBS hash.
 */
unsafe extern "C" fn uefi_blacklist_x509_tbs(
    _source: *const c_char,
    data: *const c_void,
    len: size_t,
) {
    unsafe {
        mark_hash_blacklisted(data, len, BLACKLIST_HASH_X509_TBS);
    }
}

/*
 * Blacklist the hash of an executable.
 */
unsafe extern "C" fn uefi_blacklist_binary(
    _source: *const c_char,
    data: *const c_void,
    len: size_t,
) {
    unsafe {
        mark_hash_blacklisted(data, len, BLACKLIST_HASH_BINARY);
    }
}

/*
 * Add an X509 cert to the revocation list.
 */
unsafe extern "C" fn uefi_revocation_list_x509(
    _source: *const c_char,
    data: *const c_void,
    len: size_t,
) {
    unsafe {
        add_key_to_revocation_list(data, len);
    }
}

/*
 * Return the appropriate handler for particular signature list types found in
 * the UEFI db tables.
 */
#[no_mangle]
pub unsafe extern "C" fn get_handler_for_db(sig_type: *const efi_guid_t) -> efi_element_handler_t {
    unsafe {
        if efi_guidcmp(*sig_type, efi_cert_x509_guid) == 0 {
            return Some(add_to_platform_keyring);
        }
    }
    None
}

/*
 * Return the appropriate handler for particular signature list types found in
 * the MokListRT tables.
 */
#[no_mangle]
pub unsafe extern "C" fn get_handler_for_mok(sig_type: *const efi_guid_t) -> efi_element_handler_t {
    unsafe {
        if efi_guidcmp(*sig_type, efi_cert_x509_guid) == 0 {
            /*
             * C condition:
             * IS_ENABLED(CONFIG_INTEGRITY_MACHINE_KEYRING) &&
             * imputed_trust_enabled()
             */
            #[cfg(CONFIG_INTEGRITY_MACHINE_KEYRING)]
            {
                if imputed_trust_enabled() {
                    return Some(add_to_machine_keyring);
                } else {
                    return Some(add_to_platform_keyring);
                }
            }

            #[cfg(not(CONFIG_INTEGRITY_MACHINE_KEYRING))]
            {
                return Some(add_to_platform_keyring);
            }
        }
    }
    None
}

#[no_mangle]
pub unsafe extern "C" fn get_handler_for_ca_keys(
    sig_type: *const efi_guid_t,
) -> efi_element_handler_t {
    unsafe {
        if efi_guidcmp(*sig_type, efi_cert_x509_guid) == 0 {
            return Some(add_to_machine_keyring);
        }
    }

    None
}

#[no_mangle]
pub unsafe extern "C" fn get_handler_for_code_signing_keys(
    sig_type: *const efi_guid_t,
) -> efi_element_handler_t {
    unsafe {
        if efi_guidcmp(*sig_type, efi_cert_x509_guid) == 0 {
            return Some(add_to_secondary_keyring);
        }
    }

    None
}

/*
 * Return the appropriate handler for particular signature list types found in
 * the UEFI dbx and MokListXRT tables.
 */
#[no_mangle]
pub unsafe extern "C" fn get_handler_for_dbx(sig_type: *const efi_guid_t) -> efi_element_handler_t {
    unsafe {
        if efi_guidcmp(*sig_type, efi_cert_x509_sha256_guid) == 0 {
            return Some(uefi_blacklist_x509_tbs);
        }
        if efi_guidcmp(*sig_type, efi_cert_sha256_guid) == 0 {
            return Some(uefi_blacklist_binary);
        }
        if efi_guidcmp(*sig_type, efi_cert_x509_guid) == 0 {
            return Some(uefi_revocation_list_x509);
        }
    }
    None
}


// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
