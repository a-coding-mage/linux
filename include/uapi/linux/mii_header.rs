/* SPDX-License-Identifier: GPL-2.0 WITH Linux-syscall-note */
/*
 * linux/mii.h: definitions for MII-compatible transceivers
 * Originally drivers/net/sunhme.h.
 *
 * Copyright (C) 1996, 1999, 2001 David S. Miller (davem@redhat.com)
 */

/* Generic MII registers. */
pub const MII_BMCR: u16 = 0x00;
pub const MII_BMSR: u16 = 0x01;
pub const MII_PHYSID1: u16 = 0x02;
pub const MII_PHYSID2: u16 = 0x03;
pub const MII_ADVERTISE: u16 = 0x04;
pub const MII_LPA: u16 = 0x05;
pub const MII_EXPANSION: u16 = 0x06;
pub const MII_CTRL1000: u16 = 0x09;
pub const MII_STAT1000: u16 = 0x0a;
pub const MII_MMD_CTRL: u16 = 0x0d;
pub const MII_MMD_DATA: u16 = 0x0e;
pub const MII_ESTATUS: u16 = 0x0f;
pub const MII_DCOUNTER: u16 = 0x12;
pub const MII_FCSCOUNTER: u16 = 0x13;
pub const MII_NWAYTEST: u16 = 0x14;
pub const MII_RERRCOUNTER: u16 = 0x15;
pub const MII_SREVISION: u16 = 0x16;
pub const MII_RESV1: u16 = 0x17;
pub const MII_LBRERROR: u16 = 0x18;
pub const MII_PHYADDR: u16 = 0x19;
pub const MII_RESV2: u16 = 0x1a;
pub const MII_TPISTATUS: u16 = 0x1b;
pub const MII_NCONFIG: u16 = 0x1c;

/* Basic mode control register. */
pub const BMCR_RESV: u16 = 0x003f;
pub const BMCR_SPEED1000: u16 = 0x0040;
pub const BMCR_CTST: u16 = 0x0080;
pub const BMCR_FULLDPLX: u16 = 0x0100;
pub const BMCR_ANRESTART: u16 = 0x0200;
pub const BMCR_ISOLATE: u16 = 0x0400;
pub const BMCR_PDOWN: u16 = 0x0800;
pub const BMCR_ANENABLE: u16 = 0x1000;
pub const BMCR_SPEED100: u16 = 0x2000;
pub const BMCR_LOOPBACK: u16 = 0x4000;
pub const BMCR_RESET: u16 = 0x8000;
pub const BMCR_SPEED10: u16 = 0x0000;

/* Basic mode status register. */
pub const BMSR_ERCAP: u16 = 0x0001;
pub const BMSR_JCD: u16 = 0x0002;
pub const BMSR_LSTATUS: u16 = 0x0004;
pub const BMSR_ANEGCAPABLE: u16 = 0x0008;
pub const BMSR_RFAULT: u16 = 0x0010;
pub const BMSR_ANEGCOMPLETE: u16 = 0x0020;
pub const BMSR_RESV: u16 = 0x00c0;
pub const BMSR_ESTATEN: u16 = 0x0100;
pub const BMSR_100HALF2: u16 = 0x0200;
pub const BMSR_100FULL2: u16 = 0x0400;
pub const BMSR_10HALF: u16 = 0x0800;
pub const BMSR_10FULL: u16 = 0x1000;
pub const BMSR_100HALF: u16 = 0x2000;
pub const BMSR_100FULL: u16 = 0x4000;
pub const BMSR_100BASE4: u16 = 0x8000;

/* Advertisement control register. */
pub const ADVERTISE_SLCT: u16 = 0x001f;
pub const ADVERTISE_CSMA: u16 = 0x0001;
pub const ADVERTISE_10HALF: u16 = 0x0020;
pub const ADVERTISE_1000XFULL: u16 = 0x0020;
pub const ADVERTISE_10FULL: u16 = 0x0040;
pub const ADVERTISE_1000XHALF: u16 = 0x0040;
pub const ADVERTISE_100HALF: u16 = 0x0080;
pub const ADVERTISE_1000XPAUSE: u16 = 0x0080;
pub const ADVERTISE_100FULL: u16 = 0x0100;
pub const ADVERTISE_1000XPSE_ASYM: u16 = 0x0100;
pub const ADVERTISE_100BASE4: u16 = 0x0200;
pub const ADVERTISE_PAUSE_CAP: u16 = 0x0400;
pub const ADVERTISE_PAUSE_ASYM: u16 = 0x0800;
pub const ADVERTISE_XNP: u16 = 0x1000;
pub const ADVERTISE_RESV: u16 = ADVERTISE_XNP;
pub const ADVERTISE_RFAULT: u16 = 0x2000;
pub const ADVERTISE_LPACK: u16 = 0x4000;
pub const ADVERTISE_NPAGE: u16 = 0x8000;
pub const ADVERTISE_FULL: u16 = ADVERTISE_100FULL | ADVERTISE_10FULL | ADVERTISE_CSMA;
pub const ADVERTISE_ALL: u16 = ADVERTISE_10HALF | ADVERTISE_10FULL | ADVERTISE_100HALF | ADVERTISE_100FULL;

/* Link partner ability register. */
pub const LPA_SLCT: u16 = 0x001f;
pub const LPA_10HALF: u16 = 0x0020;
pub const LPA_1000XFULL: u16 = 0x0020;
pub const LPA_10FULL: u16 = 0x0040;
pub const LPA_1000XHALF: u16 = 0x0040;
pub const LPA_100HALF: u16 = 0x0080;
pub const LPA_1000XPAUSE: u16 = 0x0080;
pub const LPA_100FULL: u16 = 0x0100;
pub const LPA_1000XPAUSE_ASYM: u16 = 0x0100;
pub const LPA_100BASE4: u16 = 0x0200;
pub const LPA_PAUSE_CAP: u16 = 0x0400;
pub const LPA_PAUSE_ASYM: u16 = 0x0800;
pub const LPA_RESV: u16 = 0x1000;
pub const LPA_RFAULT: u16 = 0x2000;
pub const LPA_LPACK: u16 = 0x4000;
pub const LPA_NPAGE: u16 = 0x8000;
pub const LPA_DUPLEX: u16 = LPA_10FULL | LPA_100FULL;
pub const LPA_100: u16 = LPA_100FULL | LPA_100HALF | LPA_100BASE4;

/* Expansion register for auto-negotiation. */
pub const EXPANSION_NWAY: u16 = 0x0001;
pub const EXPANSION_LCWP: u16 = 0x0002;
pub const EXPANSION_ENABLENPAGE: u16 = 0x0004;
pub const EXPANSION_NPCAPABLE: u16 = 0x0008;
pub const EXPANSION_MFAULTS: u16 = 0x0010;
pub const EXPANSION_RESV: u16 = 0xffe0;
pub const ESTATUS_1000_XFULL: u16 = 0x8000;
pub const ESTATUS_1000_XHALF: u16 = 0x4000;
pub const ESTATUS_1000_TFULL: u16 = 0x2000;
pub const ESTATUS_1000_THALF: u16 = 0x1000;

/* N-way test register. */
pub const NWAYTEST_RESV1: u16 = 0x00ff;
pub const NWAYTEST_LOOPBACK: u16 = 0x0100;
pub const NWAYTEST_RESV2: u16 = 0xfe00;

/* MAC and PHY tx_config_Reg[15:0] for SGMII in-band auto-negotiation. */
pub const ADVERTISE_SGMII: u16 = 0x0001;
pub const LPA_SGMII: u16 = 0x0001;
pub const LPA_SGMII_SPD_MASK: u16 = 0x0c00;
pub const LPA_SGMII_FULL_DUPLEX: u16 = 0x1000;
pub const LPA_SGMII_DPX_SPD_MASK: u16 = 0x1C00;
pub const LPA_SGMII_10: u16 = 0x0000;
pub const LPA_SGMII_10HALF: u16 = 0x0000;
pub const LPA_SGMII_10FULL: u16 = 0x1000;
pub const LPA_SGMII_100: u16 = 0x0400;
pub const LPA_SGMII_100HALF: u16 = 0x0400;
pub const LPA_SGMII_100FULL: u16 = 0x1400;
pub const LPA_SGMII_1000: u16 = 0x0800;
pub const LPA_SGMII_1000HALF: u16 = 0x0800;
pub const LPA_SGMII_1000FULL: u16 = 0x1800;
pub const LPA_SGMII_LINK: u16 = 0x8000;

/* 1000BASE-T Control register */
pub const ADVERTISE_1000FULL: u16 = 0x0200;
pub const ADVERTISE_1000HALF: u16 = 0x0100;
pub const CTL1000_PREFER_MASTER: u16 = 0x0400;
pub const CTL1000_AS_MASTER: u16 = 0x0800;
pub const CTL1000_ENABLE_MASTER: u16 = 0x1000;

/* 1000BASE-T Status register */
pub const LPA_1000MSFAIL: u16 = 0x8000;
pub const LPA_1000MSRES: u16 = 0x4000;
pub const LPA_1000LOCALRXOK: u16 = 0x2000;
pub const LPA_1000REMRXOK: u16 = 0x1000;
pub const LPA_1000FULL: u16 = 0x0800;
pub const LPA_1000HALF: u16 = 0x0400;

/* Flow control flags */
pub const FLOW_CTRL_TX: u16 = 0x01;
pub const FLOW_CTRL_RX: u16 = 0x02;

/* MMD Access Control register fields */
pub const MII_MMD_CTRL_DEVAD_MASK: u16 = 0x1f;
pub const MII_MMD_CTRL_ADDR: u16 = 0x0000;
pub const MII_MMD_CTRL_NOINCR: u16 = 0x4000;
pub const MII_MMD_CTRL_INCR_RDWT: u16 = 0x8000;
pub const MII_MMD_CTRL_INCR_ON_WT: u16 = 0xC000;

/* This structure is used in all SIOCxMIIxxx ioctl calls */
#[repr(C)]
pub struct mii_ioctl_data {
    pub phy_id: u16,
    pub reg_num: u16,
    pub val_in: u16,
    pub val_out: u16,
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
