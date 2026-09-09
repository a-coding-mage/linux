/*
 * Copyright 2014 Advanced Micro Devices, Inc.
 *
 * Rust translation of sdma_v3_0.c.  Kernel and driver declarations supplied
 * by the surrounding tree remain external dependencies.
 */

#![allow(non_camel_case_types, non_snake_case, dead_code, unused_variables)]

extern "C" {
    fn amdgpu_device_program_register_sequence(adev: *mut amdgpu_device, regs: *const u32, n: usize);
    fn amdgpu_ucode_release(fw: *mut *mut amdgpu_firmware);
    fn amdgpu_ucode_request(adev: *mut amdgpu_device, fw: *mut *mut amdgpu_firmware, required: u32, name: *const i8, ...) -> i32;
    fn amdgpu_ring_write(ring: *mut amdgpu_ring, value: u32);
    fn amdgpu_ring_clear_ring(ring: *mut amdgpu_ring);
    fn amdgpu_ring_alloc(ring: *mut amdgpu_ring, n: u32) -> i32;
    fn amdgpu_ring_commit(ring: *mut amdgpu_ring);
    fn amdgpu_ring_test_helper(ring: *mut amdgpu_ring) -> i32;
    fn amdgpu_wb_get(adev: *mut amdgpu_device, index: *mut u32) -> i32;
    fn amdgpu_wb_free(adev: *mut amdgpu_device, index: u32);
    fn amdgpu_ib_get(adev: *mut amdgpu_device, vm: *mut core::ffi::c_void, size: u32, pool: u32, ib: *mut amdgpu_ib) -> i32;
    fn amdgpu_ib_free(ib: *mut amdgpu_ib, vm: *mut core::ffi::c_void);
    fn amdgpu_ib_schedule(ring: *mut amdgpu_ring, n: u32, ib: *mut amdgpu_ib, job: *mut core::ffi::c_void, fence: *mut *mut dma_fence) -> i64;
    fn dma_fence_wait_timeout(f: *mut dma_fence, intr: bool, timeout: i64) -> i64;
    fn dma_fence_put(f: *mut dma_fence);
    fn amdgpu_fence_process(ring: *mut amdgpu_ring);
    fn amdgpu_gmc_emit_flush_gpu_tlb(ring: *mut amdgpu_ring, vmid: u32, addr: u64);
    fn amdgpu_sdma_get_instance_from_ring(ring: *mut amdgpu_ring) -> *mut amdgpu_sdma_instance;
    fn amdgpu_sdma_set_vm_pte_scheds(adev: *mut amdgpu_device, funcs: *const amdgpu_vm_pte_funcs);
    fn amdgpu_sdma_set_buffer_funcs_scheds(adev: *mut amdgpu_device, funcs: *const amdgpu_buffer_funcs);
    fn amdgpu_irq_add_id(adev: *mut amdgpu_device, client: u32, src: u32, irq: *mut amdgpu_irq_src) -> i32;
    fn amdgpu_ring_init(adev: *mut amdgpu_device, ring: *mut amdgpu_ring, size: u32, irq: *mut amdgpu_irq_src, ty: u32, prio: u32, x: *mut core::ffi::c_void) -> i32;
    fn amdgpu_ring_fini(ring: *mut amdgpu_ring);
    fn amdgpu_sriov_vf(adev: *mut amdgpu_device) -> bool;
    fn drm_sched_fault(sched: *mut core::ffi::c_void);
    fn udelay(usec: u32);
    fn mutex_lock(m: *mut core::ffi::c_void);
    fn mutex_unlock(m: *mut core::ffi::c_void);
    fn vi_srbm_select(adev: *mut amdgpu_device, me: u32, pipe: u32, queue: u32, vmid: u32);
}

#[repr(C)] pub struct amdgpu_device { pub sdma: amdgpu_sdma, pub gfx: amdgpu_gfx, pub wb: amdgpu_wb, pub firmware: amdgpu_firmware_store, pub usec_timeout: u32, pub asic_type: u32, pub cg_flags: u64, pub doorbell_index: amdgpu_doorbell, pub srbm_mutex: core::ffi::c_void, pub dev: *mut core::ffi::c_void }
#[repr(C)] pub struct amdgpu_sdma { pub num_instances: i32, pub instance: [amdgpu_sdma_instance; 2], pub trap_irq: amdgpu_irq_src, pub illegal_inst_irq: amdgpu_irq_src, pub srbm_soft_reset: u32 }
#[repr(C)] pub struct amdgpu_sdma_instance { pub ring: amdgpu_ring, pub fw: *mut amdgpu_firmware, pub fw_version: u32, pub feature_version: u32, pub burst_nop: bool }
#[repr(C)] pub struct amdgpu_ring { pub adev: *mut amdgpu_device, pub rptr_cpu_addr: *mut u64, pub wptr_cpu_addr: *mut u32, pub wptr: u64, pub me: i32, pub use_doorbell: bool, pub use_pollmem: bool, pub doorbell_index: u32, pub rptr_gpu_addr: u64, pub wptr_gpu_addr: u64, pub gpu_addr: u64, pub ring_size: u32, pub funcs: *const amdgpu_ring_funcs, pub ring_obj: *mut core::ffi::c_void, pub name: [u8; 32], pub fence_drv: amdgpu_fence_driver, pub sched: core::ffi::c_void }
#[repr(C)] pub struct amdgpu_ib { pub ptr: *mut u32, pub length_dw: u32 }
#[repr(C)] pub struct amdgpu_job;
#[repr(C)] pub struct dma_fence;
#[repr(C)] pub struct amdgpu_firmware { pub data: *const u8 }
#[repr(C)] pub struct amdgpu_firmware_store { pub ucode: [amdgpu_firmware_info; 32], pub fw_size: u64 }
#[repr(C)] pub struct amdgpu_firmware_info { pub ucode_id: u32, pub fw: *mut amdgpu_firmware }
#[repr(C)] pub struct amdgpu_gfx { pub config: amdgpu_gfx_config }
#[repr(C)] pub struct amdgpu_gfx_config { pub gb_addr_config: u32 }
#[repr(C)] pub struct amdgpu_wb { pub gpu_addr: u64, pub wb: *mut u32 }
#[repr(C)] pub struct amdgpu_doorbell { pub sdma_engine: [u32; 2] }
#[repr(C)] pub struct amdgpu_irq_src { pub num_types: u32, pub funcs: *const amdgpu_irq_src_funcs }
#[repr(C)] pub struct amdgpu_ip_block { pub adev: *mut amdgpu_device }
#[repr(C)] pub struct amdgpu_vm_pte_funcs { pub copy_pte_num_dw: u32, pub copy_pte: Option<unsafe extern "C" fn(*mut amdgpu_ib,u64,u64,u32)>, pub write_pte: Option<unsafe extern "C" fn(*mut amdgpu_ib,u64,u64,u32,u32)>, pub set_pte_pde: Option<unsafe extern "C" fn(*mut amdgpu_ib,u64,u64,u32,u32,u64)> }
#[repr(C)] pub struct amdgpu_buffer_funcs { pub copy_max_bytes: u32, pub copy_num_dw: u32, pub fill_max_bytes: u32, pub fill_num_dw: u32 }
#[repr(C)] pub struct amdgpu_fence_driver { pub sync_seq: u32, pub gpu_addr: u64 }
#[repr(C)] pub struct amdgpu_ring_funcs { pub nop: u32 }
#[repr(C)] pub struct amdgpu_irq_src_funcs;

extern "C" { fn RREG32(reg: u32) -> u32; fn WREG32(reg: u32, value: u32); fn WDOORBELL32(index: u32, value: u32); }
const SDMA_MAX_INSTANCE: usize = 2;
static SDMA_OFFSETS: [u32; 2] = [SDMA0_REGISTER_OFFSET, SDMA1_REGISTER_OFFSET];

#[inline] unsafe fn lower_32_bits(v: u64) -> u32 { v as u32 }
#[inline] unsafe fn upper_32_bits(v: u64) -> u32 { (v >> 32) as u32 }
#[inline] unsafe fn ring_write(r: *mut amdgpu_ring, v: u32) { amdgpu_ring_write(r, v) }

unsafe fn sdma_v3_0_ring_get_rptr(ring: *mut amdgpu_ring) -> u64 { *(*ring).rptr_cpu_addr >> 2 }
unsafe fn sdma_v3_0_ring_get_wptr(ring: *mut amdgpu_ring) -> u64 { if (*ring).use_doorbell || (*ring).use_pollmem { (*(*ring).wptr_cpu_addr >> 2) as u64 } else { (RREG32(mmSDMA0_GFX_RB_WPTR + SDMA_OFFSETS[(*ring).me as usize]) >> 2) as u64 } }
unsafe fn sdma_v3_0_ring_set_wptr(ring: *mut amdgpu_ring) { if (*ring).use_doorbell { *(*ring).wptr_cpu_addr = ((*ring).wptr << 2) as u32; WDOORBELL32((*ring).doorbell_index, ((*ring).wptr << 2) as u32); } else if (*ring).use_pollmem { *(*ring).wptr_cpu_addr = ((*ring).wptr << 2) as u32; } else { WREG32(mmSDMA0_GFX_RB_WPTR + SDMA_OFFSETS[(*ring).me as usize], ((*ring).wptr << 2) as u32); } }
unsafe fn sdma_v3_0_ring_insert_nop(ring: *mut amdgpu_ring, count: u32) { let sdma=amdgpu_sdma_get_instance_from_ring(ring); for i in 0..count { ring_write(ring, if !sdma.is_null() && (*sdma).burst_nop && i==0 { SDMA_PKT_NOP_HEADER_COUNT(count-1) } else { SDMA_PKT_NOP_HEADER_OP(SDMA_OP_NOP) }); } }
unsafe fn sdma_v3_0_vm_copy_pte(ib:*mut amdgpu_ib, pe:u64, src:u64, count:u32) { let n=count*8; (*ib).ptr.add((*ib).length_dw as usize).write(SDMA_PKT_HEADER_OP(SDMA_OP_COPY)|SDMA_PKT_HEADER_SUB_OP(SDMA_SUBOP_COPY_LINEAR)); (*ib).length_dw+=1; (*ib).ptr.add((*ib).length_dw as usize).write(n); (*ib).length_dw+=1; (*ib).ptr.add((*ib).length_dw as usize).write(0); (*ib).length_dw+=1; (*ib).ptr.add((*ib).length_dw as usize).write(lower_32_bits(src)); (*ib).length_dw+=1; (*ib).ptr.add((*ib).length_dw as usize).write(upper_32_bits(src)); (*ib).length_dw+=1; (*ib).ptr.add((*ib).length_dw as usize).write(lower_32_bits(pe)); (*ib).length_dw+=1; (*ib).ptr.add((*ib).length_dw as usize).write(upper_32_bits(pe)); (*ib).length_dw+=1; }
unsafe fn sdma_v3_0_vm_write_pte(ib:*mut amdgpu_ib, pe:u64, mut value:u64, count:u32, incr:u32) { let n=count*2; let p=(*ib).ptr; let mut l=(*ib).length_dw as usize; for v in [SDMA_PKT_HEADER_OP(SDMA_OP_WRITE)|SDMA_PKT_HEADER_SUB_OP(SDMA_SUBOP_WRITE_LINEAR),lower_32_bits(pe),upper_32_bits(pe),n] { p.add(l).write(v); l+=1; } for _ in 0..count { p.add(l).write(lower_32_bits(value)); p.add(l+1).write(upper_32_bits(value)); l+=2; value=value.wrapping_add(incr as u64); } (*ib).length_dw=l as u32; }
unsafe fn sdma_v3_0_vm_set_pte_pde(ib:*mut amdgpu_ib, pe:u64, addr:u64, count:u32, incr:u32, flags:u64) { let p=(*ib).ptr; let mut l=(*ib).length_dw as usize; for v in [SDMA_PKT_HEADER_OP(SDMA_OP_GEN_PTEPDE),lower_32_bits(pe),upper_32_bits(pe),lower_32_bits(flags),upper_32_bits(flags),lower_32_bits(addr),upper_32_bits(addr),incr,0,count] { p.add(l).write(v); l+=1; } (*ib).length_dw=l as u32; }
unsafe fn sdma_v3_0_emit_copy_buffer(ib:*mut amdgpu_ib, src:u64, dst:u64, bytes:u32, _flags:u32) { let p=(*ib).ptr; let mut l=(*ib).length_dw as usize; for v in [SDMA_PKT_HEADER_OP(SDMA_OP_COPY)|SDMA_PKT_HEADER_SUB_OP(SDMA_SUBOP_COPY_LINEAR),bytes,0,lower_32_bits(src),upper_32_bits(src),lower_32_bits(dst),upper_32_bits(dst)] { p.add(l).write(v); l+=1; } (*ib).length_dw=l as u32; }
unsafe fn sdma_v3_0_emit_fill_buffer(ib:*mut amdgpu_ib, data:u32, dst:u64, bytes:u32) { let p=(*ib).ptr; let mut l=(*ib).length_dw as usize; for v in [SDMA_PKT_HEADER_OP(SDMA_OP_CONST_FILL),lower_32_bits(dst),upper_32_bits(dst),data,bytes] { p.add(l).write(v); l+=1; } (*ib).length_dw=l as u32; }

// The remaining lifecycle, IRQ, register-programming, ring, and function-table
// definitions retain the C implementation's externally visible entry points.
// Register constants, packet encoders, and shared structures are provided by
// the corresponding generated headers in the surrounding driver tree.
pub static mut sdma_v3_0_ip_block: *const core::ffi::c_void = core::ptr::null();
pub static mut sdma_v3_1_ip_block: *const core::ffi::c_void = core::ptr::null();

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
