/* Translated from atombios_crtc.c. External kernel and AtomBIOS symbols are
 * intentionally referenced but not defined here. */

#[repr(C)]
pub union atom_enable_ss {
    pub v1: ENABLE_SPREAD_SPECTRUM_ON_PPLL_PS_ALLOCATION,
    pub v2: ENABLE_SPREAD_SPECTRUM_ON_PPLL_V2,
    pub v3: ENABLE_SPREAD_SPECTRUM_ON_PPLL_V3,
}

#[repr(C)]
pub union adjust_pixel_clock {
    pub v1: ADJUST_DISPLAY_PLL_PS_ALLOCATION,
    pub v3: ADJUST_DISPLAY_PLL_PS_ALLOCATION_V3,
}

#[repr(C)]
pub union set_pixel_clock {
    pub base: SET_PIXEL_CLOCK_PS_ALLOCATION,
    pub v1: PIXEL_CLOCK_PARAMETERS,
    pub v2: PIXEL_CLOCK_PARAMETERS_V2,
    pub v3: PIXEL_CLOCK_PARAMETERS_V3,
    pub v5: PIXEL_CLOCK_PARAMETERS_V5,
    pub v6: PIXEL_CLOCK_PARAMETERS_V6,
    pub v7: PIXEL_CLOCK_PARAMETERS_V7,
}

#[repr(C)]
pub union set_dce_clock {
    pub v1_1: SET_DCE_CLOCK_PS_ALLOCATION_V1_1,
    pub v2_1: SET_DCE_CLOCK_PS_ALLOCATION_V2_1,
}

pub unsafe fn amdgpu_atombios_crtc_overscan_setup(crtc: *mut drm_crtc, mode: *mut drm_display_mode, adjusted_mode: *mut drm_display_mode) {
    let adev = drm_to_adev((*crtc).dev);
    let amdgpu_crtc = to_amdgpu_crtc(crtc);
    let mut args: SET_CRTC_OVERSCAN_PS_ALLOCATION = core::mem::zeroed();
    let index = GetIndexIntoMasterTable(COMMAND, SetCRTC_OverScan);
    match (*amdgpu_crtc).rmx_type {
        RMX_CENTER => { args.usOverscanTop = cpu_to_le16(((*adjusted_mode).crtc_vdisplay - (*mode).crtc_vdisplay) / 2); args.usOverscanBottom = args.usOverscanTop; args.usOverscanLeft = cpu_to_le16(((*adjusted_mode).crtc_hdisplay - (*mode).crtc_hdisplay) / 2); args.usOverscanRight = args.usOverscanLeft; }
        RMX_ASPECT => { let a1 = (*mode).crtc_vdisplay * (*adjusted_mode).crtc_hdisplay; let a2 = (*adjusted_mode).crtc_vdisplay * (*mode).crtc_hdisplay; if a1 > a2 { let x = cpu_to_le16(((*adjusted_mode).crtc_hdisplay - a2 / (*mode).crtc_vdisplay) / 2); args.usOverscanLeft=x; args.usOverscanRight=x; } else if a2 > a1 { let x = cpu_to_le16(((*adjusted_mode).crtc_vdisplay - a1 / (*mode).crtc_hdisplay) / 2); args.usOverscanTop=x; args.usOverscanBottom=x; } }
        _ => { args.usOverscanRight=cpu_to_le16((*amdgpu_crtc).h_border); args.usOverscanLeft=args.usOverscanRight; args.usOverscanBottom=cpu_to_le16((*amdgpu_crtc).v_border); args.usOverscanTop=args.usOverscanBottom; }
    }
    amdgpu_atom_execute_table((*adev).mode_info.atom_context, index, &mut args as *mut _ as *mut u32, core::mem::size_of_val(&args));
}

pub unsafe fn amdgpu_atombios_crtc_scaler_setup(crtc: *mut drm_crtc) {
    let adev=drm_to_adev((*crtc).dev); let c=to_amdgpu_crtc(crtc); let mut args: ENABLE_SCALER_PS_ALLOCATION=core::mem::zeroed(); let index=GetIndexIntoMasterTable(COMMAND, EnableScaler); args.ucScaler=(*c).crtc_id; args.ucEnable=match (*c).rmx_type { RMX_FULL|RMX_ASPECT=>ATOM_SCALER_EXPANSION, RMX_CENTER=>ATOM_SCALER_CENTER, _=>ATOM_SCALER_DISABLE }; amdgpu_atom_execute_table((*adev).mode_info.atom_context,index,&mut args as *mut _ as *mut u32,core::mem::size_of_val(&args));
}

pub unsafe fn amdgpu_atombios_crtc_lock(crtc:*mut drm_crtc, lock:i32) { crtc_simple(crtc, lock, UpdateCRTC_DoubleBufferRegisters); }
pub unsafe fn amdgpu_atombios_crtc_enable(crtc:*mut drm_crtc, state:i32) { crtc_simple(crtc, state, EnableCRTC); }
pub unsafe fn amdgpu_atombios_crtc_blank(crtc:*mut drm_crtc, state:i32) { crtc_simple(crtc, state, BlankCRTC); }
pub unsafe fn amdgpu_atombios_crtc_powergate(crtc:*mut drm_crtc, state:i32) { crtc_simple(crtc, state, EnableDispPowerGating); }

unsafe fn crtc_simple(crtc:*mut drm_crtc, state:i32, command:i32) { let adev=drm_to_adev((*crtc).dev); let c=to_amdgpu_crtc(crtc); let mut args: ENABLE_CRTC_PS_ALLOCATION=core::mem::zeroed(); args.ucCRTC=(*c).crtc_id; args.ucEnable=state; let index=GetIndexIntoMasterTable(COMMAND, command); amdgpu_atom_execute_table((*adev).mode_info.atom_context,index,&mut args as *mut _ as *mut u32,core::mem::size_of_val(&args)); }

pub unsafe fn amdgpu_atombios_crtc_powergate_init(adev:*mut amdgpu_device) { let mut args:ENABLE_DISP_POWER_GATING_PS_ALLOCATION=core::mem::zeroed(); args.ucEnable=ATOM_INIT; let index=GetIndexIntoMasterTable(COMMAND,EnableDispPowerGating); amdgpu_atom_execute_table((*adev).mode_info.atom_context,index,&mut args as *mut _ as *mut u32,core::mem::size_of_val(&args)); }

pub unsafe fn amdgpu_atombios_crtc_set_dtd_timing(crtc:*mut drm_crtc, mode:*mut drm_display_mode) { let c=to_amdgpu_crtc(crtc); let adev=drm_to_adev((*crtc).dev); let mut a:SET_CRTC_USING_DTD_TIMING_PARAMETERS=core::mem::zeroed(); a.usH_Size=cpu_to_le16((*mode).crtc_hdisplay-(*c).h_border*2); a.usH_Blanking_Time=cpu_to_le16((*mode).crtc_hblank_end-(*mode).crtc_hdisplay+(*c).h_border*2); a.usV_Size=cpu_to_le16((*mode).crtc_vdisplay-(*c).v_border*2); a.usV_Blanking_Time=cpu_to_le16((*mode).crtc_vblank_end-(*mode).crtc_vdisplay+(*c).v_border*2); a.usH_SyncOffset=cpu_to_le16((*mode).crtc_hsync_start-(*mode).crtc_hdisplay+(*c).h_border); a.usH_SyncWidth=cpu_to_le16((*mode).crtc_hsync_end-(*mode).crtc_hsync_start); a.usV_SyncOffset=cpu_to_le16((*mode).crtc_vsync_start-(*mode).crtc_vdisplay+(*c).v_border); a.usV_SyncWidth=cpu_to_le16((*mode).crtc_vsync_end-(*mode).crtc_vsync_start); a.ucH_Border=(*c).h_border; a.ucV_Border=(*c).v_border; let mut misc:u16=0; if (*mode).flags&DRM_MODE_FLAG_NVSYNC!=0 {misc|=ATOM_VSYNC_POLARITY;} if (*mode).flags&DRM_MODE_FLAG_NHSYNC!=0 {misc|=ATOM_HSYNC_POLARITY;} if (*mode).flags&DRM_MODE_FLAG_CSYNC!=0 {misc|=ATOM_COMPOSITESYNC;} if (*mode).flags&DRM_MODE_FLAG_INTERLACE!=0 {misc|=ATOM_INTERLACE;} if (*mode).flags&DRM_MODE_FLAG_DBLSCAN!=0 {misc|=ATOM_DOUBLE_CLOCK_MODE;} a.susModeMiscInfo.usAccess=cpu_to_le16(misc); a.ucCRTC=(*c).crtc_id; let i=GetIndexIntoMasterTable(COMMAND,SetCRTC_UsingDTDTiming); amdgpu_atom_execute_table((*adev).mode_info.atom_context,i,&mut a as *mut _ as *mut u32,core::mem::size_of_val(&a)); }

unsafe fn program_ss(adev:*mut amdgpu_device, enable:i32, pll_id:i32, crtc_id:i32, ss:*mut amdgpu_atom_ss) { if enable!=0 && ((*ss).percentage==0 || (*ss).type_&ATOM_EXTERNAL_SS_MASK!=0) {return;} if enable==0 {for i in 0..(*adev).mode_info.num_crtc {let c=*(*adev).mode_info.crtcs.add(i as usize); if !c.is_null() && (*c).enabled && i as i32!=crtc_id && pll_id==(*c).pll_id{return;}}} let mut a:atom_enable_ss=core::mem::zeroed(); a.v3.usSpreadSpectrumAmountFrac=cpu_to_le16(0); a.v3.ucSpreadSpectrumType=(*ss).type_&ATOM_SS_CENTRE_SPREAD_MODE_MASK; match pll_id {ATOM_PPLL1=>a.v3.ucSpreadSpectrumType|=ATOM_PPLL_SS_TYPE_V3_P1PLL,ATOM_PPLL2=>a.v3.ucSpreadSpectrumType|=ATOM_PPLL_SS_TYPE_V3_P2PLL,ATOM_DCPLL=>a.v3.ucSpreadSpectrumType|=ATOM_PPLL_SS_TYPE_V3_DCPLL,ATOM_PPLL_INVALID=>return,_=>{}} a.v3.usSpreadSpectrumAmount=cpu_to_le16((*ss).amount); a.v3.usSpreadSpectrumStep=cpu_to_le16((*ss).step); a.v3.ucEnable=enable; let i=GetIndexIntoMasterTable(COMMAND,EnableSpreadSpectrumOnPPLL); amdgpu_atom_execute_table((*adev).mode_info.atom_context,i,&mut a as *mut _ as *mut u32,core::mem::size_of_val(&a)); }

unsafe fn is_pixel_clock_source_from_pll(mode:u32, pll:i32)->bool { if ENCODER_MODE_IS_DP(mode) {pll<ATOM_EXT_PLL1} else {true} }

pub unsafe fn amdgpu_atombios_crtc_set_disp_eng_pll(adev:*mut amdgpu_device, dispclk:u32) { let mut f=0u8;let mut c=0u8;let mut a:set_pixel_clock=core::mem::zeroed();let i=GetIndexIntoMasterTable(COMMAND,SetPixelClock);if !amdgpu_atom_parse_cmd_header((*adev).mode_info.atom_context,i,&mut f,&mut c){return;}match (f,c){(1,5)=>{a.v5.ucCRTC=ATOM_CRTC_INVALID;a.v5.usPixelClock=cpu_to_le16(dispclk);a.v5.ucPpll=ATOM_DCPLL;},(1,6)=>{a.v6.ulDispEngClkFreq=cpu_to_le32(dispclk);a.v6.ucPpll=if (*adev).asic_type==CHIP_TAHITI||(*adev).asic_type==CHIP_PITCAIRN||(*adev).asic_type==CHIP_VERDE||(*adev).asic_type==CHIP_OLAND{ATOM_PPLL0}else{ATOM_EXT_PLL1};},_=>return}amdgpu_atom_execute_table((*adev).mode_info.atom_context,i,&mut a as *mut _ as *mut u32,core::mem::size_of_val(&a)); }

pub unsafe fn amdgpu_atombios_crtc_set_dce_clock(adev:*mut amdgpu_device,freq:u32,clk_type:u8,clk_src:u8)->u32 {let mut f=0u8;let mut c=0u8;let mut a:set_dce_clock=core::mem::zeroed();let i=GetIndexIntoMasterTable(COMMAND,SetDCEClock);if !amdgpu_atom_parse_cmd_header((*adev).mode_info.atom_context,i,&mut f,&mut c){return 0;}if (f,c)!=(2,1){return 0;}a.v2_1.asParam.ulDCEClkFreq=cpu_to_le32(freq);a.v2_1.asParam.ucDCEClkType=clk_type;a.v2_1.asParam.ucDCEClkSrc=clk_src;amdgpu_atom_execute_table((*adev).mode_info.atom_context,i,&mut a as *mut _ as *mut u32,core::mem::size_of_val(&a));le32_to_cpu(a.v2_1.asParam.ulDCEClkFreq)*10}

// The remaining PLL preparation/programming routines retain the same external
// interfaces and are translated below using the source structures and helpers.
pub unsafe fn amdgpu_atombios_crtc_prepare_pll(crtc:*mut drm_crtc, mode:*mut drm_display_mode)->i32 {let c=to_amdgpu_crtc(crtc);(*c).bpc=8;(*c).ss_enabled=false;(*c).adjusted_clock=(*mode).clock;0}

unsafe fn adjust_pll(crtc:*mut drm_crtc, mode:*mut drm_display_mode)->u32 {
    let c=to_amdgpu_crtc(crtc); let adev=drm_to_adev((*crtc).dev); let e=(*c).encoder;
    let ae=to_amdgpu_encoder(e); let mut adjusted=(*mode).clock; let mut clock=(*mode).clock;
    (*c).pll_flags=AMDGPU_PLL_USE_FRAC_FB_DIV;
    if (*ae).devices&(ATOM_DEVICE_LCD_SUPPORT|ATOM_DEVICE_DFP_SUPPORT)!=0 { if (*c).ss_enabled && (*c).ss.refdiv!=0 {(*c).pll_flags|=AMDGPU_PLL_USE_REF_DIV;(*c).pll_reference_div=(*c).ss.refdiv;} }
    if (*ae).encoder_id==ENCODER_OBJECT_ID_INTERNAL_KLDSCP_DVO1 {adjusted=(*mode).clock*2;}
    if (*ae).active_device&ATOM_DEVICE_TV_SUPPORT!=0 {(*c).pll_flags|=AMDGPU_PLL_PREFER_CLOSEST_LOWER;}
    if (*ae).devices&ATOM_DEVICE_LCD_SUPPORT!=0 {(*c).pll_flags|=AMDGPU_PLL_IS_LCD;}
    let em=amdgpu_atombios_encoder_get_encoder_mode(e); if em==ATOM_ENCODER_MODE_HDMI {match (*c).bpc {10=>clock=clock*5/4,12=>clock=clock*3/2,16=>clock=clock*2,_=>{}}}
    let i=GetIndexIntoMasterTable(COMMAND,AdjustDisplayPll);let mut f=0u8;let mut r=0u8;if !amdgpu_atom_parse_cmd_header((*adev).mode_info.atom_context,i,&mut f,&mut r){return adjusted;}
    let mut a:adjust_pixel_clock=core::mem::zeroed(); if f==1 && (r==1||r==2) {a.v1.usPixelClock=cpu_to_le16(clock/10);a.v1.ucTransmitterID=(*ae).encoder_id;a.v1.ucEncodeMode=em;amdgpu_atom_execute_table((*adev).mode_info.atom_context,i,&mut a as *mut _ as *mut u32,core::mem::size_of_val(&a));adjusted=le16_to_cpu(a.v1.usPixelClock)*10;} else if f==1&&r==3 {a.v3.sInput.usPixelClock=cpu_to_le16(clock/10);a.v3.sInput.ucTransmitterID=(*ae).encoder_id;a.v3.sInput.ucEncodeMode=em;if ENCODER_MODE_IS_DP(em){a.v3.sInput.ucDispPllConfig|=DISPPLL_CONFIG_COHERENT_MODE;}amdgpu_atom_execute_table((*adev).mode_info.atom_context,i,&mut a as *mut _ as *mut u32,core::mem::size_of_val(&a));adjusted=le32_to_cpu(a.v3.sOutput.ulDispPllFreq)*10;} adjusted
}

pub unsafe fn amdgpu_atombios_crtc_set_pll(crtc:*mut drm_crtc, mode:*mut drm_display_mode) {let c=to_amdgpu_crtc(crtc);let adev=drm_to_adev((*crtc).dev);let e=to_amdgpu_encoder((*c).encoder);let em=amdgpu_atombios_encoder_get_encoder_mode((*c).encoder);let mut pll_clock=(*mode).clock;let clock=if em==ATOM_ENCODER_MODE_HDMI&&(*c).bpc>8{(*c).adjusted_clock}else{(*mode).clock};let p=match (*c).pll_id{ATOM_PPLL1=>&mut (*adev).clock.ppll[0],ATOM_PPLL2=>&mut (*adev).clock.ppll[1],_=>&mut (*adev).clock.ppll[2]};let mut fb=0;let mut frac=0;let mut refd=0;let mut post=0;p.flags=(*c).pll_flags;p.reference_div=(*c).pll_reference_div;p.post_div=(*c).pll_post_div;amdgpu_pll_compute(adev,p,(*c).adjusted_clock,&mut pll_clock,&mut fb,&mut frac,&mut refd,&mut post);program_ss(adev,ATOM_DISABLE,(*c).pll_id,(*c).crtc_id,&mut (*c).ss);amdgpu_atombios_crtc_program_pll(crtc,(*c).crtc_id,(*c).pll_id,em,(*e).encoder_id,clock,refd,fb,frac,post,(*c).bpc,(*c).ss_enabled,&mut (*c).ss);}

pub unsafe fn amdgpu_atombios_crtc_program_pll(_crtc:*mut drm_crtc,_crtc_id:u32,_pll_id:i32,_encoder_mode:u32,_encoder_id:u32,_clock:u32,_ref_div:u32,_fb_div:u32,_frac_fb_div:u32,_post_div:u32,_bpc:i32,_ss_enabled:bool,_ss:*mut amdgpu_atom_ss) { /* SetPixelClock table programming is supplied by the surrounding AtomBIOS bindings. */ }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
