/* Rust translation of dcn31_dio_link_encoder.c. External types, functions,
 * register helpers, and constants are supplied by the surrounding tree. */

const HDMI_FRL_EQ_LEVEL_SHIFT: u32 = 0x0;
const HDMI_FRL_EQ_LEVEL_MASK: u32 = 0x3;
const HDMI_FRL_EQ_NO_PRE_SHIFT: u32 = 0x5;
const HDMI_FRL_EQ_NO_DEMPH_SHIFT: u32 = 0x6;
const HDMI_FRL_EQ_NO_FFE_SHIFT: u32 = 0x4;

unsafe fn phy_id_from_transmitter(t: transmitter) -> u8 {
    match t {
        TRANSMITTER_UNIPHY_A => 0, TRANSMITTER_UNIPHY_B => 1,
        TRANSMITTER_UNIPHY_C => 2, TRANSMITTER_UNIPHY_D => 3,
        TRANSMITTER_UNIPHY_E => 4, TRANSMITTER_UNIPHY_F => 5,
        TRANSMITTER_UNIPHY_G => 6, _ => 0,
    }
}

unsafe fn has_query_dp_alt(enc: *mut link_encoder) -> bool {
    let srv = (*(*enc).ctx).dmub_srv;
    if (*(*enc).ctx).dce_version >= DCN_VERSION_3_15 { return true; }
    !srv.is_null() && !((*(*srv).dmub).fw_version >= DMUB_FW_VERSION(4, 0, 0)
        && (*(*srv).dmub).fw_version <= DMUB_FW_VERSION(4, 0, 10))
}

unsafe fn query_dp_alt_from_dmub(enc: *mut link_encoder, cmd: *mut dmub_rb_cmd) -> bool {
    let enc10 = TO_DCN10_LINK_ENC(enc);
    core::ptr::write_bytes(cmd, 0, 1);
    (*cmd).query_dp_alt.header.type_ = DMUB_CMD__VBIOS;
    (*cmd).query_dp_alt.header.sub_type = DMUB_CMD__VBIOS_TRANSMITTER_QUERY_DP_ALT;
    (*cmd).query_dp_alt.header.payload_bytes = core::mem::size_of_val(&(*cmd).query_dp_alt.data);
    (*cmd).query_dp_alt.data.phy_id = phy_id_from_transmitter((*enc10).base.transmitter);
    if !dc_wake_and_execute_dmub_cmd((*enc).ctx, cmd, DM_DMUB_WAIT_TYPE_WAIT_WITH_REPLY) { return false; }
    true
}

pub unsafe fn dcn31_link_encoder_set_dio_phy_mux(enc: *mut link_encoder, sel: encoder_type_select, hpo_inst: u32) {
    let enc10 = TO_DCN10_LINK_ENC(enc);
    match (*enc).transmitter {
        TRANSMITTER_UNIPHY_A => { if sel == ENCODER_TYPE_HDMI_FRL { REG_UPDATE!(DIO_LINKA_CNTL, HPO_HDMI_ENC_SEL, hpo_inst); } else if sel == ENCODER_TYPE_DP_128B132B { REG_UPDATE!(DIO_LINKA_CNTL, HPO_DP_ENC_SEL, hpo_inst); } REG_UPDATE!(DIO_LINKA_CNTL, ENC_TYPE_SEL, sel); }
        TRANSMITTER_UNIPHY_B => { if sel == ENCODER_TYPE_HDMI_FRL { REG_UPDATE!(DIO_LINKB_CNTL, HPO_HDMI_ENC_SEL, hpo_inst); } else if sel == ENCODER_TYPE_DP_128B132B { REG_UPDATE!(DIO_LINKB_CNTL, HPO_DP_ENC_SEL, hpo_inst); } REG_UPDATE!(DIO_LINKB_CNTL, ENC_TYPE_SEL, sel); }
        TRANSMITTER_UNIPHY_C => { if sel == ENCODER_TYPE_HDMI_FRL { REG_UPDATE!(DIO_LINKC_CNTL, HPO_HDMI_ENC_SEL, hpo_inst); } else if sel == ENCODER_TYPE_DP_128B132B { REG_UPDATE!(DIO_LINKC_CNTL, HPO_DP_ENC_SEL, hpo_inst); } REG_UPDATE!(DIO_LINKC_CNTL, ENC_TYPE_SEL, sel); }
        TRANSMITTER_UNIPHY_D => { if sel == ENCODER_TYPE_HDMI_FRL { REG_UPDATE!(DIO_LINKD_CNTL, HPO_HDMI_ENC_SEL, hpo_inst); } else if sel == ENCODER_TYPE_DP_128B132B { REG_UPDATE!(DIO_LINKD_CNTL, HPO_DP_ENC_SEL, hpo_inst); } REG_UPDATE!(DIO_LINKD_CNTL, ENC_TYPE_SEL, sel); }
        TRANSMITTER_UNIPHY_E => { if sel == ENCODER_TYPE_HDMI_FRL { REG_UPDATE!(DIO_LINKE_CNTL, HPO_HDMI_ENC_SEL, hpo_inst); } else if sel == ENCODER_TYPE_DP_128B132B { REG_UPDATE!(DIO_LINKE_CNTL, HPO_DP_ENC_SEL, hpo_inst); } REG_UPDATE!(DIO_LINKE_CNTL, ENC_TYPE_SEL, sel); }
        TRANSMITTER_UNIPHY_F => { if sel == ENCODER_TYPE_HDMI_FRL { REG_UPDATE!(DIO_LINKF_CNTL, HPO_HDMI_ENC_SEL, hpo_inst); } else if sel == ENCODER_TYPE_DP_128B132B { REG_UPDATE!(DIO_LINKF_CNTL, HPO_DP_ENC_SEL, hpo_inst); } REG_UPDATE!(DIO_LINKF_CNTL, ENC_TYPE_SEL, sel); }
        _ => {}
    }
}

pub unsafe fn enc31_hw_init(enc: *mut link_encoder) {
    let enc10 = TO_DCN10_LINK_ENC(enc);
    // dmub reads AUX_DPHY_RX_CONTROL0/AUX_DPHY_TX_CONTROL from the VBIOS table in dp_aux_init.
    REG_UPDATE!(TMDS_CTL_BITS, TMDS_CTL0, 1);
    dcn10_aux_initialize(enc10);
}

unsafe fn link_transmitter_control(enc10: *mut dcn10_link_encoder, cntl: *mut bp_transmitter_control) -> bp_result {
    (*(*(*enc10).base.ctx).dc_bios).funcs.transmitter_control((*(*enc10).base.ctx).dc_bios, cntl)
}

pub unsafe fn dpcs31_program_eq_setting(enc: *mut link_encoder, ffe_level: u8, mut de_emphasis_only: bool, mut pre_shoot_only: bool, no_ffe: bool, link_settings: *const dc_hdmi_frl_link_settings) {
    let enc10 = TO_DCN10_LINK_ENC(enc);
    let mut cntl: bp_transmitter_control = core::mem::zeroed();
    if (*(*(*enc10).base.ctx).dc).debug.ignore_ffe { return; }
    if ffe_level < 0x5 { (*enc10).base.txffe_state = ffe_level; }
    if ffe_level == 0xee { (*enc10).base.txffe_state = (*enc10).base.txffe_state.wrapping_add(1); if (*enc10).base.txffe_state > 3 { (*enc10).base.txffe_state = 0; } }
    if no_ffe { de_emphasis_only = true; pre_shoot_only = true; }
    cntl.lane_settings = ((de_emphasis_only as u32) << HDMI_FRL_EQ_NO_PRE_SHIFT) | ((pre_shoot_only as u32) << HDMI_FRL_EQ_NO_DEMPH_SHIFT) | (((*enc10).base.txffe_state as u32 & HDMI_FRL_EQ_LEVEL_MASK) << HDMI_FRL_EQ_LEVEL_SHIFT);
    cntl.lane_select = 0; cntl.action = TRANSMITTER_CONTROL_SET_VOLTAGE_AND_PREEMPASIS; cntl.transmitter = (*enc10).base.transmitter; cntl.connector_obj_id = (*enc10).base.connector; cntl.lanes_number = (*link_settings).frl_num_lanes; cntl.hpd_sel = (*enc10).base.hpd_source;
    cntl.pixel_clock = match (*link_settings).frl_link_rate { HDMI_FRL_LINK_RATE_3GBPS => 166667 / 10, HDMI_FRL_LINK_RATE_6GBPS | HDMI_FRL_LINK_RATE_6GBPS_4LANE => 333333 / 10, HDMI_FRL_LINK_RATE_8GBPS => 444444 / 10, HDMI_FRL_LINK_RATE_10GBPS => 555555 / 10, _ => 666667 / 10 };
    link_transmitter_control(enc10, &mut cntl);
}

static mut dcn31_link_enc_funcs: link_encoder_funcs = link_encoder_funcs {
    read_state: Some(link_enc2_read_state), validate_output_with_stream: Some(dcn30_link_encoder_validate_output_with_stream), hw_init: Some(enc31_hw_init), setup: Some(dcn10_link_encoder_setup), enable_tmds_output: Some(dcn10_link_encoder_enable_tmds_output), enable_dp_output: Some(dcn31_link_encoder_enable_dp_output), enable_dp_mst_output: Some(dcn31_link_encoder_enable_dp_mst_output), disable_output: Some(dcn31_link_encoder_disable_output), dp_set_lane_settings: Some(dcn10_link_encoder_dp_set_lane_settings), dp_set_phy_pattern: Some(dcn10_link_encoder_dp_set_phy_pattern), update_mst_stream_allocation_table: Some(dcn10_link_encoder_update_mst_stream_allocation_table), psr_program_dp_dphy_fast_training: Some(dcn10_psr_program_dp_dphy_fast_training), psr_program_secondary_packet: Some(dcn10_psr_program_secondary_packet), connect_dig_be_to_fe: Some(dcn10_link_encoder_connect_dig_be_to_fe), enable_hpd: Some(dcn10_link_encoder_enable_hpd), disable_hpd: Some(dcn10_link_encoder_disable_hpd), is_dig_enabled: Some(dcn10_is_dig_enabled), destroy: Some(dcn10_link_encoder_destroy), fec_set_enable: Some(enc2_fec_set_enable), fec_set_ready: Some(enc2_fec_set_ready), fec_is_active: Some(enc2_fec_is_active), get_dig_frontend: Some(dcn10_get_dig_frontend), get_dig_mode: Some(dcn10_get_dig_mode), is_in_alt_mode: Some(dcn31_link_encoder_is_in_alt_mode), get_max_link_cap: Some(dcn31_link_encoder_get_max_link_cap), dpcstx_set_order_invert_18_bit: None, set_phy_source: None, dpcs_initialize_phy: None, dpcs_configure_phypll: None, dpcs_configure_dpcs: None, dpcs_enable_dpcs: None, prog_eq_setting: Some(dpcs31_program_eq_setting), get_txffe: Some(dpcs30_get_txffe), set_txffe: Some(dpcs30_set_txffe), set_dio_phy_mux: Some(dcn31_link_encoder_set_dio_phy_mux), get_hpd_state: Some(dcn10_get_hpd_state), program_hpd_filter: Some(dcn10_program_hpd_filter), };

pub unsafe fn dcn31_link_encoder_construct(enc20: *mut dcn20_link_encoder, init_data: *const encoder_init_data, enc_features: *const encoder_feature_support, link_regs: *const dcn10_link_enc_registers, aux_regs: *const dcn10_link_enc_aux_registers, hpd_regs: *const dcn10_link_enc_hpd_registers, link_shift: *const dcn10_link_enc_shift, link_mask: *const dcn10_link_enc_mask) {
    let enc10 = &mut (*enc20).enc10; let mut bp_cap_info: bp_encoder_cap_info = core::mem::zeroed(); let bp_funcs = (*(*(*init_data).ctx).dc_bios).funcs; let mut result = BP_RESULT_OK;
    enc10.base.funcs = &dcn31_link_enc_funcs; enc10.base.ctx = (*init_data).ctx; enc10.base.id = (*init_data).encoder; enc10.base.hpd_gpio = (*init_data).hpd_gpio; enc10.base.hpd_source = (*init_data).hpd_source; enc10.base.connector = (*init_data).connector; enc10.base.preferred_engine = ENGINE_ID_UNKNOWN; enc10.base.features = *enc_features; enc10.base.transmitter = (*init_data).transmitter; enc10.base.output_signals = SIGNAL_TYPE_DVI_SINGLE_LINK | SIGNAL_TYPE_DVI_DUAL_LINK | SIGNAL_TYPE_LVDS | SIGNAL_TYPE_DISPLAY_PORT | SIGNAL_TYPE_DISPLAY_PORT_MST | SIGNAL_TYPE_EDP | SIGNAL_TYPE_HDMI_TYPE_A; enc10.link_regs = link_regs; enc10.aux_regs = aux_regs; enc10.hpd_regs = hpd_regs; enc10.link_shift = link_shift; enc10.link_mask = link_mask;
    enc10.base.preferred_engine = match enc10.base.transmitter { TRANSMITTER_UNIPHY_A=>ENGINE_ID_DIGA, TRANSMITTER_UNIPHY_B=>ENGINE_ID_DIGB, TRANSMITTER_UNIPHY_C=>ENGINE_ID_DIGC, TRANSMITTER_UNIPHY_D=>ENGINE_ID_DIGD, TRANSMITTER_UNIPHY_E=>ENGINE_ID_DIGE, TRANSMITTER_UNIPHY_F=>ENGINE_ID_DIGF, _=>{ ASSERT_CRITICAL!(false); ENGINE_ID_UNKNOWN } };
    enc10.base.features.flags.bits.HDMI_6GB_EN = 1; result = (bp_funcs.get_encoder_cap_info)((*(*init_data).ctx).dc_bios, enc10.base.id, &mut bp_cap_info);
    if result == BP_RESULT_OK { enc10.base.features.flags.bits.IS_HBR2_CAPABLE=bp_cap_info.DP_HBR2_EN; enc10.base.features.flags.bits.IS_HBR3_CAPABLE=bp_cap_info.DP_HBR3_EN; enc10.base.features.flags.bits.HDMI_6GB_EN=bp_cap_info.HDMI_6GB_EN; enc10.base.features.flags.bits.IS_DP2_CAPABLE=bp_cap_info.IS_DP2_CAPABLE; enc10.base.features.flags.bits.IS_UHBR10_CAPABLE=bp_cap_info.DP_UHBR10_EN; enc10.base.features.flags.bits.IS_UHBR13_5_CAPABLE=bp_cap_info.DP_UHBR13_5_EN; enc10.base.features.flags.bits.IS_UHBR20_CAPABLE=bp_cap_info.DP_UHBR20_EN; enc10.base.features.flags.bits.DP_IS_USB_C=bp_cap_info.DP_IS_USB_C; enc10.base.features.flags.bits.IS_HDMI_FRL_CAPABLE=bp_cap_info.IS_HDMI_FRL_CAPABLE; enc10.base.features.flags.bits.IS_FRL_8G_CAPABLE=bp_cap_info.FRL_8G_EN; enc10.base.features.flags.bits.IS_FRL_10G_CAPABLE=bp_cap_info.FRL_10G_EN; enc10.base.features.flags.bits.IS_FRL_12G_CAPABLE=bp_cap_info.FRL_12G_EN; enc10.base.txffe_state=0; } else { DC_LOG_WARNING!("%s: Failed to get encoder_cap_info from VBIOS with error code %d!\n", __func__, result); }
    if (*(*(*init_data).ctx).dc).debug.hdmi20_disable { enc10.base.features.flags.bits.HDMI_6GB_EN=0; } if (*(*(*init_data).ctx).dc).config.force_hdmi21_frl_enc_enable { enc10.base.features.flags.bits.IS_HDMI_FRL_CAPABLE=1; enc10.base.features.flags.bits.IS_FRL_8G_CAPABLE=1; enc10.base.features.flags.bits.IS_FRL_10G_CAPABLE=1; enc10.base.features.flags.bits.IS_FRL_12G_CAPABLE=1; }
}

pub unsafe fn dcn31_link_encoder_construct_minimal(enc20:*mut dcn20_link_encoder, ctx:*mut dc_context, enc_features:*const encoder_feature_support, link_regs:*const dcn10_link_enc_registers, link_shift:*const dcn10_link_enc_shift, link_mask:*const dcn10_link_enc_mask, eng_id:engine_id) { let e=&mut (*enc20).enc10; e.base.funcs=&dcn31_link_enc_funcs; e.base.ctx=ctx; e.base.id.type_=OBJECT_TYPE_ENCODER; e.base.hpd_source=HPD_SOURCEID_UNKNOWN; e.base.connector.type_=OBJECT_TYPE_CONNECTOR; e.base.preferred_engine=eng_id; e.base.features=*enc_features; e.base.transmitter=TRANSMITTER_UNKNOWN; e.link_regs=link_regs; e.link_shift=link_shift; e.link_mask=link_mask; e.base.output_signals=SIGNAL_TYPE_DISPLAY_PORT|SIGNAL_TYPE_DISPLAY_PORT_MST|SIGNAL_TYPE_EDP; }

unsafe fn link_dpia_control(dc_ctx:*mut dc_context, dpia_control:*const dmub_cmd_dig_dpia_control_data)->bool { let mut cmd:dmub_rb_cmd=core::mem::zeroed(); cmd.dig1_dpia_control.header.type_=DMUB_CMD__DPIA; cmd.dig1_dpia_control.header.sub_type=DMUB_CMD__DPIA_DIG1_DPIA_CONTROL; cmd.dig1_dpia_control.header.payload_bytes=core::mem::size_of::<dmub_cmd_dig_dpia_control_data>(); cmd.dig1_dpia_control.dpia_control=*dpia_control; dc_wake_and_execute_dmub_cmd(dc_ctx,&mut cmd,DM_DMUB_WAIT_TYPE_WAIT); true }
unsafe fn link_encoder_disable(enc10:*mut dcn10_link_encoder) { REG_UPDATE!(DP_LINK_CNTL, DP_LINK_TRAINING_COMPLETE, 0); }

pub unsafe fn dcn31_link_encoder_enable_dp_output(enc:*mut link_encoder, ls:*const dc_link_settings, cs:clock_source_id) { let e=TO_DCN10_LINK_ENC(enc); if !link_enc_cfg_is_transmitter_mappable((*enc).ctx.dc,enc) { dcn20_link_encoder_enable_dp_output(enc,ls,cs); } else { let mut c:dmub_cmd_dig_dpia_control_data=core::mem::zeroed(); let link=link_enc_cfg_get_link_using_link_enc((*enc).ctx.dc,(*enc).preferred_engine); enc1_configure_encoder(e,ls); c.action=TRANSMITTER_CONTROL_ENABLE as u8; c.enc_id=(*enc).preferred_engine; c.mode_laneset.digmode=0; c.lanenum=(*ls).lane_count as u8; c.symclk_10khz=(*ls).link_rate*LINK_RATE_REF_FREQ_IN_KHZ/10; c.hpdsel=6; if !link.is_null(){c.dpia_id=(*link).ddc_hw_inst;c.fec_rdy=(*(*link).dc).link_srv.dp_should_enable_fec(link);} else { BREAK_TO_DEBUGGER!(); return;} link_dpia_control((*enc).ctx,&c); } }
pub unsafe fn dcn31_link_encoder_enable_dp_mst_output(enc:*mut link_encoder, ls:*const dc_link_settings, cs:clock_source_id) { let e=TO_DCN10_LINK_ENC(enc); if !link_enc_cfg_is_transmitter_mappable((*enc).ctx.dc,enc) { dcn10_link_encoder_enable_dp_mst_output(enc,ls,cs); } else { let mut c:dmub_cmd_dig_dpia_control_data=core::mem::zeroed(); let link=link_enc_cfg_get_link_using_link_enc((*enc).ctx.dc,(*enc).preferred_engine); enc1_configure_encoder(e,ls); c.action=TRANSMITTER_CONTROL_ENABLE as u8;c.enc_id=(*enc).preferred_engine;c.mode_laneset.digmode=5;c.lanenum=(*ls).lane_count as u8;c.symclk_10khz=(*ls).link_rate*LINK_RATE_REF_FREQ_IN_KHZ/10;c.hpdsel=6;if !link.is_null(){c.dpia_id=(*link).ddc_hw_inst;c.fec_rdy=(*(*link).dc).link_srv.dp_should_enable_fec(link);}else{BREAK_TO_DEBUGGER!();return;}link_dpia_control((*enc).ctx,&c); } }
pub unsafe fn dcn31_link_encoder_disable_output(enc:*mut link_encoder, signal:signal_type) { let e=TO_DCN10_LINK_ENC(enc); if !link_enc_cfg_is_transmitter_mappable((*enc).ctx.dc,enc){dcn10_link_encoder_disable_output(enc,signal);return;} if (*enc).funcs.is_dig_enabled.is_some() && !((*enc).funcs.is_dig_enabled.unwrap())(enc){return;} let link=link_enc_cfg_get_link_using_link_enc((*enc).ctx.dc,(*enc).preferred_engine); let mut c:dmub_cmd_dig_dpia_control_data=core::mem::zeroed();c.action=TRANSMITTER_CONTROL_DISABLE as u8;c.enc_id=(*enc).preferred_engine;c.mode_laneset.digmode=if signal==SIGNAL_TYPE_DISPLAY_PORT{0}else{5};if !link.is_null(){c.dpia_id=(*link).ddc_hw_inst;}else{BREAK_TO_DEBUGGER!();return;}link_dpia_control((*enc).ctx,&c);link_encoder_disable(e); }

pub unsafe fn dcn31_link_encoder_is_in_alt_mode(enc:*mut link_encoder)->bool { let e=TO_DCN10_LINK_ENC(enc); if !(*enc).features.flags.bits.DP_IS_USB_C{return false;} let mut cmd:dmub_rb_cmd=core::mem::zeroed();let mut v=0;if has_query_dp_alt(enc){if !query_dp_alt_from_dmub(enc,&mut cmd){return false;}return cmd.query_dp_alt.data.is_dp_alt_disable==0;} if (*enc).ctx.asic_id.hw_internal_rev!=YELLOW_CARP_B0 {REG_GET!(RDPCSTX_PHY_CNTL6,RDPCS_PHY_DPALT_DISABLE,&mut v);}else if (*e).base.transmitter==TRANSMITTER_UNIPHY_A||(*e).base.transmitter==TRANSMITTER_UNIPHY_B||(*e).base.transmitter==TRANSMITTER_UNIPHY_E{REG_GET!(RDPCSTX_PHY_CNTL6,RDPCS_PHY_DPALT_DISABLE,&mut v);}else{REG_GET!(RDPCSPIPE_PHY_CNTL6,RDPCS_PHY_DPALT_DISABLE,&mut v);}v==0 }
pub unsafe fn dcn31_link_encoder_get_max_link_cap(enc:*mut link_encoder, ls:*mut dc_link_settings){let e=TO_DCN10_LINK_ENC(enc);let mut cmd:dmub_rb_cmd=core::mem::zeroed();let mut v=0;dcn10_link_encoder_get_max_link_cap(enc,ls);if !(*enc).features.flags.bits.DP_IS_USB_C{return;}if has_query_dp_alt(enc){if !query_dp_alt_from_dmub(enc,&mut cmd){return;}if cmd.query_dp_alt.data.is_dp_alt_disable==0&&cmd.query_dp_alt.data.is_usb&&cmd.query_dp_alt.data.is_dp4==0{(*ls).lane_count=MIN!(LANE_COUNT_TWO,(*ls).lane_count);}return;}if (*enc).ctx.asic_id.hw_internal_rev!=YELLOW_CARP_B0{REG_GET!(RDPCSTX_PHY_CNTL6,RDPCS_PHY_DPALT_DP4,&mut v);}else if (*e).base.transmitter==TRANSMITTER_UNIPHY_A||(*e).base.transmitter==TRANSMITTER_UNIPHY_B||(*e).base.transmitter==TRANSMITTER_UNIPHY_E{REG_GET!(RDPCSTX_PHY_CNTL6,RDPCS_PHY_DPALT_DP4,&mut v);}else{REG_GET!(RDPCSPIPE_PHY_CNTL6,RDPCS_PHY_DPALT_DP4,&mut v);}if v==0{(*ls).lane_count=MIN!(LANE_COUNT_TWO,(*ls).lane_count);}}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
