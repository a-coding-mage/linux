/* SPDX-License-Identifier: GPL-2.0 */

// <linux/io.h>, <linux/types.h>, and <linux/bitops.h> provide the MMIO and
// integer interfaces referenced by this header.

pub const SRSTR: usize = 0x000;
pub const SRSTR_MAGIC: u32 = 0x1971; /* Fixed magical read value */

pub const INTASR: usize = 0x010;
pub const INTAMR: usize = 0x020;
pub const MODSWR: usize = 0x030;
pub const INTTESTR: usize = 0x040;
pub const SYSSR: usize = 0x050;
pub const NRGPR: usize = 0x060;

pub const NMISR: usize = 0x070;
pub const NMISR_MAN_NMI: u32 = 1u32 << 0;
pub const NMISR_AUX_NMI: u32 = 1u32 << 1;
pub const NMISR_MASK: u32 = NMISR_MAN_NMI | NMISR_AUX_NMI;

pub const NMIMR: usize = 0x080;
pub const NMIMR_MAN_NMIM: u32 = 1u32 << 0; /* Manual NMI mask */
pub const NMIMR_AUX_NMIM: u32 = 1u32 << 1; /* Auxiliary NMI mask */
pub const NMIMR_MASK: u32 = NMIMR_MAN_NMIM | NMIMR_AUX_NMIM;

pub const INTBSR: usize = 0x090;
pub const INTBMR: usize = 0x0a0;
pub const USRLEDR: usize = 0x0b0;
pub const MAPSWR: usize = 0x0c0;
pub const FPGAVR: usize = 0x0d0;
pub const FPGADR: usize = 0x0e0;
pub const PCBRR: usize = 0x0f0;
pub const RSR: usize = 0x100;
pub const EXTASR: usize = 0x110;
pub const SPCAR: usize = 0x120;
pub const INTMSR: usize = 0x130;

pub const PCIECR: usize = 0x140;
pub const PCIECR_PCIEMUX1: u32 = 1u32 << 15;
pub const PCIECR_PCIEMUX0: u32 = 1u32 << 14;
pub const PCIECR_PRST4: u32 = 1u32 << 12; /* slot 4 card present */
pub const PCIECR_PRST3: u32 = 1u32 << 11; /* slot 3 card present */
pub const PCIECR_PRST2: u32 = 1u32 << 10; /* slot 2 card present */
pub const PCIECR_PRST1: u32 = 1u32 << 9; /* slot 1 card present */
pub const PCIECR_CLKEN: u32 = 1u32 << 4; /* oscillator enable */

pub const FAER: usize = 0x150;
pub const USRGPIR: usize = 0x160;
/* 0x170 reserved */

pub const LCLASR: usize = 0x180;
pub const LCLASR_FRAMEN: u32 = 1u32 << 15;
pub const LCLASR_FPGA_SEL_SHIFT: u32 = 12;
pub const LCLASR_NAND_SEL_SHIFT: u32 = 8;
pub const LCLASR_NORB_SEL_SHIFT: u32 = 4;
pub const LCLASR_NORA_SEL_SHIFT: u32 = 0;
pub const LCLASR_AREA_MASK: u32 = 0x7;
pub const LCLASR_FPGA_SEL_MASK: u32 = LCLASR_AREA_MASK << LCLASR_FPGA_SEL_SHIFT;
pub const LCLASR_NAND_SEL_MASK: u32 = LCLASR_AREA_MASK << LCLASR_NAND_SEL_SHIFT;
pub const LCLASR_NORB_SEL_MASK: u32 = LCLASR_AREA_MASK << LCLASR_NORB_SEL_SHIFT;
pub const LCLASR_NORA_SEL_MASK: u32 = LCLASR_AREA_MASK << LCLASR_NORA_SEL_SHIFT;

pub const SBCR: usize = 0x190;
pub const SCBR_I2CMEN: u32 = 1u32 << 0; /* FPGA I2C master enable */
pub const SCBR_I2CCEN: u32 = 1u32 << 1; /* CPU I2C master enable */

pub const PWRCR: usize = 0x1a0;
pub const PWRCR_SCISEL0: u32 = 1u32 << 0;
pub const PWRCR_SCISEL1: u32 = 1u32 << 1;
pub const PWRCR_SCIEN: u32 = 1u32 << 2; /* Serial port enable */
pub const PWRCR_PDWNACK: u32 = 1u32 << 5; /* Power down acknowledge */
pub const PWRCR_PDWNREQ: u32 = 1u32 << 7; /* Power down request */
pub const PWRCR_INT2: u32 = 1u32 << 11; /* INT2 connection to power manager */
pub const PWRCR_BUPINIT: u32 = 1u32 << 13; /* DDR backup initialize */
pub const PWRCR_BKPRST: u32 = 1u32 << 15; /* Backup power reset */

pub const SPCBR: usize = 0x1b0;
pub const SPICR: usize = 0x1c0;
pub const SPIDR: usize = 0x1d0;
pub const I2CCR: usize = 0x1e0;
pub const I2CDR: usize = 0x1f0;
pub const FPGACR: usize = 0x200;

pub const IASELR1: usize = 0x210;
pub const IASELR2: usize = 0x220;
pub const IASELR3: usize = 0x230;
pub const IASELR4: usize = 0x240;
pub const IASELR5: usize = 0x250;
pub const IASELR6: usize = 0x260;
pub const IASELR7: usize = 0x270;
pub const IASELR8: usize = 0x280;
pub const IASELR9: usize = 0x290;
pub const IASELR10: usize = 0x2a0;
pub const IASELR11: usize = 0x2b0;
pub const IASELR12: usize = 0x2c0;
pub const IASELR13: usize = 0x2d0;
pub const IASELR14: usize = 0x2e0;
pub const IASELR15: usize = 0x2f0;
/* 0x300 reserved */
pub const IBSELR1: usize = 0x310;
pub const IBSELR2: usize = 0x320;
pub const IBSELR3: usize = 0x330;
pub const IBSELR4: usize = 0x340;
pub const IBSELR5: usize = 0x350;
pub const IBSELR6: usize = 0x360;
pub const IBSELR7: usize = 0x370;
pub const IBSELR8: usize = 0x380;
pub const IBSELR9: usize = 0x390;
pub const IBSELR10: usize = 0x3a0;
pub const IBSELR11: usize = 0x3b0;
pub const IBSELR12: usize = 0x3c0;
pub const IBSELR13: usize = 0x3d0;
pub const IBSELR14: usize = 0x3e0;
pub const IBSELR15: usize = 0x3f0;
pub const USRACR: usize = 0x400;
pub const BEEPR: usize = 0x410;
pub const USRLCDR: usize = 0x420;
pub const SMBCR: usize = 0x430;
pub const SMBDR: usize = 0x440;
pub const USBCR: usize = 0x450;
pub const AMSR: usize = 0x460;
pub const ACCR: usize = 0x470;
pub const SDIFCR: usize = 0x480;

/* arch/sh/boards/mach-sdk7786/fpga.c */
extern "C" {
    pub static mut sdk7786_fpga_base: *mut u8;
    pub fn sdk7786_fpga_init();
}

/* arch/sh/boards/mach-sdk7786/nmi.c */
extern "C" {
    pub fn sdk7786_nmi_init();
}

pub unsafe fn sdk7786_fpga_regaddr(reg: usize) -> *mut u8 {
    sdk7786_fpga_base.add(reg)
}

/*
 * A convenience wrapper from register offset to internal I2C address,
 * when the FPGA is in I2C slave mode.
 */
pub const fn sdk7786_fpga_i2caddr(reg: usize) -> usize {
    reg >> 3
}

extern "C" {
    pub fn ioread16(addr: *const u8) -> u16;
    pub fn iowrite16(val: u16, addr: *mut u8);
}

pub unsafe fn fpga_read_reg(reg: usize) -> u16 {
    ioread16(sdk7786_fpga_base.add(reg) as *const u8)
}

pub unsafe fn fpga_write_reg(val: u16, reg: usize) {
    iowrite16(val, sdk7786_fpga_base.add(reg));
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
