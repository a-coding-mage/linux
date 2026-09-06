// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * SELinux NetLabel Support
 *
 * This file provides the necessary glue to tie NetLabel into the SELinux
 * subsystem.
 *
 * Author: Paul Moore <paul@paul-moore.com>
 */

/*
 * (c) Copyright Hewlett-Packard Development Company, L.P., 2007, 2008
 */

/* Dependencies from:
 * linux/spinlock.h, linux/rcupdate.h, linux/gfp.h, linux/ip.h, linux/ipv6.h,
 * linux/lsm_hooks.h, net/sock.h, net/netlabel.h, net/ip.h, net/ipv6.h,
 * objsec.h, security.h, netlabel.h
 */

#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(dead_code)]
#![allow(improper_ctypes)]

use core::ffi::c_void;

type u16 = core::ffi::c_ushort;
type u32 = core::ffi::c_uint;
type c_int = core::ffi::c_int;

const GFP_ATOMIC: c_int = 0;
const ENOMEM: c_int = 12;
const EAFNOSUPPORT: c_int = 97;
const EDESTADDRREQ: c_int = 89;
const EACCES: c_int = 13;
const ENOMSG: c_int = 42;

const NETLBL_SECATTR_NONE: u32 = 0;
const NETLBL_SECATTR_CACHEABLE: u32 = 1 << 0;
const NETLBL_SECATTR_CACHE: u32 = 1 << 1;
const NETLBL_SECATTR_SECID: u32 = 1 << 2;
const NETLBL_NLTYPE_NONE: u32 = 0;

const SECSID_NULL: u32 = 0;
const SECINITSID_UNLABELED: u32 = 2;

const PF_INET: u16 = 2;
const PF_INET6: u16 = 10;
const AF_INET: u16 = 2;
const AF_INET6: u16 = 10;
const AF_UNSPEC: u16 = 0;

const IPPROTO_IP: c_int = 0;
const IPPROTO_IPV6: c_int = 41;
const IP_OPTIONS: c_int = 4;
const IPV6_HOPOPTS: c_int = 54;

const SECCLASS_UDP_SOCKET: u16 = 1;
const SECCLASS_TCP_SOCKET: u16 = 2;
const UDP_SOCKET__RECVFROM: u32 = 1;
const TCP_SOCKET__RECVFROM: u32 = 1;
const RAWIP_SOCKET__RECVFROM: u32 = 1;

const NLBL_UNSET: u32 = 0;
const NLBL_REQSKB: u32 = 1;
const NLBL_LABELED: u32 = 2;
const NLBL_CONNLABELED: u32 = 3;

#[repr(C)]
pub struct sk_buff {
    _private: [u8; 0],
}

#[repr(C)]
pub struct sock {
    pub sk_family: u16,
}

#[repr(C)]
pub struct socket {
    pub sk: *mut sock,
}

#[repr(C)]
pub struct request_sock {
    pub secid: u32,
}

#[repr(C)]
pub struct sctp_association_base {
    pub sk: *mut sock,
}

#[repr(C)]
pub struct sctp_association {
    pub base: sctp_association_base,
    pub secid: u32,
}

#[repr(C)]
pub struct common_audit_data {
    _private: [u8; 0],
}

#[repr(C)]
pub struct sk_security_struct {
    pub nlbl_secattr: *mut netlbl_lsm_secattr,
    pub nlbl_state: u32,
    pub sid: u32,
    pub sclass: u16,
}

#[repr(C)]
pub struct netlbl_lsm_secattr {
    pub flags: u32,
    pub type_: u32,
    pub attr: netlbl_lsm_secattr_attr,
}

#[repr(C)]
pub union netlbl_lsm_secattr_attr {
    pub secid: u32,
}

#[repr(C)]
pub struct in_addr {
    pub s_addr: u32,
}

#[repr(C)]
pub struct in6_addr {
    pub s6_addr: [u8; 16],
}

#[repr(C)]
pub struct sockaddr {
    pub sa_family: u16,
    pub sa_data: [u8; 14],
}

#[repr(C)]
pub struct sockaddr_in {
    pub sin_family: u16,
    pub sin_addr: in_addr,
}

#[repr(C)]
pub struct sockaddr_in6 {
    pub sin6_family: u16,
    pub sin6_addr: in6_addr,
}

#[repr(C)]
pub struct iphdr {
    pub version: u8,
    pub saddr: u32,
}

#[repr(C)]
pub struct ipv6hdr {
    pub saddr: in6_addr,
}

unsafe extern "C" {
    fn security_netlbl_secattr_to_sid(secattr: *mut netlbl_lsm_secattr, sid: *mut u32) -> c_int;
    fn security_netlbl_sid_to_secattr(sid: u32, secattr: *mut netlbl_lsm_secattr) -> c_int;
    fn netlbl_cache_add(skb: *mut sk_buff, family: u16, secattr: *mut netlbl_lsm_secattr);
    fn netlbl_secattr_alloc(flags: c_int) -> *mut netlbl_lsm_secattr;
    fn netlbl_secattr_free(secattr: *mut netlbl_lsm_secattr);
    fn netlbl_cache_invalidate();
    fn netlbl_skbuff_err(skb: *mut sk_buff, family: u16, error: c_int, gateway: c_int);
    fn netlbl_enabled() -> bool;
    fn netlbl_secattr_init(secattr: *mut netlbl_lsm_secattr);
    fn netlbl_skbuff_getattr(
        skb: *mut sk_buff,
        family: u16,
        secattr: *mut netlbl_lsm_secattr,
    ) -> c_int;
    fn netlbl_secattr_destroy(secattr: *mut netlbl_lsm_secattr);
    fn skb_to_full_sk(skb: *mut sk_buff) -> *mut sock;
    fn netlbl_skbuff_setattr(
        skb: *mut sk_buff,
        family: u16,
        secattr: *mut netlbl_lsm_secattr,
    ) -> c_int;
    fn netlbl_conn_setattr(
        sk: *mut sock,
        addr: *mut c_void,
        secattr: *mut netlbl_lsm_secattr,
    ) -> c_int;
    fn netlbl_req_setattr(req: *mut request_sock, secattr: *mut netlbl_lsm_secattr) -> c_int;
    fn netlbl_sock_setattr(
        sk: *mut sock,
        family: u16,
        secattr: *mut netlbl_lsm_secattr,
        reclaim: bool,
    ) -> c_int;
    fn avc_has_perm(
        ssid: u32,
        tsid: u32,
        tclass: u16,
        requested: u32,
        auditdata: *mut common_audit_data,
    ) -> c_int;
    fn lock_sock(sk: *mut sock);
    fn netlbl_sock_getattr(sk: *mut sock, secattr: *mut netlbl_lsm_secattr) -> c_int;
    fn release_sock(sk: *mut sock);
    fn netlbl_sock_delattr(sk: *mut sock);
    fn selinux_sock(sk: *const sock) -> *mut sk_security_struct;
    fn ip_hdr(skb: *mut sk_buff) -> *mut iphdr;
    fn ipv6_hdr(skb: *mut sk_buff) -> *mut ipv6hdr;
}

unsafe fn ERR_PTR(error: c_int) -> *mut netlbl_lsm_secattr {
    error as isize as *mut netlbl_lsm_secattr
}

unsafe fn IS_ERR(ptr: *const netlbl_lsm_secattr) -> bool {
    (ptr as isize) < 0 && (ptr as isize) >= -4095
}

unsafe fn PTR_ERR(ptr: *const netlbl_lsm_secattr) -> c_int {
    ptr as isize as c_int
}

fn IS_ENABLED_CONFIG_IPV6() -> bool {
    true
}

/**
 * selinux_netlbl_sidlookup_cached - Cache a SID lookup
 * @skb: the packet
 * @family: the packet's address family
 * @secattr: the NetLabel security attributes
 * @sid: the SID
 *
 * Description:
 * Query the SELinux security server to lookup the correct SID for the given
 * security attributes.  If the query is successful, cache the result to speed
 * up future lookups.  Returns zero on success, negative values on failure.
 *
 */
unsafe fn selinux_netlbl_sidlookup_cached(
    skb: *mut sk_buff,
    family: u16,
    secattr: *mut netlbl_lsm_secattr,
    sid: *mut u32,
) -> c_int {
    let rc: c_int;

    rc = security_netlbl_secattr_to_sid(secattr, sid);
    if rc == 0
        && ((*secattr).flags & NETLBL_SECATTR_CACHEABLE) != 0
        && ((*secattr).flags & NETLBL_SECATTR_CACHE) != 0
    {
        netlbl_cache_add(skb, family, secattr);
    }

    rc
}

/**
 * selinux_netlbl_sock_genattr - Generate the NetLabel socket secattr
 * @sk: the socket
 *
 * Description:
 * Generate the NetLabel security attributes for a socket, making full use of
 * the socket's attribute cache.  Returns a pointer to the security attributes
 * on success, or an ERR_PTR on failure.
 *
 */
unsafe fn selinux_netlbl_sock_genattr(sk: *mut sock) -> *mut netlbl_lsm_secattr {
    let rc: c_int;
    let sksec: *mut sk_security_struct = selinux_sock(sk);
    let secattr: *mut netlbl_lsm_secattr;

    if !(*sksec).nlbl_secattr.is_null() {
        return (*sksec).nlbl_secattr;
    }

    secattr = netlbl_secattr_alloc(GFP_ATOMIC);
    if secattr.is_null() {
        return ERR_PTR(-ENOMEM);
    }

    rc = security_netlbl_sid_to_secattr((*sksec).sid, secattr);
    if rc != 0 {
        netlbl_secattr_free(secattr);
        return ERR_PTR(rc);
    }
    (*sksec).nlbl_secattr = secattr;

    secattr
}

/**
 * selinux_netlbl_sock_getattr - Get the cached NetLabel secattr
 * @sk: the socket
 * @sid: the SID
 *
 * Query the socket's cached secattr and if the SID matches the cached value
 * return the cache, otherwise return NULL.
 *
 */
unsafe fn selinux_netlbl_sock_getattr(sk: *const sock, sid: u32) -> *mut netlbl_lsm_secattr {
    let sksec: *mut sk_security_struct = selinux_sock(sk);
    let secattr: *mut netlbl_lsm_secattr = (*sksec).nlbl_secattr;

    if secattr.is_null() {
        return core::ptr::null_mut();
    }

    if ((*secattr).flags & NETLBL_SECATTR_SECID) != 0 && (*secattr).attr.secid == sid {
        return secattr;
    }

    core::ptr::null_mut()
}

/**
 * selinux_netlbl_cache_invalidate - Invalidate the NetLabel cache
 *
 * Description:
 * Invalidate the NetLabel security attribute mapping cache.
 *
 */
#[no_mangle]
pub unsafe extern "C" fn selinux_netlbl_cache_invalidate() {
    netlbl_cache_invalidate();
}

/**
 * selinux_netlbl_err - Handle a NetLabel packet error
 * @skb: the packet
 * @family: the packet's address family
 * @error: the error code
 * @gateway: true if host is acting as a gateway, false otherwise
 *
 * Description:
 * When a packet is dropped due to a call to avc_has_perm() pass the error
 * code to the NetLabel subsystem so any protocol specific processing can be
 * done.  This is safe to call even if you are unsure if NetLabel labeling is
 * present on the packet, NetLabel is smart enough to only act when it should.
 *
 */
#[no_mangle]
pub unsafe extern "C" fn selinux_netlbl_err(
    skb: *mut sk_buff,
    family: u16,
    error: c_int,
    gateway: c_int,
) {
    netlbl_skbuff_err(skb, family, error, gateway);
}

/**
 * selinux_netlbl_sk_security_free - Free the NetLabel fields
 * @sksec: the sk_security_struct
 *
 * Description:
 * Free all of the memory in the NetLabel fields of a sk_security_struct.
 *
 */
#[no_mangle]
pub unsafe extern "C" fn selinux_netlbl_sk_security_free(sksec: *mut sk_security_struct) {
    if (*sksec).nlbl_secattr.is_null() {
        return;
    }

    netlbl_secattr_free((*sksec).nlbl_secattr);
    (*sksec).nlbl_secattr = core::ptr::null_mut();
    (*sksec).nlbl_state = NLBL_UNSET;
}

/**
 * selinux_netlbl_sk_security_reset - Reset the NetLabel fields
 * @sksec: the sk_security_struct
 *
 * Description:
 * Called when the NetLabel state of a sk_security_struct needs to be reset.
 * The caller is responsible for all the NetLabel sk_security_struct locking.
 *
 */
#[no_mangle]
pub unsafe extern "C" fn selinux_netlbl_sk_security_reset(sksec: *mut sk_security_struct) {
    (*sksec).nlbl_state = NLBL_UNSET;
}

/**
 * selinux_netlbl_skbuff_getsid - Get the sid of a packet using NetLabel
 * @skb: the packet
 * @family: protocol family
 * @type: NetLabel labeling protocol type
 * @sid: the SID
 *
 * Description:
 * Call the NetLabel mechanism to get the security attributes of the given
 * packet and use those attributes to determine the correct context/SID to
 * assign to the packet.  Returns zero on success, negative values on failure.
 *
 */
#[no_mangle]
pub unsafe extern "C" fn selinux_netlbl_skbuff_getsid(
    skb: *mut sk_buff,
    family: u16,
    type_: *mut u32,
    sid: *mut u32,
) -> c_int {
    let mut rc: c_int;
    let mut secattr: netlbl_lsm_secattr = core::mem::zeroed();

    if !netlbl_enabled() {
        *type_ = NETLBL_NLTYPE_NONE;
        *sid = SECSID_NULL;
        return 0;
    }

    netlbl_secattr_init(&mut secattr);
    rc = netlbl_skbuff_getattr(skb, family, &mut secattr);
    if rc == 0 && secattr.flags != NETLBL_SECATTR_NONE {
        rc = selinux_netlbl_sidlookup_cached(skb, family, &mut secattr, sid);
    } else {
        *sid = SECSID_NULL;
    }
    *type_ = secattr.type_;
    netlbl_secattr_destroy(&mut secattr);

    rc
}

/**
 * selinux_netlbl_skbuff_setsid - Set the NetLabel on a packet given a sid
 * @skb: the packet
 * @family: protocol family
 * @sid: the SID
 *
 * Description
 * Call the NetLabel mechanism to set the label of a packet using @sid.
 * Returns zero on success, negative values on failure.
 *
 */
#[no_mangle]
pub unsafe extern "C" fn selinux_netlbl_skbuff_setsid(
    skb: *mut sk_buff,
    family: u16,
    sid: u32,
) -> c_int {
    let mut rc: c_int;
    let mut secattr_storage: netlbl_lsm_secattr = core::mem::zeroed();
    let mut secattr: *mut netlbl_lsm_secattr = core::ptr::null_mut();
    let sk: *mut sock;

    /* if this is a locally generated packet check to see if it is already
     * being labeled by it's parent socket, if it is just exit */
    sk = skb_to_full_sk(skb);
    if !sk.is_null() {
        let sksec: *mut sk_security_struct = selinux_sock(sk);

        if (*sksec).nlbl_state != NLBL_REQSKB {
            return 0;
        }
        secattr = selinux_netlbl_sock_getattr(sk, sid);
    }
    if secattr.is_null() {
        secattr = &mut secattr_storage;
        netlbl_secattr_init(secattr);
        rc = security_netlbl_sid_to_secattr(sid, secattr);
        if rc != 0 {
            goto_skbuff_setsid_return(secattr, &mut secattr_storage, rc);
            return rc;
        }
    }

    rc = netlbl_skbuff_setattr(skb, family, secattr);

    if core::ptr::eq(secattr, &mut secattr_storage) {
        netlbl_secattr_destroy(secattr);
    }
    rc
}

unsafe fn goto_skbuff_setsid_return(
    secattr: *mut netlbl_lsm_secattr,
    secattr_storage: *mut netlbl_lsm_secattr,
    rc: c_int,
) {
    if core::ptr::eq(secattr, secattr_storage) {
        netlbl_secattr_destroy(secattr);
    }
    let _ = rc;
}

/**
 * selinux_netlbl_sctp_assoc_request - Label an incoming sctp association.
 * @asoc: incoming association.
 * @skb: the packet.
 *
 * Description:
 * A new incoming connection is represented by @asoc, ......
 * Returns zero on success, negative values on failure.
 *
 */
#[no_mangle]
pub unsafe extern "C" fn selinux_netlbl_sctp_assoc_request(
    asoc: *mut sctp_association,
    skb: *mut sk_buff,
) -> c_int {
    let mut rc: c_int;
    let mut secattr: netlbl_lsm_secattr = core::mem::zeroed();
    let sksec: *mut sk_security_struct = selinux_sock((*asoc).base.sk);
    let mut addr4: sockaddr_in = core::mem::zeroed();
    let mut addr6: sockaddr_in6 = core::mem::zeroed();

    if (*(*asoc).base.sk).sk_family != PF_INET && (*(*asoc).base.sk).sk_family != PF_INET6 {
        return 0;
    }

    netlbl_secattr_init(&mut secattr);
    rc = security_netlbl_sid_to_secattr((*asoc).secid, &mut secattr);
    if rc != 0 {
        netlbl_secattr_destroy(&mut secattr);
        return rc;
    }

    /* Move skb hdr address info to a struct sockaddr and then call
     * netlbl_conn_setattr().
     */
    if (*ip_hdr(skb)).version == 4 {
        addr4.sin_family = AF_INET;
        addr4.sin_addr.s_addr = (*ip_hdr(skb)).saddr;
        rc = netlbl_conn_setattr((*asoc).base.sk, &mut addr4 as *mut _ as *mut c_void, &mut secattr);
    } else if IS_ENABLED_CONFIG_IPV6() && (*ip_hdr(skb)).version == 6 {
        addr6.sin6_family = AF_INET6;
        addr6.sin6_addr = (*ipv6_hdr(skb)).saddr;
        rc = netlbl_conn_setattr((*asoc).base.sk, &mut addr6 as *mut _ as *mut c_void, &mut secattr);
    } else {
        rc = -EAFNOSUPPORT;
    }

    if rc == 0 {
        (*sksec).nlbl_state = NLBL_LABELED;
    }

    netlbl_secattr_destroy(&mut secattr);
    rc
}

/**
 * selinux_netlbl_inet_conn_request - Label an incoming stream connection
 * @req: incoming connection request socket
 * @family: the request socket's address family
 *
 * Description:
 * A new incoming connection request is represented by @req, we need to label
 * the new request_sock here and the stack will ensure the on-the-wire label
 * will get preserved when a full sock is created once the connection handshake
 * is complete.  Returns zero on success, negative values on failure.
 *
 */
#[no_mangle]
pub unsafe extern "C" fn selinux_netlbl_inet_conn_request(
    req: *mut request_sock,
    family: u16,
) -> c_int {
    let mut rc: c_int;
    let mut secattr: netlbl_lsm_secattr = core::mem::zeroed();

    if family != PF_INET && family != PF_INET6 {
        return 0;
    }

    netlbl_secattr_init(&mut secattr);
    rc = security_netlbl_sid_to_secattr((*req).secid, &mut secattr);
    if rc != 0 {
        netlbl_secattr_destroy(&mut secattr);
        return rc;
    }
    rc = netlbl_req_setattr(req, &mut secattr);
    netlbl_secattr_destroy(&mut secattr);
    rc
}

/**
 * selinux_netlbl_inet_csk_clone - Initialize the newly created sock
 * @sk: the new sock
 * @family: the sock's address family
 *
 * Description:
 * A new connection has been established using @sk, we've already labeled the
 * socket via the request_sock struct in selinux_netlbl_inet_conn_request() but
 * we need to set the NetLabel state here since we now have a sock structure.
 *
 */
#[no_mangle]
pub unsafe extern "C" fn selinux_netlbl_inet_csk_clone(sk: *mut sock, family: u16) {
    let sksec: *mut sk_security_struct = selinux_sock(sk);

    if family == PF_INET || family == PF_INET6 {
        (*sksec).nlbl_state = NLBL_LABELED;
    } else {
        (*sksec).nlbl_state = NLBL_UNSET;
    }
}

/**
 * selinux_netlbl_sctp_sk_clone - Copy state to the newly created sock
 * @sk: current sock
 * @newsk: the new sock
 *
 * Description:
 * Called whenever a new socket is created by accept(2) or sctp_peeloff(3).
 */
#[no_mangle]
pub unsafe extern "C" fn selinux_netlbl_sctp_sk_clone(sk: *mut sock, newsk: *mut sock) {
    let sksec: *mut sk_security_struct = selinux_sock(sk);
    let newsksec: *mut sk_security_struct = selinux_sock(newsk);

    (*newsksec).nlbl_state = (*sksec).nlbl_state;
}

/**
 * selinux_netlbl_socket_post_create - Label a socket using NetLabel
 * @sk: the sock to label
 * @family: protocol family
 *
 * Description:
 * Attempt to label a socket using the NetLabel mechanism using the given
 * SID.  Returns zero values on success, negative values on failure.
 *
 */
#[no_mangle]
pub unsafe extern "C" fn selinux_netlbl_socket_post_create(sk: *mut sock, family: u16) -> c_int {
    let mut rc: c_int;
    let sksec: *mut sk_security_struct = selinux_sock(sk);
    let secattr: *mut netlbl_lsm_secattr;

    if family != PF_INET && family != PF_INET6 {
        return 0;
    }

    secattr = selinux_netlbl_sock_genattr(sk);
    if IS_ERR(secattr) {
        return PTR_ERR(secattr);
    }
    /* On socket creation, replacement of IP options is safe even if
     * the caller does not hold the socket lock.
     */
    rc = netlbl_sock_setattr(sk, family, secattr, true);
    match rc {
        0 => {
            (*sksec).nlbl_state = NLBL_LABELED;
        }
        x if x == -EDESTADDRREQ => {
            (*sksec).nlbl_state = NLBL_REQSKB;
            rc = 0;
        }
        _ => {}
    }

    rc
}

/**
 * selinux_netlbl_sock_rcv_skb - Do an inbound access check using NetLabel
 * @sksec: the sock's sk_security_struct
 * @skb: the packet
 * @family: protocol family
 * @ad: the audit data
 *
 * Description:
 * Fetch the NetLabel security attributes from @skb and perform an access check
 * against the receiving socket.  Returns zero on success, negative values on
 * error.
 *
 */
#[no_mangle]
pub unsafe extern "C" fn selinux_netlbl_sock_rcv_skb(
    sksec: *mut sk_security_struct,
    skb: *mut sk_buff,
    family: u16,
    ad: *mut common_audit_data,
) -> c_int {
    let mut rc: c_int;
    let mut nlbl_sid: u32;
    let perm: u32;
    let mut secattr: netlbl_lsm_secattr = core::mem::zeroed();

    if !netlbl_enabled() {
        return 0;
    }

    netlbl_secattr_init(&mut secattr);
    rc = netlbl_skbuff_getattr(skb, family, &mut secattr);
    if rc == 0 && secattr.flags != NETLBL_SECATTR_NONE {
        rc = selinux_netlbl_sidlookup_cached(skb, family, &mut secattr, &mut nlbl_sid);
    } else {
        nlbl_sid = SECINITSID_UNLABELED;
    }
    netlbl_secattr_destroy(&mut secattr);
    if rc != 0 {
        return rc;
    }

    match (*sksec).sclass {
        SECCLASS_UDP_SOCKET => {
            perm = UDP_SOCKET__RECVFROM;
        }
        SECCLASS_TCP_SOCKET => {
            perm = TCP_SOCKET__RECVFROM;
        }
        _ => {
            perm = RAWIP_SOCKET__RECVFROM;
        }
    }

    rc = avc_has_perm((*sksec).sid, nlbl_sid, (*sksec).sclass, perm, ad);
    if rc == 0 {
        return 0;
    }

    if nlbl_sid != SECINITSID_UNLABELED {
        netlbl_skbuff_err(skb, family, rc, 0);
    }
    rc
}

/**
 * selinux_netlbl_option - Is this a NetLabel option
 * @level: the socket level or protocol
 * @optname: the socket option name
 *
 * Description:
 * Returns true if @level and @optname refer to a NetLabel option.
 * Helper for selinux_netlbl_socket_setsockopt().
 */
#[inline]
unsafe fn selinux_netlbl_option(level: c_int, optname: c_int) -> c_int {
    ((level == IPPROTO_IP && optname == IP_OPTIONS)
        || (level == IPPROTO_IPV6 && optname == IPV6_HOPOPTS)) as c_int
}

/**
 * selinux_netlbl_socket_setsockopt - Do not allow users to remove a NetLabel
 * @sock: the socket
 * @level: the socket level or protocol
 * @optname: the socket option name
 *
 * Description:
 * Check the setsockopt() call and if the user is trying to replace the IP
 * options on a socket and a NetLabel is in place for the socket deny the
 * access; otherwise allow the access.  Returns zero when the access is
 * allowed, -EACCES when denied, and other negative values on error.
 *
 */
#[no_mangle]
pub unsafe extern "C" fn selinux_netlbl_socket_setsockopt(
    sock: *mut socket,
    level: c_int,
    optname: c_int,
) -> c_int {
    let mut rc: c_int = 0;
    let sk: *mut sock = (*sock).sk;
    let sksec: *mut sk_security_struct = selinux_sock(sk);
    let mut secattr: netlbl_lsm_secattr = core::mem::zeroed();

    if selinux_netlbl_option(level, optname) != 0
        && ((*sksec).nlbl_state == NLBL_LABELED || (*sksec).nlbl_state == NLBL_CONNLABELED)
    {
        netlbl_secattr_init(&mut secattr);
        lock_sock(sk);
        /* call the netlabel function directly as we want to see the
         * on-the-wire label that is assigned via the socket's options
         * and not the cached netlabel/lsm attributes */
        rc = netlbl_sock_getattr(sk, &mut secattr);
        release_sock(sk);
        if rc == 0 {
            rc = -EACCES;
        } else if rc == -ENOMSG {
            rc = 0;
        }
        netlbl_secattr_destroy(&mut secattr);
    }

    rc
}

/**
 * selinux_netlbl_socket_connect_helper - Help label a client-side socket on
 * connect
 * @sk: the socket to label
 * @addr: the destination address
 *
 * Description:
 * Attempt to label a connected socket with NetLabel using the given address.
 * Returns zero values on success, negative values on failure.
 *
 */
unsafe fn selinux_netlbl_socket_connect_helper(sk: *mut sock, addr: *mut sockaddr) -> c_int {
    let rc: c_int;
    let sksec: *mut sk_security_struct = selinux_sock(sk);
    let secattr: *mut netlbl_lsm_secattr;

    /* connected sockets are allowed to disconnect when the address family
     * is set to AF_UNSPEC, if that is what is happening we want to reset
     * the socket */
    if (*addr).sa_family == AF_UNSPEC {
        netlbl_sock_delattr(sk);
        (*sksec).nlbl_state = NLBL_REQSKB;
        rc = 0;
        return rc;
    }
    secattr = selinux_netlbl_sock_genattr(sk);
    if IS_ERR(secattr) {
        return PTR_ERR(secattr);
    }

    rc = netlbl_conn_setattr(sk, addr as *mut c_void, secattr);
    if rc == 0 {
        (*sksec).nlbl_state = NLBL_CONNLABELED;
    }

    rc
}

/**
 * selinux_netlbl_socket_connect_locked - Label a client-side socket on
 * connect
 * @sk: the socket to label
 * @addr: the destination address
 *
 * Description:
 * Attempt to label a connected socket that already has the socket locked
 * with NetLabel using the given address.
 * Returns zero values on success, negative values on failure.
 *
 */
#[no_mangle]
pub unsafe extern "C" fn selinux_netlbl_socket_connect_locked(
    sk: *mut sock,
    addr: *mut sockaddr,
) -> c_int {
    let sksec: *mut sk_security_struct = selinux_sock(sk);

    if (*sksec).nlbl_state != NLBL_REQSKB && (*sksec).nlbl_state != NLBL_CONNLABELED {
        return 0;
    }

    selinux_netlbl_socket_connect_helper(sk, addr)
}

/**
 * selinux_netlbl_socket_connect - Label a client-side socket on connect
 * @sk: the socket to label
 * @addr: the destination address
 *
 * Description:
 * Attempt to label a connected socket with NetLabel using the given address.
 * Returns zero values on success, negative values on failure.
 *
 */
#[no_mangle]
pub unsafe extern "C" fn selinux_netlbl_socket_connect(
    sk: *mut sock,
    addr: *mut sockaddr,
) -> c_int {
    let rc: c_int;

    lock_sock(sk);
    rc = selinux_netlbl_socket_connect_locked(sk, addr);
    release_sock(sk);

    rc
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
