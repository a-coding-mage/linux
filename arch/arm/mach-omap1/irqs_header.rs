/* SPDX-License-Identifier: GPL-2.0-or-later */
/* Translated from arm/mach-omap1/irqs.h. */

// IRQ numbers for interrupt handler 1.
pub const INT_CAMERA: usize = NR_IRQS_LEGACY + 1;
pub const INT_FIQ: usize = NR_IRQS_LEGACY + 3;
pub const INT_RTDX: usize = NR_IRQS_LEGACY + 6;
pub const INT_DSP_MMU_ABORT: usize = NR_IRQS_LEGACY + 7;
pub const INT_HOST: usize = NR_IRQS_LEGACY + 8;
pub const INT_ABORT: usize = NR_IRQS_LEGACY + 9;
pub const INT_BRIDGE_PRIV: usize = NR_IRQS_LEGACY + 13;
pub const INT_GPIO_BANK1: usize = NR_IRQS_LEGACY + 14;
pub const INT_UART3: usize = NR_IRQS_LEGACY + 15;
pub const INT_TIMER3: usize = NR_IRQS_LEGACY + 16;
pub const INT_DMA_CH0_6: usize = NR_IRQS_LEGACY + 19;
pub const INT_DMA_CH1_7: usize = NR_IRQS_LEGACY + 20;
pub const INT_DMA_CH2_8: usize = NR_IRQS_LEGACY + 21;
pub const INT_DMA_CH3: usize = NR_IRQS_LEGACY + 22;
pub const INT_DMA_CH4: usize = NR_IRQS_LEGACY + 23;
pub const INT_DMA_CH5: usize = NR_IRQS_LEGACY + 24;
pub const INT_TIMER1: usize = NR_IRQS_LEGACY + 26;
pub const INT_WD_TIMER: usize = NR_IRQS_LEGACY + 27;
pub const INT_BRIDGE_PUB: usize = NR_IRQS_LEGACY + 28;
pub const INT_TIMER2: usize = NR_IRQS_LEGACY + 30;
pub const INT_LCD_CTRL: usize = NR_IRQS_LEGACY + 31;

pub const INT_1510_IH2_IRQ: usize = NR_IRQS_LEGACY + 0;
pub const INT_1510_RES2: usize = NR_IRQS_LEGACY + 2;
pub const INT_1510_SPI_TX: usize = NR_IRQS_LEGACY + 4;
pub const INT_1510_SPI_RX: usize = NR_IRQS_LEGACY + 5;
pub const INT_1510_DSP_MAILBOX1: usize = NR_IRQS_LEGACY + 10;
pub const INT_1510_DSP_MAILBOX2: usize = NR_IRQS_LEGACY + 11;
pub const INT_1510_RES12: usize = NR_IRQS_LEGACY + 12;
pub const INT_1510_LB_MMU: usize = NR_IRQS_LEGACY + 17;
pub const INT_1510_RES18: usize = NR_IRQS_LEGACY + 18;
pub const INT_1510_LOCAL_BUS: usize = NR_IRQS_LEGACY + 29;

pub const INT_1610_IH2_IRQ: usize = INT_1510_IH2_IRQ;
pub const INT_1610_IH2_FIQ: usize = NR_IRQS_LEGACY + 2;
pub const INT_1610_McBSP2_TX: usize = NR_IRQS_LEGACY + 4;
pub const INT_1610_McBSP2_RX: usize = NR_IRQS_LEGACY + 5;
pub const INT_1610_DSP_MAILBOX1: usize = NR_IRQS_LEGACY + 10;
pub const INT_1610_DSP_MAILBOX2: usize = NR_IRQS_LEGACY + 11;
pub const INT_1610_LCD_LINE: usize = NR_IRQS_LEGACY + 12;
pub const INT_1610_GPTIMER1: usize = NR_IRQS_LEGACY + 17;
pub const INT_1610_GPTIMER2: usize = NR_IRQS_LEGACY + 18;
pub const INT_1610_SSR_FIFO_0: usize = NR_IRQS_LEGACY + 29;

pub const INT_7XX_IH2_FIQ: usize = NR_IRQS_LEGACY + 0;
pub const INT_7XX_IH2_IRQ: usize = NR_IRQS_LEGACY + 1;
pub const INT_7XX_USB_NON_ISO: usize = NR_IRQS_LEGACY + 2;
pub const INT_7XX_USB_ISO: usize = NR_IRQS_LEGACY + 3;
pub const INT_7XX_ICR: usize = NR_IRQS_LEGACY + 4;
pub const INT_7XX_EAC: usize = NR_IRQS_LEGACY + 5;
pub const INT_7XX_GPIO_BANK1: usize = NR_IRQS_LEGACY + 6;
pub const INT_7XX_GPIO_BANK2: usize = NR_IRQS_LEGACY + 7;
pub const INT_7XX_GPIO_BANK3: usize = NR_IRQS_LEGACY + 8;
pub const INT_7XX_McBSP2TX: usize = NR_IRQS_LEGACY + 10;
pub const INT_7XX_McBSP2RX: usize = NR_IRQS_LEGACY + 11;
pub const INT_7XX_McBSP2RX_OVF: usize = NR_IRQS_LEGACY + 12;
pub const INT_7XX_LCD_LINE: usize = NR_IRQS_LEGACY + 14;
pub const INT_7XX_GSM_PROTECT: usize = NR_IRQS_LEGACY + 15;
pub const INT_7XX_TIMER3: usize = NR_IRQS_LEGACY + 16;
pub const INT_7XX_GPIO_BANK5: usize = NR_IRQS_LEGACY + 17;
pub const INT_7XX_GPIO_BANK6: usize = NR_IRQS_LEGACY + 18;
pub const INT_7XX_SPGIO_WR: usize = NR_IRQS_LEGACY + 29;

pub const IH2_BASE: usize = NR_IRQS_LEGACY + 32;

pub const INT_KEYBOARD: usize = 1 + IH2_BASE;
pub const INT_uWireTX: usize = 2 + IH2_BASE;
pub const INT_uWireRX: usize = 3 + IH2_BASE;
pub const INT_I2C: usize = 4 + IH2_BASE;
pub const INT_MPUIO: usize = 5 + IH2_BASE;
pub const INT_USB_HHC_1: usize = 6 + IH2_BASE;
pub const INT_McBSP3TX: usize = 10 + IH2_BASE;
pub const INT_McBSP3RX: usize = 11 + IH2_BASE;
pub const INT_McBSP1TX: usize = 12 + IH2_BASE;
pub const INT_McBSP1RX: usize = 13 + IH2_BASE;
pub const INT_UART1: usize = 14 + IH2_BASE;
pub const INT_UART2: usize = 15 + IH2_BASE;
pub const INT_BT_MCSI1TX: usize = 16 + IH2_BASE;
pub const INT_BT_MCSI1RX: usize = 17 + IH2_BASE;
pub const INT_SOSSI_MATCH: usize = 19 + IH2_BASE;
pub const INT_USB_W2FC: usize = 20 + IH2_BASE;
pub const INT_1WIRE: usize = 21 + IH2_BASE;
pub const INT_OS_TIMER: usize = 22 + IH2_BASE;
pub const INT_MMC: usize = 23 + IH2_BASE;
pub const INT_GAUGE_32K: usize = 24 + IH2_BASE;
pub const INT_RTC_TIMER: usize = 25 + IH2_BASE;
pub const INT_RTC_ALARM: usize = 26 + IH2_BASE;
pub const INT_MEM_STICK: usize = 27 + IH2_BASE;

pub const INT_1510_DSP_MMU: usize = 28 + IH2_BASE;
pub const INT_1510_COM_SPI_RO: usize = 31 + IH2_BASE;

pub const INT_1610_FAC: usize = IH2_BASE;
pub const INT_1610_USB_HHC_2: usize = 7 + IH2_BASE;
pub const INT_1610_USB_OTG: usize = 8 + IH2_BASE;
pub const INT_1610_SoSSI: usize = 9 + IH2_BASE;
pub const INT_1610_SoSSI_MATCH: usize = 19 + IH2_BASE;
pub const INT_1610_DSP_MMU: usize = 28 + IH2_BASE;
pub const INT_1610_McBSP2RX_OF: usize = 31 + IH2_BASE;
pub const INT_1610_STI: usize = 32 + IH2_BASE;
pub const INT_1610_STI_WAKEUP: usize = 33 + IH2_BASE;
pub const INT_1610_GPTIMER3: usize = 34 + IH2_BASE;
pub const INT_1610_GPTIMER4: usize = 35 + IH2_BASE;
pub const INT_1610_GPTIMER5: usize = 36 + IH2_BASE;
pub const INT_1610_GPTIMER6: usize = 37 + IH2_BASE;
pub const INT_1610_GPTIMER7: usize = 38 + IH2_BASE;
pub const INT_1610_GPTIMER8: usize = 39 + IH2_BASE;
pub const INT_1610_GPIO_BANK2: usize = 40 + IH2_BASE;
pub const INT_1610_GPIO_BANK3: usize = 41 + IH2_BASE;
pub const INT_1610_MMC2: usize = 42 + IH2_BASE;
pub const INT_1610_CF: usize = 43 + IH2_BASE;
pub const INT_1610_WAKE_UP_REQ: usize = 46 + IH2_BASE;
pub const INT_1610_GPIO_BANK4: usize = 48 + IH2_BASE;
pub const INT_1610_SPI: usize = 49 + IH2_BASE;
pub const INT_1610_DMA_CH6: usize = 53 + IH2_BASE;
pub const INT_1610_DMA_CH7: usize = 54 + IH2_BASE;
pub const INT_1610_DMA_CH8: usize = 55 + IH2_BASE;
pub const INT_1610_DMA_CH9: usize = 56 + IH2_BASE;
pub const INT_1610_DMA_CH10: usize = 57 + IH2_BASE;
pub const INT_1610_DMA_CH11: usize = 58 + IH2_BASE;
pub const INT_1610_DMA_CH12: usize = 59 + IH2_BASE;
pub const INT_1610_DMA_CH13: usize = 60 + IH2_BASE;
pub const INT_1610_DMA_CH14: usize = 61 + IH2_BASE;
pub const INT_1610_DMA_CH15: usize = 62 + IH2_BASE;
pub const INT_1610_NAND: usize = 63 + IH2_BASE;
pub const INT_1610_SHA1MD5: usize = 91 + IH2_BASE;

// OMAP-7xx specific IRQ numbers for interrupt handler 2.
pub const INT_7XX_HW_ERRORS: usize = IH2_BASE;
pub const INT_7XX_NFIQ_PWR_FAIL: usize = 1 + IH2_BASE;
pub const INT_7XX_CFCD: usize = 2 + IH2_BASE;
pub const INT_7XX_CFIREQ: usize = 3 + IH2_BASE;
pub const INT_7XX_I2C: usize = 4 + IH2_BASE;
pub const INT_7XX_PCC: usize = 5 + IH2_BASE;
pub const INT_7XX_MPU_EXT_NIRQ: usize = 6 + IH2_BASE;
pub const INT_7XX_SPI_100K_1: usize = 7 + IH2_BASE;
pub const INT_7XX_SYREN_SPI: usize = 8 + IH2_BASE;
pub const INT_7XX_VLYNQ: usize = 9 + IH2_BASE;
pub const INT_7XX_GPIO_BANK4: usize = 10 + IH2_BASE;
pub const INT_7XX_McBSP1TX: usize = 11 + IH2_BASE;
pub const INT_7XX_McBSP1RX: usize = 12 + IH2_BASE;
pub const INT_7XX_McBSP1RX_OF: usize = 13 + IH2_BASE;
pub const INT_7XX_UART_MODEM_IRDA_2: usize = 14 + IH2_BASE;
pub const INT_7XX_UART_MODEM_1: usize = 15 + IH2_BASE;
pub const INT_7XX_MCSI: usize = 16 + IH2_BASE;
pub const INT_7XX_uWireTX: usize = 17 + IH2_BASE;
pub const INT_7XX_uWireRX: usize = 18 + IH2_BASE;
pub const INT_7XX_SMC_CD: usize = 19 + IH2_BASE;
pub const INT_7XX_SMC_IREQ: usize = 20 + IH2_BASE;
pub const INT_7XX_HDQ_1WIRE: usize = 21 + IH2_BASE;
pub const INT_7XX_TIMER32K: usize = 22 + IH2_BASE;
pub const INT_7XX_MMC_SDIO: usize = 23 + IH2_BASE;
pub const INT_7XX_UPLD: usize = 24 + IH2_BASE;
pub const INT_7XX_USB_HHC_1: usize = 27 + IH2_BASE;
pub const INT_7XX_USB_HHC_2: usize = 28 + IH2_BASE;
pub const INT_7XX_USB_GENI: usize = 29 + IH2_BASE;
pub const INT_7XX_USB_OTG: usize = 30 + IH2_BASE;
pub const INT_7XX_CAMERA_IF: usize = 31 + IH2_BASE;
pub const INT_7XX_RNG: usize = 32 + IH2_BASE;
pub const INT_7XX_DUAL_MODE_TIMER: usize = 33 + IH2_BASE;
pub const INT_7XX_DBB_RF_EN: usize = 34 + IH2_BASE;
pub const INT_7XX_MPUIO_KEYPAD: usize = 35 + IH2_BASE;
pub const INT_7XX_SHA1_MD5: usize = 36 + IH2_BASE;
pub const INT_7XX_SPI_100K_2: usize = 37 + IH2_BASE;
pub const INT_7XX_RNG_IDLE: usize = 38 + IH2_BASE;
pub const INT_7XX_MPUIO: usize = 39 + IH2_BASE;
pub const INT_7XX_LLPC_LCD_CTRL_CAN_BE_OFF: usize = 40 + IH2_BASE;
pub const INT_7XX_LLPC_OE_FALLING: usize = 41 + IH2_BASE;
pub const INT_7XX_LLPC_OE_RISING: usize = 42 + IH2_BASE;
pub const INT_7XX_LLPC_VSYNC: usize = 43 + IH2_BASE;
pub const INT_7XX_WAKE_UP_REQ: usize = 46 + IH2_BASE;
pub const INT_7XX_DMA_CH6: usize = 53 + IH2_BASE;
pub const INT_7XX_DMA_CH7: usize = 54 + IH2_BASE;
pub const INT_7XX_DMA_CH8: usize = 55 + IH2_BASE;
pub const INT_7XX_DMA_CH9: usize = 56 + IH2_BASE;
pub const INT_7XX_DMA_CH10: usize = 57 + IH2_BASE;
pub const INT_7XX_DMA_CH11: usize = 58 + IH2_BASE;
pub const INT_7XX_DMA_CH12: usize = 59 + IH2_BASE;
pub const INT_7XX_DMA_CH13: usize = 60 + IH2_BASE;
pub const INT_7XX_DMA_CH14: usize = 61 + IH2_BASE;
pub const INT_7XX_DMA_CH15: usize = 62 + IH2_BASE;
pub const INT_7XX_NAND: usize = 63 + IH2_BASE;

// Max. 128 level 2 IRQs (OMAP1610), 192 GPIOs (OMAP730/850), and 16 MPUIO lines.
pub const OMAP_MAX_GPIO_LINES: usize = 192;
pub const IH_GPIO_BASE: usize = 128 + IH2_BASE;
pub const IH_MPUIO_BASE: usize = OMAP_MAX_GPIO_LINES + IH_GPIO_BASE;
pub const OMAP_IRQ_END: usize = IH_MPUIO_BASE + 16;

#[macro_export]
macro_rules! OMAP_IRQ_BIT {
    ($irq:expr) => { 1usize << (($irq - $crate::NR_IRQS_LEGACY) % 32) };
}

// C: defined only when CONFIG_FIQ is enabled.
#[cfg(feature = "CONFIG_FIQ")]
pub const FIQ_START: usize = 1024;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
