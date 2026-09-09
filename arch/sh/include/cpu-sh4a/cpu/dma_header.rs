/* SPDX-License-Identifier: GPL-2.0 */

// Dependency supplied by the surrounding SH interrupt-controller bindings:
// `evt2irq`.
//
// The following configuration branches preserve the C preprocessor
// conditions from the original header as Rust `cfg` conditions.

#[cfg(any(feature = "CONFIG_CPU_SUBTYPE_SH7343", feature = "CONFIG_CPU_SUBTYPE_SH7730"))]
pub const DMTE0_IRQ: _ = evt2irq(0x800);
#[cfg(any(feature = "CONFIG_CPU_SUBTYPE_SH7343", feature = "CONFIG_CPU_SUBTYPE_SH7730"))]
pub const DMTE4_IRQ: _ = evt2irq(0xb80);
#[cfg(any(feature = "CONFIG_CPU_SUBTYPE_SH7343", feature = "CONFIG_CPU_SUBTYPE_SH7730"))]
pub const DMAE0_IRQ: _ = evt2irq(0xbc0); /* DMA Error IRQ */
#[cfg(any(feature = "CONFIG_CPU_SUBTYPE_SH7343", feature = "CONFIG_CPU_SUBTYPE_SH7730"))]
pub const SH_DMAC_BASE0: usize = 0xFE008020;

#[cfg(feature = "CONFIG_CPU_SUBTYPE_SH7722")]
pub const DMTE0_IRQ: _ = evt2irq(0x800);
#[cfg(feature = "CONFIG_CPU_SUBTYPE_SH7722")]
pub const DMTE4_IRQ: _ = evt2irq(0xb80);
#[cfg(feature = "CONFIG_CPU_SUBTYPE_SH7722")]
pub const DMAE0_IRQ: _ = evt2irq(0xbc0); /* DMA Error IRQ */
#[cfg(feature = "CONFIG_CPU_SUBTYPE_SH7722")]
pub const SH_DMAC_BASE0: usize = 0xFE008020;

#[cfg(feature = "CONFIG_CPU_SUBTYPE_SH7763")]
pub const DMTE0_IRQ: _ = evt2irq(0x640);
#[cfg(feature = "CONFIG_CPU_SUBTYPE_SH7763")]
pub const DMTE4_IRQ: _ = evt2irq(0x780);
#[cfg(feature = "CONFIG_CPU_SUBTYPE_SH7763")]
pub const DMAE0_IRQ: _ = evt2irq(0x6c0);
#[cfg(feature = "CONFIG_CPU_SUBTYPE_SH7763")]
pub const SH_DMAC_BASE0: usize = 0xFF608020;

#[cfg(any(feature = "CONFIG_CPU_SUBTYPE_SH7723", feature = "CONFIG_CPU_SUBTYPE_SH7724"))]
pub const DMTE0_IRQ: _ = evt2irq(0x800); /* DMAC0A */
#[cfg(any(feature = "CONFIG_CPU_SUBTYPE_SH7723", feature = "CONFIG_CPU_SUBTYPE_SH7724"))]
pub const DMTE4_IRQ: _ = evt2irq(0xb80); /* DMAC0B */
#[cfg(any(feature = "CONFIG_CPU_SUBTYPE_SH7723", feature = "CONFIG_CPU_SUBTYPE_SH7724"))]
pub const DMTE6_IRQ: _ = evt2irq(0x700);
#[cfg(any(feature = "CONFIG_CPU_SUBTYPE_SH7723", feature = "CONFIG_CPU_SUBTYPE_SH7724"))]
pub const DMTE8_IRQ: _ = evt2irq(0x740); /* DMAC1A */
#[cfg(any(feature = "CONFIG_CPU_SUBTYPE_SH7723", feature = "CONFIG_CPU_SUBTYPE_SH7724"))]
pub const DMTE9_IRQ: _ = evt2irq(0x760);
#[cfg(any(feature = "CONFIG_CPU_SUBTYPE_SH7723", feature = "CONFIG_CPU_SUBTYPE_SH7724"))]
pub const DMTE10_IRQ: _ = evt2irq(0xb00); /* DMAC1B */
#[cfg(any(feature = "CONFIG_CPU_SUBTYPE_SH7723", feature = "CONFIG_CPU_SUBTYPE_SH7724"))]
pub const DMTE11_IRQ: _ = evt2irq(0xb20);
#[cfg(any(feature = "CONFIG_CPU_SUBTYPE_SH7723", feature = "CONFIG_CPU_SUBTYPE_SH7724"))]
pub const DMAE0_IRQ: _ = evt2irq(0xbc0); /* DMA Error IRQ */
#[cfg(any(feature = "CONFIG_CPU_SUBTYPE_SH7723", feature = "CONFIG_CPU_SUBTYPE_SH7724"))]
pub const DMAE1_IRQ: _ = evt2irq(0xb40); /* DMA Error IRQ */
#[cfg(any(feature = "CONFIG_CPU_SUBTYPE_SH7723", feature = "CONFIG_CPU_SUBTYPE_SH7724"))]
pub const SH_DMAC_BASE0: usize = 0xFE008020;
#[cfg(any(feature = "CONFIG_CPU_SUBTYPE_SH7723", feature = "CONFIG_CPU_SUBTYPE_SH7724"))]
pub const SH_DMAC_BASE1: usize = 0xFDC08020;

#[cfg(feature = "CONFIG_CPU_SUBTYPE_SH7780")]
pub const DMTE0_IRQ: _ = evt2irq(0x640);
#[cfg(feature = "CONFIG_CPU_SUBTYPE_SH7780")]
pub const DMTE4_IRQ: _ = evt2irq(0x780);
#[cfg(feature = "CONFIG_CPU_SUBTYPE_SH7780")]
pub const DMTE6_IRQ: _ = evt2irq(0x7c0);
#[cfg(feature = "CONFIG_CPU_SUBTYPE_SH7780")]
pub const DMTE8_IRQ: _ = evt2irq(0xd80);
#[cfg(feature = "CONFIG_CPU_SUBTYPE_SH7780")]
pub const DMTE9_IRQ: _ = evt2irq(0xda0);
#[cfg(feature = "CONFIG_CPU_SUBTYPE_SH7780")]
pub const DMTE10_IRQ: _ = evt2irq(0xdc0);
#[cfg(feature = "CONFIG_CPU_SUBTYPE_SH7780")]
pub const DMTE11_IRQ: _ = evt2irq(0xde0);
#[cfg(feature = "CONFIG_CPU_SUBTYPE_SH7780")]
pub const DMAE0_IRQ: _ = evt2irq(0x6c0); /* DMA Error IRQ */
#[cfg(feature = "CONFIG_CPU_SUBTYPE_SH7780")]
pub const SH_DMAC_BASE0: usize = 0xFC808020;
#[cfg(feature = "CONFIG_CPU_SUBTYPE_SH7780")]
pub const SH_DMAC_BASE1: usize = 0xFC818020;

// C `#else` branch: SH7785.
#[cfg(not(any(
    feature = "CONFIG_CPU_SUBTYPE_SH7343",
    feature = "CONFIG_CPU_SUBTYPE_SH7730",
    feature = "CONFIG_CPU_SUBTYPE_SH7722",
    feature = "CONFIG_CPU_SUBTYPE_SH7763",
    feature = "CONFIG_CPU_SUBTYPE_SH7723",
    feature = "CONFIG_CPU_SUBTYPE_SH7724",
    feature = "CONFIG_CPU_SUBTYPE_SH7780",
)))]
pub const DMTE0_IRQ: _ = evt2irq(0x620);
#[cfg(not(any(
    feature = "CONFIG_CPU_SUBTYPE_SH7343", feature = "CONFIG_CPU_SUBTYPE_SH7730",
    feature = "CONFIG_CPU_SUBTYPE_SH7722", feature = "CONFIG_CPU_SUBTYPE_SH7763",
    feature = "CONFIG_CPU_SUBTYPE_SH7723", feature = "CONFIG_CPU_SUBTYPE_SH7724",
    feature = "CONFIG_CPU_SUBTYPE_SH7780",
)))]
pub const DMTE4_IRQ: _ = evt2irq(0x6a0);
#[cfg(not(any(
    feature = "CONFIG_CPU_SUBTYPE_SH7343", feature = "CONFIG_CPU_SUBTYPE_SH7730",
    feature = "CONFIG_CPU_SUBTYPE_SH7722", feature = "CONFIG_CPU_SUBTYPE_SH7763",
    feature = "CONFIG_CPU_SUBTYPE_SH7723", feature = "CONFIG_CPU_SUBTYPE_SH7724",
    feature = "CONFIG_CPU_SUBTYPE_SH7780",
)))]
pub const DMTE6_IRQ: _ = evt2irq(0x880);
#[cfg(not(any(
    feature = "CONFIG_CPU_SUBTYPE_SH7343", feature = "CONFIG_CPU_SUBTYPE_SH7730",
    feature = "CONFIG_CPU_SUBTYPE_SH7722", feature = "CONFIG_CPU_SUBTYPE_SH7763",
    feature = "CONFIG_CPU_SUBTYPE_SH7723", feature = "CONFIG_CPU_SUBTYPE_SH7724",
    feature = "CONFIG_CPU_SUBTYPE_SH7780",
)))]
pub const DMTE8_IRQ: _ = evt2irq(0x8c0);
#[cfg(not(any(
    feature = "CONFIG_CPU_SUBTYPE_SH7343", feature = "CONFIG_CPU_SUBTYPE_SH7730",
    feature = "CONFIG_CPU_SUBTYPE_SH7722", feature = "CONFIG_CPU_SUBTYPE_SH7763",
    feature = "CONFIG_CPU_SUBTYPE_SH7723", feature = "CONFIG_CPU_SUBTYPE_SH7724",
    feature = "CONFIG_CPU_SUBTYPE_SH7780",
)))]
pub const DMTE9_IRQ: _ = evt2irq(0x8e0);
#[cfg(not(any(
    feature = "CONFIG_CPU_SUBTYPE_SH7343", feature = "CONFIG_CPU_SUBTYPE_SH7730",
    feature = "CONFIG_CPU_SUBTYPE_SH7722", feature = "CONFIG_CPU_SUBTYPE_SH7763",
    feature = "CONFIG_CPU_SUBTYPE_SH7723", feature = "CONFIG_CPU_SUBTYPE_SH7724",
    feature = "CONFIG_CPU_SUBTYPE_SH7780",
)))]
pub const DMTE10_IRQ: _ = evt2irq(0x900);
#[cfg(not(any(
    feature = "CONFIG_CPU_SUBTYPE_SH7343", feature = "CONFIG_CPU_SUBTYPE_SH7730",
    feature = "CONFIG_CPU_SUBTYPE_SH7722", feature = "CONFIG_CPU_SUBTYPE_SH7763",
    feature = "CONFIG_CPU_SUBTYPE_SH7723", feature = "CONFIG_CPU_SUBTYPE_SH7724",
    feature = "CONFIG_CPU_SUBTYPE_SH7780",
)))]
pub const DMTE11_IRQ: _ = evt2irq(0x920);
#[cfg(not(any(
    feature = "CONFIG_CPU_SUBTYPE_SH7343", feature = "CONFIG_CPU_SUBTYPE_SH7730",
    feature = "CONFIG_CPU_SUBTYPE_SH7722", feature = "CONFIG_CPU_SUBTYPE_SH7763",
    feature = "CONFIG_CPU_SUBTYPE_SH7723", feature = "CONFIG_CPU_SUBTYPE_SH7724",
    feature = "CONFIG_CPU_SUBTYPE_SH7780",
)))]
pub const DMAE0_IRQ: _ = evt2irq(0x6e0); /* DMA Error IRQ0 */
#[cfg(not(any(
    feature = "CONFIG_CPU_SUBTYPE_SH7343", feature = "CONFIG_CPU_SUBTYPE_SH7730",
    feature = "CONFIG_CPU_SUBTYPE_SH7722", feature = "CONFIG_CPU_SUBTYPE_SH7763",
    feature = "CONFIG_CPU_SUBTYPE_SH7723", feature = "CONFIG_CPU_SUBTYPE_SH7724",
    feature = "CONFIG_CPU_SUBTYPE_SH7780",
)))]
pub const DMAE1_IRQ: _ = evt2irq(0x940); /* DMA Error IRQ1 */
#[cfg(not(any(
    feature = "CONFIG_CPU_SUBTYPE_SH7343", feature = "CONFIG_CPU_SUBTYPE_SH7730",
    feature = "CONFIG_CPU_SUBTYPE_SH7722", feature = "CONFIG_CPU_SUBTYPE_SH7763",
    feature = "CONFIG_CPU_SUBTYPE_SH7723", feature = "CONFIG_CPU_SUBTYPE_SH7724",
    feature = "CONFIG_CPU_SUBTYPE_SH7780",
)))]
pub const SH_DMAC_BASE0: usize = 0xFC808020;
#[cfg(not(any(
    feature = "CONFIG_CPU_SUBTYPE_SH7343", feature = "CONFIG_CPU_SUBTYPE_SH7730",
    feature = "CONFIG_CPU_SUBTYPE_SH7722", feature = "CONFIG_CPU_SUBTYPE_SH7763",
    feature = "CONFIG_CPU_SUBTYPE_SH7723", feature = "CONFIG_CPU_SUBTYPE_SH7724",
    feature = "CONFIG_CPU_SUBTYPE_SH7780",
)))]
pub const SH_DMAC_BASE1: usize = 0xFCC08020;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
