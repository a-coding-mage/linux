// SPDX-License-Identifier: GPL-2.0
/* System Control and Management Interface (SCMI) Performance Protocol */

// Kernel includes and symbols are supplied by the surrounding Rust translation.

pub const SCMI_PROTOCOL_SUPPORTED_VERSION: u32 = 0x40000;
pub const MAX_OPPS: usize = 64;

#[repr(u32)]
pub enum ScmiPerformanceProtocolCmd { PerfDomainAttributes = 0x3, PerfDescribeLevels = 0x4, PerfLimitsSet = 0x5, PerfLimitsGet = 0x6, PerfLevelSet = 0x7, PerfLevelGet = 0x8, PerfNotifyLimits = 0x9, PerfNotifyLevel = 0xa, PerfDescribeFastchannel = 0xb, PerfDomainNameGet = 0xc }
pub const PERF_FC_LEVEL: usize = 0;
pub const PERF_FC_LIMIT: usize = 1;
pub const PERF_FC_MAX: usize = 2;

#[repr(C)] pub struct ScmiOpp { pub perf: u32, pub power: u32, pub trans_latency_us: u32, pub indicative_freq: u32, pub level_index: u32, pub hash: HlistNode }
#[repr(C)] pub struct ScmiMsgRespPerfAttributes { pub num_domains: Le16, pub flags: Le16, pub stats_addr_low: Le32, pub stats_addr_high: Le32, pub stats_size: Le32 }
#[repr(C)] pub struct ScmiMsgRespPerfDomainAttributes { pub flags: Le32, pub rate_limit_us: Le32, pub sustained_freq_khz: Le32, pub sustained_perf_level: Le32, pub name: [u8; SCMI_SHORT_NAME_MAX_SIZE] }
#[repr(C)] pub struct ScmiMsgPerfDescribeLevels { pub domain: Le32, pub level_index: Le32 }
#[repr(C)] pub struct ScmiPerfSetLimits { pub domain: Le32, pub max_level: Le32, pub min_level: Le32 }
#[repr(C)] pub struct ScmiPerfGetLimits { pub max_level: Le32, pub min_level: Le32 }
#[repr(C)] pub struct ScmiPerfSetLevel { pub domain: Le32, pub level: Le32 }
#[repr(C)] pub struct ScmiPerfNotifyLevelOrLimits { pub domain: Le32, pub notify_enable: Le32 }
#[repr(C)] pub struct ScmiPerfLimitsNotifyPayld { pub agent_id: Le32, pub domain_id: Le32, pub range_max: Le32, pub range_min: Le32 }
#[repr(C)] pub struct ScmiPerfLevelNotifyPayld { pub agent_id: Le32, pub domain_id: Le32, pub performance_level: Le32 }
#[repr(C)] pub struct ScmiMsgRespPerfDescribeLevels { pub num_returned: Le16, pub num_remaining: Le16, pub opp: [ScmiOppV3; 0] }
#[repr(C)] pub struct ScmiOppV3 { pub perf_val: Le32, pub power: Le32, pub transition_latency_us: Le16, pub reserved: Le16 }
#[repr(C)] pub struct ScmiMsgRespPerfDescribeLevelsV4 { pub num_returned: Le16, pub num_remaining: Le16, pub opp: [ScmiOppV4; 0] }
#[repr(C)] pub struct ScmiOppV4 { pub perf_val: Le32, pub power: Le32, pub transition_latency_us: Le16, pub reserved: Le16, pub indicative_freq: Le32, pub level_index: Le32 }

#[repr(C)] pub struct PerfDomInfo { pub id: u32, pub set_limits: bool, pub perf_limit_notify: bool, pub perf_level_notify: bool, pub perf_fastchannels: bool, pub level_indexing_mode: bool, pub opp_count: u32, pub rate_limit_us: u32, pub sustained_freq_khz: u32, pub sustained_perf_level: u32, pub mult_factor: usize, pub info: ScmiPerfDomainInfo, pub opp: [ScmiOpp; MAX_OPPS], pub fc_info: *mut ScmiFcInfo, pub opps_by_idx: Xarray, pub opps_by_lvl: Xarray, pub opps_by_freq: Htable }
#[repr(C)] pub struct ScmiPerfInfo { pub num_domains: u16, pub power_scale: ScmiPowerScale, pub stats_addr: u64, pub stats_size: u32, pub notify_lvl_cmd: bool, pub notify_lim_cmd: bool, pub dom_info: *mut PerfDomInfo }

static EVT_2_CMD: [ScmiPerformanceProtocolCmd; 2] = [ScmiPerformanceProtocolCmd::PerfNotifyLimits, ScmiPerformanceProtocolCmd::PerfNotifyLevel];

unsafe fn scmi_perf_attributes_get(ph: *const ScmiProtocolHandle, pi: *mut ScmiPerfInfo) -> i32 { let mut t: *mut ScmiXfer = core::ptr::null_mut(); let ret = ((*(*ph).xops).xfer_get_init)(ph, PROTOCOL_ATTRIBUTES, 0, core::mem::size_of::<ScmiMsgRespPerfAttributes>(), &mut t); if ret != 0 { return ret; } let attr = (*t).rx.buf as *mut ScmiMsgRespPerfAttributes; let ret = ((*(*ph).xops).do_xfer)(ph, t); if ret == 0 { let flags = le16_to_cpu((*attr).flags); (*pi).num_domains = le16_to_cpu((*attr).num_domains); if flags & BIT(0) != 0 { (*pi).power_scale = ScmiPowerScale::Milliwatts; } if protocol_rev_major((*ph).version) >= 3 && flags & BIT(1) != 0 { (*pi).power_scale = ScmiPowerScale::Microwatts; } (*pi).stats_addr = le32_to_cpu((*attr).stats_addr_low) as u64 | ((le32_to_cpu((*attr).stats_addr_high) as u64) << 32); (*pi).stats_size = le32_to_cpu((*attr).stats_size); } ((*(*ph).xops).xfer_put)(ph, t); ret }

unsafe fn scmi_perf_domain_lookup(ph: *const ScmiProtocolHandle, domain: u32) -> *mut PerfDomInfo { let pi = ((*ph).get_priv)(ph) as *mut ScmiPerfInfo; if domain >= (*pi).num_domains as u32 { return ERR_PTR(-EINVAL); } (*pi).dom_info.add(domain as usize) }
unsafe fn scmi_perf_num_domains_get(ph: *const ScmiProtocolHandle) -> i32 { (*((*ph).get_priv)(ph) as *mut ScmiPerfInfo)).num_domains as i32 }
unsafe fn scmi_perf_info_get(ph: *const ScmiProtocolHandle, domain: u32) -> *const ScmiPerfDomainInfo { let d = scmi_perf_domain_lookup(ph, domain); if IS_ERR(d) { return ERR_PTR(-EINVAL); } &(*d).info }

unsafe fn scmi_perf_msg_limits_set(ph: *const ScmiProtocolHandle, domain: u32, max_perf: u32, min_perf: u32) -> i32 { let mut t=core::ptr::null_mut(); let ret=((*(*ph).xops).xfer_get_init)(ph, ScmiPerformanceProtocolCmd::PerfLimitsSet as u32, core::mem::size_of::<ScmiPerfSetLimits>(),0,&mut t); if ret!=0{return ret;} let l=(*t).tx.buf as *mut ScmiPerfSetLimits; (*l).domain=cpu_to_le32(domain);(*l).max_level=cpu_to_le32(max_perf);(*l).min_level=cpu_to_le32(min_perf);let r=((*(*ph).xops).do_xfer)(ph,t);((*(*ph).xops).xfer_put)(ph,t);r }
unsafe fn scmi_perf_limits_set(ph:*const ScmiProtocolHandle,domain:u32,max_perf:u32,min_perf:u32)->i32{let d=scmi_perf_domain_lookup(ph,domain);if IS_ERR(d){return PTR_ERR(d);}if !(*d).set_limits{return -EOPNOTSUPP;}scmi_perf_msg_limits_set(ph,(*d).id,max_perf,min_perf)}
unsafe fn scmi_perf_level_set(ph:*const ScmiProtocolHandle,domain:u32,level:u32,_poll:bool)->i32{let d=scmi_perf_domain_lookup(ph,domain);if IS_ERR(d){return PTR_ERR(d);}if !(*d).info.set_perf{return -EOPNOTSUPP;}let mut t=core::ptr::null_mut();let r=((*(*ph).xops).xfer_get_init)(ph,ScmiPerformanceProtocolCmd::PerfLevelSet as u32,core::mem::size_of::<ScmiPerfSetLevel>(),0,&mut t);if r!=0{return r;}let l=(*t).tx.buf as *mut ScmiPerfSetLevel;(*l).domain=cpu_to_le32(domain);(*l).level=cpu_to_le32(level);let r=((*(*ph).xops).do_xfer)(ph,t);((*(*ph).xops).xfer_put)(ph,t);r}
unsafe fn scmi_perf_level_get(ph:*const ScmiProtocolHandle,domain:u32,level:*mut u32,_poll:bool)->i32{let d=scmi_perf_domain_lookup(ph,domain);if IS_ERR(d){return PTR_ERR(d);}let mut t=core::ptr::null_mut();let r=((*(*ph).xops).xfer_get_init)(ph,ScmiPerformanceProtocolCmd::PerfLevelGet as u32,4,4,&mut t);if r!=0{return r;}let p=(*t).tx.buf as *mut u32;*p=cpu_to_le32(domain);let r=((*(*ph).xops).do_xfer)(ph,t);if r==0{*level=get_unaligned_le32((*t).rx.buf);}((*(*ph).xops).xfer_put)(ph,t);r}

// Remaining protocol callbacks and registration retain the same externally supplied
// SCMI operation tables and are declared for linkage with the surrounding translation.
extern "C" { pub static scmi_perf: ScmiProtocol; }


// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
