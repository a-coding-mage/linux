// SPDX-License-Identifier: GPL-2.0-only
/*
 * Copyright 1993 by Theodore Ts'o.
 */
// Linux kernel headers and symbols are supplied by other translation units.

#[repr(C)]
pub enum LoopState {
    Lo_unbound,
    Lo_bound,
    Lo_rundown,
    Lo_deleting,
}

#[repr(C)]
pub struct loop_device {
    pub lo_number: i32,
    pub lo_offset: i64,
    pub lo_sizelimit: i64,
    pub lo_flags: i32,
    pub lo_file_name: [u8; LO_NAME_SIZE],
    pub lo_backing_file: *mut file,
    pub lo_min_dio_size: u32,
    pub lo_dio_mem_align: u32,
    pub lo_device: *mut block_device,
    pub old_gfp_mask: gfp_t,
    pub lo_lock: spinlock_t,
    pub lo_state: i32,
    pub lo_work_lock: spinlock_t,
    pub workqueue: *mut workqueue_struct,
    pub rootcg_work: work_struct,
    pub rootcg_cmd_list: list_head,
    pub idle_worker_list: list_head,
    pub worker_tree: rb_root,
    pub timer: timer_list,
    pub sysfs_inited: bool,
    pub lo_queue: *mut request_queue,
    pub tag_set: blk_mq_tag_set,
    pub lo_disk: *mut gendisk,
    pub lo_mutex: mutex,
    pub idr_visible: bool,
}

#[repr(C)]
pub struct loop_cmd {
    pub list_entry: list_head,
    pub use_aio: bool,
    pub ref_: atomic_t,
    pub ret: isize,
    pub iocb: kiocb,
    pub bvec: *mut bio_vec,
    pub blkcg_css: *mut cgroup_subsys_state,
    pub memcg_css: *mut cgroup_subsys_state,
}

pub const LOOP_IDLE_WORKER_TIMEOUT: u32 = 60 * HZ;
pub const LOOP_DEFAULT_HW_Q_DEPTH: u32 = 128;

static mut loop_index_idr: idr = unsafe { core::mem::zeroed() };
static mut loop_ctl_mutex: mutex = unsafe { core::mem::zeroed() };
static mut loop_validate_mutex: mutex = unsafe { core::mem::zeroed() };

unsafe fn loop_global_lock_killable(lo: *mut loop_device, global: bool) -> i32 {
    let mut err: i32;
    if global {
        err = mutex_lock_killable(&raw mut loop_validate_mutex);
        if err != 0 { return err; }
    }
    err = mutex_lock_killable(&mut (*lo).lo_mutex);
    if err != 0 && global { mutex_unlock(&raw mut loop_validate_mutex); }
    err
}

unsafe fn loop_global_unlock(lo: *mut loop_device, global: bool) {
    mutex_unlock(&mut (*lo).lo_mutex);
    if global { mutex_unlock(&raw mut loop_validate_mutex); }
}

static mut max_part: i32 = 0;
static mut part_shift: i32 = 0;

unsafe fn lo_calculate_size(lo: *mut loop_device, file_: *mut file) -> i64 {
    let mut loopsize: i64;
    let mut ret: i32;
    if S_ISBLK((*file_inode(file_)).i_mode) {
        loopsize = i_size_read((*file_).f_mapping.host);
    } else {
        let mut stat: kstat = core::mem::zeroed();
        ret = vfs_getattr_nosec(&(*file_).f_path, &mut stat, STATX_SIZE, 0);
        if ret != 0 { return 0; }
        loopsize = stat.size;
    }
    if (*lo).lo_offset > 0 { loopsize -= (*lo).lo_offset; }
    if loopsize < 0 { return 0; }
    if (*lo).lo_sizelimit > 0 && (*lo).lo_sizelimit < loopsize { loopsize = (*lo).lo_sizelimit; }
    loopsize >> 9
}

unsafe fn lo_can_use_dio(lo: *mut loop_device) -> bool {
    if (*(*lo).lo_backing_file).f_mode & FMODE_CAN_ODIRECT == 0 { return false; }
    if queue_logical_block_size((*lo).lo_queue) < (*lo).lo_min_dio_size { return false; }
    if (*lo).lo_offset & ((*lo).lo_min_dio_size as i64 - 1) != 0 { return false; }
    true
}

#[inline]
unsafe fn loop_update_dio(lo: *mut loop_device) {
    lockdep_assert_held(&mut (*lo).lo_mutex);
    WARN_ON_ONCE((*lo).lo_state == Lo_bound && (*(*lo).lo_queue).mq_freeze_depth == 0);
    if (*lo).lo_flags & LO_FLAGS_DIRECT_IO != 0 && !lo_can_use_dio(lo) { (*lo).lo_flags &= !LO_FLAGS_DIRECT_IO; }
}

unsafe fn loop_set_size(lo: *mut loop_device, size: i64) {
    if !set_capacity_and_notify((*lo).lo_disk, size) { kobject_uevent(&mut (*disk_to_dev((*lo).lo_disk)).kobj, KOBJ_CHANGE); }
}

unsafe fn loop_clear_limits(lo: *mut loop_device, mode: i32) {
    let mut lim = queue_limits_start_update((*lo).lo_queue);
    if mode & FALLOC_FL_ZERO_RANGE != 0 { lim.max_write_zeroes_sectors = 0; }
    if mode & FALLOC_FL_PUNCH_HOLE != 0 { lim.max_hw_discard_sectors = 0; lim.discard_granularity = 0; }
    queue_limits_commit_update((*lo).lo_queue, &mut lim);
}

unsafe fn lo_fallocate(lo: *mut loop_device, rq: *mut request, pos: i64, mut mode: i32) -> i32 {
    let file_ = (*lo).lo_backing_file;
    mode |= FALLOC_FL_KEEP_SIZE;
    if bdev_max_discard_sectors((*lo).lo_device) == 0 { return -EOPNOTSUPP; }
    let mut ret = ((*(*file_).f_op).fallocate)(file_, mode, pos, blk_rq_bytes(rq));
    if ret != 0 && ret != -EINVAL && ret != -EOPNOTSUPP { ret = -EIO; }
    if ret == -EOPNOTSUPP { loop_clear_limits(lo, mode); }
    ret
}

unsafe fn lo_req_flush(lo: *mut loop_device, _rq: *mut request) -> i32 {
    let mut ret = vfs_fsync((*lo).lo_backing_file, 0);
    if ret != 0 && ret != -EINVAL { ret = -EIO; }
    ret
}

unsafe fn lo_complete_rq(rq: *mut request) {
    let cmd = blk_mq_rq_to_pdu(rq) as *mut loop_cmd;
    let mut ret = BLK_STS_OK;
    if (*cmd).ret < 0 || (*cmd).ret == blk_rq_bytes(rq) as isize || req_op(rq) != REQ_OP_READ {
        if (*cmd).ret < 0 { ret = errno_to_blk_status((*cmd).ret as i32); }
        blk_mq_end_request(rq, ret); return;
    }
    if (*cmd).ret != 0 {
        blk_update_request(rq, BLK_STS_OK, (*cmd).ret as u32);
        (*cmd).ret = 0; blk_mq_requeue_request(rq, true);
    } else {
        let mut bio = (*rq).bio;
        while !bio.is_null() { zero_fill_bio(bio); bio = (*bio).bi_next; }
        blk_mq_end_request(rq, BLK_STS_IOERR);
    }
}

// The remaining declarations preserve the source interface; kernel-provided types and helpers
// are intentionally left external to this isolated translation unit.
unsafe extern "C" {
    static HZ: u32;
}

unsafe fn lo_rw_aio_do_completion(cmd: *mut loop_cmd) {
    let rq = blk_mq_rq_from_pdu(cmd);
    if !atomic_dec_and_test(&mut (*cmd).ref_) { return; }
    kfree((*cmd).bvec as *mut core::ffi::c_void); (*cmd).bvec = core::ptr::null_mut();
    if req_op(rq) == REQ_OP_WRITE { kiocb_end_write(&mut (*cmd).iocb); }
    if !blk_should_fake_timeout((*rq).q) { blk_mq_complete_request(rq); }
}

unsafe extern "C" fn lo_rw_aio_complete(iocb: *mut kiocb, ret: isize) {
    let cmd = container_of(iocb, loop_cmd, iocb);
    (*cmd).ret = ret; lo_rw_aio_do_completion(cmd);
}

unsafe fn lo_rw_aio(lo: *mut loop_device, cmd: *mut loop_cmd, pos: i64, rw: i32) -> i32 {
    let rq = blk_mq_rq_from_pdu(cmd); let file_ = (*lo).lo_backing_file;
    let nr_bvec = blk_rq_nr_bvec(rq); let mut iter: iov_iter = core::mem::zeroed();
    let mut rq_iter: req_iterator = core::mem::zeroed();
    if (*rq).bio != (*rq).biotail {
        (*cmd).bvec = kmalloc_objs((*cmd).bvec, nr_bvec, GFP_NOIO);
        if (*cmd).bvec.is_null() { return -EIO; }
        let mut bvec = (*cmd).bvec; let mut tmp: bio_vec = core::mem::zeroed();
        rq_for_each_bvec!(tmp, rq, rq_iter, { *bvec = tmp; bvec = bvec.add(1); });
        iov_iter_bvec(&mut iter, rw, (*cmd).bvec, nr_bvec, blk_rq_bytes(rq)); iter.iov_offset = 0;
    } else {
        iov_iter_bvec(&mut iter, rw, __bvec_iter_bvec((*(*rq).bio).bi_io_vec, (*(*rq).bio).bi_iter), nr_bvec, blk_rq_bytes(rq));
        iter.iov_offset = (*(*rq).bio).bi_iter.bi_offset;
    }
    atomic_set(&mut (*cmd).ref_, 2); (*cmd).iocb.ki_pos = pos; (*cmd).iocb.ki_filp = file_; (*cmd).iocb.ki_ioprio = req_get_ioprio(rq);
    if (*cmd).use_aio { (*cmd).iocb.ki_complete = Some(lo_rw_aio_complete); (*cmd).iocb.ki_flags = IOCB_DIRECT; }
    else { (*cmd).iocb.ki_complete = None; (*cmd).iocb.ki_flags = 0; }
    let ret = if rw == ITER_SOURCE { kiocb_start_write(&mut (*cmd).iocb); ((*(*file_).f_op).write_iter)(&mut (*cmd).iocb, &mut iter) } else { ((*(*file_).f_op).read_iter)(&mut (*cmd).iocb, &mut iter) };
    lo_rw_aio_do_completion(cmd); if ret != -EIOCBQUEUED { lo_rw_aio_complete(&mut (*cmd).iocb, ret); } -EIOCBQUEUED
}

unsafe fn do_req_filebacked(lo: *mut loop_device, rq: *mut request) -> i32 {
    let cmd = blk_mq_rq_to_pdu(rq) as *mut loop_cmd; let pos = ((blk_rq_pos(rq) as i64) << 9) + (*lo).lo_offset;
    match req_op(rq) { REQ_OP_FLUSH => lo_req_flush(lo, rq), REQ_OP_WRITE_ZEROES => lo_fallocate(lo, rq, pos, if (*rq).cmd_flags & REQ_NOUNMAP != 0 { FALLOC_FL_ZERO_RANGE } else { FALLOC_FL_PUNCH_HOLE }), REQ_OP_DISCARD => lo_fallocate(lo, rq, pos, FALLOC_FL_PUNCH_HOLE), REQ_OP_WRITE => lo_rw_aio(lo, cmd, pos, ITER_SOURCE), REQ_OP_READ => lo_rw_aio(lo, cmd, pos, ITER_DEST), _ => { WARN_ON_ONCE(true); -EIO } }
}

unsafe fn loop_reread_partitions(lo: *mut loop_device) { mutex_lock(&mut (*(*lo).lo_disk).open_mutex); let rc = bdev_disk_changed((*lo).lo_disk, false); mutex_unlock(&mut (*(*lo).lo_disk).open_mutex); if rc != 0 { pr_warn!("partition scan failed"); } }
unsafe fn is_loop_device(file_: *mut file) -> bool { let i = (*file_inode(file_)); !i.is_null() && S_ISBLK((*i).i_mode) && imajor(i) == LOOP_MAJOR }
unsafe fn loop_check_backing_file(file_: *mut file) -> i32 { if (*(*file_).f_op).read_iter.is_none() { return -EINVAL; } if (*file_).f_mode & FMODE_WRITE != 0 && (*(*file_).f_op).write_iter.is_none() { return -EINVAL; } 0 }

unsafe fn loop_attr_offset_show(lo: *mut loop_device, buf: *mut u8) -> isize { sysfs_emit(buf, "%llu\n", (*lo).lo_offset as u64) }
unsafe fn loop_attr_sizelimit_show(lo: *mut loop_device, buf: *mut u8) -> isize { sysfs_emit(buf, "%llu\n", (*lo).lo_sizelimit as u64) }
unsafe fn loop_attr_autoclear_show(lo: *mut loop_device, buf: *mut u8) -> isize { sysfs_emit(buf, "%s\n", if (*lo).lo_flags & LO_FLAGS_AUTOCLEAR != 0 { "1" } else { "0" }) }
unsafe fn loop_attr_partscan_show(lo: *mut loop_device, buf: *mut u8) -> isize { sysfs_emit(buf, "%s\n", if (*lo).lo_flags & LO_FLAGS_PARTSCAN != 0 { "1" } else { "0" }) }
unsafe fn loop_attr_dio_show(lo: *mut loop_device, buf: *mut u8) -> isize { sysfs_emit(buf, "%s\n", if (*lo).lo_flags & LO_FLAGS_DIRECT_IO != 0 { "1" } else { "0" }) }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
