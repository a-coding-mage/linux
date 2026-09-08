/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * A policy database (policydb) specifies the
 * configuration data for the security policy.
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
 */

use core::ffi::{c_char, c_void};

/*
 * C dependencies removed from executable Rust:
 * symtab.h, avtab.h, sidtab.h, ebitmap.h, mls_types.h, context.h,
 * constraint.h.
 */

/*
 * A datum type is defined for each kind of symbol
 * in the configuration data:  individual permissions,
 * common prefixes for access vectors, classes,
 * users, roles, types, sensitivities, categories, etc.
 */

/* Permission attributes */
#[repr(C)]
pub struct perm_datum {
    pub value: u32, /* permission bit + 1 */
}

/* Attributes of a common prefix for access vectors */
#[repr(C)]
pub struct common_datum {
    pub value: u32,             /* internal common value */
    pub permissions: symtab,    /* common permissions */
}

/* Options how a new object user, role, and type should be decided */
pub const DEFAULT_SOURCE: i32 = 1;
pub const DEFAULT_TARGET: i32 = 2;

/* Options how a new object range should be decided */
pub const DEFAULT_SOURCE_LOW: i32 = 1;
pub const DEFAULT_SOURCE_HIGH: i32 = 2;
pub const DEFAULT_SOURCE_LOW_HIGH: i32 = 3;
pub const DEFAULT_TARGET_LOW: i32 = 4;
pub const DEFAULT_TARGET_HIGH: i32 = 5;
pub const DEFAULT_TARGET_LOW_HIGH: i32 = 6;
pub const DEFAULT_GLBLUB: i32 = 7;

/* Class attributes */
#[repr(C)]
pub struct class_datum {
    pub value: u16,                         /* class value */
    pub comkey: *mut c_char,                /* common name */
    pub comdatum: *mut common_datum,        /* common datum */
    pub permissions: symtab,                /* class-specific permission symbol table */
    pub constraints: *mut constraint_node,  /* constraints on class perms */
    pub validatetrans: *mut constraint_node, /* special transition rules */
    pub default_user: c_char,
    pub default_role: c_char,
    pub default_type: c_char,
    pub default_range: c_char,
}

/* Role attributes */
#[repr(C)]
pub struct role_datum {
    pub value: u32,         /* internal role value */
    pub bounds: u32,        /* boundary of role, 0 for none */
    pub dominates: ebitmap, /* set of roles dominated by this role */
    pub types: ebitmap,     /* set of authorized types for role */
}

#[repr(C)]
pub struct role_trans_key {
    pub role: u32,   /* current role */
    pub type_: u32,  /* program executable type, or new object type */
    pub tclass: u16, /* process class, or new object class */
}

#[repr(C)]
pub struct role_trans_datum {
    pub new_role: u32, /* new role */
}

#[repr(C)]
pub struct filename_trans_key {
    pub ttype: u32,             /* parent dir context */
    pub tclass: u16,            /* class of new object */
    pub name: *const c_char,    /* last path component */
}

#[repr(C)]
pub struct filename_trans_datum {
    pub stypes: ebitmap,                    /* bitmap of source types for this otype */
    pub otype: u32,                         /* resulting type of new object */
    pub next: *mut filename_trans_datum,    /* record for next otype*/
}

#[repr(C)]
pub struct role_allow {
    pub role: u32,              /* current role */
    pub new_role: u32,          /* new role */
    pub next: *mut role_allow,
}

/* Type attributes */
#[repr(C)]
pub struct type_datum {
    pub value: u32,             /* internal type value */
    pub bounds: u32,            /* boundary of type, 0 for none */
    /* internally unused, only forwarded via policydb_write() */
    pub primary: u8,            /* primary name? */
    pub attribute: u8,          /* attribute ?*/
}

/* User attributes */
#[repr(C)]
pub struct user_datum {
    pub value: u32,             /* internal user value */
    pub bounds: u32,            /* bounds of user, 0 for none */
    pub roles: ebitmap,         /* set of authorized roles for user */
    pub range: mls_range,       /* MLS range (min - max) for user */
    pub dfltlevel: mls_level,   /* default login MLS level for user */
}

/* Sensitivity attributes */
#[repr(C)]
pub struct level_datum {
    pub level: mls_level,       /* sensitivity and associated categories */
    pub isalias: u8,            /* is this sensitivity an alias for another? */
}

/* Category attributes */
#[repr(C)]
pub struct cat_datum {
    pub value: u32,             /* internal category bit + 1 */
    pub isalias: u8,            /* is this category an alias for another? */
}

#[repr(C)]
pub struct range_trans {
    pub source_type: u32,
    pub target_type: u32,
    pub target_class: u16,
}

/* Boolean data type */
#[repr(C)]
pub struct cond_bool_datum {
    pub value: u32, /* internal type value */
    pub state: i32,
}

#[repr(C)]
pub struct cond_node {
    _unused: [u8; 0],
}

/*
 * type set preserves data needed to determine constraint info from
 * policy source. This is not used by the kernel policy but allows
 * utilities such as audit2allow to determine constraint denials.
 */
#[repr(C)]
pub struct type_set {
    pub types: ebitmap,
    pub negset: ebitmap,
    pub flags: u32,
}

#[repr(C)]
pub struct ocontext_port {
    pub protocol: u8,
    pub low_port: u16,
    pub high_port: u16,
}

#[repr(C)]
pub struct ocontext_node {
    pub addr: u32,
    pub mask: u32,
}

#[repr(C)]
pub struct ocontext_node6 {
    pub addr: [u32; 4],
    pub mask: [u32; 4],
}

#[repr(C)]
pub struct ocontext_ibpkey {
    pub subnet_prefix: u64,
    pub low_pkey: u16,
    pub high_pkey: u16,
}

#[repr(C)]
pub struct ocontext_ibendport {
    pub dev_name: *mut c_char,
    pub port: u8,
}

#[repr(C)]
pub union ocontext_u {
    pub name: *mut c_char, /* name of initial SID, fs, netif, fstype, path */
    pub port: ocontext_port, /* TCP or UDP port information */
    pub node: ocontext_node, /* node information */
    pub node6: ocontext_node6, /* IPv6 node information */
    pub ibpkey: ocontext_ibpkey,
    pub ibendport: ocontext_ibendport,
}

#[repr(C)]
pub union ocontext_v {
    pub sclass: u16,  /* security class for genfs (can be 0 for wildcard) */
    pub behavior: u32, /* labeling behavior for fs_use */
}

/*
 * The configuration data includes security contexts for
 * initial SIDs, unlabeled file systems, TCP and UDP port numbers,
 * network interfaces, and nodes.  This structure stores the
 * relevant data for one such entry.  Entries of the same kind
 * (e.g. all initial SIDs) are linked together into a list.
 */
#[repr(C)]
pub struct ocontext {
    pub u: ocontext_u,
    pub v: ocontext_v,
    pub context: [context; 2], /* security context(s) */
    pub sid: [u32; 2],         /* SID(s) */
    pub next: *mut ocontext,
}

#[repr(C)]
pub struct genfs {
    pub fstype: *mut c_char,
    pub head: *mut ocontext,
    pub next: *mut genfs,
}

/* symbol table array indices */
pub const SYM_COMMONS: usize = 0;
pub const SYM_CLASSES: usize = 1;
pub const SYM_ROLES: usize = 2;
pub const SYM_TYPES: usize = 3;
pub const SYM_USERS: usize = 4;
pub const SYM_BOOLS: usize = 5;
pub const SYM_LEVELS: usize = 6;
pub const SYM_CATS: usize = 7;
pub const SYM_NUM: usize = 8;

/* object context array indices */
pub const OCON_ISID: usize = 0;      /* initial SIDs */
pub const OCON_FS: usize = 1;        /* unlabeled file systems (deprecated) */
pub const OCON_PORT: usize = 2;      /* TCP and UDP port numbers */
pub const OCON_NETIF: usize = 3;     /* network interfaces */
pub const OCON_NODE: usize = 4;      /* nodes */
pub const OCON_FSUSE: usize = 5;     /* fs_use */
pub const OCON_NODE6: usize = 6;     /* IPv6 nodes */
pub const OCON_IBPKEY: usize = 7;    /* Infiniband PKeys */
pub const OCON_IBENDPORT: usize = 8; /* Infiniband end ports */
pub const OCON_NUM: usize = 9;

/* The policy database */
#[repr(C)]
pub struct policydb {
    pub mls_enabled: i32,

    /* symbol tables */
    pub symtab: [symtab; SYM_NUM],

    /* symbol names indexed by (value - 1) */
    pub sym_val_to_name: [*mut *mut c_char; SYM_NUM],

    /* class, role, and user attributes indexed by (value - 1) */
    pub class_val_to_struct: *mut *mut class_datum,
    pub role_val_to_struct: *mut *mut role_datum,
    pub user_val_to_struct: *mut *mut user_datum,
    pub type_val_to_struct: *mut *mut type_datum,

    /* type enforcement access vectors and transitions */
    pub te_avtab: avtab,

    /* role transitions */
    pub role_tr: hashtab,

    /* file transitions with the last path component */
    /* quickly exclude lookups when parent ttype has no rules */
    pub filename_trans_ttypes: ebitmap,
    /* actual set of filename_trans rules */
    pub filename_trans: hashtab,
    /* only used if policyvers < POLICYDB_VERSION_COMP_FTRANS */
    pub compat_filename_trans_count: u32,

    /* bools indexed by (value - 1) */
    pub bool_val_to_struct: *mut *mut cond_bool_datum,
    /* type enforcement conditional access vectors and transitions */
    pub te_cond_avtab: avtab,
    /* array indexing te_cond_avtab by conditional */
    pub cond_list: *mut cond_node,
    pub cond_list_len: u32,

    /* role allows */
    pub role_allow: *mut role_allow,

    /* security contexts of initial SIDs, unlabeled file systems,
       TCP or UDP port numbers, network interfaces and nodes */
    pub ocontexts: [*mut ocontext; OCON_NUM],

    /* security contexts for files in filesystems that cannot support
       a persistent label mapping or use another
       fixed labeling behavior. */
    pub genfs: *mut genfs,

    /* range transitions table (range_trans_key -> mls_range) */
    pub range_tr: hashtab,

    /* type -> attribute reverse mapping */
    pub type_attr_map_array: *mut ebitmap,

    pub policycaps: ebitmap,

    pub permissive_map: ebitmap,

    pub neveraudit_map: ebitmap,

    /* length of this policy when it was loaded */
    pub len: usize,

    pub policyvers: u32,

    /*
     * C bitfields:
     * unsigned int reject_unknown : 1;
     * unsigned int allow_unknown : 1;
     */
    pub reject_unknown_allow_unknown: u32,

    pub process_class: u16,
    pub process_trans_perms: u32,
}

/* C field-access macros:
 * p_commons -> symtab[SYM_COMMONS]
 * p_classes -> symtab[SYM_CLASSES]
 * p_roles   -> symtab[SYM_ROLES]
 * p_types   -> symtab[SYM_TYPES]
 * p_users   -> symtab[SYM_USERS]
 * p_bools   -> symtab[SYM_BOOLS]
 * p_levels  -> symtab[SYM_LEVELS]
 * p_cats    -> symtab[SYM_CATS]
 */

#[repr(C)]
pub struct policy_file {
    pub data: *mut c_char,
    pub len: usize,
}

unsafe extern "C" {
    pub fn policydb_destroy(p: *mut policydb);
    pub fn policydb_load_isids(p: *mut policydb, s: *mut sidtab) -> i32;
    pub fn policydb_context_isvalid(p: *const policydb, c: *const context) -> bool;
    pub fn policydb_class_isvalid(p: *const policydb, class: u16) -> bool;
    pub fn policydb_type_isvalid(p: *const policydb, type_: u32) -> bool;
    pub fn policydb_simpletype_isvalid(p: *const policydb, type_: u32) -> bool;
    pub fn policydb_role_isvalid(p: *const policydb, role: u32) -> bool;
    pub fn policydb_user_isvalid(p: *const policydb, user: u32) -> bool;
    pub fn policydb_read(p: *mut policydb, fp: *mut policy_file) -> i32;
    pub fn policydb_write(p: *mut policydb, fp: *mut policy_file) -> i32;

    pub fn policydb_filenametr_search(
        p: *mut policydb,
        key: *mut filename_trans_key,
    ) -> *mut filename_trans_datum;

    pub fn policydb_rangetr_search(
        p: *mut policydb,
        key: *mut range_trans,
    ) -> *mut mls_range;

    pub fn policydb_roletr_search(
        p: *mut policydb,
        key: *mut role_trans_key,
    ) -> *mut role_trans_datum;
}

pub const POLICYDB_CONFIG_MLS: i32 = 1;

/* the config flags related to unknown classes/perms are bits 2 and 3 */
pub const REJECT_UNKNOWN: u32 = 0x00000002;
pub const ALLOW_UNKNOWN: u32 = 0x00000004;

pub const OBJECT_R: &[u8; 9] = b"object_r\0";
pub const OBJECT_R_VAL: i32 = 1;

pub const POLICYDB_MAGIC: u32 = SELINUX_MAGIC;
pub const POLICYDB_STRING: &[u8; 9] = b"SE Linux\0";

#[repr(C)]
pub struct policy_data {
    pub p: *mut policydb,
    pub fp: *mut policy_file,
}

#[inline]
pub unsafe fn size_check(bytes: usize, num: usize, fp: *const policy_file) -> i32 {
    let mut len: usize = 0;

    if unlikely(check_mul_overflow(bytes, num, &mut len as *mut usize)) {
        return -EINVAL;
    }

    if unlikely(len > (*fp).len) {
        return -EINVAL;
    }

    0
}

#[inline]
pub unsafe fn next_entry(buf: *mut c_void, fp: *mut policy_file, bytes: usize) -> i32 {
    if bytes > (*fp).len {
        return -EINVAL;
    }

    memcpy(buf, (*fp).data as *const c_void, bytes);
    (*fp).data = (*fp).data.add(bytes);
    (*fp).len -= bytes;
    0
}

#[inline]
pub unsafe fn put_entry(
    buf: *const c_void,
    bytes: usize,
    num: usize,
    fp: *mut policy_file,
) -> i32 {
    let mut len: usize = 0;

    if unlikely(check_mul_overflow(bytes, num, &mut len as *mut usize)) {
        return -EINVAL;
    }

    if len > (*fp).len {
        return -EINVAL;
    }
    memcpy((*fp).data as *mut c_void, buf, len);
    (*fp).data = (*fp).data.add(len);
    (*fp).len -= len;

    0
}

#[inline]
pub unsafe fn sym_name(
    p: *const policydb,
    sym_num: u32,
    element_nr: u32,
) -> *const c_char {
    *(*(*p).sym_val_to_name.as_ptr().add(sym_num as usize)).add(element_nr as usize)
        as *const c_char
}

#[inline]
pub fn val_is_boolean(value: u32) -> bool {
    value == 0 || value == 1
}

unsafe extern "C" {
    pub fn str_read(
        strp: *mut *mut c_char,
        flags: gfp_t,
        fp: *mut policy_file,
        len: u32,
    ) -> i32;

    pub fn string_to_security_class(p: *mut policydb, name: *const c_char) -> u16;
    pub fn string_to_av_perm(p: *mut policydb, tclass: u16, name: *const c_char) -> u32;
}

/*
 * C variadic macro translated as a Rust macro with equivalent one-policy
 * warning suppression behavior at each expansion site.
 */
#[macro_export]
macro_rules! pr_warn_once_policyload {
    ($policy:expr, $fmt:expr $(, $arg:expr)* $(,)?) => {{
        static mut PREV_POLICY__: *const core::ffi::c_void = core::ptr::null();
        let policy__ = $policy as *const core::ffi::c_void;
        unsafe {
            if PREV_POLICY__ != policy__ {
                pr_warn!($fmt $(, $arg)*);
                PREV_POLICY__ = policy__;
            }
        }
    }};
}


// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
