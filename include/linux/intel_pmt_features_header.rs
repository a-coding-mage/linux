/* SPDX-License-Identifier: GPL-2.0 */

use core::ffi::c_char;

/* Common masks */
pub const PMT_CAP_TELEM: u32 = 1 << 0;
pub const PMT_CAP_WATCHER: u32 = 1 << 1;
pub const PMT_CAP_CRASHLOG: u32 = 1 << 2;
pub const PMT_CAP_STREAMING: u32 = 1 << 3;
pub const PMT_CAP_THRESHOLD: u32 = 1 << 4;
pub const PMT_CAP_WINDOW: u32 = 1 << 5;
pub const PMT_CAP_CONFIG: u32 = 1 << 6;
pub const PMT_CAP_TRACING: u32 = 1 << 7;
pub const PMT_CAP_INBAND: u32 = 1 << 8;
pub const PMT_CAP_OOB: u32 = 1 << 9;
pub const PMT_CAP_SECURED_CHAN: u32 = 1 << 10;

pub const PMT_CAP_PMT_SP: u32 = 1 << 11;
pub const PMT_CAP_PMT_SP_POLICY: u32 = 0x3f << 12;

/* Per Core Performance Telemetry (PCPT) specific masks */
pub const PMT_CAP_PCPT_CORE_PERF: u32 = 1 << 18;
pub const PMT_CAP_PCPT_CORE_C0_RES: u32 = 1 << 19;
pub const PMT_CAP_PCPT_CORE_ACTIVITY: u32 = 1 << 20;
pub const PMT_CAP_PCPT_CACHE_PERF: u32 = 1 << 21;
pub const PMT_CAP_PCPT_QUALITY_TELEM: u32 = 1 << 22;

/* Per Core Environmental Telemetry (PCET) specific masks */
pub const PMT_CAP_PCET_WORKPOINT_HIST: u32 = 1 << 18;
pub const PMT_CAP_PCET_CORE_CURR_TEMP: u32 = 1 << 19;
pub const PMT_CAP_PCET_CORE_INST_RES: u32 = 1 << 20;
pub const PMT_CAP_PCET_QUALITY_TELEM: u32 = 1 << 21; /* Same as PMT_CAP_PCPT */
pub const PMT_CAP_PCET_CORE_CDYN_LVL: u32 = 1 << 22;
pub const PMT_CAP_PCET_CORE_STRESS_LVL: u32 = 1 << 23;
pub const PMT_CAP_PCET_CORE_DAS: u32 = 1 << 24;
pub const PMT_CAP_PCET_FIVR_HEALTH: u32 = 1 << 25;
pub const PMT_CAP_PCET_ENERGY: u32 = 1 << 26;
pub const PMT_CAP_PCET_PEM_STATUS: u32 = 1 << 27;
pub const PMT_CAP_PCET_CORE_C_STATE: u32 = 1 << 28;

/* Per RMID Performance Telemetry specific masks */
pub const PMT_CAP_RMID_CORES_PERF: u32 = 1 << 18;
pub const PMT_CAP_RMID_CACHE_PERF: u32 = 1 << 19;
pub const PMT_CAP_RMID_PERF_QUAL: u32 = 1 << 20;

/* Accelerator Telemetry specific masks */
pub const PMT_CAP_ACCEL_CPM_TELEM: u32 = 1 << 18;
pub const PMT_CAP_ACCEL_TIP_TELEM: u32 = 1 << 19;

/* Uncore Telemetry specific masks */
pub const PMT_CAP_UNCORE_IO_CA_TELEM: u32 = 1 << 18;
pub const PMT_CAP_UNCORE_RMID_TELEM: u32 = 1 << 19;
pub const PMT_CAP_UNCORE_D2D_ULA_TELEM: u32 = 1 << 20;
pub const PMT_CAP_UNCORE_PKGC_TELEM: u32 = 1 << 21;

/* Crash Log specific masks */
pub const PMT_CAP_CRASHLOG_MAN_TRIG: u32 = 1 << 11;
pub const PMT_CAP_CRASHLOG_CORE: u32 = 1 << 12;
pub const PMT_CAP_CRASHLOG_UNCORE: u32 = 1 << 13;
pub const PMT_CAP_CRASHLOG_TOR: u32 = 1 << 14;
pub const PMT_CAP_CRASHLOG_S3M: u32 = 1 << 15;
pub const PMT_CAP_CRASHLOG_PERSISTENCY: u32 = 1 << 16;
pub const PMT_CAP_CRASHLOG_CLIP_GPIO: u32 = 1 << 17;
pub const PMT_CAP_CRASHLOG_PRE_RESET: u32 = 1 << 18;
pub const PMT_CAP_CRASHLOG_POST_RESET: u32 = 1 << 19;

/* PeTe Log specific masks */
pub const PMT_CAP_PETE_MAN_TRIG: u32 = 1 << 11;
pub const PMT_CAP_PETE_ENCRYPTION: u32 = 1 << 12;
pub const PMT_CAP_PETE_PERSISTENCY: u32 = 1 << 13;
pub const PMT_CAP_PETE_REQ_TOKENS: u32 = 1 << 14;
pub const PMT_CAP_PETE_PROD_ENABLED: u32 = 1 << 15;
pub const PMT_CAP_PETE_DEBUG_ENABLED: u32 = 1 << 16;

/* TPMI control specific masks */
pub const PMT_CAP_TPMI_MAILBOX: u32 = 1 << 11;
pub const PMT_CAP_TPMI_LOCK: u32 = 1 << 12;

/* Tracing specific masks */
pub const PMT_CAP_TRACE_SRAR: u32 = 1 << 11;
pub const PMT_CAP_TRACE_CORRECTABLE: u32 = 1 << 12;
pub const PMT_CAP_TRACE_MCTP: u32 = 1 << 13;
pub const PMT_CAP_TRACE_MRT: u32 = 1 << 14;

/* Per RMID Energy Telemetry specific masks */
pub const PMT_CAP_RMID_ENERGY: u32 = 1 << 18;
pub const PMT_CAP_RMID_ACTIVITY: u32 = 1 << 19;
pub const PMT_CAP_RMID_ENERGY_QUAL: u32 = 1 << 20;

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum pmt_feature_id {
    FEATURE_INVALID = 0x0,
    FEATURE_PER_CORE_PERF_TELEM = 0x1,
    FEATURE_PER_CORE_ENV_TELEM = 0x2,
    FEATURE_PER_RMID_PERF_TELEM = 0x3,
    FEATURE_ACCEL_TELEM = 0x4,
    FEATURE_UNCORE_TELEM = 0x5,
    FEATURE_CRASH_LOG = 0x6,
    FEATURE_PETE_LOG = 0x7,
    FEATURE_TPMI_CTRL = 0x8,
    FEATURE_RESERVED = 0x9,
    FEATURE_TRACING = 0xA,
    FEATURE_PER_RMID_ENERGY_TELEM = 0xB,
    FEATURE_MAX = 0xB,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum feature_layout {
    LAYOUT_RMID,
    LAYOUT_WATCHER,
    LAYOUT_COMMAND,
    LAYOUT_CAPS_ONLY,
}

#[repr(C)]
pub struct pmt_cap {
    pub mask: u32,
    pub name: *const c_char,
}

extern "C" {
    pub static pmt_feature_names: [*const c_char];
    pub static mut feature_layout: [feature_layout];
    pub static mut pmt_cap_common: [pmt_cap];
    pub static mut pmt_cap_pcpt: [pmt_cap];
    pub static mut pmt_caps_pcpt: [*mut pmt_cap];
    pub static mut pmt_cap_pcet: [pmt_cap];
    pub static mut pmt_caps_pcet: [*mut pmt_cap];
    pub static mut pmt_cap_rmid_perf: [pmt_cap];
    pub static mut pmt_caps_rmid_perf: [*mut pmt_cap];
    pub static mut pmt_cap_accel: [pmt_cap];
    pub static mut pmt_caps_accel: [*mut pmt_cap];
    pub static mut pmt_cap_uncore: [pmt_cap];
    pub static mut pmt_caps_uncore: [*mut pmt_cap];
    pub static mut pmt_cap_crashlog: [pmt_cap];
    pub static mut pmt_caps_crashlog: [*mut pmt_cap];
    pub static mut pmt_cap_pete: [pmt_cap];
    pub static mut pmt_caps_pete: [*mut pmt_cap];
    pub static mut pmt_cap_tpmi: [pmt_cap];
    pub static mut pmt_caps_tpmi: [*mut pmt_cap];
    pub static mut pmt_cap_s3m: [pmt_cap];
    pub static mut pmt_caps_s3m: [*mut pmt_cap];
    pub static mut pmt_cap_tracing: [pmt_cap];
    pub static mut pmt_caps_tracing: [*mut pmt_cap];
    pub static mut pmt_cap_rmid_energy: [pmt_cap];
    pub static mut pmt_caps_rmid_energy: [*mut pmt_cap];
}

#[inline]
pub fn pmt_feature_id_is_valid(id: pmt_feature_id) -> bool {
    if (id as u32) > (pmt_feature_id::FEATURE_MAX as u32) {
        return false;
    }

    if id == pmt_feature_id::FEATURE_INVALID || id == pmt_feature_id::FEATURE_RESERVED {
        return false;
    }

    true
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
