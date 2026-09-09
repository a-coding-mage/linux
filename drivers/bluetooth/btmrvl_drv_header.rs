/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * Marvell Bluetooth driver: global definitions & declarations
 *
 * Copyright (C) 2009, Marvell International Ltd.
 */

// Linux header dependencies are supplied by the surrounding translation unit.

pub const BTM_HEADER_LEN: u32 = 4;
pub const BTM_UPLD_SIZE: u32 = 2312;

/* Timeouts are expressed in jiffies by the Linux msecs_to_jiffies macro. */
pub const WAIT_UNTIL_HS_STATE_CHANGED: u32 = 5000;
pub const WAIT_UNTIL_CMD_RESP: u32 = 5000;

#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum rdwr_status {
    RDWR_STATUS_SUCCESS = 0,
    RDWR_STATUS_FAILURE = 1,
    RDWR_STATUS_DONE = 2,
}

pub const FW_DUMP_MAX_NAME_LEN: usize = 8;
pub const FW_DUMP_HOST_READY: u8 = 0xEE;
pub const FW_DUMP_DONE: u8 = 0xFF;
pub const FW_DUMP_READ_DONE: u8 = 0xFE;

#[repr(C)]
pub struct memory_type_mapping {
    pub mem_name: [u8; FW_DUMP_MAX_NAME_LEN],
    pub mem_ptr: *mut u8,
    pub mem_size: u32,
    pub done_flag: u8,
}

#[repr(C)]
pub struct btmrvl_thread {
    pub task: *mut task_struct,
    pub wait_q: wait_queue_head_t,
    pub priv_: *mut core::ffi::c_void,
}

#[repr(C)]
pub struct btmrvl_device {
    pub card: *mut core::ffi::c_void,
    pub hcidev: *mut hci_dev,
    pub dev_type: u8,
    pub tx_dnld_rdy: u8,
    pub psmode: u8,
    pub pscmd: u8,
    pub hsmode: u8,
    pub hscmd: u8,
    /* Low byte is gap, high byte is GPIO */
    pub gpio_gap: u16,
    pub hscfgcmd: u8,
    pub sendcmdflag: u8,
}

#[repr(C)]
pub struct btmrvl_adapter {
    pub hw_regs_buf: *mut core::ffi::c_void,
    pub hw_regs: *mut u8,
    pub int_count: u32,
    pub tx_queue: sk_buff_head,
    pub psmode: u8,
    pub ps_state: u8,
    pub hs_state: u8,
    pub wakeup_tries: u8,
    pub cmd_wait_q: wait_queue_head_t,
    pub event_hs_wait_q: wait_queue_head_t,
    pub cmd_complete: u8,
    pub is_suspended: bool,
    pub is_suspending: bool,
}

#[repr(C)]
pub struct btmrvl_private {
    pub btmrvl_dev: btmrvl_device,
    pub adapter: *mut btmrvl_adapter,
    pub main_thread: btmrvl_thread,
    pub hw_host_to_card: Option<unsafe extern "C" fn(*mut btmrvl_private, *mut u8, u16) -> i32>,
    pub hw_wakeup_firmware: Option<unsafe extern "C" fn(*mut btmrvl_private) -> i32>,
    pub hw_process_int_status: Option<unsafe extern "C" fn(*mut btmrvl_private) -> i32>,
    pub driver_lock: spinlock_t, /* spinlock used by driver */
    #[cfg(CONFIG_DEBUG_FS)]
    pub debugfs_data: *mut core::ffi::c_void,
    pub surprise_removed: bool,
}

pub const MRVL_VENDOR_PKT: u8 = 0xFE;
pub const BT_CMD_PSCAN_WIN_REPORT_ENABLE: u16 = 0xFC03;
pub const BT_CMD_ROUTE_SCO_TO_HOST: u16 = 0xFC1D;
pub const BT_CMD_SET_BDADDR: u16 = 0xFC22;
pub const BT_CMD_AUTO_SLEEP_MODE: u16 = 0xFC23;
pub const BT_CMD_HOST_SLEEP_CONFIG: u16 = 0xFC59;
pub const BT_CMD_HOST_SLEEP_ENABLE: u16 = 0xFC5A;
pub const BT_CMD_MODULE_CFG_REQ: u16 = 0xFC5B;
pub const BT_CMD_LOAD_CONFIG_DATA: u16 = 0xFC61;
pub const MODULE_BRINGUP_REQ: u8 = 0xF1;
pub const MODULE_BROUGHT_UP: u8 = 0x00;
pub const MODULE_ALREADY_UP: u8 = 0x0C;
pub const MODULE_SHUTDOWN_REQ: u8 = 0xF2;
pub const BT_EVENT_AUTO_SLEEP_MODE: u8 = 0x23;
pub const BT_EVENT_HOST_SLEEP_CONFIG: u8 = 0x59;
pub const BT_EVENT_HOST_SLEEP_ENABLE: u8 = 0x5A;
pub const BT_EVENT_MODULE_CFG_REQ: u8 = 0x5B;
pub const BT_EVENT_POWER_STATE: u8 = 0x20;
pub const BT_PS_ENABLE: u8 = 0x02;
pub const BT_PS_DISABLE: u8 = 0x03;
pub const BT_PS_SLEEP: u8 = 0x01;
pub const HS_ACTIVATED: u8 = 0x01;
pub const HS_DEACTIVATED: u8 = 0x00;
pub const PS_SLEEP: u8 = 0x01;
pub const PS_AWAKE: u8 = 0x00;
pub const BT_CAL_HDR_LEN: u32 = 4;
pub const BT_CAL_DATA_SIZE: u32 = 28;

#[repr(C, packed)]
pub struct btmrvl_event {
    pub ec: u8,
    pub length: u8,
    pub data: [u8; 4],
}

extern "C" {
    pub fn btmrvl_register_hdev(priv_: *mut btmrvl_private) -> i32;
    pub fn btmrvl_add_card(card: *mut core::ffi::c_void) -> *mut btmrvl_private;
    pub fn btmrvl_remove_card(priv_: *mut btmrvl_private) -> i32;
    pub fn btmrvl_interrupt(priv_: *mut btmrvl_private);
    pub fn btmrvl_check_evtpkt(priv_: *mut btmrvl_private, skb: *mut sk_buff) -> bool;
    pub fn btmrvl_process_event(priv_: *mut btmrvl_private, skb: *mut sk_buff) -> i32;
    pub fn btmrvl_send_module_cfg_cmd(priv_: *mut btmrvl_private, subcmd: u8) -> i32;
    pub fn btmrvl_pscan_window_reporting(priv_: *mut btmrvl_private, subcmd: u8) -> i32;
    pub fn btmrvl_send_hscfg_cmd(priv_: *mut btmrvl_private) -> i32;
    pub fn btmrvl_enable_ps(priv_: *mut btmrvl_private) -> i32;
    pub fn btmrvl_prepare_command(priv_: *mut btmrvl_private) -> i32;
    pub fn btmrvl_enable_hs(priv_: *mut btmrvl_private) -> i32;
    #[cfg(CONFIG_DEBUG_FS)]
    pub fn btmrvl_debugfs_init(hdev: *mut hci_dev);
    #[cfg(CONFIG_DEBUG_FS)]
    pub fn btmrvl_debugfs_remove(hdev: *mut hci_dev);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
