/* SPDX-License-Identifier: GPL-2.0-or-later */
/*
 * NetLabel Unlabeled Support
 *
 * This file defines functions for dealing with unlabeled packets for the
 * NetLabel system.  The NetLabel system manages static and dynamic label
 * mappings for network protocols such as CIPSO and RIPSO.
 *
 * Author: Paul Moore <paul@paul-moore.com>
 */

/*
 * (c) Copyright Hewlett-Packard Development Company, L.P., 2006
 */

/* Dependency supplied by the surrounding translation unit/environment:
 * <net/netlabel.h>
 */

/*
 * The following NetLabel payloads are supported by the Unlabeled subsystem.
 *
 * o STATICADD
 *   This message is sent from an application to add a new static label for
 *   incoming unlabeled connections.
 *
 *   Required attributes:
 *
 *     NLBL_UNLABEL_A_IFACE
 *     NLBL_UNLABEL_A_SECCTX
 *
 *   If IPv4 is specified the following attributes are required:
 *
 *     NLBL_UNLABEL_A_IPV4ADDR
 *     NLBL_UNLABEL_A_IPV4MASK
 *
 *   If IPv6 is specified the following attributes are required:
 *
 *     NLBL_UNLABEL_A_IPV6ADDR
 *     NLBL_UNLABEL_A_IPV6MASK
 *
 * o STATICREMOVE
 *   This message is sent from an application to remove an existing static
 *   label for incoming unlabeled connections.
 *
 *   Required attributes:
 *
 *     NLBL_UNLABEL_A_IFACE
 *
 *   If IPv4 is specified the following attributes are required:
 *
 *     NLBL_UNLABEL_A_IPV4ADDR
 *     NLBL_UNLABEL_A_IPV4MASK
 *
 *   If IPv6 is specified the following attributes are required:
 *
 *     NLBL_UNLABEL_A_IPV6ADDR
 *     NLBL_UNLABEL_A_IPV6MASK
 *
 * o STATICLIST
 *   This message can be sent either from an application or by the kernel in
 *   response to an application generated STATICLIST message.  When sent by
 *   an application there is no payload and the NLM_F_DUMP flag should be set.
 *   The kernel should response with a series of the following messages.
 *
 *   Required attributes:
 *
 *     NLBL_UNLABEL_A_IFACE
 *     NLBL_UNLABEL_A_SECCTX
 *
 *   If IPv4 is specified the following attributes are required:
 *
 *     NLBL_UNLABEL_A_IPV4ADDR
 *     NLBL_UNLABEL_A_IPV4MASK
 *
 *   If IPv6 is specified the following attributes are required:
 *
 *     NLBL_UNLABEL_A_IPV6ADDR
 *     NLBL_UNLABEL_A_IPV6MASK
 *
 * o STATICADDDEF
 *   This message is sent from an application to set the default static
 *   label for incoming unlabeled connections.
 *
 *   Required attribute:
 *
 *     NLBL_UNLABEL_A_SECCTX
 *
 *   If IPv4 is specified the following attributes are required:
 *
 *     NLBL_UNLABEL_A_IPV4ADDR
 *     NLBL_UNLABEL_A_IPV4MASK
 *
 *   If IPv6 is specified the following attributes are required:
 *
 *     NLBL_UNLABEL_A_IPV6ADDR
 *     NLBL_UNLABEL_A_IPV6MASK
 *
 * o STATICREMOVEDEF
 *   This message is sent from an application to remove the existing default
 *   static label for incoming unlabeled connections.
 *
 *   If IPv4 is specified the following attributes are required:
 *
 *     NLBL_UNLABEL_A_IPV4ADDR
 *     NLBL_UNLABEL_A_IPV4MASK
 *
 *   If IPv6 is specified the following attributes are required:
 *
 *     NLBL_UNLABEL_A_IPV6ADDR
 *     NLBL_UNLABEL_A_IPV6MASK
 *
 * o STATICLISTDEF
 *   This message can be sent either from an application or by the kernel in
 *   response to an application generated STATICLISTDEF message.  When sent by
 *   an application there is no payload and the NLM_F_DUMP flag should be set.
 *   The kernel should response with the following message.
 *
 *   Required attribute:
 *
 *     NLBL_UNLABEL_A_SECCTX
 *
 *   If IPv4 is specified the following attributes are required:
 *
 *     NLBL_UNLABEL_A_IPV4ADDR
 *     NLBL_UNLABEL_A_IPV4MASK
 *
 *   If IPv6 is specified the following attributes are required:
 *
 *     NLBL_UNLABEL_A_IPV6ADDR
 *     NLBL_UNLABEL_A_IPV6MASK
 *
 * o ACCEPT
 *   This message is sent from an application to specify if the kernel should
 *   allow unlabled packets to pass if they do not match any of the static
 *   mappings defined in the unlabeled module.
 *
 *   Required attributes:
 *
 *     NLBL_UNLABEL_A_ACPTFLG
 *
 * o LIST
 *   This message can be sent either from an application or by the kernel in
 *   response to an application generated LIST message.  When sent by an
 *   application there is no payload.  The kernel should respond to a LIST
 *   message with a LIST message on success.
 *
 *   Required attributes:
 *
 *     NLBL_UNLABEL_A_ACPTFLG
 *
 */

/* NetLabel Unlabeled commands */
#[repr(i32)]
pub enum NetlblUnlabelCommand {
    NLBL_UNLABEL_C_UNSPEC,
    NLBL_UNLABEL_C_ACCEPT,
    NLBL_UNLABEL_C_LIST,
    NLBL_UNLABEL_C_STATICADD,
    NLBL_UNLABEL_C_STATICREMOVE,
    NLBL_UNLABEL_C_STATICLIST,
    NLBL_UNLABEL_C_STATICADDDEF,
    NLBL_UNLABEL_C_STATICREMOVEDEF,
    NLBL_UNLABEL_C_STATICLISTDEF,
    __NLBL_UNLABEL_C_MAX,
}

/* NetLabel Unlabeled attributes */
#[repr(i32)]
pub enum NetlblUnlabelAttribute {
    NLBL_UNLABEL_A_UNSPEC,
    NLBL_UNLABEL_A_ACPTFLG,
    /* (NLA_U8)
     * if true then unlabeled packets are allowed to pass, else unlabeled
     * packets are rejected */
    NLBL_UNLABEL_A_IPV6ADDR,
    /* (NLA_BINARY, struct in6_addr)
     * an IPv6 address */
    NLBL_UNLABEL_A_IPV6MASK,
    /* (NLA_BINARY, struct in6_addr)
     * an IPv6 address mask */
    NLBL_UNLABEL_A_IPV4ADDR,
    /* (NLA_BINARY, struct in_addr)
     * an IPv4 address */
    NLBL_UNLABEL_A_IPV4MASK,
    /* (NLA_BINARY, struct in_addr)
     * and IPv4 address mask */
    NLBL_UNLABEL_A_IFACE,
    /* (NLA_NULL_STRING)
     * network interface */
    NLBL_UNLABEL_A_SECCTX,
    /* (NLA_BINARY)
     * a LSM specific security context */
    __NLBL_UNLABEL_A_MAX,
}

pub const NLBL_UNLABEL_A_MAX: i32 = NetlblUnlabelAttribute::__NLBL_UNLABEL_A_MAX as i32 - 1;

/* NetLabel protocol functions */
extern "C" {
    pub fn netlbl_unlabel_genl_init() -> i32;

    /* General Unlabeled init function */
    pub fn netlbl_unlabel_init(size: u32) -> i32;

    /* Static/Fallback label management functions */
    pub fn netlbl_unlhsh_add(
        net: *mut net,
        dev_name: *const core::ffi::c_char,
        addr: *const core::ffi::c_void,
        mask: *const core::ffi::c_void,
        addr_len: u32,
        secid: u32,
        audit_info: *mut netlbl_audit,
    ) -> i32;
    pub fn netlbl_unlhsh_remove(
        net: *mut net,
        dev_name: *const core::ffi::c_char,
        addr: *const core::ffi::c_void,
        mask: *const core::ffi::c_void,
        addr_len: u32,
        audit_info: *mut netlbl_audit,
    ) -> i32;

    /* Process Unlabeled incoming network packets */
    pub fn netlbl_unlabel_getattr(
        skb: *const sk_buff,
        family: u16,
        secattr: *mut netlbl_lsm_secattr,
    ) -> i32;

    /* Set the default configuration to allow Unlabeled packets */
    pub fn netlbl_unlabel_defconf() -> i32;
}

/* Unlabeled connection hash table size */
/* XXX - currently this number is an uneducated guess */
pub const NETLBL_UNLHSH_BITSIZE: u32 = 7;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
