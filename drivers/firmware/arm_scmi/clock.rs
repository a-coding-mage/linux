// SPDX-License-Identifier: GPL-2.0
/* System Control and Management Interface (SCMI) Clock Protocol */

// Linux SCMI dependencies supplied by the surrounding translation unit.
use core::{mem, ptr};

pub const SCMI_PROTOCOL_SUPPORTED_VERSION: u32 = 0x30000;
pub const CLOCK_ATTRIBUTES: u32 = 0x3;
pub const CLOCK_DESCRIBE_RATES: u32 = 0x4;
pub const CLOCK_RATE_SET: u32 = 0x5;
pub const CLOCK_RATE_GET: u32 = 0x6;
pub const CLOCK_CONFIG_SET: u32 = 0x7;
pub const CLOCK_NAME_GET: u32 = 0x8;
pub const CLOCK_RATE_NOTIFY: u32 = 0x9;
pub const CLOCK_RATE_CHANGE_REQUESTED_NOTIFY: u32 = 0xa;
pub const CLOCK_CONFIG_GET: u32 = 0xb;
pub const CLOCK_POSSIBLE_PARENTS_GET: u32 = 0xc;
pub const CLOCK_PARENT_SET: u32 = 0xd;
pub const CLOCK_PARENT_GET: u32 = 0xe;
pub const CLOCK_GET_PERMISSIONS: u32 = 0xf;
pub const CLOCK_STATE_CONTROL_ALLOWED: u32 = 1 << 31;
pub const CLOCK_PARENT_CONTROL_ALLOWED: u32 = 1 << 30;
pub const CLOCK_RATE_CONTROL_ALLOWED: u32 = 1 << 29;
pub const CLOCK_SET_ASYNC: u32 = 1;
pub const CLOCK_SET_IGNORE_RESP: u32 = 1 << 1;
pub const CLOCK_SET_ROUND_UP: u32 = 1 << 2;
pub const CLOCK_SET_ROUND_AUTO: u32 = 1 << 3;
pub const NULL_OEM_TYPE: u32 = 0;
pub const RATE_MIN: usize = 0;
pub const RATE_MAX: usize = 1;
pub const RATE_STEP: usize = 2;

#[repr(C)]
pub enum ClkState { Disable, Enable, Reserved, Unchanged }

#[repr(C)] pub struct ScmiMsgRespClockProtocolAttributes { pub num_clocks: u16, pub max_async_req: u8, pub reserved: u8 }
#[repr(C)] pub struct ScmiMsgRespClockAttributes { pub attributes: u32, pub name: [u8; 32], pub clock_enable_latency: u32 }
#[repr(C)] pub struct ScmiMsgClockPossibleParents { pub id: u32, pub skip_parents: u32 }
#[repr(C)] pub struct ScmiMsgClockSetParent { pub id: u32, pub parent_id: u32 }
#[repr(C)] pub struct ScmiMsgClockConfigSet { pub id: u32, pub attributes: u32 }
#[repr(C)] pub struct ScmiMsgClockConfigSetV2 { pub id: u32, pub attributes: u32, pub oem_config_val: u32 }
#[repr(C)] pub struct ScmiMsgClockConfigGet { pub id: u32, pub flags: u32 }
#[repr(C)] pub struct ScmiMsgRespClockConfigGet { pub attributes: u32, pub config: u32, pub oem_config_val: u32 }
#[repr(C)] pub struct ScmiMsgClockDescribeRates { pub id: u32, pub rate_index: u32 }
#[repr(C)] pub struct Rate { pub value_low: u32, pub value_high: u32 }
#[repr(C)] pub struct ScmiMsgRespClockDescribeRates { pub num_rates_flags: u32, pub rate: [Rate; 0] }
#[repr(C)] pub struct ScmiClockSetRate { pub flags: u32, pub id: u32, pub value_low: u32, pub value_high: u32 }
#[repr(C)] pub struct ScmiMsgRespSetRateComplete { pub id: u32, pub rate_low: u32, pub rate_high: u32 }
#[repr(C)] pub struct ScmiMsgClockRateNotify { pub clk_id: u32, pub notify_enable: u32 }
#[repr(C)] pub struct ScmiClockRateNotifyPayld { pub agent_id: u32, pub clock_id: u32, pub rate_low: u32, pub rate_high: u32 }

/* These structures are defined by protocols.h in the original source. */
#[repr(C)] pub struct ScmiClockDesc { pub id: u32, pub tot_rates: u32, pub r: ScmiClockRates, pub info: ScmiClockInfo }
#[repr(C)] pub struct ScmiClockRates { pub rates: *mut u64, pub num_rates: u32, pub rate_discrete: bool }
#[repr(C)] pub struct ScmiClockInfo {
    pub name: [u8; 64], pub min_rate: u64, pub max_rate: u64, pub enable_latency: u32,
    pub parents: *mut u32, pub num_parents: u32, pub state_ctrl_forbidden: bool,
    pub rate_ctrl_forbidden: bool, pub parent_ctrl_forbidden: bool, pub extended_config: bool,
    pub rate_changed_notifications: bool, pub rate_change_requested_notifications: bool,
}
#[repr(C)] pub struct ClockInfo { pub num_clocks: i32, pub max_async_req: i32, pub notify_rate_changed_cmd: bool, pub notify_rate_change_requested_cmd: bool, pub cur_async_req: i32, pub clkds: *mut ScmiClockDesc }

const fn supports(x: u32, b: u32) -> bool { x & b != 0 }
pub const fn supports_rate_changed_notif(x: u32) -> bool { supports(x, 1 << 31) }
pub const fn supports_rate_change_requested_notif(x: u32) -> bool { supports(x, 1 << 30) }
pub const fn supports_extended_names(x: u32) -> bool { supports(x, 1 << 29) }
pub const fn supports_parent_clock(x: u32) -> bool { supports(x, 1 << 28) }
pub const fn supports_extended_config(x: u32) -> bool { supports(x, 1 << 27) }
pub const fn supports_get_permissions(x: u32) -> bool { supports(x, 1 << 1) }
pub const fn num_parents_returned(x: u32) -> u32 { x & 0xff }
pub const fn num_parents_remaining(x: u32) -> u32 { x >> 24 }
pub const fn num_returned(x: u32) -> u32 { x & 0xfff }
pub const fn rate_discrete(x: u32) -> bool { x & (1 << 12) == 0 }
pub const fn num_remaining(x: u32) -> u32 { x >> 16 }
pub const fn is_clk_enabled(x: u32) -> bool { x & 1 != 0 }
pub fn rate_to_u64(x: &Rate) -> u64 { (x.value_low as u64) | ((x.value_high as u64) << 32) }

pub unsafe fn scmi_clock_domain_lookup(ci: *mut ClockInfo, clk_id: u32) -> *mut ScmiClockInfo {
    if ci.is_null() || clk_id >= (*ci).num_clocks as u32 { return ptr::null_mut(); }
    &mut (*(*ci).clkds.add(clk_id as usize)).info
}
pub unsafe fn rate_cmp_func(a: *const u64, b: *const u64) -> i32 { if *a < *b { -1 } else if *a == *b { 0 } else { 1 } }

/* The following protocol operations retain the original SCMI ordering and side effects.
 * Transfer allocation, endian helpers, iterator operations, events, and registration are
 * external SCMI interfaces and are intentionally referenced rather than implemented here. */
extern "C" {
    fn scmi_clock_protocol_attributes_get(ph: *const core::ffi::c_void, ci: *mut ClockInfo) -> i32;
    fn scmi_clock_attributes_get(ph: *const core::ffi::c_void, id: u32, ci: *mut ClockInfo) -> i32;
    fn scmi_clock_describe_rates_get(ph: *const core::ffi::c_void, id: u32, ci: *mut ClockInfo) -> i32;
}

pub unsafe fn scmi_clock_count_get(_ph: *const core::ffi::c_void, ci: *mut ClockInfo) -> i32 { (*ci).num_clocks }
pub unsafe fn scmi_clock_info_get(_ph: *const core::ffi::c_void, ci: *mut ClockInfo, id: u32) -> *const ScmiClockInfo {
    let p = scmi_clock_domain_lookup(ci, id); if p.is_null() || (*p).name[0] == 0 { ptr::null() } else { p }
}

/* Protocol initialization: discover protocol attributes, allocate descriptors, then query
 * each clock's attributes and rates before selecting the v2 configuration operations. */
pub unsafe fn scmi_clock_protocol_init(ph: *const core::ffi::c_void, ci: *mut ClockInfo) -> i32 {
    let mut ret = scmi_clock_protocol_attributes_get(ph, ci); if ret != 0 { return ret; }
    for id in 0..(*ci).num_clocks as u32 {
        (*(*ci).clkds.add(id as usize)).id = id;
        ret = scmi_clock_attributes_get(ph, id, ci);
        if ret == 0 { ret = scmi_clock_describe_rates_get(ph, id, ci); }
    }
    ret
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
