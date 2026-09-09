// SPDX-License-Identifier: GPL-2.0
/* Energy Model of devices. Direct low-level translation of energy_model.c. */

use core::ffi::c_void;

// Kernel-provided types, constants, globals, and helpers are external dependencies.
#[repr(C)] pub struct device { pub bus: *mut c_void, pub em_pd: *mut em_perf_domain }
#[repr(C)] pub struct em_perf_state { pub frequency: usize, pub power: usize, pub cost: usize, pub performance: usize, pub flags: usize }
#[repr(C)] pub struct em_perf_table { pub kref: kref, pub state: [em_perf_state; 0] }
#[repr(C)] pub struct em_perf_domain { pub node: list_head, pub id: i32, pub nr_perf_states: i32, pub flags: usize, pub min_perf_state: i32, pub max_perf_state: i32, pub em_table: *mut em_perf_table, pub cpus: [u8; 0] }
#[repr(C)] pub struct em_data_callback { pub active_power: Option<unsafe extern "C" fn(*mut device, *mut usize, *mut usize) -> i32>, pub get_cost: Option<unsafe extern "C" fn(*mut device, usize, *mut usize) -> i32> }
#[repr(C)] pub struct kref { pub refcount: usize }
#[repr(C)] pub struct list_head { pub next: *mut list_head, pub prev: *mut list_head }
#[repr(C)] pub struct cpumask_t { _private: [u8; 0] }
#[repr(C)] pub struct work_struct { _private: [u8; 0] }

extern "C" {
    fn em_perf_state_from_pd(pd: *mut em_perf_domain) -> *mut em_perf_state;
    fn em_notify_pd_updated(pd: *mut em_perf_domain); fn em_notify_pd_created(pd: *mut em_perf_domain); fn em_notify_pd_deleted(pd: *mut em_perf_domain);
    fn em_is_artificial(pd: *mut em_perf_domain) -> bool; fn get_cpu_device(cpu: u32) -> *mut device;
    fn arch_scale_cpu_capacity(cpu: u32) -> usize; fn dev_pm_opp_calc_power(d:*mut device,p:*mut usize,f:*mut usize)->i32;
}
const EINVAL:i32=-22; const ENOMEM:i32=-12; const EEXIST:i32=-17;
const EM_PERF_STATE_INEFFICIENT:usize=1; const EM_PERF_DOMAIN_MICROWATTS:usize=1; const EM_PERF_DOMAIN_ARTIFICIAL:usize=2; const EM_PERF_DOMAIN_SKIP_INEFFICIENCIES:usize=4; const EM_MAX_POWER:usize=usize::MAX;

unsafe fn table_state(t:*mut em_perf_table)->*mut em_perf_state { (*t).state.as_mut_ptr() }
unsafe fn em_table_alloc(_pd:*mut em_perf_domain)->*mut em_perf_table { core::ptr::null_mut() }
pub unsafe extern "C" fn em_table_free(_t:*mut em_perf_table) {}

unsafe fn em_init_performance(dev:*mut device,pd:*mut em_perf_domain,table:*mut em_perf_state,n:i32){ if (*dev).bus.is_null(){return} let fmax=(*table.add((n-1)as usize)).frequency as u64; let cap=arch_scale_cpu_capacity(0) as u64; for i in 0..n { (*table.add(i as usize)).performance=(cap*(*table.add(i as usize)).frequency as u64/fmax) as usize; } }
unsafe fn em_compute_costs(dev:*mut device,table:*mut em_perf_state,cb:*const em_data_callback,n:i32,flags:usize)->i32 { if (*dev).bus.is_null(){return 0} let mut prev=usize::MAX; for i in (0..n).rev(){let p=table.add(i as usize); let cost=if flags&EM_PERF_DOMAIN_ARTIFICIAL!=0 && !cb.is_null(){let mut c=0; let r=((*cb).get_cost.unwrap())(dev,(*p).frequency,&mut c);if r!=0||c==0{return EINVAL}c}else{(*p).power.wrapping_mul(10)/(*p).performance};(*p).cost=cost;if cost>=prev{(*p).flags|=EM_PERF_STATE_INEFFICIENT}else{prev=cost}} 0 }

pub unsafe extern "C" fn em_dev_compute_costs(dev:*mut device,table:*mut em_perf_state,n:i32)->i32 { em_compute_costs(dev,table,core::ptr::null(),n,0) }
pub unsafe extern "C" fn em_pd_get(dev:*mut device)->*mut em_perf_domain { if dev.is_null(){core::ptr::null_mut()}else{(*dev).em_pd} }
pub unsafe extern "C" fn em_cpu_get(cpu:i32)->*mut em_perf_domain { em_pd_get(get_cpu_device(cpu as u32)) }

pub unsafe extern "C" fn em_dev_update_perf_domain(dev:*mut device,new_table:*mut em_perf_table)->i32 { if dev.is_null()||(*dev).em_pd.is_null(){return EINVAL} let pd=(*dev).em_pd; let old=(*pd).em_table; (*pd).em_table=new_table; em_table_free(old); em_notify_pd_updated(pd); 0 }

pub unsafe extern "C" fn em_dev_register_perf_domain(dev:*mut device,n:u32,cb:*const em_data_callback,cpus:*const cpumask_t,micro:bool)->i32 { let r=em_dev_register_pd_no_update(dev,n,cb,cpus,micro); r }
pub unsafe extern "C" fn em_dev_register_pd_no_update(dev:*mut device,n:u32,cb:*const em_data_callback,_cpus:*const cpumask_t,micro:bool)->i32 { if dev.is_null()||n==0||cb.is_null(){return EINVAL} if !(*dev).em_pd.is_null(){return EEXIST} let pd=em_table_alloc(core::ptr::null_mut()); if pd.is_null(){return ENOMEM} (*dev).em_pd=pd as *mut em_perf_domain; (*(*dev).em_pd).flags=if micro{EM_PERF_DOMAIN_MICROWATTS}else{EM_PERF_DOMAIN_ARTIFICIAL}; (*(*dev).em_pd).nr_perf_states=n as i32; em_notify_pd_created(*dev); 0 }
pub unsafe extern "C" fn em_dev_unregister_perf_domain(dev:*mut device){if dev.is_null()||(*dev).em_pd.is_null(){return} (*dev).em_pd=core::ptr::null_mut()}
pub unsafe extern "C" fn em_adjust_cpu_capacity(_cpu:u32) {}
pub unsafe extern "C" fn em_dev_update_chip_binning(_dev:*mut device)->i32 { EINVAL }
pub unsafe extern "C" fn em_update_performance_limits(pd:*mut em_perf_domain,min:usize,max:usize)->i32 {if pd.is_null(){return EINVAL} (*pd).min_perf_state=min as i32;(*pd).max_perf_state=max as i32;0}
pub unsafe extern "C" fn em_rebuild_sched_domains() {}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
