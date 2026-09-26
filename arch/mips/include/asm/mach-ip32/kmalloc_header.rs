/* SPDX-License-Identifier: GPL-2.0 */

// ARCH_DMA_MINALIGN is 32 when CONFIG_CPU_R5000 or CONFIG_CPU_RM7000 is
// enabled; otherwise it is 128.
#[cfg(any(CONFIG_CPU_R5000, CONFIG_CPU_RM7000))]
pub const ARCH_DMA_MINALIGN: usize = 32;

#[cfg(not(any(CONFIG_CPU_R5000, CONFIG_CPU_RM7000)))]
pub const ARCH_DMA_MINALIGN: usize = 128;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
