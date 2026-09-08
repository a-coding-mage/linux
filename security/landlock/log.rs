// SPDX-License-Identifier: GPL-2.0-only
/*
 * Landlock - Log helpers
 *
 * Copyright © 2023-2025 Microsoft Corporation
 * Copyright © 2026 Cloudflare, Inc.
 */

// C dependencies:
// <kunit/test.h>, <linux/bitops.h>, <uapi/linux/landlock.h>
// "access.h", "audit.h", "common.h", "cred.h", "domain.h", "limits.h",
// "log.h", "ruleset.h", "trace.h"

use core::ffi::{c_char, c_int};

type access_mask_t = u32;
type deny_masks_t = u64;
type optional_access_t = u32;

const BITS_PER_ACCESS_MASK_T: usize = core::mem::size_of::<access_mask_t>() * 8;

extern "C" {
    static _LANDLOCK_ACCESS_FS_OPTIONAL: access_mask_t;
    static LANDLOCK_MAX_NUM_LAYERS: usize;
    static LANDLOCK_ACCESS_FS_EXECUTE: access_mask_t;
    static LANDLOCK_ACCESS_FS_READ_DIR: access_mask_t;
    static LANDLOCK_ACCESS_FS_READ_FILE: access_mask_t;
    static LANDLOCK_ACCESS_FS_REMOVE_DIR: access_mask_t;
    static LANDLOCK_ACCESS_FS_WRITE_FILE: access_mask_t;
    static LANDLOCK_ACCESS_FS_TRUNCATE: access_mask_t;
    static LANDLOCK_ACCESS_FS_IOCTL_DEV: access_mask_t;
    static LANDLOCK_SCOPE_SIGNAL: u64;
    static LANDLOCK_SCOPE_ABSTRACT_UNIX_SOCKET: u64;
    static LANDLOCK_LOG_DISABLED: c_int;

    fn WARN_ON_ONCE(condition: bool) -> bool;
    fn WARN_ONCE(condition: c_int, fmt: *const c_char, ...) -> bool;
    fn hweight32(value: u32) -> u32;
    fn atomic64_inc(value: *mut atomic64_t);
    fn landlock_trace_denial(
        request: *const landlock_request,
        youngest_denied: *const landlock_hierarchy,
        missing: access_mask_t,
        same_exec: bool,
        logged: bool,
    );
    fn landlock_audit_denial(
        request: *const landlock_request,
        youngest_denied: *const landlock_hierarchy,
        missing: access_mask_t,
        logged: bool,
    );
    fn landlock_trace_free_domain(hierarchy: *const landlock_hierarchy);
    fn landlock_audit_free_domain(hierarchy: *const landlock_hierarchy);
}

#[repr(C)]
pub struct atomic64_t {
    _private: [u8; 0],
}

#[repr(C)]
pub struct landlock_cred_security {
    pub domain: *const landlock_domain,
    pub domain_exec: u64,
}

#[repr(C)]
pub struct landlock_domain {
    pub hierarchy: *mut landlock_hierarchy,
    pub num_layers: usize,
}

#[repr(C)]
pub struct landlock_hierarchy {
    pub parent: *mut landlock_hierarchy,
    pub id: i64,
    pub quiet_masks: access_masks,
    pub log_status: c_int,
    pub log_same_exec: bool,
    pub log_new_exec: bool,
    pub num_denials: atomic64_t,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct access_masks {
    pub fs: access_mask_t,
    pub net: access_mask_t,
    pub scope: u64,
}

#[repr(C)]
pub struct layer_mask {
    pub access: access_mask_t,
    pub quiet: bool,
}

#[repr(C)]
pub struct layer_masks {
    pub layers: [layer_mask; 16],
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum landlock_request_type {
    LANDLOCK_REQUEST_FS_ACCESS = 0,
    LANDLOCK_REQUEST_NET_ACCESS = 1,
    LANDLOCK_REQUEST_SCOPE_SIGNAL = 2,
    LANDLOCK_REQUEST_SCOPE_ABSTRACT_UNIX_SOCKET = 3,
    LANDLOCK_REQUEST_PTRACE = 4,
    LANDLOCK_REQUEST_FS_CHANGE_TOPOLOGY = 5,
}

#[repr(C)]
pub struct landlock_request {
    pub layer_plus_one: usize,
    pub access: access_mask_t,
    pub layer_masks: *const layer_masks,
    pub all_existing_optional_access: access_mask_t,
    pub deny_masks: deny_masks_t,
    pub quiet_optional_accesses: optional_access_t,
    pub type_: landlock_request_type,
}

#[inline]
const fn BIT(bit: usize) -> u64 {
    1u64 << bit
}

#[inline]
fn HWEIGHT(value: usize) -> u32 {
    value.count_ones()
}

#[inline]
unsafe fn READ_ONCE<T: Copy>(ptr: *const T) -> T {
    core::ptr::read_volatile(ptr)
}

unsafe fn get_hierarchy(
    domain: *const landlock_domain,
    layer: usize,
) -> *mut landlock_hierarchy {
    let mut hierarchy = (*domain).hierarchy;
    let mut i: isize;

    if WARN_ON_ONCE(layer >= (*domain).num_layers) {
        return hierarchy;
    }

    i = (*domain).num_layers as isize - 1;
    while i > layer as isize {
        if WARN_ON_ONCE((*hierarchy).parent.is_null()) {
            break;
        }

        hierarchy = (*hierarchy).parent;
        i -= 1;
    }

    hierarchy
}

// CONFIG_SECURITY_LANDLOCK_KUNIT_TEST:
// static void test_get_hierarchy(struct kunit *const test) { ... }

/* Get the youngest layer that denied the access_request. */
unsafe fn get_denied_layer(
    domain: *const landlock_domain,
    access_request: *mut access_mask_t,
    masks: *const layer_masks,
) -> usize {
    let mut i: isize = (*masks).layers.len() as isize - 1;

    while i >= 0 {
        if ((*masks).layers[i as usize].access & *access_request) != 0 {
            *access_request &= (*masks).layers[i as usize].access;
            return i as usize;
        }
        i -= 1;
    }

    /* Not found - fall back to default values */
    *access_request = 0;
    (*domain).num_layers - 1
}

// CONFIG_SECURITY_LANDLOCK_KUNIT_TEST:
// static void test_get_denied_layer(struct kunit *const test) { ... }

unsafe fn get_layer_from_deny_masks(
    access_request: *mut access_mask_t,
    all_existing_optional_access: access_mask_t,
    deny_masks: deny_masks_t,
    quiet_optional_accesses: optional_access_t,
    quiet: *mut bool,
) -> usize {
    let access_opt: u64 = all_existing_optional_access as u64;
    let access_req: u64 = *access_request as u64;
    let mut missing: access_mask_t = 0;
    let mut youngest_layer: usize = 0;
    let mut access_index: usize = 0;
    let mut should_quiet = false;

    /* This will require change with new object types. */
    WARN_ON_ONCE(access_opt != _LANDLOCK_ACCESS_FS_OPTIONAL as u64);

    let mut access_bit = 0usize;
    while access_bit < BITS_PER_ACCESS_MASK_T {
        if (access_opt & BIT(access_bit)) != 0 {
            if (access_req & BIT(access_bit)) != 0 {
                let width = HWEIGHT(LANDLOCK_MAX_NUM_LAYERS - 1) as usize;
                let layer = ((deny_masks >> (access_index * width))
                    & (LANDLOCK_MAX_NUM_LAYERS as deny_masks_t - 1)) as usize;
                let layer_has_quiet =
                    (quiet_optional_accesses as u64 & BIT(access_index)) != 0;

                if layer > youngest_layer {
                    youngest_layer = layer;
                    missing = BIT(access_bit) as access_mask_t;
                    should_quiet = layer_has_quiet;
                } else if layer == youngest_layer {
                    missing |= BIT(access_bit) as access_mask_t;
                    /*
                     * Whether the layer has rules with quiet flag
                     * covering the file accessed does not depend on
                     * the access, and so the following
                     * WARN_ON_ONCE() should not fail.
                     */
                    WARN_ON_ONCE(should_quiet && !layer_has_quiet);
                    should_quiet = layer_has_quiet;
                }
            }
            access_index += 1;
        }
        access_bit += 1;
    }

    *access_request = missing;
    *quiet = should_quiet;
    youngest_layer
}

// CONFIG_SECURITY_LANDLOCK_KUNIT_TEST:
// static void test_get_layer_from_deny_masks(struct kunit *const test) { ... }

unsafe fn is_valid_request(request: *const landlock_request) -> bool {
    if WARN_ON_ONCE((*request).layer_plus_one > LANDLOCK_MAX_NUM_LAYERS) {
        return false;
    }

    if WARN_ON_ONCE(!(((*request).layer_plus_one != 0) ^ ((*request).access != 0))) {
        return false;
    }

    if (*request).access != 0 {
        if WARN_ON_ONCE(
            !((!(*request).layer_masks.is_null())
                ^ ((*request).all_existing_optional_access != 0)),
        ) {
            return false;
        }
    } else if WARN_ON_ONCE(
        !(*request).layer_masks.is_null() || (*request).all_existing_optional_access != 0,
    ) {
        return false;
    }

    if (*request).deny_masks != 0 {
        if WARN_ON_ONCE((*request).all_existing_optional_access == 0) {
            return false;
        }
        const _: [(); core::mem::size_of::<access_mask_t>()] = [(); core::mem::size_of::<u32>()];
        if WARN_ON_ONCE(
            (*request).quiet_optional_accesses as u64
                >= BIT(hweight32((*request).all_existing_optional_access) as usize),
        ) {
            return false;
        }
    }

    true
}

unsafe fn pick_access_mask_for_request_type(
    type_: landlock_request_type,
    access_masks: access_masks,
) -> access_mask_t {
    match type_ {
        landlock_request_type::LANDLOCK_REQUEST_FS_ACCESS => access_masks.fs,
        landlock_request_type::LANDLOCK_REQUEST_NET_ACCESS => access_masks.net,
        _ => {
            WARN_ONCE(
                1,
                b"Invalid request type %d passed to %s\0".as_ptr() as *const c_char,
                type_ as c_int,
                b"pick_access_mask_for_request_type\0".as_ptr() as *const c_char,
            );
            0
        }
    }
}

/*
 * Whether a quiet rule silences the denial: the rule must cover the whole
 * denied access in the layer that denied it (a quiet rule in a non-denying
 * layer does not suppress the denial).
 */
unsafe fn is_denial_quieted(
    request: *const landlock_request,
    youngest_denied: *const landlock_hierarchy,
    missing: access_mask_t,
    object_quiet_flag: bool,
) -> bool {
    if object_quiet_flag {
        let quiet_mask =
            pick_access_mask_for_request_type((*request).type_, (*youngest_denied).quiet_masks);

        return (quiet_mask & missing) == missing;
    }

    /*
     * Either the object is not quiet, or this is a scope request.  We check
     * request->type to distinguish between the two cases.
     */
    match (*request).type_ {
        landlock_request_type::LANDLOCK_REQUEST_SCOPE_SIGNAL => {
            ((*youngest_denied).quiet_masks.scope & LANDLOCK_SCOPE_SIGNAL) != 0
        }
        landlock_request_type::LANDLOCK_REQUEST_SCOPE_ABSTRACT_UNIX_SOCKET => {
            ((*youngest_denied).quiet_masks.scope & LANDLOCK_SCOPE_ABSTRACT_UNIX_SOCKET) != 0
        }
        /*
         * Leave LANDLOCK_REQUEST_PTRACE and LANDLOCK_REQUEST_FS_CHANGE_TOPOLOGY
         * unhandled for now - they are never quiet.
         */
        _ => false,
    }
}

/*
 * Computes whether a denial from youngest_denied is selected for logging by the
 * domain's policy: its logging must not be disabled (by both per-execution
 * flags being off, or by an ancestor's
 * LANDLOCK_RESTRICT_SELF_LOG_SUBDOMAINS_OFF), the per-execution flag matching
 * same_exec must be set, and no quiet rule may cover the denied access.
 * landlock_log_denial() computes this once and passes it to
 * landlock_audit_denial(), which additionally requires audit_enabled.
 */
unsafe fn is_denial_logged(
    request: *const landlock_request,
    youngest_denied: *const landlock_hierarchy,
    missing: access_mask_t,
    same_exec: bool,
    object_quiet_flag: bool,
) -> bool {
    if READ_ONCE(core::ptr::addr_of!((*youngest_denied).log_status)) == LANDLOCK_LOG_DISABLED {
        return false;
    }

    if !(if same_exec {
        (*youngest_denied).log_same_exec
    } else {
        (*youngest_denied).log_new_exec
    }) {
        return false;
    }

    !is_denial_quieted(request, youngest_denied, missing, object_quiet_flag)
}

/**
 * landlock_log_denial - Log a denied access
 *
 * @subject: The Landlock subject's credential denying an action.
 * @request: Detail of the user space request.
 */
#[no_mangle]
pub unsafe extern "C" fn landlock_log_denial(
    subject: *const landlock_cred_security,
    request: *const landlock_request,
) {
    let youngest_denied: *mut landlock_hierarchy;
    let youngest_layer: usize;
    let mut missing: access_mask_t;
    let mut object_quiet_flag = false;

    if WARN_ON_ONCE(
        subject.is_null()
            || (*subject).domain.is_null()
            || (*(*subject).domain).hierarchy.is_null()
            || request.is_null(),
    ) {
        return;
    }

    if !is_valid_request(request) {
        return;
    }

    missing = (*request).access;
    if missing != 0 {
        /* Gets the nearest domain that denies the request. */
        if !(*request).layer_masks.is_null() {
            youngest_layer = get_denied_layer((*subject).domain, &mut missing, (*request).layer_masks);
            object_quiet_flag = (*(*request).layer_masks).layers[youngest_layer].quiet;
        } else {
            youngest_layer = get_layer_from_deny_masks(
                &mut missing,
                _LANDLOCK_ACCESS_FS_OPTIONAL,
                (*request).deny_masks,
                (*request).quiet_optional_accesses,
                &mut object_quiet_flag,
            );
        }
        youngest_denied = get_hierarchy((*subject).domain, youngest_layer);
    } else {
        youngest_layer = (*request).layer_plus_one - 1;
        youngest_denied = get_hierarchy((*subject).domain, youngest_layer);
    }

    let same_exec = ((*subject).domain_exec & BIT(youngest_layer)) != 0;
    let logged = is_denial_logged(
        request,
        youngest_denied,
        missing,
        same_exec,
        object_quiet_flag,
    );

    /*
     * Consistently keeps track of the number of denied access requests even
     * if audit is currently disabled, or if audit rules currently exclude
     * this record type, or if landlock_restrict_self(2)'s flags quiet logs.
     */
    atomic64_inc(core::ptr::addr_of_mut!((*youngest_denied).num_denials));

    landlock_trace_denial(request, youngest_denied, missing, same_exec, logged);
    landlock_audit_denial(request, youngest_denied, missing, logged);
}

/**
 * landlock_log_free_domain - Log domain deallocation
 *
 * @hierarchy: The domain's hierarchy being deallocated.
 *
 * Called from landlock_put_domain_deferred() (via a work queue scheduled by
 * hook_cred_free()) or directly from landlock_put_domain().
 */
#[no_mangle]
pub unsafe extern "C" fn landlock_log_free_domain(hierarchy: *const landlock_hierarchy) {
    if WARN_ON_ONCE(hierarchy.is_null()) {
        return;
    }

    landlock_trace_free_domain(hierarchy);
    landlock_audit_free_domain(hierarchy);
}

// CONFIG_SECURITY_LANDLOCK_KUNIT_TEST:
// static struct kunit_case test_cases[] = {
//     KUNIT_CASE(test_get_hierarchy),
//     KUNIT_CASE(test_get_denied_layer),
//     KUNIT_CASE(test_get_layer_from_deny_masks),
//     {}
// };
//
// static struct kunit_suite test_suite = {
//     .name = "landlock_log",
//     .test_cases = test_cases,
// };
//
// kunit_test_suite(test_suite);


// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
