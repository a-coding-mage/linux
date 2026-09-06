// SPDX-License-Identifier: GPL-2.0
/*
 * Implementation of the security services.
 *
 * Author : Stephen Smalley, <stephen.smalley.work@gmail.com>
 */

// C dependency: "policydb.h"

/* Mapping for a single class */
#[repr(C)]
pub struct selinux_mapping {
    pub value: u16,     /* policy value for class */
    pub num_perms: u16, /* number of permissions in class */
    pub perms: [u32; core::mem::size_of::<u32>() * 8], /* policy values for permissions */
}

/* Map for all of the classes, with array size */
#[repr(C)]
pub struct selinux_map {
    pub mapping: *mut selinux_mapping, /* indexed by class */
    pub size: u16,                     /* array size of mapping */
}

#[repr(C)]
pub struct selinux_policy {
    pub sidtab: *mut sidtab,
    pub policydb: policydb,
    pub map: selinux_map,
    pub latest_granting: u32,
}
// C attribute preserved for intent: __randomize_layout

#[repr(C)]
pub struct convert_context_args {
    pub oldp: *mut policydb,
    pub newp: *mut policydb,
}

unsafe extern "C" {
    pub fn services_compute_xperms_drivers(
        xperms: *mut extended_perms,
        node: *mut avtab_node,
    );
    pub fn services_compute_xperms_decision(
        xpermd: *mut extended_perms_decision,
        node: *mut avtab_node,
    );

    pub fn services_convert_context(
        args: *mut convert_context_args,
        oldc: *mut context,
        newc: *mut context,
        gfp_flags: gfp_t,
    ) -> core::ffi::c_int;
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
