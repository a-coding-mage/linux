/* SPDX-License-Identifier: GPL-2.0 */
/* Common DCR / SDR / CPR register definitions used on various IBM/AMCC 4xx processors. */

/* Most device DCRs are obtained from the device tree; these are fixed and
 * indirect DCRs commonly used outside specific drivers. */

/* CPRs (440GX and 440SP/440SPe) */
pub const DCRN_CPR0_CONFIG_ADDR: u32 = 0xc;
pub const DCRN_CPR0_CONFIG_DATA: u32 = 0xd;

/* SDRs (440GX and 440SP/440SPe) */
pub const DCRN_SDR0_CONFIG_ADDR: u32 = 0xe;
pub const DCRN_SDR0_CONFIG_DATA: u32 = 0xf;
pub const SDR0_PFC0: u32 = 0x4100;
pub const SDR0_PFC1: u32 = 0x4101;
pub const SDR0_PFC1_EPS: u32 = 0x1c00000;
pub const SDR0_PFC1_EPS_SHIFT: u32 = 22;
pub const SDR0_PFC1_RMII: u32 = 0x02000000;
pub const SDR0_MFR: u32 = 0x4300;
pub const SDR0_MFR_TAH0: u32 = 0x80000000; /* TAHOE0 Enable */
pub const SDR0_MFR_TAH1: u32 = 0x40000000; /* TAHOE1 Enable */
pub const SDR0_MFR_PCM: u32 = 0x10000000; /* PPC440GP irq compat mode */
pub const SDR0_MFR_ECS: u32 = 0x08000000; /* EMAC int clk */
pub const SDR0_MFR_T0TXFL: u32 = 0x00080000;
pub const SDR0_MFR_T0TXFH: u32 = 0x00040000;
pub const SDR0_MFR_T1TXFL: u32 = 0x00020000;
pub const SDR0_MFR_T1TXFH: u32 = 0x00010000;
pub const SDR0_MFR_E0TXFL: u32 = 0x00008000;
pub const SDR0_MFR_E0TXFH: u32 = 0x00004000;
pub const SDR0_MFR_E0RXFL: u32 = 0x00002000;
pub const SDR0_MFR_E0RXFH: u32 = 0x00001000;
pub const SDR0_MFR_E1TXFL: u32 = 0x00000800;
pub const SDR0_MFR_E1TXFH: u32 = 0x00000400;
pub const SDR0_MFR_E1RXFL: u32 = 0x00000200;
pub const SDR0_MFR_E1RXFH: u32 = 0x00000100;
pub const SDR0_MFR_E2TXFL: u32 = 0x00000080;
pub const SDR0_MFR_E2TXFH: u32 = 0x00000040;
pub const SDR0_MFR_E2RXFL: u32 = 0x00000020;
pub const SDR0_MFR_E2RXFH: u32 = 0x00000010;
pub const SDR0_MFR_E3TXFL: u32 = 0x00000008;
pub const SDR0_MFR_E3TXFH: u32 = 0x00000004;
pub const SDR0_MFR_E3RXFL: u32 = 0x00000002;
pub const SDR0_MFR_E3RXFH: u32 = 0x00000001;
pub const SDR0_UART0: u32 = 0x0120;
pub const SDR0_UART1: u32 = 0x0121;
pub const SDR0_UART2: u32 = 0x0122;
pub const SDR0_UART3: u32 = 0x0123;
pub const SDR0_CUST0: u32 = 0x4000;

/* SDR for 405EZ */
pub const DCRN_SDR_ICINTSTAT: u32 = 0x4510;
pub const ICINTSTAT_ICRX: u32 = 0x80000000;
pub const ICINTSTAT_ICTX0: u32 = 0x40000000;
pub const ICINTSTAT_ICTX1: u32 = 0x20000000;
pub const ICINTSTAT_ICTX: u32 = 0x60000000;

/* SDRs (460EX/460GT) */
pub const SDR0_ETH_CFG: u32 = 0x4103;
pub const SDR0_ETH_CFG_ECS: u32 = 0x00000100; /* EMAC int clk source */

/* DCR register addresses are offsets from the SRAM0 controller base. */
pub const DCRN_SRAM0_SB0CR: u32 = 0x00;
pub const DCRN_SRAM0_SB1CR: u32 = 0x01;
pub const DCRN_SRAM0_SB2CR: u32 = 0x02;
pub const DCRN_SRAM0_SB3CR: u32 = 0x03;
pub const SRAM_SBCR_BU_MASK: u32 = 0x00000180;
pub const SRAM_SBCR_BS_64KB: u32 = 0x00000800;
pub const SRAM_SBCR_BU_RO: u32 = 0x00000080;
pub const SRAM_SBCR_BU_RW: u32 = 0x00000180;
pub const DCRN_SRAM0_BEAR: u32 = 0x04;
pub const DCRN_SRAM0_BESR0: u32 = 0x05;
pub const DCRN_SRAM0_BESR1: u32 = 0x06;
pub const DCRN_SRAM0_PMEG: u32 = 0x07;
pub const DCRN_SRAM0_CID: u32 = 0x08;
pub const DCRN_SRAM0_REVID: u32 = 0x09;
pub const DCRN_SRAM0_DPC: u32 = 0x0a;
pub const SRAM_DPC_ENABLE: u32 = 0x80000000;

/* L2C0 DCR register offsets. */
pub const DCRN_L2C0_CFG: u32 = 0x00;
pub const L2C_CFG_L2M: u32 = 0x80000000;
pub const L2C_CFG_ICU: u32 = 0x40000000;
pub const L2C_CFG_DCU: u32 = 0x20000000;
pub const L2C_CFG_DCW_MASK: u32 = 0x1e000000;
pub const L2C_CFG_TPC: u32 = 0x01000000;
pub const L2C_CFG_CPC: u32 = 0x00800000;
pub const L2C_CFG_FRAN: u32 = 0x00200000;
pub const L2C_CFG_SS_MASK: u32 = 0x00180000;
pub const L2C_CFG_SS_256: u32 = 0x00000000;
pub const L2C_CFG_CPIM: u32 = 0x00040000;
pub const L2C_CFG_TPIM: u32 = 0x00020000;
pub const L2C_CFG_LIM: u32 = 0x00010000;
pub const L2C_CFG_PMUX_MASK: u32 = 0x00007000;
pub const L2C_CFG_PMUX_SNP: u32 = 0x00000000;
pub const L2C_CFG_PMUX_IF: u32 = 0x00001000;
pub const L2C_CFG_PMUX_DF: u32 = 0x00002000;
pub const L2C_CFG_PMUX_DS: u32 = 0x00003000;
pub const L2C_CFG_PMIM: u32 = 0x00000800;
pub const L2C_CFG_TPEI: u32 = 0x00000400;
pub const L2C_CFG_CPEI: u32 = 0x00000200;
pub const L2C_CFG_NAM: u32 = 0x00000100;
pub const L2C_CFG_SMCM: u32 = 0x00000080;
pub const L2C_CFG_NBRM: u32 = 0x00000040;
pub const L2C_CFG_RDBW: u32 = 0x00000008; /* only 460EX/GT */
pub const DCRN_L2C0_CMD: u32 = 0x01;
pub const L2C_CMD_CLR: u32 = 0x80000000;
pub const L2C_CMD_DIAG: u32 = 0x40000000;
pub const L2C_CMD_INV: u32 = 0x20000000;
pub const L2C_CMD_CCP: u32 = 0x10000000;
pub const L2C_CMD_CTE: u32 = 0x08000000;
pub const L2C_CMD_STRC: u32 = 0x04000000;
pub const L2C_CMD_STPC: u32 = 0x02000000;
pub const L2C_CMD_RPMC: u32 = 0x01000000;
pub const L2C_CMD_HCC: u32 = 0x00800000;
pub const DCRN_L2C0_ADDR: u32 = 0x02;
pub const DCRN_L2C0_DATA: u32 = 0x03;
pub const DCRN_L2C0_SR: u32 = 0x04;
pub const L2C_SR_CC: u32 = 0x80000000;
pub const L2C_SR_CPE: u32 = 0x40000000;
pub const L2C_SR_TPE: u32 = 0x20000000;
pub const L2C_SR_LRU: u32 = 0x10000000;
pub const L2C_SR_PCS: u32 = 0x08000000;
pub const DCRN_L2C0_REVID: u32 = 0x05;
pub const DCRN_L2C0_SNP0: u32 = 0x06;
pub const DCRN_L2C0_SNP1: u32 = 0x07;
pub const L2C_SNP_BA_MASK: u32 = 0xffff0000;
pub const L2C_SNP_SSR_MASK: u32 = 0x0000f000;
pub const L2C_SNP_SSR_32G: u32 = 0x0000f000;
pub const L2C_SNP_ESR: u32 = 0x00000800;

/* 440SP/440SPe I2O/DMA and reset DCRs. */
pub const DCRN_I2O0_IBAL: u32 = 0x006;
pub const DCRN_I2O0_IBAH: u32 = 0x007;
pub const I2O_REG_ENABLE: u32 = 0x00000001; /* Enable I2O/DMA access */
pub const DCRN_SDR0_SRST: u32 = 0x0200;
pub const DCRN_SDR0_SRST_I2ODMA: u32 = 0x80000000u32 >> 15; /* Reset I2O/DMA */

/* 440SP/440SPe Memory Queue DCR offsets */
pub const DCRN_MQ0_XORBA: u32 = 0x04;
pub const DCRN_MQ0_CF2H: u32 = 0x06;
pub const DCRN_MQ0_CFBHL: u32 = 0x0f;
pub const DCRN_MQ0_BAUH: u32 = 0x10;
/* HB/LL Paths Configuration Register */
pub const MQ0_CFBHL_TPLM: u32 = 28;
pub const MQ0_CFBHL_HBCL: u32 = 23;
pub const MQ0_CFBHL_POLY: u32 = 15;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
