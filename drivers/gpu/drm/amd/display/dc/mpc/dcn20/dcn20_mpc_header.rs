/* Copyright 2012-15 Advanced Micro Devices, Inc.
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
 */

// Dependency: dcn10/dcn10_mpc.h supplies the common register lists, types,
// constants, and field declarations referenced below.

#[macro_export]
macro_rules! TO_DCN20_MPC {
    ($mpc_base:expr) => {{
        // C equivalent: container_of($mpc_base, struct dcn20_mpc, base)
        unsafe { &mut *((($mpc_base as *mut _ as *mut u8).sub(0)) as *mut dcn20_mpc) }
    }};
}

// Register-list macros retain the source register metadata and depend on the
// corresponding externally supplied Rust register-list macros.
#[macro_export]
macro_rules! MPC_REG_LIST_DCN2_0 {
    ($inst:expr) => { MPC_COMMON_REG_LIST_DCN1_0!($inst) };
}
#[macro_export]
macro_rules! MPC_OUT_MUX_REG_LIST_DCN2_0 {
    ($inst:expr) => { MPC_OUT_MUX_COMMON_REG_LIST_DCN1_0!($inst) };
}
#[macro_export]
macro_rules! MPC_DBG_REG_LIST_DCN2_0 {
    () => { (MPC_OCSC_TEST_DEBUG_DATA, MPC_OCSC_TEST_DEBUG_INDEX) };
}

pub const MPC_OCSC_TEST_DEBUG_DATA_STATUS_IDX: u32 = 1;
pub const MPC_DEBUG_REG_LIST_SH_DCN20_MPC_OCSC_TEST_DEBUG_DATA_OCSC_MODE: u32 = 0;
pub const MPC_DEBUG_REG_LIST_MASK_DCN20_MPC_OCSC_TEST_DEBUG_DATA_OCSC_MODE: u32 = 0x3;

#[repr(C)]
pub struct dcn20_mpc_registers {
    // MPC_COMMON_REG_VARIABLE_LIST
    pub MPCC_TOP_GAIN: [u32; MAX_MPCC],
    pub MPCC_BOT_GAIN_INSIDE: [u32; MAX_MPCC],
    pub MPCC_BOT_GAIN_OUTSIDE: [u32; MAX_MPCC],
    pub MPCC_OGAM_RAMA_START_CNTL_B: [u32; MAX_MPCC],
    pub MPCC_OGAM_RAMA_START_CNTL_G: [u32; MAX_MPCC],
    pub MPCC_OGAM_RAMA_START_CNTL_R: [u32; MAX_MPCC],
    pub MPCC_OGAM_RAMA_SLOPE_CNTL_B: [u32; MAX_MPCC],
    pub MPCC_OGAM_RAMA_SLOPE_CNTL_G: [u32; MAX_MPCC],
    pub MPCC_OGAM_RAMA_SLOPE_CNTL_R: [u32; MAX_MPCC],
    pub MPCC_OGAM_RAMA_END_CNTL1_B: [u32; MAX_MPCC],
    pub MPCC_OGAM_RAMA_END_CNTL2_B: [u32; MAX_MPCC],
    pub MPCC_OGAM_RAMA_END_CNTL1_G: [u32; MAX_MPCC],
    pub MPCC_OGAM_RAMA_END_CNTL2_G: [u32; MAX_MPCC],
    pub MPCC_OGAM_RAMA_END_CNTL1_R: [u32; MAX_MPCC],
    pub MPCC_OGAM_RAMA_END_CNTL2_R: [u32; MAX_MPCC],
    pub MPCC_OGAM_RAMA_REGION_0_1: [u32; MAX_MPCC],
    pub MPCC_OGAM_RAMA_REGION_32_33: [u32; MAX_MPCC],
    pub MPCC_OGAM_RAMB_START_CNTL_B: [u32; MAX_MPCC],
    pub MPCC_OGAM_RAMB_START_CNTL_G: [u32; MAX_MPCC],
    pub MPCC_OGAM_RAMB_START_CNTL_R: [u32; MAX_MPCC],
    pub MPCC_OGAM_RAMB_SLOPE_CNTL_B: [u32; MAX_MPCC],
    pub MPCC_OGAM_RAMB_SLOPE_CNTL_G: [u32; MAX_MPCC],
    pub MPCC_OGAM_RAMB_SLOPE_CNTL_R: [u32; MAX_MPCC],
    pub MPCC_OGAM_RAMB_END_CNTL1_B: [u32; MAX_MPCC],
    pub MPCC_OGAM_RAMB_END_CNTL2_B: [u32; MAX_MPCC],
    pub MPCC_OGAM_RAMB_END_CNTL1_G: [u32; MAX_MPCC],
    pub MPCC_OGAM_RAMB_END_CNTL2_G: [u32; MAX_MPCC],
    pub MPCC_OGAM_RAMB_END_CNTL1_R: [u32; MAX_MPCC],
    pub MPCC_OGAM_RAMB_END_CNTL2_R: [u32; MAX_MPCC],
    pub MPCC_OGAM_RAMB_REGION_0_1: [u32; MAX_MPCC],
    pub MPCC_OGAM_RAMB_REGION_32_33: [u32; MAX_MPCC],
    pub MPCC_MEM_PWR_CTRL: [u32; MAX_MPCC],
    pub MPCC_OGAM_LUT_INDEX: [u32; MAX_MPCC],
    pub MPCC_OGAM_LUT_RAM_CONTROL: [u32; MAX_MPCC],
    pub MPCC_OGAM_LUT_DATA: [u32; MAX_MPCC],
    pub MPCC_OGAM_MODE: [u32; MAX_MPCC],
    pub MPC_OCSC_TEST_DEBUG_DATA: u32,
    pub MPC_OCSC_TEST_DEBUG_INDEX: u32,
    pub CSC_MODE: [u32; MAX_OPP],
    pub CSC_C11_C12_A: [u32; MAX_OPP],
    pub CSC_C33_C34_A: [u32; MAX_OPP],
    pub CSC_C11_C12_B: [u32; MAX_OPP],
    pub CSC_C33_C34_B: [u32; MAX_OPP],
    pub DENORM_CONTROL: [u32; MAX_OPP],
    pub DENORM_CLAMP_G_Y: [u32; MAX_OPP],
    pub DENORM_CLAMP_B_CB: [u32; MAX_OPP],
}

// MPC_REG_FIELD_LIST_DCN2_0(type), expanded for the two concrete C types.
#[repr(C)]
pub struct dcn20_mpc_shift { pub fields: [u8; 45] }
#[repr(C)]
pub struct dcn20_mpc_mask { pub fields: [u32; 45] }

#[repr(C)]
pub struct dcn20_mpc {
    pub base: mpc,
    pub mpcc_in_use_mask: i32,
    pub num_mpcc: i32,
    pub mpc_regs: *const dcn20_mpc_registers,
    pub mpc_shift: *const dcn20_mpc_shift,
    pub mpc_mask: *const dcn20_mpc_mask,
}

extern "C" {
    pub fn dcn20_mpc_construct(mpcc20: *mut dcn20_mpc, ctx: *mut dc_context,
        mpc_regs: *const dcn20_mpc_registers, mpc_shift: *const dcn20_mpc_shift,
        mpc_mask: *const dcn20_mpc_mask, num_mpcc: i32);
    pub fn mpc2_update_blending(mpc: *mut mpc, blnd_cfg: *mut mpcc_blnd_cfg, mpcc_id: i32);
    pub fn mpc2_set_denorm(mpc: *mut mpc, opp_id: i32, output_depth: dc_color_depth);
    pub fn mpc2_set_denorm_clamp(mpc: *mut mpc, opp_id: i32, denorm_clamp: mpc_denorm_clamp);
    pub fn mpc2_set_output_csc(mpc: *mut mpc, opp_id: i32, regval: *const u16, ocsc_mode: mpc_output_csc_mode);
    pub fn mpc2_set_ocsc_default(mpc: *mut mpc, opp_id: i32, color_space: dc_color_space, ocsc_mode: mpc_output_csc_mode);
    pub fn mpc2_set_output_gamma(mpc: *mut mpc, mpcc_id: i32, params: *const pwl_params);
    pub fn mpc2_assert_idle_mpcc(mpc: *mut mpc, id: i32);
    pub fn mpc2_assert_mpcc_idle_before_connect(mpc: *mut mpc, mpcc_id: i32);
    pub fn mpc20_power_on_ogam_lut(mpc: *mut mpc, mpcc_id: i32, power_on: bool);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
