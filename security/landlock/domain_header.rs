/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * Landlock - Domain management
 *
 * Copyright © 2016-2020 Mickaël Salaün <mic@digikod.net>
 * Copyright © 2018-2020 ANSSI
 * Copyright © 2024-2025 Microsoft Corporation
 * Copyright © 2026 Cloudflare, Inc.
 */

/* Dependencies from the original header:
 * <linux/cleanup.h>, <linux/limits.h>, <linux/mm.h>, <linux/path.h>,
 * <linux/pid.h>, <linux/refcount.h>, <linux/sched.h>, <linux/slab.h>,
 * <linux/workqueue.h>, "access.h", "log.h", and "ruleset.h".
 */

use core::ffi::{c_char, c_int};
use core::mem::ManuallyDrop;

pub const LANDLOCK_LOG_UNCOMMITTED: landlock_log_status = 0;
pub const LANDLOCK_LOG_PENDING: landlock_log_status = 1;
pub const LANDLOCK_LOG_RECORDED: landlock_log_status = 2;
pub const LANDLOCK_LOG_DISABLED: landlock_log_status = 3;

pub type landlock_log_status = u32;

/**
 * struct landlock_details - Domain's creation information
 *
 * Rarely accessed, mainly when logging the first domain's denial.
 *
 * The contained pointers are initialized at the domain creation time and never
 * changed again.
 */
#[repr(C)]
pub struct landlock_details {
    /**
     * @pid: PID of the task that initially restricted itself.  It still
     * identifies the same task.  Keeping a reference to this PID ensures that
     * it will not be recycled.
     */
    pub pid: *mut pid,
    /**
     * @uid: UID of the task that initially restricted itself, at creation time.
     */
    pub uid: uid_t,
    /**
     * @comm: Command line of the task that initially restricted itself, at
     * creation time.  Always NULL terminated.
     */
    pub comm: [c_char; TASK_COMM_LEN],
    /**
     * @exe_path: Executable path of the task that initially restricted
     * itself, at creation time.  Always NULL terminated, and never greater
     * than LANDLOCK_PATH_MAX_SIZE.
     */
    pub exe_path: [c_char; 0],
}

/* Adds 11 extra characters for the potential " (deleted)" suffix. */
pub const LANDLOCK_PATH_MAX_SIZE: usize = PATH_MAX + 11;

/* Makes sure the greatest landlock_details can be allocated.
 * C static_assert:
 * struct_size_t(struct landlock_details, exe_path, LANDLOCK_PATH_MAX_SIZE) <=
 * KMALLOC_MAX_SIZE
 */

/**
 * struct landlock_hierarchy - Node in a domain hierarchy
 */
#[repr(C)]
pub struct landlock_hierarchy {
    /**
     * @parent: Pointer to the parent node, or NULL if it is a root
     * Landlock domain.
     */
    pub parent: *mut landlock_hierarchy,
    /**
     * @usage: Number of potential children domains plus their parent
     * domain.
     */
    pub usage: refcount_t,

    /* CONFIG_SECURITY_LANDLOCK_LOG */
    /**
     * @log_status: Whether this domain should be logged or not.  Because
     * concurrent log entries may be created at the same time, it is still
     * possible to have several domain records of the same domain.
     */
    pub log_status: landlock_log_status,
    /**
     * @num_denials: Number of access requests denied by this domain.
     * Masked (i.e. never logged) denials are still counted.
     */
    pub num_denials: atomic64_t,
    /**
     * @id: Landlock domain ID, set once at domain creation time.
     */
    pub id: u64,
    /**
     * @details: Information about the related domain.
     */
    pub details: *const landlock_details,
    /**
     * @log_same_exec: Set if the domain is *not* configured with
     * %LANDLOCK_RESTRICT_SELF_LOG_SAME_EXEC_OFF.  Set to true by default.
     *
     * @log_new_exec: Set if the domain is configured with
     * %LANDLOCK_RESTRICT_SELF_LOG_NEW_EXEC_ON.  Set to false by default.
     *
     * Original C bitfields: u32 log_same_exec : 1, log_new_exec : 1.
     */
    pub log_exec_bitfields: u32,
    /**
     * @quiet_masks: Bitmasks of access that should be quieted (i.e. not
     * logged) if the related object is marked as quiet.
     */
    pub quiet_masks: access_masks,
}

extern "C" {
    pub fn landlock_get_deny_masks(
        all_existing_optional_access: access_mask_t,
        optional_access: access_mask_t,
        masks: *const layer_masks,
    ) -> deny_masks_t;

    pub fn landlock_get_quiet_optional_accesses(
        all_existing_optional_access: access_mask_t,
        deny_masks: deny_masks_t,
        masks: *const layer_masks,
    ) -> optional_access_t;

    pub fn landlock_log_free_domain(hierarchy: *mut landlock_hierarchy);
    pub fn put_pid(pid: *mut pid);
    pub fn kfree(ptr: *const core::ffi::c_void);
}

/* CONFIG_SECURITY_LANDLOCK_LOG */
extern "C" {
    pub fn landlock_init_hierarchy_log(hierarchy: *mut landlock_hierarchy) -> c_int;
}

/* !CONFIG_SECURITY_LANDLOCK_LOG equivalent:
pub unsafe fn landlock_init_hierarchy_log(
    _hierarchy: *mut landlock_hierarchy,
) -> c_int {
    0
}
*/

#[inline]
pub unsafe fn landlock_free_hierarchy_details(hierarchy: *mut landlock_hierarchy) {
    if hierarchy.is_null() || (*hierarchy).details.is_null() {
        return;
    }

    put_pid((*(*hierarchy).details).pid);
    kfree((*hierarchy).details.cast());
}

/* !CONFIG_SECURITY_LANDLOCK_LOG equivalent:
#[inline]
pub unsafe fn landlock_free_hierarchy_details(_hierarchy: *mut landlock_hierarchy) {}
*/

#[inline]
pub unsafe fn landlock_get_hierarchy(hierarchy: *mut landlock_hierarchy) {
    if !hierarchy.is_null() {
        refcount_inc(&mut (*hierarchy).usage);
    }
}

#[inline]
pub unsafe fn landlock_put_hierarchy(mut hierarchy: *mut landlock_hierarchy) {
    while !hierarchy.is_null() && refcount_dec_and_test(&mut (*hierarchy).usage) {
        let freeme: *const landlock_hierarchy = hierarchy;

        landlock_log_free_domain(hierarchy);
        landlock_free_hierarchy_details(hierarchy);
        hierarchy = (*hierarchy).parent;
        kfree(freeme.cast());
    }
}

/**
 * struct landlock_domain - Immutable Landlock domain
 *
 * A domain is created from a ruleset by landlock_merge_ruleset() and enforced
 * on a task.  Once created, its rules and access masks are immutable.  Unlike
 * &struct landlock_ruleset, a domain has no lock field.
 */
#[repr(C)]
pub struct landlock_domain {
    /**
     * @rules: Red-black tree storage for rules.
     */
    pub rules: landlock_rules,
    /**
     * @hierarchy: Enables hierarchy identification even when a parent
     * domain vanishes.  This is needed for the ptrace and scope
     * restrictions.
     */
    pub hierarchy: *mut landlock_hierarchy,
    pub storage: landlock_domain_storage,
}

#[repr(C)]
pub union landlock_domain_storage {
    /**
     * @work_free: Enables to free a domain within a lockless
     * section.  This is only used by landlock_put_domain_deferred()
     * when @usage reaches zero.  The fields @usage, @num_layers and
     * @handled_masks are then unused.
     */
    pub work_free: ManuallyDrop<work_struct>,
    pub live: ManuallyDrop<landlock_domain_live>,
}

#[repr(C)]
pub struct landlock_domain_live {
    /**
     * @usage: Number of credentials referencing this domain.
     */
    pub usage: refcount_t,
    /**
     * @num_layers: Number of layers that are used in this
     * domain.  This enables to check that all the layers
     * allow an access request.
     */
    pub num_layers: u32,
    /**
     * @handled_masks: Contains the subset of filesystem and
     * network actions that are restricted by a domain.  A
     * domain saves all layers of merged rulesets in a stack
     * (FAM), starting from the first layer to the last one.
     * These layers are used when merging rulesets, for user
     * space backward compatibility (i.e. future-proof), and
     * to properly handle merged rulesets without
     * overlapping access rights.  These layers are set once
     * and never changed for the lifetime of the domain.
     */
    pub handled_masks: [access_masks; 0],
}

#[inline]
pub unsafe fn landlock_get_fs_access_mask(
    domain: *const landlock_domain,
    layer_level: u16,
) -> access_mask_t {
    ((*(*domain).storage.live).handled_masks.as_ptr().add(layer_level as usize))
        .read()
        .fs
        | _LANDLOCK_ACCESS_FS_INITIALLY_DENIED
}

#[inline]
pub unsafe fn landlock_get_net_access_mask(
    domain: *const landlock_domain,
    layer_level: u16,
) -> access_mask_t {
    ((*(*domain).storage.live).handled_masks.as_ptr().add(layer_level as usize))
        .read()
        .net
}

#[inline]
pub unsafe fn landlock_get_scope_mask(
    domain: *const landlock_domain,
    layer_level: u16,
) -> access_mask_t {
    ((*(*domain).storage.live).handled_masks.as_ptr().add(layer_level as usize))
        .read()
        .scope
}

/**
 * landlock_union_access_masks - Return all access rights handled in the
 *                              domain
 *
 * @domain: Landlock domain
 *
 * Return: An access_masks result of the OR of all the domain's access masks.
 */
#[inline]
pub unsafe fn landlock_union_access_masks(domain: *const landlock_domain) -> access_masks {
    let mut matches = access_masks_all { all: 0 };
    let mut layer_level: usize = 0;

    while layer_level < (*(*domain).storage.live).num_layers as usize {
        let layer = access_masks_all {
            masks: ((*(*domain).storage.live)
                .handled_masks
                .as_ptr()
                .add(layer_level))
            .read(),
        };

        matches.all |= layer.all;
        layer_level += 1;
    }

    matches.masks
}

extern "C" {
    pub fn landlock_put_domain(domain: *mut landlock_domain);
    pub fn landlock_put_domain_deferred(domain: *mut landlock_domain);
}

/* DEFINE_FREE(landlock_put_domain, struct landlock_domain *,
 *             if (!IS_ERR_OR_NULL(_T)) landlock_put_domain(_T))
 */
pub unsafe fn landlock_put_domain_free(domain: *mut landlock_domain) {
    if !IS_ERR_OR_NULL(domain.cast()) {
        landlock_put_domain(domain);
    }
}

extern "C" {
    pub fn landlock_merge_ruleset(
        parent: *mut landlock_domain,
        ruleset: *mut landlock_ruleset,
    ) -> *mut landlock_domain;

    pub fn landlock_unmask_layers(
        domain: *const landlock_domain,
        id: landlock_id,
        masks: *mut layer_masks,
        matched_rule: *mut *const landlock_rule,
    ) -> bool;

    pub fn landlock_init_layer_masks(
        domain: *const landlock_domain,
        access_request: access_mask_t,
        masks: *mut layer_masks,
        key_type: landlock_key_type,
    ) -> access_mask_t;
}

#[inline]
pub unsafe fn landlock_get_domain(domain: *mut landlock_domain) {
    if !domain.is_null() {
        refcount_inc(&mut (*(*domain).storage.live).usage);
    }
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
