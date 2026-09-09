/* SPDX-License-Identifier: MIT */
/*
 * Copyright 2024 Advanced Micro Devices, Inc.
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

// The C header guard is omitted in Rust; module inclusion provides equivalent protection.
// Dependency: declarations supplied by amdgpu.h.

// C: #ifdef CONFIG_DEV_COREDUMP
#[cfg(feature = "CONFIG_DEV_COREDUMP")]
pub const AMDGPU_COREDUMP_VERSION: &str = "1";

#[cfg(feature = "CONFIG_DEV_COREDUMP")]
#[repr(C)]
pub struct amdgpu_coredump_ring {
	pub rptr: u64,
	pub wptr: u64,
	pub ring_dw: *mut u32,
	pub ring_index: u32,
}

#[cfg(feature = "CONFIG_DEV_COREDUMP")]
#[repr(C)]
pub struct amdgpu_coredump_ib_info {
	pub gpu_addr: u64,
	pub ib_size_dw: u32,
}

#[cfg(feature = "CONFIG_DEV_COREDUMP")]
#[repr(C)]
pub struct amdgpu_coredump_info {
	pub adev: *mut amdgpu_device,
	pub reset_task_info: amdgpu_task_info,
	pub reset_time: timespec64,

	pub skip_vram_check: bool,
	pub reset_vram_lost: bool,
	pub ring: *mut amdgpu_ring,

	pub rings: *mut amdgpu_coredump_ring,
	pub num_rings: u32,

	/* Readable form of coredevdump, generate once to speed up
	 * reading it (see drm_coredump_printer's documentation).
	 */
	pub formatted_size: ssize_t,
	pub formatted: *mut core::ffi::c_char,

	pub pasid: core::ffi::c_uint,
	pub vmid: core::ffi::c_uint,
	pub num_ibs: core::ffi::c_int,
	// C flexible array member: ibs[] __counted_by(num_ibs).
	pub ibs: [amdgpu_coredump_ib_info; 0],
}

extern "C" {
	pub fn amdgpu_coredump(
		adev: *mut amdgpu_device,
		skip_vram_check: bool,
		vram_lost: bool,
		job: *mut amdgpu_job,
	);
	pub fn amdgpu_coredump_init(adev: *mut amdgpu_device);
	pub fn amdgpu_coredump_fini(adev: *mut amdgpu_device);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
