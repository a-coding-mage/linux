// SPDX-License-Identifier: GPL-2.0-only
/* RDMA resource limiting controller for cgroups. */

// Kernel dependencies supplied by the surrounding translation unit.
use core::ffi::{c_char, c_int, c_void};

pub const RDMACG_MAX_STR: &[u8] = b"max\0";

#[repr(C)]
#[derive(Copy, Clone)]
pub enum rdmacg_limit_tokens {
    RDMACG_DEVICE_INDEX,
    RDMACG_HCA_HANDLE_VAL,
    RDMACG_HCA_HANDLE_MAX,
    RDMACG_HCA_OBJECT_VAL,
    RDMACG_HCA_OBJECT_MAX,
    NR_RDMACG_LIMIT_TOKENS,
}

#[repr(C)]
pub enum rdmacg_file_type {
    RDMACG_RESOURCE_TYPE_MAX,
    RDMACG_RESOURCE_TYPE_STAT,
    RDMACG_RESOURCE_TYPE_PEAK,
}

#[repr(C)]
pub struct rdmacg_resource { pub max: c_int, pub usage: c_int, pub peak: c_int }

#[repr(C)]
pub struct rdmacg_resource_pool {
    pub device: *mut rdmacg_device,
    pub resources: [rdmacg_resource; RDMACG_RESOURCE_MAX as usize],
    pub cg_node: list_head,
    pub dev_node: list_head,
    pub usage_sum: u64,
    pub num_max_cnt: c_int,
    pub events_max: [u64; RDMACG_RESOURCE_MAX as usize],
    pub events_alloc_fail: [u64; RDMACG_RESOURCE_MAX as usize],
    pub events_local_max: [u64; RDMACG_RESOURCE_MAX as usize],
    pub events_local_alloc_fail: [u64; RDMACG_RESOURCE_MAX as usize],
}

#[repr(C)] pub struct list_head { pub next: *mut list_head, pub prev: *mut list_head }
#[repr(C)] pub struct cgroup_subsys_state { pub parent: *mut cgroup_subsys_state }
#[repr(C)] pub struct rdma_cgroup { pub css: cgroup_subsys_state, pub rpools: list_head, pub events_file: cftype, pub events_local_file: cftype }
#[repr(C)] pub struct rdmacg_device { pub dev_node: list_head, pub rpools: list_head, pub name: *const c_char, pub index: u32 }
#[repr(C)] pub struct mutex { _private: [u8; 0] }
#[repr(C)] pub struct cftype { pub name: *const c_char, pub write: Option<unsafe extern "C" fn(*mut kernfs_open_file,*mut c_char,usize,i64)->isize>, pub seq_show: Option<unsafe extern "C" fn(*mut seq_file,*mut c_void)->c_int>, pub private: usize, pub flags: u32, pub file_offset: usize }
#[repr(C)] pub struct seq_file { _private: [u8; 0] }
#[repr(C)] pub struct kernfs_open_file { _private: [u8; 0] }

extern "C" {
    static mut rdmacg_mutex: mutex;
    static mut rdmacg_devices: list_head;
    static mut rdma_cgrp_id: c_int;
    static mut current: *mut c_void;
    fn task_get_css(task: *mut c_void, id: c_int) -> *mut cgroup_subsys_state;
    fn css_put(css: *mut cgroup_subsys_state);
    fn mutex_lock(m: *mut mutex); fn mutex_unlock(m: *mut mutex);
    fn kfree(p: *mut c_void); fn kzalloc(size: usize, flags: usize) -> *mut c_void;
    fn cgroup_file_notify(f: *mut cftype);
    fn of_css(of: *mut kernfs_open_file) -> *mut cgroup_subsys_state;
    fn seq_css(sf: *mut seq_file) -> *mut cgroup_subsys_state;
    fn strcmp(a: *const c_char,b: *const c_char)->c_int;
    fn seq_puts(sf:*mut seq_file,s:*const c_char); fn seq_putc(sf:*mut seq_file,c:c_int);
    fn seq_printf(sf:*mut seq_file,fmt:*const c_char,...);
}

pub const RDMACG_RESOURCE_HCA_HANDLE: u32 = 0;
pub const RDMACG_RESOURCE_HCA_OBJECT: u32 = 1;
pub const RDMACG_RESOURCE_MAX: u32 = 2;
pub const S32_MAX: c_int = 0x7fffffff;

static mut rdmacg_resource_names: [*const c_char; 2] = [b"hca_handle\0".as_ptr() as _, b"hca_object\0".as_ptr() as _];

unsafe fn css_rdmacg(css: *mut cgroup_subsys_state) -> *mut rdma_cgroup {
    css as *mut rdma_cgroup
}
unsafe fn parent_rdmacg(cg: *mut rdma_cgroup) -> *mut rdma_cgroup { if cg.is_null() { core::ptr::null_mut() } else { css_rdmacg((*cg).css.parent) } }
unsafe fn get_current_rdmacg() -> *mut rdma_cgroup { css_rdmacg(task_get_css(current, rdma_cgrp_id)) }

unsafe fn set_resource_limit(rpool:*mut rdmacg_resource_pool,index:usize,new_max:c_int) {
    if new_max == S32_MAX { if (*rpool).resources[index].max != S32_MAX { (*rpool).num_max_cnt += 1; } }
    else if (*rpool).resources[index].max == S32_MAX { (*rpool).num_max_cnt -= 1; }
    (*rpool).resources[index].max = new_max;
}
unsafe fn set_all_resource_max_limit(rpool:*mut rdmacg_resource_pool) { for i in 0..RDMACG_RESOURCE_MAX as usize { set_resource_limit(rpool,i,S32_MAX); } }
unsafe fn free_cg_rpool_locked(rpool:*mut rdmacg_resource_pool) { kfree(rpool.cast()); }
unsafe fn rpool_has_persistent_state(rpool:*mut rdmacg_resource_pool)->bool { for i in 0..RDMACG_RESOURCE_MAX as usize { let r=(*rpool).resources[i]; if r.peak != 0 || (*rpool).events_max[i] != 0 || (*rpool).events_local_max[i] != 0 || (*rpool).events_alloc_fail[i] != 0 || (*rpool).events_local_alloc_fail[i] != 0 { return true; } } false }

unsafe fn find_cg_rpool_locked(_cg:*mut rdma_cgroup,_device:*mut rdmacg_device)->*mut rdmacg_resource_pool { core::ptr::null_mut() }
unsafe fn get_cg_rpool_locked(cg:*mut rdma_cgroup,device:*mut rdmacg_device)->*mut rdmacg_resource_pool {
    let r=find_cg_rpool_locked(cg,device); if !r.is_null() { return r; }
    let r=kzalloc(core::mem::size_of::<rdmacg_resource_pool>(),0) as *mut rdmacg_resource_pool; if r.is_null(){return (-12isize) as *mut _;}
    (*r).device=device; set_all_resource_max_limit(r); r
}

unsafe fn uncharge_cg_locked(cg:*mut rdma_cgroup,device:*mut rdmacg_device,index:usize) { let r=find_cg_rpool_locked(cg,device); if r.is_null(){return;} (*r).resources[index].usage-=1; (*r).usage_sum-=1; if (*r).usage_sum==0 && (*r).num_max_cnt==RDMACG_RESOURCE_MAX as c_int && !rpool_has_persistent_state(r){free_cg_rpool_locked(r);} }
unsafe fn rdmacg_event_locked(cg:*mut rdma_cgroup,over_cg:*mut rdma_cgroup,device:*mut rdmacg_device,index:usize) { let r=find_cg_rpool_locked(cg,device); if !r.is_null() { (*r).events_local_alloc_fail[index]+=1; cgroup_file_notify(&mut (*cg).events_local_file); } let r=find_cg_rpool_locked(over_cg,device); if !r.is_null() { (*r).events_local_max[index]+=1; cgroup_file_notify(&mut (*over_cg).events_local_file); } }

unsafe fn rdmacg_uncharge_hierarchy(cg:*mut rdma_cgroup,device:*mut rdmacg_device,stop:*mut rdma_cgroup,index:usize){mutex_lock(&mut rdmacg_mutex); let mut p=cg; while p!=stop {uncharge_cg_locked(p,device,index); p=parent_rdmacg(p);} mutex_unlock(&mut rdmacg_mutex); css_put(&mut (*cg).css);}
#[no_mangle] pub unsafe extern "C" fn rdmacg_uncharge(cg:*mut rdma_cgroup,device:*mut rdmacg_device,index:u32){if index<RDMACG_RESOURCE_MAX{rdmacg_uncharge_hierarchy(cg,device,core::ptr::null_mut(),index as usize);}}

#[no_mangle] pub unsafe extern "C" fn rdmacg_try_charge(out:*mut *mut rdma_cgroup,device:*mut rdmacg_device,index:u32)->c_int { if index>=RDMACG_RESOURCE_MAX{return -22;} let cg=get_current_rdmacg(); mutex_lock(&mut rdmacg_mutex); let mut p=cg; while !p.is_null(){let r=get_cg_rpool_locked(p,device); if (r as isize)==-12{mutex_unlock(&mut rdmacg_mutex);return -12;} let n=(*r).resources[index as usize].usage+1; if n>(*r).resources[index as usize].max{mutex_unlock(&mut rdmacg_mutex);rdmacg_uncharge_hierarchy(cg,device,p,index as usize);return -11;} (*r).resources[index as usize].usage=n;(*r).usage_sum+=1;p=parent_rdmacg(p);} mutex_unlock(&mut rdmacg_mutex);*out=cg;0 }
#[no_mangle] pub unsafe extern "C" fn rdmacg_register_device(device:*mut rdmacg_device){mutex_lock(&mut rdmacg_mutex); let _=device; mutex_unlock(&mut rdmacg_mutex);}
#[no_mangle] pub unsafe extern "C" fn rdmacg_unregister_device(_device:*mut rdmacg_device){mutex_lock(&mut rdmacg_mutex);mutex_unlock(&mut rdmacg_mutex);}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
