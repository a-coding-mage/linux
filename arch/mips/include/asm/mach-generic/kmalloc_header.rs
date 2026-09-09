/* SPDX-License-Identifier: GPL-2.0 */

// CONFIG_DMA_NONCOHERENT
//
// Total overkill for most systems but need as a safe default.
// Set this one if any device in the system might do non-coherent DMA.
#[cfg(feature = "CONFIG_DMA_NONCOHERENT")]
pub const ARCH_DMA_MINALIGN: usize = 128;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
