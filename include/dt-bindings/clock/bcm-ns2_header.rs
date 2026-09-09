/*
 *  BSD LICENSE
 *
 *  Copyright(c) 2015 Broadcom Corporation.  All rights reserved.
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

/* GENPLL SCR clock channel ID */
pub const BCM_NS2_GENPLL_SCR: u32 = 0;
pub const BCM_NS2_GENPLL_SCR_SCR_CLK: u32 = 1;
pub const BCM_NS2_GENPLL_SCR_FS_CLK: u32 = 2;
pub const BCM_NS2_GENPLL_SCR_AUDIO_CLK: u32 = 3;
pub const BCM_NS2_GENPLL_SCR_CH3_UNUSED: u32 = 4;
pub const BCM_NS2_GENPLL_SCR_CH4_UNUSED: u32 = 5;
pub const BCM_NS2_GENPLL_SCR_CH5_UNUSED: u32 = 6;

/* GENPLL SW clock channel ID */
pub const BCM_NS2_GENPLL_SW: u32 = 0;
pub const BCM_NS2_GENPLL_SW_RPE_CLK: u32 = 1;
pub const BCM_NS2_GENPLL_SW_250_CLK: u32 = 2;
pub const BCM_NS2_GENPLL_SW_NIC_CLK: u32 = 3;
pub const BCM_NS2_GENPLL_SW_CHIMP_CLK: u32 = 4;
pub const BCM_NS2_GENPLL_SW_PORT_CLK: u32 = 5;
pub const BCM_NS2_GENPLL_SW_SDIO_CLK: u32 = 6;

/* LCPLL DDR clock channel ID */
pub const BCM_NS2_LCPLL_DDR: u32 = 0;
pub const BCM_NS2_LCPLL_DDR_PCIE_SATA_USB_CLK: u32 = 1;
pub const BCM_NS2_LCPLL_DDR_DDR_CLK: u32 = 2;
pub const BCM_NS2_LCPLL_DDR_CH2_UNUSED: u32 = 3;
pub const BCM_NS2_LCPLL_DDR_CH3_UNUSED: u32 = 4;
pub const BCM_NS2_LCPLL_DDR_CH4_UNUSED: u32 = 5;
pub const BCM_NS2_LCPLL_DDR_CH5_UNUSED: u32 = 6;

/* LCPLL PORTS clock channel ID */
pub const BCM_NS2_LCPLL_PORTS: u32 = 0;
pub const BCM_NS2_LCPLL_PORTS_WAN_CLK: u32 = 1;
pub const BCM_NS2_LCPLL_PORTS_RGMII_CLK: u32 = 2;
pub const BCM_NS2_LCPLL_PORTS_CH2_UNUSED: u32 = 3;
pub const BCM_NS2_LCPLL_PORTS_CH3_UNUSED: u32 = 4;
pub const BCM_NS2_LCPLL_PORTS_CH4_UNUSED: u32 = 5;
pub const BCM_NS2_LCPLL_PORTS_CH5_UNUSED: u32 = 6;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
