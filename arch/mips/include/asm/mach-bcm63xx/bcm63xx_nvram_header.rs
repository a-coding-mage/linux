/* SPDX-License-Identifier: GPL-2.0 */

//! Translated declarations from `bcm63xx_nvram.h`.

/**
 * bcm63xx_nvram_init() - initializes nvram
 * @nvram: address of the nvram data
 *
 * Initialized the local nvram copy from the target address and checks
 * its checksum.
 */
extern "C" {
    pub fn bcm63xx_nvram_init(nvram: *mut core::ffi::c_void);
}

/**
 * bcm63xx_nvram_get_name() - returns the board name according to nvram
 *
 * Returns the board name field from nvram. Note that it might not be
 * null terminated if it is exactly 16 bytes long.
 */
extern "C" {
    pub fn bcm63xx_nvram_get_name() -> *mut u8;
}

/**
 * bcm63xx_nvram_get_mac_address() - register & return a new mac address
 * @mac: pointer to array for allocated mac
 *
 * Registers and returns a mac address from the allocated macs from nvram.
 *
 * Returns 0 on success.
 */
extern "C" {
    pub fn bcm63xx_nvram_get_mac_address(mac: *mut u8) -> core::ffi::c_int;
}

extern "C" {
    pub fn bcm63xx_nvram_get_psi_size() -> core::ffi::c_int;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
