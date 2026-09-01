// SPDX-License-Identifier: GPL-2.0

use std::mem::size_of;
use std::os::raw::{c_char, c_int, c_long, c_uint, c_uchar, c_ushort, c_void};
use std::ptr;

type loff_t = i64;
type off64_t = i64;
type io_uring_op = c_int;

// MAX_BACK_FILES is supplied by the environment headers.
pub const MAX_BACK_FILES: usize = 32;
pub const NR_STRIPE: usize = MAX_BACK_FILES;

pub const UBLK_IO_OP_READ: c_int = 0;
pub const UBLK_IO_OP_WRITE: c_int = 1;
pub const UBLK_IO_OP_FLUSH: c_int = 2;
pub const UBLK_IO_OP_WRITE_ZEROES: c_int = 3;
pub const UBLK_IO_OP_DISCARD: c_int = 4;

pub const UBLK_U_IO_REGISTER_IO_BUF: c_int = 100;
pub const UBLK_U_IO_UNREGISTER_IO_BUF: c_int = 101;

pub const UBLK_PARAM_TYPE_BASIC: c_uint = 1;
pub const UBLK_ATTR_VOLATILE_CACHE: c_uint = 1;

pub const UBLK_F_SUPPORT_ZERO_COPY: c_uint = 0x1;
pub const UBLK_DBG_IO: c_int = 0;

pub const IOSQE_FIXED_FILE: c_ushort = 0x001;
pub const IOSQE_CQE_SKIP_SUCCESS: c_ushort = 0x0002;
pub const IOSQE_IO_HARDLINK: c_ushort = 0x0004;

pub const IORING_OP_READV: io_uring_op = 1;
pub const IORING_OP_READV_FIXED: io_uring_op = 2;
pub const IORING_OP_WRITEV: io_uring_op = 3;
pub const IORING_OP_WRITEV_FIXED: io_uring_op = 4;

pub const IORING_FSYNC_DATASYNC: c_int = 1;

pub const ENOTSUP: c_int = 95;
pub const EINVAL: c_int = 22;
pub const EIO: c_int = 5;

#[repr(C)]
#[derive(Clone, Copy)]
pub struct option {
    pub name: *const c_char,
    pub has_arg: c_int,
    pub flag: *mut c_int,
    pub val: c_int,
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct iovec {
    pub iov_base: *mut c_void,
    pub iov_len: usize,
}

#[repr(C)]
pub struct stripe_conf {
    pub nr_files: c_uint,
    pub shift: c_uint,
}

#[repr(C)]
pub struct stripe {
    pub start: loff_t,
    pub nr_sects: c_uint,
    pub seq: c_int,
    pub vec: *mut iovec,
    pub nr_vec: c_uint,
    pub cap: c_uint,
}

#[repr(C)]
pub struct stripe_array {
    pub s: [stripe; NR_STRIPE],
    pub nr: c_uint,
    pub _vec: [iovec; 0],
}

#[repr(C)]
pub struct ublk_basic_params {
    pub attrs: c_uint,
    pub logical_bs_shift: c_uchar,
    pub physical_bs_shift: c_uchar,
    pub io_opt_shift: c_uchar,
    pub io_min_shift: c_uchar,
    pub max_sectors: c_uint,
    pub dev_sectors: loff_t,
}

#[repr(C)]
pub struct ublk_params {
    pub types: c_uint,
    pub basic: ublk_basic_params,
}

#[repr(C)]
pub struct ublk_tgt_ctx {
    pub nr_backing_files: c_uint,
    pub backing_file_size: *mut loff_t,
    pub dev_size: loff_t,
    pub sq_depth: c_uint,
    pub cq_depth: c_uint,
    pub params: ublk_params,
}

#[repr(C)]
pub struct dev_ctx_stripe {
    pub chunk_size: c_uint,
}

#[repr(C)]
pub struct dev_ctx {
    pub auto_zc_fallback: c_int,
    pub metadata_size: c_uint,
    pub stripe: dev_ctx_stripe,
}

#[repr(C)]
pub struct ublk_tgt_dev_info {
    pub max_io_buf_bytes: c_uint,
    pub queue_depth: c_uint,
    pub flags: c_uint,
}

#[repr(C)]
pub struct ublk_dev {
    pub dev_info: ublk_tgt_dev_info,
    pub tgt: ublk_tgt_ctx,
    pub nr_fds: c_uint,
    pub private_data: *mut c_void,
}

#[repr(C)]
pub struct ublk_queue {
    pub dev: *mut ublk_dev,
    pub q_id: c_uint,
}

#[repr(C)]
pub struct ublksrv_io_desc {
    pub op_flags: c_uint,
    pub start_sector: loff_t,
    pub nr_sectors: c_uint,
}

#[repr(C)]
pub struct ublk_io {
    pub buf_addr: *mut c_void,
    pub result: c_int,
    pub private_data: *mut c_void,
    pub tgt_ios: c_uint,
}

#[repr(C)]
pub struct io_uring_cqe {
    pub user_data: u64,
    pub res: c_int,
}

#[repr(C)]
pub struct io_uring_sqe {
    pub opcode: c_uchar,
    pub flags: c_ushort,
    pub ioprio: c_ushort,
    pub fd: c_int,
    pub off: u64,
    pub addr: u64,
    pub len: c_uint,
    pub op_flags: c_uint,
    pub rw_flags: c_uint,
    pub user_data: u64,
    pub buf_index: c_ushort,
    pub personality: c_ushort,
    pub cmd_op: c_int,
}

#[repr(C)]
pub struct ublk_tgt_ops {
    pub name: *const c_char,
    pub init_tgt: Option<unsafe extern "C" fn(*const dev_ctx, *mut ublk_dev) -> c_int>,
    pub deinit_tgt: Option<unsafe extern "C" fn(*mut ublk_dev)>,
    pub queue_io: Option<unsafe extern "C" fn(*mut ublk_thread, *mut ublk_queue, c_int) -> c_int>,
    pub tgt_io_done: Option<unsafe extern "C" fn(*mut ublk_thread, *mut ublk_queue, *const io_uring_cqe)>,
    pub parse_cmd_line: Option<unsafe extern "C" fn(*mut dev_ctx, c_int, *mut *mut c_char)>,
    pub usage: Option<unsafe extern "C" fn(*const ublk_tgt_ops)>,
}

#[repr(C)]
pub struct ublk_thread;

#[allow(improper_ctypes)]
unsafe extern "C" {
    pub fn malloc(size: usize) -> *mut c_void;
    pub fn free(ptr: *mut c_void);

    pub fn printf(format: *const c_char, ...) -> c_int;
    pub fn strcmp(s1: *const c_char, s2: *const c_char) -> c_int;
    pub fn strtol(s: *const c_char, endptr: *mut *mut c_char, base: c_int) -> c_long;

    pub static mut optarg: *mut c_char;

    pub fn getopt_long(
        argc: c_int,
        argv: *mut *mut c_char,
        optstring: *const c_char,
        longopts: *const option,
        longind: *mut c_int,
    ) -> c_int;

    pub fn ublk_queue_use_auto_zc(q: *mut ublk_queue) -> c_int;
    pub fn ublk_queue_use_zc(q: *mut ublk_queue) -> c_int;
    pub fn ublk_io_alloc_sqes(t: *mut ublk_thread, sqe: *mut *mut io_uring_sqe, nr: c_int);
    pub fn ublk_get_io(q: *mut ublk_queue, tag: c_int) -> *mut ublk_io;
    pub fn ublk_get_iod(q: *mut ublk_queue, tag: c_int) -> *const ublksrv_io_desc;
    pub fn ublksrv_get_op(iod: *const ublksrv_io_desc) -> c_int;
    pub fn ublk_cmd_op_nr(op: c_int) -> c_int;
    pub fn build_user_data(tag: c_int, op: c_int, tgt: c_uint, q_id: c_uint, x: c_int) -> u64;
    pub fn user_data_to_tag(v: u64) -> c_uint;
    pub fn user_data_to_op(v: u64) -> c_uint;
    pub fn user_data_to_tgt_data(v: u64) -> c_uint;
    pub fn ublk_queued_tgt_io(t: *mut ublk_thread, q: *mut ublk_queue, tag: c_int, nr: c_int);
    pub fn ublk_completed_tgt_io(t: *mut ublk_thread, q: *mut ublk_queue, tag: c_uint) -> c_int;
    pub fn ublk_complete_io(t: *mut ublk_thread, q: *mut ublk_queue, tag: c_uint, res: c_int);
    pub fn ublk_io_buf_idx(t: *mut ublk_thread, q: *mut ublk_queue, tag: c_int) -> c_ushort;
    pub fn ublk_assert(cond: c_int);
    pub fn ublk_err(format: *const c_char, ...);
    pub fn ublk_dbg(level: c_int, format: *const c_char, ...);

    pub fn io_uring_prep_rw(
        op: io_uring_op,
        sqe: *mut io_uring_sqe,
        fd: c_int,
        iovecs: *mut iovec,
        nr_vec: c_uint,
        offset: u64,
    );
    pub fn io_uring_sqe_set_flags(sqe: *mut io_uring_sqe, flags: c_ushort);
    pub fn io_uring_prep_fsync(sqe: *mut io_uring_sqe, fd: c_int, fsync_flags: c_int);
    pub fn io_uring_prep_buf_register(
        sqe: *mut io_uring_sqe,
        q: *mut ublk_queue,
        tag: c_int,
        q_id: c_uint,
        buf_idx: c_ushort,
    );
    pub fn io_uring_prep_buf_unregister(
        sqe: *mut io_uring_sqe,
        q: *mut ublk_queue,
        tag: c_int,
        q_id: c_uint,
        buf_idx: c_ushort,
    );

    pub fn backing_file_tgt_init(dev: *mut ublk_dev, nr: c_uint) -> c_int;
    pub fn backing_file_tgt_deinit(dev: *mut ublk_dev);
    pub fn ilog2(v: c_uint) -> c_int;
}

fn get_chunk_shift(q: *const ublk_queue) -> *const stripe_conf {
    unsafe { (*(*q).dev).private_data as *const stripe_conf }
}

unsafe fn calculate_nr_vec(conf: *const stripe_conf, iod: *const ublksrv_io_desc) -> c_uint {
    let shift = (*conf).shift.wrapping_sub(9);
    let unit_sects = (*conf).nr_files << shift;
    let start = (*iod).start_sector;
    let end = start + (*iod).nr_sectors as loff_t;

    ((end / unit_sects as loff_t) - (start / unit_sects as loff_t) + 1) as c_uint
}

#[inline]
unsafe fn stripe_array_vec_base(s: *mut stripe_array) -> *mut iovec {
    (s as *mut u8).add(size_of::<stripe_array>()) as *mut iovec
}

fn alloc_stripe_array(conf: *const stripe_conf, iod: *const ublksrv_io_desc) -> *mut stripe_array {
    let nr_vecs = unsafe { calculate_nr_vec(conf, iod) };
    let total = (nr_vecs as usize).wrapping_mul(unsafe { (*conf).nr_files as usize });
    let size = size_of::<stripe_array>() + total.saturating_mul(size_of::<iovec>());
    let s = unsafe { malloc(size) as *mut stripe_array };

    unsafe {
        (*s).nr = 0;
        let mut i = 0;
        while i < (*conf).nr_files as c_int {
            let t = &mut (*s).s[i as usize];
            t.nr_vec = 0;
            t.vec = stripe_array_vec_base(s).add(i as usize * nr_vecs as usize);
            t.nr_sects = 0;
            t.cap = nr_vecs;
            i += 1;
        }
    }

    s
}

unsafe fn free_stripe_array(s: *mut stripe_array) {
    free(s as *mut c_void);
}

unsafe fn calculate_stripe_array(
    conf: *const stripe_conf,
    iod: *const ublksrv_io_desc,
    s: *mut stripe_array,
    base: *mut c_void,
) {
    let shift = (*conf).shift.wrapping_sub(9);
    let chunk_sects = 1u32 << shift;
    let unit_sects = (*conf).nr_files << shift;
    let mut start = (*iod).start_sector;
    let end = start + (*iod).nr_sectors as loff_t;
    let mut done: usize = 0;
    let mut idx = 0usize;

    while start < end {
        let mut nr_sects = chunk_sects - (start as u64 & (chunk_sects as u64 - 1)) as u32;
        let unit_off = (start / unit_sects as loff_t) * unit_sects as loff_t;
        let seq = ((start - unit_off) >> shift) as c_int;
        let this = &mut (*s).s[idx];
        let stripe_off =
            (unit_off / (*conf).nr_files as loff_t) + (start & ((chunk_sects - 1) as loff_t));

        if nr_sects as loff_t > end - start {
            nr_sects = (end - start) as u32;
        }

        if this.nr_sects == 0 {
            this.nr_sects = nr_sects;
            this.start = stripe_off;
            this.seq = seq;
            (*s).nr = (*s).nr.wrapping_add(1);
        } else {
            ublk_assert((seq == this.seq) as c_int);
            ublk_assert((this.start + this.nr_sects as loff_t == stripe_off) as c_int);
            this.nr_sects = this.nr_sects.wrapping_add(nr_sects);
        }

        ublk_assert((this.nr_vec < this.cap) as c_int);
        let vec = this.vec.add(this.nr_vec as usize);
        (*vec).iov_base = (base as *mut u8).add(done) as *mut c_void;
        (*vec).iov_len = (nr_sects as usize) << 9;
        this.nr_vec = this.nr_vec.wrapping_add(1);

        start += nr_sects as loff_t;
        done += (nr_sects as usize) << 9;
        idx = (idx + 1) % (*conf).nr_files as usize;
    }
}

unsafe fn stripe_to_uring_op(iod: *const ublksrv_io_desc, zc: c_int) -> io_uring_op {
    let ublk_op = ublksrv_get_op(iod);

    if ublk_op == UBLK_IO_OP_READ {
        if zc != 0 {
            return IORING_OP_READV_FIXED;
        }
        return IORING_OP_READV;
    }

    if ublk_op == UBLK_IO_OP_WRITE {
        if zc != 0 {
            return IORING_OP_WRITEV_FIXED;
        }
        return IORING_OP_WRITEV;
    }

    ublk_assert(0);
    -1
}

unsafe fn stripe_queue_tgt_rw_io(
    t: *mut ublk_thread,
    q: *mut ublk_queue,
    iod: *const ublksrv_io_desc,
    tag: c_int,
) -> c_int {
    let conf = get_chunk_shift(q);
    let auto_zc = (ublk_queue_use_auto_zc(q) != 0) as c_int;
    let zc = (ublk_queue_use_zc(q) != 0) as c_int;
    let op = stripe_to_uring_op(iod, zc | auto_zc);
    let mut sqe: [*mut io_uring_sqe; NR_STRIPE] = [ptr::null_mut(); NR_STRIPE];
    let s = alloc_stripe_array(conf, iod);
    let io = ublk_get_io(q, tag);
    let mut i: c_int = 0;
    let extra = if zc != 0 { 2 } else { 0 };
    let base = (*io).buf_addr;
    let buf_idx = ublk_io_buf_idx(t, q, tag);

    (*io).private_data = s as *mut c_void;
    calculate_stripe_array(conf, iod, s, base);

    ublk_io_alloc_sqes(t, sqe.as_mut_ptr(), (*s).nr as c_int + extra);

    if zc != 0 {
        io_uring_prep_buf_register(sqe[0], q, tag, (*q).q_id, buf_idx);
        (*sqe[0]).flags |= IOSQE_CQE_SKIP_SUCCESS | IOSQE_IO_HARDLINK;
        (*sqe[0]).user_data = build_user_data(
            tag,
            ublk_cmd_op_nr((*sqe[0]).cmd_op),
            0,
            (*q).q_id,
            1,
        );
    }

    while i < (*s).nr + extra - zc {
        let this = &mut (*s).s[(i as usize) - (zc as usize)];

        io_uring_prep_rw(
            op,
            sqe[i as usize],
            this.seq + 1,
            this.vec,
            this.nr_vec,
            (this.start as u64) << 9,
        );
        io_uring_sqe_set_flags(sqe[i as usize], IOSQE_FIXED_FILE);

        if auto_zc != 0 || zc != 0 {
            (*sqe[i as usize]).buf_index = buf_idx;
            if zc != 0 {
                (*sqe[i as usize]).flags |= IOSQE_IO_HARDLINK;
            }
        }

        /* bit63 marks us as tgt io */
        (*sqe[i as usize]).user_data = build_user_data(
            tag,
            ublksrv_get_op(iod),
            (i - zc) as c_uint,
            (*q).q_id,
            1,
        );

        i += 1;
    }

    if zc != 0 {
        let unreg = sqe[(*s).nr as usize + 1];
        io_uring_prep_buf_unregister(unreg, q, tag, (*q).q_id, buf_idx);
        (*unreg).user_data = build_user_data(
            tag,
            ublk_cmd_op_nr((*unreg).cmd_op),
            0,
            (*q).q_id,
            1,
        );
    }

    /* register buffer is skip_success */
    (*s).nr as c_int + zc
}

unsafe fn handle_flush(t: *mut ublk_thread, q: *mut ublk_queue, _iod: *const ublksrv_io_desc, tag: c_int) -> c_int {
    let conf = get_chunk_shift(q);
    let mut sqe: [*mut io_uring_sqe; NR_STRIPE] = [ptr::null_mut(); NR_STRIPE];
    let mut i = 0;

    ublk_io_alloc_sqes(t, sqe.as_mut_ptr(), (*conf).nr_files as c_int);
    while i < (*conf).nr_files as c_int {
        io_uring_prep_fsync(sqe[i as usize], i + 1, IORING_FSYNC_DATASYNC);
        io_uring_sqe_set_flags(sqe[i as usize], IOSQE_FIXED_FILE);
        (*sqe[i as usize]).user_data = build_user_data(tag, UBLK_IO_OP_FLUSH, 0, (*q).q_id, 1);
        i += 1;
    }

    (*conf).nr_files as c_int
}

unsafe fn stripe_queue_tgt_io(t: *mut ublk_thread, q: *mut ublk_queue, tag: c_int) -> c_int {
    let iod = ublk_get_iod(q, tag);
    let ublk_op = ublksrv_get_op(iod);
    let mut ret = 0;

    match ublk_op {
        UBLK_IO_OP_FLUSH => ret = handle_flush(t, q, iod, tag),
        x if x == UBLK_IO_OP_WRITE_ZEROES || x == UBLK_IO_OP_DISCARD => ret = -ENOTSUP,
        x if x == UBLK_IO_OP_READ || x == UBLK_IO_OP_WRITE => {
            ret = stripe_queue_tgt_rw_io(t, q, iod, tag)
        }
        _ => ret = -EINVAL,
    }

    ublk_dbg(
        UBLK_DBG_IO,
        b"%s: tag %d ublk io %x %llx %u ret %d\n\0".as_ptr() as *const c_char,
        b"ublk_stripe_queue_tgt_io\0".as_ptr() as *const c_char,
        tag,
        (*iod).op_flags,
        (*iod).start_sector,
        (*iod).nr_sectors << 9,
        ret,
    );

    ret
}

unsafe extern "C" fn ublk_stripe_queue_io(t: *mut ublk_thread, q: *mut ublk_queue, tag: c_int) -> c_int {
    let queued = stripe_queue_tgt_io(t, q, tag);
    ublk_queued_tgt_io(t, q, tag, queued);
    0
}

unsafe extern "C" fn ublk_stripe_io_done(t: *mut ublk_thread, q: *mut ublk_queue, cqe: *const io_uring_cqe) {
    let tag = user_data_to_tag((*cqe).user_data);
    let iod = ublk_get_iod(q, tag as c_int);
    let op = user_data_to_op((*cqe).user_data);
    let io = ublk_get_io(q, tag as c_int);
    let res = (*cqe).res;

    if res < 0 || op != ublk_cmd_op_nr(UBLK_U_IO_UNREGISTER_IO_BUF) as c_uint {
        if (*io).result == 0 {
            (*io).result = res;
        }
        if res < 0 {
            ublk_err(
                b"%s: io failure %d tag %u\n\0".as_ptr() as *const c_char,
                b"ublk_stripe_io_done\0".as_ptr() as *const c_char,
                res,
                tag,
            );
        }
    }

    /* buffer register op is IOSQE_CQE_SKIP_SUCCESS */
    if op == ublk_cmd_op_nr(UBLK_U_IO_REGISTER_IO_BUF) as c_uint {
        (*io).tgt_ios = (*io).tgt_ios.wrapping_add(1);
    }

    /* fail short READ/WRITE simply */
    if op == UBLK_IO_OP_READ as c_uint || op == UBLK_IO_OP_WRITE as c_uint {
        let seq = user_data_to_tgt_data((*cqe).user_data) as usize;
        let s = (*io).private_data as *mut stripe_array;

        if res < (*s).s[seq].nr_sects as c_int << 9 {
            (*io).result = -EIO;
            ublk_err(
                b"%s: short rw op %u res %d exp %u tag %u\n\0".as_ptr() as *const c_char,
                b"ublk_stripe_io_done\0".as_ptr() as *const c_char,
                op,
                res,
                (*s).s[seq].nr_sects << 9,
                tag,
            );
        }
    }

    if ublk_completed_tgt_io(t, q, tag) != 0 {
        let mut result = (*io).result;

        if result == 0 {
            result = ((*iod).nr_sectors as c_int) << 9;
        }

        ublk_complete_io(t, q, tag, result);
        free_stripe_array((*io).private_data as *mut stripe_array);
        (*io).private_data = ptr::null_mut();
    }
}

unsafe extern "C" fn ublk_stripe_tgt_init(ctx: *const dev_ctx, dev: *mut ublk_dev) -> c_int {
    let mut p = ublk_params {
        types: UBLK_PARAM_TYPE_BASIC,
        basic: ublk_basic_params {
            attrs: UBLK_ATTR_VOLATILE_CACHE,
            logical_bs_shift: 9,
            physical_bs_shift: 12,
            io_opt_shift: 12,
            io_min_shift: 9,
            max_sectors: 0,
            dev_sectors: 0,
        },
    };
    let chunk_size = (*ctx).stripe.chunk_size;
    let mut conf: *mut stripe_conf;
    let chunk_shift: c_int;
    let mut bytes = 0i64;
    let mut ret: c_int;
    let mut i: c_int;
    let mut mul = 1;

    if (*ctx).auto_zc_fallback != 0 {
        ublk_err(
            b"%s: not support auto_zc_fallback\n\0".as_ptr() as *const c_char,
            b"ublk_stripe_tgt_init\0".as_ptr() as *const c_char,
        );
        return -EINVAL;
    }

    if (*ctx).metadata_size != 0 {
        ublk_err(
            b"%s: integrity not supported\n\0".as_ptr() as *const c_char,
            b"ublk_stripe_tgt_init\0".as_ptr() as *const c_char,
        );
        return -EINVAL;
    }

    if (chunk_size & (chunk_size - 1)) != 0 || chunk_size == 0 {
        ublk_err(
            b"invalid chunk size %u\n\0".as_ptr() as *const c_char,
            chunk_size,
        );
        return -EINVAL;
    }

    if chunk_size < 4096 || chunk_size > 512 * 1024 {
        ublk_err(
            b"invalid chunk size %u\n\0".as_ptr() as *const c_char,
            chunk_size,
        );
        return -EINVAL;
    }

    chunk_shift = ilog2(chunk_size);

    ret = backing_file_tgt_init(dev, (*dev).tgt.nr_backing_files);
    if ret != 0 {
        return ret;
    }

    if (*dev).tgt.nr_backing_files == 0 || (*dev).tgt.nr_backing_files > NR_STRIPE as c_uint {
        return -EINVAL;
    }

    ublk_assert(((*dev).nr_fds == (*dev).tgt.nr_backing_files + 1) as c_int);

    i = 0;
    while i < (*dev).tgt.nr_backing_files as c_int {
        let fsize = (*dev).tgt.backing_file_size.add(i as usize);
        *fsize &= !((1i64 << chunk_shift) - 1);
        i += 1;
    }

    i = 0;
    while i < (*dev).tgt.nr_backing_files as c_int {
        let size = *(*dev).tgt.backing_file_size.add(i as usize);

        if size != *(*dev).tgt.backing_file_size {
            return -EINVAL;
        }
        bytes += size;
        i += 1;
    }

    conf = malloc(size_of::<stripe_conf>()) as *mut stripe_conf;
    (*conf).shift = chunk_shift as c_uint;
    (*conf).nr_files = (*dev).tgt.nr_backing_files;
    (*dev).private_data = conf as *mut c_void;
    (*dev).tgt.dev_size = bytes;
    p.basic.max_sectors = (*dev).dev_info.max_io_buf_bytes >> 9;
    p.basic.dev_sectors = bytes >> 9;
    (*dev).tgt.params = p;

    if (*dev).dev_info.flags & UBLK_F_SUPPORT_ZERO_COPY != 0 {
        mul = 2;
    }

    (*dev).tgt.sq_depth = (mul as c_uint) * (*dev).dev_info.queue_depth * (*dev).tgt.nr_backing_files;
    (*dev).tgt.cq_depth = (mul as c_uint) * (*dev).dev_info.queue_depth * (*dev).tgt.nr_backing_files;

    printf(
        b"%s: shift %u files %u\n\0".as_ptr() as *const c_char,
        b"ublk_stripe_tgt_init\0".as_ptr() as *const c_char,
        (*conf).shift,
        (*conf).nr_files,
    );

    0
}

unsafe extern "C" fn ublk_stripe_tgt_deinit(dev: *mut ublk_dev) {
    free((*dev).private_data);
    backing_file_tgt_deinit(dev);
}

unsafe extern "C" fn ublk_stripe_cmd_line(ctx: *mut dev_ctx, argc: c_int, argv: *mut *mut c_char) {
    static LONG_OPTS: [option; 2] = [
        option {
            name: b"chunk_size\0".as_ptr() as *const c_char,
            has_arg: 1,
            flag: ptr::null_mut(),
            val: 0,
        },
        option {
            name: ptr::null(),
            has_arg: 0,
            flag: ptr::null_mut(),
            val: 0,
        },
    ];

    (*ctx).stripe.chunk_size = 65536;

    let mut option_idx = 0;
    loop {
        let opt = getopt_long(argc, argv, b"\0".as_ptr() as *const c_char, LONG_OPTS.as_ptr(), &mut option_idx);
        if opt == -1 {
            break;
        }

        match opt {
            0 => {
                if strcmp(LONG_OPTS[option_idx as usize].name, b"chunk_size\0".as_ptr() as *const c_char) == 0 {
                    (*ctx).stripe.chunk_size = strtol(optarg, ptr::null_mut(), 10) as c_uint;
                }
            }
            _ => {}
        }
    }
}

unsafe extern "C" fn ublk_stripe_usage(_ops: *const ublk_tgt_ops) {
    printf(b"\tstripe: [--chunk_size chunk_size (default 65536)]\n\0".as_ptr() as *const c_char);
}

#[allow(non_upper_case_globals)]
pub static stripe_tgt_ops: ublk_tgt_ops = ublk_tgt_ops {
    name: b"stripe\0".as_ptr() as *const c_char,
    init_tgt: Some(ublk_stripe_tgt_init),
    deinit_tgt: Some(ublk_stripe_tgt_deinit),
    queue_io: Some(ublk_stripe_queue_io),
    tgt_io_done: Some(ublk_stripe_io_done),
    parse_cmd_line: Some(ublk_stripe_cmd_line),
    usage: Some(ublk_stripe_usage),
};

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
