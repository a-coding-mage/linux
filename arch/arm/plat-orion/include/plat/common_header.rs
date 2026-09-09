/*
 * arch/arm/plat-orion/include/plat/common.h
 *
 * Marvell Orion SoC common setup code used by different mach-/common.c
 *
 * This file is licensed under the terms of the GNU General Public
 * License version 2  This program is licensed "as is" without any
 * warranty of any kind, whether express or implied.
 */

// The original header guard was: __PLAT_COMMON_H
// Dependencies supplied by the surrounding kernel translation are intentionally
// referenced here rather than reimplemented.

use core::ffi::c_char;

extern "C" {
    pub fn orion_uart0_init(
        membase: *mut core::ffi::c_void,
        mapbase: resource_size_t,
        irq: core::ffi::c_uint,
        clk: *mut clk,
    );

    pub fn orion_uart1_init(
        membase: *mut core::ffi::c_void,
        mapbase: resource_size_t,
        irq: core::ffi::c_uint,
        clk: *mut clk,
    );

    pub fn orion_uart2_init(
        membase: *mut core::ffi::c_void,
        mapbase: resource_size_t,
        irq: core::ffi::c_uint,
        clk: *mut clk,
    );

    pub fn orion_uart3_init(
        membase: *mut core::ffi::c_void,
        mapbase: resource_size_t,
        irq: core::ffi::c_uint,
        clk: *mut clk,
    );

    pub fn orion_rtc_init(mapbase: core::ffi::c_ulong, irq: core::ffi::c_ulong);

    pub fn orion_ge00_init(
        eth_data: *mut mv643xx_eth_platform_data,
        mapbase: core::ffi::c_ulong,
        irq: core::ffi::c_ulong,
        irq_err: core::ffi::c_ulong,
        tx_csum_limit: core::ffi::c_uint,
    );

    pub fn orion_ge01_init(
        eth_data: *mut mv643xx_eth_platform_data,
        mapbase: core::ffi::c_ulong,
        irq: core::ffi::c_ulong,
        tx_csum_limit: core::ffi::c_uint,
    );

    pub fn orion_ge10_init(
        eth_data: *mut mv643xx_eth_platform_data,
        mapbase: core::ffi::c_ulong,
        irq: core::ffi::c_ulong,
    );

    pub fn orion_ge11_init(
        eth_data: *mut mv643xx_eth_platform_data,
        mapbase: core::ffi::c_ulong,
        irq: core::ffi::c_ulong,
    );

    pub fn orion_i2c_init(
        mapbase: core::ffi::c_ulong,
        irq: core::ffi::c_ulong,
        freq_m: core::ffi::c_ulong,
    );

    pub fn orion_i2c_1_init(
        mapbase: core::ffi::c_ulong,
        irq: core::ffi::c_ulong,
        freq_m: core::ffi::c_ulong,
    );

    pub fn orion_spi_init(mapbase: core::ffi::c_ulong);

    pub fn orion_spi_1_init(mapbase: core::ffi::c_ulong);

    pub fn orion_xor0_init(
        mapbase_low: core::ffi::c_ulong,
        mapbase_high: core::ffi::c_ulong,
        irq_0: core::ffi::c_ulong,
        irq_1: core::ffi::c_ulong,
    );

    pub fn orion_xor1_init(
        mapbase_low: core::ffi::c_ulong,
        mapbase_high: core::ffi::c_ulong,
        irq_0: core::ffi::c_ulong,
        irq_1: core::ffi::c_ulong,
    );

    pub fn orion_ehci_init(
        mapbase: core::ffi::c_ulong,
        irq: core::ffi::c_ulong,
        phy_version: orion_ehci_phy_ver,
    );

    pub fn orion_ehci_1_init(mapbase: core::ffi::c_ulong, irq: core::ffi::c_ulong);

    pub fn orion_ehci_2_init(mapbase: core::ffi::c_ulong, irq: core::ffi::c_ulong);

    pub fn orion_sata_init(
        sata_data: *mut mv_sata_platform_data,
        mapbase: core::ffi::c_ulong,
        irq: core::ffi::c_ulong,
    );

    pub fn orion_crypto_init(
        mapbase: core::ffi::c_ulong,
        srambase: core::ffi::c_ulong,
        sram_size: core::ffi::c_ulong,
        irq: core::ffi::c_ulong,
    );

    pub fn orion_clkdev_add(
        con_id: *const c_char,
        dev_id: *const c_char,
        clk: *mut clk,
    );

    pub fn orion_clkdev_init(tclk: *mut clk);
}

// External types declared by the included kernel headers or other translated
// files.
pub type resource_size_t = usize;

#[repr(C)]
pub struct mv_sata_platform_data {
    _private: [u8; 0],
}

#[repr(C)]
pub struct mv643xx_eth_platform_data {
    _private: [u8; 0],
}

#[repr(C)]
pub struct clk {
    _private: [u8; 0],
}


// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
