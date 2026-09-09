/*
 * PCI / PCI-X / PCI-Express support for 4xx parts
 *
 * Copyright 2007 Ben. Herrenschmidt <benh@kernel.crashing.org>, IBM Corp.
 *
 * Bits and pieces extracted from arch/ppc support by
 *
 * Matt Porter <mporter@kernel.crashing.org>
 *
 * Copyright 2002-2005 MontaVista Software Inc.
 */

/*
 * 4xx PCI-X bridge register definitions
 */
pub const PCIX0_VENDID: u32 = 0x000;
pub const PCIX0_DEVID: u32 = 0x002;
pub const PCIX0_COMMAND: u32 = 0x004;
pub const PCIX0_STATUS: u32 = 0x006;
pub const PCIX0_REVID: u32 = 0x008;
pub const PCIX0_CLS: u32 = 0x009;
pub const PCIX0_CACHELS: u32 = 0x00c;
pub const PCIX0_LATTIM: u32 = 0x00d;
pub const PCIX0_HDTYPE: u32 = 0x00e;
pub const PCIX0_BIST: u32 = 0x00f;
pub const PCIX0_BAR0L: u32 = 0x010;
pub const PCIX0_BAR0H: u32 = 0x014;
pub const PCIX0_BAR1: u32 = 0x018;
pub const PCIX0_BAR2L: u32 = 0x01c;
pub const PCIX0_BAR2H: u32 = 0x020;
pub const PCIX0_BAR3: u32 = 0x024;
pub const PCIX0_CISPTR: u32 = 0x028;
pub const PCIX0_SBSYSVID: u32 = 0x02c;
pub const PCIX0_SBSYSID: u32 = 0x02e;
pub const PCIX0_EROMBA: u32 = 0x030;
pub const PCIX0_CAP: u32 = 0x034;
pub const PCIX0_RES0: u32 = 0x035;
pub const PCIX0_RES1: u32 = 0x036;
pub const PCIX0_RES2: u32 = 0x038;
pub const PCIX0_INTLN: u32 = 0x03c;
pub const PCIX0_INTPN: u32 = 0x03d;
pub const PCIX0_MINGNT: u32 = 0x03e;
pub const PCIX0_MAXLTNCY: u32 = 0x03f;
pub const PCIX0_BRDGOPT1: u32 = 0x040;
pub const PCIX0_BRDGOPT2: u32 = 0x044;
pub const PCIX0_ERREN: u32 = 0x050;
pub const PCIX0_ERRSTS: u32 = 0x054;
pub const PCIX0_PLBBESR: u32 = 0x058;
pub const PCIX0_PLBBEARL: u32 = 0x05c;
pub const PCIX0_PLBBEARH: u32 = 0x060;
pub const PCIX0_POM0LAL: u32 = 0x068;
pub const PCIX0_POM0LAH: u32 = 0x06c;
pub const PCIX0_POM0SA: u32 = 0x070;
pub const PCIX0_POM0PCIAL: u32 = 0x074;
pub const PCIX0_POM0PCIAH: u32 = 0x078;
pub const PCIX0_POM1LAL: u32 = 0x07c;
pub const PCIX0_POM1LAH: u32 = 0x080;
pub const PCIX0_POM1SA: u32 = 0x084;
pub const PCIX0_POM1PCIAL: u32 = 0x088;
pub const PCIX0_POM1PCIAH: u32 = 0x08c;
pub const PCIX0_POM2SA: u32 = 0x090;
pub const PCIX0_PIM0SAL: u32 = 0x098;
pub const PCIX0_PIM0SA: u32 = PCIX0_PIM0SAL;
pub const PCIX0_PIM0LAL: u32 = 0x09c;
pub const PCIX0_PIM0LAH: u32 = 0x0a0;
pub const PCIX0_PIM1SA: u32 = 0x0a4;
pub const PCIX0_PIM1LAL: u32 = 0x0a8;
pub const PCIX0_PIM1LAH: u32 = 0x0ac;
pub const PCIX0_PIM2SAL: u32 = 0x0b0;
pub const PCIX0_PIM2SA: u32 = PCIX0_PIM2SAL;
pub const PCIX0_PIM2LAL: u32 = 0x0b4;
pub const PCIX0_PIM2LAH: u32 = 0x0b8;
pub const PCIX0_OMCAPID: u32 = 0x0c0;
pub const PCIX0_OMNIPTR: u32 = 0x0c1;
pub const PCIX0_OMMC: u32 = 0x0c2;
pub const PCIX0_OMMA: u32 = 0x0c4;
pub const PCIX0_OMMUA: u32 = 0x0c8;
pub const PCIX0_OMMDATA: u32 = 0x0cc;
pub const PCIX0_OMMEOI: u32 = 0x0ce;
pub const PCIX0_PMCAPID: u32 = 0x0d0;
pub const PCIX0_PMNIPTR: u32 = 0x0d1;
pub const PCIX0_PMC: u32 = 0x0d2;
pub const PCIX0_PMCSR: u32 = 0x0d4;
pub const PCIX0_PMCSRBSE: u32 = 0x0d6;
pub const PCIX0_PMDATA: u32 = 0x0d7;
pub const PCIX0_PMSCRR: u32 = 0x0d8;
pub const PCIX0_CAPID: u32 = 0x0dc;
pub const PCIX0_NIPTR: u32 = 0x0dd;
pub const PCIX0_CMD: u32 = 0x0de;
pub const PCIX0_STS: u32 = 0x0e0;
pub const PCIX0_IDR: u32 = 0x0e4;
pub const PCIX0_CID: u32 = 0x0e8;
pub const PCIX0_RID: u32 = 0x0ec;
pub const PCIX0_PIM0SAH: u32 = 0x0f8;
pub const PCIX0_PIM2SAH: u32 = 0x0fc;
pub const PCIX0_MSGIL: u32 = 0x100;
pub const PCIX0_MSGIH: u32 = 0x104;
pub const PCIX0_MSGOL: u32 = 0x108;
pub const PCIX0_MSGOH: u32 = 0x10c;
pub const PCIX0_IM: u32 = 0x1f8;

/*
 * 4xx PCI bridge register definitions
 */
pub const PCIL0_PMM0LA: u32 = 0x00;
pub const PCIL0_PMM0MA: u32 = 0x04;
pub const PCIL0_PMM0PCILA: u32 = 0x08;
pub const PCIL0_PMM0PCIHA: u32 = 0x0c;
pub const PCIL0_PMM1LA: u32 = 0x10;
pub const PCIL0_PMM1MA: u32 = 0x14;
pub const PCIL0_PMM1PCILA: u32 = 0x18;
pub const PCIL0_PMM1PCIHA: u32 = 0x1c;
pub const PCIL0_PMM2LA: u32 = 0x20;
pub const PCIL0_PMM2MA: u32 = 0x24;
pub const PCIL0_PMM2PCILA: u32 = 0x28;
pub const PCIL0_PMM2PCIHA: u32 = 0x2c;
pub const PCIL0_PTM1MS: u32 = 0x30;
pub const PCIL0_PTM1LA: u32 = 0x34;
pub const PCIL0_PTM2MS: u32 = 0x38;
pub const PCIL0_PTM2LA: u32 = 0x3c;

/*
 * 4xx PCIe bridge register definitions
 */

/* DCR offsets */
pub const DCRO_PEGPL_CFGBAH: u32 = 0x00;
pub const DCRO_PEGPL_CFGBAL: u32 = 0x01;
pub const DCRO_PEGPL_CFGMSK: u32 = 0x02;
pub const DCRO_PEGPL_MSGBAH: u32 = 0x03;
pub const DCRO_PEGPL_MSGBAL: u32 = 0x04;
pub const DCRO_PEGPL_MSGMSK: u32 = 0x05;
pub const DCRO_PEGPL_OMR1BAH: u32 = 0x06;
pub const DCRO_PEGPL_OMR1BAL: u32 = 0x07;
pub const DCRO_PEGPL_OMR1MSKH: u32 = 0x08;
pub const DCRO_PEGPL_OMR1MSKL: u32 = 0x09;
pub const DCRO_PEGPL_OMR2BAH: u32 = 0x0a;
pub const DCRO_PEGPL_OMR2BAL: u32 = 0x0b;
pub const DCRO_PEGPL_OMR2MSKH: u32 = 0x0c;
pub const DCRO_PEGPL_OMR2MSKL: u32 = 0x0d;
pub const DCRO_PEGPL_OMR3BAH: u32 = 0x0e;
pub const DCRO_PEGPL_OMR3BAL: u32 = 0x0f;
pub const DCRO_PEGPL_OMR3MSKH: u32 = 0x10;
pub const DCRO_PEGPL_OMR3MSKL: u32 = 0x11;
pub const DCRO_PEGPL_REGBAH: u32 = 0x12;
pub const DCRO_PEGPL_REGBAL: u32 = 0x13;
pub const DCRO_PEGPL_REGMSK: u32 = 0x14;
pub const DCRO_PEGPL_SPECIAL: u32 = 0x15;
pub const DCRO_PEGPL_CFG: u32 = 0x16;
pub const DCRO_PEGPL_ESR: u32 = 0x17;
pub const DCRO_PEGPL_EARH: u32 = 0x18;
pub const DCRO_PEGPL_EARL: u32 = 0x19;
pub const DCRO_PEGPL_EATR: u32 = 0x1a;

/* DMER mask */
pub const GPL_DMER_MASK_DISA: u32 = 0x02000000;

/*
 * System DCRs (SDRs)
 */
pub const PESDR0_PLLLCT1: u32 = 0x03a0;
pub const PESDR0_PLLLCT2: u32 = 0x03a1;
pub const PESDR0_PLLLCT3: u32 = 0x03a2;

/*
 * 440SPe additional DCRs
 */
pub const PESDR0_440SPE_UTLSET1: u32 = 0x0300;
pub const PESDR0_440SPE_UTLSET2: u32 = 0x0301;
pub const PESDR0_440SPE_DLPSET: u32 = 0x0302;
pub const PESDR0_440SPE_LOOP: u32 = 0x0303;
pub const PESDR0_440SPE_RCSSET: u32 = 0x0304;
pub const PESDR0_440SPE_RCSSTS: u32 = 0x0305;
pub const PESDR0_440SPE_HSSL0SET1: u32 = 0x0306;
pub const PESDR0_440SPE_HSSL0SET2: u32 = 0x0307;
pub const PESDR0_440SPE_HSSL0STS: u32 = 0x0308;
pub const PESDR0_440SPE_HSSL1SET1: u32 = 0x0309;
pub const PESDR0_440SPE_HSSL1SET2: u32 = 0x030a;
pub const PESDR0_440SPE_HSSL1STS: u32 = 0x030b;
pub const PESDR0_440SPE_HSSL2SET1: u32 = 0x030c;
pub const PESDR0_440SPE_HSSL2SET2: u32 = 0x030d;
pub const PESDR0_440SPE_HSSL2STS: u32 = 0x030e;
pub const PESDR0_440SPE_HSSL3SET1: u32 = 0x030f;
pub const PESDR0_440SPE_HSSL3SET2: u32 = 0x0310;
pub const PESDR0_440SPE_HSSL3STS: u32 = 0x0311;
pub const PESDR0_440SPE_HSSL4SET1: u32 = 0x0312;
pub const PESDR0_440SPE_HSSL4SET2: u32 = 0x0313;
pub const PESDR0_440SPE_HSSL4STS: u32 = 0x0314;
pub const PESDR0_440SPE_HSSL5SET1: u32 = 0x0315;
pub const PESDR0_440SPE_HSSL5SET2: u32 = 0x0316;
pub const PESDR0_440SPE_HSSL5STS: u32 = 0x0317;
pub const PESDR0_440SPE_HSSL6SET1: u32 = 0x0318;
pub const PESDR0_440SPE_HSSL6SET2: u32 = 0x0319;
pub const PESDR0_440SPE_HSSL6STS: u32 = 0x031a;
pub const PESDR0_440SPE_HSSL7SET1: u32 = 0x031b;
pub const PESDR0_440SPE_HSSL7SET2: u32 = 0x031c;
pub const PESDR0_440SPE_HSSL7STS: u32 = 0x031d;
pub const PESDR0_440SPE_HSSCTLSET: u32 = 0x031e;
pub const PESDR0_440SPE_LANE_ABCD: u32 = 0x031f;
pub const PESDR0_440SPE_LANE_EFGH: u32 = 0x0320;

pub const PESDR1_440SPE_UTLSET1: u32 = 0x0340;
pub const PESDR1_440SPE_UTLSET2: u32 = 0x0341;
pub const PESDR1_440SPE_DLPSET: u32 = 0x0342;
pub const PESDR1_440SPE_LOOP: u32 = 0x0343;
pub const PESDR1_440SPE_RCSSET: u32 = 0x0344;
pub const PESDR1_440SPE_RCSSTS: u32 = 0x0345;
pub const PESDR1_440SPE_HSSL0SET1: u32 = 0x0346;
pub const PESDR1_440SPE_HSSL0SET2: u32 = 0x0347;
pub const PESDR1_440SPE_HSSL0STS: u32 = 0x0348;
pub const PESDR1_440SPE_HSSL1SET1: u32 = 0x0349;
pub const PESDR1_440SPE_HSSL1SET2: u32 = 0x034a;
pub const PESDR1_440SPE_HSSL1STS: u32 = 0x034b;
pub const PESDR1_440SPE_HSSL2SET1: u32 = 0x034c;
pub const PESDR1_440SPE_HSSL2SET2: u32 = 0x034d;
pub const PESDR1_440SPE_HSSL2STS: u32 = 0x034e;
pub const PESDR1_440SPE_HSSL3SET1: u32 = 0x034f;
pub const PESDR1_440SPE_HSSL3SET2: u32 = 0x0350;
pub const PESDR1_440SPE_HSSL3STS: u32 = 0x0351;
pub const PESDR1_440SPE_HSSCTLSET: u32 = 0x0352;
pub const PESDR1_440SPE_LANE_ABCD: u32 = 0x0353;

pub const PESDR2_440SPE_UTLSET1: u32 = 0x0370;
pub const PESDR2_440SPE_UTLSET2: u32 = 0x0371;
pub const PESDR2_440SPE_DLPSET: u32 = 0x0372;
pub const PESDR2_440SPE_LOOP: u32 = 0x0373;
pub const PESDR2_440SPE_RCSSET: u32 = 0x0374;
pub const PESDR2_440SPE_RCSSTS: u32 = 0x0375;
pub const PESDR2_440SPE_HSSL0SET1: u32 = 0x0376;
pub const PESDR2_440SPE_HSSL0SET2: u32 = 0x0377;
pub const PESDR2_440SPE_HSSL0STS: u32 = 0x0378;
pub const PESDR2_440SPE_HSSL1SET1: u32 = 0x0379;
pub const PESDR2_440SPE_HSSL1SET2: u32 = 0x037a;
pub const PESDR2_440SPE_HSSL1STS: u32 = 0x037b;
pub const PESDR2_440SPE_HSSL2SET1: u32 = 0x037c;
pub const PESDR2_440SPE_HSSL2SET2: u32 = 0x037d;
pub const PESDR2_440SPE_HSSL2STS: u32 = 0x037e;
pub const PESDR2_440SPE_HSSL3SET1: u32 = 0x037f;
pub const PESDR2_440SPE_HSSL3SET2: u32 = 0x0380;
pub const PESDR2_440SPE_HSSL3STS: u32 = 0x0381;
pub const PESDR2_440SPE_HSSCTLSET: u32 = 0x0382;
pub const PESDR2_440SPE_LANE_ABCD: u32 = 0x0383;

/*
 * 405EX additional DCRs
 */
pub const PESDR0_405EX_UTLSET1: u32 = 0x0400;
pub const PESDR0_405EX_UTLSET2: u32 = 0x0401;
pub const PESDR0_405EX_DLPSET: u32 = 0x0402;
pub const PESDR0_405EX_LOOP: u32 = 0x0403;
pub const PESDR0_405EX_RCSSET: u32 = 0x0404;
pub const PESDR0_405EX_RCSSTS: u32 = 0x0405;
pub const PESDR0_405EX_PHYSET1: u32 = 0x0406;
pub const PESDR0_405EX_PHYSET2: u32 = 0x0407;
pub const PESDR0_405EX_BIST: u32 = 0x0408;
pub const PESDR0_405EX_LPB: u32 = 0x040B;
pub const PESDR0_405EX_PHYSTA: u32 = 0x040C;

pub const PESDR1_405EX_UTLSET1: u32 = 0x0440;
pub const PESDR1_405EX_UTLSET2: u32 = 0x0441;
pub const PESDR1_405EX_DLPSET: u32 = 0x0442;
pub const PESDR1_405EX_LOOP: u32 = 0x0443;
pub const PESDR1_405EX_RCSSET: u32 = 0x0444;
pub const PESDR1_405EX_RCSSTS: u32 = 0x0445;
pub const PESDR1_405EX_PHYSET1: u32 = 0x0446;
pub const PESDR1_405EX_PHYSET2: u32 = 0x0447;
pub const PESDR1_405EX_BIST: u32 = 0x0448;
pub const PESDR1_405EX_LPB: u32 = 0x044B;
pub const PESDR1_405EX_PHYSTA: u32 = 0x044C;

/*
 * 460EX additional DCRs
 */
pub const PESDR0_460EX_L0BIST: u32 = 0x0308;
pub const PESDR0_460EX_L0BISTSTS: u32 = 0x0309;
pub const PESDR0_460EX_L0CDRCTL: u32 = 0x030A;
pub const PESDR0_460EX_L0DRV: u32 = 0x030B;
pub const PESDR0_460EX_L0REC: u32 = 0x030C;
pub const PESDR0_460EX_L0LPB: u32 = 0x030D;
pub const PESDR0_460EX_L0CLK: u32 = 0x030E;
pub const PESDR0_460EX_PHY_CTL_RST: u32 = 0x030F;
pub const PESDR0_460EX_RSTSTA: u32 = 0x0310;
pub const PESDR0_460EX_OBS: u32 = 0x0311;
pub const PESDR0_460EX_L0ERRC: u32 = 0x0320;

pub const PESDR1_460EX_L0BIST: u32 = 0x0348;
pub const PESDR1_460EX_L1BIST: u32 = 0x0349;
pub const PESDR1_460EX_L2BIST: u32 = 0x034A;
pub const PESDR1_460EX_L3BIST: u32 = 0x034B;
pub const PESDR1_460EX_L0BISTSTS: u32 = 0x034C;
pub const PESDR1_460EX_L1BISTSTS: u32 = 0x034D;
pub const PESDR1_460EX_L2BISTSTS: u32 = 0x034E;
pub const PESDR1_460EX_L3BISTSTS: u32 = 0x034F;
pub const PESDR1_460EX_L0CDRCTL: u32 = 0x0350;
pub const PESDR1_460EX_L1CDRCTL: u32 = 0x0351;
pub const PESDR1_460EX_L2CDRCTL: u32 = 0x0352;
pub const PESDR1_460EX_L3CDRCTL: u32 = 0x0353;
pub const PESDR1_460EX_L0DRV: u32 = 0x0354;
pub const PESDR1_460EX_L1DRV: u32 = 0x0355;
pub const PESDR1_460EX_L2DRV: u32 = 0x0356;
pub const PESDR1_460EX_L3DRV: u32 = 0x0357;
pub const PESDR1_460EX_L0REC: u32 = 0x0358;
pub const PESDR1_460EX_L1REC: u32 = 0x0359;
pub const PESDR1_460EX_L2REC: u32 = 0x035A;
pub const PESDR1_460EX_L3REC: u32 = 0x035B;
pub const PESDR1_460EX_L0LPB: u32 = 0x035C;
pub const PESDR1_460EX_L1LPB: u32 = 0x035D;
pub const PESDR1_460EX_L2LPB: u32 = 0x035E;
pub const PESDR1_460EX_L3LPB: u32 = 0x035F;
pub const PESDR1_460EX_L0CLK: u32 = 0x0360;
pub const PESDR1_460EX_L1CLK: u32 = 0x0361;
pub const PESDR1_460EX_L2CLK: u32 = 0x0362;
pub const PESDR1_460EX_L3CLK: u32 = 0x0363;
pub const PESDR1_460EX_PHY_CTL_RST: u32 = 0x0364;
pub const PESDR1_460EX_RSTSTA: u32 = 0x0365;
pub const PESDR1_460EX_OBS: u32 = 0x0366;
pub const PESDR1_460EX_L0ERRC: u32 = 0x0368;
pub const PESDR1_460EX_L1ERRC: u32 = 0x0369;
pub const PESDR1_460EX_L2ERRC: u32 = 0x036A;
pub const PESDR1_460EX_L3ERRC: u32 = 0x036B;
pub const PESDR0_460EX_IHS1: u32 = 0x036C;
pub const PESDR0_460EX_IHS2: u32 = 0x036D;

/*
 * 460SX additional DCRs
 */
pub const PESDRn_460SX_RCEI: u32 = 0x02;

pub const PESDR0_460SX_HSSL0DAMP: u32 = 0x320;
pub const PESDR0_460SX_HSSL1DAMP: u32 = 0x321;
pub const PESDR0_460SX_HSSL2DAMP: u32 = 0x322;
pub const PESDR0_460SX_HSSL3DAMP: u32 = 0x323;
pub const PESDR0_460SX_HSSL4DAMP: u32 = 0x324;
pub const PESDR0_460SX_HSSL5DAMP: u32 = 0x325;
pub const PESDR0_460SX_HSSL6DAMP: u32 = 0x326;
pub const PESDR0_460SX_HSSL7DAMP: u32 = 0x327;

pub const PESDR1_460SX_HSSL0DAMP: u32 = 0x354;
pub const PESDR1_460SX_HSSL1DAMP: u32 = 0x355;
pub const PESDR1_460SX_HSSL2DAMP: u32 = 0x356;
pub const PESDR1_460SX_HSSL3DAMP: u32 = 0x357;

pub const PESDR2_460SX_HSSL0DAMP: u32 = 0x384;
pub const PESDR2_460SX_HSSL1DAMP: u32 = 0x385;
pub const PESDR2_460SX_HSSL2DAMP: u32 = 0x386;
pub const PESDR2_460SX_HSSL3DAMP: u32 = 0x387;

pub const PESDR0_460SX_HSSL0COEFA: u32 = 0x328;
pub const PESDR0_460SX_HSSL1COEFA: u32 = 0x329;
pub const PESDR0_460SX_HSSL2COEFA: u32 = 0x32A;
pub const PESDR0_460SX_HSSL3COEFA: u32 = 0x32B;
pub const PESDR0_460SX_HSSL4COEFA: u32 = 0x32C;
pub const PESDR0_460SX_HSSL5COEFA: u32 = 0x32D;
pub const PESDR0_460SX_HSSL6COEFA: u32 = 0x32E;
pub const PESDR0_460SX_HSSL7COEFA: u32 = 0x32F;

pub const PESDR1_460SX_HSSL0COEFA: u32 = 0x358;
pub const PESDR1_460SX_HSSL1COEFA: u32 = 0x359;
pub const PESDR1_460SX_HSSL2COEFA: u32 = 0x35A;
pub const PESDR1_460SX_HSSL3COEFA: u32 = 0x35B;

pub const PESDR2_460SX_HSSL0COEFA: u32 = 0x388;
pub const PESDR2_460SX_HSSL1COEFA: u32 = 0x389;
pub const PESDR2_460SX_HSSL2COEFA: u32 = 0x38A;
pub const PESDR2_460SX_HSSL3COEFA: u32 = 0x38B;

pub const PESDR0_460SX_HSSL1CALDRV: u32 = 0x339;
pub const PESDR1_460SX_HSSL1CALDRV: u32 = 0x361;
pub const PESDR2_460SX_HSSL1CALDRV: u32 = 0x391;

pub const PESDR0_460SX_HSSSLEW: u32 = 0x338;
pub const PESDR1_460SX_HSSSLEW: u32 = 0x360;
pub const PESDR2_460SX_HSSSLEW: u32 = 0x390;

pub const PESDR0_460SX_HSSCTLSET: u32 = 0x31E;
pub const PESDR1_460SX_HSSCTLSET: u32 = 0x352;
pub const PESDR2_460SX_HSSCTLSET: u32 = 0x382;

pub const PESDR0_460SX_RCSSET: u32 = 0x304;
pub const PESDR1_460SX_RCSSET: u32 = 0x344;
pub const PESDR2_460SX_RCSSET: u32 = 0x374;
/*
 * Of the above, some are common offsets from the base
 */
pub const PESDRn_UTLSET1: u32 = 0x00;
pub const PESDRn_UTLSET2: u32 = 0x01;
pub const PESDRn_DLPSET: u32 = 0x02;
pub const PESDRn_LOOP: u32 = 0x03;
pub const PESDRn_RCSSET: u32 = 0x04;
pub const PESDRn_RCSSTS: u32 = 0x05;

/* 440spe only */
pub const PESDRn_440SPE_HSSL0SET1: u32 = 0x06;
pub const PESDRn_440SPE_HSSL0SET2: u32 = 0x07;
pub const PESDRn_440SPE_HSSL0STS: u32 = 0x08;
pub const PESDRn_440SPE_HSSL1SET1: u32 = 0x09;
pub const PESDRn_440SPE_HSSL1SET2: u32 = 0x0a;
pub const PESDRn_440SPE_HSSL1STS: u32 = 0x0b;
pub const PESDRn_440SPE_HSSL2SET1: u32 = 0x0c;
pub const PESDRn_440SPE_HSSL2SET2: u32 = 0x0d;
pub const PESDRn_440SPE_HSSL2STS: u32 = 0x0e;
pub const PESDRn_440SPE_HSSL3SET1: u32 = 0x0f;
pub const PESDRn_440SPE_HSSL3SET2: u32 = 0x10;
pub const PESDRn_440SPE_HSSL3STS: u32 = 0x11;

/* 440spe port 0 only */
pub const PESDRn_440SPE_HSSL4SET1: u32 = 0x12;
pub const PESDRn_440SPE_HSSL4SET2: u32 = 0x13;
pub const PESDRn_440SPE_HSSL4STS: u32 = 0x14;
pub const PESDRn_440SPE_HSSL5SET1: u32 = 0x15;
pub const PESDRn_440SPE_HSSL5SET2: u32 = 0x16;
pub const PESDRn_440SPE_HSSL5STS: u32 = 0x17;
pub const PESDRn_440SPE_HSSL6SET1: u32 = 0x18;
pub const PESDRn_440SPE_HSSL6SET2: u32 = 0x19;
pub const PESDRn_440SPE_HSSL6STS: u32 = 0x1a;
pub const PESDRn_440SPE_HSSL7SET1: u32 = 0x1b;
pub const PESDRn_440SPE_HSSL7SET2: u32 = 0x1c;
pub const PESDRn_440SPE_HSSL7STS: u32 = 0x1d;

/* 405ex only */
pub const PESDRn_405EX_PHYSET1: u32 = 0x06;
pub const PESDRn_405EX_PHYSET2: u32 = 0x07;
pub const PESDRn_405EX_PHYSTA: u32 = 0x0c;

/*
 * UTL register offsets
 */
pub const PEUTL_PBCTL: u32 = 0x00;
pub const PEUTL_PBBSZ: u32 = 0x20;
pub const PEUTL_OPDBSZ: u32 = 0x68;
pub const PEUTL_IPHBSZ: u32 = 0x70;
pub const PEUTL_IPDBSZ: u32 = 0x78;
pub const PEUTL_OUTTR: u32 = 0x90;
pub const PEUTL_INTR: u32 = 0x98;
pub const PEUTL_PCTL: u32 = 0xa0;
pub const PEUTL_RCSTA: u32 = 0xB0;
pub const PEUTL_RCIRQEN: u32 = 0xb8;

/*
 * Config space register offsets
 */
pub const PECFG_ECRTCTL: u32 = 0x074;

pub const PECFG_BAR0LMPA: u32 = 0x210;
pub const PECFG_BAR0HMPA: u32 = 0x214;
pub const PECFG_BAR1MPA: u32 = 0x218;
pub const PECFG_BAR2LMPA: u32 = 0x220;
pub const PECFG_BAR2HMPA: u32 = 0x224;

pub const PECFG_PIMEN: u32 = 0x33c;
pub const PECFG_PIM0LAL: u32 = 0x340;
pub const PECFG_PIM0LAH: u32 = 0x344;
pub const PECFG_PIM1LAL: u32 = 0x348;
pub const PECFG_PIM1LAH: u32 = 0x34c;
pub const PECFG_PIM01SAL: u32 = 0x350;
pub const PECFG_PIM01SAH: u32 = 0x354;

pub const PECFG_POM0LAL: u32 = 0x380;
pub const PECFG_POM0LAH: u32 = 0x384;
pub const PECFG_POM1LAL: u32 = 0x388;
pub const PECFG_POM1LAH: u32 = 0x38c;
pub const PECFG_POM2LAL: u32 = 0x390;
pub const PECFG_POM2LAH: u32 = 0x394;

/* 460sx only */
pub const PECFG_460SX_DLLSTA: u32 = 0x3f8;

/* 460sx Bit Mappings */
pub const PECFG_460SX_DLLSTA_LINKUP: u32 = 0x00000010;
pub const DCRO_PEGPL_460SX_OMR1MSKL_UOT: u32 = 0x00000004;

/* PEGPL Bit Mappings */
pub const DCRO_PEGPL_OMRxMSKL_VAL: u32 = 0x00000001;
pub const DCRO_PEGPL_OMR1MSKL_UOT: u32 = 0x00000002;
pub const DCRO_PEGPL_OMR3MSKL_IO: u32 = 0x00000002;

/* 476FPE */
pub const PCCFG_LCPA: u32 = 0x270;
pub const PECFG_TLDLP: u32 = 0x3F8;
pub const PECFG_TLDLP_LNKUP: u32 = 0x00000008;
pub const PECFG_TLDLP_PRESENT: u32 = 0x00000010;
pub const DCRO_PEGPL_476FPE_OMR1MSKL_UOT: u32 = 0x00000004;

/* SDR Bit Mappings */
pub const PESDRx_RCSSET_HLDPLB: u32 = 0x10000000;
pub const PESDRx_RCSSET_RSTGU: u32 = 0x01000000;
pub const PESDRx_RCSSET_RDY: u32 = 0x00100000;
pub const PESDRx_RCSSET_RSTDL: u32 = 0x00010000;
pub const PESDRx_RCSSET_RSTPYN: u32 = 0x00001000;

const _: () = {
    // C enum constants
    pub const PTYPE_ENDPOINT: u32 = 0x0;
    pub const PTYPE_LEGACY_ENDPOINT: u32 = 0x1;
    pub const PTYPE_ROOT_PORT: u32 = 0x4;

    pub const LNKW_X1: u32 = 0x1;
    pub const LNKW_X4: u32 = 0x4;
    pub const LNKW_X8: u32 = 0x8;
};



// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
