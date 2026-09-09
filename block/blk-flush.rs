// SPDX-License-Identifier: GPL-2.0
/* Functions to sequence PREFLUSH and FUA writes. */

/* Dependencies supplied by the surrounding kernel translation. */

#[repr(C)]
pub enum ReqFseq {
    REQ_FSEQ_PREFLUSH = 1 << 0,
    REQ_FSEQ_DATA = 1 << 1,
    REQ_FSEQ_POSTFLUSH = 1 << 2,
    REQ_FSEQ_DONE = 1 << 3,
    REQ_FSEQ_ACTIONS = (1 << 0) | (1 << 1) | (1 << 2),
    FLUSH_PENDING_TIMEOUT = 5 * HZ,
}

extern "C" {
    static HZ: usize;
    fn blk_mq_map_queue(op: usize, ctx: *mut blk_mq_ctx) -> *mut blk_mq_hw_ctx;
    fn ffz(x: usize) -> usize;
    fn part_stat_lock();
    fn part_stat_inc(part: *mut block_device, stat: usize);
    fn part_stat_add(part: *mut block_device, stat: usize, value: u64);
    fn part_stat_unlock();
    fn blk_time_get_ns() -> u64;
    fn req_ref_put_and_test(rq: *mut request) -> bool;
    fn req_ref_set(rq: *mut request, value: usize);
    fn blk_mq_end_request(rq: *mut request, error: blk_status_t);
    fn blk_mq_kick_requeue_list(q: *mut request_queue);
    fn blk_rq_init(q: *mut request_queue, rq: *mut request);
    fn blk_mq_put_driver_tag(rq: *mut request);
    fn blk_mq_sched_restart(hctx: *mut blk_mq_hw_ctx);
    fn submit_bio_wait(bio: *mut bio) -> i32;
    fn bio_init(bio: *mut bio, bdev: *mut block_device, vec: *mut core::ffi::c_void,
                nr: usize, opf: blk_opf_t);
    fn blk_queue_write_cache(q: *mut request_queue) -> bool;
    fn blk_rq_sectors(rq: *mut request) -> usize;
    fn kfree(p: *mut core::ffi::c_void);
    fn kzalloc_node(size: usize, flags: gfp_t, node: i32) -> *mut core::ffi::c_void;
    fn spin_lock_init(lock: *mut spinlock_t);
    fn cache_line_size() -> usize;
    fn round_up(x: usize, y: usize) -> usize;
    fn init_list_head(head: *mut list_head);
    fn lockdep_set_class(lock: *mut spinlock_t, key: *mut lock_class_key);
}

type blk_opf_t = usize;
type blk_status_t = u32;
type gfp_t = usize;
type spinlock_t = core::ffi::c_void;
type lock_class_key = core::ffi::c_void;
type enum_rq_end_io_ret = i32;

#[repr(C)] pub struct list_head { pub next: *mut list_head, pub prev: *mut list_head }
#[repr(C)] pub struct block_device { pub _opaque: [u8; 0] }
#[repr(C)] pub struct bio { pub bi_iter: bio_iter }
#[repr(C)] pub struct bio_iter { pub bi_sector: u64 }
#[repr(C)] pub struct blk_mq_ctx { pub _opaque: [u8; 0] }
#[repr(C)] pub struct blk_mq_hw_ctx { pub fq: *mut blk_flush_queue, pub _opaque: [u8; 0] }
#[repr(C)] pub struct request_queue { pub disk: *mut disk, pub elevator: *mut core::ffi::c_void, pub requeue_lock: spinlock_t, pub requeue_list: list_head, pub flush_list: list_head, pub limits: queue_limits }
#[repr(C)] pub struct disk { pub part0: *mut block_device }
#[repr(C)] pub struct queue_limits { pub features: usize }
#[repr(C)] pub struct request { pub q: *mut request_queue, pub bio: *mut bio, pub biotail: *mut bio, pub __sector: u64, pub rq_flags: usize, pub end_io: Option<unsafe extern "C" fn(*mut request, blk_status_t, *const io_comp_batch) -> enum_rq_end_io_ret>, pub flush: flush_data, pub cmd_flags: blk_opf_t, pub queuelist: list_head, pub mq_ctx: *mut blk_mq_ctx, pub mq_hctx: *mut blk_mq_hw_ctx, pub tag: i32, pub internal_tag: i32, pub state: usize, pub start_time_ns: u64 }
#[repr(C)] pub struct flush_data { pub seq: usize, pub saved_end_io: Option<unsafe extern "C" fn(*mut request, blk_status_t, *const io_comp_batch) -> enum_rq_end_io_ret> }
#[repr(C)] pub struct blk_flush_queue { pub mq_flush_lock: spinlock_t, pub flush_queue: [list_head; 2], pub flush_pending_idx: usize, pub flush_running_idx: usize, pub flush_pending_since: usize, pub flush_data_in_flight: usize, pub flush_rq: *mut request, pub rq_status: blk_status_t }
#[repr(C)] pub struct io_comp_batch { pub _opaque: [u8; 0] }

const REQ_OP_FLUSH: usize = 0;
const REQ_OP_WRITE: usize = 1;
const REQ_PREFLUSH: usize = 1 << 8;
const REQ_FUA: usize = 1 << 9;
const REQ_SYNC: usize = 1 << 10;
const REQ_DRV: usize = 1 << 11;
const REQ_FAILFAST_MASK: usize = 0;
const RQF_FLUSH_SEQ: usize = 1 << 0;
const BLK_FEAT_FUA: usize = 1;
const BLK_STS_OK: blk_status_t = 0;
const BLK_MQ_NO_TAG: i32 = -1;
const STAT_FLUSH: usize = 0;
const MQ_RQ_IDLE: usize = 0;
const RQ_END_IO_NONE: i32 = 0;

unsafe fn blk_get_flush_queue(ctx: *mut blk_mq_ctx) -> *mut blk_flush_queue { (*blk_mq_map_queue(REQ_OP_FLUSH, ctx)).fq }
unsafe fn blk_flush_cur_seq(rq: *mut request) -> usize { 1usize << ffz((*rq).flush.seq) }

unsafe fn blk_flush_restore_request(rq: *mut request) {
    (*rq).bio = (*rq).biotail;
    if !(*rq).bio.is_null() { (*rq).__sector = (*(*rq).bio).bi_iter.bi_sector; }
    (*rq).rq_flags &= !RQF_FLUSH_SEQ;
    (*rq).end_io = (*rq).flush.saved_end_io;
}

unsafe fn blk_account_io_flush(rq: *mut request) {
    let part = (*(*rq).q).disk.as_ref().unwrap().part0;
    part_stat_lock(); part_stat_inc(part, STAT_FLUSH);
    part_stat_add(part, STAT_FLUSH, blk_time_get_ns() - (*rq).start_time_ns); part_stat_unlock();
}

unsafe fn blk_flush_complete_seq(rq: *mut request, fq: *mut blk_flush_queue, mut seq: usize, error: blk_status_t) {
    let q = (*rq).q; let pending = &mut (*fq).flush_queue[(*fq).flush_pending_idx];
    (*rq).flush.seq |= seq;
    seq = if error == 0 { blk_flush_cur_seq(rq) } else { ReqFseq::REQ_FSEQ_DONE as usize };
    match seq {
        x if x == ReqFseq::REQ_FSEQ_PREFLUSH as usize || x == ReqFseq::REQ_FSEQ_POSTFLUSH as usize => { (*fq).flush_pending_since = 0; (*pending).prev = (*rq).queuelist.prev; }
        x if x == ReqFseq::REQ_FSEQ_DATA as usize => { (*fq).flush_data_in_flight += 1; (*rq).queuelist.next = (*q).requeue_list.next; blk_mq_kick_requeue_list(q); }
        x if x == ReqFseq::REQ_FSEQ_DONE as usize => { blk_flush_restore_request(rq); blk_mq_end_request(rq, error); }
        _ => { core::hint::unreachable_unchecked() }
    }
    blk_kick_flush(q, fq, (*rq).cmd_flags);
}

unsafe fn blk_kick_flush(q: *mut request_queue, fq: *mut blk_flush_queue, flags: blk_opf_t) {
    let pending = &mut (*fq).flush_queue[(*fq).flush_pending_idx];
    if (*fq).flush_pending_idx != (*fq).flush_running_idx || pending.next == pending { return; }
    (*fq).flush_pending_idx ^= 1;
    let flush_rq = (*fq).flush_rq;
    blk_rq_init(q, flush_rq);
    (*flush_rq).cmd_flags = REQ_OP_FLUSH | REQ_PREFLUSH | (flags & (REQ_DRV | REQ_FAILFAST_MASK));
    (*flush_rq).rq_flags |= RQF_FLUSH_SEQ; (*flush_rq).end_io = Some(flush_end_io);
    req_ref_set(flush_rq, 1); (*fq).flush_running_idx ^= 1; blk_mq_kick_requeue_list(q);
}

pub unsafe extern "C" fn is_flush_rq(rq: *mut request) -> bool { (*rq).end_io.map(|f| f as usize) == Some(flush_end_io as usize) }

unsafe extern "C" fn flush_end_io(rq: *mut request, error: blk_status_t, _iob: *const io_comp_batch) -> enum_rq_end_io_ret { let fq = blk_get_flush_queue((*rq).mq_ctx); if !req_ref_put_and_test(rq) { (*fq).rq_status = error; return RQ_END_IO_NONE; } blk_account_io_flush(rq); (*rq).state = MQ_RQ_IDLE; (*fq).rq_status = BLK_STS_OK; (*fq).flush_running_idx ^= 1; RQ_END_IO_NONE }
unsafe extern "C" fn mq_flush_data_end_io(rq: *mut request, error: blk_status_t, _iob: *const io_comp_batch) -> enum_rq_end_io_ret { let fq = blk_get_flush_queue((*rq).mq_ctx); (*fq).flush_data_in_flight -= 1; blk_flush_complete_seq(rq, fq, ReqFseq::REQ_FSEQ_DATA as usize, error); blk_mq_sched_restart((*rq).mq_hctx); RQ_END_IO_NONE }

unsafe fn blk_rq_init_flush(rq: *mut request) { (*rq).flush.seq = 0; (*rq).rq_flags |= RQF_FLUSH_SEQ; (*rq).flush.saved_end_io = (*rq).end_io; (*rq).end_io = Some(mq_flush_data_end_io); }

pub unsafe extern "C" fn blk_insert_flush(rq: *mut request) -> bool {
    let q = (*rq).q; let fq = blk_get_flush_queue((*rq).mq_ctx); let supports_fua = (*q).limits.features & BLK_FEAT_FUA != 0; let mut policy = 0;
    if blk_rq_sectors(rq) != 0 { policy |= ReqFseq::REQ_FSEQ_DATA as usize; }
    if blk_queue_write_cache(q) { if (*rq).cmd_flags & REQ_PREFLUSH != 0 { policy |= ReqFseq::REQ_FSEQ_PREFLUSH as usize; } if (*rq).cmd_flags & REQ_FUA != 0 && !supports_fua { policy |= ReqFseq::REQ_FSEQ_POSTFLUSH as usize; } }
    (*rq).cmd_flags &= !REQ_PREFLUSH; if !supports_fua { (*rq).cmd_flags &= !REQ_FUA; } (*rq).cmd_flags |= REQ_SYNC;
    match policy { 0 => { blk_mq_end_request(rq, 0); true }, x if x == ReqFseq::REQ_FSEQ_DATA as usize => false,
        x if x == (ReqFseq::REQ_FSEQ_DATA as usize | ReqFseq::REQ_FSEQ_POSTFLUSH as usize) => { blk_rq_init_flush(rq); (*rq).flush.seq |= ReqFseq::REQ_FSEQ_PREFLUSH as usize; (*fq).flush_data_in_flight += 1; false },
        _ => { blk_rq_init_flush(rq); blk_flush_complete_seq(rq, fq, (ReqFseq::REQ_FSEQ_ACTIONS as usize) & !policy, 0); true } }
}

pub unsafe extern "C" fn blkdev_issue_flush(bdev: *mut block_device) -> i32 { let mut bio = bio { bi_iter: bio_iter { bi_sector: 0 } }; bio_init(&mut bio, bdev, core::ptr::null_mut(), 0, REQ_OP_WRITE | REQ_PREFLUSH); submit_bio_wait(&mut bio) }

pub unsafe extern "C" fn blk_alloc_flush_queue(node: i32, cmd_size: i32, flags: gfp_t) -> *mut blk_flush_queue { let fq = kzalloc_node(core::mem::size_of::<blk_flush_queue>(), flags, node) as *mut blk_flush_queue; if fq.is_null() { return core::ptr::null_mut(); } spin_lock_init(&mut (*fq).mq_flush_lock); let sz = round_up(core::mem::size_of::<request>() + cmd_size as usize, cache_line_size()); (*fq).flush_rq = kzalloc_node(sz, flags, node) as *mut request; if (*fq).flush_rq.is_null() { kfree(fq as *mut _); return core::ptr::null_mut(); } init_list_head(&mut (*fq).flush_queue[0]); init_list_head(&mut (*fq).flush_queue[1]); fq }
pub unsafe extern "C" fn blk_free_flush_queue(fq: *mut blk_flush_queue) { if fq.is_null() { return; } kfree((*fq).flush_rq as *mut _); kfree(fq as *mut _); }
pub unsafe extern "C" fn blk_mq_hctx_set_fq_lock_class(hctx: *mut blk_mq_hw_ctx, key: *mut lock_class_key) { lockdep_set_class(&mut (*(*hctx).fq).mq_flush_lock, key); }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
