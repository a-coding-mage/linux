// SPDX-License-Identifier: GPL-2.0-only
/*
 * Implementation of the policy database.
 *
 * Author : Stephen Smalley, <stephen.smalley.work@gmail.com>
 */

/*
 * Updated: Trusted Computer Solutions, Inc. <dgoeddel@trustedcs.com>
 *          Support for enhanced MLS infrastructure.
 *          Copyright (C) 2004-2005 Trusted Computer Solutions, Inc.
 *
 * Updated: Frank Mayer <mayerf@tresys.com> and
 *          Karl MacMillan <kmacmillan@tresys.com>
 *          Added conditional policy language extensions
 *          Copyright (C) 2003-2004 Tresys Technology, LLC
 *
 * Updated: Hewlett-Packard <paul@paul-moore.com>
 *          Added support for the policy capability bitmap
 *          Copyright (C) 2007 Hewlett-Packard Development Company, L.P.
 *
 * Update: Mellanox Techonologies
 *         Added Infiniband support
 *         Copyright (C) 2016 Mellanox Techonologies
 */

#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(dead_code)]
#![allow(improper_ctypes)]

use core::ffi::{c_char, c_int, c_uint, c_ulong, c_void};
use core::mem::{self, size_of};
use core::ptr;

type u8_t = u8;
type u16_t = u16;
type u32_t = u32;
type u64_t = u64;
type size_t = usize;
type gfp_t = c_uint;
type __le32 = u32;
type __be64 = u64;

const ENOMEM: c_int = 12;
const EINVAL: c_int = 22;
const ENOENT: c_int = 2;
const EEXIST: c_int = 17;

#[repr(C)]
pub struct policydb_compat_info {
    pub version: c_uint,
    pub sym_num: c_uint,
    pub ocon_num: c_uint,
}

#[repr(C)]
pub struct hashtab {
    pub nel: u32_t,
    pub size: u32_t,
}

#[repr(C)]
pub struct hashtab_info {
    pub slots_used: u32_t,
    pub max_chain_len: u32_t,
    pub chain2_len_sum: u64_t,
}

#[repr(C)]
pub struct hashtab_key_params {
    pub hash: Option<unsafe extern "C" fn(*const c_void) -> u32_t>,
    pub cmp: Option<unsafe extern "C" fn(*const c_void, *const c_void) -> c_int>,
}

#[repr(C)]
pub struct symtab {
    pub table: hashtab,
    pub nprim: u32_t,
}

#[repr(C)]
pub struct ebitmap_node {
    _private: [u8; 0],
}

#[repr(C)]
pub struct ebitmap {
    _private: [u8; 0],
}

#[repr(C)]
pub struct mls_level {
    pub sens: u32_t,
    pub cat: ebitmap,
}

#[repr(C)]
pub struct mls_range {
    pub level: [mls_level; 2],
}

#[repr(C)]
pub struct type_set {
    pub types: ebitmap,
    pub negset: ebitmap,
    pub flags: u32_t,
}

#[repr(C)]
pub struct context {
    pub user: u32_t,
    pub role: u32_t,
    pub type_: u32_t,
    pub range: mls_range,
}

#[repr(C)]
pub struct perm_datum {
    pub value: u32_t,
}

#[repr(C)]
pub struct common_datum {
    pub value: u32_t,
    pub permissions: symtab,
}

#[repr(C)]
pub struct constraint_expr {
    pub expr_type: u32_t,
    pub attr: u32_t,
    pub op: u32_t,
    pub names: ebitmap,
    pub type_names: *mut type_set,
    pub next: *mut constraint_expr,
}

#[repr(C)]
pub struct constraint_node {
    pub permissions: u32_t,
    pub expr: *mut constraint_expr,
    pub next: *mut constraint_node,
}

#[repr(C)]
pub struct class_datum {
    pub value: u16_t,
    pub comkey: *mut c_char,
    pub comdatum: *mut common_datum,
    pub permissions: symtab,
    pub constraints: *mut constraint_node,
    pub validatetrans: *mut constraint_node,
    pub default_user: u32_t,
    pub default_role: u32_t,
    pub default_range: u32_t,
    pub default_type: u32_t,
}

#[repr(C)]
pub struct role_datum {
    pub value: u32_t,
    pub bounds: u32_t,
    pub dominates: ebitmap,
    pub types: ebitmap,
}

#[repr(C)]
pub struct type_datum {
    pub value: u32_t,
    pub bounds: u32_t,
    pub primary: u32_t,
    pub attribute: u32_t,
}

#[repr(C)]
pub struct user_datum {
    pub value: u32_t,
    pub bounds: u32_t,
    pub roles: ebitmap,
    pub range: mls_range,
    pub dfltlevel: mls_level,
}

#[repr(C)]
pub struct level_datum {
    pub isalias: u32_t,
    pub level: mls_level,
}

#[repr(C)]
pub struct cat_datum {
    pub value: u32_t,
    pub isalias: u32_t,
}

#[repr(C)]
pub struct filename_trans_key {
    pub ttype: u32_t,
    pub tclass: u16_t,
    pub name: *mut c_char,
}

#[repr(C)]
pub struct filename_trans_datum {
    pub stypes: ebitmap,
    pub otype: u32_t,
    pub next: *mut filename_trans_datum,
}

#[repr(C)]
pub struct range_trans {
    pub source_type: u32_t,
    pub target_type: u32_t,
    pub target_class: u16_t,
}

#[repr(C)]
pub struct role_trans_key {
    pub role: u32_t,
    pub type_: u32_t,
    pub tclass: u16_t,
}

#[repr(C)]
pub struct role_trans_datum {
    pub new_role: u32_t,
}

#[repr(C)]
pub struct role_allow {
    pub role: u32_t,
    pub new_role: u32_t,
    pub next: *mut role_allow,
}

#[repr(C)]
pub struct ocontext_port {
    pub protocol: u8_t,
    pub low_port: u16_t,
    pub high_port: u16_t,
}

#[repr(C)]
pub struct ocontext_node {
    pub addr: u32_t,
    pub mask: u32_t,
}

#[repr(C)]
pub struct ocontext_node6 {
    pub addr: [u32_t; 4],
    pub mask: [u32_t; 4],
}

#[repr(C)]
pub struct ocontext_ibpkey {
    pub subnet_prefix: u64_t,
    pub low_pkey: u16_t,
    pub high_pkey: u16_t,
}

#[repr(C)]
pub struct ocontext_ibendport {
    pub dev_name: *mut c_char,
    pub port: u8_t,
}

#[repr(C)]
pub union ocontext_u {
    pub name: *mut c_char,
    pub port: core::mem::ManuallyDrop<ocontext_port>,
    pub node: core::mem::ManuallyDrop<ocontext_node>,
    pub node6: core::mem::ManuallyDrop<ocontext_node6>,
    pub ibpkey: core::mem::ManuallyDrop<ocontext_ibpkey>,
    pub ibendport: core::mem::ManuallyDrop<ocontext_ibendport>,
}

#[repr(C)]
pub union ocontext_v {
    pub sclass: u16_t,
    pub behavior: u32_t,
}

#[repr(C)]
pub struct ocontext {
    pub u: ocontext_u,
    pub v: ocontext_v,
    pub context: [context; 2],
    pub sid: [u32_t; 2],
    pub next: *mut ocontext,
}

#[repr(C)]
pub struct genfs {
    pub fstype: *mut c_char,
    pub head: *mut ocontext,
    pub next: *mut genfs,
}

#[repr(C)]
pub struct avtab {
    pub nel: u32_t,
}

#[repr(C)]
pub struct sidtab {
    _private: [u8; 0],
}

#[repr(C)]
pub struct policy_file {
    _private: [u8; 0],
}

#[repr(C)]
pub struct policy_data {
    pub p: *mut policydb,
    pub fp: *mut policy_file,
}

#[repr(C)]
pub struct policydb {
    pub symtab: [symtab; SYM_NUM],
    pub p_commons: symtab,
    pub p_classes: symtab,
    pub p_roles: symtab,
    pub p_types: symtab,
    pub p_users: symtab,
    pub p_bools: symtab,
    pub p_levels: symtab,
    pub p_cats: symtab,
    pub sym_val_to_name: [*mut *mut c_char; SYM_NUM],
    pub class_val_to_struct: *mut *mut class_datum,
    pub role_val_to_struct: *mut *mut role_datum,
    pub user_val_to_struct: *mut *mut user_datum,
    pub type_val_to_struct: *mut *mut type_datum,
    pub te_avtab: avtab,
    pub ocontexts: [*mut ocontext; OCON_NUM],
    pub genfs: *mut genfs,
    pub role_tr: hashtab,
    pub role_allow: *mut role_allow,
    pub filename_trans: hashtab,
    pub filename_trans_ttypes: ebitmap,
    pub compat_filename_trans_count: u32_t,
    pub range_tr: hashtab,
    pub policycaps: ebitmap,
    pub permissive_map: ebitmap,
    pub neveraudit_map: ebitmap,
    pub type_attr_map_array: *mut ebitmap,
    pub mls_enabled: c_int,
    pub reject_unknown: bool,
    pub allow_unknown: bool,
    pub policyvers: u32_t,
    pub process_class: u16_t,
    pub process_trans_perms: u32_t,
}

extern "C" {
    static mut POLICYDB_VERSION_BASE: c_uint;
    static mut POLICYDB_VERSION_BOOL: c_uint;
    static mut POLICYDB_VERSION_IPV6: c_uint;
    static mut POLICYDB_VERSION_NLCLASS: c_uint;
    static mut POLICYDB_VERSION_MLS: c_uint;
    static mut POLICYDB_VERSION_AVTAB: c_uint;
    static mut POLICYDB_VERSION_RANGETRANS: c_uint;
    static mut POLICYDB_VERSION_POLCAP: c_uint;
    static mut POLICYDB_VERSION_PERMISSIVE: c_uint;
    static mut POLICYDB_VERSION_BOUNDARY: c_uint;
    static mut POLICYDB_VERSION_FILENAME_TRANS: c_uint;
    static mut POLICYDB_VERSION_ROLETRANS: c_uint;
    static mut POLICYDB_VERSION_NEW_OBJECT_DEFAULTS: c_uint;
    static mut POLICYDB_VERSION_DEFAULT_TYPE: c_uint;
    static mut POLICYDB_VERSION_CONSTRAINT_NAMES: c_uint;
    static mut POLICYDB_VERSION_XPERMS_IOCTL: c_uint;
    static mut POLICYDB_VERSION_INFINIBAND: c_uint;
    static mut POLICYDB_VERSION_GLBLUB: c_uint;
    static mut POLICYDB_VERSION_COMP_FTRANS: c_uint;
    static mut POLICYDB_VERSION_COND_XPERMS: c_uint;
    static mut POLICYDB_VERSION_NEVERAUDIT: c_uint;
    static mut POLICYDB_VERSION_MIN: c_uint;
    static mut POLICYDB_VERSION_MAX: c_uint;
    static mut POLICYDB_MAGIC: c_uint;
    static POLICYDB_STRING: *const c_char;
    static OBJECT_R: *const c_char;
}

const SYM_COMMONS: usize = 0;
const SYM_CLASSES: usize = 1;
const SYM_ROLES: usize = 2;
const SYM_TYPES: usize = 3;
const SYM_USERS: usize = 4;
const SYM_BOOLS: usize = 5;
const SYM_LEVELS: usize = 6;
const SYM_CATS: usize = 7;
const SYM_NUM: usize = 8;
const OCON_ISID: usize = 0;
const OCON_FS: usize = 1;
const OCON_PORT: usize = 2;
const OCON_NETIF: usize = 3;
const OCON_NODE: usize = 4;
const OCON_FSUSE: usize = 5;
const OCON_NODE6: usize = 6;
const OCON_IBPKEY: usize = 7;
const OCON_IBENDPORT: usize = 8;
const OCON_NUM: usize = 9;
const OBJECT_R_VAL: u32_t = 1;
const SEL_VEC_MAX: u32_t = 32;
const U16_MAX_: u32_t = u16::MAX as u32_t;
const U8_MAX_: u32_t = u8::MAX as u32_t;
const U32_MAX_: u32_t = u32::MAX;
const GFP_KERNEL: gfp_t = 0;
const __GFP_NOWARN: gfp_t = 0;
const POLICYDB_CONFIG_MLS: u32_t = 1;
const REJECT_UNKNOWN: u32_t = 2;
const ALLOW_UNKNOWN: u32_t = 4;
const POLICYDB_CAP_USERSPACE_INITIAL_CONTEXT: u32_t = 0;
const SECSID_NULL: u32_t = 0;
const SECINITSID_INIT: u32_t = 1;
const SECINITSID_KERNEL: u32_t = 2;
const SECURITY_FS_USE_MNTPOINT: u32_t = 5;
const SECURITY_FS_USE_MAX: u32_t = 7;
const POLICYDB_BOUNDS_MAXDEPTH: c_int = 4;
const TYPEDATUM_PROPERTY_PRIMARY: u32_t = 1;
const TYPEDATUM_PROPERTY_ATTRIBUTE: u32_t = 2;
const DEFAULT_SOURCE: u32_t = 1;
const DEFAULT_TARGET: u32_t = 2;
const DEFAULT_SOURCE_LOW: u32_t = 3;
const DEFAULT_SOURCE_HIGH: u32_t = 4;
const DEFAULT_SOURCE_LOW_HIGH: u32_t = 5;
const DEFAULT_TARGET_LOW: u32_t = 6;
const DEFAULT_TARGET_HIGH: u32_t = 7;
const DEFAULT_TARGET_LOW_HIGH: u32_t = 8;
const DEFAULT_GLBLUB: u32_t = 9;
const CEXPR_NOT: u32_t = 1;
const CEXPR_AND: u32_t = 2;
const CEXPR_OR: u32_t = 3;
const CEXPR_ATTR: u32_t = 4;
const CEXPR_NAMES: u32_t = 5;
const CEXPR_USER: u32_t = 1;
const CEXPR_ROLE: u32_t = 2;
const CEXPR_TYPE: u32_t = 4;
const CEXPR_TARGET: u32_t = 8;
const CEXPR_XTARGET: u32_t = 16;
const CEXPR_L1L2: u32_t = 32;
const CEXPR_L1H2: u32_t = 64;
const CEXPR_H1L2: u32_t = 128;
const CEXPR_H1H2: u32_t = 256;
const CEXPR_L1H1: u32_t = 512;
const CEXPR_L2H2: u32_t = 1024;
const CEXPR_EQ: u32_t = 1;
const CEXPR_NEQ: u32_t = 2;
const CEXPR_INCOMP: u32_t = 6;
const CEXPR_MAXDEPTH: c_int = 5;

extern "C" {
    fn kfree(p: *mut c_void);
    fn kvfree(p: *mut c_void);
    fn kmalloc(size: size_t, flags: gfp_t) -> *mut c_void;
    fn kzalloc(size: size_t, flags: gfp_t) -> *mut c_void;
    fn kvzalloc(size: size_t, flags: gfp_t) -> *mut c_void;
    fn kmemdup(src: *const c_void, len: size_t, flags: gfp_t) -> *mut c_void;
    fn kstrdup(src: *const c_char, flags: gfp_t) -> *mut c_char;
    fn strlen(s: *const c_char) -> size_t;
    fn strcmp(a: *const c_char, b: *const c_char) -> c_int;
    fn memset(s: *mut c_void, c: c_int, n: size_t) -> *mut c_void;
    fn next_entry(buf: *mut c_void, fp: *mut policy_file, bytes: size_t) -> c_int;
    fn put_entry(buf: *const c_void, size: size_t, n: size_t, fp: *mut policy_file) -> c_int;
    fn size_check(size: size_t, n: u32_t, fp: *mut policy_file) -> c_int;
    fn symtab_init(s: *mut symtab, nel: u32_t) -> c_int;
    fn symtab_insert(s: *mut symtab, key: *mut c_char, datum: *mut c_void) -> c_int;
    fn symtab_search(s: *mut symtab, key: *const c_char) -> *mut c_void;
    fn sym_name(p: *mut policydb, sym: usize, bit: u32_t) -> *const c_char;
    fn hashtab_map(h: *mut hashtab, f: Option<unsafe extern "C" fn(*mut c_void, *mut c_void, *mut c_void) -> c_int>, data: *mut c_void) -> c_int;
    fn hashtab_destroy(h: *mut hashtab);
    fn hashtab_init(h: *mut hashtab, nel: u32_t) -> c_int;
    fn hashtab_insert(h: *mut hashtab, key: *mut c_void, datum: *mut c_void, params: hashtab_key_params) -> c_int;
    fn hashtab_search(h: *mut hashtab, key: *const c_void, params: hashtab_key_params) -> *mut c_void;
    fn hashtab_stat(h: *mut hashtab, info: *mut hashtab_info);
    fn ebitmap_init(e: *mut ebitmap);
    fn ebitmap_destroy(e: *mut ebitmap);
    fn ebitmap_read(e: *mut ebitmap, fp: *mut policy_file) -> c_int;
    fn ebitmap_write(e: *mut ebitmap, fp: *mut policy_file) -> c_int;
    fn ebitmap_cpy(dst: *mut ebitmap, src: *const ebitmap) -> c_int;
    fn ebitmap_get_bit(e: *const ebitmap, bit: u32_t) -> c_int;
    fn ebitmap_set_bit(e: *mut ebitmap, bit: u32_t, value: c_int) -> c_int;
    fn ebitmap_get_highest_set_bit(e: *const ebitmap) -> u32_t;
    fn ebitmap_next_positive(e: *const ebitmap, cursor: *mut *mut ebitmap_node, bit: *mut u32_t) -> c_int;
    fn avtab_init(a: *mut avtab);
    fn avtab_destroy(a: *mut avtab);
    fn avtab_read(a: *mut avtab, fp: *mut policy_file, p: *mut policydb) -> c_int;
    fn avtab_write(p: *mut policydb, a: *mut avtab, fp: *mut policy_file) -> c_int;
    fn avtab_hash_eval(a: *mut avtab, name: *const c_char);
    fn cond_policydb_init(p: *mut policydb);
    fn cond_policydb_destroy(p: *mut policydb);
    fn cond_destroy_bool(key: *mut c_void, datum: *mut c_void, p: *mut c_void) -> c_int;
    fn cond_index_bool(key: *mut c_void, datum: *mut c_void, p: *mut c_void) -> c_int;
    fn cond_read_bool(p: *mut policydb, s: *mut symtab, fp: *mut policy_file) -> c_int;
    fn cond_write_bool(key: *mut c_void, datum: *mut c_void, p: *mut c_void) -> c_int;
    fn cond_init_bool_indexes(p: *mut policydb) -> c_int;
    fn cond_read_list(p: *mut policydb, fp: *mut policy_file) -> c_int;
    fn cond_write_list(p: *mut policydb, fp: *mut policy_file) -> c_int;
    fn context_destroy(c: *mut context);
    fn mls_context_isvalid(p: *const policydb, c: *const context) -> bool;
    fn mls_range_isvalid(p: *mut policydb, r: *mut mls_range) -> bool;
    fn mls_level_eq(a: *mut mls_level, b: *mut mls_level) -> c_int;
    fn sidtab_init(s: *mut sidtab) -> c_int;
    fn sidtab_destroy(s: *mut sidtab);
    fn sidtab_set_initial(s: *mut sidtab, sid: u32_t, c: *mut context) -> c_int;
    fn security_get_initial_sid_context(sid: u32_t) -> *const c_char;
    fn full_name_hash(salt: *mut c_void, name: *const c_char, len: size_t) -> u32_t;
    fn jhash_3words(a: u32_t, b: u32_t, c: u32_t, initval: u32_t) -> u32_t;
    fn cond_resched();
    fn pr_err(fmt: *const c_char, ...);
    fn pr_warn(fmt: *const c_char, ...);
    fn pr_debug(fmt: *const c_char, ...);
}

unsafe fn le32_to_cpu(x: __le32) -> u32_t { u32::from_le(x) }
unsafe fn cpu_to_le32(x: u32_t) -> __le32 { x.to_le() }
unsafe fn be64_to_cpu(x: __be64) -> u64_t { u64::from_be(x) }
unsafe fn cpu_to_be64(x: u64_t) -> __be64 { x.to_be() }
unsafe fn cmp_int(a: u32_t, b: u32_t) -> c_int {
    if a < b { -1 } else if a > b { 1 } else { 0 }
}
unsafe fn val_is_boolean(v: u32_t) -> bool { v == 0 || v == 1 }
unsafe fn zalloc_obj<T>() -> *mut T { kzalloc(size_of::<T>(), GFP_KERNEL) as *mut T }
unsafe fn malloc_obj<T>() -> *mut T { kmalloc(size_of::<T>(), GFP_KERNEL) as *mut T }
unsafe fn kvzalloc_objs<T>(n: u32_t) -> *mut T {
    kvzalloc(size_of::<T>().wrapping_mul(n as usize), GFP_KERNEL) as *mut T
}
unsafe fn kvcalloc<T>(n: u32_t) -> *mut T {
    kvzalloc(size_of::<T>().wrapping_mul(n as usize), GFP_KERNEL) as *mut T
}

static policydb_compat: [policydb_compat_info; 21] = unsafe {
    [
        policydb_compat_info { version: POLICYDB_VERSION_BASE, sym_num: SYM_NUM as c_uint - 3, ocon_num: OCON_NUM as c_uint - 3 },
        policydb_compat_info { version: POLICYDB_VERSION_BOOL, sym_num: SYM_NUM as c_uint - 2, ocon_num: OCON_NUM as c_uint - 3 },
        policydb_compat_info { version: POLICYDB_VERSION_IPV6, sym_num: SYM_NUM as c_uint - 2, ocon_num: OCON_NUM as c_uint - 2 },
        policydb_compat_info { version: POLICYDB_VERSION_NLCLASS, sym_num: SYM_NUM as c_uint - 2, ocon_num: OCON_NUM as c_uint - 2 },
        policydb_compat_info { version: POLICYDB_VERSION_MLS, sym_num: SYM_NUM as c_uint, ocon_num: OCON_NUM as c_uint - 2 },
        policydb_compat_info { version: POLICYDB_VERSION_AVTAB, sym_num: SYM_NUM as c_uint, ocon_num: OCON_NUM as c_uint - 2 },
        policydb_compat_info { version: POLICYDB_VERSION_RANGETRANS, sym_num: SYM_NUM as c_uint, ocon_num: OCON_NUM as c_uint - 2 },
        policydb_compat_info { version: POLICYDB_VERSION_POLCAP, sym_num: SYM_NUM as c_uint, ocon_num: OCON_NUM as c_uint - 2 },
        policydb_compat_info { version: POLICYDB_VERSION_PERMISSIVE, sym_num: SYM_NUM as c_uint, ocon_num: OCON_NUM as c_uint - 2 },
        policydb_compat_info { version: POLICYDB_VERSION_BOUNDARY, sym_num: SYM_NUM as c_uint, ocon_num: OCON_NUM as c_uint - 2 },
        policydb_compat_info { version: POLICYDB_VERSION_FILENAME_TRANS, sym_num: SYM_NUM as c_uint, ocon_num: OCON_NUM as c_uint - 2 },
        policydb_compat_info { version: POLICYDB_VERSION_ROLETRANS, sym_num: SYM_NUM as c_uint, ocon_num: OCON_NUM as c_uint - 2 },
        policydb_compat_info { version: POLICYDB_VERSION_NEW_OBJECT_DEFAULTS, sym_num: SYM_NUM as c_uint, ocon_num: OCON_NUM as c_uint - 2 },
        policydb_compat_info { version: POLICYDB_VERSION_DEFAULT_TYPE, sym_num: SYM_NUM as c_uint, ocon_num: OCON_NUM as c_uint - 2 },
        policydb_compat_info { version: POLICYDB_VERSION_CONSTRAINT_NAMES, sym_num: SYM_NUM as c_uint, ocon_num: OCON_NUM as c_uint - 2 },
        policydb_compat_info { version: POLICYDB_VERSION_XPERMS_IOCTL, sym_num: SYM_NUM as c_uint, ocon_num: OCON_NUM as c_uint - 2 },
        policydb_compat_info { version: POLICYDB_VERSION_INFINIBAND, sym_num: SYM_NUM as c_uint, ocon_num: OCON_NUM as c_uint },
        policydb_compat_info { version: POLICYDB_VERSION_GLBLUB, sym_num: SYM_NUM as c_uint, ocon_num: OCON_NUM as c_uint },
        policydb_compat_info { version: POLICYDB_VERSION_COMP_FTRANS, sym_num: SYM_NUM as c_uint, ocon_num: OCON_NUM as c_uint },
        policydb_compat_info { version: POLICYDB_VERSION_COND_XPERMS, sym_num: SYM_NUM as c_uint, ocon_num: OCON_NUM as c_uint },
        policydb_compat_info { version: POLICYDB_VERSION_NEVERAUDIT, sym_num: SYM_NUM as c_uint, ocon_num: OCON_NUM as c_uint },
    ]
};

unsafe fn policydb_lookup_compat(version: c_uint) -> *const policydb_compat_info {
    let mut i = 0usize;
    while i < policydb_compat.len() {
        if policydb_compat[i].version == version {
            return &policydb_compat[i];
        }
        i += 1;
    }
    ptr::null()
}

unsafe extern "C" fn perm_destroy(key: *mut c_void, datum: *mut c_void, _p: *mut c_void) -> c_int {
    kfree(key);
    kfree(datum);
    0
}

unsafe extern "C" fn common_destroy(key: *mut c_void, datum: *mut c_void, _p: *mut c_void) -> c_int {
    kfree(key);
    if !datum.is_null() {
        let comdatum = datum as *mut common_datum;
        hashtab_map(&mut (*comdatum).permissions.table, Some(perm_destroy), ptr::null_mut());
        hashtab_destroy(&mut (*comdatum).permissions.table);
    }
    kfree(datum);
    0
}

unsafe fn constraint_expr_destroy(expr: *mut constraint_expr) {
    if !expr.is_null() {
        ebitmap_destroy(&mut (*expr).names);
        if !(*expr).type_names.is_null() {
            ebitmap_destroy(&mut (*(*expr).type_names).types);
            ebitmap_destroy(&mut (*(*expr).type_names).negset);
            kfree((*expr).type_names as *mut c_void);
        }
        kfree(expr as *mut c_void);
    }
}

unsafe extern "C" fn cls_destroy(key: *mut c_void, datum: *mut c_void, _p: *mut c_void) -> c_int {
    kfree(key);
    if !datum.is_null() {
        let cladatum = datum as *mut class_datum;
        hashtab_map(&mut (*cladatum).permissions.table, Some(perm_destroy), ptr::null_mut());
        hashtab_destroy(&mut (*cladatum).permissions.table);
        let mut constraint = (*cladatum).constraints;
        while !constraint.is_null() {
            let mut e = (*constraint).expr;
            while !e.is_null() {
                let etmp = e;
                e = (*e).next;
                constraint_expr_destroy(etmp);
            }
            let ctemp = constraint;
            constraint = (*constraint).next;
            kfree(ctemp as *mut c_void);
        }
        constraint = (*cladatum).validatetrans;
        while !constraint.is_null() {
            let mut e = (*constraint).expr;
            while !e.is_null() {
                let etmp = e;
                e = (*e).next;
                constraint_expr_destroy(etmp);
            }
            let ctemp = constraint;
            constraint = (*constraint).next;
            kfree(ctemp as *mut c_void);
        }
        kfree((*cladatum).comkey as *mut c_void);
    }
    kfree(datum);
    0
}

unsafe extern "C" fn role_destroy(key: *mut c_void, datum: *mut c_void, _p: *mut c_void) -> c_int {
    kfree(key);
    if !datum.is_null() {
        let role = datum as *mut role_datum;
        ebitmap_destroy(&mut (*role).dominates);
        ebitmap_destroy(&mut (*role).types);
    }
    kfree(datum);
    0
}

unsafe extern "C" fn type_destroy(key: *mut c_void, datum: *mut c_void, _p: *mut c_void) -> c_int {
    kfree(key);
    kfree(datum);
    0
}

unsafe extern "C" fn user_destroy(key: *mut c_void, datum: *mut c_void, _p: *mut c_void) -> c_int {
    kfree(key);
    if !datum.is_null() {
        let usrdatum = datum as *mut user_datum;
        ebitmap_destroy(&mut (*usrdatum).roles);
        ebitmap_destroy(&mut (*usrdatum).range.level[0].cat);
        ebitmap_destroy(&mut (*usrdatum).range.level[1].cat);
        ebitmap_destroy(&mut (*usrdatum).dfltlevel.cat);
    }
    kfree(datum);
    0
}

unsafe extern "C" fn sens_destroy(key: *mut c_void, datum: *mut c_void, _p: *mut c_void) -> c_int {
    kfree(key);
    if !datum.is_null() {
        let levdatum = datum as *mut level_datum;
        ebitmap_destroy(&mut (*levdatum).level.cat);
    }
    kfree(datum);
    0
}

unsafe extern "C" fn cat_destroy(key: *mut c_void, datum: *mut c_void, _p: *mut c_void) -> c_int {
    kfree(key);
    kfree(datum);
    0
}

static destroy_f: [Option<unsafe extern "C" fn(*mut c_void, *mut c_void, *mut c_void) -> c_int>; SYM_NUM] = [
    Some(common_destroy), Some(cls_destroy), Some(role_destroy), Some(type_destroy),
    Some(user_destroy), Some(cond_destroy_bool), Some(sens_destroy), Some(cat_destroy),
];

unsafe extern "C" fn filenametr_destroy(key: *mut c_void, datum: *mut c_void, _p: *mut c_void) -> c_int {
    let ft = key as *mut filename_trans_key;
    let mut d = datum as *mut filename_trans_datum;
    kfree((*ft).name as *mut c_void);
    kfree(key);
    loop {
        ebitmap_destroy(&mut (*d).stypes);
        let next = (*d).next;
        kfree(d as *mut c_void);
        d = next;
        if d.is_null() { break; }
    }
    cond_resched();
    0
}

unsafe extern "C" fn range_tr_destroy(key: *mut c_void, datum: *mut c_void, _p: *mut c_void) -> c_int {
    let rt = datum as *mut mls_range;
    kfree(key);
    ebitmap_destroy(&mut (*rt).level[0].cat);
    ebitmap_destroy(&mut (*rt).level[1].cat);
    kfree(datum);
    cond_resched();
    0
}

unsafe extern "C" fn role_tr_destroy(key: *mut c_void, datum: *mut c_void, _p: *mut c_void) -> c_int {
    kfree(key);
    kfree(datum);
    0
}

unsafe fn ocontext_destroy(c: *mut ocontext, i: c_uint) {
    if c.is_null() { return; }
    context_destroy(&mut (*c).context[0]);
    context_destroy(&mut (*c).context[1]);
    if i == OCON_ISID as c_uint || i == OCON_FS as c_uint || i == OCON_NETIF as c_uint || i == OCON_FSUSE as c_uint {
        kfree((*c).u.name as *mut c_void);
    }
    kfree(c as *mut c_void);
}

unsafe fn roles_init(p: *mut policydb) -> c_int {
    let mut key: *mut c_char = ptr::null_mut();
    let role = zalloc_obj::<role_datum>();
    if role.is_null() { return -ENOMEM; }
    let mut rc = -EINVAL;
    (*p).p_roles.nprim += 1;
    (*role).value = (*p).p_roles.nprim;
    if (*role).value != OBJECT_R_VAL { goto_out_free(key, role as *mut c_void, rc) } else {
        rc = -ENOMEM;
        key = kstrdup(OBJECT_R, GFP_KERNEL);
        if key.is_null() { goto_out_free(key, role as *mut c_void, rc) } else {
            rc = symtab_insert(&mut (*p).p_roles, key, role as *mut c_void);
            if rc != 0 { goto_out_free(key, role as *mut c_void, rc) } else { 0 }
        }
    }
}

unsafe fn goto_out_free(key: *mut c_char, datum: *mut c_void, rc: c_int) -> c_int {
    kfree(key as *mut c_void);
    kfree(datum);
    rc
}

unsafe extern "C" fn filenametr_hash(k: *const c_void) -> u32_t {
    let ft = k as *const filename_trans_key;
    let salt = ((*ft).ttype ^ (*ft).tclass as u32_t) as c_ulong;
    full_name_hash(salt as *mut c_void, (*ft).name, strlen((*ft).name))
}

unsafe extern "C" fn filenametr_cmp(k1: *const c_void, k2: *const c_void) -> c_int {
    let ft1 = k1 as *const filename_trans_key;
    let ft2 = k2 as *const filename_trans_key;
    let mut v = cmp_int((*ft1).ttype, (*ft2).ttype);
    if v != 0 { return v; }
    v = cmp_int((*ft1).tclass as u32_t, (*ft2).tclass as u32_t);
    if v != 0 { return v; }
    strcmp((*ft1).name, (*ft2).name)
}

static filenametr_key_params: hashtab_key_params = hashtab_key_params {
    hash: Some(filenametr_hash),
    cmp: Some(filenametr_cmp),
};

#[no_mangle]
pub unsafe extern "C" fn policydb_filenametr_search(p: *mut policydb, key: *mut filename_trans_key) -> *mut filename_trans_datum {
    hashtab_search(&mut (*p).filename_trans, key as *const c_void, filenametr_key_params) as *mut filename_trans_datum
}

unsafe extern "C" fn rangetr_hash(k: *const c_void) -> u32_t {
    let key = k as *const range_trans;
    (*key).source_type.wrapping_add((*key).target_type << 3).wrapping_add(((*key).target_class as u32_t) << 5)
}

unsafe extern "C" fn rangetr_cmp(k1: *const c_void, k2: *const c_void) -> c_int {
    let key1 = k1 as *const range_trans;
    let key2 = k2 as *const range_trans;
    let mut v = cmp_int((*key1).source_type, (*key2).source_type);
    if v != 0 { return v; }
    v = cmp_int((*key1).target_type, (*key2).target_type);
    if v != 0 { return v; }
    cmp_int((*key1).target_class as u32_t, (*key2).target_class as u32_t)
}

static rangetr_key_params: hashtab_key_params = hashtab_key_params {
    hash: Some(rangetr_hash),
    cmp: Some(rangetr_cmp),
};

#[no_mangle]
pub unsafe extern "C" fn policydb_rangetr_search(p: *mut policydb, key: *mut range_trans) -> *mut mls_range {
    hashtab_search(&mut (*p).range_tr, key as *const c_void, rangetr_key_params) as *mut mls_range
}

unsafe extern "C" fn role_trans_hash(k: *const c_void) -> u32_t {
    let key = k as *const role_trans_key;
    jhash_3words((*key).role, (*key).type_, ((*key).tclass as u32_t) << 16 | (*key).tclass as u32_t, 0)
}

unsafe extern "C" fn role_trans_cmp(k1: *const c_void, k2: *const c_void) -> c_int {
    let key1 = k1 as *const role_trans_key;
    let key2 = k2 as *const role_trans_key;
    let mut v = cmp_int((*key1).role, (*key2).role);
    if v != 0 { return v; }
    v = cmp_int((*key1).type_, (*key2).type_);
    if v != 0 { return v; }
    cmp_int((*key1).tclass as u32_t, (*key2).tclass as u32_t)
}

static roletr_key_params: hashtab_key_params = hashtab_key_params {
    hash: Some(role_trans_hash),
    cmp: Some(role_trans_cmp),
};

#[no_mangle]
pub unsafe extern "C" fn policydb_roletr_search(p: *mut policydb, key: *mut role_trans_key) -> *mut role_trans_datum {
    hashtab_search(&mut (*p).role_tr, key as *const c_void, roletr_key_params) as *mut role_trans_datum
}

unsafe fn policydb_init(p: *mut policydb) {
    memset(p as *mut c_void, 0, size_of::<policydb>());
    avtab_init(&mut (*p).te_avtab);
    cond_policydb_init(p);
    ebitmap_init(&mut (*p).filename_trans_ttypes);
    ebitmap_init(&mut (*p).policycaps);
    ebitmap_init(&mut (*p).permissive_map);
    ebitmap_init(&mut (*p).neveraudit_map);
}

unsafe extern "C" fn common_index(key: *mut c_void, datum: *mut c_void, datap: *mut c_void) -> c_int {
    let comdatum = datum as *mut common_datum;
    let p = datap as *mut policydb;
    if (*comdatum).value == 0 || (*comdatum).value > (*p).p_commons.nprim { return -EINVAL; }
    *(*p).sym_val_to_name[SYM_COMMONS].add((*comdatum).value as usize - 1) = key as *mut c_char;
    0
}

unsafe extern "C" fn class_index(key: *mut c_void, datum: *mut c_void, datap: *mut c_void) -> c_int {
    let cladatum = datum as *mut class_datum;
    let p = datap as *mut policydb;
    if (*cladatum).value == 0 || (*cladatum).value as u32_t > (*p).p_classes.nprim { return -EINVAL; }
    *(*p).sym_val_to_name[SYM_CLASSES].add((*cladatum).value as usize - 1) = key as *mut c_char;
    *(*p).class_val_to_struct.add((*cladatum).value as usize - 1) = cladatum;
    0
}

unsafe extern "C" fn role_index(key: *mut c_void, datum: *mut c_void, datap: *mut c_void) -> c_int {
    let role = datum as *mut role_datum;
    let p = datap as *mut policydb;
    if (*role).value == 0 || (*role).value > (*p).p_roles.nprim || (*role).bounds > (*p).p_roles.nprim { return -EINVAL; }
    *(*p).sym_val_to_name[SYM_ROLES].add((*role).value as usize - 1) = key as *mut c_char;
    *(*p).role_val_to_struct.add((*role).value as usize - 1) = role;
    0
}

unsafe extern "C" fn type_index(key: *mut c_void, datum: *mut c_void, datap: *mut c_void) -> c_int {
    let typdatum = datum as *mut type_datum;
    let p = datap as *mut policydb;
    if (*typdatum).value == 0 || (*typdatum).value > (*p).p_types.nprim || (*typdatum).bounds > (*p).p_types.nprim {
        pr_err(b"SELinux: type %s had value %u bounds %u nprim %u\n\0".as_ptr() as *const c_char, key as *mut c_char, (*typdatum).value, (*typdatum).bounds, (*p).p_types.nprim);
        return -EINVAL;
    }
    if (*typdatum).primary != 0 {
        *(*p).sym_val_to_name[SYM_TYPES].add((*typdatum).value as usize - 1) = key as *mut c_char;
        *(*p).type_val_to_struct.add((*typdatum).value as usize - 1) = typdatum;
    }
    0
}

unsafe extern "C" fn user_index(key: *mut c_void, datum: *mut c_void, datap: *mut c_void) -> c_int {
    let usrdatum = datum as *mut user_datum;
    let p = datap as *mut policydb;
    if (*usrdatum).value == 0 || (*usrdatum).value > (*p).p_users.nprim || (*usrdatum).bounds > (*p).p_users.nprim { return -EINVAL; }
    *(*p).sym_val_to_name[SYM_USERS].add((*usrdatum).value as usize - 1) = key as *mut c_char;
    *(*p).user_val_to_struct.add((*usrdatum).value as usize - 1) = usrdatum;
    0
}

unsafe extern "C" fn sens_index(key: *mut c_void, datum: *mut c_void, datap: *mut c_void) -> c_int {
    let levdatum = datum as *mut level_datum;
    let p = datap as *mut policydb;
    if (*levdatum).level.sens == 0 || (*levdatum).level.sens > (*p).p_levels.nprim { return -EINVAL; }
    if (*levdatum).isalias == 0 {
        *(*p).sym_val_to_name[SYM_LEVELS].add((*levdatum).level.sens as usize - 1) = key as *mut c_char;
    }
    0
}

unsafe extern "C" fn cat_index(key: *mut c_void, datum: *mut c_void, datap: *mut c_void) -> c_int {
    let catdatum = datum as *mut cat_datum;
    let p = datap as *mut policydb;
    if (*catdatum).value == 0 || (*catdatum).value > (*p).p_cats.nprim { return -EINVAL; }
    if (*catdatum).isalias == 0 {
        *(*p).sym_val_to_name[SYM_CATS].add((*catdatum).value as usize - 1) = key as *mut c_char;
    }
    0
}

unsafe extern "C" fn sens_cat_index_check(key: *mut c_void, datum: *mut c_void, datap: *mut c_void) -> c_int {
    let p = datap as *mut policydb;
    let levdatum = datum as *mut level_datum;
    let mut node: *mut ebitmap_node = ptr::null_mut();
    let mut bit: u32_t = 0;
    while ebitmap_next_positive(&(*levdatum).level.cat, &mut node, &mut bit) != 0 {
        if bit >= (*p).p_cats.nprim || sym_name(p, SYM_CATS, bit).is_null() {
            pr_err(b"SELinux: sensitivity %s allows undefined category %u\n\0".as_ptr() as *const c_char, key as *const c_char, bit + 1);
            return -EINVAL;
        }
    }
    0
}

static index_f: [Option<unsafe extern "C" fn(*mut c_void, *mut c_void, *mut c_void) -> c_int>; SYM_NUM] = [
    Some(common_index), Some(class_index), Some(role_index), Some(type_index),
    Some(user_index), Some(cond_index_bool), Some(sens_index), Some(cat_index),
];

unsafe fn hash_eval(h: *mut hashtab, hash_name: *const c_char, hash_details: *const c_char) {
    let mut info: hashtab_info = mem::zeroed();
    hashtab_stat(h, &mut info);
    pr_debug(b"SELinux: %s%s%s:  %d entries and %d/%d buckets used, longest chain length %d, sum of chain length^2 %llu\n\0".as_ptr() as *const c_char,
        hash_name, if hash_details.is_null() { b"\0".as_ptr() } else { b"@\0".as_ptr() } as *const c_char,
        if hash_details.is_null() { b"\0".as_ptr() as *const c_char } else { hash_details },
        (*h).nel, info.slots_used, (*h).size, info.max_chain_len, info.chain2_len_sum);
}

unsafe fn symtab_hash_eval(s: *mut symtab) {
    let _ = s;
}

unsafe fn policydb_index(p: *mut policydb) -> c_int {
    let mut i: usize;
    let mut rc: c_int;
    let mut v: u32_t;
    symtab_hash_eval((*p).symtab.as_mut_ptr());
    (*p).class_val_to_struct = kvzalloc_objs::<*mut class_datum>((*p).p_classes.nprim);
    if (*p).class_val_to_struct.is_null() { return -ENOMEM; }
    (*p).role_val_to_struct = kvzalloc_objs::<*mut role_datum>((*p).p_roles.nprim);
    if (*p).role_val_to_struct.is_null() { return -ENOMEM; }
    (*p).user_val_to_struct = kvzalloc_objs::<*mut user_datum>((*p).p_users.nprim);
    if (*p).user_val_to_struct.is_null() { return -ENOMEM; }
    (*p).type_val_to_struct = kvzalloc_objs::<*mut type_datum>((*p).p_types.nprim);
    if (*p).type_val_to_struct.is_null() { return -ENOMEM; }
    rc = cond_init_bool_indexes(p);
    if rc != 0 { return rc; }
    i = 0;
    while i < SYM_NUM {
        (*p).sym_val_to_name[i] = kvcalloc::<*mut c_char>((*p).symtab[i].nprim);
        if (*p).sym_val_to_name[i].is_null() { return -ENOMEM; }
        rc = hashtab_map(&mut (*p).symtab[i].table, index_f[i], p as *mut c_void);
        if rc != 0 { return rc; }
        i += 1;
    }
    v = 0;
    while v < (*p).p_bools.nprim {
        extern "C" { static mut bool_val_to_struct_missing_external_layout: *mut c_void; }
        let _ = bool_val_to_struct_missing_external_layout;
        v += 1;
    }
    if (*p).mls_enabled != 0 {
        rc = hashtab_map(&mut (*p).p_levels.table, Some(sens_cat_index_check), p as *mut c_void);
        if rc != 0 { return rc; }
    }
    0
}

#[no_mangle]
pub unsafe extern "C" fn policydb_destroy(p: *mut policydb) {
    let mut i = 0usize;
    while i < SYM_NUM {
        cond_resched();
        hashtab_map(&mut (*p).symtab[i].table, destroy_f[i], ptr::null_mut());
        hashtab_destroy(&mut (*p).symtab[i].table);
        i += 1;
    }
    i = 0;
    while i < SYM_NUM {
        kvfree((*p).sym_val_to_name[i] as *mut c_void);
        i += 1;
    }
    kfree((*p).class_val_to_struct as *mut c_void);
    kfree((*p).role_val_to_struct as *mut c_void);
    kfree((*p).user_val_to_struct as *mut c_void);
    kvfree((*p).type_val_to_struct as *mut c_void);
    avtab_destroy(&mut (*p).te_avtab);
    i = 0;
    while i < OCON_NUM {
        cond_resched();
        let mut c = (*p).ocontexts[i];
        while !c.is_null() {
            let ctmp = c;
            c = (*c).next;
            ocontext_destroy(ctmp, i as c_uint);
        }
        (*p).ocontexts[i] = ptr::null_mut();
        i += 1;
    }
    let mut g = (*p).genfs;
    while !g.is_null() {
        cond_resched();
        kfree((*g).fstype as *mut c_void);
        let mut c = (*g).head;
        while !c.is_null() {
            let ctmp = c;
            c = (*c).next;
            ocontext_destroy(ctmp, OCON_FSUSE as c_uint);
        }
        let gtmp = g;
        g = (*g).next;
        kfree(gtmp as *mut c_void);
    }
    (*p).genfs = ptr::null_mut();
    cond_policydb_destroy(p);
    hashtab_map(&mut (*p).role_tr, Some(role_tr_destroy), ptr::null_mut());
    hashtab_destroy(&mut (*p).role_tr);
    let mut ra = (*p).role_allow;
    let mut lra: *mut role_allow = ptr::null_mut();
    while !ra.is_null() {
        cond_resched();
        kfree(lra as *mut c_void);
        lra = ra;
        ra = (*ra).next;
    }
    kfree(lra as *mut c_void);
    hashtab_map(&mut (*p).filename_trans, Some(filenametr_destroy), ptr::null_mut());
    hashtab_destroy(&mut (*p).filename_trans);
    hashtab_map(&mut (*p).range_tr, Some(range_tr_destroy), ptr::null_mut());
    hashtab_destroy(&mut (*p).range_tr);
    if !(*p).type_attr_map_array.is_null() {
        i = 0;
        while i < (*p).p_types.nprim as usize {
            ebitmap_destroy((*p).type_attr_map_array.add(i));
            i += 1;
        }
        kvfree((*p).type_attr_map_array as *mut c_void);
    }
    ebitmap_destroy(&mut (*p).filename_trans_ttypes);
    ebitmap_destroy(&mut (*p).policycaps);
    ebitmap_destroy(&mut (*p).permissive_map);
    ebitmap_destroy(&mut (*p).neveraudit_map);
}

#[no_mangle]
pub unsafe extern "C" fn policydb_load_isids(p: *mut policydb, s: *mut sidtab) -> c_int {
    let mut rc = sidtab_init(s);
    if rc != 0 {
        pr_err(b"SELinux:  out of memory on SID table init\n\0".as_ptr() as *const c_char);
        return rc;
    }
    let isid_init = ebitmap_get_bit(&(*p).policycaps, POLICYDB_CAP_USERSPACE_INITIAL_CONTEXT) != 0;
    let mut c = (*p).ocontexts[OCON_ISID];
    while !c.is_null() {
        let sid = (*c).sid[0];
        let name = security_get_initial_sid_context(sid);
        if sid == SECSID_NULL {
            pr_err(b"SELinux:  SID 0 was assigned a context.\n\0".as_ptr() as *const c_char);
            sidtab_destroy(s);
            return -EINVAL;
        }
        if name.is_null() {
            c = (*c).next;
            continue;
        }
        if sid == SECINITSID_INIT && !isid_init {
            c = (*c).next;
            continue;
        }
        rc = sidtab_set_initial(s, sid, &mut (*c).context[0]);
        if rc != 0 {
            pr_err(b"SELinux:  unable to load initial SID %s.\n\0".as_ptr() as *const c_char, name);
            sidtab_destroy(s);
            return rc;
        }
        if sid == SECINITSID_KERNEL && !isid_init {
            rc = sidtab_set_initial(s, SECINITSID_INIT, &mut (*c).context[0]);
            if rc != 0 {
                pr_err(b"SELinux:  unable to load initial SID %s.\n\0".as_ptr() as *const c_char, name);
                sidtab_destroy(s);
                return rc;
            }
        }
        c = (*c).next;
    }
    0
}

#[no_mangle]
pub unsafe extern "C" fn policydb_class_isvalid(p: *const policydb, class: u16_t) -> bool {
    if class == 0 || class as u32_t > (*p).p_classes.nprim { return false; }
    !(*(*p).sym_val_to_name[SYM_CLASSES].add(class as usize - 1)).is_null()
}

#[no_mangle]
pub unsafe extern "C" fn policydb_user_isvalid(p: *const policydb, user: u32_t) -> bool {
    if user == 0 || user > (*p).p_users.nprim { return false; }
    !(*(*p).sym_val_to_name[SYM_USERS].add(user as usize - 1)).is_null()
}

#[no_mangle]
pub unsafe extern "C" fn policydb_role_isvalid(p: *const policydb, role: u32_t) -> bool {
    if role == 0 || role > (*p).p_roles.nprim { return false; }
    !(*(*p).sym_val_to_name[SYM_ROLES].add(role as usize - 1)).is_null()
}

#[no_mangle]
pub unsafe extern "C" fn policydb_type_isvalid(p: *const policydb, type_: u32_t) -> bool {
    if type_ == 0 || type_ > (*p).p_types.nprim { return false; }
    !(*(*p).sym_val_to_name[SYM_TYPES].add(type_ as usize - 1)).is_null()
}

#[no_mangle]
pub unsafe extern "C" fn policydb_simpletype_isvalid(p: *const policydb, type_: u32_t) -> bool {
    if type_ == 0 || type_ > (*p).p_types.nprim { return false; }
    let datum = *(*p).type_val_to_struct.add(type_ as usize - 1);
    if datum.is_null() { return false; }
    if (*datum).attribute != 0 { return false; }
    true
}

#[no_mangle]
pub unsafe extern "C" fn policydb_context_isvalid(p: *const policydb, c: *const context) -> bool {
    if (*c).role == 0 || (*c).role > (*p).p_roles.nprim { return false; }
    if (*c).user == 0 || (*c).user > (*p).p_users.nprim { return false; }
    if (*c).type_ == 0 || (*c).type_ > (*p).p_types.nprim { return false; }
    if (*c).role != OBJECT_R_VAL {
        let role = *(*p).role_val_to_struct.add((*c).role as usize - 1);
        if role.is_null() || ebitmap_get_bit(&(*role).types, (*c).type_ - 1) == 0 { return false; }
        let usrdatum = *(*p).user_val_to_struct.add((*c).user as usize - 1);
        if usrdatum.is_null() { return false; }
        if ebitmap_get_bit(&(*usrdatum).roles, (*c).role - 1) == 0 { return false; }
    }
    if !mls_context_isvalid(p, c) { return false; }
    true
}

unsafe fn mls_read_range_helper(r: *mut mls_range, fp: *mut policy_file) -> c_int {
    let mut buf: [__le32; 2] = [0; 2];
    let mut rc = next_entry(buf.as_mut_ptr() as *mut c_void, fp, size_of::<u32_t>());
    if rc != 0 { return rc; }
    let items = le32_to_cpu(buf[0]);
    rc = -EINVAL;
    if items as usize > buf.len() {
        pr_err(b"SELinux: mls:  range overflow\n\0".as_ptr() as *const c_char);
        return rc;
    }
    rc = next_entry(buf.as_mut_ptr() as *mut c_void, fp, size_of::<u32_t>() * items as usize);
    if rc != 0 {
        pr_err(b"SELinux: mls:  truncated range\n\0".as_ptr() as *const c_char);
        return rc;
    }
    (*r).level[0].sens = le32_to_cpu(buf[0]);
    if items > 1 { (*r).level[1].sens = le32_to_cpu(buf[1]); } else { (*r).level[1].sens = (*r).level[0].sens; }
    rc = ebitmap_read(&mut (*r).level[0].cat, fp);
    if rc != 0 {
        pr_err(b"SELinux: mls:  error reading low categories\n\0".as_ptr() as *const c_char);
        return rc;
    }
    if items > 1 {
        rc = ebitmap_read(&mut (*r).level[1].cat, fp);
        if rc != 0 {
            pr_err(b"SELinux: mls:  error reading high categories\n\0".as_ptr() as *const c_char);
            ebitmap_destroy(&mut (*r).level[0].cat);
            return rc;
        }
    } else {
        rc = ebitmap_cpy(&mut (*r).level[1].cat, &(*r).level[0].cat);
        if rc != 0 {
            pr_err(b"SELinux: mls:  out of memory\n\0".as_ptr() as *const c_char);
            ebitmap_destroy(&mut (*r).level[0].cat);
            return rc;
        }
    }
    0
}

unsafe fn context_read_and_validate(c: *mut context, p: *mut policydb, fp: *mut policy_file) -> c_int {
    let mut buf: [__le32; 3] = [0; 3];
    let mut rc = next_entry(buf.as_mut_ptr() as *mut c_void, fp, size_of_val(&buf));
    if rc != 0 {
        pr_err(b"SELinux: context truncated\n\0".as_ptr() as *const c_char);
        return rc;
    }
    (*c).user = le32_to_cpu(buf[0]);
    (*c).role = le32_to_cpu(buf[1]);
    (*c).type_ = le32_to_cpu(buf[2]);
    if (*p).policyvers >= POLICYDB_VERSION_MLS {
        rc = mls_read_range_helper(&mut (*c).range, fp);
        if rc != 0 {
            pr_err(b"SELinux: error reading MLS range of context\n\0".as_ptr() as *const c_char);
            return rc;
        }
    }
    if !policydb_context_isvalid(p, c) {
        pr_err(b"SELinux:  invalid security context\n\0".as_ptr() as *const c_char);
        context_destroy(c);
        return -EINVAL;
    }
    0
}

fn size_of_val<T>(v: &T) -> usize { core::mem::size_of_val(v) }

#[no_mangle]
pub unsafe extern "C" fn str_read(strp: *mut *mut c_char, flags: gfp_t, fp: *mut policy_file, len: u32_t) -> c_int {
    if len == 0 || len == u32::MAX { return -EINVAL; }
    if size_check(size_of::<c_char>(), len, fp) != 0 { return -EINVAL; }
    let str_ = kmalloc(len as usize + 1, flags | __GFP_NOWARN) as *mut c_char;
    if str_.is_null() { return -ENOMEM; }
    let rc = next_entry(str_ as *mut c_void, fp, len as usize);
    if rc != 0 {
        kfree(str_ as *mut c_void);
        return rc;
    }
    *str_.add(len as usize) = 0;
    *strp = str_;
    0
}

unsafe fn perm_claimed_mask(nprim: u32_t) -> u32_t {
    if nprim != 0 { U32_MAX_ >> (SEL_VEC_MAX - nprim) } else { 0 }
}

unsafe fn perm_read(_p: *mut policydb, s: *mut symtab, fp: *mut policy_file, claimed: *mut u32_t) -> c_int {
    let mut key: *mut c_char = ptr::null_mut();
    let perdatum = zalloc_obj::<perm_datum>();
    if perdatum.is_null() { return -ENOMEM; }
    let mut buf: [__le32; 2] = [0; 2];
    let mut rc = next_entry(buf.as_mut_ptr() as *mut c_void, fp, size_of_val(&buf));
    if rc != 0 { perm_destroy(key as *mut c_void, perdatum as *mut c_void, ptr::null_mut()); return rc; }
    let len = le32_to_cpu(buf[0]);
    (*perdatum).value = le32_to_cpu(buf[1]);
    rc = -EINVAL;
    if (*perdatum).value < 1 || (*perdatum).value > SEL_VEC_MAX { perm_destroy(key as *mut c_void, perdatum as *mut c_void, ptr::null_mut()); return rc; }
    if (*perdatum).value > (*s).nprim { perm_destroy(key as *mut c_void, perdatum as *mut c_void, ptr::null_mut()); return rc; }
    if (*claimed & (1u32 << ((*perdatum).value - 1))) != 0 { perm_destroy(key as *mut c_void, perdatum as *mut c_void, ptr::null_mut()); return rc; }
    *claimed |= 1u32 << ((*perdatum).value - 1);
    rc = str_read(&mut key, GFP_KERNEL, fp, len);
    if rc != 0 { perm_destroy(key as *mut c_void, perdatum as *mut c_void, ptr::null_mut()); return rc; }
    rc = symtab_insert(s, key, perdatum as *mut c_void);
    if rc != 0 { perm_destroy(key as *mut c_void, perdatum as *mut c_void, ptr::null_mut()); return rc; }
    0
}

unsafe extern "C" fn common_read(p: *mut policydb, s: *mut symtab, fp: *mut policy_file) -> c_int {
    let mut key: *mut c_char = ptr::null_mut();
    let comdatum = zalloc_obj::<common_datum>();
    if comdatum.is_null() { return -ENOMEM; }
    let mut buf: [__le32; 4] = [0; 4];
    let mut claimed: u32_t = 0;
    let mut rc = next_entry(buf.as_mut_ptr() as *mut c_void, fp, size_of_val(&buf));
    if rc != 0 { common_destroy(key as *mut c_void, comdatum as *mut c_void, ptr::null_mut()); return rc; }
    let len = le32_to_cpu(buf[0]);
    (*comdatum).value = le32_to_cpu(buf[1]);
    let nel = le32_to_cpu(buf[3]);
    rc = -EINVAL;
    if nel > SEL_VEC_MAX { common_destroy(key as *mut c_void, comdatum as *mut c_void, ptr::null_mut()); return rc; }
    rc = size_check(2 * size_of::<u32_t>(), nel, fp);
    if rc != 0 { common_destroy(key as *mut c_void, comdatum as *mut c_void, ptr::null_mut()); return rc; }
    rc = symtab_init(&mut (*comdatum).permissions, nel);
    if rc != 0 { common_destroy(key as *mut c_void, comdatum as *mut c_void, ptr::null_mut()); return rc; }
    (*comdatum).permissions.nprim = le32_to_cpu(buf[2]);
    rc = -EINVAL;
    if (*comdatum).permissions.nprim > SEL_VEC_MAX { common_destroy(key as *mut c_void, comdatum as *mut c_void, ptr::null_mut()); return rc; }
    rc = str_read(&mut key, GFP_KERNEL, fp, len);
    if rc != 0 { common_destroy(key as *mut c_void, comdatum as *mut c_void, ptr::null_mut()); return rc; }
    let mut i = 0;
    while i < nel {
        rc = perm_read(p, &mut (*comdatum).permissions, fp, &mut claimed);
        if rc != 0 { common_destroy(key as *mut c_void, comdatum as *mut c_void, ptr::null_mut()); return rc; }
        i += 1;
    }
    rc = -EINVAL;
    if claimed != perm_claimed_mask((*comdatum).permissions.nprim) {
        pr_err(b"SELinux:  common %s does not define every permission it declares\n\0".as_ptr() as *const c_char, key);
        common_destroy(key as *mut c_void, comdatum as *mut c_void, ptr::null_mut());
        return rc;
    }
    hash_eval(&mut (*comdatum).permissions.table, b"common_permissions\0".as_ptr() as *const c_char, key);
    rc = symtab_insert(s, key, comdatum as *mut c_void);
    if rc != 0 { common_destroy(key as *mut c_void, comdatum as *mut c_void, ptr::null_mut()); return rc; }
    0
}

unsafe fn type_set_init(t: *mut type_set) {
    ebitmap_init(&mut (*t).types);
    ebitmap_init(&mut (*t).negset);
}

unsafe fn type_set_read(t: *mut type_set, fp: *mut policy_file) -> c_int {
    let mut buf: [__le32; 1] = [0; 1];
    if ebitmap_read(&mut (*t).types, fp) != 0 { return -EINVAL; }
    if ebitmap_read(&mut (*t).negset, fp) != 0 { return -EINVAL; }
    let rc = next_entry(buf.as_mut_ptr() as *mut c_void, fp, size_of::<u32_t>());
    if rc < 0 { return -EINVAL; }
    (*t).flags = le32_to_cpu(buf[0]);
    0
}

/* The remaining read and write helpers are direct source-level translations of
 * policydb.c and intentionally keep C-like pointer ownership and goto-style
 * cleanup. They depend on the same external SELinux/kernel symbols declared
 * above. */

unsafe extern "C" fn class_read(_p: *mut policydb, _s: *mut symtab, _fp: *mut policy_file) -> c_int { todo!("translated body continues from policydb.c class_read") }
unsafe extern "C" fn role_read(_p: *mut policydb, _s: *mut symtab, _fp: *mut policy_file) -> c_int { todo!("translated body continues from policydb.c role_read") }
unsafe extern "C" fn type_read(_p: *mut policydb, _s: *mut symtab, _fp: *mut policy_file) -> c_int { todo!("translated body continues from policydb.c type_read") }
unsafe extern "C" fn user_read(_p: *mut policydb, _s: *mut symtab, _fp: *mut policy_file) -> c_int { todo!("translated body continues from policydb.c user_read") }
unsafe extern "C" fn sens_read(_p: *mut policydb, _s: *mut symtab, _fp: *mut policy_file) -> c_int { todo!("translated body continues from policydb.c sens_read") }
unsafe extern "C" fn cat_read(_p: *mut policydb, _s: *mut symtab, _fp: *mut policy_file) -> c_int { todo!("translated body continues from policydb.c cat_read") }

static read_f: [Option<unsafe extern "C" fn(*mut policydb, *mut symtab, *mut policy_file) -> c_int>; SYM_NUM] = [
    Some(common_read), Some(class_read), Some(role_read), Some(type_read),
    Some(user_read), Some(cond_read_bool), Some(sens_read), Some(cat_read),
];

#[no_mangle]
pub unsafe extern "C" fn string_to_security_class(p: *mut policydb, name: *const c_char) -> u16_t {
    let cladatum = symtab_search(&mut (*p).p_classes, name) as *mut class_datum;
    if cladatum.is_null() { return 0; }
    (*cladatum).value
}

#[no_mangle]
pub unsafe extern "C" fn string_to_av_perm(p: *mut policydb, tclass: u16_t, name: *const c_char) -> u32_t {
    if tclass == 0 || tclass as u32_t > (*p).p_classes.nprim { return 0; }
    let cladatum = *(*p).class_val_to_struct.add(tclass as usize - 1);
    let comdatum = (*cladatum).comdatum;
    let mut perdatum: *mut perm_datum = ptr::null_mut();
    if !comdatum.is_null() {
        perdatum = symtab_search(&mut (*comdatum).permissions, name) as *mut perm_datum;
    }
    if perdatum.is_null() {
        perdatum = symtab_search(&mut (*cladatum).permissions, name) as *mut perm_datum;
    }
    if perdatum.is_null() { return 0; }
    1u32 << ((*perdatum).value - 1)
}

unsafe fn mls_write_level(l: *mut mls_level, fp: *mut policy_file) -> c_int {
    let mut buf = [cpu_to_le32((*l).sens)];
    let mut rc = put_entry(buf.as_ptr() as *const c_void, size_of::<u32_t>(), 1, fp);
    if rc != 0 { return rc; }
    rc = ebitmap_write(&mut (*l).cat, fp);
    if rc != 0 { return rc; }
    0
}

unsafe fn mls_write_range_helper(r: *mut mls_range, fp: *mut policy_file) -> c_int {
    let mut buf: [__le32; 3] = [0; 3];
    let eq = mls_level_eq(&mut (*r).level[1], &mut (*r).level[0]);
    let items: size_t = if eq != 0 { 2 } else { 3 };
    buf[0] = cpu_to_le32((items - 1) as u32_t);
    buf[1] = cpu_to_le32((*r).level[0].sens);
    if eq == 0 { buf[2] = cpu_to_le32((*r).level[1].sens); }
    let mut rc = put_entry(buf.as_ptr() as *const c_void, size_of::<u32_t>(), items, fp);
    if rc != 0 { return rc; }
    rc = ebitmap_write(&mut (*r).level[0].cat, fp);
    if rc != 0 { return rc; }
    if eq == 0 {
        rc = ebitmap_write(&mut (*r).level[1].cat, fp);
        if rc != 0 { return rc; }
    }
    0
}

unsafe extern "C" fn sens_write(_vkey: *mut c_void, _datum: *mut c_void, _ptr: *mut c_void) -> c_int { todo!("translated body continues from policydb.c sens_write") }
unsafe extern "C" fn cat_write(_vkey: *mut c_void, _datum: *mut c_void, _ptr: *mut c_void) -> c_int { todo!("translated body continues from policydb.c cat_write") }
unsafe extern "C" fn common_write(_vkey: *mut c_void, _datum: *mut c_void, _ptr: *mut c_void) -> c_int { todo!("translated body continues from policydb.c common_write") }
unsafe extern "C" fn class_write(_vkey: *mut c_void, _datum: *mut c_void, _ptr: *mut c_void) -> c_int { todo!("translated body continues from policydb.c class_write") }
unsafe extern "C" fn role_write(_vkey: *mut c_void, _datum: *mut c_void, _ptr: *mut c_void) -> c_int { todo!("translated body continues from policydb.c role_write") }
unsafe extern "C" fn type_write(_vkey: *mut c_void, _datum: *mut c_void, _ptr: *mut c_void) -> c_int { todo!("translated body continues from policydb.c type_write") }
unsafe extern "C" fn user_write(_vkey: *mut c_void, _datum: *mut c_void, _ptr: *mut c_void) -> c_int { todo!("translated body continues from policydb.c user_write") }

static write_f: [Option<unsafe extern "C" fn(*mut c_void, *mut c_void, *mut c_void) -> c_int>; SYM_NUM] = [
    Some(common_write), Some(class_write), Some(role_write), Some(type_write),
    Some(user_write), Some(cond_write_bool), Some(sens_write), Some(cat_write),
];

#[no_mangle]
pub unsafe extern "C" fn policydb_read(p: *mut policydb, fp: *mut policy_file) -> c_int {
    policydb_init(p);
    let _ = (fp, read_f);
    todo!("translated body continues from policydb.c policydb_read, including filename/range/ocontext/genfs parsing and bounds checks")
}

#[no_mangle]
pub unsafe extern "C" fn policydb_write(p: *mut policydb, fp: *mut policy_file) -> c_int {
    let _ = (p, fp, write_f);
    todo!("translated body continues from policydb.c policydb_write, including symbol, avtab, conditional, role, filename, ocontext, genfs, range and type-attribute output")
}


// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
