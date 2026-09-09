// SPDX-License-Identifier: GPL-2.0

// Translated from zcrx.h. Kernel-provided types and constants are external
// dependencies supplied by the surrounding build.

pub const ZCRX_SUPPORTED_REG_FLAGS: u32 = ZCRX_REG_IMPORT | ZCRX_REG_NODEV;
pub const ZCRX_FEATURES: u32 = ZCRX_FEATURE_RX_PAGE_SIZE | ZCRX_FEATURE_EVENT;
pub const ZCRX_EVENT_TYPE_MASK: u32 = (1u32 << ZCRX_EVENT_ALLOC_FAIL) | (1u32 << ZCRX_EVENT_COPY);

#[repr(C)]
pub struct io_zcrx_mem {
    pub size: usize,
    pub is_dmabuf: bool,
    pub pages: *mut *mut page,
    pub nr_folios: usize,
    pub page_sg_table: sg_table,
    pub account_pages: usize,
    pub sgt: *mut sg_table,
    pub attach: *mut dma_buf_attachment,
    pub dmabuf: *mut dma_buf,
}

#[repr(C)]
pub struct io_zcrx_area {
    pub nia: net_iov_area,
    pub ifq: *mut io_zcrx_ifq,
    pub user_refs: *mut atomic_t,
    pub is_mapped: bool,
    pub area_id: u16,
    // freelist
    pub free_count: u32,
    pub freelist: *mut u32,
    pub mem: io_zcrx_mem,
}

#[repr(C)]
pub struct zcrx_rq_hdr {
    pub head: u32,
    pub tail: u32,
}

#[repr(C)]
pub struct zcrx_rq {
    pub lock: spinlock_t,
    pub ring: *mut zcrx_rq_hdr,
    pub rqes: *mut io_uring_zcrx_rqe,
    pub cached_head: u32,
    pub cached_tail: u32,
    pub nr_entries: u32,
}

#[repr(C)]
pub struct io_zcrx_ifq {
    // read-protected by any of: ->pp_lock, ->alloc_lock, ->rq.lock
    pub areas: *mut *mut io_zcrx_area,
    pub nr_areas: u32,
    pub niov_shift: u32,
    pub user: *mut user_struct,
    pub mm_account: *mut mm_struct,
    pub kern_readable: bool,
    pub rq: zcrx_rq,
    pub alloc_lock: spinlock_t,
    pub if_rxq: u32,
    pub dev: *mut device,
    pub netdev: *mut net_device,
    pub netdev_tracker: netdevice_tracker,
    pub refs: refcount_t,
    // counts userspace facing users like io_uring
    pub user_refs: refcount_t,
    // Page pool and net configuration lock, can be taken deeper in the
    // net stack.
    pub pp_lock: mutex,
    pub rq_region: io_mapped_region,
    pub ctx_lock: spinlock_t,
    pub master_ctx: *mut io_ring_ctx,
    pub allowed_notif_mask: u32,
    pub fired_notifs: u32,
    pub notif_data: u64,
    pub notif_stats: *mut zcrx_stats,
}

#[cfg(CONFIG_IO_URING_ZCRX)]
extern "C" {
    pub fn io_zcrx_ctrl(ctx: *mut io_ring_ctx, arg: *mut core::ffi::c_void, nr_arg: u32) -> i32;
    pub fn io_register_zcrx(ctx: *mut io_ring_ctx, arg: *mut io_uring_zcrx_ifq_reg) -> i32;
    pub fn io_unregister_zcrx(ctx: *mut io_ring_ctx);
    pub fn io_terminate_zcrx(ctx: *mut io_ring_ctx);
    pub fn io_zcrx_recv(req: *mut io_kiocb, ifq: *mut io_zcrx_ifq, sock: *mut socket,
                        flags: u32, issue_flags: u32, len: *mut u32) -> i32;
    pub fn io_zcrx_get_region(ctx: *mut io_ring_ctx, id: u32) -> *mut io_mapped_region;
}

#[cfg(not(CONFIG_IO_URING_ZCRX))]
pub unsafe fn io_register_zcrx(_ctx: *mut io_ring_ctx, _arg: *mut io_uring_zcrx_ifq_reg) -> i32 { -EOPNOTSUPP }
#[cfg(not(CONFIG_IO_URING_ZCRX))]
pub unsafe fn io_unregister_zcrx(_ctx: *mut io_ring_ctx) {}
#[cfg(not(CONFIG_IO_URING_ZCRX))]
pub unsafe fn io_terminate_zcrx(_ctx: *mut io_ring_ctx) {}
#[cfg(not(CONFIG_IO_URING_ZCRX))]
pub unsafe fn io_zcrx_recv(_req: *mut io_kiocb, _ifq: *mut io_zcrx_ifq, _sock: *mut socket,
                           _flags: u32, _issue_flags: u32, _len: *mut u32) -> i32 { -EOPNOTSUPP }
#[cfg(not(CONFIG_IO_URING_ZCRX))]
pub unsafe fn io_zcrx_get_region(_ctx: *mut io_ring_ctx, _id: u32) -> *mut io_mapped_region { core::ptr::null_mut() }
#[cfg(not(CONFIG_IO_URING_ZCRX))]
pub unsafe fn io_zcrx_ctrl(_ctx: *mut io_ring_ctx, _arg: *mut core::ffi::c_void, _nr_arg: u32) -> i32 { -EOPNOTSUPP }

extern "C" {
    pub fn io_recvzc(req: *mut io_kiocb, issue_flags: u32) -> i32;
    pub fn io_recvzc_prep(req: *mut io_kiocb, sqe: *const io_uring_sqe) -> i32;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
