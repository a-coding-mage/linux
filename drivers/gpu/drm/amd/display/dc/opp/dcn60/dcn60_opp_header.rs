/* SPDX-License-Identifier: MIT */
/* Copyright 2025 Advanced Micro Devices, Inc. */

// Dependencies supplied by the DCN20/DCN35 OPP headers are intentionally
// referenced but not implemented here.

// Separate initializer list is required for DCN6 because
// OPPBUF_3D_PARAMETERS_0::OPPBUF_3D_VACT_SPACE2_SIZE is removed in DCN6.
// Keep the field, but leave it uninitialized.
#[macro_export]
macro_rules! OPP_MASK_SH_LIST_DCN60 {
    ($mask_sh:ident) => {
        OPP_DPG_MASK_SH_LIST!($mask_sh),
        OPP_SF!(FMT0_FMT_BIT_DEPTH_CONTROL, FMT_TRUNCATE_EN, $mask_sh),
        OPP_SF!(FMT0_FMT_BIT_DEPTH_CONTROL, FMT_TRUNCATE_DEPTH, $mask_sh),
        OPP_SF!(FMT0_FMT_BIT_DEPTH_CONTROL, FMT_TRUNCATE_MODE, $mask_sh),
        OPP_SF!(FMT0_FMT_BIT_DEPTH_CONTROL, FMT_SPATIAL_DITHER_EN, $mask_sh),
        OPP_SF!(FMT0_FMT_BIT_DEPTH_CONTROL, FMT_SPATIAL_DITHER_MODE, $mask_sh),
        OPP_SF!(FMT0_FMT_BIT_DEPTH_CONTROL, FMT_SPATIAL_DITHER_DEPTH, $mask_sh),
        OPP_SF!(FMT0_FMT_BIT_DEPTH_CONTROL, FMT_TEMPORAL_DITHER_EN, $mask_sh),
        OPP_SF!(FMT0_FMT_BIT_DEPTH_CONTROL, FMT_HIGHPASS_RANDOM_ENABLE, $mask_sh),
        OPP_SF!(FMT0_FMT_BIT_DEPTH_CONTROL, FMT_FRAME_RANDOM_ENABLE, $mask_sh),
        OPP_SF!(FMT0_FMT_BIT_DEPTH_CONTROL, FMT_RGB_RANDOM_ENABLE, $mask_sh),
        OPP_SF!(FMT0_FMT_CONTROL, FMT_SPATIAL_DITHER_FRAME_COUNTER_MAX, $mask_sh),
        OPP_SF!(FMT0_FMT_CONTROL, FMT_SPATIAL_DITHER_FRAME_COUNTER_BIT_SWAP, $mask_sh),
        OPP_SF!(FMT0_FMT_CONTROL, FMT_PIXEL_ENCODING, $mask_sh),
        OPP_SF!(FMT0_FMT_CONTROL, FMT_SUBSAMPLING_MODE, $mask_sh),
        OPP_SF!(FMT0_FMT_CONTROL, FMT_CBCR_BIT_REDUCTION_BYPASS, $mask_sh),
        OPP_SF!(FMT0_FMT_CONTROL, FMT_STEREOSYNC_OVERRIDE, $mask_sh),
        OPP_SF!(FMT0_FMT_DITHER_RAND_R_SEED, FMT_RAND_R_SEED, $mask_sh),
        OPP_SF!(FMT0_FMT_DITHER_RAND_G_SEED, FMT_RAND_G_SEED, $mask_sh),
        OPP_SF!(FMT0_FMT_DITHER_RAND_B_SEED, FMT_RAND_B_SEED, $mask_sh),
        OPP_SF!(FMT0_FMT_CLAMP_CNTL, FMT_CLAMP_DATA_EN, $mask_sh),
        OPP_SF!(FMT0_FMT_CLAMP_CNTL, FMT_CLAMP_COLOR_FORMAT, $mask_sh),
        OPP_SF!(FMT0_FMT_DYNAMIC_EXP_CNTL, FMT_DYNAMIC_EXP_EN, $mask_sh),
        OPP_SF!(FMT0_FMT_DYNAMIC_EXP_CNTL, FMT_DYNAMIC_EXP_MODE, $mask_sh),
        OPP_SF!(FMT0_FMT_MAP420_MEMORY_CONTROL, FMT_MAP420MEM_PWR_FORCE, $mask_sh),
        OPP_SF!(OPPBUF0_OPPBUF_CONTROL, OPPBUF_ACTIVE_WIDTH, $mask_sh),
        OPP_SF!(OPPBUF0_OPPBUF_CONTROL, OPPBUF_PIXEL_REPETITION, $mask_sh),
        OPP_SF!(OPPBUF0_OPPBUF_3D_PARAMETERS_0, OPPBUF_3D_VACT_SPACE1_SIZE, $mask_sh),
        OPP_SF!(OPP_PIPE0_OPP_PIPE_CONTROL, OPP_PIPE_CLOCK_EN, $mask_sh),
        OPP_SF!(OPPBUF0_OPPBUF_CONTROL, OPPBUF_DISPLAY_SEGMENTATION, $mask_sh),
        OPP_SF!(OPPBUF0_OPPBUF_CONTROL, OPPBUF_OVERLAP_PIXEL_NUM, $mask_sh),
        OPP_SF!(FMT0_FMT_422_CONTROL, FMT_LEFT_EDGE_EXTRA_PIXEL_COUNT, $mask_sh),
        OPP_SF!(OPP_TOP_CLK_CONTROL, OPP_FGCG_REP_DIS, $mask_sh)
    };
}

#[repr(C)]
pub struct dcn60_opp_registers {
    pub fields: OPP_REG_VARIABLE_LIST_DCN3_5,
}

#[repr(C)]
pub struct dcn60_opp_shift {
    pub fields: OPP_DCN35_REG_FIELD_LIST!(u8),
}

#[repr(C)]
pub struct dcn60_opp_mask {
    pub fields: OPP_DCN35_REG_FIELD_LIST!(u32),
}

extern "C" {
    pub fn dcn60_opp_construct(
        oppn20: *mut dcn20_opp,
        ctx: *mut dc_context,
        inst: u32,
        regs: *const dcn60_opp_registers,
        opp_shift: *const dcn60_opp_shift,
        opp_mask: *const dcn60_opp_mask,
    );
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
