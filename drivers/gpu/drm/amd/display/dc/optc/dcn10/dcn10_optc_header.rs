/* Faithful source-level Rust header translation of dcn10_optc.h. */

// The C preprocessor register and field-list macros depend on definitions from
// optc.h and the generated register headers; their names and intent are kept.
macro_rules! DCN10TG_FROM_TG { ($tg:expr) => { $tg }; }
macro_rules! TG_COMMON_REG_LIST_DCN { ($inst:expr) => { () }; }
macro_rules! TG_COMMON_REG_LIST_DCN1_0 { ($inst:expr) => { () }; }
macro_rules! TG_COMMON_MASK_SH_LIST_DCN { ($mask_sh:expr) => { () }; }
macro_rules! TG_COMMON_MASK_SH_LIST_DCN1_0 { ($mask_sh:expr) => { () }; }
macro_rules! TG_REG_FIELD_LIST_DCN1_0 { ($ty:ty) => { () }; }
macro_rules! V_TOTAL_REGS { ($ty:ty) => { () }; }
macro_rules! TG_REG_FIELD_LIST { ($ty:ty) => { () }; }
macro_rules! TG_REG_FIELD_LIST_DCN2_0 { ($ty:ty) => { () }; }
macro_rules! TG_REG_FIELD_LIST_DCN3_2 { ($ty:ty) => { () }; }
macro_rules! TG_REG_FIELD_LIST_DCN3_5 { ($ty:ty) => { () }; }
macro_rules! TG_REG_FIELD_LIST_DCN3_6 { ($ty:ty) => { () }; }
macro_rules! TG_REG_FIELD_LIST_DCN401 { ($ty:ty) => { () }; }
macro_rules! TG_REG_FIELD_LIST_DCN42 { ($ty:ty) => { () }; }

#[repr(C)]
pub struct dcn_optc_registers {
    pub OTG_GLOBAL_CONTROL1: u32,
    pub OTG_GLOBAL_CONTROL2: u32,
    pub OTG_VERT_SYNC_CONTROL: u32,
    pub OTG_MASTER_UPDATE_MODE: u32,
    pub OTG_GSL_CONTROL: u32,
    pub OTG_VSTARTUP_PARAM: u32,
    pub OTG_VUPDATE_PARAM: u32,
    pub OTG_VREADY_PARAM: u32,
    pub OTG_BLANK_CONTROL: u32,
    pub OTG_MASTER_UPDATE_LOCK: u32,
    pub OTG_GLOBAL_CONTROL0: u32,
    pub OTG_DOUBLE_BUFFER_CONTROL: u32,
    pub OTG_H_TOTAL: u32,
    pub OTG_H_BLANK_START_END: u32,
    pub OTG_H_SYNC_A: u32,
    pub OTG_H_SYNC_A_CNTL: u32,
    pub OTG_H_TIMING_CNTL: u32,
    pub OTG_V_TOTAL: u32,
    pub OTG_V_BLANK_START_END: u32,
    pub OTG_V_SYNC_A: u32,
    pub OTG_V_SYNC_A_CNTL: u32,
    pub OTG_INTERLACE_CONTROL: u32,
    pub OTG_CONTROL: u32,
    pub OTG_STEREO_CONTROL: u32,
    pub OTG_3D_STRUCTURE_CONTROL: u32,
    pub OTG_STEREO_STATUS: u32,
    pub OTG_V_TOTAL_MAX: u32,
    pub OTG_V_TOTAL_MID: u32,
    pub OTG_V_TOTAL_MIN: u32,
    pub OTG_V_TOTAL_CONTROL: u32,
    pub OTG_V_COUNT_STOP_CONTROL: u32,
    pub OTG_V_COUNT_STOP_CONTROL2: u32,
    pub OTG_TRIGA_CNTL: u32,
    pub OTG_TRIGA_MANUAL_TRIG: u32,
    pub OTG_MANUAL_FLOW_CONTROL: u32,
    pub OTG_FORCE_COUNT_NOW_CNTL: u32,
    pub OTG_STATIC_SCREEN_CONTROL: u32,
    pub OTG_STATUS_FRAME_COUNT: u32,
    pub OTG_STATUS: u32,
    pub OTG_STATUS_POSITION: u32,
    pub OTG_NOM_VERT_POSITION: u32,
    pub OTG_BLACK_COLOR: u32,
    pub OTG_TEST_PATTERN_PARAMETERS: u32,
    pub OTG_TEST_PATTERN_CONTROL: u32,
    pub OTG_TEST_PATTERN_COLOR: u32,
    pub OTG_CLOCK_CONTROL: u32,
    pub OTG_VERTICAL_INTERRUPT0_CONTROL: u32,
    pub OTG_VERTICAL_INTERRUPT0_POSITION: u32,
    pub OTG_VERTICAL_INTERRUPT1_CONTROL: u32,
    pub OTG_VERTICAL_INTERRUPT1_POSITION: u32,
    pub OTG_VERTICAL_INTERRUPT2_CONTROL: u32,
    pub OTG_VERTICAL_INTERRUPT2_POSITION: u32,
    pub OPTC_INPUT_CLOCK_CONTROL: u32,
    pub OPTC_DATA_SOURCE_SELECT: u32,
    pub OPTC_MEMORY_CONFIG: u32,
    pub OPTC_INPUT_GLOBAL_CONTROL: u32,
    pub CONTROL: u32,
    pub OTG_DRR_V_TOTAL_REACH_RANGE: u32,
}

#[repr(C)]
pub struct dcn_optc_shift { }

#[repr(C)]
pub struct dcn_optc_mask { }

extern "C" {
    pub fn dcn10_timing_generator_init(optc: *mut optc);
}

// `struct optc` is supplied by optc.h.

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
