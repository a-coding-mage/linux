/* SPDX-License-Identifier: GPL-2.0 */
/* Translation of linux/include/asm-m68k/raw_io.h. */
/* The original declarations are kernel-only and depend on asm/byteorder.h. */

#[inline]
pub unsafe fn in_8(addr: usize) -> u8 { core::ptr::read_volatile(addr as *const u8) }
#[inline]
pub unsafe fn in_be16(addr: usize) -> u16 { core::ptr::read_volatile(addr as *const u16) }
#[inline]
pub unsafe fn in_be32(addr: usize) -> u32 { core::ptr::read_volatile(addr as *const u32) }
#[inline]
pub unsafe fn in_le16(addr: usize) -> u16 { u16::from_le(core::ptr::read_volatile(addr as *const u16)) }
#[inline]
pub unsafe fn in_le32(addr: usize) -> u32 { u32::from_le(core::ptr::read_volatile(addr as *const u32)) }

#[inline]
pub unsafe fn out_8(addr: usize, value: u8) { core::ptr::write_volatile(addr as *mut u8, value); }
#[inline]
pub unsafe fn out_be16(addr: usize, value: u16) { core::ptr::write_volatile(addr as *mut u16, value); }
#[inline]
pub unsafe fn out_be32(addr: usize, value: u32) { core::ptr::write_volatile(addr as *mut u32, value); }
#[inline]
pub unsafe fn out_le16(addr: usize, value: u16) { core::ptr::write_volatile(addr as *mut u16, value.to_le()); }
#[inline]
pub unsafe fn out_le32(addr: usize, value: u32) { core::ptr::write_volatile(addr as *mut u32, value.to_le()); }

pub use in_8 as raw_inb;
pub use in_be16 as raw_inw;
pub use in_be32 as raw_inl;
pub use in_8 as __raw_readb;
pub use in_be16 as __raw_readw;
pub use in_be32 as __raw_readl;

#[inline] pub unsafe fn raw_outb(value: u8, port: usize) { out_8(port, value) }
#[inline] pub unsafe fn raw_outw(value: u16, port: usize) { out_be16(port, value) }
#[inline] pub unsafe fn raw_outl(value: u32, port: usize) { out_be32(port, value) }
#[inline] pub unsafe fn __raw_writeb(value: u8, addr: usize) { out_8(addr, value) }
#[inline] pub unsafe fn __raw_writew(value: u16, addr: usize) { out_be16(addr, value) }
#[inline] pub unsafe fn __raw_writel(value: u32, addr: usize) { out_be32(addr, value) }

/* CONFIG_ATARI_ROM_ISA conditional declarations from the original header. */
#[cfg(feature = "CONFIG_ATARI_ROM_ISA")]
pub unsafe fn rom_in_8(addr: usize) -> u8 { (core::ptr::read_volatile(addr as *const u16) >> 8) as u8 }
#[cfg(feature = "CONFIG_ATARI_ROM_ISA")]
pub unsafe fn rom_in_be16(addr: usize) -> u16 { core::ptr::read_volatile(addr as *const u16) }
#[cfg(feature = "CONFIG_ATARI_ROM_ISA")]
pub unsafe fn rom_in_le16(addr: usize) -> u16 { u16::from_le(core::ptr::read_volatile(addr as *const u16)) }

#[cfg(feature = "CONFIG_ATARI_ROM_ISA")]
pub unsafe fn rom_out_8(addr: usize, value: u8) {
    let a = (addr as u32) | 0x10000;
    let _ = core::ptr::read_volatile((a.wrapping_add((value as u32) << 1)) as *const u8);
}
#[cfg(feature = "CONFIG_ATARI_ROM_ISA")]
pub unsafe fn rom_out_be16(addr: usize, value: u16) {
    let a = addr as u32;
    let _ = core::ptr::read_volatile(((a & 0xffff0000).wrapping_add(((value & 0xff) as u32) << 1)) as *const u16);
    let _ = core::ptr::read_volatile(((a | 0x10000).wrapping_add(((value >> 8) as u32) << 1)) as *const u16);
}
#[cfg(feature = "CONFIG_ATARI_ROM_ISA")]
pub unsafe fn rom_out_le16(addr: usize, value: u16) {
    let a = addr as u32;
    let _ = core::ptr::read_volatile(((a & 0xffff0000).wrapping_add(((value >> 8) as u32) << 1)) as *const u16);
    let _ = core::ptr::read_volatile(((a | 0x10000).wrapping_add(((value & 0xff) as u32) << 1)) as *const u16);
}

#[inline] pub unsafe fn raw_insb(port: *const u8, buf: *mut u8, len: usize) { for i in 0..len { *buf.add(i) = in_8(port as usize); } }
#[inline] pub unsafe fn raw_outsb(port: *mut u8, buf: *const u8, nr: usize) { for i in 0..nr { out_8(port as usize, *buf.add(i)); } }
#[inline] pub unsafe fn raw_insw(port: *const u16, buf: *mut u16, nr: usize) { for i in 0..nr { *buf.add(i) = in_be16(port as usize); } }
#[inline] pub unsafe fn raw_outsw(port: *mut u16, buf: *const u16, nr: usize) { for i in 0..nr { out_be16(port as usize, *buf.add(i)); } }
#[inline] pub unsafe fn raw_insl(port: *const u32, buf: *mut u32, nr: usize) { for i in 0..nr { *buf.add(i) = in_be32(port as usize); } }
#[inline] pub unsafe fn raw_outsl(port: *mut u32, buf: *const u32, nr: usize) { for i in 0..nr { out_be32(port as usize, *buf.add(i)); } }

#[inline] pub unsafe fn raw_insw_swapw(port: *const u16, buf: *mut u16, nr: usize) { for i in 0..nr { *buf.add(i) = in_be16(port as usize).rotate_left(8); } }
#[inline] pub unsafe fn raw_outsw_swapw(port: *mut u16, buf: *const u16, nr: usize) { for i in 0..nr { out_be16(port as usize, (*buf.add(i)).rotate_left(8)); } }

#[cfg(feature = "CONFIG_ATARI_ROM_ISA")]
#[inline] pub unsafe fn raw_rom_insb(port: *const u8, buf: *mut u8, len: usize) { for i in 0..len { *buf.add(i) = rom_in_8(port as usize); } }
#[cfg(feature = "CONFIG_ATARI_ROM_ISA")]
#[inline] pub unsafe fn raw_rom_outsb(port: *mut u8, buf: *const u8, len: usize) { for i in 0..len { rom_out_8(port as usize, *buf.add(i)); } }
#[cfg(feature = "CONFIG_ATARI_ROM_ISA")]
#[inline] pub unsafe fn raw_rom_insw(port: *const u16, buf: *mut u16, nr: usize) { for i in 0..nr { *buf.add(i) = rom_in_be16(port as usize); } }
#[cfg(feature = "CONFIG_ATARI_ROM_ISA")]
#[inline] pub unsafe fn raw_rom_outsw(port: *mut u16, buf: *const u16, nr: usize) { for i in 0..nr { rom_out_be16(port as usize, *buf.add(i)); } }
#[cfg(feature = "CONFIG_ATARI_ROM_ISA")]
#[inline] pub unsafe fn raw_rom_insw_swapw(port: *const u16, buf: *mut u16, nr: usize) { for i in 0..nr { *buf.add(i) = rom_in_le16(port as usize); } }
#[cfg(feature = "CONFIG_ATARI_ROM_ISA")]
#[inline] pub unsafe fn raw_rom_outsw_swapw(port: *mut u16, buf: *const u16, nr: usize) { for i in 0..nr { rom_out_le16(port as usize, *buf.add(i)); } }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
