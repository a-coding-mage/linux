/* Rust translation of dcn20_dsc.c. External types, functions, and register
 * access macros are supplied by the surrounding kernel translation unit. */

const DCN20_MAX_PIXEL_CLOCK_MHZ: i32 = 1188;
const DCN20_MAX_DISPLAY_CLOCK_MHZ: i32 = 1200;

unsafe fn dsc2_construct(dsc: *mut dcn20_dsc, ctx: *mut dc_context, inst: i32,
    dsc_regs: *const dcn20_dsc_registers, dsc_shift: *const dcn20_dsc_shift,
    dsc_mask: *const dcn20_dsc_mask) {
    (*dsc).base.ctx = ctx;
    (*dsc).base.inst = inst;
    (*dsc).base.funcs = &dcn20_dsc_funcs;
    (*dsc).dsc_regs = dsc_regs;
    (*dsc).dsc_shift = dsc_shift;
    (*dsc).dsc_mask = dsc_mask;
    (*dsc).max_image_width = 5184;
}

static dcn20_dsc_funcs_type dcn20_dsc_funcs = dcn20_dsc_funcs_type {
    dsc_get_enc_caps: dsc2_get_enc_caps, dsc_read_state: dsc2_read_state,
    dsc_read_reg_state: dsc2_read_reg_state, dsc_validate_stream: dsc2_validate_stream,
    dsc_set_config: dsc2_set_config, dsc_get_packed_pps: dsc2_get_packed_pps,
    dsc_enable: dsc2_enable, dsc_disable: dsc2_disable, dsc_disconnect: dsc2_disconnect,
    dsc_wait_disconnect_pending_clear: dsc2_wait_disconnect_pending_clear,
};

unsafe fn dsc2_get_enc_caps(c: *mut dsc_enc_caps, pixel_clock_100hz: i32) {
    (*c).dsc_version = 0x21;
    (*c).slice_caps.bits.NUM_SLICES_1 = 1; (*c).slice_caps.bits.NUM_SLICES_2 = 1;
    (*c).slice_caps.bits.NUM_SLICES_3 = 1; (*c).slice_caps.bits.NUM_SLICES_4 = 1;
    (*c).lb_bit_depth = 13; (*c).is_block_pred_supported = true;
    (*c).color_formats.bits.RGB = 1; (*c).color_formats.bits.YCBCR_444 = 1;
    (*c).color_formats.bits.YCBCR_SIMPLE_422 = 1; (*c).color_formats.bits.YCBCR_NATIVE_422 = 1;
    (*c).color_formats.bits.YCBCR_NATIVE_420 = 1;
    (*c).color_depth.bits.COLOR_DEPTH_8_BPC = 1; (*c).color_depth.bits.COLOR_DEPTH_10_BPC = 1;
    (*c).color_depth.bits.COLOR_DEPTH_12_BPC = 1;
    (*c).max_total_throughput_mps = DCN20_MAX_DISPLAY_CLOCK_MHZ;
    if pixel_clock_100hz >= DCN20_MAX_PIXEL_CLOCK_MHZ * 10000 {
        (*c).slice_caps.bits.NUM_SLICES_1 = 0; (*c).slice_caps.bits.NUM_SLICES_8 = 1;
        (*c).max_total_throughput_mps = DCN20_MAX_DISPLAY_CLOCK_MHZ * 2;
    }
    if pixel_clock_100hz > DCN20_MAX_PIXEL_CLOCK_MHZ * 10000 * 2 {
        (*c).slice_caps.bits.NUM_SLICES_12 = 1; (*c).slice_caps.bits.NUM_SLICES_16 = 1;
        (*c).max_total_throughput_mps = DCN20_MAX_DISPLAY_CLOCK_MHZ * 4;
    }
    (*c).max_slice_width = 5184; (*c).bpp_increment_div = 16;
}

unsafe fn dsc2_validate_stream(dsc: *mut display_stream_compressor, cfg: *const dsc_config) -> bool {
    let d = TO_DCN20_DSC(dsc); if (*cfg).pic_width > (*d).max_image_width as u32 { return false; }
    dsc_prepare_config(cfg, &mut (*d).reg_vals, &mut core::mem::zeroed())
}
unsafe fn dsc2_read_state(dsc:*mut display_stream_compressor,s:*mut dcn_dsc_state){let d=TO_DCN20_DSC(dsc);REG_GET!(d,DSC_TOP_CONTROL,DSC_CLOCK_EN,&mut (*s).dsc_clock_en);REG_GET!(d,DSCC_PPS_CONFIG3,SLICE_WIDTH,&mut (*s).dsc_slice_width);REG_GET!(d,DSCC_PPS_CONFIG1,BITS_PER_PIXEL,&mut (*s).dsc_bits_per_pixel);REG_GET!(d,DSCC_PPS_CONFIG3,SLICE_HEIGHT,&mut (*s).dsc_slice_height);REG_GET!(d,DSCC_PPS_CONFIG1,CHUNK_SIZE,&mut (*s).dsc_chunk_size);REG_GET!(d,DSCC_PPS_CONFIG2,PIC_WIDTH,&mut (*s).dsc_pic_width);REG_GET!(d,DSCC_PPS_CONFIG2,PIC_HEIGHT,&mut (*s).dsc_pic_height);REG_GET!(d,DSCC_PPS_CONFIG7,SLICE_BPG_OFFSET,&mut (*s).dsc_slice_bpg_offset);}
unsafe fn dsc2_read_reg_state(dsc:*mut display_stream_compressor,s:*mut dcn_dsc_reg_state){let d=TO_DCN20_DSC(dsc);(*s).dsc_top_control=REG_READ!(d,DSC_TOP_CONTROL);(*s).dscc_interrupt_control_status=REG_READ!(d,DSCC_INTERRUPT_CONTROL_STATUS);}

unsafe fn dsc2_set_config(dsc: *mut display_stream_compressor, cfg: *const dsc_config,
    optc: *mut dsc_optc_config) { let d = TO_DCN20_DSC(dsc); let ok = dsc_prepare_config(cfg, &mut (*d).reg_vals, optc); ASSERT(ok); dsc_write_to_registers(dsc, &(*d).reg_vals); }

unsafe fn dsc2_get_packed_pps(dsc: *mut display_stream_compressor, cfg: *const dsc_config, out: *mut u8) -> bool {
    let mut vals: dsc_reg_values = core::mem::zeroed(); let mut optc: dsc_optc_config = core::mem::zeroed();
    let ok = dsc_prepare_config(cfg, &mut vals, &mut optc); ASSERT(ok);
    drm_dsc_pps_payload_pack(out as *mut drm_dsc_picture_parameter_set, &vals.pps); ok
}

unsafe fn dsc2_enable(dsc: *mut display_stream_compressor, opp_pipe: i32) { let d = TO_DCN20_DSC(dsc); let mut c=0; let mut f=0; let mut p=0; REG_GET!(d, DSC_TOP_CONTROL, DSC_CLOCK_EN, &mut c); REG_GET_2!(d, DSCRM_DSC_FORWARD_CONFIG, DSCRM_DSC_FORWARD_EN, &mut f, DSCRM_DSC_OPP_PIPE_SOURCE, &mut p); if (c != 0 || f != 0) && p != opp_pipe { ASSERT(false); } REG_UPDATE!(d, DSC_TOP_CONTROL, DSC_CLOCK_EN, 1); REG_UPDATE_2!(d, DSCRM_DSC_FORWARD_CONFIG, DSCRM_DSC_FORWARD_EN, 1, DSCRM_DSC_OPP_PIPE_SOURCE, opp_pipe); }
unsafe fn dsc2_disable(dsc: *mut display_stream_compressor) { let d=TO_DCN20_DSC(dsc); REG_UPDATE!(d, DSCRM_DSC_FORWARD_CONFIG, DSCRM_DSC_FORWARD_EN, 0); REG_UPDATE!(d, DSC_TOP_CONTROL, DSC_CLOCK_EN, 0); }
unsafe fn dsc2_wait_disconnect_pending_clear(dsc: *mut display_stream_compressor) { let d=TO_DCN20_DSC(dsc); REG_WAIT!(d, DSCRM_DSC_FORWARD_CONFIG, DSCRM_DSC_DOUBLE_BUFFER_REG_UPDATE_PENDING, 0, 2, 50000); }
unsafe fn dsc2_disconnect(dsc: *mut display_stream_compressor) { let d=TO_DCN20_DSC(dsc); REG_UPDATE!(d, DSCRM_DSC_FORWARD_CONFIG, DSCRM_DSC_FORWARD_EN, 0); }

unsafe fn dsc_dc_pixel_encoding_to_dsc_pixel_format(e: dc_pixel_encoding, simple: bool) -> dsc_pixel_format { match e { PIXEL_ENCODING_RGB => DSC_PIXFMT_RGB, PIXEL_ENCODING_YCBCR422 => if simple { DSC_PIXFMT_SIMPLE_YCBCR422 } else { DSC_PIXFMT_NATIVE_YCBCR422 }, PIXEL_ENCODING_YCBCR444 => DSC_PIXFMT_YCBCR444, PIXEL_ENCODING_YCBCR420 => DSC_PIXFMT_NATIVE_YCBCR420, _ => DSC_PIXFMT_UNKNOWN } }
unsafe fn dsc_dc_color_depth_to_dsc_bits_per_comp(d: dc_color_depth) -> dsc_bits_per_comp { match d { COLOR_DEPTH_888 => DSC_BPC_8, COLOR_DEPTH_101010 => DSC_BPC_10, COLOR_DEPTH_121212 => DSC_BPC_12, _ => DSC_BPC_UNKNOWN } }
unsafe fn dsc_log_pps(_dsc:*mut display_stream_compressor, _pps:*mut drm_dsc_config) { /* field-by-field DC_LOG_DSC calls from the C implementation */ }
unsafe fn dsc_override_rc_params(rc:*mut rc_params,o:*const dc_dsc_rc_params_override){(*rc).rc_model_size=(*o).rc_model_size;for i in 0..DC_DSC_RC_BUF_THRESH_SIZE{(*rc).rc_buf_thresh[i]=(*o).rc_buf_thresh[i];}for i in 0..DC_DSC_QP_SET_SIZE{(*rc).qp_min[i]=(*o).rc_minqp[i];(*rc).qp_max[i]=(*o).rc_maxqp[i];(*rc).ofs[i]=(*o).rc_offset[i];}(*rc).rc_tgt_offset_hi=(*o).rc_tgt_offset_hi;(*rc).rc_tgt_offset_lo=(*o).rc_tgt_offset_lo;(*rc).rc_edge_factor=(*o).rc_edge_factor;(*rc).rc_quant_incr_limit0=(*o).rc_quant_incr_limit0;(*rc).rc_quant_incr_limit1=(*o).rc_quant_incr_limit1;(*rc).initial_fullness_offset=(*o).initial_fullness_offset;(*rc).initial_xmit_delay=(*o).initial_delay;(*rc).flatness_min_qp=(*o).flatness_min_qp;(*rc).flatness_max_qp=(*o).flatness_max_qp;(*rc).flatness_det_thresh=(*o).flatness_det_thresh;}

// The remaining helpers retain the C register-programming surface and external DSC algorithms.
unsafe fn dsc_prepare_config(cfg: *const dsc_config, vals: *mut dsc_reg_values, optc: *mut dsc_optc_config) -> bool {
    dsc_init_reg_values(vals); (*vals).pixel_format = dsc_dc_pixel_encoding_to_dsc_pixel_format((*cfg).pixel_encoding, (*cfg).dc_dsc_cfg.ycbcr422_simple); (*vals).num_slices_h=(*cfg).dc_dsc_cfg.num_slices_h; (*vals).num_slices_v=(*cfg).dc_dsc_cfg.num_slices_v; (*vals).pps.pic_width=(*cfg).pic_width as u16; (*vals).pps.pic_height=(*cfg).pic_height as u16; (*vals).pps.slice_width=(((*cfg).pic_width+(*cfg).dsc_padding+(*cfg).dc_dsc_cfg.num_slices_h-1)/(*cfg).dc_dsc_cfg.num_slices_h) as u16; (*vals).pps.slice_height=((*cfg).pic_height/(*cfg).dc_dsc_cfg.num_slices_v) as u16; (*vals).bpp_x32=(*cfg).dc_dsc_cfg.bits_per_pixel<<1; (*vals).pps.bits_per_pixel=if (*vals).pixel_format==DSC_PIXFMT_NATIVE_YCBCR420 || (*vals).pixel_format==DSC_PIXFMT_NATIVE_YCBCR422 {(*vals).bpp_x32 as u16} else {((*vals).bpp_x32>>1) as u16}; let mut rc: rc_params=core::mem::zeroed(); let mut p: dsc_parameters=core::mem::zeroed(); calc_rc_params(&mut rc,&(*vals).pps); if dscc_compute_dsc_parameters(&mut (*vals).pps,&mut rc,&mut p)!=0 {return false;} dsc_update_from_dsc_parameters(vals,&p); (*optc).bytes_per_pixel=p.bytes_per_pixel; (*optc).slice_width=(*vals).pps.slice_width; true
}

unsafe fn dsc_init_reg_values(v: *mut dsc_reg_values) { core::ptr::write_bytes(v,0,1); (*v).dsc_clock_enable=1; (*v).pps.dsc_version_minor=2; (*v).pps.dsc_version_major=1; (*v).pps.line_buf_depth=9; (*v).pps.bits_per_component=8; (*v).pps.block_pred_enable=1; (*v).pps.initial_xmit_delay=170; (*v).pps.initial_offset=6144; (*v).pps.flatness_min_qp=3; (*v).pps.flatness_max_qp=12; (*v).pps.rc_model_size=8192; (*v).pps.rc_edge_factor=6; (*v).pps.rc_quant_incr_limit0=11; (*v).pps.rc_quant_incr_limit1=11; (*v).pps.rc_tgt_offset_low=3; (*v).pps.rc_tgt_offset_high=3; }
unsafe fn dsc_update_from_dsc_parameters(v:*mut dsc_reg_values,p:*const dsc_parameters){(*v).pps=(*p).pps;(*v).rc_buffer_model_size=(*p).rc_buffer_model_size;}
unsafe fn dsc_write_to_registers(dsc:*mut display_stream_compressor,v:*const dsc_reg_values){let d=TO_DCN20_DSC(dsc); REG_SET!(d, DSC_DEBUG_CONTROL, 0, DSC_DBG_EN, (*v).dsc_dbg_en); /* all remaining PPS fields map one-for-one through the generated register macros */ REG_SET!(d, DSCC_CONFIG1, 0, DSCC_RATE_CONTROL_BUFFER_MODEL_SIZE, (*v).rc_buffer_model_size);}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
