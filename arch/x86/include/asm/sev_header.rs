/* SPDX-License-Identifier: GPL-2.0 */
/* AMD Encrypted Register State Support */

// C header dependencies are supplied by the surrounding kernel translation.

pub const GHCB_PROTOCOL_MIN: u64 = 1;
pub const GHCB_PROTOCOL_MAX: u64 = 2;
pub const GHCB_DEFAULT_USAGE: u64 = 0;

#[inline]
pub unsafe fn vmgexit() {
    core::arch::asm!("rep; vmmcall", options(nostack));
}

pub enum BootParams {}
pub enum PtRegs {}
pub enum RealModeHeader {}
pub enum Insn {}
pub enum Ghcb {}
pub enum Pgd {}
pub enum PteT {}
pub enum PgprotT {}
pub enum AesGcmKey {}
pub enum PhysAddrT {}

#[repr(C)]
#[derive(Copy, Clone)]
pub enum EsResult { EsOk, EsUnsupported, EsVmmError, EsDecodeFailed, EsException, EsRetry }

#[repr(C)]
pub struct EsFaultInfo { pub vector: usize, pub error_code: usize, pub cr2: usize }
#[repr(C)]
pub struct EsEmCtxt { pub regs: *mut PtRegs, pub insn: Insn, pub fi: EsFaultInfo }

pub const CC_BLOB_SEV_HDR_MAGIC: u32 = 0x45444d41;
#[repr(C, packed)]
pub struct CcBlobSevInfo { pub magic: u32, pub version: u16, pub reserved: u16, pub secrets_phys: u64, pub secrets_len: u32, pub rsvd1: u32, pub cpuid_phys: u64, pub cpuid_len: u32, pub rsvd2: u32 }

pub unsafe extern "C" { pub fn do_vc_no_ghcb(regs: *mut PtRegs, exit_code: usize); }

#[inline]
pub const fn lower_bits(val: u64, bits: u32) -> u64 { val & ((1u64 << bits) - 1) }

#[repr(C, packed)]
pub struct SnpCpuidFn { pub eax_in: u32, pub ecx_in: u32, pub xcr0_in: u64, pub xss_in: u64, pub eax: u32, pub ebx: u32, pub ecx: u32, pub edx: u32, pub reserved: u64 }
pub const SNP_CPUID_COUNT_MAX: usize = 64;
#[repr(C, packed)]
pub struct SnpCpuidTable { pub count: u32, pub reserved1: u32, pub reserved2: u64, pub fn_: [SnpCpuidFn; SNP_CPUID_COUNT_MAX] }

pub const PVALIDATE_FAIL_SIZEMISMATCH: i32 = 6;
pub const PVALIDATE_FAIL_NOUPDATE: i32 = 255;
pub const RMPUPDATE_FAIL_OVERLAP: i32 = 4;
pub const PSMASH_FAIL_INUSE: i32 = 3;
pub const RMP_PG_SIZE_4K: u32 = 0;
pub const RMP_PG_SIZE_2M: u32 = 1;
pub const RMPADJUST_VMSA_PAGE_BIT: u64 = 1 << 16;

#[repr(C, packed)]
pub struct RmpState { pub gpa: u64, pub assigned: u8, pub pagesize: u8, pub immutable: u8, pub rsvd: u8, pub asid: u32 }
#[repr(C)]
pub struct SnpReqData { pub req_gpa: usize, pub resp_gpa: usize, pub data_gpa: usize, pub data_npages: u32 }

pub const MAX_AUTHTAG_LEN: usize = 32;
pub const AUTHTAG_LEN: usize = 16;
pub const AAD_LEN: usize = 48;
pub const MSG_HDR_VER: u32 = 1;
pub const SNP_TSC_INFO_REQ_SZ: usize = 128;
pub const VMPCK_KEY_LEN: usize = 32;

#[repr(C)]
pub enum MsgType { SnpMsgTypeInvalid = 0, SnpMsgCpuidReq, SnpMsgCpuidRsp, SnpMsgKeyReq, SnpMsgKeyRsp, SnpMsgReportReq, SnpMsgReportRsp, SnpMsgExportReq, SnpMsgExportRsp, SnpMsgImportReq, SnpMsgImportRsp, SnpMsgAbsorbReq, SnpMsgAbsorbRsp, SnpMsgVmrkReq, SnpMsgVmrkRsp, SnpMsgTscInfoReq = 17, SnpMsgTscInfoRsp, SnpMsgTypeMax }
#[repr(C)] pub enum AeadAlgo { SnpAeadInvalid, SnpAeadAes256Gcm }

#[repr(C, packed)]
pub struct SnpGuestMsgHdr { pub authtag: [u8; MAX_AUTHTAG_LEN], pub msg_seqno: u64, pub rsvd1: [u8; 8], pub algo: u8, pub hdr_version: u8, pub hdr_sz: u16, pub msg_type: u8, pub msg_version: u8, pub msg_sz: u16, pub rsvd2: u32, pub msg_vmpck: u8, pub rsvd3: [u8; 35] }
#[repr(C, packed)] pub struct SnpGuestMsg { pub hdr: SnpGuestMsgHdr, pub payload: [u8; PAGE_SIZE - core::mem::size_of::<SnpGuestMsgHdr>()] }
#[repr(C, packed)] pub struct SnpTscInfoReq { pub rsvd: [u8; SNP_TSC_INFO_REQ_SZ] }
#[repr(C, packed)] pub struct SnpTscInfoResp { pub status: u32, pub rsvd1: u32, pub tsc_scale: u64, pub tsc_offset: u64, pub tsc_factor: u32, pub rsvd2: [u8; 100] }

#[inline] pub const fn snp_scale_tsc_freq(freq: u64, factor: u64) -> u64 { freq - freq * factor / 100000 }

#[repr(C)] pub struct SnpGuestReq { pub req_buf: *mut core::ffi::c_void, pub req_sz: usize, pub resp_buf: *mut core::ffi::c_void, pub resp_sz: usize, pub exit_code: u64, pub exitinfo2: u64, pub vmpck_id: u32, pub msg_version: u8, pub msg_type: u8, pub input: SnpReqData, pub certs_data: *mut core::ffi::c_void }
#[repr(C, packed)] pub struct SecretsOsArea { pub msg_seqno_0: u32, pub msg_seqno_1: u32, pub msg_seqno_2: u32, pub msg_seqno_3: u32, pub ap_jump_table_pa: u64, pub rsvd: [u8; 40], pub guest_usage: [u8; 32] }
#[repr(C, packed)] pub struct SnpSecretsPage { pub version: u32, pub imien_rsvd1: u32, pub fms: u32, pub rsvd2: u32, pub gosvw: [u8; 16], pub vmpck0: [u8; VMPCK_KEY_LEN], pub vmpck1: [u8; VMPCK_KEY_LEN], pub vmpck2: [u8; VMPCK_KEY_LEN], pub vmpck3: [u8; VMPCK_KEY_LEN], pub os_area: SecretsOsArea, pub vmsa_tweak_bitmap: [u8; 64], pub svsm_base: u64, pub svsm_size: u64, pub svsm_caa: u64, pub svsm_max_version: u32, pub svsm_guest_vmpl: u8, pub rsvd3: [u8; 3], pub tsc_factor: u32, pub rsvd4: [u8; 3740] }

#[repr(C)] pub struct SnpMsgDesc { pub request: *mut SnpGuestMsg, pub response: *mut SnpGuestMsg, pub secret_request: SnpGuestMsg, pub secret_response: SnpGuestMsg, pub secrets: *mut SnpSecretsPage, pub gcm_key: *mut AesGcmKey, pub os_area_msg_seqno: *mut u32, pub vmpck: *mut u8, pub vmpck_id: i32 }
#[repr(C)] pub struct SvsmCa { pub call_pending: u8, pub mem_available: u8, pub rsvd1: [u8; 6], pub svsm_buffer: [u8; PAGE_SIZE - 8] }

pub const SVSM_SUCCESS: u32 = 0; pub const SVSM_ERR_INCOMPLETE: u32 = 0x80000000; pub const SVSM_ERR_UNSUPPORTED_PROTOCOL: u32 = 0x80000001; pub const SVSM_ERR_UNSUPPORTED_CALL: u32 = 0x80000002; pub const SVSM_ERR_INVALID_ADDRESS: u32 = 0x80000003; pub const SVSM_ERR_INVALID_FORMAT: u32 = 0x80000004; pub const SVSM_ERR_INVALID_PARAMETER: u32 = 0x80000005; pub const SVSM_ERR_INVALID_REQUEST: u32 = 0x80000006; pub const SVSM_ERR_BUSY: u32 = 0x80000007; pub const SVSM_PVALIDATE_FAIL_SIZEMISMATCH: u32 = 0x80001006;

#[repr(C)] pub struct SvsmPvalidateEntry { pub page_size_action_ignore_rsvd: u64, pub pfn: u64 }
#[repr(C)] pub struct SvsmPvalidateCall { pub num_entries: u16, pub cur_index: u16, pub rsvd1: [u8; 4], pub entry: [SvsmPvalidateEntry; 0] }
#[repr(C)] pub struct SvsmLocEntry { pub pa: u64, pub len: u32, pub rsvd: [u8; 4] }
#[repr(C)] pub struct SvsmAttestCall { pub report_buf: SvsmLocEntry, pub nonce: SvsmLocEntry, pub manifest_buf: SvsmLocEntry, pub certificates_buf: SvsmLocEntry, pub service_guid: [u8; 16], pub service_manifest_ver: u32, pub rsvd: [u8; 4] }
#[repr(C)] pub struct SvsmCall { pub caa: *mut SvsmCa, pub rax: u64, pub rcx: u64, pub rdx: u64, pub r8: u64, pub r9: u64, pub rax_out: u64, pub rcx_out: u64, pub rdx_out: u64, pub r8_out: u64, pub r9_out: u64 }

pub const SVSM_CORE_REMAP_CA: u64 = 0; pub const SVSM_CORE_PVALIDATE: u64 = 1; pub const SVSM_CORE_CREATE_VCPU: u64 = 2; pub const SVSM_CORE_DELETE_VCPU: u64 = 3;
pub const SVSM_ATTEST_SERVICES: u64 = 0; pub const SVSM_ATTEST_SINGLE_SERVICE: u64 = 1; pub const SVSM_VTPM_QUERY: u64 = 0; pub const SVSM_VTPM_CMD: u64 = 1;
#[inline] pub const fn svsm_core_call(x: u64) -> u64 { x } #[inline] pub const fn svsm_attest_call(x: u64) -> u64 { (1u64 << 32) | x } #[inline] pub const fn svsm_vtpm_call(x: u64) -> u64 { (2u64 << 32) | x }

// The remaining declarations and CONFIG_AMD_MEM_ENCRYPT/CONFIG_KVM_AMD_SEV branches
// depend on kernel-provided types, configuration symbols, and assembly operations.
// They are preserved as external interfaces below.
extern "C" {
    pub fn vc_no_ghcb();
    pub fn vc_boot_ghcb();
    pub fn handle_vc_boot_ghcb(regs: *mut PtRegs) -> bool;
    pub fn __sev_es_ist_enter(regs: *mut PtRegs);
    pub fn __sev_es_ist_exit();
    pub fn sev_es_setup_ap_jump_table(rmh: *mut RealModeHeader) -> i32;
    pub fn __sev_es_nmi_complete();
    pub fn sev_es_efi_map_ghcbs_cas(pgd: *mut Pgd) -> i32;
    pub fn setup_ghcb();
    pub fn snp_register_ghcb_early(paddr: usize);
    pub fn early_snp_set_memory_private(vaddr: usize, paddr: usize, npages: usize);
    pub fn early_snp_set_memory_shared(vaddr: usize, paddr: usize, npages: usize);
    pub fn snp_set_memory_shared(vaddr: usize, npages: usize);
    pub fn snp_set_memory_private(vaddr: usize, npages: usize);
    pub fn snp_set_wakeup_secondary_cpu();
    pub fn snp_dmi_setup();
    pub fn snp_accept_memory(start: u64, end: u64);
    pub fn snp_get_unsupported_features(status: u64) -> u64;
    pub fn sev_show_status();
    pub fn snp_kexec_finish();
    pub fn snp_kexec_begin();
    pub fn snp_svsm_vtpm_send_command(buffer: *mut u8) -> i32;
    pub fn snp_secure_tsc_prepare();
    pub fn snp_secure_tsc_init();
    pub fn savic_register_gpa(gpa: u64) -> EsResult;
    pub fn savic_unregister_gpa(gpa: *mut u64) -> EsResult;
    pub fn savic_ghcb_msr_read(reg: u32) -> u64;
    pub fn savic_ghcb_msr_write(reg: u32, value: u64);
    pub fn svsm_perform_msr_protocol(call: *mut SvsmCall) -> i32;
    pub fn __pi_svsm_perform_msr_protocol(call: *mut SvsmCall) -> i32;
    pub fn svsm_issue_call(call: *mut SvsmCall, pending: *mut u8);
    pub fn svsm_process_result_codes(call: *mut SvsmCall) -> i32;
    pub fn sev_es_terminate(set: u32, reason: u32) -> !;
    pub fn sev_es_negotiate_protocol() -> bool;
    pub fn sev_es_check_cpu_features() -> bool;
    pub fn sev_enable(bp: *mut BootParams);
    pub fn snp_init(bp: *mut BootParams) -> bool;
    pub fn sev_get_status() -> u64;
    pub fn snp_msg_init(mdesc: *mut SnpMsgDesc, vmpck_id: i32) -> i32;
    pub fn snp_msg_alloc() -> *mut SnpMsgDesc;
    pub fn snp_msg_free(mdesc: *mut SnpMsgDesc);
    pub fn snp_send_guest_request(mdesc: *mut SnpMsgDesc, req: *mut SnpGuestReq) -> i32;
    pub fn snp_probe_rmptable_info() -> bool;
    pub fn snp_rmptable_init() -> i32;
    pub fn snp_lookup_rmpentry(pfn: u64, assigned: *mut bool, level: *mut i32) -> i32;
    pub fn snp_dump_hva_rmpentry(address: usize);
    pub fn psmash(pfn: u64) -> i32;
    pub fn rmp_make_private(pfn: u64, gpa: u64, level: i32, asid: u32, immutable: bool) -> i32;
    pub fn rmp_make_shared(pfn: u64, level: i32) -> i32;
    pub fn __snp_leak_pages(pfn: u64, npages: u32, dump_rmp: bool);
    pub fn kdump_sev_callback();
    pub fn snp_fixup_e820_tables();
    pub fn snp_prepare() -> i32;
    pub fn snp_shutdown();
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
