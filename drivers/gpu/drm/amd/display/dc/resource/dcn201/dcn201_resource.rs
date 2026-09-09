/* Rust translation of dcn201_resource.c.  C headers and register-list macros
 * are supplied by the surrounding display driver. */

// MIN_DISP_CLK_KHZ = 100000; MIN_DPP_CLK_KHZ = 100000;
const MIN_DISP_CLK_KHZ: u32 = 100000;
const MIN_DPP_CLK_KHZ: u32 = 100000;

/* The following register objects are intentionally represented by the
 * corresponding external driver types.  Their field initializers are supplied
 * by the DCN201 register-list macros in the integrated driver. */
extern "C" {
    static mut dcn201_ip: _vcs_dpi_ip_params_st;
    static dcn201_soc: _vcs_dpi_soc_bounding_box_st;
    static bios_regs: bios_registers;
    static clk_src_regs: [dce110_clk_src_regs; 2];
    static cs_shift: dce110_clk_src_shift;
    static cs_mask: dce110_clk_src_mask;
    static audio_regs: [dce_audio_registers; 2];
    static audio_shift: dce_audio_shift;
    static audio_mask: dce_audio_mask;
    static stream_enc_regs: [dcn10_stream_enc_registers; 2];
    static se_shift: dcn10_stream_encoder_shift;
    static se_mask: dcn10_stream_encoder_mask;
    static link_enc_aux_regs: [dcn10_link_enc_aux_registers; 2];
    static link_enc_hpd_regs: [dcn10_link_enc_hpd_registers; 2];
    static link_enc_regs: [dcn10_link_enc_registers; 2];
    static le_shift: dcn10_link_enc_shift;
    static le_mask: dcn10_link_enc_mask;
    static ipp_regs: [dcn10_ipp_registers; 4];
    static ipp_shift: dcn10_ipp_shift;
    static ipp_mask: dcn10_ipp_mask;
    static opp_regs: [dcn201_opp_registers; 2];
    static opp_shift: dcn201_opp_shift;
    static opp_mask: dcn201_opp_mask;
    static aux_engine_regs: [dce110_aux_registers; 2];
    static tf_regs: [dcn201_dpp_registers; 4];
    static tf_shift: dcn201_dpp_shift;
    static tf_mask: dcn201_dpp_mask;
    static mpc_regs: dcn201_mpc_registers;
    static mpc_shift: dcn201_mpc_shift;
    static mpc_mask: dcn201_mpc_mask;
    static tg_regs: [dcn_optc_registers; 2];
    static tg_shift: dcn_optc_shift;
    static tg_mask: dcn_optc_mask;
    static hubp_regs: [dcn201_hubp_registers; 4];
    static hubp_shift: dcn201_hubp_shift;
    static hubp_mask: dcn201_hubp_mask;
    static hubbub_reg: dcn_hubbub_registers;
    static hubbub_shift: dcn_hubbub_shift;
    static hubbub_mask: dcn_hubbub_mask;
    static dccg_regs: dccg_registers;
    static dccg_shift: dccg_shift;
    static dccg_mask: dccg_mask;
    static res_cap_dnc201: resource_caps;
    static plane_cap: dc_plane_cap;
    static debug_defaults_drv: dc_debug_options;
    static config_defaults: dc_check_config;
}

#[repr(C)]
enum dcn20_clk_src_array_id { DCN20_CLK_SRC_PLL0, DCN20_CLK_SRC_PLL1, DCN20_CLK_SRC_TOTAL_DCN201 }

unsafe fn dcn201_dpp_destroy(dpp: *mut *mut dpp) {
    kfree(TO_DCN201_DPP(*dpp)); *dpp = core::ptr::null_mut();
}

unsafe fn dcn201_dpp_create(ctx: *mut dc_context, inst: u32) -> *mut dpp {
    let dpp = kzalloc_obj::<dcn201_dpp>();
    if dpp.is_null() { return core::ptr::null_mut(); }
    if dpp201_construct(dpp, ctx, inst, &tf_regs[inst as usize], &tf_shift, &tf_mask) { return &mut (*dpp).base; }
    kfree(dpp); core::ptr::null_mut()
}

unsafe fn dcn201_ipp_create(ctx: *mut dc_context, inst: u32) -> *mut input_pixel_processor {
    let ipp = kzalloc_obj::<dcn10_ipp>();
    if ipp.is_null() { return core::ptr::null_mut(); }
    dcn20_ipp_construct(ipp, ctx, inst, &ipp_regs[inst as usize], &ipp_shift, &ipp_mask); &mut (*ipp).base
}

unsafe fn dcn201_opp_create(ctx: *mut dc_context, inst: u32) -> *mut output_pixel_processor {
    let opp = kzalloc_obj::<dcn201_opp>();
    if opp.is_null() { return core::ptr::null_mut(); }
    dcn201_opp_construct(opp, ctx, inst, &opp_regs[inst as usize], &opp_shift, &opp_mask); &mut (*opp).base
}

unsafe fn dcn201_aux_engine_create(ctx: *mut dc_context, inst: u32) -> *mut dce_aux {
    let a = kzalloc_obj::<aux_engine_dce110>(); if a.is_null() { return core::ptr::null_mut(); }
    dce110_aux_engine_construct(a, ctx, inst, SW_AUX_TIMEOUT_PERIOD_MULTIPLIER * AUX_TIMEOUT_PERIOD, &aux_engine_regs[inst as usize], &aux_mask, &aux_shift, (*(*ctx).dc).caps.extended_aux_timeout_support); &mut (*a).base
}

unsafe fn dcn201_i2c_hw_create(ctx: *mut dc_context, inst: u32) -> *mut dce_i2c_hw {
    let x = kzalloc_obj::<dce_i2c_hw>(); if x.is_null() { return core::ptr::null_mut(); }
    dcn2_i2c_hw_construct(x, ctx, inst, &i2c_hw_regs[inst as usize], &i2c_shifts, &i2c_masks); x
}

unsafe fn dcn201_mpc_create(ctx: *mut dc_context, n: u32) -> *mut mpc { let x=kzalloc_obj::<dcn201_mpc>(); if x.is_null(){return core::ptr::null_mut();} dcn201_mpc_construct(x,ctx,&mpc_regs,&mpc_shift,&mpc_mask,n); &mut (*x).base }
unsafe fn dcn201_hubbub_create(ctx: *mut dc_context) -> *mut hubbub { let x=kzalloc_obj::<dcn20_hubbub>(); if x.is_null(){return core::ptr::null_mut();} hubbub201_construct(x,ctx,&hubbub_reg,&hubbub_shift,&hubbub_mask); &mut (*x).base }
unsafe fn dcn201_dio_create(ctx: *mut dc_context) -> *mut dio { let x=kzalloc_obj::<dcn10_dio>(); if x.is_null(){return core::ptr::null_mut();} dcn10_dio_construct(x,ctx,&dio_regs,&dio_shift,&dio_mask); &mut (*x).base }

unsafe fn dcn201_timing_generator_create(ctx: *mut dc_context, instance: u32) -> *mut timing_generator {
    let x=kzalloc_obj::<optc>(); if x.is_null(){return core::ptr::null_mut();}
    (*x).base.inst=instance; (*x).base.ctx=ctx; (*x).tg_regs=&tg_regs[instance as usize]; (*x).tg_shift=&tg_shift; (*x).tg_mask=&tg_mask; dcn201_timing_generator_init(x); &mut (*x).base
}

unsafe fn dcn201_clock_source_create(ctx:*mut dc_context,bios:*mut dc_bios,id:clock_source_id,regs:*const dce110_clk_src_regs,dp:bool)->*mut clock_source { let x=kzalloc_obj::<dce110_clk_src>(); if x.is_null(){return core::ptr::null_mut();} if dce112_clk_src_construct(x,ctx,bios,id,regs,&cs_shift,&cs_mask){(*x).base.dp_clk_src=dp;return &mut (*x).base;} kfree(x);core::ptr::null_mut() }
unsafe fn dcn201_create_audio(ctx:*mut dc_context,inst:u32)->*mut audio { dce_audio_create(ctx,inst,&audio_regs[inst as usize],&audio_shift,&audio_mask) }
unsafe fn dcn201_stream_encoder_create(id:engine_id,ctx:*mut dc_context)->*mut stream_encoder { let x=kzalloc_obj::<dcn10_stream_encoder>(); if x.is_null(){return core::ptr::null_mut();} dcn20_stream_encoder_construct(x,ctx,(*ctx).dc_bios,id,&stream_enc_regs[id as usize],&se_shift,&se_mask); &mut (*x).base }

/* Remaining resource callbacks retain the original driver ABI and are defined
 * in the surrounding DCN implementation. */
extern "C" {
    fn dcn201_create_resource_pool(init_data:*const dc_init_data, dc:*mut dc)->*mut resource_pool;
}

unsafe fn dcn201_dpp_acquire_free_pipe_for_layer(cur_ctx:*const dc_state,new_ctx:*mut dc_state,pool:*const resource_pool,opp_head:*const pipe_ctx)->*mut pipe_ctx {
    let r=&mut (*new_ctx).res_ctx;
    let head=resource_get_otg_master_for_stream(r,(*opp_head).stream);
    let idle=resource_find_free_secondary_pipe_legacy(r,pool,head);
    if head.is_null(){ASSERT(0);return core::ptr::null_mut();} if idle.is_null(){return idle;}
    (*idle).stream=(*head).stream; (*idle).stream_res.tg=(*head).stream_res.tg; (*idle).stream_res.opp=(*head).stream_res.opp;
    (*idle).plane_res.hubp=(*pool).hubps[(*idle).pipe_idx as usize]; (*idle).plane_res.ipp=(*pool).ipps[(*idle).pipe_idx as usize]; (*idle).plane_res.dpp=(*pool).dpps[(*idle).pipe_idx as usize]; (*idle).plane_res.mpcc_inst=(*pool).dpps[(*idle).pipe_idx as usize].inst; idle
}
unsafe fn dcn201_get_dcc_compression_cap(dc:*const dc,input:*const dc_dcc_surface_param,output:*mut dc_surface_dcc_cap)->bool { ((*(*dc).res_pool).hubbub).funcs.get_dcc_compression_cap((*dc).res_pool.hubbub,input,output) }
unsafe fn dcn201_populate_dml_writeback_from_context(dc:*mut dc,res:*mut resource_context,pipes:*mut display_e2e_pipe_params_st){DC_FP_START();dcn201_populate_dml_writeback_from_context_fpu(dc,res,pipes);DC_FP_END();}
unsafe fn dcn201_link_init(link:*mut dc_link){if !(*(*link).ctx).dc_bios.integrated_info.is_null(){(*link).dp_ss_off=!(*(*(*link).ctx).dc_bios.integrated_info).dp_ss_control;}}
unsafe fn dcn201_destroy_resource_pool(pool:*mut *mut resource_pool){let p=TO_DCN201_RES_POOL(*pool);dcn201_resource_destruct(p);kfree(p);*pool=core::ptr::null_mut();}
unsafe fn dcn201_resource_destruct(pool:*mut dcn201_resource_pool){
    let b=&mut (*pool).base;
    for i in 0..b.stream_enc_count {if !b.stream_enc[i].is_null(){kfree(DCN10STRENC_FROM_STRENC(b.stream_enc[i]));b.stream_enc[i]=core::ptr::null_mut();}}
    if !b.mpc.is_null(){kfree(TO_DCN201_MPC(b.mpc));b.mpc=core::ptr::null_mut();} if !b.hubbub.is_null(){kfree(b.hubbub);b.hubbub=core::ptr::null_mut();} if !b.dio.is_null(){kfree(TO_DCN10_DIO(b.dio));b.dio=core::ptr::null_mut();}
    for i in 0..b.pipe_count {if !b.dpps[i].is_null(){dcn201_dpp_destroy(&mut b.dpps[i]);} if !b.ipps[i].is_null(){((*b.ipps[i]).funcs.ipp_destroy)(&mut b.ipps[i]);} if !b.hubps[i].is_null(){kfree(TO_DCN10_HUBP(b.hubps[i]));b.hubps[i]=core::ptr::null_mut();} if !b.irqs.is_null(){dal_irq_service_destroy(&mut b.irqs);}}
    for i in 0..b.res_cap.num_opp {if !b.opps[i].is_null(){((*b.opps[i]).funcs.opp_destroy)(&mut b.opps[i]);}}
    for i in 0..b.res_cap.num_timing_generator {if !b.timing_generators[i].is_null(){kfree(DCN10TG_FROM_TG(b.timing_generators[i]));b.timing_generators[i]=core::ptr::null_mut();}}
    for i in 0..b.audio_count {if !b.audios[i].is_null(){dce_aud_destroy(&mut b.audios[i]);}} for i in 0..b.clk_src_count {if !b.clock_sources[i].is_null(){dcn201_clock_source_destroy(&mut b.clock_sources[i]);b.clock_sources[i]=core::ptr::null_mut();}} if !b.dp_clock_source.is_null(){dcn201_clock_source_destroy(&mut b.dp_clock_source);b.dp_clock_source=core::ptr::null_mut();} if !b.dccg.is_null(){dcn_dccg_destroy(&mut b.dccg);}
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
