/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * Industry-pack bus.
 *
 * Copyright (C) 2011-2012 CERN (www.cern.ch)
 * Author: Samuel Iglesias Gonsalvez <siglesias@igalia.com>
 */

// Dependencies supplied by the kernel and other headers are intentionally
// referenced here rather than reimplemented.

pub const IPACK_IDPROM_OFFSET_I: u8 = 0x01;
pub const IPACK_IDPROM_OFFSET_P: u8 = 0x03;
pub const IPACK_IDPROM_OFFSET_A: u8 = 0x05;
pub const IPACK_IDPROM_OFFSET_C: u8 = 0x07;
pub const IPACK_IDPROM_OFFSET_MANUFACTURER_ID: u8 = 0x09;
pub const IPACK_IDPROM_OFFSET_MODEL: u8 = 0x0B;
pub const IPACK_IDPROM_OFFSET_REVISION: u8 = 0x0D;
pub const IPACK_IDPROM_OFFSET_RESERVED: u8 = 0x0F;
pub const IPACK_IDPROM_OFFSET_DRIVER_ID_L: u8 = 0x11;
pub const IPACK_IDPROM_OFFSET_DRIVER_ID_H: u8 = 0x13;
pub const IPACK_IDPROM_OFFSET_NUM_BYTES: u8 = 0x15;
pub const IPACK_IDPROM_OFFSET_CRC: u8 = 0x17;

// IndustryPack Format, Vendor and Device IDs.
pub const IPACK_ID_VERSION_INVALID: u8 = 0x00;
pub const IPACK_ID_VERSION_1: u8 = 0x01;
pub const IPACK_ID_VERSION_2: u8 = 0x02;

pub const IPACK1_VENDOR_ID_RESERVED1: u8 = 0x00;
pub const IPACK1_VENDOR_ID_RESERVED2: u8 = 0xFF;
pub const IPACK1_VENDOR_ID_UNREGISTRED01: u8 = 0x01;
pub const IPACK1_VENDOR_ID_UNREGISTRED02: u8 = 0x02;
pub const IPACK1_VENDOR_ID_UNREGISTRED03: u8 = 0x03;
pub const IPACK1_VENDOR_ID_UNREGISTRED04: u8 = 0x04;
pub const IPACK1_VENDOR_ID_UNREGISTRED05: u8 = 0x05;
pub const IPACK1_VENDOR_ID_UNREGISTRED06: u8 = 0x06;
pub const IPACK1_VENDOR_ID_UNREGISTRED07: u8 = 0x07;
pub const IPACK1_VENDOR_ID_UNREGISTRED08: u8 = 0x08;
pub const IPACK1_VENDOR_ID_UNREGISTRED09: u8 = 0x09;
pub const IPACK1_VENDOR_ID_UNREGISTRED10: u8 = 0x0A;
pub const IPACK1_VENDOR_ID_UNREGISTRED11: u8 = 0x0B;
pub const IPACK1_VENDOR_ID_UNREGISTRED12: u8 = 0x0C;
pub const IPACK1_VENDOR_ID_UNREGISTRED13: u8 = 0x0D;
pub const IPACK1_VENDOR_ID_UNREGISTRED14: u8 = 0x0E;
pub const IPACK1_VENDOR_ID_UNREGISTRED15: u8 = 0x0F;
pub const IPACK1_VENDOR_ID_SBS: u8 = 0xF0;
pub const IPACK1_DEVICE_ID_SBS_OCTAL_232: u8 = 0x22;
pub const IPACK1_DEVICE_ID_SBS_OCTAL_422: u8 = 0x2A;
pub const IPACK1_DEVICE_ID_SBS_OCTAL_485: u8 = 0x48;

pub enum ipack_bus_ops {}
pub enum ipack_driver {}
pub enum ipack_device_id {}
pub enum device {}
pub enum device_driver {}
pub enum module {}
pub type phys_addr_t = usize;
pub type irqreturn_t = i32;

#[repr(C)]
pub enum ipack_space {
    IPACK_IO_SPACE = 0,
    IPACK_ID_SPACE,
    IPACK_INT_SPACE,
    IPACK_MEM8_SPACE,
    IPACK_MEM16_SPACE,
    IPACK_SPACE_COUNT,
}

#[repr(C)]
pub struct ipack_region { pub start: phys_addr_t, pub size: usize }

#[repr(C)]
pub struct ipack_device {
    pub slot: u32,
    pub bus: *mut ipack_bus_device,
    pub dev: device,
    pub release: Option<unsafe extern "C" fn(*mut ipack_device)>,
    pub region: [ipack_region; 5],
    pub id: *mut u8,
    pub id_avail: usize,
    pub id_vendor: u32,
    pub id_device: u32,
    pub id_format: u8,
    pub id_crc_correct: u32,
    pub speed_8mhz: u32,
    pub speed_32mhz: u32,
}

#[repr(C)]
pub struct ipack_driver_ops {
    pub probe: Option<unsafe extern "C" fn(*mut ipack_device) -> i32>,
    pub remove: Option<unsafe extern "C" fn(*mut ipack_device)>,
}

#[repr(C)]
pub struct ipack_driver {
    pub driver: device_driver,
    pub id_table: *const ipack_device_id,
    pub ops: *const ipack_driver_ops,
}

#[repr(C)]
pub struct ipack_bus_ops {
    pub request_irq: Option<unsafe extern "C" fn(*mut ipack_device, Option<unsafe extern "C" fn(*mut core::ffi::c_void) -> irqreturn_t>, *mut core::ffi::c_void) -> i32>,
    pub free_irq: Option<unsafe extern "C" fn(*mut ipack_device) -> i32>,
    pub get_clockrate: Option<unsafe extern "C" fn(*mut ipack_device) -> i32>,
    pub set_clockrate: Option<unsafe extern "C" fn(*mut ipack_device, i32) -> i32>,
    pub get_error: Option<unsafe extern "C" fn(*mut ipack_device) -> i32>,
    pub get_timeout: Option<unsafe extern "C" fn(*mut ipack_device) -> i32>,
    pub reset_timeout: Option<unsafe extern "C" fn(*mut ipack_device) -> i32>,
}

#[repr(C)]
pub struct ipack_bus_device {
    pub owner: *mut module,
    pub parent: *mut device,
    pub slots: i32,
    pub bus_nr: i32,
    pub ops: *const ipack_bus_ops,
}

extern "C" {
    pub fn ipack_bus_register(parent: *mut device, slots: i32, ops: *const ipack_bus_ops, owner: *mut module) -> *mut ipack_bus_device;
    pub fn ipack_bus_unregister(bus: *mut ipack_bus_device) -> i32;
    pub fn ipack_driver_register(edrv: *mut ipack_driver, owner: *mut module, name: *const core::ffi::c_char) -> i32;
    pub fn ipack_driver_unregister(edrv: *mut ipack_driver);
    pub fn ipack_device_init(dev: *mut ipack_device) -> i32;
    pub fn ipack_device_add(dev: *mut ipack_device) -> i32;
    pub fn ipack_device_del(dev: *mut ipack_device);
    pub fn ipack_get_device(dev: *mut ipack_device);
    pub fn ipack_put_device(dev: *mut ipack_device);
    pub fn try_module_get(owner: *mut module) -> bool;
    pub fn module_put(owner: *mut module);
}

#[macro_export]
macro_rules! DEFINE_IPACK_DEVICE_TABLE { ($table:ident) => { pub static mut $table: [ipack_device_id; 0] = []; }; }
#[macro_export]
macro_rules! IPACK_DEVICE { ($format:expr, $vend:expr, $dev:expr) => { format: $format, vendor: $vend, device: $dev }; }

#[inline]
pub unsafe fn ipack_get_carrier(dev: *mut ipack_device) -> bool { try_module_get((*(*dev).bus).owner) }
#[inline]
pub unsafe fn ipack_put_carrier(dev: *mut ipack_device) { module_put((*(*dev).bus).owner); }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
