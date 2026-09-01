/* SPDX-License-Identifier: GPL-2.0-or-later */
/*
 * Copyright (C) 2015-2017 Josh Poimboeuf <jpoimboe@redhat.com>
 */

/* Dependencies from the original header:
 * #include <arch/cfi_regs.h>
 * #include <linux/list.h>
 */

pub const CFI_UNDEFINED: i32 = -1;
pub const CFI_CFA: i32 = -2;
pub const CFI_SP_INDIRECT: i32 = -3;
pub const CFI_BP_INDIRECT: i32 = -4;

#[repr(C)]
pub struct cfi_reg {
    pub base: ::std::os::raw::c_int,
    pub offset: ::std::os::raw::c_int,
}

#[repr(C)]
pub struct cfi_init_state {
    pub regs: [cfi_reg; CFI_NUM_REGS],
    pub cfa: cfi_reg,
}

#[repr(C)]
pub struct cfi_state {
    pub hash: hlist_node, /* must be first, cficmp() */
    pub regs: [cfi_reg; CFI_NUM_REGS],
    pub vals: [cfi_reg; CFI_NUM_REGS],
    pub cfa: cfi_reg,
    pub stack_size: ::std::os::raw::c_int,
    pub drap_reg: ::std::os::raw::c_int,
    pub drap_offset: ::std::os::raw::c_int,
    pub type_: ::std::os::raw::c_uchar,
    pub bp_scratch: bool,
    pub drap: bool,
    pub signal: bool,
    pub end: bool,
    pub force_undefined: bool,
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
