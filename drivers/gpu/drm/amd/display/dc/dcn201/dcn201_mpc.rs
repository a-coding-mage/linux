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
 * The above copyright notice and this permission notice shall be included in
 * all copies or substantial portions of the Software.
 *
 * THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
 * IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
 * FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT.  IN NO EVENT SHALL
 * THE COPYRIGHT HOLDER(S) OR AUTHOR(S) BE LIABLE FOR ANY CLAIM, DAMAGES OR
 * OTHER LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE,
 * ARISING FROM, OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR
 * OTHER DEALINGS IN THE SOFTWARE.
 *
 * Authors: AMD
 */

// Dependencies are supplied by the surrounding translation unit.

unsafe fn mpc201_set_out_rate_control(
    mpc: *mut mpc,
    opp_id: i32,
    enable: bool,
    rate_2x_mode: bool,
    flow_control: *mut mpc_dwb_flow_control,
) {
    let mpc201: *mut dcn201_mpc = TO_DCN201_MPC(mpc);

    REG_UPDATE_2!(
        (*(*mpc201).mpc_regs).MUX[opp_id as usize],
        MPC_OUT_RATE_CONTROL_DISABLE, !enable,
        MPC_OUT_RATE_CONTROL, rate_2x_mode
    );

    if !flow_control.is_null() {
        REG_UPDATE_3!(
            (*(*mpc201).mpc_regs).MUX[opp_id as usize],
            MPC_OUT_FLOW_CONTROL_MODE, (*flow_control).flow_ctrl_mode,
            MPC_OUT_FLOW_CONTROL_COUNT0, (*flow_control).flow_ctrl_cnt0,
            MPC_OUT_FLOW_CONTROL_COUNT1, (*flow_control).flow_ctrl_cnt1
        );
    }
}

unsafe fn mpc201_init_mpcc(mpcc: *mut mpcc, mpcc_inst: i32) {
    (*mpcc).mpcc_id = mpcc_inst;
    (*mpcc).dpp_id = 0xf;
    (*mpcc).mpcc_bot = core::ptr::null_mut();
    (*mpcc).blnd_cfg.overlap_only = false;
    (*mpcc).blnd_cfg.global_alpha = 0xff;
    (*mpcc).blnd_cfg.global_gain = 0xff;
    (*mpcc).blnd_cfg.background_color_bpc = 4;
    (*mpcc).blnd_cfg.bottom_gain_mode = 0;
    (*mpcc).blnd_cfg.top_gain = 0x1f000;
    (*mpcc).blnd_cfg.bottom_inside_gain = 0x1f000;
    (*mpcc).blnd_cfg.bottom_outside_gain = 0x1f000;
    (*mpcc).sm_cfg.enable = false;
    (*mpcc).shared_bottom = false;
}

static dcn201_mpc_funcs dcn201_mpc_funcs = dcn201_mpc_funcs {
    read_mpcc_state: Some(mpc1_read_mpcc_state),
    insert_plane: Some(mpc1_insert_plane),
    remove_mpcc: Some(mpc1_remove_mpcc),
    mpc_init: Some(mpc1_mpc_init),
    mpc_init_single_inst: Some(mpc1_mpc_init_single_inst),
    update_blending: Some(mpc2_update_blending),
    cursor_lock: Some(mpc1_cursor_lock),
    get_mpcc_for_dpp: Some(mpc1_get_mpcc_for_dpp),
    get_mpcc_for_dpp_from_secondary: None,
    wait_for_idle: Some(mpc2_assert_idle_mpcc),
    assert_mpcc_idle_before_connect: Some(mpc2_assert_mpcc_idle_before_connect),
    init_mpcc_list_from_hw: Some(mpc1_init_mpcc_list_from_hw),
    set_denorm: Some(mpc2_set_denorm),
    set_denorm_clamp: Some(mpc2_set_denorm_clamp),
    set_output_csc: Some(mpc2_set_output_csc),
    set_ocsc_default: Some(mpc2_set_ocsc_default),
    set_output_gamma: Some(mpc2_set_output_gamma),
    set_out_rate_control: Some(mpc201_set_out_rate_control),
    power_on_mpc_mem_pwr: Some(mpc20_power_on_ogam_lut),
    get_mpc_out_mux: Some(mpc1_get_mpc_out_mux),
    set_bg_color: Some(mpc1_set_bg_color),
};

unsafe fn dcn201_mpc_construct(
    mpc201: *mut dcn201_mpc,
    ctx: *mut dc_context,
    mpc_regs: *const dcn201_mpc_registers,
    mpc_shift: *const dcn201_mpc_shift,
    mpc_mask: *const dcn201_mpc_mask,
    num_mpcc: i32,
) {
    (*mpc201).base.ctx = ctx;
    (*mpc201).base.funcs = &dcn201_mpc_funcs;
    (*mpc201).mpc_regs = mpc_regs;
    (*mpc201).mpc_shift = mpc_shift;
    (*mpc201).mpc_mask = mpc_mask;
    (*mpc201).mpcc_in_use_mask = 0;
    (*mpc201).num_mpcc = num_mpcc;

    let mut i = 0;
    while i < MAX_MPCC {
        mpc201_init_mpcc(&mut (*mpc201).base.mpcc_array[i], i as i32);
        i += 1;
    }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
