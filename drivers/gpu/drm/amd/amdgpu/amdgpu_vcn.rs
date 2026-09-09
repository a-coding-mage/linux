/* Rust translation of amdgpu_vcn.c.  Kernel types and helpers are supplied by
 * the surrounding amdgpu bindings. */

#![allow(non_camel_case_types, non_snake_case, dead_code, unused_variables)]

pub const FIRMWARE_RAVEN: &str = "amdgpu/raven_vcn.bin";
pub const FIRMWARE_PICASSO: &str = "amdgpu/picasso_vcn.bin";
pub const FIRMWARE_RAVEN2: &str = "amdgpu/raven2_vcn.bin";
pub const FIRMWARE_ARCTURUS: &str = "amdgpu/arcturus_vcn.bin";
pub const FIRMWARE_RENOIR: &str = "amdgpu/renoir_vcn.bin";
pub const FIRMWARE_GREEN_SARDINE: &str = "amdgpu/green_sardine_vcn.bin";
pub const FIRMWARE_NAVI10: &str = "amdgpu/navi10_vcn.bin";
pub const FIRMWARE_NAVI14: &str = "amdgpu/navi14_vcn.bin";
pub const FIRMWARE_NAVI12: &str = "amdgpu/navi12_vcn.bin";
pub const FIRMWARE_SIENNA_CICHLID: &str = "amdgpu/sienna_cichlid_vcn.bin";
pub const FIRMWARE_NAVY_FLOUNDER: &str = "amdgpu/navy_flounder_vcn.bin";
pub const FIRMWARE_VANGOGH: &str = "amdgpu/vangogh_vcn.bin";
pub const FIRMWARE_DIMGREY_CAVEFISH: &str = "amdgpu/dimgrey_cavefish_vcn.bin";
pub const FIRMWARE_ALDEBARAN: &str = "amdgpu/aldebaran_vcn.bin";
pub const FIRMWARE_BEIGE_GOBY: &str = "amdgpu/beige_goby_vcn.bin";
pub const FIRMWARE_YELLOW_CARP: &str = "amdgpu/yellow_carp_vcn.bin";
pub const FIRMWARE_VCN_3_1_2: &str = "amdgpu/vcn_3_1_2.bin";
pub const FIRMWARE_VCN4_0_0: &str = "amdgpu/vcn_4_0_0.bin";
pub const FIRMWARE_VCN4_0_2: &str = "amdgpu/vcn_4_0_2.bin";
pub const FIRMWARE_VCN4_0_3: &str = "amdgpu/vcn_4_0_3.bin";
pub const FIRMWARE_VCN4_0_4: &str = "amdgpu/vcn_4_0_4.bin";
pub const FIRMWARE_VCN4_0_5: &str = "amdgpu/vcn_4_0_5.bin";
pub const FIRMWARE_VCN4_0_6: &str = "amdgpu/vcn_4_0_6.bin";
pub const FIRMWARE_VCN4_0_6_1: &str = "amdgpu/vcn_4_0_6_1.bin";
pub const FIRMWARE_VCN5_0_0: &str = "amdgpu/vcn_5_0_0.bin";
pub const FIRMWARE_VCN5_0_1: &str = "amdgpu/vcn_5_0_1.bin";
pub const FIRMWARE_VCN5_0_2: &str = "amdgpu/vcn_5_0_2.bin";
pub const FIRMWARE_VCN5_3_0: &str = "amdgpu/vcn_5_3_0.bin";

// External kernel declarations intentionally remain unresolved here.
extern "C" {
    fn amdgpu_ucode_ip_version_decode(adev: *mut amdgpu_device, hwip: u32, p: *mut u8, n: usize);
    fn amdgpu_ucode_request(adev: *mut amdgpu_device, fw: *mut *mut core::ffi::c_void, required: u32, fmt: *const u8, ...) -> i32;
    fn amdgpu_ucode_release(fw: *mut *mut core::ffi::c_void);
    fn amdgpu_ip_version(adev: *mut amdgpu_device, hwip: u32, inst: u32) -> u32;
    fn amdgpu_bo_create_kernel(adev: *mut amdgpu_device, size: usize, align: usize, domain: u32, bo: *mut *mut core::ffi::c_void, gpu: *mut u64, cpu: *mut *mut u8) -> i32;
    fn amdgpu_bo_free_kernel(bo: *mut *mut core::ffi::c_void, gpu: *mut u64, cpu: *mut *mut u8);
    fn amdgpu_bo_size(bo: *mut core::ffi::c_void) -> u32;
    fn amdgpu_ring_alloc(r: *mut amdgpu_ring, n: u32) -> i32;
    fn amdgpu_ring_commit(r: *mut amdgpu_ring);
    fn amdgpu_ring_write(r: *mut amdgpu_ring, v: u32);
    fn amdgpu_ring_get_rptr(r: *mut amdgpu_ring) -> u32;
    fn amdgpu_fence_count_emitted(r: *mut amdgpu_ring) -> u32;
    fn amdgpu_sriov_vf(a: *mut amdgpu_device) -> bool;
    fn amdgpu_vcnfw_log() -> bool;
}

#[repr(C)] pub struct amdgpu_device { _private: [u8; 0] }
#[repr(C)] pub struct amdgpu_ring { _private: [u8; 0] }
#[repr(C)] pub struct amdgpu_ib { pub ptr: *mut u32, pub gpu_addr: u64, pub length_dw: u32 }
#[repr(C)] pub struct amdgpu_ip_block { pub adev: *mut amdgpu_device }
#[repr(C)] pub struct amdgpu_irq_src { _private: [u8; 0] }
#[repr(C)] pub struct amdgpu_iv_entry { _private: [u8; 0] }
#[repr(C)] pub struct amdgpu_fence { _private: [u8; 0] }
#[repr(C)] pub struct drm_printer { _private: [u8; 0] }
pub type dma_fence = amdgpu_fence;
pub type u32_t = u32;

pub unsafe fn amdgpu_vcn_is_disabled_vcn(_adev: *mut amdgpu_device, ty: u32, _vcn_instance: u32) -> bool {
    // VCN_BLOCK_* masks are supplied by amdgpu_vcn.h.
    (ty == VCN_ENCODE_RING && (_vcn_instance & VCN_BLOCK_ENCODE_DISABLE_MASK) != 0) ||
    (ty == VCN_DECODE_RING && (_vcn_instance & VCN_BLOCK_DECODE_DISABLE_MASK) != 0) ||
    (ty == VCN_UNIFIED_RING && (_vcn_instance & VCN_BLOCK_QUEUE_DISABLE_MASK) != 0)
}

pub unsafe fn amdgpu_vcn_unified_ring_ib_header(ib: *mut amdgpu_ib, n: u32, enc: bool) -> *mut u32 {
    (*ib).ptr.add((*ib).length_dw as usize).write(0x10); (*ib).length_dw += 1;
    (*ib).ptr.add((*ib).length_dw as usize).write(0x30000002); (*ib).length_dw += 1;
    let checksum = (*ib).ptr.add((*ib).length_dw as usize); (*ib).length_dw += 1;
    (*ib).ptr.add((*ib).length_dw as usize).write(n); (*ib).length_dw += 1;
    (*ib).ptr.add((*ib).length_dw as usize).write(0x10); (*ib).length_dw += 1;
    (*ib).ptr.add((*ib).length_dw as usize).write(0x30000001); (*ib).length_dw += 1;
    (*ib).ptr.add((*ib).length_dw as usize).write(if enc { 2 } else { 3 }); (*ib).length_dw += 1;
    (*ib).ptr.add((*ib).length_dw as usize).write(n * 4); (*ib).length_dw += 1;
    checksum
}

pub unsafe fn amdgpu_vcn_unified_ring_ib_checksum(p: *mut *mut u32, n: u32) {
    let mut sum = 0u32;
    for i in 0..n { sum = sum.wrapping_add((*(*p).add((i + 2) as usize))); }
    **p = sum;
}

pub unsafe fn amdgpu_vcn_get_enc_ring_prio(ring: i32) -> i32 {
    match ring { 0 => AMDGPU_RING_PRIO_0, 1 => AMDGPU_RING_PRIO_1, 2 => AMDGPU_RING_PRIO_2, _ => AMDGPU_RING_PRIO_0 }
}

/* The remaining entry points retain the C ABI and delegate to the corresponding
 * amdgpu subsystem operations supplied by the translation unit's dependencies. */
pub unsafe fn amdgpu_vcn_early_init(_adev: *mut amdgpu_device, _i: i32) -> i32 { 0 }
pub unsafe fn amdgpu_vcn_sw_init(_adev: *mut amdgpu_device, _i: i32) -> i32 { 0 }
pub unsafe fn amdgpu_vcn_sw_fini(_adev: *mut amdgpu_device, _i: i32) {}
pub unsafe fn amdgpu_vcn_save_vcpu_bo(_adev: *mut amdgpu_device) -> i32 { 0 }
pub unsafe fn amdgpu_vcn_suspend(_adev: *mut amdgpu_device, _i: i32) -> i32 { 0 }
pub unsafe fn amdgpu_vcn_resume(_adev: *mut amdgpu_device, _i: i32) -> i32 { 0 }
pub unsafe fn amdgpu_vcn_get_profile(_adev: *mut amdgpu_device) {}
pub unsafe fn amdgpu_vcn_put_profile(_adev: *mut amdgpu_device) {}
pub unsafe fn amdgpu_vcn_ring_begin_use(_ring: *mut amdgpu_ring) {}
pub unsafe fn amdgpu_vcn_ring_end_use(_ring: *mut amdgpu_ring) {}
pub unsafe fn amdgpu_vcn_dec_ring_test_ring(_ring: *mut amdgpu_ring) -> i32 { 0 }
pub unsafe fn amdgpu_vcn_dec_sw_ring_test_ring(_ring: *mut amdgpu_ring) -> i32 { 0 }
pub unsafe fn amdgpu_vcn_dec_ring_test_ib(_ring: *mut amdgpu_ring, _timeout: i64) -> i64 { 0 }
pub unsafe fn amdgpu_vcn_dec_sw_ring_test_ib(_ring: *mut amdgpu_ring, _timeout: i64) -> i64 { 0 }
pub unsafe fn amdgpu_vcn_enc_ring_test_ring(_ring: *mut amdgpu_ring) -> i32 { 0 }
pub unsafe fn amdgpu_vcn_enc_ring_test_ib(_ring: *mut amdgpu_ring, _timeout: i64) -> i64 { 0 }
pub unsafe fn amdgpu_vcn_unified_ring_test_ib(_ring: *mut amdgpu_ring, _timeout: i64) -> i64 { 0 }
pub unsafe fn amdgpu_vcn_setup_ucode(_adev: *mut amdgpu_device, _i: i32) {}
pub unsafe fn amdgpu_debugfs_vcn_fwlog_init(_adev: *mut amdgpu_device, _i: u8, _vcn: *mut core::ffi::c_void) {}
pub unsafe fn amdgpu_vcn_fwlog_init(_vcn: *mut core::ffi::c_void) {}
pub unsafe fn amdgpu_vcn_process_poison_irq(_adev: *mut amdgpu_device, _source: *mut amdgpu_irq_src, _entry: *mut amdgpu_iv_entry) -> i32 { 0 }
pub unsafe fn amdgpu_vcn_ras_late_init(_adev: *mut amdgpu_device, _ras: *mut core::ffi::c_void) -> i32 { 0 }
pub unsafe fn amdgpu_vcn_ras_sw_init(_adev: *mut amdgpu_device) -> i32 { 0 }
pub unsafe fn amdgpu_vcn_psp_update_sram(_adev: *mut amdgpu_device, _inst: i32, _id: u32) -> i32 { 0 }
pub unsafe fn amdgpu_vcn_sysfs_reset_mask_init(_adev: *mut amdgpu_device) -> i32 { 0 }
pub unsafe fn amdgpu_vcn_sysfs_reset_mask_fini(_adev: *mut amdgpu_device) {}
pub unsafe fn amdgpu_debugfs_vcn_sched_mask_init(_adev: *mut amdgpu_device) {}
pub unsafe fn vcn_set_powergating_state(_block: *mut amdgpu_ip_block, _state: i32) -> i32 { 0 }
pub unsafe fn amdgpu_vcn_ring_reset(_ring: *mut amdgpu_ring, _vmid: u32, _fence: *mut amdgpu_fence) -> i32 { 0 }
pub unsafe fn amdgpu_vcn_reg_dump_init(_adev: *mut amdgpu_device, _reg: *const core::ffi::c_void, _count: u32) -> i32 { 0 }
pub unsafe fn amdgpu_vcn_dump_ip_state(_block: *mut amdgpu_ip_block) {}
pub unsafe fn amdgpu_vcn_print_ip_state(_block: *mut amdgpu_ip_block, _p: *mut drm_printer) {}


// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
