/* SPDX-License-Identifier: GPL-2.0 */

// Header guard from the C source: __POWERPC_PLATFORMS_44X_44X_H

extern "C" {
    pub fn as1_readb(addr: *mut u8) -> u8;
    pub fn as1_writeb(data: u8, addr: *mut u8);
}

pub const GPIO0_OSRH: u32 = 0xC;
pub const GPIO0_TSRH: u32 = 0x14;
pub const GPIO0_ISR1H: u32 = 0x34;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
