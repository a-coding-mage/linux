// SPDX-License-Identifier: GPL-2.0

use core::ffi::{c_char, c_short, c_uchar, c_uint, c_ulong, c_ushort, c_void};
use core::mem::MaybeUninit;

pub type __u8 = u8;
pub type __u16 = u16;
pub type __u32 = u32;
pub type __u64 = u64;
pub type __s32 = i32;
pub type __s64 = i64;
pub type c_int = i32;
pub type c_long = isize;
pub type size_t = usize;

pub const MAX_BACK_FILES: usize = 4;

/****************** part 1: libublk ********************/

pub const CTRL_DEV: &str = "/dev/ublk-control";
pub const UBLKC_DEV: &str = "/dev/ublkc";
pub const UBLKB_DEV: &str = "/dev/ublkb";
pub const UBLK_CTRL_RING_DEPTH: c_uint = 32;
pub const ERROR_EVTFD_DEVID: c_int = -2;

pub const UBLK_IO_MAX_BYTES: usize = 1 << 20;
pub const UBLK_MAX_QUEUES_SHIFT: usize = 5;
pub const UBLK_MAX_QUEUES: usize = 1 << UBLK_MAX_QUEUES_SHIFT;
pub const UBLK_MAX_THREADS_SHIFT: usize = 5;
pub const UBLK_MAX_THREADS: usize = 1 << UBLK_MAX_THREADS_SHIFT;
pub const UBLK_QUEUE_DEPTH: usize = 1024;

#[repr(C)]
pub struct ublk_dev;
#[repr(C)]
pub struct ublk_queue;
#[repr(C)]
pub struct ublk_thread;

#[repr(C)]
pub struct stripe_ctx {
    pub chunk_size: c_uint,
}

#[repr(C)]
pub struct fault_inject_ctx {
    pub delay_us: c_ulong,
    pub die_during_fetch: bool,
}

#[repr(C)]
pub struct params_ctx {
    pub types: __u32,

    pub logical_bs_shift: __u32,
    pub physical_bs_shift: __u32,
    pub io_min_shift: __u32,
    pub io_opt_shift: __u32,
    pub max_sectors: __u32,
    pub chunk_sectors: __u32,
    pub dev_sectors: __u64,

    pub max_open_zones: __u32,
    pub max_active_zones: __u32,
    pub max_zone_append_sectors: __u32,
}

#[repr(C)]
pub union DevCtxOps {
    pub stripe: stripe_ctx,
    pub fault_inject: fault_inject_ctx,
}

#[repr(C)]
pub struct dev_ctx {
    pub tgt_type: [c_char; 16],
    pub flags: c_ulong,
    pub nr_hw_queues: c_uint,
    pub nthreads: c_ushort,
    pub queue_depth: c_uint,
    pub dev_id: c_int,
    pub nr_files: c_int,
    pub files: [*mut c_char; MAX_BACK_FILES],

    pub _bitflags1: __u32,

    pub integrity_flags: __u32,
    pub metadata_size: __u8,
    pub pi_offset: __u8,
    pub csum_type: __u8,
    pub tag_size: __u8,
    pub io_desc_size: __u16,

    pub _evtfd: c_int,
    pub _shmid: c_int,

    /// built from shmem, only for ublk_dump_dev()
    pub shadow_dev: *mut ublk_dev,

    /// for 'update_size' command
    pub size: __u64,

    pub params: params_ctx,

    pub htlb_path: *mut c_char,

    pub ops: DevCtxOps,
}

pub const DEV_CTX_LOGGING_BIT: __u32 = 1 << 0;
pub const DEV_CTX_ALL_BIT: __u32 = 1 << 1;
pub const DEV_CTX_FG_BIT: __u32 = 1 << 2;
pub const DEV_CTX_RECOVERY_BIT: __u32 = 1 << 3;
pub const DEV_CTX_AUTO_ZC_FALLBACK_BIT: __u32 = 1 << 4;
pub const DEV_CTX_PER_IO_TASKS_BIT: __u32 = 1 << 5;
pub const DEV_CTX_NO_UBLK_FIXED_FD_BIT: __u32 = 1 << 6;
pub const DEV_CTX_SAFE_STOP_BIT: __u32 = 1 << 7;
pub const DEV_CTX_NO_AUTO_PART_SCAN_BIT: __u32 = 1 << 8;
pub const DEV_CTX_RDONLY_SHMEM_BUF_BIT: __u32 = 1 << 9;
pub const DEV_CTX_ROTATE_AUTO_BUF_BIT: __u32 = 1 << 10;

impl dev_ctx {
    #[inline(always)]
    pub const fn logging(self) -> bool {
        (self._bitflags1 & DEV_CTX_LOGGING_BIT) != 0
    }
    #[inline(always)]
    pub const fn all(self) -> bool {
        (self._bitflags1 & DEV_CTX_ALL_BIT) != 0
    }
    #[inline(always)]
    pub const fn fg(self) -> bool {
        (self._bitflags1 & DEV_CTX_FG_BIT) != 0
    }
    #[inline(always)]
    pub const fn recovery(self) -> bool {
        (self._bitflags1 & DEV_CTX_RECOVERY_BIT) != 0
    }
    #[inline(always)]
    pub const fn auto_zc_fallback(self) -> bool {
        (self._bitflags1 & DEV_CTX_AUTO_ZC_FALLBACK_BIT) != 0
    }
    #[inline(always)]
    pub const fn per_io_tasks(self) -> bool {
        (self._bitflags1 & DEV_CTX_PER_IO_TASKS_BIT) != 0
    }
    #[inline(always)]
    pub const fn no_ublk_fixed_fd(self) -> bool {
        (self._bitflags1 & DEV_CTX_NO_UBLK_FIXED_FD_BIT) != 0
    }
    #[inline(always)]
    pub const fn safe_stop(self) -> bool {
        (self._bitflags1 & DEV_CTX_SAFE_STOP_BIT) != 0
    }
    #[inline(always)]
    pub const fn no_auto_part_scan(self) -> bool {
        (self._bitflags1 & DEV_CTX_NO_AUTO_PART_SCAN_BIT) != 0
    }
    #[inline(always)]
    pub const fn rdonly_shmem_buf(self) -> bool {
        (self._bitflags1 & DEV_CTX_RDONLY_SHMEM_BUF_BIT) != 0
    }
    #[inline(always)]
    pub const fn rotate_auto_buf(self) -> bool {
        (self._bitflags1 & DEV_CTX_ROTATE_AUTO_BUF_BIT) != 0
    }
}

#[repr(C)]
pub struct ublk_ctrl_cmd_data {
    pub cmd_op: __u32,
    pub flags: __u32,
    pub data: [__u64; 2],
    pub addr: __u64,
    pub len: __u32,
}

pub const CTRL_CMD_HAS_DATA: __u32 = 1;
pub const CTRL_CMD_HAS_BUF: __u32 = 2;

#[repr(C)]
pub struct ublk_io {
    pub buf_addr: *mut c_char,
    pub integrity_buf: *mut c_void,

    pub flags: c_ushort,
    /// used by target code only
    pub refs: c_ushort,

    pub tag: c_int,
    pub result: c_int,

    pub buf_index: c_ushort,
    pub tgt_ios: c_ushort,
    pub auto_buf_phase: c_uchar,
    pub private_data: *mut c_void,
}

pub const UBLKS_IO_NEED_FETCH_RQ: c_uint = 1 << 0;
pub const UBLKS_IO_NEED_COMMIT_RQ_COMP: c_uint = 1 << 1;
pub const UBLKS_IO_FREE: c_uint = 1 << 2;
pub const UBLKS_IO_NEED_GET_DATA: c_uint = 1 << 3;
pub const UBLKS_IO_NEED_REG_BUF: c_uint = 1 << 4;

#[repr(C)]
pub struct ublk_tgt_ops {
    pub name: *const c_char,
    pub init_tgt: Option<unsafe extern "C" fn(*const dev_ctx, *mut ublk_dev) -> c_int>,
    pub deinit_tgt: Option<unsafe extern "C" fn(*mut ublk_dev)>,
    pub pre_fetch_io: Option<unsafe extern "C" fn(*mut ublk_thread, *mut ublk_queue, c_int, bool)>,
    pub queue_io: Option<unsafe extern "C" fn(*mut ublk_thread, *mut ublk_queue, c_int) -> c_int>,
    pub tgt_io_done: Option<unsafe extern "C" fn(*mut ublk_thread, *mut ublk_queue, *const io_uring_cqe)>,
    pub parse_cmd_line: Option<unsafe extern "C" fn(*mut dev_ctx, c_int, *mut *mut c_char)>,
    pub usage: Option<unsafe extern "C" fn(*const ublk_tgt_ops)>,
    pub buf_index: Option<unsafe extern "C" fn(*const ublk_thread, *const ublk_queue, c_int) -> c_ushort>,
}

#[repr(C)]
pub struct ublk_param_basic {
    pub logical_bs_shift: __u32,
    pub physical_bs_shift: __u32,
    pub io_min_shift: __u32,
    pub io_opt_shift: __u32,
    pub max_sectors: __u32,
    pub chunk_sectors: __u32,
    pub dev_sectors: __u64,

    pub max_open_zones: __u32,
    pub max_active_zones: __u32,
    pub max_zone_append_sectors: __u32,
}

#[repr(C)]
pub struct ublk_param_integrity {
    pub flags: __u32,
    pub interval_exp: __u32,
    pub metadata_size: __u8,
    pub pi_offset: __u8,
    pub csum_type: __u8,
    pub tag_size: __u8,
}

#[repr(C)]
pub struct ublk_params {
    pub types: __u32,
    pub basic: ublk_param_basic,
    pub integrity: ublk_param_integrity,
}

#[repr(C)]
pub struct ublksrv_ctrl_dev_info {
    pub flags: c_ulong,
}

#[repr(C)]
pub struct ublk_tgt {
    pub dev_size: c_ulong,
    pub sq_depth: c_uint,
    pub cq_depth: c_uint,
    pub ops: *const ublk_tgt_ops,
    pub params: ublk_params,

    pub nr_backing_files: c_int,
    pub backing_file_size: [c_ulong; MAX_BACK_FILES],
    pub backing_file: [[c_char; PATH_MAX as usize]; MAX_BACK_FILES],
}

#[repr(C)]
pub struct ublk_queue {
    pub q_id: c_int,
    pub q_depth: c_int,
    pub dev: *mut ublk_dev,
    pub tgt_ops: *const ublk_tgt_ops,
    pub io_cmd_buf: *mut ublksrv_io_desc,

    pub flags: __u64,
    pub ublk_fd: c_int,
    pub metadata_size: c_uchar,
    pub io_desc_size: c_ushort,
    pub ios: [ublk_io; UBLK_QUEUE_DEPTH],

    pub lock: pthread_spinlock_t,
}

pub const UBLKS_Q_AUTO_BUF_REG_FALLBACK: __u64 = 1u64 << 63;
pub const UBLKS_Q_NO_UBLK_FIXED_FD: __u64 = 1u64 << 62;
pub const UBLKS_Q_PREPARED: __u64 = 1u64 << 61;
pub const UBLKS_Q_ROTATE_AUTO_BUF: __u64 = 1u64 << 60;

#[repr(C)]
pub struct ublksrv_io_desc {
    pub op_flags: c_uint,
}

#[repr(C)]
pub struct ublk_batch_elem {
    pub tag: __u16,
    pub buf_index: __u16,
    pub result: __s32,
    pub buf_addr: __u64,
}

#[repr(C)]
pub struct batch_commit_buf {
    pub q_id: c_ushort,
    pub buf_idx: c_ushort,
    pub elem: *mut c_void,
    pub done: c_ushort,
    pub count: c_ushort,
}

#[repr(C)]
pub struct batch_fetch_buf {
    pub br: *mut io_uring_buf_ring,
    pub fetch_buf: *mut c_void,
    pub fetch_buf_size: c_uint,
    pub fetch_buf_off: c_uint,
}

#[repr(C)]
pub struct allocator;

#[repr(C)]
pub struct ublk_thread {
    pub q_map: [c_uchar; UBLK_MAX_QUEUES],
    pub dev: *mut ublk_dev,
    pub idx: c_ushort,
    pub nr_queues: c_ushort,

    pub state: c_uint,
    pub cmd_inflight: c_uint,
    pub io_inflight: c_uint,

    pub nr_bufs: c_ushort,
    pub auto_buf_stride: c_ushort,

    pub commit_buf_start: c_ushort,
    pub commit_buf_elem_size: c_uchar,

    pub cmd_flags: c_ushort,
    pub nr_commit_buf: c_uint,
    pub commit_buf_size: c_uint,
    pub commit_buf: *mut c_void,

    pub commit_buf_alloc: allocator,
    pub commit: *mut batch_commit_buf,

    pub nr_fetch_bufs: c_ushort,
    pub fetch: *mut batch_fetch_buf,

    pub ring: io_uring,
}

pub const UBLKS_T_STOPPING: c_uint = 1 << 0;
pub const UBLKS_T_IDLE: c_uint = 1 << 1;
pub const UBLKS_T_BATCH_IO: c_uint = 1 << 31;

#[repr(C)]
pub struct ublk_dev {
    pub tgt: ublk_tgt,
    pub dev_info: ublksrv_ctrl_dev_info,
    pub q: [ublk_queue; UBLK_MAX_QUEUES],
    pub nthreads: c_uint,
    pub per_io_tasks: c_uint,

    pub fds: [c_int; MAX_BACK_FILES + 1],
    pub nr_fds: c_int,
    pub ctrl_fd: c_int,
    pub ring: io_uring,

    pub private_data: *mut c_void,
}

#[repr(C)]
pub struct io_uring {
    _private: [c_uchar; 0],
}

#[repr(C)]
pub struct io_uring_cqe {
    pub user_data: __u64,
    pub res: c_int,
    pub flags: c_uint,
}

#[repr(C)]
pub struct io_uring_buf_ring {
    _private: [c_uchar; 0],
}

#[repr(C)]
pub struct io_uring_sqe {
    pub off: __u64,
    pub flags: c_uint,
    pub opcode: c_uchar,
    pub cmd: [c_uchar; 48],
}

#[repr(C)]
pub struct ublksrv_io_cmd {
    pub tag: c_int,
    pub addr: __u64,
    pub q_id: c_int,
    pub cmd_op: c_uint,
}

pub type pthread_spinlock_t = c_int;

extern "C" {
    pub fn ublk_queue_io_cmd(t: *mut ublk_thread, io: *mut ublk_io) -> c_int;

    pub fn io_uring_sq_space_left(ring: *mut io_uring) -> c_uint;
    pub fn io_uring_submit(ring: *mut io_uring) -> c_int;
    pub fn io_uring_get_sqe(ring: *mut io_uring) -> *mut io_uring_sqe;
    pub fn io_uring_prep_read(
        sqe: *mut io_uring_sqe,
        fd: c_int,
        buf: *mut c_void,
        nbytes: __u32,
        offset: __u64,
    );

    pub static mut shmem_table: [ublk_shmem_entry; UBLK_BUF_MAX];
    pub static mut shmem_count: c_int;

    pub static null_tgt_ops: ublk_tgt_ops;
    pub static loop_tgt_ops: ublk_tgt_ops;
    pub static stripe_tgt_ops: ublk_tgt_ops;
    pub static fault_inject_tgt_ops: ublk_tgt_ops;

    pub fn backing_file_tgt_deinit(dev: *mut ublk_dev);
    pub fn backing_file_tgt_init(dev: *mut ublk_dev, nr_direct: c_uint) -> c_int;
}

pub const PATH_MAX: c_uint = 4096;

pub const UBLK_IO_F_NEED_REG_BUF: __u64 = 1u64 << 0;
pub const UBLKSRV_IO_BUF_OFFSET: __u64 = 0;
pub const UBLK_QID_OFF: __u64 = 0;
pub const UBLK_TAG_OFF: __u64 = 0;
pub const UBLK_F_BATCH_IO: __u64 = 0;
pub const UBLK_F_SUPPORT_ZERO_COPY: __u64 = 0;
pub const UBLK_F_AUTO_BUF_REG: __u64 = 0;
pub const UBLK_F_USER_COPY: __u64 = 0;
pub const UBLK_PARAM_TYPE_INTEGRITY: __u32 = 1;
pub const UBLK_U_IO_REGISTER_IO_BUF: __u32 = 0;
pub const UBLK_U_IO_UNREGISTER_IO_BUF: __u32 = 0;
pub const IOSQE_FIXED_FILE: c_uint = 1 << 0;
pub const IORING_OP_URING_CMD: c_uchar = 0;

pub const UBLKS_T_COMMIT_BUF_INV_IDX: c_ushort = !0;
pub const UBLK_BUF_MAX: usize = 256;

#[repr(C)]
pub struct ublk_shmem_entry {
    pub fd: c_int,
    pub mmap_base: *mut c_void,
    pub size: size_t,
}

#[inline(always)]
pub fn __ublk_use_batch_io(flags: __u64) -> c_int {
    (flags & UBLK_F_BATCH_IO) as c_int
}

#[inline(always)]
pub fn ublk_queue_batch_io(q: &ublk_queue) -> c_int {
    __ublk_use_batch_io(q.flags)
}

#[inline(always)]
pub fn ublk_dev_batch_io(dev: &ublk_dev) -> c_int {
    __ublk_use_batch_io(dev.dev_info.flags as __u64)
}

#[inline(always)]
pub fn ublk_thread_batch_io(t: &ublk_thread) -> c_int {
    ((t.state & UBLKS_T_BATCH_IO) != 0) as c_int
}

#[inline(always)]
pub fn ublk_assert(cond: bool) {
    if !cond {
        panic!("ublk_assert");
    }
}

#[inline(always)]
pub unsafe fn ublk_set_integrity_params(ctx: *const dev_ctx, params: *mut ublk_params) {
    if (*ctx).metadata_size == 0 {
        return;
    }

    (*params).types |= UBLK_PARAM_TYPE_INTEGRITY;
    (*params).integrity = ublk_param_integrity {
        flags: (*ctx).integrity_flags,
        interval_exp: (*params).basic.logical_bs_shift,
        metadata_size: (*ctx).metadata_size,
        pi_offset: (*ctx).pi_offset,
        csum_type: (*ctx).csum_type,
        tag_size: (*ctx).tag_size,
    };
}

#[inline(always)]
pub fn ublk_integrity_len(q: &ublk_queue, len: size_t) -> size_t {
    (len >> 9) * (q.metadata_size as size_t)
}

#[inline(always)]
pub fn ublk_integrity_data_len(q: &ublk_queue, integrity_len: size_t) -> size_t {
    (integrity_len / q.metadata_size as size_t) << 9
}

#[inline(always)]
pub fn ublk_io_auto_zc_fallback(iod: &ublksrv_io_desc) -> c_int {
    ((iod.op_flags as __u64 & UBLK_IO_F_NEED_REG_BUF) != 0) as c_int
}

#[inline(always)]
pub fn ublk_user_copy_offset(q_id: c_uint, tag: c_uint) -> __u64 {
    UBLKSRV_IO_BUF_OFFSET + (((q_id as __u64) << UBLK_QID_OFF) | ((tag as __u64) << UBLK_TAG_OFF))
}

#[inline(always)]
pub fn is_target_io(user_data: __u64) -> c_int {
    ((user_data & (1u64 << 63)) != 0) as c_int
}

#[inline(always)]
pub fn build_user_data(tag: c_uint, op: c_uint, tgt_data: c_uint, q_id: c_uint, is_target_io: c_uint) -> __u64 {
    // _Static_assert(UBLK_MAX_QUEUES_SHIFT <= 7, "UBLK_MAX_QUEUES_SHIFT must be <= 7")
    const _: [(); 1] = [(); (UBLK_MAX_QUEUES_SHIFT <= 7) as usize];
    ublk_assert((tag >> 16) == 0 && (op >> 8) == 0 && (tgt_data >> 16) == 0 && (q_id >> 7) == 0);

    (tag as __u64)
        | ((op as __u64) << 16)
        | ((tgt_data as __u64) << 24)
        | ((q_id as __u64) << 56)
        | ((is_target_io as __u64) << 63)
}

#[inline(always)]
pub fn user_data_to_tag(user_data: __u64) -> c_uint {
    (user_data & 0xffff) as c_uint
}

#[inline(always)]
pub fn user_data_to_op(user_data: __u64) -> c_uint {
    ((user_data >> 16) & 0xff) as c_uint
}

#[inline(always)]
pub fn user_data_to_tgt_data(user_data: __u64) -> c_uint {
    ((user_data >> 24) & 0xffff) as c_uint
}

#[inline(always)]
pub fn user_data_to_q_id(user_data: __u64) -> c_uint {
    ((user_data >> 56) & 0x7f) as c_uint
}

#[inline(always)]
pub fn ublk_cmd_op_nr(op: c_uint) -> c_ushort {
    // _IOC_NR(op)
    (op & 0xffff) as c_ushort
}

#[inline(always)]
pub unsafe fn ublk_io_to_queue(io: *const ublk_io) -> *mut ublk_queue {
    let io_ptr = io as *const c_uchar;
    let uninit = MaybeUninit::<ublk_queue>::uninit();
    let base = uninit.as_ptr() as usize;
    let field = &(*uninit.as_ptr()).ios as *const _ as usize;
    let offset = field - base;
    io_ptr.sub(offset) as *mut ublk_queue
}

#[inline(always)]
pub unsafe fn ublk_io_alloc_sqes(
    t: *mut ublk_thread,
    sqes: *mut *mut io_uring_sqe,
    nr_sqes: c_int,
) -> c_int {
    let ring = &mut (*t).ring as *mut io_uring;
    let left = io_uring_sq_space_left(ring) as c_int;

    if left < nr_sqes {
        io_uring_submit(ring);
    }

    let mut i = 0;
    while i < nr_sqes {
        *sqes.add(i as usize) = io_uring_get_sqe(ring);
        if (*sqes.add(i as usize)).is_null() {
            return i;
        }
        i += 1;
    }

    nr_sqes
}

#[inline(always)]
pub fn ublk_get_registered_fd(q: &ublk_queue, fd_index: c_int) -> c_int {
    if (q.flags & UBLKS_Q_NO_UBLK_FIXED_FD) != 0 {
        if fd_index == 0 {
            q.ublk_fd
        } else {
            fd_index - 1
        }
    } else {
        fd_index
    }
}

#[inline(always)]
pub unsafe fn __io_uring_prep_buf_reg_unreg(
    sqe: *mut io_uring_sqe,
    q: *mut ublk_queue,
    tag: c_int,
    q_id: c_int,
    index: __u64,
) {
    let cmd = &mut *(sqe.as_mut().unwrap().cmd.as_mut_ptr() as *mut ublksrv_io_cmd);
    let dev_fd = ublk_get_registered_fd(&*q, 0);

    io_uring_prep_read(sqe, dev_fd, core::ptr::null_mut(), 0, 0);
    (*sqe).opcode = IORING_OP_URING_CMD;
    if ((*q).flags & UBLKS_Q_NO_UBLK_FIXED_FD) != 0 {
        (*sqe).flags &= !IOSQE_FIXED_FILE;
    } else {
        (*sqe).flags |= IOSQE_FIXED_FILE;
    }

    cmd.tag = tag;
    cmd.addr = index;
    cmd.q_id = q_id;
}

#[inline(always)]
pub unsafe fn io_uring_prep_buf_register(
    sqe: *mut io_uring_sqe,
    q: *mut ublk_queue,
    tag: c_int,
    q_id: c_int,
    index: __u64,
) {
    __io_uring_prep_buf_reg_unreg(sqe, q, tag, q_id, index);
    ublk_set_sqe_cmd_op(sqe, UBLK_U_IO_REGISTER_IO_BUF);
}

#[inline(always)]
pub unsafe fn io_uring_prep_buf_unregister(
    sqe: *mut io_uring_sqe,
    q: *mut ublk_queue,
    tag: c_int,
    q_id: c_int,
    index: __u64,
) {
    __io_uring_prep_buf_reg_unreg(sqe, q, tag, q_id, index);
    ublk_set_sqe_cmd_op(sqe, UBLK_U_IO_UNREGISTER_IO_BUF);
}

#[inline(always)]
pub fn ublk_get_sqe_cmd(sqe: &io_uring_sqe) -> *const c_void {
    &sqe.cmd as *const _ as *const c_void
}

#[inline(always)]
pub unsafe fn ublk_set_io_res(q: *mut ublk_queue, tag: c_int, res: c_int) {
    (*q).ios[tag as usize].result = res;
}

#[inline(always)]
pub unsafe fn ublk_get_io_res(q: *const ublk_queue, tag: c_uint) -> c_int {
    (*q).ios[tag as usize].result
}

#[inline(always)]
pub unsafe fn ublk_mark_io_done(io: *mut ublk_io, res: c_int) {
    (*io).flags = ((*io).flags | UBLKS_IO_NEED_COMMIT_RQ_COMP as c_ushort) | UBLKS_IO_FREE as c_ushort;
    (*io).result = res;
}

#[inline(always)]
pub unsafe fn ublk_get_iod(q: *const ublk_queue, tag: c_ushort) -> *const ublksrv_io_desc {
    (q as *const c_uchar).add(((*q).io_desc_size as usize) * (tag as usize)) as *const ublksrv_io_desc
}

#[inline(always)]
pub fn ublk_set_sqe_cmd_op(sqe: &mut io_uring_sqe, cmd_op: __u32) {
    let addr = &mut sqe.off as *mut __u64 as *mut __u32;
    unsafe {
        *addr = cmd_op;
        *addr.add(1) = 0;
    }
}

#[inline(always)]
pub unsafe fn ublk_io_buf_idx(t: *const ublk_thread, q: *const ublk_queue, tag: c_uint) -> c_ushort {
    if ublk_queue_batch_io(&*q) != 0 {
        ublk_batch_io_buf_idx(t, q, tag)
    } else {
        (*q).ios[tag as usize].buf_index
    }
}

#[inline(always)]
pub unsafe fn ublk_batch_io_buf_idx(t: *const ublk_thread, q: *const ublk_queue, tag: c_uint) -> c_ushort {
    let base = ublk_queue_idx_in_thread(t, q).wrapping_mul((*q).q_depth as c_uint) + tag;
    if ((*q).flags & UBLKS_Q_ROTATE_AUTO_BUF) != 0 {
        base + ((*q).ios[tag as usize].auto_buf_phase as c_uint) * (*t).auto_buf_stride as c_uint
    } else {
        base
    } as c_ushort
}

#[inline(always)]
pub unsafe fn ublk_batch_io_buf_idx_next(t: *mut ublk_thread, q: *mut ublk_queue, tag: c_uint) -> c_ushort {
    if ((*q).flags & UBLKS_Q_ROTATE_AUTO_BUF) != 0 {
        (*q).ios[tag as usize].auto_buf_phase ^= 1;
    }
    ublk_batch_io_buf_idx(t, q, tag)
}

#[inline(always)]
pub unsafe fn ublk_get_io(q: *mut ublk_queue, tag: c_uint) -> *mut ublk_io {
    (*q).ios.as_mut_ptr().add(tag as usize)
}

#[inline(always)]
pub unsafe fn ublk_completed_tgt_io(t: *mut ublk_thread, q: *mut ublk_queue, tag: c_uint) -> c_int {
    let io = ublk_get_io(q, tag);
    (*t).io_inflight -= 1;
    (*io).tgt_ios -= 1;
    ((*io).tgt_ios == 0) as c_int
}

#[inline(always)]
pub fn ublk_queue_use_zc(q: &ublk_queue) -> bool {
    (q.flags & UBLK_F_SUPPORT_ZERO_COPY) != 0
}

#[inline(always)]
pub fn ublk_queue_use_auto_zc(q: &ublk_queue) -> bool {
    (q.flags & UBLK_F_AUTO_BUF_REG) != 0
}

#[inline(always)]
pub fn ublk_queue_auto_zc_fallback(q: &ublk_queue) -> bool {
    (q.flags & UBLKS_Q_AUTO_BUF_REG_FALLBACK) != 0
}

#[inline(always)]
pub fn ublk_queue_use_user_copy(q: &ublk_queue) -> bool {
    (q.flags & UBLK_F_USER_COPY) != 0
}

#[inline(always)]
pub fn ublk_queue_no_buf(q: &ublk_queue) -> c_int {
    (ublk_queue_use_zc(q) || ublk_queue_use_auto_zc(q)) as c_int
}

#[inline(always)]
pub fn ublk_batch_commit_prepared(cb: *mut batch_commit_buf) -> c_int {
    unsafe {
        if cb.is_null() {
            return 0;
        }
        ((*cb).buf_idx != UBLKS_T_COMMIT_BUF_INV_IDX) as c_int
    }
}

#[inline(always)]
pub unsafe fn ublk_queue_idx_in_thread(t: *const ublk_thread, q: *const ublk_queue) -> c_uint {
    let idx = (*t).q_map[(*q).q_id as usize];
    ublk_assert(idx != 0);
    (idx - 1) as c_uint
}

extern "C" {
    pub fn ublk_batch_queue_prep_io_cmds(t: *mut ublk_thread, q: *mut ublk_queue) -> c_int;
    pub fn ublk_batch_start_fetch(t: *mut ublk_thread);
    pub fn ublk_batch_compl_cmd(t: *mut ublk_thread, cqe: *const io_uring_cqe);
    pub fn ublk_batch_prepare(t: *mut ublk_thread);
    pub fn ublk_batch_alloc_buf(t: *mut ublk_thread) -> c_int;
    pub fn ublk_batch_free_buf(t: *mut ublk_thread);
    pub fn ublk_batch_prep_commit(t: *mut ublk_thread);
    pub fn ublk_batch_commit_io_cmds(t: *mut ublk_thread);
    pub fn ublk_batch_complete_io(t: *mut ublk_thread, q: *mut ublk_queue, tag: c_uint, res: c_int);
    pub fn ublk_batch_setup_map(q_thread_map: *mut [c_uchar; UBLK_MAX_QUEUES], nthreads: c_int, queues: c_int);
}

#[inline(always)]
pub unsafe fn ublk_complete_io(t: *mut ublk_thread, q: *mut ublk_queue, tag: c_uint, res: c_int) -> c_int {
    if ublk_queue_batch_io(&*q) != 0 {
        ublk_batch_complete_io(t, q, tag, res);
        0
    } else {
        let io = &mut (*q).ios[tag as usize];
        ublk_mark_io_done(io as *mut ublk_io, res);
        ublk_queue_io_cmd(t, io as *mut _)
    }
}

#[inline(always)]
pub unsafe fn ublk_queued_tgt_io(t: *mut ublk_thread, q: *mut ublk_queue, tag: c_uint, queued: c_int) {
    if queued < 0 {
        ublk_complete_io(t, q, tag, queued);
    } else {
        let io = ublk_get_io(q, tag);
        (*t).io_inflight += queued as c_uint;
        (*io).tgt_ios = queued as c_ushort;
        (*io).result = 0;
    }
}


// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
