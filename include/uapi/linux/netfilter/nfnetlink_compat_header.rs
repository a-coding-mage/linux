/* SPDX-License-Identifier: GPL-2.0 WITH Linux-syscall-note */
// Translated from nfnetlink_compat.h.  The original declarations are for
// userspace; the __KERNEL__ conditional is therefore intentionally omitted.

pub const NF_NETLINK_CONNTRACK_NEW: u32 = 0x0000_0001;
pub const NF_NETLINK_CONNTRACK_UPDATE: u32 = 0x0000_0002;
pub const NF_NETLINK_CONNTRACK_DESTROY: u32 = 0x0000_0004;
pub const NF_NETLINK_CONNTRACK_EXP_NEW: u32 = 0x0000_0008;
pub const NF_NETLINK_CONNTRACK_EXP_UPDATE: u32 = 0x0000_0010;
pub const NF_NETLINK_CONNTRACK_EXP_DESTROY: u32 = 0x0000_0020;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct nfattr {
    pub nfa_len: u16,
    pub nfa_type: u16,
}

pub const NFNL_NFA_NEST: u16 = 0x8000;
pub const NFA_ALIGNTO: usize = 4;

#[inline]
pub const fn nfa_align(len: usize) -> usize {
    (len + NFA_ALIGNTO - 1) & !(NFA_ALIGNTO - 1)
}

#[inline]
pub const fn nfa_length(len: usize) -> usize {
    nfa_align(core::mem::size_of::<nfattr>()) + len
}

#[inline]
pub const fn nfa_space(len: usize) -> usize {
    nfa_align(nfa_length(len))
}

#[inline]
pub unsafe fn nfa_type(attr: *const nfattr) -> u16 {
    (*attr).nfa_type & 0x7fff
}

#[inline]
pub unsafe fn nfa_ok(nfa: *const nfattr, len: usize) -> bool {
    len > 0
        && ((*nfa).nfa_len as usize) >= core::mem::size_of::<nfattr>()
        && ((*nfa).nfa_len as usize) <= len
}

#[inline]
pub unsafe fn nfa_next(nfa: *mut nfattr, attrlen: &mut usize) -> *mut nfattr {
    let aligned = nfa_align((*nfa).nfa_len as usize);
    *attrlen -= aligned;
    (nfa as *mut u8).add(aligned) as *mut nfattr
}

#[inline]
pub unsafe fn nfa_data(nfa: *mut nfattr) -> *mut core::ffi::c_void {
    (nfa as *mut u8).add(nfa_length(0)) as *mut core::ffi::c_void
}

#[inline]
pub unsafe fn nfa_payload(nfa: *const nfattr) -> i32 {
    (*nfa).nfa_len as i32 - nfa_length(0) as i32
}

/* NFA_NEST, NFA_NEST_END, and NFA_NEST_CANCEL retain their original
 * statement-expression behavior through Rust macros and depend on the
 * externally supplied skb/NFA_PUT interfaces. */
#[macro_export]
macro_rules! NFA_NEST {
    ($skb:expr, $type:expr) => {{
        let __start = $crate::nfattr {
            nfa_len: 0,
            nfa_type: 0,
        };
        NFA_PUT($skb, (NFNL_NFA_NEST | $type), 0, core::ptr::null_mut());
        __start
    }};
}

#[macro_export]
macro_rules! NFA_NEST_END {
    ($skb:expr, $start:expr) => {{
        ($start).nfa_len = skb_tail_pointer($skb).offset_from(
            (&mut ($start)) as *mut nfattr as *mut u8,
        ) as u16;
        ($skb).len
    }};
}

#[macro_export]
macro_rules! NFA_NEST_CANCEL {
    ($skb:expr, $start:expr) => {{
        if !$start.is_null() {
            skb_trim($skb, (($start as *mut u8).offset_from(($skb).data)) as usize);
        }
        -1
    }};
}

#[inline]
pub unsafe fn nfm_nfa(n: *mut core::ffi::c_void) -> *mut nfattr {
    (n as *mut u8).add(nlmsg_align(core::mem::size_of::<nfgenmsg>())) as *mut nfattr
}

#[inline]
pub unsafe fn nfm_payload(n: *const core::ffi::c_void) -> usize {
    nlmsg_payload(n, core::mem::size_of::<nfgenmsg>())
}

// External types and helpers referenced by the original header (including
// nfgenmsg, skb_tail_pointer, skb_trim, NFA_PUT, nlmsg_align, and
// nlmsg_payload) are supplied by the surrounding translation unit.

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
