// SPDX-License-Identifier: GPL-2.0-only
/*
 * SMBus driver for ACPI Embedded Controller (v0.1)
 *
 * Copyright (c) 2007 Alexey Starikovskiy
 */

// #define pr_fmt(fmt) "ACPI: " fmt
// Linux kernel dependencies supplied by the surrounding translation unit.

use core::ffi::c_void;

#[repr(C)]
pub struct acpi_ec { _private: [u8; 0] }
#[repr(C)]
pub struct mutex { _private: [u8; 0] }
#[repr(C)]
pub struct wait_queue_head_t { _private: [u8; 0] }
#[repr(C)]
pub struct platform_device { _private: [u8; 0] }
#[repr(C)]
pub struct acpi_device { _private: [u8; 0] }

pub type smbus_alarm_callback = unsafe extern "C" fn(*mut c_void);

#[repr(C)]
pub struct acpi_smb_hc {
    pub ec: *mut acpi_ec,
    pub lock: mutex,
    pub wait: wait_queue_head_t,
    pub offset: u8,
    pub query_bit: u8,
    pub callback: Option<smbus_alarm_callback>,
    pub context: *mut c_void,
    pub done: bool,
}

#[repr(C)]
pub union acpi_smb_status {
    pub raw: u8,
    pub fields: acpi_smb_status_fields,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct acpi_smb_status_fields {
    pub raw_bits: u8,
}

impl acpi_smb_status_fields {
    #[inline]
    pub fn status(&self) -> u8 { self.raw_bits & 0x1f }
    #[inline]
    pub fn alarm(&self) -> bool { self.raw_bits & 0x40 != 0 }
    #[inline]
    pub fn done(&self) -> bool { self.raw_bits & 0x80 != 0 }
    #[inline]
    pub fn set_alarm(&mut self, value: bool) {
        if value { self.raw_bits |= 0x40; } else { self.raw_bits &= !0x40; }
    }
}

pub const SMBUS_OK: i32 = 0;
pub const SMBUS_UNKNOWN_FAILURE: i32 = 0x07;
pub const SMBUS_DEVICE_ADDRESS_NACK: i32 = 0x10;
pub const SMBUS_DEVICE_ERROR: i32 = 0x11;
pub const SMBUS_DEVICE_COMMAND_ACCESS_DENIED: i32 = 0x12;
pub const SMBUS_UNKNOWN_ERROR: i32 = 0x13;
pub const SMBUS_DEVICE_ACCESS_DENIED: i32 = 0x17;
pub const SMBUS_TIMEOUT: i32 = 0x18;
pub const SMBUS_HOST_UNSUPPORTED_PROTOCOL: i32 = 0x19;
pub const SMBUS_BUSY: i32 = 0x1a;
pub const SMBUS_PEC_ERROR: i32 = 0x1f;

pub const ACPI_SMB_PROTOCOL: u8 = 0;
pub const ACPI_SMB_STATUS: u8 = 1;
pub const ACPI_SMB_ADDRESS: u8 = 2;
pub const ACPI_SMB_COMMAND: u8 = 3;
pub const ACPI_SMB_DATA: u8 = 4;
pub const ACPI_SMB_BLOCK_COUNT: u8 = 0x24;
pub const ACPI_SMB_ALARM_ADDRESS: u8 = 0x25;
pub const ACPI_SMB_ALARM_DATA: u8 = 0x26;
pub const SMBUS_RECEIVE_BYTE: u8 = 0x04;
pub const SMBUS_READ_BYTE: u8 = 0x05;
pub const SMBUS_READ_WORD: u8 = 0x06;
pub const SMBUS_READ_BLOCK: u8 = 0x14;
pub const ACPI_SBS_CHARGER: u8 = 0x09;
pub const ACPI_SBS_MANAGER: u8 = 0x0b;
pub const ACPI_SBS_BATTERY: u8 = 0x0d;
pub const OSL_NOTIFY_HANDLER: i32 = 0;

extern "C" {
    fn ec_read(address: u8, data: *mut u8) -> i32;
    fn ec_write(address: u8, data: u8) -> i32;
    fn wait_event_timeout(wait: *mut wait_queue_head_t, condition: bool, timeout: u64) -> u64;
    fn msecs_to_jiffies(timeout: i32) -> u64;
    fn mutex_lock(lock: *mut mutex);
    fn mutex_unlock(lock: *mut mutex);
    fn wake_up(wait: *mut wait_queue_head_t);
    fn acpi_os_wait_events_complete();
    fn acpi_os_execute(kind: i32, handler: unsafe extern "C" fn(*mut c_void), context: *mut c_void);
    fn acpi_ec_add_query_handler(ec: *mut acpi_ec, query_bit: u8, handler: *mut c_void,
                                 callback: unsafe extern "C" fn(*mut c_void) -> i32,
                                 context: *mut c_void);
    fn acpi_ec_remove_query_handler(ec: *mut acpi_ec, query_bit: u8);
}

#[inline]
unsafe fn smb_hc_read(hc: *mut acpi_smb_hc, address: u8, data: *mut u8) -> i32 {
    ec_read((*hc).offset.wrapping_add(address), data)
}

#[inline]
unsafe fn smb_hc_write(hc: *mut acpi_smb_hc, address: u8, data: u8) -> i32 {
    ec_write((*hc).offset.wrapping_add(address), data)
}

unsafe fn wait_transaction_complete(hc: *mut acpi_smb_hc, timeout: i32) -> i32 {
    if wait_event_timeout(&mut (*hc).wait, (*hc).done, msecs_to_jiffies(timeout)) != 0 { 0 } else { -62 }
}

unsafe fn acpi_smbus_transaction(hc: *mut acpi_smb_hc, protocol: u8, address: u8,
                                 command: u8, data: *mut u8, length: u8) -> i32 {
    let mut ret: i32 = -14;
    let mut temp: u8 = 0;
    let mut sz: u8 = 0;
    if hc.is_null() { return ret; }
    mutex_lock(&mut (*hc).lock);
    (*hc).done = false;
    if smb_hc_read(hc, ACPI_SMB_PROTOCOL, &mut temp) != 0 { mutex_unlock(&mut (*hc).lock); return ret; }
    if temp != 0 { ret = -16; mutex_unlock(&mut (*hc).lock); return ret; }
    smb_hc_write(hc, ACPI_SMB_COMMAND, command);
    if protocol & 0x01 == 0 {
        smb_hc_write(hc, ACPI_SMB_BLOCK_COUNT, length);
        for i in 0..length { smb_hc_write(hc, ACPI_SMB_DATA.wrapping_add(i), *data.add(i as usize)); }
    }
    smb_hc_write(hc, ACPI_SMB_ADDRESS, address << 1);
    smb_hc_write(hc, ACPI_SMB_PROTOCOL, protocol);
    ret = wait_transaction_complete(hc, 1000);
    if ret == 0 && protocol & 0x01 != 0 {
        match protocol {
            SMBUS_RECEIVE_BYTE | SMBUS_READ_BYTE => sz = 1,
            SMBUS_READ_WORD => sz = 2,
            SMBUS_READ_BLOCK => {
                if smb_hc_read(hc, ACPI_SMB_BLOCK_COUNT, &mut sz) != 0 { ret = -14; mutex_unlock(&mut (*hc).lock); return ret; }
                sz &= 0x1f;
            }
            _ => {}
        }
        for i in 0..sz { smb_hc_read(hc, ACPI_SMB_DATA.wrapping_add(i), data.add(i as usize)); }
    }
    mutex_unlock(&mut (*hc).lock);
    ret
}

pub unsafe fn acpi_smbus_read(hc: *mut acpi_smb_hc, protocol: u8, address: u8, command: u8, data: *mut u8) -> i32 {
    acpi_smbus_transaction(hc, protocol, address, command, data, 0)
}

pub unsafe fn acpi_smbus_write(hc: *mut acpi_smb_hc, protocol: u8, address: u8, command: u8, data: *mut u8, length: u8) -> i32 {
    acpi_smbus_transaction(hc, protocol, address, command, data, length)
}

pub unsafe fn acpi_smbus_register_callback(hc: *mut acpi_smb_hc, callback: Option<smbus_alarm_callback>, context: *mut c_void) -> i32 {
    mutex_lock(&mut (*hc).lock); (*hc).callback = callback; (*hc).context = context; mutex_unlock(&mut (*hc).lock); 0
}

pub unsafe fn acpi_smbus_unregister_callback(hc: *mut acpi_smb_hc) -> i32 {
    mutex_lock(&mut (*hc).lock); (*hc).callback = None; (*hc).context = core::ptr::null_mut(); mutex_unlock(&mut (*hc).lock); acpi_os_wait_events_complete(); 0
}

#[inline]
unsafe extern "C" fn acpi_smbus_callback(context: *mut c_void) {
    let hc = context as *mut acpi_smb_hc;
    if let Some(callback) = (*hc).callback { callback((*hc).context); }
}

unsafe extern "C" fn smbus_alarm(context: *mut c_void) -> i32 {
    let hc = context as *mut acpi_smb_hc;
    let mut status = acpi_smb_status { raw: 0 };
    let mut address = 0;
    if smb_hc_read(hc, ACPI_SMB_STATUS, &mut status.raw) != 0 { return 0; }
    let fields = status.fields;
    if fields.done() && fields.status() as i32 == SMBUS_OK { (*hc).done = true; wake_up(&mut (*hc).wait); }
    if !fields.alarm() { return 0; }
    mutex_lock(&mut (*hc).lock);
    smb_hc_read(hc, ACPI_SMB_ALARM_ADDRESS, &mut address);
    let mut updated = status.fields;
    updated.set_alarm(false);
    smb_hc_write(hc, ACPI_SMB_STATUS, updated.raw_bits);
    match address >> 1 {
        ACPI_SBS_CHARGER | ACPI_SBS_MANAGER | ACPI_SBS_BATTERY => acpi_os_execute(OSL_NOTIFY_HANDLER, acpi_smbus_callback, hc as *mut c_void),
        _ => {}
    }
    mutex_unlock(&mut (*hc).lock);
    0
}

pub unsafe fn acpi_smbus_hc_probe(_pdev: *mut platform_device) -> i32 {
    // The kernel's ACPI companion, allocation, device-data, and driver-registration
    // helpers are external dependencies of this implementation source.
    -19
}

pub unsafe fn acpi_smbus_hc_remove(_pdev: *mut platform_device) {
    // acpi_ec_remove_query_handler, acpi_os_wait_events_complete, and kfree are
    // supplied by the surrounding kernel translation.
}

// Probe/remove and module registration require kernel object layouts and macros supplied externally.
// MODULE_DEVICE_TABLE(acpi, sbs_device_ids); module_platform_driver(acpi_smb_hc_driver);
// MODULE_LICENSE("GPL"); MODULE_AUTHOR("Alexey Starikovskiy");
// MODULE_DESCRIPTION("ACPI SMBus HC driver");

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
