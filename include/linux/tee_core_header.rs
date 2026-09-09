/* SPDX-License-Identifier: GPL-2.0-only */
/* Copyright (c) 2024 Linaro Limited */

/* Translated from linux/tee_core.h. Included kernel types are external dependencies. */

pub const TEE_SHM_DYNAMIC: u32 = 1 << 0;
pub const TEE_SHM_USER_MAPPED: u32 = 1 << 1;
pub const TEE_SHM_POOL: u32 = 1 << 2;
pub const TEE_SHM_PRIV: u32 = 1 << 3;
pub const TEE_SHM_DMA_BUF: u32 = 1 << 4;
pub const TEE_SHM_DMA_MEM: u32 = 1 << 5;

pub const TEE_DEVICE_FLAG_REGISTERED: u32 = 0x1;
pub const TEE_MAX_DEV_NAME_LEN: usize = 32;

#[repr(C)]
pub enum tee_dma_heap_id {
    TEE_DMA_HEAP_SECURE_VIDEO_PLAY = 1,
    TEE_DMA_HEAP_TRUSTED_UI,
    TEE_DMA_HEAP_SECURE_VIDEO_RECORD,
}

#[repr(C)]
pub struct tee_device {
    pub name: [core::ffi::c_char; TEE_MAX_DEV_NAME_LEN],
    pub desc: *const tee_desc,
    pub id: core::ffi::c_int,
    pub flags: core::ffi::c_uint,
    pub dev: device,
    pub cdev: cdev,
    pub num_users: usize,
    pub c_no_users: completion,
    pub mutex: mutex,
    pub idr: idr,
    pub pool: *mut tee_shm_pool,
}

#[repr(C)]
pub struct tee_driver_ops {
    pub get_version: Option<unsafe extern "C" fn(*mut tee_device, *mut tee_ioctl_version_data)>,
    pub get_tee_revision: Option<unsafe extern "C" fn(*mut tee_device, *mut core::ffi::c_char, usize) -> core::ffi::c_int>,
    pub open: Option<unsafe extern "C" fn(*mut tee_context) -> core::ffi::c_int>,
    pub close_context: Option<unsafe extern "C" fn(*mut tee_context)>,
    pub release: Option<unsafe extern "C" fn(*mut tee_context)>,
    pub open_session: Option<unsafe extern "C" fn(*mut tee_context, *mut tee_ioctl_open_session_arg, *mut tee_param) -> core::ffi::c_int>,
    pub close_session: Option<unsafe extern "C" fn(*mut tee_context, u32) -> core::ffi::c_int>,
    pub system_session: Option<unsafe extern "C" fn(*mut tee_context, u32) -> core::ffi::c_int>,
    pub invoke_func: Option<unsafe extern "C" fn(*mut tee_context, *mut tee_ioctl_invoke_arg, *mut tee_param) -> core::ffi::c_int>,
    pub object_invoke_func: Option<unsafe extern "C" fn(*mut tee_context, *mut tee_ioctl_object_invoke_arg, *mut tee_param) -> core::ffi::c_int>,
    pub cancel_req: Option<unsafe extern "C" fn(*mut tee_context, u32, u32) -> core::ffi::c_int>,
    pub supp_recv: Option<unsafe extern "C" fn(*mut tee_context, *mut u32, *mut u32, *mut tee_param) -> core::ffi::c_int>,
    pub supp_send: Option<unsafe extern "C" fn(*mut tee_context, u32, u32, *mut tee_param) -> core::ffi::c_int>,
    pub shm_register: Option<unsafe extern "C" fn(*mut tee_context, *mut tee_shm, *mut *mut page, usize, usize) -> core::ffi::c_int>,
    pub shm_unregister: Option<unsafe extern "C" fn(*mut tee_context, *mut tee_shm) -> core::ffi::c_int>,
}

pub const TEE_REVISION_STR_SIZE: usize = 128;
pub const TEE_DESC_PRIVILEGED: u32 = 0x1;

#[repr(C)]
pub struct tee_desc {
    pub name: *const core::ffi::c_char,
    pub ops: *const tee_driver_ops,
    pub owner: *mut module,
    pub flags: u32,
}

#[repr(C)]
pub struct tee_protmem_pool { pub ops: *const tee_protmem_pool_ops }

#[repr(C)]
pub struct tee_protmem_pool_ops {
    pub alloc: Option<unsafe extern "C" fn(*mut tee_protmem_pool, *mut sg_table, usize, *mut usize) -> core::ffi::c_int>,
    pub free: Option<unsafe extern "C" fn(*mut tee_protmem_pool, *mut sg_table)>,
    pub update_shm: Option<unsafe extern "C" fn(*mut tee_protmem_pool, *mut sg_table, usize, *mut tee_shm, *mut *mut tee_shm) -> core::ffi::c_int>,
    pub destroy_pool: Option<unsafe extern "C" fn(*mut tee_protmem_pool)>,
}

#[repr(C)]
pub struct tee_shm_pool { pub ops: *const tee_shm_pool_ops, pub private_data: *mut core::ffi::c_void }

#[repr(C)]
pub struct tee_shm_pool_ops {
    pub alloc: Option<unsafe extern "C" fn(*mut tee_shm_pool, *mut tee_shm, usize, usize) -> core::ffi::c_int>,
    pub free: Option<unsafe extern "C" fn(*mut tee_shm_pool, *mut tee_shm)>,
    pub destroy_pool: Option<unsafe extern "C" fn(*mut tee_shm_pool)>,
}

extern "C" {
    pub fn tee_device_alloc(teedesc: *const tee_desc, dev: *mut device, pool: *mut tee_shm_pool, driver_data: *mut core::ffi::c_void) -> *mut tee_device;
    pub fn tee_device_register(teedev: *mut tee_device) -> core::ffi::c_int;
    pub fn tee_device_unregister(teedev: *mut tee_device);
    pub fn tee_device_register_dma_heap(teedev: *mut tee_device, id: tee_dma_heap_id, pool: *mut tee_protmem_pool) -> core::ffi::c_int;
    pub fn tee_device_put_all_dma_heaps(teedev: *mut tee_device);
    pub fn tee_device_get(teedev: *mut tee_device) -> bool;
    pub fn tee_device_put(teedev: *mut tee_device);
    pub fn tee_device_set_dev_groups(teedev: *mut tee_device, dev_groups: *const *const attribute_group);
    pub fn tee_session_calc_client_uuid(uuid: *mut uuid_t, connection_method: u32, connection_data: *const u8) -> core::ffi::c_int;
    pub fn tee_shm_pool_alloc_res_mem(vaddr: usize, paddr: phys_addr_t, size: usize, min_alloc_order: core::ffi::c_int) -> *mut tee_shm_pool;
    pub fn tee_protmem_static_pool_alloc(paddr: phys_addr_t, size: usize) -> *mut tee_protmem_pool;
    pub fn tee_get_drvdata(teedev: *mut tee_device) -> *mut core::ffi::c_void;
    pub fn tee_shm_alloc_priv_buf(ctx: *mut tee_context, size: usize) -> *mut tee_shm;
    pub fn tee_shm_alloc_dma_mem(ctx: *mut tee_context, page_count: usize) -> *mut tee_shm;
    pub fn tee_dyn_shm_alloc_helper(shm: *mut tee_shm, size: usize, align: usize, shm_register: Option<unsafe extern "C" fn(*mut tee_context, *mut tee_shm, *mut *mut page, usize, usize) -> core::ffi::c_int>) -> core::ffi::c_int;
    pub fn tee_dyn_shm_free_helper(shm: *mut tee_shm, shm_unregister: Option<unsafe extern "C" fn(*mut tee_context, *mut tee_shm) -> core::ffi::c_int>);
    pub fn tee_shm_put(shm: *mut tee_shm);
    pub fn tee_shm_get_from_id(ctx: *mut tee_context, id: core::ffi::c_int) -> *mut tee_shm;
    pub fn teedev_open(teedev: *mut tee_device) -> *mut tee_context;
    pub fn teedev_close_context(ctx: *mut tee_context);
    pub fn teedev_ctx_get(ctx: *mut tee_context);
    pub fn teedev_ctx_put(ctx: *mut tee_context);
}

pub unsafe fn tee_shm_pool_free(pool: *mut tee_shm_pool) { ((*pool).ops.as_ref().unwrap().destroy_pool.unwrap())(pool); }
pub unsafe fn tee_shm_is_dynamic(shm: *mut tee_shm) -> bool { !shm.is_null() && ((*shm).flags & TEE_SHM_DYNAMIC) != 0 }
pub unsafe fn tee_shm_get_id(shm: *mut tee_shm) -> core::ffi::c_int { (*shm).id }
pub unsafe fn tee_param_is_memref(param: *mut tee_param) -> bool {
    match (*param).attr & TEE_IOCTL_PARAM_ATTR_TYPE_MASK {
        TEE_IOCTL_PARAM_ATTR_TYPE_MEMREF_INPUT | TEE_IOCTL_PARAM_ATTR_TYPE_MEMREF_OUTPUT | TEE_IOCTL_PARAM_ATTR_TYPE_MEMREF_INOUT => true,
        _ => false,
    }
}

/* External kernel and TEE types referenced above. */
pub enum device {}
pub enum cdev {}
pub enum completion {}
pub enum mutex {}
pub enum idr {}
pub enum module {}
pub enum sg_table {}
pub enum page {}
pub enum tee_context {}
pub enum tee_shm { pub flags: u32, pub id: core::ffi::c_int }
pub enum tee_param { pub attr: u64 }
pub enum tee_ioctl_version_data {}
pub enum tee_ioctl_open_session_arg {}
pub enum tee_ioctl_invoke_arg {}
pub enum tee_ioctl_object_invoke_arg {}
pub enum attribute_group {}
pub enum uuid_t {}
pub type phys_addr_t = u64;
pub const TEE_IOCTL_PARAM_ATTR_TYPE_MASK: u64 = 0;
pub const TEE_IOCTL_PARAM_ATTR_TYPE_MEMREF_INPUT: u64 = 0;
pub const TEE_IOCTL_PARAM_ATTR_TYPE_MEMREF_OUTPUT: u64 = 0;
pub const TEE_IOCTL_PARAM_ATTR_TYPE_MEMREF_INOUT: u64 = 0;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
