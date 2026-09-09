/* SPDX-License-Identifier: GPL-2.0 */
/* Intel Software Guard Extensions (SGX) support. */

/* Architectural definitions. */
pub const SGX_CPUID: u32 = 0x12;
pub const SGX_CPUID_EPC: u32 = 2;
pub const SGX_CPUID_EPC_INVALID: u32 = 0x0;
pub const SGX_CPUID_EPC_SECTION: u32 = 0x1;
pub const SGX_CPUID_EPC_MASK: u32 = 0xF;

#[repr(i32)]
pub enum sgx_encls_function {
    ECREATE = 0x00,
    EADD = 0x01,
    EINIT = 0x02,
    EREMOVE = 0x03,
    EDGBRD = 0x04,
    EDGBWR = 0x05,
    EEXTEND = 0x06,
    ELDU = 0x08,
    EBLOCK = 0x09,
    EPA = 0x0A,
    EWB = 0x0B,
    ETRACK = 0x0C,
    EAUG = 0x0D,
    EMODPR = 0x0E,
    EMODT = 0x0F,
    EUPDATESVN = 0x18,
}

pub const SGX_ENCLS_FAULT_FLAG: u32 = 0x40000000;

#[repr(i32)]
pub enum sgx_return_code {
    SGX_EPC_PAGE_CONFLICT = 7,
    SGX_NOT_TRACKED = 11,
    SGX_CHILD_PRESENT = 13,
    SGX_INVALID_EINITTOKEN = 16,
    SGX_PAGE_NOT_MODIFIABLE = 20,
    SGX_INSUFFICIENT_ENTROPY = 29,
    SGX_NO_UPDATE = 31,
    SGX_UNMASKED_EVENT = 128,
}

pub const SGX_MODULUS_SIZE: usize = 384;

#[repr(i32)]
pub enum sgx_miscselect { SGX_MISC_EXINFO = 1 << 0 }
pub const SGX_MISC_RESERVED_MASK: u64 = !((1u64 << 1) - 1);
pub const SGX_SSA_GPRS_SIZE: usize = 184;
pub const SGX_SSA_MISC_EXINFO_SIZE: usize = 16;

#[repr(i32)]
pub enum sgx_attribute {
    SGX_ATTR_INIT = 1 << 0,
    SGX_ATTR_DEBUG = 1 << 1,
    SGX_ATTR_MODE64BIT = 1 << 2,
    SGX_ATTR_PROVISIONKEY = 1 << 4,
    SGX_ATTR_EINITTOKENKEY = 1 << 5,
    SGX_ATTR_KSS = 1 << 7,
    SGX_ATTR_ASYNC_EXIT_NOTIFY = 1 << 10,
}
pub const SGX_ATTR_RESERVED_MASK: u64 = (1u64 << 3) | (1u64 << 6) | (1u64 << 8) | (1u64 << 9) | (!0u64 << 11);
pub const SGX_ATTR_UNPRIV_MASK: u64 = (1 << 1) | (1 << 2) | (1 << 7) | (1 << 10);
pub const SGX_ATTR_PRIV_MASK: u64 = (1 << 4) | (1 << 5);

#[repr(C, packed)]
pub struct sgx_secs {
    pub size: u64, pub base: u64, pub ssa_frame_size: u32, pub miscselect: u32,
    pub reserved1: [u8; 24], pub attributes: u64, pub xfrm: u64,
    pub mrenclave: [u32; 8], pub reserved2: [u8; 32], pub mrsigner: [u32; 8],
    pub reserved3: [u8; 32], pub config_id: [u32; 16], pub isv_prod_id: u16,
    pub isv_svn: u16, pub config_svn: u16, pub reserved4: [u8; 3834],
}

#[repr(i32)]
pub enum sgx_tcs_flags { SGX_TCS_DBGOPTIN = 0x01 }
pub const SGX_TCS_RESERVED_MASK: u64 = !((1u64 << 1) - 1);
pub const SGX_TCS_RESERVED_SIZE: usize = 4024;

#[repr(C, packed)]
pub struct sgx_tcs {
    pub state: u64, pub flags: u64, pub ssa_offset: u64, pub ssa_index: u32,
    pub nr_ssa_frames: u32, pub entry_offset: u64, pub exit_addr: u64,
    pub fs_offset: u64, pub gs_offset: u64, pub fs_limit: u32, pub gs_limit: u32,
    pub reserved: [u8; SGX_TCS_RESERVED_SIZE],
}

#[repr(C, packed, align(32))]
pub struct sgx_pageinfo { pub addr: u64, pub contents: u64, pub metadata: u64, pub secs: u64 }

#[repr(i32)]
pub enum sgx_page_type { SGX_PAGE_TYPE_SECS, SGX_PAGE_TYPE_TCS, SGX_PAGE_TYPE_REG, SGX_PAGE_TYPE_VA, SGX_PAGE_TYPE_TRIM }
pub const SGX_NR_PAGE_TYPES: usize = 5;
pub const SGX_PAGE_TYPE_MASK: u64 = 0xFF;

#[repr(i32)]
pub enum sgx_secinfo_flags {
    SGX_SECINFO_R = 1 << 0, SGX_SECINFO_W = 1 << 1, SGX_SECINFO_X = 1 << 2,
    SGX_SECINFO_SECS = 0 << 8, SGX_SECINFO_TCS = 1 << 8, SGX_SECINFO_REG = 2 << 8,
    SGX_SECINFO_VA = 3 << 8, SGX_SECINFO_TRIM = 4 << 8,
}
pub const SGX_SECINFO_PERMISSION_MASK: u64 = 0x7;
pub const SGX_SECINFO_PAGE_TYPE_MASK: u64 = 0xFF00;
pub const SGX_SECINFO_RESERVED_MASK: u64 = !(SGX_SECINFO_PERMISSION_MASK | SGX_SECINFO_PAGE_TYPE_MASK);

#[repr(C, packed, align(64))]
pub struct sgx_secinfo { pub flags: u64, pub reserved: [u8; 56] }
pub const SGX_PCMD_RESERVED_SIZE: usize = 40;

#[repr(C, packed, align(128))]
pub struct sgx_pcmd { pub secinfo: sgx_secinfo, pub enclave_id: u64, pub reserved: [u8; SGX_PCMD_RESERVED_SIZE], pub mac: [u8; 16] }

pub const SGX_SIGSTRUCT_RESERVED1_SIZE: usize = 84;
pub const SGX_SIGSTRUCT_RESERVED2_SIZE: usize = 20;
pub const SGX_SIGSTRUCT_RESERVED3_SIZE: usize = 32;
pub const SGX_SIGSTRUCT_RESERVED4_SIZE: usize = 12;

#[repr(C, packed)]
pub struct sgx_sigstruct_header { pub header1: [u64; 2], pub vendor: u32, pub date: u32, pub header2: [u64; 2], pub swdefined: u32, pub reserved1: [u8; 84] }

#[repr(C, packed)]
pub struct sgx_sigstruct_body {
    pub miscselect: u32, pub misc_mask: u32, pub reserved2: [u8; 20], pub attributes: u64,
    pub xfrm: u64, pub attributes_mask: u64, pub xfrm_mask: u64, pub mrenclave: [u8; 32],
    pub reserved3: [u8; 32], pub isvprodid: u16, pub isvsvn: u16,
}

#[repr(C, packed)]
pub struct sgx_sigstruct {
    pub header: sgx_sigstruct_header, pub modulus: [u8; SGX_MODULUS_SIZE], pub exponent: u32,
    pub signature: [u8; SGX_MODULUS_SIZE], pub body: sgx_sigstruct_body, pub reserved4: [u8; 12],
    pub q1: [u8; SGX_MODULUS_SIZE], pub q2: [u8; SGX_MODULUS_SIZE],
}

pub const SGX_LAUNCH_TOKEN_SIZE: usize = 304;

/* CONFIG_X86_SGX_KVM declarations are retained as conditional external interfaces. */
#[cfg(CONFIG_X86_SGX_KVM)]
extern "C" {
    pub fn sgx_virt_ecreate(pageinfo: *mut sgx_pageinfo, secs: *mut core::ffi::c_void, trapnr: *mut i32) -> i32;
    pub fn sgx_virt_einit(sigstruct: *mut core::ffi::c_void, token: *mut core::ffi::c_void, secs: *mut core::ffi::c_void, lepubkeyhash: *mut u64, trapnr: *mut i32) -> i32;
}

extern "C" {
    pub fn sgx_set_attribute(allowed_attributes: *mut usize, attribute_fd: u32) -> i32;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
