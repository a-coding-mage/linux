/*
 * Copyright 2012-15 Advanced Micro Devices, Inc.
 *
 * Permission is hereby granted, free of charge, to any person obtaining a
 * copy of this software and associated documentation files (the "Software"),
 * to deal in the Software without restriction, including without limitation
 * the rights to use, copy, modify, merge, publish, distribute, sublicense,
 *  and/or sell copies of the Software, and to permit persons to whom the
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

// Dependency supplied by the surrounding translation unit: panel_cntl.h

/* set register offset with instance */
// DCE_PANEL_CNTL_SR, DCE_PANEL_CNTL_REG_LIST, DCN_PANEL_CNTL_SR, and
// DCN_PANEL_CNTL_REG_LIST use C token-pasting and designated initializers;
// their register symbols are supplied by the target hardware definitions.
// DCE_PANEL_CNTL_SF and DCE_PANEL_CNTL_MASK_SH_LIST likewise use token-pasting
// for register-field symbols and are preserved here as source-level intent.

#[repr(C)]
pub struct dce_panel_cntl_shift {
    pub LVTMA_BLON: u8,
    pub LVTMA_BLON_OVRD: u8,
    pub LVTMA_DIGON: u8,
    pub LVTMA_DIGON_OVRD: u8,
    pub LVTMA_PWRSEQ_TARGET_STATE: u8,
    pub LVTMA_PWRSEQ_TARGET_STATE_R: u8,
    pub BL_PWM_REF_DIV: u8,
    pub BL_PWM_EN: u8,
    pub BL_ACTIVE_INT_FRAC_CNT: u8,
    pub BL_PWM_FRACTIONAL_EN: u8,
    pub BL_PWM_PERIOD: u8,
    pub BL_PWM_PERIOD_BITCNT: u8,
    pub BL_PWM_GRP1_IGNORE_MASTER_LOCK_EN: u8,
    pub BL_PWM_GRP1_REG_LOCK: u8,
    pub BL_PWM_GRP1_REG_UPDATE_PENDING: u8,
}

#[repr(C)]
pub struct dce_panel_cntl_mask {
    pub LVTMA_BLON: u32,
    pub LVTMA_BLON_OVRD: u32,
    pub LVTMA_DIGON: u32,
    pub LVTMA_DIGON_OVRD: u32,
    pub LVTMA_PWRSEQ_TARGET_STATE: u32,
    pub LVTMA_PWRSEQ_TARGET_STATE_R: u32,
    pub BL_PWM_REF_DIV: u32,
    pub BL_PWM_EN: u32,
    pub BL_ACTIVE_INT_FRAC_CNT: u32,
    pub BL_PWM_FRACTIONAL_EN: u32,
    pub BL_PWM_PERIOD: u32,
    pub BL_PWM_PERIOD_BITCNT: u32,
    pub BL_PWM_GRP1_IGNORE_MASTER_LOCK_EN: u32,
    pub BL_PWM_GRP1_REG_LOCK: u32,
    pub BL_PWM_GRP1_REG_UPDATE_PENDING: u32,
}

#[repr(C)]
pub struct dce_panel_cntl_registers {
    pub PWRSEQ_CNTL: u32,
    pub PWRSEQ_STATE: u32,
    pub BL_PWM_CNTL: u32,
    pub BL_PWM_CNTL2: u32,
    pub BL_PWM_PERIOD_CNTL: u32,
    pub BL_PWM_GRP1_REG_LOCK: u32,
    pub PWRSEQ_REF_DIV: u32,
    pub BIOS_SCRATCH_2: u32,
}

#[repr(C)]
pub struct dce_panel_cntl {
    pub base: panel_cntl,
    pub regs: *const dce_panel_cntl_registers,
    pub shift: *const dce_panel_cntl_shift,
    pub mask: *const dce_panel_cntl_mask,
}

extern "C" {
    pub fn dce_panel_cntl_construct(
        panel_cntl: *mut dce_panel_cntl,
        init_data: *const panel_cntl_init_data,
        regs: *const dce_panel_cntl_registers,
        shift: *const dce_panel_cntl_shift,
        mask: *const dce_panel_cntl_mask,
    );
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
