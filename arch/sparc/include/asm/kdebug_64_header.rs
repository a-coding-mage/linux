/* SPDX-License-Identifier: GPL-2.0 */

// Forward declaration of the register structure supplied by another file.
#[repr(C)]
pub struct pt_regs;

unsafe extern "C" {
    pub fn bad_trap(regs: *mut pt_regs, arg: core::ffi::c_long);
}

/* Grossly misnamed. */
#[repr(C)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum die_val {
    DIE_OOPS = 1,
    DIE_DEBUG,      /* ta 0x70 */
    DIE_DEBUG_2,    /* ta 0x71 */
    DIE_BPT,        /* ta 0x73 */
    DIE_SSTEP,      /* ta 0x74 */
    DIE_DIE,
    DIE_TRAP,
    DIE_TRAP_TL1,
    DIE_CALL,
    DIE_NMI,
    DIE_NMIWATCHDOG,
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
