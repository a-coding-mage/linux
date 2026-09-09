/* SPDX-License-Identifier: GPL-2.0
 *
 * SH7786 PCI-Express controller definitions.
 *
 * Copyright (C) 2008, 2009 Renesas Technology Corp.
 * All rights reserved.
 */

/* PCIe bus-0(x4) on SH7786 */			// Rev1.171
pub const SH4A_PCIE_SPW_BASE: u64 = 0xFE000000;
pub const SH4A_PCIE_SPW_BASE1: u64 = 0xFE200000;
pub const SH4A_PCIE_SPW_BASE2: u64 = 0xFCC00000;
pub const SH4A_PCIE_SPW_BASE_LEN: u64 = 0x00080000;

pub const SH4A_PCI_CNFG_BASE: u64 = 0xFE040000;
pub const SH4A_PCI_CNFG_BASE1: u64 = 0xFE240000;
pub const SH4A_PCI_CNFG_BASE2: u64 = 0xFCC40000;
pub const SH4A_PCI_CNFG_BASE_LEN: u64 = 0x00040000;

pub const SH4A_PCIPIO_ADDR_OFFSET: u64 = 0x000001c0;
pub const SH4A_PCIPIO_DATA_OFFSET: u64 = 0x00000220;

/*
 * for PEX8111(Max Payload Size=128B,PCIIO_SIZE=64K),
 * for other(Max Payload Size=4096B,PCIIO_SIZE=8M)
 */

/* PCI0: PCI memory target transfer 32-bit address translation value(Rev1.11T)*/
pub const SH4A_PCIBMSTR_TRANSLATION: u64 = 0x20000000;

/*	SPVCR0		*/
pub const SH4A_PCIEVCR0: u64 = (0x000000);
pub const BITS_TOP_MB: u64 = (24);
pub const MASK_TOP_MB: u64 = (0xff<<BITS_TOP_MB);
pub const BITS_BOT_MB: u64 = (16);
pub const MASK_BOT_MB: u64 = (0xff<<BITS_BOT_MB);
pub const BITS_VC_ID: u64 = (0);
pub const MASK_VC_ID: u64 = (0xffff<<BITS_VC_ID);

/*	SPVCR1		*/
pub const SH4A_PCIEVCR1: u64 = (0x000004);
pub const BITS_BADOPC: u64 = (5);
pub const MASK_BADOPC: u64 = (1<<BITS_BADOPC);
pub const BITS_BADDEST: u64 = (4);
pub const MASK_BADDEST: u64 = (1<<BITS_BADDEST);
pub const BITS_UNSOLRESP: u64 = (3);
pub const MASK_UNSOLRESP: u64 = (1<<BITS_UNSOLRESP);
pub const BITS_ERRSNT: u64 = (1);
pub const MASK_ERRSNT: u64 = (1<<BITS_ERRSNT);
pub const BITS_ERRRCV: u64 = (0);
pub const MASK_ERRRCV: u64 = (1<<BITS_ERRRCV);

/*	PCIEENBLR	 */
pub const SH4A_PCIEENBLR: u64 = (0x000008);

/*	PCIEECR		*/
pub const SH4A_PCIEECR: u64 = (0x00000C);
pub const BITS_ENBL: u64 = (0);
pub const MASK_ENBL: u64 = (1<<BITS_ENBL);

/*	PCIEPAR		*/
pub const SH4A_PCIEPAR: u64 = (0x000010);
pub const BITS_BN: u64 = (24);
pub const MASK_BN: u64 = (0xff<<BITS_BN);
pub const BITS_DN: u64 = (19);
pub const MASK_DN: u64 = (0x1f<<BITS_DN);
pub const BITS_FN: u64 = (16);
pub const MASK_FN: u64 = (0x7<<BITS_FN);
pub const BITS_EREGNO: u64 = (8);
pub const MASK_EREGNO: u64 = (0xff<<BITS_EREGNO);
pub const BITS_REGNO: u64 = (2);
pub const MASK_REGNO: u64 = (0x3f<<BITS_REGNO);

/*	PCIEPCTLR	*/
pub const SH4A_PCIEPCTLR: u64 = (0x000018);
pub const BITS_CCIE: u64 = (31);
pub const MASK_CCIE: u64 = (1<<BITS_CCIE);
pub const BITS_TYPE: u64 = (8);
pub const MASK_TYPE: u64 = (1<<BITS_TYPE);
pub const BITS_C_VC: u64 = (0);
pub const MASK_C_VC: u64 = (1<<BITS_C_VC);

/*	PCIEPDR		*/
pub const SH4A_PCIEPDR: u64 = (0x000020);
pub const BITS_PDR: u64 = (0);
pub const MASK_PDR: u64 = (0xffffffff<<BITS_PDR);

/*	PCIEMSGALR	*/
pub const SH4A_PCIEMSGALR: u64 = (0x000030);
pub const BITS_MSGADRL: u64 = (0);
pub const MASK_MSGADRL: u64 = (0xffffffff<<BITS_MSGADRL);

/*	PCIEMSGAHR	*/
pub const SH4A_PCIEMSGAHR: u64 = (0x000034);
pub const BITS_MSGADRH: u64 = (0);
pub const MASK_MSGADRH: u64 = (0xffffffff<<BITS_MSGADRH);

/*	PCIEMSGCTLR	*/
pub const SH4A_PCIEMSGCTLR: u64 = (0x000038);
pub const BITS_MSGIE: u64 = (31);
pub const MASK_MSGIE: u64 = (1<<BITS_MSGIE);
pub const BITS_MROUTE: u64 = (16);
pub const MASK_MROUTE: u64 = (0x7<<BITS_MROUTE);
pub const BITS_MCODE: u64 = (8);
pub const MASK_MCODE: u64 = (0xff<<BITS_MCODE);
pub const BITS_M_VC: u64 = (0);
pub const MASK_M_VC: u64 = (1<<BITS_M_VC);

/*	PCIEMSG		*/
pub const SH4A_PCIEMSG: u64 = (0x000040);
pub const BITS_MDATA: u64 = (0);
pub const MASK_MDATA: u64 = (0xffffffff<<BITS_MDATA);

/*	PCIEUNLOCKCR	*/
pub const SH4A_PCIEUNLOCKCR: u64 = (0x000048);

/*	PCIEIDR		*/
pub const SH4A_PCIEIDR: u64 = (0x000060);

/*	PCIEDBGCTLR	*/
pub const SH4A_PCIEDBGCTLR: u64 = (0x000100);

/*	PCIEINTXR	*/
pub const SH4A_PCIEINTXR: u64 = (0x004000);

/*	PCIERMSGR	*/
pub const SH4A_PCIERMSGR: u64 = (0x004010);

/*	PCIERSTR	*/
macro_rules! SH4A_PCIERSTR { ($x:expr) => { ((0x008000 + (($x) * 0x4))) }; }

/*	PCIESRSTR	 */
pub const SH4A_PCIESRSTR: u64 = (0x008040);

/*	PCIEPHYCTLR	*/
pub const SH4A_PCIEPHYCTLR: u64 = (0x010000);
pub const BITS_CKE: u64 = (0);
pub const MASK_CKE: u64 = (1<<BITS_CKE);

/*	PCIERMSGIER	*/
pub const SH4A_PCIERMSGIER: u64 = (0x004040);

/*	PCIEPHYADRR	*/
pub const SH4A_PCIEPHYADRR: u64 = (0x010004);
pub const BITS_ACK: u64 = (24);
pub const MASK_ACK: u64 = (1<<BITS_ACK);
pub const BITS_CMD: u64 = (16);
pub const MASK_CMD: u64 = (0x03<<BITS_CMD);
pub const BITS_LANE: u64 = (8);
pub const MASK_LANE: u64 = (0x0f<<BITS_LANE);
pub const BITS_ADR: u64 = (0);
pub const MASK_ADR: u64 = (0xff<<BITS_ADR);

/*	PCIEPHYDINR	*/							// Rev1.171 start.
pub const SH4A_PCIEPHYDINR: u64 = (0x010008);

/*	PCIEPHYDOUTR	*/
pub const SH4A_PCIEPHYDOUTR: u64 = (0x01000C);

/*	PCIEPHYSR	*/
pub const SH4A_PCIEPHYSR: u64 = (0x010010);

/*	PCIEPHYDATAR	*/
pub const SH4A_PCIEPHYDATAR: u64 = (0x00008);
pub const BITS_DATA: u64 = (0);
pub const MASK_DATA: u64 = (0xffffffff<<BITS_DATA);

/*	PCIETCTLR	*/
pub const SH4A_PCIETCTLR: u64 = (0x020000);
pub const BITS_CFINT: u64 = (0);
pub const MASK_CFINT: u64 = (1<<BITS_CFINT);

/*	PCIETSTR	*/
pub const SH4A_PCIETSTR: u64 = (0x020004);

/*	PCIEINTR	*/
pub const SH4A_PCIEINTR: u64 = (0x020008);
pub const BITS_INT_RX_ERP: u64 = (31);
pub const MASK_INT_RX_ERP: u64 = (1<<BITS_INT_RX_ERP);
pub const BITS_INT_RX_VCX_Posted: u64 = (30);
pub const MASK_INT_RX_VCX_Posted: u64 = (1<<BITS_INT_RX_VCX_Posted);
pub const BITS_INT_RX_VCX_NonPosted: u64 = (29);
pub const MASK_INT_RX_VCX_NonPosted: u64 = (1<<BITS_INT_RX_VCX_NonPosted);
pub const BITS_INT_RX_VCX_CPL: u64 = (28);
pub const MASK_INT_RX_VCX_CPL: u64 = (1<<BITS_INT_RX_VCX_CPL);
pub const BITS_INT_TX_VCX_Posted: u64 = (26);
pub const MASK_INT_TX_VCX_Posted: u64 = (1<<BITS_INT_TX_VCX_Posted);
pub const BITS_INT_TX_VCX_NonPosted: u64 = (25);
pub const MASK_INT_TX_VCX_NonPosted: u64 = (1<<BITS_INT_TX_VCX_NonPosted);
pub const BITS_INT_TX_VCX_CPL: u64 = (24);
pub const MASK_INT_TX_VCX_CPL: u64 = (1<<BITS_INT_TX_VCX_CPL);
pub const BITS_INT_RX_VC0_Posted: u64 = (22);
pub const MASK_INT_RX_VC0_Posted: u64 = (1<<BITS_INT_RX_VC0_Posted);
pub const BITS_INT_RX_VC0_NonPosted: u64 = (21);
pub const MASK_INT_RX_VC0_NonPosted: u64 = (1<<BITS_INT_RX_VC0_NonPosted);
pub const BITS_INT_RX_VC0_CPL: u64 = (20);
pub const MASK_INT_RX_VC0_CPL: u64 = (1<<BITS_INT_RX_VC0_CPL);
pub const BITS_INT_TX_VC0_Posted: u64 = (18);
pub const MASK_INT_TX_VC0_Posted: u64 = (1<<BITS_INT_TX_VC0_Posted);
pub const BITS_INT_TX_VC0_NonPosted: u64 = (17);
pub const MASK_INT_TX_VC0_NonPosted: u64 = (1<<BITS_INT_TX_VC0_NonPosted);
pub const BITS_INT_TX_VC0_CPL: u64 = (16);
pub const MASK_INT_TX_VC0_CPL: u64 = (1<<BITS_INT_TX_VC0_CPL);
pub const BITS_INT_RX_CTRL: u64 = (15);
pub const MASK_INT_RX_CTRL: u64 = (1<<BITS_INT_RX_CTRL);
pub const BITS_INT_TX_CTRL: u64 = (14);
pub const MASK_INT_TX_CTRL: u64 = (1<<BITS_INT_TX_CTRL);
pub const BITS_INTTL: u64 = (11);
pub const MASK_INTTL: u64 = (1<<BITS_INTTL);
pub const BITS_INTDL: u64 = (10);
pub const MASK_INTDL: u64 = (1<<BITS_INTDL);
pub const BITS_INTMAC: u64 = (9);
pub const MASK_INTMAC: u64 = (1<<BITS_INTMAC);
pub const BITS_INTPM: u64 = (8);
pub const MASK_INTPM: u64 = (1<<BITS_INTPM);

/*	PCIEINTER	*/
pub const SH4A_PCIEINTER: u64 = (0x02000C);

/*	PCIEEH0R	*/
macro_rules! SH4A_PCIEEHR { ($x:expr) => { ((0x020010 + (($x) * 0x4))) }; }

/*	PCIEAIR	 */
pub const SH4A_PCIEAIR: u64 = (SH4A_PCIE_BASE + 0x020010);

/*	 PCIECIR	 */
pub const SH4A_PCIECIR: u64 = (SH4A_PCIE_BASE);

/*	 PCIEERRFR	 */								// Rev1.18
pub const SH4A_PCIEERRFR: u64 = (0x020020);

/*	PCIEERRFER	*/
pub const SH4A_PCIEERRFER: u64 = (0x020024);

/*	PCIEERRFR2	*/
pub const SH4A_PCIEERRFR2: u64 = (0x020028);

/*	PCIEMSIR	*/
pub const SH4A_PCIEMSIR: u64 = (0x020040);

/*	PCIEMSIFR	*/
pub const SH4A_PCIEMSIFR: u64 = (0x020044);

/*	PCIEPWRCTLR	*/
pub const SH4A_PCIEPWRCTLR: u64 = (0x020100);

/*	PCIEPCCTLR	*/
pub const SH4A_PCIEPCCTLR: u64 = (0x020180);

											// Rev1.18
/*	PCIELAR0	*/
pub const SH4A_PCIELAR0: u64 = (0x020200);
pub const BITS_LARn: u64 = (20);
pub const MASK_LARn: u64 = (0xfff<<BITS_LARn);

pub const SH4A_PCIE_020204: u64 = (0x020204);

/*	PCIELAMR0	*/
pub const SH4A_PCIELAMR0: u64 = (0x020208);
pub const BITS_LAMRn: u64 = (20);
pub const MASK_LAMRn: u64 = (0x1ff<<BITS_LAMRn);
pub const BITS_LAREn: u64 = (0);
pub const MASK_LAREn: u64 = (0x1<<BITS_LAREn);

/*	PCIECSCR0	*/
pub const SH4A_PCIECSCR0: u64 = (0x020210);
pub const BITS_RANGE: u64 = (2);
pub const MASK_RANGE: u64 = (0x7<<BITS_RANGE);
pub const BITS_SNPMD: u64 = (0);
pub const MASK_SNPMD: u64 = (0x3<<BITS_SNPMD);

/*	PCIECSAR0	*/
pub const SH4A_PCIECSAR0: u64 = (0x020214);
pub const BITS_CSADR: u64 = (0);
pub const MASK_CSADR: u64 = (0xffffffff<<BITS_CSADR);

/*	PCIESTCTLR0	*/
pub const SH4A_PCIESTCTLR0: u64 = (0x020218);
pub const BITS_SHPRI: u64 = (8);
pub const MASK_SHPRI: u64 = (0x0f<<BITS_SHPRI);

pub const SH4A_PCIE_020224: u64 = (0x020224);

pub const SH4A_PCIELAR1: u64 = (0x020220);
pub const SH4A_PCIELAMR1: u64 = (0x020228);
pub const SH4A_PCIECSCR1: u64 = (0x020230);
pub const SH4A_PCIECSAR1: u64 = (0x020234);
pub const SH4A_PCIESTCTLR1: u64 = (0x020238);

pub const SH4A_PCIELAR2: u64 = (0x020240);
pub const SH4A_PCIE_020244: u64 = (0x020244);
pub const SH4A_PCIELAMR2: u64 = (0x020248);
pub const SH4A_PCIECSCR2: u64 = (0x020250);
pub const SH4A_PCIECSAR2: u64 = (0x020254);
pub const SH4A_PCIESTCTLR2: u64 = (0x020258);

pub const SH4A_PCIELAR3: u64 = (0x020260);
pub const SH4A_PCIE_020264: u64 = (0x020264);
pub const SH4A_PCIELAMR3: u64 = (0x020268);
pub const SH4A_PCIECSCR3: u64 = (0x020270);
pub const SH4A_PCIECSAR3: u64 = (0x020274);
pub const SH4A_PCIESTCTLR3: u64 = (0x020278);

pub const SH4A_PCIELAR4: u64 = (0x020280);
pub const SH4A_PCIE_020284: u64 = (0x020284);
pub const SH4A_PCIELAMR4: u64 = (0x020288);
pub const SH4A_PCIECSCR4: u64 = (0x020290);
pub const SH4A_PCIECSAR4: u64 = (0x020294);
pub const SH4A_PCIESTCTLR4: u64 = (0x020298);

pub const SH4A_PCIELAR5: u64 = (0x0202A0);
pub const SH4A_PCIE_0202A4: u64 = (0x0202A4);
pub const SH4A_PCIELAMR5: u64 = (0x0202A8);
pub const SH4A_PCIECSCR5: u64 = (0x0202B0);
pub const SH4A_PCIECSAR5: u64 = (0x0202B4);
pub const SH4A_PCIESTCTLR5: u64 = (0x0202B8);

/*	PCIEPARL	*/
macro_rules! SH4A_PCIEPARL { ($x:expr) => { ((0x020400 + (($x) * 0x20))) }; }
pub const BITS_PAL: u64 = (18);
pub const MASK_PAL: u64 = (0x3fff<<BITS_PAL);

/*	PCIEPARH	*/
macro_rules! SH4A_PCIEPARH { ($x:expr) => { ((0x020404 + (($x) * 0x20))) }; }
pub const BITS_PAH: u64 = (0);
pub const MASK_PAH: u64 = (0xffffffff<<BITS_PAH);

/*	PCIEPAMR	 */
macro_rules! SH4A_PCIEPAMR { ($x:expr) => { ((0x020408 + (($x) * 0x20))) }; }
pub const BITS_PAM: u64 = (18);
pub const MASK_PAM: u64 = (0x3fff<<BITS_PAM);

/*	PCIEPTCTLR	*/
macro_rules! SH4A_PCIEPTCTLR { ($x:expr) => { ((0x02040C + (($x) * 0x20))) }; }
pub const BITS_PARE: u64 = (31);
pub const MASK_PARE: u64 = (0x1<<BITS_PARE);
pub const BITS_TC: u64 = (20);
pub const MASK_TC: u64 = (0x7<<BITS_TC);
pub const BITS_T_VC: u64 = (16);
pub const MASK_T_VC: u64 = (0x1<<BITS_T_VC);
pub const BITS_LOCK: u64 = (12);
pub const MASK_LOCK: u64 = (0x1<<BITS_LOCK);
pub const BITS_SPC: u64 = (8);
pub const MASK_SPC: u64 = (0x1<<BITS_SPC);

pub const SH4A_PCIEDMAOR: u64 = (0x021000);
pub const SH4A_PCIEDMSAR0: u64 = (0x021100);
pub const SH4A_PCIEDMSAHR0: u64 = (0x021104);
pub const SH4A_PCIEDMDAR0: u64 = (0x021108);
pub const SH4A_PCIEDMDAHR0: u64 = (0x02110C);
pub const SH4A_PCIEDMBCNTR0: u64 = (0x021110);
pub const SH4A_PCIEDMSBCNTR0: u64 = (0x021114);
pub const SH4A_PCIEDMSTRR0: u64 = (0x021118);
pub const SH4A_PCIEDMCCAR0: u64 = (0x02111C);
pub const SH4A_PCIEDMCCR0: u64 = (0x021120);
pub const SH4A_PCIEDMCC2R0: u64 = (0x021124);
pub const SH4A_PCIEDMCCCR0: u64 = (0x021128);
pub const SH4A_PCIEDMCHSR0: u64 = (0x02112C);
pub const SH4A_PCIEDMSAR1: u64 = (0x021140);
pub const SH4A_PCIEDMSAHR1: u64 = (0x021144);
pub const SH4A_PCIEDMDAR1: u64 = (0x021148);
pub const SH4A_PCIEDMDAHR1: u64 = (0x02114C);
pub const SH4A_PCIEDMBCNTR1: u64 = (0x021150);
pub const SH4A_PCIEDMSBCNTR1: u64 = (0x021154);
pub const SH4A_PCIEDMSTRR1: u64 = (0x021158);
pub const SH4A_PCIEDMCCAR1: u64 = (0x02115C);
pub const SH4A_PCIEDMCCR1: u64 = (0x021160);
pub const SH4A_PCIEDMCC2R1: u64 = (0x021164);
pub const SH4A_PCIEDMCCCR1: u64 = (0x021168);
pub const SH4A_PCIEDMCHSR1: u64 = (0x02116C);
pub const SH4A_PCIEDMSAR2: u64 = (0x021180);
pub const SH4A_PCIEDMSAHR2: u64 = (0x021184);
pub const SH4A_PCIEDMDAR2: u64 = (0x021188);
pub const SH4A_PCIEDMDAHR2: u64 = (0x02118C);
pub const SH4A_PCIEDMBCNTR2: u64 = (0x021190);
pub const SH4A_PCIEDMSBCNTR2: u64 = (0x021194);
pub const SH4A_PCIEDMSTRR2: u64 = (0x021198);
pub const SH4A_PCIEDMCCAR2: u64 = (0x02119C);
pub const SH4A_PCIEDMCCR2: u64 = (0x0211A0);
pub const SH4A_PCIEDMCC2R2: u64 = (0x0211A4);
pub const SH4A_PCIEDMCCCR2: u64 = (0x0211A8);
pub const SH4A_PCIEDMSAR3: u64 = (0x0211C0);
pub const SH4A_PCIEDMSAHR3: u64 = (0x0211C4);
pub const SH4A_PCIEDMDAR3: u64 = (0x0211C8);
pub const SH4A_PCIEDMDAHR3: u64 = (0x0211CC);
pub const SH4A_PCIEDMBCNTR3: u64 = (0x0211D0);
pub const SH4A_PCIEDMSBCNTR3: u64 = (0x0211D4);
pub const SH4A_PCIEDMSTRR3: u64 = (0x0211D8);
pub const SH4A_PCIEDMCCAR3: u64 = (0x0211DC);
pub const SH4A_PCIEDMCCR3: u64 = (0x0211E0);
pub const SH4A_PCIEDMCC2R3: u64 = (0x0211E4);
pub const SH4A_PCIEDMCCCR3: u64 = (0x0211E8);
pub const SH4A_PCIEDMCHSR3: u64 = (0x0211EC);
pub const SH4A_PCIEPCICONF0: u64 = (0x040000);
pub const SH4A_PCIEPCICONF1: u64 = (0x040004);
pub const SH4A_PCIEPCICONF2: u64 = (0x040008);
pub const SH4A_PCIEPCICONF3: u64 = (0x04000C);
pub const SH4A_PCIEPCICONF4: u64 = (0x040010);
pub const SH4A_PCIEPCICONF5: u64 = (0x040014);
pub const SH4A_PCIEPCICONF6: u64 = (0x040018);
pub const SH4A_PCIEPCICONF7: u64 = (0x04001C);
pub const SH4A_PCIEPCICONF8: u64 = (0x040020);
pub const SH4A_PCIEPCICONF9: u64 = (0x040024);
pub const SH4A_PCIEPCICONF10: u64 = (0x040028);
pub const SH4A_PCIEPCICONF11: u64 = (0x04002C);
pub const SH4A_PCIEPCICONF12: u64 = (0x040030);
pub const SH4A_PCIEPCICONF13: u64 = (0x040034);
pub const SH4A_PCIEPCICONF14: u64 = (0x040038);
pub const SH4A_PCIEPCICONF15: u64 = (0x04003C);
pub const SH4A_PCIEPMCAP0: u64 = (0x040040);
pub const SH4A_PCIEPMCAP1: u64 = (0x040044);
pub const SH4A_PCIEMSICAP0: u64 = (0x040050);
pub const SH4A_PCIEMSICAP1: u64 = (0x040054);
pub const SH4A_PCIEMSICAP2: u64 = (0x040058);
pub const SH4A_PCIEMSICAP3: u64 = (0x04005C);
pub const SH4A_PCIEMSICAP4: u64 = (0x040060);
pub const SH4A_PCIEMSICAP5: u64 = (0x040064);
pub const SH4A_PCIEEXPCAP0: u64 = (0x040070);
pub const SH4A_PCIEEXPCAP1: u64 = (0x040074);
pub const SH4A_PCIEEXPCAP2: u64 = (0x040078);
pub const SH4A_PCIEEXPCAP3: u64 = (0x04007C);
pub const SH4A_PCIEEXPCAP4: u64 = (0x040080);
pub const SH4A_PCIEEXPCAP5: u64 = (0x040084);
pub const SH4A_PCIEEXPCAP6: u64 = (0x040088);
pub const SH4A_PCIEEXPCAP7: u64 = (0x04008C);
pub const SH4A_PCIEEXPCAP8: u64 = (0x040090);
pub const SH4A_PCIEVCCAP0: u64 = (0x040100);
pub const SH4A_PCIEVCCAP1: u64 = (0x040104);
pub const SH4A_PCIEVCCAP2: u64 = (0x040108);
pub const SH4A_PCIEVCCAP3: u64 = (0x04010C);
pub const SH4A_PCIEVCCAP4: u64 = (0x040110);
pub const SH4A_PCIEVCCAP5: u64 = (0x040114);
pub const SH4A_PCIEVCCAP6: u64 = (0x040118);
pub const SH4A_PCIEVCCAP7: u64 = (0x04011C);
pub const SH4A_PCIEVCCAP8: u64 = (0x040120);
pub const SH4A_PCIEVCCAP9: u64 = (0x040124);
pub const SH4A_PCIENUMCAP0: u64 = (0x0001B0);
pub const SH4A_PCIENUMCAP1: u64 = (0x0001B4);
pub const SH4A_PCIENUMCAP2: u64 = (0x0001B8);
pub const SH4A_PCIEIDSETR0: u64 = (0x041000);
pub const SH4A_PCIEIDSETR1: u64 = (0x041004);
pub const SH4A_PCIEBAR0SETR: u64 = (0x041008);
pub const SH4A_PCIEBAR1SETR: u64 = (0x04100C);
pub const SH4A_PCIEBAR2SETR: u64 = (0x041010);
pub const SH4A_PCIEBAR3SETR: u64 = (0x041014);
pub const SH4A_PCIEBAR4SETR: u64 = (0x041018);
pub const SH4A_PCIEBAR5SETR: u64 = (0x04101C);
pub const SH4A_PCIECISSETR: u64 = (0x041020);
pub const SH4A_PCIEIDSETR2: u64 = (0x041024);
pub const SH4A_PCIEEROMSETR: u64 = (0x041028);
pub const SH4A_PCIEDSERSETR0: u64 = (0x04102C);
pub const SH4A_PCIEDSERSETR1: u64 = (0x041030);
pub const SH4A_PCIECTLR: u64 = (0x041040);
pub const SH4A_PCIETLSR: u64 = (0x041044);
pub const SH4A_PCIETLCTLR: u64 = (0x041048);
pub const SH4A_PCIEDLSR: u64 = (0x04104C);
pub const SH4A_PCIEDLCTLR: u64 = (0x041050);
pub const SH4A_PCIEMACSR: u64 = (0x041054);
pub const SH4A_PCIEMACCTLR: u64 = (0x041058);
pub const PCIEMACCTLR_SCR_DIS: u64 = (1 << 27);
pub const SH4A_PCIEPMSTR: u64 = (0x04105C);
pub const SH4A_PCIEPMCTLR: u64 = (0x041060);
pub const SH4A_PCIETLINTENR: u64 = (0x041064);
pub const SH4A_PCIEDLINTENR: u64 = (0x041068);
pub const PCIEDLINTENR_DLL_ACT_ENABLE: u64 = (1 << 31);
pub const SH4A_PCIEMACINTENR: u64 = (0x04106C);
pub const SH4A_PCIEPMINTENR: u64 = (0x041070);
pub const SH4A_PCIETXDCTLR: u64 = (0x044000);
pub const SH4A_PCIETXCTLR: u64 = (0x044020);
pub const SH4A_PCIETXSR: u64 = (0x044028);
pub const SH4A_PCIETXVC0DCTLR: u64 = (0x044100);
pub const SH4A_PCIETXVC0SR: u64 = (0x044108);
pub const SH4A_PCIEVC0PDTXR: u64 = (0x044110);
pub const SH4A_PCIEVC0PHTXR: u64 = (0x044118);
pub const SH4A_PCIEVC0NPDTXR: u64 = (0x044120);
pub const SH4A_PCIEVC0NPHTXR: u64 = (0x044128);
pub const SH4A_PCIEVC0CDTXR: u64 = (0x044130);
pub const SH4A_PCIEVC0CHTXR: u64 = (0x044138);
pub const SH4A_PCIETXVCXDCTLR: u64 = (0x044200);
pub const SH4A_PCIETXVCXSR: u64 = (0x044208);
pub const SH4A_PCIEVCXPDTXR: u64 = (0x044210);
pub const SH4A_PCIEVCXPHTXR: u64 = (0x044218);
pub const SH4A_PCIEVCXNPDTXR: u64 = (0x044220);
pub const SH4A_PCIEVCXNPHTXR: u64 = (0x044228);
pub const SH4A_PCIEVCXCDTXR: u64 = (0x044230);
pub const SH4A_PCIEVCXCHTXR: u64 = (0x044238);
pub const SH4A_PCIERDCTLR: u64 = (0x046000);
pub const SH4A_PCIEERPCTLR: u64 = (0x046008);
pub const SH4A_PCIEERPHR: u64 = (0x046010);
pub const SH4A_PCIEERPERR: u64 = (0x046018);
pub const SH4A_PCIERXVC0DCTLR: u64 = (0x046100);
pub const SH4A_PCIERXVC0SR: u64 = (0x046108);
pub const SH4A_PCIEVC0PDRXR: u64 = (0x046140);
pub const SH4A_PCIEVC0PHRXR: u64 = (0x046148);
pub const SH4A_PCIEVC0PERR: u64 = (0x046150);
pub const SH4A_PCIEVC0NPDRXR: u64 = (0x046158);
pub const SH4A_PCIEVC0NPHRXR: u64 = (0x046160);
pub const SH4A_PCIEVC0NPERR: u64 = (0x046168);
pub const SH4A_PCIEVC0CDRXR: u64 = (0x046170);
pub const SH4A_PCIEVC0CHRXR: u64 = (0x046178);
pub const SH4A_PCIEVC0CERR: u64 = (0x046180);
pub const SH4A_PCIERXVCXDCTLR: u64 = (0x046200);
pub const SH4A_PCIERXVCXSR: u64 = (0x046208);
pub const SH4A_PCIEVCXPDRXR: u64 = (0x046240);
pub const SH4A_PCIEVCXPHRXR: u64 = (0x046248);
pub const SH4A_PCIEVCXPERR: u64 = (0x046250);
pub const SH4A_PCIEVCXNPDRXR: u64 = (0x046258);
pub const SH4A_PCIEVCXNPHRXR: u64 = (0x046260);
pub const SH4A_PCIEVCXNPERR: u64 = (0x046268);
pub const SH4A_PCIEVCXCDRXR: u64 = (0x046270);
pub const SH4A_PCIEVCXCHRXR: u64 = (0x046278);
pub const SH4A_PCIEVCXCERR: u64 = (0x046280);

/* SSI Register Definition for MSI WORK AROUND --hamada */
pub const SH4A_PCI_SSI_BASE: u64 = 0xFFE00000;
pub const SH4A_PCI_SSI_BASE_LEN: u64 = 0x00100000;

pub const SH4A_SSICR0: u64 = (0x000000);
pub const SH4A_SSICR1: u64 = (0x010000);
pub const SH4A_SSICR2: u64 = (0x020000);
pub const SH4A_SSICR3: u64 = (0x030000);

macro_rules! PCI_REG { ($x:expr) => { ((($x) + 0x40000)) }; }

extern "C" {
    fn __raw_writel(val: u64, addr: u64);
    fn __raw_readl(addr: u64) -> u64;
}
#[repr(C)]
pub struct pci_channel { pub reg_base: u64 }
pub unsafe fn pci_write_reg(chan: *mut pci_channel, val: u64, reg: u64) {
    __raw_writel(val, (*chan).reg_base + reg);
}
pub unsafe fn pci_read_reg(chan: *mut pci_channel, reg: u64) -> u64 {
    __raw_readl((*chan).reg_base + reg)
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
