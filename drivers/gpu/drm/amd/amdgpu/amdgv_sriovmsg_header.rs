/* Direct Rust translation of amdgv_sriovmsg.h. */

pub const AMD_SRIOV_MSG_SIZE_KB: usize = 1;
pub const AMD_SRIOV_MSG_VBIOS_SIZE_KB_V1: usize = 64;
pub const AMD_SRIOV_MSG_PF2VF_SIZE_KB_V1: usize = 1;
pub const AMD_SRIOV_MSG_VF2PF_SIZE_KB_V1: usize = 1;
pub const AMD_SRIOV_MSG_BAD_PAGE_SIZE_KB_V1: usize = 2;
pub const AMD_SRIOV_MSG_RAS_TELEMETRY_SIZE_KB_V1: usize = 64;
pub const AMD_SRIOV_MSG_DATAEXCHANGE_SIZE_KB_V1: usize = AMD_SRIOV_MSG_PF2VF_SIZE_KB_V1 + AMD_SRIOV_MSG_VF2PF_SIZE_KB_V1 + AMD_SRIOV_MSG_BAD_PAGE_SIZE_KB_V1;
pub const AMD_SRIOV_MSG_VBIOS_OFFSET_V1: usize = 0;
pub const AMD_SRIOV_MSG_DATAEXCHANGE_OFFSET_KB_V1: usize = AMD_SRIOV_MSG_VBIOS_SIZE_KB_V1;
pub const AMD_SRIOV_MSG_TMR_OFFSET_KB: usize = 2048;
pub const AMD_SRIOV_MSG_PF2VF_OFFSET_KB_V1: usize = AMD_SRIOV_MSG_DATAEXCHANGE_OFFSET_KB_V1;
pub const AMD_SRIOV_MSG_VF2PF_OFFSET_KB_V1: usize = AMD_SRIOV_MSG_PF2VF_OFFSET_KB_V1 + AMD_SRIOV_MSG_SIZE_KB;
pub const AMD_SRIOV_MSG_BAD_PAGE_OFFSET_KB_V1: usize = AMD_SRIOV_MSG_VF2PF_OFFSET_KB_V1 + AMD_SRIOV_MSG_SIZE_KB;
pub const AMD_SRIOV_MSG_RAS_TELEMETRY_OFFSET_KB_V1: usize = AMD_SRIOV_MSG_BAD_PAGE_OFFSET_KB_V1 + AMD_SRIOV_MSG_BAD_PAGE_SIZE_KB_V1;
pub const AMD_SRIOV_MSG_INIT_DATA_TOT_SIZE_KB_V1: usize = AMD_SRIOV_MSG_VBIOS_SIZE_KB_V1 + AMD_SRIOV_MSG_DATAEXCHANGE_SIZE_KB_V1 + AMD_SRIOV_MSG_RAS_TELEMETRY_SIZE_KB_V1;

#[repr(u32)] pub enum AmdSriovCritRegionVersion { GPU_CRIT_REGION_V1 = 1, GPU_CRIT_REGION_V2 = 2 }
#[repr(u32)] pub enum AmdSriovMsgTableIdEnum { AMD_SRIOV_MSG_IPD_TABLE_ID = 0, AMD_SRIOV_MSG_VBIOS_IMG_TABLE_ID, AMD_SRIOV_MSG_RAS_TELEMETRY_TABLE_ID, AMD_SRIOV_MSG_DATAEXCHANGE_TABLE_ID, AMD_SRIOV_MSG_BAD_PAGE_INFO_TABLE_ID, AMD_SRIOV_MSG_INITD_H_TABLE_ID, AMD_SRIOV_MSG_MAX_TABLE_ID }

#[repr(C)] pub struct AmdSriovMsgInitDataHeader { pub signature: [i8;4], pub version:u32, pub checksum:u32, pub initdata_offset:u32, pub initdata_size_in_kb:u32, pub valid_tables:u32, pub vbios_img_offset:u32, pub vbios_img_size_in_kb:u32, pub dataexchange_offset:u32, pub dataexchange_size_in_kb:u32, pub ras_tele_info_offset:u32, pub ras_tele_info_size_in_kb:u32, pub ip_discovery_offset:u32, pub ip_discovery_size_in_kb:u32, pub bad_page_info_offset:u32, pub bad_page_size_in_kb:u32, pub reserved:[u32;8] }

pub const AMD_SRIOV_MSG_FW_VRAM_PF2VF_VER:u32=2; pub const AMD_SRIOV_MSG_FW_VRAM_VF2PF_VER:u32=3;
pub const AMD_SRIOV_MSG_RESERVE_UCODE:usize=24; pub const AMD_SRIOV_MSG_RESERVE_VCN_INST:usize=4;
#[repr(u32)] pub enum AmdSriovUcodeEngineId { AMD_SRIOV_UCODE_ID_VCE=0, AMD_SRIOV_UCODE_ID_UVD, AMD_SRIOV_UCODE_ID_MC, AMD_SRIOV_UCODE_ID_ME, AMD_SRIOV_UCODE_ID_PFP, AMD_SRIOV_UCODE_ID_CE, AMD_SRIOV_UCODE_ID_RLC, AMD_SRIOV_UCODE_ID_RLC_SRLC, AMD_SRIOV_UCODE_ID_RLC_SRLG, AMD_SRIOV_UCODE_ID_RLC_SRLS, AMD_SRIOV_UCODE_ID_MEC, AMD_SRIOV_UCODE_ID_MEC2, AMD_SRIOV_UCODE_ID_SOS, AMD_SRIOV_UCODE_ID_ASD, AMD_SRIOV_UCODE_ID_TA_RAS, AMD_SRIOV_UCODE_ID_TA_XGMI, AMD_SRIOV_UCODE_ID_SMC, AMD_SRIOV_UCODE_ID_SDMA, AMD_SRIOV_UCODE_ID_SDMA2, AMD_SRIOV_UCODE_ID_VCN, AMD_SRIOV_UCODE_ID_DMCU, AMD_SRIOV_UCODE_ID__MAX }

#[repr(C)] pub struct AmdSriovMsgFeatureFlags { pub all:u32 }
#[repr(C)] pub struct AmdSriovRegAccessFlags { pub all:u32 }
#[repr(C)] pub struct AmdSriovRasCaps { pub all:u64 }
#[repr(C)] pub struct AmdSriovMsgOsInfo { pub all:u32 }
#[repr(C)] pub struct AmdSriovMsgUuidInfo { pub time_low:u32, pub time_mid_high:u32, pub clock_seq:u16, pub asic_4:u16, pub asic_0:u32 }
#[repr(C)] pub struct AmdSriovMsgPf2vfInfoHeader { pub size:u32, pub version:u32, pub reserved:[u32;2] }
pub const AMD_SRIOV_MSG_PF2VF_INFO_FILLED_SIZE:usize=59;
#[repr(C, packed)] pub struct AmdSriovMsgPf2vfInfo { pub header:AmdSriovMsgPf2vfInfoHeader, pub checksum:u32, pub feature_flags:AmdSriovMsgFeatureFlags, pub hevc_enc_max_mb_per_second:u32, pub hevc_enc_max_mb_per_frame:u32, pub avc_enc_max_mb_per_second:u32, pub avc_enc_max_mb_per_frame:u32, pub mecfw_offset:u64, pub mecfw_size:u32, pub uvdfw_offset:u64, pub uvdfw_size:u32, pub vcefw_offset:u64, pub vcefw_size:u32, pub bp_block_offset_low:u32, pub bp_block_offset_high:u32, pub bp_block_size:u32, pub vf2pf_update_interval_ms:u32, pub uuid:u64, pub pad:u32, pub reg_access_flags:AmdSriovRegAccessFlags, pub mm_bw_management:[[u32;4];AMD_SRIOV_MSG_RESERVE_VCN_INST], pub uuid_info:AmdSriovMsgUuidInfo, pub pcie_atomic_ops_support_flags:u32, pub gpu_capacity:u32, pub bdf_on_host:u32, pub more_bp:u32, pub ras_en_caps:AmdSriovRasCaps, pub ras_telemetry_en_caps:AmdSriovRasCaps, pub ptl_enabled:u32, pub ptl_pref_format1:u32, pub ptl_pref_format2:u32, pub unitid:u8, pub padding:[u8;3], pub reserved:[u32;256-AMD_SRIOV_MSG_PF2VF_INFO_FILLED_SIZE] }
#[repr(C)] pub struct AmdSriovMsgVf2pfInfoHeader { pub size:u32, pub version:u32, pub reserved:[u32;2] }
pub const AMD_SRIOV_MSG_VF2PF_INFO_FILLED_SIZE:usize=73;
#[repr(C, packed)] pub struct AmdSriovUcodeInfo { pub id:u8, pub version:u32 }
#[repr(C, packed)] pub struct AmdSriovMsgVf2pfInfo { pub header:AmdSriovMsgVf2pfInfoHeader, pub checksum:u32, pub driver_version:[u8;64], pub driver_cert:u32, pub os_info:AmdSriovMsgOsInfo, pub fb_usage:u32, pub gfx_usage:u32, pub gfx_health:u32, pub compute_usage:u32, pub compute_health:u32, pub avc_enc_usage:u32, pub avc_enc_health:u32, pub hevc_enc_usage:u32, pub hevc_enc_health:u32, pub encode_usage:u32, pub decode_usage:u32, pub pf2vf_version_required:u32, pub fb_vis_usage:u32, pub fb_vis_size:u32, pub fb_size:u32, pub ucode_info:[AmdSriovUcodeInfo;AMD_SRIOV_MSG_RESERVE_UCODE], pub dummy_page_addr:u64, pub mes_info_addr:u64, pub mes_info_size:u32, pub reserved:[u32;256-AMD_SRIOV_MSG_VF2PF_INFO_FILLED_SIZE] }

#[repr(u32)] pub enum AmdSriovMailboxRequestMessage { MB_REQ_MSG_REQ_GPU_INIT_ACCESS=1, MB_REQ_MSG_REL_GPU_INIT_ACCESS, MB_REQ_MSG_REQ_GPU_FINI_ACCESS, MB_REQ_MSG_REL_GPU_FINI_ACCESS, MB_REQ_MSG_REQ_GPU_RESET_ACCESS, MB_REQ_MSG_REQ_GPU_INIT_DATA, MB_REQ_MSG_PSP_VF_CMD_RELAY, MB_REQ_MSG_LOG_VF_ERROR=200, MB_REQ_MSG_READY_TO_RESET, MB_REQ_MSG_RAS_POISON, MB_REQ_RAS_ERROR_COUNT, MB_REQ_RAS_CPER_DUMP, MB_REQ_RAS_BAD_PAGES, MB_REQ_RAS_CHK_CRITI, MB_REQ_RAS_REMOTE_CMD, MB_REQ_MSG_PTL_UPDATE }
#[repr(u32)] pub enum AmdSriovMailboxResponseMessage { MB_RES_MSG_CLR_MSG_BUF=0, MB_RES_MSG_READY_TO_ACCESS_GPU, MB_RES_MSG_FLR_NOTIFICATION, MB_RES_MSG_FLR_NOTIFICATION_COMPLETION, MB_RES_MSG_SUCCESS, MB_RES_MSG_FAIL, MB_RES_MSG_QUERY_ALIVE, MB_RES_MSG_GPU_INIT_DATA_READY, MB_RES_MSG_RAS_POISON_READY, MB_RES_MSG_PF_SOFT_FLR_NOTIFICATION, MB_RES_MSG_GPU_RMA, MB_RES_MSG_RAS_ERROR_COUNT_READY, MB_REQ_RAS_CPER_DUMP_READY=14, MB_RES_MSG_RAS_BAD_PAGES_READY, MB_RES_MSG_RAS_BAD_PAGES_NOTIFICATION, MB_RES_MSG_UNRECOV_ERR_NOTIFICATION, MB_RES_RAS_CHK_CRITI_READY, MB_RES_RAS_REMOTE_CMD_READY, MB_RES_MSG_PTL_UPDATE_READY, MB_RES_MSG_TEXT_MESSAGE=255 }
#[repr(u32)] pub enum AmdSriovResponseStatus { AMD_SRIOV_RESP_SUCCESS=0, AMD_SRIOV_RESP_FAIL, AMD_SRIOV_RESP_UNSUPPORTED }
pub const fn amd_sriov_ptl_pack_formats(fmt1:u32,fmt2:u32)->u32 { ((fmt1&0xffff)<<16)|(fmt2&0xffff) }
pub const fn amd_sriov_ptl_unpack_status(dw:u32)->u32 {(dw>>16)&0xffff} pub const fn amd_sriov_ptl_unpack_state(dw:u32)->u32 {dw&0xffff} pub const fn amd_sriov_ptl_unpack_fmt1(dw:u32)->u32 {(dw>>16)&0xffff} pub const fn amd_sriov_ptl_unpack_fmt2(dw:u32)->u32 {dw&0xffff}
#[repr(u32)] pub enum AmdSriovRasTelemetryGpuBlock { RAS_TELEMETRY_GPU_BLOCK_UMC=0, RAS_TELEMETRY_GPU_BLOCK_SDMA, RAS_TELEMETRY_GPU_BLOCK_GFX, RAS_TELEMETRY_GPU_BLOCK_MMHUB, RAS_TELEMETRY_GPU_BLOCK_ATHUB, RAS_TELEMETRY_GPU_BLOCK_PCIE_BIF, RAS_TELEMETRY_GPU_BLOCK_HDP, RAS_TELEMETRY_GPU_BLOCK_XGMI_WAFL, RAS_TELEMETRY_GPU_BLOCK_DF, RAS_TELEMETRY_GPU_BLOCK_SMN, RAS_TELEMETRY_GPU_BLOCK_SEM, RAS_TELEMETRY_GPU_BLOCK_MP0, RAS_TELEMETRY_GPU_BLOCK_MP1, RAS_TELEMETRY_GPU_BLOCK_FUSE, RAS_TELEMETRY_GPU_BLOCK_MCA, RAS_TELEMETRY_GPU_BLOCK_VCN, RAS_TELEMETRY_GPU_BLOCK_JPEG, RAS_TELEMETRY_GPU_BLOCK_IH, RAS_TELEMETRY_GPU_BLOCK_MPIO, RAS_TELEMETRY_GPU_BLOCK_COUNT }
#[repr(C)] pub struct AmdSriovRasTelemetryHeader { pub checksum:u32,pub used_size:u32,pub reserved:[u32;2] }
#[repr(C)] pub struct AmdSriovRasTelemetryErrorCount { pub block:[[u32;12];19] }
#[repr(C)] pub struct AmdSriovRasCperDump { pub more:u32,pub overflow_count:u64,pub count:u64,pub wptr:u64,pub buf:[u32;0] }
#[repr(C)] pub struct AmdSriovRasChkCriti { pub hit:u32 }
#[repr(C)] pub union AmdSriovRasHostPush { pub error_count:AmdSriovRasTelemetryErrorCount,pub cper_dump:AmdSriovRasCperDump,pub chk_criti:AmdSriovRasChkCriti }
pub const AMD_SRIOV_UNIRAS_BLOCKS_BUF_SIZE:usize=4096; pub const AMD_SRIOV_UNIRAS_CMD_MAX_SIZE:usize=4096*13;
#[repr(C)] pub struct AmdSriovUnirasSharedMem { pub blocks_ecc_buf:[u8;AMD_SRIOV_UNIRAS_BLOCKS_BUF_SIZE],pub cmd_buf:[u8;AMD_SRIOV_UNIRAS_CMD_MAX_SIZE] }
#[repr(C)] pub struct AmdsriovRasTelemetry { pub header:AmdSriovRasTelemetryHeader,pub body:AmdSriovRasHostPush,pub uniras_shared_mem:AmdSriovUnirasSharedMem }
#[repr(u32)] pub enum AmdSriovGpuInitDataVersion { GPU_INIT_DATA_READY_V1=1 }
extern "C" { pub fn amd_sriov_msg_checksum(obj:*mut core::ffi::c_void,obj_size:core::ffi::c_ulong,key:core::ffi::c_uint,checksum:core::ffi::c_uint)->core::ffi::c_uint; }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
