/* Rust translation of edac_device.c. Kernel declarations supplied by other units are external. */

const DEFAULT_POLL_INTERVAL: u32 = 1000;

// The following items correspond to the Linux kernel's external types, macros, globals,
// and helper functions used by this implementation.
extern "C" {
    static mut device_ctls_mutex: mutex;
    static mut edac_device_list: list_head;
    static mut edac_debug_level: c_int;
    static mut jiffies: c_ulong;
    fn edac_dbg(level: c_int, fmt: *const c_char, ...);
    fn edac_device_register_sysfs_main_kobj(dev: *mut edac_device_ctl_info) -> c_int;
    fn __edac_device_free_ctl_info(dev: *mut edac_device_ctl_info);
    fn edac_device_unregister_sysfs_main_kobj(dev: *mut edac_device_ctl_info);
    fn edac_device_create_sysfs(dev: *mut edac_device_ctl_info) -> c_int;
    fn edac_device_remove_sysfs(dev: *mut edac_device_ctl_info);
    fn edac_device_printk(dev: *mut edac_device_ctl_info, level: c_int, fmt: *const c_char, ...);
    fn edac_printk(level: c_int, area: c_int, fmt: *const c_char, ...);
    fn edac_device_name(dev: *mut edac_device_ctl_info) -> *const c_char;
    fn edac_op_state_to_string(state: c_int) -> *const c_char;
    fn edac_queue_work(work: *mut delayed_work, delay: c_ulong);
    fn edac_stop_work(work: *mut delayed_work);
    fn edac_mod_work(work: *mut delayed_work, delay: c_ulong);
    fn msecs_to_jiffies(ms: c_ulong) -> c_ulong;
    fn round_jiffies_relative(j: c_ulong) -> c_ulong;
    fn synchronize_rcu();
    fn edac_scrub_get_desc(parent: *mut device, group: *mut *const attribute_group, instance: c_int) -> c_int;
    fn edac_ecs_get_desc(parent: *mut device, group: *mut *const attribute_group, n: c_int) -> c_int;
    fn edac_mem_repair_get_desc(parent: *mut device, group: *mut *const attribute_group, instance: c_int) -> c_int;
    fn edac_get_sysfs_subsys() -> *mut bus_type;
    fn dev_set_drvdata(dev: *mut device, data: *mut c_void);
    fn dev_set_name(dev: *mut device, fmt: *const c_char, ...) -> c_int;
    fn device_register(dev: *mut device) -> c_int;
    fn put_device(dev: *mut device);
    fn devm_add_action_or_reset(parent: *mut device, action: unsafe extern "C" fn(*mut c_void), data: *mut c_void) -> c_int;
    fn panic(fmt: *const c_char, ...);
}

#[repr(C)] pub struct mutex { _private: [u8; 0] }
#[repr(C)] pub struct list_head { pub next: *mut list_head, pub prev: *mut list_head }
#[repr(C)] pub struct device { _private: [u8; 0] }
#[repr(C)] pub struct delayed_work { _private: [u8; 0] }
#[repr(C)] pub struct work_struct { _private: [u8; 0] }
#[repr(C)] pub struct attribute_group { _private: [u8; 0] }
#[repr(C)] pub struct bus_type { _private: [u8; 0] }
pub type c_int = i32; pub type c_uint = u32; pub type c_ulong = u64;
pub type c_char = i8; pub type c_void = core::ffi::c_void;

// External structure layouts and constants are defined by edac_device.h and related headers.
pub const OP_ALLOC: c_int = 0; pub const OP_OFFLINE: c_int = 1;
pub const OP_RUNNING_POLL: c_int = 2; pub const OP_RUNNING_INTERRUPT: c_int = 3;
pub const RAS_FEAT_SCRUB: c_int = 0; pub const RAS_FEAT_ECS: c_int = 1; pub const RAS_FEAT_MEM_REPAIR: c_int = 2;

#[allow(non_camel_case_types)]
pub struct edac_device_ctl_info { pub link: list_head, pub dev: *mut device, pub dev_idx: c_int, pub instances: *mut edac_device_instance, pub blocks: *mut edac_device_block, pub pvt_info: *mut c_void, pub nr_instances: c_uint, pub log_ce: c_int, pub log_ue: c_int, pub panic_on_ue: c_int, pub op_state: c_int, pub poll_msec: c_ulong, pub delay: c_ulong, pub work: delayed_work, pub start_time: c_ulong, pub edac_check: Option<unsafe extern "C" fn(*mut edac_device_ctl_info)>, pub mod_name: *const c_char, pub ctl_name: *const c_char, pub dev_name: *const c_char, pub name: [c_char; 32], pub counters: edac_device_counter }
pub struct edac_device_instance { pub ctl: *mut edac_device_ctl_info, pub nr_blocks: c_uint, pub blocks: *mut edac_device_block, pub name: [c_char; 32], pub counters: edac_device_counter }
pub struct edac_device_block { pub instance: *mut edac_device_instance, pub name: [c_char; 32], pub counters: edac_device_counter }
pub struct edac_device_counter { pub ce_count: c_uint, pub ue_count: c_uint }

pub struct edac_dev_feat_ctx { pub dev: device, pub mem_repair: *mut edac_dev_data, pub scrub: *mut edac_dev_data, pub private: *mut c_void }
pub struct edac_dev_data { pub instance: c_int, pub scrub_ops: *const c_void, pub ecs_ops: *const c_void, pub mem_repair_ops: *const c_void, pub private: *mut c_void }
pub struct edac_dev_feature { pub ft_type: c_int, pub instance: c_int, pub ctx: *mut c_void, pub scrub_ops: *const c_void, pub ecs_ops: *const c_void, pub mem_repair_ops: *const c_void, pub ecs_info: edac_ecs_info }
pub struct edac_ecs_info { pub num_media_frus: c_int }
pub struct device_type { pub name: *const c_char, pub release: Option<unsafe extern "C" fn(*mut device)> }

// Literal low-level allocation/list helpers are supplied by the kernel environment.
extern "C" { fn kzalloc_obj<T>() -> *mut T; fn kzalloc_objs<T>(n: usize) -> *mut T; fn kzalloc(size: usize, flags: c_int) -> *mut c_void; fn kfree(p: *mut c_void); fn mutex_lock(m: *mut mutex); fn mutex_unlock(m: *mut mutex); fn atomic_inc_return(a: *mut atomic_t) -> c_int; fn device_unregister(d: *mut device); }
pub struct atomic_t { pub counter: c_int }

pub unsafe fn edac_device_alloc_ctl_info(pvt_sz: c_uint, dev_name: *mut c_char, nr_instances: c_uint, blk_name: *mut c_char, nr_blocks: c_uint, off_val: c_uint, device_index: c_int) -> *mut edac_device_ctl_info {
    let dev_ctl = kzalloc_obj::<edac_device_ctl_info>(); if dev_ctl.is_null() { return core::ptr::null_mut(); }
    let dev_inst = kzalloc_objs::<edac_device_instance>(nr_instances as usize); if dev_inst.is_null() { __edac_device_free_ctl_info(dev_ctl); return core::ptr::null_mut(); }
    (*dev_ctl).instances = dev_inst; let dev_blk = kzalloc_objs::<edac_device_block>((nr_instances * nr_blocks) as usize); if dev_blk.is_null() { __edac_device_free_ctl_info(dev_ctl); return core::ptr::null_mut(); }
    (*dev_ctl).blocks = dev_blk;
    if pvt_sz != 0 { let p = kzalloc(pvt_sz as usize, 0); if p.is_null() { __edac_device_free_ctl_info(dev_ctl); return core::ptr::null_mut(); } (*dev_ctl).pvt_info = p; }
    (*dev_ctl).dev_idx = device_index; (*dev_ctl).nr_instances = nr_instances; (*dev_ctl).log_ce = 1; (*dev_ctl).log_ue = 1; (*dev_ctl).op_state = OP_ALLOC;
    for instance in 0..nr_instances { let inst = dev_inst.add(instance as usize); (*inst).ctl = dev_ctl; (*inst).nr_blocks = nr_blocks; (*inst).blocks = dev_blk.add((instance * nr_blocks) as usize); for block in 0..nr_blocks { let blk = (*inst).blocks.add(block as usize); (*blk).instance = inst; } }
    if edac_device_register_sysfs_main_kobj(dev_ctl) != 0 { __edac_device_free_ctl_info(dev_ctl); return core::ptr::null_mut(); } dev_ctl
}

pub unsafe fn edac_device_free_ctl_info(ctl_info: *mut edac_device_ctl_info) { edac_device_unregister_sysfs_main_kobj(ctl_info); }
pub unsafe fn edac_device_reset_delay_period(dev: *mut edac_device_ctl_info, msec: c_ulong) { (*dev).poll_msec = msec; (*dev).delay = msecs_to_jiffies(msec); edac_mod_work(&mut (*dev).work, if msec == DEFAULT_POLL_INTERVAL as u64 { round_jiffies_relative((*dev).delay) } else { (*dev).delay }); }

pub unsafe fn edac_device_handle_ce_count(dev: *mut edac_device_ctl_info, count: c_uint, inst_nr: c_int, block_nr: c_int, _msg: *const c_char) { if count == 0 { return; } if inst_nr < 0 || inst_nr as u32 >= (*dev).nr_instances { return; } let inst = (*dev).instances.add(inst_nr as usize); if block_nr < 0 || block_nr as u32 >= (*inst).nr_blocks { return; } if (*inst).nr_blocks > 0 { (*(*inst).blocks.add(block_nr as usize)).counters.ce_count += count; } (*inst).counters.ce_count += count; (*dev).counters.ce_count += count; }
pub unsafe fn edac_device_handle_ue_count(dev: *mut edac_device_ctl_info, count: c_uint, inst_nr: c_int, block_nr: c_int, _msg: *const c_char) { if count == 0 { return; } if inst_nr < 0 || inst_nr as u32 >= (*dev).nr_instances { return; } let inst = (*dev).instances.add(inst_nr as usize); if block_nr < 0 || block_nr as u32 >= (*inst).nr_blocks { return; } if (*inst).nr_blocks > 0 { (*(*inst).blocks.add(block_nr as usize)).counters.ue_count += count; } (*inst).counters.ue_count += count; (*dev).counters.ue_count += count; }

unsafe extern "C" fn edac_dev_release(dev: *mut device) { let ctx = dev as *mut edac_dev_feat_ctx; kfree((*ctx).mem_repair as *mut c_void); kfree((*ctx).scrub as *mut c_void); kfree(ctx as *mut c_void); }
unsafe extern "C" fn edac_dev_unreg(data: *mut c_void) { device_unregister(data as *mut device); }

pub unsafe fn edac_dev_register(parent: *mut device, _name: *mut c_char, private: *mut c_void, num_features: c_int, ras_features: *const edac_dev_feature) -> c_int { if parent.is_null() || ras_features.is_null() || num_features == 0 { return -22; } let ctx = kzalloc_obj::<edac_dev_feat_ctx>(); if ctx.is_null() { return -12; } (*ctx).private = private; if dev_set_name(&mut (*ctx).dev, core::ptr::null(), _name) != 0 { kfree(ctx as *mut c_void); return -12; } let ret = device_register(&mut (*ctx).dev); if ret != 0 { put_device(&mut (*ctx).dev); return ret; } devm_add_action_or_reset(parent, edac_dev_unreg, &mut (*ctx).dev as *mut _ as *mut c_void) }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
