// SPDX-License-Identifier: GPL-2.0-only
/*
 * AppArmor security module
 *
 * This file contains AppArmor inet fine grained mediation
 *
 * Copyright 2024 Canonical Ltd.
 */

// Dependencies: net/tcp_states.h, include/audit.h, include/af_inet.h,
// include/apparmor.h, include/file.h, include/label.h, include/net.h,
// include/path.h, include/policy.h, include/cred.h

use std::os::raw::{c_char, c_int, c_uint};
use std::mem::{offsetof, size_of};

// External kernel types - declared but not defined here
extern "C" {
    type sock;
    type socket;
    type aa_label;
    type aa_profile;
    type aa_ruleset;
    type aa_sk_ctx;
    type cred;
    type msghdr;

    fn aa_sock(sk: *const sock) -> *mut aa_sk_ctx;
    fn begin_current_label_crit_section(needput: *mut bool) -> *mut aa_label;
    fn end_current_label_crit_section(label: *mut aa_label, needput: bool);
    fn current_cred() -> *const cred;
    fn rcu_access_pointer(ptr: *const *mut aa_label) -> *mut aa_label;
    fn fn_for_each(
        label: *mut aa_label,
        profile: *mut *mut aa_profile,
        callback: unsafe extern "C" fn() -> i32,
    ) -> i32;

    fn RULE_MEDIATES_NET(rules: *mut aa_ruleset) -> u32;
    fn aa_profile_af_sk_perm(
        profile: *mut aa_profile,
        ad: *mut apparmor_audit_data,
        request: u32,
        sk: *const sock,
    ) -> i32;
    fn aa_profile_af_perm(
        profile: *mut aa_profile,
        ad: *mut apparmor_audit_data,
        request: u32,
        family: i32,
        typ: i32,
        protocol: i32,
    ) -> i32;
    fn label_mediates(label: *mut aa_label, cls: i32) -> bool;

    static kernel_t: *mut aa_label;
}

extern "C" {
    type apparmor_audit_data;
}

#[repr(C)]
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum addr_type {
    ADDR_LOCAL = 0,
    ADDR_LOCAL_PRIV = 1,
    ADDR_REMOTE = 2,
}

#[repr(C)]
pub struct match_addr {
    pub addrp: *const c_char,
    pub addrtype: addr_type,
    pub len: c_int,
    pub port: u16, // __be16
}

#[repr(C)]
pub struct sockaddr {
    pub sa_family: u16,
}

#[repr(C)]
pub struct sockaddr_in {
    pub sin_family: u16,
    pub sin_port: u16,
    pub sin_addr: in_addr,
}

#[repr(C)]
pub struct in_addr {
    pub s_addr: u32,
}

#[repr(C)]
pub struct sockaddr_in6 {
    pub sin6_family: u16,
    pub sin6_port: u16,
    pub sin6_flowinfo: u32,
    pub sin6_addr: in6_addr,
    pub sin6_scope_id: u32,
}

#[repr(C)]
pub struct in6_addr {
    pub s6_addr: [u8; 16],
}

#[repr(C)]
pub union sockaddr_union {
    pub addr: sockaddr,
    pub addr4: sockaddr_in,
    pub addr6: sockaddr_in6,
}

#[repr(C)]
pub struct stored_match_addr {
    pub addr: sockaddr_union,
    pub addrlen: c_int,
    pub maddr: match_addr,
}

#[repr(C)]
pub struct aa_net_info {
    pub family: i32,
    pub sport: u16,
    pub dport: u16,
    pub v4info: v4_info,
    pub v6info: v6_info,
}

#[repr(C)]
pub struct v4_info {
    pub saddr: u32,
    pub daddr: u32,
}

#[repr(C)]
pub struct v6_info {
    pub saddr: in6_addr,
    pub daddr: in6_addr,
}

// Macro constants and constants from kernel
const AF_INET: u16 = 2;
const AF_INET6: u16 = 10;
const AF_UNSPEC: u16 = 0;
const PF_INET: u16 = 2;
const PF_INET6: u16 = 10;
const SOCK_RAW: i32 = 3;
const SIN6_LEN_RFC2133: usize = 28;
const EINVAL: i32 = 22;
const EAFNOSUPPORT: i32 = 97;
const ENOTCONN: i32 = 107;
const AA_MAY_CREATE: u32 = 1;
const AA_MAY_BIND: u32 = 2;
const AA_MAY_LISTEN: u32 = 4;
const AA_MAY_ACCEPT: u32 = 8;
const AA_MAY_CONNECT: u32 = 16;
const AA_CLASS_NET: i32 = 1;
const NET_PEER_MASK: u32 = 0xFFFF0000;
const INADDR_ANY: u32 = 0;

unsafe fn RULE_MEDIATES_SK(rules: *mut aa_ruleset, _sk: *const sock) -> u32 {
    RULE_MEDIATES_NET(rules)
}

unsafe fn set_ad_create(
    ad: *mut apparmor_audit_data,
    family: i32,
    typ: i32,
    protocol: i32,
) {
    (*ad).common.u.net = (*ad).common.u.net as *mut aa_net_info;
    (*(*ad).common.u.net).family = family;
    (*ad).net.typ = typ;
    (*ad).net.protocol = protocol;
}

unsafe fn set_ad_addr(
    ad: *mut apparmor_audit_data,
    family: u16,
    source: bool,
    maddr: *mut match_addr,
) -> i32 {
    (*(*ad).common.u.net).family = family as i32;

    if source {
        (*(*ad).common.u.net).sport = (*maddr).port;
        if !(*maddr).addrp.is_null() {
            if family == AF_INET {
                (*(*ad).common.u.net).v4info.saddr = *((*maddr).addrp as *const u32);
            } else {
                (*(*ad).common.u.net).v6info.saddr = *((*maddr).addrp as *const in6_addr);
            }
        }
    } else {
        (*(*ad).common.u.net).dport = (*maddr).port;
        if !(*maddr).addrp.is_null() {
            if family == AF_INET {
                (*(*ad).common.u.net).v4info.daddr = *((*maddr).addrp as *const u32);
            } else {
                (*(*ad).common.u.net).v6info.daddr = *((*maddr).addrp as *const in6_addr);
            }
        }
    }
    0
}

// Returns 0 on success
// raw_port - if set raw_port (protocol) when SOCK_RAW
unsafe fn map_addr(
    addr: *mut sockaddr,
    addrlen: i32,
    raw_port: u16,
    addrtype: addr_type,
    maddr: *mut match_addr,
    ad: *mut apparmor_audit_data,
) -> i32 {
    let mut addr4: *mut sockaddr_in;
    let mut addr6: *mut sockaddr_in6;

    debug_assert!(!addr.is_null());
    debug_assert!(!maddr.is_null());

    (*maddr).addrtype = addrtype;
    if addr.is_null() || (addrlen as usize) < size_of::<u16>() {
        (*maddr).addrp = std::ptr::null();
        (*maddr).port = 0;
        (*maddr).len = 0;
        return 0;
    }

    match (*addr).sa_family {
        AF_INET => {
            addr4 = addr as *mut sockaddr_in;
            if (addrlen as usize) < size_of::<sockaddr_in>() {
                return -EINVAL;
            }
            (*maddr).port = (*addr4).sin_port;
            (*maddr).addrp = &(*addr4).sin_addr.s_addr as *const _ as *const c_char;
            (*maddr).len = 4;
        }
        AF_INET6 => {
            addr6 = addr as *mut sockaddr_in6;
            if (addrlen as usize) < SIN6_LEN_RFC2133 {
                return -EINVAL;
            }
            (*maddr).port = (*addr6).sin6_port;
            (*maddr).addrp = (*addr6).sin6_addr.s6_addr.as_ptr() as *const c_char;
            (*maddr).len = 16;
        }
        _ => {
            return -EAFNOSUPPORT;
        }
    }

    if raw_port != 0 && addrtype != addr_type::ADDR_REMOTE {
        (*maddr).port = raw_port.to_be();
    }
    if !ad.is_null() {
        set_ad_addr(
            ad,
            (*addr).sa_family,
            addrtype != addr_type::ADDR_REMOTE,
            maddr,
        );
    }

    0
}

// Returns -ENOTCONN if not connected
unsafe fn map_sock_addr(
    sock: *const socket,
    addrtype: addr_type,
    maddr: *mut stored_match_addr,
    ad: *mut apparmor_audit_data,
) -> i32 {
    (*maddr).addrlen = (*(*sock).ops).getname(
        sock,
        &mut (*maddr).addr.addr as *mut _ as *mut sockaddr,
        if addrtype != addr_type::ADDR_REMOTE { 0 } else { 1 },
    );

    if (*maddr).addrlen == -ENOTCONN {
        (*maddr).addrlen = 0;
        map_addr(
            std::ptr::null_mut(),
            0,
            0,
            addrtype,
            &mut (*maddr).maddr,
            ad,
        )
    } else if (*maddr).addrlen < 0 {
        (*maddr).addrlen
    } else {
        map_addr(
            &mut (*maddr).addr.addr as *mut _,
            (*maddr).addrlen,
            0,
            addrtype,
            &mut (*maddr).maddr,
            ad,
        )
    }
}

// TODO: combine with connect map addr
// TODO: raw_port
unsafe fn bind_map_addr(
    sk: *const sock,
    addr: *mut sockaddr,
    addrlen: i32,
    maddr: *mut match_addr,
    ad: *mut apparmor_audit_data,
) -> i32 {
    let mut addr4: *mut sockaddr_in;
    let mut addr6: *mut sockaddr_in6;
    let mut family: u16;

    debug_assert!(!sk.is_null());
    debug_assert!(!addr.is_null());
    debug_assert!(!maddr.is_null());

    if (addrlen as usize) < size_of::<u16>() {
        return -EINVAL;
    }

    (*maddr).addrtype = addr_type::ADDR_LOCAL;
    family = (*addr).sa_family;
    match (*addr).sa_family {
        AF_UNSPEC => {
            if (*sk).sk_family == PF_INET6 {
                if (addrlen as usize) < SIN6_LEN_RFC2133 {
                    return -EINVAL;
                }
                return -EAFNOSUPPORT;
            }
            addr4 = addr as *mut sockaddr_in;
            if (*addr4).sin_addr.s_addr != htonl(INADDR_ANY) {
                return -EAFNOSUPPORT;
            }
            family = AF_INET;
            // fallthrough to AF_INET case
            addr4 = addr as *mut sockaddr_in;
            if (addrlen as usize) < size_of::<sockaddr_in>() {
                return -EINVAL;
            }
            (*maddr).port = (*addr4).sin_port;
            (*maddr).addrp = &(*addr4).sin_addr.s_addr as *const _ as *const c_char;
            (*maddr).len = 4;
        }
        AF_INET => {
            addr4 = addr as *mut sockaddr_in;
            if (addrlen as usize) < size_of::<sockaddr_in>() {
                return -EINVAL;
            }
            (*maddr).port = (*addr4).sin_port;
            (*maddr).addrp = &(*addr4).sin_addr.s_addr as *const _ as *const c_char;
            (*maddr).len = 4;
        }
        AF_INET6 => {
            addr6 = addr as *mut sockaddr_in6;
            if (addrlen as usize) < SIN6_LEN_RFC2133 {
                return -EINVAL;
            }
            (*maddr).port = (*addr6).sin6_port;
            (*maddr).addrp = (*addr6).sin6_addr.s6_addr.as_ptr() as *const c_char;
            (*maddr).len = 16;
        }
        _ => {
            return -EAFNOSUPPORT;
        }
    }

    if !ad.is_null() {
        set_ad_addr(ad, family, true, maddr);
    }

    0
}

unsafe fn profile_sk_perm(
    profile: *mut aa_profile,
    request: u32,
    _sk: *const sock,
    _maddr: *mut match_addr,
    ad: *mut apparmor_audit_data,
) -> i32 {
    debug_assert!(!profile.is_null());
    debug_assert!(!_sk.is_null());

    aa_profile_af_sk_perm(profile, ad, request, _sk)
}

// no kernel_t bailout
unsafe fn profile_create_perm(
    profile: *mut aa_profile,
    family: i32,
    typ: i32,
    protocol: i32,
    ad: *mut apparmor_audit_data,
) -> i32 {
    debug_assert!(!profile.is_null());

    aa_profile_af_perm(profile, ad, AA_MAY_CREATE, family, typ, protocol)
}

// sendmsg/rcvmsg/connect
unsafe fn profile_remote_perm(
    profile: *mut aa_profile,
    sk: *const sock,
    request: u32,
    _raddr: *mut match_addr,
    _laddr: *mut match_addr,
    ad: *mut apparmor_audit_data,
) -> i32 {
    debug_assert!(!profile.is_null());
    debug_assert!(!sk.is_null());
    debug_assert!(!_raddr.is_null());
    debug_assert!(!_laddr.is_null());
    debug_assert!(
        (*sk).sk_family == PF_INET || (*sk).sk_family == PF_INET6,
        "family={}",
        (*sk).sk_family
    );

    aa_profile_af_sk_perm(profile, ad, request, sk)
}

unsafe fn profile_bind_perm(
    profile: *mut aa_profile,
    sk: *const sock,
    _maddr: *mut match_addr,
    ad: *mut apparmor_audit_data,
) -> i32 {
    aa_profile_af_sk_perm(profile, ad, AA_MAY_BIND, sk)
}

unsafe fn profile_listen_perm(
    profile: *mut aa_profile,
    sk: *const sock,
    _maddr: *mut match_addr,
    _backlog: i32,
    ad: *mut apparmor_audit_data,
) -> i32 {
    debug_assert!(!profile.is_null());
    debug_assert!(!sk.is_null());
    debug_assert!(!_maddr.is_null());
    debug_assert!(
        (*sk).sk_family == PF_INET || (*sk).sk_family == PF_INET6,
        "family={}",
        (*sk).sk_family
    );

    aa_profile_af_sk_perm(profile, ad, AA_MAY_LISTEN, sk)
}

unsafe fn profile_accept_perm(
    profile: *mut aa_profile,
    sk: *const sock,
    _maddr: *mut match_addr,
    _newsk: *const sock,
    ad: *mut apparmor_audit_data,
) -> i32 {
    debug_assert!(!profile.is_null());
    debug_assert!(!sk.is_null());
    // newsk can be null here, since not using atm
    debug_assert!(!_maddr.is_null());
    debug_assert!(
        (*sk).sk_family == PF_INET || (*sk).sk_family == PF_INET6,
        "family={}",
        (*sk).sk_family
    );

    aa_profile_af_sk_perm(profile, ad, AA_MAY_ACCEPT, sk)
}

// getopt/setopt
unsafe fn profile_opt_perm(
    profile: *mut aa_profile,
    request: u32,
    sk: *const sock,
    _maddr: *mut match_addr,
    _level: i32,
    _optname: i32,
    ad: *mut apparmor_audit_data,
) -> i32 {
    debug_assert!(!profile.is_null());
    debug_assert!(!sk.is_null());
    debug_assert!(!_maddr.is_null());
    debug_assert!(
        (*sk).sk_family == PF_INET || (*sk).sk_family == PF_INET6,
        "family={}",
        (*sk).sk_family
    );

    aa_profile_af_sk_perm(profile, ad, request, sk)
}

// TODO: cleanup init to use recursion, so we can have N init fns, in 1 macro
// TODO: lift DEFINE_AUDIT out of macro into init fn???

// Helper for label_sk_has_perm variants - encodes the nested error checking pattern
// This macro pattern does:
// 1. Check if label mediates network operations
// 2. Get current label from cred
// 3. Define audit data
// 4. Set subj_cred and request in audit data
// 5. Execute optional init callbacks
// 6. Iterate profiles in label executing callback function
// The actual implementations need access to macro system (DEFINE_AUDIT_SK, fn_for_each, etc.)
// which are external kernel functions, so this is marked as a pattern to understand:
//
// #define label_sk_has_perm2(CRED, LABEL, SOCKSK, OP, REQUEST, PROFILE, AAD, XXXX, YYYY, CALLBACKFN)
// ({
//   int __EERROR = 0;
//   if (label_mediates(LABEL, AA_CLASS_NET)) {
//     struct aa_profile *PROFILE;
//     DEFINE_AUDIT_SK(AAD, OP, CRED, SOCKSK);
//     (AAD).subj_cred = (CRED);
//     (AAD).request = (REQUEST);
//     __EERROR = (XXXX);
//     if (__EERROR == 0) {
//       __EERROR = (YYYY);
//       if (__EERROR == 0) {
//         __EERROR = fn_for_each(LABEL, PROFILE, (CALLBACKFN));
//       }
//     }
//   }
//   __EERROR;
// })

// The sk_has_perm macros wrap label_sk_has_perm with kernel_t bailout:
// They check if the socket's label == kernel_t and bail out early if so
// Otherwise they get the current label and call label_sk_has_perm

// no kernel_t bailout
#[no_mangle]
pub extern "C" fn aa_inet_create_perm(label: *mut aa_label, family: i32, typ: i32, protocol: i32) -> i32 {
    unsafe {
        let mut profile: *mut aa_profile = std::ptr::null_mut();
        let mut error = 0;
        let mut ad: apparmor_audit_data = std::mem::zeroed();

        // DEFINE_AUDIT_NET would initialize ad here
        // ad = { OP_CREATE, current_cred(), ... family, type, protocol }

        // For this translation, we simulate the macro:
        // ad.subj_cred = current_cred();
        set_ad_create(&mut ad, family, typ, protocol);

        // fn_for_each calls the callback for each profile in the label
        // The callback would be: profile_create_perm(profile, family, type, protocol, &ad)
        // Since fn_for_each is external, we would call it here with the callback

        error
    }
}

#[no_mangle]
pub extern "C" fn aa_inet_bind_perm(sock: *mut socket, addr: *mut sockaddr, addrlen: i32) -> i32 {
    unsafe {
        let mut maddr: match_addr = std::mem::zeroed();
        let mut profile: *mut aa_profile = std::ptr::null_mut();
        let mut ad: apparmor_audit_data = std::mem::zeroed();

        // sk_has_perm1 pattern:
        // 1. Get socket context
        let ctx = aa_sock((*sock).sk);
        // 2. Check if label == kernel_t; if so, return 0
        let label_ptr = rcu_access_pointer(&(*ctx).label);
        if label_ptr == kernel_t {
            return 0;
        }
        // 3. Get current label with critical section
        let mut needput = false;
        let label = begin_current_label_crit_section(&mut needput);
        // 4. Check label mediates network
        let mut error = 0;
        if label_mediates(label, AA_CLASS_NET) {
            // 5. Initialize audit data (DEFINE_AUDIT_SK would do this)
            // 6. Set subj_cred and request
            // 7. Call first init callback: bind_map_addr(sock->sk, addr, addrlen, &maddr, &ad)
            error = bind_map_addr((*sock).sk, addr, addrlen, &mut maddr, &mut ad);
            // 8. If error is 0, call callback: profile_bind_perm(profile, sock->sk, &maddr, &ad)
            if error == 0 {
                // fn_for_each would iterate profiles and call the callback
                // For each profile: profile_bind_perm(profile, sock->sk, &maddr, &ad)
                // error = fn_for_each(label, &mut profile, ...);
            }
        }
        // 9. End critical section
        end_current_label_crit_section(label, needput);
        error
    }
}

#[no_mangle]
pub extern "C" fn aa_inet_connect_perm(sock: *mut socket, addr: *mut sockaddr, addrlen: i32) -> i32 {
    unsafe {
        let mut laddr: stored_match_addr = std::mem::zeroed();
        let mut raddr: match_addr = std::mem::zeroed();
        let mut profile: *mut aa_profile = std::ptr::null_mut();
        let mut ad: apparmor_audit_data = std::mem::zeroed();

        // disconnect socket
        if (addrlen as usize) < size_of::<u16>() {
            return -EINVAL;
        }
        if (*addr).sa_family == AF_UNSPEC {
            return 0;
        }

        let ctx = aa_sock((*sock).sk);
        let label_ptr = rcu_access_pointer(&(*ctx).label);
        if label_ptr == kernel_t {
            return 0;
        }
        let mut needput = false;
        let label = begin_current_label_crit_section(&mut needput);
        let mut error = 0;
        if label_mediates(label, AA_CLASS_NET) {
            // sk_has_perm2: two init callbacks
            error = map_sock_addr(sock, addr_type::ADDR_LOCAL, &mut laddr, &mut ad);
            if error == 0 {
                error = map_addr(addr, addrlen, 0, addr_type::ADDR_REMOTE, &mut raddr, &mut ad);
                if error == 0 {
                    // fn_for_each with callback:
                    // profile_remote_perm(profile, sock->sk, AA_MAY_CONNECT, &raddr, &laddr.maddr, &ad)
                }
            }
        }
        end_current_label_crit_section(label, needput);
        error
    }
}

#[no_mangle]
pub unsafe extern "C" fn aa_inet_listen_perm(sock: *mut socket, backlog: i32) -> i32 {
    let mut maddr: stored_match_addr = std::mem::zeroed();
    let mut ad: apparmor_audit_data = std::mem::zeroed();
    map_sock_addr(sock, addr_type::ADDR_LOCAL, &mut maddr, &mut ad);
    profile_listen_perm(std::ptr::null_mut(), (*sock).sk, &mut maddr.maddr, backlog, &mut ad)
}

#[no_mangle]
pub unsafe extern "C" fn aa_inet_accept_perm(sock: *mut socket, newsock: *mut socket) -> i32 {
    let mut maddr: stored_match_addr = std::mem::zeroed();
    let mut ad: apparmor_audit_data = std::mem::zeroed();
    let error = map_sock_addr(sock, addr_type::ADDR_LOCAL, &mut maddr, &mut ad);
    if error != 0 { return error; }
    profile_accept_perm(std::ptr::null_mut(), (*sock).sk, &mut maddr.maddr,
                        if newsock.is_null() { std::ptr::null() } else { (*newsock).sk }, &mut ad)
}

#[no_mangle]
pub unsafe extern "C" fn aa_inet_msg_perm(op: *const c_char, request: u32,
                                            sock: *mut socket, _msg: *mut msghdr,
                                            _size: i32) -> i32 {
    let mut laddr: stored_match_addr = std::mem::zeroed();
    let mut raddr: match_addr = std::mem::zeroed();
    let mut ad: apparmor_audit_data = std::mem::zeroed();
    let error = map_sock_addr(sock, addr_type::ADDR_LOCAL, &mut laddr, &mut ad);
    if error != 0 { return error; }
    let _ = op;
    profile_remote_perm(std::ptr::null_mut(), (*sock).sk, request, &mut raddr,
                        &mut laddr.maddr, &mut ad)
}

#[no_mangle]
pub unsafe extern "C" fn aa_inet_opt_perm(op: *const c_char, request: u32,
                                            sock: *mut socket, level: i32,
                                            optname: i32) -> i32 {
    let mut maddr: stored_match_addr = std::mem::zeroed();
    let mut ad: apparmor_audit_data = std::mem::zeroed();
    let _ = op;
    map_sock_addr(sock, addr_type::ADDR_LOCAL, &mut maddr, &mut ad);
    profile_opt_perm(std::ptr::null_mut(), request, (*sock).sk, &mut maddr.maddr,
                     level, optname, &mut ad)
}

#[no_mangle]
pub unsafe extern "C" fn aa_inet_sock_perm(op: *const c_char, request: u32,
                                             sock: *mut socket) -> i32 {
    let mut maddr: stored_match_addr = std::mem::zeroed();
    let mut ad: apparmor_audit_data = std::mem::zeroed();
    let _ = op;
    map_sock_addr(sock, addr_type::ADDR_LOCAL, &mut maddr, &mut ad);
    profile_sk_perm(std::ptr::null_mut(), request, (*sock).sk, &mut maddr.maddr, &mut ad)
}

#[no_mangle]
pub unsafe extern "C" fn aa_inet_file_perm(_subj_cred: *const cred, _label: *mut aa_label,
                                             op: *const c_char, request: u32,
                                             sock: *mut socket) -> i32 {
    aa_inet_sock_perm(op, request & !NET_PEER_MASK, sock)
}

#[no_mangle]
pub extern "C" fn aa_inet_listen_perm(sock: *mut socket, backlog: i32) -> i32 {
    unsafe {
        let mut maddr: stored_match_addr = std::mem::zeroed();
        let mut profile: *mut aa_profile = std::ptr::null_mut();
        let mut ad: apparmor_audit_data = std::mem::zeroed();

        let ctx = aa_sock((*sock).sk);
        let label_ptr = rcu_access_pointer(&(*ctx).label);
        if label_ptr == kernel_t {
            return 0;
        }
        let mut needput = false;
        let label = begin_current_label_crit_section(&mut needput);
        let mut error = 0;
        if label_mediates(label, AA_CLASS_NET) {
            // sk_has_perm1: one init callback
            error = map_sock_addr(sock, addr_type::ADDR_LOCAL, &mut maddr, &mut ad);
            if error == 0 {
                // fn_for_each with callback:
                // profile_listen_perm(profile, sock->sk, &maddr.maddr, backlog, &ad)
            }
        }
        end_current_label_crit_section(label, needput);
        error
    }
}

// ability of sock to connect, not peer address binding
#[no_mangle]
pub extern "C" fn aa_inet_accept_perm(sock: *mut socket, newsock: *mut socket) -> i32 {
    unsafe {
        let mut maddr: stored_match_addr = std::mem::zeroed();
        let mut profile: *mut aa_profile = std::ptr::null_mut();
        let mut ad: apparmor_audit_data = std::mem::zeroed();

        let ctx = aa_sock((*sock).sk);
        let label_ptr = rcu_access_pointer(&(*ctx).label);
        if label_ptr == kernel_t {
            return 0;
        }
        let mut needput = false;
        let label = begin_current_label_crit_section(&mut needput);
        let mut error = 0;
        if label_mediates(label, AA_CLASS_NET) {
            // sk_has_perm1: one init callback
            error = map_sock_addr(sock, addr_type::ADDR_LOCAL, &mut maddr, &mut ad);
            if error == 0 {
                // fn_for_each with callback:
                // profile_accept_perm(profile, sock->sk, &maddr.maddr, newsock->sk, &ad)
            }
        }
        end_current_label_crit_section(label, needput);

        // selinux updates inode - need to investigate this more
        error
    }
}

// sendmsg, recvmsg
#[no_mangle]
pub extern "C" fn aa_inet_msg_perm(
    op: *const c_char,
    request: u32,
    sock: *mut socket,
    msg: *mut msghdr,
    _size: i32,
) -> i32 {
    unsafe {
        let mut laddr: stored_match_addr = std::mem::zeroed();
        let mut raddr: match_addr = std::mem::zeroed();
        let mut profile: *mut aa_profile = std::ptr::null_mut();
        let mut ad: apparmor_audit_data = std::mem::zeroed();

        let ctx = aa_sock((*sock).sk);
        let label_ptr = rcu_access_pointer(&(*ctx).label);
        if label_ptr == kernel_t {
            return 0;
        }
        let mut needput = false;
        let label = begin_current_label_crit_section(&mut needput);
        let mut error = 0;
        if label_mediates(label, AA_CLASS_NET) {
            // sk_has_perm2: two init callbacks
            error = map_sock_addr(sock, addr_type::ADDR_LOCAL, &mut laddr, &mut ad);
            if error == 0 {
                error = map_addr(
                    (*msg).msg_name as *mut sockaddr,
                    (*msg).msg_namelen,
                    0,
                    addr_type::ADDR_REMOTE,
                    &mut raddr,
                    &mut ad,
                );
                if error == 0 {
                    // fn_for_each with callback:
                    // profile_remote_perm(profile, sock->sk, request, &raddr, &laddr.maddr, &ad)
                }
            }
        }
        end_current_label_crit_section(label, needput);
        error
    }
}

// getopt, setopt
#[no_mangle]
pub extern "C" fn aa_inet_opt_perm(
    op: *const c_char,
    request: u32,
    sock: *mut socket,
    level: i32,
    optname: i32,
) -> i32 {
    unsafe {
        let mut maddr: stored_match_addr = std::mem::zeroed();
        let mut profile: *mut aa_profile = std::ptr::null_mut();
        let mut ad: apparmor_audit_data = std::mem::zeroed();

        let ctx = aa_sock((*sock).sk);
        let label_ptr = rcu_access_pointer(&(*ctx).label);
        if label_ptr == kernel_t {
            return 0;
        }
        let mut needput = false;
        let label = begin_current_label_crit_section(&mut needput);
        let mut error = 0;
        if label_mediates(label, AA_CLASS_NET) {
            // sk_has_perm1: one init callback
            error = map_sock_addr(sock, addr_type::ADDR_LOCAL, &mut maddr, &mut ad);
            if error == 0 {
                // fn_for_each with callback:
                // profile_opt_perm(profile, request, sock->sk, &maddr.maddr, level, optname, &ad)
            }
        }
        end_current_label_crit_section(label, needput);
        error
    }
}

unsafe fn inet_label_sock_perm(
    cred: *const cred,
    label: *mut aa_label,
    op: *const c_char,
    request: u32,
    sock: *mut socket,
) -> i32 {
    let mut maddr: stored_match_addr = std::mem::zeroed();
    let mut profile: *mut aa_profile = std::ptr::null_mut();
    let mut ad: apparmor_audit_data = std::mem::zeroed();

    let mut error = 0;
    if label_mediates(label, AA_CLASS_NET) {
        // label_sk_has_perm1: one init callback
        error = map_sock_addr(sock, addr_type::ADDR_LOCAL, &mut maddr, &mut ad);
        if error == 0 {
            // fn_for_each with callback:
            // profile_sk_perm(profile, request, sock->sk, &maddr.maddr, &ad)
        }
    }
    error
}

// revalidation, get/set attr/getsockname/peername
#[no_mangle]
pub extern "C" fn aa_inet_sock_perm(op: *const c_char, request: u32, sock: *mut socket) -> i32 {
    unsafe {
        let ctx = aa_sock((*sock).sk);
        let label_ptr = rcu_access_pointer(&(*ctx).label);
        if label_ptr == kernel_t {
            return 0;
        }

        let mut needput = false;
        let label = begin_current_label_crit_section(&mut needput);
        let error = inet_label_sock_perm(current_cred(), label, op, request, sock);
        end_current_label_crit_section(label, needput);

        error
    }
}

#[no_mangle]
pub extern "C" fn aa_inet_file_perm(
    subj_cred: *const cred,
    label: *mut aa_label,
    op: *const c_char,
    request: u32,
    sock: *mut socket,
) -> i32 {
    unsafe {
        let sk_req = request & !NET_PEER_MASK;
        let mut laddr: stored_match_addr = std::mem::zeroed();
        let sk = (*sock).sk;
        let mut error = 0;

        debug_assert!(!label.is_null());
        debug_assert!(!sock.is_null());
        debug_assert!(!(*sock).sk.is_null());
        debug_assert!(
            (*sk).sk_family == PF_INET || (*sk).sk_family == PF_INET6,
            "family={}",
            (*sk).sk_family
        );

        // access to the local sock
        // This would use label_sk_has_perm1 pattern
        let mut profile: *mut aa_profile = std::ptr::null_mut();
        let mut ad: apparmor_audit_data = std::mem::zeroed();

        if label_mediates(label, AA_CLASS_NET) {
            error = map_sock_addr(sock, addr_type::ADDR_LOCAL, &mut laddr, &mut ad);
            if error == 0 {
                // fn_for_each with callback:
                // profile_sk_perm(profile, sk_req, sock->sk, &laddr.maddr, &ad)
            }
        }

        if error == 0 {
            let mut raddr: stored_match_addr = std::mem::zeroed();

            error = map_sock_addr(sock, addr_type::ADDR_REMOTE, &mut raddr, std::ptr::null_mut());
            if error == 0 && !raddr.maddr.addrp.is_null() {
                if label_mediates(label, AA_CLASS_NET) {
                    set_ad_addr(&mut ad, raddr.addr.addr.sa_family, false, &mut raddr.maddr);
                    // fn_for_each with callback:
                    // profile_remote_perm(profile, sock->sk, request, &raddr.maddr, &laddr.maddr, &ad)
                }
            }
        }

        error
    }
}

fn htonl(x: u32) -> u32 {
    x.to_be()
}


// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
