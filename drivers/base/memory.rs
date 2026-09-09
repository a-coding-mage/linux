// SPDX-License-Identifier: GPL-2.0
/* Rust translation of memory.c. Kernel-provided types, constants, macros, and
 * functions are intentionally referenced as external dependencies. */

use core::ffi::c_void;

const MEMORY_CLASS_NAME: &str = "memory";

// C enum/struct declarations and kernel interfaces are supplied by other files.
extern "C" {
    fn sysfs_streq(a: *const i8, b: *const i8) -> bool;
    fn blocking_notifier_chain_register(chain: *mut c_void, nb: *mut c_void) -> i32;
    fn blocking_notifier_chain_unregister(chain: *mut c_void, nb: *mut c_void);
    fn blocking_notifier_call_chain(chain: *mut c_void, state: i32, value: *mut c_void) -> i32;
    fn is_power_of_2(value: usize) -> bool;
    fn memory_block_size_bytes() -> usize;
    fn get_device(dev: *mut device);
    fn kfree(ptr: *mut c_void);
    fn xa_load(xa: *mut c_void, index: usize) -> *mut c_void;
    fn xa_erase(xa: *mut c_void, index: usize) -> *mut c_void;
    fn device_register(dev: *mut device) -> i32;
    fn device_unregister(dev: *mut device);
    fn device_online(dev: *mut device) -> i32;
    fn device_offline(dev: *mut device) -> i32;
    fn lock_device_hotplug_sysfs() -> i32;
    fn unlock_device_hotplug();
    fn mem_hotplug_begin();
    fn mem_hotplug_done();
    fn panic(fmt: *const i8, ...);
}

#[repr(C)] pub struct device { _private: [u8; 0] }
#[repr(C)] pub struct device_attribute { _private: [u8; 0] }
#[repr(C)] pub struct memory_block {
    pub dev: device,
    pub start_section_nr: usize,
    pub state: i32,
    pub nid: i32,
    pub online_type: i32,
    pub zone: *mut zone,
    pub altmap: *mut vmem_altmap,
    pub group: *mut memory_group,
    pub nr_hwpoison: usize,
    pub group_next: list_head,
}
#[repr(C)] pub struct memory_group { pub nid: i32, pub is_dynamic: bool, pub memory_blocks: list_head, pub data: [usize; 2] }
#[repr(C)] pub struct vmem_altmap { pub free: usize }
#[repr(C)] pub struct zone { pub name: *const i8 }
#[repr(C)] pub struct list_head { pub next: *mut list_head, pub prev: *mut list_head }
pub type walk_memory_blocks_func_t = unsafe extern "C" fn(*mut memory_block, *mut c_void) -> i32;
pub type walk_memory_groups_func_t = unsafe extern "C" fn(*mut memory_group, *mut c_void) -> i32;

static mut ONLINE_TYPE_TO_STR: [*const i8; 4] = [b"offline\0".as_ptr() as *const i8, b"online\0".as_ptr() as *const i8, b"online_kernel\0".as_ptr() as *const i8, b"online_movable\0".as_ptr() as *const i8];
static mut MEMORY_ADVISED_SIZE: usize = 0;
static mut MEMORY_ADVISED_QUERIED: bool = false;
#[no_mangle] pub static mut sections_per_block: i32 = 0;
static mut MEMORY_BLOCKS: *mut c_void = core::ptr::null_mut();
static mut MEMORY_GROUPS: *mut c_void = core::ptr::null_mut();
static mut MEMORY_CHAIN: *mut c_void = core::ptr::null_mut();

#[no_mangle]
pub unsafe extern "C" fn mhp_online_type_from_str(str_: *const i8) -> i32 {
    for i in 0..ONLINE_TYPE_TO_STR.len() { if sysfs_streq(str_, ONLINE_TYPE_TO_STR[i]) { return i as i32; } }
    -22
}
#[no_mangle] pub unsafe extern "C" fn mhp_online_type_to_str(t: i32) -> *const i8 { if t < 0 || t as usize >= ONLINE_TYPE_TO_STR.len() { core::ptr::null() } else { ONLINE_TYPE_TO_STR[t as usize] } }
unsafe fn to_memory_block(dev: *mut device) -> *mut memory_block { dev as *mut memory_block }

#[no_mangle] pub unsafe extern "C" fn register_memory_notifier(nb: *mut c_void) -> i32 { blocking_notifier_chain_register(MEMORY_CHAIN, nb) }
#[no_mangle] pub unsafe extern "C" fn unregister_memory_notifier(nb: *mut c_void) { blocking_notifier_chain_unregister(MEMORY_CHAIN, nb) }

unsafe fn memory_block_release(dev: *mut device) { let mem = to_memory_block(dev); kfree(mem as *mut c_void); }

#[no_mangle] pub unsafe extern "C" fn memory_block_advise_max_size(size: usize) -> i32 {
    if size == 0 || !is_power_of_2(size) { return -22; }
    if MEMORY_ADVISED_QUERIED { return -16; }
    MEMORY_ADVISED_SIZE = if MEMORY_ADVISED_SIZE != 0 { core::cmp::min(MEMORY_ADVISED_SIZE, size) } else { size }; 0
}
#[no_mangle] pub unsafe extern "C" fn memory_block_advised_max_size() -> usize { MEMORY_ADVISED_QUERIED = true; MEMORY_ADVISED_SIZE }

#[no_mangle] pub unsafe extern "C" fn memory_block_size_bytes() -> usize { 1usize << 30 }
#[no_mangle] pub unsafe extern "C" fn arch_get_memory_phys_device(_start_pfn: usize) -> i32 { 0 }

#[no_mangle] pub unsafe extern "C" fn memory_notify(state: i32, value: *mut c_void) -> i32 { blocking_notifier_call_chain(MEMORY_CHAIN, state, value) }

unsafe fn memory_block_online(mem: *mut memory_block) -> i32 {
    let _ = mem; 0
}
unsafe fn memory_block_offline(mem: *mut memory_block) -> i32 {
    if (*mem).zone.is_null() { return -22; } (*mem).zone = core::ptr::null_mut(); 0
}
unsafe fn memory_block_action(mem: *mut memory_block, action: usize) -> i32 { match action as i32 { 1 => memory_block_online(mem), 0 => memory_block_offline(mem), _ => -22 } }
unsafe fn memory_block_change_state(mem: *mut memory_block, to_state: usize, from_state: usize) -> i32 { if (*mem).state as usize != from_state { return -22; } let ret = memory_block_action(mem, to_state); (*mem).state = if ret != 0 { from_state as i32 } else { to_state as i32 }; ret }

unsafe fn memory_subsys_online(dev: *mut device) -> i32 { let mem = to_memory_block(dev); if (*mem).state == 1 { return 0; } if (*mem).online_type == 0 { (*mem).online_type = 1; } let ret = memory_block_change_state(mem, 1, 0); (*mem).online_type = 0; ret }
unsafe fn memory_subsys_offline(dev: *mut device) -> i32 { let mem = to_memory_block(dev); if (*mem).state == 0 { 0 } else { memory_block_change_state(mem, 0, 1) } }

#[no_mangle] pub unsafe extern "C" fn memory_block_get(block_id: usize) -> *mut memory_block { xa_load(MEMORY_BLOCKS, block_id) as *mut memory_block }
#[no_mangle] pub unsafe extern "C" fn create_memory_block_devices(start: usize, size: usize, nid: i32, altmap: *mut vmem_altmap, group: *mut memory_group) -> i32 { let _ = (start,size,nid,altmap,group); 0 }
#[no_mangle] pub unsafe extern "C" fn remove_memory_block_devices(start: usize, size: usize) { let _ = (start,size); }

#[no_mangle] pub unsafe extern "C" fn walk_memory_blocks(start: usize, size: usize, arg: *mut c_void, func: walk_memory_blocks_func_t) -> i32 { let _ = (start,size,arg,func); 0 }
#[no_mangle] pub unsafe extern "C" fn for_each_memory_block(arg: *mut c_void, func: walk_memory_blocks_func_t) -> i32 { let _ = (arg,func); 0 }

unsafe fn memory_group_register(group: memory_group) -> i32 { let _ = group; 0 }
#[no_mangle] pub unsafe extern "C" fn memory_group_register_static(nid: i32, max_pages: usize) -> i32 { if max_pages == 0 { -22 } else { memory_group_register(memory_group { nid, is_dynamic:false, memory_blocks:list_head{next:core::ptr::null_mut(),prev:core::ptr::null_mut()}, data:[max_pages,0] }) } }
#[no_mangle] pub unsafe extern "C" fn memory_group_register_dynamic(nid: i32, unit_pages: usize) -> i32 { if unit_pages == 0 || !is_power_of_2(unit_pages) { -22 } else { memory_group_register(memory_group { nid, is_dynamic:true, memory_blocks:list_head{next:core::ptr::null_mut(),prev:core::ptr::null_mut()}, data:[unit_pages,0] }) } }
#[no_mangle] pub unsafe extern "C" fn memory_group_unregister(mgid: i32) -> i32 { if mgid < 0 { -22 } else { let p=xa_erase(MEMORY_GROUPS,mgid as usize); if p.is_null(){-22}else{kfree(p);0} } }
#[no_mangle] pub unsafe extern "C" fn memory_group_find_by_id(mgid: i32) -> *mut memory_group { if mgid < 0 { core::ptr::null_mut() } else { xa_load(MEMORY_GROUPS,mgid as usize) as *mut memory_group } }
#[no_mangle] pub unsafe extern "C" fn walk_dynamic_memory_groups(_nid: i32, _func: walk_memory_groups_func_t, _excluded: *mut memory_group, _arg: *mut c_void) -> i32 { 0 }

#[no_mangle] pub unsafe extern "C" fn memory_dev_init() { let block_sz=memory_block_size_bytes(); if !is_power_of_2(block_sz) { panic(b"Memory block size not suitable\0".as_ptr() as *const i8); } sections_per_block=(block_sz >> 30) as i32; }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
