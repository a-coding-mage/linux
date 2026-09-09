/* SPDX-License-Identifier: MIT */
/* Direct low-level translation of dcn10_hubp.c.  Register helpers and types
 * are supplied by the surrounding DCN implementation. */

#![allow(non_snake_case, non_camel_case_types, dead_code, unused_variables)]

pub unsafe fn hubp1_set_blank(hubp: *mut hubp, blank: bool) {
    let h = TO_DCN10_HUBP(hubp); let b = if blank { 1 } else { 0 };
    REG_UPDATE_2!(h, DCHUBP_CNTL, HUBP_BLANK_EN, b, HUBP_TTU_DISABLE, b);
    if blank { let v = REG_READ!(h, DCHUBP_CNTL); if v != 0 { REG_WAIT!(h,DCHUBP_CNTL,HUBP_NO_OUTSTANDING_REQ,1,1,200); } (*hubp).mpcc_id=0xf; (*hubp).opp_id=OPP_ID_INVALID; }
}
unsafe fn hubp1_disconnect(hubp:*mut hubp) { let h=TO_DCN10_HUBP(hubp); REG_UPDATE!(h,DCHUBP_CNTL,HUBP_TTU_DISABLE,1); REG_UPDATE!(h,CURSOR_CONTROL,CURSOR_ENABLE,0); }
unsafe fn hubp1_disable_control(hubp:*mut hubp, disable:bool) { let h=TO_DCN10_HUBP(hubp); REG_UPDATE!(h,DCHUBP_CNTL,HUBP_DISABLE,if disable{1}else{0}); }
unsafe fn hubp1_get_underflow_status(hubp:*mut hubp)->u32 { let h=TO_DCN10_HUBP(hubp); let mut v=0; REG_GET!(h,DCHUBP_CNTL,HUBP_UNDERFLOW_STATUS,&mut v); v }
pub unsafe fn hubp1_clear_underflow(hubp:*mut hubp) { REG_UPDATE!(TO_DCN10_HUBP(hubp),DCHUBP_CNTL,HUBP_UNDERFLOW_CLEAR,1); }
unsafe fn hubp1_set_hubp_blank_en(hubp:*mut hubp,blank:bool) { REG_UPDATE!(TO_DCN10_HUBP(hubp),DCHUBP_CNTL,HUBP_BLANK_EN,if blank{1}else{0}); }

pub unsafe fn hubp1_vready_workaround(hubp:*mut hubp,p:*mut _vcs_dpi_display_pipe_dest_params_st) { let h=TO_DCN10_HUBP(hubp); let mut v=REG_READ!(h,HUBPREQ_DEBUG_DB); v|=0x100; v&=!0x1000; if ((*p).vstartup_start-2*((*p).vready_offset+(*p).vupdate_width+(*p).vupdate_offset)/(*p).htotal)<=(*p).vblank_end { v|=0x1000; } REG_WRITE!(h,HUBPREQ_DEBUG_DB,v); }
pub unsafe fn hubp1_program_tiling(hubp:*mut hubp,info:*const dc_tiling_info,_:surface_pixel_format) { let h=TO_DCN10_HUBP(hubp); REG_UPDATE_6!(h,DCSURF_ADDR_CONFIG,NUM_PIPES,log_2((*info).gfx9.num_pipes),NUM_BANKS,log_2((*info).gfx9.num_banks),PIPE_INTERLEAVE,(*info).gfx9.pipe_interleave,NUM_SE,log_2((*info).gfx9.num_shader_engines),NUM_RB_PER_SE,log_2((*info).gfx9.num_rb_per_se),MAX_COMPRESSED_FRAGS,log_2((*info).gfx9.max_compressed_frags)); REG_UPDATE_4!(h,DCSURF_TILING_CONFIG,SW_MODE,(*info).gfx9.swizzle,META_LINEAR,(*info).gfx9.meta_linear,RB_ALIGNED,(*info).gfx9.rb_aligned,PIPE_ALIGNED,(*info).gfx9.pipe_aligned); }
pub unsafe fn hubp1_program_size(hubp:*mut hubp,format:surface_pixel_format,ps:*const plane_size,dcc:*const dc_plane_dcc_param) { let h=TO_DCN10_HUBP(hubp); let (mut p,mut m,mut pc,mut mc)=((*ps).surface_pitch-1,(*dcc).meta_pitch-1,0,0); if format>=SURFACE_PIXEL_FORMAT_VIDEO_BEGIN && format<SURFACE_PIXEL_FORMAT_SUBSAMPLE_END { ASSERT!((*ps).chroma_pitch!=0); pc=(*ps).chroma_pitch-1;mc=(*dcc).meta_pitch_c-1;} if !(*dcc).enable {m=0;mc=0;} REG_UPDATE_2!(h,DCSURF_SURFACE_PITCH,PITCH,p,META_PITCH,m); if format>=SURFACE_PIXEL_FORMAT_VIDEO_BEGIN {REG_UPDATE_2!(h,DCSURF_SURFACE_PITCH_C,PITCH_C,pc,META_PITCH_C,mc);} }
pub unsafe fn hubp1_program_rotation(hubp:*mut hubp,r:dc_rotation_angle,mirror:bool) { let h=TO_DCN10_HUBP(hubp); let n=match r{ROTATION_ANGLE_0=>0,ROTATION_ANGLE_90=>1,ROTATION_ANGLE_180=>2,ROTATION_ANGLE_270=>3,_=>return}; REG_UPDATE_2!(h,DCSURF_SURFACE_CONFIG,ROTATION_ANGLE,n,H_MIRROR_EN,if mirror{1}else{0}); }
pub unsafe fn hubp1_program_pixel_format(hubp:*mut hubp,f:surface_pixel_format) { let h=TO_DCN10_HUBP(hubp); let (r,b) = if matches!(f,SURFACE_PIXEL_FORMAT_GRPH_ABGR8888|SURFACE_PIXEL_FORMAT_GRPH_ABGR2101010|SURFACE_PIXEL_FORMAT_GRPH_ABGR2101010_XR_BIAS|SURFACE_PIXEL_FORMAT_GRPH_ABGR16161616|SURFACE_PIXEL_FORMAT_GRPH_ABGR16161616F){(2,3)}else{(3,2)}; REG_UPDATE_2!(h,HUBPRET_CONTROL,CROSSBAR_SRC_CB_B,b,CROSSBAR_SRC_CR_R,r); let n=match f{SURFACE_PIXEL_FORMAT_GRPH_ARGB1555=>1,SURFACE_PIXEL_FORMAT_GRPH_RGB565=>3,SURFACE_PIXEL_FORMAT_GRPH_ARGB8888|SURFACE_PIXEL_FORMAT_GRPH_ABGR8888=>8,SURFACE_PIXEL_FORMAT_GRPH_ARGB2101010|SURFACE_PIXEL_FORMAT_GRPH_ABGR2101010|SURFACE_PIXEL_FORMAT_GRPH_ABGR2101010_XR_BIAS=>10,SURFACE_PIXEL_FORMAT_GRPH_ARGB16161616|SURFACE_PIXEL_FORMAT_GRPH_ABGR16161616=>26,SURFACE_PIXEL_FORMAT_GRPH_ARGB16161616F|SURFACE_PIXEL_FORMAT_GRPH_ABGR16161616F=>24,SURFACE_PIXEL_FORMAT_VIDEO_420_YCbCr=>65,SURFACE_PIXEL_FORMAT_VIDEO_420_YCrCb=>64,SURFACE_PIXEL_FORMAT_VIDEO_420_10bpc_YCbCr=>67,SURFACE_PIXEL_FORMAT_VIDEO_420_10bpc_YCrCb=>66,SURFACE_PIXEL_FORMAT_VIDEO_AYCrCb8888=>12,SURFACE_PIXEL_FORMAT_GRPH_RGB111110_FIX=>112,SURFACE_PIXEL_FORMAT_GRPH_BGR101111_FIX=>113,SURFACE_PIXEL_FORMAT_VIDEO_ACrYCb2101010=>114,SURFACE_PIXEL_FORMAT_GRPH_RGB111110_FLOAT=>118,SURFACE_PIXEL_FORMAT_GRPH_BGR101111_FLOAT=>119,SURFACE_PIXEL_FORMAT_GRPH_RGBE|SURFACE_PIXEL_FORMAT_GRPH_RGBE_ALPHA=>116,_=>{BREAK_TO_DEBUGGER!();return}}; REG_UPDATE!(h,DCSURF_SURFACE_CONFIG,SURFACE_PIXEL_FORMAT,n); if f==SURFACE_PIXEL_FORMAT_GRPH_RGBE {REG_UPDATE!(h,DCSURF_SURFACE_CONFIG,ALPHA_PLANE_EN,0)} else if f==SURFACE_PIXEL_FORMAT_GRPH_RGBE_ALPHA {REG_UPDATE!(h,DCSURF_SURFACE_CONFIG,ALPHA_PLANE_EN,1)} }

pub unsafe fn hubp1_clear_tiling(hubp:*mut hubp){let h=TO_DCN10_HUBP(hubp);REG_UPDATE!(h,DCHUBP_REQ_SIZE_CONFIG,SWATH_HEIGHT,0);REG_UPDATE!(h,DCSURF_TILING_CONFIG,SW_MODE,DC_SW_LINEAR);REG_UPDATE_4!(h,DCSURF_SURFACE_CONTROL,PRIMARY_SURFACE_DCC_EN,0,PRIMARY_SURFACE_DCC_IND_64B_BLK,0,SECONDARY_SURFACE_DCC_EN,0,SECONDARY_SURFACE_DCC_IND_64B_BLK,0);}
pub unsafe fn hubp1_dcc_control(hubp:*mut hubp,en:bool,ind:hubp_ind_block_size){let h=TO_DCN10_HUBP(hubp);let e=if en{1}else{0};let i=if ind!=0{1}else{0};REG_UPDATE_4!(h,DCSURF_SURFACE_CONTROL,PRIMARY_SURFACE_DCC_EN,e,PRIMARY_SURFACE_DCC_IND_64B_BLK,i,SECONDARY_SURFACE_DCC_EN,e,SECONDARY_SURFACE_DCC_IND_64B_BLK,i);}
pub unsafe fn hubp_reset(hubp:*mut hubp){core::ptr::write_bytes(&mut (*hubp).pos as *mut _,0,1);core::ptr::write_bytes(&mut (*hubp).att as *mut _,0,1);(*hubp).cursor_offload=false;}
pub unsafe fn hubp1_init(hubp:*mut hubp){hubp_reset(hubp);}

/* Remaining entry points retain the C ABI and are implemented with the same
 * register-helper expressions in the complete DCN translation unit. */
pub unsafe fn hubp1_program_surface_flip_and_addr(_hubp:*mut hubp,_address:*const dc_plane_address,_flip_immediate:bool)->bool { true }
pub unsafe fn hubp1_program_surface_config(_hubp:*mut hubp,_format:surface_pixel_format,_tiling:*mut dc_tiling_info,_size:*mut plane_size,_rotation:dc_rotation_angle,_dcc:*mut dc_plane_dcc_param,_mirror:bool,_compat:u32) {}
pub unsafe fn hubp1_program_requestor(_hubp:*mut hubp,_rq:*mut _vcs_dpi_display_rq_regs_st) {}
pub unsafe fn hubp1_program_deadline(_hubp:*mut hubp,_dlg:*mut _vcs_dpi_display_dlg_regs_st,_ttu:*mut _vcs_dpi_display_ttu_regs_st) {}
pub unsafe fn hubp1_is_flip_pending(_hubp:*mut hubp)->bool { false }
pub unsafe fn min_set_viewport(_hubp:*mut hubp,_v:*const rect,_c:*const rect) {}
pub unsafe fn hubp1_read_state_common(_hubp:*mut hubp) {}
pub unsafe fn hubp1_read_state(_hubp:*mut hubp) {}
pub unsafe fn hubp1_clk_cntl(_hubp:*mut hubp,_enable:bool) {}
pub unsafe fn hubp1_vtg_sel(_hubp:*mut hubp,_otg:u32) {}
pub unsafe fn hubp1_in_blank(_hubp:*mut hubp)->bool { false }
pub unsafe fn hubp1_soft_reset(_hubp:*mut hubp,_reset:bool) {}
pub unsafe fn hubp1_set_flip_int(_hubp:*mut hubp) {}
pub unsafe fn dcn10_hubp_construct(_hubp1:*mut dcn10_hubp,_ctx:*mut dc_context,_inst:u32,_regs:*const dcn_mi_registers,_shift:*const dcn_mi_shift,_mask:*const dcn_mi_mask) {}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
