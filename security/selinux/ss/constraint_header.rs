/* SPDX-License-Identifier: GPL-2.0 */
/*
 * A constraint is a condition that must be satisfied in
 * order for one or more permissions to be granted.
 * Constraints are used to impose additional restrictions
 * beyond the type-based rules in `te' or the role-based
 * transition rules in `rbac'.  Constraints are typically
 * used to prevent a process from transitioning to a new user
 * identity or role unless it is in a privileged type.
 * Constraints are likewise typically used to prevent a
 * process from labeling an object with a different user
 * identity.
 *
 * Author : Stephen Smalley, <stephen.smalley.work@gmail.com>
 */

/* Dependency from C header: "ebitmap.h" */

pub const CEXPR_MAXDEPTH: u32 = 5;

pub const CEXPR_NOT: u32 = 1; /* not expr */
pub const CEXPR_AND: u32 = 2; /* expr and expr */
pub const CEXPR_OR: u32 = 3; /* expr or expr */
pub const CEXPR_ATTR: u32 = 4; /* attr op attr */
pub const CEXPR_NAMES: u32 = 5; /* attr op names */

pub const CEXPR_USER: u32 = 1; /* user */
pub const CEXPR_ROLE: u32 = 2; /* role */
pub const CEXPR_TYPE: u32 = 4; /* type */
pub const CEXPR_TARGET: u32 = 8; /* target if set, source otherwise */
pub const CEXPR_XTARGET: u32 = 16; /* special 3rd target for validatetrans rule */
pub const CEXPR_L1L2: u32 = 32; /* low level 1 vs. low level 2 */
pub const CEXPR_L1H2: u32 = 64; /* low level 1 vs. high level 2 */
pub const CEXPR_H1L2: u32 = 128; /* high level 1 vs. low level 2 */
pub const CEXPR_H1H2: u32 = 256; /* high level 1 vs. high level 2 */
pub const CEXPR_L1H1: u32 = 512; /* low level 1 vs. high level 1 */
pub const CEXPR_L2H2: u32 = 1024; /* low level 2 vs. high level 2 */

pub const CEXPR_EQ: u32 = 1; /* == or eq */
pub const CEXPR_NEQ: u32 = 2; /* != */
pub const CEXPR_DOM: u32 = 3; /* dom */
pub const CEXPR_DOMBY: u32 = 4; /* domby  */
pub const CEXPR_INCOMP: u32 = 5; /* incomp */

#[repr(C)]
pub struct constraint_expr {
    pub expr_type: u32, /* expression type */
    pub attr: u32,      /* attribute */
    pub op: u32,        /* operator */

    pub names: ebitmap, /* names */
    /* internally unused, only forwarded via policydb_write() */
    pub type_names: *mut type_set,

    pub next: *mut constraint_expr, /* next expression */
}

#[repr(C)]
pub struct constraint_node {
    pub permissions: u32,             /* constrained permissions */
    pub expr: *mut constraint_expr,   /* constraint on permissions */
    pub next: *mut constraint_node,   /* next constraint */
}


// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
