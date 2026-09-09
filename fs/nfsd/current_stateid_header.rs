/* SPDX-License-Identifier: GPL-2.0 */

// Dependencies supplied by the corresponding state and XDR definitions.

extern "C" {
    pub fn clear_current_stateid(cstate: *mut crate::nfsd4_compound_state);
}

/*
 * functions to set current state id
 */
extern "C" {
    pub fn nfsd4_set_opendowngradestateid(
        cstate: *mut crate::nfsd4_compound_state,
        op: *mut crate::nfsd4_op_u,
    );
    pub fn nfsd4_set_openstateid(
        cstate: *mut crate::nfsd4_compound_state,
        op: *mut crate::nfsd4_op_u,
    );
    pub fn nfsd4_set_lockstateid(
        cstate: *mut crate::nfsd4_compound_state,
        op: *mut crate::nfsd4_op_u,
    );
    pub fn nfsd4_set_closestateid(
        cstate: *mut crate::nfsd4_compound_state,
        op: *mut crate::nfsd4_op_u,
    );
}

/*
 * functions to consume current state id
 */
extern "C" {
    pub fn nfsd4_get_opendowngradestateid(
        cstate: *mut crate::nfsd4_compound_state,
        op: *mut crate::nfsd4_op_u,
    );
    pub fn nfsd4_get_delegreturnstateid(
        cstate: *mut crate::nfsd4_compound_state,
        op: *mut crate::nfsd4_op_u,
    );
    pub fn nfsd4_get_freestateid(
        cstate: *mut crate::nfsd4_compound_state,
        op: *mut crate::nfsd4_op_u,
    );
    pub fn nfsd4_get_setattrstateid(
        cstate: *mut crate::nfsd4_compound_state,
        op: *mut crate::nfsd4_op_u,
    );
    pub fn nfsd4_get_closestateid(
        cstate: *mut crate::nfsd4_compound_state,
        op: *mut crate::nfsd4_op_u,
    );
    pub fn nfsd4_get_lockustateid(
        cstate: *mut crate::nfsd4_compound_state,
        op: *mut crate::nfsd4_op_u,
    );
    pub fn nfsd4_get_readstateid(
        cstate: *mut crate::nfsd4_compound_state,
        op: *mut crate::nfsd4_op_u,
    );
    pub fn nfsd4_get_writestateid(
        cstate: *mut crate::nfsd4_compound_state,
        op: *mut crate::nfsd4_op_u,
    );
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
