/* SPDX-License-Identifier: GPL-2.0 */
/*
 * Definitions for use with the Alpha wrperfmon PAL call.
 */

/* Following commands are implemented on all CPUs */
pub const PERFMON_CMD_DISABLE: u64 = 0;
pub const PERFMON_CMD_ENABLE: u64 = 1;
pub const PERFMON_CMD_DESIRED_EVENTS: u64 = 2;
pub const PERFMON_CMD_LOGGING_OPTIONS: u64 = 3;
/* Following commands on EV5/EV56/PCA56 only */
pub const PERFMON_CMD_INT_FREQ: u64 = 4;
pub const PERFMON_CMD_ENABLE_CLEAR: u64 = 7;
/* Following commands are on EV5 and better CPUs */
pub const PERFMON_CMD_READ: u64 = 5;
pub const PERFMON_CMD_WRITE: u64 = 6;
/* Following command are on EV6 and better CPUs */
pub const PERFMON_CMD_ENABLE_WRITE: u64 = 7;
/* Following command are on EV67 and better CPUs */
pub const PERFMON_CMD_I_STAT: u64 = 8;
pub const PERFMON_CMD_PMPC: u64 = 9;

/* EV5/EV56/PCA56 Counters */
pub const EV5_PCTR_0: u64 = 1u64 << 0;
pub const EV5_PCTR_1: u64 = 1u64 << 1;
pub const EV5_PCTR_2: u64 = 1u64 << 2;

pub const EV5_PCTR_0_COUNT_SHIFT: u64 = 48;
pub const EV5_PCTR_1_COUNT_SHIFT: u64 = 32;
pub const EV5_PCTR_2_COUNT_SHIFT: u64 = 16;

pub const EV5_PCTR_0_COUNT_MASK: u64 = 0xffffu64;
pub const EV5_PCTR_1_COUNT_MASK: u64 = 0xffffu64;
pub const EV5_PCTR_2_COUNT_MASK: u64 = 0x3fffu64;

/* EV6 Counters */
pub const EV6_PCTR_0: u64 = 1u64 << 0;
pub const EV6_PCTR_1: u64 = 1u64 << 1;

pub const EV6_PCTR_0_COUNT_SHIFT: u64 = 28;
pub const EV6_PCTR_1_COUNT_SHIFT: u64 = 6;

pub const EV6_PCTR_0_COUNT_MASK: u64 = 0xfffffu64;
pub const EV6_PCTR_1_COUNT_MASK: u64 = 0xfffffu64;

/* EV67 (and subsequent) counters */
pub const EV67_PCTR_0: u64 = 1u64 << 0;
pub const EV67_PCTR_1: u64 = 1u64 << 1;

pub const EV67_PCTR_0_COUNT_SHIFT: u64 = 28;
pub const EV67_PCTR_1_COUNT_SHIFT: u64 = 6;

pub const EV67_PCTR_0_COUNT_MASK: u64 = 0xfffffu64;
pub const EV67_PCTR_1_COUNT_MASK: u64 = 0xfffffu64;

/*
 * The Alpha Architecure Handbook, vers. 4 (1998) appears to have a misprint
 *  in Table E-23 regarding the bits that set the event PCTR 1 counts.
 *  Hopefully what we have here is correct.
 */
pub const EV6_PCTR_0_EVENT_MASK: u64 = 0x10u64;
pub const EV6_PCTR_1_EVENT_MASK: u64 = 0x0fu64;

/* EV6 Events */
pub const EV6_PCTR_0_CYCLES: u64 = 0u64 << 4;
pub const EV6_PCTR_0_INSTRUCTIONS: u64 = 1u64 << 4;

pub const EV6_PCTR_1_CYCLES: u64 = 0;
pub const EV6_PCTR_1_BRANCHES: u64 = 1;
pub const EV6_PCTR_1_BRANCH_MISPREDICTS: u64 = 2;
pub const EV6_PCTR_1_DTB_SINGLE_MISSES: u64 = 3;
pub const EV6_PCTR_1_DTB_DOUBLE_MISSES: u64 = 4;
pub const EV6_PCTR_1_ITB_MISSES: u64 = 5;
pub const EV6_PCTR_1_UNALIGNED_TRAPS: u64 = 6;
pub const EV6_PCTR_1_REPLY_TRAPS: u64 = 7;

/* From the Alpha Architecture Reference Manual, 4th edn., 2002 */
pub const EV67_PCTR_MODE_MASK: u64 = 0x10u64;
pub const EV67_PCTR_EVENT_MASK: u64 = 0x0cu64;

pub const EV67_PCTR_MODE_PROFILEME: u64 = 1u64 << 4;
pub const EV67_PCTR_MODE_AGGREGATE: u64 = 0u64 << 4;

pub const EV67_PCTR_INSTR_CYCLES: u64 = 0u64 << 2;
pub const EV67_PCTR_CYCLES_UNDEF: u64 = 1u64 << 2;
pub const EV67_PCTR_INSTR_BCACHEMISS: u64 = 2u64 << 2;
pub const EV67_PCTR_CYCLES_MBOX: u64 = 3u64 << 2;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
