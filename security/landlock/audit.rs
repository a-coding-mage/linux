// SPDX-License-Identifier: GPL-2.0-only
/*
 * Landlock - Audit helpers
 *
 * Copyright (c) 2023-2025 Microsoft Corporation
 */

// Dependencies from the original C source:
// <linux/audit.h>, <linux/bitops.h>, <linux/landlock.h>,
// <linux/lsm_audit.h>, <linux/pid.h>, <uapi/linux/landlock.h>,
// "access.h", "audit.h", "common.h", "cred.h", "domain.h",
// "limits.h", and "log.h".

use core::ffi::{c_char, c_int, c_ulong, c_uint, c_ulonglong};

type bool_ = bool;
type access_mask_t = c_ulong;

#[repr(C)]
pub struct audit_buffer {
    _private: [u8; 0],
}

#[repr(C)]
pub struct audit_context {
    _private: [u8; 0],
}

#[repr(C)]
pub struct pid {
    _private: [u8; 0],
}

#[repr(C)]
pub struct landlock_hierarchy_details {
    pub pid: *mut pid,
    pub uid: c_uint,
    pub exe_path: *const c_char,
    pub comm: *const c_char,
}

#[repr(C)]
pub struct atomic64_t {
    _private: [u8; 0],
}

#[repr(C)]
pub struct landlock_hierarchy {
    pub log_status: c_int,
    pub id: c_ulonglong,
    pub details: *mut landlock_hierarchy_details,
    pub num_denials: atomic64_t,
}

#[repr(C)]
pub struct common_audit_data {
    _private: [u8; 0],
}

#[repr(C)]
pub struct landlock_request {
    pub type_: landlock_request_type,
    pub audit: common_audit_data,
}

#[repr(C)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum landlock_request_type {
    LANDLOCK_REQUEST_PTRACE,
    LANDLOCK_REQUEST_FS_CHANGE_TOPOLOGY,
    LANDLOCK_REQUEST_FS_ACCESS,
    LANDLOCK_REQUEST_NET_ACCESS,
    LANDLOCK_REQUEST_SCOPE_ABSTRACT_UNIX_SOCKET,
    LANDLOCK_REQUEST_SCOPE_SIGNAL,
}

extern "C" {
    static audit_enabled: bool_;

    static fs_access_strings: [*const c_char; LANDLOCK_NUM_ACCESS_FS as usize];
    static net_access_strings: [*const c_char; LANDLOCK_NUM_ACCESS_NET as usize];
    static scope_strings: [*const c_char; LANDLOCK_NUM_SCOPE as usize];

    static LANDLOCK_NUM_ACCESS_FS: c_ulong;
    static LANDLOCK_NUM_ACCESS_NET: c_ulong;
    static LANDLOCK_NUM_SCOPE: c_ulong;
    static LANDLOCK_SCOPE_ABSTRACT_UNIX_SOCKET: c_ulong;
    static LANDLOCK_SCOPE_SIGNAL: c_ulong;
    static LANDLOCK_LOG_RECORDED: c_int;

    static GFP_ATOMIC: c_uint;
    static __GFP_NOWARN: c_uint;
    static GFP_KERNEL: c_uint;
    static AUDIT_LANDLOCK_DOMAIN: c_int;
    static AUDIT_LANDLOCK_ACCESS: c_int;

    fn audit_context() -> *mut audit_context;
    fn audit_log_start(
        ctx: *mut audit_context,
        gfp_mask: c_uint,
        type_: c_int,
    ) -> *mut audit_buffer;
    fn audit_log_format(ab: *mut audit_buffer, fmt: *const c_char, ...);
    fn audit_log_untrustedstring(ab: *mut audit_buffer, string: *const c_char);
    fn audit_log_lsm_data(ab: *mut audit_buffer, audit: *const common_audit_data);
    fn audit_log_end(ab: *mut audit_buffer);
    fn pid_nr(pid: *mut pid) -> c_int;
    fn atomic64_read(v: *const atomic64_t) -> c_ulonglong;
    fn WARN_ON_ONCE(condition: bool_) -> bool_;
}

/*
 * Access-right and scope names are built from the lists shared with the trace
 * events (see <linux/landlock.h>).  The designated initializer places each name
 * at its bit index, so the lookup stays O(1) and does not depend on the entry
 * order.  log_blockers() adds the "fs."/"net."/"scope." category prefix.
 *
 * C macro translated intent:
 * #define _LANDLOCK_NAME_ENTRY(mask, name) [BIT_INDEX(mask)] = name
 *
 * The actual fs_access_strings, net_access_strings, and scope_strings tables
 * are supplied by external Landlock definitions in this isolated translation.
 */

unsafe fn BIT_INDEX(mask: c_ulong) -> c_ulong {
    mask.trailing_zeros() as c_ulong
}

unsafe fn READ_ONCE<T: Copy>(p: *const T) -> T {
    core::ptr::read_volatile(p)
}

unsafe fn WRITE_ONCE<T>(p: *mut T, value: T) {
    core::ptr::write_volatile(p, value);
}

unsafe fn ARRAY_SIZE_fs_access_strings() -> c_ulong {
    LANDLOCK_NUM_ACCESS_FS
}

unsafe fn ARRAY_SIZE_net_access_strings() -> c_ulong {
    LANDLOCK_NUM_ACCESS_NET
}

unsafe fn get_blocker(type_: landlock_request_type, access_bit: c_ulong) -> *const c_char {
    match type_ {
        landlock_request_type::LANDLOCK_REQUEST_PTRACE => {
            WARN_ON_ONCE(access_bit != (-1isize as c_ulong));
            c"ptrace".as_ptr()
        }

        landlock_request_type::LANDLOCK_REQUEST_FS_CHANGE_TOPOLOGY => {
            WARN_ON_ONCE(access_bit != (-1isize as c_ulong));
            c"change_topology".as_ptr()
        }

        landlock_request_type::LANDLOCK_REQUEST_FS_ACCESS => {
            if WARN_ON_ONCE(access_bit >= ARRAY_SIZE_fs_access_strings()) {
                return c"unknown".as_ptr();
            }
            fs_access_strings[access_bit as usize]
        }

        landlock_request_type::LANDLOCK_REQUEST_NET_ACCESS => {
            if WARN_ON_ONCE(access_bit >= ARRAY_SIZE_net_access_strings()) {
                return c"unknown".as_ptr();
            }
            net_access_strings[access_bit as usize]
        }

        landlock_request_type::LANDLOCK_REQUEST_SCOPE_ABSTRACT_UNIX_SOCKET => {
            WARN_ON_ONCE(access_bit != (-1isize as c_ulong));
            scope_strings[BIT_INDEX(LANDLOCK_SCOPE_ABSTRACT_UNIX_SOCKET) as usize]
        }

        landlock_request_type::LANDLOCK_REQUEST_SCOPE_SIGNAL => {
            WARN_ON_ONCE(access_bit != (-1isize as c_ulong));
            scope_strings[BIT_INDEX(LANDLOCK_SCOPE_SIGNAL) as usize]
        }
    }
}

/*
 * Returns the audit category prefix prepended to the unprefixed blocker name
 * returned by get_blocker() (filesystem and network access rights,
 * change_topology, and scopes).  The ptrace blocker is standalone and carries
 * its full name in get_blocker(), so it uses no prefix.
 */
unsafe fn blocker_prefix(type_: landlock_request_type) -> *const c_char {
    match type_ {
        landlock_request_type::LANDLOCK_REQUEST_PTRACE => c"".as_ptr(),

        landlock_request_type::LANDLOCK_REQUEST_FS_CHANGE_TOPOLOGY
        | landlock_request_type::LANDLOCK_REQUEST_FS_ACCESS => c"fs.".as_ptr(),

        landlock_request_type::LANDLOCK_REQUEST_NET_ACCESS => c"net.".as_ptr(),

        landlock_request_type::LANDLOCK_REQUEST_SCOPE_ABSTRACT_UNIX_SOCKET
        | landlock_request_type::LANDLOCK_REQUEST_SCOPE_SIGNAL => c"scope.".as_ptr(),
    }
}

unsafe fn log_blockers(
    ab: *mut audit_buffer,
    type_: landlock_request_type,
    access: access_mask_t,
) {
    let access_mask: c_ulong = access;
    let prefix: *const c_char = blocker_prefix(type_);
    let mut access_bit: c_ulong = 0;
    let mut is_first: bool_ = true;
    let bits_per_type_access = (core::mem::size_of::<access_mask_t>() * 8) as c_ulong;

    while access_bit < bits_per_type_access {
        if ((access_mask >> access_bit) & 1) != 0 {
            audit_log_format(
                ab,
                c"%s%s%s".as_ptr(),
                if is_first { c"".as_ptr() } else { c",".as_ptr() },
                prefix,
                get_blocker(type_, access_bit),
            );
            is_first = false;
        }
        access_bit = access_bit.wrapping_add(1);
    }
    if is_first {
        audit_log_format(
            ab,
            c"%s%s".as_ptr(),
            prefix,
            get_blocker(type_, -1isize as c_ulong),
        );
    }
}

unsafe fn log_domain(hierarchy: *mut landlock_hierarchy) {
    let mut ab: *mut audit_buffer;

    /* Ignores already logged domains.  */
    if READ_ONCE(core::ptr::addr_of!((*hierarchy).log_status)) == LANDLOCK_LOG_RECORDED {
        return;
    }

    /* Uses consistent allocation flags wrt common_lsm_audit(). */
    ab = audit_log_start(
        audit_context(),
        GFP_ATOMIC | __GFP_NOWARN,
        AUDIT_LANDLOCK_DOMAIN,
    );
    if ab.is_null() {
        return;
    }

    WARN_ON_ONCE((*hierarchy).id == 0);
    audit_log_format(
        ab,
        c"domain=%llx status=allocated mode=enforcing pid=%d uid=%u exe=".as_ptr(),
        (*hierarchy).id,
        pid_nr((*(*hierarchy).details).pid),
        (*(*hierarchy).details).uid,
    );
    audit_log_untrustedstring(ab, (*(*hierarchy).details).exe_path);
    audit_log_format(ab, c" comm=".as_ptr());
    audit_log_untrustedstring(ab, (*(*hierarchy).details).comm);
    audit_log_end(ab);

    /*
     * There may be race condition leading to logging of the same domain
     * several times but that is OK.
     */
    WRITE_ONCE(
        core::ptr::addr_of_mut!((*hierarchy).log_status),
        LANDLOCK_LOG_RECORDED,
    );
}

/**
 * landlock_audit_denial - Create an audit record for a denied access request
 *
 * @request: Detail of the user space request.
 * @youngest_denied: The youngest hierarchy node that denied the access.
 * @missing: The set of denied access rights.
 * @logged: Whether the denial is selected for logging, as computed by
 *          landlock_log_denial() (domain policy and quiet rules).
 *
 * Emits the record when audit is enabled and the denial is selected for
 * logging.
 */
#[no_mangle]
pub unsafe extern "C" fn landlock_audit_denial(
    request: *const landlock_request,
    youngest_denied: *mut landlock_hierarchy,
    missing: access_mask_t,
    logged: bool_,
) {
    let mut ab: *mut audit_buffer;

    if !audit_enabled {
        return;
    }

    /*
     * Skips denials the domain's policy or a quiet rule excludes from
     * logging (folded into @logged by landlock_log_denial()).
     */
    if !logged {
        return;
    }

    /* Uses consistent allocation flags wrt common_lsm_audit(). */
    ab = audit_log_start(
        audit_context(),
        GFP_ATOMIC | __GFP_NOWARN,
        AUDIT_LANDLOCK_ACCESS,
    );
    if ab.is_null() {
        return;
    }

    audit_log_format(
        ab,
        c"domain=%llx blockers=".as_ptr(),
        (*youngest_denied).id,
    );
    log_blockers(ab, (*request).type_, missing);
    audit_log_lsm_data(ab, core::ptr::addr_of!((*request).audit));
    audit_log_end(ab);

    /* Logs this domain the first time it shows in log. */
    log_domain(youngest_denied);
}

/**
 * landlock_audit_free_domain - Create an audit record on domain deallocation
 *
 * @hierarchy: The domain's hierarchy being deallocated.
 *
 * Only domains which previously appeared in the audit logs are logged again.
 * This is useful to know when a domain will never show again in the audit log.
 *
 * Called from landlock_log_free_domain().
 */
#[no_mangle]
pub unsafe extern "C" fn landlock_audit_free_domain(hierarchy: *const landlock_hierarchy) {
    let mut ab: *mut audit_buffer;

    if !audit_enabled {
        return;
    }

    /* Ignores domains that were not logged.  */
    if READ_ONCE(core::ptr::addr_of!((*hierarchy).log_status)) != LANDLOCK_LOG_RECORDED {
        return;
    }

    /*
     * If logging of domain allocation succeeded, warns about failure to log
     * domain deallocation to highlight unbalanced domain lifetime logs.
     */
    ab = audit_log_start(audit_context(), GFP_KERNEL, AUDIT_LANDLOCK_DOMAIN);
    if ab.is_null() {
        return;
    }

    audit_log_format(
        ab,
        c"domain=%llx status=deallocated denials=%llu".as_ptr(),
        (*hierarchy).id,
        atomic64_read(core::ptr::addr_of!((*hierarchy).num_denials)),
    );
    audit_log_end(ab);
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
