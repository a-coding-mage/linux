/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * AArch64 KGDB support
 *
 * Based on arch/arm/include/kgdb.h
 *
 * Copyright (C) 2013 Cavium Inc.
 * Author: Vijaya Kumar K <vijaya.kumar@caviumnetworks.com>
 */

// C dependencies: linux::ptrace and asm::debug_monitors provide `PtRegs`,
// `KGDB_COMPILED_DBG_BRK_IMM`, and `DBG_HOOK_ERROR`.

#[cfg(not(target_arch = "aarch64"))]
compile_error!("this header targets AArch64");

pub unsafe fn arch_kgdb_breakpoint() {
    core::arch::asm!("brk {imm}", imm = const KGDB_COMPILED_DBG_BRK_IMM);
}

unsafe extern "C" {
    pub fn kgdb_handle_bus_error();
    pub static mut kgdb_fault_expected: core::ffi::c_int;

    pub fn kgdb_brk_handler(regs: *mut PtRegs, esr: core::ffi::c_ulong) -> core::ffi::c_int;
    pub fn kgdb_compiled_brk_handler(
        regs: *mut PtRegs,
        esr: core::ffi::c_ulong,
    ) -> core::ffi::c_int;
}

#[cfg(feature = "CONFIG_KGDB")]
unsafe extern "C" {
    pub fn kgdb_single_step_handler(
        regs: *mut PtRegs,
        esr: core::ffi::c_ulong,
    ) -> core::ffi::c_int;
}

#[cfg(not(feature = "CONFIG_KGDB"))]
pub unsafe fn kgdb_single_step_handler(
    _regs: *mut PtRegs,
    _esr: core::ffi::c_ulong,
) -> core::ffi::c_int {
    DBG_HOOK_ERROR
}

/*
 * gdb remote procotol (well most versions of it) expects the following
 * register layout.
 *
 * General purpose regs:
 *     r0-r30: 64 bit
 *     sp,pc : 64 bit
 *     pstate  : 32 bit
 *     Total: 33 + 1
 * FPU regs:
 *     f0-f31: 128 bit
 *     fpsr & fpcr: 32 bit
 *     Total: 32 + 2
 *
 * To expand a little on the "most versions of it"... when the gdb remote
 * protocol for AArch64 was developed it depended on a statement in the
 * Architecture Reference Manual that claimed "SPSR_ELx is a 32-bit register".
 * and, as a result, allocated only 32-bits for the PSTATE in the remote
 * protocol. In fact this statement is still present in ARM DDI 0487A.i.
 *
 * Unfortunately "is a 32-bit register" has a very special meaning for
 * system registers. It means that "the upper bits, bits[63:32], are
 * RES0.". RES0 is heavily used in the ARM architecture documents as a way
 * to leave space for future architecture changes. So to translate a little
 * for people who don't spend their spare time reading ARM architecture
 * manuals, what "is a 32-bit register" actually means in this context is
 * "is a 64-bit register but one with no meaning allocated to any of the
 * upper 32-bits... *yet*".
 *
 * Perhaps then we should not be surprised that this has led to some
 * confusion. Specifically a patch, influenced by the above translation,
 * that extended PSTATE to 64-bit was accepted into gdb-7.7 but the patch
 * was reverted in gdb-7.8.1 and all later releases, when this was
 * discovered to be an undocumented protocol change.
 *
 * So... it is *not* wrong for us to only allocate 32-bits to PSTATE
 * here even though the kernel itself allocates 64-bits for the same
 * state. That is because this bit of code tells the kernel how the gdb
 * remote protocol (well most versions of it) describes the register state.
 *
 * Note that if you are using one of the versions of gdb that supports
 * the gdb-7.7 version of the protocol you cannot use kgdb directly
 * without providing a custom register description (gdb can load new
 * protocol descriptions at runtime).
 */

pub const _GP_REGS: usize = 33;
pub const _FP_REGS: usize = 32;
pub const _EXTRA_REGS: usize = 3;
pub const GP_REG_BYTES: usize = _GP_REGS * 8;
pub const DBG_MAX_REG_NUM: usize = _GP_REGS + _FP_REGS + _EXTRA_REGS;
pub const BUFMAX: usize = 2048;
pub const NUMREGBYTES: usize = (_GP_REGS * 8) + (_FP_REGS * 16) + (_EXTRA_REGS * 4);

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
