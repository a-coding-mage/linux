/* SPDX-License-Identifier: GPL-2.0-or-later */
/* SB1250 interrupt mapper definitions. */

/* Dependency: definitions from <asm/sibyte/sb1250_defs.h> are supplied externally. */

/* Interrupt sources (Table 4-8, UM 0.2). */
pub const K_INT_SOURCES: u32 = 64;
pub const K_INT_WATCHDOG_TIMER_0: u32 = 0;
pub const K_INT_WATCHDOG_TIMER_1: u32 = 1;
pub const K_INT_TIMER_0: u32 = 2;
pub const K_INT_TIMER_1: u32 = 3;
pub const K_INT_TIMER_2: u32 = 4;
pub const K_INT_TIMER_3: u32 = 5;
pub const K_INT_SMB_0: u32 = 6;
pub const K_INT_SMB_1: u32 = 7;
pub const K_INT_UART_0: u32 = 8;
pub const K_INT_UART_1: u32 = 9;
pub const K_INT_SER_0: u32 = 10;
pub const K_INT_SER_1: u32 = 11;
pub const K_INT_PCMCIA: u32 = 12;
pub const K_INT_ADDR_TRAP: u32 = 13;
pub const K_INT_PERF_CNT: u32 = 14;
pub const K_INT_TRACE_FREEZE: u32 = 15;
pub const K_INT_BAD_ECC: u32 = 16;
pub const K_INT_COR_ECC: u32 = 17;
pub const K_INT_IO_BUS: u32 = 18;
pub const K_INT_MAC_0: u32 = 19;
pub const K_INT_MAC_1: u32 = 20;
pub const K_INT_MAC_2: u32 = 21;
pub const K_INT_DM_CH_0: u32 = 22;
pub const K_INT_DM_CH_1: u32 = 23;
pub const K_INT_DM_CH_2: u32 = 24;
pub const K_INT_DM_CH_3: u32 = 25;
pub const K_INT_MBOX_0: u32 = 26;
pub const K_INT_MBOX_1: u32 = 27;
pub const K_INT_MBOX_2: u32 = 28;
pub const K_INT_MBOX_3: u32 = 29;
/* Conditional in C: SIBYTE_HDR_FEATURE(1250, PASS2) || SIBYTE_HDR_FEATURE(112x, PASS1). */
pub const K_INT_CYCLE_CP0_INT: u32 = 30;
pub const K_INT_CYCLE_CP1_INT: u32 = 31;
pub const K_INT_GPIO_0: u32 = 32;
pub const K_INT_GPIO_1: u32 = 33;
pub const K_INT_GPIO_2: u32 = 34;
pub const K_INT_GPIO_3: u32 = 35;
pub const K_INT_GPIO_4: u32 = 36;
pub const K_INT_GPIO_5: u32 = 37;
pub const K_INT_GPIO_6: u32 = 38;
pub const K_INT_GPIO_7: u32 = 39;
pub const K_INT_GPIO_8: u32 = 40;
pub const K_INT_GPIO_9: u32 = 41;
pub const K_INT_GPIO_10: u32 = 42;
pub const K_INT_GPIO_11: u32 = 43;
pub const K_INT_GPIO_12: u32 = 44;
pub const K_INT_GPIO_13: u32 = 45;
pub const K_INT_GPIO_14: u32 = 46;
pub const K_INT_GPIO_15: u32 = 47;
pub const K_INT_LDT_FATAL: u32 = 48;
pub const K_INT_LDT_NONFATAL: u32 = 49;
pub const K_INT_LDT_SMI: u32 = 50;
pub const K_INT_LDT_NMI: u32 = 51;
pub const K_INT_LDT_INIT: u32 = 52;
pub const K_INT_LDT_STARTUP: u32 = 53;
pub const K_INT_LDT_EXT: u32 = 54;
pub const K_INT_PCI_ERROR: u32 = 55;
pub const K_INT_PCI_INTA: u32 = 56;
pub const K_INT_PCI_INTB: u32 = 57;
pub const K_INT_PCI_INTC: u32 = 58;
pub const K_INT_PCI_INTD: u32 = 59;
pub const K_INT_SPARE_2: u32 = 60;
pub const K_INT_MAC_0_CH1: u32 = 61;
pub const K_INT_MAC_1_CH1: u32 = 62;
pub const K_INT_MAC_2_CH1: u32 = 63;

/* Mask values use the externally supplied _SB_MAKEMASK1/_SB_MAKEMASK macros. */
macro_rules! int_masks { ($($name:ident = $value:ident),* $(,)?) => { $(pub const $name: u64 = _SB_MAKEMASK1($value);)* }; }
int_masks!(
    M_INT_WATCHDOG_TIMER_0 = K_INT_WATCHDOG_TIMER_0, M_INT_WATCHDOG_TIMER_1 = K_INT_WATCHDOG_TIMER_1,
    M_INT_TIMER_0 = K_INT_TIMER_0, M_INT_TIMER_1 = K_INT_TIMER_1, M_INT_TIMER_2 = K_INT_TIMER_2, M_INT_TIMER_3 = K_INT_TIMER_3,
    M_INT_SMB_0 = K_INT_SMB_0, M_INT_SMB_1 = K_INT_SMB_1, M_INT_UART_0 = K_INT_UART_0, M_INT_UART_1 = K_INT_UART_1,
    M_INT_SER_0 = K_INT_SER_0, M_INT_SER_1 = K_INT_SER_1, M_INT_PCMCIA = K_INT_PCMCIA, M_INT_ADDR_TRAP = K_INT_ADDR_TRAP,
    M_INT_PERF_CNT = K_INT_PERF_CNT, M_INT_TRACE_FREEZE = K_INT_TRACE_FREEZE, M_INT_BAD_ECC = K_INT_BAD_ECC, M_INT_COR_ECC = K_INT_COR_ECC,
    M_INT_IO_BUS = K_INT_IO_BUS, M_INT_MAC_0 = K_INT_MAC_0, M_INT_MAC_1 = K_INT_MAC_1, M_INT_MAC_2 = K_INT_MAC_2,
    M_INT_DM_CH_0 = K_INT_DM_CH_0, M_INT_DM_CH_1 = K_INT_DM_CH_1, M_INT_DM_CH_2 = K_INT_DM_CH_2, M_INT_DM_CH_3 = K_INT_DM_CH_3,
    M_INT_MBOX_0 = K_INT_MBOX_0, M_INT_MBOX_1 = K_INT_MBOX_1, M_INT_MBOX_2 = K_INT_MBOX_2, M_INT_MBOX_3 = K_INT_MBOX_3,
    M_INT_GPIO_0 = K_INT_GPIO_0, M_INT_GPIO_1 = K_INT_GPIO_1, M_INT_GPIO_2 = K_INT_GPIO_2, M_INT_GPIO_3 = K_INT_GPIO_3,
    M_INT_GPIO_4 = K_INT_GPIO_4, M_INT_GPIO_5 = K_INT_GPIO_5, M_INT_GPIO_6 = K_INT_GPIO_6, M_INT_GPIO_7 = K_INT_GPIO_7,
    M_INT_GPIO_8 = K_INT_GPIO_8, M_INT_GPIO_9 = K_INT_GPIO_9, M_INT_GPIO_10 = K_INT_GPIO_10, M_INT_GPIO_11 = K_INT_GPIO_11,
    M_INT_GPIO_12 = K_INT_GPIO_12, M_INT_GPIO_13 = K_INT_GPIO_13, M_INT_GPIO_14 = K_INT_GPIO_14, M_INT_GPIO_15 = K_INT_GPIO_15,
    M_INT_LDT_FATAL = K_INT_LDT_FATAL, M_INT_LDT_NONFATAL = K_INT_LDT_NONFATAL, M_INT_LDT_SMI = K_INT_LDT_SMI,
    M_INT_LDT_NMI = K_INT_LDT_NMI, M_INT_LDT_INIT = K_INT_LDT_INIT, M_INT_LDT_STARTUP = K_INT_LDT_STARTUP, M_INT_LDT_EXT = K_INT_LDT_EXT,
    M_INT_PCI_ERROR = K_INT_PCI_ERROR, M_INT_PCI_INTA = K_INT_PCI_INTA, M_INT_PCI_INTB = K_INT_PCI_INTB, M_INT_PCI_INTC = K_INT_PCI_INTC,
    M_INT_PCI_INTD = K_INT_PCI_INTD, M_INT_SPARE_2 = K_INT_SPARE_2, M_INT_MAC_0_CH1 = K_INT_MAC_0_CH1,
    M_INT_MAC_1_CH1 = K_INT_MAC_1_CH1, M_INT_MAC_2_CH1 = K_INT_MAC_2_CH1
);
pub const M_INT_MBOX_ALL: u64 = _SB_MAKEMASK(4, K_INT_MBOX_0);
pub const M_INT_CYCLE_CP0_INT: u64 = _SB_MAKEMASK1(K_INT_CYCLE_CP0_INT);
pub const M_INT_CYCLE_CP1_INT: u64 = _SB_MAKEMASK1(K_INT_CYCLE_CP1_INT);

pub const K_INT_MAP_I0: u32 = 0;
pub const K_INT_MAP_I1: u32 = 1;
pub const K_INT_MAP_I2: u32 = 2;
pub const K_INT_MAP_I3: u32 = 3;
pub const K_INT_MAP_I4: u32 = 4;
pub const K_INT_MAP_I5: u32 = 5;
pub const K_INT_MAP_NMI: u32 = 6;
pub const K_INT_MAP_DINT: u32 = 7;

pub const S_INT_LDT_INTMSG: u32 = 0;
pub const M_INT_LDT_INTMSG: u64 = _SB_MAKEMASK(3, S_INT_LDT_INTMSG);
#[inline] pub const fn V_INT_LDT_INTMSG(x: u64) -> u64 { _SB_MAKEVALUE(x, S_INT_LDT_INTMSG) }
#[inline] pub const fn G_INT_LDT_INTMSG(x: u64) -> u64 { _SB_GETVALUE(x, S_INT_LDT_INTMSG, M_INT_LDT_INTMSG) }
pub const K_INT_LDT_INTMSG_FIXED: u32 = 0;
pub const K_INT_LDT_INTMSG_ARBITRATED: u32 = 1;
pub const K_INT_LDT_INTMSG_SMI: u32 = 2;
pub const K_INT_LDT_INTMSG_NMI: u32 = 3;
pub const K_INT_LDT_INTMSG_INIT: u32 = 4;
pub const K_INT_LDT_INTMSG_STARTUP: u32 = 5;
pub const K_INT_LDT_INTMSG_EXTINT: u32 = 6;
pub const K_INT_LDT_INTMSG_RESERVED: u32 = 7;
pub const M_INT_LDT_EDGETRIGGER: u32 = 0;
pub const M_INT_LDT_LEVELTRIGGER: u64 = _SB_MAKEMASK1(3);
pub const M_INT_LDT_PHYSICALDEST: u32 = 0;
pub const M_INT_LDT_LOGICALDEST: u64 = _SB_MAKEMASK1(4);
pub const S_INT_LDT_INTDEST: u32 = 5;
pub const M_INT_LDT_INTDEST: u64 = _SB_MAKEMASK(10, S_INT_LDT_INTDEST);
#[inline] pub const fn V_INT_LDT_INTDEST(x: u64) -> u64 { _SB_MAKEVALUE(x, S_INT_LDT_INTDEST) }
#[inline] pub const fn G_INT_LDT_INTDEST(x: u64) -> u64 { _SB_GETVALUE(x, S_INT_LDT_INTDEST, M_INT_LDT_INTDEST) }
pub const S_INT_LDT_VECTOR: u32 = 13;
pub const M_INT_LDT_VECTOR: u64 = _SB_MAKEMASK(8, S_INT_LDT_VECTOR);
#[inline] pub const fn V_INT_LDT_VECTOR(x: u64) -> u64 { _SB_MAKEVALUE(x, S_INT_LDT_VECTOR) }
#[inline] pub const fn G_INT_LDT_VECTOR(x: u64) -> u64 { _SB_GETVALUE(x, S_INT_LDT_VECTOR, M_INT_LDT_VECTOR) }
pub const M_LDTVECT_RAISEINT: u32 = 0x00;
pub const M_LDTVECT_RAISEMBOX: u32 = 0x40;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
