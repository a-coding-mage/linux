// SPDX-License-Identifier: GPL-2.0-only
/* Rust translation of cppc_cpufreq.c. Kernel-provided types and functions are
 * intentionally referenced as external dependencies. */

use core::ffi::{c_char, c_int, c_uint, c_ulong, c_void};

#[repr(C)] pub struct cpufreq_driver { pub flags: u32, pub verify: Option<unsafe extern "C" fn(*mut cpufreq_policy_data)->c_int>, pub target: Option<unsafe extern "C" fn(*mut cpufreq_policy,c_uint,c_uint)->c_int>, pub get: Option<unsafe extern "C" fn(c_uint)->c_uint>, pub fast_switch: Option<unsafe extern "C" fn(*mut cpufreq_policy,c_uint)->c_uint>, pub init: Option<unsafe extern "C" fn(*mut cpufreq_policy)->c_int>, pub exit: Option<unsafe extern "C" fn(*mut cpufreq_policy)>, pub set_boost: Option<unsafe extern "C" fn(*mut cpufreq_policy,c_int)->c_int>, pub attr: *mut *mut freq_attr, pub name: *const c_char, pub register_em: Option<unsafe extern "C" fn(*mut cpufreq_policy)> }
#[repr(C)] pub struct cpufreq_policy { pub cpu: c_uint, pub cur:u32, pub min:u32, pub max:u32, pub cpuinfo: cpufreq_cpuinfo, pub boost_enabled: bool, pub boost_supported: bool, pub transition_delay_us:u32, pub shared_type:u32, pub fast_switch_possible:bool, pub dvfs_possible_from_any_cpu:bool, pub cpus:*mut c_void, pub related_cpus:*mut c_void, pub driver_data:*mut c_void }
#[repr(C)] pub struct cpufreq_policy_data { _p: [u8;0] }
#[repr(C)] pub struct cpufreq_cpuinfo { pub min_freq:u32, pub max_freq:u32 }
#[repr(C)] pub struct freq_attr { _p:[u8;0] }
#[repr(C)] pub struct cppc_perf_caps { pub lowest_perf:u32,pub lowest_nonlinear_perf:u32,pub nominal_perf:u32,pub highest_perf:u32,pub reference_perf:u64 }
#[repr(C)] pub struct cppc_perf_ctrls { pub min_perf:u32,pub max_perf:u32,pub desired_perf:u32,pub auto_sel:bool,pub energy_perf:u64 }
#[repr(C)] pub struct cppc_cpudata { pub perf_caps:cppc_perf_caps,pub perf_ctrls:cppc_perf_ctrls,pub shared_cpu_map:*mut c_void,pub shared_type:u32 }
#[repr(C)] pub struct cppc_perf_fb_ctrs { pub reference:u64,pub delivered:u64 }
#[repr(C)] pub struct device { pub id:c_uint }

extern "C" {
    static mut cppc_cpufreq_driver: cpufreq_driver;
    fn cppc_get_perf_ctrs(c:c_int, a:*mut cppc_perf_fb_ctrs)->c_int; fn cppc_perf_ctrs_in_pcc_cpu(c:c_int)->bool; fn cppc_perf_ctrs_in_pcc()->bool;
    fn cppc_khz_to_perf(c:*const cppc_perf_caps,f:u32)->u32; fn cppc_perf_to_khz(c:*const cppc_perf_caps,p:u64)->u32; fn cppc_set_perf(c:c_uint,p:*const cppc_perf_ctrls)->c_int;
    fn cppc_get_transition_latency(c:c_uint)->c_int; fn cppc_allow_fast_switch(m:*mut c_void)->bool; fn cppc_get_desired_perf(c:c_uint,p:*mut u64)->c_int;
    fn cppc_get_auto_sel(c:c_uint,v:*mut bool)->c_int; fn cppc_set_auto_sel(c:c_uint,v:bool)->c_int; fn cppc_get_auto_act_window(c:c_int,v:*mut u64)->c_int; fn cppc_set_auto_act_window(c:c_int,v:u64)->c_int;
    fn cppc_get_epp_perf(c:c_int,v:*mut u64)->c_int; fn cppc_set_epp(c:c_uint,v:u64)->c_int; fn cppc_get_perf_limited(c:c_int,v:*mut u64)->c_int; fn cppc_set_perf_limited(c:c_int,v:u64)->c_int;
    fn acpi_cpc_valid()->bool; fn acpi_get_psd_map(c:c_uint,d:*mut cppc_cpudata)->c_int; fn cppc_get_perf_caps(c:c_uint,p:*mut cppc_perf_caps)->c_int; fn cppc_get_perf(c:c_uint,p:*mut cppc_perf_ctrls)->c_int;
    fn cpufreq_register_driver(d:*mut cpufreq_driver)->c_int; fn cpufreq_unregister_driver(d:*mut cpufreq_driver); fn cpufreq_verify_within_cpu_limits(p:*mut cpufreq_policy_data); fn cpufreq_cpu_get_raw(c:c_uint)->*mut cpufreq_policy; fn cpufreq_cpu_get(c:c_uint)->*mut cpufreq_policy;
    fn cpufreq_freq_transition_begin(p:*mut cpufreq_policy,f:*mut cpufreq_freqs); fn cpufreq_freq_transition_end(p:*mut cpufreq_policy,f:*mut cpufreq_freqs,b:bool); fn cpufreq_show_cpus(m:*mut c_void,b:*mut c_char)->isize;
}
#[repr(C)] pub struct cpufreq_freqs { pub old:u32,pub new:u32 }

unsafe fn cppc_cpufreq_get_perf_limits(d:*mut cppc_cpudata,p:*mut cpufreq_policy,min:*mut u32,max:*mut u32) { let c=&(*d).perf_caps; let mut lo=(*p).min; let hi=(*p).max; if lo>hi {lo=hi;} *min=cppc_khz_to_perf(c,lo).clamp(c.lowest_perf,c.highest_perf); *max=cppc_khz_to_perf(c,hi).clamp(c.lowest_perf,c.highest_perf); }
unsafe fn cppc_cpufreq_update_perf_limits(d:*mut cppc_cpudata,p:*mut cpufreq_policy) { cppc_cpufreq_get_perf_limits(d,p,&mut (*d).perf_ctrls.min_perf,&mut (*d).perf_ctrls.max_perf); }
unsafe extern "C" fn cppc_cpufreq_set_target(p:*mut cpufreq_policy,target:c_uint,_relation:c_uint)->c_int { let d=(*p).driver_data as *mut cppc_cpudata; (*d).perf_ctrls.desired_perf=cppc_khz_to_perf(&(*d).perf_caps,target); cppc_cpufreq_update_perf_limits(d,p); let mut f=cpufreq_freqs{old:(*p).cur,new:target}; cpufreq_freq_transition_begin(p,&mut f); let r=cppc_set_perf((*p).cpu,&(*d).perf_ctrls); cpufreq_freq_transition_end(p,&mut f,r!=0); r }
unsafe extern "C" fn cppc_cpufreq_fast_switch(p:*mut cpufreq_policy,target:c_uint)->c_uint { let d=(*p).driver_data as *mut cppc_cpudata; (*d).perf_ctrls.desired_perf=cppc_khz_to_perf(&(*d).perf_caps,target); cppc_cpufreq_update_perf_limits(d,p); if cppc_set_perf((*p).cpu,&(*d).perf_ctrls)!=0 {0} else {target} }
unsafe extern "C" fn cppc_verify_policy(p:*mut cpufreq_policy_data)->c_int { cpufreq_verify_within_cpu_limits(p); 0 }
unsafe fn __cppc_cpufreq_get_transition_delay_us(cpu:c_uint)->u32 { let n=cppc_get_transition_latency(cpu); if n<0 {1000} else {(n as u32)/1000} }
unsafe extern "C" fn cppc_cpufreq_get_transition_delay_us(cpu:c_uint)->u32 { __cppc_cpufreq_get_transition_delay_us(cpu) }

unsafe fn get_delta(t1:u64,t0:u64)->u64 { if t1>t0 || t0>u32::MAX as u64 {t1.wrapping_sub(t0)} else {(t1 as u32).wrapping_sub(t0 as u32) as u64} }
unsafe fn cppc_perf_from_fbctrs(reference:u64,a:*const cppc_perf_fb_ctrs,b:*const cppc_perf_fb_ctrs)->u64 { let dr=get_delta((*b).reference,(*a).reference); let dd=get_delta((*b).delivered,(*a).delivered); if dr==0||dd==0 {0} else {reference.wrapping_mul(dd)/dr} }
unsafe fn cppc_get_perf_ctrs_sample(cpu:c_int,a:*mut cppc_perf_fb_ctrs,b:*mut cppc_perf_fb_ctrs)->c_int { let r=cppc_get_perf_ctrs(cpu,a); if r!=0{return r;} /* udelay(2) */ cppc_get_perf_ctrs(cpu,b) }
unsafe extern "C" fn cppc_cpufreq_get_rate(cpu:c_uint)->c_uint { let p=cpufreq_cpu_get(cpu); if p.is_null(){return 0;} let d=(*p).driver_data as *mut cppc_cpudata; let mut a=cppc_perf_fb_ctrs{reference:0,delivered:0}; let mut b=a; if cppc_get_perf_ctrs_sample(cpu as c_int,&mut a,&mut b)!=0 { return cppc_perf_to_khz(&(*d).perf_caps,(*d).perf_ctrls.desired_perf); } let perf=cppc_perf_from_fbctrs((*d).perf_caps.reference_perf,&a,&b); if perf==0 {cppc_perf_to_khz(&(*d).perf_caps,(*d).perf_ctrls.desired_perf)} else {cppc_perf_to_khz(&(*d).perf_caps,perf)} }

unsafe extern "C" fn cppc_cpufreq_set_boost(p:*mut cpufreq_policy,state:c_int)->c_int { let d=(*p).driver_data as *mut cppc_cpudata; (*p).cpuinfo.max_freq=cppc_perf_to_khz(&(*d).perf_caps,if state!=0 {(*d).perf_caps.highest_perf as u64}else{(*d).perf_caps.nominal_perf as u64}); 0 }
unsafe extern "C" fn cppc_cpufreq_init()->c_int { if !acpi_cpc_valid(){return -19;} cpufreq_register_driver(&mut cppc_cpufreq_driver) }
unsafe extern "C" fn cppc_cpufreq_exit(){cpufreq_unregister_driver(&mut cppc_cpufreq_driver)}

#[no_mangle] pub static mut cppc_cpufreq_driver_impl:cpufreq_driver=cpufreq_driver{flags:0,verify:Some(cppc_verify_policy),target:Some(cppc_cpufreq_set_target),get:Some(cppc_cpufreq_get_rate),fast_switch:Some(cppc_cpufreq_fast_switch),init:None,exit:None,set_boost:Some(cppc_cpufreq_set_boost),attr:core::ptr::null_mut(),name:b"cppc_cpufreq\0".as_ptr() as *const c_char,register_em:None};

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
