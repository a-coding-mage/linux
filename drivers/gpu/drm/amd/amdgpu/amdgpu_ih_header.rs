/*
 * Copyright 2014 Advanced Micro Devices, Inc.
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

/* Maximum number of IVs processed at once */
pub const AMDGPU_IH_MAX_NUM_IVS: usize = 32;
pub const IH_RING_SIZE: usize = 256 * 1024;
pub const IH_SW_RING_SIZE: usize = 16 * 1024; /* enough for 512 CAM entries */

#[repr(C)]
pub struct amdgpu_device;
#[repr(C)]
pub struct amdgpu_iv_entry;
#[repr(C)]
pub struct amdgpu_bo;

#[repr(C)]
pub struct amdgpu_ih_regs {
    pub ih_rb_base: u32,
    pub ih_rb_base_hi: u32,
    pub ih_rb_cntl: u32,
    pub ih_rb_wptr: u32,
    pub ih_rb_rptr: u32,
    pub ih_doorbell_rptr: u32,
    pub ih_rb_wptr_addr_lo: u32,
    pub ih_rb_wptr_addr_hi: u32,
    pub psp_reg_id: u32,
}

/* R6xx+ IH ring */
#[repr(C)]
pub struct amdgpu_ih_ring {
    pub ring_size: usize,
    pub ptr_mask: u32,
    pub doorbell_index: u32,
    pub use_doorbell: bool,
    pub use_bus_addr: bool,
    pub ring_obj: *mut amdgpu_bo,
    pub ring: *mut u32,
    pub gpu_addr: u64,
    pub wptr_addr: u64,
    pub wptr_cpu: *mut u32,
    pub rptr_addr: u64,
    pub rptr_cpu: *mut u32,
    pub enabled: bool,
    pub rptr: usize,
    pub ih_regs: amdgpu_ih_regs,
    pub wait_process: wait_queue_head_t,
    pub processed_timestamp: u64,
    pub overflow: bool,
}

/* return true if time stamp t2 is after t1 with 48bit wrap around */
#[inline]
pub const fn amdgpu_ih_ts_after(t1: i64, t2: i64) -> bool {
    ((t2 << 16).wrapping_sub(t1 << 16)) > 0
}

#[inline]
pub const fn amdgpu_ih_ts_after_or_equal(t1: i64, t2: i64) -> bool {
    ((t2 << 16).wrapping_sub(t1 << 16)) >= 0
}

/* provided by the ih block */
#[repr(C)]
pub struct amdgpu_ih_funcs {
    /* ring read/write ptr handling, called from interrupt context */
    pub get_wptr: Option<unsafe extern "C" fn(*mut amdgpu_device, *mut amdgpu_ih_ring) -> u32>,
    pub decode_iv: Option<unsafe extern "C" fn(*mut amdgpu_device, *mut amdgpu_ih_ring, *mut amdgpu_iv_entry)>,
    pub decode_iv_ts: Option<unsafe extern "C" fn(*mut amdgpu_ih_ring, u32, i32) -> u64>,
    pub set_rptr: Option<unsafe extern "C" fn(*mut amdgpu_device, *mut amdgpu_ih_ring)>,
    /* Decode IH cookie node_id into a human-readable die name string.
     * Returns buf, or NULL if this IH version does not support node_id decoding.
     */
    pub node_id_to_die_name: Option<unsafe extern "C" fn(*mut amdgpu_device, u32, *mut i8, usize) -> *const i8>,
}

#[macro_export]
macro_rules! amdgpu_ih_get_wptr { ($adev:expr, $ih:expr) => { (*$adev).irq.ih_funcs.get_wptr.unwrap()($adev, $ih) }; }
#[macro_export]
macro_rules! amdgpu_ih_decode_iv { ($adev:expr, $ih:expr, $iv:expr) => { (*$adev).irq.ih_funcs.decode_iv.unwrap()($adev, $ih, $iv) }; }
#[macro_export]
macro_rules! amdgpu_ih_decode_iv_ts { ($adev:expr, $ih:expr, $rptr:expr, $offset:expr) => { (*$adev).irq.ih_funcs.decode_iv_ts.map(|f| f($ih, $rptr, $offset)).unwrap_or(0) }; }
#[macro_export]
macro_rules! amdgpu_ih_set_rptr { ($adev:expr, $ih:expr) => { (*$adev).irq.ih_funcs.set_rptr.unwrap()($adev, $ih) }; }

extern "C" {
    pub fn amdgpu_ih_ring_init(adev: *mut amdgpu_device, ih: *mut amdgpu_ih_ring, ring_size: u32, use_bus_addr: bool) -> i32;
    pub fn amdgpu_ih_ring_fini(adev: *mut amdgpu_device, ih: *mut amdgpu_ih_ring);
    pub fn amdgpu_ih_ring_write(adev: *mut amdgpu_device, ih: *mut amdgpu_ih_ring, iv: *const u32, num_dw: u32);
    pub fn amdgpu_ih_wait_on_checkpoint_process_ts(adev: *mut amdgpu_device, ih: *mut amdgpu_ih_ring) -> i32;
    pub fn amdgpu_ih_process(adev: *mut amdgpu_device, ih: *mut amdgpu_ih_ring) -> i32;
    pub fn amdgpu_ih_decode_iv_helper(adev: *mut amdgpu_device, ih: *mut amdgpu_ih_ring, entry: *mut amdgpu_iv_entry);
    pub fn amdgpu_ih_decode_iv_ts_helper(ih: *mut amdgpu_ih_ring, rptr: u32, offset: i32) -> u64;
    pub fn amdgpu_ih_ring_name(adev: *mut amdgpu_device, ih: *mut amdgpu_ih_ring) -> *const i8;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
