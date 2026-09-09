/* SPDX-License-Identifier: GPL-2.0-or-later */
/* Bluetooth support for Intel devices */

#[repr(u8)]
pub enum IntelTlvType {
    IntelTlvCnviTop = 0x10,
    IntelTlvCnvrTop,
    IntelTlvCnviBt,
    IntelTlvCnvrBt,
    IntelTlvCnviOtp,
    IntelTlvCnvrOtp,
    IntelTlvDevRevId,
    IntelTlvUsbVendorId,
    IntelTlvUsbProductId,
    IntelTlvPcieVendorId,
    IntelTlvPcieDeviceId,
    IntelTlvPcieSubsystemId,
    IntelTlvImageType,
    IntelTlvTimeStamp,
    IntelTlvBuildType,
    IntelTlvBuildNum,
    IntelTlvFwBuildProduct,
    IntelTlvFwBuildHw,
    IntelTlvFwStep,
    IntelTlvBtSpec,
    IntelTlvMfgName,
    IntelTlvHciRev,
    IntelTlvLmpSubver,
    IntelTlvOtpPatchVer,
    IntelTlvSecureBoot,
    IntelTlvKeyFromHdr,
    IntelTlvOtpLock,
    IntelTlvApiLock,
    IntelTlvDebugLock,
    IntelTlvMinFw,
    IntelTlvLimitedCce,
    IntelTlvSbeType,
    IntelTlvOtpBdaddr,
    IntelTlvUnlockedState,
    IntelTlvGitSha1,
    IntelTlvFwId = 0x50,
}

#[repr(C, packed)]
pub struct IntelTlv { pub r#type: u8, pub len: u8, pub val: [u8; 0] }

pub const BTINTEL_HCI_OP_RESET: u16 = 0xfc01;
pub const BTINTEL_HCI_OP_DEBUG: u16 = 0xfcd9;
pub const BTINTEL_CNVI_BLAZARI: u16 = 0x900;
pub const BTINTEL_CNVI_BLAZARIW: u16 = 0x901;
pub const BTINTEL_CNVI_GAP: u16 = 0x910;
pub const BTINTEL_CNVI_BLAZARU: u16 = 0x930;
pub const BTINTEL_CNVI_SCP: u16 = 0xA00;
pub const BTINTEL_CNVI_SCP2: u16 = 0xA10;
pub const BTINTEL_CNVI_SCP2F: u16 = 0xA20;
pub const BTINTEL_CNVR_FMP2: u16 = 0x910;
pub const BTINTEL_CNVR_WHP2: u16 = 0xA10;
pub const BTINTEL_IMG_BOOTLOADER: u8 = 0x01;
pub const BTINTEL_IMG_IML: u8 = 0x02;
pub const BTINTEL_IMG_OP: u8 = 0x03;
pub const BTINTEL_FWID_MAXLEN: usize = 64;
pub const BTINTEL_HWID_GAP: u8 = 0x1c;
pub const BTINTEL_HWID_BZRI: u8 = 0x1e;
pub const BTINTEL_HWID_BZRU: u8 = 0x1d;
pub const BTINTEL_HWID_SCP: u8 = 0x1f;
pub const BTINTEL_HWID_SCP2: u8 = 0x20;
pub const BTINTEL_HWID_SCP2F: u8 = 0x21;
pub const BTINTEL_HWID_BZRIW: u8 = 0x22;

extern "C" { pub static btintel_guid_dsm: guid_t; }

#[repr(C)]
pub struct intel_version_tlv {
    pub cnvi_top: u32, pub cnvr_top: u32, pub cnvi_bt: u32, pub cnvr_bt: u32,
    pub dev_rev_id: u16, pub img_type: u8, pub timestamp: u16, pub build_type: u8,
    pub build_num: u32, pub secure_boot: u8, pub otp_lock: u8, pub api_lock: u8,
    pub debug_lock: u8, pub min_fw_build_nn: u8, pub min_fw_build_cw: u8,
    pub min_fw_build_yy: u8, pub limited_cce: u8, pub sbe_type: u8, pub git_sha1: u32,
    pub fw_id: [u8; BTINTEL_FWID_MAXLEN], pub otp_bd_addr: bdaddr_t,
}
#[repr(C, packed)] pub struct intel_version { pub status:u8, pub hw_platform:u8, pub hw_variant:u8, pub hw_revision:u8, pub fw_variant:u8, pub fw_revision:u8, pub fw_build_num:u8, pub fw_build_ww:u8, pub fw_build_yy:u8, pub fw_patch_num:u8 }
#[repr(C, packed)] pub struct intel_boot_params { pub status:u8, pub otp_format:u8, pub otp_content:u8, pub otp_patch:u8, pub dev_revid:__le16, pub secure_boot:u8, pub key_from_hdr:u8, pub key_type:u8, pub otp_lock:u8, pub api_lock:u8, pub debug_lock:u8, pub otp_bdaddr:bdaddr_t, pub min_fw_build_nn:u8, pub min_fw_build_cw:u8, pub min_fw_build_yy:u8, pub limited_cce:u8, pub unlocked_state:u8 }
#[repr(C, packed)] pub struct intel_bootup { pub zero:u8, pub num_cmds:u8, pub source:u8, pub reset_type:u8, pub reset_reason:u8, pub ddc_status:u8 }
#[repr(C, packed)] pub struct intel_secure_send_result { pub result:u8, pub opcode:__le16, pub status:u8 }
#[repr(C, packed)] pub struct intel_reset { pub reset_type:u8, pub patch_enable:u8, pub ddc_reload:u8, pub boot_option:u8, pub boot_param:__le32 }
#[repr(C, packed)] pub struct intel_debug_features { pub page1:[u8;16] }
#[repr(C, packed)] pub struct intel_offload_use_cases { pub status:u8, pub preset:[u8;8] }
pub const INTEL_OP_PPAG_CMD:u16 = 0xFE0B;
#[repr(C, packed)] pub struct hci_ppag_enable_cmd { pub ppag_enable_flags:__le32 }
pub const INTEL_TLV_TYPE_ID:u8=0x01; pub const INTEL_TLV_SYSTEM_EXCEPTION:u8=0; pub const INTEL_TLV_FATAL_EXCEPTION:u8=1; pub const INTEL_TLV_DEBUG_EXCEPTION:u8=2; pub const INTEL_TLV_TEST_EXCEPTION:u8=0xDE;
#[repr(C, packed)] pub struct btintel_cp_ddc_write { pub len:u8, pub id:__le16, pub data:[u8;0] }
#[repr(C)] pub struct btintel_sar_inc_pwr { pub revision:u8, pub bt_sar_bios:u32, pub inc_power_mode:u32, pub sar_2400_chain_a:u8, pub br:u8, pub edr2:u8, pub edr3:u8, pub le:u8, pub le_2mhz:u8, pub le_lr:u8 }
#[repr(C)] pub struct btintel_sar_band_limits { pub subband_2g4:u8, pub subband_5g2:u8, pub subband_5g8_5g9:u8, pub subband_6g1:u8, pub subband_6g3:u8 }
#[repr(C)] pub struct btintel_sar_rev2 { pub revision:u8, pub bt_sar_bios:u32, pub inc_power_mode:u32, pub chain_a:btintel_sar_band_limits, pub chain_b:btintel_sar_band_limits }
#[inline] pub const fn intel_hw_platform(cnvx_bt:u32)->u8 { ((cnvx_bt & 0x0000ff00)>>8) as u8 }
#[inline] pub const fn intel_hw_variant(cnvx_bt:u32)->u8 { ((cnvx_bt & 0x003f0000)>>16) as u8 }
#[inline] pub const fn intel_cnvx_top_type(cnvx_top:u32)->u32 { cnvx_top & 0x00000fff }
#[inline] pub const fn intel_cnvx_top_step(cnvx_top:u32)->u32 { (cnvx_top & 0x0f000000)>>24 }
#[inline] pub const fn intel_cnvx_top_pack_swab(t:u16,s:u16)->u16 { ((t<<4)|s).swap_bytes() }

#[repr(usize)] pub enum IntelFlag { IntelBootloader, IntelDownloading, IntelFirmwareLoaded, IntelFirmwareFailed, IntelBooting, IntelBrokenInitialNcmd, IntelBrokenShutdownLed, IntelRomLegacy, IntelRomLegacyNoWbsSupport, IntelAcpiResetActive, IntelWaitForD0, __IntelNumFlags }
#[repr(C)] pub struct btintel_data { pub flags: [u64; 1], pub acpi_reset_method: Option<unsafe extern "C" fn(*mut hci_dev)->c_int> }

/* The following declarations are conditional on CONFIG_BT_INTEL or CONFIG_BT_INTEL_PCIE. */
extern "C" {
    pub fn btintel_check_bdaddr(hdev:*mut hci_dev)->c_int; pub fn btintel_enter_mfg(hdev:*mut hci_dev)->c_int; pub fn btintel_exit_mfg(hdev:*mut hci_dev, reset:bool, patched:bool)->c_int; pub fn btintel_set_bdaddr(hdev:*mut hci_dev, bdaddr:*const bdaddr_t)->c_int; pub fn btintel_set_diag(hdev:*mut hci_dev, enable:bool)->c_int;
    pub fn btintel_version_info(hdev:*mut hci_dev, ver:*mut intel_version)->c_int; pub fn btintel_load_ddc_config(hdev:*mut hci_dev, ddc_name:*const c_char)->c_int; pub fn btintel_set_event_mask_mfg(hdev:*mut hci_dev, debug:bool)->c_int; pub fn btintel_read_version(hdev:*mut hci_dev, ver:*mut intel_version)->c_int;
    pub fn btintel_regmap_init(hdev:*mut hci_dev, opcode_read:u16, opcode_write:u16)->*mut regmap; pub fn btintel_send_intel_reset(hdev:*mut hci_dev, boot_param:u32)->c_int; pub fn btintel_read_boot_params(hdev:*mut hci_dev, params:*mut intel_boot_params)->c_int; pub fn btintel_download_firmware(dev:*mut hci_dev, ver:*mut intel_version, fw:*const firmware, boot_param:*mut u32)->c_int; pub fn btintel_configure_setup(hdev:*mut hci_dev, driver_name:*const c_char)->c_int;
    pub fn btintel_recv_event(hdev:*mut hci_dev, skb:*mut sk_buff)->c_int; pub fn btintel_bootup(hdev:*mut hci_dev, ptr:*const c_void, len:c_uint); pub fn btintel_secure_send_result(hdev:*mut hci_dev, ptr:*const c_void, len:c_uint); pub fn btintel_set_quality_report(hdev:*mut hci_dev, enable:bool)->c_int; pub fn btintel_version_info_tlv(hdev:*mut hci_dev, version:*mut intel_version_tlv)->c_int; pub fn btintel_parse_version_tlv(hdev:*mut hci_dev, version:*mut intel_version_tlv, skb:*mut sk_buff)->c_int; pub fn btintel_set_msft_opcode(hdev:*mut hci_dev, hw_variant:u8); pub fn btintel_bootloader_setup_tlv(hdev:*mut hci_dev, ver:*mut intel_version_tlv)->c_int; pub fn btintel_shutdown_combined(hdev:*mut hci_dev)->c_int; pub fn btintel_hw_error(hdev:*mut hci_dev, code:u8); pub fn btintel_print_fseq_info(hdev:*mut hci_dev); pub fn btintel_acpi_reset_method(hdev:*mut hci_dev)->c_int;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
