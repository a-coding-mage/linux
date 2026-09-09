/* SPDX-License-Identifier: GPL-2.0 */

// Dependency supplied by <linux/device-id/parisc.h>.

pub const HWTYPE_ANY_ID: _ = PA_HWTYPE_ANY_ID;
pub const HVERSION_ANY_ID: _ = PA_HVERSION_ANY_ID;
pub const HVERSION_REV_ANY_ID: _ = PA_HVERSION_REV_ANY_ID;
pub const SVERSION_ANY_ID: _ = PA_SVERSION_ANY_ID;

#[repr(C, packed)]
pub struct hp_hardware {
    // C bit-fields: hw_type:8, hversion:12, sversion:12.
    pub hardware_id: u32,
    pub opt: u8,
    pub name: [u8; 59], /* The hardware description */
}

pub struct parisc_device;

#[repr(C)]
pub enum cpu_type {
    pcx = 0,  /* pa7000        pa 1.0  */
    pcxs = 1, /* pa7000        pa 1.1a */
    pcxt = 2, /* pa7100        pa 1.1b */
    pcxt_ = 3, /* pa7200 (t')   pa 1.1c */
    pcxl = 4, /* pa7100lc      pa 1.1d */
    pcxl2 = 5, /* pa7300lc      pa 1.1e */
    pcxu = 6, /* pa8000        pa 2.0  */
    pcxu_ = 7, /* pa8200 (u+)   pa 2.0  */
    pcxw = 8, /* pa8500        pa 2.0  */
    pcxw_ = 9, /* pa8600 (w+)   pa 2.0  */
    pcxw2 = 10, /* pa8700        pa 2.0  */
    mako = 11, /* pa8800        pa 2.0  */
    mako2 = 12, /* pa8900        pa 2.0  */
}

unsafe extern "C" {
    pub static cpu_name_version: [[*const core::ffi::c_char; 2]; 13]; /* mapping from enum cpu_type to strings */
}

pub struct parisc_driver;

#[repr(C)]
pub struct io_module {
    pub nothing: u32, /* volatile reg 0 */
    pub io_eim: u32,
    pub io_dc_adata: u32,
    pub io_ii_cdata: u32,
    pub io_dma_link: u32, /* volatile reg 4 */
    pub io_dma_command: u32,
    pub io_dma_address: u32,
    pub io_dma_count: u32,
    pub io_flex: u32, /* volatile reg 8 */
    pub io_spa_address: u32,
    pub reserved1: [u32; 2],
    pub io_command: u32, /* volatile reg 12 */
    pub io_status: u32,
    pub io_control: u32,
    pub io_data: u32,
    pub reserved2: u32, /* volatile reg 16 */
    pub chain_addr: u32,
    pub sub_mask_clr: u32,
    pub reserved3: [u32; 13],
    pub undefined: [u32; 480],
    pub unpriv: [u32; 512],
}

#[repr(C)]
pub struct bc_module {
    pub unused1: [u32; 12],
    pub io_command: u32,
    pub io_status: u32,
    pub io_control: u32,
    pub unused2: [u32; 1],
    pub io_err_resp: u32,
    pub io_err_info: u32,
    pub io_err_req: u32,
    pub unused3: [u32; 11],
    pub io_io_low: u32,
    pub io_io_high: u32,
}

pub const HPHW_NPROC: u32 = 0;
pub const HPHW_MEMORY: u32 = 1;
pub const HPHW_B_DMA: u32 = 2;
pub const HPHW_OBSOLETE: u32 = 3;
pub const HPHW_A_DMA: u32 = 4;
pub const HPHW_A_DIRECT: u32 = 5;
pub const HPHW_OTHER: u32 = 6;
pub const HPHW_BCPORT: u32 = 7;
pub const HPHW_CIO: u32 = 8;
pub const HPHW_CONSOLE: u32 = 9;
pub const HPHW_FIO: u32 = 10;
pub const HPHW_BA: u32 = 11;
pub const HPHW_IOA: u32 = 12;
pub const HPHW_BRIDGE: u32 = 13;
pub const HPHW_FABRIC: u32 = 14;
pub const HPHW_MC: u32 = 15;
pub const HPHW_FAULTY: u32 = 31;

pub struct parisc_device_id;
pub struct pci_dev;
pub struct hardware_path;
pub struct device;

unsafe extern "C" {
    pub fn parisc_hardware_description(id: *mut parisc_device_id) -> *const core::ffi::c_char;
    pub fn parisc_get_cpu_type(hversion: core::ffi::c_ulong) -> cpu_type;
    pub fn alloc_pa_dev(hpa: core::ffi::c_ulong, path: *mut hardware_path) -> *mut parisc_device;
    pub fn register_parisc_device(dev: *mut parisc_device) -> core::ffi::c_int;
    pub fn register_parisc_driver(driver: *mut parisc_driver) -> core::ffi::c_int;
    pub fn count_parisc_driver(driver: *mut parisc_driver) -> core::ffi::c_int;
    pub fn unregister_parisc_driver(driver: *mut parisc_driver) -> core::ffi::c_int;
    pub fn walk_central_bus();
    pub fn find_pa_parent_type(dev: *const parisc_device, ty: core::ffi::c_int) -> *const parisc_device;
    pub fn print_parisc_devices();
    pub fn print_pa_hwpath(dev: *mut parisc_device, path: *mut core::ffi::c_char) -> *mut core::ffi::c_char;
    pub fn print_pci_hwpath(dev: *mut pci_dev, path: *mut core::ffi::c_char) -> *mut core::ffi::c_char;
    pub fn get_pci_node_path(dev: *mut pci_dev, path: *mut hardware_path);
    pub fn init_parisc_bus();
    pub fn hwpath_to_device(modpath: *mut hardware_path) -> *mut device;
    pub fn device_to_hwpath(dev: *mut device, path: *mut hardware_path);
    pub fn machine_has_merced_bus() -> core::ffi::c_int;
    pub fn do_memory_inventory();
    pub fn do_device_inventory();
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
