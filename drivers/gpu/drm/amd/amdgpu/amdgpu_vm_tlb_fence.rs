// SPDX-License-Identifier: GPL-2.0 OR MIT
/*
 * Copyright 2023 Advanced Micro Devices, Inc.
 *
 * Permission is hereby granted, free of charge, to any person obtaining a
 * copy of this software and associated documentation files (the "Software"),
 * to deal in the Software without restriction, including without limitation
 * the rights to use, copy, modify, merge, publish, distribute, sublicense,
 * and/or sell copies of the Software, and to permit persons to whom the
 * Software is furnished to do so, subject to the following conditions:
 *
 * THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
 * IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
 * FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL
 * THE COPYRIGHT HOLDER(S) OR AUTHOR(S) BE LIABLE FOR ANY CLAIM, DAMAGES OR
 * OTHER LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE,
 * ARISING FROM, OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR
 * OTHER DEALINGS IN THE SOFTWARE.
 */

use core::ffi::{c_char, c_int, c_void};

#[repr(C)]
pub struct dma_fence { _opaque: [u8; 0] }
#[repr(C)]
pub struct work_struct { _opaque: [u8; 0] }
#[repr(C)]
pub struct spinlock_t { _opaque: [u8; 0] }
#[repr(C)]
pub struct amdgpu_device { pub dev: *mut c_void }
#[repr(C)]
pub struct amdgpu_vm {
    pub pasid: u16,
    pub tlb_fence_context: u64,
    pub tlb_seq: u64,
}

#[repr(C)]
pub struct dma_fence_ops {
    pub get_driver_name: Option<unsafe extern "C" fn(*mut dma_fence) -> *const c_char>,
    pub get_timeline_name: Option<unsafe extern "C" fn(*mut dma_fence) -> *const c_char>,
}

#[repr(C)]
pub struct amdgpu_tlb_fence {
    pub base: dma_fence,
    pub adev: *mut amdgpu_device,
    pub dependency: *mut dma_fence,
    pub work: work_struct,
    pub lock: spinlock_t,
    pub pasid: u16,
}

extern "C" {
    fn dma_fence_wait(fence: *mut dma_fence, intr: bool) -> c_int;
    fn dma_fence_put(fence: *mut dma_fence);
    fn dma_fence_set_error(fence: *mut dma_fence, error: c_int);
    fn dma_fence_signal(fence: *mut dma_fence);
    fn dma_fence_get(fence: *mut dma_fence) -> *mut dma_fence;
    fn dma_fence_get_stub() -> *mut dma_fence;
    fn dma_fence_init64(fence: *mut dma_fence, ops: *const dma_fence_ops,
                        lock: *mut spinlock_t, context: u64, seqno: u64);
    fn init_work(work: *mut work_struct, func: unsafe extern "C" fn(*mut work_struct));
    fn spin_lock_init(lock: *mut spinlock_t);
    fn schedule_work(work: *mut work_struct);
    fn amdgpu_gmc_flush_gpu_tlb_pasid(adev: *mut amdgpu_device, pasid: u16,
                                       flush_type: u32, all_hub: bool, inst: u32) -> c_int;
    fn dev_err(dev: *mut c_void, fmt: *const c_char, ...);
    fn atomic64_read(value: *const u64) -> u64;
}

unsafe extern "C" fn amdgpu_tlb_fence_get_driver_name(_fence: *mut dma_fence) -> *const c_char {
    b"amdgpu tlb fence\0".as_ptr() as *const c_char
}

unsafe extern "C" fn amdgpu_tlb_fence_get_timeline_name(_f: *mut dma_fence) -> *const c_char {
    b"amdgpu tlb timeline\0".as_ptr() as *const c_char
}

unsafe extern "C" fn amdgpu_tlb_fence_work(work: *mut work_struct) {
    let f = (work as *mut u8).sub(core::mem::offset_of!(amdgpu_tlb_fence, work))
        as *mut amdgpu_tlb_fence;
    let mut r: c_int;

    if !(*f).dependency.is_null() {
        dma_fence_wait((*f).dependency, false);
        dma_fence_put((*f).dependency);
        (*f).dependency = core::ptr::null_mut();
    }

    r = amdgpu_gmc_flush_gpu_tlb_pasid((*f).adev, (*f).pasid, 2, true, 0);
    if r != 0 {
        dev_err((*(*f).adev).dev, b"TLB flush failed for PASID %d.\n\0".as_ptr() as *const c_char,
                (*f).pasid as c_int);
        dma_fence_set_error(&mut (*f).base, r);
    }

    dma_fence_signal(&mut (*f).base);
    dma_fence_put(&mut (*f).base);
}

#[no_mangle]
pub static amdgpu_tlb_fence_ops: dma_fence_ops = dma_fence_ops {
    get_driver_name: Some(amdgpu_tlb_fence_get_driver_name),
    get_timeline_name: Some(amdgpu_tlb_fence_get_timeline_name),
};

#[no_mangle]
pub unsafe extern "C" fn amdgpu_vm_tlb_fence_create(
    adev: *mut amdgpu_device,
    vm: *mut amdgpu_vm,
    fence: *mut *mut dma_fence,
) {
    let f = Box::into_raw(Box::new(core::mem::zeroed::<amdgpu_tlb_fence>()));

    if f.is_null() {
        if !(*fence).is_null() { dma_fence_wait(*fence, false); }
        amdgpu_gmc_flush_gpu_tlb_pasid(adev, (*vm).pasid, 2, true, 0);
        *fence = dma_fence_get_stub();
        return;
    }

    (*f).adev = adev;
    (*f).dependency = *fence;
    (*f).pasid = (*vm).pasid;
    init_work(&mut (*f).work, amdgpu_tlb_fence_work);
    spin_lock_init(&mut (*f).lock);
    dma_fence_init64(&mut (*f).base, &amdgpu_tlb_fence_ops, &mut (*f).lock,
                     (*vm).tlb_fence_context, atomic64_read(&(*vm).tlb_seq));

    dma_fence_get(&mut (*f).base);
    schedule_work(&mut (*f).work);
    *fence = &mut (*f).base;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
