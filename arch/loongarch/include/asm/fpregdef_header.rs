/* SPDX-License-Identifier: GPL-2.0 */
/*
 * Definitions for the FPU register names
 *
 * Copyright (C) 2020-2022 Loongson Technology Corporation Limited
 */

pub const fa0: &str = "$f0"; /* argument registers, fa0/fa1 reused as fv0/fv1 for return value */
pub const fa1: &str = "$f1";
pub const fa2: &str = "$f2";
pub const fa3: &str = "$f3";
pub const fa4: &str = "$f4";
pub const fa5: &str = "$f5";
pub const fa6: &str = "$f6";
pub const fa7: &str = "$f7";
pub const ft0: &str = "$f8"; /* caller saved */
pub const ft1: &str = "$f9";
pub const ft2: &str = "$f10";
pub const ft3: &str = "$f11";
pub const ft4: &str = "$f12";
pub const ft5: &str = "$f13";
pub const ft6: &str = "$f14";
pub const ft7: &str = "$f15";
pub const ft8: &str = "$f16";
pub const ft9: &str = "$f17";
pub const ft10: &str = "$f18";
pub const ft11: &str = "$f19";
pub const ft12: &str = "$f20";
pub const ft13: &str = "$f21";
pub const ft14: &str = "$f22";
pub const ft15: &str = "$f23";
pub const fs0: &str = "$f24"; /* callee saved */
pub const fs1: &str = "$f25";
pub const fs2: &str = "$f26";
pub const fs3: &str = "$f27";
pub const fs4: &str = "$f28";
pub const fs5: &str = "$f29";
pub const fs6: &str = "$f30";
pub const fs7: &str = "$f31";

/*
 * Build-time condition preserved from CONFIG_AS_HAS_FCSR_CLASS:
 * when unavailable, current binutils expects GPRs at FCSR positions.
 */
#[cfg(not(CONFIG_AS_HAS_FCSR_CLASS))]
pub const fcsr0: &str = "$r0";
#[cfg(not(CONFIG_AS_HAS_FCSR_CLASS))]
pub const fcsr1: &str = "$r1";
#[cfg(not(CONFIG_AS_HAS_FCSR_CLASS))]
pub const fcsr2: &str = "$r2";
#[cfg(not(CONFIG_AS_HAS_FCSR_CLASS))]
pub const fcsr3: &str = "$r3";

#[cfg(CONFIG_AS_HAS_FCSR_CLASS)]
pub const fcsr0: &str = "$fcsr0";
#[cfg(CONFIG_AS_HAS_FCSR_CLASS)]
pub const fcsr1: &str = "$fcsr1";
#[cfg(CONFIG_AS_HAS_FCSR_CLASS)]
pub const fcsr2: &str = "$fcsr2";
#[cfg(CONFIG_AS_HAS_FCSR_CLASS)]
pub const fcsr3: &str = "$fcsr3";

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
