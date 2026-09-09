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
 * FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT.  IN NO EVENT SHALL
 * THE COPYRIGHT HOLDER(S) OR AUTHOR(S) BE LIABLE FOR ANY CLAIM, DAMAGES OR
 * OTHER LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE,
 * ARISING FROM, OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR
 * OTHER DEALINGS IN THE SOFTWARE.
 *
 * Authors: AMD
 */

// Dependencies supplied by panel_cntl.h and dce/dce_panel_cntl.h remain
// external to this translated header.

macro_rules! DCN301_PANEL_CNTL_REG_LIST {
    ($id:expr) => {
        SRIR!(PWRSEQ_CNTL, CNTL, PANEL_PWRSEQ, $id),
        SRIR!(PWRSEQ_STATE, STATE, PANEL_PWRSEQ, $id),
        SRIR!(PWRSEQ_REF_DIV, REF_DIV, PANEL_PWRSEQ, $id),
        SRIR!(BL_PWM_CNTL, CNTL, BL_PWM, $id),
        SRIR!(BL_PWM_CNTL2, CNTL2, BL_PWM, $id),
        SRIR!(BL_PWM_PERIOD_CNTL, PERIOD_CNTL, BL_PWM, $id),
        SRIR!(BL_PWM_GRP1_REG_LOCK, GRP1_REG_LOCK, BL_PWM, $id)
    };
}

macro_rules! DCN301_PANEL_CNTL_SF {
    ($reg_name:ident, $field_name:ident, $post_fix:ident) => {
        $field_name: $reg_name ## __ ## $field_name ## $post_fix
    };
}

macro_rules! DCN301_PANEL_CNTL_MASK_SH_LIST {
    ($mask_sh:ident) => {
        DCN301_PANEL_CNTL_SF!(PANEL_PWRSEQ0_CNTL, PANEL_BLON, $mask_sh),
        DCN301_PANEL_CNTL_SF!(PANEL_PWRSEQ0_CNTL, PANEL_DIGON, $mask_sh),
        DCN301_PANEL_CNTL_SF!(PANEL_PWRSEQ0_CNTL, PANEL_DIGON_OVRD, $mask_sh),
        DCN301_PANEL_CNTL_SF!(PANEL_PWRSEQ0_STATE, PANEL_PWRSEQ_TARGET_STATE_R, $mask_sh),
        DCN301_PANEL_CNTL_SF!(PANEL_PWRSEQ0_REF_DIV, BL_PWM_REF_DIV, $mask_sh),
        DCN301_PANEL_CNTL_SF!(BL_PWM0_PERIOD_CNTL, BL_PWM_PERIOD, $mask_sh),
        DCN301_PANEL_CNTL_SF!(BL_PWM0_PERIOD_CNTL, BL_PWM_PERIOD_BITCNT, $mask_sh),
        DCN301_PANEL_CNTL_SF!(BL_PWM0_CNTL, BL_ACTIVE_INT_FRAC_CNT, $mask_sh),
        DCN301_PANEL_CNTL_SF!(BL_PWM0_CNTL, BL_PWM_FRACTIONAL_EN, $mask_sh),
        DCN301_PANEL_CNTL_SF!(BL_PWM0_CNTL, BL_PWM_EN, $mask_sh),
        DCN301_PANEL_CNTL_SF!(BL_PWM0_GRP1_REG_LOCK, BL_PWM_GRP1_IGNORE_MASTER_LOCK_EN, $mask_sh),
        DCN301_PANEL_CNTL_SF!(BL_PWM0_GRP1_REG_LOCK, BL_PWM_GRP1_REG_LOCK, $mask_sh),
        DCN301_PANEL_CNTL_SF!(BL_PWM0_GRP1_REG_LOCK, BL_PWM_GRP1_REG_UPDATE_PENDING, $mask_sh)
    };
}

macro_rules! DCN301_PANEL_CNTL_REG_FIELD_LIST {
    ($type:ty) => {
        PANEL_BLON: $type,
        PANEL_DIGON: $type,
        PANEL_DIGON_OVRD: $type,
        PANEL_PWRSEQ_TARGET_STATE_R: $type,
        BL_PWM_EN: $type,
        BL_ACTIVE_INT_FRAC_CNT: $type,
        BL_PWM_FRACTIONAL_EN: $type,
        BL_PWM_PERIOD: $type,
        BL_PWM_PERIOD_BITCNT: $type,
        BL_PWM_GRP1_IGNORE_MASTER_LOCK_EN: $type,
        BL_PWM_GRP1_REG_LOCK: $type,
        BL_PWM_GRP1_REG_UPDATE_PENDING: $type,
        BL_PWM_REF_DIV: $type,
    };
}

#[repr(C)]
pub struct dcn301_panel_cntl_shift {
    DCN301_PANEL_CNTL_REG_FIELD_LIST!(u8);
}

#[repr(C)]
pub struct dcn301_panel_cntl_mask {
    DCN301_PANEL_CNTL_REG_FIELD_LIST!(u32);
}

#[repr(C)]
pub struct dcn301_panel_cntl {
    pub base: panel_cntl,
    pub regs: *const dce_panel_cntl_registers,
    pub shift: *const dcn301_panel_cntl_shift,
    pub mask: *const dcn301_panel_cntl_mask,
}

extern "C" {
    pub fn dcn301_panel_cntl_construct(
        panel_cntl: *mut dcn301_panel_cntl,
        init_data: *const panel_cntl_init_data,
        regs: *const dce_panel_cntl_registers,
        shift: *const dcn301_panel_cntl_shift,
        mask: *const dcn301_panel_cntl_mask,
    );
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
