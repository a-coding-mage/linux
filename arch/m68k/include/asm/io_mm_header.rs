/* SPDX-License-Identifier: GPL-2.0 */
/* Rust translation of linux/include/asm-m68k/io.h (header-only interfaces). */

/* C preprocessor configuration branches are retained as cfg-intent comments;
 * the symbols below are supplied by the surrounding kernel translation. */

#[allow(non_camel_case_types, non_snake_case, non_upper_case_globals)]
pub type u8 = core::primitive::u8;
#[allow(non_camel_case_types)]
pub type u16 = core::primitive::u16;
#[allow(non_camel_case_types)]
pub type u32 = core::primitive::u32;

#[cfg(feature = "CONFIG_Q40")]
pub const q40_isa_io_base: usize = 0xff400000;
#[cfg(feature = "CONFIG_Q40")]
pub const q40_isa_mem_base: usize = 0xff800000;

#[cfg(feature = "CONFIG_Q40")]
macro_rules! Q40_ISA_IO_B { ($ioaddr:expr) => { q40_isa_io_base + 1 + 4 * (($ioaddr) as usize) }; }
#[cfg(feature = "CONFIG_Q40")]
macro_rules! Q40_ISA_IO_W { ($ioaddr:expr) => { q40_isa_io_base + 4 * (($ioaddr) as usize) }; }
#[cfg(feature = "CONFIG_Q40")]
macro_rules! Q40_ISA_MEM_B { ($madr:expr) => { q40_isa_mem_base + 1 + 4 * (($madr) as usize) }; }
#[cfg(feature = "CONFIG_Q40")]
macro_rules! Q40_ISA_MEM_W { ($madr:expr) => { q40_isa_mem_base + 4 * (($madr) as usize) }; }

#[cfg(feature = "CONFIG_AMIGA_PCMCIA")]
macro_rules! AG_ISA_IO_B { ($ioaddr:expr) => { GAYLE_IO + ($ioaddr) + ((($ioaddr) & 1) * GAYLE_ODD) }; }
#[cfg(feature = "CONFIG_AMIGA_PCMCIA")]
macro_rules! AG_ISA_IO_W { ($ioaddr:expr) => { GAYLE_IO + ($ioaddr) }; }

#[cfg(feature = "CONFIG_ATARI_ROM_ISA")]
pub const enec_isa_read_base: usize = 0xfffa0000;
#[cfg(feature = "CONFIG_ATARI_ROM_ISA")]
pub const enec_isa_write_base: usize = 0xfffb0000;
#[cfg(feature = "CONFIG_ATARI_ROM_ISA")]
macro_rules! ENEC_ISA_IO_B { ($ioaddr:expr) => { enec_isa_read_base + (((($ioaddr) as usize) & 0x7f) << 9) }; }
#[cfg(feature = "CONFIG_ATARI_ROM_ISA")]
macro_rules! ENEC_ISA_IO_W { ($ioaddr:expr) => { enec_isa_read_base + (((($ioaddr) as usize) & 0x7f) << 9) }; }
#[cfg(feature = "CONFIG_ATARI_ROM_ISA")]
macro_rules! ENEC_ISA_MEM_B { ($madr:expr) => { enec_isa_read_base + (((($madr) as usize) & 0x7f) << 9) }; }
#[cfg(feature = "CONFIG_ATARI_ROM_ISA")]
macro_rules! ENEC_ISA_MEM_W { ($madr:expr) => { enec_isa_read_base + (((($madr) as usize) & 0x7f) << 9) }; }

pub const ISA_TYPE_Q40: i32 = 1;
pub const ISA_TYPE_AG: i32 = 2;
pub const ISA_TYPE_ENEC: i32 = 3;

#[cfg(feature = "MULTI_ISA")]
extern "C" {
    pub static mut isa_type: i32;
    pub static mut isa_sex: i32;
}

#[cfg(feature = "CONFIG_Q40")]
const ISA_TYPE: i32 = ISA_TYPE_Q40;
#[cfg(feature = "CONFIG_AMIGA_PCMCIA")]
const ISA_TYPE: i32 = ISA_TYPE_AG;
#[cfg(feature = "CONFIG_ATARI_ROM_ISA")]
const ISA_TYPE: i32 = ISA_TYPE_ENEC;

#[inline]
pub unsafe fn isa_itb(addr: usize) -> *mut u8 {
    match ISA_TYPE {
        #[cfg(feature = "CONFIG_Q40")] ISA_TYPE_Q40 => Q40_ISA_IO_B!(addr) as *mut u8,
        #[cfg(feature = "CONFIG_AMIGA_PCMCIA")] ISA_TYPE_AG => AG_ISA_IO_B!(addr) as *mut u8,
        #[cfg(feature = "CONFIG_ATARI_ROM_ISA")] ISA_TYPE_ENEC => ENEC_ISA_IO_B!(addr) as *mut u8,
        _ => core::ptr::null_mut(),
    }
}
#[inline]
pub unsafe fn isa_itw(addr: usize) -> *mut u16 {
    match ISA_TYPE {
        #[cfg(feature = "CONFIG_Q40")] ISA_TYPE_Q40 => Q40_ISA_IO_W!(addr) as *mut u16,
        #[cfg(feature = "CONFIG_AMIGA_PCMCIA")] ISA_TYPE_AG => AG_ISA_IO_W!(addr) as *mut u16,
        #[cfg(feature = "CONFIG_ATARI_ROM_ISA")] ISA_TYPE_ENEC => ENEC_ISA_IO_W!(addr) as *mut u16,
        _ => core::ptr::null_mut(),
    }
}
#[inline]
pub unsafe fn isa_itl(addr: usize) -> *mut u32 {
    match ISA_TYPE {
        #[cfg(feature = "CONFIG_AMIGA_PCMCIA")] ISA_TYPE_AG => AG_ISA_IO_W!(addr) as *mut u32,
        _ => core::ptr::null_mut(),
    }
}
#[inline]
pub unsafe fn isa_mtb(addr: usize) -> *mut u8 {
    match ISA_TYPE {
        #[cfg(feature = "CONFIG_Q40")] ISA_TYPE_Q40 => Q40_ISA_MEM_B!(addr) as *mut u8,
        #[cfg(feature = "CONFIG_AMIGA_PCMCIA")] ISA_TYPE_AG => addr as *mut u8,
        #[cfg(feature = "CONFIG_ATARI_ROM_ISA")] ISA_TYPE_ENEC => ENEC_ISA_MEM_B!(addr) as *mut u8,
        _ => core::ptr::null_mut(),
    }
}
#[inline]
pub unsafe fn isa_mtw(addr: usize) -> *mut u16 {
    match ISA_TYPE {
        #[cfg(feature = "CONFIG_Q40")] ISA_TYPE_Q40 => Q40_ISA_MEM_W!(addr) as *mut u16,
        #[cfg(feature = "CONFIG_AMIGA_PCMCIA")] ISA_TYPE_AG => addr as *mut u16,
        #[cfg(feature = "CONFIG_ATARI_ROM_ISA")] ISA_TYPE_ENEC => ENEC_ISA_MEM_W!(addr) as *mut u16,
        _ => core::ptr::null_mut(),
    }
}

#[inline]
pub unsafe fn isa_delay() {
    match ISA_TYPE {
        #[cfg(feature = "CONFIG_Q40")] ISA_TYPE_Q40 => isa_outb!(0, 0x80),
        _ => (),
    }
}

macro_rules! isa_inb { ($port:expr) => { in_8(unsafe { isa_itb($port as usize) }) }; }
macro_rules! isa_inw { ($port:expr) => { if ISA_SEX != 0 { in_be16(unsafe { isa_itw($port as usize) }) } else { in_le16(unsafe { isa_itw($port as usize) }) } }; }
macro_rules! isa_inl { ($port:expr) => { if ISA_SEX != 0 { in_be32(unsafe { isa_itl($port as usize) }) } else { in_le32(unsafe { isa_itl($port as usize) }) } }; }
macro_rules! isa_outb { ($val:expr, $port:expr) => { out_8(unsafe { isa_itb($port as usize) }, $val) }; }
macro_rules! isa_outw { ($val:expr, $port:expr) => { if ISA_SEX != 0 { out_be16(unsafe { isa_itw($port as usize) }, $val) } else { out_le16(unsafe { isa_itw($port as usize) }, $val) } }; }
macro_rules! isa_outl { ($val:expr, $port:expr) => { if ISA_SEX != 0 { out_be32(unsafe { isa_itl($port as usize) }, $val) } else { out_le32(unsafe { isa_itl($port as usize) }, $val) } }; }
macro_rules! isa_readb { ($p:expr) => { in_8(unsafe { isa_mtb($p as usize) }) }; }
macro_rules! isa_readw { ($p:expr) => { if ISA_SEX != 0 { in_be16(unsafe { isa_mtw($p as usize) }) } else { in_le16(unsafe { isa_mtw($p as usize) }) } }; }
macro_rules! isa_writeb { ($val:expr, $p:expr) => { out_8(unsafe { isa_mtb($p as usize) }, $val) }; }
macro_rules! isa_writew { ($val:expr, $p:expr) => { if ISA_SEX != 0 { out_be16(unsafe { isa_mtw($p as usize) }, $val) } else { out_le16(unsafe { isa_mtw($p as usize) }, $val) } }; }

macro_rules! isa_inb_p { ($p:expr) => {{ let v = isa_inb!($p); unsafe { isa_delay() }; v }}; }
macro_rules! isa_outb_p { ($v:expr, $p:expr) => {{ isa_outb!($v, $p); unsafe { isa_delay() }; }}; }
macro_rules! isa_inw_p { ($p:expr) => {{ let v = isa_inw!($p); unsafe { isa_delay() }; v }}; }
macro_rules! isa_outw_p { ($v:expr, $p:expr) => {{ isa_outw!($v, $p); unsafe { isa_delay() }; }}; }
macro_rules! isa_inl_p { ($p:expr) => {{ let v = isa_inl!($p); unsafe { isa_delay() }; v }}; }
macro_rules! isa_outl_p { ($v:expr, $p:expr) => {{ isa_outl!($v, $p); unsafe { isa_delay() }; }}; }

macro_rules! isa_insb { ($port:expr, $buf:expr, $nr:expr) => { raw_insb(unsafe { isa_itb($port as usize) }, $buf as *mut u8, $nr) }; }
macro_rules! isa_outsb { ($port:expr, $buf:expr, $nr:expr) => { raw_outsb(unsafe { isa_itb($port as usize) }, $buf as *const u8, $nr) }; }
macro_rules! isa_insw { ($port:expr, $buf:expr, $nr:expr) => { if ISA_SEX != 0 { raw_insw(unsafe { isa_itw($port as usize) }, $buf as *mut u16, $nr) } else { raw_insw_swapw(unsafe { isa_itw($port as usize) }, $buf as *mut u16, $nr) } }; }
macro_rules! isa_outsw { ($port:expr, $buf:expr, $nr:expr) => { if ISA_SEX != 0 { raw_outsw(unsafe { isa_itw($port as usize) }, $buf as *const u16, $nr) } else { raw_outsw_swapw(unsafe { isa_itw($port as usize) }, $buf as *const u16, $nr) } }; }
macro_rules! isa_insl { ($port:expr, $buf:expr, $nr:expr) => { if ISA_SEX != 0 { raw_insl(unsafe { isa_itl($port as usize) }, $buf as *mut u32, $nr) } else { raw_insw_swapw(unsafe { isa_itw($port as usize) }, $buf as *mut u16, ($nr) << 1) } }; }
macro_rules! isa_outsl { ($port:expr, $buf:expr, $nr:expr) => { if ISA_SEX != 0 { raw_outsl(unsafe { isa_itl($port as usize) }, $buf as *const u32, $nr) } else { raw_outsw_swapw(unsafe { isa_itw($port as usize) }, $buf as *const u16, ($nr) << 1) } }; }

macro_rules! readl { ($addr:expr) => { in_le32($addr) }; }
macro_rules! writel { ($val:expr, $addr:expr) => { out_le32($addr, $val) }; }
macro_rules! readsb { ($port:expr, $buf:expr, $nr:expr) => { raw_insb($port, $buf as *mut u8, $nr) }; }
macro_rules! readsw { ($port:expr, $buf:expr, $nr:expr) => { raw_insw($port, $buf as *mut u16, $nr) }; }
macro_rules! readsl { ($port:expr, $buf:expr, $nr:expr) => { raw_insl($port, $buf as *mut u32, $nr) }; }
macro_rules! writesb { ($port:expr, $buf:expr, $nr:expr) => { raw_outsb($port, $buf as *const u8, $nr) }; }
macro_rules! writesw { ($port:expr, $buf:expr, $nr:expr) => { raw_outsw($port, $buf as *const u16, $nr) }; }
macro_rules! writesl { ($port:expr, $buf:expr, $nr:expr) => { raw_outsl($port, $buf as *const u32, $nr) }; }

#[cfg(not(feature = "CONFIG_SUN3"))]
pub const IO_SPACE_LIMIT: usize = 0xffff;
#[cfg(feature = "CONFIG_SUN3")]
pub const IO_SPACE_LIMIT: usize = 0x0fffffff;
pub const __ARCH_HAS_NO_PAGE_ZERO_MAPPED: i32 = 1;

macro_rules! readb_relaxed { ($addr:expr) => { readb!($addr) }; }
macro_rules! readw_relaxed { ($addr:expr) => { readw!($addr) }; }
macro_rules! readl_relaxed { ($addr:expr) => { readl!($addr) }; }
macro_rules! writeb_relaxed { ($b:expr, $addr:expr) => { writeb!($b, $addr) }; }
macro_rules! writew_relaxed { ($b:expr, $addr:expr) => { writew!($b, $addr) }; }
macro_rules! writel_relaxed { ($b:expr, $addr:expr) => { writel!($b, $addr) }; }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
