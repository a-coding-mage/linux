// SPDX-License-Identifier: GPL-2.0
/*
 * System Control and Management Interface (SCMI) Power Protocol
 *
 * Copyright (C) 2018-2022 ARM Ltd.
 */

// Dependency intent: Linux SCMI protocol, protocol, and notification definitions
// are supplied by other translation units.

const SCMI_PROTOCOL_SUPPORTED_VERSION: u32 = 0x30001;

#[repr(C)]
enum ScmiPowerProtocolCmd {
    PowerDomainAttributes = 0x3,
    PowerStateSet = 0x4,
    PowerStateGet = 0x5,
    PowerStateNotify = 0x6,
    PowerDomainNameGet = 0x8,
}

#[repr(C)]
struct ScmiMsgRespPowerAttributes { num_domains: u16, reserved: u16, stats_addr_low: u32, stats_addr_high: u32, stats_size: u32 }

#[repr(C)]
struct ScmiMsgRespPowerDomainAttributes { flags: u32, name: [u8; SCMI_SHORT_NAME_MAX_SIZE] }

#[repr(C)]
struct ScmiPowerSetState { flags: u32, domain: u32, state: u32 }

#[repr(C)]
struct ScmiPowerStateNotify { domain: u32, notify_enable: u32 }

#[repr(C)]
struct ScmiPowerStateNotifyPayld { agent_id: u32, domain_id: u32, power_state: u32 }

#[repr(C)]
struct PowerDomInfo { state_set_sync: bool, state_set_async: bool, state_set_notify: bool, name: [i8; SCMI_MAX_STR_SIZE] }

#[repr(C)]
struct ScmiPowerInfo { notify_state_change_cmd: bool, num_domains: i32, stats_addr: u64, stats_size: u32, dom_info: *mut PowerDomInfo }

unsafe fn scmi_power_attributes_get(ph: *const ScmiProtocolHandle, pi: *mut ScmiPowerInfo) -> i32 {
    let mut ret: i32;
    let mut t: *mut ScmiXfer = core::ptr::null_mut();
    ret = (*(*ph).xops).xfer_get_init(ph, PROTOCOL_ATTRIBUTES, 0, core::mem::size_of::<ScmiMsgRespPowerAttributes>(), &mut t);
    if ret != 0 { return ret; }
    let attr = (*t).rx.buf as *mut ScmiMsgRespPowerAttributes;
    ret = (*(*ph).xops).do_xfer(ph, t);
    if ret == 0 {
        (*pi).num_domains = u16::from_le((*attr).num_domains) as i32;
        (*pi).stats_addr = u32::from_le((*attr).stats_addr_low) as u64 | ((u32::from_le((*attr).stats_addr_high) as u64) << 32);
        (*pi).stats_size = u32::from_le((*attr).stats_size);
    }
    (*(*ph).xops).xfer_put(ph, t);
    if ret == 0 && (*(*ph).hops).protocol_msg_check(ph, PowerStateNotify as u32, core::ptr::null_mut()) == 0 { (*pi).notify_state_change_cmd = true; }
    ret
}

unsafe fn scmi_power_domain_attributes_get(ph: *const ScmiProtocolHandle, domain: u32, dom_info: *mut PowerDomInfo, notify_state_change_cmd: bool) -> i32 {
    let mut ret: i32;
    let mut flags: u32 = 0;
    let mut t: *mut ScmiXfer = core::ptr::null_mut();
    ret = (*(*ph).xops).xfer_get_init(ph, PowerDomainAttributes as u32, core::mem::size_of::<u32>(), core::mem::size_of::<ScmiMsgRespPowerDomainAttributes>(), &mut t);
    if ret != 0 { return ret; }
    put_unaligned_le32(domain, (*t).tx.buf);
    let attr = (*t).rx.buf as *mut ScmiMsgRespPowerDomainAttributes;
    ret = (*(*ph).xops).do_xfer(ph, t);
    if ret == 0 {
        flags = u32::from_le((*attr).flags);
        if notify_state_change_cmd { (*dom_info).state_set_notify = (flags & (1u32 << 31)) != 0; }
        (*dom_info).state_set_async = (flags & (1u32 << 30)) != 0;
        (*dom_info).state_set_sync = (flags & (1u32 << 29)) != 0;
        strscpy((*dom_info).name.as_mut_ptr(), (*attr).name.as_ptr(), SCMI_SHORT_NAME_MAX_SIZE);
    }
    (*(*ph).xops).xfer_put(ph, t);
    if ret == 0 && PROTOCOL_REV_MAJOR((*ph).version) >= 0x3 && (flags & (1u32 << 27)) != 0 { (*(*ph).hops).extended_name_get(ph, PowerDomainNameGet as u32, domain, core::ptr::null_mut(), (*dom_info).name.as_mut_ptr(), SCMI_MAX_STR_SIZE); }
    ret
}

unsafe fn scmi_power_state_set(ph: *const ScmiProtocolHandle, domain: u32, state: u32) -> i32 {
    let mut t: *mut ScmiXfer = core::ptr::null_mut();
    let mut ret = (*(*ph).xops).xfer_get_init(ph, PowerStateSet as u32, core::mem::size_of::<ScmiPowerSetState>(), 0, &mut t);
    if ret != 0 { return ret; }
    let st = (*t).tx.buf as *mut ScmiPowerSetState;
    (*st).flags = 0u32.to_le(); (*st).domain = domain.to_le(); (*st).state = state.to_le();
    ret = (*(*ph).xops).do_xfer(ph, t); (*(*ph).xops).xfer_put(ph, t); ret
}

unsafe fn scmi_power_state_get(ph: *const ScmiProtocolHandle, domain: u32, state: *mut u32) -> i32 {
    let mut t: *mut ScmiXfer = core::ptr::null_mut();
    let mut ret = (*(*ph).xops).xfer_get_init(ph, PowerStateGet as u32, core::mem::size_of::<u32>(), core::mem::size_of::<u32>(), &mut t);
    if ret != 0 { return ret; }
    put_unaligned_le32(domain, (*t).tx.buf); ret = (*(*ph).xops).do_xfer(ph, t);
    if ret == 0 { *state = get_unaligned_le32((*t).rx.buf); }
    (*(*ph).xops).xfer_put(ph, t); ret
}

unsafe fn scmi_power_num_domains_get(ph: *const ScmiProtocolHandle) -> i32 { (*((*ph).get_priv)(ph) as *mut ScmiPowerInfo).as_ref().unwrap().num_domains }

unsafe fn scmi_power_name_get(ph: *const ScmiProtocolHandle, domain: u32) -> *const i8 {
    let pi = (*ph).get_priv(ph) as *mut ScmiPowerInfo;
    if domain >= (*pi).num_domains as u32 { return b"unknown\0".as_ptr() as *const i8; }
    (*pi).dom_info.add(domain as usize).as_ref().unwrap().name.as_ptr()
}

unsafe fn scmi_power_request_notify(ph: *const ScmiProtocolHandle, domain: u32, enable: bool) -> i32 {
    let mut t: *mut ScmiXfer = core::ptr::null_mut();
    let mut ret = (*(*ph).xops).xfer_get_init(ph, PowerStateNotify as u32, core::mem::size_of::<ScmiPowerStateNotify>(), 0, &mut t);
    if ret != 0 { return ret; }
    let n = (*t).tx.buf as *mut ScmiPowerStateNotify; (*n).domain = domain.to_le(); (*n).notify_enable = if enable { 1u32.to_le() } else { 0 };
    ret = (*(*ph).xops).do_xfer(ph, t); (*(*ph).xops).xfer_put(ph, t); ret
}

unsafe fn scmi_power_notify_supported(ph: *const ScmiProtocolHandle, evt_id: u8, src_id: u32) -> bool {
    let pinfo = (*ph).get_priv(ph) as *mut ScmiPowerInfo;
    if evt_id != SCMI_EVENT_POWER_STATE_CHANGED || src_id >= (*pinfo).num_domains as u32 { return false; }
    (*pinfo).dom_info.add(src_id as usize).as_ref().unwrap().state_set_notify
}

unsafe fn scmi_power_set_notify_enabled(ph: *const ScmiProtocolHandle, evt_id: u8, src_id: u32, enable: bool) -> i32 {
    let ret = scmi_power_request_notify(ph, src_id, enable);
    if ret != 0 { pr_debug!("FAIL_ENABLE - evt[%X] dom[%d] - ret:%d\n", evt_id, src_id, ret); }
    ret
}

unsafe fn scmi_power_fill_custom_report(ph: *const ScmiProtocolHandle, evt_id: u8, timestamp: KtimeT, payld: *const core::ffi::c_void, payld_sz: usize, report: *mut core::ffi::c_void, src_id: *mut u32) -> *mut core::ffi::c_void {
    if evt_id != SCMI_EVENT_POWER_STATE_CHANGED || core::mem::size_of::<ScmiPowerStateNotifyPayld>() != payld_sz { return core::ptr::null_mut(); }
    let p = payld as *const ScmiPowerStateNotifyPayld;
    let r = report as *mut ScmiPowerStateChangedReport;
    (*r).timestamp = timestamp; (*r).agent_id = u32::from_le((*p).agent_id); (*r).domain_id = u32::from_le((*p).domain_id); (*r).power_state = u32::from_le((*p).power_state); *src_id = (*r).domain_id; r as *mut core::ffi::c_void
}

unsafe fn scmi_power_get_num_sources(ph: *const ScmiProtocolHandle) -> i32 {
    let pinfo = (*ph).get_priv(ph) as *mut ScmiPowerInfo;
    if pinfo.is_null() { return -EINVAL; } (*pinfo).num_domains
}

unsafe fn scmi_power_protocol_init(ph: *const ScmiProtocolHandle) -> i32 {
    dev_dbg!((*ph).dev, "Power Version %d.%d\n", PROTOCOL_REV_MAJOR((*ph).version), PROTOCOL_REV_MINOR((*ph).version));
    let pinfo = devm_kzalloc((*ph).dev, core::mem::size_of::<ScmiPowerInfo>(), GFP_KERNEL) as *mut ScmiPowerInfo;
    if pinfo.is_null() { return -ENOMEM; }
    let mut ret = scmi_power_attributes_get(ph, pinfo); if ret != 0 { return ret; }
    (*pinfo).dom_info = devm_kcalloc((*ph).dev, (*pinfo).num_domains as usize, core::mem::size_of::<PowerDomInfo>(), GFP_KERNEL) as *mut PowerDomInfo;
    if (*pinfo).dom_info.is_null() { return -ENOMEM; }
    for domain in 0..(*pinfo).num_domains { scmi_power_domain_attributes_get(ph, domain as u32, (*pinfo).dom_info.add(domain as usize), (*pinfo).notify_state_change_cmd); }
    ret = (*ph).set_priv(ph, pinfo as *mut core::ffi::c_void); ret
}

extern "C" {
    static POWER_PROTO_OPS: ScmiProtocolOps;
    static POWER_PROTOCOL_EVENTS: ScmiProtocolEvents;
}

// Protocol operation and event tables retain the C source's externally supplied SCMI types.
static SCMI_POWER: ScmiProtocol = ScmiProtocol { id: SCMI_PROTOCOL_POWER, owner: THIS_MODULE, instance_init: scmi_power_protocol_init, ops: &POWER_PROTO_OPS, events: &POWER_PROTOCOL_EVENTS, supported_version: SCMI_PROTOCOL_SUPPORTED_VERSION };

DEFINE_SCMI_PROTOCOL_REGISTER_UNREGISTER!(power, SCMI_POWER);

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
