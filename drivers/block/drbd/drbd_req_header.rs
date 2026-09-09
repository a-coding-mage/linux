/* SPDX-License-Identifier: GPL-2.0-only */
/*
   drbd_req.h

   This file is part of DRBD by Philipp Reisner and Lars Ellenberg.

   Copyright (C) 2006-2008, LINBIT Information Technologies GmbH.
   Copyright (C) 2006-2008, Lars Ellenberg <lars.ellenberg@linbit.com>.
   Copyright (C) 2006-2008, Philipp Reisner <philipp.reisner@linbit.com>.
 */

/* C dependencies supplied by other translation units are intentionally not
 * implemented here. */

#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum drbd_req_event {
    CREATED,
    TO_BE_SENT,
    TO_BE_SUBMITTED,
    QUEUE_FOR_NET_WRITE,
    QUEUE_FOR_NET_READ,
    QUEUE_FOR_SEND_OOS,
    QUEUE_AS_DRBD_BARRIER,
    SEND_CANCELED,
    SEND_FAILED,
    HANDED_OVER_TO_NETWORK,
    OOS_HANDED_TO_NETWORK,
    CONNECTION_LOST_WHILE_PENDING,
    READ_RETRY_REMOTE_CANCELED,
    RECV_ACKED_BY_PEER,
    WRITE_ACKED_BY_PEER,
    WRITE_ACKED_BY_PEER_AND_SIS,
    CONFLICT_RESOLVED,
    POSTPONE_WRITE,
    NEG_ACKED,
    BARRIER_ACKED,
    DATA_RECEIVED,
    COMPLETED_OK,
    READ_COMPLETED_WITH_ERROR,
    READ_AHEAD_COMPLETED_WITH_ERROR,
    WRITE_COMPLETED_WITH_ERROR,
    DISCARD_COMPLETED_NOTSUPP,
    DISCARD_COMPLETED_WITH_ERROR,
    ABORT_DISK_IO,
    RESEND,
    FAIL_FROZEN_DISK_IO,
    RESTART_FROZEN_DISK_IO,
    NOTHING,
}

#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum drbd_req_state_bits {
    __RQ_LOCAL_PENDING,
    __RQ_LOCAL_COMPLETED,
    __RQ_LOCAL_OK,
    __RQ_LOCAL_ABORTED,
    __RQ_NET_PENDING,
    __RQ_NET_QUEUED,
    __RQ_NET_SENT,
    __RQ_NET_DONE,
    __RQ_NET_OK,
    __RQ_NET_SIS,
    __RQ_NET_MAX,
    __RQ_WRITE,
    __RQ_WSAME,
    __RQ_UNMAP,
    __RQ_ZEROES,
    __RQ_IN_ACT_LOG,
    __RQ_UNPLUG,
    __RQ_POSTPONED,
    __RQ_COMPLETION_SUSP,
    __RQ_EXP_RECEIVE_ACK,
    __RQ_EXP_WRITE_ACK,
    __RQ_EXP_BARR_ACK,
}

pub const RQ_LOCAL_PENDING: usize = 1usize << (__RQ_LOCAL_PENDING as usize);
pub const RQ_LOCAL_COMPLETED: usize = 1usize << (__RQ_LOCAL_COMPLETED as usize);
pub const RQ_LOCAL_OK: usize = 1usize << (__RQ_LOCAL_OK as usize);
pub const RQ_LOCAL_ABORTED: usize = 1usize << (__RQ_LOCAL_ABORTED as usize);
pub const RQ_LOCAL_MASK: usize = (RQ_LOCAL_ABORTED << 1).wrapping_sub(1);

pub const RQ_NET_PENDING: usize = 1usize << (__RQ_NET_PENDING as usize);
pub const RQ_NET_QUEUED: usize = 1usize << (__RQ_NET_QUEUED as usize);
pub const RQ_NET_SENT: usize = 1usize << (__RQ_NET_SENT as usize);
pub const RQ_NET_DONE: usize = 1usize << (__RQ_NET_DONE as usize);
pub const RQ_NET_OK: usize = 1usize << (__RQ_NET_OK as usize);
pub const RQ_NET_SIS: usize = 1usize << (__RQ_NET_SIS as usize);
pub const RQ_NET_MASK: usize = ((1usize << (__RQ_NET_MAX as usize)).wrapping_sub(1)) & !RQ_LOCAL_MASK;

pub const RQ_WRITE: usize = 1usize << (__RQ_WRITE as usize);
pub const RQ_WSAME: usize = 1usize << (__RQ_WSAME as usize);
pub const RQ_UNMAP: usize = 1usize << (__RQ_UNMAP as usize);
pub const RQ_ZEROES: usize = 1usize << (__RQ_ZEROES as usize);
pub const RQ_IN_ACT_LOG: usize = 1usize << (__RQ_IN_ACT_LOG as usize);
pub const RQ_UNPLUG: usize = 1usize << (__RQ_UNPLUG as usize);
pub const RQ_POSTPONED: usize = 1usize << (__RQ_POSTPONED as usize);
pub const RQ_COMPLETION_SUSP: usize = 1usize << (__RQ_COMPLETION_SUSP as usize);
pub const RQ_EXP_RECEIVE_ACK: usize = 1usize << (__RQ_EXP_RECEIVE_ACK as usize);
pub const RQ_EXP_WRITE_ACK: usize = 1usize << (__RQ_EXP_WRITE_ACK as usize);
pub const RQ_EXP_BARR_ACK: usize = 1usize << (__RQ_EXP_BARR_ACK as usize);

pub const MR_WRITE: i32 = 1;
pub const MR_READ: i32 = 2;

#[repr(C)]
pub struct bio_and_error {
    pub bio: *mut bio,
    pub error: i32,
}

extern "C" {
    pub fn start_new_tl_epoch(connection: *mut drbd_connection);
    pub fn drbd_req_destroy(kref: *mut kref);
    pub fn __req_mod(req: *mut drbd_request, what: drbd_req_event,
        peer_device: *mut drbd_peer_device, m: *mut bio_and_error) -> i32;
    pub fn complete_master_bio(device: *mut drbd_device, m: *mut bio_and_error);
    pub fn request_timer_fn(t: *mut timer_list);
    pub fn tl_restart(connection: *mut drbd_connection, what: drbd_req_event);
    pub fn _tl_restart(connection: *mut drbd_connection, what: drbd_req_event);
    pub fn tl_abort_disk_io(device: *mut drbd_device);
    pub fn drbd_restart_request(req: *mut drbd_request);
    pub fn spin_lock_irqsave(lock: *mut spinlock_t, flags: *mut usize);
    pub fn spin_unlock_irqrestore(lock: *mut spinlock_t, flags: usize);
    pub fn drbd_should_do_remote(state: drbd_dev_state) -> bool;
}

#[inline]
pub unsafe fn _req_mod(req: *mut drbd_request, what: drbd_req_event,
    peer_device: *mut drbd_peer_device) -> i32 {
    let device = (*req).device;
    let mut m = bio_and_error { bio: core::ptr::null_mut(), error: 0 };
    let rv = __req_mod(req, what, peer_device, &mut m);
    if !m.bio.is_null() {
        complete_master_bio(device, &mut m);
    }
    rv
}

#[inline]
pub unsafe fn req_mod(req: *mut drbd_request, what: drbd_req_event,
    peer_device: *mut drbd_peer_device) -> i32 {
    let mut flags: usize = 0;
    let device = (*req).device;
    let mut m = bio_and_error { bio: core::ptr::null_mut(), error: 0 };
    spin_lock_irqsave((*(*device).resource).req_lock, &mut flags);
    let rv = __req_mod(req, what, peer_device, &mut m);
    spin_unlock_irqrestore((*(*device).resource).req_lock, flags);
    if !m.bio.is_null() {
        complete_master_bio(device, &mut m);
    }
    rv
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
