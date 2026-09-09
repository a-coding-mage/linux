/* SPDX-License-Identifier: GPL-2.0 */
/*
 * Turris Mox module configuration bus driver
 *
 * Copyright (C) 2019 Marek Behún <kabel@kernel.org>
 */

/* Dependencies supplied by the Linux kernel: device, irq, irqdomain, mutex. */

pub const TURRIS_MOX_MAX_MODULES: usize = 10;

#[repr(u8)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum TurrisMoxCpuModuleId {
    TURRIS_MOX_CPU_ID_EMMC = 0x00,
    TURRIS_MOX_CPU_ID_SD = 0x10,
}

#[repr(u8)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum TurrisMoxModuleId {
    TURRIS_MOX_MODULE_FIRST = 0x01,
    TURRIS_MOX_MODULE_SFP = 0x01,
    TURRIS_MOX_MODULE_PCI = 0x02,
    TURRIS_MOX_MODULE_TOPAZ = 0x03,
    TURRIS_MOX_MODULE_PERIDOT = 0x04,
    TURRIS_MOX_MODULE_USB3 = 0x05,
    TURRIS_MOX_MODULE_PCI_BRIDGE = 0x06,
    TURRIS_MOX_MODULE_LAST = 0x06,
}

pub const MOXTET_NIRQS: usize = 16;

#[repr(C)]
pub struct MoxtetIrqpos {
    pub idx: u8,
    pub bit: u8,
}

#[repr(C)]
pub struct MoxtetIrq {
    pub domain: *mut core::ffi::c_void, // struct irq_domain *
    pub chip: core::ffi::c_void,        // struct irq_chip
    pub masked: usize,                  // unsigned long
    pub exists: usize,                  // unsigned long
    pub position: [MoxtetIrqpos; MOXTET_NIRQS],
}

#[repr(C)]
pub struct Moxtet {
    pub dev: *mut core::ffi::c_void, // struct device *
    pub lock: core::ffi::c_void,     // struct mutex
    pub modules: [u8; TURRIS_MOX_MAX_MODULES],
    pub count: core::ffi::c_int,
    pub tx: [u8; TURRIS_MOX_MAX_MODULES],
    pub dev_irq: core::ffi::c_int,
    pub irq: MoxtetIrq,
    #[cfg(CONFIG_DEBUG_FS)]
    pub debugfs_root: *mut core::ffi::c_void, // struct dentry *
}

#[repr(C)]
pub struct MoxtetDriver {
    pub id_table: *const TurrisMoxModuleId,
    pub driver: core::ffi::c_void, // struct device_driver
}

#[repr(C)]
pub struct MoxtetDevice {
    pub dev: core::ffi::c_void, // struct device
    pub moxtet: *mut Moxtet,
    pub id: TurrisMoxModuleId,
    pub idx: core::ffi::c_uint,
}

extern "C" {
    pub fn __moxtet_register_driver(
        owner: *mut core::ffi::c_void, // struct module *
        mdrv: *mut MoxtetDriver,
    ) -> core::ffi::c_int;
    pub fn moxtet_device_read(dev: *mut core::ffi::c_void) -> core::ffi::c_int;
    pub fn moxtet_device_write(dev: *mut core::ffi::c_void, val: u8) -> core::ffi::c_int;
    pub fn moxtet_device_written(dev: *mut core::ffi::c_void) -> core::ffi::c_int;
}

#[inline]
pub unsafe fn moxtet_unregister_driver(mdrv: *mut MoxtetDriver) {
    if !mdrv.is_null() {
        driver_unregister(&mut (*mdrv).driver);
    }
}

/* C macros moxtet_register_driver and module_moxtet_driver require kernel
 * macro context and are preserved here as declarations of their intent. */

#[inline]
pub unsafe fn to_moxtet_device(dev: *mut core::ffi::c_void) -> *mut MoxtetDevice {
    if dev.is_null() {
        return core::ptr::null_mut();
    }
    (dev as *mut u8).sub(core::mem::offset_of!(MoxtetDevice, dev)) as *mut MoxtetDevice
}

extern "C" {
    fn driver_unregister(driver: *mut core::ffi::c_void);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
