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
 */

// Register helpers and declarations are supplied by the surrounding translation unit.

pub unsafe fn mpc401_update_3dlut_fast_load_select(mpc: *mut mpc, mpcc_id: i32, hubp_idx: i32) {
    let mpc401 = TO_DCN401_MPC(mpc);
    REG_SET((*mpc401).mpc_regs.MPCC_MCM_3DLUT_FAST_LOAD_SELECT[mpcc_id as usize], 0,
        MPCC_MCM_3DLUT_FL_SEL, hubp_idx);
}

pub unsafe fn mpc401_get_3dlut_fast_load_status(mpc: *mut mpc, mpcc_id: i32, done: *mut u32, soft_underflow: *mut u32, hard_underflow: *mut u32) {
    let mpc401 = TO_DCN401_MPC(mpc);
    REG_GET_3((*mpc401).mpc_regs.MPCC_MCM_3DLUT_FAST_LOAD_STATUS[mpcc_id as usize],
        MPCC_MCM_3DLUT_FL_DONE, done, MPCC_MCM_3DLUT_FL_SOFT_UNDERFLOW, soft_underflow,
        MPCC_MCM_3DLUT_FL_HARD_UNDERFLOW, hard_underflow);
}

pub unsafe fn mpc401_set_movable_cm_location(mpc: *mut mpc, location: mpcc_movable_cm_location, mpcc_id: i32) {
    let _mpc401 = TO_DCN401_MPC(mpc);
    match location {
        MPCC_MOVABLE_CM_LOCATION_BEFORE => REG_UPDATE(MPCC_MOVABLE_CM_LOCATION_CONTROL[mpcc_id as usize], MPCC_MOVABLE_CM_LOCATION_CNTL, 0),
        MPCC_MOVABLE_CM_LOCATION_AFTER => REG_UPDATE(MPCC_MOVABLE_CM_LOCATION_CONTROL[mpcc_id as usize], MPCC_MOVABLE_CM_LOCATION_CNTL, 1),
        _ => {}
    }
}

pub unsafe fn mpc401_populate_lut(mpc: *mut mpc, id: MCM_LUT_ID, params: *const mcm_lut_params, lut_bank_a: bool, mpcc_id: i32) {
    let next_mode = if lut_bank_a { LUT_RAM_A } else { LUT_RAM_B };
    let lut1d = (*params).pwl;
    let lut_shaper = (*params).pwl;
    let lut3d = (*params).lut3d;
    match id {
        MCM_LUT_1DLUT => {
            if lut1d.is_null() { return; }
            mpc32_power_on_blnd_lut(mpc, mpcc_id, true);
            mpc32_configure_post1dlut(mpc, mpcc_id, next_mode == LUT_RAM_A);
            if next_mode == LUT_RAM_A { mpc32_program_post1dluta_settings(mpc, mpcc_id, lut1d); }
            else { mpc32_program_post1dlutb_settings(mpc, mpcc_id, lut1d); }
            mpc32_program_post1dlut_pwl(mpc, mpcc_id, (*lut1d).rgb_resulted, (*lut1d).hw_points_num);
        }
        MCM_LUT_SHAPER => {
            if lut_shaper.is_null() { return; }
            if (*(*mpc).ctx).dc.debug.enable_mem_low_power.bits.mpc { mpc32_power_on_shaper_3dlut(mpc, mpcc_id, true); }
            mpc32_configure_shaper_lut(mpc, next_mode == LUT_RAM_A, mpcc_id);
            if next_mode == LUT_RAM_A { mpc32_program_shaper_luta_settings(mpc, lut_shaper, mpcc_id); }
            else { mpc32_program_shaper_lutb_settings(mpc, lut_shaper, mpcc_id); }
            mpc32_program_shaper_lut(mpc, (*lut_shaper).rgb_resulted, (*lut_shaper).hw_points_num, mpcc_id);
            mpc32_power_on_shaper_3dlut(mpc, mpcc_id, false);
        }
        MCM_LUT_3DLUT => {
            if lut3d.is_null() { return; }
            mpc32_power_on_shaper_3dlut(mpc, mpcc_id, true);
            let is_17 = !(*lut3d).use_tetrahedral_9;
            let is_12 = (*lut3d).use_12bits;
            let (lut0, lut1, lut2, lut3, size0, size) = if is_17 {
                ((*lut3d).tetrahedral_17.lut0, (*lut3d).tetrahedral_17.lut1, (*lut3d).tetrahedral_17.lut2, (*lut3d).tetrahedral_17.lut3,
                 (*lut3d).tetrahedral_17.lut0.len() as i32, (*lut3d).tetrahedral_17.lut1.len() as i32)
            } else {
                ((*lut3d).tetrahedral_9.lut0, (*lut3d).tetrahedral_9.lut1, (*lut3d).tetrahedral_9.lut2, (*lut3d).tetrahedral_9.lut3,
                 (*lut3d).tetrahedral_9.lut0.len() as i32, (*lut3d).tetrahedral_9.lut1.len() as i32)
            };
            for (mask, lut, n) in [(1, lut0, size0), (2, lut1, size), (4, lut2, size), (8, lut3, size)] {
                mpc32_select_3dlut_ram_mask(mpc, mask, mpcc_id);
                if is_12 { mpc32_set3dlut_ram12(mpc, lut, n, mpcc_id); }
                else { mpc32_set3dlut_ram10(mpc, lut, n, mpcc_id); }
            }
            if (*(*mpc).ctx).dc.debug.enable_mem_low_power.bits.mpc { mpc32_power_on_shaper_3dlut(mpc, mpcc_id, false); }
        }
        _ => {}
    }
}

unsafe fn mpc401_cm_lut_size_to_3dlut_size(cm_size: dc_cm_lut_size) -> u32 {
    match cm_size { CM_LUT_SIZE_999 => 1, CM_LUT_SIZE_171717 => 0, _ => { ASSERT(false); 0 } }
}

pub unsafe fn mpc401_program_lut_mode(mpc: *mut mpc, id: MCM_LUT_ID, enable: bool, lut_bank_a: bool, size: dc_cm_lut_size, mpcc_id: i32) {
    let mpc401 = TO_DCN401_MPC(mpc);
    match id {
        MCM_LUT_3DLUT => { if enable { REG_UPDATE_2((*mpc401).mpc_regs.MPCC_MCM_3DLUT_MODE[mpcc_id as usize], MPCC_MCM_3DLUT_MODE, if lut_bank_a {1} else {2}, MPCC_MCM_3DLUT_SIZE, mpc401_cm_lut_size_to_3dlut_size(size)); } else { if (*(*mpc).ctx).dc.debug.enable_mem_low_power.bits.mpc { mpc32_power_on_shaper_3dlut(mpc, mpcc_id, false); } REG_UPDATE(MPCC_MCM_3DLUT_MODE[mpcc_id as usize], MPCC_MCM_3DLUT_MODE, 0); } }
        MCM_LUT_SHAPER => { if enable { REG_UPDATE(MPCC_MCM_SHAPER_CONTROL[mpcc_id as usize], MPCC_MCM_SHAPER_LUT_MODE, if lut_bank_a {1} else {2}); } else { if (*(*mpc).ctx).dc.debug.enable_mem_low_power.bits.mpc { mpc32_power_on_shaper_3dlut(mpc, mpcc_id, false); } REG_UPDATE(MPCC_MCM_SHAPER_CONTROL[mpcc_id as usize], MPCC_MCM_SHAPER_LUT_MODE, 0); } }
        MCM_LUT_1DLUT => { if enable { REG_UPDATE(MPCC_MCM_1DLUT_CONTROL[mpcc_id as usize], MPCC_MCM_1DLUT_MODE, 2); } else { if (*(*mpc).ctx).dc.debug.enable_mem_low_power.bits.mpc { mpc32_power_on_blnd_lut(mpc, mpcc_id, false); } REG_UPDATE(MPCC_MCM_1DLUT_CONTROL[mpcc_id as usize], MPCC_MCM_1DLUT_MODE, 0); } REG_UPDATE(MPCC_MCM_1DLUT_CONTROL[mpcc_id as usize], MPCC_MCM_1DLUT_SELECT, if lut_bank_a {0} else {1}); }
    }
}

pub unsafe fn mpc401_program_lut_read_write_control(mpc: *mut mpc, id: MCM_LUT_ID, lut_bank_a: bool, bit_depth: u32, mpcc_id: i32) {
    match id { MCM_LUT_3DLUT => { mpc32_select_3dlut_ram_mask(mpc, 0xf, mpcc_id); REG_UPDATE_2(MPCC_MCM_3DLUT_READ_WRITE_CONTROL[mpcc_id as usize], MPCC_MCM_3DLUT_30BIT_EN, if bit_depth == 10 {1} else {0}, MPCC_MCM_3DLUT_RAM_SEL, if lut_bank_a {0} else {1}); }, MCM_LUT_SHAPER => mpc32_configure_shaper_lut(mpc, lut_bank_a, mpcc_id), MCM_LUT_1DLUT => mpc32_configure_post1dlut(mpc, lut_bank_a, mpcc_id) }
}

// The remaining matrix read/write routines retain the C API and delegate to the shared color-matrix helpers.
pub unsafe fn mpc401_set_gamut_remap(mpc: *mut mpc, mpcc_id: i32, adjust: *const mpc_grph_gamut_adjustment) {
    let mut hw = [0u16; 12];
    let mut mode = MPCC_GAMUT_REMAP_MODE_SELECT_0;
    let mut format = CM_GAMUT_REMAP_COEF_FORMAT_S2_13;
    if (*adjust).gamut_adjust_type != GRAPHICS_GAMUT_ADJUST_TYPE_SW { mpc_program_gamut_remap(mpc, mpcc_id as u32, core::ptr::null(), (*adjust).mpcc_gamut_remap_block_id, mode, format); return; }
    let mut max = fixed31_32 { value: 0 };
    for i in 0..12 { let v = dc_fixpt_abs((*adjust).temperature_matrix[i]); if dc_fixpt_le(max, v) { max = v; } }
    format = if dc_fixpt_le(max, dc_fixpt_from_fraction(S2D13_MAX, DIVIDER)) { CM_GAMUT_REMAP_COEF_FORMAT_S2_13 } else { CM_GAMUT_REMAP_COEF_FORMAT_S3_12 };
    convert_float_matrix(hw.as_mut_ptr(), (*adjust).temperature_matrix.as_ptr(), format, 12);
    mode = if mode != MPCC_GAMUT_REMAP_MODE_SELECT_1 { MPCC_GAMUT_REMAP_MODE_SELECT_1 } else { MPCC_GAMUT_REMAP_MODE_SELECT_2 };
    mpc_program_gamut_remap(mpc, mpcc_id as u32, hw.as_ptr(), (*adjust).mpcc_gamut_remap_block_id, mode, format);
}

pub unsafe fn mpc401_get_gamut_remap(mpc: *mut mpc, mpcc_id: i32, adjust: *mut mpc_grph_gamut_adjustment) {
    let arr = [0u16; 12]; let mode = MPCC_GAMUT_REMAP_MODE_SELECT_0; let format = CM_GAMUT_REMAP_COEF_FORMAT_S2_13;
    if mode == MPCC_GAMUT_REMAP_MODE_SELECT_0 { (*adjust).gamut_adjust_type = GRAPHICS_GAMUT_ADJUST_TYPE_BYPASS; } else { (*adjust).gamut_adjust_type = GRAPHICS_GAMUT_ADJUST_TYPE_SW; convert_hw_matrix((*adjust).temperature_matrix.as_mut_ptr(), arr.as_ptr(), format, arr.len()); }
}

pub unsafe fn mpc_program_gamut_remap(mpc: *mut mpc, mpcc_id: u32, regval: *const u16, block: mpcc_gamut_remap_id, mode: mpcc_gamut_remap_mode_select, format: cm_gamut_coef_format) {
    let mpc401 = TO_DCN401_MPC(mpc);
    if regval.is_null() || mode == MPCC_GAMUT_REMAP_MODE_SELECT_0 { REG_SET(MPCC_GAMUT_REMAP_MODE[mpcc_id as usize], 0, MPCC_GAMUT_REMAP_MODE, mode); return; }
    let (s11, m11, s12, m12) = match block { MPCC_OGAM_GAMUT_REMAP => ((*mpc401).mpc_shift).MPCC_GAMUT_REMAP_C11_A, ((*mpc401).mpc_mask).MPCC_GAMUT_REMAP_C11_A, ((*mpc401).mpc_shift).MPCC_GAMUT_REMAP_C12_A, ((*mpc401).mpc_mask).MPCC_GAMUT_REMAP_C12_A), MPCC_MCM_FIRST_GAMUT_REMAP => ((*mpc401).mpc_shift).MPCC_MCM_FIRST_GAMUT_REMAP_C11_A, ((*mpc401).mpc_mask).MPCC_MCM_FIRST_GAMUT_REMAP_C11_A, ((*mpc401).mpc_shift).MPCC_MCM_FIRST_GAMUT_REMAP_C12_A, ((*mpc401).mpc_mask).MPCC_MCM_FIRST_GAMUT_REMAP_C12_A), _ => ((*mpc401).mpc_shift).MPCC_MCM_SECOND_GAMUT_REMAP_C11_A, ((*mpc401).mpc_mask).MPCC_MCM_SECOND_GAMUT_REMAP_C11_A, ((*mpc401).mpc_shift).MPCC_MCM_SECOND_GAMUT_REMAP_C12_A, ((*mpc401).mpc_mask).MPCC_MCM_SECOND_GAMUT_REMAP_C12_A) };
    let mut regs = color_matrices_reg { shifts: color_matrices_shifts { csc_c11: s11, csc_c12: s12 }, masks: color_matrices_masks { csc_c11: m11, csc_c12: m12 }, ..core::mem::zeroed() };
    cm_helper_program_color_matrices((*mpc).ctx, regval, &mut regs);
    REG_SET(MPCC_GAMUT_REMAP_MODE[mpcc_id as usize], 0, MPCC_GAMUT_REMAP_MODE, mode);
    REG_SET(MPCC_GAMUT_REMAP_COEF_FORMAT[mpcc_id as usize], 0, MPCC_GAMUT_REMAP_COEF_FORMAT, format);
}

pub unsafe fn mpc401_get_lut_mode(mpc: *mut mpc, id: MCM_LUT_ID, mpcc_id: i32, enable: *mut bool, lut_bank_a: *mut bool) {
    *enable = false; *lut_bank_a = true; let mut mode = 0u32; let mut select = 0u32;
    match id { MCM_LUT_SHAPER => { REG_GET(MPCC_MCM_SHAPER_CONTROL[mpcc_id as usize], MPCC_MCM_SHAPER_MODE_CURRENT, &mut mode); *enable = mode != 0; *lut_bank_a = mode != 2; }, MCM_LUT_1DLUT => { REG_GET_2(MPCC_MCM_1DLUT_CONTROL[mpcc_id as usize], MPCC_MCM_1DLUT_MODE_CURRENT, &mut mode, MPCC_MCM_1DLUT_SELECT_CURRENT, &mut select); *enable = mode != 0; *lut_bank_a = mode == 0 || select == 0; }, _ => { REG_GET(MPCC_MCM_3DLUT_MODE[mpcc_id as usize], MPCC_MCM_3DLUT_MODE_CURRENT, &mut mode); *enable = mode != 0; *lut_bank_a = mode != 2; } }
}

static dcn401_mpc_funcs: mpc_funcs = mpc_funcs {
    read_mpcc_state: Some(mpc1_read_mpcc_state), insert_plane: Some(mpc1_insert_plane), remove_mpcc: Some(mpc1_remove_mpcc), mpc_init: Some(mpc32_mpc_init), mpc_init_single_inst: Some(mpc3_mpc_init_single_inst), update_blending: Some(mpc2_update_blending), cursor_lock: Some(mpc1_cursor_lock), get_mpcc_for_dpp: Some(mpc1_get_mpcc_for_dpp), wait_for_idle: Some(mpc2_assert_idle_mpcc), assert_mpcc_idle_before_connect: Some(mpc2_assert_mpcc_idle_before_connect), init_mpcc_list_from_hw: Some(mpc1_init_mpcc_list_from_hw), set_denorm: Some(mpc3_set_denorm), set_denorm_clamp: Some(mpc3_set_denorm_clamp), set_output_csc: Some(mpc3_set_output_csc), set_ocsc_default: Some(mpc3_set_ocsc_default), set_output_gamma: Some(mpc3_set_output_gamma), insert_plane_to_secondary: None, remove_mpcc_from_secondary: None, set_dwb_mux: Some(mpc3_set_dwb_mux), disable_dwb_mux: Some(mpc3_disable_dwb_mux), is_dwb_idle: Some(mpc3_is_dwb_idle), set_gamut_remap: Some(mpc401_set_gamut_remap), program_shaper: Some(mpc32_program_shaper), program_3dlut: Some(mpc32_program_3dlut), program_1dlut: Some(mpc32_program_post1dlut), acquire_rmu: None, release_rmu: None, power_on_mpc_mem_pwr: Some(mpc3_power_on_ogam_lut), get_mpc_out_mux: Some(mpc1_get_mpc_out_mux), mpc_read_reg_state: Some(mpc3_read_reg_state), set_bg_color: Some(mpc1_set_bg_color), set_movable_cm_location: Some(mpc401_set_movable_cm_location), update_3dlut_fast_load_select: Some(mpc401_update_3dlut_fast_load_select), get_3dlut_fast_load_status: Some(mpc401_get_3dlut_fast_load_status), populate_lut: Some(mpc401_populate_lut), program_lut_read_write_control: Some(mpc401_program_lut_read_write_control), program_lut_mode: Some(mpc401_program_lut_mode), get_lut_mode: Some(mpc401_get_lut_mode), };

pub unsafe fn dcn401_mpc_construct(mpc401: *mut dcn401_mpc, ctx: *mut dc_context, mpc_regs: *const dcn401_mpc_registers, mpc_shift: *const dcn401_mpc_shift, mpc_mask: *const dcn401_mpc_mask, num_mpcc: i32, num_rmu: i32) {
    (*mpc401).base.ctx = ctx; (*mpc401).base.funcs = &dcn401_mpc_funcs; (*mpc401).mpc_regs = mpc_regs; (*mpc401).mpc_shift = mpc_shift; (*mpc401).mpc_mask = mpc_mask; (*mpc401).mpcc_in_use_mask = 0; (*mpc401).num_mpcc = num_mpcc; (*mpc401).num_rmu = num_rmu;
    for i in 0..MAX_MPCC { mpc3_init_mpcc(&mut (*mpc401).base.mpcc_array[i], i as i32); }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
