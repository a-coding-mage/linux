// SPDX-License-Identifier: GPL-2.0

// Translated from file_backed.c. Dependencies originally supplied by "kublk.h"
// are declared here as external C items or C-layout types.

#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(dead_code)]

use core::ffi::{c_char, c_int, c_uint, c_ulong, c_ulonglong, c_void};

type __u8 = u8;
type __u32 = u32;
type __u64 = u64;
type __s32 = i32;
type size_t = usize;
type off_t = i64;

type io_uring_op = c_uint;

const UBLK_IO_OP_READ: c_uint = 0;
const UBLK_IO_OP_WRITE: c_uint = 1;
const UBLK_IO_OP_FLUSH: c_uint = 2;
const UBLK_IO_OP_DISCARD: c_uint = 3;
const UBLK_IO_OP_WRITE_ZEROES: c_uint = 4;

const IORING_OP_READ: io_uring_op = 22;
const IORING_OP_WRITE: io_uring_op = 23;
const IORING_OP_READ_FIXED: io_uring_op = 4;
const IORING_OP_WRITE_FIXED: io_uring_op = 5;

const IORING_FSYNC_DATASYNC: c_uint = 1;
const IOSQE_FIXED_FILE: u8 = 1 << 0;
const IOSQE_IO_HARDLINK: u8 = 1 << 2;
const IOSQE_CQE_SKIP_SUCCESS: u8 = 1 << 6;

const UBLK_IO_F_SHMEM_ZC: c_uint = 1 << 0;
const UBLK_IO_F_INTEGRITY: c_uint = 1 << 1;

const UBLK_BUF_MAX: __u32 = 64;
const UBLK_DBG_IO: c_int = 0;
const UBLK_U_IO_REGISTER_IO_BUF: c_uint = 0;
const UBLK_U_IO_UNREGISTER_IO_BUF: c_uint = 0;

const UBLK_PARAM_TYPE_BASIC: c_uint = 1 << 0;
const UBLK_PARAM_TYPE_DMA_ALIGN: c_uint = 1 << 1;
const UBLK_ATTR_VOLATILE_CACHE: c_uint = 1 << 0;

const EINVAL: c_int = 22;
const ENOMEM: c_int = 12;
const ENOTSUP: c_int = 95;
const EIO: c_int = 5;

#[repr(C)]
pub struct ublksrv_io_desc {
    pub op_flags: c_uint,
    pub start_sector: __u64,
    pub nr_sectors: __u32,
    pub addr: __u64,
}

#[repr(C)]
pub struct io_uring_sqe {
    pub user_data: __u64,
    pub flags: u8,
    pub buf_index: u16,
    pub cmd_op: c_uint,
}

#[repr(C)]
pub struct io_uring_cqe {
    pub user_data: __u64,
    pub res: __s32,
}

#[repr(C)]
pub struct ublk_thread {
    _private: [u8; 0],
}

#[repr(C)]
pub struct ublk_queue {
    pub q_id: c_uint,
}

#[repr(C)]
pub struct ublk_io {
    pub buf_addr: *mut c_void,
    pub integrity_buf: *mut c_void,
    pub result: __s32,
    pub tgt_ios: c_uint,
}

#[repr(C)]
pub struct shmem_table_entry {
    pub mmap_base: *mut u8,
}

#[repr(C)]
pub struct dev_ctx {
    pub auto_zc_fallback: c_int,
    pub metadata_size: c_ulong,
}

#[repr(C)]
pub struct ublk_dev_info {
    pub max_io_buf_bytes: c_uint,
}

#[repr(C)]
pub struct ublk_tgt {
    pub nr_backing_files: c_uint,
    pub backing_file_size: [c_ulonglong; 2],
    pub dev_size: c_ulonglong,
    pub params: ublk_params,
}

#[repr(C)]
pub struct ublk_dev {
    pub dev_info: ublk_dev_info,
    pub tgt: ublk_tgt,
    pub fds: [c_int; 3],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ublk_param_basic {
    pub attrs: c_uint,
    pub logical_bs_shift: c_uint,
    pub physical_bs_shift: c_uint,
    pub io_opt_shift: c_uint,
    pub io_min_shift: c_uint,
    pub max_sectors: c_uint,
    pub dev_sectors: c_ulonglong,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ublk_param_dma {
    pub alignment: c_uint,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ublk_params {
    pub types: c_uint,
    pub basic: ublk_param_basic,
    pub dma: ublk_param_dma,
}

#[repr(C)]
pub struct ublk_tgt_ops {
    pub name: *const c_char,
    pub init_tgt: Option<unsafe extern "C" fn(*const dev_ctx, *mut ublk_dev) -> c_int>,
    pub deinit_tgt: Option<unsafe extern "C" fn(*mut ublk_dev)>,
    pub queue_io: Option<unsafe extern "C" fn(*mut ublk_thread, *mut ublk_queue, c_int) -> c_int>,
    pub tgt_io_done:
        Option<unsafe extern "C" fn(*mut ublk_thread, *mut ublk_queue, *const io_uring_cqe)>,
}

unsafe extern "C" {
    static mut shmem_table: [shmem_table_entry; UBLK_BUF_MAX as usize];
    static mut errno: c_int;

    fn ublksrv_get_op(iod: *const ublksrv_io_desc) -> c_uint;
    fn ublk_assert(expr: c_int);
    fn ublk_io_alloc_sqes(t: *mut ublk_thread, sqe: *mut *mut io_uring_sqe, nr: c_int);
    fn io_uring_prep_fsync(sqe: *mut io_uring_sqe, fd: c_int, fsync_flags: c_uint);
    fn io_uring_sqe_set_flags(sqe: *mut io_uring_sqe, flags: c_uint);
    fn ublk_get_registered_fd(q: *mut ublk_queue, index: c_int) -> c_int;
    fn build_user_data(tag: c_int, op: c_uint, tgt_data: c_uint, q_id: c_uint, tgt_io: c_uint) -> __u64;
    fn ublk_shmem_zc_index(addr: __u64) -> __u32;
    fn ublk_shmem_zc_offset(addr: __u64) -> __u32;
    fn io_uring_prep_rw(
        op: io_uring_op,
        sqe: *mut io_uring_sqe,
        fd: c_int,
        addr: *mut c_void,
        len: c_uint,
        offset: __u64,
    );
    fn ublk_queue_use_zc(q: *mut ublk_queue) -> c_uint;
    fn ublk_queue_use_auto_zc(q: *mut ublk_queue) -> c_uint;
    fn ublk_get_io(q: *mut ublk_queue, tag: c_int) -> *mut ublk_io;
    fn ublk_io_buf_idx(t: *mut ublk_thread, q: *mut ublk_queue, tag: c_int) -> u16;
    fn ublk_integrity_len(q: *mut ublk_queue, len: __u64) -> __u64;
    fn io_uring_prep_buf_register(
        sqe: *mut io_uring_sqe,
        q: *mut ublk_queue,
        tag: c_int,
        q_id: c_uint,
        buf_index: u16,
    );
    fn ublk_cmd_op_nr(op: c_uint) -> c_uint;
    fn io_uring_prep_buf_unregister(
        sqe: *mut io_uring_sqe,
        q: *mut ublk_queue,
        tag: c_int,
        q_id: c_uint,
        buf_index: u16,
    );
    fn ublk_get_iod(q: *mut ublk_queue, tag: c_int) -> *const ublksrv_io_desc;
    fn ublk_dbg(level: c_int, fmt: *const c_char, ...);
    fn ublk_queued_tgt_io(t: *mut ublk_thread, q: *mut ublk_queue, tag: c_int, queued: c_int);
    fn user_data_to_tag(user_data: __u64) -> c_uint;
    fn user_data_to_op(user_data: __u64) -> c_uint;
    fn ublk_err(fmt: *const c_char, ...);
    fn user_data_to_tgt_data(user_data: __u64) -> c_uint;
    fn ublk_integrity_data_len(q: *mut ublk_queue, len: __s32) -> __s32;
    fn ublk_completed_tgt_io(t: *mut ublk_thread, q: *mut ublk_queue, tag: c_int) -> c_int;
    fn ublk_complete_io(t: *mut ublk_thread, q: *mut ublk_queue, tag: c_int, result: __s32);
    fn memset(s: *mut c_void, c: c_int, n: size_t) -> *mut c_void;
    fn pwrite(fd: c_int, buf: *const c_void, count: size_t, offset: off_t) -> isize;
    fn ublk_set_integrity_params(ctx: *const dev_ctx, p: *mut ublk_params);
    fn backing_file_tgt_init(dev: *mut ublk_dev, index: c_int) -> c_int;
    fn backing_file_tgt_deinit(dev: *mut ublk_dev);
}

unsafe fn ublk_to_uring_op(iod: *const ublksrv_io_desc, zc: c_int) -> io_uring_op {
    let ublk_op: c_uint = ublksrv_get_op(iod);

    if ublk_op == UBLK_IO_OP_READ {
        if zc != 0 {
            IORING_OP_READ_FIXED
        } else {
            IORING_OP_READ
        }
    } else if ublk_op == UBLK_IO_OP_WRITE {
        if zc != 0 {
            IORING_OP_WRITE_FIXED
        } else {
            IORING_OP_WRITE
        }
    } else {
        ublk_assert(0);
        0
    }
}

unsafe fn loop_queue_flush_io(
    t: *mut ublk_thread,
    q: *mut ublk_queue,
    iod: *const ublksrv_io_desc,
    tag: c_int,
) -> c_int {
    let ublk_op: c_uint = ublksrv_get_op(iod);
    let mut sqe: [*mut io_uring_sqe; 1] = [core::ptr::null_mut(); 1];

    ublk_io_alloc_sqes(t, sqe.as_mut_ptr(), 1);
    io_uring_prep_fsync(
        sqe[0],
        ublk_get_registered_fd(q, 1),
        IORING_FSYNC_DATASYNC,
    );
    io_uring_sqe_set_flags(sqe[0], IOSQE_FIXED_FILE as c_uint);
    /* bit63 marks us as tgt io */
    (*sqe[0]).user_data = build_user_data(tag, ublk_op, 0, (*q).q_id, 1);
    1
}

/*
 * Shared memory zero-copy I/O: when UBLK_IO_F_SHMEM_ZC is set, the
 * request's data lives in a registered shared memory buffer. Decode
 * index + offset from iod->addr and use the server's mmap of that
 * buffer as the I/O buffer for the backing file.
 */
unsafe fn loop_queue_shmem_zc_io(
    t: *mut ublk_thread,
    q: *mut ublk_queue,
    iod: *const ublksrv_io_desc,
    tag: c_int,
) -> c_int {
    let ublk_op: c_uint = ublksrv_get_op(iod);
    let op: io_uring_op = ublk_to_uring_op(iod, 0);
    let file_offset: __u64 = (*iod).start_sector << 9;
    let len: __u32 = (*iod).nr_sectors << 9;
    let shmem_idx: __u32 = ublk_shmem_zc_index((*iod).addr);
    let shmem_off: __u32 = ublk_shmem_zc_offset((*iod).addr);
    let mut sqe: [*mut io_uring_sqe; 1] = [core::ptr::null_mut(); 1];
    let addr: *mut c_void;

    if shmem_idx >= UBLK_BUF_MAX || shmem_table[shmem_idx as usize].mmap_base.is_null() {
        return -EINVAL;
    }

    addr = shmem_table[shmem_idx as usize]
        .mmap_base
        .add(shmem_off as usize) as *mut c_void;

    ublk_io_alloc_sqes(t, sqe.as_mut_ptr(), 1);
    if sqe[0].is_null() {
        return -ENOMEM;
    }

    io_uring_prep_rw(op, sqe[0], ublk_get_registered_fd(q, 1), addr, len, file_offset);
    io_uring_sqe_set_flags(sqe[0], IOSQE_FIXED_FILE as c_uint);
    (*sqe[0]).user_data = build_user_data(tag, ublk_op, 0, (*q).q_id, 1);
    1
}

unsafe fn loop_queue_tgt_rw_io(
    t: *mut ublk_thread,
    q: *mut ublk_queue,
    iod: *const ublksrv_io_desc,
    tag: c_int,
) -> c_int {
    let ublk_op: c_uint = ublksrv_get_op(iod);
    let zc: c_uint = ublk_queue_use_zc(q);
    let auto_zc: c_uint = ublk_queue_use_auto_zc(q);
    let op: io_uring_op = ublk_to_uring_op(iod, (zc | auto_zc) as c_int);
    let io: *mut ublk_io = ublk_get_io(q, tag);
    let offset: __u64 = (*iod).start_sector << 9;
    let len: __u32 = (*iod).nr_sectors << 9;
    let mut sqe: [*mut io_uring_sqe; 3] = [core::ptr::null_mut(); 3];
    let addr: *mut c_void = (*io).buf_addr;
    let buf_index: u16 = ublk_io_buf_idx(t, q, tag);

    /* shared memory zero-copy path */
    if ((*iod).op_flags & UBLK_IO_F_SHMEM_ZC) != 0 {
        return loop_queue_shmem_zc_io(t, q, iod, tag);
    }

    if ((*iod).op_flags & UBLK_IO_F_INTEGRITY) != 0 {
        ublk_io_alloc_sqes(t, sqe.as_mut_ptr(), 1);
        /* Use second backing file for integrity data */
        io_uring_prep_rw(
            op,
            sqe[0],
            ublk_get_registered_fd(q, 2),
            (*io).integrity_buf,
            ublk_integrity_len(q, len as __u64) as c_uint,
            ublk_integrity_len(q, offset),
        );
        (*sqe[0]).flags = IOSQE_FIXED_FILE;
        /* tgt_data = 1 indicates integrity I/O */
        (*sqe[0]).user_data = build_user_data(tag, ublk_op, 1, (*q).q_id, 1);
    }

    if zc == 0 || auto_zc != 0 {
        ublk_io_alloc_sqes(t, sqe.as_mut_ptr(), 1);
        if sqe[0].is_null() {
            return -ENOMEM;
        }

        io_uring_prep_rw(
            op,
            sqe[0],
            ublk_get_registered_fd(q, 1),
            addr,
            len,
            offset,
        );
        if auto_zc != 0 {
            (*sqe[0]).buf_index = buf_index;
        }
        io_uring_sqe_set_flags(sqe[0], IOSQE_FIXED_FILE as c_uint);
        /* bit63 marks us as tgt io */
        (*sqe[0]).user_data = build_user_data(tag, ublk_op, 0, (*q).q_id, 1);
        return (((*iod).op_flags & UBLK_IO_F_INTEGRITY) != 0) as c_int + 1;
    }

    ublk_io_alloc_sqes(t, sqe.as_mut_ptr(), 3);

    io_uring_prep_buf_register(sqe[0], q, tag, (*q).q_id, buf_index);
    (*sqe[0]).flags |= IOSQE_CQE_SKIP_SUCCESS | IOSQE_IO_HARDLINK;
    (*sqe[0]).user_data =
        build_user_data(tag, ublk_cmd_op_nr((*sqe[0]).cmd_op), 0, (*q).q_id, 1);

    io_uring_prep_rw(
        op,
        sqe[1],
        ublk_get_registered_fd(q, 1),
        core::ptr::null_mut(),
        len,
        offset,
    );
    (*sqe[1]).buf_index = buf_index;
    (*sqe[1]).flags |= IOSQE_FIXED_FILE | IOSQE_IO_HARDLINK;
    (*sqe[1]).user_data = build_user_data(tag, ublk_op, 0, (*q).q_id, 1);

    io_uring_prep_buf_unregister(sqe[2], q, tag, (*q).q_id, buf_index);
    (*sqe[2]).user_data =
        build_user_data(tag, ublk_cmd_op_nr((*sqe[2]).cmd_op), 0, (*q).q_id, 1);

    (((*iod).op_flags & UBLK_IO_F_INTEGRITY) != 0) as c_int + 2
}

unsafe fn loop_queue_tgt_io(t: *mut ublk_thread, q: *mut ublk_queue, tag: c_int) -> c_int {
    let iod: *const ublksrv_io_desc = ublk_get_iod(q, tag);
    let ublk_op: c_uint = ublksrv_get_op(iod);
    let ret: c_int;

    match ublk_op {
        UBLK_IO_OP_FLUSH => {
            ret = loop_queue_flush_io(t, q, iod, tag);
        }
        UBLK_IO_OP_WRITE_ZEROES | UBLK_IO_OP_DISCARD => {
            ret = -ENOTSUP;
        }
        UBLK_IO_OP_READ | UBLK_IO_OP_WRITE => {
            ret = loop_queue_tgt_rw_io(t, q, iod, tag);
        }
        _ => {
            ret = -EINVAL;
        }
    }

    ublk_dbg(
        UBLK_DBG_IO,
        c"%s: tag %d ublk io %x %llx %u\n".as_ptr(),
        c"loop_queue_tgt_io".as_ptr(),
        tag,
        (*iod).op_flags,
        (*iod).start_sector,
        (*iod).nr_sectors << 9,
    );
    ret
}

unsafe extern "C" fn ublk_loop_queue_io(
    t: *mut ublk_thread,
    q: *mut ublk_queue,
    tag: c_int,
) -> c_int {
    let queued: c_int = loop_queue_tgt_io(t, q, tag);

    ublk_queued_tgt_io(t, q, tag, queued);
    0
}

unsafe extern "C" fn ublk_loop_io_done(
    t: *mut ublk_thread,
    q: *mut ublk_queue,
    cqe: *const io_uring_cqe,
) {
    let tag: c_uint = user_data_to_tag((*cqe).user_data);
    let op: c_uint = user_data_to_op((*cqe).user_data);
    let io: *mut ublk_io = ublk_get_io(q, tag as c_int);

    if (*cqe).res < 0 {
        (*io).result = (*cqe).res;
        ublk_err(
            c"%s: io failed op %x user_data %lx\n".as_ptr(),
            c"ublk_loop_io_done".as_ptr(),
            op,
            (*cqe).user_data as c_ulong,
        );
    } else if op != ublk_cmd_op_nr(UBLK_U_IO_UNREGISTER_IO_BUF) {
        let data_len: __s32 = if user_data_to_tgt_data((*cqe).user_data) != 0 {
            ublk_integrity_data_len(q, (*cqe).res)
        } else {
            (*cqe).res
        };

        if (*io).result == 0 || data_len < (*io).result {
            (*io).result = data_len;
        }
    }

    /* buffer register op is IOSQE_CQE_SKIP_SUCCESS */
    if op == ublk_cmd_op_nr(UBLK_U_IO_REGISTER_IO_BUF) {
        (*io).tgt_ios += 1;
    }

    if ublk_completed_tgt_io(t, q, tag as c_int) != 0 {
        ublk_complete_io(t, q, tag as c_int, (*io).result);
    }
}

unsafe fn ublk_loop_memset_file(fd: c_int, byte: __u8, mut len: size_t) -> c_int {
    let mut offset: off_t = 0;
    let mut buf: [__u8; 4096] = [0; 4096];

    memset(buf.as_mut_ptr() as *mut c_void, byte as c_int, core::mem::size_of_val(&buf));
    while len != 0 {
        let ret: isize = pwrite(
            fd,
            buf.as_ptr() as *const c_void,
            core::cmp::min(len, core::mem::size_of_val(&buf)),
            offset,
        );

        if ret < 0 {
            return -errno;
        }
        if ret == 0 {
            return -EIO;
        }

        len -= ret as size_t;
        offset += ret as off_t;
    }
    0
}

unsafe extern "C" fn ublk_loop_tgt_init(ctx: *const dev_ctx, dev: *mut ublk_dev) -> c_int {
    let bytes: c_ulonglong;
    let mut blocks: c_ulong;
    let mut ret: c_int;
    let mut p = ublk_params {
        types: UBLK_PARAM_TYPE_BASIC | UBLK_PARAM_TYPE_DMA_ALIGN,
        basic: ublk_param_basic {
            attrs: UBLK_ATTR_VOLATILE_CACHE,
            logical_bs_shift: 9,
            physical_bs_shift: 12,
            io_opt_shift: 12,
            io_min_shift: 9,
            max_sectors: (*dev).dev_info.max_io_buf_bytes >> 9,
            dev_sectors: 0,
        },
        dma: ublk_param_dma { alignment: 511 },
    };

    ublk_set_integrity_params(ctx, &mut p);
    if (*ctx).auto_zc_fallback != 0 {
        ublk_err(
            c"%s: not support auto_zc_fallback\n".as_ptr(),
            c"ublk_loop_tgt_init".as_ptr(),
        );
        return -EINVAL;
    }

    /* Use O_DIRECT only for data file */
    ret = backing_file_tgt_init(dev, 1);
    if ret != 0 {
        return ret;
    }

    /* Expect a second file for integrity data */
    if (*dev).tgt.nr_backing_files != 1 + (((*ctx).metadata_size != 0) as c_uint) {
        return -EINVAL;
    }

    blocks = ((*dev).tgt.backing_file_size[0] >> p.basic.logical_bs_shift) as c_ulong;
    if (*ctx).metadata_size != 0 {
        let metadata_blocks: c_ulong =
            ((*dev).tgt.backing_file_size[1] / (*ctx).metadata_size as c_ulonglong) as c_ulong;
        let integrity_len: c_ulong;

        /* Ensure both data and integrity data fit in backing files */
        blocks = core::cmp::min(blocks, metadata_blocks);
        integrity_len = blocks * (*ctx).metadata_size;
        /*
         * Initialize PI app tag and ref tag to 0xFF
         * to disable bio-integrity-auto checks
         */
        ret = ublk_loop_memset_file((*dev).fds[2], 0xFF, integrity_len as size_t);
        if ret != 0 {
            return ret;
        }
    }
    bytes = (blocks as c_ulonglong) << p.basic.logical_bs_shift;
    (*dev).tgt.dev_size = bytes;
    p.basic.dev_sectors = bytes >> 9;
    (*dev).tgt.params = p;

    0
}

#[unsafe(no_mangle)]
pub static loop_tgt_ops: ublk_tgt_ops = ublk_tgt_ops {
    name: c"loop".as_ptr(),
    init_tgt: Some(ublk_loop_tgt_init),
    deinit_tgt: Some(backing_file_tgt_deinit),
    queue_io: Some(ublk_loop_queue_io),
    tgt_io_done: Some(ublk_loop_io_done),
};

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
