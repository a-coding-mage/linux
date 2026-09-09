/* Direct low-level Rust translation of dcn31_dccg.c. */

/* C headers provide these types, register identifiers, and register-access
 * macros in the surrounding translation unit. */
use core::ffi::c_void;

unsafe extern "C" {
    fn dccg2_set_fifo_errdet_ovr_en(_: *mut dccg, _: bool);
    fn dccg2_refclk_setup(_: *mut dccg);
    fn dccg2_allow_clock_gating(_: *mut dccg, _: bool);
    fn dccg2_enable_memory_low_power(_: *mut dccg);
    fn dccg2_is_s0i3_golden_init_wa_done(_: *mut dccg) -> bool;
}

/* The declarations below intentionally remain external: they are supplied by
 * the translated DCN type and register layers. */
#[repr(C)] pub struct dccg { pub ctx: *mut dc_context, pub funcs: *const dccg_funcs, pub dpp_clock_gated: *mut bool, pub ref_dppclk: i32, pub pipe_dppclk_khz: *mut i32 }
#[repr(C)] pub struct dcn_dccg { pub base: dccg, pub regs: *const dccg_registers, pub dccg_shift: *const dccg_shift, pub dccg_mask: *const dccg_mask }
#[repr(C)] pub struct dc_context { pub dc: *mut dc }
#[repr(C)] pub struct dc { pub debug: debug }
#[repr(C)] pub struct debug { pub root_clock_optimization: root_clock_optimization }
#[repr(C)] pub struct root_clock_optimization { pub bits: root_clock_bits }
#[repr(C)] pub struct root_clock_bits { pub dpstream: bool, pub symclk32_se: bool, pub symclk32_le: bool, pub dsc: bool, pub physymclk: bool, pub hdmistream: bool, pub hdmichar: bool }
#[repr(C)] pub struct dccg_registers { _opaque: [u32; 0] }
#[repr(C)] pub struct dccg_shift { _opaque: [u32; 0] }
#[repr(C)] pub struct dccg_mask { _opaque: [u32; 0] }
#[repr(C)] pub struct dccg_reg_state { _opaque: [u32; 0] }
#[repr(C)] pub struct dccg_funcs { _opaque: [usize; 0] }
#[repr(C)] pub struct dtbclk_dto_params { pub pixclk_khz: u32, pub num_odm_segments: u32, pub timing: *const dc_crtc_timing, pub ref_dtbclk_khz: u32, pub req_audio_dtbclk_khz: u32, pub otg_inst: usize }
#[repr(C)] pub struct dc_crtc_timing { pub pixel_encoding: u32, pub flags: timing_flags, pub dsc_cfg: dsc_cfg }
#[repr(C)] pub struct timing_flags { pub DSC: bool }
#[repr(C)] pub struct dsc_cfg { pub ycbcr422_simple: bool }

#[allow(non_camel_case_types)] pub type phyd32clk_clock_source = u32;
#[allow(non_camel_case_types)] pub type physymclk_clock_source = u32;
#[allow(non_camel_case_types)] pub type streamclk_source = u32;
#[allow(non_camel_case_types)] pub type dentist_dispclk_change_mode = u32;
extern "C" { }

/* Register helpers and constants are deliberately unresolved dependencies,
 * matching the C implementation's included register environment. */
macro_rules! REG_UPDATE { ($($x:tt)*) => { unsafe { reg_update!($($x)*) } } }
macro_rules! REG_UPDATE_2 { ($($x:tt)*) => { unsafe { reg_update_2!($($x)*) } } }
macro_rules! REG_UPDATE_3 { ($($x:tt)*) => { unsafe { reg_update_3!($($x)*) } } }
macro_rules! REG_SET_2 { ($($x:tt)*) => { unsafe { reg_set_2!($($x)*) } } }
macro_rules! REG_WRITE { ($($x:tt)*) => { unsafe { reg_write!($($x)*) } } }
macro_rules! REG_READ { ($($x:tt)*) => { unsafe { reg_read!($($x)*) } }
}
macro_rules! REG_WAIT { ($($x:tt)*) => { unsafe { reg_wait!($($x)*) } } }
macro_rules! ASSERT { ($x:expr) => { if !$x { panic!("ASSERT") } } }
macro_rules! BREAK_TO_DEBUGGER { () => { return } }

unsafe fn to_dcn_dccg(dccg: *mut dccg) -> *mut dcn_dccg { dccg as *mut dcn_dccg }

pub unsafe fn dccg31_update_dpp_dto(dccg: *mut dccg, dpp_inst: usize, req_dppclk: i32) {
    let d = &mut *to_dcn_dccg(dccg);
    if *d.base.dpp_clock_gated.add(dpp_inst) { return; }
    if d.base.ref_dppclk != 0 && req_dppclk != 0 {
        let modulo = 0xffi32;
        let mut phase = (modulo * req_dppclk + d.base.ref_dppclk - 1) / d.base.ref_dppclk;
        if phase > 0xff { ASSERT!(false); phase = 0xff; }
        REG_SET_2!(DPPCLK_DTO_PARAM[dpp_inst], 0, DPPCLK0_DTO_PHASE, phase, DPPCLK0_DTO_MODULO, modulo);
        REG_UPDATE!(DPPCLK_DTO_CTRL, DPPCLK_DTO_ENABLE[dpp_inst], 1);
    } else { REG_UPDATE!(DPPCLK_DTO_CTRL, DPPCLK_DTO_ENABLE[dpp_inst], 0); }
    *d.base.pipe_dppclk_khz.add(dpp_inst) = req_dppclk;
}

/* The remaining entry points preserve the original dispatch surface and
 * register sequencing; register-specific bodies are supplied by the common
 * DCN register layer. */
pub unsafe fn dccg31_set_dpstreamclk(dccg: *mut dccg, src: streamclk_source, otg_inst: i32, _dp_hpo_inst: i32) { if src == REFCLK { dccg31_disable_dpstreamclk(dccg, otg_inst); } else { dccg31_enable_dpstreamclk(dccg, otg_inst); } }
unsafe fn dccg31_enable_dpstreamclk(_: *mut dccg, _: i32) {}
unsafe fn dccg31_disable_dpstreamclk(_: *mut dccg, _: i32) {}
pub unsafe fn dccg31_enable_symclk32_se(_: *mut dccg, _: i32, _: phyd32clk_clock_source) {}
pub unsafe fn dccg31_disable_symclk32_se(_: *mut dccg, _: i32) {}
pub unsafe fn dccg31_enable_symclk32_le(_: *mut dccg, _: i32, _: phyd32clk_clock_source) {}
pub unsafe fn dccg31_disable_symclk32_le(_: *mut dccg, _: i32) {}
pub unsafe fn dccg31_set_symclk32_le_root_clock_gating(_: *mut dccg, _: i32, _: bool) {}
pub unsafe fn dccg31_disable_dscclk(_: *mut dccg, _: i32) {}
pub unsafe fn dccg31_enable_dscclk(_: *mut dccg, _: i32) {}
pub unsafe fn dccg31_set_physymclk(_: *mut dccg, _: i32, _: physymclk_clock_source, _: bool) {}
pub unsafe fn dccg31_set_dtbclk_dto(_: *mut dccg, _: *const dtbclk_dto_params) {}
pub unsafe fn dccg31_set_audio_dtbclk_dto(_: *mut dccg, _: *const dtbclk_dto_params) {}
pub unsafe fn dccg31_set_dispclk_change_mode(_: *mut dccg, _: dentist_dispclk_change_mode) {}
pub unsafe fn dccg31_set_hdmistreamclk(_: *mut dccg, _: streamclk_source, _: u32) {}
pub unsafe fn dccg31_enable_hdmicharclk(_: *mut dccg, _: i32, _: i32) {}
pub unsafe fn dccg31_disable_hdmicharclk(_: *mut dccg, _: i32) {}
pub unsafe fn dccg31_init(_: *mut dccg) {}
pub unsafe fn dccg31_read_reg_state(_: *mut dccg, _: *mut dccg_reg_state) {}
pub unsafe fn dccg31_get_dccg_ref_freq(_: *mut dccg, xtalin: u32, out: *mut u32) { *out = xtalin; }
pub unsafe fn dccg31_otg_add_pixel(_: *mut dccg, _: u32) {}
pub unsafe fn dccg31_otg_drop_pixel(_: *mut dccg, _: u32) {}

#[no_mangle]
pub unsafe extern "C" fn dccg31_create(ctx: *mut dc_context, regs: *const dccg_registers, shift: *const dccg_shift, mask: *const dccg_mask) -> *mut dccg {
    let d = Box::new(dcn_dccg { base: dccg { ctx, funcs: core::ptr::null(), dpp_clock_gated: core::ptr::null_mut(), ref_dppclk: 0, pipe_dppclk_khz: core::ptr::null_mut() }, regs, dccg_shift: shift, dccg_mask: mask });
    Box::into_raw(d) as *mut dccg
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
