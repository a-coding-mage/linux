/* SPDX-License-Identifier: GPL-2.0 */
// Rust translation of net/netlink.h.  Names supplied by the kernel are kept
// external; this header intentionally does not provide their implementations.

#[repr(C)]
pub struct netlink_range_validation { pub min: u64, pub max: u64 }
#[repr(C)]
pub struct netlink_range_validation_signed { pub min: i64, pub max: i64 }

#[repr(C)]
pub union nla_policy_validation_data {
    pub strict_start_type: u16,
    pub bitfield32_valid: u32,
    pub mask: u32,
    pub reject_message: *const core::ffi::c_char,
    pub nested_policy: *const nla_policy,
    pub range: *const netlink_range_validation,
    pub range_signed: *const netlink_range_validation_signed,
    pub min_max: [i16; 2],
    pub validate: Option<unsafe extern "C" fn(*const nlattr, *mut netlink_ext_ack) -> i32>,
}
#[repr(C)]
pub struct nla_policy { pub r#type: u8, pub validation_type: u8, pub len: u16, pub data: nla_policy_validation_data }

#[repr(C)]
pub struct nl_info {
    pub nlh: *mut nlmsghdr,
    pub nl_net: *mut net,
    pub portid: u32,
    pub skip_notify: u8,
    pub skip_notify_kernel: u8,
}

pub const NLA_UNSPEC: i32 = 0;
pub const NLA_U8: i32 = 1; pub const NLA_U16: i32 = 2; pub const NLA_U32: i32 = 3;
pub const NLA_U64: i32 = 4; pub const NLA_STRING: i32 = 5; pub const NLA_FLAG: i32 = 6;
pub const NLA_MSECS: i32 = 7; pub const NLA_NESTED: i32 = 8; pub const NLA_NESTED_ARRAY: i32 = 9;
pub const NLA_NUL_STRING: i32 = 10; pub const NLA_BINARY: i32 = 11; pub const NLA_S8: i32 = 12;
pub const NLA_S16: i32 = 13; pub const NLA_S32: i32 = 14; pub const NLA_S64: i32 = 15;
pub const NLA_BITFIELD32: i32 = 16; pub const NLA_REJECT: i32 = 17; pub const NLA_BE16: i32 = 18;
pub const NLA_BE32: i32 = 19; pub const NLA_SINT: i32 = 20; pub const NLA_UINT: i32 = 21;
pub const __NLA_TYPE_MAX: i32 = 22; pub const NLA_TYPE_MAX: i32 = __NLA_TYPE_MAX - 1;

pub const NLA_VALIDATE_NONE: i32 = 0;
pub const NLA_VALIDATE_RANGE: i32 = 1;
pub const NLA_VALIDATE_RANGE_WARN_TOO_LONG: i32 = 2;
pub const NLA_VALIDATE_MIN: i32 = 3;
pub const NLA_VALIDATE_MAX: i32 = 4;
pub const NLA_VALIDATE_MASK: i32 = 5;
pub const NLA_VALIDATE_RANGE_PTR: i32 = 6;
pub const NLA_VALIDATE_FUNCTION: i32 = 7;

pub const NL_VALIDATE_LIBERAL: u32 = 0;
pub const NL_VALIDATE_TRAILING: u32 = 1 << 0;
pub const NL_VALIDATE_MAXTYPE: u32 = 1 << 1;
pub const NL_VALIDATE_UNSPEC: u32 = 1 << 2;
pub const NL_VALIDATE_STRICT_ATTRS: u32 = 1 << 3;
pub const NL_VALIDATE_NESTED: u32 = 1 << 4;
pub const NL_VALIDATE_DEPRECATED_STRICT: u32 = NL_VALIDATE_TRAILING | NL_VALIDATE_MAXTYPE;
pub const NL_VALIDATE_STRICT: u32 = NL_VALIDATE_DEPRECATED_STRICT | NL_VALIDATE_UNSPEC | NL_VALIDATE_STRICT_ATTRS | NL_VALIDATE_NESTED;

// External kernel types and functions referenced by this header.
#[repr(C)] pub struct nlmsghdr { pub nlmsg_len: u32, pub nlmsg_type: u16, pub nlmsg_flags: u16, pub nlmsg_seq: u32, pub nlmsg_pid: u32 }
#[repr(C)] pub struct nlattr { pub nla_len: u16, pub nla_type: u16 }
#[repr(C)] pub struct sk_buff { _private: [u8; 0] }
#[repr(C)] pub struct sock { _private: [u8; 0] }
#[repr(C)] pub struct net { _private: [u8; 0] }
#[repr(C)] pub struct netlink_ext_ack { _private: [u8; 0] }
#[repr(C)] pub struct netlink_callback { pub skb: *mut sk_buff, pub nlh: *mut nlmsghdr, pub prev_seq: u32, pub seq: u32 }

extern "C" {
    pub fn __nla_validate(head: *const nlattr, len: i32, maxtype: i32, policy: *const nla_policy, validate: u32, extack: *mut netlink_ext_ack) -> i32;
    pub fn __nla_parse(tb: *mut *mut nlattr, maxtype: i32, head: *const nlattr, len: i32, policy: *const nla_policy, validate: u32, extack: *mut netlink_ext_ack) -> i32;
    pub fn nla_find(head: *const nlattr, len: i32, attrtype: i32) -> *mut nlattr;
    pub fn nla_put(skb: *mut sk_buff, attrtype: i32, attrlen: i32, data: *const core::ffi::c_void) -> i32;
    pub fn nla_put_64bit(skb: *mut sk_buff, attrtype: i32, attrlen: i32, data: *const core::ffi::c_void, padattr: i32) -> i32;
}

#[inline] pub unsafe fn nlmsg_msg_size(payload: i32) -> i32 { 16 + payload }
#[inline] pub unsafe fn nlmsg_len(nlh: *const nlmsghdr) -> i32 { (*nlh).nlmsg_len as i32 - 16 }
#[inline] pub unsafe fn nla_attr_size(payload: i32) -> i32 { 4 + payload }
#[inline] pub unsafe fn nla_type(nla: *const nlattr) -> i32 { ((*nla).nla_type & 0x3fff) as i32 }
#[inline] pub unsafe fn nla_len(nla: *const nlattr) -> u16 { (*nla).nla_len - 4 }


// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
