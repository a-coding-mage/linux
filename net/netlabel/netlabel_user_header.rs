/* SPDX-License-Identifier: GPL-2.0-or-later */
/*
 * NetLabel NETLINK Interface
 *
 * This file defines the NETLINK interface for the NetLabel system.  The
 * NetLabel system manages static and dynamic label mappings for network
 * protocols such as CIPSO and RIPSO.
 *
 * Author: Paul Moore <paul@paul-moore.com>
 */

/*
 * (c) Copyright Hewlett-Packard Development Company, L.P., 2006
 */

/* C header dependencies: linux/types.h, linux/skbuff.h, linux/capability.h,
 * linux/audit.h, net/netlink.h, net/genetlink.h, and net/netlabel.h. */

/* NetLabel NETLINK helper functions */

/**
 * netlbl_netlink_auditinfo - Fetch the audit information from a NETLINK msg
 * @audit_info: NetLabel audit information
 */
#[inline]
pub unsafe fn netlbl_netlink_auditinfo(audit_info: *mut crate::netlbl_audit) {
    crate::security_current_getlsmprop_subj(&mut (*audit_info).prop);
    (*audit_info).loginuid = crate::audit_get_loginuid(crate::current);
    (*audit_info).sessionid = crate::audit_get_sessionid(crate::current);
}

/* NetLabel NETLINK I/O functions */

extern "C" {
    pub fn netlbl_netlink_init() -> ::core::ffi::c_int;
}

/* NetLabel Audit Functions */

extern "C" {
    pub fn netlbl_audit_start_common(
        type_: ::core::ffi::c_int,
        audit_info: *mut crate::netlbl_audit,
    ) -> *mut crate::audit_buffer;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
