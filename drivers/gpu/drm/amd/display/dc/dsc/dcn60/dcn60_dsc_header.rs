// SPDX-License-Identifier: MIT
//
// Copyright 2024 Advanced Micro Devices, Inc.

// Dependencies supplied by the surrounding translation unit:
// dsc.h, dsc/dscc_types.h, dcn20/dcn20_dsc.h, dcn401/dcn401_dsc.h,
// and drm/display/drm_dsc.h.

macro_rules! TO_DCN60_DSC {
    ($dsc:expr) => {
        container_of!($dsc, dcn60_dsc, base)
    };
}

/* Register field list for DCN 6.0.  DSC_SF/DSC2_SF are supplied externally. */
macro_rules! DSC_REG_LIST_SH_MASK_DCN60 {
    ($mask_sh:expr) => {
        DSC_SF!(DSC_TOP0_DSC_TOP_CONTROL, DSC_CLOCK_EN, $mask_sh),
        DSC_SF!(DSC_TOP0_DSC_TOP_CONTROL, DSC_DISPCLK_R_GATE_DIS, $mask_sh),
        DSC_SF!(DSC_TOP0_DSC_TOP_CONTROL, DSC_DSCCLK_R_GATE_DIS, $mask_sh),
        DSC_SF!(DSC_TOP0_DSC_TOP_CONTROL, DSC_FGCG_REP_DIS, $mask_sh),
        DSC_SF!(DSC_TOP0_DSC_DEBUG_CONTROL, DSC_DBG_EN, $mask_sh),
        DSC_SF!(DSC_TOP0_DSC_DEBUG_CONTROL, DSC_TEST_CLOCK_MUX_SEL, $mask_sh),
        DSC_SF!(DSCC0_DSCC_CONFIG0, ICH_RESET_AT_END_OF_LINE, $mask_sh),
        DSC_SF!(DSCC0_DSCC_CONFIG0, NUMBER_OF_SLICES_PER_LINE, $mask_sh),
        DSC_SF!(DSCC0_DSCC_CONFIG0, ALTERNATE_ICH_ENCODING_EN, $mask_sh),
        DSC_SF!(DSCC0_DSCC_CONFIG0, NUMBER_OF_SLICES_IN_VERTICAL_DIRECTION, $mask_sh),
        DSC_SF!(DSCC0_DSCC_CONFIG1, DSCC_RATE_CONTROL_BUFFER_MODEL_SIZE, $mask_sh),
        DSC_SF!(DSCC0_DSCC_STATUS, DSCC_DOUBLE_BUFFER_REG_UPDATE_PENDING, $mask_sh),
        DSC2_SF!(DSCC0, DSCC_PPS_CONFIG0__BITS_PER_COMPONENT, $mask_sh),
        DSC_SF!(DSCC0_DSCC_PPS_CONFIG1, BITS_PER_PIXEL, $mask_sh),
        DSC_SF!(DSCC0_DSCC_PPS_CONFIG1, VBR_ENABLE, $mask_sh),
        DSC_SF!(DSCC0_DSCC_PPS_CONFIG1, SIMPLE_422, $mask_sh),
        DSC_SF!(DSCC0_DSCC_PPS_CONFIG1, CONVERT_RGB, $mask_sh),
        DSC_SF!(DSCC0_DSCC_PPS_CONFIG1, BLOCK_PRED_ENABLE, $mask_sh),
        DSC_SF!(DSCC0_DSCC_PPS_CONFIG1, NATIVE_422, $mask_sh),
        DSC_SF!(DSCC0_DSCC_PPS_CONFIG1, NATIVE_420, $mask_sh),
        DSC_SF!(DSCC0_DSCC_PPS_CONFIG1, CHUNK_SIZE, $mask_sh),
        DSC_SF!(DSCC0_DSCC_PPS_CONFIG2, PIC_WIDTH, $mask_sh),
        DSC_SF!(DSCC0_DSCC_PPS_CONFIG2, PIC_HEIGHT, $mask_sh),
        DSC_SF!(DSCC0_DSCC_PPS_CONFIG3, SLICE_WIDTH, $mask_sh),
        DSC_SF!(DSCC0_DSCC_PPS_CONFIG3, SLICE_HEIGHT, $mask_sh),
        DSC_SF!(DSCC0_DSCC_PPS_CONFIG4, INITIAL_XMIT_DELAY, $mask_sh),
        DSC_SF!(DSCC0_DSCC_PPS_CONFIG4, INITIAL_DEC_DELAY, $mask_sh),
        DSC_SF!(DSCC0_DSCC_PPS_CONFIG5, INITIAL_SCALE_VALUE, $mask_sh),
        DSC_SF!(DSCC0_DSCC_PPS_CONFIG5, SCALE_INCREMENT_INTERVAL, $mask_sh),
        DSC_SF!(DSCC0_DSCC_PPS_CONFIG6, SCALE_DECREMENT_INTERVAL, $mask_sh),
        DSC_SF!(DSCC0_DSCC_PPS_CONFIG6, FIRST_LINE_BPG_OFFSET, $mask_sh),
        DSC_SF!(DSCC0_DSCC_PPS_CONFIG6, SECOND_LINE_BPG_OFFSET, $mask_sh),
        DSC_SF!(DSCC0_DSCC_PPS_CONFIG7, NFL_BPG_OFFSET, $mask_sh),
        DSC_SF!(DSCC0_DSCC_PPS_CONFIG7, SLICE_BPG_OFFSET, $mask_sh),
        DSC_SF!(DSCC0_DSCC_PPS_CONFIG8, NSL_BPG_OFFSET, $mask_sh),
        DSC_SF!(DSCC0_DSCC_PPS_CONFIG8, SECOND_LINE_OFFSET_ADJ, $mask_sh),
        DSC_SF!(DSCC0_DSCC_PPS_CONFIG9, INITIAL_OFFSET, $mask_sh),
        DSC_SF!(DSCC0_DSCC_PPS_CONFIG9, FINAL_OFFSET, $mask_sh),
        DSC_SF!(DSCC0_DSCC_PPS_CONFIG10, FLATNESS_MIN_QP, $mask_sh),
        DSC_SF!(DSCC0_DSCC_PPS_CONFIG10, FLATNESS_MAX_QP, $mask_sh),
        DSC_SF!(DSCC0_DSCC_PPS_CONFIG10, RC_MODEL_SIZE, $mask_sh),
        DSC_SF!(DSCC0_DSCC_MEM_POWER_CONTROL0, DSCC_DEFAULT_MEM_LOW_POWER_STATE, $mask_sh),
        DSC_SF!(DSCC0_DSCC_MEM_POWER_CONTROL0, DSCC_MEM_PWR_FORCE, $mask_sh),
        DSC_SF!(DSCC0_DSCC_MEM_POWER_CONTROL0, DSCC_MEM_PWR_DIS, $mask_sh),
        DSC_SF!(DSCC0_DSCC_MEM_POWER_CONTROL0, DSCC_MEM_PWR_STATE, $mask_sh)
    };
}

macro_rules! DSC_FIELD_LIST_DCN60 {
    ($type:ty) => {
        DSC_FIELD_LIST_DCN401!($type);
        $type DSCC_RATE_CONTROL_BUFFER_MODEL_OVERFLOW_OCCURRED_INT_EN4;
        $type DSCC_RATE_CONTROL_BUFFER_MODEL_OVERFLOW_OCCURRED_INT_EN5;
        $type DSCC_RATE_CONTROL_BUFFER_MODEL_OVERFLOW_OCCURRED_INT_EN6;
        $type DSCC_RATE_CONTROL_BUFFER_MODEL_OVERFLOW_OCCURRED_INT_EN7;
    };
}

#[repr(C)]
pub struct dcn60_dsc_shift {
    pub fields: DSC_FIELD_LIST_DCN60!(u8),
}

#[repr(C)]
pub struct dcn60_dsc_mask {
    pub fields: DSC_FIELD_LIST_DCN60!(u32),
}

#[repr(C)]
pub struct dsc60_reg_values {
    /* PPS registers */
    pub pps: drm_dsc_config,
    /* Additional registers */
    pub dsc_clock_enable: u32,
    pub dsc_clock_gating_disable: u32,
    pub underflow_recovery_en: u32,
    pub underflow_occurred_int_en: u32,
    pub underflow_occurred_status: u32,
    pub pixel_format: dsc_pixel_format,
    pub ich_reset_at_eol: u32,
    pub alternate_ich_encoding_en: u32,
    pub num_slices_h: u32,
    pub num_slices_v: u32,
    pub rc_buffer_model_size: u32,
    pub disable_ich: u32,
    pub bpp_x32: u32,
    pub dsc_dbg_en: u32,
    pub rc_buffer_model_overflow_int_en: [u32; 8],
}

#[repr(C)]
pub struct dcn60_dsc {
    pub base: display_stream_compressor,
    pub dsc_regs: *const dcn401_dsc_registers,
    pub dsc_shift: *const dcn60_dsc_shift,
    pub dsc_mask: *const dcn60_dsc_mask,
    pub reg_vals: dsc60_reg_values,
    pub max_image_width: i32,
}

extern "C" {
    pub fn dsc60_construct(
        dsc: *mut dcn60_dsc,
        ctx: *mut dc_context,
        inst: i32,
        dsc_regs: *const dcn401_dsc_registers,
        dsc_shift: *const dcn60_dsc_shift,
        dsc_mask: *const dcn60_dsc_mask,
    );
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
