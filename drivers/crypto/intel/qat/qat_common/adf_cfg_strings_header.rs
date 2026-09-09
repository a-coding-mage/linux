/* SPDX-License-Identifier: (BSD-3-Clause OR GPL-2.0-only) */
/* Copyright(c) 2014 - 2020 Intel Corporation */

pub const ADF_GENERAL_SEC: &str = "GENERAL";
pub const ADF_KERNEL_SEC: &str = "KERNEL";
pub const ADF_ACCEL_SEC: &str = "Accelerator";
pub const ADF_NUM_CY: &str = "NumberCyInstances";
pub const ADF_NUM_DC: &str = "NumberDcInstances";
pub const ADF_RING_SYM_SIZE: &str = "NumConcurrentSymRequests";
pub const ADF_RING_ASYM_SIZE: &str = "NumConcurrentAsymRequests";
pub const ADF_RING_DC_SIZE: &str = "NumConcurrentRequests";
pub const ADF_RING_ASYM_TX: &str = "RingAsymTx";
pub const ADF_RING_SYM_TX: &str = "RingSymTx";
pub const ADF_RING_ASYM_RX: &str = "RingAsymRx";
pub const ADF_RING_SYM_RX: &str = "RingSymRx";
pub const ADF_RING_DC_TX: &str = "RingTx";
pub const ADF_RING_DC_RX: &str = "RingRx";
pub const ADF_ETRMGR_BANK: &str = "Bank";
pub const ADF_RING_SYM_BANK_NUM: &str = "BankSymNumber";
pub const ADF_RING_ASYM_BANK_NUM: &str = "BankAsymNumber";
pub const ADF_RING_DC_BANK_NUM: &str = "BankDcNumber";
pub const ADF_CY: &str = "Cy";
pub const ADF_DC: &str = "Dc";
pub const ADF_CFG_DC: &str = "dc";
pub const ADF_CFG_DECOMP: &str = "decomp";
pub const ADF_CFG_CY: &str = "sym;asym";
pub const ADF_CFG_SYM: &str = "sym";
pub const ADF_CFG_ASYM: &str = "asym";
pub const ADF_CFG_DCC: &str = "dcc";
pub const ADF_SERVICES_ENABLED: &str = "ServicesEnabled";
pub const ADF_SERVICES_DELIMITER: &str = ";";
pub const ADF_PM_IDLE_SUPPORT: &str = "PmIdleSupport";
pub const ADF_ETRMGR_COALESCING_ENABLED: &str = "InterruptCoalescingEnabled";
pub const ADF_ETRMGR_COALESCING_ENABLED_FORMAT: &str = "Bank%dInterruptCoalescingEnabled";
pub const ADF_ETRMGR_COALESCE_TIMER: &str = "InterruptCoalescingTimerNs";
pub const ADF_ETRMGR_COALESCE_TIMER_FORMAT: &str = "Bank%dInterruptCoalescingTimerNs";
pub const ADF_ETRMGR_COALESCING_MSG_ENABLED: &str = "InterruptCoalescingNumResponses";
pub const ADF_ETRMGR_COALESCING_MSG_ENABLED_FORMAT: &str = "Bank%dInterruptCoalescingNumResponses";
pub const ADF_ETRMGR_CORE_AFFINITY: &str = "CoreAffinity";
pub const ADF_ETRMGR_CORE_AFFINITY_FORMAT: &str = "Bank%dCoreAffinity";
pub const ADF_ACCEL_STR: &str = "Accelerator%d";
pub const ADF_HEARTBEAT_TIMER: &str = "HeartbeatTimer";
pub const ADF_SRIOV_ENABLED: &str = "SriovEnabled";

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
