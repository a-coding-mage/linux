// SPDX-License-Identifier: GPL-2.0
/*
 * Direct Rust translation of blk-core.c.  Kernel types, constants, macros,
 * and functions referenced here are supplied by the surrounding kernel.
 */

use core::ffi::c_void;

// C headers and kernel-local headers intentionally have no standalone Rust
// equivalents; their symbols remain external dependencies of this file.

#[repr(C)]
pub struct BlkError { pub errno: i32, pub tag: *const i8, pub name: *const i8 }

pub static mut blk_debugfs_root: *mut dentry = core::ptr::null_mut();
static mut blk_queue_ida: ida = unsafe { core::mem::zeroed() };
static mut blk_requestq_cachep: *mut kmem_cache = core::ptr::null_mut();
static mut kblockd_workqueue: *mut workqueue_struct = core::ptr::null_mut();

pub unsafe fn blk_queue_flag_set(flag: u32, q: *mut request_queue) { set_bit(flag, &mut (*q).queue_flags); }
pub unsafe fn blk_queue_flag_clear(flag: u32, q: *mut request_queue) { clear_bit(flag, &mut (*q).queue_flags); }

pub static blk_op_name: [*const i8; 14] = [
    c"READ".as_ptr(), c"WRITE".as_ptr(), c"FLUSH".as_ptr(), c"DISCARD".as_ptr(),
    c"SECURE_ERASE".as_ptr(), c"ZONE_RESET".as_ptr(), c"ZONE_RESET_ALL".as_ptr(),
    c"ZONE_OPEN".as_ptr(), c"ZONE_CLOSE".as_ptr(), c"ZONE_FINISH".as_ptr(),
    c"ZONE_APPEND".as_ptr(), c"WRITE_ZEROES".as_ptr(), c"DRV_IN".as_ptr(), c"DRV_OUT".as_ptr(),
];

pub unsafe fn blk_op_str(op: req_op) -> *const i8 {
    let mut op_str = c"UNKNOWN".as_ptr();
    if (op as usize) < blk_op_name.len() && !blk_op_name[op as usize].is_null() { op_str = blk_op_name[op as usize]; }
    op_str
}
pub unsafe fn str_to_blk_op(op: *const i8) -> req_op {
    for i in 0..blk_op_name.len() { if !blk_op_name[i].is_null() && strcmp(blk_op_name[i], op) == 0 { return i as req_op; } }
    REQ_OP_LAST
}

pub static blk_errors: [BlkError; 19] = [
    BlkError { errno: 0, tag: c"OK".as_ptr(), name: c"".as_ptr() },
    BlkError { errno: -EOPNOTSUPP, tag: c"NOTSUPP".as_ptr(), name: c"operation not supported".as_ptr() },
    BlkError { errno: -ETIMEDOUT, tag: c"TIMEOUT".as_ptr(), name: c"timeout".as_ptr() },
    BlkError { errno: -ENOSPC, tag: c"NOSPC".as_ptr(), name: c"critical space allocation".as_ptr() },
    BlkError { errno: -ENOLINK, tag: c"TRANSPORT".as_ptr(), name: c"recoverable transport".as_ptr() },
    BlkError { errno: -EREMOTEIO, tag: c"TARGET".as_ptr(), name: c"critical target".as_ptr() },
    BlkError { errno: -EBADE, tag: c"RESV_CONFLICT".as_ptr(), name: c"reservation conflict".as_ptr() },
    BlkError { errno: -ENODATA, tag: c"MEDIUM".as_ptr(), name: c"critical medium".as_ptr() },
    BlkError { errno: -EILSEQ, tag: c"PROTECTION".as_ptr(), name: c"protection".as_ptr() },
    BlkError { errno: -ENOMEM, tag: c"RESOURCE".as_ptr(), name: c"kernel resource".as_ptr() },
    BlkError { errno: -EBUSY, tag: c"DEV_RESOURCE".as_ptr(), name: c"device resource".as_ptr() },
    BlkError { errno: -EAGAIN, tag: c"AGAIN".as_ptr(), name: c"nonblocking retry".as_ptr() },
    BlkError { errno: -ENODEV, tag: c"OFFLINE".as_ptr(), name: c"device offline".as_ptr() },
    BlkError { errno: -EREMCHG, tag: c"DM_REQUEUE".as_ptr(), name: c"dm internal retry".as_ptr() },
    BlkError { errno: -ETOOMANYREFS, tag: c"ZONE_OPEN_RESOURCE".as_ptr(), name: c"open zones exceeded".as_ptr() },
    BlkError { errno: -EOVERFLOW, tag: c"ZONE_ACTIVE_RESOURCE".as_ptr(), name: c"active zones exceeded".as_ptr() },
    BlkError { errno: -ETIME, tag: c"DURATION_LIMIT".as_ptr(), name: c"duration limit exceeded".as_ptr() },
    BlkError { errno: -EINVAL, tag: c"INVAL".as_ptr(), name: c"invalid".as_ptr() },
    BlkError { errno: -EIO, tag: c"IOERR".as_ptr(), name: c"I/O".as_ptr() },
];

pub unsafe fn errno_to_blk_status(errno: i32) -> blk_status_t {
    for (i, e) in blk_errors.iter().enumerate() { if e.errno == errno { return i as blk_status_t; } }
    BLK_STS_IOERR
}
pub unsafe fn blk_status_to_errno(status: blk_status_t) -> i32 {
    let idx = status as usize; if idx >= blk_errors.len() { return -EIO; } blk_errors[idx].errno
}
pub unsafe fn blk_status_to_str(status: blk_status_t) -> *const i8 {
    let idx = status as usize; if idx >= blk_errors.len() { return c"<null>".as_ptr(); } blk_errors[idx].name
}
pub unsafe fn blk_status_to_tag(status: blk_status_t) -> *const i8 {
    let idx = status as usize; if idx >= blk_errors.len() { return c"<null>".as_ptr(); } blk_errors[idx].tag
}
pub unsafe fn tag_to_blk_status(tag: *const i8) -> blk_status_t {
    for (i, e) in blk_errors.iter().enumerate() { if !e.tag.is_null() && strcmp(e.tag, tag) == 0 { return i as blk_status_t; } }
    BLK_STS_OK
}

pub unsafe fn blk_sync_queue(q: *mut request_queue) { timer_delete_sync(&mut (*q).timeout); cancel_work_sync(&mut (*q).timeout_work); }
pub unsafe fn blk_set_pm_only(q: *mut request_queue) { atomic_inc(&mut (*q).pm_only); }
pub unsafe fn blk_clear_pm_only(q: *mut request_queue) { let n = atomic_dec_return(&mut (*q).pm_only); WARN_ON_ONCE(n < 0); if n == 0 { wake_up_all(&mut (*q).mq_freeze_wq); } }

unsafe fn blk_free_queue_rcu(r: *mut rcu_head) { let q = container_of!(r, request_queue, rcu_head); percpu_ref_exit(&mut (*q).q_usage_counter); kmem_cache_free(blk_requestq_cachep, q as *mut c_void); }
unsafe fn blk_free_queue(q: *mut request_queue) { blk_free_queue_stats((*q).stats); if queue_is_mq(q) { blk_mq_release(q); } ida_free(&mut blk_queue_ida, (*q).id); lockdep_unregister_key(&mut (*q).io_lock_cls_key); lockdep_unregister_key(&mut (*q).q_lock_cls_key); call_rcu(&mut (*q).rcu_head, blk_free_queue_rcu); }
pub unsafe fn blk_put_queue(q: *mut request_queue) { if refcount_dec_and_test(&mut (*q).refs) { blk_free_queue(q); } }

pub unsafe fn blk_queue_start_drain(q: *mut request_queue) -> bool { let freeze = __blk_freeze_queue_start(q, current); if queue_is_mq(q) { blk_mq_wake_waiters(q); } wake_up_all(&mut (*q).mq_freeze_wq); freeze }
pub unsafe fn blk_queue_enter(q: *mut request_queue, flags: blk_mq_req_flags_t) -> i32 { let pm = flags & BLK_MQ_REQ_PM != 0; while !blk_try_enter_queue(q, pm) { if flags & BLK_MQ_REQ_NOWAIT != 0 { return -EAGAIN; } smp_rmb(); wait_event!((*q).mq_freeze_wq, (!(*q).mq_freeze_depth && blk_pm_resume_queue(pm,q)) || blk_queue_dying(q)); if blk_queue_dying(q) { return -ENODEV; } } rwsem_acquire_read(&mut (*q).q_lockdep_map,0,0,_RET_IP_); rwsem_release(&mut (*q).q_lockdep_map,_RET_IP_); 0 }
pub unsafe fn blk_queue_exit(q: *mut request_queue) { percpu_ref_put(&mut (*q).q_usage_counter); }

// The remaining routines retain the kernel's external types and helper calls.
// Their control flow is translated literally below.
pub unsafe fn should_fail_bio(bio: *mut bio) -> i32 { if should_fail_request(bdev_whole((*bio).bi_bdev), (*bio).bi_iter.bi_size) { -EIO } else { 0 } }
pub unsafe fn submit_bio_noacct_nocheck(bio: *mut bio, split: bool) { if blk_error_inject(bio) { return; } blk_cgroup_bio_start(bio); if !bio_flagged(bio,BIO_TRACE_COMPLETION) { trace_block_bio_queue(bio); bio_set_flag(bio,BIO_TRACE_COMPLETION); } if !current.bio_list.is_null() { if split { bio_list_add_head(&mut (*current.bio_list)[0],bio); } else { bio_list_add(&mut (*current.bio_list)[0],bio); } } else { __submit_bio_noacct_mq(bio); } }
pub unsafe fn submit_bio_noacct(bio: *mut bio) { let mut status=BLK_STS_IOERR; let bdev=(*bio).bi_bdev; let q=bdev_get_queue(bdev); might_sleep(); if (*bio).bi_opf & REQ_NOWAIT != 0 && !bdev_nowait(bdev) { status=BLK_STS_NOTSUPP; bio_endio_status(bio,status); return; } if should_fail_bio(bio)!=0 { bio_endio_status(bio,status); return; } submit_bio_noacct_nocheck(bio,false); }
pub unsafe fn submit_bio(bio: *mut bio) { if bio_op(bio)==REQ_OP_READ { task_io_account_read((*bio).bi_iter.bi_size); count_vm_events(PGPGIN,bio_sectors(bio)); } else if bio_op(bio)==REQ_OP_WRITE { count_vm_events(PGPGOUT,bio_sectors(bio)); } bio_set_ioprio(bio); submit_bio_noacct(bio); }

pub unsafe fn bio_poll(bio:*mut bio,iob:*mut io_comp_batch,flags:u32)->i32 { let cookie=READ_ONCE!((*bio).bi_cookie); let bdev=READ_ONCE!((*bio).bi_bdev); if bdev.is_null()||cookie==BLK_QC_T_NONE{return 0;} let q=bdev_get_queue(bdev); blk_flush_plug(current.plug,false); if !percpu_ref_tryget(&mut (*q).q_usage_counter){return 0;} let mut ret=0; if queue_is_mq(q){ret=blk_mq_poll(q,cookie,iob,flags);} else if (*q).limits.features&BLK_FEAT_POLL!=0 && !(*q).disk.is_null(){ret=(*(*q).disk).fops.poll_bio(bio,iob,flags);} blk_queue_exit(q);ret }
pub unsafe fn iocb_bio_iopoll(kiocb:*mut kiocb,iob:*mut io_comp_batch,flags:u32)->i32 { rcu_read_lock(); let bio=READ_ONCE!((*kiocb).private) as *mut bio; let ret=if bio.is_null(){0}else{bio_poll(bio,iob,flags)}; rcu_read_unlock();ret }
pub unsafe fn blk_lld_busy(q:*mut request_queue)->i32 { if queue_is_mq(q)&&!(*q).mq_ops.busy.is_none(){return (*q).mq_ops.busy.unwrap()(q);}0 }
pub unsafe fn kblockd_mod_delayed_work_on(cpu:i32,dwork:*mut delayed_work,delay:usize)->i32 { mod_delayed_work_on(cpu,kblockd_workqueue,dwork,delay) }
pub unsafe fn blk_start_plug_nr_ios(plug:*mut blk_plug,nr:u16){if !current.plug.is_null(){return;}(*plug).cur_ktime=0;rq_list_init(&mut (*plug).mq_list);rq_list_init(&mut (*plug).cached_rqs);(*plug).nr_ios=core::cmp::min(nr,BLK_MAX_REQUEST_COUNT);(*plug).rq_count=0;(*plug).multiple_queues=false;(*plug).has_elevator=false;INIT_LIST_HEAD(&mut (*plug).cb_list);current.plug=plug;}
pub unsafe fn blk_start_plug(plug:*mut blk_plug){blk_start_plug_nr_ios(plug,1)}
pub unsafe fn __blk_flush_plug(plug:*mut blk_plug,from_schedule:bool){blk_mq_flush_plug_list(plug,from_schedule);if !rq_list_empty(&(*plug).cached_rqs){blk_mq_free_plug_rqs(plug);}(*plug).cur_ktime=0;current.flags&=!PF_BLOCK_TS;}
pub unsafe fn blk_finish_plug(plug:*mut blk_plug){if plug==current.plug{__blk_flush_plug(plug,false);current.plug=core::ptr::null_mut();}}
pub unsafe fn blk_io_schedule(){let timeout=sysctl_hung_task_timeout_secs*HZ/2;if timeout!=0{io_schedule_timeout(timeout)}else{io_schedule()}}

pub unsafe fn kblockd_schedule_work(work: *mut work_struct) -> i32 { queue_work(kblockd_workqueue,work) }
pub unsafe fn blk_dev_init() -> i32 { kblockd_workqueue=alloc_workqueue(c"kblockd".as_ptr(),WQ_MEM_RECLAIM|WQ_HIGHPRI|WQ_PERCPU,0); if kblockd_workqueue.is_null(){ panic!("Failed to create kblockd"); } blk_requestq_cachep=KMEM_CACHE(request_queue,SLAB_PANIC); blk_debugfs_root=debugfs_create_dir(c"block".as_ptr(),core::ptr::null_mut()); 0 }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
