/* SPDX-License-Identifier: GPL-2.0 */

// Dependency supplied by the original header: linux/sh_intc.h.

// CONFIG_CPU_SUBTYPE_SH7720, CONFIG_CPU_SUBTYPE_SH7721,
// CONFIG_CPU_SUBTYPE_SH7710, and CONFIG_CPU_SUBTYPE_SH7712 select this base.
#[cfg(any(
    feature = "CONFIG_CPU_SUBTYPE_SH7720",
    feature = "CONFIG_CPU_SUBTYPE_SH7721",
    feature = "CONFIG_CPU_SUBTYPE_SH7710",
    feature = "CONFIG_CPU_SUBTYPE_SH7712",
))]
pub const SH_DMAC_BASE0: u32 = 0xa4010020;

// SH7705/06/07/09
#[cfg(not(any(
    feature = "CONFIG_CPU_SUBTYPE_SH7720",
    feature = "CONFIG_CPU_SUBTYPE_SH7721",
    feature = "CONFIG_CPU_SUBTYPE_SH7710",
    feature = "CONFIG_CPU_SUBTYPE_SH7712",
)))]
pub const SH_DMAC_BASE0: u32 = 0xa4000020;

// evt2irq is supplied by the translated interrupt-controller dependency.
pub const DMTE0_IRQ: u32 = evt2irq!(0x800);
pub const DMTE4_IRQ: u32 = evt2irq!(0xb80);

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
