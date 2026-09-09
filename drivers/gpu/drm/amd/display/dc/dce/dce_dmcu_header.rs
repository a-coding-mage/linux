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

// Dependency supplied by dmcu.h.

macro_rules! DMCU_COMMON_REG_LIST_DCE_BASE {
    () => { SR!(DMCU_CTRL), SR!(DMCU_STATUS), SR!(DMCU_RAM_ACCESS_CTRL),
        SR!(DMCU_IRAM_WR_CTRL), SR!(DMCU_IRAM_WR_DATA), SR!(MASTER_COMM_DATA_REG1),
        SR!(MASTER_COMM_DATA_REG2), SR!(MASTER_COMM_DATA_REG3), SR!(MASTER_COMM_CMD_REG),
        SR!(MASTER_COMM_CNTL_REG), SR!(SLAVE_COMM_DATA_REG1), SR!(SLAVE_COMM_DATA_REG2),
        SR!(SLAVE_COMM_DATA_REG3), SR!(SLAVE_COMM_CMD_REG), SR!(DMCU_IRAM_RD_CTRL),
        SR!(DMCU_IRAM_RD_DATA), SR!(DMCU_INTERRUPT_TO_UC_EN_MASK),
        SR!(SMU_INTERRUPT_CONTROL), SR!(DC_DMCU_SCRATCH) };
}

macro_rules! DMCU_DCE60_REG_LIST { () => { DMCU_COMMON_REG_LIST_DCE_BASE!() }; }
macro_rules! DMCU_DCE80_REG_LIST { () => { DMCU_COMMON_REG_LIST_DCE_BASE!() }; }
macro_rules! DMCU_DCE110_COMMON_REG_LIST { () => { DMCU_COMMON_REG_LIST_DCE_BASE!(), SR!(DCI_MEM_PWR_STATUS) }; }
macro_rules! DMCU_DCN10_REG_LIST { () => { DMCU_COMMON_REG_LIST_DCE_BASE!(), SR!(DMU_MEM_PWR_CNTL) }; }
macro_rules! DMCU_DCN20_REG_LIST { () => { DMCU_DCN10_REG_LIST!(), SR!(DMCUB_SCRATCH15) }; }

// C token-pasting macro; callers may provide the corresponding field directly.
macro_rules! DMCU_SF { ($reg:ident, $field:ident, $post_fix:ident) => { .$field }; }
macro_rules! DMCU_COMMON_MASK_SH_LIST_DCE_COMMON_BASE { ($mask_sh:ident) => {
    DMCU_SF!(DMCU_CTRL, DMCU_ENABLE, $mask_sh), DMCU_SF!(DMCU_STATUS, UC_IN_STOP_MODE, $mask_sh),
    DMCU_SF!(DMCU_STATUS, UC_IN_RESET, $mask_sh), DMCU_SF!(DMCU_RAM_ACCESS_CTRL, IRAM_HOST_ACCESS_EN, $mask_sh),
    DMCU_SF!(DMCU_RAM_ACCESS_CTRL, IRAM_WR_ADDR_AUTO_INC, $mask_sh), DMCU_SF!(DMCU_RAM_ACCESS_CTRL, IRAM_RD_ADDR_AUTO_INC, $mask_sh),
    DMCU_SF!(MASTER_COMM_CMD_REG, MASTER_COMM_CMD_REG_BYTE0, $mask_sh), DMCU_SF!(MASTER_COMM_CNTL_REG, MASTER_COMM_INTERRUPT, $mask_sh),
    DMCU_SF!(SLAVE_COMM_CNTL_REG, SLAVE_COMM_INTERRUPT, $mask_sh), DMCU_SF!(DMCU_INTERRUPT_TO_UC_EN_MASK, STATIC_SCREEN1_INT_TO_UC_EN, $mask_sh),
    DMCU_SF!(DMCU_INTERRUPT_TO_UC_EN_MASK, STATIC_SCREEN2_INT_TO_UC_EN, $mask_sh), DMCU_SF!(DMCU_INTERRUPT_TO_UC_EN_MASK, STATIC_SCREEN3_INT_TO_UC_EN, $mask_sh),
    DMCU_SF!(DMCU_INTERRUPT_TO_UC_EN_MASK, STATIC_SCREEN4_INT_TO_UC_EN, $mask_sh), DMCU_SF!(SMU_INTERRUPT_CONTROL, DC_SMU_INT_ENABLE, $mask_sh)
}; }
macro_rules! DMCU_MASK_SH_LIST_DCE60 { ($mask_sh:ident) => { DMCU_COMMON_MASK_SH_LIST_DCE_COMMON_BASE!($mask_sh) }; }
macro_rules! DMCU_MASK_SH_LIST_DCE80 { ($mask_sh:ident) => { DMCU_COMMON_MASK_SH_LIST_DCE_COMMON_BASE!($mask_sh) }; }
macro_rules! DMCU_MASK_SH_LIST_DCE110 { ($mask_sh:ident) => { DMCU_COMMON_MASK_SH_LIST_DCE_COMMON_BASE!($mask_sh), DMCU_SF!(DCI_MEM_PWR_STATUS, DMCU_IRAM_MEM_PWR_STATE, $mask_sh) }; }
macro_rules! DMCU_MASK_SH_LIST_DCN10 { ($mask_sh:ident) => { DMCU_COMMON_MASK_SH_LIST_DCE_COMMON_BASE!($mask_sh), DMCU_SF!(DMU_MEM_PWR_CNTL, DMCU_IRAM_MEM_PWR_STATE, $mask_sh) }; }

macro_rules! DMCU_REG_FIELD_LIST { ($type:ty) => {
    DMCU_IRAM_MEM_PWR_STATE: $type, IRAM_HOST_ACCESS_EN: $type, IRAM_WR_ADDR_AUTO_INC: $type,
    IRAM_RD_ADDR_AUTO_INC: $type, DMCU_ENABLE: $type, UC_IN_STOP_MODE: $type, UC_IN_RESET: $type,
    MASTER_COMM_CMD_REG_BYTE0: $type, MASTER_COMM_INTERRUPT: $type, SLAVE_COMM_INTERRUPT: $type,
    DPHY_RX_FAST_TRAINING_CAPABLE: $type, DPHY_LOAD_BS_COUNT: $type, STATIC_SCREEN1_INT_TO_UC_EN: $type,
    STATIC_SCREEN2_INT_TO_UC_EN: $type, STATIC_SCREEN3_INT_TO_UC_EN: $type, STATIC_SCREEN4_INT_TO_UC_EN: $type,
    DP_SEC_GSP0_LINE_NUM: $type, DP_SEC_GSP0_PRIORITY: $type, DC_SMU_INT_ENABLE: $type
}; }

#[repr(C)]
pub struct dce_dmcu_shift { pub fields: [u8; 19] }
#[repr(C)]
pub struct dce_dmcu_mask { pub fields: [u32; 19] }

#[repr(C)]
pub struct dce_dmcu_registers {
    pub DMCU_CTRL: u32, pub DMCU_STATUS: u32, pub DMCU_RAM_ACCESS_CTRL: u32,
    pub DCI_MEM_PWR_STATUS: u32, pub DMU_MEM_PWR_CNTL: u32, pub DMCU_IRAM_WR_CTRL: u32,
    pub DMCU_IRAM_WR_DATA: u32, pub MASTER_COMM_DATA_REG1: u32, pub MASTER_COMM_DATA_REG2: u32,
    pub MASTER_COMM_DATA_REG3: u32, pub MASTER_COMM_CMD_REG: u32, pub MASTER_COMM_CNTL_REG: u32,
    pub SLAVE_COMM_DATA_REG1: u32, pub SLAVE_COMM_DATA_REG2: u32, pub SLAVE_COMM_DATA_REG3: u32,
    pub SLAVE_COMM_CMD_REG: u32, pub SLAVE_COMM_CNTL_REG: u32, pub DMCU_IRAM_RD_CTRL: u32,
    pub DMCU_IRAM_RD_DATA: u32, pub DMCU_INTERRUPT_TO_UC_EN_MASK: u32, pub SMU_INTERRUPT_CONTROL: u32,
    pub DC_DMCU_SCRATCH: u32, pub DMCUB_SCRATCH15: u32,
}

#[repr(C)]
pub struct dce_dmcu {
    pub base: dmcu,
    pub regs: *const dce_dmcu_registers,
    pub dmcu_shift: *const dce_dmcu_shift,
    pub dmcu_mask: *const dce_dmcu_mask,
}

// C bitfields are preserved as their containing 32-bit word; ranges are the source ranges.
#[repr(C)] pub union dce_dmcu_psr_config_data_reg1 { pub bits: u32, pub u32All: u32 }
#[repr(C)] pub union dce_dmcu_psr_config_data_reg2 { pub bits: u32, pub u32All: u32 }
#[repr(C)] pub union dce_dmcu_psr_config_data_reg3 { pub bits: u32, pub u32All: u32 }
#[repr(C)] pub union dce_dmcu_psr_config_data_wait_loop_reg1 { pub bits: u32, pub u32_: u32 }

extern "C" {
    pub fn dce_dmcu_create(ctx: *mut dc_context, regs: *const dce_dmcu_registers, dmcu_shift: *const dce_dmcu_shift, dmcu_mask: *const dce_dmcu_mask) -> *mut dmcu;
    pub fn dcn10_dmcu_create(ctx: *mut dc_context, regs: *const dce_dmcu_registers, dmcu_shift: *const dce_dmcu_shift, dmcu_mask: *const dce_dmcu_mask) -> *mut dmcu;
    pub fn dcn20_dmcu_create(ctx: *mut dc_context, regs: *const dce_dmcu_registers, dmcu_shift: *const dce_dmcu_shift, dmcu_mask: *const dce_dmcu_mask) -> *mut dmcu;
    pub fn dcn21_dmcu_create(ctx: *mut dc_context, regs: *const dce_dmcu_registers, dmcu_shift: *const dce_dmcu_shift, dmcu_mask: *const dce_dmcu_mask) -> *mut dmcu;
    pub fn dce_dmcu_destroy(dmcu: *mut *mut dmcu);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
