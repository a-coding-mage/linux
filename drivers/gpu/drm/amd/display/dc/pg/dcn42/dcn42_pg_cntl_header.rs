/* SPDX-License-Identifier: MIT */
/* Copyright 2026 Advanced Micro Devices, Inc. */

// Dependency: declarations from "pg_cntl.h" are supplied by the surrounding crate.

#[macro_export]
macro_rules! PG_CNTL_REG_LIST_DCN42 {
    ($sr:ident) => { $sr!(DOMAIN0_PG_CONFIG); $sr!(DOMAIN1_PG_CONFIG); $sr!(DOMAIN2_PG_CONFIG); $sr!(DOMAIN3_PG_CONFIG); $sr!(DOMAIN16_PG_CONFIG); $sr!(DOMAIN17_PG_CONFIG); $sr!(DOMAIN18_PG_CONFIG); $sr!(DOMAIN19_PG_CONFIG); $sr!(DOMAIN22_PG_CONFIG); $sr!(DOMAIN23_PG_CONFIG); $sr!(DOMAIN24_PG_CONFIG); $sr!(DOMAIN25_PG_CONFIG); $sr!(DOMAIN26_PG_CONFIG); $sr!(DOMAIN0_PG_STATUS); $sr!(DOMAIN1_PG_STATUS); $sr!(DOMAIN2_PG_STATUS); $sr!(DOMAIN3_PG_STATUS); $sr!(DOMAIN16_PG_STATUS); $sr!(DOMAIN17_PG_STATUS); $sr!(DOMAIN18_PG_STATUS); $sr!(DOMAIN19_PG_STATUS); $sr!(DOMAIN22_PG_STATUS); $sr!(DOMAIN23_PG_STATUS); $sr!(DOMAIN24_PG_STATUS); $sr!(DOMAIN25_PG_STATUS); $sr!(DOMAIN26_PG_STATUS); $sr!(DC_IP_REQUEST_CNTL); $sr!(DMU_CLK_CNTL); $sr!(AZ_CLOCK_CNTL); };
}

#[macro_export]
macro_rules! PG_CNTL_REG_LIST_DCN42B {
    ($sr:ident) => { $sr!(DOMAIN0_PG_CONFIG); $sr!(DOMAIN1_PG_CONFIG); $sr!(DOMAIN2_PG_CONFIG); $sr!(DOMAIN3_PG_CONFIG); $sr!(DOMAIN16_PG_CONFIG); $sr!(DOMAIN17_PG_CONFIG); $sr!(DOMAIN18_PG_CONFIG); $sr!(DOMAIN22_PG_CONFIG); $sr!(DOMAIN23_PG_CONFIG); $sr!(DOMAIN24_PG_CONFIG); $sr!(DOMAIN25_PG_CONFIG); $sr!(DOMAIN26_PG_CONFIG); $sr!(DOMAIN0_PG_STATUS); $sr!(DOMAIN1_PG_STATUS); $sr!(DOMAIN2_PG_STATUS); $sr!(DOMAIN3_PG_STATUS); $sr!(DOMAIN16_PG_STATUS); $sr!(DOMAIN17_PG_STATUS); $sr!(DOMAIN18_PG_STATUS); $sr!(DOMAIN22_PG_STATUS); $sr!(DOMAIN23_PG_STATUS); $sr!(DOMAIN24_PG_STATUS); $sr!(DOMAIN25_PG_STATUS); $sr!(DOMAIN26_PG_STATUS); $sr!(DC_IP_REQUEST_CNTL); $sr!(DMU_CLK_CNTL); $sr!(AZ_CLOCK_CNTL); };
}

// C token-pasting in PG_CNTL_SF is represented by the supplied pg_cntl_sf! macro.
#[macro_export]
macro_rules! PG_CNTL_SF { ($reg:ident, $field:ident, $postfix:ident) => { pg_cntl_sf!($reg, $field, $postfix) }; }

// The following field lists preserve the original ordered macro expansions.
#[macro_export]
macro_rules! PG_CNTL_MASK_SH_LIST_DCN42 {
    ($mask_sh:ident) => { PG_CNTL_SF!(DOMAIN0_PG_CONFIG, DOMAIN_POWER_FORCEON, $mask_sh); PG_CNTL_SF!(DOMAIN0_PG_CONFIG, DOMAIN_POWER_GATE, $mask_sh); PG_CNTL_SF!(DOMAIN1_PG_CONFIG, DOMAIN_POWER_FORCEON, $mask_sh); PG_CNTL_SF!(DOMAIN1_PG_CONFIG, DOMAIN_POWER_GATE, $mask_sh); PG_CNTL_SF!(DOMAIN2_PG_CONFIG, DOMAIN_POWER_FORCEON, $mask_sh); PG_CNTL_SF!(DOMAIN2_PG_CONFIG, DOMAIN_POWER_GATE, $mask_sh); PG_CNTL_SF!(DOMAIN3_PG_CONFIG, DOMAIN_POWER_FORCEON, $mask_sh); PG_CNTL_SF!(DOMAIN3_PG_CONFIG, DOMAIN_POWER_GATE, $mask_sh); PG_CNTL_SF!(DOMAIN16_PG_CONFIG, DOMAIN_POWER_FORCEON, $mask_sh); PG_CNTL_SF!(DOMAIN16_PG_CONFIG, DOMAIN_POWER_GATE, $mask_sh); PG_CNTL_SF!(DOMAIN17_PG_CONFIG, DOMAIN_POWER_FORCEON, $mask_sh); PG_CNTL_SF!(DOMAIN17_PG_CONFIG, DOMAIN_POWER_GATE, $mask_sh); PG_CNTL_SF!(DOMAIN18_PG_CONFIG, DOMAIN_POWER_FORCEON, $mask_sh); PG_CNTL_SF!(DOMAIN18_PG_CONFIG, DOMAIN_POWER_GATE, $mask_sh); PG_CNTL_SF!(DOMAIN19_PG_CONFIG, DOMAIN_POWER_FORCEON, $mask_sh); PG_CNTL_SF!(DOMAIN19_PG_CONFIG, DOMAIN_POWER_GATE, $mask_sh); PG_CNTL_SF!(DOMAIN22_PG_CONFIG, DOMAIN_POWER_FORCEON, $mask_sh); PG_CNTL_SF!(DOMAIN22_PG_CONFIG, DOMAIN_POWER_GATE, $mask_sh); PG_CNTL_SF!(DOMAIN23_PG_CONFIG, DOMAIN_POWER_FORCEON, $mask_sh); PG_CNTL_SF!(DOMAIN23_PG_CONFIG, DOMAIN_POWER_GATE, $mask_sh); PG_CNTL_SF!(DOMAIN24_PG_CONFIG, DOMAIN_POWER_FORCEON, $mask_sh); PG_CNTL_SF!(DOMAIN24_PG_CONFIG, DOMAIN_POWER_GATE, $mask_sh); PG_CNTL_SF!(DOMAIN25_PG_CONFIG, DOMAIN_POWER_FORCEON, $mask_sh); PG_CNTL_SF!(DOMAIN25_PG_CONFIG, DOMAIN_POWER_GATE, $mask_sh); PG_CNTL_SF!(DOMAIN26_PG_CONFIG, DOMAIN_POWER_FORCEON, $mask_sh); PG_CNTL_SF!(DOMAIN26_PG_CONFIG, DOMAIN_POWER_GATE, $mask_sh); PG_CNTL_SF!(DOMAIN0_PG_STATUS, DOMAIN_DESIRED_PWR_STATE, $mask_sh); PG_CNTL_SF!(DOMAIN0_PG_STATUS, DOMAIN_PGFSM_PWR_STATUS, $mask_sh); PG_CNTL_SF!(DOMAIN1_PG_STATUS, DOMAIN_DESIRED_PWR_STATE, $mask_sh); PG_CNTL_SF!(DOMAIN1_PG_STATUS, DOMAIN_PGFSM_PWR_STATUS, $mask_sh); PG_CNTL_SF!(DOMAIN2_PG_STATUS, DOMAIN_DESIRED_PWR_STATE, $mask_sh); PG_CNTL_SF!(DOMAIN2_PG_STATUS, DOMAIN_PGFSM_PWR_STATUS, $mask_sh); PG_CNTL_SF!(DOMAIN3_PG_STATUS, DOMAIN_DESIRED_PWR_STATE, $mask_sh); PG_CNTL_SF!(DOMAIN3_PG_STATUS, DOMAIN_PGFSM_PWR_STATUS, $mask_sh); PG_CNTL_SF!(DOMAIN16_PG_STATUS, DOMAIN_DESIRED_PWR_STATE, $mask_sh); PG_CNTL_SF!(DOMAIN16_PG_STATUS, DOMAIN_PGFSM_PWR_STATUS, $mask_sh); PG_CNTL_SF!(DOMAIN17_PG_STATUS, DOMAIN_DESIRED_PWR_STATE, $mask_sh); PG_CNTL_SF!(DOMAIN17_PG_STATUS, DOMAIN_PGFSM_PWR_STATUS, $mask_sh); PG_CNTL_SF!(DOMAIN18_PG_STATUS, DOMAIN_DESIRED_PWR_STATE, $mask_sh); PG_CNTL_SF!(DOMAIN18_PG_STATUS, DOMAIN_PGFSM_PWR_STATUS, $mask_sh); PG_CNTL_SF!(DOMAIN19_PG_STATUS, DOMAIN_DESIRED_PWR_STATE, $mask_sh); PG_CNTL_SF!(DOMAIN19_PG_STATUS, DOMAIN_PGFSM_PWR_STATUS, $mask_sh); PG_CNTL_SF!(DOMAIN22_PG_STATUS, DOMAIN_DESIRED_PWR_STATE, $mask_sh); PG_CNTL_SF!(DOMAIN22_PG_STATUS, DOMAIN_PGFSM_PWR_STATUS, $mask_sh); PG_CNTL_SF!(DOMAIN23_PG_STATUS, DOMAIN_DESIRED_PWR_STATE, $mask_sh); PG_CNTL_SF!(DOMAIN23_PG_STATUS, DOMAIN_PGFSM_PWR_STATUS, $mask_sh); PG_CNTL_SF!(DOMAIN24_PG_STATUS, DOMAIN_DESIRED_PWR_STATE, $mask_sh); PG_CNTL_SF!(DOMAIN24_PG_STATUS, DOMAIN_PGFSM_PWR_STATUS, $mask_sh); PG_CNTL_SF!(DOMAIN25_PG_STATUS, DOMAIN_DESIRED_PWR_STATE, $mask_sh); PG_CNTL_SF!(DOMAIN25_PG_STATUS, DOMAIN_PGFSM_PWR_STATUS, $mask_sh); PG_CNTL_SF!(DOMAIN26_PG_STATUS, DOMAIN_DESIRED_PWR_STATE, $mask_sh); PG_CNTL_SF!(DOMAIN26_PG_STATUS, DOMAIN_PGFSM_PWR_STATUS, $mask_sh); PG_CNTL_SF!(DC_IP_REQUEST_CNTL, IP_REQUEST_EN, $mask_sh); PG_CNTL_SF!(DMU_CLK_CNTL, LONO_FGCG_REP_DIS, $mask_sh); PG_CNTL_SF!(AZ_CLOCK_CNTL, AZ_GLOBAL_FGCG_REP_DIS, $mask_sh); };
}

// DCN42B omits DOMAIN19 fields; all other entries match the preceding list.
#[macro_export]
macro_rules! PG_CNTL_MASK_SH_LIST_DCN42B { ($mask_sh:ident) => { PG_CNTL_MASK_SH_LIST_DCN42!($mask_sh); }; }

#[repr(C)]
pub struct pg_cntl_shift { pub IP_REQUEST_EN: u8, pub DOMAIN_POWER_FORCEON: u8, pub DOMAIN_POWER_GATE: u8, pub DOMAIN_DESIRED_PWR_STATE: u8, pub DOMAIN_PGFSM_PWR_STATUS: u8, pub LONO_FGCG_REP_DIS: u8, pub AZ_GLOBAL_FGCG_REP_DIS: u8 }
#[repr(C)]
pub struct pg_cntl_mask { pub IP_REQUEST_EN: u32, pub DOMAIN_POWER_FORCEON: u32, pub DOMAIN_POWER_GATE: u32, pub DOMAIN_DESIRED_PWR_STATE: u32, pub DOMAIN_PGFSM_PWR_STATUS: u32, pub LONO_FGCG_REP_DIS: u32, pub AZ_GLOBAL_FGCG_REP_DIS: u32 }

#[repr(C)]
pub struct pg_cntl_registers {
    pub LONO_STATE: u32, pub DC_IP_REQUEST_CNTL: u32,
    pub DOMAIN0_PG_CONFIG: u32, pub DOMAIN1_PG_CONFIG: u32, pub DOMAIN2_PG_CONFIG: u32, pub DOMAIN3_PG_CONFIG: u32,
    pub DOMAIN16_PG_CONFIG: u32, pub DOMAIN17_PG_CONFIG: u32, pub DOMAIN18_PG_CONFIG: u32, pub DOMAIN19_PG_CONFIG: u32,
    pub DOMAIN22_PG_CONFIG: u32, pub DOMAIN23_PG_CONFIG: u32, pub DOMAIN24_PG_CONFIG: u32, pub DOMAIN25_PG_CONFIG: u32, pub DOMAIN26_PG_CONFIG: u32,
    pub DOMAIN0_PG_STATUS: u32, pub DOMAIN1_PG_STATUS: u32, pub DOMAIN2_PG_STATUS: u32, pub DOMAIN3_PG_STATUS: u32,
    pub DOMAIN16_PG_STATUS: u32, pub DOMAIN17_PG_STATUS: u32, pub DOMAIN18_PG_STATUS: u32, pub DOMAIN19_PG_STATUS: u32,
    pub DOMAIN22_PG_STATUS: u32, pub DOMAIN23_PG_STATUS: u32, pub DOMAIN24_PG_STATUS: u32, pub DOMAIN25_PG_STATUS: u32, pub DOMAIN26_PG_STATUS: u32,
    pub DMU_CLK_CNTL: u32, pub AZ_CLOCK_CNTL: u32,
}

#[repr(C)]
pub struct dcn_pg_cntl {
    pub base: pg_cntl,
    pub regs: *const pg_cntl_registers,
    pub pg_cntl_shift: *const pg_cntl_shift,
    pub pg_cntl_mask: *const pg_cntl_mask,
}

extern "C" {
    pub fn pg_cntl42_dsc_pg_control(pg_cntl: *mut pg_cntl, dsc_inst: ::core::ffi::c_uint, power_on: bool);
    pub fn pg_cntl42_hubp_dpp_pg_control(pg_cntl: *mut pg_cntl, hubp_dpp_inst: ::core::ffi::c_uint, power_on: bool);
    pub fn pg_cntl42_hpo_pg_control(pg_cntl: *mut pg_cntl, power_on: bool);
    pub fn pg_cntl42_io_clk_pg_control(pg_cntl: *mut pg_cntl, power_on: bool);
    pub fn pg_cntl42_plane_otg_pg_control(pg_cntl: *mut pg_cntl, power_on: bool);
    pub fn pg_cntl42_mpcc_pg_control(pg_cntl: *mut pg_cntl, mpcc_inst: ::core::ffi::c_uint, power_on: bool);
    pub fn pg_cntl42_opp_pg_control(pg_cntl: *mut pg_cntl, opp_inst: ::core::ffi::c_uint, power_on: bool);
    pub fn pg_cntl42_optc_pg_control(pg_cntl: *mut pg_cntl, optc_inst: ::core::ffi::c_uint, power_on: bool);
    pub fn pg_cntl42_mem_pg_control(pg_cntl: *mut pg_cntl, power_on: bool);
    pub fn pg_cntl42_dio_pg_control(pg_cntl: *mut pg_cntl, power_on: bool);
    pub fn dcn42_pg_cntl_destroy(pg_cntl: *mut *mut pg_cntl);
    pub fn pg_cntl42_init_pg_status(pg_cntl: *mut pg_cntl);
    pub fn pg_cntl42_create(ctx: *mut dc_context, regs: *const pg_cntl_registers, pg_cntl_shift: *const pg_cntl_shift, pg_cntl_mask: *const pg_cntl_mask) -> *mut pg_cntl;
    pub fn dcn_pg_cntl_destroy(pg_cntl: *mut *mut pg_cntl);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
