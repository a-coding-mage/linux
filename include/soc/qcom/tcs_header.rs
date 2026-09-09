/* SPDX-License-Identifier: GPL-2.0 */
/*
 * Copyright (c) 2016-2019, The Linux Foundation. All rights reserved.
 */

// Translated from the C header __SOC_QCOM_TCS_H__.

pub const MAX_RPMH_PAYLOAD: usize = 16;

/**
 * rpmh_state: state for the request
 *
 * RPMH_SLEEP_STATE:       State of the resource when the processor subsystem
 *                         is powered down. There is no client using the
 *                         resource actively.
 * RPMH_WAKE_ONLY_STATE:   Resume resource state to the value previously
 *                         requested before the processor was powered down.
 * RPMH_ACTIVE_ONLY_STATE: Active or AMC mode requests. Resource state
 *                         is aggregated immediately.
 */
#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum RpmhState {
    RPMH_SLEEP_STATE,
    RPMH_WAKE_ONLY_STATE,
    RPMH_ACTIVE_ONLY_STATE,
}

/**
 * struct tcs_cmd: an individual request to RPMH.
 *
 * @addr: the address of the resource slv_id:18:16 | offset:0:15
 * @data: the resource state request
 * @wait: ensure that this command is complete before returning.
 *        Setting "wait" here only makes sense during rpmh_write_batch() for
 *        active-only transfers, this is because:
 *        rpmh_write() - Always waits.
 *                       (DEFINE_RPMH_MSG_ONSTACK will set .wait_for_compl)
 *        rpmh_write_async() - Never waits.
 *                       (There's no request completion callback)
 */
#[repr(C)]
pub struct TcsCmd {
    pub addr: u32,
    pub data: u32,
    pub wait: u32,
}

/**
 * struct tcs_request: A set of tcs_cmds sent together in a TCS
 *
 * @state:          state for the request.
 * @is_read:        set for read only requests
 * @wait_for_compl: wait until we get a response from the h/w accelerator
 *                  (same as setting cmd->wait for all commands in the request)
 * @num_cmds:       the number of @cmds in this request
 * @cmds:           an array of tcs_cmds
 */
#[repr(C)]
pub struct TcsRequest {
    pub state: RpmhState,
    pub is_read: bool,
    pub wait_for_compl: u32,
    pub num_cmds: u32,
    pub cmds: *mut TcsCmd,
}

pub const BCM_TCS_CMD_COMMIT_MASK: u32 = 1u32 << 30;
pub const BCM_TCS_CMD_VALID_MASK: u32 = 1u32 << 29;
pub const BCM_TCS_CMD_VOTE_MASK: u32 = (1u32 << 14) - 1;
pub const BCM_TCS_CMD_VOTE_Y_MASK: u32 = (1u32 << 14) - 1;
pub const BCM_TCS_CMD_VOTE_X_MASK: u32 = ((1u32 << 14) - 1) << 14;

/* Construct a Bus Clock Manager (BCM) specific TCS command */
#[inline]
pub const fn bcm_tcs_cmd(commit: u32, valid: u32, vote_x: u32, vote_y: u32) -> u32 {
    ((commit << 30) & BCM_TCS_CMD_COMMIT_MASK)
        | ((valid << 29) & BCM_TCS_CMD_VALID_MASK)
        | ((vote_x << 14) & BCM_TCS_CMD_VOTE_X_MASK)
        | (vote_y & BCM_TCS_CMD_VOTE_Y_MASK)
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
