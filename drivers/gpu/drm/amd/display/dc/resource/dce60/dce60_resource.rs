/* Faithful low-level translation; external register/type macros are supplied by dependent modules. */
/*
 * Copyright 2020 Mauro Rossi <issor.oruam@gmail.com>
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
 *
 */







/* TODO remove this include */





	#define mmBIOS_SCRATCH_0 0x05C9
	#define mmBIOS_SCRATCH_2 0x05CB
	#define mmBIOS_SCRATCH_3 0x05CC
	#define mmBIOS_SCRATCH_6 0x05CF

	#define mmDP_DPHY_FAST_TRAINING                         0x1CCE
	#define mmDP0_DP_DPHY_FAST_TRAINING                     0x1CCE
	#define mmDP1_DP_DPHY_FAST_TRAINING                     0x1FCE
	#define mmDP2_DP_DPHY_FAST_TRAINING                     0x42CE
	#define mmDP3_DP_DPHY_FAST_TRAINING                     0x45CE
	#define mmDP4_DP_DPHY_FAST_TRAINING                     0x48CE
	#define mmDP5_DP_DPHY_FAST_TRAINING                     0x4BCE


	#define mmHPD_DC_HPD_CONTROL                            0x189A
	#define mmHPD0_DC_HPD_CONTROL                           0x189A
	#define mmHPD1_DC_HPD_CONTROL                           0x18A2
	#define mmHPD2_DC_HPD_CONTROL                           0x18AA
	#define mmHPD3_DC_HPD_CONTROL                           0x18B2
	#define mmHPD4_DC_HPD_CONTROL                           0x18BA
	#define mmHPD5_DC_HPD_CONTROL                           0x18C2


static dce110_timing_generator_offsets dce60_tg_offsets[] = {
		{
			.crtc = (mmCRTC0_CRTC_CONTROL - mmCRTC_CONTROL),
			.dcp =  (mmGRPH_CONTROL - mmGRPH_CONTROL),
			.dmif = (mmDMIF_PG0_DPG_PIPE_ARBITRATION_CONTROL3
					- mmDPG_PIPE_ARBITRATION_CONTROL3),
		},
		{
			.crtc = (mmCRTC1_CRTC_CONTROL - mmCRTC_CONTROL),
			.dcp = (mmDCP1_GRPH_CONTROL - mmGRPH_CONTROL),
			.dmif = (mmDMIF_PG1_DPG_PIPE_ARBITRATION_CONTROL3
					- mmDPG_PIPE_ARBITRATION_CONTROL3),
		},
		{
			.crtc = (mmCRTC2_CRTC_CONTROL - mmCRTC_CONTROL),
			.dcp = (mmDCP2_GRPH_CONTROL - mmGRPH_CONTROL),
			.dmif = (mmDMIF_PG2_DPG_PIPE_ARBITRATION_CONTROL3
					- mmDPG_PIPE_ARBITRATION_CONTROL3),
		},
		{
			.crtc = (mmCRTC3_CRTC_CONTROL - mmCRTC_CONTROL),
			.dcp = (mmDCP3_GRPH_CONTROL - mmGRPH_CONTROL),
			.dmif = (mmDMIF_PG3_DPG_PIPE_ARBITRATION_CONTROL3
					- mmDPG_PIPE_ARBITRATION_CONTROL3),
		},
		{
			.crtc = (mmCRTC4_CRTC_CONTROL - mmCRTC_CONTROL),
			.dcp = (mmDCP4_GRPH_CONTROL - mmGRPH_CONTROL),
			.dmif = (mmDMIF_PG4_DPG_PIPE_ARBITRATION_CONTROL3
					- mmDPG_PIPE_ARBITRATION_CONTROL3),
		},
		{
			.crtc = (mmCRTC5_CRTC_CONTROL - mmCRTC_CONTROL),
			.dcp = (mmDCP5_GRPH_CONTROL - mmGRPH_CONTROL),
			.dmif = (mmDMIF_PG5_DPG_PIPE_ARBITRATION_CONTROL3
					- mmDPG_PIPE_ARBITRATION_CONTROL3),
		}
};

/* set register offset */
	.reg_name = mm ## reg_name

/* set register offset with instance */
	.reg_name = mm ## block ## id ## _ ## reg_name

[id] = {\
		IPP_COMMON_REG_LIST_DCE_BASE(id)\
}

static dce_ipp_registers ipp_regs[] = {
		ipp_regs(0),
		ipp_regs(1),
		ipp_regs(2),
		ipp_regs(3),
		ipp_regs(4),
		ipp_regs(5)
};

static dce_ipp_shift ipp_shift = {
		IPP_DCE60_MASK_SH_LIST_DCE_COMMON_BASE(__SHIFT)
};

static dce_ipp_mask ipp_mask = {
		IPP_DCE60_MASK_SH_LIST_DCE_COMMON_BASE(_MASK)
};

[id] = {\
		XFM_COMMON_REG_LIST_DCE60(id)\
}

static dce_transform_registers xfm_regs[] = {
		transform_regs(0),
		transform_regs(1),
		transform_regs(2),
		transform_regs(3),
		transform_regs(4),
		transform_regs(5)
};

static dce_transform_shift xfm_shift = {
		XFM_COMMON_MASK_SH_LIST_DCE60(__SHIFT)
};

static dce_transform_mask xfm_mask = {
		XFM_COMMON_MASK_SH_LIST_DCE60(_MASK)
};

[id] = {\
	AUX_REG_LIST(id)\
}

static dce110_link_enc_aux_registers link_enc_aux_regs[] = {
	aux_regs(0),
	aux_regs(1),
	aux_regs(2),
	aux_regs(3),
	aux_regs(4),
	aux_regs(5)
};

[id] = {\
	HPD_REG_LIST(id)\
}

static dce110_link_enc_hpd_registers link_enc_hpd_regs[] = {
		hpd_regs(0),
		hpd_regs(1),
		hpd_regs(2),
		hpd_regs(3),
		hpd_regs(4),
		hpd_regs(5)
};

[id] = {\
	LE_DCE60_REG_LIST(id)\
}

static dce110_link_enc_registers link_enc_regs[] = {
	link_regs(0),
	link_regs(1),
	link_regs(2),
	link_regs(3),
	link_regs(4),
	link_regs(5),
	{0},
	{0}
};

[id] = {\
	SE_COMMON_REG_LIST_DCE_BASE(id),\
	.AFMT_CNTL = 0,\
}

static dce110_stream_enc_registers stream_enc_regs[] = {
	stream_enc_regs(0),
	stream_enc_regs(1),
	stream_enc_regs(2),
	stream_enc_regs(3),
	stream_enc_regs(4),
	stream_enc_regs(5),
	{0},
	{SR(DAC_SOURCE_SELECT),} /* DACA */
};

static dce_stream_encoder_shift se_shift = {
		SE_COMMON_MASK_SH_LIST_DCE80_100(__SHIFT)
};

static dce_stream_encoder_mask se_mask = {
		SE_COMMON_MASK_SH_LIST_DCE80_100(_MASK)
};

static dce_panel_cntl_registers panel_cntl_regs[] = {
	{ DCE_PANEL_CNTL_REG_LIST() }
};

static dce_panel_cntl_shift panel_cntl_shift = {
	DCE_PANEL_CNTL_MASK_SH_LIST(__SHIFT)
};

static dce_panel_cntl_mask panel_cntl_mask = {
	DCE_PANEL_CNTL_MASK_SH_LIST(_MASK)
};

[id] = {\
	OPP_DCE_60_REG_LIST(id),\
}

static dce_opp_registers opp_regs[] = {
	opp_regs(0),
	opp_regs(1),
	opp_regs(2),
	opp_regs(3),
	opp_regs(4),
	opp_regs(5)
};

static dce_opp_shift opp_shift = {
	OPP_COMMON_MASK_SH_LIST_DCE_60(__SHIFT)
};

static dce_opp_mask opp_mask = {
	OPP_COMMON_MASK_SH_LIST_DCE_60(_MASK)
};

static dce110_aux_registers_shift aux_shift = {
	DCE10_AUX_MASK_SH_LIST(__SHIFT)
};

static dce110_aux_registers_mask aux_mask = {
	DCE10_AUX_MASK_SH_LIST(_MASK)
};

[id] = {\
	AUX_COMMON_REG_LIST(id), \
	.AUX_RESET_MASK = 0 \
}

static dce110_aux_registers aux_engine_regs[] = {
		aux_engine_regs(0),
		aux_engine_regs(1),
		aux_engine_regs(2),
		aux_engine_regs(3),
		aux_engine_regs(4),
		aux_engine_regs(5)
};

[id] = {\
	AUD_COMMON_REG_LIST(id)\
}

static dce_audio_registers audio_regs[] = {
	audio_regs(0),
	audio_regs(1),
	audio_regs(2),
	audio_regs(3),
	audio_regs(4),
	audio_regs(5),
};

static dce_audio_shift audio_shift = {
		AUD_DCE60_MASK_SH_LIST(__SHIFT)
};

static dce_audio_mask audio_mask = {
		AUD_DCE60_MASK_SH_LIST(_MASK)
};

[id] = {\
	CS_COMMON_REG_LIST_DCE_80(id),\
}


static dce110_clk_src_regs clk_src_regs[] = {
	clk_src_regs(0),
	clk_src_regs(1),
	clk_src_regs(2)
};

static dce110_clk_src_shift cs_shift = {
		CS_COMMON_MASK_SH_LIST_DCE_COMMON_BASE(__SHIFT)
};

static dce110_clk_src_mask cs_mask = {
		CS_COMMON_MASK_SH_LIST_DCE_COMMON_BASE(_MASK)
};

static bios_registers bios_regs = {
	.BIOS_SCRATCH_0 = mmBIOS_SCRATCH_0,
	.BIOS_SCRATCH_3 = mmBIOS_SCRATCH_3,
	.BIOS_SCRATCH_6 = mmBIOS_SCRATCH_6
};

static resource_caps res_cap = {
		.num_timing_generator = 6,
		.num_audio = 6,
		.num_analog_stream_encoder = 1,
		.num_stream_encoder = 6,
		.num_pll = 3,
		.num_ddc = 6,
};

static resource_caps res_cap_61 = {
		.num_timing_generator = 4,
		.num_audio = 6,
		.num_stream_encoder = 6,
		.num_analog_stream_encoder = 1,
		.num_pll = 3,
		.num_ddc = 6,
};

static resource_caps res_cap_64 = {
		.num_timing_generator = 2,
		.num_audio = 2,
		.num_analog_stream_encoder = 1,
		.num_stream_encoder = 2,
		.num_pll = 3,
		.num_ddc = 2,
};

static dc_plane_cap plane_cap = {
	.type = DC_PLANE_TYPE_DCE_RGB,

	.pixel_format_support = {
			.argb8888 = true,
			.nv12 = false,
			.fp16 = false
	},

	.max_upscale_factor = {
			.argb8888 = 1,
			.nv12 = 1,
			.fp16 = 1
	},

	.max_downscale_factor = {
			.argb8888 = 1,
			.nv12 = 1,
			.fp16 = 1
	}
};

static dce_dmcu_registers dmcu_regs = {
		DMCU_DCE60_REG_LIST()
};

static dce_dmcu_shift dmcu_shift = {
		DMCU_MASK_SH_LIST_DCE60(__SHIFT)
};

static dce_dmcu_mask dmcu_mask = {
		DMCU_MASK_SH_LIST_DCE60(_MASK)
};
static dce_abm_registers abm_regs = {
		ABM_DCE110_COMMON_REG_LIST()
};

static dce_abm_shift abm_shift = {
		ABM_MASK_SH_LIST_DCE110(__SHIFT)
};

static dce_abm_mask abm_mask = {
		ABM_MASK_SH_LIST_DCE110(_MASK)
};



static int map_transmitter_id_to_phy_instance(
	enum transmitter transmitter)
{
	switch (transmitter) {
	case TRANSMITTER_UNIPHY_A:
		return 0;
	case TRANSMITTER_UNIPHY_B:
		return 1;
	case TRANSMITTER_UNIPHY_C:
		return 2;
	case TRANSMITTER_UNIPHY_D:
		return 3;
	case TRANSMITTER_UNIPHY_E:
		return 4;
	case TRANSMITTER_UNIPHY_F:
		return 5;
	case TRANSMITTER_UNIPHY_G:
		return 6;
	default:
		ASSERT(0);
		return 0;
	}
}

static void read_dce_straps(
	struct dc_context *ctx,
	struct resource_straps *straps)
{
	REG_GET_2(CC_DC_HDMI_STRAPS,
			HDMI_DISABLE, &straps->hdmi_disable,
			AUDIO_STREAM_NUMBER, &straps->audio_stream_number);

	REG_GET(DC_PINSTRAPS, DC_PINSTRAPS_AUDIO, &straps->dc_pinstraps_audio);
}

static audio *create_audio(
		struct dc_context *ctx, unsigned int inst)
{
	return dce_audio_create(ctx, inst,
			&audio_regs[inst], &audio_shift, &audio_mask);
}

static timing_generator *dce60_timing_generator_create(
		struct dc_context *ctx,
		uint32_t instance,
		const struct dce110_timing_generator_offsets *offsets)
{
	struct dce110_timing_generator *tg110 =
		kzalloc_obj(struct dce110_timing_generator);

	if (!tg110)
		return core::ptr::null_mut();

	dce60_timing_generator_construct(tg110, ctx, instance, offsets);
	return &tg110->base;
}

static output_pixel_processor *dce60_opp_create(
	struct dc_context *ctx,
	uint32_t inst)
{
	struct dce110_opp *opp =
		kzalloc_obj(struct dce110_opp);

	if (!opp)
		return core::ptr::null_mut();

	dce60_opp_construct(opp,
			     ctx, inst, &opp_regs[inst], &opp_shift, &opp_mask);
	return &opp->base;
}

static dce_aux *dce60_aux_engine_create(
	struct dc_context *ctx,
	uint32_t inst)
{
	struct aux_engine_dce110 *aux_engine =
		kzalloc_obj(struct aux_engine_dce110);

	if (!aux_engine)
		return core::ptr::null_mut();

	dce110_aux_engine_construct(aux_engine, ctx, inst,
				    SW_AUX_TIMEOUT_PERIOD_MULTIPLIER * AUX_TIMEOUT_PERIOD,
				    &aux_engine_regs[inst],
					&aux_mask,
					&aux_shift,
					ctx->dc->caps.extended_aux_timeout_support);

	return &aux_engine->base;
}

static dce_i2c_registers i2c_hw_regs[] = {
		i2c_inst_regs(1),
		i2c_inst_regs(2),
		i2c_inst_regs(3),
		i2c_inst_regs(4),
		i2c_inst_regs(5),
		i2c_inst_regs(6),
};

static dce_i2c_shift i2c_shifts = {
		I2C_COMMON_MASK_SH_LIST_DCE_COMMON_BASE(__SHIFT)
};

static dce_i2c_mask i2c_masks = {
		I2C_COMMON_MASK_SH_LIST_DCE_COMMON_BASE(_MASK)
};

static dce_i2c_hw *dce60_i2c_hw_create(
	struct dc_context *ctx,
	uint32_t inst)
{
	struct dce_i2c_hw *dce_i2c_hw =
		kzalloc_obj(struct dce_i2c_hw);

	if (!dce_i2c_hw)
		return core::ptr::null_mut();

	dce_i2c_hw_construct(dce_i2c_hw, ctx, inst,
				    &i2c_hw_regs[inst], &i2c_shifts, &i2c_masks);

	return dce_i2c_hw;
}

static dce_i2c_sw *dce60_i2c_sw_create(
	struct dc_context *ctx)
{
	struct dce_i2c_sw *dce_i2c_sw =
		kzalloc_obj(struct dce_i2c_sw);

	if (!dce_i2c_sw)
		return core::ptr::null_mut();

	dce_i2c_sw_construct(dce_i2c_sw, ctx);

	return dce_i2c_sw;
}
static stream_encoder *dce60_stream_encoder_create(
	enum engine_id eng_id,
	struct dc_context *ctx)
{
	struct dce110_stream_encoder *enc110 =
		kzalloc_obj(struct dce110_stream_encoder);

	if (!enc110)
		return core::ptr::null_mut();

	if (eng_id == ENGINE_ID_DACA || eng_id == ENGINE_ID_DACB) {
		dce110_analog_stream_encoder_construct(enc110, ctx, ctx->dc_bios, eng_id,
			&stream_enc_regs[eng_id], &se_shift, &se_mask);
		return &enc110->base;
	}

	dce110_stream_encoder_construct(enc110, ctx, ctx->dc_bios, eng_id,
					&stream_enc_regs[eng_id],
					&se_shift, &se_mask);
	return &enc110->base;
}

	.reg_name[id] = mm ## block ## id ## _ ## reg_name

static dce_hwseq_registers hwseq_reg = {
		HWSEQ_DCE6_REG_LIST()
};

static dce_hwseq_shift hwseq_shift = {
		HWSEQ_DCE6_MASK_SH_LIST(__SHIFT)
};

static dce_hwseq_mask hwseq_mask = {
		HWSEQ_DCE6_MASK_SH_LIST(_MASK)
};

static dce_hwseq *dce60_hwseq_create(
	struct dc_context *ctx)
{
	struct dce_hwseq *hws = kzalloc_obj(struct dce_hwseq);

	if (hws) {
		hws->ctx = ctx;
		hws->regs = &hwseq_reg;
		hws->shifts = &hwseq_shift;
		hws->masks = &hwseq_mask;
	}
	return hws;
}

static resource_create_funcs res_create_funcs = {
	.read_dce_straps = read_dce_straps,
	.create_audio = create_audio,
	.create_stream_encoder = dce60_stream_encoder_create,
	.create_hwseq = dce60_hwseq_create,
};

	MI_DCE6_REG_LIST(id), \
	.MC_HUB_RDREQ_DMIF_LIMIT = mmMC_HUB_RDREQ_DMIF_LIMIT \
}
static dce_mem_input_registers mi_regs[] = {
		mi_inst_regs(0),
		mi_inst_regs(1),
		mi_inst_regs(2),
		mi_inst_regs(3),
		mi_inst_regs(4),
		mi_inst_regs(5),
};

static dce_mem_input_shift mi_shifts = {
		MI_DCE6_MASK_SH_LIST(__SHIFT),
		.ENABLE = MC_HUB_RDREQ_DMIF_LIMIT__ENABLE__SHIFT
};

static dce_mem_input_mask mi_masks = {
		MI_DCE6_MASK_SH_LIST(_MASK),
		.ENABLE = MC_HUB_RDREQ_DMIF_LIMIT__ENABLE_MASK
};

static mem_input *dce60_mem_input_create(
	struct dc_context *ctx,
	uint32_t inst)
{
	struct dce_mem_input *dce_mi = kzalloc_obj(struct dce_mem_input);

	if (!dce_mi) {
		BREAK_TO_DEBUGGER();
		return core::ptr::null_mut();
	}

	dce60_mem_input_construct(dce_mi, ctx, inst, &mi_regs[inst], &mi_shifts, &mi_masks);
	dce_mi->wa.single_head_rdreq_dmif_limit = 2;
	return &dce_mi->base;
}

static void dce60_transform_destroy(struct transform **xfm)
{
	kfree(TO_DCE_TRANSFORM(*xfm));
	*xfm = core::ptr::null_mut();
}

static transform *dce60_transform_create(
	struct dc_context *ctx,
	uint32_t inst)
{
	struct dce_transform *transform =
		kzalloc_obj(struct dce_transform);

	if (!transform)
		return core::ptr::null_mut();

	dce60_transform_construct(transform, ctx, inst,
				&xfm_regs[inst], &xfm_shift, &xfm_mask);
	transform->prescaler_on = false;
	return &transform->base;
}

static encoder_feature_support link_enc_feature = {
		.max_hdmi_deep_color = COLOR_DEPTH_121212,
		.max_hdmi_pixel_clock = 297000,
		.flags.bits.IS_HBR2_CAPABLE = true,
		.flags.bits.IS_TPS3_CAPABLE = true
};

static link_encoder *dce60_link_encoder_create(
	struct dc_context *ctx,
	const struct encoder_init_data *enc_init_data)
{
	struct dce110_link_encoder *enc110 =
		kzalloc_obj(struct dce110_link_encoder);
	int link_regs_id;

	if (!enc110)
		return core::ptr::null_mut();

	if (enc_init_data->connector.id == CONNECTOR_ID_VGA &&
	    enc_init_data->analog_engine != ENGINE_ID_UNKNOWN) {
		dce60_link_encoder_construct(enc110,
			enc_init_data,
			&link_enc_feature,
			&link_enc_regs[ENGINE_ID_DACA],
			core::ptr::null_mut(),
			core::ptr::null_mut());
		return &enc110->base;
	}

	link_regs_id =
		map_transmitter_id_to_phy_instance(enc_init_data->transmitter);

	dce60_link_encoder_construct(enc110,
				     enc_init_data,
				     &link_enc_feature,
				     &link_enc_regs[link_regs_id],
				     enc_init_data->channel == CHANNEL_ID_UNKNOWN ?
				     core::ptr::null_mut() : &link_enc_aux_regs[enc_init_data->channel - 1],
				     enc_init_data->hpd_source >= ARRAY_SIZE(link_enc_hpd_regs) ?
				     core::ptr::null_mut() : &link_enc_hpd_regs[enc_init_data->hpd_source]);
	return &enc110->base;
}

static panel_cntl *dce60_panel_cntl_create(const struct panel_cntl_init_data *init_data)
{
	struct dce_panel_cntl *panel_cntl =
		kzalloc_obj(struct dce_panel_cntl);

	if (!panel_cntl)
		return core::ptr::null_mut();

	dce_panel_cntl_construct(panel_cntl,
			init_data,
			&panel_cntl_regs[init_data->inst],
			&panel_cntl_shift,
			&panel_cntl_mask);

	return &panel_cntl->base;
}

static clock_source *dce60_clock_source_create(
	struct dc_context *ctx,
	struct dc_bios *bios,
	enum clock_source_id id,
	const struct dce110_clk_src_regs *regs,
	bool dp_clk_src)
{
	struct dce110_clk_src *clk_src =
		kzalloc_obj(struct dce110_clk_src);

	if (!clk_src)
		return core::ptr::null_mut();

	if (dce110_clk_src_construct(clk_src, ctx, bios, id,
			regs, &cs_shift, &cs_mask)) {
		clk_src->base.dp_clk_src = dp_clk_src;
		return &clk_src->base;
	}

	kfree(clk_src);
	BREAK_TO_DEBUGGER();
	return core::ptr::null_mut();
}

static void dce60_clock_source_destroy(struct clock_source **clk_src)
{
	kfree(TO_DCE110_CLK_SRC(*clk_src));
	*clk_src = core::ptr::null_mut();
}

static input_pixel_processor *dce60_ipp_create(
	struct dc_context *ctx, uint32_t inst)
{
	struct dce_ipp *ipp = kzalloc_obj(struct dce_ipp);

	if (!ipp) {
		BREAK_TO_DEBUGGER();
		return core::ptr::null_mut();
	}

	dce60_ipp_construct(ipp, ctx, inst,
			&ipp_regs[inst], &ipp_shift, &ipp_mask);
	return &ipp->base;
}

static void dce60_resource_destruct(struct dce110_resource_pool *pool)
{
	unsigned int i;

	for (i = 0; i < pool->base.pipe_count; i++) {
		if (pool->base.opps[i] != core::ptr::null_mut())
			dce110_opp_destroy(&pool->base.opps[i]);

		if (pool->base.transforms[i] != core::ptr::null_mut())
			dce60_transform_destroy(&pool->base.transforms[i]);

		if (pool->base.ipps[i] != core::ptr::null_mut())
			dce_ipp_destroy(&pool->base.ipps[i]);

		if (pool->base.mis[i] != core::ptr::null_mut()) {
			kfree(TO_DCE_MEM_INPUT(pool->base.mis[i]));
			pool->base.mis[i] = core::ptr::null_mut();
		}

		if (pool->base.timing_generators[i] != core::ptr::null_mut())	{
			kfree(DCE110TG_FROM_TG(pool->base.timing_generators[i]));
			pool->base.timing_generators[i] = core::ptr::null_mut();
		}
	}

	for (i = 0; i < pool->base.res_cap->num_ddc; i++) {
		if (pool->base.engines[i] != core::ptr::null_mut())
			dce110_engine_destroy(&pool->base.engines[i]);
		if (pool->base.hw_i2cs[i] != core::ptr::null_mut()) {
			kfree(pool->base.hw_i2cs[i]);
			pool->base.hw_i2cs[i] = core::ptr::null_mut();
		}
		if (pool->base.sw_i2cs[i] != core::ptr::null_mut()) {
			kfree(pool->base.sw_i2cs[i]);
			pool->base.sw_i2cs[i] = core::ptr::null_mut();
		}
	}

	for (i = 0; i < pool->base.stream_enc_count; i++) {
		if (pool->base.stream_enc[i] != core::ptr::null_mut())
			kfree(DCE110STRENC_FROM_STRENC(pool->base.stream_enc[i]));
	}

	for (i = 0; i < pool->base.clk_src_count; i++) {
		if (pool->base.clock_sources[i] != core::ptr::null_mut()) {
			dce60_clock_source_destroy(&pool->base.clock_sources[i]);
		}
	}

	if (pool->base.abm != core::ptr::null_mut())
			dce_abm_destroy(&pool->base.abm);

	if (pool->base.dmcu != core::ptr::null_mut())
			dce_dmcu_destroy(&pool->base.dmcu);

	if (pool->base.dp_clock_source != core::ptr::null_mut())
		dce60_clock_source_destroy(&pool->base.dp_clock_source);

	for (i = 0; i < pool->base.audio_count; i++)	{
		if (pool->base.audios[i] != core::ptr::null_mut()) {
			dce_aud_destroy(&pool->base.audios[i]);
		}
	}

	if (pool->base.irqs != core::ptr::null_mut()) {
		dal_irq_service_destroy(&pool->base.irqs);
	}
}

static void dce60_destroy_resource_pool(struct resource_pool **pool)
{
	struct dce110_resource_pool *dce110_pool = TO_DCE110_RES_POOL(*pool);

	dce60_resource_destruct(dce110_pool);
	kfree(dce110_pool);
	*pool = core::ptr::null_mut();
}

static resource_funcs dce60_res_pool_funcs = {
	.destroy = dce60_destroy_resource_pool,
	.link_enc_create = dce60_link_encoder_create,
	.panel_cntl_create = dce60_panel_cntl_create,
	.validate_bandwidth = dce100_validate_bandwidth,
	.validate_plane = dce100_validate_plane,
	.add_stream_to_ctx = dce100_add_stream_to_ctx,
	.validate_global = dce100_validate_global,
	.find_first_free_match_stream_enc_for_link = dce100_find_first_free_match_stream_enc_for_link
};

static bool dce60_construct(
	uint8_t num_virtual_links,
	struct dc *dc,
	struct dce110_resource_pool *pool)
{
	unsigned int i;
	struct dc_context *ctx = dc->ctx;
	struct dc_bios *bp;

	ctx->dc_bios->regs = &bios_regs;

	pool->base.res_cap = &res_cap;
	pool->base.funcs = &dce60_res_pool_funcs;


	/*************************************************
	 *  Resource + asic cap harcoding                *
	 *************************************************/
	pool->base.underlay_pipe_index = NO_UNDERLAY_PIPE;
	pool->base.pipe_count = res_cap.num_timing_generator;
	pool->base.timing_generator_count = res_cap.num_timing_generator;
	dc->caps.max_downscale_ratio = 200;
	dc->caps.i2c_speed_in_khz = 40;
	dc->caps.max_cursor_size = 64;
	dc->caps.dual_link_dvi = true;
	dc->caps.extended_aux_timeout_support = false;

	/*************************************************
	 *  Create resources                             *
	 *************************************************/

	bp = ctx->dc_bios;

	if (bp->fw_info_valid && bp->fw_info.external_clock_source_frequency_for_dp != 0) {
		pool->base.dp_clock_source =
			dce60_clock_source_create(ctx, bp, CLOCK_SOURCE_ID_EXTERNAL, core::ptr::null_mut(), true);

		/* DCE 6.0 and 6.4: PLL0 can only be used with DP. Don't initialize it here. */
		pool->base.clock_sources[0] =
			dce60_clock_source_create(ctx, bp, CLOCK_SOURCE_ID_PLL1, &clk_src_regs[1], false);
		pool->base.clock_sources[1] =
			dce60_clock_source_create(ctx, bp, CLOCK_SOURCE_ID_PLL2, &clk_src_regs[2], false);
		pool->base.clk_src_count = 2;

	} else {
		pool->base.dp_clock_source =
			dce60_clock_source_create(ctx, bp, CLOCK_SOURCE_ID_PLL0, &clk_src_regs[0], true);

		pool->base.clock_sources[0] =
			dce60_clock_source_create(ctx, bp, CLOCK_SOURCE_ID_PLL1, &clk_src_regs[1], false);
		pool->base.clock_sources[1] =
			dce60_clock_source_create(ctx, bp, CLOCK_SOURCE_ID_PLL2, &clk_src_regs[2], false);
		pool->base.clk_src_count = 2;
	}

	if (pool->base.dp_clock_source == core::ptr::null_mut()) {
		dm_error("DC: failed to create dp clock source!\n");
		BREAK_TO_DEBUGGER();
		goto res_create_fail;
	}

	for (i = 0; i < pool->base.clk_src_count; i++) {
		if (pool->base.clock_sources[i] == core::ptr::null_mut()) {
			dm_error("DC: failed to create clock sources!\n");
			BREAK_TO_DEBUGGER();
			goto res_create_fail;
		}
	}

	pool->base.dmcu = dce_dmcu_create(ctx,
			&dmcu_regs,
			&dmcu_shift,
			&dmcu_mask);
	if (pool->base.dmcu == core::ptr::null_mut()) {
		dm_error("DC: failed to create dmcu!\n");
		BREAK_TO_DEBUGGER();
		goto res_create_fail;
	}

	pool->base.abm = dce_abm_create(ctx,
			&abm_regs,
			&abm_shift,
			&abm_mask);
	if (pool->base.abm == core::ptr::null_mut()) {
		dm_error("DC: failed to create abm!\n");
		BREAK_TO_DEBUGGER();
		goto res_create_fail;
	}

	{
		struct irq_service_init_data init_data;
		init_data.ctx = dc->ctx;
		pool->base.irqs = dal_irq_service_dce60_create(&init_data);
		if (!pool->base.irqs)
			goto res_create_fail;
	}

	for (i = 0; i < pool->base.pipe_count; i++) {
		pool->base.timing_generators[i] = dce60_timing_generator_create(
				ctx, i, &dce60_tg_offsets[i]);
		if (pool->base.timing_generators[i] == core::ptr::null_mut()) {
			BREAK_TO_DEBUGGER();
			dm_error("DC: failed to create tg!\n");
			goto res_create_fail;
		}

		pool->base.mis[i] = dce60_mem_input_create(ctx, i);
		if (pool->base.mis[i] == core::ptr::null_mut()) {
			BREAK_TO_DEBUGGER();
			dm_error("DC: failed to create memory input!\n");
			goto res_create_fail;
		}

		pool->base.ipps[i] = dce60_ipp_create(ctx, i);
		if (pool->base.ipps[i] == core::ptr::null_mut()) {
			BREAK_TO_DEBUGGER();
			dm_error("DC: failed to create input pixel processor!\n");
			goto res_create_fail;
		}

		pool->base.transforms[i] = dce60_transform_create(ctx, i);
		if (pool->base.transforms[i] == core::ptr::null_mut()) {
			BREAK_TO_DEBUGGER();
			dm_error("DC: failed to create transform!\n");
			goto res_create_fail;
		}

		pool->base.opps[i] = dce60_opp_create(ctx, i);
		if (pool->base.opps[i] == core::ptr::null_mut()) {
			BREAK_TO_DEBUGGER();
			dm_error("DC: failed to create output pixel processor!\n");
			goto res_create_fail;
		}
	}

	for (i = 0; i < pool->base.res_cap->num_ddc; i++) {
		pool->base.engines[i] = dce60_aux_engine_create(ctx, i);
		if (pool->base.engines[i] == core::ptr::null_mut()) {
			BREAK_TO_DEBUGGER();
			dm_error(
				"DC:failed to create aux engine!!\n");
			goto res_create_fail;
		}
		pool->base.hw_i2cs[i] = dce60_i2c_hw_create(ctx, i);
		if (pool->base.hw_i2cs[i] == core::ptr::null_mut()) {
			BREAK_TO_DEBUGGER();
			dm_error(
				"DC:failed to create i2c engine!!\n");
			goto res_create_fail;
		}
		pool->base.sw_i2cs[i] = dce60_i2c_sw_create(ctx);
		if (pool->base.sw_i2cs[i] == core::ptr::null_mut()) {
			BREAK_TO_DEBUGGER();
			dm_error(
				"DC:failed to create sw i2c!!\n");
			goto res_create_fail;
		}
	}

	dc->caps.max_planes =  pool->base.pipe_count;

	for (i = 0; i < dc->caps.max_planes; ++i)
		dc->caps.planes[i] = plane_cap;

	dc->caps.disable_dp_clk_share = true;

	if (!resource_construct(num_virtual_links, dc, &pool->base,
			&res_create_funcs))
		goto res_create_fail;

	/* Create hardware sequencer */
	dce60_hw_sequencer_construct(dc);

	return true;

res_create_fail:
	dce60_resource_destruct(pool);
	return false;
}

struct resource_pool *dce60_create_resource_pool(
	uint8_t num_virtual_links,
	struct dc *dc)
{
	struct dce110_resource_pool *pool =
		kzalloc_obj(struct dce110_resource_pool);

	if (!pool)
		return core::ptr::null_mut();

	if (dce60_construct(num_virtual_links, dc, pool))
		return &pool->base;

	kfree(pool);
	BREAK_TO_DEBUGGER();
	return core::ptr::null_mut();
}

static bool dce61_construct(
	uint8_t num_virtual_links,
	struct dc *dc,
	struct dce110_resource_pool *pool)
{
	unsigned int i;
	struct dc_context *ctx = dc->ctx;
	struct dc_bios *bp;

	ctx->dc_bios->regs = &bios_regs;

	pool->base.res_cap = &res_cap_61;
	pool->base.funcs = &dce60_res_pool_funcs;


	/*************************************************
	 *  Resource + asic cap harcoding                *
	 *************************************************/
	pool->base.underlay_pipe_index = NO_UNDERLAY_PIPE;
	pool->base.pipe_count = res_cap_61.num_timing_generator;
	pool->base.timing_generator_count = res_cap_61.num_timing_generator;
	dc->caps.max_downscale_ratio = 200;
	dc->caps.i2c_speed_in_khz = 40;
	dc->caps.max_cursor_size = 64;
	dc->caps.is_apu = true;

	/*************************************************
	 *  Create resources                             *
	 *************************************************/

	bp = ctx->dc_bios;

	if (bp->fw_info_valid && bp->fw_info.external_clock_source_frequency_for_dp != 0) {
		pool->base.dp_clock_source =
				dce60_clock_source_create(ctx, bp, CLOCK_SOURCE_ID_EXTERNAL, core::ptr::null_mut(), true);

		pool->base.clock_sources[0] =
				dce60_clock_source_create(ctx, bp, CLOCK_SOURCE_ID_PLL0, &clk_src_regs[0], false);
		pool->base.clock_sources[1] =
				dce60_clock_source_create(ctx, bp, CLOCK_SOURCE_ID_PLL1, &clk_src_regs[1], false);
		pool->base.clock_sources[2] =
				dce60_clock_source_create(ctx, bp, CLOCK_SOURCE_ID_PLL2, &clk_src_regs[2], false);
		pool->base.clk_src_count = 3;

	} else {
		pool->base.dp_clock_source =
				dce60_clock_source_create(ctx, bp, CLOCK_SOURCE_ID_PLL0, &clk_src_regs[0], true);

		pool->base.clock_sources[0] =
				dce60_clock_source_create(ctx, bp, CLOCK_SOURCE_ID_PLL1, &clk_src_regs[1], false);
		pool->base.clock_sources[1] =
				dce60_clock_source_create(ctx, bp, CLOCK_SOURCE_ID_PLL2, &clk_src_regs[2], false);
		pool->base.clk_src_count = 2;
	}

	if (pool->base.dp_clock_source == core::ptr::null_mut()) {
		dm_error("DC: failed to create dp clock source!\n");
		BREAK_TO_DEBUGGER();
		goto res_create_fail;
	}

	for (i = 0; i < pool->base.clk_src_count; i++) {
		if (pool->base.clock_sources[i] == core::ptr::null_mut()) {
			dm_error("DC: failed to create clock sources!\n");
			BREAK_TO_DEBUGGER();
			goto res_create_fail;
		}
	}

	pool->base.dmcu = dce_dmcu_create(ctx,
			&dmcu_regs,
			&dmcu_shift,
			&dmcu_mask);
	if (pool->base.dmcu == core::ptr::null_mut()) {
		dm_error("DC: failed to create dmcu!\n");
		BREAK_TO_DEBUGGER();
		goto res_create_fail;
	}

	pool->base.abm = dce_abm_create(ctx,
			&abm_regs,
			&abm_shift,
			&abm_mask);
	if (pool->base.abm == core::ptr::null_mut()) {
		dm_error("DC: failed to create abm!\n");
		BREAK_TO_DEBUGGER();
		goto res_create_fail;
	}

	{
		struct irq_service_init_data init_data;
		init_data.ctx = dc->ctx;
		pool->base.irqs = dal_irq_service_dce60_create(&init_data);
		if (!pool->base.irqs)
			goto res_create_fail;
	}

	for (i = 0; i < pool->base.pipe_count; i++) {
		pool->base.timing_generators[i] = dce60_timing_generator_create(
				ctx, i, &dce60_tg_offsets[i]);
		if (pool->base.timing_generators[i] == core::ptr::null_mut()) {
			BREAK_TO_DEBUGGER();
			dm_error("DC: failed to create tg!\n");
			goto res_create_fail;
		}

		pool->base.mis[i] = dce60_mem_input_create(ctx, i);
		if (pool->base.mis[i] == core::ptr::null_mut()) {
			BREAK_TO_DEBUGGER();
			dm_error("DC: failed to create memory input!\n");
			goto res_create_fail;
		}

		pool->base.ipps[i] = dce60_ipp_create(ctx, i);
		if (pool->base.ipps[i] == core::ptr::null_mut()) {
			BREAK_TO_DEBUGGER();
			dm_error("DC: failed to create input pixel processor!\n");
			goto res_create_fail;
		}

		pool->base.transforms[i] = dce60_transform_create(ctx, i);
		if (pool->base.transforms[i] == core::ptr::null_mut()) {
			BREAK_TO_DEBUGGER();
			dm_error("DC: failed to create transform!\n");
			goto res_create_fail;
		}

		pool->base.opps[i] = dce60_opp_create(ctx, i);
		if (pool->base.opps[i] == core::ptr::null_mut()) {
			BREAK_TO_DEBUGGER();
			dm_error("DC: failed to create output pixel processor!\n");
			goto res_create_fail;
		}
	}

	for (i = 0; i < pool->base.res_cap->num_ddc; i++) {
		pool->base.engines[i] = dce60_aux_engine_create(ctx, i);
		if (pool->base.engines[i] == core::ptr::null_mut()) {
			BREAK_TO_DEBUGGER();
			dm_error(
				"DC:failed to create aux engine!!\n");
			goto res_create_fail;
		}
		pool->base.hw_i2cs[i] = dce60_i2c_hw_create(ctx, i);
		if (pool->base.hw_i2cs[i] == core::ptr::null_mut()) {
			BREAK_TO_DEBUGGER();
			dm_error(
				"DC:failed to create i2c engine!!\n");
			goto res_create_fail;
		}
		pool->base.sw_i2cs[i] = dce60_i2c_sw_create(ctx);
		if (pool->base.sw_i2cs[i] == core::ptr::null_mut()) {
			BREAK_TO_DEBUGGER();
			dm_error(
				"DC:failed to create sw i2c!!\n");
			goto res_create_fail;
		}
	}

	dc->caps.max_planes =  pool->base.pipe_count;

	for (i = 0; i < dc->caps.max_planes; ++i)
		dc->caps.planes[i] = plane_cap;

	dc->caps.disable_dp_clk_share = true;

	if (!resource_construct(num_virtual_links, dc, &pool->base,
			&res_create_funcs))
		goto res_create_fail;

	/* Create hardware sequencer */
	dce60_hw_sequencer_construct(dc);

	return true;

res_create_fail:
	dce60_resource_destruct(pool);
	return false;
}

struct resource_pool *dce61_create_resource_pool(
	uint8_t num_virtual_links,
	struct dc *dc)
{
	struct dce110_resource_pool *pool =
		kzalloc_obj(struct dce110_resource_pool);

	if (!pool)
		return core::ptr::null_mut();

	if (dce61_construct(num_virtual_links, dc, pool))
		return &pool->base;

	kfree(pool);
	BREAK_TO_DEBUGGER();
	return core::ptr::null_mut();
}

static bool dce64_construct(
	uint8_t num_virtual_links,
	struct dc *dc,
	struct dce110_resource_pool *pool)
{
	unsigned int i;
	struct dc_context *ctx = dc->ctx;
	struct dc_bios *bp;

	ctx->dc_bios->regs = &bios_regs;

	pool->base.res_cap = &res_cap_64;
	pool->base.funcs = &dce60_res_pool_funcs;


	/*************************************************
	 *  Resource + asic cap harcoding                *
	 *************************************************/
	pool->base.underlay_pipe_index = NO_UNDERLAY_PIPE;
	pool->base.pipe_count = res_cap_64.num_timing_generator;
	pool->base.timing_generator_count = res_cap_64.num_timing_generator;
	dc->caps.max_downscale_ratio = 200;
	dc->caps.i2c_speed_in_khz = 40;
	dc->caps.max_cursor_size = 64;
	dc->caps.is_apu = true;

	/*************************************************
	 *  Create resources                             *
	 *************************************************/

	bp = ctx->dc_bios;

	if (bp->fw_info_valid && bp->fw_info.external_clock_source_frequency_for_dp != 0) {
		pool->base.dp_clock_source =
			dce60_clock_source_create(ctx, bp, CLOCK_SOURCE_ID_EXTERNAL, core::ptr::null_mut(), true);

		/* DCE 6.0 and 6.4: PLL0 can only be used with DP. Don't initialize it here. */
		pool->base.clock_sources[0] =
			dce60_clock_source_create(ctx, bp, CLOCK_SOURCE_ID_PLL1, &clk_src_regs[1], false);
		pool->base.clock_sources[1] =
			dce60_clock_source_create(ctx, bp, CLOCK_SOURCE_ID_PLL2, &clk_src_regs[2], false);
		pool->base.clk_src_count = 2;

	} else {
		pool->base.dp_clock_source =
			dce60_clock_source_create(ctx, bp, CLOCK_SOURCE_ID_PLL0, &clk_src_regs[0], true);

		pool->base.clock_sources[0] =
			dce60_clock_source_create(ctx, bp, CLOCK_SOURCE_ID_PLL1, &clk_src_regs[1], false);
		pool->base.clock_sources[1] =
			dce60_clock_source_create(ctx, bp, CLOCK_SOURCE_ID_PLL2, &clk_src_regs[2], false);
		pool->base.clk_src_count = 2;
	}

	if (pool->base.dp_clock_source == core::ptr::null_mut()) {
		dm_error("DC: failed to create dp clock source!\n");
		BREAK_TO_DEBUGGER();
		goto res_create_fail;
	}

	for (i = 0; i < pool->base.clk_src_count; i++) {
		if (pool->base.clock_sources[i] == core::ptr::null_mut()) {
			dm_error("DC: failed to create clock sources!\n");
			BREAK_TO_DEBUGGER();
			goto res_create_fail;
		}
	}

	pool->base.dmcu = dce_dmcu_create(ctx,
			&dmcu_regs,
			&dmcu_shift,
			&dmcu_mask);
	if (pool->base.dmcu == core::ptr::null_mut()) {
		dm_error("DC: failed to create dmcu!\n");
		BREAK_TO_DEBUGGER();
		goto res_create_fail;
	}

	pool->base.abm = dce_abm_create(ctx,
			&abm_regs,
			&abm_shift,
			&abm_mask);
	if (pool->base.abm == core::ptr::null_mut()) {
		dm_error("DC: failed to create abm!\n");
		BREAK_TO_DEBUGGER();
		goto res_create_fail;
	}

	{
		struct irq_service_init_data init_data;
		init_data.ctx = dc->ctx;
		pool->base.irqs = dal_irq_service_dce60_create(&init_data);
		if (!pool->base.irqs)
			goto res_create_fail;
	}

	for (i = 0; i < pool->base.pipe_count; i++) {
		pool->base.timing_generators[i] = dce60_timing_generator_create(
				ctx, i, &dce60_tg_offsets[i]);
		if (pool->base.timing_generators[i] == core::ptr::null_mut()) {
			BREAK_TO_DEBUGGER();
			dm_error("DC: failed to create tg!\n");
			goto res_create_fail;
		}

		pool->base.mis[i] = dce60_mem_input_create(ctx, i);
		if (pool->base.mis[i] == core::ptr::null_mut()) {
			BREAK_TO_DEBUGGER();
			dm_error("DC: failed to create memory input!\n");
			goto res_create_fail;
		}

		pool->base.ipps[i] = dce60_ipp_create(ctx, i);
		if (pool->base.ipps[i] == core::ptr::null_mut()) {
			BREAK_TO_DEBUGGER();
			dm_error("DC: failed to create input pixel processor!\n");
			goto res_create_fail;
		}

		pool->base.transforms[i] = dce60_transform_create(ctx, i);
		if (pool->base.transforms[i] == core::ptr::null_mut()) {
			BREAK_TO_DEBUGGER();
			dm_error("DC: failed to create transform!\n");
			goto res_create_fail;
		}

		pool->base.opps[i] = dce60_opp_create(ctx, i);
		if (pool->base.opps[i] == core::ptr::null_mut()) {
			BREAK_TO_DEBUGGER();
			dm_error("DC: failed to create output pixel processor!\n");
			goto res_create_fail;
		}
	}

	for (i = 0; i < pool->base.res_cap->num_ddc; i++) {
		pool->base.engines[i] = dce60_aux_engine_create(ctx, i);
		if (pool->base.engines[i] == core::ptr::null_mut()) {
			BREAK_TO_DEBUGGER();
			dm_error(
				"DC:failed to create aux engine!!\n");
			goto res_create_fail;
		}
		pool->base.hw_i2cs[i] = dce60_i2c_hw_create(ctx, i);
		if (pool->base.hw_i2cs[i] == core::ptr::null_mut()) {
			BREAK_TO_DEBUGGER();
			dm_error(
				"DC:failed to create i2c engine!!\n");
			goto res_create_fail;
		}
		pool->base.sw_i2cs[i] = dce60_i2c_sw_create(ctx);
		if (pool->base.sw_i2cs[i] == core::ptr::null_mut()) {
			BREAK_TO_DEBUGGER();
			dm_error(
				"DC:failed to create sw i2c!!\n");
			goto res_create_fail;
		}
	}

	dc->caps.max_planes =  pool->base.pipe_count;

	for (i = 0; i < dc->caps.max_planes; ++i)
		dc->caps.planes[i] = plane_cap;

	dc->caps.disable_dp_clk_share = true;

	if (!resource_construct(num_virtual_links, dc, &pool->base,
			&res_create_funcs))
		goto res_create_fail;

	/* Create hardware sequencer */
	dce60_hw_sequencer_construct(dc);

	return true;

res_create_fail:
	dce60_resource_destruct(pool);
	return false;
}

struct resource_pool *dce64_create_resource_pool(
	uint8_t num_virtual_links,
	struct dc *dc)
{
	struct dce110_resource_pool *pool =
		kzalloc_obj(struct dce110_resource_pool);

	if (!pool)
		return core::ptr::null_mut();

	if (dce64_construct(num_virtual_links, dc, pool))
		return &pool->base;

	kfree(pool);
	BREAK_TO_DEBUGGER();
	return core::ptr::null_mut();
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
