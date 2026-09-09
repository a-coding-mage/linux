/* SPDX-License-Identifier: GPL-2.0-or-later */
/* Rust translation of netlabel.h; included kernel types/functions are external dependencies. */

pub const NETLBL_PROTO_VERSION: u32 = 3;
pub const NETLBL_NLTYPE_NONE: u32 = 0;
pub const NETLBL_NLTYPE_MGMT: u32 = 1;
pub const NETLBL_NLTYPE_MGMT_NAME: &str = "NLBL_MGMT";
pub const NETLBL_NLTYPE_RIPSO: u32 = 2;
pub const NETLBL_NLTYPE_RIPSO_NAME: &str = "NLBL_RIPSO";
pub const NETLBL_NLTYPE_CIPSOV4: u32 = 3;
pub const NETLBL_NLTYPE_CIPSOV4_NAME: &str = "NLBL_CIPSOv4";
pub const NETLBL_NLTYPE_CIPSOV6: u32 = 4;
pub const NETLBL_NLTYPE_CIPSOV6_NAME: &str = "NLBL_CIPSOv6";
pub const NETLBL_NLTYPE_UNLABELED: u32 = 5;
pub const NETLBL_NLTYPE_UNLABELED_NAME: &str = "NLBL_UNLBL";
pub const NETLBL_NLTYPE_ADDRSELECT: u32 = 6;
pub const NETLBL_NLTYPE_ADDRSELECT_NAME: &str = "NLBL_ADRSEL";
pub const NETLBL_NLTYPE_CALIPSO: u32 = 7;
pub const NETLBL_NLTYPE_CALIPSO_NAME: &str = "NLBL_CALIPSO";

pub const NETLBL_CATMAP_MAPCNT: usize = 4;
pub const NETLBL_CATMAP_MAPSIZE: usize = core::mem::size_of::<u64>() * 8;
pub const NETLBL_CATMAP_SIZE: usize = NETLBL_CATMAP_MAPSIZE * NETLBL_CATMAP_MAPCNT;
pub const NETLBL_CATMAP_BIT: u64 = 0x01;

pub const NETLBL_SECATTR_NONE: u32 = 0x00000000;
pub const NETLBL_SECATTR_DOMAIN: u32 = 0x00000001;
pub const NETLBL_SECATTR_CACHE: u32 = 0x00000002;
pub const NETLBL_SECATTR_MLS_LVL: u32 = 0x00000004;
pub const NETLBL_SECATTR_MLS_CAT: u32 = 0x00000008;
pub const NETLBL_SECATTR_SECID: u32 = 0x00000010;
pub const NETLBL_SECATTR_FREE_DOMAIN: u32 = 0x01000000;
pub const NETLBL_SECATTR_DOMAIN_CPY: u32 = NETLBL_SECATTR_DOMAIN | NETLBL_SECATTR_FREE_DOMAIN;
pub const NETLBL_SECATTR_CACHEABLE: u32 = NETLBL_SECATTR_MLS_LVL | NETLBL_SECATTR_MLS_CAT | NETLBL_SECATTR_SECID;

#[repr(C)]
pub struct netlbl_audit { pub prop: lsm_prop, pub loginuid: kuid_t, pub sessionid: u32 }

#[repr(C)]
pub struct netlbl_lsm_cache {
    pub refcount: refcount_t,
    pub free: Option<unsafe extern "C" fn(*const core::ffi::c_void)>,
    pub data: *mut core::ffi::c_void,
}

#[repr(C)]
pub struct netlbl_lsm_catmap {
    pub startbit: u32,
    pub bitmap: [u64; NETLBL_CATMAP_MAPCNT],
    pub next: *mut netlbl_lsm_catmap,
}

#[repr(C)]
pub struct netlbl_lsm_secattr {
    pub flags: u32,
    pub type_: u32,
    pub domain: *mut core::ffi::c_char,
    pub cache: *mut netlbl_lsm_cache,
    pub attr: netlbl_lsm_secattr_attr,
}
#[repr(C)]
pub struct netlbl_lsm_secattr_attr { pub mls: netlbl_lsm_secattr_mls, pub secid: u32 }
#[repr(C)]
pub struct netlbl_lsm_secattr_mls { pub cat: *mut netlbl_lsm_catmap, pub lvl: u32 }

#[repr(C)]
pub struct netlbl_calipso_ops {
    pub doi_add: Option<unsafe extern "C" fn(*mut calipso_doi, *mut netlbl_audit) -> i32>,
    pub doi_free: Option<unsafe extern "C" fn(*mut calipso_doi)>,
    pub doi_remove: Option<unsafe extern "C" fn(u32, *mut netlbl_audit) -> i32>,
    pub doi_getdef: Option<unsafe extern "C" fn(u32) -> *mut calipso_doi>,
    pub doi_putdef: Option<unsafe extern "C" fn(*mut calipso_doi)>,
    pub doi_walk: Option<unsafe extern "C" fn(*mut u32, Option<unsafe extern "C" fn(*mut calipso_doi, *mut core::ffi::c_void) -> i32>, *mut core::ffi::c_void) -> i32>,
    pub sock_getattr: Option<unsafe extern "C" fn(*mut sock, *mut netlbl_lsm_secattr) -> i32>,
    pub sock_setattr: Option<unsafe extern "C" fn(*mut sock, *const calipso_doi, *const netlbl_lsm_secattr) -> i32>,
    pub sock_delattr: Option<unsafe extern "C" fn(*mut sock)>,
    pub req_setattr: Option<unsafe extern "C" fn(*mut request_sock, *const calipso_doi, *const netlbl_lsm_secattr) -> i32>,
    pub req_delattr: Option<unsafe extern "C" fn(*mut request_sock)>,
    pub opt_getattr: Option<unsafe extern "C" fn(*const u8, *mut netlbl_lsm_secattr) -> i32>,
    pub skbuff_optptr: Option<unsafe extern "C" fn(*const sk_buff) -> *mut u8>,
    pub skbuff_setattr: Option<unsafe extern "C" fn(*mut sk_buff, *const calipso_doi, *const netlbl_lsm_secattr) -> i32>,
    pub skbuff_delattr: Option<unsafe extern "C" fn(*mut sk_buff) -> i32>,
    pub cache_invalidate: Option<unsafe extern "C" fn()>,
    pub cache_add: Option<unsafe extern "C" fn(*const u8, *const netlbl_lsm_secattr) -> i32>,
}

extern "C" {
    pub fn netlbl_calipso_ops_register(ops: *const netlbl_calipso_ops) -> *const netlbl_calipso_ops;
}

extern "C" {
    pub fn netlbl_cfg_map_del(domain: *const i8, family: u16, addr: *const core::ffi::c_void, mask: *const core::ffi::c_void, audit: *mut netlbl_audit) -> i32;
    pub fn netlbl_cfg_unlbl_map_add(domain: *const i8, family: u16, addr: *const core::ffi::c_void, mask: *const core::ffi::c_void, audit: *mut netlbl_audit) -> i32;
    pub fn netlbl_cfg_unlbl_static_add(net: *mut net, dev_name: *const i8, addr: *const core::ffi::c_void, mask: *const core::ffi::c_void, family: u16, secid: u32, audit: *mut netlbl_audit) -> i32;
    pub fn netlbl_cfg_unlbl_static_del(net: *mut net, dev_name: *const i8, addr: *const core::ffi::c_void, mask: *const core::ffi::c_void, family: u16, audit: *mut netlbl_audit) -> i32;
    pub fn netlbl_cfg_cipsov4_add(doi: *mut cipso_v4_doi, audit: *mut netlbl_audit) -> i32;
    pub fn netlbl_cfg_cipsov4_del(doi: u32, audit: *mut netlbl_audit);
    pub fn netlbl_cfg_calipso_add(doi: *mut calipso_doi, audit: *mut netlbl_audit) -> i32;
    pub fn netlbl_cfg_calipso_del(doi: u32, audit: *mut netlbl_audit);
    pub fn netlbl_catmap_walk(catmap: *mut netlbl_lsm_catmap, offset: u32) -> i32;
    pub fn netlbl_catmap_walkrng(catmap: *mut netlbl_lsm_catmap, offset: u32) -> i32;
    pub fn netlbl_catmap_setbit(catmap: *mut *mut netlbl_lsm_catmap, bit: u32, flags: gfp_t) -> i32;
    pub fn netlbl_catmap_setrng(catmap: *mut *mut netlbl_lsm_catmap, start: u32, end: u32, flags: gfp_t) -> i32;
    pub fn netlbl_enabled() -> i32;
    pub fn netlbl_sock_setattr(sk: *mut sock, family: u16, secattr: *const netlbl_lsm_secattr, sk_locked: bool) -> i32;
    pub fn netlbl_sock_delattr(sk: *mut sock);
    pub fn netlbl_sock_getattr(sk: *mut sock, secattr: *mut netlbl_lsm_secattr) -> i32;
    pub fn netlbl_req_setattr(req: *mut request_sock, secattr: *const netlbl_lsm_secattr) -> i32;
    pub fn netlbl_req_delattr(req: *mut request_sock);
    pub fn netlbl_skbuff_setattr(skb: *mut sk_buff, family: u16, secattr: *const netlbl_lsm_secattr) -> i32;
    pub fn netlbl_skbuff_getattr(skb: *const sk_buff, family: u16, secattr: *mut netlbl_lsm_secattr) -> i32;
    pub fn netlbl_cache_invalidate();
}

/* External kernel types supplied by the including environment. */
extern "C" {
    type lsm_prop; type kuid_t; type refcount_t; type calipso_doi; type cipso_v4_doi; type sock; type request_sock; type sk_buff; type net; type gfp_t;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
