/* SPDX-License-Identifier: GPL-2.0 OR BSD-3-Clause */

/*
 * Rust translation of net/ynl/lib/ynl-priv.h.
 *
 * C header guard and include directives are intentionally omitted.  This file
 * expects the Rust translations/bindings for the C include dependencies
 * (<stdbool.h>, <stddef.h>, <linux/types.h>, netlink headers, and libc memory
 * helpers) to be supplied by the surrounding crate.
 */

use core::ffi::{c_char, c_int, c_uint, c_void};
use core::mem::{offset_of, size_of};
use core::ptr;

#[repr(C)]
pub struct ynl_sock {
    _private: [u8; 0],
}

pub type __s8 = i8;
pub type __s16 = i16;
pub type __s32 = i32;
pub type __s64 = i64;
pub type __u8 = u8;
pub type __u16 = u16;
pub type __u32 = u32;
pub type __u64 = u64;

#[repr(C)]
pub struct nlmsghdr {
    pub nlmsg_len: __u32,
    pub nlmsg_type: __u16,
    pub nlmsg_flags: __u16,
    pub nlmsg_seq: __u32,
    pub nlmsg_pid: __u32,
}

#[repr(C)]
pub struct nlattr {
    pub nla_len: __u16,
    pub nla_type: __u16,
}

unsafe extern "C" {
    pub static mut YNL_LIST_END: *mut ynl_dump_list_type;

    pub static NLMSG_HDRLEN: c_uint;
    pub static NLA_HDRLEN: c_uint;
    pub static NLA_TYPE_MASK: c_uint;
    pub static NLA_F_NESTED: c_uint;

    pub fn NLMSG_ALIGN(len: c_uint) -> c_uint;
    pub fn NLA_ALIGN(len: c_uint) -> c_uint;

    pub fn memset(s: *mut c_void, c: c_int, n: usize) -> *mut c_void;
    pub fn memcpy(dest: *mut c_void, src: *const c_void, n: usize) -> *mut c_void;
    pub fn strlen(s: *const c_char) -> usize;
    pub fn strcpy(dest: *mut c_char, src: *const c_char) -> *mut c_char;
}

/*
 * YNL internals / low level stuff
 */

#[repr(C)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum ynl_policy_type {
    YNL_PT_REJECT = 1,
    YNL_PT_IGNORE = 2,
    YNL_PT_NEST = 3,
    YNL_PT_FLAG = 4,
    YNL_PT_BINARY = 5,
    YNL_PT_U8 = 6,
    YNL_PT_U16 = 7,
    YNL_PT_U32 = 8,
    YNL_PT_U64 = 9,
    YNL_PT_UINT = 10,
    YNL_PT_NUL_STR = 11,
    YNL_PT_BITFIELD32 = 12,
    YNL_PT_SUBMSG = 13,
}

#[repr(C)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum ynl_parse_result {
    YNL_PARSE_CB_ERROR = -1,
    YNL_PARSE_CB_STOP = 0,
    YNL_PARSE_CB_OK = 1,
}

pub const YNL_SOCKET_BUFFER_SIZE: usize = 1 << 17;

#[macro_export]
macro_rules! YNL_ARRAY_SIZE {
    ($array:expr) => {{
        if ::core::mem::size_of_val(&$array) != 0 {
            ::core::mem::size_of_val(&$array) / ::core::mem::size_of_val(&$array[0])
        } else {
            0
        }
    }};
}

pub type ynl_parse_cb_t =
    Option<unsafe extern "C" fn(nlh: *const nlmsghdr, yarg: *mut ynl_parse_arg) -> c_int>;

#[repr(C)]
pub struct ynl_policy_attr {
    /* C bitfields: enum ynl_policy_type type:8; __u8 is_submsg:1; __u8 is_selector:1; */
    pub type_is_submsg_is_selector: __u16,
    pub selector_type: __u16,
    pub len: c_uint,
    pub name: *const c_char,
    pub nest: *const ynl_policy_nest,
}

impl ynl_policy_attr {
    pub const TYPE_MASK: __u16 = 0x00ff;
    pub const IS_SUBMSG_MASK: __u16 = 0x0100;
    pub const IS_SELECTOR_MASK: __u16 = 0x0200;
}

#[repr(C)]
pub struct ynl_policy_nest {
    pub max_attr: c_uint,
    pub table: *const ynl_policy_attr,
}

#[repr(C)]
pub struct ynl_parse_arg {
    pub ys: *mut ynl_sock,
    pub rsp_policy: *const ynl_policy_nest,
    pub data: *mut c_void,
}

#[repr(C)]
pub struct ynl_dump_list_type {
    pub next: *mut ynl_dump_list_type,
    pub data: [u64; 0],
}

#[inline]
pub unsafe fn ynl_dump_obj_is_last(obj: *mut c_void) -> bool {
    let mut uptr = obj as usize;

    uptr = uptr.wrapping_sub(offset_of!(ynl_dump_list_type, data));
    uptr == unsafe { YNL_LIST_END } as usize
}

#[inline]
pub unsafe fn ynl_dump_obj_next(obj: *mut c_void) -> *mut c_void {
    let mut uptr = obj as usize;
    let list: *mut ynl_dump_list_type;

    uptr = uptr.wrapping_sub(offset_of!(ynl_dump_list_type, data));
    list = uptr as *mut ynl_dump_list_type;
    uptr = unsafe { (*list).next } as usize;
    uptr = uptr.wrapping_add(offset_of!(ynl_dump_list_type, data));

    uptr as *mut c_void
}

#[repr(C)]
pub struct ynl_ntf_base_type {
    pub family: __u16,
    pub cmd: __u8,
    pub next: *mut ynl_ntf_base_type,
    pub free: Option<unsafe extern "C" fn(ntf: *mut ynl_ntf_base_type)>,
    pub data: [u64; 0],
}

unsafe extern "C" {
    pub fn ynl_msg_start_req(ys: *mut ynl_sock, id: __u32, flags: __u16) -> *mut nlmsghdr;
    pub fn ynl_msg_start_dump(ys: *mut ynl_sock, id: __u32) -> *mut nlmsghdr;

    pub fn ynl_gemsg_start_req(
        ys: *mut ynl_sock,
        id: __u32,
        cmd: __u8,
        version: __u8,
    ) -> *mut nlmsghdr;
    pub fn ynl_gemsg_start_dump(
        ys: *mut ynl_sock,
        id: __u32,
        cmd: __u8,
        version: __u8,
    ) -> *mut nlmsghdr;

    pub fn ynl_submsg_failed(
        yarg: *mut ynl_parse_arg,
        field_name: *const c_char,
        sel_name: *const c_char,
    ) -> c_int;
}

/* YNL specific helpers used by the auto-generated code */

#[repr(C)]
pub struct ynl_req_state {
    pub yarg: ynl_parse_arg,
    pub cb: ynl_parse_cb_t,
    pub rsp_cmd: __u32,
}

#[repr(C)]
pub struct ynl_dump_state {
    pub yarg: ynl_parse_arg,
    pub first: *mut c_void,
    pub last: *mut ynl_dump_list_type,
    pub alloc_sz: usize,
    pub cb: ynl_parse_cb_t,
    pub rsp_cmd: __u32,
}

#[repr(C)]
pub struct ynl_ntf_info {
    pub policy: *const ynl_policy_nest,
    pub cb: ynl_parse_cb_t,
    pub alloc_sz: usize,
    pub free: Option<unsafe extern "C" fn(ntf: *mut ynl_ntf_base_type)>,
}

unsafe extern "C" {
    pub fn ynl_exec(
        ys: *mut ynl_sock,
        req_nlh: *mut nlmsghdr,
        yrs: *mut ynl_req_state,
    ) -> c_int;
    pub fn ynl_exec_dump(
        ys: *mut ynl_sock,
        req_nlh: *mut nlmsghdr,
        yds: *mut ynl_dump_state,
    ) -> c_int;

    pub fn ynl_error_unknown_notification(ys: *mut ynl_sock, cmd: __u8);
    pub fn ynl_error_parse(yarg: *mut ynl_parse_arg, msg: *const c_char) -> c_int;
}

/* Netlink message handling helpers */

pub const YNL_MSG_OVERFLOW: __u32 = 1;

#[inline]
pub unsafe fn ynl_nlmsg_put_header(buf: *mut c_void) -> *mut nlmsghdr {
    let nlh = buf as *mut nlmsghdr;

    unsafe {
        memset(nlh as *mut c_void, 0, size_of::<nlmsghdr>());
        (*nlh).nlmsg_len = NLMSG_HDRLEN;
    }

    nlh
}

#[inline]
pub unsafe fn ynl_nlmsg_data_len(nlh: *const nlmsghdr) -> c_uint {
    unsafe { (*nlh).nlmsg_len.wrapping_sub(NLMSG_HDRLEN) }
}

#[inline]
pub unsafe fn ynl_nlmsg_data(nlh: *const nlmsghdr) -> *mut c_void {
    unsafe { (nlh as *mut u8).add(NLMSG_HDRLEN as usize) as *mut c_void }
}

#[inline]
pub unsafe fn ynl_nlmsg_data_offset(nlh: *const nlmsghdr, offset: c_uint) -> *mut c_void {
    unsafe { (nlh as *mut u8).add((NLMSG_HDRLEN + offset) as usize) as *mut c_void }
}

#[inline]
pub unsafe fn ynl_nlmsg_end_addr(nlh: *const nlmsghdr) -> *mut c_void {
    unsafe { (nlh as *mut c_char).add((*nlh).nlmsg_len as usize) as *mut c_void }
}

#[inline]
pub unsafe fn ynl_nlmsg_put_extra_header(nlh: *mut nlmsghdr, size: c_uint) -> *mut c_void {
    let tail = unsafe { ynl_nlmsg_end_addr(nlh) };

    unsafe {
        (*nlh).nlmsg_len = (*nlh).nlmsg_len.wrapping_add(NLMSG_ALIGN(size));
    }
    tail
}

/* Netlink attribute helpers */

#[inline]
pub unsafe fn ynl_attr_type(attr: *const nlattr) -> c_uint {
    unsafe { ((*attr).nla_type as c_uint) & NLA_TYPE_MASK }
}

#[inline]
pub unsafe fn ynl_attr_data_len(attr: *const nlattr) -> c_uint {
    unsafe { ((*attr).nla_len as c_uint).wrapping_sub(NLA_HDRLEN) }
}

#[inline]
pub unsafe fn ynl_attr_data(attr: *const nlattr) -> *mut c_void {
    unsafe { (attr as *mut u8).add(NLA_HDRLEN as usize) as *mut c_void }
}

#[inline]
pub unsafe fn ynl_attr_data_end(attr: *const nlattr) -> *mut c_void {
    unsafe { (ynl_attr_data(attr) as *mut c_char).add(ynl_attr_data_len(attr) as usize) as *mut c_void }
}

#[macro_export]
macro_rules! ynl_attr_for_each {
    ($attr:ident, $nlh:expr, $fixed_hdr_sz:expr, $body:block) => {{
        $attr = unsafe { ynl_attr_first($nlh as *const _, unsafe { (*$nlh).nlmsg_len } as usize, (unsafe { NLMSG_HDRLEN } + $fixed_hdr_sz) as usize) };
        while !$attr.is_null() {
            $body
            $attr = unsafe { ynl_attr_next(ynl_nlmsg_end_addr($nlh as *const _), $attr as *const _) };
        }
    }};
}

#[macro_export]
macro_rules! ynl_attr_for_each_nested_off {
    ($attr:ident, $outer:expr, $offset:expr, $body:block) => {{
        $attr = unsafe {
            ynl_attr_first(
                $outer as *const _ as *const ::core::ffi::c_void,
                unsafe { (*$outer).nla_len } as usize,
                ::core::mem::size_of::<nlattr>() + $offset,
            )
        };
        while !$attr.is_null() {
            $body
            $attr = unsafe { ynl_attr_next(ynl_attr_data_end($outer as *const _), $attr as *const _) };
        }
    }};
}

#[macro_export]
macro_rules! ynl_attr_for_each_nested {
    ($attr:ident, $outer:expr, $body:block) => {
        ynl_attr_for_each_nested_off!($attr, $outer, 0usize, $body)
    };
}

#[macro_export]
macro_rules! ynl_attr_for_each_payload {
    ($start:expr, $len:expr, $attr:ident, $body:block) => {{
        $attr = unsafe { ynl_attr_first($start as *const _, $len as usize, 0) };
        while !$attr.is_null() {
            $body
            $attr = unsafe {
                ynl_attr_next(
                    unsafe { ($start as *const u8).add($len as usize) } as *const ::core::ffi::c_void,
                    $attr as *const _,
                )
            };
        }
    }};
}

#[inline]
pub unsafe fn ynl_attr_if_good(end: *const c_void, attr: *mut nlattr) -> *mut nlattr {
    unsafe {
        if attr.add(1) > end as *mut nlattr {
            return ptr::null_mut();
        }
        if ynl_attr_data_end(attr) > end as *mut c_void {
            return ptr::null_mut();
        }
    }
    attr
}

#[inline]
pub unsafe fn ynl_attr_next(end: *const c_void, prev: *const nlattr) -> *mut nlattr {
    let attr: *mut nlattr;

    unsafe {
        attr = (prev as *mut c_char).add(NLA_ALIGN((*prev).nla_len as c_uint) as usize) as *mut nlattr;
        ynl_attr_if_good(end, attr)
    }
}

#[inline]
pub unsafe fn ynl_attr_first(start: *const c_void, len: usize, skip: usize) -> *mut nlattr {
    let attr: *mut nlattr;

    unsafe {
        attr = (start as *mut c_char).add(NLMSG_ALIGN(skip as c_uint) as usize) as *mut nlattr;
        ynl_attr_if_good((start as *const c_char).add(len) as *const c_void, attr)
    }
}

#[inline]
pub unsafe fn __ynl_attr_put_overflow(nlh: *mut nlmsghdr, size: usize) -> bool {
    let o: bool;

    unsafe {
        /* ynl_msg_start() stashed buffer length in nlmsg_pid. */
        o = ((*nlh).nlmsg_len as usize)
            .wrapping_add(NLA_HDRLEN as usize)
            .wrapping_add(NLMSG_ALIGN(size as c_uint) as usize)
            > (*nlh).nlmsg_pid as usize;
        if o {
            /* YNL_MSG_OVERFLOW is < NLMSG_HDRLEN, all subsequent checks
             * are guaranteed to fail.
             */
            (*nlh).nlmsg_pid = YNL_MSG_OVERFLOW;
        }
    }
    o
}

#[inline]
pub unsafe fn ynl_attr_nest_start(nlh: *mut nlmsghdr, attr_type: c_uint) -> *mut nlattr {
    let attr: *mut nlattr;

    unsafe {
        if __ynl_attr_put_overflow(nlh, 0) {
            return (ynl_nlmsg_end_addr(nlh) as *mut nlattr).sub(1);
        }

        attr = ynl_nlmsg_end_addr(nlh) as *mut nlattr;
        (*attr).nla_type = (attr_type | NLA_F_NESTED) as __u16;
        (*nlh).nlmsg_len = (*nlh).nlmsg_len.wrapping_add(NLA_HDRLEN);
    }

    attr
}

#[inline]
pub unsafe fn ynl_attr_nest_end(nlh: *mut nlmsghdr, attr: *mut nlattr) {
    unsafe {
        (*attr).nla_len = (ynl_nlmsg_end_addr(nlh) as *mut c_char).offset_from(attr as *mut c_char) as __u16;
    }
}

#[inline]
pub unsafe fn ynl_attr_put(
    nlh: *mut nlmsghdr,
    attr_type: c_uint,
    value: *const c_void,
    size: usize,
) {
    let attr: *mut nlattr;

    unsafe {
        if __ynl_attr_put_overflow(nlh, size) {
            return;
        }

        attr = ynl_nlmsg_end_addr(nlh) as *mut nlattr;
        (*attr).nla_type = attr_type as __u16;
        (*attr).nla_len = (NLA_HDRLEN as usize).wrapping_add(size) as __u16;

        memcpy(ynl_attr_data(attr), value, size);

        (*nlh).nlmsg_len = (*nlh).nlmsg_len.wrapping_add(NLMSG_ALIGN((*attr).nla_len as c_uint));
    }
}

#[inline]
pub unsafe fn ynl_attr_put_str(nlh: *mut nlmsghdr, attr_type: c_uint, str_: *const c_char) {
    let attr: *mut nlattr;
    let len: usize;

    unsafe {
        len = strlen(str_).wrapping_add(1);
        if __ynl_attr_put_overflow(nlh, len) {
            return;
        }

        attr = ynl_nlmsg_end_addr(nlh) as *mut nlattr;
        (*attr).nla_type = attr_type as __u16;

        strcpy(ynl_attr_data(attr) as *mut c_char, str_);
        (*attr).nla_len = (NLA_HDRLEN as usize).wrapping_add(len) as __u16;

        (*nlh).nlmsg_len = (*nlh).nlmsg_len.wrapping_add(NLMSG_ALIGN((*attr).nla_len as c_uint));
    }
}

#[inline]
pub unsafe fn ynl_attr_get_str(attr: *const nlattr) -> *const c_char {
    unsafe { ynl_attr_data(attr) as *const c_char }
}

#[inline]
pub unsafe fn ynl_attr_get_s8(attr: *const nlattr) -> __s8 {
    unsafe { *(ynl_attr_data(attr) as *const __s8) }
}

#[inline]
pub unsafe fn ynl_attr_get_s16(attr: *const nlattr) -> __s16 {
    unsafe { *(ynl_attr_data(attr) as *const __s16) }
}

#[inline]
pub unsafe fn ynl_attr_get_s32(attr: *const nlattr) -> __s32 {
    unsafe { *(ynl_attr_data(attr) as *const __s32) }
}

#[inline]
pub unsafe fn ynl_attr_get_s64(attr: *const nlattr) -> __s64 {
    let mut tmp: __s64 = 0;

    unsafe {
        memcpy(
            &mut tmp as *mut __s64 as *mut c_void,
            (attr.add(1)) as *const u8 as *const c_void,
            size_of::<__s64>(),
        );
    }
    tmp
}

#[inline]
pub unsafe fn ynl_attr_get_u8(attr: *const nlattr) -> __u8 {
    unsafe { *(ynl_attr_data(attr) as *const __u8) }
}

#[inline]
pub unsafe fn ynl_attr_get_u16(attr: *const nlattr) -> __u16 {
    unsafe { *(ynl_attr_data(attr) as *const __u16) }
}

#[inline]
pub unsafe fn ynl_attr_get_u32(attr: *const nlattr) -> __u32 {
    unsafe { *(ynl_attr_data(attr) as *const __u32) }
}

#[inline]
pub unsafe fn ynl_attr_get_u64(attr: *const nlattr) -> __u64 {
    let mut tmp: __u64 = 0;

    unsafe {
        memcpy(
            &mut tmp as *mut __u64 as *mut c_void,
            (attr.add(1)) as *const u8 as *const c_void,
            size_of::<__u64>(),
        );
    }
    tmp
}

#[inline]
pub unsafe fn ynl_attr_put_s8(nlh: *mut nlmsghdr, attr_type: c_uint, value: __s8) {
    unsafe { ynl_attr_put(nlh, attr_type, &value as *const __s8 as *const c_void, size_of::<__s8>()) }
}

#[inline]
pub unsafe fn ynl_attr_put_s16(nlh: *mut nlmsghdr, attr_type: c_uint, value: __s16) {
    unsafe { ynl_attr_put(nlh, attr_type, &value as *const __s16 as *const c_void, size_of::<__s16>()) }
}

#[inline]
pub unsafe fn ynl_attr_put_s32(nlh: *mut nlmsghdr, attr_type: c_uint, value: __s32) {
    unsafe { ynl_attr_put(nlh, attr_type, &value as *const __s32 as *const c_void, size_of::<__s32>()) }
}

#[inline]
pub unsafe fn ynl_attr_put_s64(nlh: *mut nlmsghdr, attr_type: c_uint, value: __s64) {
    unsafe { ynl_attr_put(nlh, attr_type, &value as *const __s64 as *const c_void, size_of::<__s64>()) }
}

#[inline]
pub unsafe fn ynl_attr_put_u8(nlh: *mut nlmsghdr, attr_type: c_uint, value: __u8) {
    unsafe { ynl_attr_put(nlh, attr_type, &value as *const __u8 as *const c_void, size_of::<__u8>()) }
}

#[inline]
pub unsafe fn ynl_attr_put_u16(nlh: *mut nlmsghdr, attr_type: c_uint, value: __u16) {
    unsafe { ynl_attr_put(nlh, attr_type, &value as *const __u16 as *const c_void, size_of::<__u16>()) }
}

#[inline]
pub unsafe fn ynl_attr_put_u32(nlh: *mut nlmsghdr, attr_type: c_uint, value: __u32) {
    unsafe { ynl_attr_put(nlh, attr_type, &value as *const __u32 as *const c_void, size_of::<__u32>()) }
}

#[inline]
pub unsafe fn ynl_attr_put_u64(nlh: *mut nlmsghdr, attr_type: c_uint, value: __u64) {
    unsafe { ynl_attr_put(nlh, attr_type, &value as *const __u64 as *const c_void, size_of::<__u64>()) }
}

#[inline]
pub unsafe fn ynl_attr_get_uint(attr: *const nlattr) -> __u64 {
    unsafe {
        match ynl_attr_data_len(attr) {
            4 => ynl_attr_get_u32(attr) as __u64,
            8 => ynl_attr_get_u64(attr),
            _ => 0,
        }
    }
}

#[inline]
pub unsafe fn ynl_attr_get_sint(attr: *const nlattr) -> __s64 {
    unsafe {
        match ynl_attr_data_len(attr) {
            4 => ynl_attr_get_s32(attr) as __s64,
            8 => ynl_attr_get_s64(attr),
            _ => 0,
        }
    }
}

#[inline]
pub unsafe fn ynl_attr_put_uint(nlh: *mut nlmsghdr, type_: __u16, data: __u64) {
    unsafe {
        if (data as __u32) as __u64 == data {
            ynl_attr_put_u32(nlh, type_ as c_uint, data as __u32);
        } else {
            ynl_attr_put_u64(nlh, type_ as c_uint, data);
        }
    }
}

#[inline]
pub unsafe fn ynl_attr_put_sint(nlh: *mut nlmsghdr, type_: __u16, data: __s64) {
    unsafe {
        if (data as __s32) as __s64 == data {
            ynl_attr_put_s32(nlh, type_ as c_uint, data as __s32);
        } else {
            ynl_attr_put_s64(nlh, type_ as c_uint, data);
        }
    }
}

unsafe extern "C" {
    pub fn __ynl_attr_validate(
        yarg: *mut ynl_parse_arg,
        attr: *const nlattr,
        type_: c_uint,
    ) -> c_int;
}

#[inline]
pub unsafe fn ynl_attr_validate(yarg: *mut ynl_parse_arg, attr: *const nlattr) -> c_int {
    unsafe { __ynl_attr_validate(yarg, attr, ynl_attr_type(attr)) }
}
