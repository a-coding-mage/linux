/* SPDX-License-Identifier: GPL-2.0 */
/* Block data types and constants. */

// External Linux types and declarations are supplied by other translated headers.

pub type BioEndIoT = unsafe extern "C" fn(*mut bio);

pub const SECTOR_SHIFT: u32 = 9;
pub const SECTOR_SIZE: u32 = 1u32 << SECTOR_SHIFT;
pub const PAGE_SECTORS_SHIFT: u32 = PAGE_SHIFT - SECTOR_SHIFT;
pub const PAGE_SECTORS: u32 = 1u32 << PAGE_SECTORS_SHIFT;
pub const SECTOR_MASK: u32 = PAGE_SECTORS - 1;

#[repr(C)]
pub struct block_device {
    pub bd_start_sect: sector_t,
    pub bd_nr_sectors: sector_t,
    pub bd_disk: *mut gendisk,
    pub bd_queue: *mut request_queue,
    pub bd_stats: *mut disk_stats,
    pub bd_stamp: c_ulong,
    pub __bd_flags: atomic_t,
    pub bd_dev: dev_t,
    pub bd_mapping: *mut address_space,
    pub bd_openers: atomic_t,
    pub bd_size_lock: spinlock_t,
    pub bd_claiming: *mut c_void,
    pub bd_holder: *mut c_void,
    pub bd_holder_ops: *const blk_holder_ops,
    pub bd_holder_lock: mutex,
    pub bd_holders: c_int,
    pub bd_holder_dir: *mut kobject,
    pub bd_fsfreeze_count: atomic_t,
    pub bd_fsfreeze_mutex: mutex,
    pub bd_meta_info: *mut partition_meta_info,
    pub bd_writers: c_int,
    // #ifdef CONFIG_SECURITY
    pub bd_security: *mut c_void,
    // #endif
    pub bd_device: device,
}

pub const BD_PARTNO: u32 = 255;
pub const BD_READ_ONLY: u32 = 1u32 << 8;
pub const BD_WRITE_HOLDER: u32 = 1u32 << 9;
pub const BD_HAS_SUBMIT_BIO: u32 = 1u32 << 10;
pub const BD_RO_WARNED: u32 = 1u32 << 11;
// #ifdef CONFIG_FAIL_MAKE_REQUEST
pub const BD_MAKE_IT_FAIL: u32 = 1u32 << 12;
// #endif

pub type blk_status_t = u8;
pub type blk_short_t = u16;
pub const BLK_STS_OK: blk_status_t = 0;
pub const BLK_STS_NOTSUPP: blk_status_t = 1;
pub const BLK_STS_TIMEOUT: blk_status_t = 2;
pub const BLK_STS_NOSPC: blk_status_t = 3;
pub const BLK_STS_TRANSPORT: blk_status_t = 4;
pub const BLK_STS_TARGET: blk_status_t = 5;
pub const BLK_STS_RESV_CONFLICT: blk_status_t = 6;
pub const BLK_STS_MEDIUM: blk_status_t = 7;
pub const BLK_STS_PROTECTION: blk_status_t = 8;
pub const BLK_STS_RESOURCE: blk_status_t = 9;
pub const BLK_STS_IOERR: blk_status_t = 10;
pub const BLK_STS_DM_REQUEUE: blk_status_t = 11;
pub const BLK_STS_AGAIN: blk_status_t = 12;
pub const BLK_STS_DEV_RESOURCE: blk_status_t = 13;
pub const BLK_STS_ZONE_OPEN_RESOURCE: blk_status_t = 14;
pub const BLK_STS_ZONE_ACTIVE_RESOURCE: blk_status_t = 15;
pub const BLK_STS_OFFLINE: blk_status_t = 16;
pub const BLK_STS_DURATION_LIMIT: blk_status_t = 17;
pub const BLK_STS_INVAL: blk_status_t = 19;

#[inline]
pub fn blk_path_error(error: blk_status_t) -> bool {
    !matches!(error, BLK_STS_NOTSUPP | BLK_STS_NOSPC | BLK_STS_TARGET |
        BLK_STS_RESV_CONFLICT | BLK_STS_MEDIUM | BLK_STS_PROTECTION)
}

pub type blk_opf_t = u32;
pub type blk_qc_t = c_uint;
pub const BLK_QC_T_NONE: blk_qc_t = c_uint::MAX;

#[repr(C)]
pub union bio_cookie_or_segments {
    pub bi_cookie: blk_qc_t,
    pub __bi_nr_segments: c_uint,
}

#[repr(C)]
pub struct bio {
    pub bi_next: *mut bio,
    pub bi_bdev: *mut block_device,
    pub bi_opf: blk_opf_t,
    pub bi_flags: c_ushort,
    pub bi_ioprio: c_ushort,
    pub bi_write_hint: rw_hint,
    pub bi_write_stream: u8,
    pub bi_status: blk_status_t,
    pub bi_bvec_gap_bit: u8,
    pub __bi_remaining: atomic_t,
    pub bi_io_vec: *mut bio_vec,
    pub bi_iter: bvec_iter,
    pub bi_cookie_or_segments: bio_cookie_or_segments,
    pub bi_end_io: Option<BioEndIoT>,
    pub bi_private: *mut c_void,
    // #ifdef CONFIG_BLK_CGROUP
    pub bi_blkg: *mut blkcg_gq,
    pub issue_time_ns: u64,
    // #ifdef CONFIG_BLK_CGROUP_IOCOST
    pub bi_iocost_cost: u64,
    // #endif
    // #endif
    // #ifdef CONFIG_BLK_INLINE_ENCRYPTION
    pub bi_crypt_context: *mut bio_crypt_ctx,
    // #endif
    // #if defined(CONFIG_BLK_DEV_INTEGRITY)
    pub bi_integrity: *mut bio_integrity_payload,
    // #endif
    pub bi_vcnt: c_ushort,
    pub bi_max_vecs: c_ushort,
    pub __bi_cnt: atomic_t,
    pub bi_pool: *mut bio_set,
}

pub const BIO_RESET_BYTES: usize = core::mem::offset_of!(bio, bi_max_vecs);
pub const BIO_MAX_SIZE: c_uint = c_uint::MAX;
pub const BIO_MAX_SECTORS: c_uint = BIO_MAX_SIZE >> SECTOR_SHIFT;

#[inline]
pub unsafe fn bio_inline_vecs(bio: *mut bio) -> *mut bio_vec {
    bio.add(1) as *mut bio_vec
}

pub const BIO_PAGE_PINNED: u32 = 0;
pub const BIO_CLONED: u32 = 1;
pub const BIO_QUIET: u32 = 2;
pub const BIO_CHAIN: u32 = 3;
pub const BIO_REFFED: u32 = 4;
pub const BIO_BPS_THROTTLED: u32 = 5;
pub const BIO_TRACE_COMPLETION: u32 = 6;
pub const BIO_CGROUP_ACCT: u32 = 7;
pub const BIO_QOS_THROTTLED: u32 = 8;
pub const BIO_TG_BPS_THROTTLED: u32 = BIO_QOS_THROTTLED;
pub const BIO_QOS_MERGED: u32 = 9;
pub const BIO_REMAPPED: u32 = 10;
pub const BIO_ZONE_WRITE_PLUGGING: u32 = 11;
pub const BIO_EMULATES_ZONE_APPEND: u32 = 12;
pub const BIO_COMPLETE_IN_TASK: u32 = 13;
pub const BIO_FLAG_LAST: u32 = 14;

pub type blk_mq_req_flags_t = u32;
pub const REQ_OP_BITS: u32 = 8;
pub const REQ_OP_MASK: blk_opf_t = (1u32 << REQ_OP_BITS) - 1;
pub const REQ_FLAG_BITS: u32 = 24;

#[repr(u32)]
pub enum req_op {
    REQ_OP_READ = 0, REQ_OP_WRITE = 1, REQ_OP_FLUSH = 2, REQ_OP_DISCARD = 3,
    REQ_OP_SECURE_ERASE = 5, REQ_OP_ZONE_APPEND = 7, REQ_OP_WRITE_ZEROES = 9,
    REQ_OP_ZONE_OPEN = 11, REQ_OP_ZONE_CLOSE = 13, REQ_OP_ZONE_FINISH = 15,
    REQ_OP_ZONE_RESET = 17, REQ_OP_ZONE_RESET_ALL = 19,
    REQ_OP_DRV_IN = 34, REQ_OP_DRV_OUT = 35, REQ_OP_LAST = 36,
}

pub const __REQ_FAILFAST_DEV: u32 = REQ_OP_BITS;
pub const __REQ_FAILFAST_TRANSPORT: u32 = __REQ_FAILFAST_DEV + 1;
pub const __REQ_FAILFAST_DRIVER: u32 = __REQ_FAILFAST_TRANSPORT + 1;
pub const __REQ_SYNC: u32 = __REQ_FAILFAST_DRIVER + 1;
pub const __REQ_META: u32 = __REQ_SYNC + 1;
pub const __REQ_PRIO: u32 = __REQ_META + 1;
pub const __REQ_NOMERGE: u32 = __REQ_PRIO + 1;
pub const __REQ_IDLE: u32 = __REQ_NOMERGE + 1;
pub const __REQ_INTEGRITY: u32 = __REQ_IDLE + 1;
pub const __REQ_FUA: u32 = __REQ_INTEGRITY + 1;
pub const __REQ_PREFLUSH: u32 = __REQ_FUA + 1;
pub const __REQ_RAHEAD: u32 = __REQ_PREFLUSH + 1;
pub const __REQ_BACKGROUND: u32 = __REQ_RAHEAD + 1;
pub const __REQ_NOWAIT: u32 = __REQ_BACKGROUND + 1;
pub const __REQ_POLLED: u32 = __REQ_NOWAIT + 1;
pub const __REQ_ALLOC_CACHE: u32 = __REQ_POLLED + 1;
pub const __REQ_SWAP: u32 = __REQ_ALLOC_CACHE + 1;
pub const __REQ_DRV: u32 = __REQ_SWAP + 1;
pub const __REQ_FS_PRIVATE: u32 = __REQ_DRV + 1;
pub const __REQ_ATOMIC: u32 = __REQ_FS_PRIVATE + 1;
pub const __REQ_NOUNMAP: u32 = __REQ_ATOMIC + 1;
pub const __REQ_NR_BITS: u32 = __REQ_NOUNMAP + 1;

pub const REQ_FAILFAST_DEV: blk_opf_t = 1u32 << __REQ_FAILFAST_DEV;
pub const REQ_FAILFAST_TRANSPORT: blk_opf_t = 1u32 << __REQ_FAILFAST_TRANSPORT;
pub const REQ_FAILFAST_DRIVER: blk_opf_t = 1u32 << __REQ_FAILFAST_DRIVER;
pub const REQ_SYNC: blk_opf_t = 1u32 << __REQ_SYNC;
pub const REQ_META: blk_opf_t = 1u32 << __REQ_META;
pub const REQ_PRIO: blk_opf_t = 1u32 << __REQ_PRIO;
pub const REQ_NOMERGE: blk_opf_t = 1u32 << __REQ_NOMERGE;
pub const REQ_IDLE: blk_opf_t = 1u32 << __REQ_IDLE;
pub const REQ_INTEGRITY: blk_opf_t = 1u32 << __REQ_INTEGRITY;
pub const REQ_FUA: blk_opf_t = 1u32 << __REQ_FUA;
pub const REQ_PREFLUSH: blk_opf_t = 1u32 << __REQ_PREFLUSH;
pub const REQ_RAHEAD: blk_opf_t = 1u32 << __REQ_RAHEAD;
pub const REQ_BACKGROUND: blk_opf_t = 1u32 << __REQ_BACKGROUND;
pub const REQ_NOWAIT: blk_opf_t = 1u32 << __REQ_NOWAIT;
pub const REQ_POLLED: blk_opf_t = 1u32 << __REQ_POLLED;
pub const REQ_ALLOC_CACHE: blk_opf_t = 1u32 << __REQ_ALLOC_CACHE;
pub const REQ_SWAP: blk_opf_t = 1u32 << __REQ_SWAP;
pub const REQ_DRV: blk_opf_t = 1u32 << __REQ_DRV;
pub const REQ_FS_PRIVATE: blk_opf_t = 1u32 << __REQ_FS_PRIVATE;
pub const REQ_ATOMIC: blk_opf_t = 1u32 << __REQ_ATOMIC;
pub const REQ_NOUNMAP: blk_opf_t = 1u32 << __REQ_NOUNMAP;
pub const REQ_FAILFAST_MASK: blk_opf_t = REQ_FAILFAST_DEV | REQ_FAILFAST_TRANSPORT | REQ_FAILFAST_DRIVER;
pub const REQ_NOMERGE_FLAGS: blk_opf_t = REQ_NOMERGE | REQ_PREFLUSH | REQ_FUA;

pub const STAT_READ: u32 = 0;
pub const STAT_WRITE: u32 = 1;
pub const STAT_DISCARD: u32 = 2;
pub const STAT_FLUSH: u32 = 3;
pub const NR_STAT_GROUPS: u32 = 4;

#[inline] pub unsafe fn bio_op(bio: *const bio) -> blk_opf_t { (*bio).bi_opf & REQ_OP_MASK }
#[inline] pub fn op_is_write(op: blk_opf_t) -> bool { (op & 1) != 0 }
#[inline] pub fn op_is_flush(op: blk_opf_t) -> bool { (op & (REQ_FUA | REQ_PREFLUSH)) != 0 }
#[inline] pub fn op_is_sync(op: blk_opf_t) -> bool { (op & REQ_OP_MASK) == 0 || (op & (REQ_SYNC | REQ_FUA | REQ_PREFLUSH)) != 0 }
#[inline] pub fn op_is_discard(op: blk_opf_t) -> bool { (op & REQ_OP_MASK) == 3 }
#[inline] pub fn op_is_zone_mgmt(op: blk_opf_t) -> bool { matches!(op & REQ_OP_MASK, 17 | 19 | 11 | 13 | 15) }
#[inline] pub fn op_stat_group(op: blk_opf_t) -> u32 { if op_is_discard(op) { STAT_DISCARD } else { op_is_write(op) as u32 } }

#[repr(C)]
pub struct blk_rq_stat { pub mean: u64, pub min: u64, pub max: u64, pub nr_samples: u32, pub batch: u64 }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
