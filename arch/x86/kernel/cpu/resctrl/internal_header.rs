/* SPDX-License-Identifier: GPL-2.0 */

// Dependency supplied by linux/resctrl.h.

pub const L3_QOS_CDP_ENABLE: u64 = 0x01;
pub const L2_QOS_CDP_ENABLE: u64 = 0x01;
pub const MBM_CNTR_WIDTH_BASE: u32 = 24;
pub const MBA_IS_LINEAR: u32 = 0x4;
pub const MBM_CNTR_WIDTH_OFFSET_AMD: u32 = 20;
pub const MBM_CNTR_WIDTH_OFFSET_HYGON: u32 = 8;
pub const RMID_VAL_ERROR: u64 = 1u64 << 63;
pub const RMID_VAL_UNAVAIL: u64 = 1u64 << 62;
pub const MBM_CNTR_WIDTH_OFFSET_MAX: u32 = 62 - MBM_CNTR_WIDTH_BASE;

#[repr(C)]
pub struct arch_mbm_state {
    pub chunks: u64,
    pub prev_msr: u64,
}

pub const ABMC_ENABLE_BIT: u32 = 0;
pub const ABMC_EXTENDED_EVT_ID: u32 = 1u32 << 31;
pub const ABMC_EVT_ID: u32 = 1;
pub const SDCIAE_ENABLE_BIT: u32 = 1;

#[repr(C)]
pub struct rdt_hw_ctrl_domain {
    pub d_resctrl: rdt_ctrl_domain,
    pub ctrl_val: *mut u32,
}

#[repr(C)]
pub struct rdt_hw_l3_mon_domain {
    pub d_resctrl: rdt_l3_mon_domain,
    pub arch_mbm_states: [*mut arch_mbm_state; QOS_NUM_L3_MBM_EVENTS as usize],
}

#[inline]
pub unsafe fn resctrl_to_arch_ctrl_dom(r: *mut rdt_ctrl_domain) -> *mut rdt_hw_ctrl_domain {
    container_of!(r, rdt_hw_ctrl_domain, d_resctrl)
}

#[inline]
pub unsafe fn resctrl_to_arch_mon_dom(r: *mut rdt_l3_mon_domain) -> *mut rdt_hw_l3_mon_domain {
    container_of!(r, rdt_hw_l3_mon_domain, d_resctrl)
}

#[repr(C)]
pub struct rdt_perf_pkg_mon_domain {
    pub hdr: rdt_domain_hdr,
}

#[repr(C)]
pub struct msr_param {
    pub res: *mut rdt_resource,
    pub dom: *mut rdt_ctrl_domain,
    pub low: u32,
    pub high: u32,
}

#[repr(C)]
pub struct rdt_hw_resource {
    pub r_resctrl: rdt_resource,
    pub num_closid: u32,
    pub msr_base: libc::c_uint,
    pub msr_update: Option<unsafe extern "C" fn(m: *mut msr_param)>,
    pub mon_scale: libc::c_uint,
    pub mbm_width: libc::c_uint,
    pub cdp_enabled: bool,
    pub mbm_cntr_assign_enabled: bool,
    pub sdciae_enabled: bool,
}

#[inline]
pub unsafe fn resctrl_to_arch_res(r: *mut rdt_resource) -> *mut rdt_hw_resource {
    container_of!(r, rdt_hw_resource, r_resctrl)
}

unsafe extern "C" {
    pub static mut rdt_resources_all: rdt_hw_resource;
    pub fn arch_mon_domain_online(r: *mut rdt_resource, d: *mut rdt_l3_mon_domain);
}

#[repr(C)]
pub union cpuid_0x10_1_eax {
    pub split: cpuid_0x10_1_eax_split,
    pub full: libc::c_uint,
}
#[repr(C)]
pub struct cpuid_0x10_1_eax_split { pub cbm_len: libc::c_uint }

#[repr(C)]
pub union cpuid_0x10_3_eax {
    pub split: cpuid_0x10_3_eax_split,
    pub full: libc::c_uint,
}
#[repr(C)]
pub struct cpuid_0x10_3_eax_split { pub max_delay: libc::c_uint }

#[repr(C)]
pub union cpuid_0x10_x_ecx {
    pub split: cpuid_0x10_x_ecx_split,
    pub full: libc::c_uint,
}
#[repr(C)]
pub struct cpuid_0x10_x_ecx_split {
    pub reserved: libc::c_uint,
    pub noncont: libc::c_uint,
}

#[repr(C)]
pub union cpuid_0x10_x_edx {
    pub split: cpuid_0x10_x_edx_split,
    pub full: libc::c_uint,
}
#[repr(C)]
pub struct cpuid_0x10_x_edx_split { pub cos_max: libc::c_uint }

#[repr(C)]
pub union l3_qos_abmc_cfg {
    pub split: l3_qos_abmc_cfg_split,
    pub full: libc::c_ulong,
}
#[repr(C)]
pub struct l3_qos_abmc_cfg_split {
    pub bw_type: libc::c_ulong,
    pub bw_src: libc::c_ulong,
    pub reserved1: libc::c_ulong,
    pub is_clos: libc::c_ulong,
    pub cntr_id: libc::c_ulong,
    pub reserved: libc::c_ulong,
    pub cntr_en: libc::c_ulong,
    pub cfg_en: libc::c_ulong,
}

unsafe extern "C" {
    pub fn rdt_ctrl_update(arg: *mut libc::c_void);
    pub fn rdt_get_l3_mon_config(r: *mut rdt_resource) -> libc::c_int;
    pub fn rdt_cpu_has(flag: libc::c_int) -> bool;
    pub fn intel_rdt_mbm_apply_quirk();
    pub fn rdt_domain_reconfigure_cdp(r: *mut rdt_resource);
    pub fn resctrl_arch_mbm_cntr_assign_set_one(r: *mut rdt_resource);
}

#[cfg(CONFIG_X86_CPU_RESCTRL_INTEL_AET)]
unsafe extern "C" {
    pub fn intel_aet_get_events() -> bool;
    pub fn intel_aet_exit();
    pub fn intel_aet_read_event(domid: libc::c_int, rmid: u32, arch_priv: *mut libc::c_void, val: *mut u64) -> libc::c_int;
    pub fn intel_aet_mon_domain_setup(cpu: libc::c_int, id: libc::c_int, r: *mut rdt_resource, add_pos: *mut list_head);
    pub fn intel_handle_aet_option(force_off: bool, tok: *mut libc::c_char) -> bool;
}

#[cfg(not(CONFIG_X86_CPU_RESCTRL_INTEL_AET))]
#[inline]
pub fn intel_aet_get_events() -> bool { false }
#[cfg(not(CONFIG_X86_CPU_RESCTRL_INTEL_AET))]
#[inline]
pub fn intel_aet_exit() {}
#[cfg(not(CONFIG_X86_CPU_RESCTRL_INTEL_AET))]
#[inline]
pub fn intel_aet_read_event(_domid: libc::c_int, _rmid: u32, _arch_priv: *mut libc::c_void, _val: *mut u64) -> libc::c_int { -22 }
#[cfg(not(CONFIG_X86_CPU_RESCTRL_INTEL_AET))]
#[inline]
pub fn intel_aet_mon_domain_setup(_cpu: libc::c_int, _id: libc::c_int, _r: *mut rdt_resource, _add_pos: *mut list_head) {}
#[cfg(not(CONFIG_X86_CPU_RESCTRL_INTEL_AET))]
#[inline]
pub fn intel_handle_aet_option(_force_off: bool, _tok: *mut libc::c_char) -> bool { false }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
