/* Faithful low-level Rust FFI translation of amdgpu_ras.h. */

// Kernel and project-provided types are intentionally left as external dependencies.

pub const AMDGPU_RAS_BOOT_STATUS_POLLING_LIMIT: u32 = 100;
pub const AMDGPU_RAS_BOOT_STEADY_STATUS: u32 = 0xBA;
pub const AMDGPU_RAS_BOOT_STATUS_MASK: u32 = 0xFF;
pub const AMDGPU_RAS_FLAG_INIT_BY_VBIOS: u32 = 0x1 << 0;
pub const AMDGPU_RAS_INST_MASK: u32 = 0xfffff000;
pub const AMDGPU_RAS_INST_SHIFT: u32 = 0xc;
pub const AMDGPU_RAS_FEATURES_SOCKETID_SHIFT: u32 = 29;
pub const AMDGPU_RAS_FEATURES_SOCKETID_MASK: u32 = 0xe0000000;
pub const AMDGPU_RAS_RESERVED_VRAM_SIZE_DEFAULT: u64 = 16u64 << 20;
pub const RAS_EVENT_INVALID_ID: u64 = 1u64 << 63;
pub const AMDGPU_RAS_ERR_INFO_VALID: u32 = 1 << 0;
pub const AMDGPU_RAS_ERR_STATUS_VALID: u32 = 1 << 1;
pub const AMDGPU_RAS_ERR_ADDRESS_VALID: u32 = 1 << 2;
pub const AMDGPU_RAS_GPU_RESET_MODE2_RESET: u32 = 1 << 0;
pub const AMDGPU_RAS_GPU_RESET_MODE1_RESET: u32 = 1 << 1;
pub const MAX_UMC_CHANNEL_NUM: usize = 32;

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum amdgpu_ras_block {
    AMDGPU_RAS_BLOCK__UMC = 0, AMDGPU_RAS_BLOCK__SDMA, AMDGPU_RAS_BLOCK__GFX,
    AMDGPU_RAS_BLOCK__MMHUB, AMDGPU_RAS_BLOCK__ATHUB, AMDGPU_RAS_BLOCK__PCIE_BIF,
    AMDGPU_RAS_BLOCK__HDP, AMDGPU_RAS_BLOCK__XGMI_WAFL, AMDGPU_RAS_BLOCK__DF,
    AMDGPU_RAS_BLOCK__SMN, AMDGPU_RAS_BLOCK__SEM, AMDGPU_RAS_BLOCK__MP0,
    AMDGPU_RAS_BLOCK__MP1, AMDGPU_RAS_BLOCK__FUSE, AMDGPU_RAS_BLOCK__MCA,
    AMDGPU_RAS_BLOCK__VCN, AMDGPU_RAS_BLOCK__JPEG, AMDGPU_RAS_BLOCK__IH,
    AMDGPU_RAS_BLOCK__MPIO, AMDGPU_RAS_BLOCK__MMSCH, AMDGPU_RAS_BLOCK__LAST,
    AMDGPU_RAS_BLOCK__ANY = -1,
}
#[repr(C)] pub enum amdgpu_ras_mca_block { AMDGPU_RAS_MCA_BLOCK__MP0=0, AMDGPU_RAS_MCA_BLOCK__MP1, AMDGPU_RAS_MCA_BLOCK__MPIO, AMDGPU_RAS_MCA_BLOCK__IOHC, AMDGPU_RAS_MCA_BLOCK__LAST }
#[repr(C)] pub enum amdgpu_ras_error_type { AMDGPU_RAS_ERROR__NONE=0, AMDGPU_RAS_ERROR__PARITY=1, AMDGPU_RAS_ERROR__SINGLE_CORRECTABLE=2, AMDGPU_RAS_ERROR__MULTI_UNCORRECTABLE=4, AMDGPU_RAS_ERROR__POISON=8 }
#[repr(C)] pub enum amdgpu_ras_ret { AMDGPU_RAS_SUCCESS=0, AMDGPU_RAS_FAIL, AMDGPU_RAS_UE, AMDGPU_RAS_CE, AMDGPU_RAS_PT }
#[repr(C)] pub enum amdgpu_ras_error_query_mode { AMDGPU_RAS_INVALID_ERROR_QUERY=0, AMDGPU_RAS_DIRECT_ERROR_QUERY, AMDGPU_RAS_FIRMWARE_ERROR_QUERY, AMDGPU_RAS_VIRT_ERROR_COUNT_QUERY }
#[repr(C)] pub enum ras_event_type { RAS_EVENT_TYPE_INVALID=0, RAS_EVENT_TYPE_FATAL, RAS_EVENT_TYPE_POISON_CREATION, RAS_EVENT_TYPE_POISON_CONSUMPTION, RAS_EVENT_TYPE_COUNT }

#[repr(C)] pub struct amdgpu_ras_err_status_reg_entry { pub hwip:u32,pub ip_inst:u32,pub seg_lo:u32,pub reg_lo:u32,pub seg_hi:u32,pub reg_hi:u32,pub reg_inst:u32,pub flags:u32,pub block_name:*const core::ffi::c_char }
#[repr(C)] pub struct amdgpu_ras_memory_id_entry { pub memory_id:u32,pub name:*const core::ffi::c_char }
#[repr(C)] pub struct ras_common_if { pub block:amdgpu_ras_block,pub type_:amdgpu_ras_error_type,pub sub_block_index:u32,pub name:[core::ffi::c_char;32] }
#[repr(C)] pub struct ecc_info_per_ch { pub ce_count_lo_chip:u16,pub ce_count_hi_chip:u16,pub mca_umc_status:u64,pub mca_umc_addr:u64,pub mca_ceumc_addr:u64 }
#[repr(C)] pub struct umc_ecc_info { pub ecc:[ecc_info_per_ch;MAX_UMC_CHANNEL_NUM],pub record_ce_addr_supported:i32 }
#[repr(C)] pub struct ras_event_state { pub last_seqno:u64,pub count:atomic64_t }
#[repr(C)] pub struct ras_event_manager { pub seqno:atomic64_t,pub event_state:[ras_event_state;5] }
#[repr(C)] pub struct ras_event_id { pub type_:ras_event_type,pub event_id:u64 }
#[repr(C)] pub struct ras_query_context { pub evid:ras_event_id }
#[repr(C)] pub struct ras_err_pages { pub count:u32,pub pfn:*mut u64 }
#[repr(C)] pub struct ras_ecc_err { pub status:u64,pub ipid:u64,pub addr:u64,pub pa_pfn:u64,pub channel_idx:u32,pub err_pages:ras_err_pages }
#[repr(C)] pub struct ras_badpage { pub bp:u32,pub size:u32,pub flags:u32 }
#[repr(C)] pub struct ras_inject_if { pub head:ras_common_if,pub address:u64,pub value:u64,pub instance_mask:u32 }
#[repr(C)] pub struct ras_cure_if { pub head:ras_common_if,pub address:u64 }

#[allow(non_camel_case_types)] pub type atomic64_t = core::ffi::c_long;
extern "C" {
    pub fn amdgpu_ras_init_badpage_info(adev:*mut amdgpu_device)->i32;
    pub fn amdgpu_ras_init(adev:*mut amdgpu_device)->i32;
    pub fn amdgpu_ras_fini(adev:*mut amdgpu_device)->i32;
    pub fn amdgpu_ras_query_error_status(adev:*mut amdgpu_device, info:*mut ras_query_if)->i32;
    pub fn amdgpu_ras_error_inject(adev:*mut amdgpu_device, info:*mut ras_inject_if)->i32;
    pub fn amdgpu_ras_reset_gpu(adev:*mut amdgpu_device)->i32;
}
#[repr(C)] pub struct amdgpu_device { _private:[u8;0] }
#[repr(C)] pub struct ras_query_if { pub head:ras_common_if,pub ue_count:usize,pub ce_count:usize,pub de_count:usize }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
