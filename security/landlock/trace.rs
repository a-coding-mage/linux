// SPDX-License-Identifier: GPL-2.0-only
/*
 * Landlock - Tracepoint helpers
 *
 * Copyright © 2025 Microsoft Corporation
 * Copyright © 2026 Cloudflare, Inc.
 */

/* C dependencies:
 * linux/cleanup.h, linux/dcache.h, linux/err.h, linux/fs.h,
 * linux/lsm_audit.h, net/sock.h
 *
 * Local dependencies:
 * access.h, domain.h, fs.h, log.h, ruleset.h, trace.h
 */

/*
 * Generates the tracepoint definitions in this translation unit.  The trace
 * event header dereferences the traced objects in TP_fast_assign, so the full
 * struct definitions (e.g. ruleset.h, domain.h) must be included before it.
 *
 * C used:
 *   #define CREATE_TRACE_POINTS
 *   #include <trace/events/landlock.h>
 */

pub type access_mask_t = u64;

pub const LANDLOCK_LOG_UNCOMMITTED: i32 = 0;

pub const LANDLOCK_REQUEST_FS_ACCESS: i32 = 0;
pub const LANDLOCK_REQUEST_FS_CHANGE_TOPOLOGY: i32 = 1;
pub const LANDLOCK_REQUEST_NET_ACCESS: i32 = 2;
pub const LANDLOCK_REQUEST_PTRACE: i32 = 3;
pub const LANDLOCK_REQUEST_SCOPE_SIGNAL: i32 = 4;
pub const LANDLOCK_REQUEST_SCOPE_ABSTRACT_UNIX_SOCKET: i32 = 5;

pub const LSM_AUDIT_DATA_FILE: i32 = 0;
pub const LSM_AUDIT_DATA_IOCTL_OP: i32 = 1;
pub const LSM_AUDIT_DATA_DENTRY: i32 = 2;
pub const LSM_AUDIT_DATA_PATH: i32 = 3;

pub const PATH_MAX: usize = 4096;
pub const ENAMETOOLONG: isize = 36;

#[repr(C)]
pub struct landlock_hierarchy {
    pub log_status: i32,
}

#[repr(C)]
pub struct landlock_request {
    pub type_: i32,
    pub audit: common_audit_data,
    pub other_domain_id: u64,
}

#[repr(C)]
pub struct common_audit_data {
    pub type_: i32,
    pub u: common_audit_data_union,
}

#[repr(C)]
pub union common_audit_data_union {
    pub file: *const file,
    pub op: *const lsm_ioctlop_audit,
    pub dentry: *mut dentry,
    pub path: path,
    pub net: *const lsm_network_audit,
    pub tsk: *mut task_struct,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct path {
    pub mnt: *mut vfsmount,
    pub dentry: *mut dentry,
}

#[repr(C)]
pub struct file {
    pub f_path: path,
}

#[repr(C)]
pub struct lsm_ioctlop_audit {
    pub path: path,
}

#[repr(C)]
pub struct lsm_network_audit {
    pub sk: *mut sock,
    pub sport: u16,
    pub dport: u16,
}

#[repr(C)]
pub struct dentry {
    _private: [u8; 0],
}

#[repr(C)]
pub struct vfsmount {
    _private: [u8; 0],
}

#[repr(C)]
pub struct sock {
    _private: [u8; 0],
}

#[repr(C)]
pub struct task_struct {
    _private: [u8; 0],
}

extern "C" {
    fn trace_landlock_free_domain(hierarchy: *const landlock_hierarchy);

    fn trace_landlock_deny_access_fs_enabled() -> bool;
    fn trace_landlock_deny_access_fs(
        youngest_denied: *const landlock_hierarchy,
        same_exec: bool,
        logged: bool,
        missing: access_mask_t,
        path: *const path,
        pathname: *const core::ffi::c_char,
    );

    fn trace_landlock_deny_access_net_enabled() -> bool;
    fn trace_landlock_deny_access_net(
        youngest_denied: *const landlock_hierarchy,
        same_exec: bool,
        logged: bool,
        missing: access_mask_t,
        sk: *mut sock,
        sport: u16,
        dport: u16,
    );

    fn trace_landlock_deny_ptrace_enabled() -> bool;
    fn trace_landlock_deny_ptrace(
        youngest_denied: *const landlock_hierarchy,
        same_exec: bool,
        logged: bool,
        other_domain_id: u64,
        tsk: *mut task_struct,
    );

    fn trace_landlock_deny_scope_signal_enabled() -> bool;
    fn trace_landlock_deny_scope_signal(
        youngest_denied: *const landlock_hierarchy,
        same_exec: bool,
        logged: bool,
        other_domain_id: u64,
        tsk: *mut task_struct,
    );

    fn trace_landlock_deny_scope_abstract_unix_socket_enabled() -> bool;
    fn trace_landlock_deny_scope_abstract_unix_socket(
        youngest_denied: *const landlock_hierarchy,
        same_exec: bool,
        logged: bool,
        other_domain_id: u64,
        sk: *mut sock,
    );

    fn __getname() -> *mut core::ffi::c_char;
    fn __putname(name: *mut core::ffi::c_char);
    fn dentry_path_raw(
        dentry: *mut dentry,
        buf: *mut core::ffi::c_char,
        buflen: usize,
    ) -> *const core::ffi::c_char;
    fn resolve_path_for_trace(path: *const path, buf: *mut core::ffi::c_char)
        -> *const core::ffi::c_char;
    fn WARN_ONCE(condition: i32, fmt: *const core::ffi::c_char, ...);
    fn IS_ERR(ptr: *const core::ffi::c_char) -> bool;
    fn PTR_ERR(ptr: *const core::ffi::c_char) -> isize;
}

#[inline]
unsafe fn READ_ONCE_i32(src: *const i32) -> i32 {
    core::ptr::read_volatile(src)
}

#[inline]
fn ntohs(value: u16) -> u16 {
    u16::from_be(value)
}

const NO_MEM: &[u8] = b"<no_mem>\0";
const TOO_LONG: &[u8] = b"<too_long>\0";
const UNREACHABLE: &[u8] = b"<unreachable>\0";
const UNHANDLED_FS_AUDIT_TYPE: &[u8] = b"Unhandled Landlock FS audit type %d\0";
const UNHANDLED_REQUEST_TYPE: &[u8] = b"Unhandled Landlock request type %d\0";

/**
 * landlock_trace_free_domain - Emit a tracepoint on domain deallocation
 *
 * @hierarchy: The domain's hierarchy being deallocated.
 *
 * Fires only for a hierarchy whose creation event was emitted, i.e. one that
 * left LANDLOCK_LOG_UNCOMMITTED in landlock_restrict_self().  This keeps the
 * create/free pair balanced: a hierarchy that never became observable is freed
 * silently, while a domain that landlock_restrict_self() created and a
 * thread-sync failure then aborted still fires free_domain, because its
 * creation event already fired.
 *
 * Called from landlock_log_free_domain().
 */
#[no_mangle]
pub unsafe extern "C" fn landlock_trace_free_domain(hierarchy: *const landlock_hierarchy) {
    /*
     * The log_status read is a correctness guard (keep the create/free pair
     * balanced), not a cost guard, so this cold path needs no
     * trace_..._enabled() check: the tracepoint is a static-branch no-op
     * when disabled.  The denial path guards trace_..._enabled() instead
     * because it does expensive __getname()/path work before emitting.
     */
    if READ_ONCE_i32(core::ptr::addr_of!((*hierarchy).log_status)) != LANDLOCK_LOG_UNCOMMITTED {
        trace_landlock_free_domain(hierarchy);
    }
}

/**
 * landlock_trace_denial - Emit a tracepoint for a denied access request
 *
 * @request: Detail of the user space request.
 * @youngest_denied: The youngest hierarchy node that denied the access.
 * @missing: The set of denied access rights.
 * @same_exec: Whether the current task is the same executable that called
 *             landlock_restrict_self() for the denying domain, as computed
 *             by landlock_log_denial().
 * @logged: Whether the domain's policy selects this denial for logging, as
 *          computed by landlock_log_denial().
 *
 * Emits the tracepoint matching @request->type when its event is enabled.
 * Unlike audit, fires regardless of @logged; the value is recorded in the event
 * so consumers can filter on it.
 *
 * Called from landlock_log_denial().
 */
#[no_mangle]
pub unsafe extern "C" fn landlock_trace_denial(
    request: *const landlock_request,
    youngest_denied: *const landlock_hierarchy,
    missing: access_mask_t,
    same_exec: bool,
    logged: bool,
) {
    match (*request).type_ {
        LANDLOCK_REQUEST_FS_ACCESS | LANDLOCK_REQUEST_FS_CHANGE_TOPOLOGY => {
            if trace_landlock_deny_access_fs_enabled() {
                let buf = __getname();
                let mut dentry_path = path {
                    mnt: core::ptr::null_mut(),
                    dentry: core::ptr::null_mut(),
                };
                let pathname: *const core::ffi::c_char;
                let mut path_ptr: *const path = core::ptr::null();

                /*
                 * Selects the path from the audit data type, as
                 * dump_common_audit_data() does.  A FS_ACCESS denial
                 * carries a file (hook_file_truncate) or an ioctl op
                 * (hook_file_ioctl) rather than a path;
                 * FS_CHANGE_TOPOLOGY carries a path or a bare dentry.
                 * Reading the wrong union member would dereference
                 * garbage, so every reachable type is handled here.
                 */
                match (*request).audit.type_ {
                    LSM_AUDIT_DATA_FILE => {
                        path_ptr = core::ptr::addr_of!((*(*request).audit.u.file).f_path);
                    }
                    LSM_AUDIT_DATA_IOCTL_OP => {
                        path_ptr = core::ptr::addr_of!((*(*request).audit.u.op).path);
                    }
                    LSM_AUDIT_DATA_DENTRY => {
                        /*
                         * Build a path on the stack with the real
                         * dentry so TP_fast_assign can extract dev and
                         * ino; the mnt field is unused there.
                         */
                        dentry_path = path {
                            mnt: core::ptr::null_mut(),
                            dentry: (*request).audit.u.dentry,
                        };
                        path_ptr = core::ptr::addr_of!(dentry_path);
                    }
                    LSM_AUDIT_DATA_PATH => {
                        path_ptr = core::ptr::addr_of!((*request).audit.u.path);
                    }
                    _ => {
                        WARN_ONCE(
                            1,
                            UNHANDLED_FS_AUDIT_TYPE.as_ptr() as *const core::ffi::c_char,
                            (*request).audit.type_,
                        );
                    }
                }

                if path_ptr.is_null() {
                    if !buf.is_null() {
                        __putname(buf);
                    }
                    return;
                }

                if buf.is_null() {
                    pathname = NO_MEM.as_ptr() as *const core::ffi::c_char;
                } else if (*request).audit.type_ == LSM_AUDIT_DATA_DENTRY {
                    /* No vfsmount: render the dentry path alone. */
                    let raw_pathname = dentry_path_raw((*request).audit.u.dentry, buf, PATH_MAX);
                    if IS_ERR(raw_pathname) {
                        pathname = if PTR_ERR(raw_pathname) == -ENAMETOOLONG {
                            TOO_LONG.as_ptr() as *const core::ffi::c_char
                        } else {
                            UNREACHABLE.as_ptr() as *const core::ffi::c_char
                        };
                    } else {
                        pathname = raw_pathname;
                    }
                } else {
                    pathname = resolve_path_for_trace(path_ptr, buf);
                }

                trace_landlock_deny_access_fs(
                    youngest_denied,
                    same_exec,
                    logged,
                    missing,
                    path_ptr,
                    pathname,
                );

                if !buf.is_null() {
                    __putname(buf);
                }
            }
        }
        LANDLOCK_REQUEST_NET_ACCESS => {
            if trace_landlock_deny_access_net_enabled() {
                trace_landlock_deny_access_net(
                    youngest_denied,
                    same_exec,
                    logged,
                    missing,
                    (*(*request).audit.u.net).sk,
                    ntohs((*(*request).audit.u.net).sport),
                    ntohs((*(*request).audit.u.net).dport),
                );
            }
        }
        LANDLOCK_REQUEST_PTRACE => {
            if trace_landlock_deny_ptrace_enabled() {
                trace_landlock_deny_ptrace(
                    youngest_denied,
                    same_exec,
                    logged,
                    (*request).other_domain_id,
                    (*request).audit.u.tsk,
                );
            }
        }
        LANDLOCK_REQUEST_SCOPE_SIGNAL => {
            if trace_landlock_deny_scope_signal_enabled() {
                trace_landlock_deny_scope_signal(
                    youngest_denied,
                    same_exec,
                    logged,
                    (*request).other_domain_id,
                    (*request).audit.u.tsk,
                );
            }
        }
        LANDLOCK_REQUEST_SCOPE_ABSTRACT_UNIX_SOCKET => {
            if trace_landlock_deny_scope_abstract_unix_socket_enabled() {
                trace_landlock_deny_scope_abstract_unix_socket(
                    youngest_denied,
                    same_exec,
                    logged,
                    (*request).other_domain_id,
                    (*(*request).audit.u.net).sk,
                );
            }
        }
        _ => {
            WARN_ONCE(
                1,
                UNHANDLED_REQUEST_TYPE.as_ptr() as *const core::ffi::c_char,
                (*request).type_,
            );
        }
    }
}


// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
