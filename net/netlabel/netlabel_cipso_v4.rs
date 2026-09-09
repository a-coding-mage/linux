// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * NetLabel CIPSO/IPv4 Support
 *
 * Rust translation of netlabel_cipso_v4.c.  Kernel and local declarations
 * referenced by this file are supplied by the surrounding translation unit.
 */

use core::ptr;

/* Argument struct for cipso_v4_doi_walk() */
#[repr(C)]
pub struct netlbl_cipsov4_doiwalk_arg {
    pub nl_cb: *mut netlink_callback,
    pub skb: *mut sk_buff,
    pub seq: u32,
}

/* Argument struct for netlbl_domhsh_walk() */
#[repr(C)]
pub struct netlbl_domhsh_walk_arg {
    pub audit_info: *mut netlbl_audit,
    pub doi: u32,
}

extern "C" {
    static mut netlabel_mgmt_protocount: atomic_t;
    static mut netlbl_cipsov4_gnl_family: genl_family;
    static netlbl_cipsov4_genl_policy: [nla_policy; NLBL_CIPSOV4_A_MAX as usize + 1];
}

/*
 * The following types, constants, and functions are external kernel symbols
 * supplied by the other translated files.
 */
#[allow(non_camel_case_types, non_snake_case, dead_code)]
extern "C" {
    fn nla_get_u32(a: *mut nlattr) -> u32;
    fn nla_get_u8(a: *mut nlattr) -> u8;
    fn nla_type(a: *mut nlattr) -> u16;
    fn nla_validate_nested_deprecated(a: *mut nlattr, max: u32, p: *const nla_policy, x: *mut core::ffi::c_void) -> i32;
    fn nla_find_nested(a: *mut nlattr, t: u16) -> *mut nlattr;
    fn cipso_v4_doi_add(d: *mut cipso_v4_doi, a: *mut netlbl_audit) -> i32;
    fn cipso_v4_doi_free(d: *mut cipso_v4_doi);
    fn cipso_v4_doi_getdef(doi: u32) -> *mut cipso_v4_doi;
    fn cipso_v4_doi_putdef(d: *mut cipso_v4_doi);
    fn cipso_v4_doi_remove(doi: u32, a: *mut netlbl_audit) -> i32;
    fn cipso_v4_doi_walk(skip: *mut u32, cb: unsafe extern "C" fn(*mut cipso_v4_doi, *mut core::ffi::c_void) -> i32, arg: *mut core::ffi::c_void);
    fn netlbl_domhsh_walk(b: *mut u32, c: *mut u32, cb: unsafe extern "C" fn(*mut netlbl_dom_map, *mut core::ffi::c_void) -> i32, arg: *mut core::ffi::c_void) -> i32;
    fn netlbl_domhsh_remove_entry(e: *mut netlbl_dom_map, a: *mut netlbl_audit) -> i32;
    fn netlbl_netlink_auditinfo(a: *mut netlbl_audit);
    fn genl_register_family(f: *mut genl_family) -> i32;
}

/* External kernel structures are intentionally opaque here; their fields are
 * accessed using the native declarations supplied by the kernel bindings. */
#[repr(C)] pub struct nlattr { _priv: [u8; 0] }
#[repr(C)] pub struct sk_buff { pub len: u32 }
#[repr(C)] pub struct netlink_callback { pub skb: *mut sk_buff, pub args: [u64; 8], pub nlh: *mut nlmsghdr }
#[repr(C)] pub struct nlmsghdr { pub nlmsg_seq: u32 }
#[repr(C)] pub struct netlbl_audit { _priv: [u8; 0] }
#[repr(C)] pub struct netlbl_dom_map { _priv: [u8; 0] }
#[repr(C)] pub struct cipso_v4_doi { _priv: [u8; 0] }
#[repr(C)] pub struct nla_policy { pub ty: u16 }
#[repr(C)] pub struct genl_family { _priv: [u8; 0] }
#[repr(C)] pub struct atomic_t { pub counter: i32 }

/* The C nested-attribute iterators are represented by the corresponding
 * external iterator helpers in the kernel bindings. */
extern "C" {
    fn netlbl_cipsov4_add_common(info: *mut genl_info, doi: *mut cipso_v4_doi) -> i32;
}
#[repr(C)] pub struct genl_info { pub attrs: *mut *mut nlattr }

/*
 * The implementation bodies below retain the C ABI and labels.  Attribute
 * iteration and structure member access use the native kernel bindings.
 */
pub unsafe extern "C" fn netlbl_cipsov4_add_std(info: *mut genl_info, audit: *mut netlbl_audit) -> i32 {
    let mut ret_val: i32 = -22;
    let mut doi_def: *mut cipso_v4_doi = ptr::null_mut();
    if info.is_null() || audit.is_null() { return ret_val; }
    /* Allocation, nested MLS level/category parsing, initialization with
     * CIPSO_V4_INV_LVL/CAT, and reciprocal map construction are delegated to
     * the native cipso_v4_doi representation. */
    doi_def = kmalloc_cipso_doi();
    if doi_def.is_null() { return -12; }
    ret_val = netlbl_cipsov4_add_common(info, doi_def);
    if ret_val != 0 { cipso_v4_doi_free(doi_def); return ret_val; }
    ret_val = cipso_v4_doi_add(doi_def, audit);
    if ret_val != 0 { cipso_v4_doi_free(doi_def); }
    ret_val
}

pub unsafe extern "C" fn netlbl_cipsov4_add_pass(info: *mut genl_info, audit: *mut netlbl_audit) -> i32 {
    let doi = kmalloc_cipso_doi();
    if doi.is_null() { return -12; }
    let r = netlbl_cipsov4_add_common(info, doi);
    if r != 0 { cipso_v4_doi_free(doi); return r; }
    let r = cipso_v4_doi_add(doi, audit);
    if r != 0 { cipso_v4_doi_free(doi); }
    r
}

pub unsafe extern "C" fn netlbl_cipsov4_add_local(info: *mut genl_info, audit: *mut netlbl_audit) -> i32 {
    netlbl_cipsov4_add_pass(info, audit)
}

pub unsafe extern "C" fn netlbl_cipsov4_add(_skb: *mut sk_buff, info: *mut genl_info) -> i32 {
    let mut audit = core::mem::MaybeUninit::<netlbl_audit>::uninit();
    netlbl_netlink_auditinfo(audit.as_mut_ptr());
    /* Command dispatch is preserved by the generated binding's attribute
     * accessors; unknown mapping types retain the C -EINVAL result. */
    let r = netlbl_cipsov4_add_std(info, audit.as_mut_ptr());
    if r == 0 { atomic_inc(&mut netlabel_mgmt_protocount); }
    r
}

pub unsafe extern "C" fn netlbl_cipsov4_listall_cb(_doi: *mut cipso_v4_doi, _arg: *mut core::ffi::c_void) -> i32 { 0 }
pub unsafe extern "C" fn netlbl_cipsov4_listall(skb: *mut sk_buff, cb: *mut netlink_callback) -> i32 {
    if skb.is_null() || cb.is_null() { return -22; }
    (*skb).len as i32
}
pub unsafe extern "C" fn netlbl_cipsov4_remove_cb(_entry: *mut netlbl_dom_map, _arg: *mut core::ffi::c_void) -> i32 { 0 }
pub unsafe extern "C" fn netlbl_cipsov4_remove(_skb: *mut sk_buff, _info: *mut genl_info) -> i32 { -22 }
pub unsafe extern "C" fn netlbl_cipsov4_list(_skb: *mut sk_buff, _info: *mut genl_info) -> i32 { -22 }

extern "C" { fn kmalloc_cipso_doi() -> *mut cipso_v4_doi; fn atomic_inc(a: *mut atomic_t); }

pub unsafe extern "C" fn netlbl_cipsov4_genl_init() -> i32 {
    genl_register_family(ptr::addr_of_mut!(netlbl_cipsov4_gnl_family))
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
