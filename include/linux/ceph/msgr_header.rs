/* SPDX-License-Identifier: GPL-2.0 */

/*
 * Data types for message passing layer used by Ceph.
 */

pub const CEPH_MON_PORT: u32 = 6789;

/* tcp connection banner; keep this string length constant. */
pub const CEPH_BANNER: &str = "ceph v027";
pub const CEPH_BANNER_LEN: usize = 9;
pub const CEPH_BANNER_MAX_LEN: usize = 30;

/* messenger V2 connection banner prefix. */
pub const CEPH_BANNER_V2: &str = "ceph v2\n";
pub const CEPH_BANNER_V2_LEN: usize = 8;
pub const CEPH_BANNER_V2_PREFIX_LEN: usize = CEPH_BANNER_V2_LEN + core::mem::size_of::<__le16>();

/* messenger V2 features */
pub const CEPH_MSGR2_INCARNATION_1: u64 = 0;
pub const CEPH_MSGR2_FEATURE_REVISION_1: u64 = 1u64 << 0;
pub const CEPH_MSGR2_FEATUREMASK_REVISION_1: u64 =
    (1u64 << 0) | CEPH_MSGR2_INCARNATION_1;
pub const CEPH_MSGR2_SUPPORTED_FEATURES: u64 = CEPH_MSGR2_FEATURE_REVISION_1;
pub const CEPH_MSGR2_REQUIRED_FEATURES: u64 = CEPH_MSGR2_FEATURE_REVISION_1;

pub type ceph_seq_t = __u32;

#[inline]
pub const fn ceph_seq_cmp(a: __u32, b: __u32) -> __s32 {
    (a as __s32).wrapping_sub(b as __s32)
}

#[repr(C, packed)]
pub struct ceph_entity_name {
    pub type_: __u8,
    pub num: __le64,
}

pub const CEPH_ENTITY_TYPE_MON: u32 = 0x01;
pub const CEPH_ENTITY_TYPE_MDS: u32 = 0x02;
pub const CEPH_ENTITY_TYPE_OSD: u32 = 0x04;
pub const CEPH_ENTITY_TYPE_CLIENT: u32 = 0x08;
pub const CEPH_ENTITY_TYPE_AUTH: u32 = 0x20;
pub const CEPH_ENTITY_TYPE_ANY: u32 = 0xFF;

extern "C" {
    pub fn ceph_entity_type_name(type_: core::ffi::c_int) -> *const core::ffi::c_char;
}

#[repr(C, packed)]
pub struct ceph_entity_addr {
    pub type_: __le32,
    pub nonce: __le32,
    pub in_addr: sockaddr_storage,
}

extern "C" {
    pub fn memcmp(a: *const core::ffi::c_void, b: *const core::ffi::c_void, n: usize) -> core::ffi::c_int;
}

#[inline]
pub unsafe fn ceph_addr_equal_no_type(
    lhs: *const ceph_entity_addr,
    rhs: *const ceph_entity_addr,
) -> bool {
    memcmp(
        core::ptr::addr_of!((*lhs).in_addr) as *const core::ffi::c_void,
        core::ptr::addr_of!((*rhs).in_addr) as *const core::ffi::c_void,
        core::mem::size_of::<sockaddr_storage>(),
    ) == 0 && (*lhs).nonce == (*rhs).nonce
}

#[repr(C, packed)]
pub struct ceph_entity_inst {
    pub name: ceph_entity_name,
    pub addr: ceph_entity_addr,
}

pub const CEPH_MSGR_TAG_READY: u32 = 1;
pub const CEPH_MSGR_TAG_RESETSESSION: u32 = 2;
pub const CEPH_MSGR_TAG_WAIT: u32 = 3;
pub const CEPH_MSGR_TAG_RETRY_SESSION: u32 = 4;
pub const CEPH_MSGR_TAG_RETRY_GLOBAL: u32 = 5;
pub const CEPH_MSGR_TAG_CLOSE: u32 = 6;
pub const CEPH_MSGR_TAG_MSG: u32 = 7;
pub const CEPH_MSGR_TAG_ACK: u32 = 8;
pub const CEPH_MSGR_TAG_KEEPALIVE: u32 = 9;
pub const CEPH_MSGR_TAG_BADPROTOVER: u32 = 10;
pub const CEPH_MSGR_TAG_BADAUTHORIZER: u32 = 11;
pub const CEPH_MSGR_TAG_FEATURES: u32 = 12;
pub const CEPH_MSGR_TAG_SEQ: u32 = 13;
pub const CEPH_MSGR_TAG_KEEPALIVE2: u32 = 14;
pub const CEPH_MSGR_TAG_KEEPALIVE2_ACK: u32 = 15;
pub const CEPH_MSGR_TAG_CHALLENGE_AUTHORIZER: u32 = 16;

#[repr(C, packed)]
pub struct ceph_msg_connect {
    pub features: __le64,
    pub host_type: __le32,
    pub global_seq: __le32,
    pub connect_seq: __le32,
    pub protocol_version: __le32,
    pub authorizer_protocol: __le32,
    pub authorizer_len: __le32,
    pub flags: __u8,
}

#[repr(C, packed)]
pub struct ceph_msg_connect_reply {
    pub tag: __u8,
    pub features: __le64,
    pub global_seq: __le32,
    pub connect_seq: __le32,
    pub protocol_version: __le32,
    pub authorizer_len: __le32,
    pub flags: __u8,
}

pub const CEPH_MSG_CONNECT_LOSSY: u32 = 1;

#[repr(C, packed)]
pub struct ceph_msg_header_old {
    pub seq: __le64, pub tid: __le64, pub type_: __le16, pub priority: __le16, pub version: __le16,
    pub front_len: __le32, pub middle_len: __le32, pub data_len: __le32, pub data_off: __le16,
    pub src: ceph_entity_inst, pub orig_src: ceph_entity_inst, pub reserved: __le32, pub crc: __le32,
}

#[repr(C, packed)]
pub struct ceph_msg_header {
    pub seq: __le64, pub tid: __le64, pub type_: __le16, pub priority: __le16, pub version: __le16,
    pub front_len: __le32, pub middle_len: __le32, pub data_len: __le32, pub data_off: __le16,
    pub src: ceph_entity_name, pub compat_version: __le16, pub reserved: __le16, pub crc: __le32,
}

#[repr(C, packed)]
pub struct ceph_msg_header2 {
    pub seq: __le64, pub tid: __le64, pub type_: __le16, pub priority: __le16, pub version: __le16,
    pub data_pre_padding_len: __le32, pub data_off: __le16, pub ack_seq: __le64, pub flags: __u8,
    pub compat_version: __le16, pub reserved: __le16,
}

pub const CEPH_MSG_PRIO_LOW: u32 = 64;
pub const CEPH_MSG_PRIO_DEFAULT: u32 = 127;
pub const CEPH_MSG_PRIO_HIGH: u32 = 196;
pub const CEPH_MSG_PRIO_HIGHEST: u32 = 255;

#[repr(C, packed)]
pub struct ceph_msg_footer_old {
    pub front_crc: __le32, pub middle_crc: __le32, pub data_crc: __le32, pub flags: __u8,
}

#[repr(C, packed)]
pub struct ceph_msg_footer {
    pub front_crc: __le32, pub middle_crc: __le32, pub data_crc: __le32,
    // sig holds the 64 bits of the digital signature for the message PLR
    pub sig: __le64, pub flags: __u8,
}

pub const CEPH_MSG_FOOTER_COMPLETE: u32 = 1 << 0;
pub const CEPH_MSG_FOOTER_NOCRC: u32 = 1 << 1;
pub const CEPH_MSG_FOOTER_SIGNED: u32 = 1 << 2;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
