/* SPDX-License-Identifier: GPL-2.0 */
/*
 * Copyright (C) 2025 Arm Ltd.
 * Based on arch/x86/kernel/cpu/resctrl/internal.h
 */

pub const MAX_MBA_BW: u32 = 100u32;
pub const MBM_OVERFLOW_INTERVAL: i32 = 1000;

/* Reads to Local DRAM Memory */
pub const READS_TO_LOCAL_MEM: u32 = 1u32 << 0;

/* Reads to Remote DRAM Memory */
pub const READS_TO_REMOTE_MEM: u32 = 1u32 << 1;

/* Non-Temporal Writes to Local Memory */
pub const NON_TEMP_WRITE_TO_LOCAL_MEM: u32 = 1u32 << 2;

/* Non-Temporal Writes to Remote Memory */
pub const NON_TEMP_WRITE_TO_REMOTE_MEM: u32 = 1u32 << 3;

/* Reads to Local Memory the system identifies as "Slow Memory" */
pub const READS_TO_LOCAL_S_MEM: u32 = 1u32 << 4;

/* Reads to Remote Memory the system identifies as "Slow Memory" */
pub const READS_TO_REMOTE_S_MEM: u32 = 1u32 << 5;

/* Dirty Victims to All Types of Memory */
pub const DIRTY_VICTIMS_TO_ALL_MEM: u32 = 1u32 << 6;

/* Max event bits supported */
pub const MAX_EVT_CONFIG_BITS: u32 = (1u32 << (6 + 1)) - 1;

/* Number of memory transactions that an MBM event can be configured with */
pub const NUM_MBM_TRANSACTIONS: i32 = 7;

/* Event IDs */
#[repr(i32)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum ResctrlEventId {
    /* Must match value of first event below */
    QOS_FIRST_EVENT = 0x01,

    /*
     * These values match those used to program IA32_QM_EVTSEL before
     * reading IA32_QM_CTR on RDT systems.
     */
    QOS_L3_OCCUP_EVENT_ID = 0x01,
    QOS_L3_MBM_TOTAL_EVENT_ID = 0x02,
    QOS_L3_MBM_LOCAL_EVENT_ID = 0x03,

    /* Intel Telemetry Events */
    PMT_EVENT_ENERGY = 0x04,
    PMT_EVENT_ACTIVITY = 0x05,
    PMT_EVENT_STALLS_LLC_HIT = 0x06,
    PMT_EVENT_C1_RES = 0x07,
    PMT_EVENT_UNHALTED_CORE_CYCLES = 0x08,
    PMT_EVENT_STALLS_LLC_MISS = 0x09,
    PMT_EVENT_AUTO_C6_RES = 0x0a,
    PMT_EVENT_UNHALTED_REF_CYCLES = 0x0b,
    PMT_EVENT_UOPS_RETIRED = 0x0c,

    /* Must be the last */
    QOS_NUM_EVENTS = 0x0d,
}

pub const QOS_NUM_L3_MBM_EVENTS: i32 =
    (QOS_L3_MBM_LOCAL_EVENT_ID as i32) - (QOS_L3_MBM_TOTAL_EVENT_ID as i32) + 1;

#[inline]
pub const fn MBM_STATE_IDX(evt: i32) -> i32 {
    evt - QOS_L3_MBM_TOTAL_EVENT_ID as i32
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
