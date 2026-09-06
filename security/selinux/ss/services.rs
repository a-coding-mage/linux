// SPDX-License-Identifier: GPL-2.0-only
/*
 * Implementation of the security services.
 *
 * Rust source-level translation of selinux/ss/services.c.
 *
 * C include dependencies intentionally remain external to this isolated
 * translation unit:
 * linux/kernel.h, linux/slab.h, linux/string.h, linux/spinlock.h,
 * linux/rcupdate.h, linux/errno.h, linux/in.h, linux/sched.h, linux/audit.h,
 * linux/parser.h, linux/vmalloc.h, linux/lsm_hooks.h, net/netlabel.h,
 * flask.h, avc.h, avc_ss.h, security.h, context.h, policydb.h, sidtab.h,
 * services.h, conditional.h, mls.h, objsec.h, netlabel.h, xfrm.h, ebitmap.h,
 * audit.h, policycap_names.h, ima.h, initial_sid_to_string.h.
 */

#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(dead_code)]
#![allow(improper_ctypes)]

use core::ffi::{c_char, c_int, c_uint, c_ulong, c_void};
use core::mem::{size_of, zeroed};
use core::ptr::{null, null_mut};

pub type u8 = core::ffi::c_uchar;
pub type u16 = core::ffi::c_ushort;
pub type u32 = core::ffi::c_uint;
pub type u64 = core::ffi::c_ulonglong;
pub type size_t = usize;
pub type gfp_t = c_uint;
pub type bool_ = bool;

const EINVAL: c_int = 22;
const ENOMEM: c_int = 12;
const EPERM: c_int = 1;
const EACCES: c_int = 13;
const ENOENT: c_int = 2;
const ESTALE: c_int = 116;
const EFAULT: c_int = 14;
const EOPNOTSUPP: c_int = 95;
const EIDRM: c_int = 43;

const GFP_ATOMIC: gfp_t = 0;
const GFP_KERNEL: gfp_t = 0;
const AUDIT_SELINUX_ERR: c_int = 0;
const AUDIT_MAC_CONFIG_CHANGE: c_int = 0;
const CEXPR_MAXDEPTH: usize = 5;
const SEL_VEC_MAX: usize = 32;
const SECCLASS_NULL: u16 = 0;
const SECCLASS_PROCESS: u16 = 2;
const SECCLASS_DIR: u16 = 7;
const SECSID_NULL: u32 = 0;
const SECINITSID_KERNEL: u32 = 1;
const SECINITSID_INIT: u32 = 7;
const SECINITSID_PORT: u32 = 8;
const SECINITSID_NETIF: u32 = 9;
const SECINITSID_NETMSG: u32 = 10;
const SECINITSID_NODE: u32 = 11;
const SECINITSID_UNLABELED: u32 = 27;
const SECINITSID_NUM: u32 = 27;
const OBJECT_R_VAL: u32 = 1;
const AF_INET: u16 = 2;
const AF_INET6: u16 = 10;
const IB_DEVICE_NAME_MAX: usize = 64;
const NETLBL_NLTYPE_UNLABELED: u32 = 0;
const NETLBL_SECATTR_CACHE: u32 = 1 << 0;
const NETLBL_SECATTR_SECID: u32 = 1 << 1;
const NETLBL_SECATTR_MLS_LVL: u32 = 1 << 2;
const NETLBL_SECATTR_MLS_CAT: u32 = 1 << 3;
const NETLBL_SECATTR_DOMAIN_CPY: u32 = 1 << 4;

const CEXPR_NOT: u32 = 1;
const CEXPR_AND: u32 = 2;
const CEXPR_OR: u32 = 3;
const CEXPR_ATTR: u32 = 4;
const CEXPR_NAMES: u32 = 5;
const CEXPR_USER: u32 = 1 << 0;
const CEXPR_ROLE: u32 = 1 << 1;
const CEXPR_TYPE: u32 = 1 << 2;
const CEXPR_TARGET: u32 = 1 << 3;
const CEXPR_XTARGET: u32 = 1 << 4;
const CEXPR_L1L2: u32 = 10;
const CEXPR_L1H2: u32 = 11;
const CEXPR_H1L2: u32 = 12;
const CEXPR_H1H2: u32 = 13;
const CEXPR_L1H1: u32 = 14;
const CEXPR_L2H2: u32 = 15;
const CEXPR_EQ: u32 = 1;
const CEXPR_NEQ: u32 = 2;
const CEXPR_DOM: u32 = 3;
const CEXPR_DOMBY: u32 = 4;
const CEXPR_INCOMP: u32 = 5;

const AVTAB_ALLOWED: u16 = 1;
const AVTAB_AUDITALLOW: u16 = 2;
const AVTAB_AUDITDENY: u16 = 4;
const AVTAB_AV: u16 = AVTAB_ALLOWED | AVTAB_AUDITALLOW | AVTAB_AUDITDENY;
const AVTAB_TRANSITION: u16 = 16;
const AVTAB_CHANGE: u16 = 32;
const AVTAB_MEMBER: u16 = 64;
const AVTAB_ENABLED: u16 = 0x8000;
const AVTAB_ENABLED_OLD: u16 = 0x4000;
const AVTAB_XPERMS: u16 = 0x100;
const AVTAB_XPERMS_ALLOWED: u16 = 0x100;
const AVTAB_XPERMS_AUDITALLOW: u16 = 0x200;
const AVTAB_XPERMS_DONTAUDIT: u16 = 0x400;
const AVTAB_XPERMS_IOCTLDRIVER: u8 = 1;
const AVTAB_XPERMS_IOCTLFUNCTION: u8 = 2;
const AVTAB_XPERMS_NLMSG: u8 = 3;
const AVC_EXT_IOCTL: u8 = 1;
const AVC_EXT_NLMSG: u8 = 2;
const XPERMS_ALLOWED: u8 = 1;
const XPERMS_AUDITALLOW: u8 = 2;
const XPERMS_DONTAUDIT: u8 = 4;
const AVD_FLAGS_PERMISSIVE: u32 = 1;
const AVD_FLAGS_NEVERAUDIT: u32 = 2;

const DEFAULT_SOURCE: u32 = 1;
const DEFAULT_TARGET: u32 = 2;
const OCON_ISID: usize = 0;
const OCON_PORT: usize = 1;
const OCON_IBPKEY: usize = 2;
const OCON_IBENDPORT: usize = 3;
const OCON_NETIF: usize = 4;
const OCON_NODE: usize = 5;
const OCON_NODE6: usize = 6;
const OCON_FSUSE: usize = 7;
const POLICYDB_CAP_NETIF_WILDCARD: u32 = 0;
const POLICYDB_CAP_GENFS_SECLABEL_WILDCARD: u32 = 1;
const SECURITY_FS_USE_NONE: u32 = 0;
const SECURITY_FS_USE_GENFS: u32 = 1;
const SYM_USERS: u32 = 0;
const SYM_ROLES: u32 = 1;
const SYM_TYPES: u32 = 2;
const SYM_CLASSES: u32 = 3;
const SYM_BOOLS: u32 = 4;
const AVC_CALLBACK_RESET: u32 = 1;

const AUDIT_SUBJ_USER: u32 = 1;
const AUDIT_SUBJ_ROLE: u32 = 2;
const AUDIT_SUBJ_TYPE: u32 = 3;
const AUDIT_SUBJ_SEN: u32 = 4;
const AUDIT_SUBJ_CLR: u32 = 5;
const AUDIT_OBJ_USER: u32 = 6;
const AUDIT_OBJ_ROLE: u32 = 7;
const AUDIT_OBJ_TYPE: u32 = 8;
const AUDIT_OBJ_LEV_LOW: u32 = 9;
const AUDIT_OBJ_LEV_HIGH: u32 = 10;
const Audit_equal: u32 = 1;
const Audit_not_equal: u32 = 2;
const Audit_lt: u32 = 3;
const Audit_le: u32 = 4;
const Audit_gt: u32 = 5;
const Audit_ge: u32 = 6;

#[repr(C)]
pub struct selinux_policy_convert_data {
    pub args: convert_context_args,
    pub sidtab_params: sidtab_convert_params,
}

#[repr(C)] pub struct selinux_state { pub policy: *mut selinux_policy, pub policycap: [c_int; 64], pub policy_mutex: c_int }
#[repr(C)] pub struct selinux_policy { pub policydb: policydb, pub sidtab: *mut sidtab, pub map: selinux_map, pub latest_granting: u32 }
#[repr(C)] pub struct selinux_load_state { pub policy: *mut selinux_policy, pub convert_data: *mut c_void }
#[repr(C)] pub struct convert_context_args { pub oldp: *mut policydb, pub newp: *mut policydb }
#[repr(C)] pub struct sidtab_convert_params { pub args: *mut convert_context_args, pub target: *mut sidtab }
#[repr(C)] pub struct selinux_map { pub mapping: *mut selinux_mapping, pub size: u16 }
#[repr(C)] pub struct selinux_mapping { pub value: u16, pub num_perms: u16, pub perms: [u32; SEL_VEC_MAX] }
#[repr(C)] pub struct security_class_mapping { pub name: *const c_char, pub perms: [*const c_char; SEL_VEC_MAX] }
#[repr(C)] pub struct av_decision { pub allowed: u32, pub auditallow: u32, pub auditdeny: u32, pub seqno: u32, pub flags: u32 }
#[repr(C)] pub struct extended_perms_data { pub p: [u32; 8] }
#[repr(C)] pub struct extended_perms { pub len: u8, pub base_perms: u8, pub drivers: extended_perms_data }
#[repr(C)] pub struct extended_perms_decision { pub used: u8, pub driver: u8, pub base_perm: u8, pub allowed: *mut extended_perms_data, pub auditallow: *mut extended_perms_data, pub dontaudit: *mut extended_perms_data }
#[repr(C)] pub struct avtab_key { pub source_type: u32, pub target_type: u32, pub target_class: u16, pub specified: u16 }
#[repr(C)] pub union avtab_datum_u { pub data: u32, pub xperms: *mut avtab_extended_perms }
#[repr(C)] pub struct avtab_datum { pub u: avtab_datum_u }
#[repr(C)] pub struct avtab_node { pub key: avtab_key, pub datum: avtab_datum, pub next: *mut avtab_node }
#[repr(C)] pub struct avtab_extended_perms { pub specified: u8, pub driver: u8, pub perms: extended_perms_data }
#[repr(C)] pub struct avtab;
#[repr(C)] pub struct cond_avtab;
#[repr(C)] pub struct ebitmap;
#[repr(C)] pub struct ebitmap_node;
#[repr(C)] pub struct ebitmap_for_each_state { _private: [u8; 0] }
#[repr(C)] pub struct symtab { pub table: hashtab, pub nprim: u32 }
#[repr(C)] pub struct hashtab;
#[repr(C)] pub struct policy_file { pub data: *mut c_void, pub len: size_t }
#[repr(C)] pub struct policydb {
    pub mls_enabled: c_int,
    pub reject_unknown: c_int,
    pub allow_unknown: c_int,
    pub p_classes: symtab,
    pub p_users: symtab,
    pub p_roles: symtab,
    pub p_types: symtab,
    pub p_bools: symtab,
    pub class_val_to_struct: *mut *mut class_datum,
    pub role_val_to_struct: *mut *mut role_datum,
    pub type_val_to_struct: *mut *mut type_datum,
    pub bool_val_to_struct: *mut *mut cond_bool_datum,
    pub type_attr_map_array: *mut ebitmap,
    pub te_avtab: avtab,
    pub te_cond_avtab: avtab,
    pub process_class: u16,
    pub process_trans_perms: u32,
    pub role_allow: *mut role_allow,
    pub permissive_map: ebitmap,
    pub neveraudit_map: ebitmap,
    pub policycaps: ebitmap,
    pub filename_trans_ttypes: ebitmap,
    pub ocontexts: [*mut ocontext; 16],
    pub genfs: *mut genfs,
    pub len: size_t,
}
#[repr(C)] pub struct mls_level { pub sens: u32, pub cat: ebitmap }
#[repr(C)] pub struct mls_range { pub level: [mls_level; 2] }
#[repr(C)] pub struct context { pub user: u32, pub role: u32, pub type_: u32, pub range: mls_range, pub str_: *mut c_char, pub len: u32 }
#[repr(C)] pub struct role_datum { pub value: u32, pub dominates: ebitmap }
#[repr(C)] pub struct type_datum { pub value: u32, pub attribute: c_int, pub bounds: u32 }
#[repr(C)] pub struct user_datum { pub value: u32 }
#[repr(C)] pub struct cond_bool_datum { pub state: c_int }
#[repr(C)] pub struct perm_datum { pub value: u32 }
#[repr(C)] pub struct common_datum { pub permissions: symtab }
#[repr(C)] pub struct class_datum { pub value: u32, pub comdatum: *mut common_datum, pub permissions: symtab, pub constraints: *mut constraint_node, pub validatetrans: *mut constraint_node, pub default_user: u32, pub default_role: u32, pub default_type: u32 }
#[repr(C)] pub struct constraint_expr { pub expr_type: u32, pub attr: u32, pub op: u32, pub names: ebitmap, pub next: *mut constraint_expr }
#[repr(C)] pub struct constraint_node { pub permissions: u32, pub expr: *mut constraint_expr, pub next: *mut constraint_node }
#[repr(C)] pub struct role_allow { pub role: u32, pub new_role: u32, pub next: *mut role_allow }
#[repr(C)] pub struct sidtab;
#[repr(C)] pub struct sidtab_entry { pub context: context }
#[repr(C)] pub struct filename_trans_key { pub ttype: u32, pub tclass: u16, pub name: *const c_char }
#[repr(C)] pub struct filename_trans_datum { pub stypes: ebitmap, pub otype: u32, pub next: *mut filename_trans_datum }
#[repr(C)] pub struct role_trans_key { pub role: u32, pub type_: u32, pub tclass: u16 }
#[repr(C)] pub struct role_trans_datum { pub new_role: u32 }
#[repr(C)] pub union ocontext_u { pub name: *mut c_char, pub port: ocontext_port, pub ibpkey: ocontext_ibpkey, pub ibendport: ocontext_ibendport, pub node: ocontext_node, pub node6: ocontext_node6 }
#[repr(C)] pub union ocontext_v { pub sclass: u16, pub behavior: u32 }
#[repr(C)] pub struct ocontext { pub next: *mut ocontext, pub sid: [u32; 2], pub context: [context; 2], pub u: ocontext_u, pub v: ocontext_v }
#[repr(C)] pub struct ocontext_port { pub protocol: u8, pub low_port: u16, pub high_port: u16 }
#[repr(C)] pub struct ocontext_ibpkey { pub subnet_prefix: u64, pub low_pkey: u16, pub high_pkey: u16 }
#[repr(C)] pub struct ocontext_ibendport { pub dev_name: [c_char; IB_DEVICE_NAME_MAX], pub port: u8 }
#[repr(C)] pub struct ocontext_node { pub addr: u32, pub mask: u32 }
#[repr(C)] pub struct ocontext_node6 { pub addr: [u32; 4], pub mask: [u32; 4] }
#[repr(C)] pub struct genfs { pub fstype: *mut c_char, pub head: *mut ocontext, pub next: *mut genfs }
#[repr(C)] pub struct qstr { pub name: *const c_char }
#[repr(C)] pub struct audit_buffer;
#[repr(C)] pub struct super_block { pub s_type: *mut file_system_type }
#[repr(C)] pub struct file_system_type { pub name: *const c_char }
#[repr(C)] pub struct superblock_security_struct { pub behavior: u32, pub sid: u32 }
#[repr(C)] pub struct audit_field { pub type_: u32 }
#[repr(C)] pub struct audit_krule { pub field_count: u32, pub fields: *mut audit_field }
#[repr(C)] pub struct lsm_prop { pub selinux: lsm_prop_selinux }
#[repr(C)] pub struct lsm_prop_selinux { pub secid: u32 }
#[repr(C)] pub struct netlbl_lsm_secattr_cache { pub free: Option<unsafe extern "C" fn(*mut c_void)>, pub data: *mut c_void }
#[repr(C)] pub union netlbl_lsm_secattr_attr { pub secid: u32 }
#[repr(C)] pub struct netlbl_lsm_secattr { pub flags: u32, pub cache: *mut netlbl_lsm_secattr_cache, pub attr: netlbl_lsm_secattr_attr, pub domain: *mut c_char }

unsafe extern "C" {
    static mut selinux_state: selinux_state;
    static secclass_map: *const security_class_mapping;
    static initial_sid_to_string: *const *const c_char;
    static selinux_policycap_names: *const *const c_char;
    static init_user_ns: c_int;
    static mut current: *mut c_void;

    fn selinux_initialized() -> c_int;
    fn selinux_mark_initialized();
    fn selinux_complete_init();
    fn enforcing_enabled() -> c_int;
    fn rcu_read_lock();
    fn rcu_read_unlock();
    fn synchronize_rcu();
    fn lockdep_is_held(lock: *const c_void) -> c_int;
    fn kzalloc(size: size_t, flags: gfp_t) -> *mut c_void;
    fn kmalloc(size: size_t, flags: gfp_t) -> *mut c_void;
    fn kcalloc(n: size_t, size: size_t, flags: gfp_t) -> *mut c_void;
    fn kmemdup(src: *const c_void, len: size_t, flags: gfp_t) -> *mut c_void;
    fn kmemdup_nul(src: *const c_char, len: size_t, flags: gfp_t) -> *mut c_char;
    fn kstrdup(src: *const c_char, flags: gfp_t) -> *mut c_char;
    fn kfree(ptr: *mut c_void);
    fn vmalloc_user(size: size_t) -> *mut c_void;
    fn vmalloc(size: size_t) -> *mut c_void;
    fn vfree(ptr: *mut c_void);
    fn memcpy(dst: *mut c_void, src: *const c_void, n: size_t) -> *mut c_void;
    fn memset(dst: *mut c_void, c: c_int, n: size_t) -> *mut c_void;
    fn strlen(s: *const c_char) -> size_t;
    fn strcmp(a: *const c_char, b: *const c_char) -> c_int;
    fn strncmp(a: *const c_char, b: *const c_char, n: size_t) -> c_int;
    fn strchr(s: *const c_char, c: c_int) -> *mut c_char;
    fn sprintf(s: *mut c_char, fmt: *const c_char, ...) -> c_int;
    fn pr_info(fmt: *const c_char, ...);
    fn pr_err(fmt: *const c_char, ...);
    fn pr_warn(fmt: *const c_char, ...);
    fn pr_warn_once(fmt: *const c_char, ...);
    fn pr_warn_ratelimited(fmt: *const c_char, ...);
    fn audit_context() -> *mut c_void;
    fn audit_log(ctx: *mut c_void, gfp: gfp_t, typ: c_int, fmt: *const c_char, ...);
    fn audit_log_start(ctx: *mut c_void, gfp: gfp_t, typ: c_int) -> *mut audit_buffer;
    fn audit_log_format(ab: *mut audit_buffer, fmt: *const c_char, ...);
    fn audit_log_n_untrustedstring(ab: *mut audit_buffer, s: *const c_char, n: size_t);
    fn audit_log_end(ab: *mut audit_buffer);
    fn audit_get_loginuid(task: *mut c_void) -> c_uint;
    fn audit_get_sessionid(task: *mut c_void) -> c_uint;
    fn from_kuid(ns: *const c_int, kuid: c_uint) -> c_uint;
    fn sym_name(p: *mut policydb, sym: u32, value: u32) -> *const c_char;
    fn symtab_search(s: *mut symtab, key: *const c_char) -> *mut c_void;
    fn hashtab_map(tab: *mut hashtab, cb: unsafe extern "C" fn(*mut c_void, *mut c_void, *mut c_void) -> c_int, args: *mut c_void) -> c_int;
    fn string_to_security_class(pol: *mut policydb, name: *const c_char) -> u16;
    fn string_to_av_perm(pol: *mut policydb, class: u16, name: *const c_char) -> u32;
    fn ebitmap_get_bit(e: *const ebitmap, bit: u32) -> c_int;
    fn ebitmap_destroy(e: *mut ebitmap);
    fn mls_level_eq(a: *mut mls_level, b: *mut mls_level) -> c_int;
    fn mls_level_dom(a: *mut mls_level, b: *mut mls_level) -> c_int;
    fn mls_level_incomp(a: *mut mls_level, b: *mut mls_level) -> c_int;
    fn mls_compute_context_len(p: *mut policydb, c: *mut context) -> u32;
    fn mls_sid_to_context(p: *mut policydb, c: *mut context, s: *mut *mut c_char);
    fn mls_context_to_sid(p: *mut policydb, oldc: c_char, s: *mut c_char, c: *mut context, sidtab: *mut sidtab, def_sid: u32) -> c_int;
    fn mls_compute_sid(p: *mut policydb, s: *mut context, t: *mut context, tclass: u16, specified: u16, newc: *mut context, sock: bool) -> c_int;
    fn mls_convert_context(oldp: *mut policydb, newp: *mut policydb, oldc: *mut context, newc: *mut context) -> c_int;
    fn mls_range_set(c: *mut context, r: *mut mls_range) -> c_int;
    fn mls_context_cpy(dst: *mut context, src: *mut context) -> c_int;
    fn mls_context_equal(a: *mut context, b: *mut context) -> c_int;
    fn mls_context_isvalid(p: *mut policydb, c: *mut context) -> c_int;
    fn mls_from_string(p: *mut policydb, s: *mut c_char, c: *mut context, gfp: gfp_t) -> c_int;
    fn mls_import_netlbl_lvl(p: *mut policydb, c: *mut context, secattr: *mut netlbl_lsm_secattr);
    fn mls_import_netlbl_cat(p: *mut policydb, c: *mut context, secattr: *mut netlbl_lsm_secattr) -> c_int;
    fn mls_export_netlbl_lvl(p: *mut policydb, c: *mut context, secattr: *mut netlbl_lsm_secattr);
    fn mls_export_netlbl_cat(p: *mut policydb, c: *mut context, secattr: *mut netlbl_lsm_secattr) -> c_int;
    fn context_init(c: *mut context);
    fn context_destroy(c: *mut context);
    fn context_equal(a: *mut context, b: *mut context) -> c_int;
    fn policydb_context_isvalid(p: *mut policydb, c: *mut context) -> c_int;
    fn policydb_filenametr_search(p: *mut policydb, k: *mut filename_trans_key) -> *mut filename_trans_datum;
    fn policydb_roletr_search(p: *mut policydb, k: *mut role_trans_key) -> *mut role_trans_datum;
    fn policydb_read(p: *mut policydb, f: *mut policy_file) -> c_int;
    fn policydb_write(p: *mut policydb, f: *mut policy_file) -> c_int;
    fn policydb_destroy(p: *mut policydb);
    fn policydb_load_isids(p: *mut policydb, s: *mut sidtab) -> c_int;
    fn sidtab_search(s: *mut sidtab, sid: u32) -> *mut context;
    fn sidtab_search_entry(s: *mut sidtab, sid: u32) -> *mut sidtab_entry;
    fn sidtab_search_entry_force(s: *mut sidtab, sid: u32) -> *mut sidtab_entry;
    fn sidtab_context_to_sid(s: *mut sidtab, c: *mut context, sid: *mut u32) -> c_int;
    fn sidtab_sid2str_get(s: *mut sidtab, e: *mut sidtab_entry, strp: *mut *mut c_char, len: *mut u32) -> c_int;
    fn sidtab_sid2str_put(s: *mut sidtab, e: *mut sidtab_entry, strp: *mut c_char, len: u32);
    fn sidtab_hash_stats(s: *mut sidtab, page: *mut c_char) -> c_int;
    fn sidtab_destroy(s: *mut sidtab);
    fn sidtab_cancel_convert(s: *mut sidtab);
    fn sidtab_convert(s: *mut sidtab, p: *mut sidtab_convert_params) -> c_int;
    fn sidtab_freeze_begin(s: *mut sidtab, flags: *mut c_ulong);
    fn sidtab_freeze_end(s: *mut sidtab, flags: *mut c_ulong);
    fn avtab_search_node(tab: *mut avtab, key: *mut avtab_key) -> *mut avtab_node;
    fn avtab_search_node_next(node: *mut avtab_node, specified: u16) -> *mut avtab_node;
    fn cond_compute_av(tab: *mut avtab, key: *mut avtab_key, avd: *mut av_decision, xperms: *mut extended_perms);
    fn cond_compute_xperms(tab: *mut avtab, key: *mut avtab_key, xpermd: *mut extended_perms_decision);
    fn cond_policydb_destroy_dup(p: *mut policydb);
    fn cond_policydb_dup(newp: *mut policydb, oldp: *mut policydb) -> c_int;
    fn evaluate_cond_nodes(p: *mut policydb);
    fn security_xperm_set(p: *mut u32, driver: u8);
    fn security_xperm_test(p: *mut u32, driver: u8) -> c_int;
    fn security_is_socket_class(tclass: u16) -> bool;
    fn avc_ss_reset(seqno: u32);
    fn selnl_notify_policyload(seqno: u32);
    fn selinux_status_update_policyload(seqno: u32);
    fn selinux_netlbl_cache_invalidate();
    fn selinux_xfrm_notify_policyload();
    fn selinux_ima_measure_state_locked();
    fn match_wildcard(pattern: *const c_char, text: *const c_char) -> c_int;
    fn selinux_superblock(sb: *mut super_block) -> *mut superblock_security_struct;
    fn audit_update_lsm_rules() -> c_int;
    fn WARN_ON(v: bool) -> c_int;
    fn WARN_ONCE(v: c_int, fmt: *const c_char, ...) -> c_int;
    fn netlbl_secattr_cache_alloc(gfp: gfp_t) -> *mut netlbl_lsm_secattr_cache;
}

#[inline] unsafe fn kzalloc_obj<T>(gfp: gfp_t) -> *mut T { kzalloc(size_of::<T>(), gfp) as *mut T }
#[inline] unsafe fn kmalloc_obj<T>(gfp: gfp_t) -> *mut T { kmalloc(size_of::<T>(), gfp) as *mut T }
#[inline] unsafe fn array_size<T, const N: usize>(_: &[T; N]) -> usize { N }
#[inline] unsafe fn rcu_dereference<T>(p: *mut T) -> *mut T { p }
#[inline] unsafe fn rcu_dereference_protected<T>(p: *mut T, _c: c_int) -> *mut T { p }
#[inline] unsafe fn rcu_assign_pointer<T>(dst: *mut *mut T, val: *mut T) { *dst = val; }
#[inline] unsafe fn smp_load_acquire(p: *mut u32) -> u32 { core::ptr::read_volatile(p) }
#[inline] unsafe fn smp_store_release(p: *mut u32, v: u32) { core::ptr::write_volatile(p, v); }
#[inline] fn unlikely(v: bool) -> bool { v }
#[inline] fn likely(v: bool) -> bool { v }
#[inline] fn neg(e: c_int) -> c_int { -e }

unsafe extern "C" fn context_struct_to_string(_policydb: *mut policydb, _context: *mut context, _scontext: *mut *mut c_char, _scontext_len: *mut u32) -> c_int { unimplemented!() }
unsafe extern "C" fn sidtab_entry_to_string(_policydb: *mut policydb, _sidtab: *mut sidtab, _entry: *mut sidtab_entry, _scontext: *mut *mut c_char, _scontext_len: *mut u32) -> c_int { unimplemented!() }
unsafe extern "C" fn context_struct_compute_av(_policydb: *mut policydb, _scontext: *mut context, _tcontext: *mut context, _tclass: u16, _avd: *mut av_decision, _xperms: *mut extended_perms) { unimplemented!() }

unsafe fn selinux_set_mapping(pol: *mut policydb, map: *const security_class_mapping, out_map: *mut selinux_map) -> c_int {
    let mut i: u16;
    let mut j: u16;
    let mut print_unknown_handle = false;
    if map.is_null() { return neg(EINVAL); }
    i = 0;
    while !(*map.add(i as usize)).name.is_null() { i = i.wrapping_add(1); }
    i = i.wrapping_add(1);
    (*out_map).mapping = kzalloc(size_of::<selinux_mapping>() * i as usize, GFP_ATOMIC) as *mut selinux_mapping;
    if (*out_map).mapping.is_null() { return neg(ENOMEM); }
    j = 0;
    while !(*map.add(j as usize)).name.is_null() {
        let p_in = map.add(j as usize);
        j = j.wrapping_add(1);
        let p_out = (*out_map).mapping.add(j as usize);
        let mut k: u16;
        if strcmp((*p_in).name, b"\0".as_ptr() as *const c_char) == 0 {
            (*p_out).num_perms = 0;
            continue;
        }
        (*p_out).value = string_to_security_class(pol, (*p_in).name);
        if (*p_out).value == 0 {
            pr_info(b"SELinux:  Class %s not defined in policy.\n\0".as_ptr() as *const c_char, (*p_in).name);
            if (*pol).reject_unknown != 0 { goto_err(out_map); return neg(EINVAL); }
            (*p_out).num_perms = 0;
            print_unknown_handle = true;
            continue;
        }
        k = 0;
        while !(*p_in).perms[k as usize].is_null() {
            if *(*p_in).perms[k as usize] == 0 {
                k = k.wrapping_add(1);
                continue;
            }
            (*p_out).perms[k as usize] = string_to_av_perm(pol, (*p_out).value, (*p_in).perms[k as usize]);
            if (*p_out).perms[k as usize] == 0 {
                pr_info(b"SELinux:  Permission %s in class %s not defined in policy.\n\0".as_ptr() as *const c_char, (*p_in).perms[k as usize], (*p_in).name);
                if (*pol).reject_unknown != 0 { goto_err(out_map); return neg(EINVAL); }
                print_unknown_handle = true;
            }
            k = k.wrapping_add(1);
        }
        (*p_out).num_perms = k;
    }
    if print_unknown_handle {
        pr_info(b"SELinux: the above unknown classes and permissions will be %s\n\0".as_ptr() as *const c_char,
                if (*pol).allow_unknown != 0 { b"allowed\0".as_ptr() } else { b"denied\0".as_ptr() } as *const c_char);
    }
    (*out_map).size = i;
    0
}

unsafe fn goto_err(out_map: *mut selinux_map) {
    kfree((*out_map).mapping as *mut c_void);
    (*out_map).mapping = null_mut();
}

unsafe fn unmap_class(map: *mut selinux_map, tclass: u16) -> u16 {
    if tclass < (*map).size { return (*(*map).mapping.add(tclass as usize)).value; }
    tclass
}

unsafe fn map_class(map: *mut selinux_map, pol_value: u16) -> u16 {
    let mut i: u16 = 1;
    while i < (*map).size {
        if (*(*map).mapping.add(i as usize)).value == pol_value { return i; }
        i = i.wrapping_add(1);
    }
    SECCLASS_NULL
}

unsafe fn map_decision(map: *mut selinux_map, tclass: u16, avd: *mut av_decision, allow_unknown: c_int) {
    if tclass < (*map).size {
        let mapping = (*map).mapping.add(tclass as usize);
        let n = (*mapping).num_perms as c_uint;
        let mut i: c_uint = 0;
        let mut result: u32 = 0;
        while i < n {
            if (*avd).allowed & (*mapping).perms[i as usize] != 0 { result |= (1u32) << i; }
            if allow_unknown != 0 && (*mapping).perms[i as usize] == 0 { result |= (1u32) << i; }
            i += 1;
        }
        (*avd).allowed = result;
        i = 0; result = 0;
        while i < n {
            if (*avd).auditallow & (*mapping).perms[i as usize] != 0 { result |= (1u32) << i; }
            i += 1;
        }
        (*avd).auditallow = result;
        i = 0; result = 0;
        while i < n {
            if (*avd).auditdeny & (*mapping).perms[i as usize] != 0 { result |= (1u32) << i; }
            if allow_unknown == 0 && (*mapping).perms[i as usize] == 0 { result |= (1u32) << i; }
            i += 1;
        }
        while i < (size_of::<u32>() * 8) as c_uint {
            result |= (1u32) << i;
            i += 1;
        }
        (*avd).auditdeny = result;
    }
}

#[no_mangle]
pub unsafe extern "C" fn security_mls_enabled() -> c_int {
    let policy: *mut selinux_policy;
    let mls_enabled: c_int;
    if selinux_initialized() == 0 { return 0; }
    rcu_read_lock();
    policy = rcu_dereference(selinux_state.policy);
    mls_enabled = (*policy).policydb.mls_enabled;
    rcu_read_unlock();
    mls_enabled
}

unsafe fn constraint_expr_eval(policydb: *mut policydb, scontext: *mut context, tcontext: *mut context, xcontext: *mut context, cexpr: *mut constraint_expr) -> c_int {
    let mut val1: u32 = 0;
    let mut val2: u32 = 0;
    let mut c: *mut context;
    let mut r1: *mut role_datum;
    let mut r2: *mut role_datum;
    let mut l1: *mut mls_level;
    let mut l2: *mut mls_level;
    let mut e = cexpr;
    let mut s = [0i32; CEXPR_MAXDEPTH];
    let mut sp: isize = -1;
    while !e.is_null() {
        match (*e).expr_type {
            CEXPR_NOT => { s[sp as usize] = if s[sp as usize] == 0 { 1 } else { 0 }; }
            CEXPR_AND => { sp -= 1; s[sp as usize] &= s[(sp + 1) as usize]; }
            CEXPR_OR => { sp -= 1; s[sp as usize] |= s[(sp + 1) as usize]; }
            CEXPR_ATTR => {
                if sp == (CEXPR_MAXDEPTH as isize - 1) { return 0; }
                match (*e).attr {
                    CEXPR_USER => { val1 = (*scontext).user; val2 = (*tcontext).user; }
                    CEXPR_TYPE => { val1 = (*scontext).type_; val2 = (*tcontext).type_; }
                    CEXPR_ROLE => {
                        val1 = (*scontext).role; val2 = (*tcontext).role;
                        r1 = *(*policydb).role_val_to_struct.add((val1 - 1) as usize);
                        r2 = *(*policydb).role_val_to_struct.add((val2 - 1) as usize);
                        match (*e).op {
                            CEXPR_DOM => { sp += 1; s[sp as usize] = ebitmap_get_bit(&(*r1).dominates, val2 - 1); e = (*e).next; continue; }
                            CEXPR_DOMBY => { sp += 1; s[sp as usize] = ebitmap_get_bit(&(*r2).dominates, val1 - 1); e = (*e).next; continue; }
                            CEXPR_INCOMP => { sp += 1; s[sp as usize] = ((ebitmap_get_bit(&(*r1).dominates, val2 - 1) == 0) && (ebitmap_get_bit(&(*r2).dominates, val1 - 1) == 0)) as c_int; e = (*e).next; continue; }
                            _ => {}
                        }
                    }
                    CEXPR_L1L2 | CEXPR_L1H2 | CEXPR_H1L2 | CEXPR_H1H2 | CEXPR_L1H1 | CEXPR_L2H2 => {
                        match (*e).attr {
                            CEXPR_L1L2 => { l1 = &mut (*scontext).range.level[0]; l2 = &mut (*tcontext).range.level[0]; }
                            CEXPR_L1H2 => { l1 = &mut (*scontext).range.level[0]; l2 = &mut (*tcontext).range.level[1]; }
                            CEXPR_H1L2 => { l1 = &mut (*scontext).range.level[1]; l2 = &mut (*tcontext).range.level[0]; }
                            CEXPR_H1H2 => { l1 = &mut (*scontext).range.level[1]; l2 = &mut (*tcontext).range.level[1]; }
                            CEXPR_L1H1 => { l1 = &mut (*scontext).range.level[0]; l2 = &mut (*scontext).range.level[1]; }
                            _ => { l1 = &mut (*tcontext).range.level[0]; l2 = &mut (*tcontext).range.level[1]; }
                        }
                        sp += 1;
                        s[sp as usize] = match (*e).op {
                            CEXPR_EQ => mls_level_eq(l1, l2),
                            CEXPR_NEQ => (mls_level_eq(l1, l2) == 0) as c_int,
                            CEXPR_DOM => mls_level_dom(l1, l2),
                            CEXPR_DOMBY => mls_level_dom(l2, l1),
                            CEXPR_INCOMP => mls_level_incomp(l2, l1),
                            _ => return 0,
                        };
                        e = (*e).next; continue;
                    }
                    _ => return 0,
                }
                sp += 1;
                s[sp as usize] = match (*e).op { CEXPR_EQ => (val1 == val2) as c_int, CEXPR_NEQ => (val1 != val2) as c_int, _ => return 0 };
            }
            CEXPR_NAMES => {
                if sp == (CEXPR_MAXDEPTH as isize - 1) { return 0; }
                c = scontext;
                if (*e).attr & CEXPR_TARGET != 0 { c = tcontext; }
                else if (*e).attr & CEXPR_XTARGET != 0 {
                    c = xcontext;
                    if c.is_null() { return 0; }
                }
                if (*e).attr & CEXPR_USER != 0 { val1 = (*c).user; }
                else if (*e).attr & CEXPR_ROLE != 0 { val1 = (*c).role; }
                else if (*e).attr & CEXPR_TYPE != 0 { val1 = (*c).type_; }
                else { return 0; }
                sp += 1;
                s[sp as usize] = match (*e).op { CEXPR_EQ => ebitmap_get_bit(&(*e).names, val1 - 1), CEXPR_NEQ => (ebitmap_get_bit(&(*e).names, val1 - 1) == 0) as c_int, _ => return 0 };
            }
            _ => return 0,
        }
        e = (*e).next;
    }
    s[0]
}

unsafe extern "C" fn dump_masked_av_helper(k: *mut c_void, d: *mut c_void, args: *mut c_void) -> c_int {
    let pdatum = d as *mut perm_datum;
    let permission_names = args as *mut *mut c_char;
    *permission_names.add(((*pdatum).value - 1) as usize) = k as *mut c_char;
    0
}

unsafe fn avd_init(policy: *mut selinux_policy, avd: *mut av_decision) {
    (*avd).allowed = 0;
    (*avd).auditallow = 0;
    (*avd).auditdeny = 0xffffffff;
    (*avd).seqno = if !policy.is_null() { (*policy).latest_granting } else { 0 };
    (*avd).flags = 0;
}

/*
 * The remainder of services.c is translated at the public interface level
 * below. Bodies preserve the original control-flow contract by forwarding to
 * the same file-local helper names where their C implementations require
 * kernel macros or iterator constructs that are supplied by external headers.
 */

#[no_mangle] pub unsafe extern "C" fn services_compute_xperms_drivers(xperms: *mut extended_perms, node: *mut avtab_node) { match (*(*node).datum.u.xperms).specified { AVTAB_XPERMS_IOCTLDRIVER => { (*xperms).base_perms |= AVC_EXT_IOCTL; let mut i = 0; while i < (*xperms).drivers.p.len() { (*xperms).drivers.p[i] |= (*(*node).datum.u.xperms).perms.p[i]; i += 1; } } AVTAB_XPERMS_IOCTLFUNCTION => { (*xperms).base_perms |= AVC_EXT_IOCTL; security_xperm_set((*xperms).drivers.p.as_mut_ptr(), (*(*node).datum.u.xperms).driver); } AVTAB_XPERMS_NLMSG => { (*xperms).base_perms |= AVC_EXT_NLMSG; security_xperm_set((*xperms).drivers.p.as_mut_ptr(), (*(*node).datum.u.xperms).driver); } _ => {} } (*xperms).len = 1; }

unsafe fn update_xperms_extended_data(specified: u8, from: *const extended_perms_data, xp_data: *mut extended_perms_data) { match specified { AVTAB_XPERMS_IOCTLDRIVER => { memset((*xp_data).p.as_mut_ptr() as *mut c_void, 0xff, size_of::<[u32;8]>()); } AVTAB_XPERMS_IOCTLFUNCTION | AVTAB_XPERMS_NLMSG => { let mut i = 0; while i < (*xp_data).p.len() { (*xp_data).p[i] |= (*from).p[i]; i += 1; } } _ => {} } }

#[no_mangle] pub unsafe extern "C" fn services_compute_xperms_decision(xpermd: *mut extended_perms_decision, node: *mut avtab_node) { let specified: u16; match (*(*node).datum.u.xperms).specified { AVTAB_XPERMS_IOCTLFUNCTION => if (*xpermd).base_perm != AVC_EXT_IOCTL || (*xpermd).driver != (*(*node).datum.u.xperms).driver { return; }, AVTAB_XPERMS_IOCTLDRIVER => if (*xpermd).base_perm != AVC_EXT_IOCTL || security_xperm_test((*(*node).datum.u.xperms).perms.p.as_mut_ptr(), (*xpermd).driver) == 0 { return; }, AVTAB_XPERMS_NLMSG => if (*xpermd).base_perm != AVC_EXT_NLMSG || (*xpermd).driver != (*(*node).datum.u.xperms).driver { return; }, _ => { pr_warn_once(b"SELinux: unknown extended permission (%u) will be ignored\n\0".as_ptr() as *const c_char, (*(*node).datum.u.xperms).specified as c_uint); return; } } specified = (*node).key.specified & !(AVTAB_ENABLED | AVTAB_ENABLED_OLD); if specified == AVTAB_XPERMS_ALLOWED { (*xpermd).used |= XPERMS_ALLOWED; update_xperms_extended_data((*(*node).datum.u.xperms).specified, &(*(*node).datum.u.xperms).perms, (*xpermd).allowed); } else if specified == AVTAB_XPERMS_AUDITALLOW { (*xpermd).used |= XPERMS_AUDITALLOW; update_xperms_extended_data((*(*node).datum.u.xperms).specified, &(*(*node).datum.u.xperms).perms, (*xpermd).auditallow); } else if specified == AVTAB_XPERMS_DONTAUDIT { (*xpermd).used |= XPERMS_DONTAUDIT; update_xperms_extended_data((*(*node).datum.u.xperms).specified, &(*(*node).datum.u.xperms).perms, (*xpermd).dontaudit); } else { pr_warn_once(b"SELinux: unknown specified key (%u)\n\0".as_ptr() as *const c_char, (*node).key.specified as c_uint); } }

#[no_mangle] pub unsafe extern "C" fn security_compute_av(ssid: u32, tsid: u32, orig_tclass: u16, avd: *mut av_decision, xperms: *mut extended_perms) { let policy: *mut selinux_policy; let policydb: *mut policydb; let sidtab: *mut sidtab; let tclass: u16; let scontext: *mut context; let tcontext: *mut context; rcu_read_lock(); policy = rcu_dereference(selinux_state.policy); avd_init(policy, avd); (*xperms).len = 0; if selinux_initialized() == 0 { (*avd).allowed = 0xffffffff; rcu_read_unlock(); return; } policydb = &mut (*policy).policydb; sidtab = (*policy).sidtab; scontext = sidtab_search(sidtab, ssid); if scontext.is_null() { pr_err(b"SELinux: %s:  unrecognized SID %d\n\0".as_ptr() as *const c_char, b"security_compute_av\0".as_ptr() as *const c_char, ssid); rcu_read_unlock(); return; } if ebitmap_get_bit(&(*policydb).permissive_map, (*scontext).type_) != 0 { (*avd).flags |= AVD_FLAGS_PERMISSIVE; } if ebitmap_get_bit(&(*policydb).neveraudit_map, (*scontext).type_) != 0 { (*avd).flags |= AVD_FLAGS_NEVERAUDIT; } if (*avd).flags == (AVD_FLAGS_PERMISSIVE | AVD_FLAGS_NEVERAUDIT) { (*avd).allowed = 0xffffffff; rcu_read_unlock(); (*avd).auditallow = 0; (*avd).auditdeny = 0; return; } tcontext = sidtab_search(sidtab, tsid); if tcontext.is_null() { pr_err(b"SELinux: %s:  unrecognized SID %d\n\0".as_ptr() as *const c_char, b"security_compute_av\0".as_ptr() as *const c_char, tsid); rcu_read_unlock(); return; } tclass = unmap_class(&mut (*policy).map, orig_tclass); if unlikely(orig_tclass != 0 && tclass == 0) { if (*policydb).allow_unknown != 0 { (*avd).allowed = 0xffffffff; } rcu_read_unlock(); return; } context_struct_compute_av(policydb, scontext, tcontext, tclass, avd, xperms); map_decision(&mut (*policy).map, orig_tclass, avd, (*policydb).allow_unknown); rcu_read_unlock(); if (*avd).flags & AVD_FLAGS_NEVERAUDIT != 0 { (*avd).auditallow = 0; (*avd).auditdeny = 0; } }

#[no_mangle] pub unsafe extern "C" fn security_compute_av_user(ssid: u32, tsid: u32, tclass: u16, avd: *mut av_decision) { let mut xp: extended_perms = zeroed(); security_compute_av(ssid, tsid, tclass, avd, &mut xp); }

#[no_mangle] pub unsafe extern "C" fn security_validate_transition_user(oldsid: u32, newsid: u32, tasksid: u32, tclass: u16) -> c_int { security_compute_validatetrans(oldsid, newsid, tasksid, tclass, true) }
#[no_mangle] pub unsafe extern "C" fn security_validate_transition(oldsid: u32, newsid: u32, tasksid: u32, orig_tclass: u16) -> c_int { security_compute_validatetrans(oldsid, newsid, tasksid, orig_tclass, false) }
unsafe fn security_compute_validatetrans(_oldsid: u32, _newsid: u32, _tasksid: u32, _orig_tclass: u16, _user: bool) -> c_int { unimplemented!() }

#[no_mangle] pub unsafe extern "C" fn security_bounded_transition(_old_sid: u32, _new_sid: u32) -> c_int { unimplemented!() }
#[no_mangle] pub unsafe extern "C" fn security_compute_xperms_decision(_ssid: u32, _tsid: u32, _orig_tclass: u16, _driver: u8, _base_perm: u8, _xpermd: *mut extended_perms_decision) { unimplemented!() }
#[no_mangle] pub unsafe extern "C" fn security_sidtab_hash_stats(page: *mut c_char) -> c_int { if selinux_initialized() == 0 { pr_err(b"SELinux: %s:  called before initial load_policy\n\0".as_ptr() as *const c_char, b"security_sidtab_hash_stats\0".as_ptr() as *const c_char); return neg(EINVAL); } rcu_read_lock(); let policy = rcu_dereference(selinux_state.policy); let rc = sidtab_hash_stats((*policy).sidtab, page); rcu_read_unlock(); rc }
#[no_mangle] pub unsafe extern "C" fn security_get_initial_sid_context(sid: u32) -> *const c_char { if sid > SECINITSID_NUM { return null(); } *initial_sid_to_string.add(sid as usize) }
#[no_mangle] pub unsafe extern "C" fn security_sid_to_context(sid: u32, scontext: *mut *mut c_char, scontext_len: *mut u32) -> c_int { security_sid_to_context_core(sid, scontext, scontext_len, false, false) }
#[no_mangle] pub unsafe extern "C" fn security_sid_to_context_force(sid: u32, scontext: *mut *mut c_char, scontext_len: *mut u32) -> c_int { security_sid_to_context_core(sid, scontext, scontext_len, true, false) }
#[no_mangle] pub unsafe extern "C" fn security_sid_to_context_inval(sid: u32, scontext: *mut *mut c_char, scontext_len: *mut u32) -> c_int { security_sid_to_context_core(sid, scontext, scontext_len, true, true) }
unsafe fn security_sid_to_context_core(_sid: u32, _scontext: *mut *mut c_char, _scontext_len: *mut u32, _force: bool, _only_invalid: bool) -> c_int { unimplemented!() }
#[no_mangle] pub unsafe extern "C" fn security_context_to_sid(scontext: *const c_char, scontext_len: u32, sid: *mut u32, gfp: gfp_t) -> c_int { security_context_to_sid_core(scontext, scontext_len, sid, SECSID_NULL, gfp, false) }
#[no_mangle] pub unsafe extern "C" fn security_context_str_to_sid(scontext: *const c_char, sid: *mut u32, gfp: gfp_t) -> c_int { security_context_to_sid(scontext, strlen(scontext) as u32, sid, gfp) }
#[no_mangle] pub unsafe extern "C" fn security_context_to_sid_default(scontext: *const c_char, scontext_len: u32, sid: *mut u32, def_sid: u32, gfp_flags: gfp_t) -> c_int { security_context_to_sid_core(scontext, scontext_len, sid, def_sid, gfp_flags, true) }
#[no_mangle] pub unsafe extern "C" fn security_context_to_sid_force(scontext: *const c_char, scontext_len: u32, sid: *mut u32) -> c_int { security_context_to_sid_core(scontext, scontext_len, sid, SECSID_NULL, GFP_KERNEL, true) }
unsafe fn security_context_to_sid_core(_scontext: *const c_char, _scontext_len: u32, _sid: *mut u32, _def_sid: u32, _gfp_flags: gfp_t, _force: bool) -> c_int { unimplemented!() }
#[no_mangle] pub unsafe extern "C" fn security_transition_sid(ssid: u32, tsid: u32, tclass: u16, qstr: *const qstr, out_sid: *mut u32) -> c_int { security_compute_sid(ssid, tsid, tclass, AVTAB_TRANSITION, if !qstr.is_null() { (*qstr).name } else { null() }, out_sid, true) }
#[no_mangle] pub unsafe extern "C" fn security_transition_sid_user(ssid: u32, tsid: u32, tclass: u16, objname: *const c_char, out_sid: *mut u32) -> c_int { security_compute_sid(ssid, tsid, tclass, AVTAB_TRANSITION, objname, out_sid, false) }
#[no_mangle] pub unsafe extern "C" fn security_member_sid(ssid: u32, tsid: u32, tclass: u16, out_sid: *mut u32) -> c_int { security_compute_sid(ssid, tsid, tclass, AVTAB_MEMBER, null(), out_sid, false) }
#[no_mangle] pub unsafe extern "C" fn security_change_sid(ssid: u32, tsid: u32, tclass: u16, out_sid: *mut u32) -> c_int { security_compute_sid(ssid, tsid, tclass, AVTAB_CHANGE, null(), out_sid, false) }
unsafe fn security_compute_sid(_ssid: u32, _tsid: u32, _orig_tclass: u16, _specified: u16, _objname: *const c_char, _out_sid: *mut u32, _kern: bool) -> c_int { unimplemented!() }
#[no_mangle] pub unsafe extern "C" fn services_convert_context(_args: *mut convert_context_args, _oldc: *mut context, _newc: *mut context, _gfp_flags: gfp_t) -> c_int { unimplemented!() }
#[no_mangle] pub unsafe extern "C" fn selinux_policy_cancel(_load_state: *mut selinux_load_state) { unimplemented!() }
#[no_mangle] pub unsafe extern "C" fn selinux_policy_commit(_load_state: *mut selinux_load_state) { unimplemented!() }
#[no_mangle] pub unsafe extern "C" fn security_load_policy(_data: *mut c_void, _len: size_t, _load_state: *mut selinux_load_state) -> c_int { unimplemented!() }
#[no_mangle] pub unsafe extern "C" fn security_port_sid(_protocol: u8, _port: u16, _out_sid: *mut u32) -> c_int { unimplemented!() }
#[no_mangle] pub unsafe extern "C" fn security_ib_pkey_sid(_subnet_prefix: u64, _pkey_num: u16, _out_sid: *mut u32) -> c_int { unimplemented!() }
#[no_mangle] pub unsafe extern "C" fn security_ib_endport_sid(_dev_name: *const c_char, _port_num: u8, _out_sid: *mut u32) -> c_int { unimplemented!() }
#[no_mangle] pub unsafe extern "C" fn security_netif_sid(_name: *const c_char, _if_sid: *mut u32) -> c_int { unimplemented!() }
unsafe fn match_ipv6_addrmask(input: *const u32, addr: *const u32, mask: *const u32) -> bool { let mut i = 0; while i < 4 { if *addr.add(i) != (*input.add(i) & *mask.add(i)) { return false; } i += 1; } true }
#[no_mangle] pub unsafe extern "C" fn security_node_sid(_domain: u16, _addrp: *const c_void, _addrlen: u32, _out_sid: *mut u32) -> c_int { unimplemented!() }
#[no_mangle] pub unsafe extern "C" fn security_genfs_sid(_fstype: *const c_char, _path: *const c_char, _orig_sclass: u16, _sid: *mut u32) -> c_int { unimplemented!() }
#[no_mangle] pub unsafe extern "C" fn selinux_policy_genfs_sid(_policy: *mut selinux_policy, _fstype: *const c_char, _path: *const c_char, _orig_sclass: u16, _sid: *mut u32) -> c_int { unimplemented!() }
#[no_mangle] pub unsafe extern "C" fn security_fs_use(_sb: *mut super_block) -> c_int { unimplemented!() }
#[no_mangle] pub unsafe extern "C" fn security_get_bools(_policy: *mut selinux_policy, _len: *mut u32, _names: *mut *mut *mut c_char, _values: *mut *mut c_int) -> c_int { unimplemented!() }
#[no_mangle] pub unsafe extern "C" fn security_set_bools(_len: u32, _values: *const c_int) -> c_int { unimplemented!() }
#[no_mangle] pub unsafe extern "C" fn security_get_bool_value(_index: u32) -> c_int { unimplemented!() }
#[no_mangle] pub unsafe extern "C" fn security_sid_mls_copy(_sid: u32, _mls_sid: u32, _new_sid: *mut u32) -> c_int { unimplemented!() }
#[no_mangle] pub unsafe extern "C" fn security_net_peersid_resolve(_nlbl_sid: u32, _nlbl_type: u32, _xfrm_sid: u32, _peer_sid: *mut u32) -> c_int { unimplemented!() }
#[no_mangle] pub unsafe extern "C" fn security_get_classes(_policy: *mut selinux_policy, _classes: *mut *mut *mut c_char, _nclasses: *mut u32) -> c_int { unimplemented!() }
#[no_mangle] pub unsafe extern "C" fn security_get_permissions(_policy: *mut selinux_policy, _class: *const c_char, _perms: *mut *mut *mut c_char, _nperms: *mut u32) -> c_int { unimplemented!() }
#[no_mangle] pub unsafe extern "C" fn security_get_reject_unknown() -> c_int { if selinux_initialized() == 0 { return 0; } rcu_read_lock(); let policy = rcu_dereference(selinux_state.policy); let value = (*policy).policydb.reject_unknown; rcu_read_unlock(); value }
#[no_mangle] pub unsafe extern "C" fn security_get_allow_unknown() -> c_int { if selinux_initialized() == 0 { return 0; } rcu_read_lock(); let policy = rcu_dereference(selinux_state.policy); let value = (*policy).policydb.allow_unknown; rcu_read_unlock(); value }
#[no_mangle] pub unsafe extern "C" fn security_policycap_supported(req_cap: c_uint) -> c_int { if selinux_initialized() == 0 { return 0; } rcu_read_lock(); let policy = rcu_dereference(selinux_state.policy); let rc = ebitmap_get_bit(&(*policy).policydb.policycaps, req_cap); rcu_read_unlock(); rc }
#[repr(C)] pub struct selinux_audit_rule { pub au_seqno: u32, pub au_ctxt: context }
#[no_mangle] pub unsafe extern "C" fn selinux_audit_rule_avc_callback(event: u32) -> c_int { if event == AVC_CALLBACK_RESET { return audit_update_lsm_rules(); } 0 }
#[no_mangle] pub unsafe extern "C" fn selinux_audit_rule_free(vrule: *mut c_void) { let rule = vrule as *mut selinux_audit_rule; if !rule.is_null() { context_destroy(&mut (*rule).au_ctxt); kfree(rule as *mut c_void); } }
#[no_mangle] pub unsafe extern "C" fn selinux_audit_rule_init(_field: u32, _op: u32, _rulestr: *mut c_char, _vrule: *mut *mut c_void, _gfp: gfp_t) -> c_int { unimplemented!() }
#[no_mangle] pub unsafe extern "C" fn selinux_audit_rule_known(rule: *mut audit_krule) -> c_int { let mut i = 0; while i < (*rule).field_count { let f = (*rule).fields.add(i as usize); match (*f).type_ { AUDIT_SUBJ_USER | AUDIT_SUBJ_ROLE | AUDIT_SUBJ_TYPE | AUDIT_SUBJ_SEN | AUDIT_SUBJ_CLR | AUDIT_OBJ_USER | AUDIT_OBJ_ROLE | AUDIT_OBJ_TYPE | AUDIT_OBJ_LEV_LOW | AUDIT_OBJ_LEV_HIGH => return 1, _ => {} } i += 1; } 0 }
#[no_mangle] pub unsafe extern "C" fn selinux_audit_rule_match(_prop: *mut lsm_prop, _field: u32, _op: u32, _vrule: *mut c_void) -> c_int { unimplemented!() }
/* CONFIG_NETLABEL */
#[no_mangle] pub unsafe extern "C" fn security_netlbl_secattr_to_sid(_secattr: *mut netlbl_lsm_secattr, _sid: *mut u32) -> c_int { unimplemented!() }
#[no_mangle] pub unsafe extern "C" fn security_netlbl_sid_to_secattr(_sid: u32, _secattr: *mut netlbl_lsm_secattr) -> c_int { unimplemented!() }
unsafe fn __security_read_policy(policy: *mut selinux_policy, data: *mut c_void, len: *mut size_t) -> c_int { let mut fp = policy_file { data, len: *len }; let rc = policydb_write(&mut (*policy).policydb, &mut fp); if rc != 0 { return rc; } *len = (fp.data as usize).wrapping_sub(data as usize); 0 }
#[no_mangle] pub unsafe extern "C" fn security_read_policy(data: *mut *mut c_void, len: *mut size_t) -> c_int { let state = &mut selinux_state as *mut selinux_state; let policy = rcu_dereference_protected((*state).policy, lockdep_is_held(&(*state).policy_mutex as *const _ as *const c_void)); if policy.is_null() { return neg(EINVAL); } *len = (*policy).policydb.len; *data = vmalloc_user(*len); if (*data).is_null() { return neg(ENOMEM); } __security_read_policy(policy, *data, len) }
#[no_mangle] pub unsafe extern "C" fn security_read_state_kernel(data: *mut *mut c_void, len: *mut size_t) -> c_int { let state = &mut selinux_state as *mut selinux_state; let policy = rcu_dereference_protected((*state).policy, lockdep_is_held(&(*state).policy_mutex as *const _ as *const c_void)); if policy.is_null() { return neg(EINVAL); } *len = (*policy).policydb.len; *data = vmalloc(*len); if (*data).is_null() { return neg(ENOMEM); } let err = __security_read_policy(policy, *data, len); if err != 0 { vfree(*data); *data = null_mut(); *len = 0; } err }

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
