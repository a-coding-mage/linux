/* Copyright 2023 Advanced Micro Devices, Inc. */
/* Translated from dcn401_dccg.c. External types, constants, and register
 * access macros are supplied by the surrounding driver bindings. */

// Dependency intent: reg_helper.h, core_types.h, dcn401_dccg.h,
// dcn31/dcn31_dccg.h, and dcn20/dcn20_dccg.h.

#[inline]
unsafe fn to_dcn_dccg(dccg: *mut dccg) -> *mut dcn_dccg {
    container_of!(dccg, dcn_dccg, base)
}

unsafe fn dcn401_set_dppclk_enable(dccg: *mut dccg, dpp_inst: u32, enable: u32) {
    let dccg_dcn = to_dcn_dccg(dccg);
    match dpp_inst {
        0 => REG_UPDATE!(dccg_dcn, DPPCLK_CTRL, DPPCLK0_EN, enable),
        1 => REG_UPDATE!(dccg_dcn, DPPCLK_CTRL, DPPCLK1_EN, enable),
        2 => REG_UPDATE!(dccg_dcn, DPPCLK_CTRL, DPPCLK2_EN, enable),
        3 => REG_UPDATE!(dccg_dcn, DPPCLK_CTRL, DPPCLK3_EN, enable),
        _ => {}
    }
}

pub unsafe fn dccg401_update_dpp_dto(dccg: *mut dccg, dpp_inst: i32, req_dppclk: i32) {
    let dccg_dcn = to_dcn_dccg(dccg);
    if (*dccg).ref_dppclk != 0 && req_dppclk != 0 {
        let ref_dppclk = (*dccg).ref_dppclk;
        let modulo = 0xff;
        let mut phase = (modulo * req_dppclk + ref_dppclk - 1) / ref_dppclk;
        if phase > 0xff { ASSERT!(false); phase = 0xff; }
        REG_SET_2!(dccg_dcn, DPPCLK_DTO_PARAM[dpp_inst], 0,
            DPPCLK0_DTO_PHASE, phase, DPPCLK0_DTO_MODULO, modulo);
        dcn401_set_dppclk_enable(dccg, dpp_inst as u32, 1);
    } else { dcn401_set_dppclk_enable(dccg, dpp_inst as u32, 0); }
    (*dccg).pipe_dppclk_khz[dpp_inst as usize] = req_dppclk;
}

unsafe fn dccg401_wait_for_dentist_change_done(dccg: *mut dccg) {
    let dccg_dcn = to_dcn_dccg(dccg);
    let v = REG_READ!(dccg_dcn, DENTIST_DISPCLK_CNTL);
    REG_WRITE!(dccg_dcn, DENTIST_DISPCLK_CNTL, v);
    REG_WAIT!(dccg_dcn, DENTIST_DISPCLK_CNTL, DENTIST_DISPCLK_CHG_DONE, 1, 50, 2000);
}

pub unsafe fn dccg401_get_pixel_rate_div(dccg: *mut dccg, otg_inst: u32,
    tmds_div: *mut u32, dp_dto_int: *mut u32) {
    let d = to_dcn_dccg(dccg); let mut v = PIXEL_RATE_DIV_NA;
    match otg_inst {
        0 => REG_GET_2!(d, OTG_PIXEL_RATE_DIV, OTG0_TMDS_PIXEL_RATE_DIV, &mut v, DPDTO0_INT, dp_dto_int),
        1 => REG_GET_2!(d, OTG_PIXEL_RATE_DIV, OTG1_TMDS_PIXEL_RATE_DIV, &mut v, DPDTO1_INT, dp_dto_int),
        2 => REG_GET_2!(d, OTG_PIXEL_RATE_DIV, OTG2_TMDS_PIXEL_RATE_DIV, &mut v, DPDTO2_INT, dp_dto_int),
        3 => REG_GET_2!(d, OTG_PIXEL_RATE_DIV, OTG3_TMDS_PIXEL_RATE_DIV, &mut v, DPDTO3_INT, dp_dto_int),
        _ => { BREAK_TO_DEBUGGER!(); return; }
    }
    *tmds_div = if v == 0 { PIXEL_RATE_DIV_BY_2 } else { PIXEL_RATE_DIV_BY_4 };
}

pub unsafe fn dccg401_set_pixel_rate_div(dccg: *mut dccg, otg_inst: u32,
    tmds_div: pixel_rate_div, _unused: pixel_rate_div) {
    let d = to_dcn_dccg(dccg); let mut cur = PIXEL_RATE_DIV_NA; let mut dp = 0;
    if tmds_div != PIXEL_RATE_DIV_BY_2 && tmds_div != PIXEL_RATE_DIV_BY_4 { return; }
    dccg401_get_pixel_rate_div(dccg, otg_inst, &mut cur, &mut dp);
    if tmds_div == cur { return; }
    let val = if tmds_div == PIXEL_RATE_DIV_BY_4 { 1 } else { 0 };
    match otg_inst {
        0 => { REG_UPDATE!(d, OTG_PIXEL_RATE_DIV, OTG0_TMDS_PIXEL_RATE_DIV, val); dccg401_wait_for_dentist_change_done(dccg); }
        1 => { REG_UPDATE!(d, OTG_PIXEL_RATE_DIV, OTG1_TMDS_PIXEL_RATE_DIV, val); dccg401_wait_for_dentist_change_done(dccg); }
        2 => { REG_UPDATE!(d, OTG_PIXEL_RATE_DIV, OTG2_TMDS_PIXEL_RATE_DIV, val); dccg401_wait_for_dentist_change_done(dccg); }
        3 => { REG_UPDATE!(d, OTG_PIXEL_RATE_DIV, OTG3_TMDS_PIXEL_RATE_DIV, val); dccg401_wait_for_dentist_change_done(dccg); }
        _ => { BREAK_TO_DEBUGGER!(); }
    }
}

pub unsafe fn dccg401_set_dtbclk_p_src(dccg: *mut dccg, src: streamclk_source, otg_inst: u32) {
    let d = to_dcn_dccg(dccg); let sel = if src == DTBCLK0 { 2 } else { 0 };
    match otg_inst {
        0 => if src == REFCLK { REG_UPDATE!(d, DTBCLK_P_CNTL, DTBCLK_P0_EN, 0) } else { REG_UPDATE_2!(d, DTBCLK_P_CNTL, DTBCLK_P0_SRC_SEL, sel, DTBCLK_P0_EN, 1) },
        1 => if src == REFCLK { REG_UPDATE!(d, DTBCLK_P_CNTL, DTBCLK_P1_EN, 0) } else { REG_UPDATE_2!(d, DTBCLK_P_CNTL, DTBCLK_P1_SRC_SEL, sel, DTBCLK_P1_EN, 1) },
        2 => if src == REFCLK { REG_UPDATE!(d, DTBCLK_P_CNTL, DTBCLK_P2_EN, 0) } else { REG_UPDATE_2!(d, DTBCLK_P_CNTL, DTBCLK_P2_SRC_SEL, sel, DTBCLK_P2_EN, 1) },
        3 => if src == REFCLK { REG_UPDATE!(d, DTBCLK_P_CNTL, DTBCLK_P3_EN, 0) } else { REG_UPDATE_2!(d, DTBCLK_P_CNTL, DTBCLK_P3_SRC_SEL, sel, DTBCLK_P3_EN, 1) },
        _ => { BREAK_TO_DEBUGGER!(); }
    }
}

pub unsafe fn dccg401_get_dccg_ref_freq(_dccg: *mut dccg, xtalin_freq_in_khz: u32, out: *mut u32) { *out = xtalin_freq_in_khz; }

unsafe fn dccg401_otg_add_pixel(dccg: *mut dccg, i: u32) { REG_UPDATE!(to_dcn_dccg(dccg), OTG_PIXEL_RATE_CNTL[i], OTG_ADD_PIXEL[i], 1); }
unsafe fn dccg401_otg_drop_pixel(dccg: *mut dccg, i: u32) { REG_UPDATE!(to_dcn_dccg(dccg), OTG_PIXEL_RATE_CNTL[i], OTG_DROP_PIXEL[i], 1); }

pub unsafe fn dccg401_set_hdmistreamclk(dccg: *mut dccg, src: streamclk_source, otg_inst: u32) {
    let d = to_dcn_dccg(dccg);
    if src == REFCLK { REG_UPDATE!(d, HDMISTREAMCLK_CNTL, HDMISTREAMCLK0_EN, 0); }
    else { REG_UPDATE_2!(d, HDMISTREAMCLK_CNTL, HDMISTREAMCLK0_EN, 1, HDMISTREAMCLK0_SRC_SEL, otg_inst); }
}

pub unsafe fn dccg401_enable_hdmicharclk(dccg: *mut dccg, hpo_inst: i32, phypll_inst: i32) {
    let d = to_dcn_dccg(dccg); ASSERT!(hpo_inst >= 0 && phypll_inst >= 0);
    REG_UPDATE_2!(d, HDMICHARCLK_CLOCK_CNTL[hpo_inst], HDMICHARCLK0_EN, 1, HDMICHARCLK0_SRC_SEL, phypll_inst);
    dccg401_set_physymclk(dccg, phypll_inst, PHYSYMCLK_FORCE_SRC_PHYD18CLK, true);
}
pub unsafe fn dccg401_disable_hdmicharclk(dccg: *mut dccg, hpo_inst: i32) {
    let d = to_dcn_dccg(dccg); ASSERT!(hpo_inst >= 0); REG_WRITE!(d, HDMICHARCLK_CLOCK_CNTL[hpo_inst], 0);
}
unsafe fn dccg401_disable_hdmistreamclk(dccg: *mut dccg) { REG_UPDATE_2!(to_dcn_dccg(dccg), HDMISTREAMCLK_CNTL, HDMISTREAMCLK0_EN, 0, HDMISTREAMCLK0_SRC_SEL, 0); }

// The following routines retain the four hardware-instance branches of the C implementation.
pub unsafe fn dccg401_set_physymclk(dccg: *mut dccg, phy_inst: i32, src: physymclk_clock_source, force: bool) {
    let d = to_dcn_dccg(dccg); let (reg,en,sel,gate) = match phy_inst { 0=>(PHYASYMCLK_CLOCK_CNTL,PHYASYMCLK_EN,PHYASYMCLK_SRC_SEL,PHYASYMCLK_ROOT_GATE_DISABLE), 1=>(PHYBSYMCLK_CLOCK_CNTL,PHYBSYMCLK_EN,PHYBSYMCLK_SRC_SEL,PHYBSYMCLK_ROOT_GATE_DISABLE), 2=>(PHYCSYMCLK_CLOCK_CNTL,PHYCSYMCLK_EN,PHYCSYMCLK_SRC_SEL,PHYCSYMCLK_ROOT_GATE_DISABLE), 3=>(PHYDSYMCLK_CLOCK_CNTL,PHYDSYMCLK_EN,PHYDSYMCLK_SRC_SEL,PHYDSYMCLK_ROOT_GATE_DISABLE), _=>{ BREAK_TO_DEBUGGER!(); return } };
    REG_UPDATE_2!(d, reg, en, if force {1} else {0}, sel, if force {src} else {0});
    if (*(*dccg).ctx).dc.debug.root_clock_optimization.bits.physymclk { REG_UPDATE!(d, DCCG_GATE_DISABLE_CNTL2, gate, if force {1} else {0}); }
}

pub unsafe fn dccg401_enable_symclk32_le(dccg: *mut dccg, i: i32, src: phyd32clk_clock_source) { let d=to_dcn_dccg(dccg); let (s,e,g,r)=match i {0=>(SYMCLK32_LE0_SRC_SEL,SYMCLK32_LE0_EN,SYMCLK32_LE0_GATE_DISABLE,SYMCLK32_ROOT_LE0_GATE_DISABLE),1=>(SYMCLK32_LE1_SRC_SEL,SYMCLK32_LE1_EN,SYMCLK32_LE1_GATE_DISABLE,SYMCLK32_ROOT_LE1_GATE_DISABLE),2=>(SYMCLK32_LE2_SRC_SEL,SYMCLK32_LE2_EN,SYMCLK32_LE2_GATE_DISABLE,SYMCLK32_ROOT_LE2_GATE_DISABLE),3=>(SYMCLK32_LE3_SRC_SEL,SYMCLK32_LE3_EN,SYMCLK32_LE3_GATE_DISABLE,SYMCLK32_ROOT_LE3_GATE_DISABLE),_=>(return)}; if (*(*dccg).ctx).dc.debug.root_clock_optimization.bits.symclk32_le {REG_UPDATE_2!(d,DCCG_GATE_DISABLE_CNTL3,g,1,r,1)} REG_UPDATE_2!(d,SYMCLK32_LE_CNTL,s,src,e,1); }
pub unsafe fn dccg401_disable_symclk32_le(dccg:*mut dccg,i:i32){let d=to_dcn_dccg(dccg);let(s,e,g,r)=match i{0=>(SYMCLK32_LE0_SRC_SEL,SYMCLK32_LE0_EN,SYMCLK32_LE0_GATE_DISABLE,SYMCLK32_ROOT_LE0_GATE_DISABLE),1=>(SYMCLK32_LE1_SRC_SEL,SYMCLK32_LE1_EN,SYMCLK32_LE1_GATE_DISABLE,SYMCLK32_ROOT_LE1_GATE_DISABLE),2=>(SYMCLK32_LE2_SRC_SEL,SYMCLK32_LE2_EN,SYMCLK32_LE2_GATE_DISABLE,SYMCLK32_ROOT_LE2_GATE_DISABLE),3=>(SYMCLK32_LE3_SRC_SEL,SYMCLK32_LE3_EN,SYMCLK32_LE3_GATE_DISABLE,SYMCLK32_ROOT_LE3_GATE_DISABLE),_=>(return)};REG_UPDATE_2!(d,SYMCLK32_LE_CNTL,s,0,e,0);if(*(*dccg).ctx).dc.debug.root_clock_optimization.bits.symclk32_le{REG_UPDATE_2!(d,DCCG_GATE_DISABLE_CNTL3,g,0,r,0)}}

pub unsafe fn dccg401_disable_dpstreamclk(dccg:*mut dccg,i:i32){let d=to_dcn_dccg(dccg);let e=match i{0=>DPSTREAMCLK0_EN,1=>DPSTREAMCLK1_EN,2=>DPSTREAMCLK2_EN,3=>DPSTREAMCLK3_EN,_=>{BREAK_TO_DEBUGGER!();return}};REG_UPDATE!(d,DPSTREAMCLK_CNTL,e,0);}
pub unsafe fn dccg401_set_dpstreamclk(dccg:*mut dccg,src:streamclk_source,otg:i32,dp:i32){if src==REFCLK{dccg401_disable_dpstreamclk(dccg,dp)}else{let d=to_dcn_dccg(dccg);let(s,e)=match dp{0=>(DPSTREAMCLK0_SRC_SEL,DPSTREAMCLK0_EN),1=>(DPSTREAMCLK1_SRC_SEL,DPSTREAMCLK1_EN),2=>(DPSTREAMCLK2_SRC_SEL,DPSTREAMCLK2_EN),3=>(DPSTREAMCLK3_SRC_SEL,DPSTREAMCLK3_EN),_=>(return)};REG_UPDATE_2!(d,DPSTREAMCLK_CNTL,s,otg,e,1)}}

pub unsafe fn dccg401_init(dccg:*mut dccg){for i in 0..4{dccg31_disable_symclk32_se(dccg,i);dccg401_disable_symclk32_le(dccg,i);dccg401_disable_dpstreamclk(dccg,i);}for i in 0..4{dccg401_set_physymclk(dccg,i,PHYSYMCLK_FORCE_SRC_SYMCLK,false);}dccg401_disable_hdmistreamclk(dccg);dccg401_disable_hdmicharclk(dccg,0);}

// DTO and DSC programming are intentionally expressed through the same register macros.
pub unsafe fn dccg401_set_dto_dscclk(dccg:*mut dccg,inst:u32,_slices:u32){let d=to_dcn_dccg(dccg);let(p,e)=match inst{0=>(DSCCLK0_DTO_PARAM,DSCCLK0_EN),1=>(DSCCLK1_DTO_PARAM,DSCCLK1_EN),2=>(DSCCLK2_DTO_PARAM,DSCCLK2_EN),3=>(DSCCLK3_DTO_PARAM,DSCCLK3_EN),_=>(return)};REG_UPDATE_2!(d,p,DSCCLK0_DTO_PHASE,1,DSCCLK0_DTO_MODULO,1);REG_UPDATE!(d,DSCCLK_DTO_CTRL,e,1);}
pub unsafe fn dccg401_set_ref_dscclk(dccg:*mut dccg,inst:u32){let d=to_dcn_dccg(dccg);let(p,e)=match inst{0=>(DSCCLK0_DTO_PARAM,DSCCLK0_EN),1=>(DSCCLK1_DTO_PARAM,DSCCLK1_EN),2=>(DSCCLK2_DTO_PARAM,DSCCLK2_EN),3=>(DSCCLK3_DTO_PARAM,DSCCLK3_EN),_=>(return)};REG_UPDATE!(d,DSCCLK_DTO_CTRL,e,0);REG_UPDATE_2!(d,p,DSCCLK0_DTO_PHASE,0,DSCCLK0_DTO_MODULO,0);}

pub unsafe fn dccg401_set_dp_dto(dccg:*mut dccg,p:*const dp_dto_params){let d=to_dcn_dccg(dccg);let mut enable=false;if !dc_is_tmds_signal((*p).signal){if (*p).refclk_hz==0{BREAK_TO_DEBUGGER!();return}enable=true;let modulo=(*p).refclk_hz;let integer=div_u64((*p).pixclk_hz,modulo as u32);let phase=(*p).pixclk_hz-integer*modulo;REG_WRITE!(d,DP_DTO_PHASE[(*p).otg_inst],phase as u32);REG_WRITE!(d,DP_DTO_MODULO[(*p).otg_inst],modulo as u32);match (*p).otg_inst{0=>REG_UPDATE!(d,OTG_PIXEL_RATE_DIV,DPDTO0_INT,integer as u32),1=>REG_UPDATE!(d,OTG_PIXEL_RATE_DIV,DPDTO1_INT,integer as u32),2=>REG_UPDATE!(d,OTG_PIXEL_RATE_DIV,DPDTO2_INT,integer as u32),3=>REG_UPDATE!(d,OTG_PIXEL_RATE_DIV,DPDTO3_INT,integer as u32),_=>(BREAK_TO_DEBUGGER!();return)}}REG_UPDATE_2!(d,OTG_PIXEL_RATE_CNTL[(*p).otg_inst],DP_DTO_ENABLE[(*p).otg_inst],enable,PIPE_DTO_SRC_SEL[(*p).otg_inst],enable);}

pub unsafe fn dccg401_enable_symclk_se(dccg:*mut dccg,inst:u32,link:u32){let d=to_dcn_dccg(dccg);let(r,e,s)=match inst{0=>(SYMCLKA_CLOCK_ENABLE,SYMCLKA_FE_EN,SYMCLKA_FE_SRC_SEL),1=>(SYMCLKB_CLOCK_ENABLE,SYMCLKB_FE_EN,SYMCLKB_FE_SRC_SEL),2=>(SYMCLKC_CLOCK_ENABLE,SYMCLKC_FE_EN,SYMCLKC_FE_SRC_SEL),3=>(SYMCLKD_CLOCK_ENABLE,SYMCLKD_FE_EN,SYMCLKD_FE_SRC_SEL),4=>(SYMCLKE_CLOCK_ENABLE,SYMCLKE_FE_EN,SYMCLKE_FE_SRC_SEL),_=>(return)};REG_UPDATE_2!(d,r,e,1,s,link);}
pub unsafe fn dccg401_disable_symclk_se(dccg:*mut dccg,inst:u32,_link:u32){let d=to_dcn_dccg(dccg);let(r,e,s)=match inst{0=>(SYMCLKA_CLOCK_ENABLE,SYMCLKA_FE_EN,SYMCLKA_FE_SRC_SEL),1=>(SYMCLKB_CLOCK_ENABLE,SYMCLKB_FE_EN,SYMCLKB_FE_SRC_SEL),2=>(SYMCLKC_CLOCK_ENABLE,SYMCLKC_FE_EN,SYMCLKC_FE_SRC_SEL),3=>(SYMCLKD_CLOCK_ENABLE,SYMCLKD_FE_EN,SYMCLKD_FE_SRC_SEL),4=>(SYMCLKE_CLOCK_ENABLE,SYMCLKE_FE_EN,SYMCLKE_FE_SRC_SEL),_=>(return)};REG_UPDATE_2!(d,r,e,0,s,0);}

pub unsafe fn dccg401_create(ctx:*mut dc_context,regs:*const dccg_registers,shift:*const dccg_shift,mask:*const dccg_mask)->*mut dccg{let d=kzalloc_obj!(dcn_dccg);if d.is_null(){BREAK_TO_DEBUGGER!();return core::ptr::null_mut()}(*d).base.ctx=ctx;(*d).base.funcs=&dccg401_funcs;(*d).regs=regs;(*d).dccg_shift=shift;(*d).dccg_mask=mask;&mut (*d).base}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
