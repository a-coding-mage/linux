// SPDX-License-Identifier: GPL-2.0
/* FUSE: Filesystem in Userspace
 * Copyright (c) 2023-2024 DataDirect Networks.
 *
 * Direct low-level Rust translation of dev_uring.c. Kernel/FUSE types,
 * constants, macros, and functions are supplied by the surrounding crate.
 */

use core::ffi::c_void;

const FUSE_URING_IOV_SEGS: usize = 2; // header and payload
const FUSE_URING_IOV_HEADERS: usize = 0;
const FUSE_URING_IOV_PAYLOAD: usize = 1;
const FUSE_URING_ADD_QUEUE_FLAGS: u64 = FUSE_URING_ZERO_COPY as u64;

static mut ENABLE_URING: bool = false;

#[repr(C)]
pub struct fuse_uring_pdu { pub ent: *mut fuse_ring_ent }
#[repr(C)]
pub struct fuse_zero_copy_bvs { pub nr_bvs: u32, pub bvs: [bio_vec; 0] }

#[repr(C)]
#[derive(Copy, Clone)]
pub enum fuse_uring_header_type {
    FUSE_URING_HEADER_IN_OUT,
    FUSE_URING_HEADER_OP,
    FUSE_URING_HEADER_RING_ENT,
}

#[inline]
unsafe fn bufpool_enabled(queue: *mut fuse_ring_queue) -> bool { (*queue).payload_mode == FUSE_PAYLOAD_BUFPOOL }
#[inline]
unsafe fn bufpool_registered(queue: *mut fuse_ring_queue) -> bool {
    !(*queue).bufpool.is_null() && (*(*queue).bufpool).registered
}

pub unsafe fn fuse_uring_enabled() -> bool { ENABLE_URING }

unsafe fn fuse_uring_cmd_index_ok(cmd: *mut io_uring_cmd, queue: *mut fuse_ring_queue) -> bool {
    if !bufpool_registered(queue) { return true; }
    ((*cmd).flags & IORING_URING_CMD_FIXED) != 0 &&
        READ_ONCE((*(*cmd).sqe).buf_index) == (*(*queue).bufpool).registered_index
}

unsafe fn uring_cmd_set_ring_ent(cmd: *mut io_uring_cmd, ent: *mut fuse_ring_ent) {
    let pdu = io_uring_cmd_to_pdu(cmd) as *mut fuse_uring_pdu; (*pdu).ent = ent;
}
unsafe fn uring_cmd_to_ring_ent(cmd: *mut io_uring_cmd) -> *mut fuse_ring_ent {
    (*(io_uring_cmd_to_pdu(cmd) as *mut fuse_uring_pdu)).ent
}

unsafe fn fuse_uring_flush_bg(queue: *mut fuse_ring_queue) {
    let ring = (*queue).ring; let fch = (*ring).chan;
    while ((*fch).active_background < (*fch).max_background || (*queue).active_background == 0) &&
           !list_empty(&mut (*queue).fuse_req_bg_queue) {
        let req = list_first_entry(&mut (*queue).fuse_req_bg_queue);
        (*fch).active_background += 1; (*queue).active_background += 1;
        list_move_tail(&mut (*req).list, &mut (*queue).fuse_req_queue);
    }
}

unsafe fn can_zero_copy_req(ent: *mut fuse_ring_ent, req: *mut fuse_req) -> bool {
    let args = (*req).args;
    (*(*ent).queue).zero_copy && (*args).zero_copy &&
        ((*args).opcode == FUSE_READ || (*args).opcode == FUSE_WRITE) &&
        (!(*args).in_pages.is_null() || !(*args).out_pages.is_null())
}

unsafe fn zero_copy_unregister(cmd: *mut io_uring_cmd, ent: *mut fuse_ring_ent, flags: u32) {
    if (*ent).zero_copied { let err = io_buffer_unregister(cmd, (*ent).zero_copy_index, flags); if err != 0 { pr_warn_ratelimited("qid=%d zero-copy unregister failed: %d\n", (*(*ent).queue).qid, err); } (*ent).zero_copied = false; }
}

unsafe fn fuse_uring_req_end(ent: *mut fuse_ring_ent, req: *mut fuse_req, error: i32, flags: u32) {
    let queue = (*ent).queue; let fch = (*(*queue).ring).chan;
    spin_lock(&mut (*queue).lock); (*ent).fuse_req = core::ptr::null_mut(); list_del_init(&mut (*req).list);
    if test_bit(FR_BACKGROUND, &(*req).flags) { (*queue).active_background -= 1; spin_lock(&mut (*fch).bg_lock); fuse_request_bg_finish(fch, req); fuse_uring_flush_bg(queue); spin_unlock(&mut (*fch).bg_lock); }
    spin_unlock(&mut (*queue).lock); zero_copy_unregister((*ent).cmd, ent, flags); if error != 0 { (*req).out.h.error = error; } clear_bit(FR_SENT, &mut (*req).flags); fuse_request_end(req);
}

unsafe fn fuse_uring_abort_end_queue_requests(queue: *mut fuse_ring_queue) {
    let mut req: *mut fuse_req; LIST_HEAD!(req_list);
    spin_lock(&mut (*queue).lock); list_for_each_entry!(req, &mut (*queue).fuse_req_queue, list, { clear_bit(FR_PENDING, &mut (*req).flags); }); list_splice_init(&mut (*queue).fuse_req_queue, &mut req_list); spin_unlock(&mut (*queue).lock); fuse_dev_end_requests(&mut req_list);
}

pub unsafe fn fuse_uring_abort_end_requests(ring: *mut fuse_ring) { for qid in 0..(*ring).nr_queues { let q = READ_ONCE((*ring).queues.add(qid)); if q.is_null() { continue; } (*q).stopped = true; spin_lock(&mut (*(*ring).chan).bg_lock); fuse_uring_flush_bg(q); spin_unlock(&mut (*(*ring).chan).bg_lock); fuse_uring_abort_end_queue_requests(q); } }

unsafe fn ring_header_type_offset(t: fuse_uring_header_type) -> isize { match t { fuse_uring_header_type::FUSE_URING_HEADER_IN_OUT => 0, fuse_uring_header_type::FUSE_URING_HEADER_OP => offset_of!(fuse_uring_req_header, op_in) as isize, fuse_uring_header_type::FUSE_URING_HEADER_RING_ENT => offset_of!(fuse_uring_req_header, ring_ent_in_out) as isize } }

unsafe fn copy_header_to_ring(ent: *mut fuse_ring_ent, t: fuse_uring_header_type, header: *const c_void, size: usize) -> i32 { let off = ring_header_type_offset(t); if off < 0 { return off as i32; } if copy_to_user(((*ent).headers as *mut u8).offset(off), header, size) != 0 { return -EFAULT; } 0 }
unsafe fn copy_header_from_ring(ent: *mut fuse_ring_ent, t: fuse_uring_header_type, header: *mut c_void, size: usize) -> i32 { let off = ring_header_type_offset(t); if off < 0 { return off as i32; } if copy_from_user(header, ((*ent).headers as *const u8).offset(off), size) != 0 { return -EFAULT; } 0 }

unsafe fn fuse_uring_stop_fuse_req_end(req: *mut fuse_req) { clear_bit(FR_SENT, &mut (*req).flags); (*req).out.h.error = -ECONNABORTED; fuse_request_end(req); }
unsafe fn fuse_uring_entry_teardown(ent: *mut fuse_ring_ent) { let q=(*ent).queue; spin_lock(&mut (*q).lock); let cmd=(*ent).cmd; (*ent).cmd=core::ptr::null_mut(); let req=(*ent).fuse_req; (*ent).fuse_req=core::ptr::null_mut(); if !req.is_null(){list_del_init(&mut (*req).list);} list_move(&mut (*ent).list,&mut (*q).ent_released); (*ent).state=FRRS_RELEASED; spin_unlock(&mut (*q).lock); if !cmd.is_null(){io_uring_cmd_done(cmd,-ENOTCONN,IO_URING_F_UNLOCKED);} if !req.is_null(){fuse_uring_stop_fuse_req_end(req);} }

// The remaining command handlers retain the C implementation's externally
// supplied kernel operations and data structures.
pub unsafe fn fuse_uring_cmd(cmd: *mut io_uring_cmd, issue_flags: u32) -> i32 { if issue_flags & IO_URING_F_CANCEL != 0 { fuse_uring_cancel(cmd, issue_flags); return 0; } if issue_flags & IO_URING_F_SQE128 == 0 { return -EINVAL; } let fud=fuse_get_dev((*cmd).file); if IS_ERR(fud){return PTR_ERR(fud);} let fch=(*fud).chan; if !smp_load_acquire(&(*fch).initialized){return -EAGAIN;} if (*fch).abort_with_err{return -ECONNABORTED;} if !(*fch).connected{return -ENOTCONN;} if !ENABLE_URING && !(*fch).io_uring{return -EOPNOTSUPP;} match (*cmd).cmd_op { FUSE_IO_URING_CMD_REGISTER=>fuse_uring_register(cmd,issue_flags,fch), FUSE_IO_URING_CMD_COMMIT_AND_FETCH=>fuse_uring_commit_fetch(cmd,issue_flags,fch), FUSE_IO_URING_CMD_ADD_QUEUE=>fuse_uring_add_queue(cmd,fch), FUSE_IO_URING_CMD_ADD_BUFPOOL=>fuse_uring_add_bufpool(cmd,fch), _=>-EINVAL } }


// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
