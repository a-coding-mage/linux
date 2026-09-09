// SPDX-License-Identifier: GPL-2.0
/*
 * System Control and Management Interface (SCMI) System Power Protocol
 *
 * Copyright (C) 2020-2022 ARM Ltd.
 */

// C dependencies supplied by the surrounding SCMI/Linux translation.

const SCMI_PROTOCOL_SUPPORTED_VERSION: u32 = 0x20001;
const SCMI_SYSTEM_NUM_SOURCES: usize = 1;

#[repr(u32)]
enum ScmiSystemProtocolCmd {
    SystemPowerStateNotify = 0x5,
}

#[repr(C)]
struct ScmiSystemPowerStateNotify {
    notify_enable: u32,
}

#[repr(C)]
struct ScmiSystemPowerStateNotifierPayld {
    agent_id: u32,
    flags: u32,
    system_state: u32,
    timeout: u32,
}

#[repr(C)]
struct ScmiSystemInfo {
    graceful_timeout_supported: bool,
    power_state_notify_cmd: bool,
}

unsafe fn scmi_system_notify_supported(
    ph: *const ScmiProtocolHandle,
    evt_id: u8,
    _src_id: u32,
) -> bool {
    let pinfo = ((*ph).get_priv)(ph) as *mut ScmiSystemInfo;

    if evt_id != SCMI_EVENT_SYSTEM_POWER_STATE_NOTIFIER {
        return false;
    }

    (*pinfo).power_state_notify_cmd
}

unsafe fn scmi_system_request_notify(
    ph: *const ScmiProtocolHandle,
    enable: bool,
) -> i32 {
    let mut t: *mut ScmiXfer = core::ptr::null_mut();
    let ret = ((*(*ph).xops).xfer_get_init)(
        ph,
        ScmiSystemProtocolCmd::SystemPowerStateNotify as u32,
        core::mem::size_of::<ScmiSystemPowerStateNotify>(),
        0,
        &mut t,
    );
    if ret != 0 {
        return ret;
    }

    let notify = (*t).tx.buf as *mut ScmiSystemPowerStateNotify;
    (*notify).notify_enable = if enable { 1u32 } else { 0u32 };

    let ret = ((*(*ph).xops).do_xfer)(ph, t);
    ((*(*ph).xops).xfer_put)(ph, t);
    ret
}

unsafe fn scmi_system_set_notify_enabled(
    ph: *const ScmiProtocolHandle,
    evt_id: u8,
    _src_id: u32,
    enable: bool,
) -> i32 {
    let ret = scmi_system_request_notify(ph, enable);
    if ret != 0 {
        // pr_debug("FAIL_ENABLE - evt[%X] - ret:%d\n", evt_id, ret);
    }
    ret
}

unsafe fn scmi_system_fill_custom_report(
    ph: *const ScmiProtocolHandle,
    evt_id: u8,
    timestamp: KtimeT,
    payld: *const core::ffi::c_void,
    payld_sz: usize,
    report: *mut core::ffi::c_void,
    src_id: *mut u32,
) -> *mut core::ffi::c_void {
    let p = payld as *const ScmiSystemPowerStateNotifierPayld;
    let r = report as *mut ScmiSystemPowerStateNotifierReport;
    let pinfo = ((*ph).get_priv)(ph) as *mut ScmiSystemInfo;

    let expected_sz = if (*pinfo).graceful_timeout_supported {
        core::mem::size_of::<ScmiSystemPowerStateNotifierPayld>()
    } else {
        core::mem::size_of::<ScmiSystemPowerStateNotifierPayld>() - core::mem::size_of::<u32>()
    };
    if evt_id != SCMI_EVENT_SYSTEM_POWER_STATE_NOTIFIER || payld_sz != expected_sz {
        return core::ptr::null_mut();
    }

    (*r).timestamp = timestamp;
    (*r).agent_id = u32::from_le((*p).agent_id);
    (*r).flags = u32::from_le((*p).flags);
    (*r).system_state = u32::from_le((*p).system_state);
    if (*pinfo).graceful_timeout_supported
        && (*r).system_state == SCMI_SYSTEM_SHUTDOWN
        && SCMI_SYSPOWER_IS_REQUEST_GRACEFUL((*r).flags)
    {
        (*r).timeout = u32::from_le((*p).timeout);
    } else {
        (*r).timeout = 0x00;
    }
    *src_id = 0;
    report
}

static mut SYSTEM_EVENTS: [ScmiEvent; 1] = [ScmiEvent {
    id: SCMI_EVENT_SYSTEM_POWER_STATE_NOTIFIER,
    max_payld_sz: core::mem::size_of::<ScmiSystemPowerStateNotifierPayld>(),
    max_report_sz: core::mem::size_of::<ScmiSystemPowerStateNotifierReport>(),
}];

static SYSTEM_EVENT_OPS: ScmiEventOps = ScmiEventOps {
    is_notify_supported: Some(scmi_system_notify_supported),
    set_notify_enabled: Some(scmi_system_set_notify_enabled),
    fill_custom_report: Some(scmi_system_fill_custom_report),
};

static SYSTEM_PROTOCOL_EVENTS: ScmiProtocolEvents = ScmiProtocolEvents {
    queue_sz: SCMI_PROTO_QUEUE_SZ,
    ops: &SYSTEM_EVENT_OPS,
    evts: unsafe { &SYSTEM_EVENTS },
    num_events: 1,
    num_sources: SCMI_SYSTEM_NUM_SOURCES,
};

unsafe fn scmi_system_protocol_init(ph: *const ScmiProtocolHandle) -> i32 {
    let pinfo = devm_kzalloc((*ph).dev, core::mem::size_of::<ScmiSystemInfo>(), GFP_KERNEL)
        as *mut ScmiSystemInfo;
    if pinfo.is_null() {
        return -ENOMEM;
    }

    if PROTOCOL_REV_MAJOR((*ph).version) >= 0x2 {
        (*pinfo).graceful_timeout_supported = true;
    }
    if !((*(*ph).hops).protocol_msg_check)(
        ph,
        ScmiSystemProtocolCmd::SystemPowerStateNotify as u32,
        core::ptr::null_mut(),
    ) {
        (*pinfo).power_state_notify_cmd = true;
    }
    ((*ph).set_priv)(ph, pinfo as *mut core::ffi::c_void)
}

static SCMI_SYSTEM: ScmiProtocol = ScmiProtocol {
    id: SCMI_PROTOCOL_SYSTEM,
    owner: THIS_MODULE,
    instance_init: Some(scmi_system_protocol_init),
    ops: None,
    events: Some(&SYSTEM_PROTOCOL_EVENTS),
    supported_version: SCMI_PROTOCOL_SUPPORTED_VERSION,
};

// DEFINE_SCMI_PROTOCOL_REGISTER_UNREGISTER(system, scmi_system)

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
