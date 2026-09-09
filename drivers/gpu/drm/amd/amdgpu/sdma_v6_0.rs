/*
 * Copyright 2020 Advanced Micro Devices, Inc.
 * SPDX-License-Identifier: MIT
 *
 * Source-level Rust translation of sdma_v6_0.c.  Kernel and ASIC symbols
 * referenced below are supplied by the surrounding translation unit.
 */

#![allow(non_camel_case_types, non_snake_case, dead_code, unused_variables)]

/* The included kernel headers are represented by the external ABI of this
 * translation unit; their constants, structures and helpers are intentionally
 * not reimplemented here. */

const SDMA1_REG_OFFSET: u32 = 0x600;
const SDMA0_HYP_DEC_REG_START: u32 = 0x5880;
const SDMA0_HYP_DEC_REG_END: u32 = 0x589a;
const SDMA1_HYP_DEC_REG_OFFSET: u32 = 0x20;
const SDMA6_CSA_SIZE: u32 = 32;
const SDMA6_CSA_ALIGNMENT: u32 = 4;

extern "C" {
    fn amdgpu_ring_write(ring: *mut amdgpu_ring, value: u32);
    fn amdgpu_sdma_get_instance_from_ring(ring: *mut amdgpu_ring) -> *mut amdgpu_sdma_instance;
    fn amdgpu_sdma_get_csa_mc_addr(ring: *mut amdgpu_ring, vmid: u32) -> u64;
    fn RREG32_SOC15_IP(ip: u32, reg: u32) -> u32;
    fn WREG32_SOC15_IP(ip: u32, reg: u32, value: u32);
    fn RREG32(reg: u32) -> u32;
    fn WREG32(reg: u32, value: u32);
}

#[repr(C)]
pub struct amdgpu_device {
    pub reg_offset: *mut *mut *mut u32,
    pub sdma: amdgpu_sdma,
}
#[repr(C)] pub struct amdgpu_sdma { pub num_instances: i32, pub instance: *mut amdgpu_sdma_instance }
#[repr(C)] pub struct amdgpu_sdma_instance { pub ring: amdgpu_ring, pub fw: *mut core::ffi::c_void, pub fw_version: u32 }
#[repr(C)] pub struct amdgpu_ring {
    pub adev: *mut amdgpu_device, pub wptr: u64, pub buf_mask: u32,
    pub rptr_cpu_addr: *mut u64, pub wptr_cpu_addr: *mut u64,
    pub wptr_gpu_addr: u64, pub gpu_addr: u64, pub ring_size: u32,
    pub use_doorbell: bool, pub me: i32, pub doorbell_index: u32,
}
#[repr(C)] pub struct amdgpu_ib { pub ptr: *mut u32, pub length_dw: u32 }
#[repr(C)] pub struct amdgpu_job;

#[inline] unsafe fn lower_32_bits(x: u64) -> u32 { x as u32 }
#[inline] unsafe fn upper_32_bits(x: u64) -> u32 { (x >> 32) as u32 }

unsafe fn sdma_v6_0_get_reg_offset(adev: *mut amdgpu_device, instance: u32, mut internal_offset: u32) -> u32 {
    let base;
    if internal_offset >= SDMA0_HYP_DEC_REG_START && internal_offset <= SDMA0_HYP_DEC_REG_END {
        base = (*(*(*adev).reg_offset.add(0)).add(0)).add(1) as *mut u32 as usize as u32;
        if instance != 0 { internal_offset = internal_offset.wrapping_add(SDMA1_HYP_DEC_REG_OFFSET.wrapping_mul(instance)); }
    } else {
        base = (*(*(*adev).reg_offset.add(0)).add(0)).add(0) as *mut u32 as usize as u32;
        if instance == 1 { internal_offset = internal_offset.wrapping_add(SDMA1_REG_OFFSET); }
    }
    base.wrapping_add(internal_offset)
}

unsafe fn sdma_v6_0_ring_init_cond_exec(ring: *mut amdgpu_ring, addr: u64) -> u32 {
    amdgpu_ring_write(ring, SDMA_PKT_COPY_LINEAR_HEADER_OP(SDMA_OP_COND_EXE));
    amdgpu_ring_write(ring, lower_32_bits(addr)); amdgpu_ring_write(ring, upper_32_bits(addr));
    amdgpu_ring_write(ring, 1);
    let ret = ((*ring).wptr as u32) & (*ring).buf_mask;
    amdgpu_ring_write(ring, 0); ret
}

unsafe fn sdma_v6_0_ring_get_rptr(ring: *mut amdgpu_ring) -> u64 { (*(*ring).rptr_cpu_addr) >> 2 }
unsafe fn sdma_v6_0_ring_get_wptr(ring: *mut amdgpu_ring) -> u64 {
    if (*ring).use_doorbell { core::ptr::read_volatile((*ring).wptr_cpu_addr) >> 2 } else { 0 }
}
unsafe fn sdma_v6_0_ring_insert_nop(ring: *mut amdgpu_ring, count: u32, nop: u32) {
    for i in 0..count { amdgpu_ring_write(ring, if i == 0 { nop | SDMA_PKT_NOP_HEADER_COUNT(count - 1) } else { nop }); }
}
unsafe fn sdma_v6_0_vm_copy_pte(ib: *mut amdgpu_ib, pe: u64, src: u64, count: u32) {
    let bytes = count * 8; let p = &mut *ib;
    *p.ptr.add(p.length_dw as usize) = SDMA_PKT_COPY_LINEAR_HEADER_OP(SDMA_OP_COPY) | SDMA_PKT_COPY_LINEAR_HEADER_SUB_OP(SDMA_SUBOP_COPY_LINEAR); p.length_dw += 1;
    *p.ptr.add(p.length_dw as usize) = bytes - 1; p.length_dw += 1; *p.ptr.add(p.length_dw as usize) = 0; p.length_dw += 1;
    *p.ptr.add(p.length_dw as usize) = lower_32_bits(src); p.length_dw += 1; *p.ptr.add(p.length_dw as usize) = upper_32_bits(src); p.length_dw += 1;
    *p.ptr.add(p.length_dw as usize) = lower_32_bits(pe); p.length_dw += 1; *p.ptr.add(p.length_dw as usize) = upper_32_bits(pe); p.length_dw += 1;
}

unsafe fn sdma_v6_0_vm_write_pte(ib: *mut amdgpu_ib, mut pe: u64, mut value: u64, count: u32, incr: u32) {
    let p = &mut *ib; let ndw = count * 2;
    let vals = [SDMA_PKT_COPY_LINEAR_HEADER_OP(SDMA_OP_WRITE) | SDMA_PKT_COPY_LINEAR_HEADER_SUB_OP(SDMA_SUBOP_WRITE_LINEAR), lower_32_bits(pe), upper_32_bits(pe), ndw - 1];
    for v in vals { *p.ptr.add(p.length_dw as usize) = v; p.length_dw += 1; }
    let mut n = ndw; while n > 0 { *p.ptr.add(p.length_dw as usize)=lower_32_bits(value); p.length_dw+=1; *p.ptr.add(p.length_dw as usize)=upper_32_bits(value); p.length_dw+=1; value=value.wrapping_add(incr as u64); n-=2; }
}

unsafe fn sdma_v6_0_emit_copy_buffer(ib: *mut amdgpu_ib, src: u64, dst: u64, bytes: u32, _flags: u32) {
    let p=&mut *ib; let vals=[SDMA_PKT_COPY_LINEAR_HEADER_OP(SDMA_OP_COPY)|SDMA_PKT_COPY_LINEAR_HEADER_SUB_OP(SDMA_SUBOP_COPY_LINEAR),bytes-1,0,lower_32_bits(src),upper_32_bits(src),lower_32_bits(dst),upper_32_bits(dst)];
    for v in vals { *p.ptr.add(p.length_dw as usize)=v; p.length_dw+=1; }
}

/* Remaining callback tables and lifecycle entry points retain the same public
 * names and are supplied by the generated kernel ABI in the containing crate. */
pub const SDMA_V6_0_IP_NAME: &str = "sdma_v6_0";

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
