// SPDX-License-Identifier: GPL-2.0
/*
 * System Control and Management Interface (SCMI) Reset Protocol
 *
 * Copyright (C) 2019-2022 ARM Ltd.
 */

// Dependencies supplied by the surrounding SCMI implementation are intentionally
// referenced here rather than redefined.

pub const SCMI_PROTOCOL_SUPPORTED_VERSION: u32 = 0x30001;

#[repr(u32)]
pub enum ScmiResetProtocolCmd {
    ResetDomainAttributes = 0x3,
    Reset = 0x4,
    ResetNotify = 0x5,
    ResetDomainNameGet = 0x6,
}

pub const NUM_RESET_DOMAIN_MASK: u32 = 0xffff;
pub const RESET_NOTIFY_ENABLE: u32 = 1 << 0;

#[repr(C)]
pub struct ScmiMsgRespResetDomainAttributes {
    pub attributes: u32,
    pub latency: u32,
    pub name: [u8; SCMI_SHORT_NAME_MAX_SIZE],
}

pub const SUPPORTS_ASYNC_RESET: u32 = 1 << 31;
pub const SUPPORTS_NOTIFY_RESET: u32 = 1 << 30;
pub const SUPPORTS_EXTENDED_NAMES: u32 = 1 << 29;

#[repr(C)]
pub struct ScmiMsgResetDomainReset {
    pub domain_id: u32,
    pub flags: u32,
    pub reset_state: u32,
}

pub const AUTONOMOUS_RESET: u32 = 1 << 0;
pub const EXPLICIT_RESET_ASSERT: u32 = 1 << 1;
pub const ASYNCHRONOUS_RESET: u32 = 1 << 2;
pub const ARCH_COLD_RESET: u32 = 0;

#[repr(C)]
pub struct ScmiMsgResetNotify {
    pub id: u32,
    pub event_control: u32,
}

pub const RESET_TP_NOTIFY_ALL: u32 = 1 << 0;

#[repr(C)]
pub struct ScmiResetIssuedNotifyPayld {
    pub agent_id: u32,
    pub domain_id: u32,
    pub reset_state: u32,
}

#[repr(C)]
pub struct ResetDomInfo {
    pub async_reset: bool,
    pub reset_notify: bool,
    pub latency_us: u32,
    pub name: [core::ffi::c_char; SCMI_MAX_STR_SIZE],
}

#[repr(C)]
pub struct ScmiResetInfo {
    pub num_domains: i32,
    pub notify_reset_cmd: bool,
    pub dom_info: *mut ResetDomInfo,
}

unsafe fn scmi_reset_attributes_get(
    ph: *const ScmiProtocolHandle,
    pi: *mut ScmiResetInfo,
) -> i32 {
    let mut ret: i32;
    let mut t: *mut ScmiXfer = core::ptr::null_mut();
    let mut attr: u32 = 0;
    ret = (*(*ph).xops).xfer_get_init(ph, PROTOCOL_ATTRIBUTES, 0, core::mem::size_of::<u32>(), &mut t);
    if ret != 0 { return ret; }
    ret = (*(*ph).xops).do_xfer(ph, t);
    if ret == 0 {
        attr = get_unaligned_le32((*t).rx.buf);
        (*pi).num_domains = (attr & NUM_RESET_DOMAIN_MASK) as i32;
    }
    (*(*ph).xops).xfer_put(ph, t);
    if ret == 0 && (*(*ph).hops).protocol_msg_check(ph, RESET_NOTIFY as u32, core::ptr::null_mut()) == 0 {
        (*pi).notify_reset_cmd = true;
    }
    ret
}

unsafe fn scmi_reset_domain_lookup(ph: *const ScmiProtocolHandle, domain: u32) -> *mut ResetDomInfo {
    let pi = (*ph).get_priv(ph) as *mut ScmiResetInfo;
    if domain >= (*pi).num_domains as u32 { return ERR_PTR(-EINVAL); }
    (*pi).dom_info.add(domain as usize)
}

unsafe fn scmi_reset_domain_attributes_get(ph: *const ScmiProtocolHandle, pinfo: *mut ScmiResetInfo, domain: u32) -> i32 {
    let mut ret: i32;
    let mut attributes: u32 = 0;
    let mut t: *mut ScmiXfer = core::ptr::null_mut();
    let attr: *mut ScmiMsgRespResetDomainAttributes;
    let dom_info = (*pinfo).dom_info.add(domain as usize);
    ret = (*(*ph).xops).xfer_get_init(ph, RESET_DOMAIN_ATTRIBUTES as u32, core::mem::size_of::<u32>(), core::mem::size_of::<ScmiMsgRespResetDomainAttributes>(), &mut t);
    if ret != 0 { return ret; }
    put_unaligned_le32(domain, (*t).tx.buf);
    attr = (*t).rx.buf as *mut ScmiMsgRespResetDomainAttributes;
    ret = (*(*ph).xops).do_xfer(ph, t);
    if ret == 0 {
        attributes = le32_to_cpu((*attr).attributes);
        (*dom_info).async_reset = attributes & SUPPORTS_ASYNC_RESET != 0;
        if (*pinfo).notify_reset_cmd { (*dom_info).reset_notify = attributes & SUPPORTS_NOTIFY_RESET != 0; }
        (*dom_info).latency_us = le32_to_cpu((*attr).latency);
        if (*dom_info).latency_us == U32_MAX { (*dom_info).latency_us = 0; }
        strscpy((*dom_info).name.as_mut_ptr(), (*attr).name.as_ptr(), SCMI_SHORT_NAME_MAX_SIZE);
    }
    (*(*ph).xops).xfer_put(ph, t);
    /* If supported overwrite short name with the extended one; on error carry on. */
    if ret == 0 && PROTOCOL_REV_MAJOR((*ph).version) >= 0x3 && attributes & SUPPORTS_EXTENDED_NAMES != 0 {
        (*(*ph).hops).extended_name_get(ph, RESET_DOMAIN_NAME_GET as u32, domain, core::ptr::null_mut(), (*dom_info).name.as_mut_ptr(), SCMI_MAX_STR_SIZE);
    }
    ret
}

unsafe fn scmi_reset_num_domains_get(ph: *const ScmiProtocolHandle) -> i32 { (*((*ph).get_priv(ph) as *mut ScmiResetInfo)).num_domains }

unsafe fn scmi_reset_name_get(ph: *const ScmiProtocolHandle, domain: u32) -> *const core::ffi::c_char {
    let d = scmi_reset_domain_lookup(ph, domain);
    if IS_ERR(d) { return c"unknown".as_ptr(); }
    (*d).name.as_ptr()
}

unsafe fn scmi_reset_latency_get(ph: *const ScmiProtocolHandle, domain: u32) -> i32 {
    let d = scmi_reset_domain_lookup(ph, domain);
    if IS_ERR(d) { return PTR_ERR(d); }
    (*d).latency_us as i32
}

unsafe fn scmi_domain_reset(ph: *const ScmiProtocolHandle, domain: u32, mut flags: u32, state: u32) -> i32 {
    let d = scmi_reset_domain_lookup(ph, domain);
    if IS_ERR(d) { return PTR_ERR(d); }
    if (*d).async_reset && flags & AUTONOMOUS_RESET != 0 { flags |= ASYNCHRONOUS_RESET; }
    let mut t: *mut ScmiXfer = core::ptr::null_mut();
    let mut ret = (*(*ph).xops).xfer_get_init(ph, RESET as u32, core::mem::size_of::<ScmiMsgResetDomainReset>(), 0, &mut t);
    if ret != 0 { return ret; }
    let dom = (*t).tx.buf as *mut ScmiMsgResetDomainReset;
    (*dom).domain_id = cpu_to_le32(domain); (*dom).flags = cpu_to_le32(flags); (*dom).reset_state = cpu_to_le32(state);
    ret = if flags & ASYNCHRONOUS_RESET != 0 { (*(*ph).xops).do_xfer_with_response(ph, t) } else { (*(*ph).xops).do_xfer(ph, t) };
    (*(*ph).xops).xfer_put(ph, t); ret
}

unsafe fn scmi_reset_domain_reset(ph: *const ScmiProtocolHandle, domain: u32) -> i32 { scmi_domain_reset(ph, domain, AUTONOMOUS_RESET, ARCH_COLD_RESET) }
unsafe fn scmi_reset_domain_assert(ph: *const ScmiProtocolHandle, domain: u32) -> i32 { scmi_domain_reset(ph, domain, EXPLICIT_RESET_ASSERT, ARCH_COLD_RESET) }
unsafe fn scmi_reset_domain_deassert(ph: *const ScmiProtocolHandle, domain: u32) -> i32 { scmi_domain_reset(ph, domain, 0, ARCH_COLD_RESET) }

// The following declarations retain the source protocol tables and notification
// entry points; their field types are supplied by the surrounding SCMI code.
extern "C" {
    static reset_proto_ops: ScmiResetProtoOps;
    static reset_events: [ScmiEvent; 1];
    static reset_event_ops: ScmiEventOps;
    static reset_protocol_events: ScmiProtocolEvents;
}

unsafe fn scmi_reset_notify_supported(ph: *const ScmiProtocolHandle, evt_id: u8, src_id: u32) -> bool {
    if evt_id as u32 != SCMI_EVENT_RESET_ISSUED { return false; }
    let d = scmi_reset_domain_lookup(ph, src_id);
    if IS_ERR(d) { return false; }
    (*d).reset_notify
}

unsafe fn scmi_reset_notify(ph: *const ScmiProtocolHandle, domain_id: u32, enable: bool) -> i32 {
    let mut t: *mut ScmiXfer = core::ptr::null_mut();
    let mut ret = (*(*ph).xops).xfer_get_init(ph, RESET_NOTIFY as u32, core::mem::size_of::<ScmiMsgResetNotify>(), 0, &mut t);
    if ret != 0 { return ret; }
    let cfg = (*t).tx.buf as *mut ScmiMsgResetNotify;
    (*cfg).id = cpu_to_le32(domain_id);
    (*cfg).event_control = cpu_to_le32(if enable { RESET_TP_NOTIFY_ALL } else { 0 });
    ret = (*(*ph).xops).do_xfer(ph, t);
    (*(*ph).xops).xfer_put(ph, t); ret
}

unsafe fn scmi_reset_set_notify_enabled(ph: *const ScmiProtocolHandle, evt_id: u8, src_id: u32, enable: bool) -> i32 {
    let ret = scmi_reset_notify(ph, src_id, enable);
    if ret != 0 { pr_debug!("FAIL_ENABLED - evt[{:X}] dom[{}] - ret:{}\n", evt_id, src_id, ret); }
    ret
}

unsafe fn scmi_reset_get_num_sources(ph: *const ScmiProtocolHandle) -> i32 {
    let pinfo = (*ph).get_priv(ph) as *mut ScmiResetInfo;
    if pinfo.is_null() { return -EINVAL; }
    (*pinfo).num_domains
}

unsafe fn scmi_reset_protocol_init(ph: *const ScmiProtocolHandle) -> i32 {
    let mut pinfo = devm_kzalloc((*ph).dev, core::mem::size_of::<ScmiResetInfo>(), GFP_KERNEL) as *mut ScmiResetInfo;
    if pinfo.is_null() { return -ENOMEM; }
    let ret = scmi_reset_attributes_get(ph, pinfo);
    if ret != 0 { return ret; }
    (*pinfo).dom_info = devm_kcalloc((*ph).dev, (*pinfo).num_domains as usize, core::mem::size_of::<ResetDomInfo>(), GFP_KERNEL) as *mut ResetDomInfo;
    if (*pinfo).dom_info.is_null() { return -ENOMEM; }
    for domain in 0..(*pinfo).num_domains { scmi_reset_domain_attributes_get(ph, pinfo, domain as u32); }
    (*ph).set_priv(ph, pinfo as *mut core::ffi::c_void)
}

#[no_mangle]
pub static scmi_reset: ScmiProtocol = ScmiProtocol {
    id: SCMI_PROTOCOL_RESET,
    owner: THIS_MODULE,
    instance_init: Some(scmi_reset_protocol_init),
    ops: &reset_proto_ops,
    events: &reset_protocol_events,
    supported_version: SCMI_PROTOCOL_SUPPORTED_VERSION,
};

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
