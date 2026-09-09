/* SPDX-License-Identifier: GPL-2.0 */
/****************************************************************************/
/*
 * mcf8390.h -- NS8390 support for ColdFire eval boards.
 *
 * This is a source-level Rust translation of the original C header.
 */
/****************************************************************************/

/* Support for NE2000 clone devices in ColdFire based boards. */
pub const fn bswap(w: u32) -> u32 {
    (w << 8) | (w >> 8)
}

pub const fn rswap(w: u32) -> u32 {
    w
}

/* Basic hardware resources of NE2000 boards. */

#[cfg(feature = "CONFIG_ARN5206")]
pub const NE2000_ADDR: usize = 0x40000300;
#[cfg(feature = "CONFIG_ARN5206")]
pub const NE2000_ODDOFFSET: usize = 0x00010000;
#[cfg(feature = "CONFIG_ARN5206")]
pub const NE2000_ADDRSIZE: usize = 0x00020000;
#[cfg(feature = "CONFIG_ARN5206")]
pub const NE2000_IRQ_VECTOR: u32 = 0xf0;
#[cfg(feature = "CONFIG_ARN5206")]
pub const NE2000_IRQ_PRIORITY: u32 = 2;
#[cfg(feature = "CONFIG_ARN5206")]
pub const NE2000_IRQ_LEVEL: u32 = 4;
#[cfg(feature = "CONFIG_ARN5206")]
pub type NE2000_BYTE = u16; // volatile unsigned short

#[cfg(feature = "CONFIG_M5206eC3")]
pub const NE2000_ADDR: usize = 0x40000300;
#[cfg(feature = "CONFIG_M5206eC3")]
pub const NE2000_ODDOFFSET: usize = 0x00010000;
#[cfg(feature = "CONFIG_M5206eC3")]
pub const NE2000_ADDRSIZE: usize = 0x00020000;
#[cfg(feature = "CONFIG_M5206eC3")]
pub const NE2000_IRQ_VECTOR: u32 = 0x1c;
#[cfg(feature = "CONFIG_M5206eC3")]
pub const NE2000_IRQ_PRIORITY: u32 = 2;
#[cfg(feature = "CONFIG_M5206eC3")]
pub const NE2000_IRQ_LEVEL: u32 = 4;
#[cfg(feature = "CONFIG_M5206eC3")]
pub type NE2000_BYTE = u16; // volatile unsigned short

#[cfg(all(feature = "CONFIG_M5206e", feature = "CONFIG_NETtel"))]
pub const NE2000_ADDR: usize = 0x30000300;
#[cfg(all(feature = "CONFIG_M5206e", feature = "CONFIG_NETtel"))]
pub const NE2000_ADDRSIZE: usize = 0x00001000;
#[cfg(all(feature = "CONFIG_M5206e", feature = "CONFIG_NETtel"))]
pub const NE2000_IRQ_VECTOR: u32 = 25;
#[cfg(all(feature = "CONFIG_M5206e", feature = "CONFIG_NETtel"))]
pub const NE2000_IRQ_PRIORITY: u32 = 1;
#[cfg(all(feature = "CONFIG_M5206e", feature = "CONFIG_NETtel"))]
pub const NE2000_IRQ_LEVEL: u32 = 3;
#[cfg(all(feature = "CONFIG_M5206e", feature = "CONFIG_NETtel"))]
pub type NE2000_BYTE = u8; // volatile unsigned char

#[cfg(feature = "CONFIG_M5307C3")]
pub const NE2000_ADDR: usize = 0x40000300;
#[cfg(feature = "CONFIG_M5307C3")]
pub const NE2000_ODDOFFSET: usize = 0x00010000;
#[cfg(feature = "CONFIG_M5307C3")]
pub const NE2000_ADDRSIZE: usize = 0x00020000;
#[cfg(feature = "CONFIG_M5307C3")]
pub const NE2000_IRQ_VECTOR: u32 = 0x1b;
#[cfg(feature = "CONFIG_M5307C3")]
pub type NE2000_BYTE = u16; // volatile unsigned short

#[cfg(all(feature = "CONFIG_M5272", feature = "CONFIG_NETtel"))]
pub const NE2000_ADDR: usize = 0x30600300;
#[cfg(all(feature = "CONFIG_M5272", feature = "CONFIG_NETtel"))]
pub const NE2000_ODDOFFSET: usize = 0x00008000;
#[cfg(all(feature = "CONFIG_M5272", feature = "CONFIG_NETtel"))]
pub const NE2000_ADDRSIZE: usize = 0x00010000;
#[cfg(all(feature = "CONFIG_M5272", feature = "CONFIG_NETtel"))]
pub const NE2000_IRQ_VECTOR: u32 = 67;
#[cfg(all(feature = "CONFIG_M5272", feature = "CONFIG_NETtel"))]
pub type NE2000_BYTE = u16; // volatile unsigned short
#[cfg(all(feature = "CONFIG_M5272", feature = "CONFIG_NETtel"))]
pub const fn bswap_m5272(w: u32) -> u32 { w }
#[cfg(all(feature = "CONFIG_M5272", feature = "CONFIG_NETtel"))]
pub const fn rswap_m5272(w: u32) -> u32 { (w << 8) | (w >> 8) }

#[cfg(all(feature = "CONFIG_M5307", feature = "CONFIG_NETtel"))]
pub const NE2000_ADDR0: usize = 0x30600300;
#[cfg(all(feature = "CONFIG_M5307", feature = "CONFIG_NETtel"))]
pub const NE2000_ADDR1: usize = 0x30800300;
#[cfg(all(feature = "CONFIG_M5307", feature = "CONFIG_NETtel"))]
pub const NE2000_ODDOFFSET: usize = 0x00008000;
#[cfg(all(feature = "CONFIG_M5307", feature = "CONFIG_NETtel"))]
pub const NE2000_ADDRSIZE: usize = 0x00010000;
#[cfg(all(feature = "CONFIG_M5307", feature = "CONFIG_NETtel"))]
pub const NE2000_IRQ_VECTOR0: u32 = 27;
#[cfg(all(feature = "CONFIG_M5307", feature = "CONFIG_NETtel"))]
pub const NE2000_IRQ_VECTOR1: u32 = 29;
#[cfg(all(feature = "CONFIG_M5307", feature = "CONFIG_NETtel"))]
pub type NE2000_BYTE = u16; // volatile unsigned short
#[cfg(all(feature = "CONFIG_M5307", feature = "CONFIG_NETtel"))]
pub const fn bswap_m5307(w: u32) -> u32 { w }
#[cfg(all(feature = "CONFIG_M5307", feature = "CONFIG_NETtel"))]
pub const fn rswap_m5307(w: u32) -> u32 { (w << 8) | (w >> 8) }

#[cfg(all(feature = "CONFIG_M5307", feature = "CONFIG_SECUREEDGEMP3"))]
pub const NE2000_ADDR: usize = 0x30600300;
#[cfg(all(feature = "CONFIG_M5307", feature = "CONFIG_SECUREEDGEMP3"))]
pub const NE2000_ODDOFFSET: usize = 0x00008000;
#[cfg(all(feature = "CONFIG_M5307", feature = "CONFIG_SECUREEDGEMP3"))]
pub const NE2000_ADDRSIZE: usize = 0x00010000;
#[cfg(all(feature = "CONFIG_M5307", feature = "CONFIG_SECUREEDGEMP3"))]
pub const NE2000_IRQ_VECTOR: u32 = 27;
#[cfg(all(feature = "CONFIG_M5307", feature = "CONFIG_SECUREEDGEMP3"))]
pub type NE2000_BYTE = u16; // volatile unsigned short
#[cfg(all(feature = "CONFIG_M5307", feature = "CONFIG_SECUREEDGEMP3"))]
pub const fn bswap_m5307_secureedge(w: u32) -> u32 { w }
#[cfg(all(feature = "CONFIG_M5307", feature = "CONFIG_SECUREEDGEMP3"))]
pub const fn rswap_m5307_secureedge(w: u32) -> u32 { (w << 8) | (w >> 8) }

#[cfg(feature = "CONFIG_ARN5307")]
pub const NE2000_ADDR: usize = 0xfe600300;
#[cfg(feature = "CONFIG_ARN5307")]
pub const NE2000_ODDOFFSET: usize = 0x00010000;
#[cfg(feature = "CONFIG_ARN5307")]
pub const NE2000_ADDRSIZE: usize = 0x00020000;
#[cfg(feature = "CONFIG_ARN5307")]
pub const NE2000_IRQ_VECTOR: u32 = 0x1b;
#[cfg(feature = "CONFIG_ARN5307")]
pub const NE2000_IRQ_PRIORITY: u32 = 2;
#[cfg(feature = "CONFIG_ARN5307")]
pub const NE2000_IRQ_LEVEL: u32 = 3;
#[cfg(feature = "CONFIG_ARN5307")]
pub type NE2000_BYTE = u16; // volatile unsigned short

#[cfg(feature = "CONFIG_M5407C3")]
pub const NE2000_ADDR: usize = 0x40000300;
#[cfg(feature = "CONFIG_M5407C3")]
pub const NE2000_ODDOFFSET: usize = 0x00010000;
#[cfg(feature = "CONFIG_M5407C3")]
pub const NE2000_ADDRSIZE: usize = 0x00020000;
#[cfg(feature = "CONFIG_M5407C3")]
pub const NE2000_IRQ_VECTOR: u32 = 0x1b;
#[cfg(feature = "CONFIG_M5407C3")]
pub type NE2000_BYTE = u16; // volatile unsigned short

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
