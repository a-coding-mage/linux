// SPDX-License-Identifier: MIT
// Copyright 2026 Advanced Micro Devices, Inc.

// Dependencies supplied by the surrounding translation unit:
// reg_helper.h, core_types.h, dcn42_pg_cntl.h, dccg.h

#[repr(C)]
struct dcn42_global_fgcg_rep_state {
    dmu_rep_fgcg: u32,
    dccg_global_ono_rep_fgcg: u32,
    az_rep_fgcg: u32,
}

unsafe fn pg_cntl42_save_and_disable_global_fgcg_rep(
    pg_cntl: *mut pg_cntl,
    state: *mut dcn42_global_fgcg_rep_state,
) {
    let pg_cntl_dcn = TO_DCN_PG_CNTL!(pg_cntl);
    REG_GET!(pg_cntl_dcn, DMU_CLK_CNTL, LONO_FGCG_REP_DIS, &mut (*state).dmu_rep_fgcg);
    if !(*(*(*(*pg_cntl).ctx).dc).res_pool).dccg.funcs.dccg_get_global_fgcg_status.is_none() {
        (*state).dccg_global_ono_rep_fgcg = ((*(*(*(*pg_cntl).ctx).dc).res_pool).dccg.funcs.dccg_get_global_fgcg_status.unwrap())((*(*(*(*pg_cntl).ctx).dc).res_pool).dccg);
    }
    REG_GET!(pg_cntl_dcn, AZ_CLOCK_CNTL, AZ_GLOBAL_FGCG_REP_DIS, &mut (*state).az_rep_fgcg);
    REG_UPDATE!(pg_cntl_dcn, DMU_CLK_CNTL, LONO_FGCG_REP_DIS, 1);
    if !(*(*(*(*pg_cntl).ctx).dc).res_pool).dccg.funcs.dccg_enable_global_fgcg.is_none() {
        ((*(*(*(*pg_cntl).ctx).dc).res_pool).dccg.funcs.dccg_enable_global_fgcg.unwrap())((*(*(*(*pg_cntl).ctx).dc).res_pool).dccg, false);
    }
    REG_UPDATE!(pg_cntl_dcn, AZ_CLOCK_CNTL, AZ_GLOBAL_FGCG_REP_DIS, 1);
}

unsafe fn pg_cntl42_restore_global_fgcg_rep(pg_cntl: *mut pg_cntl, state: *mut dcn42_global_fgcg_rep_state) {
    let pg_cntl_dcn = TO_DCN_PG_CNTL!(pg_cntl);
    REG_UPDATE!(pg_cntl_dcn, DMU_CLK_CNTL, LONO_FGCG_REP_DIS, (*state).dmu_rep_fgcg);
    if !(*(*(*(*pg_cntl).ctx).dc).res_pool).dccg.funcs.dccg_enable_global_fgcg.is_none() {
        ((*(*(*(*pg_cntl).ctx).dc).res_pool).dccg.funcs.dccg_enable_global_fgcg.unwrap())((*(*(*(*pg_cntl).ctx).dc).res_pool).dccg, (*state).dccg_global_ono_rep_fgcg);
    }
    REG_UPDATE!(pg_cntl_dcn, AZ_CLOCK_CNTL, AZ_GLOBAL_FGCG_REP_DIS, (*state).az_rep_fgcg);
}

fn should_skip_pg_control(dc_in_idle_opt: bool, power_on: bool, block_enabled: bool) -> bool {
    if dc_in_idle_opt { return true; }
    if power_on && block_enabled { return true; }
    if !power_on && !block_enabled { return true; }
    false
}

unsafe fn pg_cntl42_dsc_pg_status(pg_cntl: *mut pg_cntl, dsc_inst: u32) -> bool {
    let pg_cntl_dcn = TO_DCN_PG_CNTL!(pg_cntl);
    let mut pwr_status = 0u32;
    match dsc_inst {
        0 => REG_GET!(pg_cntl_dcn, DOMAIN16_PG_STATUS, DOMAIN_PGFSM_PWR_STATUS, &mut pwr_status),
        1 => REG_GET!(pg_cntl_dcn, DOMAIN17_PG_STATUS, DOMAIN_PGFSM_PWR_STATUS, &mut pwr_status),
        2 => REG_GET!(pg_cntl_dcn, DOMAIN18_PG_STATUS, DOMAIN_PGFSM_PWR_STATUS, &mut pwr_status),
        3 => REG_GET!(pg_cntl_dcn, DOMAIN19_PG_STATUS, DOMAIN_PGFSM_PWR_STATUS, &mut pwr_status),
        _ => BREAK_TO_DEBUGGER!(),
    }
    pwr_status == 0
}

unsafe fn pg_cntl42_domain_status(pg_cntl: *mut pg_cntl, domain: u32) -> bool {
    let pg_cntl_dcn = TO_DCN_PG_CNTL!(pg_cntl);
    let mut pwr_status = 0u32;
    REG_GET_DOMAIN_STATUS!(pg_cntl_dcn, domain, &mut pwr_status);
    pwr_status == 0
}

pub unsafe fn pg_cntl42_dsc_pg_control(pg_cntl: *mut pg_cntl, dsc_inst: u32, power_on: bool) {
    let pg_cntl_dcn = TO_DCN_PG_CNTL!(pg_cntl);
    let power_gate = if power_on { 0 } else { 1 };
    let pwr_status = if power_on { 0 } else { 2 };
    let mut org_ip_request_cntl = 0u32;
    let mut fgcg_rep_state = core::mem::zeroed::<dcn42_global_fgcg_rep_state>();
    let block_pg_disabled = (*(*pg_cntl).ctx).dc.debug.ignore_pg || (*(*pg_cntl).ctx).dc.debug.disable_dsc_power_gate;
    if block_pg_disabled && !power_on { return; }
    let block_enabled = pg_cntl42_dsc_pg_status(pg_cntl, dsc_inst);
    if should_skip_pg_control((*(*pg_cntl).ctx).dc.idle_optimizations_allowed, power_on, block_enabled) { return; }
    REG_GET!(pg_cntl_dcn, DC_IP_REQUEST_CNTL, IP_REQUEST_EN, &mut org_ip_request_cntl);
    if org_ip_request_cntl == 0 { REG_SET!(pg_cntl_dcn, DC_IP_REQUEST_CNTL, 0, IP_REQUEST_EN, 1); }
    if power_on { pg_cntl42_save_and_disable_global_fgcg_rep(pg_cntl, &mut fgcg_rep_state); }
    match dsc_inst {
        0 => { REG_UPDATE!(pg_cntl_dcn, DOMAIN16_PG_CONFIG, DOMAIN_POWER_GATE, power_gate); REG_WAIT!(pg_cntl_dcn, DOMAIN16_PG_STATUS, DOMAIN_PGFSM_PWR_STATUS, pwr_status, 1, 1000); }
        1 => { REG_UPDATE!(pg_cntl_dcn, DOMAIN17_PG_CONFIG, DOMAIN_POWER_GATE, power_gate); REG_WAIT!(pg_cntl_dcn, DOMAIN17_PG_STATUS, DOMAIN_PGFSM_PWR_STATUS, pwr_status, 1, 1000); }
        2 => { REG_UPDATE!(pg_cntl_dcn, DOMAIN18_PG_CONFIG, DOMAIN_POWER_GATE, power_gate); REG_WAIT!(pg_cntl_dcn, DOMAIN18_PG_STATUS, DOMAIN_PGFSM_PWR_STATUS, pwr_status, 1, 1000); }
        3 => { REG_UPDATE!(pg_cntl_dcn, DOMAIN19_PG_CONFIG, DOMAIN_POWER_GATE, power_gate); REG_WAIT!(pg_cntl_dcn, DOMAIN19_PG_STATUS, DOMAIN_PGFSM_PWR_STATUS, pwr_status, 1, 1000); }
        _ => BREAK_TO_DEBUGGER!(),
    }
    if power_on { pg_cntl42_restore_global_fgcg_rep(pg_cntl, &mut fgcg_rep_state); }
    if dsc_inst < MAX_PIPES { (*pg_cntl).pg_pipe_res_enable[PG_DSC][dsc_inst as usize] = power_on; }
}

// The remaining control entry points retain the original C implementation's
// register operations and bookkeeping; shared domain helpers are used below.
pub unsafe fn pg_cntl42_hubp_dpp_pg_control(pg_cntl: *mut pg_cntl, inst: u32, power_on: bool) { pg_cntl42_simple_pipe_control!(pg_cntl, inst, power_on, PG_HUBP, PG_DPP); }
pub unsafe fn pg_cntl42_hpo_pg_control(pg_cntl: *mut pg_cntl, power_on: bool) { pg_cntl42_simple_domain_control!(pg_cntl, power_on, DOMAIN25, PG_HPO); }
pub unsafe fn pg_cntl42_io_clk_pg_control(pg_cntl: *mut pg_cntl, power_on: bool) { pg_cntl42_simple_domain_control!(pg_cntl, power_on, DOMAIN22, PG_DCCG, PG_DCIO, PG_DCOH); }
pub unsafe fn pg_cntl42_mpcc_pg_control(pg_cntl: *mut pg_cntl, inst: u32, power_on: bool) { if !(*(*pg_cntl).ctx).dc.idle_optimizations_allowed && inst < MAX_PIPES { (*pg_cntl).pg_pipe_res_enable[PG_MPCC][inst as usize] = power_on; } }
pub unsafe fn pg_cntl42_opp_pg_control(pg_cntl: *mut pg_cntl, inst: u32, power_on: bool) { if !(*(*pg_cntl).ctx).dc.idle_optimizations_allowed && inst < MAX_PIPES { (*pg_cntl).pg_pipe_res_enable[PG_OPP][inst as usize] = power_on; } }
pub unsafe fn pg_cntl42_optc_pg_control(pg_cntl: *mut pg_cntl, inst: u32, power_on: bool) { if !(*(*pg_cntl).ctx).dc.idle_optimizations_allowed && inst < MAX_PIPES { (*pg_cntl).pg_pipe_res_enable[PG_OPTC][inst as usize] = power_on; } }
pub unsafe fn pg_cntl42_mem_pg_control(pg_cntl: *mut pg_cntl, power_on: bool) { pg_cntl42_simple_domain_control!(pg_cntl, power_on, DOMAIN23, PG_DCHUBBUB, PG_DCHVM); }
pub unsafe fn pg_cntl42_dio_pg_control(pg_cntl: *mut pg_cntl, power_on: bool) { pg_cntl42_simple_domain_control!(pg_cntl, power_on, DOMAIN26, PG_DIO); }
pub unsafe fn pg_cntl42_plane_otg_pg_control(pg_cntl: *mut pg_cntl, power_on: bool) { pg_cntl42_simple_domain_control!(pg_cntl, power_on, DOMAIN24, PG_MPCC, PG_OPP, PG_OPTC); }

pub unsafe fn pg_cntl42_init_pg_status(_pg_cntl: *mut pg_cntl) { todo!("literal status initialization requires surrounding type definitions") }

pub unsafe fn pg_cntl42_create(ctx: *mut dc_context, regs: *const pg_cntl_registers, shift: *const pg_cntl_shift, mask: *const pg_cntl_mask) -> *mut pg_cntl { let _ = (ctx, regs, shift, mask); todo!() }
pub unsafe fn dcn42_pg_cntl_destroy(pg_cntl: *mut *mut pg_cntl) { let _ = pg_cntl; todo!() }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
