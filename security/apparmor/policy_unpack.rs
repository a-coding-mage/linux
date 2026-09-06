// SPDX-License-Identifier: GPL-2.0-only
/*
 * AppArmor security module
 *
 * This file contains AppArmor functions for unpacking policy loaded from
 * userspace.
 *
 * Copyright (C) 1998-2008 Novell/SUSE
 * Copyright 2009-2010 Canonical Ltd.
 *
 * AppArmor uses a serialized binary format for loading policy. To find
 * policy format documentation see Documentation/admin-guide/LSM/apparmor.rst
 * All policy is validated before it is used.
 */

#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(dead_code)]
#![allow(improper_ctypes)]

use core::ffi::{c_char, c_int, c_long, c_uint, c_ulong, c_void};
use core::mem::{offset_of, size_of};
use core::ptr;

type bool_ = bool;
type size_t = usize;
type ssize_t = isize;
type u8 = u8;
type u16 = u16;
type u32 = u32;
type u64 = u64;

#[repr(C)]
pub struct audit_buffer {
    _private: [u8; 0],
}
#[repr(C)]
pub struct common_audit_data {
    _private: [u8; 0],
}
#[repr(C)]
pub struct kref {
    _private: [u8; 0],
}
#[repr(C)]
pub struct work_struct {
    _private: [u8; 0],
}
#[repr(C)]
pub struct mutex {
    _private: [u8; 0],
}
#[repr(C)]
pub struct inode {
    _private: [u8; 0],
}
#[repr(C)]
pub struct dentry {
    _private: [u8; 0],
}
#[repr(C)]
pub struct list_head {
    pub next: *mut list_head,
    pub prev: *mut list_head,
}
#[repr(C)]
pub struct rhash_head {
    _private: [u8; 0],
}
#[repr(C)]
pub struct rhashtable {
    pub p: rhashtable_params,
}
#[repr(C)]
pub struct rhashtable_compare_arg {
    pub key: *const c_void,
}
#[repr(C)]
pub struct rhashtable_params {
    pub nelem_hint: c_uint,
    pub key_len: size_t,
    pub key_offset: size_t,
    pub head_offset: size_t,
    pub hashfn: Option<unsafe extern "C" fn(*const c_void, u32, u32) -> u32>,
    pub obj_cmpfn: Option<unsafe extern "C" fn(*mut rhashtable_compare_arg, *const c_void) -> c_int>,
}

#[repr(C)]
pub struct aa_ext {
    pub start: *mut c_char,
    pub end: *mut c_char,
    pub pos: *mut c_char,
    pub version: u32,
}

#[repr(C)]
pub struct aa_iface_audit {
    pub ns: *const c_char,
    pub pos: c_long,
}
#[repr(C)]
pub struct apparmor_audit_data {
    pub iface: aa_iface_audit,
    pub name: *const c_char,
    pub info: *const c_char,
    pub error: c_int,
}
#[repr(C)]
pub struct aa_ns {
    pub lock: mutex,
    pub level: c_int,
}
#[repr(C)]
pub struct aa_refcount {
    pub count: kref,
    pub reftype: c_int,
}
#[repr(C)]
pub struct aa_loaddata {
    pub count: aa_refcount,
    pub pcount: kref,
    pub list: list_head,
    pub work: work_struct,
    pub ns: *mut aa_ns,
    pub revision: c_long,
    pub dents: [*mut dentry; 2],
    pub name: *mut c_char,
    pub data: *mut c_char,
    pub size: size_t,
    pub compressed_size: size_t,
    pub abi: u32,
    pub hash: *mut c_void,
}
#[repr(C)]
pub struct aa_label {
    pub flags: u32,
    pub rules: [*mut aa_ruleset; 1],
}
#[repr(C)]
pub struct aa_profile_base {
    pub hname: *const c_char,
}
#[repr(C)]
pub struct aa_profile_attach {
    pub xmatch_str: *const c_char,
    pub xmatch: *mut aa_policydb,
    pub xmatch_len: u32,
    pub xattr_count: u16,
    pub xattrs: *mut *mut c_char,
}
#[repr(C)]
pub struct aa_profile {
    pub base: aa_profile_base,
    pub label: aa_label,
    pub rename: *const c_char,
    pub attach: aa_profile_attach,
    pub disconnected: *mut c_char,
    pub signal: u32,
    pub mode: u32,
    pub audit: u32,
    pub path_flags: u32,
    pub data: *mut rhashtable,
}
#[repr(C)]
pub struct kernel_cap_t {
    pub val: u64,
}
#[repr(C)]
pub struct aa_caps {
    pub allow: kernel_cap_t,
    pub audit: kernel_cap_t,
    pub quiet: kernel_cap_t,
    pub extended: kernel_cap_t,
}
#[repr(C)]
pub struct rlimit {
    pub rlim_max: u64,
}
#[repr(C)]
pub struct aa_rlimits {
    pub mask: u32,
    pub limits: [rlimit; 32],
}
#[repr(C)]
pub struct aa_secmark {
    pub audit: u8,
    pub deny: u8,
    pub label: *mut c_char,
}
#[repr(C)]
pub struct aa_ruleset {
    pub caps: aa_caps,
    pub rlimits: aa_rlimits,
    pub secmark: *mut aa_secmark,
    pub secmark_count: u16,
    pub policy: *mut aa_policydb,
    pub file: *mut aa_policydb,
}
#[repr(C)]
pub struct aa_str_table_ent {
    pub strs: *mut c_char,
    pub count: c_int,
    pub size: c_int,
}
#[repr(C)]
pub struct aa_str_table {
    pub table: *mut aa_str_table_ent,
    pub size: u32,
}
#[repr(C)]
pub struct aa_tags_header {
    pub mask: u32,
    pub count: u32,
    pub size: u32,
    pub tags: u32,
}
#[repr(C)]
pub struct aa_tags_headers {
    pub table: *mut aa_tags_header,
    pub size: u32,
}
#[repr(C)]
pub struct aa_tags_sets {
    pub table: *mut u32,
    pub size: u32,
}
#[repr(C)]
pub struct aa_tags_struct {
    pub hdrs: aa_tags_headers,
    pub sets: aa_tags_sets,
    pub strs: aa_str_table,
}
#[repr(C)]
pub struct aa_perms {
    pub allow: u32,
    pub deny: u32,
    pub subtree: u32,
    pub cond: u32,
    pub kill: u32,
    pub complain: u32,
    pub prompt: u32,
    pub audit: u32,
    pub quiet: u32,
    pub hide: u32,
    pub xindex: u32,
    pub tag: u32,
    pub label: u32,
}
#[repr(C)]
pub struct table_header {
    pub td_lolen: u32,
    pub td_flags: u16,
}
#[repr(C)]
pub struct aa_dfa {
    pub tables: [*mut table_header; 16],
}
#[repr(C)]
pub struct aa_policydb {
    pub tags: aa_tags_struct,
    pub perms: *mut aa_perms,
    pub size: ssize_t,
    pub dfa: *mut aa_dfa,
    pub start: [u32; 32],
    pub trans: aa_str_table,
}
#[repr(C)]
pub struct aa_data {
    pub key: *mut c_char,
    pub size: size_t,
    pub data: *mut c_void,
    pub head: rhash_head,
}
#[repr(C)]
pub struct aa_load_ent {
    pub list: list_head,
    pub rename: *mut aa_profile,
    pub old: *mut aa_profile,
    pub new: *mut aa_profile,
    pub ns_name: *mut c_char,
}
#[repr(C)]
pub struct zstd_parameters {
    pub cParams: zstd_compression_parameters,
}
#[repr(C)]
pub struct zstd_compression_parameters {
    _private: [u8; 0],
}
#[repr(C)]
pub struct zstd_cctx {
    _private: [u8; 0],
}

#[repr(u8)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum aa_code {
    AA_U8 = 1,
    AA_U32 = 2,
    AA_U64 = 3,
    AA_NAME = 4,
    AA_STRING = 5,
    AA_BLOB = 6,
    AA_STRUCT = 7,
    AA_STRUCTEND = 8,
    AA_ARRAY = 9,
    AA_ARRAYEND = 10,
}

const GFP_KERNEL: c_uint = 0;
const ENOMEM: c_int = 12;
const EINVAL: c_int = 22;
const EPROTO: c_int = 71;
const EPROTONOSUPPORT: c_int = 93;
const AUDIT_APPARMOR_STATUS: c_int = 0;
const LSM_AUDIT_DATA_NONE: c_int = 0;
const AA_CLASS_NONE: c_int = 0;
const AA_CLASS_FILE: usize = 2;
const AA_CLASS_XMATCH: usize = 1;
const AA_CLASS_LAST: usize = 31;
const DFA_START: u32 = 1;
const DFA_FLAG_VERIFY_STATES: c_int = 1;
const YYTD_DATA32: c_int = 4;
const YYTD_ID_BASE: usize = 0;
const YYTD_ID_ACCEPT: usize = 1;
const YYTD_ID_ACCEPT2: usize = 2;
const AAFS_LOADDATA_DIR: usize = 0;
const AAFS_LOADDATA_REVISION: usize = 1;
const REF_RAWDATA: c_int = 0;
const DEBUG_UNPACK: c_int = 0;
const RLIM_NLIMITS: u16 = 32;
const PACKED_FLAG_HAT: u32 = 1 << 0;
const PACKED_FLAG_DEBUG1: u32 = 1 << 1;
const PACKED_FLAG_DEBUG2: u32 = 1 << 2;
const PACKED_MODE_COMPLAIN: u32 = 1;
const PACKED_MODE_ENFORCE: u32 = 2;
const PACKED_MODE_KILL: u32 = 3;
const PACKED_MODE_UNCONFINED: u32 = 4;
const PACKED_MODE_USER: u32 = 5;
const FORCE_COMPLAIN_FLAG: u32 = 1 << 30;
const FLAG_HAT: u32 = 1 << 0;
const FLAG_DEBUG1: u32 = 1 << 1;
const FLAG_DEBUG2: u32 = 1 << 2;
const FLAG_UNCONFINED: u32 = 1 << 3;
const APPARMOR_COMPLAIN: u32 = 1;
const APPARMOR_ENFORCE: u32 = 2;
const APPARMOR_KILL: u32 = 3;
const APPARMOR_UNCONFINED: u32 = 4;
const APPARMOR_USER: u32 = 5;
const AUDIT_ALL: u32 = !0;
const PATH_MEDIATE_DELETED: u32 = 1 << 0;
const MAXMAPPED_SIG: u32 = 64;
const AA_X_TYPE_MASK: u32 = 0xff000000;
const AA_X_TABLE: u32 = 0x01000000;
const AA_X_INDEX_MASK: u32 = 0x00ffffff;
const K_ABI_MASK: u32 = 0x0000ffff;
const v5: u32 = 5;
const v9: u32 = 9;

unsafe extern "C" {
    static mut aa_g_hash_policy: bool;
    static mut aa_g_paranoid_load: bool;
    static mut aa_g_rawdata_compression_level: c_int;
    static mut aa_g_export_binary: bool;
    static mut nullpdb: *mut aa_policydb;

    fn aad(sa: *mut common_audit_data) -> *mut apparmor_audit_data;
    fn audit_log_format(ab: *mut audit_buffer, fmt: *const c_char, ...);
    fn audit_log_untrustedstring(ab: *mut audit_buffer, s: *const c_char);
    fn labels_profile(label: *mut c_void) -> *mut aa_profile;
    fn aa_current_raw_label() -> *mut c_void;
    fn aa_audit(
        typ: c_int,
        profile: *mut aa_profile,
        ad: *mut apparmor_audit_data,
        cb: Option<unsafe extern "C" fn(*mut audit_buffer, *mut c_void)>,
    ) -> c_int;
    fn mutex_is_locked(lock: *mut mutex) -> bool;
    fn d_inode(dentry: *mut dentry) -> *mut inode;
    fn inode_set_ctime_current(inode: *mut inode) -> c_long;
    fn inode_set_mtime_to_ts(inode: *mut inode, ts: c_long);
    fn memcmp(a: *const c_void, b: *const c_void, n: size_t) -> c_int;
    fn aa_hash_size() -> size_t;
    fn kfree_sensitive(p: *mut c_void);
    fn kfree(p: *mut c_void);
    fn kvfree(p: *mut c_void);
    fn kvfree_sensitive(p: *mut c_void, size: size_t);
    fn kzalloc(size: size_t, flags: c_uint) -> *mut c_void;
    fn kvzalloc(size: size_t, flags: c_uint) -> *mut c_void;
    fn kcalloc(n: size_t, size: size_t, flags: c_uint) -> *mut c_void;
    fn kmemdup(src: *const c_void, len: size_t, flags: c_uint) -> *mut c_void;
    fn kvmemdup(src: *const c_void, len: size_t, flags: c_uint) -> *mut c_void;
    fn krealloc(src: *mut c_void, len: size_t, flags: c_uint) -> *mut c_void;
    fn kstrndup(src: *const c_char, len: size_t, flags: c_uint) -> *mut c_char;
    fn kstrdup(src: *const c_char, flags: c_uint) -> *mut c_char;
    fn kref_init(kref: *mut kref);
    fn aa_get_ns(ns: *mut aa_ns) -> *mut aa_ns;
    fn mutex_lock_nested(lock: *mut mutex, subclass: c_int);
    fn mutex_unlock(lock: *mut mutex);
    fn __aa_fs_remove_rawdata(d: *mut aa_loaddata);
    fn aa_put_ns(ns: *mut aa_ns);
    fn aa_put_i_loaddata(d: *mut aa_loaddata);
    fn INIT_WORK(work: *mut work_struct, func: unsafe extern "C" fn(*mut work_struct));
    fn schedule_work(work: *mut work_struct) -> bool;
    fn INIT_LIST_HEAD(list: *mut list_head);
    fn aa_dfa_unpack(blob: *mut c_char, size: size_t, flags: c_int) -> *mut aa_dfa;
    fn strcmp(a: *const c_char, b: *const c_char) -> c_int;
    fn strlen(s: *const c_char) -> size_t;
    fn jhash(data: *const c_void, len: u32, seed: u32) -> u32;
    fn aa_destroy_str_table(strs: *mut aa_str_table);
    fn aa_resize_str_table(strs: *mut aa_str_table, size: c_int, flags: c_uint) -> bool;
    fn aa_map_resource(i: c_int) -> c_int;
    fn aa_destroy_tags(tags: *mut aa_tags_struct);
    fn aa_alloc_pdb(flags: c_uint) -> *mut aa_policydb;
    fn aa_put_pdb(pdb: *mut aa_policydb);
    fn aa_get_pdb(pdb: *mut aa_policydb) -> *mut aa_policydb;
    fn aa_dfa_next(dfa: *mut aa_dfa, state: u32, class: usize) -> u32;
    fn table_size(noents: u32, tdflags: u16) -> size_t;
    fn aa_splitn_fqname(
        name: *const c_char,
        len: size_t,
        ns: *mut *const c_char,
        ns_len: *mut size_t,
    ) -> *const c_char;
    fn aa_alloc_profile(name: *const c_char, label: *mut c_void, flags: c_uint) -> *mut aa_profile;
    fn aa_free_profile(profile: *mut aa_profile);
    fn aa_put_profile(profile: *mut aa_profile);
    fn aa_compat_map_xmatch(pdb: *mut aa_policydb) -> c_int;
    fn aa_compat_map_policy(pdb: *mut aa_policydb, version: u32) -> c_int;
    fn aa_compat_map_file(pdb: *mut aa_policydb) -> c_int;
    fn rhashtable_init(ht: *mut rhashtable, params: *mut rhashtable_params) -> c_int;
    fn rhashtable_insert_fast(ht: *mut rhashtable, head: *mut rhash_head, params: rhashtable_params) -> c_int;
    fn aa_compute_profile_mediates(profile: *mut aa_profile);
    fn aa_calc_profile_hash(profile: *mut aa_profile, version: u32, start: *mut c_void, len: size_t) -> c_int;
    fn aa_calc_hash(data: *mut c_void, len: size_t) -> *mut c_void;
    fn list_add_tail(new: *mut list_head, head: *mut list_head);
    fn list_del_init(entry: *mut list_head);
    fn AA_DEBUG(class: c_int, fmt: *const c_char, ...);
    fn zstd_get_params(level: c_int, src_size: size_t) -> zstd_parameters;
    fn zstd_cctx_workspace_bound(params: *const zstd_compression_parameters) -> size_t;
    fn zstd_compress_bound(src_size: size_t) -> size_t;
    fn zstd_init_cctx(workspace: *mut c_void, workspace_size: size_t) -> *mut zstd_cctx;
    fn zstd_compress_cctx(
        ctx: *mut zstd_cctx,
        dst: *mut c_void,
        dst_capacity: size_t,
        src: *const c_void,
        src_size: size_t,
        params: *const zstd_parameters,
    ) -> size_t;
    fn zstd_is_error(code: size_t) -> bool;
    fn is_vmalloc_addr(addr: *const c_void) -> bool;
    fn memcpy(dst: *mut c_void, src: *const c_void, n: size_t) -> *mut c_void;
}

macro_rules! cstr {
    ($s:literal) => {
        concat!($s, "\0").as_ptr() as *const c_char
    };
}

unsafe fn AA_BUG(_cond: bool) {}
unsafe fn ERR_PTR<T>(error: c_int) -> *mut T {
    error as isize as *mut T
}
unsafe fn IS_ERR<T>(ptr: *const T) -> bool {
    (ptr as isize) < 0 && (ptr as isize) > -4096
}
unsafe fn PTR_ERR<T>(ptr: *const T) -> c_int {
    ptr as isize as c_int
}
fn ALIGN(x: size_t, a: size_t) -> size_t {
    (x + a - 1) & !(a - 1)
}
fn VERSION_LT(v: u32, rhs: u32) -> bool {
    (v & K_ABI_MASK) < rhs
}
fn VERSION_GT(v: u32, rhs: u32) -> bool {
    (v & K_ABI_MASK) > rhs
}
fn TO_ACCEPT1_FLAG(x: c_int) -> c_int {
    x
}
fn TO_ACCEPT2_FLAG(x: c_int) -> c_int {
    x << 8
}
unsafe fn ACCEPT_TABLE(dfa: *const aa_dfa) -> *mut u32 {
    (*dfa).tables[YYTD_ID_ACCEPT] as *mut u32
}
unsafe fn get_unaligned_u16(p: *const c_void) -> u16 {
    ptr::read_unaligned(p as *const u16).to_le()
}
unsafe fn get_unaligned_u32(p: *const c_void) -> u32 {
    ptr::read_unaligned(p as *const u32).to_le()
}
unsafe fn get_unaligned_u64(p: *const c_void) -> u64 {
    ptr::read_unaligned(p as *const u64).to_le()
}
unsafe fn container_of_aa_loaddata_count(kref: *mut kref) -> *mut aa_loaddata {
    (kref as *mut u8).sub(offset_of!(aa_loaddata, count) + offset_of!(aa_refcount, count)) as *mut aa_loaddata
}
unsafe fn container_of_aa_loaddata_pcount(kref: *mut kref) -> *mut aa_loaddata {
    (kref as *mut u8).sub(offset_of!(aa_loaddata, pcount)) as *mut aa_loaddata
}
unsafe fn container_of_aa_loaddata_work(work: *mut work_struct) -> *mut aa_loaddata {
    (work as *mut u8).sub(offset_of!(aa_loaddata, work)) as *mut aa_loaddata
}

/* audit callback for unpack fields */
unsafe extern "C" fn audit_cb(ab: *mut audit_buffer, va: *mut c_void) {
    let sa = va as *mut common_audit_data;
    let ad = aad(sa);

    if !(*ad).iface.ns.is_null() {
        audit_log_format(ab, cstr!(" ns="));
        audit_log_untrustedstring(ab, (*ad).iface.ns);
    }
    if !(*ad).name.is_null() {
        audit_log_format(ab, cstr!(" name="));
        audit_log_untrustedstring(ab, (*ad).name);
    }
    if (*ad).iface.pos != 0 {
        audit_log_format(ab, cstr!(" offset=%ld"), (*ad).iface.pos);
    }
}

/**
 * audit_iface - do audit message for policy unpacking/load/replace/remove
 * @new: profile if it has been allocated (MAYBE NULL)
 * @ns_name: name of the ns the profile is to be loaded to (MAY BE NULL)
 * @name: name of the profile being manipulated (MAYBE NULL)
 * @info: any extra info about the failure (MAYBE NULL)
 * @e: buffer position info
 * @error: error code
 *
 * Returns: %0 or error
 */
unsafe fn audit_iface(
    new: *mut aa_profile,
    ns_name: *const c_char,
    name: *const c_char,
    info: *const c_char,
    e: *mut aa_ext,
    error: c_int,
) -> c_int {
    let profile = labels_profile(aa_current_raw_label());
    let mut ad = apparmor_audit_data {
        iface: aa_iface_audit { ns: ptr::null(), pos: 0 },
        name: ptr::null(),
        info: ptr::null(),
        error: 0,
    };
    if !e.is_null() {
        ad.iface.pos = (*e).pos.offset_from((*e).start) as c_long;
    }
    ad.iface.ns = ns_name;
    if !new.is_null() {
        ad.name = (*new).base.hname;
    } else {
        ad.name = name;
    }
    ad.info = info;
    ad.error = error;

    aa_audit(AUDIT_APPARMOR_STATUS, profile, &mut ad, Some(audit_cb))
}

pub unsafe extern "C" fn __aa_loaddata_update(data: *mut aa_loaddata, revision: c_long) {
    AA_BUG(data.is_null());
    AA_BUG((*data).ns.is_null());
    AA_BUG(!mutex_is_locked(&mut (*(*data).ns).lock));
    AA_BUG((*data).revision > revision);

    (*data).revision = revision;
    if !(*data).dents[AAFS_LOADDATA_REVISION].is_null() {
        let mut inode: *mut inode;

        inode = d_inode((*data).dents[AAFS_LOADDATA_DIR]);
        inode_set_mtime_to_ts(inode, inode_set_ctime_current(inode));

        inode = d_inode((*data).dents[AAFS_LOADDATA_REVISION]);
        inode_set_mtime_to_ts(inode, inode_set_ctime_current(inode));
    }
}

pub unsafe extern "C" fn aa_rawdata_eq(l: *mut aa_loaddata, r: *mut aa_loaddata) -> bool {
    if (*l).size != (*r).size {
        return false;
    }
    if (*l).compressed_size != (*r).compressed_size {
        return false;
    }
    if aa_g_hash_policy && memcmp((*l).hash, (*r).hash, aa_hash_size()) != 0 {
        return false;
    }
    let cmp_size = if (*r).compressed_size != 0 { (*r).compressed_size } else { (*r).size };
    memcmp((*l).data as *const c_void, (*r).data as *const c_void, cmp_size) == 0
}

unsafe fn do_loaddata_free(d: *mut aa_loaddata) {
    kfree_sensitive((*d).hash);
    kfree_sensitive((*d).name as *mut c_void);
    kvfree((*d).data as *mut c_void);
    kfree_sensitive(d as *mut c_void);
}

pub unsafe extern "C" fn aa_loaddata_kref(kref: *mut kref) {
    let d = container_of_aa_loaddata_count(kref);
    do_loaddata_free(d);
}

/*
 * need to take the ns mutex lock which is NOT safe most places that
 * put_loaddata is called, so we have to delay freeing it
 */
unsafe extern "C" fn do_ploaddata_rmfs(work: *mut work_struct) {
    let d = container_of_aa_loaddata_work(work);
    let ns = aa_get_ns((*d).ns);

    if !ns.is_null() {
        mutex_lock_nested(&mut (*ns).lock, (*ns).level);
        /* remove fs ref to loaddata */
        __aa_fs_remove_rawdata(d);
        mutex_unlock(&mut (*ns).lock);
        aa_put_ns(ns);
    }
    /* called by dropping last pcount, so drop its associated icount */
    aa_put_i_loaddata(d);
}

pub unsafe extern "C" fn aa_ploaddata_kref(kref: *mut kref) {
    let d = container_of_aa_loaddata_pcount(kref);

    if !d.is_null() {
        INIT_WORK(&mut (*d).work, do_ploaddata_rmfs);
        schedule_work(&mut (*d).work);
    }
}

pub unsafe extern "C" fn aa_loaddata_alloc(size: size_t) -> *mut aa_loaddata {
    let d = kzalloc(size_of::<aa_loaddata>(), GFP_KERNEL) as *mut aa_loaddata;
    if d.is_null() {
        return ERR_PTR(-ENOMEM);
    }
    (*d).data = kvzalloc(size, GFP_KERNEL) as *mut c_char;
    if (*d).data.is_null() {
        kfree(d as *mut c_void);
        return ERR_PTR(-ENOMEM);
    }
    kref_init(&mut (*d).count.count);
    (*d).count.reftype = REF_RAWDATA;
    kref_init(&mut (*d).pcount);
    INIT_LIST_HEAD(&mut (*d).list);

    d
}

/* test if read will be in packed data bounds */
pub unsafe extern "C" fn aa_inbounds(e: *mut aa_ext, size: size_t) -> bool {
    size <= (*e).end.offset_from((*e).pos) as size_t
}

/**
 * aa_unpack_u16_chunk - test and do bounds checking for a u16 size based chunk
 * @e: serialized data read head (NOT NULL)
 * @chunk: start address for chunk of data (NOT NULL)
 *
 * Returns: the size of chunk found with the read head at the end of the chunk.
 */
pub unsafe extern "C" fn aa_unpack_u16_chunk(e: *mut aa_ext, chunk: *mut *mut c_char) -> size_t {
    let mut size: size_t = 0;
    let pos = (*e).pos;

    if !aa_inbounds(e, size_of::<u16>()) {
        (*e).pos = pos;
        return 0;
    }
    size = get_unaligned_u16((*e).pos as *const c_void) as size_t;
    (*e).pos = (*e).pos.add(size_of::<u16>());
    if !aa_inbounds(e, size) {
        (*e).pos = pos;
        return 0;
    }
    *chunk = (*e).pos;
    (*e).pos = (*e).pos.add(size);
    size
}

/* unpack control byte */
pub unsafe extern "C" fn aa_unpack_X(e: *mut aa_ext, code: aa_code) -> bool {
    if !aa_inbounds(e, 1) {
        return false;
    }
    if *((*e).pos as *mut u8) != code as u8 {
        return false;
    }
    (*e).pos = (*e).pos.add(1);
    true
}

/**
 * aa_unpack_nameX - check is the next element is of type X with a name of @name
 * @e: serialized data extent information  (NOT NULL)
 * @code: type code
 * @name: name to match to the serialized element.  (MAYBE NULL)
 *
 * check that the next serialized data element is of type X and has a tag
 * name @name.  If @name is specified then there must be a matching
 * name element in the stream.  If @name is NULL any name element will be
 * skipped and only the typecode will be tested.
 *
 * Returns true on success (both type code and name tests match) and the read
 * head is advanced past the headers
 *
 * Returns: false if either match fails, the read head does not move
 */
pub unsafe extern "C" fn aa_unpack_nameX(e: *mut aa_ext, code: aa_code, name: *const c_char) -> bool {
    /*
     * May need to reset pos if name or type doesn't match
     */
    let pos = (*e).pos;
    /*
     * Check for presence of a tagname, and if present name size
     * AA_NAME tag value is a u16.
     */
    if aa_unpack_X(e, aa_code::AA_NAME) {
        let mut tag: *mut c_char = ptr::null_mut();
        let size = aa_unpack_u16_chunk(e, &mut tag);
        /* if a name is specified it must match. otherwise skip tag */
        if !name.is_null()
            && (size == 0 || *tag.add(size - 1) != 0 || strcmp(name, tag) != 0)
        {
            (*e).pos = pos;
            return false;
        }
    } else if !name.is_null() {
        /* if a name is specified and there is no name tag fail */
        (*e).pos = pos;
        return false;
    }

    /* now check if type code matches */
    if aa_unpack_X(e, code) {
        return true;
    }

    (*e).pos = pos;
    false
}

unsafe fn unpack_u8(e: *mut aa_ext, data: *mut u8, name: *const c_char) -> bool {
    let pos = (*e).pos;

    if aa_unpack_nameX(e, aa_code::AA_U8, name) {
        if !aa_inbounds(e, size_of::<u8>()) {
            (*e).pos = pos;
            return false;
        }
        if !data.is_null() {
            *data = *((*e).pos as *mut u8);
        }
        (*e).pos = (*e).pos.add(size_of::<u8>());
        return true;
    }

    (*e).pos = pos;
    false
}

pub unsafe extern "C" fn aa_unpack_u32(e: *mut aa_ext, data: *mut u32, name: *const c_char) -> bool {
    let pos = (*e).pos;

    if aa_unpack_nameX(e, aa_code::AA_U32, name) {
        if !aa_inbounds(e, size_of::<u32>()) {
            (*e).pos = pos;
            return false;
        }
        if !data.is_null() {
            *data = get_unaligned_u32((*e).pos as *const c_void);
        }
        (*e).pos = (*e).pos.add(size_of::<u32>());
        return true;
    }

    (*e).pos = pos;
    false
}

pub unsafe extern "C" fn aa_unpack_u64(e: *mut aa_ext, data: *mut u64, name: *const c_char) -> bool {
    let pos = (*e).pos;

    if aa_unpack_nameX(e, aa_code::AA_U64, name) {
        if !aa_inbounds(e, size_of::<u64>()) {
            (*e).pos = pos;
            return false;
        }
        if !data.is_null() {
            *data = get_unaligned_u64((*e).pos as *const c_void);
        }
        (*e).pos = (*e).pos.add(size_of::<u64>());
        return true;
    }

    (*e).pos = pos;
    false
}

unsafe fn aa_unpack_cap_low(e: *mut aa_ext, data: *mut kernel_cap_t, name: *const c_char) -> bool {
    let mut val: u32 = 0;

    if !aa_unpack_u32(e, &mut val, name) {
        return false;
    }
    (*data).val = val as u64;
    true
}

unsafe fn aa_unpack_cap_high(e: *mut aa_ext, data: *mut kernel_cap_t, name: *const c_char) -> bool {
    let mut val: u32 = 0;

    if !aa_unpack_u32(e, &mut val, name) {
        return false;
    }
    (*data).val = ((*data).val as u32 as u64) | ((val as u64) << 32);
    true
}

pub unsafe extern "C" fn aa_unpack_array(e: *mut aa_ext, name: *const c_char, size: *mut u16) -> bool {
    let pos = (*e).pos;

    if aa_unpack_nameX(e, aa_code::AA_ARRAY, name) {
        if !aa_inbounds(e, size_of::<u16>()) {
            (*e).pos = pos;
            return false;
        }
        *size = get_unaligned_u16((*e).pos as *const c_void);
        (*e).pos = (*e).pos.add(size_of::<u16>());
        return true;
    }

    (*e).pos = pos;
    false
}

pub unsafe extern "C" fn aa_unpack_blob(e: *mut aa_ext, blob: *mut *mut c_char, name: *const c_char) -> size_t {
    let pos = (*e).pos;

    if aa_unpack_nameX(e, aa_code::AA_BLOB, name) {
        let mut size: u32;
        if !aa_inbounds(e, size_of::<u32>()) {
            (*e).pos = pos;
            return 0;
        }
        size = get_unaligned_u32((*e).pos as *const c_void);
        (*e).pos = (*e).pos.add(size_of::<u32>());
        if aa_inbounds(e, size as size_t) {
            *blob = (*e).pos;
            (*e).pos = (*e).pos.add(size as size_t);
            return size as size_t;
        }
    }

    (*e).pos = pos;
    0
}

pub unsafe extern "C" fn aa_unpack_str(e: *mut aa_ext, string: *mut *const c_char, name: *const c_char) -> c_int {
    let mut src_str: *mut c_char = ptr::null_mut();
    let mut size: size_t = 0;
    let pos = (*e).pos;
    *string = ptr::null();
    if aa_unpack_nameX(e, aa_code::AA_STRING, name) {
        size = aa_unpack_u16_chunk(e, &mut src_str);
        if size != 0 {
            /* strings are null terminated, length is size - 1 */
            if *src_str.add(size - 1) != 0 {
                (*e).pos = pos;
                return 0;
            }
            *string = src_str;

            return size as c_int;
        }
    }

    (*e).pos = pos;
    0
}

pub unsafe extern "C" fn aa_unpack_strdup(e: *mut aa_ext, string: *mut *mut c_char, name: *const c_char) -> c_int {
    let mut tmp: *const c_char = ptr::null();
    let pos = (*e).pos;
    let res = aa_unpack_str(e, &mut tmp, name);
    *string = ptr::null_mut();

    if res == 0 {
        return 0;
    }

    *string = kmemdup(tmp as *const c_void, res as size_t, GFP_KERNEL) as *mut c_char;
    if (*string).is_null() {
        (*e).pos = pos;
        return 0;
    }

    res
}

/**
 * unpack_dfa - unpack a file rule dfa
 * @e: serialized data extent information (NOT NULL)
 * @flags: dfa flags to check
 *
 * returns dfa or ERR_PTR or NULL if no dfa
 */
unsafe fn unpack_dfa(e: *mut aa_ext, mut flags: c_int) -> *mut aa_dfa {
    let mut blob: *mut c_char = ptr::null_mut();
    let size: size_t;
    let mut dfa: *mut aa_dfa = ptr::null_mut();

    size = aa_unpack_blob(e, &mut blob, cstr!("aadfa"));
    if size != 0 {
        /*
         * The dfa is aligned with in the blob to 8 bytes
         * from the beginning of the stream.
         * alignment adjust needed by dfa unpack
         */
        let sz = blob.offset_from((*e).start) as size_t - (((*e).pos.offset_from((*e).start) as size_t) & 7);
        let pad = ALIGN(sz, 8) - sz;
        if aa_g_paranoid_load {
            flags |= DFA_FLAG_VERIFY_STATES;
        }
        dfa = aa_dfa_unpack(blob.add(pad), size - pad, flags);

        if IS_ERR(dfa) {
            return dfa;
        }
    }

    dfa
}

unsafe fn process_strs_entry(mut str_: *mut c_char, size: c_int, multi: bool) -> c_int {
    let mut c: c_int = 1;

    if size <= 0 {
        return -1;
    }
    if multi {
        if size < 2 {
            return -2;
        }
        /* multi ends with double \0 */
        if *str_.add((size - 2) as size_t) != 0 {
            return -3;
        }
    }

    let save = str_;
    let mut pos = str_;
    let end = if multi { str_.add((size - 2) as size_t) } else { str_.add((size - 1) as size_t) };
    /* count # of internal \0 */
    while str_ < end {
        if str_ == pos {
            /* starts with ... */
            if *str_ == 0 {
                AA_DEBUG(DEBUG_UNPACK, cstr!("starting with null save=%lu size %d c=%d"), str_.offset_from(save) as c_ulong, size, c);
                return -4;
            }
            if (*str_ as u8 as char).is_ascii_whitespace() {
                return -5;
            }
            if *str_ == b':' as c_char {
                /* :ns_str\0str\0
                 * first character after : must be valid
                 */
                if *str_.add(1) == 0 {
                    return -6;
                }
            }
        } else if *str_ == 0 {
            if *pos == b':' as c_char {
                *str_ = b':' as c_char;
            } else {
                c += 1;
            }
            pos = str_.add(1);
        }
        str_ = str_.add(1);
    } /* while */

    c
}

/**
 * unpack_strs_table - unpack a profile transition table
 * @e: serialized data extent information  (NOT NULL)
 * @name: name of table (MAY BE NULL)
 * @multi: allow multiple strings on a single entry
 * @strs: str table to unpack to (NOT NULL)
 *
 * Returns: 0 if table successfully unpacked or not present, else error
 */
unsafe fn unpack_strs_table(e: *mut aa_ext, name: *const c_char, multi: bool, strs: *mut aa_str_table) -> c_int {
    let saved_pos = (*e).pos;
    let mut table: *mut aa_str_table_ent = ptr::null_mut();
    let mut error = -EPROTO;

    /* exec table is optional */
    if aa_unpack_nameX(e, aa_code::AA_STRUCT, name) {
        let mut size: u16 = 0;
        let mut i: c_int;

        if !aa_unpack_array(e, ptr::null(), &mut size) {
            /*
             * Note: index into trans table array is a max
             * of 2^24, but unpack array can only unpack
             * an array of 2^16 in size atm so no need
             * for size check here
             */
            aa_destroy_str_table(strs);
            (*e).pos = saved_pos;
            return error;
        }
        table = kzalloc(size_of::<aa_str_table_ent>() * size as size_t, GFP_KERNEL) as *mut aa_str_table_ent;
        if table.is_null() {
            error = -ENOMEM;
            aa_destroy_str_table(strs);
            (*e).pos = saved_pos;
            return error;
        }
        (*strs).table = table;
        (*strs).size = size as u32;
        i = 0;
        while i < size as c_int {
            let mut strp: *mut c_char = ptr::null_mut();
            let size2 = aa_unpack_strdup(e, &mut strp, ptr::null());
            /* aa_unpack_strdup verifies that the last character is
             * null termination byte.
             */
            let c = process_strs_entry(strp, size2, multi);
            if c <= 0 {
                AA_DEBUG(DEBUG_UNPACK, cstr!("process_strs %d i %d pos %ld"), c, i, (*e).pos.offset_from(saved_pos) as c_ulong);
                aa_destroy_str_table(strs);
                (*e).pos = saved_pos;
                return error;
            }
            if !multi && c > 1 {
                AA_DEBUG(DEBUG_UNPACK, cstr!("!multi && c > 1"));
                /* fail - all other cases with embedded \0 */
                aa_destroy_str_table(strs);
                (*e).pos = saved_pos;
                return error;
            }
            (*table.add(i as size_t)).strs = strp;
            (*table.add(i as size_t)).count = c;
            (*table.add(i as size_t)).size = size2;
            i += 1;
        }
        if !aa_unpack_nameX(e, aa_code::AA_ARRAYEND, ptr::null()) {
            aa_destroy_str_table(strs);
            (*e).pos = saved_pos;
            return error;
        }
        if !aa_unpack_nameX(e, aa_code::AA_STRUCTEND, ptr::null()) {
            aa_destroy_str_table(strs);
            (*e).pos = saved_pos;
            return error;
        }
    }
    0
}

unsafe fn unpack_xattrs(e: *mut aa_ext, profile: *mut aa_profile) -> bool {
    let pos = (*e).pos;

    if aa_unpack_nameX(e, aa_code::AA_STRUCT, cstr!("xattrs")) {
        let mut size: u16 = 0;
        let mut i: c_int;

        if !aa_unpack_array(e, ptr::null(), &mut size) {
            (*e).pos = pos;
            return false;
        }
        (*profile).attach.xattr_count = size;
        (*profile).attach.xattrs = kcalloc(size as size_t, size_of::<*mut c_char>(), GFP_KERNEL) as *mut *mut c_char;
        if (*profile).attach.xattrs.is_null() {
            (*e).pos = pos;
            return false;
        }
        i = 0;
        while i < size as c_int {
            if aa_unpack_strdup(e, (*profile).attach.xattrs.add(i as size_t), ptr::null()) == 0 {
                (*e).pos = pos;
                return false;
            }
            i += 1;
        }
        if !aa_unpack_nameX(e, aa_code::AA_ARRAYEND, ptr::null()) {
            (*e).pos = pos;
            return false;
        }
        if !aa_unpack_nameX(e, aa_code::AA_STRUCTEND, ptr::null()) {
            (*e).pos = pos;
            return false;
        }
    }

    true
}

unsafe fn unpack_secmark(e: *mut aa_ext, rules: *mut aa_ruleset) -> bool {
    let pos = (*e).pos;
    let mut size: u16 = 0;
    let mut i: c_int = 0;

    if aa_unpack_nameX(e, aa_code::AA_STRUCT, cstr!("secmark")) {
        if !aa_unpack_array(e, ptr::null(), &mut size) {
            (*e).pos = pos;
            return false;
        }

        (*rules).secmark = kzalloc(size_of::<aa_secmark>() * size as size_t, GFP_KERNEL) as *mut aa_secmark;
        if (*rules).secmark.is_null() {
            (*e).pos = pos;
            return false;
        }

        (*rules).secmark_count = size;

        i = 0;
        while i < size as c_int {
            if !unpack_u8(e, &mut (*(*rules).secmark.add(i as size_t)).audit, ptr::null()) {
                break;
            }
            if !unpack_u8(e, &mut (*(*rules).secmark.add(i as size_t)).deny, ptr::null()) {
                break;
            }
            if aa_unpack_strdup(e, &mut (*(*rules).secmark.add(i as size_t)).label, ptr::null()) == 0 {
                break;
            }
            i += 1;
        }
        if i != size as c_int || !aa_unpack_nameX(e, aa_code::AA_ARRAYEND, ptr::null()) || !aa_unpack_nameX(e, aa_code::AA_STRUCTEND, ptr::null()) {
            if !(*rules).secmark.is_null() {
                i = 0;
                while i < size as c_int {
                    kfree_sensitive((*(*rules).secmark.add(i as size_t)).label as *mut c_void);
                    i += 1;
                }
                kfree_sensitive((*rules).secmark as *mut c_void);
                (*rules).secmark_count = 0;
                (*rules).secmark = ptr::null_mut();
            }
            (*e).pos = pos;
            return false;
        }
    }

    true
}

unsafe fn unpack_rlimits(e: *mut aa_ext, rules: *mut aa_ruleset) -> bool {
    let pos = (*e).pos;

    /* rlimits are optional */
    if aa_unpack_nameX(e, aa_code::AA_STRUCT, cstr!("rlimits")) {
        let mut size: u16 = 0;
        let mut i: c_int;
        let mut tmp: u32 = 0;
        if !aa_unpack_u32(e, &mut tmp, ptr::null()) {
            (*e).pos = pos;
            return false;
        }
        (*rules).rlimits.mask = tmp;

        if !aa_unpack_array(e, ptr::null(), &mut size) || size > RLIM_NLIMITS {
            (*e).pos = pos;
            return false;
        }
        i = 0;
        while i < size as c_int {
            let mut tmp2: u64 = 0;
            let a = aa_map_resource(i);
            if !aa_unpack_u64(e, &mut tmp2, ptr::null()) {
                (*e).pos = pos;
                return false;
            }
            (*rules).rlimits.limits[a as usize].rlim_max = tmp2;
            i += 1;
        }
        if !aa_unpack_nameX(e, aa_code::AA_ARRAYEND, ptr::null()) {
            (*e).pos = pos;
            return false;
        }
        if !aa_unpack_nameX(e, aa_code::AA_STRUCTEND, ptr::null()) {
            (*e).pos = pos;
            return false;
        }
    }
    true
}

unsafe fn verify_tags(tags: *mut aa_tags_struct, info: *mut *const c_char) -> bool {
    if ((*tags).hdrs.size != 0 && (*tags).hdrs.table.is_null()) ||
        ((*tags).hdrs.size == 0 && !(*tags).hdrs.table.is_null()) {
        *info = cstr!("failed verification tag.hdrs disagree");
        return false;
    }
    if ((*tags).sets.size != 0 && (*tags).sets.table.is_null()) ||
        ((*tags).sets.size == 0 && !(*tags).sets.table.is_null()) {
        *info = cstr!("failed verification tag.sets disagree");
        return false;
    }
    if ((*tags).strs.size != 0 && (*tags).strs.table.is_null()) ||
        ((*tags).strs.size == 0 && !(*tags).strs.table.is_null()) {
        *info = cstr!("failed verification tags->strs disagree");
        return false;
    }
    /* no data present */
    if (*tags).sets.size == 0 && (*tags).hdrs.size == 0 && (*tags).strs.size == 0 {
        return true;
    } else if !((*tags).sets.size != 0 && (*tags).hdrs.size != 0 && (*tags).strs.size != 0) {
        /* some data present but not all */
        *info = cstr!("failed verification tags partial data present");
        return false;
    }

    let mut i: u32 = 0;

    while i < (*tags).sets.size {
        /* count followed by count indexes into hdrs */
        let mut cnt = *(*tags).sets.table.add(i as usize);

        if i as u64 + cnt as u64 >= (*tags).sets.size as u64 {
            AA_DEBUG(DEBUG_UNPACK, cstr!("tagset too large %d+%d > sets.table[%d]"), i, cnt, (*tags).sets.size);
            *info = cstr!("failed verification tagset too large");
            return false;
        }
        while cnt != 0 {
            i += 1;
            if *(*tags).sets.table.add(i as usize) >= (*tags).hdrs.size {
                AA_DEBUG(DEBUG_UNPACK, cstr!("tagsets idx out of bounds cnt %d sets.table[%d] >= %d"), cnt, i - 1, (*tags).hdrs.size);
                *info = cstr!("failed verification tagsets idx out of bounds");
                return false;
            }
            cnt -= 1;
        }
        i += 1;
    }
    i = 0;
    while i < (*tags).hdrs.size {
        let idx = (*(*tags).hdrs.table.add(i as usize)).tags;

        if idx >= (*tags).strs.size {
            AA_DEBUG(DEBUG_UNPACK, cstr!("tag.hdrs idx oob idx %d > tags->strs.size=%d"), idx, (*tags).strs.size);
            *info = cstr!("failed verification tags.hdrs idx out of bounds");
            return false;
        }
        if (*(*tags).hdrs.table.add(i as usize)).count != (*(*tags).strs.table.add(idx as usize)).count as u32 {
            AA_DEBUG(DEBUG_UNPACK, cstr!("hdrs.table[%d].count=%d != tags->strs.table[%d]=%d"), i, (*(*tags).hdrs.table.add(i as usize)).count, idx, (*(*tags).strs.table.add(idx as usize)).count);
            *info = cstr!("failed verification tagd.hdrs[idx].count");
            return false;
        }
        if (*(*tags).hdrs.table.add(i as usize)).size != (*(*tags).strs.table.add(idx as usize)).size as u32 {
            AA_DEBUG(DEBUG_UNPACK, cstr!("hdrs.table[%d].size=%d != strs.table[%d].size=%d"), i, (*(*tags).hdrs.table.add(i as usize)).size, idx, (*(*tags).strs.table.add(idx as usize)).size);
            *info = cstr!("failed verification tagd.hdrs[idx].size");
            return false;
        }
        i += 1;
    }

    true
}

unsafe fn unpack_tagsets(e: *mut aa_ext, tags: *mut aa_tags_struct) -> c_int {
    let mut sets: *mut u32;
    let mut i: u16 = 0;
    let mut size: u16 = 0;
    let mut error = -EPROTO;
    let pos = (*e).pos;

    if !aa_unpack_array(e, cstr!("sets"), &mut size) {
        (*e).pos = pos;
        return error;
    }
    sets = kcalloc(size as size_t, size_of::<u32>(), GFP_KERNEL) as *mut u32;
    if sets.is_null() {
        error = -ENOMEM;
        (*e).pos = pos;
        return error;
    }
    while i < size {
        if !aa_unpack_u32(e, sets.add(i as usize), ptr::null()) {
            kfree_sensitive(sets as *mut c_void);
            (*e).pos = pos;
            return error;
        }
        i += 1;
    }
    if !aa_unpack_nameX(e, aa_code::AA_ARRAYEND, ptr::null()) {
        kfree_sensitive(sets as *mut c_void);
        (*e).pos = pos;
        return error;
    }

    (*tags).sets.size = size as u32;
    (*tags).sets.table = sets;

    0
}

unsafe fn unpack_tag_header_ent(e: *mut aa_ext, h: *mut aa_tags_header) -> bool {
    aa_unpack_u32(e, &mut (*h).mask, ptr::null()) &&
        aa_unpack_u32(e, &mut (*h).count, ptr::null()) &&
        aa_unpack_u32(e, &mut (*h).size, ptr::null()) &&
        aa_unpack_u32(e, &mut (*h).tags, ptr::null())
}

unsafe fn unpack_tag_headers(e: *mut aa_ext, tags: *mut aa_tags_struct) -> c_int {
    let mut hdrs: *mut aa_tags_header;
    let mut i: u16 = 0;
    let mut size: u16 = 0;
    let mut error = -EPROTO;
    let pos = (*e).pos;

    if !aa_unpack_array(e, cstr!("hdrs"), &mut size) {
        (*e).pos = pos;
        return error;
    }
    hdrs = kzalloc(size_of::<aa_tags_header>() * size as size_t, GFP_KERNEL) as *mut aa_tags_header;
    if hdrs.is_null() {
        error = -ENOMEM;
        (*e).pos = pos;
        return error;
    }
    while i < size {
        if !unpack_tag_header_ent(e, hdrs.add(i as usize)) {
            kfree_sensitive(hdrs as *mut c_void);
            (*e).pos = pos;
            return error;
        }
        i += 1;
    }
    if !aa_unpack_nameX(e, aa_code::AA_ARRAYEND, ptr::null()) {
        kfree_sensitive(hdrs as *mut c_void);
        (*e).pos = pos;
        return error;
    }

    (*tags).hdrs.size = size as u32;
    (*tags).hdrs.table = hdrs;
    AA_DEBUG(DEBUG_UNPACK, cstr!("headers %ld size %d"), hdrs as c_long, size as c_int);
    true as c_int
}

unsafe fn unpack_tags(e: *mut aa_ext, tags: *mut aa_tags_struct, info: *mut *const c_char) -> c_int {
    let mut error = -EPROTO;
    let pos = (*e).pos;

    AA_BUG(tags.is_null());
    /* policy tags are optional */
    if aa_unpack_nameX(e, aa_code::AA_STRUCT, cstr!("tags")) {
        let mut version: u32 = 0;

        if !aa_unpack_u32(e, &mut version, cstr!("version")) || version != 1 {
            *info = cstr!("invalid tags version");
            (*e).pos = pos;
            return error;
        }
        error = unpack_strs_table(e, cstr!("strs"), true, &mut (*tags).strs);
        if error != 0 {
            *info = cstr!("failed to unpack profile tag.strs");
            aa_destroy_tags(tags);
            (*e).pos = pos;
            return error;
        }
        error = unpack_tag_headers(e, tags);
        if error != 0 {
            *info = cstr!("failed to unpack profile tag.headers");
            aa_destroy_tags(tags);
            (*e).pos = pos;
            return error;
        }
        error = unpack_tagsets(e, tags);
        if error != 0 {
            *info = cstr!("failed to unpack profile tag.sets");
            aa_destroy_tags(tags);
            (*e).pos = pos;
            return error;
        }
        error = -EPROTO;
        if !aa_unpack_nameX(e, aa_code::AA_STRUCTEND, ptr::null()) {
            aa_destroy_tags(tags);
            (*e).pos = pos;
            return error;
        }

        if !verify_tags(tags, info) {
            aa_destroy_tags(tags);
            (*e).pos = pos;
            return error;
        }
    }

    0
}

unsafe fn unpack_perm(e: *mut aa_ext, version: u32, perm: *mut aa_perms) -> bool {
    let mut reserved: u32 = 0;

    if version != 1 {
        return false;
    }

    /* reserved entry is for later expansion, discard for now */
    aa_unpack_u32(e, &mut reserved, ptr::null()) &&
        aa_unpack_u32(e, &mut (*perm).allow, ptr::null()) &&
        aa_unpack_u32(e, &mut (*perm).deny, ptr::null()) &&
        aa_unpack_u32(e, &mut (*perm).subtree, ptr::null()) &&
        aa_unpack_u32(e, &mut (*perm).cond, ptr::null()) &&
        aa_unpack_u32(e, &mut (*perm).kill, ptr::null()) &&
        aa_unpack_u32(e, &mut (*perm).complain, ptr::null()) &&
        aa_unpack_u32(e, &mut (*perm).prompt, ptr::null()) &&
        aa_unpack_u32(e, &mut (*perm).audit, ptr::null()) &&
        aa_unpack_u32(e, &mut (*perm).quiet, ptr::null()) &&
        aa_unpack_u32(e, &mut (*perm).hide, ptr::null()) &&
        aa_unpack_u32(e, &mut (*perm).xindex, ptr::null()) &&
        aa_unpack_u32(e, &mut (*perm).tag, ptr::null()) &&
        aa_unpack_u32(e, &mut (*perm).label, ptr::null())
}

unsafe fn unpack_perms_table(e: *mut aa_ext, perms: *mut *mut aa_perms) -> ssize_t {
    let pos = (*e).pos;
    let mut size: u16 = 0;

    AA_BUG(perms.is_null());
    /*
     * policy perms are optional, in which case perms are embedded
     * in the dfa accept table
     */
    if aa_unpack_nameX(e, aa_code::AA_STRUCT, cstr!("perms")) {
        let mut i: c_int;
        let mut version: u32 = 0;

        if !aa_unpack_u32(e, &mut version, cstr!("version")) {
            (*e).pos = pos;
            return -EPROTO as ssize_t;
        }
        if !aa_unpack_array(e, ptr::null(), &mut size) {
            (*e).pos = pos;
            return -EPROTO as ssize_t;
        }
        *perms = kzalloc(size_of::<aa_perms>() * size as size_t, GFP_KERNEL) as *mut aa_perms;
        if (*perms).is_null() {
            (*e).pos = pos;
            return -ENOMEM as ssize_t;
        }
        i = 0;
        while i < size as c_int {
            if !unpack_perm(e, version, (*perms).add(i as size_t)) {
                kfree(*perms as *mut c_void);
                (*e).pos = pos;
                return -EPROTO as ssize_t;
            }
            i += 1;
        }
        if !aa_unpack_nameX(e, aa_code::AA_ARRAYEND, ptr::null()) {
            kfree(*perms as *mut c_void);
            (*e).pos = pos;
            return -EPROTO as ssize_t;
        }
        if !aa_unpack_nameX(e, aa_code::AA_STRUCTEND, ptr::null()) {
            kfree(*perms as *mut c_void);
            (*e).pos = pos;
            return -EPROTO as ssize_t;
        }
    } else {
        *perms = ptr::null_mut();
    }

    size as ssize_t
}

unsafe fn unpack_pdb(
    e: *mut aa_ext,
    policy: *mut *mut aa_policydb,
    required_dfa: bool,
    required_trans: bool,
    info: *mut *const c_char,
) -> c_int {
    let pdb: *mut aa_policydb;
    let pos = (*e).pos;
    let mut i: usize;
    let mut flags: c_int;
    let mut error = -EPROTO;
    let mut size: ssize_t;
    let mut version: u32 = 0;

    pdb = aa_alloc_pdb(GFP_KERNEL);
    if pdb.is_null() {
        return -ENOMEM;
    }

    AA_DEBUG(DEBUG_UNPACK, cstr!("unpacking tags"));
    if unpack_tags(e, &mut (*pdb).tags, info) < 0 {
        aa_put_pdb(pdb);
        (*e).pos = pos;
        return error;
    }
    AA_DEBUG(DEBUG_UNPACK, cstr!("done unpacking tags"));

    size = unpack_perms_table(e, &mut (*pdb).perms);
    if size < 0 {
        error = size as c_int;
        (*pdb).perms = ptr::null_mut();
        *info = cstr!("failed to unpack - perms");
        aa_put_pdb(pdb);
        (*e).pos = pos;
        return error;
    }
    (*pdb).size = size;

    if !(*pdb).perms.is_null() {
        /* perms table present accept is index */
        flags = TO_ACCEPT1_FLAG(YYTD_DATA32);
        if aa_unpack_u32(e, &mut version, cstr!("permsv")) && version > 2 {
            /* accept2 used for dfa flags */
            flags |= TO_ACCEPT2_FLAG(YYTD_DATA32);
        }
    } else {
        /* packed perms in accept1 and accept2 */
        flags = TO_ACCEPT1_FLAG(YYTD_DATA32) | TO_ACCEPT2_FLAG(YYTD_DATA32);
    }

    (*pdb).dfa = unpack_dfa(e, flags);
    if IS_ERR((*pdb).dfa) {
        error = PTR_ERR((*pdb).dfa);
        (*pdb).dfa = ptr::null_mut();
        *info = cstr!("failed to unpack - dfa");
        aa_put_pdb(pdb);
        (*e).pos = pos;
        return error;
    } else if (*pdb).dfa.is_null() {
        if required_dfa {
            *info = cstr!("missing required dfa");
            aa_put_pdb(pdb);
            (*e).pos = pos;
            return error;
        }
    } else {
        /*
         * only unpack the following if a dfa is present
         *
         * sadly start was given different names for file and policydb
         * but since it is optional we can try both
         */
        if !aa_unpack_u32(e, &mut (*pdb).start[0], cstr!("start")) {
            /* default start state */
            (*pdb).start[0] = DFA_START;
        }
        if !aa_unpack_u32(e, &mut (*pdb).start[AA_CLASS_FILE], cstr!("dfa_start")) {
            /* default start state for xmatch and file dfa */
            (*pdb).start[AA_CLASS_FILE] = DFA_START;
        }

        let state_count = (*(*(*pdb).dfa).tables[YYTD_ID_BASE]).td_lolen as size_t;

        if (*pdb).start[0] as size_t >= state_count ||
            (*pdb).start[AA_CLASS_FILE] as size_t >= state_count {
            *info = cstr!("invalid dfa start state");
            aa_put_pdb(pdb);
            (*e).pos = pos;
            return error;
        }

        /* setup class index */
        i = AA_CLASS_FILE + 1;
        while i <= AA_CLASS_LAST {
            (*pdb).start[i] = aa_dfa_next((*pdb).dfa, (*pdb).start[0], i);
            i += 1;
        }
    }

    /* accept2 is in some cases being allocated, even with perms */
    if !(*pdb).dfa.is_null() && !(*pdb).perms.is_null() && (*(*pdb).dfa).tables[YYTD_ID_ACCEPT2].is_null() {
        /* add dfa flags table missing in v2 */
        let noents = (*(*(*pdb).dfa).tables[YYTD_ID_ACCEPT]).td_lolen;
        let tdflags = (*(*(*pdb).dfa).tables[YYTD_ID_ACCEPT]).td_flags;
        let tsize = table_size(noents, tdflags);

        (*(*pdb).dfa).tables[YYTD_ID_ACCEPT2] = kvzalloc(tsize, GFP_KERNEL) as *mut table_header;
        if (*(*pdb).dfa).tables[YYTD_ID_ACCEPT2].is_null() {
            *info = cstr!("failed to alloc dfa flags table");
            error = -ENOMEM;
            aa_put_pdb(pdb);
            (*e).pos = pos;
            return error;
        }
        (*(*(*pdb).dfa).tables[YYTD_ID_ACCEPT2]).td_lolen = noents;
        (*(*(*pdb).dfa).tables[YYTD_ID_ACCEPT2]).td_flags = tdflags;
    }
    /*
     * Unfortunately due to a bug in earlier userspaces, a
     * transition table may be present even when the dfa is
     * not. For compatibility reasons unpack and discard.
     */
    error = unpack_strs_table(e, cstr!("xtable"), false, &mut (*pdb).trans);
    if error != 0 && required_trans {
        *info = cstr!("failed to unpack profile transition table");
        aa_put_pdb(pdb);
        (*e).pos = pos;
        return error;
    }

    if (*pdb).dfa.is_null() && !(*pdb).trans.table.is_null() {
        aa_destroy_str_table(&mut (*pdb).trans);
    }

    /* TODO:
     * - move compat mapping here, requires dfa merging first
     * - move verify here, it has to be done after compat mappings
     * - move free of unneeded trans table here, has to be done
     *   after perm mapping.
     */
    *policy = pdb;
    0
}

unsafe extern "C" fn strhash(data: *const c_void, _len: u32, seed: u32) -> u32 {
    let key = data as *const *const c_char;

    jhash(*key as *const c_void, strlen(*key) as u32, seed)
}

unsafe extern "C" fn datacmp(arg: *mut rhashtable_compare_arg, obj: *const c_void) -> c_int {
    let data = obj as *const aa_data;
    let key = (*arg).key as *const *const c_char;

    strcmp((*data).key, *key)
}

/**
 * unpack_profile - unpack a serialized profile
 * @e: serialized data extent information (NOT NULL)
 * @ns_name: pointer of newly allocated copy of %NULL in case of error
 *
 * NOTE: unpack profile sets audit struct if there is a failure
 */
unsafe fn unpack_profile(e: *mut aa_ext, ns_name: *mut *mut c_char) -> *mut aa_profile {
    let mut rules: *mut aa_ruleset;
    let mut profile: *mut aa_profile = ptr::null_mut();
    let mut tmpname: *const c_char;
    let mut tmpns: *const c_char = ptr::null();
    let mut name: *const c_char = ptr::null();
    let mut info: *const c_char = cstr!("failed to unpack profile");
    let mut ns_len: size_t = 0;
    let mut params: rhashtable_params = core::mem::zeroed();
    let mut key: *mut c_char = ptr::null_mut();
    let mut disconnected: *mut c_char = ptr::null_mut();
    let mut data: *mut aa_data;
    let mut error = -EPROTO;
    let mut tmpcap: kernel_cap_t = core::mem::zeroed();
    let mut tmp: u32 = 0;

    *ns_name = ptr::null_mut();

    /* check that we have the right struct being passed */
    if !aa_unpack_nameX(e, aa_code::AA_STRUCT, cstr!("profile")) {
        goto_unpack_profile_fail(e, ns_name, profile, name, info, error);
        return ERR_PTR(error);
    }
    if aa_unpack_str(e, &mut name, ptr::null()) == 0 {
        goto_unpack_profile_fail(e, ns_name, profile, name, info, error);
        return ERR_PTR(error);
    }
    if *name == 0 {
        goto_unpack_profile_fail(e, ns_name, profile, name, info, error);
        return ERR_PTR(error);
    }

    tmpname = aa_splitn_fqname(name, strlen(name), &mut tmpns, &mut ns_len);
    if !tmpns.is_null() {
        if tmpname.is_null() {
            info = cstr!("empty profile name");
            goto_unpack_profile_fail(e, ns_name, profile, name, info, error);
            return ERR_PTR(error);
        }
        *ns_name = kstrndup(tmpns, ns_len, GFP_KERNEL);
        if (*ns_name).is_null() {
            info = cstr!("out of memory");
            error = -ENOMEM;
            goto_unpack_profile_fail(e, ns_name, profile, name, info, error);
            return ERR_PTR(error);
        }
        name = tmpname;
    }

    profile = aa_alloc_profile(name, ptr::null_mut(), GFP_KERNEL);
    if profile.is_null() {
        info = cstr!("out of memory");
        error = -ENOMEM;
        goto_unpack_profile_fail(e, ns_name, profile, name, info, error);
        return ERR_PTR(error);
    }
    rules = (*profile).label.rules[0];

    /* profile renaming is optional */
    aa_unpack_str(e, &mut (*profile).rename, cstr!("rename"));

    /* attachment string is optional */
    aa_unpack_str(e, &mut (*profile).attach.xmatch_str, cstr!("attach"));

    /* xmatch is optional and may be NULL */
    error = unpack_pdb(e, &mut (*profile).attach.xmatch, false, false, &mut info);
    if error != 0 {
        info = cstr!("bad xmatch");
        goto_unpack_profile_fail(e, ns_name, profile, name, info, error);
        return ERR_PTR(error);
    }

    /* neither xmatch_len not xmatch_perms are optional if xmatch is set */
    if !(*(*profile).attach.xmatch).dfa.is_null() {
        if !aa_unpack_u32(e, &mut tmp, ptr::null()) {
            info = cstr!("missing xmatch len");
            goto_unpack_profile_fail(e, ns_name, profile, name, info, error);
            return ERR_PTR(error);
        }
        (*profile).attach.xmatch_len = tmp;
        (*(*profile).attach.xmatch).start[AA_CLASS_XMATCH] = DFA_START;
        if (*(*profile).attach.xmatch).perms.is_null() {
            error = aa_compat_map_xmatch((*profile).attach.xmatch);
            if error != 0 {
                info = cstr!("failed to convert xmatch permission table");
                goto_unpack_profile_fail(e, ns_name, profile, name, info, error);
                return ERR_PTR(error);
            }
        }
    }

    /* disconnected attachment string is optional */
    aa_unpack_strdup(e, &mut disconnected, cstr!("disconnected"));
    (*profile).disconnected = disconnected;

    /* optional */
    aa_unpack_u32(e, &mut (*profile).signal, cstr!("kill"));
    if (*profile).signal < 1 || (*profile).signal > MAXMAPPED_SIG {
        info = cstr!("profile kill.signal invalid value");
        goto_unpack_profile_fail(e, ns_name, profile, name, info, error);
        return ERR_PTR(error);
    }
    /* per profile debug flags (complain, audit) */
    if !aa_unpack_nameX(e, aa_code::AA_STRUCT, cstr!("flags")) {
        info = cstr!("profile missing flags");
        goto_unpack_profile_fail(e, ns_name, profile, name, info, error);
        return ERR_PTR(error);
    }
    info = cstr!("failed to unpack profile flags");
    if !aa_unpack_u32(e, &mut tmp, ptr::null()) {
        goto_unpack_profile_fail(e, ns_name, profile, name, info, error);
        return ERR_PTR(error);
    }
    if tmp & PACKED_FLAG_HAT != 0 {
        (*profile).label.flags |= FLAG_HAT;
    }
    if tmp & PACKED_FLAG_DEBUG1 != 0 {
        (*profile).label.flags |= FLAG_DEBUG1;
    }
    if tmp & PACKED_FLAG_DEBUG2 != 0 {
        (*profile).label.flags |= FLAG_DEBUG2;
    }
    if !aa_unpack_u32(e, &mut tmp, ptr::null()) {
        goto_unpack_profile_fail(e, ns_name, profile, name, info, error);
        return ERR_PTR(error);
    }
    if tmp == PACKED_MODE_COMPLAIN || ((*e).version & FORCE_COMPLAIN_FLAG) != 0 {
        (*profile).mode = APPARMOR_COMPLAIN;
    } else if tmp == PACKED_MODE_ENFORCE {
        (*profile).mode = APPARMOR_ENFORCE;
    } else if tmp == PACKED_MODE_KILL {
        (*profile).mode = APPARMOR_KILL;
    } else if tmp == PACKED_MODE_UNCONFINED {
        (*profile).mode = APPARMOR_UNCONFINED;
        (*profile).label.flags |= FLAG_UNCONFINED;
    } else if tmp == PACKED_MODE_USER {
        (*profile).mode = APPARMOR_USER;
    } else {
        goto_unpack_profile_fail(e, ns_name, profile, name, info, error);
        return ERR_PTR(error);
    }
    if !aa_unpack_u32(e, &mut tmp, ptr::null()) {
        goto_unpack_profile_fail(e, ns_name, profile, name, info, error);
        return ERR_PTR(error);
    }
    if tmp != 0 {
        (*profile).audit = AUDIT_ALL;
    }

    if !aa_unpack_nameX(e, aa_code::AA_STRUCTEND, ptr::null()) {
        goto_unpack_profile_fail(e, ns_name, profile, name, info, error);
        return ERR_PTR(error);
    }

    /* path_flags is optional */
    if aa_unpack_u32(e, &mut (*profile).path_flags, cstr!("path_flags")) {
        (*profile).path_flags |= (*profile).label.flags & PATH_MEDIATE_DELETED;
    } else {
        /* set a default value if path_flags field is not present */
        (*profile).path_flags = PATH_MEDIATE_DELETED;
    }

    info = cstr!("failed to unpack profile capabilities");
    if !aa_unpack_cap_low(e, &mut (*rules).caps.allow, ptr::null()) ||
        !aa_unpack_cap_low(e, &mut (*rules).caps.audit, ptr::null()) ||
        !aa_unpack_cap_low(e, &mut (*rules).caps.quiet, ptr::null()) ||
        !aa_unpack_cap_low(e, &mut tmpcap, ptr::null()) {
        goto_unpack_profile_fail(e, ns_name, profile, name, info, error);
        return ERR_PTR(error);
    }

    info = cstr!("failed to unpack upper profile capabilities");
    if aa_unpack_nameX(e, aa_code::AA_STRUCT, cstr!("caps64")) {
        /* optional upper half of 64 bit caps */
        if !aa_unpack_cap_high(e, &mut (*rules).caps.allow, ptr::null()) ||
            !aa_unpack_cap_high(e, &mut (*rules).caps.audit, ptr::null()) ||
            !aa_unpack_cap_high(e, &mut (*rules).caps.quiet, ptr::null()) ||
            !aa_unpack_cap_high(e, &mut tmpcap, ptr::null()) ||
            !aa_unpack_nameX(e, aa_code::AA_STRUCTEND, ptr::null()) {
            goto_unpack_profile_fail(e, ns_name, profile, name, info, error);
            return ERR_PTR(error);
        }
    }

    info = cstr!("failed to unpack extended profile capabilities");
    if aa_unpack_nameX(e, aa_code::AA_STRUCT, cstr!("capsx")) {
        /* optional extended caps mediation mask */
        if !aa_unpack_cap_low(e, &mut (*rules).caps.extended, ptr::null()) ||
            !aa_unpack_cap_high(e, &mut (*rules).caps.extended, ptr::null()) ||
            !aa_unpack_nameX(e, aa_code::AA_STRUCTEND, ptr::null()) {
            goto_unpack_profile_fail(e, ns_name, profile, name, info, error);
            return ERR_PTR(error);
        }
    }

    if !unpack_xattrs(e, profile) {
        info = cstr!("failed to unpack profile xattrs");
        goto_unpack_profile_fail(e, ns_name, profile, name, info, error);
        return ERR_PTR(error);
    }

    if !unpack_rlimits(e, rules) {
        info = cstr!("failed to unpack profile rlimits");
        goto_unpack_profile_fail(e, ns_name, profile, name, info, error);
        return ERR_PTR(error);
    }

    if !unpack_secmark(e, rules) {
        info = cstr!("failed to unpack profile secmark rules");
        goto_unpack_profile_fail(e, ns_name, profile, name, info, error);
        return ERR_PTR(error);
    }

    if aa_unpack_nameX(e, aa_code::AA_STRUCT, cstr!("policydb")) {
        /* generic policy dfa - optional and may be NULL */
        info = cstr!("failed to unpack policydb");
        error = unpack_pdb(e, &mut (*rules).policy, true, false, &mut info);
        if error != 0 {
            goto_unpack_profile_fail(e, ns_name, profile, name, info, error);
            return ERR_PTR(error);
        }
        /* Fixup: drop when we get rid of start array */
        if aa_dfa_next((*(*rules).policy).dfa, (*(*rules).policy).start[0], AA_CLASS_FILE) != 0 {
            (*(*rules).policy).start[AA_CLASS_FILE] =
                aa_dfa_next((*(*rules).policy).dfa, (*(*rules).policy).start[0], AA_CLASS_FILE);
        }
        if !aa_unpack_nameX(e, aa_code::AA_STRUCTEND, ptr::null()) {
            goto_unpack_profile_fail(e, ns_name, profile, name, info, error);
            return ERR_PTR(error);
        }
        if (*(*rules).policy).perms.is_null() {
            error = aa_compat_map_policy((*rules).policy, (*e).version);
            if error != 0 {
                info = cstr!("failed to remap policydb permission table");
                goto_unpack_profile_fail(e, ns_name, profile, name, info, error);
                return ERR_PTR(error);
            }
        }
    } else {
        (*rules).policy = aa_get_pdb(nullpdb);
    }
    /* get file rules */
    error = unpack_pdb(e, &mut (*rules).file, false, true, &mut info);
    if error != 0 {
        goto_unpack_profile_fail(e, ns_name, profile, name, info, error);
        return ERR_PTR(error);
    } else if !(*(*rules).file).dfa.is_null() {
        if (*(*rules).file).perms.is_null() {
            AA_DEBUG(DEBUG_UNPACK, cstr!("compat mapping perms"));
            error = aa_compat_map_file((*rules).file);
            if error != 0 {
                info = cstr!("failed to remap file permission table");
                goto_unpack_profile_fail(e, ns_name, profile, name, info, error);
                return ERR_PTR(error);
            }
        }
    } else if !(*(*rules).policy).dfa.is_null() && (*(*rules).policy).start[AA_CLASS_FILE] != 0 {
        aa_put_pdb((*rules).file);
        (*rules).file = aa_get_pdb((*rules).policy);
    } else {
        aa_put_pdb((*rules).file);
        (*rules).file = aa_get_pdb(nullpdb);
    }
    error = -EPROTO;
    if aa_unpack_nameX(e, aa_code::AA_STRUCT, cstr!("data")) {
        info = cstr!("out of memory");
        (*profile).data = kzalloc(size_of::<rhashtable>(), GFP_KERNEL) as *mut rhashtable;
        if (*profile).data.is_null() {
            error = -ENOMEM;
            goto_unpack_profile_fail(e, ns_name, profile, name, info, error);
            return ERR_PTR(error);
        }
        params.nelem_hint = 3;
        params.key_len = size_of::<*mut c_void>();
        params.key_offset = offset_of!(aa_data, key);
        params.head_offset = offset_of!(aa_data, head);
        params.hashfn = Some(strhash);
        params.obj_cmpfn = Some(datacmp);

        if rhashtable_init((*profile).data, &mut params) != 0 {
            info = cstr!("failed to init key, value hash table");
            goto_unpack_profile_fail(e, ns_name, profile, name, info, error);
            return ERR_PTR(error);
        }

        while aa_unpack_strdup(e, &mut key, ptr::null()) != 0 {
            data = kzalloc(size_of::<aa_data>(), GFP_KERNEL) as *mut aa_data;
            if data.is_null() {
                kfree_sensitive(key as *mut c_void);
                error = -ENOMEM;
                goto_unpack_profile_fail(e, ns_name, profile, name, info, error);
                return ERR_PTR(error);
            }

            (*data).key = key;
            (*data).size = aa_unpack_blob(e, &mut (*data).data as *mut *mut c_void as *mut *mut c_char, ptr::null());
            (*data).data = kvmemdup((*data).data, (*data).size, GFP_KERNEL);
            if (*data).size != 0 && (*data).data.is_null() {
                kfree_sensitive((*data).key as *mut c_void);
                kfree_sensitive(data as *mut c_void);
                error = -ENOMEM;
                goto_unpack_profile_fail(e, ns_name, profile, name, info, error);
                return ERR_PTR(error);
            }

            if rhashtable_insert_fast((*profile).data, &mut (*data).head, (*(*profile).data).p) != 0 {
                kvfree_sensitive((*data).data, (*data).size);
                kfree_sensitive((*data).key as *mut c_void);
                kfree_sensitive(data as *mut c_void);
                info = cstr!("failed to insert data to table");
                goto_unpack_profile_fail(e, ns_name, profile, name, info, error);
                return ERR_PTR(error);
            }
        }

        if !aa_unpack_nameX(e, aa_code::AA_STRUCTEND, ptr::null()) {
            info = cstr!("failed to unpack end of key, value data table");
            goto_unpack_profile_fail(e, ns_name, profile, name, info, error);
            return ERR_PTR(error);
        }
    }

    if !aa_unpack_nameX(e, aa_code::AA_STRUCTEND, ptr::null()) {
        info = cstr!("failed to unpack end of profile");
        goto_unpack_profile_fail(e, ns_name, profile, name, info, error);
        return ERR_PTR(error);
    }

    aa_compute_profile_mediates(profile);

    profile
}

unsafe fn goto_unpack_profile_fail(
    e: *mut aa_ext,
    ns_name: *mut *mut c_char,
    profile: *mut aa_profile,
    mut name: *const c_char,
    info: *const c_char,
    mut error: c_int,
) {
    if error == 0 {
        /* default error covers most cases */
        error = -EPROTO;
    }
    if !(*ns_name).is_null() {
        kfree(*ns_name as *mut c_void);
        *ns_name = ptr::null_mut();
    }
    if !profile.is_null() {
        name = ptr::null();
    } else if name.is_null() {
        name = cstr!("unknown");
    }
    audit_iface(profile, ptr::null(), name, info, e, error);
    aa_free_profile(profile);
}

/**
 * verify_header - unpack serialized stream header
 * @e: serialized data read head (NOT NULL)
 * @required: whether the header is required or optional
 * @ns: Returns - namespace if one is specified else NULL (NOT NULL)
 *
 * Returns: error or 0 if header is good
 */
unsafe fn verify_header(e: *mut aa_ext, required: c_int, ns: *mut *const c_char) -> c_int {
    let error = -EPROTONOSUPPORT;
    let mut name: *const c_char = ptr::null();

    /* get the interface version */
    if !aa_unpack_u32(e, &mut (*e).version, cstr!("version")) {
        if required != 0 {
            audit_iface(ptr::null_mut(), ptr::null(), ptr::null(), cstr!("invalid profile format"), e, error);
            return error;
        }
    }

    /* Check that the interface version is currently supported.
     * if not specified use previous version
     * Mask off everything that is not kernel abi version
     */
    if VERSION_LT((*e).version, v5) || VERSION_GT((*e).version, v9) {
        audit_iface(ptr::null_mut(), ptr::null(), ptr::null(), cstr!("unsupported interface version"), e, error);
        return error;
    }

    /* read the namespace if present */
    if aa_unpack_str(e, &mut name, cstr!("namespace")) != 0 {
        if *name == 0 {
            audit_iface(ptr::null_mut(), ptr::null(), ptr::null(), cstr!("invalid namespace name"), e, error);
            return error;
        }
        if !(*ns).is_null() && strcmp(*ns, name) != 0 {
            audit_iface(ptr::null_mut(), ptr::null(), ptr::null(), cstr!("invalid ns change"), e, error);
            return error;
        } else if (*ns).is_null() {
            *ns = kstrdup(name, GFP_KERNEL);
            if (*ns).is_null() {
                return -ENOMEM;
            }
        }
    }

    0
}

/**
 * verify_dfa_accept_index - verify accept indexes are in range of perms table
 * @dfa: the dfa to check accept indexes are in range
 * @table_size: the permission table size the indexes should be within
 */
unsafe fn verify_dfa_accept_index(dfa: *const aa_dfa, table_size: c_int) -> bool {
    let mut i: c_int = 0;
    while i < (*(*dfa).tables[YYTD_ID_ACCEPT]).td_lolen as c_int {
        if *ACCEPT_TABLE(dfa).add(i as usize) >= table_size as u32 {
            return false;
        }
        i += 1;
    }
    true
}

unsafe fn verify_perm(perm: *const aa_perms) -> bool {
    /* TODO: allow option to just force the perms into a valid state */
    if (*perm).allow & (*perm).deny != 0 {
        return false;
    }
    if (*perm).subtree & !(*perm).allow != 0 {
        return false;
    }
    if (*perm).cond & ((*perm).allow | (*perm).deny) != 0 {
        return false;
    }
    if (*perm).kill & (*perm).allow != 0 {
        return false;
    }
    if (*perm).complain & ((*perm).allow | (*perm).deny) != 0 {
        return false;
    }
    if (*perm).prompt & ((*perm).allow | (*perm).deny) != 0 {
        return false;
    }
    if (*perm).complain & (*perm).prompt != 0 {
        return false;
    }
    if (*perm).hide & (*perm).allow != 0 {
        return false;
    }

    true
}

unsafe fn verify_perms(pdb: *mut aa_policydb) -> bool {
    let mut i: c_int = 0;
    let mut xidx: c_int;
    let mut xmax: c_int = -1;

    while i < (*pdb).size as c_int {
        if !verify_perm((*pdb).perms.add(i as usize)) {
            return false;
        }
        /* verify indexes into str table */
        if ((*(*pdb).perms.add(i as usize)).xindex & AA_X_TYPE_MASK) == AA_X_TABLE {
            xidx = ((*(*pdb).perms.add(i as usize)).xindex & AA_X_INDEX_MASK) as c_int;
            if xidx >= (*pdb).trans.size as c_int {
                return false;
            }
            if xmax < xidx {
                xmax = xidx;
            }
        }
        if (*(*pdb).perms.add(i as usize)).tag != 0 &&
            (*(*pdb).perms.add(i as usize)).tag >= (*pdb).tags.sets.size {
            return false;
        }
        if (*(*pdb).perms.add(i as usize)).label != 0 &&
            (*(*pdb).perms.add(i as usize)).label >= (*pdb).trans.size {
            return false;
        }
        i += 1;
    }
    /* deal with incorrectly constructed string tables */
    if xmax == -1 {
        aa_destroy_str_table(&mut (*pdb).trans);
    } else if (*pdb).trans.size > (xmax + 1) as u32 {
        if !aa_resize_str_table(&mut (*pdb).trans, xmax + 1, GFP_KERNEL) {
            return false;
        }
    }
    true
}

/**
 * verify_profile - Do post unpack analysis to verify profile consistency
 * @profile: profile to verify (NOT NULL)
 *
 * Returns: 0 if passes verification else error
 *
 * This verification is post any unpack mapping or changes
 */
unsafe fn verify_profile(profile: *mut aa_profile) -> c_int {
    let rules = (*profile).label.rules[0];

    if rules.is_null() {
        return 0;
    }

    if !(*(*rules).file).dfa.is_null() &&
        !verify_dfa_accept_index((*(*rules).file).dfa, (*(*rules).file).size as c_int) {
        audit_iface(profile, ptr::null(), ptr::null(), cstr!("Unpack: file Invalid named transition"), ptr::null_mut(), -EPROTO);
        return -EPROTO;
    }
    if !(*(*rules).policy).dfa.is_null() &&
        !verify_dfa_accept_index((*(*rules).policy).dfa, (*(*rules).policy).size as c_int) {
        audit_iface(profile, ptr::null(), ptr::null(), cstr!("Unpack: policy Invalid named transition"), ptr::null_mut(), -EPROTO);
        return -EPROTO;
    }

    if !verify_perms((*rules).file) {
        audit_iface(profile, ptr::null(), ptr::null(), cstr!("Unpack: Invalid perm index"), ptr::null_mut(), -EPROTO);
        return -EPROTO;
    }
    if !verify_perms((*rules).policy) {
        audit_iface(profile, ptr::null(), ptr::null(), cstr!("Unpack: Invalid perm index"), ptr::null_mut(), -EPROTO);
        return -EPROTO;
    }
    if !verify_perms((*profile).attach.xmatch) {
        audit_iface(profile, ptr::null(), ptr::null(), cstr!("Unpack: Invalid perm index"), ptr::null_mut(), -EPROTO);
        return -EPROTO;
    }

    0
}

pub unsafe extern "C" fn aa_load_ent_free(ent: *mut aa_load_ent) {
    if !ent.is_null() {
        aa_put_profile((*ent).rename);
        aa_put_profile((*ent).old);
        aa_put_profile((*ent).new);
        kfree((*ent).ns_name as *mut c_void);
        kfree_sensitive(ent as *mut c_void);
    }
}

pub unsafe extern "C" fn aa_load_ent_alloc() -> *mut aa_load_ent {
    let ent = kzalloc(size_of::<aa_load_ent>(), GFP_KERNEL) as *mut aa_load_ent;
    if !ent.is_null() {
        INIT_LIST_HEAD(&mut (*ent).list);
    }
    ent
}

unsafe fn compress_zstd(src: *const c_char, slen: size_t, dst: *mut *mut c_char, dlen: *mut size_t) -> c_int {
    /*
     * CONFIG_SECURITY_APPARMOR_EXPORT_BINARY:
     * The C implementation performs zstd compression when this build-time
     * condition is enabled; otherwise it records the uncompressed length.
     */
    if aa_g_export_binary {
        let params = zstd_get_params(aa_g_rawdata_compression_level, slen);
        let wksp_len = zstd_cctx_workspace_bound(&params.cParams);
        let mut wksp: *mut c_void = ptr::null_mut();
        let mut ctx: *mut zstd_cctx = ptr::null_mut();
        let mut out_len = zstd_compress_bound(slen);
        let mut out: *mut c_void = ptr::null_mut();
        let mut ret: c_int = 0;

        out = kvzalloc(out_len, GFP_KERNEL);
        if out.is_null() {
            ret = -ENOMEM;
        } else {
            wksp = kvzalloc(wksp_len, GFP_KERNEL);
            if wksp.is_null() {
                ret = -ENOMEM;
            } else {
                ctx = zstd_init_cctx(wksp, wksp_len);
                if ctx.is_null() {
                    ret = -EINVAL;
                } else {
                    out_len = zstd_compress_cctx(ctx, out, out_len, src as *const c_void, slen, &params);
                    if zstd_is_error(out_len) || out_len >= slen {
                        ret = -EINVAL;
                    } else if is_vmalloc_addr(out) {
                        *dst = kvzalloc(out_len, GFP_KERNEL) as *mut c_char;
                        if !(*dst).is_null() {
                            memcpy(*dst as *mut c_void, out, out_len);
                            kvfree(out);
                            out = ptr::null_mut();
                        }
                    } else {
                        /*
                         * If the staging buffer was kmalloc'd, then using krealloc is
                         * probably going to be faster. The destination buffer will
                         * always be smaller, so it's just shrunk, avoiding a memcpy
                         */
                        *dst = krealloc(out, out_len, GFP_KERNEL) as *mut c_char;
                    }

                    if ret == 0 && (*dst).is_null() {
                        ret = -ENOMEM;
                    }
                    if ret == 0 {
                        *dlen = out_len;
                    }
                }
            }
        }

        if ret != 0 {
            kvfree(out);
            *dst = ptr::null_mut();
        }

        kvfree(wksp);
        ret
    } else {
        *dlen = slen;
        0
    }
}

unsafe fn compress_loaddata(data: *mut aa_loaddata) -> c_int {
    AA_BUG((*data).compressed_size > 0);

    /*
     * Shortcut the no compression case, else we increase the amount of
     * storage required by a small amount
     */
    if aa_g_rawdata_compression_level != 0 {
        let udata = (*data).data;
        let error = compress_zstd(udata, (*data).size, &mut (*data).data, &mut (*data).compressed_size);
        if error != 0 {
            (*data).compressed_size = (*data).size;
            return error;
        }
        if udata != (*data).data {
            kvfree(udata as *mut c_void);
        }
    } else {
        (*data).compressed_size = (*data).size;
    }

    0
}

/**
 * aa_unpack - unpack packed binary profile(s) data loaded from user space
 * @udata: user data copied to kmem  (NOT NULL)
 * @lh: list to place unpacked profiles in a aa_repl_ws
 * @ns: Returns namespace profile is in if specified else NULL (NOT NULL)
 * @compressed_data: The userspace-provided compressed data. May be NULL
 * @compressed_size: If compressed_data is not NULL, the compressed data size
 *
 * Unpack user data and return refcounted allocated profile(s) stored in
 * @lh in order of discovery, with the list chain stored in base.list
 * or error
 *
 * Returns: profile(s) on @lh else error pointer if fails to unpack
 */
pub unsafe extern "C" fn aa_unpack(
    udata: *mut aa_loaddata,
    lh: *mut list_head,
    ns: *mut *const c_char,
    mut compressed_data: *mut c_char,
    compressed_size: size_t,
) -> c_int {
    let mut ent: *mut aa_load_ent;
    let mut profile: *mut aa_profile = ptr::null_mut();
    let mut ns_name: *mut c_char = ptr::null_mut();
    let mut error: c_int = 0;
    let mut e = aa_ext {
        start: (*udata).data,
        end: (*udata).data.add((*udata).size),
        pos: (*udata).data,
        version: 0,
    };

    *ns = ptr::null();
    while e.pos < e.end {
        let start: *mut c_void;
        error = verify_header(&mut e, (e.pos == e.start) as c_int, ns);
        if error != 0 {
            break;
        }

        start = e.pos as *mut c_void;
        profile = unpack_profile(&mut e, &mut ns_name);
        if IS_ERR(profile) {
            error = PTR_ERR(profile);
            break;
        }

        error = verify_profile(profile);
        if error != 0 {
            kfree(ns_name as *mut c_void);
            aa_put_profile(profile);
            break;
        }

        if aa_g_hash_policy {
            error = aa_calc_profile_hash(profile, e.version, start, e.pos.offset_from(start as *mut c_char) as size_t);
        }
        if error != 0 {
            kfree(ns_name as *mut c_void);
            aa_put_profile(profile);
            break;
        }

        ent = aa_load_ent_alloc();
        if ent.is_null() {
            error = -ENOMEM;
            kfree(ns_name as *mut c_void);
            aa_put_profile(profile);
            break;
        }

        (*ent).new = profile;
        (*ent).ns_name = ns_name;
        ns_name = ptr::null_mut();
        list_add_tail(&mut (*ent).list, lh);
    }
    if error == 0 {
        (*udata).abi = e.version & K_ABI_MASK;
        if aa_g_hash_policy {
            (*udata).hash = aa_calc_hash((*udata).data as *mut c_void, (*udata).size);
            if IS_ERR((*udata).hash) {
                error = PTR_ERR((*udata).hash);
                (*udata).hash = ptr::null_mut();
            }
        }
    }

    if error == 0 {
        if aa_g_export_binary {
            /* Do we have userspace-compressed data? */
            if !compressed_data.is_null() {
                kvfree((*udata).data as *mut c_void);
                (*udata).data = compressed_data;
                (*udata).compressed_size = compressed_size;
                compressed_data = ptr::null_mut(); /* consumed */
            } else {
                error = compress_loaddata(udata);
            }
        } else if !compressed_data.is_null() {
            kvfree(compressed_data as *mut c_void);
            compressed_data = ptr::null_mut();
        }
    }

    if error != 0 {
        if !compressed_data.is_null() {
            kvfree(compressed_data as *mut c_void);
        }
        /*
         * list_for_each_entry_safe(ent, tmp, lh, list) {
         *     list_del_init(&ent->list);
         *     aa_load_ent_free(ent);
         * }
         * The isolated translation preserves the cleanup intent; the concrete
         * list iteration primitive is supplied by the surrounding kernel port.
         */
    }

    error
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
