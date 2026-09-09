/* SPDX-License-Identifier: GPL-2.0+ WITH Linux-syscall-note */
/* Rust translation of include/asm-s390/zcrypt.h. */

pub const ZCRYPT_VERSION: u32 = 2;
pub const ZCRYPT_RELEASE: u32 = 2;
pub const ZCRYPT_VARIANT: u32 = 1;
pub const ZCRYPT_NAME: &str = "zcrypt";

#[repr(C)]
pub struct ica_rsa_modexpo {
    pub inputdata: *mut u8,
    pub inputdatalength: u32,
    pub outputdata: *mut u8,
    pub outputdatalength: u32,
    pub b_key: *mut u8,
    pub n_modulus: *mut u8,
}

#[repr(C)]
pub struct ica_rsa_modexpo_crt {
    pub inputdata: *mut u8,
    pub inputdatalength: u32,
    pub outputdata: *mut u8,
    pub outputdatalength: u32,
    pub bp_key: *mut u8,
    pub bq_key: *mut u8,
    pub np_prime: *mut u8,
    pub nq_prime: *mut u8,
    pub u_mult_inv: *mut u8,
}

#[repr(C, packed)]
pub struct CPRBX {
    pub cprb_len: u16, pub cprb_ver_id: u8, pub ctfm: u8, pub pad_000: [u8; 2],
    pub func_id: [u8; 2], pub cprb_flags: [u8; 4], pub req_parml: u32,
    pub req_datal: u32, pub rpl_msgbl: u32, pub rpld_parml: u32, pub rpl_datal: u32,
    pub rpld_datal: u32, pub req_extbl: u32, pub _pad_001: [u8; 4], pub rpld_extbl: u32,
    pub _pad_002: [u8; 8], pub req_parmb: *mut u8, pub _pad_003: [u8; 8],
    pub req_datab: *mut u8, pub _pad_004: [u8; 8], pub rpl_parmb: *mut u8,
    pub _pad_005: [u8; 8], pub rpl_datab: *mut u8, pub _pad_006: [u8; 8],
    pub req_extb: *mut u8, pub _pad_007: [u8; 8], pub rpl_extb: *mut u8,
    pub ccp_rtcode: u16, pub ccp_rscode: u16, pub mac_data_len: u32,
    pub logon_id: [u8; 8], pub mac_value: [u8; 8], pub mac_content_flgs: u8,
    pub _pad_008: u8, pub domain: u16, pub _pad_009: [u8; 12], pub _pad_010: [u8; 36],
}

#[repr(C, packed)]
pub struct ica_xcRB {
    pub agent_ID: u16, pub user_defined: u32, pub request_ID: u16,
    pub request_control_blk_length: u32, pub _padding1: [u8; 8],
    pub request_control_blk_addr: *mut u8, pub request_data_length: u32,
    pub _padding2: [u8; 8], pub request_data_address: *mut u8,
    pub reply_control_blk_length: u32, pub _padding3: [u8; 8],
    pub reply_control_blk_addr: *mut u8, pub reply_data_length: u32,
    pub __padding4: [u8; 8], pub reply_data_addr: *mut u8,
    pub priority_window: u16, pub status: u32,
}

#[repr(C, packed)]
pub struct ep11_cprb { pub cprb_len: u16, pub cprb_ver_id: u8, pub pad_000: [u8; 2], pub flags: u8, pub func_id: [u8; 2], pub source_id: u32, pub target_id: u32, pub ret_code: u32, pub reserved1: u32, pub reserved2: u32, pub payload_len: u32 }
#[repr(C)]
pub struct ep11_target_dev { pub ap_id: u16, pub dom_id: u16 }
#[repr(C, packed)]
pub struct ep11_urb { pub targets_num: u16, pub targets: *mut u8, pub weight: u64, pub req_no: u64, pub req_len: u64, pub req: *mut u8, pub resp_len: u64, pub resp: *mut u8 }

#[repr(C)]
pub struct zcrypt_device_status_ext { pub bits: u32 }
#[repr(C)]
pub struct zcrypt_device_matrix_ext { pub device: [zcrypt_device_status_ext; MAX_ZDEV_ENTRIES_EXT as usize] }
pub const MAX_ZDEV_CARDIDS_EXT: u32 = 256;
pub const MAX_ZDEV_DOMAINS_EXT: u32 = 256;
pub const MAX_ZDEV_ENTRIES_EXT: u32 = MAX_ZDEV_CARDIDS_EXT * MAX_ZDEV_DOMAINS_EXT;
pub const AUTOSELECT: u32 = 0xffff_ffff;
pub const AUTOSEL_AP: u16 = 0xffff;
pub const AUTOSEL_DOM: u16 = 0xffff;

const IOC_NRBITS: u32 = 8; const IOC_TYPEBITS: u32 = 8; const IOC_SIZEBITS: u32 = 14;
const IOC_NRSHIFT: u32 = 0; const IOC_TYPESHIFT: u32 = IOC_NRSHIFT + IOC_NRBITS;
const IOC_SIZESHIFT: u32 = IOC_TYPESHIFT + IOC_TYPEBITS; const IOC_DIRSHIFT: u32 = IOC_SIZESHIFT + IOC_SIZEBITS;
const IOC_WRITE: u32 = 1; const IOC_READ: u32 = 2;
const fn ioc(dir: u32, ty: u32, nr: u32, size: u32) -> u32 { (dir << IOC_DIRSHIFT) | (ty << IOC_TYPESHIFT) | (nr << IOC_NRSHIFT) | (size << IOC_SIZESHIFT) }
const fn ior(ty: u32, nr: u32, size: u32) -> u32 { ioc(IOC_READ, ty, nr, size) }
const fn iowr(ty: u32, nr: u32) -> u32 { ioc(IOC_READ | IOC_WRITE, ty, nr, 0) }
pub const ZCRYPT_IOCTL_MAGIC: u32 = b'z' as u32;
pub const ICARSAMODEXPO: u32 = iowr(ZCRYPT_IOCTL_MAGIC, 0x05);
pub const ICARSACRT: u32 = iowr(ZCRYPT_IOCTL_MAGIC, 0x06);
pub const ZSECSENDCPRB: u32 = iowr(ZCRYPT_IOCTL_MAGIC, 0x81);
pub const ZSENDEP11CPRB: u32 = iowr(ZCRYPT_IOCTL_MAGIC, 0x04);
pub const ZCRYPT_DEVICE_STATUS: u32 = iowr(ZCRYPT_IOCTL_MAGIC, 0x5f);
pub const ZCRYPT_STATUS_MASK: u32 = ior(ZCRYPT_IOCTL_MAGIC, 0x58, MAX_ZDEV_CARDIDS_EXT);
pub const ZCRYPT_QDEPTH_MASK: u32 = ior(ZCRYPT_IOCTL_MAGIC, 0x59, MAX_ZDEV_CARDIDS_EXT);
pub const ZCRYPT_PERDEV_REQCNT: u32 = ior(ZCRYPT_IOCTL_MAGIC, 0x5a, MAX_ZDEV_CARDIDS_EXT * 4);
pub const ZCRYPT_MAX_MINOR_NODES: u32 = 256;
pub const MAX_ZDEV_IOCTLS: u32 = 1 << 8;
pub const MAX_ZDEV_CARDIDS: u32 = 64; pub const MAX_ZDEV_DOMAINS: u32 = 256;
pub const MAX_ZDEV_ENTRIES: u32 = MAX_ZDEV_CARDIDS * MAX_ZDEV_DOMAINS;
#[repr(C)] pub struct zcrypt_device_status { pub bits: u32 }
#[repr(C)] pub struct zcrypt_device_matrix { pub device: [zcrypt_device_status; MAX_ZDEV_ENTRIES as usize] }
pub const ZDEVICESTATUS: u32 = iowr(ZCRYPT_IOCTL_MAGIC, 0x4f);
pub const Z90STAT_STATUS_MASK: u32 = ior(ZCRYPT_IOCTL_MAGIC, 0x48, 64);
pub const Z90STAT_QDEPTH_MASK: u32 = ior(ZCRYPT_IOCTL_MAGIC, 0x49, 64);
pub const Z90STAT_PERDEV_REQCNT: u32 = ior(ZCRYPT_IOCTL_MAGIC, 0x4a, 64 * 4);
pub const Z90STAT_REQUESTQ_COUNT: u32 = ior(ZCRYPT_IOCTL_MAGIC, 0x44, 4);
pub const Z90STAT_PENDINGQ_COUNT: u32 = ior(ZCRYPT_IOCTL_MAGIC, 0x45, 4);
pub const Z90STAT_TOTALOPEN_COUNT: u32 = ior(ZCRYPT_IOCTL_MAGIC, 0x46, 4);
pub const Z90STAT_DOMAIN_INDEX: u32 = ior(ZCRYPT_IOCTL_MAGIC, 0x47, 4);

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
