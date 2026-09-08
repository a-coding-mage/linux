// SPDX-License-Identifier: GPL-2.0+
/*
 * Platform keyring for firmware/platform keys
 *
 * Copyright IBM Corporation, 2018
 * Author(s): Nayna Jain <nayna@linux.ibm.com>
 */

use core::ffi::{c_char, c_int, c_void};

// Key permission type (from Linux kernel)
type key_perm_t = u32;

// Constants from Linux kernel headers
// KEY_POS_ALL, KEY_POS_SETATTR, KEY_USR_VIEW from <linux/key.h>
// INTEGRITY_KEYRING_PLATFORM from ../integrity.h
const KEY_POS_ALL: key_perm_t = 0x3f000000;
const KEY_POS_SETATTR: key_perm_t = 0x00040000;
const KEY_USR_VIEW: key_perm_t = 0x00000001;
const INTEGRITY_KEYRING_PLATFORM: c_int = 0;

// External functions from integrity subsystem and Linux kernel
extern "C" {
    fn integrity_load_cert(
        keyring: c_int,
        source: *const c_char,
        data: *const c_void,
        len: usize,
        perm: key_perm_t,
    ) -> c_int;

    fn integrity_init_keyring(keyring: c_int) -> c_int;

    fn pr_info(fmt: *const c_char, ...) -> c_int;
    fn pr_notice(fmt: *const c_char, ...) -> c_int;
}

/**
 * add_to_platform_keyring - Add to platform keyring without validation.
 * @source: Source of key
 * @data: The blob holding the key
 * @len: The length of the data blob
 *
 * Add a key to the platform keyring without checking its trust chain.  This
 * is available only during kernel initialisation.
 */
pub unsafe extern "C" fn add_to_platform_keyring(
    source: *const c_char,
    data: *const c_void,
    len: usize,
) {
    let perm: key_perm_t = (KEY_POS_ALL & !KEY_POS_SETATTR) | KEY_USR_VIEW;

    let rc = integrity_load_cert(INTEGRITY_KEYRING_PLATFORM, source, data, len, perm);
    if rc != 0 {
        pr_info(
            b"Error adding keys to platform keyring %s\n\0".as_ptr() as *const c_char,
            source,
        );
    }
}

/*
 * Create the trusted keyrings.
 */
pub unsafe extern "C" fn platform_keyring_init() -> c_int {
    let rc = integrity_init_keyring(INTEGRITY_KEYRING_PLATFORM);
    if rc != 0 {
        return rc;
    }

    pr_notice(b"Platform Keyring initialized\n\0".as_ptr() as *const c_char);
    0
}

/*
 * Must be initialised before we try and load the keys into the keyring.
 *
 * Note: device_initcall(platform_keyring_init) is a Linux kernel macro
 * that registers platform_keyring_init to be called during device initialization.
 * This registration is handled by the kernel build system and linker scripts.
 */


// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
