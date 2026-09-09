/* SPDX-License-Identifier: GPL-2.0-or-later */
/*
 * NetLabel Management Support
 *
 * This file defines the management functions for the NetLabel system.  The
 * NetLabel system manages static and dynamic label mappings for network
 * protocols such as CIPSO and RIPSO.
 *
 * Author: Paul Moore <paul@paul-moore.com>
 */

/*
 * (c) Copyright Hewlett-Packard Development Company, L.P., 2006
 */

/*
 * The following NetLabel payloads are supported by the management interface.
 *
 * The detailed ADD, REMOVE, LISTALL, ADDDEF, REMOVEDEF, LISTDEF, PROTOCOLS,
 * and VERSION payload documentation from the C header is retained here as
 * comments; the declarations below are the file-local interface.
 */

/* NetLabel Management commands */
#[repr(i32)]
#[allow(non_camel_case_types)]
pub enum netlbl_mgmt_commands {
    NLBL_MGMT_C_UNSPEC,
    NLBL_MGMT_C_ADD,
    NLBL_MGMT_C_REMOVE,
    NLBL_MGMT_C_LISTALL,
    NLBL_MGMT_C_ADDDEF,
    NLBL_MGMT_C_REMOVEDEF,
    NLBL_MGMT_C_LISTDEF,
    NLBL_MGMT_C_PROTOCOLS,
    NLBL_MGMT_C_VERSION,
    __NLBL_MGMT_C_MAX,
}

/* NetLabel Management attributes */
#[repr(i32)]
#[allow(non_camel_case_types)]
pub enum netlbl_mgmt_attributes {
    NLBL_MGMT_A_UNSPEC,
    NLBL_MGMT_A_DOMAIN,
    /* (NLA_NUL_STRING): the NULL terminated LSM domain string */
    NLBL_MGMT_A_PROTOCOL,
    /* (NLA_U32): the NetLabel protocol type (defined by NETLBL_NLTYPE_*) */
    NLBL_MGMT_A_VERSION,
    /* (NLA_U32): the NetLabel protocol version number (defined by NETLBL_PROTO_VERSION) */
    NLBL_MGMT_A_CV4DOI,
    /* (NLA_U32): the CIPSOv4 DOI value */
    NLBL_MGMT_A_IPV6ADDR,
    /* (NLA_BINARY, struct in6_addr): an IPv6 address */
    NLBL_MGMT_A_IPV6MASK,
    /* (NLA_BINARY, struct in6_addr): an IPv6 address mask */
    NLBL_MGMT_A_IPV4ADDR,
    /* (NLA_BINARY, struct in_addr): an IPv4 address */
    NLBL_MGMT_A_IPV4MASK,
    /* (NLA_BINARY, struct in_addr): an IPv4 address mask */
    NLBL_MGMT_A_ADDRSELECTOR,
    /*
     * (NLA_NESTED): an IP address selector, must contain an address, mask,
     * and protocol attribute plus any protocol specific attributes
     */
    NLBL_MGMT_A_SELECTORLIST,
    /*
     * (NLA_NESTED): the selector list, there must be at least one
     * NLBL_MGMT_A_ADDRSELECTOR attribute
     */
    NLBL_MGMT_A_FAMILY,
    /* (NLA_U16): The address family */
    NLBL_MGMT_A_CLPDOI,
    /* (NLA_U32): the CALIPSO DOI value */
    __NLBL_MGMT_A_MAX,
}

pub const NLBL_MGMT_A_MAX: i32 = __NLBL_MGMT_A_MAX as i32 - 1;

/* NetLabel protocol functions */
extern "C" {
    pub fn netlbl_mgmt_genl_init() -> i32;
}

/* NetLabel configured protocol reference counter */
extern "C" {
    pub static mut netlabel_mgmt_protocount: atomic_t;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
