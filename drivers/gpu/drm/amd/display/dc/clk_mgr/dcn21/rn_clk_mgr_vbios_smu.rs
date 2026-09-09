/*
 * Copyright 2012-16 Advanced Micro Devices, Inc.
 *
 * Permission is hereby granted, free of charge, to any person obtaining a
 * copy of this software and associated documentation files (the "Software"),
 * to deal in the Software without restriction, including without limitation
 * the rights to use, copy, modify, merge, publish, distribute, sublicense,
 * and/or sell copies of the Software, and to permit persons to whom the
 * Software is furnished to do so, subject to the following conditions:
 *
 * The above copyright notice and this permission notice shall be included in
 * all copies or substantial portions of the Software.
 *
 * THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
 * IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
 * FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT.  IN NO EVENT SHALL
 * THE COPYRIGHT HOLDER(S) OR AUTHOR(S) BE LIABLE FOR ANY CLAIM, DAMAGES OR
 * OTHER LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE,
 * ARISING FROM, OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR
 * OTHER DEALINGS IN THE SOFTWARE.
 *
 * Authors: AMD
 */

// Dependencies supplied by the surrounding driver and kernel bindings.

const VBIOSSMC_MSG_TEST_MESSAGE: u32 = 0x1;
const VBIOSSMC_MSG_GET_SMU_VERSION: u32 = 0x2;
const VBIOSSMC_MSG_POWER_UP_GFX: u32 = 0x3;
const VBIOSSMC_MSG_SET_DISPCLK_FREQ: u32 = 0x4;
const VBIOSSMC_MSG_SET_DPREFCLK_FREQ: u32 = 0x5;
const VBIOSSMC_MSG_POWER_DOWN_GFX: u32 = 0x6;
const VBIOSSMC_MSG_SET_DPPCLK_FREQ: u32 = 0x7;
const VBIOSSMC_MSG_SET_HARD_MIN_DCFCLK_BY_FREQ: u32 = 0x8;
const VBIOSSMC_MSG_SET_MIN_DEEP_SLEEP_DCFCLK: u32 = 0x9;
const VBIOSSMC_MSG_SET_PHYCLK_VOLTAGE_BY_FREQ: u32 = 0xA;
const VBIOSSMC_MSG_GET_FCLK_FREQUENCY: u32 = 0xB;
const VBIOSSMC_MSG_SET_DISPLAY_COUNT: u32 = 0xC;
const VBIOSSMC_MSG_ENABLE_TMDP48MHZ_REFCLK_PWR_DOWN: u32 = 0xD;
const VBIOSSMC_MSG_UPDATE_PME_RESTORE: u32 = 0xE;
const VBIOSSMC_MSG_IS_PERIODIC_RETRAINING_DISABLED: u32 = 0xF;

const VBIOSSMC_STATUS_BUSY: u32 = 0x0;
const VBIOSSMC_RESULT_OK: u32 = 0x1;
const VBIOSSMC_RESULT_FAILED: u32 = 0xFF;
const VBIOSSMC_RESULT_UNKNOWN_CMD: u32 = 0xFE;
const VBIOSSMC_RESULT_CMD_REJECTED_PREREQ: u32 = 0xFD;
const VBIOSSMC_RESULT_CMD_REJECTED_BUSY: u32 = 0xFC;

/*
 * Function to be used instead of REG_WAIT macro because the wait ends when
 * the register is NOT EQUAL to zero, and because the translation in msg_if.h
 * won't work with REG_WAIT.
 */
unsafe fn rn_smu_wait_for_response(
    clk_mgr: *mut clk_mgr_internal,
    delay_us: u32,
    mut max_retries: u32,
) -> u32 {
    let mut res_val: u32 = VBIOSSMC_STATUS_BUSY;

    loop {
        res_val = REG_READ!(MP1_SMN_C2PMSG_91);
        if res_val != VBIOSSMC_STATUS_BUSY {
            break;
        }

        if delay_us >= 1000 {
            msleep(delay_us / 1000);
        } else if delay_us > 0 {
            udelay(delay_us);
        }

        if max_retries == 0 {
            break;
        }
        max_retries = max_retries.wrapping_sub(1);
    }

    res_val
}

unsafe fn rn_vbios_smu_send_msg_with_param(
    clk_mgr: *mut clk_mgr_internal,
    msg_id: u32,
    param: u32,
) -> i32 {
    let mut result: u32;

    result = rn_smu_wait_for_response(clk_mgr, 10, 200000);

    if result != VBIOSSMC_RESULT_OK {
        smu_print!("SMU Response was not OK. SMU response after wait received is: %d\n", result);
    }

    if result == VBIOSSMC_STATUS_BUSY {
        return -1;
    }

    /* First clear response register */
    REG_WRITE!(MP1_SMN_C2PMSG_91, VBIOSSMC_STATUS_BUSY);

    /* Set the parameter register for the SMU message, unit is Mhz */
    REG_WRITE!(MP1_SMN_C2PMSG_83, param);

    /* Trigger the message transaction by writing the message ID */
    REG_WRITE!(MP1_SMN_C2PMSG_67, msg_id);

    result = rn_smu_wait_for_response(clk_mgr, 10, 200000);

    if IS_SMU_TIMEOUT!(result) {
        ASSERT!(0);
        dm_helpers_smu_timeout!(CTX!(clk_mgr), msg_id, param, 10 * 200000);
    }

    /* Actual dispclk set is returned in the parameter register */
    REG_READ!(MP1_SMN_C2PMSG_83) as i32
}

unsafe fn rn_vbios_smu_get_smu_version(clk_mgr: *mut clk_mgr_internal) -> i32 {
    rn_vbios_smu_send_msg_with_param(clk_mgr, VBIOSSMC_MSG_GET_SMU_VERSION, 0)
}

unsafe fn rn_vbios_smu_set_dispclk(
    clk_mgr: *mut clk_mgr_internal,
    requested_dispclk_khz: i32,
) -> i32 {
    let mut actual_dispclk_set_mhz: i32 = -1;
    let dc = (*(*clk_mgr).base.ctx).dc;
    let dmcu = (*dc).res_pool.dmcu;

    /* Unit of SMU msg parameter is Mhz */
    actual_dispclk_set_mhz = rn_vbios_smu_send_msg_with_param(
        clk_mgr,
        VBIOSSMC_MSG_SET_DISPCLK_FREQ,
        khz_to_mhz_ceil!(requested_dispclk_khz),
    );

    if !dmcu.is_null() && ((*dmcu).funcs.is_dmcu_initialized)(dmcu) {
        if (*clk_mgr).dfs_bypass_disp_clk != actual_dispclk_set_mhz {
            ((*dmcu).funcs.set_psr_wait_loop)(dmcu, actual_dispclk_set_mhz / 7);
        }
    }

    // pmfw always set clock more than or equal requested clock
    ASSERT!(actual_dispclk_set_mhz >= khz_to_mhz_ceil!(requested_dispclk_khz));

    actual_dispclk_set_mhz * 1000
}

unsafe fn rn_vbios_smu_set_hard_min_dcfclk(
    clk_mgr: *mut clk_mgr_internal,
    requested_dcfclk_khz: i32,
) -> i32 {
    let mut actual_dcfclk_set_mhz: i32 = -1;

    if (*clk_mgr).smu_ver < 0x370c00 {
        return actual_dcfclk_set_mhz;
    }

    actual_dcfclk_set_mhz = rn_vbios_smu_send_msg_with_param(
        clk_mgr,
        VBIOSSMC_MSG_SET_HARD_MIN_DCFCLK_BY_FREQ,
        khz_to_mhz_ceil!(requested_dcfclk_khz),
    );

    actual_dcfclk_set_mhz * 1000
}

unsafe fn rn_vbios_smu_set_min_deep_sleep_dcfclk(
    clk_mgr: *mut clk_mgr_internal,
    requested_min_ds_dcfclk_khz: i32,
) -> i32 {
    let mut actual_min_ds_dcfclk_mhz: i32 = -1;

    if (*clk_mgr).smu_ver < 0x370c00 {
        return actual_min_ds_dcfclk_mhz;
    }

    actual_min_ds_dcfclk_mhz = rn_vbios_smu_send_msg_with_param(
        clk_mgr,
        VBIOSSMC_MSG_SET_MIN_DEEP_SLEEP_DCFCLK,
        khz_to_mhz_ceil!(requested_min_ds_dcfclk_khz),
    );

    actual_min_ds_dcfclk_mhz * 1000
}

unsafe fn rn_vbios_smu_set_phyclk(clk_mgr: *mut clk_mgr_internal, requested_phyclk_khz: i32) {
    rn_vbios_smu_send_msg_with_param(
        clk_mgr,
        VBIOSSMC_MSG_SET_PHYCLK_VOLTAGE_BY_FREQ,
        khz_to_mhz_ceil!(requested_phyclk_khz),
    );
}

unsafe fn rn_vbios_smu_set_dppclk(
    clk_mgr: *mut clk_mgr_internal,
    requested_dpp_khz: i32,
) -> i32 {
    let mut actual_dppclk_set_mhz: i32 = -1;

    actual_dppclk_set_mhz = rn_vbios_smu_send_msg_with_param(
        clk_mgr,
        VBIOSSMC_MSG_SET_DPPCLK_FREQ,
        khz_to_mhz_ceil!(requested_dpp_khz),
    );

    ASSERT!(actual_dppclk_set_mhz >= khz_to_mhz_ceil!(requested_dpp_khz));

    actual_dppclk_set_mhz * 1000
}

unsafe fn rn_vbios_smu_set_dcn_low_power_state(
    clk_mgr: *mut clk_mgr_internal,
    state: dcn_pwr_state,
) {
    let disp_count: u32;

    if state == DCN_PWR_STATE_LOW_POWER {
        disp_count = 0;
    } else {
        disp_count = 1;
    }

    rn_vbios_smu_send_msg_with_param(clk_mgr, VBIOSSMC_MSG_SET_DISPLAY_COUNT, disp_count);
}

unsafe fn rn_vbios_smu_enable_48mhz_tmdp_refclk_pwrdwn(
    clk_mgr: *mut clk_mgr_internal,
    enable: bool,
) {
    rn_vbios_smu_send_msg_with_param(
        clk_mgr,
        VBIOSSMC_MSG_ENABLE_TMDP48MHZ_REFCLK_PWR_DOWN,
        enable as u32,
    );
}

unsafe fn rn_vbios_smu_enable_pme_wa(clk_mgr: *mut clk_mgr_internal) {
    rn_vbios_smu_send_msg_with_param(clk_mgr, VBIOSSMC_MSG_UPDATE_PME_RESTORE, 0);
}

unsafe fn rn_vbios_smu_is_periodic_retraining_disabled(
    clk_mgr: *mut clk_mgr_internal,
) -> i32 {
    rn_vbios_smu_send_msg_with_param(
        clk_mgr,
        VBIOSSMC_MSG_IS_PERIODIC_RETRAINING_DISABLED,
        1, // if PMFW doesn't support this message, assume retraining is disabled
           // so we only use most optimal watermark if we know retraining is enabled.
    )
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
