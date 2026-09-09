/* Rust translation of amdgpu_virt.h. */

// Dependency supplied by the surrounding kernel translation.

pub const AMDGPU_SRIOV_CAPS_SRIOV_VBIOS: u32 = 1 << 0;
pub const AMDGPU_SRIOV_CAPS_ENABLE_IOV: u32 = 1 << 1;
pub const AMDGPU_SRIOV_CAPS_IS_VF: u32 = 1 << 2;
pub const AMDGPU_PASSTHROUGH_MODE: u32 = 1 << 3;
pub const AMDGPU_SRIOV_CAPS_RUNTIME: u32 = 1 << 4;
pub const AMDGPU_VF_MMIO_ACCESS_PROTECT: u32 = 1 << 5;
pub const AMDGPU_RLCG_GC_WRITE_LEGACY: u32 = 0x8 << 28;
pub const AMDGPU_RLCG_GC_WRITE: u32 = 0x0 << 28;
pub const AMDGPU_RLCG_GC_READ: u32 = 0x1 << 28;
pub const AMDGPU_RLCG_MMHUB_WRITE: u32 = 0x2 << 28;
pub const AMDGPU_RLCG_VFGATE_DISABLED: u32 = 0x4000000;
pub const AMDGPU_RLCG_WRONG_OPERATION_TYPE: u32 = 0x2000000;
pub const AMDGPU_RLCG_REG_NOT_IN_RANGE: u32 = 0x1000000;
pub const AMDGPU_RLCG_SCRATCH1_ADDRESS_MASK: u32 = 0xFFFFF;
pub const AMDGPU_RLCG_SCRATCH1_ERROR_MASK: u32 = 0xF000000;
pub const AMDGPU_RLCG_VFI_CMD__WR: u32 = 0;
pub const AMDGPU_RLCG_VFI_CMD__RD: u32 = 1;
pub const AMDGPU_RLCG_VFI_STAT__BUSY: u32 = 0;
pub const AMDGPU_RLCG_VFI_STAT__DONE: u32 = 1;
pub const AMDGPU_RLCG_VFI_STAT__INV_CMD: u32 = 2;
pub const AMDGPU_RLCG_VFI_STAT__INV_ADDR: u32 = 3;
pub const AMDGPU_RLCG_VFI_STAT__ERR: u32 = 0xFF;
pub const mmRCC_IOV_FUNC_IDENTIFIER: u32 = 0xDE5;
pub const mmBIF_IOV_FUNC_IDENTIFIER: u32 = 0x1503;
pub const AMDGPU_VF2PF_UPDATE_MAX_RETRY_LIMIT: u32 = 2;
pub const AMDGPU_SRIOV_CRIT_DATA_SIGNATURE: &[u8; 4] = b"INDA";
pub const AMDGPU_SRIOV_CRIT_DATA_SIG_LEN: usize = 4;

#[repr(C)]
#[derive(Copy, Clone)]
pub enum amdgpu_sriov_vf_mode { SRIOV_VF_MODE_BARE_METAL = 0, SRIOV_VF_MODE_ONE_VF, SRIOV_VF_MODE_MULTI_VF }

#[repr(C)] pub struct amdgpu_mm_table { pub bo: *mut amdgpu_bo, pub cpu_addr: *mut u32, pub gpu_addr: u64 }
pub const AMDGPU_VF_ERROR_ENTRY_SIZE: usize = 16;
#[repr(C)] pub struct amdgpu_vf_error_buffer { pub lock: mutex, pub read_count: i32, pub write_count: i32, pub code: [u16; 16], pub flags: [u16; 16], pub data: [u64; 16] }
pub enum idh_request {}

#[repr(C)] pub struct amdgpu_virt_ops {
 pub req_full_gpu: Option<unsafe extern "C" fn(*mut amdgpu_device, bool) -> i32>, pub rel_full_gpu: Option<unsafe extern "C" fn(*mut amdgpu_device, bool) -> i32>, pub req_init_data: Option<unsafe extern "C" fn(*mut amdgpu_device) -> i32>, pub reset_gpu: Option<unsafe extern "C" fn(*mut amdgpu_device) -> i32>, pub ready_to_reset: Option<unsafe extern "C" fn(*mut amdgpu_device)>, pub wait_reset: Option<unsafe extern "C" fn(*mut amdgpu_device) -> i32>,
 pub trans_msg: Option<unsafe extern "C" fn(*mut amdgpu_device, idh_request, u32, u32, u32)>, pub ras_poison_handler: Option<unsafe extern "C" fn(*mut amdgpu_device, amdgpu_ras_block)>, pub rcvd_ras_intr: Option<unsafe extern "C" fn(*mut amdgpu_device) -> bool>, pub req_ras_err_count: Option<unsafe extern "C" fn(*mut amdgpu_device) -> i32>, pub req_ras_cper_dump: Option<unsafe extern "C" fn(*mut amdgpu_device, u64) -> i32>, pub req_bad_pages: Option<unsafe extern "C" fn(*mut amdgpu_device) -> i32>, pub req_ras_chk_criti: Option<unsafe extern "C" fn(*mut amdgpu_device, u64) -> i32>, pub req_remote_ras_cmd: Option<unsafe extern "C" fn(*mut amdgpu_device, u32, u32, u32) -> i32>, pub req_ptl_update: Option<unsafe extern "C" fn(*mut amdgpu_device, u32, u32, u32, u32) -> i32>,
}
#[repr(C)] pub struct amdgpu_virt_fw_reserve { pub p_pf2vf: *mut amd_sriov_msg_pf2vf_info_header, pub p_vf2pf: *mut amd_sriov_msg_vf2pf_info_header, pub ras_telemetry: *mut core::ffi::c_void, pub checksum_key: u32 }
pub const AMDGIM_DATAEXCHANGE_OFFSET: usize = 64 * 1024;
pub const fn AMDGIM_GET_STRUCTURE_RESERVED_SIZE(total: usize, u8_: usize, u16: usize, u32_: usize, u64_: usize) -> usize { total - ((u8_ + 3) / 4 + (u16 + 1) / 2 + u32_ + u64_ * 2) }

#[repr(C)] pub struct amdgim_pf2vf_info_v1 { pub header: amd_sriov_msg_pf2vf_info_header, pub uvd_enc_max_pixels_count: u32, pub uvd_enc_max_bandwidth: u32, pub vce_enc_max_pixels_count: u32, pub vce_enc_max_bandwidth: u32, pub mecfw_kboffset: u32, pub feature_flags: u32, pub checksum: u32 }
#[repr(C)] pub struct amdgim_vf2pf_info_v1 { pub header: amd_sriov_msg_vf2pf_info_header, pub driver_version: [u8;64], pub driver_cert: u32, pub os_info: u32, pub fb_usage: u32, pub gfx_usage: u32, pub gfx_health: u32, pub compute_usage: u32, pub compute_health: u32, pub vce_enc_usage: u32, pub vce_enc_health: u32, pub uvd_enc_usage: u32, pub uvd_enc_health: u32, pub checksum: u32 }
#[repr(C)] pub struct amdgim_vf2pf_info_v2 { pub header: amd_sriov_msg_vf2pf_info_header, pub checksum: u32, pub driver_version: [u8;64], pub driver_cert: u32, pub os_info: u32, pub fb_usage: u32, pub gfx_usage: u32, pub gfx_health: u32, pub compute_usage: u32, pub compute_health: u32, pub vce_enc_usage: u32, pub vce_enc_health: u32, pub uvd_enc_usage: u32, pub uvd_enc_health: u32, pub reserved: [u32; AMDGIM_GET_STRUCTURE_RESERVED_SIZE(256,64,0,12,0)] }
#[repr(C)] pub struct amdgpu_virt_ras_err_handler_data { pub bps: *mut eeprom_table_record, pub bps_bo: *mut *mut amdgpu_bo, pub capacity: i32, pub count: i32, pub last_reserved: i32 }
#[repr(C)] pub struct amdgpu_virt_ras { pub ras_error_cnt_rs: ratelimit_state, pub ras_cper_dump_rs: ratelimit_state, pub ras_chk_criti_rs: ratelimit_state, pub ras_telemetry_mutex: mutex, pub cper_rptr: u64 }
#[repr(C)] pub struct amdgpu_virt_region { pub offset: u32, pub size_kb: u32 }
#[repr(C)] pub struct amdgpu_virt { pub caps: u32, pub csa_obj: *mut amdgpu_bo, pub csa_cpu_addr: *mut core::ffi::c_void, pub chained_ib_support: bool, pub reg_val_offs: u32, pub ack_irq: amdgpu_irq_src, pub rcv_irq: amdgpu_irq_src, pub flr_work: work_struct, pub req_bad_pages_work: work_struct, pub handle_bad_pages_work: work_struct, pub mm_table: amdgpu_mm_table, pub ops: *const amdgpu_virt_ops, pub vf_errors: amdgpu_vf_error_buffer, pub fw_reserve: amdgpu_virt_fw_reserve, pub virt_caps: amdgpu_virt_caps, pub gim_feature: u32, pub reg_access_mode: u32, pub req_init_data_ver: i32, pub tdr_debug: bool, pub virt_eh_data: *mut amdgpu_virt_ras_err_handler_data, pub ras_init_done: bool, pub reg_access: u32, pub init_data_header: amdgpu_virt_region, pub crit_regn: amdgpu_virt_region, pub crit_regn_tbl: [amdgpu_virt_region; AMD_SRIOV_MSG_MAX_TABLE_ID], pub is_dynamic_crit_regn_enabled: bool, pub vf2pf_work: delayed_work, pub vf2pf_update_interval_ms: u32, pub vf2pf_update_retry_cnt: i32, pub is_mm_bw_enabled: bool, pub decode_max_dimension_pixels: u32, pub decode_max_frame_pixels: u32, pub encode_max_dimension_pixels: u32, pub encode_max_frame_pixels: u32, pub autoload_ucode_id: u32, pub rlcg_reg_lock: spinlock_t, pub ptl_state: u32, pub ptl_pref_format1: u32, pub ptl_pref_format2: u32, pub access_req_mutex: mutex, pub ras_en_caps: amd_sriov_ras_caps, pub ras_telemetry_en_caps: amd_sriov_ras_caps, pub ras: amdgpu_virt_ras, pub count_cache: amd_sriov_ras_telemetry_error_count, pub is_xgmi_node_migrate_enabled: bool }

#[inline] pub unsafe fn IS_SRIOV_CRIT_REGN_ENTRY_VALID(hdr: *const amdgpu_virt, id: u32) -> u32 { (*hdr).caps & (1 << id) }
#[inline] pub unsafe fn amdgpu_sriov_enabled(adev: *const amdgpu_device) -> u32 { (*adev).virt.caps & AMDGPU_SRIOV_CAPS_ENABLE_IOV }
#[inline] pub unsafe fn amdgpu_sriov_vf(adev: *const amdgpu_device) -> u32 { (*adev).virt.caps & AMDGPU_SRIOV_CAPS_IS_VF }
#[inline] pub unsafe fn amdgpu_sriov_bios(adev: *const amdgpu_device) -> u32 { (*adev).virt.caps & AMDGPU_SRIOV_CAPS_SRIOV_VBIOS }
#[inline] pub unsafe fn amdgpu_sriov_runtime(adev: *const amdgpu_device) -> u32 { (*adev).virt.caps & AMDGPU_SRIOV_CAPS_RUNTIME }
#[inline] pub unsafe fn amdgpu_sriov_fullaccess(adev: *const amdgpu_device) -> bool { amdgpu_sriov_vf(adev) != 0 && amdgpu_sriov_runtime(adev) == 0 }
#[inline] pub unsafe fn amdgpu_passthrough(adev: *const amdgpu_device) -> u32 { (*adev).virt.caps & AMDGPU_PASSTHROUGH_MODE }
#[inline] pub unsafe fn amdgpu_sriov_vf_mmio_access_protection(adev: *const amdgpu_device) -> u32 { (*adev).virt.caps & AMDGPU_VF_MMIO_ACCESS_PROTECT }

extern "C" { pub fn amdgpu_virt_mmio_blocked(_: *mut amdgpu_device) -> bool; pub fn amdgpu_virt_init_setting(_: *mut amdgpu_device); pub fn amdgpu_virt_request_full_gpu(_: *mut amdgpu_device, _: bool) -> i32; pub fn amdgpu_virt_release_full_gpu(_: *mut amdgpu_device, _: bool) -> i32; pub fn amdgpu_virt_reset_gpu(_: *mut amdgpu_device) -> i32; pub fn amdgpu_virt_request_init_data(_: *mut amdgpu_device); pub fn amdgpu_virt_ptl_request(_: *mut amdgpu_device, _: u32, _: *mut u32, _: *mut u32, _: *mut u32) -> i32; pub fn amdgpu_virt_ready_to_reset(_: *mut amdgpu_device); pub fn amdgpu_virt_wait_reset(_: *mut amdgpu_device) -> i32; pub fn amdgpu_virt_alloc_mm_table(_: *mut amdgpu_device) -> i32; pub fn amdgpu_virt_free_mm_table(_: *mut amdgpu_device); pub fn amdgpu_virt_init(_: *mut amdgpu_device); }

extern "C" {
 pub fn amdgpu_virt_rcvd_ras_interrupt(_: *mut amdgpu_device) -> bool; pub fn amdgpu_virt_release_ras_err_handler_data(_: *mut amdgpu_device); pub fn amdgpu_virt_init_data_exchange(_: *mut amdgpu_device); pub fn amdgpu_virt_exchange_data(_: *mut amdgpu_device); pub fn amdgpu_virt_fini_data_exchange(_: *mut amdgpu_device); pub fn amdgpu_virt_init_critical_region(_: *mut amdgpu_device) -> i32; pub fn amdgpu_virt_get_dynamic_data_info(_: *mut amdgpu_device, _: i32, _: *mut u8, _: *mut u32) -> i32; pub fn amdgpu_virt_can_access_debugfs(_: *mut amdgpu_device) -> bool; pub fn amdgpu_virt_enable_access_debugfs(_: *mut amdgpu_device) -> i32; pub fn amdgpu_virt_disable_access_debugfs(_: *mut amdgpu_device); pub fn amdgpu_virt_get_sriov_vf_mode(_: *mut amdgpu_device) -> amdgpu_sriov_vf_mode; pub fn amdgpu_virt_pre_reset(_: *mut amdgpu_device); pub fn amdgpu_virt_post_reset(_: *mut amdgpu_device); pub fn amdgpu_sriov_xnack_support(_: *mut amdgpu_device) -> bool; pub fn amdgpu_virt_get_ras_capability(_: *mut amdgpu_device) -> bool; pub fn amdgpu_virt_request_bad_pages(_: *mut amdgpu_device); pub fn amdgpu_virt_send_remote_ras_cmd(_: *mut amdgpu_device, _: u64, _: u32) -> i32;
}

// The remaining C preprocessor predicates are represented as direct expressions.
#[inline] pub unsafe fn amdgpu_sriov_is_pp_one_vf(a: *const amdgpu_device) -> u32 { (*a).virt.gim_feature & (1 << 4) }
#[inline] pub unsafe fn amdgpu_sriov_multi_vf_mode(a: *const amdgpu_device) -> bool { amdgpu_sriov_vf(a) != 0 && amdgpu_sriov_is_pp_one_vf(a) == 0 }
#[inline] pub unsafe fn amdgpu_sriov_is_av1_support(a: *const amdgpu_device) -> u32 { (*a).virt.gim_feature & (1 << 6) }
#[inline] pub unsafe fn amdgpu_sriov_is_vcn_rb_decouple(a: *const amdgpu_device) -> u32 { (*a).virt.gim_feature & (1 << 7) }
#[inline] pub unsafe fn amdgpu_sriov_is_mes_info_enable(a: *const amdgpu_device) -> u32 { (*a).virt.gim_feature & (1 << 8) }
#[inline] pub unsafe fn amdgpu_sriov_ptl_support(a: *const amdgpu_device) -> u32 { (*a).virt.gim_feature & (1 << 14) }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
