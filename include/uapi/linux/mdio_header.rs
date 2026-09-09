// Translated from linux/mdio.h. External symbols such as MII_*, BMCR_*, BMSR_*, ADVERTISE_*, LPA_*, BIT, and GENMASK are supplied by other headers.

/* SPDX-License-Identifier: GPL-2.0 WITH Linux-syscall-note */
/*
 * linux/mdio.h: definitions for MDIO (clause 45) transceivers
 * Copyright 2006-2009 Solarflare Communications Inc.
 *
 * This program is free software; you can redistribute it and/or modify it
 * under the terms of the GNU General Public License version 2 as published
 * by the Free Software Foundation, incorporated herein by reference.
 */


// #include <linux/types.h>
// #include <linux/mii.h>

/* MDIO Manageable Devices (MMDs). */
pub const MDIO_MMD_PMAPMD: u32 = 1	/* Physical Medium Attachment/;
					 * Physical Medium Dependent */
pub const MDIO_MMD_WIS: u32 = 2	/* WAN Interface Sublayer */;
pub const MDIO_MMD_PCS: u32 = 3	/* Physical Coding Sublayer */;
pub const MDIO_MMD_PHYXS: u32 = 4	/* PHY Extender Sublayer */;
pub const MDIO_MMD_DTEXS: u32 = 5	/* DTE Extender Sublayer */;
pub const MDIO_MMD_TC: u32 = 6	/* Transmission Convergence */;
pub const MDIO_MMD_AN: u32 = 7	/* Auto-Negotiation */;
pub const MDIO_MMD_SEP_PMA1: u32 = 8	/* Separated PMA (1) */;
pub const MDIO_MMD_SEP_PMA2: u32 = 9	/* Separated PMA (2) */;
pub const MDIO_MMD_SEP_PMA3: u32 = 10	/* Separated PMA (3) */;
pub const MDIO_MMD_SEP_PMA4: u32 = 11	/* Separated PMA (4) */;
pub const MDIO_MMD_POWER_UNIT: u32 = 13	/* PHY Power Unit */;
pub const MDIO_MMD_C22EXT: u32 = 29	/* Clause 22 extension */;
pub const MDIO_MMD_VEND1: u32 = 30	/* Vendor specific 1 */;
pub const MDIO_MMD_VEND2: u32 = 31	/* Vendor specific 2 */;

/* Generic MDIO registers. */
pub const MDIO_CTRL1: u32 = MII_BMCR;
pub const MDIO_STAT1: u32 = MII_BMSR;
pub const MDIO_DEVID1: u32 = MII_PHYSID1;
pub const MDIO_DEVID2: u32 = MII_PHYSID2;
pub const MDIO_SPEED: u32 = 4	/* Speed ability */;
pub const MDIO_DEVS1: u32 = 5	/* Devices in package */;
pub const MDIO_DEVS2: u32 = 6;
pub const MDIO_CTRL2: u32 = 7	/* 10G control 2 */;
pub const MDIO_STAT2: u32 = 8	/* 10G status 2 */;
pub const MDIO_PMA_TXDIS: u32 = 9	/* 10G PMA/PMD transmit disable */;
pub const MDIO_PMA_RXDET: u32 = 10	/* 10G PMA/PMD receive signal detect */;
pub const MDIO_PMA_EXTABLE: u32 = 11	/* 10G PMA/PMD extended ability */;
pub const MDIO_PKGID1: u32 = 14	/* Package identifier */;
pub const MDIO_PKGID2: u32 = 15;
pub const MDIO_AN_ADVERTISE: u32 = 16	/* AN advertising (base page) */;
pub const MDIO_AN_LPA: u32 = 19	/* AN LP abilities (base page) */;
pub const MDIO_PCS_EEE_ABLE: u32 = 20	/* EEE Capability register */;
pub const MDIO_PCS_EEE_ABLE2: u32 = 21	/* EEE Capability register 2 */;
pub const MDIO_PMA_NG_EXTABLE: u32 = 21	/* 2.5G/5G PMA/PMD extended ability */;
pub const MDIO_PCS_EEE_WK_ERR: u32 = 22	/* EEE wake error counter */;
pub const MDIO_PHYXS_LNSTAT: u32 = 24	/* PHY XGXS lane state */;
pub const MDIO_AN_EEE_ADV: u32 = 60	/* EEE advertisement */;
pub const MDIO_AN_EEE_LPABLE: u32 = 61	/* EEE link partner ability */;
pub const MDIO_AN_EEE_ADV2: u32 = 62	/* EEE advertisement 2 */;
pub const MDIO_AN_EEE_LPABLE2: u32 = 63	/* EEE link partner ability 2 */;
pub const MDIO_AN_CTRL2: u32 = 64	/* AN THP bypass request control */;

/* Media-dependent registers. */
pub const MDIO_PMA_10GBT_SWAPPOL: u32 = 130	/* 10GBASE-T pair swap & polarity */;
pub const MDIO_PMA_10GBT_TXPWR: u32 = 131	/* 10GBASE-T TX power control */;
pub const MDIO_PMA_10GBT_SNR: u32 = 133	/* 10GBASE-T SNR margin, lane A.;
					 * Lanes B-D are numbered 134-136. */
pub const MDIO_PMA_10GBR_FSRT_CSR: u32 = 147	/* 10GBASE-R fast retrain status and control */;
pub const MDIO_PMA_10GBR_FECABLE: u32 = 170	/* 10GBASE-R FEC ability */;
pub const MDIO_PMA_RSFEC_CTRL: u32 = 200	/* RSFEC control */;
pub const MDIO_PMA_RSFEC_LANE_MAP: u32 = 206	/* RSFEC lane mapping */;
pub const MDIO_PCS_10GBX_STAT1: u32 = 24	/* 10GBASE-X PCS status 1 */;
pub const MDIO_PCS_10GBRT_STAT1: u32 = 32	/* 10GBASE-R/-T PCS status 1 */;
pub const MDIO_PCS_10GBRT_STAT2: u32 = 33	/* 10GBASE-R/-T PCS status 2 */;
pub const MDIO_AN_10GBT_CTRL: u32 = 32	/* 10GBASE-T auto-negotiation control */;
pub const MDIO_AN_10GBT_STAT: u32 = 33	/* 10GBASE-T auto-negotiation status */;
pub const MDIO_B10L_PMA_CTRL: u32 = 2294	/* 10BASE-T1L PMA control */;
pub const MDIO_PMA_10T1L_STAT: u32 = 2295	/* 10BASE-T1L PMA status */;
pub const MDIO_PCS_10T1L_CTRL: u32 = 2278	/* 10BASE-T1L PCS control */;
pub const MDIO_PMA_PMD_BT1: u32 = 18	/* BASE-T1 PMA/PMD extended ability */;
pub const MDIO_AN_T1_CTRL: u32 = 512	/* BASE-T1 AN control */;
pub const MDIO_AN_T1_STAT: u32 = 513	/* BASE-T1 AN status */;
pub const MDIO_AN_T1_ADV_L: u32 = 514	/* BASE-T1 AN advertisement register [15:0] */;
pub const MDIO_AN_T1_ADV_M: u32 = 515	/* BASE-T1 AN advertisement register [31:16] */;
pub const MDIO_AN_T1_ADV_H: u32 = 516	/* BASE-T1 AN advertisement register [47:32] */;
pub const MDIO_AN_T1_LP_L: u32 = 517	/* BASE-T1 AN LP Base Page ability register [15:0] */;
pub const MDIO_AN_T1_LP_M: u32 = 518	/* BASE-T1 AN LP Base Page ability register [31:16] */;
pub const MDIO_AN_T1_LP_H: u32 = 519	/* BASE-T1 AN LP Base Page ability register [47:32] */;
pub const MDIO_AN_10BT1_AN_CTRL: u32 = 526	/* 10BASE-T1 AN control register */;
pub const MDIO_AN_10BT1_AN_STAT: u32 = 527	/* 10BASE-T1 AN status register */;
pub const MDIO_PMA_PMD_BT1_CTRL: u32 = 2100	/* BASE-T1 PMA/PMD control register */;
pub const MDIO_PCS_1000BT1_CTRL: u32 = 2304	/* 1000BASE-T1 PCS control register */;
pub const MDIO_PCS_1000BT1_STAT: u32 = 2305	/* 1000BASE-T1 PCS status register */;

/* LASI (Link Alarm Status Interrupt) registers, defined by XENPAK MSA. */
pub const MDIO_PMA_LASI_RXCTRL: u32 = 0x9000	/* RX_ALARM control */;
pub const MDIO_PMA_LASI_TXCTRL: u32 = 0x9001	/* TX_ALARM control */;
pub const MDIO_PMA_LASI_CTRL: u32 = 0x9002	/* LASI control */;
pub const MDIO_PMA_LASI_RXSTAT: u32 = 0x9003	/* RX_ALARM status */;
pub const MDIO_PMA_LASI_TXSTAT: u32 = 0x9004	/* TX_ALARM status */;
pub const MDIO_PMA_LASI_STAT: u32 = 0x9005	/* LASI status */;

/* Control register 1. */
/* Enable extended speed selection */
pub const MDIO_CTRL1_SPEEDSELEXT: u32 = (BMCR_SPEED1000 | BMCR_SPEED100);
/* All speed selection bits */
pub const MDIO_CTRL1_SPEEDSEL: u32 = (MDIO_CTRL1_SPEEDSELEXT | 0x003c);
pub const MDIO_CTRL1_FULLDPLX: u32 = BMCR_FULLDPLX;
pub const MDIO_CTRL1_LPOWER: u32 = BMCR_PDOWN;
pub const MDIO_CTRL1_RESET: u32 = BMCR_RESET;
pub const MDIO_PMA_CTRL1_LOOPBACK: u32 = 0x0001;
pub const MDIO_PMA_CTRL1_SPEED1000: u32 = BMCR_SPEED1000;
pub const MDIO_PMA_CTRL1_SPEED100: u32 = BMCR_SPEED100;
pub const MDIO_PCS_CTRL1_LOOPBACK: u32 = BMCR_LOOPBACK;
pub const MDIO_PHYXS_CTRL1_LOOPBACK: u32 = BMCR_LOOPBACK;
pub const MDIO_AN_CTRL1_RESTART: u32 = BMCR_ANRESTART;
pub const MDIO_AN_CTRL1_ENABLE: u32 = BMCR_ANENABLE;
pub const MDIO_AN_CTRL1_XNP: u32 = 0x2000	/* Enable extended next page */;
pub const MDIO_PCS_CTRL1_CLKSTOP_EN: u32 = 0x400	/* Stop the clock during LPI */;

/* 10 Gb/s */
pub const MDIO_CTRL1_SPEED10G: u32 = (MDIO_CTRL1_SPEEDSELEXT | 0x00);
/* 10PASS-TS/2BASE-TL */
pub const MDIO_CTRL1_SPEED10P2B: u32 = (MDIO_CTRL1_SPEEDSELEXT | 0x04);
/* Note: the MDIO_CTRL1_SPEED_XXX values for everything past 10PASS-TS/2BASE-TL
 * do not match between the PCS and PMA values. Any additions past this point
 * should be PMA or PCS specific. The following 2 defines are workarounds for
 * values added before this was caught. They should be considered deprecated.
 */
pub const MDIO_CTRL1_SPEED2_5G: u32 = MDIO_PMA_CTRL1_SPEED2_5G;
pub const MDIO_CTRL1_SPEED5G: u32 = MDIO_PMA_CTRL1_SPEED5G;
/* 100 Gb/s */
pub const MDIO_PCS_CTRL1_SPEED100G: u32 = (MDIO_CTRL1_SPEEDSELEXT | 0x10);
/* 25 Gb/s */
pub const MDIO_PCS_CTRL1_SPEED25G: u32 = (MDIO_CTRL1_SPEEDSELEXT | 0x14);
/* 50 Gb/s */
pub const MDIO_PCS_CTRL1_SPEED50G: u32 = (MDIO_CTRL1_SPEEDSELEXT | 0x18);
/* 2.5 Gb/s */
pub const MDIO_PMA_CTRL1_SPEED2_5G: u32 = (MDIO_CTRL1_SPEEDSELEXT | 0x18);
/* 5 Gb/s */
pub const MDIO_PMA_CTRL1_SPEED5G: u32 = (MDIO_CTRL1_SPEEDSELEXT | 0x1c);


/* Status register 1. */
pub const MDIO_STAT1_LPOWERABLE: u32 = 0x0002	/* Low-power ability */;
pub const MDIO_STAT1_LSTATUS: u32 = BMSR_LSTATUS;
pub const MDIO_STAT1_FAULT: u32 = 0x0080	/* Fault */;
pub const MDIO_PCS_STAT1_CLKSTOP_CAP: u32 = 0x0040;
pub const MDIO_AN_STAT1_LPABLE: u32 = 0x0001	/* Link partner AN ability */;
pub const MDIO_AN_STAT1_ABLE: u32 = BMSR_ANEGCAPABLE;
pub const MDIO_AN_STAT1_RFAULT: u32 = BMSR_RFAULT;
pub const MDIO_AN_STAT1_COMPLETE: u32 = BMSR_ANEGCOMPLETE;
pub const MDIO_AN_STAT1_PAGE: u32 = 0x0040	/* Page received */;
pub const MDIO_AN_STAT1_XNP: u32 = 0x0080	/* Extended next page status */;

/* Device Identifier 2 */
pub const MDIO_DEVID2_OUI: u32 = 0xfc00	/* OUI Portion of PHY ID */;
pub const MDIO_DEVID2_MODEL_NUM: u32 = 0x03f0	/* Manufacturer's Model Number */;
pub const MDIO_DEVID2_REV_NUM: u32 = 0x000f	/* Revision Number */;

/* Speed register. */
pub const MDIO_SPEED_10G: u32 = 0x0001	/* 10G capable */;
pub const MDIO_PMA_SPEED_2B: u32 = 0x0002	/* 2BASE-TL capable */;
pub const MDIO_PMA_SPEED_10P: u32 = 0x0004	/* 10PASS-TS capable */;
pub const MDIO_PMA_SPEED_1000: u32 = 0x0010	/* 1000M capable */;
pub const MDIO_PMA_SPEED_100: u32 = 0x0020	/* 100M capable */;
pub const MDIO_PMA_SPEED_10: u32 = 0x0040	/* 10M capable */;
pub const MDIO_PMA_SPEED_2_5G: u32 = 0x2000	/* 2.5G capable */;
pub const MDIO_PMA_SPEED_5G: u32 = 0x4000	/* 5G capable */;
pub const MDIO_PCS_SPEED_10P2B: u32 = 0x0002	/* 10PASS-TS/2BASE-TL capable */;
pub const MDIO_PCS_SPEED_2_5G: u32 = 0x0040	/* 2.5G capable */;
pub const MDIO_PCS_SPEED_5G: u32 = 0x0080	/* 5G capable */;

/* Device present registers. */
pub const fn MDIO_DEVS_PRESENT(devad: u32) -> u32 { 1u32 << devad }
pub const MDIO_DEVS_C22PRESENT: u32 = MDIO_DEVS_PRESENT(0);
pub const MDIO_DEVS_PMAPMD: u32 = MDIO_DEVS_PRESENT(MDIO_MMD_PMAPMD);
pub const MDIO_DEVS_WIS: u32 = MDIO_DEVS_PRESENT(MDIO_MMD_WIS);
pub const MDIO_DEVS_PCS: u32 = MDIO_DEVS_PRESENT(MDIO_MMD_PCS);
pub const MDIO_DEVS_PHYXS: u32 = MDIO_DEVS_PRESENT(MDIO_MMD_PHYXS);
pub const MDIO_DEVS_DTEXS: u32 = MDIO_DEVS_PRESENT(MDIO_MMD_DTEXS);
pub const MDIO_DEVS_TC: u32 = MDIO_DEVS_PRESENT(MDIO_MMD_TC);
pub const MDIO_DEVS_AN: u32 = MDIO_DEVS_PRESENT(MDIO_MMD_AN);
pub const MDIO_DEVS_SEP_PMA1: u32 = MDIO_DEVS_PRESENT(MDIO_MMD_SEP_PMA1);
pub const MDIO_DEVS_SEP_PMA2: u32 = MDIO_DEVS_PRESENT(MDIO_MMD_SEP_PMA2);
pub const MDIO_DEVS_SEP_PMA3: u32 = MDIO_DEVS_PRESENT(MDIO_MMD_SEP_PMA3);
pub const MDIO_DEVS_SEP_PMA4: u32 = MDIO_DEVS_PRESENT(MDIO_MMD_SEP_PMA4);
pub const MDIO_DEVS_C22EXT: u32 = MDIO_DEVS_PRESENT(MDIO_MMD_C22EXT);
pub const MDIO_DEVS_VEND1: u32 = MDIO_DEVS_PRESENT(MDIO_MMD_VEND1);
pub const MDIO_DEVS_VEND2: u32 = MDIO_DEVS_PRESENT(MDIO_MMD_VEND2);

/* Control register 2. */
pub const MDIO_PMA_CTRL2_TYPE: u32 = 0x000f	/* PMA/PMD type selection */;
pub const MDIO_PMA_CTRL2_10GBCX4: u32 = 0x0000	/* 10GBASE-CX4 type */;
pub const MDIO_PMA_CTRL2_10GBEW: u32 = 0x0001	/* 10GBASE-EW type */;
pub const MDIO_PMA_CTRL2_10GBLW: u32 = 0x0002	/* 10GBASE-LW type */;
pub const MDIO_PMA_CTRL2_10GBSW: u32 = 0x0003	/* 10GBASE-SW type */;
pub const MDIO_PMA_CTRL2_10GBLX4: u32 = 0x0004	/* 10GBASE-LX4 type */;
pub const MDIO_PMA_CTRL2_10GBER: u32 = 0x0005	/* 10GBASE-ER type */;
pub const MDIO_PMA_CTRL2_10GBLR: u32 = 0x0006	/* 10GBASE-LR type */;
pub const MDIO_PMA_CTRL2_10GBSR: u32 = 0x0007	/* 10GBASE-SR type */;
pub const MDIO_PMA_CTRL2_10GBLRM: u32 = 0x0008	/* 10GBASE-LRM type */;
pub const MDIO_PMA_CTRL2_10GBT: u32 = 0x0009	/* 10GBASE-T type */;
pub const MDIO_PMA_CTRL2_10GBKX4: u32 = 0x000a	/* 10GBASE-KX4 type */;
pub const MDIO_PMA_CTRL2_10GBKR: u32 = 0x000b	/* 10GBASE-KR type */;
pub const MDIO_PMA_CTRL2_1000BT: u32 = 0x000c	/* 1000BASE-T type */;
pub const MDIO_PMA_CTRL2_1000BKX: u32 = 0x000d	/* 1000BASE-KX type */;
pub const MDIO_PMA_CTRL2_100BTX: u32 = 0x000e	/* 100BASE-TX type */;
pub const MDIO_PMA_CTRL2_10BT: u32 = 0x000f	/* 10BASE-T type */;
pub const MDIO_PMA_CTRL2_2_5GBT: u32 = 0x0030  /* 2.5GBaseT type */;
pub const MDIO_PMA_CTRL2_5GBT: u32 = 0x0031  /* 5GBaseT type */;
pub const MDIO_PMA_CTRL2_BASET1: u32 = 0x003D  /* BASE-T1 type */;
pub const MDIO_PCS_CTRL2_TYPE: u32 = 0x0003	/* PCS type selection */;
pub const MDIO_PCS_CTRL2_10GBR: u32 = 0x0000	/* 10GBASE-R type */;
pub const MDIO_PCS_CTRL2_10GBX: u32 = 0x0001	/* 10GBASE-X type */;
pub const MDIO_PCS_CTRL2_10GBW: u32 = 0x0002	/* 10GBASE-W type */;
pub const MDIO_PCS_CTRL2_10GBT: u32 = 0x0003	/* 10GBASE-T type */;

/* Status register 2. */
pub const MDIO_STAT2_RXFAULT: u32 = 0x0400	/* Receive fault */;
pub const MDIO_STAT2_TXFAULT: u32 = 0x0800	/* Transmit fault */;
pub const MDIO_STAT2_DEVPRST: u32 = 0xc000	/* Device present */;
pub const MDIO_STAT2_DEVPRST_VAL: u32 = 0x8000	/* Device present value */;
pub const MDIO_PMA_STAT2_LBABLE: u32 = 0x0001	/* PMA loopback ability */;
pub const MDIO_PMA_STAT2_10GBEW: u32 = 0x0002	/* 10GBASE-EW ability */;
pub const MDIO_PMA_STAT2_10GBLW: u32 = 0x0004	/* 10GBASE-LW ability */;
pub const MDIO_PMA_STAT2_10GBSW: u32 = 0x0008	/* 10GBASE-SW ability */;
pub const MDIO_PMA_STAT2_10GBLX4: u32 = 0x0010	/* 10GBASE-LX4 ability */;
pub const MDIO_PMA_STAT2_10GBER: u32 = 0x0020	/* 10GBASE-ER ability */;
pub const MDIO_PMA_STAT2_10GBLR: u32 = 0x0040	/* 10GBASE-LR ability */;
pub const MDIO_PMA_STAT2_10GBSR: u32 = 0x0080	/* 10GBASE-SR ability */;
pub const MDIO_PMD_STAT2_TXDISAB: u32 = 0x0100	/* PMD TX disable ability */;
pub const MDIO_PMA_STAT2_EXTABLE: u32 = 0x0200	/* Extended abilities */;
pub const MDIO_PMA_STAT2_RXFLTABLE: u32 = 0x1000	/* Receive fault ability */;
pub const MDIO_PMA_STAT2_TXFLTABLE: u32 = 0x2000	/* Transmit fault ability */;
pub const MDIO_PCS_STAT2_10GBR: u32 = 0x0001	/* 10GBASE-R capable */;
pub const MDIO_PCS_STAT2_10GBX: u32 = 0x0002	/* 10GBASE-X capable */;
pub const MDIO_PCS_STAT2_10GBW: u32 = 0x0004	/* 10GBASE-W capable */;
pub const MDIO_PCS_STAT2_RXFLTABLE: u32 = 0x1000	/* Receive fault ability */;
pub const MDIO_PCS_STAT2_TXFLTABLE: u32 = 0x2000	/* Transmit fault ability */;

/* Transmit disable register. */
pub const MDIO_PMD_TXDIS_GLOBAL: u32 = 0x0001	/* Global PMD TX disable */;
pub const MDIO_PMD_TXDIS_0: u32 = 0x0002	/* PMD TX disable 0 */;
pub const MDIO_PMD_TXDIS_1: u32 = 0x0004	/* PMD TX disable 1 */;
pub const MDIO_PMD_TXDIS_2: u32 = 0x0008	/* PMD TX disable 2 */;
pub const MDIO_PMD_TXDIS_3: u32 = 0x0010	/* PMD TX disable 3 */;

/* Receive signal detect register. */
pub const MDIO_PMD_RXDET_GLOBAL: u32 = 0x0001	/* Global PMD RX signal detect */;
pub const MDIO_PMD_RXDET_0: u32 = 0x0002	/* PMD RX signal detect 0 */;
pub const MDIO_PMD_RXDET_1: u32 = 0x0004	/* PMD RX signal detect 1 */;
pub const MDIO_PMD_RXDET_2: u32 = 0x0008	/* PMD RX signal detect 2 */;
pub const MDIO_PMD_RXDET_3: u32 = 0x0010	/* PMD RX signal detect 3 */;

/* Extended abilities register. */
pub const MDIO_PMA_EXTABLE_10GCX4: u32 = 0x0001	/* 10GBASE-CX4 ability */;
pub const MDIO_PMA_EXTABLE_10GBLRM: u32 = 0x0002	/* 10GBASE-LRM ability */;
pub const MDIO_PMA_EXTABLE_10GBT: u32 = 0x0004	/* 10GBASE-T ability */;
pub const MDIO_PMA_EXTABLE_10GBKX4: u32 = 0x0008	/* 10GBASE-KX4 ability */;
pub const MDIO_PMA_EXTABLE_10GBKR: u32 = 0x0010	/* 10GBASE-KR ability */;
pub const MDIO_PMA_EXTABLE_1000BT: u32 = 0x0020	/* 1000BASE-T ability */;
pub const MDIO_PMA_EXTABLE_1000BKX: u32 = 0x0040	/* 1000BASE-KX ability */;
pub const MDIO_PMA_EXTABLE_100BTX: u32 = 0x0080	/* 100BASE-TX ability */;
pub const MDIO_PMA_EXTABLE_10BT: u32 = 0x0100	/* 10BASE-T ability */;
pub const MDIO_PMA_EXTABLE_BT1: u32 = 0x0800	/* BASE-T1 ability */;
pub const MDIO_PMA_EXTABLE_NBT: u32 = 0x4000  /* 2.5/5GBASE-T ability */;

/* AN Clause 73 linkword */
pub const MDIO_AN_C73_0_S_MASK: u32 = GENMASK(4, 0);
pub const MDIO_AN_C73_0_E_MASK: u32 = GENMASK(9, 5);
pub const MDIO_AN_C73_0_PAUSE: u32 = BIT(10);
pub const MDIO_AN_C73_0_ASM_DIR: u32 = BIT(11);
pub const MDIO_AN_C73_0_C2: u32 = BIT(12);
pub const MDIO_AN_C73_0_RF: u32 = BIT(13);
pub const MDIO_AN_C73_0_ACK: u32 = BIT(14);
pub const MDIO_AN_C73_0_NP: u32 = BIT(15);
pub const MDIO_AN_C73_1_T_MASK: u32 = GENMASK(4, 0);
pub const MDIO_AN_C73_1_1000BASE_KX: u32 = BIT(5);
pub const MDIO_AN_C73_1_10GBASE_KX4: u32 = BIT(6);
pub const MDIO_AN_C73_1_10GBASE_KR: u32 = BIT(7);
pub const MDIO_AN_C73_1_40GBASE_KR4: u32 = BIT(8);
pub const MDIO_AN_C73_1_40GBASE_CR4: u32 = BIT(9);
pub const MDIO_AN_C73_1_100GBASE_CR10: u32 = BIT(10);
pub const MDIO_AN_C73_1_100GBASE_KP4: u32 = BIT(11);
pub const MDIO_AN_C73_1_100GBASE_KR4: u32 = BIT(12);
pub const MDIO_AN_C73_1_100GBASE_CR4: u32 = BIT(13);
pub const MDIO_AN_C73_1_25GBASE_R_S: u32 = BIT(14);
pub const MDIO_AN_C73_1_25GBASE_R: u32 = BIT(15);
pub const MDIO_AN_C73_2_2500BASE_KX: u32 = BIT(0);
pub const MDIO_AN_C73_2_5GBASE_KR: u32 = BIT(1);

/* PHY XGXS lane state register. */
pub const MDIO_PHYXS_LNSTAT_SYNC0: u32 = 0x0001;
pub const MDIO_PHYXS_LNSTAT_SYNC1: u32 = 0x0002;
pub const MDIO_PHYXS_LNSTAT_SYNC2: u32 = 0x0004;
pub const MDIO_PHYXS_LNSTAT_SYNC3: u32 = 0x0008;
pub const MDIO_PHYXS_LNSTAT_ALIGN: u32 = 0x1000;

/* PMA 10GBASE-T pair swap & polarity */
pub const MDIO_PMA_10GBT_SWAPPOL_ABNX: u32 = 0x0001	/* Pair A/B uncrossed */;
pub const MDIO_PMA_10GBT_SWAPPOL_CDNX: u32 = 0x0002	/* Pair C/D uncrossed */;
pub const MDIO_PMA_10GBT_SWAPPOL_AREV: u32 = 0x0100	/* Pair A polarity reversed */;
pub const MDIO_PMA_10GBT_SWAPPOL_BREV: u32 = 0x0200	/* Pair B polarity reversed */;
pub const MDIO_PMA_10GBT_SWAPPOL_CREV: u32 = 0x0400	/* Pair C polarity reversed */;
pub const MDIO_PMA_10GBT_SWAPPOL_DREV: u32 = 0x0800	/* Pair D polarity reversed */;

/* PMA 10GBASE-T TX power register. */
pub const MDIO_PMA_10GBT_TXPWR_SHORT: u32 = 0x0001	/* Short-reach mode */;

/* PMA 10GBASE-T SNR registers. */
/* Value is SNR margin in dB, clamped to range [-127, 127], plus 0x8000. */
pub const MDIO_PMA_10GBT_SNR_BIAS: u32 = 0x8000;
pub const MDIO_PMA_10GBT_SNR_MAX: u32 = 127;

/* PMA 10GBASE-R FEC ability register. */
pub const MDIO_PMA_10GBR_FECABLE_ABLE: u32 = 0x0001	/* FEC ability */;
pub const MDIO_PMA_10GBR_FECABLE_ERRABLE: u32 = 0x0002	/* FEC error indic. ability */;

/* PMA 10GBASE-R Fast Retrain status and control register. */
pub const MDIO_PMA_10GBR_FSRT_ENABLE: u32 = 0x0001	/* Fast retrain enable */;

/* PCS 10GBASE-R/-T status register 1. */
pub const MDIO_PCS_10GBRT_STAT1_BLKLK: u32 = 0x0001	/* Block lock attained */;

/* PCS 10GBASE-R/-T status register 2. */
pub const MDIO_PCS_10GBRT_STAT2_ERR: u32 = 0x00ff;
pub const MDIO_PCS_10GBRT_STAT2_BER: u32 = 0x3f00;

/* AN 10GBASE-T control register. */
pub const MDIO_AN_10GBT_CTRL_ADVFSRT2_5G: u32 = 0x0020	/* Advertise 2.5GBASE-T fast retrain */;
pub const MDIO_AN_10GBT_CTRL_ADV2_5G: u32 = 0x0080	/* Advertise 2.5GBASE-T */;
pub const MDIO_AN_10GBT_CTRL_ADV5G: u32 = 0x0100	/* Advertise 5GBASE-T */;
pub const MDIO_AN_10GBT_CTRL_ADV10G: u32 = 0x1000	/* Advertise 10GBASE-T */;
pub const MDIO_AN_10GBT_CTRL_MS_ENABLE: u32 = 0x8000	/* Master/slave manual config enable */;
pub const MDIO_AN_10GBT_CTRL_MS_VALUE: u32 = 0x4000	/* Master/slave config value (1=Master) */;
pub const MDIO_AN_10GBT_CTRL_MS_PORT_TYPE: u32 = 0x2000	/* Master Preferred Type */;

/* AN 10GBASE-T status register. */
pub const MDIO_AN_10GBT_STAT_MS_FAULT: u32 = 0x8000	/* Master/slave fault */;
pub const MDIO_AN_10GBT_STAT_MS_RES: u32 = 0x4000	/* Master/slave resolution (1=Master) */;
pub const MDIO_AN_10GBT_STAT_LP2_5G: u32 = 0x0020  /* LP is 2.5GBT capable */;
pub const MDIO_AN_10GBT_STAT_LP5G: u32 = 0x0040  /* LP is 5GBT capable */;
pub const MDIO_AN_10GBT_STAT_LPTRR: u32 = 0x0200	/* LP training reset req. */;
pub const MDIO_AN_10GBT_STAT_LPLTABLE: u32 = 0x0400	/* LP loop timing ability */;
pub const MDIO_AN_10GBT_STAT_LP10G: u32 = 0x0800	/* LP is 10GBT capable */;
pub const MDIO_AN_10GBT_STAT_REMOK: u32 = 0x1000	/* Remote OK */;
pub const MDIO_AN_10GBT_STAT_LOCOK: u32 = 0x2000	/* Local OK */;
pub const MDIO_AN_10GBT_STAT_MS: u32 = 0x4000	/* Master/slave config */;
pub const MDIO_AN_10GBT_STAT_MSFLT: u32 = 0x8000	/* Master/slave config fault */;

/* 10BASE-T1L PMA control */
pub const MDIO_PMA_10T1L_CTRL_LB_EN: u32 = 0x0001	/* Enable loopback mode */;
pub const MDIO_PMA_10T1L_CTRL_EEE_EN: u32 = 0x0400	/* Enable EEE mode */;
pub const MDIO_PMA_10T1L_CTRL_LOW_POWER: u32 = 0x0800	/* Low-power mode */;
pub const MDIO_PMA_10T1L_CTRL_2V4_EN: u32 = 0x1000	/* Enable 2.4 Vpp operating mode */;
pub const MDIO_PMA_10T1L_CTRL_TX_DIS: u32 = 0x4000	/* Transmit disable */;
pub const MDIO_PMA_10T1L_CTRL_PMA_RST: u32 = 0x8000	/* MA reset */;

/* 10BASE-T1L PMA status register. */
pub const MDIO_PMA_10T1L_STAT_LINK: u32 = 0x0001	/* PMA receive link up */;
pub const MDIO_PMA_10T1L_STAT_FAULT: u32 = 0x0002	/* Fault condition detected */;
pub const MDIO_PMA_10T1L_STAT_POLARITY: u32 = 0x0004	/* Receive polarity is reversed */;
pub const MDIO_PMA_10T1L_STAT_RECV_FAULT: u32 = 0x0200	/* Able to detect fault on receive path */;
pub const MDIO_PMA_10T1L_STAT_EEE: u32 = 0x0400	/* PHY has EEE ability */;
pub const MDIO_PMA_10T1L_STAT_LOW_POWER: u32 = 0x0800	/* PMA has low-power ability */;
pub const MDIO_PMA_10T1L_STAT_2V4_ABLE: u32 = 0x1000	/* PHY has 2.4 Vpp operating mode ability */;
pub const MDIO_PMA_10T1L_STAT_LB_ABLE: u32 = 0x2000	/* PHY has loopback ability */;

/* 10BASE-T1L PCS control register. */
pub const MDIO_PCS_10T1L_CTRL_LB: u32 = 0x4000	/* Enable PCS level loopback mode */;
pub const MDIO_PCS_10T1L_CTRL_RESET: u32 = 0x8000	/* PCS reset */;

/* BASE-T1 PMA/PMD extended ability register. */
pub const MDIO_PMA_PMD_BT1_B100_ABLE: u32 = 0x0001	/* 100BASE-T1 Ability */;
pub const MDIO_PMA_PMD_BT1_B1000_ABLE: u32 = 0x0002	/* 1000BASE-T1 Ability */;
pub const MDIO_PMA_PMD_BT1_B10L_ABLE: u32 = 0x0004	/* 10BASE-T1L Ability */;

/* BASE-T1 auto-negotiation advertisement register [15:0] */
pub const MDIO_AN_T1_ADV_L_PAUSE_CAP: u32 = ADVERTISE_PAUSE_CAP;
pub const MDIO_AN_T1_ADV_L_PAUSE_ASYM: u32 = ADVERTISE_PAUSE_ASYM;
pub const MDIO_AN_T1_ADV_L_FORCE_MS: u32 = 0x1000	/* Force Master/slave Configuration */;
pub const MDIO_AN_T1_ADV_L_REMOTE_FAULT: u32 = ADVERTISE_RFAULT;
pub const MDIO_AN_T1_ADV_L_ACK: u32 = ADVERTISE_LPACK;
pub const MDIO_AN_T1_ADV_L_NEXT_PAGE_REQ: u32 = ADVERTISE_NPAGE;

/* BASE-T1 auto-negotiation advertisement register [31:16] */
pub const MDIO_AN_T1_ADV_M_B10L: u32 = 0x4000	/* device is compatible with 10BASE-T1L */;
pub const MDIO_AN_T1_ADV_M_1000BT1: u32 = 0x0080	/* advertise 1000BASE-T1 */;
pub const MDIO_AN_T1_ADV_M_100BT1: u32 = 0x0020	/* advertise 100BASE-T1 */;
pub const MDIO_AN_T1_ADV_M_MST: u32 = 0x0010	/* advertise master preference */;

/* BASE-T1 auto-negotiation advertisement register [47:32] */
pub const MDIO_AN_T1_ADV_H_10L_TX_HI_REQ: u32 = 0x1000	/* 10BASE-T1L High Level Transmit Request */;
pub const MDIO_AN_T1_ADV_H_10L_TX_HI: u32 = 0x2000	/* 10BASE-T1L High Level Transmit Ability */;

/* BASE-T1 AN LP Base Page ability register [15:0] */
pub const MDIO_AN_T1_LP_L_PAUSE_CAP: u32 = LPA_PAUSE_CAP;
pub const MDIO_AN_T1_LP_L_PAUSE_ASYM: u32 = LPA_PAUSE_ASYM;
pub const MDIO_AN_T1_LP_L_FORCE_MS: u32 = 0x1000	/* LP Force Master/slave Configuration */;
pub const MDIO_AN_T1_LP_L_REMOTE_FAULT: u32 = LPA_RFAULT;
pub const MDIO_AN_T1_LP_L_ACK: u32 = LPA_LPACK;
pub const MDIO_AN_T1_LP_L_NEXT_PAGE_REQ: u32 = LPA_NPAGE;

/* BASE-T1 AN LP Base Page ability register [31:16] */
pub const MDIO_AN_T1_LP_M_MST: u32 = 0x0010	/* LP master preference */;
pub const MDIO_AN_T1_LP_M_B10L: u32 = 0x4000	/* LP is compatible with 10BASE-T1L */;

/* BASE-T1 AN LP Base Page ability register [47:32] */
pub const MDIO_AN_T1_LP_H_10L_TX_HI_REQ: u32 = 0x1000	/* 10BASE-T1L High Level LP Transmit Request */;
pub const MDIO_AN_T1_LP_H_10L_TX_HI: u32 = 0x2000	/* 10BASE-T1L High Level LP Transmit Ability */;

/* 10BASE-T1 AN control register */
pub const MDIO_AN_10BT1_AN_CTRL_ADV_EEE_T1L: u32 = 0x4000 /* 10BASE-T1L EEE ability advertisement */;

/* 10BASE-T1 AN status register */
pub const MDIO_AN_10BT1_AN_STAT_LPA_EEE_T1L: u32 = 0x4000 /* 10BASE-T1L LP EEE ability advertisement */;

/* BASE-T1 PMA/PMD control register */
pub const MDIO_PMA_PMD_BT1_CTRL_STRAP: u32 = 0x000F /* Type selection (Strap) */;
pub const MDIO_PMA_PMD_BT1_CTRL_STRAP_B1000: u32 = 0x0001 /* Select 1000BASE-T1 */;
pub const MDIO_PMA_PMD_BT1_CTRL_CFG_MST: u32 = 0x4000 /* MASTER-SLAVE config value */;

/* 1000BASE-T1 PCS control register */
pub const MDIO_PCS_1000BT1_CTRL_LOW_POWER: u32 = 0x0800 /* Low power mode */;
pub const MDIO_PCS_1000BT1_CTRL_DISABLE_TX: u32 = 0x4000 /* Global PMA transmit disable */;
pub const MDIO_PCS_1000BT1_CTRL_RESET: u32 = 0x8000 /* Software reset value */;

/* 1000BASE-T1 PCS status register */
pub const MDIO_PCS_1000BT1_STAT_LINK: u32 = 0x0004 /* PCS Link is up */;
pub const MDIO_PCS_1000BT1_STAT_FAULT: u32 = 0x0080 /* There is a fault condition */;


/* EEE Supported/Advertisement/LP Advertisement registers.
 *
 * EEE capability Register (3.20), Advertisement (7.60) and
 * Link partner ability (7.61) registers have and can use the same identical
 * bit masks.
 */
pub const MDIO_AN_EEE_ADV_100TX: u32 = 0x0002	/* Advertise 100TX EEE cap */;
pub const MDIO_AN_EEE_ADV_1000T: u32 = 0x0004	/* Advertise 1000T EEE cap */;
/* Note: the two defines above can be potentially used by the user-land
 * and cannot remove them now.
 * So, we define the new generic MDIO_EEE_100TX and MDIO_EEE_1000T macros
 * using the previous ones (that can be considered obsolete).
 */
pub const MDIO_EEE_100TX: u32 = MDIO_AN_EEE_ADV_100TX	/* 100TX EEE cap */;
pub const MDIO_EEE_1000T: u32 = MDIO_AN_EEE_ADV_1000T	/* 1000T EEE cap */;
pub const MDIO_EEE_10GT: u32 = 0x0008	/* 10GT EEE cap */;
pub const MDIO_EEE_1000KX: u32 = 0x0010	/* 1000KX EEE cap */;
pub const MDIO_EEE_10GKX4: u32 = 0x0020	/* 10G KX4 EEE cap */;
pub const MDIO_EEE_10GKR: u32 = 0x0040	/* 10G KR EEE cap */;
pub const MDIO_EEE_40GR_FW: u32 = 0x0100	/* 40G R fast wake */;
pub const MDIO_EEE_40GR_DS: u32 = 0x0200	/* 40G R deep sleep */;
pub const MDIO_EEE_100GR_FW: u32 = 0x1000	/* 100G R fast wake */;
pub const MDIO_EEE_100GR_DS: u32 = 0x2000	/* 100G R deep sleep */;

pub const MDIO_EEE_2_5GT: u32 = 0x0001	/* 2.5GT EEE cap */;
pub const MDIO_EEE_5GT: u32 = 0x0002	/* 5GT EEE cap */;

/* AN MultiGBASE-T AN control 2 */
pub const MDIO_AN_THP_BP2_5GT: u32 = 0x0008	/* 2.5GT THP bypass request */;

/* 2.5G/5G Extended abilities register. */
pub const MDIO_PMA_NG_EXTABLE_2_5GBT: u32 = 0x0001	/* 2.5GBASET ability */;
pub const MDIO_PMA_NG_EXTABLE_5GBT: u32 = 0x0002	/* 5GBASET ability */;

/* LASI RX_ALARM control/status registers. */
pub const MDIO_PMA_LASI_RX_PHYXSLFLT: u32 = 0x0001	/* PHY XS RX local fault */;
pub const MDIO_PMA_LASI_RX_PCSLFLT: u32 = 0x0008	/* PCS RX local fault */;
pub const MDIO_PMA_LASI_RX_PMALFLT: u32 = 0x0010	/* PMA/PMD RX local fault */;
pub const MDIO_PMA_LASI_RX_OPTICPOWERFLT: u32 = 0x0020	/* RX optical power fault */;
pub const MDIO_PMA_LASI_RX_WISLFLT: u32 = 0x0200	/* WIS local fault */;

/* LASI TX_ALARM control/status registers. */
pub const MDIO_PMA_LASI_TX_PHYXSLFLT: u32 = 0x0001	/* PHY XS TX local fault */;
pub const MDIO_PMA_LASI_TX_PCSLFLT: u32 = 0x0008	/* PCS TX local fault */;
pub const MDIO_PMA_LASI_TX_PMALFLT: u32 = 0x0010	/* PMA/PMD TX local fault */;
pub const MDIO_PMA_LASI_TX_LASERPOWERFLT: u32 = 0x0080	/* Laser output power fault */;
pub const MDIO_PMA_LASI_TX_LASERTEMPFLT: u32 = 0x0100	/* Laser temperature fault */;
pub const MDIO_PMA_LASI_TX_LASERBICURRFLT: u32 = 0x0200	/* Laser bias current fault */;

/* LASI control/status registers. */
pub const MDIO_PMA_LASI_LSALARM: u32 = 0x0001	/* LS_ALARM enable/status */;
pub const MDIO_PMA_LASI_TXALARM: u32 = 0x0002	/* TX_ALARM enable/status */;
pub const MDIO_PMA_LASI_RXALARM: u32 = 0x0004	/* RX_ALARM enable/status */;

/* Mapping between MDIO PRTAD/DEVAD and mii_ioctl_data::phy_id */

pub const MDIO_PHY_ID_C45: u32 = 0x8000;
pub const MDIO_PHY_ID_PRTAD: u32 = 0x03e0;
pub const MDIO_PHY_ID_DEVAD: u32 = 0x001f;
pub const MDIO_PHY_ID_C45_MASK: u32 = MDIO_PHY_ID_C45 | MDIO_PHY_ID_PRTAD | MDIO_PHY_ID_DEVAD;

pub const fn mdio_phy_id_c45(prtad: i32, devad: i32) -> u16 {
{
    (MDIO_PHY_ID_C45 | ((prtad << 5) as u32) | (devad as u32)) as u16
}

/* UsxgmiiChannelInfo[15:0] for USXGMII in-band auto-negotiation.*/
pub const MDIO_USXGMII_EEE_CLK_STP: u32 = 0x0080	/* EEE clock stop supported */;
pub const MDIO_USXGMII_EEE: u32 = 0x0100	/* EEE supported */;
pub const MDIO_USXGMII_SPD_MASK: u32 = 0x0e00	/* USXGMII speed mask */;
pub const MDIO_USXGMII_FULL_DUPLEX: u32 = 0x1000	/* USXGMII full duplex */;
pub const MDIO_USXGMII_DPX_SPD_MASK: u32 = 0x1e00	/* USXGMII duplex and speed bits */;
pub const MDIO_USXGMII_10: u32 = 0x0000	/* 10Mbps */;
pub const MDIO_USXGMII_10HALF: u32 = 0x0000	/* 10Mbps half-duplex */;
pub const MDIO_USXGMII_10FULL: u32 = 0x1000	/* 10Mbps full-duplex */;
pub const MDIO_USXGMII_100: u32 = 0x0200	/* 100Mbps */;
pub const MDIO_USXGMII_100HALF: u32 = 0x0200	/* 100Mbps half-duplex */;
pub const MDIO_USXGMII_100FULL: u32 = 0x1200	/* 100Mbps full-duplex */;
pub const MDIO_USXGMII_1000: u32 = 0x0400	/* 1000Mbps */;
pub const MDIO_USXGMII_1000HALF: u32 = 0x0400	/* 1000Mbps half-duplex */;
pub const MDIO_USXGMII_1000FULL: u32 = 0x1400	/* 1000Mbps full-duplex */;
pub const MDIO_USXGMII_10G: u32 = 0x0600	/* 10Gbps */;
pub const MDIO_USXGMII_10GHALF: u32 = 0x0600	/* 10Gbps half-duplex */;
pub const MDIO_USXGMII_10GFULL: u32 = 0x1600	/* 10Gbps full-duplex */;
pub const MDIO_USXGMII_2500: u32 = 0x0800	/* 2500Mbps */;
pub const MDIO_USXGMII_2500HALF: u32 = 0x0800	/* 2500Mbps half-duplex */;
pub const MDIO_USXGMII_2500FULL: u32 = 0x1800	/* 2500Mbps full-duplex */;
pub const MDIO_USXGMII_5000: u32 = 0x0a00	/* 5000Mbps */;
pub const MDIO_USXGMII_5000HALF: u32 = 0x0a00	/* 5000Mbps half-duplex */;
pub const MDIO_USXGMII_5000FULL: u32 = 0x1a00	/* 5000Mbps full-duplex */;
pub const MDIO_USXGMII_LINK: u32 = 0x8000	/* PHY link with copper-side partner */;


// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
