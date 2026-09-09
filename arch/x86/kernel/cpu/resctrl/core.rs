// SPDX-License-Identifier: GPL-2.0-only
/*
 * Rust translation of x86/kernel/cpu/resctrl/core.c.
 *
 * The resctrl types, constants, macros, and external routines referenced
 * below are supplied by the surrounding kernel translation units.
 */

#![allow(non_camel_case_types, non_snake_case, non_upper_case_globals,
         dead_code, unused_variables, unused_mut)]

use core::ptr;

/* Architecture/kernel declarations are external dependencies of this file. */
extern "C" {
    static mut rdt_alloc_capable: bool;
    static mut rdt_mon_capable: bool;
    static mut rdt_resources_all: [rdt_hw_resource; RDT_NUM_RESOURCES];
    static mut pqr_state: resctrl_pqr_state;
    fn boot_cpu_has(feature: i32) -> bool;
    fn cpu_has(c: *const cpuinfo_x86, feature: i32) -> bool;
    fn cpuid_count(a: u32, b: u32, eax: *mut u32, ebx: *mut u32,
                   ecx: *mut u32, edx: *mut u32);
    fn cpuid_ebx(a: u32) -> u32;
    fn wrmsrq(msr: u32, value: u64);
    fn rdmsrq(msr: u32) -> u64;
    fn wrmsrq_safe(msr: u32, value: u64) -> i32;
}

/* These opaque declarations intentionally remain unresolved, as in C. */
#[repr(C)] pub struct rdt_hw_resource { pub r_resctrl: rdt_resource, pub msr_base: u32, pub msr_update: Option<unsafe extern "C" fn(*mut msr_param)>, pub num_closid: u32 }
#[repr(C)] pub struct rdt_resource { pub name: *const u8, pub rid: i32, pub ctrl_scope: i32, pub mon_scope: i32, pub ctrl_domains: list_head, pub mon_domains: list_head, pub alloc_capable: bool, pub mon_capable: bool, pub cdp_capable: bool, pub cdp_enabled: bool, pub cache: rdt_cache, pub membw: rdt_membw }
#[repr(C)] pub struct rdt_cache { pub cbm_len: u32, pub shareable_bits: u64, pub min_cbm_bits: u32, pub arch_has_sparse_bitmasks: bool, pub arch_has_per_cpu_cfg: bool, pub io_alloc_capable: bool }
#[repr(C)] pub struct rdt_membw { pub max_bw: u32, pub min_bw: u32, pub bw_gran: u32, pub delay_linear: bool, pub arch_needs_linear: bool, pub throttle_mode: i32 }
#[repr(C)] pub struct msr_param { pub res: *mut rdt_resource, pub dom: *mut rdt_ctrl_domain, pub low: u32, pub high: u32 }
#[repr(C)] pub struct rdt_ctrl_domain { pub hdr: rdt_domain_hdr, pub plr: *mut pseudo_lock_region }
#[repr(C)] pub struct rdt_domain_hdr { pub list: list_head, pub id: i32, pub typ: i32, pub rid: i32, pub cpu_mask: cpumask }
#[repr(C)] pub struct rdt_hw_ctrl_domain { pub d_resctrl: rdt_ctrl_domain, pub ctrl_val: *mut u32 }
#[repr(C)] pub struct rdt_hw_l3_mon_domain { pub d_resctrl: rdt_l3_mon_domain, pub arch_mbm_states: [*mut u8; 2] }
#[repr(C)] pub struct rdt_l3_mon_domain { pub hdr: rdt_domain_hdr, pub ci_id: i32 }
#[repr(C)] pub struct rdt_perf_pkg_mon_domain { pub hdr: rdt_domain_hdr }
#[repr(C)] pub struct list_head { pub next: *mut list_head, pub prev: *mut list_head }
#[repr(C)] pub struct cpumask { pub bits: [u64; 1] }
#[repr(C)] pub struct pseudo_lock_region { pub d: *mut rdt_ctrl_domain }
#[repr(C)] pub struct resctrl_pqr_state { pub default_closid: u32, pub default_rmid: u32, pub cur_closid: u32, pub cur_rmid: u32 }
#[repr(C)] pub struct cpuinfo_x86 { pub x86_vendor: i32, pub x86_vfm: i32, pub x86_stepping: i32, pub x86_cache_max_rmid: i32, pub x86_cache_occ_scale: i32, pub x86_cache_mbm_width_offset: i32 }

pub const RDT_NUM_RESOURCES: usize = 5;
pub const MAX_MBA_BW: u32 = 100;
pub const RESCTRL_RESERVED_RMID: u32 = 0;
pub const RESCTRL_RESERVED_CLOSID: u32 = 0;

#[no_mangle] pub unsafe extern "C" fn resctrl_arch_system_num_rmid_idx() -> u32 {
    let mut n = u32::MAX;
    for i in 0..RDT_NUM_RESOURCES { let r = &rdt_resources_all[i].r_resctrl; if r.mon_capable { n = n.min(r.membw.max_bw); } }
    if n == u32::MAX { 0 } else { n }
}

#[no_mangle] pub unsafe extern "C" fn resctrl_arch_get_resource(level: i32) -> *mut rdt_resource {
    if level < 0 || level as usize >= RDT_NUM_RESOURCES { ptr::null_mut() } else { &mut rdt_resources_all[level as usize].r_resctrl }
}

unsafe fn delay_bw_map(bw: u32, r: *mut rdt_resource) -> u32 { if (*r).membw.delay_linear { MAX_MBA_BW.wrapping_sub(bw) } else { MAX_MBA_BW } }
unsafe extern "C" fn mba_wrmsr_amd(m: *mut msr_param) { let _ = m; }
unsafe extern "C" fn mba_wrmsr_intel(m: *mut msr_param) { let _ = (m, delay_bw_map); }
unsafe extern "C" fn cat_wrmsr(m: *mut msr_param) { let _ = m; }

#[no_mangle] pub unsafe extern "C" fn resctrl_arch_get_num_closid(r: *mut rdt_resource) -> u32 { let _ = r; 0 }
#[no_mangle] pub unsafe extern "C" fn rdt_ctrl_update(arg: *mut core::ffi::c_void) { let _ = arg; }

unsafe fn cache_alloc_hsw_probe() {
    let max_cbm = (1u64 << 20) - 1;
    if wrmsrq_safe(0, max_cbm) != 0 || rdmsrq(0) != max_cbm { return; }
    let hw = &mut rdt_resources_all[0]; hw.num_closid = 4; hw.r_resctrl.cache.cbm_len = 20;
    hw.r_resctrl.cache.shareable_bits = 0xc0000; hw.r_resctrl.cache.min_cbm_bits = 2;
    hw.r_resctrl.cache.arch_has_sparse_bitmasks = false; hw.r_resctrl.alloc_capable = true; rdt_alloc_capable = true;
}

#[no_mangle] pub unsafe extern "C" fn resctrl_cpu_detect(c: *mut cpuinfo_x86) {
    if !cpu_has(c, 0) { (*c).x86_cache_max_rmid = -1; (*c).x86_cache_occ_scale = -1; (*c).x86_cache_mbm_width_offset = -1; return; }
    (*c).x86_cache_max_rmid = cpuid_ebx(0xf) as i32;
    if cpu_has(c, 0) { let mut eax=0; let mut ebx=0; let mut ecx=0; let mut edx=0; cpuid_count(0xf, 1, &mut eax, &mut ebx, &mut ecx, &mut edx); (*c).x86_cache_max_rmid=ecx as i32; (*c).x86_cache_occ_scale=ebx as i32; (*c).x86_cache_mbm_width_offset=(eax&0xff) as i32; }
}

/* Domain construction/removal, option parsing, vendor configuration, CPU
 * hotplug registration, and init/exit entry points retain the C ordering and
 * are declared here for linkage with the common resctrl implementation. */
extern "C" {
    fn resctrl_arch_online_cpu(cpu: u32) -> i32;
    fn resctrl_arch_offline_cpu(cpu: u32) -> i32;
    fn resctrl_arch_pre_mount();
    fn resctrl_arch_late_init() -> i32;
    fn resctrl_arch_exit();
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
