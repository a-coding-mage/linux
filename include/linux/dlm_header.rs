/* SPDX-License-Identifier: GPL-2.0-only */
/******************************************************************************
*******************************************************************************
**
**  Copyright (C) Sistina Software, Inc.  1997-2003  All rights reserved.
**  Copyright (C) 2004-2011 Red Hat, Inc.  All rights reserved.
**
**
*******************************************************************************
******************************************************************************/

// Dependency provided by the corresponding Linux DLM UAPI bindings.

#[repr(C)]
pub struct dlm_slot {
    pub nodeid: ::core::ffi::c_int, /* 1 to MAX_INT */
    pub slot: ::core::ffi::c_int,   /* 1 to MAX_INT */
}

/*
 * recover_prep: called before the dlm begins lock recovery.
 *   Notfies lockspace user that locks from failed members will be granted.
 * recover_slot: called after recover_prep and before recover_done.
 *   Identifies a failed lockspace member.
 * recover_done: called after the dlm completes lock recovery.
 *   Identifies lockspace members and lockspace generation number.
 */

#[repr(C)]
pub struct dlm_lockspace_ops {
    pub recover_prep:
        Option<unsafe extern "C" fn(ops_arg: *mut ::core::ffi::c_void)>,
    pub recover_slot: Option<unsafe extern "C" fn(
        ops_arg: *mut ::core::ffi::c_void,
        slot: *mut dlm_slot,
    )>,
    pub recover_done: Option<unsafe extern "C" fn(
        ops_arg: *mut ::core::ffi::c_void,
        slots: *mut dlm_slot,
        num_slots: ::core::ffi::c_int,
        our_slot: ::core::ffi::c_int,
        generation: u32,
    )>,
}

/* only relevant for kernel lockspaces, will be removed in future */
pub const DLM_LSFL_SOFTIRQ: u32 = __DLM_LSFL_RESERVED0;

extern "C" {
    pub fn dlm_new_lockspace(
        name: *const ::core::ffi::c_char,
        cluster: *const ::core::ffi::c_char,
        flags: u32,
        lvblen: ::core::ffi::c_int,
        ops: *const dlm_lockspace_ops,
        ops_arg: *mut ::core::ffi::c_void,
        ops_result: *mut ::core::ffi::c_int,
        lockspace: *mut *mut dlm_lockspace_t,
    ) -> ::core::ffi::c_int;
}

/*
 * dlm_release_lockspace() release_option values:
 *
 * DLM_RELEASE_NO_LOCKS returns -EBUSY if any locks (lkb's)
 *   exist in the local lockspace.
 *
 * DLM_RELEASE_UNUSED previous value that is no longer used.
 *
 * DLM_RELEASE_NORMAL releases the lockspace regardless of any
 *   locks managed in the local lockspace.
 *
 * DLM_RELEASE_NO_EVENT release the lockspace regardless of any
 *   locks managed in the local lockspace, and does not submit
 *   a leave event to the cluster manager, so other nodes will
 *   not be notified that the node should be removed from the
 *   list of lockspace members.
 *
 * DLM_RELEASE_RECOVER like DLM_RELEASE_NORMAL, but the remaining
 *   nodes will handle the removal of the node as if the node
 *   had failed, e.g. the recover_slot() callback would be used.
 */
pub const DLM_RELEASE_NO_LOCKS: ::core::ffi::c_uint = 0;
pub const DLM_RELEASE_UNUSED: ::core::ffi::c_uint = 1;
pub const DLM_RELEASE_NORMAL: ::core::ffi::c_uint = 2;
pub const DLM_RELEASE_NO_EVENT: ::core::ffi::c_uint = 3;
pub const DLM_RELEASE_RECOVER: ::core::ffi::c_uint = 4;
pub const __DLM_RELEASE_MAX: ::core::ffi::c_uint = DLM_RELEASE_RECOVER;

extern "C" {
    pub fn dlm_release_lockspace(
        lockspace: *mut dlm_lockspace_t,
        release_option: ::core::ffi::c_uint,
    ) -> ::core::ffi::c_int;

    pub fn dlm_lock(
        lockspace: *mut dlm_lockspace_t,
        mode: ::core::ffi::c_int,
        lksb: *mut dlm_lksb,
        flags: u32,
        name: *const ::core::ffi::c_void,
        namelen: ::core::ffi::c_uint,
        parent_lkid: u32,
        lockast: Option<unsafe extern "C" fn(astarg: *mut ::core::ffi::c_void)>,
        astarg: *mut ::core::ffi::c_void,
        bast: Option<unsafe extern "C" fn(
            astarg: *mut ::core::ffi::c_void,
            mode: ::core::ffi::c_int,
        )>,
    ) -> ::core::ffi::c_int;

    pub fn dlm_unlock(
        lockspace: *mut dlm_lockspace_t,
        lkid: u32,
        flags: u32,
        lksb: *mut dlm_lksb,
        astarg: *mut ::core::ffi::c_void,
    ) -> ::core::ffi::c_int;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
