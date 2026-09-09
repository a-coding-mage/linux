/*
 * Copyright 2019 Advanced Micro Devices, Inc.
 *
 * Permission is hereby granted, free of charge, to any person obtaining a
 * copy of this software and associated documentation files (the "Software"),
 * to deal in the Software without restriction, including without limitation
 * the rights to use, copy, modify, merge, publish, distribute, sublicense,
 * and/or sell copies of the Software, and to permit persons to whom the
 * Software is furnished to do so, subject to the following conditions:
 *
 * The above copyright notice and this permission notice shall be included in
 * all copies or substantial portions of the Software.
 *
 * THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
 * IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
 * FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT.  IN NO EVENT SHALL
 * THE COPYRIGHT HOLDER(S) OR AUTHOR(S) BE LIABLE FOR ANY CLAIM, DAMAGES OR
 * OTHER LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE,
 * ARISING FROM, OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR
 * OTHER DEALINGS IN THE SOFTWARE.
 */

/** Secure Display related enumerations */

/** Secure Display Command ID */
#[repr(u32)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum ta_securedisplay_command {
    /// Query whether TA is responding. It is used only for validation purpose
    TA_SECUREDISPLAY_COMMAND__QUERY_TA = 1,
    /// Send region of Interest and CRC value to I2C
    TA_SECUREDISPLAY_COMMAND__SEND_ROI_CRC = 2,
    /// V2 to send multiple regions of Interest and CRC value to I2C
    TA_SECUREDISPLAY_COMMAND__SEND_ROI_CRC_V2 = 3,
    /// Maximum Command ID
    TA_SECUREDISPLAY_COMMAND__MAX_ID = 0x7FFFFFFF,
}

/** Secure Display status returns in shared buffer status */
#[repr(u32)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum ta_securedisplay_status {
    TA_SECUREDISPLAY_STATUS__SUCCESS = 0x00,
    TA_SECUREDISPLAY_STATUS__GENERIC_FAILURE = 0x01,
    TA_SECUREDISPLAY_STATUS__INVALID_PARAMETER = 0x02,
    TA_SECUREDISPLAY_STATUS__NULL_POINTER = 0x03,
    TA_SECUREDISPLAY_STATUS__I2C_WRITE_ERROR = 0x04,
    TA_SECUREDISPLAY_STATUS__READ_DIO_SCRATCH_ERROR = 0x05,
    TA_SECUREDISPLAY_STATUS__READ_CRC_ERROR = 0x06,
    TA_SECUREDISPLAY_STATUS__I2C_INIT_ERROR = 0x07,
    TA_SECUREDISPLAY_STATUS__MAX = 0x7FFFFFFF,
}

/** Physical ID number to use for reading corresponding DIO Scratch register for ROI */
#[repr(u32)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum ta_securedisplay_phy_ID {
    TA_SECUREDISPLAY_PHY0 = 0,
    TA_SECUREDISPLAY_PHY1 = 1,
    TA_SECUREDISPLAY_PHY2 = 2,
    TA_SECUREDISPLAY_PHY3 = 3,
    TA_SECUREDISPLAY_MAX_PHY = 4,
}

/** A predefined specific return value used to validate communication to Secure Display TA. */
pub const TA_SECUREDISPLAY_QUERY_CMD_RET: u32 = 0xAB;

/** I2C buffer sizes. */
pub const TA_SECUREDISPLAY_I2C_BUFFER_SIZE: usize = 15;
pub const TA_SECUREDISPLAY_V2_I2C_BUFFER_SIZE: usize = 16;

/** Input/output structures for Secure Display commands */

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ta_securedisplay_send_roi_crc_input {
    pub phy_id: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ta_securedisplay_send_roi_crc_v2_input {
    pub phy_id: u32,
    pub roi_idx: u8,
}

#[repr(C)]
pub union ta_securedisplay_cmd_input {
    pub send_roi_crc: ta_securedisplay_send_roi_crc_input,
    pub send_roi_crc_v2: ta_securedisplay_send_roi_crc_v2_input,
    pub reserved: [u32; 4],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ta_securedisplay_query_ta_output {
    pub query_cmd_ret: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ta_securedisplay_send_roi_crc_output {
    pub i2c_buf: [u8; TA_SECUREDISPLAY_I2C_BUFFER_SIZE],
    pub reserved: u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ta_securedisplay_send_roi_crc_v2_output {
    pub i2c_buf: [u8; TA_SECUREDISPLAY_V2_I2C_BUFFER_SIZE],
}

#[repr(C)]
pub union ta_securedisplay_cmd_output {
    pub query_ta: ta_securedisplay_query_ta_output,
    pub send_roi_crc: ta_securedisplay_send_roi_crc_output,
    pub send_roi_crc_v2: ta_securedisplay_send_roi_crc_v2_output,
    pub reserved: [u32; 4],
}

/** Secure display command which is shared buffer memory */
#[repr(C)]
pub struct ta_securedisplay_cmd {
    /** +0 Bytes Command ID */
    pub cmd_id: u32,
    /** +4 Bytes Status code returned by the secure display TA */
    pub status: ta_securedisplay_status,
    /** +8 Bytes Reserved */
    pub reserved: [u32; 2],
    /** +16 Bytes Command input buffer */
    pub securedisplay_in_message: ta_securedisplay_cmd_input,
    /** +32 Bytes Command output buffer */
    pub securedisplay_out_message: ta_securedisplay_cmd_output,
    /** Total 48 Bytes */
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
