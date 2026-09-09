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

// Dependencies supplied by amdgpu.h, amdgpu_ras.h, and linux/kfd_ioctl.h.

#[repr(C)]
pub struct amdgpu_device {
    pub hdp: amdgpu_hdp,
    pub rmmio_remap: amdgpu_rmmio_remap,
    pub nbio: amdgpu_nbio,
    pub asic_funcs: *mut amdgpu_asic_funcs,
    pub dev: *mut core::ffi::c_void,
}

#[repr(C)]
pub struct amdgpu_hdp {
    pub ras: *mut amdgpu_hdp_ras,
    pub ras_if: *mut amdgpu_ras_comm,
    pub funcs: *mut amdgpu_hdp_funcs,
}

#[repr(C)]
pub struct amdgpu_hdp_ras { pub ras_block: amdgpu_ras_block }
#[repr(C)]
pub struct amdgpu_ras_block { pub ras_comm: amdgpu_ras_comm }
#[repr(C)]
pub struct amdgpu_ras_comm { pub name: [u8; 32], pub block: i32, pub type_: i32 }
#[repr(C)]
pub struct amdgpu_rmmio_remap { pub reg_offset: u32 }
#[repr(C)]
pub struct amdgpu_nbio { pub funcs: *mut amdgpu_nbio_funcs }
#[repr(C)]
pub struct amdgpu_nbio_funcs { pub get_memsize: Option<unsafe extern "C" fn(*mut amdgpu_device)> }
#[repr(C)]
pub struct amdgpu_hdp_funcs {
    pub invalidate_hdp: Option<unsafe extern "C" fn(*mut amdgpu_device, *mut amdgpu_ring)>,
    pub flush_hdp: Option<unsafe extern "C" fn(*mut amdgpu_device, *mut amdgpu_ring)>,
}
#[repr(C)]
pub struct amdgpu_asic_funcs {
    pub invalidate_hdp: Option<unsafe extern "C" fn(*mut amdgpu_device, *mut amdgpu_ring)>,
    pub flush_hdp: Option<unsafe extern "C" fn(*mut amdgpu_device, *mut amdgpu_ring)>,
}
#[repr(C)]
pub struct amdgpu_ring { pub funcs: *mut amdgpu_ring_funcs }
#[repr(C)]
pub struct amdgpu_ring_funcs { pub emit_wreg: Option<unsafe extern "C" fn()> }

pub const AMDGPU_RAS_BLOCK__HDP: i32 = 0;
pub const AMDGPU_RAS_ERROR__MULTI_UNCORRECTABLE: i32 = 0;
pub const KFD_MMIO_REMAP_HDP_MEM_FLUSH_CNTL: u32 = 0;

unsafe extern "C" {
    fn amdgpu_ras_register_ras_block(adev: *mut amdgpu_device, block: *mut amdgpu_ras_block) -> i32;
    fn dev_err(dev: *mut core::ffi::c_void, fmt: *const core::ffi::c_char, ...);
    fn WREG32(reg: u32, value: u32);
    fn amdgpu_ring_emit_wreg(ring: *mut amdgpu_ring, reg: u32, value: u32);
}

pub unsafe extern "C" fn amdgpu_hdp_ras_sw_init(adev: *mut amdgpu_device) -> i32 {
    let mut err: i32;
    let ras: *mut amdgpu_hdp_ras;

    if (*adev).hdp.ras.is_null() { return 0; }

    ras = (*adev).hdp.ras;
    err = amdgpu_ras_register_ras_block(adev, &mut (*ras).ras_block);
    if err != 0 {
        dev_err((*adev).dev, b"Failed to register hdp ras block!\0".as_ptr() as *const _,);
        return err;
    }

    (*ras).ras_block.ras_comm.name[..4].copy_from_slice(b"hdp\0");
    (*ras).ras_block.ras_comm.block = AMDGPU_RAS_BLOCK__HDP;
    (*ras).ras_block.ras_comm.type_ = AMDGPU_RAS_ERROR__MULTI_UNCORRECTABLE;
    (*adev).hdp.ras_if = &mut (*ras).ras_block.ras_comm;

    /* hdp ras follows amdgpu_ras_block_late_init_default for late init */
    0
}

pub unsafe extern "C" fn amdgpu_hdp_generic_flush(adev: *mut amdgpu_device, ring: *mut amdgpu_ring) {
    if ring.is_null() || (*ring).funcs.is_null() || (*(*ring).funcs).emit_wreg.is_none() {
        WREG32(((*adev).rmmio_remap.reg_offset + KFD_MMIO_REMAP_HDP_MEM_FLUSH_CNTL) >> 2, 0);
        if !(*adev).nbio.funcs.is_null() {
            if let Some(get_memsize) = (*(*adev).nbio.funcs).get_memsize { get_memsize(adev); }
        }
    } else {
        amdgpu_ring_emit_wreg(ring, ((*adev).rmmio_remap.reg_offset + KFD_MMIO_REMAP_HDP_MEM_FLUSH_CNTL) >> 2, 0);
    }
}

pub unsafe extern "C" fn amdgpu_hdp_invalidate(adev: *mut amdgpu_device, ring: *mut amdgpu_ring) {
    if !(*adev).asic_funcs.is_null() {
        if let Some(f) = (*(*adev).asic_funcs).invalidate_hdp { f(adev, ring); }
    } else if !(*adev).hdp.funcs.is_null() {
        if let Some(f) = (*(*adev).hdp.funcs).invalidate_hdp { f(adev, ring); }
    }
}

pub unsafe extern "C" fn amdgpu_hdp_flush(adev: *mut amdgpu_device, ring: *mut amdgpu_ring) {
    if !(*adev).asic_funcs.is_null() {
        if let Some(f) = (*(*adev).asic_funcs).flush_hdp { f(adev, ring); }
    } else if !(*adev).hdp.funcs.is_null() {
        if let Some(f) = (*(*adev).hdp.funcs).flush_hdp { f(adev, ring); }
    }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
