/* SPDX-License-Identifier: (GPL-2.0 WITH Linux-syscall-note) OR MIT */
/* Rust translation of linux/io_uring.h. External kernel types are supplied by dependencies. */

#[allow(non_camel_case_types, non_snake_case, non_upper_case_globals, dead_code)]
pub type __u8 = u8;
#[allow(non_camel_case_types)] pub type __u16 = u16;
#[allow(non_camel_case_types)] pub type __u32 = u32;
#[allow(non_camel_case_types)] pub type __u64 = u64;
#[allow(non_camel_case_types)] pub type __s32 = i32;

#[repr(C)]
pub struct io_uring_sqe {
    pub opcode: __u8, pub flags: __u8, pub ioprio: __u16, pub fd: __s32,
    pub off: __u64, pub addr: __u64, pub len: __u32, pub rw_flags: __u32,
    pub user_data: __u64, pub buf_index: __u16, pub personality: __u16,
    pub splice_fd_in: __s32, pub addr3: __u64, pub pad2: __u64,
}
#[repr(C)] pub struct io_uring_attr_pi { pub flags: __u16, pub app_tag: __u16, pub len: __u32, pub addr: __u64, pub seed: __u64, pub rsvd: __u64 }

pub const IORING_RW_ATTR_FLAG_PI: u32 = 1 << 0;
pub const IORING_FILE_INDEX_ALLOC: u32 = !0;
pub const IOSQE_FIXED_FILE_BIT: u32 = 0; pub const IOSQE_IO_DRAIN_BIT: u32 = 1; pub const IOSQE_IO_LINK_BIT: u32 = 2;
pub const IOSQE_IO_HARDLINK_BIT: u32 = 3; pub const IOSQE_ASYNC_BIT: u32 = 4; pub const IOSQE_BUFFER_SELECT_BIT: u32 = 5; pub const IOSQE_CQE_SKIP_SUCCESS_BIT: u32 = 6;
pub const IOSQE_FIXED_FILE: u32=1<<0; pub const IOSQE_IO_DRAIN:u32=1<<1; pub const IOSQE_IO_LINK:u32=1<<2; pub const IOSQE_IO_HARDLINK:u32=1<<3; pub const IOSQE_ASYNC:u32=1<<4; pub const IOSQE_BUFFER_SELECT:u32=1<<5; pub const IOSQE_CQE_SKIP_SUCCESS:u32=1<<6;

pub const IORING_SETUP_IOPOLL:u32=1<<0; pub const IORING_SETUP_SQPOLL:u32=1<<1; pub const IORING_SETUP_SQ_AFF:u32=1<<2; pub const IORING_SETUP_CQSIZE:u32=1<<3; pub const IORING_SETUP_CLAMP:u32=1<<4; pub const IORING_SETUP_ATTACH_WQ:u32=1<<5; pub const IORING_SETUP_R_DISABLED:u32=1<<6; pub const IORING_SETUP_SUBMIT_ALL:u32=1<<7; pub const IORING_SETUP_COOP_TASKRUN:u32=1<<8; pub const IORING_SETUP_TASKRUN_FLAG:u32=1<<9; pub const IORING_SETUP_SQE128:u32=1<<10; pub const IORING_SETUP_CQE32:u32=1<<11; pub const IORING_SETUP_SINGLE_ISSUER:u32=1<<12; pub const IORING_SETUP_DEFER_TASKRUN:u32=1<<13; pub const IORING_SETUP_NO_MMAP:u32=1<<14; pub const IORING_SETUP_REGISTERED_FD_ONLY:u32=1<<15; pub const IORING_SETUP_NO_SQARRAY:u32=1<<16; pub const IORING_SETUP_HYBRID_IOPOLL:u32=1<<17; pub const IORING_SETUP_CQE_MIXED:u32=1<<18; pub const IORING_SETUP_SQE_MIXED:u32=1<<19; pub const IORING_SETUP_SQ_REWIND:u32=1<<20;

#[repr(u32)] pub enum io_uring_op { IORING_OP_NOP, IORING_OP_READV, IORING_OP_WRITEV, IORING_OP_FSYNC, IORING_OP_READ_FIXED, IORING_OP_WRITE_FIXED, IORING_OP_POLL_ADD, IORING_OP_POLL_REMOVE, IORING_OP_SYNC_FILE_RANGE, IORING_OP_SENDMSG, IORING_OP_RECVMSG, IORING_OP_TIMEOUT, IORING_OP_TIMEOUT_REMOVE, IORING_OP_ACCEPT, IORING_OP_ASYNC_CANCEL, IORING_OP_LINK_TIMEOUT, IORING_OP_CONNECT, IORING_OP_FALLOCATE, IORING_OP_OPENAT, IORING_OP_CLOSE, IORING_OP_FILES_UPDATE, IORING_OP_STATX, IORING_OP_READ, IORING_OP_WRITE, IORING_OP_FADVISE, IORING_OP_MADVISE, IORING_OP_SEND, IORING_OP_RECV, IORING_OP_OPENAT2, IORING_OP_EPOLL_CTL, IORING_OP_SPLICE, IORING_OP_PROVIDE_BUFFERS, IORING_OP_REMOVE_BUFFERS, IORING_OP_TEE, IORING_OP_SHUTDOWN, IORING_OP_RENAMEAT, IORING_OP_UNLINKAT, IORING_OP_MKDIRAT, IORING_OP_SYMLINKAT, IORING_OP_LINKAT, IORING_OP_MSG_RING, IORING_OP_FSETXATTR, IORING_OP_SETXATTR, IORING_OP_FGETXATTR, IORING_OP_GETXATTR, IORING_OP_SOCKET, IORING_OP_URING_CMD, IORING_OP_SEND_ZC, IORING_OP_SENDMSG_ZC, IORING_OP_READ_MULTISHOT, IORING_OP_WAITID, IORING_OP_FUTEX_WAIT, IORING_OP_FUTEX_WAKE, IORING_OP_FUTEX_WAITV, IORING_OP_FIXED_FD_INSTALL, IORING_OP_FTRUNCATE, IORING_OP_BIND, IORING_OP_LISTEN, IORING_OP_RECV_ZC, IORING_OP_EPOLL_WAIT, IORING_OP_READV_FIXED, IORING_OP_WRITEV_FIXED, IORING_OP_PIPE, IORING_OP_NOP128, IORING_OP_URING_CMD128, IORING_OP_LAST }

pub const IORING_URING_CMD_FIXED:u32=1; pub const IORING_URING_CMD_MULTISHOT:u32=2; pub const IORING_URING_CMD_MASK:u32=3;
pub const IORING_FSYNC_DATASYNC:u32=1; pub const IORING_TIMEOUT_ABS:u32=1; pub const IORING_TIMEOUT_UPDATE:u32=2; pub const IORING_TIMEOUT_BOOTTIME:u32=4; pub const IORING_TIMEOUT_REALTIME:u32=8; pub const IORING_LINK_TIMEOUT_UPDATE:u32=16; pub const IORING_TIMEOUT_ETIME_SUCCESS:u32=32; pub const IORING_TIMEOUT_MULTISHOT:u32=64; pub const IORING_TIMEOUT_IMMEDIATE_ARG:u32=128; pub const IORING_TIMEOUT_CLOCK_MASK:u32=12; pub const IORING_TIMEOUT_UPDATE_MASK:u32=18;
pub const SPLICE_F_FD_IN_FIXED:u32=1<<31; pub const IORING_POLL_ADD_MULTI:u32=1; pub const IORING_POLL_UPDATE_EVENTS:u32=2; pub const IORING_POLL_UPDATE_USER_DATA:u32=4; pub const IORING_POLL_ADD_LEVEL:u32=8;
pub const IORING_ASYNC_CANCEL_ALL:u32=1; pub const IORING_ASYNC_CANCEL_FD:u32=2; pub const IORING_ASYNC_CANCEL_ANY:u32=4; pub const IORING_ASYNC_CANCEL_FD_FIXED:u32=8; pub const IORING_ASYNC_CANCEL_USERDATA:u32=16; pub const IORING_ASYNC_CANCEL_OP:u32=32;
pub const IORING_RECVSEND_POLL_FIRST:u32=1; pub const IORING_RECV_MULTISHOT:u32=2; pub const IORING_RECVSEND_FIXED_BUF:u32=4; pub const IORING_SEND_ZC_REPORT_USAGE:u32=8; pub const IORING_RECVSEND_BUNDLE:u32=16; pub const IORING_SEND_VECTORIZED:u32=32; pub const IORING_NOTIF_USAGE_ZC_COPIED:u32=1<<31; pub const IORING_ACCEPT_MULTISHOT:u32=1; pub const IORING_ACCEPT_DONTWAIT:u32=2; pub const IORING_ACCEPT_POLL_FIRST:u32=4;

#[repr(C)] pub struct io_uring_cqe { pub user_data:__u64, pub res:__s32, pub flags:__u32, pub big_cqe:[__u64;0] }
pub const IORING_CQE_F_BUFFER:u32=1; pub const IORING_CQE_F_MORE:u32=2; pub const IORING_CQE_F_SOCK_NONEMPTY:u32=4; pub const IORING_CQE_F_NOTIF:u32=8; pub const IORING_CQE_F_BUF_MORE:u32=16; pub const IORING_CQE_F_SKIP:u32=32; pub const IORING_CQE_F_32:u32=1<<15; pub const IORING_CQE_BUFFER_SHIFT:u32=16;
pub const IORING_OFF_SQ_RING:u64=0; pub const IORING_OFF_CQ_RING:u64=0x8000000; pub const IORING_OFF_SQES:u64=0x10000000; pub const IORING_OFF_PBUF_RING:u64=0x80000000; pub const IORING_OFF_PBUF_SHIFT:u32=16; pub const IORING_OFF_MMAP_MASK:u64=0xf8000000;
#[repr(C)] pub struct io_sqring_offsets { pub head:__u32,pub tail:__u32,pub ring_mask:__u32,pub ring_entries:__u32,pub flags:__u32,pub dropped:__u32,pub array:__u32,pub resv1:__u32,pub user_addr:__u64 }
pub const IORING_SQ_NEED_WAKEUP:u32=1; pub const IORING_SQ_CQ_OVERFLOW:u32=2; pub const IORING_SQ_TASKRUN:u32=4;
#[repr(C)] pub struct io_cqring_offsets { pub head:__u32,pub tail:__u32,pub ring_mask:__u32,pub ring_entries:__u32,pub overflow:__u32,pub cqes:__u32,pub flags:__u32,pub resv1:__u32,pub user_addr:__u64 }
pub const IORING_CQ_EVENTFD_DISABLED:u32=1; pub const IORING_ENTER_GETEVENTS:u32=1; pub const IORING_ENTER_SQ_WAKEUP:u32=2; pub const IORING_ENTER_SQ_WAIT:u32=4; pub const IORING_ENTER_EXT_ARG:u32=8; pub const IORING_ENTER_REGISTERED_RING:u32=16; pub const IORING_ENTER_ABS_TIMER:u32=32; pub const IORING_ENTER_EXT_ARG_REG:u32=64; pub const IORING_ENTER_NO_IOWAIT:u32=128;
#[repr(C)] pub struct io_uring_params { pub sq_entries:__u32,pub cq_entries:__u32,pub flags:__u32,pub sq_thread_cpu:__u32,pub sq_thread_idle:__u32,pub features:__u32,pub wq_fd:__u32,pub resv:[__u32;3],pub sq_off:io_sqring_offsets,pub cq_off:io_cqring_offsets }
pub const IORING_FEAT_SINGLE_MMAP:u32=1; pub const IORING_FEAT_NODROP:u32=2; pub const IORING_FEAT_SUBMIT_STABLE:u32=4; pub const IORING_FEAT_RW_CUR_POS:u32=8; pub const IORING_FEAT_CUR_PERSONALITY:u32=16; pub const IORING_FEAT_FAST_POLL:u32=32; pub const IORING_FEAT_POLL_32BITS:u32=64; pub const IORING_FEAT_SQPOLL_NONFIXED:u32=128; pub const IORING_FEAT_EXT_ARG:u32=256; pub const IORING_FEAT_NATIVE_WORKERS:u32=512; pub const IORING_FEAT_RSRC_TAGS:u32=1024; pub const IORING_FEAT_CQE_SKIP:u32=2048; pub const IORING_FEAT_LINKED_FILE:u32=4096; pub const IORING_FEAT_REG_REG_RING:u32=8192; pub const IORING_FEAT_RECVSEND_BUNDLE:u32=16384; pub const IORING_FEAT_MIN_TIMEOUT:u32=32768; pub const IORING_FEAT_RW_ATTR:u32=65536; pub const IORING_FEAT_NO_IOWAIT:u32=131072;

#[repr(u32)] pub enum io_uring_register_op { IORING_REGISTER_BUFFERS=0, IORING_UNREGISTER_BUFFERS, IORING_REGISTER_FILES, IORING_UNREGISTER_FILES, IORING_REGISTER_EVENTFD, IORING_UNREGISTER_EVENTFD, IORING_REGISTER_FILES_UPDATE, IORING_REGISTER_EVENTFD_ASYNC, IORING_REGISTER_PROBE, IORING_REGISTER_PERSONALITY, IORING_UNREGISTER_PERSONALITY, IORING_REGISTER_RESTRICTIONS, IORING_REGISTER_ENABLE_RINGS, IORING_REGISTER_FILES2, IORING_REGISTER_FILES_UPDATE2, IORING_REGISTER_BUFFERS2, IORING_REGISTER_BUFFERS_UPDATE, IORING_REGISTER_IOWQ_AFF, IORING_UNREGISTER_IOWQ_AFF, IORING_REGISTER_IOWQ_MAX_WORKERS, IORING_REGISTER_RING_FDS, IORING_UNREGISTER_RING_FDS, IORING_REGISTER_PBUF_RING, IORING_UNREGISTER_PBUF_RING, IORING_REGISTER_SYNC_CANCEL, IORING_REGISTER_FILE_ALLOC_RANGE, IORING_REGISTER_PBUF_STATUS, IORING_REGISTER_NAPI, IORING_UNREGISTER_NAPI, IORING_REGISTER_CLOCK, IORING_REGISTER_CLONE_BUFFERS, IORING_REGISTER_SEND_MSG_RING, IORING_REGISTER_ZCRX_IFQ, IORING_REGISTER_RESIZE_RINGS, IORING_REGISTER_MEM_REGION, IORING_REGISTER_QUERY, IORING_REGISTER_ZCRX_CTRL, IORING_REGISTER_BPF_FILTER, IORING_REGISTER_LAST, IORING_REGISTER_USE_REGISTERED_RING=1<<31 }
#[repr(C)] pub struct io_uring_files_update { pub offset:__u32,pub resv:__u32,pub fds:__u64 }
pub const IORING_MEM_REGION_TYPE_USER:u32=1; #[repr(C)] pub struct io_uring_region_desc { pub user_addr:__u64,pub size:__u64,pub flags:__u32,pub id:__u32,pub mmap_offset:__u64,pub __resv:[__u64;4] }
pub const IORING_MEM_REGION_REG_WAIT_ARG:u32=1; #[repr(C)] pub struct io_uring_mem_region_reg { pub region_uptr:__u64,pub flags:__u64,pub __resv:[__u64;2] }
pub const IORING_RSRC_REGISTER_SPARSE:u32=1; #[repr(C)] pub struct io_uring_rsrc_register { pub nr:__u32,pub flags:__u32,pub resv2:__u64,pub data:__u64,pub tags:__u64 }
#[repr(C)] pub struct io_uring_rsrc_update { pub offset:__u32,pub resv:__u32,pub data:__u64 }
#[repr(C)] pub struct io_uring_rsrc_update2 { pub offset:__u32,pub resv:__u32,pub data:__u64,pub tags:__u64,pub nr:__u32,pub resv2:__u32 }
pub const IORING_REGISTER_FILES_SKIP:i32=-2; pub const IO_URING_OP_SUPPORTED:u32=1;
#[repr(C)] pub struct io_uring_probe_op { pub op:__u8,pub resv:__u8,pub flags:__u16,pub resv2:__u32 }
#[repr(C)] pub struct io_uring_probe { pub last_op:__u8,pub ops_len:__u8,pub resv:__u16,pub resv2:[__u32;3],pub ops:[io_uring_probe_op;0] }
#[repr(C)] pub struct io_uring_restriction { pub opcode:__u16,pub register_op:__u8,pub resv:__u8,pub resv2:[__u32;3] }
#[repr(C)] pub struct io_uring_task_restriction { pub flags:__u16,pub nr_res:__u16,pub resv:[__u32;3],pub restrictions:[io_uring_restriction;0] }
#[repr(C)] pub struct io_uring_clock_register { pub clockid:__u32,pub __resv:[__u32;3] }
pub const IORING_REGISTER_SRC_REGISTERED:u32=1; pub const IORING_REGISTER_DST_REPLACE:u32=2;
#[repr(C)] pub struct io_uring_clone_buffers { pub src_fd:__u32,pub flags:__u32,pub src_off:__u32,pub dst_off:__u32,pub nr:__u32,pub pad:[__u32;3] }
#[repr(C)] pub struct io_uring_buf { pub addr:__u64,pub len:__u32,pub bid:__u16,pub resv:__u16 }
#[repr(C)] pub struct io_uring_buf_ring { pub resv1:__u64,pub resv2:__u32,pub resv3:__u16,pub tail:__u16,pub bufs:[io_uring_buf;0] }
pub const IOU_PBUF_RING_MMAP:u32=1; pub const IOU_PBUF_RING_INC:u32=2;
#[repr(C)] pub struct io_uring_buf_reg { pub ring_addr:__u64,pub ring_entries:__u32,pub bgid:__u16,pub flags:__u16,pub min_left:__u32,pub resv:[__u32;5] }
#[repr(C)] pub struct io_uring_buf_status { pub buf_group:__u32,pub head:__u32,pub resv:[__u32;8] }
#[repr(u32)] pub enum io_uring_napi_op { IO_URING_NAPI_REGISTER_OP=0, IO_URING_NAPI_STATIC_ADD_ID=1, IO_URING_NAPI_STATIC_DEL_ID=2 }
#[repr(u32)] pub enum io_uring_napi_tracking_strategy { IO_URING_NAPI_TRACKING_DYNAMIC=0, IO_URING_NAPI_TRACKING_STATIC=1, IO_URING_NAPI_TRACKING_INACTIVE=255 }
#[repr(C)] pub struct io_uring_napi { pub busy_poll_to:__u32,pub prefer_busy_poll:__u8,pub opcode:__u8,pub pad:[__u8;2],pub op_param:__u32,pub resv:__u32 }
pub const IORING_REG_WAIT_TS:u32=1;
#[repr(C)] pub struct io_uring_getevents_arg { pub sigmask:__u64,pub sigmask_sz:__u32,pub min_wait_usec:__u32,pub ts:__u64 }
#[repr(C)] pub struct io_uring_file_index_range { pub off:__u32,pub len:__u32,pub resv:__u64 }
#[repr(C)] pub struct io_uring_recvmsg_out { pub namelen:__u32,pub controllen:__u32,pub payloadlen:__u32,pub flags:__u32 }
#[repr(u32)] pub enum io_uring_socket_op { SOCKET_URING_OP_SIOCINQ=0, SOCKET_URING_OP_SIOCOUTQ, SOCKET_URING_OP_GETSOCKOPT, SOCKET_URING_OP_SETSOCKOPT, SOCKET_URING_OP_TX_TIMESTAMP, SOCKET_URING_OP_GETSOCKNAME }
pub const IORING_TIMESTAMP_HW_SHIFT:u32=16; pub const IORING_TIMESTAMP_TYPE_SHIFT:u32=17; pub const IORING_CQE_F_TSTAMP_HW:u32=1<<16;
#[repr(C)] pub struct io_timespec { pub tv_sec:__u64,pub tv_nsec:__u64 }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
