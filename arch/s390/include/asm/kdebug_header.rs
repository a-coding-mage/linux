/* SPDX-License-Identifier: GPL-2.0 */

/*
 * Feb 2006 Ported to s390 <grundym@us.ibm.com>
 */

#[repr(C)]
pub struct pt_regs {
    _private: [u8; 0],
}

#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum die_val {
    DIE_OOPS = 1,
    DIE_BPT,
    DIE_SSTEP,
    DIE_PANIC,
    DIE_NMI,
    DIE_DIE,
    DIE_NMIWATCHDOG,
    DIE_KERNELDEBUG,
    DIE_TRAP,
    DIE_GPF,
    DIE_CALL,
    DIE_NMI_IPI,
}

extern "C" {
    pub fn die(regs: *mut pt_regs, str: *const core::ffi::c_char) -> !;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
