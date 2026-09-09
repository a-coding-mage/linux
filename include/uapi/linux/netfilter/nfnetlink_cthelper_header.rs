/* SPDX-License-Identifier: GPL-2.0 WITH Linux-syscall-note */
// Translated from nfnetlink_cthelper.h.

pub const NFCT_HELPER_STATUS_DISABLED: i32 = 0;
pub const NFCT_HELPER_STATUS_ENABLED: i32 = 1;

#[repr(C)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum nfnl_cthelper_msg_types {
    NFNL_MSG_CTHELPER_NEW = 0,
    NFNL_MSG_CTHELPER_GET,
    NFNL_MSG_CTHELPER_DEL,
    NFNL_MSG_CTHELPER_MAX,
}

#[repr(C)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum nfnl_cthelper_type {
    NFCTH_UNSPEC = 0,
    NFCTH_NAME,
    NFCTH_TUPLE,
    NFCTH_QUEUE_NUM,
    NFCTH_POLICY,
    NFCTH_PRIV_DATA_LEN,
    NFCTH_STATUS,
    __NFCTH_MAX,
}

pub const NFCTH_MAX: i32 = (__NFCTH_MAX as i32) - 1;

#[repr(C)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum nfnl_cthelper_policy_type {
    NFCTH_POLICY_SET_UNSPEC = 0,
    NFCTH_POLICY_SET_NUM,
    NFCTH_POLICY_SET,
    NFCTH_POLICY_SET1 = NFCTH_POLICY_SET as isize,
    NFCTH_POLICY_SET2,
    NFCTH_POLICY_SET3,
    NFCTH_POLICY_SET4,
    __NFCTH_POLICY_SET_MAX,
}

pub const NFCTH_POLICY_SET_MAX: i32 = (__NFCTH_POLICY_SET_MAX as i32) - 1;

#[repr(C)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum nfnl_cthelper_pol_type {
    NFCTH_POLICY_UNSPEC = 0,
    NFCTH_POLICY_NAME,
    NFCTH_POLICY_EXPECT_MAX,
    NFCTH_POLICY_EXPECT_TIMEOUT,
    __NFCTH_POLICY_MAX,
}

pub const NFCTH_POLICY_MAX: i32 = (__NFCTH_POLICY_MAX as i32) - 1;

#[repr(C)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum nfnl_cthelper_tuple_type {
    NFCTH_TUPLE_UNSPEC = 0,
    NFCTH_TUPLE_L3PROTONUM,
    NFCTH_TUPLE_L4PROTONUM,
    __NFCTH_TUPLE_MAX,
}

pub const NFCTH_TUPLE_MAX: i32 = (__NFCTH_TUPLE_MAX as i32) - 1;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
