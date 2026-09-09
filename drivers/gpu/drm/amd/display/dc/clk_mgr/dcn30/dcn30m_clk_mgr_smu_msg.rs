/*
 * Copyright 2020 Advanced Micro Devices, Inc.
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

// Dependencies supplied by the surrounding translation unit.

pub const MM_DAL_MSG_REG: u32 = 0x1628A;
pub const MM_DAL_ARG_REG: u32 = 0x16273;
pub const MM_DAL_RESP_REG: u32 = 0x16274;

// The C REG(reg_name) macro expands mm##reg_name.

extern "C" {
    fn msleep(milliseconds: u32);
    fn udelay(microseconds: u32);
}

#[repr(C)]
pub struct clk_mgr_internal {
    _private: [u8; 0],
}

unsafe fn dcn30m_smu_wait_for_response(
    clk_mgr: *mut clk_mgr_internal,
    delay_us: u32,
    mut max_retries: u32,
) -> u32 {
    let mut reg: u32 = 0;

    loop {
        reg = REG_READ(clk_mgr, DAL_RESP_REG);
        if reg != 0 {
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

    /* handle DALSMC_Result_CmdRejectedBusy? */

    /* Log? */

    reg
}

unsafe fn dcn30m_smu_send_msg_with_param(
    clk_mgr: *mut clk_mgr_internal,
    msg_id: u32,
    param_in: u32,
    param_out: *mut u32,
) -> bool {
    let result: u32;
    /* Wait for response register to be ready */
    dcn30m_smu_wait_for_response(clk_mgr, 10, 200000);

    /* Clear response register */
    REG_WRITE(clk_mgr, DAL_RESP_REG, 0);

    /* Set the parameter register for the SMU message */
    REG_WRITE(clk_mgr, DAL_ARG_REG, param_in);

    /* Trigger the message transaction by writing the message ID */
    REG_WRITE(clk_mgr, DAL_MSG_REG, msg_id);

    result = dcn30m_smu_wait_for_response(clk_mgr, 10, 200000);

    if IS_SMU_TIMEOUT(result) {
        dm_helpers_smu_timeout(CTX, msg_id, param_in, 10 * 200000);
    }

    /* Wait for response */
    if result == DALSMC_Result_OK {
        if !param_out.is_null() {
            *param_out = REG_READ(clk_mgr, DAL_ARG_REG);
        }

        return true;
    }

    false
}

pub unsafe fn dcn30m_smu_set_smart_mux_switch(
    clk_mgr: *mut clk_mgr_internal,
    pins_to_set: u32,
) -> u32 {
    let mut response: u32 = 0;

    smu_print!("SMU Set SmartMux Switch: switch_dgpu = %d\n", pins_to_set);

    dcn30m_smu_send_msg_with_param(
        clk_mgr,
        DALSMC_MSG_SmartAccess,
        pins_to_set,
        &mut response,
    );

    response
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
