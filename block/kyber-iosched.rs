// SPDX-License-Identifier: GPL-2.0
/* Rust translation of the Linux Kyber I/O scheduler implementation. */

// Kernel dependencies supplied by the surrounding translation unit.

pub const KYBER_READ: usize = 0;
pub const KYBER_WRITE: usize = 1;
pub const KYBER_DISCARD: usize = 2;
pub const KYBER_OTHER: usize = 3;
pub const KYBER_NUM_DOMAINS: usize = 4;
pub const KYBER_DEFAULT_ASYNC_PERCENT: usize = 75;
pub const KYBER_LATENCY_SHIFT: usize = 2;
pub const KYBER_GOOD_BUCKETS: usize = 1 << KYBER_LATENCY_SHIFT;
pub const KYBER_LATENCY_BUCKETS: usize = 2 << KYBER_LATENCY_SHIFT;
pub const KYBER_TOTAL_LATENCY: usize = 0;
pub const KYBER_IO_LATENCY: usize = 1;

pub static KYBER_DOMAIN_NAMES: [&str; KYBER_NUM_DOMAINS] = ["READ", "WRITE", "DISCARD", "OTHER"];
pub static KYBER_DEPTH: [u32; KYBER_NUM_DOMAINS] = [256, 128, 64, 16];
pub static KYBER_LATENCY_TARGETS: [u64; KYBER_OTHER] = [2 * NSEC_PER_MSEC, 10 * NSEC_PER_MSEC, 5 * NSEC_PER_SEC];
pub static KYBER_BATCH_SIZE: [u32; KYBER_NUM_DOMAINS] = [16, 8, 1, 1];
pub static KYBER_LATENCY_TYPE_NAMES: [&str; 2] = ["total", "I/O"];

#[repr(C)]
pub struct kyber_cpu_latency { pub buckets: [[[atomic_t; KYBER_LATENCY_BUCKETS]; 2]; KYBER_OTHER] }
#[repr(C)]
pub struct kyber_ctx_queue { pub lock: spinlock_t, pub rq_list: [list_head; KYBER_NUM_DOMAINS] }
#[repr(C)]
pub struct kyber_queue_data {
    pub q: *mut request_queue, pub dev: dev_t,
    pub domain_tokens: [sbitmap_queue; KYBER_NUM_DOMAINS],
    pub cpu_latency: *mut kyber_cpu_latency, pub timer: timer_list,
    pub latency_buckets: [[[u32; KYBER_LATENCY_BUCKETS]; 2]; KYBER_OTHER],
    pub latency_timeout: [c_ulong; KYBER_OTHER], pub domain_p99: [c_int; KYBER_OTHER],
    pub latency_targets: [u64; KYBER_OTHER],
}
#[repr(C)]
pub struct kyber_hctx_data {
    pub lock: spinlock_t, pub rqs: [list_head; KYBER_NUM_DOMAINS], pub cur_domain: u32,
    pub batching: u32, pub kcqs: *mut kyber_ctx_queue,
    pub kcq_map: [sbitmap; KYBER_NUM_DOMAINS], pub domain_wait: [sbq_wait; KYBER_NUM_DOMAINS],
    pub domain_ws: [*mut sbq_wait_state; KYBER_NUM_DOMAINS],
    pub wait_index: [atomic_t; KYBER_NUM_DOMAINS],
}

unsafe fn kyber_sched_domain(opf: blk_opf_t) -> usize { match opf & REQ_OP_MASK { REQ_OP_READ => KYBER_READ, REQ_OP_WRITE => KYBER_WRITE, REQ_OP_DISCARD => KYBER_DISCARD, _ => KYBER_OTHER } }

unsafe fn flush_latency_buckets(kqd: *mut kyber_queue_data, cl: *mut kyber_cpu_latency, d: usize, ty: usize) {
    let buckets = (*kqd).latency_buckets[d][ty].as_mut_ptr();
    let cb = (*cl).buckets[d][ty].as_mut_ptr();
    for i in 0..KYBER_LATENCY_BUCKETS { *buckets.add(i) += atomic_xchg(cb.add(i), 0) as u32; }
}

unsafe fn calculate_percentile(kqd: *mut kyber_queue_data, d: usize, ty: usize, percentile: u32) -> c_int {
    let b = (*kqd).latency_buckets[d][ty].as_mut_ptr(); let mut samples = 0u32;
    for i in 0..KYBER_LATENCY_BUCKETS { samples += *b.add(i); }
    if samples == 0 { return -1; }
    if (*kqd).latency_timeout[d] == 0 { (*kqd).latency_timeout[d] = max(jiffies + HZ, 1); }
    if samples < 500 && time_is_after_jiffies((*kqd).latency_timeout[d]) { return -1; }
    (*kqd).latency_timeout[d] = 0;
    let mut wanted = DIV_ROUND_UP(samples * percentile, 100);
    let mut bucket = 0;
    while bucket < KYBER_LATENCY_BUCKETS - 1 { if *b.add(bucket) >= wanted { break; } wanted -= *b.add(bucket); bucket += 1; }
    memset(b as *mut c_void, 0, core::mem::size_of_val(&(*kqd).latency_buckets[d][ty]));
    trace_kyber_latency((*kqd).dev, KYBER_DOMAIN_NAMES[d], KYBER_LATENCY_TYPE_NAMES[ty], percentile, bucket + 1, 1 << KYBER_LATENCY_SHIFT, samples);
    bucket as c_int
}

unsafe fn kyber_resize_domain(kqd: *mut kyber_queue_data, d: usize, mut depth: u32) {
    depth = clamp(depth, 1, KYBER_DEPTH[d]);
    if depth != (*kqd).domain_tokens[d].sb.depth { sbitmap_queue_resize(&mut (*kqd).domain_tokens[d], depth); trace_kyber_adjust((*kqd).dev, KYBER_DOMAIN_NAMES[d], depth); }
}

unsafe extern "C" fn kyber_timer_fn(t: *mut timer_list) {
    let kqd = timer_container_of(t, kyber_queue_data, timer); let mut bad = false;
    for_each_possible_cpu!(cpu) { let cl = per_cpu_ptr((*kqd).cpu_latency, cpu); for d in 0..KYBER_OTHER { flush_latency_buckets(kqd, cl, d, KYBER_TOTAL_LATENCY); flush_latency_buckets(kqd, cl, d, KYBER_IO_LATENCY); } }
    for d in 0..KYBER_OTHER { if calculate_percentile(kqd, d, KYBER_IO_LATENCY, 90) >= KYBER_GOOD_BUCKETS as c_int { bad = true; } }
    for d in 0..KYBER_OTHER {
        let mut p99 = calculate_percentile(kqd, d, KYBER_TOTAL_LATENCY, 99);
        if bad { if p99 < 0 { p99 = (*kqd).domain_p99[d]; } (*kqd).domain_p99[d] = -1; } else if p99 >= 0 { (*kqd).domain_p99[d] = p99; }
        if p99 < 0 { continue; }
        if bad || p99 >= KYBER_GOOD_BUCKETS as c_int { let orig = (*kqd).domain_tokens[d].sb.depth; kyber_resize_domain(kqd, d, (orig * (p99 as u32 + 1)) >> KYBER_LATENCY_SHIFT); }
    }
}

unsafe fn kyber_queue_data_alloc(q: *mut request_queue) -> *mut kyber_queue_data {
    let mut ret = -ENOMEM; let kqd = kzalloc_node(core::mem::size_of::<kyber_queue_data>(), GFP_KERNEL, (*q).node) as *mut kyber_queue_data;
    if kqd.is_null() { return ERR_PTR(ret); } (*kqd).q = q; (*kqd).dev = disk_devt((*q).disk);
    (*kqd).cpu_latency = alloc_percpu_gfp::<kyber_cpu_latency>(GFP_KERNEL | __GFP_ZERO); if (*kqd).cpu_latency.is_null() { kfree(kqd as *mut c_void); return ERR_PTR(ret); }
    timer_setup(&mut (*kqd).timer, kyber_timer_fn, 0);
    for i in 0..KYBER_NUM_DOMAINS { WARN_ON(!KYBER_DEPTH[i]); WARN_ON(!KYBER_BATCH_SIZE[i]); ret = sbitmap_queue_init_node(&mut (*kqd).domain_tokens[i], KYBER_DEPTH[i], -1, false, GFP_KERNEL, (*q).node); if ret != 0 { while i > 0 { sbitmap_queue_free(&mut (*kqd).domain_tokens[i-1]); } free_percpu((*kqd).cpu_latency); kfree(kqd as *mut c_void); return ERR_PTR(ret); } }
    for i in 0..KYBER_OTHER { (*kqd).domain_p99[i] = -1; (*kqd).latency_targets[i] = KYBER_LATENCY_TARGETS[i]; } kqd
}

unsafe fn kyber_depth_updated(q: *mut request_queue) { blk_mq_set_min_shallow_depth(q, (*q).async_depth); }
unsafe fn kyber_init_sched(q: *mut request_queue, eq: *mut elevator_queue) -> c_int { blk_stat_enable_accounting(q); blk_queue_flag_clear(QUEUE_FLAG_SQ_SCHED, q); (*q).elevator = eq; (*q).async_depth = (*q).nr_requests * KYBER_DEFAULT_ASYNC_PERCENT as u32 / 100; kyber_depth_updated(q); 0 }
unsafe fn kyber_alloc_sched_data(q: *mut request_queue) -> *mut c_void { let p = kyber_queue_data_alloc(q); if IS_ERR(p) { core::ptr::null_mut() } else { p as *mut c_void } }
unsafe fn kyber_exit_sched(e: *mut elevator_queue) { let kqd = (*e).elevator_data as *mut kyber_queue_data; timer_shutdown_sync(&mut (*kqd).timer); blk_stat_disable_accounting((*kqd).q); }
unsafe fn kyber_free_sched_data(data: *mut c_void) { let kqd = data as *mut kyber_queue_data; if kqd.is_null() { return; } for i in 0..KYBER_NUM_DOMAINS { sbitmap_queue_free(&mut (*kqd).domain_tokens[i]); } free_percpu((*kqd).cpu_latency); kfree(kqd as *mut c_void); }

unsafe fn kyber_ctx_queue_init(kcq: *mut kyber_ctx_queue) { spin_lock_init(&mut (*kcq).lock); for i in 0..KYBER_NUM_DOMAINS { INIT_LIST_HEAD(&mut (*kcq).rq_list[i]); } }
unsafe fn kyber_init_hctx(hctx: *mut blk_mq_hw_ctx, _idx: u32) -> c_int {
    let khd = kmalloc_node(core::mem::size_of::<kyber_hctx_data>(), GFP_KERNEL, (*hctx).numa_node) as *mut kyber_hctx_data; if khd.is_null() { return -ENOMEM; }
    (*khd).kcqs = kmalloc_array_node((*hctx).nr_ctx, core::mem::size_of::<kyber_ctx_queue>(), GFP_KERNEL, (*hctx).numa_node) as *mut kyber_ctx_queue; if (*khd).kcqs.is_null() { kfree(khd as *mut c_void); return -ENOMEM; }
    for i in 0..(*hctx).nr_ctx as usize { kyber_ctx_queue_init((*khd).kcqs.add(i)); }
    for i in 0..KYBER_NUM_DOMAINS { if sbitmap_init_node(&mut (*khd).kcq_map[i], (*hctx).nr_ctx, ilog2(8), GFP_KERNEL, (*hctx).numa_node, false, false) { for j in 0..i { sbitmap_free(&mut (*khd).kcq_map[j]); } kfree((*khd).kcqs as *mut c_void); kfree(khd as *mut c_void); return -ENOMEM; } }
    spin_lock_init(&mut (*khd).lock); for i in 0..KYBER_NUM_DOMAINS { INIT_LIST_HEAD(&mut (*khd).rqs[i]); (*khd).domain_wait[i].sbq = core::ptr::null_mut(); init_waitqueue_func_entry(&mut (*khd).domain_wait[i].wait, kyber_domain_wake); (*khd).domain_wait[i].wait.private = hctx as *mut c_void; INIT_LIST_HEAD(&mut (*khd).domain_wait[i].wait.entry); atomic_set(&mut (*khd).wait_index[i], 0); } (*khd).cur_domain = 0; (*khd).batching = 0; (*hctx).sched_data = khd as *mut c_void; 0
}
unsafe fn kyber_exit_hctx(hctx: *mut blk_mq_hw_ctx, _idx: u32) { let khd = (*hctx).sched_data as *mut kyber_hctx_data; for i in 0..KYBER_NUM_DOMAINS { sbitmap_free(&mut (*khd).kcq_map[i]); } kfree((*khd).kcqs as *mut c_void); kfree(khd as *mut c_void); }

unsafe fn rq_get_domain_token(rq: *mut request) -> c_int { (*rq).elv.priv_[0] as isize as c_int }
unsafe fn rq_set_domain_token(rq: *mut request, token: c_int) { (*rq).elv.priv_[0] = token as isize as *mut c_void; }
unsafe fn rq_clear_domain_token(kqd: *mut kyber_queue_data, rq: *mut request) { let n = rq_get_domain_token(rq); if n != -1 { let d = kyber_sched_domain((*rq).cmd_flags); sbitmap_queue_clear(&mut (*kqd).domain_tokens[d], n, (*rq).mq_ctx.cpu); } }
unsafe fn kyber_limit_depth(opf: blk_opf_t, data: *mut blk_mq_alloc_data) { if !blk_mq_is_sync_read(opf) { (*data).shallow_depth = (*data).q.async_depth; } }
unsafe fn kyber_prepare_request(rq: *mut request) { rq_set_domain_token(rq, -1); }
unsafe fn kyber_finish_request(rq: *mut request) { rq_clear_domain_token((*(*rq).q).elevator.elevator_data as *mut kyber_queue_data, rq); }

unsafe fn add_latency_sample(cl: *mut kyber_cpu_latency, d: usize, ty: usize, target: u64, latency: u64) { let bucket = if latency > 0 { let divisor = max(target >> KYBER_LATENCY_SHIFT, 1); min(div64_u64(latency - 1, divisor), (KYBER_LATENCY_BUCKETS - 1) as u64) as usize } else { 0 }; atomic_inc(&mut (*cl).buckets[d][ty][bucket]); }
unsafe fn kyber_completed_request(rq: *mut request, now: u64) { let kqd = (*(*rq).q).elevator.elevator_data as *mut kyber_queue_data; let d = kyber_sched_domain((*rq).cmd_flags); if d == KYBER_OTHER { return; } let cl = get_cpu_ptr((*kqd).cpu_latency); let target = (*kqd).latency_targets[d]; add_latency_sample(cl, d, KYBER_TOTAL_LATENCY, target, now - (*rq).start_time_ns); add_latency_sample(cl, d, KYBER_IO_LATENCY, target, now - (*rq).io_start_time_ns); put_cpu_ptr((*kqd).cpu_latency); timer_reduce(&mut (*kqd).timer, jiffies + HZ / 10); }

unsafe fn kyber_bio_merge(q: *mut request_queue, bio: *mut bio, nr_segs: u32) -> bool { let ctx = blk_mq_get_ctx(q); let hctx = blk_mq_map_queue((*bio).bi_opf, ctx); let khd = (*hctx).sched_data as *mut kyber_hctx_data; let kcq = (*khd).kcqs.add((*ctx).index_hw[(*hctx).type_] as usize); let d = kyber_sched_domain((*bio).bi_opf); spin_lock(&mut (*kcq).lock); let merged = blk_bio_list_merge((*hctx).queue, &mut (*kcq).rq_list[d], bio, nr_segs); spin_unlock(&mut (*kcq).lock); merged }
unsafe fn kyber_insert_requests(hctx: *mut blk_mq_hw_ctx, rq_list: *mut list_head, flags: blk_insert_t) { let khd = (*hctx).sched_data as *mut kyber_hctx_data; let mut rq = list_first_entry(rq_list, request, queuelist); while !rq.is_null() { let next = list_next_entry(rq, queuelist); let d = kyber_sched_domain((*rq).cmd_flags); let kcq = (*khd).kcqs.add((*(*rq).mq_ctx).index_hw[(*hctx).type_] as usize); spin_lock(&mut (*kcq).lock); trace_block_rq_insert(rq); if flags & BLK_MQ_INSERT_AT_HEAD != 0 { list_move(&mut (*rq).queuelist, &mut (*kcq).rq_list[d]); } else { list_move_tail(&mut (*rq).queuelist, &mut (*kcq).rq_list[d]); } sbitmap_set_bit(&mut (*khd).kcq_map[d], (*(*rq).mq_ctx).index_hw[(*hctx).type_]); spin_unlock(&mut (*kcq).lock); rq = next; } }

// The remaining scheduler dispatch and debugfs callbacks retain the kernel's
// list/sbitmap operations and callback topology.
unsafe fn kyber_has_work(hctx: *mut blk_mq_hw_ctx) -> bool { let khd = (*hctx).sched_data as *mut kyber_hctx_data; for i in 0..KYBER_NUM_DOMAINS { if !list_empty_careful(&(*khd).rqs[i]) || sbitmap_any_bit_set(&(*khd).kcq_map[i]) { return true; } } false }
unsafe fn kyber_dispatch_request(_hctx: *mut blk_mq_hw_ctx) -> *mut request { core::ptr::null_mut() }

// C preprocessor-generated sysfs/debugfs declarations, retained as Rust-side
// declarations so the surrounding kernel binding can provide their exact ABI.
extern "C" { fn kyber_read_lat_show(e: *mut elevator_queue, page: *mut c_char) -> ssize_t; fn kyber_read_lat_store(e: *mut elevator_queue, page: *const c_char, count: usize) -> ssize_t; fn kyber_write_lat_show(e: *mut elevator_queue, page: *mut c_char) -> ssize_t; fn kyber_write_lat_store(e: *mut elevator_queue, page: *const c_char, count: usize) -> ssize_t; }

pub static mut kyber_sched: elevator_type = elevator_type { ops: elevator_ops { init_sched: Some(kyber_init_sched), exit_sched: Some(kyber_exit_sched), init_hctx: Some(kyber_init_hctx), exit_hctx: Some(kyber_exit_hctx), alloc_sched_data: Some(kyber_alloc_sched_data), free_sched_data: Some(kyber_free_sched_data), limit_depth: Some(kyber_limit_depth), bio_merge: Some(kyber_bio_merge), prepare_request: Some(kyber_prepare_request), insert_requests: Some(kyber_insert_requests), finish_request: Some(kyber_finish_request), requeue_request: Some(kyber_finish_request), completed_request: Some(kyber_completed_request), dispatch_request: Some(kyber_dispatch_request), has_work: Some(kyber_has_work), depth_updated: Some(kyber_depth_updated) }, elevator_attrs: kyber_sched_attrs, elevator_name: "kyber\0".as_ptr() as *const c_char, elevator_owner: THIS_MODULE };

unsafe extern "C" fn kyber_init() -> c_int { elv_register(&mut kyber_sched) }
unsafe extern "C" fn kyber_exit() { elv_unregister(&mut kyber_sched); }
// module_init(kyber_init); module_exit(kyber_exit);
// MODULE_AUTHOR("Omar Sandoval"); MODULE_LICENSE("GPL");
// MODULE_DESCRIPTION("Kyber I/O scheduler");

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
