/*
 * Copyright 2021 Advanced Micro Devices, Inc.
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
 *
 */

// Dependencies supplied by the corresponding C headers:
// #include "core_types.h"
// #include "dcn30/dcn30_clk_mgr_smu_msg.h"

pub const FCLK_PSTATE_NOTSUPPORTED: u32 = 0x00;
pub const FCLK_PSTATE_SUPPORTED: u32 = 0x01;

// TODO Remove this MSG ID define after it becomes available in dalsmc
pub const DALSMC_MSG_SetCabForUclkPstate: u32 = 0x12;
pub const DALSMC_Result_OK: u32 = 0x1;

extern "C" {
    pub fn dcn32_smu_send_fclk_pstate_message(
        clk_mgr: *mut clk_mgr_internal,
        enable: bool,
    );
    pub fn dcn32_smu_send_cab_for_uclk_message(
        clk_mgr: *mut clk_mgr_internal,
        num_ways: u32,
    );
    pub fn dcn32_smu_transfer_wm_table_dram_2_smu(clk_mgr: *mut clk_mgr_internal);
    pub fn dcn32_smu_set_pme_workaround(clk_mgr: *mut clk_mgr_internal);
    pub fn dcn32_smu_set_hard_min_by_freq(
        clk_mgr: *mut clk_mgr_internal,
        clk: u32,
        freq_mhz: u16,
    ) -> u32;
    pub fn dcn32_smu_wait_for_dmub_ack_mclk(
        clk_mgr: *mut clk_mgr_internal,
        enable: bool,
    );
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
