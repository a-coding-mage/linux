/*
 * Definitions for the FPU register names
 *
 * This file is subject to the terms and conditions of the GNU General Public
 * License.  See the file "COPYING" in the main directory of this archive
 * for more details.
 *
 * Copyright (C) 1995, 1999 Ralf Baechle
 * Copyright (C) 1985 MIPS Computer Systems, Inc.
 * Copyright (C) 1990 - 1992, 1999 Silicon Graphics, Inc.
 */

// The original definitions depend on _MIPS_SIM from <asm/sgidefs.h>.
// They are represented as assembly-register name strings here.

// _MIPS_SIM == _MIPS_SIM_ABI32
// These definitions only cover the R3000-ish 16/32 register model.
// But we're trying to be R3000 friendly anyway ...
#[cfg(feature = "mips_sim_abi32")]
pub mod abi32 {
    pub const fv0: &str = "$f0"; // return value
    pub const fv0f: &str = "$f1";
    pub const fv1: &str = "$f2";
    pub const fv1f: &str = "$f3";
    pub const fa0: &str = "$f12"; // argument registers
    pub const fa0f: &str = "$f13";
    pub const fa1: &str = "$f14";
    pub const fa1f: &str = "$f15";
    pub const ft0: &str = "$f4"; // caller saved
    pub const ft0f: &str = "$f5";
    pub const ft1: &str = "$f6";
    pub const ft1f: &str = "$f7";
    pub const ft2: &str = "$f8";
    pub const ft2f: &str = "$f9";
    pub const ft3: &str = "$f10";
    pub const ft3f: &str = "$f11";
    pub const ft4: &str = "$f16";
    pub const ft4f: &str = "$f17";
    pub const ft5: &str = "$f18";
    pub const ft5f: &str = "$f19";
    pub const fs0: &str = "$f20"; // callee saved
    pub const fs0f: &str = "$f21";
    pub const fs1: &str = "$f22";
    pub const fs1f: &str = "$f23";
    pub const fs2: &str = "$f24";
    pub const fs2f: &str = "$f25";
    pub const fs3: &str = "$f26";
    pub const fs3f: &str = "$f27";
    pub const fs4: &str = "$f28";
    pub const fs4f: &str = "$f29";
    pub const fs5: &str = "$f30";
    pub const fs5f: &str = "$f31";
    pub const fcr31: &str = "$31"; // FPU status register
}

// _MIPS_SIM == _MIPS_SIM_ABI64 || _MIPS_SIM == _MIPS_SIM_NABI32
#[cfg(any(feature = "mips_sim_abi64", feature = "mips_sim_nabi32"))]
pub mod abi64_or_nabi32 {
    pub const fv0: &str = "$f0"; // return value
    pub const fv1: &str = "$f2";
    pub const fa0: &str = "$f12"; // argument registers
    pub const fa1: &str = "$f13";
    pub const fa2: &str = "$f14";
    pub const fa3: &str = "$f15";
    pub const fa4: &str = "$f16";
    pub const fa5: &str = "$f17";
    pub const fa6: &str = "$f18";
    pub const fa7: &str = "$f19";
    pub const ft0: &str = "$f4"; // caller saved
    pub const ft1: &str = "$f5";
    pub const ft2: &str = "$f6";
    pub const ft3: &str = "$f7";
    pub const ft4: &str = "$f8";
    pub const ft5: &str = "$f9";
    pub const ft6: &str = "$f10";
    pub const ft7: &str = "$f11";
    pub const ft8: &str = "$f20";
    pub const ft9: &str = "$f21";
    pub const ft10: &str = "$f22";
    pub const ft11: &str = "$f23";
    pub const ft12: &str = "$f1";
    pub const ft13: &str = "$f3";
    pub const fs0: &str = "$f24"; // callee saved
    pub const fs1: &str = "$f25";
    pub const fs2: &str = "$f26";
    pub const fs3: &str = "$f27";
    pub const fs4: &str = "$f28";
    pub const fs5: &str = "$f29";
    pub const fs6: &str = "$f30";
    pub const fs7: &str = "$f31";
    pub const fcr31: &str = "$31";
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
