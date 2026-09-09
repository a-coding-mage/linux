// SPDX-License-Identifier: GPL-2.0-only
// cec - HDMI Consumer Electronics Control support header

// Dependencies supplied by the Linux/media environment are intentionally left external.

pub const CEC_CAP_DEFAULTS: u32 = CEC_CAP_LOG_ADDRS | CEC_CAP_TRANSMIT | CEC_CAP_PASSTHROUGH | CEC_CAP_RC;

#[repr(C)]
pub struct cec_devnode {
    pub dev: device,
    pub cdev: cdev,
    pub minor: core::ffi::c_int,
    pub lock: mutex,
    pub registered: bool,
    pub unregistered: bool,
    pub lock_fhs: mutex,
    pub fhs: list_head,
}

pub struct cec_adapter;
pub struct cec_pin;
pub struct cec_notifier;

#[repr(C)]
pub struct cec_data {
    pub list: list_head,
    pub xfer_list: list_head,
    pub adap: *mut cec_adapter,
    pub msg: cec_msg,
    pub match_len: u8,
    pub match_reply: [u8; 5],
    pub fh: *mut cec_fh,
    pub work: delayed_work,
    pub c: completion,
    pub attempts: u8,
    pub blocking: bool,
    pub completed: bool,
}

#[repr(C)]
pub struct cec_msg_entry { pub list: list_head, pub msg: cec_msg }
#[repr(C)]
pub struct cec_event_entry { pub list: list_head, pub ev: cec_event }

pub const CEC_NUM_EVENTS: usize = CEC_EVENT_PIN_5V_HIGH as usize;

#[repr(C)]
pub struct cec_fh {
    pub list: list_head,
    pub xfer_list: list_head,
    pub adap: *mut cec_adapter,
    pub mode_initiator: u8,
    pub mode_follower: u8,
    pub wait: wait_queue_head_t,
    pub lock: mutex,
    pub events: [list_head; CEC_NUM_EVENTS],
    pub queued_events: [u16; CEC_NUM_EVENTS],
    pub total_queued_events: core::ffi::c_uint,
    pub msgs: list_head,
    pub queued_msgs: core::ffi::c_uint,
}

pub const CEC_SIGNAL_FREE_TIME_RETRY: u32 = 3;
pub const CEC_SIGNAL_FREE_TIME_NEW_INITIATOR: u32 = 5;
pub const CEC_SIGNAL_FREE_TIME_NEXT_XFER: u32 = 7;
#[inline]
pub const fn CEC_FREE_TIME_TO_USEC(ft: u32) -> u32 { ft.wrapping_mul(2400) }

#[repr(C)]
pub struct cec_adap_ops {
    pub adap_enable: Option<unsafe extern "C" fn(*mut cec_adapter, bool) -> core::ffi::c_int>,
    pub adap_monitor_all_enable: Option<unsafe extern "C" fn(*mut cec_adapter, bool) -> core::ffi::c_int>,
    pub adap_monitor_pin_enable: Option<unsafe extern "C" fn(*mut cec_adapter, bool) -> core::ffi::c_int>,
    pub adap_log_addr: Option<unsafe extern "C" fn(*mut cec_adapter, u8) -> core::ffi::c_int>,
    pub adap_unconfigured: Option<unsafe extern "C" fn(*mut cec_adapter)>,
    pub adap_transmit: Option<unsafe extern "C" fn(*mut cec_adapter, u8, u32, *mut cec_msg) -> core::ffi::c_int>,
    pub adap_nb_transmit_canceled: Option<unsafe extern "C" fn(*mut cec_adapter, *const cec_msg)>,
    pub adap_status: Option<unsafe extern "C" fn(*mut cec_adapter, *mut seq_file)>,
    pub adap_free: Option<unsafe extern "C" fn(*mut cec_adapter)>,
    pub error_inj_show: Option<unsafe extern "C" fn(*mut cec_adapter, *mut seq_file) -> core::ffi::c_int>,
    pub error_inj_parse_line: Option<unsafe extern "C" fn(*mut cec_adapter, *mut core::ffi::c_char) -> bool>,
    pub configured: Option<unsafe extern "C" fn(*mut cec_adapter)>,
    pub received: Option<unsafe extern "C" fn(*mut cec_adapter, *mut cec_msg) -> core::ffi::c_int>,
}

pub const CEC_MAX_MSG_RX_QUEUE_SZ: u32 = 18 * 3;
pub const CEC_MAX_MSG_TX_QUEUE_SZ: u32 = 18;

#[repr(C)]
pub struct cec_adapter {
    pub owner: *mut module,
    pub name: [core::ffi::c_char; 32],
    pub devnode: cec_devnode,
    pub lock: mutex,
    pub rc: *mut rc_dev,
    pub transmit_queue: list_head,
    pub transmit_queue_sz: core::ffi::c_uint,
    pub wait_queue: list_head,
    pub transmitting: *mut cec_data,
    pub transmit_in_progress: bool,
    pub transmit_in_progress_aborted: bool,
    pub xfer_timeout_ms: core::ffi::c_uint,
    pub kthread_config: *mut task_struct,
    pub config_completion: completion,
    pub kthread: *mut task_struct,
    pub kthread_waitq: wait_queue_head_t,
    pub ops: *const cec_adap_ops,
    pub priv_: *mut core::ffi::c_void,
    pub capabilities: u32,
    pub available_log_addrs: u8,
    pub phys_addr: u16,
    pub needs_hpd: bool,
    pub is_enabled: bool,
    pub is_claiming_log_addrs: bool,
    pub is_configuring: bool,
    pub must_reconfigure: bool,
    pub is_configured: bool,
    pub cec_pin_is_high: bool,
    pub adap_controls_phys_addr: bool,
    pub last_initiator: u8,
    pub monitor_all_cnt: u32,
    pub monitor_pin_cnt: u32,
    pub follower_cnt: u32,
    pub cec_follower: *mut cec_fh,
    pub cec_initiator: *mut cec_fh,
    pub passthrough: bool,
    pub log_addrs: cec_log_addrs,
    pub conn_info: cec_connector_info,
    pub tx_timeout_cnt: u32,
    pub tx_low_drive_cnt: u32,
    pub tx_error_cnt: u32,
    pub tx_arb_lost_cnt: u32,
    pub tx_low_drive_log_cnt: u32,
    pub tx_error_log_cnt: u32,
    pub error_inj_tx_timeouts: u32,
    // CONFIG_CEC_NOTIFIER controls this field in the C source.
    pub notifier: *mut cec_notifier,
    // CONFIG_CEC_PIN controls this field in the C source.
    pub pin: *mut cec_pin,
    pub cec_dir: *mut dentry,
    pub sequence: u32,
    pub input_phys: [core::ffi::c_char; 40],
}

pub unsafe fn cec_get_device(adap: *mut cec_adapter) -> core::ffi::c_int {
    let devnode = &mut (*adap).devnode;
    mutex_lock(&mut devnode.lock);
    if !devnode.registered { mutex_unlock(&mut devnode.lock); return -ENODEV; }
    get_device(&mut devnode.dev);
    mutex_unlock(&mut devnode.lock);
    0
}
pub unsafe fn cec_put_device(adap: *mut cec_adapter) { put_device(&mut (*adap).devnode.dev); }
pub unsafe fn cec_get_drvdata(adap: *const cec_adapter) -> *mut core::ffi::c_void { (*adap).priv_ }
pub unsafe fn cec_has_log_addr(adap: *const cec_adapter, log_addr: u8) -> bool { (*adap).log_addrs.log_addr_mask & (1u32 << log_addr) != 0 }
pub unsafe fn cec_is_sink(adap: *const cec_adapter) -> bool { (*adap).phys_addr == 0 }
pub unsafe fn cec_is_registered(adap: *const cec_adapter) -> bool { !adap.is_null() && (*adap).devnode.registered }

#[macro_export]
macro_rules! cec_phys_addr_exp { ($pa:expr) => { (($pa) >> 12, (($pa) >> 8) & 0xf, (($pa) >> 4) & 0xf, ($pa) & 0xf) }; }

pub struct edid;
pub struct drm_connector;

// IS_REACHABLE(CONFIG_CEC_CORE) is a build-time kernel condition.
extern "C" {
    pub fn cec_allocate_adapter(ops: *const cec_adap_ops, priv_: *mut core::ffi::c_void, name: *const core::ffi::c_char, caps: u32, available_las: u8) -> *mut cec_adapter;
    pub fn cec_register_adapter(adap: *mut cec_adapter, parent: *mut device) -> core::ffi::c_int;
    pub fn cec_unregister_adapter(adap: *mut cec_adapter);
    pub fn cec_delete_adapter(adap: *mut cec_adapter);
    pub fn cec_s_log_addrs(adap: *mut cec_adapter, log_addrs: *mut cec_log_addrs, block: bool) -> core::ffi::c_int;
    pub fn cec_s_phys_addr(adap: *mut cec_adapter, phys_addr: u16, block: bool);
    pub fn cec_s_phys_addr_from_edid(adap: *mut cec_adapter, edid: *const edid);
    pub fn cec_s_conn_info(adap: *mut cec_adapter, conn_info: *const cec_connector_info);
    pub fn cec_transmit_msg(adap: *mut cec_adapter, msg: *mut cec_msg, block: bool) -> core::ffi::c_int;
    pub fn cec_transmit_done_ts(adap: *mut cec_adapter, status: u8, arb_lost_cnt: u8, nack_cnt: u8, low_drive_cnt: u8, error_cnt: u8, ts: ktime_t);
    pub fn cec_transmit_attempt_done_ts(adap: *mut cec_adapter, status: u8, ts: ktime_t);
    pub fn cec_received_msg_ts(adap: *mut cec_adapter, msg: *mut cec_msg, ts: ktime_t);
    pub fn cec_queue_pin_cec_event(adap: *mut cec_adapter, is_high: bool, dropped_events: bool, ts: ktime_t);
    pub fn cec_queue_pin_hpd_event(adap: *mut cec_adapter, is_high: bool, ts: ktime_t);
    pub fn cec_queue_pin_5v_event(adap: *mut cec_adapter, is_high: bool, ts: ktime_t);
    pub fn cec_get_edid_phys_addr(edid: *const u8, size: core::ffi::c_uint, offset: *mut core::ffi::c_uint) -> u16;
    pub fn cec_fill_conn_info_from_drm(conn_info: *mut cec_connector_info, connector: *const drm_connector);
}

#[inline] pub unsafe fn cec_transmit_done(adap: *mut cec_adapter, status: u8, arb: u8, nack: u8, low: u8, error: u8) { cec_transmit_done_ts(adap, status, arb, nack, low, error, ktime_get()); }
#[inline] pub unsafe fn cec_transmit_attempt_done(adap: *mut cec_adapter, status: u8) { cec_transmit_attempt_done_ts(adap, status, ktime_get()); }
#[inline] pub unsafe fn cec_received_msg(adap: *mut cec_adapter, msg: *mut cec_msg) { cec_received_msg_ts(adap, msg, ktime_get()); }
#[inline] pub unsafe fn cec_phys_addr_invalidate(adap: *mut cec_adapter) { cec_s_phys_addr(adap, CEC_PHYS_ADDR_INVALID, false); }

#[inline]
pub unsafe fn cec_get_edid_spa_location(edid: *const u8, size: core::ffi::c_uint) -> core::ffi::c_uint {
    let mut blocks = size / 128;
    if blocks < 2 || size % 128 != 0 { return 0; }
    if (*edid.add(0x7e) as core::ffi::c_uint) + 1 < blocks { blocks = *edid.add(0x7e) as core::ffi::c_uint + 1; }
    let mut block = 1;
    while block < blocks {
        let offset = block * 128;
        if *edid.add(offset as usize) != 0x02 || *edid.add((offset + 1) as usize) != 0x03 { block += 1; continue; }
        let d = *edid.add((offset + 2) as usize) & 0x7f;
        if d > 4 {
            let mut i = offset + 4;
            let end = offset + d as core::ffi::c_uint;
            while i < end {
                let tag = *edid.add(i as usize) >> 5;
                let len = *edid.add(i as usize) & 0x1f;
                if tag == 3 && len >= 5 && i + len as core::ffi::c_uint <= end && *edid.add((i + 1) as usize) == 3 && *edid.add((i + 2) as usize) == 0x0c && *edid.add((i + 3) as usize) == 0 { return i + 4; }
                i += len as core::ffi::c_uint + 1;
            }
        }
        block += 1;
    }
    0
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
