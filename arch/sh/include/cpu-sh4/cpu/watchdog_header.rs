/* SPDX-License-Identifier: GPL-2.0
 *
 * include/asm-sh/cpu-sh4/watchdog.h
 *
 * Copyright (C) 2002, 2003 Paul Mundt
 * Copyright (C) 2009 Siemens AG
 * Copyright (C) 2009 Sitdikov Valentin
 */

// The original CONFIG_CPU_SUBTYPE_* preprocessor conditions are represented
// here as Cargo feature conditions with the corresponding names.

#[cfg(any(
    feature = "CONFIG_CPU_SUBTYPE_SH7785",
    feature = "CONFIG_CPU_SUBTYPE_SH7780"
))]
pub const WTBST_HIGH: u32 = 0x55;

#[cfg(any(
    feature = "CONFIG_CPU_SUBTYPE_SH7785",
    feature = "CONFIG_CPU_SUBTYPE_SH7780"
))]
pub const WTCNT_R: usize = 0xffcc0010; // WDTCNT

#[cfg(any(
    feature = "CONFIG_CPU_SUBTYPE_SH7785",
    feature = "CONFIG_CPU_SUBTYPE_SH7780"
))]
pub const WTCSR: usize = 0xffcc0004; // WDTCSR

#[cfg(any(
    feature = "CONFIG_CPU_SUBTYPE_SH7785",
    feature = "CONFIG_CPU_SUBTYPE_SH7780"
))]
pub const WTCNT: usize = 0xffcc0000; // WDTST

#[cfg(any(
    feature = "CONFIG_CPU_SUBTYPE_SH7785",
    feature = "CONFIG_CPU_SUBTYPE_SH7780"
))]
pub const WTST: usize = WTCNT;

#[cfg(any(
    feature = "CONFIG_CPU_SUBTYPE_SH7785",
    feature = "CONFIG_CPU_SUBTYPE_SH7780"
))]
pub const WTBST: usize = 0xffcc0008; // WDTBST

#[cfg(any(
    feature = "CONFIG_CPU_SUBTYPE_SH7722",
    feature = "CONFIG_CPU_SUBTYPE_SH7723",
    feature = "CONFIG_CPU_SUBTYPE_SH7724"
))]
pub const WTCNT: usize = 0xa4520000;

#[cfg(any(
    feature = "CONFIG_CPU_SUBTYPE_SH7722",
    feature = "CONFIG_CPU_SUBTYPE_SH7723",
    feature = "CONFIG_CPU_SUBTYPE_SH7724"
))]
pub const WTCSR: usize = 0xa4520004;

#[cfg(not(any(
    feature = "CONFIG_CPU_SUBTYPE_SH7785",
    feature = "CONFIG_CPU_SUBTYPE_SH7780",
    feature = "CONFIG_CPU_SUBTYPE_SH7722",
    feature = "CONFIG_CPU_SUBTYPE_SH7723",
    feature = "CONFIG_CPU_SUBTYPE_SH7724"
)))]
pub const WTCNT: usize = 0xffc00008;

#[cfg(not(any(
    feature = "CONFIG_CPU_SUBTYPE_SH7785",
    feature = "CONFIG_CPU_SUBTYPE_SH7780",
    feature = "CONFIG_CPU_SUBTYPE_SH7722",
    feature = "CONFIG_CPU_SUBTYPE_SH7723",
    feature = "CONFIG_CPU_SUBTYPE_SH7724"
)))]
pub const WTCSR: usize = 0xffc0000c;

/* Bit definitions */
pub const WTCSR_TME: u32 = 0x80;
pub const WTCSR_WT: u32 = 0x40;
pub const WTCSR_RSTS: u32 = 0x20;
pub const WTCSR_WOVF: u32 = 0x10;
pub const WTCSR_IOVF: u32 = 0x08;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
