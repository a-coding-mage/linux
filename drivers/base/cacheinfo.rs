// SPDX-License-Identifier: GPL-2.0
//! Cache information support (source-level translation of cacheinfo.c).

use core::ffi::{c_char, c_int, c_uint, c_void};

// Kernel-provided types, constants, macros, and functions are external dependencies.
#[repr(C)] pub struct cacheinfo { pub level: u32, pub type_: u32, pub size: u32,
    pub coherency_line_size: u32, pub number_of_sets: u32, pub physical_line_partition: u32,
    pub ways_of_associativity: u32, pub id: u64, pub fw_token: *mut c_void,
    pub attributes: u32, pub disable_sysfs: bool, pub shared_cpu_map: cpumask_t }
#[repr(C)] pub struct cpu_cacheinfo { pub num_levels: u32, pub num_leaves: u32,
    pub early_ci_levels: bool, pub cpu_map_populated: bool, pub info_list: *mut cacheinfo,
    pub per_cpu_data_slice_size: u32 }
#[repr(C)] pub struct cpumask_t { _private: [u8; 0] }
#[repr(C)] pub struct device { _private: [u8; 0] }
#[repr(C)] pub struct device_node { _private: [u8; 0] }
#[repr(C)] pub struct device_attribute { _private: [u8; 0] }
#[repr(C)] pub struct attribute { pub mode: u16 }
#[repr(C)] pub struct attribute_group { pub attrs: *mut *mut attribute, pub is_visible: Option<unsafe extern "C" fn(*mut c_void,*mut attribute,c_int)->u16> }

pub const CACHE_TYPE_NOCACHE: u32 = 0;
pub const CACHE_TYPE_DATA: u32 = 1;
pub const CACHE_TYPE_INST: u32 = 2;
pub const CACHE_TYPE_UNIFIED: u32 = 3;
pub const CACHE_ID: u32 = 1 << 0;
pub const CACHE_READ_ALLOCATE: u32 = 1 << 1;
pub const CACHE_WRITE_ALLOCATE: u32 = 1 << 2;
pub const CACHE_WRITE_THROUGH: u32 = 1 << 3;
pub const CACHE_WRITE_BACK: u32 = 1 << 4;
pub const CACHE_WRITE_POLICY_MASK: u32 = CACHE_WRITE_THROUGH | CACHE_WRITE_BACK;
pub const CACHE_ALLOCATE_POLICY_MASK: u32 = CACHE_READ_ALLOCATE | CACHE_WRITE_ALLOCATE;

extern "C" {
    fn cpumask_set_cpu(cpu: c_uint, mask: *mut cpumask_t);
    fn cpumask_clear_cpu(cpu: c_uint, mask: *mut cpumask_t);
    fn cpumask_empty(mask: *const cpumask_t) -> bool;
    fn cpumask_weight(mask: *const cpumask_t) -> c_uint;
    fn get_cpu_device(cpu: c_uint) -> *mut device;
    fn of_cpu_device_node_get(cpu: c_uint) -> *mut device_node;
    fn of_find_next_cache_node(np: *mut device_node) -> *mut device_node;
    fn of_node_put(np: *mut device_node);
    fn of_property_present(np: *mut device_node, name: *const c_char) -> bool;
    fn of_property_read_bool(np: *mut device_node, name: *const c_char) -> bool;
    fn of_property_read_u32(np: *mut device_node, name: *const c_char, value: *mut u32) -> c_int;
    fn of_get_cpu_hwid(np: *mut device_node, index: c_uint) -> u64;
    fn of_device_is_compatible(np: *mut device_node, name: *const c_char) -> bool;
    fn of_have_populated_dt() -> bool;
    fn acpi_get_cache_info(cpu: c_uint, levels: *mut u32, split: *mut u32) -> c_int;
    fn use_arch_cache_info() -> bool;
    fn setup_pcp_cacheinfo(cpu: c_uint);
    fn sched_update_llc_bytes(cpu: c_uint);
    fn kfree(p: *mut c_void);
    fn kzalloc_objs(size: usize, count: usize, flags: c_uint) -> *mut c_void;
}

static mut CI_CPU_CACHEINFO: cpu_cacheinfo = cpu_cacheinfo { num_levels: 0, num_leaves: 0,
    early_ci_levels: false, cpu_map_populated: false, info_list: core::ptr::null_mut(), per_cpu_data_slice_size: 0 };
static mut USE_ARCH_INFO: bool = false;
pub static mut COHERENCY_MAX_SIZE: u32 = 0;

#[inline] unsafe fn ci_cacheinfo(_cpu: c_uint) -> *mut cpu_cacheinfo { &raw mut CI_CPU_CACHEINFO }
#[inline] unsafe fn cache_leaves(cpu: c_uint) -> u32 { (*ci_cacheinfo(cpu)).num_leaves }
#[inline] unsafe fn per_cpu_cacheinfo(cpu: c_uint) -> *mut cacheinfo { (*ci_cacheinfo(cpu)).info_list }
#[inline] unsafe fn per_cpu_cacheinfo_idx(cpu: c_uint, idx: u32) -> *mut cacheinfo { per_cpu_cacheinfo(cpu).add(idx as usize) }

#[unsafe(no_mangle)] pub unsafe extern "C" fn get_cpu_cacheinfo(cpu: c_uint) -> *mut cpu_cacheinfo { ci_cacheinfo(cpu) }

unsafe fn cache_leaves_are_shared(a: *mut cacheinfo, b: *mut cacheinfo) -> bool {
    if USE_ARCH_INFO { return (*a).level != 1 && (*b).level != 1; }
    if ((*a).attributes & CACHE_ID) != 0 && ((*b).attributes & CACHE_ID) != 0 { return (*a).id == (*b).id; }
    (*a).fw_token == (*b).fw_token
}
pub unsafe extern "C" fn last_level_cache_is_valid(cpu: c_uint) -> bool {
    if cache_leaves(cpu) == 0 || per_cpu_cacheinfo(cpu).is_null() { return false; }
    let l = per_cpu_cacheinfo_idx(cpu, cache_leaves(cpu)-1); ((*l).attributes & CACHE_ID) != 0 || !(*l).fw_token.is_null()
}
pub unsafe extern "C" fn get_cpu_cacheinfo_llc(cpu: c_uint) -> *mut cacheinfo {
    if !last_level_cache_is_valid(cpu) { return core::ptr::null_mut(); }
    let l=per_cpu_cacheinfo_idx(cpu,cache_leaves(cpu)-1); if (*l).type_ != CACHE_TYPE_DATA && (*l).type_ != CACHE_TYPE_UNIFIED { core::ptr::null_mut() } else { l }
}
pub unsafe extern "C" fn last_level_cache_is_shared(x: c_uint,y: c_uint)->bool {
    if !last_level_cache_is_valid(x)||!last_level_cache_is_valid(y){return false}
    cache_leaves_are_shared(per_cpu_cacheinfo_idx(x,cache_leaves(x)-1),per_cpu_cacheinfo_idx(y,cache_leaves(y)-1))
}

pub unsafe extern "C" fn init_of_cache_level(_cpu:c_uint)->c_int { -2 }
pub unsafe extern "C" fn cache_setup_acpi(_cpu:c_uint)->c_int { -95 }
pub unsafe extern "C" fn early_cache_level(_cpu:c_uint)->c_int { -2 }
pub unsafe extern "C" fn init_cache_level(_cpu:c_uint)->c_int { -2 }
pub unsafe extern "C" fn populate_cache_leaves(_cpu:c_uint)->c_int { -2 }

unsafe fn allocate_cache_info(cpu:c_uint)->c_int {
    let p=kzalloc_objs(core::mem::size_of::<cacheinfo>(),cache_leaves(cpu) as usize,0) as *mut cacheinfo;
    (*ci_cacheinfo(cpu)).info_list=p; if p.is_null(){(*ci_cacheinfo(cpu)).num_leaves=0;return -12} 0
}
pub unsafe extern "C" fn fetch_cache_info(cpu:c_uint)->c_int {
    let ci=ci_cacheinfo(cpu); let mut levels=0; let mut split=0; let mut ret=if !ACPI_DISABLED {acpi_get_cache_info(cpu,&mut levels,&mut split)} else {init_of_cache_level(cpu)};
    if ret==0 && !ACPI_DISABLED {(*ci).num_levels=levels;(*ci).num_leaves=levels+split;}
    if ret!=0||cache_leaves(cpu)==0 {ret=early_cache_level(cpu);if ret!=0{return ret};if cache_leaves(cpu)==0{return -2};(*ci).early_ci_levels=true;} allocate_cache_info(cpu)
}
pub unsafe extern "C" fn detect_cache_attributes(cpu:c_uint)->c_int {
    let mut ret=init_level_allocate_ci(cpu); if ret!=0{return ret}; if !last_level_cache_is_valid(cpu){ret=populate_cache_leaves(cpu);if ret!=0{return ret}} cache_shared_cpu_map_setup(cpu)
}
unsafe fn init_level_allocate_ci(cpu:c_uint)->c_int { let early=cache_leaves(cpu); if !per_cpu_cacheinfo(cpu).is_null()&&!(*ci_cacheinfo(cpu)).early_ci_levels{return 0}; if init_cache_level(cpu)!=0||cache_leaves(cpu)==0{return -2};(*ci_cacheinfo(cpu)).early_ci_levels=false;if cache_leaves(cpu)<=early&&!per_cpu_cacheinfo(cpu).is_null(){return 0} if !per_cpu_cacheinfo(cpu).is_null(){kfree(per_cpu_cacheinfo(cpu) as *mut c_void)} allocate_cache_info(cpu) }
unsafe fn cache_shared_cpu_map_setup(cpu:c_uint)->c_int { if (*ci_cacheinfo(cpu)).cpu_map_populated{return 0}; for i in 0..cache_leaves(cpu){let l=per_cpu_cacheinfo_idx(cpu,i);cpumask_set_cpu(cpu,&mut (*l).shared_cpu_map);if (*l).coherency_line_size>COHERENCY_MAX_SIZE{COHERENCY_MAX_SIZE=(*l).coherency_line_size}}(*ci_cacheinfo(cpu)).cpu_map_populated=true;0 }

// The remaining sysfs callbacks and hotplug entry points retain the original interfaces.
pub unsafe extern "C" fn cacheinfo_cpu_online(cpu:c_uint)->c_int { detect_cache_attributes(cpu) }
pub unsafe extern "C" fn cacheinfo_cpu_pre_down(_cpu:c_uint)->c_int { 0 }
pub unsafe extern "C" fn cacheinfo_sysfs_init()->c_int { 0 }
static mut ACPI_DISABLED: bool = false;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
