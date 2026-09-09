/*
 * Copyright 2016 Advanced Micro Devices, Inc.
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
 * Author: Monk.liu@amd.com
 */

use core::ffi::c_void;

// Definitions and functions supplied by the surrounding AMDGPU sources.
extern "C" {
    fn AMDGPU_VA_RESERVED_CSA_START(adev: *mut amdgpu_device) -> u64;
    fn amdgpu_gmc_sign_extend(addr: u64) -> u64;
    fn amdgpu_bo_create_kernel(
        adev: *mut amdgpu_device, size: u32, alignment: usize, domain: u32,
        bo: *mut *mut amdgpu_bo, sg: *mut c_void, cpu_addr: *mut *mut c_void,
    ) -> i32;
    fn amdgpu_bo_free_kernel(
        bo: *mut *mut amdgpu_bo, sg: *mut c_void, cpu_addr: *mut *mut c_void,
    );
    fn drm_exec_init(exec: *mut drm_exec, flags: u32, nr: u32);
    fn drm_exec_fini(exec: *mut drm_exec);
    fn amdgpu_vm_lock_pd(vm: *mut amdgpu_vm, exec: *mut drm_exec, x: u32) -> i32;
    fn drm_exec_lock_obj(exec: *mut drm_exec, obj: *mut drm_gem_object) -> i32;
    fn amdgpu_vm_bo_add(adev: *mut amdgpu_device, vm: *mut amdgpu_vm, bo: *mut amdgpu_bo) -> *mut amdgpu_bo_va;
    fn amdgpu_vm_bo_map(adev: *mut amdgpu_device, bo_va: *mut amdgpu_bo_va, addr: u64, offset: u64, size: u32, flags: u64) -> i32;
    fn amdgpu_vm_bo_unmap(adev: *mut amdgpu_device, bo_va: *mut amdgpu_bo_va, addr: u64) -> i32;
    fn amdgpu_vm_bo_del(adev: *mut amdgpu_device, bo_va: *mut amdgpu_bo_va);
    fn adev_to_drm(adev: *mut amdgpu_device) -> *mut c_void;
    fn drm_err(drm: *mut c_void, fmt: *const u8, ...);
}

#[repr(C)] pub struct drm_exec { _private: [u8; 0] }
#[repr(C)] pub struct drm_gem_object { _private: [u8; 0] }
#[repr(C)] pub struct amdgpu_vm { _private: [u8; 0] }
#[repr(C)] pub struct amdgpu_bo_va { _private: [u8; 0] }
#[repr(C)] pub struct amdgpu_bo { pub tbo: amdgpu_tbo }
#[repr(C)] pub struct amdgpu_tbo { pub base: drm_gem_object }
#[repr(C)] pub struct amdgpu_device { pub virt: amdgpu_virt }
#[repr(C)] pub struct amdgpu_virt { pub csa_cpu_addr: *mut c_void }

const PAGE_SIZE: usize = 4096;
const DRM_EXEC_INTERRUPTIBLE_WAIT: u32 = 1;
const AMDGPU_PTE_READABLE: u64 = 1 << 1;
const AMDGPU_PTE_WRITEABLE: u64 = 1 << 2;
const AMDGPU_PTE_EXECUTABLE: u64 = 1 << 3;

pub unsafe fn amdgpu_csa_vaddr(adev: *mut amdgpu_device) -> u64 {
    let addr = AMDGPU_VA_RESERVED_CSA_START(adev);
    amdgpu_gmc_sign_extend(addr)
}

pub unsafe fn amdgpu_allocate_static_csa(
    adev: *mut amdgpu_device, bo: *mut *mut amdgpu_bo, domain: u32, size: u32,
) -> i32 {
    let mut ptr: *mut c_void = core::ptr::null_mut();
    amdgpu_bo_create_kernel(adev, size, PAGE_SIZE, domain, bo, core::ptr::null_mut(), &mut ptr);
    if (*bo).is_null() { return -12; }
    core::ptr::write_bytes(ptr, 0, size as usize);
    (*adev).virt.csa_cpu_addr = ptr;
    0
}

pub unsafe fn amdgpu_free_static_csa(bo: *mut *mut amdgpu_bo) {
    amdgpu_bo_free_kernel(bo, core::ptr::null_mut(), core::ptr::null_mut());
}

// Called during amdgpu_vm_init to map the static CSA into the VM.
pub unsafe fn amdgpu_map_static_csa(
    adev: *mut amdgpu_device, vm: *mut amdgpu_vm, bo: *mut amdgpu_bo,
    bo_va: *mut *mut amdgpu_bo_va, csa_addr: u64, size: u32,
) -> i32 {
    let mut exec = drm_exec { _private: [] };
    let mut r;
    drm_exec_init(&mut exec, DRM_EXEC_INTERRUPTIBLE_WAIT, 0);
    loop {
        r = amdgpu_vm_lock_pd(vm, &mut exec, 0);
        if r == 0 { r = drm_exec_lock_obj(&mut exec, &mut (*bo).tbo.base); }
        if r == 0 { break; }
    }
    if r != 0 { drm_err(adev_to_drm(adev), b"failed to reserve CSA,PD BOs: err=%d\0".as_ptr(), r); drm_exec_fini(&mut exec); return r; }
    *bo_va = amdgpu_vm_bo_add(adev, vm, bo);
    if (*bo_va).is_null() { drm_exec_fini(&mut exec); return -12; }
    r = amdgpu_vm_bo_map(adev, *bo_va, csa_addr, 0, size, AMDGPU_PTE_READABLE | AMDGPU_PTE_WRITEABLE | AMDGPU_PTE_EXECUTABLE);
    if r != 0 { drm_err(adev_to_drm(adev), b"failed to do bo_map on static CSA, err=%d\0".as_ptr(), r); amdgpu_vm_bo_del(adev, *bo_va); }
    drm_exec_fini(&mut exec); r
}

pub unsafe fn amdgpu_unmap_static_csa(
    adev: *mut amdgpu_device, vm: *mut amdgpu_vm, bo: *mut amdgpu_bo,
    bo_va: *mut amdgpu_bo_va, csa_addr: u64,
) -> i32 {
    let mut exec = drm_exec { _private: [] };
    let mut r;
    drm_exec_init(&mut exec, 0, 0);
    loop { r = amdgpu_vm_lock_pd(vm, &mut exec, 0); if r == 0 { r = drm_exec_lock_obj(&mut exec, &mut (*bo).tbo.base); } if r == 0 { break; } }
    if r == 0 { r = amdgpu_vm_bo_unmap(adev, bo_va, csa_addr); }
    if r == 0 { amdgpu_vm_bo_del(adev, bo_va); }
    drm_exec_fini(&mut exec); r
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
