/* SPDX-License-Identifier: GPL-2.0 */

/* C dependencies: <linux/rtnetlink.h> and <net/netlink.h>. */

/* Equivalent of NLA_ALIGN from <net/netlink.h>. */
#[inline]
const fn nla_align(len: usize) -> usize {
    (len + 3) & !3
}

#[inline]
pub unsafe fn rtnh_ok(rtnh: *const rtnexthop, remaining: i32) -> i32 {
    (remaining >= core::mem::size_of::<rtnexthop>() as i32
        && (*rtnh).rtnh_len as usize >= core::mem::size_of::<rtnexthop>()
        && (*rtnh).rtnh_len as i32 <= remaining) as i32
}

#[inline]
pub unsafe fn rtnh_next(
    rtnh: *const rtnexthop,
    remaining: *mut i32,
) -> *mut rtnexthop {
    let totlen = nla_align((*rtnh).rtnh_len as usize);

    *remaining -= totlen as i32;
    (rtnh as *const u8).add(totlen) as *mut rtnexthop
}

#[inline]
pub unsafe fn rtnh_attrs(rtnh: *const rtnexthop) -> *mut nlattr {
    (rtnh as *const u8).add(nla_align(core::mem::size_of::<rtnexthop>())) as *mut nlattr
}

#[inline]
pub unsafe fn rtnh_attrlen(rtnh: *const rtnexthop) -> i32 {
    (*rtnh).rtnh_len as i32 - nla_align(core::mem::size_of::<rtnexthop>()) as i32
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
