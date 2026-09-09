/* Rust translation of dcn10_dpp.h.  Register-list macros are retained as
 * token-oriented Rust macros because their identifiers are supplied by the
 * platform register-generation headers. */

// Dependency supplied by dpp.h in the original header.

pub const LB_TOTAL_NUMBER_OF_ENTRIES: u32 = 5124;
pub const LB_BITS_PER_ENTRY: u32 = 144;

#[macro_export]
macro_rules! TF_SF { ($reg:ident, $field:ident, $post_fix:ident) => { $field: $reg }; }
#[macro_export]
macro_rules! TF2_SF { ($reg:ident, $field:ident, $post_fix:ident) => { $field: $reg }; }
#[macro_export]
macro_rules! TO_DCN10_DPP { ($dpp:expr) => { $dpp as *mut dcn10_dpp }; }

#[repr(C)]
pub struct dcn_dpp_shift {
    pub fields: DppFieldList<u8>,
}

#[repr(C)]
pub struct dcn_dpp_mask {
    pub fields: DppFieldList<u32>,
}

// The C header expands these register/field lists with build-generated names.
// The generic representation preserves the declaration shape and integer width.
#[repr(C)]
pub struct DppFieldList<T> {
    pub values: *mut T,
}

#[repr(C)]
pub struct dcn_dpp_registers {
    pub registers: *mut u32,
}

#[repr(C)]
pub struct dcn10_dpp {
    pub base: dpp,
    pub tf_regs: *const dcn_dpp_registers,
    pub tf_shift: *const dcn_dpp_shift,
    pub tf_mask: *const dcn_dpp_mask,
    pub filter_v: *const u16,
    pub filter_h: *const u16,
    pub filter_v_c: *const u16,
    pub filter_h_c: *const u16,
    pub lb_pixel_depth_supported: i32,
    pub lb_memory_size: i32,
    pub lb_bits_per_entry: i32,
    pub is_write_to_ram_a_safe: bool,
    pub scl_data: scaler_data,
    pub pwl_data: pwl_params,
}

#[repr(C)]
pub struct dpp { pub _opaque: [u8; 0] }
#[repr(C)]
pub struct scaler_data { pub _opaque: [u8; 0] }
#[repr(C)]
pub struct pwl_params { pub _opaque: [u8; 0] }

#[repr(C)]
#[derive(Copy, Clone)]
pub enum dcn10_input_csc_select {
    INPUT_CSC_SELECT_BYPASS = 0,
    INPUT_CSC_SELECT_ICSC = 1,
    INPUT_CSC_SELECT_COMA = 2,
}

// External declarations from the original header.  Referenced C types remain
// external dependencies, as they do in the source translation unit.
extern "C" {
    pub fn dpp1_set_cursor_attributes(dpp_base: *mut dpp, cursor_attributes: *mut dc_cursor_attributes);
    pub fn dpp1_set_cursor_position(dpp_base: *mut dpp, pos: *const dc_cursor_position, param: *const dc_cursor_mi_param, width: u32, height: u32);
    pub fn dpp1_cnv_set_optional_cursor_attributes(dpp_base: *mut dpp, attr: *mut dpp_cursor_attributes);
    pub fn dpp1_dscl_is_lb_conf_valid(ceil_vratio: i32, num_partitions: i32, vtaps: i32) -> bool;
    pub fn dpp1_dscl_calc_lb_num_partitions(scl_data: *const scaler_data, lb_config: lb_memory_config, num_part_y: *mut i32, num_part_c: *mut i32);
    pub fn dpp1_degamma_ram_select(dpp_base: *mut dpp, use_ram_a: bool);
    pub fn dpp1_program_degamma_luta_settings(dpp_base: *mut dpp, params: *const pwl_params);
    pub fn dpp1_program_degamma_lutb_settings(dpp_base: *mut dpp, params: *const pwl_params);
    pub fn dpp_reset(dpp_base: *mut dpp);
    pub fn dpp1_full_bypass(dpp_base: *mut dpp);
    pub fn dpp1_set_hdr_multiplier(dpp_base: *mut dpp, multiplier: u32);
    pub fn dpp_force_disable_cursor(dpp_base: *mut dpp);
}

#[repr(C)] pub struct dc_cursor_attributes { pub _opaque: [u8; 0] }
#[repr(C)] pub struct dc_cursor_position { pub _opaque: [u8; 0] }
#[repr(C)] pub struct dc_cursor_mi_param { pub _opaque: [u8; 0] }
#[repr(C)] pub struct dpp_cursor_attributes { pub _opaque: [u8; 0] }
#[repr(C)] pub struct lb_memory_config { pub _opaque: [u8; 0] }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
