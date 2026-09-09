// Translated from gt64120.h.
// C includes and build-time dependency mach-gt64120.h are intentionally omitted; their symbols remain external dependencies.
/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * Copyright (C) 2000, 2004, 2005  MIPS Technologies, Inc.
 *	All rights reserved.
 *	Authors: Carsten Langgaard <carstenl@mips.com>
 *		 Maciej W. Rozycki <macro@mips.com>
 * Copyright (C) 2005 Ralf Baechle (ralf@linux-mips.org)
 */

// #include <asm/addrspace.h>
// #include <asm/byteorder.h>

pub const fn MSK(n: u32) -> u32 { (1u32.wrapping_shl(n)).wrapping_sub(1) }

/*
 *  Register offset addresses
 */
/* CPU Configuration.  */
pub const GT_CPU_OFS: u32 = 0x000;

pub const GT_MULTI_OFS: u32 = 0x120;

/* CPU Address Decode.	*/
pub const GT_SCS10LD_OFS: u32 = 0x008;
pub const GT_SCS10HD_OFS: u32 = 0x010;
pub const GT_SCS32LD_OFS: u32 = 0x018;
pub const GT_SCS32HD_OFS: u32 = 0x020;
pub const GT_CS20LD_OFS: u32 = 0x028;
pub const GT_CS20HD_OFS: u32 = 0x030;
pub const GT_CS3BOOTLD_OFS: u32 = 0x038;
pub const GT_CS3BOOTHD_OFS: u32 = 0x040;
pub const GT_PCI0IOLD_OFS: u32 = 0x048;
pub const GT_PCI0IOHD_OFS: u32 = 0x050;
pub const GT_PCI0M0LD_OFS: u32 = 0x058;
pub const GT_PCI0M0HD_OFS: u32 = 0x060;
pub const GT_ISD_OFS: u32 = 0x068;

pub const GT_PCI0M1LD_OFS: u32 = 0x080;
pub const GT_PCI0M1HD_OFS: u32 = 0x088;
pub const GT_PCI1IOLD_OFS: u32 = 0x090;
pub const GT_PCI1IOHD_OFS: u32 = 0x098;
pub const GT_PCI1M0LD_OFS: u32 = 0x0a0;
pub const GT_PCI1M0HD_OFS: u32 = 0x0a8;
pub const GT_PCI1M1LD_OFS: u32 = 0x0b0;
pub const GT_PCI1M1HD_OFS: u32 = 0x0b8;
pub const GT_PCI1M1LD_OFS: u32 = 0x0b0;
pub const GT_PCI1M1HD_OFS: u32 = 0x0b8;

pub const GT_SCS10AR_OFS: u32 = 0x0d0;
pub const GT_SCS32AR_OFS: u32 = 0x0d8;
pub const GT_CS20R_OFS: u32 = 0x0e0;
pub const GT_CS3BOOTR_OFS: u32 = 0x0e8;

pub const GT_PCI0IOREMAP_OFS: u32 = 0x0f0;
pub const GT_PCI0M0REMAP_OFS: u32 = 0x0f8;
pub const GT_PCI0M1REMAP_OFS: u32 = 0x100;
pub const GT_PCI1IOREMAP_OFS: u32 = 0x108;
pub const GT_PCI1M0REMAP_OFS: u32 = 0x110;
pub const GT_PCI1M1REMAP_OFS: u32 = 0x118;

/* CPU Error Report.  */
pub const GT_CPUERR_ADDRLO_OFS: u32 = 0x070;
pub const GT_CPUERR_ADDRHI_OFS: u32 = 0x078;

pub const GT_CPUERR_DATALO_OFS: u32 = 0x128			/* GT-64120A only  */;
pub const GT_CPUERR_DATAHI_OFS: u32 = 0x130			/* GT-64120A only  */;
pub const GT_CPUERR_PARITY_OFS: u32 = 0x138			/* GT-64120A only  */;

/* CPU Sync Barrier.  */
pub const GT_PCI0SYNC_OFS: u32 = 0x0c0;
pub const GT_PCI1SYNC_OFS: u32 = 0x0c8;

/* SDRAM and Device Address Decode.  */
pub const GT_SCS0LD_OFS: u32 = 0x400;
pub const GT_SCS0HD_OFS: u32 = 0x404;
pub const GT_SCS1LD_OFS: u32 = 0x408;
pub const GT_SCS1HD_OFS: u32 = 0x40c;
pub const GT_SCS2LD_OFS: u32 = 0x410;
pub const GT_SCS2HD_OFS: u32 = 0x414;
pub const GT_SCS3LD_OFS: u32 = 0x418;
pub const GT_SCS3HD_OFS: u32 = 0x41c;
pub const GT_CS0LD_OFS: u32 = 0x420;
pub const GT_CS0HD_OFS: u32 = 0x424;
pub const GT_CS1LD_OFS: u32 = 0x428;
pub const GT_CS1HD_OFS: u32 = 0x42c;
pub const GT_CS2LD_OFS: u32 = 0x430;
pub const GT_CS2HD_OFS: u32 = 0x434;
pub const GT_CS3LD_OFS: u32 = 0x438;
pub const GT_CS3HD_OFS: u32 = 0x43c;
pub const GT_BOOTLD_OFS: u32 = 0x440;
pub const GT_BOOTHD_OFS: u32 = 0x444;

pub const GT_ADERR_OFS: u32 = 0x470;

/* SDRAM Configuration.	 */
pub const GT_SDRAM_CFG_OFS: u32 = 0x448;

pub const GT_SDRAM_OPMODE_OFS: u32 = 0x474;
pub const GT_SDRAM_BM_OFS: u32 = 0x478;
pub const GT_SDRAM_ADDRDECODE_OFS: u32 = 0x47c;

/* SDRAM Parameters.  */
pub const GT_SDRAM_B0_OFS: u32 = 0x44c;
pub const GT_SDRAM_B1_OFS: u32 = 0x450;
pub const GT_SDRAM_B2_OFS: u32 = 0x454;
pub const GT_SDRAM_B3_OFS: u32 = 0x458;

/* Device Parameters.  */
pub const GT_DEV_B0_OFS: u32 = 0x45c;
pub const GT_DEV_B1_OFS: u32 = 0x460;
pub const GT_DEV_B2_OFS: u32 = 0x464;
pub const GT_DEV_B3_OFS: u32 = 0x468;
pub const GT_DEV_BOOT_OFS: u32 = 0x46c;

/* ECC.	 */
pub const GT_ECC_ERRDATALO: u32 = 0x480			/* GT-64120A only  */;
pub const GT_ECC_ERRDATAHI: u32 = 0x484			/* GT-64120A only  */;
pub const GT_ECC_MEM: u32 = 0x488			/* GT-64120A only  */;
pub const GT_ECC_CALC: u32 = 0x48c			/* GT-64120A only  */;
pub const GT_ECC_ERRADDR: u32 = 0x490			/* GT-64120A only  */;

/* DMA Record.	*/
pub const GT_DMA0_CNT_OFS: u32 = 0x800;
pub const GT_DMA1_CNT_OFS: u32 = 0x804;
pub const GT_DMA2_CNT_OFS: u32 = 0x808;
pub const GT_DMA3_CNT_OFS: u32 = 0x80c;
pub const GT_DMA0_SA_OFS: u32 = 0x810;
pub const GT_DMA1_SA_OFS: u32 = 0x814;
pub const GT_DMA2_SA_OFS: u32 = 0x818;
pub const GT_DMA3_SA_OFS: u32 = 0x81c;
pub const GT_DMA0_DA_OFS: u32 = 0x820;
pub const GT_DMA1_DA_OFS: u32 = 0x824;
pub const GT_DMA2_DA_OFS: u32 = 0x828;
pub const GT_DMA3_DA_OFS: u32 = 0x82c;
pub const GT_DMA0_NEXT_OFS: u32 = 0x830;
pub const GT_DMA1_NEXT_OFS: u32 = 0x834;
pub const GT_DMA2_NEXT_OFS: u32 = 0x838;
pub const GT_DMA3_NEXT_OFS: u32 = 0x83c;

pub const GT_DMA0_CUR_OFS: u32 = 0x870;
pub const GT_DMA1_CUR_OFS: u32 = 0x874;
pub const GT_DMA2_CUR_OFS: u32 = 0x878;
pub const GT_DMA3_CUR_OFS: u32 = 0x87c;

/* DMA Channel Control.	 */
pub const GT_DMA0_CTRL_OFS: u32 = 0x840;
pub const GT_DMA1_CTRL_OFS: u32 = 0x844;
pub const GT_DMA2_CTRL_OFS: u32 = 0x848;
pub const GT_DMA3_CTRL_OFS: u32 = 0x84c;

/* DMA Arbiter.	 */
pub const GT_DMA_ARB_OFS: u32 = 0x860;

/* Timer/Counter.  */
pub const GT_TC0_OFS: u32 = 0x850;
pub const GT_TC1_OFS: u32 = 0x854;
pub const GT_TC2_OFS: u32 = 0x858;
pub const GT_TC3_OFS: u32 = 0x85c;

pub const GT_TC_CONTROL_OFS: u32 = 0x864;

/* PCI Internal.  */
pub const GT_PCI0_CMD_OFS: u32 = 0xc00;
pub const GT_PCI0_TOR_OFS: u32 = 0xc04;
pub const GT_PCI0_BS_SCS10_OFS: u32 = 0xc08;
pub const GT_PCI0_BS_SCS32_OFS: u32 = 0xc0c;
pub const GT_PCI0_BS_CS20_OFS: u32 = 0xc10;
pub const GT_PCI0_BS_CS3BT_OFS: u32 = 0xc14;

pub const GT_PCI1_IACK_OFS: u32 = 0xc30;
pub const GT_PCI0_IACK_OFS: u32 = 0xc34;

pub const GT_PCI0_BARE_OFS: u32 = 0xc3c;
pub const GT_PCI0_PREFMBR_OFS: u32 = 0xc40;

pub const GT_PCI0_SCS10_BAR_OFS: u32 = 0xc48;
pub const GT_PCI0_SCS32_BAR_OFS: u32 = 0xc4c;
pub const GT_PCI0_CS20_BAR_OFS: u32 = 0xc50;
pub const GT_PCI0_CS3BT_BAR_OFS: u32 = 0xc54;
pub const GT_PCI0_SSCS10_BAR_OFS: u32 = 0xc58;
pub const GT_PCI0_SSCS32_BAR_OFS: u32 = 0xc5c;

pub const GT_PCI0_SCS3BT_BAR_OFS: u32 = 0xc64;

pub const GT_PCI1_CMD_OFS: u32 = 0xc80;
pub const GT_PCI1_TOR_OFS: u32 = 0xc84;
pub const GT_PCI1_BS_SCS10_OFS: u32 = 0xc88;
pub const GT_PCI1_BS_SCS32_OFS: u32 = 0xc8c;
pub const GT_PCI1_BS_CS20_OFS: u32 = 0xc90;
pub const GT_PCI1_BS_CS3BT_OFS: u32 = 0xc94;

pub const GT_PCI1_BARE_OFS: u32 = 0xcbc;
pub const GT_PCI1_PREFMBR_OFS: u32 = 0xcc0;

pub const GT_PCI1_SCS10_BAR_OFS: u32 = 0xcc8;
pub const GT_PCI1_SCS32_BAR_OFS: u32 = 0xccc;
pub const GT_PCI1_CS20_BAR_OFS: u32 = 0xcd0;
pub const GT_PCI1_CS3BT_BAR_OFS: u32 = 0xcd4;
pub const GT_PCI1_SSCS10_BAR_OFS: u32 = 0xcd8;
pub const GT_PCI1_SSCS32_BAR_OFS: u32 = 0xcdc;

pub const GT_PCI1_SCS3BT_BAR_OFS: u32 = 0xce4;

pub const GT_PCI1_CFGADDR_OFS: u32 = 0xcf0;
pub const GT_PCI1_CFGDATA_OFS: u32 = 0xcf4;
pub const GT_PCI0_CFGADDR_OFS: u32 = 0xcf8;
pub const GT_PCI0_CFGDATA_OFS: u32 = 0xcfc;

/* Interrupts.	*/
pub const GT_INTRCAUSE_OFS: u32 = 0xc18;
pub const GT_INTRMASK_OFS: u32 = 0xc1c;

pub const GT_PCI0_ICMASK_OFS: u32 = 0xc24;
pub const GT_PCI0_SERR0MASK_OFS: u32 = 0xc28;

pub const GT_CPU_INTSEL_OFS: u32 = 0xc70;
pub const GT_PCI0_INTSEL_OFS: u32 = 0xc74;

pub const GT_HINTRCAUSE_OFS: u32 = 0xc98;
pub const GT_HINTRMASK_OFS: u32 = 0xc9c;

pub const GT_PCI0_HICMASK_OFS: u32 = 0xca4;
pub const GT_PCI1_SERR1MASK_OFS: u32 = 0xca8;


/*
 * I2O Support Registers
 */
pub const INBOUND_MESSAGE_REGISTER0_PCI_SIDE: u32 = 0x010;
pub const INBOUND_MESSAGE_REGISTER1_PCI_SIDE: u32 = 0x014;
pub const OUTBOUND_MESSAGE_REGISTER0_PCI_SIDE: u32 = 0x018;
pub const OUTBOUND_MESSAGE_REGISTER1_PCI_SIDE: u32 = 0x01c;
pub const INBOUND_DOORBELL_REGISTER_PCI_SIDE: u32 = 0x020;
pub const INBOUND_INTERRUPT_CAUSE_REGISTER_PCI_SIDE: u32 = 0x024;
pub const INBOUND_INTERRUPT_MASK_REGISTER_PCI_SIDE: u32 = 0x028;
pub const OUTBOUND_DOORBELL_REGISTER_PCI_SIDE: u32 = 0x02c;
pub const OUTBOUND_INTERRUPT_CAUSE_REGISTER_PCI_SIDE: u32 = 0x030;
pub const OUTBOUND_INTERRUPT_MASK_REGISTER_PCI_SIDE: u32 = 0x034;
pub const INBOUND_QUEUE_PORT_VIRTUAL_REGISTER_PCI_SIDE: u32 = 0x040;
pub const OUTBOUND_QUEUE_PORT_VIRTUAL_REGISTER_PCI_SIDE: u32 = 0x044;
pub const QUEUE_CONTROL_REGISTER_PCI_SIDE: u32 = 0x050;
pub const QUEUE_BASE_ADDRESS_REGISTER_PCI_SIDE: u32 = 0x054;
pub const INBOUND_FREE_HEAD_POINTER_REGISTER_PCI_SIDE: u32 = 0x060;
pub const INBOUND_FREE_TAIL_POINTER_REGISTER_PCI_SIDE: u32 = 0x064;
pub const INBOUND_POST_HEAD_POINTER_REGISTER_PCI_SIDE: u32 = 0x068;
pub const INBOUND_POST_TAIL_POINTER_REGISTER_PCI_SIDE: u32 = 0x06c;
pub const OUTBOUND_FREE_HEAD_POINTER_REGISTER_PCI_SIDE: u32 = 0x070;
pub const OUTBOUND_FREE_TAIL_POINTER_REGISTER_PCI_SIDE: u32 = 0x074;
pub const OUTBOUND_POST_HEAD_POINTER_REGISTER_PCI_SIDE: u32 = 0x078;
pub const OUTBOUND_POST_TAIL_POINTER_REGISTER_PCI_SIDE: u32 = 0x07c;

pub const INBOUND_MESSAGE_REGISTER0_CPU_SIDE: u32 = 0x1c10;
pub const INBOUND_MESSAGE_REGISTER1_CPU_SIDE: u32 = 0x1c14;
pub const OUTBOUND_MESSAGE_REGISTER0_CPU_SIDE: u32 = 0x1c18;
pub const OUTBOUND_MESSAGE_REGISTER1_CPU_SIDE: u32 = 0x1c1c;
pub const INBOUND_DOORBELL_REGISTER_CPU_SIDE: u32 = 0x1c20;
pub const INBOUND_INTERRUPT_CAUSE_REGISTER_CPU_SIDE: u32 = 0x1c24;
pub const INBOUND_INTERRUPT_MASK_REGISTER_CPU_SIDE: u32 = 0x1c28;
pub const OUTBOUND_DOORBELL_REGISTER_CPU_SIDE: u32 = 0x1c2c;
pub const OUTBOUND_INTERRUPT_CAUSE_REGISTER_CPU_SIDE: u32 = 0x1c30;
pub const OUTBOUND_INTERRUPT_MASK_REGISTER_CPU_SIDE: u32 = 0x1c34;
pub const INBOUND_QUEUE_PORT_VIRTUAL_REGISTER_CPU_SIDE: u32 = 0x1c40;
pub const OUTBOUND_QUEUE_PORT_VIRTUAL_REGISTER_CPU_SIDE: u32 = 0x1c44;
pub const QUEUE_CONTROL_REGISTER_CPU_SIDE: u32 = 0x1c50;
pub const QUEUE_BASE_ADDRESS_REGISTER_CPU_SIDE: u32 = 0x1c54;
pub const INBOUND_FREE_HEAD_POINTER_REGISTER_CPU_SIDE: u32 = 0x1c60;
pub const INBOUND_FREE_TAIL_POINTER_REGISTER_CPU_SIDE: u32 = 0x1c64;
pub const INBOUND_POST_HEAD_POINTER_REGISTER_CPU_SIDE: u32 = 0x1c68;
pub const INBOUND_POST_TAIL_POINTER_REGISTER_CPU_SIDE: u32 = 0x1c6c;
pub const OUTBOUND_FREE_HEAD_POINTER_REGISTER_CPU_SIDE: u32 = 0x1c70;
pub const OUTBOUND_FREE_TAIL_POINTER_REGISTER_CPU_SIDE: u32 = 0x1c74;
pub const OUTBOUND_POST_HEAD_POINTER_REGISTER_CPU_SIDE: u32 = 0x1c78;
pub const OUTBOUND_POST_TAIL_POINTER_REGISTER_CPU_SIDE: u32 = 0x1c7c;

/*
 *  Register encodings
 */
pub const GT_CPU_ENDIAN_SHF: u32 = 12;
pub const GT_CPU_ENDIAN_MSK: u32 = MSK(1) << GT_CPU_ENDIAN_SHF;
pub const GT_CPU_ENDIAN_BIT: u32 = GT_CPU_ENDIAN_MSK;
pub const GT_CPU_WR_SHF: u32 = 16;
pub const GT_CPU_WR_MSK: u32 = MSK(1) << GT_CPU_WR_SHF;
pub const GT_CPU_WR_BIT: u32 = GT_CPU_WR_MSK;
pub const GT_CPU_WR_DXDXDXDX: u32 = 0;
pub const GT_CPU_WR_DDDD: u32 = 1;


pub const GT_PCI_DCRM_SHF: u32 = 21;
pub const GT_PCI_LD_SHF: u32 = 0;
pub const GT_PCI_LD_MSK: u32 = MSK(15) << GT_PCI_LD_SHF;
pub const GT_PCI_HD_SHF: u32 = 0;
pub const GT_PCI_HD_MSK: u32 = MSK(7) << GT_PCI_HD_SHF;
pub const GT_PCI_REMAP_SHF: u32 = 0;
pub const GT_PCI_REMAP_MSK: u32 = MSK(11) << GT_PCI_REMAP_SHF;


pub const GT_CFGADDR_CFGEN_SHF: u32 = 31;
pub const GT_CFGADDR_CFGEN_MSK: u32 = MSK(1) << GT_CFGADDR_CFGEN_SHF;
pub const GT_CFGADDR_CFGEN_BIT: u32 = GT_CFGADDR_CFGEN_MSK;

pub const GT_CFGADDR_BUSNUM_SHF: u32 = 16;
pub const GT_CFGADDR_BUSNUM_MSK: u32 = MSK(8) << GT_CFGADDR_BUSNUM_SHF;

pub const GT_CFGADDR_DEVNUM_SHF: u32 = 11;
pub const GT_CFGADDR_DEVNUM_MSK: u32 = MSK(5) << GT_CFGADDR_DEVNUM_SHF;

pub const GT_CFGADDR_FUNCNUM_SHF: u32 = 8;
pub const GT_CFGADDR_FUNCNUM_MSK: u32 = MSK(3) << GT_CFGADDR_FUNCNUM_SHF;

pub const GT_CFGADDR_REGNUM_SHF: u32 = 2;
pub const GT_CFGADDR_REGNUM_MSK: u32 = MSK(6) << GT_CFGADDR_REGNUM_SHF;


pub const GT_SDRAM_BM_ORDER_SHF: u32 = 2;
pub const GT_SDRAM_BM_ORDER_MSK: u32 = MSK(1) << GT_SDRAM_BM_ORDER_SHF;
pub const GT_SDRAM_BM_ORDER_BIT: u32 = GT_SDRAM_BM_ORDER_MSK;
pub const GT_SDRAM_BM_ORDER_SUB: u32 = 1;
pub const GT_SDRAM_BM_ORDER_LIN: u32 = 0;

pub const GT_SDRAM_BM_RSVD_ALL1: u32 = 0xffb;


pub const GT_SDRAM_ADDRDECODE_ADDR_SHF: u32 = 0;
pub const GT_SDRAM_ADDRDECODE_ADDR_MSK: u32 = MSK(3) << GT_SDRAM_ADDRDECODE_ADDR_SHF;
pub const GT_SDRAM_ADDRDECODE_ADDR_0: u32 = 0;
pub const GT_SDRAM_ADDRDECODE_ADDR_1: u32 = 1;
pub const GT_SDRAM_ADDRDECODE_ADDR_2: u32 = 2;
pub const GT_SDRAM_ADDRDECODE_ADDR_3: u32 = 3;
pub const GT_SDRAM_ADDRDECODE_ADDR_4: u32 = 4;
pub const GT_SDRAM_ADDRDECODE_ADDR_5: u32 = 5;
pub const GT_SDRAM_ADDRDECODE_ADDR_6: u32 = 6;
pub const GT_SDRAM_ADDRDECODE_ADDR_7: u32 = 7;


pub const GT_SDRAM_B0_CASLAT_SHF: u32 = 0;
pub const GT_SDRAM_B0_CASLAT_MSK: u32 = MSK(2) << GT_SDRAM_B0__SHF;
pub const GT_SDRAM_B0_CASLAT_2: u32 = 1;
pub const GT_SDRAM_B0_CASLAT_3: u32 = 2;

pub const GT_SDRAM_B0_FTDIS_SHF: u32 = 2;
pub const GT_SDRAM_B0_FTDIS_MSK: u32 = MSK(1) << GT_SDRAM_B0_FTDIS_SHF;
pub const GT_SDRAM_B0_FTDIS_BIT: u32 = GT_SDRAM_B0_FTDIS_MSK;

pub const GT_SDRAM_B0_SRASPRCHG_SHF: u32 = 3;
pub const GT_SDRAM_B0_SRASPRCHG_MSK: u32 = MSK(1) << GT_SDRAM_B0_SRASPRCHG_SHF;
pub const GT_SDRAM_B0_SRASPRCHG_BIT: u32 = GT_SDRAM_B0_SRASPRCHG_MSK;
pub const GT_SDRAM_B0_SRASPRCHG_2: u32 = 0;
pub const GT_SDRAM_B0_SRASPRCHG_3: u32 = 1;

pub const GT_SDRAM_B0_B0COMPAB_SHF: u32 = 4;
pub const GT_SDRAM_B0_B0COMPAB_MSK: u32 = MSK(1) << GT_SDRAM_B0_B0COMPAB_SHF;
pub const GT_SDRAM_B0_B0COMPAB_BIT: u32 = GT_SDRAM_B0_B0COMPAB_MSK;

pub const GT_SDRAM_B0_64BITINT_SHF: u32 = 5;
pub const GT_SDRAM_B0_64BITINT_MSK: u32 = MSK(1) << GT_SDRAM_B0_64BITINT_SHF;
pub const GT_SDRAM_B0_64BITINT_BIT: u32 = GT_SDRAM_B0_64BITINT_MSK;
pub const GT_SDRAM_B0_64BITINT_2: u32 = 0;
pub const GT_SDRAM_B0_64BITINT_4: u32 = 1;

pub const GT_SDRAM_B0_BW_SHF: u32 = 6;
pub const GT_SDRAM_B0_BW_MSK: u32 = MSK(1) << GT_SDRAM_B0_BW_SHF;
pub const GT_SDRAM_B0_BW_BIT: u32 = GT_SDRAM_B0_BW_MSK;
pub const GT_SDRAM_B0_BW_32: u32 = 0;
pub const GT_SDRAM_B0_BW_64: u32 = 1;

pub const GT_SDRAM_B0_BLODD_SHF: u32 = 7;
pub const GT_SDRAM_B0_BLODD_MSK: u32 = MSK(1) << GT_SDRAM_B0_BLODD_SHF;
pub const GT_SDRAM_B0_BLODD_BIT: u32 = GT_SDRAM_B0_BLODD_MSK;

pub const GT_SDRAM_B0_PAR_SHF: u32 = 8;
pub const GT_SDRAM_B0_PAR_MSK: u32 = MSK(1) << GT_SDRAM_B0_PAR_SHF;
pub const GT_SDRAM_B0_PAR_BIT: u32 = GT_SDRAM_B0_PAR_MSK;

pub const GT_SDRAM_B0_BYPASS_SHF: u32 = 9;
pub const GT_SDRAM_B0_BYPASS_MSK: u32 = MSK(1) << GT_SDRAM_B0_BYPASS_SHF;
pub const GT_SDRAM_B0_BYPASS_BIT: u32 = GT_SDRAM_B0_BYPASS_MSK;

pub const GT_SDRAM_B0_SRAS2SCAS_SHF: u32 = 10;
pub const GT_SDRAM_B0_SRAS2SCAS_MSK: u32 = MSK(1) << GT_SDRAM_B0_SRAS2SCAS_SHF;
pub const GT_SDRAM_B0_SRAS2SCAS_BIT: u32 = GT_SDRAM_B0_SRAS2SCAS_MSK;
pub const GT_SDRAM_B0_SRAS2SCAS_2: u32 = 0;
pub const GT_SDRAM_B0_SRAS2SCAS_3: u32 = 1;

pub const GT_SDRAM_B0_SIZE_SHF: u32 = 11;
pub const GT_SDRAM_B0_SIZE_MSK: u32 = MSK(1) << GT_SDRAM_B0_SIZE_SHF;
pub const GT_SDRAM_B0_SIZE_BIT: u32 = GT_SDRAM_B0_SIZE_MSK;
pub const GT_SDRAM_B0_SIZE_16M: u32 = 0;
pub const GT_SDRAM_B0_SIZE_64M: u32 = 1;

pub const GT_SDRAM_B0_EXTPAR_SHF: u32 = 12;
pub const GT_SDRAM_B0_EXTPAR_MSK: u32 = MSK(1) << GT_SDRAM_B0_EXTPAR_SHF;
pub const GT_SDRAM_B0_EXTPAR_BIT: u32 = GT_SDRAM_B0_EXTPAR_MSK;

pub const GT_SDRAM_B0_BLEN_SHF: u32 = 13;
pub const GT_SDRAM_B0_BLEN_MSK: u32 = MSK(1) << GT_SDRAM_B0_BLEN_SHF;
pub const GT_SDRAM_B0_BLEN_BIT: u32 = GT_SDRAM_B0_BLEN_MSK;
pub const GT_SDRAM_B0_BLEN_8: u32 = 0;
pub const GT_SDRAM_B0_BLEN_4: u32 = 1;


pub const GT_SDRAM_CFG_REFINT_SHF: u32 = 0;
pub const GT_SDRAM_CFG_REFINT_MSK: u32 = MSK(14) << GT_SDRAM_CFG_REFINT_SHF;

pub const GT_SDRAM_CFG_NINTERLEAVE_SHF: u32 = 14;
pub const GT_SDRAM_CFG_NINTERLEAVE_MSK: u32 = MSK(1) << GT_SDRAM_CFG_NINTERLEAVE_SHF;
pub const GT_SDRAM_CFG_NINTERLEAVE_BIT: u32 = GT_SDRAM_CFG_NINTERLEAVE_MSK;

pub const GT_SDRAM_CFG_RMW_SHF: u32 = 15;
pub const GT_SDRAM_CFG_RMW_MSK: u32 = MSK(1) << GT_SDRAM_CFG_RMW_SHF;
pub const GT_SDRAM_CFG_RMW_BIT: u32 = GT_SDRAM_CFG_RMW_MSK;

pub const GT_SDRAM_CFG_NONSTAGREF_SHF: u32 = 16;
pub const GT_SDRAM_CFG_NONSTAGREF_MSK: u32 = MSK(1) << GT_SDRAM_CFG_NONSTAGREF_SHF;
pub const GT_SDRAM_CFG_NONSTAGREF_BIT: u32 = GT_SDRAM_CFG_NONSTAGREF_MSK;

pub const GT_SDRAM_CFG_DUPCNTL_SHF: u32 = 19;
pub const GT_SDRAM_CFG_DUPCNTL_MSK: u32 = MSK(1) << GT_SDRAM_CFG_DUPCNTL_SHF;
pub const GT_SDRAM_CFG_DUPCNTL_BIT: u32 = GT_SDRAM_CFG_DUPCNTL_MSK;

pub const GT_SDRAM_CFG_DUPBA_SHF: u32 = 20;
pub const GT_SDRAM_CFG_DUPBA_MSK: u32 = MSK(1) << GT_SDRAM_CFG_DUPBA_SHF;
pub const GT_SDRAM_CFG_DUPBA_BIT: u32 = GT_SDRAM_CFG_DUPBA_MSK;

pub const GT_SDRAM_CFG_DUPEOT0_SHF: u32 = 21;
pub const GT_SDRAM_CFG_DUPEOT0_MSK: u32 = MSK(1) << GT_SDRAM_CFG_DUPEOT0_SHF;
pub const GT_SDRAM_CFG_DUPEOT0_BIT: u32 = GT_SDRAM_CFG_DUPEOT0_MSK;

pub const GT_SDRAM_CFG_DUPEOT1_SHF: u32 = 22;
pub const GT_SDRAM_CFG_DUPEOT1_MSK: u32 = MSK(1) << GT_SDRAM_CFG_DUPEOT1_SHF;
pub const GT_SDRAM_CFG_DUPEOT1_BIT: u32 = GT_SDRAM_CFG_DUPEOT1_MSK;

pub const GT_SDRAM_OPMODE_OP_SHF: u32 = 0;
pub const GT_SDRAM_OPMODE_OP_MSK: u32 = MSK(3) << GT_SDRAM_OPMODE_OP_SHF;
pub const GT_SDRAM_OPMODE_OP_NORMAL: u32 = 0;
pub const GT_SDRAM_OPMODE_OP_NOP: u32 = 1;
pub const GT_SDRAM_OPMODE_OP_PRCHG: u32 = 2;
pub const GT_SDRAM_OPMODE_OP_MODE: u32 = 3;
pub const GT_SDRAM_OPMODE_OP_CBR: u32 = 4;

pub const GT_TC_CONTROL_ENTC0_SHF: u32 = 0;
pub const GT_TC_CONTROL_ENTC0_MSK: u32 = MSK(1) << GT_TC_CONTROL_ENTC0_SHF;
pub const GT_TC_CONTROL_ENTC0_BIT: u32 = GT_TC_CONTROL_ENTC0_MSK;
pub const GT_TC_CONTROL_SELTC0_SHF: u32 = 1;
pub const GT_TC_CONTROL_SELTC0_MSK: u32 = MSK(1) << GT_TC_CONTROL_SELTC0_SHF;
pub const GT_TC_CONTROL_SELTC0_BIT: u32 = GT_TC_CONTROL_SELTC0_MSK;


pub const GT_PCI0_BARE_SWSCS3BOOTDIS_SHF: u32 = 0;
pub const GT_PCI0_BARE_SWSCS3BOOTDIS_MSK: u32 = MSK(1) << GT_PCI0_BARE_SWSCS3BOOTDIS_SHF;
pub const GT_PCI0_BARE_SWSCS3BOOTDIS_BIT: u32 = GT_PCI0_BARE_SWSCS3BOOTDIS_MSK;

pub const GT_PCI0_BARE_SWSCS32DIS_SHF: u32 = 1;
pub const GT_PCI0_BARE_SWSCS32DIS_MSK: u32 = MSK(1) << GT_PCI0_BARE_SWSCS32DIS_SHF;
pub const GT_PCI0_BARE_SWSCS32DIS_BIT: u32 = GT_PCI0_BARE_SWSCS32DIS_MSK;

pub const GT_PCI0_BARE_SWSCS10DIS_SHF: u32 = 2;
pub const GT_PCI0_BARE_SWSCS10DIS_MSK: u32 = MSK(1) << GT_PCI0_BARE_SWSCS10DIS_SHF;
pub const GT_PCI0_BARE_SWSCS10DIS_BIT: u32 = GT_PCI0_BARE_SWSCS10DIS_MSK;

pub const GT_PCI0_BARE_INTIODIS_SHF: u32 = 3;
pub const GT_PCI0_BARE_INTIODIS_MSK: u32 = MSK(1) << GT_PCI0_BARE_INTIODIS_SHF;
pub const GT_PCI0_BARE_INTIODIS_BIT: u32 = GT_PCI0_BARE_INTIODIS_MSK;

pub const GT_PCI0_BARE_INTMEMDIS_SHF: u32 = 4;
pub const GT_PCI0_BARE_INTMEMDIS_MSK: u32 = MSK(1) << GT_PCI0_BARE_INTMEMDIS_SHF;
pub const GT_PCI0_BARE_INTMEMDIS_BIT: u32 = GT_PCI0_BARE_INTMEMDIS_MSK;

pub const GT_PCI0_BARE_CS3BOOTDIS_SHF: u32 = 5;
pub const GT_PCI0_BARE_CS3BOOTDIS_MSK: u32 = MSK(1) << GT_PCI0_BARE_CS3BOOTDIS_SHF;
pub const GT_PCI0_BARE_CS3BOOTDIS_BIT: u32 = GT_PCI0_BARE_CS3BOOTDIS_MSK;

pub const GT_PCI0_BARE_CS20DIS_SHF: u32 = 6;
pub const GT_PCI0_BARE_CS20DIS_MSK: u32 = MSK(1) << GT_PCI0_BARE_CS20DIS_SHF;
pub const GT_PCI0_BARE_CS20DIS_BIT: u32 = GT_PCI0_BARE_CS20DIS_MSK;

pub const GT_PCI0_BARE_SCS32DIS_SHF: u32 = 7;
pub const GT_PCI0_BARE_SCS32DIS_MSK: u32 = MSK(1) << GT_PCI0_BARE_SCS32DIS_SHF;
pub const GT_PCI0_BARE_SCS32DIS_BIT: u32 = GT_PCI0_BARE_SCS32DIS_MSK;

pub const GT_PCI0_BARE_SCS10DIS_SHF: u32 = 8;
pub const GT_PCI0_BARE_SCS10DIS_MSK: u32 = MSK(1) << GT_PCI0_BARE_SCS10DIS_SHF;
pub const GT_PCI0_BARE_SCS10DIS_BIT: u32 = GT_PCI0_BARE_SCS10DIS_MSK;


pub const GT_INTRCAUSE_MASABORT0_SHF: u32 = 18;
pub const GT_INTRCAUSE_MASABORT0_MSK: u32 = MSK(1) << GT_INTRCAUSE_MASABORT0_SHF;
pub const GT_INTRCAUSE_MASABORT0_BIT: u32 = GT_INTRCAUSE_MASABORT0_MSK;

pub const GT_INTRCAUSE_TARABORT0_SHF: u32 = 19;
pub const GT_INTRCAUSE_TARABORT0_MSK: u32 = MSK(1) << GT_INTRCAUSE_TARABORT0_SHF;
pub const GT_INTRCAUSE_TARABORT0_BIT: u32 = GT_INTRCAUSE_TARABORT0_MSK;


pub const GT_PCI0_CFGADDR_REGNUM_SHF: u32 = 2;
pub const GT_PCI0_CFGADDR_REGNUM_MSK: u32 = MSK(6) << GT_PCI0_CFGADDR_REGNUM_SHF;
pub const GT_PCI0_CFGADDR_FUNCTNUM_SHF: u32 = 8;
pub const GT_PCI0_CFGADDR_FUNCTNUM_MSK: u32 = MSK(3) << GT_PCI0_CFGADDR_FUNCTNUM_SHF;
pub const GT_PCI0_CFGADDR_DEVNUM_SHF: u32 = 11;
pub const GT_PCI0_CFGADDR_DEVNUM_MSK: u32 = MSK(5) << GT_PCI0_CFGADDR_DEVNUM_SHF;
pub const GT_PCI0_CFGADDR_BUSNUM_SHF: u32 = 16;
pub const GT_PCI0_CFGADDR_BUSNUM_MSK: u32 = MSK(8) << GT_PCI0_CFGADDR_BUSNUM_SHF;
pub const GT_PCI0_CFGADDR_CONFIGEN_SHF: u32 = 31;
pub const GT_PCI0_CFGADDR_CONFIGEN_MSK: u32 = MSK(1) << GT_PCI0_CFGADDR_CONFIGEN_SHF;
pub const GT_PCI0_CFGADDR_CONFIGEN_BIT: u32 = GT_PCI0_CFGADDR_CONFIGEN_MSK;

pub const GT_PCI0_CMD_MBYTESWAP_SHF: u32 = 0;
pub const GT_PCI0_CMD_MBYTESWAP_MSK: u32 = MSK(1) << GT_PCI0_CMD_MBYTESWAP_SHF;
pub const GT_PCI0_CMD_MBYTESWAP_BIT: u32 = GT_PCI0_CMD_MBYTESWAP_MSK;
pub const GT_PCI0_CMD_MWORDSWAP_SHF: u32 = 10;
pub const GT_PCI0_CMD_MWORDSWAP_MSK: u32 = MSK(1) << GT_PCI0_CMD_MWORDSWAP_SHF;
pub const GT_PCI0_CMD_MWORDSWAP_BIT: u32 = GT_PCI0_CMD_MWORDSWAP_MSK;
pub const GT_PCI0_CMD_SBYTESWAP_SHF: u32 = 16;
pub const GT_PCI0_CMD_SBYTESWAP_MSK: u32 = MSK(1) << GT_PCI0_CMD_SBYTESWAP_SHF;
pub const GT_PCI0_CMD_SBYTESWAP_BIT: u32 = GT_PCI0_CMD_SBYTESWAP_MSK;
pub const GT_PCI0_CMD_SWORDSWAP_SHF: u32 = 11;
pub const GT_PCI0_CMD_SWORDSWAP_MSK: u32 = MSK(1) << GT_PCI0_CMD_SWORDSWAP_SHF;
pub const GT_PCI0_CMD_SWORDSWAP_BIT: u32 = GT_PCI0_CMD_SWORDSWAP_MSK;

pub const GT_INTR_T0EXP_SHF: u32 = 8;
pub const GT_INTR_T0EXP_MSK: u32 = MSK(1) << GT_INTR_T0EXP_SHF;
pub const GT_INTR_T0EXP_BIT: u32 = GT_INTR_T0EXP_MSK;
pub const GT_INTR_RETRYCTR0_SHF: u32 = 20;
pub const GT_INTR_RETRYCTR0_MSK: u32 = MSK(1) << GT_INTR_RETRYCTR0_SHF;
pub const GT_INTR_RETRYCTR0_BIT: u32 = GT_INTR_RETRYCTR0_MSK;

/*
 *  Misc
 */
pub const GT_DEF_PCI0_IO_BASE: u32 = 0x10000000;
pub const GT_DEF_PCI0_IO_SIZE: u32 = 0x02000000;
pub const GT_DEF_PCI0_MEM0_BASE: u32 = 0x12000000;
pub const GT_DEF_PCI0_MEM0_SIZE: u32 = 0x02000000;
pub const GT_DEF_BASE: u32 = 0x14000000;

pub const GT_MAX_BANKSIZE: u32 = (256 * 1024 * 1024)	/* Max 256MB bank  */;
pub const GT_LATTIM_MIN: u32 = 6			/* Minimum lat	*/;

/*
 * The gt64120_dep.h file must define the following macros
 *
 *   GT_READ(ofs, data_pointer)
 *   GT_WRITE(ofs, data)	   - read/write GT64120 registers in 32bit
 *
 *   TIMER	- gt64120 timer irq, temporary solution until
 *		  full gt64120 cascade interrupt support is in place
 */

// #include <mach-gt64120.h>

/*
 * Because of an error/peculiarity in the Galileo chip, we need to swap the
 * bytes when running bigendian.  We also provide non-swapping versions.
 */
macro_rules! __GT_READ { ($ofs:expr) => { unsafe { core::ptr::read_volatile((GT64120_BASE + ($ofs)) as *const u32) } }; }
macro_rules! __GT_WRITE { ($ofs:expr, $data:expr) => {{ unsafe { core::ptr::write_volatile((GT64120_BASE + ($ofs)) as *mut u32, $data); } }}; }
macro_rules! GT_WRITE { ($ofs:expr, $data:expr) => { __GT_WRITE!($ofs, cpu_to_le32!($data)); }; }

extern "C" { pub fn gt641xx_set_base_clock(clock: u32); }
extern "C" { pub fn gt641xx_timer0_state() -> i32; }


// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
