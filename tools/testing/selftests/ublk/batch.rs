/* SPDX-License-Identifier: MIT */
/*
 * Description: UBLK_F_BATCH_IO buffer management
 */

use std::ffi::c_void;
use std::os::raw::{c_char, c_int, c_uint, c_ushort, c_uchar, c_ulong, c_longlong};

pub type __u16 = u16;
pub type __u32 = u32;
pub type __u64 = u64;

#[repr(C)]
pub struct ublk_dev_info {
    pub flags: __u32,
    pub queue_depth: __u16,
    pub nr_hw_queues: __u16,
}

#[repr(C)]
pub struct ublk_dev {
    pub dev_info: ublk_dev_info,
    pub q: *mut ublk_queue,
}

#[repr(C)]
pub struct ublk_batch_elem {
    pub tag: __u16,
    pub buf_index: __u16,
    pub buf_addr: __u64,
    pub result: c_int,
    pub reserved: __u32,
}

#[repr(C)]
pub struct ublk_batch_io {
    pub q_id: __u16,
    pub flags: __u16,
    pub reserved: __u16,
    pub elem_bytes: __u16,
    pub nr_elem: __u16,
}

#[repr(C)]
pub struct batch_commit_buf {
    pub buf_idx: __u16,
    pub elem: *mut c_void,
    pub done: __u16,
    pub count: __u16,
    pub q_id: __u16,
}

#[repr(C)]
pub struct ublk_fetch_buf {
    pub fetch_buf: *mut c_void,
    pub fetch_buf_size: __u32,
    pub br: *mut io_uring_buf_ring,
    pub fetch_buf_off: __u32,
}

#[repr(C)]
pub struct ublk_io {
    pub buf_addr: __u64,
}

#[repr(C)]
pub struct ublk_queue_ops {
    pub queue_io: Option<unsafe extern "C" fn(t: *mut ublk_thread, q: *mut ublk_queue, tag: __u16)>,
}

#[repr(C)]
pub struct pthread_spinlock_t {
    _private: [u8; 0],
}

#[repr(C)]
pub struct ublk_queue {
    pub q_id: __u16,
    pub q_depth: __u16,
    pub flags: c_uint,
    pub tgt_ops: *mut ublk_queue_ops,
    pub ios: *mut ublk_io,
    pub lock: pthread_spinlock_t,
}

#[repr(C)]
pub struct allocator {
    _private: [u8; 0],
}

#[repr(C)]
pub struct io_uring_buf_ring {
    _private: [u8; 0],
}

#[repr(C)]
pub struct io_uring {
    _private: [u8; 0],
}

#[repr(C)]
pub struct io_uring_sqe {
    pub opcode: u8,
    pub flags: u8,
    pub ioprio: u16,
    pub fd: c_uint,
    pub off: __u64,
    pub addr: __u64,
    pub len: __u32,
    pub rw_flags: c_uint,
    pub buf_group: __u16,
    pub personarity: __u16,
}

#[repr(C)]
pub struct io_uring_cqe {
    pub user_data: __u64,
    pub res: c_int,
    pub flags: c_uint,
}

#[repr(C)]
pub struct ublk_thread {
    pub dev: *mut ublk_dev,
    pub q_map: *mut c_uint,
    pub nr_bufs: __u16,
    pub commit_buf_start: __u16,
    pub nr_commit_buf: __u16,
    pub commit_buf_size: __u32,
    pub commit_buf_elem_size: __u16,
    pub commit_buf: *mut c_void,
    pub commit_buf_alloc: allocator,
    pub commit: *mut batch_commit_buf,
    pub nr_queues: c_uint,
    pub cmd_flags: __u16,
    pub state: __u32,
    pub nr_fetch_bufs: c_uint,
    pub fetch: *mut ublk_fetch_buf,
    pub cmd_inflight: __u32,
    pub idx: __u32,
    pub ring: io_uring,
}

pub const UBLKS_T_COMMIT_BUF_INV_IDX: __u16 = 0xffff;

pub const UBLK_F_SUPPORT_ZERO_COPY: __u32 = 0;
pub const UBLK_F_USER_COPY: __u32 = 0;
pub const UBLK_F_AUTO_BUF_REG: __u32 = 0;

pub const UBLK_BATCH_F_AUTO_BUF_REG_FALLBACK: __u16 = 1;
pub const UBLK_BATCH_F_HAS_BUF_ADDR: __u16 = 2;

pub const UBLK_U_IO_PREP_IO_CMDS: c_ulong = 0;
pub const UBLK_U_IO_COMMIT_IO_CMDS: c_ulong = 1;
pub const UBLK_U_IO_FETCH_IO_CMDS: c_ulong = 2;

pub const UBLK_MAX_QUEUES: usize = 1024;

pub const UBLK_Q_DEPTH_UNKNOWN: u16 = 0;

pub const IOSQE_FIXED_FILE: u8 = 1 << 0;
pub const IOSQE_BUFFER_SELECT: u8 = 1 << 1;
pub const IORING_OP_URING_CMD: u8 = 1;
pub const IORING_URING_CMD_MULTISHOT: c_uint = 1 << 0;
pub const IORING_CQE_F_MORE: c_uint = 1 << 1;
pub const IOU_PBUF_RING_INC: c_uint = 0;

pub const UBLKS_Q_PREPARED: c_uint = 1 << 0;
pub const UBLKS_T_BATCH_IO: __u32 = 1 << 0;
pub const UBLKS_T_STOPPING: __u32 = 1 << 1;

pub const ENOMEM: c_int = 12;
pub const ENOBUFS: c_int = 105;

pub const UBLK_DBG_IO_CMD: c_int = 0;

#[inline]
fn ioc_nr(op: c_ulong) -> c_uint {
    (op & 0xff) as c_uint
}

#[inline]
unsafe fn round_up(v: __u32, a: __u32) -> __u32 {
    if a == 0 { return v; }
    (v + a - 1) & !(a - 1)
}

extern "C" {
    fn ublk_assert(expr: c_int);
    fn ublk_err(fmt: *const c_char, ...);
    fn ublk_log(fmt: *const c_char, ...);
    fn ublk_dbg(level: c_int, fmt: *const c_char, ...);

    fn ublk_queue_use_auto_zc(q: *mut ublk_queue) -> c_int;
    fn ublk_queue_auto_zc_fallback(q: *mut ublk_queue) -> c_int;
    fn ublk_queue_no_buf(q: *mut ublk_queue) -> c_int;

    fn ublk_batch_io_buf_idx(t: *mut ublk_thread, q: *mut ublk_queue, tag: __u16) -> __u16;
    fn ublk_batch_io_buf_idx_next(t: *mut ublk_thread, q: *mut ublk_queue, tag: __u16) -> __u16;
    fn ublk_batch_commit_prepared(cb: *const batch_commit_buf) -> c_int;
    fn ublk_queue_idx_in_thread(t: *mut ublk_thread, q: *mut ublk_queue) -> usize;

    fn allocator_get(a: *mut allocator) -> c_int;
    fn allocator_get_val(a: *mut allocator, idx: c_ushort) -> c_int;
    fn allocator_put(a: *mut allocator, idx: c_ushort);
    fn allocator_init(a: *mut allocator, n: c_uint);
    fn allocator_deinit(a: *mut allocator);

    fn ublk_get_sqe_cmd(sqe: *mut io_uring_sqe) -> *mut c_void;
    fn ublk_set_sqe_cmd_op(sqe: *mut io_uring_sqe, op: c_ulong);
    fn ublk_io_alloc_sqes(t: *mut ublk_thread, sqe: *mut *mut io_uring_sqe, nr: c_int);

    fn io_uring_buf_ring_add(br: *mut io_uring_buf_ring, addr: *const c_void, len: __u32, bgid: c_uint, bid: c_uint, flags: c_uint);
    fn io_uring_buf_ring_advance(br: *mut io_uring_buf_ring, nr: c_uint);
    fn io_uring_free_buf_ring(ring: *mut io_uring, br: *mut io_uring_buf_ring, flags: c_uint, bgid: c_uint);
    fn io_uring_sqe_set_data64(sqe: *mut io_uring_sqe, data: __u64);
    fn io_uring_setup_buf_ring(ring: *mut io_uring, nr: c_uint, bgid: c_uint, flags: c_uint, ret: *mut c_int) -> *mut io_uring_buf_ring;

    fn build_user_data(buf_idx: __u16, op: c_uint, nr_elem: __u16, q_id: __u16, reserved: c_uint) -> __u64;
    fn user_data_to_tag(data: __u64) -> __u16;
    fn user_data_to_tgt_data(data: __u64) -> c_int;
    fn user_data_to_q_id(data: __u64) -> __u16;
    fn user_data_to_op(data: __u64) -> c_uint;

    fn strerror(errnum: c_int) -> *const c_char;
    fn __errno_location() -> *mut c_int;

    fn calloc(nmemb: usize, size: usize) -> *mut c_void;
    fn free(ptr: *mut c_void);
    fn posix_memalign(memptr: *mut *mut c_void, alignment: usize, size: usize) -> c_int;
    fn getpagesize() -> c_int;
    fn mlock(addr: *const c_void, len: usize) -> c_int;
    fn munlock(addr: *const c_void, len: usize) -> c_int;

    fn pthread_spin_lock(lock: *mut pthread_spinlock_t) -> c_int;
    fn pthread_spin_unlock(lock: *mut pthread_spinlock_t) -> c_int;
}

#[inline]
unsafe fn c_errno() -> c_int {
    *__errno_location()
}

#[inline]
unsafe fn ublk_get_commit_buf(t: *mut ublk_thread, buf_idx: __u16) -> *mut c_void {
    if buf_idx < (*t).commit_buf_start || buf_idx >= (*t).commit_buf_start.wrapping_add((*t).nr_commit_buf) {
        return core::ptr::null_mut();
    }

    let idx = buf_idx.wrapping_sub((*t).commit_buf_start) as usize;
    (*t)
        .commit_buf
        .cast::<u8>()
        .add(idx.saturating_mul((*t).commit_buf_size as usize))
        .cast::<c_void>()
}

#[inline]
unsafe fn ublk_alloc_commit_buf(t: *mut ublk_thread) -> __u16 {
    let idx = allocator_get(&mut (*t).commit_buf_alloc);
    if idx >= 0 {
        return idx as __u16 + (*t).commit_buf_start;
    }
    UBLKS_T_COMMIT_BUF_INV_IDX
}

#[inline]
unsafe fn ublk_free_commit_buf(t: *mut ublk_thread, i: __u16) {
    let idx = i.wrapping_sub((*t).commit_buf_start);

    ublk_assert((idx < (*t).nr_commit_buf) as c_int);
    ublk_assert((allocator_get_val(&mut (*t).commit_buf_alloc, idx) != 0) as c_int);
    allocator_put(&mut (*t).commit_buf_alloc, idx);
}

unsafe fn ublk_commit_elem_buf_size(dev: *mut ublk_dev) -> c_uchar {
    if ((*dev).dev_info.flags & (UBLK_F_SUPPORT_ZERO_COPY | UBLK_F_USER_COPY | UBLK_F_AUTO_BUF_REG)) != 0 {
        8
    } else {
        16
    }
}

unsafe fn ublk_commit_buf_size(t: *mut ublk_thread) -> __u32 {
    let elem_size = ublk_commit_elem_buf_size((*t).dev) as __u32;
    let total = elem_size.saturating_mul((*(*t).dev).dev_info.queue_depth as __u32);
    let page_sz = getpagesize() as __u32;
    round_up(total, page_sz)
}

unsafe fn free_batch_commit_buf(t: *mut ublk_thread) {
    if !(*t).commit_buf.is_null() {
        let buf_size = ublk_commit_buf_size(t);
        let total = buf_size.saturating_mul((*t).nr_commit_buf as __u32);
        munlock((*t).commit_buf, total as usize);
        free((*t).commit_buf);
    }

    allocator_deinit(&mut (*t).commit_buf_alloc);
    free((*t).commit as *mut c_void);
}

unsafe fn alloc_batch_commit_buf(t: *mut ublk_thread) -> c_int {
    let buf_size = ublk_commit_buf_size(t);
    let total = buf_size.saturating_mul((*t).nr_commit_buf as __u32);
    let page_sz = getpagesize() as usize;
    let mut buf: *mut c_void = core::ptr::null_mut();
    let mut i = 0usize;
    let mut j = 0usize;

    (*t).commit = calloc((*t).nr_queues as usize, core::mem::size_of::<batch_commit_buf>()) as *mut batch_commit_buf;

    while i < (*(*t).dev).dev_info.nr_hw_queues as usize {
        if !(*t).q_map.is_null() && *(*t).q_map.add(i) != 0 {
            (*(*t).commit.add(j)).q_id = i as __u16;
            j += 1;
        }
        i += 1;
    }

    allocator_init(&mut (*t).commit_buf_alloc, (*t).nr_commit_buf as c_uint);

    (*t).commit_buf = core::ptr::null_mut();
    let ret = posix_memalign(&mut buf, page_sz, total as usize);
    if ret != 0 || buf.is_null() {
        free_batch_commit_buf(t);
        return ret;
    }

    (*t).commit_buf = buf;

    if mlock((*t).commit_buf, total as usize) != 0 {
        ublk_err(b"%s: can't lock commit buffer %s\n\0".as_ptr() as *const c_char,
                 b"alloc_batch_commit_buf\0".as_ptr() as *const c_char,
                 strerror(c_errno()));
    }

    0
}

unsafe fn ublk_thread_nr_queues(t: *const ublk_thread) -> c_uint {
    let mut i = 0usize;
    let mut ret: c_uint = 0;

    while i < (*(*t).dev).dev_info.nr_hw_queues as usize {
        if !(*t).q_map.is_null() && *(*t).q_map.add(i) != 0 {
            ret = ret.saturating_add(1);
        }
        i += 1;
    }

    ret
}

pub unsafe fn ublk_batch_prepare(t: *mut ublk_thread) {
    let q = &mut *((*(*t).dev).q);

    (*t).nr_queues = ublk_thread_nr_queues(t);
    (*t).commit_buf_elem_size = ublk_commit_elem_buf_size((*t).dev);
    (*t).commit_buf_size = ublk_commit_buf_size(t);
    (*t).commit_buf_start = (*t).nr_bufs;
    (*t).nr_commit_buf = (2 * (*t).nr_queues) as __u16;
    (*t).nr_bufs = (*t).nr_bufs.wrapping_add((*t).nr_commit_buf);

    (*t).cmd_flags = 0;
    if ublk_queue_use_auto_zc(q) != 0 {
        if ublk_queue_auto_zc_fallback(q) != 0 {
            (*t).cmd_flags |= UBLK_BATCH_F_AUTO_BUF_REG_FALLBACK;
        }
    } else if ublk_queue_no_buf(q) == 0 {
        (*t).cmd_flags |= UBLK_BATCH_F_HAS_BUF_ADDR;
    }

    (*t).state |= UBLKS_T_BATCH_IO;

    ublk_log(
        b"%s: thread %d commit(nr_bufs %u, buf_size %u, start %u)\n\0".as_ptr() as *const c_char,
        b"ublk_batch_prepare\0".as_ptr() as *const c_char,
        (*t).idx,
        (*t).nr_commit_buf as c_uint,
        (*t).commit_buf_size,
        (*t).nr_bufs,
    );
}

unsafe fn free_batch_fetch_buf(t: *mut ublk_thread) {
    let mut i = 0usize;

    while i < (*t).nr_fetch_bufs as usize {
        io_uring_free_buf_ring(&mut (*t).ring, (*(*t).fetch.add(i)).br, 1, i as c_uint);
        munlock((*(*t).fetch.add(i)).fetch_buf, (*(*t).fetch.add(i)).fetch_buf_size as usize);
        free((*(*t).fetch.add(i)).fetch_buf);
        i += 1;
    }

    free((*t).fetch as *mut c_void);
}

unsafe fn alloc_batch_fetch_buf(t: *mut ublk_thread) -> c_int {
    let pg_sz = getpagesize() as __u32;
    let buf_size = round_up(((*(*t).dev).dev_info.queue_depth as __u32).saturating_mul(2), pg_sz);

    (*t).nr_fetch_bufs = (*t).nr_queues.saturating_mul(2);
    (*t).fetch = calloc((*t).nr_fetch_bufs as usize, core::mem::size_of::<ublk_fetch_buf>()) as *mut ublk_fetch_buf;

    let mut i = 0usize;
    while i < (*t).nr_fetch_bufs as usize {
        (*(*t).fetch.add(i)).fetch_buf_size = buf_size;

        if posix_memalign(
            &mut (*(*t).fetch.add(i)).fetch_buf,
            pg_sz as usize,
            (*(*t).fetch.add(i)).fetch_buf_size as usize,
        ) != 0 {
            return -ENOMEM;
        }

        if mlock((*(*t).fetch.add(i)).fetch_buf, (*(*t).fetch.add(i)).fetch_buf_size as usize) != 0 {
            ublk_err(
                b"%s: can't lock fetch buffer %s\n\0".as_ptr() as *const c_char,
                b"alloc_batch_fetch_buf\0".as_ptr() as *const c_char,
                strerror(c_errno()),
            );
        }

        let mut ret = 0;
        (*(*t).fetch.add(i)).br = io_uring_setup_buf_ring(
            &mut (*t).ring,
            1,
            i as c_uint,
            IOU_PBUF_RING_INC,
            &mut ret,
        );
        if (*(*t).fetch.add(i)).br.is_null() {
            ublk_err(
                b"Buffer ring register failed %d\n\0".as_ptr() as *const c_char,
                ret,
            );
            return ret;
        }
        i += 1;
    }

    0
}

pub unsafe fn ublk_batch_alloc_buf(t: *mut ublk_thread) -> c_int {
    let ret;

    ublk_assert(((*t).nr_commit_buf as usize) < 2 * UBLK_MAX_QUEUES);

    ret = alloc_batch_commit_buf(t);
    if ret != 0 {
        return ret;
    }
    alloc_batch_fetch_buf(t)
}

pub unsafe fn ublk_batch_free_buf(t: *mut ublk_thread) {
    free_batch_commit_buf(t);
    free_batch_fetch_buf(t);
}

unsafe fn ublk_init_batch_cmd(
    t: *mut ublk_thread,
    q_id: __u16,
    sqe: *mut io_uring_sqe,
    op: c_ulong,
    elem_bytes: __u16,
    nr_elem: __u16,
    buf_idx: __u16,
) {
    let cmd = ublk_get_sqe_cmd(sqe) as *mut ublk_batch_io;
    let user_data = build_user_data(buf_idx, op as c_uint, nr_elem, q_id, 0);

    ublk_set_sqe_cmd_op(sqe, op);

    (*sqe).fd = 0;
    (*sqe).opcode = IORING_OP_URING_CMD;
    (*sqe).flags |= IOSQE_FIXED_FILE;

    (*cmd).q_id = q_id;
    (*cmd).flags = 0;
    (*cmd).reserved = 0;
    (*cmd).elem_bytes = elem_bytes;
    (*cmd).nr_elem = nr_elem;

    io_uring_sqe_set_data64(sqe, user_data);

    (*t).cmd_inflight = (*t).cmd_inflight.wrapping_add(1);

    ublk_dbg(
        UBLK_DBG_IO_CMD,
        b"%s: thread %u qid %d cmd_op %x data %lx nr_elem %u elem_bytes %u buf_size %u buf_idx %d cmd_inflight %u\n\0".as_ptr() as *const c_char,
        b"ublk_init_batch_cmd\0".as_ptr() as *const c_char,
        (*t).idx,
        q_id,
        op,
        user_data,
        (*cmd).nr_elem as c_uint,
        (*cmd).elem_bytes as c_uint,
        (nr_elem as c_uint).saturating_mul(elem_bytes as c_uint),
        buf_idx,
        (*t).cmd_inflight,
    );
}

unsafe fn ublk_setup_commit_sqe(t: *mut ublk_thread, sqe: *mut io_uring_sqe, _buf_idx: __u16) {
    let cmd = ublk_get_sqe_cmd(sqe) as *mut ublk_batch_io;
    (*cmd).flags |= (*t).cmd_flags;
}

unsafe fn ublk_batch_queue_fetch(t: *mut ublk_thread, q: *mut ublk_queue, buf_idx: __u16) {
    let nr_elem = ((*(*t).fetch.add(buf_idx as usize)).fetch_buf_size / 2) as __u16;
    let mut sqe: *mut io_uring_sqe;

    io_uring_buf_ring_add(
        (*(*t).fetch.add(buf_idx as usize)).br,
        (*(*t).fetch.add(buf_idx as usize)).fetch_buf,
        (*(*t).fetch.add(buf_idx as usize)).fetch_buf_size,
        0,
        0,
        0,
    );
    io_uring_buf_ring_advance((*(*t).fetch.add(buf_idx as usize)).br, 1);

    ublk_io_alloc_sqes(t, &mut sqe, 1);

    ublk_init_batch_cmd(
        t,
        (*q).q_id,
        sqe,
        UBLK_U_IO_FETCH_IO_CMDS,
        2,
        nr_elem,
        buf_idx,
    );

    (*sqe).rw_flags = IORING_URING_CMD_MULTISHOT;
    (*sqe).buf_group = buf_idx;
    (*sqe).flags |= IOSQE_BUFFER_SELECT;

    (*(*t).fetch.add(buf_idx as usize)).fetch_buf_off = 0;
}

pub unsafe fn ublk_batch_start_fetch(t: *mut ublk_thread) {
    let mut i = 0usize;
    let mut j = 0u16;

    while i < (*(*t).dev).dev_info.nr_hw_queues as usize {
        if !(*t).q_map.is_null() && *(*t).q_map.add(i) != 0 {
            let q = (*(*t).dev).q.add(i);
            ublk_batch_queue_fetch(t, q, j);
            j = j.wrapping_add(1);
            ublk_batch_queue_fetch(t, q, j);
            j = j.wrapping_add(1);
        }
        i += 1;
    }
}

unsafe fn ublk_compl_batch_fetch(t: *mut ublk_thread, q: *mut ublk_queue, cqe: *const io_uring_cqe) -> __u16 {
    let buf_idx = user_data_to_tag((*cqe).user_data);
    let start = (*(*t).fetch.add(buf_idx as usize)).fetch_buf_off;
    let end = start.saturating_add((*cqe).res as c_uint);
    let mut i = start;

    if (*cqe).res < 0 {
        return buf_idx;
    }

    if (end - start) / 2 > (*q).q_depth as c_uint {
        ublk_err(
            b"%s: fetch duplicated ios offset %u count %u\n\0".as_ptr() as *const c_char,
            b"ublk_compl_batch_fetch\0".as_ptr() as *const c_char,
            start,
            (*cqe).res as c_uint,
        );
        while i < end {
            let tag = *(*(*t).fetch.add(buf_idx as usize)).fetch_buf.cast::<u8>().add(i as usize).cast::<__u16>();
            ublk_err(b"%u \0".as_ptr() as *const c_char, tag as c_uint);
            i = i.wrapping_add(2);
        }
        ublk_err(b"\n\0".as_ptr() as *const c_char);
        i = start;
    }

    while i < end {
        let tag = *(*(*t).fetch.add(buf_idx as usize)).fetch_buf.cast::<u8>().add(i as usize).cast::<__u16>();
        if tag >= (*q).q_depth {
            ublk_err(
                b"%s: bad tag %u\n\0".as_ptr() as *const c_char,
                b"ublk_compl_batch_fetch\0".as_ptr() as *const c_char,
                tag as c_uint,
            );
        }

        if !(*q).tgt_ops.is_null() {
            if let Some(queue_io) = (*(*q).tgt_ops).queue_io {
                queue_io(t, q, tag);
            }
        }

        i = i.wrapping_add(2);
    }

    (*(*t).fetch.add(buf_idx as usize)).fetch_buf_off = end;
    buf_idx
}

unsafe fn __ublk_batch_queue_prep_io_cmds(t: *mut ublk_thread, q: *mut ublk_queue) -> c_int {
    let nr_elem = (*q).q_depth;
    let buf_idx = ublk_alloc_commit_buf(t);
    let mut sqe: *mut io_uring_sqe;
    let buf;
    let mut i = 0u16;

    ublk_assert((buf_idx != UBLKS_T_COMMIT_BUF_INV_IDX) as c_int);

    ublk_io_alloc_sqes(t, &mut sqe, 1);

    ublk_assert((nr_elem == (*q).q_depth) as c_int);
    buf = ublk_get_commit_buf(t, buf_idx);

    while i < nr_elem {
        let elem = buf
            .cast::<u8>()
            .add(i as usize * (*t).commit_buf_elem_size as usize)
            .cast::<ublk_batch_elem>();
        let io = (*q).ios.add(i as usize);

        (*elem).tag = i;
        (*elem).result = 0;

        if ublk_queue_use_auto_zc(q) != 0 {
            (*elem).buf_index = ublk_batch_io_buf_idx(t, q, i);
        } else if ublk_queue_no_buf(q) == 0 {
            (*elem).buf_addr = (*io).buf_addr;
        }

        i = i.wrapping_add(1);
    }

    (*sqe).addr = buf as __u64;
    (*sqe).len = (*t).commit_buf_elem_size as c_uint * nr_elem as c_uint;

    ublk_init_batch_cmd(
        t,
        (*q).q_id,
        sqe,
        UBLK_U_IO_PREP_IO_CMDS,
        (*t).commit_buf_elem_size,
        nr_elem,
        buf_idx,
    );
    ublk_setup_commit_sqe(t, sqe, buf_idx);
    0
}

pub unsafe fn ublk_batch_queue_prep_io_cmds(t: *mut ublk_thread, q: *mut ublk_queue) -> c_int {
    let mut ret = 0;

    pthread_spin_lock(&mut (*q).lock);
    if (*q).flags & UBLKS_Q_PREPARED == 0 {
        ret = __ublk_batch_queue_prep_io_cmds(t, q);
        if ret == 0 {
            (*q).flags |= UBLKS_Q_PREPARED;
        }
    }
    pthread_spin_unlock(&mut (*q).lock);

    ret
}

unsafe fn ublk_batch_compl_commit_cmd(t: *mut ublk_thread, cqe: *const io_uring_cqe, op: c_uint) {
    let buf_idx = user_data_to_tag((*cqe).user_data);

    if op == ioc_nr(UBLK_U_IO_PREP_IO_CMDS) {
        ublk_assert(((*cqe).res == 0) as c_int);
    } else if op == ioc_nr(UBLK_U_IO_COMMIT_IO_CMDS) {
        let nr_elem = user_data_to_tgt_data((*cqe).user_data);
        ublk_assert(((*cqe).res == (*t).commit_buf_elem_size as c_int * nr_elem) as c_int);
    } else {
        ublk_assert(0);
    }

    ublk_free_commit_buf(t, buf_idx);
}

pub unsafe fn ublk_batch_compl_cmd(t: *mut ublk_thread, cqe: *const io_uring_cqe) {
    let op = user_data_to_op((*cqe).user_data);

    if op == ioc_nr(UBLK_U_IO_PREP_IO_CMDS) || op == ioc_nr(UBLK_U_IO_COMMIT_IO_CMDS) {
        (*t).cmd_inflight = (*t).cmd_inflight.wrapping_sub(1);
        ublk_batch_compl_commit_cmd(t, cqe, op);
        return;
    }

    let q_id = user_data_to_q_id((*cqe).user_data);
    let q = (*(*t).dev).q.add(q_id as usize);
    let buf_idx = ublk_compl_batch_fetch(t, q, cqe);

    if (*cqe).res < 0 && (*cqe).res != -ENOBUFS {
        (*t).cmd_inflight = (*t).cmd_inflight.wrapping_sub(1);
        (*t).state |= UBLKS_T_STOPPING;
    } else if ((*cqe).flags & IORING_CQE_F_MORE) == 0 || (*cqe).res == -ENOBUFS {
        (*t).cmd_inflight = (*t).cmd_inflight.wrapping_sub(1);
        ublk_batch_queue_fetch(t, q, buf_idx);
    }
}

unsafe fn __ublk_batch_commit_io_cmds(t: *mut ublk_thread, cb: *mut batch_commit_buf) {
    let mut sqe: *mut io_uring_sqe;
    let buf_idx = (*cb).buf_idx;
    let nr_elem = (*cb).done;

    if nr_elem == 0 {
        ublk_free_commit_buf(t, (*cb).buf_idx);
        return;
    }

    ublk_io_alloc_sqes(t, &mut sqe, 1);

    (*sqe).addr = (*cb).elem as c_longlong as __u64;
    (*sqe).len = nr_elem as c_uint * (*t).commit_buf_elem_size as c_uint;

    ublk_init_batch_cmd(
        t,
        (*cb).q_id,
        sqe,
        UBLK_U_IO_COMMIT_IO_CMDS,
        (*t).commit_buf_elem_size,
        nr_elem,
        buf_idx,
    );
    ublk_setup_commit_sqe(t, sqe, buf_idx);
}

pub unsafe fn ublk_batch_commit_io_cmds(t: *mut ublk_thread) {
    let mut i = 0usize;

    while i < (*t).nr_queues as usize {
        let cb = (*t).commit.add(i);
        if (*cb).buf_idx != UBLKS_T_COMMIT_BUF_INV_IDX {
            __ublk_batch_commit_io_cmds(t, cb);
        }
        i += 1;
    }
}

unsafe fn __ublk_batch_init_commit(t: *mut ublk_thread, cb: *mut batch_commit_buf, buf_idx: __u16) {
    (*cb).buf_idx = buf_idx;
    (*cb).elem = ublk_get_commit_buf(t, buf_idx);
    (*cb).done = 0;
    (*cb).count = ((*t).commit_buf_size / (*t).commit_buf_elem_size as __u32) as __u16;
}

unsafe fn ublk_batch_init_commit(t: *mut ublk_thread, cb: *mut batch_commit_buf) {
    let buf_idx = ublk_alloc_commit_buf(t);

    ublk_assert((buf_idx != UBLKS_T_COMMIT_BUF_INV_IDX) as c_int);
    ublk_assert((ublk_batch_commit_prepared(cb) == 0) as c_int);

    __ublk_batch_init_commit(t, cb, buf_idx);
}

pub unsafe fn ublk_batch_prep_commit(t: *mut ublk_thread) {
    let mut i = 0usize;

    while i < (*t).nr_queues as usize {
        (*(*t).commit.add(i)).buf_idx = UBLKS_T_COMMIT_BUF_INV_IDX;
        i += 1;
    }
}

pub unsafe fn ublk_batch_complete_io(t: *mut ublk_thread, q: *mut ublk_queue, tag: __u16, res: c_int) {
    let q_t_idx = ublk_queue_idx_in_thread(t, q);
    let cb = (*t).commit.add(q_t_idx);
    let elem: *mut ublk_batch_elem;
    let io = (*q).ios.add(tag as usize);

    if ublk_batch_commit_prepared(cb) == 0 {
        ublk_batch_init_commit(t, cb);
    }

    ublk_assert(((*q).q_id == (*cb).q_id) as c_int);

    elem = (*cb)
        .elem
        .cast::<u8>()
        .add((*cb).done as usize * (*t).commit_buf_elem_size as usize)
        .cast::<ublk_batch_elem>();

    (*elem).tag = tag;
    (*elem).buf_index = ublk_batch_io_buf_idx_next(t, q, tag);
    (*elem).result = res;

    if ublk_queue_no_buf(q) == 0 {
        (*elem).buf_addr = (*io).buf_addr;
    }

    (*cb).done = (*cb).done.wrapping_add(1);
    ublk_assert(((*cb).done <= (*cb).count) as c_int);
}

pub unsafe fn ublk_batch_setup_map(q_thread_map: *mut [c_uchar; UBLK_MAX_QUEUES], nthreads: c_int, queues: c_int) {
    let mut i = 0;
    let mut j = 0;

    while i < queues || j < nthreads {
        (*q_thread_map.add((j % nthreads) as usize))[(i % queues) as usize] = 1;
        i += 1;
        j += 1;
    }

    j = 0;
    while j < nthreads {
        let mut seq: c_uchar = 1;
        i = 0;
        while i < queues {
            if (*q_thread_map.add(j as usize))[i as usize] != 0 {
                (*q_thread_map.add(j as usize))[i as usize] = seq;
                seq = seq.wrapping_add(1);
            }
            i += 1;
        }
        j += 1;
    }

    /*
    for (j = 0; j < nthreads; j++) {
        printf("thread %0d: ", j);
        for (i = 0; i < queues; i++) {
            if (q_thread_map[j][i])
                printf("%03u ", i);
        }
        printf("\n");
    }
    printf("\n");
    for (j = 0; j < nthreads; j++) {
        for (i = 0; i < queues; i++) {
            printf("%03u ", q_thread_map[j][i]);
        }
        printf("\n");
    }
    */
}
