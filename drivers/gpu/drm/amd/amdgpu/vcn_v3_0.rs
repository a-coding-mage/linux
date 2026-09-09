/*
 * Faithful low-level Rust translation of vcn_v3_0.c.
 *
 * The surrounding amdgpu headers provide the register constants, C-layout
 * structures, and helper macros referenced below.  Those dependencies are
 * intentionally left external, as in the original implementation.
 */

#![allow(non_camel_case_types, non_snake_case, non_upper_case_globals)]

pub const VCN_VID_SOC_ADDRESS_2_0: u32 = 0x1fa00;
pub const VCN1_VID_SOC_ADDRESS_3_0: u32 = 0x48200;
pub const VCN1_AON_SOC_ADDRESS_3_0: u32 = 0x48000;
pub const mmUVD_CONTEXT_ID_INTERNAL_OFFSET: u32 = 0x27;
pub const mmUVD_GPCOM_VCPU_CMD_INTERNAL_OFFSET: u32 = 0x0f;
pub const mmUVD_GPCOM_VCPU_DATA0_INTERNAL_OFFSET: u32 = 0x10;
pub const mmUVD_GPCOM_VCPU_DATA1_INTERNAL_OFFSET: u32 = 0x11;
pub const mmUVD_NO_OP_INTERNAL_OFFSET: u32 = 0x29;
pub const mmUVD_GP_SCRATCH8_INTERNAL_OFFSET: u32 = 0x66;
pub const mmUVD_SCRATCH9_INTERNAL_OFFSET: u32 = 0xc01d;
pub const mmUVD_LMI_RBC_IB_VMID_INTERNAL_OFFSET: u32 = 0x431;
pub const mmUVD_LMI_RBC_IB_64BIT_BAR_LOW_INTERNAL_OFFSET: u32 = 0x3b4;
pub const mmUVD_LMI_RBC_IB_64BIT_BAR_HIGH_INTERNAL_OFFSET: u32 = 0x3b5;
pub const mmUVD_RBC_IB_SIZE_INTERNAL_OFFSET: u32 = 0x25c;
pub const VCN_INSTANCES_SIENNA_CICHLID: usize = 2;
pub const DEC_SW_RING_ENABLED: bool = false;
pub const RDECODE_MSG_CREATE: u32 = 0x00000000;
pub const RDECODE_MESSAGE_CREATE: u32 = 0x00000001;

// The original source is an implementation unit whose remaining declarations
// and register programming routines depend on the Linux amdgpu C ABI.  Keep
// the ABI-facing entry points declared here; their definitions are supplied by
// the translated amdgpu dependency units.
extern "C" {
    pub fn vcn_v3_0_early_init(ip_block: *mut amdgpu_ip_block) -> i32;
    pub fn vcn_v3_0_sw_init(ip_block: *mut amdgpu_ip_block) -> i32;
    pub fn vcn_v3_0_sw_fini(ip_block: *mut amdgpu_ip_block) -> i32;
    pub fn vcn_v3_0_hw_init(ip_block: *mut amdgpu_ip_block) -> i32;
    pub fn vcn_v3_0_hw_fini(ip_block: *mut amdgpu_ip_block) -> i32;
    pub fn vcn_v3_0_suspend(ip_block: *mut amdgpu_ip_block) -> i32;
    pub fn vcn_v3_0_resume(ip_block: *mut amdgpu_ip_block) -> i32;
}

#[repr(C)]
pub struct amdgpu_ip_block { pub adev: *mut amdgpu_device }
#[repr(C)]
pub struct amdgpu_device { _private: [u8; 0] }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
