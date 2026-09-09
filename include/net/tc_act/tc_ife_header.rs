/* SPDX-License-Identifier: GPL-2.0 */

// C dependencies supplied by other headers:
// net/act_api.h, linux/etherdevice.h, linux/rtnetlink.h

use core::ffi::c_char;

pub const ETH_ALEN: usize = 6;

#[repr(C)]
pub struct module {
    _private: [u8; 0],
}

#[repr(C)]
pub struct list_head {
    pub next: *mut list_head,
    pub prev: *mut list_head,
}

#[repr(C)]
pub struct rcu_head {
    pub next: *mut rcu_head,
    pub func: Option<unsafe extern "C" fn(*mut rcu_head)>,
}

#[repr(C)]
pub struct sk_buff {
    _private: [u8; 0],
}

#[repr(C)]
pub struct tc_action {
    _private: [u8; 0],
}

pub type gfp_t = usize;

#[repr(C)]
pub struct tcf_ife_params {
    pub eth_dst: [u8; ETH_ALEN],
    pub eth_src: [u8; ETH_ALEN],
    pub eth_type: u16,
    pub flags: u16,
    pub metalist: list_head,
    pub rcu: rcu_head,
}

#[repr(C)]
pub struct tcf_ife_info {
    pub common: tc_action,
    // __rcu annotation from C; access remains raw-pointer/unsafe as required.
    pub params: *mut tcf_ife_params,
}

#[inline]
pub unsafe fn to_ife(a: *mut tc_action) -> *mut tcf_ife_info {
    a as *mut tcf_ife_info
}

#[repr(C)]
pub struct tcf_meta_info {
    pub ops: *const tcf_meta_ops,
    pub metaval: *mut core::ffi::c_void,
    pub metaid: u16,
    pub metalist: list_head,
}

#[repr(C)]
pub struct tcf_meta_ops {
    pub metaid: u16, // Maintainer provided ID
    pub metatype: u16, // netlink attribute type (look at net/netlink.h)
    pub name: *const c_char,
    pub synopsis: *const c_char,
    pub list: list_head,
    pub check_presence:
        Option<unsafe extern "C" fn(*mut sk_buff, *mut tcf_meta_info) -> i32>,
    pub encode: Option<unsafe extern "C" fn(
        *mut sk_buff,
        *mut core::ffi::c_void,
        *mut tcf_meta_info,
    ) -> i32>,
    pub decode: Option<unsafe extern "C" fn(*mut sk_buff, *mut core::ffi::c_void, u16) -> i32>,
    pub get: Option<unsafe extern "C" fn(*mut sk_buff, *mut tcf_meta_info) -> i32>,
    pub alloc: Option<unsafe extern "C" fn(
        *mut tcf_meta_info,
        *mut core::ffi::c_void,
        gfp_t,
    ) -> i32>,
    pub release: Option<unsafe extern "C" fn(*mut tcf_meta_info)>,
    pub validate: Option<unsafe extern "C" fn(*mut core::ffi::c_void, i32) -> i32>,
    pub owner: *mut module,
}

// MODULE_ALIAS_IFE_META(metan) expands to MODULE_ALIAS("ife-meta-" metan).

extern "C" {
    pub fn ife_get_meta_u32(skb: *mut sk_buff, mi: *mut tcf_meta_info) -> i32;
    pub fn ife_get_meta_u16(skb: *mut sk_buff, mi: *mut tcf_meta_info) -> i32;
    pub fn ife_alloc_meta_u32(
        mi: *mut tcf_meta_info,
        metaval: *mut core::ffi::c_void,
        gfp: gfp_t,
    ) -> i32;
    pub fn ife_alloc_meta_u16(
        mi: *mut tcf_meta_info,
        metaval: *mut core::ffi::c_void,
        gfp: gfp_t,
    ) -> i32;
    pub fn ife_check_meta_u32(metaval: u32, mi: *mut tcf_meta_info) -> i32;
    pub fn ife_check_meta_u16(metaval: u16, mi: *mut tcf_meta_info) -> i32;
    pub fn ife_encode_meta_u32(
        metaval: u32,
        skbdata: *mut core::ffi::c_void,
        mi: *mut tcf_meta_info,
    ) -> i32;
    pub fn ife_validate_meta_u32(val: *mut core::ffi::c_void, len: i32) -> i32;
    pub fn ife_validate_meta_u16(val: *mut core::ffi::c_void, len: i32) -> i32;
    pub fn ife_encode_meta_u16(
        metaval: u16,
        skbdata: *mut core::ffi::c_void,
        mi: *mut tcf_meta_info,
    ) -> i32;
    pub fn ife_release_meta_gen(mi: *mut tcf_meta_info);
    pub fn register_ife_op(mops: *mut tcf_meta_ops) -> i32;
    pub fn unregister_ife_op(mops: *mut tcf_meta_ops) -> i32;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
