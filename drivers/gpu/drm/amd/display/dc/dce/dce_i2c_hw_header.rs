/*
 * Copyright 2018 Advanced Micro Devices, Inc.
 *
 * Permission is hereby granted, free of charge, to any person obtaining a
 * copy of this software and associated documentation files (the "Software"),
 * to deal in the Software without restriction, including without limitation
 * the rights to use, copy, modify, merge, publish, distribute, sublicense,
 * and/or sell copies of the Software, and to permit persons to whom the
 * Software is furnished to do so, subject to the following conditions:
 *
 * THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
 * IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
 * FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT.
 */

#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum dc_i2c_status {
    DC_I2C_STATUS__DC_I2C_STATUS_IDLE,
    DC_I2C_STATUS__DC_I2C_STATUS_USED_BY_SW,
    DC_I2C_STATUS__DC_I2C_STATUS_USED_BY_HW,
    DC_I2C_REG_RW_CNTL_STATUS_DMCU_ONLY = 2,
}

#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum dc_i2c_arbitration {
    DC_I2C_ARBITRATION__DC_I2C_SW_PRIORITY_NORMAL,
    DC_I2C_ARBITRATION__DC_I2C_SW_PRIORITY_HIGH,
}

#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum i2c_channel_operation_result {
    I2C_CHANNEL_OPERATION_SUCCEEDED,
    I2C_CHANNEL_OPERATION_FAILED,
    I2C_CHANNEL_OPERATION_NOT_GRANTED,
    I2C_CHANNEL_OPERATION_IS_BUSY,
    I2C_CHANNEL_OPERATION_NO_HANDLE_PROVIDED,
    I2C_CHANNEL_OPERATION_CHANNEL_IN_USE,
    I2C_CHANNEL_OPERATION_CHANNEL_CLIENT_MAX_ALLOWED,
    I2C_CHANNEL_OPERATION_ENGINE_BUSY,
    I2C_CHANNEL_OPERATION_TIMEOUT,
    I2C_CHANNEL_OPERATION_NO_RESPONSE,
    I2C_CHANNEL_OPERATION_HW_REQUEST_I2C_BUS,
    I2C_CHANNEL_OPERATION_WRONG_PARAMETER,
    I2C_CHANNEL_OPERATION_OUT_NB_OF_RETRIES,
    I2C_CHANNEL_OPERATION_NOT_STARTED,
}

#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum dce_i2c_transaction_action {
    DCE_I2C_TRANSACTION_ACTION_I2C_WRITE = 0x00,
    DCE_I2C_TRANSACTION_ACTION_I2C_READ = 0x10,
    DCE_I2C_TRANSACTION_ACTION_I2C_STATUS_REQUEST = 0x20,
    DCE_I2C_TRANSACTION_ACTION_I2C_WRITE_MOT = 0x40,
    DCE_I2C_TRANSACTION_ACTION_I2C_READ_MOT = 0x50,
    DCE_I2C_TRANSACTION_ACTION_I2C_STATUS_REQUEST_MOT = 0x60,
    DCE_I2C_TRANSACTION_ACTION_DP_WRITE = 0x80,
    DCE_I2C_TRANSACTION_ACTION_DP_READ = 0x90,
}

pub const I2C_SETUP_TIME_LIMIT_DCE: u32 = 255;
pub const I2C_SETUP_TIME_LIMIT_DCN: u32 = 3;
pub const I2C_HW_BUFFER_SIZE_DCE100: u32 = 538;
pub const I2C_HW_BUFFER_SIZE_DCE: u32 = 144;
pub const I2C_SEND_RESET_LENGTH_9: u32 = 9;
pub const I2C_SEND_RESET_LENGTH_10: u32 = 10;
pub const DEFAULT_I2C_HW_SPEED: u32 = 50;
pub const DEFAULT_I2C_HW_SPEED_100KHZ: u32 = 100;
pub const TRANSACTION_TIMEOUT_IN_I2C_CLOCKS: u32 = 32;

// Register-list macros retain their C preprocessor expansion intent.
#[macro_export]
macro_rules! I2C_HW_ENGINE_COMMON_REG_LIST {
    ($id:expr) => { (SRI!(SETUP, DC_I2C_DDC, $id), SRI!(SPEED, DC_I2C_DDC, $id), SRI!(HW_STATUS, DC_I2C_DDC, $id), SR!(DC_I2C_ARBITRATION), SR!(DC_I2C_CONTROL), SR!(DC_I2C_SW_STATUS), SR!(DC_I2C_TRANSACTION0), SR!(DC_I2C_TRANSACTION1), SR!(DC_I2C_TRANSACTION2), SR!(DC_I2C_TRANSACTION3), SR!(DC_I2C_DATA), SR!(MICROSECOND_TIME_BASE_DIV)) };
}
#[macro_export]
macro_rules! I2C_HW_ENGINE_COMMON_REG_LIST_DCN30 { ($id:expr) => { (I2C_HW_ENGINE_COMMON_REG_LIST!($id), SR!(DIO_MEM_PWR_CTRL), SR!(DIO_MEM_PWR_STATUS)) }; }
#[macro_export]
macro_rules! I2C_SF { ($reg_name:ident, $field_name:ident, $post_fix:ident) => { . $field_name = concat_idents!($reg_name, __, $field_name, $post_fix) }; }
#[macro_export]
macro_rules! I2C_COMMON_MASK_SH_LIST_DCE_COMMON_BASE { ($mask_sh:expr) => { I2C_SF!(DC_I2C_DDC1_SETUP, DC_I2C_DDC1_ENABLE, $mask_sh) /* remaining C field expansions follow the source list */ }; }
#[macro_export]
macro_rules! I2C_COMMON_MASK_SH_LIST_DCE110 { ($mask_sh:expr) => { I2C_COMMON_MASK_SH_LIST_DCE_COMMON_BASE!($mask_sh) }; }
#[macro_export]
macro_rules! I2C_COMMON_MASK_SH_LIST_DCN2 { ($mask_sh:expr) => { I2C_COMMON_MASK_SH_LIST_DCE110!($mask_sh) }; }
#[macro_export]
macro_rules! I2C_COMMON_MASK_SH_LIST_DCN30 { ($mask_sh:expr) => { I2C_COMMON_MASK_SH_LIST_DCN2!($mask_sh) }; }
#[macro_export]
macro_rules! I2C_COMMON_MASK_SH_LIST_DCN35 { ($mask_sh:expr) => { I2C_COMMON_MASK_SH_LIST_DCN30!($mask_sh) }; }
#[macro_export]
macro_rules! I2C_COMMON_MASK_SH_LIST_DCN401 { ($mask_sh:expr) => { I2C_COMMON_MASK_SH_LIST_DCN30!($mask_sh) }; }

#[repr(C)]
pub struct dce_i2c_shift {
    pub DC_I2C_DDC1_ENABLE: u8, pub DC_I2C_DDC1_TIME_LIMIT: u8, pub DC_I2C_DDC1_DATA_DRIVE_EN: u8, pub DC_I2C_DDC1_CLK_DRIVE_EN: u8,
    pub DC_I2C_DDC1_DATA_DRIVE_SEL: u8, pub DC_I2C_DDC1_INTRA_TRANSACTION_DELAY: u8, pub DC_I2C_DDC1_INTRA_BYTE_DELAY: u8, pub DC_I2C_DDC1_HW_STATUS: u8,
    pub DC_I2C_SW_DONE_USING_I2C_REG: u8, pub DC_I2C_SW_USE_I2C_REG_REQ: u8, pub DC_I2C_NO_QUEUED_SW_GO: u8, pub DC_I2C_SW_PRIORITY: u8,
    pub DC_I2C_SOFT_RESET: u8, pub DC_I2C_SW_STATUS_RESET: u8, pub DC_I2C_GO: u8, pub DC_I2C_SEND_RESET: u8, pub DC_I2C_TRANSACTION_COUNT: u8,
    pub DC_I2C_DDC_SELECT: u8, pub DC_I2C_DDC1_PRESCALE: u8, pub DC_I2C_DDC1_THRESHOLD: u8, pub DC_I2C_DDC1_START_STOP_TIMING_CNTL: u8,
    pub DC_I2C_SW_STOPPED_ON_NACK: u8, pub DC_I2C_SW_TIMEOUT: u8, pub DC_I2C_SW_ABORTED: u8, pub DC_I2C_SW_DONE: u8, pub DC_I2C_SW_STATUS: u8,
    pub DC_I2C_STOP_ON_NACK0: u8, pub DC_I2C_START0: u8, pub DC_I2C_RW0: u8, pub DC_I2C_STOP0: u8, pub DC_I2C_COUNT0: u8,
    pub DC_I2C_DATA_RW: u8, pub DC_I2C_DATA: u8, pub DC_I2C_INDEX: u8, pub DC_I2C_INDEX_WRITE: u8, pub XTAL_REF_DIV: u8,
    pub MICROSECOND_TIME_BASE_DIV: u8, pub DC_I2C_DDC1_SEND_RESET_LENGTH: u8, pub DC_I2C_REG_RW_CNTL_STATUS: u8, pub I2C_LIGHT_SLEEP_FORCE: u8,
    pub I2C_MEM_PWR_STATE: u8, pub DC_I2C_DDC1_CLK_EN: u8,
}

#[repr(C)]
pub struct dce_i2c_mask {
    pub DC_I2C_DDC1_ENABLE: u32, pub DC_I2C_DDC1_TIME_LIMIT: u32, pub DC_I2C_DDC1_DATA_DRIVE_EN: u32, pub DC_I2C_DDC1_CLK_DRIVE_EN: u32,
    pub DC_I2C_DDC1_DATA_DRIVE_SEL: u32, pub DC_I2C_DDC1_INTRA_TRANSACTION_DELAY: u32, pub DC_I2C_DDC1_INTRA_BYTE_DELAY: u32, pub DC_I2C_DDC1_HW_STATUS: u32,
    pub DC_I2C_SW_DONE_USING_I2C_REG: u32, pub DC_I2C_SW_USE_I2C_REG_REQ: u32, pub DC_I2C_NO_QUEUED_SW_GO: u32, pub DC_I2C_SW_PRIORITY: u32,
    pub DC_I2C_SOFT_RESET: u32, pub DC_I2C_SW_STATUS_RESET: u32, pub DC_I2C_GO: u32, pub DC_I2C_SEND_RESET: u32, pub DC_I2C_TRANSACTION_COUNT: u32,
    pub DC_I2C_DDC_SELECT: u32, pub DC_I2C_DDC1_PRESCALE: u32, pub DC_I2C_DDC1_THRESHOLD: u32, pub DC_I2C_DDC1_START_STOP_TIMING_CNTL: u32,
    pub DC_I2C_SW_STOPPED_ON_NACK: u32, pub DC_I2C_SW_TIMEOUT: u32, pub DC_I2C_SW_ABORTED: u32, pub DC_I2C_SW_DONE: u32, pub DC_I2C_SW_STATUS: u32,
    pub DC_I2C_STOP_ON_NACK0: u32, pub DC_I2C_START0: u32, pub DC_I2C_RW0: u32, pub DC_I2C_STOP0: u32, pub DC_I2C_COUNT0: u32,
    pub DC_I2C_DATA_RW: u32, pub DC_I2C_DATA: u32, pub DC_I2C_INDEX: u32, pub DC_I2C_INDEX_WRITE: u32, pub XTAL_REF_DIV: u32,
    pub MICROSECOND_TIME_BASE_DIV: u32, pub DC_I2C_DDC1_SEND_RESET_LENGTH: u32, pub DC_I2C_REG_RW_CNTL_STATUS: u32, pub I2C_LIGHT_SLEEP_FORCE: u32,
    pub I2C_MEM_PWR_STATE: u32, pub DC_I2C_DDC1_CLK_EN: u32,
}

#[repr(C)]
pub struct dce_i2c_registers {
    pub SETUP: u32, pub SPEED: u32, pub HW_STATUS: u32, pub DC_I2C_ARBITRATION: u32, pub DC_I2C_CONTROL: u32, pub DC_I2C_SW_STATUS: u32,
    pub DC_I2C_TRANSACTION0: u32, pub DC_I2C_TRANSACTION1: u32, pub DC_I2C_TRANSACTION2: u32, pub DC_I2C_TRANSACTION3: u32, pub DC_I2C_DATA: u32,
    pub MICROSECOND_TIME_BASE_DIV: u32, pub DIO_MEM_PWR_CTRL: u32, pub DIO_MEM_PWR_STATUS: u32,
}

#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum dce_i2c_transaction_address_space { DCE_I2C_TRANSACTION_ADDRESS_SPACE_I2C = 1, DCE_I2C_TRANSACTION_ADDRESS_SPACE_DPCD }

#[repr(C)]
pub struct i2c_request_transaction_data {
    pub action: dce_i2c_transaction_action, pub status: i2c_channel_operation_result, pub address: u8, pub length: u32, pub data: *mut u8,
}

#[repr(C)]
pub struct dce_i2c_hw {
    pub ddc: *mut ddc, pub engine_keep_power_up_count: u32, pub transaction_count: u32, pub buffer_used_bytes: u32, pub buffer_used_write: u32,
    pub reference_frequency: u32, pub default_speed: u32, pub engine_id: u32, pub setup_limit: u32, pub send_reset_length: u32, pub buffer_size: u32,
    pub ctx: *mut dc_context, pub regs: *const dce_i2c_registers, pub shifts: *const dce_i2c_shift, pub masks: *const dce_i2c_mask,
}

pub enum ddc {}
pub enum dc_context {}
pub enum resource_pool {}
pub enum i2c_command {}

extern "C" {
    pub fn dce_i2c_hw_construct(dce_i2c_hw: *mut dce_i2c_hw, ctx: *mut dc_context, engine_id: u32, regs: *const dce_i2c_registers, shifts: *const dce_i2c_shift, masks: *const dce_i2c_mask);
    pub fn dce100_i2c_hw_construct(dce_i2c_hw: *mut dce_i2c_hw, ctx: *mut dc_context, engine_id: u32, regs: *const dce_i2c_registers, shifts: *const dce_i2c_shift, masks: *const dce_i2c_mask);
    pub fn dce112_i2c_hw_construct(dce_i2c_hw: *mut dce_i2c_hw, ctx: *mut dc_context, engine_id: u32, regs: *const dce_i2c_registers, shifts: *const dce_i2c_shift, masks: *const dce_i2c_mask);
    pub fn dcn1_i2c_hw_construct(dce_i2c_hw: *mut dce_i2c_hw, ctx: *mut dc_context, engine_id: u32, regs: *const dce_i2c_registers, shifts: *const dce_i2c_shift, masks: *const dce_i2c_mask);
    pub fn dcn2_i2c_hw_construct(dce_i2c_hw: *mut dce_i2c_hw, ctx: *mut dc_context, engine_id: u32, regs: *const dce_i2c_registers, shifts: *const dce_i2c_shift, masks: *const dce_i2c_mask);
    pub fn dce_i2c_submit_command_hw(pool: *mut resource_pool, ddc: *mut ddc, cmd: *mut i2c_command, dce_i2c_hw: *mut dce_i2c_hw) -> bool;
    pub fn acquire_i2c_hw_engine(pool: *mut resource_pool, ddc: *mut ddc) -> *mut dce_i2c_hw;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
