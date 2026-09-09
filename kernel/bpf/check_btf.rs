// SPDX-License-Identifier: GPL-2.0-only
/* Copyright (c) 2026 Meta Platforms, Inc. and affiliates. */

// Dependencies supplied by the surrounding kernel/BPF codebase.
use core::ffi::{c_int, c_void};

const MIN_BPF_FUNCINFO_SIZE: u32 = 8;
const MAX_FUNCINFO_REC_SIZE: u32 = 252;
const MIN_BPF_LINEINFO_SIZE: usize = 0; // offsetofend(struct bpf_line_info, line_col)
const MAX_LINEINFO_REC_SIZE: u32 = MAX_FUNCINFO_REC_SIZE;
const MIN_CORE_RELO_SIZE: usize = 0; // sizeof(struct bpf_core_relo)
const MAX_CORE_RELO_SIZE: u32 = MAX_FUNCINFO_REC_SIZE;

#[repr(C)]
pub struct bpf_verifier_env { pub subprog_cnt: u32, pub subprog_info: *mut bpf_subprog_info, pub prog: *mut bpf_prog, pub log: bpf_log }
#[repr(C)] pub struct bpf_subprog_info { pub has_ld_abs: bool, pub has_tail_call: bool, pub start: u32, pub linfo_idx: u32, pub name: *const u8 }
#[repr(C)] pub struct bpf_prog { pub aux: *mut bpf_prog_aux, pub len: u32, pub insnsi: *mut bpf_insn }
#[repr(C)] pub struct bpf_prog_aux { pub btf: *const btf, pub func_info: *mut bpf_func_info, pub func_info_cnt: u32, pub func_info_aux: *mut bpf_func_info_aux, pub linfo: *mut bpf_line_info, pub nr_linfo: u32 }
#[repr(C)] pub struct bpf_log { _private: [u8; 0] }
#[repr(C)] pub struct bpf_attr { pub func_info_cnt: u32, pub func_info_rec_size: u32, pub func_info: u64, pub line_info_cnt: u32, pub line_info_rec_size: u32, pub line_info: u64, pub core_relo_cnt: u32, pub core_relo_rec_size: u32, pub core_relos: u64, pub prog_btf_fd: u32 }
#[repr(C)] pub struct bpfptr_t { pub ptr: u64, pub is_kernel: bool }
#[repr(C)] pub struct bpf_func_info { pub insn_off: u32, pub type_id: u32 }
#[repr(C)] pub struct bpf_func_info_aux { pub linkage: u32 }
#[repr(C)] pub struct bpf_line_info { pub insn_off: u32, pub file_name_off: u32, pub line_off: u32, pub line_col: u32 }
#[repr(C)] pub struct bpf_core_relo { pub insn_off: u32, pub type_id: u32, pub access_str_off: u32, pub kind: u32 }
#[repr(C)] pub struct bpf_insn { pub code: u8, pub rest: [u8; 7] }
#[repr(C)] pub struct btf { _private: [u8; 0] }
#[repr(C)] pub struct btf_type { pub name_off: u32, pub info: u32, pub type_: u32 }
#[repr(C)] pub struct bpf_core_ctx { pub log: *mut bpf_log, pub btf: *const btf }

extern "C" {
    fn bpf_verifier_log_write(env: *mut bpf_verifier_env, fmt: *const u8, ...);
    fn make_bpfptr(ptr: u64, is_kernel: bool) -> bpfptr_t;
    fn bpf_check_uarg_tail_zero(ptr: bpfptr_t, expected: u32, actual: u32) -> c_int;
    fn copy_to_bpfptr_offset(ptr: bpfptr_t, offset: usize, src: *const c_void, size: usize) -> c_int;
    fn copy_from_bpfptr(dst: *mut c_void, src: bpfptr_t, size: u32) -> c_int;
    fn bpfptr_add(ptr: *mut bpfptr_t, size: u32);
    fn kvcalloc(n: usize, size: usize, flags: u64) -> *mut c_void;
    fn kvzalloc(size: usize, flags: u64) -> *mut c_void;
    fn kvfree(ptr: *mut c_void);
    fn kfree(ptr: *mut c_void);
    fn btf_type_by_id(btf: *const btf, id: u32) -> *const btf_type;
    fn btf_type_is_func(ty: *const btf_type) -> bool;
    fn btf_type_is_func_proto(ty: *const btf_type) -> bool;
    fn btf_type_skip_modifiers(btf: *const btf, id: u32, t: *mut *const btf_type) -> *const btf_type;
    fn btf_type_is_small_int(ty: *const btf_type) -> bool;
    fn btf_is_any_enum(ty: *const btf_type) -> bool;
    fn btf_name_by_offset(btf: *const btf, off: u32) -> *const u8;
    fn btf_get_by_fd(fd: u32) -> *mut btf;
    fn btf_is_kernel(btf: *const btf) -> bool;
    fn btf_put(btf: *mut btf);
    fn bpf_core_apply(ctx: *mut bpf_core_ctx, relo: *const bpf_core_relo, idx: u32, insn: *mut bpf_insn) -> c_int;
}

unsafe fn check_abnormal_return(env: *mut bpf_verifier_env) -> c_int {
    for i in 1..(*env).subprog_cnt {
        let sub = &*(*env).subprog_info.add(i as usize);
        if sub.has_ld_abs || sub.has_tail_call { return -22; }
    }
    0
}

pub unsafe fn bpf_prepare_btf_info(env: *mut bpf_verifier_env, attr: *const bpf_attr, uattr: bpfptr_t) -> c_int {
    if (*attr).func_info_cnt == 0 && (*attr).line_info_cnt == 0 { return check_abnormal_return(env); }
    let btf = btf_get_by_fd((*attr).prog_btf_fd);
    if btf.is_null() || btf_is_kernel(btf) { if !btf.is_null() { btf_put(btf); } return -13; }
    (*(*env).prog).aux.as_mut().unwrap().btf = btf;
    prepare_btf_func(env, attr, uattr)
}

unsafe fn prepare_btf_func(env: *mut bpf_verifier_env, attr: *const bpf_attr, uattr: bpfptr_t) -> c_int {
    if (*attr).func_info_cnt == 0 { return check_abnormal_return(env); }
    let prog = (*env).prog; let btf = (*(*prog).aux).btf;
    let n = (*attr).func_info_cnt as usize;
    let rec = kvcalloc(n, core::mem::size_of::<bpf_func_info>(), 0);
    if rec.is_null() { return -12; }
    let k = rec as *mut bpf_func_info; let mut prev = 0;
    let mut urec = make_bpfptr((*attr).func_info, uattr.is_kernel);
    for i in 0..n {
        if bpf_check_uarg_tail_zero(urec, 8, (*attr).func_info_rec_size) != 0 { kvfree(rec); return -22; }
        if copy_from_bpfptr(k.add(i) as *mut c_void, urec, 8) != 0 { kvfree(rec); return -14; }
        if (i == 0 && (*k.add(i)).insn_off != 0) || (i != 0 && (*k.add(i)).insn_off <= prev) { kvfree(rec); return -22; }
        let ty = btf_type_by_id(btf, (*k.add(i)).type_id);
        if ty.is_null() || !btf_type_is_func(ty) || { let p = btf_type_by_id(btf, (*ty).type_); p.is_null() || !btf_type_is_func_proto(p) } { kvfree(rec); return -22; }
        prev = (*k.add(i)).insn_off; bpfptr_add(&mut urec, (*attr).func_info_rec_size);
    }
    (*(*prog).aux).func_info = k; (*(*prog).aux).func_info_cnt = (*attr).func_info_cnt; 0
}

pub unsafe fn bpf_check_btf_info(env: *mut bpf_verifier_env, attr: *const bpf_attr, _uattr: bpfptr_t) -> c_int {
    if (*attr).func_info_cnt == 0 && (*attr).line_info_cnt == 0 { return check_abnormal_return(env); }
    let r = check_btf_func(env, attr); if r != 0 { return r; }
    let r = check_btf_line(env, attr); if r != 0 { return r; }
    check_core_relo(env, attr)
}

unsafe fn check_btf_func(env: *mut bpf_verifier_env, attr: *const bpf_attr) -> c_int {
    if (*attr).func_info_cnt == 0 { return check_abnormal_return(env); }
    if (*attr).func_info_cnt != (*env).subprog_cnt { return -22; }
    let prog = (*env).prog; let aux = &mut *(*prog).aux; let btf = aux.btf;
    let rec = aux.func_info; let info = kvcalloc((*attr).func_info_cnt as usize, core::mem::size_of::<bpf_func_info_aux>(), 0) as *mut bpf_func_info_aux;
    if info.is_null() { return -12; }
    for i in 0..(*attr).func_info_cnt as usize {
        if (*(*env).subprog_info.add(i)).start != (*rec.add(i)).insn_off { kfree(info as *mut c_void); return -22; }
        let ty = btf_type_by_id(btf, (*rec.add(i)).type_id); (*info.add(i)).linkage = (*ty).info;
        let proto = btf_type_by_id(btf, (*ty).type_); let ret = btf_type_skip_modifiers(btf, (*proto).type_, core::ptr::null_mut());
        let scalar = btf_type_is_small_int(ret) || btf_is_any_enum(ret);
        if i != 0 && !scalar && ((*env).subprog_info.add(i).as_ref().unwrap().has_ld_abs || (*env).subprog_info.add(i).as_ref().unwrap().has_tail_call) { kfree(info as *mut c_void); return -22; }
        (*env).subprog_info.add(i).as_mut().unwrap().name = btf_name_by_offset(btf, (*ty).name_off);
    }
    aux.func_info_aux = info; 0
}

unsafe fn check_btf_line(_env: *mut bpf_verifier_env, attr: *const bpf_attr) -> c_int {
    if (*attr).line_info_cnt == 0 { return 0; }
    if (*attr).line_info_rec_size < MIN_BPF_LINEINFO_SIZE as u32 || (*attr).line_info_rec_size > MAX_LINEINFO_REC_SIZE || (*attr).line_info_rec_size % 4 != 0 { return -22; }
    0
}

unsafe fn check_core_relo(_env: *mut bpf_verifier_env, attr: *const bpf_attr) -> c_int {
    if (*attr).core_relo_cnt == 0 { return 0; }
    if (*attr).core_relo_rec_size < MIN_CORE_RELO_SIZE as u32 || (*attr).core_relo_rec_size > MAX_CORE_RELO_SIZE || (*attr).core_relo_rec_size % 4 != 0 { return -22; }
    0
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
