/*
 * Copyright 2022 Advanced Micro Devices, Inc.
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
 */

// addressBlock: aid_smuio_smuio_reset_SmuSmuioDec
// base address: 0x5a300
pub const regSMUIO_MP_RESET_INTR: u32 = 0x00c1;
pub const regSMUIO_MP_RESET_INTR_BASE_IDX: u32 = 1;
pub const regSMUIO_SOC_HALT: u32 = 0x00c2;
pub const regSMUIO_SOC_HALT_BASE_IDX: u32 = 1;

// addressBlock: aid_smuio_smuio_tsc_SmuSmuioDec
// base address: 0x5a8a0
pub const regPWROK_REFCLK_GAP_CYCLES: u32 = 0x0028;
pub const regPWROK_REFCLK_GAP_CYCLES_BASE_IDX: u32 = 2;
pub const regGOLDEN_TSC_INCREMENT_UPPER: u32 = 0x002b;
pub const regGOLDEN_TSC_INCREMENT_UPPER_BASE_IDX: u32 = 2;
pub const regGOLDEN_TSC_INCREMENT_LOWER: u32 = 0x002c;
pub const regGOLDEN_TSC_INCREMENT_LOWER_BASE_IDX: u32 = 2;
pub const regGOLDEN_TSC_COUNT_UPPER: u32 = 0x002d;
pub const regGOLDEN_TSC_COUNT_UPPER_BASE_IDX: u32 = 2;
pub const regGOLDEN_TSC_COUNT_LOWER: u32 = 0x002e;
pub const regGOLDEN_TSC_COUNT_LOWER_BASE_IDX: u32 = 2;
pub const regSOC_GOLDEN_TSC_SHADOW_UPPER: u32 = 0x002f;
pub const regSOC_GOLDEN_TSC_SHADOW_UPPER_BASE_IDX: u32 = 2;
pub const regSOC_GOLDEN_TSC_SHADOW_LOWER: u32 = 0x0030;
pub const regSOC_GOLDEN_TSC_SHADOW_LOWER_BASE_IDX: u32 = 2;
pub const regSOC_GAP_PWROK: u32 = 0x0031;
pub const regSOC_GAP_PWROK_BASE_IDX: u32 = 2;

// addressBlock: aid_smuio_smuio_swtimer_SmuSmuioDec
// base address: 0x5ac70
pub const regPWR_VIRT_RESET_REQ: u32 = 0x011c;
pub const regPWR_VIRT_RESET_REQ_BASE_IDX: u32 = 2;
pub const regPWR_DISP_TIMER_CONTROL: u32 = 0x011d;
pub const regPWR_DISP_TIMER_CONTROL_BASE_IDX: u32 = 2;
pub const regPWR_DISP_TIMER_DEBUG: u32 = 0x011e;
pub const regPWR_DISP_TIMER_DEBUG_BASE_IDX: u32 = 2;
pub const regPWR_DISP_TIMER2_CONTROL: u32 = 0x011f;
pub const regPWR_DISP_TIMER2_CONTROL_BASE_IDX: u32 = 2;
pub const regPWR_DISP_TIMER2_DEBUG: u32 = 0x0120;
pub const regPWR_DISP_TIMER2_DEBUG_BASE_IDX: u32 = 2;
pub const regPWR_DISP_TIMER_GLOBAL_CONTROL: u32 = 0x0121;
pub const regPWR_DISP_TIMER_GLOBAL_CONTROL_BASE_IDX: u32 = 2;
pub const regPWR_IH_CONTROL: u32 = 0x0122;
pub const regPWR_IH_CONTROL_BASE_IDX: u32 = 2;

// addressBlock: aid_smuio_smuio_misc_SmuSmuioDec
// base address: 0x5a000
pub const regSMUIO_MCM_CONFIG: u32 = 0x0023;
pub const regSMUIO_MCM_CONFIG_BASE_IDX: u32 = 1;
pub const regIP_DISCOVERY_VERSION: u32 = 0x0000;
pub const regIP_DISCOVERY_VERSION_BASE_IDX: u32 = 2;
pub const regSCRATCH_REGISTER0: u32 = 0x01bd;
pub const regSCRATCH_REGISTER0_BASE_IDX: u32 = 2;
pub const regSCRATCH_REGISTER1: u32 = 0x01be;
pub const regSCRATCH_REGISTER1_BASE_IDX: u32 = 2;
pub const regSCRATCH_REGISTER2: u32 = 0x01bf;
pub const regSCRATCH_REGISTER2_BASE_IDX: u32 = 2;
pub const regSCRATCH_REGISTER3: u32 = 0x01c0;
pub const regSCRATCH_REGISTER3_BASE_IDX: u32 = 2;
pub const regSCRATCH_REGISTER4: u32 = 0x01c1;
pub const regSCRATCH_REGISTER4_BASE_IDX: u32 = 2;
pub const regSCRATCH_REGISTER5: u32 = 0x01c2;
pub const regSCRATCH_REGISTER5_BASE_IDX: u32 = 2;
pub const regSCRATCH_REGISTER6: u32 = 0x01c3;
pub const regSCRATCH_REGISTER6_BASE_IDX: u32 = 2;
pub const regSCRATCH_REGISTER7: u32 = 0x01c4;
pub const regSCRATCH_REGISTER7_BASE_IDX: u32 = 2;

// addressBlock: aid_smuio_smuio_gpio_SmuSmuioDec
// base address: 0x5a500
pub const regSMU_GPIOPAD_SW_INT_STAT: u32 = 0x0140;
pub const regSMU_GPIOPAD_SW_INT_STAT_BASE_IDX: u32 = 1;
pub const regSMU_GPIOPAD_MASK: u32 = 0x0141;
pub const regSMU_GPIOPAD_MASK_BASE_IDX: u32 = 1;
pub const regSMU_GPIOPAD_A: u32 = 0x0142;
pub const regSMU_GPIOPAD_A_BASE_IDX: u32 = 1;
pub const regSMU_GPIOPAD_TXIMPSEL: u32 = 0x0143;
pub const regSMU_GPIOPAD_TXIMPSEL_BASE_IDX: u32 = 1;
pub const regSMU_GPIOPAD_EN: u32 = 0x0144;
pub const regSMU_GPIOPAD_EN_BASE_IDX: u32 = 1;
pub const regSMU_GPIOPAD_Y: u32 = 0x0145;
pub const regSMU_GPIOPAD_Y_BASE_IDX: u32 = 1;
pub const regSMU_GPIOPAD_RXEN: u32 = 0x0146;
pub const regSMU_GPIOPAD_RXEN_BASE_IDX: u32 = 1;
pub const regSMU_GPIOPAD_RCVR_SEL0: u32 = 0x0147;
pub const regSMU_GPIOPAD_RCVR_SEL0_BASE_IDX: u32 = 1;
pub const regSMU_GPIOPAD_RCVR_SEL1: u32 = 0x0148;
pub const regSMU_GPIOPAD_RCVR_SEL1_BASE_IDX: u32 = 1;
pub const regSMU_GPIOPAD_PU_EN: u32 = 0x0149;
pub const regSMU_GPIOPAD_PU_EN_BASE_IDX: u32 = 1;
pub const regSMU_GPIOPAD_PD_EN: u32 = 0x014a;
pub const regSMU_GPIOPAD_PD_EN_BASE_IDX: u32 = 1;
pub const regSMU_GPIOPAD_PINSTRAPS: u32 = 0x014b;
pub const regSMU_GPIOPAD_PINSTRAPS_BASE_IDX: u32 = 1;
pub const regDFT_PINSTRAPS: u32 = 0x014c;
pub const regDFT_PINSTRAPS_BASE_IDX: u32 = 1;
pub const regSMU_GPIOPAD_INT_STAT_EN: u32 = 0x014d;
pub const regSMU_GPIOPAD_INT_STAT_EN_BASE_IDX: u32 = 1;
pub const regSMU_GPIOPAD_INT_STAT: u32 = 0x014e;
pub const regSMU_GPIOPAD_INT_STAT_BASE_IDX: u32 = 1;
pub const regSMU_GPIOPAD_INT_STAT_AK: u32 = 0x014f;
pub const regSMU_GPIOPAD_INT_STAT_AK_BASE_IDX: u32 = 1;
pub const regSMU_GPIOPAD_INT_EN: u32 = 0x0150;
pub const regSMU_GPIOPAD_INT_EN_BASE_IDX: u32 = 1;
pub const regSMU_GPIOPAD_INT_TYPE: u32 = 0x0151;
pub const regSMU_GPIOPAD_INT_TYPE_BASE_IDX: u32 = 1;
pub const regSMU_GPIOPAD_INT_POLARITY: u32 = 0x0152;
pub const regSMU_GPIOPAD_INT_POLARITY_BASE_IDX: u32 = 1;
pub const regSMUIO_PCC_GPIO_SELECT: u32 = 0x0155;
pub const regSMUIO_PCC_GPIO_SELECT_BASE_IDX: u32 = 1;
pub const regSMU_GPIOPAD_S0: u32 = 0x0156;
pub const regSMU_GPIOPAD_S0_BASE_IDX: u32 = 1;
pub const regSMU_GPIOPAD_S1: u32 = 0x0157;
pub const regSMU_GPIOPAD_S1_BASE_IDX: u32 = 1;
pub const regSMU_GPIOPAD_SCHMEN: u32 = 0x0158;
pub const regSMU_GPIOPAD_SCHMEN_BASE_IDX: u32 = 1;
pub const regSMU_GPIOPAD_SCL_EN: u32 = 0x0159;
pub const regSMU_GPIOPAD_SCL_EN_BASE_IDX: u32 = 1;
pub const regSMU_GPIOPAD_SDA_EN: u32 = 0x015a;
pub const regSMU_GPIOPAD_SDA_EN_BASE_IDX: u32 = 1;
pub const regSMUIO_GPIO_INT0_SELECT: u32 = 0x015b;
pub const regSMUIO_GPIO_INT0_SELECT_BASE_IDX: u32 = 1;
pub const regSMUIO_GPIO_INT1_SELECT: u32 = 0x015c;
pub const regSMUIO_GPIO_INT1_SELECT_BASE_IDX: u32 = 1;
pub const regSMUIO_GPIO_INT2_SELECT: u32 = 0x015d;
pub const regSMUIO_GPIO_INT2_SELECT_BASE_IDX: u32 = 1;
pub const regSMUIO_GPIO_INT3_SELECT: u32 = 0x015e;
pub const regSMUIO_GPIO_INT3_SELECT_BASE_IDX: u32 = 1;
pub const regSMU_GPIOPAD_MP_INT0_STAT: u32 = 0x015f;
pub const regSMU_GPIOPAD_MP_INT0_STAT_BASE_IDX: u32 = 1;
pub const regSMU_GPIOPAD_MP_INT1_STAT: u32 = 0x0160;
pub const regSMU_GPIOPAD_MP_INT1_STAT_BASE_IDX: u32 = 1;
pub const regSMU_GPIOPAD_MP_INT2_STAT: u32 = 0x0161;
pub const regSMU_GPIOPAD_MP_INT2_STAT_BASE_IDX: u32 = 1;
pub const regSMU_GPIOPAD_MP_INT3_STAT: u32 = 0x0162;
pub const regSMU_GPIOPAD_MP_INT3_STAT_BASE_IDX: u32 = 1;
pub const regSMIO_INDEX: u32 = 0x0163;
pub const regSMIO_INDEX_BASE_IDX: u32 = 1;
pub const regS0_VID_SMIO_CNTL: u32 = 0x0164;
pub const regS0_VID_SMIO_CNTL_BASE_IDX: u32 = 1;
pub const regS1_VID_SMIO_CNTL: u32 = 0x0165;
pub const regS1_VID_SMIO_CNTL_BASE_IDX: u32 = 1;
pub const regOPEN_DRAIN_SELECT: u32 = 0x0166;
pub const regOPEN_DRAIN_SELECT_BASE_IDX: u32 = 1;
pub const regSMIO_ENABLE: u32 = 0x0167;
pub const regSMIO_ENABLE_BASE_IDX: u32 = 1;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
