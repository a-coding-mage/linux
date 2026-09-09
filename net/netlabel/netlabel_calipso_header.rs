/* SPDX-License-Identifier: GPL-2.0-or-later */
/*
 * NetLabel CALIPSO Support
 *
 * This file defines the CALIPSO functions for the NetLabel system.  The
 * NetLabel system manages static and dynamic label mappings for network
 * protocols such as CIPSO and RIPSO.
 *
 * Authors: Paul Moore <paul@paul-moore.com>
 *          Huw Davies <huw@codeweavers.com>
 */

/* (c) Copyright Hewlett-Packard Development Company, L.P., 2006
 * (c) Copyright Huw Davies <huw@codeweavers.com>, 2015
 */

/* C header dependencies: <net/netlabel.h>, <net/calipso.h>. */

use core::ffi::c_void;

/* Opaque types supplied by the dependent headers. */
pub enum calipso_doi {}
pub enum netlbl_audit {}
pub enum netlbl_lsm_secattr {}
pub enum sock {}
pub enum request_sock {}
pub enum sk_buff {}

/* The following NetLabel payloads are supported by the CALIPSO subsystem.
 *
 * o ADD:
 *   Sent by an application to add a new DOI mapping table.
 *
 * o REMOVE:
 *   Sent by an application to remove a specific DOI mapping table from the
 *   CALIPSO system.
 *
 * o LIST:
 *   Sent by an application to list the details of a DOI definition.
 *
 * o LISTALL:
 *   This message is sent by an application to list the valid DOIs on the
 *   system.
 */

/* NetLabel CALIPSO commands */
#[repr(C)]
pub enum NetlblCalipsoCommand {
    NLBL_CALIPSO_C_UNSPEC,
    NLBL_CALIPSO_C_ADD,
    NLBL_CALIPSO_C_REMOVE,
    NLBL_CALIPSO_C_LIST,
    NLBL_CALIPSO_C_LISTALL,
    __NLBL_CALIPSO_C_MAX,
}

/* NetLabel CALIPSO attributes */
#[repr(C)]
pub enum NetlblCalipsoAttribute {
    NLBL_CALIPSO_A_UNSPEC,
    NLBL_CALIPSO_A_DOI,
    /* (NLA_U32) the DOI value */
    NLBL_CALIPSO_A_MTYPE,
    /* (NLA_U32) the mapping table type (defined in calipso.h as
     * CALIPSO_MAP_*) */
    __NLBL_CALIPSO_A_MAX,
}

pub const NLBL_CALIPSO_A_MAX: u32 =
    NetlblCalipsoAttribute::__NLBL_CALIPSO_A_MAX as u32 - 1;

/* NetLabel protocol functions. The CONFIG_IPV6 condition is supplied by the
 * build configuration. */
#[cfg(feature = "CONFIG_IPV6")]
extern "C" {
    pub fn netlbl_calipso_genl_init() -> i32;
}

#[cfg(not(feature = "CONFIG_IPV6"))]
#[inline]
pub fn netlbl_calipso_genl_init() -> i32 {
    0
}

extern "C" {
    pub fn calipso_doi_add(
        doi_def: *mut calipso_doi,
        audit_info: *mut netlbl_audit,
    ) -> i32;
    pub fn calipso_doi_free(doi_def: *mut calipso_doi);
    pub fn calipso_doi_remove(doi: u32, audit_info: *mut netlbl_audit) -> i32;
    pub fn calipso_doi_getdef(doi: u32) -> *mut calipso_doi;
    pub fn calipso_doi_putdef(doi_def: *mut calipso_doi);
    pub fn calipso_doi_walk(
        skip_cnt: *mut u32,
        callback: Option<unsafe extern "C" fn(*mut calipso_doi, *mut c_void) -> i32>,
        cb_arg: *mut c_void,
    ) -> i32;
    pub fn calipso_sock_getattr(sk: *mut sock, secattr: *mut netlbl_lsm_secattr) -> i32;
    pub fn calipso_sock_setattr(
        sk: *mut sock,
        doi_def: *const calipso_doi,
        secattr: *const netlbl_lsm_secattr,
    ) -> i32;
    pub fn calipso_sock_delattr(sk: *mut sock);
    pub fn calipso_req_setattr(
        req: *mut request_sock,
        doi_def: *const calipso_doi,
        secattr: *const netlbl_lsm_secattr,
    ) -> i32;
    pub fn calipso_req_delattr(req: *mut request_sock);
    pub fn calipso_optptr(skb: *const sk_buff) -> *mut u8;
    pub fn calipso_getattr(calipso: *const u8, secattr: *mut netlbl_lsm_secattr) -> i32;
    pub fn calipso_skbuff_setattr(
        skb: *mut sk_buff,
        doi_def: *const calipso_doi,
        secattr: *const netlbl_lsm_secattr,
    ) -> i32;
    pub fn calipso_skbuff_delattr(skb: *mut sk_buff) -> i32;
    pub fn calipso_cache_invalidate();
    pub fn calipso_cache_add(
        calipso_ptr: *const u8,
        secattr: *const netlbl_lsm_secattr,
    ) -> i32;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
