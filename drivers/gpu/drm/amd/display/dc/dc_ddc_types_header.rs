/*
 * Copyright 2012-15 Advanced Micro Devices, Inc.
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
 *
 * Authors: AMD
 */

#[repr(C)]
pub enum aux_transaction_type { AUX_TRANSACTION_TYPE_DP, AUX_TRANSACTION_TYPE_I2C }

#[repr(C)]
pub enum i2caux_transaction_action {
    I2CAUX_TRANSACTION_ACTION_I2C_WRITE = 0x00,
    I2CAUX_TRANSACTION_ACTION_I2C_READ = 0x10,
    I2CAUX_TRANSACTION_ACTION_I2C_STATUS_REQUEST = 0x20,
    I2CAUX_TRANSACTION_ACTION_I2C_WRITE_MOT = 0x40,
    I2CAUX_TRANSACTION_ACTION_I2C_READ_MOT = 0x50,
    I2CAUX_TRANSACTION_ACTION_I2C_STATUS_REQUEST_MOT = 0x60,
    I2CAUX_TRANSACTION_ACTION_DP_WRITE = 0x80,
    I2CAUX_TRANSACTION_ACTION_DP_READ = 0x90,
}

#[repr(C)]
pub struct aux_request_transaction_data {
    pub type_: aux_transaction_type,
    pub action: i2caux_transaction_action,
    pub address: u32,
    pub delay: u8,
    pub length: u32,
    pub data: *mut u8,
}

#[repr(C)]
pub enum aux_transaction_reply {
    AUX_TRANSACTION_REPLY_AUX_ACK = 0x00,
    AUX_TRANSACTION_REPLY_AUX_NACK = 0x01,
    AUX_TRANSACTION_REPLY_AUX_DEFER = 0x02,
    AUX_TRANSACTION_REPLY_I2C_OVER_AUX_NACK = 0x04,
    AUX_TRANSACTION_REPLY_I2C_OVER_AUX_DEFER = 0x08,
    AUX_TRANSACTION_REPLY_I2C_ACK = 0x00,
    AUX_TRANSACTION_REPLY_I2C_NACK = 0x10,
    AUX_TRANSACTION_REPLY_I2C_DEFER = 0x20,
    AUX_TRANSACTION_REPLY_HPD_DISCON = 0x40,
    AUX_TRANSACTION_REPLY_INVALID = 0xFF,
}

#[repr(C)]
pub struct aux_reply_transaction_data { pub status: aux_transaction_reply, pub length: u32, pub data: *mut u8 }

#[repr(C)]
pub struct aux_payload {
    pub i2c_over_aux: bool,
    pub write: bool,
    pub mot: bool,
    pub write_status_update: bool,
    pub address: u32,
    pub length: u32,
    pub data: *mut u8,
    pub reply: *mut u8,
    pub defer_delay: u32,
}

pub const DEFAULT_AUX_MAX_DATA_SIZE: u32 = 16;

#[repr(C)]
pub struct i2c_payload { pub write: bool, pub address: u8, pub length: u32, pub data: *mut u8 }

#[repr(C)]
pub enum i2c_command_engine { I2C_COMMAND_ENGINE_DEFAULT, I2C_COMMAND_ENGINE_SW, I2C_COMMAND_ENGINE_HW }

pub const DDC_I2C_COMMAND_ENGINE: i2c_command_engine = i2c_command_engine::I2C_COMMAND_ENGINE_SW;

#[repr(C)]
pub struct i2c_command {
    pub payloads: *mut i2c_payload,
    pub number_of_payloads: u8,
    pub engine: i2c_command_engine,
    pub speed: u32,
}

#[repr(C)]
pub struct gpio_ddc_hw_info { pub hw_supported: bool, pub ddc_channel: u32 }

#[repr(C)]
pub struct ddc { pub pin_data: *mut gpio, pub pin_clock: *mut gpio, pub hw_info: gpio_ddc_hw_info, pub ctx: *mut dc_context }

#[repr(C)]
pub struct ddc_wa_bits { pub DP_SKIP_POWER_OFF: u32, pub DP_AUX_POWER_UP_WA_DELAY: u32 }

#[repr(C)]
pub union ddc_wa { pub bits: ddc_wa_bits, pub raw: u32 }

#[repr(C)]
pub struct ddc_flags {
    pub EDID_QUERY_DONE_ONCE: u8,
    pub IS_INTERNAL_DISPLAY: u8,
    pub FORCE_READ_REPEATED_START: u8,
    pub EDID_STRESS_READ: u8,
}

#[repr(C)]
pub enum ddc_transaction_type {
    DDC_TRANSACTION_TYPE_NONE = 0,
    DDC_TRANSACTION_TYPE_I2C,
    DDC_TRANSACTION_TYPE_I2C_OVER_AUX,
    DDC_TRANSACTION_TYPE_I2C_OVER_AUX_WITH_DEFER,
    DDC_TRANSACTION_TYPE_I2C_OVER_AUX_RETRY_DEFER,
}

#[repr(C)]
pub enum display_dongle_type {
    DISPLAY_DONGLE_NONE = 0,
    DISPLAY_DONGLE_DP_VGA_CONVERTER,
    DISPLAY_DONGLE_DP_DVI_CONVERTER,
    DISPLAY_DONGLE_DP_HDMI_CONVERTER,
    DISPLAY_DONGLE_DP_DVI_DONGLE,
    DISPLAY_DONGLE_DP_HDMI_DONGLE,
    DISPLAY_DONGLE_DP_HDMI_MISMATCHED_DONGLE,
}

pub const DC_MAX_EDID_BUFFER_SIZE: usize = 2048;
pub const DC_EDID_BLOCK_SIZE: usize = 128;

#[repr(C)]
pub struct ddc_service {
    pub ddc_pin: *mut ddc,
    pub flags: ddc_flags,
    pub wa: ddc_wa,
    pub transaction_type: ddc_transaction_type,
    pub dongle_type: display_dongle_type,
    pub ctx: *mut dc_context,
    pub link: *mut dc_link,
    pub address: u32,
    pub edid_buf_len: u32,
    pub edid_buf: [u8; DC_MAX_EDID_BUFFER_SIZE],
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
