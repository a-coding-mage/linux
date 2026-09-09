/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * Bluetooth support for Qualcomm Atheros ROME chips
 *
 * Copyright (c) 2015 The Linux Foundation. All rights reserved.
 */

pub const EDL_PATCH_CMD_OPCODE: u32 = 0xFC00;
pub const EDL_NVM_ACCESS_OPCODE: u32 = 0xFC0B;
pub const EDL_WRITE_BD_ADDR_OPCODE: u32 = 0xFC14;
pub const EDL_PATCH_CMD_LEN: u32 = 1;
pub const EDL_PATCH_VER_REQ_CMD: u32 = 0x19;
pub const EDL_PATCH_TLV_REQ_CMD: u32 = 0x1E;
pub const EDL_GET_BUILD_INFO_CMD: u32 = 0x20;
pub const EDL_GET_BID_REQ_CMD: u32 = 0x23;
pub const EDL_NVM_ACCESS_SET_REQ_CMD: u32 = 0x01;
pub const EDL_PATCH_CONFIG_CMD: u32 = 0x28;
pub const MAX_SIZE_PER_TLV_SEGMENT: u32 = 243;
pub const QCA_PRE_SHUTDOWN_CMD: u32 = 0xFC08;
pub const QCA_DISABLE_LOGGING: u32 = 0xFC17;

pub const EDL_CMD_REQ_RES_EVT: u32 = 0x00;
pub const EDL_PATCH_VER_RES_EVT: u32 = 0x19;
pub const EDL_APP_VER_RES_EVT: u32 = 0x02;
pub const EDL_TVL_DNLD_RES_EVT: u32 = 0x04;
pub const EDL_CMD_EXE_STATUS_EVT: u32 = 0x00;
pub const EDL_SET_BAUDRATE_RSP_EVT: u32 = 0x92;
pub const EDL_NVM_ACCESS_CODE_EVT: u32 = 0x0B;
pub const EDL_PATCH_CONFIG_RES_EVT: u32 = 0x00;
pub const QCA_DISABLE_LOGGING_SUB_OP: u32 = 0x14;

pub const EDL_TAG_ID_BD_ADDR: u32 = 2;
pub const EDL_TAG_ID_HCI: u32 = 17;
pub const EDL_TAG_ID_DEEP_SLEEP: u32 = 27;

pub const QCA_WCN3990_POWERON_PULSE: u32 = 0xFC;
pub const QCA_WCN3990_POWEROFF_PULSE: u32 = 0xC0;
pub const QCA_HCI_CC_OPCODE: u32 = 0xFC00;
pub const QCA_HCI_CC_SUCCESS: u32 = 0x00;
pub const QCA_WCN3991_SOC_ID: u32 = 0x40014320;
pub const QCA_WCN3950_SOC_ID_T: u32 = 0x40074130;
pub const QCA_WCN3950_SOC_ID_S: u32 = 0x40075130;

/* The SoC version combines the upper two bytes of the SoC and lower two of the patch. */
#[inline]
pub const fn get_soc_ver(soc_id: u32, rom_ver: u16) -> u32 {
    (u32::from_le(soc_id) << 16) | u32::from(u16::from_le(rom_ver))
}

pub const QCA_HSP_GF_SOC_ID: u32 = 0x1200;
pub const QCA_HSP_GF_SOC_MASK: u32 = 0x0000ff00;

#[repr(i32)]
pub enum qca_baudrate {
    QCA_BAUDRATE_115200 = 0,
    QCA_BAUDRATE_57600,
    QCA_BAUDRATE_38400,
    QCA_BAUDRATE_19200,
    QCA_BAUDRATE_9600,
    QCA_BAUDRATE_230400,
    QCA_BAUDRATE_250000,
    QCA_BAUDRATE_460800,
    QCA_BAUDRATE_500000,
    QCA_BAUDRATE_720000,
    QCA_BAUDRATE_921600,
    QCA_BAUDRATE_1000000,
    QCA_BAUDRATE_1250000,
    QCA_BAUDRATE_2000000,
    QCA_BAUDRATE_3000000,
    QCA_BAUDRATE_4000000,
    QCA_BAUDRATE_1600000,
    QCA_BAUDRATE_3200000,
    QCA_BAUDRATE_3500000,
    QCA_BAUDRATE_AUTO = 0xFE,
    QCA_BAUDRATE_RESERVED,
}

#[repr(i32)]
pub enum qca_tlv_dnld_mode { QCA_SKIP_EVT_NONE, QCA_SKIP_EVT_VSE, QCA_SKIP_EVT_CC, QCA_SKIP_EVT_VSE_CC }

#[repr(i32)]
pub enum qca_tlv_type { TLV_TYPE_PATCH = 1, TLV_TYPE_NVM, ELF_TYPE_PATCH }

#[repr(C)]
pub struct qca_fw_config {
    pub type_: u8,
    pub fwname: [u8; 64],
    pub user_baud_rate: u8,
    pub dnld_mode: qca_tlv_dnld_mode,
    pub dnld_type: qca_tlv_dnld_mode,
    pub bdaddr: bdaddr_t,
}

#[repr(C, packed)]
pub struct edl_event_hdr { pub cresp: u8, pub rtype: u8, pub data: [u8; 0] }
#[repr(C, packed)]
pub struct qca_btsoc_version { pub product_id: u32, pub patch_ver: u16, pub rom_ver: u16, pub soc_id: u32 }
#[repr(C, packed)]
pub struct tlv_seg_resp { pub result: u8 }
#[repr(C, packed)]
pub struct tlv_type_patch { pub total_size: u32, pub data_length: u32, pub format_version: u8, pub signature: u8, pub download_mode: u8, pub reserved1: u8, pub product_id: u16, pub rom_build: u16, pub patch_version: u16, pub reserved2: u16, pub entry: u32 }
#[repr(C, packed)]
pub struct tlv_type_nvm { pub tag_id: u16, pub tag_len: u16, pub reserve1: u32, pub reserve2: u32, pub data: [u8; 0] }
#[repr(C, packed)]
pub struct tlv_type_hdr { pub type_len: u32, pub data: [u8; 0] }

#[repr(i32)]
pub enum qca_btsoc_type { QCA_INVALID = -1, QCA_AR3002, QCA_ROME, QCA_WCN3950, QCA_WCN3988, QCA_WCN3990, QCA_WCN3998, QCA_WCN3991, QCA_QCA2066, QCA_QCA6390, QCA_WCN6750, QCA_WCN6855, QCA_WCN7850 }

/* CONFIG_BT_QCA controls whether these are external functions or unsupported stubs. */
extern "C" {
    pub fn qca_set_bdaddr_rome(hdev: *mut hci_dev, bdaddr: *const bdaddr_t) -> i32;
    pub fn qca_uart_setup(hdev: *mut hci_dev, baudrate: u8, soc_type: qca_btsoc_type, ver: qca_btsoc_version, firmware_name: *const i8, rampatch_name: *const i8) -> i32;
    pub fn qca_read_soc_version(hdev: *mut hci_dev, ver: *mut qca_btsoc_version, soc_type: qca_btsoc_type) -> i32;
    pub fn qca_set_bdaddr(hdev: *mut hci_dev, bdaddr: *const bdaddr_t) -> i32;
    pub fn qca_send_pre_shutdown_cmd(hdev: *mut hci_dev) -> i32;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
