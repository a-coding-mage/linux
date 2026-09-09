/* SPDX-License-Identifier: GPL-2.0-or-later */
/*
 * (C) Copyright 2004, 2005 Cavium Networks
 */

#[repr(C)]
pub struct boot_init_vector {
    /* First stage address - in ram instead of flash */
    pub code_addr: u64,
    /* Setup code for application, NOT application entry point */
    pub app_start_func_addr: u32,
    /* k0 is used for global data - needs to be passed to other cores */
    pub k0_val: u32,
    /* Address of boot info block structure */
    pub boot_info_addr: u64,
    pub flags: u32, /* flags */
    pub pad: u32,
}

/* similar to bootloader's linux_app_boot_info but without global data */
#[repr(C)]
#[cfg(target_endian = "big")]
pub struct linux_app_boot_info {
    pub labi_signature: u32,
    pub start_core0_addr: u32,
    pub avail_coremask: u32,
    pub pci_console_active: u32,
    pub icache_prefetch_disable: u32,
    pub padding: u32,
    pub InitTLBStart_addr: u64,
    pub start_app_addr: u32,
    pub cur_exception_base: u32,
    pub no_mark_private_data: u32,
    pub compact_flash_common_base_addr: u32,
    pub compact_flash_attribute_base_addr: u32,
    pub led_display_base_addr: u32,
}

#[repr(C)]
#[cfg(not(target_endian = "big"))]
pub struct linux_app_boot_info {
    pub start_core0_addr: u32,
    pub labi_signature: u32,
    pub pci_console_active: u32,
    pub avail_coremask: u32,
    pub padding: u32,
    pub icache_prefetch_disable: u32,
    pub InitTLBStart_addr: u64,
    pub cur_exception_base: u32,
    pub start_app_addr: u32,
    pub compact_flash_common_base_addr: u32,
    pub no_mark_private_data: u32,
    pub led_display_base_addr: u32,
    pub compact_flash_attribute_base_addr: u32,
}

/* If not to copy a lot of bootloader's structures
   here is only offset of requested member */
pub const AVAIL_COREMASK_OFFSET_IN_LINUX_APP_BOOT_BLOCK: u32 = 0x765c;

/* hardcoded in bootloader */
pub const LABI_ADDR_IN_BOOTLOADER: u32 = 0x700;

pub const LINUX_APP_BOOT_BLOCK_NAME: &str = "linux-app-boot";

pub const LABI_SIGNATURE: u32 = 0xAABBCC01;

/* from uboot-headers/octeon_mem_map.h */
pub const EXCEPTION_BASE_INCR: u32 = 4 * 1024;
/* Increment size for exception base addresses (4k minimum) */
pub const EXCEPTION_BASE_BASE: u32 = 0;
pub const BOOTLOADER_PRIV_DATA_BASE: u32 = EXCEPTION_BASE_BASE + 0x800;
pub const BOOTLOADER_BOOT_VECTOR: u32 = BOOTLOADER_PRIV_DATA_BASE;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
