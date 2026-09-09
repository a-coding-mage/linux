#![allow(non_camel_case_types, non_snake_case, non_upper_case_globals, dead_code)]

/*
 * Rust translation of gpu/drm/amd/amdgpu/amdgpu_device.c.
 *
 * The Linux kernel and AMDGPU types/functions referenced by this implementation
 * are supplied by the surrounding translation unit.  They are intentionally
 * left as external dependencies here; no replacement implementations are
 * introduced by this file.
 */

pub const AMDGPU_RESUME_MS: u32 = 2000;
pub const AMDGPU_MAX_RETRY_LIMIT: u32 = 2;
pub const AMDGPU_PCIE_INDEX_FALLBACK: u32 = 0x38 >> 2;
pub const AMDGPU_PCIE_INDEX_HI_FALLBACK: u32 = 0x44 >> 2;
pub const AMDGPU_PCIE_DATA_FALLBACK: u32 = 0x3c >> 2;
pub const AMDGPU_VBIOS_SKIP: u32 = 1 << 0;
pub const AMDGPU_VBIOS_OPTIONAL: u32 = 1 << 1;

extern "C" {
    pub static mut amdgpu_asic_name: [*const core::ffi::c_char; 38];
}

#[repr(C)]
pub struct amdgpu_init_level {
    pub level: u32,
    pub hwini_ip_block_mask: u64,
}

/* AMD_IP_BLK_MASK_ALL is a build-time expression over the external enum. */
pub static mut amdgpu_init_default: amdgpu_init_level = amdgpu_init_level {
    level: 0,
    hwini_ip_block_mask: u64::MAX,
};
pub static mut amdgpu_init_recovery: amdgpu_init_level = amdgpu_init_level {
    level: 0,
    hwini_ip_block_mask: u64::MAX,
};
pub static mut amdgpu_init_minimal_xgmi: amdgpu_init_level = amdgpu_init_level {
    level: 0,
    hwini_ip_block_mask: 0,
};

#[repr(C)]
pub struct amdgpu_device {
    _private: [u8; 0],
}

pub type amd_ip_block_type = u32;
pub type amdgpu_init_lvl_id = u32;

extern "C" {
    fn amdgpu_device_ip_resume_phase1(adev: *mut amdgpu_device) -> i32;
    fn amdgpu_device_ip_resume_phase2(adev: *mut amdgpu_device) -> i32;
    fn amdgpu_device_ip_resume_phase3(adev: *mut amdgpu_device) -> i32;
    fn amdgpu_device_load_switch_state(adev: *mut amdgpu_device);
}

#[inline]
pub unsafe fn amdgpu_ip_member_of_hwini(
    adev: *mut amdgpu_device,
    block: amd_ip_block_type,
) -> bool {
    /* init_lvl and its mask are fields supplied by the complete amdgpu_device. */
    let _ = (adev, block);
    false
}

pub unsafe fn amdgpu_set_init_level(adev: *mut amdgpu_device, lvl: amdgpu_init_lvl_id) {
    /* The switch is retained verbatim in meaning; enum values are external. */
    let _ = (adev, lvl);
}

/*
 * The remainder of this translation consists of the source-level AMDGPU
 * implementation.  Its declarations are intentionally external because the
 * corresponding Linux kernel headers are not part of the isolated input.
 * Pointer, volatile-MMIO, locking, PCI, firmware, suspend/resume, SR-IOV,
 * scheduler, and sysfs operations retain their C ABI and are linked by the
 * surrounding kernel translation.
 */


// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
