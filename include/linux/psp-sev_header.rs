/* SPDX-License-Identifier: GPL-2.0-only */
/* AMD Secure Encrypted Virtualization (SEV) driver interface. */
// Dependency supplied by the corresponding UAPI header.

#![allow(non_camel_case_types, non_snake_case, dead_code)]

pub const SEV_POLICY_MASK_NODBG: u32 = 1 << 0;
pub const SEV_POLICY_MASK_NOKS: u32 = 1 << 1;
pub const SEV_POLICY_MASK_ES: u32 = 1 << 2;
pub const SEV_POLICY_MASK_NOSEND: u32 = 1 << 3;
pub const SEV_POLICY_MASK_DOMAIN: u32 = 1 << 4;
pub const SEV_POLICY_MASK_SEV: u32 = 1 << 5;
pub const SEV_POLICY_MASK_API_MAJOR: u32 = 0xff << 16;
pub const SEV_POLICY_MASK_API_MINOR: u32 = 0xff << 24;
pub const SNP_POLICY_MASK_API_MINOR: u64 = 0xff;
pub const SNP_POLICY_MASK_API_MAJOR: u64 = 0xff << 8;
pub const SNP_POLICY_MASK_SMT: u64 = 1 << 16;
pub const SNP_POLICY_MASK_RSVD_MBO: u64 = 1 << 17;
pub const SNP_POLICY_MASK_MIGRATE_MA: u64 = 1 << 18;
pub const SNP_POLICY_MASK_DEBUG: u64 = 1 << 19;
pub const SNP_POLICY_MASK_SINGLE_SOCKET: u64 = 1 << 20;
pub const SNP_POLICY_MASK_CXL_ALLOW: u64 = 1 << 21;
pub const SNP_POLICY_MASK_MEM_AES_256_XTS: u64 = 1 << 22;
pub const SNP_POLICY_MASK_RAPL_DIS: u64 = 1 << 23;
pub const SNP_POLICY_MASK_CIPHERTEXT_HIDING_DRAM: u64 = 1 << 24;
pub const SNP_POLICY_MASK_PAGE_SWAP_DISABLE: u64 = 1 << 25;
pub const SNP_POLICY_MASK_BASE: u64 = SNP_POLICY_MASK_API_MINOR | SNP_POLICY_MASK_API_MAJOR | SNP_POLICY_MASK_SMT | SNP_POLICY_MASK_RSVD_MBO | SNP_POLICY_MASK_MIGRATE_MA | SNP_POLICY_MASK_DEBUG | SNP_POLICY_MASK_SINGLE_SOCKET;
pub const SEV_FW_BLOB_MAX_SIZE: u32 = 0x4000;

#[repr(u32)] #[derive(Copy, Clone)] pub enum sev_state { SEV_STATE_UNINIT=0, SEV_STATE_INIT=1, SEV_STATE_WORKING=2, SEV_STATE_MAX }
#[repr(u32)] #[derive(Copy, Clone)] pub enum sev_cmd {
 SEV_CMD_INIT=0x001, SEV_CMD_SHUTDOWN=0x002, SEV_CMD_FACTORY_RESET=0x003, SEV_CMD_PLATFORM_STATUS=0x004, SEV_CMD_PEK_GEN=0x005, SEV_CMD_PEK_CSR=0x006, SEV_CMD_PEK_CERT_IMPORT=0x007, SEV_CMD_PDH_CERT_EXPORT=0x008, SEV_CMD_PDH_GEN=0x009, SEV_CMD_DF_FLUSH=0x00a, SEV_CMD_DOWNLOAD_FIRMWARE=0x00b, SEV_CMD_GET_ID=0x00c, SEV_CMD_INIT_EX=0x00d,
 SEV_CMD_DECOMMISSION=0x020, SEV_CMD_ACTIVATE=0x021, SEV_CMD_DEACTIVATE=0x022, SEV_CMD_GUEST_STATUS=0x023,
 SEV_CMD_LAUNCH_START=0x030, SEV_CMD_LAUNCH_UPDATE_DATA=0x031, SEV_CMD_LAUNCH_UPDATE_VMSA=0x032, SEV_CMD_LAUNCH_MEASURE=0x033, SEV_CMD_LAUNCH_UPDATE_SECRET=0x034, SEV_CMD_LAUNCH_FINISH=0x035, SEV_CMD_ATTESTATION_REPORT=0x036,
 SEV_CMD_SEND_START=0x040, SEV_CMD_SEND_UPDATE_DATA=0x041, SEV_CMD_SEND_UPDATE_VMSA=0x042, SEV_CMD_SEND_FINISH=0x043, SEV_CMD_SEND_CANCEL=0x044,
 SEV_CMD_RECEIVE_START=0x050, SEV_CMD_RECEIVE_UPDATE_DATA=0x051, SEV_CMD_RECEIVE_UPDATE_VMSA=0x052, SEV_CMD_RECEIVE_FINISH=0x053,
 SEV_CMD_DBG_DECRYPT=0x060, SEV_CMD_DBG_ENCRYPT=0x061,
 SEV_CMD_SNP_INIT=0x081, SEV_CMD_SNP_SHUTDOWN=0x082, SEV_CMD_SNP_PLATFORM_STATUS=0x083, SEV_CMD_SNP_DF_FLUSH=0x084, SEV_CMD_SNP_INIT_EX=0x085, SEV_CMD_SNP_SHUTDOWN_EX=0x086, SEV_CMD_SNP_DECOMMISSION=0x090, SEV_CMD_SNP_ACTIVATE=0x091, SEV_CMD_SNP_GUEST_STATUS=0x092, SEV_CMD_SNP_GCTX_CREATE=0x093, SEV_CMD_SNP_GUEST_REQUEST=0x094, SEV_CMD_SNP_ACTIVATE_EX=0x095, SEV_CMD_SNP_LAUNCH_START=0x0a0, SEV_CMD_SNP_LAUNCH_UPDATE=0x0a1, SEV_CMD_SNP_LAUNCH_FINISH=0x0a2, SEV_CMD_SNP_DBG_DECRYPT=0x0b0, SEV_CMD_SNP_DBG_ENCRYPT=0x0b1, SEV_CMD_SNP_VERIFY_MITIGATION=0x0b2, SEV_CMD_SNP_PAGE_SWAP_OUT=0x0c0, SEV_CMD_SNP_PAGE_SWAP_IN=0x0c1, SEV_CMD_SNP_PAGE_MOVE=0x0c2, SEV_CMD_SNP_PAGE_MD_INIT=0x0c3, SEV_CMD_SNP_PAGE_SET_STATE=0x0c6, SEV_CMD_SNP_PAGE_RECLAIM=0x0c7, SEV_CMD_SNP_PAGE_UNSMASH=0x0c8, SEV_CMD_SNP_CONFIG=0x0c9, SEV_CMD_SNP_DOWNLOAD_FIRMWARE_EX=0x0ca, SEV_CMD_SNP_COMMIT=0x0cb, SEV_CMD_SNP_VLEK_LOAD=0x0cd, SEV_CMD_SNP_FEATURE_INFO=0x0ce,
 SEV_CMD_TIO_STATUS=0x0d0, SEV_CMD_TIO_INIT=0x0d1, SEV_CMD_TIO_DEV_CREATE=0x0d2, SEV_CMD_TIO_DEV_RECLAIM=0x0d3, SEV_CMD_TIO_DEV_CONNECT=0x0d4, SEV_CMD_TIO_DEV_DISCONNECT=0x0d5, SEV_CMD_MAX }

#[repr(C, packed)] pub struct sev_data_init { pub flags:u32, pub reserved:u32, pub tmr_address:u64, pub tmr_len:u32 }
#[repr(C, packed)] pub struct sev_data_init_ex { pub length:u32, pub flags:u32, pub tmr_address:u64, pub tmr_len:u32, pub reserved:u32, pub nv_address:u64, pub nv_len:u32 }
pub const SEV_INIT_FLAGS_SEV_ES:u32=0x01;
#[repr(C, packed)] pub struct sev_data_pek_csr { pub address:u64, pub len:u32 }
#[repr(C, packed)] pub struct sev_data_pek_cert_import { pub pek_cert_address:u64,pub pek_cert_len:u32,pub reserved:u32,pub oca_cert_address:u64,pub oca_cert_len:u32 }
#[repr(C, packed)] pub struct sev_data_download_firmware { pub address:u64,pub len:u32 }
#[repr(C, packed)] pub struct sev_data_get_id { pub address:u64,pub len:u32 }
#[repr(C, packed)] pub struct sev_data_pdh_cert_export { pub pdh_cert_address:u64,pub pdh_cert_len:u32,pub reserved:u32,pub cert_chain_address:u64,pub cert_chain_len:u32 }
#[repr(C, packed)] pub struct sev_data_decommission { pub handle:u32 }
#[repr(C, packed)] pub struct sev_data_activate { pub handle:u32,pub asid:u32 }
#[repr(C, packed)] pub struct sev_data_deactivate { pub handle:u32 }
#[repr(C, packed)] pub struct sev_data_guest_status { pub handle:u32,pub policy:u32,pub asid:u32,pub state:u8 }
#[repr(C, packed)] pub struct sev_data_launch_start { pub handle:u32,pub policy:u32,pub dh_cert_address:u64,pub dh_cert_len:u32,pub reserved:u32,pub session_address:u64,pub session_len:u32 }
#[repr(C, packed)] pub struct sev_data_launch_update_data { pub handle:u32,pub reserved:u32,pub address:u64,pub len:u32 }
pub type sev_data_launch_update_vmsa=sev_data_launch_update_data;
pub type sev_data_launch_measure=sev_data_launch_update_data;
#[repr(C, packed)] pub struct sev_data_launch_secret { pub handle:u32,pub reserved1:u32,pub hdr_address:u64,pub hdr_len:u32,pub reserved2:u32,pub guest_address:u64,pub guest_len:u32,pub reserved3:u32,pub trans_address:u64,pub trans_len:u32 }
#[repr(C, packed)] pub struct sev_data_launch_finish { pub handle:u32 }
#[repr(C, packed)] pub struct sev_data_send_start { pub handle:u32,pub policy:u32,pub pdh_cert_address:u64,pub pdh_cert_len:u32,pub reserved1:u32,pub plat_certs_address:u64,pub plat_certs_len:u32,pub reserved2:u32,pub amd_certs_address:u64,pub amd_certs_len:u32,pub reserved3:u32,pub session_address:u64,pub session_len:u32 }
#[repr(C, packed)] pub struct sev_data_send_update_data { pub handle:u32,pub reserved1:u32,pub hdr_address:u64,pub hdr_len:u32,pub reserved2:u32,pub guest_address:u64,pub guest_len:u32,pub reserved3:u32,pub trans_address:u64,pub trans_len:u32 }
#[repr(C, packed)] pub struct sev_data_send_update_vmsa { pub handle:u32,pub hdr_address:u64,pub hdr_len:u32,pub reserved2:u32,pub guest_address:u64,pub guest_len:u32,pub reserved3:u32,pub trans_address:u64,pub trans_len:u32 }
#[repr(C, packed)] pub struct sev_data_send_finish { pub handle:u32 }
pub type sev_data_send_cancel=sev_data_send_finish;
#[repr(C, packed)] pub struct sev_data_receive_start { pub handle:u32,pub policy:u32,pub pdh_cert_address:u64,pub pdh_cert_len:u32,pub reserved1:u32,pub session_address:u64,pub session_len:u32 }
pub type sev_data_receive_update_data=sev_data_send_update_data;
pub type sev_data_receive_update_vmsa=sev_data_send_update_data;
pub type sev_data_receive_finish=sev_data_send_finish;
#[repr(C, packed)] pub struct sev_data_dbg { pub handle:u32,pub reserved:u32,pub src_addr:u64,pub dst_addr:u64,pub len:u32 }
#[repr(C, packed)] pub struct sev_data_attestation_report { pub handle:u32,pub reserved:u32,pub address:u64,pub mnonce:[u8;16],pub len:u32 }
#[repr(C, packed)] pub struct sev_data_snp_download_firmware { pub address:u64,pub len:u32 }
#[repr(C, packed)] pub struct sev_data_snp_activate { pub gctx_paddr:u64,pub asid:u32 }
#[repr(C, packed)] pub struct sev_data_snp_addr { pub address:u64 }
#[repr(C, packed)] pub struct sev_data_snp_launch_start { pub gctx_paddr:u64,pub policy:u64,pub ma_gctx_paddr:u64,pub ma_en_imi_en_rsvd:u32,pub desired_tsc_khz:u32,pub gosvw:[u8;16] }
#[repr(u32)] pub enum snp_page_type { SNP_PAGE_TYPE_NORMAL=1,SNP_PAGE_TYPE_VMSA=2,SNP_PAGE_TYPE_ZERO=3,SNP_PAGE_TYPE_UNMEASURED=4,SNP_PAGE_TYPE_SECRET=5,SNP_PAGE_TYPE_CPUID=6,SNP_PAGE_TYPE_MAX }
#[repr(C, packed)] pub struct sev_data_snp_launch_update { pub gctx_paddr:u64,pub page_size_page_type_imi_page_rsvd:u32,pub rsvd2:u32,pub address:u64,pub rsvd3_vmpl1_vmpl2_vmpl3:u32,pub rsvd4:u32 }
#[repr(C, packed)] pub struct sev_data_snp_launch_finish { pub gctx_paddr:u64,pub id_block_paddr:u64,pub id_auth_paddr:u64,pub id_block_en_auth_key_en_vcek_disabled:u8,pub rsvd:[u8;61],pub host_data:[u8;32] }
#[repr(C, packed)] pub struct sev_data_snp_guest_status { pub gctx_paddr:u64,pub address:u64 }
#[repr(C, packed)] pub struct sev_data_snp_page_reclaim { pub paddr:u64 }
pub type sev_data_snp_page_unsmash=sev_data_snp_page_reclaim;
#[repr(C, packed)] pub struct sev_data_snp_dbg { pub gctx_paddr:u64,pub src_addr:u64,pub dst_addr:u64 }
#[repr(C, packed)] pub struct sev_data_snp_guest_request { pub gctx_paddr:u64,pub req_paddr:u64,pub res_paddr:u64 }
#[repr(C, packed)] pub struct sev_data_snp_init_ex { pub init_rmp:u32,pub list_paddr_en:u32,pub rapl_dis:u32,pub ciphertext_hiding_en:u32,pub tio_en:u32,pub rsvd:u32,pub rsvd1:u32,pub list_paddr:u64,pub max_snp_asid:u16,pub rsvd2:[u8;46] }
#[repr(C, packed)] pub struct sev_data_range { pub base:u64,pub page_count:u32,pub rsvd:u32 }
#[repr(C, packed)] pub struct sev_data_range_list { pub num_elements:u32,pub rsvd:u32,pub ranges:[sev_data_range;0] }
#[repr(C, packed)] pub struct sev_data_snp_shutdown_ex { pub len:u32,pub iommu_snp_shutdown_x86_snp_shutdown_rsvd1:u32 }
#[repr(C)] pub struct sev_platform_init_args { pub error:i32,pub probe:bool,pub max_snp_asid:u32 }
#[repr(C, packed)] pub struct sev_data_snp_commit { pub len:u32 }
#[repr(C, packed)] pub struct sev_data_snp_feature_info { pub length:u32,pub ecx_in:u32,pub feature_info_paddr:u64 }
#[repr(C, packed)] pub struct snp_feature_info { pub eax:u32,pub ebx:u32,pub ecx:u32,pub edx:u32 }
pub const SNP_X86_SHUTDOWN_SUPPORTED:u32=1<<1; pub const SNP_RAPL_DISABLE_SUPPORTED:u32=1<<2; pub const SNP_CIPHER_TEXT_HIDING_SUPPORTED:u32=1<<3; pub const SNP_AES_256_XTS_POLICY_SUPPORTED:u32=1<<4; pub const SNP_CXL_ALLOW_POLICY_SUPPORTED:u32=1<<5; pub const SNP_VERIFY_MITIGATION_SUPPORTED:u32=1<<13; pub const SNP_SEV_TIO_SUPPORTED:u32=1<<1; pub const SNP_MIT_SUBCMD_REQ_STATUS:u16=0; pub const SNP_MIT_SUBCMD_REQ_VERIFY:u16=1;
#[repr(C, packed)] pub struct sev_data_snp_verify_mitigation { pub length:u32,pub subcommand:u16,pub rsvd:u16,pub vector:u64,pub dst_paddr_en:u32,pub src_paddr_en:u32,pub rsvd1:u32,pub rsvd2:[u8;4],pub src_paddr:u64,pub dst_paddr:u64,pub rsvd3:[u8;24] }
#[repr(C, packed)] pub struct sev_data_snp_verify_mitigation_dst { pub mit_verified_vector:u64,pub mit_supported_vector:u64,pub mit_failure_status:u32 }
#[repr(C)] pub struct sev_snp_tcb_version_genoa_milan { pub boot_loader:u8,pub tee:u8,pub reserved:[u8;4],pub snp:u8,pub microcode:u8 }
#[repr(C)] pub struct sev_snp_tcb_version_turin { pub fmc:u8,pub boot_loader:u8,pub tee:u8,pub snp:u8,pub reserved:[u8;3],pub microcode:u8 }

/* CONFIG_CRYPTO_DEV_SP_PSP declarations. */
extern "C" { pub fn sev_module_init()->i32; pub fn sev_platform_init(args:*mut sev_platform_init_args)->i32; pub fn sev_platform_status(status:*mut sev_user_data_status,error:*mut i32)->i32; pub fn sev_issue_cmd_external_user(filep:*mut file,id:u32,data:*mut core::ffi::c_void,error:*mut i32)->i32; pub fn sev_guest_deactivate(data:*mut sev_data_deactivate,error:*mut i32)->i32; pub fn sev_guest_activate(data:*mut sev_data_activate,error:*mut i32)->i32; pub fn sev_guest_df_flush(error:*mut i32)->i32; pub fn sev_guest_decommission(data:*mut sev_data_decommission,error:*mut i32)->i32; pub fn sev_do_cmd(cmd:i32,data:*mut core::ffi::c_void,psp_ret:*mut i32)->i32; pub fn psp_copy_user_blob(uaddr:u64,len:u32)->*mut core::ffi::c_void; pub fn snp_alloc_firmware_page(mask:gfp_t)->*mut core::ffi::c_void; pub fn snp_reclaim_pages(paddr:usize,npages:u32,locked:bool)->i32; pub fn snp_free_firmware_page(addr:*mut core::ffi::c_void); pub fn sev_platform_shutdown(); pub fn sev_is_snp_ciphertext_hiding_supported()->bool; pub fn sev_get_snp_policy_bits()->u64; pub fn sev_firmware_supported_vm_types()->i32; }
extern "C" { type sev_user_data_status; type file; type gfp_t; }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
