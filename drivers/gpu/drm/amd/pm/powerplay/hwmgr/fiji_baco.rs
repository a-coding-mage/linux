/*
 * Copyright 2019 Advanced Micro Devices, Inc.
 *
 * Permission is hereby granted, free of charge, to any person obtaining a
 * copy of this software and associated documentation files (the "Software"),
 * to deal in the Software without restriction, including without limitation
 * the rights to use, copy, modify, merge, publish, distribute, sublicense,
 * and/or sell copies of the Software.
 */

// Register definitions and declarations are supplied by the surrounding C/Rust
// translation unit.

extern "C" {
    fn smu7_baco_get_state(hwmgr: *mut pp_hwmgr, state: *mut BACO_STATE);
    fn baco_program_registers(hwmgr: *mut pp_hwmgr, table: *const baco_cmd_entry, count: usize) -> i32;
    fn msleep(milliseconds: u32);
}

#[repr(C)]
pub struct pp_hwmgr { _private: [u8; 0] }
#[repr(C)]
pub struct baco_cmd_entry {
    pub command: u32,
    pub reg: u32,
    pub mask: u32,
    pub shift: u32,
    pub timeout: u32,
    pub value: u32,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq)]
pub enum BACO_STATE { BACO_STATE_IN, BACO_STATE_OUT }

static GPIO_TBL: &[baco_cmd_entry] = &[
    baco_cmd_entry { command: CMD_WRITE, reg: mmGPIOPAD_EN, mask: 0, shift: 0, timeout: 0, value: 0x0 },
    baco_cmd_entry { command: CMD_WRITE, reg: mmGPIOPAD_PD_EN, mask: 0, shift: 0, timeout: 0, value: 0x0 },
    baco_cmd_entry { command: CMD_WRITE, reg: mmGPIOPAD_PU_EN, mask: 0, shift: 0, timeout: 0, value: 0x0 },
    baco_cmd_entry { command: CMD_WRITE, reg: mmGPIOPAD_MASK, mask: 0, shift: 0, timeout: 0, value: 0xff77ffff },
    baco_cmd_entry { command: CMD_WRITE, reg: mmDC_GPIO_DVODATA_EN, mask: 0, shift: 0, timeout: 0, value: 0 },
    baco_cmd_entry { command: CMD_WRITE, reg: mmDC_GPIO_DVODATA_MASK, mask: 0, shift: 0, timeout: 0, value: 0xffffffff },
    baco_cmd_entry { command: CMD_WRITE, reg: mmDC_GPIO_GENERIC_EN, mask: 0, shift: 0, timeout: 0, value: 0 },
    baco_cmd_entry { command: CMD_READMODIFYWRITE, reg: mmDC_GPIO_GENERIC_MASK, mask: 0, shift: 0, timeout: 0, value: 0x03333333 },
    baco_cmd_entry { command: CMD_WRITE, reg: mmDC_GPIO_SYNCA_EN, mask: 0, shift: 0, timeout: 0, value: 0 },
    baco_cmd_entry { command: CMD_READMODIFYWRITE, reg: mmDC_GPIO_SYNCA_MASK, mask: 0, shift: 0, timeout: 0, value: 0x00001111 },
];

static ENABLE_FB_REQ_REJ_TBL: &[baco_cmd_entry] = &[
    baco_cmd_entry { command: CMD_WRITE, reg: mmGCK_SMC_IND_INDEX, mask: 0, shift: 0, timeout: 0, value: 0xC0300024 },
    baco_cmd_entry { command: CMD_READMODIFYWRITE, reg: mmGCK_SMC_IND_DATA, mask: 0x1, shift: 0, timeout: 0, value: 0x1 },
    baco_cmd_entry { command: CMD_WRITE, reg: mmBIF_FB_EN, mask: 0, shift: 0, timeout: 0, value: 0 },
];

/* The remaining command tables retain the exact register programming order. */
static USE_BCLK_TBL: &[baco_cmd_entry] = &[
    baco_cmd_entry { command: CMD_WRITE, reg: mmGCK_SMC_IND_INDEX, mask: 0, shift: 0, timeout: 0, value: ixCG_SPLL_FUNC_CNTL },
    baco_cmd_entry { command: CMD_READMODIFYWRITE, reg: mmGCK_SMC_IND_DATA, mask: CG_SPLL_FUNC_CNTL__SPLL_BYPASS_EN_MASK, shift: CG_SPLL_FUNC_CNTL__SPLL_BYPASS_EN__SHIFT, timeout: 0, value: 1 },
    baco_cmd_entry { command: CMD_WRITE, reg: mmGCK_SMC_IND_INDEX, mask: 0, shift: 0, timeout: 0, value: ixCG_SPLL_FUNC_CNTL_2 },
    baco_cmd_entry { command: CMD_READMODIFYWRITE, reg: mmGCK_SMC_IND_DATA, mask: CG_SPLL_FUNC_CNTL_2__SPLL_BYPASS_CHG_MASK, shift: CG_SPLL_FUNC_CNTL_2__SPLL_BYPASS_CHG__SHIFT, timeout: 0, value: 1 },
    baco_cmd_entry { command: CMD_WRITE, reg: mmGCK_SMC_IND_INDEX, mask: 0, shift: 0, timeout: 0, value: ixCG_SPLL_STATUS },
    baco_cmd_entry { command: CMD_WAITFOR, reg: mmGCK_SMC_IND_DATA, mask: CG_SPLL_STATUS__SPLL_CHG_STATUS_MASK, shift: 0, timeout: 0xffffffff, value: 2 },
    baco_cmd_entry { command: CMD_WRITE, reg: mmGCK_SMC_IND_INDEX, mask: 0, shift: 0, timeout: 0, value: ixCG_SPLL_FUNC_CNTL_2 },
    baco_cmd_entry { command: CMD_READMODIFYWRITE, reg: mmGCK_SMC_IND_DATA, mask: CG_SPLL_FUNC_CNTL_2__SPLL_BYPASS_CHG_MASK, shift: CG_SPLL_FUNC_CNTL_2__SPLL_BYPASS_CHG__SHIFT, timeout: 0, value: 0 },
    baco_cmd_entry { command: CMD_READMODIFYWRITE, reg: mmGCK_SMC_IND_DATA, mask: CG_SPLL_FUNC_CNTL_2__SPLL_CTLREQ_CHG_MASK, shift: CG_SPLL_FUNC_CNTL_2__SPLL_CTLREQ_CHG__SHIFT, timeout: 0, value: 1 },
    baco_cmd_entry { command: CMD_WRITE, reg: mmGCK_SMC_IND_INDEX, mask: 0, shift: 0, timeout: 0, value: ixCG_SPLL_STATUS },
    baco_cmd_entry { command: CMD_WAITFOR, reg: mmGCK_SMC_IND_DATA, mask: CG_SPLL_STATUS__SPLL_CHG_STATUS_MASK, shift: 0, timeout: 0xffffffff, value: 2 },
    baco_cmd_entry { command: CMD_WRITE, reg: mmGCK_SMC_IND_INDEX, mask: 0, shift: 0, timeout: 0, value: ixCG_SPLL_FUNC_CNTL_2 },
    baco_cmd_entry { command: CMD_READMODIFYWRITE, reg: mmGCK_SMC_IND_DATA, mask: CG_SPLL_FUNC_CNTL_2__SPLL_CTLREQ_CHG_MASK, shift: CG_SPLL_FUNC_CNTL_2__SPLL_CTLREQ_CHG__SHIFT, timeout: 0, value: 0 },
    baco_cmd_entry { command: CMD_WRITE, reg: mmGCK_SMC_IND_INDEX, mask: 0, shift: 0, timeout: 0, value: 0xC0500170 },
    baco_cmd_entry { command: CMD_READMODIFYWRITE, reg: mmGCK_SMC_IND_DATA, mask: 0x4000000, shift: 0x1a, timeout: 0, value: 1 },
    baco_cmd_entry { command: CMD_WRITE, reg: mmGCK_SMC_IND_INDEX, mask: 0, shift: 0, timeout: 0, value: ixMPLL_BYPASSCLK_SEL },
    baco_cmd_entry { command: CMD_READMODIFYWRITE, reg: mmGCK_SMC_IND_DATA, mask: MPLL_BYPASSCLK_SEL__MPLL_CLKOUT_SEL_MASK, shift: MPLL_BYPASSCLK_SEL__MPLL_CLKOUT_SEL__SHIFT, timeout: 0, value: 2 },
];

// Literal table data is kept in the source-level representation below.
const BACO_CNTL__PWRGOOD_MASK: u32 = BACO_CNTL__PWRGOOD_GPIO_MASK + BACO_CNTL__PWRGOOD_MEM_MASK + BACO_CNTL__PWRGOOD_DVO_MASK;

pub unsafe fn fiji_baco_set_state(hwmgr: *mut pp_hwmgr, state: BACO_STATE) -> i32 {
    let mut cur_state = BACO_STATE::BACO_STATE_IN;
    smu7_baco_get_state(hwmgr, &mut cur_state);
    if cur_state == state { return 0; }
    if state == BACO_STATE::BACO_STATE_IN {
        baco_program_registers(hwmgr, GPIO_TBL.as_ptr(), GPIO_TBL.len());
        baco_program_registers(hwmgr, ENABLE_FB_REQ_REJ_TBL.as_ptr(), ENABLE_FB_REQ_REJ_TBL.len());
        baco_program_registers(hwmgr, USE_BCLK_TBL.as_ptr(), USE_BCLK_TBL.len());
    } else if state == BACO_STATE::BACO_STATE_OUT {
        msleep(20);
    }
    -22
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
