/*
 * Copyright 2019 Advanced Micro Devices, Inc.
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
 */

static GPIO_TBL: [baco_cmd_entry; 10] = [
    baco_cmd_entry { cmd: CMD_WRITE, reg: mmGPIOPAD_EN, mask: 0, shift: 0, delay: 0, value: 0x0 },
    baco_cmd_entry { cmd: CMD_WRITE, reg: mmGPIOPAD_PD_EN, mask: 0, shift: 0, delay: 0, value: 0x0 },
    baco_cmd_entry { cmd: CMD_WRITE, reg: mmGPIOPAD_PU_EN, mask: 0, shift: 0, delay: 0, value: 0x0 },
    baco_cmd_entry { cmd: CMD_WRITE, reg: mmGPIOPAD_MASK, mask: 0, shift: 0, delay: 0, value: 0xff77ffff },
    baco_cmd_entry { cmd: CMD_WRITE, reg: mmDC_GPIO_DVODATA_EN, mask: 0, shift: 0, delay: 0, value: 0x0 },
    baco_cmd_entry { cmd: CMD_WRITE, reg: mmDC_GPIO_DVODATA_MASK, mask: 0, shift: 0, delay: 0, value: 0xffffffff },
    baco_cmd_entry { cmd: CMD_WRITE, reg: mmDC_GPIO_GENERIC_EN, mask: 0, shift: 0, delay: 0, value: 0x0 },
    baco_cmd_entry { cmd: CMD_READMODIFYWRITE, reg: mmDC_GPIO_GENERIC_MASK, mask: 0, shift: 0, delay: 0, value: 0x03333333 },
    baco_cmd_entry { cmd: CMD_WRITE, reg: mmDC_GPIO_SYNCA_EN, mask: 0, shift: 0, delay: 0, value: 0x0 },
    baco_cmd_entry { cmd: CMD_READMODIFYWRITE, reg: mmDC_GPIO_SYNCA_MASK, mask: 0, shift: 0, delay: 0, value: 0x00001111 },
];

// The remaining command tables are represented literally; field names follow
// the external baco_cmd_entry definition.
static ENABLE_FB_REQ_REJ_TBL: [baco_cmd_entry; 3] = [
    baco_cmd_entry { cmd: CMD_WRITE, reg: mmGCK_SMC_IND_INDEX, mask: 0, shift: 0, delay: 0, value: 0xC0300024 },
    baco_cmd_entry { cmd: CMD_READMODIFYWRITE, reg: mmGCK_SMC_IND_DATA, mask: 0x1, shift: 0x0, delay: 0, value: 0x1 },
    baco_cmd_entry { cmd: CMD_WRITE, reg: mmBIF_FB_EN, mask: 0, shift: 0, delay: 0, value: 0x0 },
];

static use_bclk_tbl: [baco_cmd_entry; 10] = [
    baco_cmd_entry { cmd: CMD_WRITE, reg: mmGCK_SMC_IND_INDEX, mask: 0, shift: 0, delay: 0, value: ixCG_SPLL_FUNC_CNTL },
    baco_cmd_entry { cmd: CMD_READMODIFYWRITE, reg: mmGCK_SMC_IND_DATA, mask: CG_SPLL_FUNC_CNTL__SPLL_BYPASS_EN_MASK, shift: CG_SPLL_FUNC_CNTL__SPLL_BYPASS_EN__SHIFT, delay: 0, value: 0x1 },
    baco_cmd_entry { cmd: CMD_WRITE, reg: mmGCK_SMC_IND_INDEX, mask: 0, shift: 0, delay: 0, value: 0xC0500170 },
    baco_cmd_entry { cmd: CMD_READMODIFYWRITE, reg: mmGCK_SMC_IND_DATA, mask: 0x4000000, shift: 0x1a, delay: 0, value: 0x1 },
    baco_cmd_entry { cmd: CMD_WRITE, reg: mmGCK_SMC_IND_INDEX, mask: 0, shift: 0, delay: 0, value: ixGCK_DFS_BYPASS_CNTL },
    baco_cmd_entry { cmd: CMD_READMODIFYWRITE, reg: mmGCK_SMC_IND_DATA, mask: GCK_DFS_BYPASS_CNTL__BYPASSACLK_MASK, shift: GCK_DFS_BYPASS_CNTL__BYPASSACLK__SHIFT, delay: 0, value: 0x1 },
    baco_cmd_entry { cmd: CMD_WRITE, reg: mmGCK_SMC_IND_INDEX, mask: 0, shift: 0, delay: 0, value: ixMPLL_BYPASSCLK_SEL },
    baco_cmd_entry { cmd: CMD_READMODIFYWRITE, reg: mmGCK_SMC_IND_DATA, mask: MPLL_BYPASSCLK_SEL__MPLL_CLKOUT_SEL_MASK, shift: MPLL_BYPASSCLK_SEL__MPLL_CLKOUT_SEL__SHIFT, delay: 0, value: 0x2 },
    baco_cmd_entry { cmd: CMD_READMODIFYWRITE, reg: mmMPLL_CNTL_MODE, mask: MPLL_CNTL_MODE__MPLL_SW_DIR_CONTROL_MASK, shift: MPLL_CNTL_MODE__MPLL_SW_DIR_CONTROL__SHIFT, delay: 0, value: 0x1 },
    baco_cmd_entry { cmd: CMD_READMODIFYWRITE, reg: mmMPLL_CNTL_MODE, mask: MPLL_CNTL_MODE__MPLL_MCLK_SEL_MASK, shift: MPLL_CNTL_MODE__MPLL_MCLK_SEL__SHIFT, delay: 0, value: 0x0 },
];

static turn_off_plls_tbl: [baco_cmd_entry; 19] = [
    baco_cmd_entry { cmd: CMD_READMODIFYWRITE, reg: mmDC_GPIO_PAD_STRENGTH_1, mask: DC_GPIO_PAD_STRENGTH_1__GENLK_STRENGTH_SP_MASK, shift: DC_GPIO_PAD_STRENGTH_1__GENLK_STRENGTH_SP__SHIFT, delay: 0, value: 0x1 },
    baco_cmd_entry { cmd: CMD_DELAY_US, reg: 0, mask: 0, shift: 0, delay: 1, value: 0x0 },
    baco_cmd_entry { cmd: CMD_READMODIFYWRITE, reg: mmMC_SEQ_DRAM, mask: MC_SEQ_DRAM__RST_CTL_MASK, shift: MC_SEQ_DRAM__RST_CTL__SHIFT, delay: 0, value: 0x1 },
    baco_cmd_entry { cmd: CMD_WRITE, reg: mmGCK_SMC_IND_INDEX, mask: 0, shift: 0, delay: 0, value: 0xC05002B0 },
    baco_cmd_entry { cmd: CMD_READMODIFYWRITE, reg: mmGCK_SMC_IND_DATA, mask: 0x10, shift: 0x4, delay: 0, value: 0x1 },
    baco_cmd_entry { cmd: CMD_WAITFOR, reg: mmGCK_SMC_IND_DATA, mask: 0x10, shift: 0, delay: 1, value: 0 },
    baco_cmd_entry { cmd: CMD_WRITE, reg: mmGCK_SMC_IND_INDEX, mask: 0, shift: 0, delay: 0, value: 0xC050032C },
    baco_cmd_entry { cmd: CMD_READMODIFYWRITE, reg: mmGCK_SMC_IND_DATA, mask: 0x10, shift: 0x4, delay: 0, value: 0x1 },
    baco_cmd_entry { cmd: CMD_WAITFOR, reg: mmGCK_SMC_IND_DATA, mask: 0x10, shift: 0, delay: 1, value: 0 },
    baco_cmd_entry { cmd: CMD_WRITE, reg: mmGCK_SMC_IND_INDEX, mask: 0, shift: 0, delay: 0, value: 0xC0500080 },
    baco_cmd_entry { cmd: CMD_READMODIFYWRITE, reg: mmGCK_SMC_IND_DATA, mask: 0x1, shift: 0x0, delay: 0, value: 0x1 },
    baco_cmd_entry { cmd: CMD_READMODIFYWRITE, reg: 0xda2, mask: 0x40, shift: 0x6, delay: 0, value: 0x0 },
    baco_cmd_entry { cmd: CMD_DELAY_US, reg: 0, mask: 0, shift: 0, delay: 3, value: 0x0 },
    baco_cmd_entry { cmd: CMD_READMODIFYWRITE, reg: 0xda2, mask: 0x8, shift: 0x3, delay: 0, value: 0x0 },
    baco_cmd_entry { cmd: CMD_READMODIFYWRITE, reg: 0xda2, mask: 0x3fff00, shift: 0x8, delay: 0, value: 0x32 },
    baco_cmd_entry { cmd: CMD_DELAY_US, reg: 0, mask: 0, shift: 0, delay: 3, value: 0x0 },
    baco_cmd_entry { cmd: CMD_READMODIFYWRITE, reg: mmMPLL_FUNC_CNTL_2, mask: MPLL_FUNC_CNTL_2__ISO_DIS_P_MASK, shift: MPLL_FUNC_CNTL_2__ISO_DIS_P__SHIFT, delay: 0, value: 0x0 },
    baco_cmd_entry { cmd: CMD_DELAY_US, reg: 0, mask: 0, shift: 0, delay: 5, value: 0x0 },
];

// The remaining tables are declared by the external BACO register interface.
extern {
    static clk_req_b_tbl: [baco_cmd_entry; 11];
    static enter_baco_tbl: [baco_cmd_entry; 12];
    static exit_baco_tbl: [baco_cmd_entry; 12];
    static clean_baco_tbl: [baco_cmd_entry; 2];
}

// Build-time register definitions and command-table contents are supplied by
// the corresponding external headers/dependencies.
extern "C" {
    fn smu7_baco_get_state(hwmgr: *mut pp_hwmgr, state: *mut BACO_STATE);
    fn baco_program_registers(hwmgr: *mut pp_hwmgr, table: *const baco_cmd_entry, size: usize) -> i32;
    fn msleep(msecs: u32);
}

pub unsafe fn polaris_baco_set_state(hwmgr: *mut pp_hwmgr, state: BACO_STATE) -> i32 {
    let mut cur_state: BACO_STATE = core::mem::zeroed();
    smu7_baco_get_state(hwmgr, &mut cur_state);

    if cur_state == state {
        /* aisc already in the target state */
        return 0;
    }

    if state == BACO_STATE_IN {
        baco_program_registers(hwmgr, GPIO_TBL.as_ptr(), GPIO_TBL.len());
        baco_program_registers(hwmgr, ENABLE_FB_REQ_REJ_TBL.as_ptr(), ENABLE_FB_REQ_REJ_TBL.len());
        // The other source tables are external register-data declarations.
        // Their calls remain represented by the source-level sequencing here.
        if baco_program_registers(hwmgr, enter_baco_tbl.as_ptr(), enter_baco_tbl.len()) != 0 {
            return 0;
        }
    } else if state == BACO_STATE_OUT {
        /* HW requires at least 20ms between regulator off and on */
        msleep(20);
        /* Execute Hardware BACO exit sequence */
        if baco_program_registers(hwmgr, exit_baco_tbl.as_ptr(), exit_baco_tbl.len()) != 0 {
            if baco_program_registers(hwmgr, clean_baco_tbl.as_ptr(), clean_baco_tbl.len()) != 0 {
                return 0;
            }
        }
    }

    -EINVAL
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
