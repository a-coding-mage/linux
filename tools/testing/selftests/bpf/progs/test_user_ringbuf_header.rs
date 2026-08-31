/* SPDX-License-Identifier: GPL-2.0 */
/* Copyright (c) 2022 Meta Platforms, Inc. and affiliates. */

pub const TEST_OP_64: i32 = 4;
pub const TEST_OP_32: i32 = 2;

#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum test_msg_op {
    TEST_MSG_OP_INC64,
    TEST_MSG_OP_INC32,
    TEST_MSG_OP_MUL64,
    TEST_MSG_OP_MUL32,

    // Must come last.
    TEST_MSG_OP_NUM_OPS,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub union test_msg__bindgen_ty_1 {
    pub operand_64: __s64,
    pub operand_32: __s32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct test_msg {
    pub msg_op: test_msg_op,
    pub __bindgen_anon_1: test_msg__bindgen_ty_1,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct sample {
    pub pid: ::std::os::raw::c_int,
    pub seq: ::std::os::raw::c_int,
    pub value: ::std::os::raw::c_long,
    pub comm: [::std::os::raw::c_char; 16],
}
