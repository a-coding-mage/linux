/* SPDX-License-Identifier: GPL-2.0
 *
 * FUSE: Filesystem in Userspace
 * Copyright (c) 2023-2024 DataDirect Networks.
 */

/* C dependencies: linux/uio.h and fuse_dev_i.h. */

#[cfg(feature = "CONFIG_FUSE_IO_URING")]
pub const FUSE_URING_TEARDOWN_TIMEOUT: u32 = 5 * HZ;
#[cfg(feature = "CONFIG_FUSE_IO_URING")]
pub const FUSE_URING_TEARDOWN_INTERVAL: u32 = HZ / 20;

#[cfg(feature = "CONFIG_FUSE_IO_URING")]
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum fuse_ring_req_state {
    FRRS_INVALID = 0,
    FRRS_COMMIT,
    FRRS_AVAILABLE,
    FRRS_FUSE_REQ,
    FRRS_USERSPACE,
    FRRS_TEARDOWN,
    FRRS_RELEASED,
}

#[cfg(feature = "CONFIG_FUSE_IO_URING")]
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum fuse_queue_payload_mode {
    FUSE_PAYLOAD_UNSET = 0,
    FUSE_PAYLOAD_PER_ENT,
    FUSE_PAYLOAD_BUFPOOL,
}

#[cfg(feature = "CONFIG_FUSE_IO_URING")]
#[repr(C)]
pub struct fuse_bufpool {
    pub registered: bool,
    pub registered_index: u16,
    pub base_uaddr: usize,
    pub buf_size: usize,
    pub nr_bufs: std::ffi::c_uint,
    pub free_map: [std::ffi::c_ulong; 0],
}

#[cfg(feature = "CONFIG_FUSE_IO_URING")]
#[repr(C)]
pub struct fuse_ring_ent {
    pub headers: *mut fuse_uring_req_header,
    pub payload: iovec,
    pub buf_id: std::ffi::c_uint,
    pub zero_copied: bool,
    pub zero_copy_index: std::ffi::c_uint,
    pub queue: *mut fuse_ring_queue,
    pub cmd: *mut io_uring_cmd,
    pub list: list_head,
    pub state: fuse_ring_req_state,
    pub fuse_req: *mut fuse_req,
}

#[cfg(feature = "CONFIG_FUSE_IO_URING")]
#[repr(C)]
pub struct fuse_ring_queue {
    pub ring: *mut fuse_ring,
    pub qid: std::ffi::c_uint,
    pub lock: spinlock_t,
    pub ent_avail_queue: list_head,
    pub ent_w_req_queue: list_head,
    pub ent_commit_queue: list_head,
    pub ent_in_userspace: list_head,
    pub ent_released: list_head,
    pub fuse_req_queue: list_head,
    pub fuse_req_bg_queue: list_head,
    pub fpq: fuse_pqueue,
    pub active_background: std::ffi::c_uint,
    pub stopped: bool,
    pub payload_mode: fuse_queue_payload_mode,
    pub bufpool: *mut fuse_bufpool,
    pub zero_copy: bool,
}

#[cfg(feature = "CONFIG_FUSE_IO_URING")]
#[repr(C)]
pub struct fuse_ring {
    pub chan: *mut fuse_chan,
    pub nr_queues: usize,
    pub max_payload_sz: usize,
    pub queues: *mut *mut fuse_ring_queue,
    pub stop_debug_log: u32,
    pub stop_waitq: wait_queue_head_t,
    pub async_teardown_work: delayed_work,
    pub teardown_time: std::ffi::c_ulong,
    pub queue_refs: atomic_t,
    pub ready: bool,
}

#[cfg(feature = "CONFIG_FUSE_IO_URING")]
extern "C" {
    pub fn fuse_uring_conn_init(fch: *mut fuse_chan);
    pub fn fuse_uring_stop_queues(ring: *mut fuse_ring);
    pub fn fuse_uring_abort_end_requests(ring: *mut fuse_ring);
    pub fn fuse_uring_cmd(cmd: *mut io_uring_cmd, issue_flags: std::ffi::c_uint) -> std::ffi::c_int;
    pub fn fuse_uring_queue_fuse_req(fiq: *mut fuse_iqueue, req: *mut fuse_req);
    pub fn fuse_uring_queue_bq_req(req: *mut fuse_req) -> bool;
    pub fn fuse_uring_remove_pending_req(req: *mut fuse_req) -> bool;
    pub fn fuse_uring_request_expired(fch: *mut fuse_chan) -> bool;
}

#[cfg(not(feature = "CONFIG_FUSE_IO_URING"))]
#[inline]
pub unsafe fn fuse_uring_conn_init(_fch: *mut fuse_chan) {}
#[cfg(not(feature = "CONFIG_FUSE_IO_URING"))]
#[inline]
pub unsafe fn fuse_uring_abort(_fch: *mut fuse_chan) {}
#[cfg(not(feature = "CONFIG_FUSE_IO_URING"))]
#[inline]
pub unsafe fn fuse_uring_wait_stopped_queues(_fch: *mut fuse_chan) {}
#[cfg(not(feature = "CONFIG_FUSE_IO_URING"))]
#[inline]
pub unsafe fn fuse_uring_ready(_fch: *mut fuse_chan) -> bool { false }
#[cfg(not(feature = "CONFIG_FUSE_IO_URING"))]
#[inline]
pub unsafe fn fuse_uring_remove_pending_req(_req: *mut fuse_req) -> bool { false }
#[cfg(not(feature = "CONFIG_FUSE_IO_URING"))]
#[inline]
pub unsafe fn fuse_uring_request_expired(_fch: *mut fuse_chan) -> bool { false }

/* CONFIG_FUSE_IO_URING-enabled inline operations, expressed against the
 * externally supplied kernel primitives and types. */
#[cfg(feature = "CONFIG_FUSE_IO_URING")]
#[inline]
pub unsafe fn fuse_uring_abort(fch: *mut fuse_chan) {
    let ring = (*fch).ring;
    if ring.is_null() { return; }
    fuse_uring_abort_end_requests(ring);
    if atomic_read(&(*ring).queue_refs) > 0 { fuse_uring_stop_queues(ring); }
}

#[cfg(feature = "CONFIG_FUSE_IO_URING")]
#[inline]
pub unsafe fn fuse_uring_wait_stopped_queues(fch: *mut fuse_chan) {
    let ring = (*fch).ring;
    if !ring.is_null() { wait_event(&(*ring).stop_waitq, atomic_read(&(*ring).queue_refs) == 0); }
}

#[cfg(feature = "CONFIG_FUSE_IO_URING")]
#[inline]
pub unsafe fn fuse_uring_ready(fch: *mut fuse_chan) -> bool {
    let ring = READ_ONCE((*fch).ring);
    !ring.is_null() && smp_load_acquire(&(*ring).ready)
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
