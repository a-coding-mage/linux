// SPDX-License-Identifier: ((GPL-2.0 WITH Linux-syscall-note) OR BSD-3-Clause)
// Faithful source-level Rust translation of the generated kernel netlink source.
// External kernel types, constants, callbacks, and symbols are supplied by dependencies.

#[repr(C)]
pub struct NetlinkRangeValidation { pub max: u32 }
#[repr(C)]
pub struct NlAttr { _private: [u8; 0] }
#[repr(C)]
pub struct NetlinkExtAck { _private: [u8; 0] }

pub static mut devlink_attr_index_range: NetlinkRangeValidation =
    NetlinkRangeValidation { max: u32::MAX };

extern "C" {
    fn nla_get_u8(attr: *const NlAttr) -> u8;
    fn nl_set_err_msg_attr(extack: *mut NetlinkExtAck, attr: *const NlAttr, msg: *const u8);
}

pub unsafe fn devlink_attr_param_type_validate(
    attr: *const NlAttr,
    extack: *mut NetlinkExtAck,
) -> i32 {
    match nla_get_u8(attr) {
        DEVLINK_VAR_ATTR_TYPE_U8
        | DEVLINK_VAR_ATTR_TYPE_U16
        | DEVLINK_VAR_ATTR_TYPE_U32
        | DEVLINK_VAR_ATTR_TYPE_U64
        | DEVLINK_VAR_ATTR_TYPE_STRING
        | DEVLINK_VAR_ATTR_TYPE_FLAG
        | DEVLINK_VAR_ATTR_TYPE_NUL_STRING
        | DEVLINK_VAR_ATTR_TYPE_BINARY
        | DEVLINK_VAR_ATTR_TYPE_U64_ARRAY => 0,
        _ => {
            nl_set_err_msg_attr(extack, attr, b"invalid enum value\\0".as_ptr());
            -22
        }
    }
}

macro_rules! policy_decl {
    ($name:ident, $source:expr) => {
        #[doc = $source]
        pub static mut $name: [u8; 0] = [];
    };
}


macro_rules! ops_decl {
    ($name:ident, $source:expr) => {
        #[doc = $source]
        pub static mut $name: [u8; 0] = [];
    };
}
ops_decl!(devlink_nl_ops, r#""#);


// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
