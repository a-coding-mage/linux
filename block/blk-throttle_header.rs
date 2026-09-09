/* SPDX-License-Identifier: GPL-2.0 */

/* Dependency declarations are supplied by the surrounding translation unit. */

/*
 * To implement hierarchical throttling, throtl_grps form a tree and bios
 * are dispatched upwards level by level until they reach the top and get
 * issued. When dispatching bios from the children and local group at each
 * level, they are kept in separate queues and dispatched round-robin to avoid
 * starvation.
 */
#[repr(C)]
pub struct throtl_qnode {
    pub node: list_head,
    pub bios_bps: bio_list,
    pub bios_iops: bio_list,
    pub tg: *mut throtl_grp,
}

#[repr(C)]
pub struct throtl_service_queue {
    pub parent_sq: *mut throtl_service_queue,
    pub queued: [list_head; 2],
    pub nr_queued_bps: [c_uint; 2],
    pub nr_queued_iops: [c_uint; 2],
    pub pending_tree: rb_root_cached,
    pub nr_pending: c_uint,
    pub first_pending_disptime: c_ulong,
    pub pending_timer: timer_list,
}

#[repr(C)]
pub enum tg_state_flags {
    THROTL_TG_PENDING = 1 << 0,
    THROTL_TG_WAS_EMPTY = 1 << 1,
    THROTL_TG_IOPS_WAS_EMPTY = 1 << 2,
    THROTL_TG_CANCELING = 1 << 3,
}

#[repr(C)]
pub struct throtl_grp {
    /* must be the first member */
    pub pd: blkg_policy_data,
    pub rb_node: rb_node,
    pub td: *mut throtl_data,
    pub service_queue: throtl_service_queue,
    pub qnode_on_self: [throtl_qnode; 2],
    pub qnode_on_parent: [throtl_qnode; 2],
    pub disptime: c_ulong,
    pub flags: c_uint,
    pub has_rules_bps: [bool; 2],
    pub has_rules_iops: [bool; 2],
    pub bps: [u64; 2],
    pub iops: [c_uint; 2],
    pub bytes_disp: [i64; 2],
    pub io_disp: [c_int; 2],
    pub slice_start: [c_ulong; 2],
    pub slice_end: [c_ulong; 2],
    pub stat_bytes: blkg_rwstat,
    pub stat_ios: blkg_rwstat,
}

pub extern "C" {
    pub static mut blkcg_policy_throtl: blkcg_policy;
}

#[inline]
pub unsafe fn pd_to_tg(pd: *mut blkg_policy_data) -> *mut throtl_grp {
    if !pd.is_null() {
        container_of!(pd, throtl_grp, pd)
    } else {
        core::ptr::null_mut()
    }
}

#[inline]
pub unsafe fn blkg_to_tg(blkg: *mut blkcg_gq) -> *mut throtl_grp {
    pd_to_tg(blkg_to_pd(blkg, &raw mut blkcg_policy_throtl))
}

#[cfg(not(feature = "CONFIG_BLK_DEV_THROTTLING"))]
#[inline]
pub unsafe fn blk_throtl_exit(_disk: *mut gendisk) {}

#[cfg(not(feature = "CONFIG_BLK_DEV_THROTTLING"))]
#[inline]
pub unsafe fn blk_throtl_bio(_bio: *mut bio) -> bool { false }

#[cfg(not(feature = "CONFIG_BLK_DEV_THROTTLING"))]
#[inline]
pub unsafe fn blk_throtl_cancel_bios(_disk: *mut gendisk) {}

#[cfg(feature = "CONFIG_BLK_DEV_THROTTLING")]
extern "C" {
    pub fn blk_throtl_exit(disk: *mut gendisk);
    pub fn __blk_throtl_bio(bio: *mut bio) -> bool;
    pub fn blk_throtl_cancel_bios(disk: *mut gendisk);
}

#[cfg(feature = "CONFIG_BLK_DEV_THROTTLING")]
#[inline]
pub unsafe fn blk_throtl_activated(q: *mut request_queue) -> bool {
    !(*q).td.is_null() && blkcg_policy_enabled(q, &raw mut blkcg_policy_throtl)
}

#[cfg(feature = "CONFIG_BLK_DEV_THROTTLING")]
#[inline]
pub unsafe fn blk_should_throtl(bio: *mut bio) -> bool {
    let tg = blkg_to_tg((*bio).bi_blkg);
    let rw = bio_data_dir(bio) as usize;

    if !blk_throtl_activated((*(*bio).bi_bdev).bd_queue) {
        return false;
    }
    if !cgroup_subsys_on_dfl(io_cgrp_subsys) {
        if !bio_flagged(bio, BIO_CGROUP_ACCT) {
            bio_set_flag(bio, BIO_CGROUP_ACCT);
            blkg_rwstat_add(&mut (*tg).stat_bytes, (*bio).bi_opf, (*bio).bi_iter.bi_size);
        }
        blkg_rwstat_add(&mut (*tg).stat_ios, (*bio).bi_opf, 1);
    }
    if (*tg).has_rules_iops[rw] { return true; }
    if (*tg).has_rules_bps[rw] && !bio_flagged(bio, BIO_BPS_THROTTLED) { return true; }
    false
}

#[cfg(feature = "CONFIG_BLK_DEV_THROTTLING")]
#[inline]
pub unsafe fn blk_throtl_bio(bio: *mut bio) -> bool {
    if !blk_should_throtl(bio) { return false; }
    __blk_throtl_bio(bio)
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
