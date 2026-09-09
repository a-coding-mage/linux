/* SPDX-License-Identifier: GPL-2.0 */
// Translation of trace/events/io_uring.h.  C tracepoint registration and
// printk formatting are represented by the entry layouts and assignment
// functions below; the referenced kernel types and constants are external.

use core::ffi::c_void;

#[repr(C)]
pub struct io_wq_work;
#[repr(C)]
pub struct io_kiocb { pub ctx: *mut io_ring_ctx, pub cqe: io_uring_cqe, pub flags: u64, pub opcode: u8, pub work: io_wq_work }
#[repr(C)]
pub struct io_ring_ctx { pub flags: u32 }
#[repr(C)]
pub struct io_uring_cqe { pub user_data: u64, pub res: i32, pub flags: u32, pub big_cqe: [u64; 2] }
#[repr(C)]
pub struct io_uring_sqe {
    pub user_data: u64, pub opcode: u8, pub flags: u8, pub ioprio: u8,
    pub off: u64, pub addr: u64, pub len: u32, pub poll32_events: u32,
    pub buf_index: u16, pub personality: u16, pub file_index: u32,
    pub __pad2: [u64; 1], pub addr3: u64,
}

extern "C" {
    pub fn io_uring_get_opcode(opcode: u8) -> *const core::ffi::c_char;
}

pub const IORING_SETUP_CQE32: u32 = 1 << 10;
pub const IORING_SETUP_SQPOLL: u32 = 1 << 1;
pub const IORING_CQE_F_32: u32 = 1 << 1;

#[repr(C)] pub struct io_uring_create_entry { pub fd: i32, pub ctx: *mut c_void, pub sq_entries: u32, pub cq_entries: u32, pub flags: u32 }
#[repr(C)] pub struct io_uring_register_entry { pub ctx: *mut c_void, pub opcode: u32, pub nr_files: u32, pub nr_bufs: u32, pub ret: i64 }
#[repr(C)] pub struct io_uring_file_get_entry { pub ctx: *mut c_void, pub req: *mut c_void, pub user_data: u64, pub fd: i32 }
#[repr(C)] pub struct io_uring_queue_async_work_entry { pub ctx: *mut c_void, pub req: *mut c_void, pub user_data: u64, pub opcode: u8, pub flags: u64, pub work: *mut io_wq_work, pub hashed: bool, pub op_str: *const core::ffi::c_char }
#[repr(C)] pub struct io_uring_defer_entry { pub ctx: *mut c_void, pub req: *mut c_void, pub data: u64, pub opcode: u8, pub op_str: *const core::ffi::c_char }
#[repr(C)] pub struct io_uring_link_entry { pub ctx: *mut c_void, pub req: *mut c_void, pub target_req: *mut c_void }
#[repr(C)] pub struct io_uring_cqring_wait_entry { pub ctx: *mut c_void, pub min_events: i32 }
#[repr(C)] pub struct io_uring_fail_link_entry { pub ctx: *mut c_void, pub req: *mut c_void, pub user_data: u64, pub opcode: u8, pub link: *mut c_void, pub op_str: *const core::ffi::c_char }
#[repr(C)] pub struct io_uring_complete_entry { pub ctx: *mut c_void, pub req: *mut c_void, pub user_data: u64, pub res: i32, pub cflags: u32, pub extra1: u64, pub extra2: u64 }
#[repr(C)] pub struct io_uring_submit_req_entry { pub ctx: *mut c_void, pub req: *mut c_void, pub user_data: u64, pub opcode: u8, pub flags: u64, pub sq_thread: bool, pub op_str: *const core::ffi::c_char }
#[repr(C)] pub struct io_uring_poll_arm_entry { pub ctx: *mut c_void, pub req: *mut c_void, pub user_data: u64, pub opcode: u8, pub mask: i32, pub events: i32, pub op_str: *const core::ffi::c_char }
#[repr(C)] pub struct io_uring_task_add_entry { pub ctx: *mut c_void, pub req: *mut c_void, pub user_data: u64, pub opcode: u8, pub mask: i32, pub op_str: *const core::ffi::c_char }
#[repr(C)] pub struct io_uring_req_failed_entry { pub ctx: *mut c_void, pub req: *mut c_void, pub user_data: u64, pub opcode: u8, pub flags: u8, pub ioprio: u8, pub off: u64, pub addr: u64, pub len: u32, pub op_flags: u32, pub buf_index: u16, pub personality: u16, pub file_index: u32, pub pad1: u64, pub addr3: u64, pub error: i32, pub op_str: *const core::ffi::c_char }
#[repr(C)] pub struct io_uring_cqe_overflow_entry { pub ctx: *mut c_void, pub user_data: u64, pub res: i32, pub cflags: u32, pub ocqe: *mut c_void }
#[repr(C)] pub struct io_uring_task_work_run_entry { pub tctx: *mut c_void, pub count: u32 }
#[repr(C)] pub struct io_uring_short_write_entry { pub ctx: *mut c_void, pub fpos: u64, pub wanted: u64, pub got: u64 }
#[repr(C)] pub struct io_uring_local_work_run_entry { pub ctx: *mut c_void, pub count: i32, pub loops: u32 }

pub unsafe fn io_uring_complete_assign(e: &mut io_uring_complete_entry, ctx: *mut io_ring_ctx, req: *mut c_void, cqe: *const io_uring_cqe) {
    e.ctx = ctx as *mut c_void; e.req = req; e.user_data = (*cqe).user_data; e.res = (*cqe).res; e.cflags = (*cqe).flags;
    let extended = ((*ctx).flags & IORING_SETUP_CQE32) != 0 || ((*cqe).flags & IORING_CQE_F_32) != 0;
    e.extra1 = if extended { (*cqe).big_cqe[0] } else { 0 }; e.extra2 = if extended { (*cqe).big_cqe[1] } else { 0 };
}

pub unsafe fn io_uring_create_assign(e: &mut io_uring_create_entry, fd: i32, ctx: *mut c_void, sq_entries: u32, cq_entries: u32, flags: u32) { e.fd=fd; e.ctx=ctx; e.sq_entries=sq_entries; e.cq_entries=cq_entries; e.flags=flags; }
pub unsafe fn io_uring_register_assign(e: &mut io_uring_register_entry, ctx: *mut c_void, opcode: u32, nr_files: u32, nr_bufs: u32, ret: i64) { e.ctx=ctx; e.opcode=opcode; e.nr_files=nr_files; e.nr_bufs=nr_bufs; e.ret=ret; }
pub unsafe fn io_uring_file_get_assign(e: &mut io_uring_file_get_entry, req: *mut io_kiocb, fd: i32) { e.ctx=(*req).ctx as *mut c_void; e.req=req as *mut c_void; e.user_data=(*req).cqe.user_data; e.fd=fd; }
pub unsafe fn io_uring_queue_async_work_assign(e: &mut io_uring_queue_async_work_entry, req: *mut io_kiocb, hashed: bool) { e.ctx=(*req).ctx as *mut c_void; e.req=req as *mut c_void; e.user_data=(*req).cqe.user_data; e.flags=(*req).flags; e.opcode=(*req).opcode; e.work=&mut (*req).work; e.hashed=hashed; e.op_str=io_uring_get_opcode((*req).opcode); }
pub unsafe fn io_uring_defer_assign(e: &mut io_uring_defer_entry, req: *mut io_kiocb) { e.ctx=(*req).ctx as *mut c_void; e.req=req as *mut c_void; e.data=(*req).cqe.user_data; e.opcode=(*req).opcode; e.op_str=io_uring_get_opcode((*req).opcode); }
pub unsafe fn io_uring_link_assign(e: &mut io_uring_link_entry, req: *mut io_kiocb, target_req: *mut io_kiocb) { e.ctx=(*req).ctx as *mut c_void; e.req=req as *mut c_void; e.target_req=target_req as *mut c_void; }
pub unsafe fn io_uring_cqring_wait_assign(e: &mut io_uring_cqring_wait_entry, ctx: *mut c_void, min_events: i32) { e.ctx=ctx; e.min_events=min_events; }
pub unsafe fn io_uring_fail_link_assign(e: &mut io_uring_fail_link_entry, req: *mut io_kiocb, link: *mut io_kiocb) { e.ctx=(*req).ctx as *mut c_void; e.req=req as *mut c_void; e.user_data=(*req).cqe.user_data; e.opcode=(*req).opcode; e.link=link as *mut c_void; e.op_str=io_uring_get_opcode((*req).opcode); }
pub unsafe fn io_uring_submit_req_assign(e: &mut io_uring_submit_req_entry, req: *mut io_kiocb) { e.ctx=(*req).ctx as *mut c_void; e.req=req as *mut c_void; e.user_data=(*req).cqe.user_data; e.opcode=(*req).opcode; e.flags=(*req).flags; e.sq_thread=((*req).ctx).flags & IORING_SETUP_SQPOLL != 0; e.op_str=io_uring_get_opcode((*req).opcode); }
pub unsafe fn io_uring_poll_arm_assign(e: &mut io_uring_poll_arm_entry, req: *mut io_kiocb, mask: i32, events: i32) { e.ctx=(*req).ctx as *mut c_void; e.req=req as *mut c_void; e.user_data=(*req).cqe.user_data; e.opcode=(*req).opcode; e.mask=mask; e.events=events; e.op_str=io_uring_get_opcode((*req).opcode); }
pub unsafe fn io_uring_task_add_assign(e: &mut io_uring_task_add_entry, req: *mut io_kiocb, mask: i32) { e.ctx=(*req).ctx as *mut c_void; e.req=req as *mut c_void; e.user_data=(*req).cqe.user_data; e.opcode=(*req).opcode; e.mask=mask; e.op_str=io_uring_get_opcode((*req).opcode); }
pub unsafe fn io_uring_req_failed_assign(e: &mut io_uring_req_failed_entry, sqe: *const io_uring_sqe, req: *mut io_kiocb, error: i32) { e.ctx=(*req).ctx as *mut c_void; e.req=req as *mut c_void; e.user_data=(*sqe).user_data; e.opcode=(*sqe).opcode; e.flags=(*sqe).flags; e.ioprio=(*sqe).ioprio; e.off=(*sqe).off; e.addr=(*sqe).addr; e.len=(*sqe).len; e.op_flags=(*sqe).poll32_events; e.buf_index=(*sqe).buf_index; e.personality=(*sqe).personality; e.file_index=(*sqe).file_index; e.pad1=(*sqe).__pad2[0]; e.addr3=(*sqe).addr3; e.error=error; e.op_str=io_uring_get_opcode((*sqe).opcode); }
pub unsafe fn io_uring_cqe_overflow_assign(e: &mut io_uring_cqe_overflow_entry, ctx: *mut c_void, user_data: u64, res: i32, cflags: u32, ocqe: *mut c_void) { e.ctx=ctx; e.user_data=user_data; e.res=res; e.cflags=cflags; e.ocqe=ocqe; }
pub unsafe fn io_uring_task_work_run_assign(e: &mut io_uring_task_work_run_entry, tctx: *mut c_void, count: u32) { e.tctx=tctx; e.count=count; }
pub unsafe fn io_uring_short_write_assign(e: &mut io_uring_short_write_entry, ctx: *mut c_void, fpos: u64, wanted: u64, got: u64) { e.ctx=ctx; e.fpos=fpos; e.wanted=wanted; e.got=got; }
pub unsafe fn io_uring_local_work_run_assign(e: &mut io_uring_local_work_run_entry, ctx: *mut c_void, count: i32, loops: u32) { e.ctx=ctx; e.count=count; e.loops=loops; }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
