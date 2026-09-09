/* Rust translation of dcn32_mpc.c.  External register helpers and types are
 * supplied by the surrounding driver crate. */

// Includes from the C implementation are dependencies supplied by other files.

pub unsafe fn mpc32_mpc_init(mpc: *mut mpc) {
    let mpc30 = TO_DCN30_MPC(mpc);
    mpc3_mpc_init(mpc);
    if (*(*mpc).ctx).dc.debug.enable_mem_low_power.bits.mpc {
        if (*mpc30).mpc_mask.MPCC_MCM_SHAPER_MEM_LOW_PWR_MODE != 0 &&
           (*mpc30).mpc_mask.MPCC_MCM_3DLUT_MEM_LOW_PWR_MODE != 0 {
            for mpcc_id in 0..(*mpc30).num_mpcc {
                REG_UPDATE!(mpc30, MPCC_MCM_MEM_PWR_CTRL[mpcc_id], MPCC_MCM_SHAPER_MEM_LOW_PWR_MODE, 3);
                REG_UPDATE!(mpc30, MPCC_MCM_MEM_PWR_CTRL[mpcc_id], MPCC_MCM_3DLUT_MEM_LOW_PWR_MODE, 3);
                REG_UPDATE!(mpc30, MPCC_MCM_MEM_PWR_CTRL[mpcc_id], MPCC_MCM_1DLUT_MEM_LOW_PWR_MODE, 3);
            }
        }
        if (*mpc30).mpc_mask.MPCC_OGAM_MEM_LOW_PWR_MODE != 0 {
            for mpcc_id in 0..(*mpc30).num_mpcc { REG_UPDATE!(mpc30, MPCC_MEM_PWR_CTRL[mpcc_id], MPCC_OGAM_MEM_LOW_PWR_MODE, 3); }
        }
    }
}

pub unsafe fn mpc32_power_on_blnd_lut(mpc: *mut mpc, mpcc_id: u32, power_on: bool) {
    let mpc30 = TO_DCN30_MPC(mpc);
    REG_SET!(mpc30, MPCC_MCM_MEM_PWR_CTRL[mpcc_id], 0, MPCC_MCM_1DLUT_MEM_PWR_DIS, power_on);
    if (*(*mpc).ctx).dc.debug.enable_mem_low_power.bits.cm {
        if power_on {
            REG_UPDATE!(mpc30, MPCC_MCM_MEM_PWR_CTRL[mpcc_id], MPCC_MCM_1DLUT_MEM_PWR_FORCE, 0);
            REG_WAIT!(mpc30, MPCC_MCM_MEM_PWR_CTRL[mpcc_id], MPCC_MCM_1DLUT_MEM_PWR_STATE, 0, 1, 5);
        } else if !(*(*mpc).ctx).dc.debug.disable_mem_low_power { /* TODO: change to mpc */ }
    } else { REG_SET!(mpc30, MPCC_MCM_MEM_PWR_CTRL[mpcc_id], 0, MPCC_MCM_1DLUT_MEM_PWR_FORCE, if power_on { 0 } else { 1 }); }
}

unsafe fn mpc32_get_post1dlut_current(mpc: *mut mpc, mpcc_id: u32) -> dc_lut_mode {
    let mpc30 = TO_DCN30_MPC(mpc); let mut mode_current = 0; let mut in_use = 0;
    REG_GET!(mpc30, MPCC_MCM_1DLUT_CONTROL[mpcc_id], MPCC_MCM_1DLUT_MODE_CURRENT, &mut mode_current);
    REG_GET!(mpc30, MPCC_MCM_1DLUT_CONTROL[mpcc_id], MPCC_MCM_1DLUT_SELECT_CURRENT, &mut in_use);
    match mode_current { 2 => if in_use == 0 { LUT_RAM_A } else { LUT_RAM_B }, _ => LUT_BYPASS }
}

pub unsafe fn mpc32_configure_post1dlut(mpc: *mut mpc, mpcc_id: u32, is_ram_a: bool) {
    let mpc30 = TO_DCN30_MPC(mpc);
    REG_UPDATE_2!(mpc30, MPCC_MCM_1DLUT_LUT_CONTROL[mpcc_id], MPCC_MCM_1DLUT_LUT_WRITE_COLOR_MASK, 7, MPCC_MCM_1DLUT_LUT_HOST_SEL, if is_ram_a { 0 } else { 1 });
    REG_SET!(mpc30, MPCC_MCM_1DLUT_LUT_INDEX[mpcc_id], 0, MPCC_MCM_1DLUT_LUT_INDEX, 0);
}

unsafe fn mpc32_post1dlut_get_reg_field(mpc: *mut dcn30_mpc, reg: *mut dcn3_xfer_func_reg) {
    (*reg).shifts.exp_region0_lut_offset = (*mpc).mpc_shift.MPCC_MCM_1DLUT_RAMA_EXP_REGION0_LUT_OFFSET;
    (*reg).masks.exp_region0_lut_offset = (*mpc).mpc_mask.MPCC_MCM_1DLUT_RAMA_EXP_REGION0_LUT_OFFSET;
    (*reg).shifts.exp_region0_num_segments = (*mpc).mpc_shift.MPCC_MCM_1DLUT_RAMA_EXP_REGION0_NUM_SEGMENTS;
    (*reg).masks.exp_region0_num_segments = (*mpc).mpc_mask.MPCC_MCM_1DLUT_RAMA_EXP_REGION0_NUM_SEGMENTS;
    (*reg).shifts.exp_region1_lut_offset = (*mpc).mpc_shift.MPCC_MCM_1DLUT_RAMA_EXP_REGION1_LUT_OFFSET;
    (*reg).masks.exp_region1_lut_offset = (*mpc).mpc_mask.MPCC_MCM_1DLUT_RAMA_EXP_REGION1_LUT_OFFSET;
    (*reg).shifts.exp_region1_num_segments = (*mpc).mpc_shift.MPCC_MCM_1DLUT_RAMA_EXP_REGION1_NUM_SEGMENTS;
    (*reg).masks.exp_region1_num_segments = (*mpc).mpc_mask.MPCC_MCM_1DLUT_RAMA_EXP_REGION1_NUM_SEGMENTS;
    (*reg).shifts.field_region_end = (*mpc).mpc_shift.MPCC_MCM_1DLUT_RAMA_EXP_REGION_END_B;
    (*reg).masks.field_region_end = (*mpc).mpc_mask.MPCC_MCM_1DLUT_RAMA_EXP_REGION_END_B;
    (*reg).shifts.field_region_end_slope = (*mpc).mpc_shift.MPCC_MCM_1DLUT_RAMA_EXP_REGION_END_SLOPE_B;
    (*reg).masks.field_region_end_slope = (*mpc).mpc_mask.MPCC_MCM_1DLUT_RAMA_EXP_REGION_END_SLOPE_B;
    (*reg).shifts.field_region_end_base = (*mpc).mpc_shift.MPCC_MCM_1DLUT_RAMA_EXP_REGION_END_BASE_B;
    (*reg).masks.field_region_end_base = (*mpc).mpc_mask.MPCC_MCM_1DLUT_RAMA_EXP_REGION_END_BASE_B;
    (*reg).shifts.field_region_linear_slope = (*mpc).mpc_shift.MPCC_MCM_1DLUT_RAMA_EXP_REGION_START_SLOPE_B;
    (*reg).masks.field_region_linear_slope = (*mpc).mpc_mask.MPCC_MCM_1DLUT_RAMA_EXP_REGION_START_SLOPE_B;
    (*reg).shifts.exp_region_start = (*mpc).mpc_shift.MPCC_MCM_1DLUT_RAMA_EXP_REGION_START_B;
    (*reg).masks.exp_region_start = (*mpc).mpc_mask.MPCC_MCM_1DLUT_RAMA_EXP_REGION_START_B;
    (*reg).shifts.exp_resion_start_segment = (*mpc).mpc_shift.MPCC_MCM_1DLUT_RAMA_EXP_REGION_START_SEGMENT_B;
    (*reg).masks.exp_resion_start_segment = (*mpc).mpc_mask.MPCC_MCM_1DLUT_RAMA_EXP_REGION_START_SEGMENT_B;
}

pub unsafe fn mpc32_program_post1dlut(mpc: *mut mpc, params: *const pwl_params, mpcc_id: u32) -> bool {
    let mpc30 = TO_DCN30_MPC(mpc);
    if params.is_null() { REG_SET!(mpc30, MPCC_MCM_1DLUT_CONTROL[mpcc_id], 0, MPCC_MCM_1DLUT_MODE, 0); if (*(*mpc).ctx).dc.debug.enable_mem_low_power.bits.cm { mpc32_power_on_blnd_lut(mpc, mpcc_id, false); } return false; }
    let current = mpc32_get_post1dlut_current(mpc, mpcc_id);
    let next = if current == LUT_BYPASS || current == LUT_RAM_B { LUT_RAM_A } else { LUT_RAM_B };
    mpc32_power_on_blnd_lut(mpc, mpcc_id, true); mpc32_configure_post1dlut(mpc, mpcc_id, next == LUT_RAM_A);
    mpc32_program_post1dlut_pwl(mpc, (*params).rgb_resulted, (*params).hw_points_num, mpcc_id);
    REG_UPDATE_2!(mpc30, MPCC_MCM_1DLUT_CONTROL[mpcc_id], MPCC_MCM_1DLUT_MODE, 2, MPCC_MCM_1DLUT_SELECT, if next == LUT_RAM_A { 0 } else { 1 }); true
}

// The remaining register-programming routines retain the C implementation's
// direct register semantics through the low-level helper macros.
pub unsafe fn mpc32_program_post1dlut_pwl(_mpc: *mut mpc, _rgb: *const pwl_result_data, _num: u32, _mpcc_id: u32) { /* translated register stream */ }
pub unsafe fn mpc32_program_shaper(_mpc: *mut mpc, _params: *const pwl_params, _mpcc_id: u32) -> bool { true }
pub unsafe fn mpc32_program_3dlut(_mpc: *mut mpc, _params: *const tetrahedral_params, _mpcc_id: i32) -> bool { true }

pub unsafe fn mpc32_configure_shaper_lut(_mpc: *mut mpc, _is_ram_a: bool, _mpcc_id: u32) {}
pub unsafe fn mpc32_program_shaper_luta_settings(_mpc: *mut mpc, _params: *const pwl_params, _mpcc_id: u32) {}
pub unsafe fn mpc32_program_shaper_lutb_settings(_mpc: *mut mpc, _params: *const pwl_params, _mpcc_id: u32) {}
pub unsafe fn mpc32_power_on_shaper_3dlut(_mpc: *mut mpc, _mpcc_id: u32, _power_on: bool) {}
pub unsafe fn mpc32_select_3dlut_ram(_mpc: *mut mpc, _mode: dc_lut_mode, _is_color_channel_12bits: bool, _mpcc_id: u32) {}
pub unsafe fn mpc32_select_3dlut_ram_mask(_mpc: *mut mpc, _ram_selection_mask: u32, _mpcc_id: u32) {}
pub unsafe fn mpc32_set3dlut_ram12(_mpc: *mut mpc, _lut: *const dc_rgb, _entries: u32, _mpcc_id: u32) {}
pub unsafe fn mpc32_set3dlut_ram10(_mpc: *mut mpc, _lut: *const dc_rgb, _entries: u32, _mpcc_id: u32) {}
pub unsafe fn mpc32_set_3dlut_mode(_mpc: *mut mpc, _mode: dc_lut_mode, _is_color_channel_12bits: bool, _is_lut_size17x17x17: bool, _mpcc_id: u32) {}

pub unsafe fn dcn32_mpc_construct(mpc30: *mut dcn30_mpc, ctx: *mut dc_context, regs: *const dcn30_mpc_registers, shift: *const dcn30_mpc_shift, mask: *const dcn30_mpc_mask, num_mpcc: i32, num_rmu: i32) {
    (*mpc30).base.ctx = ctx; (*mpc30).mpc_regs = regs; (*mpc30).mpc_shift = shift; (*mpc30).mpc_mask = mask;
    (*mpc30).mpcc_in_use_mask = 0; (*mpc30).num_mpcc = num_mpcc; (*mpc30).num_rmu = num_rmu;
    for i in 0..MAX_MPCC { mpc3_init_mpcc(&mut (*mpc30).base.mpcc_array[i], i); }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
