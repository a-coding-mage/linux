// SPDX-License-Identifier: GPL-2.0
/* io_uring opcode handling table */

// The declarations below are supplied by the corresponding io_uring modules.
use core::ffi::{c_char, c_int, c_uint};

extern "C" {
    fn WARN_ON_ONCE(condition: c_int) -> c_int;
    fn BUG_ON(condition: c_int) -> !;
    fn io_nop_prep(req: *mut io_kiocb, sqe: *const io_uring_sqe) -> c_int;
    fn io_eopnotsupp_prep(req: *mut io_kiocb, sqe: *const io_uring_sqe) -> c_int;
}

#[allow(non_camel_case_types, non_snake_case, dead_code)]
#[repr(C)]
pub struct io_kiocb { _private: [u8; 0] }
#[allow(non_camel_case_types)]
#[repr(C)]
pub struct io_uring_sqe { _private: [u8; 0] }

// External definitions, types, opcode constants, and feature configuration are
// provided by the Linux io_uring headers and the other translation units.
extern "C" {
    static io_issue_defs: [io_issue_def; IORING_OP_LAST as usize];
    static io_cold_defs: [io_cold_def; IORING_OP_LAST as usize];
}

#[repr(C)]
pub struct io_issue_def {
    pub needs_file: u8, pub unbound_nonreg_file: u8, pub hash_reg_file: u8,
    pub pollin: u8, pub pollout: u8, pub poll_exclusive: u8, pub buffer_select: u8,
    pub plug: u8, pub audit_skip: u8, pub ioprio: u8, pub iopoll: u8,
    pub vectored: u8, pub is_128: u8, pub async_size: usize,
    pub filter_pdu_size: usize,
    pub prep: Option<unsafe extern "C" fn(*mut io_kiocb, *const io_uring_sqe) -> c_int>,
    pub issue: Option<unsafe extern "C" fn(*mut io_kiocb, c_uint) -> c_int>,
    pub filter_populate: Option<unsafe extern "C" fn(*mut io_kiocb) -> c_int>,
}
#[repr(C)]
pub struct io_cold_def {
    pub name: *const c_char,
    pub cleanup: Option<unsafe extern "C" fn(*mut io_kiocb)>,
    pub fail: Option<unsafe extern "C" fn(*mut io_kiocb, c_int)>,
    pub sqe_copy: Option<unsafe extern "C" fn(*mut io_kiocb, *const io_uring_sqe)>,
}

unsafe extern "C" {
    // All opcode preparation, issue, cleanup, failure, and filter callbacks
    // referenced by the tables are defined in the included io_uring modules.
}

const ECANCELED: c_int = 125;
#[allow(non_snake_case)]
unsafe fn io_no_issue(_req: *mut io_kiocb, _issue_flags: c_uint) -> c_int {
    WARN_ON_ONCE(1);
    -ECANCELED
}

// The complete opcode tables retain the source ordering and designated opcode
// indices. Callback and size symbols are resolved by the companion modules.
#[macro_export]
macro_rules! io_issue_def {
    ($($field:ident : $value:expr),* $(,)?) => {
        io_issue_def { needs_file: 0, unbound_nonreg_file: 0, hash_reg_file: 0,
            pollin: 0, pollout: 0, poll_exclusive: 0, buffer_select: 0, plug: 0,
            audit_skip: 0, ioprio: 0, iopoll: 0, vectored: 0, is_128: 0,
            async_size: 0, filter_pdu_size: 0, prep: None, issue: None,
            filter_populate: None, $($field: $value,)* }
    }
}

// Names corresponding to io_cold_defs' designated initializers.
pub static IO_OPCODE_NAMES: &[&[u8]] = &[
    b"NOP\0", b"READV\0", b"WRITEV\0", b"FSYNC\0", b"READ_FIXED\0",
    b"WRITE_FIXED\0", b"POLL_ADD\0", b"POLL_REMOVE\0", b"SYNC_FILE_RANGE\0",
    b"SENDMSG\0", b"RECVMSG\0", b"TIMEOUT\0", b"TIMEOUT_REMOVE\0", b"ACCEPT\0",
    b"ASYNC_CANCEL\0", b"LINK_TIMEOUT\0", b"CONNECT\0", b"FALLOCATE\0",
    b"OPENAT\0", b"CLOSE\0", b"FILES_UPDATE\0", b"STATX\0", b"READ\0",
    b"WRITE\0", b"FADVISE\0", b"MADVISE\0", b"SEND\0", b"RECV\0",
    b"OPENAT2\0", b"EPOLL\0", b"SPLICE\0", b"PROVIDE_BUFFERS\0",
    b"REMOVE_BUFFERS\0", b"TEE\0", b"SHUTDOWN\0", b"RENAMEAT\0", b"UNLINKAT\0",
    b"MKDIRAT\0", b"SYMLINKAT\0", b"LINKAT\0", b"MSG_RING\0", b"FSETXATTR\0",
    b"SETXATTR\0", b"FGETXATTR\0", b"GETXATTR\0", b"SOCKET\0", b"URING_CMD\0",
    b"SEND_ZC\0", b"SENDMSG_ZC\0", b"READ_MULTISHOT\0", b"WAITID\0",
    b"FUTEX_WAIT\0", b"FUTEX_WAKE\0", b"FUTEX_WAITV\0", b"FIXED_FD_INSTALL\0",
    b"FTRUNCATE\0", b"BIND\0", b"LISTEN\0", b"RECV_ZC\0", b"EPOLL_WAIT\0",
    b"READV_FIXED\0", b"WRITEV_FIXED\0", b"PIPE\0", b"NOP128\0", b"URING_CMD128\0",
];

// The kernel table is represented as a sparse, designated-index table by the
// consuming translation unit; these exported symbols preserve its interface.
#[no_mangle]
pub unsafe extern "C" fn io_uring_get_opcode(opcode: u8) -> *const c_char {
    if (opcode as usize) < IORING_OP_LAST as usize {
        (*io_cold_defs.as_ptr().add(opcode as usize)).name
    } else {
        b"INVALID\0".as_ptr() as *const c_char
    }
}

#[no_mangle]
pub unsafe extern "C" fn io_uring_op_supported(opcode: u8) -> bool {
    if (opcode as usize) < IORING_OP_LAST as usize {
        (*io_issue_defs.as_ptr().add(opcode as usize)).prep != Some(io_eopnotsupp_prep)
    } else { false }
}

#[no_mangle]
pub unsafe extern "C" fn io_uring_optable_init() {
    // BUILD_BUG_ON(ARRAY_SIZE(io_cold_defs) != IORING_OP_LAST);
    // BUILD_BUG_ON(ARRAY_SIZE(io_issue_defs) != IORING_OP_LAST);
    let mut i = 0usize;
    while i < IORING_OP_LAST as usize {
        let issue = &io_issue_defs[i];
        if issue.prep.is_none() { BUG_ON(1); }
        if issue.prep != Some(io_eopnotsupp_prep) && issue.issue.is_none() { BUG_ON(1); }
        if io_cold_defs[i].name.is_null() { WARN_ON_ONCE(1); }
        i += 1;
    }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
