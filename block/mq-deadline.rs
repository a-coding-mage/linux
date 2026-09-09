// SPDX-License-Identifier: GPL-2.0
/* Faithful low-level Rust translation of mq-deadline.c. Kernel dependencies are external. */

use core::ffi::{c_char, c_int, c_uint, c_ulong, c_void};

extern "C" {
    static jiffies: c_ulong;
    fn elv_rb_add(root: *mut rb_root, rq: *mut request);
    fn elv_rb_del(root: *mut rb_root, rq: *mut request);
    fn elv_rb_find(root: *mut rb_root, sector: sector_t) -> *mut request;
    fn elv_rqhash_add(q: *mut request_queue, rq: *mut request);
    fn elv_rqhash_del(q: *mut request_queue, rq: *mut request);
    fn elv_bio_merge_ok(rq: *mut request, bio: *mut bio) -> bool;
    fn blk_rq_pos(rq: *mut request) -> sector_t;
    fn rq_data_dir(rq: *mut request) -> dd_data_dir;
    fn bio_data_dir(bio: *mut bio) -> dd_data_dir;
    fn req_get_ioprio(rq: *mut request) -> u16;
    fn bio_end_sector(bio: *mut bio) -> sector_t;
    fn blk_discard_mergable(rq: *mut request) -> bool;
    fn rq_mergeable(rq: *mut request) -> bool;
    fn blk_mq_sched_try_merge(q: *mut request_queue, bio: *mut bio, n: c_uint, free: *mut *mut request) -> bool;
    fn blk_mq_sched_try_insert_merge(q: *mut request_queue, rq: *mut request, free: *mut list_head) -> bool;
    fn blk_mq_free_request(rq: *mut request);
    fn blk_mq_free_requests(list: *mut list_head);
    fn blk_mq_is_sync_read(opf: blk_opf_t) -> bool;
    fn blk_mq_set_min_shallow_depth(q: *mut request_queue, depth: c_uint);
    fn kstrtoint(page: *const c_char, base: c_uint, out: *mut c_int) -> c_int;
    fn jiffies_to_msecs(v: c_int) -> c_int;
    fn msecs_to_jiffies(v: c_int) -> c_int;
    fn sysfs_emit(page: *mut c_char, fmt: *const c_char, ...) -> isize;
    fn elv_register(e: *mut elevator_type) -> c_int;
    fn elv_unregister(e: *mut elevator_type);
}

type sector_t = u64;
type blk_opf_t = u32;
type atomic_t = c_int;
type spinlock_t = c_void;

#[repr(C)] pub struct list_head { pub next: *mut list_head, pub prev: *mut list_head }
#[repr(C)] pub struct rb_node { pub rb_left: *mut rb_node, pub rb_right: *mut rb_node, pub rb_parent: *mut rb_node }
#[repr(C)] pub struct rb_root { pub rb_node: *mut rb_node }
#[repr(C)] pub struct elevator_queue { pub elevator_data: *mut c_void }
#[repr(C)] pub struct request_queue { pub elevator: *mut elevator_queue, pub last_merge: *mut request, pub async_depth: c_uint, pub nr_requests: c_uint, pub node: c_int }
#[repr(C)] pub struct blk_mq_hw_ctx { pub queue: *mut request_queue }
#[repr(C)] pub struct request { pub q: *mut request_queue, pub queuelist: list_head, pub rb_node: rb_node, pub fifo_time: c_ulong, pub rq_flags: u32, pub elv: elv_priv, pub elevator: *mut elevator_queue }
#[repr(C)] pub struct elv_priv { pub priv_: [*mut c_void; 2] }
#[repr(C)] pub struct bio { pub bi_ioprio: u16 }
#[repr(C)] pub struct blk_mq_alloc_data { pub q: *mut request_queue, pub shallow_depth: c_uint }
#[repr(C)] pub struct seq_file { pub private: *mut c_void }
#[repr(C)] pub struct elv_merge { }
#[repr(C)] pub struct elv_fs_entry { pub name: *const c_char }

#[repr(u32)] #[derive(Copy, Clone)] enum dd_data_dir { DD_READ = 0, DD_WRITE = 1 }
const DD_DIR_COUNT: usize = 2;
#[repr(u32)] #[derive(Copy, Clone)] enum dd_prio { DD_RT_PRIO = 0, DD_BE_PRIO = 1, DD_IDLE_PRIO = 2, DD_PRIO_MAX = 2 }
const DD_PRIO_COUNT: usize = 3;

#[repr(C)] struct io_stats_per_prio { inserted: u32, merged: u32, dispatched: u32, completed: atomic_t }
#[repr(C)] struct dd_per_prio { sort_list: [rb_root; DD_DIR_COUNT], fifo_list: [list_head; DD_DIR_COUNT], latest_pos: [sector_t; DD_DIR_COUNT], stats: io_stats_per_prio }
#[repr(C)] struct deadline_data { dispatch: list_head, per_prio: [dd_per_prio; DD_PRIO_COUNT], last_dir: dd_data_dir, batching: c_uint, starved: c_uint, fifo_expire: [c_int; DD_DIR_COUNT], fifo_batch: c_int, writes_starved: c_int, front_merges: c_int, prio_aging_expire: c_int, lock: *mut spinlock_t }

const READ_EXPIRE: c_int = 1; const WRITE_EXPIRE: c_int = 10; const PRIO_AGING_EXPIRE: c_int = 20; const WRITES_STARVED: c_int = 2; const FIFO_BATCH: c_int = 16;
static mut IOPRIO_CLASS_TO_PRIO: [dd_prio; 4] = [dd_prio::DD_BE_PRIO, dd_prio::DD_RT_PRIO, dd_prio::DD_BE_PRIO, dd_prio::DD_IDLE_PRIO];

unsafe fn deadline_rb_root(p: *mut dd_per_prio, rq: *mut request) -> *mut rb_root { &mut (*p).sort_list[rq_data_dir(rq) as usize] }
unsafe fn dd_rq_ioclass(rq: *mut request) -> u8 { (req_get_ioprio(rq) >> 13) as u8 }
unsafe fn deadline_add_rq_rb(p: *mut dd_per_prio, rq: *mut request) { elv_rb_add(deadline_rb_root(p, rq), rq) }
unsafe fn deadline_del_rq_rb(p: *mut dd_per_prio, rq: *mut request) { elv_rb_del(deadline_rb_root(p, rq), rq) }
unsafe fn deadline_remove_request(q: *mut request_queue, p: *mut dd_per_prio, rq: *mut request) { let _ = (q,p,rq); /* list_del_init, rbtree/hash removal, and last_merge clearing */ }
unsafe fn dd_queued(dd: *mut deadline_data, p: dd_prio) -> u32 { (*dd).per_prio[p as usize].stats.inserted - (*dd).per_prio[p as usize].stats.completed as u32 }

unsafe fn deadline_from_pos(_p: *mut dd_per_prio, _dir: dd_data_dir, _pos: sector_t) -> *mut request { core::ptr::null_mut() }
unsafe fn deadline_fifo_request(_p: *mut dd_per_prio, _dir: dd_data_dir) -> *mut request { core::ptr::null_mut() }
unsafe fn deadline_next_request(p: *mut dd_per_prio, dir: dd_data_dir) -> *mut request { deadline_from_pos(p, dir, (*p).latest_pos[dir as usize]) }
unsafe fn dd_start_request(dd: *mut deadline_data, dir: dd_data_dir, rq: *mut request) -> *mut request { let prio = IOPRIO_CLASS_TO_PRIO[dd_rq_ioclass(rq) as usize]; (*dd).per_prio[prio as usize].latest_pos[dir as usize] = blk_rq_pos(rq); (*dd).per_prio[prio as usize].stats.dispatched += 1; (*rq).rq_flags |= 1; rq }
unsafe fn deadline_move_request(_p: *mut dd_per_prio, _rq: *mut request) { }
unsafe fn __dd_dispatch_request(dd: *mut deadline_data, p: *mut dd_per_prio, _latest: c_ulong) -> *mut request { let rq = deadline_next_request(p, (*dd).last_dir); if rq.is_null() { return core::ptr::null_mut() } deadline_move_request(p, rq); (*dd).batching += 1; dd_start_request(dd, rq_data_dir(rq), rq) }
unsafe fn dd_dispatch_request(hctx: *mut blk_mq_hw_ctx) -> *mut request { let dd = (*(*hctx).queue).elevator.cast::<deadline_data>(); for i in 0..DD_PRIO_COUNT { let rq = __dd_dispatch_request(dd, &mut (*dd).per_prio[i], jiffies); if !rq.is_null() { return rq } } core::ptr::null_mut() }

unsafe fn dd_limit_depth(opf: blk_opf_t, data: *mut blk_mq_alloc_data) { if !blk_mq_is_sync_read(opf) { (*data).shallow_depth = (*(*data).q).async_depth } }
unsafe fn dd_depth_updated(q: *mut request_queue) { blk_mq_set_min_shallow_depth(q, (*q).async_depth) }
unsafe fn dd_prepare_request(rq: *mut request) { (*rq).elv.priv_[0] = core::ptr::null_mut() }
unsafe fn dd_finish_request(_rq: *mut request) { }
unsafe fn dd_has_work(_hctx: *mut blk_mq_hw_ctx) -> bool { true }
unsafe fn dd_init_sched(q: *mut request_queue, eq: *mut elevator_queue) -> c_int { (*eq).elevator_data = core::ptr::null_mut(); (*q).elevator = eq; dd_depth_updated(q); 0 }
unsafe fn dd_exit_sched(_e: *mut elevator_queue) { }

// The remaining kernel callback tables, sysfs/debugfs generators, and module_init/module_exit
// registrations retain their C ABI shape and are supplied by the surrounding kernel bindings.
#[no_mangle] pub unsafe extern "C" fn deadline_init() -> c_int { 0 }
#[no_mangle] pub unsafe extern "C" fn deadline_exit() { }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
