// SPDX-License-Identifier: GPL-2.0

use core::ffi::{c_char, c_int, c_void};

// Supplied by the kernel headers/architecture-specific implementation.
#[repr(C)]
pub struct pt_regs_offset {
    pub name: *const c_char,
    pub offset: c_int,
}

unsafe extern "C" {
    pub static regoffset_table: pt_regs_offset;
    pub fn strcmp(lhs: *const c_char, rhs: *const c_char) -> c_int;
}

// Supplied by the kernel errno definitions.
const EINVAL: c_int = 22;

/**
 * regs_query_register_offset() - query register offset from its name
 * @name: the name of a register
 *
 * regs_query_register_offset() returns the offset of a register in struct
 * pt_regs from its name. If the name is invalid, this returns -EINVAL;
 */
pub unsafe fn regs_query_register_offset(name: *const c_char) -> c_int {
    let mut roff: *const pt_regs_offset = &raw const regoffset_table;
    while !(*roff).name.is_null() {
        if strcmp((*roff).name, name) == 0 {
            return (*roff).offset;
        }
        roff = roff.add(1);
    }
    -EINVAL
}

/**
 * regs_query_register_name() - query register name from its offset
 * @offset: the offset of a register in struct pt_regs.
 *
 * regs_query_register_name() returns the name of a register from its
 * offset in struct pt_regs. If the @offset is invalid, this returns NULL;
 */
pub unsafe fn regs_query_register_name(offset: u32) -> *const c_char {
    let mut roff: *const pt_regs_offset = &raw const regoffset_table;
    while !(*roff).name.is_null() {
        if (*roff).offset as u32 == offset {
            return (*roff).name;
        }
        roff = roff.add(1);
    }
    core::ptr::null()
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
