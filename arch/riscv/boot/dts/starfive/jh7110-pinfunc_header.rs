/* SPDX-License-Identifier: GPL-2.0 OR MIT */
/*
 * Copyright (C) 2022 Emil Renner Berthing <kernel@esmil.dk>
 * Copyright (C) 2022 StarFive Technology Co., Ltd.
 */

/*
 * mux bits:
 *  | 31 - 24 | 23 - 16 | 15 - 10 |  9 - 8   |  7 - 0  |
 *  |  din    |  dout   |  doen   | function | gpio nr |
 *
 * dout:     output signal
 * doen:     output enable signal
 * din:      optional input signal, 0xff = none
 * function: function selector
 * gpio nr:  gpio number, 0 - 63
 */
macro_rules! GPIOMUX {
    ($n:expr, $dout:expr, $doen:expr, $din:expr) => {
        ((($din & 0xff) << 24) | (($dout & 0xff) << 16) |
            (($doen & 0x3f) << 10) | ($n & 0x3f))
    };
}

macro_rules! PINMUX {
    ($n:expr, $func:expr) => {
        ((1 << 10) | (($func & 0x3) << 8) | ($n & 0xff))
    };
}

/* sys_iomux dout */
pub const GPOUT_LOW: u32 = 0;
pub const GPOUT_HIGH: u32 = 1;
pub const GPOUT_SYS_WAVE511_UART_TX: u32 = 2;
pub const GPOUT_SYS_CAN0_STBY: u32 = 3;
pub const GPOUT_SYS_CAN0_TST_NEXT_BIT: u32 = 4;
pub const GPOUT_SYS_CAN0_TST_SAMPLE_POINT: u32 = 5;
pub const GPOUT_SYS_CAN0_TXD: u32 = 6;
pub const GPOUT_SYS_USB_DRIVE_VBUS: u32 = 7;
pub const GPOUT_SYS_QSPI_CS1: u32 = 8;
pub const GPOUT_SYS_SPDIF: u32 = 9;
pub const GPOUT_SYS_HDMI_CEC_SDA: u32 = 10;
pub const GPOUT_SYS_HDMI_DDC_SCL: u32 = 11;
pub const GPOUT_SYS_HDMI_DDC_SDA: u32 = 12;
pub const GPOUT_SYS_WATCHDOG: u32 = 13;
pub const GPOUT_SYS_I2C0_CLK: u32 = 14;
pub const GPOUT_SYS_I2C0_DATA: u32 = 15;
pub const GPOUT_SYS_SDIO0_BACK_END_POWER: u32 = 16;
pub const GPOUT_SYS_SDIO0_CARD_POWER_EN: u32 = 17;
pub const GPOUT_SYS_SDIO0_CCMD_OD_PULLUP_EN: u32 = 18;
pub const GPOUT_SYS_SDIO0_RST: u32 = 19;
pub const GPOUT_SYS_UART0_TX: u32 = 20;
pub const GPOUT_SYS_HIFI4_JTAG_TDO: u32 = 21;
pub const GPOUT_SYS_JTAG_TDO: u32 = 22;
pub const GPOUT_SYS_PDM_MCLK: u32 = 23;
pub const GPOUT_SYS_PWM_CHANNEL0: u32 = 24;
pub const GPOUT_SYS_PWM_CHANNEL1: u32 = 25;
pub const GPOUT_SYS_PWM_CHANNEL2: u32 = 26;
pub const GPOUT_SYS_PWM_CHANNEL3: u32 = 27;
pub const GPOUT_SYS_PWMDAC_LEFT: u32 = 28;
pub const GPOUT_SYS_PWMDAC_RIGHT: u32 = 29;
pub const GPOUT_SYS_SPI0_CLK: u32 = 30;
pub const GPOUT_SYS_SPI0_FSS: u32 = 31;
pub const GPOUT_SYS_SPI0_TXD: u32 = 32;
pub const GPOUT_SYS_GMAC_PHYCLK: u32 = 33;
pub const GPOUT_SYS_I2SRX_BCLK: u32 = 34;
pub const GPOUT_SYS_I2SRX_LRCK: u32 = 35;
pub const GPOUT_SYS_I2STX0_BCLK: u32 = 36;
pub const GPOUT_SYS_I2STX0_LRCK: u32 = 37;
pub const GPOUT_SYS_MCLK: u32 = 38;
pub const GPOUT_SYS_TDM_CLK: u32 = 39;
pub const GPOUT_SYS_TDM_SYNC: u32 = 40;
pub const GPOUT_SYS_TDM_TXD: u32 = 41;
pub const GPOUT_SYS_TRACE_DATA0: u32 = 42;
pub const GPOUT_SYS_TRACE_DATA1: u32 = 43;
pub const GPOUT_SYS_TRACE_DATA2: u32 = 44;
pub const GPOUT_SYS_TRACE_DATA3: u32 = 45;
pub const GPOUT_SYS_TRACE_REF: u32 = 46;
pub const GPOUT_SYS_CAN1_STBY: u32 = 47;
pub const GPOUT_SYS_CAN1_TST_NEXT_BIT: u32 = 48;
pub const GPOUT_SYS_CAN1_TST_SAMPLE_POINT: u32 = 49;
pub const GPOUT_SYS_CAN1_TXD: u32 = 50;
pub const GPOUT_SYS_I2C1_CLK: u32 = 51;
pub const GPOUT_SYS_I2C1_DATA: u32 = 52;
pub const GPOUT_SYS_SDIO1_BACK_END_POWER: u32 = 53;
pub const GPOUT_SYS_SDIO1_CARD_POWER_EN: u32 = 54;
pub const GPOUT_SYS_SDIO1_CLK: u32 = 55;
pub const GPOUT_SYS_SDIO1_CMD_OD_PULLUP_EN: u32 = 56;
pub const GPOUT_SYS_SDIO1_CMD: u32 = 57;
pub const GPOUT_SYS_SDIO1_DATA0: u32 = 58;
pub const GPOUT_SYS_SDIO1_DATA1: u32 = 59;
pub const GPOUT_SYS_SDIO1_DATA2: u32 = 60;
pub const GPOUT_SYS_SDIO1_DATA3: u32 = 61;
pub const GPOUT_SYS_SDIO1_DATA4: u32 = 62;
pub const GPOUT_SYS_SDIO1_DATA5: u32 = 63;
pub const GPOUT_SYS_SDIO1_DATA6: u32 = 64;
pub const GPOUT_SYS_SDIO1_DATA7: u32 = 65;
pub const GPOUT_SYS_SDIO1_RST: u32 = 66;
pub const GPOUT_SYS_UART1_RTS: u32 = 67;
pub const GPOUT_SYS_UART1_TX: u32 = 68;
pub const GPOUT_SYS_I2STX1_SDO0: u32 = 69;
pub const GPOUT_SYS_I2STX1_SDO1: u32 = 70;
pub const GPOUT_SYS_I2STX1_SDO2: u32 = 71;
pub const GPOUT_SYS_I2STX1_SDO3: u32 = 72;
pub const GPOUT_SYS_SPI1_CLK: u32 = 73;
pub const GPOUT_SYS_SPI1_FSS: u32 = 74;
pub const GPOUT_SYS_SPI1_TXD: u32 = 75;
pub const GPOUT_SYS_I2C2_CLK: u32 = 76;
pub const GPOUT_SYS_I2C2_DATA: u32 = 77;
pub const GPOUT_SYS_UART2_RTS: u32 = 78;
pub const GPOUT_SYS_UART2_TX: u32 = 79;
pub const GPOUT_SYS_SPI2_CLK: u32 = 80;
pub const GPOUT_SYS_SPI2_FSS: u32 = 81;
pub const GPOUT_SYS_SPI2_TXD: u32 = 82;
pub const GPOUT_SYS_I2C3_CLK: u32 = 83;
pub const GPOUT_SYS_I2C3_DATA: u32 = 84;
pub const GPOUT_SYS_UART3_TX: u32 = 85;
pub const GPOUT_SYS_SPI3_CLK: u32 = 86;
pub const GPOUT_SYS_SPI3_FSS: u32 = 87;
pub const GPOUT_SYS_SPI3_TXD: u32 = 88;
pub const GPOUT_SYS_I2C4_CLK: u32 = 89;
pub const GPOUT_SYS_I2C4_DATA: u32 = 90;
pub const GPOUT_SYS_UART4_RTS: u32 = 91;
pub const GPOUT_SYS_UART4_TX: u32 = 92;
pub const GPOUT_SYS_SPI4_CLK: u32 = 93;
pub const GPOUT_SYS_SPI4_FSS: u32 = 94;
pub const GPOUT_SYS_SPI4_TXD: u32 = 95;
pub const GPOUT_SYS_I2C5_CLK: u32 = 96;
pub const GPOUT_SYS_I2C5_DATA: u32 = 97;
pub const GPOUT_SYS_UART5_RTS: u32 = 98;
pub const GPOUT_SYS_UART5_TX: u32 = 99;
pub const GPOUT_SYS_SPI5_CLK: u32 = 100;
pub const GPOUT_SYS_SPI5_FSS: u32 = 101;
pub const GPOUT_SYS_SPI5_TXD: u32 = 102;
pub const GPOUT_SYS_I2C6_CLK: u32 = 103;
pub const GPOUT_SYS_I2C6_DATA: u32 = 104;
pub const GPOUT_SYS_SPI6_CLK: u32 = 105;
pub const GPOUT_SYS_SPI6_FSS: u32 = 106;
pub const GPOUT_SYS_SPI6_TXD: u32 = 107;

/* aon_iomux dout */
pub const GPOUT_AON_CLK_32K_OUT: u32 = 2;
pub const GPOUT_AON_PTC0_PWM4: u32 = 3;
pub const GPOUT_AON_PTC0_PWM5: u32 = 4;
pub const GPOUT_AON_PTC0_PWM6: u32 = 5;
pub const GPOUT_AON_PTC0_PWM7: u32 = 6;
pub const GPOUT_AON_CLK_GCLK0: u32 = 7;
pub const GPOUT_AON_CLK_GCLK1: u32 = 8;
pub const GPOUT_AON_CLK_GCLK2: u32 = 9;

/* sys_iomux doen */
pub const GPOEN_ENABLE: u32 = 0;
pub const GPOEN_DISABLE: u32 = 1;
pub const GPOEN_SYS_HDMI_CEC_SDA: u32 = 2;
pub const GPOEN_SYS_HDMI_DDC_SCL: u32 = 3;
pub const GPOEN_SYS_HDMI_DDC_SDA: u32 = 4;
pub const GPOEN_SYS_I2C0_CLK: u32 = 5;
pub const GPOEN_SYS_I2C0_DATA: u32 = 6;
pub const GPOEN_SYS_HIFI4_JTAG_TDO: u32 = 7;
pub const GPOEN_SYS_JTAG_TDO: u32 = 8;
pub const GPOEN_SYS_PWM0_CHANNEL0: u32 = 9;
pub const GPOEN_SYS_PWM0_CHANNEL1: u32 = 10;
pub const GPOEN_SYS_PWM0_CHANNEL2: u32 = 11;
pub const GPOEN_SYS_PWM0_CHANNEL3: u32 = 12;
pub const GPOEN_SYS_SPI0_NSSPCTL: u32 = 13;
pub const GPOEN_SYS_SPI0_NSSP: u32 = 14;
pub const GPOEN_SYS_TDM_SYNC: u32 = 15;
pub const GPOEN_SYS_TDM_TXD: u32 = 16;
pub const GPOEN_SYS_I2C1_CLK: u32 = 17;
pub const GPOEN_SYS_I2C1_DATA: u32 = 18;
pub const GPOEN_SYS_SDIO1_CMD: u32 = 19;
pub const GPOEN_SYS_SDIO1_DATA0: u32 = 20;
pub const GPOEN_SYS_SDIO1_DATA1: u32 = 21;
pub const GPOEN_SYS_SDIO1_DATA2: u32 = 22;
pub const GPOEN_SYS_SDIO1_DATA3: u32 = 23;
pub const GPOEN_SYS_SDIO1_DATA4: u32 = 24;
pub const GPOEN_SYS_SDIO1_DATA5: u32 = 25;
pub const GPOEN_SYS_SDIO1_DATA6: u32 = 26;
pub const GPOEN_SYS_SDIO1_DATA7: u32 = 27;
pub const GPOEN_SYS_SPI1_NSSPCTL: u32 = 28;
pub const GPOEN_SYS_SPI1_NSSP: u32 = 29;
pub const GPOEN_SYS_I2C2_CLK: u32 = 30;
pub const GPOEN_SYS_I2C2_DATA: u32 = 31;
pub const GPOEN_SYS_SPI2_NSSPCTL: u32 = 32;
pub const GPOEN_SYS_SPI2_NSSP: u32 = 33;
pub const GPOEN_SYS_I2C3_CLK: u32 = 34;
pub const GPOEN_SYS_I2C3_DATA: u32 = 35;
pub const GPOEN_SYS_SPI3_NSSPCTL: u32 = 36;
pub const GPOEN_SYS_SPI3_NSSP: u32 = 37;
pub const GPOEN_SYS_I2C4_CLK: u32 = 38;
pub const GPOEN_SYS_I2C4_DATA: u32 = 39;
pub const GPOEN_SYS_SPI4_NSSPCTL: u32 = 40;
pub const GPOEN_SYS_SPI4_NSSP: u32 = 41;
pub const GPOEN_SYS_I2C5_CLK: u32 = 42;
pub const GPOEN_SYS_I2C5_DATA: u32 = 43;
pub const GPOEN_SYS_SPI5_NSSPCTL: u32 = 44;
pub const GPOEN_SYS_SPI5_NSSP: u32 = 45;
pub const GPOEN_SYS_I2C6_CLK: u32 = 46;
pub const GPOEN_SYS_I2C6_DATA: u32 = 47;
pub const GPOEN_SYS_SPI6_NSSPCTL: u32 = 48;
pub const GPOEN_SYS_SPI6_NSSP: u32 = 49;

/* aon_iomux doen */
pub const GPOEN_AON_PTC0_OE_N_4: u32 = 2;
pub const GPOEN_AON_PTC0_OE_N_5: u32 = 3;
pub const GPOEN_AON_PTC0_OE_N_6: u32 = 4;
pub const GPOEN_AON_PTC0_OE_N_7: u32 = 5;

/* sys_iomux gin */
pub const GPI_NONE: u32 = 255;

pub const GPI_SYS_WAVE511_UART_RX: u32 = 0;
pub const GPI_SYS_CAN0_RXD: u32 = 1;
pub const GPI_SYS_USB_OVERCURRENT: u32 = 2;
pub const GPI_SYS_SPDIF: u32 = 3;
pub const GPI_SYS_JTAG_RST: u32 = 4;
pub const GPI_SYS_HDMI_CEC_SDA: u32 = 5;
pub const GPI_SYS_HDMI_DDC_SCL: u32 = 6;
pub const GPI_SYS_HDMI_DDC_SDA: u32 = 7;
pub const GPI_SYS_HDMI_HPD: u32 = 8;
pub const GPI_SYS_I2C0_CLK: u32 = 9;
pub const GPI_SYS_I2C0_DATA: u32 = 10;
pub const GPI_SYS_SDIO0_CD: u32 = 11;
pub const GPI_SYS_SDIO0_INT: u32 = 12;
pub const GPI_SYS_SDIO0_WP: u32 = 13;
pub const GPI_SYS_UART0_RX: u32 = 14;
pub const GPI_SYS_HIFI4_JTAG_TCK: u32 = 15;
pub const GPI_SYS_HIFI4_JTAG_TDI: u32 = 16;
pub const GPI_SYS_HIFI4_JTAG_TMS: u32 = 17;
pub const GPI_SYS_HIFI4_JTAG_RST: u32 = 18;
pub const GPI_SYS_JTAG_TDI: u32 = 19;
pub const GPI_SYS_JTAG_TMS: u32 = 20;
pub const GPI_SYS_PDM_DMIC0: u32 = 21;
pub const GPI_SYS_PDM_DMIC1: u32 = 22;
pub const GPI_SYS_I2SRX_SDIN0: u32 = 23;
pub const GPI_SYS_I2SRX_SDIN1: u32 = 24;
pub const GPI_SYS_I2SRX_SDIN2: u32 = 25;
pub const GPI_SYS_SPI0_CLK: u32 = 26;
pub const GPI_SYS_SPI0_FSS: u32 = 27;
pub const GPI_SYS_SPI0_RXD: u32 = 28;
pub const GPI_SYS_JTAG_TCK: u32 = 29;
pub const GPI_SYS_MCLK_EXT: u32 = 30;
pub const GPI_SYS_I2SRX_BCLK: u32 = 31;
pub const GPI_SYS_I2SRX_LRCK: u32 = 32;
pub const GPI_SYS_I2STX1_BCLK: u32 = 33;
pub const GPI_SYS_I2STX1_LRCK: u32 = 34;
pub const GPI_SYS_TDM_CLK: u32 = 35;
pub const GPI_SYS_TDM_RXD: u32 = 36;
pub const GPI_SYS_TDM_SYNC: u32 = 37;
pub const GPI_SYS_CAN1_RXD: u32 = 38;
pub const GPI_SYS_I2C1_CLK: u32 = 39;
pub const GPI_SYS_I2C1_DATA: u32 = 40;
pub const GPI_SYS_SDIO1_CD: u32 = 41;
pub const GPI_SYS_SDIO1_INT: u32 = 42;
pub const GPI_SYS_SDIO1_WP: u32 = 43;
pub const GPI_SYS_SDIO1_CMD: u32 = 44;
pub const GPI_SYS_SDIO1_DATA0: u32 = 45;
pub const GPI_SYS_SDIO1_DATA1: u32 = 46;
pub const GPI_SYS_SDIO1_DATA2: u32 = 47;
pub const GPI_SYS_SDIO1_DATA3: u32 = 48;
pub const GPI_SYS_SDIO1_DATA4: u32 = 49;
pub const GPI_SYS_SDIO1_DATA5: u32 = 50;
pub const GPI_SYS_SDIO1_DATA6: u32 = 51;
pub const GPI_SYS_SDIO1_DATA7: u32 = 52;
pub const GPI_SYS_SDIO1_STRB: u32 = 53;
pub const GPI_SYS_UART1_CTS: u32 = 54;
pub const GPI_SYS_UART1_RX: u32 = 55;
pub const GPI_SYS_SPI1_CLK: u32 = 56;
pub const GPI_SYS_SPI1_FSS: u32 = 57;
pub const GPI_SYS_SPI1_RXD: u32 = 58;
pub const GPI_SYS_I2C2_CLK: u32 = 59;
pub const GPI_SYS_I2C2_DATA: u32 = 60;
pub const GPI_SYS_UART2_CTS: u32 = 61;
pub const GPI_SYS_UART2_RX: u32 = 62;
pub const GPI_SYS_SPI2_CLK: u32 = 63;
pub const GPI_SYS_SPI2_FSS: u32 = 64;
pub const GPI_SYS_SPI2_RXD: u32 = 65;
pub const GPI_SYS_I2C3_CLK: u32 = 66;
pub const GPI_SYS_I2C3_DATA: u32 = 67;
pub const GPI_SYS_UART3_RX: u32 = 68;
pub const GPI_SYS_SPI3_CLK: u32 = 69;
pub const GPI_SYS_SPI3_FSS: u32 = 70;
pub const GPI_SYS_SPI3_RXD: u32 = 71;
pub const GPI_SYS_I2C4_CLK: u32 = 72;
pub const GPI_SYS_I2C4_DATA: u32 = 73;
pub const GPI_SYS_UART4_CTS: u32 = 74;
pub const GPI_SYS_UART4_RX: u32 = 75;
pub const GPI_SYS_SPI4_CLK: u32 = 76;
pub const GPI_SYS_SPI4_FSS: u32 = 77;
pub const GPI_SYS_SPI4_RXD: u32 = 78;
pub const GPI_SYS_I2C5_CLK: u32 = 79;
pub const GPI_SYS_I2C5_DATA: u32 = 80;
pub const GPI_SYS_UART5_CTS: u32 = 81;
pub const GPI_SYS_UART5_RX: u32 = 82;
pub const GPI_SYS_SPI5_CLK: u32 = 83;
pub const GPI_SYS_SPI5_FSS: u32 = 84;
pub const GPI_SYS_SPI5_RXD: u32 = 85;
pub const GPI_SYS_I2C6_CLK: u32 = 86;
pub const GPI_SYS_I2C6_DATA: u32 = 87;
pub const GPI_SYS_SPI6_CLK: u32 = 88;
pub const GPI_SYS_SPI6_FSS: u32 = 89;
pub const GPI_SYS_SPI6_RXD: u32 = 90;

/* aon_iomux gin */
pub const GPI_AON_PMU_GPIO_WAKEUP_0: u32 = 0;
pub const GPI_AON_PMU_GPIO_WAKEUP_1: u32 = 1;
pub const GPI_AON_PMU_GPIO_WAKEUP_2: u32 = 2;
pub const GPI_AON_PMU_GPIO_WAKEUP_3: u32 = 3;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
