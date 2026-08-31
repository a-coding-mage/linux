/* SPDX-License-Identifier: (LGPL-2.1 OR BSD-2-Clause) */

/*
 * NETLINK      Netlink attributes
 *
 * Copyright (c) 2003-2013 Thomas Graf <tgraf@suug.ch>
 */

/* C header guard and include directives omitted.
 * Dependencies expected from the original includes:
 * stdint.h, string.h, errno.h, linux/netlink.h, linux/rtnetlink.h,
 * linux/genetlink.h.
 */

/* avoid multiple definition of netlink features */
pub const __LINUX_NETLINK_H: bool = true;

/**
 * Standard attribute types to specify validation policy
 */
pub const LIBBPF_NLA_UNSPEC: u32 = 0; /* Unspecified type, binary data chunk */
pub const LIBBPF_NLA_U8: u32 = 1; /* 8 bit integer */
pub const LIBBPF_NLA_U16: u32 = 2; /* 16 bit integer */
pub const LIBBPF_NLA_U32: u32 = 3; /* 32 bit integer */
pub const LIBBPF_NLA_U64: u32 = 4; /* 64 bit integer */
pub const LIBBPF_NLA_STRING: u32 = 5; /* NUL terminated character string */
pub const LIBBPF_NLA_FLAG: u32 = 6; /* Flag */
pub const LIBBPF_NLA_MSECS: u32 = 7; /* Micro seconds (64bit) */
pub const LIBBPF_NLA_NESTED: u32 = 8; /* Nested attributes */
pub const __LIBBPF_NLA_TYPE_MAX: u32 = 9;

pub const LIBBPF_NLA_TYPE_MAX: u32 = __LIBBPF_NLA_TYPE_MAX - 1;

/**
 * @ingroup attr
 * Attribute validation policy.
 *
 * See section @core_doc{core_attr_parse,Attribute Parsing} for more details.
 */
#[repr(C)]
pub struct libbpf_nla_policy {
    /** Type of attribute or LIBBPF_NLA_UNSPEC */
    pub type_: u16,

    /** Minimal length of payload required */
    pub minlen: u16,

    /** Maximal length of payload allowed */
    pub maxlen: u16,
}

#[repr(C)]
pub union libbpf_nla_req_union {
    pub ifinfo: ifinfomsg,
    pub tc: tcmsg,
    pub gnl: genlmsghdr,
}

#[repr(C)]
pub struct libbpf_nla_req {
    pub nh: nlmsghdr,
    pub u: libbpf_nla_req_union,
    pub buf: [::std::os::raw::c_char; 128],
}

/**
 * @ingroup attr
 * Iterate over a stream of attributes
 * @arg pos loop counter, set to current attribute
 * @arg head head of attribute stream
 * @arg len length of attribute stream
 * @arg rem initialized to len, holds bytes currently remaining in stream
 *
 * Original C macro:
 * libbpf_nla_for_each_attr(pos, head, len, rem)
 *     for (pos = head, rem = len; nla_ok(pos, rem); pos = nla_next(pos, &(rem)))
 */

/**
 * libbpf_nla_data - head of payload
 * @nla: netlink attribute
 */
#[inline]
pub unsafe fn libbpf_nla_data(nla: *const nlattr) -> *mut ::std::os::raw::c_void {
    (nla as *mut u8).add(NLA_HDRLEN as usize) as *mut ::std::os::raw::c_void
}

#[inline]
pub unsafe fn libbpf_nla_getattr_u8(nla: *const nlattr) -> u8 {
    *(libbpf_nla_data(nla) as *mut u8)
}

#[inline]
pub unsafe fn libbpf_nla_getattr_u16(nla: *const nlattr) -> u16 {
    *(libbpf_nla_data(nla) as *mut u16)
}

#[inline]
pub unsafe fn libbpf_nla_getattr_u32(nla: *const nlattr) -> u32 {
    *(libbpf_nla_data(nla) as *mut u32)
}

#[inline]
pub unsafe fn libbpf_nla_getattr_u64(nla: *const nlattr) -> u64 {
    *(libbpf_nla_data(nla) as *mut u64)
}

#[inline]
pub unsafe fn libbpf_nla_getattr_str(nla: *const nlattr) -> *const ::std::os::raw::c_char {
    libbpf_nla_data(nla) as *const ::std::os::raw::c_char
}

/**
 * libbpf_nla_len - length of payload
 * @nla: netlink attribute
 */
#[inline]
pub unsafe fn libbpf_nla_len(nla: *const nlattr) -> ::std::os::raw::c_int {
    ((*nla).nla_len as ::std::os::raw::c_int) - (NLA_HDRLEN as ::std::os::raw::c_int)
}

unsafe extern "C" {
    pub fn libbpf_nla_parse(
        tb: *mut *mut nlattr,
        maxtype: ::std::os::raw::c_int,
        head: *mut nlattr,
        len: ::std::os::raw::c_int,
        policy: *mut libbpf_nla_policy,
    ) -> ::std::os::raw::c_int;
    pub fn libbpf_nla_parse_nested(
        tb: *mut *mut nlattr,
        maxtype: ::std::os::raw::c_int,
        nla: *mut nlattr,
        policy: *mut libbpf_nla_policy,
    ) -> ::std::os::raw::c_int;

    pub fn libbpf_nla_dump_errormsg(nlh: *mut nlmsghdr) -> ::std::os::raw::c_int;
}

#[inline]
pub unsafe fn nla_data(nla: *mut nlattr) -> *mut nlattr {
    (nla as *mut u8).add(NLA_HDRLEN as usize) as *mut nlattr
}

#[inline]
pub unsafe fn req_tail(req: *mut libbpf_nla_req) -> *mut nlattr {
    (req as *mut u8).add(NLMSG_ALIGN((*req).nh.nlmsg_len) as usize) as *mut nlattr
}

#[inline]
pub unsafe fn nlattr_add(
    req: *mut libbpf_nla_req,
    type_: ::std::os::raw::c_int,
    data: *const ::std::os::raw::c_void,
    len: ::std::os::raw::c_int,
) -> ::std::os::raw::c_int {
    let nla: *mut nlattr;

    if NLMSG_ALIGN((*req).nh.nlmsg_len)
        + NLA_ALIGN((NLA_HDRLEN as ::std::os::raw::c_int + len) as u32)
        > ::std::mem::size_of::<libbpf_nla_req>() as u32
    {
        return -EMSGSIZE;
    }
    if (!data.is_null()) != (len != 0) {
        return -EINVAL;
    }

    nla = req_tail(req);
    (*nla).nla_type = type_ as _;
    (*nla).nla_len = (NLA_HDRLEN as ::std::os::raw::c_int + len) as _;
    if !data.is_null() {
        ::std::ptr::copy_nonoverlapping(
            data as *const u8,
            nla_data(nla) as *mut u8,
            len as usize,
        );
    }
    (*req).nh.nlmsg_len =
        NLMSG_ALIGN((*req).nh.nlmsg_len) + NLA_ALIGN((*nla).nla_len as u32);
    0
}

#[inline]
pub unsafe fn nlattr_begin_nested(
    req: *mut libbpf_nla_req,
    type_: ::std::os::raw::c_int,
) -> *mut nlattr {
    let tail: *mut nlattr;

    tail = req_tail(req);
    if nlattr_add(req, type_ | NLA_F_NESTED, ::std::ptr::null(), 0) != 0 {
        return ::std::ptr::null_mut();
    }
    tail
}

#[inline]
pub unsafe fn nlattr_end_nested(req: *mut libbpf_nla_req, tail: *mut nlattr) {
    (*tail).nla_len = (req_tail(req) as *mut u8).offset_from(tail as *mut u8) as _;
}
