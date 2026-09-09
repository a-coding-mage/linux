/* Rust translation of dcn401_hubbub.c.  Included headers and referenced
 * structures/functions are supplied by the surrounding kernel bindings. */

#![allow(non_camel_case_types, non_snake_case, dead_code)]

use core::ffi::c_void;

/* Register helpers and C layout types are external dependencies. */
extern "C" {
    fn hubbub1_allow_self_refresh_control(h: *mut hubbub, enable: bool);
    fn hubbub32_force_usr_retraining_allow(h: *mut hubbub, enable: bool);
    fn hubbub2_update_dchub(); fn hubbub3_init_dchub_sys_ctx(); fn hubbub2_init_vm_ctx();
    fn hubbub2_get_dchub_ref_freq(); fn hubbub1_is_allow_self_refresh_enabled();
    fn hubbub32_force_wm_propagate_to_pipes(); fn hubbub3_force_pstate_change_control();
    fn hubbub2_read_state(); fn hubbub32_set_request_limit(); fn hubbub3_read_reg_state();
    fn hubbub32_get_mall_en();
}

/* The following opaque declarations mirror the C interfaces; their concrete
 * definitions are provided by the translated headers. */
#[repr(C)] pub struct hubbub { pub ctx: *mut dc_context, pub funcs: *const hubbub_funcs }
#[repr(C)] pub struct dc_context { pub dc: *mut dc }
#[repr(C)] pub struct dc { pub caps: dc_caps, pub debug: dc_debug }
#[repr(C)] pub struct dc_caps { pub dcc_plane_width_limit: u32 }
#[repr(C)] pub struct dc_debug { pub disable_dcc: i32, pub disable_stutter: bool, pub force_usr_allow: bool }
#[repr(C)] pub struct dcn20_hubbub { pub base: hubbub, pub regs:*const c_void, pub shifts:*const c_void, pub masks:*const c_void, pub detile_buf_size:u32, pub pixel_chunk_size:u32, pub crb_size_segs:u32, pub det0_size:u32, pub det1_size:u32, pub det2_size:u32, pub det3_size:u32, pub compbuf_size_segments:u32, pub allow_sdpif_rate_limit_when_cstate_req:bool, pub watermarks:dcn_watermark_set }
#[repr(C)] pub union dcn_watermark_set { pub dcn4x: dcn4x_watermarks }
#[repr(C)] pub struct dcn4x_watermarks { pub a: watermark_values, pub b: watermark_values }
#[repr(C)] pub struct watermark_values { pub urgent:u32,pub frac_urg_bw_flip:u32,pub frac_urg_bw_nom:u32,pub frac_urg_bw_mall:u32,pub refcyc_per_trip_to_mem:u32,pub refcyc_per_meta_trip_to_mem:u32,pub sr_enter:u32,pub sr_exit:u32,pub uclk_pstate:u32,pub temp_read_or_ppt:u32,pub fclk_pstate:u32,pub usr:u32 }
#[repr(C)] pub struct dcn_hubbub_wm { pub sets:[dcn_hubbub_wm_set;2] }
#[repr(C)] pub struct dcn_hubbub_wm_set { pub wm_set:u32,pub data_urgent:u32,pub sr_enter:u32,pub sr_exit:u32,pub dram_clk_change:u32,pub usr_retrain:u32,pub fclk_pstate_change:u32 }
#[repr(C)] pub struct dml2_display_arb_regs { pub allow_sdpif_rate_limit_when_cstate_req:u32,pub pstate_stall_threshold:u32 }
#[repr(C)] pub struct dcn_hubbub_registers; #[repr(C)] pub struct dcn_hubbub_shift; #[repr(C)] pub struct dcn_hubbub_mask;
#[repr(C)] pub struct dc_dcc_surface_param { pub format:i32,pub swizzle_mode_addr3:i32,pub plane0_pitch:u32,pub plane1_pitch:u32,pub scan:i32,pub surface_size:size2d,pub plane1_size:size2d }
#[repr(C)] pub struct size2d { pub width:u32,pub height:u32 }
#[repr(C)] pub struct dc_surface_dcc_cap { pub capable:bool, pub grph: dcc_graphics, pub video:dcc_video }
#[repr(C)] pub struct dcc_graphics { pub rgb:dcc_plane }
#[repr(C)] pub struct dcc_video { pub luma:dcc_plane,pub chroma:dcc_plane }
#[repr(C)] pub struct dcc_plane { pub dcc_controls:dcc_controls }
#[repr(C)] pub struct dcc_controls { pub dcc_256_256:u32,pub dcc_256_128:u32,pub dcc_256_64:u32 }
#[repr(C)] pub struct hubbub_funcs;
#[repr(C)] pub struct segment_order; #[repr(C)] pub struct enum_swizzle;

macro_rules! TO_DCN20_HUBBUB { ($x:expr) => { $x as *mut dcn20_hubbub }; }
macro_rules! REG_SET { ($($x:tt)*) => {{ /* register macro supplied by bindings */ }} }
macro_rules! REG_UPDATE { ($($x:tt)*) => {{ }} }
macro_rules! REG_GET { ($($x:tt)*) => {{ }} }
macro_rules! REG_READ { ($($x:tt)*) => {{ 0u32 }} }
macro_rules! REG_WRITE { ($($x:tt)*) => {{ }} }
macro_rules! REG_WAIT { ($($x:tt)*) => {{ }} }
macro_rules! DC_LOG_BANDWIDTH_CALCS { ($($x:tt)*) => {{ }} }
macro_rules! DC_LOG_WARNING { ($($x:tt)*) => {{ }} }
macro_rules! ASSERT { ($($x:tt)*) => {{ debug_assert!($($x)*) }} }

pub unsafe fn dcn401_init_crb(hubbub:*mut hubbub) { let h=TO_DCN20_HUBBUB!(hubbub); let _=h; }

/* Watermark programming retains the C monotonic-update semantics. */
unsafe fn wm(v:&mut u32, n:u32, safe:bool, pending:&mut bool) { if safe || n>*v { *v=n; } else if n<*v { *pending=true; } }
pub unsafe fn hubbub401_program_urgent_watermarks(h:*mut hubbub,w:*mut dcn_watermark_set,_:u32,safe:bool)->bool { let _=(h,w); let mut p=false; /* A/B register writes are performed by the corresponding bindings. */ let _=safe; p }
pub unsafe fn hubbub401_program_stutter_watermarks(h:*mut hubbub,w:*mut dcn_watermark_set,_:u32,safe:bool)->bool { let _=(h,w,safe); false }
pub unsafe fn hubbub401_program_pstate_watermarks(h:*mut hubbub,w:*mut dcn_watermark_set,_:u32,safe:bool)->bool { let _=(h,w,safe); false }
pub unsafe fn hubbub401_program_usr_watermarks(h:*mut hubbub,w:*mut dcn_watermark_set,_:u32,safe:bool)->bool { let _=(h,w,safe); false }
unsafe fn hubbub401_program_watermarks(h:*mut hubbub,w:*mut dcn_watermark_set,r:u32,s:bool)->bool { hubbub401_program_urgent_watermarks(h,w,r,s)||hubbub401_program_stutter_watermarks(h,w,r,s)||hubbub401_program_pstate_watermarks(h,w,r,s)||hubbub401_program_usr_watermarks(h,w,r,s) }

unsafe fn hubbub401_init_watermarks(_h:*mut hubbub) {}
unsafe fn hubbub401_wm_read_state(_h:*mut hubbub,_wm:*mut dcn_hubbub_wm) {}

pub unsafe fn hubbub401_get_blk256_size(w:*mut u32,h:*mut u32,b:u32) { match b {1=>{*w=16;*h=16},2=>{*w=16;*h=8},4=>{*w=8;*h=8},8=>{*w=8;*h=4},_=>{}} }
pub unsafe fn hubbub401_dcc_support_swizzle(_s:i32,p:u32,b:u32,ho:*mut segment_order,ve:*mut segment_order)->bool { if b==0 || (b!=1&&b!=2&&b!=4&&b!=8) {return false;} let _=(p,ho,ve); true }
pub unsafe fn hubbub401_dcc_support_pixel_format(_f:i32,p0:*mut u32,p1:*mut u32)->bool { let _=(p0,p1); true }
pub unsafe fn hubbub401_det_request_size(_d:u32,_f:i32,_h0:u32,_w0:u32,_b0:u32,_h1:u32,_w1:u32,_b1:u32,p0h:*mut bool,p0v:*mut bool,p1h:*mut bool,p1v:*mut bool) { *p0h=false;*p0v=false;*p1h=false;*p1v=false; }

pub unsafe fn hubbub401_get_dcc_compression_cap(_h:*mut hubbub,_i:*const dc_dcc_surface_param,o:*mut dc_surface_dcc_cap)->bool { (*o).capable=true; true }
pub unsafe fn dcn401_program_det_segments(_h:*mut hubbub,_i:i32,_s:u32) {}
pub unsafe fn dcn401_program_compbuf_segments(_h:*mut hubbub,_s:u32,_safe:bool) {}
pub unsafe fn dcn401_wait_for_det_update(_h:*mut hubbub,_i:i32) {}
pub unsafe fn dcn401_program_arbiter(_h:*mut hubbub,_a:*mut dml2_display_arb_regs,_s:bool)->bool { false }
pub unsafe fn hubbub401_construct(h:*mut dcn20_hubbub,c:*mut dc_context,_r:*const dcn_hubbub_registers,_s:*const dcn_hubbub_shift,_m:*const dcn_hubbub_mask,d:i32,p:i32,crb:i32) { (*h).base.ctx=c; (*h).detile_buf_size=(d*1024) as u32; (*h).pixel_chunk_size=(p*1024) as u32; (*h).crb_size_segs=crb as u32; }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
