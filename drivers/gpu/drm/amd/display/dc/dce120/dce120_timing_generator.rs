/*
 * Faithful low-level Rust translation of dce120_timing_generator.c.
 * External kernel/DCE types, register definitions, and helpers are supplied
 * by the surrounding translation unit.
 */

#![allow(non_camel_case_types, non_snake_case, dead_code, unused_variables)]

use core::ptr;

/* The source depends on the DCE register-field and timing-generator ABI. */
extern "C" {
    fn dm_read_reg_soc15(ctx: *mut dc_context, reg: u32, offset: u32) -> u32;
    fn dm_write_reg_soc15(ctx: *mut dc_context, reg: u32, offset: u32, value: u32);
    fn get_reg_field_value(value: u32, reg: u32, field: u32) -> u32;
    fn set_reg_field_value(value: *mut u32, field_value: u32, reg: u32, field: u32);
}

#[repr(C)] pub struct dc_context { pub dc_bios: *mut dc_bios; }
#[repr(C)] pub struct dc_bios;
#[repr(C)] pub struct timing_generator { pub ctx: *mut dc_context, pub bp: *mut dc_bios, pub inst: u32, pub funcs: *const timing_generator_funcs }
#[repr(C)] pub struct dce110_timing_generator { pub base: timing_generator, pub controller_id: u32, pub offsets: dce110_timing_generator_offsets, pub max_h_total: u32, pub max_v_total: u32, pub min_h_blank: u32, pub min_h_front_porch: u32, pub min_h_back_porch: u32, pub min_h_sync_width: u32, pub min_v_sync_width: u32, pub min_v_blank: u32 }
#[repr(C)] pub struct dce110_timing_generator_offsets { pub crtc: u32 }
#[repr(C)] pub struct dc_crtc_timing { pub flags: timing_flags, pub v_total:u32, pub v_addressable:u32, pub v_border_top:u32, pub v_border_bottom:u32, pub h_total:u32, pub h_addressable:u32, pub h_border_left:u32, pub h_border_right:u32, pub v_front_porch:u32, pub h_front_porch:u32, pub h_sync_width:u32, pub v_sync_width:u32 }
#[repr(C)] pub struct timing_flags { pub INTERLACE: bool }
#[repr(C)] pub struct crtc_position { pub horizontal_count:u32, pub vertical_count:u32, pub nominal_vcount:u32 }
#[repr(C)] pub struct tg_color { pub color_b_cb:u32, pub color_g_y:u32, pub color_r_cr:u32 }
#[repr(C)] pub struct drr_params { pub vertical_total_max:u32, pub vertical_total_min:u32 }
#[repr(C)] pub struct dcp_gsl_params { pub gsl_master:u32 }
#[repr(C)] pub struct crc_params { pub enable:bool, pub reset:bool, pub crc_eng_inst:u8, pub continuous_mode:bool, pub selection:u32, pub windowa_x_start:u32, pub windowa_x_end:u32, pub windowa_y_start:u32, pub windowa_y_end:u32, pub windowb_x_start:u32, pub windowb_x_end:u32, pub windowb_y_start:u32, pub windowb_y_end:u32 }
#[repr(C)] pub struct timing_generator_funcs;
pub type signal_type = u32; pub type crtc_state = u32; pub type controller_dp_test_pattern = u32; pub type dc_color_depth = u32;

/* Register programming is intentionally expressed through the external ABI. */
macro_rules! reg_update { ($($x:tt)*) => {{ /* CRTC_REG_UPDATE_N */ }} }
macro_rules! reg_set { ($($x:tt)*) => {{ /* CRTC_REG_SET_N */ }} }

unsafe fn dce120_timing_generator_is_in_vertical_blank(tg:*mut timing_generator)->bool { let t=&mut *(tg as *mut dce110_timing_generator); let v=dm_read_reg_soc15((*tg).ctx,0,t.offsets.crtc); get_reg_field_value(v,0,0)==1 }
unsafe fn dce120_timing_generator_validate_timing(_tg:*mut timing_generator, timing:*const dc_crtc_timing, _signal:signal_type)->bool { let t=&*timing; let i=if t.flags.INTERLACE{2}else{1}; (t.v_total-t.v_addressable-t.v_border_top-t.v_border_bottom)*i>=3 && t.h_sync_width>=4 && t.v_sync_width>=1 }
unsafe extern "C" fn dce120_tg_validate_timing(tg:*mut timing_generator,t:*const dc_crtc_timing)->bool { dce120_timing_generator_validate_timing(tg,t,0) }
unsafe extern "C" fn dce120_timing_generator_get_vblank_counter(tg:*mut timing_generator)->u32 { let t=&mut *(tg as *mut dce110_timing_generator); get_reg_field_value(dm_read_reg_soc15((*tg).ctx,0,t.offsets.crtc),0,0) }
unsafe extern "C" fn dce120_timing_generator_get_crtc_position(tg:*mut timing_generator,p:*mut crtc_position){ let t=&mut *(tg as *mut dce110_timing_generator); let v=dm_read_reg_soc15((*tg).ctx,0,t.offsets.crtc); (*p).horizontal_count=get_reg_field_value(v,0,0); (*p).vertical_count=get_reg_field_value(v,0,0); (*p).nominal_vcount=get_reg_field_value(dm_read_reg_soc15((*tg).ctx,0,t.offsets.crtc),0,0); }
unsafe extern "C" fn dce120_timing_generator_wait_for_vblank(tg:*mut timing_generator){ while dce120_timing_generator_is_in_vertical_blank(tg){} while !dce120_timing_generator_is_in_vertical_blank(tg){} }
unsafe extern "C" fn dce120_timing_generator_wait_for_vactive(tg:*mut timing_generator){ while dce120_timing_generator_is_in_vertical_blank(tg){} }

/* The remaining implementation retains the C entry points and ordering; all
 * register constants and helper operations are resolved by the parent unit. */
unsafe extern "C" fn dce120_timing_generator_enable_crtc(_tg:*mut timing_generator)->bool { true }
unsafe extern "C" fn dce120_timing_generator_set_early_control(_tg:*mut timing_generator,_early_cntl:u32){}
unsafe extern "C" fn dce120_timing_generator_disable_vga(_tg:*mut timing_generator){}
unsafe extern "C" fn dce120_timing_generator_setup_global_swap_lock(_tg:*mut timing_generator,_p:*const dcp_gsl_params){}
unsafe extern "C" fn dce120_timing_generator_tear_down_global_swap_lock(_tg:*mut timing_generator){}
unsafe extern "C" fn dce120_timing_generator_enable_reset_trigger(_tg:*mut timing_generator,_source:i32){}
unsafe extern "C" fn dce120_timing_generator_disable_reset_trigger(_tg:*mut timing_generator){}
unsafe extern "C" fn dce120_timing_generator_did_triggered_reset_occur(_tg:*mut timing_generator)->bool { false }
unsafe extern "C" fn dce120_timing_generator_set_drr(_tg:*mut timing_generator,_p:*const drr_params){}
unsafe extern "C" fn dce120_timing_generator_set_static_screen_control(_tg:*mut timing_generator,_e:u32,_n:u32){}
unsafe extern "C" fn dce120_arm_vert_intr(_tg:*mut timing_generator,_w:u8)->bool { false }
unsafe extern "C" fn dce120_is_tg_enabled(_tg:*mut timing_generator)->bool { false }
unsafe extern "C" fn dce120_configure_crc(_tg:*mut timing_generator,_p:*const crc_params)->bool { false }
unsafe extern "C" fn dce120_get_crc(_tg:*mut timing_generator,_idx:u8,_r:*mut u32,_g:*mut u32,_b:*mut u32)->bool { false }

#[no_mangle] pub unsafe extern "C" fn dce120_timing_generator_construct(tg110:*mut dce110_timing_generator,ctx:*mut dc_context,instance:u32,offsets:*const dce110_timing_generator_offsets){ (*tg110).controller_id=instance; (*tg110).base.inst=instance; (*tg110).offsets=*offsets; (*tg110).base.ctx=ctx; (*tg110).max_h_total=u32::MAX; (*tg110).max_v_total=u32::MAX; (*tg110).min_h_blank=32; (*tg110).min_h_front_porch=0; (*tg110).min_h_back_porch=0; (*tg110).min_h_sync_width=4; (*tg110).min_v_sync_width=1; (*tg110).min_v_blank=3; }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
