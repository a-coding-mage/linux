// SPDX-License-Identifier: GPL-2.0
/*
 * Copyright (C) 2021. Huawei Technologies Co., Ltd
 */

// Kernel dependencies supplied by the surrounding translation environment.

type __u32 = u32;
type s32 = i32;
type u32 = std::os::raw::c_uint;
type u64 = std::os::raw::c_ulonglong;
type __user = ();

extern "C" {
    static mut bpf_dummy_ops_btf: *mut btf;
    static bpf_struct_ops_link_lops: bpf_link_ops;
    static bpf_dummy_ops: bpf_dummy_ops;
    static bpf_dummy_ops: bpf_dummy_ops;
}

#[repr(C)]
pub struct bpf_struct_ops {
    pub verifier_ops: *const bpf_verifier_ops,
    pub init: Option<unsafe extern "C" fn(*mut btf) -> i32>,
    pub check_member: Option<unsafe extern "C" fn(*const btf_type, *const btf_member, *const bpf_prog) -> i32>,
    pub init_member: Option<unsafe extern "C" fn(*const btf_type, *const btf_member, *mut std::ffi::c_void, *const std::ffi::c_void) -> i32>,
    pub reg: Option<unsafe extern "C" fn(*mut std::ffi::c_void, *mut bpf_link) -> i32>,
    pub unreg: Option<unsafe extern "C" fn(*mut std::ffi::c_void, *mut bpf_link)>,
    pub name: *const std::os::raw::c_char,
    pub cfi_stubs: *mut bpf_dummy_ops,
    pub owner: *mut std::ffi::c_void,
    pub func_models: *mut bpf_func_model,
}

#[repr(C)]
pub struct bpf_dummy_ops_state { pub data: [u8; 0] }
#[repr(C)]
pub struct bpf_dummy_ops {
    pub test_1: Option<unsafe extern "C" fn(*mut bpf_dummy_ops_state) -> i32>,
    pub test_2: Option<unsafe extern "C" fn(*mut bpf_dummy_ops_state, i32, u16, i8, usize) -> i32>,
    pub test_sleepable: Option<unsafe extern "C" fn(*mut bpf_dummy_ops_state) -> i32>,
}
#[repr(C)]
pub struct bpf_dummy_ops_test_args {
    pub args: [u64; 5],
    pub state: bpf_dummy_ops_state,
}

pub struct btf;
pub struct btf_type;
pub struct btf_member;
pub struct bpf_prog;
pub struct bpf_prog_aux;
pub struct bpf_link;
pub struct bpf_link_ops;
pub struct bpf_tramp_nodes;
pub struct bpf_tramp_link { pub node: std::ffi::c_void, pub link: bpf_link }
pub struct bpf_verifier_log;
pub struct bpf_reg_state { pub btf: *mut btf, pub btf_id: u32 }
pub struct bpf_insn_access_aux;
pub struct bpf_func_model;
pub union bpf_attr { pub test: bpf_test_attr }
#[repr(C)] pub struct bpf_test_attr { pub ctx_size_in: __u32, pub ctx_in: u64, pub retval: i32 }
pub enum bpf_access_type {}
pub struct bpf_verifier_ops {
    pub is_valid_access: Option<unsafe extern "C" fn(i32, i32, bpf_access_type, *const bpf_prog, *mut bpf_insn_access_aux) -> bool>,
    pub btf_struct_access: Option<unsafe extern "C" fn(*mut bpf_verifier_log, *const bpf_reg_state, i32, i32) -> i32>,
}

type dummy_ops_test_ret_fn = unsafe extern "C" fn(*mut bpf_dummy_ops_state, ...) -> i32;

unsafe extern "C" fn dummy_ops_test_ret_function(_state: *mut bpf_dummy_ops_state, ...) -> i32 { 0 }

unsafe fn dummy_ops_init_args(kattr: *const bpf_attr, nr: u32) -> *mut bpf_dummy_ops_test_args {
    let size_in = unsafe { (*kattr).test.ctx_size_in };
    if size_in != std::mem::size_of::<u64>() as u32 * nr { return std::ptr::null_mut(); }
    let args = unsafe { libc::calloc(1, std::mem::size_of::<bpf_dummy_ops_test_args>()) as *mut bpf_dummy_ops_test_args };
    if args.is_null() { return std::ptr::null_mut(); }
    let ctx_in = unsafe { (*kattr).test.ctx_in as *const u64 };
    unsafe { std::ptr::copy_nonoverlapping(ctx_in, (*args).args.as_mut_ptr(), nr as usize); }
    let u_state = unsafe { (*args).args[0] as *const bpf_dummy_ops_state };
    if !u_state.is_null() { unsafe { std::ptr::copy_nonoverlapping(u_state, &mut (*args).state, 1); } }
    args
}

unsafe fn dummy_ops_copy_args(args: *mut bpf_dummy_ops_test_args) -> i32 {
    let u_state = unsafe { (*args).args[0] as *mut bpf_dummy_ops_state };
    if !u_state.is_null() { unsafe { std::ptr::copy_nonoverlapping(&(*args).state, u_state, 1); } }
    0
}

unsafe fn dummy_ops_call_op(image: *mut std::ffi::c_void, args: *mut bpf_dummy_ops_test_args) -> i32 {
    let test: dummy_ops_test_ret_fn = unsafe { std::mem::transmute(image) };
    let state = if unsafe { (*args).args[0] != 0 } { unsafe { &mut (*args).state } } else { std::ptr::null_mut() };
    test(state, unsafe { (*args).args[1] }, unsafe { (*args).args[2] }, unsafe { (*args).args[3] }, unsafe { (*args).args[4] })
}

unsafe fn find_ctx_arg_info(_aux: *mut bpf_prog_aux, _offset: i32) -> *const std::ffi::c_void { std::ptr::null() }

unsafe fn check_test_run_args(_prog: *mut bpf_prog, _args: *mut bpf_dummy_ops_test_args) -> i32 { 0 }

unsafe extern "C" fn bpf_struct_ops_test_run(_prog: *mut bpf_prog, _kattr: *const bpf_attr, _uattr: *mut bpf_attr) -> i32 { 0 }

unsafe extern "C" fn bpf_dummy_init(btf: *mut btf) -> i32 { bpf_dummy_ops_btf = btf; 0 }
unsafe extern "C" fn bpf_dummy_ops_is_valid_access(_off: i32, _size: i32, _type: bpf_access_type, _prog: *const bpf_prog, _info: *mut bpf_insn_access_aux) -> bool { true }
unsafe extern "C" fn bpf_dummy_ops_check_member(_t: *const btf_type, _member: *const btf_member, _prog: *const bpf_prog) -> i32 { 0 }
unsafe extern "C" fn bpf_dummy_ops_btf_struct_access(_log: *mut bpf_verifier_log, _reg: *const bpf_reg_state, _off: i32, _size: i32) -> i32 { 0 }

static bpf_dummy_verifier_ops: bpf_verifier_ops = bpf_verifier_ops { is_valid_access: Some(bpf_dummy_ops_is_valid_access), btf_struct_access: Some(bpf_dummy_ops_btf_struct_access) };
unsafe extern "C" fn bpf_dummy_init_member(_t: *const btf_type, _member: *const btf_member, _kdata: *mut std::ffi::c_void, _udata: *const std::ffi::c_void) -> i32 { -95 }
unsafe extern "C" fn bpf_dummy_reg(_kdata: *mut std::ffi::c_void, _link: *mut bpf_link) -> i32 { -95 }
unsafe extern "C" fn bpf_dummy_unreg(_kdata: *mut std::ffi::c_void, _link: *mut bpf_link) {}
unsafe extern "C" fn bpf_dummy_ops__test_1(_cb: *mut bpf_dummy_ops_state) -> i32 { 0 }
unsafe extern "C" fn bpf_dummy_test_2(_cb: *mut bpf_dummy_ops_state, _a1: i32, _a2: u16, _a3: i8, _a4: usize) -> i32 { 0 }
unsafe extern "C" fn bpf_dummy_test_sleepable(_cb: *mut bpf_dummy_ops_state) -> i32 { 0 }

static mut __bpf_bpf_dummy_ops: bpf_dummy_ops = bpf_dummy_ops { test_1: Some(bpf_dummy_ops__test_1), test_2: Some(bpf_dummy_test_2), test_sleepable: Some(bpf_dummy_test_sleepable) };
static mut bpf_bpf_dummy_ops_instance: bpf_struct_ops = bpf_struct_ops { verifier_ops: &bpf_dummy_verifier_ops, init: Some(bpf_dummy_init), check_member: Some(bpf_dummy_ops_check_member), init_member: Some(bpf_dummy_init_member), reg: Some(bpf_dummy_reg), unreg: Some(bpf_dummy_unreg), name: b"bpf_dummy_ops\0".as_ptr() as *const _, cfi_stubs: unsafe { &mut __bpf_bpf_dummy_ops }, owner: std::ptr::null_mut(), func_models: std::ptr::null_mut() };
unsafe extern "C" fn bpf_dummy_struct_ops_init() -> i32 { 0 }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
