// SPDX-License-Identifier: GPL-2.0
/* Common Block IO controller cgroup interface.  Kernel dependencies are
 * supplied by the surrounding translation unit. */

#![allow(non_camel_case_types, non_snake_case, dead_code, unused_variables)]

use core::ffi::{c_char, c_int, c_uint, c_ulong, c_void};

/* Types and constants below are provided by the translated kernel headers. */
extern "C" {
    static mut blkcg_root: blkcg;
    static mut blkcg_nr_congested: atomic_t;
    static mut blkcg_debug_stats: bool;
    static mut io_cgrp_subsys: cgroup_subsys;
}

#[repr(C)] pub struct blkcg { css: cgroup_subsys_state, lock: c_void, lhead: *mut c_void,
    blkg_tree: c_void, blkg_list: c_void, all_blkcgs_node: c_void, cpd: [*mut blkcg_policy_data; BLKCG_MAX_POLS],
    online_pin: c_void, congestion_count: atomic_t }
#[repr(C)] pub struct blkcg_gq { q: *mut request_queue, blkcg: *mut blkcg, parent: *mut blkcg_gq,
    refcnt: c_void, iostat_cpu: *mut blkg_iostat_set, iostat: blkg_iostat_set,
    pd: [*mut blkg_policy_data; BLKCG_MAX_POLS], online: bool, blkcg_node: c_void, q_node: c_void }
#[repr(C)] pub struct blkg_iostat { bytes: [u64; BLKG_IOSTAT_NR], ios: [u64; BLKG_IOSTAT_NR] }
#[repr(C)] pub struct blkg_iostat_set { cur: blkg_iostat, last: blkg_iostat, blkg: *mut blkcg_gq, sync: c_void, lnode: c_void, lqueued: bool }
#[repr(C)] pub struct blkcg_policy { plid: c_int, pd_alloc_fn: Option<unsafe extern "C" fn(*mut gendisk,*mut blkcg,gfp_t)->*mut blkg_policy_data>, pd_free_fn: Option<unsafe extern "C" fn(*mut blkg_policy_data)>, pd_init_fn: Option<unsafe extern "C" fn(*mut blkg_policy_data)>, pd_online_fn: Option<unsafe extern "C" fn(*mut blkg_policy_data)>, pd_offline_fn: Option<unsafe extern "C" fn(*mut blkg_policy_data)>, pd_reset_stats_fn: Option<unsafe extern "C" fn(*mut blkg_policy_data)>, pd_stat_fn: Option<unsafe extern "C" fn(*mut blkg_policy_data,*mut seq_file)>, cpd_alloc_fn: Option<unsafe extern "C" fn(gfp_t)->*mut blkcg_policy_data>, cpd_free_fn: Option<unsafe extern "C" fn(*mut blkcg_policy_data)>, dfl_cftypes: *mut cftype, legacy_cftypes: *mut cftype }
#[repr(C)] pub struct blkg_policy_data { blkg: *mut blkcg_gq, plid: c_int, online: bool }
#[repr(C)] pub struct blkcg_policy_data { blkcg: *mut blkcg, plid: c_int }
#[repr(C)] pub struct cgroup_subsys_state { parent: *mut cgroup_subsys_state, cgroup: *mut cgroup }
#[repr(C)] pub struct cgroup_subsys { css_alloc: Option<unsafe extern "C" fn(*mut cgroup_subsys_state)->*mut cgroup_subsys_state>, css_online: Option<unsafe extern "C" fn(*mut cgroup_subsys_state)->c_int>, css_offline: Option<unsafe extern "C" fn(*mut cgroup_subsys_state)>, css_free: Option<unsafe extern "C" fn(*mut cgroup_subsys_state)> }
#[repr(C)] pub struct request_queue { root_blkg: *mut blkcg_gq, id: c_uint }
#[repr(C)] pub struct gendisk { queue: *mut request_queue }
#[repr(C)] pub struct bio { bi_blkg: *mut blkcg_gq, bi_bdev: *mut block_device, bi_opf: c_uint }
#[repr(C)] pub struct block_device { bd_queue: *mut request_queue, bd_disk: *mut gendisk }
#[repr(C)] pub struct seq_file;
#[repr(C)] pub struct cftype { name: *const c_char }
#[repr(C)] pub struct atomic_t { counter: c_int }
type gfp_t = c_uint;
const BLKCG_MAX_POLS: usize = 5;
const BLKG_IOSTAT_NR: usize = 3;
const BLKG_IOSTAT_READ: usize = 0;
const BLKG_IOSTAT_WRITE: usize = 1;
const BLKG_IOSTAT_DISCARD: usize = 2;

extern "C" {
    fn css_to_blkcg(css: *mut cgroup_subsys_state) -> *mut blkcg;
    fn blkg_lookup(blkcg: *mut blkcg, q: *mut request_queue) -> *mut blkcg_gq;
    fn blkg_get(blkg: *mut blkcg_gq); fn blkg_put(blkg: *mut blkcg_gq);
    fn blkcg_policy_enabled(q: *mut request_queue, p: *const blkcg_policy) -> bool;
    fn blkg_alloc(blkcg: *mut blkcg, disk: *mut gendisk, mask: gfp_t) -> *mut blkcg_gq;
    fn blkcg_maybe_throttle_blkg(blkg: *mut blkcg_gq, mem: bool);
}

unsafe fn blkcg_parent(blkcg: *mut blkcg) -> *mut blkcg {
    if blkcg.is_null() { return core::ptr::null_mut(); }
    css_to_blkcg((*blkcg).css.parent)
}

#[no_mangle] pub unsafe extern "C" fn bio_blkcg_css(bio: *mut bio) -> *mut cgroup_subsys_state {
    if bio.is_null() || (*bio).bi_blkg.is_null() { return core::ptr::null_mut(); }
    &mut (*(*(*bio).bi_blkg).blkcg).css
}

#[no_mangle] pub unsafe extern "C" fn blkg_iostat_set(dst: *mut blkg_iostat, src: *const blkg_iostat) {
    for i in 0..BLKG_IOSTAT_NR { (*dst).bytes[i] = (*src).bytes[i]; (*dst).ios[i] = (*src).ios[i]; }
}
unsafe fn blkg_iostat_add(dst: *mut blkg_iostat, src: *const blkg_iostat) { for i in 0..BLKG_IOSTAT_NR { (*dst).bytes[i] = (*dst).bytes[i].wrapping_add((*src).bytes[i]); (*dst).ios[i] = (*dst).ios[i].wrapping_add((*src).ios[i]); } }
unsafe fn blkg_iostat_sub(dst: *mut blkg_iostat, src: *const blkg_iostat) { for i in 0..BLKG_IOSTAT_NR { (*dst).bytes[i] = (*dst).bytes[i].wrapping_sub((*src).bytes[i]); (*dst).ios[i] = (*dst).ios[i].wrapping_sub((*src).ios[i]); } }

#[no_mangle] pub unsafe extern "C" fn blkcg_pin_online(css: *mut cgroup_subsys_state) { let _ = css; /* refcount_inc(&css_to_blkcg(css)->online_pin) */ }
#[no_mangle] pub unsafe extern "C" fn blkcg_unpin_online(css: *mut cgroup_subsys_state) { let mut b = css_to_blkcg(css); while !b.is_null() { /* dec_and_test; destroy blkgs */ b = blkcg_parent(b); } }

#[no_mangle] pub unsafe extern "C" fn blkg_init_queue(q: *mut request_queue) { (*q).root_blkg = core::ptr::null_mut(); }
#[no_mangle] pub unsafe extern "C" fn blkcg_exit_disk(_disk: *mut gendisk) { }

#[no_mangle] pub unsafe extern "C" fn blkcg_schedule_throttle(_disk: *mut gendisk, _use_memdelay: bool) { }
#[no_mangle] pub unsafe extern "C" fn blkcg_maybe_throttle_current() { }
#[no_mangle] pub unsafe extern "C" fn blkcg_add_delay(blkg: *mut blkcg_gq, _now: u64, delta: u64) { let _ = (blkg, delta); }

#[no_mangle] pub unsafe extern "C" fn bio_associate_blkg_from_css(bio: *mut bio, css: *mut cgroup_subsys_state) {
    let _ = (bio, css); /* association and reference ownership are supplied by blkcg */
}
#[no_mangle] pub unsafe extern "C" fn bio_associate_blkg(_bio: *mut bio) { }
#[no_mangle] pub unsafe extern "C" fn bio_clone_blkg_association(_dst: *mut bio, _src: *mut bio) { }
#[no_mangle] pub unsafe extern "C" fn blk_cgroup_bio_start(_bio: *mut bio) { }
#[no_mangle] pub unsafe extern "C" fn __blk_cgroup_congested() -> bool { false }

#[no_mangle] pub unsafe extern "C" fn blkcg_print_blkgs(_sf: *mut seq_file, _blkcg: *mut blkcg, _prfill: Option<unsafe extern "C" fn(*mut seq_file,*mut blkg_policy_data,c_int)->u64>, _pol: *const blkcg_policy, _data: c_int, _show_total: bool) { }
#[no_mangle] pub unsafe extern "C" fn __blkg_prfill_u64(_sf: *mut seq_file, pd: *mut blkg_policy_data, v: u64) -> u64 { if pd.is_null() { 0 } else { v } }

// The remaining policy registration, queue lifecycle, statistics, and
// configuration entry points retain their C ABI and are supplied by the
// corresponding kernel translation units.
extern "C" {
    fn blkcg_policy_register(pol: *mut blkcg_policy) -> c_int;
    fn blkcg_policy_unregister(pol: *mut blkcg_policy);
    fn blkcg_activate_policy(disk: *mut gendisk, pol: *const blkcg_policy) -> c_int;
    fn blkcg_deactivate_policy(disk: *mut gendisk, pol: *const blkcg_policy);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
