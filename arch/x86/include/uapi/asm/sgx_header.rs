/* SPDX-License-Identifier: GPL-2.0 WITH Linux-syscall-note */
/*
 * Copyright(c) 2016-20 Intel Corporation.
 */

// The original header includes <linux/types.h> and <linux/ioctl.h>.

#[repr(i32)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum sgx_page_flags {
    SGX_PAGE_MEASURE = 0x01,
}

pub const SGX_MAGIC: u32 = 0xA4;

// Linux _IOC encoding, corresponding to the ioctl macros used by the header.
const _IOC_NRBITS: u32 = 8;
const _IOC_TYPEBITS: u32 = 8;
const _IOC_SIZEBITS: u32 = 14;
const _IOC_NRSHIFT: u32 = 0;
const _IOC_TYPESHIFT: u32 = _IOC_NRSHIFT + _IOC_NRBITS;
const _IOC_SIZESHIFT: u32 = _IOC_TYPESHIFT + _IOC_TYPEBITS;
const _IOC_DIRSHIFT: u32 = _IOC_SIZESHIFT + _IOC_SIZEBITS;
const _IOC_NONE: u32 = 0;
const _IOC_WRITE: u32 = 1;
const _IOC_READ: u32 = 2;

const fn _IOC(dir: u32, ty: u32, nr: u32, size: usize) -> u32 {
    (dir << _IOC_DIRSHIFT)
        | (ty << _IOC_TYPESHIFT)
        | (nr << _IOC_NRSHIFT)
        | ((size as u32) << _IOC_SIZESHIFT)
}

pub const SGX_IOC_ENCLAVE_CREATE: u32 = _IOC(
    _IOC_WRITE, SGX_MAGIC, 0x00, core::mem::size_of::<sgx_enclave_create>()
);
pub const SGX_IOC_ENCLAVE_ADD_PAGES: u32 = _IOC(
    _IOC_READ | _IOC_WRITE, SGX_MAGIC, 0x01, core::mem::size_of::<sgx_enclave_add_pages>()
);
pub const SGX_IOC_ENCLAVE_INIT: u32 = _IOC(
    _IOC_WRITE, SGX_MAGIC, 0x02, core::mem::size_of::<sgx_enclave_init>()
);
pub const SGX_IOC_ENCLAVE_PROVISION: u32 = _IOC(
    _IOC_WRITE, SGX_MAGIC, 0x03, core::mem::size_of::<sgx_enclave_provision>()
);
pub const SGX_IOC_VEPC_REMOVE_ALL: u32 = _IOC(_IOC_NONE, SGX_MAGIC, 0x04, 0);
pub const SGX_IOC_ENCLAVE_RESTRICT_PERMISSIONS: u32 = _IOC(
    _IOC_READ | _IOC_WRITE, SGX_MAGIC, 0x05,
    core::mem::size_of::<sgx_enclave_restrict_permissions>()
);
pub const SGX_IOC_ENCLAVE_MODIFY_TYPES: u32 = _IOC(
    _IOC_READ | _IOC_WRITE, SGX_MAGIC, 0x06,
    core::mem::size_of::<sgx_enclave_modify_types>()
);
pub const SGX_IOC_ENCLAVE_REMOVE_PAGES: u32 = _IOC(
    _IOC_READ | _IOC_WRITE, SGX_MAGIC, 0x07,
    core::mem::size_of::<sgx_enclave_remove_pages>()
);

#[repr(C)]
pub struct sgx_enclave_create {
    pub src: u64,
}

#[repr(C)]
pub struct sgx_enclave_add_pages {
    pub src: u64,
    pub offset: u64,
    pub length: u64,
    pub secinfo: u64,
    pub flags: u64,
    pub count: u64,
}

#[repr(C)]
pub struct sgx_enclave_init {
    pub sigstruct: u64,
}

#[repr(C)]
pub struct sgx_enclave_provision {
    pub fd: u64,
}

#[repr(C)]
pub struct sgx_enclave_restrict_permissions {
    pub offset: u64,
    pub length: u64,
    pub permissions: u64,
    pub result: u64,
    pub count: u64,
}

#[repr(C)]
pub struct sgx_enclave_modify_types {
    pub offset: u64,
    pub length: u64,
    pub page_type: u64,
    pub result: u64,
    pub count: u64,
}

#[repr(C)]
pub struct sgx_enclave_remove_pages {
    pub offset: u64,
    pub length: u64,
    pub count: u64,
}

pub type sgx_enclave_user_handler_t = unsafe extern "C" fn(
    rdi: libc::c_long,
    rsi: libc::c_long,
    rdx: libc::c_long,
    rsp: libc::c_long,
    r8: libc::c_long,
    r9: libc::c_long,
    run: *mut sgx_enclave_run,
) -> libc::c_int;

#[repr(C)]
pub struct sgx_enclave_run {
    pub tcs: u64,
    pub function: u32,
    pub exception_vector: u16,
    pub exception_error_code: u16,
    pub exception_addr: u64,
    pub user_handler: u64,
    pub user_data: u64,
    pub reserved: [u8; 216],
}

pub type vdso_sgx_enter_enclave_t = unsafe extern "C" fn(
    rdi: libc::c_ulong,
    rsi: libc::c_ulong,
    rdx: libc::c_ulong,
    function: libc::c_uint,
    r8: libc::c_ulong,
    r9: libc::c_ulong,
    run: *mut sgx_enclave_run,
) -> libc::c_int;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
