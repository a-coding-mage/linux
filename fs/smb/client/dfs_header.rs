/* SPDX-License-Identifier: GPL-2.0 */
/*
 * Copyright (c) 2022 Paulo Alcantara <palcantara@suse.de>
 */

// Dependencies supplied by the surrounding translation unit:
// cifsglob.h, cifsproto.h, fs_context.h, dfs_cache.h, cifs_unicode.h,
// linux/namei.h, and linux/errno.h.

#[inline]
pub const fn dfs_interlink(v: u32) -> bool {
    (v & DFSREF_REFERRAL_SERVER) != 0 && (v & DFSREF_STORAGE_SERVER) == 0
}

#[repr(C)]
pub struct dfs_ref {
    pub path: *mut core::ffi::c_char,
    pub full_path: *mut core::ffi::c_char,
    pub ses: *mut cifs_ses,
    pub tl: dfs_cache_tgt_list,
    pub tit: *mut dfs_cache_tgt_iterator,
}

#[repr(C)]
pub struct dfs_ref_walk {
    pub mnt_ctx: *mut cifs_mount_ctx,
    pub r#ref: *mut dfs_ref,
    pub refs: [dfs_ref; MAX_NESTED_LINKS],
}

#[inline]
pub unsafe fn ref_walk_start(w: *mut dfs_ref_walk) -> *mut dfs_ref { (*w).refs.as_mut_ptr() }
#[inline]
pub unsafe fn ref_walk_end(w: *mut dfs_ref_walk) -> *mut dfs_ref {
    (*w).refs.as_mut_ptr().add((*w).refs.len() - 1)
}
#[inline]
pub unsafe fn ref_walk_cur(w: *mut dfs_ref_walk) -> *mut dfs_ref { (*w).r#ref }
#[inline]
pub unsafe fn ref_walk_descend(w: *mut dfs_ref_walk) -> bool {
    (*w).r#ref = (*w).r#ref.sub(1);
    (*w).r#ref >= ref_walk_start(w)
}
#[inline]
pub unsafe fn ref_walk_tit(w: *mut dfs_ref_walk) -> *mut dfs_cache_tgt_iterator { (*ref_walk_cur(w)).tit }
#[inline]
pub unsafe fn ref_walk_path(w: *mut dfs_ref_walk) -> *mut core::ffi::c_char { (*ref_walk_cur(w)).path }
#[inline]
pub unsafe fn ref_walk_fpath(w: *mut dfs_ref_walk) -> *mut core::ffi::c_char { (*ref_walk_cur(w)).full_path }
#[inline]
pub unsafe fn ref_walk_tl(w: *mut dfs_ref_walk) -> *mut dfs_cache_tgt_list { &mut (*ref_walk_cur(w)).tl }
#[inline]
pub unsafe fn ref_walk_ses(w: *mut dfs_ref_walk) -> *mut cifs_ses { (*ref_walk_cur(w)).ses }

#[inline]
pub unsafe fn ref_walk_alloc() -> *mut dfs_ref_walk {
    let rw = kmalloc_obj::<dfs_ref_walk>();
    if rw.is_null() { ERR_PTR(-ENOMEM) } else { rw }
}

#[inline]
pub unsafe fn ref_walk_init(rw: *mut dfs_ref_walk, mnt_ctx: *mut cifs_mount_ctx) {
    core::ptr::write_bytes(rw, 0, 1);
    (*rw).mnt_ctx = mnt_ctx;
    (*rw).r#ref = ref_walk_start(rw);
}

#[inline]
pub unsafe fn __ref_walk_free(r: *mut dfs_ref) {
    kfree((*r).path);
    kfree((*r).full_path);
    dfs_cache_free_tgts(&mut (*r).tl);
    if !(*r).ses.is_null() { cifs_put_smb_ses((*r).ses); }
    core::ptr::write_bytes(r, 0, 1);
}

#[inline]
pub unsafe fn ref_walk_free(rw: *mut dfs_ref_walk) {
    if rw.is_null() { return; }
    let mut r = ref_walk_start(rw);
    while r <= ref_walk_end(rw) { __ref_walk_free(r); r = r.add(1); }
    kfree(rw);
}

#[inline]
pub unsafe fn ref_walk_advance(rw: *mut dfs_ref_walk) -> i32 {
    let r = ref_walk_cur(rw).add(1);
    if r > ref_walk_end(rw) { return -ELOOP; }
    __ref_walk_free(r);
    (*rw).r#ref = r;
    0
}

#[inline]
pub unsafe fn ref_walk_next_tgt(rw: *mut dfs_ref_walk) -> *mut dfs_cache_tgt_iterator {
    let r = ref_walk_cur(rw);
    if IS_ERR((*r).tit) { return core::ptr::null_mut(); }
    let tit = if (*r).tit.is_null() { dfs_cache_get_tgt_iterator(&mut (*r).tl) } else { dfs_cache_get_next_tgt(&mut (*r).tl, (*r).tit) };
    if tit.is_null() { (*r).tit = ERR_PTR(-ENOENT); return core::ptr::null_mut(); }
    (*r).tit = tit; tit
}

#[inline]
pub unsafe fn ref_walk_get_tgt(rw: *mut dfs_ref_walk, tgt: *mut dfs_info3_param) -> i32 {
    zfree_dfs_info_param(tgt);
    dfs_cache_get_tgt_referral(ref_walk_path(rw).add(1), ref_walk_tit(rw), tgt)
}

#[inline]
pub unsafe fn ref_walk_set_tgt_hint(rw: *mut dfs_ref_walk) { dfs_cache_noreq_update_tgthint(ref_walk_path(rw).add(1), ref_walk_tit(rw)); }

#[inline]
pub unsafe fn ref_walk_set_tcon(rw: *mut dfs_ref_walk, tcon: *mut cifs_tcon) {
    let mut r = ref_walk_start(rw);
    while r <= ref_walk_cur(rw) {
        if (*r).ses.is_null() { r = r.add(1); continue; }
        list_add(&mut (*(*r).ses).dlist, &mut (*tcon).dfs_ses_list);
        (*r).ses = core::ptr::null_mut(); r = r.add(1);
    }
}

#[inline]
pub unsafe fn ref_walk_mark_end(rw: *mut dfs_ref_walk) {
    let r = ref_walk_cur(rw).sub(1);
    dfs_cache_noreq_update_tgthint((*r).path.add(1), (*r).tit);
    (*r).tit = ERR_PTR(-ENOENT);
}

extern "C" {
    pub fn dfs_parse_target_referral(full_path: *const core::ffi::c_char, r: *const dfs_info3_param, ctx: *mut smb3_fs_context) -> i32;
    pub fn dfs_mount_share(mnt_ctx: *mut cifs_mount_ctx) -> i32;
}

#[inline]
pub unsafe fn dfs_get_path(sb: *mut cifs_sb_info, path: *const core::ffi::c_char) -> *mut core::ffi::c_char { dfs_cache_canonical_path(path, (*sb).local_nls, cifs_remap(sb)) }

#[inline]
pub unsafe fn dfs_get_referral(mnt_ctx: *mut cifs_mount_ctx, path: *const core::ffi::c_char, tl: *mut dfs_cache_tgt_list) -> i32 {
    let ctx = (*mnt_ctx).fs_ctx;
    let sb = (*mnt_ctx).cifs_sb;
    let rses = if !(*ctx).dfs_root_ses.is_null() { (*ctx).dfs_root_ses } else { (*mnt_ctx).ses };
    dfs_cache_find((*mnt_ctx).xid, rses, (*sb).local_nls, cifs_remap(sb), path, core::ptr::null(), tl)
}

#[inline]
pub unsafe fn dfs_put_root_smb_sessions(head: *mut list_head) {
    let mut ses: *mut cifs_ses = core::ptr::null_mut();
    let mut n: *mut cifs_ses = core::ptr::null_mut();
    list_for_each_entry_safe(&mut ses, &mut n, head);
    // The loop body is expressed by the surrounding list iteration primitive.
    while !ses.is_null() { list_del_init(&mut (*ses).dlist); cifs_put_smb_ses(ses); ses = n; }
}

#[inline]
pub unsafe fn dfs_ses_refpath(ses: *mut cifs_ses) -> *const core::ffi::c_char {
    let path = (*(*ses).server).leaf_fullpath;
    if !path.is_null() { path.add(1) } else { ERR_PTR(-ENOENT) }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
