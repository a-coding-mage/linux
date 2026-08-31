/* SPDX-License-Identifier: (GPL-2.0 WITH Linux-syscall-note) OR MIT */
/*
 * Header file for the io_uring interface.
 *
 * Copyright (C) 2019 Jens Axboe
 * Copyright (C) 2019 Christoph Hellwig
 */

/* Dependencies from the original C header:
 * linux/fs.h, linux/types.h, and conditionally linux/time_types.h.
 */

/*
 * IO submission data structure (Submission Queue Entry)
 */
#[repr(C)]
#[derive(Copy, Clone)]
pub struct io_uring_sqe_cmd_op {
    pub cmd_op: __u32,
    pub __pad1: __u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub union io_uring_sqe_off_union {
    pub off: __u64, /* offset into file */
    pub addr2: __u64,
    pub cmd: io_uring_sqe_cmd_op,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct io_uring_sqe_sockopt {
    pub level: __u32,
    pub optname: __u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub union io_uring_sqe_addr_union {
    pub addr: __u64, /* pointer to buffer or iovecs */
    pub splice_off_in: __u64,
    pub sockopt: io_uring_sqe_sockopt,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub union io_uring_sqe_flags_union {
    pub rw_flags: __kernel_rwf_t,
    pub fsync_flags: __u32,
    pub poll_events: __u16,   /* compatibility */
    pub poll32_events: __u32, /* word-reversed for BE */
    pub sync_range_flags: __u32,
    pub msg_flags: __u32,
    pub timeout_flags: __u32,
    pub accept_flags: __u32,
    pub cancel_flags: __u32,
    pub open_flags: __u32,
    pub statx_flags: __u32,
    pub fadvise_advice: __u32,
    pub splice_flags: __u32,
    pub rename_flags: __u32,
    pub unlink_flags: __u32,
    pub hardlink_flags: __u32,
    pub xattr_flags: __u32,
    pub msg_ring_flags: __u32,
    pub uring_cmd_flags: __u32,
    pub waitid_flags: __u32,
    pub futex_flags: __u32,
}

/* pack this to avoid bogus arm OABI complaints */
#[repr(C, packed)]
#[derive(Copy, Clone)]
pub union io_uring_sqe_buf_union {
    pub buf_index: __u16, /* index into fixed buffers, if used */
    pub buf_group: __u16, /* for grouped buffer selection */
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct io_uring_sqe_addr_len {
    pub addr_len: __u16,
    pub __pad3: [__u16; 1],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub union io_uring_sqe_file_union {
    pub splice_fd_in: __s32,
    pub file_index: __u32,
    pub optlen: __u32,
    pub addr_len: io_uring_sqe_addr_len,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct io_uring_sqe_addr3 {
    pub addr3: __u64,
    pub __pad2: [__u64; 1],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub union io_uring_sqe_cmd_union {
    pub addr3: io_uring_sqe_addr3,
    pub optval: __u64,
    /*
     * If the ring is initialized with IORING_SETUP_SQE128, then
     * this field is used for 80 bytes of arbitrary command data
     */
    pub cmd: [__u8; 0],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct io_uring_sqe {
    pub opcode: __u8,  /* type of operation for this sqe */
    pub flags: __u8,   /* IOSQE_ flags */
    pub ioprio: __u16, /* ioprio for the request */
    pub fd: __s32,     /* file descriptor to do IO on */
    pub off: io_uring_sqe_off_union,
    pub addr: io_uring_sqe_addr_union,
    pub len: __u32, /* buffer size or number of iovecs */
    pub flags_union: io_uring_sqe_flags_union,
    pub user_data: __u64, /* data to be passed back at completion time */
    pub buf: io_uring_sqe_buf_union,
    pub personality: __u16, /* personality to use, if used */
    pub file: io_uring_sqe_file_union,
    pub cmd: io_uring_sqe_cmd_union,
}

/*
 * If sqe->file_index is set to this for opcodes that instantiate a new
 * direct descriptor (like openat/openat2/accept), then io_uring will allocate
 * an available direct descriptor instead of having the application pass one
 * in. The picked direct descriptor will be returned in cqe->res, or -ENFILE
 * if the space is full.
 */
pub const IORING_FILE_INDEX_ALLOC: __u32 = !0u32;

pub const IOSQE_FIXED_FILE_BIT: __u32 = 0;
pub const IOSQE_IO_DRAIN_BIT: __u32 = 1;
pub const IOSQE_IO_LINK_BIT: __u32 = 2;
pub const IOSQE_IO_HARDLINK_BIT: __u32 = 3;
pub const IOSQE_ASYNC_BIT: __u32 = 4;
pub const IOSQE_BUFFER_SELECT_BIT: __u32 = 5;
pub const IOSQE_CQE_SKIP_SUCCESS_BIT: __u32 = 6;

/*
 * sqe->flags
 */
pub const IOSQE_FIXED_FILE: __u32 = 1u32 << IOSQE_FIXED_FILE_BIT; /* use fixed fileset */
pub const IOSQE_IO_DRAIN: __u32 = 1u32 << IOSQE_IO_DRAIN_BIT; /* issue after inflight IO */
pub const IOSQE_IO_LINK: __u32 = 1u32 << IOSQE_IO_LINK_BIT; /* links next sqe */
pub const IOSQE_IO_HARDLINK: __u32 = 1u32 << IOSQE_IO_HARDLINK_BIT; /* like LINK, but stronger */
pub const IOSQE_ASYNC: __u32 = 1u32 << IOSQE_ASYNC_BIT; /* always go async */
pub const IOSQE_BUFFER_SELECT: __u32 = 1u32 << IOSQE_BUFFER_SELECT_BIT; /* select buffer from sqe->buf_group */
pub const IOSQE_CQE_SKIP_SUCCESS: __u32 = 1u32 << IOSQE_CQE_SKIP_SUCCESS_BIT; /* don't post CQE if request succeeded */

/*
 * io_uring_setup() flags
 */
pub const IORING_SETUP_IOPOLL: __u32 = 1u32 << 0; /* io_context is polled */
pub const IORING_SETUP_SQPOLL: __u32 = 1u32 << 1; /* SQ poll thread */
pub const IORING_SETUP_SQ_AFF: __u32 = 1u32 << 2; /* sq_thread_cpu is valid */
pub const IORING_SETUP_CQSIZE: __u32 = 1u32 << 3; /* app defines CQ size */
pub const IORING_SETUP_CLAMP: __u32 = 1u32 << 4; /* clamp SQ/CQ ring sizes */
pub const IORING_SETUP_ATTACH_WQ: __u32 = 1u32 << 5; /* attach to existing wq */
pub const IORING_SETUP_R_DISABLED: __u32 = 1u32 << 6; /* start with ring disabled */
pub const IORING_SETUP_SUBMIT_ALL: __u32 = 1u32 << 7; /* continue submit on error */
/*
 * Cooperative task running. When requests complete, they often require
 * forcing the submitter to transition to the kernel to complete. If this
 * flag is set, work will be done when the task transitions anyway, rather
 * than force an inter-processor interrupt reschedule. This avoids interrupting
 * a task running in userspace, and saves an IPI.
 */
pub const IORING_SETUP_COOP_TASKRUN: __u32 = 1u32 << 8;
/*
 * If COOP_TASKRUN is set, get notified if task work is available for
 * running and a kernel transition would be needed to run it. This sets
 * IORING_SQ_TASKRUN in the sq ring flags. Not valid with COOP_TASKRUN.
 */
pub const IORING_SETUP_TASKRUN_FLAG: __u32 = 1u32 << 9;
pub const IORING_SETUP_SQE128: __u32 = 1u32 << 10; /* SQEs are 128 byte */
pub const IORING_SETUP_CQE32: __u32 = 1u32 << 11; /* CQEs are 32 byte */
/*
 * Only one task is allowed to submit requests
 */
pub const IORING_SETUP_SINGLE_ISSUER: __u32 = 1u32 << 12;

/*
 * Defer running task work to get events.
 * Rather than running bits of task work whenever the task transitions
 * try to do it just before it is needed.
 */
pub const IORING_SETUP_DEFER_TASKRUN: __u32 = 1u32 << 13;

/*
 * Application provides the memory for the rings
 */
pub const IORING_SETUP_NO_MMAP: __u32 = 1u32 << 14;

/*
 * Register the ring fd in itself for use with
 * IORING_REGISTER_USE_REGISTERED_RING; return a registered fd index rather
 * than an fd.
 */
pub const IORING_SETUP_REGISTERED_FD_ONLY: __u32 = 1u32 << 15;

/*
 * Removes indirection through the SQ index array.
 */
pub const IORING_SETUP_NO_SQARRAY: __u32 = 1u32 << 16;

#[repr(C)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum io_uring_op {
    IORING_OP_NOP = 0,
    IORING_OP_READV = 1,
    IORING_OP_WRITEV = 2,
    IORING_OP_FSYNC = 3,
    IORING_OP_READ_FIXED = 4,
    IORING_OP_WRITE_FIXED = 5,
    IORING_OP_POLL_ADD = 6,
    IORING_OP_POLL_REMOVE = 7,
    IORING_OP_SYNC_FILE_RANGE = 8,
    IORING_OP_SENDMSG = 9,
    IORING_OP_RECVMSG = 10,
    IORING_OP_TIMEOUT = 11,
    IORING_OP_TIMEOUT_REMOVE = 12,
    IORING_OP_ACCEPT = 13,
    IORING_OP_ASYNC_CANCEL = 14,
    IORING_OP_LINK_TIMEOUT = 15,
    IORING_OP_CONNECT = 16,
    IORING_OP_FALLOCATE = 17,
    IORING_OP_OPENAT = 18,
    IORING_OP_CLOSE = 19,
    IORING_OP_FILES_UPDATE = 20,
    IORING_OP_STATX = 21,
    IORING_OP_READ = 22,
    IORING_OP_WRITE = 23,
    IORING_OP_FADVISE = 24,
    IORING_OP_MADVISE = 25,
    IORING_OP_SEND = 26,
    IORING_OP_RECV = 27,
    IORING_OP_OPENAT2 = 28,
    IORING_OP_EPOLL_CTL = 29,
    IORING_OP_SPLICE = 30,
    IORING_OP_PROVIDE_BUFFERS = 31,
    IORING_OP_REMOVE_BUFFERS = 32,
    IORING_OP_TEE = 33,
    IORING_OP_SHUTDOWN = 34,
    IORING_OP_RENAMEAT = 35,
    IORING_OP_UNLINKAT = 36,
    IORING_OP_MKDIRAT = 37,
    IORING_OP_SYMLINKAT = 38,
    IORING_OP_LINKAT = 39,
    IORING_OP_MSG_RING = 40,
    IORING_OP_FSETXATTR = 41,
    IORING_OP_SETXATTR = 42,
    IORING_OP_FGETXATTR = 43,
    IORING_OP_GETXATTR = 44,
    IORING_OP_SOCKET = 45,
    IORING_OP_URING_CMD = 46,
    IORING_OP_SEND_ZC = 47,
    IORING_OP_SENDMSG_ZC = 48,
    IORING_OP_READ_MULTISHOT = 49,
    IORING_OP_WAITID = 50,
    IORING_OP_FUTEX_WAIT = 51,
    IORING_OP_FUTEX_WAKE = 52,
    IORING_OP_FUTEX_WAITV = 53,

    /* this goes last, obviously */
    IORING_OP_LAST = 54,
}

/*
 * sqe->uring_cmd_flags		top 8bits aren't available for userspace
 * IORING_URING_CMD_FIXED	use registered buffer; pass this flag
 *				along with setting sqe->buf_index.
 */
pub const IORING_URING_CMD_FIXED: __u32 = 1u32 << 0;
pub const IORING_URING_CMD_MASK: __u32 = IORING_URING_CMD_FIXED;

/*
 * sqe->fsync_flags
 */
pub const IORING_FSYNC_DATASYNC: __u32 = 1u32 << 0;

/*
 * sqe->timeout_flags
 */
pub const IORING_TIMEOUT_ABS: __u32 = 1u32 << 0;
pub const IORING_TIMEOUT_UPDATE: __u32 = 1u32 << 1;
pub const IORING_TIMEOUT_BOOTTIME: __u32 = 1u32 << 2;
pub const IORING_TIMEOUT_REALTIME: __u32 = 1u32 << 3;
pub const IORING_LINK_TIMEOUT_UPDATE: __u32 = 1u32 << 4;
pub const IORING_TIMEOUT_ETIME_SUCCESS: __u32 = 1u32 << 5;
pub const IORING_TIMEOUT_MULTISHOT: __u32 = 1u32 << 6;
pub const IORING_TIMEOUT_CLOCK_MASK: __u32 = IORING_TIMEOUT_BOOTTIME | IORING_TIMEOUT_REALTIME;
pub const IORING_TIMEOUT_UPDATE_MASK: __u32 = IORING_TIMEOUT_UPDATE | IORING_LINK_TIMEOUT_UPDATE;
/*
 * sqe->splice_flags
 * extends splice(2) flags
 */
pub const SPLICE_F_FD_IN_FIXED: __u32 = 1u32 << 31; /* the last bit of __u32 */

/*
 * POLL_ADD flags. Note that since sqe->poll_events is the flag space, the
 * command flags for POLL_ADD are stored in sqe->len.
 *
 * IORING_POLL_ADD_MULTI	Multishot poll. Sets IORING_CQE_F_MORE if
 *				the poll handler will continue to report
 *				CQEs on behalf of the same SQE.
 *
 * IORING_POLL_UPDATE		Update existing poll request, matching
 *				sqe->addr as the old user_data field.
 *
 * IORING_POLL_LEVEL		Level triggered poll.
 */
pub const IORING_POLL_ADD_MULTI: __u32 = 1u32 << 0;
pub const IORING_POLL_UPDATE_EVENTS: __u32 = 1u32 << 1;
pub const IORING_POLL_UPDATE_USER_DATA: __u32 = 1u32 << 2;
pub const IORING_POLL_ADD_LEVEL: __u32 = 1u32 << 3;

/*
 * ASYNC_CANCEL flags.
 *
 * IORING_ASYNC_CANCEL_ALL	Cancel all requests that match the given key
 * IORING_ASYNC_CANCEL_FD	Key off 'fd' for cancelation rather than the
 *				request 'user_data'
 * IORING_ASYNC_CANCEL_ANY	Match any request
 * IORING_ASYNC_CANCEL_FD_FIXED	'fd' passed in is a fixed descriptor
 * IORING_ASYNC_CANCEL_USERDATA	Match on user_data, default for no other key
 * IORING_ASYNC_CANCEL_OP	Match request based on opcode
 */
pub const IORING_ASYNC_CANCEL_ALL: __u32 = 1u32 << 0;
pub const IORING_ASYNC_CANCEL_FD: __u32 = 1u32 << 1;
pub const IORING_ASYNC_CANCEL_ANY: __u32 = 1u32 << 2;
pub const IORING_ASYNC_CANCEL_FD_FIXED: __u32 = 1u32 << 3;
pub const IORING_ASYNC_CANCEL_USERDATA: __u32 = 1u32 << 4;
pub const IORING_ASYNC_CANCEL_OP: __u32 = 1u32 << 5;

/*
 * send/sendmsg and recv/recvmsg flags (sqe->ioprio)
 *
 * IORING_RECVSEND_POLL_FIRST	If set, instead of first attempting to send
 *				or receive and arm poll if that yields an
 *				-EAGAIN result, arm poll upfront and skip
 *				the initial transfer attempt.
 *
 * IORING_RECV_MULTISHOT	Multishot recv. Sets IORING_CQE_F_MORE if
 *				the handler will continue to report
 *				CQEs on behalf of the same SQE.
 *
 * IORING_RECVSEND_FIXED_BUF	Use registered buffers, the index is stored in
 *				the buf_index field.
 *
 * IORING_SEND_ZC_REPORT_USAGE
 *				If set, SEND[MSG]_ZC should report
 *				the zerocopy usage in cqe.res
 *				for the IORING_CQE_F_NOTIF cqe.
 *				0 is reported if zerocopy was actually possible.
 *				IORING_NOTIF_USAGE_ZC_COPIED if data was copied
 *				(at least partially).
 */
pub const IORING_RECVSEND_POLL_FIRST: __u32 = 1u32 << 0;
pub const IORING_RECV_MULTISHOT: __u32 = 1u32 << 1;
pub const IORING_RECVSEND_FIXED_BUF: __u32 = 1u32 << 2;
pub const IORING_SEND_ZC_REPORT_USAGE: __u32 = 1u32 << 3;

/*
 * cqe.res for IORING_CQE_F_NOTIF if
 * IORING_SEND_ZC_REPORT_USAGE was requested
 *
 * It should be treated as a flag, all other
 * bits of cqe.res should be treated as reserved!
 */
pub const IORING_NOTIF_USAGE_ZC_COPIED: __u32 = 1u32 << 31;

/*
 * accept flags stored in sqe->ioprio
 */
pub const IORING_ACCEPT_MULTISHOT: __u32 = 1u32 << 0;

/*
 * IORING_OP_MSG_RING command types, stored in sqe->addr
 */
pub const IORING_MSG_DATA: __u32 = 0; /* pass sqe->len as 'res' and off as user_data */
pub const IORING_MSG_SEND_FD: __u32 = 1; /* send a registered fd to another ring */

/*
 * IORING_OP_MSG_RING flags (sqe->msg_ring_flags)
 *
 * IORING_MSG_RING_CQE_SKIP	Don't post a CQE to the target ring. Not
 *				applicable for IORING_MSG_DATA, obviously.
 */
pub const IORING_MSG_RING_CQE_SKIP: __u32 = 1u32 << 0;
/* Pass through the flags from sqe->file_index to cqe->flags */
pub const IORING_MSG_RING_FLAGS_PASS: __u32 = 1u32 << 1;

/*
 * IO completion data structure (Completion Queue Entry)
 */
#[repr(C)]
#[derive(Copy, Clone)]
pub struct io_uring_cqe {
    pub user_data: __u64, /* sqe->data submission passed back */
    pub res: __s32,      /* result code for this event */
    pub flags: __u32,

    /*
     * If the ring is initialized with IORING_SETUP_CQE32, then this field
     * contains 16-bytes of padding, doubling the size of the CQE.
     */
    pub big_cqe: [__u64; 0],
}

/*
 * cqe->flags
 *
 * IORING_CQE_F_BUFFER	If set, the upper 16 bits are the buffer ID
 * IORING_CQE_F_MORE	If set, parent SQE will generate more CQE entries
 * IORING_CQE_F_SOCK_NONEMPTY	If set, more data to read after socket recv
 * IORING_CQE_F_NOTIF	Set for notification CQEs. Can be used to distinct
 * 			them from sends.
 */
pub const IORING_CQE_F_BUFFER: __u32 = 1u32 << 0;
pub const IORING_CQE_F_MORE: __u32 = 1u32 << 1;
pub const IORING_CQE_F_SOCK_NONEMPTY: __u32 = 1u32 << 2;
pub const IORING_CQE_F_NOTIF: __u32 = 1u32 << 3;

pub const IORING_CQE_BUFFER_SHIFT: __u32 = 16;

/*
 * Magic offsets for the application to mmap the data it needs
 */
pub const IORING_OFF_SQ_RING: __u64 = 0u64;
pub const IORING_OFF_CQ_RING: __u64 = 0x8000000u64;
pub const IORING_OFF_SQES: __u64 = 0x10000000u64;
pub const IORING_OFF_PBUF_RING: __u64 = 0x80000000u64;
pub const IORING_OFF_PBUF_SHIFT: __u32 = 16;
pub const IORING_OFF_MMAP_MASK: __u64 = 0xf8000000u64;

/*
 * Filled with the offset for mmap(2)
 */
#[repr(C)]
#[derive(Copy, Clone)]
pub struct io_sqring_offsets {
    pub head: __u32,
    pub tail: __u32,
    pub ring_mask: __u32,
    pub ring_entries: __u32,
    pub flags: __u32,
    pub dropped: __u32,
    pub array: __u32,
    pub resv1: __u32,
    pub user_addr: __u64,
}

/*
 * sq_ring->flags
 */
pub const IORING_SQ_NEED_WAKEUP: __u32 = 1u32 << 0; /* needs io_uring_enter wakeup */
pub const IORING_SQ_CQ_OVERFLOW: __u32 = 1u32 << 1; /* CQ ring is overflown */
pub const IORING_SQ_TASKRUN: __u32 = 1u32 << 2; /* task should enter the kernel */

#[repr(C)]
#[derive(Copy, Clone)]
pub struct io_cqring_offsets {
    pub head: __u32,
    pub tail: __u32,
    pub ring_mask: __u32,
    pub ring_entries: __u32,
    pub overflow: __u32,
    pub cqes: __u32,
    pub flags: __u32,
    pub resv1: __u32,
    pub user_addr: __u64,
}

/*
 * cq_ring->flags
 */

/* disable eventfd notifications */
pub const IORING_CQ_EVENTFD_DISABLED: __u32 = 1u32 << 0;

/*
 * io_uring_enter(2) flags
 */
pub const IORING_ENTER_GETEVENTS: __u32 = 1u32 << 0;
pub const IORING_ENTER_SQ_WAKEUP: __u32 = 1u32 << 1;
pub const IORING_ENTER_SQ_WAIT: __u32 = 1u32 << 2;
pub const IORING_ENTER_EXT_ARG: __u32 = 1u32 << 3;
pub const IORING_ENTER_REGISTERED_RING: __u32 = 1u32 << 4;

/*
 * Passed in for io_uring_setup(2). Copied back with updated info on success
 */
#[repr(C)]
#[derive(Copy, Clone)]
pub struct io_uring_params {
    pub sq_entries: __u32,
    pub cq_entries: __u32,
    pub flags: __u32,
    pub sq_thread_cpu: __u32,
    pub sq_thread_idle: __u32,
    pub features: __u32,
    pub wq_fd: __u32,
    pub resv: [__u32; 3],
    pub sq_off: io_sqring_offsets,
    pub cq_off: io_cqring_offsets,
}

/*
 * io_uring_params->features flags
 */
pub const IORING_FEAT_SINGLE_MMAP: __u32 = 1u32 << 0;
pub const IORING_FEAT_NODROP: __u32 = 1u32 << 1;
pub const IORING_FEAT_SUBMIT_STABLE: __u32 = 1u32 << 2;
pub const IORING_FEAT_RW_CUR_POS: __u32 = 1u32 << 3;
pub const IORING_FEAT_CUR_PERSONALITY: __u32 = 1u32 << 4;
pub const IORING_FEAT_FAST_POLL: __u32 = 1u32 << 5;
pub const IORING_FEAT_POLL_32BITS: __u32 = 1u32 << 6;
pub const IORING_FEAT_SQPOLL_NONFIXED: __u32 = 1u32 << 7;
pub const IORING_FEAT_EXT_ARG: __u32 = 1u32 << 8;
pub const IORING_FEAT_NATIVE_WORKERS: __u32 = 1u32 << 9;
pub const IORING_FEAT_RSRC_TAGS: __u32 = 1u32 << 10;
pub const IORING_FEAT_CQE_SKIP: __u32 = 1u32 << 11;
pub const IORING_FEAT_LINKED_FILE: __u32 = 1u32 << 12;
pub const IORING_FEAT_REG_REG_RING: __u32 = 1u32 << 13;

/*
 * io_uring_register(2) opcodes and arguments
 */
pub const IORING_REGISTER_BUFFERS: __u32 = 0;
pub const IORING_UNREGISTER_BUFFERS: __u32 = 1;
pub const IORING_REGISTER_FILES: __u32 = 2;
pub const IORING_UNREGISTER_FILES: __u32 = 3;
pub const IORING_REGISTER_EVENTFD: __u32 = 4;
pub const IORING_UNREGISTER_EVENTFD: __u32 = 5;
pub const IORING_REGISTER_FILES_UPDATE: __u32 = 6;
pub const IORING_REGISTER_EVENTFD_ASYNC: __u32 = 7;
pub const IORING_REGISTER_PROBE: __u32 = 8;
pub const IORING_REGISTER_PERSONALITY: __u32 = 9;
pub const IORING_UNREGISTER_PERSONALITY: __u32 = 10;
pub const IORING_REGISTER_RESTRICTIONS: __u32 = 11;
pub const IORING_REGISTER_ENABLE_RINGS: __u32 = 12;

/* extended with tagging */
pub const IORING_REGISTER_FILES2: __u32 = 13;
pub const IORING_REGISTER_FILES_UPDATE2: __u32 = 14;
pub const IORING_REGISTER_BUFFERS2: __u32 = 15;
pub const IORING_REGISTER_BUFFERS_UPDATE: __u32 = 16;

/* set/clear io-wq thread affinities */
pub const IORING_REGISTER_IOWQ_AFF: __u32 = 17;
pub const IORING_UNREGISTER_IOWQ_AFF: __u32 = 18;

/* set/get max number of io-wq workers */
pub const IORING_REGISTER_IOWQ_MAX_WORKERS: __u32 = 19;

/* register/unregister io_uring fd with the ring */
pub const IORING_REGISTER_RING_FDS: __u32 = 20;
pub const IORING_UNREGISTER_RING_FDS: __u32 = 21;

/* register ring based provide buffer group */
pub const IORING_REGISTER_PBUF_RING: __u32 = 22;
pub const IORING_UNREGISTER_PBUF_RING: __u32 = 23;

/* sync cancelation API */
pub const IORING_REGISTER_SYNC_CANCEL: __u32 = 24;

/* register a range of fixed file slots for automatic slot allocation */
pub const IORING_REGISTER_FILE_ALLOC_RANGE: __u32 = 25;

/* this goes last */
pub const IORING_REGISTER_LAST: __u32 = 26;

/* flag added to the opcode to use a registered ring fd */
pub const IORING_REGISTER_USE_REGISTERED_RING: __u32 = 1u32 << 31;

/* io-wq worker categories */
pub const IO_WQ_BOUND: __u32 = 0;
pub const IO_WQ_UNBOUND: __u32 = 1;

/* deprecated, see struct io_uring_rsrc_update */
#[repr(C)]
#[derive(Copy, Clone)]
pub struct io_uring_files_update {
    pub offset: __u32,
    pub resv: __u32,
    pub fds: __aligned_u64, /* __s32 * */
}

/*
 * Register a fully sparse file space, rather than pass in an array of all
 * -1 file descriptors.
 */
pub const IORING_RSRC_REGISTER_SPARSE: __u32 = 1u32 << 0;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct io_uring_rsrc_register {
    pub nr: __u32,
    pub flags: __u32,
    pub resv2: __u64,
    pub data: __aligned_u64,
    pub tags: __aligned_u64,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct io_uring_rsrc_update {
    pub offset: __u32,
    pub resv: __u32,
    pub data: __aligned_u64,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct io_uring_rsrc_update2 {
    pub offset: __u32,
    pub resv: __u32,
    pub data: __aligned_u64,
    pub tags: __aligned_u64,
    pub nr: __u32,
    pub resv2: __u32,
}

/* Skip updating fd indexes set to this value in the fd table */
pub const IORING_REGISTER_FILES_SKIP: i32 = -2;

pub const IO_URING_OP_SUPPORTED: __u32 = 1u32 << 0;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct io_uring_probe_op {
    pub op: __u8,
    pub resv: __u8,
    pub flags: __u16, /* IO_URING_OP_* flags */
    pub resv2: __u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct io_uring_probe {
    pub last_op: __u8, /* last opcode supported */
    pub ops_len: __u8, /* length of ops[] array below */
    pub resv: __u16,
    pub resv2: [__u32; 3],
    pub ops: [io_uring_probe_op; 0],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub union io_uring_restriction_union {
    pub register_op: __u8, /* IORING_RESTRICTION_REGISTER_OP */
    pub sqe_op: __u8,      /* IORING_RESTRICTION_SQE_OP */
    pub sqe_flags: __u8,   /* IORING_RESTRICTION_SQE_FLAGS_* */
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct io_uring_restriction {
    pub opcode: __u16,
    pub restriction: io_uring_restriction_union,
    pub resv: __u8,
    pub resv2: [__u32; 3],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct io_uring_buf {
    pub addr: __u64,
    pub len: __u32,
    pub bid: __u16,
    pub resv: __u16,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct io_uring_buf_ring_tail {
    pub resv1: __u64,
    pub resv2: __u32,
    pub resv3: __u16,
    pub tail: __u16,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub union io_uring_buf_ring {
    /*
     * To avoid spilling into more pages than we need to, the
     * ring tail is overlaid with the io_uring_buf->resv field.
     */
    pub tail: io_uring_buf_ring_tail,
    pub bufs: [io_uring_buf; 0],
}

/*
 * Flags for IORING_REGISTER_PBUF_RING.
 *
 * IOU_PBUF_RING_MMAP:	If set, kernel will allocate the memory for the ring.
 *			The application must not set a ring_addr in struct
 *			io_uring_buf_reg, instead it must subsequently call
 *			mmap(2) with the offset set as:
 *			IORING_OFF_PBUF_RING | (bgid << IORING_OFF_PBUF_SHIFT)
 *			to get a virtual mapping for the ring.
 */
pub const IOU_PBUF_RING_MMAP: __u32 = 1;

/* argument for IORING_(UN)REGISTER_PBUF_RING */
#[repr(C)]
#[derive(Copy, Clone)]
pub struct io_uring_buf_reg {
    pub ring_addr: __u64,
    pub ring_entries: __u32,
    pub bgid: __u16,
    pub flags: __u16,
    pub resv: [__u64; 3],
}

/*
 * io_uring_restriction->opcode values
 */
/* Allow an io_uring_register(2) opcode */
pub const IORING_RESTRICTION_REGISTER_OP: __u32 = 0;

/* Allow an sqe opcode */
pub const IORING_RESTRICTION_SQE_OP: __u32 = 1;

/* Allow sqe flags */
pub const IORING_RESTRICTION_SQE_FLAGS_ALLOWED: __u32 = 2;

/* Require sqe flags (these flags must be set on each submission) */
pub const IORING_RESTRICTION_SQE_FLAGS_REQUIRED: __u32 = 3;

pub const IORING_RESTRICTION_LAST: __u32 = 4;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct io_uring_getevents_arg {
    pub sigmask: __u64,
    pub sigmask_sz: __u32,
    pub pad: __u32,
    pub ts: __u64,
}

/*
 * Argument for IORING_REGISTER_SYNC_CANCEL
 */
#[repr(C)]
#[derive(Copy, Clone)]
pub struct io_uring_sync_cancel_reg {
    pub addr: __u64,
    pub fd: __s32,
    pub flags: __u32,
    pub timeout: __kernel_timespec,
    pub opcode: __u8,
    pub pad: [__u8; 7],
    pub pad2: [__u64; 3],
}

/*
 * Argument for IORING_REGISTER_FILE_ALLOC_RANGE
 * The range is specified as [off, off + len)
 */
#[repr(C)]
#[derive(Copy, Clone)]
pub struct io_uring_file_index_range {
    pub off: __u32,
    pub len: __u32,
    pub resv: __u64,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct io_uring_recvmsg_out {
    pub namelen: __u32,
    pub controllen: __u32,
    pub payloadlen: __u32,
    pub flags: __u32,
}

/*
 * Argument for IORING_OP_URING_CMD when file is a socket
 */
pub const SOCKET_URING_OP_SIOCINQ: __u32 = 0;
pub const SOCKET_URING_OP_SIOCOUTQ: __u32 = 1;
pub const SOCKET_URING_OP_GETSOCKOPT: __u32 = 2;
pub const SOCKET_URING_OP_SETSOCKOPT: __u32 = 3;
