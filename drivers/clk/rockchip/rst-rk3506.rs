// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * Copyright (c) 2025 Rockchip Electronics Co., Ltd.
 * Author: Finley Xiao <finley.xiao@rock-chips.com>
 */

// Dependencies supplied by the surrounding kernel/Rust integration.

#[allow(non_camel_case_types)]
pub struct device_node {
    _private: [u8; 0],
}

extern "C" {
    fn rockchip_register_softrst_lut(
        np: *mut device_node,
        lut: *const i32,
        nr_resets: usize,
        reg_base: *mut core::ffi::c_void,
        flags: u32,
    );
}

// Build-time constants supplied by dt-bindings/reset/rockchip,rk3506-cru.h.
// The reset identifiers are intentionally referenced as external constants.
extern "C" {
    static RK3506_SOFTRST_CON_0: usize;
    static ROCKCHIP_SOFTRST_HIWORD_MASK: u32;
}

#[inline]
const fn rk3506_cru_reset_offset(id: usize, reg: i32, bit: i32) -> i32 {
    let _ = id;
    reg * 16 + bit
}

/* mapping table for reset ID to register offset */
#[allow(non_upper_case_globals)]
pub static rk3506_register_offset: &[i32] = &[
    /* CRU-->SOFTRST_CON00 */
    rk3506_cru_reset_offset(SRST_NCOREPORESET0_AC, 0, 0),
    rk3506_cru_reset_offset(SRST_NCOREPORESET1_AC, 0, 1),
    rk3506_cru_reset_offset(SRST_NCOREPORESET2_AC, 0, 2),
    rk3506_cru_reset_offset(SRST_NCORESET0_AC, 0, 4),
    rk3506_cru_reset_offset(SRST_NCORESET1_AC, 0, 5),
    rk3506_cru_reset_offset(SRST_NCORESET2_AC, 0, 6),
    rk3506_cru_reset_offset(SRST_NL2RESET_AC, 0, 8),
    rk3506_cru_reset_offset(SRST_A_CORE_BIU_AC, 0, 9),
    rk3506_cru_reset_offset(SRST_H_M0_AC, 0, 10),
    /* CRU-->SOFTRST_CON02 */
    rk3506_cru_reset_offset(SRST_NDBGRESET, 2, 10),
    rk3506_cru_reset_offset(SRST_P_CORE_BIU, 2, 14),
    rk3506_cru_reset_offset(SRST_PMU, 2, 15),
    /* CRU-->SOFTRST_CON03 */
    rk3506_cru_reset_offset(SRST_P_DBG, 3, 1),
    rk3506_cru_reset_offset(SRST_POT_DBG, 3, 2),
    rk3506_cru_reset_offset(SRST_P_CORE_GRF, 3, 4),
    rk3506_cru_reset_offset(SRST_CORE_EMA_DETECT, 3, 6),
    rk3506_cru_reset_offset(SRST_REF_PVTPLL_CORE, 3, 7),
    rk3506_cru_reset_offset(SRST_P_GPIO1, 3, 8),
    rk3506_cru_reset_offset(SRST_DB_GPIO1, 3, 9),
    /* CRU-->SOFTRST_CON04 */
    rk3506_cru_reset_offset(SRST_A_CORE_PERI_BIU, 4, 3),
    rk3506_cru_reset_offset(SRST_A_DSMC, 4, 5),
    rk3506_cru_reset_offset(SRST_P_DSMC, 4, 6),
    rk3506_cru_reset_offset(SRST_FLEXBUS, 4, 7),
    rk3506_cru_reset_offset(SRST_A_FLEXBUS, 4, 9),
    rk3506_cru_reset_offset(SRST_H_FLEXBUS, 4, 10),
    rk3506_cru_reset_offset(SRST_A_DSMC_SLV, 4, 11),
    rk3506_cru_reset_offset(SRST_H_DSMC_SLV, 4, 12),
    rk3506_cru_reset_offset(SRST_DSMC_SLV, 4, 13),
    /* CRU-->SOFTRST_CON05 */
    rk3506_cru_reset_offset(SRST_A_BUS_BIU, 5, 3),
    rk3506_cru_reset_offset(SRST_H_BUS_BIU, 5, 4),
    rk3506_cru_reset_offset(SRST_P_BUS_BIU, 5, 5),
    rk3506_cru_reset_offset(SRST_A_SYSRAM, 5, 6),
    rk3506_cru_reset_offset(SRST_H_SYSRAM, 5, 7),
    rk3506_cru_reset_offset(SRST_A_DMAC0, 5, 8),
    rk3506_cru_reset_offset(SRST_A_DMAC1, 5, 9),
    rk3506_cru_reset_offset(SRST_H_M0, 5, 10),
    rk3506_cru_reset_offset(SRST_M0_JTAG, 5, 11),
    rk3506_cru_reset_offset(SRST_H_CRYPTO, 5, 15),
    /* CRU-->SOFTRST_CON06 */
    rk3506_cru_reset_offset(SRST_H_RNG, 6, 0), rk3506_cru_reset_offset(SRST_P_BUS_GRF, 6, 1),
    rk3506_cru_reset_offset(SRST_P_TIMER0, 6, 2), rk3506_cru_reset_offset(SRST_TIMER0_CH0, 6, 3),
    rk3506_cru_reset_offset(SRST_TIMER0_CH1, 6, 4), rk3506_cru_reset_offset(SRST_TIMER0_CH2, 6, 5),
    rk3506_cru_reset_offset(SRST_TIMER0_CH3, 6, 6), rk3506_cru_reset_offset(SRST_TIMER0_CH4, 6, 7),
    rk3506_cru_reset_offset(SRST_TIMER0_CH5, 6, 8), rk3506_cru_reset_offset(SRST_P_WDT0, 6, 9),
    rk3506_cru_reset_offset(SRST_T_WDT0, 6, 10), rk3506_cru_reset_offset(SRST_P_WDT1, 6, 11),
    rk3506_cru_reset_offset(SRST_T_WDT1, 6, 12), rk3506_cru_reset_offset(SRST_P_MAILBOX, 6, 13),
    rk3506_cru_reset_offset(SRST_P_INTMUX, 6, 14), rk3506_cru_reset_offset(SRST_P_SPINLOCK, 6, 15),
    /* CRU-->SOFTRST_CON07 */
    rk3506_cru_reset_offset(SRST_P_DDRC, 7, 0), rk3506_cru_reset_offset(SRST_H_DDRPHY, 7, 1),
    rk3506_cru_reset_offset(SRST_P_DDRMON, 7, 2), rk3506_cru_reset_offset(SRST_DDRMON_OSC, 7, 3),
    rk3506_cru_reset_offset(SRST_P_DDR_LPC, 7, 4), rk3506_cru_reset_offset(SRST_H_USBOTG0, 7, 5),
    rk3506_cru_reset_offset(SRST_USBOTG0_ADP, 7, 7), rk3506_cru_reset_offset(SRST_H_USBOTG1, 7, 8),
    rk3506_cru_reset_offset(SRST_USBOTG1_ADP, 7, 10), rk3506_cru_reset_offset(SRST_P_USBPHY, 7, 11),
    rk3506_cru_reset_offset(SRST_USBPHY_POR, 7, 12), rk3506_cru_reset_offset(SRST_USBPHY_OTG0, 7, 13),
    rk3506_cru_reset_offset(SRST_USBPHY_OTG1, 7, 14),
    /* CRU-->SOFTRST_CON08 */ rk3506_cru_reset_offset(SRST_A_DMA2DDR, 8, 0), rk3506_cru_reset_offset(SRST_P_DMA2DDR, 8, 1),
    /* CRU-->SOFTRST_CON09 */ rk3506_cru_reset_offset(SRST_USBOTG0_UTMI, 9, 0), rk3506_cru_reset_offset(SRST_USBOTG1_UTMI, 9, 1),
    /* CRU-->SOFTRST_CON10 */ rk3506_cru_reset_offset(SRST_A_DDRC_0, 10, 0), rk3506_cru_reset_offset(SRST_A_DDRC_1, 10, 1), rk3506_cru_reset_offset(SRST_A_DDR_BIU, 10, 2), rk3506_cru_reset_offset(SRST_DDRC, 10, 3), rk3506_cru_reset_offset(SRST_DDRMON, 10, 4),
    /* CRU-->SOFTRST_CON11 */ rk3506_cru_reset_offset(SRST_H_LSPERI_BIU, 11, 2), rk3506_cru_reset_offset(SRST_P_UART0, 11, 4), rk3506_cru_reset_offset(SRST_P_UART1, 11, 5), rk3506_cru_reset_offset(SRST_P_UART2, 11, 6), rk3506_cru_reset_offset(SRST_P_UART3, 11, 7), rk3506_cru_reset_offset(SRST_P_UART4, 11, 8), rk3506_cru_reset_offset(SRST_UART0, 11, 9), rk3506_cru_reset_offset(SRST_UART1, 11, 10), rk3506_cru_reset_offset(SRST_UART2, 11, 11), rk3506_cru_reset_offset(SRST_UART3, 11, 12), rk3506_cru_reset_offset(SRST_UART4, 11, 13), rk3506_cru_reset_offset(SRST_P_I2C0, 11, 14), rk3506_cru_reset_offset(SRST_I2C0, 11, 15),
    /* CRU-->SOFTRST_CON12 */ rk3506_cru_reset_offset(SRST_P_I2C1, 12, 0), rk3506_cru_reset_offset(SRST_I2C1, 12, 1), rk3506_cru_reset_offset(SRST_P_I2C2, 12, 2), rk3506_cru_reset_offset(SRST_I2C2, 12, 3), rk3506_cru_reset_offset(SRST_P_PWM1, 12, 4), rk3506_cru_reset_offset(SRST_PWM1, 12, 5), rk3506_cru_reset_offset(SRST_P_SPI0, 12, 10), rk3506_cru_reset_offset(SRST_SPI0, 12, 11), rk3506_cru_reset_offset(SRST_P_SPI1, 12, 12), rk3506_cru_reset_offset(SRST_SPI1, 12, 13), rk3506_cru_reset_offset(SRST_P_GPIO2, 12, 14), rk3506_cru_reset_offset(SRST_DB_GPIO2, 12, 15),
    /* CRU-->SOFTRST_CON13 */ rk3506_cru_reset_offset(SRST_P_GPIO3, 13, 0), rk3506_cru_reset_offset(SRST_DB_GPIO3, 13, 1), rk3506_cru_reset_offset(SRST_P_GPIO4, 13, 2), rk3506_cru_reset_offset(SRST_DB_GPIO4, 13, 3), rk3506_cru_reset_offset(SRST_H_CAN0, 13, 4), rk3506_cru_reset_offset(SRST_CAN0, 13, 5), rk3506_cru_reset_offset(SRST_H_CAN1, 13, 6), rk3506_cru_reset_offset(SRST_CAN1, 13, 7), rk3506_cru_reset_offset(SRST_H_PDM, 13, 8), rk3506_cru_reset_offset(SRST_M_PDM, 13, 9), rk3506_cru_reset_offset(SRST_PDM, 13, 10), rk3506_cru_reset_offset(SRST_SPDIFTX, 13, 11), rk3506_cru_reset_offset(SRST_H_SPDIFTX, 13, 12), rk3506_cru_reset_offset(SRST_H_SPDIFRX, 13, 13), rk3506_cru_reset_offset(SRST_SPDIFRX, 13, 14), rk3506_cru_reset_offset(SRST_M_SAI0, 13, 15),
    /* CRU-->SOFTRST_CON14 */ rk3506_cru_reset_offset(SRST_H_SAI0, 14, 0), rk3506_cru_reset_offset(SRST_M_SAI1, 14, 2), rk3506_cru_reset_offset(SRST_H_SAI1, 14, 3), rk3506_cru_reset_offset(SRST_H_ASRC0, 14, 5), rk3506_cru_reset_offset(SRST_ASRC0, 14, 6), rk3506_cru_reset_offset(SRST_H_ASRC1, 14, 7), rk3506_cru_reset_offset(SRST_ASRC1, 14, 8),
    /* CRU-->SOFTRST_CON17 */ rk3506_cru_reset_offset(SRST_H_HSPERI_BIU, 17, 4), rk3506_cru_reset_offset(SRST_H_SDMMC, 17, 7), rk3506_cru_reset_offset(SRST_H_FSPI, 17, 8), rk3506_cru_reset_offset(SRST_S_FSPI, 17, 9), rk3506_cru_reset_offset(SRST_P_SPI2, 17, 10), rk3506_cru_reset_offset(SRST_A_MAC0, 17, 11), rk3506_cru_reset_offset(SRST_A_MAC1, 17, 12),
    /* CRU-->SOFTRST_CON18 */ rk3506_cru_reset_offset(SRST_M_SAI2, 18, 2), rk3506_cru_reset_offset(SRST_H_SAI2, 18, 3), rk3506_cru_reset_offset(SRST_H_SAI3, 18, 6), rk3506_cru_reset_offset(SRST_M_SAI3, 18, 7), rk3506_cru_reset_offset(SRST_H_SAI4, 18, 10), rk3506_cru_reset_offset(SRST_M_SAI4, 18, 11), rk3506_cru_reset_offset(SRST_H_DSM, 18, 12), rk3506_cru_reset_offset(SRST_M_DSM, 18, 13), rk3506_cru_reset_offset(SRST_P_AUDIO_ADC, 18, 14), rk3506_cru_reset_offset(SRST_M_AUDIO_ADC, 18, 15),
    /* CRU-->SOFTRST_CON19 */ rk3506_cru_reset_offset(SRST_P_SARADC, 19, 0), rk3506_cru_reset_offset(SRST_SARADC, 19, 1), rk3506_cru_reset_offset(SRST_SARADC_PHY, 19, 2), rk3506_cru_reset_offset(SRST_P_OTPC_NS, 19, 3), rk3506_cru_reset_offset(SRST_SBPI_OTPC_NS, 19, 4), rk3506_cru_reset_offset(SRST_USER_OTPC_NS, 19, 5), rk3506_cru_reset_offset(SRST_P_UART5, 19, 6), rk3506_cru_reset_offset(SRST_UART5, 19, 7), rk3506_cru_reset_offset(SRST_P_GPIO234_IOC, 19, 8),
    /* CRU-->SOFTRST_CON21 */ rk3506_cru_reset_offset(SRST_A_VIO_BIU, 21, 3), rk3506_cru_reset_offset(SRST_H_VIO_BIU, 21, 4), rk3506_cru_reset_offset(SRST_H_RGA, 21, 6), rk3506_cru_reset_offset(SRST_A_RGA, 21, 7), rk3506_cru_reset_offset(SRST_CORE_RGA, 21, 8), rk3506_cru_reset_offset(SRST_A_VOP, 21, 9), rk3506_cru_reset_offset(SRST_H_VOP, 21, 10), rk3506_cru_reset_offset(SRST_VOP, 21, 11), rk3506_cru_reset_offset(SRST_P_DPHY, 21, 12), rk3506_cru_reset_offset(SRST_P_DSI_HOST, 21, 13), rk3506_cru_reset_offset(SRST_P_TSADC, 21, 14), rk3506_cru_reset_offset(SRST_TSADC, 21, 15),
    /* CRU-->SOFTRST_CON22 */ rk3506_cru_reset_offset(SRST_P_GPIO1_IOC, 22, 1),
];

pub unsafe fn rk3506_rst_init(np: *mut device_node, reg_base: *mut core::ffi::c_void) {
    rockchip_register_softrst_lut(
        np,
        rk3506_register_offset.as_ptr(),
        rk3506_register_offset.len(),
        reg_base.add(RK3506_SOFTRST_CON_0),
        ROCKCHIP_SOFTRST_HIWORD_MASK,
    );
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
