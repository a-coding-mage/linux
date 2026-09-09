// SPDX-License-Identifier: MIT
/* Rust translation of amdgpu_dm_pp_smu.c. */

// External kernel/repository types, constants, macros, and functions are supplied by
// the surrounding translation unit.

unsafe fn build_pm_display_cfg(pm: *mut amd_pp_display_configuration, pp: *const dm_pp_display_configuration) {
    core::ptr::write_bytes(pm as *mut u8, 0, core::mem::size_of::<amd_pp_display_configuration>());
    (*pm).cpu_cc6_disable = (*pp).cpu_cc6_disable;
    (*pm).cpu_pstate_disable = (*pp).cpu_pstate_disable;
    (*pm).cpu_pstate_separation_time = (*pp).cpu_pstate_separation_time;
    (*pm).nb_pstate_switch_disable = (*pp).nb_pstate_switch_disable;
    (*pm).num_display = (*pp).display_count;
    (*pm).num_path_including_non_display = (*pp).display_count;
    (*pm).min_core_set_clock = (*pp).min_engine_clock_khz / 10;
    (*pm).min_core_set_clock_in_sr = (*pp).min_engine_clock_deep_sleep_khz / 10;
    (*pm).min_mem_set_clock = (*pp).min_memory_clock_khz / 10;
    (*pm).min_dcef_deep_sleep_set_clk = (*pp).min_engine_clock_deep_sleep_khz / 10;
    (*pm).min_dcef_set_clk = (*pp).min_dcfclock_khz / 10;
    (*pm).multi_monitor_in_sync = (*pp).all_displays_in_sync;
    (*pm).min_vblank_time = (*pp).avail_mclk_switch_time_us;
    (*pm).display_clk = (*pp).disp_clk_khz / 10;
    (*pm).dce_tolerable_mclk_in_active_latency = (*pp).avail_mclk_switch_time_in_disp_active_us;
    (*pm).crtc_index = (*pp).crtc_index;
    (*pm).line_time_in_us = (*pp).line_time_in_us;
    (*pm).vrefresh = (*pp).disp_configs[0].v_refresh;
    (*pm).crossfire_display_index = -1;
    (*pm).min_bus_bandwidth = 0;
    for i in 0..(*pp).display_count {
        let dc = &(*pp).disp_configs[i];
        (*pm).displays[i].controller_id = dc.pipe_idx + 1;
        (*pm).displays[i].pixel_clock = dc.pixel_clock;
    }
}

pub unsafe fn dm_pp_apply_display_requirements(ctx: *const dc_context, pp: *const dm_pp_display_configuration) -> bool {
    let adev = (*ctx).driver_context;
    if (*adev).pm.dpm_enabled {
        build_pm_display_cfg(&mut (*adev).pm.pm_display_cfg, pp);
        amdgpu_dpm_display_configuration_change(adev, &(*adev).pm.pm_display_cfg);
        amdgpu_dpm_compute_clocks(adev);
    }
    true
}

unsafe fn get_default_clock_levels(t: dm_pp_clock_type, clks: *mut dm_pp_clock_levels) {
    let disp = [300000u32,400000,496560,626090,685720,757900];
    let s = [300000u32,360000,423530,514290,626090,720000];
    let m = [333000u32,800000];
    match t {
        DM_PP_CLOCK_TYPE_DISPLAY_CLK => { (*clks).num_levels=6; (*clks).clocks_in_khz[..6].copy_from_slice(&disp); }
        DM_PP_CLOCK_TYPE_ENGINE_CLK => { (*clks).num_levels=6; (*clks).clocks_in_khz[..6].copy_from_slice(&s); }
        DM_PP_CLOCK_TYPE_MEMORY_CLK => { (*clks).num_levels=2; (*clks).clocks_in_khz[..2].copy_from_slice(&m); }
        _ => (*clks).num_levels=0,
    }
}

unsafe fn dc_to_pp_clock_type(t: dm_pp_clock_type) -> amd_pp_clock_type {
    match t {
        DM_PP_CLOCK_TYPE_DISPLAY_CLK => amd_pp_disp_clock,
        DM_PP_CLOCK_TYPE_ENGINE_CLK => amd_pp_sys_clock,
        DM_PP_CLOCK_TYPE_MEMORY_CLK => amd_pp_mem_clock,
        DM_PP_CLOCK_TYPE_DCEFCLK => amd_pp_dcef_clock,
        DM_PP_CLOCK_TYPE_DCFCLK => amd_pp_dcf_clock,
        DM_PP_CLOCK_TYPE_PIXELCLK => amd_pp_pixel_clock,
        DM_PP_CLOCK_TYPE_FCLK => amd_pp_f_clock,
        DM_PP_CLOCK_TYPE_DISPLAYPHYCLK => amd_pp_phy_clock,
        DM_PP_CLOCK_TYPE_DPPCLK => amd_pp_dpp_clock,
        _ => { DRM_ERROR!("DM_PPLIB: invalid clock type: %d!\n", t); 0 }
    }
}

unsafe fn pp_to_dc_clock_levels(pp: *const amd_pp_clocks, dc: *mut dm_pp_clock_levels, _t: dm_pp_clock_type) {
    (*dc).num_levels = if (*pp).count > DM_PP_MAX_CLOCK_LEVELS { DM_PP_MAX_CLOCK_LEVELS } else { (*pp).count };
    for i in 0..(*dc).num_levels { (*dc).clocks_in_khz[i] = (*pp).clock[i]; }
}
unsafe fn pp_to_dc_clock_levels_with_latency(pp: *const pp_clock_levels_with_latency, dc: *mut dm_pp_clock_levels_with_latency, _t: dm_pp_clock_type) {
    (*dc).num_levels = if (*pp).num_levels > DM_PP_MAX_CLOCK_LEVELS { DM_PP_MAX_CLOCK_LEVELS } else { (*pp).num_levels };
    for i in 0..(*dc).num_levels { (*dc).data[i].clocks_in_khz=(*pp).data[i].clocks_in_khz; (*dc).data[i].latency_in_us=(*pp).data[i].latency_in_us; }
}
unsafe fn pp_to_dc_clock_levels_with_voltage(pp: *const pp_clock_levels_with_voltage, dc: *mut dm_pp_clock_levels_with_voltage, _t: dm_pp_clock_type) {
    (*dc).num_levels = if (*pp).num_levels > DM_PP_MAX_CLOCK_LEVELS { DM_PP_MAX_CLOCK_LEVELS } else { (*pp).num_levels };
    for i in 0..(*dc).num_levels { (*dc).data[i].clocks_in_khz=(*pp).data[i].clocks_in_khz; (*dc).data[i].voltage_in_mv=(*pp).data[i].voltage_in_mv; }
}

unsafe fn cap_clock_levels_to_validation(dc: *mut dm_pp_clock_levels, t: dm_pp_clock_type, v: *const amd_pp_simple_clock_info) {
    let max = match t { DM_PP_CLOCK_TYPE_ENGINE_CLK => (*v).engine_max_clock, DM_PP_CLOCK_TYPE_MEMORY_CLK => (*v).memory_max_clock, _ => return };
    for i in 0..(*dc).num_levels { if (*dc).clocks_in_khz[i] > max { (*dc).num_levels = if i > 0 { i } else { 1 }; break; } }
}

pub unsafe fn dm_pp_get_clock_levels_by_type(ctx:*const dc_context,t:dm_pp_clock_type,dc:*mut dm_pp_clock_levels)->bool { let a=(*ctx).driver_context; let mut p=core::mem::zeroed(); let mut v=core::mem::zeroed(); if amdgpu_dpm_get_clock_by_type(a,dc_to_pp_clock_type(t),&mut p)!=0 { get_default_clock_levels(t,dc); return true; } pp_to_dc_clock_levels(&p,dc,t); if amdgpu_dpm_get_display_mode_validation_clks(a,&mut v)!=0 { v.engine_max_clock=72000; v.memory_max_clock=80000; } v.engine_max_clock*=10; v.memory_max_clock*=10; cap_clock_levels_to_validation(dc,t,&v); true }
pub unsafe fn dm_pp_get_clock_levels_by_type_with_latency(c:*const dc_context,t:dm_pp_clock_type,d:*mut dm_pp_clock_levels_with_latency)->bool { let mut p=core::mem::zeroed(); if amdgpu_dpm_get_clock_by_type_with_latency((*c).driver_context,dc_to_pp_clock_type(t),&mut p)!=0{return false} pp_to_dc_clock_levels_with_latency(&p,d,t); true }
pub unsafe fn dm_pp_get_clock_levels_by_type_with_voltage(c:*const dc_context,t:dm_pp_clock_type,d:*mut dm_pp_clock_levels_with_voltage)->bool { let mut p=core::mem::zeroed(); if amdgpu_dpm_get_clock_by_type_with_voltage((*c).driver_context,dc_to_pp_clock_type(t),&mut p)!=0{return false} pp_to_dc_clock_levels_with_voltage(&p,d,t); true }

pub unsafe fn dm_pp_notify_wm_clock_changes(c:*const dc_context,w:*mut dm_pp_wm_sets_with_clock_ranges)->bool { let a=(*c).driver_context; if (*a).asic_type>=CHIP_POLARIS10 && (*a).asic_type<=CHIP_VEGAM && amdgpu_dpm_set_watermarks_for_clocks_ranges(a,w as *mut core::ffi::c_void)==0 {true} else {false} }
pub unsafe fn dm_pp_apply_clock_for_voltage_request(c:*const dc_context,r:*mut dm_pp_clock_for_voltage_req)->bool { let mut q=core::mem::zeroed(); q.clock_type=dc_to_pp_clock_type((*r).clk_type); q.clock_freq_in_khz=(*r).clocks_in_khz; if q.clock_type==0{return false} let n=amdgpu_dpm_display_clock_voltage_request((*c).driver_context,&mut q); n==0 || n==-EOPNOTSUPP }

// The remaining callback adapters retain the C ABI-facing behavior and delegate to
// the corresponding external amdgpu DPM operations.
pub unsafe fn pp_nv_set_display_count(p:*mut pp_smu,n:i32)->pp_smu_status { let r=amdgpu_dpm_set_active_display_count((*(*p).dm).driver_context,n); if r==-EOPNOTSUPP{PP_SMU_RESULT_UNSUPPORTED}else if r!=0{PP_SMU_RESULT_FAIL}else{PP_SMU_RESULT_OK} }
pub unsafe fn pp_nv_set_min_deep_sleep_dcfclk(p:*mut pp_smu,n:i32)->pp_smu_status { let r=amdgpu_dpm_set_min_deep_sleep_dcefclk((*(*p).dm).driver_context,n); if r==-EOPNOTSUPP{PP_SMU_RESULT_UNSUPPORTED}else if r!=0{PP_SMU_RESULT_FAIL}else{PP_SMU_RESULT_OK} }
pub unsafe fn pp_nv_set_hard_min_dcefclk_by_freq(p:*mut pp_smu,n:i32)->pp_smu_status { let mut q=core::mem::zeroed();q.clock_type=amd_pp_dcef_clock;q.clock_freq_in_khz=n*1000;let r=amdgpu_dpm_display_clock_voltage_request((*(*p).dm).driver_context,&mut q);if r==-EOPNOTSUPP{PP_SMU_RESULT_UNSUPPORTED}else if r!=0{PP_SMU_RESULT_FAIL}else{PP_SMU_RESULT_OK} }
pub unsafe fn pp_nv_set_hard_min_uclk_by_freq(p:*mut pp_smu,n:i32)->pp_smu_status { let mut q=core::mem::zeroed();q.clock_type=amd_pp_mem_clock;q.clock_freq_in_khz=n*1000;let r=amdgpu_dpm_display_clock_voltage_request((*(*p).dm).driver_context,&mut q);if r==-EOPNOTSUPP{PP_SMU_RESULT_UNSUPPORTED}else if r!=0{PP_SMU_RESULT_FAIL}else{PP_SMU_RESULT_OK} }
pub unsafe fn pp_nv_set_pstate_handshake_support(p:*mut pp_smu,b:bool)->pp_smu_status { if amdgpu_dpm_display_disable_memory_clock_switch((*(*p).dm).driver_context,!b)!=0{PP_SMU_RESULT_FAIL}else{PP_SMU_RESULT_OK} }
pub unsafe fn pp_nv_set_wm_ranges(p:*mut pp_smu,r:*mut pp_smu_wm_range_sets)->pp_smu_status { amdgpu_dpm_set_watermarks_for_clocks_ranges((*(*p).dm).driver_context,r); PP_SMU_RESULT_OK }
pub unsafe fn pp_rv_set_pme_wa_enable(p:*mut pp_smu){amdgpu_dpm_notify_smu_enable_pwe((*(*p).dm).driver_context)}
pub unsafe fn pp_rv_set_active_display_count(p:*mut pp_smu,n:i32){amdgpu_dpm_set_active_display_count((*(*p).dm).driver_context,n)}
pub unsafe fn pp_rv_set_min_deep_sleep_dcfclk(p:*mut pp_smu,n:i32){amdgpu_dpm_set_min_deep_sleep_dcefclk((*(*p).dm).driver_context,n)}
pub unsafe fn pp_rv_set_hard_min_dcefclk_by_freq(p:*mut pp_smu,n:i32){amdgpu_dpm_set_hard_min_dcefclk_by_freq((*(*p).dm).driver_context,n)}
pub unsafe fn pp_rv_set_hard_min_fclk_by_freq(p:*mut pp_smu,n:i32){amdgpu_dpm_set_hard_min_fclk_by_freq((*(*p).dm).driver_context,n)}
pub unsafe fn build_wm_clock_ranges_soc15(r:*const pp_smu_wm_range_sets,w:*mut dm_pp_wm_sets_with_clock_ranges_soc15){(*w).num_wm_dmif_sets=(*r).num_reader_wm_sets;(*w).num_wm_mcif_sets=(*r).num_writer_wm_sets;for i in 0..(*w).num_wm_dmif_sets{let x=&(*r).reader_wm_sets[i];let d=&mut (*w).wm_dmif_clocks_ranges[i];d.wm_set_id=if x.wm_inst>3{WM_SET_A}else{x.wm_inst};d.wm_max_dcfclk_clk_in_khz=x.max_drain_clk_mhz*1000;d.wm_min_dcfclk_clk_in_khz=x.min_drain_clk_mhz*1000;d.wm_max_mem_clk_in_khz=x.max_fill_clk_mhz*1000;d.wm_min_mem_clk_in_khz=x.min_fill_clk_mhz*1000}for i in 0..(*w).num_wm_mcif_sets{let x=&(*r).writer_wm_sets[i];let d=&mut (*w).wm_mcif_clocks_ranges[i];d.wm_set_id=if x.wm_inst>3{WM_SET_A}else{x.wm_inst};d.wm_max_socclk_clk_in_khz=x.max_fill_clk_mhz*1000;d.wm_min_socclk_clk_in_khz=x.min_fill_clk_mhz*1000;d.wm_max_mem_clk_in_khz=x.max_drain_clk_mhz*1000;d.wm_min_mem_clk_in_khz=x.min_drain_clk_mhz*1000}}
pub unsafe fn pp_rv_set_wm_ranges(p:*mut pp_smu,r:*mut pp_smu_wm_range_sets){let mut w=core::mem::zeroed();build_wm_clock_ranges_soc15(r,&mut w);amdgpu_dpm_set_watermarks_for_clocks_ranges((*(*p).dm).driver_context,&mut w)}

pub unsafe fn pp_nv_set_voltage_by_freq(p:*mut pp_smu,id:pp_smu_nv_clock_id,n:i32)->pp_smu_status { let t=match id{PP_SMU_NV_DISPCLK=>amd_pp_disp_clock,PP_SMU_NV_PHYCLK=>amd_pp_phy_clock,PP_SMU_NV_PIXELCLK=>amd_pp_pixel_clock,_=>return PP_SMU_RESULT_FAIL};let mut q=core::mem::zeroed();q.clock_type=t;q.clock_freq_in_khz=n*1000;let r=amdgpu_dpm_display_clock_voltage_request((*(*p).dm).driver_context,&mut q);if r==-EOPNOTSUPP{PP_SMU_RESULT_UNSUPPORTED}else if r!=0{PP_SMU_RESULT_FAIL}else{PP_SMU_RESULT_OK} }
pub unsafe fn pp_nv_get_maximum_sustainable_clocks(p:*mut pp_smu,c:*mut pp_smu_nv_clock_table)->pp_smu_status {let r=amdgpu_dpm_get_max_sustainable_clocks_by_dc((*(*p).dm).driver_context,c);if r==-EOPNOTSUPP{PP_SMU_RESULT_UNSUPPORTED}else if r!=0{PP_SMU_RESULT_FAIL}else{PP_SMU_RESULT_OK}}
pub unsafe fn pp_nv_get_uclk_dpm_states(p:*mut pp_smu,c:*mut u32,n:*mut u32)->pp_smu_status {let r=amdgpu_dpm_get_uclk_dpm_states((*(*p).dm).driver_context,c,n);if r==-EOPNOTSUPP{PP_SMU_RESULT_UNSUPPORTED}else if r!=0{PP_SMU_RESULT_FAIL}else{PP_SMU_RESULT_OK}}
pub unsafe fn pp_rn_get_dpm_clock_table(p:*mut pp_smu,c:*mut dpm_clocks)->pp_smu_status {let r=amdgpu_dpm_get_dpm_clock_table((*(*p).dm).driver_context,c);if r==-EOPNOTSUPP{PP_SMU_RESULT_UNSUPPORTED}else if r!=0{PP_SMU_RESULT_FAIL}else{PP_SMU_RESULT_OK}}

pub unsafe fn dm_pp_get_funcs(ctx:*mut dc_context,funcs:*mut pp_smu_funcs){match (*ctx).dce_version{DCN_VERSION_1_0|DCN_VERSION_1_01=>{(*funcs).ctx.ver=PP_SMU_VER_RV;(*funcs).rv_funcs.pp_smu.dm=ctx;(*funcs).rv_funcs.set_wm_ranges=Some(pp_rv_set_wm_ranges);(*funcs).rv_funcs.set_pme_wa_enable=Some(pp_rv_set_pme_wa_enable);(*funcs).rv_funcs.set_display_count=Some(pp_rv_set_active_display_count);(*funcs).rv_funcs.set_min_deep_sleep_dcfclk=Some(pp_rv_set_min_deep_sleep_dcfclk);(*funcs).rv_funcs.set_hard_min_dcfclk_by_freq=Some(pp_rv_set_hard_min_dcefclk_by_freq);(*funcs).rv_funcs.set_hard_min_fclk_by_freq=Some(pp_rv_set_hard_min_fclk_by_freq)},DCN_VERSION_2_0=>{(*funcs).ctx.ver=PP_SMU_VER_NV;(*funcs).nv_funcs.pp_smu.dm=ctx;(*funcs).nv_funcs.set_display_count=Some(pp_nv_set_display_count);(*funcs).nv_funcs.set_hard_min_dcfclk_by_freq=Some(pp_nv_set_hard_min_dcefclk_by_freq);(*funcs).nv_funcs.set_min_deep_sleep_dcfclk=Some(pp_nv_set_min_deep_sleep_dcfclk);(*funcs).nv_funcs.set_voltage_by_freq=Some(pp_nv_set_voltage_by_freq);(*funcs).nv_funcs.set_wm_ranges=Some(pp_nv_set_wm_ranges);(*funcs).nv_funcs.set_hard_min_uclk_by_freq=Some(pp_nv_set_hard_min_uclk_by_freq);(*funcs).nv_funcs.get_maximum_sustainable_clocks=Some(pp_nv_get_maximum_sustainable_clocks);(*funcs).nv_funcs.get_uclk_dpm_states=Some(pp_nv_get_uclk_dpm_states);(*funcs).nv_funcs.set_pstate_handshake_support=Some(pp_nv_set_pstate_handshake_support)},DCN_VERSION_2_1=>{(*funcs).ctx.ver=PP_SMU_VER_RN;(*funcs).rn_funcs.pp_smu.dm=ctx;(*funcs).rn_funcs.set_wm_ranges=Some(pp_nv_set_wm_ranges);(*funcs).rn_funcs.get_dpm_clock_table=Some(pp_rn_get_dpm_clock_table)},_=>{DRM_ERROR!("smu version is not supported !\n")}}}

pub unsafe fn amdgpu_dm_smu_write_watermarks_table(a:*mut amdgpu_device)->i32 {match amdgpu_ip_version(a,DCE_HWIP,0){IP_VERSION!(2,0,2)|IP_VERSION!(2,0,0)=>{},_=>return 0}let r=amdgpu_dpm_write_watermarks_table(a);if r!=0{drm_err!(adev_to_drm(a),"Failed to update WMTABLE!\n");return r}0}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
