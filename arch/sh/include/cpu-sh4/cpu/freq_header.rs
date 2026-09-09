/* SPDX-License-Identifier: GPL-2.0
 *
 * include/asm-sh/cpu-sh4/freq.h
 *
 * Copyright (C) 2002, 2003 Paul Mundt
 */

// C preprocessor configuration conditions are represented as Rust `cfg`
// feature conditions below.

#[cfg(any(
    feature = "CONFIG_CPU_SUBTYPE_SH7722",
    feature = "CONFIG_CPU_SUBTYPE_SH7723",
    feature = "CONFIG_CPU_SUBTYPE_SH7343",
    feature = "CONFIG_CPU_SUBTYPE_SH7366",
))]
pub const FRQCR: usize = 0xa4150000;
#[cfg(any(
    feature = "CONFIG_CPU_SUBTYPE_SH7722",
    feature = "CONFIG_CPU_SUBTYPE_SH7723",
    feature = "CONFIG_CPU_SUBTYPE_SH7343",
    feature = "CONFIG_CPU_SUBTYPE_SH7366",
))]
pub const VCLKCR: usize = 0xa4150004;
#[cfg(any(
    feature = "CONFIG_CPU_SUBTYPE_SH7722",
    feature = "CONFIG_CPU_SUBTYPE_SH7723",
    feature = "CONFIG_CPU_SUBTYPE_SH7343",
    feature = "CONFIG_CPU_SUBTYPE_SH7366",
))]
pub const SCLKACR: usize = 0xa4150008;
#[cfg(any(
    feature = "CONFIG_CPU_SUBTYPE_SH7722",
    feature = "CONFIG_CPU_SUBTYPE_SH7723",
    feature = "CONFIG_CPU_SUBTYPE_SH7343",
    feature = "CONFIG_CPU_SUBTYPE_SH7366",
))]
pub const SCLKBCR: usize = 0xa415000c;
#[cfg(any(
    feature = "CONFIG_CPU_SUBTYPE_SH7722",
    feature = "CONFIG_CPU_SUBTYPE_SH7723",
    feature = "CONFIG_CPU_SUBTYPE_SH7343",
    feature = "CONFIG_CPU_SUBTYPE_SH7366",
))]
pub const IrDACLKCR: usize = 0xa4150010;
#[cfg(any(
    feature = "CONFIG_CPU_SUBTYPE_SH7722",
    feature = "CONFIG_CPU_SUBTYPE_SH7723",
    feature = "CONFIG_CPU_SUBTYPE_SH7343",
    feature = "CONFIG_CPU_SUBTYPE_SH7366",
))]
pub const MSTPCR0: usize = 0xa4150030;
#[cfg(any(
    feature = "CONFIG_CPU_SUBTYPE_SH7722",
    feature = "CONFIG_CPU_SUBTYPE_SH7723",
    feature = "CONFIG_CPU_SUBTYPE_SH7343",
    feature = "CONFIG_CPU_SUBTYPE_SH7366",
))]
pub const MSTPCR1: usize = 0xa4150034;
#[cfg(any(
    feature = "CONFIG_CPU_SUBTYPE_SH7722",
    feature = "CONFIG_CPU_SUBTYPE_SH7723",
    feature = "CONFIG_CPU_SUBTYPE_SH7343",
    feature = "CONFIG_CPU_SUBTYPE_SH7366",
))]
pub const MSTPCR2: usize = 0xa4150038;

#[cfg(feature = "CONFIG_CPU_SUBTYPE_SH7757")]
pub const FRQCR: usize = 0xffc80000;
#[cfg(feature = "CONFIG_CPU_SUBTYPE_SH7757")]
pub const OSCCR: usize = 0xffc80018;
#[cfg(feature = "CONFIG_CPU_SUBTYPE_SH7757")]
pub const PLLCR: usize = 0xffc80024;

#[cfg(any(feature = "CONFIG_CPU_SUBTYPE_SH7763", feature = "CONFIG_CPU_SUBTYPE_SH7780"))]
pub const FRQCR: usize = 0xffc80000;

#[cfg(feature = "CONFIG_CPU_SUBTYPE_SH7724")]
pub const FRQCRA: usize = 0xa4150000;
#[cfg(feature = "CONFIG_CPU_SUBTYPE_SH7724")]
pub const FRQCRB: usize = 0xa4150004;
#[cfg(feature = "CONFIG_CPU_SUBTYPE_SH7724")]
pub const VCLKCR: usize = 0xa4150048;
#[cfg(feature = "CONFIG_CPU_SUBTYPE_SH7724")]
pub const FCLKACR: usize = 0xa4150008;
#[cfg(feature = "CONFIG_CPU_SUBTYPE_SH7724")]
pub const FCLKBCR: usize = 0xa415000c;
#[cfg(feature = "CONFIG_CPU_SUBTYPE_SH7724")]
pub const FRQCR: usize = FRQCRA;
#[cfg(feature = "CONFIG_CPU_SUBTYPE_SH7724")]
pub const SCLKACR: usize = FCLKACR;
#[cfg(feature = "CONFIG_CPU_SUBTYPE_SH7724")]
pub const SCLKBCR: usize = FCLKBCR;
#[cfg(feature = "CONFIG_CPU_SUBTYPE_SH7724")]
pub const IrDACLKCR: usize = 0xa4150018;
#[cfg(feature = "CONFIG_CPU_SUBTYPE_SH7724")]
pub const MSTPCR0: usize = 0xa4150030;
#[cfg(feature = "CONFIG_CPU_SUBTYPE_SH7724")]
pub const MSTPCR1: usize = 0xa4150034;
#[cfg(feature = "CONFIG_CPU_SUBTYPE_SH7724")]
pub const MSTPCR2: usize = 0xa4150038;

#[cfg(feature = "CONFIG_CPU_SUBTYPE_SH7734")]
pub const FRQCR0: usize = 0xffc80000;
#[cfg(feature = "CONFIG_CPU_SUBTYPE_SH7734")]
pub const FRQCR2: usize = 0xffc80008;
#[cfg(feature = "CONFIG_CPU_SUBTYPE_SH7734")]
pub const FRQMR1: usize = 0xffc80014;
#[cfg(feature = "CONFIG_CPU_SUBTYPE_SH7734")]
pub const FRQMR2: usize = 0xffc80018;

#[cfg(feature = "CONFIG_CPU_SUBTYPE_SH7785")]
pub const FRQCR0: usize = 0xffc80000;
#[cfg(feature = "CONFIG_CPU_SUBTYPE_SH7785")]
pub const FRQCR1: usize = 0xffc80004;
#[cfg(feature = "CONFIG_CPU_SUBTYPE_SH7785")]
pub const FRQMR1: usize = 0xffc80014;

#[cfg(feature = "CONFIG_CPU_SUBTYPE_SH7786")]
pub const FRQCR0: usize = 0xffc40000;
#[cfg(feature = "CONFIG_CPU_SUBTYPE_SH7786")]
pub const FRQCR1: usize = 0xffc40004;
#[cfg(feature = "CONFIG_CPU_SUBTYPE_SH7786")]
pub const FRQMR1: usize = 0xffc40014;

#[cfg(feature = "CONFIG_CPU_SUBTYPE_SHX3")]
pub const FRQCR0: usize = 0xffc00000;
#[cfg(feature = "CONFIG_CPU_SUBTYPE_SHX3")]
pub const FRQCR1: usize = 0xffc00004;
#[cfg(feature = "CONFIG_CPU_SUBTYPE_SHX3")]
pub const FRQMR1: usize = 0xffc00014;

#[cfg(not(any(
    feature = "CONFIG_CPU_SUBTYPE_SH7722",
    feature = "CONFIG_CPU_SUBTYPE_SH7723",
    feature = "CONFIG_CPU_SUBTYPE_SH7343",
    feature = "CONFIG_CPU_SUBTYPE_SH7366",
    feature = "CONFIG_CPU_SUBTYPE_SH7757",
    feature = "CONFIG_CPU_SUBTYPE_SH7763",
    feature = "CONFIG_CPU_SUBTYPE_SH7780",
    feature = "CONFIG_CPU_SUBTYPE_SH7724",
    feature = "CONFIG_CPU_SUBTYPE_SH7734",
    feature = "CONFIG_CPU_SUBTYPE_SH7785",
    feature = "CONFIG_CPU_SUBTYPE_SH7786",
    feature = "CONFIG_CPU_SUBTYPE_SHX3",
))]
pub const FRQCR: usize = 0xffc00000;
#[cfg(not(any(
    feature = "CONFIG_CPU_SUBTYPE_SH7722",
    feature = "CONFIG_CPU_SUBTYPE_SH7723",
    feature = "CONFIG_CPU_SUBTYPE_SH7343",
    feature = "CONFIG_CPU_SUBTYPE_SH7366",
    feature = "CONFIG_CPU_SUBTYPE_SH7757",
    feature = "CONFIG_CPU_SUBTYPE_SH7763",
    feature = "CONFIG_CPU_SUBTYPE_SH7780",
    feature = "CONFIG_CPU_SUBTYPE_SH7724",
    feature = "CONFIG_CPU_SUBTYPE_SH7734",
    feature = "CONFIG_CPU_SUBTYPE_SH7785",
    feature = "CONFIG_CPU_SUBTYPE_SH7786",
    feature = "CONFIG_CPU_SUBTYPE_SHX3",
))]
pub const FRQCR_PSTBY: usize = 0x0200;
#[cfg(not(any(
    feature = "CONFIG_CPU_SUBTYPE_SH7722",
    feature = "CONFIG_CPU_SUBTYPE_SH7723",
    feature = "CONFIG_CPU_SUBTYPE_SH7343",
    feature = "CONFIG_CPU_SUBTYPE_SH7366",
    feature = "CONFIG_CPU_SUBTYPE_SH7757",
    feature = "CONFIG_CPU_SUBTYPE_SH7763",
    feature = "CONFIG_CPU_SUBTYPE_SH7780",
    feature = "CONFIG_CPU_SUBTYPE_SH7724",
    feature = "CONFIG_CPU_SUBTYPE_SH7734",
    feature = "CONFIG_CPU_SUBTYPE_SH7785",
    feature = "CONFIG_CPU_SUBTYPE_SH7786",
    feature = "CONFIG_CPU_SUBTYPE_SHX3",
))]
pub const FRQCR_PLLEN: usize = 0x0400;
#[cfg(not(any(
    feature = "CONFIG_CPU_SUBTYPE_SH7722",
    feature = "CONFIG_CPU_SUBTYPE_SH7723",
    feature = "CONFIG_CPU_SUBTYPE_SH7343",
    feature = "CONFIG_CPU_SUBTYPE_SH7366",
    feature = "CONFIG_CPU_SUBTYPE_SH7757",
    feature = "CONFIG_CPU_SUBTYPE_SH7763",
    feature = "CONFIG_CPU_SUBTYPE_SH7780",
    feature = "CONFIG_CPU_SUBTYPE_SH7724",
    feature = "CONFIG_CPU_SUBTYPE_SH7734",
    feature = "CONFIG_CPU_SUBTYPE_SH7785",
    feature = "CONFIG_CPU_SUBTYPE_SH7786",
    feature = "CONFIG_CPU_SUBTYPE_SHX3",
))]
pub const FRQCR_CKOEN: usize = 0x0800;

pub const MIN_DIVISOR_NR: usize = 0;
pub const MAX_DIVISOR_NR: usize = 3;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
