/* SPDX-License-Identifier: GPL-2.0-or-later */

// Dependencies supplied by the surrounding kernel translation.

pub const IORING_URING_CMD_CANCELABLE: u32 = 1u32 << 30;
pub const IORING_URING_CMD_REISSUE: u32 = 1u32 << 31;

#[repr(C)]
pub struct io_uring_cmd {
    pub file: *mut file,
    pub sqe: *const io_uring_sqe,
    pub cmd_op: u32,
    pub flags: u32,
    pub pdu: [u8; 32],
    pub unused: [u8; 8],
}

#[inline]
pub unsafe fn io_uring_sqe128_cmd<T>(sqe: *const io_uring_sqe) -> *const T {
    // BUILD_BUG_ON(sizeof(T) > ((2 * sizeof(struct io_uring_sqe)) -
    //                            offsetof(struct io_uring_sqe, cmd)));
    (*sqe).cmd.as_ptr() as *const T
}

#[inline]
pub unsafe fn io_uring_sqe_cmd<T>(sqe: *const io_uring_sqe) -> *const T {
    // BUILD_BUG_ON(sizeof(T) > (sizeof(struct io_uring_sqe) -
    //                            offsetof(struct io_uring_sqe, cmd)));
    (*sqe).cmd.as_ptr() as *const T
}

#[inline]
pub const fn io_uring_cmd_private_sz_check(_cmd_sz: usize) {
    // BUILD_BUG_ON(cmd_sz > sizeof_field(struct io_uring_cmd, pdu));
}

#[inline]
pub unsafe fn io_uring_cmd_to_pdu<T>(cmd: *mut io_uring_cmd) -> *mut T {
    io_uring_cmd_private_sz_check(core::mem::size_of::<T>());
    (*cmd).pdu.as_mut_ptr() as *mut T
}

#[cfg(feature = "CONFIG_IO_URING")]
extern "C" {
    pub fn io_uring_cmd_import_fixed(
        ubuf: u64, len: libc::c_ulong, rw: libc::c_int,
        iter: *mut iov_iter, ioucmd: *mut io_uring_cmd,
        issue_flags: libc::c_uint,
    ) -> libc::c_int;
    pub fn io_uring_cmd_import_fixed_vec(
        ioucmd: *mut io_uring_cmd, uvec: *const iovec, uvec_segs: usize,
        ddir: libc::c_int, iter: *mut iov_iter, issue_flags: libc::c_uint,
    ) -> libc::c_int;
    pub fn __io_uring_cmd_done(cmd: *mut io_uring_cmd, ret: i32, res2: u64,
                               issue_flags: libc::c_uint, is_cqe32: bool);
    pub fn __io_uring_cmd_do_in_task(ioucmd: *mut io_uring_cmd,
                                     task_work_cb: io_req_tw_func_t,
                                     flags: libc::c_uint);
    pub fn io_uring_cmd_mark_cancelable(cmd: *mut io_uring_cmd,
                                        issue_flags: libc::c_uint);
    pub fn io_uring_cmd_issue_blocking(ioucmd: *mut io_uring_cmd);
    pub fn io_uring_cmd_buffer_select(ioucmd: *mut io_uring_cmd,
                                      buf_group: libc::c_uint, len: *mut usize,
                                      issue_flags: libc::c_uint) -> io_br_sel;
    pub fn io_uring_mshot_cmd_post_cqe(ioucmd: *mut io_uring_cmd,
                                       sel: *mut io_br_sel,
                                       issue_flags: libc::c_uint) -> bool;
    pub fn io_buffer_register_request(cmd: *mut io_uring_cmd, rq: *mut request,
                                      release: Option<unsafe extern "C" fn(*mut libc::c_void)>,
                                      index: libc::c_uint, issue_flags: libc::c_uint) -> libc::c_int;
    pub fn io_buffer_register_bvec(cmd: *mut io_uring_cmd, bvs: *const bio_vec,
                                   nr_bvecs: libc::c_uint,
                                   release: Option<unsafe extern "C" fn(*mut libc::c_void)>,
                                   priv_: *mut libc::c_void, dir: u8,
                                   index: libc::c_uint, issue_flags: libc::c_uint) -> libc::c_int;
    pub fn io_buffer_unregister(cmd: *mut io_uring_cmd, index: libc::c_uint,
                                issue_flags: libc::c_uint) -> libc::c_int;
}

#[inline]
pub unsafe fn io_uring_cmd_from_tw(tw_req: io_tw_req) -> *mut io_uring_cmd {
    io_kiocb_to_cmd::<io_uring_cmd>(tw_req.req)
}

pub const IO_URING_CMD_TASK_WORK_ISSUE_FLAGS: u32 = IO_URING_F_COMPLETE_DEFER;

#[inline]
pub unsafe fn io_uring_cmd_do_in_task_lazy(cmd: *mut io_uring_cmd,
                                           cb: io_req_tw_func_t) {
    __io_uring_cmd_do_in_task(cmd, cb, IOU_F_TWQ_LAZY_WAKE);
}

#[inline]
pub unsafe fn io_uring_cmd_complete_in_task(cmd: *mut io_uring_cmd,
                                             cb: io_req_tw_func_t) {
    __io_uring_cmd_do_in_task(cmd, cb, 0);
}

#[inline]
pub unsafe fn io_uring_cmd_get_task(cmd: *mut io_uring_cmd) -> *mut task_struct {
    (*cmd_to_io_kiocb(cmd)).tctx.task
}

#[inline]
pub unsafe fn io_uring_cmd_ctx_handle(cmd: *mut io_uring_cmd) -> *mut libc::c_void {
    (*cmd_to_io_kiocb(cmd)).ctx
}

#[inline]
pub unsafe fn io_uring_cmd_done(cmd: *mut io_uring_cmd, ret: i32,
                                issue_flags: libc::c_uint) {
    __io_uring_cmd_done(cmd, ret, 0, issue_flags, false);
}

#[inline]
pub unsafe fn io_uring_cmd_done32(cmd: *mut io_uring_cmd, ret: i32, res2: u64,
                                  issue_flags: libc::c_uint) {
    __io_uring_cmd_done(cmd, ret, res2, issue_flags, true);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
