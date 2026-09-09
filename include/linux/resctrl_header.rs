/* SPDX-License-Identifier: GPL-2.0 */

/* Dependencies supplied by the surrounding kernel translation. */

pub const RESCTRL_RESERVED_CLOSID: u32 = 0;
pub const RESCTRL_RESERVED_RMID: u32 = 0;
pub const RESCTRL_PICK_ANY_CPU: i32 = -1;

#[cfg(CONFIG_PROC_CPU_RESCTRL)]
extern "C" {
    pub fn proc_resctrl_show(
        m: *mut seq_file,
        ns: *mut pid_namespace,
        pid: *mut pid,
        tsk: *mut task_struct,
    ) -> i32;
}

pub const MBA_MAX_MBPS: u32 = u32::MAX;

#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum resctrl_res_level {
    RDT_RESOURCE_L3,
    RDT_RESOURCE_L2,
    RDT_RESOURCE_MBA,
    RDT_RESOURCE_SMBA,
    RDT_RESOURCE_PERF_PKG,
}

pub const RDT_RESOURCE_LAST: resctrl_res_level = resctrl_res_level::RDT_RESOURCE_PERF_PKG;
pub const RDT_NUM_RESOURCES: usize = resctrl_res_level::RDT_RESOURCE_PERF_PKG as usize + 1;

#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum resctrl_conf_type {
    CDP_NONE,
    CDP_CODE,
    CDP_DATA,
}

pub const CDP_LAST: resctrl_conf_type = resctrl_conf_type::CDP_DATA;
pub const CDP_NUM_TYPES: usize = resctrl_conf_type::CDP_DATA as usize + 1;

#[repr(C)]
pub struct pseudo_lock_region {
    pub s: *mut resctrl_schema,
    pub closid: u32,
    pub d: *mut rdt_ctrl_domain,
    pub cbm: u32,
    pub lock_thread_wq: wait_queue_head_t,
    pub thread_done: i32,
    pub cpu: i32,
    pub line_size: u32,
    pub size: u32,
    pub kmem: *mut core::ffi::c_void,
    pub minor: u32,
    pub debugfs_dir: *mut dentry,
    pub pm_reqs: list_head,
}

#[repr(C)]
pub struct resctrl_staged_config {
    pub new_ctrl: u32,
    pub have_new_ctrl: bool,
}

#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum resctrl_domain_type {
    RESCTRL_CTRL_DOMAIN,
    RESCTRL_MON_DOMAIN,
}

#[repr(C)]
pub struct rdt_domain_hdr {
    pub list: list_head,
    pub id: i32,
    pub type_: resctrl_domain_type,
    pub rid: resctrl_res_level,
    pub cpu_mask: cpumask,
}

pub unsafe fn domain_header_is_valid(
    hdr: *mut rdt_domain_hdr,
    type_: resctrl_domain_type,
    rid: resctrl_res_level,
) -> bool {
    !WARN_ON_ONCE((*hdr).type_ != type_ || (*hdr).rid != rid)
}

#[repr(C)]
pub struct rdt_ctrl_domain {
    pub hdr: rdt_domain_hdr,
    pub plr: *mut pseudo_lock_region,
    pub staged_config: [resctrl_staged_config; CDP_NUM_TYPES],
    pub mbps_val: *mut u32,
}

#[repr(C)]
pub struct mbm_cntr_cfg {
    pub evtid: resctrl_event_id,
    pub rdtgrp: *mut rdtgroup,
}

#[repr(C)]
pub struct rdt_l3_mon_domain {
    pub hdr: rdt_domain_hdr,
    pub ci_id: u32,
    pub rmid_busy_llc: *mut u64,
    pub mbm_states: [*mut mbm_state; QOS_NUM_L3_MBM_EVENTS],
    pub mbm_over: delayed_work,
    pub cqm_limbo: delayed_work,
    pub mbm_work_cpu: i32,
    pub cqm_work_cpu: i32,
    pub cntr_cfg: *mut mbm_cntr_cfg,
}

#[repr(C)]
pub struct resctrl_cache {
    pub cbm_len: u32,
    pub min_cbm_bits: u32,
    pub shareable_bits: u32,
    pub arch_has_sparse_bitmasks: bool,
    pub arch_has_per_cpu_cfg: bool,
    pub io_alloc_capable: bool,
}

#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum membw_throttle_mode {
    THREAD_THROTTLE_UNDEFINED = 0,
    THREAD_THROTTLE_MAX,
    THREAD_THROTTLE_PER_THREAD,
}

#[repr(C)]
pub struct resctrl_membw {
    pub min_bw: u32,
    pub max_bw: u32,
    pub bw_gran: u32,
    pub delay_linear: u32,
    pub arch_needs_linear: bool,
    pub throttle_mode: membw_throttle_mode,
    pub mba_sc: bool,
    pub mb_map: *mut u32,
}

#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum resctrl_scope {
    RESCTRL_L2_CACHE = 2,
    RESCTRL_L3_CACHE = 3,
    RESCTRL_L3_NODE,
    RESCTRL_PACKAGE,
}

#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum resctrl_schema_fmt {
    RESCTRL_SCHEMA_BITMAP,
    RESCTRL_SCHEMA_RANGE,
}

#[repr(C)]
pub struct resctrl_mon {
    pub num_rmid: u32,
    pub mbm_cfg_mask: u32,
    pub num_mbm_cntrs: i32,
    pub mbm_cntr_assignable: bool,
    pub mbm_assign_on_mkdir: bool,
    pub mbm_cntr_configurable: bool,
    pub mbm_cntr_assign_fixed: bool,
}

#[repr(C)]
pub struct rdt_resource {
    pub rid: resctrl_res_level,
    pub alloc_capable: bool,
    pub mon_capable: bool,
    pub ctrl_scope: resctrl_scope,
    pub mon_scope: resctrl_scope,
    pub cache: resctrl_cache,
    pub membw: resctrl_membw,
    pub mon: resctrl_mon,
    pub ctrl_domains: list_head,
    pub mon_domains: list_head,
    pub name: *mut core::ffi::c_char,
    pub schema_fmt: resctrl_schema_fmt,
    pub cdp_capable: bool,
}

extern "C" {
    pub fn resctrl_arch_get_resource(l: resctrl_res_level) -> *mut rdt_resource;
}

#[repr(C)]
pub struct resctrl_schema {
    pub list: list_head,
    pub name: [core::ffi::c_char; 8],
    pub fmt_str: *const core::ffi::c_char,
    pub conf_type: resctrl_conf_type,
    pub res: *mut rdt_resource,
    pub num_closid: u32,
}

#[repr(C)]
pub struct resctrl_cpu_defaults {
    pub closid: u32,
    pub rmid: u32,
}

#[repr(C)]
pub struct resctrl_mon_config_info {
    pub r: *mut rdt_resource,
    pub d: *mut rdt_l3_mon_domain,
    pub evtid: u32,
    pub mon_config: u32,
}

pub unsafe fn resctrl_get_default_ctrl(r: *mut rdt_resource) -> u32 {
    match (*r).schema_fmt {
        resctrl_schema_fmt::RESCTRL_SCHEMA_BITMAP => {
            (1u32 << (*r).cache.cbm_len).wrapping_sub(1)
        }
        resctrl_schema_fmt::RESCTRL_SCHEMA_RANGE => (*r).membw.max_bw,
    }
}

extern "C" {
    pub fn resctrl_arch_sync_cpu_closid_rmid(info: *mut core::ffi::c_void);
    pub fn resctrl_arch_get_num_closid(r: *mut rdt_resource) -> u32;
    pub fn resctrl_arch_system_num_rmid_idx() -> u32;
    pub fn resctrl_arch_update_domains(r: *mut rdt_resource, closid: u32) -> i32;
    pub fn resctrl_enable_mon_event(eventid: resctrl_event_id, any_cpu: bool, binary_bits: u32, arch_priv: *mut core::ffi::c_void) -> bool;
    pub fn resctrl_is_mon_event_enabled(eventid: resctrl_event_id) -> bool;
    pub fn resctrl_arch_is_evt_configurable(evt: resctrl_event_id) -> bool;
    pub fn resctrl_get_mon_evt_cfg(eventid: resctrl_event_id) -> u32;
    pub fn resctrl_arch_mon_event_config_write(config_info: *mut core::ffi::c_void);
    pub fn resctrl_arch_mon_event_config_read(config_info: *mut core::ffi::c_void);
    pub fn resctrl_arch_get_cdp_enabled(l: resctrl_res_level) -> bool;
    pub fn resctrl_arch_set_cdp_enabled(l: resctrl_res_level, enable: bool) -> i32;
    pub fn resctrl_arch_mbm_cntr_assign_enabled(r: *mut rdt_resource) -> bool;
    pub fn resctrl_arch_mbm_cntr_assign_set(r: *mut rdt_resource, enable: bool) -> i32;
    pub fn resctrl_arch_update_one(r: *mut rdt_resource, d: *mut rdt_ctrl_domain, closid: u32, t: resctrl_conf_type, cfg_val: u32) -> i32;
    pub fn resctrl_arch_get_config(r: *mut rdt_resource, d: *mut rdt_ctrl_domain, closid: u32, type_: resctrl_conf_type) -> u32;
    pub fn resctrl_online_ctrl_domain(r: *mut rdt_resource, d: *mut rdt_ctrl_domain) -> i32;
    pub fn resctrl_online_mon_domain(r: *mut rdt_resource, hdr: *mut rdt_domain_hdr) -> i32;
    pub fn resctrl_offline_ctrl_domain(r: *mut rdt_resource, d: *mut rdt_ctrl_domain);
    pub fn resctrl_offline_mon_domain(r: *mut rdt_resource, hdr: *mut rdt_domain_hdr);
}

pub unsafe fn resctrl_is_mbm_event(eventid: resctrl_event_id) -> bool {
    eventid >= QOS_L3_MBM_TOTAL_EVENT_ID && eventid <= QOS_L3_MBM_LOCAL_EVENT_ID
}

pub unsafe fn resctrl_get_config_index(closid: u32, type_: resctrl_conf_type) -> u32 {
    match type_ {
        resctrl_conf_type::CDP_CODE => closid.wrapping_mul(2).wrapping_add(1),
        resctrl_conf_type::CDP_DATA => closid.wrapping_mul(2),
        resctrl_conf_type::CDP_NONE => closid,
    }
}

/* C iteration macros translated as equivalent loop forms at call sites:
 * for_each_rdt_resource, for_each_capable_rdt_resource,
 * for_each_alloc_capable_rdt_resource, for_each_mon_capable_rdt_resource,
 * for_each_mbm_event_id, and for_each_mbm_idx.
 */

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
