/* SPDX-License-Identifier: GPL-2.0
 *
 * Copyright (C) 1999 by Kaz Kojima
 *
 * Definitions for the address spaces of the SH CPUs.
 */

// Dependency supplied by the CPU-specific address-space definitions.

/* If this CPU supports segmentation, hook up the helpers. */
// The following items are conditional on the C build-time definition P1SEG.

/*
   [ P0/U0 (virtual) ]          0x00000000     <------ User space
   [ P1 (fixed)   cached ]      0x80000000     <------ Kernel space
   [ P2 (fixed)  non-cachable]  0xA0000000     <------ Physical access
   [ P3 (virtual) cached]       0xC0000000     <------ vmalloced area
   [ P4 control   ]             0xE0000000
 */

/// Returns the privileged segment base of a given address.
#[macro_export]
macro_rules! PXSEG {
    ($a:expr) => {
        (($a as usize) & 0xe0000000usize)
    };
}

// Under CONFIG_29BIT, map an address to a certain privileged segment.
// P1SEG, P2SEG, P3SEG, and P4SEG are supplied by the CPU address-space
// definitions.
#[cfg(feature = "CONFIG_29BIT")]
#[macro_export]
macro_rules! P1SEGADDR {
    ($a:expr) => {
        (($a as usize & 0x1fffffffusize) | P1SEG)
    };
}
#[cfg(feature = "CONFIG_29BIT")]
#[macro_export]
macro_rules! P2SEGADDR {
    ($a:expr) => {
        (($a as usize & 0x1fffffffusize) | P2SEG)
    };
}
#[cfg(feature = "CONFIG_29BIT")]
#[macro_export]
macro_rules! P3SEGADDR {
    ($a:expr) => {
        (($a as usize & 0x1fffffffusize) | P3SEG)
    };
}
#[cfg(feature = "CONFIG_29BIT")]
#[macro_export]
macro_rules! P4SEGADDR {
    ($a:expr) => {
        (($a as usize & 0x1fffffffusize) | P4SEG)
    };
}

// These will never work in 32-bit, don't even bother.
#[cfg(not(feature = "CONFIG_29BIT"))]
#[macro_export]
macro_rules! P1SEGADDR {
    ($a:expr) => {{
        let _ = &$a;
        BUG!();
        core::ptr::null_mut()
    }};
}
#[cfg(not(feature = "CONFIG_29BIT"))]
#[macro_export]
macro_rules! P2SEGADDR {
    ($a:expr) => {{
        let _ = &$a;
        BUG!();
        core::ptr::null_mut()
    }};
}
#[cfg(not(feature = "CONFIG_29BIT"))]
#[macro_export]
macro_rules! P3SEGADDR {
    ($a:expr) => {{
        let _ = &$a;
        BUG!();
        core::ptr::null_mut()
    }};
}
#[cfg(not(feature = "CONFIG_29BIT"))]
#[macro_export]
macro_rules! P4SEGADDR {
    ($a:expr) => {{
        let _ = &$a;
        BUG!();
        core::ptr::null_mut()
    }};
}

/// Check if an address can be reached in 29 bits.
#[macro_export]
macro_rules! IS_29BIT {
    ($a:expr) => {
        (($a as usize) < 0x20000000usize)
    };
}

// P3_ADDR_MAX depends on the C build-time definition CONFIG_SH_STORE_QUEUES.
#[cfg(feature = "CONFIG_SH_STORE_QUEUES")]
pub const P3_ADDR_MAX: usize = P4SEG_STORE_QUE + 0x04000000usize;

#[cfg(not(feature = "CONFIG_SH_STORE_QUEUES"))]
pub const P3_ADDR_MAX: usize = P4SEG;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
