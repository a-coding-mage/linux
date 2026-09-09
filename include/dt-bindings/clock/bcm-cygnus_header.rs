/*
 *  BSD LICENSE
 *
 *  Copyright(c) 2014 Broadcom Corporation.  All rights reserved.
 *
 *  Redistribution and use in source and binary forms, with or without
 *  modification, are permitted provided that the following conditions
 *  are met:
 *
 *    * Redistributions of source code must retain the above copyright
 *      notice, this list of conditions and the following disclaimer.
 *    * Redistributions in binary form must reproduce the above copyright
 *      notice, this list of conditions and the following disclaimer in
 *      the documentation and/or other materials provided with the
 *      distribution.
 *    * Neither the name of Broadcom Corporation nor the names of its
 *      contributors may be used to endorse or promote products derived
 *      from this software without specific prior written permission.
 *
 *  THIS SOFTWARE IS PROVIDED BY THE COPYRIGHT HOLDERS AND CONTRIBUTORS
 *  "AS IS" AND ANY EXPRESS OR IMPLIED WARRANTIES, INCLUDING, BUT NOT
 *  LIMITED TO, THE IMPLIED WARRANTIES OF MERCHANTABILITY AND FITNESS FOR
 *  A PARTICULAR PURPOSE ARE DISCLAIMED. IN NO EVENT SHALL THE COPYRIGHT
 *  OWNER OR CONTRIBUTORS BE LIABLE FOR ANY DIRECT, INDIRECT, INCIDENTAL,
 *  SPECIAL, EXEMPLARY, OR CONSEQUENTIAL DAMAGES (INCLUDING, BUT NOT
 *  LIMITED TO, PROCUREMENT OF SUBSTITUTE GOODS OR SERVICES; LOSS OF USE,
 *  DATA, OR PROFITS; OR BUSINESS INTERRUPTION) HOWEVER CAUSED AND ON ANY
 *  THEORY OF LIABILITY, WHETHER IN CONTRACT, STRICT LIABILITY, OR TORT
 *  (INCLUDING NEGLIGENCE OR OTHERWISE) ARISING IN ANY WAY OUT OF THE USE
 *  OF THIS SOFTWARE, EVEN IF ADVISED OF THE POSSIBILITY OF SUCH DAMAGE.
 */

/* GENPLL clock ID */
pub const BCM_CYGNUS_GENPLL: i32 = 0;
pub const BCM_CYGNUS_GENPLL_AXI21_CLK: i32 = 1;
pub const BCM_CYGNUS_GENPLL_250MHZ_CLK: i32 = 2;
pub const BCM_CYGNUS_GENPLL_IHOST_SYS_CLK: i32 = 3;
pub const BCM_CYGNUS_GENPLL_ENET_SW_CLK: i32 = 4;
pub const BCM_CYGNUS_GENPLL_AUDIO_125_CLK: i32 = 5;
pub const BCM_CYGNUS_GENPLL_CAN_CLK: i32 = 6;

/* LCPLL0 clock ID */
pub const BCM_CYGNUS_LCPLL0: i32 = 0;
pub const BCM_CYGNUS_LCPLL0_PCIE_PHY_REF_CLK: i32 = 1;
pub const BCM_CYGNUS_LCPLL0_DDR_PHY_CLK: i32 = 2;
pub const BCM_CYGNUS_LCPLL0_SDIO_CLK: i32 = 3;
pub const BCM_CYGNUS_LCPLL0_USB_PHY_REF_CLK: i32 = 4;
pub const BCM_CYGNUS_LCPLL0_SMART_CARD_CLK: i32 = 5;
pub const BCM_CYGNUS_LCPLL0_CH5_UNUSED: i32 = 6;

/* MIPI PLL clock ID */
pub const BCM_CYGNUS_MIPIPLL: i32 = 0;
pub const BCM_CYGNUS_MIPIPLL_CH0_UNUSED: i32 = 1;
pub const BCM_CYGNUS_MIPIPLL_CH1_LCD: i32 = 2;
pub const BCM_CYGNUS_MIPIPLL_CH2_V3D: i32 = 3;
pub const BCM_CYGNUS_MIPIPLL_CH3_UNUSED: i32 = 4;
pub const BCM_CYGNUS_MIPIPLL_CH4_UNUSED: i32 = 5;
pub const BCM_CYGNUS_MIPIPLL_CH5_UNUSED: i32 = 6;

/* ASIU clock ID */
pub const BCM_CYGNUS_ASIU_KEYPAD_CLK: i32 = 0;
pub const BCM_CYGNUS_ASIU_ADC_CLK: i32 = 1;
pub const BCM_CYGNUS_ASIU_PWM_CLK: i32 = 2;

/* AUDIO clock ID */
pub const BCM_CYGNUS_AUDIOPLL: i32 = 0;
pub const BCM_CYGNUS_AUDIOPLL_CH0: i32 = 1;
pub const BCM_CYGNUS_AUDIOPLL_CH1: i32 = 2;
pub const BCM_CYGNUS_AUDIOPLL_CH2: i32 = 3;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
