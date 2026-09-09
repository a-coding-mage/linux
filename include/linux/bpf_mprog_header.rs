/* SPDX-License-Identifier: GPL-2.0 */
/* Copyright (c) 2023 Isovalent */

/* Translated from bpf_mprog.h. Kernel-provided types and helpers are external dependencies. */

pub const BPF_MPROG_MAX: usize = 64;

#[repr(C)]
pub struct bpf_mprog_fp {
    pub prog: *mut bpf_prog,
}

#[repr(C)]
pub struct bpf_mprog_cp {
    pub link: *mut bpf_link,
}

#[repr(C)]
pub struct bpf_mprog_entry {
    pub fp_items: [bpf_mprog_fp; BPF_MPROG_MAX],
    pub parent: *mut bpf_mprog_bundle,
}

#[repr(C)]
pub struct bpf_mprog_bundle {
    pub a: bpf_mprog_entry,
    pub b: bpf_mprog_entry,
    pub cp_items: [bpf_mprog_cp; BPF_MPROG_MAX],
    pub ref_: *mut bpf_prog,
    pub revision: atomic64_t,
    pub count: u32,
}

#[repr(C)]
pub struct bpf_tuple {
    pub prog: *mut bpf_prog,
    pub link: *mut bpf_link,
}

extern "C" {
    pub fn bpf_prog_put(prog: *mut bpf_prog);
    pub fn bpf_net_capable() -> bool;
}

extern "C" {
    pub fn atomic64_set(v: *mut atomic64_t, i: i64);
    pub fn atomic64_inc(v: *mut atomic64_t);
    pub fn atomic64_read(v: *const atomic64_t) -> u64;
}

extern "C" {
    pub type bpf_prog;
    pub type bpf_link;
    pub type atomic64_t;
    pub type bpf_attr;
    pub type bpf_prog_type;
}

pub const BPF_PROG_TYPE_SCHED_CLS: bpf_prog_type = 3 as bpf_prog_type;

#[inline]
pub unsafe fn bpf_mprog_peer(entry: *const bpf_mprog_entry) -> *mut bpf_mprog_entry {
    if entry == &(*(*entry).parent).a as *const _ {
        &mut (*(*entry).parent).b
    } else {
        &mut (*(*entry).parent).a
    }
}

#[inline]
pub unsafe fn bpf_mprog_bundle_init(bundle: *mut bpf_mprog_bundle) {
    core::ptr::write_bytes(bundle as *mut u8, 0, core::mem::size_of::<bpf_mprog_bundle>());
    atomic64_set(&mut (*bundle).revision, 1);
    (*bundle).a.parent = bundle;
    (*bundle).b.parent = bundle;
}

#[inline]
pub unsafe fn bpf_mprog_inc(entry: *mut bpf_mprog_entry) { (*(*entry).parent).count += 1; }

#[inline]
pub unsafe fn bpf_mprog_dec(entry: *mut bpf_mprog_entry) { (*(*entry).parent).count -= 1; }

#[inline]
pub const fn bpf_mprog_max() -> i32 { (BPF_MPROG_MAX - 1) as i32 }

#[inline]
pub unsafe fn bpf_mprog_total(entry: *mut bpf_mprog_entry) -> i32 {
    (*(*entry).parent).count as i32
}

#[inline]
pub unsafe fn bpf_mprog_exists(entry: *mut bpf_mprog_entry, prog: *const bpf_prog) -> bool {
    for i in 0..BPF_MPROG_MAX {
        let tmp = core::ptr::read_volatile((*entry).fp_items[i].prog);
        if tmp == prog as *mut bpf_prog { return true; }
        if tmp.is_null() { break; }
    }
    false
}

#[inline]
pub unsafe fn bpf_mprog_mark_for_release(entry: *mut bpf_mprog_entry, tuple: *mut bpf_tuple) {
    if (*tuple).link.is_null() { (*(*entry).parent).ref_ = (*tuple).prog; }
}

#[inline]
pub unsafe fn bpf_mprog_complete_release(entry: *mut bpf_mprog_entry) {
    if !(*(*entry).parent).ref_.is_null() {
        bpf_prog_put((*(*entry).parent).ref_);
        (*(*entry).parent).ref_ = core::ptr::null_mut();
    }
}

#[inline]
pub unsafe fn bpf_mprog_revision_new(entry: *mut bpf_mprog_entry) {
    atomic64_inc(&mut (*(*entry).parent).revision);
}

#[inline]
pub unsafe fn bpf_mprog_commit(entry: *mut bpf_mprog_entry) {
    bpf_mprog_complete_release(entry);
    bpf_mprog_revision_new(entry);
}

#[inline]
pub unsafe fn bpf_mprog_revision(entry: *mut bpf_mprog_entry) -> u64 {
    atomic64_read(&(*(*entry).parent).revision)
}

#[inline]
pub unsafe fn bpf_mprog_entry_copy(dst: *mut bpf_mprog_entry, src: *mut bpf_mprog_entry) {
    core::ptr::copy_nonoverlapping((*src).fp_items.as_ptr(), (*dst).fp_items.as_mut_ptr(), BPF_MPROG_MAX);
}

#[inline]
pub unsafe fn bpf_mprog_entry_clear(dst: *mut bpf_mprog_entry) {
    core::ptr::write_bytes((*dst).fp_items.as_mut_ptr(), 0, BPF_MPROG_MAX);
}

#[inline]
pub unsafe fn bpf_mprog_clear_all(entry: *mut bpf_mprog_entry, entry_new: *mut *mut bpf_mprog_entry) {
    let peer = bpf_mprog_peer(entry);
    bpf_mprog_entry_clear(peer);
    (*(*peer).parent).count = 0;
    *entry_new = peer;
}

#[inline]
pub unsafe fn bpf_mprog_entry_grow(entry: *mut bpf_mprog_entry, idx: usize) {
    let total = bpf_mprog_total(entry) as usize;
    core::ptr::copy((*entry).fp_items.as_ptr().add(idx), (*entry).fp_items.as_mut_ptr().add(idx + 1), total - idx);
    core::ptr::copy((*(*entry).parent).cp_items.as_ptr().add(idx), (*(*entry).parent).cp_items.as_mut_ptr().add(idx + 1), total - idx);
}

#[inline]
pub unsafe fn bpf_mprog_entry_shrink(entry: *mut bpf_mprog_entry, idx: usize) {
    core::ptr::copy((*entry).fp_items.as_ptr().add(idx + 1), (*entry).fp_items.as_mut_ptr().add(idx), BPF_MPROG_MAX - idx - 1);
    core::ptr::copy((*(*entry).parent).cp_items.as_ptr().add(idx + 1), (*(*entry).parent).cp_items.as_mut_ptr().add(idx), BPF_MPROG_MAX - idx - 1);
}

#[inline]
pub unsafe fn bpf_mprog_read(entry: *mut bpf_mprog_entry, idx: usize, fp: *mut *mut bpf_mprog_fp, cp: *mut *mut bpf_mprog_cp) {
    *fp = (*entry).fp_items.as_mut_ptr().add(idx);
    *cp = (*(*entry).parent).cp_items.as_mut_ptr().add(idx);
}

#[inline]
pub unsafe fn bpf_mprog_write(fp: *mut bpf_mprog_fp, cp: *mut bpf_mprog_cp, tuple: *mut bpf_tuple) {
    core::ptr::write_volatile(&mut (*fp).prog, (*tuple).prog);
    (*cp).link = (*tuple).link;
}

extern "C" {
    pub fn bpf_mprog_attach(entry: *mut bpf_mprog_entry, entry_new: *mut *mut bpf_mprog_entry, prog_new: *mut bpf_prog, link: *mut bpf_link, prog_old: *mut bpf_prog, flags: u32, id_or_fd: u32, revision: u64) -> i32;
    pub fn bpf_mprog_detach(entry: *mut bpf_mprog_entry, entry_new: *mut *mut bpf_mprog_entry, prog: *mut bpf_prog, link: *mut bpf_link, flags: u32, id_or_fd: u32, revision: u64) -> i32;
    pub fn bpf_mprog_query(attr: *const bpf_attr, uattr: *mut bpf_attr, entry: *mut bpf_mprog_entry) -> i32;
}

#[inline]
pub fn bpf_mprog_supported(type_: bpf_prog_type) -> bool { type_ == BPF_PROG_TYPE_SCHED_CLS }

#[inline]
pub unsafe fn bpf_mprog_detach_empty(type_: bpf_prog_type) -> bool {
    if type_ == BPF_PROG_TYPE_SCHED_CLS { bpf_net_capable() } else { false }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
