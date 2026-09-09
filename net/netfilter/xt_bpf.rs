// SPDX-License-Identifier: GPL-2.0-only
/* Xtables module to match packets using a BPF filter.
 * Copyright 2013 Google Inc.
 * Written by Willem de Bruijn <willemb@google.com>
 */

// C includes and kernel-provided definitions are supplied by other files.

use core::mem::{offset_of, size_of};

extern "C" {
    fn bpf_prog_create(ret: *mut *mut bpf_prog, program: *mut sock_fprog_kern) -> i32;
    fn bpf_prog_get_type(fd: i32, prog_type: u32) -> *mut bpf_prog;
    fn bpf_prog_get_type_path(path: *const core::ffi::c_char, prog_type: u32) -> *mut bpf_prog;
    fn bpf_prog_run(prog: *mut bpf_prog, skb: *const sk_buff) -> bool;
    fn bpf_prog_run_save_cb(prog: *mut bpf_prog, skb: *mut sk_buff) -> u32;
    fn bpf_prog_destroy(prog: *mut bpf_prog);
    fn xt_register_matches(matches: *mut xt_match, count: usize) -> i32;
    fn xt_unregister_matches(matches: *mut xt_match, count: usize);
    fn strnlen(s: *const core::ffi::c_char, maxlen: usize) -> usize;
}

#[repr(C)]
pub struct sock_filter {
    _private: [u8; 0],
}

#[repr(C)]
pub struct sock_fprog_kern {
    pub len: u16,
    pub filter: *mut sock_filter,
}

#[repr(C)]
pub struct bpf_prog {
    _private: [u8; 0],
}

#[repr(C)]
pub struct sk_buff {
    _private: [u8; 0],
}

#[repr(C)]
pub struct xt_mtchk_param {
    pub matchinfo: *mut core::ffi::c_void,
}

#[repr(C)]
pub struct xt_action_param {
    pub matchinfo: *mut core::ffi::c_void,
}

#[repr(C)]
pub struct xt_mtdtor_param {
    pub matchinfo: *mut core::ffi::c_void,
}

#[repr(C)]
pub struct xt_bpf_info {
    pub bpf_program: *mut sock_filter,
    pub bpf_program_num_elem: u16,
    pub filter: *mut bpf_prog,
}

#[repr(C)]
pub struct xt_bpf_info_v1 {
    pub mode: u32,
    pub fd: i32,
    pub path: [core::ffi::c_char; XT_BPF_PATH_MAX],
    pub bpf_program: *mut sock_filter,
    pub bpf_program_num_elem: u16,
    pub filter: *mut bpf_prog,
}

#[repr(C)]
pub struct xt_match {
    pub name: *const core::ffi::c_char,
    pub revision: u8,
    pub family: u16,
    pub checkentry: Option<unsafe extern "C" fn(*const xt_mtchk_param) -> i32>,
    pub r#match: Option<unsafe extern "C" fn(*const sk_buff, *mut xt_action_param) -> bool>,
    pub destroy: Option<unsafe extern "C" fn(*const xt_mtdtor_param)>,
    pub matchsize: usize,
    pub usersize: usize,
    pub me: *mut core::ffi::c_void,
}

const XT_BPF_MAX_NUM_INSTR: u16 = 4096;
const XT_BPF_PATH_MAX: usize = 256;
const BPF_PROG_TYPE_SOCKET_FILTER: u32 = 1;
const XT_BPF_MODE_BYTECODE: u32 = 0;
const XT_BPF_MODE_FD_ELF: u32 = 1;
const XT_BPF_MODE_PATH_PINNED: u32 = 2;
const NFPROTO_UNSPEC: u16 = 0;

unsafe fn __bpf_mt_check_bytecode(
    insns: *mut sock_filter,
    len: u16,
    ret: *mut *mut bpf_prog,
) -> i32 {
    if len > XT_BPF_MAX_NUM_INSTR {
        return -22;
    }

    let mut program = sock_fprog_kern { len, filter: insns };

    if bpf_prog_create(ret, &mut program) != 0 {
        return -22;
    }

    0
}

unsafe fn __bpf_mt_check_fd(fd: i32, ret: *mut *mut bpf_prog) -> i32 {
    let prog = bpf_prog_get_type(fd, BPF_PROG_TYPE_SOCKET_FILTER);
    if (prog as isize) < 0 && (prog as isize) >= -4095 {
        return prog as i32;
    }

    *ret = prog;
    0
}

unsafe fn __bpf_mt_check_path(
    path: *const core::ffi::c_char,
    ret: *mut *mut bpf_prog,
) -> i32 {
    if strnlen(path, XT_BPF_PATH_MAX) == XT_BPF_PATH_MAX {
        return -22;
    }

    *ret = bpf_prog_get_type_path(path, BPF_PROG_TYPE_SOCKET_FILTER);
    let value = *ret as isize;
    if value < 0 && value >= -4095 { value as i32 } else { 0 }
}

unsafe extern "C" fn bpf_mt_check(par: *const xt_mtchk_param) -> i32 {
    let info = (*par).matchinfo as *mut xt_bpf_info;
    __bpf_mt_check_bytecode((*info).bpf_program, (*info).bpf_program_num_elem, &mut (*info).filter)
}

unsafe extern "C" fn bpf_mt_check_v1(par: *const xt_mtchk_param) -> i32 {
    let info = (*par).matchinfo as *mut xt_bpf_info_v1;
    if (*info).mode == XT_BPF_MODE_BYTECODE {
        __bpf_mt_check_bytecode((*info).bpf_program, (*info).bpf_program_num_elem, &mut (*info).filter)
    } else if (*info).mode == XT_BPF_MODE_FD_ELF {
        __bpf_mt_check_fd((*info).fd, &mut (*info).filter)
    } else if (*info).mode == XT_BPF_MODE_PATH_PINNED {
        __bpf_mt_check_path((*info).path.as_ptr(), &mut (*info).filter)
    } else { -22 }
}

unsafe extern "C" fn bpf_mt(skb: *const sk_buff, par: *mut xt_action_param) -> bool {
    let info = (*par).matchinfo as *const xt_bpf_info;
    bpf_prog_run((*info).filter, skb)
}

unsafe extern "C" fn bpf_mt_v1(skb: *const sk_buff, par: *mut xt_action_param) -> bool {
    let info = (*par).matchinfo as *const xt_bpf_info_v1;
    bpf_prog_run_save_cb((*info).filter, skb as *mut sk_buff) != 0
}

unsafe extern "C" fn bpf_mt_destroy(par: *const xt_mtdtor_param) {
    let info = (*par).matchinfo as *const xt_bpf_info;
    bpf_prog_destroy((*info).filter);
}

unsafe extern "C" fn bpf_mt_destroy_v1(par: *const xt_mtdtor_param) {
    let info = (*par).matchinfo as *const xt_bpf_info_v1;
    bpf_prog_destroy((*info).filter);
}

static mut BPF_MT_REG: [xt_match; 2] = [
    xt_match { name: b"bpf\0".as_ptr() as *const _, revision: 0, family: NFPROTO_UNSPEC, checkentry: Some(bpf_mt_check), r#match: Some(bpf_mt), destroy: Some(bpf_mt_destroy), matchsize: size_of::<xt_bpf_info>(), usersize: offset_of!(xt_bpf_info, filter), me: core::ptr::null_mut() },
    xt_match { name: b"bpf\0".as_ptr() as *const _, revision: 1, family: NFPROTO_UNSPEC, checkentry: Some(bpf_mt_check_v1), r#match: Some(bpf_mt_v1), destroy: Some(bpf_mt_destroy_v1), matchsize: size_of::<xt_bpf_info_v1>(), usersize: offset_of!(xt_bpf_info_v1, filter), me: core::ptr::null_mut() },
];

unsafe extern "C" fn bpf_mt_init() -> i32 {
    xt_register_matches(BPF_MT_REG.as_mut_ptr(), BPF_MT_REG.len())
}

unsafe extern "C" fn bpf_mt_exit() {
    xt_unregister_matches(BPF_MT_REG.as_mut_ptr(), BPF_MT_REG.len());
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
