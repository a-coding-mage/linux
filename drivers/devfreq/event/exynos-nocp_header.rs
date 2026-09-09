/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * exynos-nocp.h - Exynos NoC (Network on Chip) Probe header file
 *
 * Copyright (c) 2016 Samsung Electronics Co., Ltd.
 * Author : Chanwoo Choi <cw00.choi@samsung.com>
 */

#[repr(C)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum NocpReg {
    NocpIdRevisionId = 0x04,
    NocpMainCtl = 0x08,
    NocpCfgCtl = 0x0C,

    NocpStatPeriod = 0x24,
    NocpStatGo = 0x28,
    NocpStatAlarmMin = 0x2C,
    NocpStatAlarmMax = 0x30,
    NocpStatAlarmStatus = 0x34,
    NocpStatAlarmClr = 0x38,

    NocpCounters0Src = 0x138,
    NocpCounters0AlarmMode = 0x13C,
    NocpCounters0Val = 0x140,

    NocpCounters1Src = 0x14C,
    NocpCounters1AlarmMode = 0x150,
    NocpCounters1Val = 0x154,

    NocpCounters2Src = 0x160,
    NocpCounters2AlarmMode = 0x164,
    NocpCounters2Val = 0x168,

    NocpCounters3Src = 0x174,
    NocpCounters3AlarmMode = 0x178,
    NocpCounters3Val = 0x17C,
}

/* NOCP_MAIN_CTL register */
pub const NOCP_MAIN_CTL_ERREN_MASK: u32 = 1 << 0;
pub const NOCP_MAIN_CTL_TRACEEN_MASK: u32 = 1 << 1;
pub const NOCP_MAIN_CTL_PAYLOADEN_MASK: u32 = 1 << 2;
pub const NOCP_MAIN_CTL_STATEN_MASK: u32 = 1 << 3;
pub const NOCP_MAIN_CTL_ALARMEN_MASK: u32 = 1 << 4;
pub const NOCP_MAIN_CTL_STATCONDDUMP_MASK: u32 = 1 << 5;
pub const NOCP_MAIN_CTL_INTRUSIVEMODE_MASK: u32 = 1 << 6;

/* NOCP_CFG_CTL register */
pub const NOCP_CFG_CTL_GLOBALEN_MASK: u32 = 1 << 0;
pub const NOCP_CFG_CTL_ACTIVE_MASK: u32 = 1 << 1;

/* NOCP_COUNTERS_x_SRC register */
pub const NOCP_CNT_SRC_INTEVENT_SHIFT: u32 = 0;
pub const NOCP_CNT_SRC_INTEVENT_MASK: u32 = 0x1F << NOCP_CNT_SRC_INTEVENT_SHIFT;
pub const NOCP_CNT_SRC_INTEVENT_OFF_MASK: u32 = 0x0 << NOCP_CNT_SRC_INTEVENT_SHIFT;
pub const NOCP_CNT_SRC_INTEVENT_CYCLE_MASK: u32 = 0x1 << NOCP_CNT_SRC_INTEVENT_SHIFT;
pub const NOCP_CNT_SRC_INTEVENT_IDLE_MASK: u32 = 0x2 << NOCP_CNT_SRC_INTEVENT_SHIFT;
pub const NOCP_CNT_SRC_INTEVENT_XFER_MASK: u32 = 0x3 << NOCP_CNT_SRC_INTEVENT_SHIFT;
pub const NOCP_CNT_SRC_INTEVENT_BUSY_MASK: u32 = 0x4 << NOCP_CNT_SRC_INTEVENT_SHIFT;
pub const NOCP_CNT_SRC_INTEVENT_WAIT_MASK: u32 = 0x5 << NOCP_CNT_SRC_INTEVENT_SHIFT;
pub const NOCP_CNT_SRC_INTEVENT_PKT_MASK: u32 = 0x6 << NOCP_CNT_SRC_INTEVENT_SHIFT;
pub const NOCP_CNT_SRC_INTEVENT_BYTE_MASK: u32 = 0x8 << NOCP_CNT_SRC_INTEVENT_SHIFT;
pub const NOCP_CNT_SRC_INTEVENT_CHAIN_MASK: u32 = 0x10 << NOCP_CNT_SRC_INTEVENT_SHIFT;

/* NOCP_COUNTERS_x_ALARM_MODE register */
pub const NOCP_CNT_ALARM_MODE_SHIFT: u32 = 0;
pub const NOCP_CNT_ALARM_MODE_MASK: u32 = 0x3 << NOCP_CNT_ALARM_MODE_SHIFT;
pub const NOCP_CNT_ALARM_MODE_OFF_MASK: u32 = 0x0 << NOCP_CNT_ALARM_MODE_SHIFT;
pub const NOCP_CNT_ALARM_MODE_MIN_MASK: u32 = 0x1 << NOCP_CNT_ALARM_MODE_SHIFT;
pub const NOCP_CNT_ALARM_MODE_MAX_MASK: u32 = 0x2 << NOCP_CNT_ALARM_MODE_SHIFT;
pub const NOCP_CNT_ALARM_MODE_MIN_MAX_MASK: u32 = 0x3 << NOCP_CNT_ALARM_MODE_SHIFT;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
