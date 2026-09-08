// SPDX-License-Identifier: GPL-2.0-only
/*
 * Landlock - Ptrace and scope hooks
 *
 * Copyright © 2017-2020 Mickaël Salaün <mic@digikod.net>
 * Copyright © 2019-2020 ANSSI
 * Copyright © 2024-2025 Microsoft Corporation
 */

// C dependencies:
// <asm/current.h>
// <linux/cleanup.h>
// <linux/cred.h>
// <linux/errno.h>
// <linux/kernel.h>
// <linux/lsm_audit.h>
// <linux/lsm_hooks.h>
// <linux/rcupdate.h>
// <linux/sched.h>
// <linux/sched/signal.h>
// <net/af_unix.h>
// <net/sock.h>
// "common.h"
// "cred.h"
// "domain.h"
// "fs.h"
// "log.h"
// "ruleset.h"
// "setup.h"
// "task.h"

#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(dead_code)]
#![allow(improper_ctypes)]

use core::ffi::{c_int, c_uint, c_ulonglong};
use core::mem::offset_of;
use core::ptr;

type bool_ = bool;
type u64 = c_ulonglong;
type size_t = usize;
type access_mask_t = u64;

const EPERM: c_int = 1;
const PTRACE_MODE_NOAUDIT: c_uint = 0;
const LANDLOCK_SCOPE_ABSTRACT_UNIX_SOCKET: access_mask_t = 0;
const LANDLOCK_SCOPE_SIGNAL: access_mask_t = 0;
const LANDLOCK_MAX_NUM_LAYERS: c_int = 0;
const LANDLOCK_REQUEST_PTRACE: c_int = 0;
const LANDLOCK_REQUEST_SCOPE_ABSTRACT_UNIX_SOCKET: c_int = 0;
const LANDLOCK_REQUEST_SCOPE_SIGNAL: c_int = 0;
const LSM_AUDIT_DATA_TASK: c_int = 0;
const LSM_AUDIT_DATA_NET: c_int = 0;

#[repr(C)]
struct landlock_hierarchy {
    parent: *const landlock_hierarchy,
    id: u64,
}

#[repr(C)]
struct landlock_domain {
    hierarchy: *const landlock_hierarchy,
    num_layers: c_int,
}

#[repr(C)]
struct landlock_cred_security {
    domain: *const landlock_domain,
}

#[repr(C)]
struct access_masks {
    scope: access_mask_t,
}

#[repr(C)]
struct task_struct {
    _private: [u8; 0],
}

#[repr(C)]
struct cred {
    _private: [u8; 0],
}

#[repr(C)]
struct kernel_siginfo {
    _private: [u8; 0],
}

#[repr(C)]
struct pid {
    _private: [u8; 0],
}

#[repr(C)]
struct file {
    f_cred: *const cred,
}

#[repr(C)]
struct socket {
    file: *mut file,
    sk: *mut sock,
}

#[repr(C)]
struct sock {
    sk_socket: *mut socket,
}

#[repr(C)]
struct sockaddr_un {
    sun_family: u16,
    sun_path: [i8; 108],
}

#[repr(C)]
struct unix_address_name {
    sun_path: [i8; 108],
}

#[repr(C)]
struct unix_address {
    len: c_int,
    name: *mut unix_address_name,
}

#[repr(C)]
struct unix_sock {
    lock: lock_type,
    addr: *mut unix_address,
}

#[repr(C)]
struct lock_type {
    _private: [u8; 0],
}

#[repr(C)]
struct fown_struct {
    lock: lock_type,
    file: *mut file,
}

#[repr(C)]
struct landlock_file_security {
    fown_subject: landlock_cred_security,
    fown_tg: *mut pid,
    fown_layer: size_t,
}

#[repr(C)]
struct lsm_network_audit {
    sk: *mut sock,
}

#[repr(C)]
union lsm_audit_data_u {
    tsk: *mut task_struct,
    net: *mut lsm_network_audit,
}

#[repr(C)]
struct lsm_audit_data {
    type_: c_int,
    u: lsm_audit_data_u,
}

#[repr(C)]
struct landlock_request {
    type_: c_int,
    audit: lsm_audit_data,
    layer_plus_one: size_t,
    other_domain_id: u64,
}

#[repr(C)]
struct security_hook_list {
    _private: [u8; 0],
}

#[repr(C)]
struct lsm_id {
    _private: [u8; 0],
}

unsafe extern "C" {
    static mut current: *mut task_struct;
    static landlock_lsmid: lsm_id;

    fn current_cred() -> *const cred;
    fn __task_cred(task: *mut task_struct) -> *const cred;
    fn landlock_cred(cred: *const cred) -> *const landlock_cred_security;
    fn landlock_get_task_domain(task: *mut task_struct) -> *const landlock_domain;
    fn landlock_get_current_domain() -> *const landlock_domain;
    fn landlock_get_scope_mask(domain: *const landlock_domain, layer: c_int) -> access_mask_t;
    fn landlock_get_applicable_subject(
        cred: *const cred,
        masks: access_masks,
        layer: *mut size_t,
    ) -> *const landlock_cred_security;
    fn landlock_log_denial(subject: *const landlock_cred_security, request: *const landlock_request);
    fn unix_sk(sock: *mut sock) -> *mut unix_sock;
    fn unix_peer(sock: *mut sock) -> *mut sock;
    fn same_thread_group(p: *mut task_struct, current: *mut task_struct) -> bool_;
    fn task_tgid(task: *mut task_struct) -> *mut pid;
    fn landlock_file(file: *mut file) -> *mut landlock_file_security;
    fn security_add_hooks(hooks: *mut security_hook_list, count: usize, lsmid: *const lsm_id);
}

unsafe fn WARN_ON_ONCE(cond: bool_) -> bool_ {
    cond
}

unsafe fn unlikely(cond: bool_) -> bool_ {
    cond
}

unsafe fn lockdep_assert_held<T>(_lock: *const T) {}

/**
 * domain_scope_le - Checks domain ordering for scoped ptrace
 *
 * @parent: Parent domain.
 * @child: Potential child of @parent.
 *
 * Checks if the @parent domain is less or equal to (i.e. an ancestor, which
 * means a subset of) the @child domain.
 *
 * Return: True if @parent is an ancestor of or equal to @child, false
 * otherwise.
 */
unsafe fn domain_scope_le(
    parent: *const landlock_domain,
    child: *const landlock_domain,
) -> bool_ {
    let mut walker: *const landlock_hierarchy;

    /* Quick return for non-landlocked tasks. */
    if parent.is_null() {
        return true;
    }

    if child.is_null() {
        return false;
    }

    walker = (*child).hierarchy;
    while !walker.is_null() {
        if walker == (*parent).hierarchy {
            /* @parent is in the scoped hierarchy of @child. */
            return true;
        }
        walker = (*walker).parent;
    }

    /* There is no relationship between @parent and @child. */
    false
}

unsafe fn domain_ptrace(
    parent: *const landlock_domain,
    child: *const landlock_domain,
) -> c_int {
    if domain_scope_le(parent, child) {
        return 0;
    }

    -EPERM
}

/**
 * hook_ptrace_access_check - Determines whether the current process may access
 *			      another
 *
 * @child: Process to be accessed.
 * @mode: Mode of attachment.
 *
 * If the current task has Landlock rules, then the child must have at least
 * the same rules.  Else denied.
 *
 * Return: 0 if permission is granted, -errno if denied.
 */
unsafe fn hook_ptrace_access_check(child: *mut task_struct, mode: c_uint) -> c_int {
    let parent_subject: *const landlock_cred_security;
    let mut tracee_domain_id: u64 = 0;
    let err: c_int;

    /* Quick return for non-landlocked tasks. */
    parent_subject = landlock_cred(current_cred());
    if parent_subject.is_null() {
        return 0;
    }

    {
        // scoped_guard(rcu)
        let child_dom: *const landlock_domain = landlock_get_task_domain(child);
        err = domain_ptrace((*parent_subject).domain, child_dom);
        // CONFIG_SECURITY_LANDLOCK_LOG
        if !child_dom.is_null() {
            tracee_domain_id = (*(*child_dom).hierarchy).id;
        }
    }

    if err == 0 {
        return 0;
    }

    /*
     * For the ptrace_access_check case, we log the current/parent domain
     * and the child task.
     */
    if (mode & PTRACE_MODE_NOAUDIT) == 0 {
        landlock_log_denial(
            parent_subject,
            &landlock_request {
                type_: LANDLOCK_REQUEST_PTRACE,
                audit: lsm_audit_data {
                    type_: LSM_AUDIT_DATA_TASK,
                    u: lsm_audit_data_u { tsk: child },
                },
                layer_plus_one: (*(*parent_subject).domain).num_layers as size_t,
                other_domain_id: tracee_domain_id,
            },
        );
    }

    err
}

/**
 * hook_ptrace_traceme - Determines whether another process may trace the
 *			 current one
 *
 * @parent: Task proposed to be the tracer.
 *
 * If the parent has Landlock rules, then the current task must have the same
 * or more rules.  Else denied.
 *
 * Return: 0 if permission is granted, -errno if denied.
 */
unsafe fn hook_ptrace_traceme(parent: *mut task_struct) -> c_int {
    let parent_subject: *const landlock_cred_security;
    let child_dom: *const landlock_domain;
    let mut tracee_domain_id: u64 = 0;
    let err: c_int;

    child_dom = landlock_get_current_domain();

    // guard(rcu)();
    parent_subject = landlock_cred(__task_cred(parent));
    err = domain_ptrace((*parent_subject).domain, child_dom);

    if err == 0 {
        return 0;
    }

    // CONFIG_SECURITY_LANDLOCK_LOG
    /* The tracee is the current task; its domain is stable here. */
    if !child_dom.is_null() {
        tracee_domain_id = (*(*child_dom).hierarchy).id;
    }

    /*
     * For the ptrace_traceme case, we log the domain which is the cause of
     * the denial, which means the parent domain instead of the current
     * domain.  This may look unusual because the ptrace_traceme action is a
     * request to be traced, but the semantic is consistent with
     * hook_ptrace_access_check().
     */
    landlock_log_denial(
        parent_subject,
        &landlock_request {
            type_: LANDLOCK_REQUEST_PTRACE,
            audit: lsm_audit_data {
                type_: LSM_AUDIT_DATA_TASK,
                u: lsm_audit_data_u { tsk: current },
            },
            layer_plus_one: (*(*parent_subject).domain).num_layers as size_t,
            other_domain_id: tracee_domain_id,
        },
    );
    err
}

/**
 * domain_is_scoped - Check if an interaction from a client/sender to a
 *		      server/receiver should be restricted based on scope controls.
 *
 * @client: IPC sender domain.
 * @server: IPC receiver domain.
 * @scope: The scope restriction criteria.
 *
 * Return: True if @server is in a different domain from @client and @client
 * is scoped to access @server (i.e. access should be denied), false otherwise.
 */
unsafe fn domain_is_scoped(
    client: *const landlock_domain,
    server: *const landlock_domain,
    scope: access_mask_t,
) -> bool_ {
    let mut client_layer: c_int;
    let mut server_layer: c_int;
    let mut client_walker: *const landlock_hierarchy;
    let mut server_walker: *const landlock_hierarchy;

    /* Quick return if client has no domain */
    if WARN_ON_ONCE(client.is_null()) {
        return false;
    }

    client_layer = (*client).num_layers - 1;
    client_walker = (*client).hierarchy;
    /*
     * client_layer must be able to represent all numbers from
     * LANDLOCK_MAX_NUM_LAYERS - 1 to -1 for the loop below to terminate.
     * (It must be large enough, and it must be signed.)
     */
    // BUILD_BUG_ON(!is_signed_type(typeof(client_layer)));
    // BUILD_BUG_ON(LANDLOCK_MAX_NUM_LAYERS - 1 > type_max(typeof(client_layer)));
    let _ = LANDLOCK_MAX_NUM_LAYERS;

    server_layer = if !server.is_null() {
        (*server).num_layers - 1
    } else {
        -1
    };
    server_walker = if !server.is_null() {
        (*server).hierarchy
    } else {
        ptr::null()
    };

    /*
     * Walks client's parent domains down to the same hierarchy level
     * as the server's domain, and checks that none of these client's
     * parent domains are scoped.
     */
    while client_layer > server_layer {
        if (landlock_get_scope_mask(client, client_layer) & scope) != 0 {
            return true;
        }

        client_walker = (*client_walker).parent;
        client_layer -= 1;
    }
    /*
     * Walks server's parent domains down to the same hierarchy level as
     * the client's domain.
     */
    while server_layer > client_layer {
        server_walker = (*server_walker).parent;
        server_layer -= 1;
    }

    while client_layer >= 0 {
        if (landlock_get_scope_mask(client, client_layer) & scope) != 0 {
            /*
             * Client and server are at the same level in the
             * hierarchy. If the client is scoped, the request is
             * only allowed if this domain is also a server's
             * ancestor.
             */
            return server_walker != client_walker;
        }
        client_walker = (*client_walker).parent;
        server_walker = (*server_walker).parent;
        client_layer -= 1;
    }
    false
}

unsafe fn sock_is_scoped(
    other: *mut sock,
    domain: *const landlock_domain,
    peer_domain_id: *mut u64,
) -> bool_ {
    let dom_other: *const landlock_domain;

    /* The credentials will not change. */
    lockdep_assert_held(&(*unix_sk(other)).lock);

    /*
     * A live kernel socket (e.g. from sock_create_kern()) has no backing
     * file, hence no Landlock domain, so treat it as unscoped.  The
     * sk_socket check only guards that dereference; sk_socket is NULL
     * solely for a dead peer, which the caller already excludes under the
     * held lock, so no separate SOCK_DEAD check is needed.
     */
    if unlikely((*other).sk_socket.is_null() || (*(*other).sk_socket).file.is_null()) {
        return false;
    }

    dom_other = (*landlock_cred((*(*(*other).sk_socket).file).f_cred)).domain;
    // CONFIG_SECURITY_LANDLOCK_LOG
    *peer_domain_id = if !dom_other.is_null() {
        (*(*dom_other).hierarchy).id
    } else {
        0
    };
    domain_is_scoped(domain, dom_other, LANDLOCK_SCOPE_ABSTRACT_UNIX_SOCKET)
}

unsafe fn is_abstract_socket(sock: *mut sock) -> bool_ {
    let addr: *mut unix_address = (*unix_sk(sock)).addr;

    if addr.is_null() {
        return false;
    }

    if (*addr).len as usize >= offset_of!(sockaddr_un, sun_path) + 1
        && (*(*addr).name).sun_path[0] == 0
    {
        return true;
    }

    false
}

static unix_scope: access_masks = access_masks {
    scope: LANDLOCK_SCOPE_ABSTRACT_UNIX_SOCKET,
};

unsafe fn hook_unix_stream_connect(
    _sock: *mut sock,
    other: *mut sock,
    _newsk: *mut sock,
) -> c_int {
    let mut handle_layer: size_t = 0;
    let mut peer_domain_id: u64 = 0;
    let subject: *const landlock_cred_security =
        landlock_get_applicable_subject(current_cred(), unix_scope, &mut handle_layer);

    /* Quick return for non-landlocked tasks. */
    if subject.is_null() {
        return 0;
    }

    if !is_abstract_socket(other) {
        return 0;
    }

    if !sock_is_scoped(other, (*subject).domain, &mut peer_domain_id) {
        return 0;
    }

    let mut net_audit = lsm_network_audit { sk: other };
    landlock_log_denial(
        subject,
        &landlock_request {
            type_: LANDLOCK_REQUEST_SCOPE_ABSTRACT_UNIX_SOCKET,
            audit: lsm_audit_data {
                type_: LSM_AUDIT_DATA_NET,
                u: lsm_audit_data_u { net: &mut net_audit },
            },
            layer_plus_one: handle_layer + 1,
            other_domain_id: peer_domain_id,
        },
    );
    -EPERM
}

unsafe fn hook_unix_may_send(sock: *mut socket, other: *mut socket) -> c_int {
    let mut handle_layer: size_t = 0;
    let mut peer_domain_id: u64 = 0;
    let subject: *const landlock_cred_security =
        landlock_get_applicable_subject(current_cred(), unix_scope, &mut handle_layer);

    if subject.is_null() {
        return 0;
    }

    /*
     * Checks if this datagram socket was already allowed to be connected
     * to other.
     */
    if unix_peer((*sock).sk) == (*other).sk {
        return 0;
    }

    if !is_abstract_socket((*other).sk) {
        return 0;
    }

    if !sock_is_scoped((*other).sk, (*subject).domain, &mut peer_domain_id) {
        return 0;
    }

    let mut net_audit = lsm_network_audit { sk: (*other).sk };
    landlock_log_denial(
        subject,
        &landlock_request {
            type_: LANDLOCK_REQUEST_SCOPE_ABSTRACT_UNIX_SOCKET,
            audit: lsm_audit_data {
                type_: LSM_AUDIT_DATA_NET,
                u: lsm_audit_data_u { net: &mut net_audit },
            },
            layer_plus_one: handle_layer + 1,
            other_domain_id: peer_domain_id,
        },
    );
    -EPERM
}

static signal_scope: access_masks = access_masks {
    scope: LANDLOCK_SCOPE_SIGNAL,
};

unsafe fn hook_task_kill(
    p: *mut task_struct,
    _info: *mut kernel_siginfo,
    _sig: c_int,
    mut cred: *const cred,
) -> c_int {
    let is_scoped: bool_;
    let mut handle_layer: size_t = 0;
    let mut target_domain_id: u64 = 0;
    let subject: *const landlock_cred_security;

    if cred.is_null() {
        /*
         * Always allow sending signals between threads of the same process.
         * This is required for process credential changes by the Native POSIX
         * Threads Library and implemented by the set*id(2) wrappers and
         * libcap(3) with tgkill(2).  See nptl(7) and libpsx(3).
         *
         * This exception is similar to the __ptrace_may_access() one.
         */
        if same_thread_group(p, current) {
            return 0;
        }

        /* Not dealing with USB IO. */
        cred = current_cred();
    }

    subject = landlock_get_applicable_subject(cred, signal_scope, &mut handle_layer);

    /* Quick return for non-landlocked tasks. */
    if subject.is_null() {
        return 0;
    }

    {
        // scoped_guard(rcu)
        let other: *const landlock_domain = landlock_get_task_domain(p);

        is_scoped = domain_is_scoped((*subject).domain, other, signal_scope.scope);
        // CONFIG_SECURITY_LANDLOCK_LOG
        if !other.is_null() {
            target_domain_id = (*(*other).hierarchy).id;
        }
    }

    if !is_scoped {
        return 0;
    }

    landlock_log_denial(
        subject,
        &landlock_request {
            type_: LANDLOCK_REQUEST_SCOPE_SIGNAL,
            audit: lsm_audit_data {
                type_: LSM_AUDIT_DATA_TASK,
                u: lsm_audit_data_u { tsk: p },
            },
            layer_plus_one: handle_layer + 1,
            other_domain_id: target_domain_id,
        },
    );
    -EPERM
}

unsafe fn hook_file_send_sigiotask(
    tsk: *mut task_struct,
    fown: *mut fown_struct,
    _signum: c_int,
) -> c_int {
    let subject: *const landlock_cred_security;
    let mut is_scoped: bool_ = false;
    let mut target_domain_id: u64 = 0;

    /* Lock already held by send_sigio() and send_sigurg(). */
    lockdep_assert_held(&(*fown).lock);
    subject = &(*landlock_file((*fown).file)).fown_subject;

    /*
     * Quick return for unowned socket.
     *
     * subject->domain has already been filtered when saved by
     * hook_file_set_fowner(), so there is no need to call
     * landlock_get_applicable_subject() here.
     */
    if (*subject).domain.is_null() {
        return 0;
    }

    /*
     * Always allow delivery to the file owner's own process, including a
     * thread-group leader reached through a process-group owner.  This
     * mirrors hook_task_kill()'s same-process exemption and preserves the
     * guarantee of commit 18eb75f3af40 ("landlock: Always allow signals
     * between threads of the same process"), which the registration-time
     * check cannot honor for a process-group target.
     */
    if task_tgid(tsk) == (*landlock_file((*fown).file)).fown_tg {
        return 0;
    }

    {
        // scoped_guard(rcu)
        let other: *const landlock_domain = landlock_get_task_domain(tsk);

        is_scoped = domain_is_scoped((*subject).domain, other, signal_scope.scope);
        // CONFIG_SECURITY_LANDLOCK_LOG
        if !other.is_null() {
            target_domain_id = (*(*other).hierarchy).id;
        }
    }

    if !is_scoped {
        return 0;
    }

    landlock_log_denial(
        subject,
        &landlock_request {
            type_: LANDLOCK_REQUEST_SCOPE_SIGNAL,
            audit: lsm_audit_data {
                type_: LSM_AUDIT_DATA_TASK,
                u: lsm_audit_data_u { tsk },
            },
            // CONFIG_SECURITY_LANDLOCK_LOG
            layer_plus_one: (*landlock_file((*fown).file)).fown_layer + 1,
            other_domain_id: target_domain_id,
        },
    );
    -EPERM
}

// __ro_after_init
static mut landlock_hooks: [security_hook_list; 6] = [
    // LSM_HOOK_INIT(ptrace_access_check, hook_ptrace_access_check)
    security_hook_list { _private: [] },
    // LSM_HOOK_INIT(ptrace_traceme, hook_ptrace_traceme)
    security_hook_list { _private: [] },
    // LSM_HOOK_INIT(unix_stream_connect, hook_unix_stream_connect)
    security_hook_list { _private: [] },
    // LSM_HOOK_INIT(unix_may_send, hook_unix_may_send)
    security_hook_list { _private: [] },
    // LSM_HOOK_INIT(task_kill, hook_task_kill)
    security_hook_list { _private: [] },
    // LSM_HOOK_INIT(file_send_sigiotask, hook_file_send_sigiotask)
    security_hook_list { _private: [] },
];

// __init
unsafe fn landlock_add_task_hooks() {
    security_add_hooks(
        landlock_hooks.as_mut_ptr(),
        landlock_hooks.len(),
        &landlock_lsmid,
    );
}


// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
