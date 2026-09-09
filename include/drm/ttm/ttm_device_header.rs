/*
 * Copyright 2020 Advanced Micro Devices, Inc.
 *
 * Permission is hereby granted, free of charge, to any person obtaining a
 * copy of this software and associated documentation files (the "Software"),
 * to deal in the Software without restriction, including without limitation
 * the rights to use, copy, modify, merge, publish, distribute, sublicense,
 * and/or sell copies of the Software, and to permit persons to whom the
 * Software is furnished to do so, subject to the following conditions:
 *
 * The above copyright notice and this permission notice shall be included in
 * all copies or substantial portions of the Software.
 *
 * THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
 * IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
 * FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT.  IN NO EVENT SHALL
 * THE COPYRIGHT HOLDER(S) OR AUTHOR(S) BE LIABLE FOR ANY CLAIM, DAMAGES OR
 * OTHER LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE,
 * ARISING FROM, OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR
 * OTHER DEALINGS IN THE SOFTWARE.
 *
 * Authors: Christian König
 */

use core::ffi::c_void;

// Dependencies supplied by the surrounding translation unit.
pub type uint32_t = u32;
pub type c_int = i32;
pub type c_uint = u32;
pub type c_ulong = usize;
pub type gfp_t = usize;

pub enum ttm_placement {}
pub enum ttm_buffer_object {}
pub enum ttm_operation_ctx {}
pub enum ttm_tt {}
pub enum page {}
pub enum drm_vma_offset_manager {}
pub enum address_space {}
pub enum workqueue_struct {}

#[repr(C)]
pub struct list_head { pub next: *mut list_head, pub prev: *mut list_head }
#[repr(C)]
pub struct atomic_t { pub counter: c_int }
#[repr(C)]
pub struct spinlock_t { pub raw: [u8; 0] }
pub enum ttm_resource_manager {}
pub enum ttm_pool {}
pub enum ttm_resource {}
pub enum ttm_place {}
pub enum device {}

/** struct ttm_global - Buffer object driver global data. */
#[repr(C)]
pub struct ttm_global {
    pub dummy_read_page: *mut page,
    pub device_list: list_head,
    pub bo_count: atomic_t,
}

extern "C" {
    pub static mut ttm_glob: ttm_global;

    pub fn ttm_global_swapout(ctx: *mut ttm_operation_ctx, gfp_flags: gfp_t) -> c_int;
    pub fn ttm_device_swapout(bdev: *mut ttm_device, ctx: *mut ttm_operation_ctx,
                              gfp_flags: gfp_t) -> c_int;
    pub fn ttm_device_prepare_hibernation(bdev: *mut ttm_device) -> c_int;
    pub fn ttm_device_init(bdev: *mut ttm_device, funcs: *const ttm_device_funcs,
                           dev: *mut device, mapping: *mut address_space,
                           vma_manager: *mut drm_vma_offset_manager,
                           alloc_flags: c_uint) -> c_int;
    pub fn ttm_device_fini(bdev: *mut ttm_device);
    pub fn ttm_device_clear_dma_mappings(bdev: *mut ttm_device);
}

#[repr(C)]
pub struct ttm_device_funcs {
    pub ttm_tt_create: Option<unsafe extern "C" fn(bo: *mut ttm_buffer_object,
                                                     page_flags: uint32_t) -> *mut ttm_tt>,
    pub ttm_tt_populate: Option<unsafe extern "C" fn(bdev: *mut ttm_device,
                                                       ttm: *mut ttm_tt,
                                                       ctx: *mut ttm_operation_ctx) -> c_int>,
    pub ttm_tt_unpopulate: Option<unsafe extern "C" fn(bdev: *mut ttm_device,
                                                         ttm: *mut ttm_tt)>,
    pub ttm_tt_destroy: Option<unsafe extern "C" fn(bdev: *mut ttm_device,
                                                       ttm: *mut ttm_tt)>,
    pub eviction_valuable: Option<unsafe extern "C" fn(bo: *mut ttm_buffer_object,
                                                         place: *const ttm_place) -> bool>,
    pub evict_flags: Option<unsafe extern "C" fn(bo: *mut ttm_buffer_object,
                                                   placement: *mut ttm_placement)>,
    pub move_: Option<unsafe extern "C" fn(bo: *mut ttm_buffer_object, evict: bool,
                                             ctx: *mut ttm_operation_ctx,
                                             new_mem: *mut ttm_resource,
                                             hop: *mut ttm_place) -> c_int>,
    pub delete_mem_notify: Option<unsafe extern "C" fn(bo: *mut ttm_buffer_object)>,
    pub swap_notify: Option<unsafe extern "C" fn(bo: *mut ttm_buffer_object)>,
    pub io_mem_reserve: Option<unsafe extern "C" fn(bdev: *mut ttm_device,
                                                      mem: *mut ttm_resource) -> c_int>,
    pub io_mem_free: Option<unsafe extern "C" fn(bdev: *mut ttm_device,
                                                   mem: *mut ttm_resource)>,
    pub io_mem_pfn: Option<unsafe extern "C" fn(bo: *mut ttm_buffer_object,
                                                  page_offset: c_ulong) -> c_ulong>,
    pub access_memory: Option<unsafe extern "C" fn(bo: *mut ttm_buffer_object,
                                                     offset: c_ulong, buf: *mut c_void,
                                                     len: c_int, write: c_int) -> c_int>,
    pub release_notify: Option<unsafe extern "C" fn(bo: *mut ttm_buffer_object)>,
}

#[repr(C)]
pub struct ttm_device {
    pub device_list: list_head,
    pub alloc_flags: c_uint,
    pub funcs: *const ttm_device_funcs,
    pub sysman: ttm_resource_manager,
    pub man_drv: [*mut ttm_resource_manager; TTM_NUM_MEM_TYPES],
    pub vma_manager: *mut drm_vma_offset_manager,
    pub pool: ttm_pool,
    pub lru_lock: spinlock_t,
    pub unevictable: list_head,
    pub dev_mapping: *mut address_space,
    pub wq: *mut workqueue_struct,
}

// Build-time constant and bounds check corresponding to TTM_NUM_MEM_TYPES.
pub const TTM_NUM_MEM_TYPES: usize = 16;

#[inline]
pub unsafe fn ttm_manager_type(bdev: *mut ttm_device, mem_type: c_int) -> *mut ttm_resource_manager {
    (*bdev).man_drv[mem_type as usize]
}

#[inline]
pub unsafe fn ttm_set_driver_manager(bdev: *mut ttm_device, ty: c_int,
                                     manager: *mut ttm_resource_manager) {
    (*bdev).man_drv[ty as usize] = manager;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
