/* SPDX-License-Identifier: GPL-2.0-only */
/* Authors: Karl MacMillan <kmacmillan@tresys.com>
 *          Frank Mayer <mayerf@tresys.com>
 *          Copyright (C) 2003 - 2004 Tresys Technology, LLC
 */

/* C header dependencies:
 * "avtab.h", "symtab.h", "policydb.h", "../include/conditional.h"
 */

use core::ffi::{c_int, c_void};

pub const COND_EXPR_MAXDEPTH: u32 = 10;

/*
 * A conditional expression is a list of operators and operands
 * in reverse polish notation.
 */
pub const COND_BOOL: u32 = 1; /* plain bool */
pub const COND_NOT: u32 = 2; /* !bool */
pub const COND_OR: u32 = 3; /* bool || bool */
pub const COND_AND: u32 = 4; /* bool && bool */
pub const COND_XOR: u32 = 5; /* bool ^ bool */
pub const COND_EQ: u32 = 6; /* bool == bool */
pub const COND_NEQ: u32 = 7; /* bool != bool */
pub const COND_LAST: u32 = COND_NEQ;

#[repr(C)]
pub struct cond_expr_node {
    pub expr_type: u32,
    pub boolean: u32,
}

#[repr(C)]
pub struct cond_expr {
    pub nodes: *mut cond_expr_node,
    pub len: u32,
}

/*
 * Each cond_node contains a list of rules to be enabled/disabled
 * depending on the current value of the conditional expression. This
 * struct is for that list.
 */
#[repr(C)]
pub struct cond_av_list {
    pub nodes: *mut *mut avtab_node,
    pub len: u32,
}

/*
 * A cond node represents a conditional block in a policy. It
 * contains a conditional expression, the current state of the expression,
 * two lists of rules to enable/disable depending on the value of the
 * expression (the true list corresponds to if and the false list corresponds
 * to else)..
 */
#[repr(C)]
pub struct cond_node {
    pub cur_state: c_int,
    pub expr: cond_expr,
    pub true_list: cond_av_list,
    pub false_list: cond_av_list,
}

unsafe extern "C" {
    pub type av_decision;
    pub type avtab;
    pub type avtab_key;
    pub type avtab_node;
    pub type extended_perms;
    pub type extended_perms_decision;
    pub type policy_file;
    pub type policydb;
    pub type symtab;

    pub fn cond_policydb_init(p: *mut policydb);
    pub fn cond_policydb_destroy(p: *mut policydb);

    pub fn cond_init_bool_indexes(p: *mut policydb) -> c_int;
    pub fn cond_destroy_bool(key: *mut c_void, datum: *mut c_void, p: *mut c_void) -> c_int;

    pub fn cond_index_bool(key: *mut c_void, datum: *mut c_void, datap: *mut c_void) -> c_int;

    pub fn cond_read_bool(p: *mut policydb, s: *mut symtab, fp: *mut policy_file) -> c_int;
    pub fn cond_read_list(p: *mut policydb, fp: *mut policy_file) -> c_int;
    pub fn cond_write_bool(key: *mut c_void, datum: *mut c_void, ptr: *mut c_void) -> c_int;
    pub fn cond_write_list(p: *mut policydb, fp: *mut policy_file) -> c_int;

    pub fn cond_compute_av(
        ctab: *mut avtab,
        key: *mut avtab_key,
        avd: *mut av_decision,
        xperms: *mut extended_perms,
    );
    pub fn cond_compute_xperms(
        ctab: *mut avtab,
        key: *mut avtab_key,
        xpermd: *mut extended_perms_decision,
    );
    pub fn evaluate_cond_nodes(p: *mut policydb);
    pub fn cond_policydb_destroy_dup(p: *mut policydb);
    pub fn cond_policydb_dup(r#new: *mut policydb, orig: *const policydb) -> c_int;
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
