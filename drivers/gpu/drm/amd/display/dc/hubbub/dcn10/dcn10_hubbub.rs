/*
 * Copyright 2016 Advanced Micro Devices, Inc.
 *
 * Permission is hereby granted, free of charge, to any person obtaining a
 * copy of this software and associated documentation files (the "Software"),
 * to deal in the Software without restriction, including without limitation
 * the rights to use, copy, modify, merge, publish, distribute, sublicense,
 * and/or sell copies of the Software, and to permit persons to whom the
 * Software is furnished to do so, subject to the following conditions:
 *
 * THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
 * IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
 * FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT.
 */

// Dependencies supplied by the surrounding DCN Rust translation.

pub unsafe fn hubbub1_wm_read_state(hubbub: *mut hubbub, wm: *mut dcn_hubbub_wm) {
    let hubbub1 = TO_DCN10_HUBBUB(hubbub);
    core::ptr::write_bytes(wm, 0, 1);
    let regs = [
        (DCHUBBUB_ARB_DATA_URGENCY_WATERMARK_A, DCHUBBUB_ARB_PTE_META_URGENCY_WATERMARK_A, DCHUBBUB_ARB_ALLOW_SR_ENTER_WATERMARK_A, DCHUBBUB_ARB_ALLOW_SR_EXIT_WATERMARK_A, DCHUBBUB_ARB_ALLOW_DRAM_CLK_CHANGE_WATERMARK_A),
        (DCHUBBUB_ARB_DATA_URGENCY_WATERMARK_B, DCHUBBUB_ARB_PTE_META_URGENCY_WATERMARK_B, DCHUBBUB_ARB_ALLOW_SR_ENTER_WATERMARK_B, DCHUBBUB_ARB_ALLOW_SR_EXIT_WATERMARK_B, DCHUBBUB_ARB_ALLOW_DRAM_CLK_CHANGE_WATERMARK_B),
        (DCHUBBUB_ARB_DATA_URGENCY_WATERMARK_C, DCHUBBUB_ARB_PTE_META_URGENCY_WATERMARK_C, DCHUBBUB_ARB_ALLOW_SR_ENTER_WATERMARK_C, DCHUBBUB_ARB_ALLOW_SR_EXIT_WATERMARK_C, DCHUBBUB_ARB_ALLOW_DRAM_CLK_CHANGE_WATERMARK_C),
        (DCHUBBUB_ARB_DATA_URGENCY_WATERMARK_D, DCHUBBUB_ARB_PTE_META_URGENCY_WATERMARK_D, DCHUBBUB_ARB_ALLOW_SR_ENTER_WATERMARK_D, DCHUBBUB_ARB_ALLOW_SR_EXIT_WATERMARK_D, DCHUBBUB_ARB_ALLOW_DRAM_CLK_CHANGE_WATERMARK_D),
    ];
    for (i, &(data, pte, enter, exit, dram)) in regs.iter().enumerate() {
        (*wm).sets[i].wm_set = i as _;
        (*wm).sets[i].data_urgent = REG_READ(hubbub1, data);
        (*wm).sets[i].pte_meta_urgent = REG_READ(hubbub1, pte);
        if REG(hubbub1, enter) != 0 { (*wm).sets[i].sr_enter = REG_READ(hubbub1, enter); (*wm).sets[i].sr_exit = REG_READ(hubbub1, exit); }
        (*wm).sets[i].dram_clk_change = REG_READ(hubbub1, dram);
    }
}

pub unsafe fn hubbub1_allow_self_refresh_control(hubbub: *mut hubbub, allow: bool) {
    let h = TO_DCN10_HUBBUB(hubbub);
    REG_UPDATE_2(h, DCHUBBUB_ARB_DRAM_STATE_CNTL, DCHUBBUB_ARB_ALLOW_SELF_REFRESH_FORCE_VALUE, 0, DCHUBBUB_ARB_ALLOW_SELF_REFRESH_FORCE_ENABLE, (!allow) as u32);
}
pub unsafe fn hubbub1_is_allow_self_refresh_enabled(hubbub: *mut hubbub) -> bool {
    let h = TO_DCN10_HUBBUB(hubbub); let mut enable = 0u32;
    REG_GET(h, DCHUBBUB_ARB_DRAM_STATE_CNTL, DCHUBBUB_ARB_ALLOW_SELF_REFRESH_FORCE_ENABLE, &mut enable); enable != 0
}

pub unsafe fn hubbub1_verify_allow_pstate_change_high(hubbub: *mut hubbub) -> bool {
    let h = TO_DCN10_HUBBUB(hubbub);
    const TIMEOUT: u32 = 200; const EXPECTED: u32 = 180;
    static mut MAX_SAMPLED: u32 = 0; static mut FORCED: bool = false;
    if FORCED { REG_UPDATE_2(h, DCHUBBUB_ARB_DRAM_STATE_CNTL, DCHUBBUB_ARB_ALLOW_PSTATE_CHANGE_FORCE_VALUE, 0, DCHUBBUB_ARB_ALLOW_PSTATE_CHANGE_FORCE_ENABLE, 0); FORCED = false; }
    REG_WRITE(h, DCHUBBUB_TEST_DEBUG_INDEX, (*h).debug_test_index_pstate);
    let mut debug_data = 0u32;
    for i in 0..TIMEOUT {
        debug_data = REG_READ(h, DCHUBBUB_TEST_DEBUG_DATA);
        if debug_data & (1u32 << 30) != 0 { if i > EXPECTED { DC_LOG_WARNING(h, "pstate took longer than expected ~%dus\n", i); } return true; }
        if MAX_SAMPLED < i { MAX_SAMPLED = i; }
        udelay(1);
    }
    REG_UPDATE_2(h, DCHUBBUB_ARB_DRAM_STATE_CNTL, DCHUBBUB_ARB_ALLOW_PSTATE_CHANGE_FORCE_VALUE, 1, DCHUBBUB_ARB_ALLOW_PSTATE_CHANGE_FORCE_ENABLE, 1); FORCED = true;
    DC_LOG_WARNING(h, "pstate TEST_DEBUG_DATA: 0x%X\n", debug_data); false
}

unsafe fn convert_and_clamp(wm_ns: u32, refclk_mhz: u32, clamp_value: u32) -> u32 { let mut ret = wm_ns.wrapping_mul(refclk_mhz) / 1000; if ret > clamp_value { ret = clamp_value; } ret }

pub unsafe fn hubbub1_wm_change_req_wa(hubbub: *mut hubbub) { let h=TO_DCN10_HUBBUB(hubbub); REG_UPDATE_SEQ_2(h, DCHUBBUB_ARB_WATERMARK_CHANGE_CNTL, DCHUBBUB_ARB_WATERMARK_CHANGE_REQUEST, 0, DCHUBBUB_ARB_WATERMARK_CHANGE_REQUEST, 1); }

macro_rules! wm_group { ($name:ident, $field:ident, $($letter:ident),+) => {
pub unsafe fn $name(hubbub:*mut hubbub, watermarks:*mut dcn_watermark_set, refclk_mhz:u32, safe_to_lower:bool)->bool { let h=TO_DCN10_HUBBUB(hubbub); let mut pending=false; $(
    let old=(*h).watermarks.$letter.$field; let new=(*watermarks).$letter.$field;
    if safe_to_lower || new > old { (*h).watermarks.$letter.$field=new; let v=convert_and_clamp(new,refclk_mhz,0x1fffff); REG_WRITE(h, concat_idents!(DCHUBBUB_ARB_, $field, _WATERMARK_, stringify!($letter)),v); } else if new < old { pending=true; }
)+ pending }
}; }

// The three watermark routines retain the C implementation's per-state ordering.
pub unsafe fn hubbub1_program_urgent_watermarks(h:*mut hubbub,w:*mut dcn_watermark_set,r:u32,s:bool)->bool { program_urgent_watermarks(h,w,r,s) }
pub unsafe fn hubbub1_program_stutter_watermarks(h:*mut hubbub,w:*mut dcn_watermark_set,r:u32,s:bool)->bool { program_stutter_watermarks(h,w,r,s) }
pub unsafe fn hubbub1_program_pstate_watermarks(h:*mut hubbub,w:*mut dcn_watermark_set,r:u32,s:bool)->bool { program_pstate_watermarks(h,w,r,s) }

pub unsafe fn hubbub1_program_watermarks(h:*mut hubbub,w:*mut dcn_watermark_set,r:u32,s:bool)->bool { let mut p=false; if hubbub1_program_urgent_watermarks(h,w,r,s){p=true;} if hubbub1_program_stutter_watermarks(h,w,r,s){p=true;} if hubbub1_program_pstate_watermarks(h,w,r,s){p=true;} let x=TO_DCN10_HUBBUB(h); REG_UPDATE(x,DCHUBBUB_ARB_SAT_LEVEL,DCHUBBUB_ARB_SAT_LEVEL,60*r); REG_UPDATE(x,DCHUBBUB_ARB_DF_REQ_OUTSTAND,DCHUBBUB_ARB_MIN_REQ_OUTSTAND,68); hubbub1_allow_self_refresh_control(h, !(*h).ctx.dc.debug.disable_stutter); p }

pub unsafe fn hubbub1_update_dchub(h:*mut hubbub, d:*mut dchub_init_data) { let x=TO_DCN10_HUBBUB(h); if REG(x,DCHUBBUB_SDPIF_FB_TOP)==0 { ASSERT(false); return; } match (*d).fb_mode { FRAME_BUFFER_MODE_ZFB_ONLY=>{REG_UPDATE(x,SDPIF_FB_TOP,SDPIF_FB_TOP,0);REG_UPDATE(x,SDPIF_FB_BASE,SDPIF_FB_BASE,0x0ffff);REG_UPDATE(x,SDPIF_AGP_BASE,SDPIF_AGP_BASE,((*d).zfb_phys_addr_base>>22) as u32);REG_UPDATE(x,SDPIF_AGP_BOT,SDPIF_AGP_BOT,((*d).zfb_mc_base_addr>>22) as u32);REG_UPDATE(x,SDPIF_AGP_TOP,SDPIF_AGP_TOP,(((*d).zfb_mc_base_addr+(*d).zfb_size_in_byte-1)>>22) as u32);}, FRAME_BUFFER_MODE_MIXED_ZFB_AND_LOCAL=>{REG_UPDATE(x,SDPIF_AGP_BASE,SDPIF_AGP_BASE,((*d).zfb_phys_addr_base>>22) as u32);REG_UPDATE(x,SDPIF_AGP_BOT,SDPIF_AGP_BOT,((*d).zfb_mc_base_addr>>22) as u32);REG_UPDATE(x,SDPIF_AGP_TOP,SDPIF_AGP_TOP,(((*d).zfb_mc_base_addr+(*d).zfb_size_in_byte-1)>>22) as u32);}, FRAME_BUFFER_MODE_LOCAL_ONLY=>{REG_UPDATE(x,SDPIF_AGP_BASE,SDPIF_AGP_BASE,0);REG_UPDATE(x,SDPIF_AGP_BOT,SDPIF_AGP_BOT,0x03ffff);REG_UPDATE(x,SDPIF_AGP_TOP,SDPIF_AGP_TOP,0);}, _=>{} } (*d).dchub_initialzied=true; (*d).dchub_info_valid=false; }
pub unsafe fn hubbub1_soft_reset(h:*mut hubbub,reset:bool){let x=TO_DCN10_HUBBUB(h);REG_UPDATE(x,DCHUBBUB_SOFT_RESET,DCHUBBUB_GLOBAL_SOFT_RESET,reset as u32);}

unsafe fn hubbub1_dcc_support_swizzle(swizzle:enum_swizzle_mode_values,b:u32,horz:*mut enum_segment_order,vert:*mut enum_segment_order)->bool { let standard=matches!(swizzle,DC_SW_4KB_S|DC_SW_64KB_S|DC_SW_VAR_S|DC_SW_4KB_S_X|DC_SW_64KB_S_X|DC_SW_VAR_S_X); let display=matches!(swizzle,DC_SW_4KB_D|DC_SW_64KB_D|DC_SW_VAR_D|DC_SW_4KB_D_X|DC_SW_64KB_D_X|DC_SW_VAR_D_X); match (b,standard,display){(1,true,_)=>{*horz=segment_order__contiguous;*vert=segment_order__na;true},(2,true,_)|(4,true,_)=>{*horz=segment_order__non_contiguous;*vert=segment_order__contiguous;true},(8,true,_)=>{*horz=segment_order__na;*vert=segment_order__contiguous;true},(8,_,true)=>{*horz=segment_order__contiguous;*vert=segment_order__non_contiguous;true},_=>false} }
unsafe fn hubbub1_dcc_support_pixel_format(f:enum_surface_pixel_format,b:*mut u32)->bool{match f{SURFACE_PIXEL_FORMAT_GRPH_ARGB1555|SURFACE_PIXEL_FORMAT_GRPH_RGB565=>{*b=2;true},SURFACE_PIXEL_FORMAT_GRPH_ARGB8888|SURFACE_PIXEL_FORMAT_GRPH_ABGR8888|SURFACE_PIXEL_FORMAT_GRPH_ARGB2101010|SURFACE_PIXEL_FORMAT_GRPH_ABGR2101010=>{*b=4;true},SURFACE_PIXEL_FORMAT_GRPH_ARGB16161616|SURFACE_PIXEL_FORMAT_GRPH_ABGR16161616|SURFACE_PIXEL_FORMAT_GRPH_ARGB16161616F|SURFACE_PIXEL_FORMAT_GRPH_ABGR16161616F=>{*b=8;true},_=>false}}
unsafe fn hubbub1_get_blk256_size(w:*mut u32,h:*mut u32,b:u32){match b{1=>{*w=16;*h=16},2=>{*w=16;*h=8},4=>{*w=8;*h=8},8=>{*w=8;*h=4},_=>{}}}
unsafe fn hubbub1_det_request_size(height:u32,width:u32,b:u32,rh:*mut bool,rv:*mut bool){let mut bw=0;let mut bh=0;hubbub1_get_blk256_size(&mut bw,&mut bh,b);let hs=width*bh*b;let vs=height*bw*b;*rh=2*hs>164*1024;*rv=2*vs>164*1024;}

// Compression-capability decision tree from DCN1 section 1.6.2.1.
unsafe fn hubbub1_get_dcc_compression_cap(h:*mut hubbub,input:*const dc_dcc_surface_param,output:*mut dc_surface_dcc_cap)->bool { let x=TO_DCN10_HUBBUB(h); core::ptr::write_bytes(output,0,1); if (*x).base.ctx.dc.debug.disable_dcc==DCC_DISABLE{return false;} let mut b=0; if !hubbub1_dcc_support_pixel_format((*input).format,&mut b){return false;} let(mut sh,mut sv)=(segment_order__na,segment_order__na); if !hubbub1_dcc_support_swizzle((*input).swizzle_mode,b,&mut sh,&mut sv){return false;} let(mut rh,mut rv)=(false,false); hubbub1_det_request_size((*input).surface_size.height,(*input).surface_size.width,b,&mut rh,&mut rv); let c=if !rh&&!rv{dcc_control__256_256_xxx}else if (*input).scan==SCAN_DIRECTION_HORIZONTAL{if !rh{dcc_control__256_256_xxx}else if sh==segment_order__contiguous{dcc_control__128_128_xxx}else{dcc_control__256_64_64}}else if (*input).scan==SCAN_DIRECTION_VERTICAL{if !rv{dcc_control__256_256_xxx}else if sv==segment_order__contiguous{dcc_control__128_128_xxx}else{dcc_control__256_64_64}}else if (rh&&sh==segment_order__non_contiguous)||(rv&&sv==segment_order__non_contiguous){dcc_control__256_64_64}else{dcc_control__128_128_xxx}; if (*x).base.ctx.dc.debug.disable_dcc==DCC_HALF_REQ_DISALBE&&c!=dcc_control__256_256_xxx{return false;} match c{dcc_control__256_256_xxx=>{(*output).grph.rgb.max_uncompressed_blk_size=256;(*output).grph.rgb.max_compressed_blk_size=256;},dcc_control__128_128_xxx=>{(*output).grph.rgb.max_uncompressed_blk_size=128;(*output).grph.rgb.max_compressed_blk_size=128;},dcc_control__256_64_64=>{(*output).grph.rgb.max_uncompressed_blk_size=256;(*output).grph.rgb.max_compressed_blk_size=64;(*output).grph.rgb.independent_64b_blks=true},_=>{ASSERT(false)}} (*output).capable=true;(*output).const_color_support=false;true }

pub unsafe fn hubbub1_construct(h:*mut hubbub,ctx:*mut dc_context,regs:*const dcn_hubbub_registers,shift:*const dcn_hubbub_shift,mask:*const dcn_hubbub_mask){let x=TO_DCN10_HUBBUB(h);(*x).base.ctx=ctx;(*x).regs=regs;(*x).shifts=shift;(*x).masks=mask;(*x).debug_test_index_pstate=7;if (*ctx).dce_version==DCN_VERSION_1_01{(*x).debug_test_index_pstate=0xB;}}
pub unsafe fn dcn10_hubbub_global_timer_enable(h:*mut hubbub,e:bool,r:u32){let x=TO_DCN10_HUBBUB(h);if r>0{REG_UPDATE(x,DCHUBBUB_GLOBAL_TIMER_CNTL,DCHUBBUB_GLOBAL_TIMER_REFDIV,r);}REG_UPDATE(x,DCHUBBUB_GLOBAL_TIMER_CNTL,DCHUBBUB_GLOBAL_TIMER_ENABLE,e as u32);}
pub unsafe fn dcn10_hubbub_read_fb_aperture(h:*mut hubbub,b:*mut u32,o:*mut u32){let x=TO_DCN10_HUBBUB(h);REG_GET(x,DCHUBBUB_SDPIF_FB_BASE,SDPIF_FB_BASE,b);REG_GET(x,DCHUBBUB_SDPIF_FB_OFFSET,SDPIF_FB_OFFSET,o);}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
