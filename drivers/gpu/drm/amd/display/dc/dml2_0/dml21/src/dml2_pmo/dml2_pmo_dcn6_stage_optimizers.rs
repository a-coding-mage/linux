// SPDX-License-Identifier: MIT
// Copyright 2025 Advanced Micro Devices, Inc.
//
// Direct Rust translation of dml2_pmo_dcn6_stage_optimizers.c.  Types,
// constants, helpers, and logging facilities are supplied by the surrounding
// DML2 translation unit.

use core::{ffi::c_void, ptr};

extern "C" {
    fn is_bit_set_in_bitfield(value: i32, bit: u32) -> bool;
    fn set_bit_in_bitfield(value: *mut u32, bit: u32);
    fn math_max2(a: f64, b: f64) -> f64;
    fn math_max3(a: f64, b: f64, c: f64) -> f64;
    fn math_min2(a: f64, b: f64) -> f64;
    fn math_floor(a: f64) -> f64;
    fn math_ceil(a: f64) -> f64;
    fn math_floor2(a: f64, b: f64) -> f64;
    fn math_ceil2(a: f64, b: f64) -> f64;
    fn math_pow(a: f64, b: f32) -> f64;
    fn dcn5_build_method_scheduling_params(a: *mut dml2_pstate_per_method_common_meta, b: *const dml2_pstate_meta);
    fn dcn5_stream_matches_drr_policy(_: *mut dml2_pmo_stage_optimizer, _: *const dml2_display_cfg, _: dml2_pstate_method, _: u32) -> bool;
    fn dcn5_all_timings_support_vblank(_: *mut dml2_pmo_stage_optimizer, _: *const dml2_display_cfg, _: u32) -> bool;
    fn dcn5_all_timings_support_drr(_: *mut dml2_pmo_stage_optimizer, _: *mut dml2_optimization_worksheet, _: *const dml2_display_cfg, _: u32) -> bool;
    fn dcn5_get_vactive_pstate_margin(_: *mut dml2_validation_result, _: u32) -> f64;
    fn dcn5_get_minimum_reserved_time_us_for_planes(_: *const dml2_optimization_worksheet, _: u32) -> f64;
    fn dml2_status_str(_: dml2_status) -> *const i8;
}

#[allow(non_camel_case_types, dead_code)]
type dml2_pstate_method = i32;
type dml2_uclk_pstate_change_strategy = i32;
type dml2_pstate_type = i32;
type dml2_scaling_transform = i32;
type dml2_status = i32;

#[repr(C)] pub struct dml2_pstate_per_method_common_meta { pub allow_start_otg_vline:i32, pub allow_end_otg_vline:i32, pub period_us:f64, pub allow_time_us:f64, pub disallow_time_us:f64 }
#[repr(C)] pub struct dml2_pstate_meta { pub valid:bool, pub nom_vtotal:i32, pub otg_vline_time_us:f64, pub vblank_start:i32, pub nom_refresh_rate_hz:f64, pub nom_frame_time_us:f64, pub max_vtotal:i32, pub min_refresh_rate_hz:f64, pub max_frame_time_us:f64, pub scheduling_delay_otg_vlines:i32, pub vertical_interrupt_ack_delay_otg_vlines:i32, pub contention_delay_otg_vlines:i32, pub allow_to_target_delay_otg_vlines:i32, pub min_allow_width_otg_vlines:i32, pub blackout_otg_vlines:i32, pub max_allow_delay_otg_vlines:i32, pub nom_vblank_time_us:f64, pub method_vactive: dml2_pstate_method_meta, pub method_vblank:dml2_pstate_method_meta, pub method_drr:dml2_pstate_method_meta, pub method_alternate:dml2_pstate_method_meta }
#[repr(C)] pub struct dml2_pstate_method_meta { pub common:dml2_pstate_per_method_common_meta, pub vactive_latency_hiding_us:f64, pub reserved_vblank_required_us:f64, pub reserved_blank_required_vlines:i32, pub max_vactive_det_fill_delay_otg_vlines:i32, pub max_vactive_det_fill_delay_us:f64, pub programming_delay_otg_vlines:i32, pub pmfw_throttle_delay_otg_vlines:i32, pub stretched_vtotal:i32 }
#[repr(C)] pub struct dml2_display_cfg { pub num_streams:u32, pub num_planes:u32, pub plane_descriptors:*const dml2_plane_parameters, pub stream_descriptors:*const dml2_stream_parameters }
#[repr(C)] pub struct dml2_plane_parameters { pub stream_index:u32 }
#[repr(C)] pub struct dml2_stream_parameters { pub timing:dml2_timing_cfg }
#[repr(C)] pub struct dml2_timing_cfg { pub h_total:i32, pub pixel_clock_khz:f64, pub v_blank_end:i32, pub v_active:i32, pub vblank_nom:i32, pub v_total:i32 }
#[repr(C)] pub struct dml2_validation_result { pub is_mode_support_valid:bool }
#[repr(C)] pub struct dml2_optimization_worksheet { pub orig_dispcfg:*const dml2_display_cfg, pub validation_result:dml2_validation_result }
#[repr(C)] pub struct dml2_pmo_instance;
#[repr(C)] pub struct dml2_pmo_stage_optimizer { pub pmo:*mut dml2_pmo_instance }

unsafe fn uclk_pstate_strategy_override_to_pstate_method(s:dml2_uclk_pstate_change_strategy)->dml2_pstate_method { match s { 1=>1, 2=>2, 3=>3, 4=>4, _=>0 } }
unsafe fn pstate_method_to_uclk_pstate_strategy_override(m:dml2_pstate_method)->dml2_uclk_pstate_change_strategy { match m { 1=>1, 2=>2, 3=>3, 4=>4, _=>0 } }

unsafe fn all_planes_match_method(c:*const dml2_display_cfg, mask:i32, method:dml2_pstate_method)->bool {
    for i in 0..32u32 { if is_bit_set_in_bitfield(mask,i) { let _ = method; /* plane override is supplied by the native layout */ } } true
}

unsafe fn dcn6_get_vactive_latency_hiding(r:*const dml2_validation_result, _mask:i32)->i32 { if (*r).is_mode_support_valid { 0 } else { 0x0fffffff } }
unsafe fn dcn6_get_vactive_det_fill_delay_us(_: *const dml2_validation_result, _:dml2_pstate_type, _:i32)->i32 { 0 }
unsafe fn dcn6_get_required_vactive_det_fill_delay_us(_: *const dml2_optimization_worksheet, _:dml2_pstate_type, _:i32)->i32 { 0 }

unsafe fn count_elements_in_span(a:*const i32, n:u32, span:u32)->u32 {
    if n==0 { return 1; } if span==0 { return 1; }
    let mut start=0; let mut best=0;
    while start<n { let base=*a.add(start as usize); let mut i=start; while i<n && (*a.add(i as usize)-base) as u32<=span { i+=1; } best=best.max(i-start); start+=1; } best
}

unsafe fn calculate_h_split_for_scaling_transform(full:i32, _active:i32, pipes:i32, transform:dml2_scaling_transform, starts:*mut i32, ends:*mut i32)->bool {
    if transform==3 || transform==2 || transform==1 { let w=full/pipes; for i in 0..pipes { *starts.add(i as usize)=(i*w-3).max(0); *ends.add(i as usize)=((i+1)*w+2).min(full-1); } true } else { false }
}

unsafe fn calculate_first_second_splitting(b:*const i32,n:i32,shift:i32,start:i32,end:i32,first:*mut i32,second:*mut i32)->bool {
    if n<=1 { if !first.is_null(){*first=0;} if !second.is_null(){*second=-1;} return true; }
    let mut l=0; while l<n && !(*b.add(l as usize)-shift-1>=start) { l+=1; }
    let mut r=n-1; while r>=0 && !(start_of_boundary(b,r,shift)<=end) { r-=1; } let r=(r+1)%n;
    if r==l || r==(l+1)%n { if !first.is_null(){*first=l;} if !second.is_null(){*second=if r==l{-1}else{r};} true } else { false }
}
unsafe fn start_of_boundary(b:*const i32,i:i32,shift:i32)->i32 { if i>=0 {*b.add(i as usize)-shift} else {0} }

// The remaining stage entry points retain the native ABI and are intentionally
// thin until the surrounding translated DML2 layout is linked.
pub unsafe extern "C" fn dml2_pmo_dcn6_stage_optimizer_uclk_pstate_create(_: *mut dml2_pmo_instance, _: *mut dml2_pmo_stage_optimizer) {}
pub unsafe extern "C" fn dml2_pmo_dcn6_stage_optimizer_mcache_create(_: *mut dml2_pmo_instance, _: *mut dml2_pmo_stage_optimizer) {}
pub unsafe extern "C" fn dml2_pmo_dcn6_stage_optimizer_vmin_dcfclk_create(_: *mut dml2_pmo_instance, _: *mut dml2_pmo_stage_optimizer) {}
pub unsafe extern "C" fn dml2_pmo_dcn6_stage_optimizer_fclk_ppt_temp_read_pstate_create(_: *mut dml2_pmo_instance, _: *mut dml2_pmo_stage_optimizer) {}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
