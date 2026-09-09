/* SPDX-License-Identifier: GPL-2.0 */

/*
 * linux/include/alpha/irq.h
 *
 * (C) 1994 Linus Torvalds
 *
 * The original header includes linux/linkage.h; its declarations are
 * supplied by the surrounding translation unit.
 */

/*
 * For CONFIG_ALPHA_GENERIC, NR_IRQS is an upper bound rather than an exact
 * value.  alpha_mv.nr_irqs supplies the real value where needed.
 *
 * When CONFIG_ALPHA_LEGACY_START_ADDRESS is selected, TITAN, WILDFIRE, and
 * MARVEL are left out to keep the kernel object size reasonable.
 *
 * These cfg conditions preserve the original build-time configuration.
 */
#[cfg(all(feature = "CONFIG_ALPHA_GENERIC", feature = "CONFIG_ALPHA_LEGACY_START_ADDRESS"))]
pub const NR_IRQS: i32 = 128; /* max is RAWHIDE/TAKARA */

#[cfg(all(
    feature = "CONFIG_ALPHA_GENERIC",
    not(feature = "CONFIG_ALPHA_LEGACY_START_ADDRESS")
))]
pub const NR_IRQS: i32 = 32768 + 16; /* marvel - 32 pids */

#[cfg(any(
    feature = "CONFIG_ALPHA_PC164",
    feature = "CONFIG_ALPHA_LX164"
))]
pub const NR_IRQS: i32 = 35;

#[cfg(feature = "CONFIG_ALPHA_MIKASA")]
pub const NR_IRQS: i32 = 32;

#[cfg(any(
    feature = "CONFIG_ALPHA_ALCOR",
    feature = "CONFIG_ALPHA_MIATA",
    feature = "CONFIG_ALPHA_RUFFIAN",
    feature = "CONFIG_ALPHA_RX164",
    feature = "CONFIG_ALPHA_NORITAKE"
))]
pub const NR_IRQS: i32 = 48;

#[cfg(any(feature = "CONFIG_ALPHA_SABLE", feature = "CONFIG_ALPHA_SX164"))]
pub const NR_IRQS: i32 = 40;

#[cfg(any(feature = "CONFIG_ALPHA_DP264", feature = "CONFIG_ALPHA_SHARK"))]
pub const NR_IRQS: i32 = 64;

#[cfg(feature = "CONFIG_ALPHA_TITAN")]
pub const NR_IRQS: i32 = 80;

#[cfg(any(
    feature = "CONFIG_ALPHA_RAWHIDE",
    feature = "CONFIG_ALPHA_TAKARA",
    feature = "CONFIG_ALPHA_EIGER"
))]
pub const NR_IRQS: i32 = 128;

#[cfg(feature = "CONFIG_ALPHA_WILDFIRE")]
pub const NR_IRQS: i32 = 2048; /* enuff for 8 QBBs */

#[cfg(feature = "CONFIG_ALPHA_MARVEL")]
pub const NR_IRQS: i32 = 32768 + 16; /* marvel - 32 pids */

#[cfg(not(any(
    feature = "CONFIG_ALPHA_GENERIC",
    feature = "CONFIG_ALPHA_PC164",
    feature = "CONFIG_ALPHA_LX164",
    feature = "CONFIG_ALPHA_MIKASA",
    feature = "CONFIG_ALPHA_ALCOR",
    feature = "CONFIG_ALPHA_MIATA",
    feature = "CONFIG_ALPHA_RUFFIAN",
    feature = "CONFIG_ALPHA_RX164",
    feature = "CONFIG_ALPHA_NORITAKE",
    feature = "CONFIG_ALPHA_SABLE",
    feature = "CONFIG_ALPHA_SX164",
    feature = "CONFIG_ALPHA_DP264",
    feature = "CONFIG_ALPHA_SHARK",
    feature = "CONFIG_ALPHA_TITAN",
    feature = "CONFIG_ALPHA_RAWHIDE",
    feature = "CONFIG_ALPHA_TAKARA",
    feature = "CONFIG_ALPHA_EIGER",
    feature = "CONFIG_ALPHA_WILDFIRE",
    feature = "CONFIG_ALPHA_MARVEL"
))]
pub const NR_IRQS: i32 = 16;

#[inline]
pub const fn irq_canonicalize(irq: i32) -> i32 {
    /*
     * XXX is this true for all Alpha's?  The old serial driver
     * did it this way for years without any complaints, so....
     */
    if irq == 2 { 9 } else { irq }
}

pub struct pt_regs;

pub static mut perf_irq: Option<unsafe extern "C" fn(u64, *mut pt_regs)> = None;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
