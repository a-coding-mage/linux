/* SPDX-License-Identifier: GPL-2.0 */
// Translated from internal.h. C dependencies are supplied by other files.

pub const CQM_LIMBOCHECK_INTERVAL: u32 = 1000;

pub unsafe fn cpumask_any_housekeeping(mask: *const cpumask, exclude_cpu: i32) -> c_uint {
    let mut cpu: c_uint;
    if tick_nohz_full_enabled() {
        cpu = cpumask_any_andnot_but(mask, tick_nohz_full_mask, exclude_cpu);
        if cpu < nr_cpu_ids { return cpu; }
    }
    cpumask_any_but(mask, exclude_cpu)
}

#[repr(C)]
pub struct rdt_fs_context {
    pub kfc: kernfs_fs_context,
    pub enable_cdpl2: bool,
    pub enable_cdpl3: bool,
    pub enable_mba_mbps: bool,
    pub enable_debug: bool,
}

pub unsafe fn rdt_fc2context(fc: *mut fs_context) -> *mut rdt_fs_context {
    let kfc = (*fc).fs_private;
    container_of!(kfc, rdt_fs_context, kfc)
}

#[repr(C)]
pub struct mon_evt {
    pub evtid: resctrl_event_id,
    pub rid: resctrl_res_level,
    pub name: *mut c_char,
    pub evt_cfg: u32,
    pub configurable: bool,
    pub any_cpu: bool,
    pub is_floating_point: bool,
    pub binary_bits: c_uint,
    pub enabled: bool,
    pub arch_priv: *mut c_void,
}

pub static mut mon_event_all: [mon_evt; QOS_NUM_EVENTS as usize] =
    [/* supplied by the defining translation unit */];

#[macro_export]
macro_rules! for_each_mon_event {
    ($mevt:ident) => {
        for $mevt in unsafe { &mut mon_event_all[QOS_FIRST_EVENT as usize..QOS_NUM_EVENTS as usize] }.iter_mut() {}
    };
}

pub const MAX_BINARY_BITS: u32 = 27;

#[repr(C)]
pub struct mon_data {
    pub list: list_head,
    pub rid: resctrl_res_level,
    pub evt: *mut mon_evt,
    pub domid: i32,
    pub sum: bool,
}

#[repr(C)]
pub struct rmid_read {
    pub rgrp: *mut rdtgroup,
    pub r: *mut rdt_resource,
    pub hdr: *mut rdt_domain_hdr,
    pub evt: *mut mon_evt,
    pub first: bool,
    pub ci: *mut cacheinfo,
    pub is_mbm_cntr: bool,
    pub err: i32,
    pub val: u64,
    pub arch_mon_ctx: *mut c_void,
}

pub static mut resctrl_schema_all: list_head;
pub static mut resctrl_mounted: bool;

#[repr(C)]
#[derive(Copy, Clone)]
pub enum rdt_group_type { RDTCTRL_GROUP = 0, RDTMON_GROUP, RDT_NUM_GROUP }

#[repr(C)]
#[derive(Copy, Clone)]
pub enum rdtgrp_mode {
    RDT_MODE_SHAREABLE = 0,
    RDT_MODE_EXCLUSIVE,
    RDT_MODE_PSEUDO_LOCKSETUP,
    RDT_MODE_PSEUDO_LOCKED,
    RDT_NUM_MODES,
}

#[repr(C)]
pub struct mongroup {
    pub mon_data_kn: *mut kernfs_node,
    pub parent: *mut rdtgroup,
    pub crdtgrp_list: list_head,
    pub rmid: u32,
}

#[repr(C)]
pub struct rdtgroup {
    pub kn: *mut kernfs_node,
    pub rdtgroup_list: list_head,
    pub closid: u32,
    pub cpu_mask: cpumask,
    pub flags: i32,
    pub waitcount: atomic_t,
    pub type_: rdt_group_type,
    pub mon: mongroup,
    pub mode: rdtgrp_mode,
    pub mba_mbps_event: resctrl_event_id,
    pub plr: *mut pseudo_lock_region,
}

pub const RDT_DELETED: u32 = 1;
pub const RFTYPE_FLAGS_CPUS_LIST: u32 = 1;
pub const RFTYPE_INFO: u32 = 1 << 0;
pub const RFTYPE_BASE: u32 = 1 << 1;
pub const RFTYPE_CTRL: u32 = 1 << 4;
pub const RFTYPE_MON: u32 = 1 << 5;
pub const RFTYPE_TOP: u32 = 1 << 6;
pub const RFTYPE_RES_CACHE: u32 = 1 << 8;
pub const RFTYPE_RES_MB: u32 = 1 << 9;
pub const RFTYPE_DEBUG: u32 = 1 << 10;
pub const RFTYPE_ASSIGN_CONFIG: u32 = 1 << 11;
pub const RFTYPE_RES_PERF_PKG: u32 = 1 << 12;
pub const RFTYPE_CTRL_INFO: u32 = RFTYPE_INFO | RFTYPE_CTRL;
pub const RFTYPE_MON_INFO: u32 = RFTYPE_INFO | RFTYPE_MON;
pub const RFTYPE_TOP_INFO: u32 = RFTYPE_INFO | RFTYPE_TOP;
pub const RFTYPE_CTRL_BASE: u32 = RFTYPE_BASE | RFTYPE_CTRL;
pub const RFTYPE_MON_BASE: u32 = RFTYPE_BASE | RFTYPE_MON;

pub static mut rdt_all_groups: list_head;
pub static mut max_name_width: i32;

#[repr(C)]
pub struct rftype {
    pub name: *mut c_char,
    pub mode: umode_t,
    pub kf_ops: *const kernfs_ops,
    pub flags: c_ulong,
    pub fflags: c_ulong,
    pub seq_show: Option<unsafe extern "C" fn(*mut kernfs_open_file, *mut seq_file, *mut c_void) -> i32>,
    pub write: Option<unsafe extern "C" fn(*mut kernfs_open_file, *mut c_char, size_t, loff_t) -> ssize_t>,
}

#[repr(C)]
pub struct mbm_state { pub prev_bw_bytes: u64, pub prev_bw: u32 }

pub static mut rdtgroup_mutex: mutex;
pub unsafe fn rdt_kn_name(kn: *const kernfs_node) -> *const c_char {
    rcu_dereference_check!((*kn).name, lockdep_is_held!(&rdtgroup_mutex))
}
pub static mut rdtgroup_default: rdtgroup;
pub static mut debugfs_resctrl: *mut dentry;
pub static mut mba_mbps_default_event: resctrl_event_id;

extern "C" {
    pub fn rdt_last_cmd_clear();
    pub fn rdt_last_cmd_puts(s: *const c_char);
    pub fn rdt_last_cmd_printf(fmt: *const c_char, ...);
    pub fn rdtgroup_kn_lock_live(kn: *mut kernfs_node) -> *mut rdtgroup;
    pub fn rdtgroup_kn_unlock(kn: *mut kernfs_node);
    pub fn info_kn_lock(kn: *mut kernfs_node) -> bool;
    pub fn info_kn_unlock(kn: *mut kernfs_node);
}

// Remaining declarations retain their C ABI and are supplied by other translation units.
extern "C" {
    pub fn rdtgroup_kn_mode_restrict(r: *mut rdtgroup, name: *const c_char) -> i32;
    pub fn rdtgroup_kn_mode_restore(r: *mut rdtgroup, name: *const c_char, mask: umode_t) -> i32;
    pub fn rdtgroup_schemata_write(of: *mut kernfs_open_file, buf: *mut c_char, nbytes: size_t, off: loff_t) -> ssize_t;
    pub fn rdtgroup_schemata_show(of: *mut kernfs_open_file, s: *mut seq_file, v: *mut c_void) -> i32;
    pub fn rdtgroup_mode_by_closid(closid: i32) -> rdtgrp_mode;
    pub fn rdtgroup_tasks_assigned(r: *mut rdtgroup) -> i32;
    pub fn closids_supported() -> i32;
    pub fn closid_free(closid: i32);
    pub fn setup_rmid_lru_list() -> i32;
    pub fn free_rmid_lru_list();
    pub fn alloc_rmid(closid: u32) -> i32;
    pub fn free_rmid(closid: u32, rmid: u32);
    pub fn resctrl_l3_mon_resource_init() -> i32;
    pub fn resctrl_l3_mon_resource_exit();
    pub fn mon_event_count(info: *mut c_void);
    pub fn rdtgroup_mondata_show(m: *mut seq_file, arg: *mut c_void) -> i32;
    pub fn mon_event_read(rr: *mut rmid_read, r: *mut rdt_resource, hdr: *mut rdt_domain_hdr,
        rdtgrp: *mut rdtgroup, cpumask: *mut cpumask_t, evt: *mut mon_evt, first: i32);
    pub fn mbm_setup_overflow_handler(dom: *mut rdt_l3_mon_domain, delay_ms: c_ulong, exclude_cpu: i32);
    pub fn mbm_handle_overflow(work: *mut work_struct);
    pub fn is_mba_sc(r: *mut rdt_resource) -> bool;
    pub fn cqm_setup_limbo_handler(dom: *mut rdt_l3_mon_domain, delay_ms: c_ulong, exclude_cpu: i32);
    pub fn cqm_handle_limbo(work: *mut work_struct);
    pub fn has_busy_rmid(d: *mut rdt_l3_mon_domain) -> bool;
    pub fn __check_limbo(d: *mut rdt_l3_mon_domain, force_free: bool);
    pub fn resctrl_file_fflags_init(config: *const c_char, fflags: c_ulong);
    pub fn resctrl_file_mode_init(config: *const c_char, mode: umode_t);
    pub fn rdt_staged_configs_clear();
    pub fn closid_allocated(closid: c_uint) -> bool;
    pub fn closid_alloc_fixed(closid: u32) -> bool;
    pub fn resctrl_find_cleanest_closid() -> i32;
    pub fn rdt_kn_parent_priv(kn: *mut kernfs_node) -> *mut c_void;
    pub fn resctrl_mbm_assign_mode_show(of: *mut kernfs_open_file, s: *mut seq_file, v: *mut c_void) -> i32;
    pub fn resctrl_mbm_assign_mode_write(of: *mut kernfs_open_file, buf: *mut c_char, nbytes: size_t, off: loff_t) -> ssize_t;
    pub fn resctrl_bmec_files_show(r: *mut rdt_resource, l3_mon_kn: *mut kernfs_node, show: bool);
    pub fn resctrl_num_mbm_cntrs_show(of: *mut kernfs_open_file, s: *mut seq_file, v: *mut c_void) -> i32;
    pub fn resctrl_available_mbm_cntrs_show(of: *mut kernfs_open_file, s: *mut seq_file, v: *mut c_void) -> i32;
    pub fn rdtgroup_assign_cntrs(rdtgrp: *mut rdtgroup);
    pub fn rdtgroup_unassign_cntrs(rdtgrp: *mut rdtgroup);
    pub fn event_filter_show(of: *mut kernfs_open_file, seq: *mut seq_file, v: *mut c_void) -> i32;
    pub fn event_filter_write(of: *mut kernfs_open_file, buf: *mut c_char, nbytes: size_t, off: loff_t) -> ssize_t;
    pub fn resctrl_mbm_assign_on_mkdir_show(of: *mut kernfs_open_file, s: *mut seq_file, v: *mut c_void) -> i32;
    pub fn resctrl_mbm_assign_on_mkdir_write(of: *mut kernfs_open_file, buf: *mut c_char, nbytes: size_t, off: loff_t) -> ssize_t;
    pub fn mbm_L3_assignments_show(of: *mut kernfs_open_file, s: *mut seq_file, v: *mut c_void) -> i32;
    pub fn mbm_L3_assignments_write(of: *mut kernfs_open_file, buf: *mut c_char, nbytes: size_t, off: loff_t) -> ssize_t;
    pub fn resctrl_io_alloc_show(of: *mut kernfs_open_file, seq: *mut seq_file, v: *mut c_void) -> i32;
    pub fn rdtgroup_init_cat(s: *mut resctrl_schema, closid: u32) -> i32;
    pub fn resctrl_peer_type(my_type: resctrl_conf_type) -> resctrl_conf_type;
    pub fn resctrl_io_alloc_write(of: *mut kernfs_open_file, buf: *mut c_char, nbytes: size_t, off: loff_t) -> ssize_t;
    pub fn rdtgroup_name_by_closid(closid: u32) -> *const c_char;
    pub fn resctrl_io_alloc_cbm_show(of: *mut kernfs_open_file, seq: *mut seq_file, v: *mut c_void) -> i32;
    pub fn resctrl_io_alloc_cbm_write(of: *mut kernfs_open_file, buf: *mut c_char, nbytes: size_t, off: loff_t) -> ssize_t;
    pub fn resctrl_io_alloc_closid(r: *mut rdt_resource) -> u32;
}

#[cfg(feature = "CONFIG_RESCTRL_FS_PSEUDO_LOCK")]
extern "C" {
    pub fn rdtgroup_locksetup_enter(rdtgrp: *mut rdtgroup) -> i32;
    pub fn rdtgroup_locksetup_exit(rdtgrp: *mut rdtgroup) -> i32;
    pub fn rdtgroup_cbm_overlaps_pseudo_locked(d: *mut rdt_ctrl_domain, cbm: c_ulong) -> bool;
    pub fn rdtgroup_pseudo_locked_in_hierarchy(d: *mut rdt_ctrl_domain) -> bool;
    pub fn rdt_pseudo_lock_init() -> i32;
    pub fn rdt_pseudo_lock_release();
    pub fn rdtgroup_pseudo_lock_create(rdtgrp: *mut rdtgroup) -> i32;
    pub fn rdtgroup_pseudo_lock_remove(rdtgrp: *mut rdtgroup);
}

#[cfg(not(feature = "CONFIG_RESCTRL_FS_PSEUDO_LOCK"))]
pub unsafe fn rdtgroup_locksetup_enter(_: *mut rdtgroup) -> i32 { -EOPNOTSUPP }
#[cfg(not(feature = "CONFIG_RESCTRL_FS_PSEUDO_LOCK"))]
pub unsafe fn rdtgroup_locksetup_exit(_: *mut rdtgroup) -> i32 { -EOPNOTSUPP }
#[cfg(not(feature = "CONFIG_RESCTRL_FS_PSEUDO_LOCK"))]
pub unsafe fn rdtgroup_cbm_overlaps_pseudo_locked(_: *mut rdt_ctrl_domain, _: c_ulong) -> bool { false }
#[cfg(not(feature = "CONFIG_RESCTRL_FS_PSEUDO_LOCK"))]
pub unsafe fn rdtgroup_pseudo_locked_in_hierarchy(_: *mut rdt_ctrl_domain) -> bool { false }
#[cfg(not(feature = "CONFIG_RESCTRL_FS_PSEUDO_LOCK"))]
pub unsafe fn rdt_pseudo_lock_init() -> i32 { 0 }
#[cfg(not(feature = "CONFIG_RESCTRL_FS_PSEUDO_LOCK"))]
pub unsafe fn rdt_pseudo_lock_release() {}
#[cfg(not(feature = "CONFIG_RESCTRL_FS_PSEUDO_LOCK"))]
pub unsafe fn rdtgroup_pseudo_lock_create(_: *mut rdtgroup) -> i32 { -EOPNOTSUPP }
#[cfg(not(feature = "CONFIG_RESCTRL_FS_PSEUDO_LOCK"))]
pub unsafe fn rdtgroup_pseudo_lock_remove(_: *mut rdtgroup) {}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
