/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * Copyright (c) Qualcomm Technologies, Inc. and/or its subsidiaries.
 */

//! Rust translation of `qcom_scm_trace.h`.
//!
//! The Linux tracepoint registration performed by `TRACE_EVENT` and
//! `define_trace.h` is supplied by the surrounding trace framework.

use core::cmp::min;

/// Arguments passed to an ARM SMC call (the fields used by this header).
#[repr(C)]
pub struct ArmSmcccArgs {
    pub args: [core::ffi::c_ulong; 8],
}

/// Results returned by an ARM SMC call (the fields used by this header).
#[repr(C)]
pub struct ArmSmcccRes {
    pub a0: core::ffi::c_ulong,
    pub a1: core::ffi::c_ulong,
    pub a2: core::ffi::c_ulong,
    pub a3: core::ffi::c_ulong,
}

#[repr(C)]
pub struct ScmSmcRequest {
    pub smc_id: u64,
    pub svc_id: u8,
    pub cmd_id: u8,
    pub args_cnt: u8,
    pub args: [core::ffi::c_ulong; 6],
}

/// Translation of the `scm_smc_request` tracepoint's fast assignment.
pub unsafe fn scm_smc_request(a0: core::ffi::c_ulong, smc: *const ArmSmcccArgs) -> ScmSmcRequest {
    let smc_ref = &*smc;
    let n = min((smc_ref.args[1] & 0xF) as u8, 6u8);
    let mut entry = ScmSmcRequest {
        smc_id: a0 as u64,
        svc_id: ((smc_ref.args[0] >> 8) & 0xFF) as u8,
        cmd_id: (smc_ref.args[0] & 0xFF) as u8,
        args_cnt: n,
        args: [0; 6],
    };

    for i in 0..n as usize {
        entry.args[i] = smc_ref.args[2 + i];
    }
    entry
}

#[repr(C)]
pub struct ScmWaitqSleep {
    pub wq_ctx: u32,
    pub smc_call_ctx: u32,
}

pub fn scm_waitq_sleep(wq_ctx: u32, smc_ctx: u32) -> ScmWaitqSleep {
    ScmWaitqSleep { wq_ctx, smc_call_ctx: smc_ctx }
}

#[repr(C)]
pub struct ScmWaitqResume {
    pub smc_call_ctx: u32,
}

pub fn scm_waitq_resume(smc_ctx: u32) -> ScmWaitqResume {
    ScmWaitqResume { smc_call_ctx: smc_ctx }
}

#[repr(C)]
pub struct ScmWaitqGetWqCtx {
    pub wq_ctx: u32,
    pub flags: u32,
    pub more_pending: u32,
}

pub fn scm_waitq_get_wq_ctx(wq_ctx: u32, flags: u32, pending: u32) -> ScmWaitqGetWqCtx {
    ScmWaitqGetWqCtx { wq_ctx, flags, more_pending: pending }
}

#[repr(C)]
pub struct ScmSmcDone {
    pub ret: i32,
    pub smc_id: u64,
    pub res: core::ffi::c_ulong,
    pub res0: core::ffi::c_ulong,
    pub res1: core::ffi::c_ulong,
    pub res2: core::ffi::c_ulong,
}

/// Translation of the `scm_smc_done` tracepoint's fast assignment.
pub unsafe fn scm_smc_done(
    ret: i32,
    smc_id: u64,
    smc_res: *const ArmSmcccRes,
) -> ScmSmcDone {
    let result = &*smc_res;
    ScmSmcDone {
        ret,
        smc_id,
        res: result.a0,
        res0: result.a1,
        res1: result.a2,
        res2: result.a3,
    }
}


// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
