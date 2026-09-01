/* SPDX-License-Identifier: GPL-2.0 */
/* Copyright (c) 2020 Facebook */

// Translated from a C header. The original included <linux/types.h>.

#[repr(C)]
pub struct task_struct {
    _private: [u8; 0],
}

#[repr(C)]
pub struct cgroup {
    _private: [u8; 0],
}

#[repr(C)]
pub struct module {
    _private: [u8; 0],
}

#[repr(C)]
pub struct hlist_node {
    _private: [u8; 0],
}

#[repr(C)]
pub struct bpf_testmod_test_read_ctx {
    pub buf: *mut i8,
    pub off: i64,
    pub len: usize,
}

#[repr(C)]
pub struct bpf_testmod_test_write_ctx {
    pub buf: *mut i8,
    pub off: i64,
    pub len: usize,
}

#[repr(C)]
pub struct bpf_testmod_test_writable_ctx {
    pub early_ret: bool,
    pub val: i32,
}

/* BPF iter that returns *value* *n* times in a row */
#[repr(C)]
pub struct bpf_iter_testmod_seq {
    pub value: i64,
    pub cnt: i32,
}

#[repr(C)]
pub struct bpf_testmod_ops_unsupported {
    pub a: i32,
    pub b: i32,
}

#[repr(C)]
pub struct bpf_testmod_ops {
    pub test_1: Option<unsafe extern "C" fn() -> i32>,
    pub test_2: Option<unsafe extern "C" fn(a: i32, b: i32)>,
    /* Used to test nullable arguments. */
    pub test_maybe_null: Option<unsafe extern "C" fn(dummy: i32, task: *mut task_struct) -> i32>,
    pub unsupported_ops: Option<unsafe extern "C" fn() -> i32>,
    /* Used to test ref_acquired arguments. */
    pub test_refcounted: Option<unsafe extern "C" fn(dummy: i32, task: *mut task_struct) -> i32>,
    /* Used to test checking of __ref arguments when it not the first argument. */
    pub test_refcounted_multi: Option<
        unsafe extern "C" fn(dummy: i32, task: *mut task_struct, task2: *mut task_struct) -> i32,
    >,
    /* Used to test returning referenced kptr. */
    pub test_return_ref_kptr: Option<
        unsafe extern "C" fn(
            dummy: i32,
            task: *mut task_struct,
            cgrp: *mut cgroup,
        ) -> *mut task_struct,
    >,

    /* The following fields are used to test shadow copies. */
    pub onebyte: i8,
    pub unsupported: bpf_testmod_ops_unsupported,
    pub data: i32,

    /* The following pointers are used to test the maps having multiple
     * pages of trampolines.
     */
    pub tramp_1: Option<unsafe extern "C" fn(value: i32) -> i32>,
    pub tramp_2: Option<unsafe extern "C" fn(value: i32) -> i32>,
    pub tramp_3: Option<unsafe extern "C" fn(value: i32) -> i32>,
    pub tramp_4: Option<unsafe extern "C" fn(value: i32) -> i32>,
    pub tramp_5: Option<unsafe extern "C" fn(value: i32) -> i32>,
    pub tramp_6: Option<unsafe extern "C" fn(value: i32) -> i32>,
    pub tramp_7: Option<unsafe extern "C" fn(value: i32) -> i32>,
    pub tramp_8: Option<unsafe extern "C" fn(value: i32) -> i32>,
    pub tramp_9: Option<unsafe extern "C" fn(value: i32) -> i32>,
    pub tramp_10: Option<unsafe extern "C" fn(value: i32) -> i32>,
    pub tramp_11: Option<unsafe extern "C" fn(value: i32) -> i32>,
    pub tramp_12: Option<unsafe extern "C" fn(value: i32) -> i32>,
    pub tramp_13: Option<unsafe extern "C" fn(value: i32) -> i32>,
    pub tramp_14: Option<unsafe extern "C" fn(value: i32) -> i32>,
    pub tramp_15: Option<unsafe extern "C" fn(value: i32) -> i32>,
    pub tramp_16: Option<unsafe extern "C" fn(value: i32) -> i32>,
    pub tramp_17: Option<unsafe extern "C" fn(value: i32) -> i32>,
    pub tramp_18: Option<unsafe extern "C" fn(value: i32) -> i32>,
    pub tramp_19: Option<unsafe extern "C" fn(value: i32) -> i32>,
    pub tramp_20: Option<unsafe extern "C" fn(value: i32) -> i32>,
    pub tramp_21: Option<unsafe extern "C" fn(value: i32) -> i32>,
    pub tramp_22: Option<unsafe extern "C" fn(value: i32) -> i32>,
    pub tramp_23: Option<unsafe extern "C" fn(value: i32) -> i32>,
    pub tramp_24: Option<unsafe extern "C" fn(value: i32) -> i32>,
    pub tramp_25: Option<unsafe extern "C" fn(value: i32) -> i32>,
    pub tramp_26: Option<unsafe extern "C" fn(value: i32) -> i32>,
    pub tramp_27: Option<unsafe extern "C" fn(value: i32) -> i32>,
    pub tramp_28: Option<unsafe extern "C" fn(value: i32) -> i32>,
    pub tramp_29: Option<unsafe extern "C" fn(value: i32) -> i32>,
    pub tramp_30: Option<unsafe extern "C" fn(value: i32) -> i32>,
    pub tramp_31: Option<unsafe extern "C" fn(value: i32) -> i32>,
    pub tramp_32: Option<unsafe extern "C" fn(value: i32) -> i32>,
    pub tramp_33: Option<unsafe extern "C" fn(value: i32) -> i32>,
    pub tramp_34: Option<unsafe extern "C" fn(value: i32) -> i32>,
    pub tramp_35: Option<unsafe extern "C" fn(value: i32) -> i32>,
    pub tramp_36: Option<unsafe extern "C" fn(value: i32) -> i32>,
    pub tramp_37: Option<unsafe extern "C" fn(value: i32) -> i32>,
    pub tramp_38: Option<unsafe extern "C" fn(value: i32) -> i32>,
    pub tramp_39: Option<unsafe extern "C" fn(value: i32) -> i32>,
    pub tramp_40: Option<unsafe extern "C" fn(value: i32) -> i32>,
}

#[repr(C)]
pub struct bpf_testmod_ops2 {
    pub test_1: Option<unsafe extern "C" fn() -> i32>,
}

/* 16 bytes, so it takes two argument slots when passed by value */
#[repr(C)]
pub struct bpf_testmod_arena_pair {
    pub a: u64,
    pub b: u64,
}

#[repr(C)]
pub struct bpf_testmod_ops3 {
    pub test_1: Option<unsafe extern "C" fn() -> i32>,
    pub test_2: Option<unsafe extern "C" fn() -> i32>,
    /* Used to test arena pointer arguments. */
    pub test_arena: Option<unsafe extern "C" fn(ptr: *mut u64) -> i32>,
    pub test_arena_nullable: Option<unsafe extern "C" fn(ptr: *mut u64) -> i32>,
    /* enough leading args to force @ptr onto the stack on x86 and arm64 */
    pub test_arena_stack: Option<
        unsafe extern "C" fn(
            a: u64,
            b: u64,
            c: u64,
            d: u64,
            e: u64,
            f: u64,
            g: u64,
            h: u64,
            ptr: *mut u64,
        ) -> i32,
    >,
    /* a multi-slot leading arg, so @ptr is not at the slot its arg index suggests */
    pub test_arena_multislot:
        Option<unsafe extern "C" fn(p: bpf_testmod_arena_pair, ptr: *mut u64) -> i32>,
}

#[repr(C)]
pub struct st_ops_args {
    pub a: u64,
}

#[repr(C)]
pub struct bpf_testmod_st_ops {
    pub test_prologue: Option<unsafe extern "C" fn(args: *mut st_ops_args) -> i32>,
    pub test_epilogue: Option<unsafe extern "C" fn(args: *mut st_ops_args) -> i32>,
    pub test_pro_epilogue: Option<unsafe extern "C" fn(args: *mut st_ops_args) -> i32>,
    pub owner: *mut module,
}

#[repr(C)]
pub struct bpf_testmod_multi_st_ops {
    pub test_1: Option<unsafe extern "C" fn(args: *mut st_ops_args) -> i32>,
    pub node: hlist_node,
    pub id: i32,
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
