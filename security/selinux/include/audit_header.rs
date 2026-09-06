/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * SELinux support for the Audit LSM hooks
 *
 * Author: James Morris <jmorris@redhat.com>
 *
 * Copyright (C) 2005 Red Hat, Inc., James Morris <jmorris@redhat.com>
 * Copyright (C) 2006 Trusted Computer Solutions, Inc. <dgoeddel@trustedcs.com>
 * Copyright (C) 2006 IBM Corporation, Timothy R. Chavez <tinytim@us.ibm.com>
 */

/* C header dependencies: <linux/audit.h>, <linux/types.h>. */

use core::ffi::{c_char, c_int, c_uint, c_void};

pub type gfp_t = c_uint;

#[repr(C)]
pub struct lsm_prop {
    _private: [u8; 0],
}

#[repr(C)]
pub struct audit_krule {
    _private: [u8; 0],
}

unsafe extern "C" {
    /**
     * selinux_audit_rule_avc_callback - update the audit LSM rules on AVC events.
     * @event: the AVC event
     *
     * Update any audit LSM rules based on the AVC event specified in @event.
     * Returns 0 on success, negative values otherwise.
     */
    pub fn selinux_audit_rule_avc_callback(event: u32) -> c_int;

    /**
     * selinux_audit_rule_init - alloc/init an selinux audit rule structure.
     * @field: the field this rule refers to
     * @op: the operator the rule uses
     * @rulestr: the text "target" of the rule
     * @rule: pointer to the new rule structure returned via this
     * @gfp: GFP flag used for kmalloc
     *
     * Returns 0 if successful, -errno if not.  On success, the rule structure
     * will be allocated internally.  The caller must free this structure with
     * selinux_audit_rule_free() after use.
     */
    pub fn selinux_audit_rule_init(
        field: u32,
        op: u32,
        rulestr: *mut c_char,
        rule: *mut *mut c_void,
        gfp: gfp_t,
    ) -> c_int;

    /**
     * selinux_audit_rule_free - free an selinux audit rule structure.
     * @rule: pointer to the audit rule to be freed
     *
     * This will free all memory associated with the given rule.
     * If @rule is NULL, no operation is performed.
     */
    pub fn selinux_audit_rule_free(rule: *mut c_void);

    /**
     * selinux_audit_rule_match - determine if a context ID matches a rule.
     * @prop: includes the context ID to check
     * @field: the field this rule refers to
     * @op: the operator the rule uses
     * @rule: pointer to the audit rule to check against
     *
     * Returns 1 if the context id matches the rule, 0 if it does not, and
     * -errno on failure.
     */
    pub fn selinux_audit_rule_match(
        prop: *mut lsm_prop,
        field: u32,
        op: u32,
        rule: *mut c_void,
    ) -> c_int;

    /**
     * selinux_audit_rule_known - check to see if rule contains selinux fields.
     * @rule: rule to be checked
     * Returns 1 if there are selinux fields specified in the rule, 0 otherwise.
     */
    pub fn selinux_audit_rule_known(rule: *mut audit_krule) -> c_int;
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
