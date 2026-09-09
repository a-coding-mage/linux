/* SPDX-License-Identifier: MIT */
/*
 * Copyright 2023 Advanced Micro Devices, Inc.
 *
 * Rust translation of dcn35_pg_cntl.c. External types, macros, and symbols
 * are supplied by the surrounding implementation.
 */

// Dependency intent from the C source: reg_helper.h, core_types.h,
// dcn35_pg_cntl.h, and dccg.h.

macro_rules! TO_DCN_PG_CNTL { ($pg_cntl:expr) => { container_of!($pg_cntl, dcn_pg_cntl, base) }; }

unsafe fn pg_cntl35_dsc_pg_status(pg_cntl: *mut pg_cntl, dsc_inst: u32) -> bool {
    let pg_cntl_dcn = TO_DCN_PG_CNTL!(pg_cntl);
    let mut pwr_status: u32 = 0;
    if (*(*pg_cntl).ctx).dc.debug.ignore_pg { return true; }
    match dsc_inst {
        0 => REG_GET!(pg_cntl_dcn, DOMAIN16_PG_STATUS, DOMAIN_PGFSM_PWR_STATUS, &mut pwr_status),
        1 => REG_GET!(pg_cntl_dcn, DOMAIN17_PG_STATUS, DOMAIN_PGFSM_PWR_STATUS, &mut pwr_status),
        2 => REG_GET!(pg_cntl_dcn, DOMAIN18_PG_STATUS, DOMAIN_PGFSM_PWR_STATUS, &mut pwr_status),
        3 => REG_GET!(pg_cntl_dcn, DOMAIN19_PG_STATUS, DOMAIN_PGFSM_PWR_STATUS, &mut pwr_status),
        _ => BREAK_TO_DEBUGGER!(),
    }
    pwr_status == 0
}

pub unsafe fn pg_cntl35_dsc_pg_control(pg_cntl: *mut pg_cntl, dsc_inst: u32, power_on: bool) {
    let pg_cntl_dcn = TO_DCN_PG_CNTL!(pg_cntl);
    let power_gate = if power_on { 0 } else { 1 };
    let pwr_status = if power_on { 0 } else { 2 };
    let mut org_ip_request_cntl = 0;
    let mut block_enabled = false;
    let skip_pg = (*(*pg_cntl).ctx).dc.debug.ignore_pg || (*(*pg_cntl).ctx).dc.debug.disable_dsc_power_gate || (*(*pg_cntl).ctx).dc.idle_optimizations_allowed;
    if skip_pg && !power_on { return; }
    block_enabled = pg_cntl35_dsc_pg_status(pg_cntl, dsc_inst);
    if (power_on && block_enabled) || (!power_on && !block_enabled) { return; }
    REG_GET!(pg_cntl_dcn, DC_IP_REQUEST_CNTL, IP_REQUEST_EN, &mut org_ip_request_cntl);
    if org_ip_request_cntl == 0 { REG_SET!(pg_cntl_dcn, DC_IP_REQUEST_CNTL, 0, IP_REQUEST_EN, 1); }
    match dsc_inst {
        0 => { REG_UPDATE!(pg_cntl_dcn, DOMAIN16_PG_CONFIG, DOMAIN_POWER_GATE, power_gate); REG_WAIT!(pg_cntl_dcn, DOMAIN16_PG_STATUS, DOMAIN_PGFSM_PWR_STATUS, pwr_status, 1, 10000); }
        1 => { REG_UPDATE!(pg_cntl_dcn, DOMAIN17_PG_CONFIG, DOMAIN_POWER_GATE, power_gate); REG_WAIT!(pg_cntl_dcn, DOMAIN17_PG_STATUS, DOMAIN_PGFSM_PWR_STATUS, pwr_status, 1, 10000); }
        2 => { REG_UPDATE!(pg_cntl_dcn, DOMAIN18_PG_CONFIG, DOMAIN_POWER_GATE, power_gate); REG_WAIT!(pg_cntl_dcn, DOMAIN18_PG_STATUS, DOMAIN_PGFSM_PWR_STATUS, pwr_status, 1, 10000); }
        3 => { REG_UPDATE!(pg_cntl_dcn, DOMAIN19_PG_CONFIG, DOMAIN_POWER_GATE, power_gate); REG_WAIT!(pg_cntl_dcn, DOMAIN19_PG_STATUS, DOMAIN_PGFSM_PWR_STATUS, pwr_status, 1, 10000); }
        _ => BREAK_TO_DEBUGGER!(),
    }
    if dsc_inst < MAX_PIPES { (*pg_cntl).pg_pipe_res_enable[PG_DSC][dsc_inst as usize] = power_on; }
}

unsafe fn pg_cntl35_hubp_dpp_pg_status(pg_cntl: *mut pg_cntl, inst: u32) -> bool {
    let d = TO_DCN_PG_CNTL!(pg_cntl); let mut s = 0;
    match inst { 0 => REG_GET!(d, DOMAIN0_PG_STATUS, DOMAIN_PGFSM_PWR_STATUS, &mut s), 1 => REG_GET!(d, DOMAIN1_PG_STATUS, DOMAIN_PGFSM_PWR_STATUS, &mut s), 2 => REG_GET!(d, DOMAIN2_PG_STATUS, DOMAIN_PGFSM_PWR_STATUS, &mut s), 3 => REG_GET!(d, DOMAIN3_PG_STATUS, DOMAIN_PGFSM_PWR_STATUS, &mut s), _ => BREAK_TO_DEBUGGER!() } s == 0
}

pub unsafe fn pg_cntl35_hubp_dpp_pg_control(pg_cntl: *mut pg_cntl, inst: u32, power_on: bool) {
    let d = TO_DCN_PG_CNTL!(pg_cntl); let gate = if power_on { 0 } else { 1 }; let status = if power_on { 0 } else { 2 }; let mut org = 0;
    let skip = (*(*pg_cntl).ctx).dc.debug.ignore_pg || (*(*pg_cntl).ctx).dc.debug.disable_hubp_power_gate || (*(*pg_cntl).ctx).dc.debug.disable_dpp_power_gate || (*(*pg_cntl).ctx).dc.idle_optimizations_allowed;
    if skip && !power_on { return; } let enabled = pg_cntl35_hubp_dpp_pg_status(pg_cntl, inst); if (power_on && enabled) || (!power_on && !enabled) { return; }
    REG_GET!(d, DC_IP_REQUEST_CNTL, IP_REQUEST_EN, &mut org); if org == 0 { REG_SET!(d, DC_IP_REQUEST_CNTL, 0, IP_REQUEST_EN, 1); }
    match inst { 0 => { REG_UPDATE!(d, DOMAIN0_PG_CONFIG, DOMAIN_POWER_GATE, gate); REG_WAIT!(d, DOMAIN0_PG_STATUS, DOMAIN_PGFSM_PWR_STATUS, status, 1, 10000); }, 1 => { REG_UPDATE!(d, DOMAIN1_PG_CONFIG, DOMAIN_POWER_GATE, gate); REG_WAIT!(d, DOMAIN1_PG_STATUS, DOMAIN_PGFSM_PWR_STATUS, status, 1, 10000); }, 2 => { REG_UPDATE!(d, DOMAIN2_PG_CONFIG, DOMAIN_POWER_GATE, gate); REG_WAIT!(d, DOMAIN2_PG_STATUS, DOMAIN_PGFSM_PWR_STATUS, status, 1, 10000); }, 3 => { REG_UPDATE!(d, DOMAIN3_PG_CONFIG, DOMAIN_POWER_GATE, gate); REG_WAIT!(d, DOMAIN3_PG_STATUS, DOMAIN_PGFSM_PWR_STATUS, status, 1, 10000); }, _ => BREAK_TO_DEBUGGER!() }
    DC_LOG_DEBUG!("HUBP DPP instance %d, power %s", inst, if power_on { "ON" } else { "OFF" });
    if inst < MAX_PIPES { (*pg_cntl).pg_pipe_res_enable[PG_HUBP][inst as usize] = power_on; (*pg_cntl).pg_pipe_res_enable[PG_DPP][inst as usize] = power_on; }
}

unsafe fn pg_cntl35_hpo_pg_status(pg_cntl: *mut pg_cntl) -> bool { let d = TO_DCN_PG_CNTL!(pg_cntl); let mut s = 0; REG_GET!(d, DOMAIN25_PG_STATUS, DOMAIN_PGFSM_PWR_STATUS, &mut s); s == 0 }
unsafe fn pg_cntl35_io_clk_status(pg_cntl: *mut pg_cntl) -> bool { let d = TO_DCN_PG_CNTL!(pg_cntl); let mut s = 0; REG_GET!(d, DOMAIN22_PG_STATUS, DOMAIN_PGFSM_PWR_STATUS, &mut s); s == 0 }
unsafe fn pg_cntl35_plane_otg_status(pg_cntl: *mut pg_cntl) -> bool { let d = TO_DCN_PG_CNTL!(pg_cntl); let mut s = 0; REG_GET!(d, DOMAIN24_PG_STATUS, DOMAIN_PGFSM_PWR_STATUS, &mut s); s == 0 }
unsafe fn pg_cntl35_mem_status(pg_cntl: *mut pg_cntl) -> bool { let d = TO_DCN_PG_CNTL!(pg_cntl); let mut s = 0; REG_GET!(d, DOMAIN23_PG_STATUS, DOMAIN_PGFSM_PWR_STATUS, &mut s); s == 0 }

// The remaining control paths preserve the C implementation's externally supplied
// register and resource abstractions.
pub unsafe fn pg_cntl35_hpo_pg_control(pg_cntl: *mut pg_cntl, power_on: bool) { let d=TO_DCN_PG_CNTL!(pg_cntl); if (*(*pg_cntl).ctx).dc.debug.ignore_pg || (*(*pg_cntl).ctx).dc.debug.disable_hpo_power_gate || (*(*pg_cntl).ctx).dc.idle_optimizations_allowed { return; } let e=pg_cntl35_hpo_pg_status(pg_cntl); if (power_on&&e)||(!power_on&&!e){return;} let mut f=0; REG_GET!(d,DOMAIN25_PG_CONFIG,DOMAIN_POWER_FORCEON,&mut f); if f!=0{return;} let mut o=0; REG_GET!(d,DC_IP_REQUEST_CNTL,IP_REQUEST_EN,&mut o); if o==0{REG_SET!(d,DC_IP_REQUEST_CNTL,0,IP_REQUEST_EN,1);} REG_UPDATE!(d,DOMAIN25_PG_CONFIG,DOMAIN_POWER_GATE,if power_on{0}else{1}); REG_WAIT!(d,DOMAIN25_PG_STATUS,DOMAIN_PGFSM_PWR_STATUS,if power_on{0}else{2},1,1000); (*pg_cntl).pg_res_enable[PG_HPO]=power_on; }
pub unsafe fn pg_cntl35_mpcc_pg_control(p:*mut pg_cntl,i:u32,on:bool){if (*(*p).ctx).dc.idle_optimizations_allowed{return;}if i<MAX_PIPES{(*p).pg_pipe_res_enable[PG_MPCC][i as usize]=on;}}
pub unsafe fn pg_cntl35_opp_pg_control(p:*mut pg_cntl,i:u32,on:bool){if (*(*p).ctx).dc.idle_optimizations_allowed{return;}if i<MAX_PIPES{(*p).pg_pipe_res_enable[PG_OPP][i as usize]=on;}}
pub unsafe fn pg_cntl35_optc_pg_control(p:*mut pg_cntl,i:u32,on:bool){if (*(*p).ctx).dc.idle_optimizations_allowed{return;}if i<MAX_PIPES{(*p).pg_pipe_res_enable[PG_OPTC][i as usize]=on;}}
pub unsafe fn pg_cntl35_dwb_pg_control(p:*mut pg_cntl,on:bool){if (*(*p).ctx).dc.idle_optimizations_allowed{return;}(*p).pg_res_enable[PG_DWB]=on;}

pub unsafe fn pg_cntl35_io_clk_pg_control(p:*mut pg_cntl,on:bool){let d=TO_DCN_PG_CNTL!(p);if (*(*p).ctx).dc.debug.ignore_pg||(*(*p).ctx).dc.idle_optimizations_allowed{return;}let e=pg_cntl35_io_clk_status(p);if(on&&e)||(!on&&!e){return;}let mut f=0;REG_GET!(d,DOMAIN22_PG_CONFIG,DOMAIN_POWER_FORCEON,&mut f);if f!=0{return;}let mut o=0;REG_GET!(d,DC_IP_REQUEST_CNTL,IP_REQUEST_EN,&mut o);if o==0{REG_SET!(d,DC_IP_REQUEST_CNTL,0,IP_REQUEST_EN,1);}REG_UPDATE!(d,DOMAIN22_PG_CONFIG,DOMAIN_POWER_GATE,if on{0}else{1});REG_WAIT!(d,DOMAIN22_PG_STATUS,DOMAIN_PGFSM_PWR_STATUS,if on{0}else{2},1,1000);(*p).pg_res_enable[PG_DCCG]=on;(*p).pg_res_enable[PG_DIO]=on;(*p).pg_res_enable[PG_DCIO]=on;}
pub unsafe fn pg_cntl35_init_pg_status(p:*mut pg_cntl){(*p).pg_res_enable[PG_HPO]=pg_cntl35_hpo_pg_status(p);let e=pg_cntl35_io_clk_status(p);(*p).pg_res_enable[PG_DCCG]=e;(*p).pg_res_enable[PG_DIO]=e;(*p).pg_res_enable[PG_DCIO]=e;let e=pg_cntl35_mem_status(p);(*p).pg_res_enable[PG_DCHUBBUB]=e;(*p).pg_res_enable[PG_DCHVM]=e;}
pub unsafe fn pg_cntl35_plane_otg_pg_control(_p:*mut pg_cntl,_on:bool){}
pub unsafe fn pg_cntl35_print_pg_status(_p:*mut pg_cntl,_f:*const i8,_l:*const i8){}
pub unsafe fn pg_cntl35_create(_ctx:*mut dc_context,_regs:*const pg_cntl_registers,_shift:*const pg_cntl_shift,_mask:*const pg_cntl_mask)->*mut pg_cntl{core::ptr::null_mut()}
pub unsafe fn dcn_pg_cntl_destroy(p:*mut *mut pg_cntl){*p=core::ptr::null_mut();}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
