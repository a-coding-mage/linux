/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * TI AM33XX EMIF Routines
 *
 * Copyright (C) 2016-2017 Texas Instruments Inc.
 *	Dave Gerlach
 */

// Translated from the non-assembly portion of <linux/ti-emif-sram.h>.

#[repr(C)]
pub struct emif_regs_amx3 {
    pub emif_sdcfg_val: u32,
    pub emif_timing1_val: u32,
    pub emif_timing2_val: u32,
    pub emif_timing3_val: u32,
    pub emif_ref_ctrl_val: u32,
    pub emif_zqcfg_val: u32,
    pub emif_pmcr_val: u32,
    pub emif_pmcr_shdw_val: u32,
    pub emif_rd_wr_level_ramp_ctrl: u32,
    pub emif_rd_wr_exec_thresh: u32,
    pub emif_cos_config: u32,
    pub emif_priority_to_cos_mapping: u32,
    pub emif_connect_id_serv_1_map: u32,
    pub emif_connect_id_serv_2_map: u32,
    pub emif_ocp_config_val: u32,
    pub emif_lpddr2_nvm_tim: u32,
    pub emif_lpddr2_nvm_tim_shdw: u32,
    pub emif_dll_calib_ctrl_val: u32,
    pub emif_dll_calib_ctrl_val_shdw: u32,
    pub emif_ddr_phy_ctlr_1: u32,
    pub emif_ext_phy_ctrl_vals: [u32; 120],
}

#[repr(C, packed(8))]
pub struct ti_emif_pm_data {
    pub ti_emif_base_addr_virt: *mut core::ffi::c_void,
    pub ti_emif_base_addr_phys: usize, // phys_addr_t
    pub ti_emif_sram_config: usize, // unsigned long
    pub regs_virt: *mut emif_regs_amx3,
    pub regs_phys: usize, // phys_addr_t
}

#[repr(C, packed(8))]
pub struct ti_emif_pm_functions {
    pub save_context: u32,
    pub restore_context: u32,
    pub run_hw_leveling: u32,
    pub enter_sr: u32,
    pub exit_sr: u32,
    pub abort_sr: u32,
}

// The C DEFINE/BLANK macros emit assembler offset definitions.  These Rust
// constants preserve the same offsets and sizes for consumers needing them.
pub const EMIF_SDCFG_VAL_OFFSET: usize = core::mem::offset_of!(emif_regs_amx3, emif_sdcfg_val);
pub const EMIF_TIMING1_VAL_OFFSET: usize = core::mem::offset_of!(emif_regs_amx3, emif_timing1_val);
pub const EMIF_TIMING2_VAL_OFFSET: usize = core::mem::offset_of!(emif_regs_amx3, emif_timing2_val);
pub const EMIF_TIMING3_VAL_OFFSET: usize = core::mem::offset_of!(emif_regs_amx3, emif_timing3_val);
pub const EMIF_REF_CTRL_VAL_OFFSET: usize = core::mem::offset_of!(emif_regs_amx3, emif_ref_ctrl_val);
pub const EMIF_ZQCFG_VAL_OFFSET: usize = core::mem::offset_of!(emif_regs_amx3, emif_zqcfg_val);
pub const EMIF_PMCR_VAL_OFFSET: usize = core::mem::offset_of!(emif_regs_amx3, emif_pmcr_val);
pub const EMIF_PMCR_SHDW_VAL_OFFSET: usize = core::mem::offset_of!(emif_regs_amx3, emif_pmcr_shdw_val);
pub const EMIF_RD_WR_LEVEL_RAMP_CTRL_OFFSET: usize = core::mem::offset_of!(emif_regs_amx3, emif_rd_wr_level_ramp_ctrl);
pub const EMIF_RD_WR_EXEC_THRESH_OFFSET: usize = core::mem::offset_of!(emif_regs_amx3, emif_rd_wr_exec_thresh);
pub const EMIF_COS_CONFIG_OFFSET: usize = core::mem::offset_of!(emif_regs_amx3, emif_cos_config);
pub const EMIF_PRIORITY_TO_COS_MAPPING_OFFSET: usize = core::mem::offset_of!(emif_regs_amx3, emif_priority_to_cos_mapping);
pub const EMIF_CONNECT_ID_SERV_1_MAP_OFFSET: usize = core::mem::offset_of!(emif_regs_amx3, emif_connect_id_serv_1_map);
pub const EMIF_CONNECT_ID_SERV_2_MAP_OFFSET: usize = core::mem::offset_of!(emif_regs_amx3, emif_connect_id_serv_2_map);
pub const EMIF_OCP_CONFIG_VAL_OFFSET: usize = core::mem::offset_of!(emif_regs_amx3, emif_ocp_config_val);
pub const EMIF_LPDDR2_NVM_TIM_OFFSET: usize = core::mem::offset_of!(emif_regs_amx3, emif_lpddr2_nvm_tim);
pub const EMIF_LPDDR2_NVM_TIM_SHDW_OFFSET: usize = core::mem::offset_of!(emif_regs_amx3, emif_lpddr2_nvm_tim_shdw);
pub const EMIF_DLL_CALIB_CTRL_VAL_OFFSET: usize = core::mem::offset_of!(emif_regs_amx3, emif_dll_calib_ctrl_val);
pub const EMIF_DLL_CALIB_CTRL_VAL_SHDW_OFFSET: usize = core::mem::offset_of!(emif_regs_amx3, emif_dll_calib_ctrl_val_shdw);
pub const EMIF_DDR_PHY_CTLR_1_OFFSET: usize = core::mem::offset_of!(emif_regs_amx3, emif_ddr_phy_ctlr_1);
pub const EMIF_EXT_PHY_CTRL_VALS_OFFSET: usize = core::mem::offset_of!(emif_regs_amx3, emif_ext_phy_ctrl_vals);
pub const EMIF_REGS_AMX3_SIZE: usize = core::mem::size_of::<emif_regs_amx3>();

pub const EMIF_PM_BASE_ADDR_VIRT_OFFSET: usize = core::mem::offset_of!(ti_emif_pm_data, ti_emif_base_addr_virt);
pub const EMIF_PM_BASE_ADDR_PHYS_OFFSET: usize = core::mem::offset_of!(ti_emif_pm_data, ti_emif_base_addr_phys);
pub const EMIF_PM_CONFIG_OFFSET: usize = core::mem::offset_of!(ti_emif_pm_data, ti_emif_sram_config);
pub const EMIF_PM_REGS_VIRT_OFFSET: usize = core::mem::offset_of!(ti_emif_pm_data, regs_virt);
pub const EMIF_PM_REGS_PHYS_OFFSET: usize = core::mem::offset_of!(ti_emif_pm_data, regs_phys);
pub const EMIF_PM_DATA_SIZE: usize = core::mem::size_of::<ti_emif_pm_data>();

pub const EMIF_PM_SAVE_CONTEXT_OFFSET: usize = core::mem::offset_of!(ti_emif_pm_functions, save_context);
pub const EMIF_PM_RESTORE_CONTEXT_OFFSET: usize = core::mem::offset_of!(ti_emif_pm_functions, restore_context);
pub const EMIF_PM_RUN_HW_LEVELING: usize = core::mem::offset_of!(ti_emif_pm_functions, run_hw_leveling);
pub const EMIF_PM_ENTER_SR_OFFSET: usize = core::mem::offset_of!(ti_emif_pm_functions, enter_sr);
pub const EMIF_PM_EXIT_SR_OFFSET: usize = core::mem::offset_of!(ti_emif_pm_functions, exit_sr);
pub const EMIF_PM_ABORT_SR_OFFSET: usize = core::mem::offset_of!(ti_emif_pm_functions, abort_sr);
pub const EMIF_PM_FUNCTIONS_SIZE: usize = core::mem::size_of::<ti_emif_pm_functions>();

// External declarations supplied by other translation units.
pub struct gen_pool;

unsafe extern "C" {
    pub fn ti_emif_copy_pm_function_table(sram_pool: *mut gen_pool, dst: *mut core::ffi::c_void) -> i32;
    pub fn ti_emif_get_mem_type() -> i32;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
