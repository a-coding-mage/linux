/* SPDX-License-Identifier: GPL-2.0 */
/*
 * Preliminary support for HW exception handing for Microblaze
 *
 * Copyright (C) 2008-2009 Michal Simek <monstr@monstr.eu>
 * Copyright (C) 2008-2009 PetaLogix
 * Copyright (C) 2005 John Williams <jwilliams@itee.uq.edu.au>
 */

/* Original declarations are available only to the kernel and non-assembler
 * translation units.  The corresponding build-time conditions are preserved
 * here as comments because their definitions are supplied externally.
 */

/* Macros to enable and disable HW exceptions in the MSR */
/* Define MSR enable bit for HW exceptions */
pub const HWEX_MSR_BIT: u32 = 1u32 << 8;

/* CONFIG_XILINX_MICROBLAZE0_USE_MSR_INSTR selects the first implementation. */
#[cfg(feature = "CONFIG_XILINX_MICROBLAZE0_USE_MSR_INSTR")]
#[inline]
pub unsafe fn __enable_hw_exceptions() {
    core::arch::asm!("msrset r0, {0}; nop;", const HWEX_MSR_BIT, options(nostack));
}

#[cfg(feature = "CONFIG_XILINX_MICROBLAZE0_USE_MSR_INSTR")]
#[inline]
pub unsafe fn __disable_hw_exceptions() {
    core::arch::asm!("msrclr r0, {0}; nop;", const HWEX_MSR_BIT, options(nostack));
}

/* !CONFIG_XILINX_MICROBLAZE0_USE_MSR_INSTR */
#[cfg(not(feature = "CONFIG_XILINX_MICROBLAZE0_USE_MSR_INSTR"))]
#[inline]
pub unsafe fn __enable_hw_exceptions() {
    core::arch::asm!(
        "mfs r12, rmsr; nop; ori r12, r12, {0}; mts rmsr, r12; nop;",
        const HWEX_MSR_BIT,
        out("r12") _,
        options(nostack)
    );
}

#[cfg(not(feature = "CONFIG_XILINX_MICROBLAZE0_USE_MSR_INSTR"))]
#[inline]
pub unsafe fn __disable_hw_exceptions() {
    core::arch::asm!(
        "mfs r12, rmsr; nop; andi r12, r12, {0}; mts rmsr, r12; nop;",
        const HWEX_MSR_BIT,
        out("r12") _,
        options(nostack)
    );
}

extern "C" {
    pub fn full_exception(regs: *mut pt_regs, type_: u32, fsr: i32, addr: i32);
    pub fn sw_exception(regs: *mut pt_regs);
    pub fn bad_page_fault(regs: *mut pt_regs, address: core::ffi::c_ulong, sig: i32);
    pub fn die(str_: *const core::ffi::c_char, fp: *mut pt_regs, err: core::ffi::c_long);
    pub fn _exception(signr: i32, regs: *mut pt_regs, code: i32, addr: core::ffi::c_ulong);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
