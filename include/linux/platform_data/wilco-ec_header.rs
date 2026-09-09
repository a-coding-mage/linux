/* SPDX-License-Identifier: GPL-2.0 */
/*
 * ChromeOS Wilco Embedded Controller
 *
 * Copyright 2018 Google LLC
 */

/* Message flags for using the mailbox() interface */
pub const WILCO_EC_FLAG_NO_RESPONSE: u32 = 1u32 << 0; /* EC does not respond */

/* Normal commands have a maximum 32 bytes of data */
pub const EC_MAILBOX_DATA_SIZE: usize = 32;

/* Opaque types supplied by the Linux kernel and other dependencies. */
#[repr(C)]
pub struct device {
    _private: [u8; 0],
}
#[repr(C)]
pub struct resource {
    _private: [u8; 0],
}
#[repr(C)]
pub struct platform_device {
    _private: [u8; 0],
}
#[repr(C)]
pub struct mutex {
    _private: [u8; 0],
}

/**
 * struct wilco_ec_device - Wilco Embedded Controller handle.
 * @dev: Device handle.
 * @mailbox_lock: Mutex to ensure one mailbox command at a time.
 * @io_command: I/O port for mailbox command.  Provided by ACPI.
 * @io_data: I/O port for mailbox data.  Provided by ACPI.
 * @io_packet: I/O port for mailbox packet data.  Provided by ACPI.
 * @data_buffer: Buffer used for EC communication.  The same buffer
 *               is used to hold the request and the response.
 * @data_size: Size of the data buffer used for EC communication.
 * @debugfs_pdev: The child platform_device used by the debugfs sub-driver.
 * @rtc_pdev: The child platform_device used by the RTC sub-driver.
 * @charger_pdev: Child platform_device used by the charger config sub-driver.
 * @telem_pdev: The child platform_device used by the telemetry sub-driver.
 */
#[repr(C)]
pub struct wilco_ec_device {
    pub dev: *mut device,
    pub mailbox_lock: mutex,
    pub io_command: *mut resource,
    pub io_data: *mut resource,
    pub io_packet: *mut resource,
    pub data_buffer: *mut core::ffi::c_void,
    pub data_size: usize,
    pub debugfs_pdev: *mut platform_device,
    pub rtc_pdev: *mut platform_device,
    pub charger_pdev: *mut platform_device,
    pub telem_pdev: *mut platform_device,
}

/** Mailbox request message format. */
#[repr(C, packed)]
pub struct wilco_ec_request {
    pub struct_version: u8,
    pub checksum: u8,
    pub mailbox_id: u16,
    pub mailbox_version: u8,
    pub reserved: u8,
    pub data_size: u16,
}

/** Mailbox response message format. */
#[repr(C, packed)]
pub struct wilco_ec_response {
    pub struct_version: u8,
    pub checksum: u8,
    pub result: u16,
    pub data_size: u16,
    pub reserved: [u8; 2],
    pub data: [u8; 0],
}

/** Message type to select a set of command codes. */
#[repr(u16)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum wilco_ec_msg_type {
    WILCO_EC_MSG_LEGACY = 0x00f0,
    WILCO_EC_MSG_PROPERTY = 0x00f2,
    WILCO_EC_MSG_TELEMETRY = 0x00f5,
}

/** Request and response message. */
#[repr(C)]
pub struct wilco_ec_message {
    pub type_: wilco_ec_msg_type,
    pub flags: u8,
    pub request_size: usize,
    pub request_data: *mut core::ffi::c_void,
    pub response_size: usize,
    pub response_data: *mut core::ffi::c_void,
}

/**
 * Property is typically a data item that is stored to NVRAM by the EC.
 */
pub const WILCO_EC_PROPERTY_MAX_SIZE: usize = 4;

/** Message to get or set a property. */
#[repr(C)]
pub struct wilco_ec_property_msg {
    pub property_id: u32,
    pub length: core::ffi::c_int,
    pub data: [u8; WILCO_EC_PROPERTY_MAX_SIZE],
}

extern "C" {
    pub fn wilco_ec_mailbox(
        ec: *mut wilco_ec_device,
        msg: *mut wilco_ec_message,
    ) -> core::ffi::c_int;

    pub fn wilco_keyboard_leds_init(ec: *mut wilco_ec_device) -> core::ffi::c_int;

    pub fn wilco_ec_get_property(
        ec: *mut wilco_ec_device,
        prop_msg: *mut wilco_ec_property_msg,
    ) -> core::ffi::c_int;

    pub fn wilco_ec_set_property(
        ec: *mut wilco_ec_device,
        prop_msg: *mut wilco_ec_property_msg,
    ) -> core::ffi::c_int;

    pub fn wilco_ec_get_byte_property(
        ec: *mut wilco_ec_device,
        property_id: u32,
        val: *mut u8,
    ) -> core::ffi::c_int;

    pub fn wilco_ec_set_byte_property(
        ec: *mut wilco_ec_device,
        property_id: u32,
        val: u8,
    ) -> core::ffi::c_int;

    pub fn wilco_ec_add_sysfs(ec: *mut wilco_ec_device) -> core::ffi::c_int;
    pub fn wilco_ec_remove_sysfs(ec: *mut wilco_ec_device);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
