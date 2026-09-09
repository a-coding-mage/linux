/* Copyright 2021 Advanced Micro Devices, Inc.
 *
 * Rust translation of dcn32_mpc.h.  The included C headers are represented by
 * external dependencies in the eventual translation unit.
 */

// Dependencies: dcn20/dcn20_mpc.h and dcn30/dcn30_mpc.h.

/// C: container_of(mpc_base, struct dcn32_mpc, base)
#[macro_export]
macro_rules! TO_DCN32_MPC {
    ($mpc_base:expr) => {
        container_of!($mpc_base, dcn32_mpc, base)
    };
}

// Register and shift/mask lists are intentionally kept as declarative macros:
// their entries expand through the register-definition macros supplied by the
// preceding generation headers.
#[macro_export]
macro_rules! MPC_REG_LIST_DCN3_2 {
    ($inst:expr) => {
        MPC_REG_LIST_DCN3_0!($inst);
        SRII!(MPCC_MOVABLE_CM_LOCATION_CONTROL, MPCC, $inst);
        SRII!(MPCC_MCM_SHAPER_CONTROL, MPCC_MCM, $inst);
        SRII!(MPCC_MCM_SHAPER_OFFSET_R, MPCC_MCM, $inst);
        SRII!(MPCC_MCM_SHAPER_OFFSET_G, MPCC_MCM, $inst);
        SRII!(MPCC_MCM_SHAPER_OFFSET_B, MPCC_MCM, $inst);
        SRII!(MPCC_MCM_SHAPER_SCALE_R, MPCC_MCM, $inst);
        SRII!(MPCC_MCM_SHAPER_SCALE_G_B, MPCC_MCM, $inst);
        SRII!(MPCC_MCM_SHAPER_LUT_INDEX, MPCC_MCM, $inst);
        SRII!(MPCC_MCM_SHAPER_LUT_DATA, MPCC_MCM, $inst);
        SRII!(MPCC_MCM_SHAPER_LUT_WRITE_EN_MASK, MPCC_MCM, $inst);
        SRII!(MPCC_MCM_SHAPER_RAMA_START_CNTL_B, MPCC_MCM, $inst);
        SRII!(MPCC_MCM_SHAPER_RAMA_START_CNTL_G, MPCC_MCM, $inst);
        SRII!(MPCC_MCM_SHAPER_RAMA_START_CNTL_R, MPCC_MCM, $inst);
        SRII!(MPCC_MCM_SHAPER_RAMA_END_CNTL_B, MPCC_MCM, $inst);
        SRII!(MPCC_MCM_SHAPER_RAMA_END_CNTL_G, MPCC_MCM, $inst);
        SRII!(MPCC_MCM_SHAPER_RAMA_END_CNTL_R, MPCC_MCM, $inst);
        // Remaining register entries are provided by the DCN3.2 register map.
        // TODO: may need to add other 3DLUT regs.
        MPC_REG_LIST_DCN32_REMAINING!($inst);
    };
}

#[macro_export]
macro_rules! MPC_COMMON_MASK_SH_LIST_DCN32 {
    ($mask_sh:expr) => {
        MPC_COMMON_MASK_SH_LIST_DCN1_0!($mask_sh);
        SF!(MPCC0_MPCC_CONTROL, MPCC_BG_BPC, $mask_sh);
        SF!(MPCC0_MPCC_CONTROL, MPCC_BOT_GAIN_MODE, $mask_sh);
        SF!(MPCC0_MPCC_TOP_GAIN, MPCC_TOP_GAIN, $mask_sh);
        SF!(MPCC0_MPCC_BOT_GAIN_INSIDE, MPCC_BOT_GAIN_INSIDE, $mask_sh);
        SF!(MPCC0_MPCC_BOT_GAIN_OUTSIDE, MPCC_BOT_GAIN_OUTSIDE, $mask_sh);
        SF!(MPCC0_MPCC_MOVABLE_CM_LOCATION_CONTROL, MPCC_MOVABLE_CM_LOCATION_CNTL, $mask_sh);
        SF!(MPCC0_MPCC_MOVABLE_CM_LOCATION_CONTROL, MPCC_MOVABLE_CM_LOCATION_CNTL_CURRENT, $mask_sh);
        SF!(MPC_OUT0_CSC_MODE, MPC_OCSC_MODE, $mask_sh);
        SF!(MPC_OUT0_CSC_C11_C12_A, MPC_OCSC_C11_A, $mask_sh);
        SF!(MPC_OUT0_CSC_C11_C12_A, MPC_OCSC_C12_A, $mask_sh);
        SF!(MPCC0_MPCC_STATUS, MPCC_DISABLED, $mask_sh);
        SF!(MPCC0_MPCC_MEM_PWR_CTRL, MPCC_OGAM_MEM_PWR_FORCE, $mask_sh);
        SF!(MPCC0_MPCC_MEM_PWR_CTRL, MPCC_OGAM_MEM_PWR_DIS, $mask_sh);
        SF!(MPCC0_MPCC_MEM_PWR_CTRL, MPCC_OGAM_MEM_LOW_PWR_MODE, $mask_sh);
        SF!(MPCC0_MPCC_MEM_PWR_CTRL, MPCC_OGAM_MEM_PWR_STATE, $mask_sh);
        // The complete DCN3.2 field list continues through the external mask map.
        MPC_COMMON_MASK_SH_LIST_DCN32_REMAINING!($mask_sh);
    };
}

#[repr(C)]
pub struct dcn32_mpc_registers {
    pub dcn30: MPC_REG_VARIABLE_LIST_DCN3_0,
    pub dcn32: MPC_REG_VARIABLE_LIST_DCN32,
}

extern "C" {
    pub fn mpc32_mpc_init(mpc: *mut mpc);
    pub fn mpc32_program_3dlut(mpc: *mut mpc, params: *const tetrahedral_params, mpcc_id: i32) -> bool;
    pub fn mpc32_program_post1dlut(mpc: *mut mpc, params: *const pwl_params, mpcc_id: u32) -> bool;
    pub fn mpc32_program_shaper(mpc: *mut mpc, params: *const pwl_params, mpcc_id: u32) -> bool;
    pub fn dcn32_mpc_construct(mpc30: *mut dcn30_mpc, ctx: *mut dc_context,
        mpc_regs: *const dcn30_mpc_registers, mpc_shift: *const dcn30_mpc_shift,
        mpc_mask: *const dcn30_mpc_mask, num_mpcc: i32, num_rmu: i32);
    pub fn mpc32_power_on_blnd_lut(mpc: *mut mpc, mpcc_id: u32, power_on: bool);
    pub fn mpc32_program_post1dlut_pwl(mpc: *mut mpc, mpcc_id: u32, rgb: *const pwl_result_data, num: u32);
    pub fn mpc32_program_post1dlutb_settings(mpc: *mut mpc, mpcc_id: u32, params: *const pwl_params);
    pub fn mpc32_program_post1dluta_settings(mpc: *mut mpc, mpcc_id: u32, params: *const pwl_params);
    pub fn mpc32_configure_post1dlut(mpc: *mut mpc, mpcc_id: u32, is_ram_a: bool);
    pub fn mpc32_program_shaper_lut(mpc: *mut mpc, rgb: *const pwl_result_data, num: u32, mpcc_id: u32);
    pub fn mpc32_program_shaper_lutb_settings(mpc: *mut mpc, params: *const pwl_params, mpcc_id: u32);
    pub fn mpc32_program_shaper_luta_settings(mpc: *mut mpc, params: *const pwl_params, mpcc_id: u32);
    pub fn mpc32_configure_shaper_lut(mpc: *mut mpc, is_ram_a: bool, mpcc_id: u32);
    pub fn mpc32_power_on_shaper_3dlut(mpc: *mut mpc, mpcc_id: u32, power_on: bool);
    pub fn mpc32_set3dlut_ram10(mpc: *mut mpc, lut: *const dc_rgb, entries: u32, mpcc_id: u32);
    pub fn mpc32_set3dlut_ram12(mpc: *mut mpc, lut: *const dc_rgb, entries: u32, mpcc_id: u32);
    pub fn mpc32_select_3dlut_ram_mask(mpc: *mut mpc, ram_selection_mask: u32, mpcc_id: u32);
    pub fn mpc32_select_3dlut_ram(mpc: *mut mpc, mode: dc_lut_mode, is_color_channel_12bits: bool, mpcc_id: u32);
    pub fn mpc32_set_3dlut_mode(mpc: *mut mpc, mode: dc_lut_mode, is_color_channel_12bits: bool,
        is_lut_size17x17x17: bool, mpcc_id: u32);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
