/*
 * Copyright (C) 2023  Advanced Micro Devices, Inc.
 *
 * Permission is hereby granted, free of charge, to any person obtaining a
 * copy of this software and associated documentation files (the "Software"),
 * to deal in the Software without restriction, including without limitation
 * the rights to use, copy, modify, merge, publish, distribute, sublicense,
 * and/or sell copies of the Software, and to permit persons to whom the
 * Software is furnished to do so, subject to the following conditions:
 *
 * The above copyright notice and this permission notice shall be included
 * in all copies or substantial portions of the Software.
 *
 * THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS
 * OR IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
 * FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT.  IN NO EVENT SHALL
 * THE COPYRIGHT HOLDER(S) BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER LIABILITY, WHETHER IN
 * AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM, OUT OF OR IN
 * CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE SOFTWARE.
 */

// addressBlock: smuio_smuio_misc_SmuSmuioDec
// base address: 0x5a000
pub const mmSMUIO_MCM_CONFIG: u32 = 0x0023;
pub const mmSMUIO_MCM_CONFIG_BASE_IDX: u32 = 0;
pub const mmIP_DISCOVERY_VERSION: u32 = 0x0000;
pub const mmIP_DISCOVERY_VERSION_BASE_IDX: u32 = 1;
pub const mmIO_SMUIO_PINSTRAP: u32 = 0x01b1;
pub const mmIO_SMUIO_PINSTRAP_BASE_IDX: u32 = 1;
pub const mmSCRATCH_REGISTER0: u32 = 0x01b2;
pub const mmSCRATCH_REGISTER0_BASE_IDX: u32 = 1;
pub const mmSCRATCH_REGISTER1: u32 = 0x01b3;
pub const mmSCRATCH_REGISTER1_BASE_IDX: u32 = 1;
pub const mmSCRATCH_REGISTER2: u32 = 0x01b4;
pub const mmSCRATCH_REGISTER2_BASE_IDX: u32 = 1;
pub const mmSCRATCH_REGISTER3: u32 = 0x01b5;
pub const mmSCRATCH_REGISTER3_BASE_IDX: u32 = 1;
pub const mmSCRATCH_REGISTER4: u32 = 0x01b6;
pub const mmSCRATCH_REGISTER4_BASE_IDX: u32 = 1;
pub const mmSCRATCH_REGISTER5: u32 = 0x01b7;
pub const mmSCRATCH_REGISTER5_BASE_IDX: u32 = 1;
pub const mmSCRATCH_REGISTER6: u32 = 0x01b8;
pub const mmSCRATCH_REGISTER6_BASE_IDX: u32 = 1;
pub const mmSCRATCH_REGISTER7: u32 = 0x01b9;
pub const mmSCRATCH_REGISTER7_BASE_IDX: u32 = 1;

// addressBlock: smuio_smuio_reset_SmuSmuioDec
// base address: 0x5a300
pub const mmSMUIO_MP_RESET_INTR: u32 = 0x00c1;
pub const mmSMUIO_MP_RESET_INTR_BASE_IDX: u32 = 0;
pub const mmSMUIO_SOC_HALT: u32 = 0x00c2;
pub const mmSMUIO_SOC_HALT_BASE_IDX: u32 = 0;
pub const mmSMUIO_GFX_MISC_CNTL: u32 = 0x00c8;
pub const mmSMUIO_GFX_MISC_CNTL_BASE_IDX: u32 = 0;

// addressBlock: smuio_smuio_ccxctrl_SmuSmuioDec
// base address: 0x5a000
pub const mmPWROK_REFCLK_GAP_CYCLES: u32 = 0x0001;
pub const mmPWROK_REFCLK_GAP_CYCLES_BASE_IDX: u32 = 1;
pub const mmGOLDEN_TSC_INCREMENT_UPPER: u32 = 0x0004;
pub const mmGOLDEN_TSC_INCREMENT_UPPER_BASE_IDX: u32 = 1;
pub const mmGOLDEN_TSC_INCREMENT_LOWER: u32 = 0x0005;
pub const mmGOLDEN_TSC_INCREMENT_LOWER_BASE_IDX: u32 = 1;
pub const mmGOLDEN_TSC_COUNT_UPPER: u32 = 0x0025;
pub const mmGOLDEN_TSC_COUNT_UPPER_BASE_IDX: u32 = 1;
pub const mmGOLDEN_TSC_COUNT_LOWER: u32 = 0x0026;
pub const mmGOLDEN_TSC_COUNT_LOWER_BASE_IDX: u32 = 1;
pub const mmGFX_GOLDEN_TSC_SHADOW_UPPER: u32 = 0x0029;
pub const mmGFX_GOLDEN_TSC_SHADOW_UPPER_BASE_IDX: u32 = 1;
pub const mmGFX_GOLDEN_TSC_SHADOW_LOWER: u32 = 0x002a;
pub const mmGFX_GOLDEN_TSC_SHADOW_LOWER_BASE_IDX: u32 = 1;
pub const mmSOC_GOLDEN_TSC_SHADOW_UPPER: u32 = 0x002b;
pub const mmSOC_GOLDEN_TSC_SHADOW_UPPER_BASE_IDX: u32 = 1;
pub const mmSOC_GOLDEN_TSC_SHADOW_LOWER: u32 = 0x002c;
pub const mmSOC_GOLDEN_TSC_SHADOW_LOWER_BASE_IDX: u32 = 1;
pub const mmSOC_GAP_PWROK: u32 = 0x002d;
pub const mmSOC_GAP_PWROK_BASE_IDX: u32 = 1;

// addressBlock: smuio_smuio_swtimer_SmuSmuioDec
// base address: 0x5ac40
pub const mmPWR_VIRT_RESET_REQ: u32 = 0x0110;
pub const mmPWR_VIRT_RESET_REQ_BASE_IDX: u32 = 1;
pub const mmPWR_DISP_TIMER_CONTROL: u32 = 0x0111;
pub const mmPWR_DISP_TIMER_CONTROL_BASE_IDX: u32 = 1;
pub const mmPWR_DISP_TIMER2_CONTROL: u32 = 0x0113;
pub const mmPWR_DISP_TIMER2_CONTROL_BASE_IDX: u32 = 1;
pub const mmPWR_DISP_TIMER_GLOBAL_CONTROL: u32 = 0x0115;
pub const mmPWR_DISP_TIMER_GLOBAL_CONTROL_BASE_IDX: u32 = 1;
pub const mmPWR_IH_CONTROL: u32 = 0x0116;
pub const mmPWR_IH_CONTROL_BASE_IDX: u32 = 1;

// addressBlock: smuio_smuio_svi0_SmuSmuioDec
// base address: 0x6f000
pub const mmSMUSVI0_TEL_PLANE0: u32 = 0x520e;
pub const mmSMUSVI0_TEL_PLANE0_BASE_IDX: u32 = 1;
pub const mmSMUSVI0_PLANE0_CURRENTVID: u32 = 0x5217;
pub const mmSMUSVI0_PLANE0_CURRENTVID_BASE_IDX: u32 = 1;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
