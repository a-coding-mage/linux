// SPDX-License-Identifier: MIT
//
// Copyright 2026 Advanced Micro Devices, Inc.


	mpc42.mpc_regs.reg

	mpc42.base.ctx

	mpc42.mpc_shift.field_name, mpc42.mpc_mask.field_name


unsafe fn mpc42_init_mpcc(mpcc *mpcc, int mpcc_inst)
{
	mpcc.mpcc_id = mpcc_inst;
	mpcc.dpp_id = 0xf;
	mpcc.mpcc_bot = core::ptr::null_mut();
	mpcc.blnd_cfg.overlap_only = false;
	mpcc.blnd_cfg.global_alpha = 0xfff;
	mpcc.blnd_cfg.global_gain = 0xfff;
	mpcc.blnd_cfg.background_color_bpc = 4;
	mpcc.blnd_cfg.bottom_gain_mode = 0;
	mpcc.blnd_cfg.top_gain = 0x1f000;
	mpcc.blnd_cfg.bottom_inside_gain = 0x1f000;
	mpcc.blnd_cfg.bottom_outside_gain = 0x1f000;
	mpcc.sm_cfg.enable = false;
	mpcc.shared_bottom = false;
}

unsafe fn mpc42_update_blending(
	mpc *mpc,
	mpcc_blnd_cfg *blnd_cfg,
	int mpcc_id)
{
	dcn42_mpc *mpc42 = TO_DCN42_MPC(mpc);

	mpcc *mpcc = mpc1_get_mpcc(mpc, mpcc_id);

	reg_update_5!(MPCC_CONTROL[mpcc_id],
			MPCC_ALPHA_BLND_MODE,		blnd_cfg.alpha_mode,
			MPCC_ALPHA_MULTIPLIED_MODE,	blnd_cfg.pre_multiplied_alpha,
			MPCC_BLND_ACTIVE_OVERLAP_ONLY,	blnd_cfg.overlap_only,
			MPCC_BG_BPC,			blnd_cfg.background_color_bpc,
			MPCC_BOT_GAIN_MODE,		blnd_cfg.bottom_gain_mode);
	reg_update_2!(MPCC_CONTROL2[mpcc_id],
			MPCC_GLOBAL_ALPHA,		blnd_cfg.global_alpha,
			MPCC_GLOBAL_GAIN,		blnd_cfg.global_gain);

	reg_set!(MPCC_TOP_GAIN[mpcc_id], 0, MPCC_TOP_GAIN, blnd_cfg.top_gain);
	reg_set!(MPCC_BOT_GAIN_INSIDE[mpcc_id], 0, MPCC_BOT_GAIN_INSIDE, blnd_cfg.bottom_inside_gain);
	reg_set!(MPCC_BOT_GAIN_OUTSIDE[mpcc_id], 0, MPCC_BOT_GAIN_OUTSIDE, blnd_cfg.bottom_outside_gain);

	mpcc.blnd_cfg = *blnd_cfg;
}

/* RMCM Shaper functions */
unsafe fn mpc42_power_on_rmcm_shaper_3dlut(
	mpc *mpc,
	u32 mpcc_id,
	bool power_on)
{
	u32 power_status_shaper = 2;
	u32 power_status_3dlut  = 2;
	dcn42_mpc *mpc42 = TO_DCN42_MPC(mpc);
	int max_retries = 10;

	reg_set!(MPC_RMCM_MEM_PWR_CTRL[mpcc_id], 0,
		MPC_RMCM_3DLUT_MEM_PWR_DIS, power_on == true ? 0 : 1);
	reg_set!(MPC_RMCM_MEM_PWR_CTRL[mpcc_id], 0,
		MPC_RMCM_SHAPER_MEM_PWR_DIS, power_on == true ? 0 : 1);
	/* wait for memory to fully power up */
	if (power_on && mpc.ctx.dc.debug.enable_mem_low_power.bits.mpc) {
		REG_WAIT(MPC_RMCM_MEM_PWR_CTRL[mpcc_id], MPC_RMCM_SHAPER_MEM_PWR_STATE, 0, 1, max_retries);
		REG_WAIT(MPC_RMCM_MEM_PWR_CTRL[mpcc_id], MPC_RMCM_3DLUT_MEM_PWR_STATE, 0, 1, max_retries);
	}

	/*read status is not mandatory, it is just for debugging*/
	reg_get!(MPC_RMCM_MEM_PWR_CTRL[mpcc_id], MPC_RMCM_SHAPER_MEM_PWR_STATE, &power_status_shaper);
	reg_get!(MPC_RMCM_MEM_PWR_CTRL[mpcc_id], MPC_RMCM_3DLUT_MEM_PWR_STATE, &power_status_3dlut);

	if (power_status_shaper != 0 && power_on == true)
		break_to_debugger!();

	if (power_status_3dlut != 0 && power_on == true)
		break_to_debugger!();
}

unsafe fn mpc42_configure_rmcm_shaper_lut(
	mpc *mpc,
	bool is_ram_a,
	u32 mpcc_id)
{
	dcn42_mpc *mpc42 = TO_DCN42_MPC(mpc);

	reg_update!(MPC_RMCM_SHAPER_SCALE_G_B[mpcc_id],
		MPC_RMCM_SHAPER_SCALE_B, 0x7000);
	reg_update!(MPC_RMCM_SHAPER_SCALE_G_B[mpcc_id],
		MPC_RMCM_SHAPER_SCALE_G, 0x7000);
	reg_update!(MPC_RMCM_SHAPER_SCALE_R[mpcc_id],
		MPC_RMCM_SHAPER_SCALE_R, 0x7000);
	reg_update!(MPC_RMCM_SHAPER_LUT_WRITE_EN_MASK[mpcc_id],
			MPC_RMCM_SHAPER_LUT_WRITE_EN_MASK, 7);
	reg_update!(MPC_RMCM_SHAPER_LUT_WRITE_EN_MASK[mpcc_id],
			MPC_RMCM_SHAPER_LUT_WRITE_SEL, is_ram_a == true ? 0:1);
	reg_set!(MPC_RMCM_SHAPER_LUT_INDEX[mpcc_id], 0, MPC_RMCM_SHAPER_LUT_INDEX, 0);
}

unsafe fn mpc42_program_rmcm_shaper_luta_settings(
	mpc *mpc,
	const pwl_params *params,
	u32 mpcc_id)
{
	const gamma_curve *curve;
	dcn42_mpc *mpc42 = TO_DCN42_MPC(mpc);

	reg_set_2!(MPC_RMCM_SHAPER_RAMA_START_CNTL_B[mpcc_id], 0,
		MPC_RMCM_SHAPER_RAMA_EXP_REGION_START_B, params.corner_points[0].blue.custom_float_x,
		MPC_RMCM_SHAPER_RAMA_EXP_REGION_START_SEGMENT_B, 0);
	reg_set_2!(MPC_RMCM_SHAPER_RAMA_START_CNTL_G[mpcc_id], 0,
			MPC_RMCM_SHAPER_RAMA_EXP_REGION_START_B, params.corner_points[0].green.custom_float_x,
			MPC_RMCM_SHAPER_RAMA_EXP_REGION_START_SEGMENT_B, 0);
	reg_set_2!(MPC_RMCM_SHAPER_RAMA_START_CNTL_R[mpcc_id], 0,
			MPC_RMCM_SHAPER_RAMA_EXP_REGION_START_B, params.corner_points[0].red.custom_float_x,
			MPC_RMCM_SHAPER_RAMA_EXP_REGION_START_SEGMENT_B, 0);

	reg_set_2!(MPC_RMCM_SHAPER_RAMA_END_CNTL_B[mpcc_id], 0,
			MPC_RMCM_SHAPER_RAMA_EXP_REGION_END_B, params.corner_points[1].blue.custom_float_x,
			MPC_RMCM_SHAPER_RAMA_EXP_REGION_END_BASE_B, params.corner_points[1].blue.custom_float_y);
	reg_set_2!(MPC_RMCM_SHAPER_RAMA_END_CNTL_G[mpcc_id], 0,
			MPC_RMCM_SHAPER_RAMA_EXP_REGION_END_B, params.corner_points[1].green.custom_float_x,
			MPC_RMCM_SHAPER_RAMA_EXP_REGION_END_BASE_B, params.corner_points[1].green.custom_float_y);
	reg_set_2!(MPC_RMCM_SHAPER_RAMA_END_CNTL_R[mpcc_id], 0,
			MPC_RMCM_SHAPER_RAMA_EXP_REGION_END_B, params.corner_points[1].red.custom_float_x,
			MPC_RMCM_SHAPER_RAMA_EXP_REGION_END_BASE_B, params.corner_points[1].red.custom_float_y);

	curve = params.arr_curve_points;
	if (curve) {
		reg_set_4!(MPC_RMCM_SHAPER_RAMA_REGION_0_1[mpcc_id], 0,
			MPC_RMCM_SHAPER_RAMA_EXP_REGION0_LUT_OFFSET, curve[0].offset,
			MPC_RMCM_SHAPER_RAMA_EXP_REGION0_NUM_SEGMENTS, curve[0].segments_num,
			MPC_RMCM_SHAPER_RAMA_EXP_REGION1_LUT_OFFSET, curve[1].offset,
			MPC_RMCM_SHAPER_RAMA_EXP_REGION1_NUM_SEGMENTS, curve[1].segments_num);

		curve += 2;
		reg_set_4!(MPC_RMCM_SHAPER_RAMA_REGION_2_3[mpcc_id], 0,
			MPC_RMCM_SHAPER_RAMA_EXP_REGION0_LUT_OFFSET, curve[0].offset,
			MPC_RMCM_SHAPER_RAMA_EXP_REGION0_NUM_SEGMENTS, curve[0].segments_num,
			MPC_RMCM_SHAPER_RAMA_EXP_REGION1_LUT_OFFSET, curve[1].offset,
			MPC_RMCM_SHAPER_RAMA_EXP_REGION1_NUM_SEGMENTS, curve[1].segments_num);

		curve += 2;
		reg_set_4!(MPC_RMCM_SHAPER_RAMA_REGION_4_5[mpcc_id], 0,
			MPC_RMCM_SHAPER_RAMA_EXP_REGION0_LUT_OFFSET, curve[0].offset,
			MPC_RMCM_SHAPER_RAMA_EXP_REGION0_NUM_SEGMENTS, curve[0].segments_num,
			MPC_RMCM_SHAPER_RAMA_EXP_REGION1_LUT_OFFSET, curve[1].offset,
			MPC_RMCM_SHAPER_RAMA_EXP_REGION1_NUM_SEGMENTS, curve[1].segments_num);

		curve += 2;
		reg_set_4!(MPC_RMCM_SHAPER_RAMA_REGION_6_7[mpcc_id], 0,
			MPC_RMCM_SHAPER_RAMA_EXP_REGION0_LUT_OFFSET, curve[0].offset,
			MPC_RMCM_SHAPER_RAMA_EXP_REGION0_NUM_SEGMENTS, curve[0].segments_num,
			MPC_RMCM_SHAPER_RAMA_EXP_REGION1_LUT_OFFSET, curve[1].offset,
			MPC_RMCM_SHAPER_RAMA_EXP_REGION1_NUM_SEGMENTS, curve[1].segments_num);

		curve += 2;
		reg_set_4!(MPC_RMCM_SHAPER_RAMA_REGION_8_9[mpcc_id], 0,
			MPC_RMCM_SHAPER_RAMA_EXP_REGION0_LUT_OFFSET, curve[0].offset,
			MPC_RMCM_SHAPER_RAMA_EXP_REGION0_NUM_SEGMENTS, curve[0].segments_num,
			MPC_RMCM_SHAPER_RAMA_EXP_REGION1_LUT_OFFSET, curve[1].offset,
			MPC_RMCM_SHAPER_RAMA_EXP_REGION1_NUM_SEGMENTS, curve[1].segments_num);

		curve += 2;
		reg_set_4!(MPC_RMCM_SHAPER_RAMA_REGION_10_11[mpcc_id], 0,
			MPC_RMCM_SHAPER_RAMA_EXP_REGION0_LUT_OFFSET, curve[0].offset,
			MPC_RMCM_SHAPER_RAMA_EXP_REGION0_NUM_SEGMENTS, curve[0].segments_num,
			MPC_RMCM_SHAPER_RAMA_EXP_REGION1_LUT_OFFSET, curve[1].offset,
			MPC_RMCM_SHAPER_RAMA_EXP_REGION1_NUM_SEGMENTS, curve[1].segments_num);

		curve += 2;
		reg_set_4!(MPC_RMCM_SHAPER_RAMA_REGION_12_13[mpcc_id], 0,
			MPC_RMCM_SHAPER_RAMA_EXP_REGION0_LUT_OFFSET, curve[0].offset,
			MPC_RMCM_SHAPER_RAMA_EXP_REGION0_NUM_SEGMENTS, curve[0].segments_num,
			MPC_RMCM_SHAPER_RAMA_EXP_REGION1_LUT_OFFSET, curve[1].offset,
			MPC_RMCM_SHAPER_RAMA_EXP_REGION1_NUM_SEGMENTS, curve[1].segments_num);

		curve += 2;
		reg_set_4!(MPC_RMCM_SHAPER_RAMA_REGION_14_15[mpcc_id], 0,
			MPC_RMCM_SHAPER_RAMA_EXP_REGION0_LUT_OFFSET, curve[0].offset,
			MPC_RMCM_SHAPER_RAMA_EXP_REGION0_NUM_SEGMENTS, curve[0].segments_num,
			MPC_RMCM_SHAPER_RAMA_EXP_REGION1_LUT_OFFSET, curve[1].offset,
			MPC_RMCM_SHAPER_RAMA_EXP_REGION1_NUM_SEGMENTS, curve[1].segments_num);


		curve += 2;
		reg_set_4!(MPC_RMCM_SHAPER_RAMA_REGION_16_17[mpcc_id], 0,
			MPC_RMCM_SHAPER_RAMA_EXP_REGION0_LUT_OFFSET, curve[0].offset,
			MPC_RMCM_SHAPER_RAMA_EXP_REGION0_NUM_SEGMENTS, curve[0].segments_num,
			MPC_RMCM_SHAPER_RAMA_EXP_REGION1_LUT_OFFSET, curve[1].offset,
			MPC_RMCM_SHAPER_RAMA_EXP_REGION1_NUM_SEGMENTS, curve[1].segments_num);

		curve += 2;
		reg_set_4!(MPC_RMCM_SHAPER_RAMA_REGION_18_19[mpcc_id], 0,
			MPC_RMCM_SHAPER_RAMA_EXP_REGION0_LUT_OFFSET, curve[0].offset,
			MPC_RMCM_SHAPER_RAMA_EXP_REGION0_NUM_SEGMENTS, curve[0].segments_num,
			MPC_RMCM_SHAPER_RAMA_EXP_REGION1_LUT_OFFSET, curve[1].offset,
			MPC_RMCM_SHAPER_RAMA_EXP_REGION1_NUM_SEGMENTS, curve[1].segments_num);

		curve += 2;
		reg_set_4!(MPC_RMCM_SHAPER_RAMA_REGION_20_21[mpcc_id], 0,
			MPC_RMCM_SHAPER_RAMA_EXP_REGION0_LUT_OFFSET, curve[0].offset,
			MPC_RMCM_SHAPER_RAMA_EXP_REGION0_NUM_SEGMENTS, curve[0].segments_num,
			MPC_RMCM_SHAPER_RAMA_EXP_REGION1_LUT_OFFSET, curve[1].offset,
			MPC_RMCM_SHAPER_RAMA_EXP_REGION1_NUM_SEGMENTS, curve[1].segments_num);

		curve += 2;
		reg_set_4!(MPC_RMCM_SHAPER_RAMA_REGION_22_23[mpcc_id], 0,
			MPC_RMCM_SHAPER_RAMA_EXP_REGION0_LUT_OFFSET, curve[0].offset,
			MPC_RMCM_SHAPER_RAMA_EXP_REGION0_NUM_SEGMENTS, curve[0].segments_num,
			MPC_RMCM_SHAPER_RAMA_EXP_REGION1_LUT_OFFSET, curve[1].offset,
			MPC_RMCM_SHAPER_RAMA_EXP_REGION1_NUM_SEGMENTS, curve[1].segments_num);

		curve += 2;
		reg_set_4!(MPC_RMCM_SHAPER_RAMA_REGION_24_25[mpcc_id], 0,
			MPC_RMCM_SHAPER_RAMA_EXP_REGION0_LUT_OFFSET, curve[0].offset,
			MPC_RMCM_SHAPER_RAMA_EXP_REGION0_NUM_SEGMENTS, curve[0].segments_num,
			MPC_RMCM_SHAPER_RAMA_EXP_REGION1_LUT_OFFSET, curve[1].offset,
			MPC_RMCM_SHAPER_RAMA_EXP_REGION1_NUM_SEGMENTS, curve[1].segments_num);

		curve += 2;
		reg_set_4!(MPC_RMCM_SHAPER_RAMA_REGION_26_27[mpcc_id], 0,
				MPC_RMCM_SHAPER_RAMA_EXP_REGION0_LUT_OFFSET, curve[0].offset,
				MPC_RMCM_SHAPER_RAMA_EXP_REGION0_NUM_SEGMENTS, curve[0].segments_num,
				MPC_RMCM_SHAPER_RAMA_EXP_REGION1_LUT_OFFSET, curve[1].offset,
				MPC_RMCM_SHAPER_RAMA_EXP_REGION1_NUM_SEGMENTS, curve[1].segments_num);

		curve += 2;
		reg_set_4!(MPC_RMCM_SHAPER_RAMA_REGION_28_29[mpcc_id], 0,
			MPC_RMCM_SHAPER_RAMA_EXP_REGION0_LUT_OFFSET, curve[0].offset,
			MPC_RMCM_SHAPER_RAMA_EXP_REGION0_NUM_SEGMENTS, curve[0].segments_num,
			MPC_RMCM_SHAPER_RAMA_EXP_REGION1_LUT_OFFSET, curve[1].offset,
			MPC_RMCM_SHAPER_RAMA_EXP_REGION1_NUM_SEGMENTS, curve[1].segments_num);

		curve += 2;
		reg_set_4!(MPC_RMCM_SHAPER_RAMA_REGION_30_31[mpcc_id], 0,
			MPC_RMCM_SHAPER_RAMA_EXP_REGION0_LUT_OFFSET, curve[0].offset,
			MPC_RMCM_SHAPER_RAMA_EXP_REGION0_NUM_SEGMENTS, curve[0].segments_num,
			MPC_RMCM_SHAPER_RAMA_EXP_REGION1_LUT_OFFSET, curve[1].offset,
			MPC_RMCM_SHAPER_RAMA_EXP_REGION1_NUM_SEGMENTS, curve[1].segments_num);

		curve += 2;
		reg_set_4!(MPC_RMCM_SHAPER_RAMA_REGION_32_33[mpcc_id], 0,
			MPC_RMCM_SHAPER_RAMA_EXP_REGION0_LUT_OFFSET, curve[0].offset,
			MPC_RMCM_SHAPER_RAMA_EXP_REGION0_NUM_SEGMENTS, curve[0].segments_num,
			MPC_RMCM_SHAPER_RAMA_EXP_REGION1_LUT_OFFSET, curve[1].offset,
			MPC_RMCM_SHAPER_RAMA_EXP_REGION1_NUM_SEGMENTS, curve[1].segments_num);
	}
}


unsafe fn mpc42_program_rmcm_shaper_lutb_settings(
	mpc *mpc,
	const pwl_params *params,
	u32 mpcc_id)
{
	const gamma_curve *curve;
	dcn42_mpc *mpc42 = TO_DCN42_MPC(mpc);

	reg_set_2!(MPC_RMCM_SHAPER_RAMB_START_CNTL_B[mpcc_id], 0,
		MPC_RMCM_SHAPER_RAMB_EXP_REGION_START_B, params.corner_points[0].blue.custom_float_x,
		MPC_RMCM_SHAPER_RAMB_EXP_REGION_START_SEGMENT_B, 0);
	reg_set_2!(MPC_RMCM_SHAPER_RAMB_START_CNTL_G[mpcc_id], 0,
			MPC_RMCM_SHAPER_RAMB_EXP_REGION_START_B, params.corner_points[0].green.custom_float_x,
			MPC_RMCM_SHAPER_RAMB_EXP_REGION_START_SEGMENT_B, 0);
	reg_set_2!(MPC_RMCM_SHAPER_RAMB_START_CNTL_R[mpcc_id], 0,
			MPC_RMCM_SHAPER_RAMB_EXP_REGION_START_B, params.corner_points[0].red.custom_float_x,
			MPC_RMCM_SHAPER_RAMB_EXP_REGION_START_SEGMENT_B, 0);

	reg_set_2!(MPC_RMCM_SHAPER_RAMB_END_CNTL_B[mpcc_id], 0,
			MPC_RMCM_SHAPER_RAMB_EXP_REGION_END_B, params.corner_points[1].blue.custom_float_x,
			MPC_RMCM_SHAPER_RAMB_EXP_REGION_END_BASE_B, params.corner_points[1].blue.custom_float_y);
	reg_set_2!(MPC_RMCM_SHAPER_RAMB_END_CNTL_G[mpcc_id], 0,
			MPC_RMCM_SHAPER_RAMB_EXP_REGION_END_B, params.corner_points[1].green.custom_float_x,
			MPC_RMCM_SHAPER_RAMB_EXP_REGION_END_BASE_B, params.corner_points[1].green.custom_float_y);
	reg_set_2!(MPC_RMCM_SHAPER_RAMB_END_CNTL_R[mpcc_id], 0,
			MPC_RMCM_SHAPER_RAMB_EXP_REGION_END_B, params.corner_points[1].red.custom_float_x,
			MPC_RMCM_SHAPER_RAMB_EXP_REGION_END_BASE_B, params.corner_points[1].red.custom_float_y);

	curve = params.arr_curve_points;
	if (curve) {
		reg_set_4!(MPC_RMCM_SHAPER_RAMB_REGION_0_1[mpcc_id], 0,
			MPC_RMCM_SHAPER_RAMB_EXP_REGION0_LUT_OFFSET, curve[0].offset,
			MPC_RMCM_SHAPER_RAMB_EXP_REGION0_NUM_SEGMENTS, curve[0].segments_num,
			MPC_RMCM_SHAPER_RAMB_EXP_REGION1_LUT_OFFSET, curve[1].offset,
			MPC_RMCM_SHAPER_RAMB_EXP_REGION1_NUM_SEGMENTS, curve[1].segments_num);

		curve += 2;
		reg_set_4!(MPC_RMCM_SHAPER_RAMB_REGION_2_3[mpcc_id], 0,
				MPC_RMCM_SHAPER_RAMB_EXP_REGION0_LUT_OFFSET, curve[0].offset,
				MPC_RMCM_SHAPER_RAMB_EXP_REGION0_NUM_SEGMENTS, curve[0].segments_num,
				MPC_RMCM_SHAPER_RAMB_EXP_REGION1_LUT_OFFSET, curve[1].offset,
				MPC_RMCM_SHAPER_RAMB_EXP_REGION1_NUM_SEGMENTS, curve[1].segments_num);


		curve += 2;
		reg_set_4!(MPC_RMCM_SHAPER_RAMB_REGION_4_5[mpcc_id], 0,
				MPC_RMCM_SHAPER_RAMB_EXP_REGION0_LUT_OFFSET, curve[0].offset,
				MPC_RMCM_SHAPER_RAMB_EXP_REGION0_NUM_SEGMENTS, curve[0].segments_num,
				MPC_RMCM_SHAPER_RAMB_EXP_REGION1_LUT_OFFSET, curve[1].offset,
				MPC_RMCM_SHAPER_RAMB_EXP_REGION1_NUM_SEGMENTS, curve[1].segments_num);

		curve += 2;
		reg_set_4!(MPC_RMCM_SHAPER_RAMB_REGION_6_7[mpcc_id], 0,
				MPC_RMCM_SHAPER_RAMB_EXP_REGION0_LUT_OFFSET, curve[0].offset,
				MPC_RMCM_SHAPER_RAMB_EXP_REGION0_NUM_SEGMENTS, curve[0].segments_num,
				MPC_RMCM_SHAPER_RAMB_EXP_REGION1_LUT_OFFSET, curve[1].offset,
				MPC_RMCM_SHAPER_RAMB_EXP_REGION1_NUM_SEGMENTS, curve[1].segments_num);

		curve += 2;
		reg_set_4!(MPC_RMCM_SHAPER_RAMB_REGION_8_9[mpcc_id], 0,
			MPC_RMCM_SHAPER_RAMB_EXP_REGION0_LUT_OFFSET, curve[0].offset,
			MPC_RMCM_SHAPER_RAMB_EXP_REGION0_NUM_SEGMENTS, curve[0].segments_num,
			MPC_RMCM_SHAPER_RAMB_EXP_REGION1_LUT_OFFSET, curve[1].offset,
			MPC_RMCM_SHAPER_RAMB_EXP_REGION1_NUM_SEGMENTS, curve[1].segments_num);

		curve += 2;
		reg_set_4!(MPC_RMCM_SHAPER_RAMB_REGION_10_11[mpcc_id], 0,
				MPC_RMCM_SHAPER_RAMB_EXP_REGION0_LUT_OFFSET, curve[0].offset,
				MPC_RMCM_SHAPER_RAMB_EXP_REGION0_NUM_SEGMENTS, curve[0].segments_num,
				MPC_RMCM_SHAPER_RAMB_EXP_REGION1_LUT_OFFSET, curve[1].offset,
				MPC_RMCM_SHAPER_RAMB_EXP_REGION1_NUM_SEGMENTS, curve[1].segments_num);

		curve += 2;
		reg_set_4!(MPC_RMCM_SHAPER_RAMB_REGION_12_13[mpcc_id], 0,
				MPC_RMCM_SHAPER_RAMB_EXP_REGION0_LUT_OFFSET, curve[0].offset,
				MPC_RMCM_SHAPER_RAMB_EXP_REGION0_NUM_SEGMENTS, curve[0].segments_num,
				MPC_RMCM_SHAPER_RAMB_EXP_REGION1_LUT_OFFSET, curve[1].offset,
				MPC_RMCM_SHAPER_RAMB_EXP_REGION1_NUM_SEGMENTS, curve[1].segments_num);

		curve += 2;
		reg_set_4!(MPC_RMCM_SHAPER_RAMB_REGION_14_15[mpcc_id], 0,
				MPC_RMCM_SHAPER_RAMB_EXP_REGION0_LUT_OFFSET, curve[0].offset,
				MPC_RMCM_SHAPER_RAMB_EXP_REGION0_NUM_SEGMENTS, curve[0].segments_num,
				MPC_RMCM_SHAPER_RAMB_EXP_REGION1_LUT_OFFSET, curve[1].offset,
				MPC_RMCM_SHAPER_RAMB_EXP_REGION1_NUM_SEGMENTS, curve[1].segments_num);

		curve += 2;
		reg_set_4!(MPC_RMCM_SHAPER_RAMB_REGION_16_17[mpcc_id], 0,
				MPC_RMCM_SHAPER_RAMB_EXP_REGION0_LUT_OFFSET, curve[0].offset,
				MPC_RMCM_SHAPER_RAMB_EXP_REGION0_NUM_SEGMENTS, curve[0].segments_num,
				MPC_RMCM_SHAPER_RAMB_EXP_REGION1_LUT_OFFSET, curve[1].offset,
				MPC_RMCM_SHAPER_RAMB_EXP_REGION1_NUM_SEGMENTS, curve[1].segments_num);

		curve += 2;
		reg_set_4!(MPC_RMCM_SHAPER_RAMB_REGION_18_19[mpcc_id], 0,
				MPC_RMCM_SHAPER_RAMB_EXP_REGION0_LUT_OFFSET, curve[0].offset,
				MPC_RMCM_SHAPER_RAMB_EXP_REGION0_NUM_SEGMENTS, curve[0].segments_num,
				MPC_RMCM_SHAPER_RAMB_EXP_REGION1_LUT_OFFSET, curve[1].offset,
				MPC_RMCM_SHAPER_RAMB_EXP_REGION1_NUM_SEGMENTS, curve[1].segments_num);

		curve += 2;
		reg_set_4!(MPC_RMCM_SHAPER_RAMB_REGION_20_21[mpcc_id], 0,
				MPC_RMCM_SHAPER_RAMB_EXP_REGION0_LUT_OFFSET, curve[0].offset,
				MPC_RMCM_SHAPER_RAMB_EXP_REGION0_NUM_SEGMENTS, curve[0].segments_num,
				MPC_RMCM_SHAPER_RAMB_EXP_REGION1_LUT_OFFSET, curve[1].offset,
				MPC_RMCM_SHAPER_RAMB_EXP_REGION1_NUM_SEGMENTS, curve[1].segments_num);

		curve += 2;
		reg_set_4!(MPC_RMCM_SHAPER_RAMB_REGION_22_23[mpcc_id], 0,
				MPC_RMCM_SHAPER_RAMB_EXP_REGION0_LUT_OFFSET, curve[0].offset,
				MPC_RMCM_SHAPER_RAMB_EXP_REGION0_NUM_SEGMENTS, curve[0].segments_num,
				MPC_RMCM_SHAPER_RAMB_EXP_REGION1_LUT_OFFSET, curve[1].offset,
				MPC_RMCM_SHAPER_RAMB_EXP_REGION1_NUM_SEGMENTS, curve[1].segments_num);

		curve += 2;
		reg_set_4!(MPC_RMCM_SHAPER_RAMB_REGION_24_25[mpcc_id], 0,
				MPC_RMCM_SHAPER_RAMB_EXP_REGION0_LUT_OFFSET, curve[0].offset,
				MPC_RMCM_SHAPER_RAMB_EXP_REGION0_NUM_SEGMENTS, curve[0].segments_num,
				MPC_RMCM_SHAPER_RAMB_EXP_REGION1_LUT_OFFSET, curve[1].offset,
				MPC_RMCM_SHAPER_RAMB_EXP_REGION1_NUM_SEGMENTS, curve[1].segments_num);

		curve += 2;
		reg_set_4!(MPC_RMCM_SHAPER_RAMB_REGION_26_27[mpcc_id], 0,
				MPC_RMCM_SHAPER_RAMB_EXP_REGION0_LUT_OFFSET, curve[0].offset,
				MPC_RMCM_SHAPER_RAMB_EXP_REGION0_NUM_SEGMENTS, curve[0].segments_num,
				MPC_RMCM_SHAPER_RAMB_EXP_REGION1_LUT_OFFSET, curve[1].offset,
				MPC_RMCM_SHAPER_RAMB_EXP_REGION1_NUM_SEGMENTS, curve[1].segments_num);

		curve += 2;
		reg_set_4!(MPC_RMCM_SHAPER_RAMB_REGION_28_29[mpcc_id], 0,
				MPC_RMCM_SHAPER_RAMB_EXP_REGION0_LUT_OFFSET, curve[0].offset,
				MPC_RMCM_SHAPER_RAMB_EXP_REGION0_NUM_SEGMENTS, curve[0].segments_num,
				MPC_RMCM_SHAPER_RAMB_EXP_REGION1_LUT_OFFSET, curve[1].offset,
				MPC_RMCM_SHAPER_RAMB_EXP_REGION1_NUM_SEGMENTS, curve[1].segments_num);

		curve += 2;
		reg_set_4!(MPC_RMCM_SHAPER_RAMB_REGION_30_31[mpcc_id], 0,
				MPC_RMCM_SHAPER_RAMB_EXP_REGION0_LUT_OFFSET, curve[0].offset,
				MPC_RMCM_SHAPER_RAMB_EXP_REGION0_NUM_SEGMENTS, curve[0].segments_num,
				MPC_RMCM_SHAPER_RAMB_EXP_REGION1_LUT_OFFSET, curve[1].offset,
				MPC_RMCM_SHAPER_RAMB_EXP_REGION1_NUM_SEGMENTS, curve[1].segments_num);

		curve += 2;
		reg_set_4!(MPC_RMCM_SHAPER_RAMB_REGION_32_33[mpcc_id], 0,
				MPC_RMCM_SHAPER_RAMB_EXP_REGION0_LUT_OFFSET, curve[0].offset,
				MPC_RMCM_SHAPER_RAMB_EXP_REGION0_NUM_SEGMENTS, curve[0].segments_num,
				MPC_RMCM_SHAPER_RAMB_EXP_REGION1_LUT_OFFSET, curve[1].offset,
				MPC_RMCM_SHAPER_RAMB_EXP_REGION1_NUM_SEGMENTS, curve[1].segments_num);
	}
}

unsafe fn mpc42_program_rmcm_shaper_lut(
	mpc *mpc,
	const pwl_result_data *rgb,
	u32 num,
	u32 mpcc_id)
{
	u32 i, red, green, blue;
	u32  red_delta, green_delta, blue_delta;
	u32  red_value, green_value, blue_value;

	dcn42_mpc *mpc42 = TO_DCN42_MPC(mpc);

	for (i = 0; i < num; i++) {

		red   = rgb[i].red_reg;
		green = rgb[i].green_reg;
		blue  = rgb[i].blue_reg;

		red_delta   = rgb[i].delta_red_reg;
		green_delta = rgb[i].delta_green_reg;
		blue_delta  = rgb[i].delta_blue_reg;

		red_value   = ((red_delta   & 0x3ff) << 14) | (red   & 0x3fff);
		green_value = ((green_delta & 0x3ff) << 14) | (green & 0x3fff);
		blue_value  = ((blue_delta  & 0x3ff) << 14) | (blue  & 0x3fff);

		reg_set!(MPC_RMCM_SHAPER_LUT_DATA[mpcc_id], 0, MPC_RMCM_SHAPER_LUT_DATA, red_value);
		reg_set!(MPC_RMCM_SHAPER_LUT_DATA[mpcc_id], 0, MPC_RMCM_SHAPER_LUT_DATA, green_value);
		reg_set!(MPC_RMCM_SHAPER_LUT_DATA[mpcc_id], 0, MPC_RMCM_SHAPER_LUT_DATA, blue_value);
	}
}

unsafe fn mpc42_enable_3dlut_fl(mpc *mpc, bool enable, int mpcc_id)
{
	dcn42_mpc *mpc42 = TO_DCN42_MPC(mpc);

	//if enabled cho0se mpc 0, else: off (default value)
	reg_update!(MPC_RMCM_CNTL[mpcc_id], MPC_RMCM_CNTL, enable ? 0 : 0xF); //0xF is not connected

	reg_update!(MPC_RMCM_3DLUT_READ_WRITE_CONTROL[mpcc_id], MPC_RMCM_3DLUT_WRITE_EN_MASK, 0);

	reg_update!(MPC_RMCM_MEM_PWR_CTRL[mpcc_id], MPC_RMCM_3DLUT_MEM_PWR_DIS, enable ? 0 : 3);
}

unsafe fn mpc42_update_3dlut_fast_load_select(mpc *mpc, int mpcc_id, int hubp_idx)
{
	dcn42_mpc *mpc42 = TO_DCN42_MPC(mpc);

	reg_set!(MPC_RMCM_3DLUT_FAST_LOAD_SELECT[mpcc_id], 0,
		MPC_RMCM_3DLUT_FL_SEL,
		hubp_idx);
}

unsafe fn mpc42_populate_rmcm_lut(mpc *mpc, const mcm_lut_params params,
	bool lut_bank_a, int mpcc_id)
{
	const dc_lut_mode next_mode = lut_bank_a ? LUT_RAM_A : LUT_RAM_B;
	const pwl_params *lut_shaper = params.pwl;

	if (lut_shaper == core::ptr::null_mut())
		return;
	if (mpc.ctx.dc.debug.enable_mem_low_power.bits.mpc)
		mpc42_power_on_rmcm_shaper_3dlut(mpc, mpcc_id, true);

	mpc42_configure_rmcm_shaper_lut(mpc, next_mode == LUT_RAM_A, mpcc_id);

	if (next_mode == LUT_RAM_A)
		mpc42_program_rmcm_shaper_luta_settings(mpc, lut_shaper, mpcc_id);
	else
		mpc42_program_rmcm_shaper_lutb_settings(mpc, lut_shaper, mpcc_id);

	mpc42_program_rmcm_shaper_lut(
			mpc, lut_shaper.rgb_resulted, lut_shaper.hw_points_num, mpcc_id);

	mpc42_power_on_rmcm_shaper_3dlut(mpc, mpcc_id, false);
}

unsafe fn mpc42_program_rmcm_lut_read_write_control(mpc *mpc, const MCM_LUT_ID id,
	bool lut_bank_a, bool enabled, int mpcc_id)
{
	dcn42_mpc *mpc42 = TO_DCN42_MPC(mpc);

	switch (id) {
	case MCM_LUT_3DLUT:
		reg_update!(MPC_RMCM_3DLUT_MODE[mpcc_id], MPC_RMCM_3DLUT_MODE,
			(!enabled) ? 0 :
			(lut_bank_a) ? 1 : 2);

		reg_update!(MPC_RMCM_3DLUT_READ_WRITE_CONTROL[mpcc_id],
			MPC_RMCM_3DLUT_RAM_SEL,
			(lut_bank_a) ? 0 : 1);
		break;
	case MCM_LUT_SHAPER:
		reg_update!(MPC_RMCM_SHAPER_LUT_WRITE_EN_MASK[mpcc_id],
			MPC_RMCM_SHAPER_LUT_WRITE_EN_MASK, 7);

		reg_update!(MPC_RMCM_SHAPER_LUT_WRITE_EN_MASK[mpcc_id],
			MPC_RMCM_SHAPER_LUT_WRITE_SEL,
			lut_bank_a == true ? 0:1);

		reg_set!(MPC_RMCM_SHAPER_LUT_INDEX[mpcc_id], 0,
			MPC_RMCM_SHAPER_LUT_INDEX, 0);
		break;
	default:
		break;
	}
}

unsafe fn mpc42_program_lut_mode(mpc *mpc,
	bool enable,
	bool lut_bank_a,
	int mpcc_id)
{
	dcn42_mpc *mpc42 = TO_DCN42_MPC(mpc);

	if (enable) {
		reg_update!(MPC_RMCM_SHAPER_CONTROL[mpcc_id], MPC_RMCM_SHAPER_LUT_MODE, lut_bank_a ? 1 : 2);
	} else {
		reg_update!(MPC_RMCM_SHAPER_CONTROL[mpcc_id], MPC_RMCM_SHAPER_LUT_MODE, 0);
	}
}

static u32 mpc42_get_rmcm_3dlut_width(
		const dc_cm_lut_size size)
{
	u32 width = 0;

	switch (size) {
	case CM_LUT_SIZE_333333:
		width = 2;
		break;
	case CM_LUT_SIZE_171717:
	default:
		width = 0;
		break;
	}

	return width;
}

unsafe fn mpc42_program_rmcm_3dlut_size(mpc *mpc,
		const dc_cm_lut_size size,
		int mpcc_id)
{
	dcn42_mpc *mpc42 = TO_DCN42_MPC(mpc);
	u32 width = mpc42_get_rmcm_3dlut_width(size);

	reg_update!(MPC_RMCM_3DLUT_MODE[mpcc_id],
			MPC_RMCM_3DLUT_SIZE, width);
}

unsafe fn mpc42_program_rmcm_3dlut_fast_load_bias_scale(mpc *mpc, u16 bias, u16 scale, int mpcc_id)
{
	dcn42_mpc *mpc42 = TO_DCN42_MPC(mpc);

	reg_update_2!(MPC_RMCM_3DLUT_OUT_OFFSET_R[mpcc_id],
		MPC_RMCM_3DLUT_OUT_OFFSET_R, bias,
		MPC_RMCM_3DLUT_OUT_SCALE_R, scale);

	reg_update_2!(MPC_RMCM_3DLUT_OUT_OFFSET_G[mpcc_id],
		MPC_RMCM_3DLUT_OUT_OFFSET_G, bias,
		MPC_RMCM_3DLUT_OUT_SCALE_G, scale);

	reg_update_2!(MPC_RMCM_3DLUT_OUT_OFFSET_B[mpcc_id],
		MPC_RMCM_3DLUT_OUT_OFFSET_B, bias,
		MPC_RMCM_3DLUT_OUT_SCALE_B, scale);
}

unsafe fn mpc42_program_rmcm_bit_depth(mpc *mpc, u16 bit_depth, int mpcc_id)
{
	dcn42_mpc *mpc42 = TO_DCN42_MPC(mpc);

	reg_update!(MPC_RMCM_3DLUT_READ_WRITE_CONTROL[mpcc_id], MPC_RMCM_3DLUT_WRITE_EN_MASK, 0xF);

	//program bit_depth
	reg_update!(MPC_RMCM_3DLUT_READ_WRITE_CONTROL[mpcc_id],
		MPC_RMCM_3DLUT_30BIT_EN,
		(bit_depth == 10) ? 1 : 0);
}

unsafe fn mpc42_set_fl_config(
	mpc *mpc,
	mpc_fl_3dlut_config *cfg,
	int mpcc_id)
{
	dcn42_mpc *mpc42 = TO_DCN42_MPC(mpc);

	u32 width = mpc42_get_rmcm_3dlut_width(cfg.size);
	/*
	From: Jie Zhou

		To program any of the memories content.  The following sequence is used.
	Set the MPCC_OGAM/SHAPER/3DLUT/1DLUT_PWR_DIS to 1 (Only need to set the one
	that is being programmed) Set DISPCLK_G_PIPE<i>_GATE_DISABLE to 1 for the
	MPCC pipe that’s being used, so the memory’s clock is ungated. Program the
	target memory. Set the MPCC_OGAM/SHAPER/3DLUT/1DLUT_PWR_DIS back to 0.
	Set DISPCLK_G_PIPE<i>_GATE_DISABLE back to 0
	*/

	//disconnect fl from mpc
	reg_set!(MPCC_MCM_3DLUT_FAST_LOAD_SELECT[mpcc_id], 0,
		MPCC_MCM_3DLUT_FL_SEL, 0xF);

	reg_update!(MPC_RMCM_3DLUT_READ_WRITE_CONTROL[mpcc_id],
		MPC_RMCM_3DLUT_WRITE_EN_MASK, 0xF);

	//program bit_depth
	reg_update!(MPC_RMCM_3DLUT_READ_WRITE_CONTROL[mpcc_id],
		MPC_RMCM_3DLUT_30BIT_EN, (cfg.bit_depth == 10) ? 1 : 0);

	reg_update!(MPC_RMCM_3DLUT_READ_WRITE_CONTROL[mpcc_id],
		MPC_RMCM_3DLUT_RAM_SEL, (cfg.select_lut_bank_a) ? 0 : 1);

	//bias and scale
	reg_update_2!(MPC_RMCM_3DLUT_OUT_OFFSET_R[mpcc_id],
		MPC_RMCM_3DLUT_OUT_OFFSET_R, cfg.bias,
		MPC_RMCM_3DLUT_OUT_SCALE_R, cfg.scale);

	reg_update_2!(MPC_RMCM_3DLUT_OUT_OFFSET_G[mpcc_id],
		MPC_RMCM_3DLUT_OUT_OFFSET_G, cfg.bias,
		MPC_RMCM_3DLUT_OUT_SCALE_G, cfg.scale);

	reg_update_2!(MPC_RMCM_3DLUT_OUT_OFFSET_B[mpcc_id],
		MPC_RMCM_3DLUT_OUT_OFFSET_B, cfg.bias,
		MPC_RMCM_3DLUT_OUT_SCALE_B, cfg.scale);

	//width
	reg_update_2!(MPC_RMCM_3DLUT_MODE[mpcc_id],
		MPC_RMCM_3DLUT_SIZE, width,
		MPC_RMCM_3DLUT_MODE, (!cfg.enabled) ? 0 : (cfg.select_lut_bank_a) ? 1 : 2);

	//connect to hubp
	reg_set!(MPC_RMCM_3DLUT_FAST_LOAD_SELECT[mpcc_id], 0,
		MPC_RMCM_3DLUT_FL_SEL, cfg.hubp_index);

	//ENABLE
	//if enabled pick mpc 0, else: off (0xF)
	//in future we'll select specific MPC
	reg_update!(MPC_RMCM_CNTL[mpcc_id], MPC_RMCM_CNTL, cfg.enabled ? 0 : 0xF);
}

unsafe fn mpc42_get_rmcm_3dlut_mode(
	mpc *mpc,
	int mpcc_id,
	bool *enable,
	bool *lut_bank_a)
{
	dcn42_mpc *mpc42 = TO_DCN42_MPC(mpc);
	u32 mode_current   = 0;

	reg_get!(MPC_RMCM_3DLUT_MODE[mpcc_id], MPC_RMCM_3DLUT_MODE_CURRENT, &mode_current);

	/* MPC_RMCM_3DLUT_MODE encoding:
	 *   0 . disabled, 1 . bank A, 2 . bank B
	 */
	*enable     = mode_current != 0;
	*lut_bank_a = mode_current != 2;
}

unsafe fn mpc42_read_mpcc_state(
		mpc *mpc,
		int mpcc_inst,
		mpcc_state *s)
{
	dcn42_mpc *mpc42 = TO_DCN42_MPC(mpc);

	mpc1_read_mpcc_state(mpc, mpcc_inst, s);

	if (mpcc_inst < 2) {
		/* RMCM 3DLUT Status */
		reg_get_4!(MPC_RMCM_MEM_PWR_CTRL[mpcc_inst], MPC_RMCM_3DLUT_MEM_PWR_FORCE, &s.rmcm_regs.rmcm_3dlut_mem_pwr_force,
				MPC_RMCM_3DLUT_MEM_PWR_DIS, &s.rmcm_regs.rmcm_3dlut_mem_pwr_dis,
				MPC_RMCM_3DLUT_MEM_LOW_PWR_MODE, &s.rmcm_regs.rmcm_3dlut_mem_pwr_mode,
				MPC_RMCM_3DLUT_MEM_PWR_STATE, &s.rmcm_regs.rmcm_3dlut_mem_pwr_state);

		reg_get_3!(MPC_RMCM_3DLUT_MODE[mpcc_inst], MPC_RMCM_3DLUT_SIZE, &s.rmcm_regs.rmcm_3dlut_size,
				MPC_RMCM_3DLUT_MODE, &s.rmcm_regs.rmcm_3dlut_mode,
				MPC_RMCM_3DLUT_MODE_CURRENT, &s.rmcm_regs.rmcm_3dlut_mode_cur);

		reg_get_4!(MPC_RMCM_3DLUT_READ_WRITE_CONTROL[mpcc_inst], MPC_RMCM_3DLUT_READ_SEL, &s.rmcm_regs.rmcm_3dlut_read_sel,
				MPC_RMCM_3DLUT_30BIT_EN, &s.rmcm_regs.rmcm_3dlut_30bit_en,
				MPC_RMCM_3DLUT_WRITE_EN_MASK, &s.rmcm_regs.rmcm_3dlut_wr_en_mask,
				MPC_RMCM_3DLUT_RAM_SEL, &s.rmcm_regs.rmcm_3dlut_ram_sel);

		reg_get!(MPC_RMCM_3DLUT_OUT_NORM_FACTOR[mpcc_inst], MPC_RMCM_3DLUT_OUT_NORM_FACTOR, &s.rmcm_regs.rmcm_3dlut_out_norm_factor);

		reg_get!(MPC_RMCM_3DLUT_FAST_LOAD_SELECT[mpcc_inst], MPC_RMCM_3DLUT_FL_SEL, &s.rmcm_regs.rmcm_3dlut_fl_sel);

		reg_get_2!(MPC_RMCM_3DLUT_OUT_OFFSET_R[mpcc_inst], MPC_RMCM_3DLUT_OUT_OFFSET_R, &s.rmcm_regs.rmcm_3dlut_out_offset_r,
				MPC_RMCM_3DLUT_OUT_SCALE_R, &s.rmcm_regs.rmcm_3dlut_out_scale_r);

		reg_get_3!(MPC_RMCM_3DLUT_FAST_LOAD_STATUS[mpcc_inst], MPC_RMCM_3DLUT_FL_DONE, &s.rmcm_regs.rmcm_3dlut_fl_done,
				MPC_RMCM_3DLUT_FL_SOFT_UNDERFLOW, &s.rmcm_regs.rmcm_3dlut_fl_soft_underflow,
				MPC_RMCM_3DLUT_FL_HARD_UNDERFLOW, &s.rmcm_regs.rmcm_3dlut_fl_hard_underflow);

		/* RMCM Shaper Status */
		reg_get_4!(MPC_RMCM_MEM_PWR_CTRL[mpcc_inst], MPC_RMCM_SHAPER_MEM_PWR_FORCE, &s.rmcm_regs.rmcm_shaper_mem_pwr_force,
				MPC_RMCM_SHAPER_MEM_PWR_DIS, &s.rmcm_regs.rmcm_shaper_mem_pwr_dis,
				MPC_RMCM_SHAPER_MEM_LOW_PWR_MODE, &s.rmcm_regs.rmcm_shaper_mem_pwr_mode,
				MPC_RMCM_SHAPER_MEM_PWR_STATE, &s.rmcm_regs.rmcm_shaper_mem_pwr_state);

		reg_get_2!(MPC_RMCM_SHAPER_CONTROL[mpcc_inst], MPC_RMCM_SHAPER_LUT_MODE, &s.rmcm_regs.rmcm_shaper_lut_mode,
				MPC_RMCM_SHAPER_MODE_CURRENT, &s.rmcm_regs.rmcm_shaper_mode_cur);

		reg_get_2!(MPC_RMCM_SHAPER_LUT_WRITE_EN_MASK[mpcc_inst], MPC_RMCM_SHAPER_LUT_WRITE_EN_MASK, &s.rmcm_regs.rmcm_shaper_lut_write_en_mask,
				MPC_RMCM_SHAPER_LUT_WRITE_SEL, &s.rmcm_regs.rmcm_shaper_lut_write_sel);

		reg_get!(MPC_RMCM_SHAPER_OFFSET_B[mpcc_inst], MPC_RMCM_SHAPER_OFFSET_B, &s.rmcm_regs.rmcm_shaper_offset_b);

		reg_get!(MPC_RMCM_SHAPER_SCALE_G_B[mpcc_inst], MPC_RMCM_SHAPER_SCALE_B, &s.rmcm_regs.rmcm_shaper_scale_b);

		reg_get_2!(MPC_RMCM_SHAPER_RAMA_START_CNTL_B[mpcc_inst], MPC_RMCM_SHAPER_RAMA_EXP_REGION_START_B, &s.rmcm_regs.rmcm_shaper_rama_exp_region_start_b,
				MPC_RMCM_SHAPER_RAMA_EXP_REGION_START_SEGMENT_B, &s.rmcm_regs.rmcm_shaper_rama_exp_region_start_seg_b);

		reg_get_2!(MPC_RMCM_SHAPER_RAMA_END_CNTL_B[mpcc_inst], MPC_RMCM_SHAPER_RAMA_EXP_REGION_END_B, &s.rmcm_regs.rmcm_shaper_rama_exp_region_end_b,
				MPC_RMCM_SHAPER_RAMA_EXP_REGION_END_BASE_B, &s.rmcm_regs.rmcm_shaper_rama_exp_region_end_base_b);

		reg_get!(MPC_RMCM_CNTL[mpcc_inst], MPC_RMCM_CNTL, &s.rmcm_regs.rmcm_cntl);
	}
}

static const mpc_funcs dcn42_mpc_funcs = {
	.read_mpcc_state = mpc42_read_mpcc_state,
	.insert_plane = mpc1_insert_plane,
	.remove_mpcc = mpc1_remove_mpcc,
	.mpc_init = mpc32_mpc_init,
	.mpc_init_single_inst = mpc3_mpc_init_single_inst,
	.update_blending = mpc42_update_blending,
	.cursor_lock = mpc1_cursor_lock,
	.get_mpcc_for_dpp = mpc1_get_mpcc_for_dpp,
	.wait_for_idle = mpc2_assert_idle_mpcc,
	.assert_mpcc_idle_before_connect = mpc2_assert_mpcc_idle_before_connect,
	.init_mpcc_list_from_hw = mpc1_init_mpcc_list_from_hw,
	.set_denorm =  mpc3_set_denorm,
	.set_denorm_clamp = mpc3_set_denorm_clamp,
	.set_output_csc = mpc3_set_output_csc,
	.set_ocsc_default = mpc3_set_ocsc_default,
	.set_output_gamma = mpc3_set_output_gamma,
	.set_dwb_mux = mpc3_set_dwb_mux,
	.disable_dwb_mux = mpc3_disable_dwb_mux,
	.is_dwb_idle = mpc3_is_dwb_idle,
	.set_gamut_remap = mpc401_set_gamut_remap,
	.program_shaper = mpc32_program_shaper,
	.program_3dlut = mpc32_program_3dlut,
	.program_1dlut = mpc32_program_post1dlut,
	.power_on_mpc_mem_pwr = mpc3_power_on_ogam_lut,
	.get_mpc_out_mux = mpc1_get_mpc_out_mux,
	.mpc_read_reg_state = mpc3_read_reg_state,
	.set_bg_color = mpc1_set_bg_color,
	.set_movable_cm_location = mpc401_set_movable_cm_location,
	.update_3dlut_fast_load_select = mpc401_update_3dlut_fast_load_select,
	.get_3dlut_fast_load_status = mpc401_get_3dlut_fast_load_status,
	.populate_lut = mpc401_populate_lut,
	.program_lut_read_write_control = mpc401_program_lut_read_write_control,
	.program_lut_mode = mpc401_program_lut_mode,
	.get_lut_mode = mpc401_get_lut_mode,
	.rmcm = {
		.enable_3dlut_fl = mpc42_enable_3dlut_fl,
		.update_3dlut_fast_load_select = mpc42_update_3dlut_fast_load_select,
		.program_lut_read_write_control = mpc42_program_rmcm_lut_read_write_control,
		.program_lut_mode = mpc42_program_lut_mode,
		.program_3dlut_size = mpc42_program_rmcm_3dlut_size,
		.program_bias_scale = mpc42_program_rmcm_3dlut_fast_load_bias_scale,
		.program_bit_depth = mpc42_program_rmcm_bit_depth,
		.power_on_shaper_3dlut = mpc42_power_on_rmcm_shaper_3dlut,
		.populate_lut = mpc42_populate_rmcm_lut,
		.fl_3dlut_configure = mpc42_set_fl_config,
		.get_3dlut_mode = mpc42_get_rmcm_3dlut_mode,
	},
};

unsafe fn dcn42_mpc_construct(dcn42_mpc *mpc42,
	dc_context *ctx,
	const dcn42_mpc_registers *mpc_regs,
	const dcn42_mpc_shift *mpc_shift,
	const dcn42_mpc_mask *mpc_mask,
	int num_mpcc,
	int num_rmu)
{
	int i;

	mpc42.base.ctx = ctx;

	mpc42.base.funcs = &dcn42_mpc_funcs;

	mpc42.mpc_regs = mpc_regs;
	mpc42.mpc_shift = mpc_shift;
	mpc42.mpc_mask = mpc_mask;

	mpc42.mpcc_in_use_mask = 0;
	mpc42.num_mpcc = num_mpcc;
	mpc42.num_rmu = num_rmu;

	for (i = 0; i < MAX_MPCC; i++)
		mpc42_init_mpcc(&mpc42.base.mpcc_array[i], i);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
