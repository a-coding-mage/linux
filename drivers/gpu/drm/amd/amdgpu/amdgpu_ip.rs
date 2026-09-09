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
 * THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
 * IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
 * FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL
 * THE COPYRIGHT HOLDER(S) OR AUTHOR(S) BE LIABLE FOR ANY CLAIM, DAMAGES OR
 * OTHER LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE,
 * ARISING FROM, OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR
 * OTHER DEALINGS IN THE SOFTWARE.
 */

use core::ffi::{c_char, c_int, c_void};

extern "C" {
    fn ffs(i: c_int) -> c_int;
    fn dev_err(dev: *mut c_void, fmt: *const c_char, ...);
    fn dev_info(dev: *mut c_void, fmt: *const c_char, ...);
    fn dev_warn(dev: *mut c_void, fmt: *const c_char, ...);
    fn amdgpu_device_lock_reset_domain(domain: *mut c_void);
    fn amdgpu_device_unlock_reset_domain(domain: *mut c_void);
    fn amdgpu_multi_ring_reset_helper_begin(mask: u32, ring: *mut amdgpu_ring, fence: *mut amdgpu_fence);
    fn amdgpu_multi_ring_reset_helper_end(mask: u32, ring: *mut amdgpu_ring, r: c_int) -> c_int;
}

const EINVAL: c_int = 22;
const EOPNOTSUPP: c_int = 95;
const HWIP_MAX_INSTANCE: usize = 32;

#[inline]
unsafe fn bit(n: u32) -> u32 { 1u32.wrapping_shl(n) }

unsafe fn amdgpu_logical_to_dev_inst(adev: *mut amdgpu_device, block: amd_hw_ip_block_type, inst: i8) -> i8 {
    match block {
        GC_HWIP | SDMA0_HWIP | VCN_HWIP => (*adev).ip_map.dev_inst[block as usize][inst as usize],
        _ => inst,
    }
}

unsafe fn amdgpu_logical_to_dev_mask(adev: *mut amdgpu_device, block: amd_hw_ip_block_type, mut mask: u32) -> u32 {
    let mut dev_mask = 0;
    while mask != 0 {
        let log_inst = (ffs(mask as c_int) - 1) as i8;
        let dev_inst = amdgpu_logical_to_dev_inst(adev, block, log_inst);
        dev_mask |= bit(dev_inst as u32);
        mask &= !bit(log_inst as u32);
    }
    dev_mask
}

unsafe fn amdgpu_populate_ip_map(adev: *mut amdgpu_device, ip_block: u32, mut inst_mask: u32) {
    let mut l = 0usize;
    while inst_mask != 0 {
        let i = (ffs(inst_mask as c_int) - 1) as usize;
        (*adev).ip_map.dev_inst[ip_block as usize][l] = i as i8;
        l += 1;
        inst_mask &= !bit(i as u32);
    }
    while l < HWIP_MAX_INSTANCE {
        (*adev).ip_map.dev_inst[ip_block as usize][l] = -1;
        l += 1;
    }
}

pub unsafe fn amdgpu_ip_map_init(adev: *mut amdgpu_device) {
    let ip_map = [(GC_HWIP as u32, (*adev).gfx.xcc_mask),
                  (SDMA0_HWIP as u32, (*adev).sdma.sdma_mask),
                  (VCN_HWIP as u32, (*adev).vcn.inst_mask)];
    for (block, mask) in ip_map { amdgpu_populate_ip_map(adev, block, mask); }
    (*adev).ip_map.logical_to_dev_inst = Some(amdgpu_logical_to_dev_inst);
    (*adev).ip_map.logical_to_dev_mask = Some(amdgpu_logical_to_dev_mask);
}

pub unsafe fn amdgpu_ip_block_suspend(ip_block: *mut amdgpu_ip_block) -> c_int {
    if let Some(suspend) = (*(*(*ip_block).version).funcs).suspend {
        let r = suspend(ip_block);
        if r != 0 { dev_err((*ip_block).adev.as_ref().unwrap().dev, b"suspend of IP block <%s> failed %d\0".as_ptr() as _, (*(*(*ip_block).version).funcs).name, r); return r; }
    }
    (*ip_block).status.hw = false; 0
}

pub unsafe fn amdgpu_ip_block_resume(ip_block: *mut amdgpu_ip_block) -> c_int {
    if let Some(resume) = (*(*(*ip_block).version).funcs).resume {
        let r = resume(ip_block);
        if r != 0 { dev_err((*ip_block).adev.as_ref().unwrap().dev, b"resume of IP block <%s> failed %d\0".as_ptr() as _, (*(*(*ip_block).version).funcs).name, r); return r; }
    }
    (*ip_block).status.hw = true; 0
}

pub unsafe fn amdgpu_device_ip_get_ip_block(adev: *mut amdgpu_device, ty: amd_ip_block_type) -> *mut amdgpu_ip_block {
    for i in 0..(*adev).num_ip_blocks as usize { if (*adev).ip_blocks[i].version.as_ref().unwrap().ty == ty { return &mut (*adev).ip_blocks[i]; } }
    core::ptr::null_mut()
}

pub unsafe fn amdgpu_device_ip_block_version_cmp(adev: *mut amdgpu_device, ty: amd_ip_block_type, major: u32, minor: u32) -> c_int {
    let b = amdgpu_device_ip_get_ip_block(adev, ty);
    if !b.is_null() && ((*(*b).version).major > major || ((*(*b).version).major == major && (*(*b).version).minor >= minor)) { 0 } else { 1 }
}

static IP_BLOCK_NAMES: [&[u8]; 18] = [b"common\0", b"gmc\0", b"ih\0", b"smu\0", b"psp\0", b"dce\0", b"gfx\0", b"sdma\0", b"uvd\0", b"vce\0", b"acp\0", b"vcn\0", b"mes\0", b"jpeg\0", b"vpe\0", b"umsch_mm\0", b"isp\0", b"ras\0"];

unsafe fn ip_block_name(_adev: *mut amdgpu_device, ty: amd_ip_block_type) -> *const c_char {
    if (ty as usize) < IP_BLOCK_NAMES.len() { IP_BLOCK_NAMES[ty as usize].as_ptr() as _ } else { b"unknown\0".as_ptr() as _ }
}

pub unsafe fn amdgpu_device_ip_block_add(adev: *mut amdgpu_device, v: *const amdgpu_ip_block_version) -> c_int {
    if v.is_null() { return -EINVAL; }
    match (*v).ty { AMD_IP_BLOCK_TYPE_VCN if (*adev).harvest_ip_mask & AMD_HARVEST_IP_VCN_MASK != 0 => return 0,
        AMD_IP_BLOCK_TYPE_JPEG if (*adev).harvest_ip_mask & AMD_HARVEST_IP_JPEG_MASK != 0 => return 0, _ => {} }
    (*adev).ip_blocks[(*adev).num_ip_blocks as usize].adev = adev;
    (*adev).ip_blocks[(*adev).num_ip_blocks as usize].version = Some(v);
    (*adev).num_ip_blocks += 1; 0
}

pub unsafe fn amdgpu_device_ip_set_clockgating_state(adev: *mut amdgpu_device, ty: amd_ip_block_type, state: amd_clockgating_state) -> c_int { let mut r=0; for i in 0..(*adev).num_ip_blocks as usize { let b=&mut (*adev).ip_blocks[i]; if b.status.valid && b.version.as_ref().unwrap().ty==ty { if let Some(f)=b.version.as_ref().unwrap().funcs.as_ref().unwrap().set_clockgating_state { r=f(b,state); } } } r }
pub unsafe fn amdgpu_device_ip_set_powergating_state(adev: *mut amdgpu_device, ty: amd_ip_block_type, state: amd_powergating_state) -> c_int { let mut r=0; for i in 0..(*adev).num_ip_blocks as usize { let b=&mut (*adev).ip_blocks[i]; if b.status.valid && b.version.as_ref().unwrap().ty==ty { if let Some(f)=b.version.as_ref().unwrap().funcs.as_ref().unwrap().set_powergating_state { r=f(b,state); } } } r }
pub unsafe fn amdgpu_device_ip_get_clockgating_state(adev: *mut amdgpu_device, flags: *mut u64) { for i in 0..(*adev).num_ip_blocks as usize { let b=&mut (*adev).ip_blocks[i]; if b.status.valid { if let Some(f)=b.version.as_ref().unwrap().funcs.as_ref().unwrap().get_clockgating_state { f(b,flags); } } } }
pub unsafe fn amdgpu_device_ip_wait_for_idle(adev: *mut amdgpu_device, ty: amd_ip_block_type) -> c_int { let b=amdgpu_device_ip_get_ip_block(adev,ty); if b.is_null() || !(*b).status.valid {0} else if let Some(f)=(*(*b).version).funcs.as_ref().unwrap().wait_for_idle {f(b)} else {0} }
pub unsafe fn amdgpu_device_ip_is_valid(adev: *mut amdgpu_device, ty: amd_ip_block_type) -> bool { let b=amdgpu_device_ip_get_ip_block(adev,ty); !b.is_null() && (*b).status.valid }

unsafe fn amdgpu_ip_from_ring(t: amdgpu_ring_type) -> amd_ip_block_type { match t { AMDGPU_RING_TYPE_GFX|AMDGPU_RING_TYPE_COMPUTE=>AMD_IP_BLOCK_TYPE_GFX, AMDGPU_RING_TYPE_SDMA=>AMD_IP_BLOCK_TYPE_SDMA, AMDGPU_RING_TYPE_UVD|AMDGPU_RING_TYPE_UVD_ENC=>AMD_IP_BLOCK_TYPE_UVD, AMDGPU_RING_TYPE_VCE=>AMD_IP_BLOCK_TYPE_VCE, AMDGPU_RING_TYPE_VCN_DEC|AMDGPU_RING_TYPE_VCN_ENC=>AMD_IP_BLOCK_TYPE_VCN, AMDGPU_RING_TYPE_VCN_JPEG=>AMD_IP_BLOCK_TYPE_JPEG, AMDGPU_RING_TYPE_VPE=>AMD_IP_BLOCK_TYPE_VPE, _=>AMD_IP_BLOCK_TYPE_NUM } }
unsafe fn amdgpu_ring_mask_from_ip(t: amd_ip_block_type) -> u32 { match t { AMD_IP_BLOCK_TYPE_GFX=>bit(AMDGPU_RING_TYPE_GFX as u32)|bit(AMDGPU_RING_TYPE_COMPUTE as u32), AMD_IP_BLOCK_TYPE_SDMA=>bit(AMDGPU_RING_TYPE_SDMA as u32), AMD_IP_BLOCK_TYPE_UVD=>bit(AMDGPU_RING_TYPE_UVD as u32)|bit(AMDGPU_RING_TYPE_UVD_ENC as u32), AMD_IP_BLOCK_TYPE_VCE=>bit(AMD_IP_BLOCK_TYPE_VCE as u32), AMD_IP_BLOCK_TYPE_VCN=>bit(AMDGPU_RING_TYPE_VCN_DEC as u32)|bit(AMDGPU_RING_TYPE_VCN_ENC as u32), AMD_IP_BLOCK_TYPE_JPEG=>bit(AMDGPU_RING_TYPE_VCN_JPEG as u32), AMD_IP_BLOCK_TYPE_VPE=>bit(AMDGPU_RING_TYPE_VPE as u32), _=>0 } }

pub unsafe fn amdgpu_device_ip_soft_reset(guilty_ring: *mut amdgpu_ring, guilty_fence: *mut amdgpu_fence) -> c_int { let adev=(*guilty_ring).adev; let ty=amdgpu_ip_from_ring((*(*guilty_ring).funcs).ty); let b=amdgpu_device_ip_get_ip_block(adev,ty); if b.is_null() || (*(*b).version).funcs.as_ref().unwrap().soft_reset.is_none() { return -EOPNOTSUPP; } let mask=amdgpu_ring_mask_from_ip(ty); amdgpu_device_lock_reset_domain((*adev).reset_domain); amdgpu_multi_ring_reset_helper_begin(mask,guilty_ring,guilty_fence); let r=((*(*b).version).funcs.as_ref().unwrap().soft_reset.unwrap())(b); let r=amdgpu_multi_ring_reset_helper_end(mask,guilty_ring,r); amdgpu_device_unlock_reset_domain((*adev).reset_domain); r }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
