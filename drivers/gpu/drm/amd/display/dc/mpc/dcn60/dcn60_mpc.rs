// SPDX-License-Identifier: MIT
//
// Copyright 2025 Advanced Micro Devices, Inc.

// C headers and register-helper macros are supplied by the surrounding tree.

unsafe fn mpc60_insert_plane(
    mpc: *mut mpc,
    tree: *mut mpc_tree,
    blnd_cfg: *mut mpcc_blnd_cfg,
    sm_cfg: *mut mpcc_sm_cfg,
    insert_above_mpcc: *mut mpcc,
    dpp_id: i32,
    mpcc_id: i32,
) -> *mut mpcc {
    let _ = sm_cfg;
    let mpc60 = TO_DCN60_MPC(mpc);
    let mut new_mpcc: *mut mpcc = core::ptr::null_mut();

    ASSERT(mpcc_id < (*mpc60).num_mpcc);
    ASSERT(((*mpc60).mpcc_in_use_mask & (1 << mpcc_id)) == 0);

    if !insert_above_mpcc.is_null() {
        let mut temp_mpcc = (*tree).opp_list;
        if temp_mpcc != insert_above_mpcc {
            while !temp_mpcc.is_null() && (*temp_mpcc).mpcc_bot != insert_above_mpcc {
                temp_mpcc = (*temp_mpcc).mpcc_bot;
            }
        }
        if temp_mpcc.is_null() {
            return core::ptr::null_mut();
        }
    }

    new_mpcc = mpc1_get_mpcc(mpc, mpcc_id);
    (*new_mpcc).dpp_id = dpp_id;

    if !insert_above_mpcc.is_null() {
        (*new_mpcc).mpcc_bot = insert_above_mpcc;
        REG_SET!(mpc60, MPCC_BOT_SEL[mpcc_id], 0, MPCC_BOT_SEL, (*insert_above_mpcc).mpcc_id);
        REG_UPDATE!(mpc60, MPCC_CONTROL[mpcc_id], MPCC_MODE, MPCC_BLEND_MODE_TOP_BOT_BLENDING);
    } else {
        (*new_mpcc).mpcc_bot = core::ptr::null_mut();
        REG_SET!(mpc60, MPCC_BOT_SEL[mpcc_id], 0, MPCC_BOT_SEL, 0xf);
        REG_UPDATE!(mpc60, MPCC_CONTROL[mpcc_id], MPCC_MODE, MPCC_BLEND_MODE_TOP_LAYER_ONLY);
    }
    REG_SET!(mpc60, MPCC_TOP_SEL[mpcc_id], 0, MPCC_TOP_SEL, dpp_id);
    REG_SET!(mpc60, MPCC_OPP_ID[mpcc_id], 0, MPCC_OPP_ID, (*tree).opp_id);
    REG_SET!(mpc60, MPCC_UPDATE_LOCK_SEL[mpcc_id], 0, MPCC_UPDATE_LOCK_SEL, (*tree).opp_id);

    if (*tree).opp_list == insert_above_mpcc {
        (*tree).opp_list = new_mpcc;
        REG_UPDATE!(mpc60, MUX[(*tree).opp_id], MPC_OUT_MUX, mpcc_id);
    } else {
        let mut temp_mpcc = (*tree).opp_list;
        while !temp_mpcc.is_null() && (*temp_mpcc).mpcc_bot != insert_above_mpcc {
            temp_mpcc = (*temp_mpcc).mpcc_bot;
        }
        if !temp_mpcc.is_null() && (*temp_mpcc).mpcc_bot == insert_above_mpcc {
            REG_SET!(mpc60, MPCC_BOT_SEL[(*temp_mpcc).mpcc_id], 0, MPCC_BOT_SEL, mpcc_id);
            (*temp_mpcc).mpcc_bot = new_mpcc;
            if insert_above_mpcc.is_null() {
                REG_UPDATE!(mpc60, MPCC_CONTROL[(*temp_mpcc).mpcc_id], MPCC_MODE, MPCC_BLEND_MODE_TOP_BOT_BLENDING);
            }
        }
    }

    ((*mpc).funcs).update_blending(mpc, blnd_cfg, mpcc_id);
    (*mpc60).mpcc_in_use_mask |= 1 << mpcc_id;
    new_mpcc
}

pub unsafe fn mpc60_program_rmcm_lut_read_write_control(
    mpc: *mut mpc, id: MCM_LUT_ID, lut_bank_a: bool, enabled: bool, mpcc_id: i32,
) {
    let mpc60 = TO_DCN60_MPC(mpc);
    match id {
        MCM_LUT_ID::MCM_LUT_3DLUT => REG_UPDATE!(mpc60, MPC_RMCM_3DLUT_MODE[mpcc_id], MPC_RMCM_3DLUT_MODE, if !enabled { 0 } else if lut_bank_a { 1 } else { 2 }),
        MCM_LUT_ID::MCM_LUT_SHAPER => {
            REG_UPDATE!(mpc60, MPC_RMCM_SHAPER_LUT_WRITE_EN_MASK[mpcc_id], MPC_RMCM_SHAPER_LUT_WRITE_EN_MASK, 7);
            REG_UPDATE!(mpc60, MPC_RMCM_SHAPER_LUT_WRITE_EN_MASK[mpcc_id], MPC_RMCM_SHAPER_LUT_WRITE_SEL, if lut_bank_a { 0 } else { 1 });
            REG_SET!(mpc60, MPC_RMCM_SHAPER_LUT_INDEX[mpcc_id], 0, MPC_RMCM_SHAPER_LUT_INDEX, 0);
        }
        _ => {}
    }
}

unsafe fn mpc60_program_lut_read_write_control(mpc: *mut mpc, id: MCM_LUT_ID, lut_bank_a: bool, bit_depth: u32, mpcc_id: i32) {
    let mpc60 = TO_DCN60_MPC(mpc);
    match id {
        MCM_LUT_ID::MCM_LUT_3DLUT => { mpc32_select_3dlut_ram_mask(mpc, 0xf, mpcc_id); REG_UPDATE!(mpc60, MPCC_MCM_3DLUT_READ_WRITE_CONTROL[mpcc_id], MPCC_MCM_3DLUT_30BIT_EN, if bit_depth == 10 { 1 } else { 0 }); }
        MCM_LUT_ID::MCM_LUT_SHAPER => mpc32_configure_shaper_lut(mpc, lut_bank_a, mpcc_id),
        MCM_LUT_ID::MCM_LUT_1DLUT => mpc32_configure_post1dlut(mpc, lut_bank_a, mpcc_id),
    }
}

unsafe fn mpc60_cm_lut_size_to_3dlut_size(cm_size: dc_cm_lut_size) -> u32 {
    match cm_size { dc_cm_lut_size::CM_LUT_SIZE_999 => 1, dc_cm_lut_size::CM_LUT_SIZE_171717 => 0, _ => { ASSERT(false); 0 } }
}

unsafe fn mpc60_program_lut_mode(mpc: *mut mpc, id: MCM_LUT_ID, enable: bool, lut_bank_a: bool, size: dc_cm_lut_size, mpcc_id: i32) {
    let mpc60 = TO_DCN60_MPC(mpc);
    match id {
        MCM_LUT_ID::MCM_LUT_3DLUT => { if enable { REG_UPDATE_2!(mpc60, MPCC_MCM_3DLUT_MODE[mpcc_id], MPCC_MCM_3DLUT_MODE, 1, MPCC_MCM_3DLUT_SIZE, mpc60_cm_lut_size_to_3dlut_size(size)); } else { REG_UPDATE!(mpc60, MPCC_MCM_3DLUT_MODE[mpcc_id], MPCC_MCM_3DLUT_MODE, 0); } }
        MCM_LUT_ID::MCM_LUT_SHAPER => REG_UPDATE!(mpc60, MPCC_MCM_SHAPER_CONTROL[mpcc_id], MPCC_MCM_SHAPER_LUT_MODE, if enable { if lut_bank_a { 1 } else { 2 } } else { 0 }),
        MCM_LUT_ID::MCM_LUT_1DLUT => { REG_UPDATE!(mpc60, MPCC_MCM_1DLUT_CONTROL[mpcc_id], MPCC_MCM_1DLUT_MODE, if enable { 2 } else { 0 }); REG_UPDATE!(mpc60, MPCC_MCM_1DLUT_CONTROL[mpcc_id], MPCC_MCM_1DLUT_SELECT, if lut_bank_a { 0 } else { 1 }); }
    }
}

unsafe fn mpc60_program_3dlut(mpc: *mut mpc, params: *const tetrahedral_params, mpcc_id: i32) -> bool {
    let mut lut_params: mcm_lut_params = core::mem::zeroed();
    (*(&mut lut_params)).lut3d = params;
    mpc60_program_lut_read_write_control(mpc, MCM_LUT_ID::MCM_LUT_3DLUT, true, if (*params).use_12bits { 12 } else { 10 }, mpcc_id);
    mpc401_populate_lut(mpc, MCM_LUT_ID::MCM_LUT_3DLUT, &lut_params, true, mpcc_id);
    mpc60_program_lut_mode(mpc, MCM_LUT_ID::MCM_LUT_3DLUT, true, true, if (*params).use_tetrahedral_9 { dc_cm_lut_size::CM_LUT_SIZE_999 } else { dc_cm_lut_size::CM_LUT_SIZE_171717 }, mpcc_id);
    true
}

// Direct translation of the C function table; all referenced implementations
// are external symbols supplied by the surrounding MPC implementation.
static dcn60_mpc_funcs: mpc_funcs = mpc_funcs {
    read_mpcc_state: mpc1_read_mpcc_state,
    insert_plane: mpc60_insert_plane,
    remove_mpcc: mpc1_remove_mpcc,
    mpc_init: mpc32_mpc_init,
    mpc_init_single_inst: mpc3_mpc_init_single_inst,
    update_blending: mpc42_update_blending,
    cursor_lock: mpc1_cursor_lock,
    get_mpcc_for_dpp: mpc1_get_mpcc_for_dpp,
    wait_for_idle: mpc2_assert_idle_mpcc,
    assert_mpcc_idle_before_connect: mpc2_assert_mpcc_idle_before_connect,
    init_mpcc_list_from_hw: mpc1_init_mpcc_list_from_hw,
    set_denorm: mpc3_set_denorm,
    set_denorm_clamp: mpc3_set_denorm_clamp,
    set_output_csc: mpc3_set_output_csc,
    set_ocsc_default: mpc3_set_ocsc_default,
    set_output_gamma: mpc3_set_output_gamma,
    set_gamut_remap: mpc401_set_gamut_remap,
    program_shaper: mpc32_program_shaper,
    program_3dlut: mpc60_program_3dlut,
    program_1dlut: mpc32_program_post1dlut,
    power_on_mpc_mem_pwr: mpc3_power_on_ogam_lut,
    get_mpc_out_mux: mpc1_get_mpc_out_mux,
    mpc_read_reg_state: mpc3_read_reg_state,
    set_bg_color: mpc1_set_bg_color,
    set_movable_cm_location: mpc401_set_movable_cm_location,
    update_3dlut_fast_load_select: mpc401_update_3dlut_fast_load_select,
    get_3dlut_fast_load_status: mpc401_get_3dlut_fast_load_status,
    populate_lut: mpc401_populate_lut,
    program_lut_read_write_control: mpc60_program_lut_read_write_control,
    program_lut_mode: mpc60_program_lut_mode,
    get_lut_mode: mpc401_get_lut_mode,
    ..mpc_funcs::default()
};

pub unsafe fn dcn60_mpc_construct(mpc60: *mut dcn60_mpc, ctx: *mut dc_context, mpc_regs: *const dcn60_mpc_registers, mpc_shift: *const dcn60_mpc_shift, mpc_mask: *const dcn60_mpc_mask, num_mpcc: i32, num_rmu: i32) {
    (*mpc60).base.ctx = ctx;
    (*mpc60).base.funcs = &dcn60_mpc_funcs;
    (*mpc60).mpc_regs = mpc_regs;
    (*mpc60).mpc_shift = mpc_shift;
    (*mpc60).mpc_mask = mpc_mask;
    (*mpc60).mpcc_in_use_mask = 0;
    (*mpc60).num_mpcc = num_mpcc;
    (*mpc60).num_rmu = num_rmu;
    for i in 0..MAX_MPCC { mpc42_init_mpcc(&mut (*mpc60).base.mpcc_array[i], i); }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
