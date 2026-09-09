/* SPDX-License-Identifier: GPL-2.0 */

pub const PA_SMSC: u32 = 0x3000_0000;
pub const PA_MRSHPC: u32 = 0x3400_0000;
pub const PA_LED: u32 = 0x3140_0000;

unsafe extern "C" {
    pub fn init_se7206_IRQ();
}

// __IO_PREFIX se7206
// Dependency: <asm/io_generic.h> supplies the generic I/O declarations.

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
