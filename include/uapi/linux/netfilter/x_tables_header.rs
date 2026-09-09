/* SPDX-License-Identifier: GPL-2.0 WITH Linux-syscall-note */

/* Dependencies supplied by the corresponding Linux headers. */
use core::ffi::c_char;

pub const XT_FUNCTION_MAXNAMELEN: usize = 30;
pub const XT_EXTENSION_MAXNAMELEN: usize = 29;
pub const XT_TABLE_MAXNAMELEN: usize = 32;

#[repr(C)]
pub struct xt_match;
#[repr(C)]
pub struct xt_target;

#[repr(C)]
pub union xt_entry_match_u {
    pub user: xt_entry_match_u_user,
    pub kernel: xt_entry_match_u_kernel,
    pub match_size: u16,
}

#[repr(C)]
pub struct xt_entry_match_u_user {
    pub match_size: u16,
    /* Used by userspace */
    pub name: [c_char; XT_EXTENSION_MAXNAMELEN],
    pub revision: u8,
}

#[repr(C)]
pub struct xt_entry_match_u_kernel {
    pub match_size: u16,
    /* Used inside the kernel */
    pub r#match: *mut xt_match,
}

#[repr(C)]
pub struct xt_entry_match {
    pub u: xt_entry_match_u,
    pub data: [u8; 0],
}

#[repr(C)]
pub union xt_entry_target_u {
    pub user: xt_entry_target_u_user,
    pub kernel: xt_entry_target_u_kernel,
    pub target_size: u16,
}

#[repr(C)]
pub struct xt_entry_target_u_user {
    pub target_size: u16,
    /* Used by userspace */
    pub name: [c_char; XT_EXTENSION_MAXNAMELEN],
    pub revision: u8,
}

#[repr(C)]
pub struct xt_entry_target_u_kernel {
    pub target_size: u16,
    /* Used inside the kernel */
    pub target: *mut xt_target,
}

#[repr(C)]
pub struct xt_entry_target {
    pub u: xt_entry_target_u,
    pub data: [u8; 0],
}

/* XT_TARGET_INIT(__name, __size): initialize an xt_entry_target. */

#[repr(C)]
pub struct xt_standard_target {
    pub target: xt_entry_target,
    pub verdict: i32,
}

#[repr(C)]
pub struct xt_error_target {
    pub target: xt_entry_target,
    pub errorname: [c_char; XT_FUNCTION_MAXNAMELEN],
}

/* The argument to IPT_SO_GET_REVISION_*. Returns highest revision
 * kernel supports, if >= revision. */
#[repr(C)]
pub struct xt_get_revision {
    pub name: [c_char; XT_EXTENSION_MAXNAMELEN],
    pub revision: u8,
}

/* CONTINUE verdict for targets */
pub const XT_CONTINUE: u32 = 0xFFFF_FFFF;

/* For standard target: (-NF_REPEAT - 1). */
pub const XT_RETURN: i32 = -1 - 1; /* NF_REPEAT is supplied by linux headers. */

/* This is a dummy structure to find out the alignment requirement for a struct
 * containing all the fundamental data types that are used in ipt_entry,
 * ip6t_entry and arpt_entry. */
#[repr(C)]
pub struct _xt_align {
    pub u8: u8,
    pub u16: u16,
    pub u32: u32,
    pub u64: u64,
}

/* XT_ALIGN(s) is __ALIGN_KERNEL((s), __alignof__(struct _xt_align)). */

/* Standard return verdict, or do jump. */
pub const XT_STANDARD_TARGET: &str = "";
/* Error verdict. */
pub const XT_ERROR_TARGET: &str = "ERROR";

#[repr(C)]
pub struct xt_counters {
    pub pcnt: u64,
    pub bcnt: u64,
}

#[repr(C)]
pub struct xt_counters_info {
    /* Which table. */
    pub name: [c_char; XT_TABLE_MAXNAMELEN],
    pub num_counters: u32,
    /* The counters (actually `number' of these). */
    pub counters: [xt_counters; 0],
}

pub const XT_INV_PROTO: u8 = 0x40;

/* SET_COUNTER(c,b,p): c.bcnt = b; c.pcnt = p. */
/* ADD_COUNTER(c,b,p): c.bcnt += b; c.pcnt += p. */

/* Userspace iteration macros XT_MATCH_ITERATE, XT_ENTRY_ITERATE_CONTINUE,
 * XT_ENTRY_ITERATE, xt_entry_foreach, and xt_ematch_foreach are represented
 * by their source-level intent here; their callback/type arguments are C
 * macro parameters and have no standalone Rust item equivalent. */

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
