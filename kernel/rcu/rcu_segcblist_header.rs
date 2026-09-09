/* SPDX-License-Identifier: GPL-2.0+ */
/*
 * RCU segmented callback lists, internal-to-rcu header file
 *
 * Copyright IBM Corporation, 2017
 *
 * Authors: Paul E. McKenney <paulmck@linux.ibm.com>
 */

/* Dependency supplied by the surrounding kernel translation. */

#[inline]
pub unsafe fn rcu_cblist_n_cbs(rclp: *mut rcu_cblist) -> core::ffi::c_long {
    core::ptr::read_volatile(&(*rclp).len)
}

extern "C" {
    pub fn rcu_segcblist_get_seglen(
        rsclp: *mut rcu_segcblist,
        seg: core::ffi::c_int,
    ) -> core::ffi::c_long;
    pub fn rcu_segcblist_n_segment_cbs(rsclp: *mut rcu_segcblist) -> core::ffi::c_long;

    pub fn rcu_cblist_init(rclp: *mut rcu_cblist);
    pub fn rcu_cblist_enqueue(rclp: *mut rcu_cblist, rhp: *mut rcu_head);
    pub fn rcu_cblist_flush_enqueue(
        drclp: *mut rcu_cblist,
        srclp: *mut rcu_cblist,
        rhp: *mut rcu_head,
    );
    pub fn rcu_cblist_dequeue(rclp: *mut rcu_cblist) -> *mut rcu_head;
}

#[inline]
pub unsafe fn rcu_segcblist_empty(rsclp: *mut rcu_segcblist) -> bool {
    core::ptr::read_volatile(&(*rsclp).head).is_null()
}

#[inline]
pub unsafe fn rcu_segcblist_n_cbs(rsclp: *mut rcu_segcblist) -> core::ffi::c_long {
    /* CONFIG_RCU_NOCB_CPU selects an atomic read in the C implementation. */
    core::ptr::read_volatile(&(*rsclp).len)
}

#[inline]
pub unsafe fn rcu_segcblist_set_flags(rsclp: *mut rcu_segcblist, flags: core::ffi::c_int) {
    let old = core::ptr::read_volatile(&(*rsclp).flags);
    core::ptr::write_volatile(&mut (*rsclp).flags, old | flags);
}

#[inline]
pub unsafe fn rcu_segcblist_clear_flags(rsclp: *mut rcu_segcblist, flags: core::ffi::c_int) {
    let old = core::ptr::read_volatile(&(*rsclp).flags);
    core::ptr::write_volatile(&mut (*rsclp).flags, old & !flags);
}

#[inline]
pub unsafe fn rcu_segcblist_test_flags(
    rsclp: *mut rcu_segcblist,
    flags: core::ffi::c_int,
) -> bool {
    (core::ptr::read_volatile(&(*rsclp).flags) & flags) != 0
}

#[inline]
pub unsafe fn rcu_segcblist_is_enabled(rsclp: *mut rcu_segcblist) -> bool {
    rcu_segcblist_test_flags(rsclp, SEGCBLIST_ENABLED)
}

#[inline]
pub unsafe fn rcu_segcblist_is_offloaded(rsclp: *mut rcu_segcblist) -> bool {
    /* IS_ENABLED(CONFIG_RCU_NOCB_CPU) is a build-time condition. */
    rcu_segcblist_test_flags(rsclp, SEGCBLIST_OFFLOADED)
}

#[inline]
pub unsafe fn rcu_segcblist_restempty(rsclp: *mut rcu_segcblist, seg: core::ffi::c_int) -> bool {
    let tail = core::ptr::read_volatile(&(*rsclp).tails[seg as usize]);
    core::ptr::read_volatile(tail).is_null()
}

#[inline]
pub unsafe fn rcu_segcblist_segempty(rsclp: *mut rcu_segcblist, seg: core::ffi::c_int) -> bool {
    if seg == RCU_DONE_TAIL {
        return (&(*rsclp).head as *const _) == (*rsclp).tails[RCU_DONE_TAIL as usize];
    }
    (*rsclp).tails[(seg - 1) as usize] == (*rsclp).tails[seg as usize]
}

extern "C" {
    pub fn rcu_segcblist_inc_len(rsclp: *mut rcu_segcblist);
    pub fn rcu_segcblist_add_len(rsclp: *mut rcu_segcblist, v: core::ffi::c_long);
    pub fn rcu_segcblist_init(rsclp: *mut rcu_segcblist);
    pub fn rcu_segcblist_disable(rsclp: *mut rcu_segcblist);
    pub fn rcu_segcblist_ready_cbs(rsclp: *mut rcu_segcblist) -> bool;
    pub fn rcu_segcblist_pend_cbs(rsclp: *mut rcu_segcblist) -> bool;
    pub fn rcu_segcblist_first_cb(rsclp: *mut rcu_segcblist) -> *mut rcu_head;
    pub fn rcu_segcblist_first_pend_cb(rsclp: *mut rcu_segcblist) -> *mut rcu_head;
    pub fn rcu_segcblist_nextgp(rsclp: *mut rcu_segcblist, gsp: *mut rcu_gp_seq) -> bool;
    pub fn rcu_segcblist_enqueue(rsclp: *mut rcu_segcblist, rhp: *mut rcu_head);
    pub fn rcu_segcblist_entrain(rsclp: *mut rcu_segcblist, rhp: *mut rcu_head) -> bool;
    pub fn rcu_segcblist_extract_done_cbs(rsclp: *mut rcu_segcblist, rclp: *mut rcu_cblist);
    pub fn rcu_segcblist_extract_pend_cbs(rsclp: *mut rcu_segcblist, rclp: *mut rcu_cblist);
    pub fn rcu_segcblist_insert_count(rsclp: *mut rcu_segcblist, rclp: *mut rcu_cblist);
    pub fn rcu_segcblist_insert_done_cbs(rsclp: *mut rcu_segcblist, rclp: *mut rcu_cblist);
    pub fn rcu_segcblist_insert_pend_cbs(rsclp: *mut rcu_segcblist, rclp: *mut rcu_cblist);
    pub fn rcu_segcblist_advance(rsclp: *mut rcu_segcblist);
    pub fn rcu_segcblist_accelerate(rsclp: *mut rcu_segcblist, gsp: *mut rcu_gp_seq) -> bool;
    pub fn rcu_segcblist_merge(dst_rsclp: *mut rcu_segcblist, src_rsclp: *mut rcu_segcblist);
    pub fn srcu_segcblist_advance(rsclp: *mut rcu_segcblist, seq: core::ffi::c_ulong);
    pub fn srcu_segcblist_accelerate(rsclp: *mut rcu_segcblist, seq: core::ffi::c_ulong) -> bool;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
