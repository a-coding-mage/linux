// SPDX-License-Identifier: GPL-2.0
/* System Control and Management Interface (SCMI) Base Protocol */

// C dependencies (linux/math.h, linux/module.h, linux/scmi_protocol.h,
// common.h, and notify.h) are supplied by the surrounding translation.

const SCMI_PROTOCOL_SUPPORTED_VERSION: u32 = 0x20001;
const SCMI_BASE_NUM_SOURCES: usize = 1;
const SCMI_BASE_MAX_CMD_ERR_COUNT: usize = 1024;

#[repr(u8)]
enum ScmiBaseProtocolCmd {
    BaseDiscoverVendor = 0x3,
    BaseDiscoverSubVendor = 0x4,
    BaseDiscoverImplementVersion = 0x5,
    BaseDiscoverListProtocols = 0x6,
    BaseDiscoverAgent = 0x7,
    BaseNotifyErrors = 0x8,
    BaseSetDevicePermissions = 0x9,
    BaseSetProtocolPermissions = 0xa,
    BaseResetAgentConfiguration = 0xb,
}

#[repr(C)]
struct ScmiMsgRespBaseAttributes { num_protocols: u8, num_agents: u8, reserved: u16 }
#[repr(C)]
struct ScmiMsgRespBaseDiscoverAgent { agent_id: u32, name: [u8; SCMI_SHORT_NAME_MAX_SIZE] }
#[repr(C)]
struct ScmiMsgBaseErrorNotify { event_control: u32 }
const BASE_TP_NOTIFY_ALL: u32 = 1 << 0;
#[repr(C)]
struct ScmiBaseErrorNotifyPayld {
    agent_id: u32,
    error_status: u32,
    msg_reports: [u64; SCMI_BASE_MAX_CMD_ERR_COUNT],
}

#[inline] fn is_fatal_error(x: u32) -> bool { (x & (1 << 31)) != 0 }
#[inline] fn error_cmd_count(x: u32) -> usize { (x & ((1 << 10) - 1)) as usize }

unsafe fn scmi_base_attributes_get(ph: *const ScmiProtocolHandle) -> i32 {
    let mut t: *mut ScmiXfer = core::ptr::null_mut();
    let rev = ((*ph).get_priv)(ph);
    let mut ret = ((*(*ph).xops).xfer_get_init)(ph, PROTOCOL_ATTRIBUTES, 0,
        core::mem::size_of::<ScmiMsgRespBaseAttributes>(), &mut t);
    if ret != 0 { return ret; }
    ret = ((*(*ph).xops).do_xfer)(ph, t);
    if ret == 0 {
        let a = (*t).rx.buf as *const ScmiMsgRespBaseAttributes;
        (*rev).num_protocols = (*a).num_protocols;
        (*rev).num_agents = (*a).num_agents;
    }
    ((*(*ph).xops).xfer_put)(ph, t); ret
}

unsafe fn scmi_base_vendor_id_get(ph: *const ScmiProtocolHandle, sub_vendor: bool) -> i32 {
    let rev = ((*ph).get_priv)(ph);
    let (cmd, vendor_id, size) = if sub_vendor {
        (ScmiBaseProtocolCmd::BaseDiscoverSubVendor as u8, (*rev).sub_vendor_id.as_mut_ptr(), (*rev).sub_vendor_id.len())
    } else {
        (ScmiBaseProtocolCmd::BaseDiscoverVendor as u8, (*rev).vendor_id.as_mut_ptr(), (*rev).vendor_id.len())
    };
    let mut t: *mut ScmiXfer = core::ptr::null_mut();
    let mut ret = ((*(*ph).xops).xfer_get_init)(ph, cmd, 0, size, &mut t);
    if ret != 0 { return ret; }
    ret = ((*(*ph).xops).do_xfer)(ph, t);
    if ret == 0 { strscpy(vendor_id, (*t).rx.buf, size); }
    ((*(*ph).xops).xfer_put)(ph, t); ret
}

unsafe fn scmi_base_implementation_version_get(ph: *const ScmiProtocolHandle) -> i32 {
    let rev = ((*ph).get_priv)(ph); let mut t = core::ptr::null_mut();
    let mut ret = ((*(*ph).xops).xfer_get_init)(ph, ScmiBaseProtocolCmd::BaseDiscoverImplementVersion as u8, 0, 4, &mut t);
    if ret != 0 { return ret; }
    ret = ((*(*ph).xops).do_xfer)(ph, t);
    if ret == 0 { (*rev).impl_ver = u32::from_le(*((*t).rx.buf as *const u32)); }
    ((*(*ph).xops).xfer_put)(ph, t); ret
}

unsafe fn scmi_base_implementation_list_get(ph: *const ScmiProtocolHandle, protocols_imp: *mut u8) -> i32 {
    let rev = ((*ph).get_priv)(ph); let dev = (*ph).dev; let mut t = core::ptr::null_mut();
    let mut ret = ((*(*ph).xops).xfer_get_init)(ph, ScmiBaseProtocolCmd::BaseDiscoverListProtocols as u8, 4, 0, &mut t);
    if ret != 0 { return ret; }
    let num_skip = (*t).tx.buf as *mut u32; let num_ret = (*t).rx.buf as *mut u32;
    let list = (*t).rx.buf.add(4); let mut tot_num_ret: u32 = 0;
    loop {
        *num_skip = tot_num_ret.to_le(); ret = ((*(*ph).xops).do_xfer)(ph, t); if ret != 0 { break; }
        let loop_num_ret = u32::from_le(*num_ret); if loop_num_ret == 0 { break; }
        if loop_num_ret > (*rev).num_protocols as u32 - tot_num_ret { dev_err(dev, "No. Returned protocols > Total protocols.\n"); break; }
        if (*t).rx.len < 8 { dev_err(dev, "Truncated reply - rx.len:%zd\n", (*t).rx.len); ret = -EPROTO; break; }
        let real_list_sz = (*t).rx.len - 4; let calc_list_sz = round_up(loop_num_ret as usize, 4);
        if calc_list_sz != real_list_sz { dev_warn(dev, "Malformed reply - real_sz:%zd  calc_sz:%u  (loop_num_ret:%d)\n", real_list_sz, calc_list_sz, loop_num_ret); if calc_list_sz > real_list_sz { ret = -EPROTO; break; } }
        for i in 0..loop_num_ret as usize { *protocols_imp.add(tot_num_ret as usize + i) = *list.add(i); }
        tot_num_ret += loop_num_ret; ((*(*ph).xops).reset_rx_to_maxsz)(ph, t);
        if tot_num_ret >= (*rev).num_protocols as u32 { break; }
    }
    ((*(*ph).xops).xfer_put)(ph, t); ret
}

unsafe fn scmi_base_discover_agent_get(ph: *const ScmiProtocolHandle, id: i32, name: *mut i8) -> i32 {
    let mut t = core::ptr::null_mut(); let mut ret = ((*(*ph).xops).xfer_get_init)(ph, ScmiBaseProtocolCmd::BaseDiscoverAgent as u8, 4, core::mem::size_of::<ScmiMsgRespBaseDiscoverAgent>(), &mut t);
    if ret != 0 { return ret; } put_unaligned_le32(id as u32, (*t).tx.buf);
    ret = ((*(*ph).xops).do_xfer)(ph, t); if ret == 0 { let a = (*t).rx.buf as *const ScmiMsgRespBaseDiscoverAgent; strscpy(name, (*a).name.as_ptr(), SCMI_SHORT_NAME_MAX_SIZE); }
    ((*(*ph).xops).xfer_put)(ph, t); ret
}

unsafe fn scmi_base_error_notify(ph: *const ScmiProtocolHandle, enable: bool) -> i32 {
    let mut t = core::ptr::null_mut(); let mut ret = ((*(*ph).xops).xfer_get_init)(ph, ScmiBaseProtocolCmd::BaseNotifyErrors as u8, 4, 0, &mut t);
    if ret != 0 { return ret; } (*( (*t).tx.buf as *mut ScmiMsgBaseErrorNotify)).event_control = if enable { BASE_TP_NOTIFY_ALL } else { 0 }.to_le(); ret = ((*(*ph).xops).do_xfer)(ph, t); ((*(*ph).xops).xfer_put)(ph, t); ret
}

unsafe fn scmi_base_set_notify_enabled(ph: *const ScmiProtocolHandle, evt_id: u8, _src_id: u32, enable: bool) -> i32 { let ret = scmi_base_error_notify(ph, enable); if ret != 0 { pr_debug!("FAIL_ENABLED - evt[%X] ret:%d\n", evt_id, ret); } ret }

unsafe fn scmi_base_fill_custom_report(ph: *const ScmiProtocolHandle, evt_id: u8, timestamp: KtimeT, payld: *const core::ffi::c_void, payld_sz: usize, report: *mut core::ffi::c_void, src_id: *mut u32) -> *mut core::ffi::c_void {
    let p = payld as *const ScmiBaseErrorNotifyPayld; let r = report as *mut ScmiBaseErrorReport;
    if evt_id != SCMI_EVENT_BASE_ERROR_EVENT || core::mem::size_of::<ScmiBaseErrorNotifyPayld>() < payld_sz || payld_sz < core::mem::offset_of!(ScmiBaseErrorNotifyPayld, msg_reports) { return core::ptr::null_mut(); }
    (*r).timestamp = timestamp; (*r).agent_id = u32::from_le((*p).agent_id); let status = u32::from_le((*p).error_status); (*r).fatal = is_fatal_error(status); (*r).cmd_count = error_cmd_count(status);
    let expected = core::mem::offset_of!(ScmiBaseErrorNotifyPayld, msg_reports) + (*r).cmd_count * 8; if payld_sz < expected { return core::ptr::null_mut(); }
    for i in 0..(*r).cmd_count { (*r).reports[i] = u64::from_le((*p).msg_reports[i]); } *src_id = 0; report
}

static BASE_EVENTS: [ScmiEvent; 1] = [ScmiEvent { id: SCMI_EVENT_BASE_ERROR_EVENT, max_payld_sz: core::mem::size_of::<ScmiBaseErrorNotifyPayld>(), max_report_sz: core::mem::size_of::<ScmiBaseErrorReport>() + SCMI_BASE_MAX_CMD_ERR_COUNT * 8 }];
static BASE_EVENT_OPS: ScmiEventOps = ScmiEventOps { set_notify_enabled: scmi_base_set_notify_enabled, fill_custom_report: scmi_base_fill_custom_report };
static BASE_PROTOCOL_EVENTS: ScmiProtocolEvents = ScmiProtocolEvents { queue_sz: 4 * SCMI_PROTO_QUEUE_SZ, ops: &BASE_EVENT_OPS, evts: &BASE_EVENTS, num_events: 1, num_sources: SCMI_BASE_NUM_SOURCES };

unsafe fn scmi_base_protocol_init(ph: *const ScmiProtocolHandle) -> i32 {
    let rev = scmi_revision_area_get(ph); (*rev).major_ver = PROTOCOL_REV_MAJOR((*ph).version); (*rev).minor_ver = PROTOCOL_REV_MINOR((*ph).version); ((*ph).set_priv)(ph, rev);
    let mut ret = scmi_base_attributes_get(ph); if ret != 0 { return ret; }
    let prot_imp = devm_kcalloc((*ph).dev, (*rev).num_protocols as usize, 1, GFP_KERNEL); if prot_imp.is_null() { return -ENOMEM; }
    scmi_base_vendor_id_get(ph, false); scmi_base_vendor_id_get(ph, true); scmi_base_implementation_version_get(ph); scmi_base_implementation_list_get(ph, prot_imp);
    scmi_setup_protocol_implemented(ph, prot_imp); let mut name = [0i8; SCMI_SHORT_NAME_MAX_SIZE];
    dev_info!((*ph).dev, "SCMI Protocol v%d.%d '%s:%s' Firmware version 0x%x\n", (*rev).major_ver, (*rev).minor_ver, (*rev).vendor_id, (*rev).sub_vendor_id, (*rev).impl_ver);
    dev_dbg!((*ph).dev, "Found %d protocol(s) %d agent(s)\n", (*rev).num_protocols, (*rev).num_agents);
    for id in 0..(*rev).num_agents { scmi_base_discover_agent_get(ph, id as i32, name.as_mut_ptr()); dev_dbg!((*ph).dev, "Agent %d: %s\n", id, name.as_ptr()); } ret = 0; ret
}

static SCMI_BASE: ScmiProtocol = ScmiProtocol { id: SCMI_PROTOCOL_BASE, owner: core::ptr::null(), instance_init: scmi_base_protocol_init, ops: core::ptr::null(), events: &BASE_PROTOCOL_EVENTS, supported_version: SCMI_PROTOCOL_SUPPORTED_VERSION };
// DEFINE_SCMI_PROTOCOL_REGISTER_UNREGISTER(base, scmi_base)

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
