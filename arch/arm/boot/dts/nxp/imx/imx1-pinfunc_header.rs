/* SPDX-License-Identifier: GPL-2.0-or-later */
/*
 * Copyright (C) 2014 Alexander Shiyan <shc_work@mail.ru>
 */

// Header guard omitted in Rust.
// Header guard omitted in Rust.

/*
 * The pin function ID is a tuple of
 * <pin mux_id>
 * mux_id consists of
 * function + (direction << 2) + (gpio_oconf << 4) + (gpio_iconfa << 8) + (gpio_iconfb << 10)
 *
 * function:      0 - Primary function
 *                1 - Alternate function
 *                2 - GPIO
 * direction:     0 - Input
 *                1 - Output
 * gpio_oconf:    0 - A_IN
 *                1 - B_IN
 *                2 - A_OUT
 *                3 - Data Register
 * gpio_iconfa/b: 0 - GPIO_IN
 *                1 - Interrupt Status Register
 *                2 - 0
 *                3 - 1
 *
 * 'pin' is an integer between 0 and 0xbf. i.MX1 has 4 ports with 32
 * configurable pins each. 'pin' is PORT * 32 + PORT_PIN, PORT_PIN is
 * the pin number on the specific port (between 0 and 31).
 */

#define MX1_PAD_A24__A24			0x00 0x004
pub const MX1_PAD_A24__GPIO1_0: (u32, u32) = (0x00, 0x032);
pub const MX1_PAD_A24__SPI2_CLK: (u32, u32) = (0x00, 0x006);
pub const MX1_PAD_TIN__TIN: (u32, u32) = (0x01, 0x000);
pub const MX1_PAD_TIN__GPIO1_1: (u32, u32) = (0x01, 0x032);
pub const MX1_PAD_TIN__SPI2_RXD: (u32, u32) = (0x01, 0x022);
pub const MX1_PAD_PWMO__PWMO: (u32, u32) = (0x02, 0x004);
pub const MX1_PAD_PWMO__GPIO1_2: (u32, u32) = (0x02, 0x032);
pub const MX1_PAD_CSI_MCLK__CSI_MCLK: (u32, u32) = (0x03, 0x004);
pub const MX1_PAD_CSI_MCLK__GPIO1_3: (u32, u32) = (0x03, 0x032);
pub const MX1_PAD_CSI_D0__CSI_D0: (u32, u32) = (0x04, 0x000);
pub const MX1_PAD_CSI_D0__GPIO1_4: (u32, u32) = (0x04, 0x032);
pub const MX1_PAD_CSI_D1__CSI_D1: (u32, u32) = (0x05, 0x000);
pub const MX1_PAD_CSI_D1__GPIO1_5: (u32, u32) = (0x05, 0x032);
pub const MX1_PAD_CSI_D2__CSI_D2: (u32, u32) = (0x06, 0x000);
pub const MX1_PAD_CSI_D2__GPIO1_6: (u32, u32) = (0x06, 0x032);
pub const MX1_PAD_CSI_D3__CSI_D3: (u32, u32) = (0x07, 0x000);
pub const MX1_PAD_CSI_D3__GPIO1_7: (u32, u32) = (0x07, 0x032);
pub const MX1_PAD_CSI_D4__CSI_D4: (u32, u32) = (0x08, 0x000);
pub const MX1_PAD_CSI_D4__GPIO1_8: (u32, u32) = (0x08, 0x032);
pub const MX1_PAD_CSI_D5__CSI_D5: (u32, u32) = (0x09, 0x000);
pub const MX1_PAD_CSI_D5__GPIO1_9: (u32, u32) = (0x09, 0x032);
pub const MX1_PAD_CSI_D6__CSI_D6: (u32, u32) = (0x0a, 0x000);
pub const MX1_PAD_CSI_D6__GPIO1_10: (u32, u32) = (0x0a, 0x032);
pub const MX1_PAD_CSI_D7__CSI_D7: (u32, u32) = (0x0b, 0x000);
pub const MX1_PAD_CSI_D7__GPIO1_11: (u32, u32) = (0x0b, 0x032);
pub const MX1_PAD_CSI_VSYNC__CSI_VSYNC: (u32, u32) = (0x0c, 0x000);
pub const MX1_PAD_CSI_VSYNC__GPIO1_12: (u32, u32) = (0x0c, 0x032);
pub const MX1_PAD_CSI_HSYNC__CSI_HSYNC: (u32, u32) = (0x0d, 0x000);
pub const MX1_PAD_CSI_HSYNC__GPIO1_13: (u32, u32) = (0x0d, 0x032);
pub const MX1_PAD_CSI_PIXCLK__CSI_PIXCLK: (u32, u32) = (0x0e, 0x000);
pub const MX1_PAD_CSI_PIXCLK__GPIO1_14: (u32, u32) = (0x0e, 0x032);
pub const MX1_PAD_I2C_SDA__I2C_SDA: (u32, u32) = (0x0f, 0x000);
pub const MX1_PAD_I2C_SDA__GPIO1_15: (u32, u32) = (0x0f, 0x032);
pub const MX1_PAD_I2C_SCL__I2C_SCL: (u32, u32) = (0x10, 0x004);
pub const MX1_PAD_I2C_SCL__GPIO1_16: (u32, u32) = (0x10, 0x032);
pub const MX1_PAD_DTACK__DTACK: (u32, u32) = (0x11, 0x000);
pub const MX1_PAD_DTACK__GPIO1_17: (u32, u32) = (0x11, 0x032);
pub const MX1_PAD_DTACK__SPI2_SS: (u32, u32) = (0x11, 0x002);
pub const MX1_PAD_DTACK__A25: (u32, u32) = (0x11, 0x016);
pub const MX1_PAD_BCLK__BCLK: (u32, u32) = (0x12, 0x004);
pub const MX1_PAD_BCLK__GPIO1_18: (u32, u32) = (0x12, 0x032);
pub const MX1_PAD_LBA__LBA: (u32, u32) = (0x13, 0x004);
pub const MX1_PAD_LBA__GPIO1_19: (u32, u32) = (0x13, 0x032);
pub const MX1_PAD_ECB__ECB: (u32, u32) = (0x14, 0x000);
pub const MX1_PAD_ECB__GPIO1_20: (u32, u32) = (0x14, 0x032);
pub const MX1_PAD_A0__A0: (u32, u32) = (0x15, 0x004);
pub const MX1_PAD_A0__GPIO1_21: (u32, u32) = (0x15, 0x032);
pub const MX1_PAD_CS4__CS4: (u32, u32) = (0x16, 0x004);
pub const MX1_PAD_CS4__GPIO1_22: (u32, u32) = (0x16, 0x032);
pub const MX1_PAD_CS5__CS5: (u32, u32) = (0x17, 0x004);
pub const MX1_PAD_CS5__GPIO1_23: (u32, u32) = (0x17, 0x032);
pub const MX1_PAD_A16__A16: (u32, u32) = (0x18, 0x004);
pub const MX1_PAD_A16__GPIO1_24: (u32, u32) = (0x18, 0x032);
pub const MX1_PAD_A17__A17: (u32, u32) = (0x19, 0x004);
pub const MX1_PAD_A17__GPIO1_25: (u32, u32) = (0x19, 0x032);
pub const MX1_PAD_A18__A18: (u32, u32) = (0x1a, 0x004);
pub const MX1_PAD_A18__GPIO1_26: (u32, u32) = (0x1a, 0x032);
pub const MX1_PAD_A19__A19: (u32, u32) = (0x1b, 0x004);
pub const MX1_PAD_A19__GPIO1_27: (u32, u32) = (0x1b, 0x032);
pub const MX1_PAD_A20__A20: (u32, u32) = (0x1c, 0x004);
pub const MX1_PAD_A20__GPIO1_28: (u32, u32) = (0x1c, 0x032);
pub const MX1_PAD_A21__A21: (u32, u32) = (0x1d, 0x004);
pub const MX1_PAD_A21__GPIO1_29: (u32, u32) = (0x1d, 0x032);
pub const MX1_PAD_A22__A22: (u32, u32) = (0x1e, 0x004);
pub const MX1_PAD_A22__GPIO1_30: (u32, u32) = (0x1e, 0x032);
pub const MX1_PAD_A23__A23: (u32, u32) = (0x1f, 0x004);
pub const MX1_PAD_A23__GPIO1_31: (u32, u32) = (0x1f, 0x032);
pub const MX1_PAD_SD_DAT0__SD_DAT0: (u32, u32) = (0x28, 0x000);
pub const MX1_PAD_SD_DAT0__MS_PI0: (u32, u32) = (0x28, 0x001);
pub const MX1_PAD_SD_DAT0__GPIO2_8: (u32, u32) = (0x28, 0x032);
pub const MX1_PAD_SD_DAT1__SD_DAT1: (u32, u32) = (0x29, 0x000);
pub const MX1_PAD_SD_DAT1__MS_PI1: (u32, u32) = (0x29, 0x001);
pub const MX1_PAD_SD_DAT1__GPIO2_9: (u32, u32) = (0x29, 0x032);
pub const MX1_PAD_SD_DAT2__SD_DAT2: (u32, u32) = (0x2a, 0x000);
pub const MX1_PAD_SD_DAT2__MS_SCLKI: (u32, u32) = (0x2a, 0x001);
pub const MX1_PAD_SD_DAT2__GPIO2_10: (u32, u32) = (0x2a, 0x032);
pub const MX1_PAD_SD_DAT3__SD_DAT3: (u32, u32) = (0x2b, 0x000);
pub const MX1_PAD_SD_DAT3__MS_SDIO: (u32, u32) = (0x2b, 0x001);
pub const MX1_PAD_SD_DAT3__GPIO2_11: (u32, u32) = (0x2b, 0x032);
pub const MX1_PAD_SD_SCLK__SD_SCLK: (u32, u32) = (0x2c, 0x004);
pub const MX1_PAD_SD_SCLK__MS_SCLKO: (u32, u32) = (0x2c, 0x005);
pub const MX1_PAD_SD_SCLK__GPIO2_12: (u32, u32) = (0x2c, 0x032);
pub const MX1_PAD_SD_CMD__SD_CMD: (u32, u32) = (0x2d, 0x000);
pub const MX1_PAD_SD_CMD__MS_BS: (u32, u32) = (0x2d, 0x005);
pub const MX1_PAD_SD_CMD__GPIO2_13: (u32, u32) = (0x2d, 0x032);
pub const MX1_PAD_SIM_SVEN__SIM_SVEN: (u32, u32) = (0x2e, 0x004);
pub const MX1_PAD_SIM_SVEN__SSI_RXFS: (u32, u32) = (0x2e, 0x001);
pub const MX1_PAD_SIM_SVEN__GPIO2_14: (u32, u32) = (0x2e, 0x032);
pub const MX1_PAD_SIM_PD__SIM_PD: (u32, u32) = (0x2f, 0x000);
pub const MX1_PAD_SIM_PD__SSI_RXCLK: (u32, u32) = (0x2f, 0x001);
pub const MX1_PAD_SIM_PD__GPIO2_15: (u32, u32) = (0x2f, 0x032);
pub const MX1_PAD_SIM_TX__SIM_TX: (u32, u32) = (0x30, 0x000);
pub const MX1_PAD_SIM_TX__SSI_RXDAT: (u32, u32) = (0x30, 0x001);
pub const MX1_PAD_SIM_TX__GPIO2_16: (u32, u32) = (0x30, 0x032);
pub const MX1_PAD_SIM_RX__SIM_RX: (u32, u32) = (0x31, 0x000);
pub const MX1_PAD_SIM_RX__SSI_TXDAT: (u32, u32) = (0x31, 0x005);
pub const MX1_PAD_SIM_RX__GPIO2_17: (u32, u32) = (0x31, 0x032);
pub const MX1_PAD_SIM_RST__SIM_RST: (u32, u32) = (0x32, 0x004);
pub const MX1_PAD_SIM_RST__SSI_TXFS: (u32, u32) = (0x32, 0x001);
pub const MX1_PAD_SIM_RST__GPIO2_18: (u32, u32) = (0x32, 0x032);
pub const MX1_PAD_SIM_CLK__SIM_CLK: (u32, u32) = (0x33, 0x004);
pub const MX1_PAD_SIM_CLK__SSI_TXCLK: (u32, u32) = (0x33, 0x001);
pub const MX1_PAD_SIM_CLK__GPIO2_19: (u32, u32) = (0x33, 0x032);
pub const MX1_PAD_USBD_AFE__USBD_AFE: (u32, u32) = (0x34, 0x004);
pub const MX1_PAD_USBD_AFE__GPIO2_20: (u32, u32) = (0x34, 0x032);
pub const MX1_PAD_USBD_OE__USBD_OE: (u32, u32) = (0x35, 0x004);
pub const MX1_PAD_USBD_OE__GPIO2_21: (u32, u32) = (0x35, 0x032);
pub const MX1_PAD_USBD_RCV__USBD_RCV: (u32, u32) = (0x36, 0x000);
pub const MX1_PAD_USBD_RCV__GPIO2_22: (u32, u32) = (0x36, 0x032);
pub const MX1_PAD_USBD_SUSPND__USBD_SUSPND: (u32, u32) = (0x37, 0x004);
pub const MX1_PAD_USBD_SUSPND__GPIO2_23: (u32, u32) = (0x37, 0x032);
pub const MX1_PAD_USBD_VP__USBD_VP: (u32, u32) = (0x38, 0x000);
pub const MX1_PAD_USBD_VP__GPIO2_24: (u32, u32) = (0x38, 0x032);
pub const MX1_PAD_USBD_VM__USBD_VM: (u32, u32) = (0x39, 0x000);
pub const MX1_PAD_USBD_VM__GPIO2_25: (u32, u32) = (0x39, 0x032);
pub const MX1_PAD_USBD_VPO__USBD_VPO: (u32, u32) = (0x3a, 0x004);
pub const MX1_PAD_USBD_VPO__GPIO2_26: (u32, u32) = (0x3a, 0x032);
pub const MX1_PAD_USBD_VMO__USBD_VMO: (u32, u32) = (0x3b, 0x004);
pub const MX1_PAD_USBD_VMO__GPIO2_27: (u32, u32) = (0x3b, 0x032);
pub const MX1_PAD_UART2_CTS__UART2_CTS: (u32, u32) = (0x3c, 0x004);
pub const MX1_PAD_UART2_CTS__GPIO2_28: (u32, u32) = (0x3c, 0x032);
pub const MX1_PAD_UART2_RTS__UART2_RTS: (u32, u32) = (0x3d, 0x000);
pub const MX1_PAD_UART2_RTS__GPIO2_29: (u32, u32) = (0x3d, 0x032);
pub const MX1_PAD_UART2_TXD__UART2_TXD: (u32, u32) = (0x3e, 0x004);
pub const MX1_PAD_UART2_TXD__GPIO2_30: (u32, u32) = (0x3e, 0x032);
pub const MX1_PAD_UART2_RXD__UART2_RXD: (u32, u32) = (0x3f, 0x000);
pub const MX1_PAD_UART2_RXD__GPIO2_31: (u32, u32) = (0x3f, 0x032);
pub const MX1_PAD_SSI_RXFS__SSI_RXFS: (u32, u32) = (0x43, 0x000);
pub const MX1_PAD_SSI_RXFS__GPIO3_3: (u32, u32) = (0x43, 0x032);
pub const MX1_PAD_SSI_RXCLK__SSI_RXCLK: (u32, u32) = (0x44, 0x000);
pub const MX1_PAD_SSI_RXCLK__GPIO3_4: (u32, u32) = (0x44, 0x032);
pub const MX1_PAD_SSI_RXDAT__SSI_RXDAT: (u32, u32) = (0x45, 0x000);
pub const MX1_PAD_SSI_RXDAT__GPIO3_5: (u32, u32) = (0x45, 0x032);
pub const MX1_PAD_SSI_TXDAT__SSI_TXDAT: (u32, u32) = (0x46, 0x004);
pub const MX1_PAD_SSI_TXDAT__GPIO3_6: (u32, u32) = (0x46, 0x032);
pub const MX1_PAD_SSI_TXFS__SSI_TXFS: (u32, u32) = (0x47, 0x000);
pub const MX1_PAD_SSI_TXFS__GPIO3_7: (u32, u32) = (0x47, 0x032);
pub const MX1_PAD_SSI_TXCLK__SSI_TXCLK: (u32, u32) = (0x48, 0x000);
pub const MX1_PAD_SSI_TXCLK__GPIO3_8: (u32, u32) = (0x48, 0x032);
pub const MX1_PAD_UART1_CTS__UART1_CTS: (u32, u32) = (0x49, 0x004);
pub const MX1_PAD_UART1_CTS__GPIO3_9: (u32, u32) = (0x49, 0x032);
pub const MX1_PAD_UART1_RTS__UART1_RTS: (u32, u32) = (0x4a, 0x000);
pub const MX1_PAD_UART1_RTS__GPIO3_10: (u32, u32) = (0x4a, 0x032);
pub const MX1_PAD_UART1_TXD__UART1_TXD: (u32, u32) = (0x4b, 0x004);
pub const MX1_PAD_UART1_TXD__GPIO3_11: (u32, u32) = (0x4b, 0x032);
pub const MX1_PAD_UART1_RXD__UART1_RXD: (u32, u32) = (0x4c, 0x000);
pub const MX1_PAD_UART1_RXD__GPIO3_12: (u32, u32) = (0x4c, 0x032);
pub const MX1_PAD_SPI1_RDY__SPI1_RDY: (u32, u32) = (0x4d, 0x000);
pub const MX1_PAD_SPI1_RDY__GPIO3_13: (u32, u32) = (0x4d, 0x032);
pub const MX1_PAD_SPI1_SCLK__SPI1_SCLK: (u32, u32) = (0x4e, 0x004);
pub const MX1_PAD_SPI1_SCLK__GPIO3_14: (u32, u32) = (0x4e, 0x032);
pub const MX1_PAD_SPI1_SS__SPI1_SS: (u32, u32) = (0x4f, 0x000);
pub const MX1_PAD_SPI1_SS__GPIO3_15: (u32, u32) = (0x4f, 0x032);
pub const MX1_PAD_SPI1_MISO__SPI1_MISO: (u32, u32) = (0x50, 0x000);
pub const MX1_PAD_SPI1_MISO__GPIO3_16: (u32, u32) = (0x50, 0x032);
pub const MX1_PAD_SPI1_MOSI__SPI1_MOSI: (u32, u32) = (0x51, 0x004);
pub const MX1_PAD_SPI1_MOSI__GPIO3_17: (u32, u32) = (0x51, 0x032);
pub const MX1_PAD_BT13__BT13: (u32, u32) = (0x53, 0x004);
pub const MX1_PAD_BT13__SSI2_RXCLK: (u32, u32) = (0x53, 0x001);
pub const MX1_PAD_BT13__GPIO3_19: (u32, u32) = (0x53, 0x032);
pub const MX1_PAD_BT12__BT12: (u32, u32) = (0x54, 0x004);
pub const MX1_PAD_BT12__SSI2_TXFS: (u32, u32) = (0x54, 0x001);
pub const MX1_PAD_BT12__GPIO3_20: (u32, u32) = (0x54, 0x032);
pub const MX1_PAD_BT11__BT11: (u32, u32) = (0x55, 0x004);
pub const MX1_PAD_BT11__SSI2_TXCLK: (u32, u32) = (0x55, 0x001);
pub const MX1_PAD_BT11__GPIO3_21: (u32, u32) = (0x55, 0x032);
pub const MX1_PAD_BT10__BT10: (u32, u32) = (0x56, 0x004);
pub const MX1_PAD_BT10__SSI2_TX: (u32, u32) = (0x56, 0x001);
pub const MX1_PAD_BT10__GPIO3_22: (u32, u32) = (0x56, 0x032);
pub const MX1_PAD_BT9__BT9: (u32, u32) = (0x57, 0x004);
pub const MX1_PAD_BT9__SSI2_RX: (u32, u32) = (0x57, 0x001);
pub const MX1_PAD_BT9__GPIO3_23: (u32, u32) = (0x57, 0x032);
pub const MX1_PAD_BT8__BT8: (u32, u32) = (0x58, 0x004);
pub const MX1_PAD_BT8__SSI2_RXFS: (u32, u32) = (0x58, 0x001);
pub const MX1_PAD_BT8__GPIO3_24: (u32, u32) = (0x58, 0x032);
pub const MX1_PAD_BT8__UART3_RI: (u32, u32) = (0x58, 0x016);
pub const MX1_PAD_BT7__BT7: (u32, u32) = (0x59, 0x004);
pub const MX1_PAD_BT7__GPIO3_25: (u32, u32) = (0x59, 0x032);
pub const MX1_PAD_BT7__UART3_DSR: (u32, u32) = (0x59, 0x016);
pub const MX1_PAD_BT6__BT6: (u32, u32) = (0x5a, 0x004);
pub const MX1_PAD_BT6__GPIO3_26: (u32, u32) = (0x5a, 0x032);
pub const MX1_PAD_BT6__SPI2_SS3: (u32, u32) = (0x5a, 0x016);
pub const MX1_PAD_BT6__UART3_DTR: (u32, u32) = (0x5a, 0x022);
pub const MX1_PAD_BT5__BT5: (u32, u32) = (0x5b, 0x000);
pub const MX1_PAD_BT5__GPIO3_27: (u32, u32) = (0x5b, 0x032);
pub const MX1_PAD_BT5__UART3_DCD: (u32, u32) = (0x5b, 0x016);
pub const MX1_PAD_BT4__BT4: (u32, u32) = (0x5c, 0x000);
pub const MX1_PAD_BT4__GPIO3_28: (u32, u32) = (0x5c, 0x032);
pub const MX1_PAD_BT4__UART3_CTS: (u32, u32) = (0x5c, 0x016);
pub const MX1_PAD_BT3__BT3: (u32, u32) = (0x5d, 0x000);
pub const MX1_PAD_BT3__GPIO3_29: (u32, u32) = (0x5d, 0x032);
pub const MX1_PAD_BT3__UART3_RTS: (u32, u32) = (0x5d, 0x022);
pub const MX1_PAD_BT2__BT2: (u32, u32) = (0x5e, 0x004);
pub const MX1_PAD_BT2__GPIO3_30: (u32, u32) = (0x5e, 0x032);
pub const MX1_PAD_BT2__UART3_TX: (u32, u32) = (0x5e, 0x016);
pub const MX1_PAD_BT1__BT1: (u32, u32) = (0x5f, 0x000);
pub const MX1_PAD_BT1__GPIO3_31: (u32, u32) = (0x5f, 0x032);
pub const MX1_PAD_BT1__UART3_RX: (u32, u32) = (0x5f, 0x022);
pub const MX1_PAD_LSCLK__LSCLK: (u32, u32) = (0x66, 0x004);
pub const MX1_PAD_LSCLK__GPIO4_6: (u32, u32) = (0x66, 0x032);
pub const MX1_PAD_REV__REV: (u32, u32) = (0x67, 0x004);
pub const MX1_PAD_REV__UART2_DTR: (u32, u32) = (0x67, 0x001);
pub const MX1_PAD_REV__GPIO4_7: (u32, u32) = (0x67, 0x032);
pub const MX1_PAD_REV__SPI2_CLK: (u32, u32) = (0x67, 0x006);
pub const MX1_PAD_CLS__CLS: (u32, u32) = (0x68, 0x004);
pub const MX1_PAD_CLS__UART2_DCD: (u32, u32) = (0x68, 0x005);
pub const MX1_PAD_CLS__GPIO4_8: (u32, u32) = (0x68, 0x032);
pub const MX1_PAD_CLS__SPI2_SS: (u32, u32) = (0x68, 0x002);
pub const MX1_PAD_PS__PS: (u32, u32) = (0x69, 0x004);
pub const MX1_PAD_PS__UART2_RI: (u32, u32) = (0x69, 0x005);
pub const MX1_PAD_PS__GPIO4_9: (u32, u32) = (0x69, 0x032);
pub const MX1_PAD_PS__SPI2_RXD: (u32, u32) = (0x69, 0x022);
pub const MX1_PAD_SPL_SPR__SPL_SPR: (u32, u32) = (0x6a, 0x004);
pub const MX1_PAD_SPL_SPR__UART2_DSR: (u32, u32) = (0x6a, 0x005);
pub const MX1_PAD_SPL_SPR__GPIO4_10: (u32, u32) = (0x6a, 0x032);
pub const MX1_PAD_SPL_SPR__SPI2_TXD: (u32, u32) = (0x6a, 0x006);
pub const MX1_PAD_CONTRAST__CONTRAST: (u32, u32) = (0x6b, 0x004);
pub const MX1_PAD_CONTRAST__GPIO4_11: (u32, u32) = (0x6b, 0x032);
pub const MX1_PAD_CONTRAST__SPI2_SS2: (u32, u32) = (0x6b, 0x012);
pub const MX1_PAD_ACD_OE__ACD_OE: (u32, u32) = (0x6c, 0x004);
pub const MX1_PAD_ACD_OE__GPIO4_12: (u32, u32) = (0x6c, 0x032);
pub const MX1_PAD_LP_HSYNC__LP_HSYNC: (u32, u32) = (0x6d, 0x004);
pub const MX1_PAD_LP_HSYNC__GPIO4_13: (u32, u32) = (0x6d, 0x032);
pub const MX1_PAD_FLM_VSYNC__FLM_VSYNC: (u32, u32) = (0x6e, 0x004);
pub const MX1_PAD_FLM_VSYNC__GPIO4_14: (u32, u32) = (0x6e, 0x032);
pub const MX1_PAD_LD0__LD0: (u32, u32) = (0x6f, 0x004);
pub const MX1_PAD_LD0__GPIO4_15: (u32, u32) = (0x6f, 0x032);
pub const MX1_PAD_LD1__LD1: (u32, u32) = (0x70, 0x004);
pub const MX1_PAD_LD1__GPIO4_16: (u32, u32) = (0x70, 0x032);
pub const MX1_PAD_LD2__LD2: (u32, u32) = (0x71, 0x004);
pub const MX1_PAD_LD2__GPIO4_17: (u32, u32) = (0x71, 0x032);
pub const MX1_PAD_LD3__LD3: (u32, u32) = (0x72, 0x004);
pub const MX1_PAD_LD3__GPIO4_18: (u32, u32) = (0x72, 0x032);
pub const MX1_PAD_LD4__LD4: (u32, u32) = (0x73, 0x004);
pub const MX1_PAD_LD4__GPIO4_19: (u32, u32) = (0x73, 0x032);
pub const MX1_PAD_LD5__LD5: (u32, u32) = (0x74, 0x004);
pub const MX1_PAD_LD5__GPIO4_20: (u32, u32) = (0x74, 0x032);
pub const MX1_PAD_LD6__LD6: (u32, u32) = (0x75, 0x004);
pub const MX1_PAD_LD6__GPIO4_21: (u32, u32) = (0x75, 0x032);
pub const MX1_PAD_LD7__LD7: (u32, u32) = (0x76, 0x004);
pub const MX1_PAD_LD7__GPIO4_22: (u32, u32) = (0x76, 0x032);
pub const MX1_PAD_LD8__LD8: (u32, u32) = (0x77, 0x004);
pub const MX1_PAD_LD8__GPIO4_23: (u32, u32) = (0x77, 0x032);
pub const MX1_PAD_LD9__LD9: (u32, u32) = (0x78, 0x004);
pub const MX1_PAD_LD9__GPIO4_24: (u32, u32) = (0x78, 0x032);
pub const MX1_PAD_LD10__LD10: (u32, u32) = (0x79, 0x004);
pub const MX1_PAD_LD10__GPIO4_25: (u32, u32) = (0x79, 0x032);
pub const MX1_PAD_LD11__LD11: (u32, u32) = (0x7a, 0x004);
pub const MX1_PAD_LD11__GPIO4_26: (u32, u32) = (0x7a, 0x032);
pub const MX1_PAD_LD12__LD12: (u32, u32) = (0x7b, 0x004);
pub const MX1_PAD_LD12__GPIO4_27: (u32, u32) = (0x7b, 0x032);
pub const MX1_PAD_LD13__LD13: (u32, u32) = (0x7c, 0x004);
pub const MX1_PAD_LD13__GPIO4_28: (u32, u32) = (0x7c, 0x032);
pub const MX1_PAD_LD14__LD14: (u32, u32) = (0x7d, 0x004);
pub const MX1_PAD_LD14__GPIO4_29: (u32, u32) = (0x7d, 0x032);
pub const MX1_PAD_LD15__LD15: (u32, u32) = (0x7e, 0x004);
pub const MX1_PAD_LD15__GPIO4_30: (u32, u32) = (0x7e, 0x032);
pub const MX1_PAD_TMR2OUT__TMR2OUT: (u32, u32) = (0x7f, 0x000);
pub const MX1_PAD_TMR2OUT__GPIO4_31: (u32, u32) = (0x7f, 0x032);
pub const MX1_PAD_TMR2OUT__SPI2_TXD: (u32, u32) = (0x7f, 0x006);

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
