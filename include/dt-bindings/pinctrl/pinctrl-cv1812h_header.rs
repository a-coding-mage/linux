/* SPDX-License-Identifier: GPL-2.0-only OR BSD-2-Clause */
/*
 * Copyright (C) 2024 Inochi Amaoto <inochiama@outlook.com>
 *
 * This file is generated from vendor pinout definition.
 */

// Dependency intent: <dt-bindings/pinctrl/pinctrl-cv18xx.h>

macro_rules! PINPOS {
    ($row:expr, $col:expr) => {
        ((($row as i32 - 'A' as i32 + 1) << 8) + ($col as i32 - 1))
    };
}

pub const PIN_MIPI_TXM4: i32 = PINPOS!('A', 2);
pub const PIN_MIPIRX0N: i32 = PINPOS!('A', 4);
pub const PIN_MIPIRX3P: i32 = PINPOS!('A', 6);
pub const PIN_MIPIRX4P: i32 = PINPOS!('A', 7);
pub const PIN_VIVO_D2: i32 = PINPOS!('A', 9);
pub const PIN_VIVO_D3: i32 = PINPOS!('A', 10);
pub const PIN_VIVO_D10: i32 = PINPOS!('A', 12);
pub const PIN_USB_VBUS_DET: i32 = PINPOS!('A', 13);
pub const PIN_MIPI_TXP3: i32 = PINPOS!('B', 1);
pub const PIN_MIPI_TXM3: i32 = PINPOS!('B', 2);
pub const PIN_MIPI_TXP4: i32 = PINPOS!('B', 3);
pub const PIN_MIPIRX0P: i32 = PINPOS!('B', 4);
pub const PIN_MIPIRX1N: i32 = PINPOS!('B', 5);
pub const PIN_MIPIRX2N: i32 = PINPOS!('B', 6);
pub const PIN_MIPIRX4N: i32 = PINPOS!('B', 7);
pub const PIN_MIPIRX5N: i32 = PINPOS!('B', 8);
pub const PIN_VIVO_D1: i32 = PINPOS!('B', 9);
pub const PIN_VIVO_D5: i32 = PINPOS!('B', 10);
pub const PIN_VIVO_D7: i32 = PINPOS!('B', 11);
pub const PIN_VIVO_D9: i32 = PINPOS!('B', 12);
pub const PIN_USB_ID: i32 = PINPOS!('B', 13);
pub const PIN_ETH_RXM: i32 = PINPOS!('B', 15);
pub const PIN_MIPI_TXP2: i32 = PINPOS!('C', 1);
pub const PIN_MIPI_TXM2: i32 = PINPOS!('C', 2);
pub const PIN_CAM_PD0: i32 = PINPOS!('C', 3);
pub const PIN_CAM_MCLK0: i32 = PINPOS!('C', 4);
pub const PIN_MIPIRX1P: i32 = PINPOS!('C', 5);
pub const PIN_MIPIRX2P: i32 = PINPOS!('C', 6);
pub const PIN_MIPIRX3N: i32 = PINPOS!('C', 7);
pub const PIN_MIPIRX5P: i32 = PINPOS!('C', 8);
pub const PIN_VIVO_CLK: i32 = PINPOS!('C', 9);
pub const PIN_VIVO_D6: i32 = PINPOS!('C', 10);
pub const PIN_VIVO_D8: i32 = PINPOS!('C', 11);
pub const PIN_USB_VBUS_EN: i32 = PINPOS!('C', 12);
pub const PIN_ETH_RXP: i32 = PINPOS!('C', 14);
pub const PIN_GPIO_RTX: i32 = PINPOS!('C', 15);
pub const PIN_MIPI_TXP1: i32 = PINPOS!('D', 1);
pub const PIN_MIPI_TXM1: i32 = PINPOS!('D', 2);
pub const PIN_CAM_MCLK1: i32 = PINPOS!('D', 3);
pub const PIN_IIC3_SCL: i32 = PINPOS!('D', 4);
pub const PIN_VIVO_D4: i32 = PINPOS!('D', 10);
pub const PIN_ETH_TXM: i32 = PINPOS!('D', 14);
pub const PIN_ETH_TXP: i32 = PINPOS!('D', 15);
pub const PIN_MIPI_TXP0: i32 = PINPOS!('E', 1);
pub const PIN_MIPI_TXM0: i32 = PINPOS!('E', 2);
pub const PIN_CAM_PD1: i32 = PINPOS!('E', 4);
pub const PIN_CAM_RST0: i32 = PINPOS!('E', 5);
pub const PIN_VIVO_D0: i32 = PINPOS!('E', 10);
pub const PIN_ADC1: i32 = PINPOS!('E', 13);
pub const PIN_ADC2: i32 = PINPOS!('E', 14);
pub const PIN_ADC3: i32 = PINPOS!('E', 15);
pub const PIN_AUD_AOUTL: i32 = PINPOS!('F', 2);
pub const PIN_IIC3_SDA: i32 = PINPOS!('F', 4);
pub const PIN_SD1_D2: i32 = PINPOS!('F', 14);
pub const PIN_AUD_AOUTR: i32 = PINPOS!('G', 2);
pub const PIN_SD1_D3: i32 = PINPOS!('G', 13);
pub const PIN_SD1_CLK: i32 = PINPOS!('G', 14);
pub const PIN_SD1_CMD: i32 = PINPOS!('G', 15);
pub const PIN_AUD_AINL_MIC: i32 = PINPOS!('H', 1);
pub const PIN_RSTN: i32 = PINPOS!('H', 12);
pub const PIN_PWM0_BUCK: i32 = PINPOS!('H', 13);
pub const PIN_SD1_D1: i32 = PINPOS!('H', 14);
pub const PIN_SD1_D0: i32 = PINPOS!('H', 15);
pub const PIN_AUD_AINR_MIC: i32 = PINPOS!('J', 1);
pub const PIN_IIC2_SCL: i32 = PINPOS!('J', 13);
pub const PIN_IIC2_SDA: i32 = PINPOS!('J', 14);
pub const PIN_SD0_CD: i32 = PINPOS!('K', 2);
pub const PIN_SD0_D1: i32 = PINPOS!('K', 3);
pub const PIN_UART2_RX: i32 = PINPOS!('K', 13);
pub const PIN_UART2_CTS: i32 = PINPOS!('K', 14);
pub const PIN_UART2_TX: i32 = PINPOS!('K', 15);
pub const PIN_SD0_CLK: i32 = PINPOS!('L', 1);
pub const PIN_SD0_D0: i32 = PINPOS!('L', 2);
pub const PIN_SD0_CMD: i32 = PINPOS!('L', 3);
pub const PIN_CLK32K: i32 = PINPOS!('L', 14);
pub const PIN_UART2_RTS: i32 = PINPOS!('L', 15);
pub const PIN_SD0_D3: i32 = PINPOS!('M', 1);
pub const PIN_SD0_D2: i32 = PINPOS!('M', 2);
pub const PIN_UART0_RX: i32 = PINPOS!('M', 4);
pub const PIN_UART0_TX: i32 = PINPOS!('M', 5);
pub const PIN_JTAG_CPU_TRST: i32 = PINPOS!('M', 6);
pub const PIN_PWR_ON: i32 = PINPOS!('M', 11);
pub const PIN_PWR_GPIO2: i32 = PINPOS!('M', 12);
pub const PIN_PWR_GPIO0: i32 = PINPOS!('M', 13);
pub const PIN_CLK25M: i32 = PINPOS!('M', 14);
pub const PIN_SD0_PWR_EN: i32 = PINPOS!('N', 1);
pub const PIN_SPK_EN: i32 = PINPOS!('N', 3);
pub const PIN_JTAG_CPU_TCK: i32 = PINPOS!('N', 4);
pub const PIN_JTAG_CPU_TMS: i32 = PINPOS!('N', 6);
pub const PIN_PWR_WAKEUP1: i32 = PINPOS!('N', 11);
pub const PIN_PWR_WAKEUP0: i32 = PINPOS!('N', 12);
pub const PIN_PWR_GPIO1: i32 = PINPOS!('N', 13);
pub const PIN_EMMC_DAT3: i32 = PINPOS!('P', 1);
pub const PIN_EMMC_DAT0: i32 = PINPOS!('P', 2);
pub const PIN_EMMC_DAT2: i32 = PINPOS!('P', 3);
pub const PIN_EMMC_RSTN: i32 = PINPOS!('P', 4);
pub const PIN_AUX0: i32 = PINPOS!('P', 5);
pub const PIN_IIC0_SDA: i32 = PINPOS!('P', 6);
pub const PIN_PWR_SEQ3: i32 = PINPOS!('P', 10);
pub const PIN_PWR_VBAT_DET: i32 = PINPOS!('P', 11);
pub const PIN_PWR_SEQ1: i32 = PINPOS!('P', 12);
pub const PIN_PWR_BUTTON1: i32 = PINPOS!('P', 13);
pub const PIN_EMMC_DAT1: i32 = PINPOS!('R', 2);
pub const PIN_EMMC_CMD: i32 = PINPOS!('R', 3);
pub const PIN_EMMC_CLK: i32 = PINPOS!('R', 4);
pub const PIN_IIC0_SCL: i32 = PINPOS!('R', 6);
pub const PIN_GPIO_ZQ: i32 = PINPOS!('R', 10);
pub const PIN_PWR_RSTN: i32 = PINPOS!('R', 11);
pub const PIN_PWR_SEQ2: i32 = PINPOS!('R', 12);
pub const PIN_XTAL_XIN: i32 = PINPOS!('R', 13);

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
