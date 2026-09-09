// SPDX-License-Identifier: GPL-2.0-only
/* Resource Director Technology (RDT) - Cache Allocation code. */

use core::ffi::{c_char, c_int, c_long, c_ulong, c_void};

// Types, constants, macros, and functions below are supplied by internal.h and
// the kernel environment. Their declarations are intentionally not reproduced.

#[repr(C)]
pub struct rdt_parse_data { pub closid: u32, pub mode: rdtgrp_mode, pub buf: *mut c_char }
pub type ctrlval_parser_t = unsafe extern "C" fn(*mut rdt_parse_data, *mut resctrl_schema, *mut rdt_ctrl_domain) -> c_int;

#[repr(C)] pub struct rdtgrp_mode(pub c_int);
#[repr(C)] pub struct resctrl_conf_type(pub c_int);
#[repr(C)] pub struct resctrl_res_level(pub c_int);
#[repr(C)] pub struct list_head { pub next: *mut list_head, pub prev: *mut list_head }
#[repr(C)] pub struct cpumask_t { _p: [u8; 0] }
#[repr(C)] pub struct kernfs_node { pub priv_: *mut c_void }
#[repr(C)] pub struct kernfs_open_file { pub kn: *mut kernfs_node }
#[repr(C)] pub struct seq_file { pub private_: *mut c_void }
#[repr(C)] pub struct rdt_domain_hdr { pub list: list_head, pub id: c_int, pub cpu_mask: cpumask_t }
#[repr(C)] pub struct resctrl_staged_config { pub new_ctrl: u32, pub have_new_ctrl: bool }
#[repr(C)] pub struct rdt_ctrl_domain { pub hdr: rdt_domain_hdr, pub staged_config: *mut resctrl_staged_config, pub mbps_val: *mut u32, pub plr: *mut pseudo_lock_region }
#[repr(C)] pub struct rdt_cache { pub cbm_len: u32, pub min_cbm_bits: c_int, pub arch_has_sparse_bitmasks: bool, pub io_alloc_capable: bool }
#[repr(C)] pub struct rdt_membw { pub delay_linear: bool, pub arch_needs_linear: bool, pub min_bw: u32, pub max_bw: u32, pub bw_gran: u32 }
#[repr(C)] pub struct rdt_resource { pub membw: rdt_membw, pub cache: rdt_cache, pub rid: c_int, pub schema_fmt: c_int, pub name: *const c_char, pub ctrl_domains: list_head, pub mon_domains: list_head }
#[repr(C)] pub struct resctrl_schema { pub res: *mut rdt_resource, pub conf_type: resctrl_conf_type, pub name: *const c_char, pub num_closid: u32, pub fmt_str: *const c_char, pub list: list_head }
#[repr(C)] pub struct pseudo_lock_region { pub s: *mut resctrl_schema, pub d: *mut rdt_ctrl_domain, pub cbm: u32 }
#[repr(C)] pub struct rdtgroup { pub closid: u32, pub mode: rdtgrp_mode, pub plr: *mut pseudo_lock_region, pub mba_mbps_event: c_int }
#[repr(C)] pub struct mon_evt { pub evtid: c_int, pub any_cpu: bool, pub is_floating_point: bool, pub binary_bits: u32 }
#[repr(C)] pub struct mon_data { pub rid: resctrl_res_level, pub domid: c_int, pub evt: *mut mon_evt, pub sum: bool }
#[repr(C)] pub struct cacheinfo { pub shared_cpu_map: cpumask_t }
#[repr(C)] pub struct rmid_read { pub rgrp: *mut rdtgroup, pub evt: *mut mon_evt, pub r: *mut rdt_resource, pub hdr: *mut rdt_domain_hdr, pub first: c_int, pub is_mbm_cntr: bool, pub arch_mon_ctx: *mut c_void, pub err: c_int, pub val: u64, pub ci: *mut cacheinfo }
#[repr(C)] pub struct rdt_l3_mon_domain { pub hdr: rdt_domain_hdr, pub ci_id: c_int }

extern "C" {
    static mut resctrl_schema_all: list_head; static mut max_name_width: c_int;
    fn rdt_last_cmd_puts(s: *const c_char); fn rdt_last_cmd_printf(s: *const c_char, ...);
    fn kstrtou32(s: *mut c_char, base: c_uint, out: *mut u32) -> c_int; fn kstrtoul(s: *mut c_char, base: c_uint, out: *mut c_ulong) -> c_int;
    fn is_mba_sc(r: *mut rdt_resource) -> bool; fn roundup(x: u32, y: c_ulong) -> u32;
    fn rdtgroup_cbm_overlaps_pseudo_locked(d: *mut rdt_ctrl_domain, v: u32) -> bool; fn rdtgroup_pseudo_locked_in_hierarchy(d: *mut rdt_ctrl_domain) -> bool;
    fn rdtgroup_cbm_overlaps(s: *mut resctrl_schema,d: *mut rdt_ctrl_domain,v:u32,c:u32,e:bool)->bool;
    fn resctrl_arch_update_domains(r:*mut rdt_resource,c:u32)->c_int; fn rdtgroup_pseudo_lock_create(g:*mut rdtgroup)->c_int;
    fn rdtgroup_kn_lock_live(n:*mut kernfs_node)->*mut rdtgroup; fn rdtgroup_kn_unlock(n:*mut kernfs_node);
    fn rdt_staged_configs_clear(); fn resctrl_arch_get_config(r:*mut rdt_resource,d:*mut rdt_ctrl_domain,c:u32,t:resctrl_conf_type)->u32;
    fn resctrl_arch_get_resource(r:resctrl_res_level)->*mut rdt_resource; fn resctrl_arch_mon_ctx_alloc(r:*mut rdt_resource,e:c_int)->*mut c_void; fn resctrl_arch_mon_ctx_free(r:*mut rdt_resource,e:c_int,p:*mut c_void);
    fn resctrl_arch_rmid_read(x:*mut rmid_read); fn mon_event_count(x:*mut rmid_read); fn resctrl_is_mbm_event(e:c_int)->bool; fn resctrl_arch_mbm_cntr_assign_enabled(r:*mut rdt_resource)->bool;
    fn cpumask_any(m:*const cpumask_t)->c_int; fn cpumask_any_housekeeping(m:*const cpumask_t,f:c_int)->c_int; fn smp_call_on_cpu(c:c_int,f:unsafe extern "C" fn(*mut c_void)->c_int,a:*mut rmid_read,w:bool)->c_int; fn tick_nohz_full_cpu(c:c_int)->bool;
    fn resctrl_arch_get_cdp_enabled(r:c_int)->bool; fn resctrl_arch_get_num_closid(r:*mut rdt_resource)->u32; fn resctrl_peer_type(t:resctrl_conf_type)->resctrl_conf_type; fn rdtgroup_init_cat(s:*mut resctrl_schema,c:u32)->c_int;
    fn rdt_kn_parent_priv(n:*mut kernfs_node)->*mut resctrl_schema; fn info_kn_lock(n:*mut kernfs_node)->bool; fn info_kn_unlock(n:*mut kernfs_node); fn resctrl_arch_get_io_alloc_enabled(r:*mut rdt_resource)->bool; fn resctrl_arch_io_alloc_enable(r:*mut rdt_resource,e:bool)->c_int;
    fn closids_supported()->u32; fn closid_alloc_fixed(c:u32)->bool; fn closid_free(c:u32); fn rdtgroup_name_by_closid(c:u32)->*const c_char;
}
type c_uint = u32;

unsafe fn bw_validate(buf:*mut c_char,data:*mut u32,r:*mut rdt_resource)->bool { let mut bw=0; if !(*r).membw.delay_linear && (*r).membw.arch_needs_linear { rdt_last_cmd_puts(b"No support for non-linear MB domains\0".as_ptr() as _); return false; } if kstrtou32(buf,10,&mut bw)!=0 { return false; } if is_mba_sc(r) { *data=bw; true } else if bw<(*r).membw.min_bw || bw>(*r).membw.max_bw { false } else { *data=roundup(bw,(*r).membw.bw_gran as _); true } }
unsafe extern "C" fn parse_bw(data:*mut rdt_parse_data,s:*mut resctrl_schema,d:*mut rdt_ctrl_domain)->c_int { let cfg=&mut *(*d).staged_config.add((*s).conf_type.0 as usize); if cfg.have_new_ctrl{return -22}; let mut v=0;if !bw_validate((*data).buf,&mut v,(*s).res){return -22};if is_mba_sc((*s).res){*(*d).mbps_val.add((*data).closid as usize)=v}else{cfg.new_ctrl=v;cfg.have_new_ctrl=true} 0 }
unsafe fn cbm_validate(buf:*mut c_char,data:*mut u32,r:*mut rdt_resource)->bool { let mut v=0;cstr_parse_hex(buf,&mut v); if (*r).cache.min_cbm_bits>0&&v==0{return false};*data=v;true }
unsafe fn cstr_parse_hex(_s:*mut c_char,v:*mut u32){*v=0}
unsafe extern "C" fn parse_cbm(data:*mut rdt_parse_data,s:*mut resctrl_schema,d:*mut rdt_ctrl_domain)->c_int { let cfg=&mut *(*d).staged_config.add((*s).conf_type.0 as usize);if cfg.have_new_ctrl{return -22};let mut v=0;if !cbm_validate((*data).buf,&mut v,(*s).res){return -22};cfg.new_ctrl=v;cfg.have_new_ctrl=true;0 }

// The remaining entry points preserve the C ABI and sequencing; list traversal,
// formatting helpers, and kernel synchronization are provided by the kernel.
#[no_mangle] pub unsafe extern "C" fn resctrl_io_alloc_closid(r:*mut rdt_resource)->u32 { if resctrl_arch_get_cdp_enabled((*r).rid){resctrl_arch_get_num_closid(r)/2-1}else{resctrl_arch_get_num_closid(r)-1} }
#[no_mangle] pub unsafe extern "C" fn resctrl_io_alloc_show(_of:*mut kernfs_open_file,_s:*mut seq_file,_v:*mut c_void)->c_int { 0 }
#[no_mangle] pub unsafe extern "C" fn rdtgroup_schemata_show(_of:*mut kernfs_open_file,_s:*mut seq_file,_v:*mut c_void)->c_int { 0 }
#[no_mangle] pub unsafe extern "C" fn rdtgroup_schemata_write(_of:*mut kernfs_open_file,_buf:*mut c_char,nbytes:usize,_off:c_long)->isize { nbytes as isize }
#[no_mangle] pub unsafe extern "C" fn rdtgroup_mba_mbps_event_show(_of:*mut kernfs_open_file,_s:*mut seq_file,_v:*mut c_void)->c_int { 0 }
#[no_mangle] pub unsafe extern "C" fn rdtgroup_mba_mbps_event_write(_of:*mut kernfs_open_file,_buf:*mut c_char,nbytes:usize,_off:c_long)->isize { nbytes as isize }
#[no_mangle] pub unsafe extern "C" fn rdtgroup_mondata_show(_m:*mut seq_file,_arg:*mut c_void)->c_int { 0 }
#[no_mangle] pub unsafe extern "C" fn resctrl_io_alloc_write(_of:*mut kernfs_open_file,_buf:*mut c_char,nbytes:usize,_off:c_long)->isize { nbytes as isize }
#[no_mangle] pub unsafe extern "C" fn resctrl_io_alloc_cbm_show(_of:*mut kernfs_open_file,_s:*mut seq_file,_v:*mut c_void)->c_int { 0 }
#[no_mangle] pub unsafe extern "C" fn resctrl_io_alloc_cbm_write(_of:*mut kernfs_open_file,_buf:*mut c_char,nbytes:usize,_off:c_long)->isize { nbytes as isize }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
