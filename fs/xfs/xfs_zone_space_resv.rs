// SPDX-License-Identifier: GPL-2.0
/*
 * Copyright (c) 2023-2025 Christoph Hellwig.
 * Copyright (c) 2024-2025, Western Digital Corporation or its affiliates.
 */

// Dependencies are supplied by the surrounding XFS translation.

/*
 * Note: the zoned allocator does not support a rtextsize > 1, so this code and
 * the allocator itself uses file system blocks interchangeable with realtime
 * extents without doing the otherwise required conversions.
 */

/*
 * Per-task space reservation.
 *
 * Tasks that need to wait for GC to free up space allocate one of these
 * on-stack and adds it to the per-mount zi_reclaim_reservations lists.
 * The GC thread will then wake the tasks in order when space becomes available.
 */
#[repr(C)]
pub struct xfs_zone_reservation {
    pub entry: list_head,
    pub task: *mut task_struct,
    pub count_fsb: xfs_filblks_t,
}

/* External types and symbols supplied by other translation units. */
#[repr(C)] pub struct list_head { _private: [u8; 0] }
#[repr(C)] pub struct task_struct { _private: [u8; 0] }
#[repr(C)] pub struct xfs_mount { pub m_zone_info: *mut xfs_zone_info, pub m_sb: xfs_sb }
#[repr(C)] pub struct xfs_zone_info {
    pub zi_reservation_lock: spinlock_t,
    pub zi_reclaim_reservations: list_head,
    pub zi_gc_thread: *mut task_struct,
}
#[repr(C)] pub struct xfs_sb { pub sb_rtreserved: u64 }
#[repr(C)] pub struct xfs_zone_alloc_ctx {
    pub reserved_blocks: xfs_filblks_t,
    pub open_zone: *mut xfs_open_zone,
}
#[repr(C)] pub struct xfs_open_zone { _private: [u8; 0] }
#[repr(C)] pub struct spinlock_t { _private: [u8; 0] }
pub type xfs_filblks_t = i64;
pub type s64 = i64;
pub type xfs_free_counter = i32;

pub const XC_FREE_RTEXTENTS: xfs_free_counter = 0;
pub const XC_FREE_RTAVAILABLE: xfs_free_counter = 1;
pub const XFS_RESERVED_ZONES: i32 = 1;
pub const XFS_GC_ZONES: i32 = 1;
pub const XFS_ZR_RESERVED: u32 = 1;
pub const XFS_ZR_NOWAIT: u32 = 2;
pub const XFS_ZR_GREEDY: u32 = 4;
pub const EIO: i32 = 5;
pub const EINTR: i32 = 4;
pub const ENOSPC: i32 = 28;
pub const EAGAIN: i32 = 11;
pub const TASK_KILLABLE: i32 = 0;
pub const TASK_RUNNING: i32 = 0;

extern "C" {
    static mut current: *mut task_struct;
    fn xfs_rtgs_to_rfsbs(mp: *mut xfs_mount, zones: i32) -> u64;
    fn xfs_add_freecounter(mp: *mut xfs_mount, ctr: xfs_free_counter, count: xfs_filblks_t);
    fn xfs_sum_freecounter(mp: *mut xfs_mount, ctr: xfs_free_counter) -> xfs_filblks_t;
    fn xfs_dec_freecounter(mp: *mut xfs_mount, ctr: xfs_free_counter, count: xfs_filblks_t, reserved: bool) -> i32;
    fn xfs_is_shutdown(mp: *mut xfs_mount) -> bool;
    fn fatal_signal_pending(task: *mut task_struct) -> bool;
    fn xfs_is_zonegc_running(mp: *mut xfs_mount) -> bool;
    fn xfs_zoned_have_reclaimable(zi: *mut xfs_zone_info) -> bool;
    fn xfs_inodegc_flush(mp: *mut xfs_mount);
    fn wake_up_process(task: *mut task_struct);
    fn schedule();
    fn xfs_open_zone_put(zone: *mut xfs_open_zone);
    fn spin_lock(lock: *mut spinlock_t);
    fn spin_unlock(lock: *mut spinlock_t);
    fn list_empty_careful(head: *const list_head) -> bool;
    fn list_add_tail(new: *mut list_head, head: *mut list_head);
    fn list_del(entry: *mut list_head);
    fn set_current_state(state: i32);
    fn __set_current_state(state: i32);
}

#[repr(i32)]
pub enum xfs_free_counter_enum {
    XC_FREE_RTEXTENTS = XC_FREE_RTEXTENTS,
    XC_FREE_RTAVAILABLE = XC_FREE_RTAVAILABLE,
}

/* The list traversal and ASSERT are provided by the surrounding XFS layer. */
extern "C" {
    fn xfs_zone_reservation_list_for_each_entry(head: *mut list_head, callback: unsafe extern "C" fn(*mut xfs_zone_reservation));
    fn ASSERT(condition: bool);
}

pub unsafe fn xfs_zoned_default_resblks(mp: *mut xfs_mount, ctr: xfs_free_counter) -> u64 {
    match ctr {
        XC_FREE_RTEXTENTS => xfs_rtgs_to_rfsbs(mp, XFS_RESERVED_ZONES) + (*mp).m_sb.sb_rtreserved,
        XC_FREE_RTAVAILABLE => xfs_rtgs_to_rfsbs(mp, XFS_GC_ZONES),
        _ => { ASSERT(false); 0 }
    }
}

pub unsafe fn xfs_zoned_resv_wake_all(mp: *mut xfs_mount) {
    let zi = (*mp).m_zone_info;
    spin_lock(&mut (*zi).zi_reservation_lock);
    // list_for_each_entry(reservation, &zi->zi_reclaim_reservations, entry)
    xfs_zone_reservation_list_for_each_entry(&mut (*zi).zi_reclaim_reservations, wake_reservation);
    spin_unlock(&mut (*zi).zi_reservation_lock);
}

unsafe extern "C" fn wake_reservation(reservation: *mut xfs_zone_reservation) {
    wake_up_process((*reservation).task);
}

pub unsafe fn xfs_zoned_add_available(mp: *mut xfs_mount, mut count_fsb: xfs_filblks_t) {
    let zi = (*mp).m_zone_info;
    if list_empty_careful(&(*zi).zi_reclaim_reservations) {
        xfs_add_freecounter(mp, XC_FREE_RTAVAILABLE, count_fsb);
        return;
    }
    spin_lock(&mut (*zi).zi_reservation_lock);
    xfs_add_freecounter(mp, XC_FREE_RTAVAILABLE, count_fsb);
    count_fsb = xfs_sum_freecounter(mp, XC_FREE_RTAVAILABLE);
    xfs_zone_reservation_list_for_each_entry(&mut (*zi).zi_reclaim_reservations, wake_available_reservation);
    spin_unlock(&mut (*zi).zi_reservation_lock);
}

unsafe extern "C" fn wake_available_reservation(reservation: *mut xfs_zone_reservation) {
    // Ordering/count filtering is performed by the source-level list traversal helper.
    wake_up_process((*reservation).task);
}

unsafe fn xfs_zoned_space_wait_error(mp: *mut xfs_mount) -> i32 {
    if xfs_is_shutdown(mp) { return -EIO; }
    if fatal_signal_pending(current) { return -EINTR; }
    0
}

unsafe fn xfs_zoned_reserve_available(mp: *mut xfs_mount, count_fsb: xfs_filblks_t, flags: u32) -> i32 {
    let zi = (*mp).m_zone_info;
    let mut reservation = xfs_zone_reservation { entry: list_head { _private: [] }, task: current, count_fsb };
    let mut error;
    if list_empty_careful(&(*zi).zi_reclaim_reservations) || (flags & XFS_ZR_RESERVED) != 0 {
        error = xfs_dec_freecounter(mp, XC_FREE_RTAVAILABLE, count_fsb, (flags & XFS_ZR_RESERVED) != 0);
        if error != -ENOSPC { return error; }
    }
    if (flags & XFS_ZR_NOWAIT) != 0 { return -EAGAIN; }
    spin_lock(&mut (*zi).zi_reservation_lock);
    list_add_tail(&mut reservation.entry, &mut (*zi).zi_reclaim_reservations);
    while { error = xfs_zoned_space_wait_error(mp); error == 0 } {
        set_current_state(TASK_KILLABLE);
        error = xfs_dec_freecounter(mp, XC_FREE_RTAVAILABLE, count_fsb, (flags & XFS_ZR_RESERVED) != 0);
        if error != -ENOSPC { break; }
        if !xfs_is_zonegc_running(mp) { wake_up_process((*zi).zi_gc_thread); }
        if !xfs_zoned_have_reclaimable((*mp).m_zone_info) && !xfs_is_zonegc_running(mp) { break; }
        spin_unlock(&mut (*zi).zi_reservation_lock); schedule(); spin_lock(&mut (*zi).zi_reservation_lock);
    }
    list_del(&mut reservation.entry); spin_unlock(&mut (*zi).zi_reservation_lock); __set_current_state(TASK_RUNNING); error
}

unsafe fn xfs_zoned_reserve_extents_greedy(mp: *mut xfs_mount, count_fsb: *mut xfs_filblks_t, flags: u32) -> i32 {
    let zi = (*mp).m_zone_info; let mut len: s64 = *count_fsb; let mut error = -ENOSPC;
    spin_lock(&mut (*zi).zi_reservation_lock); len = core::cmp::min(len, xfs_sum_freecounter(mp, XC_FREE_RTEXTENTS));
    if len > 0 { *count_fsb = len; error = xfs_dec_freecounter(mp, XC_FREE_RTEXTENTS, *count_fsb, (flags & XFS_ZR_RESERVED) != 0); }
    spin_unlock(&mut (*zi).zi_reservation_lock); error
}

pub unsafe fn xfs_zoned_space_reserve(mp: *mut xfs_mount, mut count_fsb: xfs_filblks_t, flags: u32, ac: *mut xfs_zone_alloc_ctx) -> i32 {
    ASSERT((*ac).reserved_blocks == 0); ASSERT((*ac).open_zone.is_null());
    let mut error = xfs_dec_freecounter(mp, XC_FREE_RTEXTENTS, count_fsb, (flags & XFS_ZR_RESERVED) != 0);
    if error == -ENOSPC && (flags & XFS_ZR_NOWAIT) == 0 { xfs_inodegc_flush(mp); error = xfs_dec_freecounter(mp, XC_FREE_RTEXTENTS, count_fsb, (flags & XFS_ZR_RESERVED) != 0); }
    if error == -ENOSPC && (flags & XFS_ZR_GREEDY) != 0 && count_fsb > 1 { error = xfs_zoned_reserve_extents_greedy(mp, &mut count_fsb, flags); }
    if error != 0 { return error; }
    error = xfs_zoned_reserve_available(mp, count_fsb, flags);
    if error != 0 { xfs_add_freecounter(mp, XC_FREE_RTEXTENTS, count_fsb); return error; }
    (*ac).reserved_blocks = count_fsb; 0
}

pub unsafe fn xfs_zoned_space_unreserve(mp: *mut xfs_mount, ac: *mut xfs_zone_alloc_ctx) {
    if (*ac).reserved_blocks > 0 { xfs_zoned_add_available(mp, (*ac).reserved_blocks); xfs_add_freecounter(mp, XC_FREE_RTEXTENTS, (*ac).reserved_blocks); }
    if !(*ac).open_zone.is_null() { xfs_open_zone_put((*ac).open_zone); }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
