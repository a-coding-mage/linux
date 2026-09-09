/* SPDX-License-Identifier: GPL-2.0-or-later */
/*
 * Copyright (c) 2009-2013, NVIDIA Corporation. All rights reserved.
 */

/* C dependencies: linux/device.h, dma-direction.h, dma-fence.h, spinlock.h, types.h */

#[repr(C)]
#[derive(Copy, Clone)]
pub enum host1x_class {
    HOST1X_CLASS_HOST1X = 0x1,
    HOST1X_CLASS_NVJPG1 = 0x7,
    HOST1X_CLASS_NVENC = 0x21,
    HOST1X_CLASS_NVENC1 = 0x22,
    HOST1X_CLASS_GR2D = 0x51,
    HOST1X_CLASS_GR2D_SB = 0x52,
    HOST1X_CLASS_VIC = 0x5D,
    HOST1X_CLASS_GR3D = 0x60,
    HOST1X_CLASS_NVJPG = 0xC0,
    HOST1X_CLASS_NVDEC = 0xF0,
    HOST1X_CLASS_NVDEC1 = 0xF5,
    HOST1X_CLASS_OFA = 0xF8,
}

#[repr(C)] pub struct host1x { _private: [u8; 0] }
#[repr(C)] pub struct host1x_channel { _private: [u8; 0] }
#[repr(C)] pub struct host1x_syncpt { _private: [u8; 0] }
#[repr(C)] pub struct host1x_syncpt_base { _private: [u8; 0] }
#[repr(C)] pub struct host1x_job_cmd { _private: [u8; 0] }
#[repr(C)] pub struct host1x_job_unpin_data { _private: [u8; 0] }
#[repr(C)] pub struct host1x_memory_context { _private: [u8; 0] }
#[repr(C)] pub struct device { _private: [u8; 0] }
#[repr(C)] pub struct iommu_group { _private: [u8; 0] }
#[repr(C)] pub struct list_head { _private: [u8; 0] }
#[repr(C)] pub struct mutex { _private: [u8; 0] }
#[repr(C)] pub struct spinlock_t { _private: [u8; 0] }
#[repr(C)] pub struct kref { _private: [u8; 0] }
#[repr(C)] pub struct dma_buf_attachment { _private: [u8; 0] }
#[repr(C)] pub struct sg_table { _private: [u8; 0] }
#[repr(C)] pub struct dma_fence { _private: [u8; 0] }
#[repr(C)] pub struct dma_fence_cb { _private: [u8; 0] }
#[repr(C)] pub struct device_driver { _private: [u8; 0] }
#[repr(C)] pub struct of_device_id { _private: [u8; 0] }
#[repr(C)] pub struct module { _private: [u8; 0] }
#[repr(C)] pub struct lock_class_key { _private: [u8; 0] }
#[repr(C)] pub struct pid { _private: [u8; 0] }
#[repr(C)] pub struct device_dma_parameters { _private: [u8; 0] }

pub type u8_t = u8;
pub type u32_t = u32;
pub type u64_t = u64;
pub type dma_addr_t = usize;
pub type dma_data_direction = i32;
pub type size_t = usize;
pub type refcount_t = i32;

#[repr(C)] pub struct host1x_bo_cache { pub mappings: list_head, pub lock: mutex }
#[repr(C)] pub struct host1x_client_ops {
    pub early_init: Option<unsafe extern "C" fn(*mut host1x_client) -> i32>, pub init: Option<unsafe extern "C" fn(*mut host1x_client) -> i32>, pub exit: Option<unsafe extern "C" fn(*mut host1x_client) -> i32>, pub late_exit: Option<unsafe extern "C" fn(*mut host1x_client) -> i32>, pub suspend: Option<unsafe extern "C" fn(*mut host1x_client) -> i32>, pub resume: Option<unsafe extern "C" fn(*mut host1x_client) -> i32>,
}
#[repr(C)] pub struct host1x_client {
    pub list: list_head, pub host: *mut device, pub dev: *mut device, pub group: *mut iommu_group, pub ops: *const host1x_client_ops, pub class: host1x_class, pub channel: *mut host1x_channel, pub syncpts: *mut *mut host1x_syncpt, pub num_syncpts: u32, pub parent: *mut host1x_client, pub usecount: u32, pub lock: mutex, pub cache: host1x_bo_cache,
}
#[repr(C)] pub struct host1x_bo_mapping { pub ref_: kref, pub attach: *mut dma_buf_attachment, pub direction: dma_data_direction, pub list: list_head, pub bo: *mut host1x_bo, pub sgt: *mut sg_table, pub chunks: u32, pub dev: *mut device, pub phys: dma_addr_t, pub size: size_t, pub cache: *mut host1x_bo_cache, pub entry: list_head }
#[repr(C)] pub struct host1x_bo_ops { pub get: Option<unsafe extern "C" fn(*mut host1x_bo) -> *mut host1x_bo>, pub put: Option<unsafe extern "C" fn(*mut host1x_bo)>, pub pin: Option<unsafe extern "C" fn(*mut device,*mut host1x_bo,dma_data_direction)->*mut host1x_bo_mapping>, pub unpin: Option<unsafe extern "C" fn(*mut host1x_bo_mapping)>, pub mmap: Option<unsafe extern "C" fn(*mut host1x_bo)->*mut core::ffi::c_void>, pub munmap: Option<unsafe extern "C" fn(*mut host1x_bo,*mut core::ffi::c_void)> }
#[repr(C)] pub struct host1x_bo { pub ops: *const host1x_bo_ops, pub mappings: list_head, pub lock: spinlock_t }
#[repr(C)] pub struct host1x_reloc { pub cmdbuf: host1x_reloc_part, pub target: host1x_reloc_part, pub shift: usize, pub flags: usize }
#[repr(C)] pub struct host1x_reloc_part { pub bo: *mut host1x_bo, pub offset: usize }
#[repr(C)] pub struct host1x_job {
    pub ref_: kref, pub list: list_head, pub channel: *mut host1x_channel, pub client: *mut host1x_client, pub cmds: *mut host1x_job_cmd, pub num_cmds: u32, pub relocs: *mut host1x_reloc, pub num_relocs: u32, pub unpins: *mut host1x_job_unpin_data, pub num_unpins: u32, pub addr_phys: *mut dma_addr_t, pub gather_addr_phys: *mut dma_addr_t, pub reloc_addr_phys: *mut dma_addr_t, pub syncpt: *mut host1x_syncpt, pub syncpt_incrs: u32, pub syncpt_end: u32, pub fence: *mut dma_fence, pub fence_cb: dma_fence_cb, pub timeout: u32, pub cancelled: bool, pub first_get: u32, pub num_slots: u32, pub gather_copy_size: size_t, pub gather_copy: dma_addr_t, pub gather_copy_mapped: *mut u8, pub is_addr_reg: Option<unsafe extern "C" fn(*mut device,u32,u32)->i32>, pub is_valid_class: Option<unsafe extern "C" fn(u32)->i32>, pub class: u32, pub serialize: bool, pub syncpt_recovery: bool, pub release: Option<unsafe extern "C" fn(*mut host1x_job)>, pub user_data: *mut core::ffi::c_void, pub enable_firewall: bool, pub memory_context: *mut host1x_memory_context, pub engine_fallback_streamid: u32, pub engine_streamid_offset: u32,
}
#[repr(C)] pub struct host1x_driver { pub driver: device_driver, pub subdevs: *const of_device_id, pub list: list_head, pub probe: Option<unsafe extern "C" fn(*mut host1x_device)->i32>, pub remove: Option<unsafe extern "C" fn(*mut host1x_device)>, pub shutdown: Option<unsafe extern "C" fn(*mut host1x_device)> }
#[repr(C)] pub struct host1x_device { pub driver: *mut host1x_driver, pub list: list_head, pub dev: device, pub subdevs_lock: mutex, pub subdevs: list_head, pub active: list_head, pub clients_lock: mutex, pub clients: list_head, pub registered: bool, pub dma_parms: device_dma_parameters }

pub const HOST1X_SYNCPT_CLIENT_MANAGED: usize = 1 << 0; pub const HOST1X_SYNCPT_HAS_BASE: usize = 1 << 1;
pub const HOST1X_RELOC_READ: usize = 1 << 0; pub const HOST1X_RELOC_WRITE: usize = 1 << 1;

extern "C" {
    fn INIT_LIST_HEAD(list: *mut list_head); fn mutex_init(lock: *mut mutex); fn mutex_destroy(lock: *mut mutex); fn spin_lock_init(lock: *mut spinlock_t);
}
pub unsafe fn host1x_bo_cache_init(cache: *mut host1x_bo_cache) { INIT_LIST_HEAD(&mut (*cache).mappings); mutex_init(&mut (*cache).lock); }
pub unsafe fn host1x_bo_cache_destroy(cache: *mut host1x_bo_cache) { mutex_destroy(&mut (*cache).lock); }
pub unsafe fn host1x_bo_init(bo: *mut host1x_bo, ops: *const host1x_bo_ops) { INIT_LIST_HEAD(&mut (*bo).mappings); spin_lock_init(&mut (*bo).lock); (*bo).ops = ops; }
pub unsafe fn host1x_bo_get(bo: *mut host1x_bo) -> *mut host1x_bo { ((*(*bo).ops).get.unwrap())(bo) }
pub unsafe fn host1x_bo_put(bo: *mut host1x_bo) { ((*(*bo).ops).put.unwrap())(bo); }
pub unsafe fn host1x_bo_mmap(bo: *mut host1x_bo) -> *mut core::ffi::c_void { ((*(*bo).ops).mmap.unwrap())(bo) }
pub unsafe fn host1x_bo_munmap(bo: *mut host1x_bo, addr: *mut core::ffi::c_void) { ((*(*bo).ops).munmap.unwrap())(bo, addr); }

pub unsafe fn to_host1x_driver(driver: *mut device_driver) -> *mut host1x_driver { driver as *mut host1x_driver }
pub unsafe fn to_host1x_device(dev: *mut device) -> *mut host1x_device { dev as *mut host1x_device }
pub unsafe fn host1x_driver_register(driver: *mut host1x_driver, owner: *mut module) -> i32 { host1x_driver_register_full(driver, owner) }
pub unsafe fn host1x_client_init(client: *mut host1x_client, key: *mut lock_class_key) { __host1x_client_init(client, key); }
pub unsafe fn host1x_client_register(client: *mut host1x_client, key: *mut lock_class_key) -> i32 { __host1x_client_init(client, key); __host1x_client_register(client) }

extern "C" {
    pub fn host1x_get_dma_mask(host1x: *mut host1x) -> u64;
    pub fn host1x_bo_pin(dev:*mut device, bo:*mut host1x_bo, dir:dma_data_direction, cache:*mut host1x_bo_cache)->*mut host1x_bo_mapping; pub fn host1x_bo_unpin(map:*mut host1x_bo_mapping); pub fn host1x_bo_clear_cached_mappings(bo:*mut host1x_bo);
    pub fn host1x_syncpt_get_by_id(host:*mut host1x,id:u32)->*mut host1x_syncpt; pub fn host1x_syncpt_get_by_id_noref(host:*mut host1x,id:u32)->*mut host1x_syncpt; pub fn host1x_syncpt_get(sp:*mut host1x_syncpt)->*mut host1x_syncpt; pub fn host1x_syncpt_id(sp:*mut host1x_syncpt)->u32; pub fn host1x_syncpt_read_min(sp:*mut host1x_syncpt)->u32; pub fn host1x_syncpt_read_max(sp:*mut host1x_syncpt)->u32; pub fn host1x_syncpt_read(sp:*mut host1x_syncpt)->u32; pub fn host1x_syncpt_incr(sp:*mut host1x_syncpt)->i32; pub fn host1x_syncpt_incr_max(sp:*mut host1x_syncpt,incrs:u32)->u32; pub fn host1x_syncpt_wait(sp:*mut host1x_syncpt,thresh:u32,timeout:isize,value:*mut u32)->i32; pub fn host1x_syncpt_request(client:*mut host1x_client,flags:usize)->*mut host1x_syncpt; pub fn host1x_syncpt_put(sp:*mut host1x_syncpt); pub fn host1x_syncpt_alloc(host:*mut host1x,flags:usize,name:*const i8)->*mut host1x_syncpt; pub fn host1x_syncpt_get_base(sp:*mut host1x_syncpt)->*mut host1x_syncpt_base; pub fn host1x_syncpt_base_id(base:*mut host1x_syncpt_base)->u32; pub fn host1x_syncpt_release_vblank_reservation(client:*mut host1x_client,id:u32); pub fn host1x_fence_create(sp:*mut host1x_syncpt,threshold:u32,timeout:bool)->*mut dma_fence; pub fn host1x_fence_cancel(fence:*mut dma_fence);
    pub fn host1x_channel_request(client:*mut host1x_client)->*mut host1x_channel; pub fn host1x_channel_get(channel:*mut host1x_channel)->*mut host1x_channel; pub fn host1x_channel_stop(channel:*mut host1x_channel); pub fn host1x_channel_put(channel:*mut host1x_channel); pub fn host1x_job_submit(job:*mut host1x_job)->i32;
    pub fn host1x_job_alloc(ch:*mut host1x_channel,num_cmdbufs:u32,num_relocs:u32,skip_firewall:bool)->*mut host1x_job; pub fn host1x_job_add_gather(job:*mut host1x_job,bo:*mut host1x_bo,words:u32,offset:u32); pub fn host1x_job_add_wait(job:*mut host1x_job,id:u32,thresh:u32,relative:bool,next_class:u32); pub fn host1x_job_get(job:*mut host1x_job)->*mut host1x_job; pub fn host1x_job_put(job:*mut host1x_job); pub fn host1x_job_pin(job:*mut host1x_job,dev:*mut device)->i32; pub fn host1x_job_unpin(job:*mut host1x_job);
    pub fn host1x_driver_register_full(driver:*mut host1x_driver,owner:*mut module)->i32; pub fn host1x_driver_unregister(driver:*mut host1x_driver); pub fn host1x_device_init(device:*mut host1x_device)->i32; pub fn host1x_device_exit(device:*mut host1x_device)->i32; pub fn __host1x_client_init(client:*mut host1x_client,key:*mut lock_class_key); pub fn host1x_client_exit(client:*mut host1x_client); pub fn __host1x_client_register(client:*mut host1x_client)->i32; pub fn host1x_client_unregister(client:*mut host1x_client); pub fn host1x_client_suspend(client:*mut host1x_client)->i32; pub fn host1x_client_resume(client:*mut host1x_client)->i32;
}

#[cfg(feature = "CONFIG_IOMMU_API")]
extern "C" { pub fn host1x_memory_context_alloc(host1x:*mut host1x,dev:*mut device,pid:*mut pid)->*mut host1x_memory_context; pub fn host1x_memory_context_get(cd:*mut host1x_memory_context); pub fn host1x_memory_context_put(cd:*mut host1x_memory_context); }
#[cfg(not(feature = "CONFIG_IOMMU_API"))]
pub unsafe fn host1x_memory_context_alloc(_: *mut host1x, _: *mut device, _: *mut pid) -> *mut host1x_memory_context { core::ptr::null_mut() }
#[cfg(not(feature = "CONFIG_IOMMU_API"))] pub unsafe fn host1x_memory_context_get(_: *mut host1x_memory_context) {}
#[cfg(not(feature = "CONFIG_IOMMU_API"))] pub unsafe fn host1x_memory_context_put(_: *mut host1x_memory_context) {}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
