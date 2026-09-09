/* SPDX-License-Identifier: GPL-2.0 */
/*
 * This header provides constants for the defined trigger signal
 * types on CoreSight CTI.
 */

pub const GEN_IO: i32 = 0;
pub const GEN_INTREQ: i32 = 1;
pub const GEN_INTACK: i32 = 2;
pub const GEN_HALTREQ: i32 = 3;
pub const GEN_RESTARTREQ: i32 = 4;
pub const PE_EDBGREQ: i32 = 5;
pub const PE_DBGRESTART: i32 = 6;
pub const PE_CTIIRQ: i32 = 7;
pub const PE_PMUIRQ: i32 = 8;
pub const PE_DBGTRIGGER: i32 = 9;
pub const ETM_EXTOUT: i32 = 10;
pub const ETM_EXTIN: i32 = 11;
pub const SNK_FULL: i32 = 12;
pub const SNK_ACQCOMP: i32 = 13;
pub const SNK_FLUSHCOMP: i32 = 14;
pub const SNK_FLUSHIN: i32 = 15;
pub const SNK_TRIGIN: i32 = 16;
pub const STM_ASYNCOUT: i32 = 17;
pub const STM_TOUT_SPTE: i32 = 18;
pub const STM_TOUT_SW: i32 = 19;
pub const STM_TOUT_HETE: i32 = 20;
pub const STM_HWEVENT: i32 = 21;
pub const ELA_TSTART: i32 = 22;
pub const ELA_TSTOP: i32 = 23;
pub const ELA_DBGREQ: i32 = 24;
pub const CTI_TRIG_MAX: i32 = 25;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
