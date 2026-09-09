/* SPDX-License-Identifier: MIT */
/* Copyright 2023 Advanced Micro Devices, Inc. */

// Dependency declarations and types are supplied by the corresponding DML headers.

unsafe fn get_optimal_ntuple(socbb: *const soc_bounding_box_st, entry: *mut soc_state_bounding_box_st) {
    if (*entry).dcfclk_mhz > 0 {
        let bw_on_sdp = (*entry).dcfclk_mhz as f32 * (*socbb).return_bus_width_bytes as f32
            * ((*socbb).pct_ideal_sdp_bw_after_urgent as f32 / 100.0);
        (*entry).fabricclk_mhz = bw_on_sdp / ((*socbb).return_bus_width_bytes as f32
            * (*socbb).pct_ideal_fabric_bw_after_urgent as f32 / 100.0);
        (*entry).dram_speed_mts = bw_on_sdp / ((*socbb).num_chans as f32
            * (*socbb).dram_channel_width_bytes as f32
            * (*socbb).pct_ideal_dram_bw_after_urgent_pixel_only as f32 / 100.0);
    } else if (*entry).fabricclk_mhz > 0 {
        let bw_on_fabric = (*entry).fabricclk_mhz as f32 * (*socbb).return_bus_width_bytes as f32
            * (*socbb).pct_ideal_fabric_bw_after_urgent as f32 / 100.0;
        (*entry).dcfclk_mhz = bw_on_fabric / ((*socbb).return_bus_width_bytes as f32
            * (*socbb).pct_ideal_sdp_bw_after_urgent as f32 / 100.0);
        (*entry).dram_speed_mts = bw_on_fabric / ((*socbb).num_chans as f32
            * (*socbb).dram_channel_width_bytes as f32
            * (*socbb).pct_ideal_dram_bw_after_urgent_pixel_only as f32 / 100.0);
    } else if (*entry).dram_speed_mts > 0 {
        let bw_on_dram = (*entry).dram_speed_mts as f32 * (*socbb).num_chans as f32
            * (*socbb).dram_channel_width_bytes as f32
            * (*socbb).pct_ideal_dram_bw_after_urgent_pixel_only as f32 / 100.0;
        (*entry).fabricclk_mhz = bw_on_dram / ((*socbb).return_bus_width_bytes as f32
            * (*socbb).pct_ideal_fabric_bw_after_urgent as f32 / 100.0);
        (*entry).dcfclk_mhz = bw_on_dram / ((*socbb).return_bus_width_bytes as f32
            * (*socbb).pct_ideal_sdp_bw_after_urgent as f32 / 100.0);
    }
}

unsafe fn calculate_net_bw_in_mbytes_sec(socbb: *const soc_bounding_box_st, entry: *mut soc_state_bounding_box_st) -> f32 {
    let memory = (*entry).dram_speed_mts as f32 * (*socbb).num_chans as f32 * (*socbb).dram_channel_width_bytes as f32 * (*socbb).pct_ideal_dram_bw_after_urgent_pixel_only as f32 / 100.0;
    let fabric = (*entry).fabricclk_mhz as f32 * (*socbb).return_bus_width_bytes as f32 * (*socbb).pct_ideal_fabric_bw_after_urgent as f32 / 100.0;
    let sdp = (*entry).dcfclk_mhz as f32 * (*socbb).return_bus_width_bytes as f32 * (*socbb).pct_ideal_sdp_bw_after_urgent as f32 / 100.0;
    memory.min(fabric).min(sdp)
}

unsafe fn insert_entry_into_table_sorted(socbb: *const soc_bounding_box_st, table: *mut soc_states_st, entry: *mut soc_state_bounding_box_st) {
    let mut index: usize = 0;
    get_optimal_ntuple(socbb, entry);
    if (*table).num_states != 0 {
        let bw = calculate_net_bw_in_mbytes_sec(socbb, entry);
        while bw > calculate_net_bw_in_mbytes_sec(socbb, &mut (*table).state_array[index]) {
            index += 1;
            if index >= (*table).num_states as usize { break; }
        }
        for i in (index..(*table).num_states as usize).rev() { (*table).state_array[i + 1] = (*table).state_array[i]; }
    }
    (*table).state_array[index] = *entry;
    (*table).state_array[index].dcfclk_mhz = (*entry).dcfclk_mhz as i32;
    (*table).state_array[index].fabricclk_mhz = (*entry).fabricclk_mhz as i32;
    (*table).state_array[index].dram_speed_mts = (*entry).dram_speed_mts as i32;
    (*table).num_states += 1;
}

unsafe fn remove_entry_from_table_at_index(table: *mut soc_states_st, index: usize) {
    if (*table).num_states == 0 { return; }
    for i in index..((*table).num_states as usize - 1) { (*table).state_array[i] = (*table).state_array[i + 1]; }
    (*table).num_states -= 1;
    core::ptr::write_bytes(&mut (*table).state_array[(*table).num_states as usize], 0, 1);
}

pub unsafe fn dml2_policy_build_synthetic_soc_states(s: *mut dml2_policy_build_synthetic_soc_states_scratch, p: *mut dml2_policy_build_synthetic_soc_states_params) -> i32 {
    let mut min_fclk = (*p).in_states.state_array[0].fabricclk_mhz as u32;
    let mut min_dcfclk = (*p).in_states.state_array[0].dcfclk_mhz as u32;
    let mut min_socclk = (*p).in_states.state_array[0].socclk_mhz as u32;
    let (mut max_dcfclk, mut max_dispclk, mut max_dppclk, mut max_phyclk, mut max_dtbclk, mut max_fclk, mut max_uclk, mut max_socclk) = (0,0,0,0,0,0,0,0);
    let (mut num_uclk_dpms, mut num_fclk_dpms) = (0,0);
    for i in 0..__DML_MAX_STATE_ARRAY_SIZE__ as usize { let e = &(*p).in_states.state_array[i]; max_dcfclk=max_dcfclk.max(e.dcfclk_mhz as i32); max_fclk=max_fclk.max(e.fabricclk_mhz as i32); max_socclk=max_socclk.max(e.socclk_mhz as i32); max_uclk=max_uclk.max(e.dram_speed_mts as i32); max_dispclk=max_dispclk.max(e.dispclk_mhz as i32); max_dppclk=max_dppclk.max(e.dppclk_mhz as i32); max_phyclk=max_phyclk.max(e.phyclk_mhz as i32); max_dtbclk=max_dtbclk.max(e.dtbclk_mhz as i32); if e.fabricclk_mhz>0 {num_fclk_dpms+=1;} if e.dram_speed_mts>0 {num_uclk_dpms+=1;} }
    if max_dcfclk==0 || max_dispclk==0 || max_dppclk==0 || max_phyclk==0 || max_dtbclk==0 { return -1; }
    (*p).out_states.num_states=0; (*s).entry=(*p).in_states.state_array[0];
    (*s).entry.dispclk_mhz=max_dispclk; (*s).entry.dppclk_mhz=max_dppclk; (*s).entry.dtbclk_mhz=max_dtbclk; (*s).entry.phyclk_mhz=max_phyclk; (*s).entry.dscclk_mhz=max_dispclk/3;
    for i in 0..(*p).num_dcfclk_stas as usize { (*s).entry.dcfclk_mhz=(*p).dcfclk_stas_mhz[i]; (*s).entry.fabricclk_mhz=0; (*s).entry.dram_speed_mts=0; if i>0 {(*s).entry.socclk_mhz=max_socclk;} insert_entry_into_table_sorted((*p).in_bbox, &mut (*p).out_states, &mut (*s).entry); }
    for i in 0..num_uclk_dpms as usize { (*s).entry.dcfclk_mhz=0; (*s).entry.fabricclk_mhz=0; (*s).entry.dram_speed_mts=(*p).in_states.state_array[i].dram_speed_mts; (*s).entry.socclk_mhz=if i==0 {min_socclk as _} else {max_socclk}; insert_entry_into_table_sorted((*p).in_bbox, &mut (*p).out_states, &mut (*s).entry); }
    if num_fclk_dpms > 2 { for i in 0..num_fclk_dpms as usize { (*s).entry.dcfclk_mhz=0; (*s).entry.fabricclk_mhz=(*p).in_states.state_array[i].fabricclk_mhz; (*s).entry.dram_speed_mts=0; insert_entry_into_table_sorted((*p).in_bbox, &mut (*p).out_states, &mut (*s).entry); } } else { (*s).entry.dcfclk_mhz=0; (*s).entry.fabricclk_mhz=(*p).in_states.state_array[(num_fclk_dpms-1) as usize].fabricclk_mhz; (*s).entry.dram_speed_mts=0; insert_entry_into_table_sorted((*p).in_bbox, &mut (*p).out_states, &mut (*s).entry); }
    for i in (0..(*p).out_states.num_states as usize).rev() { let e=&(*p).out_states.state_array[i]; if e.dcfclk_mhz as i32>max_dcfclk || e.fabricclk_mhz as i32>max_fclk || e.dram_speed_mts as i32>max_uclk { remove_entry_from_table_at_index(&mut (*p).out_states,i); } }
    for i in (0..(*p).out_states.num_states as usize).rev() { for j in 0..num_uclk_dpms as usize { if (*p).in_states.state_array[j].dram_speed_mts>=(*p).out_states.state_array[i].dram_speed_mts { (*p).out_states.state_array[i].dram_speed_mts=(*p).in_states.state_array[j].dram_speed_mts; break; } } }
    if num_fclk_dpms>2 { for i in (0..(*p).out_states.num_states as usize).rev() { for j in 0..num_fclk_dpms as usize { if (*p).in_states.state_array[j].fabricclk_mhz>=(*p).out_states.state_array[i].fabricclk_mhz { (*p).out_states.state_array[i].fabricclk_mhz=(*p).in_states.state_array[j].fabricclk_mhz; break; } } } }
    for i in (0..(*p).out_states.num_states as usize).rev() { if (*p).out_states.state_array[i].fabricclk_mhz<min_fclk as _ {(*p).out_states.state_array[i].fabricclk_mhz=min_fclk as _;} if (*p).out_states.state_array[i].dcfclk_mhz<min_dcfclk as _ {(*p).out_states.state_array[i].dcfclk_mhz=min_dcfclk as _;} }
    let mut i=0usize; while i+1<(*p).out_states.num_states as usize { let a=&(*p).out_states.state_array[i]; let b=&(*p).out_states.state_array[i+1]; if a.dcfclk_mhz==b.dcfclk_mhz && a.fabricclk_mhz==b.fabricclk_mhz && a.dram_speed_mts==b.dram_speed_mts {remove_entry_from_table_at_index(&mut (*p).out_states,i);} else {i+=1;} }
    0
}

pub unsafe fn build_unoptimized_policy_settings(project: dml_project_id, policy: *mut dml_mode_eval_policy_st) {
    for i in 0..__DML_NUM_PLANES__ as usize { (*policy).MPCCombineUse[i]=dml_mpc_as_needed_for_voltage; (*policy).ODMUse[i]=dml_odm_use_policy_combine_as_needed; (*policy).ImmediateFlipRequirement[i]=dml_immediate_flip_required; (*policy).AllowForPStateChangeOrStutterInVBlank[i]=dml_prefetch_support_uclk_fclk_and_stutter_if_possible; }
    (*policy).UseUnboundedRequesting=dml_unbounded_requesting_enable; (*policy).UseMinimumRequiredDCFCLK=false; (*policy).DRAMClockChangeRequirementFinal=true; (*policy).FCLKChangeRequirementFinal=true; (*policy).USRRetrainingRequiredFinal=true; (*policy).EnhancedPrefetchScheduleAccelerationFinal=true; (*policy).NomDETInKByteOverrideEnable=false; (*policy).NomDETInKByteOverrideValue=0; (*policy).DCCProgrammingAssumesScanDirectionUnknownFinal=true; (*policy).SynchronizeTimingsFinal=true; (*policy).SynchronizeDRRDisplaysForUCLKPStateChangeFinal=true; (*policy).AssumeModeSupportAtMaxPwrStateEvenDRAMClockChangeNotSupported=true; (*policy).AssumeModeSupportAtMaxPwrStateEvenFClockChangeNotSupported=true;
    if project==dml_project_dcn35 || project==dml_project_dcn36 || project==dml_project_dcn351 { (*policy).DCCProgrammingAssumesScanDirectionUnknownFinal=false; (*policy).EnhancedPrefetchScheduleAccelerationFinal=0; (*policy).AllowForPStateChangeOrStutterInVBlankFinal=dml_prefetch_support_uclk_fclk_and_stutter_if_possible; (*policy).UseOnlyMaxPrefetchModes=1; }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
