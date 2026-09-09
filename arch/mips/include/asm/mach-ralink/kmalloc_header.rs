/* SPDX-License-Identifier: GPL-2.0 */

// The C header defines this only when CONFIG_DMA_NONCOHERENT is enabled.
#[cfg(CONFIG_DMA_NONCOHERENT)]
pub const ARCH_DMA_MINALIGN: usize = L1_CACHE_BYTES;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
