/* SPDX-License-Identifier: GPL-2.0-or-later */
/* BCM1280/BCM1400 Board Support Package: SCD Constants and Macros. */
/* The original header includes sb1250_defs.h and sb1250_scd.h; those
 * dependencies are supplied by the surrounding translation. */

/* System Identification and Revision Register (Table 12). */
/* New part definitions */
pub const K_SYS_PART_BCM1480: u64 = 0x1406;
pub const K_SYS_PART_BCM1280: u64 = 0x1206;
pub const K_SYS_PART_BCM1455: u64 = 0x1407;
pub const K_SYS_PART_BCM1255: u64 = 0x1257;
pub const K_SYS_PART_BCM1158: u64 = 0x1156;

/* System Configuration Register (Table 15). */
pub const M_BCM1480_SYS_RESERVED0: u64 = 1u64 << 0;
pub const M_BCM1480_SYS_HT_MINRSTCNT: u64 = 1u64 << 1;
pub const M_BCM1480_SYS_RESERVED2: u64 = 1u64 << 2;
pub const M_BCM1480_SYS_RESERVED3: u64 = 1u64 << 3;
pub const M_BCM1480_SYS_RESERVED4: u64 = 1u64 << 4;
pub const M_BCM1480_SYS_IOB_DIV: u64 = 1u64 << 5;

macro_rules! field { ($x:expr, $shift:expr, $width:expr) => { (($x as u64) & ((1u64 << $width) - 1)) << $shift }; }
macro_rules! get_field { ($x:expr, $shift:expr, $width:expr, $mask:expr) => { (($x as u64) & $mask) >> $shift }; }

pub const S_BCM1480_SYS_PLL_DIV: u64 = 6;
pub const M_BCM1480_SYS_PLL_DIV: u64 = (0x1fu64) << S_BCM1480_SYS_PLL_DIV;
macro_rules! V_BCM1480_SYS_PLL_DIV { ($x:expr) => { field!($x, S_BCM1480_SYS_PLL_DIV, 5) }; }
macro_rules! G_BCM1480_SYS_PLL_DIV { ($x:expr) => { get_field!($x, S_BCM1480_SYS_PLL_DIV, 5, M_BCM1480_SYS_PLL_DIV) }; }
pub const S_BCM1480_SYS_SW_DIV: u64 = 11;
pub const M_BCM1480_SYS_SW_DIV: u64 = 0x1fu64 << S_BCM1480_SYS_SW_DIV;
macro_rules! V_BCM1480_SYS_SW_DIV { ($x:expr) => { field!($x, S_BCM1480_SYS_SW_DIV, 5) }; }
macro_rules! G_BCM1480_SYS_SW_DIV { ($x:expr) => { get_field!($x, S_BCM1480_SYS_SW_DIV, 5, M_BCM1480_SYS_SW_DIV) }; }
pub const M_BCM1480_SYS_PCMCIA_ENABLE: u64 = 1u64 << 16;
pub const M_BCM1480_SYS_DUART1_ENABLE: u64 = 1u64 << 17;
pub const S_BCM1480_SYS_BOOT_MODE: u64 = 18;
pub const M_BCM1480_SYS_BOOT_MODE: u64 = 3u64 << S_BCM1480_SYS_BOOT_MODE;
macro_rules! V_BCM1480_SYS_BOOT_MODE { ($x:expr) => { field!($x, S_BCM1480_SYS_BOOT_MODE, 2) }; }
macro_rules! G_BCM1480_SYS_BOOT_MODE { ($x:expr) => { get_field!($x, S_BCM1480_SYS_BOOT_MODE, 2, M_BCM1480_SYS_BOOT_MODE) }; }
pub const K_BCM1480_SYS_BOOT_MODE_ROM32: u64 = 0;
pub const K_BCM1480_SYS_BOOT_MODE_ROM8: u64 = 1;
pub const K_BCM1480_SYS_BOOT_MODE_SMBUS_SMALL: u64 = 2;
pub const K_BCM1480_SYS_BOOT_MODE_SMBUS_BIG: u64 = 3;
pub const M_BCM1480_SYS_BOOT_MODE_SMBUS: u64 = 1u64 << 19;
pub const M_BCM1480_SYS_PCI_HOST: u64 = 1u64 << 20;
pub const M_BCM1480_SYS_PCI_ARBITER: u64 = 1u64 << 21;
pub const M_BCM1480_SYS_BIG_ENDIAN: u64 = 1u64 << 22;
pub const M_BCM1480_SYS_GENCLK_EN: u64 = 1u64 << 23;
pub const M_BCM1480_SYS_GEN_PARITY_EN: u64 = 1u64 << 24;
pub const M_BCM1480_SYS_RESERVED25: u64 = 1u64 << 25;
pub const S_BCM1480_SYS_CONFIG: u64 = 26;
pub const M_BCM1480_SYS_CONFIG: u64 = 0x3fu64 << S_BCM1480_SYS_CONFIG;
macro_rules! V_BCM1480_SYS_CONFIG { ($x:expr) => { field!($x, S_BCM1480_SYS_CONFIG, 6) }; }
macro_rules! G_BCM1480_SYS_CONFIG { ($x:expr) => { get_field!($x, S_BCM1480_SYS_CONFIG, 6, M_BCM1480_SYS_CONFIG) }; }
pub const M_BCM1480_SYS_RESERVED32: u64 = 0x7fffu64 << 32;
pub const S_BCM1480_SYS_NODEID: u64 = 47;
pub const M_BCM1480_SYS_NODEID: u64 = 0xfu64 << S_BCM1480_SYS_NODEID;
macro_rules! V_BCM1480_SYS_NODEID { ($x:expr) => { field!($x, S_BCM1480_SYS_NODEID, 4) }; }
macro_rules! G_BCM1480_SYS_NODEID { ($x:expr) => { get_field!($x, S_BCM1480_SYS_NODEID, 4, M_BCM1480_SYS_NODEID) }; }
pub const M_BCM1480_SYS_CCNUMA_EN: u64 = 1u64 << 51;
pub const M_BCM1480_SYS_CPU_RESET_0: u64 = 1u64 << 52;
pub const M_BCM1480_SYS_CPU_RESET_1: u64 = 1u64 << 53;
pub const M_BCM1480_SYS_CPU_RESET_2: u64 = 1u64 << 54;
pub const M_BCM1480_SYS_CPU_RESET_3: u64 = 1u64 << 55;
pub const S_BCM1480_SYS_DISABLECPU0: u64 = 56;
pub const M_BCM1480_SYS_DISABLECPU0: u64 = 1u64 << S_BCM1480_SYS_DISABLECPU0;
pub const S_BCM1480_SYS_DISABLECPU1: u64 = 57;
pub const M_BCM1480_SYS_DISABLECPU1: u64 = 1u64 << S_BCM1480_SYS_DISABLECPU1;
pub const S_BCM1480_SYS_DISABLECPU2: u64 = 58;
pub const M_BCM1480_SYS_DISABLECPU2: u64 = 1u64 << S_BCM1480_SYS_DISABLECPU2;
pub const S_BCM1480_SYS_DISABLECPU3: u64 = 59;
pub const M_BCM1480_SYS_DISABLECPU3: u64 = 1u64 << S_BCM1480_SYS_DISABLECPU3;
pub const M_BCM1480_SYS_SB_SOFTRES: u64 = 1u64 << 60;
pub const M_BCM1480_SYS_EXT_RESET: u64 = 1u64 << 61;
pub const M_BCM1480_SYS_SYSTEM_RESET: u64 = 1u64 << 62;
pub const M_BCM1480_SYS_SW_FLAG: u64 = 1u64 << 63;

/* Watchdog Timer Configuration Registers (Table 25). */
pub const M_BCM1480_SCD_WDOG_ENABLE: u64 = 1;
pub const S_BCM1480_SCD_WDOG_RESET_TYPE: u64 = 2;
pub const M_BCM1480_SCD_WDOG_RESET_TYPE: u64 = 0x1fu64 << S_BCM1480_SCD_WDOG_RESET_TYPE;
macro_rules! V_BCM1480_SCD_WDOG_RESET_TYPE { ($x:expr) => { field!($x, S_BCM1480_SCD_WDOG_RESET_TYPE, 5) }; }
macro_rules! G_BCM1480_SCD_WDOG_RESET_TYPE { ($x:expr) => { get_field!($x, S_BCM1480_SCD_WDOG_RESET_TYPE, 5, M_BCM1480_SCD_WDOG_RESET_TYPE) }; }
pub const K_BCM1480_SCD_WDOG_RESET_FULL: u64 = 0;
pub const K_BCM1480_SCD_WDOG_RESET_SOFT: u64 = 1;
pub const K_BCM1480_SCD_WDOG_RESET_CPU0: u64 = 3;
pub const K_BCM1480_SCD_WDOG_RESET_CPU1: u64 = 5;
pub const K_BCM1480_SCD_WDOG_RESET_CPU2: u64 = 9;
pub const K_BCM1480_SCD_WDOG_RESET_CPU3: u64 = 17;
pub const K_BCM1480_SCD_WDOG_RESET_ALL_CPUS: u64 = 31;
pub const M_BCM1480_SCD_WDOG_HAS_RESET: u64 = 1u64 << 8;

/* System Performance Counter Configuration Register (Table 31). */
macro_rules! bitfield8 { ($name:ident, $vname:ident, $gname:ident, $shift:expr) => {
    pub const $name: u64 = 0xffu64 << $shift;
    macro_rules! $vname { ($x:expr) => { field!($x, $shift, 8) }; }
    macro_rules! $gname { ($x:expr) => { get_field!($x, $shift, 8, $name) }; }
}; }
bitfield8!(M_SPC_CFG_SRC4, V_SPC_CFG_SRC4, G_SPC_CFG_SRC4, 32);
bitfield8!(M_SPC_CFG_SRC5, V_SPC_CFG_SRC5, G_SPC_CFG_SRC5, 40);
bitfield8!(M_SPC_CFG_SRC6, V_SPC_CFG_SRC6, G_SPC_CFG_SRC6, 48);
bitfield8!(M_SPC_CFG_SRC7, V_SPC_CFG_SRC7, G_SPC_CFG_SRC7, 56);
pub const S_SPC_CFG_SRC4: u64 = 32;
pub const S_SPC_CFG_SRC5: u64 = 40;
pub const S_SPC_CFG_SRC6: u64 = 48;
pub const S_SPC_CFG_SRC7: u64 = 56;
pub const M_BCM1480_SPC_CFG_CLEAR: u64 = 1;
pub const M_BCM1480_SPC_CFG_ENABLE: u64 = 2;
/* #if SIBYTE_HDR_FEATURE_CHIP(1480): aliases apply for BCM1480 builds. */
pub const M_SPC_CFG_CLEAR: u64 = M_BCM1480_SPC_CFG_CLEAR;
pub const M_SPC_CFG_ENABLE: u64 = M_BCM1480_SPC_CFG_ENABLE;

pub const S_BCM1480_SPC_CNT_COUNT: u64 = 0;
pub const M_BCM1480_SPC_CNT_COUNT: u64 = (1u64 << 40) - 1;
macro_rules! V_BCM1480_SPC_CNT_COUNT { ($x:expr) => { ($x as u64) & M_BCM1480_SPC_CNT_COUNT }; }
macro_rules! G_BCM1480_SPC_CNT_COUNT { ($x:expr) => { ($x as u64) & M_BCM1480_SPC_CNT_COUNT }; }
pub const M_BCM1480_SPC_CNT_OFLOW: u64 = 1u64 << 40;

/* Address Trap Registers. */
pub const M_BCM1480_ATRAP_INDEX: u64 = 0xf;
pub const M_BCM1480_ATRAP_ADDRESS: u64 = (1u64 << 40) - 1;
pub const S_BCM1480_ATRAP_CFG_CNT: u64 = 0;
pub const M_BCM1480_ATRAP_CFG_CNT: u64 = 7;
macro_rules! V_BCM1480_ATRAP_CFG_CNT { ($x:expr) => { ($x as u64) & M_BCM1480_ATRAP_CFG_CNT }; }
macro_rules! G_BCM1480_ATRAP_CFG_CNT { ($x:expr) => { ($x as u64) & M_BCM1480_ATRAP_CFG_CNT }; }
pub const M_BCM1480_ATRAP_CFG_WRITE: u64 = 1u64 << 3;
pub const M_BCM1480_ATRAP_CFG_ALL: u64 = 1u64 << 4;
pub const M_BCM1480_ATRAP_CFG_INV: u64 = 1u64 << 5;
pub const M_BCM1480_ATRAP_CFG_USESRC: u64 = 1u64 << 6;
pub const M_BCM1480_ATRAP_CFG_SRCINV: u64 = 1u64 << 7;
pub const S_BCM1480_ATRAP_CFG_AGENTID: u64 = 8;
pub const M_BCM1480_ATRAP_CFG_AGENTID: u64 = 0xfu64 << 8;
macro_rules! V_BCM1480_ATRAP_CFG_AGENTID { ($x:expr) => { field!($x, S_BCM1480_ATRAP_CFG_AGENTID, 4) }; }
macro_rules! G_BCM1480_ATRAP_CFG_AGENTID { ($x:expr) => { get_field!($x, S_BCM1480_ATRAP_CFG_AGENTID, 4, M_BCM1480_ATRAP_CFG_AGENTID) }; }
pub const K_BCM1480_BUS_AGENT_CPU0: u64 = 0;
pub const K_BCM1480_BUS_AGENT_CPU1: u64 = 1;
pub const K_BCM1480_BUS_AGENT_NC: u64 = 2;
pub const K_BCM1480_BUS_AGENT_IOB: u64 = 3;
pub const K_BCM1480_BUS_AGENT_SCD: u64 = 4;
pub const K_BCM1480_BUS_AGENT_L2C: u64 = 6;
pub const K_BCM1480_BUS_AGENT_MC: u64 = 7;
pub const K_BCM1480_BUS_AGENT_CPU2: u64 = 8;
pub const K_BCM1480_BUS_AGENT_CPU3: u64 = 9;
pub const K_BCM1480_BUS_AGENT_PM: u64 = 10;
pub const S_BCM1480_ATRAP_CFG_CATTR: u64 = 12;
pub const M_BCM1480_ATRAP_CFG_CATTR: u64 = 3u64 << 12;
macro_rules! V_BCM1480_ATRAP_CFG_CATTR { ($x:expr) => { field!($x, S_BCM1480_ATRAP_CFG_CATTR, 2) }; }
macro_rules! G_BCM1480_ATRAP_CFG_CATTR { ($x:expr) => { get_field!($x, S_BCM1480_ATRAP_CFG_CATTR, 2, M_BCM1480_ATRAP_CFG_CATTR) }; }
pub const K_BCM1480_ATRAP_CFG_CATTR_IGNORE: u64 = 0;
pub const K_BCM1480_ATRAP_CFG_CATTR_UNC: u64 = 1;
pub const K_BCM1480_ATRAP_CFG_CATTR_NONCOH: u64 = 2;
pub const K_BCM1480_ATRAP_CFG_CATTR_COHERENT: u64 = 3;
pub const M_BCM1480_ATRAP_CFG_CATTRINV: u64 = 1u64 << 14;

/* Trace Sequence Control Registers (Table 48). */
pub const M_BCM1480_SCD_TRSEQ_TID_MATCH_EN: u64 = 1u64 << 25;
pub const S_BCM1480_SCD_TRSEQ_SWFUNC: u64 = 26;
pub const M_BCM1480_SCD_TRSEQ_SWFUNC: u64 = 3u64 << 26;
macro_rules! V_BCM1480_SCD_TRSEQ_SWFUNC { ($x:expr) => { field!($x, S_BCM1480_SCD_TRSEQ_SWFUNC, 2) }; }
macro_rules! G_BCM1480_SCD_TRSEQ_SWFUNC { ($x:expr) => { get_field!($x, S_BCM1480_SCD_TRSEQ_SWFUNC, 2, M_BCM1480_SCD_TRSEQ_SWFUNC) }; }
/* Trace Control Register (Table 49). */
pub const S_BCM1480_SCD_TRACE_CFG_MODE: u64 = 16;
pub const M_BCM1480_SCD_TRACE_CFG_MODE: u64 = 3u64 << 16;
macro_rules! V_BCM1480_SCD_TRACE_CFG_MODE { ($x:expr) => { field!($x, S_BCM1480_SCD_TRACE_CFG_MODE, 2) }; }
macro_rules! G_BCM1480_SCD_TRACE_CFG_MODE { ($x:expr) => { get_field!($x, S_BCM1480_SCD_TRACE_CFG_MODE, 2, M_BCM1480_SCD_TRACE_CFG_MODE) }; }
pub const K_BCM1480_SCD_TRACE_CFG_MODE_BLOCKERS: u64 = 0;
pub const K_BCM1480_SCD_TRACE_CFG_MODE_BYTEEN_INT: u64 = 1;
pub const K_BCM1480_SCD_TRACE_CFG_MODE_FLOW_ID: u64 = 2;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
