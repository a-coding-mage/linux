/* Copyright 2017 Advanced Micro Devices, Inc. */

// Dependencies supplied by the surrounding translation unit:
// dsc.h, dsc/dscc_types.h, and drm/display/drm_dsc.h.

macro_rules! TO_DCN20_DSC { ($dsc:expr) => { container_of!($dsc, dcn20_dsc, base) }; }

macro_rules! DSC_REG_LIST_DCN20 {
    ($id:expr) => {
        SRI!(DSC_TOP_CONTROL, DSC_TOP, $id), SRI!(DSC_DEBUG_CONTROL, DSC_TOP, $id),
        SRI!(DSCC_CONFIG0, DSCC, $id), SRI!(DSCC_CONFIG1, DSCC, $id), SRI!(DSCC_STATUS, DSCC, $id),
        SRI!(DSCC_INTERRUPT_CONTROL_STATUS, DSCC, $id), SRI!(DSCC_PPS_CONFIG0, DSCC, $id),
        SRI!(DSCC_PPS_CONFIG1, DSCC, $id), SRI!(DSCC_PPS_CONFIG2, DSCC, $id), SRI!(DSCC_PPS_CONFIG3, DSCC, $id),
        SRI!(DSCC_PPS_CONFIG4, DSCC, $id), SRI!(DSCC_PPS_CONFIG5, DSCC, $id), SRI!(DSCC_PPS_CONFIG6, DSCC, $id),
        SRI!(DSCC_PPS_CONFIG7, DSCC, $id), SRI!(DSCC_PPS_CONFIG8, DSCC, $id), SRI!(DSCC_PPS_CONFIG9, DSCC, $id),
        SRI!(DSCC_PPS_CONFIG10, DSCC, $id), SRI!(DSCC_PPS_CONFIG11, DSCC, $id), SRI!(DSCC_PPS_CONFIG12, DSCC, $id),
        SRI!(DSCC_PPS_CONFIG13, DSCC, $id), SRI!(DSCC_PPS_CONFIG14, DSCC, $id), SRI!(DSCC_PPS_CONFIG15, DSCC, $id),
        SRI!(DSCC_PPS_CONFIG16, DSCC, $id), SRI!(DSCC_PPS_CONFIG17, DSCC, $id), SRI!(DSCC_PPS_CONFIG18, DSCC, $id),
        SRI!(DSCC_PPS_CONFIG19, DSCC, $id), SRI!(DSCC_PPS_CONFIG20, DSCC, $id), SRI!(DSCC_PPS_CONFIG21, DSCC, $id),
        SRI!(DSCC_PPS_CONFIG22, DSCC, $id), SRI!(DSCC_MEM_POWER_CONTROL, DSCC, $id),
        SRI!(DSCC_R_Y_SQUARED_ERROR_LOWER, DSCC, $id), SRI!(DSCC_R_Y_SQUARED_ERROR_UPPER, DSCC, $id),
        SRI!(DSCC_G_CB_SQUARED_ERROR_LOWER, DSCC, $id), SRI!(DSCC_G_CB_SQUARED_ERROR_UPPER, DSCC, $id),
        SRI!(DSCC_B_CR_SQUARED_ERROR_LOWER, DSCC, $id), SRI!(DSCC_B_CR_SQUARED_ERROR_UPPER, DSCC, $id),
        SRI!(DSCC_MAX_ABS_ERROR0, DSCC, $id), SRI!(DSCC_MAX_ABS_ERROR1, DSCC, $id),
        SRI!(DSCC_RATE_BUFFER0_MAX_FULLNESS_LEVEL, DSCC, $id), SRI!(DSCC_RATE_BUFFER1_MAX_FULLNESS_LEVEL, DSCC, $id),
        SRI!(DSCC_RATE_BUFFER2_MAX_FULLNESS_LEVEL, DSCC, $id), SRI!(DSCC_RATE_BUFFER3_MAX_FULLNESS_LEVEL, DSCC, $id),
        SRI!(DSCC_RATE_CONTROL_BUFFER0_MAX_FULLNESS_LEVEL, DSCC, $id), SRI!(DSCC_RATE_CONTROL_BUFFER1_MAX_FULLNESS_LEVEL, DSCC, $id),
        SRI!(DSCC_RATE_CONTROL_BUFFER2_MAX_FULLNESS_LEVEL, DSCC, $id), SRI!(DSCC_RATE_CONTROL_BUFFER3_MAX_FULLNESS_LEVEL, DSCC, $id),
        SRI!(DSCC_TEST_DEBUG_BUS_ROTATE, DSCC, $id), SRI!(DSCCIF_CONFIG0, DSCCIF, $id),
        SRI!(DSCCIF_CONFIG1, DSCCIF, $id), SRI!(DSCRM_DSC_FORWARD_CONFIG, DSCRM, $id)
    };
}

macro_rules! DSC_SF { ($reg:ident, $field:ident, $post_fix:ident) => { .$field = concat_idents!($reg, __, $field, $post_fix) }; }
// Used in resolving the corner case with duplicate field name.
macro_rules! DSC2_SF { ($reg:ident, $field:ident, $post_fix:ident) => { .$field = concat_idents!($reg, _, $field, $post_fix) }; }

macro_rules! dsc_fields {
    ($t:ty) => {
        pub DSC_CLOCK_EN: $t, pub DSC_DISPCLK_R_GATE_DIS: $t, pub DSC_DSCCLK_R_GATE_DIS: $t,
        pub DSC_DBG_EN: $t, pub DSC_TEST_CLOCK_MUX_SEL: $t, pub ICH_RESET_AT_END_OF_LINE: $t,
        pub NUMBER_OF_SLICES_PER_LINE: $t, pub ALTERNATE_ICH_ENCODING_EN: $t,
        pub NUMBER_OF_SLICES_IN_VERTICAL_DIRECTION: $t, pub DSCC_RATE_CONTROL_BUFFER_MODEL_SIZE: $t,
        pub DSCC_DOUBLE_BUFFER_REG_UPDATE_PENDING: $t,
        pub DSC_VERSION_MINOR: $t, pub DSC_VERSION_MAJOR: $t, pub PPS_IDENTIFIER: $t, pub LINEBUF_DEPTH: $t,
        pub DSCC_PPS_CONFIG0__BITS_PER_COMPONENT: $t, pub BITS_PER_PIXEL: $t, pub VBR_ENABLE: $t,
        pub SIMPLE_422: $t, pub CONVERT_RGB: $t, pub BLOCK_PRED_ENABLE: $t, pub NATIVE_422: $t,
        pub NATIVE_420: $t, pub CHUNK_SIZE: $t, pub PIC_WIDTH: $t, pub PIC_HEIGHT: $t,
        pub SLICE_WIDTH: $t, pub SLICE_HEIGHT: $t, pub INITIAL_XMIT_DELAY: $t, pub INITIAL_DEC_DELAY: $t,
        pub INITIAL_SCALE_VALUE: $t, pub SCALE_INCREMENT_INTERVAL: $t, pub SCALE_DECREMENT_INTERVAL: $t,
        pub FIRST_LINE_BPG_OFFSET: $t, pub SECOND_LINE_BPG_OFFSET: $t, pub NFL_BPG_OFFSET: $t,
        pub SLICE_BPG_OFFSET: $t, pub NSL_BPG_OFFSET: $t, pub SECOND_LINE_OFFSET_ADJ: $t,
        pub INITIAL_OFFSET: $t, pub FINAL_OFFSET: $t, pub FLATNESS_MIN_QP: $t, pub FLATNESS_MAX_QP: $t,
        pub RC_MODEL_SIZE: $t, pub RC_EDGE_FACTOR: $t, pub RC_QUANT_INCR_LIMIT0: $t,
        pub RC_QUANT_INCR_LIMIT1: $t, pub RC_TGT_OFFSET_LO: $t, pub RC_TGT_OFFSET_HI: $t,
        pub DSCC_DEFAULT_MEM_LOW_POWER_STATE: $t, pub DSCC_MEM_PWR_FORCE: $t, pub DSCC_MEM_PWR_DIS: $t,
        pub DSCC_MEM_PWR_STATE: $t, pub DSCC_NATIVE_422_MEM_PWR_FORCE: $t, pub DSCC_NATIVE_422_MEM_PWR_DIS: $t,
        pub DSCC_NATIVE_422_MEM_PWR_STATE: $t, pub DSCC_R_Y_SQUARED_ERROR_LOWER: $t,
        pub DSCC_R_Y_SQUARED_ERROR_UPPER: $t, pub DSCC_G_CB_SQUARED_ERROR_LOWER: $t,
        pub DSCC_G_CB_SQUARED_ERROR_UPPER: $t, pub DSCC_B_CR_SQUARED_ERROR_LOWER: $t,
        pub DSCC_B_CR_SQUARED_ERROR_UPPER: $t, pub DSCC_R_Y_MAX_ABS_ERROR: $t,
        pub DSCC_G_CB_MAX_ABS_ERROR: $t, pub DSCC_B_CR_MAX_ABS_ERROR: $t,
        pub DSCC_RATE_BUFFER0_MAX_FULLNESS_LEVEL: $t, pub DSCC_RATE_BUFFER1_MAX_FULLNESS_LEVEL: $t,
        pub DSCC_RATE_BUFFER2_MAX_FULLNESS_LEVEL: $t, pub DSCC_RATE_BUFFER3_MAX_FULLNESS_LEVEL: $t,
        pub DSCC_RATE_CONTROL_BUFFER0_MAX_FULLNESS_LEVEL: $t, pub DSCC_RATE_CONTROL_BUFFER1_MAX_FULLNESS_LEVEL: $t,
        pub DSCC_RATE_CONTROL_BUFFER2_MAX_FULLNESS_LEVEL: $t, pub DSCC_RATE_CONTROL_BUFFER3_MAX_FULLNESS_LEVEL: $t,
        pub DSCC_TEST_DEBUG_BUS0_ROTATE: $t, pub DSCC_TEST_DEBUG_BUS1_ROTATE: $t,
        pub DSCC_TEST_DEBUG_BUS2_ROTATE: $t, pub DSCC_TEST_DEBUG_BUS3_ROTATE: $t,
        pub INPUT_INTERFACE_UNDERFLOW_RECOVERY_EN: $t, pub INPUT_INTERFACE_UNDERFLOW_OCCURRED_INT_EN: $t,
        pub INPUT_INTERFACE_UNDERFLOW_OCCURRED_STATUS: $t, pub INPUT_PIXEL_FORMAT: $t,
        pub DSCCIF_CONFIG0__BITS_PER_COMPONENT: $t, pub DOUBLE_BUFFER_REG_UPDATE_PENDING: $t,
        pub DSCRM_DSC_FORWARD_EN: $t, pub DSCRM_DSC_OPP_PIPE_SOURCE: $t,
    };
}

#[repr(C)]
pub enum dsc_bits_per_comp { DSC_BPC_8 = 8, DSC_BPC_10 = 10, DSC_BPC_12 = 12, DSC_BPC_UNKNOWN }

#[repr(C)]
pub struct dcn20_dsc_registers {
    pub DSC_TOP_CONTROL: u32, pub DSC_DEBUG_CONTROL: u32, pub DSCC_CONFIG0: u32, pub DSCC_CONFIG1: u32,
    pub DSCC_STATUS: u32, pub DSCC_INTERRUPT_CONTROL_STATUS: u32,
    pub DSCC_PPS_CONFIG0: u32, pub DSCC_PPS_CONFIG1: u32, pub DSCC_PPS_CONFIG2: u32, pub DSCC_PPS_CONFIG3: u32,
    pub DSCC_PPS_CONFIG4: u32, pub DSCC_PPS_CONFIG5: u32, pub DSCC_PPS_CONFIG6: u32, pub DSCC_PPS_CONFIG7: u32,
    pub DSCC_PPS_CONFIG8: u32, pub DSCC_PPS_CONFIG9: u32, pub DSCC_PPS_CONFIG10: u32, pub DSCC_PPS_CONFIG11: u32,
    pub DSCC_PPS_CONFIG12: u32, pub DSCC_PPS_CONFIG13: u32, pub DSCC_PPS_CONFIG14: u32, pub DSCC_PPS_CONFIG15: u32,
    pub DSCC_PPS_CONFIG16: u32, pub DSCC_PPS_CONFIG17: u32, pub DSCC_PPS_CONFIG18: u32, pub DSCC_PPS_CONFIG19: u32,
    pub DSCC_PPS_CONFIG20: u32, pub DSCC_PPS_CONFIG21: u32, pub DSCC_PPS_CONFIG22: u32,
    pub DSCC_MEM_POWER_CONTROL: u32, pub DSCC_R_Y_SQUARED_ERROR_LOWER: u32, pub DSCC_R_Y_SQUARED_ERROR_UPPER: u32,
    pub DSCC_G_CB_SQUARED_ERROR_LOWER: u32, pub DSCC_G_CB_SQUARED_ERROR_UPPER: u32,
    pub DSCC_B_CR_SQUARED_ERROR_LOWER: u32, pub DSCC_B_CR_SQUARED_ERROR_UPPER: u32,
    pub DSCC_MAX_ABS_ERROR0: u32, pub DSCC_MAX_ABS_ERROR1: u32,
    pub DSCC_RATE_BUFFER0_MAX_FULLNESS_LEVEL: u32, pub DSCC_RATE_BUFFER1_MAX_FULLNESS_LEVEL: u32,
    pub DSCC_RATE_BUFFER2_MAX_FULLNESS_LEVEL: u32, pub DSCC_RATE_BUFFER3_MAX_FULLNESS_LEVEL: u32,
    pub DSCC_RATE_CONTROL_BUFFER0_MAX_FULLNESS_LEVEL: u32, pub DSCC_RATE_CONTROL_BUFFER1_MAX_FULLNESS_LEVEL: u32,
    pub DSCC_RATE_CONTROL_BUFFER2_MAX_FULLNESS_LEVEL: u32, pub DSCC_RATE_CONTROL_BUFFER3_MAX_FULLNESS_LEVEL: u32,
    pub DSCC_TEST_DEBUG_BUS_ROTATE: u32, pub DSCCIF_CONFIG0: u32, pub DSCCIF_CONFIG1: u32,
    pub DSCRM_DSC_FORWARD_CONFIG: u32,
}

#[repr(C)] pub struct dcn20_dsc_shift { dsc_fields!(u8); }
#[repr(C)] pub struct dcn20_dsc_mask { dsc_fields!(u32); }

#[repr(C)] pub enum dsc_pixel_format { DSC_PIXFMT_RGB, DSC_PIXFMT_YCBCR444, DSC_PIXFMT_SIMPLE_YCBCR422, DSC_PIXFMT_NATIVE_YCBCR422, DSC_PIXFMT_NATIVE_YCBCR420, DSC_PIXFMT_UNKNOWN }

#[repr(C)] pub struct dsc_reg_values {
    pub pps: drm_dsc_config, pub dsc_clock_enable: u32, pub dsc_clock_gating_disable: u32,
    pub underflow_recovery_en: u32, pub underflow_occurred_int_en: u32, pub underflow_occurred_status: u32,
    pub pixel_format: dsc_pixel_format, pub ich_reset_at_eol: u32, pub alternate_ich_encoding_en: u32,
    pub num_slices_h: u32, pub num_slices_v: u32, pub rc_buffer_model_size: u32, pub disable_ich: u32,
    pub bpp_x32: u32, pub dsc_dbg_en: u32, pub rc_buffer_model_overflow_int_en: [u32; 4],
}

#[repr(C)] pub struct dcn20_dsc {
    pub base: display_stream_compressor, pub dsc_regs: *const dcn20_dsc_registers,
    pub dsc_shift: *const dcn20_dsc_shift, pub dsc_mask: *const dcn20_dsc_mask,
    pub reg_vals: dsc_reg_values, pub max_image_width: i32,
}

extern "C" {
    pub fn dsc_config_log(dsc: *mut display_stream_compressor, config: *const dsc_config);
    pub fn dsc_log_pps(dsc: *mut display_stream_compressor, pps: *mut drm_dsc_config);
    pub fn dsc_override_rc_params(rc: *mut rc_params, override_: *const dc_dsc_rc_params_override);
    pub fn dsc_prepare_config(dsc_cfg: *const dsc_config, dsc_reg_vals: *mut dsc_reg_values, dsc_optc_cfg: *mut dsc_optc_config) -> bool;
    pub fn dsc_dc_pixel_encoding_to_dsc_pixel_format(dc_pix_enc: dc_pixel_encoding, is_ycbcr422_simple: bool) -> dsc_pixel_format;
    pub fn dsc_dc_color_depth_to_dsc_bits_per_comp(dc_color_depth: dc_color_depth) -> dsc_bits_per_comp;
    pub fn dsc_init_reg_values(reg_vals: *mut dsc_reg_values);
    pub fn dsc_update_from_dsc_parameters(reg_vals: *mut dsc_reg_values, dsc_params: *const dsc_parameters);
    pub fn dsc2_construct(dsc: *mut dcn20_dsc, ctx: *mut dc_context, inst: i32, dsc_regs: *const dcn20_dsc_registers, dsc_shift: *const dcn20_dsc_shift, dsc_mask: *const dcn20_dsc_mask);
    pub fn dsc2_get_enc_caps(dsc_enc_caps: *mut dsc_enc_caps, pixel_clock_100Hz: i32);
    pub fn dsc2_get_packed_pps(dsc: *mut display_stream_compressor, dsc_cfg: *const dsc_config, dsc_packed_pps: *mut u8) -> bool;
    pub fn dsc2_read_state(dsc: *mut display_stream_compressor, s: *mut dcn_dsc_state);
    pub fn dsc2_read_reg_state(dsc: *mut display_stream_compressor, dccg_reg_state: *mut dcn_dsc_reg_state);
    pub fn dsc2_validate_stream(dsc: *mut display_stream_compressor, dsc_cfg: *const dsc_config) -> bool;
    pub fn dsc2_set_config(dsc: *mut display_stream_compressor, dsc_cfg: *const dsc_config, dsc_optc_cfg: *mut dsc_optc_config);
    pub fn dsc2_enable(dsc: *mut display_stream_compressor, opp_pipe: i32);
    pub fn dsc2_disable(dsc: *mut display_stream_compressor);
    pub fn dsc2_disconnect(dsc: *mut display_stream_compressor);
    pub fn dsc2_wait_disconnect_pending_clear(dsc: *mut display_stream_compressor);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
