/* Copyright 2020 Advanced Micro Devices, Inc.
 *
 * Rust translation of dcn30_mpc.h.  Register-list macros retain their
 * preprocessor role and are intentionally left as token-producing macros.
 */

#![allow(non_camel_case_types, non_snake_case, dead_code)]

// Dependency supplied by the corresponding translated headers.
use core::ffi::c_void;

pub const MAX_RMU: usize = 3;

// C preprocessor register-list constructs.  Their arguments and expansion
// are supplied by the surrounding register-definition translation unit.
macro_rules! TO_DCN30_MPC { ($mpc_base:expr) => { container_of!($mpc_base, dcn30_mpc, base) }; }
macro_rules! MPC_REG_LIST_DCN3_0 { ($inst:tt) => { MPC_COMMON_REG_LIST_DCN1_0!($inst) }; }
macro_rules! MPC_OUT_MUX_REG_LIST_DCN3_0 { ($inst:tt) => { MPC_OUT_MUX_COMMON_REG_LIST_DCN1_0!($inst) }; }
macro_rules! MPC_RMU_GLOBAL_REG_LIST_DCN3AG { () => { }; }
macro_rules! MPC_RMU_REG_LIST_DCN3AG { ($inst:tt) => { }; }
macro_rules! MPC_DWB_MUX_REG_LIST_DCN3_0 { ($inst:tt) => { }; }
macro_rules! MPC_REG_VARIABLE_LIST_DCN3_0 { () => { }; }
macro_rules! MPC_REG_VARIABLE_LIST_DCN32 { () => { }; }
macro_rules! MPC_COMMON_MASK_SH_LIST_DCN3_0 { ($mask_sh:tt) => { MPC_COMMON_MASK_SH_LIST_DCN1_0!($mask_sh) }; }
macro_rules! MPC_COMMON_MASK_SH_LIST_DCN30 { ($mask_sh:tt) => { MPC_COMMON_MASK_SH_LIST_DCN1_0!($mask_sh) }; }
macro_rules! MPC_COMMON_MASK_SH_LIST_DCN303 { ($mask_sh:tt) => { MPC_COMMON_MASK_SH_LIST_DCN1_0!($mask_sh) }; }

// The following two macros are C type-list macros; each invocation expands to
// the fields supplied by the base-generation macro plus the DCN3 additions.
macro_rules! MPC_REG_FIELD_LIST_DCN3_0 { ($ty:ty) => { MPC_REG_FIELD_LIST_DCN2_0!($ty) }; }
macro_rules! MPC_REG_FIELD_LIST_DCN32 { ($ty:ty) => { }; }

#[repr(C)]
pub struct dcn30_mpc_registers {
    pub _base: [u32; 0],
}

#[repr(C)]
pub struct dcn30_mpc_shift {
    pub _base: [u8; 0],
}

#[repr(C)]
pub struct dcn30_mpc_mask {
    pub _base: [u32; 0],
}

#[repr(C)]
pub struct dcn30_mpc {
    pub base: mpc,
    pub mpcc_in_use_mask: core::ffi::c_int,
    pub num_mpcc: core::ffi::c_int,
    pub mpc_regs: *const dcn30_mpc_registers,
    pub mpc_shift: *const dcn30_mpc_shift,
    pub mpc_mask: *const dcn30_mpc_mask,
    pub num_rmu: core::ffi::c_int,
}

extern "C" {
    pub fn dcn30_mpc_construct(
        mpc30: *mut dcn30_mpc, ctx: *mut dc_context,
        mpc_regs: *const dcn30_mpc_registers,
        mpc_shift: *const dcn30_mpc_shift,
        mpc_mask: *const dcn30_mpc_mask,
        num_mpcc: core::ffi::c_int, num_rmu: core::ffi::c_int,
    );
    pub fn mpc3_mpc_init(mpc: *mut mpc);
    pub fn mpc3_mpc_init_single_inst(mpc: *mut mpc, mpcc_id: u32);
    pub fn mpc3_program_shaper(mpc: *mut mpc, params: *const pwl_params, rmu_idx: u32) -> bool;
    pub fn mpc3_program_3dlut(mpc: *mut mpc, params: *const tetrahedral_params, rmu_idx: core::ffi::c_int) -> bool;
    pub fn mpcc3_acquire_rmu(mpc: *mut mpc, mpcc_id: core::ffi::c_int, rmu_idx: core::ffi::c_int) -> u32;
    pub fn mpc3_set_denorm(mpc: *mut mpc, opp_id: core::ffi::c_int, output_depth: dc_color_depth);
    pub fn mpc3_set_denorm_clamp(mpc: *mut mpc, opp_id: core::ffi::c_int, denorm_clamp: mpc_denorm_clamp);
    pub fn mpc3_set_output_csc(mpc: *mut mpc, opp_id: core::ffi::c_int, regval: *const u16, ocsc_mode: mpc_output_csc_mode);
    pub fn mpc3_set_ocsc_default(mpc: *mut mpc, opp_id: core::ffi::c_int, color_space: dc_color_space, ocsc_mode: mpc_output_csc_mode);
    pub fn mpc3_set_output_gamma(mpc: *mut mpc, mpcc_id: core::ffi::c_int, params: *const pwl_params);
    pub fn mpc3_get_rmu_mux_status(mpc: *mut mpc, rmu_idx: core::ffi::c_int) -> u32;
    pub fn mpc3_set_gamut_remap(mpc: *mut mpc, mpcc_id: core::ffi::c_int, adjust: *const mpc_grph_gamut_adjustment);
    pub fn mpc3_get_gamut_remap(mpc: *mut mpc, mpcc_id: core::ffi::c_int, adjust: *mut mpc_grph_gamut_adjustment);
    pub fn mpc3_set_rmu_mux(mpc: *mut mpc, rmu_idx: core::ffi::c_int, value: core::ffi::c_int);
    pub fn mpc3_set_dwb_mux(mpc: *mut mpc, dwb_id: core::ffi::c_int, mpcc_id: core::ffi::c_int);
    pub fn mpc3_disable_dwb_mux(mpc: *mut mpc, dwb_id: core::ffi::c_int);
    pub fn mpc3_is_dwb_idle(mpc: *mut mpc, dwb_id: core::ffi::c_int) -> bool;
    pub fn mpc3_set_out_rate_control(mpc: *mut mpc, opp_id: core::ffi::c_int, enable: bool, rate_2x_mode: bool, flow_control: *mut mpc_dwb_flow_control);
    pub fn mpc3_power_on_ogam_lut(mpc: *mut mpc, mpcc_id: core::ffi::c_int, power_on: bool);
    pub fn mpc3_read_reg_state(mpc: *mut mpc, mpcc_inst: core::ffi::c_int, mpc_reg_state: *mut dcn_mpc_reg_state);
    pub fn mpc3_init_mpcc(mpcc: *mut mpcc, mpcc_inst: core::ffi::c_int);
    pub fn mpc3_get_ogam_current(mpc: *mut mpc, mpcc_id: core::ffi::c_int) -> dc_lut_mode;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
