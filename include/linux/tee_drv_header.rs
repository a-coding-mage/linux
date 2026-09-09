/* SPDX-License-Identifier: GPL-2.0-only */
/* Copyright (c) 2015-2024 Linaro Limited */

// Translated from tee_drv.h.  Kernel-provided types and macros remain external
// dependencies supplied by the surrounding Rust kernel bindings.

use core::ffi::c_void;

#[repr(C)]
pub struct tee_device;

#[repr(C)]
pub struct tee_context {
    pub teedev: *mut tee_device,
    pub data: *mut c_void,
    pub refcount: kref,
    pub releasing: bool,
    pub supp_nowait: bool,
    pub cap_memref_null: bool,
}

#[repr(C)]
pub struct tee_shm {
    pub ctx: *mut tee_context,
    pub paddr: phys_addr_t,
    pub kaddr: *mut c_void,
    pub size: usize,
    pub offset: u32,
    pub pages: *mut *mut page,
    pub num_pages: usize,
    pub refcount: refcount_t,
    pub flags: u32,
    pub id: i32,
    pub sec_world_id: u64,
}

#[repr(C)]
pub struct tee_param_memref {
    pub shm_offs: usize,
    pub size: usize,
    pub shm: *mut tee_shm,
}

#[repr(C)]
pub struct tee_param_ubuf {
    pub uaddr: *mut c_void,
    pub size: usize,
}

#[repr(C)]
pub struct tee_param_objref {
    pub id: u64,
    pub flags: u64,
}

#[repr(C)]
pub struct tee_param_value {
    pub a: u64,
    pub b: u64,
    pub c: u64,
}

#[repr(C)]
pub union tee_param_union {
    pub memref: tee_param_memref,
    pub objref: tee_param_objref,
    pub ubuf: tee_param_ubuf,
    pub value: tee_param_value,
}

#[repr(C)]
pub struct tee_param {
    pub attr: u64,
    pub u: tee_param_union,
}

extern "C" {
    pub fn tee_shm_alloc_kernel_buf(ctx: *mut tee_context, size: usize) -> *mut tee_shm;
    pub fn tee_shm_register_kernel_buf(
        ctx: *mut tee_context,
        addr: *mut c_void,
        length: usize,
    ) -> *mut tee_shm;
    pub fn tee_shm_register_fd(ctx: *mut tee_context, fd: i32) -> *mut tee_shm;
    pub fn tee_shm_free(shm: *mut tee_shm);
    pub fn tee_shm_get_va(shm: *mut tee_shm, offs: usize) -> *mut c_void;
    pub fn tee_shm_get_pa(shm: *mut tee_shm, offs: usize, pa: *mut phys_addr_t) -> i32;

    pub fn tee_client_open_context(
        start: *mut tee_context,
        r#match: Option<unsafe extern "C" fn(*mut tee_ioctl_version_data, *const c_void) -> i32>,
        data: *const c_void,
        vers: *mut tee_ioctl_version_data,
    ) -> *mut tee_context;
    pub fn tee_client_close_context(ctx: *mut tee_context);
    pub fn tee_client_get_version(ctx: *mut tee_context, vers: *mut tee_ioctl_version_data);
    pub fn tee_client_open_session(
        ctx: *mut tee_context,
        arg: *mut tee_ioctl_open_session_arg,
        param: *mut tee_param,
    ) -> i32;
    pub fn tee_client_close_session(ctx: *mut tee_context, session: u32) -> i32;
    pub fn tee_client_system_session(ctx: *mut tee_context, session: u32) -> i32;
    pub fn tee_client_invoke_func(
        ctx: *mut tee_context,
        arg: *mut tee_ioctl_invoke_arg,
        param: *mut tee_param,
    ) -> i32;
    pub fn tee_client_cancel_req(ctx: *mut tee_context, arg: *mut tee_ioctl_cancel_arg) -> i32;

    pub static tee_bus_type: bus_type;
    pub fn __tee_client_driver_register(
        driver: *mut tee_client_driver,
        module: *mut module,
    ) -> i32;
    pub fn tee_client_driver_unregister(driver: *mut tee_client_driver);
}

#[inline]
pub unsafe fn tee_shm_get_size(shm: *mut tee_shm) -> usize {
    (*shm).size
}

#[inline]
pub unsafe fn tee_shm_get_pages(shm: *mut tee_shm, num_pages: *mut usize) -> *mut *mut page {
    *num_pages = (*shm).num_pages;
    (*shm).pages
}

#[inline]
pub unsafe fn tee_shm_get_page_offset(shm: *mut tee_shm) -> usize {
    (*shm).offset as usize
}

#[repr(C)]
pub struct tee_client_device {
    pub id: tee_client_device_id,
    pub dev: device,
}

#[repr(C)]
pub struct tee_client_driver {
    pub probe: Option<unsafe extern "C" fn(*mut tee_client_device) -> i32>,
    pub remove: Option<unsafe extern "C" fn(*mut tee_client_device)>,
    pub shutdown: Option<unsafe extern "C" fn(*mut tee_client_device)>,
    pub id_table: *const tee_client_device_id,
    pub driver: device_driver,
}

// C macros retained as intended operations; their container_of implementations
// are provided by the surrounding kernel bindings:
// to_tee_client_device(d) = container_of(d, struct tee_client_device, dev)
// to_tee_client_driver(d) = container_of_const(d, struct tee_client_driver, driver)
// tee_client_driver_register(drv) = __tee_client_driver_register(drv, THIS_MODULE)
// module_tee_client_driver(drv) = module_driver(drv, tee_client_driver_register,
//                                                tee_client_driver_unregister)

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
