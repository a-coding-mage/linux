/*
 * Copyright 2017 Advanced Micro Devices, Inc.
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
 */

// Dependency supplied externally: linux PAGE_SIZE and dma_addr_t.

/* GART structures, functions & helpers */
#[repr(C)]
pub struct amdgpu_device {
    _private: [u8; 0],
}

#[repr(C)]
pub struct amdgpu_bo {
    _private: [u8; 0],
}

pub const AMDGPU_GPU_PAGE_SIZE: u64 = 4096;
pub const AMDGPU_GPU_PAGE_MASK: u64 = AMDGPU_GPU_PAGE_SIZE - 1;
pub const AMDGPU_GPU_PAGE_SHIFT: u32 = 12;

#[macro_export]
macro_rules! AMDGPU_GPU_PAGE_ALIGN {
    ($a:expr) => {
        (($a + $crate::AMDGPU_GPU_PAGE_MASK) & !$crate::AMDGPU_GPU_PAGE_MASK)
    };
}

pub const AMDGPU_GPU_PAGES_IN_CPU_PAGE: usize =
    PAGE_SIZE / AMDGPU_GPU_PAGE_SIZE as usize;

#[repr(C)]
pub struct amdgpu_gart {
    pub bo: *mut amdgpu_bo,
    /* CPU kmapped address of gart table */
    pub ptr: *mut core::ffi::c_void,
    pub num_gpu_pages: u32,
    pub num_cpu_pages: u32,
    pub table_size: u32,

    /* Asic default pte flags */
    pub gart_pte_flags: u64,
}

extern "C" {
    pub fn amdgpu_gart_table_ram_alloc(adev: *mut amdgpu_device) -> i32;
    pub fn amdgpu_gart_table_ram_free(adev: *mut amdgpu_device);
    pub fn amdgpu_gart_table_vram_alloc(adev: *mut amdgpu_device) -> i32;
    pub fn amdgpu_gart_table_vram_free(adev: *mut amdgpu_device);
    pub fn amdgpu_gart_init(adev: *mut amdgpu_device) -> i32;
    pub fn amdgpu_gart_dummy_page_fini(adev: *mut amdgpu_device);
    pub fn amdgpu_gart_unbind(adev: *mut amdgpu_device, offset: u64, pages: i32);
    pub fn amdgpu_gart_map(
        adev: *mut amdgpu_device,
        offset: u64,
        pages: i32,
        dma_addr: *mut dma_addr_t,
        flags: u64,
        dst: *mut core::ffi::c_void,
    );
    pub fn amdgpu_gart_map_gfx9_mqd(
        adev: *mut amdgpu_device,
        offset: u64,
        pages: i32,
        dma_addr: *mut dma_addr_t,
        flags: u64,
    );
    pub fn amdgpu_gart_bind(
        adev: *mut amdgpu_device,
        offset: u64,
        pages: i32,
        dma_addr: *mut dma_addr_t,
        flags: u64,
    );
    pub fn amdgpu_gart_map_vram_range(
        adev: *mut amdgpu_device,
        pa: u64,
        start_page: u64,
        num_pages: u64,
        flags: u64,
        dst: *mut core::ffi::c_void,
    );
    pub fn amdgpu_gart_invalidate_tlb(adev: *mut amdgpu_device);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
