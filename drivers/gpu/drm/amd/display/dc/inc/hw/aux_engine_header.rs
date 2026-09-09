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

// Dependency supplied by dc_ddc_types.h.
#[repr(C)] pub enum aux_return_code_type {}
#[repr(C)] pub struct ddc;
#[repr(C)] pub struct dc_context;
#[repr(C)] pub struct ddc_service;
#[repr(C)] pub struct aux_request_transaction_data;
#[repr(C)] pub struct aux_reply_transaction_data;

#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum i2caux_transaction_operation {
    I2CAUX_TRANSACTION_READ = 0,
    I2CAUX_TRANSACTION_WRITE = 1,
}

#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum i2caux_transaction_address_space {
    I2CAUX_TRANSACTION_ADDRESS_SPACE_I2C = 1,
    I2CAUX_TRANSACTION_ADDRESS_SPACE_DPCD = 2,
}

#[repr(C)]
pub struct i2caux_transaction_payload {
    pub address_space: i2caux_transaction_address_space,
    pub address: u32,
    pub length: u32,
    pub data: *mut u8,
}

#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum i2caux_transaction_status {
    I2CAUX_TRANSACTION_STATUS_UNKNOWN = -1,
    I2CAUX_TRANSACTION_STATUS_SUCCEEDED = 0,
    I2CAUX_TRANSACTION_STATUS_FAILED_CHANNEL_BUSY,
    I2CAUX_TRANSACTION_STATUS_FAILED_TIMEOUT,
    I2CAUX_TRANSACTION_STATUS_FAILED_PROTOCOL_ERROR,
    I2CAUX_TRANSACTION_STATUS_FAILED_NACK,
    I2CAUX_TRANSACTION_STATUS_FAILED_INCOMPLETE,
    I2CAUX_TRANSACTION_STATUS_FAILED_OPERATION,
    I2CAUX_TRANSACTION_STATUS_FAILED_INVALID_OPERATION,
    I2CAUX_TRANSACTION_STATUS_FAILED_BUFFER_OVERFLOW,
    I2CAUX_TRANSACTION_STATUS_FAILED_HPD_DISCON,
}

#[repr(C)]
pub struct i2caux_transaction_request {
    pub operation: i2caux_transaction_operation,
    pub payload: i2caux_transaction_payload,
    pub status: i2caux_transaction_status,
}

#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum i2caux_engine_type {
    I2CAUX_ENGINE_TYPE_UNKNOWN = -1,
    I2CAUX_ENGINE_TYPE_AUX = 0,
    I2CAUX_ENGINE_TYPE_I2C_DDC_HW,
    I2CAUX_ENGINE_TYPE_I2C_GENERIC_HW,
    I2CAUX_ENGINE_TYPE_I2C_SW,
}

#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum i2c_default_speed {
    I2CAUX_DEFAULT_I2C_HW_SPEED = 50,
    I2CAUX_DEFAULT_I2C_SW_SPEED = 50,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct aux_config_bits {
    pub ALLOW_AUX_WHEN_HPD_LOW: u32,
}

#[repr(C)]
pub union aux_config {
    pub bits: aux_config_bits,
    pub raw: u32,
}

#[repr(C)]
pub struct aux_engine {
    pub inst: u32,
    pub ddc: *mut ddc,
    pub ctx: *mut dc_context,
    pub funcs: *const aux_engine_funcs,
    /* following values are expressed in milliseconds */
    pub delay: u32,
    pub max_defer_write_retry: u32,
    pub acquire_reset: bool,
}

#[repr(C)]
pub struct read_command_context {
    pub buffer: *mut u8,
    pub current_read_length: u32,
    pub offset: u32,
    pub status: i2caux_transaction_status,
    pub request: aux_request_transaction_data,
    pub reply: aux_reply_transaction_data,
    pub returned_byte: u8,
    pub timed_out_retry_aux: u32,
    pub invalid_reply_retry_aux: u32,
    pub defer_retry_aux: u32,
    pub defer_retry_i2c: u32,
    pub invalid_reply_retry_aux_on_ack: u32,
    pub transaction_complete: bool,
    pub operation_succeeded: bool,
}

#[repr(C)]
pub struct write_command_context {
    pub mot: bool,
    pub buffer: *mut u8,
    pub current_write_length: u32,
    pub status: i2caux_transaction_status,
    pub request: aux_request_transaction_data,
    pub reply: aux_reply_transaction_data,
    pub returned_byte: u8,
    pub timed_out_retry_aux: u32,
    pub invalid_reply_retry_aux: u32,
    pub defer_retry_aux: u32,
    pub defer_retry_i2c: u32,
    pub max_defer_retry: u32,
    pub ack_m_retry: u32,
    // DEFAULT_AUX_MAX_DATA_SIZE is supplied by the dependent header.
    pub reply_data: [u8; 16],
    pub transaction_complete: bool,
    pub operation_succeeded: bool,
}

#[repr(C)]
pub struct aux_engine_funcs {
    pub configure_timeout: Option<unsafe extern "C" fn(ddc: *mut ddc_service, timeout: u32) -> bool>,
    pub destroy: Option<unsafe extern "C" fn(ptr: *mut *mut aux_engine)>,
    pub acquire_engine: Option<unsafe extern "C" fn(engine: *mut aux_engine) -> bool>,
    pub configure: Option<unsafe extern "C" fn(engine: *mut aux_engine, cfg: aux_config)>,
    pub submit_channel_request: Option<unsafe extern "C" fn(engine: *mut aux_engine, request: *mut aux_request_transaction_data)>,
    pub process_channel_reply: Option<unsafe extern "C" fn(engine: *mut aux_engine, reply: *mut aux_reply_transaction_data)>,
    pub read_channel_reply: Option<unsafe extern "C" fn(engine: *mut aux_engine, size: u32, buffer: *mut u8, reply_result: *mut u8, sw_status: *mut u32) -> i32>,
    pub get_channel_status: Option<unsafe extern "C" fn(engine: *mut aux_engine, returned_bytes: *mut u8) -> aux_return_code_type>,
    pub is_engine_available: Option<unsafe extern "C" fn(engine: *mut aux_engine) -> bool>,
    pub acquire: Option<unsafe extern "C" fn(engine: *mut aux_engine, ddc: *mut ddc) -> bool>,
    pub submit_request: Option<unsafe extern "C" fn(engine: *mut aux_engine, request: *mut i2caux_transaction_request, middle_of_transaction: bool) -> bool>,
    pub release_engine: Option<unsafe extern "C" fn(engine: *mut aux_engine)>,
    pub destroy_engine: Option<unsafe extern "C" fn(engine: *mut *mut aux_engine)>,
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
