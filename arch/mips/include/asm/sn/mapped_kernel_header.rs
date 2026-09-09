/* SPDX-License-Identifier: GPL-2.0 */
/*
 * File created by Kanoj Sarcar 06/06/00.
 * Copyright 2000 Silicon Graphics, Inc.
 */

// Dependency supplied by the surrounding kernel translation.

/*
 * Note on how mapped kernels work: the text and data section is
 * compiled at cksseg segment (LOADADDR = 0xc001c000), and the
 * init/setup/data section gets a 16M virtual address bump in the
 * ld.script file (so that tlblo0 and tlblo1 maps the sections).
 * The vmlinux.64 section addresses are put in the xkseg range
 * using the change-addresses makefile option. Use elfdump -of
 * on IRIX to see where the sections go. The Origin loader loads
 * the two sections contiguously in physical memory. The loader
 * sets the entry point into kernel_entry using a xkphys address,
 * but instead of using 0xa800000001160000, it uses the address
 * 0xa800000000160000, which is where it physically loaded that
 * code. So no jumps can be done before we have switched to using
 * cksseg addresses.
 */

#[macro_export]
macro_rules! REP_BASE {
    () => { CAC_BASE };
}

#[cfg(feature = "CONFIG_MAPPED_KERNEL")]
#[macro_export]
macro_rules! MAPPED_ADDR_RO_TO_PHYS {
    ($x:expr) => { ($x - $crate::REP_BASE!()) };
}

#[cfg(feature = "CONFIG_MAPPED_KERNEL")]
#[macro_export]
macro_rules! MAPPED_ADDR_RW_TO_PHYS {
    ($x:expr) => { ($x - $crate::REP_BASE!() - 16_777_216) };
}

#[cfg(feature = "CONFIG_MAPPED_KERNEL")]
#[macro_export]
macro_rules! MAPPED_KERN_RO_PHYSBASE {
    ($n:expr) => { hub_data($n).kern_vars.kv_ro_baseaddr };
}

#[cfg(feature = "CONFIG_MAPPED_KERNEL")]
#[macro_export]
macro_rules! MAPPED_KERN_RW_PHYSBASE {
    ($n:expr) => { hub_data($n).kern_vars.kv_rw_baseaddr };
}

#[cfg(feature = "CONFIG_MAPPED_KERNEL")]
#[macro_export]
macro_rules! MAPPED_KERN_RO_TO_PHYS {
    ($x:expr) => {
        ((MAPPED_ADDR_RO_TO_PHYS!($x) as ::core::primitive::usize)
            | MAPPED_KERN_RO_PHYSBASE!(get_nasid()))
    };
}

#[cfg(feature = "CONFIG_MAPPED_KERNEL")]
#[macro_export]
macro_rules! MAPPED_KERN_RW_TO_PHYS {
    ($x:expr) => {
        ((MAPPED_ADDR_RW_TO_PHYS!($x) as ::core::primitive::usize)
            | MAPPED_KERN_RW_PHYSBASE!(get_nasid()))
    };
}

// CONFIG_MAPPED_KERNEL is a build-time condition; these definitions apply
// when that configuration is disabled.
#[cfg(not(feature = "CONFIG_MAPPED_KERNEL"))]
#[macro_export]
macro_rules! MAPPED_KERN_RO_TO_PHYS {
    ($x:expr) => { ($x - $crate::REP_BASE!()) };
}

#[cfg(not(feature = "CONFIG_MAPPED_KERNEL"))]
#[macro_export]
macro_rules! MAPPED_KERN_RW_TO_PHYS {
    ($x:expr) => { ($x - $crate::REP_BASE!()) };
}

#[macro_export]
macro_rules! MAPPED_KERN_RO_TO_K0 {
    ($x:expr) => { PHYS_TO_K0!(MAPPED_KERN_RO_TO_PHYS!($x)) };
}

#[macro_export]
macro_rules! MAPPED_KERN_RW_TO_K0 {
    ($x:expr) => { PHYS_TO_K0!(MAPPED_KERN_RW_TO_PHYS!($x)) };
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
