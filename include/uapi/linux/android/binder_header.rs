/* SPDX-License-Identifier: GPL-2.0 WITH Linux-syscall-note */
/*
 * Copyright (C) 2008 Google, Inc.
 *
 * Based on, but no longer compatible with, the original OpenBinder.org binder
 * driver interface.
 */

// Dependencies corresponding to <linux/types.h> and <linux/ioctl.h> are
// supplied by the surrounding translation unit.

pub const B_TYPE_LARGE: u32 = 0x85;
pub const BINDER_TYPE_BINDER: u32 = (((b's' as u32) << 24) | ((b'b' as u32) << 16) | ((b'*' as u32) << 8) | B_TYPE_LARGE);
pub const BINDER_TYPE_WEAK_BINDER: u32 = (((b'w' as u32) << 24) | ((b'b' as u32) << 16) | ((b'*' as u32) << 8) | B_TYPE_LARGE);
pub const BINDER_TYPE_HANDLE: u32 = (((b's' as u32) << 24) | ((b'h' as u32) << 16) | ((b'*' as u32) << 8) | B_TYPE_LARGE);
pub const BINDER_TYPE_WEAK_HANDLE: u32 = (((b'w' as u32) << 24) | ((b'h' as u32) << 16) | ((b'*' as u32) << 8) | B_TYPE_LARGE);
pub const BINDER_TYPE_FD: u32 = (((b'f' as u32) << 24) | ((b'd' as u32) << 16) | ((b'*' as u32) << 8) | B_TYPE_LARGE);
pub const BINDER_TYPE_FDA: u32 = (((b'f' as u32) << 24) | ((b'd' as u32) << 16) | ((b'a' as u32) << 8) | B_TYPE_LARGE);
pub const BINDER_TYPE_PTR: u32 = (((b'p' as u32) << 24) | ((b't' as u32) << 16) | ((b'*' as u32) << 8) | B_TYPE_LARGE);

pub const FLAT_BINDER_FLAG_PRIORITY_MASK: u32 = 0xff;
pub const FLAT_BINDER_FLAG_ACCEPTS_FDS: u32 = 0x100;
/// Request security contexts.
pub const FLAT_BINDER_FLAG_TXN_SECURITY_CTX: u32 = 0x1000;

#[cfg(BINDER_IPC_32BIT)]
pub type binder_size_t = __u32;
#[cfg(BINDER_IPC_32BIT)]
pub type binder_uintptr_t = __u32;
#[cfg(not(BINDER_IPC_32BIT))]
pub type binder_size_t = __u64;
#[cfg(not(BINDER_IPC_32BIT))]
pub type binder_uintptr_t = __u64;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct binder_object_header { pub type_: __u32 }

#[repr(C)]
#[derive(Copy, Clone)]
pub union flat_binder_object__bindgen_ty_1 { pub binder: binder_uintptr_t, pub handle: __u32 }
#[repr(C)]
#[derive(Copy, Clone)]
pub struct flat_binder_object { pub hdr: binder_object_header, pub flags: __u32, pub data: flat_binder_object__bindgen_ty_1, pub cookie: binder_uintptr_t }

#[repr(C)]
#[derive(Copy, Clone)]
pub union binder_fd_object__bindgen_ty_1 { pub pad_binder: binder_uintptr_t, pub fd: __u32 }
#[repr(C)]
#[derive(Copy, Clone)]
pub struct binder_fd_object { pub hdr: binder_object_header, pub pad_flags: __u32, pub data: binder_fd_object__bindgen_ty_1, pub cookie: binder_uintptr_t }

#[repr(C)]
#[derive(Copy, Clone)]
pub struct binder_buffer_object { pub hdr: binder_object_header, pub flags: __u32, pub buffer: binder_uintptr_t, pub length: binder_size_t, pub parent: binder_size_t, pub parent_offset: binder_size_t }
pub const BINDER_BUFFER_FLAG_HAS_PARENT: u32 = 0x01;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct binder_fd_array_object { pub hdr: binder_object_header, pub pad: __u32, pub num_fds: binder_size_t, pub parent: binder_size_t, pub parent_offset: binder_size_t }

#[repr(C)]
#[derive(Copy, Clone)]
pub struct binder_write_read { pub write_size: binder_size_t, pub write_consumed: binder_size_t, pub write_buffer: binder_uintptr_t, pub read_size: binder_size_t, pub read_consumed: binder_size_t, pub read_buffer: binder_uintptr_t }
#[repr(C)]
#[derive(Copy, Clone)]
pub struct binder_version { pub protocol_version: __s32 }
#[cfg(BINDER_IPC_32BIT)] pub const BINDER_CURRENT_PROTOCOL_VERSION: u32 = 7;
#[cfg(not(BINDER_IPC_32BIT))] pub const BINDER_CURRENT_PROTOCOL_VERSION: u32 = 8;
#[repr(C)] #[derive(Copy, Clone)] pub struct binder_node_debug_info { pub ptr: binder_uintptr_t, pub cookie: binder_uintptr_t, pub has_strong_ref: __u32, pub has_weak_ref: __u32 }
#[repr(C)] #[derive(Copy, Clone)] pub struct binder_node_info_for_ref { pub handle: __u32, pub strong_count: __u32, pub weak_count: __u32, pub reserved1: __u32, pub reserved2: __u32, pub reserved3: __u32 }
#[repr(C)] #[derive(Copy, Clone)] pub struct binder_freeze_info { pub pid: __u32, pub enable: __u32, pub timeout_ms: __u32 }
#[repr(C)] #[derive(Copy, Clone)] pub struct binder_frozen_status_info { pub pid: __u32, pub sync_recv: __u32, pub async_recv: __u32 }
#[repr(C)] #[derive(Copy, Clone)] pub struct binder_frozen_state_info { pub cookie: binder_uintptr_t, pub is_frozen: __u32, pub reserved: __u32 }
#[repr(C)] #[derive(Copy, Clone)] pub struct binder_extended_error { pub id: __u32, pub command: __u32, pub param: __s32 }

pub const BINDER_WRITE_READ: _ = _IOWR(b'b', 1, binder_write_read);
pub const BINDER_SET_IDLE_TIMEOUT: _ = _IOW(b'b', 3, __s64);
pub const BINDER_SET_MAX_THREADS: _ = _IOW(b'b', 5, __u32);
pub const BINDER_SET_IDLE_PRIORITY: _ = _IOW(b'b', 6, __s32);
pub const BINDER_SET_CONTEXT_MGR: _ = _IOW(b'b', 7, __s32);
pub const BINDER_THREAD_EXIT: _ = _IOW(b'b', 8, __s32);
pub const BINDER_VERSION: _ = _IOWR(b'b', 9, binder_version);
pub const BINDER_GET_NODE_DEBUG_INFO: _ = _IOWR(b'b', 11, binder_node_debug_info);
pub const BINDER_GET_NODE_INFO_FOR_REF: _ = _IOWR(b'b', 12, binder_node_info_for_ref);
pub const BINDER_SET_CONTEXT_MGR_EXT: _ = _IOW(b'b', 13, flat_binder_object);
pub const BINDER_FREEZE: _ = _IOW(b'b', 14, binder_freeze_info);
pub const BINDER_GET_FROZEN_INFO: _ = _IOWR(b'b', 15, binder_frozen_status_info);
pub const BINDER_ENABLE_ONEWAY_SPAM_DETECTION: _ = _IOW(b'b', 16, __u32);
pub const BINDER_GET_EXTENDED_ERROR: _ = _IOWR(b'b', 17, binder_extended_error);

pub const TF_ONE_WAY: u32 = 0x01; pub const TF_ROOT_OBJECT: u32 = 0x04; pub const TF_STATUS_CODE: u32 = 0x08; pub const TF_ACCEPT_FDS: u32 = 0x10; pub const TF_CLEAR_BUF: u32 = 0x20; pub const TF_UPDATE_TXN: u32 = 0x40;
#[repr(C)] #[derive(Copy, Clone)] pub union binder_transaction_data__target { pub handle: __u32, pub ptr: binder_uintptr_t }
#[repr(C)] #[derive(Copy, Clone)] pub union binder_transaction_data__data__ptr_or_buf { pub ptr: binder_transaction_data__data__ptr, pub buf: [__u8; 8] }
#[repr(C)] #[derive(Copy, Clone)] pub struct binder_transaction_data__data__ptr { pub buffer: binder_uintptr_t, pub offsets: binder_uintptr_t }
#[repr(C)] #[derive(Copy, Clone)] pub struct binder_transaction_data { pub target: binder_transaction_data__target, pub cookie: binder_uintptr_t, pub code: __u32, pub flags: __u32, pub sender_pid: __kernel_pid_t, pub sender_euid: __kernel_uid32_t, pub data_size: binder_size_t, pub offsets_size: binder_size_t, pub data: binder_transaction_data__data__ptr_or_buf }
#[repr(C)] #[derive(Copy, Clone)] pub struct binder_transaction_data_secctx { pub transaction_data: binder_transaction_data, pub secctx: binder_uintptr_t }
#[repr(C)] #[derive(Copy, Clone)] pub struct binder_transaction_data_sg { pub transaction_data: binder_transaction_data, pub buffers_size: binder_size_t }
#[repr(C)] #[derive(Copy, Clone)] pub struct binder_ptr_cookie { pub ptr: binder_uintptr_t, pub cookie: binder_uintptr_t }
#[repr(C, packed)] #[derive(Copy, Clone)] pub struct binder_handle_cookie { pub handle: __u32, pub cookie: binder_uintptr_t }
#[repr(C)] #[derive(Copy, Clone)] pub struct binder_pri_desc { pub priority: __s32, pub desc: __u32 }
#[repr(C)] #[derive(Copy, Clone)] pub struct binder_pri_ptr_cookie { pub priority: __s32, pub ptr: binder_uintptr_t, pub cookie: binder_uintptr_t }

pub const BR_ERROR: _ = _IOR(b'r', 0, __s32); pub const BR_OK: _ = _IO(b'r', 1); pub const BR_TRANSACTION_SEC_CTX: _ = _IOR(b'r', 2, binder_transaction_data_secctx); pub const BR_TRANSACTION: _ = _IOR(b'r', 2, binder_transaction_data); pub const BR_REPLY: _ = _IOR(b'r', 3, binder_transaction_data); pub const BR_ACQUIRE_RESULT: _ = _IOR(b'r', 4, __s32); pub const BR_DEAD_REPLY: _ = _IO(b'r', 5); pub const BR_TRANSACTION_COMPLETE: _ = _IO(b'r', 6); pub const BR_INCREFS: _ = _IOR(b'r', 7, binder_ptr_cookie); pub const BR_ACQUIRE: _ = _IOR(b'r', 8, binder_ptr_cookie); pub const BR_RELEASE: _ = _IOR(b'r', 9, binder_ptr_cookie); pub const BR_DECREFS: _ = _IOR(b'r', 10, binder_ptr_cookie); pub const BR_ATTEMPT_ACQUIRE: _ = _IOR(b'r', 11, binder_pri_ptr_cookie); pub const BR_NOOP: _ = _IO(b'r', 12); pub const BR_SPAWN_LOOPER: _ = _IO(b'r', 13); pub const BR_FINISHED: _ = _IO(b'r', 14); pub const BR_DEAD_BINDER: _ = _IOR(b'r', 15, binder_uintptr_t); pub const BR_CLEAR_DEATH_NOTIFICATION_DONE: _ = _IOR(b'r', 16, binder_uintptr_t); pub const BR_FAILED_REPLY: _ = _IO(b'r', 17); pub const BR_FROZEN_REPLY: _ = _IO(b'r', 18); pub const BR_ONEWAY_SPAM_SUSPECT: _ = _IO(b'r', 19); pub const BR_TRANSACTION_PENDING_FROZEN: _ = _IO(b'r', 20); pub const BR_FROZEN_BINDER: _ = _IOR(b'r', 21, binder_frozen_state_info); pub const BR_CLEAR_FREEZE_NOTIFICATION_DONE: _ = _IOR(b'r', 22, binder_uintptr_t);

pub const BC_TRANSACTION: _ = _IOW(b'c', 0, binder_transaction_data); pub const BC_REPLY: _ = _IOW(b'c', 1, binder_transaction_data); pub const BC_ACQUIRE_RESULT: _ = _IOW(b'c', 2, __s32); pub const BC_FREE_BUFFER: _ = _IOW(b'c', 3, binder_uintptr_t); pub const BC_INCREFS: _ = _IOW(b'c', 4, __u32); pub const BC_ACQUIRE: _ = _IOW(b'c', 5, __u32); pub const BC_RELEASE: _ = _IOW(b'c', 6, __u32); pub const BC_DECREFS: _ = _IOW(b'c', 7, __u32); pub const BC_INCREFS_DONE: _ = _IOW(b'c', 8, binder_ptr_cookie); pub const BC_ACQUIRE_DONE: _ = _IOW(b'c', 9, binder_ptr_cookie); pub const BC_ATTEMPT_ACQUIRE: _ = _IOW(b'c', 10, binder_pri_desc); pub const BC_REGISTER_LOOPER: _ = _IO(b'c', 11); pub const BC_ENTER_LOOPER: _ = _IO(b'c', 12); pub const BC_EXIT_LOOPER: _ = _IO(b'c', 13); pub const BC_REQUEST_DEATH_NOTIFICATION: _ = _IOW(b'c', 14, binder_handle_cookie); pub const BC_CLEAR_DEATH_NOTIFICATION: _ = _IOW(b'c', 15, binder_handle_cookie); pub const BC_DEAD_BINDER_DONE: _ = _IOW(b'c', 16, binder_uintptr_t); pub const BC_TRANSACTION_SG: _ = _IOW(b'c', 17, binder_transaction_data_sg); pub const BC_REPLY_SG: _ = _IOW(b'c', 18, binder_transaction_data_sg); pub const BC_REQUEST_FREEZE_NOTIFICATION: _ = _IOW(b'c', 19, binder_handle_cookie); pub const BC_CLEAR_FREEZE_NOTIFICATION: _ = _IOW(b'c', 20, binder_handle_cookie); pub const BC_FREEZE_NOTIFICATION_DONE: _ = _IOW(b'c', 21, binder_uintptr_t);

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
