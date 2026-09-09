/*
 * Xtensa MX interrupt distributor
 *
 * This file is subject to the terms and conditions of the GNU General Public
 * License.  See the file "COPYING" in the main directory of this archive
 * for more details.
 *
 * Copyright (C) 2008 - 2013 Tensilica Inc.
 */

/*
 * RER/WER at, as  Read/write external register
 *     at: value
 *     as: address
 *
 * Address Value
 * 00nn    0...0p..p  Interrupt Routing, route IRQ n to processor p
 * 01pp    0...0d..d  16 bits (d) 'ored' as single IPI to processor p
 * 0180    0...0m..m  Clear enable specified by mask (m)
 * 0184    0...0m..m  Set enable specified by mask (m)
 * 0190    0...0x..x  8-bit IPI partition register
 *             VVVVVVVVPPPPUUUUUUUUUUUUUUUUU
 *             V (10-bit) Release/Version
 *             P ( 4-bit) Number of cores - 1
 *             U (18-bit) ID
 * 01a0    i.......i  32-bit ConfigID
 * 0200    0...0m..m  RunStall core 'n'
 * 0220    c          Cache coherency enabled
 */

pub const fn mirout(irq: u32) -> u32 {
    0x000u32.wrapping_add(irq)
}

pub const fn mipicause(cpu: u32) -> u32 {
    0x100u32.wrapping_add(cpu)
}

pub const fn mipiset(cause: u32) -> u32 {
    0x140u32.wrapping_add(cause)
}

pub const MIENG: u32 = 0x180;
pub const MIENGSET: u32 = 0x184;
pub const MIASG: u32 = 0x188; // Read Global Assert Register
pub const MIASGSET: u32 = 0x18c; // Set Global Addert Regiter
pub const MIPIPART: u32 = 0x190;
pub const SYSCFGID: u32 = 0x1a0;
pub const MPSCORE: u32 = 0x200;
pub const CCON: u32 = 0x220;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
