/* SPDX-License-Identifier: GPL-2.0-only */
/*
 *  linux/include/amba/bus.h
 *
 *  This device type deals with ARM PrimeCells and anything else that
 *  presents a proper CID (0xB105F00D) at the end of the I/O register
 *  region or that is derived from a PrimeCell.
 *
 *  Copyright (C) 2003 Deep Blue Solutions Ltd, All Rights Reserved.
 */

// Dependencies supplied by the surrounding kernel translation.

pub const AMBA_NR_IRQS: usize = 9;
pub const AMBA_CID: u32 = 0xb105f00d;
pub const CORESIGHT_CID: u32 = 0xb105900d;

/*
 * CoreSight Architecture specification updates the ID specification
 * for components on the AMBA bus. (ARM IHI 0029E)
 *
 * Bits 15:12 of the CID are the device class.
 *
 * Class 0xF remains for PrimeCell and legacy components. (AMBA_CID above)
 * Class 0x9 defines the component as CoreSight (CORESIGHT_CID above)
 * Class 0x0, 0x1, 0xB, 0xE define components that do not have driver support
 * at present.
 * Class 0x2-0x8,0xA and 0xD-0xD are presently reserved.
 *
 * Remaining CID bits stay as 0xb105-00d
 */

/**
 * Class 0x9 components use additional values to form a Unique Component
 * Identifier (UCI), where peripheral ID values are identical for different
 * components. Passed to the amba bus code from the component driver via
 * the amba_id->data pointer.
 * @devarch       : coresight devarch register value
 * @devarch_mask  : mask bits used for matching. 0 indicates UCI not used.
 * @devtype       : coresight device type value
 * @data          : additional driver data. As we have usurped the original
 *                  pointer some devices may still need additional data
 */
#[repr(C)]
pub struct amba_cs_uci_id {
    pub devarch: u32,
    pub devarch_mask: u32,
    pub devtype: u32,
    pub data: *mut core::ffi::c_void,
}

/* define offsets for registers used by UCI */
pub const UCI_REG_DEVTYPE_OFFSET: usize = 0xFCC;
pub const UCI_REG_DEVARCH_OFFSET: usize = 0xFBC;

#[repr(C)]
pub struct amba_device {
    pub dev: device,
    pub res: resource,
    pub pclk: *mut clk,
    pub dma_parms: device_dma_parameters,
    pub periphid: u32,
    pub periphid_lock: mutex,
    pub cid: u32,
    pub uci: amba_cs_uci_id,
    pub irq: [u32; AMBA_NR_IRQS],
}

#[repr(C)]
pub struct amba_driver {
    pub drv: device_driver,
    pub probe: Option<unsafe extern "C" fn(*mut amba_device, *const amba_id) -> i32>,
    pub remove: Option<unsafe extern "C" fn(*mut amba_device)>,
    pub shutdown: Option<unsafe extern "C" fn(*mut amba_device)>,
    pub id_table: *const amba_id,
    /*
     * For most device drivers, no need to care about this flag as long as
     * all DMAs are handled through the kernel DMA API. For some special
     * ones, for example VFIO drivers, they know how to manage the DMA
     * themselves and set this flag so that the IOMMU layer will allow them
     * to setup and manage their own I/O address space.
     */
    pub driver_managed_dma: bool,
}

/*
 * Constants for the designer field of the Peripheral ID register. When bit 7
 * is set to '1', bits [6:0] should be the JEP106 manufacturer identity code.
 */
#[repr(C)]
pub enum amba_vendor {
    AMBA_VENDOR_ARM = 0x41,
    AMBA_VENDOR_ST = 0x80,
    AMBA_VENDOR_QCOM = 0x51,
    AMBA_VENDOR_LSI = 0xb6,
}

pub const AMBA_CONFIG_BITS: unsafe fn(u32) -> u32 = |a| (a >> 24) & 0xff;
pub const AMBA_REV_BITS: unsafe fn(u32) -> u32 = |a| (a >> 20) & 0x0f;
pub const AMBA_MANF_BITS: unsafe fn(u32) -> u32 = |a| (a >> 12) & 0xff;
pub const AMBA_PART_BITS: unsafe fn(u32) -> u32 = |a| a & 0xfff;

extern "C" {
    pub static amba_bustype: bus_type;
    pub fn __amba_driver_register(drv: *mut amba_driver, owner: *mut module) -> i32;
    pub fn amba_driver_unregister(drv: *mut amba_driver);
    pub fn dev_is_amba(dev: *const device) -> bool;
    pub fn amba_device_alloc(name: *const core::ffi::c_char, size: resource_size_t, sz: usize) -> *mut amba_device;
    pub fn amba_device_put(dev: *mut amba_device);
    pub fn amba_device_add(dev: *mut amba_device, res: *mut resource) -> i32;
    pub fn amba_device_register(dev: *mut amba_device, res: *mut resource) -> i32;
    pub fn amba_device_unregister(dev: *mut amba_device);
    pub fn amba_request_regions(dev: *mut amba_device, name: *const core::ffi::c_char) -> i32;
    pub fn amba_release_regions(dev: *mut amba_device);
}

#[cfg(not(CONFIG_ARM_AMBA))]
pub unsafe fn __amba_driver_register(_drv: *mut amba_driver, _owner: *mut module) -> i32 { -EINVAL }
#[cfg(not(CONFIG_ARM_AMBA))]
pub unsafe fn amba_driver_unregister(_drv: *mut amba_driver) {}
#[cfg(not(CONFIG_ARM_AMBA))]
pub unsafe fn dev_is_amba(_dev: *const device) -> bool { false }

#[macro_export]
macro_rules! amba_driver_register { ($drv:expr) => { __amba_driver_register($drv, THIS_MODULE) }; }
#[macro_export]
macro_rules! amba_get_drvdata { ($d:expr) => { dev_get_drvdata(&(*$d).dev) }; }
#[macro_export]
macro_rules! amba_set_drvdata { ($d:expr, $p:expr) => { dev_set_drvdata(&mut (*$d).dev, $p) }; }
#[macro_export]
macro_rules! AMBA_CONFIG { ($a:expr) => { (($a >> 24) & 0xff) }; }
#[macro_export]
macro_rules! AMBA_REV { ($a:expr) => { (($a >> 20) & 0x0f) }; }
#[macro_export]
macro_rules! AMBA_MANF { ($a:expr) => { (($a >> 12) & 0xff) }; }
#[macro_export]
macro_rules! AMBA_PART { ($a:expr) => { ($a & 0xfff) }; }

// The remaining C macros (container_of_const, DEFINE_RES_MEM, module_driver,
// builtin_driver, and device initializers) depend on definitions supplied by
// other headers and are represented by their source-level macro forms.
#[macro_export]
macro_rules! module_amba_driver { ($drv:expr) => { module_driver!($drv, amba_driver_register, amba_driver_unregister) }; }
#[macro_export]
macro_rules! builtin_amba_driver { ($drv:expr) => { builtin_driver!($drv, amba_driver_register) }; }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
