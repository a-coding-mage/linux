/*
 * Rust translation of dcn20_mmhubbub.h.  The register-list and field-list
 * preprocessor interfaces are retained below as declarative macros; the
 * referenced register symbols and external types are supplied by dependants.
 */

#[macro_export]
macro_rules! TO_DCN20_MMHUBBUB {
    ($mcif_wb_base:expr) => { container_of!($mcif_wb_base, dcn20_mmhubbub, base) };
}

#[macro_export]
macro_rules! MCIF_WB_COMMON_REG_LIST_DCN2_0 {
    ($inst:expr) => {
        SRI!(MCIF_WB_BUFMGR_SW_CONTROL, MCIF_WB, $inst),
        SRI!(MCIF_WB_BUFMGR_CUR_LINE_R, MCIF_WB, $inst),
        SRI!(MCIF_WB_BUFMGR_STATUS, MCIF_WB, $inst),
        SRI!(MCIF_WB_BUF_PITCH, MCIF_WB, $inst),
        SRI!(MCIF_WB_BUF_1_STATUS, MCIF_WB, $inst),
        SRI!(MCIF_WB_BUF_1_STATUS2, MCIF_WB, $inst),
        SRI!(MCIF_WB_BUF_2_STATUS, MCIF_WB, $inst),
        SRI!(MCIF_WB_BUF_2_STATUS2, MCIF_WB, $inst),
        SRI!(MCIF_WB_BUF_3_STATUS, MCIF_WB, $inst),
        SRI!(MCIF_WB_BUF_3_STATUS2, MCIF_WB, $inst),
        SRI!(MCIF_WB_BUF_4_STATUS, MCIF_WB, $inst),
        SRI!(MCIF_WB_BUF_4_STATUS2, MCIF_WB, $inst),
        SRI!(MCIF_WB_ARBITRATION_CONTROL, MCIF_WB, $inst),
        SRI!(MCIF_WB_SCLK_CHANGE, MCIF_WB, $inst),
        SRI!(MCIF_WB_TEST_DEBUG_INDEX, MCIF_WB, $inst),
        SRI!(MCIF_WB_TEST_DEBUG_DATA, MCIF_WB, $inst),
        SRI!(MCIF_WB_BUF_1_ADDR_Y, MCIF_WB, $inst),
        SRI!(MCIF_WB_BUF_1_ADDR_Y_OFFSET, MCIF_WB, $inst),
        SRI!(MCIF_WB_BUF_1_ADDR_C, MCIF_WB, $inst),
        SRI!(MCIF_WB_BUF_1_ADDR_C_OFFSET, MCIF_WB, $inst),
        SRI!(MCIF_WB_BUF_2_ADDR_Y, MCIF_WB, $inst),
        SRI!(MCIF_WB_BUF_2_ADDR_Y_OFFSET, MCIF_WB, $inst),
        SRI!(MCIF_WB_BUF_2_ADDR_C, MCIF_WB, $inst),
        SRI!(MCIF_WB_BUF_2_ADDR_C_OFFSET, MCIF_WB, $inst),
        SRI!(MCIF_WB_BUF_3_ADDR_Y, MCIF_WB, $inst),
        SRI!(MCIF_WB_BUF_3_ADDR_Y_OFFSET, MCIF_WB, $inst),
        SRI!(MCIF_WB_BUF_3_ADDR_C, MCIF_WB, $inst),
        SRI!(MCIF_WB_BUF_3_ADDR_C_OFFSET, MCIF_WB, $inst),
        SRI!(MCIF_WB_BUF_4_ADDR_Y, MCIF_WB, $inst),
        SRI!(MCIF_WB_BUF_4_ADDR_Y_OFFSET, MCIF_WB, $inst),
        SRI!(MCIF_WB_BUF_4_ADDR_C, MCIF_WB, $inst),
        SRI!(MCIF_WB_BUF_4_ADDR_C_OFFSET, MCIF_WB, $inst),
        SRI!(MCIF_WB_BUFMGR_VCE_CONTROL, MCIF_WB, $inst),
        SRI!(MCIF_WB_NB_PSTATE_LATENCY_WATERMARK, MCIF_WB, $inst),
        SRI!(MCIF_WB_NB_PSTATE_CONTROL, MCIF_WB, $inst),
        SRI!(MCIF_WB_WATERMARK, MCIF_WB, $inst),
        SRI!(MCIF_WB_CLOCK_GATER_CONTROL, MCIF_WB, $inst),
        SRI!(MCIF_WB_WARM_UP_CNTL, MCIF_WB, $inst),
        SRI!(MCIF_WB_SELF_REFRESH_CONTROL, MCIF_WB, $inst),
        SRI!(MULTI_LEVEL_QOS_CTRL, MCIF_WB, $inst),
        SRI!(MCIF_WB_SECURITY_LEVEL, MCIF_WB, $inst),
        SRI!(MCIF_WB_BUF_LUMA_SIZE, MCIF_WB, $inst),
        SRI!(MCIF_WB_BUF_CHROMA_SIZE, MCIF_WB, $inst),
        SRI!(MCIF_WB_BUF_1_ADDR_Y_HIGH, MCIF_WB, $inst),
        SRI!(MCIF_WB_BUF_1_ADDR_C_HIGH, MCIF_WB, $inst),
        SRI!(MCIF_WB_BUF_2_ADDR_Y_HIGH, MCIF_WB, $inst),
        SRI!(MCIF_WB_BUF_2_ADDR_C_HIGH, MCIF_WB, $inst),
        SRI!(MCIF_WB_BUF_3_ADDR_Y_HIGH, MCIF_WB, $inst),
        SRI!(MCIF_WB_BUF_3_ADDR_C_HIGH, MCIF_WB, $inst),
        SRI!(MCIF_WB_BUF_4_ADDR_Y_HIGH, MCIF_WB, $inst),
        SRI!(MCIF_WB_BUF_4_ADDR_C_HIGH, MCIF_WB, $inst),
        SRI!(MCIF_WB_BUF_1_RESOLUTION, MCIF_WB, $inst),
        SRI!(MCIF_WB_BUF_2_RESOLUTION, MCIF_WB, $inst),
        SRI!(MCIF_WB_BUF_3_RESOLUTION, MCIF_WB, $inst),
        SRI!(MCIF_WB_BUF_4_RESOLUTION, MCIF_WB, $inst),
        SRI!(SMU_WM_CONTROL, WBIF, $inst)
    };
}

#[repr(C)]
pub struct dcn20_mmhubbub_registers {
    pub MCIF_WB_BUFMGR_SW_CONTROL: u32, pub MCIF_WB_BUFMGR_CUR_LINE_R: u32,
    pub MCIF_WB_BUFMGR_STATUS: u32, pub MCIF_WB_BUF_PITCH: u32,
    pub MCIF_WB_BUF_1_STATUS: u32, pub MCIF_WB_BUF_1_STATUS2: u32,
    pub MCIF_WB_BUF_2_STATUS: u32, pub MCIF_WB_BUF_2_STATUS2: u32,
    pub MCIF_WB_BUF_3_STATUS: u32, pub MCIF_WB_BUF_3_STATUS2: u32,
    pub MCIF_WB_BUF_4_STATUS: u32, pub MCIF_WB_BUF_4_STATUS2: u32,
    pub MCIF_WB_ARBITRATION_CONTROL: u32, pub MCIF_WB_SCLK_CHANGE: u32,
    pub MCIF_WB_TEST_DEBUG_INDEX: u32, pub MCIF_WB_TEST_DEBUG_DATA: u32,
    pub MCIF_WB_BUF_1_ADDR_Y: u32, pub MCIF_WB_BUF_1_ADDR_Y_OFFSET: u32,
    pub MCIF_WB_BUF_1_ADDR_C: u32, pub MCIF_WB_BUF_1_ADDR_C_OFFSET: u32,
    pub MCIF_WB_BUF_2_ADDR_Y: u32, pub MCIF_WB_BUF_2_ADDR_Y_OFFSET: u32,
    pub MCIF_WB_BUF_2_ADDR_C: u32, pub MCIF_WB_BUF_2_ADDR_C_OFFSET: u32,
    pub MCIF_WB_BUF_3_ADDR_Y: u32, pub MCIF_WB_BUF_3_ADDR_Y_OFFSET: u32,
    pub MCIF_WB_BUF_3_ADDR_C: u32, pub MCIF_WB_BUF_3_ADDR_C_OFFSET: u32,
    pub MCIF_WB_BUF_4_ADDR_Y: u32, pub MCIF_WB_BUF_4_ADDR_Y_OFFSET: u32,
    pub MCIF_WB_BUF_4_ADDR_C: u32, pub MCIF_WB_BUF_4_ADDR_C_OFFSET: u32,
    pub MCIF_WB_BUFMGR_VCE_CONTROL: u32, pub MCIF_WB_NB_PSTATE_LATENCY_WATERMARK: u32,
    pub MCIF_WB_NB_PSTATE_CONTROL: u32, pub MCIF_WB_WATERMARK: u32,
    pub MCIF_WB_CLOCK_GATER_CONTROL: u32, pub MCIF_WB_WARM_UP_CNTL: u32,
    pub MCIF_WB_SELF_REFRESH_CONTROL: u32, pub MULTI_LEVEL_QOS_CTRL: u32,
    pub MCIF_WB_SECURITY_LEVEL: u32, pub MCIF_WB_BUF_LUMA_SIZE: u32,
    pub MCIF_WB_BUF_CHROMA_SIZE: u32, pub MCIF_WB_BUF_1_ADDR_Y_HIGH: u32,
    pub MCIF_WB_BUF_1_ADDR_C_HIGH: u32, pub MCIF_WB_BUF_2_ADDR_Y_HIGH: u32,
    pub MCIF_WB_BUF_2_ADDR_C_HIGH: u32, pub MCIF_WB_BUF_3_ADDR_Y_HIGH: u32,
    pub MCIF_WB_BUF_3_ADDR_C_HIGH: u32, pub MCIF_WB_BUF_4_ADDR_Y_HIGH: u32,
    pub MCIF_WB_BUF_4_ADDR_C_HIGH: u32, pub MCIF_WB_BUF_1_RESOLUTION: u32,
    pub MCIF_WB_BUF_2_RESOLUTION: u32, pub MCIF_WB_BUF_3_RESOLUTION: u32,
    pub MCIF_WB_BUF_4_RESOLUTION: u32, pub SMU_WM_CONTROL: u32,
}

#[repr(C)] pub struct dcn20_mmhubbub_mask { pub fields: [u32; 148] }
#[repr(C)] pub struct dcn20_mmhubbub_shift { pub fields: [u8; 148] }

#[repr(C)]
pub struct dcn20_mmhubbub {
    pub base: mcif_wb,
    pub mcif_wb_regs: *const dcn20_mmhubbub_registers,
    pub mcif_wb_shift: *const dcn20_mmhubbub_shift,
    pub mcif_wb_mask: *const dcn20_mmhubbub_mask,
}

extern "C" {
    pub fn mmhubbub2_config_mcif_irq(mcif_wb: *mut mcif_wb, params: *mut mcif_irq_params);
    pub fn mmhubbub2_enable_mcif(mcif_wb: *mut mcif_wb);
    pub fn mmhubbub2_disable_mcif(mcif_wb: *mut mcif_wb);
    pub fn mcifwb2_dump_frame(mcif_wb: *mut mcif_wb, mcif_params: *mut mcif_buf_params,
        out_format: dwb_scaler_mode, dest_width: c_uint, dest_height: c_uint,
        dump_info: *mut mcif_wb_frame_dump_info, luma_buffer: *mut c_uchar,
        chroma_buffer: *mut c_uchar, dest_luma_buffer: *mut c_uchar,
        dest_chroma_buffer: *mut c_uchar);
    pub fn dcn20_mmhubbub_construct(mcif_wb20: *mut dcn20_mmhubbub, ctx: *mut dc_context,
        mcif_wb_regs: *const dcn20_mmhubbub_registers,
        mcif_wb_shift: *const dcn20_mmhubbub_shift,
        mcif_wb_mask: *const dcn20_mmhubbub_mask, inst: c_int);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
