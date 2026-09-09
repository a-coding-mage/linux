/* SPDX-License-Identifier: GPL-2.0-or-later */
/*
 * RDMA Network Block Driver
 *
 * Copyright (c) 2014 - 2018 ProfitBricks GmbH. All rights reserved.
 * Copyright (c) 2018 - 2019 1&1 IONOS Cloud GmbH. All rights reserved.
 * Copyright (c) 2019 - 2020 1&1 IONOS SE. All rights reserved.
 */

/* Dependencies supplied by the Linux kernel and the other RNBD headers. */

/// Time in seconds between reconnect tries, default to 30 s.
pub const RECONNECT_DELAY: i32 = 30;
/// Number of times to reconnect on error before giving up, 0 disabled, -1 forever.
pub const MAX_RECONNECTS: i32 = -1;

#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum rnbd_clt_dev_state {
    DEV_STATE_INIT,
    DEV_STATE_MAPPED,
    DEV_STATE_MAPPED_DISCONNECTED,
    DEV_STATE_UNMAPPED,
}

#[repr(C)]
pub struct rnbd_iu_comp {
    pub wait: wait_queue_head_t,
    pub r#errno: ::core::ffi::c_int,
}

#[cfg(CONFIG_ARCH_NO_SG_CHAIN)]
pub const RNBD_INLINE_SG_CNT: usize = 0;
#[cfg(not(CONFIG_ARCH_NO_SG_CHAIN))]
pub const RNBD_INLINE_SG_CNT: usize = 2;
pub const RNBD_RDMA_SGL_SIZE: usize = ::core::mem::size_of::<scatterlist>() * RNBD_INLINE_SG_CNT;

#[repr(C)]
pub union rnbd_iu_request_or_buf {
    pub rq: *mut request,
    pub buf: *mut ::core::ffi::c_void,
}

#[repr(C)]
pub union rnbd_iu_dev_or_sess {
    pub dev: *mut rnbd_clt_dev,
    pub sess: *mut rnbd_clt_session,
}

#[repr(C)]
pub struct rnbd_iu {
    pub request_or_buf: rnbd_iu_request_or_buf,
    pub permit: *mut rtrs_permit,
    pub dev_or_sess: rnbd_iu_dev_or_sess,
    pub sgt: sg_table,
    pub work: work_struct,
    pub r#errno: ::core::ffi::c_int,
    pub comp: rnbd_iu_comp,
    pub refcount: atomic_t,
    pub first_sgl: [scatterlist; 0],
}

#[repr(C)]
pub struct rnbd_cpu_qlist {
    pub requeue_list: list_head,
    pub requeue_lock: spinlock_t,
    pub cpu: ::core::ffi::c_uint,
}

#[repr(C)]
pub struct rnbd_clt_session {
    pub list: list_head,
    pub rtrs: *mut rtrs_clt_sess,
    pub rtrs_waitq: wait_queue_head_t,
    pub rtrs_ready: bool,
    pub cpu_queues: *mut rnbd_cpu_qlist,
    pub cpu_queues_bm: [::core::ffi::c_ulong; (NR_CPUS + (8 * ::core::mem::size_of::<::core::ffi::c_ulong>()) - 1) / (8 * ::core::mem::size_of::<::core::ffi::c_ulong>())],
    pub cpu_rr: *mut ::core::ffi::c_int,
    pub busy: atomic_t,
    pub queue_depth: usize,
    pub max_io_size: u32,
    pub max_segments: u32,
    pub tag_set: blk_mq_tag_set,
    pub nr_poll_queues: u32,
    pub lock: mutex,
    pub devs_list: list_head,
    pub refcount: refcount_t,
    pub sessname: [::core::ffi::c_char; NAME_MAX],
    pub ver: u8,
}

/** Submission queues. */
#[repr(C)]
pub struct rnbd_queue {
    pub requeue_list: list_head,
    pub in_list: ::core::ffi::c_ulong,
    pub dev: *mut rnbd_clt_dev,
    pub hctx: *mut blk_mq_hw_ctx,
}

#[repr(C)]
pub struct rnbd_clt_dev {
    pub kobj: kobject,
    pub sess: *mut rnbd_clt_session,
    pub queue: *mut request_queue,
    pub hw_queues: *mut rnbd_queue,
    pub device_id: u32,
    /// Local Idr index - used to track minor number allocations.
    pub clt_device_id: ::core::ffi::c_int,
    pub lock: mutex,
    pub dev_state: rnbd_clt_dev_state,
    pub refcount: refcount_t,
    pub pathname: *mut ::core::ffi::c_char,
    pub access_mode: rnbd_access_mode,
    pub nr_poll_queues: u32,
    pub size: u64,
    pub list: list_head,
    pub gd: *mut gendisk,
    pub blk_symlink_name: *mut ::core::ffi::c_char,
    pub unmap_on_rmmod_work: work_struct,
}

extern "C" {
    pub fn rnbd_clt_map_device(
        sessname: *const ::core::ffi::c_char,
        paths: *mut rtrs_addr,
        path_cnt: usize,
        port_nr: u16,
        pathname: *const ::core::ffi::c_char,
        access_mode: rnbd_access_mode,
        nr_poll_queues: u32,
    ) -> *mut rnbd_clt_dev;
    pub fn rnbd_clt_unmap_device(
        dev: *mut rnbd_clt_dev,
        force: bool,
        sysfs_self: *const attribute,
    ) -> ::core::ffi::c_int;
    pub fn rnbd_clt_remap_device(dev: *mut rnbd_clt_dev) -> ::core::ffi::c_int;
    pub fn rnbd_clt_resize_disk(dev: *mut rnbd_clt_dev, newsize: sector_t) -> ::core::ffi::c_int;
    pub fn rnbd_clt_create_sysfs_files() -> ::core::ffi::c_int;
    pub fn rnbd_clt_destroy_sysfs_files();
    pub fn rnbd_clt_remove_dev_symlink(dev: *mut rnbd_clt_dev);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
