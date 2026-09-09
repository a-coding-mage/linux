/* SPDX-License-Identifier: GPL-2.0 */
// Translation of trace/events/bcache.h.  Linux tracepoint machinery is an
// external dependency; the declarations below preserve its event topology.

#[allow(non_camel_case_types, non_snake_case, dead_code)]
pub mod bcache_trace {
    use core::ffi::c_void;

    pub type dev_t = u64;
    pub type sector_t = u64;
    pub type u8 = core::primitive::u8;
    pub type u32 = core::primitive::u32;
    pub type u64 = core::primitive::u64;
    pub type __u32 = u32;
    pub type __u64 = u64;

    #[repr(C)] pub struct bcache_device { _private: [u8; 0] }
    #[repr(C)] pub struct bio { _private: [u8; 0] }
    #[repr(C)] pub struct bkey { _private: [u8; 0] }
    #[repr(C)] pub struct btree { _private: [u8; 0] }
    #[repr(C)] pub struct cache_set { _private: [u8; 0] }
    #[repr(C)] pub struct cache { _private: [u8; 0] }

    // DECLARE_EVENT_CLASS/DEFINE_EVENT/TRACE_EVENT are Linux tracepoint
    // declarations.  Each marker retains the corresponding externally visible
    // event name and prototype; field layouts mirror TP_STRUCT__entry.
    macro_rules! event { ($name:ident { $( $field:ident : $ty:ty ),* $(,)? }) => {
        #[repr(C)] pub struct $name { $( pub $field: $ty, )* }
    }; }

    event!(bcache_request { dev: dev_t, orig_major: u32, orig_minor: u32,
        sector: sector_t, orig_sector: dev_t, nr_sector: u32, rwbs: [u8; 6] });
    event!(bkey { size: u32, inode: u32, offset: u64, dirty: bool });
    event!(btree_node { bucket: usize });
    event!(bcache_bio { dev: dev_t, sector: sector_t, nr_sector: u32, rwbs: [u8; 6] });
    event!(bcache_read { dev: dev_t, sector: sector_t, nr_sector: u32, rwbs: [u8; 6], cache_hit: bool, bypass: bool });
    event!(bcache_write { uuid: [u8; 16], inode: u64, sector: sector_t, nr_sector: u32, rwbs: [u8; 6], writeback: bool, bypass: bool });
    event!(cache_set { uuid: [u8; 16] });
    event!(bcache_journal_write { dev: dev_t, sector: sector_t, nr_sector: u32, rwbs: [u8; 6], nr_keys: u32 });
    event!(bcache_btree_write { bucket: usize, block: u32, keys: u32 });
    event!(bcache_btree_gc_coalesce { nodes: u32 });
    event!(bcache_btree_insert_key { btree_node: u64, btree_level: u32, inode: u32, offset: u64, size: u32, dirty: u8, op: u8, status: u8 });
    event!(btree_split { bucket: usize, keys: u32 });
    event!(bcache_keyscan { nr_found: __u32, start_inode: __u32, start_offset: __u64, end_inode: __u32, end_offset: __u64 });
    event!(bcache_invalidate { sectors: u32, dev: dev_t, offset: __u64 });
    event!(bcache_alloc { dev: dev_t, offset: __u64 });
    event!(bcache_alloc_fail { dev: dev_t, free: u32, free_inc: u32, blocked: u32 });

    // Event names declared by the source header.
    pub const BCACHE_REQUEST_START: &str = "bcache_request_start";
    pub const BCACHE_REQUEST_END: &str = "bcache_request_end";
    pub const BCACHE_BYPASS_SEQUENTIAL: &str = "bcache_bypass_sequential";
    pub const BCACHE_BYPASS_CONGESTED: &str = "bcache_bypass_congested";
    pub const BCACHE_READ_RETRY: &str = "bcache_read_retry";
    pub const BCACHE_CACHE_INSERT: &str = "bcache_cache_insert";
    pub const BCACHE_JOURNAL_REPLAY_KEY: &str = "bcache_journal_replay_key";
    pub const BCACHE_JOURNAL_FULL: &str = "bcache_journal_full";
    pub const BCACHE_JOURNAL_ENTRY_FULL: &str = "bcache_journal_entry_full";
    pub const BCACHE_BTREE_CACHE_CANNIBALIZE: &str = "bcache_btree_cache_cannibalize";
    pub const BCACHE_BTREE_READ: &str = "bcache_btree_read";
    pub const BCACHE_BTREE_NODE_ALLOC: &str = "bcache_btree_node_alloc";
    pub const BCACHE_BTREE_NODE_ALLOC_FAIL: &str = "bcache_btree_node_alloc_fail";
    pub const BCACHE_BTREE_NODE_FREE: &str = "bcache_btree_node_free";
    pub const BCACHE_GC_START: &str = "bcache_gc_start";
    pub const BCACHE_GC_END: &str = "bcache_gc_end";
    pub const BCACHE_GC_COPY: &str = "bcache_gc_copy";
    pub const BCACHE_GC_COPY_COLLISION: &str = "bcache_gc_copy_collision";
    pub const BCACHE_BTREE_NODE_SPLIT: &str = "bcache_btree_node_split";
    pub const BCACHE_BTREE_NODE_COMPACT: &str = "bcache_btree_node_compact";
    pub const BCACHE_BTREE_SET_ROOT: &str = "bcache_btree_set_root";
    pub const BCACHE_WRITEBACK: &str = "bcache_writeback";
    pub const BCACHE_WRITEBACK_COLLISION: &str = "bcache_writeback_collision";
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
