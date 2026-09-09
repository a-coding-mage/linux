/* SPDX-License-Identifier: GPL-2.0-or-later */
/*
 * CIPSO - Commercial IP Security Option
 *
 * This is an implementation of the CIPSO 2.2 protocol as specified in
 * draft-ietf-cipso-ipsecurity-01.txt with additional tag types as found in
 * FIPS-188, copies of both documents can be found in the Documentation
 * directory.  While CIPSO never became a full IETF RFC standard many vendors
 * have chosen to adopt the protocol and over the years it has become a
 * de-facto standard for labeled networking.
 *
 * Author: Paul Moore <paul@paul-moore.com>
 */

/* (c) Copyright Hewlett-Packard Development Company, L.P., 2006 */

pub const CIPSO_V4_DOI_UNKNOWN: u32 = 0x00000000;
pub const CIPSO_V4_TAG_INVALID: u8 = 0;
pub const CIPSO_V4_TAG_RBITMAP: u8 = 1;
pub const CIPSO_V4_TAG_ENUM: u8 = 2;
pub const CIPSO_V4_TAG_RANGE: u8 = 5;
pub const CIPSO_V4_TAG_PBITMAP: u8 = 6;
pub const CIPSO_V4_TAG_FREEFORM: u8 = 7;
pub const CIPSO_V4_TAG_LOCAL: u8 = 128;
pub const CIPSO_V4_MAP_UNKNOWN: u32 = 0;
pub const CIPSO_V4_MAP_TRANS: u32 = 1;
pub const CIPSO_V4_MAP_PASS: u32 = 2;
pub const CIPSO_V4_MAP_LOCAL: u32 = 3;
pub const CIPSO_V4_MAX_REM_LVLS: u32 = 255;
pub const CIPSO_V4_INV_LVL: u32 = 0x80000000;
pub const CIPSO_V4_MAX_LOC_LVLS: u32 = CIPSO_V4_INV_LVL - 1;
pub const CIPSO_V4_MAX_REM_CATS: u32 = 65534;
pub const CIPSO_V4_INV_CAT: u32 = 0x80000000;
pub const CIPSO_V4_MAX_LOC_CATS: u32 = CIPSO_V4_INV_CAT - 1;

pub const CIPSO_V4_TAG_MAXCNT: usize = 5;

#[repr(C)]
pub union cipso_v4_doi_map {
    pub std: *mut cipso_v4_std_map_tbl,
}

#[repr(C)]
pub struct cipso_v4_doi {
    pub doi: u32,
    pub type_: u32,
    pub map: cipso_v4_doi_map,
    pub tags: [u8; CIPSO_V4_TAG_MAXCNT],
    pub refcount: refcount_t,
    pub list: list_head,
    pub rcu: rcu_head,
}

#[repr(C)]
pub struct cipso_v4_std_map_tbl {
    pub lvl: cipso_v4_std_map_part,
    pub cat: cipso_v4_std_map_part,
}

#[repr(C)]
pub struct cipso_v4_std_map_part {
    pub cipso: *mut u32,
    pub local: *mut u32,
    pub cipso_size: u32,
    pub local_size: u32,
}

#[cfg(feature = "CONFIG_NETLABEL")]
extern "C" {
    pub static mut cipso_v4_cache_enabled: i32;
    pub static mut cipso_v4_cache_bucketsize: i32;
    pub static mut cipso_v4_rbm_optfmt: i32;
    pub static mut cipso_v4_rbm_strictvalid: i32;

    pub fn cipso_v4_doi_add(doi_def: *mut cipso_v4_doi, audit_info: *mut netlbl_audit) -> i32;
    pub fn cipso_v4_doi_free(doi_def: *mut cipso_v4_doi);
    pub fn cipso_v4_doi_remove(doi: u32, audit_info: *mut netlbl_audit) -> i32;
    pub fn cipso_v4_doi_getdef(doi: u32) -> *mut cipso_v4_doi;
    pub fn cipso_v4_doi_putdef(doi_def: *mut cipso_v4_doi);
    pub fn cipso_v4_doi_walk(skip_cnt: *mut u32, callback: Option<unsafe extern "C" fn(*mut cipso_v4_doi, *mut core::ffi::c_void) -> i32>, cb_arg: *mut core::ffi::c_void) -> i32;

    pub fn cipso_v4_cache_invalidate();
    pub fn cipso_v4_cache_add(cipso_ptr: *const u8, secattr: *const netlbl_lsm_secattr) -> i32;
    pub fn cipso_v4_error(skb: *mut sk_buff, error: i32, gateway: u32);
    pub fn cipso_v4_getattr(cipso: *const u8, secattr: *mut netlbl_lsm_secattr) -> i32;
    pub fn cipso_v4_sock_setattr(sk: *mut sock, doi_def: *const cipso_v4_doi, secattr: *const netlbl_lsm_secattr, sk_locked: bool) -> i32;
    pub fn cipso_v4_sock_delattr(sk: *mut sock);
    pub fn cipso_v4_sock_getattr(sk: *mut sock, secattr: *mut netlbl_lsm_secattr) -> i32;
    pub fn cipso_v4_req_setattr(req: *mut request_sock, doi_def: *const cipso_v4_doi, secattr: *const netlbl_lsm_secattr) -> i32;
    pub fn cipso_v4_req_delattr(req: *mut request_sock);
    pub fn cipso_v4_skbuff_setattr(skb: *mut sk_buff, doi_def: *const cipso_v4_doi, secattr: *const netlbl_lsm_secattr) -> i32;
    pub fn cipso_v4_skbuff_delattr(skb: *mut sk_buff) -> i32;
    pub fn cipso_v4_skbuff_getattr(skb: *const sk_buff, secattr: *mut netlbl_lsm_secattr) -> i32;
    pub fn cipso_v4_optptr(skb: *const sk_buff) -> *mut u8;
    pub fn cipso_v4_validate(skb: *const sk_buff, option: *mut *mut u8) -> i32;
}

#[cfg(not(feature = "CONFIG_NETLABEL"))]
pub unsafe fn cipso_v4_doi_add(_: *mut cipso_v4_doi, _: *mut netlbl_audit) -> i32 { -38 }
#[cfg(not(feature = "CONFIG_NETLABEL"))]
pub unsafe fn cipso_v4_doi_free(_: *mut cipso_v4_doi) {}
#[cfg(not(feature = "CONFIG_NETLABEL"))]
pub unsafe fn cipso_v4_doi_remove(_: u32, _: *mut netlbl_audit) -> i32 { 0 }
#[cfg(not(feature = "CONFIG_NETLABEL"))]
pub unsafe fn cipso_v4_doi_getdef(_: u32) -> *mut cipso_v4_doi { core::ptr::null_mut() }
#[cfg(not(feature = "CONFIG_NETLABEL"))]
pub unsafe fn cipso_v4_doi_walk(_: *mut u32, _: Option<unsafe extern "C" fn(*mut cipso_v4_doi, *mut core::ffi::c_void) -> i32>, _: *mut core::ffi::c_void) -> i32 { 0 }

#[cfg(not(feature = "CONFIG_NETLABEL"))]
pub unsafe fn cipso_v4_cache_invalidate() {}
#[cfg(not(feature = "CONFIG_NETLABEL"))]
pub unsafe fn cipso_v4_cache_add(_: *const u8, _: *const netlbl_lsm_secattr) -> i32 { 0 }
#[cfg(not(feature = "CONFIG_NETLABEL"))]
pub unsafe fn cipso_v4_error(_: *mut sk_buff, _: i32, _: u32) {}
#[cfg(not(feature = "CONFIG_NETLABEL"))]
pub unsafe fn cipso_v4_getattr(_: *const u8, _: *mut netlbl_lsm_secattr) -> i32 { -38 }
#[cfg(not(feature = "CONFIG_NETLABEL"))]
pub unsafe fn cipso_v4_sock_setattr(_: *mut sock, _: *const cipso_v4_doi, _: *const netlbl_lsm_secattr, _: bool) -> i32 { -38 }
#[cfg(not(feature = "CONFIG_NETLABEL"))]
pub unsafe fn cipso_v4_sock_delattr(_: *mut sock) {}
#[cfg(not(feature = "CONFIG_NETLABEL"))]
pub unsafe fn cipso_v4_sock_getattr(_: *mut sock, _: *mut netlbl_lsm_secattr) -> i32 { -38 }
#[cfg(not(feature = "CONFIG_NETLABEL"))]
pub unsafe fn cipso_v4_req_setattr(_: *mut request_sock, _: *const cipso_v4_doi, _: *const netlbl_lsm_secattr) -> i32 { -38 }
#[cfg(not(feature = "CONFIG_NETLABEL"))]
pub unsafe fn cipso_v4_req_delattr(_: *mut request_sock) {}
#[cfg(not(feature = "CONFIG_NETLABEL"))]
pub unsafe fn cipso_v4_skbuff_setattr(_: *mut sk_buff, _: *const cipso_v4_doi, _: *const netlbl_lsm_secattr) -> i32 { -38 }
#[cfg(not(feature = "CONFIG_NETLABEL"))]
pub unsafe fn cipso_v4_skbuff_delattr(_: *mut sk_buff) -> i32 { -38 }
#[cfg(not(feature = "CONFIG_NETLABEL"))]
pub unsafe fn cipso_v4_skbuff_getattr(_: *const sk_buff, _: *mut netlbl_lsm_secattr) -> i32 { -38 }
#[cfg(not(feature = "CONFIG_NETLABEL"))]
pub unsafe fn cipso_v4_optptr(_: *const sk_buff) -> *mut u8 { core::ptr::null_mut() }
#[cfg(not(feature = "CONFIG_NETLABEL"))]
pub unsafe fn cipso_v4_validate(_: *const sk_buff, option: *mut *mut u8) -> i32 {
    let opt = *option;
    let mut err_offset: u8 = 0;
    let opt_len = *opt.add(1);
    if opt_len < 8 { err_offset = 1; }
    else if u32::from_be_bytes([*opt.add(2), *opt.add(3), *opt.add(4), *opt.add(5)]) == 0 { err_offset = 2; }
    else {
        let mut opt_iter: u8 = 6;
        while opt_iter < opt_len {
            if opt_iter + 1 == opt_len { err_offset = opt_iter; break; }
            let tag_len = *opt.add((opt_iter + 1) as usize);
            if tag_len == 0 || tag_len > opt_len - opt_iter { err_offset = opt_iter + 1; break; }
            opt_iter = opt_iter.wrapping_add(tag_len);
        }
    }
    *option = opt.add(err_offset as usize);
    err_offset as i32
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
