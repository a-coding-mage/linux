/*
 * Marvell MBUS common definitions.
 *
 * Copyright (C) 2008 Marvell Semiconductor
 *
 * This file is licensed under the terms of the GNU General Public
 * License version 2.  This program is licensed "as is" without any
 * warranty of any kind, whether express or implied.
 */

/* C dependency: linux/errno.h */

use core::ffi::c_char;

/* C dependency: struct resource */
#[repr(C)]
pub struct resource {
    _private: [u8; 0],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mbus_dram_window {
    pub cs_index: u8,
    pub mbus_attr: u8,
    pub base: u64,
    pub size: u64,
}

#[repr(C)]
pub struct mbus_dram_target_info {
    /* The 4-bit MBUS target ID of the DRAM controller. */
    pub mbus_dram_target_id: u8,

    /*
     * The base address, size, and MBUS attribute ID for each
     * of the possible DRAM chip selects.  Peripherals are
     * required to support at least 4 decode windows.
     */
    pub num_cs: i32,
    pub cs: [mbus_dram_window; 4],
}

/* Flags for PCI/PCIe address decoding regions */
pub const MVEBU_MBUS_PCI_IO: u32 = 0x1;
pub const MVEBU_MBUS_PCI_MEM: u32 = 0x2;
pub const MVEBU_MBUS_PCI_WA: u32 = 0x3;

/*
 * Magic value that explicits that we don't need a remapping-capable
 * address decoding window.
 */
pub const MVEBU_MBUS_NO_REMAP: u32 = 0xffff_ffff;

/* Maximum size of a mbus window name */
pub const MVEBU_MBUS_MAX_WINNAME_SZ: u32 = 32;

/* C dependency: phys_addr_t, size_t, bool, and __iomem are supplied elsewhere. */

#[cfg(feature = "CONFIG_PLAT_ORION")]
extern "C" {
    pub fn mv_mbus_dram_info() -> *const mbus_dram_target_info;
    pub fn mv_mbus_dram_info_nooverlap() -> *const mbus_dram_target_info;
    pub fn mvebu_mbus_get_io_win_info(
        phyaddr: phys_addr_t,
        size: *mut u32,
        target: *mut u8,
        attr: *mut u8,
    ) -> i32;
}

#[cfg(not(feature = "CONFIG_PLAT_ORION"))]
pub unsafe fn mv_mbus_dram_info() -> *const mbus_dram_target_info {
    core::ptr::null()
}

#[cfg(not(feature = "CONFIG_PLAT_ORION"))]
pub unsafe fn mv_mbus_dram_info_nooverlap() -> *const mbus_dram_target_info {
    core::ptr::null()
}

#[cfg(not(feature = "CONFIG_PLAT_ORION"))]
pub unsafe fn mvebu_mbus_get_io_win_info(
    _phyaddr: phys_addr_t,
    _size: *mut u32,
    _target: *mut u8,
    _attr: *mut u8,
) -> i32 {
    /*
     * On all ARM32 MVEBU platforms with MBus support, this stub
     * function will not get called. The real function from the
     * MBus driver is called instead. ARM64 MVEBU platforms like
     * the Armada 3700 could use the mv_xor device driver which calls
     * into this function
     */
    -22 /* -EINVAL; supplied by linux/errno.h */
}

#[cfg(feature = "CONFIG_MVEBU_MBUS")]
extern "C" {
    pub fn mvebu_mbus_save_cpu_target(store_addr: *mut u32) -> i32;
    pub fn mvebu_mbus_get_pcie_mem_aperture(res: *mut resource);
    pub fn mvebu_mbus_get_pcie_io_aperture(res: *mut resource);
    pub fn mvebu_mbus_get_dram_win_info(
        phyaddr: phys_addr_t,
        target: *mut u8,
        attr: *mut u8,
    ) -> i32;
    pub fn mvebu_mbus_add_window_remap_by_id(
        target: u32,
        attribute: u32,
        base: phys_addr_t,
        size: usize,
        remap: phys_addr_t,
    ) -> i32;
    pub fn mvebu_mbus_add_window_by_id(
        target: u32,
        attribute: u32,
        base: phys_addr_t,
        size: usize,
    ) -> i32;
    pub fn mvebu_mbus_del_window(base: phys_addr_t, size: usize) -> i32;
    pub fn mvebu_mbus_init(
        soc: *const c_char,
        mbus_phys_base: phys_addr_t,
        mbus_size: usize,
        sdram_phys_base: phys_addr_t,
        sdram_size: usize,
    ) -> i32;
    pub fn mvebu_mbus_dt_init(is_coherent: bool) -> i32;
}

#[cfg(not(feature = "CONFIG_MVEBU_MBUS"))]
pub unsafe fn mvebu_mbus_get_dram_win_info(
    _phyaddr: phys_addr_t,
    _target: *mut u8,
    _attr: *mut u8,
) -> i32 {
    -22 /* -EINVAL; supplied by linux/errno.h */
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
