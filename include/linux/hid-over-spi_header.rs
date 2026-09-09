/* SPDX-License-Identifier: GPL-2.0 */
/* Copyright 2024 Intel Corporation */

// Input report type definition in HIDSPI protocol
#[repr(i32)]
pub enum input_report_type {
    INVALID_INPUT_REPORT_TYPE_0 = 0,
    DATA = 1,
    INVALID_TYPE_2 = 2,
    RESET_RESPONSE = 3,
    COMMAND_RESPONSE = 4,
    GET_FEATURE_RESPONSE = 5,
    INVALID_TYPE_6 = 6,
    DEVICE_DESCRIPTOR_RESPONSE = 7,
    REPORT_DESCRIPTOR_RESPONSE = 8,
    SET_FEATURE_RESPONSE = 9,
    OUTPUT_REPORT_RESPONSE = 10,
    GET_INPUT_REPORT_RESPONSE = 11,
    INVALID_INPUT_REPORT_TYPE = 0xF,
}

// Output report type definition in HIDSPI protocol
#[repr(i32)]
pub enum output_report_type {
    INVALID_OUTPUT_REPORT_TYPE_0 = 0,
    DEVICE_DESCRIPTOR = 1,
    REPORT_DESCRIPTOR = 2,
    SET_FEATURE = 3,
    GET_FEATURE = 4,
    OUTPUT_REPORT = 5,
    GET_INPUT_REPORT = 6,
    COMMAND_CONTENT = 7,
}

// Set power command ID for output report
pub const HIDSPI_SET_POWER_CMD_ID: u32 = 1;

// Power state definition in HIDSPI protocol
#[repr(i32)]
pub enum hidspi_power_state {
    HIDSPI_ON = 1,
    HIDSPI_SLEEP = 2,
    HIDSPI_OFF = 3,
}

/**
 * Input report header definition in HIDSPI protocol
 * Report header size is 32bits, it includes:
 * protocol_ver:     [0:3] Current supported HIDSPI protocol version, must be 0x3
 * reserved0:        [4:7] Reserved bits
 * input_report_len: [8:21] Input report length in number bytes divided by 4
 * last_frag_flag:   [22]Indicate if this packet is last fragment.
 *                       1 - indicates last fragment
 *                       0 - indicates additional fragments
 * reserved1:        [23] Reserved bits
 * @sync_const:      [24:31] Used to validate input report header, must be 0x5A
 */
pub const HIDSPI_INPUT_HEADER_SIZE: usize = core::mem::size_of::<u32>();
pub const HIDSPI_INPUT_HEADER_VER: u32 = 0xF;
pub const HIDSPI_INPUT_HEADER_REPORT_LEN: u32 = 0x003F_F00;
pub const HIDSPI_INPUT_HEADER_LAST_FLAG: u32 = 1 << 22;
pub const HIDSPI_INPUT_HEADER_SYNC: u32 = 0xFF00_0000;

/**
 * struct input_report_body_header - Input report body header definition in HIDSPI protocol
 * @input_report_type: indicate input report type, reference to enum input_report_type
 * @content_len: this input report body packet length
 * @content_id: indicate this input report's report id
 */
#[repr(C, packed)]
pub struct input_report_body_header {
    pub input_report_type: u8,
    pub content_len: u16,
    pub content_id: u8,
}

pub const HIDSPI_INPUT_BODY_HEADER_SIZE: usize = core::mem::size_of::<input_report_body_header>();

/**
 * struct input_report_body - Input report body definition in HIDSPI protocol
 * @body_hdr: input report body header
 * @content: input report body content
 */
#[repr(C, packed)]
pub struct input_report_body {
    pub body_hdr: input_report_body_header,
    pub content: [u8; 0],
}

pub const fn HIDSPI_INPUT_BODY_SIZE(content_len: usize) -> usize {
    content_len + HIDSPI_INPUT_BODY_HEADER_SIZE
}

/**
 * struct output_report_header - Output report header definition in HIDSPI protocol
 * @report_type: output report type, reference to enum output_report_type
 * @content_len: length of content
 * @content_id: 0x00 - descriptors
 *              report id - Set/Feature feature or Input/Output Reports
 *              command opcode - for commands
 */
#[repr(C, packed)]
pub struct output_report_header {
    pub report_type: u8,
    pub content_len: u16,
    pub content_id: u8,
}

pub const HIDSPI_OUTPUT_REPORT_HEADER_SIZE: usize = core::mem::size_of::<output_report_header>();

/**
 * struct output_report - Output report definition in HIDSPI protocol
 * @output_hdr: output report header
 * @content: output report content
 */
#[repr(C, packed)]
pub struct output_report {
    pub output_hdr: output_report_header,
    pub content: [u8; 0],
}

pub const fn HIDSPI_OUTPUT_REPORT_SIZE(content_len: usize) -> usize {
    content_len + HIDSPI_OUTPUT_REPORT_HEADER_SIZE
}

/**
 * struct hidspi_dev_descriptor - HIDSPI device descriptor definition
 * @dev_desc_len: The length of the complete device descriptor, fixed to 0x18 (24).
 * @bcd_ver: The version number of the HIDSPI protocol supported.
 *           In binary coded decimal (BCD) format. Must be fixed to 0x0300.
 * @rep_desc_len: The length of the report descriptor
 * @max_input_len: The length of the largest possible HID input (or feature) report
 * @max_output_len: The length of the largest output (or feature) report
 * @max_frag_len: The length of the largest fragment, where a fragment represents
 *                the body of an input report.
 * @vendor_id: Device manufacturers vendor ID
 * @product_id: Device unique model/product ID
 * @version_id: Device’s unique version
 * @flags: Specify flags for the device’s operation
 * @reserved: Reserved and should be 0
 */
#[repr(C)]
pub struct hidspi_dev_descriptor {
    pub dev_desc_len: u16,
    pub bcd_ver: u16,
    pub rep_desc_len: u16,
    pub max_input_len: u16,
    pub max_output_len: u16,
    pub max_frag_len: u16,
    pub vendor_id: u16,
    pub product_id: u16,
    pub version_id: u16,
    pub flags: u16,
    pub reserved: u32,
}

pub const HIDSPI_DEVICE_DESCRIPTOR_SIZE: usize = core::mem::size_of::<hidspi_dev_descriptor>();
pub const HIDSPI_INPUT_DEVICE_DESCRIPTOR_SIZE: usize =
    HIDSPI_INPUT_BODY_HEADER_SIZE + HIDSPI_DEVICE_DESCRIPTOR_SIZE;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
