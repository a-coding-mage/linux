// SPDX-License-Identifier: GPL-2.0

// The C header guard and include dependencies are omitted in Rust.

#[repr(C)]
pub struct io_uring_bpf_ctx {
    _private: [u8; 0],
}

// Opaque types supplied by other headers.
#[repr(C)]
pub struct io_kiocb {
    _private: [u8; 0],
}

#[repr(C)]
pub struct io_uring_sqe {
    _private: [u8; 0],
}

#[repr(C)]
pub struct io_issue_def {
    // These fields are C unsigned one-bit bit-fields. Rust has no native
    // bit-field syntax, so they are represented by their containing word.
    pub needs_file: u32,
    pub plug: u32,
    pub ioprio: u32,
    pub iopoll: u32,
    pub buffer_select: u32,
    pub hash_reg_file: u32,
    pub unbound_nonreg_file: u32,
    pub pollin: u32,
    pub pollout: u32,
    pub poll_exclusive: u32,
    pub audit_skip: u32,
    pub vectored: u32,
    pub is_128: u32,

    pub async_size: u16,
    pub filter_pdu_size: u16,

    pub issue: Option<unsafe extern "C" fn(*mut io_kiocb, u32) -> i32>,
    pub prep: Option<unsafe extern "C" fn(*mut io_kiocb, *const io_uring_sqe) -> i32>,
    pub filter_populate:
        Option<unsafe extern "C" fn(*mut io_uring_bpf_ctx, *mut io_kiocb)>,
}

#[repr(C)]
pub struct io_cold_def {
    pub name: *const core::ffi::c_char,

    pub sqe_copy: Option<unsafe extern "C" fn(*mut io_kiocb)>,
    pub cleanup: Option<unsafe extern "C" fn(*mut io_kiocb)>,
    pub fail: Option<unsafe extern "C" fn(*mut io_kiocb)>,
}

unsafe extern "C" {
    pub static io_issue_defs: [io_issue_def; 0];
    pub static io_cold_defs: [io_cold_def; 0];

    pub fn io_uring_op_supported(opcode: u8) -> bool;

    pub fn io_uring_optable_init();
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
