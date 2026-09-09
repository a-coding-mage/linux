/*
 * Copyright 2011 Advanced Micro Devices, Inc.
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

pub const R600_ASI_DFLT: i32 = 10000;
pub const R600_BSP_DFLT: i32 = 0x41EB;
pub const R600_BSU_DFLT: i32 = 0x2;
pub const R600_AH_DFLT: i32 = 5;
pub const R600_RLP_DFLT: i32 = 25;
pub const R600_RMP_DFLT: i32 = 65;
pub const R600_LHP_DFLT: i32 = 40;
pub const R600_LMP_DFLT: i32 = 15;
pub const R600_TD_DFLT: i32 = 0;

pub const R600_UTC_DFLT_00: i32 = 0x24;
pub const R600_UTC_DFLT_01: i32 = 0x22;
pub const R600_UTC_DFLT_02: i32 = 0x22;
pub const R600_UTC_DFLT_03: i32 = 0x22;
pub const R600_UTC_DFLT_04: i32 = 0x22;
pub const R600_UTC_DFLT_05: i32 = 0x22;
pub const R600_UTC_DFLT_06: i32 = 0x22;
pub const R600_UTC_DFLT_07: i32 = 0x22;
pub const R600_UTC_DFLT_08: i32 = 0x22;
pub const R600_UTC_DFLT_09: i32 = 0x22;
pub const R600_UTC_DFLT_10: i32 = 0x22;
pub const R600_UTC_DFLT_11: i32 = 0x22;
pub const R600_UTC_DFLT_12: i32 = 0x22;
pub const R600_UTC_DFLT_13: i32 = 0x22;
pub const R600_UTC_DFLT_14: i32 = 0x22;

pub const R600_DTC_DFLT_00: i32 = 0x24;
pub const R600_DTC_DFLT_01: i32 = 0x22;
pub const R600_DTC_DFLT_02: i32 = 0x22;
pub const R600_DTC_DFLT_03: i32 = 0x22;
pub const R600_DTC_DFLT_04: i32 = 0x22;
pub const R600_DTC_DFLT_05: i32 = 0x22;
pub const R600_DTC_DFLT_06: i32 = 0x22;
pub const R600_DTC_DFLT_07: i32 = 0x22;
pub const R600_DTC_DFLT_08: i32 = 0x22;
pub const R600_DTC_DFLT_09: i32 = 0x22;
pub const R600_DTC_DFLT_10: i32 = 0x22;
pub const R600_DTC_DFLT_11: i32 = 0x22;
pub const R600_DTC_DFLT_12: i32 = 0x22;
pub const R600_DTC_DFLT_13: i32 = 0x22;
pub const R600_DTC_DFLT_14: i32 = 0x22;

pub const R600_VRC_DFLT: i32 = 0x0000C003;
pub const R600_VOLTAGERESPONSETIME_DFLT: i32 = 1000;
pub const R600_BACKBIASRESPONSETIME_DFLT: i32 = 1000;
pub const R600_VRU_DFLT: i32 = 0x3;
pub const R600_SPLLSTEPTIME_DFLT: i32 = 0x1000;
pub const R600_SPLLSTEPUNIT_DFLT: i32 = 0x3;
pub const R600_TPU_DFLT: i32 = 0;
pub const R600_TPC_DFLT: i32 = 0x200;
pub const R600_SSTU_DFLT: i32 = 0;
pub const R600_SST_DFLT: i32 = 0x00C8;
pub const R600_GICST_DFLT: i32 = 0x200;
pub const R600_FCT_DFLT: i32 = 0x0400;
pub const R600_FCTU_DFLT: i32 = 0;
pub const R600_CTXCGTT3DRPHC_DFLT: i32 = 0x20;
pub const R600_CTXCGTT3DRSDC_DFLT: i32 = 0x40;
pub const R600_VDDC3DOORPHC_DFLT: i32 = 0x100;
pub const R600_VDDC3DOORSDC_DFLT: i32 = 0x7;
pub const R600_VDDC3DOORSU_DFLT: i32 = 0;
pub const R600_MPLLLOCKTIME_DFLT: i32 = 100;
pub const R600_MPLLRESETTIME_DFLT: i32 = 150;
pub const R600_VCOSTEPPCT_DFLT: i32 = 20;
pub const R600_ENDINGVCOSTEPPCT_DFLT: i32 = 5;
pub const R600_REFERENCEDIVIDER_DFLT: i32 = 4;

pub const R600_PM_NUMBER_OF_TC: i32 = 15;
pub const R600_PM_NUMBER_OF_SCLKS: i32 = 20;
pub const R600_PM_NUMBER_OF_MCLKS: i32 = 4;
pub const R600_PM_NUMBER_OF_VOLTAGE_LEVELS: i32 = 4;
pub const R600_PM_NUMBER_OF_ACTIVITY_LEVELS: i32 = 3;

/* XXX are these ok? */
pub const R600_TEMP_RANGE_MIN: i32 = 90 * 1000;
pub const R600_TEMP_RANGE_MAX: i32 = 120 * 1000;

pub const FDO_PWM_MODE_STATIC: i32 = 1;
pub const FDO_PWM_MODE_STATIC_RPM: i32 = 5;

#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum r600_power_level {
    R600_POWER_LEVEL_LOW = 0,
    R600_POWER_LEVEL_MEDIUM = 1,
    R600_POWER_LEVEL_HIGH = 2,
    R600_POWER_LEVEL_CTXSW = 3,
}

#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum r600_td {
    R600_TD_AUTO,
    R600_TD_UP,
    R600_TD_DOWN,
}

#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum r600_display_watermark {
    R600_DISPLAY_WATERMARK_LOW = 0,
    R600_DISPLAY_WATERMARK_HIGH = 1,
}

#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum r600_display_gap {
    R600_PM_DISPLAY_GAP_VBLANK_OR_WM = 0,
    R600_PM_DISPLAY_GAP_VBLANK = 1,
    R600_PM_DISPLAY_GAP_WATERMARK = 2,
    R600_PM_DISPLAY_GAP_IGNORE = 3,
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
