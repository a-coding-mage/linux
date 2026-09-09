/* SPDX-License-Identifier: GPL-2.0-or-later */
/* Copyright (c) 2025 Stefan Metzmacher */

// Dependencies are supplied by the surrounding kernel/RDMA translation.

#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum smbdirect_socket_status {
    SMBDIRECT_SOCKET_CREATED,
    SMBDIRECT_SOCKET_LISTENING,
    SMBDIRECT_SOCKET_RESOLVE_ADDR_NEEDED,
    SMBDIRECT_SOCKET_RESOLVE_ADDR_RUNNING,
    SMBDIRECT_SOCKET_RESOLVE_ADDR_FAILED,
    SMBDIRECT_SOCKET_RESOLVE_ROUTE_NEEDED,
    SMBDIRECT_SOCKET_RESOLVE_ROUTE_RUNNING,
    SMBDIRECT_SOCKET_RESOLVE_ROUTE_FAILED,
    SMBDIRECT_SOCKET_RDMA_CONNECT_NEEDED,
    SMBDIRECT_SOCKET_RDMA_CONNECT_RUNNING,
    SMBDIRECT_SOCKET_RDMA_CONNECT_FAILED,
    SMBDIRECT_SOCKET_NEGOTIATE_NEEDED,
    SMBDIRECT_SOCKET_NEGOTIATE_RUNNING,
    SMBDIRECT_SOCKET_NEGOTIATE_FAILED,
    SMBDIRECT_SOCKET_CONNECTED,
    SMBDIRECT_SOCKET_ERROR,
    SMBDIRECT_SOCKET_DISCONNECTING,
    SMBDIRECT_SOCKET_DISCONNECTED,
    SMBDIRECT_SOCKET_DESTROYED,
}

pub unsafe fn smbdirect_socket_status_string(status: smbdirect_socket_status) -> &'static str {
    match status {
        smbdirect_socket_status::SMBDIRECT_SOCKET_CREATED => "CREATED",
        smbdirect_socket_status::SMBDIRECT_SOCKET_LISTENING => "LISTENING",
        smbdirect_socket_status::SMBDIRECT_SOCKET_RESOLVE_ADDR_NEEDED => "RESOLVE_ADDR_NEEDED",
        smbdirect_socket_status::SMBDIRECT_SOCKET_RESOLVE_ADDR_RUNNING => "RESOLVE_ADDR_RUNNING",
        smbdirect_socket_status::SMBDIRECT_SOCKET_RESOLVE_ADDR_FAILED => "RESOLVE_ADDR_FAILED",
        smbdirect_socket_status::SMBDIRECT_SOCKET_RESOLVE_ROUTE_NEEDED => "RESOLVE_ROUTE_NEEDED",
        smbdirect_socket_status::SMBDIRECT_SOCKET_RESOLVE_ROUTE_RUNNING => "RESOLVE_ROUTE_RUNNING",
        smbdirect_socket_status::SMBDIRECT_SOCKET_RESOLVE_ROUTE_FAILED => "RESOLVE_ROUTE_FAILED",
        smbdirect_socket_status::SMBDIRECT_SOCKET_RDMA_CONNECT_NEEDED => "RDMA_CONNECT_NEEDED",
        smbdirect_socket_status::SMBDIRECT_SOCKET_RDMA_CONNECT_RUNNING => "RDMA_CONNECT_RUNNING",
        smbdirect_socket_status::SMBDIRECT_SOCKET_RDMA_CONNECT_FAILED => "RDMA_CONNECT_FAILED",
        smbdirect_socket_status::SMBDIRECT_SOCKET_NEGOTIATE_NEEDED => "NEGOTIATE_NEEDED",
        smbdirect_socket_status::SMBDIRECT_SOCKET_NEGOTIATE_RUNNING => "NEGOTIATE_RUNNING",
        smbdirect_socket_status::SMBDIRECT_SOCKET_NEGOTIATE_FAILED => "NEGOTIATE_FAILED",
        smbdirect_socket_status::SMBDIRECT_SOCKET_CONNECTED => "CONNECTED",
        smbdirect_socket_status::SMBDIRECT_SOCKET_ERROR => "ERROR",
        smbdirect_socket_status::SMBDIRECT_SOCKET_DISCONNECTING => "DISCONNECTING",
        smbdirect_socket_status::SMBDIRECT_SOCKET_DISCONNECTED => "DISCONNECTED",
        smbdirect_socket_status::SMBDIRECT_SOCKET_DESTROYED => "DESTROYED",
        _ => "<unknown>",
    }
}

pub unsafe fn SMBDIRECT_DEBUG_ERR_PTR(error: libc::c_long) -> *const libc::c_void {
    if error == 0 { core::ptr::null() } else { ERR_PTR(error) }
}

#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum smbdirect_keepalive_status { SMBDIRECT_KEEPALIVE_NONE, SMBDIRECT_KEEPALIVE_PENDING, SMBDIRECT_KEEPALIVE_SENT }

#[repr(C)]
pub struct smbdirect_socket {
    pub status: smbdirect_socket_status,
    pub status_wait: wait_queue_head_t,
    pub first_error: libc::c_int,
    pub workqueues: Workqueues,
    pub disconnect_work: work_struct,
    pub refs: SocketRefs,
    pub rdma: SocketRdma,
    pub ib: SocketIb,
    pub parameters: smbdirect_socket_parameters,
    pub connect: SocketConnect,
    pub idle: SocketIdle,
    pub listen: SocketListen,
    pub accept: SocketAccept,
    pub send_io: SocketSendIo,
    pub recv_io: SocketRecvIo,
    pub mr_io: SocketMrIo,
    pub rw_io: SocketRwIo,
    pub statistics: SocketStatistics,
    pub logging: SocketLogging,
}

#[repr(C)] pub struct Workqueues { pub accept: *mut workqueue_struct, pub connect: *mut workqueue_struct, pub idle: *mut workqueue_struct, pub refill: *mut workqueue_struct, pub immediate: *mut workqueue_struct, pub cleanup: *mut workqueue_struct }
#[repr(C)] pub struct SocketRefs { pub disconnect: kref, pub destroy: kref }
#[repr(C)] pub struct SocketRdma { pub cm_id: *mut rdma_cm_id, pub expected_event: rdma_cm_event_type, pub legacy_iwarp: bool }
#[repr(C)] pub struct SocketIb { pub pd: *mut ib_pd, pub poll_ctx: ib_poll_context, pub send_cq: *mut ib_cq, pub recv_cq: *mut ib_cq, pub qp: *mut ib_qp, pub dev: *mut ib_device }
#[repr(C)] pub struct SocketConnect { pub lock: spinlock_t, pub work: work_struct }
#[repr(C)] pub struct SocketIdle { pub keepalive: smbdirect_keepalive_status, pub immediate_work: work_struct, pub timer_work: delayed_work }
#[repr(C)] pub struct SocketListen { pub lock: spinlock_t, pub pending: list_head, pub ready: list_head, pub wait_queue: wait_queue_head_t, pub backlog: libc::c_int }
#[repr(C)] pub struct SocketAccept { pub listener: *mut smbdirect_socket, pub list: list_head }
#[repr(C)] pub struct SocketMem { pub cache: *mut kmem_cache, pub pool: *mut mempool_t, pub gfp_mask: gfp_t }
#[repr(C)] pub struct SocketCounter { pub count: atomic_t, pub wait_queue: wait_queue_head_t }
#[repr(C)] pub struct SocketSendIo { pub mem: SocketMem, pub bcredits: SocketCounter, pub lcredits: SocketCounter, pub credits: SocketCounter, pub pending: SocketPending }
#[repr(C)] pub struct SocketPending { pub count: atomic_t, pub zero_wait_queue: wait_queue_head_t }
#[repr(C)] pub struct SocketRecvIo { pub expected: libc::c_int, pub mem: SocketMem, pub free: SocketFree, pub posted: SocketPosted, pub credits: SocketRecvCredits, pub reassembly: SocketReassembly }
#[repr(C)] pub struct SocketFree { pub list: list_head, pub lock: spinlock_t }
#[repr(C)] pub struct SocketPosted { pub count: atomic_t, pub refill_work: work_struct }
#[repr(C)] pub struct SocketRecvCredits { pub target: u16, pub available: atomic_t, pub count: atomic_t }
#[repr(C)] pub struct SocketReassembly { pub list: list_head, pub lock: spinlock_t, pub wait_queue: wait_queue_head_t, pub data_length: libc::c_int, pub queue_length: libc::c_int, pub first_entry_offset: libc::c_int, pub full_packet_received: bool }
#[repr(C)] pub struct SocketMrIo { pub type_: ib_mr_type, pub all: SocketFree, pub ready: SocketCounter, pub used: SocketUsed }
#[repr(C)] pub struct SocketUsed { pub count: atomic_t }
#[repr(C)] pub struct SocketRwIo { pub mem: SocketRwMem, pub credits: SocketRwCredits }
#[repr(C)] pub struct SocketRwMem { pub gfp_mask: gfp_t }
#[repr(C)] pub struct SocketRwCredits { pub max: usize, pub num_pages: usize, pub count: atomic_t, pub wait_queue: wait_queue_head_t }
#[repr(C)] pub struct SocketStatistics { pub get_receive_buffer: u64, pub put_receive_buffer: u64, pub enqueue_reassembly_queue: u64, pub dequeue_reassembly_queue: u64, pub send_empty: u64 }
#[repr(C)] pub struct SocketLogging { pub private_ptr: *mut libc::c_void, pub needed: Option<unsafe extern "C" fn(*mut smbdirect_socket, *mut libc::c_void, libc::c_uint, libc::c_uint) -> bool>, pub vaprintf: Option<unsafe extern "C" fn(*mut smbdirect_socket, *const libc::c_char, libc::c_uint, *mut libc::c_void, libc::c_uint, libc::c_uint, *mut va_format)> }

unsafe extern "C" { fn ERR_PTR(error: libc::c_long) -> *const libc::c_void; }

pub const SMBDIRECT_SEND_IO_MAX_SGE: usize = 6;
pub const SMBDIRECT_RECV_IO_MAX_SGE: usize = 1;
pub const SMBDIRECT_RDMA_CM_RETRY: i32 = 6;
pub const SMBDIRECT_RDMA_CM_RNR_RETRY: i32 = 0;

#[repr(C)] pub struct smbdirect_send_io { pub socket: *mut smbdirect_socket, pub cqe: ib_cqe, pub num_sge: usize, pub sge: [ib_sge; SMBDIRECT_SEND_IO_MAX_SGE], pub sibling_list: list_head, pub wr: ib_send_wr, pub packet: [u8; 0] }
#[repr(C)] pub struct smbdirect_send_batch { pub msg_list: list_head, pub wr_cnt: usize, pub need_invalidate_rkey: bool, pub remote_key: u32, pub credit: libc::c_int }
#[repr(C)] pub struct smbdirect_recv_io { pub socket: *mut smbdirect_socket, pub cqe: ib_cqe, pub sge: ib_sge, pub list: list_head, pub first_segment: bool, pub packet: [u8; 0] }
#[repr(C)] pub struct smbdirect_mr_io { pub socket: *mut smbdirect_socket, pub cqe: ib_cqe, pub kref: kref, pub mutex: mutex, pub list: list_head, pub state: smbdirect_mr_state, pub mr: *mut ib_mr, pub sgt: sg_table, pub dir: dma_data_direction, pub wr: ib_reg_wr, pub need_invalidate: bool, pub invalidate_done: completion }
#[repr(C)] pub struct smbdirect_rw_io { pub socket: *mut smbdirect_socket, pub cqe: ib_cqe, pub list: list_head, pub error: libc::c_int, pub completion: *mut completion, pub rdma_ctx: rdma_rw_ctx, pub sgt: sg_table, pub sg_list: [scatterlist; 0] }
#[repr(C)] #[derive(Copy, Clone)] pub enum smbdirect_mr_state { SMBDIRECT_MR_READY, SMBDIRECT_MR_REGISTERED, SMBDIRECT_MR_INVALIDATED, SMBDIRECT_MR_ERROR, SMBDIRECT_MR_DISABLED }

pub const fn smbdirect_get_buf_page_count(buf: usize, size: usize) -> usize { (buf + size + PAGE_SIZE - 1) / PAGE_SIZE - buf / PAGE_SIZE }

#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum smbdirect_expected { SMBDIRECT_EXPECT_NEGOTIATE_REQ = 1, SMBDIRECT_EXPECT_NEGOTIATE_REP = 2, SMBDIRECT_EXPECT_DATA_TRANSFER = 3 }

unsafe fn __smbdirect_socket_disabled_work(_work: *mut work_struct) { WARN_ON_ONCE(1); }
unsafe fn __smbdirect_log_needed(_sc: *mut smbdirect_socket, _private_ptr: *mut libc::c_void, _lvl: libc::c_uint, _cls: libc::c_uint) -> bool { WARN_ON_ONCE(1); false }
unsafe fn __smbdirect_log_vaprintf(_sc: *mut smbdirect_socket, _func: *const libc::c_char, _line: libc::c_uint, _private_ptr: *mut libc::c_void, _lvl: libc::c_uint, _cls: libc::c_uint, _vaf: *mut va_format) { WARN_ON_ONCE(1); }

/* The following declarations preserve the C variadic logging interface. */
unsafe extern "C" { fn __smbdirect_log_printf(sc: *mut smbdirect_socket, func: *const libc::c_char, line: libc::c_uint, lvl: libc::c_uint, cls: libc::c_uint, fmt: *const libc::c_char, ...); }

#[macro_export]
macro_rules! __smbdirect_log_generic {
    ($sc:expr, $lvl:expr, $cls:expr, $fmt:expr $(, $arg:expr)*) => {{
        if unsafe { ((*$sc).logging.needed.unwrap())($sc, (*$sc).logging.private_ptr, $lvl, $cls) } {
            unsafe { __smbdirect_log_printf($sc, core::ptr::null(), 0, $lvl, $cls, $fmt $(, $arg)*) }
        }
    }};
}
#[macro_export] macro_rules! smbdirect_log_outgoing { ($($x:tt)*) => { $crate::__smbdirect_log_generic!($($x)*) }; }
#[macro_export] macro_rules! smbdirect_log_incoming { ($($x:tt)*) => { $crate::__smbdirect_log_generic!($($x)*) }; }
#[macro_export] macro_rules! smbdirect_log_read { ($($x:tt)*) => { $crate::__smbdirect_log_generic!($($x)*) }; }
#[macro_export] macro_rules! smbdirect_log_write { ($($x:tt)*) => { $crate::__smbdirect_log_generic!($($x)*) }; }
#[macro_export] macro_rules! smbdirect_log_rdma_send { ($($x:tt)*) => { $crate::__smbdirect_log_generic!($($x)*) }; }
#[macro_export] macro_rules! smbdirect_log_rdma_recv { ($($x:tt)*) => { $crate::__smbdirect_log_generic!($($x)*) }; }
#[macro_export] macro_rules! smbdirect_log_keep_alive { ($($x:tt)*) => { $crate::__smbdirect_log_generic!($($x)*) }; }
#[macro_export] macro_rules! smbdirect_log_rdma_event { ($($x:tt)*) => { $crate::__smbdirect_log_generic!($($x)*) }; }
#[macro_export] macro_rules! smbdirect_log_rdma_mr { ($($x:tt)*) => { $crate::__smbdirect_log_generic!($($x)*) }; }
#[macro_export] macro_rules! smbdirect_log_rdma_rw { ($($x:tt)*) => { $crate::__smbdirect_log_generic!($($x)*) }; }
#[macro_export] macro_rules! smbdirect_log_negotiate { ($($x:tt)*) => { $crate::__smbdirect_log_generic!($($x)*) }; }

/* C-only status-check statement expressions remain available as an explicit Rust helper. */
pub unsafe fn smbdirect_check_status_failed(sc: *mut smbdirect_socket, expected: smbdirect_socket_status) -> bool {
    if (*sc).first_error != 0 || (*sc).status != expected { true } else { false }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
