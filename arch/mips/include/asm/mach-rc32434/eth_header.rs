/*
 *  Definitions for the Ethernet registers
 *
 *  Copyright 2002 Allend Stichter <allen.stichter@idt.com>
 *  Copyright 2008 Florian Fainelli <florian@openwrt.org>
 *
 *  This program is free software; you can redistribute  it and/or modify it
 *  under the terms of  the GNU General  Public License as published by the
 *  Free Software Foundation; either version 2 of the License, or (at your
 *  option) any later version.
 */

// C header guard: __ASM_RC32434_ETH_H

pub const ETH0_BASE_ADDR: u32 = 0x18060000;

#[repr(C)]
pub struct eth_regs {
    pub ethintfc: u32,
    pub ethfifott: u32,
    pub etharc: u32,
    pub ethhash0: u32,
    pub ethhash1: u32,
    pub ethu0: [u32; 4], // Reserved.
    pub ethpfs: u32,
    pub ethmcp: u32,
    pub eth_u1: [u32; 10], // Reserved.
    pub ethspare: u32,
    pub eth_u2: [u32; 42], // Reserved.
    pub ethsal0: u32,
    pub ethsah0: u32,
    pub ethsal1: u32,
    pub ethsah1: u32,
    pub ethsal2: u32,
    pub ethsah2: u32,
    pub ethsal3: u32,
    pub ethsah3: u32,
    pub ethrbc: u32,
    pub ethrpc: u32,
    pub ethrupc: u32,
    pub ethrfc: u32,
    pub ethtbc: u32,
    pub ethgpf: u32,
    pub eth_u9: [u32; 50], // Reserved.
    pub ethmac1: u32,
    pub ethmac2: u32,
    pub ethipgt: u32,
    pub ethipgr: u32,
    pub ethclrt: u32,
    pub ethmaxf: u32,
    pub eth_u10: u32, // Reserved.
    pub ethmtest: u32,
    pub miimcfg: u32,
    pub miimcmd: u32,
    pub miimaddr: u32,
    pub miimwtd: u32,
    pub miimrdd: u32,
    pub miimind: u32,
    pub eth_u11: u32, // Reserved.
    pub eth_u12: u32, // Reserved.
    pub ethcfsa0: u32,
    pub ethcfsa1: u32,
    pub ethcfsa2: u32,
}

/* Ethernet interrupt registers */
pub const ETH_INT_FC_EN: u32 = 1 << 0;
pub const ETH_INT_FC_ITS: u32 = 1 << 1;
pub const ETH_INT_FC_RIP: u32 = 1 << 2;
pub const ETH_INT_FC_JAM: u32 = 1 << 3;
pub const ETH_INT_FC_OVR: u32 = 1 << 4;
pub const ETH_INT_FC_UND: u32 = 1 << 5;
pub const ETH_INT_FC_IOC: u32 = 0x000000c0;

/* Ethernet FIFO registers */
pub const ETH_FIFI_TT_TTH_BIT: u32 = 0;
pub const ETH_FIFO_TT_TTH: u32 = 0x0000007f;

/* Ethernet ARC/multicast registers */
pub const ETH_ARC_PRO: u32 = 1 << 0;
pub const ETH_ARC_AM: u32 = 1 << 1;
pub const ETH_ARC_AFM: u32 = 1 << 2;
pub const ETH_ARC_AB: u32 = 1 << 3;

/* Ethernet SAL registers */
pub const ETH_SAL_BYTE_5: u32 = 0x000000ff;
pub const ETH_SAL_BYTE_4: u32 = 0x0000ff00;
pub const ETH_SAL_BYTE_3: u32 = 0x00ff0000;
pub const ETH_SAL_BYTE_2: u32 = 0xff000000;

/* Ethernet SAH registers */
pub const ETH_SAH_BYTE1: u32 = 0x000000ff;
pub const ETH_SAH_BYTE0: u32 = 0x0000ff00;

/* Ethernet GPF register */
pub const ETH_GPF_PTV: u32 = 0x0000ffff;

/* Ethernet PFG register */
pub const ETH_PFS_PFD: u32 = 1 << 0;

/* Ethernet CFSA[0-3] registers */
pub const ETH_CFSA0_CFSA4: u32 = 0x000000ff;
pub const ETH_CFSA0_CFSA5: u32 = 0x0000ff00;
pub const ETH_CFSA1_CFSA2: u32 = 0x000000ff;
pub const ETH_CFSA1_CFSA3: u32 = 0x0000ff00;
pub const ETH_CFSA1_CFSA0: u32 = 0x000000ff;
pub const ETH_CFSA1_CFSA1: u32 = 0x0000ff00;

/* Ethernet MAC1 registers */
pub const ETH_MAC1_RE: u32 = 1 << 0;
pub const ETH_MAC1_PAF: u32 = 1 << 1;
pub const ETH_MAC1_RFC: u32 = 1 << 2;
pub const ETH_MAC1_TFC: u32 = 1 << 3;
pub const ETH_MAC1_LB: u32 = 1 << 4;
pub const ETH_MAC1_MR: u32 = 1 << 31;

/* Ethernet MAC2 registers */
pub const ETH_MAC2_FD: u32 = 1 << 0;
pub const ETH_MAC2_FLC: u32 = 1 << 1;
pub const ETH_MAC2_HFE: u32 = 1 << 2;
pub const ETH_MAC2_DC: u32 = 1 << 3;
pub const ETH_MAC2_CEN: u32 = 1 << 4;
pub const ETH_MAC2_PE: u32 = 1 << 5;
pub const ETH_MAC2_VPE: u32 = 1 << 6;
pub const ETH_MAC2_APE: u32 = 1 << 7;
pub const ETH_MAC2_PPE: u32 = 1 << 8;
pub const ETH_MAC2_LPE: u32 = 1 << 9;
pub const ETH_MAC2_NB: u32 = 1 << 12;
pub const ETH_MAC2_BP: u32 = 1 << 13;
pub const ETH_MAC2_ED: u32 = 1 << 14;

/* Ethernet IPGT register */
pub const ETH_IPGT: u32 = 0x0000007f;
/* Ethernet IPGR registers */
pub const ETH_IPGR_IPGR2: u32 = 0x0000007f;
pub const ETH_IPGR_IPGR1: u32 = 0x00007f00;
/* Ethernet CLRT registers */
pub const ETH_CLRT_MAX_RET: u32 = 0x0000000f;
pub const ETH_CLRT_COL_WIN: u32 = 0x00003f00;
/* Ethernet MAXF register */
pub const ETH_MAXF: u32 = 0x0000ffff;
/* Ethernet test registers */
pub const ETH_TEST_REG: u32 = 1 << 2;
pub const ETH_MCP_DIV: u32 = 0x000000ff;

/* MII registers */
pub const ETH_MII_CFG_RSVD: u32 = 0x0000000c;
pub const ETH_MII_CMD_RD: u32 = 1 << 0;
pub const ETH_MII_CMD_SCN: u32 = 1 << 1;
pub const ETH_MII_REG_ADDR: u32 = 0x0000001f;
pub const ETH_MII_PHY_ADDR: u32 = 0x00001f00;
pub const ETH_MII_WTD_DATA: u32 = 0x0000ffff;
pub const ETH_MII_RDD_DATA: u32 = 0x0000ffff;
pub const ETH_MII_IND_BSY: u32 = 1 << 0;
pub const ETH_MII_IND_SCN: u32 = 1 << 1;
pub const ETH_MII_IND_NV: u32 = 1 << 2;

/* Values for the DEVCS field of the Ethernet DMA Rx and Tx descriptors. */
pub const ETH_RX_FD: u32 = 1 << 0;
pub const ETH_RX_LD: u32 = 1 << 1;
pub const ETH_RX_ROK: u32 = 1 << 2;
pub const ETH_RX_FM: u32 = 1 << 3;
pub const ETH_RX_MP: u32 = 1 << 4;
pub const ETH_RX_BP: u32 = 1 << 5;
pub const ETH_RX_VLT: u32 = 1 << 6;
pub const ETH_RX_CF: u32 = 1 << 7;
pub const ETH_RX_OVR: u32 = 1 << 8;
pub const ETH_RX_CRC: u32 = 1 << 9;
pub const ETH_RX_CV: u32 = 1 << 10;
pub const ETH_RX_DB: u32 = 1 << 11;
pub const ETH_RX_LE: u32 = 1 << 12;
pub const ETH_RX_LOR: u32 = 1 << 13;
pub const ETH_RX_CES: u32 = 1 << 14;
pub const ETH_RX_LEN_BIT: u32 = 16;
pub const ETH_RX_LEN: u32 = 0xffff0000;

pub const ETH_TX_FD: u32 = 1 << 0;
pub const ETH_TX_LD: u32 = 1 << 1;
pub const ETH_TX_OEN: u32 = 1 << 2;
pub const ETH_TX_PEN: u32 = 1 << 3;
pub const ETH_TX_CEN: u32 = 1 << 4;
pub const ETH_TX_HEN: u32 = 1 << 5;
pub const ETH_TX_TOK: u32 = 1 << 6;
pub const ETH_TX_MP: u32 = 1 << 7;
pub const ETH_TX_BP: u32 = 1 << 8;
pub const ETH_TX_UND: u32 = 1 << 9;
pub const ETH_TX_OF: u32 = 1 << 10;
pub const ETH_TX_ED: u32 = 1 << 11;
pub const ETH_TX_EC: u32 = 1 << 12;
pub const ETH_TX_LC: u32 = 1 << 13;
pub const ETH_TX_TD: u32 = 1 << 14;
pub const ETH_TX_CRC: u32 = 1 << 15;
pub const ETH_TX_LE: u32 = 1 << 16;
pub const ETH_TX_CC: u32 = 0x001E0000;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
