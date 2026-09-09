/* SPDX-License-Identifier: GPL-2.0-or-later */
/*
 * NetLabel Domain Hash Table
 *
 * This file manages the domain hash table that NetLabel uses to determine
 * which network labeling protocol to use for a given domain.  The NetLabel
 * system manages static and dynamic label mappings for network protocols such
 * as CIPSO and RIPSO.
 *
 * Author: Paul Moore <paul@paul-moore.com>
 */

/*
 * (c) Copyright Hewlett-Packard Development Company, L.P., 2006, 2008
 */

/* Domain hash table size */
/* XXX - currently this number is an uneducated guess */
pub const NETLBL_DOMHSH_BITSIZE: usize = 7;

/* Domain mapping definition structures */
#[repr(C)]
pub struct netlbl_domaddr_map {
    pub list4: list_head,
    pub list6: list_head,
}

#[repr(C)]
pub union netlbl_dommap_def__bindgen_ty_1 {
    pub addrsel: *mut netlbl_domaddr_map,
    pub cipso: *mut cipso_v4_doi,
    pub calipso: *mut calipso_doi,
}

#[repr(C)]
pub struct netlbl_dommap_def {
    pub type_: u32,
    pub __bindgen_anon_1: netlbl_dommap_def__bindgen_ty_1,
}

#[repr(C)]
pub struct netlbl_domaddr4_map {
    pub def: netlbl_dommap_def,
    pub list: netlbl_af4list,
}

/* container_of(iter, struct netlbl_domaddr4_map, list) */
#[macro_export]
macro_rules! netlbl_domhsh_addr4_entry {
    ($iter:expr) => {
        unsafe {
            ($iter as *mut u8).sub(core::mem::offset_of!(netlbl_domaddr4_map, list))
                as *mut netlbl_domaddr4_map
        }
    };
}

#[repr(C)]
pub struct netlbl_domaddr6_map {
    pub def: netlbl_dommap_def,
    pub list: netlbl_af6list,
}

/* container_of(iter, struct netlbl_domaddr6_map, list) */
#[macro_export]
macro_rules! netlbl_domhsh_addr6_entry {
    ($iter:expr) => {
        unsafe {
            ($iter as *mut u8).sub(core::mem::offset_of!(netlbl_domaddr6_map, list))
                as *mut netlbl_domaddr6_map
        }
    };
}

#[repr(C)]
pub struct netlbl_dom_map {
    pub domain: *mut core::ffi::c_char,
    pub def: netlbl_dommap_def,
    pub family: u16,
    pub valid: u32,
    pub list: list_head,
    pub rcu: rcu_head,
}

/* init function */
extern "C" {
    pub fn netlbl_domhsh_init(size: u32) -> core::ffi::c_int;

    /* Manipulate the domain hash table */
    pub fn netlbl_domhsh_add(
        entry: *mut netlbl_dom_map,
        audit_info: *mut netlbl_audit,
    ) -> core::ffi::c_int;
    pub fn netlbl_domhsh_add_default(
        entry: *mut netlbl_dom_map,
        audit_info: *mut netlbl_audit,
    ) -> core::ffi::c_int;
    pub fn netlbl_domhsh_remove_entry(
        entry: *mut netlbl_dom_map,
        audit_info: *mut netlbl_audit,
    ) -> core::ffi::c_int;
    pub fn netlbl_domhsh_remove_af4(
        domain: *const core::ffi::c_char,
        addr: *const in_addr,
        mask: *const in_addr,
        audit_info: *mut netlbl_audit,
    ) -> core::ffi::c_int;
    pub fn netlbl_domhsh_remove_af6(
        domain: *const core::ffi::c_char,
        addr: *const in6_addr,
        mask: *const in6_addr,
        audit_info: *mut netlbl_audit,
    ) -> core::ffi::c_int;
    pub fn netlbl_domhsh_remove(
        domain: *const core::ffi::c_char,
        family: u16,
        audit_info: *mut netlbl_audit,
    ) -> core::ffi::c_int;
    pub fn netlbl_domhsh_remove_default(
        family: u16,
        audit_info: *mut netlbl_audit,
    ) -> core::ffi::c_int;
    pub fn netlbl_domhsh_getentry(
        domain: *const core::ffi::c_char,
        family: u16,
    ) -> *mut netlbl_dom_map;
    pub fn netlbl_domhsh_getentry_af4(
        domain: *const core::ffi::c_char,
        addr: __be32,
    ) -> *mut netlbl_dommap_def;

    /* IPv6 declarations are conditional on CONFIG_IPV6 (IS_ENABLED). */
    pub fn netlbl_domhsh_getentry_af6(
        domain: *const core::ffi::c_char,
        addr: *const in6_addr,
    ) -> *mut netlbl_dommap_def;
    pub fn netlbl_domhsh_walk(
        skip_bkt: *mut u32,
        skip_chain: *mut u32,
        callback: Option<unsafe extern "C" fn(*mut netlbl_dom_map, *mut core::ffi::c_void) -> core::ffi::c_int>,
        cb_arg: *mut core::ffi::c_void,
    ) -> core::ffi::c_int;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
