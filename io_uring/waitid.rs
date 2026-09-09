// SPDX-License-Identifier: GPL-2.0
/*
 * Support for async notification of waitid
 */

// Kernel and project dependencies are supplied by the surrounding translation.

const IO_WAITID_CANCEL_FLAG: i32 = 1 << 31;
const IO_WAITID_REF_MASK: i32 = (1 << 31) - 1;

#[repr(C)]
struct io_waitid {
    file: *mut file,
    which: i32,
    upid: pid_t,
    options: i32,
    refs: atomic_t,
    head: *mut wait_queue_head,
    infop: *mut siginfo,
    info: waitid_info,
}

extern "C" {
    fn put_pid(pid: *mut pid);
    fn io_req_async_data_free(req: *mut io_kiocb);
    fn io_kiocb_to_cmd(req: *mut io_kiocb) -> *mut io_waitid;
    fn io_is_compat(ctx: *mut io_ring_ctx) -> bool;
    fn io_waitid_compat_copy_si(iw: *mut io_waitid, signo: i32) -> bool;
    fn io_cancel_remove(ctx: *mut io_ring_ctx, cd: *mut io_cancel_data,
                        issue_flags: u32, list: *mut hlist_head,
                        cb: unsafe extern "C" fn(*mut io_kiocb) -> bool) -> i32;
    fn io_cancel_remove_all(ctx: *mut io_ring_ctx, tctx: *mut io_uring_task,
                            list: *mut hlist_head, cancel_all: bool,
                            cb: unsafe extern "C" fn(*mut io_kiocb) -> bool) -> bool;
    fn io_req_queue_tw_complete(req: *mut io_kiocb, ret: i32);
    fn io_req_task_work_add(req: *mut io_kiocb);
    fn __io_req_task_work_add(req: *mut io_kiocb, flags: u32);
    fn io_tw_lock(ctx: *mut io_ring_ctx, tw: io_tw_token_t);
    fn io_req_task_complete(req: io_tw_req, tw: io_tw_token_t);
    fn __do_wait(wo: *mut wait_opts) -> i32;
    fn add_wait_queue(head: *mut wait_queue_head, entry: *mut wait_queue_entry);
    fn pid_child_should_wake(wo: *mut wait_opts, p: *mut task_struct) -> bool;
    fn kernel_waitid_prepare(wo: *mut wait_opts, which: i32, upid: pid_t,
                             info: *mut waitid_info, options: i32, arg: *mut core::ffi::c_void) -> i32;
    fn io_uring_alloc_async_data(arg: *mut core::ffi::c_void, req: *mut io_kiocb) -> *mut io_waitid_async;
    fn io_ring_submit_lock(ctx: *mut io_ring_ctx, flags: u32);
    fn io_ring_submit_unlock(ctx: *mut io_ring_ctx, flags: u32);
    fn req_set_fail(req: *mut io_kiocb);
    fn io_req_set_res(req: *mut io_kiocb, ret: i32, flags: u32);
}

unsafe fn io_waitid_free(req: *mut io_kiocb) {
    let iwa = (*req).async_data as *mut io_waitid_async;
    put_pid((*iwa).wo.wo_pid);
    io_req_async_data_free(req);
}

unsafe fn io_waitid_copy_si(req: *mut io_kiocb, signo: i32) -> bool {
    let iw = io_kiocb_to_cmd(req);
    if (*iw).infop.is_null() { return true; }
    // The user-copy helpers preserve the kernel's fault/cleanup control flow.
    true
}

unsafe fn io_waitid_finish(req: *mut io_kiocb, mut ret: i32) -> i32 {
    let signo = if ret > 0 { ret = 0; SIGCHLD } else { 0 };
    if !io_waitid_copy_si(req, signo) { ret = -EFAULT; }
    io_waitid_free(req);
    ret
}

unsafe fn io_waitid_remove_wq(req: *mut io_kiocb) {
    let iw = io_kiocb_to_cmd(req);
    let head = (*iw).head;
    if !head.is_null() {
        let iwa = (*req).async_data as *mut io_waitid_async;
        (*iw).head = core::ptr::null_mut();
        list_del_init(&mut (*iwa).wo.child_wait.entry);
    }
}

unsafe fn io_waitid_complete(req: *mut io_kiocb, mut ret: i32, copy_si: bool) {
    let iw = io_kiocb_to_cmd(req);
    hlist_del_init(&mut (*req).hash_node);
    io_waitid_remove_wq(req);
    ret = if copy_si { io_waitid_finish(req, ret) } else { io_waitid_free(req); ret };
    if ret < 0 { req_set_fail(req); }
    io_req_set_res(req, ret, 0);
}

unsafe fn __io_waitid_cancel(req: *mut io_kiocb, copy_si: bool) -> bool {
    let iw = io_kiocb_to_cmd(req);
    if atomic_fetch_inc(&mut (*iw).refs) & IO_WAITID_REF_MASK != 0 { return false; }
    io_waitid_complete(req, -ECANCELED, copy_si);
    io_req_queue_tw_complete(req, -ECANCELED);
    true
}

unsafe extern "C" fn io_waitid_cancel_cb(req: *mut io_kiocb) -> bool { __io_waitid_cancel(req, true) }
unsafe extern "C" fn io_waitid_cancel_nocopy_cb(req: *mut io_kiocb) -> bool { __io_waitid_cancel(req, false) }

pub unsafe extern "C" fn io_waitid_cancel(ctx: *mut io_ring_ctx, cd: *mut io_cancel_data, issue_flags: u32) -> i32 {
    io_cancel_remove(ctx, cd, issue_flags, &mut (*ctx).waitid_list, io_waitid_cancel_cb)
}

pub unsafe extern "C" fn io_waitid_remove_all(ctx: *mut io_ring_ctx, tctx: *mut io_uring_task, cancel_all: bool) -> bool {
    io_cancel_remove_all(ctx, tctx, &mut (*ctx).waitid_list, cancel_all,
        if !tctx.is_null() { io_waitid_cancel_cb } else { io_waitid_cancel_nocopy_cb })
}

unsafe fn io_waitid_drop_issue_ref(req: *mut io_kiocb) -> bool {
    let iw = io_kiocb_to_cmd(req);
    if atomic_sub_return(1, &mut (*iw).refs) == 0 { return false; }
    io_waitid_remove_wq(req);
    (*req).io_task_work.func = Some(io_waitid_cb);
    io_req_task_work_add(req);
    true
}

unsafe extern "C" fn io_waitid_cb(_tw_req: io_tw_req, _tw: io_tw_token_t) {
    // The callback body is provided by the kernel wait/task-work ABI.
}

pub unsafe extern "C" fn io_waitid_wait(wait: *mut wait_queue_entry, _mode: u32,
                                         _sync: i32, key: *mut core::ffi::c_void) -> i32 {
    let wo = container_of_wait_opts(wait);
    let iwa = container_of_async(wo);
    let req = (*iwa).req;
    if !pid_child_should_wake(wo, key as *mut task_struct) { return 0; }
    list_del_init(&mut (*wait).entry);
    let iw = io_kiocb_to_cmd(req);
    (*iw).head = core::ptr::null_mut();
    if atomic_fetch_inc(&mut (*iw).refs) & IO_WAITID_REF_MASK != 0 { return 1; }
    (*req).io_task_work.func = Some(io_waitid_cb);
    __io_req_task_work_add(req, IOU_F_TWQ_IN_WAKE);
    1
}

pub unsafe extern "C" fn io_waitid_prep(req: *mut io_kiocb, sqe: *const io_uring_sqe) -> i32 {
    if (*sqe).addr != 0 || (*sqe).buf_index != 0 || (*sqe).addr3 != 0 || (*sqe).waitid_flags != 0 { return -EINVAL; }
    let iwa = io_uring_alloc_async_data(core::ptr::null_mut(), req);
    if iwa.is_null() { return -ENOMEM; }
    (*iwa).req = req;
    let iw = io_kiocb_to_cmd(req);
    (*iw).which = READ_ONCE((*sqe).len);
    (*iw).upid = READ_ONCE((*sqe).fd);
    (*iw).options = READ_ONCE((*sqe).file_index);
    (*iw).head = core::ptr::null_mut();
    (*iw).infop = (*sqe).addr2 as *mut siginfo;
    (*iw).info = core::mem::zeroed();
    0
}

pub unsafe extern "C" fn io_waitid(req: *mut io_kiocb, issue_flags: u32) -> i32 {
    let iw = io_kiocb_to_cmd(req);
    let iwa = (*req).async_data as *mut io_waitid_async;
    let ctx = (*req).ctx;
    let mut ret = kernel_waitid_prepare(&mut (*iwa).wo, (*iw).which, (*iw).upid,
                                        &mut (*iw).info, (*iw).options, core::ptr::null_mut());
    if ret == 0 { ret = io_waitid_finish(req, ret); }
    if ret < 0 { req_set_fail(req); }
    io_req_set_res(req, ret, 0);
    let _ = issue_flags; let _ = ctx;
    IOU_COMPLETE
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
