/* SPDX-License-Identifier: GPL-2.0 */

#[repr(C)]
pub struct acpi_smb_hc {
    _private: [u8; 0],
}

#[repr(C)]
pub enum acpi_smb_protocol {
    SMBUS_WRITE_QUICK = 2,
    SMBUS_READ_QUICK = 3,
    SMBUS_SEND_BYTE = 4,
    SMBUS_RECEIVE_BYTE = 5,
    SMBUS_WRITE_BYTE = 6,
    SMBUS_READ_BYTE = 7,
    SMBUS_WRITE_WORD = 8,
    SMBUS_READ_WORD = 9,
    SMBUS_WRITE_BLOCK = 0xa,
    SMBUS_READ_BLOCK = 0xb,
    SMBUS_PROCESS_CALL = 0xc,
    SMBUS_BLOCK_PROCESS_CALL = 0xd,
}

#[repr(C)]
pub enum acpi_sbs_device_addr {
    ACPI_SBS_CHARGER = 0x9,
    ACPI_SBS_MANAGER = 0xa,
    ACPI_SBS_BATTERY = 0xb,
}

pub type smbus_alarm_callback = Option<unsafe extern "C" fn(context: *mut core::ffi::c_void)>;

unsafe extern "C" {
    pub fn acpi_smbus_read(
        hc: *mut acpi_smb_hc,
        protocol: u8,
        address: u8,
        command: u8,
        data: *mut u8,
    ) -> i32;

    pub fn acpi_smbus_write(
        hc: *mut acpi_smb_hc,
        protocol: u8,
        slave_address: u8,
        command: u8,
        data: *mut u8,
        length: u8,
    ) -> i32;

    pub fn acpi_smbus_register_callback(
        hc: *mut acpi_smb_hc,
        callback: smbus_alarm_callback,
        context: *mut core::ffi::c_void,
    ) -> i32;

    pub fn acpi_smbus_unregister_callback(hc: *mut acpi_smb_hc) -> i32;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
