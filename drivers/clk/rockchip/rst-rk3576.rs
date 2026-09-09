// Translated from rst-rk3576.c.
// External bindings (including reset IDs and register constants) are supplied by other files.

extern "C" {
    fn RK3576_SOFTRST_CON(index: usize) -> usize;
}

const ROCKCHIP_SOFTRST_HIWORD_MASK: u32 = 1;

// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * Copyright (c) 2021 Rockchip Electronics Co., Ltd.
 * Copyright (c) 2024 Collabora Ltd.
 * Author: Detlev Casanova <detlev.casanova@collabora.com>
 * Based on Sebastien Reichel's implementation for RK3588
 */


/* 0x27200000 + 0x0A00 */
/* 0x27208000 + 0x0A00 */
/* 0x27210000 + 0x0A00 */
/* 0x27220000 + 0x0A00 */

/* mapping table for reset ID to register offset */
const fn reset_offset(id: usize, reg: usize, bit: usize) -> i32 { let _ = id; (reg * 16 + bit) as i32 }
const fn phpcru_reset_offset(id: usize, reg: usize, bit: usize) -> i32 { let _ = id; (0x8000 * 4 + reg * 16 + bit) as i32 }
const fn securenscru_reset_offset(id: usize, reg: usize, bit: usize) -> i32 { let _ = id; (0x10000 * 4 + reg * 16 + bit) as i32 }
const fn pmu1cru_reset_offset(id: usize, reg: usize, bit: usize) -> i32 { let _ = id; (0x20000 * 4 + reg * 16 + bit) as i32 }

static rk3576_register_offset: &[i32] = &[
    /* SOFTRST_CON01 */
    reset_offset(SRST_A_TOP_BIU, 1, 3),
    reset_offset(SRST_P_TOP_BIU, 1, 5),
    reset_offset(SRST_A_TOP_MID_BIU, 1, 6),
    reset_offset(SRST_A_SECURE_HIGH_BIU, 1, 7),
    reset_offset(SRST_H_TOP_BIU, 1, 14),

    /* SOFTRST_CON02 */
    reset_offset(SRST_H_VO0VOP_CHANNEL_BIU, 2, 0),
    reset_offset(SRST_A_VO0VOP_CHANNEL_BIU, 2, 1),

    /* SOFTRST_CON06 */
    reset_offset(SRST_BISRINTF, 6, 2),

    /* SOFTRST_CON07 */
    reset_offset(SRST_H_AUDIO_BIU, 7, 2),
    reset_offset(SRST_H_ASRC_2CH_0, 7, 3),
    reset_offset(SRST_H_ASRC_2CH_1, 7, 4),
    reset_offset(SRST_H_ASRC_4CH_0, 7, 5),
    reset_offset(SRST_H_ASRC_4CH_1, 7, 6),
    reset_offset(SRST_ASRC_2CH_0, 7, 7),
    reset_offset(SRST_ASRC_2CH_1, 7, 8),
    reset_offset(SRST_ASRC_4CH_0, 7, 9),
    reset_offset(SRST_ASRC_4CH_1, 7, 10),
    reset_offset(SRST_M_SAI0_8CH, 7, 12),
    reset_offset(SRST_H_SAI0_8CH, 7, 13),
    reset_offset(SRST_H_SPDIF_RX0, 7, 14),
    reset_offset(SRST_M_SPDIF_RX0, 7, 15),

    /* SOFTRST_CON08 */
    reset_offset(SRST_H_SPDIF_RX1, 8, 0),
    reset_offset(SRST_M_SPDIF_RX1, 8, 1),
    reset_offset(SRST_M_SAI1_8CH, 8, 5),
    reset_offset(SRST_H_SAI1_8CH, 8, 6),
    reset_offset(SRST_M_SAI2_2CH, 8, 8),
    reset_offset(SRST_H_SAI2_2CH, 8, 10),
    reset_offset(SRST_M_SAI3_2CH, 8, 12),
    reset_offset(SRST_H_SAI3_2CH, 8, 14),

    /* SOFTRST_CON09 */
    reset_offset(SRST_M_SAI4_2CH, 9, 0),
    reset_offset(SRST_H_SAI4_2CH, 9, 2),
    reset_offset(SRST_H_ACDCDIG_DSM, 9, 3),
    reset_offset(SRST_M_ACDCDIG_DSM, 9, 4),
    reset_offset(SRST_PDM1, 9, 5),
    reset_offset(SRST_H_PDM1, 9, 7),
    reset_offset(SRST_M_PDM1, 9, 8),
    reset_offset(SRST_H_SPDIF_TX0, 9, 9),
    reset_offset(SRST_M_SPDIF_TX0, 9, 10),
    reset_offset(SRST_H_SPDIF_TX1, 9, 11),
    reset_offset(SRST_M_SPDIF_TX1, 9, 12),

    /* SOFTRST_CON11 */
    reset_offset(SRST_A_BUS_BIU, 11, 3),
    reset_offset(SRST_P_BUS_BIU, 11, 4),
    reset_offset(SRST_P_CRU, 11, 5),
    reset_offset(SRST_H_CAN0, 11, 6),
    reset_offset(SRST_CAN0, 11, 7),
    reset_offset(SRST_H_CAN1, 11, 8),
    reset_offset(SRST_CAN1, 11, 9),
    reset_offset(SRST_P_INTMUX2BUS, 11, 12),
    reset_offset(SRST_P_VCCIO_IOC, 11, 13),
    reset_offset(SRST_H_BUS_BIU, 11, 14),
    reset_offset(SRST_KEY_SHIFT, 11, 15),

    /* SOFTRST_CON12 */
    reset_offset(SRST_P_I2C1, 12, 0),
    reset_offset(SRST_P_I2C2, 12, 1),
    reset_offset(SRST_P_I2C3, 12, 2),
    reset_offset(SRST_P_I2C4, 12, 3),
    reset_offset(SRST_P_I2C5, 12, 4),
    reset_offset(SRST_P_I2C6, 12, 5),
    reset_offset(SRST_P_I2C7, 12, 6),
    reset_offset(SRST_P_I2C8, 12, 7),
    reset_offset(SRST_P_I2C9, 12, 8),
    reset_offset(SRST_P_WDT_BUSMCU, 12, 9),
    reset_offset(SRST_T_WDT_BUSMCU, 12, 10),
    reset_offset(SRST_A_GIC, 12, 11),
    reset_offset(SRST_I2C1, 12, 12),
    reset_offset(SRST_I2C2, 12, 13),
    reset_offset(SRST_I2C3, 12, 14),
    reset_offset(SRST_I2C4, 12, 15),

    /* SOFTRST_CON13 */
    reset_offset(SRST_I2C5, 13, 0),
    reset_offset(SRST_I2C6, 13, 1),
    reset_offset(SRST_I2C7, 13, 2),
    reset_offset(SRST_I2C8, 13, 3),
    reset_offset(SRST_I2C9, 13, 4),
    reset_offset(SRST_P_SARADC, 13, 6),
    reset_offset(SRST_SARADC, 13, 7),
    reset_offset(SRST_P_TSADC, 13, 8),
    reset_offset(SRST_TSADC, 13, 9),
    reset_offset(SRST_P_UART0, 13, 10),
    reset_offset(SRST_P_UART2, 13, 11),
    reset_offset(SRST_P_UART3, 13, 12),
    reset_offset(SRST_P_UART4, 13, 13),
    reset_offset(SRST_P_UART5, 13, 14),
    reset_offset(SRST_P_UART6, 13, 15),

    /* SOFTRST_CON14 */
    reset_offset(SRST_P_UART7, 14, 0),
    reset_offset(SRST_P_UART8, 14, 1),
    reset_offset(SRST_P_UART9, 14, 2),
    reset_offset(SRST_P_UART10, 14, 3),
    reset_offset(SRST_P_UART11, 14, 4),
    reset_offset(SRST_S_UART0, 14, 5),
    reset_offset(SRST_S_UART2, 14, 6),
    reset_offset(SRST_S_UART3, 14, 9),
    reset_offset(SRST_S_UART4, 14, 12),
    reset_offset(SRST_S_UART5, 14, 15),

    /* SOFTRST_CON15 */
    reset_offset(SRST_S_UART6, 15, 2),
    reset_offset(SRST_S_UART7, 15, 5),
    reset_offset(SRST_S_UART8, 15, 8),
    reset_offset(SRST_S_UART9, 15, 9),
    reset_offset(SRST_S_UART10, 15, 10),
    reset_offset(SRST_S_UART11, 15, 11),
    reset_offset(SRST_P_SPI0, 15, 13),
    reset_offset(SRST_P_SPI1, 15, 14),
    reset_offset(SRST_P_SPI2, 15, 15),

    /* SOFTRST_CON16 */
    reset_offset(SRST_P_SPI3, 16, 0),
    reset_offset(SRST_P_SPI4, 16, 1),
    reset_offset(SRST_SPI0, 16, 2),
    reset_offset(SRST_SPI1, 16, 3),
    reset_offset(SRST_SPI2, 16, 4),
    reset_offset(SRST_SPI3, 16, 5),
    reset_offset(SRST_SPI4, 16, 6),
    reset_offset(SRST_P_WDT0, 16, 7),
    reset_offset(SRST_T_WDT0, 16, 8),
    reset_offset(SRST_P_SYS_GRF, 16, 9),
    reset_offset(SRST_P_PWM1, 16, 10),
    reset_offset(SRST_PWM1, 16, 11),

    /* SOFTRST_CON17 */
    reset_offset(SRST_P_BUSTIMER0, 17, 3),
    reset_offset(SRST_P_BUSTIMER1, 17, 4),
    reset_offset(SRST_TIMER0, 17, 6),
    reset_offset(SRST_TIMER1, 17, 7),
    reset_offset(SRST_TIMER2, 17, 8),
    reset_offset(SRST_TIMER3, 17, 9),
    reset_offset(SRST_TIMER4, 17, 10),
    reset_offset(SRST_TIMER5, 17, 11),
    reset_offset(SRST_P_BUSIOC, 17, 12),
    reset_offset(SRST_P_MAILBOX0, 17, 13),
    reset_offset(SRST_P_GPIO1, 17, 15),

    /* SOFTRST_CON18 */
    reset_offset(SRST_GPIO1, 18, 0),
    reset_offset(SRST_P_GPIO2, 18, 1),
    reset_offset(SRST_GPIO2, 18, 2),
    reset_offset(SRST_P_GPIO3, 18, 3),
    reset_offset(SRST_GPIO3, 18, 4),
    reset_offset(SRST_P_GPIO4, 18, 5),
    reset_offset(SRST_GPIO4, 18, 6),
    reset_offset(SRST_A_DECOM, 18, 7),
    reset_offset(SRST_P_DECOM, 18, 8),
    reset_offset(SRST_D_DECOM, 18, 9),
    reset_offset(SRST_TIMER6, 18, 11),
    reset_offset(SRST_TIMER7, 18, 12),
    reset_offset(SRST_TIMER8, 18, 13),
    reset_offset(SRST_TIMER9, 18, 14),
    reset_offset(SRST_TIMER10, 18, 15),

    /* SOFTRST_CON19 */
    reset_offset(SRST_TIMER11, 19, 0),
    reset_offset(SRST_A_DMAC0, 19, 1),
    reset_offset(SRST_A_DMAC1, 19, 2),
    reset_offset(SRST_A_DMAC2, 19, 3),
    reset_offset(SRST_A_SPINLOCK, 19, 4),
    reset_offset(SRST_REF_PVTPLL_BUS, 19, 5),
    reset_offset(SRST_H_I3C0, 19, 7),
    reset_offset(SRST_H_I3C1, 19, 9),
    reset_offset(SRST_H_BUS_CM0_BIU, 19, 11),
    reset_offset(SRST_F_BUS_CM0_CORE, 19, 12),
    reset_offset(SRST_T_BUS_CM0_JTAG, 19, 13),

    /* SOFTRST_CON20 */
    reset_offset(SRST_P_INTMUX2PMU, 20, 0),
    reset_offset(SRST_P_INTMUX2DDR, 20, 1),
    reset_offset(SRST_P_PVTPLL_BUS, 20, 3),
    reset_offset(SRST_P_PWM2, 20, 4),
    reset_offset(SRST_PWM2, 20, 5),
    reset_offset(SRST_FREQ_PWM1, 20, 8),
    reset_offset(SRST_COUNTER_PWM1, 20, 9),
    reset_offset(SRST_I3C0, 20, 12),
    reset_offset(SRST_I3C1, 20, 13),

    /* SOFTRST_CON21 */
    reset_offset(SRST_P_DDR_MON_CH0, 21, 1),
    reset_offset(SRST_P_DDR_BIU, 21, 2),
    reset_offset(SRST_P_DDR_UPCTL_CH0, 21, 3),
    reset_offset(SRST_TM_DDR_MON_CH0, 21, 4),
    reset_offset(SRST_A_DDR_BIU, 21, 5),
    reset_offset(SRST_DFI_CH0, 21, 6),
    reset_offset(SRST_DDR_MON_CH0, 21, 10),
    reset_offset(SRST_P_DDR_HWLP_CH0, 21, 13),
    reset_offset(SRST_P_DDR_MON_CH1, 21, 14),
    reset_offset(SRST_P_DDR_HWLP_CH1, 21, 15),

    /* SOFTRST_CON22 */
    reset_offset(SRST_P_DDR_UPCTL_CH1, 22, 0),
    reset_offset(SRST_TM_DDR_MON_CH1, 22, 1),
    reset_offset(SRST_DFI_CH1, 22, 2),
    reset_offset(SRST_A_DDR01_MSCH0, 22, 3),
    reset_offset(SRST_A_DDR01_MSCH1, 22, 4),
    reset_offset(SRST_DDR_MON_CH1, 22, 6),
    reset_offset(SRST_DDR_SCRAMBLE_CH0, 22, 9),
    reset_offset(SRST_DDR_SCRAMBLE_CH1, 22, 10),
    reset_offset(SRST_P_AHB2APB, 22, 12),
    reset_offset(SRST_H_AHB2APB, 22, 13),
    reset_offset(SRST_H_DDR_BIU, 22, 14),
    reset_offset(SRST_F_DDR_CM0_CORE, 22, 15),

    /* SOFTRST_CON23 */
    reset_offset(SRST_P_DDR01_MSCH0, 23, 1),
    reset_offset(SRST_P_DDR01_MSCH1, 23, 2),
    reset_offset(SRST_DDR_TIMER0, 23, 4),
    reset_offset(SRST_DDR_TIMER1, 23, 5),
    reset_offset(SRST_T_WDT_DDR, 23, 6),
    reset_offset(SRST_P_WDT, 23, 7),
    reset_offset(SRST_P_TIMER, 23, 8),
    reset_offset(SRST_T_DDR_CM0_JTAG, 23, 9),
    reset_offset(SRST_P_DDR_GRF, 23, 11),

    /* SOFTRST_CON25 */
    reset_offset(SRST_DDR_UPCTL_CH0, 25, 1),
    reset_offset(SRST_A_DDR_UPCTL_0_CH0, 25, 2),
    reset_offset(SRST_A_DDR_UPCTL_1_CH0, 25, 3),
    reset_offset(SRST_A_DDR_UPCTL_2_CH0, 25, 4),
    reset_offset(SRST_A_DDR_UPCTL_3_CH0, 25, 5),
    reset_offset(SRST_A_DDR_UPCTL_4_CH0, 25, 6),

    /* SOFTRST_CON26 */
    reset_offset(SRST_DDR_UPCTL_CH1, 26, 1),
    reset_offset(SRST_A_DDR_UPCTL_0_CH1, 26, 2),
    reset_offset(SRST_A_DDR_UPCTL_1_CH1, 26, 3),
    reset_offset(SRST_A_DDR_UPCTL_2_CH1, 26, 4),
    reset_offset(SRST_A_DDR_UPCTL_3_CH1, 26, 5),
    reset_offset(SRST_A_DDR_UPCTL_4_CH1, 26, 6),

    /* SOFTRST_CON27 */
    reset_offset(SRST_REF_PVTPLL_DDR, 27, 0),
    reset_offset(SRST_P_PVTPLL_DDR, 27, 1),

    /* SOFTRST_CON28 */
    reset_offset(SRST_A_RKNN0, 28, 9),
    reset_offset(SRST_A_RKNN0_BIU, 28, 11),
    reset_offset(SRST_L_RKNN0_BIU, 28, 12),

    /* SOFTRST_CON29 */
    reset_offset(SRST_A_RKNN1, 29, 0),
    reset_offset(SRST_A_RKNN1_BIU, 29, 2),
    reset_offset(SRST_L_RKNN1_BIU, 29, 3),

    /* SOFTRST_CON31 */
    reset_offset(SRST_NPU_DAP, 31, 0),
    reset_offset(SRST_L_NPUSUBSYS_BIU, 31, 1),
    reset_offset(SRST_P_NPUTOP_BIU, 31, 9),
    reset_offset(SRST_P_NPU_TIMER, 31, 10),
    reset_offset(SRST_NPUTIMER0, 31, 12),
    reset_offset(SRST_NPUTIMER1, 31, 13),
    reset_offset(SRST_P_NPU_WDT, 31, 14),
    reset_offset(SRST_T_NPU_WDT, 31, 15),

    /* SOFTRST_CON32 */
    reset_offset(SRST_A_RKNN_CBUF, 32, 0),
    reset_offset(SRST_A_RVCORE0, 32, 1),
    reset_offset(SRST_P_NPU_GRF, 32, 2),
    reset_offset(SRST_P_PVTPLL_NPU, 32, 3),
    reset_offset(SRST_NPU_PVTPLL, 32, 4),
    reset_offset(SRST_H_NPU_CM0_BIU, 32, 6),
    reset_offset(SRST_F_NPU_CM0_CORE, 32, 7),
    reset_offset(SRST_T_NPU_CM0_JTAG, 32, 8),
    reset_offset(SRST_A_RKNNTOP_BIU, 32, 11),
    reset_offset(SRST_H_RKNN_CBUF, 32, 12),
    reset_offset(SRST_H_RKNNTOP_BIU, 32, 13),

    /* SOFTRST_CON33 */
    reset_offset(SRST_H_NVM_BIU, 33, 2),
    reset_offset(SRST_A_NVM_BIU, 33, 3),
    reset_offset(SRST_S_FSPI, 33, 6),
    reset_offset(SRST_H_FSPI, 33, 7),
    reset_offset(SRST_C_EMMC, 33, 8),
    reset_offset(SRST_H_EMMC, 33, 9),
    reset_offset(SRST_A_EMMC, 33, 10),
    reset_offset(SRST_B_EMMC, 33, 11),
    reset_offset(SRST_T_EMMC, 33, 12),

    /* SOFTRST_CON34 */
    reset_offset(SRST_P_GRF, 34, 1),
    reset_offset(SRST_P_PHP_BIU, 34, 5),
    reset_offset(SRST_A_PHP_BIU, 34, 9),
    reset_offset(SRST_P_PCIE0, 34, 13),
    reset_offset(SRST_PCIE0_POWER_UP, 34, 15),

    /* SOFTRST_CON35 */
    reset_offset(SRST_A_USB3OTG1, 35, 3),
    reset_offset(SRST_A_MMU0, 35, 11),
    reset_offset(SRST_A_SLV_MMU0, 35, 13),
    reset_offset(SRST_A_MMU1, 35, 14),

    /* SOFTRST_CON36 */
    reset_offset(SRST_A_SLV_MMU1, 36, 0),
    reset_offset(SRST_P_PCIE1, 36, 7),
    reset_offset(SRST_PCIE1_POWER_UP, 36, 9),

    /* SOFTRST_CON37 */
    reset_offset(SRST_RXOOB0, 37, 0),
    reset_offset(SRST_RXOOB1, 37, 1),
    reset_offset(SRST_PMALIVE0, 37, 2),
    reset_offset(SRST_PMALIVE1, 37, 3),
    reset_offset(SRST_A_SATA0, 37, 4),
    reset_offset(SRST_A_SATA1, 37, 5),
    reset_offset(SRST_ASIC1, 37, 6),
    reset_offset(SRST_ASIC0, 37, 7),

    /* SOFTRST_CON40 */
    reset_offset(SRST_P_CSIDPHY1, 40, 2),
    reset_offset(SRST_SCAN_CSIDPHY1, 40, 3),

    /* SOFTRST_CON42 */
    reset_offset(SRST_P_SDGMAC_GRF, 42, 3),
    reset_offset(SRST_P_SDGMAC_BIU, 42, 4),
    reset_offset(SRST_A_SDGMAC_BIU, 42, 5),
    reset_offset(SRST_H_SDGMAC_BIU, 42, 6),
    reset_offset(SRST_A_GMAC0, 42, 7),
    reset_offset(SRST_A_GMAC1, 42, 8),
    reset_offset(SRST_P_GMAC0, 42, 9),
    reset_offset(SRST_P_GMAC1, 42, 10),
    reset_offset(SRST_H_SDIO, 42, 12),

    /* SOFTRST_CON43 */
    reset_offset(SRST_H_SDMMC0, 43, 2),
    reset_offset(SRST_S_FSPI1, 43, 3),
    reset_offset(SRST_H_FSPI1, 43, 4),
    reset_offset(SRST_A_DSMC_BIU, 43, 6),
    reset_offset(SRST_A_DSMC, 43, 7),
    reset_offset(SRST_P_DSMC, 43, 8),
    reset_offset(SRST_H_HSGPIO, 43, 10),
    reset_offset(SRST_HSGPIO, 43, 11),
    reset_offset(SRST_A_HSGPIO, 43, 13),

    /* SOFTRST_CON45 */
    reset_offset(SRST_H_RKVDEC, 45, 3),
    reset_offset(SRST_H_RKVDEC_BIU, 45, 5),
    reset_offset(SRST_A_RKVDEC_BIU, 45, 6),
    reset_offset(SRST_RKVDEC_HEVC_CA, 45, 8),
    reset_offset(SRST_RKVDEC_CORE, 45, 9),

    /* SOFTRST_CON47 */
    reset_offset(SRST_A_USB_BIU, 47, 3),
    reset_offset(SRST_P_USBUFS_BIU, 47, 4),
    reset_offset(SRST_A_USB3OTG0, 47, 5),
    reset_offset(SRST_A_UFS_BIU, 47, 10),
    reset_offset(SRST_A_MMU2, 47, 12),
    reset_offset(SRST_A_SLV_MMU2, 47, 13),
    reset_offset(SRST_A_UFS_SYS, 47, 15),

    /* SOFTRST_CON48 */
    reset_offset(SRST_A_UFS, 48, 0),
    reset_offset(SRST_P_USBUFS_GRF, 48, 1),
    reset_offset(SRST_P_UFS_GRF, 48, 2),

    /* SOFTRST_CON49 */
    reset_offset(SRST_H_VPU_BIU, 49, 6),
    reset_offset(SRST_A_JPEG_BIU, 49, 7),
    reset_offset(SRST_A_RGA_BIU, 49, 10),
    reset_offset(SRST_A_VDPP_BIU, 49, 11),
    reset_offset(SRST_A_EBC_BIU, 49, 12),
    reset_offset(SRST_H_RGA2E_0, 49, 13),
    reset_offset(SRST_A_RGA2E_0, 49, 14),
    reset_offset(SRST_CORE_RGA2E_0, 49, 15),

    /* SOFTRST_CON50 */
    reset_offset(SRST_A_JPEG, 50, 0),
    reset_offset(SRST_H_JPEG, 50, 1),
    reset_offset(SRST_H_VDPP, 50, 2),
    reset_offset(SRST_A_VDPP, 50, 3),
    reset_offset(SRST_CORE_VDPP, 50, 4),
    reset_offset(SRST_H_RGA2E_1, 50, 5),
    reset_offset(SRST_A_RGA2E_1, 50, 6),
    reset_offset(SRST_CORE_RGA2E_1, 50, 7),
    reset_offset(SRST_H_EBC, 50, 10),
    reset_offset(SRST_A_EBC, 50, 11),
    reset_offset(SRST_D_EBC, 50, 12),

    /* SOFTRST_CON51 */
    reset_offset(SRST_H_VEPU0_BIU, 51, 2),
    reset_offset(SRST_A_VEPU0_BIU, 51, 3),
    reset_offset(SRST_H_VEPU0, 51, 4),
    reset_offset(SRST_A_VEPU0, 51, 5),
    reset_offset(SRST_VEPU0_CORE, 51, 6),

    /* SOFTRST_CON53 */
    reset_offset(SRST_A_VI_BIU, 53, 3),
    reset_offset(SRST_H_VI_BIU, 53, 4),
    reset_offset(SRST_P_VI_BIU, 53, 5),
    reset_offset(SRST_D_VICAP, 53, 6),
    reset_offset(SRST_A_VICAP, 53, 7),
    reset_offset(SRST_H_VICAP, 53, 8),
    reset_offset(SRST_ISP0, 53, 10),
    reset_offset(SRST_ISP0_VICAP, 53, 11),

    /* SOFTRST_CON54 */
    reset_offset(SRST_CORE_VPSS, 54, 1),
    reset_offset(SRST_P_CSI_HOST_0, 54, 4),
    reset_offset(SRST_P_CSI_HOST_1, 54, 5),
    reset_offset(SRST_P_CSI_HOST_2, 54, 6),
    reset_offset(SRST_P_CSI_HOST_3, 54, 7),
    reset_offset(SRST_P_CSI_HOST_4, 54, 8),

    /* SOFTRST_CON59 */
    reset_offset(SRST_CIFIN, 59, 0),
    reset_offset(SRST_VICAP_I0CLK, 59, 1),
    reset_offset(SRST_VICAP_I1CLK, 59, 2),
    reset_offset(SRST_VICAP_I2CLK, 59, 3),
    reset_offset(SRST_VICAP_I3CLK, 59, 4),
    reset_offset(SRST_VICAP_I4CLK, 59, 5),

    /* SOFTRST_CON61 */
    reset_offset(SRST_A_VOP_BIU, 61, 4),
    reset_offset(SRST_A_VOP2_BIU, 61, 5),
    reset_offset(SRST_H_VOP_BIU, 61, 6),
    reset_offset(SRST_P_VOP_BIU, 61, 7),
    reset_offset(SRST_H_VOP, 61, 8),
    reset_offset(SRST_A_VOP, 61, 9),
    reset_offset(SRST_D_VP0, 61, 13),

    /* SOFTRST_CON62 */
    reset_offset(SRST_D_VP1, 62, 0),
    reset_offset(SRST_D_VP2, 62, 1),
    reset_offset(SRST_P_VOP2_BIU, 62, 2),
    reset_offset(SRST_P_VOPGRF, 62, 3),

    /* SOFTRST_CON63 */
    reset_offset(SRST_H_VO0_BIU, 63, 5),
    reset_offset(SRST_P_VO0_BIU, 63, 7),
    reset_offset(SRST_A_HDCP0_BIU, 63, 9),
    reset_offset(SRST_P_VO0_GRF, 63, 10),
    reset_offset(SRST_A_HDCP0, 63, 12),
    reset_offset(SRST_H_HDCP0, 63, 13),
    reset_offset(SRST_HDCP0, 63, 14),

    /* SOFTRST_CON64 */
    reset_offset(SRST_P_DSIHOST0, 64, 5),
    reset_offset(SRST_DSIHOST0, 64, 6),
    reset_offset(SRST_P_HDMITX0, 64, 7),
    reset_offset(SRST_HDMITX0_REF, 64, 9),
    reset_offset(SRST_P_EDP0, 64, 13),
    reset_offset(SRST_EDP0_24M, 64, 14),

    /* SOFTRST_CON65 */
    reset_offset(SRST_M_SAI5_8CH, 65, 4),
    reset_offset(SRST_H_SAI5_8CH, 65, 5),
    reset_offset(SRST_M_SAI6_8CH, 65, 8),
    reset_offset(SRST_H_SAI6_8CH, 65, 9),
    reset_offset(SRST_H_SPDIF_TX2, 65, 10),
    reset_offset(SRST_M_SPDIF_TX2, 65, 13),
    reset_offset(SRST_H_SPDIF_RX2, 65, 14),
    reset_offset(SRST_M_SPDIF_RX2, 65, 15),

    /* SOFTRST_CON66 */
    reset_offset(SRST_H_SAI8_8CH, 66, 0),
    reset_offset(SRST_M_SAI8_8CH, 66, 2),

    /* SOFTRST_CON67 */
    reset_offset(SRST_H_VO1_BIU, 67, 5),
    reset_offset(SRST_P_VO1_BIU, 67, 6),
    reset_offset(SRST_M_SAI7_8CH, 67, 9),
    reset_offset(SRST_H_SAI7_8CH, 67, 10),
    reset_offset(SRST_H_SPDIF_TX3, 67, 11),
    reset_offset(SRST_H_SPDIF_TX4, 67, 12),
    reset_offset(SRST_H_SPDIF_TX5, 67, 13),
    reset_offset(SRST_M_SPDIF_TX3, 67, 14),

    /* SOFTRST_CON68 */
    reset_offset(SRST_DP0, 68, 0),
    reset_offset(SRST_P_VO1_GRF, 68, 2),
    reset_offset(SRST_A_HDCP1_BIU, 68, 3),
    reset_offset(SRST_A_HDCP1, 68, 4),
    reset_offset(SRST_H_HDCP1, 68, 5),
    reset_offset(SRST_HDCP1, 68, 6),
    reset_offset(SRST_H_SAI9_8CH, 68, 9),
    reset_offset(SRST_M_SAI9_8CH, 68, 11),
    reset_offset(SRST_M_SPDIF_TX4, 68, 12),
    reset_offset(SRST_M_SPDIF_TX5, 68, 13),

    /* SOFTRST_CON69 */
    reset_offset(SRST_GPU, 69, 3),
    reset_offset(SRST_A_S_GPU_BIU, 69, 6),
    reset_offset(SRST_A_M0_GPU_BIU, 69, 7),
    reset_offset(SRST_P_GPU_BIU, 69, 9),
    reset_offset(SRST_P_GPU_GRF, 69, 13),
    reset_offset(SRST_GPU_PVTPLL, 69, 14),
    reset_offset(SRST_P_PVTPLL_GPU, 69, 15),

    /* SOFTRST_CON72 */
    reset_offset(SRST_A_CENTER_BIU, 72, 4),
    reset_offset(SRST_A_DMA2DDR, 72, 5),
    reset_offset(SRST_A_DDR_SHAREMEM, 72, 6),
    reset_offset(SRST_A_DDR_SHAREMEM_BIU, 72, 7),
    reset_offset(SRST_H_CENTER_BIU, 72, 8),
    reset_offset(SRST_P_CENTER_GRF, 72, 9),
    reset_offset(SRST_P_DMA2DDR, 72, 10),
    reset_offset(SRST_P_SHAREMEM, 72, 11),
    reset_offset(SRST_P_CENTER_BIU, 72, 12),

    /* SOFTRST_CON75 */
    reset_offset(SRST_LINKSYM_HDMITXPHY0, 75, 1),

    /* SOFTRST_CON78 */
    reset_offset(SRST_DP0_PIXELCLK, 78, 1),
    reset_offset(SRST_PHY_DP0_TX, 78, 2),
    reset_offset(SRST_DP1_PIXELCLK, 78, 3),
    reset_offset(SRST_DP2_PIXELCLK, 78, 4),

    /* SOFTRST_CON79 */
    reset_offset(SRST_H_VEPU1_BIU, 79, 1),
    reset_offset(SRST_A_VEPU1_BIU, 79, 2),
    reset_offset(SRST_H_VEPU1, 79, 3),
    reset_offset(SRST_A_VEPU1, 79, 4),
    reset_offset(SRST_VEPU1_CORE, 79, 5),

    /* PPLL_SOFTRST_CON00 */
    phpcru_reset_offset(SRST_P_PHPPHY_CRU, 0, 1),
    phpcru_reset_offset(SRST_P_APB2ASB_SLV_CHIP_TOP, 0, 3),
    phpcru_reset_offset(SRST_P_PCIE2_COMBOPHY0, 0, 5),
    phpcru_reset_offset(SRST_P_PCIE2_COMBOPHY0_GRF, 0, 6),
    phpcru_reset_offset(SRST_P_PCIE2_COMBOPHY1, 0, 7),
    phpcru_reset_offset(SRST_P_PCIE2_COMBOPHY1_GRF, 0, 8),

    /* PPLL_SOFTRST_CON01 */
    phpcru_reset_offset(SRST_PCIE0_PIPE_PHY, 1, 5),
    phpcru_reset_offset(SRST_PCIE1_PIPE_PHY, 1, 8),

    /* SECURENS_SOFTRST_CON00 */
    securenscru_reset_offset(SRST_H_CRYPTO_NS, 0, 3),
    securenscru_reset_offset(SRST_H_TRNG_NS, 0, 4),
    securenscru_reset_offset(SRST_P_OTPC_NS, 0, 8),
    securenscru_reset_offset(SRST_OTPC_NS, 0, 9),

    /* PMU1_SOFTRST_CON00 */
    pmu1cru_reset_offset(SRST_P_HDPTX_GRF, 0, 0),
    pmu1cru_reset_offset(SRST_P_HDPTX_APB, 0, 1),
    pmu1cru_reset_offset(SRST_P_MIPI_DCPHY, 0, 2),
    pmu1cru_reset_offset(SRST_P_DCPHY_GRF, 0, 3),
    pmu1cru_reset_offset(SRST_P_BOT0_APB2ASB, 0, 4),
    pmu1cru_reset_offset(SRST_P_BOT1_APB2ASB, 0, 5),
    pmu1cru_reset_offset(SRST_USB2DEBUG, 0, 6),
    pmu1cru_reset_offset(SRST_P_CSIPHY_GRF, 0, 7),
    pmu1cru_reset_offset(SRST_P_CSIPHY, 0, 8),
    pmu1cru_reset_offset(SRST_P_USBPHY_GRF_0, 0, 9),
    pmu1cru_reset_offset(SRST_P_USBPHY_GRF_1, 0, 10),
    pmu1cru_reset_offset(SRST_P_USBDP_GRF, 0, 11),
    pmu1cru_reset_offset(SRST_P_USBDPPHY, 0, 12),
    pmu1cru_reset_offset(SRST_USBDP_COMBO_PHY_INIT, 0, 15),

    /* PMU1_SOFTRST_CON01 */
    pmu1cru_reset_offset(SRST_USBDP_COMBO_PHY_CMN, 1, 0),
    pmu1cru_reset_offset(SRST_USBDP_COMBO_PHY_LANE, 1, 1),
    pmu1cru_reset_offset(SRST_USBDP_COMBO_PHY_PCS, 1, 2),
    pmu1cru_reset_offset(SRST_M_MIPI_DCPHY, 1, 3),
    pmu1cru_reset_offset(SRST_S_MIPI_DCPHY, 1, 4),
    pmu1cru_reset_offset(SRST_SCAN_CSIPHY, 1, 5),
    pmu1cru_reset_offset(SRST_P_VCCIO6_IOC, 1, 6),
    pmu1cru_reset_offset(SRST_OTGPHY_0, 1, 7),
    pmu1cru_reset_offset(SRST_OTGPHY_1, 1, 8),
    pmu1cru_reset_offset(SRST_HDPTX_INIT, 1, 9),
    pmu1cru_reset_offset(SRST_HDPTX_CMN, 1, 10),
    pmu1cru_reset_offset(SRST_HDPTX_LANE, 1, 11),
    pmu1cru_reset_offset(SRST_HDMITXHDP, 1, 13),

    /* PMU1_SOFTRST_CON02 */
    pmu1cru_reset_offset(SRST_MPHY_INIT, 2, 0),
    pmu1cru_reset_offset(SRST_P_MPHY_GRF, 2, 1),
    pmu1cru_reset_offset(SRST_P_VCCIO7_IOC, 2, 3),

    /* PMU1_SOFTRST_CON03 */
    pmu1cru_reset_offset(SRST_H_PMU1_BIU, 3, 9),
    pmu1cru_reset_offset(SRST_P_PMU1_NIU, 3, 10),
    pmu1cru_reset_offset(SRST_H_PMU_CM0_BIU, 3, 11),
    pmu1cru_reset_offset(SRST_PMU_CM0_CORE, 3, 12),
    pmu1cru_reset_offset(SRST_PMU_CM0_JTAG, 3, 13),

    /* PMU1_SOFTRST_CON04 */
    pmu1cru_reset_offset(SRST_P_CRU_PMU1, 4, 1),
    pmu1cru_reset_offset(SRST_P_PMU1_GRF, 4, 3),
    pmu1cru_reset_offset(SRST_P_PMU1_IOC, 4, 4),
    pmu1cru_reset_offset(SRST_P_PMU1WDT, 4, 5),
    pmu1cru_reset_offset(SRST_T_PMU1WDT, 4, 6),
    pmu1cru_reset_offset(SRST_P_PMUTIMER, 4, 7),
    pmu1cru_reset_offset(SRST_PMUTIMER0, 4, 9),
    pmu1cru_reset_offset(SRST_PMUTIMER1, 4, 10),
    pmu1cru_reset_offset(SRST_P_PMU1PWM, 4, 11),
    pmu1cru_reset_offset(SRST_PMU1PWM, 4, 12),

    /* PMU1_SOFTRST_CON05 */
    pmu1cru_reset_offset(SRST_P_I2C0, 5, 1),
    pmu1cru_reset_offset(SRST_I2C0, 5, 2),
    pmu1cru_reset_offset(SRST_S_UART1, 5, 5),
    pmu1cru_reset_offset(SRST_P_UART1, 5, 6),
    pmu1cru_reset_offset(SRST_PDM0, 5, 13),
    pmu1cru_reset_offset(SRST_H_PDM0, 5, 15),

    /* PMU1_SOFTRST_CON06 */
    pmu1cru_reset_offset(SRST_M_PDM0, 6, 0),
    pmu1cru_reset_offset(SRST_H_VAD, 6, 1),

    /* PMU1_SOFTRST_CON07 */
    pmu1cru_reset_offset(SRST_P_PMU0GRF, 7, 4),
    pmu1cru_reset_offset(SRST_P_PMU0IOC, 7, 5),
    pmu1cru_reset_offset(SRST_P_GPIO0, 7, 6),
    pmu1cru_reset_offset(SRST_DB_GPIO0, 7, 7),
];

extern "C" {
    fn rockchip_register_softrst_lut(
        np: *mut device_node,
        offsets: *const i32,
        count: usize,
        reg_base: *mut core::ffi::c_void,
        flags: u32,
    );
}

#[repr(C)]
pub struct device_node {
    _private: [u8; 0],
}

pub unsafe fn rk3576_rst_init(np: *mut device_node, reg_base: *mut core::ffi::c_void) {
    rockchip_register_softrst_lut(
        np,
        rk3576_register_offset.as_ptr(),
        rk3576_register_offset.len(),
        reg_base.add(RK3576_SOFTRST_CON(0) as usize),
        ROCKCHIP_SOFTRST_HIWORD_MASK,
    );
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
