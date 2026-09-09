/* SPDX-License-Identifier: GPL-2.0-or-later */
/*
 * QUICC Engine (QE) Internal Memory Map.
 * The Internal Memory Map for devices with QE on them. This
 * is the superset of all QE devices (8360, etc.).
 *
 * Copyright (C) 2006. Freescale Semiconductor, Inc. All rights reserved.
 *
 * Authors: Shlomi Gridish <gridish@freescale.com>
 *          Li Yang <leoli@freescale.com>
 */

// C header guard: _ASM_POWERPC_IMMAP_QE_H
// C conditional: __KERNEL__

// C dependencies: linux/types.h and asm/io.h

pub const QE_IMMAP_SIZE: usize = 1024 * 1024; // 1MB from 1MB+IMMR

/* QE I-RAM */
#[repr(C, packed)]
pub struct qe_iram {
    pub iadd: __be32, // I-RAM Address Register
    pub idata: __be32, // I-RAM Data Register
    pub res0: [u8; 0x04],
    pub iready: __be32, // I-RAM Ready Register
    pub res1: [u8; 0x70],
}

/* QE Interrupt Controller */
#[repr(C, packed)]
pub struct qe_ic_regs {
    pub qicr: __be32,
    pub qivec: __be32,
    pub qripnr: __be32,
    pub qipnr: __be32,
    pub qipxcc: __be32,
    pub qipycc: __be32,
    pub qipwcc: __be32,
    pub qipzcc: __be32,
    pub qimr: __be32,
    pub qrimr: __be32,
    pub qicnr: __be32,
    pub res0: [u8; 0x4],
    pub qiprta: __be32,
    pub qiprtb: __be32,
    pub res1: [u8; 0x4],
    pub qricr: __be32,
    pub res2: [u8; 0x20],
    pub qhivec: __be32,
    pub res3: [u8; 0x1C],
}

/* Communications Processor */
#[repr(C, packed)]
pub struct cp_qe {
    pub cecr: __be32, // QE command register
    pub ceccr: __be32, // QE controller configuration register
    pub cecdr: __be32, // QE command data register
    pub res0: [u8; 0xA],
    pub ceter: __be16, // QE timer event register
    pub res1: [u8; 0x2],
    pub cetmr: __be16, // QE timers mask register
    pub cetscr: __be32, // QE time-stamp timer control register
    pub cetsr1: __be32, // QE time-stamp register 1
    pub cetsr2: __be32, // QE time-stamp register 2
    pub res2: [u8; 0x8],
    pub cevter: __be32, // QE virtual tasks event register
    pub cevtmr: __be32, // QE virtual tasks mask register
    pub cercr: __be16, // QE RAM control register
    pub res3: [u8; 0x2],
    pub res4: [u8; 0x24],
    pub ceexe1: __be16, // QE external request 1 event register
    pub res5: [u8; 0x2],
    pub ceexm1: __be16, // QE external request 1 mask register
    pub res6: [u8; 0x2],
    pub ceexe2: __be16, // QE external request 2 event register
    pub res7: [u8; 0x2],
    pub ceexm2: __be16, // QE external request 2 mask register
    pub res8: [u8; 0x2],
    pub ceexe3: __be16, // QE external request 3 event register
    pub res9: [u8; 0x2],
    pub ceexm3: __be16, // QE external request 3 mask register
    pub res10: [u8; 0x2],
    pub ceexe4: __be16, // QE external request 4 event register
    pub res11: [u8; 0x2],
    pub ceexm4: __be16, // QE external request 4 mask register
    pub res12: [u8; 0x3A],
    pub ceurnr: __be32, // QE microcode revision number register
    pub res13: [u8; 0x244],
}

/* QE Multiplexer */
#[repr(C, packed)]
pub struct qe_mux {
    pub cmxgcr: __be32, // CMX general clock route register
    pub cmxsi1cr_l: __be32, // CMX SI1 clock route low register
    pub cmxsi1cr_h: __be32, // CMX SI1 clock route high register
    pub cmxsi1syr: __be32, // CMX SI1 SYNC route register
    pub cmxucr: [__be32; 4], // CMX UCCx clock route registers
    pub cmxupcr: __be32, // CMX UPC clock route register
    pub res0: [u8; 0x1C],
}

/* QE Timers */
#[repr(C, packed)]
pub struct qe_timers {
    pub gtcfr1: u8, // Timer 1 and Timer 2 global config register
    pub res0: [u8; 0x3],
    pub gtcfr2: u8, // Timer 3 and timer 4 global config register
    pub res1: [u8; 0xB],
    pub gtmdr1: __be16, pub gtmdr2: __be16,
    pub gtrfr1: __be16, pub gtrfr2: __be16,
    pub gtcpr1: __be16, pub gtcpr2: __be16,
    pub gtcnr1: __be16, pub gtcnr2: __be16,
    pub gtmdr3: __be16, pub gtmdr4: __be16,
    pub gtrfr3: __be16, pub gtrfr4: __be16,
    pub gtcpr3: __be16, pub gtcpr4: __be16,
    pub gtcnr3: __be16, pub gtcnr4: __be16,
    pub gtevr1: __be16, pub gtevr2: __be16,
    pub gtevr3: __be16, pub gtevr4: __be16,
    pub gtps: __be16, // Timer 1 prescale register
    pub res2: [u8; 0x46],
}

/* BRG */
#[repr(C, packed)]
pub struct qe_brg { pub brgc: [__be32; 16], pub res0: [u8; 0x40] }

/* SPI */
#[repr(C, packed)]
pub struct spi {
    pub res0: [u8; 0x20], pub spmode: __be32,
    pub res1: [u8; 0x2], pub spie: u8, pub res2: [u8; 0x1],
    pub res3: [u8; 0x2], pub spim: u8, pub res4: [u8; 0x1],
    pub res5: [u8; 0x1], pub spcom: u8, pub res6: [u8; 0x2],
    pub spitd: __be32, pub spird: __be32, pub res7: [u8; 0x8],
}

/* SI */
#[repr(C, packed)]
pub struct si1 {
    pub sixmr1: [__be16; 4], pub siglmr1_h: u8, pub res0: [u8; 1],
    pub sicmdr1_h: u8, pub res2: [u8; 1], pub sistr1_h: u8, pub res3: [u8; 1],
    pub sirsr1_h: __be16, pub sitarc1: u8, pub sitbrc1: u8, pub sitcrc1: u8, pub sitdrc1: u8,
    pub sirarc1: u8, pub sirbrc1: u8, pub sircrc1: u8, pub sirdrc1: u8, pub res4: [u8; 8],
    pub siemr1: __be16, pub sifmr1: __be16, pub sigmr1: __be16, pub sihmr1: __be16,
    pub siglmg1_l: u8, pub res5: [u8; 1], pub sicmdr1_l: u8, pub res6: [u8; 1],
    pub sistr1_l: u8, pub res7: [u8; 1], pub sirsr1_l: __be16,
    pub siterc1: u8, pub sitfrc1: u8, pub sitgrc1: u8, pub sithrc1: u8,
    pub sirerc1: u8, pub sirfrc1: u8, pub sirgrc1: u8, pub sirhrc1: u8, pub res8: [u8; 8],
    pub siml1: __be32, pub siedm1: u8, pub res9: [u8; 0xBB],
}

/* SI Routing Tables */
#[repr(C, packed)]
pub struct sir { pub tx: [u8; 0x400], pub rx: [u8; 0x400], pub res0: [u8; 0x800] }

/* USB Controller */
#[repr(C, packed)]
pub struct qe_usb_ctlr {
    pub usb_usmod: u8, pub usb_usadr: u8, pub usb_uscom: u8, pub res1: [u8; 1],
    pub usb_usep: [__be16; 4], pub res2: [u8; 4], pub usb_usber: __be16,
    pub res3: [u8; 2], pub usb_usbmr: __be16, pub res4: [u8; 1], pub usb_usbs: u8,
    pub usb_ussft: __be16, pub res5: [u8; 2], pub usb_usfrn: __be16, pub res6: [u8; 0x22],
}

/* MCC */
#[repr(C, packed)]
pub struct qe_mcc {
    pub mcce: __be32, pub mccm: __be32, pub mccf: __be32, pub merl: __be32, pub res0: [u8; 0xF0],
}

/* QE UCC Slow */
#[repr(C, packed)]
pub struct ucc_slow {
    pub gumr_l: __be32, pub gumr_h: __be32, pub upsmr: __be16, pub res0: [u8; 2],
    pub utodr: __be16, pub udsr: __be16, pub ucce: __be16, pub res1: [u8; 2],
    pub uccm: __be16, pub res2: [u8; 1], pub uccs: u8, pub res3: [u8; 0x24],
    pub utpt: __be16, pub res4: [u8; 0x52], pub guemr: u8,
}

/* QE UCC Fast */
#[repr(C, packed)]
pub struct ucc_fast {
    pub gumr: __be32, pub upsmr: __be32, pub utodr: __be16, pub res0: [u8; 2],
    pub udsr: __be16, pub res1: [u8; 2], pub ucce: __be32, pub uccm: __be32,
    pub uccs: u8, pub res2: [u8; 7], pub urfb: __be32, pub urfs: __be16, pub res3: [u8; 2],
    pub urfet: __be16, pub urfset: __be16, pub utfb: __be32, pub utfs: __be16, pub res4: [u8; 2],
    pub utfet: __be16, pub res5: [u8; 2], pub utftt: __be16, pub res6: [u8; 2],
    pub utpt: __be16, pub res7: [u8; 2], pub urtry: __be32, pub res8: [u8; 0x4C], pub guemr: u8,
}

#[repr(C, packed)]
pub union ucc_union { pub slow: ucc_slow, pub fast: ucc_fast, pub res: [u8; 0x200] }

#[repr(C, packed)]
pub struct ucc { pub data: ucc_union }

/* MultiPHY UTOPIA POS Controllers (UPC) */
#[repr(C, packed)]
pub struct upc {
    pub upgcr: __be32, pub uplpa: __be32, pub uphec: __be32, pub upuc: __be32,
    pub updc1: __be32, pub updc2: __be32, pub updc3: __be32, pub updc4: __be32,
    pub upstpa: __be32, pub res0: [u8; 0xC],
    pub updrs1_h: __be32, pub updrs1_l: __be32, pub updrs2_h: __be32, pub updrs2_l: __be32,
    pub updrs3_h: __be32, pub updrs3_l: __be32, pub updrs4_h: __be32, pub updrs4_l: __be32,
    pub updrp1: __be32, pub updrp2: __be32, pub updrp3: __be32, pub updrp4: __be32,
    pub upde1: __be32, pub upde2: __be32, pub upde3: __be32, pub upde4: __be32,
    pub uprp1: __be16, pub uprp2: __be16, pub uprp3: __be16, pub uprp4: __be16, pub res1: [u8; 8],
    pub uptirr1_0: __be16, pub uptirr1_1: __be16, pub uptirr1_2: __be16, pub uptirr1_3: __be16,
    pub uptirr2_0: __be16, pub uptirr2_1: __be16, pub uptirr2_2: __be16, pub uptirr2_3: __be16,
    pub uptirr3_0: __be16, pub uptirr3_1: __be16, pub uptirr3_2: __be16, pub uptirr3_3: __be16,
    pub uptirr4_0: __be16, pub uptirr4_1: __be16, pub uptirr4_2: __be16, pub uptirr4_3: __be16,
    pub uper1: __be32, pub uper2: __be32, pub uper3: __be32, pub uper4: __be32, pub res2: [u8; 0x150],
}

/* SDMA */
#[repr(C, packed)]
pub struct sdma {
    pub sdsr: __be32, pub sdmr: __be32, pub sdtr1: __be32, pub sdtr2: __be32,
    pub sdhy1: __be32, pub sdhy2: __be32, pub sdta1: __be32, pub sdta2: __be32,
    pub sdtm1: __be32, pub sdtm2: __be32, pub res0: [u8; 0x10], pub sdaqr: __be32,
    pub sdaqmr: __be32, pub res1: [u8; 0x4], pub sdebcr: __be32, pub res2: [u8; 0x38],
}

/* Debug Space */
#[repr(C, packed)]
pub struct dbg {
    pub bpdcr: __be32, pub bpdsr: __be32, pub bpdmr: __be32, pub bprmrr0: __be32, pub bprmrr1: __be32,
    pub res0: [u8; 0x8], pub bprmtr0: __be32, pub bprmtr1: __be32, pub res1: [u8; 0x8],
    pub bprmir: __be32, pub bprmsr: __be32, pub bpemr: __be32, pub res2: [u8; 0x48],
}

/* RISC Special Registers (Trap and Breakpoint). These are described in the QE Developer's Handbook. */
#[repr(C, packed)]
pub struct rsp {
    pub tibcr: [__be32; 16], pub res0: [u8; 64], pub ibcr0: __be32, pub ibs0: __be32, pub ibcnr0: __be32,
    pub res1: [u8; 4], pub ibcr1: __be32, pub ibs1: __be32, pub ibcnr1: __be32, pub npcr: __be32,
    pub dbcr: __be32, pub dbar: __be32, pub dbamr: __be32, pub dbsr: __be32, pub dbcnr: __be32,
    pub res2: [u8; 12], pub dbdr_h: __be32, pub dbdr_l: __be32, pub dbdmr_h: __be32, pub dbdmr_l: __be32,
    pub bsr: __be32, pub bor: __be32, pub bior: __be32, pub res3: [u8; 4], pub iatr: [__be32; 4],
    pub eccr: __be32, pub eicr: __be32, pub res4: [u8; 0x100 - 0xf8],
}

#[repr(C, packed)]
pub struct qe_immap {
    pub iram: qe_iram, pub ic: qe_ic_regs, pub cp: cp_qe, pub qmx: qe_mux, pub qet: qe_timers,
    pub spi: [spi; 0x2], pub mcc: qe_mcc, pub brg: qe_brg, pub usb: qe_usb_ctlr, pub si1: si1,
    pub res11: [u8; 0x800], pub sir: sir, pub ucc1: ucc, pub ucc3: ucc, pub ucc5: ucc, pub ucc7: ucc,
    pub res12: [u8; 0x600], pub upc1: upc, pub ucc2: ucc, pub ucc4: ucc, pub ucc6: ucc, pub ucc8: ucc,
    pub res13: [u8; 0x600], pub upc2: upc, pub sdma: sdma, pub dbg: dbg, pub rsp: [rsp; 0x2],
    pub res14: [u8; 0x300], pub res15: [u8; 0x3A00], pub res16: [u8; 0x8000], pub muram: [u8; 0xC000],
    pub res17: [u8; 0x24000], pub res18: [u8; 0xC0000],
}

// extern struct qe_immap __iomem *qe_immr;
extern "C" {
    pub static mut qe_immr: *mut qe_immap;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
