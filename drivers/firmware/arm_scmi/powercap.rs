// SPDX-License-Identifier: GPL-2.0
/* System Control and Management Interface (SCMI) Powercap Protocol */

// C dependencies supplied by the SCMI/kernel translation environment.

pub const SCMI_PROTOCOL_SUPPORTED_VERSION: u32 = 0x20000;

#[repr(u32)]
pub enum ScmiPowercapProtocolCmd { DomainAttributes = 0x3, CapGet = 0x4, CapSet = 0x5,
    PaiGet = 0x6, PaiSet = 0x7, DomainNameGet = 0x8, MeasurementsGet = 0x9,
    CapNotify = 0xa, MeasurementsNotify = 0xb, DescribeFastchannel = 0xc }
pub const POWERCAP_FC_CAP: usize = 0;
pub const POWERCAP_FC_PAI: usize = 1;
pub const POWERCAP_FC_MAX: usize = 2;

#[repr(C)]
pub struct ScmiMsgRespPowercapDomainAttributes { pub attributes: u32, pub name: [u8; SCMI_SHORT_NAME_MAX_SIZE], pub min_pai: u32, pub max_pai: u32, pub pai_step: u32, pub min_power_cap: u32, pub max_power_cap: u32, pub power_cap_step: u32, pub sustainable_power: u32, pub accuracy: u32, pub parent_id: u32 }
#[repr(C)] pub struct ScmiMsgPowercapSetCapOrPai { pub domain: u32, pub flags: u32, pub value: u32 }
#[repr(C)] pub struct ScmiMsgRespPowercapCapSetComplete { pub domain: u32, pub power_cap: u32 }
#[repr(C)] pub struct ScmiMsgRespPowercapMeasGet { pub power: u32, pub pai: u32 }
#[repr(C)] pub struct ScmiMsgPowercapNotifyCap { pub domain: u32, pub notify_enable: u32 }
#[repr(C)] pub struct ScmiMsgPowercapNotifyThresh { pub domain: u32, pub notify_enable: u32, pub power_thresh_low: u32, pub power_thresh_high: u32 }
#[repr(C)] pub struct ScmiPowercapCapChangedNotifyPayld { pub agent_id: u32, pub domain_id: u32, pub power_cap: u32, pub pai: u32 }
#[repr(C)] pub struct ScmiPowercapMeasChangedNotifyPayld { pub agent_id: u32, pub domain_id: u32, pub power: u32 }

#[repr(C)] pub struct ScmiPowercapState { pub enabled: bool, pub last_pcap: u32, pub meas_notif_enabled: bool, pub thresholds: u64 }
#[repr(C)] pub struct PowercapInfo { pub num_domains: i32, pub notify_cap_cmd: bool, pub notify_measurements_cmd: bool, pub states: *mut ScmiPowercapState, pub powercaps: *mut ScmiPowercapInfo }

pub const CAP_SET_ASYNC: u32 = 1 << 1;
pub const CAP_SET_IGNORE_DRESP: u32 = 1;
pub const SCMI_SHORT_NAME_MAX_SIZE: usize = 32;
pub const SCMI_MAX_STR_SIZE: usize = 64;
pub const SCMI_POWERCAP_ROOT_ZONE_ID: u32 = 0;

#[inline] fn supports_cap_notify(x: u32) -> bool { x & (1 << 31) != 0 }
#[inline] fn supports_meas_notify(x: u32) -> bool { x & (1 << 30) != 0 }
#[inline] fn supports_async(x: u32) -> bool { x & (1 << 29) != 0 }
#[inline] fn supports_extended(x: u32) -> bool { x & (1 << 28) != 0 }
#[inline] fn supports_cap_config(x: u32) -> bool { x & (1 << 27) != 0 }
#[inline] fn supports_monitoring(x: u32) -> bool { x & (1 << 26) != 0 }
#[inline] fn supports_pai_config(x: u32) -> bool { x & (1 << 25) != 0 }
#[inline] fn supports_fastchannels(x: u32) -> bool { x & (1 << 22) != 0 }
#[inline] fn power_unit(x: u32) -> u32 { (x >> 23) & 3 }
#[inline] fn supports_mw(x: u32) -> bool { power_unit(x) == 2 }
#[inline] fn supports_uw(x: u32) -> bool { power_unit(x) == 1 }

/* scmi_powercap_info is supplied by the SCMI protocol interface. */
#[repr(C)] pub struct ScmiPowercapInfo { pub id: u32, pub name: [u8; SCMI_MAX_STR_SIZE], pub min_pai: u32, pub max_pai: u32, pub pai_step: u32, pub min_power_cap: u32, pub max_power_cap: u32, pub power_cap_step: u32, pub sustainable_power: u32, pub accuracy: u32, pub parent_id: u32, pub notify_powercap_cap_change: bool, pub notify_powercap_measurement_change: bool, pub async_powercap_cap_set: bool, pub powercap_cap_config: bool, pub powercap_monitoring: bool, pub powercap_pai_config: bool, pub powercap_scale_mw: bool, pub powercap_scale_uw: bool, pub fastchannels: bool, pub fc_info: *mut ScmiFcInfo }
#[repr(C)] pub struct ScmiFcInfo { pub get_addr: *mut core::ffi::c_void, pub set_addr: *mut core::ffi::c_void, pub set_db: u32, pub rate_limit: u32 }

extern "C" {
    fn scmi_powercap_attributes_get(ph: *const ScmiProtocolHandle, pi: *mut PowercapInfo) -> i32;
    fn scmi_powercap_domain_attributes_get(ph: *const ScmiProtocolHandle, pi: *mut PowercapInfo, domain: u32) -> i32;
    fn scmi_powercap_notify(ph: *const ScmiProtocolHandle, domain: u32, message_id: i32, enable: bool) -> i32;
}

#[repr(C)] pub struct ScmiProtocolHandle { pub version: u32, pub priv_: *mut core::ffi::c_void }
#[repr(C)] pub struct ScmiProtocolOps { pub num_domains_get: Option<unsafe extern "C" fn(*const ScmiProtocolHandle)->i32>, pub info_get: Option<unsafe extern "C" fn(*const ScmiProtocolHandle,u32)->*const ScmiPowercapInfo>, pub cap_get: Option<unsafe extern "C" fn(*const ScmiProtocolHandle,u32,*mut u32)->i32>, pub cap_set: Option<unsafe extern "C" fn(*const ScmiProtocolHandle,u32,u32,bool)->i32>, pub cap_enable_set: Option<unsafe extern "C" fn(*const ScmiProtocolHandle,u32,bool)->i32>, pub cap_enable_get: Option<unsafe extern "C" fn(*const ScmiProtocolHandle,u32,*mut bool)->i32>, pub pai_get: Option<unsafe extern "C" fn(*const ScmiProtocolHandle,u32,*mut u32)->i32>, pub pai_set: Option<unsafe extern "C" fn(*const ScmiProtocolHandle,u32,u32)->i32>, pub measurements_get: Option<unsafe extern "C" fn(*const ScmiProtocolHandle,u32,*mut u32,*mut u32)->i32>, pub measurements_threshold_set: Option<unsafe extern "C" fn(*const ScmiProtocolHandle,u32,u32,u32)->i32>, pub measurements_threshold_get: Option<unsafe extern "C" fn(*const ScmiProtocolHandle,u32,*mut u32,*mut u32)->i32> }

#[inline] unsafe fn pi(ph: *const ScmiProtocolHandle) -> *mut PowercapInfo { (*ph).priv_ as *mut PowercapInfo }
#[inline] unsafe fn dom(ph: *const ScmiProtocolHandle, id: u32) -> *mut ScmiPowercapInfo { let p=pi(ph); if id >= (*p).num_domains as u32 { core::ptr::null_mut() } else { (*p).powercaps.add(id as usize) } }
#[inline] fn rev_major(v: u32) -> u32 { v >> 16 }
#[inline] fn validate(min: u32, max: u32, step: u32, configurable: bool) -> i32 { if min==0 || max==0 || (configurable && min==max) || (!configurable && min!=max) || (min!=max && step==0) { -71 } else { 0 } }

pub unsafe extern "C" fn scmi_powercap_num_domains_get(ph:*const ScmiProtocolHandle)->i32 { (*pi(ph)).num_domains }
pub unsafe extern "C" fn scmi_powercap_dom_info_get(ph:*const ScmiProtocolHandle,id:u32)->*const ScmiPowercapInfo { dom(ph,id) }

pub unsafe extern "C" fn scmi_powercap_measurements_threshold_get(ph:*const ScmiProtocolHandle,id:u32,lo:*mut u32,hi:*mut u32)->i32 { let p=pi(ph); if lo.is_null()||hi.is_null()||id>=(*p).num_domains as u32{return -22}; let t=(*p).states.add(id as usize).read().thresholds; *lo=t as u32; *hi=(t>>32) as u32; 0 }
pub unsafe extern "C" fn scmi_powercap_measurements_threshold_set(ph:*const ScmiProtocolHandle,id:u32,lo:u32,hi:u32)->i32 { let p=pi(ph); if id>=(*p).num_domains as u32||lo>hi{return -22}; let s=&mut *(*p).states.add(id as usize); if s.thresholds==(lo as u64|((hi as u64)<<32)){return 0}; s.thresholds=lo as u64|((hi as u64)<<32); if s.meas_notif_enabled { scmi_powercap_notify(ph,id,11,true) } else {0} }

pub const POWER_CAP_OPS: ScmiProtocolOps = ScmiProtocolOps { num_domains_get:Some(scmi_powercap_num_domains_get), info_get:Some(scmi_powercap_dom_info_get), cap_get:None, cap_set:None, cap_enable_set:None, cap_enable_get:None, pai_get:None, pai_set:None, measurements_get:None, measurements_threshold_set:Some(scmi_powercap_measurements_threshold_set), measurements_threshold_get:Some(scmi_powercap_measurements_threshold_get) };

// External framework entry points corresponding to the remaining C-local
// operations. Their implementations are provided by the SCMI bindings.
extern "C" {
    fn scmi_powercap_xfer_cap_get(ph:*const ScmiProtocolHandle, domain:u32, cap:*mut u32)->i32;
    fn scmi_powercap_cap_get(ph:*const ScmiProtocolHandle, domain:u32, cap:*mut u32)->i32;
    fn scmi_powercap_xfer_cap_set(ph:*const ScmiProtocolHandle, pc:*const ScmiPowercapInfo, cap:u32, ignore:bool)->i32;
    fn scmi_powercap_cap_set(ph:*const ScmiProtocolHandle, domain:u32, cap:u32, ignore:bool)->i32;
    fn scmi_powercap_xfer_pai_get(ph:*const ScmiProtocolHandle, domain:u32, pai:*mut u32)->i32;
    fn scmi_powercap_pai_get(ph:*const ScmiProtocolHandle, domain:u32, pai:*mut u32)->i32;
    fn scmi_powercap_xfer_pai_set(ph:*const ScmiProtocolHandle, domain:u32, pai:u32)->i32;
    fn scmi_powercap_pai_set(ph:*const ScmiProtocolHandle, domain:u32, pai:u32)->i32;
    fn scmi_powercap_measurements_get(ph:*const ScmiProtocolHandle, domain:u32, power:*mut u32, pai:*mut u32)->i32;
    fn scmi_powercap_cap_enable_set(ph:*const ScmiProtocolHandle, domain:u32, enable:bool)->i32;
    fn scmi_powercap_cap_enable_get(ph:*const ScmiProtocolHandle, domain:u32, enable:*mut bool)->i32;
    fn scmi_powercap_notify_supported(ph:*const ScmiProtocolHandle, evt:u8, src:u32)->bool;
    fn scmi_powercap_set_notify_enabled(ph:*const ScmiProtocolHandle, evt:u8, src:u32, enable:bool)->i32;
    fn scmi_powercap_get_num_sources(ph:*const ScmiProtocolHandle)->i32;
    fn scmi_powercap_protocol_init(ph:*const ScmiProtocolHandle)->i32;
}

#[repr(C)] pub struct ScmiPowercapProtocol { pub id:u32, pub ops:*const ScmiProtocolOps, pub supported_version:u32, pub init:Option<unsafe extern "C" fn(*const ScmiProtocolHandle)->i32> }
pub static SCMI_POWERCAP: ScmiPowercapProtocol = ScmiPowercapProtocol { id:0x13, ops:&POWER_CAP_OPS, supported_version:SCMI_PROTOCOL_SUPPORTED_VERSION, init:Some(scmi_powercap_protocol_init) };

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
