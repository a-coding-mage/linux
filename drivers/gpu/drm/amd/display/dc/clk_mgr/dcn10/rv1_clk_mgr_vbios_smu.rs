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

const MAX_INSTANCE: usize = 5;
const MAX_SEGMENT: usize = 5;

#[repr(C)]
pub struct IP_BASE_INSTANCE {
    pub segment: [u32; MAX_SEGMENT],
}

#[repr(C)]
pub struct IP_BASE {
    pub instance: [IP_BASE_INSTANCE; MAX_INSTANCE],
}

static MP1_BASE: IP_BASE = IP_BASE {
    instance: [
        IP_BASE_INSTANCE { segment: [0x00016000, 0, 0, 0, 0] },
        IP_BASE_INSTANCE { segment: [0, 0, 0, 0, 0] },
        IP_BASE_INSTANCE { segment: [0, 0, 0, 0, 0] },
        IP_BASE_INSTANCE { segment: [0, 0, 0, 0, 0] },
        IP_BASE_INSTANCE { segment: [0, 0, 0, 0, 0] },
    ],
};

const MMMP1_SMN_C2PMSG_91: u32 = 0x29B;
const MMMP1_SMN_C2PMSG_83: u32 = 0x293;
const MMMP1_SMN_C2PMSG_67: u32 = 0x283;

const VBIOSSMC_MSG_SET_DISPCLK_FREQ: u32 = 0x4;
const VBIOSSMC_MSG_SET_DPREFCLK_FREQ: u32 = 0x5;
const VBIOSSMC_STATUS_BUSY: u32 = 0x0;
const VBIOSSMC_RESULT_OK: u32 = 0x1;
const VBIOSSMC_RESULT_FAILED: u32 = 0xFF;
const VBIOSSMC_RESULT_UNKNOWN_CMD: u32 = 0xFE;
const VBIOSSMC_RESULT_CMD_REJECTED_PREREQ: u32 = 0xFD;
const VBIOSSMC_RESULT_CMD_REJECTED_BUSY: u32 = 0xFC;

#[inline]
fn reg(reg: u32) -> u32 {
    MP1_BASE.instance[0].segment[0] + reg
}

/* Function to be used instead of REG_WAIT macro because the wait ends when
 * the register is NOT EQUAL to zero, and because the translation in msg_if.h
 * won't work with REG_WAIT. */
unsafe fn rv1_smu_wait_for_response(
    clk_mgr: *mut clk_mgr_internal,
    delay_us: u32,
    mut max_retries: u32,
) -> u32 {
    let mut res_val = VBIOSSMC_STATUS_BUSY;
    loop {
        res_val = REG_READ!(clk_mgr, reg(MMMP1_SMN_C2PMSG_91));
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
        max_retries -= 1;
    }
    res_val
}

unsafe fn rv1_vbios_smu_send_msg_with_param(
    clk_mgr: *mut clk_mgr_internal,
    msg_id: u32,
    param: u32,
) -> i32 {
    REG_WRITE!(clk_mgr, reg(MMMP1_SMN_C2PMSG_91), VBIOSSMC_STATUS_BUSY);
    REG_WRITE!(clk_mgr, reg(MMMP1_SMN_C2PMSG_83), param);
    REG_WRITE!(clk_mgr, reg(MMMP1_SMN_C2PMSG_67), msg_id);

    let result = rv1_smu_wait_for_response(clk_mgr, 10, 1000);
    ASSERT!(result == VBIOSSMC_RESULT_OK);
    REG_READ!(clk_mgr, reg(MMMP1_SMN_C2PMSG_83)) as i32
}

pub unsafe fn rv1_vbios_smu_set_dispclk(
    clk_mgr: *mut clk_mgr_internal,
    requested_dispclk_khz: i32,
) -> i32 {
    let mut actual_dispclk_set_mhz: i32 = -1;
    let dc = (*(*clk_mgr).base.ctx).dc;
    let dmcu = (*dc).res_pool.dmcu;

    actual_dispclk_set_mhz = rv1_vbios_smu_send_msg_with_param(
        clk_mgr,
        VBIOSSMC_MSG_SET_DISPCLK_FREQ,
        khz_to_mhz_ceil(requested_dispclk_khz),
    );

    if !dmcu.is_null() && ((*dmcu).funcs.is_dmcu_initialized)(dmcu) {
        if (*clk_mgr).dfs_bypass_disp_clk != actual_dispclk_set_mhz {
            ((*dmcu).funcs.set_psr_wait_loop)(dmcu, actual_dispclk_set_mhz / 7);
        }
    }
    actual_dispclk_set_mhz * 1000
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
