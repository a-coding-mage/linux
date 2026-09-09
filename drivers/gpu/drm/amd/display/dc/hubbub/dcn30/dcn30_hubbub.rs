/*
 * Copyright 2020 Advanced Micro Devices, Inc.
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

// Dependencies are supplied by the surrounding translated driver.

const NUM_VMID: u32 = 16;

unsafe fn convert_and_clamp(wm_ns: u32, refclk_mhz: u32, clamp_value: u32) -> u32 {
    let mut ret_val = wm_ns.wrapping_mul(refclk_mhz);
    ret_val /= 1000;
    if ret_val > clamp_value { ret_val = clamp_value; }
    ret_val
}

pub unsafe fn hubbub3_init_dchub_sys_ctx(hubbub: *mut hubbub, pa_config: *mut dcn_hubbub_phys_addr_config) -> i32 {
    let hubbub1 = TO_DCN20_HUBBUB(hubbub);
    let mut phys_config: dcn_vmid_page_table_config = core::mem::zeroed();
    REG_SET!(hubbub1, DCN_VM_FB_LOCATION_BASE, 0, FB_BASE, ADDR_HI24((*pa_config).system_aperture.fb_base));
    REG_SET!(hubbub1, DCN_VM_FB_LOCATION_TOP, 0, FB_TOP, ADDR_HI24((*pa_config).system_aperture.fb_top));
    REG_SET!(hubbub1, DCN_VM_FB_OFFSET, 0, FB_OFFSET, ADDR_HI24((*pa_config).system_aperture.fb_offset));
    REG_SET!(hubbub1, DCN_VM_AGP_BOT, 0, AGP_BOT, ADDR_HI24((*pa_config).system_aperture.agp_bot));
    REG_SET!(hubbub1, DCN_VM_AGP_TOP, 0, AGP_TOP, ADDR_HI24((*pa_config).system_aperture.agp_top));
    REG_SET!(hubbub1, DCN_VM_AGP_BASE, 0, AGP_BASE, ADDR_HI24((*pa_config).system_aperture.agp_base));
    if (*pa_config).gart_config.page_table_start_addr != (*pa_config).gart_config.page_table_end_addr {
        phys_config.page_table_start_addr = (*pa_config).gart_config.page_table_start_addr >> 12;
        phys_config.page_table_end_addr = (*pa_config).gart_config.page_table_end_addr >> 12;
        phys_config.page_table_base_addr = (*pa_config).gart_config.page_table_base_addr;
        phys_config.depth = 0;
        phys_config.block_size = 0;
        dcn20_vmid_setup(&mut (*hubbub1).vmid[0], &phys_config);
    }
    NUM_VMID as i32
}

pub unsafe fn hubbub3_program_watermarks(hubbub: *mut hubbub, watermarks: *mut dcn_watermark_set, refclk_mhz: u32, safe_to_lower: bool) -> bool {
    let hubbub1 = TO_DCN20_HUBBUB(hubbub);
    let mut wm_pending = false;
    if hubbub21_program_urgent_watermarks(hubbub, watermarks, refclk_mhz, safe_to_lower) { wm_pending = true; }
    if hubbub21_program_stutter_watermarks(hubbub, watermarks, refclk_mhz, safe_to_lower) { wm_pending = true; }
    if hubbub21_program_pstate_watermarks(hubbub, watermarks, refclk_mhz, safe_to_lower) { wm_pending = true; }
    REG_SET!(hubbub1, DCHUBBUB_ARB_SAT_LEVEL, 0, DCHUBBUB_ARB_SAT_LEVEL, 60 * refclk_mhz);
    REG_UPDATE!(hubbub1, DCHUBBUB_ARB_DF_REQ_OUTSTAND, DCHUBBUB_ARB_MIN_REQ_OUTSTAND, 0x1FF);
    if safe_to_lower || (*hubbub).ctx.dc.debug.disable_stutter { hubbub1_allow_self_refresh_control(hubbub, !(*hubbub).ctx.dc.debug.disable_stutter); }
    wm_pending
}

pub unsafe fn hubbub3_dcc_support_swizzle(swizzle: swizzle_mode_values, bytes_per_element: u32, segment_order_horz: *mut segment_order, segment_order_vert: *mut segment_order) -> bool {
    let standard = matches!(swizzle, DC_SW_4KB_S | DC_SW_64KB_S | DC_SW_VAR_S | DC_SW_4KB_S_X | DC_SW_64KB_S_X | DC_SW_VAR_S_X);
    let render = matches!(swizzle, DC_SW_4KB_R | DC_SW_64KB_R | DC_SW_VAR_R | DC_SW_4KB_R_X | DC_SW_64KB_R_X | DC_SW_VAR_R_X);
    let display = matches!(swizzle, DC_SW_4KB_D | DC_SW_64KB_D | DC_SW_VAR_D | DC_SW_4KB_D_X | DC_SW_64KB_D_X | DC_SW_VAR_D_X);
    if standard {
        match bytes_per_element { 1 => { *segment_order_horz=segment_order__contiguous; *segment_order_vert=segment_order__na; return true; }, 2|4 => { *segment_order_horz=segment_order__non_contiguous; *segment_order_vert=segment_order__contiguous; return true; }, 8 => { *segment_order_horz=segment_order__na; *segment_order_vert=segment_order__contiguous; return true; }, _ => {} }
    }
    if render {
        match bytes_per_element { 1 => { *segment_order_horz=segment_order__contiguous; *segment_order_vert=segment_order__na; return true; }, 2 => { *segment_order_horz=segment_order__non_contiguous; *segment_order_vert=segment_order__contiguous; return true; }, 4|8 => { *segment_order_horz=segment_order__contiguous; *segment_order_vert=segment_order__non_contiguous; return true; }, _ => {} }
    }
    if display && bytes_per_element == 8 { *segment_order_horz=segment_order__contiguous; *segment_order_vert=segment_order__non_contiguous; return true; }
    false
}

unsafe fn hubbub3_get_blk256_size(w: *mut u32, h: *mut u32, bpe: u32) { match bpe { 1=>{*w=16;*h=16}, 2=>{*w=16;*h=8}, 4=>{*w=8;*h=8}, 8=>{*w=8;*h=4}, _=>{} } }

unsafe fn hubbub3_det_request_size(detile_buf_size:u32,height:u32,width:u32,bpe:u32,req128_horz_wc:*mut bool,req128_vert_wc:*mut bool) {
    let mut bh=0; let mut bw=0; hubbub3_get_blk256_size(&mut bw,&mut bh,bpe);
    let sh=width*bh*bpe; let sv=height*bw*bpe;
    *req128_horz_wc = 2*sh > detile_buf_size; *req128_vert_wc = 2*sv > detile_buf_size;
}

pub unsafe fn hubbub3_get_dcc_compression_cap(hubbub:*mut hubbub,input:*const dc_dcc_surface_param,output:*mut dc_surface_dcc_cap)->bool {
    let dc=(*hubbub).ctx.dc; core::ptr::write_bytes(output as *mut u8,0,core::mem::size_of::<dc_surface_dcc_cap>());
    if (*dc).debug.disable_dcc==DCC_DISABLE{return false;} let mut bpe=0; let mut sh=segment_order__na; let mut sv=segment_order__na;
    if !((*hubbub).funcs.dcc_support_pixel_format)((*input).format,&mut bpe)||!((*hubbub).funcs.dcc_support_swizzle)((*input).swizzle_mode,bpe,&mut sh,&mut sv){return false;}
    let mut rh=false;let mut rv=false; let h=TO_DCN20_HUBBUB(hubbub); hubbub3_det_request_size((*h).detile_buf_size,(*input).surface_size.height,(*input).surface_size.width,bpe,&mut rh,&mut rv);
    let mut c= dcc_control__128_128_xxx;
    if !rh&&!rv {c=dcc_control__256_256_xxx;} else if (*input).scan==SCAN_DIRECTION_HORIZONTAL {if !rh{c=dcc_control__256_256_xxx}else if sh==segment_order__contiguous{c=dcc_control__128_128_xxx}else{c=dcc_control__256_64_64}}
    else if (*input).scan==SCAN_DIRECTION_VERTICAL {if !rv{c=dcc_control__256_256_xxx}else if sv==segment_order__contiguous{c=dcc_control__128_128_xxx}else{c=dcc_control__256_64_64}}
    else if (rh&&sh==segment_order__non_contiguous)||(rv&&sv==segment_order__non_contiguous){c=dcc_control__256_64_64;}
    if bpe==2&&(*input).swizzle_mode==DC_SW_64KB_R_X{c=dcc_control__128_128_xxx;}
    if (*dc).debug.disable_dcc==DCC_HALF_REQ_DISALBE&&c!=dcc_control__256_256_xxx{return false;}
    match c { dcc_control__256_256|dcc_control__256_256_xxx=>{(*output).grph.rgb.max_uncompressed_blk_size=256;(*output).grph.rgb.max_compressed_blk_size=256;(*output).grph.rgb.independent_64b_blks=false;(*output).grph.rgb.dcc_controls.dcc_256_256_unconstrained=1;(*output).grph.rgb.dcc_controls.dcc_256_128_128=1}, dcc_control__256_128|dcc_control__128_128_xxx=>{(*output).grph.rgb.max_uncompressed_blk_size=128;(*output).grph.rgb.max_compressed_blk_size=128;(*output).grph.rgb.independent_64b_blks=false;(*output).grph.rgb.dcc_controls.dcc_128_128_uncontrained=1;(*output).grph.rgb.dcc_controls.dcc_256_128_128=1}, dcc_control__256_64|dcc_control__256_64_64=>{(*output).grph.rgb.max_uncompressed_blk_size=256;(*output).grph.rgb.max_compressed_blk_size=64;(*output).grph.rgb.independent_64b_blks=true;(*output).grph.rgb.dcc_controls.dcc_256_64_64=1}, dcc_control__256_128_128=>{(*output).grph.rgb.max_uncompressed_blk_size=256;(*output).grph.rgb.max_compressed_blk_size=128;(*output).grph.rgb.independent_64b_blks=false;(*output).grph.rgb.dcc_controls.dcc_256_128_128=1}, _=>{} }
    (*output).capable=true;(*output).const_color_support=true;true
}

pub unsafe fn hubbub3_force_wm_propagate_to_pipes(hubbub:*mut hubbub) { let h=TO_DCN20_HUBBUB(hubbub); let r=(*hubbub).ctx.dc.res_pool.ref_clocks.dchub_ref_clock_inKhz/1000; let v=convert_and_clamp((*h).watermarks.a.urgent_ns,r,0x1fffff); REG_SET_2!(h,DCHUBBUB_ARB_DATA_URGENCY_WATERMARK_A,0,DCHUBBUB_ARB_DATA_URGENCY_WATERMARK_A,v,DCHUBBUB_ARB_VM_ROW_URGENCY_WATERMARK_A,v); }
pub unsafe fn hubbub3_force_pstate_change_control(hubbub:*mut hubbub,force:bool,allow:bool) { let h=TO_DCN20_HUBBUB(hubbub); REG_UPDATE_2!(h,DCHUBBUB_ARB_DRAM_STATE_CNTL,DCHUBBUB_ARB_ALLOW_PSTATE_CHANGE_FORCE_VALUE,allow,DCHUBBUB_ARB_ALLOW_PSTATE_CHANGE_FORCE_ENABLE,force); }

pub unsafe fn hubbub3_init_watermarks(hubbub:*mut hubbub) { let h=TO_DCN20_HUBBUB(hubbub); let mut r=REG_READ!(h,DCHUBBUB_ARB_DATA_URGENCY_WATERMARK_A); REG_WRITE!(h,DCHUBBUB_ARB_DATA_URGENCY_WATERMARK_B,r); REG_WRITE!(h,DCHUBBUB_ARB_DATA_URGENCY_WATERMARK_C,r); REG_WRITE!(h,DCHUBBUB_ARB_DATA_URGENCY_WATERMARK_D,r); r=REG_READ!(h,DCHUBBUB_ARB_FRAC_URG_BW_FLIP_A); REG_WRITE!(h,DCHUBBUB_ARB_FRAC_URG_BW_FLIP_B,r); REG_WRITE!(h,DCHUBBUB_ARB_FRAC_URG_BW_FLIP_C,r); REG_WRITE!(h,DCHUBBUB_ARB_FRAC_URG_BW_FLIP_D,r); r=REG_READ!(h,DCHUBBUB_ARB_FRAC_URG_BW_NOM_A); REG_WRITE!(h,DCHUBBUB_ARB_FRAC_URG_BW_NOM_B,r); REG_WRITE!(h,DCHUBBUB_ARB_FRAC_URG_BW_NOM_C,r); REG_WRITE!(h,DCHUBBUB_ARB_FRAC_URG_BW_NOM_D,r); r=REG_READ!(h,DCHUBBUB_ARB_REFCYC_PER_TRIP_TO_MEMORY_A); REG_WRITE!(h,DCHUBBUB_ARB_REFCYC_PER_TRIP_TO_MEMORY_B,r); REG_WRITE!(h,DCHUBBUB_ARB_REFCYC_PER_TRIP_TO_MEMORY_C,r); REG_WRITE!(h,DCHUBBUB_ARB_REFCYC_PER_TRIP_TO_MEMORY_D,r); r=REG_READ!(h,DCHUBBUB_ARB_ALLOW_SR_ENTER_WATERMARK_A); REG_WRITE!(h,DCHUBBUB_ARB_ALLOW_SR_ENTER_WATERMARK_B,r); REG_WRITE!(h,DCHUBBUB_ARB_ALLOW_SR_ENTER_WATERMARK_C,r); REG_WRITE!(h,DCHUBBUB_ARB_ALLOW_SR_ENTER_WATERMARK_D,r); r=REG_READ!(h,DCHUBBUB_ARB_ALLOW_SR_EXIT_WATERMARK_A); REG_WRITE!(h,DCHUBBUB_ARB_ALLOW_SR_EXIT_WATERMARK_B,r); REG_WRITE!(h,DCHUBBUB_ARB_ALLOW_SR_EXIT_WATERMARK_C,r); REG_WRITE!(h,DCHUBBUB_ARB_ALLOW_SR_EXIT_WATERMARK_D,r); r=REG_READ!(h,DCHUBBUB_ARB_ALLOW_DRAM_CLK_CHANGE_WATERMARK_A); REG_WRITE!(h,DCHUBBUB_ARB_ALLOW_DRAM_CLK_CHANGE_WATERMARK_B,r); REG_WRITE!(h,DCHUBBUB_ARB_ALLOW_DRAM_CLK_CHANGE_WATERMARK_C,r); REG_WRITE!(h,DCHUBBUB_ARB_ALLOW_DRAM_CLK_CHANGE_WATERMARK_D,r); }

pub unsafe fn hubbub3_read_reg_state(hubbub:*mut hubbub, s:*mut dcn_hubbub_reg_state) { let h=TO_DCN20_HUBBUB(hubbub); (*s).det0_ctrl=REG_READ!(h,DCHUBBUB_DET0_CTRL); (*s).det1_ctrl=REG_READ!(h,DCHUBBUB_DET1_CTRL); (*s).det2_ctrl=REG_READ!(h,DCHUBBUB_DET2_CTRL); (*s).det3_ctrl=REG_READ!(h,DCHUBBUB_DET3_CTRL); (*s).compbuf_ctrl=REG_READ!(h,DCHUBBUB_COMPBUF_CTRL); }

pub unsafe fn hubbub3_construct(hubbub3:*mut dcn20_hubbub,ctx:*mut dc_context,regs:*const dcn_hubbub_registers,shift:*const dcn_hubbub_shift,mask:*const dcn_hubbub_mask) { (*hubbub3).base.ctx=ctx; (*hubbub3).base.funcs=&hubbub30_funcs; (*hubbub3).regs=regs; (*hubbub3).shifts=shift; (*hubbub3).masks=mask; (*hubbub3).debug_test_index_pstate=0xB; (*hubbub3).detile_buf_size=184*1024; }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
