/* SPDX-License-Identifier: GPL-2.0 */

/* Translated from integrity/platform_certs/keyring_handler.h. */
/* C dependency intent: #include <linux/efi.h> */

use core::ffi::{c_char, c_void};

extern "C" {
    pub fn blacklist_hash(
        source: *const c_char,
        data: *const c_void,
        len: usize,
        type_: *const c_char,
        type_len: usize,
    );

    /*
     * Blacklist an X509 TBS hash.
     */
    pub fn blacklist_x509_tbs(source: *const c_char, data: *const c_void, len: usize);

    /*
     * Blacklist the hash of an executable.
     */
    pub fn blacklist_binary(source: *const c_char, data: *const c_void, len: usize);

    /*
     * Return the handler for particular signature list types found in the db.
     */
    pub fn get_handler_for_db(sig_type: *const efi_guid_t) -> efi_element_handler_t;

    /*
     * Return the handler for particular signature list types found in the mok.
     */
    pub fn get_handler_for_mok(sig_type: *const efi_guid_t) -> efi_element_handler_t;

    /*
     * Return the handler for particular signature list types for CA keys.
     */
    pub fn get_handler_for_ca_keys(sig_type: *const efi_guid_t) -> efi_element_handler_t;

    /*
     * Return the handler for particular signature list types for code signing keys.
     */
    pub fn get_handler_for_code_signing_keys(
        sig_type: *const efi_guid_t,
    ) -> efi_element_handler_t;

    /*
     * Return the handler for particular signature list types found in the dbx.
     */
    pub fn get_handler_for_dbx(sig_type: *const efi_guid_t) -> efi_element_handler_t;
}

/*
 * C macro preserved for dependency/conditional intent:
 *
 * #ifndef UEFI_QUIRK_SKIP_CERT
 * #define UEFI_QUIRK_SKIP_CERT(vendor, product) \
 *              .matches = { \
 *                  DMI_MATCH(DMI_BOARD_VENDOR, vendor), \
 *                  DMI_MATCH(DMI_PRODUCT_NAME, product), \
 *              },
 * #endif
 *
 * This expands to a partial C struct initializer using external DMI_MATCH,
 * DMI_BOARD_VENDOR, and DMI_PRODUCT_NAME definitions, so it has no standalone
 * Rust item equivalent in this isolated header translation.
 */


// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
