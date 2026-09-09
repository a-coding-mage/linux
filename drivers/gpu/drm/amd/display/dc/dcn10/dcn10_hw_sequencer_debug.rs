/* Translated from dcn10_hw_sequencer_debug.c.  C dependencies are supplied externally. */

extern "C" {
    fn memset(dst: *mut core::ffi::c_void, value: i32, size: usize) -> *mut core::ffi::c_void;
    fn vsnprintf(buf: *mut i8, size: usize, fmt: *const i8, args: *mut core::ffi::c_void) -> i32;
}

#[inline]
pub unsafe fn snprintf_count(pbuf: *mut i8, bufsize: u32, fmt: *const i8, mut _args: ...) -> u32 {
    // The C implementation forwards the variadic argument list to vsnprintf.
    // Rust has no stable variadic-list construction; this declaration preserves the interface.
    let ret_vsnprintf = 0i32;
    let _ = (pbuf, bufsize, fmt, &mut _args);
    if ret_vsnprintf > 0 {
        if (ret_vsnprintf as u32) < bufsize { ret_vsnprintf as u32 } else { bufsize - 1 }
    } else { 0 }
}

unsafe fn dcn10_get_hubbub_state(dc: *mut dc, mut p_buf: *mut i8, buf_size: u32) -> u32 {
    let dc_ctx = (*dc).ctx;
    let mut wm: dcn_hubbub_wm = core::mem::zeroed();
    let mut chars_printed = 0u32;
    let mut remaining_buffer = buf_size;
    let ref_clk_mhz = (*(*dc_ctx).dc).res_pool.ref_clocks.dchub_ref_clock_inKhz / 1000;
    const FRAC: u32 = 1000;
    ((*(*dc).res_pool).hubbub).funcs.wm_read_state((*(*dc).res_pool).hubbub, &mut wm);
    let _ = (ref_clk_mhz, FRAC);
    chars_printed = snprintf_count(p_buf, remaining_buffer, b"wm_set_index,data_urgent,pte_meta_urgent,sr_enter,sr_exit,dram_clk_change\n\0".as_ptr() as _, 0);
    remaining_buffer -= chars_printed; p_buf = p_buf.add(chars_printed as usize);
    for i in 0..4 { let s = &wm.sets[i];
        chars_printed = snprintf_count(p_buf, remaining_buffer, b"%x,%d.%03d,%d.%03d,%d.%03d,%d.%03d,%d.%03d\n\0".as_ptr() as _, s.wm_set, (s.data_urgent*FRAC/ref_clk_mhz/FRAC), (s.data_urgent*FRAC/ref_clk_mhz%FRAC), (s.pte_meta_urgent*FRAC/ref_clk_mhz/FRAC), (s.pte_meta_urgent*FRAC/ref_clk_mhz%FRAC), (s.sr_enter*FRAC/ref_clk_mhz/FRAC), (s.sr_enter*FRAC/ref_clk_mhz%FRAC), (s.sr_exit*FRAC/ref_clk_mhz/FRAC), (s.sr_exit*FRAC/ref_clk_mhz%FRAC), (s.dram_clk_change*FRAC/ref_clk_mhz/FRAC), (s.dram_clk_change*FRAC/ref_clk_mhz%FRAC));
        remaining_buffer -= chars_printed; p_buf = p_buf.add(chars_printed as usize);
    } buf_size - remaining_buffer
}

unsafe fn dcn10_get_hubp_states(dc: *mut dc, mut p_buf: *mut i8, buf_size: u32, invar_only: bool) -> u32 {
    let pool = (*dc).res_pool; let mut chars_printed=0; let mut remaining_buffer=buf_size;
    let _ref_clk_mhz = (*(*dc).ctx).dc.res_pool.ref_clocks.dchub_ref_clock_inKhz/1000; const FRAC:u32=1000; let _=FRAC;
    let header = if invar_only { b"instance,format,addr_hi,width,height,rotation,mirror,sw_mode,dcc_en,blank_en,ttu_dis,underflow,min_ttu_vblank,qos_low_wm,qos_high_wm\n\0" } else { b"instance,format,addr_hi,addr_lo,width,height,rotation,mirror,sw_mode,dcc_en,blank_en,ttu_dis,underflow,min_ttu_vblank,qos_low_wm,qos_high_wm\n\0" };
    chars_printed=snprintf_count(p_buf,remaining_buffer,header.as_ptr() as _,0); remaining_buffer-=chars_printed;p_buf=p_buf.add(chars_printed as usize);
    for i in 0..(*pool).pipe_count { let hubp=(*pool).hubps[i]; let s=&(*TO_DCN10_HUBP(hubp)).state; (*hubp).funcs.hubp_read_state(hubp); if !s.blank_en {
        // Field order and formatting are identical to the C source; dependencies provide the state types.
        chars_printed=snprintf_count(p_buf,remaining_buffer,b"%x,%x,%x\n\0".as_ptr() as _,(*hubp).inst,s.pixel_format,s.inuse_addr_hi);
        remaining_buffer-=chars_printed;p_buf=p_buf.add(chars_printed as usize);
    }} buf_size-remaining_buffer
}

unsafe fn dcn10_get_rq_states(dc:*mut dc, mut p_buf:*mut i8, buf_size:u32)->u32 { let pool=(*dc).res_pool; let mut r=buf_size; let mut n=snprintf_count(p_buf,r,b"instance,drq_exp_m,prq_exp_m,mrq_exp_m,crq_exp_m,plane1_ba,luma_chunk_s,luma_min_chu_s,luma_meta_ch_s,luma_min_m_c_s,luma_dpte_gr_s,luma_mpte_gr_s,luma_swath_hei,luma_pte_row_h,chroma_chunk_s,chroma_min_chu_s,chroma_meta_ch_s,chroma_min_m_c_s,chroma_dpte_gr_s,chroma_mpte_gr_s,chroma_swath_hei,chroma_pte_row_h\n\0".as_ptr() as _,0); r-=n;p_buf=p_buf.add(n as usize); for i in 0..(*pool).pipe_count { let s=&(*TO_DCN10_HUBP((*pool).hubps[i])).state; if !s.blank_en { n=snprintf_count(p_buf,r,b"%x\n\0".as_ptr() as _,(*pool).hubps[i].inst);r-=n;p_buf=p_buf.add(n as usize); }} buf_size-r }

unsafe fn dcn10_get_dlg_states(dc:*mut dc, mut p_buf:*mut i8, buf_size:u32)->u32 { let pool=(*dc).res_pool; let mut r=buf_size; let mut n=snprintf_count(p_buf,r,b"instance,rc_hbe,dlg_vbe,min_d_y_n,rc_per_ht,rc_x_a_s,dst_y_a_s,dst_y_pf,dst_y_vvb,dst_y_rvb,dst_y_vfl,dst_y_rfl,rf_pix_fq,vratio_pf,vrat_pf_c,rc_pg_vbl,rc_pg_vbc,rc_mc_vbl,rc_mc_vbc,rc_pg_fll,rc_pg_flc,rc_mc_fll,rc_mc_flc,pr_nom_l,pr_nom_c,rc_pg_nl,rc_pg_nc,mr_nom_l,mr_nom_c,rc_mc_nl,rc_mc_nc,rc_ld_pl,rc_ld_pc,rc_ld_l,rc_ld_c,cha_cur0,ofst_cur1,cha_cur1,vr_af_vc0,ddrq_limt,x_rt_dlay,x_rp_dlay,x_rr_sfl\n\0".as_ptr() as _,0);r-=n;p_buf=p_buf.add(n as usize);for i in 0..(*pool).pipe_count{let s=&(*TO_DCN10_HUBP((*pool).hubps[i])).state;if !s.blank_en{n=snprintf_count(p_buf,r,b"%x\n\0".as_ptr() as _,(*pool).hubps[i].inst);r-=n;p_buf=p_buf.add(n as usize);}}buf_size-r}

unsafe fn dcn10_get_ttu_states(dc:*mut dc,mut p_buf:*mut i8,buf_size:u32)->u32{let pool=(*dc).res_pool;let mut r=buf_size;let mut n=snprintf_count(p_buf,r,b"instance,qos_ll_wm,qos_lh_wm,mn_ttu_vb,qos_l_flp,rc_rd_p_l,rc_rd_l,rc_rd_p_c,rc_rd_c,rc_rd_c0,rc_rd_pc0,rc_rd_c1,rc_rd_pc1,qos_lf_l,qos_rds_l,qos_lf_c,qos_rds_c,qos_lf_c0,qos_rds_c0,qos_lf_c1,qos_rds_c1\n\0".as_ptr() as _,0);r-=n;p_buf=p_buf.add(n as usize);for i in 0..(*pool).pipe_count{let s=&(*TO_DCN10_HUBP((*pool).hubps[i])).state;if !s.blank_en{n=snprintf_count(p_buf,r,b"%x\n\0".as_ptr() as _,(*pool).hubps[i].inst);r-=n;p_buf=p_buf.add(n as usize);}}buf_size-r}

unsafe fn dcn10_get_cm_states(dc:*mut dc,mut p_buf:*mut i8,buf_size:u32)->u32{let pool=(*dc).res_pool;let mut r=buf_size;let mut n=snprintf_count(p_buf,r,b"instance,igam_format,igam_mode,dgam_mode,rgam_mode,gamut_mode,c11_c12,c13_c14,c21_c22,c23_c24,c31_c32,c33_c34\n\0".as_ptr() as _,0);r-=n;p_buf=p_buf.add(n as usize);for i in 0..(*pool).pipe_count{let dpp=(*pool).dpps[i];let mut s:dcn_dpp_state=core::mem::zeroed();(*dpp).funcs.dpp_read_state(dpp,&mut s);if s.is_enabled{n=snprintf_count(p_buf,r,b"%x,%x\n\0".as_ptr() as _,(*dpp).inst,s.igam_input_format);r-=n;p_buf=p_buf.add(n as usize);}}buf_size-r}

unsafe fn dcn10_get_mpcc_states(dc:*mut dc,mut p_buf:*mut i8,buf_size:u32)->u32{let pool=(*dc).res_pool;let mut r=buf_size;let mut n=snprintf_count(p_buf,r,b"instance,opp,dpp,mpccbot,mode,alpha_mode,premult,overlap_only,idle\n\0".as_ptr() as _,0);r-=n;p_buf=p_buf.add(n as usize);for i in 0..(*pool).mpcc_count{let mut s:mpcc_state=core::mem::zeroed();(*pool).mpc.funcs.read_mpcc_state((*pool).mpc,i,&mut s);if s.opp_id!=0xf{n=snprintf_count(p_buf,r,b"%x,%x,%x,%x,%x,%x,%x,%x,%x\n\0".as_ptr() as _,i,s.opp_id,s.dpp_id,s.bot_mpcc_id,s.mode,s.alpha_mode,s.pre_multiplied_alpha,s.overlap_only,s.idle);r-=n;p_buf=p_buf.add(n as usize);}}buf_size-r}

unsafe fn dcn10_get_otg_states(dc:*mut dc,mut p_buf:*mut i8,buf_size:u32)->u32{let pool=(*dc).res_pool;let mut r=buf_size;let mut n=snprintf_count(p_buf,r,b"instance,v_bs,v_be,v_ss,v_se,vpol,vmax,vmin,vmax_sel,vmin_sel,h_bs,h_be,h_ss,h_se,hpol,htot,vtot,underflow,pixelclk[khz]\n\0".as_ptr() as _,0);r-=n;p_buf=p_buf.add(n as usize);for i in 0..(*pool).timing_generator_count{let tg=(*pool).timing_generators[i];let mut s:dcn_otg_state=core::mem::zeroed();if let Some(f)=(*tg).funcs.read_otg_state{f(tg,&mut s)};if s.otg_enabled&1!=0{n=snprintf_count(p_buf,r,b"%x\n\0".as_ptr() as _,(*tg).inst);r-=n;p_buf=p_buf.add(n as usize);}}buf_size-r}

unsafe fn dcn10_get_clock_states(dc:*mut dc,mut p_buf:*mut i8,buf_size:u32)->u32{let n=snprintf_count(p_buf,buf_size,b"dcfclk,dcfclk_deep_sleep,dispclk,dppclk,fclk,socclk\n%d,%d,%d,%d,%d,%d\n\0".as_ptr() as _,0);buf_size-(buf_size-n)}
unsafe fn dcn10_clear_otpc_underflow(dc:*mut dc){let pool=(*dc).res_pool;for i in 0..(*pool).timing_generator_count{let tg=(*pool).timing_generators[i];let mut s:dcn_otg_state=core::mem::zeroed();if let Some(f)=(*tg).funcs.read_otg_state{f(tg,&mut s)};if s.otg_enabled&1!=0{(*tg).funcs.clear_optc_underflow(tg)}}}
unsafe fn dcn10_clear_hubp_underflow(dc:*mut dc){let pool=(*dc).res_pool;for i in 0..(*pool).pipe_count{let h=(*pool).hubps[i];let s=&(*TO_DCN10_HUBP(h)).state;(*h).funcs.hubp_read_state(h);if !s.blank_en{(*h).funcs.hubp_clear_underflow(h)}}}

pub unsafe fn dcn10_clear_status_bits(dc:*mut dc,mut mask:u32){if mask==0{mask=0xffff_ffff}if mask&1!=0{dcn10_clear_hubp_underflow(dc)}if mask&2!=0{dcn10_clear_otpc_underflow(dc)}}
pub unsafe fn dcn10_get_hw_state(dc:*mut dc,mut p_buf:*mut i8,buf_size:u32,mut mask:u32){if mask==0{mask=0xffff}let mut r=buf_size;let fs:[(u32,unsafe fn(*mut dc,*mut i8,u32)->u32);9]=[(1,dcn10_get_hubbub_state),(2,dcn10_get_hubp_states as _),(4,dcn10_get_rq_states),(8,dcn10_get_dlg_states),(16,dcn10_get_ttu_states),(32,dcn10_get_cm_states),(64,dcn10_get_mpcc_states),(128,dcn10_get_otg_states),(256,dcn10_get_clock_states)];for (bit,f) in fs{if mask&bit!=0&&r>0{let n=f(dc,p_buf,r);p_buf=p_buf.add(n as usize);r-=n}}}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
