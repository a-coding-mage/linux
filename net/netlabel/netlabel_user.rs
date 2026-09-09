// SPDX-License-Identifier: GPL-2.0-or-later
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

// The declarations supplied by the Linux kernel headers and NetLabel headers
// remain external dependencies of this translation.

extern "C" {
    pub fn netlbl_mgmt_genl_init() -> ::std::os::raw::c_int;
    pub fn netlbl_cipsov4_genl_init() -> ::std::os::raw::c_int;
    pub fn netlbl_calipso_genl_init() -> ::std::os::raw::c_int;
    pub fn netlbl_unlabel_genl_init() -> ::std::os::raw::c_int;

    pub static mut audit_enabled: ::std::os::raw::c_int;
    pub fn audit_context() -> *mut audit_context;
    pub fn audit_log_start(
        context: *mut audit_context,
        gfp_mask: ::std::os::raw::c_uint,
        msg_type: ::std::os::raw::c_int,
    ) -> *mut audit_buffer;
    pub fn audit_log_format(buffer: *mut audit_buffer, format: *const ::std::os::raw::c_char, ...);
    pub fn audit_log_subj_ctx(buffer: *mut audit_buffer, prop: *const lsm_prop);
    pub fn from_kuid(user_ns: *const user_namespace, uid: kuid_t) -> ::std::os::raw::c_uint;

    pub static init_user_ns: user_namespace;
}

#[repr(C)]
pub struct audit_buffer {
    _private: [u8; 0],
}

#[repr(C)]
pub struct audit_context {
    _private: [u8; 0],
}

#[repr(C)]
pub struct user_namespace {
    _private: [u8; 0],
}

#[repr(C)]
pub struct lsm_prop {
    _private: [u8; 0],
}

pub type kuid_t = u32;

#[repr(C)]
pub struct netlbl_audit {
    pub loginuid: kuid_t,
    pub sessionid: ::std::os::raw::c_uint,
    pub prop: lsm_prop,
}

pub const AUDIT_OFF: ::std::os::raw::c_int = 0;
pub const GFP_ATOMIC: ::std::os::raw::c_uint = 0;

/*
 * NetLabel NETLINK Setup Functions
 */

/**
 * netlbl_netlink_init - Initialize the NETLINK communication channel
 *
 * Description:
 * Call out to the NetLabel components so they can register their families and
 * commands with the Generic NETLINK mechanism.  Returns zero on success and
 * non-zero on failure.
 *
 */
pub unsafe fn netlbl_netlink_init() -> ::std::os::raw::c_int {
    let mut ret_val: ::std::os::raw::c_int;

    ret_val = netlbl_mgmt_genl_init();
    if ret_val != 0 {
        return ret_val;
    }

    ret_val = netlbl_cipsov4_genl_init();
    if ret_val != 0 {
        return ret_val;
    }

    ret_val = netlbl_calipso_genl_init();
    if ret_val != 0 {
        return ret_val;
    }

    netlbl_unlabel_genl_init()
}

/*
 * NetLabel Audit Functions
 */

/**
 * netlbl_audit_start_common - Start an audit message
 * @type: audit message type
 * @audit_info: NetLabel audit information
 *
 * Description:
 * Start an audit message using the type specified in @type and fill the audit
 * message with some fields common to all NetLabel audit messages.  Returns
 * a pointer to the audit buffer on success, NULL on failure.
 *
 */
pub unsafe fn netlbl_audit_start_common(
    type_: ::std::os::raw::c_int,
    audit_info: *mut netlbl_audit,
) -> *mut audit_buffer {
    let audit_buf: *mut audit_buffer;

    if audit_enabled == AUDIT_OFF {
        return ::std::ptr::null_mut();
    }

    audit_buf = audit_log_start(audit_context(), GFP_ATOMIC, type_);
    if audit_buf.is_null() {
        return ::std::ptr::null_mut();
    }

    audit_log_format(
        audit_buf,
        b"netlabel: auid=%u ses=%u\0".as_ptr() as *const ::std::os::raw::c_char,
        from_kuid(&init_user_ns, (*audit_info).loginuid),
        (*audit_info).sessionid,
    );
    audit_log_subj_ctx(audit_buf, &(*audit_info).prop);

    audit_buf
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
