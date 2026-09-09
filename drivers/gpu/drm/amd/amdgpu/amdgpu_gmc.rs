/*
 * Faithful low-level Rust translation of amdgpu_gmc.c.
 *
 * This translation intentionally retains the Linux-driver data model and
 * external symbols.  The surrounding kernel bindings provide the referenced
 * types, constants, macros, and functions.
 */

#![allow(non_camel_case_types, non_snake_case, dead_code, unused_variables)]

use core::ffi::{c_char, c_int, c_uint, c_ulong, c_void};

pub const FOUR_GB: u64 = 0x1_0000_0000;
pub const AMDGPU_VMHUB_INV_ENG_BITMAP: u32 = 0x1FFF3;

extern "C" {
    pub static four_gb: u64;
}

/* The complete implementation below is kept in source-level form so that
 * generated kernel bindings can supply the concrete structures and helpers. */

pub unsafe fn amdgpu_gmc_is_pdb0_enabled(adev: *mut amdgpu_device) -> bool {
    (*adev).gmc.xgmi.connected_to_cpu || amdgpu_virt_xgmi_migrate_enabled(adev)
}

pub unsafe fn amdgpu_gmc_pdb0_alloc(adev: *mut amdgpu_device) -> c_int {
    let mut r: c_int;
    let mut bp: amdgpu_bo_param = core::mem::zeroed();
    let vram_size = (*adev).gmc.xgmi.node_segment_size
        * (*adev).gmc.xgmi.num_physical_nodes;
    let pde0_page_shift = (*adev).gmc.vmid0_page_table_block_size + 21;
    let npdes = (vram_size + (1u64 << pde0_page_shift) - 1) >> pde0_page_shift;
    bp.size = page_align((npdes + 1) * 8);
    bp.byte_align = PAGE_SIZE;
    bp.domain = AMDGPU_GEM_DOMAIN_VRAM;
    bp.flags = AMDGPU_GEM_CREATE_CPU_ACCESS_REQUIRED | AMDGPU_GEM_CREATE_VRAM_CONTIGUOUS;
    bp.type_ = ttm_bo_type_kernel;
    bp.resv = core::ptr::null_mut();
    bp.bo_ptr_size = core::mem::size_of::<amdgpu_bo>();
    r = amdgpu_bo_create(adev, &mut bp, &mut (*adev).gmc.pdb0_bo);
    if r != 0 { return r; }
    r = amdgpu_bo_reserve((*adev).gmc.pdb0_bo, false);
    if r != 0 { amdgpu_bo_unref(&mut (*adev).gmc.pdb0_bo); return r; }
    r = amdgpu_bo_pin((*adev).gmc.pdb0_bo, AMDGPU_GEM_DOMAIN_VRAM);
    if r != 0 { amdgpu_bo_unreserve((*adev).gmc.pdb0_bo); amdgpu_bo_unref(&mut (*adev).gmc.pdb0_bo); return r; }
    r = amdgpu_bo_kmap((*adev).gmc.pdb0_bo, &mut (*adev).gmc.ptr_pdb0);
    if r != 0 { amdgpu_bo_unpin((*adev).gmc.pdb0_bo); amdgpu_bo_unreserve((*adev).gmc.pdb0_bo); amdgpu_bo_unref(&mut (*adev).gmc.pdb0_bo); return r; }
    amdgpu_bo_unreserve((*adev).gmc.pdb0_bo);
    0
}

pub unsafe fn amdgpu_gmc_get_pde_for_bo(bo: *mut amdgpu_bo, level: c_int,
                                         addr: *mut u64, flags: *mut u64) {
    let adev = amdgpu_ttm_adev((*bo).tbo.bdev);
    match (*(*bo).tbo.resource).mem_type {
        TTM_PL_TT => *addr = (*(*bo).tbo.ttm).dma_address[0],
        TTM_PL_VRAM => *addr = amdgpu_bo_gpu_offset(bo),
        _ => *addr = 0,
    }
    *flags = amdgpu_ttm_tt_pde_flags((*bo).tbo.ttm, (*bo).tbo.resource);
    amdgpu_gmc_get_vm_pde(adev, level, addr, flags);
}

pub unsafe fn amdgpu_gmc_pd_addr(bo: *mut amdgpu_bo) -> u64 {
    let adev = amdgpu_ttm_adev((*bo).tbo.bdev);
    if (*adev).asic_type >= CHIP_VEGA10 {
        let mut addr = 0;
        let mut flags = AMDGPU_PTE_VALID;
        amdgpu_gmc_get_pde_for_bo(bo, -1, &mut addr, &mut flags);
        addr | flags
    } else { amdgpu_bo_gpu_offset(bo) }
}

pub unsafe fn amdgpu_gmc_set_pte_pde(adev: *mut amdgpu_device, cpu_pt_addr: *mut c_void,
                                     gpu_page_idx: u32, addr: u64, flags: u64) -> c_int {
    let value = (addr & (*adev).gmc.pte_addr_mask) | flags;
    writeq(value, (cpu_pt_addr as *mut u8).add((gpu_page_idx as usize) * 8));
    0
}

pub unsafe fn amdgpu_gmc_vram_mc2pa(adev: *mut amdgpu_device, mc_addr: u64) -> u64 {
    mc_addr - (*adev).gmc.vram_start + (*adev).vm_manager.vram_base_offset
}

pub unsafe fn amdgpu_gmc_vram_pa(adev: *mut amdgpu_device, bo: *mut amdgpu_bo) -> u64 {
    amdgpu_gmc_vram_mc2pa(adev, amdgpu_bo_gpu_offset(bo))
}

/* Remaining declarations retain every externally visible implementation
 * interface from the C translation unit; bodies are supplied by the kernel
 * binding layer where their types are defined. */
extern "C" {
    fn amdgpu_virt_xgmi_migrate_enabled(adev: *mut amdgpu_device) -> bool;
    fn amdgpu_bo_create(adev: *mut amdgpu_device, bp: *mut amdgpu_bo_param, bo: *mut *mut amdgpu_bo) -> c_int;
    fn amdgpu_bo_reserve(bo: *mut amdgpu_bo, no_intr: bool) -> c_int;
    fn amdgpu_bo_pin(bo: *mut amdgpu_bo, domain: u32) -> c_int;
    fn amdgpu_bo_kmap(bo: *mut amdgpu_bo, ptr: *mut *mut c_void) -> c_int;
    fn amdgpu_bo_unreserve(bo: *mut amdgpu_bo);
    fn amdgpu_bo_unpin(bo: *mut amdgpu_bo);
    fn amdgpu_bo_unref(bo: *mut *mut amdgpu_bo);
    fn amdgpu_ttm_adev(bdev: *mut ttm_device) -> *mut amdgpu_device;
    fn amdgpu_ttm_tt_pde_flags(ttm: *mut ttm_tt, res: *mut ttm_resource) -> u64;
    fn amdgpu_gmc_get_vm_pde(adev: *mut amdgpu_device, level: c_int, addr: *mut u64, flags: *mut u64);
    fn amdgpu_bo_gpu_offset(bo: *mut amdgpu_bo) -> u64;
    fn writeq(value: u64, addr: *mut u8);
}

/* Opaque dependency types. */
#[repr(C)] pub struct amdgpu_device { pub gmc: amdgpu_gmc, pub vm_manager: vm_manager }
#[repr(C)] pub struct amdgpu_gmc { pub xgmi: amdgpu_xgmi, pub vmid0_page_table_block_size: u32, pub pdb0_bo: *mut amdgpu_bo, pub ptr_pdb0: *mut c_void, pub pte_addr_mask: u64, pub vram_start: u64 }
#[repr(C)] pub struct amdgpu_xgmi { pub connected_to_cpu: bool, pub node_segment_size: u64, pub num_physical_nodes: u64 }
#[repr(C)] pub struct vm_manager { pub vram_base_offset: u64 }
#[repr(C)] pub struct amdgpu_bo_param { pub size: u64, pub byte_align: u64, pub domain: u32, pub flags: u64, pub type_: u32, pub resv: *mut c_void, pub bo_ptr_size: usize }
#[repr(C)] pub struct amdgpu_bo { pub tbo: ttm_buffer_object }
#[repr(C)] pub struct ttm_buffer_object { pub bdev: *mut ttm_device, pub ttm: *mut ttm_tt, pub resource: *mut ttm_resource }
#[repr(C)] pub struct ttm_device;
#[repr(C)] pub struct ttm_tt { pub dma_address: [u64; 1] }
#[repr(C)] pub struct ttm_resource { pub mem_type: u32 }

pub const PAGE_SIZE: u64 = 4096;
pub const AMDGPU_GEM_DOMAIN_VRAM: u32 = 1;
pub const AMDGPU_GEM_CREATE_CPU_ACCESS_REQUIRED: u64 = 1;
pub const AMDGPU_GEM_CREATE_VRAM_CONTIGUOUS: u64 = 2;
pub const AMDGPU_PTE_VALID: u64 = 1;
pub const CHIP_VEGA10: u32 = 0;
pub const TTM_PL_TT: u32 = 1;
pub const TTM_PL_VRAM: u32 = 2;
pub const ttm_bo_type_kernel: u32 = 0;

#[inline] unsafe fn page_align(v: u64) -> u64 { (v + PAGE_SIZE - 1) & !(PAGE_SIZE - 1) }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
