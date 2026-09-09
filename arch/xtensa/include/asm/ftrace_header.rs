/*
 * arch/xtensa/include/asm/ftrace.h
 *
 * This file is subject to the terms and conditions of the GNU General Public
 * License.  See the file "COPYING" in the main directory of this archive
 * for more details.
 *
 * Copyright (C) 2013 Tensilica Inc.
 */

// Dependency supplied by the surrounding Xtensa environment:
// #include <asm/processor.h>

#[cfg(not(feature = "assembler"))]
unsafe extern "C" {
    pub fn return_address(level: core::ffi::c_uint) -> core::ffi::c_ulong;
}

#[cfg(not(feature = "assembler"))]
#[inline(always)]
pub unsafe fn ftrace_return_address(n: core::ffi::c_uint) -> core::ffi::c_ulong {
    // C macro: return_address(n)
    unsafe { return_address(n) }
}

// The following declarations are enabled when CONFIG_FUNCTION_TRACER is set
// in the originating build configuration.
#[cfg(feature = "function_tracer")]
pub const MCOUNT_INSN_SIZE: usize = 3;

#[cfg(feature = "function_tracer")]
unsafe extern "C" {
    pub fn _mcount();
}

#[cfg(feature = "function_tracer")]
#[inline(always)]
pub unsafe fn mcount() {
    // C alias: #define mcount _mcount
    unsafe { _mcount() }
}

#[cfg(feature = "function_tracer")]
#[inline(always)]
pub fn mcount_addr() -> core::ffi::c_ulong {
    // C macro: ((unsigned long)(_mcount))
    _mcount as usize as core::ffi::c_ulong
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
