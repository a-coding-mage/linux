/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * kgdb support for ARC
 *
 * Copyright (C) 2012 Synopsys, Inc. (www.synopsys.com)
 */

/* C header guard: __ARC_KGDB_H__ */

/* CONFIG_KGDB is a build-time condition from the original header. */
#[cfg(feature = "CONFIG_KGDB")]
/* Dependency supplied by the surrounding translated kernel sources:
 * #include <asm/ptrace.h>
 */

    /* to ensure compatibility with Linux 2.6.35, we don't implement the get/set
     * register API yet */

pub const GDB_MAX_REGS: usize = 87;

pub const BREAK_INSTR_SIZE: usize = 2;
pub const CACHE_FLUSH_IS_SAFE: usize = 1;
pub const NUMREGBYTES: usize = GDB_MAX_REGS * 4;
pub const BUFMAX: usize = 2048;

#[inline]
pub unsafe fn arch_kgdb_breakpoint() {
    core::arch::asm!("trap_s 0x4");
}

extern "C" {
    pub fn kgdb_trap(regs: *mut pt_regs);
}

    /* This is the numbering of registers according to the GDB. See GDB's
     * arc-tdep.h for details.
     *
     * Registers are ordered for GDB 7.5. It is incompatible with GDB 6.8. */
#[repr(isize)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum arc_linux_regnums {
        _R0 = 0,
        _R1,
        _R2,
        _R3,
        _R4,
        _R5,
        _R6,
        _R7,
        _R8,
        _R9,
        _R10,
        _R11,
        _R12,
        _R13,
        _R14,
        _R15,
        _R16,
        _R17,
        _R18,
        _R19,
        _R20,
        _R21,
        _R22,
        _R23,
        _R24,
        _R25,
        _R26,
        _FP = 27,
        __SP = 28,
        _R30 = 30,
        _BLINK = 31,
        _LP_COUNT = 60,
        _STOP_PC = 64,
        _RET = 64,
        _LP_START = 65,
        _LP_END = 66,
        _STATUS32 = 67,
        _ECR = 76,
        _BTA = 82,
}

/* The original non-CONFIG_KGDB branch defines kgdb_trap(regs) as empty. */
#[cfg(not(feature = "CONFIG_KGDB"))]
#[inline]
pub fn kgdb_trap<T>(_regs: *mut T) {}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
