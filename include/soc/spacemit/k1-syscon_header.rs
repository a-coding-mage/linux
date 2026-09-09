/* SPDX-License-Identifier: GPL-2.0-only */

/* SpacemiT clock and reset driver definitions for the K1 SoC */

/* Dependency: symbols from ccu.h are supplied by the surrounding translation. */

/* APBS register offset */
pub const APBS_PLL1_SWCR1: u32 = 0x100;
pub const APBS_PLL1_SWCR2: u32 = 0x104;
pub const APBS_PLL1_SWCR3: u32 = 0x108;
pub const APBS_PLL2_SWCR1: u32 = 0x118;
pub const APBS_PLL2_SWCR2: u32 = 0x11c;
pub const APBS_PLL2_SWCR3: u32 = 0x120;
pub const APBS_PLL3_SWCR1: u32 = 0x124;
pub const APBS_PLL3_SWCR2: u32 = 0x128;
pub const APBS_PLL3_SWCR3: u32 = 0x12c;

/* MPMU register offset */
pub const MPMU_POSR: u32 = 0x0010;
pub const MPMU_FCCR: u32 = 0x0008;
pub const POSR_PLL1_LOCK: u32 = 1u32 << 27;
pub const POSR_PLL2_LOCK: u32 = 1u32 << 28;
pub const POSR_PLL3_LOCK: u32 = 1u32 << 29;
pub const MPMU_SUCCR: u32 = 0x0014;
pub const MPMU_ISCCR: u32 = 0x0044;
pub const MPMU_WDTPCR: u32 = 0x0200;
pub const MPMU_RIPCCR: u32 = 0x0210;
pub const MPMU_ACGR: u32 = 0x1024;
pub const MPMU_APBCSCR: u32 = 0x1050;
pub const MPMU_SUCCR_1: u32 = 0x10b0;

/* APBC register offset */
pub const APBC_UART1_CLK_RST: u32 = 0x00;
pub const APBC_UART2_CLK_RST: u32 = 0x04;
pub const APBC_GPIO_CLK_RST: u32 = 0x08;
pub const APBC_PWM0_CLK_RST: u32 = 0x0c;
pub const APBC_PWM1_CLK_RST: u32 = 0x10;
pub const APBC_PWM2_CLK_RST: u32 = 0x14;
pub const APBC_PWM3_CLK_RST: u32 = 0x18;
pub const APBC_TWSI8_CLK_RST: u32 = 0x20;
pub const APBC_UART3_CLK_RST: u32 = 0x24;
pub const APBC_RTC_CLK_RST: u32 = 0x28;
pub const APBC_TWSI0_CLK_RST: u32 = 0x2c;
pub const APBC_TWSI1_CLK_RST: u32 = 0x30;
pub const APBC_TIMERS1_CLK_RST: u32 = 0x34;
pub const APBC_TWSI2_CLK_RST: u32 = 0x38;
pub const APBC_AIB_CLK_RST: u32 = 0x3c;
pub const APBC_TWSI4_CLK_RST: u32 = 0x40;
pub const APBC_TIMERS2_CLK_RST: u32 = 0x44;
pub const APBC_ONEWIRE_CLK_RST: u32 = 0x48;
pub const APBC_TWSI5_CLK_RST: u32 = 0x4c;
pub const APBC_DRO_CLK_RST: u32 = 0x58;
pub const APBC_IR_CLK_RST: u32 = 0x5c;
pub const APBC_TWSI6_CLK_RST: u32 = 0x60;
pub const APBC_COUNTER_CLK_SEL: u32 = 0x64;
pub const APBC_TWSI7_CLK_RST: u32 = 0x68;
pub const APBC_TSEN_CLK_RST: u32 = 0x6c;
pub const APBC_UART4_CLK_RST: u32 = 0x70;
pub const APBC_UART5_CLK_RST: u32 = 0x74;
pub const APBC_UART6_CLK_RST: u32 = 0x78;
pub const APBC_SSP3_CLK_RST: u32 = 0x7c;
pub const APBC_SSPA0_CLK_RST: u32 = 0x80;
pub const APBC_SSPA1_CLK_RST: u32 = 0x84;
pub const APBC_IPC_AP2AUD_CLK_RST: u32 = 0x90;
pub const APBC_UART7_CLK_RST: u32 = 0x94;
pub const APBC_UART8_CLK_RST: u32 = 0x98;
pub const APBC_UART9_CLK_RST: u32 = 0x9c;
pub const APBC_CAN0_CLK_RST: u32 = 0xa0;
pub const APBC_PWM4_CLK_RST: u32 = 0xa8;
pub const APBC_PWM5_CLK_RST: u32 = 0xac;
pub const APBC_PWM6_CLK_RST: u32 = 0xb0;
pub const APBC_PWM7_CLK_RST: u32 = 0xb4;
pub const APBC_PWM8_CLK_RST: u32 = 0xb8;
pub const APBC_PWM9_CLK_RST: u32 = 0xbc;
pub const APBC_PWM10_CLK_RST: u32 = 0xc0;
pub const APBC_PWM11_CLK_RST: u32 = 0xc4;
pub const APBC_PWM12_CLK_RST: u32 = 0xc8;
pub const APBC_PWM13_CLK_RST: u32 = 0xcc;
pub const APBC_PWM14_CLK_RST: u32 = 0xd0;
pub const APBC_PWM15_CLK_RST: u32 = 0xd4;
pub const APBC_PWM16_CLK_RST: u32 = 0xd8;
pub const APBC_PWM17_CLK_RST: u32 = 0xdc;
pub const APBC_PWM18_CLK_RST: u32 = 0xe0;
pub const APBC_PWM19_CLK_RST: u32 = 0xe4;

/* APMU register offset */
pub const APMU_JPG_CLK_RES_CTRL: u32 = 0x020;
pub const APMU_CSI_CCIC2_CLK_RES_CTRL: u32 = 0x024;
pub const APMU_ISP_CLK_RES_CTRL: u32 = 0x038;
pub const APMU_LCD_CLK_RES_CTRL1: u32 = 0x044;
pub const APMU_LCD_SPI_CLK_RES_CTRL: u32 = 0x048;
pub const APMU_LCD_CLK_RES_CTRL2: u32 = 0x04c;
pub const APMU_CCIC_CLK_RES_CTRL: u32 = 0x050;
pub const APMU_SDH0_CLK_RES_CTRL: u32 = 0x054;
pub const APMU_SDH1_CLK_RES_CTRL: u32 = 0x058;
pub const APMU_USB_CLK_RES_CTRL: u32 = 0x05c;
pub const APMU_QSPI_CLK_RES_CTRL: u32 = 0x060;
pub const APMU_DMA_CLK_RES_CTRL: u32 = 0x064;
pub const APMU_AES_CLK_RES_CTRL: u32 = 0x068;
pub const APMU_VPU_CLK_RES_CTRL: u32 = 0x0a4;
pub const APMU_GPU_CLK_RES_CTRL: u32 = 0x0cc;
pub const APMU_SDH2_CLK_RES_CTRL: u32 = 0x0e0;
pub const APMU_PMUA_MC_CTRL: u32 = 0x0e8;
pub const APMU_PMU_CC2_AP: u32 = 0x100;
pub const APMU_PMUA_EM_CLK_RES_CTRL: u32 = 0x104;
pub const APMU_AUDIO_CLK_RES_CTRL: u32 = 0x14c;
pub const APMU_HDMI_CLK_RES_CTRL: u32 = 0x1b8;
pub const APMU_CCI550_CLK_CTRL: u32 = 0x300;
pub const APMU_ACLK_CLK_CTRL: u32 = 0x388;
pub const APMU_CPU_C0_CLK_CTRL: u32 = 0x38C;
pub const APMU_CPU_C1_CLK_CTRL: u32 = 0x390;
pub const APMU_PCIE_CLK_RES_CTRL_0: u32 = 0x3cc;
pub const APMU_PCIE_CLK_RES_CTRL_1: u32 = 0x3d4;
pub const APMU_PCIE_CLK_RES_CTRL_2: u32 = 0x3dc;
pub const APMU_EMAC0_CLK_RES_CTRL: u32 = 0x3e4;
pub const APMU_EMAC1_CLK_RES_CTRL: u32 = 0x3ec;

/* RCPU register offsets */
pub const RCPU_SSP0_CLK_RST: u32 = 0x0028;
pub const RCPU_I2C0_CLK_RST: u32 = 0x0030;
pub const RCPU_UART1_CLK_RST: u32 = 0x003c;
pub const RCPU_CAN_CLK_RST: u32 = 0x0048;
pub const RCPU_IR_CLK_RST: u32 = 0x004c;
pub const RCPU_UART0_CLK_RST: u32 = 0x00d8;
pub const AUDIO_HDMI_CLK_CTRL: u32 = 0x2044;

/* RCPU2 register offsets */
pub const RCPU2_PWM0_CLK_RST: u32 = 0x0000;
pub const RCPU2_PWM1_CLK_RST: u32 = 0x0004;
pub const RCPU2_PWM2_CLK_RST: u32 = 0x0008;
pub const RCPU2_PWM3_CLK_RST: u32 = 0x000c;
pub const RCPU2_PWM4_CLK_RST: u32 = 0x0010;
pub const RCPU2_PWM5_CLK_RST: u32 = 0x0014;
pub const RCPU2_PWM6_CLK_RST: u32 = 0x0018;
pub const RCPU2_PWM7_CLK_RST: u32 = 0x001c;
pub const RCPU2_PWM8_CLK_RST: u32 = 0x0020;
pub const RCPU2_PWM9_CLK_RST: u32 = 0x0024;

/* APBC2 register offsets */
pub const APBC2_UART1_CLK_RST: u32 = 0x0000;
pub const APBC2_SSP2_CLK_RST: u32 = 0x0004;
pub const APBC2_TWSI3_CLK_RST: u32 = 0x0008;
pub const APBC2_RTC_CLK_RST: u32 = 0x000c;
pub const APBC2_TIMERS0_CLK_RST: u32 = 0x0010;
pub const APBC2_KPC_CLK_RST: u32 = 0x0014;
pub const APBC2_GPIO_CLK_RST: u32 = 0x001c;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
