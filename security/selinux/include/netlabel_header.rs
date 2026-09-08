/* SPDX-License-Identifier: GPL-2.0-or-later */
/*
 * SELinux interface to the NetLabel subsystem
 *
 * Author: Paul Moore <paul@paul-moore.com>
 */

/*
 * (c) Copyright Hewlett-Packard Development Company, L.P., 2006
 */

/*
 * C dependencies:
 * <linux/types.h>, <linux/fs.h>, <linux/net.h>, <linux/skbuff.h>,
 * <net/sock.h>, <net/request_sock.h>, <net/sctp/structs.h>,
 * "avc.h", and "objsec.h".
 */

#[cfg(CONFIG_NETLABEL)]
unsafe extern "C" {
    pub fn selinux_netlbl_cache_invalidate();

    pub fn selinux_netlbl_err(
        skb: *mut sk_buff,
        family: u16,
        error: ::core::ffi::c_int,
        gateway: ::core::ffi::c_int,
    );

    pub fn selinux_netlbl_sk_security_free(sksec: *mut sk_security_struct);
    pub fn selinux_netlbl_sk_security_reset(sksec: *mut sk_security_struct);

    pub fn selinux_netlbl_skbuff_getsid(
        skb: *mut sk_buff,
        family: u16,
        type_: *mut u32,
        sid: *mut u32,
    ) -> ::core::ffi::c_int;
    pub fn selinux_netlbl_skbuff_setsid(
        skb: *mut sk_buff,
        family: u16,
        sid: u32,
    ) -> ::core::ffi::c_int;
    pub fn selinux_netlbl_sctp_assoc_request(
        asoc: *mut sctp_association,
        skb: *mut sk_buff,
    ) -> ::core::ffi::c_int;
    pub fn selinux_netlbl_inet_conn_request(
        req: *mut request_sock,
        family: u16,
    ) -> ::core::ffi::c_int;
    pub fn selinux_netlbl_inet_csk_clone(sk: *mut sock, family: u16);
    pub fn selinux_netlbl_sctp_sk_clone(sk: *mut sock, newsk: *mut sock);
    pub fn selinux_netlbl_socket_post_create(
        sk: *mut sock,
        family: u16,
    ) -> ::core::ffi::c_int;
    pub fn selinux_netlbl_sock_rcv_skb(
        sksec: *mut sk_security_struct,
        skb: *mut sk_buff,
        family: u16,
        ad: *mut common_audit_data,
    ) -> ::core::ffi::c_int;
    pub fn selinux_netlbl_socket_setsockopt(
        sock: *mut socket,
        level: ::core::ffi::c_int,
        optname: ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
    pub fn selinux_netlbl_socket_connect(
        sk: *mut sock,
        addr: *mut sockaddr,
    ) -> ::core::ffi::c_int;
    pub fn selinux_netlbl_socket_connect_locked(
        sk: *mut sock,
        addr: *mut sockaddr,
    ) -> ::core::ffi::c_int;
}

#[cfg(not(CONFIG_NETLABEL))]
pub unsafe fn selinux_netlbl_cache_invalidate() {
    return;
}

#[cfg(not(CONFIG_NETLABEL))]
pub unsafe fn selinux_netlbl_err(
    skb: *mut sk_buff,
    family: u16,
    error: ::core::ffi::c_int,
    gateway: ::core::ffi::c_int,
) {
    return;
}

#[cfg(not(CONFIG_NETLABEL))]
pub unsafe fn selinux_netlbl_sk_security_free(sksec: *mut sk_security_struct) {
    return;
}

#[cfg(not(CONFIG_NETLABEL))]
pub unsafe fn selinux_netlbl_sk_security_reset(sksec: *mut sk_security_struct) {
    return;
}

#[cfg(not(CONFIG_NETLABEL))]
pub unsafe fn selinux_netlbl_skbuff_getsid(
    skb: *mut sk_buff,
    family: u16,
    type_: *mut u32,
    sid: *mut u32,
) -> ::core::ffi::c_int {
    *type_ = NETLBL_NLTYPE_NONE;
    *sid = SECSID_NULL;
    return 0;
}

#[cfg(not(CONFIG_NETLABEL))]
pub unsafe fn selinux_netlbl_skbuff_setsid(
    skb: *mut sk_buff,
    family: u16,
    sid: u32,
) -> ::core::ffi::c_int {
    return 0;
}

#[cfg(not(CONFIG_NETLABEL))]
pub unsafe fn selinux_netlbl_sctp_assoc_request(
    asoc: *mut sctp_association,
    skb: *mut sk_buff,
) -> ::core::ffi::c_int {
    return 0;
}

#[cfg(not(CONFIG_NETLABEL))]
pub unsafe fn selinux_netlbl_inet_conn_request(
    req: *mut request_sock,
    family: u16,
) -> ::core::ffi::c_int {
    return 0;
}

#[cfg(not(CONFIG_NETLABEL))]
pub unsafe fn selinux_netlbl_inet_csk_clone(sk: *mut sock, family: u16) {
    return;
}

#[cfg(not(CONFIG_NETLABEL))]
pub unsafe fn selinux_netlbl_sctp_sk_clone(sk: *mut sock, newsk: *mut sock) {
    return;
}

#[cfg(not(CONFIG_NETLABEL))]
pub unsafe fn selinux_netlbl_socket_post_create(
    sk: *mut sock,
    family: u16,
) -> ::core::ffi::c_int {
    return 0;
}

#[cfg(not(CONFIG_NETLABEL))]
pub unsafe fn selinux_netlbl_sock_rcv_skb(
    sksec: *mut sk_security_struct,
    skb: *mut sk_buff,
    family: u16,
    ad: *mut common_audit_data,
) -> ::core::ffi::c_int {
    return 0;
}

#[cfg(not(CONFIG_NETLABEL))]
pub unsafe fn selinux_netlbl_socket_setsockopt(
    sock: *mut socket,
    level: ::core::ffi::c_int,
    optname: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    return 0;
}

#[cfg(not(CONFIG_NETLABEL))]
pub unsafe fn selinux_netlbl_socket_connect(
    sk: *mut sock,
    addr: *mut sockaddr,
) -> ::core::ffi::c_int {
    return 0;
}

#[cfg(not(CONFIG_NETLABEL))]
pub unsafe fn selinux_netlbl_socket_connect_locked(
    sk: *mut sock,
    addr: *mut sockaddr,
) -> ::core::ffi::c_int {
    return 0;
}


// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
