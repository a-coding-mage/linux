/* SPDX-License-Identifier: GPL-2.0 */
/* Copyright 2024 Intel Corporation */

/* Dependency supplied by the surrounding translation unit: __le16. */

pub const HIDI2C_REG_LEN: usize = core::mem::size_of::<__le16>();

/* Input report type definition in HIDI2C protocol */
#[repr(C)]
pub enum hidi2c_report_type {
    HIDI2C_RESERVED = 0,
    HIDI2C_INPUT,
    HIDI2C_OUTPUT,
    HIDI2C_FEATURE,
}

/* Power state type definition in HIDI2C protocol */
#[repr(C)]
pub enum hidi2c_power_state {
    HIDI2C_ON,
    HIDI2C_SLEEP,
}

/* Opcode type definition in HIDI2C protocol */
#[repr(C)]
pub enum hidi2c_opcode {
    HIDI2C_RESET = 1,
    HIDI2C_GET_REPORT,
    HIDI2C_SET_REPORT,
    HIDI2C_GET_IDLE,
    HIDI2C_SET_IDLE,
    HIDI2C_GET_PROTOCOL,
    HIDI2C_SET_PROTOCOL,
    HIDI2C_SET_POWER,
}

/**
 * struct hidi2c_report_packet - Report packet definition in HIDI2C protocol
 * @len: data field length
 * @data: HIDI2C report packet data
 */
#[repr(C, packed)]
pub struct hidi2c_report_packet {
    pub len: __le16,
    pub data: [u8; 0],
}

pub const HIDI2C_LENGTH_LEN: usize = core::mem::size_of::<__le16>();

#[inline]
pub const fn HIDI2C_PACKET_LEN(data_len: usize) -> usize {
    data_len + HIDI2C_LENGTH_LEN
}

#[inline]
pub const fn HIDI2C_DATA_LEN(pkt_len: usize) -> usize {
    pkt_len - HIDI2C_LENGTH_LEN
}

pub const HIDI2C_CMD_MAX_RI: u32 = 0x0F;

/**
 * HIDI2C command data packet - Command packet definition in HIDI2C protocol
 * @report_id:		[0:3] report id (<15) for features or output reports
 * @report_type:	[4:5] indicate report type, reference to hidi2c_report_type
 * @reserved0:		[6:7] reserved bits
 * @opcode:		[8:11] command operation code, reference to hidi2c_opcode
 * @reserved1:		[12:15] reserved bits
 * @report_id_optional: [23:16] appended 3rd byte.
 *                      If the report_id in the low byte is set to the
 *                      sentinel value (HIDI2C_CMD_MAX_RI), then this
 *                      optional third byte represents the report id (>=15)
 *                      Otherwise, not this 3rd byte.
 */

pub const HIDI2C_CMD_LEN: usize = core::mem::size_of::<__le16>();
pub const HIDI2C_CMD_LEN_OPT: usize = core::mem::size_of::<__le16>() + 1;
pub const HIDI2C_CMD_REPORT_ID: u32 = 0x0000_000F;
pub const HIDI2C_CMD_REPORT_TYPE: u32 = 0x0000_0030;
pub const HIDI2C_CMD_OPCODE: u32 = 0x0000_0F00;
/* Duplicate definition in the source header retained as a comment. */
pub const HIDI2C_CMD_3RD_BYTE: u32 = 0x00FF_0000;

pub const HIDI2C_HID_DESC_BCDVERSION: u32 = 0x100;

/**
 * struct hidi2c_dev_descriptor - HIDI2C device descriptor definition
 * @dev_desc_len: The length of the complete device descriptor, fixed to 0x1E (30).
 * @bcd_ver: The version number of the HIDI2C protocol supported.
 *           In binary coded decimal (BCD) format.
 * @report_desc_len: The length of the report descriptor
 * @report_desc_reg: The register address to retrieve report descriptor
 * @input_reg: the register address to retrieve input report
 * @max_input_len: The length of the largest possible HID input (or feature) report
 * @output_reg: the register address to send output report
 * @max_output_len: The length of the largest output (or feature) report
 * @cmd_reg: the register address to send command
 * @data_reg: the register address to send command data
 * @vendor_id: Device manufacturers vendor ID
 * @product_id: Device unique model/product ID
 * @version_id: Device’s unique version
 * @reserved0: Reserved and should be 0
 * @reserved1: Reserved and should be 0
 */
#[repr(C, packed)]
pub struct hidi2c_dev_descriptor {
    pub dev_desc_len: __le16,
    pub bcd_ver: __le16,
    pub report_desc_len: __le16,
    pub report_desc_reg: __le16,
    pub input_reg: __le16,
    pub max_input_len: __le16,
    pub output_reg: __le16,
    pub max_output_len: __le16,
    pub cmd_reg: __le16,
    pub data_reg: __le16,
    pub vendor_id: __le16,
    pub product_id: __le16,
    pub version_id: __le16,
    pub reserved0: __le16,
    pub reserved1: __le16,
}

pub const HIDI2C_DEV_DESC_LEN: usize = core::mem::size_of::<hidi2c_dev_descriptor>();

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
