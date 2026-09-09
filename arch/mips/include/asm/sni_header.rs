/*
 * SNI specific definitions
 *
 * This file is subject to the terms and conditions of the GNU General Public
 * License.  See the file "COPYING" in the main directory of this archive
 * for more details.
 *
 * Copyright (C) 1997, 1998 by Ralf Baechle
 * Copyright (C) 2006 Thomas Bogendoerfer (tsbogend@alpha.franken.de)
 */

// Dependency supplied by the surrounding kernel translation: irqreturn types.

extern "C" {
    pub static mut sni_brd_type: ::core::ffi::c_uint;
}

pub const SNI_BRD_10: i32 = 2;
pub const SNI_BRD_10NEW: i32 = 3;
pub const SNI_BRD_TOWER_OASIC: i32 = 4;
pub const SNI_BRD_MINITOWER: i32 = 5;
pub const SNI_BRD_PCI_TOWER: i32 = 6;
pub const SNI_BRD_RM200: i32 = 7;
pub const SNI_BRD_PCI_MTOWER: i32 = 8;
pub const SNI_BRD_PCI_DESKTOP: i32 = 9;
pub const SNI_BRD_PCI_TOWER_CPLUS: i32 = 10;
pub const SNI_BRD_PCI_MTOWER_CPLUS: i32 = 11;

/* RM400 cpu types */
pub const SNI_CPU_M8021: i32 = 0x01;
pub const SNI_CPU_M8030: i32 = 0x04;
pub const SNI_CPU_M8031: i32 = 0x06;
pub const SNI_CPU_M8034: i32 = 0x0f;
pub const SNI_CPU_M8037: i32 = 0x07;
pub const SNI_CPU_M8040: i32 = 0x05;
pub const SNI_CPU_M8043: i32 = 0x09;
pub const SNI_CPU_M8050: i32 = 0x0b;
pub const SNI_CPU_M8053: i32 = 0x0d;

pub const SNI_PORT_BASE: usize = CKSEG1ADDR(0xb4000000);

/* ASIC PCI registers for big endian configuration. */
#[cfg(not(mipsel))]
pub const PCIMT_UCONF: usize = CKSEG1ADDR(0xbfff0004);
#[cfg(not(mipsel))] pub const PCIMT_IOADTIMEOUT2: usize = CKSEG1ADDR(0xbfff000c);
#[cfg(not(mipsel))] pub const PCIMT_IOMEMCONF: usize = CKSEG1ADDR(0xbfff0014);
#[cfg(not(mipsel))] pub const PCIMT_IOMMU: usize = CKSEG1ADDR(0xbfff001c);
#[cfg(not(mipsel))] pub const PCIMT_IOADTIMEOUT1: usize = CKSEG1ADDR(0xbfff0024);
#[cfg(not(mipsel))] pub const PCIMT_DMAACCESS: usize = CKSEG1ADDR(0xbfff002c);
#[cfg(not(mipsel))] pub const PCIMT_DMAHIT: usize = CKSEG1ADDR(0xbfff0034);
#[cfg(not(mipsel))] pub const PCIMT_ERRSTATUS: usize = CKSEG1ADDR(0xbfff003c);
#[cfg(not(mipsel))] pub const PCIMT_ERRADDR: usize = CKSEG1ADDR(0xbfff0044);
#[cfg(not(mipsel))] pub const PCIMT_SYNDROME: usize = CKSEG1ADDR(0xbfff004c);
#[cfg(not(mipsel))] pub const PCIMT_ITPEND: usize = CKSEG1ADDR(0xbfff0054);
#[cfg(not(mipsel))] pub const PCIMT_IRQSEL: usize = CKSEG1ADDR(0xbfff005c);
#[cfg(not(mipsel))] pub const PCIMT_TESTMEM: usize = CKSEG1ADDR(0xbfff0064);
#[cfg(not(mipsel))] pub const PCIMT_ECCREG: usize = CKSEG1ADDR(0xbfff006c);
#[cfg(not(mipsel))] pub const PCIMT_CONFIG_ADDRESS: usize = CKSEG1ADDR(0xbfff0074);
#[cfg(not(mipsel))] pub const PCIMT_ASIC_ID: usize = CKSEG1ADDR(0xbfff007c); // read
#[cfg(not(mipsel))] pub const PCIMT_SOFT_RESET: usize = CKSEG1ADDR(0xbfff007c); // write
#[cfg(not(mipsel))] pub const PCIMT_PIA_OE: usize = CKSEG1ADDR(0xbfff0084);
#[cfg(not(mipsel))] pub const PCIMT_PIA_DATAOUT: usize = CKSEG1ADDR(0xbfff008c);
#[cfg(not(mipsel))] pub const PCIMT_PIA_DATAIN: usize = CKSEG1ADDR(0xbfff0094);
#[cfg(not(mipsel))] pub const PCIMT_CACHECONF: usize = CKSEG1ADDR(0xbfff009c);
#[cfg(not(mipsel))] pub const PCIMT_INVSPACE: usize = CKSEG1ADDR(0xbfff00a4);

/* ASIC PCI registers for little endian configuration. */
#[cfg(mipsel)]
pub const PCIMT_UCONF: usize = CKSEG1ADDR(0xbfff0000);
#[cfg(mipsel)] pub const PCIMT_IOADTIMEOUT2: usize = CKSEG1ADDR(0xbfff0008);
#[cfg(mipsel)] pub const PCIMT_IOMEMCONF: usize = CKSEG1ADDR(0xbfff0010);
#[cfg(mipsel)] pub const PCIMT_IOMMU: usize = CKSEG1ADDR(0xbfff0018);
#[cfg(mipsel)] pub const PCIMT_IOADTIMEOUT1: usize = CKSEG1ADDR(0xbfff0020);
#[cfg(mipsel)] pub const PCIMT_DMAACCESS: usize = CKSEG1ADDR(0xbfff0028);
#[cfg(mipsel)] pub const PCIMT_DMAHIT: usize = CKSEG1ADDR(0xbfff0030);
#[cfg(mipsel)] pub const PCIMT_ERRSTATUS: usize = CKSEG1ADDR(0xbfff0038);
#[cfg(mipsel)] pub const PCIMT_ERRADDR: usize = CKSEG1ADDR(0xbfff0040);
#[cfg(mipsel)] pub const PCIMT_SYNDROME: usize = CKSEG1ADDR(0xbfff0048);
#[cfg(mipsel)] pub const PCIMT_ITPEND: usize = CKSEG1ADDR(0xbfff0050);
#[cfg(mipsel)] pub const PCIMT_IRQSEL: usize = CKSEG1ADDR(0xbfff0058);
#[cfg(mipsel)] pub const PCIMT_TESTMEM: usize = CKSEG1ADDR(0xbfff0060);
#[cfg(mipsel)] pub const PCIMT_ECCREG: usize = CKSEG1ADDR(0xbfff0068);
#[cfg(mipsel)] pub const PCIMT_CONFIG_ADDRESS: usize = CKSEG1ADDR(0xbfff0070);
#[cfg(mipsel)] pub const PCIMT_ASIC_ID: usize = CKSEG1ADDR(0xbfff0078); // read
#[cfg(mipsel)] pub const PCIMT_SOFT_RESET: usize = CKSEG1ADDR(0xbfff0078); // write
#[cfg(mipsel)] pub const PCIMT_PIA_OE: usize = CKSEG1ADDR(0xbfff0080);
#[cfg(mipsel)] pub const PCIMT_PIA_DATAOUT: usize = CKSEG1ADDR(0xbfff0088);
#[cfg(mipsel)] pub const PCIMT_PIA_DATAIN: usize = CKSEG1ADDR(0xbfff0090);
#[cfg(mipsel)] pub const PCIMT_CACHECONF: usize = CKSEG1ADDR(0xbfff0098);
#[cfg(mipsel)] pub const PCIMT_INVSPACE: usize = CKSEG1ADDR(0xbfff00a0);

pub const IT_INT2: i32 = 0x01;
pub const IT_INTD: i32 = 0x02;
pub const IT_INTC: i32 = 0x04;
pub const IT_INTB: i32 = 0x08;
pub const IT_INTA: i32 = 0x10;
pub const IT_EISA: i32 = 0x20;
pub const IT_SCSI: i32 = 0x40;
pub const IT_ETH: i32 = 0x80;

pub const PCIMT_PCI_CONF: usize = CKSEG1ADDR(0xbfff0100);
pub const PCIMT_CONFIG_DATA: u32 = 0x0cfc;
pub const PCIMT_CSMSR: usize = CKSEG1ADDR(0xbfd00000);
pub const PCIMT_CSSWITCH: usize = CKSEG1ADDR(0xbfd10000);
pub const PCIMT_CSITPEND: usize = CKSEG1ADDR(0xbfd20000);
pub const PCIMT_AUTO_PO_EN: usize = CKSEG1ADDR(0xbfd30000);
pub const PCIMT_CLR_TEMP: usize = CKSEG1ADDR(0xbfd40000);
pub const PCIMT_AUTO_PO_DIS: usize = CKSEG1ADDR(0xbfd50000);
pub const PCIMT_EXMSR: usize = CKSEG1ADDR(0xbfd60000);
pub const PCIMT_UNUSED1: usize = CKSEG1ADDR(0xbfd70000);
pub const PCIMT_CSWCSM: usize = CKSEG1ADDR(0xbfd80000);
pub const PCIMT_UNUSED2: usize = CKSEG1ADDR(0xbfd90000);
pub const PCIMT_CSLED: usize = CKSEG1ADDR(0xbfda0000);
pub const PCIMT_CSMAPISA: usize = CKSEG1ADDR(0xbfdb0000);
pub const PCIMT_CSRSTBP: usize = CKSEG1ADDR(0xbfdc0000);
pub const PCIMT_CLRPOFF: usize = CKSEG1ADDR(0xbfdd0000);
pub const PCIMT_CSTIMER: usize = CKSEG1ADDR(0xbfde0000);
pub const PCIMT_PWDN: usize = CKSEG1ADDR(0xbfdf0000);

pub const A20R_PT_CLOCK_BASE: usize = CKSEG1ADDR(0xbc040000);
pub const A20R_PT_TIM0_ACK: usize = CKSEG1ADDR(0xbc050000);
pub const A20R_PT_TIM1_ACK: usize = CKSEG1ADDR(0xbc060000);
pub const SNI_A20R_IRQ_BASE: i32 = MIPS_CPU_IRQ_BASE;
pub const SNI_A20R_IRQ_TIMER: i32 = SNI_A20R_IRQ_BASE + 5;
pub const SNI_PCIT_INT_REG: usize = CKSEG1ADDR(0xbfff000c);
pub const SNI_PCIT_INT_START: i32 = 24;
pub const SNI_PCIT_INT_END: i32 = 30;
pub const PCIT_IRQ_ETHERNET: i32 = MIPS_CPU_IRQ_BASE + 5;
pub const PCIT_IRQ_INTA: i32 = SNI_PCIT_INT_START + 0;
pub const PCIT_IRQ_INTB: i32 = SNI_PCIT_INT_START + 1;
pub const PCIT_IRQ_INTC: i32 = SNI_PCIT_INT_START + 2;
pub const PCIT_IRQ_INTD: i32 = SNI_PCIT_INT_START + 3;
pub const PCIT_IRQ_SCSI0: i32 = SNI_PCIT_INT_START + 4;
pub const PCIT_IRQ_SCSI1: i32 = SNI_PCIT_INT_START + 5;

pub const PCIMT_KEYBOARD_IRQ: i32 = 1;
pub const PCIMT_IRQ_INT2: i32 = 24;
pub const PCIMT_IRQ_INTD: i32 = 25;
pub const PCIMT_IRQ_INTC: i32 = 26;
pub const PCIMT_IRQ_INTB: i32 = 27;
pub const PCIMT_IRQ_INTA: i32 = 28;
pub const PCIMT_IRQ_EISA: i32 = 29;
pub const PCIMT_IRQ_SCSI: i32 = 30;
pub const PCIMT_IRQ_ETHERNET: i32 = MIPS_CPU_IRQ_BASE + 6;
pub const PCIMT_EISA_BASE: usize = CKSEG1ADDR(0xb0000000);
pub const PCIMT_INT_ACKNOWLEDGE: usize = CKSEG1ADDR(0xba000000);

/* SNI ID PROM */
#[cfg(target_endian = "big")] pub const __SNI_END: u32 = 0;
#[cfg(target_endian = "little")] pub const __SNI_END: u32 = 3;
pub const SNI_IDPROM_BASE: usize = CKSEG1ADDR(0x1ff00000);
pub const SNI_IDPROM_MEMSIZE: usize = SNI_IDPROM_BASE + (0x28 ^ __SNI_END as usize);
pub const SNI_IDPROM_BRDTYPE: usize = SNI_IDPROM_BASE + (0x29 ^ __SNI_END as usize);
pub const SNI_IDPROM_CPUTYPE: usize = SNI_IDPROM_BASE + (0x30 ^ __SNI_END as usize);
pub const SNI_IDPROM_SIZE: usize = 0x1000;

extern "C" {
    pub fn sni_a20r_init();
    pub fn sni_pcit_init();
    pub fn sni_rm200_init();
    pub fn sni_pcimt_init();
    pub fn sni_a20r_irq_init();
    pub fn sni_pcit_irq_init();
    pub fn sni_pcit_cplus_irq_init();
    pub fn sni_rm200_irq_init();
    pub fn sni_pcimt_irq_init();
    #[cfg(CONFIG_EISA)]
    pub fn sni_eisa_root_init() -> ::core::ffi::c_int;
    pub static mut sni_hwint: Option<unsafe extern "C" fn()>;
}

#[cfg(not(CONFIG_EISA))]
#[inline]
pub unsafe fn sni_eisa_root_init() -> ::core::ffi::c_int { 0 }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
