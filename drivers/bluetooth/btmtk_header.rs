/* SPDX-License-Identifier: ISC */
/* Copyright (C) 2021 MediaTek Inc. */

pub const FIRMWARE_MT7622: &str = "mediatek/mt7622pr2h.bin";
pub const FIRMWARE_MT7663: &str = "mediatek/mt7663pr2h.bin";
pub const FIRMWARE_MT7668: &str = "mediatek/mt7668pr2h.bin";
pub const FIRMWARE_MT7922: &str = "mediatek/BT_RAM_CODE_MT7922_1_1_hdr.bin";
pub const FIRMWARE_MT7902: &str = "mediatek/BT_RAM_CODE_MT7902_1_1_hdr.bin";
pub const FIRMWARE_MT7961: &str = "mediatek/BT_RAM_CODE_MT7961_1_2_hdr.bin";
pub const FIRMWARE_MT7925: &str = "mediatek/mt7925/BT_RAM_CODE_MT7925_1_1_hdr.bin";
pub const FIRMWARE_MT7927: &str = "mediatek/mt7927/BT_RAM_CODE_MT6639_2_1_hdr.bin";

pub const HCI_EV_WMT: u8 = 0xe4;
pub const HCI_WMT_MAX_EVENT_SIZE: usize = 64;
pub const BTMTK_WMT_REG_WRITE: u8 = 0x1;
pub const BTMTK_WMT_REG_READ: u8 = 0x2;
pub const MT7921_BTSYS_RST: u32 = 0x70002610;
pub const MT7921_BTSYS_RST_WITH_GPIO: u32 = 1 << 7;
pub const MT7921_PINMUX_0: u32 = 0x70005050;
pub const MT7921_PINMUX_1: u32 = 0x70005054;
pub const MT7921_DLSTATUS: u32 = 0x7c053c10;
pub const BT_DL_STATE: u32 = 1 << 1;
pub const MTK_COREDUMP_SIZE: usize = 1024 * 1000;
pub const MTK_COREDUMP_END: &str = "coredump end";
pub const MTK_COREDUMP_END_LEN: usize = MTK_COREDUMP_END.len() + 1;
pub const MTK_COREDUMP_NUM: u32 = 255;

/* UHW CR mapping */
pub const MTK_BT_MISC: u32 = 0x70002510;
pub const MTK_BT_SUBSYS_RST: u32 = 0x70002610;
pub const MTK_UDMA_INT_STA_BT: u32 = 0x74000024;
pub const MTK_UDMA_INT_STA_BT1: u32 = 0x74000308;
pub const MTK_BT_WDT_STATUS: u32 = 0x740003A0;
pub const MTK_EP_RST_OPT: u32 = 0x74011890;
pub const MTK_EP_RST_IN_OUT_OPT: u32 = 0x00010001;
pub const MTK_BT_RST_DONE: u32 = 0x00000100;
pub const MTK_BT_RESET_REG_CONNV3: u32 = 0x70028610;
pub const MTK_BT_READ_DEV_ID: u32 = 0x70010200;

/* MediaTek ISO Interface */
pub const MTK_ISO_IFNUM: u32 = 2;

pub const BTMTK_WMT_PATCH_DWNLD: u32 = 0x1;
pub const BTMTK_WMT_TEST: u32 = 0x2;
pub const BTMTK_WMT_WAKEUP: u32 = 0x3;
pub const BTMTK_WMT_HIF: u32 = 0x4;
pub const BTMTK_WMT_FUNC_CTRL: u32 = 0x6;
pub const BTMTK_WMT_RST: u32 = 0x7;
pub const BTMTK_WMT_REGISTER: u32 = 0x8;
pub const BTMTK_WMT_SEMAPHORE: u32 = 0x17;

pub const BTMTK_WMT_INVALID: u32 = 0;
pub const BTMTK_WMT_PATCH_UNDONE: u32 = 1;
pub const BTMTK_WMT_PATCH_PROGRESS: u32 = 2;
pub const BTMTK_WMT_PATCH_DONE: u32 = 3;
pub const BTMTK_WMT_ON_UNDONE: u32 = 4;
pub const BTMTK_WMT_ON_DONE: u32 = 5;
pub const BTMTK_WMT_ON_PROGRESS: u32 = 6;

#[repr(C, packed)]
pub struct btmtk_wmt_hdr { pub dir: u8, pub op: u8, pub dlen: u16, pub flag: u8 }
#[repr(C, packed)]
pub struct btmtk_hci_wmt_cmd { pub hdr: btmtk_wmt_hdr, pub data: [u8; 0] }
#[repr(C, packed)]
pub struct btmtk_hci_wmt_evt { pub hhdr: hci_event_hdr, pub whdr: btmtk_wmt_hdr }
#[repr(C, packed)]
pub struct btmtk_hci_wmt_evt_funcc { pub hwhdr: btmtk_hci_wmt_evt, pub status: u16 }
#[repr(C, packed)]
pub struct btmtk_hci_wmt_evt_reg { pub hwhdr: btmtk_hci_wmt_evt, pub rsv: [u8; 2], pub num: u8, pub addr: u32, pub val: u32 }
#[repr(C, packed)]
pub struct btmtk_tci_sleep { pub mode: u8, pub duration: u16, pub host_duration: u16, pub host_wakeup_pin: u8, pub time_compensation: u8 }
#[repr(C, packed)]
pub struct btmtk_wakeon { pub mode: u8, pub gpo: u8, pub active_high: u8, pub enable_delay: u16, pub wakeup_delay: u16 }
#[repr(C, packed)]
pub struct btmtk_sco { pub clock_config: u8, pub transmit_format_config: u8, pub channel_format_config: u8, pub channel_select_config: u8 }
#[repr(C, packed)]
pub struct reg_read_cmd { pub type_: u8, pub rsv: u8, pub num: u8, pub addr: u32 }
#[repr(C, packed)]
pub struct reg_write_cmd { pub type_: u8, pub rsv: u8, pub num: u8, pub addr: u32, pub data: u32, pub mask: u32 }

#[repr(C)]
pub struct btmtk_hci_wmt_params { pub op: u8, pub flag: u8, pub dlen: u16, pub data: *const core::ffi::c_void, pub status: *mut u32 }
pub const BTMTK_TX_WAIT_VND_EVT: u32 = 0;
pub const BTMTK_FIRMWARE_LOADED: u32 = 1;
pub const BTMTK_HW_RESET_ACTIVE: u32 = 2;
pub const BTMTK_ISOPKT_OVER_INTR: u32 = 3;
pub const BTMTK_ISOPKT_RUNNING: u32 = 4;
pub const BTMTK_FIRMWARE_DL_RETRY: u32 = 5;

pub type btmtk_reset_sync_func_t = unsafe extern "C" fn(*mut hci_dev, *mut core::ffi::c_void) -> i32;
#[repr(C)]
pub struct btmtk_coredump_info { pub driver_name: *const core::ffi::c_char, pub fw_version: u32, pub cnt: u16, pub state: i32 }
#[repr(C)]
pub struct btmtk_data {
    pub drv_name: *const core::ffi::c_char, pub flags: usize, pub dev_id: u32,
    pub reset_sync: Option<btmtk_reset_sync_func_t>, pub cd_info: btmtk_coredump_info,
    pub udev: *mut usb_device, pub intf: *mut usb_interface, pub ctrl_anchor: *mut usb_anchor,
    pub evt_skb: *mut sk_buff, pub isopkt_tx_ep: *mut usb_endpoint_descriptor,
    pub isopkt_rx_ep: *mut usb_endpoint_descriptor, pub isopkt_intf: *mut usb_interface,
    pub isopkt_anchor: usb_anchor, pub isopkt_skb: *mut sk_buff,
    /* spinlock for ISO data transmission */ pub isorxlock: spinlock_t,
}
pub type wmt_cmd_sync_func_t = unsafe extern "C" fn(*mut hci_dev, *mut btmtk_hci_wmt_params) -> i32;

/* The following declarations are enabled when CONFIG_BT_MTK is enabled. */
extern "C" {
    pub fn btmtk_set_bdaddr(hdev: *mut hci_dev, bdaddr: *const bdaddr_t) -> i32;
    pub fn btmtk_setup_firmware_79xx(hdev: *mut hci_dev, fwname: *const core::ffi::c_char, wmt_cmd_sync: Option<wmt_cmd_sync_func_t>, dev_id: u32) -> i32;
    pub fn btmtk_setup_firmware(hdev: *mut hci_dev, fwname: *const core::ffi::c_char, wmt_cmd_sync: Option<wmt_cmd_sync_func_t>) -> i32;
    pub fn btmtk_reset_sync(hdev: *mut hci_dev);
    pub fn btmtk_register_coredump(hdev: *mut hci_dev, name: *const core::ffi::c_char, fw_version: u32) -> i32;
    pub fn btmtk_process_coredump(hdev: *mut hci_dev, skb: *mut sk_buff) -> i32;
    pub fn btmtk_fw_get_filename(buf: *mut core::ffi::c_char, size: usize, dev_id: u32, fw_ver: u32, fw_flavor: u32);
    pub fn btmtk_usb_subsys_reset(hdev: *mut hci_dev, dev_id: u32) -> i32;
    pub fn btmtk_usb_recv_acl(hdev: *mut hci_dev, skb: *mut sk_buff) -> i32;
    pub fn alloc_mtk_intr_urb(hdev: *mut hci_dev, skb: *mut sk_buff, tx_complete: usb_complete_t) -> *mut urb;
    pub fn btmtk_usb_resume(hdev: *mut hci_dev) -> i32;
    pub fn btmtk_usb_suspend(hdev: *mut hci_dev) -> i32;
    pub fn btmtk_usb_setup(hdev: *mut hci_dev) -> i32;
    pub fn btmtk_usb_shutdown(hdev: *mut hci_dev) -> i32;
    pub fn btmtk_recv_event(hdev: *mut hci_dev, skb: *mut sk_buff) -> i32;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
