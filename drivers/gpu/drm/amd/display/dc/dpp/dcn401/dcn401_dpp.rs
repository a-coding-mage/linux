/* Rust translation of dcn401_dpp.c. */

pub unsafe fn dpp401_read_state(dpp_base: *mut dpp, s: *mut dcn_dpp_state) {
    let dpp = TO_DCN30_DPP(dpp_base);
    REG_GET((*dpp).tf_regs, DPP_CONTROL, DPP_CLOCK_ENABLE, &mut (*s).is_enabled);
    // TODO: Implement for DCN4
}

pub unsafe fn dpp401_dpp_setup(dpp_base: *mut dpp, format: surface_pixel_format,
    mode: expansion_mode, input_csc_color_matrix: dc_csc_transform,
    input_color_space: dc_color_space, alpha_2bit_lut: *mut cnv_alpha_2bit_lut) {
    let dpp = TO_DCN401_DPP(dpp_base);
    let mut pixel_format: u32 = 0; let mut alpha_en: u32 = 1;
    let mut color_space = COLOR_SPACE_SRGB; let mut select = INPUT_CSC_SELECT_BYPASS;
    let mut is_2bit: u32 = 0; let mut alpha_plane_enable: u32 = 0;
    let mut dealpha_en: u32 = 0; let mut dealpha_ablnd_en: u32 = 0;
    let mut realpha_en: u32 = 0; let mut realpha_ablnd_en: u32 = 0;
    let mut tbl_entry: out_csc_color_matrix = core::mem::zeroed();
    REG_SET_2((*dpp).tf_regs, FORMAT_CONTROL, 0, CNVC_BYPASS, 0, FORMAT_EXPANSION_MODE, mode);
    REG_UPDATE((*dpp).tf_regs, FORMAT_CONTROL, FORMAT_CNV16, 0);
    REG_UPDATE((*dpp).tf_regs, FORMAT_CONTROL, CNVC_BYPASS_MSB_ALIGN, 0);
    REG_UPDATE((*dpp).tf_regs, FORMAT_CONTROL, CLAMP_POSITIVE, 0);
    REG_UPDATE((*dpp).tf_regs, FORMAT_CONTROL, CLAMP_POSITIVE_C, 0);
    REG_UPDATE((*dpp).tf_regs, FORMAT_CONTROL, FORMAT_CROSSBAR_R, 0);
    REG_UPDATE((*dpp).tf_regs, FORMAT_CONTROL, FORMAT_CROSSBAR_G, 1);
    REG_UPDATE((*dpp).tf_regs, FORMAT_CONTROL, FORMAT_CROSSBAR_B, 2);
    match format {
        SURFACE_PIXEL_FORMAT_GRPH_ARGB1555 => pixel_format=1,
        SURFACE_PIXEL_FORMAT_GRPH_RGB565 => { pixel_format=3; alpha_en=0; },
        SURFACE_PIXEL_FORMAT_GRPH_ARGB8888 | SURFACE_PIXEL_FORMAT_GRPH_ABGR8888 => pixel_format=8,
        SURFACE_PIXEL_FORMAT_GRPH_ARGB2101010 | SURFACE_PIXEL_FORMAT_GRPH_ABGR2101010 => { pixel_format=10; is_2bit=1; },
        SURFACE_PIXEL_FORMAT_VIDEO_420_YCbCr => { pixel_format=65; color_space=COLOR_SPACE_YCBCR709; select=INPUT_CSC_SELECT_ICSC; },
        SURFACE_PIXEL_FORMAT_VIDEO_420_YCrCb => { pixel_format=64; color_space=COLOR_SPACE_YCBCR709; select=INPUT_CSC_SELECT_ICSC; },
        SURFACE_PIXEL_FORMAT_VIDEO_420_10bpc_YCbCr => { pixel_format=67; color_space=COLOR_SPACE_YCBCR709; select=INPUT_CSC_SELECT_ICSC; },
        SURFACE_PIXEL_FORMAT_VIDEO_420_10bpc_YCrCb => { pixel_format=66; color_space=COLOR_SPACE_YCBCR709; select=INPUT_CSC_SELECT_ICSC; },
        SURFACE_PIXEL_FORMAT_GRPH_ARGB16161616 | SURFACE_PIXEL_FORMAT_GRPH_ABGR16161616 => pixel_format=26,
        SURFACE_PIXEL_FORMAT_GRPH_ARGB16161616F => pixel_format=24,
        SURFACE_PIXEL_FORMAT_GRPH_ABGR16161616F => pixel_format=25,
        SURFACE_PIXEL_FORMAT_VIDEO_AYCrCb8888 => { pixel_format=12; color_space=COLOR_SPACE_YCBCR709; select=INPUT_CSC_SELECT_ICSC; },
        SURFACE_PIXEL_FORMAT_GRPH_RGB111110_FIX => { pixel_format=112; alpha_en=0; },
        SURFACE_PIXEL_FORMAT_GRPH_BGR101111_FIX => { pixel_format=113; alpha_en=0; },
        SURFACE_PIXEL_FORMAT_VIDEO_ACrYCb2101010 => { pixel_format=114; color_space=COLOR_SPACE_YCBCR709; select=INPUT_CSC_SELECT_ICSC; is_2bit=1; },
        SURFACE_PIXEL_FORMAT_VIDEO_CrYCbA1010102 => { pixel_format=115; color_space=COLOR_SPACE_YCBCR709; select=INPUT_CSC_SELECT_ICSC; is_2bit=1; },
        SURFACE_PIXEL_FORMAT_GRPH_RGBE => { pixel_format=116; alpha_plane_enable=0; },
        SURFACE_PIXEL_FORMAT_GRPH_RGBE_ALPHA => { pixel_format=116; alpha_plane_enable=1; },
        SURFACE_PIXEL_FORMAT_GRPH_RGB111110_FLOAT => { pixel_format=118; alpha_en=0; },
        SURFACE_PIXEL_FORMAT_GRPH_BGR101111_FLOAT => { pixel_format=119; alpha_en=0; }, _ => {}
    }
    color_space = if input_color_space != 0 { input_color_space } else { color_space };
    if is_2bit == 1 && !alpha_2bit_lut.is_null() { REG_UPDATE((*dpp).tf_regs, ALPHA_2BIT_LUT, ALPHA_2BIT_LUT0, (*alpha_2bit_lut).lut0); REG_UPDATE((*dpp).tf_regs, ALPHA_2BIT_LUT, ALPHA_2BIT_LUT1, (*alpha_2bit_lut).lut1); REG_UPDATE((*dpp).tf_regs, ALPHA_2BIT_LUT, ALPHA_2BIT_LUT2, (*alpha_2bit_lut).lut2); REG_UPDATE((*dpp).tf_regs, ALPHA_2BIT_LUT, ALPHA_2BIT_LUT3, (*alpha_2bit_lut).lut3); }
    REG_SET_2((*dpp).tf_regs, CNVC_SURFACE_PIXEL_FORMAT, 0, CNVC_SURFACE_PIXEL_FORMAT, pixel_format, CNVC_ALPHA_PLANE_ENABLE, alpha_plane_enable);
    REG_UPDATE((*dpp).tf_regs, FORMAT_CONTROL, FORMAT_CONTROL__ALPHA_EN, alpha_en);
    REG_SET_2((*dpp).tf_regs, PRE_DEALPHA, 0, PRE_DEALPHA_EN, dealpha_en, PRE_DEALPHA_ABLND_EN, dealpha_ablnd_en);
    REG_SET_2((*dpp).tf_regs, PRE_REALPHA, 0, PRE_REALPHA_EN, realpha_en, PRE_REALPHA_ABLND_EN, realpha_ablnd_en);
    if input_csc_color_matrix.enable_adjustment == true {
        for i in 0..12 { tbl_entry.regval[i] = input_csc_color_matrix.matrix[i]; }
        tbl_entry.color_space = input_color_space;
        select = if dpp3_should_bypass_post_csc_for_colorspace(color_space) { INPUT_CSC_SELECT_BYPASS } else { INPUT_CSC_SELECT_ICSC };
        dpp3_program_post_csc(dpp_base, color_space, select, &mut tbl_entry);
    } else { dpp3_program_post_csc(dpp_base, color_space, select, core::ptr::null_mut()); }
}

pub unsafe fn dpp401_construct(dpp: *mut dcn401_dpp, ctx: *mut dc_context, inst: u32,
    tf_regs: *const dcn401_dpp_registers, tf_shift: *const dcn401_dpp_shift,
    tf_mask: *const dcn401_dpp_mask) -> bool {
    (*dpp).base.ctx=ctx; (*dpp).base.inst=inst; (*dpp).tf_regs=tf_regs; (*dpp).tf_shift=tf_shift; (*dpp).tf_mask=tf_mask; true
}

// Function table and capability data from the C implementation.  The pointed-to
// functions and structures are supplied by the surrounding driver translation.
static mut dcn401_dpp_funcs: dpp_funcs = dpp_funcs {
    dpp_program_gamcor_lut: Some(dpp3_program_gamcor_lut), dpp_read_state: Some(dpp401_read_state),
    dpp_reset: Some(dpp_reset), dpp_set_scaler: Some(dpp401_dscl_set_scaler_manual_scale),
    dpp_get_optimal_number_of_taps: Some(dpp3_get_optimal_number_of_taps), dpp_set_gamut_remap: None,
    dpp_set_csc_adjustment: None, dpp_set_csc_default: None, dpp_program_regamma_pwl: None,
    dpp_set_pre_degam: Some(dpp3_set_pre_degam), dpp_program_input_lut: None, dpp_full_bypass: None,
    dpp_setup: Some(dpp401_dpp_setup), dpp_program_degamma_pwl: None,
    dpp_program_cm_dealpha: Some(dpp3_program_cm_dealpha), dpp_program_cm_bias: Some(dpp3_program_cm_bias),
    dpp_program_blnd_lut: None, dpp_program_shaper_lut: None, dpp_program_3dlut: None,
    dpp_program_bias_and_scale: Some(dpp35_program_bias_and_scale_fcnv), dpp_cnv_set_alpha_keyer: Some(dpp2_cnv_set_alpha_keyer),
    set_cursor_attributes: Some(dpp401_set_cursor_attributes), set_cursor_position: Some(dpp401_set_cursor_position),
    set_optional_cursor_attributes: Some(dpp401_set_optional_cursor_attributes), dpp_dppclk_control: Some(dpp1_dppclk_control),
    dpp_set_hdr_multiplier: Some(dpp3_set_hdr_multiplier), dpp_read_reg_state: Some(dpp30_read_reg_state),
    set_cursor_matrix: Some(dpp401_set_cursor_matrix),
};

static mut dcn401_dpp_cap: dpp_caps = dpp_caps {
    dscl_data_proc_format: DSCL_DATA_PRCESSING_FLOAT_FORMAT,
    max_lb_partitions: 63, dscl_calc_lb_num_partitions: Some(dscl401_calc_lb_num_partitions),
};

pub unsafe fn dscl401_calc_lb_num_partitions(s: *const scaler_data, lb_config: lb_memory_config, y: *mut i32, c: *mut i32) {
    let mut ly=(*s).viewport.width.min((*s).recout.width); let mut lc=(*s).viewport_c.width.min((*s).recout.width); if ly==0 {ly=1} if lc==0 {lc=1};
    let my=(ly+5)/6; let mc=(lc+5)/6; let ma=my; let (mut by,mut bc,ba) = lb_sizes((*s).viewport.width==(*s).h_active && (*s).viewport.height==(*s).v_active, lb_config);
    *y=by/my; *c=bc/mc; let pa=ba/ma; if (*s).lb_params.alpha_en && pa<*y {*y=pa} if *y>64 {*y=64} if *c>64 {*c=64};
}

pub unsafe fn dscl401_spl_calc_lb_num_partitions(alpha_en: bool, s: *const spl_scaler_data, lb_config: lb_memory_config, y: *mut i32, c: *mut i32) {
    let ly=(*s).viewport.width.min((*s).recout.width).max(1); let lc=(*s).viewport_c.width.min((*s).recout.width).max(1); let my=(ly+5)/6; let mc=(lc+5)/6; let ma=my;
    let full=(*s).viewport.width==(*s).h_active && (*s).viewport.height==(*s).v_active && (*s).taps.h_taps==1 && (*s).taps.v_taps==1; let (by,bc,ba)=lb_sizes(full,lb_config); *y=by/my; *c=bc/mc; let pa=ba/ma; if alpha_en&&pa<*y {*y=pa} if *y>64 {*y=64} if *c>64 {*c=64};
}

unsafe fn lb_sizes(full: bool, c: lb_memory_config) -> (i32,i32,i32) { match c { LB_MEMORY_CONFIG_1=>(970,970,970), LB_MEMORY_CONFIG_2=>(1290,1290,1290), LB_MEMORY_CONFIG_3=>if full {(5770,2260,3430)} else {(3712,2260,3228)}, _=>if full {(3430,3430,3430)} else {(2744,2744,2744)} } }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
