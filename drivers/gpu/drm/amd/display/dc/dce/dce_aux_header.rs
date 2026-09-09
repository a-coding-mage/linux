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
 * THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
 * IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
 * FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL
 * THE COPYRIGHT HOLDER(S) OR AUTHOR(S) BE LIABLE FOR ANY CLAIM, DAMAGES OR
 * OTHER LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE,
 * ARISING FROM, OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR
 * OTHER DEALINGS IN THE SOFTWARE.
 *
 * Authors: AMD
 */

// Dependencies supplied by the surrounding translation unit:
// gpio_service_interface.h, inc/hw/aux_engine.h

pub enum aux_return_code_type {}

#[macro_export]
macro_rules! AUX_COMMON_REG_LIST0 { ($id:expr) => { SRI!(AUX_CONTROL, DP_AUX, $id), SRI!(AUX_ARB_CONTROL, DP_AUX, $id), SRI!(AUX_SW_DATA, DP_AUX, $id), SRI!(AUX_SW_CONTROL, DP_AUX, $id), SRI!(AUX_INTERRUPT_CONTROL, DP_AUX, $id), SRI!(AUX_DPHY_RX_CONTROL1, DP_AUX, $id), SRI!(AUX_SW_STATUS, DP_AUX, $id) }; }
#[macro_export]
macro_rules! AUX_COMMON_REG_LIST { ($id:expr) => { SRI!(AUX_CONTROL, DP_AUX, $id), SRI!(AUX_ARB_CONTROL, DP_AUX, $id), SRI!(AUX_SW_DATA, DP_AUX, $id), SRI!(AUX_SW_CONTROL, DP_AUX, $id), SRI!(AUX_INTERRUPT_CONTROL, DP_AUX, $id), SRI!(AUX_SW_STATUS, DP_AUX, $id), SR!(AUXN_IMPCAL), SR!(AUXP_IMPCAL) }; }

#[repr(C)]
pub struct dce110_aux_registers { pub AUX_CONTROL: u32, pub AUX_ARB_CONTROL: u32, pub AUX_SW_DATA: u32, pub AUX_SW_CONTROL: u32, pub AUX_INTERRUPT_CONTROL: u32, pub AUX_DPHY_RX_CONTROL1: u32, pub AUX_SW_STATUS: u32, pub AUXN_IMPCAL: u32, pub AUXP_IMPCAL: u32, pub AUX_RESET_MASK: u32 }

#[macro_export]
macro_rules! DCE_AUX_REG_FIELD_LIST { ($ty:ty) => { AUX_EN: $ty, AUX_RESET: $ty, AUX_RESET_DONE: $ty, AUX_REG_RW_CNTL_STATUS: $ty, AUX_SW_USE_AUX_REG_REQ: $ty, AUX_SW_DONE_USING_AUX_REG: $ty, AUX_SW_AUTOINCREMENT_DISABLE: $ty, AUX_SW_DATA_RW: $ty, AUX_SW_INDEX: $ty, AUX_SW_GO: $ty, AUX_SW_DATA: $ty, AUX_SW_REPLY_BYTE_COUNT: $ty, AUX_SW_DONE: $ty, AUX_SW_DONE_ACK: $ty, AUXN_IMPCAL_ENABLE: $ty, AUXP_IMPCAL_ENABLE: $ty, AUXN_IMPCAL_OVERRIDE_ENABLE: $ty, AUXP_IMPCAL_OVERRIDE_ENABLE: $ty, AUX_RX_TIMEOUT_LEN: $ty, AUX_RX_TIMEOUT_LEN_MUL: $ty, AUXN_CALOUT_ERROR_AK: $ty, AUXP_CALOUT_ERROR_AK: $ty, AUX_SW_START_DELAY: $ty, AUX_SW_WR_BYTES: $ty }; }

#[repr(C)]
pub struct dce110_aux_registers_mask { pub AUX_EN: u32, pub AUX_RESET: u32, pub AUX_RESET_DONE: u32, pub AUX_REG_RW_CNTL_STATUS: u32, pub AUX_SW_USE_AUX_REG_REQ: u32, pub AUX_SW_DONE_USING_AUX_REG: u32, pub AUX_SW_AUTOINCREMENT_DISABLE: u32, pub AUX_SW_DATA_RW: u32, pub AUX_SW_INDEX: u32, pub AUX_SW_GO: u32, pub AUX_SW_DATA: u32, pub AUX_SW_REPLY_BYTE_COUNT: u32, pub AUX_SW_DONE: u32, pub AUX_SW_DONE_ACK: u32, pub AUXN_IMPCAL_ENABLE: u32, pub AUXP_IMPCAL_ENABLE: u32, pub AUXN_IMPCAL_OVERRIDE_ENABLE: u32, pub AUXP_IMPCAL_OVERRIDE_ENABLE: u32, pub AUX_RX_TIMEOUT_LEN: u32, pub AUX_RX_TIMEOUT_LEN_MUL: u32, pub AUXN_CALOUT_ERROR_AK: u32, pub AUXP_CALOUT_ERROR_AK: u32, pub AUX_SW_START_DELAY: u32, pub AUX_SW_WR_BYTES: u32 }

#[repr(C)]
pub struct dce110_aux_registers_shift { pub AUX_EN: u8, pub AUX_RESET: u8, pub AUX_RESET_DONE: u8, pub AUX_REG_RW_CNTL_STATUS: u8, pub AUX_SW_USE_AUX_REG_REQ: u8, pub AUX_SW_DONE_USING_AUX_REG: u8, pub AUX_SW_AUTOINCREMENT_DISABLE: u8, pub AUX_SW_DATA_RW: u8, pub AUX_SW_INDEX: u8, pub AUX_SW_GO: u8, pub AUX_SW_DATA: u8, pub AUX_SW_REPLY_BYTE_COUNT: u8, pub AUX_SW_DONE: u8, pub AUX_SW_DONE_ACK: u8, pub AUXN_IMPCAL_ENABLE: u8, pub AUXP_IMPCAL_ENABLE: u8, pub AUXN_IMPCAL_OVERRIDE_ENABLE: u8, pub AUXP_IMPCAL_OVERRIDE_ENABLE: u8, pub AUX_RX_TIMEOUT_LEN: u8, pub AUX_RX_TIMEOUT_LEN_MUL: u8, pub AUXN_CALOUT_ERROR_AK: u8, pub AUXP_CALOUT_ERROR_AK: u8, pub AUX_SW_START_DELAY: u8, pub AUX_SW_WR_BYTES: u8 }

#[macro_export]
macro_rules! AUX_SF { ($reg:ident, $field:ident, $post_fix:ident) => { $field = $reg ## __ ## $field ## $post_fix }; }

// Register-mask field-list macros.  The concatenated C register-field names
// are retained as macro tokens for use by the generated register definitions.
#[macro_export]
macro_rules! DCE10_AUX_MASK_SH_LIST { ($mask_sh:expr) => { AUX_SF!(AUX_CONTROL, AUX_EN, $mask_sh); AUX_SF!(AUX_ARB_CONTROL, AUX_REG_RW_CNTL_STATUS, $mask_sh); AUX_SF!(AUX_ARB_CONTROL, AUX_SW_USE_AUX_REG_REQ, $mask_sh); AUX_SF!(AUX_ARB_CONTROL, AUX_SW_DONE_USING_AUX_REG, $mask_sh); AUX_SF!(AUX_SW_CONTROL, AUX_SW_START_DELAY, $mask_sh); AUX_SF!(AUX_SW_CONTROL, AUX_SW_WR_BYTES, $mask_sh); AUX_SF!(AUX_SW_CONTROL, AUX_SW_GO, $mask_sh); AUX_SF!(AUX_SW_DATA, AUX_SW_AUTOINCREMENT_DISABLE, $mask_sh); AUX_SF!(AUX_SW_DATA, AUX_SW_DATA_RW, $mask_sh); AUX_SF!(AUX_SW_DATA, AUX_SW_INDEX, $mask_sh); AUX_SF!(AUX_SW_DATA, AUX_SW_DATA, $mask_sh); AUX_SF!(AUX_SW_STATUS, AUX_SW_REPLY_BYTE_COUNT, $mask_sh); AUX_SF!(AUX_SW_STATUS, AUX_SW_DONE, $mask_sh); AUX_SF!(AUX_INTERRUPT_CONTROL, AUX_SW_DONE_ACK, $mask_sh); AUX_SF!(AUXN_IMPCAL, AUXN_CALOUT_ERROR_AK, $mask_sh); AUX_SF!(AUXP_IMPCAL, AUXP_CALOUT_ERROR_AK, $mask_sh); AUX_SF!(AUXN_IMPCAL, AUXN_IMPCAL_ENABLE, $mask_sh); AUX_SF!(AUXP_IMPCAL, AUXP_IMPCAL_ENABLE, $mask_sh); AUX_SF!(AUXP_IMPCAL, AUXP_IMPCAL_OVERRIDE_ENABLE, $mask_sh); AUX_SF!(AUXN_IMPCAL, AUXN_IMPCAL_OVERRIDE_ENABLE, $mask_sh) }; }
#[macro_export]
macro_rules! DCE_AUX_MASK_SH_LIST { ($mask_sh:expr) => { DCE10_AUX_MASK_SH_LIST!($mask_sh); AUX_SF!(AUX_CONTROL, AUX_RESET, $mask_sh); AUX_SF!(AUX_CONTROL, AUX_RESET_DONE, $mask_sh) }; }
#[macro_export]
macro_rules! DCE12_AUX_MASK_SH_LIST { ($mask_sh:expr) => { DCE_AUX_MASK_SH_LIST!($mask_sh); AUX_SF!(DP_AUX0_AUX_CONTROL, AUX_EN, $mask_sh); AUX_SF!(DP_AUX0_AUX_SW_DATA, AUX_SW_INDEX, $mask_sh); AUX_SF!(DP_AUX0_AUX_SW_DATA, AUX_SW_DATA, $mask_sh) }; }
#[macro_export]
macro_rules! DCN10_AUX_MASK_SH_LIST { ($mask_sh:expr) => { DCE12_AUX_MASK_SH_LIST!($mask_sh) }; }
#[macro_export]
macro_rules! DCN_AUX_MASK_SH_LIST { ($mask_sh:expr) => { DCN10_AUX_MASK_SH_LIST!($mask_sh); AUX_SF!(DP_AUX0_AUX_DPHY_RX_CONTROL1, AUX_RX_TIMEOUT_LEN, $mask_sh); AUX_SF!(DP_AUX0_AUX_DPHY_RX_CONTROL1, AUX_RX_TIMEOUT_LEN_MUL, $mask_sh) }; }

pub const AUX_TIMEOUT_PERIOD: u32 = 400;
pub const SW_AUX_TIMEOUT_PERIOD_MULTIPLIER: u32 = 6;

#[repr(C)]
pub struct dce_aux { pub inst: u32, pub ddc: *mut ddc, pub ctx: *mut dc_context, pub delay: u32, pub max_defer_write_retry: u32, pub acquire_reset: bool, pub funcs: *mut dce_aux_funcs }

#[repr(C)]
pub struct aux_engine_dce110 { pub base: dce_aux, pub regs: *const dce110_aux_registers, pub mask: *const dce110_aux_registers_mask, pub shift: *const dce110_aux_registers_shift, pub addr: aux_engine_dce110_addr, pub polling_timeout_period: u32 }
#[repr(C)]
pub struct aux_engine_dce110_addr { pub aux_control: u32, pub aux_arb_control: u32, pub aux_sw_data: u32, pub aux_sw_control: u32, pub aux_interrupt_control: u32, pub aux_dphy_rx_control1: u32, pub aux_dphy_rx_control0: u32, pub aux_sw_status: u32 }
#[repr(C)]
pub struct aux_engine_dce110_init_data { pub engine_id: u32, pub timeout_period: u32, pub ctx: *mut dc_context, pub regs: *const dce110_aux_registers }

extern "C" {
    pub fn dce110_aux_engine_construct(aux_engine110: *mut aux_engine_dce110, ctx: *mut dc_context, inst: u32, timeout_period: u32, regs: *const dce110_aux_registers, mask: *const dce110_aux_registers_mask, shift: *const dce110_aux_registers_shift, is_ext_aux_timeout_configurable: bool) -> *mut dce_aux;
    pub fn dce110_engine_destroy(engine: *mut *mut dce_aux);
    pub fn dce110_aux_engine_acquire(aux_engine: *mut dce_aux, ddc: *mut ddc) -> bool;
    pub fn dce_aux_transfer_raw(ddc: *mut ddc_service, cmd: *mut aux_payload, operation_result: *mut aux_return_code_type) -> i32;
    pub fn dce_aux_transfer_raw_with_ddc_pin(ddc: *mut ddc_service, cmd: *mut aux_payload, operation_result: *mut aux_return_code_type) -> i32;
    pub fn dce_aux_transfer_raw_without_ddc_pin(ddc: *mut ddc_service, cmd: *mut aux_payload, operation_result: *mut aux_return_code_type) -> i32;
    pub fn dce_aux_transfer_dmub_raw(ddc: *mut ddc_service, payload: *mut aux_payload, operation_result: *mut aux_return_code_type) -> i32;
    pub fn dce_aux_transfer_with_retries(ddc: *mut ddc_service, cmd: *mut aux_payload) -> bool;
}

#[repr(C)]
pub struct dce_aux_funcs { pub configure_timeout: Option<unsafe extern "C" fn(ddc: *mut ddc_service, timeout: u32) -> u32>, pub destroy: Option<unsafe extern "C" fn(ptr: *mut *mut aux_engine)> }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
