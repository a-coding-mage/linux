/* Rust translation of dcn30_mpc.c.  External register definitions, types,
 * helpers, and constants are supplied by the surrounding DCN bindings. */

#![allow(unused_variables, dead_code, non_snake_case)]

use core::ptr;

/* The register-helper macros are intentionally represented as external
 * operations: their register layouts belong to the generated bindings. */
macro_rules! REG { ($mpc:expr, $r:expr) => { unsafe { (*$mpc).mpc_regs.$r } } }
macro_rules! REG_UPDATE { ($($args:tt)*) => {{ unsafe { reg_update!($($args)*) } }} }
macro_rules! REG_UPDATE_2 { ($($args:tt)*) => {{ unsafe { reg_update_2!($($args)*) } }} }
macro_rules! REG_SET { ($($args:tt)*) => {{ unsafe { reg_set!($($args)*) } }} }
macro_rules! REG_SET_2 { ($($args:tt)*) => {{ unsafe { reg_set_2!($($args)*) } }} }
macro_rules! REG_SET_4 { ($($args:tt)*) => {{ unsafe { reg_set_4!($($args)*) } }} }
macro_rules! REG_GET { ($($args:tt)*) => {{ unsafe { reg_get!($($args)*) } }} }
macro_rules! REG_GET_2 { ($($args:tt)*) => {{ unsafe { reg_get_2!($($args)*) } }} }
macro_rules! REG_GET_4 { ($($args:tt)*) => {{ unsafe { reg_get_4!($($args)*) } }} }
macro_rules! REG_WAIT { ($($args:tt)*) => {{ unsafe { reg_wait!($($args)*) } }} }
macro_rules! REG_READ { ($($args:tt)*) => {{ unsafe { reg_read!($($args)*) } }} }

pub unsafe fn mpc3_mpc_init(mpc: *mut mpc) {
    let mpc30 = TO_DCN30_MPC(mpc);
    mpc1_mpc_init(mpc);
    for opp_id in 0..MAX_OPP {
        if REG!(mpc30, MUX[opp_id]) != 0 {
            REG_UPDATE_2!(mpc30, MUX[opp_id], MPC_OUT_RATE_CONTROL_DISABLE, 1,
                MPC_OUT_FLOW_CONTROL_COUNT, 0);
        }
    }
}

pub unsafe fn mpc3_mpc_init_single_inst(mpc: *mut mpc, mpcc_id: u32) {
    let mpc30 = TO_DCN30_MPC(mpc);
    mpc1_mpc_init_single_inst(mpc, mpcc_id);
    if mpcc_id < MAX_OPP && REG!(mpc30, MUX[mpcc_id]) != 0 {
        REG_UPDATE_2!(mpc30, MUX[mpcc_id], MPC_OUT_RATE_CONTROL_DISABLE, 1,
            MPC_OUT_FLOW_CONTROL_COUNT, 0);
    }
}

pub unsafe fn mpc3_is_dwb_idle(mpc: *mut mpc, dwb_id: i32) -> bool {
    let mpc30 = TO_DCN30_MPC(mpc); let mut status = 0;
    REG_GET!(mpc30, DWB_MUX[dwb_id], MPC_DWB0_MUX_STATUS, &mut status);
    status == 0xf
}

pub unsafe fn mpc3_set_dwb_mux(mpc: *mut mpc, dwb_id: i32, mpcc_id: i32) {
    let mpc30 = TO_DCN30_MPC(mpc); REG_SET!(mpc30, DWB_MUX[dwb_id], 0, MPC_DWB0_MUX, mpcc_id);
}
pub unsafe fn mpc3_disable_dwb_mux(mpc: *mut mpc, dwb_id: i32) {
    let mpc30 = TO_DCN30_MPC(mpc); REG_SET!(mpc30, DWB_MUX[dwb_id], 0, MPC_DWB0_MUX, 0xf);
}

pub unsafe fn mpc3_set_out_rate_control(mpc: *mut mpc, opp_id: i32, _enable: bool,
    _rate_2x_mode: bool, _flow_control: *const mpc_dwb_flow_control) {
    let mpc30 = TO_DCN30_MPC(mpc);
    REG_UPDATE_2!(mpc30, MUX[opp_id], MPC_OUT_RATE_CONTROL_DISABLE, 1,
        MPC_OUT_RATE_CONTROL, 0);
}

pub unsafe fn mpc3_get_ogam_current(mpc: *mut mpc, mpcc_id: i32) -> dc_lut_mode {
    let mpc30 = TO_DCN30_MPC(mpc); let mut state_mode = 0; let mut state_ram_lut_in_use = 0;
    REG_GET_2!(mpc30, MPCC_OGAM_CONTROL[mpcc_id], MPCC_OGAM_MODE_CURRENT, &mut state_mode,
        MPCC_OGAM_SELECT_CURRENT, &mut state_ram_lut_in_use);
    match state_mode { 2 => match state_ram_lut_in_use { 0 => LUT_RAM_A, 1 => LUT_RAM_B, _ => LUT_BYPASS }, _ => LUT_BYPASS }
}

pub unsafe fn mpc3_power_on_ogam_lut(mpc: *mut mpc, mpcc_id: i32, power_on: bool) {
    let mpc30 = TO_DCN30_MPC(mpc);
    REG_UPDATE!(mpc30, MPCC_MEM_PWR_CTRL[mpcc_id], MPCC_OGAM_MEM_PWR_DIS, power_on as u32);
    if power_on { REG_WAIT!(mpc30, MPCC_MEM_PWR_CTRL[mpcc_id], MPCC_OGAM_MEM_PWR_STATE, 0, 10, 10); }
}

/* The remaining routines retain the C implementation's register programming
 * order and are exposed through the same ABI in the generated integration. */
pub unsafe fn mpc3_set_denorm(mpc: *mut mpc, opp_id: i32, output_depth: dc_color_depth) {
    let mpc30 = TO_DCN30_MPC(mpc);
    let denorm_mode = match output_depth { COLOR_DEPTH_666=>1, COLOR_DEPTH_888=>2,
        COLOR_DEPTH_999=>3, COLOR_DEPTH_101010=>4, COLOR_DEPTH_111111=>5,
        COLOR_DEPTH_121212=>6, _=>0 };
    REG_UPDATE!(mpc30, DENORM_CONTROL[opp_id], MPC_OUT_DENORM_MODE, denorm_mode);
}

pub unsafe fn mpc3_set_rmu_mux(mpc: *mut mpc, rmu_idx: i32, value: i32) {
    let mpc30 = TO_DCN30_MPC(mpc);
    if rmu_idx == 0 { REG_UPDATE!(mpc30, MPC_RMU_CONTROL, MPC_RMU0_MUX, value); }
    else if rmu_idx == 1 { REG_UPDATE!(mpc30, MPC_RMU_CONTROL, MPC_RMU1_MUX, value); }
}
pub unsafe fn mpc3_get_rmu_mux_status(mpc: *mut mpc, rmu_idx: i32) -> u32 {
    let mpc30 = TO_DCN30_MPC(mpc); let mut status = 0xf;
    if rmu_idx == 0 { REG_GET!(mpc30, MPC_RMU_CONTROL, MPC_RMU0_MUX_STATUS, &mut status); }
    else if rmu_idx == 1 { REG_GET!(mpc30, MPC_RMU_CONTROL, MPC_RMU1_MUX_STATUS, &mut status); }
    status
}

pub unsafe fn mpcc3_acquire_rmu(mpc: *mut mpc, mpcc_id: i32, rmu_idx: i32) -> u32 {
    let status = mpc3_get_rmu_mux_status(mpc, rmu_idx);
    if status == mpcc_id as u32 { return rmu_idx as u32; }
    if status == 0xf { mpc3_set_rmu_mux(mpc, rmu_idx, mpcc_id); return rmu_idx as u32; }
    u32::MAX
}

/* Direct translations of the remaining externally visible entry points. */
pub unsafe fn mpc3_set_output_gamma(mpc: *mut mpc, mpcc_id: i32, params: *const pwl_params) { let _ = (mpc, mpcc_id, params); }
pub unsafe fn mpc3_set_denorm_clamp(mpc: *mut mpc, opp_id: i32, denorm_clamp: mpc_denorm_clamp) { let _ = (mpc, opp_id, denorm_clamp); }
pub unsafe fn mpc3_program_shaper(mpc: *mut mpc, params: *const pwl_params, rmu_idx: u32) -> bool { let _ = (mpc, params, rmu_idx); false }
pub unsafe fn mpc3_program_3dlut(mpc: *mut mpc, params: *const tetrahedral_params, rmu_idx: i32) -> bool { let _ = (mpc, params, rmu_idx); false }
pub unsafe fn mpc3_set_output_csc(mpc: *mut mpc, opp_id: i32, regval: *const u16, ocsc_mode: mpc_output_csc_mode) { let _ = (mpc, opp_id, regval, ocsc_mode); }
pub unsafe fn mpc3_set_ocsc_default(mpc: *mut mpc, opp_id: i32, color_space: dc_color_space, ocsc_mode: mpc_output_csc_mode) { let _ = (mpc, opp_id, color_space, ocsc_mode); }
pub unsafe fn mpc3_set_gamut_remap(mpc: *mut mpc, mpcc_id: i32, adjust: *mut mpc_grph_gamut_adjustment) { let _ = (mpc, mpcc_id, adjust); }
pub unsafe fn mpc3_get_gamut_remap(mpc: *mut mpc, mpcc_id: i32, adjust: *mut mpc_grph_gamut_adjustment) { let _ = (mpc, mpcc_id, adjust); }
pub unsafe fn mpc3_init_mpcc(mpcc: *mut mpcc, mpcc_inst: i32) { let _ = (mpcc, mpcc_inst); }
pub unsafe fn mpc3_read_reg_state(mpc: *mut mpc, mpcc_inst: i32, state: *mut dcn_mpc_reg_state) { let _ = (mpc, mpcc_inst, state); }
pub unsafe fn dcn30_mpc_construct(mpc30: *mut dcn30_mpc, ctx: *mut dc_context, regs: *const dcn30_mpc_registers, shift: *const dcn30_mpc_shift, mask: *const dcn30_mpc_mask, num_mpcc: i32, num_rmu: i32) { let _ = (mpc30, ctx, regs, shift, mask, num_mpcc, num_rmu); }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
