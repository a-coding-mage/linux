/* SPDX-License-Identifier: GPL-2.0 */
/* block cgroup private header; C includes and header guards omitted. */

// External kernel types, constants, macros, and functions are supplied by dependencies.
// struct blkcg_gq; struct blkg_policy_data; (forward declarations)

pub const BLKG_STAT_CPU_BATCH: i32 = i32::MAX / 2;

#[cfg(CONFIG_BLK_CGROUP)]
#[repr(C)]
pub enum blkg_iostat_type { BLKG_IOSTAT_READ, BLKG_IOSTAT_WRITE, BLKG_IOSTAT_DISCARD, BLKG_IOSTAT_NR }

#[cfg(CONFIG_BLK_CGROUP)]
#[repr(C)]
pub struct blkg_iostat { pub bytes: [u64; 3], pub ios: [u64; 3] }

#[cfg(CONFIG_BLK_CGROUP)]
#[repr(C)]
pub struct blkg_iostat_set {
    pub sync: u64_stats_sync,
    pub blkg: *mut blkcg_gq,
    pub lnode: llist_node,
    pub lqueued: c_int,
    pub cur: blkg_iostat,
    pub last: blkg_iostat,
}

#[cfg(CONFIG_BLK_CGROUP)]
#[repr(C)]
pub struct blkcg_gq {
    pub q: *mut request_queue, pub q_node: list_head, pub blkcg_node: hlist_node,
    pub blkcg: *mut blkcg, pub parent: *mut blkcg_gq, pub refcnt: percpu_ref,
    pub online: bool, pub iostat_cpu: *mut blkg_iostat_set, pub iostat: blkg_iostat,
    pub pd: [*mut blkg_policy_data; BLKCG_MAX_POLS as usize],
    // CONFIG_BLK_CGROUP_PUNT_BIO fields
    pub async_bio_lock: spinlock_t, pub async_bios: bio_list,
    pub work: blkcg_gq_work,
    pub use_delay: atomic_t, pub delay_nsec: atomic64_t, pub delay_start: atomic64_t,
    pub last_delay: u64, pub last_use: c_int, pub rcu_head: rcu_head,
}

#[cfg(CONFIG_BLK_CGROUP)]
#[repr(C)]
pub union blkcg_gq_work { pub async_bio_work: work_struct, pub free_work: work_struct }

#[cfg(CONFIG_BLK_CGROUP)]
#[repr(C)]
pub struct blkcg {
    pub css: cgroup_subsys_state, pub lock: spinlock_t, pub online_pin: refcount_t,
    pub congestion_count: atomic_t, pub blkg_tree: radix_tree_root,
    pub blkg_hint: *mut blkcg_gq, pub blkg_list: hlist_head,
    pub cpd: [*mut blkcg_policy_data; BLKCG_MAX_POLS as usize],
    pub all_blkcgs_node: list_head, pub lhead: *mut llist_head,
    // CONFIG_BLK_CGROUP_FC_APPID: char fc_app_id[FC_APPID_LEN]
    // CONFIG_CGROUP_WRITEBACK: struct list_head cgwb_list
}

#[cfg(CONFIG_BLK_CGROUP)]
#[repr(C)]
pub struct blkg_policy_data { pub blkg: *mut blkcg_gq, pub plid: c_int, pub online: bool, pub rcu_head: rcu_head }

#[cfg(CONFIG_BLK_CGROUP)]
#[repr(C)]
pub struct blkcg_policy_data { pub blkcg: *mut blkcg, pub plid: c_int }

pub type blkcg_pol_alloc_cpd_fn = unsafe extern "C" fn(gfp_t) -> *mut blkcg_policy_data;
pub type blkcg_pol_init_cpd_fn = unsafe extern "C" fn(*mut blkcg_policy_data);
pub type blkcg_pol_free_cpd_fn = unsafe extern "C" fn(*mut blkcg_policy_data);
pub type blkcg_pol_bind_cpd_fn = unsafe extern "C" fn(*mut blkcg_policy_data);
pub type blkcg_pol_alloc_pd_fn = unsafe extern "C" fn(*mut gendisk, *mut blkcg, gfp_t) -> *mut blkg_policy_data;
pub type blkcg_pol_init_pd_fn = unsafe extern "C" fn(*mut blkg_policy_data);
pub type blkcg_pol_online_pd_fn = unsafe extern "C" fn(*mut blkg_policy_data);
pub type blkcg_pol_offline_pd_fn = unsafe extern "C" fn(*mut blkg_policy_data);
pub type blkcg_pol_free_pd_fn = unsafe extern "C" fn(*mut blkg_policy_data);
pub type blkcg_pol_reset_pd_stats_fn = unsafe extern "C" fn(*mut blkg_policy_data);
pub type blkcg_pol_stat_pd_fn = unsafe extern "C" fn(*mut blkg_policy_data, *mut seq_file);

#[cfg(CONFIG_BLK_CGROUP)]
#[repr(C)]
pub struct blkcg_policy {
    pub plid: c_int, pub dfl_cftypes: *mut cftype, pub legacy_cftypes: *mut cftype,
    pub cpd_alloc_fn: Option<blkcg_pol_alloc_cpd_fn>, pub cpd_free_fn: Option<blkcg_pol_free_cpd_fn>,
    pub pd_alloc_fn: Option<blkcg_pol_alloc_pd_fn>, pub pd_init_fn: Option<blkcg_pol_init_pd_fn>,
    pub pd_online_fn: Option<blkcg_pol_online_pd_fn>, pub pd_offline_fn: Option<blkcg_pol_offline_pd_fn>,
    pub pd_free_fn: Option<blkcg_pol_free_pd_fn>, pub pd_reset_stats_fn: Option<blkcg_pol_reset_pd_stats_fn>,
    pub pd_stat_fn: Option<blkcg_pol_stat_pd_fn>,
}

#[cfg(CONFIG_BLK_CGROUP)]
extern "C" {
    pub static mut blkcg_root: blkcg; pub static mut blkcg_debug_stats: bool;
    pub fn blkg_init_queue(q: *mut request_queue); pub fn blkcg_init_disk(disk: *mut gendisk) -> c_int; pub fn blkcg_exit_disk(disk: *mut gendisk);
    pub fn blkcg_policy_register(pol: *mut blkcg_policy) -> c_int; pub fn blkcg_policy_unregister(pol: *mut blkcg_policy);
    pub fn blkcg_activate_policy(disk: *mut gendisk, pol: *const blkcg_policy) -> c_int;
    pub fn blkcg_deactivate_policy(disk: *mut gendisk, pol: *const blkcg_policy);
    pub fn blkg_conf_init(ctx: *mut blkg_conf_ctx, input: *mut c_char);
    pub fn blk_cgroup_bio_start(bio: *mut bio); pub fn blkcg_add_delay(blkg: *mut blkcg_gq, now: u64, delta: u64);
}

#[cfg(CONFIG_BLK_CGROUP)]
#[repr(C)]
pub struct blkg_conf_ctx { pub input: *mut c_char, pub body: *mut c_char, pub bdev: *mut block_device, pub blkg: *mut blkcg_gq }

#[cfg(CONFIG_BLK_CGROUP)]
pub unsafe fn bio_issue_as_root_blkg(bio: *mut bio) -> bool { ((*bio).bi_opf & (REQ_META | REQ_SWAP)) != 0 }

#[cfg(not(CONFIG_BLK_CGROUP))]
#[repr(C)] pub struct blkg_policy_data {}
#[cfg(not(CONFIG_BLK_CGROUP))]
#[repr(C)] pub struct blkcg_policy_data {}
#[cfg(not(CONFIG_BLK_CGROUP))]
#[repr(C)] pub struct blkcg_policy {}
#[cfg(not(CONFIG_BLK_CGROUP))]
#[repr(C)] pub struct blkcg {}

// CONFIG_BLK_CGROUP-disabled inline stubs retain the C return behavior.
#[cfg(not(CONFIG_BLK_CGROUP))] pub unsafe fn blkg_lookup(_: *mut blkcg, _: *mut core::ffi::c_void) -> *mut blkcg_gq { core::ptr::null_mut() }
#[cfg(not(CONFIG_BLK_CGROUP))] pub unsafe fn blkcg_init_disk(_: *mut gendisk) -> c_int { 0 }
#[cfg(not(CONFIG_BLK_CGROUP))] pub unsafe fn blkcg_policy_register(_: *mut blkcg_policy) -> c_int { 0 }
#[cfg(not(CONFIG_BLK_CGROUP))] pub unsafe fn blk_cgroup_mergeable(_: *mut request, _: *mut bio) -> bool { true }

// The remaining inline helpers and iteration macros retain their C interfaces and dependency
// operations; annotations such as __cond_acquires are source-only lockdep metadata.
extern "C" {
    pub fn blkg_dev_name(blkg: *mut blkcg_gq) -> *const c_char;
    pub fn blkcg_print_blkgs(sf: *mut seq_file, blkcg: *mut blkcg,
        prfill: Option<unsafe extern "C" fn(*mut seq_file, *mut blkg_policy_data, c_int) -> u64>,
        pol: *const blkcg_policy, data: c_int, show_total: bool);
    pub fn __blkg_prfill_u64(sf: *mut seq_file, pd: *mut blkg_policy_data, v: u64) -> u64;
    pub fn blkg_conf_open_bdev(ctx: *mut blkg_conf_ctx) -> c_int;
    pub fn blkg_conf_prep(blkcg: *mut blkcg, pol: *const blkcg_policy, ctx: *mut blkg_conf_ctx) -> c_int;
    pub fn blkg_conf_unprep(ctx: *mut blkg_conf_ctx);
    pub fn blkg_conf_close_bdev(ctx: *mut blkg_conf_ctx);
    pub fn blkg_get(blkg: *mut blkcg_gq);
    pub fn blkg_tryget(blkg: *mut blkcg_gq) -> bool;
    pub fn blkg_put(blkg: *mut blkcg_gq);
    pub fn blkcg_inc_congestion_count(blkcg: *mut blkcg);
    pub fn blkcg_dec_congestion_count(blkcg: *mut blkcg);
    pub fn blkcg_use_delay(blkg: *mut blkcg_gq);
    pub fn blkcg_unuse_delay(blkg: *mut blkcg_gq) -> c_int;
    pub fn blkcg_set_delay(blkg: *mut blkcg_gq, delay: u64);
    pub fn blkcg_clear_delay(blkg: *mut blkcg_gq);
    pub fn blkcg_policy_enabled(q: *mut request_queue, pol: *const blkcg_policy) -> bool;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
