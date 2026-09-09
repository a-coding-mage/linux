/* SPDX-License-Identifier: GPL-2.0 */
/* Translated from the C header; CONFIG_* branches are represented by cfg features. */

pub const PA_NORFLASH_ADDR: u32 = 0x0000_0000;
pub const PA_NORFLASH_SIZE: u32 = 0x0400_0000;

#[cfg(feature = "CONFIG_SH_R7780MP")]
pub mod r7780mp {
    pub const PA_BCR: u32 = 0xa400_0000;
    pub const PA_SDPOW: i32 = -1;
    pub const PA_IRLMSK: u32 = PA_BCR + 0x0000;
    pub const PA_IRLMON: u32 = PA_BCR + 0x0002;
    pub const PA_IRLPRI1: u32 = PA_BCR + 0x0004;
    pub const PA_IRLPRI2: u32 = PA_BCR + 0x0006;
    pub const PA_IRLPRI3: u32 = PA_BCR + 0x0008;
    pub const PA_IRLPRI4: u32 = PA_BCR + 0x000a;
    pub const PA_RSTCTL: u32 = PA_BCR + 0x000c;
    pub const PA_PCIBD: u32 = PA_BCR + 0x000e;
    pub const PA_PCICD: u32 = PA_BCR + 0x0010;
    pub const PA_EXTGIO: u32 = PA_BCR + 0x0016;
    pub const PA_IVDRMON: u32 = PA_BCR + 0x0018;
    pub const PA_IVDRCTL: u32 = PA_BCR + 0x001a;
    pub const PA_OBLED: u32 = PA_BCR + 0x001c;
    pub const PA_OBSW: u32 = PA_BCR + 0x001e;
    pub const PA_AUDIOSEL: u32 = PA_BCR + 0x0020;
    pub const PA_EXTPLR: u32 = PA_BCR + 0x001e;
    pub const PA_TPCTL: u32 = PA_BCR + 0x0100;
    pub const PA_TPDCKCTL: u32 = PA_BCR + 0x0102;
    pub const PA_TPCTLCLR: u32 = PA_BCR + 0x0104;
    pub const PA_TPXPOS: u32 = PA_BCR + 0x0106;
    pub const PA_TPYPOS: u32 = PA_BCR + 0x0108;
    pub const PA_DBSW: u32 = PA_BCR + 0x0200;
    pub const PA_CFCTL: u32 = PA_BCR + 0x0300;
    pub const PA_CFPOW: u32 = PA_BCR + 0x0302;
    pub const PA_CFCDINTCLR: u32 = PA_BCR + 0x0304;
    pub const PA_SCSMR0: u32 = PA_BCR + 0x0400;
    pub const PA_SCBRR0: u32 = PA_BCR + 0x0404;
    pub const PA_SCSCR0: u32 = PA_BCR + 0x0408;
    pub const PA_SCFTDR0: u32 = PA_BCR + 0x040c;
    pub const PA_SCFSR0: u32 = PA_BCR + 0x0410;
    pub const PA_SCFRDR0: u32 = PA_BCR + 0x0414;
    pub const PA_SCFCR0: u32 = PA_BCR + 0x0418;
    pub const PA_SCTFDR0: u32 = PA_BCR + 0x041c;
    pub const PA_SCRFDR0: u32 = PA_BCR + 0x0420;
    pub const PA_SCSPTR0: u32 = PA_BCR + 0x0424;
    pub const PA_SCLSR0: u32 = PA_BCR + 0x0428;
    pub const PA_SCRER0: u32 = PA_BCR + 0x042c;
    pub const PA_SCSMR1: u32 = PA_BCR + 0x0500;
    pub const PA_SCBRR1: u32 = PA_BCR + 0x0504;
    pub const PA_SCSCR1: u32 = PA_BCR + 0x0508;
    pub const PA_SCFTDR1: u32 = PA_BCR + 0x050c;
    pub const PA_SCFSR1: u32 = PA_BCR + 0x0510;
    pub const PA_SCFRDR1: u32 = PA_BCR + 0x0514;
    pub const PA_SCFCR1: u32 = PA_BCR + 0x0518;
    pub const PA_SCTFDR1: u32 = PA_BCR + 0x051c;
    pub const PA_SCRFDR1: u32 = PA_BCR + 0x0520;
    pub const PA_SCSPTR1: u32 = PA_BCR + 0x0524;
    pub const PA_SCLSR1: u32 = PA_BCR + 0x0528;
    pub const PA_SCRER1: u32 = PA_BCR + 0x052c;
    pub const PA_SMCR: u32 = PA_BCR + 0x0600;
    pub const PA_SMSMADR: u32 = PA_BCR + 0x0602;
    pub const PA_SMMR: u32 = PA_BCR + 0x0604;
    pub const PA_SMSADR1: u32 = PA_BCR + 0x0606;
    pub const PA_SMTRDR1: u32 = PA_BCR + 0x0646;
    pub const PA_VERREG: u32 = PA_BCR + 0x0700;
    pub const PA_POFF: u32 = PA_BCR + 0x0800;
    pub const PA_PMR: u32 = PA_BCR + 0x0900;
    pub const IRLCNTR1: u32 = PA_BCR;
    pub const IVDR_CK_ON: u32 = 8;
}

#[cfg(feature = "CONFIG_SH_R7780RP")]
pub mod r7780rp {
    pub const PA_POFF: i32 = -1;
    pub const PA_BCR: u32 = 0xa500_0000;
    pub const PA_IRLMSK: u32 = PA_BCR + 0x0000;
    pub const PA_IRLMON: u32 = PA_BCR + 0x0002;
    pub const PA_SDPOW: u32 = PA_BCR + 0x0004;
    pub const PA_RSTCTL: u32 = PA_BCR + 0x0006;
    pub const PA_PCIBD: u32 = PA_BCR + 0x0008;
    pub const PA_PCICD: u32 = PA_BCR + 0x000a;
    pub const PA_ZIGIO1: u32 = PA_BCR + 0x000c;
    pub const PA_ZIGIO2: u32 = PA_BCR + 0x000e;
    pub const PA_ZIGIO3: u32 = PA_BCR + 0x0010;
    pub const PA_ZIGIO4: u32 = PA_BCR + 0x0012;
    pub const PA_IVDRMON: u32 = PA_BCR + 0x0014;
    pub const PA_IVDRCTL: u32 = PA_BCR + 0x0016;
    pub const PA_OBLED: u32 = PA_BCR + 0x0018;
    pub const PA_OBSW: u32 = PA_BCR + 0x001a;
    pub const PA_AUDIOSEL: u32 = PA_BCR + 0x001c;
    pub const PA_EXTPLR: u32 = PA_BCR + 0x001e;
    pub const PA_TPCTL: u32 = PA_BCR + 0x0100;
    pub const PA_TPDCKCTL: u32 = PA_BCR + 0x0102;
    pub const PA_TPCTLCLR: u32 = PA_BCR + 0x0104;
    pub const PA_TPXPOS: u32 = PA_BCR + 0x0106;
    pub const PA_TPYPOS: u32 = PA_BCR + 0x0108;
    pub const PA_DBDET: u32 = PA_BCR + 0x0200;
    pub const PA_DBDISPCTL: u32 = PA_BCR + 0x0202;
    pub const PA_DBSW: u32 = PA_BCR + 0x0204;
    pub const PA_CFCTL: u32 = PA_BCR + 0x0300;
    pub const PA_CFPOW: u32 = PA_BCR + 0x0302;
    pub const PA_CFCDINTCLR: u32 = PA_BCR + 0x0304;
    pub const PA_SCSMR: u32 = PA_BCR + 0x0400;
    pub const PA_SCBRR: u32 = PA_BCR + 0x0402;
    pub const PA_SCSCR: u32 = PA_BCR + 0x0404;
    pub const PA_SCFDTR: u32 = PA_BCR + 0x0406;
    pub const PA_SCFSR: u32 = PA_BCR + 0x0408;
    pub const PA_SCFRDR: u32 = PA_BCR + 0x040a;
    pub const PA_SCFCR: u32 = PA_BCR + 0x040c;
    pub const PA_SCFDR: u32 = PA_BCR + 0x040e;
    pub const PA_SCLSR: u32 = PA_BCR + 0x0412;
    pub const PA_SMCR: u32 = PA_BCR + 0x0500;
    pub const PA_SMSMADR: u32 = PA_BCR + 0x0502;
    pub const PA_SMMR: u32 = PA_BCR + 0x0504;
    pub const PA_SMSADR1: u32 = PA_BCR + 0x0506;
    pub const PA_SMTRDR1: u32 = PA_BCR + 0x0546;
    pub const PA_VERREG: u32 = PA_BCR + 0x0600;
    pub const PA_AX88796L: u32 = 0xa580_0400;
    pub const PA_SC1602BSLB: u32 = 0xa600_0000;
    pub const PA_IDE_OFFSET: u32 = 0x1f0;
    pub const AX88796L_IO_BASE: u32 = 0x1000;
    pub const IRLCNTR1: u32 = PA_BCR;
    pub const IVDR_CK_ON: u32 = 8;
}

#[cfg(feature = "CONFIG_SH_R7785RP")]
pub mod r7785rp {
    pub const PA_BCR: u32 = 0xa400_0000;
    pub const PA_SDPOW: i32 = -1;
    pub const PA_PCISCR: u32 = PA_BCR + 0x0000;
    pub const PA_IRLPRA: u32 = PA_BCR + 0x0002;
    pub const PA_IRLPRB: u32 = PA_BCR + 0x0004;
    pub const PA_IRLPRC: u32 = PA_BCR + 0x0006;
    pub const PA_IRLPRD: u32 = PA_BCR + 0x0008;
    pub const IRLCNTR1: u32 = PA_BCR + 0x0010;
    pub const PA_IRLPRE: u32 = PA_BCR + 0x000a;
    pub const PA_IRLPRF: u32 = PA_BCR + 0x000c;
    pub const PA_EXIRLCR: u32 = PA_BCR + 0x000e;
    pub const PA_IRLMCR1: u32 = PA_BCR + 0x0010;
    pub const PA_IRLMCR2: u32 = PA_BCR + 0x0012;
    pub const PA_IRLSSR1: u32 = PA_BCR + 0x0014;
    pub const PA_IRLSSR2: u32 = PA_BCR + 0x0016;
    pub const PA_CFTCR: u32 = PA_BCR + 0x0100;
    pub const PA_CFPCR: u32 = PA_BCR + 0x0102;
    pub const PA_PCICR: u32 = PA_BCR + 0x0110;
    pub const PA_IVDRCTL: u32 = PA_BCR + 0x0112;
    pub const PA_IVDRSR: u32 = PA_BCR + 0x0114;
    pub const PA_PDRSTCR: u32 = PA_BCR + 0x0116;
    pub const PA_POFF: u32 = PA_BCR + 0x0120;
    pub const PA_LCDCR: u32 = PA_BCR + 0x0130;
    pub const PA_TPCR: u32 = PA_BCR + 0x0140;
    pub const PA_TPCKCR: u32 = PA_BCR + 0x0142;
    pub const PA_TPRSTR: u32 = PA_BCR + 0x0144;
    pub const PA_TPXPDR: u32 = PA_BCR + 0x0146;
    pub const PA_TPYPDR: u32 = PA_BCR + 0x0148;
    pub const PA_GPIOPFR: u32 = PA_BCR + 0x0150;
    pub const PA_GPIODR: u32 = PA_BCR + 0x0152;
    pub const PA_OBLED: u32 = PA_BCR + 0x0154;
    pub const PA_SWSR: u32 = PA_BCR + 0x0156;
    pub const PA_VERREG: u32 = PA_BCR + 0x0158;
    pub const PA_SMCR: u32 = PA_BCR + 0x0200;
    pub const PA_SMSMADR: u32 = PA_BCR + 0x0202;
    pub const PA_SMMR: u32 = PA_BCR + 0x0204;
    pub const PA_SMSADR1: u32 = PA_BCR + 0x0206;
    pub const PA_SMSADR32: u32 = PA_BCR + 0x0244;
    pub const PA_SMTRDR1: u32 = PA_BCR + 0x0246;
    pub const PA_SMTRDR16: u32 = PA_BCR + 0x0264;
    pub const PA_CU3MDR: u32 = PA_BCR + 0x0300;
    pub const PA_CU5MDR: u32 = PA_BCR + 0x0302;
    pub const PA_MMSR: u32 = PA_BCR + 0x0400;
    pub const IVDR_CK_ON: u32 = 4;
}

pub const HL_FPGA_IRQ_BASE: u32 = 200 + 16;
pub const HL_NR_IRL: u32 = 15;
pub const IRQ_AX88796: u32 = HL_FPGA_IRQ_BASE;
pub const IRQ_CF: u32 = HL_FPGA_IRQ_BASE + 1;
pub const IRQ_PSW: u32 = HL_FPGA_IRQ_BASE + 2;
pub const IRQ_EXT0: u32 = HL_FPGA_IRQ_BASE + 3;
pub const IRQ_EXT1: u32 = HL_FPGA_IRQ_BASE + 4;
pub const IRQ_EXT2: u32 = HL_FPGA_IRQ_BASE + 5;
pub const IRQ_EXT3: u32 = HL_FPGA_IRQ_BASE + 6;
pub const IRQ_EXT4: u32 = HL_FPGA_IRQ_BASE + 7;
pub const IRQ_EXT5: u32 = HL_FPGA_IRQ_BASE + 8;
pub const IRQ_EXT6: u32 = HL_FPGA_IRQ_BASE + 9;
pub const IRQ_EXT7: u32 = HL_FPGA_IRQ_BASE + 10;
pub const IRQ_SMBUS: u32 = HL_FPGA_IRQ_BASE + 11;
pub const IRQ_TP: u32 = HL_FPGA_IRQ_BASE + 12;
pub const IRQ_RTC: u32 = HL_FPGA_IRQ_BASE + 13;
pub const IRQ_TH_ALERT: u32 = HL_FPGA_IRQ_BASE + 14;
pub const IRQ_SCIF0: u32 = HL_FPGA_IRQ_BASE + 15;
pub const IRQ_SCIF1: u32 = HL_FPGA_IRQ_BASE + 16;

extern "C" {
    pub fn highlander_plat_irq_setup() -> *mut u8;
}

#[cfg(feature = "CONFIG_SH_R7785RP")]
extern "C" {
    pub fn highlander_plat_pinmux_setup();
}

#[cfg(not(feature = "CONFIG_SH_R7785RP"))]
#[inline]
pub fn highlander_plat_pinmux_setup() {}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
