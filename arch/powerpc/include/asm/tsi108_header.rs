/* SPDX-License-Identifier: GPL-2.0-or-later */
/*
 * common routine and memory layout for Tundra TSI108(Grendel) host bridge
 * memory controller.
 *
 * Author: Jacob Pan (jacob.pan@freescale.com)
 *         Alex Bounine (alexandreb@tundra.com)
 *
 * Copyright 2004-2006 Freescale Semiconductor, Inc.
 */

// Dependency intent: declarations from asm/pci-bridge.h are supplied elsewhere.

/* Size of entire register space */
pub const TSI108_REG_SIZE: u32 = 0x10000;

/* Sizes of register spaces for individual blocks */
pub const TSI108_HLP_SIZE: u32 = 0x1000;
pub const TSI108_PCI_SIZE: u32 = 0x1000;
pub const TSI108_CLK_SIZE: u32 = 0x1000;
pub const TSI108_PB_SIZE: u32 = 0x1000;
pub const TSI108_SD_SIZE: u32 = 0x1000;
pub const TSI108_DMA_SIZE: u32 = 0x1000;
pub const TSI108_ETH_SIZE: u32 = 0x1000;
pub const TSI108_I2C_SIZE: u32 = 0x400;
pub const TSI108_MPIC_SIZE: u32 = 0x400;
pub const TSI108_UART0_SIZE: u32 = 0x200;
pub const TSI108_GPIO_SIZE: u32 = 0x200;
pub const TSI108_UART1_SIZE: u32 = 0x200;

/* Offsets within Tsi108(A) CSR space for individual blocks */
pub const TSI108_HLP_OFFSET: u32 = 0x0000;
pub const TSI108_PCI_OFFSET: u32 = 0x1000;
pub const TSI108_CLK_OFFSET: u32 = 0x2000;
pub const TSI108_PB_OFFSET: u32 = 0x3000;
pub const TSI108_SD_OFFSET: u32 = 0x4000;
pub const TSI108_DMA_OFFSET: u32 = 0x5000;
pub const TSI108_ETH_OFFSET: u32 = 0x6000;
pub const TSI108_I2C_OFFSET: u32 = 0x7000;
pub const TSI108_MPIC_OFFSET: u32 = 0x7400;
pub const TSI108_UART0_OFFSET: u32 = 0x7800;
pub const TSI108_GPIO_OFFSET: u32 = 0x7A00;
pub const TSI108_UART1_OFFSET: u32 = 0x7C00;

/* Tsi108 registers used by common code components */
pub const TSI108_PCI_CSR: u32 = 0x004;
pub const TSI108_PCI_IRP_CFG_CTL: u32 = 0x180;
pub const TSI108_PCI_IRP_STAT: u32 = 0x184;
pub const TSI108_PCI_IRP_ENABLE: u32 = 0x188;
pub const TSI108_PCI_IRP_INTAD: u32 = 0x18C;
pub const TSI108_PCI_IRP_STAT_P_INT: u32 = 0x00400000;
pub const TSI108_PCI_IRP_ENABLE_P_INT: u32 = 0x00400000;
pub const TSI108_CG_PWRUP_STATUS: u32 = 0x234;
pub const TSI108_PB_ISR: u32 = 0x00C;
pub const TSI108_PB_ERRCS: u32 = 0x404;
pub const TSI108_PB_AERR: u32 = 0x408;
pub const TSI108_PB_ERRCS_ES: u32 = 1 << 1;
pub const TSI108_PB_ISR_PBS_RD_ERR: u32 = 1 << 8;
pub const TSI108_PCI_CFG_SIZE: u32 = 0x01000000;

/* PHY Configuration Options */
pub const TSI108_PHY_MV88E: u32 = 0; // Marvel 88Exxxx PHY
pub const TSI108_PHY_BCM54XX: u32 = 1; // Broadcom BCM54xx PHY

/* Global variables */
extern "C" {
    pub static mut tsi108_pci_cfg_base: u32;

    pub fn tsi108_direct_write_config(
        bus: *mut pci_bus,
        devfn: core::ffi::c_uint,
        offset: core::ffi::c_int,
        len: core::ffi::c_int,
        val: u32,
    ) -> core::ffi::c_int;
    pub fn tsi108_direct_read_config(
        bus: *mut pci_bus,
        devfn: core::ffi::c_uint,
        offset: core::ffi::c_int,
        len: core::ffi::c_int,
        val: *mut u32,
    ) -> core::ffi::c_int;
    pub fn tsi108_clear_pci_error(pci_cfg_base: u32);
    pub fn get_csrbase() -> phys_addr_t;
    pub fn get_vir_csrbase() -> u32;
    pub static mut tsi108_csr_vir_base: u32;
}

#[repr(C)]
pub struct hw_info {
    pub regs: u32,       /* hw registers base address */
    pub phyregs: u32,    /* phy registers base address */
    pub phy: u16,        /* phy address */
    pub irq_num: u16,    /* irq number */
    pub mac_addr: [u8; 6], /* phy mac address */
    pub phy_type: u16,   /* type of phy on board */
}

pub unsafe fn tsi108_read_reg(reg_offset: u32) -> u32 {
    in_be32((tsi108_csr_vir_base + reg_offset) as *const volatile_u32)
}

pub unsafe fn tsi108_write_reg(reg_offset: u32, val: u32) {
    out_be32((tsi108_csr_vir_base + reg_offset) as *mut volatile_u32, val);
}

// Types and byte-order accessors are supplied by the translated platform dependencies.
pub type volatile_u32 = u32;
extern "C" {
    fn in_be32(addr: *const volatile_u32) -> u32;
    fn out_be32(addr: *mut volatile_u32, val: u32);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
