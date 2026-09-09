/* SPDX-License-Identifier: MIT */
/*
 * Copyright 2025 Advanced Micro Devices, Inc.
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
 */

// Dependency declarations supplied by amd_cper.h are external to this file.

pub const CPER_MAX_ALLOWED_COUNT: usize = 0x1000;
pub const CPER_MAX_RING_SIZE: usize = 0x100000;
pub const HDR_LEN: usize = core::mem::size_of::<cper_hdr>();
pub const SEC_DESC_LEN: usize = core::mem::size_of::<cper_sec_desc>();

pub const BOOT_SEC_LEN: usize = core::mem::size_of::<cper_sec_crashdump_boot>();
pub const FATAL_SEC_LEN: usize = core::mem::size_of::<cper_sec_crashdump_fatal>();
pub const NONSTD_SEC_LEN: usize = core::mem::size_of::<cper_sec_nonstd_err>();

#[inline]
pub const fn SEC_DESC_OFFSET(idx: usize) -> usize {
    HDR_LEN + (SEC_DESC_LEN * idx)
}

#[inline]
pub const fn BOOT_SEC_OFFSET(count: usize, idx: usize) -> usize {
    HDR_LEN + (SEC_DESC_LEN * count) + (BOOT_SEC_LEN * idx)
}

#[inline]
pub const fn FATAL_SEC_OFFSET(count: usize, idx: usize) -> usize {
    HDR_LEN + (SEC_DESC_LEN * count) + (FATAL_SEC_LEN * idx)
}

#[inline]
pub const fn NONSTD_SEC_OFFSET(count: usize, idx: usize) -> usize {
    HDR_LEN + (SEC_DESC_LEN * count) + (NONSTD_SEC_LEN * idx)
}

#[repr(C)]
#[derive(Copy, Clone)]
pub enum amdgpu_cper_type {
    AMDGPU_CPER_TYPE_RUNTIME,
    AMDGPU_CPER_TYPE_FATAL,
    AMDGPU_CPER_TYPE_BOOT,
    AMDGPU_CPER_TYPE_BP_THRESHOLD,
}

#[repr(C)]
pub struct amdgpu_cper {
    pub enabled: bool,
    pub unique_id: atomic_t,
    pub cper_lock: mutex,
    pub count: u32,
    pub max_count: u32,
    pub wptr: u32,
    pub ring: [*mut core::ffi::c_void; CPER_MAX_ALLOWED_COUNT],
    pub ring_buf: amdgpu_ring,
    pub ring_lock: mutex,
}

extern "C" {
    pub fn amdgpu_cper_entry_fill_hdr(
        adev: *mut amdgpu_device,
        hdr: *mut cper_hdr,
        type_: amdgpu_cper_type,
        sev: cper_error_severity,
    );
    pub fn amdgpu_cper_entry_fill_fatal_section(
        adev: *mut amdgpu_device,
        hdr: *mut cper_hdr,
        idx: u32,
        reg_data: cper_sec_crashdump_reg_data,
    ) -> i32;
    pub fn amdgpu_cper_entry_fill_runtime_section(
        adev: *mut amdgpu_device,
        hdr: *mut cper_hdr,
        idx: u32,
        sev: cper_error_severity,
        reg_dump: *mut u32,
        reg_count: u32,
    ) -> i32;
    pub fn amdgpu_cper_entry_fill_bad_page_threshold_section(
        adev: *mut amdgpu_device,
        hdr: *mut cper_hdr,
        section_idx: u32,
    ) -> i32;
    pub fn amdgpu_cper_alloc_entry(
        adev: *mut amdgpu_device,
        type_: amdgpu_cper_type,
        section_count: u16,
    ) -> *mut cper_hdr;
    // Bad page threshold is encoded into separated cper entry
    pub fn amdgpu_cper_generate_bp_threshold_record(adev: *mut amdgpu_device) -> i32;
    pub fn amdgpu_cper_ring_write(ring: *mut amdgpu_ring, src: *mut core::ffi::c_void, count: i32);
    pub fn amdgpu_cper_init(adev: *mut amdgpu_device) -> i32;
    pub fn amdgpu_cper_deferred_init(adev: *mut amdgpu_device) -> i32;
    pub fn amdgpu_cper_fini(adev: *mut amdgpu_device) -> i32;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
