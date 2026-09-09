/* SPDX-License-Identifier: GPL-2.0 */

// Dependency provided by <asm/dma.h> in the C source.

// The C declaration is active when both CONFIG_PCI and CONFIG_X86_32 are
// enabled; otherwise the macro expands to the constant zero.
#[cfg(all(feature = "CONFIG_PCI", feature = "CONFIG_X86_32"))]
extern "C" {
    pub static mut isa_dma_bridge_buggy: i32;
}

#[cfg(not(all(feature = "CONFIG_PCI", feature = "CONFIG_X86_32")))]
pub const isa_dma_bridge_buggy: i32 = 0;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
