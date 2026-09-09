/* SPDX-License-Identifier: GPL-2.0-or-later */
/*
 * Copyright (c) 2016 Rockchip Electronics Co. Ltd.
 * Author: Elaine <zhangqing@rock-chips.com>
 */

/* Translated from the C header; the original include guard is not needed in Rust. */
macro_rules! define_clock {
    ($name:ident, $value:expr) => { pub const $name: i32 = $value; };
}

/* core clocks */
define_clock!(PLL_APLL, 1); define_clock!(PLL_DPLL, 2); define_clock!(PLL_CPLL, 3);
define_clock!(PLL_GPLL, 4); define_clock!(PLL_NPLL, 5); define_clock!(ARMCLK, 6);

/* sclk gates (special clocks) */
define_clock!(SCLK_RTC32K, 30); define_clock!(SCLK_SDMMC_EXT, 31); define_clock!(SCLK_SPI, 32);
define_clock!(SCLK_SDMMC, 33); define_clock!(SCLK_SDIO, 34); define_clock!(SCLK_EMMC, 35);
define_clock!(SCLK_TSADC, 36); define_clock!(SCLK_SARADC, 37); define_clock!(SCLK_UART0, 38);
define_clock!(SCLK_UART1, 39); define_clock!(SCLK_UART2, 40); define_clock!(SCLK_I2S0, 41);
define_clock!(SCLK_I2S1, 42); define_clock!(SCLK_I2S2, 43); define_clock!(SCLK_I2S1_OUT, 44);
define_clock!(SCLK_I2S2_OUT, 45); define_clock!(SCLK_SPDIF, 46); define_clock!(SCLK_TIMER0, 47);
define_clock!(SCLK_TIMER1, 48); define_clock!(SCLK_TIMER2, 49); define_clock!(SCLK_TIMER3, 50);
define_clock!(SCLK_TIMER4, 51); define_clock!(SCLK_TIMER5, 52); define_clock!(SCLK_WIFI, 53);
define_clock!(SCLK_CIF_OUT, 54); define_clock!(SCLK_I2C0, 55); define_clock!(SCLK_I2C1, 56);
define_clock!(SCLK_I2C2, 57); define_clock!(SCLK_I2C3, 58); define_clock!(SCLK_CRYPTO, 59);
define_clock!(SCLK_PWM, 60); define_clock!(SCLK_PDM, 61); define_clock!(SCLK_EFUSE, 62);
define_clock!(SCLK_OTP, 63); define_clock!(SCLK_DDRCLK, 64); define_clock!(SCLK_VDEC_CABAC, 65);
define_clock!(SCLK_VDEC_CORE, 66); define_clock!(SCLK_VENC_DSP, 67); define_clock!(SCLK_VENC_CORE, 68);
define_clock!(SCLK_RGA, 69); define_clock!(SCLK_HDMI_SFC, 70); define_clock!(SCLK_HDMI_CEC, 71);
define_clock!(SCLK_USB3_REF, 72); define_clock!(SCLK_USB3_SUSPEND, 73); define_clock!(SCLK_SDMMC_DRV, 74);
define_clock!(SCLK_SDIO_DRV, 75); define_clock!(SCLK_EMMC_DRV, 76); define_clock!(SCLK_SDMMC_EXT_DRV, 77);
define_clock!(SCLK_SDMMC_SAMPLE, 78); define_clock!(SCLK_SDIO_SAMPLE, 79); define_clock!(SCLK_EMMC_SAMPLE, 80);
define_clock!(SCLK_SDMMC_EXT_SAMPLE, 81); define_clock!(SCLK_VOP, 82); define_clock!(SCLK_MAC2PHY_RXTX, 83);
define_clock!(SCLK_MAC2PHY_SRC, 84); define_clock!(SCLK_MAC2PHY_REF, 85); define_clock!(SCLK_MAC2PHY_OUT, 86);
define_clock!(SCLK_MAC2IO_RX, 87); define_clock!(SCLK_MAC2IO_TX, 88); define_clock!(SCLK_MAC2IO_REFOUT, 89);
define_clock!(SCLK_MAC2IO_REF, 90); define_clock!(SCLK_MAC2IO_OUT, 91); define_clock!(SCLK_TSP, 92);
define_clock!(SCLK_HSADC_TSP, 93); define_clock!(SCLK_USB3PHY_REF, 94); define_clock!(SCLK_REF_USB3OTG, 95);
define_clock!(SCLK_USB3OTG_REF, 96); define_clock!(SCLK_USB3OTG_SUSPEND, 97); define_clock!(SCLK_REF_USB3OTG_SRC, 98);
define_clock!(SCLK_MAC2IO_SRC, 99); define_clock!(SCLK_MAC2IO, 100); define_clock!(SCLK_MAC2PHY, 101);
define_clock!(SCLK_MAC2IO_EXT, 102);

/* dclk gates */
define_clock!(DCLK_LCDC, 120); define_clock!(DCLK_HDMIPHY, 121); define_clock!(HDMIPHY, 122);
define_clock!(USB480M, 123); define_clock!(DCLK_LCDC_SRC, 124);

/* aclk gates */
define_clock!(ACLK_AXISRAM, 130); define_clock!(ACLK_VOP_PRE, 131); define_clock!(ACLK_USB3OTG, 132);
define_clock!(ACLK_RGA_PRE, 133); define_clock!(ACLK_DMAC, 134); define_clock!(ACLK_GPU, 135);
define_clock!(ACLK_BUS_PRE, 136); define_clock!(ACLK_PERI_PRE, 137); define_clock!(ACLK_RKVDEC_PRE, 138);
define_clock!(ACLK_RKVDEC, 139); define_clock!(ACLK_RKVENC, 140); define_clock!(ACLK_VPU_PRE, 141);
define_clock!(ACLK_VIO_PRE, 142); define_clock!(ACLK_VPU, 143); define_clock!(ACLK_VIO, 144);
define_clock!(ACLK_VOP, 145); define_clock!(ACLK_GMAC, 146); define_clock!(ACLK_H265, 147);
define_clock!(ACLK_H264, 148); define_clock!(ACLK_MAC2PHY, 149); define_clock!(ACLK_MAC2IO, 150);
define_clock!(ACLK_DCF, 151); define_clock!(ACLK_TSP, 152); define_clock!(ACLK_PERI, 153);
define_clock!(ACLK_RGA, 154); define_clock!(ACLK_IEP, 155); define_clock!(ACLK_CIF, 156); define_clock!(ACLK_HDCP, 157);

/* pclk gates */
define_clock!(PCLK_GPIO0, 200); define_clock!(PCLK_GPIO1, 201); define_clock!(PCLK_GPIO2, 202); define_clock!(PCLK_GPIO3, 203);
define_clock!(PCLK_GRF, 204); define_clock!(PCLK_I2C0, 205); define_clock!(PCLK_I2C1, 206); define_clock!(PCLK_I2C2, 207);
define_clock!(PCLK_I2C3, 208); define_clock!(PCLK_SPI, 209); define_clock!(PCLK_UART0, 210); define_clock!(PCLK_UART1, 211);
define_clock!(PCLK_UART2, 212); define_clock!(PCLK_TSADC, 213); define_clock!(PCLK_PWM, 214); define_clock!(PCLK_TIMER, 215);
define_clock!(PCLK_BUS_PRE, 216); define_clock!(PCLK_PERI_PRE, 217); define_clock!(PCLK_HDMI_CTRL, 218); define_clock!(PCLK_HDMI_PHY, 219);
define_clock!(PCLK_GMAC, 220); define_clock!(PCLK_H265, 221); define_clock!(PCLK_MAC2PHY, 222); define_clock!(PCLK_MAC2IO, 223);
define_clock!(PCLK_USB3PHY_OTG, 224); define_clock!(PCLK_USB3PHY_PIPE, 225); define_clock!(PCLK_USB3_GRF, 226); define_clock!(PCLK_USB2_GRF, 227);
define_clock!(PCLK_HDMIPHY, 228); define_clock!(PCLK_DDR, 229); define_clock!(PCLK_PERI, 230); define_clock!(PCLK_HDMI, 231);
define_clock!(PCLK_HDCP, 232); define_clock!(PCLK_DCF, 233); define_clock!(PCLK_SARADC, 234); define_clock!(PCLK_ACODECPHY, 235); define_clock!(PCLK_WDT, 236);

/* hclk gates */
define_clock!(HCLK_PERI, 308); define_clock!(HCLK_TSP, 309); define_clock!(HCLK_GMAC, 310); define_clock!(HCLK_I2S0_8CH, 311);
define_clock!(HCLK_I2S1_8CH, 312); define_clock!(HCLK_I2S2_2CH, 313); define_clock!(HCLK_SPDIF_8CH, 314); define_clock!(HCLK_VOP, 315);
define_clock!(HCLK_NANDC, 316); define_clock!(HCLK_SDMMC, 317); define_clock!(HCLK_SDIO, 318); define_clock!(HCLK_EMMC, 319);
define_clock!(HCLK_SDMMC_EXT, 320); define_clock!(HCLK_RKVDEC_PRE, 321); define_clock!(HCLK_RKVDEC, 322); define_clock!(HCLK_RKVENC, 323);
define_clock!(HCLK_VPU_PRE, 324); define_clock!(HCLK_VIO_PRE, 325); define_clock!(HCLK_VPU, 326); define_clock!(HCLK_BUS_PRE, 328);
define_clock!(HCLK_PERI_PRE, 329); define_clock!(HCLK_H264, 330); define_clock!(HCLK_CIF, 331); define_clock!(HCLK_OTG_PMU, 332);
define_clock!(HCLK_OTG, 333); define_clock!(HCLK_HOST0, 334); define_clock!(HCLK_HOST0_ARB, 335); define_clock!(HCLK_CRYPTO_MST, 336);
define_clock!(HCLK_CRYPTO_SLV, 337); define_clock!(HCLK_PDM, 338); define_clock!(HCLK_IEP, 339); define_clock!(HCLK_RGA, 340); define_clock!(HCLK_HDCP, 341);

/* soft-reset indices */
define_clock!(SRST_CORE0_PO, 0); define_clock!(SRST_CORE1_PO, 1); define_clock!(SRST_CORE2_PO, 2); define_clock!(SRST_CORE3_PO, 3);
define_clock!(SRST_CORE0, 4); define_clock!(SRST_CORE1, 5); define_clock!(SRST_CORE2, 6); define_clock!(SRST_CORE3, 7);
define_clock!(SRST_CORE0_DBG, 8); define_clock!(SRST_CORE1_DBG, 9); define_clock!(SRST_CORE2_DBG, 10); define_clock!(SRST_CORE3_DBG, 11);
define_clock!(SRST_TOPDBG, 12); define_clock!(SRST_CORE_NIU, 13); define_clock!(SRST_STRC_A, 14); define_clock!(SRST_L2C, 15);
define_clock!(SRST_A53_GIC, 18); define_clock!(SRST_DAP, 19); define_clock!(SRST_PMU_P, 21); define_clock!(SRST_EFUSE, 22);
define_clock!(SRST_BUSSYS_H, 23); define_clock!(SRST_BUSSYS_P, 24); define_clock!(SRST_SPDIF, 25); define_clock!(SRST_INTMEM, 26);
define_clock!(SRST_ROM, 27); define_clock!(SRST_GPIO0, 28); define_clock!(SRST_GPIO1, 29); define_clock!(SRST_GPIO2, 30); define_clock!(SRST_GPIO3, 31);
define_clock!(SRST_I2S0, 32); define_clock!(SRST_I2S1, 33); define_clock!(SRST_I2S2, 34); define_clock!(SRST_I2S0_H, 35);
define_clock!(SRST_I2S1_H, 36); define_clock!(SRST_I2S2_H, 37); define_clock!(SRST_UART0, 38); define_clock!(SRST_UART1, 39); define_clock!(SRST_UART2, 40);
define_clock!(SRST_UART0_P, 41); define_clock!(SRST_UART1_P, 42); define_clock!(SRST_UART2_P, 43); define_clock!(SRST_I2C0, 44); define_clock!(SRST_I2C1, 45); define_clock!(SRST_I2C2, 46); define_clock!(SRST_I2C3, 47);
define_clock!(SRST_I2C0_P, 48); define_clock!(SRST_I2C1_P, 49); define_clock!(SRST_I2C2_P, 50); define_clock!(SRST_I2C3_P, 51); define_clock!(SRST_EFUSE_SE_P, 52); define_clock!(SRST_EFUSE_NS_P, 53); define_clock!(SRST_PWM0, 54); define_clock!(SRST_PWM0_P, 55);
define_clock!(SRST_DMA, 56); define_clock!(SRST_TSP_A, 57); define_clock!(SRST_TSP_H, 58); define_clock!(SRST_TSP, 59); define_clock!(SRST_TSP_HSADC, 60); define_clock!(SRST_DCF_A, 61); define_clock!(SRST_DCF_P, 62);
define_clock!(SRST_SCR, 64); define_clock!(SRST_SPI, 65); define_clock!(SRST_TSADC, 66); define_clock!(SRST_TSADC_P, 67); define_clock!(SRST_CRYPTO, 68); define_clock!(SRST_SGRF, 69); define_clock!(SRST_GRF, 70); define_clock!(SRST_USB_GRF, 71);
define_clock!(SRST_TIMER_6CH_P, 72); define_clock!(SRST_TIMER0, 73); define_clock!(SRST_TIMER1, 74); define_clock!(SRST_TIMER2, 75); define_clock!(SRST_TIMER3, 76); define_clock!(SRST_TIMER4, 77); define_clock!(SRST_TIMER5, 78); define_clock!(SRST_USB3GRF, 79);
define_clock!(SRST_PHYNIU, 80); define_clock!(SRST_HDMIPHY, 81); define_clock!(SRST_VDAC, 82); define_clock!(SRST_ACODEC_p, 83); define_clock!(SRST_SARADC, 85); define_clock!(SRST_SARADC_P, 86); define_clock!(SRST_GRF_DDR, 87); define_clock!(SRST_DFIMON, 88); define_clock!(SRST_MSCH, 89); define_clock!(SRST_DDRMSCH, 91); define_clock!(SRST_DDRCTRL, 92); define_clock!(SRST_DDRCTRL_P, 93); define_clock!(SRST_DDRPHY, 94); define_clock!(SRST_DDRPHY_P, 95);
define_clock!(SRST_GMAC_NIU_A, 96); define_clock!(SRST_GMAC_NIU_P, 97); define_clock!(SRST_GMAC2PHY_A, 98); define_clock!(SRST_GMAC2IO_A, 99); define_clock!(SRST_MACPHY, 100); define_clock!(SRST_OTP_PHY, 101); define_clock!(SRST_GPU_A, 102); define_clock!(SRST_GPU_NIU_A, 103); define_clock!(SRST_SDMMCEXT, 104); define_clock!(SRST_PERIPH_NIU_A, 105); define_clock!(SRST_PERIHP_NIU_H, 106); define_clock!(SRST_PERIHP_P, 107); define_clock!(SRST_PERIPHSYS_H, 108); define_clock!(SRST_MMC0, 109); define_clock!(SRST_SDIO, 110); define_clock!(SRST_EMMC, 111);
define_clock!(SRST_USB2OTG_H, 112); define_clock!(SRST_USB2OTG, 113); define_clock!(SRST_USB2OTG_ADP, 114); define_clock!(SRST_USB2HOST_H, 115); define_clock!(SRST_USB2HOST_ARB, 116); define_clock!(SRST_USB2HOST_AUX, 117); define_clock!(SRST_USB2HOST_EHCIPHY, 118); define_clock!(SRST_USB2HOST_UTMI, 119); define_clock!(SRST_USB3OTG, 120); define_clock!(SRST_USBPOR, 121); define_clock!(SRST_USB2OTG_UTMI, 122); define_clock!(SRST_USB2HOST_PHY_UTMI, 123); define_clock!(SRST_USB3OTG_UTMI, 124); define_clock!(SRST_USB3PHY_U2, 125); define_clock!(SRST_USB3PHY_U3, 126); define_clock!(SRST_USB3PHY_PIPE, 127);
define_clock!(SRST_VIO_A, 128); define_clock!(SRST_VIO_BUS_H, 129); define_clock!(SRST_VIO_H2P_H, 130); define_clock!(SRST_VIO_ARBI_H, 131); define_clock!(SRST_VOP_NIU_A, 132); define_clock!(SRST_VOP_A, 133); define_clock!(SRST_VOP_H, 134); define_clock!(SRST_VOP_D, 135); define_clock!(SRST_RGA, 136); define_clock!(SRST_RGA_NIU_A, 137); define_clock!(SRST_RGA_A, 138); define_clock!(SRST_RGA_H, 139); define_clock!(SRST_IEP_A, 140); define_clock!(SRST_IEP_H, 141); define_clock!(SRST_HDMI, 142); define_clock!(SRST_HDMI_P, 143);
define_clock!(SRST_HDCP_A, 144); define_clock!(SRST_HDCP, 145); define_clock!(SRST_HDCP_H, 146); define_clock!(SRST_CIF_A, 147); define_clock!(SRST_CIF_H, 148); define_clock!(SRST_CIF_P, 149); define_clock!(SRST_OTP_P, 150); define_clock!(SRST_OTP_SBPI, 151); define_clock!(SRST_OTP_USER, 152); define_clock!(SRST_DDRCTRL_A, 153); define_clock!(SRST_DDRSTDY_P, 154); define_clock!(SRST_DDRSTDY, 155); define_clock!(SRST_PDM_H, 156); define_clock!(SRST_PDM, 157); define_clock!(SRST_USB3PHY_OTG_P, 158); define_clock!(SRST_USB3PHY_PIPE_P, 159);
define_clock!(SRST_VCODEC_A, 160); define_clock!(SRST_VCODEC_NIU_A, 161); define_clock!(SRST_VCODEC_H, 162); define_clock!(SRST_VCODEC_NIU_H, 163); define_clock!(SRST_VDEC_A, 164); define_clock!(SRST_VDEC_NIU_A, 165); define_clock!(SRST_VDEC_H, 166); define_clock!(SRST_VDEC_NIU_H, 167); define_clock!(SRST_VDEC_CORE, 168); define_clock!(SRST_VDEC_CABAC, 169); define_clock!(SRST_DDRPHYDIV, 175);
define_clock!(SRST_RKVENC_NIU_A, 176); define_clock!(SRST_RKVENC_NIU_H, 177); define_clock!(SRST_RKVENC_H265_A, 178); define_clock!(SRST_RKVENC_H265_P, 179); define_clock!(SRST_RKVENC_H265_CORE, 180); define_clock!(SRST_RKVENC_H265_DSP, 181); define_clock!(SRST_RKVENC_H264_A, 182); define_clock!(SRST_RKVENC_H264_H, 183); define_clock!(SRST_RKVENC_INTMEM, 184);

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
