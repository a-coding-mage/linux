/*
 *  BSD LICENSE
 *
 *  Copyright(c) 2017 Broadcom. All rights reserved.
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

/* GENPLL 0 clock channel ID SCR HSLS FS PCIE */
pub const BCM_SR_GENPLL0: u32 = 0;
pub const BCM_SR_GENPLL0_125M_CLK: u32 = 1;
pub const BCM_SR_GENPLL0_SCR_CLK: u32 = 2;
pub const BCM_SR_GENPLL0_250M_CLK: u32 = 3;
pub const BCM_SR_GENPLL0_PCIE_AXI_CLK: u32 = 4;
pub const BCM_SR_GENPLL0_PAXC_AXI_X2_CLK: u32 = 5;
pub const BCM_SR_GENPLL0_PAXC_AXI_CLK: u32 = 6;

/* GENPLL 1 clock channel ID MHB PCIE NITRO */
pub const BCM_SR_GENPLL1: u32 = 0;
pub const BCM_SR_GENPLL1_PCIE_TL_CLK: u32 = 1;
pub const BCM_SR_GENPLL1_MHB_APB_CLK: u32 = 2;

/* GENPLL 2 clock channel ID NITRO MHB*/
pub const BCM_SR_GENPLL2: u32 = 0;
pub const BCM_SR_GENPLL2_NIC_CLK: u32 = 1;
pub const BCM_SR_GENPLL2_TS_500_CLK: u32 = 2;
pub const BCM_SR_GENPLL2_125_NITRO_CLK: u32 = 3;
pub const BCM_SR_GENPLL2_CHIMP_CLK: u32 = 4;
pub const BCM_SR_GENPLL2_NIC_FLASH_CLK: u32 = 5;
pub const BCM_SR_GENPLL2_FS4_CLK: u32 = 6;

/* GENPLL 3 HSLS clock channel ID */
pub const BCM_SR_GENPLL3: u32 = 0;
pub const BCM_SR_GENPLL3_HSLS_CLK: u32 = 1;
pub const BCM_SR_GENPLL3_SDIO_CLK: u32 = 2;

/* GENPLL 4 SCR clock channel ID */
pub const BCM_SR_GENPLL4: u32 = 0;
pub const BCM_SR_GENPLL4_CCN_CLK: u32 = 1;
pub const BCM_SR_GENPLL4_TPIU_PLL_CLK: u32 = 2;
pub const BCM_SR_GENPLL4_NOC_CLK: u32 = 3;
pub const BCM_SR_GENPLL4_CHCLK_FS4_CLK: u32 = 4;
pub const BCM_SR_GENPLL4_BRIDGE_FSCPU_CLK: u32 = 5;

/* GENPLL 5 FS4 clock channel ID */
pub const BCM_SR_GENPLL5: u32 = 0;
pub const BCM_SR_GENPLL5_FS4_HF_CLK: u32 = 1;
pub const BCM_SR_GENPLL5_CRYPTO_AE_CLK: u32 = 2;
pub const BCM_SR_GENPLL5_RAID_AE_CLK: u32 = 3;

/* GENPLL 6 NITRO clock channel ID */
pub const BCM_SR_GENPLL6: u32 = 0;
pub const BCM_SR_GENPLL6_48_USB_CLK: u32 = 1;

/* LCPLL0  clock channel ID */
pub const BCM_SR_LCPLL0: u32 = 0;
pub const BCM_SR_LCPLL0_SATA_REFP_CLK: u32 = 1;
pub const BCM_SR_LCPLL0_SATA_REFN_CLK: u32 = 2;
pub const BCM_SR_LCPLL0_SATA_350_CLK: u32 = 3;
pub const BCM_SR_LCPLL0_SATA_500_CLK: u32 = 4;

/* LCPLL1  clock channel ID */
pub const BCM_SR_LCPLL1: u32 = 0;
pub const BCM_SR_LCPLL1_WAN_CLK: u32 = 1;
pub const BCM_SR_LCPLL1_USB_REF_CLK: u32 = 2;
pub const BCM_SR_LCPLL1_CRMU_TS_CLK: u32 = 3;

/* LCPLL PCIE  clock channel ID */
pub const BCM_SR_LCPLL_PCIE: u32 = 0;
pub const BCM_SR_LCPLL_PCIE_PHY_REF_CLK: u32 = 1;

/* GENPLL EMEM0 clock channel ID */
pub const BCM_SR_EMEMPLL0: u32 = 0;
pub const BCM_SR_EMEMPLL0_EMEM_CLK: u32 = 1;

/* GENPLL EMEM0 clock channel ID */
pub const BCM_SR_EMEMPLL1: u32 = 0;
pub const BCM_SR_EMEMPLL1_EMEM_CLK: u32 = 1;

/* GENPLL EMEM0 clock channel ID */
pub const BCM_SR_EMEMPLL2: u32 = 0;
pub const BCM_SR_EMEMPLL2_EMEM_CLK: u32 = 1;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
