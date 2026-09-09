/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * arch/arm/mach-mv78xx0/common.h
 *
 * Core functions for Marvell MV78xx0 SoCs
 */

/* Dependency: linux/reboot.h */

#[repr(C)]
pub struct mv643xx_eth_platform_data {
    _private: [u8; 0],
}

#[repr(C)]
pub struct mv_sata_platform_data {
    _private: [u8; 0],
}

/* Dependency: enum reboot_mode from linux/reboot.h. */
#[repr(C)]
pub enum reboot_mode {
    _Opaque,
}

/*
 * Basic MV78xx0 init functions used early by machine-setup.
 */
extern "C" {
    pub fn mv78xx0_core_index() -> ::core::ffi::c_int;
    pub fn mv78xx0_map_io();
    pub fn mv78xx0_init();
    pub fn mv78xx0_init_early();
    pub fn mv78xx0_init_irq();

    pub fn mv78xx0_setup_cpu_mbus();
    pub fn mv78xx0_setup_pcie_io_win(
        window: ::core::ffi::c_int,
        base: u32,
        size: u32,
        maj: ::core::ffi::c_int,
        min: ::core::ffi::c_int,
    );
    pub fn mv78xx0_setup_pcie_mem_win(
        window: ::core::ffi::c_int,
        base: u32,
        size: u32,
        maj: ::core::ffi::c_int,
        min: ::core::ffi::c_int,
    );

    pub fn mv78xx0_pcie_id(dev: *mut u32, rev: *mut u32);

    pub fn mv78xx0_ehci0_init();
    pub fn mv78xx0_ehci1_init();
    pub fn mv78xx0_ehci2_init();
    pub fn mv78xx0_ge00_init(eth_data: *mut mv643xx_eth_platform_data);
    pub fn mv78xx0_ge01_init(eth_data: *mut mv643xx_eth_platform_data);
    pub fn mv78xx0_ge10_init(eth_data: *mut mv643xx_eth_platform_data);
    pub fn mv78xx0_ge11_init(eth_data: *mut mv643xx_eth_platform_data);
    pub fn mv78xx0_pcie_init(init_port0: ::core::ffi::c_int, init_port1: ::core::ffi::c_int);
    pub fn mv78xx0_sata_init(sata_data: *mut mv_sata_platform_data);
    pub fn mv78xx0_uart0_init();
    pub fn mv78xx0_uart1_init();
    pub fn mv78xx0_uart2_init();
    pub fn mv78xx0_uart3_init();
    pub fn mv78xx0_xor_init();
    pub fn mv78xx0_crypto_init();
    pub fn mv78xx0_i2c_init();
    pub fn mv78xx0_restart(mode: reboot_mode, cmd: *const ::core::ffi::c_char);

    pub fn mv78xx0_timer_init();
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
