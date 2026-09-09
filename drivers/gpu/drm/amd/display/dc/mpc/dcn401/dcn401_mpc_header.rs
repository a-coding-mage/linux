/*
 * Copyright 2023-2026 Advanced Micro Devices, Inc.
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

// Dependencies supplied by dcn30/dcn32 MPC headers are intentionally external.

macro_rules! to_dcn401_mpc {
    ($mpc_base:expr) => { container_of!($mpc_base, dcn401_mpc, base) };
}

macro_rules! mpc_reg_variable_list_dcn4_01 {
    () => {
        MPCC_MCM_FIRST_GAMUT_REMAP_COEF_FORMAT: [u32; MAX_MPCC],
        MPCC_MCM_FIRST_GAMUT_REMAP_MODE: [u32; MAX_MPCC],
        MPC_MCM_FIRST_GAMUT_REMAP_C11_C12_A: [u32; MAX_MPCC],
        MPC_MCM_FIRST_GAMUT_REMAP_C13_C14_A: [u32; MAX_MPCC],
        MPC_MCM_FIRST_GAMUT_REMAP_C21_C22_A: [u32; MAX_MPCC],
        MPC_MCM_FIRST_GAMUT_REMAP_C23_C24_A: [u32; MAX_MPCC],
        MPC_MCM_FIRST_GAMUT_REMAP_C31_C32_A: [u32; MAX_MPCC],
        MPC_MCM_FIRST_GAMUT_REMAP_C33_C34_A: [u32; MAX_MPCC],
        MPC_MCM_FIRST_GAMUT_REMAP_C11_C12_B: [u32; MAX_MPCC],
        MPC_MCM_FIRST_GAMUT_REMAP_C13_C14_B: [u32; MAX_MPCC],
        MPC_MCM_FIRST_GAMUT_REMAP_C21_C22_B: [u32; MAX_MPCC],
        MPC_MCM_FIRST_GAMUT_REMAP_C23_C24_B: [u32; MAX_MPCC],
        MPC_MCM_FIRST_GAMUT_REMAP_C31_C32_B: [u32; MAX_MPCC],
        MPC_MCM_FIRST_GAMUT_REMAP_C33_C34_B: [u32; MAX_MPCC],
        MPCC_MCM_SECOND_GAMUT_REMAP_COEF_FORMAT: [u32; MAX_MPCC],
        MPCC_MCM_SECOND_GAMUT_REMAP_MODE: [u32; MAX_MPCC],
        MPC_MCM_SECOND_GAMUT_REMAP_C11_C12_A: [u32; MAX_MPCC],
        MPC_MCM_SECOND_GAMUT_REMAP_C13_C14_A: [u32; MAX_MPCC],
        MPC_MCM_SECOND_GAMUT_REMAP_C21_C22_A: [u32; MAX_MPCC],
        MPC_MCM_SECOND_GAMUT_REMAP_C23_C24_A: [u32; MAX_MPCC],
        MPC_MCM_SECOND_GAMUT_REMAP_C31_C32_A: [u32; MAX_MPCC],
        MPC_MCM_SECOND_GAMUT_REMAP_C33_C34_A: [u32; MAX_MPCC],
        MPC_MCM_SECOND_GAMUT_REMAP_C11_C12_B: [u32; MAX_MPCC],
        MPC_MCM_SECOND_GAMUT_REMAP_C13_C14_B: [u32; MAX_MPCC],
        MPC_MCM_SECOND_GAMUT_REMAP_C21_C22_B: [u32; MAX_MPCC],
        MPC_MCM_SECOND_GAMUT_REMAP_C23_C24_B: [u32; MAX_MPCC],
        MPC_MCM_SECOND_GAMUT_REMAP_C31_C32_B: [u32; MAX_MPCC],
        MPC_MCM_SECOND_GAMUT_REMAP_C33_C34_B: [u32; MAX_MPCC],
        MPCC_MCM_3DLUT_FAST_LOAD_SELECT: [u32; MAX_MPCC],
        MPCC_MCM_3DLUT_FAST_LOAD_STATUS: [u32; MAX_MPCC],
    };
}

#[repr(C)]
pub struct dcn401_mpc_shift {
    pub MPCC_MCM_FIRST_GAMUT_REMAP_COEF_FORMAT: u8,
    pub MPCC_MCM_FIRST_GAMUT_REMAP_MODE: u8,
    pub MPCC_MCM_FIRST_GAMUT_REMAP_MODE_CURRENT: u8,
    pub MPCC_MCM_FIRST_GAMUT_REMAP_C11_A: u8, pub MPCC_MCM_FIRST_GAMUT_REMAP_C12_A: u8,
    pub MPCC_MCM_FIRST_GAMUT_REMAP_C13_A: u8, pub MPCC_MCM_FIRST_GAMUT_REMAP_C14_A: u8,
    pub MPCC_MCM_FIRST_GAMUT_REMAP_C21_A: u8, pub MPCC_MCM_FIRST_GAMUT_REMAP_C22_A: u8,
    pub MPCC_MCM_FIRST_GAMUT_REMAP_C23_A: u8, pub MPCC_MCM_FIRST_GAMUT_REMAP_C24_A: u8,
    pub MPCC_MCM_FIRST_GAMUT_REMAP_C31_A: u8, pub MPCC_MCM_FIRST_GAMUT_REMAP_C32_A: u8,
    pub MPCC_MCM_FIRST_GAMUT_REMAP_C33_A: u8, pub MPCC_MCM_FIRST_GAMUT_REMAP_C34_A: u8,
    pub MPCC_MCM_SECOND_GAMUT_REMAP_COEF_FORMAT: u8,
    pub MPCC_MCM_SECOND_GAMUT_REMAP_MODE: u8,
    pub MPCC_MCM_SECOND_GAMUT_REMAP_MODE_CURRENT: u8,
    pub MPCC_MCM_SECOND_GAMUT_REMAP_C11_A: u8, pub MPCC_MCM_SECOND_GAMUT_REMAP_C12_A: u8,
    pub MPCC_MCM_SECOND_GAMUT_REMAP_C13_A: u8, pub MPCC_MCM_SECOND_GAMUT_REMAP_C14_A: u8,
    pub MPCC_MCM_SECOND_GAMUT_REMAP_C21_A: u8, pub MPCC_MCM_SECOND_GAMUT_REMAP_C22_A: u8,
    pub MPCC_MCM_SECOND_GAMUT_REMAP_C23_A: u8, pub MPCC_MCM_SECOND_GAMUT_REMAP_C24_A: u8,
    pub MPCC_MCM_SECOND_GAMUT_REMAP_C31_A: u8, pub MPCC_MCM_SECOND_GAMUT_REMAP_C32_A: u8,
    pub MPCC_MCM_SECOND_GAMUT_REMAP_C33_A: u8, pub MPCC_MCM_SECOND_GAMUT_REMAP_C34_A: u8,
    pub MPCC_MCM_3DLUT_FL_SEL: u8, pub MPCC_MCM_3DLUT_FL_DONE: u8,
    pub MPCC_MCM_3DLUT_FL_SOFT_UNDERFLOW: u8, pub MPCC_MCM_3DLUT_FL_HARD_UNDERFLOW: u8,
}

#[repr(C)]
pub struct dcn401_mpc_mask { pub fields: dcn401_mpc_shift }

#[repr(C)]
pub struct dcn401_mpc_registers {
    pub registers: mpc401_register_fields,
    pub MPCC_CONTROL2: [u32; MAX_MPCC],
}

#[repr(C)]
pub struct mpc401_register_fields { mpc_reg_variable_list_dcn4_01!(); }

#[repr(C)]
pub struct dcn401_mpc {
    pub base: mpc,
    pub mpcc_in_use_mask: i32,
    pub num_mpcc: i32,
    pub mpc_regs: *const dcn401_mpc_registers,
    pub mpc_shift: *const dcn401_mpc_shift,
    pub mpc_mask: *const dcn401_mpc_mask,
    pub num_rmu: i32,
}

extern "C" {
    pub fn dcn401_mpc_construct(mpc401: *mut dcn401_mpc, ctx: *mut dc_context,
        mpc_regs: *const dcn401_mpc_registers, mpc_shift: *const dcn401_mpc_shift,
        mpc_mask: *const dcn401_mpc_mask, num_mpcc: i32, num_rmu: i32);
    pub fn mpc401_set_movable_cm_location(mpc: *mut mpc, location: mpcc_movable_cm_location, mpcc_id: i32);
    pub fn mpc401_populate_lut(mpc: *mut mpc, id: MCM_LUT_ID, params: *const mcm_lut_params, lut_bank_a: bool, mpcc_id: i32);
    pub fn mpc401_program_lut_mode(mpc: *mut mpc, id: MCM_LUT_ID, enable: bool, lut_bank_a: bool, size: dc_cm_lut_size, mpcc_id: i32);
    pub fn mpc401_get_lut_mode(mpc: *mut mpc, id: MCM_LUT_ID, mpcc_id: i32, enable: *mut bool, lut_bank_a: *mut bool);
    pub fn mpc401_program_lut_read_write_control(mpc: *mut mpc, id: MCM_LUT_ID, lut_bank_a: bool, bit_depth: u32, mpcc_id: i32);
    pub fn mpc401_set_gamut_remap(mpc: *mut mpc, mpcc_id: i32, adjust: *const mpc_grph_gamut_adjustment);
}


// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
