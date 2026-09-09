// SPDX-License-Identifier: GPL-2.0
/*
 * System Control and Management Interface (SCMI) NXP BBM Protocol
 *
 * Copyright 2024 NXP
 */

// C dependencies and build-time kernel/module macros are supplied externally.

const SCMI_PROTOCOL_SUPPORTED_VERSION: u32 = 0x10000;

#[repr(u32)]
enum ScmiImxBbmProtocolCmd {
    ImxBbmGprSet = 0x3,
    ImxBbmGprGet = 0x4,
    ImxBbmRtcAttributes = 0x5,
    ImxBbmRtcTimeSet = 0x6,
    ImxBbmRtcTimeGet = 0x7,
    ImxBbmRtcAlarmSet = 0x8,
    ImxBbmButtonGet = 0x9,
    ImxBbmRtcNotify = 0xA,
    ImxBbmButtonNotify = 0xB,
}

const SCMI_IMX_BBM_NOTIFY_RTC_UPDATED: u32 = 1 << 2;
const SCMI_IMX_BBM_NOTIFY_RTC_ROLLOVER: u32 = 1 << 1;
const SCMI_IMX_BBM_NOTIFY_RTC_ALARM: u32 = 1 << 0;
const SCMI_IMX_BBM_RTC_ALARM_ENABLE_FLAG: u32 = 1 << 0;
const SCMI_IMX_BBM_NOTIFY_RTC_FLAG: u32 = SCMI_IMX_BBM_NOTIFY_RTC_UPDATED
    | SCMI_IMX_BBM_NOTIFY_RTC_ROLLOVER
    | SCMI_IMX_BBM_NOTIFY_RTC_ALARM;
const SCMI_IMX_BBM_EVENT_RTC_MASK: u32 = 0xff << 24;

#[repr(C)]
struct ScmiImxBbmInfo { nr_rtc: i32, nr_gpr: i32 }

#[repr(C)]
struct ScmiMsgImxBbmProtocolAttributes { attributes: u32 }
#[repr(C)]
struct ScmiImxBbmSetTime { id: u32, flags: u32, value_low: u32, value_high: u32 }
#[repr(C)]
struct ScmiImxBbmGetTime { id: u32, flags: u32 }
#[repr(C)]
struct ScmiImxBbmAlarmTime { id: u32, flags: u32, value_low: u32, value_high: u32 }
#[repr(C)]
struct ScmiMsgImxBbmRtcNotify { rtc_id: u32, flags: u32 }
#[repr(C)]
struct ScmiMsgImxBbmButtonNotify { flags: u32 }
#[repr(C)]
struct ScmiImxBbmNotifyPayld { flags: u32 }

unsafe fn scmi_imx_bbm_attributes_get(ph: *const ScmiProtocolHandle, pi: *mut ScmiImxBbmInfo) -> i32 {
    let mut t: *mut ScmiXfer = core::ptr::null_mut();
    let mut ret = (*(*ph).xops).xfer_get_init(ph, PROTOCOL_ATTRIBUTES, 0,
        core::mem::size_of::<ScmiMsgImxBbmProtocolAttributes>(), &mut t);
    if ret != 0 { return ret; }
    let attr = (*t).rx.buf as *mut ScmiMsgImxBbmProtocolAttributes;
    ret = (*(*ph).xops).do_xfer(ph, t);
    if ret == 0 {
        (*pi).nr_rtc = (((*attr).attributes >> 16) & 0xff) as i32;
        (*pi).nr_gpr = ((*attr).attributes & 0xffff) as i32;
    }
    (*(*ph).xops).xfer_put(ph, t);
    ret
}

unsafe fn scmi_imx_bbm_notify(ph: *const ScmiProtocolHandle, _src_id: u32, message_id: u32, enable: bool) -> i32 {
    let mut t: *mut ScmiXfer = core::ptr::null_mut();
    if message_id == ScmiImxBbmProtocolCmd::ImxBbmRtcNotify as u32 {
        let ret = (*(*ph).xops).xfer_get_init(ph, message_id, core::mem::size_of::<ScmiMsgImxBbmRtcNotify>(), 0, &mut t);
        if ret != 0 { return ret; }
        let n = (*t).tx.buf as *mut ScmiMsgImxBbmRtcNotify;
        (*n).rtc_id = 0;
        (*n).flags = if enable { SCMI_IMX_BBM_NOTIFY_RTC_FLAG } else { 0 };
    } else if message_id == ScmiImxBbmProtocolCmd::ImxBbmButtonNotify as u32 {
        let ret = (*(*ph).xops).xfer_get_init(ph, message_id, core::mem::size_of::<ScmiMsgImxBbmButtonNotify>(), 0, &mut t);
        if ret != 0 { return ret; }
        (*( (*t).tx.buf as *mut ScmiMsgImxBbmButtonNotify)).flags = if enable { 1 } else { 0 };
    } else { return -EINVAL; }
    let ret = (*(*ph).xops).do_xfer(ph, t);
    (*(*ph).xops).xfer_put(ph, t);
    ret
}

static EVT_2_CMD: [u32; 2] = [ScmiImxBbmProtocolCmd::ImxBbmRtcNotify as u32, ScmiImxBbmProtocolCmd::ImxBbmButtonNotify as u32];

unsafe fn scmi_imx_bbm_set_notify_enabled(ph: *const ScmiProtocolHandle, evt_id: u8, src_id: u32, enable: bool) -> i32 {
    if (evt_id as usize) >= EVT_2_CMD.len() { return -EINVAL; }
    let ret = scmi_imx_bbm_notify(ph, src_id, EVT_2_CMD[evt_id as usize], enable);
    if ret != 0 { pr_debug!("FAIL_ENABLED - evt[{:X}] dom[{}] - ret:{}\n", evt_id, src_id, ret); }
    ret
}

unsafe fn scmi_imx_bbm_rtc_time_set(ph: *const ScmiProtocolHandle, rtc_id: u32, sec: u64) -> i32 {
    let pi = (*ph).get_priv(ph) as *mut ScmiImxBbmInfo;
    if rtc_id >= (*pi).nr_rtc as u32 { return -EINVAL; }
    let mut t = core::ptr::null_mut();
    let mut ret = (*(*ph).xops).xfer_get_init(ph, ScmiImxBbmProtocolCmd::ImxBbmRtcTimeSet as u32, core::mem::size_of::<ScmiImxBbmSetTime>(), 0, &mut t);
    if ret != 0 { return ret; }
    let c = (*t).tx.buf as *mut ScmiImxBbmSetTime;
    (*c).id = rtc_id; (*c).flags = 0; (*c).value_low = sec as u32; (*c).value_high = (sec >> 32) as u32;
    ret = (*(*ph).xops).do_xfer(ph, t); (*(*ph).xops).xfer_put(ph, t); ret
}

unsafe fn scmi_imx_bbm_rtc_time_get(ph: *const ScmiProtocolHandle, rtc_id: u32, value: *mut u64) -> i32 {
    let pi = (*ph).get_priv(ph) as *mut ScmiImxBbmInfo; if rtc_id >= (*pi).nr_rtc as u32 { return -EINVAL; }
    let mut t = core::ptr::null_mut(); let mut ret = (*(*ph).xops).xfer_get_init(ph, ScmiImxBbmProtocolCmd::ImxBbmRtcTimeGet as u32, core::mem::size_of::<ScmiImxBbmGetTime>(), 8, &mut t); if ret != 0 { return ret; }
    let c = (*t).tx.buf as *mut ScmiImxBbmGetTime; (*c).id = rtc_id; (*c).flags = 0; ret = (*(*ph).xops).do_xfer(ph, t); if ret == 0 { *value = u64::from_le_bytes(core::slice::from_raw_parts((*t).rx.buf, 8).try_into().unwrap()); } (*(*ph).xops).xfer_put(ph, t); ret
}

unsafe fn scmi_imx_bbm_rtc_alarm_set(ph: *const ScmiProtocolHandle, rtc_id: u32, enable: bool, sec: u64) -> i32 {
    let pi = (*ph).get_priv(ph) as *mut ScmiImxBbmInfo; if rtc_id >= (*pi).nr_rtc as u32 { return -EINVAL; }
    let mut t = core::ptr::null_mut(); let mut ret = (*(*ph).xops).xfer_get_init(ph, ScmiImxBbmProtocolCmd::ImxBbmRtcAlarmSet as u32, core::mem::size_of::<ScmiImxBbmAlarmTime>(), 0, &mut t); if ret != 0 { return ret; }
    let c = (*t).tx.buf as *mut ScmiImxBbmAlarmTime; (*c).id = rtc_id; (*c).flags = if enable { SCMI_IMX_BBM_RTC_ALARM_ENABLE_FLAG } else { 0 }; (*c).value_low = sec as u32; (*c).value_high = (sec >> 32) as u32; ret = (*(*ph).xops).do_xfer(ph, t); (*(*ph).xops).xfer_put(ph, t); ret
}

unsafe fn scmi_imx_bbm_button_get(ph: *const ScmiProtocolHandle, state: *mut u32) -> i32 {
    let mut t = core::ptr::null_mut(); let mut ret = (*(*ph).xops).xfer_get_init(ph, ScmiImxBbmProtocolCmd::ImxBbmButtonGet as u32, 0, 4, &mut t); if ret != 0 { return ret; }
    ret = (*(*ph).xops).do_xfer(ph, t); if ret == 0 { *state = u32::from_le_bytes(core::slice::from_raw_parts((*t).rx.buf, 4).try_into().unwrap()); } (*(*ph).xops).xfer_put(ph, t); ret
}

// The remaining event/protocol registration structures and module metadata are
// direct translations of the C declarations and use externally supplied SCMI types.
static SCMI_IMX_BBM_PROTO_OPS: ScmiImxBbmProtoOps = ScmiImxBbmProtoOps { rtc_time_get: scmi_imx_bbm_rtc_time_get, rtc_time_set: scmi_imx_bbm_rtc_time_set, rtc_alarm_set: scmi_imx_bbm_rtc_alarm_set, button_get: scmi_imx_bbm_button_get };

unsafe fn scmi_imx_bbm_fill_custom_report(ph: *const ScmiProtocolHandle, evt_id: u8, timestamp: KtimeT, payld: *const core::ffi::c_void, payld_sz: usize, report: *mut core::ffi::c_void, src_id: *mut u32) -> *mut core::ffi::c_void {
    if core::mem::size_of::<ScmiImxBbmNotifyPayld>() != payld_sz { return core::ptr::null_mut(); }
    let p = payld as *const ScmiImxBbmNotifyPayld;
    let r = report as *mut ScmiImxBbmNotifReport;
    if evt_id == SCMI_EVENT_IMX_BBM_RTC {
        (*r).is_rtc = true; (*r).is_button = false; (*r).timestamp = timestamp;
        (*r).rtc_id = ((*p).flags >> 24) & 0xff; (*r).rtc_evt = (*p).flags & SCMI_IMX_BBM_NOTIFY_RTC_FLAG;
        dev_dbg!((*ph).dev, "RTC: {} evt: {:x}\n", (*r).rtc_id, (*r).rtc_evt); *src_id = (*r).rtc_evt;
    } else if evt_id == SCMI_EVENT_IMX_BBM_BUTTON {
        (*r).is_rtc = false; (*r).is_button = true; (*r).timestamp = timestamp;
        dev_dbg!((*ph).dev, "BBM Button\n"); *src_id = 0;
    } else { WARN_ON_ONCE!(1); return core::ptr::null_mut(); }
    r as *mut core::ffi::c_void
}

static SCMI_IMX_BBM_EVENTS: [ScmiEvent; 2] = [
    ScmiEvent { id: SCMI_EVENT_IMX_BBM_RTC, max_payld_sz: core::mem::size_of::<ScmiImxBbmNotifyPayld>(), max_report_sz: core::mem::size_of::<ScmiImxBbmNotifReport>() },
    ScmiEvent { id: SCMI_EVENT_IMX_BBM_BUTTON, max_payld_sz: core::mem::size_of::<ScmiImxBbmNotifyPayld>(), max_report_sz: core::mem::size_of::<ScmiImxBbmNotifReport>() },
];
static SCMI_IMX_BBM_EVENT_OPS: ScmiEventOps = ScmiEventOps { set_notify_enabled: scmi_imx_bbm_set_notify_enabled, fill_custom_report: scmi_imx_bbm_fill_custom_report };
static SCMI_IMX_BBM_PROTOCOL_EVENTS: ScmiProtocolEvents = ScmiProtocolEvents { queue_sz: SCMI_PROTO_QUEUE_SZ, ops: &SCMI_IMX_BBM_EVENT_OPS, evts: &SCMI_IMX_BBM_EVENTS, num_events: SCMI_IMX_BBM_EVENTS.len(), num_sources: 1 };

unsafe fn scmi_imx_bbm_protocol_init(ph: *const ScmiProtocolHandle) -> i32 {
    dev_info!((*ph).dev, "NXP SM BBM Version {}.{}\n", PROTOCOL_REV_MAJOR((*ph).version), PROTOCOL_REV_MINOR((*ph).version));
    let binfo = devm_kzalloc((*ph).dev, core::mem::size_of::<ScmiImxBbmInfo>(), GFP_KERNEL) as *mut ScmiImxBbmInfo;
    if binfo.is_null() { return -ENOMEM; }
    let ret = scmi_imx_bbm_attributes_get(ph, binfo); if ret != 0 { return ret; }
    (*ph).set_priv(ph, binfo)
}

static SCMI_IMX_BBM: ScmiProtocol = ScmiProtocol {
    id: SCMI_PROTOCOL_IMX_BBM, owner: THIS_MODULE, instance_init: scmi_imx_bbm_protocol_init,
    ops: &SCMI_IMX_BBM_PROTO_OPS, events: &SCMI_IMX_BBM_PROTOCOL_EVENTS,
    supported_version: SCMI_PROTOCOL_SUPPORTED_VERSION, vendor_id: SCMI_IMX_VENDOR, sub_vendor_id: SCMI_IMX_SUBVENDOR,
};

module_scmi_protocol!(SCMI_IMX_BBM);
module_alias!(concat!("scmi-protocol-", stringify!(SCMI_PROTOCOL_IMX_BBM), "-", SCMI_IMX_VENDOR));
module_description!("i.MX SCMI BBM driver");
module_license!("GPL");

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
