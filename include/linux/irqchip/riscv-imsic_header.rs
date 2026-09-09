/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * Copyright (C) 2021 Western Digital Corporation or its affiliates.
 * Copyright (C) 2022 Ventana Micro Systems Inc.
 */

// Dependencies supplied by the surrounding kernel translation.

pub const IMSIC_MMIO_PAGE_SHIFT: u32 = 12;
pub const IMSIC_MMIO_PAGE_SZ: u32 = 1u32 << IMSIC_MMIO_PAGE_SHIFT;
pub const IMSIC_MMIO_PAGE_LE: u32 = 0x00;
pub const IMSIC_MMIO_PAGE_BE: u32 = 0x04;

pub const IMSIC_MIN_ID: u32 = 63;
pub const IMSIC_MAX_ID: u32 = 2048;

pub const IMSIC_EIDELIVERY: u32 = 0x70;

pub const IMSIC_EITHRESHOLD: u32 = 0x72;

pub const IMSIC_EIP0: u32 = 0x80;
pub const IMSIC_EIP63: u32 = 0xbf;
pub const IMSIC_EIPX_BITS: u32 = 32;

pub const IMSIC_EIE0: u32 = 0xc0;
pub const IMSIC_EIE63: u32 = 0xff;
pub const IMSIC_EIX_BITS: u32 = 32;

pub const IMSIC_FIRST: u32 = IMSIC_EIDELIVERY;
pub const IMSIC_LAST: u32 = IMSIC_EIE63;

pub const IMSIC_MMIO_SETIPNUM_LE: u32 = 0x00;
pub const IMSIC_MMIO_SETIPNUM_BE: u32 = 0x04;

#[repr(C)]
pub struct imsic_local_config {
    pub msi_pa: phys_addr_t,
    pub msi_va: *mut core::ffi::c_void,

    /* Number of guest interrupt files per-HART */
    pub nr_guest_files: u32,
}

#[repr(C)]
pub struct imsic_global_config {
    /*
     * MSI Target Address Scheme
     *
     * XLEN-1                                                12     0
     * |                                                     |     |
     * -------------------------------------------------------------
     * |xxxxxx|Group Index|xxxxxxxxxxx|HART Index|Guest Index|  0  |
     * -------------------------------------------------------------
     */

    /* Bits representing Guest index, HART index, and Group index */
    pub guest_index_bits: u32,
    pub hart_index_bits: u32,
    pub group_index_bits: u32,
    pub group_index_shift: u32,

    /* Global base address matching all target MSI addresses */
    pub base_addr: phys_addr_t,

    /* Number of interrupt identities */
    pub nr_ids: u32,

    /* Number of guest interrupt identities */
    pub nr_guest_ids: u32,

    /* Number of guest interrupt files across all HARTs */
    pub nr_guest_files: u32,

    /* Per-CPU IMSIC addresses */
    pub local: *mut imsic_local_config,
}

// When CONFIG_RISCV_IMSIC is enabled:
// const struct imsic_global_config *imsic_get_global_config(void);
extern "C" {
    pub fn imsic_get_global_config() -> *const imsic_global_config;
}

// When CONFIG_RISCV_IMSIC is disabled, the inline definition is:
// static inline const struct imsic_global_config *imsic_get_global_config(void)
// {
//     return NULL;
// }

// When both CONFIG_ACPI and CONFIG_RISCV_IMSIC are enabled:
// int imsic_platform_acpi_probe(struct fwnode_handle *fwnode);
// struct fwnode_handle *imsic_acpi_get_fwnode(struct device *dev);
extern "C" {
    pub fn imsic_platform_acpi_probe(fwnode: *mut fwnode_handle) -> i32;
    pub fn imsic_acpi_get_fwnode(dev: *mut device) -> *mut fwnode_handle;
}

// When ACPI or RISCV_IMSIC is disabled:
// static inline struct fwnode_handle *imsic_acpi_get_fwnode(struct device *dev) { return NULL; }


// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
