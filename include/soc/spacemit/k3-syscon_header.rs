// SPDX-License-Identifier: GPL-2.0-only

/* SPDX-License-Identifier: GPL-2.0-only */

/* SpacemiT clock and reset driver definitions for the K3 SoC */


// Dependency: ccu.h

/* APBS register offset */
pub const APBS_PLL1_SWCR1: u32 = 0x100u32;
pub const APBS_PLL1_SWCR2: u32 = 0x104u32;
pub const APBS_PLL1_SWCR3: u32 = 0x108u32;
pub const APBS_PLL2_SWCR1: u32 = 0x118u32;
pub const APBS_PLL2_SWCR2: u32 = 0x11cu32;
pub const APBS_PLL2_SWCR3: u32 = 0x120u32;
pub const APBS_PLL3_SWCR1: u32 = 0x124u32;
pub const APBS_PLL3_SWCR2: u32 = 0x128u32;
pub const APBS_PLL3_SWCR3: u32 = 0x12cu32;
pub const APBS_PLL4_SWCR1: u32 = 0x130u32;
pub const APBS_PLL4_SWCR2: u32 = 0x134u32;
pub const APBS_PLL4_SWCR3: u32 = 0x138u32;
pub const APBS_PLL5_SWCR1: u32 = 0x13cu32;
pub const APBS_PLL5_SWCR2: u32 = 0x140u32;
pub const APBS_PLL5_SWCR3: u32 = 0x144u32;
pub const APBS_PLL6_SWCR1: u32 = 0x148u32;
pub const APBS_PLL6_SWCR2: u32 = 0x14cu32;
pub const APBS_PLL6_SWCR3: u32 = 0x150u32;
pub const APBS_PLL7_SWCR1: u32 = 0x158u32;
pub const APBS_PLL7_SWCR2: u32 = 0x15cu32;
pub const APBS_PLL7_SWCR3: u32 = 0x160u32;
pub const APBS_PLL8_SWCR1: u32 = 0x180u32;
pub const APBS_PLL8_SWCR2: u32 = 0x184u32;
pub const APBS_PLL8_SWCR3: u32 = 0x188u32;

/* MPMU register offset */
pub const MPMU_FCCR: u32 = 0x0008u32;
pub const MPMU_POSR: u32 = 0x0010u32;
pub const POSR_PLL1_LOCK: u32 = 1u32 << 24;
pub const POSR_PLL2_LOCK: u32 = 1u32 << 25;
pub const POSR_PLL3_LOCK: u32 = 1u32 << 26;
pub const POSR_PLL4_LOCK: u32 = 1u32 << 27;
pub const POSR_PLL5_LOCK: u32 = 1u32 << 28;
pub const POSR_PLL6_LOCK: u32 = 1u32 << 29;
pub const POSR_PLL7_LOCK: u32 = 1u32 << 30;
pub const POSR_PLL8_LOCK: u32 = 1u32 << 31;
pub const MPMU_SUCCR: u32 = 0x0014u32;
pub const MPMU_ISCCR0: u32 = 0x0040u32;
pub const MPMU_ISCCR1: u32 = 0x0044u32;
pub const MPMU_WDTPCR: u32 = 0x0200u32;
pub const MPMU_RIPCCR: u32 = 0x0210u32;
pub const MPMU_ACGR: u32 = 0x1024u32;
pub const MPMU_APBCSCR: u32 = 0x1050u32;
pub const MPMU_SUCCR_1: u32 = 0x10b0u32;

pub const MPMU_I2S0_SYSCLK: u32 = 0x1100u32;
pub const MPMU_I2S2_SYSCLK: u32 = 0x1104u32;
pub const MPMU_I2S3_SYSCLK: u32 = 0x1108u32;
pub const MPMU_I2S4_SYSCLK: u32 = 0x110cu32;
pub const MPMU_I2S5_SYSCLK: u32 = 0x1110u32;
pub const MPMU_I2S_SYSCLK_CTRL: u32 = 0x1114u32;

/* APBC register offset */
pub const APBC_UART0_CLK_RST: u32 = 0x00u32;
pub const APBC_UART2_CLK_RST: u32 = 0x04u32;
pub const APBC_GPIO_CLK_RST: u32 = 0x08u32;
pub const APBC_PWM0_CLK_RST: u32 = 0x0cu32;
pub const APBC_PWM1_CLK_RST: u32 = 0x10u32;
pub const APBC_PWM2_CLK_RST: u32 = 0x14u32;
pub const APBC_PWM3_CLK_RST: u32 = 0x18u32;
pub const APBC_TWSI8_CLK_RST: u32 = 0x20u32;
pub const APBC_UART3_CLK_RST: u32 = 0x24u32;
pub const APBC_RTC_CLK_RST: u32 = 0x28u32;
pub const APBC_TWSI0_CLK_RST: u32 = 0x2cu32;
pub const APBC_TWSI1_CLK_RST: u32 = 0x30u32;
pub const APBC_TIMERS0_CLK_RST: u32 = 0x34u32;
pub const APBC_TWSI2_CLK_RST: u32 = 0x38u32;
pub const APBC_AIB_CLK_RST: u32 = 0x3cu32;
pub const APBC_TWSI4_CLK_RST: u32 = 0x40u32;
pub const APBC_TIMERS1_CLK_RST: u32 = 0x44u32;
pub const APBC_ONEWIRE_CLK_RST: u32 = 0x48u32;
pub const APBC_TWSI5_CLK_RST: u32 = 0x4cu32;
pub const APBC_DRO_CLK_RST: u32 = 0x58u32;
pub const APBC_IR0_CLK_RST: u32 = 0x5cu32;
pub const APBC_IR1_CLK_RST: u32 = 0x1cu32;
pub const APBC_TWSI6_CLK_RST: u32 = 0x60u32;
pub const APBC_COUNTER_CLK_SEL: u32 = 0x64u32;
pub const APBC_TSEN_CLK_RST: u32 = 0x6cu32;
pub const APBC_UART4_CLK_RST: u32 = 0x70u32;
pub const APBC_UART5_CLK_RST: u32 = 0x74u32;
pub const APBC_UART6_CLK_RST: u32 = 0x78u32;
pub const APBC_SSP3_CLK_RST: u32 = 0x7cu32;
pub const APBC_SSPA0_CLK_RST: u32 = 0x80u32;
pub const APBC_SSPA1_CLK_RST: u32 = 0x84u32;
pub const APBC_SSPA2_CLK_RST: u32 = 0x88u32;
pub const APBC_SSPA3_CLK_RST: u32 = 0x8cu32;
pub const APBC_IPC_AP2AUD_CLK_RST: u32 = 0x90u32;
pub const APBC_UART7_CLK_RST: u32 = 0x94u32;
pub const APBC_UART8_CLK_RST: u32 = 0x98u32;
pub const APBC_UART9_CLK_RST: u32 = 0x9cu32;
pub const APBC_CAN0_CLK_RST: u32 = 0xa0u32;
pub const APBC_CAN1_CLK_RST: u32 = 0xa4u32;
pub const APBC_PWM4_CLK_RST: u32 = 0xa8u32;
pub const APBC_PWM5_CLK_RST: u32 = 0xacu32;
pub const APBC_PWM6_CLK_RST: u32 = 0xb0u32;
pub const APBC_PWM7_CLK_RST: u32 = 0xb4u32;
pub const APBC_PWM8_CLK_RST: u32 = 0xb8u32;
pub const APBC_PWM9_CLK_RST: u32 = 0xbcu32;
pub const APBC_PWM10_CLK_RST: u32 = 0xc0u32;
pub const APBC_PWM11_CLK_RST: u32 = 0xc4u32;
pub const APBC_PWM12_CLK_RST: u32 = 0xc8u32;
pub const APBC_PWM13_CLK_RST: u32 = 0xccu32;
pub const APBC_PWM14_CLK_RST: u32 = 0xd0u32;
pub const APBC_PWM15_CLK_RST: u32 = 0xd4u32;
pub const APBC_PWM16_CLK_RST: u32 = 0xd8u32;
pub const APBC_PWM17_CLK_RST: u32 = 0xdcu32;
pub const APBC_PWM18_CLK_RST: u32 = 0xe0u32;
pub const APBC_PWM19_CLK_RST: u32 = 0xe4u32;
pub const APBC_TIMERS2_CLK_RST: u32 = 0x11cu32;
pub const APBC_TIMERS3_CLK_RST: u32 = 0x120u32;
pub const APBC_TIMERS4_CLK_RST: u32 = 0x124u32;
pub const APBC_TIMERS5_CLK_RST: u32 = 0x128u32;
pub const APBC_TIMERS6_CLK_RST: u32 = 0x12cu32;
pub const APBC_TIMERS7_CLK_RST: u32 = 0x130u32;

pub const APBC_CAN2_CLK_RST: u32 = 0x148u32;
pub const APBC_CAN3_CLK_RST: u32 = 0x14cu32;
pub const APBC_CAN4_CLK_RST: u32 = 0x150u32;
pub const APBC_UART10_CLK_RST: u32 = 0x154u32;
pub const APBC_SSP0_CLK_RST: u32 = 0x158u32;
pub const APBC_SSP1_CLK_RST: u32 = 0x15cu32;
pub const APBC_SSPA4_CLK_RST: u32 = 0x160u32;
pub const APBC_SSPA5_CLK_RST: u32 = 0x164u32;

/* APMU register offset */
pub const APMU_CSI_CCIC2_CLK_RES_CTRL: u32 = 0x024u32;
pub const APMU_ISP_CLK_RES_CTRL: u32 = 0x038u32;
pub const APMU_PMU_CLK_GATE_CTRL: u32 = 0x040u32;
pub const APMU_LCD_CLK_RES_CTRL1: u32 = 0x044u32;
pub const APMU_LCD_SPI_CLK_RES_CTRL: u32 = 0x048u32;
pub const APMU_LCD_CLK_RES_CTRL2: u32 = 0x04cu32;
pub const APMU_CCIC_CLK_RES_CTRL: u32 = 0x050u32;
pub const APMU_SDH0_CLK_RES_CTRL: u32 = 0x054u32;
pub const APMU_SDH1_CLK_RES_CTRL: u32 = 0x058u32;
pub const APMU_USB_CLK_RES_CTRL: u32 = 0x05cu32;
pub const APMU_QSPI_CLK_RES_CTRL: u32 = 0x060u32;
pub const APMU_DMA_CLK_RES_CTRL: u32 = 0x064u32;
pub const APMU_AES_CLK_RES_CTRL: u32 = 0x068u32;
pub const APMU_MCB_CLK_RES_CTRL: u32 = 0x06cu32;
pub const APMU_VPU_CLK_RES_CTRL: u32 = 0x0a4u32;
pub const APMU_DTC_CLK_RES_CTRL: u32 = 0x0acu32;
pub const APMU_GPU_CLK_RES_CTRL: u32 = 0x0ccu32;
pub const APMU_SDH2_CLK_RES_CTRL: u32 = 0x0e0u32;
pub const APMU_PMUA_MC_CTRL: u32 = 0x0e8u32;
pub const APMU_PMU_CC2_AP: u32 = 0x100u32;
pub const APMU_PMUA_EM_CLK_RES_CTRL: u32 = 0x104u32;
pub const APMU_UCIE_CTRL: u32 = 0x11cu32;
pub const APMU_RCPU_CLK_RES_CTRL: u32 = 0x14cu32;
pub const APMU_TOP_DCLK_CTRL: u32 = 0x158u32;
pub const APMU_LCD_EDP_CTRL: u32 = 0x23cu32;
pub const APMU_UFS_CLK_RES_CTRL: u32 = 0x268u32;
pub const APMU_LCD_CLK_RES_CTRL3: u32 = 0x26cu32;
pub const APMU_LCD_CLK_RES_CTRL4: u32 = 0x270u32;
pub const APMU_LCD_CLK_RES_CTRL5: u32 = 0x274u32;
pub const APMU_CCI550_CLK_CTRL: u32 = 0x300u32;
pub const APMU_ACLK_CLK_CTRL: u32 = 0x388u32;
pub const APMU_CPU_C0_CLK_CTRL: u32 = 0x38Cu32;
pub const APMU_CPU_C1_CLK_CTRL: u32 = 0x390u32;
pub const APMU_CPU_C2_CLK_CTRL: u32 = 0x394u32;
pub const APMU_CPU_C3_CLK_CTRL: u32 = 0x208u32;
pub const APMU_PCIE_CLK_RES_CTRL_A: u32 = 0x1f0u32;
pub const APMU_PCIE_CLK_RES_CTRL_B: u32 = 0x1d0u32;
pub const APMU_PCIE_CLK_RES_CTRL_C: u32 = 0x1c8u32;
pub const APMU_PCIE_CLK_RES_CTRL_D: u32 = 0x1e0u32;
pub const APMU_PCIE_CLK_RES_CTRL_E: u32 = 0x1e8u32;
pub const APMU_EMAC0_CLK_RES_CTRL: u32 = 0x3e4u32;
pub const APMU_EMAC1_CLK_RES_CTRL: u32 = 0x3ecu32;
pub const APMU_EMAC2_CLK_RES_CTRL: u32 = 0x248u32;
pub const APMU_ESPI_CLK_RES_CTRL: u32 = 0x240u32;
pub const APMU_SNR_ISIM_VCLK_CTRL: u32 = 0x3f8u32;

/* DCIU register offsets */
pub const DCIU_DMASYS_CLK_EN: u32 = 0x234u32;
pub const DCIU_DMASYS_SDMA_CLK_EN: u32 = 0x238u32;
pub const DCIU_C2_TCM_PIPE_CLK: u32 = 0x244u32;
pub const DCIU_C3_TCM_PIPE_CLK: u32 = 0x248u32;

pub const DCIU_DMASYS_S0_RSTN: u32 = 0x204u32;
pub const DCIU_DMASYS_S1_RSTN: u32 = 0x208u32;
pub const DCIU_DMASYS_A0_RSTN: u32 = 0x20Cu32;
pub const DCIU_DMASYS_A1_RSTN: u32 = 0x210u32;
pub const DCIU_DMASYS_A2_RSTN: u32 = 0x214u32;
pub const DCIU_DMASYS_A3_RSTN: u32 = 0x218u32;
pub const DCIU_DMASYS_A4_RSTN: u32 = 0x21Cu32;
pub const DCIU_DMASYS_A5_RSTN: u32 = 0x220u32;
pub const DCIU_DMASYS_A6_RSTN: u32 = 0x224u32;
pub const DCIU_DMASYS_A7_RSTN: u32 = 0x228u32;
pub const DCIU_DMASYS_RSTN: u32 = 0x22Cu32;
pub const DCIU_DMASYS_SDMA_RSTN: u32 = 0x230u32;

/* RCPU SYSCTRL register offsets */
pub const RCPU_CAN_CLK_RST: u32 = 0x4cu32;
pub const RCPU_CAN1_CLK_RST: u32 = 0xF0u32;
pub const RCPU_CAN2_CLK_RST: u32 = 0xF4u32;
pub const RCPU_CAN3_CLK_RST: u32 = 0xF8u32;
pub const RCPU_CAN4_CLK_RST: u32 = 0xFCu32;
pub const RCPU_IRC_CLK_RST: u32 = 0x48u32;
pub const RCPU_IRC1_CLK_RST: u32 = 0xECu32;
pub const RCPU_GMAC_CLK_RST: u32 = 0xE4u32;
pub const RCPU_ESPI_CLK_RST: u32 = 0xDCu32;
pub const RCPU_AUDIO_I2S0_SYS_CLK_CTRL: u32 = 0x70u32;
pub const RCPU_AUDIO_I2S1_SYS_CLK_CTRL: u32 = 0x44u32;

/* RCPU UARTCTRL register offsets */
pub const RCPU1_UART0_CLK_RST: u32 = 0x00u32;
pub const RCPU1_UART1_CLK_RST: u32 = 0x04u32;
pub const RCPU1_UART2_CLK_RST: u32 = 0x08u32;
pub const RCPU1_UART3_CLK_RST: u32 = 0x0cu32;
pub const RCPU1_UART4_CLK_RST: u32 = 0x10u32;
pub const RCPU1_UART5_CLK_RST: u32 = 0x14u32;

/* RCPU I2SCTRL register offsets */
pub const RCPU2_AUDIO_I2S0_TX_RX_CLK_CTRL: u32 = 0x60u32;
pub const RCPU2_AUDIO_I2S1_TX_RX_CLK_CTRL: u32 = 0x64u32;
pub const RCPU2_AUDIO_I2S2_TX_RX_CLK_CTRL: u32 = 0x68u32;
pub const RCPU2_AUDIO_I2S3_TX_RX_CLK_CTRL: u32 = 0x6Cu32;

pub const RCPU2_AUDIO_I2S2_SYS_CLK_CTRL: u32 = 0x44u32;
pub const RCPU2_AUDIO_I2S3_SYS_CLK_CTRL: u32 = 0x54u32;

/* RCPU SPICTRL register offsets */
pub const RCPU3_SSP0_CLK_RST: u32 = 0x00u32;
pub const RCPU3_SSP1_CLK_RST: u32 = 0x04u32;
pub const RCPU3_PWR_SSP_CLK_RST: u32 = 0x08u32;

/* RCPU I2CCTRL register offsets */
pub const RCPU4_I2C0_CLK_RST: u32 = 0x00u32;
pub const RCPU4_I2C1_CLK_RST: u32 = 0x04u32;
pub const RCPU4_PWR_I2C_CLK_RST: u32 = 0x08u32;

/* RPMU register offsets */
pub const RCPU5_AON_PER_CLK_RST_CTRL: u32 = 0x2Cu32;
pub const RCPU5_TIMER1_CLK_RST: u32 = 0x4Cu32;
pub const RCPU5_TIMER2_CLK_RST: u32 = 0x70u32;
pub const RCPU5_TIMER3_CLK_RST: u32 = 0x78u32;
pub const RCPU5_TIMER4_CLK_RST: u32 = 0x7Cu32;
pub const RCPU5_GPIO_AND_EDGE_CLK_RST: u32 = 0x74u32;
pub const RCPU5_RCPU_BUS_CLK_CTRL: u32 = 0xC0u32;
pub const RCPU5_RT24_CORE0_CLK_CTRL: u32 = 0xC4u32;
pub const RCPU5_RT24_CORE1_CLK_CTRL: u32 = 0xC8u32;
pub const RCPU5_RT24_CORE0_SW_RESET: u32 = 0xCCu32;
pub const RCPU5_RT24_CORE1_SW_RESET: u32 = 0xD0u32;

/* RCPU PWMCTRL register offsets */
pub const RCPU6_PWM0_CLK_RST: u32 = 0x00u32;
pub const RCPU6_PWM1_CLK_RST: u32 = 0x04u32;
pub const RCPU6_PWM2_CLK_RST: u32 = 0x08u32;
pub const RCPU6_PWM3_CLK_RST: u32 = 0x0cu32;
pub const RCPU6_PWM4_CLK_RST: u32 = 0x10u32;
pub const RCPU6_PWM5_CLK_RST: u32 = 0x14u32;
pub const RCPU6_PWM6_CLK_RST: u32 = 0x18u32;
pub const RCPU6_PWM7_CLK_RST: u32 = 0x1cu32;
pub const RCPU6_PWM8_CLK_RST: u32 = 0x20u32;
pub const RCPU6_PWM9_CLK_RST: u32 = 0x24u32;

/* APBC2 SEC register offsets */
pub const APBC2_UART1_CLK_RST: u32 = 0x00u32;
pub const APBC2_SSP2_CLK_RST: u32 = 0x04u32;
pub const APBC2_TWSI3_CLK_RST: u32 = 0x08u32;
pub const APBC2_RTC_CLK_RST: u32 = 0x0cu32;
pub const APBC2_TIMERS_CLK_RST: u32 = 0x10u32;
pub const APBC2_GPIO_CLK_RST: u32 = 0x1cu32;



// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
