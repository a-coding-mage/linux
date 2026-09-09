/* SPDX-License-Identifier: GPL-2.0-only */
/* Shared Transport Header file. */

// Dependency intent: kernel types such as sk_buff, tty_struct, spinlock_t,
// work_struct, platform_device, completion, firmware, pm_message_t, and u8/u16/u32
// are supplied by the surrounding kernel bindings.

#[repr(C)]
pub enum proto_type {
    ST_BT,
    ST_FM,
    ST_GPS,
    ST_MAX_CHANNELS = 16,
}

#[repr(C)]
pub struct st_proto_s {
    pub type_: proto_type,
    pub recv: Option<unsafe extern "C" fn(*mut core::ffi::c_void, *mut sk_buff) -> core::ffi::c_long>,
    pub match_packet: Option<unsafe extern "C" fn(*const u8) -> u8>,
    pub reg_complete_cb: Option<unsafe extern "C" fn(*mut core::ffi::c_void, core::ffi::c_int)>,
    pub write: Option<unsafe extern "C" fn(*mut sk_buff) -> core::ffi::c_long>,
    pub priv_data: *mut core::ffi::c_void,
    pub chnl_id: u8,
    pub max_frame_size: u16,
    pub hdr_len: u8,
    pub offset_len_in_hdr: u8,
    pub len_size: u8,
    pub reserve: u8,
}

extern "C" {
    pub fn st_register(proto: *mut st_proto_s) -> core::ffi::c_long;
    pub fn st_unregister(proto: *mut st_proto_s) -> core::ffi::c_long;
}

pub const ST_NOTEMPTY: u32 = 1;
pub const ST_EMPTY: u32 = 0;
pub const ST_INITIALIZING: u32 = 1;
pub const ST_REG_IN_PROGRESS: u32 = 2;
pub const ST_REG_PENDING: u32 = 3;
pub const ST_WAITING_FOR_RESP: u32 = 4;

pub const ST_TX_SENDING: u32 = 1;
pub const ST_TX_WAKEUP: u32 = 2;

#[repr(C)]
pub struct st_data_s {
    pub st_state: libc::c_ulong,
    pub tx_skb: *mut sk_buff,
    pub tx_state: libc::c_ulong,
    pub list: [*mut st_proto_s; 16],
    pub is_registered: [bool; 16],
    pub rx_state: libc::c_ulong,
    pub rx_count: libc::c_ulong,
    pub rx_skb: *mut sk_buff,
    pub rx_chnl: u8,
    pub txq: sk_buff_head,
    pub tx_waitq: sk_buff_head,
    pub lock: spinlock_t,
    pub protos_registered: u8,
    pub ll_state: libc::c_ulong,
    pub kim_data: *mut core::ffi::c_void,
    pub tty: *mut tty_struct,
    pub work_write_wakeup: work_struct,
}

extern "C" {
    pub fn st_get_uart_wr_room(st_gdata: *mut st_data_s) -> core::ffi::c_int;
    pub fn st_int_write(st_gdata: *mut st_data_s, data: *const u8, count: core::ffi::c_int) -> core::ffi::c_int;
    pub fn st_write(skb: *mut sk_buff) -> core::ffi::c_long;
    pub fn st_ll_send_frame(proto: proto_type, skb: *mut sk_buff);
    pub fn st_tx_wakeup(st_data: *mut st_data_s);
    pub fn st_core_init(st_data: *mut *mut st_data_s) -> core::ffi::c_int;
    pub fn st_core_exit(st_data: *mut st_data_s);
    pub fn st_kim_ref(st_data: *mut *mut st_data_s, data: core::ffi::c_int);
    pub fn gps_chrdrv_stub_write(data: *const u8, count: core::ffi::c_int) -> core::ffi::c_int;
    pub fn gps_chrdrv_stub_init();
}

pub const LDISC_TIME: u32 = 1000;
pub const CMD_RESP_TIME: u32 = 800;
pub const CMD_WR_TIME: u32 = 5000;
#[inline]
pub const fn MAKEWORD(a: u32, b: u32) -> u16 {
    ((a as u8) as u16) | (((b as u8) as u16) << 8)
}
pub const GPIO_HIGH: u32 = 1;
pub const GPIO_LOW: u32 = 0;
pub const POR_RETRY_COUNT: u32 = 5;
pub const UART_DEV_NAME_LEN: usize = 32;

#[repr(C)]
pub struct chip_version { pub full: u16, pub chip: u16, pub min_ver: u16, pub maj_ver: u16 }

#[repr(C)]
pub struct kim_data_s {
    pub uim_pid: libc::c_long,
    pub kim_pdev: *mut platform_device,
    pub kim_rcvd: completion,
    pub ldisc_installed: completion,
    pub resp_buffer: [u8; 30],
    pub fw_entry: *const firmware,
    pub nshutdown: libc::c_uint,
    pub rx_state: libc::c_ulong,
    pub rx_count: libc::c_ulong,
    pub rx_skb: *mut sk_buff,
    pub core_data: *mut st_data_s,
    pub version: chip_version,
    pub ldisc_install: u8,
    pub dev_name: [u8; UART_DEV_NAME_LEN + 1],
    pub flow_cntrl: libc::c_uint,
    pub baud_rate: libc::c_uint,
}

extern "C" {
    pub fn st_kim_start(data: *mut core::ffi::c_void) -> core::ffi::c_long;
    pub fn st_kim_stop(data: *mut core::ffi::c_void) -> core::ffi::c_long;
    pub fn st_kim_complete(data: *mut core::ffi::c_void);
    pub fn kim_st_list_protocols(st_data: *mut st_data_s, data: *mut core::ffi::c_void);
    pub fn st_kim_recv(disc_data: *mut core::ffi::c_void, data: *const u8, count: usize);
}

pub const ACTION_SEND_COMMAND: u16 = 1;
pub const ACTION_WAIT_EVENT: u16 = 2;
pub const ACTION_SERIAL: u16 = 3;
pub const ACTION_DELAY: u16 = 4;
pub const ACTION_RUN_SCRIPT: u16 = 5;
pub const ACTION_REMARKS: u16 = 6;

#[repr(C, packed)]
pub struct bts_header { pub magic: u32, pub version: u32, pub future: [u8; 24], pub actions: [u8; 0] }
#[repr(C, packed)]
pub struct bts_action { pub type_: u16, pub size: u16, pub data: [u8; 0] }
#[repr(C, packed)]
pub struct bts_action_send { pub data: [u8; 0] }
#[repr(C, packed)]
pub struct bts_action_wait { pub msec: u32, pub size: u32, pub data: [u8; 0] }
#[repr(C, packed)]
pub struct bts_action_delay { pub msec: u32 }
#[repr(C, packed)]
pub struct bts_action_serial { pub baud: u32, pub flow_control: u32 }
#[repr(C, packed)]
pub struct hci_command { pub prefix: u8, pub opcode: u16, pub plen: u8, pub speed: u32 }

pub const ST_W4_PACKET_TYPE: u32 = 0;
pub const ST_W4_HEADER: u32 = 1;
pub const ST_W4_DATA: u32 = 2;
pub const ST_LL_ASLEEP: u32 = 0;
pub const ST_LL_ASLEEP_TO_AWAKE: u32 = 1;
pub const ST_LL_AWAKE: u32 = 2;
pub const ST_LL_AWAKE_TO_ASLEEP: u32 = 3;
pub const ST_LL_INVALID: u32 = 4;
pub const LL_SLEEP_IND: u32 = 0x30;
pub const LL_SLEEP_ACK: u32 = 0x31;
pub const LL_WAKE_UP_IND: u32 = 0x32;
pub const LL_WAKE_UP_ACK: u32 = 0x33;

extern "C" {
    pub fn st_ll_init(data: *mut st_data_s) -> core::ffi::c_long;
    pub fn st_ll_deinit(data: *mut st_data_s) -> core::ffi::c_long;
    pub fn st_ll_enable(data: *mut st_data_s);
    pub fn st_ll_disable(data: *mut st_data_s);
    pub fn st_ll_getstate(data: *mut st_data_s) -> libc::c_ulong;
    pub fn st_ll_sleep_state(data: *mut st_data_s, state: u8) -> libc::c_ulong;
    pub fn st_ll_wakeup(data: *mut st_data_s);
}

#[repr(C, packed)]
pub struct fm_event_hdr { pub plen: u8 }
pub const FM_MAX_FRAME_SIZE: u32 = 0xFF;
pub const FM_EVENT_HDR_SIZE: u32 = 1;
pub const ST_FM_CH8_PKT: u32 = 0x8;
#[repr(C, packed)]
pub struct gps_event_hdr { pub opcode: u8, pub plen: u16 }

#[repr(C)]
pub struct ti_st_plat_data {
    pub nshutdown_gpio: u32,
    pub dev_name: [u8; UART_DEV_NAME_LEN],
    pub flow_cntrl: u32,
    pub baud_rate: u32,
    pub suspend: Option<unsafe extern "C" fn(*mut platform_device, pm_message_t) -> core::ffi::c_int>,
    pub resume: Option<unsafe extern "C" fn(*mut platform_device) -> core::ffi::c_int>,
    pub chip_enable: Option<unsafe extern "C" fn(*mut kim_data_s) -> core::ffi::c_int>,
    pub chip_disable: Option<unsafe extern "C" fn(*mut kim_data_s) -> core::ffi::c_int>,
    pub chip_asleep: Option<unsafe extern "C" fn(*mut kim_data_s) -> core::ffi::c_int>,
    pub chip_awake: Option<unsafe extern "C" fn(*mut kim_data_s) -> core::ffi::c_int>,
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
