/* SPDX-License-Identifier: GPL-2.0 */
/*
 * Rust translation of trace/events/readahead.h.
 * The tracepoint registration and formatting machinery is supplied by the
 * kernel tracing dependencies; these declarations preserve its entry layout.
 */

pub const TRACE_SYSTEM: &str = "readahead";

#[repr(C)]
pub struct PageCacheRaUnboundedEntry {
    pub i_ino: u64,
    pub s_dev: dev_t,
    pub index: pgoff_t,
    pub nr_to_read: ::core::ffi::c_ulong,
    pub lookahead_size: ::core::ffi::c_ulong,
}

#[repr(C)]
pub struct PageCacheRaOrderEntry {
    pub i_ino: u64,
    pub s_dev: dev_t,
    pub index: pgoff_t,
    pub order: ::core::ffi::c_uint,
    pub size: ::core::ffi::c_uint,
    pub async_size: ::core::ffi::c_uint,
    pub ra_pages: ::core::ffi::c_uint,
}

#[repr(C)]
pub struct PageCacheRaOpEntry {
    pub i_ino: u64,
    pub prev_pos: loff_t,
    pub index: pgoff_t,
    pub req_count: ::core::ffi::c_ulong,
    pub s_dev: dev_t,
    pub order: ::core::ffi::c_uint,
    pub size: ::core::ffi::c_uint,
    pub async_size: ::core::ffi::c_uint,
    pub ra_pages: ::core::ffi::c_uint,
    pub mmap_miss: ::core::ffi::c_uint,
}

/* Linux types supplied by the corresponding kernel headers. */
pub type dev_t = u64;
pub type pgoff_t = u64;
pub type loff_t = i64;

pub const PAGE_CACHE_RA_UNBOUNDED_PRINTK: &str =
    "dev=%d:%d ino=%llx index=%lu nr_to_read=%lu lookahead_size=%lu";
pub const PAGE_CACHE_RA_ORDER_PRINTK: &str =
    "dev=%d:%d ino=%llx index=%lu order=%u size=%u async_size=%u ra_pages=%u";
pub const PAGE_CACHE_RA_OP_PRINTK: &str =
    "dev=%d:%d ino=%llx index=%lu req_count=%lu order=%u size=%u async_size=%u ra_pages=%u mmap_miss=%u prev_pos=%lld";

/*
 * TRACE_EVENT(page_cache_ra_unbounded):
 * TP_PROTO(struct inode *inode, pgoff_t index, unsigned long nr_to_read,
 *          unsigned long lookahead_size)
 * TP_fast_assign:
 *   entry.i_ino = inode->i_ino;
 *   entry.s_dev = inode->i_sb->s_dev;
 *   entry.index = index;
 *   entry.nr_to_read = nr_to_read;
 *   entry.lookahead_size = lookahead_size;
 */
pub struct PageCacheRaUnbounded;

/* TRACE_EVENT(page_cache_ra_order), with the entry layout above. */
pub struct PageCacheRaOrder;

/* DECLARE_EVENT_CLASS(page_cache_ra_op), with the entry layout above. */
pub struct PageCacheRaOp;

/* DEFINE_EVENT(page_cache_ra_op, page_cache_sync_ra) */
pub struct PageCacheSyncRa;

/* DEFINE_EVENT(page_cache_ra_op, page_cache_async_ra) */
pub struct PageCacheAsyncRa;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
