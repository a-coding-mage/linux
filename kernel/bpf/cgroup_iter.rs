// SPDX-License-Identifier: GPL-2.0-only
/* Copyright (c) 2022 Google */

/* Kernel dependencies are supplied by the surrounding build. */

/* cgroup_iter provides five modes of traversal to the cgroup hierarchy.
 *
 *  1. Walk the descendants of a cgroup in pre-order.
 *  2. Walk the descendants of a cgroup in post-order.
 *  3. Walk the ancestors of a cgroup.
 *  4. Show the given cgroup only.
 *  5. Walk the children of a given parent cgroup.
 *
 * For walking descendants, cgroup_iter can walk in either pre-order or
 * post-order. For walking ancestors, the iter walks up from a cgroup to
 * the root.
 *
 * The iter program can terminate the walk early by returning 1. Walk
 * continues if prog returns 0.
 *
 * The prog can check (seq->num == 0) to determine whether this is
 * the first element. The prog may also be passed a NULL cgroup,
 * which means the walk has completed and the prog has a chance to
 * do post-processing, such as outputting an epilogue.
 *
 * Note: the iter_prog is called with cgroup_mutex held.
 *
 * Currently only one session is supported, which means, depending on the
 * volume of data bpf program intends to send to user space, the number of
 * cgroups that can be walked is limited. For example, given the current
 * buffer size is 8 * PAGE_SIZE, if the program sends 64B data for each
 * cgroup, assuming PAGE_SIZE is 4kb, the total number of cgroups that can
 * be walked is 512. This is a limitation of cgroup_iter. If the output data
 * is larger than the kernel buffer size, after all data in the kernel buffer
 * is consumed by user space, the subsequent read() syscall will signal
 * EOPNOTSUPP. In order to work around, the user may have to update their
 * program to reduce the volume of data sent to output. For example, skip
 * some uninteresting cgroups.
 */

#[repr(C)]
pub struct BpfIterCgroup {
    pub meta: *mut BpfIterMeta,
    pub cgroup: *mut Cgroup,
}

#[repr(C)]
pub struct CgroupIterPriv {
    pub start_css: *mut CgroupSubsysState,
    pub visited_all: bool,
    pub terminate: bool,
    pub order: i32,
}

#[repr(C)]
pub struct SeqFile {
    _private: [u8; 0],
}
#[repr(C)]
pub struct CgroupSubsysState {
    pub parent: *mut CgroupSubsysState,
    pub cgroup: *mut Cgroup,
}
#[repr(C)]
pub struct Cgroup {
    pub self_: CgroupSubsysState,
}
#[repr(C)]
pub struct BpfIterMeta {
    pub seq: *mut SeqFile,
}
#[repr(C)]
pub struct BpfProg {
    _private: [u8; 0],
}
#[repr(C)]
pub struct BpfIterAuxInfo {
    pub cgroup: BpfIterCgroupInfo,
}
#[repr(C)]
pub struct BpfIterCgroupInfo {
    pub start: *mut Cgroup,
    pub order: i32,
}
#[repr(C)]
pub union BpfIterLinkInfo {
    pub cgroup: BpfIterCgroupLinkInfo,
}
#[repr(C)]
pub struct BpfIterCgroupLinkInfo {
    pub cgroup_fd: i32,
    pub cgroup_id: u64,
    pub order: i32,
}
#[repr(C)]
pub struct SeqOperations {
    pub start: Option<unsafe extern "C" fn(*mut SeqFile, *mut i64) -> *mut core::ffi::c_void>,
    pub next: Option<unsafe extern "C" fn(*mut SeqFile, *mut core::ffi::c_void, *mut i64) -> *mut core::ffi::c_void>,
    pub stop: Option<unsafe extern "C" fn(*mut SeqFile, *mut core::ffi::c_void)>,
    pub show: Option<unsafe extern "C" fn(*mut SeqFile, *mut core::ffi::c_void) -> i32>,
}
#[repr(C)]
pub struct BpfIterSeqInfo {
    pub seq_ops: *const SeqOperations,
    pub init_seq_private: Option<unsafe extern "C" fn(*mut core::ffi::c_void, *mut BpfIterAuxInfo) -> i32>,
    pub fini_seq_private: Option<unsafe extern "C" fn(*mut core::ffi::c_void)>,
    pub seq_priv_size: usize,
}

extern "C" {
    fn cgroup_lock();
    fn cgroup_unlock();
    fn css_next_descendant_pre(pos: *mut CgroupSubsysState, root: *mut CgroupSubsysState) -> *mut CgroupSubsysState;
    fn css_next_descendant_post(pos: *mut CgroupSubsysState, root: *mut CgroupSubsysState) -> *mut CgroupSubsysState;
    fn css_next_child(pos: *mut CgroupSubsysState, parent: *mut CgroupSubsysState) -> *mut CgroupSubsysState;
    fn cgroup_is_dead(cgrp: *mut Cgroup) -> bool;
    fn bpf_iter_get_info(meta: *mut BpfIterMeta, in_stop: i32) -> *mut BpfProg;
    fn bpf_iter_run_prog(prog: *mut BpfProg, ctx: *mut BpfIterCgroup) -> i32;
    fn css_get(css: *mut CgroupSubsysState);
    fn css_put(css: *mut CgroupSubsysState);
    fn cgroup_v1v2_get_from_fd(fd: i32) -> *mut Cgroup;
    fn cgroup_get_from_id(id: u64) -> *mut Cgroup;
    fn cgroup_get_from_path(path: *const u8) -> *mut Cgroup;
    fn cgroup_put(cgrp: *mut Cgroup);
}

const BPF_CGROUP_ITER_DESCENDANTS_PRE: i32 = 0;
const BPF_CGROUP_ITER_DESCENDANTS_POST: i32 = 1;
const BPF_CGROUP_ITER_ANCESTORS_UP: i32 = 2;
const BPF_CGROUP_ITER_SELF_ONLY: i32 = 3;
const BPF_CGROUP_ITER_CHILDREN: i32 = 4;
const EINVAL: i32 = 22;
const EOPNOTSUPP: i32 = 95;

pub unsafe extern "C" fn cgroup_iter_seq_start(seq: *mut SeqFile, pos: *mut i64) -> *mut core::ffi::c_void {
    let p = (*seq).private as *mut CgroupIterPriv;
    cgroup_lock();
    if *pos > 0 {
        if (*p).visited_all { return core::ptr::null_mut(); }
        return (-EOPNOTSUPP) as isize as *mut core::ffi::c_void;
    }
    *pos += 1;
    (*p).terminate = false;
    (*p).visited_all = false;
    if (*p).order == BPF_CGROUP_ITER_DESCENDANTS_PRE {
        css_next_descendant_pre(core::ptr::null_mut(), (*p).start_css) as *mut _
    } else if (*p).order == BPF_CGROUP_ITER_DESCENDANTS_POST {
        css_next_descendant_post(core::ptr::null_mut(), (*p).start_css) as *mut _
    } else if (*p).order == BPF_CGROUP_ITER_CHILDREN {
        css_next_child(core::ptr::null_mut(), (*p).start_css) as *mut _
    } else { (*p).start_css as *mut _ }
}

unsafe extern "C" { fn __cgroup_iter_seq_show(seq: *mut SeqFile, css: *mut CgroupSubsysState, in_stop: i32) -> i32; }

pub unsafe extern "C" fn cgroup_iter_seq_stop(seq: *mut SeqFile, v: *mut core::ffi::c_void) {
    let p = (*seq).private as *mut CgroupIterPriv;
    cgroup_unlock();
    if v.is_null() { __cgroup_iter_seq_show(seq, core::ptr::null_mut(), true); (*p).visited_all = true; }
}

pub unsafe extern "C" fn cgroup_iter_seq_next(seq: *mut SeqFile, v: *mut core::ffi::c_void, pos: *mut i64) -> *mut core::ffi::c_void {
    let curr = v as *mut CgroupSubsysState;
    let p = (*seq).private as *mut CgroupIterPriv;
    *pos += 1;
    if (*p).terminate { return core::ptr::null_mut(); }
    if (*p).order == BPF_CGROUP_ITER_DESCENDANTS_PRE { css_next_descendant_pre(curr, (*p).start_css) as *mut _ }
    else if (*p).order == BPF_CGROUP_ITER_DESCENDANTS_POST { css_next_descendant_post(curr, (*p).start_css) as *mut _ }
    else if (*p).order == BPF_CGROUP_ITER_ANCESTORS_UP { (*curr).parent as *mut _ }
    else if (*p).order == BPF_CGROUP_ITER_CHILDREN { css_next_child(curr, (*p).start_css) as *mut _ }
    else { core::ptr::null_mut() }
}

pub unsafe extern "C" fn __cgroup_iter_seq_show_impl(seq: *mut SeqFile, css: *mut CgroupSubsysState, in_stop: i32) -> i32 {
    let p = (*seq).private as *mut CgroupIterPriv;
    if !css.is_null() && cgroup_is_dead((*css).cgroup) { return 0; }
    let mut meta = BpfIterMeta { seq };
    let mut ctx = BpfIterCgroup { meta: &mut meta, cgroup: if css.is_null() { core::ptr::null_mut() } else { (*css).cgroup } };
    let prog = bpf_iter_get_info(&mut meta, in_stop);
    let ret = if prog.is_null() { 0 } else { bpf_iter_run_prog(prog, &mut ctx) };
    if ret != 0 { (*p).terminate = true; }
    0
}

pub unsafe extern "C" fn cgroup_iter_seq_show(seq: *mut SeqFile, v: *mut core::ffi::c_void) -> i32 {
    __cgroup_iter_seq_show_impl(seq, v as *mut CgroupSubsysState, false as i32)
}

pub static CgroupIterSeqOps: SeqOperations = SeqOperations { start: Some(cgroup_iter_seq_start), next: Some(cgroup_iter_seq_next), stop: Some(cgroup_iter_seq_stop), show: Some(cgroup_iter_seq_show) };

pub unsafe extern "C" fn cgroup_iter_seq_init(priv_: *mut core::ffi::c_void, aux: *mut BpfIterAuxInfo) -> i32 {
    let p = priv_ as *mut CgroupIterPriv;
    let cgrp = (*aux).cgroup.start;
    (*p).start_css = &mut (*cgrp).self_;
    css_get((*p).start_css);
    (*p).terminate = false;
    (*p).visited_all = false;
    (*p).order = (*aux).cgroup.order;
    0
}

pub unsafe extern "C" fn cgroup_iter_seq_fini(priv_: *mut core::ffi::c_void) {
    css_put((*(priv_ as *mut CgroupIterPriv)).start_css);
}

pub static CgroupIterSeqInfo: BpfIterSeqInfo = BpfIterSeqInfo { seq_ops: &CgroupIterSeqOps, init_seq_private: Some(cgroup_iter_seq_init), fini_seq_private: Some(cgroup_iter_seq_fini), seq_priv_size: core::mem::size_of::<CgroupIterPriv>() };

pub unsafe extern "C" fn bpf_iter_attach_cgroup(prog: *mut BpfProg, linfo: *mut BpfIterLinkInfo, aux: *mut BpfIterAuxInfo) -> i32 {
    let fd = (*linfo).cgroup.cgroup_fd;
    let id = (*linfo).cgroup.cgroup_id;
    let order = (*linfo).cgroup.order;
    let cgrp;
    match order { BPF_CGROUP_ITER_DESCENDANTS_PRE | BPF_CGROUP_ITER_DESCENDANTS_POST | BPF_CGROUP_ITER_ANCESTORS_UP | BPF_CGROUP_ITER_SELF_ONLY | BPF_CGROUP_ITER_CHILDREN => {}, _ => return -EINVAL }
    if fd != 0 && id != 0 { return -EINVAL; }
    if fd != 0 { cgrp = cgroup_v1v2_get_from_fd(fd); }
    else if id != 0 { cgrp = cgroup_get_from_id(id); }
    else { cgrp = cgroup_get_from_path(b"/\0".as_ptr()); }
    (*aux).cgroup.start = cgrp;
    (*aux).cgroup.order = order;
    0
}

pub unsafe extern "C" fn bpf_iter_detach_cgroup(aux: *mut BpfIterAuxInfo) {
    cgroup_put((*aux).cgroup.start);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
