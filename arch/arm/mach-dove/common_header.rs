/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * arch/arm/mach-dove/common.h
 *
 * Core functions for Marvell Dove 88AP510 System On Chip
 */

// Dependency supplied externally: <linux/reboot.h>

#[allow(non_camel_case_types)]
pub struct mv643xx_eth_platform_data;

#[allow(non_camel_case_types)]
pub struct mv_sata_platform_data;

// Dependency supplied externally: enum reboot_mode.
#[allow(non_camel_case_types)]
pub type reboot_mode = ::core::ffi::c_int;

unsafe extern "C" {
    pub fn dove_timer_init();

    /*
     * Basic Dove init functions used early by machine-setup.
     */
    pub fn dove_map_io();
    pub fn dove_init();
    pub fn dove_init_early();
    pub fn dove_init_irq();
    pub fn dove_setup_cpu_wins();
    pub fn dove_ge00_init(eth_data: *mut mv643xx_eth_platform_data);
    pub fn dove_sata_init(sata_data: *mut mv_sata_platform_data);

    // CONFIG_PCI
    #[cfg(feature = "CONFIG_PCI")]
    pub fn dove_pcie_init(init_port0: ::core::ffi::c_int, init_port1: ::core::ffi::c_int);

    pub fn dove_ehci0_init();
    pub fn dove_ehci1_init();
    pub fn dove_uart0_init();
    pub fn dove_uart1_init();
    pub fn dove_uart2_init();
    pub fn dove_uart3_init();
    pub fn dove_spi0_init();
    pub fn dove_spi1_init();
    pub fn dove_i2c_init();
    pub fn dove_sdio0_init();
    pub fn dove_sdio1_init();
    pub fn dove_restart(mode: reboot_mode, command: *const ::core::ffi::c_char);
}

// When CONFIG_PCI is not enabled, the C header provides an empty inline function.
#[cfg(not(feature = "CONFIG_PCI"))]
#[inline]
pub fn dove_pcie_init(_init_port0: ::core::ffi::c_int, _init_port1: ::core::ffi::c_int) {}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
