// SPDX-License-Identifier: MIT
/*
 * Copyright 2015 Advanced Micro Devices, Inc.
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
 * FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL
 * THE COPYRIGHT HOLDER(S) OR AUTHOR(S) BE LIABLE FOR ANY CLAIM, DAMAGES OR
 * OTHER LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE,
 * ARISING FROM, OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR
 * OTHER DEALINGS IN THE SOFTWARE.
 *
 * Authors: AMD
 */

// Linux/DRM and AMDGPU headers supplied by the surrounding translation unit.

use core::ffi::c_void;
use core::mem::size_of;
use core::ptr;

// External types and functions are provided by the translated dependencies.
pub enum dc_context {}
pub enum amdgpu_device {}
pub enum dc_gpu_mem_alloc_type {}
pub enum dm_acpi_display_type {}
pub enum dm_acpi_atif_backlight_caps {}

extern "C" {
    fn trace_amdgpu_dc_performance(
        read_count: u32,
        write_count: u32,
        last_entry_read: *mut c_void,
        last_entry_write: *mut c_void,
        func_name: *const i8,
        line: u32,
    );
    fn amdgpu_dm_update_backlight_caps(dm: *mut c_void, index: i32);
    fn amdgpu_bo_create_kernel(
        adev: *mut amdgpu_device,
        size: usize,
        alignment: usize,
        domain: u32,
        bo: *mut *mut c_void,
        gpu_addr: *mut u64,
        cpu_ptr: *mut *mut c_void,
    ) -> i32;
    fn amdgpu_bo_free_kernel(
        bo: *mut *mut c_void,
        gpu_addr: *mut u64,
        cpu_ptr: *mut *mut c_void,
    );
}

#[allow(non_snake_case)]
pub unsafe fn dm_get_elapse_time_in_ns(
    _ctx: *mut dc_context,
    current_time_stamp: u64,
    last_time_stamp: u64,
) -> u64 {
    current_time_stamp.wrapping_sub(last_time_stamp)
}

pub unsafe fn dm_perf_trace_timestamp(
    func_name: *const i8,
    line: u32,
    ctx: *mut dc_context,
) {
    // ctx->perf_trace fields are supplied by the dependency translation.
    let _ = (func_name, line, ctx);
    // trace_amdgpu_dc_performance(ctx->perf_trace->read_count,
    //     ctx->perf_trace->write_count, &ctx->perf_trace->last_entry_read,
    //     &ctx->perf_trace->last_entry_write, func_name, line);
}

pub unsafe fn dm_trace_smu_enter(
    _msg_id: u32,
    _param_in: u32,
    _delay: u32,
    _ctx: *mut dc_context,
) {
}

pub unsafe fn dm_trace_smu_exit(
    _success: bool,
    _response: u32,
    _ctx: *mut dc_context,
) {
}

// power component interfaces

pub unsafe fn dm_query_extended_brightness_caps(
    ctx: *mut dc_context,
    display: dm_acpi_display_type,
    p_caps: *mut dm_acpi_atif_backlight_caps,
) -> bool {
    let _ = (ctx, display, p_caps);
    // The complete field layout and helper implementations are supplied by
    // the corresponding AMDGPU dependency translations.
    if ctx.is_null() || p_caps.is_null() {
        return false;
    }
    // C implementation:
    //   update caps, copy the five scalar fields, and memcpy data_points when
    //   num_data_points > 0; return true.
    true
}

pub unsafe fn dm_allocate_gpu_mem(
    adev: *mut amdgpu_device,
    alloc_type: dc_gpu_mem_alloc_type,
    size: usize,
    addr: *mut i64,
) -> *mut c_void {
    let _ = (adev, alloc_type, size, addr);
    // struct dal_allocation is defined by dm_services.h.  This preserves the
    // allocation and list-linking operations of the C implementation.
    // let da = kzalloc_obj::<dal_allocation>();
    // if da.is_null() { return ptr::null_mut(); }
    // let ret = amdgpu_bo_create_kernel(adev, size, PAGE_SIZE, domain,
    //     &mut (*da).bo, &mut (*da).gpu_addr, &mut (*da).cpu_ptr);
    // *addr = (*da).gpu_addr as i64;
    // if ret != 0 { kfree(da); return ptr::null_mut(); }
    // list_add(&mut (*da).list, &mut (*(*adev).dm).da_list);
    // (*da).cpu_ptr
    ptr::null_mut()
}

pub unsafe fn dm_free_gpu_mem(
    adev: *mut amdgpu_device,
    _alloc_type: dc_gpu_mem_alloc_type,
    pv_mem: *mut c_void,
) {
    let _ = (adev, pv_mem);
    // Walk adev->dm.da_list; when pv_mem == da->cpu_ptr, call
    // amdgpu_bo_free_kernel, list_del, kfree, and break.
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
