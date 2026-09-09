/* SPDX-License-Identifier: GPL-2.0-or-later */
/* SB1250 Board Support Package register definitions. */
/* The original header depends on build-time SIBYTE_HDR_FEATURE_* conditions;
 * conditional groups are retained below as comments where Rust cannot infer
 * those configuration symbols from this file alone. */

#![allow(non_upper_case_globals, dead_code)]

/* Register constants and address calculations. */
pub const A_MC_BASE_0: u64 = 0x0010051000;
pub const A_MC_BASE_1: u64 = 0x0010052000;
pub const MC_REGISTER_SPACING: u64 = 0x1000;
pub const R_MC_CONFIG: u64 = 0x100; pub const R_MC_DRAMCMD: u64 = 0x120;
pub const R_MC_DRAMMODE: u64 = 0x140; pub const R_MC_TIMING1: u64 = 0x160;
pub const R_MC_TIMING2: u64 = 0x180; pub const R_MC_CS_START: u64 = 0x1a0;
pub const R_MC_CS_END: u64 = 0x1c0; pub const R_MC_CS_INTERLEAVE: u64 = 0x1e0;
pub const S_MC_CS_STARTEND: u64 = 16;
pub const R_MC_CSX_BASE: u64 = 0x200; pub const R_MC_CSX_ROW: u64 = 0;
pub const R_MC_CSX_COL: u64 = 0x20; pub const R_MC_CSX_BA: u64 = 0x40;
pub const MC_CSX_SPACING: u64 = 0x60;
pub const R_MC_CS0_ROW: u64 = 0x200; pub const R_MC_CS0_COL: u64 = 0x220; pub const R_MC_CS0_BA: u64 = 0x240;
pub const R_MC_CS1_ROW: u64 = 0x260; pub const R_MC_CS1_COL: u64 = 0x280; pub const R_MC_CS1_BA: u64 = 0x2a0;
pub const R_MC_CS2_ROW: u64 = 0x2c0; pub const R_MC_CS2_COL: u64 = 0x2e0; pub const R_MC_CS2_BA: u64 = 0x300;
pub const R_MC_CS3_ROW: u64 = 0x320; pub const R_MC_CS3_COL: u64 = 0x340; pub const R_MC_CS3_BA: u64 = 0x360;
pub const R_MC_CS_ATTR: u64 = 0x380; pub const R_MC_TEST_DATA: u64 = 0x400; pub const R_MC_TEST_ECC: u64 = 0x420; pub const R_MC_MCLK_CFG: u64 = 0x500;

pub const A_L2_READ_TAG:u64=0x0010040018; pub const A_L2_ECC_TAG:u64=0x0010040038; pub const A_L2_READ_MISC:u64=0x0010040058;
pub const A_L2_WAY_DISABLE:u64=0x0010041000; pub const A_L2_MGMT_TAG_BASE:u64=0x00d0000000;
pub const A_L2_CACHE_DISABLE:u64=0x0010042000; pub const A_L2_MISC_CONFIG:u64=0x0010043000;
pub const A_PCI_TYPE00_HEADER:u64=0x00de000000; pub const A_PCI_TYPE01_HEADER:u64=0x00de000800;
pub const A_MAC_BASE_0:u64=0x0010064000; pub const A_MAC_BASE_1:u64=0x0010065000; pub const A_MAC_BASE_2:u64=0x0010066000;
pub const MAC_SPACING:u64=0x1000; pub const MAC_DMA_TXRX_SPACING:u64=0x400; pub const MAC_DMA_CHANNEL_SPACING:u64=0x100;
pub const DMA_RX:u64=0; pub const DMA_TX:u64=1; pub const MAC_NUM_DMACHAN:u64=2; pub const MAC_NUM_PORTS:u64=3;
pub const R_MAC_DMA_CHANNELS:u64=0x800;
pub const R_MAC_RMON_TX_BYTES:u64=0; pub const R_MAC_RMON_COLLISIONS:u64=8; pub const R_MAC_RMON_LATE_COL:u64=0x10; pub const R_MAC_RMON_EX_COL:u64=0x18; pub const R_MAC_RMON_FCS_ERROR:u64=0x20; pub const R_MAC_RMON_TX_ABORT:u64=0x28; pub const R_MAC_RMON_TX_BAD:u64=0x38; pub const R_MAC_RMON_TX_GOOD:u64=0x40; pub const R_MAC_RMON_TX_RUNT:u64=0x48; pub const R_MAC_RMON_TX_OVERSIZE:u64=0x50;
pub const R_MAC_RMON_RX_BYTES:u64=0x80; pub const R_MAC_RMON_RX_MCAST:u64=0x88; pub const R_MAC_RMON_RX_BCAST:u64=0x90; pub const R_MAC_RMON_RX_BAD:u64=0x98; pub const R_MAC_RMON_RX_GOOD:u64=0xa0; pub const R_MAC_RMON_RX_RUNT:u64=0xa8; pub const R_MAC_RMON_RX_OVERSIZE:u64=0xb0; pub const R_MAC_RMON_RX_FCS_ERROR:u64=0xb8; pub const R_MAC_RMON_RX_LENGTH_ERROR:u64=0xc0; pub const R_MAC_RMON_RX_CODE_ERROR:u64=0xc8; pub const R_MAC_RMON_RX_ALIGN_ERROR:u64=0xd0;
pub const R_MAC_CFG:u64=0x100; pub const R_MAC_THRSH_CFG:u64=0x108; pub const R_MAC_VLANTAG:u64=0x110; pub const R_MAC_FRAMECFG:u64=0x118; pub const R_MAC_EOPCNT:u64=0x120; pub const R_MAC_FIFO_PTRS:u64=0x128; pub const R_MAC_ADFILTER_CFG:u64=0x200; pub const R_MAC_ETHERNET_ADDR:u64=0x208; pub const R_MAC_PKT_TYPE:u64=0x210; pub const R_MAC_ADMASK0:u64=0x218; pub const R_MAC_ADMASK1:u64=0x220; pub const R_MAC_HASH_BASE:u64=0x240; pub const R_MAC_ADDR_BASE:u64=0x280; pub const R_MAC_CHLO0_BASE:u64=0x300; pub const R_MAC_CHUP0_BASE:u64=0x320; pub const R_MAC_ENABLE:u64=0x400; pub const R_MAC_STATUS:u64=0x408; pub const R_MAC_INT_MASK:u64=0x410; pub const R_MAC_TXD_CTL:u64=0x420; pub const R_MAC_MDIO:u64=0x428; pub const R_MAC_STATUS1:u64=0x430; pub const R_MAC_DEBUG_STATUS:u64=0x448; pub const MAC_HASH_COUNT:u64=8; pub const MAC_ADDR_COUNT:u64=8; pub const MAC_CHMAP_COUNT:u64=4;

pub const A_DUART:u64=0x0010060000; pub const DUART_CHANREG_SPACING:u64=0x100; pub const R_DUART_NUM_PORTS:u64=2;
pub const R_DUART_MODE_REG_1:u64=0; pub const R_DUART_MODE_REG_2:u64=0x10; pub const R_DUART_STATUS:u64=0x20; pub const R_DUART_CLK_SEL:u64=0x30; pub const R_DUART_CMD:u64=0x50; pub const R_DUART_RX_HOLD:u64=0x60; pub const R_DUART_TX_HOLD:u64=0x70; pub const R_DUART_FULL_CTL:u64=0x40; pub const R_DUART_OPCR_X:u64=0x80; pub const R_DUART_AUXCTL_X:u64=0x90;
pub const R_DUART_AUX_CTRL:u64=0x10; pub const R_DUART_ISR_A:u64=0x20; pub const R_DUART_IMR_A:u64=0x30; pub const R_DUART_ISR_B:u64=0x40; pub const R_DUART_IMR_B:u64=0x50; pub const R_DUART_OUT_PORT:u64=0x60; pub const R_DUART_OPCR:u64=0x70; pub const R_DUART_IN_PORT:u64=0x80; pub const R_DUART_SET_OPR:u64=0xb0; pub const R_DUART_CLEAR_OPR:u64=0xc0; pub const R_DUART_IN_CHNG_A:u64=0xd0; pub const R_DUART_IN_CHNG_B:u64=0xe0;

pub const A_IO_EXT_BASE:u64=0x0010061000; pub const IO_EXT_CFG_COUNT:u64=8; pub const IO_EXT_REGISTER_SPACING:u64=8; pub const A_IO_EXT_CFG_BASE:u64=0x0010061000; pub const A_IO_EXT_MULT_SIZE_BASE:u64=0x0010061100; pub const A_IO_EXT_START_ADDR_BASE:u64=0x0010061200; pub const A_IO_EXT_TIME_CFG0_BASE:u64=0x0010061600; pub const A_IO_EXT_TIME_CFG1_BASE:u64=0x0010061700;
pub const A_IO_INTERRUPT_STATUS:u64=0x0010061a00; pub const A_IO_INTERRUPT_DATA0:u64=0x0010061a10; pub const A_IO_INTERRUPT_DATA1:u64=0x0010061a18; pub const A_IO_INTERRUPT_DATA2:u64=0x0010061a20; pub const A_IO_INTERRUPT_DATA3:u64=0x0010061a28; pub const A_IO_INTERRUPT_ADDR0:u64=0x0010061a30; pub const A_IO_INTERRUPT_ADDR1:u64=0x0010061a40; pub const A_IO_INTERRUPT_PARITY:u64=0x0010061a50; pub const A_IO_PCMCIA_CFG:u64=0x0010061a60; pub const A_IO_PCMCIA_STATUS:u64=0x0010061a70;
pub const A_GPIO_CLR_EDGE:u64=0x0010061a80; pub const A_GPIO_INT_TYPE:u64=0x0010061a88; pub const A_GPIO_INPUT_INVERT:u64=0x0010061a90; pub const A_GPIO_GLITCH:u64=0x0010061a98; pub const A_GPIO_READ:u64=0x0010061aa0; pub const A_GPIO_DIRECTION:u64=0x0010061aa8; pub const A_GPIO_PIN_CLR:u64=0x0010061ab0; pub const A_GPIO_PIN_SET:u64=0x0010061ab8; pub const A_GPIO_BASE:u64=A_GPIO_CLR_EDGE;
pub const R_GPIO_CLR_EDGE:u64=0; pub const R_GPIO_INT_TYPE:u64=8; pub const R_GPIO_INPUT_INVERT:u64=0x10; pub const R_GPIO_GLITCH:u64=0x18; pub const R_GPIO_READ:u64=0x20; pub const R_GPIO_DIRECTION:u64=0x28; pub const R_GPIO_PIN_CLR:u64=0x30; pub const R_GPIO_PIN_SET:u64=0x38;

pub const A_SMB_0:u64=0x0010060000; pub const A_SMB_1:u64=0x0010060008; pub const SMB_REGISTER_SPACING:u64=8;
pub const A_SMB_XTRA_0:u64=0x0010060000; pub const A_SMB_XTRA_1:u64=0x0010060008; pub const A_SMB_FREQ_0:u64=0x0010060010; pub const A_SMB_FREQ_1:u64=0x0010060018; pub const A_SMB_STATUS_0:u64=0x0010060020; pub const A_SMB_STATUS_1:u64=0x0010060028; pub const A_SMB_CMD_0:u64=0x0010060030; pub const A_SMB_CMD_1:u64=0x0010060038; pub const A_SMB_START_0:u64=0x0010060040; pub const A_SMB_START_1:u64=0x0010060048; pub const A_SMB_DATA_0:u64=0x0010060050; pub const A_SMB_DATA_1:u64=0x0010060058; pub const A_SMB_CONTROL_0:u64=0x0010060060; pub const A_SMB_CONTROL_1:u64=0x0010060068; pub const A_SMB_PEC_0:u64=0x0010060070; pub const A_SMB_PEC_1:u64=0x0010060078;
pub const R_SMB_XTRA:u64=0; pub const R_SMB_FREQ:u64=0x10; pub const R_SMB_STATUS:u64=0x20; pub const R_SMB_CMD:u64=0x30; pub const R_SMB_START:u64=0x40; pub const R_SMB_DATA:u64=0x50; pub const R_SMB_CONTROL:u64=0x60; pub const R_SMB_PEC:u64=0x70;

pub const A_SCD_WDOG_0:u64=0x0010020050; pub const A_SCD_WDOG_1:u64=0x0010020150; pub const SCD_WDOG_SPACING:u64=0x100; pub const SCD_NUM_WDOGS:u64=2; pub const R_SCD_WDOG_INIT:u64=0; pub const R_SCD_WDOG_CNT:u64=8; pub const R_SCD_WDOG_CFG:u64=0x10;
pub const A_SCD_TIMER_0:u64=0x0010020070; pub const A_SCD_TIMER_1:u64=0x0010020078; pub const A_SCD_TIMER_2:u64=0x0010020170; pub const A_SCD_TIMER_3:u64=0x0010020178; pub const SCD_NUM_TIMERS:u64=4; pub const R_SCD_TIMER_INIT:u64=0; pub const R_SCD_TIMER_CNT:u64=0x10; pub const R_SCD_TIMER_CFG:u64=0x20;
pub const A_SCD_SYSTEM_REVISION:u64=0x0010020000; pub const A_SCD_SYSTEM_CFG:u64=0x0010020008; pub const A_SCD_SYSTEM_MANUF:u64=0x0010038000;
pub const A_ADDR_TRAP_INDEX:u64=0x00100200b0; pub const A_ADDR_TRAP_REG:u64=0x00100200b8; pub const A_ADDR_TRAP_UP_0:u64=0x0010020400; pub const A_ADDR_TRAP_DOWN_0:u64=0x0010020420; pub const A_ADDR_TRAP_CFG_0:u64=0x0010020440; pub const ADDR_TRAP_SPACING:u64=8; pub const NUM_ADDR_TRAP:u64=4;
pub const A_IMR_CPU0_BASE:u64=0x0010020000; pub const A_IMR_CPU1_BASE:u64=0x0010022000; pub const IMR_REGISTER_SPACING:u64=0x2000; pub const IMR_REGISTER_SPACING_SHIFT:u64=13;
pub const R_IMR_INTERRUPT_DIAG:u64=0x10; pub const R_IMR_INTERRUPT_LDT:u64=0x18; pub const R_IMR_INTERRUPT_MASK:u64=0x28; pub const R_IMR_INTERRUPT_TRACE:u64=0x38; pub const R_IMR_INTERRUPT_SOURCE_STATUS:u64=0x40; pub const R_IMR_LDT_INTERRUPT_SET:u64=0x48; pub const R_IMR_LDT_INTERRUPT_CLR:u64=0x20; pub const R_IMR_MAILBOX_CPU:u64=0xc0; pub const R_IMR_ALIAS_MAILBOX_CPU:u64=0x1000; pub const R_IMR_MAILBOX_SET_CPU:u64=0xc8; pub const R_IMR_ALIAS_MAILBOX_SET_CPU:u64=0x1008; pub const R_IMR_MAILBOX_CLR_CPU:u64=0xd0; pub const R_IMR_INTERRUPT_STATUS_BASE:u64=0x100; pub const R_IMR_INTERRUPT_STATUS_COUNT:u64=7; pub const R_IMR_INTERRUPT_MAP_BASE:u64=0x200; pub const R_IMR_INTERRUPT_MAP_COUNT:u64=64;
pub const A_SCD_PERF_CNT_CFG:u64=0x00100204c0; pub const A_SCD_PERF_CNT_0:u64=0x00100204d0; pub const A_SCD_PERF_CNT_1:u64=0x00100204d8; pub const A_SCD_PERF_CNT_2:u64=0x00100204e0; pub const A_SCD_PERF_CNT_3:u64=0x00100204e8; pub const SCD_NUM_PERF_CNT:u64=4; pub const SCD_PERF_CNT_SPACING:u64=8;
pub const A_SCD_BUS_ERR_STATUS:u64=0x0010020880; pub const A_BUS_ERR_DATA_0:u64=0x00100208a0; pub const A_BUS_ERR_DATA_1:u64=0x00100208a8; pub const A_BUS_ERR_DATA_2:u64=0x00100208b0; pub const A_BUS_ERR_DATA_3:u64=0x00100208b8; pub const A_BUS_L2_ERRORS:u64=0x00100208c0; pub const A_BUS_MEM_IO_ERRORS:u64=0x00100208c8; pub const A_SCD_JTAG_BASE:u64=0x0010000000;
pub const A_SCD_TRACE_CFG:u64=0x0010020a00; pub const A_SCD_TRACE_READ:u64=0x0010020a08; pub const TRACE_REGISTER_SPACING:u64=8; pub const TRACE_NUM_REGISTERS:u64=8;
pub const A_SCD_TRACE_EVENT_0:u64=0x0010020a20; pub const A_SCD_TRACE_EVENT_1:u64=0x0010020a28; pub const A_SCD_TRACE_EVENT_2:u64=0x0010020a30; pub const A_SCD_TRACE_EVENT_3:u64=0x0010020a38; pub const A_SCD_TRACE_EVENT_4:u64=0x0010020a60; pub const A_SCD_TRACE_EVENT_5:u64=0x0010020a68; pub const A_SCD_TRACE_EVENT_6:u64=0x0010020a70; pub const A_SCD_TRACE_EVENT_7:u64=0x0010020a78;
pub const A_SCD_TRACE_SEQUENCE_0:u64=0x0010020a40; pub const A_SCD_TRACE_SEQUENCE_1:u64=0x0010020a48; pub const A_SCD_TRACE_SEQUENCE_2:u64=0x0010020a50; pub const A_SCD_TRACE_SEQUENCE_3:u64=0x0010020a58; pub const A_SCD_TRACE_SEQUENCE_4:u64=0x0010020a80; pub const A_SCD_TRACE_SEQUENCE_5:u64=0x0010020a88; pub const A_SCD_TRACE_SEQUENCE_6:u64=0x0010020a90; pub const A_SCD_TRACE_SEQUENCE_7:u64=0x0010020a98;
pub const R_SER_DMA_CONFIG0:u64=0; pub const R_SER_DMA_CONFIG1:u64=8; pub const R_SER_DMA_DSCR_BASE:u64=0x10; pub const R_SER_DMA_DSCR_CNT:u64=0x18; pub const R_SER_DMA_CUR_DSCRA:u64=0x20; pub const R_SER_DMA_CUR_DSCRB:u64=0x28; pub const R_SER_DMA_CUR_DSCRADDR:u64=0x30; pub const R_SER_MODE:u64=0x100; pub const R_SER_MINFRM_SZ:u64=0x108; pub const R_SER_MAXFRM_SZ:u64=0x110; pub const R_SER_ADDR:u64=0x118; pub const R_SER_CMD:u64=0x140; pub const R_SER_DMA_ENABLE:u64=0x180; pub const R_SER_INT_MASK:u64=0x190; pub const R_SER_STATUS:u64=0x188;
pub const R_IO_INTERRUPT_STATUS:u64=0x0a00; pub const R_IO_INTERRUPT_DATA0:u64=0x0a10; pub const R_IO_INTERRUPT_DATA1:u64=0x0a18; pub const R_IO_INTERRUPT_DATA2:u64=0x0a20; pub const R_IO_INTERRUPT_DATA3:u64=0x0a28; pub const R_IO_INTERRUPT_ADDR0:u64=0x0a30; pub const R_IO_INTERRUPT_ADDR1:u64=0x0a40; pub const R_IO_INTERRUPT_PARITY:u64=0x0a50; pub const R_IO_PCMCIA_CFG:u64=0x0a60; pub const R_IO_PCMCIA_STATUS:u64=0x0a70;
pub const A_SCD_SCRATCH:u64=0x0010020c10; pub const A_SCD_ZBBUS_CYCLE_COUNT:u64=0x0010030000; pub const A_SCD_ZBBUS_CYCLE_CP0:u64=0x0010020c00; pub const A_SCD_ZBBUS_CYCLE_CP1:u64=0x0010020c08;
pub const A_DM_CRC_0:u64=0x0010020b80; pub const A_DM_CRC_1:u64=0x0010020b90; pub const DM_CRC_REGISTER_SPACING:u64=0x10; pub const DM_CRC_NUM_CHANNELS:u64=2; pub const R_CRC_DEF_0:u64=0; pub const R_CTCP_DEF_0:u64=8;

pub const A_DM_0:u64=0x0010020b00; pub const A_DM_1:u64=0x0010020b20; pub const A_DM_2:u64=0x0010020b40; pub const A_DM_3:u64=0x0010020b60; pub const DM_REGISTER_SPACING:u64=0x20; pub const DM_NUM_CHANNELS:u64=4; pub const R_DM_DSCR_BASE:u64=0; pub const R_DM_DSCR_COUNT:u64=8; pub const R_DM_CUR_DSCR_ADDR:u64=0x10; pub const R_DM_DSCR_BASE_DEBUG:u64=0x18;

/* Direct translations of function-like macros. */
#[inline] pub const fn a_mc_base(ctlid:u64)->u64 { ctlid*MC_REGISTER_SPACING+A_MC_BASE_0 }
#[inline] pub const fn a_mc_register(ctlid:u64, reg:u64)->u64 { a_mc_base(ctlid)+reg }
#[inline] pub const fn a_mac_channel_base(macnum:u64)->u64 { A_MAC_BASE_0+MAC_SPACING*macnum }
#[inline] pub const fn a_mac_register(macnum:u64,reg:u64)->u64 { a_mac_channel_base(macnum)+reg }
#[inline] pub const fn a_mac_dma_channel_base(macnum:u64,txrx:u64,chan:u64)->u64 { a_mac_channel_base(macnum)+R_MAC_DMA_CHANNELS+MAC_DMA_TXRX_SPACING*txrx+MAC_DMA_CHANNEL_SPACING*chan }
#[inline] pub const fn r_mac_dma_channel_base(txrx:u64,chan:u64)->u64 { R_MAC_DMA_CHANNELS+MAC_DMA_TXRX_SPACING*txrx+MAC_DMA_CHANNEL_SPACING*chan }
#[inline] pub const fn a_mac_dma_register(macnum:u64,txrx:u64,chan:u64,reg:u64)->u64 { a_mac_dma_channel_base(macnum,txrx,chan)+reg }
#[inline] pub const fn r_mac_dma_register(txrx:u64,chan:u64,reg:u64)->u64 { r_mac_dma_channel_base(txrx,chan)+reg }
#[inline] pub const fn a_duart_chanreg(chan:u64,reg:u64)->u64 { A_DUART+DUART_CHANREG_SPACING*(chan+1)+reg }
#[inline] pub const fn a_io_ext_reg(r:u64)->u64 { A_IO_EXT_BASE+r }
#[inline] pub const fn a_smb_base(idx:u64)->u64 { A_SMB_0+idx*SMB_REGISTER_SPACING }
#[inline] pub const fn a_smb_register(idx:u64,reg:u64)->u64 { a_smb_base(idx)+reg }
#[inline] pub const fn a_scd_wdog_base(w:u64)->u64 { A_SCD_WDOG_0+SCD_WDOG_SPACING*w }
#[inline] pub const fn a_scd_wdog_register(w:u64,r:u64)->u64 { a_scd_wdog_base(w)+r }
#[inline] pub const fn a_scd_timer_base(w:u64)->u64 { A_SCD_TIMER_0+0x08*(w&1)+0x100*((w&2)>>1) }
#[inline] pub const fn a_scd_timer_register(w:u64,r:u64)->u64 { a_scd_timer_base(w)+r }
#[inline] pub const fn a_addr_trap_up(n:u64)->u64 { A_ADDR_TRAP_UP_0+n*ADDR_TRAP_SPACING }
#[inline] pub const fn a_addr_trap_down(n:u64)->u64 { A_ADDR_TRAP_DOWN_0+n*ADDR_TRAP_SPACING }
#[inline] pub const fn a_addr_trap_cfg(n:u64)->u64 { A_ADDR_TRAP_CFG_0+n*ADDR_TRAP_SPACING }
#[inline] pub const fn a_imr_mapper(cpu:u64)->u64 { A_IMR_CPU0_BASE+cpu*IMR_REGISTER_SPACING }
#[inline] pub const fn a_imr_register(cpu:u64,reg:u64)->u64 { a_imr_mapper(cpu)+reg }
#[inline] pub const fn a_scd_perf_cnt(n:u64)->u64 { A_SCD_PERF_CNT_0+n*SCD_PERF_CNT_SPACING }
#[inline] pub const fn a_dm_base(idx:u64)->u64 { A_DM_0+idx*DM_REGISTER_SPACING }
#[inline] pub const fn a_dm_register(idx:u64,reg:u64)->u64 { a_dm_base(idx)+reg }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
