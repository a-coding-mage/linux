/* SPDX-License-Identifier: GPL-2.0 */

/* Dependencies supplied by the corresponding kernel headers are intentionally
 * left external here. */

pub const EISA_MAX_SLOTS: usize = 8;
pub const EISA_MAX_RESOURCES: usize = 4;

/* A few EISA constants/offsets... */
pub const EISA_DMA1_STATUS: usize = 8;
pub const EISA_INT1_CTRL: usize = 0x20;
pub const EISA_INT1_MASK: usize = 0x21;
pub const EISA_INT2_CTRL: usize = 0xA0;
pub const EISA_INT2_MASK: usize = 0xA1;
pub const EISA_DMA2_STATUS: usize = 0xD0;
pub const EISA_DMA2_WRITE_SINGLE: usize = 0xD4;
pub const EISA_EXT_NMI_RESET_CTRL: usize = 0x461;
pub const EISA_INT1_EDGE_LEVEL: usize = 0x4D0;
pub const EISA_INT2_EDGE_LEVEL: usize = 0x4D1;
pub const EISA_VENDOR_ID_OFFSET: usize = 0xC80;
pub const EISA_CONFIG_OFFSET: usize = 0xC84;

pub const EISA_CONFIG_ENABLED: usize = 1;
pub const EISA_CONFIG_FORCED: usize = 2;

/* Chosen to hold the longest string in eisa.ids. */
pub const EISA_DEVICE_INFO_NAME_SIZE: usize = 74;

/* There is not much we can say about an EISA device, apart from
 * signature, slot number, and base address. dma_mask is set by
 * default to parent device mask..*/
#[repr(C)]
pub struct eisa_device {
    pub id: eisa_device_id,
    pub slot: ::core::ffi::c_int,
    pub state: ::core::ffi::c_int,
    pub base_addr: ::core::ffi::c_ulong,
    pub res: [resource; EISA_MAX_RESOURCES],
    pub dma_mask: u64,
    pub dev: device, /* generic device */
    #[cfg(feature = "CONFIG_EISA_NAMES")]
    pub pretty_name: [::core::ffi::c_char; EISA_DEVICE_INFO_NAME_SIZE],
}

/* Corresponds to container_of(n, struct eisa_device, dev). */
#[macro_export]
macro_rules! to_eisa_device {
    ($n:expr) => {
        container_of!($n, eisa_device, dev)
    };
}

pub unsafe fn eisa_get_region_index(addr: *mut ::core::ffi::c_void) -> ::core::ffi::c_int {
    let mut x = addr as ::core::ffi::c_ulong;
    x &= 0xc00;
    (x >> 12) as ::core::ffi::c_int
}

#[repr(C)]
pub struct eisa_driver {
    pub id_table: *const eisa_device_id,
    pub driver: device_driver,
}

/* Corresponds to container_of_const(drv, struct eisa_driver, driver). */
#[macro_export]
macro_rules! to_eisa_driver {
    ($drv:expr) => {
        container_of_const!($drv, eisa_driver, driver)
    };
}

/* These external functions are only available when EISA support is enabled. */
#[cfg(feature = "CONFIG_EISA")]
extern "C" {
    pub static eisa_bus_type: bus_type;
    pub fn eisa_driver_register(edrv: *mut eisa_driver) -> ::core::ffi::c_int;
    pub fn eisa_driver_unregister(edrv: *mut eisa_driver);
}

#[cfg(not(feature = "CONFIG_EISA"))]
pub unsafe fn eisa_driver_register(_edrv: *mut eisa_driver) -> ::core::ffi::c_int {
    0
}

#[cfg(not(feature = "CONFIG_EISA"))]
pub unsafe fn eisa_driver_unregister(_edrv: *mut eisa_driver) {}

/* Mimics pci.h... */
pub unsafe fn eisa_get_drvdata(edev: *mut eisa_device) -> *mut ::core::ffi::c_void {
    dev_get_drvdata(&mut (*edev).dev)
}

pub unsafe fn eisa_set_drvdata(edev: *mut eisa_device, data: *mut ::core::ffi::c_void) {
    dev_set_drvdata(&mut (*edev).dev, data);
}

/* The EISA root device. There's rumours about machines with multiple
 * busses (PA-RISC ?), so we try to handle that. */
#[repr(C)]
pub struct eisa_root_device {
    pub dev: *mut device, /* Pointer to bridge device */
    pub res: *mut resource,
    pub bus_base_addr: ::core::ffi::c_ulong,
    pub slots: ::core::ffi::c_int, /* Max slot number */
    pub force_probe: ::core::ffi::c_int, /* Probe even when no slot 0 */
    pub dma_mask: u64, /* from bridge device */
    pub bus_nr: ::core::ffi::c_int, /* Set by eisa_root_register */
    pub eisa_root_res: resource, /* ditto */
}

extern "C" {
    pub fn eisa_root_register(root: *mut eisa_root_device) -> ::core::ffi::c_int;
}

#[cfg(feature = "CONFIG_EISA")]
extern "C" {
    pub static mut EISA_bus: ::core::ffi::c_int;
}

#[cfg(not(feature = "CONFIG_EISA"))]
pub const EISA_bus: ::core::ffi::c_int = 0;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
