/* SPDX-License-Identifier: MIT */
/*
 * Description: uring_cmd based ublk
 *
 * Rust source-level translation of kublk.c.  C includes from <linux/fs.h>,
 * <sys/un.h>, and "kublk.h" are represented by external declarations and
 * opaque C-layout types below; the supplying crate/module is expected to
 * provide the real definitions.
 */

#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(dead_code)]
#![allow(improper_ctypes)]
#![allow(static_mut_refs)]

use core::ffi::{c_char, c_int, c_long, c_uint, c_ulong, c_ulonglong, c_void};
use core::mem::{size_of, zeroed};
use core::ptr::{null, null_mut};

type __u8 = u8;
type __u16 = u16;
type __u32 = u32;
type __u64 = u64;
type size_t = usize;
type ssize_t = isize;
type off_t = i64;
type pthread_t = c_ulong;

const MAX_NR_TGT_ARG: usize = 64;
const KUBLK_PARAM_LOGICAL_BS_SHIFT: __u32 = 9;
const KUBLK_PARAM_PHYSICAL_BS_SHIFT: __u32 = 12;
const KUBLK_PARAM_ZONE_SECTORS: __u64 = 128;
const KUBLK_PARAM_NR_ZONES: __u64 = 16;
const KUBLK_PARAM_DEV_SECTORS: __u64 =
    KUBLK_PARAM_ZONE_SECTORS * KUBLK_PARAM_NR_ZONES;
const KUBLK_PARAM_ZONE_APPEND_SECTORS: __u32 = 8;

const WAIT_USEC: c_uint = 100000;
const MAX_WAIT_USEC: c_uint = 3 * 1000000;
const UBLK_USER_COPY_LEN: __u32 = 2048;
const UBLK_SHMEM_SOCK_DIR: &[u8] = b"/run/ublk\0";

#[repr(C)]
pub struct io_uring {
    pub ring_fd: c_int,
    _priv: [u8; 0],
}
#[repr(C)]
pub struct io_uring_params {
    pub flags: c_uint,
    pub cq_entries: c_uint,
    _priv: [u8; 0],
}
#[repr(C)]
pub struct io_uring_sqe {
    pub opcode: __u8,
    pub flags: __u8,
    pub ioprio: __u16,
    pub fd: c_int,
    pub off: __u64,
    pub addr: __u64,
    pub len: __u32,
    pub rw_flags: __u32,
    _priv: [u8; 0],
}
#[repr(C)]
pub struct io_uring_cqe {
    pub user_data: __u64,
    pub res: c_int,
    pub flags: c_uint,
}
#[repr(C)]
pub struct cpu_set_t {
    _priv: [u8; 0],
}
#[repr(C)]
pub struct sem_t {
    _priv: [u8; 0],
}
#[repr(C)]
pub struct stat {
    pub st_size: off_t,
    _priv: [u8; 0],
}
#[repr(C)]
pub struct sockaddr {
    _priv: [u8; 0],
}
#[repr(C)]
pub struct sockaddr_un {
    pub sun_family: c_ushort,
    pub sun_path: [c_char; 108],
}
type c_ushort = u16;
#[repr(C)]
pub struct iovec {
    pub iov_base: *mut c_void,
    pub iov_len: size_t,
}
#[repr(C)]
pub struct msghdr {
    pub msg_name: *mut c_void,
    pub msg_namelen: c_uint,
    pub msg_iov: *mut iovec,
    pub msg_iovlen: size_t,
    pub msg_control: *mut c_void,
    pub msg_controllen: size_t,
    pub msg_flags: c_int,
}
#[repr(C)]
pub struct cmsghdr {
    pub cmsg_len: size_t,
    pub cmsg_level: c_int,
    pub cmsg_type: c_int,
}
#[repr(C)]
pub struct pollfd {
    pub fd: c_int,
    pub events: c_short,
    pub revents: c_short,
}
type c_short = i16;
#[repr(C)]
pub struct inotify_event {
    pub wd: c_int,
    pub mask: __u32,
    pub cookie: __u32,
    pub len: __u32,
    pub name: [c_char; 0],
}
#[repr(C)]
pub struct option {
    pub name: *const c_char,
    pub has_arg: c_int,
    pub flag: *mut c_int,
    pub val: c_int,
}

#[repr(C)]
pub struct ublksrv_ctrl_dev_info {
    pub dev_id: c_int,
    pub ublksrv_pid: c_int,
    pub nr_hw_queues: c_uint,
    pub queue_depth: c_uint,
    pub max_io_buf_bytes: c_uint,
    pub flags: __u64,
    pub state: c_int,
    pub io_desc_size: c_uint,
}
#[repr(C)]
pub struct ublksrv_ctrl_cmd {
    pub addr: __u64,
    pub len: __u32,
    pub dev_id: c_int,
    pub queue_id: c_int,
    pub data: [__u64; 2],
}
#[repr(C)]
pub struct ublksrv_io_cmd {
    pub q_id: __u16,
    pub tag: __u16,
    pub result: c_int,
    pub addr: __u64,
}
#[repr(C)]
pub struct ublksrv_io_desc {
    pub op_flags: __u32,
    pub nr_sectors: __u32,
    _priv: [u8; 0],
}
#[repr(C)]
pub struct ublk_params_basic {
    pub logical_bs_shift: __u32,
    pub physical_bs_shift: __u32,
    pub io_min_shift: __u32,
    pub io_opt_shift: __u32,
    pub max_sectors: __u32,
    pub chunk_sectors: __u32,
    pub dev_sectors: __u64,
}
#[repr(C)]
pub struct ublk_params_zoned {
    pub max_open_zones: __u32,
    pub max_active_zones: __u32,
    pub max_zone_append_sectors: __u32,
}
#[repr(C)]
pub struct ublk_params {
    pub len: __u32,
    pub types: __u32,
    pub basic: ublk_params_basic,
    pub zoned: ublk_params_zoned,
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
pub struct dev_ctx {
    pub _evtfd: c_int,
    pub _shmid: c_int,
    pub queue_depth: c_uint,
    pub nr_hw_queues: c_uint,
    pub dev_id: c_int,
    pub tgt_type: [c_char; 64],
    pub csum_type: c_int,
    pub io_desc_size: size_t,
    pub params: params_ctx,
    pub all: bool,
    pub fg: bool,
    pub flags: __u64,
    pub nthreads: c_uint,
    pub per_io_tasks: bool,
    pub auto_zc_fallback: bool,
    pub rotate_auto_buf: bool,
    pub no_ublk_fixed_fd: bool,
    pub metadata_size: __u8,
    pub integrity_flags: __u32,
    pub pi_offset: __u32,
    pub tag_size: __u32,
    pub safe_stop: bool,
    pub rdonly_shmem_buf: bool,
    pub recovery: bool,
    pub logging: bool,
    pub size: __u64,
    pub nr_files: c_int,
    pub files: [*mut c_char; MAX_BACK_FILES],
    pub htlb_path: *mut c_char,
    pub shadow_dev: *mut ublk_dev,
}
#[repr(C)]
pub struct ublk_ctrl_cmd_data {
    pub cmd_op: c_uint,
    pub flags: c_uint,
    pub addr: __u64,
    pub len: __u32,
    pub data: [__u64; 2],
}
#[repr(C)]
pub struct ublk_auto_buf_reg {
    pub index: __u32,
    pub flags: __u32,
}
#[repr(C)]
pub struct ublk_shmem_buf_reg {
    pub addr: c_ulong,
    pub len: size_t,
    pub flags: __u32,
}
#[repr(C)]
pub struct ublk_shmem_entry {
    pub fd: c_int,
    pub mmap_base: *mut c_void,
    pub size: size_t,
}
#[repr(C)]
pub struct ublk_tgt_ops {
    pub name: *const c_char,
    pub init_tgt: Option<unsafe extern "C" fn(*const dev_ctx, *mut ublk_dev) -> c_int>,
    pub deinit_tgt: Option<unsafe extern "C" fn(*mut ublk_dev)>,
    pub usage: Option<unsafe extern "C" fn(*const ublk_tgt_ops)>,
    pub parse_cmd_line: Option<unsafe extern "C" fn(*mut dev_ctx, c_int, *mut *mut c_char)>,
    pub pre_fetch_io: Option<unsafe extern "C" fn(*mut ublk_thread, *mut ublk_queue, c_int, bool)>,
    pub queue_io: Option<unsafe extern "C" fn(*mut ublk_thread, *mut ublk_queue, c_uint)>,
    pub tgt_io_done: Option<unsafe extern "C" fn(*mut ublk_thread, *mut ublk_queue, *mut io_uring_cqe)>,
    pub buf_index: Option<unsafe extern "C" fn(*const ublk_thread, *const ublk_queue, c_ushort) -> __u32>,
}
#[repr(C)]
pub struct ublk_tgt {
    pub ops: *const ublk_tgt_ops,
    pub params: ublk_params,
    pub sq_depth: c_int,
    pub cq_depth: c_int,
    pub nr_backing_files: c_int,
    pub backing_file: [[c_char; 256]; MAX_BACK_FILES],
}
#[repr(C)]
pub struct ublk_io {
    pub buf_addr: *mut c_void,
    pub integrity_buf: *mut c_void,
    pub flags: c_uint,
    pub tag: c_uint,
    pub result: c_int,
    pub buf_index: c_int,
}
#[repr(C)]
pub struct ublk_queue {
    pub dev: *mut ublk_dev,
    pub tgt_ops: *const ublk_tgt_ops,
    pub flags: __u64,
    pub q_depth: c_int,
    pub q_id: c_int,
    pub metadata_size: __u8,
    pub io_desc_size: __u16,
    pub ublk_fd: c_int,
    pub io_cmd_buf: *mut c_void,
    pub lock: c_ulong,
    pub ios: [ublk_io; UBLK_QUEUE_DEPTH],
}
#[repr(C)]
pub struct ublk_thread {
    pub dev: *mut ublk_dev,
    pub ring: io_uring,
    pub idx: c_uint,
    pub state: c_uint,
    pub cmd_inflight: c_uint,
    pub io_inflight: c_uint,
    pub nr_bufs: c_uint,
    pub auto_buf_stride: c_uint,
    pub q_map: [__u8; UBLK_MAX_QUEUES],
}
#[repr(C)]
pub struct ublk_dev {
    pub ctrl_fd: c_int,
    pub ring: io_uring,
    pub dev_info: ublksrv_ctrl_dev_info,
    pub fds: [c_int; MAX_BACK_FILES + 1],
    pub nr_fds: c_int,
    pub nthreads: c_uint,
    pub per_io_tasks: bool,
    pub tgt: ublk_tgt,
    pub q: [ublk_queue; UBLK_MAX_QUEUES],
}

const MAX_BACK_FILES: usize = 8;
const UBLK_QUEUE_DEPTH: usize = 1024;
const UBLK_MAX_QUEUE_DEPTH: __u16 = 1024;
const UBLK_MAX_QUEUES: usize = 32;
const UBLK_MAX_THREADS: c_uint = 64;
const UBLK_BUF_MAX: usize = 1024;
const UBLK_CTRL_RING_DEPTH: c_int = 16;
const UBLK_LOG: c_uint = 1;
const UBLK_DBG_IO_CMD: c_uint = 1 << 1;
const UBLK_DBG_THREAD: c_uint = 1 << 2;
const UBLK_DBG_DEV: c_uint = 1 << 3;
const CTRL_CMD_HAS_BUF: c_uint = 1;
const CTRL_CMD_HAS_DATA: c_uint = 2;
const ERROR_EVTFD_DEVID: u64 = !0;

unsafe extern "C" {
    static null_tgt_ops: ublk_tgt_ops;
    static loop_tgt_ops: ublk_tgt_ops;
    static stripe_tgt_ops: ublk_tgt_ops;
    static fault_inject_tgt_ops: ublk_tgt_ops;
    static mut optarg: *mut c_char;
    static mut optind: c_int;
    static mut opterr: c_int;
    static mut errno: c_int;
    static mut stdout: *mut c_void;
    static mut stderr: *mut c_void;

    fn strcmp(a: *const c_char, b: *const c_char) -> c_int;
    fn strlen(s: *const c_char) -> size_t;
    fn strcpy(dst: *mut c_char, src: *const c_char) -> *mut c_char;
    fn snprintf(s: *mut c_char, n: size_t, fmt: *const c_char, ...) -> c_int;
    fn printf(fmt: *const c_char, ...) -> c_int;
    fn fprintf(stream: *mut c_void, fmt: *const c_char, ...) -> c_int;
    fn fflush(stream: *mut c_void) -> c_int;
    fn memset(s: *mut c_void, c: c_int, n: size_t) -> *mut c_void;
    fn memcpy(dst: *mut c_void, src: *const c_void, n: size_t) -> *mut c_void;
    fn malloc(size: size_t) -> *mut c_void;
    fn calloc(nmemb: size_t, size: size_t) -> *mut c_void;
    fn free(ptr: *mut c_void);
    fn posix_memalign(memptr: *mut *mut c_void, alignment: size_t, size: size_t) -> c_int;
    fn open(path: *const c_char, flags: c_int, ...) -> c_int;
    fn close(fd: c_int) -> c_int;
    fn read(fd: c_int, buf: *mut c_void, count: size_t) -> ssize_t;
    fn write(fd: c_int, buf: *const c_void, count: size_t) -> ssize_t;
    fn pread(fd: c_int, buf: *mut c_void, count: size_t, offset: off_t) -> ssize_t;
    fn pwrite(fd: c_int, buf: *const c_void, count: size_t, offset: off_t) -> ssize_t;
    fn lseek(fd: c_int, offset: off_t, whence: c_int) -> off_t;
    fn mmap(addr: *mut c_void, length: size_t, prot: c_int, flags: c_int, fd: c_int, offset: off_t) -> *mut c_void;
    fn munmap(addr: *mut c_void, length: size_t) -> c_int;
    fn getpagesize() -> c_int;
    fn usleep(usec: c_uint) -> c_int;
    fn strerror(errnum: c_int) -> *mut c_char;
    fn basename(path: *const c_char) -> *mut c_char;
    fn socket(domain: c_int, typ: c_int, protocol: c_int) -> c_int;
    fn bind(sockfd: c_int, addr: *const sockaddr, addrlen: c_uint) -> c_int;
    fn listen(sockfd: c_int, backlog: c_int) -> c_int;
    fn accept(sockfd: c_int, addr: *mut sockaddr, addrlen: *mut c_uint) -> c_int;
    fn send(sockfd: c_int, buf: *const c_void, len: size_t, flags: c_int) -> ssize_t;
    fn recvmsg(sockfd: c_int, msg: *mut msghdr, flags: c_int) -> ssize_t;
    fn mkdir(path: *const c_char, mode: c_uint) -> c_int;
    fn unlink(path: *const c_char) -> c_int;
    fn poll(fds: *mut pollfd, nfds: c_ulong, timeout: c_int) -> c_int;
    fn eventfd(initval: c_uint, flags: c_int) -> c_int;
    fn shmdt(shmaddr: *const c_void) -> c_int;
    fn shmget(key: c_int, size: size_t, shmflg: c_int) -> c_int;
    fn shmat(shmid: c_int, shmaddr: *const c_void, shmflg: c_int) -> *mut c_void;
    fn shmctl(shmid: c_int, cmd: c_int, buf: *mut c_void) -> c_int;
    fn fork() -> c_int;
    fn setsid() -> c_int;
    fn exit(status: c_int) -> !;
    fn wait(status: *mut c_int) -> c_int;
    fn waitpid(pid: c_int, status: *mut c_int, options: c_int) -> c_int;
    fn kill(pid: c_int, sig: c_int) -> c_int;
    fn access(pathname: *const c_char, mode: c_int) -> c_int;
    fn getpid() -> c_int;
    fn gettid() -> c_int;
    fn fstat(fd: c_int, statbuf: *mut stat) -> c_int;
    fn inotify_init() -> c_int;
    fn inotify_add_watch(fd: c_int, pathname: *const c_char, mask: __u32) -> c_int;
    fn inotify_rm_watch(fd: c_int, wd: c_int) -> c_int;
    fn pthread_create(thread: *mut pthread_t, attr: *const c_void, start_routine: unsafe extern "C" fn(*mut c_void) -> *mut c_void, arg: *mut c_void) -> c_int;
    fn pthread_join(thread: pthread_t, retval: *mut *mut c_void) -> c_int;
    fn pthread_self() -> pthread_t;
    fn pthread_setaffinity_np(thread: pthread_t, cpusetsize: size_t, cpuset: *const cpu_set_t) -> c_int;
    fn pthread_spin_init(lock: *mut c_ulong, pshared: c_int) -> c_int;
    fn sem_init(sem: *mut sem_t, pshared: c_int, value: c_uint) -> c_int;
    fn sem_wait(sem: *mut sem_t) -> c_int;
    fn sem_post(sem: *mut sem_t) -> c_int;
    fn getopt_long(argc: c_int, argv: *mut *mut c_char, optstring: *const c_char, longopts: *const option, longindex: *mut c_int) -> c_int;
    fn strtol(nptr: *const c_char, endptr: *mut *mut c_char, base: c_int) -> c_long;
    fn strtoul(nptr: *const c_char, endptr: *mut *mut c_char, base: c_int) -> c_ulong;
    fn strtoull(nptr: *const c_char, endptr: *mut *mut c_char, base: c_int) -> c_ulonglong;
    fn strdup(s: *const c_char) -> *mut c_char;
    fn strtok_r(str_: *mut c_char, delim: *const c_char, saveptr: *mut *mut c_char) -> *mut c_char;

    fn io_uring_queue_init_params(entries: c_uint, ring: *mut io_uring, p: *mut io_uring_params) -> c_int;
    fn io_uring_get_sqe(ring: *mut io_uring) -> *mut io_uring_sqe;
    fn io_uring_submit(ring: *mut io_uring) -> c_int;
    fn io_uring_wait_cqe(ring: *mut io_uring, cqe_ptr: *mut *mut io_uring_cqe) -> c_int;
    fn io_uring_cqe_seen(ring: *mut io_uring, cqe: *mut io_uring_cqe);
    fn io_uring_sqe_set_data(sqe: *mut io_uring_sqe, data: *mut c_void);
    fn io_uring_sqe_set_data64(sqe: *mut io_uring_sqe, data: __u64);
    fn io_uring_sq_space_left(ring: *mut io_uring) -> c_uint;
    fn io_uring_sq_ready(ring: *mut io_uring) -> c_uint;
    fn io_uring_submit_and_wait(ring: *mut io_uring, wait_nr: c_uint) -> c_int;
    fn io_uring_cq_advance(ring: *mut io_uring, nr: c_uint);
    fn io_uring_unregister_buffers(ring: *mut io_uring) -> c_int;
    fn io_uring_unregister_ring_fd(ring: *mut io_uring) -> c_int;
    fn io_uring_unregister_files(ring: *mut io_uring) -> c_int;
    fn io_uring_register_buffers_sparse(ring: *mut io_uring, nr: c_uint) -> c_int;
    fn io_uring_register_ring_fd(ring: *mut io_uring) -> c_int;
    fn io_uring_register_files(ring: *mut io_uring, files: *const c_int, nr_files: c_uint) -> c_int;

    fn ublk_get_sqe_cmd(sqe: *mut io_uring_sqe) -> *mut c_void;
    fn ublk_set_sqe_cmd_op(sqe: *mut io_uring_sqe, op: c_uint);
    fn ublk_err(fmt: *const c_char, ...);
    fn ublk_log(fmt: *const c_char, ...);
    fn ublk_dbg(mask: c_uint, fmt: *const c_char, ...);
    fn ublk_assert(cond: bool);
    fn round_up(size: size_t, align: size_t) -> size_t;
    fn CPU_ISSET(cpu: c_int, set: *const cpu_set_t) -> bool;
    fn CPU_CLR(cpu: c_int, set: *mut cpu_set_t);
    fn ublk_integrity_len(q: *const ublk_queue, len: __u32) -> __u32;
    fn ublk_queue_no_buf(q: *const ublk_queue) -> bool;
    fn ublk_dev_batch_io(dev: *const ublk_dev) -> bool;
    fn ublk_thread_batch_io(t: *const ublk_thread) -> bool;
    fn ublk_batch_prepare(t: *mut ublk_thread);
    fn ublk_batch_alloc_buf(t: *mut ublk_thread) -> c_int;
    fn ublk_batch_free_buf(t: *mut ublk_thread);
    fn ublk_batch_setup_map(map: *mut [__u8; UBLK_MAX_QUEUES], nthreads: c_uint, nr_hw_queues: c_uint);
    fn ublk_batch_queue_prep_io_cmds(t: *mut ublk_thread, q: *mut ublk_queue) -> c_int;
    fn ublk_batch_start_fetch(t: *mut ublk_thread);
    fn ublk_batch_compl_cmd(t: *mut ublk_thread, cqe: *const io_uring_cqe);
    fn ublk_batch_prep_commit(t: *mut ublk_thread);
    fn ublk_batch_commit_io_cmds(t: *mut ublk_thread);
    fn ublk_io_to_queue(io: *const ublk_io) -> *mut ublk_queue;
    fn ublk_get_iod(q: *const ublk_queue, tag: c_uint) -> *const ublksrv_io_desc;
    fn ublk_user_copy_offset(q_id: c_int, tag: c_uint) -> __u64;
    fn ublksrv_get_op(iod: *const ublksrv_io_desc) -> __u8;
    fn ublk_queue_use_user_copy(q: *const ublk_queue) -> bool;
    fn ublk_queue_use_auto_zc(q: *const ublk_queue) -> bool;
    fn ublk_queue_auto_zc_fallback(q: *const ublk_queue) -> bool;
    fn ublk_auto_buf_reg_to_sqe_addr(buf: *const ublk_auto_buf_reg) -> __u64;
    fn ublk_io_buf_idx(t: *const ublk_thread, q: *const ublk_queue, tag: c_ushort) -> __u32;
    fn ublk_get_registered_fd(q: *const ublk_queue, idx: c_int) -> c_int;
    fn ublk_io_alloc_sqes(t: *mut ublk_thread, sqe: *mut *mut io_uring_sqe, nr: c_int);
    fn build_user_data(tag: c_uint, op: c_uint, target: c_uint, q_id: c_int, data: c_uint) -> __u64;
    fn user_data_to_tag(user_data: __u64) -> c_uint;
    fn user_data_to_op(user_data: __u64) -> c_uint;
    fn user_data_to_q_id(user_data: __u64) -> c_uint;
    fn user_data_to_tgt_data(user_data: __u64) -> c_uint;
    fn is_target_io(user_data: __u64) -> bool;
    fn _IOC_NR(nr: c_uint) -> c_uint;
}

const EINVAL: c_int = 22;
const ENOMEM: c_int = 12;
const ENODEV: c_int = 19;
const ENOBUFS: c_int = 105;
const EAGAIN: c_int = 11;
const EBADF: c_int = 9;
const ETIMEDOUT: c_int = 110;
const EOPNOTSUPP: c_int = 95;
const ENOTSUP: c_int = 95;
const O_RDWR: c_int = 2;
const F_OK: c_int = 0;
const PROT_READ: c_int = 1;
const PROT_WRITE: c_int = 2;
const MAP_SHARED: c_int = 1;
const MAP_POPULATE: c_int = 0x8000;
const MAP_FAILED: *mut c_void = !0usize as *mut c_void;
const SEEK_END: c_int = 2;
const AF_UNIX: c_int = 1;
const SOCK_STREAM: c_int = 1;
const SOCK_NONBLOCK: c_int = 0o0004000;
const SOL_SOCKET: c_int = 1;
const SCM_RIGHTS: c_int = 1;
const POLLIN: c_short = 0x0001;
const POLL_IN: c_short = POLLIN;
const IN_CLOSE: c_int = 0x00000018;
const IPC_PRIVATE: c_int = 0;
const IPC_CREAT: c_int = 0o1000;
const IPC_RMID: c_int = 0;
const STDIN_FILENO: c_int = 0;
const STDOUT_FILENO: c_int = 1;
const STDERR_FILENO: c_int = 2;
const EXIT_SUCCESS: c_int = 0;
const EXIT_FAILURE: c_int = 1;
const PTHREAD_PROCESS_PRIVATE: c_int = 0;
const IORING_SETUP_CQSIZE: c_uint = 1 << 3;
const IORING_SETUP_SQE128: c_uint = 1 << 10;
const IORING_SETUP_COOP_TASKRUN: c_uint = 1 << 8;
const IORING_SETUP_SINGLE_ISSUER: c_uint = 1 << 12;
const IORING_SETUP_DEFER_TASKRUN: c_uint = 1 << 13;
const IORING_OP_URING_CMD: __u8 = 46;
const IOSQE_FIXED_FILE: __u8 = 1;
const CTRL_DEV: *const c_char = b"/dev/ublk-control\0".as_ptr() as *const c_char;
const UBLKC_DEV: *const c_char = b"/dev/ublkc\0".as_ptr() as *const c_char;
const UBLKSRV_CMD_BUF_OFFSET: c_ulong = 0;
const UBLKSRV_IO_INTEGRITY_FLAG: __u64 = 1 << 63;
const CPU_SETSIZE: c_int = 1024;

const UBLK_U_CMD_STOP_DEV: c_uint = 1;
const UBLK_U_CMD_TRY_STOP_DEV: c_uint = 2;
const UBLK_U_CMD_START_DEV: c_uint = 3;
const UBLK_U_CMD_START_USER_RECOVERY: c_uint = 4;
const UBLK_U_CMD_END_USER_RECOVERY: c_uint = 5;
const UBLK_U_CMD_ADD_DEV: c_uint = 6;
const UBLK_U_CMD_DEL_DEV: c_uint = 7;
const UBLK_U_CMD_GET_DEV_INFO: c_uint = 8;
const UBLK_U_CMD_SET_PARAMS: c_uint = 9;
const UBLK_U_CMD_GET_PARAMS: c_uint = 10;
const UBLK_U_CMD_GET_FEATURES: c_uint = 11;
const UBLK_U_CMD_UPDATE_SIZE: c_uint = 12;
const UBLK_U_CMD_QUIESCE_DEV: c_uint = 13;
const UBLK_U_CMD_GET_QUEUE_AFFINITY: c_uint = 14;
const UBLK_U_CMD_REG_BUF: c_uint = 15;
const UBLK_U_IO_NEED_GET_DATA: c_uint = 20;
const UBLK_U_IO_COMMIT_AND_FETCH_REQ: c_uint = 21;
const UBLK_U_IO_FETCH_REQ: c_uint = 22;
const UBLK_IO_RES_ABORT: c_int = -1;
const UBLK_IO_RES_OK: c_int = 0;
const UBLK_IO_RES_NEED_GET_DATA: c_int = 1;
const UBLK_IO_OP_WRITE: __u8 = 1;
const UBLK_IO_OP_READ: __u8 = 0;
const UBLK_IO_F_INTEGRITY: __u32 = 1 << 0;
const UBLKS_IO_NEED_FETCH_RQ: c_uint = 1 << 0;
const UBLKS_IO_NEED_COMMIT_RQ_COMP: c_uint = 1 << 1;
const UBLKS_IO_NEED_GET_DATA: c_uint = 1 << 2;
const UBLKS_IO_FREE: c_uint = 1 << 3;
const UBLKS_T_STOPPING: c_uint = 1 << 0;
const UBLKS_T_IDLE: c_uint = 1 << 1;
const UBLKS_Q_AUTO_BUF_REG_FALLBACK: __u64 = 1 << 0;
const UBLKS_Q_NO_UBLK_FIXED_FD: __u64 = 1 << 1;
const UBLKS_Q_ROTATE_AUTO_BUF: __u64 = 1 << 2;
const UBLK_AUTO_BUF_REG_FALLBACK: __u32 = 1;
const UBLK_PARAM_TYPE_BASIC: __u32 = 1;
const UBLK_PARAM_TYPE_ZONED: __u32 = 2;
const UBLK_IO_MAX_BYTES: c_uint = 128 * 1024;
const UBLK_S_DEV_DEAD: c_int = 0;
const UBLK_S_DEV_LIVE: c_int = 1;
const UBLK_S_DEV_QUIESCED: c_int = 2;
const UBLK_F_SUPPORT_ZERO_COPY: __u64 = 1 << 0;
const UBLK_F_URING_CMD_COMP_IN_TASK: __u64 = 1 << 1;
const UBLK_F_NEED_GET_DATA: __u64 = 1 << 2;
const UBLK_F_USER_RECOVERY: __u64 = 1 << 3;
const UBLK_F_USER_RECOVERY_REISSUE: __u64 = 1 << 4;
const UBLK_F_UNPRIVILEGED_DEV: __u64 = 1 << 5;
const UBLK_F_CMD_IOCTL_ENCODE: __u64 = 1 << 6;
const UBLK_F_USER_COPY: __u64 = 1 << 7;
const UBLK_F_ZONED: __u64 = 1 << 8;
const UBLK_F_USER_RECOVERY_FAIL_IO: __u64 = 1 << 9;
const UBLK_F_UPDATE_SIZE: __u64 = 1 << 10;
const UBLK_F_AUTO_BUF_REG: __u64 = 1 << 11;
const UBLK_F_QUIESCE: __u64 = 1 << 12;
const UBLK_F_PER_IO_DAEMON: __u64 = 1 << 13;
const UBLK_F_BUF_REG_OFF_DAEMON: __u64 = 1 << 14;
const UBLK_F_INTEGRITY: __u64 = 1 << 15;
const UBLK_F_SAFE_STOP_DEV: __u64 = 1 << 16;
const UBLK_F_BATCH_IO: __u64 = 1 << 17;
const UBLK_F_NO_AUTO_PART_SCAN: __u64 = 1 << 18;
const UBLK_F_SHMEM_ZC: __u64 = 1 << 19;
const UBLK_F_IO_DESC_SIZE: __u64 = 1 << 20;
const UBLK_SHMEM_BUF_READ_ONLY: __u32 = 1;
const LBMD_PI_CSUM_NONE: c_int = 0;
const LBMD_PI_CSUM_IP: c_int = 1;
const LBMD_PI_CSUM_CRC16_T10DIF: c_int = 2;
const LBMD_PI_CSUM_CRC64_NVME: c_int = 3;
const LBMD_PI_CAP_INTEGRITY: __u32 = 1;
const LBMD_PI_CAP_REFTAG: __u32 = 2;

pub static mut ublk_dbg_mask: c_uint = UBLK_LOG;
static tgt_ops_list: [*const ublk_tgt_ops; 4] = unsafe {
    [
        &null_tgt_ops,
        &loop_tgt_ops,
        &stripe_tgt_ops,
        &fault_inject_tgt_ops,
    ]
};
#[unsafe(no_mangle)]
pub static mut shmem_table: [ublk_shmem_entry; UBLK_BUF_MAX] = [ublk_shmem_entry {
    fd: 0,
    mmap_base: null_mut(),
    size: 0,
}; UBLK_BUF_MAX];
#[unsafe(no_mangle)]
pub static mut shmem_count: c_int = 0;

unsafe fn cstr(s: &'static [u8]) -> *const c_char {
    s.as_ptr() as *const c_char
}

unsafe fn ublk_find_tgt(name: *const c_char) -> *const ublk_tgt_ops {
    if name.is_null() {
        return null();
    }
    for ops in tgt_ops_list {
        if strcmp((*ops).name, name) == 0 {
            return ops;
        }
    }
    null()
}

unsafe fn ublk_setup_ring(r: *mut io_uring, depth: c_int, cq_depth: c_int, flags: c_uint) -> c_int {
    let mut p: io_uring_params = zeroed();
    p.flags = flags | IORING_SETUP_CQSIZE;
    p.cq_entries = cq_depth as c_uint;
    io_uring_queue_init_params(depth as c_uint, r, &mut p)
}

unsafe fn ublk_ctrl_init_cmd(dev: *mut ublk_dev, sqe: *mut io_uring_sqe, data: *mut ublk_ctrl_cmd_data) {
    let info = &mut (*dev).dev_info as *mut ublksrv_ctrl_dev_info;
    let cmd = ublk_get_sqe_cmd(sqe) as *mut ublksrv_ctrl_cmd;
    (*sqe).fd = (*dev).ctrl_fd;
    (*sqe).opcode = IORING_OP_URING_CMD;
    (*sqe).ioprio = 0;
    if (*data).flags & CTRL_CMD_HAS_BUF != 0 {
        (*cmd).addr = (*data).addr;
        (*cmd).len = (*data).len;
    }
    if (*data).flags & CTRL_CMD_HAS_DATA != 0 {
        (*cmd).data[0] = (*data).data[0];
    }
    (*cmd).dev_id = (*info).dev_id;
    (*cmd).queue_id = -1;
    ublk_set_sqe_cmd_op(sqe, (*data).cmd_op);
    io_uring_sqe_set_data(sqe, cmd as *mut c_void);
}

unsafe fn __ublk_ctrl_cmd(dev: *mut ublk_dev, data: *mut ublk_ctrl_cmd_data) -> c_int {
    let mut cqe: *mut io_uring_cqe = null_mut();
    let mut ret = -EINVAL;
    let sqe = io_uring_get_sqe(&mut (*dev).ring);
    if sqe.is_null() {
        ublk_err(cstr(b"%s: can't get sqe ret %d\n\0"), cstr(b"__ublk_ctrl_cmd\0"), ret);
        return ret;
    }
    ublk_ctrl_init_cmd(dev, sqe, data);
    ret = io_uring_submit(&mut (*dev).ring);
    if ret < 0 {
        ublk_err(cstr(b"uring submit ret %d\n\0"), ret);
        return ret;
    }
    ret = io_uring_wait_cqe(&mut (*dev).ring, &mut cqe);
    if ret < 0 {
        ublk_err(cstr(b"wait cqe: %s\n\0"), strerror(-ret));
        return ret;
    }
    io_uring_cqe_seen(&mut (*dev).ring, cqe);
    (*cqe).res
}

unsafe fn ublk_ctrl_stop_dev(dev: *mut ublk_dev) -> c_int {
    let mut data: ublk_ctrl_cmd_data = zeroed();
    data.cmd_op = UBLK_U_CMD_STOP_DEV;
    __ublk_ctrl_cmd(dev, &mut data)
}
unsafe fn ublk_ctrl_try_stop_dev(dev: *mut ublk_dev) -> c_int {
    let mut data: ublk_ctrl_cmd_data = zeroed();
    data.cmd_op = UBLK_U_CMD_TRY_STOP_DEV;
    __ublk_ctrl_cmd(dev, &mut data)
}
unsafe fn ublk_ctrl_start_dev(dev: *mut ublk_dev, daemon_pid: c_int) -> c_int {
    let mut data: ublk_ctrl_cmd_data = zeroed();
    data.cmd_op = UBLK_U_CMD_START_DEV;
    data.flags = CTRL_CMD_HAS_DATA;
    data.data[0] = daemon_pid as __u64;
    (*dev).dev_info.ublksrv_pid = daemon_pid;
    __ublk_ctrl_cmd(dev, &mut data)
}
unsafe fn ublk_ctrl_start_user_recovery(dev: *mut ublk_dev) -> c_int {
    let mut data: ublk_ctrl_cmd_data = zeroed();
    data.cmd_op = UBLK_U_CMD_START_USER_RECOVERY;
    __ublk_ctrl_cmd(dev, &mut data)
}
unsafe fn ublk_ctrl_end_user_recovery(dev: *mut ublk_dev, daemon_pid: c_int) -> c_int {
    let mut data: ublk_ctrl_cmd_data = zeroed();
    data.cmd_op = UBLK_U_CMD_END_USER_RECOVERY;
    data.flags = CTRL_CMD_HAS_DATA;
    data.data[0] = daemon_pid as __u64;
    (*dev).dev_info.ublksrv_pid = daemon_pid;
    __ublk_ctrl_cmd(dev, &mut data)
}
unsafe fn ublk_ctrl_add_dev(dev: *mut ublk_dev) -> c_int {
    let mut data: ublk_ctrl_cmd_data = zeroed();
    data.cmd_op = UBLK_U_CMD_ADD_DEV;
    data.flags = CTRL_CMD_HAS_BUF;
    data.addr = &mut (*dev).dev_info as *mut _ as usize as __u64;
    data.len = size_of::<ublksrv_ctrl_dev_info>() as __u32;
    __ublk_ctrl_cmd(dev, &mut data)
}
unsafe fn ublk_ctrl_del_dev(dev: *mut ublk_dev) -> c_int {
    let mut data: ublk_ctrl_cmd_data = zeroed();
    data.cmd_op = UBLK_U_CMD_DEL_DEV;
    __ublk_ctrl_cmd(dev, &mut data)
}
unsafe fn ublk_ctrl_get_info(dev: *mut ublk_dev) -> c_int {
    let mut data: ublk_ctrl_cmd_data = zeroed();
    data.cmd_op = UBLK_U_CMD_GET_DEV_INFO;
    data.flags = CTRL_CMD_HAS_BUF;
    data.addr = &mut (*dev).dev_info as *mut _ as usize as __u64;
    data.len = size_of::<ublksrv_ctrl_dev_info>() as __u32;
    __ublk_ctrl_cmd(dev, &mut data)
}
unsafe fn ublk_ctrl_set_params(dev: *mut ublk_dev, params: *mut ublk_params) -> c_int {
    let mut data: ublk_ctrl_cmd_data = zeroed();
    data.cmd_op = UBLK_U_CMD_SET_PARAMS;
    data.flags = CTRL_CMD_HAS_BUF;
    data.addr = params as usize as __u64;
    data.len = size_of::<ublk_params>() as __u32;
    (*params).len = size_of::<ublk_params>() as __u32;
    __ublk_ctrl_cmd(dev, &mut data)
}
unsafe fn ublk_ctrl_get_params(dev: *mut ublk_dev, params: *mut ublk_params) -> c_int {
    let mut data: ublk_ctrl_cmd_data = zeroed();
    data.cmd_op = UBLK_U_CMD_GET_PARAMS;
    data.flags = CTRL_CMD_HAS_BUF;
    data.addr = params as usize as __u64;
    data.len = size_of::<ublk_params>() as __u32;
    (*params).len = size_of::<ublk_params>() as __u32;
    __ublk_ctrl_cmd(dev, &mut data)
}
unsafe fn ublk_ctrl_get_features(dev: *mut ublk_dev, features: *mut __u64) -> c_int {
    let mut data: ublk_ctrl_cmd_data = zeroed();
    data.cmd_op = UBLK_U_CMD_GET_FEATURES;
    data.flags = CTRL_CMD_HAS_BUF;
    data.addr = features as usize as __u64;
    data.len = size_of::<__u64>() as __u32;
    __ublk_ctrl_cmd(dev, &mut data)
}

unsafe fn parse_param_types(arg: *const c_char, types: *mut __u32) -> c_int {
    let mut buf = [0 as c_char; 128];
    let mut save: *mut c_char = null_mut();
    if strlen(arg) >= buf.len() {
        return -EINVAL;
    }
    strcpy(buf.as_mut_ptr(), arg);
    *types = 0;
    let mut tok = strtok_r(buf.as_mut_ptr(), cstr(b",\0"), &mut save);
    while !tok.is_null() {
        if strcmp(tok, cstr(b"none\0")) == 0 {
        } else if strcmp(tok, cstr(b"basic\0")) == 0 {
            *types |= UBLK_PARAM_TYPE_BASIC;
        } else if strcmp(tok, cstr(b"zoned\0")) == 0 {
            *types |= UBLK_PARAM_TYPE_ZONED;
        } else {
            return -EINVAL;
        }
        tok = strtok_r(null_mut(), cstr(b",\0"), &mut save);
    }
    0
}

unsafe fn ublk_init_params_from_ctx(ctx: *const dev_ctx, params: *mut ublk_params) {
    let p = &(*ctx).params;
    (*params).types = p.types;
    (*params).basic.logical_bs_shift = p.logical_bs_shift;
    (*params).basic.physical_bs_shift = p.physical_bs_shift;
    (*params).basic.io_min_shift = p.io_min_shift;
    (*params).basic.io_opt_shift = p.io_opt_shift;
    (*params).basic.max_sectors = p.max_sectors;
    (*params).basic.chunk_sectors = p.chunk_sectors;
    (*params).basic.dev_sectors = p.dev_sectors;
    (*params).zoned.max_open_zones = p.max_open_zones;
    (*params).zoned.max_active_zones = p.max_active_zones;
    (*params).zoned.max_zone_append_sectors = p.max_zone_append_sectors;
}

unsafe fn ublk_ctrl_update_size(dev: *mut ublk_dev, nr_sects: __u64) -> c_int {
    let mut data: ublk_ctrl_cmd_data = zeroed();
    data.cmd_op = UBLK_U_CMD_UPDATE_SIZE;
    data.flags = CTRL_CMD_HAS_DATA;
    data.data[0] = nr_sects;
    __ublk_ctrl_cmd(dev, &mut data)
}
unsafe fn ublk_ctrl_quiesce_dev(dev: *mut ublk_dev, timeout_ms: c_uint) -> c_int {
    let mut data: ublk_ctrl_cmd_data = zeroed();
    data.cmd_op = UBLK_U_CMD_QUIESCE_DEV;
    data.flags = CTRL_CMD_HAS_DATA;
    data.data[0] = timeout_ms as __u64;
    __ublk_ctrl_cmd(dev, &mut data)
}
unsafe fn ublk_dev_state_desc(dev: *mut ublk_dev) -> *const c_char {
    match (*dev).dev_info.state {
        UBLK_S_DEV_DEAD => cstr(b"DEAD\0"),
        UBLK_S_DEV_LIVE => cstr(b"LIVE\0"),
        UBLK_S_DEV_QUIESCED => cstr(b"QUIESCED\0"),
        _ => cstr(b"UNKNOWN\0"),
    }
}
unsafe fn ublk_print_cpu_set(set: *const cpu_set_t, buf: *mut c_char, len: c_uint) {
    let mut done: c_uint = 0;
    for i in 0..CPU_SETSIZE {
        if CPU_ISSET(i, set) {
            done = done.wrapping_add(snprintf(buf.add(done as usize), (len - done) as size_t, cstr(b"%d \0"), i) as c_uint);
        }
    }
}
unsafe fn ublk_adjust_affinity(set: *mut cpu_set_t) {
    let mut updated = 0;
    for j in 0..CPU_SETSIZE {
        if CPU_ISSET(j, set) {
            if updated == 0 {
                updated = 1;
                continue;
            }
            CPU_CLR(j, set);
        }
    }
}

/* Caller must free the allocated buffer */
unsafe fn ublk_ctrl_get_affinity(ctrl_dev: *mut ublk_dev, ptr_buf: *mut *mut cpu_set_t) -> c_int {
    let mut data: ublk_ctrl_cmd_data = zeroed();
    data.cmd_op = UBLK_U_CMD_GET_QUEUE_AFFINITY;
    data.flags = CTRL_CMD_HAS_DATA | CTRL_CMD_HAS_BUF;
    let buf = malloc(size_of::<cpu_set_t>() * (*ctrl_dev).dev_info.nr_hw_queues as usize) as *mut cpu_set_t;
    if buf.is_null() {
        return -ENOMEM;
    }
    for i in 0..(*ctrl_dev).dev_info.nr_hw_queues as c_int {
        data.data[0] = i as __u64;
        data.len = size_of::<cpu_set_t>() as __u32;
        data.addr = buf.add(i as usize) as usize as __u64;
        let ret = __ublk_ctrl_cmd(ctrl_dev, &mut data);
        if ret < 0 {
            free(buf as *mut c_void);
            return ret;
        }
        ublk_adjust_affinity(buf.add(i as usize));
    }
    *ptr_buf = buf;
    0
}

unsafe fn ublk_ctrl_dump(dev: *mut ublk_dev) {
    let info = &mut (*dev).dev_info as *mut ublksrv_ctrl_dev_info;
    let mut p: ublk_params = zeroed();
    let mut affinity: *mut cpu_set_t = null_mut();
    let mut ret = ublk_ctrl_get_params(dev, &mut p);
    if ret < 0 {
        ublk_err(cstr(b"failed to get params %d %s\n\0"), ret, strerror(-ret));
        return;
    }
    ret = ublk_ctrl_get_affinity(dev, &mut affinity);
    if ret < 0 {
        ublk_err(cstr(b"failed to get affinity %m\n\0"));
        return;
    }
    ublk_log(cstr(b"dev id %d: nr_hw_queues %d queue_depth %d block size %d dev_capacity %lld\n\0"),
        (*info).dev_id, (*info).nr_hw_queues, (*info).queue_depth,
        1 << p.basic.logical_bs_shift, p.basic.dev_sectors);
    ublk_log(cstr(b"\tmax rq size %d daemon pid %d flags 0x%llx state %s\n\0"),
        (*info).max_io_buf_bytes, (*info).ublksrv_pid, (*info).flags, ublk_dev_state_desc(dev));
    if (*info).flags & UBLK_F_IO_DESC_SIZE != 0 {
        ublk_log(cstr(b"\tio_desc_size %u\n\0"), (*info).io_desc_size);
    }
    if !affinity.is_null() {
        let mut buf = [0 as c_char; 512];
        for i in 0..(*info).nr_hw_queues as c_int {
            memset(buf.as_mut_ptr() as *mut c_void, 0, buf.len());
            ublk_print_cpu_set(affinity.add(i as usize), buf.as_mut_ptr(), buf.len() as c_uint);
            printf(cstr(b"\tqueue %u: affinity(%s)\n\0"), i, buf.as_mut_ptr());
        }
        free(affinity as *mut c_void);
    }
    fflush(stdout);
}

unsafe fn ublk_ctrl_deinit(dev: *mut ublk_dev) {
    close((*dev).ctrl_fd);
    free(dev as *mut c_void);
}
unsafe fn ublk_ctrl_init() -> *mut ublk_dev {
    let dev = calloc(1, size_of::<ublk_dev>()) as *mut ublk_dev;
    if dev.is_null() {
        return null_mut();
    }
    (*dev).ctrl_fd = open(CTRL_DEV, O_RDWR);
    if (*dev).ctrl_fd < 0 {
        free(dev as *mut c_void);
        return null_mut();
    }
    (*dev).dev_info.max_io_buf_bytes = UBLK_IO_MAX_BYTES;
    let ret = ublk_setup_ring(&mut (*dev).ring, UBLK_CTRL_RING_DEPTH, UBLK_CTRL_RING_DEPTH, IORING_SETUP_SQE128);
    if ret < 0 {
        ublk_err(cstr(b"queue_init: %s\n\0"), strerror(-ret));
        free(dev as *mut c_void);
        return null_mut();
    }
    (*dev).nr_fds = 1;
    dev
}

unsafe fn __ublk_queue_cmd_buf_sz(q: *const ublk_queue, depth: __u16) -> size_t {
    let size = depth as size_t * (*q).io_desc_size as size_t;
    round_up(size, getpagesize() as size_t)
}
unsafe fn ublk_queue_max_cmd_buf_sz(q: *const ublk_queue) -> size_t {
    __ublk_queue_cmd_buf_sz(q, UBLK_MAX_QUEUE_DEPTH)
}
unsafe fn ublk_queue_cmd_buf_sz(q: *const ublk_queue) -> size_t {
    __ublk_queue_cmd_buf_sz(q, (*q).q_depth as __u16)
}
unsafe fn ublk_queue_deinit(q: *mut ublk_queue) {
    let nr_ios = (*q).q_depth;
    if !(*q).io_cmd_buf.is_null() {
        munmap((*q).io_cmd_buf, ublk_queue_cmd_buf_sz(q));
    }
    for i in 0..nr_ios {
        free((*q).ios[i as usize].buf_addr);
        free((*q).ios[i as usize].integrity_buf);
    }
}
unsafe fn ublk_thread_deinit(t: *mut ublk_thread) {
    io_uring_unregister_buffers(&mut (*t).ring);
    ublk_batch_free_buf(t);
    io_uring_unregister_ring_fd(&mut (*t).ring);
    if (*t).ring.ring_fd > 0 {
        io_uring_unregister_files(&mut (*t).ring);
        close((*t).ring.ring_fd);
        (*t).ring.ring_fd = -1;
    }
}

unsafe fn ublk_queue_init(q: *mut ublk_queue, extra_flags: c_ulonglong, metadata_size: __u8) -> c_int {
    let dev = (*q).dev;
    let depth = (*dev).dev_info.queue_depth as c_int;
    pthread_spin_init(&mut (*q).lock, PTHREAD_PROCESS_PRIVATE);
    (*q).tgt_ops = (*dev).tgt.ops;
    (*q).flags = (*dev).dev_info.flags | extra_flags as __u64;
    (*q).q_depth = depth;
    (*q).metadata_size = metadata_size;
    (*q).io_desc_size = (*dev).dev_info.io_desc_size as __u16;
    (*q).ublk_fd = (*dev).fds[0];
    let cmd_buf_size = ublk_queue_cmd_buf_sz(q);
    let off = UBLKSRV_CMD_BUF_OFFSET + (*q).q_id as c_ulong * ublk_queue_max_cmd_buf_sz(q) as c_ulong;
    (*q).io_cmd_buf = mmap(null_mut(), cmd_buf_size, PROT_READ, MAP_SHARED | MAP_POPULATE, (*dev).fds[0], off as off_t);
    if (*q).io_cmd_buf == MAP_FAILED {
        ublk_err(cstr(b"ublk dev %d queue %d map io_cmd_buf failed %m\n\0"), (*dev).dev_info.dev_id, (*q).q_id);
        return fail_queue_init(q, dev);
    }
    let io_buf_size = (*dev).dev_info.max_io_buf_bytes as size_t;
    let integrity_size = ublk_integrity_len(q, io_buf_size as __u32) as size_t;
    for i in 0..(*q).q_depth {
        (*q).ios[i as usize].buf_addr = null_mut();
        (*q).ios[i as usize].flags = UBLKS_IO_NEED_FETCH_RQ | UBLKS_IO_FREE;
        (*q).ios[i as usize].tag = i as c_uint;
        if integrity_size != 0 {
            (*q).ios[i as usize].integrity_buf = malloc(integrity_size);
            if (*q).ios[i as usize].integrity_buf.is_null() {
                ublk_err(cstr(b"ublk dev %d queue %d io %d malloc(%d) failed: %m\n\0"), (*dev).dev_info.dev_id, (*q).q_id, i, integrity_size);
                return fail_queue_init(q, dev);
            }
        }
        if ublk_queue_no_buf(q) {
            continue;
        }
        let mut p: *mut c_void = null_mut();
        if posix_memalign(&mut p, getpagesize() as size_t, io_buf_size) != 0 {
            ublk_err(cstr(b"ublk dev %d queue %d io %d posix_memalign failed %m\n\0"), (*dev).dev_info.dev_id, (*q).q_id, i);
            return fail_queue_init(q, dev);
        }
        (*q).ios[i as usize].buf_addr = p;
    }
    0
}
unsafe fn fail_queue_init(q: *mut ublk_queue, dev: *mut ublk_dev) -> c_int {
    ublk_queue_deinit(q);
    ublk_err(cstr(b"ublk dev %d queue %d failed\n\0"), (*dev).dev_info.dev_id, (*q).q_id);
    -ENOMEM
}

unsafe fn ublk_thread_init(t: *mut ublk_thread, extra_flags: c_ulonglong) -> c_int {
    let dev = (*t).dev;
    let flags = (*dev).dev_info.flags | extra_flags as __u64;
    let mut cq_depth = (*dev).tgt.cq_depth;
    if ublk_dev_batch_io(dev) {
        cq_depth += (*dev).dev_info.queue_depth as c_int * 2;
    }
    let mut ret = ublk_setup_ring(&mut (*t).ring, (*dev).tgt.sq_depth, cq_depth,
        IORING_SETUP_COOP_TASKRUN | IORING_SETUP_SINGLE_ISSUER | IORING_SETUP_DEFER_TASKRUN);
    if ret < 0 {
        ublk_err(cstr(b"ublk dev %d thread %d setup io_uring failed %d\n\0"), (*dev).dev_info.dev_id, (*t).idx, ret);
        return fail_thread_init(t, dev);
    }
    if (*dev).dev_info.flags & (UBLK_F_SUPPORT_ZERO_COPY | UBLK_F_AUTO_BUF_REG) != 0 {
        let nr_ios = (*dev).dev_info.queue_depth * (*dev).dev_info.nr_hw_queues;
        let mut max_nr_ios_per_thread = nr_ios / (*dev).nthreads;
        max_nr_ios_per_thread += (nr_ios % (*dev).nthreads != 0) as c_uint;
        (*t).auto_buf_stride = max_nr_ios_per_thread;
        (*t).nr_bufs = max_nr_ios_per_thread;
        if extra_flags as __u64 & UBLKS_Q_ROTATE_AUTO_BUF != 0 && (*dev).dev_info.flags & UBLK_F_AUTO_BUF_REG != 0 {
            (*t).nr_bufs *= 2;
        }
    } else {
        (*t).nr_bufs = 0;
        (*t).auto_buf_stride = 0;
    }
    if ublk_dev_batch_io(dev) {
        ublk_batch_prepare(t);
    }
    if (*t).nr_bufs != 0 {
        ret = io_uring_register_buffers_sparse(&mut (*t).ring, (*t).nr_bufs);
        if ret != 0 {
            ublk_err(cstr(b"ublk dev %d thread %d register spare buffers failed %d\n\0"), (*dev).dev_info.dev_id, (*t).idx, ret);
            return fail_thread_init(t, dev);
        }
    }
    if ublk_dev_batch_io(dev) {
        ret = ublk_batch_alloc_buf(t);
        if ret != 0 {
            ublk_err(cstr(b"ublk dev %d thread %d alloc batch buf failed %d\n\0"), (*dev).dev_info.dev_id, (*t).idx, ret);
            return fail_thread_init(t, dev);
        }
    }
    io_uring_register_ring_fd(&mut (*t).ring);
    if flags & UBLKS_Q_NO_UBLK_FIXED_FD != 0 {
        ret = if (*dev).nr_fds > 1 {
            io_uring_register_files(&mut (*t).ring, (*dev).fds.as_ptr().add(1), ((*dev).nr_fds - 1) as c_uint)
        } else { 0 };
    } else {
        ret = io_uring_register_files(&mut (*t).ring, (*dev).fds.as_ptr(), (*dev).nr_fds as c_uint);
    }
    if ret != 0 {
        ublk_err(cstr(b"ublk dev %d thread %d register files failed %d\n\0"), (*dev).dev_info.dev_id, (*t).idx, ret);
        return fail_thread_init(t, dev);
    }
    0
}
unsafe fn fail_thread_init(t: *mut ublk_thread, dev: *mut ublk_dev) -> c_int {
    ublk_thread_deinit(t);
    ublk_err(cstr(b"ublk dev %d thread %d init failed\n\0"), (*dev).dev_info.dev_id, (*t).idx);
    -ENOMEM
}

unsafe fn ublk_dev_prep(ctx: *const dev_ctx, dev: *mut ublk_dev) -> c_int {
    let dev_id = (*dev).dev_info.dev_id;
    let mut wait_usec: c_uint = 0;
    let mut ret = 0;
    let mut fd = -1;
    let mut buf = [0 as c_char; 64];
    snprintf(buf.as_mut_ptr(), 64, cstr(b"%s%d\0"), UBLKC_DEV, dev_id);
    while wait_usec < MAX_WAIT_USEC {
        fd = open(buf.as_ptr(), O_RDWR);
        if fd >= 0 { break; }
        usleep(WAIT_USEC);
        wait_usec += WAIT_USEC;
    }
    if fd < 0 {
        ublk_err(cstr(b"can't open %s %s\n\0"), buf.as_ptr(), strerror(errno));
        return -1;
    }
    (*dev).fds[0] = fd;
    if let Some(init_tgt) = (*(*dev).tgt.ops).init_tgt {
        ret = init_tgt(ctx, dev);
    }
    if ret != 0 {
        close((*dev).fds[0]);
    }
    ret
}
unsafe fn ublk_dev_unprep(dev: *mut ublk_dev) {
    if let Some(deinit_tgt) = (*(*dev).tgt.ops).deinit_tgt {
        deinit_tgt(dev);
    }
    close((*dev).fds[0]);
}
unsafe fn ublk_set_auto_buf_reg(t: *const ublk_thread, q: *const ublk_queue, sqe: *mut io_uring_sqe, tag: c_ushort) {
    let mut buf: ublk_auto_buf_reg = zeroed();
    if let Some(buf_index) = (*(*q).tgt_ops).buf_index {
        buf.index = buf_index(t, q, tag);
    } else {
        buf.index = ublk_io_buf_idx(t, q, tag);
    }
    if ublk_queue_auto_zc_fallback(q) {
        buf.flags = UBLK_AUTO_BUF_REG_FALLBACK;
    }
    (*sqe).addr = ublk_auto_buf_reg_to_sqe_addr(&buf);
}

unsafe fn ublk_user_copy(io: *const ublk_io, match_ublk_op: __u8) {
    let q = ublk_io_to_queue(io);
    let iod = ublk_get_iod(q, (*io).tag);
    let mut off = ublk_user_copy_offset((*q).q_id, (*io).tag);
    let ublk_op = ublksrv_get_op(iod);
    let mut len = (*iod).nr_sectors << 9;
    let mut addr = (*io).buf_addr as *mut u8;
    let mut copied: ssize_t;
    if ublk_op != match_ublk_op { return; }
    while len != 0 {
        let copy_len = if len < UBLK_USER_COPY_LEN { len } else { UBLK_USER_COPY_LEN };
        if ublk_op == UBLK_IO_OP_WRITE {
            copied = pread((*q).ublk_fd, addr as *mut c_void, copy_len as size_t, off as off_t);
        } else if ublk_op == UBLK_IO_OP_READ {
            copied = pwrite((*q).ublk_fd, addr as *const c_void, copy_len as size_t, off as off_t);
        } else {
            core::hint::unreachable_unchecked();
        }
        assert!(copied == copy_len as ssize_t);
        addr = addr.add(copy_len as usize);
        off += copy_len as __u64;
        len -= copy_len;
    }
    if (*iod).op_flags & UBLK_IO_F_INTEGRITY == 0 { return; }
    len = ublk_integrity_len(q, (*iod).nr_sectors << 9);
    off = ublk_user_copy_offset((*q).q_id, (*io).tag) | UBLKSRV_IO_INTEGRITY_FLAG;
    if ublk_op == UBLK_IO_OP_WRITE {
        copied = pread((*q).ublk_fd, (*io).integrity_buf, len as size_t, off as off_t);
    } else if ublk_op == UBLK_IO_OP_READ {
        copied = pwrite((*q).ublk_fd, (*io).integrity_buf, len as size_t, off as off_t);
    } else {
        core::hint::unreachable_unchecked();
    }
    assert!(copied == len as ssize_t);
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn ublk_queue_io_cmd(t: *mut ublk_thread, io: *mut ublk_io) -> c_int {
    let q = ublk_io_to_queue(io);
    let mut sqe: [*mut io_uring_sqe; 1] = [null_mut()];
    let mut cmd_op: c_uint = 0;
    if (*io).flags & UBLKS_IO_FREE == 0 { return 0; }
    if (*io).flags & (UBLKS_IO_NEED_FETCH_RQ | UBLKS_IO_NEED_COMMIT_RQ_COMP | UBLKS_IO_NEED_GET_DATA) == 0 { return 0; }
    if (*io).flags & UBLKS_IO_NEED_GET_DATA != 0 {
        cmd_op = UBLK_U_IO_NEED_GET_DATA;
    } else if (*io).flags & UBLKS_IO_NEED_COMMIT_RQ_COMP != 0 {
        if ublk_queue_use_user_copy(q) {
            ublk_user_copy(io, UBLK_IO_OP_READ);
        }
        cmd_op = UBLK_U_IO_COMMIT_AND_FETCH_REQ;
    } else if (*io).flags & UBLKS_IO_NEED_FETCH_RQ != 0 {
        cmd_op = UBLK_U_IO_FETCH_REQ;
    }
    if io_uring_sq_space_left(&mut (*t).ring) < 1 {
        io_uring_submit(&mut (*t).ring);
    }
    ublk_io_alloc_sqes(t, sqe.as_mut_ptr(), 1);
    if sqe[0].is_null() {
        ublk_err(cstr(b"%s: run out of sqe. thread %u, tag %d\n\0"), cstr(b"ublk_queue_io_cmd\0"), (*t).idx, (*io).tag);
        return -1;
    }
    let cmd = ublk_get_sqe_cmd(sqe[0]) as *mut ublksrv_io_cmd;
    if cmd_op == UBLK_U_IO_COMMIT_AND_FETCH_REQ {
        (*cmd).result = (*io).result;
    }
    ublk_set_sqe_cmd_op(sqe[0], cmd_op);
    (*sqe[0]).fd = ublk_get_registered_fd(q, 0);
    (*sqe[0]).opcode = IORING_OP_URING_CMD;
    (*sqe[0]).flags = if (*q).flags & UBLKS_Q_NO_UBLK_FIXED_FD != 0 { 0 } else { IOSQE_FIXED_FILE };
    (*sqe[0]).rw_flags = 0;
    (*cmd).tag = (*io).tag as __u16;
    (*cmd).q_id = (*q).q_id as __u16;
    (*cmd).addr = if !ublk_queue_no_buf(q) && !ublk_queue_use_user_copy(q) { (*io).buf_addr as usize as __u64 } else { 0 };
    if ublk_queue_use_auto_zc(q) {
        ublk_set_auto_buf_reg(t, q, sqe[0], (*io).tag as c_ushort);
    }
    let user_data = build_user_data((*io).tag, _IOC_NR(cmd_op), 0, (*q).q_id, 0);
    io_uring_sqe_set_data64(sqe[0], user_data);
    (*io).flags = 0;
    (*t).cmd_inflight += 1;
    ublk_dbg(c_uint::from(UBLK_DBG_IO_CMD), cstr(b"%s: (thread %u qid %d tag %u cmd_op %u) iof %x stopping %d\n\0"),
        cstr(b"ublk_queue_io_cmd\0"), (*t).idx, (*q).q_id, (*io).tag, cmd_op, (*io).flags, ((*t).state & UBLKS_T_STOPPING != 0) as c_int);
    1
}

unsafe fn ublk_submit_fetch_commands(t: *mut ublk_thread) {
    let mut j = 0;
    if (*(*t).dev).per_io_tasks {
        let dinfo = &(*(*t).dev).dev_info;
        let nr_ios = (dinfo.nr_hw_queues * dinfo.queue_depth) as c_int;
        let mut i = (*t).idx as c_int;
        while i < nr_ios {
            let q_id = i / dinfo.queue_depth as c_int;
            let tag = i % dinfo.queue_depth as c_int;
            let q = &mut (*(*t).dev).q[q_id as usize] as *mut ublk_queue;
            let io = &mut (*q).ios[tag as usize] as *mut ublk_io;
            (*io).buf_index = j;
            j += 1;
            if let Some(pre_fetch_io) = (*(*q).tgt_ops).pre_fetch_io {
                pre_fetch_io(t, q, tag, false);
            }
            ublk_queue_io_cmd(t, io);
            i += (*(*t).dev).nthreads as c_int;
        }
    } else {
        let q = &mut (*(*t).dev).q[(*t).idx as usize] as *mut ublk_queue;
        for i in 0..(*q).q_depth {
            let io = &mut (*q).ios[i as usize] as *mut ublk_io;
            (*io).buf_index = i;
            if let Some(pre_fetch_io) = (*(*q).tgt_ops).pre_fetch_io {
                pre_fetch_io(t, q, i, false);
            }
            ublk_queue_io_cmd(t, io);
        }
    }
}
unsafe fn ublk_thread_is_idle(t: *mut ublk_thread) -> c_int {
    (io_uring_sq_ready(&mut (*t).ring) == 0 && (*t).io_inflight == 0) as c_int
}
unsafe fn ublk_thread_is_done(t: *mut ublk_thread) -> c_int {
    (((*t).state & UBLKS_T_STOPPING != 0) && ublk_thread_is_idle(t) != 0 && (*t).cmd_inflight == 0) as c_int
}
unsafe fn ublksrv_handle_tgt_cqe(t: *mut ublk_thread, q: *mut ublk_queue, cqe: *mut io_uring_cqe) {
    if (*cqe).res < 0 && (*cqe).res != -EAGAIN {
        ublk_err(cstr(b"%s: failed tgt io: res %d qid %u tag %u, cmd_op %u\n\0"),
            cstr(b"ublksrv_handle_tgt_cqe\0"), (*cqe).res, (*q).q_id, user_data_to_tag((*cqe).user_data), user_data_to_op((*cqe).user_data));
    }
    if let Some(tgt_io_done) = (*(*q).tgt_ops).tgt_io_done {
        tgt_io_done(t, q, cqe);
    }
}
unsafe fn ublk_handle_uring_cmd(t: *mut ublk_thread, q: *mut ublk_queue, cqe: *const io_uring_cqe) {
    let fetch = (*cqe).res != UBLK_IO_RES_ABORT && ((*t).state & UBLKS_T_STOPPING == 0);
    let tag = user_data_to_tag((*cqe).user_data);
    let io = &mut (*q).ios[tag as usize] as *mut ublk_io;
    (*t).cmd_inflight -= 1;
    if !fetch {
        (*t).state |= UBLKS_T_STOPPING;
        (*io).flags &= !UBLKS_IO_NEED_FETCH_RQ;
    }
    if (*cqe).res == UBLK_IO_RES_OK {
        ublk_assert(tag < (*q).q_depth as c_uint);
        if ublk_queue_use_user_copy(q) {
            ublk_user_copy(io, UBLK_IO_OP_WRITE);
        }
        if let Some(queue_io) = (*(*q).tgt_ops).queue_io {
            queue_io(t, q, tag);
        }
    } else if (*cqe).res == UBLK_IO_RES_NEED_GET_DATA {
        (*io).flags |= UBLKS_IO_NEED_GET_DATA | UBLKS_IO_FREE;
        ublk_queue_io_cmd(t, io);
    } else {
        (*io).flags = UBLKS_IO_FREE;
    }
}
unsafe fn ublk_handle_cqe(t: *mut ublk_thread, cqe: *mut io_uring_cqe, _data: *mut c_void) {
    let dev = (*t).dev;
    let q_id = user_data_to_q_id((*cqe).user_data);
    let cmd_op = user_data_to_op((*cqe).user_data);
    if (*cqe).res < 0 && (*cqe).res != -ENODEV && (*cqe).res != -ENOBUFS {
        ublk_err(cstr(b"%s: res %d userdata %llx thread state %x\n\0"), cstr(b"ublk_handle_cqe\0"), (*cqe).res, (*cqe).user_data, (*t).state);
    }
    ublk_dbg(UBLK_DBG_IO_CMD, cstr(b"%s: res %d (thread %d qid %d tag %u cmd_op %x data %lx target %d/%d) stopping %d\n\0"),
        cstr(b"ublk_handle_cqe\0"), (*cqe).res, (*t).idx, q_id, user_data_to_tag((*cqe).user_data), cmd_op,
        (*cqe).user_data, is_target_io((*cqe).user_data) as c_int, user_data_to_tgt_data((*cqe).user_data), (*t).state & UBLKS_T_STOPPING);
    if is_target_io((*cqe).user_data) {
        ublksrv_handle_tgt_cqe(t, &mut (*dev).q[q_id as usize], cqe);
        return;
    }
    if ublk_thread_batch_io(t) {
        ublk_batch_compl_cmd(t, cqe);
    } else {
        ublk_handle_uring_cmd(t, &mut (*dev).q[q_id as usize], cqe);
    }
}
unsafe fn ublk_reap_events_uring(_t: *mut ublk_thread) -> c_int {
    /* io_uring_for_each_cqe is a C macro; preserve intent as an external iteration TODO. */
    0
}
unsafe fn ublk_process_io(t: *mut ublk_thread) -> c_int {
    ublk_dbg(UBLK_DBG_THREAD, cstr(b"dev%d-t%u: to_submit %d inflight cmd %u stopping %d\n\0"),
        (*(*t).dev).dev_info.dev_id, (*t).idx, io_uring_sq_ready(&mut (*t).ring), (*t).cmd_inflight, (*t).state & UBLKS_T_STOPPING);
    if ublk_thread_is_done(t) != 0 { return -ENODEV; }
    let ret = io_uring_submit_and_wait(&mut (*t).ring, 1);
    let reapped;
    if ublk_thread_batch_io(t) {
        ublk_batch_prep_commit(t);
        reapped = ublk_reap_events_uring(t);
        ublk_batch_commit_io_cmds(t);
    } else {
        reapped = ublk_reap_events_uring(t);
    }
    ublk_dbg(UBLK_DBG_THREAD, cstr(b"submit result %d, reapped %d stop %d idle %d\n\0"), ret, reapped, (*t).state & UBLKS_T_STOPPING, (*t).state & UBLKS_T_IDLE);
    reapped
}

#[repr(C)]
struct ublk_thread_info {
    dev: *mut ublk_dev,
    thread: pthread_t,
    idx: c_uint,
    ready: *mut sem_t,
    affinity: *mut cpu_set_t,
    extra_flags: c_ulonglong,
    q_thread_map: *mut [__u8; UBLK_MAX_QUEUES],
}
unsafe fn ublk_thread_set_sched_affinity(info: *const ublk_thread_info) {
    if pthread_setaffinity_np(pthread_self(), size_of::<cpu_set_t>(), (*info).affinity) < 0 {
        ublk_err(cstr(b"ublk dev %u thread %u set affinity failed\0"), (*(*info).dev).dev_info.dev_id, (*info).idx);
    }
}
unsafe fn ublk_batch_setup_queues(t: *mut ublk_thread) {
    for i in 0..(*(*t).dev).dev_info.nr_hw_queues as c_int {
        let q = &mut (*(*t).dev).q[i as usize] as *mut ublk_queue;
        if (*t).q_map[i as usize] == 0 { continue; }
        if let Some(pre_fetch_io) = (*(*q).tgt_ops).pre_fetch_io {
            pre_fetch_io(t, q, 0, true);
        }
        let ret = ublk_batch_queue_prep_io_cmds(t, q);
        ublk_assert(ret >= 0);
    }
}
#[inline(never)]
unsafe fn __ublk_io_handler_fn(info: *mut ublk_thread_info) -> c_int {
    let mut t: ublk_thread = zeroed();
    t.dev = (*info).dev;
    t.idx = (*info).idx;
    let dev_id = (*(*info).dev).dev_info.dev_id;
    if !(*info).q_thread_map.is_null() {
        memcpy(t.q_map.as_mut_ptr() as *mut c_void, (*info).q_thread_map.add((*info).idx as usize)).cast(), size_of::<[__u8; UBLK_MAX_QUEUES]>());
    }
    let ret = ublk_thread_init(&mut t, (*info).extra_flags);
    if ret != 0 {
        ublk_err(cstr(b"ublk dev %d thread %u init failed\n\0"), dev_id, t.idx);
        return ret;
    }
    sem_post((*info).ready);
    ublk_dbg(UBLK_DBG_THREAD, cstr(b"tid %d: ublk dev %d thread %u started\n\0"), gettid(), dev_id, t.idx);
    if !ublk_thread_batch_io(&mut t) {
        ublk_submit_fetch_commands(&mut t);
    } else {
        ublk_batch_setup_queues(&mut t);
        ublk_batch_start_fetch(&mut t);
    }
    loop {
        if ublk_process_io(&mut t) < 0 { break; }
    }
    ublk_dbg(UBLK_DBG_THREAD, cstr(b"tid %d: ublk dev %d thread %d exiting\n\0"), gettid(), dev_id, t.idx);
    ublk_thread_deinit(&mut t);
    0
}
unsafe extern "C" fn ublk_io_handler_fn(data: *mut c_void) -> *mut c_void {
    let info = data as *mut ublk_thread_info;
    if !(*info).affinity.is_null() {
        ublk_thread_set_sched_affinity(info);
    }
    __ublk_io_handler_fn(info);
    null_mut()
}

unsafe fn ublk_set_parameters(dev: *mut ublk_dev) {
    let ret = ublk_ctrl_set_params(dev, &mut (*dev).tgt.params);
    if ret != 0 {
        ublk_err(cstr(b"dev %d set basic parameter failed %d\n\0"), (*dev).dev_info.dev_id, ret);
    }
}
unsafe fn ublk_send_dev_event(ctx: *const dev_ctx, dev: *mut ublk_dev, dev_id: c_int) -> c_int {
    let evtfd = (*ctx)._evtfd;
    if evtfd < 0 { return -EBADF; }
    let id: u64 = if dev_id >= 0 { dev_id as u64 + 1 } else { ERROR_EVTFD_DEVID };
    if !dev.is_null() && !(*ctx).shadow_dev.is_null() {
        memcpy(&mut (*(*ctx).shadow_dev).q as *mut _ as *mut c_void, &(*dev).q as *const _ as *const c_void, size_of_val(&(*dev).q));
    }
    if write(evtfd, &id as *const _ as *const c_void, size_of::<u64>()) != size_of::<u64>() as ssize_t {
        return -EINVAL;
    }
    close(evtfd);
    shmdt((*ctx).shadow_dev as *const c_void);
    0
}
unsafe fn size_of_val<T>(_: &T) -> usize { size_of::<T>() }

unsafe fn ublk_shmem_sock_path(dev_id: c_int, buf: *mut c_char, len: size_t) {
    snprintf(buf, len, cstr(b"%s/ublkb%d.sock\0"), UBLK_SHMEM_SOCK_DIR.as_ptr() as *const c_char, dev_id);
}
unsafe fn ublk_shmem_sock_create(dev_id: c_int) -> c_int {
    let mut addr: sockaddr_un = zeroed();
    addr.sun_family = AF_UNIX as c_ushort;
    let mut path = [0 as c_char; 108];
    mkdir(UBLK_SHMEM_SOCK_DIR.as_ptr() as *const c_char, 0o755);
    ublk_shmem_sock_path(dev_id, path.as_mut_ptr(), path.len());
    unlink(path.as_ptr());
    let fd = socket(AF_UNIX, SOCK_STREAM | SOCK_NONBLOCK, 0);
    if fd < 0 { return -1; }
    snprintf(addr.sun_path.as_mut_ptr(), addr.sun_path.len(), cstr(b"%s\0"), path.as_ptr());
    if bind(fd, &addr as *const _ as *const sockaddr, size_of::<sockaddr_un>() as c_uint) < 0 {
        close(fd);
        return -1;
    }
    listen(fd, 4);
    ublk_dbg(UBLK_DBG_DEV, cstr(b"shmem socket created: %s\n\0"), path.as_ptr());
    fd
}
unsafe fn ublk_shmem_sock_destroy(dev_id: c_int, sock_fd: c_int) {
    let mut path = [0 as c_char; 108];
    if sock_fd >= 0 { close(sock_fd); }
    ublk_shmem_sock_path(dev_id, path.as_mut_ptr(), path.len());
    unlink(path.as_ptr());
}
unsafe fn CMSG_SPACE(len: size_t) -> size_t { size_of::<cmsghdr>() + len }
unsafe fn CMSG_FIRSTHDR(msg: *mut msghdr) -> *mut cmsghdr { (*msg).msg_control as *mut cmsghdr }
unsafe fn CMSG_DATA(cmsg: *mut cmsghdr) -> *mut c_void { (cmsg as *mut u8).add(size_of::<cmsghdr>()) as *mut c_void }

unsafe fn ublk_shmem_recv_fd(client_fd: c_int) -> c_int {
    let mut buf = [0 as c_char; 1];
    let mut iov = iovec { iov_base: buf.as_mut_ptr() as *mut c_void, iov_len: size_of::<[c_char; 1]>() };
    let mut cmsg_buf = [0u8; 64];
    let mut msg: msghdr = zeroed();
    msg.msg_iov = &mut iov;
    msg.msg_iovlen = 1;
    msg.msg_control = cmsg_buf.as_mut_ptr() as *mut c_void;
    msg.msg_controllen = CMSG_SPACE(size_of::<c_int>());
    if recvmsg(client_fd, &mut msg, 0) <= 0 { return -1; }
    let cmsg = CMSG_FIRSTHDR(&mut msg);
    if cmsg.is_null() || (*cmsg).cmsg_level != SOL_SOCKET || (*cmsg).cmsg_type != SCM_RIGHTS {
        return -1;
    }
    *(CMSG_DATA(cmsg) as *mut c_int)
}
unsafe fn ublk_shmem_register(shmem_fd: c_int) -> c_int {
    if shmem_count >= UBLK_BUF_MAX as c_int { return -1; }
    let size = lseek(shmem_fd, 0, SEEK_END);
    if size <= 0 { return -1; }
    let base = mmap(null_mut(), size as size_t, PROT_READ | PROT_WRITE, MAP_SHARED, shmem_fd, 0);
    if base == MAP_FAILED { return -1; }
    let idx = shmem_count;
    shmem_count += 1;
    shmem_table[idx as usize].fd = shmem_fd;
    shmem_table[idx as usize].mmap_base = base;
    shmem_table[idx as usize].size = size as size_t;
    ublk_dbg(UBLK_DBG_DEV, cstr(b"shmem registered: index=%d fd=%d size=%zu\n\0"), idx, shmem_fd, size as size_t);
    idx
}
unsafe fn ublk_shmem_unregister_all() {
    for i in 0..shmem_count {
        if !shmem_table[i as usize].mmap_base.is_null() {
            munmap(shmem_table[i as usize].mmap_base, shmem_table[i as usize].size);
            close(shmem_table[i as usize].fd);
            shmem_table[i as usize].mmap_base = null_mut();
        }
    }
    shmem_count = 0;
}
unsafe fn ublk_ctrl_reg_buf(dev: *mut ublk_dev, addr: *mut c_void, size: size_t, flags: __u32) -> c_int {
    let mut buf_reg = ublk_shmem_buf_reg { addr: addr as c_ulong, len: size, flags };
    let mut data: ublk_ctrl_cmd_data = zeroed();
    data.cmd_op = UBLK_U_CMD_REG_BUF;
    data.flags = CTRL_CMD_HAS_BUF;
    data.addr = &mut buf_reg as *mut _ as usize as __u64;
    data.len = size_of::<ublk_shmem_buf_reg>() as __u32;
    __ublk_ctrl_cmd(dev, &mut data)
}
unsafe fn ublk_shmem_handle_client(sock_fd: c_int, dev: *mut ublk_dev) {
    let client_fd = accept(sock_fd, null_mut(), null_mut());
    if client_fd < 0 { return; }
    let memfd = ublk_shmem_recv_fd(client_fd);
    let reply: i32;
    if memfd < 0 {
        reply = -1;
    } else {
        let size = lseek(memfd, 0, SEEK_END);
        if size <= 0 {
            close(memfd);
            reply = -1;
        } else {
            let base = mmap(null_mut(), size as size_t, PROT_READ | PROT_WRITE, MAP_SHARED | MAP_POPULATE, memfd, 0);
            if base == MAP_FAILED {
                close(memfd);
                reply = -1;
            } else {
                let ret = ublk_ctrl_reg_buf(dev, base, size as size_t, 0);
                if ret < 0 {
                    ublk_dbg(UBLK_DBG_DEV, cstr(b"shmem_zc: kernel reg failed %d\n\0"), ret);
                    munmap(base, size as size_t);
                    close(memfd);
                    reply = ret;
                } else {
                    let idx = ublk_shmem_register(memfd);
                    if idx >= 0 {
                        shmem_table[idx as usize].mmap_base = base;
                        shmem_table[idx as usize].size = size as size_t;
                    }
                    reply = idx;
                }
            }
        }
    }
    send(client_fd, &reply as *const _ as *const c_void, size_of::<i32>(), 0);
    close(client_fd);
}

#[repr(C)]
struct shmem_listener_info {
    dev_id: c_int,
    stop_efd: c_int,
    sock_fd: c_int,
    dev: *mut ublk_dev,
}
unsafe extern "C" fn ublk_shmem_listener_fn(data: *mut c_void) -> *mut c_void {
    let info = data as *mut shmem_listener_info;
    let mut pfds = [pollfd { fd: 0, events: 0, revents: 0 }; 2];
    (*info).sock_fd = ublk_shmem_sock_create((*info).dev_id);
    if (*info).sock_fd < 0 { return null_mut(); }
    pfds[0].fd = (*info).sock_fd;
    pfds[0].events = POLLIN;
    pfds[1].fd = (*info).stop_efd;
    pfds[1].events = POLLIN;
    loop {
        let ret = poll(pfds.as_mut_ptr(), 2, -1);
        if ret < 0 { break; }
        if pfds[1].revents & POLLIN != 0 { break; }
        if pfds[0].revents & POLLIN != 0 {
            ublk_shmem_handle_client((*info).sock_fd, (*info).dev);
        }
    }
    null_mut()
}

unsafe fn ublk_shmem_htlb_setup(ctx: *const dev_ctx, dev: *mut ublk_dev) -> c_int {
    let fd = open((*ctx).htlb_path, O_RDWR);
    if fd < 0 {
        ublk_err(cstr(b"htlb: can't open %s\n\0"), (*ctx).htlb_path);
        return -errno;
    }
    let mut st: stat = zeroed();
    if fstat(fd, &mut st) < 0 || st.st_size <= 0 {
        ublk_err(cstr(b"htlb: invalid file size\n\0"));
        close(fd);
        return -EINVAL;
    }
    let prot = if (*ctx).rdonly_shmem_buf { PROT_READ } else { PROT_READ | PROT_WRITE };
    let base = mmap(null_mut(), st.st_size as size_t, prot, MAP_SHARED | MAP_POPULATE, fd, 0);
    if base == MAP_FAILED {
        ublk_err(cstr(b"htlb: mmap failed\n\0"));
        close(fd);
        return -ENOMEM;
    }
    let ret = ublk_ctrl_reg_buf(dev, base, st.st_size as size_t, if (*ctx).rdonly_shmem_buf { UBLK_SHMEM_BUF_READ_ONLY } else { 0 });
    if ret < 0 {
        ublk_err(cstr(b"htlb: reg_buf failed: %d\n\0"), ret);
        munmap(base, st.st_size as size_t);
        close(fd);
        return ret;
    }
    if shmem_count >= UBLK_BUF_MAX as c_int {
        munmap(base, st.st_size as size_t);
        close(fd);
        return -ENOMEM;
    }
    let idx = shmem_count;
    shmem_count += 1;
    shmem_table[idx as usize].fd = fd;
    shmem_table[idx as usize].mmap_base = base;
    shmem_table[idx as usize].size = st.st_size as size_t;
    ublk_dbg(UBLK_DBG_DEV, cstr(b"htlb registered: index=%d size=%zu\n\0"), idx, st.st_size as size_t);
    0
}

unsafe fn ublk_start_daemon(ctx: *const dev_ctx, dev: *mut ublk_dev) -> c_int {
    let dinfo = &(*dev).dev_info as *const ublksrv_ctrl_dev_info;
    let mut linfo: shmem_listener_info = zeroed();
    let mut extra_flags: c_ulonglong = 0;
    let mut affinity_buf: *mut cpu_set_t = null_mut();
    let mut q_thread_map: *mut [__u8; UBLK_MAX_QUEUES] = null_mut();
    let stop_val: u64 = 1;
    let mut listener: pthread_t = 0;
    let mut thread_ret: *mut c_void = null_mut();
    let mut ready: sem_t = zeroed();
    ublk_dbg(UBLK_DBG_DEV, cstr(b"%s enter\n\0"), cstr(b"ublk_start_daemon\0"));
    let tinfo = calloc(size_of::<ublk_thread_info>(), (*dev).nthreads as size_t) as *mut ublk_thread_info;
    if tinfo.is_null() { return -ENOMEM; }
    sem_init(&mut ready, 0, 0);
    let mut ret = ublk_dev_prep(ctx, dev);
    if ret != 0 { return ret; }
    ret = ublk_ctrl_get_affinity(dev, &mut affinity_buf);
    if ret != 0 { return ret; }
    if ublk_dev_batch_io(dev) {
        q_thread_map = calloc((*dev).nthreads as size_t, size_of::<[__u8; UBLK_MAX_QUEUES]>()) as *mut [__u8; UBLK_MAX_QUEUES];
        if q_thread_map.is_null() { ret = -ENOMEM; goto_fail_start(ctx, dev, tinfo, q_thread_map, affinity_buf, ret); return ret; }
        ublk_batch_setup_map(q_thread_map, (*dev).nthreads, (*dinfo).nr_hw_queues);
    }
    if (*ctx).auto_zc_fallback { extra_flags = UBLKS_Q_AUTO_BUF_REG_FALLBACK as c_ulonglong; }
    if (*ctx).no_ublk_fixed_fd { extra_flags |= UBLKS_Q_NO_UBLK_FIXED_FD as c_ulonglong; }
    if (*ctx).rotate_auto_buf { extra_flags |= UBLKS_Q_ROTATE_AUTO_BUF as c_ulonglong; }
    for i in 0..(*dinfo).nr_hw_queues as c_int {
        (*dev).q[i as usize].dev = dev;
        (*dev).q[i as usize].q_id = i;
        ret = ublk_queue_init(&mut (*dev).q[i as usize], extra_flags, (*ctx).metadata_size);
        if ret != 0 {
            ublk_err(cstr(b"ublk dev %d queue %d init queue failed\n\0"), (*dinfo).dev_id, i);
            goto_fail_start(ctx, dev, tinfo, q_thread_map, affinity_buf, ret);
            return ret;
        }
    }
    for i in 0..(*dev).nthreads as c_int {
        (*tinfo.add(i as usize)).dev = dev;
        (*tinfo.add(i as usize)).idx = i as c_uint;
        (*tinfo.add(i as usize)).ready = &mut ready;
        (*tinfo.add(i as usize)).extra_flags = extra_flags;
        (*tinfo.add(i as usize)).q_thread_map = q_thread_map;
        if (*dev).nthreads == (*dinfo).nr_hw_queues {
            (*tinfo.add(i as usize)).affinity = affinity_buf.add(i as usize);
        }
        pthread_create(&mut (*tinfo.add(i as usize)).thread, null(), ublk_io_handler_fn, tinfo.add(i as usize) as *mut c_void);
    }
    for _ in 0..(*dev).nthreads {
        sem_wait(&mut ready);
    }
    free(affinity_buf as *mut c_void);
    free(q_thread_map as *mut c_void);
    if (*ctx).recovery {
        ret = ublk_ctrl_end_user_recovery(dev, getpid());
    } else {
        ublk_set_parameters(dev);
        ret = ublk_ctrl_start_dev(dev, getpid());
    }
    if ret < 0 {
        ublk_err(cstr(b"%s: ublk_ctrl_start_dev failed: %d\n\0"), cstr(b"ublk_start_daemon\0"), ret);
        ublk_ctrl_stop_dev(dev);
    } else if !(*ctx).htlb_path.is_null() {
        ret = ublk_shmem_htlb_setup(ctx, dev);
        if ret < 0 {
            ublk_err(cstr(b"htlb setup failed: %d\n\0"), ret);
            ublk_ctrl_stop_dev(dev);
        }
    }
    if ret >= 0 {
        ublk_ctrl_get_info(dev);
        if (*ctx).fg { ublk_ctrl_dump(dev); } else { ublk_send_dev_event(ctx, dev, (*dev).dev_info.dev_id); }
    }
    linfo.dev_id = (*dinfo).dev_id;
    linfo.dev = dev;
    linfo.stop_efd = eventfd(0, 0);
    if linfo.stop_efd >= 0 {
        pthread_create(&mut listener, null(), ublk_shmem_listener_fn, &mut linfo as *mut _ as *mut c_void);
    }
    for i in 0..(*dev).nthreads as c_int {
        pthread_join((*tinfo.add(i as usize)).thread, &mut thread_ret);
    }
    if linfo.stop_efd >= 0 {
        write(linfo.stop_efd, &stop_val as *const _ as *const c_void, size_of::<u64>());
        pthread_join(listener, null_mut());
        close(linfo.stop_efd);
        ublk_shmem_sock_destroy((*dinfo).dev_id, linfo.sock_fd);
    }
    ublk_shmem_unregister_all();
    free(tinfo as *mut c_void);
    for i in 0..(*dinfo).nr_hw_queues as c_int {
        ublk_queue_deinit(&mut (*dev).q[i as usize]);
    }
    ublk_dev_unprep(dev);
    ublk_dbg(UBLK_DBG_DEV, cstr(b"%s exit\n\0"), cstr(b"ublk_start_daemon\0"));
    ret
}
unsafe fn goto_fail_start(_ctx: *const dev_ctx, dev: *mut ublk_dev, tinfo: *mut ublk_thread_info, q_thread_map: *mut [__u8; UBLK_MAX_QUEUES], affinity_buf: *mut cpu_set_t, ret: c_int) {
    free(affinity_buf as *mut c_void);
    free(q_thread_map as *mut c_void);
    free(tinfo as *mut c_void);
    for i in 0..(*dev).dev_info.nr_hw_queues as c_int {
        ublk_queue_deinit(&mut (*dev).q[i as usize]);
    }
    ublk_dev_unprep(dev);
    let _ = ret;
}

unsafe fn wait_ublk_dev(path: *const c_char, evt_mask: c_int, timeout: c_uint) -> c_int {
    let ev_size = size_of::<inotify_event>();
    let ev_buf_len = 128 * (ev_size + 16);
    let mut pfd: pollfd = zeroed();
    let mut ret = -EINVAL;
    let dev_name = basename(path);
    let fd = inotify_init();
    if fd < 0 {
        ublk_dbg(UBLK_DBG_DEV, cstr(b"%s: inotify init failed\n\0"), cstr(b"wait_ublk_dev\0"));
        return fd;
    }
    let wd = inotify_add_watch(fd, cstr(b"/dev\0"), evt_mask as __u32);
    if wd == -1 {
        ublk_dbg(UBLK_DBG_DEV, cstr(b"%s: add watch for /dev failed\n\0"), cstr(b"wait_ublk_dev\0"));
        close(fd);
        return ret;
    }
    pfd.fd = fd;
    pfd.events = POLL_IN;
    loop {
        let mut i = 0usize;
        let mut buffer = vec![0u8; ev_buf_len];
        ret = poll(&mut pfd, 1, (1000 * timeout) as c_int);
        if ret == -1 {
            ublk_err(cstr(b"%s: poll inotify failed: %d\n\0"), cstr(b"wait_ublk_dev\0"), ret);
            break;
        } else if ret == 0 {
            ublk_err(cstr(b"%s: poll inotify timeout\n\0"), cstr(b"wait_ublk_dev\0"));
            ret = -ETIMEDOUT;
            break;
        }
        ret = read(fd, buffer.as_mut_ptr() as *mut c_void, ev_buf_len) as c_int;
        if ret < 0 {
            ublk_err(cstr(b"%s: read inotify fd failed\n\0"), cstr(b"wait_ublk_dev\0"));
            break;
        }
        while i < ret as usize {
            let event = buffer.as_ptr().add(i) as *const inotify_event;
            ublk_dbg(UBLK_DBG_DEV, cstr(b"%s: inotify event %x %s\n\0"), cstr(b"wait_ublk_dev\0"), (*event).mask, (*event).name.as_ptr());
            if (*event).mask & evt_mask as __u32 != 0 && strcmp((*event).name.as_ptr(), dev_name) == 0 {
                ret = 0;
                inotify_rm_watch(fd, wd);
                close(fd);
                return ret;
            }
            i += ev_size + (*event).len as usize;
        }
    }
    inotify_rm_watch(fd, wd);
    close(fd);
    ret
}
unsafe fn ublk_stop_io_daemon(dev: *const ublk_dev) -> c_int {
    let daemon_pid = (*dev).dev_info.ublksrv_pid;
    let dev_id = (*dev).dev_info.dev_id;
    let mut ublkc = [0 as c_char; 64];
    let mut ret = 0;
    if daemon_pid < 0 { return 0; }
    if kill(daemon_pid, 0) >= 0 {
        snprintf(ublkc.as_mut_ptr(), ublkc.len(), cstr(b"/dev/%s%d\0"), cstr(b"ublkc\0"), dev_id);
        if access(ublkc.as_ptr(), F_OK) == 0 {
            ret = wait_ublk_dev(ublkc.as_ptr(), IN_CLOSE, 10);
            if ret == -ETIMEDOUT {
                ret = (kill(daemon_pid, 0) < 0) as c_int;
            }
        }
    }
    waitpid(daemon_pid, null_mut(), 0);
    ublk_dbg(UBLK_DBG_DEV, cstr(b"%s: pid %d dev_id %d ret %d\n\0"), cstr(b"ublk_stop_io_daemon\0"), daemon_pid, dev_id, ret);
    ret
}

unsafe fn __cmd_dev_add(ctx: *const dev_ctx) -> c_int {
    let mut nthreads = (*ctx).nthreads;
    let nr_queues = (*ctx).nr_hw_queues;
    let tgt_type = (*ctx).tgt_type.as_ptr();
    let depth = (*ctx).queue_depth;
    let mut features: __u64 = 0;
    let mut dev: *mut ublk_dev = null_mut();
    let dev_id = (*ctx).dev_id;
    let mut ret: c_int;
    let ops = ublk_find_tgt(tgt_type);
    if ops.is_null() {
        ublk_err(cstr(b"%s: no such tgt type, type %s\n\0"), cstr(b"__cmd_dev_add\0"), tgt_type);
        ret = -ENODEV;
        goto_cmd_add_fail(ctx, dev, ret);
        return ret;
    }
    if nr_queues as usize > UBLK_MAX_QUEUES || depth as usize > UBLK_QUEUE_DEPTH {
        ublk_err(cstr(b"%s: invalid nr_queues or depth queues %u depth %u\n\0"), cstr(b"__cmd_dev_add\0"), nr_queues, depth);
        return -EINVAL;
    }
    if nthreads == 0 { nthreads = nr_queues; }
    if nthreads > UBLK_MAX_THREADS {
        ublk_err(cstr(b"%s: %u is too many threads (max %u)\n\0"), cstr(b"__cmd_dev_add\0"), nthreads, UBLK_MAX_THREADS);
        return -EINVAL;
    }
    if nthreads != nr_queues && !(*ctx).per_io_tasks && ((*ctx).flags & UBLK_F_BATCH_IO == 0) {
        ublk_err(cstr(b"%s: threads %u must be same as queues %u if not using per_io_tasks\n\0"), cstr(b"__cmd_dev_add\0"), nthreads, nr_queues);
        return -EINVAL;
    }
    dev = ublk_ctrl_init();
    if dev.is_null() {
        ublk_err(cstr(b"%s: can't alloc dev id %d, type %s\n\0"), cstr(b"__cmd_dev_add\0"), dev_id, tgt_type);
        return -ENOMEM;
    }
    ret = ublk_ctrl_get_features(dev, &mut features);
    if ret < 0 { ret = -EINVAL; goto_cmd_add_fail(ctx, dev, ret); return ret; }
    if features & UBLK_F_CMD_IOCTL_ENCODE == 0 { ret = -ENOTSUP; goto_cmd_add_fail(ctx, dev, ret); return ret; }
    (*dev).dev_info.dev_id = (*ctx).dev_id;
    (*dev).dev_info.nr_hw_queues = nr_queues;
    (*dev).dev_info.queue_depth = depth;
    (*dev).dev_info.io_desc_size = (*ctx).io_desc_size as c_uint;
    (*dev).dev_info.flags = (*ctx).flags;
    if features & UBLK_F_QUIESCE != 0 && (*dev).dev_info.flags & UBLK_F_USER_RECOVERY != 0 {
        (*dev).dev_info.flags |= UBLK_F_QUIESCE;
    }
    (*dev).nthreads = nthreads;
    (*dev).per_io_tasks = (*ctx).per_io_tasks;
    (*dev).tgt.ops = ops;
    (*dev).tgt.sq_depth = depth as c_int;
    (*dev).tgt.cq_depth = depth as c_int;
    for i in 0..MAX_BACK_FILES {
        if !(*ctx).files[i].is_null() {
            strcpy((*dev).tgt.backing_file[i].as_mut_ptr(), (*ctx).files[i]);
            (*dev).tgt.nr_backing_files += 1;
        }
    }
    ret = if (*ctx).recovery { ublk_ctrl_start_user_recovery(dev) } else { ublk_ctrl_add_dev(dev) };
    if ret < 0 {
        ublk_err(cstr(b"%s: can't add dev id %d, type %s ret %d\n\0"), cstr(b"__cmd_dev_add\0"), dev_id, tgt_type, ret);
        goto_cmd_add_fail(ctx, dev, ret);
        return ret;
    }
    if !(*ctx).per_io_tasks && (*dev).nthreads > (*dev).dev_info.nr_hw_queues {
        (*dev).nthreads = (*dev).dev_info.nr_hw_queues;
    }
    ret = ublk_start_daemon(ctx, dev);
    ublk_dbg(UBLK_DBG_DEV, cstr(b"%s: daemon exit %d\n\0"), cstr(b"__cmd_dev_add\0"), ret);
    if ret < 0 { ublk_ctrl_del_dev(dev); }
    goto_cmd_add_fail(ctx, dev, ret);
    ret
}
unsafe fn goto_cmd_add_fail(ctx: *const dev_ctx, dev: *mut ublk_dev, ret: c_int) {
    if ret < 0 { ublk_send_dev_event(ctx, dev, -1); }
    if !dev.is_null() { ublk_ctrl_deinit(dev); }
}

unsafe fn __cmd_dev_list(ctx: *mut dev_ctx) -> c_int;

unsafe fn cmd_dev_set_params(ctx: *mut dev_ctx) -> c_int {
    let dev = ublk_ctrl_init();
    if dev.is_null() { return -ENODEV; }
    let mut features: __u64 = 0;
    let mut ret = ublk_ctrl_get_features(dev, &mut features);
    if ret >= 0 {
        if features & UBLK_F_CMD_IOCTL_ENCODE == 0 {
            ret = -ENOTSUP;
        } else {
            (*dev).dev_info.dev_id = (*ctx).dev_id;
            (*dev).dev_info.nr_hw_queues = (*ctx).nr_hw_queues;
            (*dev).dev_info.queue_depth = (*ctx).queue_depth;
            (*dev).dev_info.io_desc_size = (*ctx).io_desc_size as c_uint;
            (*dev).dev_info.flags = (*ctx).flags;
            ret = ublk_ctrl_add_dev(dev);
            if ret >= 0 {
                let mut params: ublk_params = zeroed();
                ublk_init_params_from_ctx(ctx, &mut params);
                ret = ublk_ctrl_set_params(dev, &mut params);
                printf(cstr(b"SET_PARAMS returned %d\n\0"), ret);
                let del_ret = ublk_ctrl_del_dev(dev);
                if del_ret < 0 && ret == 0 { ret = del_ret; }
            }
        }
    }
    ublk_ctrl_deinit(dev);
    if ret < 0 { ret } else { 0 }
}

/* The remaining command entry points follow the original C control flow. */
unsafe fn cmd_dev_add(ctx: *mut dev_ctx) -> c_int {
    if (*ctx).fg { return __cmd_dev_add(ctx); }
    (*ctx)._shmid = shmget(IPC_PRIVATE, size_of::<ublk_dev>(), IPC_CREAT | 0o666);
    if (*ctx)._shmid < 0 {
        ublk_err(cstr(b"%s: failed to shmget %s\n\0"), cstr(b"cmd_dev_add\0"), strerror(errno));
        exit(-1);
    }
    (*ctx).shadow_dev = shmat((*ctx)._shmid, null(), 0) as *mut ublk_dev;
    if (*ctx).shadow_dev == (-1isize as *mut ublk_dev) {
        ublk_err(cstr(b"%s: failed to shmat %s\n\0"), cstr(b"cmd_dev_add\0"), strerror(errno));
        exit(-1);
    }
    (*ctx)._evtfd = eventfd(0, 0);
    if (*ctx)._evtfd < 0 {
        ublk_err(cstr(b"%s: failed to create eventfd %s\n\0"), cstr(b"cmd_dev_add\0"), strerror(errno));
        exit(-1);
    }
    let mut res = fork();
    if res == 0 {
        setsid();
        let res2 = fork();
        if res2 == 0 {
            close(STDIN_FILENO); close(STDOUT_FILENO); close(STDERR_FILENO);
            res = __cmd_dev_add(ctx);
            return res;
        } else {
            exit(EXIT_SUCCESS);
        }
    } else if res > 0 {
        let mut id: u64 = 0;
        let mut exit_code = EXIT_FAILURE;
        res = read((*ctx)._evtfd, &mut id as *mut _ as *mut c_void, size_of::<u64>()) as c_int;
        close((*ctx)._evtfd);
        if res == size_of::<u64>() as c_int && id != ERROR_EVTFD_DEVID {
            (*ctx).dev_id = (id - 1) as c_int;
            if __cmd_dev_list(ctx) >= 0 { exit_code = EXIT_SUCCESS; }
        }
        shmdt((*ctx).shadow_dev as *const c_void);
        shmctl((*ctx)._shmid, IPC_RMID, null_mut());
        wait(null_mut());
        if exit_code == EXIT_FAILURE {
            ublk_err(cstr(b"%s: command failed\n\0"), cstr(b"cmd_dev_add\0"));
        }
        exit(exit_code);
    } else {
        exit(EXIT_FAILURE);
    }
}
unsafe fn __cmd_dev_del(ctx: *mut dev_ctx) -> c_int {
    let number = (*ctx).dev_id;
    let dev = ublk_ctrl_init();
    (*dev).dev_info.dev_id = number;
    let mut ret = ublk_ctrl_get_info(dev);
    if ret >= 0 {
        ret = ublk_ctrl_stop_dev(dev);
        if ret < 0 { ublk_err(cstr(b"%s: stop dev %d failed ret %d\n\0"), cstr(b"__cmd_dev_del\0"), number, ret); }
        ret = ublk_stop_io_daemon(dev);
        if ret < 0 { ublk_err(cstr(b"%s: stop daemon id %d dev %d, ret %d\n\0"), cstr(b"__cmd_dev_del\0"), (*dev).dev_info.ublksrv_pid, number, ret); }
        ublk_ctrl_del_dev(dev);
    }
    ublk_ctrl_deinit(dev);
    if ret >= 0 { 0 } else { ret }
}
unsafe fn cmd_dev_del(ctx: *mut dev_ctx) -> c_int {
    if (*ctx).dev_id >= 0 || !(*ctx).all { return __cmd_dev_del(ctx); }
    for i in 0..255 { (*ctx).dev_id = i; __cmd_dev_del(ctx); }
    0
}
unsafe fn cmd_dev_stop(ctx: *mut dev_ctx) -> c_int {
    let number = (*ctx).dev_id;
    if number < 0 {
        ublk_err(cstr(b"%s: device id is required\n\0"), cstr(b"cmd_dev_stop\0"));
        return -EINVAL;
    }
    let dev = ublk_ctrl_init();
    (*dev).dev_info.dev_id = number;
    let mut ret = ublk_ctrl_get_info(dev);
    if ret >= 0 {
        ret = if (*ctx).safe_stop { ublk_ctrl_try_stop_dev(dev) } else { ublk_ctrl_stop_dev(dev) };
        if ret < 0 {
            ublk_err(cstr(b"%s: stop dev %d failed ret %d\n\0"), cstr(b"cmd_dev_stop\0"), number, ret);
        }
    }
    ublk_ctrl_deinit(dev);
    ret
}
unsafe fn __cmd_dev_list_impl(ctx: *mut dev_ctx) -> c_int {
    let dev = ublk_ctrl_init();
    if dev.is_null() { return -ENODEV; }
    (*dev).dev_info.dev_id = (*ctx).dev_id;
    let ret = ublk_ctrl_get_info(dev);
    if ret < 0 {
        if (*ctx).logging {
            ublk_err(cstr(b"%s: can't get dev info from %d: %d\n\0"), cstr(b"__cmd_dev_list\0"), (*ctx).dev_id, ret);
        }
    } else {
        if !(*ctx).shadow_dev.is_null() {
            memcpy(&mut (*dev).q as *mut _ as *mut c_void, &(*(*ctx).shadow_dev).q as *const _ as *const c_void, size_of_val(&(*dev).q));
        }
        ublk_ctrl_dump(dev);
    }
    ublk_ctrl_deinit(dev);
    ret
}
unsafe fn __cmd_dev_list(ctx: *mut dev_ctx) -> c_int { __cmd_dev_list_impl(ctx) }
unsafe fn cmd_dev_list(ctx: *mut dev_ctx) -> c_int {
    if (*ctx).dev_id >= 0 || !(*ctx).all { return __cmd_dev_list(ctx); }
    (*ctx).logging = false;
    for i in 0..255 { (*ctx).dev_id = i; __cmd_dev_list(ctx); }
    0
}
unsafe fn cmd_dev_get_features() -> c_int {
    let feat_map = [
        cstr(b"UBLK_F_SUPPORT_ZERO_COPY\0"),
        cstr(b"UBLK_F_URING_CMD_COMP_IN_TASK\0"),
        cstr(b"UBLK_F_NEED_GET_DATA\0"),
        cstr(b"UBLK_F_USER_RECOVERY\0"),
        cstr(b"UBLK_F_USER_RECOVERY_REISSUE\0"),
        cstr(b"UBLK_F_UNPRIVILEGED_DEV\0"),
        cstr(b"UBLK_F_CMD_IOCTL_ENCODE\0"),
        cstr(b"UBLK_F_USER_COPY\0"),
        cstr(b"UBLK_F_ZONED\0"),
        cstr(b"UBLK_F_USER_RECOVERY_FAIL_IO\0"),
        cstr(b"UBLK_F_UPDATE_SIZE\0"),
        cstr(b"UBLK_F_AUTO_BUF_REG\0"),
        cstr(b"UBLK_F_QUIESCE\0"),
        cstr(b"UBLK_F_PER_IO_DAEMON\0"),
        cstr(b"UBLK_F_BUF_REG_OFF_DAEMON\0"),
        cstr(b"UBLK_F_INTEGRITY\0"),
        cstr(b"UBLK_F_SAFE_STOP_DEV\0"),
        cstr(b"UBLK_F_BATCH_IO\0"),
        cstr(b"UBLK_F_NO_AUTO_PART_SCAN\0"),
        cstr(b"UBLK_F_SHMEM_ZC\0"),
        cstr(b"UBLK_F_IO_DESC_SIZE\0"),
    ];
    let dev = ublk_ctrl_init();
    if dev.is_null() {
        fprintf(stderr, cstr(b"ublksrv_ctrl_init failed id\n\0"));
        return -EOPNOTSUPP;
    }
    let mut features: __u64 = 0;
    let ret = ublk_ctrl_get_features(dev, &mut features);
    if ret == 0 {
        printf(cstr(b"ublk_drv features: 0x%llx\n\0"), features);
        for i in 0..(size_of::<__u64>() * 8) {
            if ((1u64 << i) & features) == 0 { continue; }
            let feat = if i < feat_map.len() { feat_map[i] } else { cstr(b"unknown\0") };
            printf(cstr(b"0x%-16llx: %s\n\0"), 1u64 << i, feat);
        }
    }
    ret
}
unsafe fn cmd_dev_update_size(ctx: *mut dev_ctx) -> c_int {
    let dev = ublk_ctrl_init();
    let mut p: ublk_params = zeroed();
    let mut ret = -EINVAL;
    if dev.is_null() { return -ENODEV; }
    if (*ctx).dev_id < 0 {
        fprintf(stderr, cstr(b"device id isn't provided\n\0"));
    } else {
        (*dev).dev_info.dev_id = (*ctx).dev_id;
        ret = ublk_ctrl_get_params(dev, &mut p);
        if ret < 0 {
            ublk_err(cstr(b"failed to get params %d %s\n\0"), ret, strerror(-ret));
        } else if (*ctx).size & (((1u64) << p.basic.logical_bs_shift) - 1) != 0 {
            ublk_err(cstr(b"size isn't aligned with logical block size\n\0"));
            ret = -EINVAL;
        } else {
            ret = ublk_ctrl_update_size(dev, (*ctx).size >> 9);
        }
    }
    ublk_ctrl_deinit(dev);
    ret
}
unsafe fn cmd_dev_quiesce(ctx: *mut dev_ctx) -> c_int {
    let dev = ublk_ctrl_init();
    let mut ret = -EINVAL;
    if dev.is_null() { return -ENODEV; }
    if (*ctx).dev_id < 0 {
        fprintf(stderr, cstr(b"device id isn't provided for quiesce\n\0"));
    } else {
        (*dev).dev_info.dev_id = (*ctx).dev_id;
        ret = ublk_ctrl_quiesce_dev(dev, 10000);
    }
    ublk_ctrl_deinit(dev);
    ret
}

unsafe fn __cmd_create_help(exe: *mut c_char, recovery: bool) {
    printf(cstr(b"%s %s -t [null|loop|stripe|fault_inject] [-q nr_queues] [-d depth] [-n dev_id]\n\0"), exe, if recovery { cstr(b"recover\0") } else { cstr(b"add\0") });
    printf(cstr(b"\t[--foreground] [--quiet] [-z] [--auto_zc] [--auto_zc_fallback] [--debug_mask mask] [-r 0|1] [-g] [-u]\n\0"));
    printf(cstr(b"\t[-e 0|1 ] [-i 0|1] [--no_ublk_fixed_fd]\n\0"));
    printf(cstr(b"\t[--nthreads threads] [--per_io_tasks]\n\0"));
    printf(cstr(b"\t[--integrity_capable] [--integrity_reftag] [--metadata_size SIZE] [--pi_offset OFFSET] [--csum_type ip|t10dif|nvme] [--tag_size SIZE]\n\0"));
    printf(cstr(b"\t[--batch|-b] [--rotate_auto_buf] [--no_auto_part_scan]\n\0"));
    printf(cstr(b"\t[--io_desc_size SIZE]\n\0"));
    printf(cstr(b"\t[target options] [backfile1] [backfile2] ...\n\0"));
    printf(cstr(b"\tdefault: nr_queues=2(max 32), depth=128(max 1024), dev_id=-1(auto allocation)\n\0"));
    printf(cstr(b"\tdefault: nthreads=nr_queues\0"));
    for ops in tgt_ops_list {
        if let Some(usage) = (*ops).usage {
            usage(ops);
        }
    }
}
unsafe fn cmd_add_help(exe: *mut c_char) { __cmd_create_help(exe, false); printf(cstr(b"\n\0")); }
unsafe fn cmd_recover_help(exe: *mut c_char) { __cmd_create_help(exe, true); printf(cstr(b"\tPlease provide exact command line for creating this device with real dev_id\n\0")); printf(cstr(b"\n\0")); }
unsafe fn cmd_dev_help(exe: *mut c_char) -> c_int {
    cmd_add_help(exe);
    cmd_recover_help(exe);
    printf(cstr(b"%s del [-n dev_id] -a \n\0"), exe);
    printf(cstr(b"\t -a delete all devices -n delete specified device\n\n\0"));
    printf(cstr(b"%s stop -n dev_id [--safe]\n\0"), exe);
    printf(cstr(b"\t --safe only stop if device has no active openers\n\n\0"));
    printf(cstr(b"%s list [-n dev_id] -a \n\0"), exe);
    printf(cstr(b"\t -a list all devices, -n list specified device, default -a \n\n\0"));
    printf(cstr(b"%s set_params [-n dev_id] [-q nr_queues] [-d depth] [-u] [--zoned]\n\0"), exe);
    printf(cstr(b"\t[--param_types basic[,zoned]|none]\n\0"));
    printf(cstr(b"\t issue ADD_DEV, SET_PARAMS and DEL_DEV without START_DEV\n\n\0"));
    printf(cstr(b"%s features\n\0"), exe);
    printf(cstr(b"%s update_size -n dev_id -s|--size size_in_bytes \n\0"), exe);
    printf(cstr(b"%s quiesce -n dev_id\n\0"), exe);
    0
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn main(argc: c_int, argv: *mut *mut c_char) -> c_int {
    /* getopt_long table and option parser translated directly in structure;
     * long option names are matched by strcmp as in the C source. */
    let mut longopts: [option; 46] = zeroed();
    macro_rules! opt { ($i:expr, $name:expr, $has:expr, $val:expr) => {{
        longopts[$i] = option { name: cstr($name), has_arg: $has, flag: null_mut(), val: $val };
    }}}
    opt!(0,b"all\0",0,b'a' as c_int); opt!(1,b"type\0",1,b't' as c_int); opt!(2,b"number\0",1,b'n' as c_int);
    opt!(3,b"queues\0",1,b'q' as c_int); opt!(4,b"depth\0",1,b'd' as c_int); opt!(5,b"debug_mask\0",1,0);
    opt!(6,b"quiet\0",0,0); opt!(7,b"zero_copy\0",0,b'z' as c_int); opt!(8,b"foreground\0",0,0);
    opt!(9,b"recovery\0",1,b'r' as c_int); opt!(10,b"recovery_fail_io\0",1,b'e' as c_int); opt!(11,b"recovery_reissue\0",1,b'i' as c_int);
    opt!(12,b"get_data\0",1,b'g' as c_int); opt!(13,b"auto_zc\0",0,0); opt!(14,b"auto_zc_fallback\0",0,0);
    opt!(15,b"user_copy\0",0,b'u' as c_int); opt!(16,b"size\0",1,b's' as c_int); opt!(17,b"nthreads\0",1,0);
    opt!(18,b"per_io_tasks\0",0,0); opt!(19,b"no_ublk_fixed_fd\0",0,0); opt!(20,b"integrity_capable\0",0,0);
    opt!(21,b"integrity_reftag\0",0,0); opt!(22,b"metadata_size\0",1,0); opt!(23,b"pi_offset\0",1,0);
    opt!(24,b"csum_type\0",1,0); opt!(25,b"tag_size\0",1,0); opt!(26,b"safe\0",0,0);
    opt!(27,b"batch\0",0,b'b' as c_int); opt!(28,b"rotate_auto_buf\0",0,0); opt!(29,b"no_auto_part_scan\0",0,0);
    opt!(30,b"shmem_zc\0",0,0); opt!(31,b"htlb\0",1,0); opt!(32,b"rdonly_shmem_buf\0",0,0);
    opt!(33,b"io_desc_size\0",1,0); opt!(34,b"zoned\0",0,0); opt!(35,b"param_types\0",1,0);
    opt!(36,b"logical_bs_shift\0",1,0); opt!(37,b"physical_bs_shift\0",1,0); opt!(38,b"io_min_shift\0",1,0);
    opt!(39,b"io_opt_shift\0",1,0); opt!(40,b"max_sectors\0",1,0); opt!(41,b"chunk_sectors\0",1,0);
    opt!(42,b"dev_sectors\0",1,0); opt!(43,b"max_zone_append_sectors\0",1,0); opt!(44,b"max_open_zones\0",1,0);
    opt!(45,b"max_active_zones\0",1,0);
    let mut option_idx: c_int = 0;
    let cmd = *argv.add(1);
    let mut ctx: dev_ctx = zeroed();
    ctx._evtfd = -1;
    ctx.queue_depth = 128;
    ctx.nr_hw_queues = 2;
    ctx.dev_id = -1;
    strcpy(ctx.tgt_type.as_mut_ptr(), cstr(b"unknown\0"));
    ctx.csum_type = LBMD_PI_CSUM_NONE;
    ctx.io_desc_size = size_of::<ublksrv_io_desc>();
    ctx.params.types = UBLK_PARAM_TYPE_BASIC;
    ctx.params.logical_bs_shift = KUBLK_PARAM_LOGICAL_BS_SHIFT;
    ctx.params.physical_bs_shift = KUBLK_PARAM_PHYSICAL_BS_SHIFT;
    ctx.params.io_min_shift = KUBLK_PARAM_LOGICAL_BS_SHIFT;
    ctx.params.io_opt_shift = KUBLK_PARAM_PHYSICAL_BS_SHIFT;
    ctx.params.max_sectors = UBLK_IO_MAX_BYTES >> KUBLK_PARAM_LOGICAL_BS_SHIFT;
    ctx.params.chunk_sectors = KUBLK_PARAM_ZONE_SECTORS as __u32;
    ctx.params.dev_sectors = KUBLK_PARAM_DEV_SECTORS;
    ctx.params.max_zone_append_sectors = KUBLK_PARAM_ZONE_APPEND_SECTORS;
    let mut ret = -EINVAL;
    let mut tgt_argc = 1;
    let mut tgt_argv: [*mut c_char; MAX_NR_TGT_ARG] = [null_mut(); MAX_NR_TGT_ARG];
    if argc == 1 { return ret; }
    opterr = 0;
    optind = 2;
    loop {
        let opt = getopt_long(argc, argv, cstr(b"t:n:d:q:r:e:i:s:gazub\0"), longopts.as_ptr(), &mut option_idx);
        if opt == -1 { break; }
        match opt {
            x if x == b'a' as c_int => ctx.all = true,
            x if x == b'b' as c_int => ctx.flags |= UBLK_F_BATCH_IO,
            x if x == b'n' as c_int => ctx.dev_id = strtol(optarg, null_mut(), 10) as c_int,
            x if x == b't' as c_int => { if strlen(optarg) < ctx.tgt_type.len() { strcpy(ctx.tgt_type.as_mut_ptr(), optarg); } }
            x if x == b'q' as c_int => ctx.nr_hw_queues = strtol(optarg, null_mut(), 10) as c_uint,
            x if x == b'd' as c_int => ctx.queue_depth = strtol(optarg, null_mut(), 10) as c_uint,
            x if x == b'z' as c_int => ctx.flags |= UBLK_F_SUPPORT_ZERO_COPY,
            x if x == b'r' as c_int => { if strtol(optarg, null_mut(), 10) != 0 { ctx.flags |= UBLK_F_USER_RECOVERY; } }
            x if x == b'e' as c_int => { if strtol(optarg, null_mut(), 10) != 0 { ctx.flags |= UBLK_F_USER_RECOVERY | UBLK_F_USER_RECOVERY_FAIL_IO; } }
            x if x == b'i' as c_int => { if strtol(optarg, null_mut(), 10) != 0 { ctx.flags |= UBLK_F_USER_RECOVERY | UBLK_F_USER_RECOVERY_REISSUE; } }
            x if x == b'g' as c_int => ctx.flags |= UBLK_F_NEED_GET_DATA,
            x if x == b'u' as c_int => ctx.flags |= UBLK_F_USER_COPY,
            x if x == b's' as c_int => ctx.size = strtoull(optarg, null_mut(), 10) as __u64,
            0 => {
                let name = longopts[option_idx as usize].name;
                if strcmp(name, cstr(b"debug_mask\0")) == 0 { ublk_dbg_mask = strtol(optarg, null_mut(), 16) as c_uint; }
                if strcmp(name, cstr(b"quiet\0")) == 0 { ublk_dbg_mask = 0; }
                if strcmp(name, cstr(b"foreground\0")) == 0 { ctx.fg = true; }
                if strcmp(name, cstr(b"auto_zc\0")) == 0 { ctx.flags |= UBLK_F_AUTO_BUF_REG; }
                if strcmp(name, cstr(b"auto_zc_fallback\0")) == 0 { ctx.auto_zc_fallback = true; }
                if strcmp(name, cstr(b"rotate_auto_buf\0")) == 0 { ctx.rotate_auto_buf = true; }
                if strcmp(name, cstr(b"nthreads\0")) == 0 { ctx.nthreads = strtol(optarg, null_mut(), 10) as c_uint; }
                if strcmp(name, cstr(b"per_io_tasks\0")) == 0 { ctx.per_io_tasks = true; }
                if strcmp(name, cstr(b"no_ublk_fixed_fd\0")) == 0 { ctx.no_ublk_fixed_fd = true; }
                if strcmp(name, cstr(b"integrity_capable\0")) == 0 { ctx.integrity_flags |= LBMD_PI_CAP_INTEGRITY; }
                if strcmp(name, cstr(b"integrity_reftag\0")) == 0 { ctx.integrity_flags |= LBMD_PI_CAP_REFTAG; }
                if strcmp(name, cstr(b"metadata_size\0")) == 0 { ctx.metadata_size = strtoul(optarg, null_mut(), 0) as __u8; }
                if strcmp(name, cstr(b"pi_offset\0")) == 0 { ctx.pi_offset = strtoul(optarg, null_mut(), 0) as __u32; }
                if strcmp(name, cstr(b"csum_type\0")) == 0 {
                    if strcmp(optarg, cstr(b"ip\0")) == 0 { ctx.csum_type = LBMD_PI_CSUM_IP; }
                    else if strcmp(optarg, cstr(b"t10dif\0")) == 0 { ctx.csum_type = LBMD_PI_CSUM_CRC16_T10DIF; }
                    else if strcmp(optarg, cstr(b"nvme\0")) == 0 { ctx.csum_type = LBMD_PI_CSUM_CRC64_NVME; }
                    else { ublk_err(cstr(b"invalid csum_type: %s\n\0"), optarg); return -EINVAL; }
                }
                if strcmp(name, cstr(b"tag_size\0")) == 0 { ctx.tag_size = strtoul(optarg, null_mut(), 0) as __u32; }
                if strcmp(name, cstr(b"safe\0")) == 0 { ctx.safe_stop = true; }
                if strcmp(name, cstr(b"no_auto_part_scan\0")) == 0 { ctx.flags |= UBLK_F_NO_AUTO_PART_SCAN; }
                if strcmp(name, cstr(b"shmem_zc\0")) == 0 { ctx.flags |= UBLK_F_SHMEM_ZC; }
                if strcmp(name, cstr(b"htlb\0")) == 0 { ctx.htlb_path = strdup(optarg); }
                if strcmp(name, cstr(b"rdonly_shmem_buf\0")) == 0 { ctx.rdonly_shmem_buf = true; }
                if strcmp(name, cstr(b"io_desc_size\0")) == 0 { ctx.flags |= UBLK_F_IO_DESC_SIZE; ctx.io_desc_size = strtoul(optarg, null_mut(), 0) as size_t; }
                if strcmp(name, cstr(b"zoned\0")) == 0 { ctx.flags |= UBLK_F_ZONED; }
                if strcmp(name, cstr(b"param_types\0")) == 0 { ret = parse_param_types(optarg, &mut ctx.params.types); if ret != 0 { return ret; } }
                if strcmp(name, cstr(b"logical_bs_shift\0")) == 0 { ctx.params.logical_bs_shift = strtoul(optarg, null_mut(), 0) as __u32; }
                if strcmp(name, cstr(b"physical_bs_shift\0")) == 0 { ctx.params.physical_bs_shift = strtoul(optarg, null_mut(), 0) as __u32; }
                if strcmp(name, cstr(b"io_min_shift\0")) == 0 { ctx.params.io_min_shift = strtoul(optarg, null_mut(), 0) as __u32; }
                if strcmp(name, cstr(b"io_opt_shift\0")) == 0 { ctx.params.io_opt_shift = strtoul(optarg, null_mut(), 0) as __u32; }
                if strcmp(name, cstr(b"max_sectors\0")) == 0 { ctx.params.max_sectors = strtoul(optarg, null_mut(), 0) as __u32; }
                if strcmp(name, cstr(b"chunk_sectors\0")) == 0 { ctx.params.chunk_sectors = strtoul(optarg, null_mut(), 0) as __u32; }
                if strcmp(name, cstr(b"dev_sectors\0")) == 0 { ctx.params.dev_sectors = strtoull(optarg, null_mut(), 0) as __u64; }
                if strcmp(name, cstr(b"max_zone_append_sectors\0")) == 0 { ctx.params.max_zone_append_sectors = strtoul(optarg, null_mut(), 0) as __u32; }
                if strcmp(name, cstr(b"max_open_zones\0")) == 0 { ctx.params.max_open_zones = strtoul(optarg, null_mut(), 0) as __u32; }
                if strcmp(name, cstr(b"max_active_zones\0")) == 0 { ctx.params.max_active_zones = strtoul(optarg, null_mut(), 0) as __u32; }
            }
            x if x == b'?' as c_int => {
                if (**argv.add(optind as usize) == b'-' as c_char) || (**argv.add((optind - 1) as usize) != b'-' as c_char) {
                    fprintf(stderr, cstr(b"every target option requires argument: %s %s\n\0"), *argv.add((optind - 1) as usize), *argv.add(optind as usize));
                    exit(EXIT_FAILURE);
                }
                if tgt_argc < (MAX_NR_TGT_ARG as c_int - 1) / 2 {
                    tgt_argv[tgt_argc as usize] = *argv.add((optind - 1) as usize);
                    tgt_argc += 1;
                    tgt_argv[tgt_argc as usize] = *argv.add(optind as usize);
                    tgt_argc += 1;
                } else {
                    fprintf(stderr, cstr(b"too many target options\n\0"));
                    exit(EXIT_FAILURE);
                }
                optind += 1;
            }
            _ => {}
        }
    }
    if ctx.per_io_tasks && (ctx.flags & UBLK_F_BATCH_IO != 0) {
        ublk_err(cstr(b"per_io_task and F_BATCH_IO conflict\n\0"));
        return -EINVAL;
    }
    if ctx.auto_zc_fallback && !((ctx.flags & UBLK_F_AUTO_BUF_REG != 0) && (ctx.flags & UBLK_F_SUPPORT_ZERO_COPY != 0)) {
        ublk_err(cstr(b"%s: auto_zc_fallback is set but neither F_AUTO_BUF_REG nor F_SUPPORT_ZERO_COPY is enabled\n\0"), cstr(b"main\0"));
        return -EINVAL;
    }
    if ((ctx.flags & UBLK_F_NEED_GET_DATA != 0) as c_int
        + (ctx.flags & UBLK_F_USER_COPY != 0) as c_int
        + ((ctx.flags & UBLK_F_SUPPORT_ZERO_COPY != 0) && !ctx.auto_zc_fallback) as c_int
        + ((ctx.flags & UBLK_F_AUTO_BUF_REG != 0) && !ctx.auto_zc_fallback) as c_int
        + ctx.auto_zc_fallback as c_int) > 1 {
        fprintf(stderr, cstr(b"too many data copy modes specified\n\0"));
        return -EINVAL;
    }
    if ctx.metadata_size != 0 {
        if ctx.flags & UBLK_F_USER_COPY == 0 {
            ublk_err(cstr(b"integrity requires user_copy\n\0"));
            return -EINVAL;
        }
        ctx.flags |= UBLK_F_INTEGRITY;
    } else if ctx.integrity_flags != 0 || ctx.pi_offset != 0 || ctx.csum_type != LBMD_PI_CSUM_NONE || ctx.tag_size != 0 {
        ublk_err(cstr(b"integrity parameters require metadata_size\n\0"));
        return -EINVAL;
    }
    if ctx.flags & UBLK_F_AUTO_BUF_REG != 0 && ctx.flags & UBLK_F_BATCH_IO != 0 && ctx.nthreads > ctx.nr_hw_queues {
        ublk_err(cstr(b"too many threads for F_AUTO_BUF_REG & F_BATCH_IO\n\0"));
        return -EINVAL;
    }
    if ctx.rotate_auto_buf && !((ctx.flags & UBLK_F_AUTO_BUF_REG != 0) && (ctx.flags & UBLK_F_BATCH_IO != 0)) {
        ublk_err(cstr(b"rotate_auto_buf requires --auto_zc and --batch\n\0"));
        return -EINVAL;
    }
    let mut i = optind;
    while i < argc && ctx.nr_files < MAX_BACK_FILES as c_int {
        ctx.files[ctx.nr_files as usize] = *argv.add(i as usize);
        ctx.nr_files += 1;
        i += 1;
    }
    let ops = ublk_find_tgt(ctx.tgt_type.as_ptr());
    if !ops.is_null() {
        if let Some(parse_cmd_line) = (*ops).parse_cmd_line {
            optind = 0;
            tgt_argv[0] = ctx.tgt_type.as_mut_ptr();
            parse_cmd_line(&mut ctx, tgt_argc, tgt_argv.as_mut_ptr());
        }
    }
    if strcmp(cmd, cstr(b"set_params\0")) == 0 { ret = cmd_dev_set_params(&mut ctx); }
    else if strcmp(cmd, cstr(b"add\0")) == 0 { ret = cmd_dev_add(&mut ctx); }
    else if strcmp(cmd, cstr(b"recover\0")) == 0 {
        if ctx.dev_id < 0 {
            fprintf(stderr, cstr(b"device id isn't provided for recovering\n\0"));
            ret = -EINVAL;
        } else {
            ctx.recovery = true;
            ret = cmd_dev_add(&mut ctx);
        }
    } else if strcmp(cmd, cstr(b"del\0")) == 0 { ret = cmd_dev_del(&mut ctx); }
    else if strcmp(cmd, cstr(b"stop\0")) == 0 { ret = cmd_dev_stop(&mut ctx); }
    else if strcmp(cmd, cstr(b"list\0")) == 0 { ctx.all = true; ret = cmd_dev_list(&mut ctx); }
    else if strcmp(cmd, cstr(b"help\0")) == 0 { ret = cmd_dev_help(*argv.add(0)); }
    else if strcmp(cmd, cstr(b"features\0")) == 0 { ret = cmd_dev_get_features(); }
    else if strcmp(cmd, cstr(b"update_size\0")) == 0 { ret = cmd_dev_update_size(&mut ctx); }
    else if strcmp(cmd, cstr(b"quiesce\0")) == 0 { ret = cmd_dev_quiesce(&mut ctx); }
    else { cmd_dev_help(*argv.add(0)); }
    ret
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
