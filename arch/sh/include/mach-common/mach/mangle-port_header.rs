/* SPDX-License-Identifier: GPL-2.0
 *
 * SH version cribbed from the MIPS copy:
 *
 * Copyright (C) 2003, 2004 Ralf Baechle
 */

/*
 * Sane hardware offers swapping of PCI/ISA I/O space accesses in hardware;
 * less sane hardware forces software to fiddle with this...
 *
 * Regardless, if the host bus endianness mismatches that of PCI/ISA, then
 * you can't have the numerical value of data and byte addresses within
 * multibyte quantities both preserved at the same time.  Hence two
 * variations of functions: non-prefixed ones that preserve the value
 * and prefixed ones that preserve byte addresses.  The latters are
 * typically used for moving raw data between a peripheral and memory (cf.
 * string I/O functions), hence the "__mem_" prefix.
 */

/* CONFIG_SWAP_IO_SPACE selects the corresponding C preprocessor branch. */
#[cfg(feature = "CONFIG_SWAP_IO_SPACE")]
macro_rules! ioswabb {
    ($x:expr) => { $x };
}

#[cfg(feature = "CONFIG_SWAP_IO_SPACE")]
macro_rules! __mem_ioswabb {
    ($x:expr) => { $x };
}

#[cfg(feature = "CONFIG_SWAP_IO_SPACE")]
macro_rules! ioswabw {
    ($x:expr) => { le16_to_cpu($x) };
}

#[cfg(feature = "CONFIG_SWAP_IO_SPACE")]
macro_rules! __mem_ioswabw {
    ($x:expr) => { $x };
}

#[cfg(feature = "CONFIG_SWAP_IO_SPACE")]
macro_rules! ioswabl {
    ($x:expr) => { le32_to_cpu($x) };
}

#[cfg(feature = "CONFIG_SWAP_IO_SPACE")]
macro_rules! __mem_ioswabl {
    ($x:expr) => { $x };
}

#[cfg(feature = "CONFIG_SWAP_IO_SPACE")]
macro_rules! ioswabq {
    ($x:expr) => { le64_to_cpu($x) };
}

#[cfg(feature = "CONFIG_SWAP_IO_SPACE")]
macro_rules! __mem_ioswabq {
    ($x:expr) => { $x };
}

#[cfg(not(feature = "CONFIG_SWAP_IO_SPACE"))]
macro_rules! ioswabb {
    ($x:expr) => { $x };
}

#[cfg(not(feature = "CONFIG_SWAP_IO_SPACE"))]
macro_rules! __mem_ioswabb {
    ($x:expr) => { $x };
}

#[cfg(not(feature = "CONFIG_SWAP_IO_SPACE"))]
macro_rules! ioswabw {
    ($x:expr) => { $x };
}

#[cfg(not(feature = "CONFIG_SWAP_IO_SPACE"))]
macro_rules! __mem_ioswabw {
    ($x:expr) => { cpu_to_le16($x) };
}

#[cfg(not(feature = "CONFIG_SWAP_IO_SPACE"))]
macro_rules! ioswabl {
    ($x:expr) => { $x };
}

#[cfg(not(feature = "CONFIG_SWAP_IO_SPACE"))]
macro_rules! __mem_ioswabl {
    ($x:expr) => { cpu_to_le32($x) };
}

#[cfg(not(feature = "CONFIG_SWAP_IO_SPACE"))]
macro_rules! ioswabq {
    ($x:expr) => { $x };
}

#[cfg(not(feature = "CONFIG_SWAP_IO_SPACE"))]
macro_rules! __mem_ioswabq {
    ($x:expr) => { cpu_to_le32($x) };
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
