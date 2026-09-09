// SPDX-License-Identifier: GPL-2.0-only
/*
 *  Copyright (C) 2008 Sascha Hauer <s.hauer@pengutronix.de>, Pengutronix
 */

// C headers and build-time declarations are supplied by the surrounding kernel bindings.

use core::ffi::{c_char, c_void};

const MX1_CCM_BASE_ADDR: usize = 0x0021b000;
const MX1_TIM1_BASE_ADDR: usize = 0x00220000;

// `NR_IRQS_LEGACY` is supplied by the architecture bindings.
const MX1_TIM1_INT: usize = NR_IRQS_LEGACY + 59;

static mut PREM_SEL_CLKS: [*const c_char; 2] = [
    c"clk32_premult".as_ptr(),
    c"clk16m".as_ptr(),
];
static mut CLKO_SEL_CLKS: [*const c_char; 6] = [
    c"per1".as_ptr(),
    c"hclk".as_ptr(),
    c"clk48m".as_ptr(),
    c"clk16m".as_ptr(),
    c"prem".as_ptr(),
    c"fclk".as_ptr(),
];

static mut CLK: [*mut clk; IMX1_CLK_MAX] = [core::ptr::null_mut(); IMX1_CLK_MAX];
static mut CLK_DATA: clk_onecell_data = clk_onecell_data {
    clks: core::ptr::null_mut(),
    clk_num: 0,
};

static mut CCM: *mut c_void = core::ptr::null_mut(); // __initdata

#[inline]
unsafe fn ccm_reg(offset: usize) -> *mut c_void {
    (CCM as *mut u8).add(offset) as *mut c_void
}

unsafe fn mx1_clocks_init_dt(np: *mut device_node) {
    CCM = of_iomap(np, 0);
    BUG_ON(CCM.is_null());

    CLK[IMX1_CLK_DUMMY] = imx_clk_fixed(c"dummy".as_ptr(), 0);
    CLK[IMX1_CLK_CLK32] = imx_obtain_fixed_clock(c"clk32".as_ptr(), 32768);
    CLK[IMX1_CLK_CLK16M_EXT] = imx_clk_fixed(c"clk16m_ext".as_ptr(), 16000000);
    CLK[IMX1_CLK_CLK16M] = imx_clk_gate(c"clk16m".as_ptr(), c"clk16m_ext".as_ptr(), ccm_reg(0x0000), 17);
    CLK[IMX1_CLK_CLK32_PREMULT] = imx_clk_fixed_factor(c"clk32_premult".as_ptr(), c"clk32".as_ptr(), 512, 1);
    CLK[IMX1_CLK_PREM] = imx_clk_mux(c"prem".as_ptr(), ccm_reg(0x0000), 16, 1, PREM_SEL_CLKS.as_ptr(), 2);
    CLK[IMX1_CLK_MPLL] = imx_clk_pllv1(IMX_PLLV1_IMX1, c"mpll".as_ptr(), c"clk32_premult".as_ptr(), ccm_reg(0x0004));
    CLK[IMX1_CLK_MPLL_GATE] = imx_clk_gate(c"mpll_gate".as_ptr(), c"mpll".as_ptr(), ccm_reg(0x0000), 0);
    CLK[IMX1_CLK_SPLL] = imx_clk_pllv1(IMX_PLLV1_IMX1, c"spll".as_ptr(), c"prem".as_ptr(), ccm_reg(0x000c));
    CLK[IMX1_CLK_SPLL_GATE] = imx_clk_gate(c"spll_gate".as_ptr(), c"spll".as_ptr(), ccm_reg(0x0000), 1);
    CLK[IMX1_CLK_MCU] = imx_clk_divider(c"mcu".as_ptr(), c"clk32_premult".as_ptr(), ccm_reg(0x0000), 15, 1);
    CLK[IMX1_CLK_FCLK] = imx_clk_divider(c"fclk".as_ptr(), c"mpll_gate".as_ptr(), ccm_reg(0x0000), 15, 1);
    CLK[IMX1_CLK_HCLK] = imx_clk_divider(c"hclk".as_ptr(), c"spll_gate".as_ptr(), ccm_reg(0x0000), 10, 4);
    CLK[IMX1_CLK_CLK48M] = imx_clk_divider(c"clk48m".as_ptr(), c"spll_gate".as_ptr(), ccm_reg(0x0000), 26, 3);
    CLK[IMX1_CLK_PER1] = imx_clk_divider(c"per1".as_ptr(), c"spll_gate".as_ptr(), ccm_reg(0x0020), 0, 4);
    CLK[IMX1_CLK_PER2] = imx_clk_divider(c"per2".as_ptr(), c"spll_gate".as_ptr(), ccm_reg(0x0020), 4, 4);
    CLK[IMX1_CLK_PER3] = imx_clk_divider(c"per3".as_ptr(), c"spll_gate".as_ptr(), ccm_reg(0x0020), 16, 7);
    CLK[IMX1_CLK_CLKO] = imx_clk_mux(c"clko".as_ptr(), ccm_reg(0x0000), 29, 3, CLKO_SEL_CLKS.as_ptr(), 6);
    CLK[IMX1_CLK_UART3_GATE] = imx_clk_gate(c"uart3_gate".as_ptr(), c"hclk".as_ptr(), ccm_reg(0x0810), 6);
    CLK[IMX1_CLK_SSI2_GATE] = imx_clk_gate(c"ssi2_gate".as_ptr(), c"hclk".as_ptr(), ccm_reg(0x0810), 5);
    CLK[IMX1_CLK_BROM_GATE] = imx_clk_gate(c"brom_gate".as_ptr(), c"hclk".as_ptr(), ccm_reg(0x0810), 4);
    CLK[IMX1_CLK_DMA_GATE] = imx_clk_gate(c"dma_gate".as_ptr(), c"hclk".as_ptr(), ccm_reg(0x0810), 3);
    CLK[IMX1_CLK_CSI_GATE] = imx_clk_gate(c"csi_gate".as_ptr(), c"hclk".as_ptr(), ccm_reg(0x0810), 2);
    CLK[IMX1_CLK_MMA_GATE] = imx_clk_gate(c"mma_gate".as_ptr(), c"hclk".as_ptr(), ccm_reg(0x0810), 1);
    CLK[IMX1_CLK_USBD_GATE] = imx_clk_gate(c"usbd_gate".as_ptr(), c"clk48m".as_ptr(), ccm_reg(0x0810), 0);

    imx_check_clocks(CLK.as_mut_ptr(), IMX1_CLK_MAX);
    CLK_DATA.clks = CLK.as_mut_ptr();
    CLK_DATA.clk_num = IMX1_CLK_MAX;
    of_clk_add_provider(np, of_clk_src_onecell_get, &mut CLK_DATA);
}

// CLK_OF_DECLARE(imx1_ccm, "fsl,imx1-ccm", mx1_clocks_init_dt);

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
