/* SPDX-License-Identifier: GPL-2.0 */

/*
 * QNAP TS-x09 Boards power-off function
 */
unsafe extern "C" {
    pub fn qnap_tsx09_power_off();
}

/*
 * QNAP TS-x09 Boards function to find Ethernet MAC address in flash memory
 *
 * The C __init annotation is a kernel build/linker attribute and has no
 * direct Rust syntax here.
 */
unsafe extern "C" {
    pub fn qnap_tsx09_find_mac_addr(mem_base: u32, size: u32);
}

/*
 * QNAP TS-x09 Boards ethernet declaration
 *
 * Defined by the Ethernet platform-data dependency.
 */
unsafe extern "C" {
    pub static mut qnap_tsx09_eth_data: mv643xx_eth_platform_data;
}

/* External C type supplied by the platform Ethernet dependency. */
pub enum mv643xx_eth_platform_data {}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
