// SPDX-License-Identifier: GPL-2.0-only
/* Copyright (c) 2020 Facebook */

// Kernel headers and build-time macros are supplied by the surrounding Rust
// kernel bindings. Their declarations are intentionally not reimplemented here.

#[repr(C)]
pub struct bpf_iter_target_info {
    pub list: list_head,
    pub reg_info: *const bpf_iter_reg,
    pub btf_id: u32,
}

#[repr(C)]
pub struct bpf_iter_link {
    pub link: bpf_link,
    pub aux: bpf_iter_aux_info,
    pub tinfo: *mut bpf_iter_target_info,
}

#[repr(C)]
pub struct bpf_iter_priv_data {
    pub tinfo: *mut bpf_iter_target_info,
    pub seq_info: *const bpf_iter_seq_info,
    pub prog: *mut bpf_prog,
    pub session_id: u64,
    pub seq_num: u64,
    pub done_stop: bool,
    pub target_private: [u8; 0],
}

static mut targets: list_head = LIST_HEAD_INIT!(targets);
static mut targets_mutex: mutex = DEFINE_MUTEX!();
static mut link_mutex: mutex = DEFINE_MUTEX!();
static mut session_id: atomic64_t = atomic64_t::new(0);

extern "C" {
    fn prepare_seq_file(file: *mut file, link: *mut bpf_iter_link) -> i32;
}

unsafe fn bpf_iter_inc_seq_num(seq: *mut seq_file) {
    let iter_priv = container_of!((*seq).private, bpf_iter_priv_data, target_private);
    (*iter_priv).seq_num += 1;
}

unsafe fn bpf_iter_dec_seq_num(seq: *mut seq_file) {
    let iter_priv = container_of!((*seq).private, bpf_iter_priv_data, target_private);
    (*iter_priv).seq_num -= 1;
}

unsafe fn bpf_iter_done_stop(seq: *mut seq_file) {
    let iter_priv = container_of!((*seq).private, bpf_iter_priv_data, target_private);
    (*iter_priv).done_stop = true;
}

#[inline]
unsafe fn bpf_iter_target_support_resched(tinfo: *const bpf_iter_target_info) -> bool {
    (*(*tinfo).reg_info).feature & BPF_ITER_RESCHED != 0
}

unsafe fn bpf_iter_support_resched(seq: *mut seq_file) -> bool {
    let iter_priv = container_of!((*seq).private, bpf_iter_priv_data, target_private);
    bpf_iter_target_support_resched((*iter_priv).tinfo)
}

const MAX_ITER_OBJECTS: i32 = 1000000;

unsafe fn bpf_seq_read(file: *mut file, buf: *mut core::ffi::c_char, size: usize,
                       ppos: *mut loff_t) -> isize {
    let seq = (*file).private_data as *mut seq_file;
    let mut n: usize;
    let mut offs: usize;
    let mut copied: usize = 0;
    let mut err: i32 = 0;
    let mut num_objs: i32 = 0;
    let can_resched: bool;
    let mut p: *mut core::ffi::c_void;

    mutex_lock(&mut (*seq).lock);
    if (*seq).buf.is_null() {
        (*seq).size = PAGE_SIZE << 3;
        (*seq).buf = kvmalloc((*seq).size, GFP_KERNEL);
        if (*seq).buf.is_null() { err = -ENOMEM; goto!('done); }
    }
    if (*seq).count != 0 {
        n = core::cmp::min((*seq).count, size);
        err = copy_to_user(buf, (*seq).buf.add((*seq).from), n);
        if err != 0 { err = -EFAULT; goto!('done); }
        (*seq).count -= n; (*seq).from += n; copied = n; goto!('done);
    }
    (*seq).from = 0;
    p = ((*(*seq).op).start)(seq, &mut (*seq).index);
    if p.is_null() { goto!('stop); }
    if IS_ERR!(p) { err = PTR_ERR!(p); ((*(*seq).op).stop)(seq, p); (*seq).count = 0; goto!('done); }
    err = ((*(*seq).op).show)(seq, p);
    if err > 0 { bpf_iter_dec_seq_num(seq); (*seq).count = 0; }
    else if err < 0 || seq_has_overflowed(seq) { if err == 0 { err = -E2BIG; } ((*(*seq).op).stop)(seq, p); (*seq).count = 0; goto!('done); }
    can_resched = bpf_iter_support_resched(seq);
    loop {
        let pos = (*seq).index;
        num_objs += 1; offs = (*seq).count;
        p = ((*(*seq).op).next)(seq, p, &mut (*seq).index);
        if pos == (*seq).index { pr_info_ratelimited!("buggy seq_file .next function %ps did not updated position index\n", (*(*seq).op).next); (*seq).index += 1; }
        if IS_ERR_OR_NULL!(p) { break; }
        bpf_iter_inc_seq_num(seq);
        if (*seq).count >= size { break; }
        if num_objs >= MAX_ITER_OBJECTS { if offs == 0 { err = -EAGAIN; ((*(*seq).op).stop)(seq, p); goto!('done); } break; }
        err = ((*(*seq).op).show)(seq, p);
        if err > 0 { bpf_iter_dec_seq_num(seq); (*seq).count = offs; }
        else if err < 0 || seq_has_overflowed(seq) { (*seq).count = offs; if offs == 0 { if err == 0 { err = -E2BIG; } ((*(*seq).op).stop)(seq, p); goto!('done); } break; }
        if can_resched { cond_resched(); }
    }
    offs = (*seq).count;
    if IS_ERR!(p) { ((*(*seq).op).stop)(seq, core::ptr::null_mut()); err = PTR_ERR!(p); goto!('done); }
    ((*(*seq).op).stop)(seq, p);
    if p.is_null() { if !seq_has_overflowed(seq) { bpf_iter_done_stop(seq); } else { (*seq).count = offs; if offs == 0 { err = -E2BIG; goto!('done); } } }
    n = core::cmp::min((*seq).count, size);
    err = copy_to_user(buf, (*seq).buf, n);
    if err != 0 { err = -EFAULT; goto!('done); }
    copied = n; (*seq).count -= n; (*seq).from = n;
    goto!('done);
    label!('done);
    let result = if copied == 0 { err as isize } else { *ppos += copied as loff_t; copied as isize };
    mutex_unlock(&mut (*seq).lock); result
}

unsafe fn __get_seq_info(link: *mut bpf_iter_link) -> *const bpf_iter_seq_info {
    if !(*link).aux.map.is_null() {
        let info = (*(*(*link).aux.map).ops).iter_seq_info;
        if !info.is_null() { return info; }
    }
    (*(*link).tinfo).reg_info.as_ref().unwrap().seq_info
}

unsafe fn iter_open(inode: *mut inode, file: *mut file) -> i32 { prepare_seq_file(file, (*inode).i_private as *mut bpf_iter_link) }

unsafe fn iter_release(inode: *mut inode, file: *mut file) -> i32 {
    let seq = (*file).private_data as *mut seq_file; if seq.is_null() { return 0; }
    let iter_priv = container_of!((*seq).private, bpf_iter_priv_data, target_private);
    if let Some(f) = (*(*iter_priv).seq_info).fini_seq_private { f((*seq).private); }
    bpf_prog_put((*iter_priv).prog); (*seq).private = iter_priv as *mut _; seq_release_private(inode, file)
}

pub static bpf_iter_fops: file_operations = file_operations { open: Some(iter_open), read: Some(bpf_seq_read), release: Some(iter_release), ..file_operations::ZERO };

// The remaining exported kernel entry points retain their C ABI and are
// declared here for the surrounding bindings to provide their field layouts.
extern "C" {
    pub fn bpf_iter_reg_target(reg_info: *const bpf_iter_reg) -> i32;
    pub fn bpf_iter_unreg_target(reg_info: *const bpf_iter_reg);
    pub fn bpf_iter_prog_supported(prog: *mut bpf_prog) -> i32;
    pub fn bpf_iter_get_func_proto(func_id: bpf_func_id, prog: *const bpf_prog) -> *const bpf_func_proto;
    pub fn bpf_link_is_iter(link: *mut bpf_link) -> bool;
    pub fn bpf_iter_link_attach(attr: *const bpf_attr, uattr: bpfptr_t, prog: *mut bpf_prog) -> i32;
    pub fn bpf_iter_new_fd(link: *mut bpf_link) -> i32;
    pub fn bpf_iter_get_info(meta: *mut bpf_iter_meta, in_stop: bool) -> *mut bpf_prog;
    pub fn bpf_iter_run_prog(prog: *mut bpf_prog, ctx: *mut core::ffi::c_void) -> i32;
}

#[repr(C, align(8))]
pub struct bpf_iter_num_kern { pub cur: i32, pub end: i32 }

pub unsafe fn bpf_iter_num_new(it: *mut bpf_iter_num, start: i32, end: i32) -> i32 {
    let s = it as *mut bpf_iter_num_kern;
    if start > end { (*s).cur = 0; (*s).end = 0; return -EINVAL; }
    if (end.wrapping_sub(start) as u32) > BPF_MAX_LOOPS { (*s).cur = 0; (*s).end = 0; return -E2BIG; }
    (*s).cur = start.wrapping_sub(1); (*s).end = end; 0
}

pub unsafe fn bpf_iter_num_next(it: *mut bpf_iter_num) -> *mut i32 {
    let s = it as *mut bpf_iter_num_kern;
    if (*s).cur.wrapping_add(1) >= (*s).end { (*s).cur = 0; (*s).end = 0; return core::ptr::null_mut(); }
    (*s).cur += 1; &mut (*s).cur
}

pub unsafe fn bpf_iter_num_destroy(_it: *mut bpf_iter_num) { /* no-op */ }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
