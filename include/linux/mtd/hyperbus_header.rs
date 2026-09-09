/* SPDX-License-Identifier: GPL-2.0
 *
 * Copyright (C) 2019 Texas Instruments Incorporated - https://www.ti.com/
 */

/* Dependency supplied by the Linux MTD map interface. */

/* HyperBus command bits */
pub const HYPERBUS_RW: u8 = 0x80; /* R/W# */
pub const HYPERBUS_RW_WRITE: u8 = 0;
pub const HYPERBUS_RW_READ: u8 = 0x80;
pub const HYPERBUS_AS: u8 = 0x40; /* Address Space */
pub const HYPERBUS_AS_MEM: u8 = 0;
pub const HYPERBUS_AS_REG: u8 = 0x40;
pub const HYPERBUS_BT: u8 = 0x20; /* Burst Type */
pub const HYPERBUS_BT_WRAPPED: u8 = 0;
pub const HYPERBUS_BT_LINEAR: u8 = 0x20;

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum hyperbus_memtype {
    HYPERFLASH,
    HYPERRAM,
}

/**
 * struct hyperbus_device - struct representing HyperBus slave device
 * @map: map_info struct for accessing MMIO HyperBus flash memory
 * @np: pointer to HyperBus slave device node
 * @mtd: pointer to MTD struct
 * @ctlr: pointer to HyperBus controller struct
 * @memtype: type of memory device: HyperFlash or HyperRAM
 * @priv: pointer to controller specific per device private data
 */
#[repr(C)]
pub struct hyperbus_device {
    pub map: map_info,
    pub np: *mut device_node,
    pub mtd: *mut mtd_info,
    pub ctlr: *mut hyperbus_ctlr,
    pub memtype: hyperbus_memtype,
    pub priv_: *mut core::ffi::c_void,
}

/**
 * struct hyperbus_ops - struct representing custom HyperBus operations
 * @read16: read 16 bit of data from flash in a single burst. Used to read
 *          from non default address space, such as ID/CFI space
 * @write16: write 16 bit of data to flash in a single burst. Used to
 *           send cmd to flash or write single 16 bit word at a time.
 * @copy_from: copy data from flash memory
 * @copy_to: copy data to flash memory
 * @calibrate: calibrate HyperBus controller
 */
#[repr(C)]
pub struct hyperbus_ops {
    pub read16: Option<unsafe extern "C" fn(hbdev: *mut hyperbus_device, addr: libc::c_ulong) -> u16>,
    pub write16: Option<unsafe extern "C" fn(hbdev: *mut hyperbus_device, addr: libc::c_ulong, val: u16)>,
    pub copy_from: Option<unsafe extern "C" fn(hbdev: *mut hyperbus_device, to: *mut core::ffi::c_void, from: libc::c_ulong, len: isize)>,
    pub copy_to: Option<unsafe extern "C" fn(dev: *mut hyperbus_device, to: libc::c_ulong, from: *const core::ffi::c_void, len: isize)>,
    pub calibrate: Option<unsafe extern "C" fn(dev: *mut hyperbus_device) -> i32>,
}

/**
 * struct hyperbus_ctlr - struct representing HyperBus controller
 * @dev: pointer to HyperBus controller device
 * @calibrated: flag to indicate ctlr calibration sequence is complete
 * @ops: HyperBus controller ops
 */
#[repr(C)]
pub struct hyperbus_ctlr {
    pub dev: *mut device,
    pub calibrated: bool,
    pub ops: *const hyperbus_ops,
}

/**
 * hyperbus_register_device - probe and register a HyperBus slave memory device
 * @hbdev: hyperbus_device struct with dev, np and ctlr field populated
 *
 * Return: 0 for success, others for failure.
 */
unsafe extern "C" {
    pub fn hyperbus_register_device(hbdev: *mut hyperbus_device) -> i32;
    /**
     * hyperbus_unregister_device - deregister HyperBus slave memory device
     * @hbdev: hyperbus_device to be unregistered
     */
    pub fn hyperbus_unregister_device(hbdev: *mut hyperbus_device);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
