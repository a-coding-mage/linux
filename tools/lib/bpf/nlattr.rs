// SPDX-License-Identifier: (LGPL-2.1 OR BSD-2-Clause)

/*
 * NETLINK      Netlink attributes
 *
 * Copyright (c) 2003-2013 Thomas Graf <tgraf@suug.ch>
 */

use core::ffi::{c_char, c_int, c_uint, c_void};
use core::mem::size_of;
use core::ptr;

// C includes translated as external dependencies:
// <errno.h>, <string.h>, <stdio.h>, <linux/rtnetlink.h>, "nlattr.h",
// and "libbpf_internal.h".

unsafe extern "C" {
    fn libbpf_nla_len(nla: *const nlattr) -> c_int;
    fn libbpf_nla_data(nla: *const nlattr) -> *mut c_void;
    fn NLA_ALIGN(len: u16) -> c_int;
    fn NLMSG_DATA(nlh: *const nlmsghdr) -> *mut c_void;
    fn pr_warn(fmt: *const c_char, ...);
}

#[repr(C)]
pub struct nlattr {
    pub nla_len: u16,
    pub nla_type: u16,
}

#[repr(C)]
pub struct nlmsghdr {
    pub nlmsg_len: u32,
    pub nlmsg_type: u16,
    pub nlmsg_flags: u16,
    pub nlmsg_seq: u32,
    pub nlmsg_pid: u32,
}

#[repr(C)]
pub struct nlmsgerr {
    pub error: c_int,
    pub msg: nlmsghdr,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct libbpf_nla_policy {
    pub type_: c_uint,
    pub minlen: c_uint,
    pub maxlen: c_uint,
}

const EINVAL: c_int = 22;

// Constants below are provided by translated headers in the complete build.
extern "C" {
    static LIBBPF_NLA_TYPE_MAX: usize;
    static LIBBPF_NLA_U8: usize;
    static LIBBPF_NLA_U16: usize;
    static LIBBPF_NLA_U32: usize;
    static LIBBPF_NLA_U64: usize;
    static LIBBPF_NLA_STRING: usize;
    static LIBBPF_NLA_FLAG: usize;
    static LIBBPF_NLA_UNSPEC: c_uint;
    static NLA_TYPE_MASK: u16;
    static NLMSG_HDRLEN: u32;
    static NLMSGERR_ATTR_MAX: usize;
    static NLMSGERR_ATTR_MSG: usize;
    static NLMSGERR_ATTR_OFFS: usize;
    static NLM_F_ACK_TLVS: u16;
    static NLM_F_CAPPED: u16;
}

static mut nla_attr_minlen: [u16; LIBBPF_NLA_TYPE_MAX + 1] = {
    let mut a = [0u16; LIBBPF_NLA_TYPE_MAX + 1];
    a[LIBBPF_NLA_U8] = size_of::<u8>() as u16;
    a[LIBBPF_NLA_U16] = size_of::<u16>() as u16;
    a[LIBBPF_NLA_U32] = size_of::<u32>() as u16;
    a[LIBBPF_NLA_U64] = size_of::<u64>() as u16;
    a[LIBBPF_NLA_STRING] = 1;
    a[LIBBPF_NLA_FLAG] = 0;
    a
};

unsafe fn nla_next(nla: *const nlattr, remaining: *mut c_int) -> *mut nlattr {
    let totlen = NLA_ALIGN((*nla).nla_len);

    *remaining -= totlen;
    (nla as *mut c_void).byte_add(totlen as usize) as *mut nlattr
}

unsafe fn nla_ok(nla: *const nlattr, remaining: c_int) -> c_int {
    (remaining >= size_of::<nlattr>() as c_int
        && (*nla).nla_len as usize >= size_of::<nlattr>()
        && ((*nla).nla_len as c_int) <= remaining) as c_int
}

unsafe fn nla_type(nla: *const nlattr) -> c_int {
    ((*nla).nla_type & NLA_TYPE_MASK) as c_int
}

unsafe fn validate_nla(
    nla: *mut nlattr,
    maxtype: c_int,
    policy: *mut libbpf_nla_policy,
) -> c_int {
    let mut minlen: c_uint = 0;
    let type_ = nla_type(nla);

    if type_ < 0 || type_ > maxtype {
        return 0;
    }

    let pt = policy.add(type_ as usize);

    if (*pt).type_ > LIBBPF_NLA_TYPE_MAX as c_uint {
        return 0;
    }

    if (*pt).minlen != 0 {
        minlen = (*pt).minlen;
    } else if (*pt).type_ != LIBBPF_NLA_UNSPEC {
        minlen = nla_attr_minlen[(*pt).type_ as usize] as c_uint;
    }

    if (libbpf_nla_len(nla) as c_uint) < minlen {
        return -EINVAL;
    }

    if (*pt).maxlen != 0 && (libbpf_nla_len(nla) as c_uint) > (*pt).maxlen {
        return -EINVAL;
    }

    if (*pt).type_ == LIBBPF_NLA_STRING as c_uint {
        let data = libbpf_nla_data(nla) as *mut c_char;

        if *data.add((libbpf_nla_len(nla) - 1) as usize) != 0 {
            return -EINVAL;
        }
    }

    0
}

#[inline]
unsafe fn nlmsg_len(nlh: *const nlmsghdr) -> c_int {
    ((*nlh).nlmsg_len - NLMSG_HDRLEN) as c_int
}

/**
 * Create attribute index based on a stream of attributes.
 * @arg tb		Index array to be filled (maxtype+1 elements).
 * @arg maxtype		Maximum attribute type expected and accepted.
 * @arg head		Head of attribute stream.
 * @arg len		Length of attribute stream.
 * @arg policy		Attribute validation policy.
 *
 * Iterates over the stream of attributes and stores a pointer to each
 * attribute in the index array using the attribute type as index to
 * the array. Attribute with a type greater than the maximum type
 * specified will be silently ignored in order to maintain backwards
 * compatibility. If \a policy is not NULL, the attribute will be
 * validated using the specified policy.
 *
 * @see nla_validate
 * @return 0 on success or a negative error code.
 */
#[no_mangle]
pub unsafe extern "C" fn libbpf_nla_parse(
    tb: *mut *mut nlattr,
    maxtype: c_int,
    head: *mut nlattr,
    len: c_int,
    policy: *mut libbpf_nla_policy,
) -> c_int {
    let mut nla: *mut nlattr;
    let mut rem: c_int = len;
    let mut err: c_int;

    ptr::write_bytes(tb, 0, (maxtype + 1) as usize);

    nla = head;
    while nla_ok(nla, rem) != 0 {
        let type_ = nla_type(nla);

        if type_ > maxtype {
            nla = nla_next(nla, &mut rem);
            continue;
        }

        if !policy.is_null() {
            err = validate_nla(nla, maxtype, policy);
            if err < 0 {
                return err;
            }
        }

        if !(*tb.add(type_ as usize)).is_null() {
            pr_warn(
                c"Attribute of type %#x found multiple times in message, previous attribute is being ignored.\n"
                    .as_ptr(),
                type_ as c_uint,
            );
        }

        *tb.add(type_ as usize) = nla;
        nla = nla_next(nla, &mut rem);
    }

    0
}

/**
 * Create attribute index based on nested attribute
 * @arg tb              Index array to be filled (maxtype+1 elements).
 * @arg maxtype         Maximum attribute type expected and accepted.
 * @arg nla             Nested Attribute.
 * @arg policy          Attribute validation policy.
 *
 * Feeds the stream of attributes nested into the specified attribute
 * to libbpf_nla_parse().
 *
 * @see libbpf_nla_parse
 * @return 0 on success or a negative error code.
 */
#[no_mangle]
pub unsafe extern "C" fn libbpf_nla_parse_nested(
    tb: *mut *mut nlattr,
    maxtype: c_int,
    nla: *mut nlattr,
    policy: *mut libbpf_nla_policy,
) -> c_int {
    libbpf_nla_parse(
        tb,
        maxtype,
        libbpf_nla_data(nla) as *mut nlattr,
        libbpf_nla_len(nla),
        policy,
    )
}

/* dump netlink extended ack error message */
#[no_mangle]
pub unsafe extern "C" fn libbpf_nla_dump_errormsg(nlh: *mut nlmsghdr) -> c_int {
    let mut extack_policy: [libbpf_nla_policy; NLMSGERR_ATTR_MAX + 1] =
        [libbpf_nla_policy {
            type_: 0,
            minlen: 0,
            maxlen: 0,
        }; NLMSGERR_ATTR_MAX + 1];
    extack_policy[NLMSGERR_ATTR_MSG] = libbpf_nla_policy {
        type_: LIBBPF_NLA_STRING as c_uint,
        minlen: 0,
        maxlen: 0,
    };
    extack_policy[NLMSGERR_ATTR_OFFS] = libbpf_nla_policy {
        type_: LIBBPF_NLA_U32 as c_uint,
        minlen: 0,
        maxlen: 0,
    };

    let mut tb: [*mut nlattr; NLMSGERR_ATTR_MAX + 1] =
        [ptr::null_mut(); NLMSGERR_ATTR_MAX + 1];
    let mut errmsg: *mut c_char = ptr::null_mut();
    let hlen: c_int;
    let alen: c_int;

    /* no TLVs, nothing to do here */
    if ((*nlh).nlmsg_flags & NLM_F_ACK_TLVS) == 0 {
        return 0;
    }

    let err = NLMSG_DATA(nlh) as *mut nlmsgerr;
    hlen = size_of::<nlmsgerr>() as c_int
        + if ((*nlh).nlmsg_flags & NLM_F_CAPPED) == 0 {
            nlmsg_len(&mut (*err).msg)
        } else {
            0
        };

    let attr = (err as *mut c_void).byte_add(hlen as usize) as *mut nlattr;
    alen = (nlh as *mut c_void)
        .byte_add((*nlh).nlmsg_len as usize)
        .offset_from(attr as *mut c_void) as c_int;

    if libbpf_nla_parse(
        tb.as_mut_ptr(),
        NLMSGERR_ATTR_MAX as c_int,
        attr,
        alen,
        extack_policy.as_mut_ptr(),
    ) != 0
    {
        pr_warn(c"Failed to parse extended error attributes\n".as_ptr());
        return 0;
    }

    if !tb[NLMSGERR_ATTR_MSG].is_null() {
        errmsg = libbpf_nla_data(tb[NLMSGERR_ATTR_MSG]) as *mut c_char;
    }

    pr_warn(c"Kernel error message: %s\n".as_ptr(), errmsg);

    0
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
