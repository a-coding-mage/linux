// SPDX-License-Identifier: MIT
//
// Copyright 2024 Advanced Micro Devices, Inc.

// Translated from dml2_dpmm_dcn5.c. Types and helper symbols are supplied by
// the surrounding DML2 translation unit.

extern "C" {
    fn math_max2(a: f64, b: f64) -> f64;
    fn math_max3(a: f64, b: f64, c: f64) -> f64;
    fn math_min2(a: f64, b: f64) -> f64;
    fn math_ceil2(a: f64, b: f64) -> f64;
    fn memcmp(a: *const core::ffi::c_void, b: *const core::ffi::c_void, n: usize) -> i32;
    fn memcpy(dst: *mut core::ffi::c_void, src: *const core::ffi::c_void, n: usize) -> *mut core::ffi::c_void;
}

unsafe fn add_margin_and_round_to_dfs_grainularity(clock_khz: f64, margin: f64, vco_freq_khz: u64, rounded_khz: *mut u64, divider_id: *mut u32) -> bool {
    const R1_START: u32 = 8; const R1_STEP: u32 = 1;
    const R2_START: u32 = 64; const R2_STEP: u32 = 2;
    const R3_START: u32 = 128; const R3_STEP: u32 = 4;
    const R4_START: u32 = 248; const R4_STEP: u32 = 264;
    const SCALE: u32 = 4;
    const DID1: u32 = 0x08; const DID2: u32 = 0x40; const DID3: u32 = 0x60; const DID4: u32 = 0x7e; const MAX_DID: u32 = 0x7f;
    if clock_khz < 1.0 || vco_freq_khz < 1 || clock_khz > vco_freq_khz as f64 { return false; }
    let clock_khz = clock_khz * (1.0 + margin);
    let divider = (SCALE as f64 * (vco_freq_khz as f64 / clock_khz)) as i32 as u32;
    *divider_id = if divider < R2_START {
        if divider < R1_START { DID1 } else { DID1 + (divider - R1_START) / R1_STEP }
    } else if divider < R3_START { DID2 + (divider - R2_START) / R2_STEP
    } else if divider < R4_START { DID3 + (divider - R3_START) / R3_STEP
    } else { let mut d = DID4 + (divider - R4_START) / R4_STEP; if d > MAX_DID { d = MAX_DID; } d };
    *rounded_khz = vco_freq_khz * SCALE as u64 / divider as u64;
    true
}

unsafe fn round_to_non_dfs_granularity(dispclk_khz: u64, dpprefclk_khz: u64, dtbrefclk_khz: u64, rounded_dispclk_khz: *mut u64, rounded_dpprefclk_khz: *mut u64, rounded_dtbrefclk_khz: *mut u64) -> bool {
    let pll = math_max2(600000.0, math_ceil2(math_max3(dispclk_khz as f64, dpprefclk_khz as f64, dtbrefclk_khz as f64), 1000.0)) as u64;
    *rounded_dispclk_khz = pll / math_min2((pll / dispclk_khz) as f64, 32.0) as u64;
    *rounded_dpprefclk_khz = pll / math_min2((pll / dpprefclk_khz) as f64, 32.0) as u64;
    *rounded_dtbrefclk_khz = if dtbrefclk_khz > 0 { pll / math_min2((pll / dtbrefclk_khz) as f64, 32.0) as u64 } else { 0 };
    true
}

unsafe fn validate_min_clocks(solution: *const dml2_display_solution, programming: *mut dml2_display_cfg_programming, utm_soc_bb: *const dml2_utm_soc_bb) -> bool {
    if utm_soc_bb.is_null() || programming.is_null() { return false; }
    let s = &*solution; let p = &*programming; let u = &*utm_soc_bb;
    if p.min_clocks.dcn4x.dispclk_khz > u.max_dispclk_khz || p.min_clocks.dcn4x.dpprefclk_khz > u.max_dppclk_khz || p.min_clocks.dcn4x.dtbrefclk_khz > u.max_dtbclk_khz { return false; }
    for i in 0..s.dispcfg.num_planes { if p.plane_programming[i].min_clocks.dcn4x.dppclk_khz > u.max_dppclk_khz { return false; } }
    for i in 0..s.dispcfg.num_streams { if p.stream_programming[i].min_clocks.dcn4x.dscclk_khz > u.max_dscclk_khz || p.stream_programming[i].min_clocks.dcn4x.dtbclk_khz > u.max_dtbclk_khz || p.stream_programming[i].min_clocks.dcn4x.phyclk_khz > u.max_phyclk_khz { return false; } }
    true
}

unsafe fn are_timings_trivially_synchronizable(solution: *const dml2_display_solution, mask: i32) -> bool {
    let s = &*solution; let mut a = [0u32; DML2_MAX_PLANES]; let mut n = 0usize;
    for i in 0..s.dispcfg.num_streams { if mask & (1 << i) != 0 { a[n] = i as u32; n += 1; } }
    if n <= 1 { return true; }
    let mut identical = true;
    for i in 1..n { if memcmp((&s.dispcfg.stream_descriptors[a[i-1] as usize].timing) as *const _ as *const _, (&s.dispcfg.stream_descriptors[a[i] as usize].timing) as *const _ as *const _, core::mem::size_of::<dml2_timing_cfg>()) != 0 { identical = false; break; } }
    let mut drr = false; for i in 0..n { if s.dispcfg.stream_descriptors[a[i] as usize].timing.drr_config.enabled { drr = true; break; } }
    !drr && identical
}

unsafe fn find_smallest_idle_time_in_vblank_us(solution: *const dml2_display_solution, mask: i32) -> i32 {
    let s = &*solution; let r = &s.validation_result.mode_support; let mut a = [0u32; DML2_MAX_PLANES]; let mut n=0usize;
    for i in 0..s.dispcfg.num_streams { if mask & (1 << i) != 0 { a[n]=i as u32; n+=1; } }
    if n == 0 { return 0; } let mut min = r.cfg_support_info.stream_support_info[a[0] as usize].vblank_reserved_time_us;
    for i in 1..n { let x=r.cfg_support_info.stream_support_info[a[i] as usize].vblank_reserved_time_us; if min>x {min=x;} } min
}

unsafe fn get_displays_without_vactive_margin_mask(solution: *const dml2_display_solution, utm_soc_bb: *const dml2_utm_soc_bb) -> i32 {
    let s=&*solution; let u=&*utm_soc_bb; let r=&s.validation_result.mode_support; let mut m=0;
    for i in 0..s.dispcfg.num_planes { if r.cfg_support_info.plane_support_info[i].active_latency_hiding_us < u.power_management_parameters.fclk_change_blackout_us as i32 { m |= 1 << i; } } m
}

unsafe fn calculate_dispclk_khz(solution:*const dml2_display_solution, u:*const dml2_utm_soc_bb, ip:*const dml2_core_ip_params)->u64 { let s=&*solution; let u=&*u; let ip=&*ip; let base=s.validation_result.mode_support.global.dispclk_khz as f64; let mut x=base*(1.0+u.dcn_downspread_percent/100.0)*(1.0+ip.dispclk_ramp_margin_percent/100.0); x=math_min2(x,u.max_dispclk_khz as f64); x=math_max2(x,base*(1.0+u.dcn_downspread_percent/100.0)); x as u64 }

unsafe fn calculate_dpprefclk_khz(s:*const dml2_display_solution,u:*const dml2_utm_soc_bb)->u64 { let s=&*s; let u=&*u; let mut x=0u64; for i in 0..s.dispcfg.num_planes { if x<s.validation_result.mode_support.per_plane[i].dppclk_khz{x=s.validation_result.mode_support.per_plane[i].dppclk_khz;} } (x as f64*(1.0+u.dcn_downspread_percent/100.0)) as u64 }
unsafe fn calculate_dtbrefclk_khz(s:*const dml2_display_solution,u:*const dml2_utm_soc_bb)->u64 { let s=&*s; let u=&*u; let mut x=0u64; for i in 0..s.dispcfg.num_streams { if x<s.validation_result.mode_support.per_stream[i].dtbclk_khz{x=s.validation_result.mode_support.per_stream[i].dtbclk_khz;} } (x as f64*(1.0+u.dcn_downspread_percent/100.0)) as u64 }
unsafe fn calculate_dppclk_khz_plane_index(i:usize,s:*const dml2_display_solution,u:*const dml2_utm_soc_bb,d:u64)->u64 { let s=&*s; let u=&*u; (d as f64/255.0*math_ceil2(s.validation_result.mode_support.per_plane[i].dppclk_khz as f64*(1.0+u.dcn_downspread_percent/100.0)*255.0/d as f64,1.0)) as u64 }

unsafe fn round_min_clocks_to_granularity(p:*mut dml2_display_cfg_programming,u:*const dml2_utm_soc_bb) { let p=&mut *p; let u=&*u; if u.no_dfs { round_to_non_dfs_granularity(p.min_clocks.dcn4x.dispclk_khz,p.min_clocks.dcn4x.dpprefclk_khz,p.min_clocks.dcn4x.dtbrefclk_khz,&mut p.min_clocks.dcn4x.dispclk_khz,&mut p.min_clocks.dcn4x.dpprefclk_khz,&mut p.min_clocks.dcn4x.dtbrefclk_khz); } else { let v=(u.dispclk_dppclk_vco_speed_mhz*1000.0) as u64; add_margin_and_round_to_dfs_grainularity(p.min_clocks.dcn4x.dispclk_khz as f64,0.0,v,&mut p.min_clocks.dcn4x.dispclk_khz,&mut p.min_clocks.dcn4x.divider_ids.dispclk_did); add_margin_and_round_to_dfs_grainularity(p.min_clocks.dcn4x.dpprefclk_khz as f64,0.0,v,&mut p.min_clocks.dcn4x.dpprefclk_khz,&mut p.min_clocks.dcn4x.divider_ids.dpprefclk_did); add_margin_and_round_to_dfs_grainularity(p.min_clocks.dcn4x.dtbrefclk_khz as f64,0.0,v,&mut p.min_clocks.dcn4x.dtbrefclk_khz,&mut p.min_clocks.dcn4x.divider_ids.dtbrefclk_did); } }

unsafe fn dcn5_populate_min_clocks_in_programming(p:*mut dml2_display_cfg_programming,u:*const dml2_utm_soc_bb,ip:*const dml2_core_ip_params,s:*const dml2_display_solution)->bool { let p=&mut *p; let s=&*s; p.min_clocks.dcn4x.dispclk_khz=calculate_dispclk_khz(s,u,ip); p.min_clocks.dcn4x.dpprefclk_khz=calculate_dpprefclk_khz(s,u); p.min_clocks.dcn4x.dtbrefclk_khz=calculate_dtbrefclk_khz(s,u); p.min_clocks.dcn4x.deepsleep_dcfclk_khz=math_min2(s.validation_result.mode_support.global.dcfclk_deepsleep_khz as f64,s.sop_constraint.dcn5.clocks.dcfclk_khz as f64) as u64; p.min_clocks.dcn4x.socclk_khz=s.sop_constraint.dcn5.clocks.socclk_khz; p.min_clocks.dcn4x.active.dcfclk_khz=s.sop_constraint.dcn5.clocks.dcfclk_khz; p.min_clocks.dcn4x.active.fclk_khz=s.sop_constraint.dcn5.clocks.fclk_khz; p.min_clocks.dcn4x.active.uclk_khz=s.sop_constraint.dcn5.clocks.uclk_khz; round_min_clocks_to_granularity(p,u); for i in 0..s.dispcfg.num_planes { p.plane_programming[i].min_clocks.dcn4x.dppclk_khz=calculate_dppclk_khz_plane_index(i,s,u,p.min_clocks.dcn4x.dpprefclk_khz); } for i in 0..s.dispcfg.num_streams { p.stream_programming[i].min_clocks.dcn4x.dscclk_khz=s.validation_result.mode_support.per_stream[i].dscclk_khz; p.stream_programming[i].min_clocks.dcn4x.dtbclk_khz=s.validation_result.mode_support.per_stream[i].dtbclk_khz; p.stream_programming[i].min_clocks.dcn4x.phyclk_khz=s.validation_result.mode_support.per_stream[i].phyclk_khz; } validate_min_clocks(s,p,u) }

pub unsafe fn dcn5_populate_pstate_support_in_programming(p:*mut dml2_display_cfg_programming,u:*const dml2_utm_soc_bb,s:*const dml2_display_solution) { let p=&mut *p; let s=&*s; let u=&*u; let mut supported=true; for i in 0..s.dispcfg.num_planes { if s.uclk_pstate_params.pstate_switch_modes[i]==dml2_pstate_method_na {supported=false;break;} } p.uclk_pstate_supported=supported; p.fclk_pstate_supported=false; let m=get_displays_without_vactive_margin_mask(s,u); if m==0 {p.fclk_pstate_supported=true;} else if are_timings_trivially_synchronizable(s,m) && find_smallest_idle_time_in_vblank_us(s,m)>=u.power_management_parameters.fclk_change_blackout_us {p.fclk_pstate_supported=true;} }
pub unsafe fn dcn5_populate_stutter_support_in_programming(p:*mut dml2_display_cfg_programming,u:*const dml2_utm_soc_bb,s:*const dml2_display_solution) { let p=&mut *p; let u=&*u; let idle=find_smallest_idle_time_in_vblank_us(s,0xff); p.stutter.supported_in_blank=u.power_management_parameters.stutter_enter_plus_exit_latency_us>0 && idle>=u.power_management_parameters.stutter_enter_plus_exit_latency_us; p.z8_stutter.meets_eco=u.power_management_parameters.z8_min_idle_time>0 && p.informative.power_management.z8.stutter_period>=u.power_management_parameters.z8_min_idle_time; p.z8_stutter.supported_in_blank=u.power_management_parameters.z8_stutter_exit_latency_us>0 && idle>=u.power_management_parameters.z8_stutter_exit_latency_us; }
unsafe fn dcn5_populate_qos_bound_in_programming(p:*mut dml2_display_cfg_programming,s:*const dml2_display_solution) { (*p).qos_bound.latency_ub=(*s).sop_constraint.dcn5.latency; (*p).qos_bound.bandwidth_lb.dcn5.urgent_bandwidth_kbps=(*s).sop_constraint.dcn5.min_available_urgent_bandwidth_KBps; }
pub unsafe fn dpmm_dcn5_map_mode_to_soc_dpm(x:*mut dml2_dpmm_map_mode_to_soc_dpm_params_in_out)->bool { dcn5_populate_qos_bound_in_programming((*x).programming,(*x).solution); dcn5_populate_pstate_support_in_programming((*x).programming,(*x).utm_soc_bb,(*x).solution); dcn5_populate_stutter_support_in_programming((*x).programming,(*x).utm_soc_bb,(*x).solution); dcn5_populate_min_clocks_in_programming((*x).programming,(*x).utm_soc_bb,(*x).ip,(*x).solution) }

pub unsafe fn dpmm_dcn5_map_watermarks(x:*mut dml2_dpmm_map_watermarks_params_in_out)->bool {
    let cfg=&(*x).solution.as_ref().unwrap().dispcfg; let lib=&(*x).core.as_ref().unwrap().clean_me_up.mode_lib; let regs=&mut (*x).programming.as_mut().unwrap().global_regs; let mcif=&mut (*x).programming.as_mut().unwrap().mcif_global_regs;
    let refclk=if cfg.overrides.hw.dlg_ref_clk_mhz>0 {cfg.overrides.hw.dlg_ref_clk_mhz as f64} else {(*x).core.as_ref().unwrap().utm_soc_bb.dchub_refclk_mhz};
    for set in [DML2_DCHUB_WATERMARK_SET_A,DML2_DCHUB_WATERMARK_SET_B] { let w=&mut regs.wm_regs[set]; w.fclk_pstate=(lib.mp.Watermark.FCLKChangeWatermark*refclk) as u32; w.sr_enter=(lib.mp.Watermark.StutterEnterPlusExitWatermark*refclk) as u32; w.sr_exit=(lib.mp.Watermark.StutterExitWatermark*refclk) as u32; w.sr_enter_z8=(lib.mp.Watermark.Z8StutterEnterPlusExitWatermark*refclk) as u32; w.sr_exit_z8=(lib.mp.Watermark.Z8StutterExitWatermark*refclk) as u32; w.temp_read_or_ppt=(lib.mp.Watermark.temp_read_or_ppt_watermark_us*refclk) as u32; w.uclk_pstate=(lib.mp.Watermark.DRAMClockChangeWatermark*refclk) as u32; w.urgent=(lib.mp.Watermark.UrgentWatermark*refclk) as u32; w.usr=(lib.mp.Watermark.USRRetrainingWatermark*refclk) as u32; w.refcyc_per_trip_to_mem=w.urgent; w.refcyc_per_meta_trip_to_mem=w.urgent; w.frac_urg_bw_flip=(lib.mp.FractionOfUrgentBandwidthImmediateFlip*1000.0) as u32; w.frac_urg_bw_nom=(lib.mp.FractionOfUrgentBandwidth*1000.0) as u32; w.frac_urg_bw_mall=(lib.mp.FractionOfUrgentBandwidthMALL*1000.0) as u32; } regs.num_watermark_sets=2;
    let w=&mut mcif.wm_regs[DML2_DCHUB_WATERMARK_SET_A]; w.urgent=(lib.mp.Watermark.WritebackUrgentWatermark*1000.0) as u32; w.uclk_pstate=(lib.mp.Watermark.WritebackDRAMClockChangeWatermark*1000.0) as u32; w.fclk_pstate=(lib.mp.Watermark.WritebackFCLKChangeWatermark*1000.0) as u32; w.temp_read_or_ppt=(lib.mp.Watermark.writeback_temp_read_or_ppt_watermark_us*1000.0) as u32;
    for set in [DML2_DCHUB_WATERMARK_SET_B,DML2_DCHUB_WATERMARK_SET_C,DML2_DCHUB_WATERMARK_SET_D] { memcpy((&mut mcif.wm_regs[set]) as *mut _ as *mut _, w as *const _ as *const _, core::mem::size_of_val(w)); } mcif.num_watermark_sets=4; true
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
