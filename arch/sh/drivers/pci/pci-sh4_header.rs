/* SPDX-License-Identifier: GPL-2.0 */

// The CPU-subtype conditional include is supplied by the surrounding build.
// The included pci-sh7780.h or pci-sh7751.h declarations are external dependencies.

pub const SH4_PCICR: u32 = 0x100;
pub const SH4_PCICR_PREFIX: u32 = 0xA5000000;
pub const SH4_PCICR_FTO: u32 = 0x00000400;
pub const SH4_PCICR_TRSB: u32 = 0x00000200;
pub const SH4_PCICR_BSWP: u32 = 0x00000100;
pub const SH4_PCICR_PLUP: u32 = 0x00000080;
pub const SH4_PCICR_ARBM: u32 = 0x00000040;
pub const SH4_PCICR_MD: u32 = 0x00000030;
pub const SH4_PCICR_SERR: u32 = 0x00000008;
pub const SH4_PCICR_INTA: u32 = 0x00000004;
pub const SH4_PCICR_PRST: u32 = 0x00000002;
pub const SH4_PCICR_CFIN: u32 = 0x00000001;
pub const SH4_PCILSR0: u32 = 0x104;
pub const SH4_PCILSR1: u32 = 0x108;
pub const SH4_PCILAR0: u32 = 0x10C;
pub const SH4_PCILAR1: u32 = 0x110;
pub const SH4_PCIINT: u32 = 0x114;
pub const SH4_PCIINT_MLCK: u32 = 0x00008000;
pub const SH4_PCIINT_TABT: u32 = 0x00004000;
pub const SH4_PCIINT_TRET: u32 = 0x00000200;
pub const SH4_PCIINT_MFDE: u32 = 0x00000100;
pub const SH4_PCIINT_PRTY: u32 = 0x00000080;
pub const SH4_PCIINT_SERR: u32 = 0x00000040;
pub const SH4_PCIINT_TWDP: u32 = 0x00000020;
pub const SH4_PCIINT_TRDP: u32 = 0x00000010;
pub const SH4_PCIINT_MTABT: u32 = 0x00000008;
pub const SH4_PCIINT_MMABT: u32 = 0x00000004;
pub const SH4_PCIINT_MWPD: u32 = 0x00000002;
pub const SH4_PCIINT_MRPD: u32 = 0x00000001;
pub const SH4_PCIINTM: u32 = 0x118;
pub const SH4_PCIINTM_TTADIM: u32 = 1 << 14;
pub const SH4_PCIINTM_TMTOIM: u32 = 1 << 9;
pub const SH4_PCIINTM_MDEIM: u32 = 1 << 8;
pub const SH4_PCIINTM_APEDIM: u32 = 1 << 7;
pub const SH4_PCIINTM_SDIM: u32 = 1 << 6;
pub const SH4_PCIINTM_DPEITWM: u32 = 1 << 5;
pub const SH4_PCIINTM_PEDITRM: u32 = 1 << 4;
pub const SH4_PCIINTM_TADIMM: u32 = 1 << 3;
pub const SH4_PCIINTM_MADIMM: u32 = 1 << 2;
pub const SH4_PCIINTM_MWPDIM: u32 = 1 << 1;
pub const SH4_PCIINTM_MRDPEIM: u32 = 1;
pub const SH4_PCIALR: u32 = 0x11C;
pub const SH4_PCICLR: u32 = 0x120;
pub const SH4_PCICLR_MPIO: u32 = 0x80000000;
pub const SH4_PCICLR_MDMA0: u32 = 0x40000000;
pub const SH4_PCICLR_MDMA1: u32 = 0x20000000;
pub const SH4_PCICLR_MDMA2: u32 = 0x10000000;
pub const SH4_PCICLR_MDMA3: u32 = 0x08000000;
pub const SH4_PCICLR_TGT: u32 = 0x04000000;
pub const SH4_PCICLR_CMDL: u32 = 0x0000000F;
pub const SH4_PCIAINT: u32 = 0x130;
pub const SH4_PCIAINT_MBKN: u32 = 0x00002000;
pub const SH4_PCIAINT_TBTO: u32 = 0x00001000;
pub const SH4_PCIAINT_MBTO: u32 = 0x00000800;
pub const SH4_PCIAINT_TABT: u32 = 0x00000008;
pub const SH4_PCIAINT_MABT: u32 = 0x00000004;
pub const SH4_PCIAINT_RDPE: u32 = 0x00000002;
pub const SH4_PCIAINT_WDPE: u32 = 0x00000001;
pub const SH4_PCIAINTM: u32 = 0x134;
pub const SH4_PCIBMLR: u32 = 0x138;
pub const SH4_PCIBMLR_REQ4: u32 = 0x10;
pub const SH4_PCIBMLR_REQ3: u32 = 0x08;
pub const SH4_PCIBMLR_REQ2: u32 = 0x04;
pub const SH4_PCIBMLR_REQ1: u32 = 0x02;
pub const SH4_PCIBMLR_REQ0: u32 = 0x01;
pub const SH4_PCIDMABT: u32 = 0x140;
pub const SH4_PCIDMABT_RRBN: u32 = 0x01;
pub const SH4_PCIDPA0: u32 = 0x180;
pub const SH4_PCIDLA0: u32 = 0x184;
pub const SH4_PCIDTC0: u32 = 0x188;
pub const SH4_PCIDCR0: u32 = 0x18C;
pub const SH4_PCIDCR_ALGN: u32 = 0x00000600;
pub const SH4_PCIDCR_MAST: u32 = 0x00000100;
pub const SH4_PCIDCR_INTM: u32 = 0x00000080;
pub const SH4_PCIDCR_INTS: u32 = 0x00000040;
pub const SH4_PCIDCR_LHLD: u32 = 0x00000020;
pub const SH4_PCIDCR_PHLD: u32 = 0x00000010;
pub const SH4_PCIDCR_IOSEL: u32 = 0x00000008;
pub const SH4_PCIDCR_DIR: u32 = 0x00000004;
pub const SH4_PCIDCR_STOP: u32 = 0x00000002;
pub const SH4_PCIDCR_STRT: u32 = 0x00000001;
pub const SH4_PCIDPA1: u32 = 0x190;
pub const SH4_PCIDLA1: u32 = 0x194;
pub const SH4_PCIDTC1: u32 = 0x198;
pub const SH4_PCIDCR1: u32 = 0x19C;
pub const SH4_PCIDPA2: u32 = 0x1A0;
pub const SH4_PCIDLA2: u32 = 0x1A4;
pub const SH4_PCIDTC2: u32 = 0x1A8;
pub const SH4_PCIDCR2: u32 = 0x1AC;
pub const SH4_PCIDPA3: u32 = 0x1B0;
pub const SH4_PCIDLA3: u32 = 0x1B4;
pub const SH4_PCIDTC3: u32 = 0x1B8;
pub const SH4_PCIDCR3: u32 = 0x1BC;
pub const SH4_PCIPAR: u32 = 0x1C0;
pub const SH4_PCIPAR_CFGEN: u32 = 0x80000000;
pub const SH4_PCIPAR_BUSNO: u32 = 0x00FF0000;
pub const SH4_PCIPAR_DEVNO: u32 = 0x0000FF00;
pub const SH4_PCIPAR_REGAD: u32 = 0x000000FC;
pub const SH4_PCIMBR: u32 = 0x1C4;
pub const SH4_PCIMBR_MASK: u32 = 0xFF000000;
pub const SH4_PCIMBR_LOCK: u32 = 0x00000001;
pub const SH4_PCIIOBR: u32 = 0x1C8;
pub const SH4_PCIIOBR_MASK: u32 = 0xFFFC0000;
pub const SH4_PCIIOBR_LOCK: u32 = 0x00000001;
pub const SH4_PCIPINT: u32 = 0x1CC;
pub const SH4_PCIPINT_D3: u32 = 0x00000002;
pub const SH4_PCIPINT_D0: u32 = 0x00000001;
pub const SH4_PCIPINTM: u32 = 0x1D0;
pub const SH4_PCICLKR: u32 = 0x1D4;
pub const SH4_PCICLKR_PCSTP: u32 = 0x00000002;
pub const SH4_PCICLKR_BCSTP: u32 = 0x00000001;
pub const SH4_PCIBCR1: u32 = 0x1E0;
pub const SH4_PCIMBR0: u32 = SH4_PCIBCR1;
pub const SH4_PCIBCR2: u32 = 0x1E4;
pub const SH4_PCIMBMR0: u32 = SH4_PCIBCR2;
pub const SH4_PCIWCR1: u32 = 0x1E8;
pub const SH4_PCIWCR2: u32 = 0x1EC;
pub const SH4_PCIWCR3: u32 = 0x1F0;
pub const SH4_PCIMBR2: u32 = SH4_PCIWCR3;
pub const SH4_PCIMCR: u32 = 0x1F4;
pub const SH4_PCIBCR3: u32 = 0x1f8;
pub const SH4_PCIPCTR: u32 = 0x200;
pub const SH4_PCIPCTR_P2EN: u32 = 0x000400000;
pub const SH4_PCIPCTR_P1EN: u32 = 0x000200000;
pub const SH4_PCIPCTR_P0EN: u32 = 0x000100000;
pub const SH4_PCIPCTR_P2UP: u32 = 0x000000020;
pub const SH4_PCIPCTR_P2IO: u32 = 0x000000010;
pub const SH4_PCIPCTR_P1UP: u32 = 0x000000008;
pub const SH4_PCIPCTR_P1IO: u32 = 0x000000004;
pub const SH4_PCIPCTR_P0UP: u32 = 0x000000002;
pub const SH4_PCIPCTR_P0IO: u32 = 0x000000001;
pub const SH4_PCIPDTR: u32 = 0x204;
pub const SH4_PCIPDTR_PB5: u32 = 0x000000020;
pub const SH4_PCIPDTR_PB4: u32 = 0x000000010;
pub const SH4_PCIPDTR_PB3: u32 = 0x000000008;
pub const SH4_PCIPDTR_PB2: u32 = 0x000000004;
pub const SH4_PCIPDTR_PB1: u32 = 0x000000002;
pub const SH4_PCIPDTR_PB0: u32 = 0x000000001;
pub const SH4_PCIPDR: u32 = 0x220;

// External declarations supplied by the surrounding PCI implementation.
#[allow(non_camel_case_types)]
pub struct pci_ops;
#[repr(C)]
pub struct pci_channel {
    pub reg_base: usize,
}

unsafe extern "C" {
    pub static mut sh4_pci_ops: pci_ops;
    pub fn pci_fixup_pcic(chan: *mut pci_channel) -> core::ffi::c_int;
    fn __raw_writel(value: u32, address: usize);
    fn __raw_readl(address: usize) -> u32;
}

#[repr(C)]
pub struct sh4_pci_address_space {
    pub base: usize,
    pub size: usize,
}

#[repr(C)]
pub struct sh4_pci_address_map {
    pub window0: sh4_pci_address_space,
    pub window1: sh4_pci_address_space,
}

#[inline]
pub unsafe fn pci_write_reg(chan: *mut pci_channel, val: usize, reg: usize) {
    __raw_writel(val as u32, (*chan).reg_base.wrapping_add(reg));
}

#[inline]
pub unsafe fn pci_read_reg(chan: *mut pci_channel, reg: usize) -> usize {
    __raw_readl((*chan).reg_base.wrapping_add(reg)) as usize
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
