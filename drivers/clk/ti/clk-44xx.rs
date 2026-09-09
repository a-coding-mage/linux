// Translation of clk-44xx.c. External kernel types, constants, and functions are supplied by other modules.
#![allow(non_upper_case_globals, non_camel_case_types, dead_code)]

// SPDX-License-Identifier: GPL-2.0-only
/*
 * OMAP4 Clock init
 *
 * Copyright (C) 2013 Texas Instruments, Inc.
 *
 * Tero Kristo (t-kristo@ti.com)
 */



/*
 * OMAP4 ABE DPLL default frequency. In OMAP4460 TRM version V, section
 * Some(b"3.6.3.2.3 CM1_ABE Clock Generator\0") states that the Some(b"DPLL_ABE_X2_CLK
 * must be set to 196.608 MHz\0") and hence, the DPLL locked frequency is
 * half of this value.
 */
const OMAP4_DPLL_ABE_DEFFREQ: u64 = 98304000;

/*
 * OMAP4 USB DPLL default frequency. In OMAP4430 TRM version V, section
 * Some(b"3.6.3.9.5 DPLL_USB Preferred Settings\0") shows that the preferred
 * locked frequency for the USB DPLL is 960MHz.
 */
const OMAP4_DPLL_USB_DEFFREQ: u64 = 960000000;

static omap4_mpuss_clkctrl_regs: [omap_clkctrl_reg_data; 64] = [
	[OMAP4_MPU_CLKCTRL, None, 0, Some(b"dpll_mpu_m2_ck\0")],
	$1,
};

static omap4_tesla_clkctrl_regs: [omap_clkctrl_reg_data; 64] = [
	[OMAP4_DSP_CLKCTRL, None, CLKF_HW_SUP | CLKF_NO_IDLEST, Some(b"dpll_iva_m4x2_ck\0")],
	$1,
};

static omap4_aess_fclk_parents: [Option<&'static [u8]>; 4] = [
	Some(b"abe_clk\0"),
	None,
};

static omap4_aess_fclk_data: omap_clkctrl_div_data = omap_clkctrl_div_data [max_div: 2,];

static omap4_aess_bit_data: [omap_clkctrl_bit_data; 64] = [
	[24, TI_CLK_DIVIDER, omap4_aess_fclk_parents, &omap4_aess_fclk_data],
	$1,
};

static omap4_func_dmic_abe_gfclk_parents: [Option<&'static [u8]>; 4] = [
	Some(b"abe-clkctrl:0018:26\0"), Some(b"pad_clks_ck\0"), Some(b"slimbus_clk\0"),
	None,
};

static omap4_dmic_sync_mux_ck_parents: [Option<&'static [u8]>; 4] = [
	Some(b"abe_24m_fclk\0"), Some(b"syc_clk_div_ck\0"), Some(b"func_24m_clk\0"),
	None,
};

static omap4_dmic_bit_data: [omap_clkctrl_bit_data; 64] = [
	[24, TI_CLK_MUX, omap4_func_dmic_abe_gfclk_parents, None],
	[26, TI_CLK_MUX, omap4_dmic_sync_mux_ck_parents, None],
	$1,
};

static omap4_func_mcasp_abe_gfclk_parents: [Option<&'static [u8]>; 4] = [
	Some(b"abe-clkctrl:0020:26\0"), Some(b"pad_clks_ck\0"), Some(b"slimbus_clk\0"),
	None,
};

static omap4_mcasp_bit_data: [omap_clkctrl_bit_data; 64] = [
	[24, TI_CLK_MUX, omap4_func_mcasp_abe_gfclk_parents, None],
	[26, TI_CLK_MUX, omap4_dmic_sync_mux_ck_parents, None],
	$1,
};

static omap4_func_mcbsp1_gfclk_parents: [Option<&'static [u8]>; 4] = [
	Some(b"abe-clkctrl:0028:26\0"), Some(b"pad_clks_ck\0"), Some(b"slimbus_clk\0"),
	None,
};

static omap4_mcbsp1_bit_data: [omap_clkctrl_bit_data; 64] = [
	[24, TI_CLK_MUX, omap4_func_mcbsp1_gfclk_parents, None],
	[26, TI_CLK_MUX, omap4_dmic_sync_mux_ck_parents, None],
	$1,
};

static omap4_func_mcbsp2_gfclk_parents: [Option<&'static [u8]>; 4] = [
	Some(b"abe-clkctrl:0030:26\0"), Some(b"pad_clks_ck\0"), Some(b"slimbus_clk\0"),
	None,
};

static omap4_mcbsp2_bit_data: [omap_clkctrl_bit_data; 64] = [
	[24, TI_CLK_MUX, omap4_func_mcbsp2_gfclk_parents, None],
	[26, TI_CLK_MUX, omap4_dmic_sync_mux_ck_parents, None],
	$1,
};

static omap4_func_mcbsp3_gfclk_parents: [Option<&'static [u8]>; 4] = [
	Some(b"abe-clkctrl:0038:26\0"), Some(b"pad_clks_ck\0"), Some(b"slimbus_clk\0"),
	None,
};

static omap4_mcbsp3_bit_data: [omap_clkctrl_bit_data; 64] = [
	[24, TI_CLK_MUX, omap4_func_mcbsp3_gfclk_parents, None],
	[26, TI_CLK_MUX, omap4_dmic_sync_mux_ck_parents, None],
	$1,
};

static omap4_slimbus1_fclk_0_parents: [Option<&'static [u8]>; 4] = [
	Some(b"abe_24m_fclk\0"),
	None,
};

static omap4_slimbus1_fclk_1_parents: [Option<&'static [u8]>; 4] = [
	Some(b"func_24m_clk\0"),
	None,
};

static omap4_slimbus1_fclk_2_parents: [Option<&'static [u8]>; 4] = [
	Some(b"pad_clks_ck\0"),
	None,
};

static omap4_slimbus1_slimbus_clk_parents: [Option<&'static [u8]>; 4] = [
	Some(b"slimbus_clk\0"),
	None,
};

static omap4_slimbus1_bit_data: [omap_clkctrl_bit_data; 64] = [
	[8, TI_CLK_GATE, omap4_slimbus1_fclk_0_parents, None],
	[9, TI_CLK_GATE, omap4_slimbus1_fclk_1_parents, None],
	[10, TI_CLK_GATE, omap4_slimbus1_fclk_2_parents, None],
	[11, TI_CLK_GATE, omap4_slimbus1_slimbus_clk_parents, None],
	$1,
};

static omap4_timer5_sync_mux_parents: [Option<&'static [u8]>; 4] = [
	Some(b"syc_clk_div_ck\0"), Some(b"sys_32k_ck\0"),
	None,
};

static omap4_timer5_bit_data: [omap_clkctrl_bit_data; 64] = [
	[24, TI_CLK_MUX, omap4_timer5_sync_mux_parents, None],
	$1,
};

static omap4_timer6_bit_data: [omap_clkctrl_bit_data; 64] = [
	[24, TI_CLK_MUX, omap4_timer5_sync_mux_parents, None],
	$1,
};

static omap4_timer7_bit_data: [omap_clkctrl_bit_data; 64] = [
	[24, TI_CLK_MUX, omap4_timer5_sync_mux_parents, None],
	$1,
};

static omap4_timer8_bit_data: [omap_clkctrl_bit_data; 64] = [
	[24, TI_CLK_MUX, omap4_timer5_sync_mux_parents, None],
	$1,
};

static omap4_abe_clkctrl_regs: [omap_clkctrl_reg_data; 64] = [
	[OMAP4_L4_ABE_CLKCTRL, None, 0, Some(b"ocp_abe_iclk\0")],
	[OMAP4_AESS_CLKCTRL, omap4_aess_bit_data, CLKF_SW_SUP, Some(b"abe-clkctrl:0008:24\0")],
	[OMAP4_MCPDM_CLKCTRL, None, CLKF_SW_SUP, Some(b"pad_clks_ck\0")],
	[OMAP4_DMIC_CLKCTRL, omap4_dmic_bit_data, CLKF_SW_SUP, Some(b"abe-clkctrl:0018:24\0")],
	[OMAP4_MCASP_CLKCTRL, omap4_mcasp_bit_data, CLKF_SW_SUP, Some(b"abe-clkctrl:0020:24\0")],
	[OMAP4_MCBSP1_CLKCTRL, omap4_mcbsp1_bit_data, CLKF_SW_SUP, Some(b"abe-clkctrl:0028:24\0")],
	[OMAP4_MCBSP2_CLKCTRL, omap4_mcbsp2_bit_data, CLKF_SW_SUP, Some(b"abe-clkctrl:0030:24\0")],
	[OMAP4_MCBSP3_CLKCTRL, omap4_mcbsp3_bit_data, CLKF_SW_SUP, Some(b"abe-clkctrl:0038:24\0")],
	[OMAP4_SLIMBUS1_CLKCTRL, omap4_slimbus1_bit_data, CLKF_SW_SUP, Some(b"abe-clkctrl:0040:8\0")],
	[OMAP4_TIMER5_CLKCTRL, omap4_timer5_bit_data, CLKF_SW_SUP, Some(b"abe-clkctrl:0048:24\0")],
	[OMAP4_TIMER6_CLKCTRL, omap4_timer6_bit_data, CLKF_SW_SUP, Some(b"abe-clkctrl:0050:24\0")],
	[OMAP4_TIMER7_CLKCTRL, omap4_timer7_bit_data, CLKF_SW_SUP, Some(b"abe-clkctrl:0058:24\0")],
	[OMAP4_TIMER8_CLKCTRL, omap4_timer8_bit_data, CLKF_SW_SUP, Some(b"abe-clkctrl:0060:24\0")],
	[OMAP4_WD_TIMER3_CLKCTRL, None, CLKF_SW_SUP, Some(b"sys_32k_ck\0")],
	$1,
};

static omap4_l4_ao_clkctrl_regs: [omap_clkctrl_reg_data; 64] = [
	[OMAP4_SMARTREFLEX_MPU_CLKCTRL, None, CLKF_SW_SUP, Some(b"l4_wkup_clk_mux_ck\0")],
	[OMAP4_SMARTREFLEX_IVA_CLKCTRL, None, CLKF_SW_SUP, Some(b"l4_wkup_clk_mux_ck\0")],
	[OMAP4_SMARTREFLEX_CORE_CLKCTRL, None, CLKF_SW_SUP, Some(b"l4_wkup_clk_mux_ck\0")],
	$1,
};

static omap4_l3_1_clkctrl_regs: [omap_clkctrl_reg_data; 64] = [
	[OMAP4_L3_MAIN_1_CLKCTRL, None, 0, Some(b"l3_div_ck\0")],
	$1,
};

static omap4_l3_2_clkctrl_regs: [omap_clkctrl_reg_data; 64] = [
	[OMAP4_L3_MAIN_2_CLKCTRL, None, 0, Some(b"l3_div_ck\0")],
	[OMAP4_GPMC_CLKCTRL, None, CLKF_HW_SUP, Some(b"l3_div_ck\0")],
	[OMAP4_OCMC_RAM_CLKCTRL, None, 0, Some(b"l3_div_ck\0")],
	$1,
};

static omap4_ducati_clkctrl_regs: [omap_clkctrl_reg_data; 64] = [
	[OMAP4_IPU_CLKCTRL, None, CLKF_HW_SUP | CLKF_NO_IDLEST, Some(b"ducati_clk_mux_ck\0")],
	$1,
};

static omap4_l3_dma_clkctrl_regs: [omap_clkctrl_reg_data; 64] = [
	[OMAP4_DMA_SYSTEM_CLKCTRL, None, 0, Some(b"l3_div_ck\0")],
	$1,
};

static omap4_l3_emif_clkctrl_regs: [omap_clkctrl_reg_data; 64] = [
	[OMAP4_DMM_CLKCTRL, None, 0, Some(b"l3_div_ck\0")],
	[OMAP4_EMIF1_CLKCTRL, None, CLKF_HW_SUP, Some(b"ddrphy_ck\0")],
	[OMAP4_EMIF2_CLKCTRL, None, CLKF_HW_SUP, Some(b"ddrphy_ck\0")],
	$1,
};

static omap4_d2d_clkctrl_regs: [omap_clkctrl_reg_data; 64] = [
	[OMAP4_C2C_CLKCTRL, None, 0, Some(b"div_core_ck\0")],
	$1,
};

static omap4_l4_cfg_clkctrl_regs: [omap_clkctrl_reg_data; 64] = [
	[OMAP4_L4_CFG_CLKCTRL, None, 0, Some(b"l4_div_ck\0")],
	[OMAP4_SPINLOCK_CLKCTRL, None, 0, Some(b"l4_div_ck\0")],
	[OMAP4_MAILBOX_CLKCTRL, None, 0, Some(b"l4_div_ck\0")],
	$1,
};

static omap4_l3_instr_clkctrl_regs: [omap_clkctrl_reg_data; 64] = [
	[OMAP4_L3_MAIN_3_CLKCTRL, None, CLKF_HW_SUP, Some(b"l3_div_ck\0")],
	[OMAP4_L3_INSTR_CLKCTRL, None, CLKF_HW_SUP, Some(b"l3_div_ck\0")],
	[OMAP4_OCP_WP_NOC_CLKCTRL, None, CLKF_HW_SUP, Some(b"l3_div_ck\0")],
	$1,
};

static omap4_ivahd_clkctrl_regs: [omap_clkctrl_reg_data; 64] = [
	[OMAP4_IVA_CLKCTRL, None, CLKF_HW_SUP | CLKF_NO_IDLEST, Some(b"dpll_iva_m5x2_ck\0")],
	[OMAP4_SL2IF_CLKCTRL, None, CLKF_HW_SUP, Some(b"dpll_iva_m5x2_ck\0")],
	$1,
};

static omap4_iss_ctrlclk_parents: [Option<&'static [u8]>; 4] = [
	Some(b"func_96m_fclk\0"),
	None,
};

static omap4_iss_bit_data: [omap_clkctrl_bit_data; 64] = [
	[8, TI_CLK_GATE, omap4_iss_ctrlclk_parents, None],
	$1,
};

static omap4_fdif_fck_parents: [Option<&'static [u8]>; 4] = [
	Some(b"dpll_per_m4x2_ck\0"),
	None,
};

static omap4_fdif_fck_data: omap_clkctrl_div_data = omap_clkctrl_div_data {
	max_div: 4,
	flags: CLK_DIVIDER_POWER_OF_TWO,
};

static omap4_fdif_bit_data: [omap_clkctrl_bit_data; 64] = [
	[24, TI_CLK_DIVIDER, omap4_fdif_fck_parents, &omap4_fdif_fck_data],
	$1,
};

static omap4_iss_clkctrl_regs: [omap_clkctrl_reg_data; 64] = [
	[OMAP4_ISS_CLKCTRL, omap4_iss_bit_data, CLKF_SW_SUP, Some(b"ducati_clk_mux_ck\0")],
	[OMAP4_FDIF_CLKCTRL, omap4_fdif_bit_data, CLKF_SW_SUP, Some(b"iss-clkctrl:0008:24\0")],
	$1,
};

static omap4_dss_dss_clk_parents: [Option<&'static [u8]>; 4] = [
	Some(b"dpll_per_m5x2_ck\0"),
	None,
};

static omap4_dss_48mhz_clk_parents: [Option<&'static [u8]>; 4] = [
	Some(b"func_48mc_fclk\0"),
	None,
};

static omap4_dss_sys_clk_parents: [Option<&'static [u8]>; 4] = [
	Some(b"syc_clk_div_ck\0"),
	None,
};

static omap4_dss_tv_clk_parents: [Option<&'static [u8]>; 4] = [
	Some(b"extalt_clkin_ck\0"),
	None,
};

static omap4_dss_core_bit_data: [omap_clkctrl_bit_data; 64] = [
	[8, TI_CLK_GATE, omap4_dss_dss_clk_parents, None],
	[9, TI_CLK_GATE, omap4_dss_48mhz_clk_parents, None],
	[10, TI_CLK_GATE, omap4_dss_sys_clk_parents, None],
	[11, TI_CLK_GATE, omap4_dss_tv_clk_parents, None],
	$1,
};

static omap4_l3_dss_clkctrl_regs: [omap_clkctrl_reg_data; 64] = [
	[OMAP4_DSS_CORE_CLKCTRL, omap4_dss_core_bit_data, CLKF_SW_SUP, Some(b"l3-dss-clkctrl:0000:8\0")],
	$1,
};

static omap4_sgx_clk_mux_parents: [Option<&'static [u8]>; 4] = [
	Some(b"dpll_core_m7x2_ck\0"), Some(b"dpll_per_m7x2_ck\0"),
	None,
};

static omap4_gpu_bit_data: [omap_clkctrl_bit_data; 64] = [
	[24, TI_CLK_MUX, omap4_sgx_clk_mux_parents, None],
	$1,
};

static omap4_l3_gfx_clkctrl_regs: [omap_clkctrl_reg_data; 64] = [
	[OMAP4_GPU_CLKCTRL, omap4_gpu_bit_data, CLKF_SW_SUP, Some(b"l3-gfx-clkctrl:0000:24\0")],
	$1,
};

static omap4_hsmmc1_fclk_parents: [Option<&'static [u8]>; 4] = [
	Some(b"func_64m_fclk\0"), Some(b"func_96m_fclk\0"),
	None,
};

static omap4_mmc1_bit_data: [omap_clkctrl_bit_data; 64] = [
	[24, TI_CLK_MUX, omap4_hsmmc1_fclk_parents, None],
	$1,
};

static omap4_mmc2_bit_data: [omap_clkctrl_bit_data; 64] = [
	[24, TI_CLK_MUX, omap4_hsmmc1_fclk_parents, None],
	$1,
};

static omap4_hsi_fck_parents: [Option<&'static [u8]>; 4] = [
	Some(b"dpll_per_m2x2_ck\0"),
	None,
};

static omap4_hsi_fck_data: omap_clkctrl_div_data = omap_clkctrl_div_data {
	max_div: 4,
	flags: CLK_DIVIDER_POWER_OF_TWO,
};

static omap4_hsi_bit_data: [omap_clkctrl_bit_data; 64] = [
	[24, TI_CLK_DIVIDER, omap4_hsi_fck_parents, &omap4_hsi_fck_data],
	$1,
};

static omap4_usb_host_hs_utmi_p1_clk_parents: [Option<&'static [u8]>; 4] = [
	Some(b"l3-init-clkctrl:0038:24\0"),
	None,
};

static omap4_usb_host_hs_utmi_p2_clk_parents: [Option<&'static [u8]>; 4] = [
	Some(b"l3-init-clkctrl:0038:25\0"),
	None,
};

static omap4_usb_host_hs_utmi_p3_clk_parents: [Option<&'static [u8]>; 4] = [
	Some(b"init_60m_fclk\0"),
	None,
};

static omap4_usb_host_hs_hsic480m_p1_clk_parents: [Option<&'static [u8]>; 4] = [
	Some(b"dpll_usb_m2_ck\0"),
	None,
};

static omap4_utmi_p1_gfclk_parents: [Option<&'static [u8]>; 4] = [
	Some(b"init_60m_fclk\0"), Some(b"xclk60mhsp1_ck\0"),
	None,
};

static omap4_utmi_p2_gfclk_parents: [Option<&'static [u8]>; 4] = [
	Some(b"init_60m_fclk\0"), Some(b"xclk60mhsp2_ck\0"),
	None,
};

static omap4_usb_host_hs_bit_data: [omap_clkctrl_bit_data; 64] = [
	[8, TI_CLK_GATE, omap4_usb_host_hs_utmi_p1_clk_parents, None],
	[9, TI_CLK_GATE, omap4_usb_host_hs_utmi_p2_clk_parents, None],
	[10, TI_CLK_GATE, omap4_usb_host_hs_utmi_p3_clk_parents, None],
	[11, TI_CLK_GATE, omap4_usb_host_hs_utmi_p3_clk_parents, None],
	[12, TI_CLK_GATE, omap4_usb_host_hs_utmi_p3_clk_parents, None],
	[13, TI_CLK_GATE, omap4_usb_host_hs_hsic480m_p1_clk_parents, None],
	[14, TI_CLK_GATE, omap4_usb_host_hs_hsic480m_p1_clk_parents, None],
	[15, TI_CLK_GATE, omap4_dss_48mhz_clk_parents, None],
	[24, TI_CLK_MUX, omap4_utmi_p1_gfclk_parents, None],
	[25, TI_CLK_MUX, omap4_utmi_p2_gfclk_parents, None],
	$1,
};

static omap4_usb_otg_hs_xclk_parents: [Option<&'static [u8]>; 4] = [
	Some(b"l3-init-clkctrl:0040:24\0"),
	None,
};

static omap4_otg_60m_gfclk_parents: [Option<&'static [u8]>; 4] = [
	Some(b"utmi_phy_clkout_ck\0"), Some(b"xclk60motg_ck\0"),
	None,
};

static omap4_usb_otg_hs_bit_data: [omap_clkctrl_bit_data; 64] = [
	[8, TI_CLK_GATE, omap4_usb_otg_hs_xclk_parents, None],
	[24, TI_CLK_MUX, omap4_otg_60m_gfclk_parents, None],
	$1,
};

static omap4_usb_tll_hs_bit_data: [omap_clkctrl_bit_data; 64] = [
	[8, TI_CLK_GATE, omap4_usb_host_hs_utmi_p3_clk_parents, None],
	[9, TI_CLK_GATE, omap4_usb_host_hs_utmi_p3_clk_parents, None],
	[10, TI_CLK_GATE, omap4_usb_host_hs_utmi_p3_clk_parents, None],
	$1,
};

static omap4_ocp2scp_usb_phy_phy_48m_parents: [Option<&'static [u8]>; 4] = [
	Some(b"func_48m_fclk\0"),
	None,
};

static omap4_ocp2scp_usb_phy_bit_data: [omap_clkctrl_bit_data; 64] = [
	[8, TI_CLK_GATE, omap4_ocp2scp_usb_phy_phy_48m_parents, None],
	$1,
};

static omap4_l3_init_clkctrl_regs: [omap_clkctrl_reg_data; 64] = [
	[OMAP4_MMC1_CLKCTRL, omap4_mmc1_bit_data, CLKF_SW_SUP, Some(b"l3-init-clkctrl:0008:24\0")],
	[OMAP4_MMC2_CLKCTRL, omap4_mmc2_bit_data, CLKF_SW_SUP, Some(b"l3-init-clkctrl:0010:24\0")],
	[OMAP4_HSI_CLKCTRL, omap4_hsi_bit_data, CLKF_HW_SUP, Some(b"l3-init-clkctrl:0018:24\0")],
	[OMAP4_USB_HOST_HS_CLKCTRL, omap4_usb_host_hs_bit_data, CLKF_SW_SUP, Some(b"init_60m_fclk\0")],
	[OMAP4_USB_OTG_HS_CLKCTRL, omap4_usb_otg_hs_bit_data, CLKF_HW_SUP, Some(b"l3_div_ck\0")],
	[OMAP4_USB_TLL_HS_CLKCTRL, omap4_usb_tll_hs_bit_data, CLKF_HW_SUP, Some(b"l4_div_ck\0")],
	[OMAP4_USB_HOST_FS_CLKCTRL, None, CLKF_SW_SUP, Some(b"func_48mc_fclk\0")],
	[OMAP4_OCP2SCP_USB_PHY_CLKCTRL, omap4_ocp2scp_usb_phy_bit_data, CLKF_HW_SUP, Some(b"l3-init-clkctrl:00c0:8\0")],
	$1,
};

static omap4_cm2_dm10_mux_parents: [Option<&'static [u8]>; 4] = [
	Some(b"sys_clkin_ck\0"), Some(b"sys_32k_ck\0"),
	None,
};

static omap4_timer10_bit_data: [omap_clkctrl_bit_data; 64] = [
	[24, TI_CLK_MUX, omap4_cm2_dm10_mux_parents, None],
	$1,
};

static omap4_timer11_bit_data: [omap_clkctrl_bit_data; 64] = [
	[24, TI_CLK_MUX, omap4_cm2_dm10_mux_parents, None],
	$1,
};

static omap4_timer2_bit_data: [omap_clkctrl_bit_data; 64] = [
	[24, TI_CLK_MUX, omap4_cm2_dm10_mux_parents, None],
	$1,
};

static omap4_timer3_bit_data: [omap_clkctrl_bit_data; 64] = [
	[24, TI_CLK_MUX, omap4_cm2_dm10_mux_parents, None],
	$1,
};

static omap4_timer4_bit_data: [omap_clkctrl_bit_data; 64] = [
	[24, TI_CLK_MUX, omap4_cm2_dm10_mux_parents, None],
	$1,
};

static omap4_timer9_bit_data: [omap_clkctrl_bit_data; 64] = [
	[24, TI_CLK_MUX, omap4_cm2_dm10_mux_parents, None],
	$1,
};

static omap4_gpio2_dbclk_parents: [Option<&'static [u8]>; 4] = [
	Some(b"sys_32k_ck\0"),
	None,
};

static omap4_gpio2_bit_data: [omap_clkctrl_bit_data; 64] = [
	[8, TI_CLK_GATE, omap4_gpio2_dbclk_parents, None],
	$1,
};

static omap4_gpio3_bit_data: [omap_clkctrl_bit_data; 64] = [
	[8, TI_CLK_GATE, omap4_gpio2_dbclk_parents, None],
	$1,
};

static omap4_gpio4_bit_data: [omap_clkctrl_bit_data; 64] = [
	[8, TI_CLK_GATE, omap4_gpio2_dbclk_parents, None],
	$1,
};

static omap4_gpio5_bit_data: [omap_clkctrl_bit_data; 64] = [
	[8, TI_CLK_GATE, omap4_gpio2_dbclk_parents, None],
	$1,
};

static omap4_gpio6_bit_data: [omap_clkctrl_bit_data; 64] = [
	[8, TI_CLK_GATE, omap4_gpio2_dbclk_parents, None],
	$1,
};

static omap4_per_mcbsp4_gfclk_parents: [Option<&'static [u8]>; 4] = [
	Some(b"l4-per-clkctrl:00c0:26\0"), Some(b"pad_clks_ck\0"),
	None,
};

static omap4_mcbsp4_sync_mux_ck_parents: [Option<&'static [u8]>; 4] = [
	Some(b"func_96m_fclk\0"), Some(b"per_abe_nc_fclk\0"),
	None,
};

static omap4_mcbsp4_bit_data: [omap_clkctrl_bit_data; 64] = [
	[24, TI_CLK_MUX, omap4_per_mcbsp4_gfclk_parents, None],
	[26, TI_CLK_MUX, omap4_mcbsp4_sync_mux_ck_parents, None],
	$1,
};

static omap4_slimbus2_fclk_0_parents: [Option<&'static [u8]>; 4] = [
	Some(b"func_24mc_fclk\0"),
	None,
};

static omap4_slimbus2_fclk_1_parents: [Option<&'static [u8]>; 4] = [
	Some(b"per_abe_24m_fclk\0"),
	None,
};

static omap4_slimbus2_slimbus_clk_parents: [Option<&'static [u8]>; 4] = [
	Some(b"pad_slimbus_core_clks_ck\0"),
	None,
};

static omap4_slimbus2_bit_data: [omap_clkctrl_bit_data; 64] = [
	[8, TI_CLK_GATE, omap4_slimbus2_fclk_0_parents, None],
	[9, TI_CLK_GATE, omap4_slimbus2_fclk_1_parents, None],
	[10, TI_CLK_GATE, omap4_slimbus2_slimbus_clk_parents, None],
	$1,
};

static omap4_l4_per_clkctrl_regs: [omap_clkctrl_reg_data; 64] = [
	[OMAP4_TIMER10_CLKCTRL, omap4_timer10_bit_data, CLKF_SW_SUP, Some(b"l4-per-clkctrl:0008:24\0")],
	[OMAP4_TIMER11_CLKCTRL, omap4_timer11_bit_data, CLKF_SW_SUP, Some(b"l4-per-clkctrl:0010:24\0")],
	[OMAP4_TIMER2_CLKCTRL, omap4_timer2_bit_data, CLKF_SW_SUP, Some(b"l4-per-clkctrl:0018:24\0")],
	[OMAP4_TIMER3_CLKCTRL, omap4_timer3_bit_data, CLKF_SW_SUP, Some(b"l4-per-clkctrl:0020:24\0")],
	[OMAP4_TIMER4_CLKCTRL, omap4_timer4_bit_data, CLKF_SW_SUP, Some(b"l4-per-clkctrl:0028:24\0")],
	[OMAP4_TIMER9_CLKCTRL, omap4_timer9_bit_data, CLKF_SW_SUP, Some(b"l4-per-clkctrl:0030:24\0")],
	[OMAP4_ELM_CLKCTRL, None, 0, Some(b"l4_div_ck\0")],
	[OMAP4_GPIO2_CLKCTRL, omap4_gpio2_bit_data, CLKF_HW_SUP, Some(b"l4_div_ck\0")],
	[OMAP4_GPIO3_CLKCTRL, omap4_gpio3_bit_data, CLKF_HW_SUP, Some(b"l4_div_ck\0")],
	[OMAP4_GPIO4_CLKCTRL, omap4_gpio4_bit_data, CLKF_HW_SUP, Some(b"l4_div_ck\0")],
	[OMAP4_GPIO5_CLKCTRL, omap4_gpio5_bit_data, CLKF_HW_SUP, Some(b"l4_div_ck\0")],
	[OMAP4_GPIO6_CLKCTRL, omap4_gpio6_bit_data, CLKF_HW_SUP, Some(b"l4_div_ck\0")],
	[OMAP4_HDQ1W_CLKCTRL, None, CLKF_SW_SUP, Some(b"func_12m_fclk\0")],
	[OMAP4_I2C1_CLKCTRL, None, CLKF_SW_SUP, Some(b"func_96m_fclk\0")],
	[OMAP4_I2C2_CLKCTRL, None, CLKF_SW_SUP, Some(b"func_96m_fclk\0")],
	[OMAP4_I2C3_CLKCTRL, None, CLKF_SW_SUP, Some(b"func_96m_fclk\0")],
	[OMAP4_I2C4_CLKCTRL, None, CLKF_SW_SUP, Some(b"func_96m_fclk\0")],
	[OMAP4_L4_PER_CLKCTRL, None, 0, Some(b"l4_div_ck\0")],
	[OMAP4_MCBSP4_CLKCTRL, omap4_mcbsp4_bit_data, CLKF_SW_SUP, Some(b"l4-per-clkctrl:00c0:24\0")],
	[OMAP4_MCSPI1_CLKCTRL, None, CLKF_SW_SUP, Some(b"func_48m_fclk\0")],
	[OMAP4_MCSPI2_CLKCTRL, None, CLKF_SW_SUP, Some(b"func_48m_fclk\0")],
	[OMAP4_MCSPI3_CLKCTRL, None, CLKF_SW_SUP, Some(b"func_48m_fclk\0")],
	[OMAP4_MCSPI4_CLKCTRL, None, CLKF_SW_SUP, Some(b"func_48m_fclk\0")],
	[OMAP4_MMC3_CLKCTRL, None, CLKF_SW_SUP, Some(b"func_48m_fclk\0")],
	[OMAP4_MMC4_CLKCTRL, None, CLKF_SW_SUP, Some(b"func_48m_fclk\0")],
	[OMAP4_SLIMBUS2_CLKCTRL, omap4_slimbus2_bit_data, CLKF_SW_SUP, Some(b"l4-per-clkctrl:0118:8\0")],
	[OMAP4_UART1_CLKCTRL, None, CLKF_SW_SUP, Some(b"func_48m_fclk\0")],
	[OMAP4_UART2_CLKCTRL, None, CLKF_SW_SUP, Some(b"func_48m_fclk\0")],
	[OMAP4_UART3_CLKCTRL, None, CLKF_SW_SUP, Some(b"func_48m_fclk\0")],
	[OMAP4_UART4_CLKCTRL, None, CLKF_SW_SUP, Some(b"func_48m_fclk\0")],
	[OMAP4_MMC5_CLKCTRL, None, CLKF_SW_SUP, Some(b"func_48m_fclk\0")],
	$1,
};

static const struct
omap_clkctrl_reg_data omap4_l4_secure_clkctrl_regs[]  = {
	[OMAP4_AES1_CLKCTRL, None, CLKF_SW_SUP, Some(b"l3_div_ck\0")],
	[OMAP4_AES2_CLKCTRL, None, CLKF_SW_SUP, Some(b"l3_div_ck\0")],
	[OMAP4_DES3DES_CLKCTRL, None, CLKF_SW_SUP, Some(b"l4_div_ck\0")],
	[OMAP4_PKA_CLKCTRL, None, CLKF_SW_SUP, Some(b"l4_div_ck\0")],
	[OMAP4_RNG_CLKCTRL, None, CLKF_HW_SUP | CLKF_SOC_NONSEC, Some(b"l4_div_ck\0")],
	[OMAP4_SHA2MD5_CLKCTRL, None, CLKF_SW_SUP, Some(b"l3_div_ck\0")],
	[OMAP4_CRYPTODMA_CLKCTRL, None, CLKF_HW_SUP | CLKF_SOC_NONSEC, Some(b"l3_div_ck\0")],
	$1,
};

static omap4_gpio1_bit_data: [omap_clkctrl_bit_data; 64] = [
	[8, TI_CLK_GATE, omap4_gpio2_dbclk_parents, None],
	$1,
};

static omap4_timer1_bit_data: [omap_clkctrl_bit_data; 64] = [
	[24, TI_CLK_MUX, omap4_cm2_dm10_mux_parents, None],
	$1,
};

static omap4_l4_wkup_clkctrl_regs: [omap_clkctrl_reg_data; 64] = [
	[OMAP4_L4_WKUP_CLKCTRL, None, 0, Some(b"l4_wkup_clk_mux_ck\0")],
	[OMAP4_WD_TIMER2_CLKCTRL, None, CLKF_SW_SUP, Some(b"sys_32k_ck\0")],
	[OMAP4_GPIO1_CLKCTRL, omap4_gpio1_bit_data, CLKF_HW_SUP, Some(b"l4_wkup_clk_mux_ck\0")],
	[OMAP4_TIMER1_CLKCTRL, omap4_timer1_bit_data, CLKF_SW_SUP, Some(b"l4-wkup-clkctrl:0020:24\0")],
	[OMAP4_COUNTER_32K_CLKCTRL, None, 0, Some(b"sys_32k_ck\0")],
	[OMAP4_KBD_CLKCTRL, None, CLKF_SW_SUP, Some(b"sys_32k_ck\0")],
	$1,
};

static omap4_pmd_stm_clock_mux_ck_parents: [Option<&'static [u8]>; 4] = [
	Some(b"sys_clkin_ck\0"), Some(b"dpll_core_m6x2_ck\0"), Some(b"tie_low_clock_ck\0"),
	None,
};

static omap4_trace_clk_div_div_ck_parents: [Option<&'static [u8]>; 4] = [
	Some(b"emu-sys-clkctrl:0000:22\0"),
	None,
};

static omap4_trace_clk_div_div_ck_divs: [i32; 6] = [
	0,
	1,
	2,
	0,
	4,
	-1,
};

static omap4_trace_clk_div_div_ck_data: omap_clkctrl_div_data = omap_clkctrl_div_data [dividers: &omap4_trace_clk_div_div_ck_divs,];

static omap4_stm_clk_div_ck_parents: [Option<&'static [u8]>; 4] = [
	Some(b"emu-sys-clkctrl:0000:20\0"),
	None,
};

static omap4_stm_clk_div_ck_data: omap_clkctrl_div_data = omap_clkctrl_div_data {
	max_div: 64,
	flags: CLK_DIVIDER_POWER_OF_TWO,
};

static omap4_debugss_bit_data: [omap_clkctrl_bit_data; 64] = [
	[20, TI_CLK_MUX, omap4_pmd_stm_clock_mux_ck_parents, None],
	[22, TI_CLK_MUX, omap4_pmd_stm_clock_mux_ck_parents, None],
	[24, TI_CLK_DIVIDER, omap4_trace_clk_div_div_ck_parents, &omap4_trace_clk_div_div_ck_data],
	[27, TI_CLK_DIVIDER, omap4_stm_clk_div_ck_parents, &omap4_stm_clk_div_ck_data],
	$1,
};

static omap4_emu_sys_clkctrl_regs: [omap_clkctrl_reg_data; 64] = [
	[OMAP4_DEBUGSS_CLKCTRL, omap4_debugss_bit_data, 0, Some(b"trace_clk_div_ck\0")],
	$1,
};

pub static omap4_clkctrl_data: [omap_clkctrl_data; 32] = [
	[0x4a004320, omap4_mpuss_clkctrl_regs],
	[0x4a004420, omap4_tesla_clkctrl_regs],
	[0x4a004520, omap4_abe_clkctrl_regs],
	[0x4a008620, omap4_l4_ao_clkctrl_regs],
	[0x4a008720, omap4_l3_1_clkctrl_regs],
	[0x4a008820, omap4_l3_2_clkctrl_regs],
	[0x4a008920, omap4_ducati_clkctrl_regs],
	[0x4a008a20, omap4_l3_dma_clkctrl_regs],
	[0x4a008b20, omap4_l3_emif_clkctrl_regs],
	[0x4a008c20, omap4_d2d_clkctrl_regs],
	[0x4a008d20, omap4_l4_cfg_clkctrl_regs],
	[0x4a008e20, omap4_l3_instr_clkctrl_regs],
	[0x4a008f20, omap4_ivahd_clkctrl_regs],
	[0x4a009020, omap4_iss_clkctrl_regs],
	[0x4a009120, omap4_l3_dss_clkctrl_regs],
	[0x4a009220, omap4_l3_gfx_clkctrl_regs],
	[0x4a009320, omap4_l3_init_clkctrl_regs],
	[0x4a009420, omap4_l4_per_clkctrl_regs],
	[0x4a0095a0, omap4_l4_secure_clkctrl_regs],
	[0x4a307820, omap4_l4_wkup_clkctrl_regs],
	[0x4a307a20, omap4_emu_sys_clkctrl_regs],
	$1,
};

static mut omap44xx_clks: [ti_dt_clk; 128] = [
	dt_clk!(None, Some(b"timer_32k_ck\0"), Some(b"sys_32k_ck\0")),
	/*
	 * XXX: All the clock aliases below are only needed for legacy
	 * hwmod support. Once hwmod is removed, these can be removed
	 * also.
	 */
	dt_clk!(None, Some(b"aess_fclk\0"), Some(b"abe-clkctrl:0008:24\0")),
	dt_clk!(None, Some(b"cm2_dm10_mux\0"), Some(b"l4-per-clkctrl:0008:24\0")),
	dt_clk!(None, Some(b"cm2_dm11_mux\0"), Some(b"l4-per-clkctrl:0010:24\0")),
	dt_clk!(None, Some(b"cm2_dm2_mux\0"), Some(b"l4-per-clkctrl:0018:24\0")),
	dt_clk!(None, Some(b"cm2_dm3_mux\0"), Some(b"l4-per-clkctrl:0020:24\0")),
	dt_clk!(None, Some(b"cm2_dm4_mux\0"), Some(b"l4-per-clkctrl:0028:24\0")),
	dt_clk!(None, Some(b"cm2_dm9_mux\0"), Some(b"l4-per-clkctrl:0030:24\0")),
	dt_clk!(None, Some(b"dmic_sync_mux_ck\0"), Some(b"abe-clkctrl:0018:26\0")),
	dt_clk!(None, Some(b"dmt1_clk_mux\0"), Some(b"l4-wkup-clkctrl:0020:24\0")),
	dt_clk!(None, Some(b"dss_48mhz_clk\0"), Some(b"l3-dss-clkctrl:0000:9\0")),
	dt_clk!(None, Some(b"dss_dss_clk\0"), Some(b"l3-dss-clkctrl:0000:8\0")),
	dt_clk!(None, Some(b"dss_sys_clk\0"), Some(b"l3-dss-clkctrl:0000:10\0")),
	dt_clk!(None, Some(b"dss_tv_clk\0"), Some(b"l3-dss-clkctrl:0000:11\0")),
	dt_clk!(None, Some(b"fdif_fck\0"), Some(b"iss-clkctrl:0008:24\0")),
	dt_clk!(None, Some(b"func_dmic_abe_gfclk\0"), Some(b"abe-clkctrl:0018:24\0")),
	dt_clk!(None, Some(b"func_mcasp_abe_gfclk\0"), Some(b"abe-clkctrl:0020:24\0")),
	dt_clk!(None, Some(b"func_mcbsp1_gfclk\0"), Some(b"abe-clkctrl:0028:24\0")),
	dt_clk!(None, Some(b"func_mcbsp2_gfclk\0"), Some(b"abe-clkctrl:0030:24\0")),
	dt_clk!(None, Some(b"func_mcbsp3_gfclk\0"), Some(b"abe-clkctrl:0038:24\0")),
	dt_clk!(None, Some(b"gpio1_dbclk\0"), Some(b"l4-wkup-clkctrl:0018:8\0")),
	dt_clk!(None, Some(b"gpio2_dbclk\0"), Some(b"l4-per-clkctrl:0040:8\0")),
	dt_clk!(None, Some(b"gpio3_dbclk\0"), Some(b"l4-per-clkctrl:0048:8\0")),
	dt_clk!(None, Some(b"gpio4_dbclk\0"), Some(b"l4-per-clkctrl:0050:8\0")),
	dt_clk!(None, Some(b"gpio5_dbclk\0"), Some(b"l4-per-clkctrl:0058:8\0")),
	dt_clk!(None, Some(b"gpio6_dbclk\0"), Some(b"l4-per-clkctrl:0060:8\0")),
	dt_clk!(None, Some(b"hsi_fck\0"), Some(b"l3-init-clkctrl:0018:24\0")),
	dt_clk!(None, Some(b"hsmmc1_fclk\0"), Some(b"l3-init-clkctrl:0008:24\0")),
	dt_clk!(None, Some(b"hsmmc2_fclk\0"), Some(b"l3-init-clkctrl:0010:24\0")),
	dt_clk!(None, Some(b"iss_ctrlclk\0"), Some(b"iss-clkctrl:0000:8\0")),
	dt_clk!(None, Some(b"mcasp_sync_mux_ck\0"), Some(b"abe-clkctrl:0020:26\0")),
	dt_clk!(None, Some(b"mcbsp1_sync_mux_ck\0"), Some(b"abe-clkctrl:0028:26\0")),
	dt_clk!(None, Some(b"mcbsp2_sync_mux_ck\0"), Some(b"abe-clkctrl:0030:26\0")),
	dt_clk!(None, Some(b"mcbsp3_sync_mux_ck\0"), Some(b"abe-clkctrl:0038:26\0")),
	dt_clk!(Some(b"40122000.mcbsp\0"), Some(b"prcm_fck\0"), Some(b"abe-clkctrl:0028:26\0")),
	dt_clk!(Some(b"40124000.mcbsp\0"), Some(b"prcm_fck\0"), Some(b"abe-clkctrl:0030:26\0")),
	dt_clk!(Some(b"40126000.mcbsp\0"), Some(b"prcm_fck\0"), Some(b"abe-clkctrl:0038:26\0")),
	dt_clk!(None, Some(b"mcbsp4_sync_mux_ck\0"), Some(b"l4-per-clkctrl:00c0:26\0")),
	dt_clk!(Some(b"48096000.mcbsp\0"), Some(b"prcm_fck\0"), Some(b"l4-per-clkctrl:00c0:26\0")),
	dt_clk!(None, Some(b"ocp2scp_usb_phy_phy_48m\0"), Some(b"l3-init-clkctrl:00c0:8\0")),
	dt_clk!(None, Some(b"otg_60m_gfclk\0"), Some(b"l3-init-clkctrl:0040:24\0")),
	dt_clk!(None, Some(b"pad_fck\0"), Some(b"pad_clks_ck\0")),
	dt_clk!(None, Some(b"per_mcbsp4_gfclk\0"), Some(b"l4-per-clkctrl:00c0:24\0")),
	dt_clk!(None, Some(b"pmd_stm_clock_mux_ck\0"), Some(b"emu-sys-clkctrl:0000:20\0")),
	dt_clk!(None, Some(b"pmd_trace_clk_mux_ck\0"), Some(b"emu-sys-clkctrl:0000:22\0")),
	dt_clk!(None, Some(b"sgx_clk_mux\0"), Some(b"l3-gfx-clkctrl:0000:24\0")),
	dt_clk!(None, Some(b"slimbus1_fclk_0\0"), Some(b"abe-clkctrl:0040:8\0")),
	dt_clk!(None, Some(b"slimbus1_fclk_1\0"), Some(b"abe-clkctrl:0040:9\0")),
	dt_clk!(None, Some(b"slimbus1_fclk_2\0"), Some(b"abe-clkctrl:0040:10\0")),
	dt_clk!(None, Some(b"slimbus1_slimbus_clk\0"), Some(b"abe-clkctrl:0040:11\0")),
	dt_clk!(None, Some(b"slimbus2_fclk_0\0"), Some(b"l4-per-clkctrl:0118:8\0")),
	dt_clk!(None, Some(b"slimbus2_fclk_1\0"), Some(b"l4-per-clkctrl:0118:9\0")),
	dt_clk!(None, Some(b"slimbus2_slimbus_clk\0"), Some(b"l4-per-clkctrl:0118:10\0")),
	dt_clk!(None, Some(b"stm_clk_div_ck\0"), Some(b"emu-sys-clkctrl:0000:27\0")),
	dt_clk!(None, Some(b"timer5_sync_mux\0"), Some(b"abe-clkctrl:0048:24\0")),
	dt_clk!(None, Some(b"timer6_sync_mux\0"), Some(b"abe-clkctrl:0050:24\0")),
	dt_clk!(None, Some(b"timer7_sync_mux\0"), Some(b"abe-clkctrl:0058:24\0")),
	dt_clk!(None, Some(b"timer8_sync_mux\0"), Some(b"abe-clkctrl:0060:24\0")),
	dt_clk!(None, Some(b"trace_clk_div_div_ck\0"), Some(b"emu-sys-clkctrl:0000:24\0")),
	dt_clk!(None, Some(b"usb_host_hs_func48mclk\0"), Some(b"l3-init-clkctrl:0038:15\0")),
	dt_clk!(None, Some(b"usb_host_hs_hsic480m_p1_clk\0"), Some(b"l3-init-clkctrl:0038:13\0")),
	dt_clk!(None, Some(b"usb_host_hs_hsic480m_p2_clk\0"), Some(b"l3-init-clkctrl:0038:14\0")),
	dt_clk!(None, Some(b"usb_host_hs_hsic60m_p1_clk\0"), Some(b"l3-init-clkctrl:0038:11\0")),
	dt_clk!(None, Some(b"usb_host_hs_hsic60m_p2_clk\0"), Some(b"l3-init-clkctrl:0038:12\0")),
	dt_clk!(None, Some(b"usb_host_hs_utmi_p1_clk\0"), Some(b"l3-init-clkctrl:0038:8\0")),
	dt_clk!(None, Some(b"usb_host_hs_utmi_p2_clk\0"), Some(b"l3-init-clkctrl:0038:9\0")),
	dt_clk!(None, Some(b"usb_host_hs_utmi_p3_clk\0"), Some(b"l3_init-clkctrl:0038:10\0")),
	dt_clk!(None, Some(b"usb_otg_hs_xclk\0"), Some(b"l3-init-clkctrl:0040:8\0")),
	dt_clk!(None, Some(b"usb_tll_hs_usb_ch0_clk\0"), Some(b"l3-init-clkctrl:0048:8\0")),
	dt_clk!(None, Some(b"usb_tll_hs_usb_ch1_clk\0"), Some(b"l3-init-clkctrl:0048:9\0")),
	dt_clk!(None, Some(b"usb_tll_hs_usb_ch2_clk\0"), Some(b"l3-init-clkctrl:0048:10\0")),
	dt_clk!(None, Some(b"utmi_p1_gfclk\0"), Some(b"l3-init-clkctrl:0038:24\0")),
	dt_clk!(None, Some(b"utmi_p2_gfclk\0"), Some(b"l3-init-clkctrl:0038:25\0")),
	DT_CLK_END,
};

int  omap4xxx_dt_clk_init(void)
{
	let mut rc: i32;
	let mut abe_dpll_ref, *abe_dpll, *sys_32k_ck, *usb_dpll: *mut clk;

	ti_dt_clocks_register(omap44xx_clks.as_ptr());

	omap2_clk_disable_autoidle_all();

	ti_clk_add_aliases();

	/*
	 * Lock USB DPLL on OMAP4 devices so that the L3INIT power
	 * domain can transition to retention state when not in use.
	 */
	usb_dpll = clk_get_sys(core::ptr::null(), b"dpll_usb_ck\0".as_ptr() as *const i8);
	rc = clk_set_rate(usb_dpll, OMAP4_DPLL_USB_DEFFREQ);
	if (rc)
		pr_err(Some(b"%s: failed to configure USB DPLL!\n\0"), __func__);

	/*
	 * On OMAP4460 the ABE DPLL fails to turn on if in idle low-power
	 * state when turning the ABE clock domain. Workaround this by
	 * locking the ABE DPLL on boot.
	 * Lock the ABE DPLL in any case to avoid issues with audio.
	 */
	abe_dpll_ref = clk_get_sys(core::ptr::null_mut(), b"abe_dpll_refclk_mux_ck\0".as_ptr() as *const i8);
	sys_32k_ck = clk_get_sys(core::ptr::null_mut(), b"sys_32k_ck\0".as_ptr() as *const i8);
	rc = clk_set_parent(abe_dpll_ref, sys_32k_ck);
	abe_dpll = clk_get_sys(core::ptr::null_mut(), b"dpll_abe_ck\0".as_ptr() as *const i8);
	if rc == 0
		rc = clk_set_rate(abe_dpll, OMAP4_DPLL_ABE_DEFFREQ);
	if (rc)
		pr_err(Some(b"%s: failed to configure ABE DPLL!\n\0"), __func__);

	return 0;
}


// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
