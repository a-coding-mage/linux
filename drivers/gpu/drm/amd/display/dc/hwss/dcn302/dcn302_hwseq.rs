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
 * FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL
 * THE COPYRIGHT HOLDER(S) OR AUTHOR(S) BE LIABLE FOR ANY CLAIM, DAMAGES OR
 * OTHER LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE,
 * ARISING FROM, OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR
 * OTHER DEALINGS IN THE SOFTWARE.
 *
 * Authors: AMD
 */

// Dependencies supplied by the surrounding translation unit.

macro_rules! DC_LOGGER_INIT { ($logger:ident) => {}; }
#[allow(unused_macros)]
macro_rules! CTX { ($hws:expr) => { $hws.ctx }; }
#[allow(unused_macros)]
macro_rules! REG { ($hws:expr, $reg:ident) => { $hws.regs.$reg }; }
#[allow(unused_macros)]
macro_rules! FN { ($hws:expr, $reg_name:ident, $field_name:ident) => {
    ($hws.shifts.$field_name, $hws.masks.$field_name)
}; }

pub unsafe fn dcn302_dpp_pg_control(
    hws: *mut dce_hwseq,
    dpp_inst: u32,
    power_on: bool,
) {
    let power_gate: u32 = if power_on { 0 } else { 1 };
    let pwr_status: u32 = if power_on { 0 } else { 2 };

    if (*(*hws).ctx).dc.debug.disable_dpp_power_gate { return; }
    if REG!(hws, DOMAIN1_PG_CONFIG) == 0 { return; }

    match dpp_inst {
        0 => {
            REG_UPDATE!(hws, DOMAIN1_PG_CONFIG, DOMAIN1_POWER_GATE, power_gate);
            REG_WAIT!(hws, DOMAIN1_PG_STATUS, DOMAIN1_PGFSM_PWR_STATUS, pwr_status, 1, 1000);
        }
        1 => {
            REG_UPDATE!(hws, DOMAIN3_PG_CONFIG, DOMAIN3_POWER_GATE, power_gate);
            REG_WAIT!(hws, DOMAIN3_PG_STATUS, DOMAIN3_PGFSM_PWR_STATUS, pwr_status, 1, 1000);
        }
        2 => {
            REG_UPDATE!(hws, DOMAIN5_PG_CONFIG, DOMAIN5_POWER_GATE, power_gate);
            REG_WAIT!(hws, DOMAIN5_PG_STATUS, DOMAIN5_PGFSM_PWR_STATUS, pwr_status, 1, 1000);
        }
        3 => {
            REG_UPDATE!(hws, DOMAIN7_PG_CONFIG, DOMAIN7_POWER_GATE, power_gate);
            REG_WAIT!(hws, DOMAIN7_PG_STATUS, DOMAIN7_PGFSM_PWR_STATUS, pwr_status, 1, 1000);
        }
        4 => {
            REG_UPDATE!(hws, DOMAIN9_PG_CONFIG, DOMAIN9_POWER_GATE, power_gate);
            REG_WAIT!(hws, DOMAIN9_PG_STATUS, DOMAIN9_PGFSM_PWR_STATUS, pwr_status, 1, 1000);
        }
        _ => { BREAK_TO_DEBUGGER!(); }
    }
}

pub unsafe fn dcn302_hubp_pg_control(
    hws: *mut dce_hwseq,
    hubp_inst: u32,
    power_on: bool,
) {
    let power_gate: u32 = if power_on { 0 } else { 1 };
    let pwr_status: u32 = if power_on { 0 } else { 2 };

    if (*(*hws).ctx).dc.debug.disable_hubp_power_gate { return; }
    if REG!(hws, DOMAIN0_PG_CONFIG) == 0 { return; }

    match hubp_inst {
        0 => { REG_UPDATE!(hws, DOMAIN0_PG_CONFIG, DOMAIN0_POWER_GATE, power_gate); REG_WAIT!(hws, DOMAIN0_PG_STATUS, DOMAIN0_PGFSM_PWR_STATUS, pwr_status, 1, 1000); }
        1 => { REG_UPDATE!(hws, DOMAIN2_PG_CONFIG, DOMAIN2_POWER_GATE, power_gate); REG_WAIT!(hws, DOMAIN2_PG_STATUS, DOMAIN2_PGFSM_PWR_STATUS, pwr_status, 1, 1000); }
        2 => { REG_UPDATE!(hws, DOMAIN4_PG_CONFIG, DOMAIN4_POWER_GATE, power_gate); REG_WAIT!(hws, DOMAIN4_PG_STATUS, DOMAIN4_PGFSM_PWR_STATUS, pwr_status, 1, 1000); }
        3 => { REG_UPDATE!(hws, DOMAIN6_PG_CONFIG, DOMAIN6_POWER_GATE, power_gate); REG_WAIT!(hws, DOMAIN6_PG_STATUS, DOMAIN6_PGFSM_PWR_STATUS, pwr_status, 1, 1000); }
        4 => { REG_UPDATE!(hws, DOMAIN8_PG_CONFIG, DOMAIN8_POWER_GATE, power_gate); REG_WAIT!(hws, DOMAIN8_PG_STATUS, DOMAIN8_PGFSM_PWR_STATUS, pwr_status, 1, 1000); }
        _ => { BREAK_TO_DEBUGGER!(); }
    }
}

pub unsafe fn dcn302_dsc_pg_control(
    hws: *mut dce_hwseq,
    dsc_inst: u32,
    power_on: bool,
) {
    let power_gate: u32 = if power_on { 0 } else { 1 };
    let pwr_status: u32 = if power_on { 0 } else { 2 };
    let mut org_ip_request_cntl: u32 = 0;

    if (*(*hws).ctx).dc.debug.disable_dsc_power_gate { return; }
    if REG!(hws, DOMAIN16_PG_CONFIG) == 0 { return; }

    REG_GET!(hws, DC_IP_REQUEST_CNTL, IP_REQUEST_EN, &mut org_ip_request_cntl);
    if org_ip_request_cntl == 0 { REG_SET!(hws, DC_IP_REQUEST_CNTL, 0, IP_REQUEST_EN, 1); }

    match dsc_inst {
        0 => { REG_UPDATE!(hws, DOMAIN16_PG_CONFIG, DOMAIN16_POWER_GATE, power_gate); REG_WAIT!(hws, DOMAIN16_PG_STATUS, DOMAIN16_PGFSM_PWR_STATUS, pwr_status, 1, 1000); }
        1 => { REG_UPDATE!(hws, DOMAIN17_PG_CONFIG, DOMAIN17_POWER_GATE, power_gate); REG_WAIT!(hws, DOMAIN17_PG_STATUS, DOMAIN17_PGFSM_PWR_STATUS, pwr_status, 1, 1000); }
        2 => { REG_UPDATE!(hws, DOMAIN18_PG_CONFIG, DOMAIN18_POWER_GATE, power_gate); REG_WAIT!(hws, DOMAIN18_PG_STATUS, DOMAIN18_PGFSM_PWR_STATUS, pwr_status, 1, 1000); }
        3 => { REG_UPDATE!(hws, DOMAIN19_PG_CONFIG, DOMAIN19_POWER_GATE, power_gate); REG_WAIT!(hws, DOMAIN19_PG_STATUS, DOMAIN19_PGFSM_PWR_STATUS, pwr_status, 1, 1000); }
        4 => { REG_UPDATE!(hws, DOMAIN20_PG_CONFIG, DOMAIN20_POWER_GATE, power_gate); REG_WAIT!(hws, DOMAIN20_PG_STATUS, DOMAIN20_PGFSM_PWR_STATUS, pwr_status, 1, 1000); }
        _ => { BREAK_TO_DEBUGGER!(); }
    }

    if org_ip_request_cntl == 0 { REG_SET!(hws, DC_IP_REQUEST_CNTL, 0, IP_REQUEST_EN, 0); }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
