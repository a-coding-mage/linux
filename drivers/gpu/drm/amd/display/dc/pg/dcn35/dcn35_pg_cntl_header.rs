/* SPDX-License-Identifier: MIT */
/*
 * Copyright 2023 Advanced Micro Devices, Inc.
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

// Dependency supplied by pg_cntl.h in the original C header.

macro_rules! PG_CNTL_REG_LIST_DCN35 {
    () => {
        SR!(DOMAIN0_PG_CONFIG), SR!(DOMAIN1_PG_CONFIG), SR!(DOMAIN2_PG_CONFIG),
        SR!(DOMAIN3_PG_CONFIG), SR!(DOMAIN16_PG_CONFIG), SR!(DOMAIN17_PG_CONFIG),
        SR!(DOMAIN18_PG_CONFIG), SR!(DOMAIN19_PG_CONFIG), SR!(DOMAIN22_PG_CONFIG),
        SR!(DOMAIN23_PG_CONFIG), SR!(DOMAIN24_PG_CONFIG), SR!(DOMAIN25_PG_CONFIG),
        SR!(DOMAIN0_PG_STATUS), SR!(DOMAIN1_PG_STATUS), SR!(DOMAIN2_PG_STATUS),
        SR!(DOMAIN3_PG_STATUS), SR!(DOMAIN16_PG_STATUS), SR!(DOMAIN17_PG_STATUS),
        SR!(DOMAIN18_PG_STATUS), SR!(DOMAIN19_PG_STATUS), SR!(DOMAIN22_PG_STATUS),
        SR!(DOMAIN23_PG_STATUS), SR!(DOMAIN24_PG_STATUS), SR!(DOMAIN25_PG_STATUS),
        SR!(DC_IP_REQUEST_CNTL)
    };
}

// C token-pasting macro retained as a declarative Rust placeholder.
macro_rules! PG_CNTL_SF { ($reg_name:ident, $field_name:ident, $post_fix:ident) => { $reg_name, $field_name, $post_fix }; }

macro_rules! PG_CNTL_REG_FIELD_LIST { ($type:ty) => { IPS2: $type, IPS1: $type, IPS0: $type, IPS0_All: $type }; }
macro_rules! PG_CNTL_DCN35_REG_FIELD_LIST { ($type:ty) => { IP_REQUEST_EN: $type, DOMAIN_POWER_FORCEON: $type, DOMAIN_POWER_GATE: $type, DOMAIN_DESIRED_PWR_STATE: $type, DOMAIN_PGFSM_PWR_STATUS: $type }; }

#[repr(C)]
pub struct pg_cntl_shift {
    pub IPS2: u8,
    pub IPS1: u8,
    pub IPS0: u8,
    pub IPS0_All: u8,
    pub IP_REQUEST_EN: u8,
    pub DOMAIN_POWER_FORCEON: u8,
    pub DOMAIN_POWER_GATE: u8,
    pub DOMAIN_DESIRED_PWR_STATE: u8,
    pub DOMAIN_PGFSM_PWR_STATUS: u8,
}

#[repr(C)]
pub struct pg_cntl_mask {
    pub IPS2: u32,
    pub IPS1: u32,
    pub IPS0: u32,
    pub IPS0_All: u32,
    pub IP_REQUEST_EN: u32,
    pub DOMAIN_POWER_FORCEON: u32,
    pub DOMAIN_POWER_GATE: u32,
    pub DOMAIN_DESIRED_PWR_STATE: u32,
    pub DOMAIN_PGFSM_PWR_STATUS: u32,
}

#[repr(C)]
pub struct pg_cntl_registers {
    pub LONO_STATE: u32,
    pub DC_IP_REQUEST_CNTL: u32,
    pub DOMAIN0_PG_CONFIG: u32,
    pub DOMAIN1_PG_CONFIG: u32,
    pub DOMAIN2_PG_CONFIG: u32,
    pub DOMAIN3_PG_CONFIG: u32,
    pub DOMAIN16_PG_CONFIG: u32,
    pub DOMAIN17_PG_CONFIG: u32,
    pub DOMAIN18_PG_CONFIG: u32,
    pub DOMAIN19_PG_CONFIG: u32,
    pub DOMAIN22_PG_CONFIG: u32,
    pub DOMAIN23_PG_CONFIG: u32,
    pub DOMAIN24_PG_CONFIG: u32,
    pub DOMAIN25_PG_CONFIG: u32,
    pub DOMAIN0_PG_STATUS: u32,
    pub DOMAIN1_PG_STATUS: u32,
    pub DOMAIN2_PG_STATUS: u32,
    pub DOMAIN3_PG_STATUS: u32,
    pub DOMAIN16_PG_STATUS: u32,
    pub DOMAIN17_PG_STATUS: u32,
    pub DOMAIN18_PG_STATUS: u32,
    pub DOMAIN19_PG_STATUS: u32,
    pub DOMAIN22_PG_STATUS: u32,
    pub DOMAIN23_PG_STATUS: u32,
    pub DOMAIN24_PG_STATUS: u32,
    pub DOMAIN25_PG_STATUS: u32,
}

#[repr(C)]
pub struct dcn_pg_cntl {
    pub base: pg_cntl,
    pub regs: *const pg_cntl_registers,
    pub pg_cntl_shift: *const pg_cntl_shift,
    pub pg_cntl_mask: *const pg_cntl_mask,
}

extern "C" {
    pub fn pg_cntl35_dsc_pg_control(pg_cntl: *mut pg_cntl, dsc_inst: u32, power_on: bool);
    pub fn pg_cntl35_hubp_dpp_pg_control(pg_cntl: *mut pg_cntl, hubp_dpp_inst: u32, power_on: bool);
    pub fn pg_cntl35_hpo_pg_control(pg_cntl: *mut pg_cntl, power_on: bool);
    pub fn pg_cntl35_io_clk_pg_control(pg_cntl: *mut pg_cntl, power_on: bool);
    pub fn pg_cntl35_plane_otg_pg_control(pg_cntl: *mut pg_cntl, power_on: bool);
    pub fn pg_cntl35_mpcc_pg_control(pg_cntl: *mut pg_cntl, mpcc_inst: u32, power_on: bool);
    pub fn pg_cntl35_opp_pg_control(pg_cntl: *mut pg_cntl, opp_inst: u32, power_on: bool);
    pub fn pg_cntl35_optc_pg_control(pg_cntl: *mut pg_cntl, optc_inst: u32, power_on: bool);
    pub fn pg_cntl35_dwb_pg_control(pg_cntl: *mut pg_cntl, power_on: bool);
    pub fn pg_cntl35_init_pg_status(pg_cntl: *mut pg_cntl);
    pub fn pg_cntl35_create(ctx: *mut dc_context, regs: *const pg_cntl_registers, pg_cntl_shift: *const pg_cntl_shift, pg_cntl_mask: *const pg_cntl_mask) -> *mut pg_cntl;
    pub fn dcn_pg_cntl_destroy(pg_cntl: *mut *mut pg_cntl);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
