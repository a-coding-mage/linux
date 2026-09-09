/* SPDX-License-Identifier: GPL-2.0 */

/* C header guard: INT_BLK_MQ_DEBUGFS_H */

/* CONFIG_BLK_DEBUG_FS declarations. */

use core::ffi::c_char;

pub enum blk_mq_hw_ctx {}

#[repr(C)]
pub struct blk_mq_debugfs_attr {
    pub name: *const c_char,
    pub mode: umode_t,
    pub show: Option<unsafe extern "C" fn(*mut core::ffi::c_void, *mut seq_file) -> i32>,
    pub write: Option<unsafe extern "C" fn(
        *mut core::ffi::c_void,
        *const c_char,
        usize,
        *mut loff_t,
    ) -> isize>,
    /* Set either .show or .seq_ops. */
    pub seq_ops: *const seq_operations,
}

extern "C" {
    pub fn __blk_mq_debugfs_rq_show(m: *mut seq_file, rq: *mut request) -> i32;
    pub fn blk_mq_debugfs_rq_show(m: *mut seq_file, v: *mut core::ffi::c_void) -> i32;

    pub fn blk_mq_debugfs_register(q: *mut request_queue);
    pub fn blk_mq_debugfs_register_hctx(q: *mut request_queue, hctx: *mut blk_mq_hw_ctx);
    pub fn blk_mq_debugfs_unregister_hctx(hctx: *mut blk_mq_hw_ctx);
    pub fn blk_mq_debugfs_register_hctxs(q: *mut request_queue);
    pub fn blk_mq_debugfs_unregister_hctxs(q: *mut request_queue);

    pub fn blk_mq_debugfs_register_sched(q: *mut request_queue);
    pub fn blk_mq_debugfs_unregister_sched(q: *mut request_queue);
    pub fn blk_mq_debugfs_register_sched_hctx(q: *mut request_queue, hctx: *mut blk_mq_hw_ctx);
    pub fn blk_mq_debugfs_unregister_sched_hctx(hctx: *mut blk_mq_hw_ctx);

    pub fn blk_mq_debugfs_register_rq_qos(q: *mut request_queue);
}

/* When CONFIG_BLK_DEBUG_FS is disabled, the C header provides empty inline functions. */
#[inline]
pub unsafe fn blk_mq_debugfs_register_disabled(_q: *mut request_queue) {}

#[inline]
pub unsafe fn blk_mq_debugfs_register_hctx_disabled(
    _q: *mut request_queue,
    _hctx: *mut blk_mq_hw_ctx,
) {
}

#[inline]
pub unsafe fn blk_mq_debugfs_unregister_hctx_disabled(_hctx: *mut blk_mq_hw_ctx) {}

#[inline]
pub unsafe fn blk_mq_debugfs_register_hctxs_disabled(_q: *mut request_queue) {}

#[inline]
pub unsafe fn blk_mq_debugfs_unregister_hctxs_disabled(_q: *mut request_queue) {}

#[inline]
pub unsafe fn blk_mq_debugfs_register_sched_disabled(_q: *mut request_queue) {}

#[inline]
pub unsafe fn blk_mq_debugfs_unregister_sched_disabled(_q: *mut request_queue) {}

#[inline]
pub unsafe fn blk_mq_debugfs_register_sched_hctx_disabled(
    _q: *mut request_queue,
    _hctx: *mut blk_mq_hw_ctx,
) {
}

#[inline]
pub unsafe fn blk_mq_debugfs_unregister_sched_hctx_disabled(_hctx: *mut blk_mq_hw_ctx) {}

#[inline]
pub unsafe fn blk_mq_debugfs_register_rq_qos_disabled(_q: *mut request_queue) {}

/* CONFIG_BLK_DEV_ZONED && CONFIG_BLK_DEBUG_FS. */
extern "C" {
    pub fn queue_zone_wplugs_show(data: *mut core::ffi::c_void, m: *mut seq_file) -> i32;
}

#[inline]
pub unsafe fn queue_zone_wplugs_show_disabled(
    _data: *mut core::ffi::c_void,
    _m: *mut seq_file,
) -> i32 {
    0
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
