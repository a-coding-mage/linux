/* SPDX-License-Identifier: (GPL-2.0-only OR BSD-2-Clause) */
/*
 * Copyright (c) 2024 Samsung Electronics Co., Ltd.
 * Author: Michal Wilczynski <m.wilczynski@samsung.com>
 */

/* AO Subsystem */
pub mod ao {
    pub const TH1520_RESET_ID_SYSTEM: i32 = 0;
    pub const TH1520_RESET_ID_RTC_APB: i32 = 1;
    pub const TH1520_RESET_ID_RTC_REF: i32 = 2;
    pub const TH1520_RESET_ID_AOGPIO_DB: i32 = 3;
    pub const TH1520_RESET_ID_AOGPIO_APB: i32 = 4;
    pub const TH1520_RESET_ID_AOI2C_APB: i32 = 5;
    pub const TH1520_RESET_ID_PVT_APB: i32 = 6;
    pub const TH1520_RESET_ID_E902_CORE: i32 = 7;
    pub const TH1520_RESET_ID_E902_HAD: i32 = 8;
    pub const TH1520_RESET_ID_AOTIMER_APB: i32 = 9;
    pub const TH1520_RESET_ID_AOTIMER_CORE: i32 = 10;
    pub const TH1520_RESET_ID_AOWDT_APB: i32 = 11;
    pub const TH1520_RESET_ID_APSYS: i32 = 12;
    pub const TH1520_RESET_ID_NPUSYS: i32 = 13;
    pub const TH1520_RESET_ID_DDRSYS: i32 = 14;
    pub const TH1520_RESET_ID_AXI_AP2CP: i32 = 15;
    pub const TH1520_RESET_ID_AXI_CP2AP: i32 = 16;
    pub const TH1520_RESET_ID_AXI_CP2SRAM: i32 = 17;
    pub const TH1520_RESET_ID_AUDSYS_CORE: i32 = 18;
    pub const TH1520_RESET_ID_AUDSYS_IOPMP: i32 = 19;
    pub const TH1520_RESET_ID_AUDSYS: i32 = 20;
    pub const TH1520_RESET_ID_DSP0: i32 = 21;
    pub const TH1520_RESET_ID_DSP1: i32 = 22;
    pub const TH1520_RESET_ID_GPU_MODULE: i32 = 23;
    pub const TH1520_RESET_ID_VDEC: i32 = 24;
    pub const TH1520_RESET_ID_VENC: i32 = 25;
    pub const TH1520_RESET_ID_ADC_APB: i32 = 26;
    pub const TH1520_RESET_ID_AUDGPIO_DB: i32 = 27;
    pub const TH1520_RESET_ID_AUDGPIO_APB: i32 = 28;
    pub const TH1520_RESET_ID_AOUART_IF: i32 = 29;
    pub const TH1520_RESET_ID_AOUART_APB: i32 = 30;
    pub const TH1520_RESET_ID_SRAM_AXI_P0: i32 = 31;
    pub const TH1520_RESET_ID_SRAM_AXI_P1: i32 = 32;
    pub const TH1520_RESET_ID_SRAM_AXI_P2: i32 = 33;
    pub const TH1520_RESET_ID_SRAM_AXI_P3: i32 = 34;
    pub const TH1520_RESET_ID_SRAM_AXI_P4: i32 = 35;
    pub const TH1520_RESET_ID_SRAM_AXI_CORE: i32 = 36;
    pub const TH1520_RESET_ID_SE: i32 = 37;
}

/* AP Subsystem */
pub mod ap {
    pub const TH1520_RESET_ID_BROM: i32 = 0;
    pub const TH1520_RESET_ID_C910_TOP: i32 = 1;
    pub const TH1520_RESET_ID_NPU: i32 = 2;
    pub const TH1520_RESET_ID_WDT0: i32 = 3;
    pub const TH1520_RESET_ID_WDT1: i32 = 4;
    pub const TH1520_RESET_ID_C910_C0: i32 = 5;
    pub const TH1520_RESET_ID_C910_C1: i32 = 6;
    pub const TH1520_RESET_ID_C910_C2: i32 = 7;
    pub const TH1520_RESET_ID_C910_C3: i32 = 8;
    pub const TH1520_RESET_ID_CHIP_DBG_CORE: i32 = 9;
    pub const TH1520_RESET_ID_CHIP_DBG_AXI: i32 = 10;
    pub const TH1520_RESET_ID_AXI4_CPUSYS2_AXI: i32 = 11;
    pub const TH1520_RESET_ID_AXI4_CPUSYS2_APB: i32 = 12;
    pub const TH1520_RESET_ID_X2H_CPUSYS: i32 = 13;
    pub const TH1520_RESET_ID_AHB2_CPUSYS: i32 = 14;
    pub const TH1520_RESET_ID_APB3_CPUSYS: i32 = 15;
    pub const TH1520_RESET_ID_MBOX0_APB: i32 = 16;
    pub const TH1520_RESET_ID_MBOX1_APB: i32 = 17;
    pub const TH1520_RESET_ID_MBOX2_APB: i32 = 18;
    pub const TH1520_RESET_ID_MBOX3_APB: i32 = 19;
    pub const TH1520_RESET_ID_TIMER0_APB: i32 = 20;
    pub const TH1520_RESET_ID_TIMER0_CORE: i32 = 21;
    pub const TH1520_RESET_ID_TIMER1_APB: i32 = 22;
    pub const TH1520_RESET_ID_TIMER1_CORE: i32 = 23;
    pub const TH1520_RESET_ID_PERISYS_AHB: i32 = 24;
    pub const TH1520_RESET_ID_PERISYS_APB1: i32 = 25;
    pub const TH1520_RESET_ID_PERISYS_APB2: i32 = 26;
    pub const TH1520_RESET_ID_GMAC0_APB: i32 = 27;
    pub const TH1520_RESET_ID_GMAC0_AHB: i32 = 28;
    pub const TH1520_RESET_ID_GMAC0_CLKGEN: i32 = 29;
    pub const TH1520_RESET_ID_GMAC0_AXI: i32 = 30;
    pub const TH1520_RESET_ID_UART0_APB: i32 = 31;
    pub const TH1520_RESET_ID_UART0_IF: i32 = 32;
    pub const TH1520_RESET_ID_UART1_APB: i32 = 33;
    pub const TH1520_RESET_ID_UART1_IF: i32 = 34;
    pub const TH1520_RESET_ID_UART2_APB: i32 = 35;
    pub const TH1520_RESET_ID_UART2_IF: i32 = 36;
    pub const TH1520_RESET_ID_UART3_APB: i32 = 37;
    pub const TH1520_RESET_ID_UART3_IF: i32 = 38;
    pub const TH1520_RESET_ID_UART4_APB: i32 = 39;
    pub const TH1520_RESET_ID_UART4_IF: i32 = 40;
    pub const TH1520_RESET_ID_UART5_APB: i32 = 41;
    pub const TH1520_RESET_ID_UART5_IF: i32 = 42;
    pub const TH1520_RESET_ID_QSPI0_IF: i32 = 43;
    pub const TH1520_RESET_ID_QSPI0_APB: i32 = 44;
    pub const TH1520_RESET_ID_QSPI1_IF: i32 = 45;
    pub const TH1520_RESET_ID_QSPI1_APB: i32 = 46;
    pub const TH1520_RESET_ID_SPI_IF: i32 = 47;
    pub const TH1520_RESET_ID_SPI_APB: i32 = 48;
    pub const TH1520_RESET_ID_I2C0_APB: i32 = 49;
    pub const TH1520_RESET_ID_I2C0_CORE: i32 = 50;
    pub const TH1520_RESET_ID_I2C1_APB: i32 = 51;
    pub const TH1520_RESET_ID_I2C1_CORE: i32 = 52;
    pub const TH1520_RESET_ID_I2C2_APB: i32 = 53;
    pub const TH1520_RESET_ID_I2C2_CORE: i32 = 54;
    pub const TH1520_RESET_ID_I2C3_APB: i32 = 55;
    pub const TH1520_RESET_ID_I2C3_CORE: i32 = 56;
    pub const TH1520_RESET_ID_I2C4_APB: i32 = 57;
    pub const TH1520_RESET_ID_I2C4_CORE: i32 = 58;
    pub const TH1520_RESET_ID_I2C5_APB: i32 = 59;
    pub const TH1520_RESET_ID_I2C5_CORE: i32 = 60;
    pub const TH1520_RESET_ID_GPIO0_DB: i32 = 61;
    pub const TH1520_RESET_ID_GPIO0_APB: i32 = 62;
    pub const TH1520_RESET_ID_GPIO1_DB: i32 = 63;
    pub const TH1520_RESET_ID_GPIO1_APB: i32 = 64;
    pub const TH1520_RESET_ID_GPIO2_DB: i32 = 65;
    pub const TH1520_RESET_ID_GPIO2_APB: i32 = 66;
    pub const TH1520_RESET_ID_PWM_COUNTER: i32 = 67;
    pub const TH1520_RESET_ID_PWM_APB: i32 = 68;
    pub const TH1520_RESET_ID_PADCTRL0_APB: i32 = 69;
    pub const TH1520_RESET_ID_CPU2PERI_X2H: i32 = 70;
    pub const TH1520_RESET_ID_CPU2AON_X2H: i32 = 71;
    pub const TH1520_RESET_ID_AON2CPU_A2X: i32 = 72;
    pub const TH1520_RESET_ID_NPUSYS_AXI: i32 = 73;
    pub const TH1520_RESET_ID_NPUSYS_AXI_APB: i32 = 74;
    pub const TH1520_RESET_ID_CPU2VP_X2P: i32 = 75;
    pub const TH1520_RESET_ID_CPU2VI_X2H: i32 = 76;
    pub const TH1520_RESET_ID_BMU_AXI: i32 = 77;
    pub const TH1520_RESET_ID_BMU_APB: i32 = 78;
    pub const TH1520_RESET_ID_DMAC_CPUSYS_AXI: i32 = 79;
    pub const TH1520_RESET_ID_DMAC_CPUSYS_AHB: i32 = 80;
    pub const TH1520_RESET_ID_SPINLOCK: i32 = 81;
    pub const TH1520_RESET_ID_CFG2TEE: i32 = 82;
    pub const TH1520_RESET_ID_DSMART: i32 = 83;
    pub const TH1520_RESET_ID_GPIO3_DB: i32 = 84;
    pub const TH1520_RESET_ID_GPIO3_APB: i32 = 85;
    pub const TH1520_RESET_ID_PERI_I2S: i32 = 86;
    pub const TH1520_RESET_ID_PERI_APB3: i32 = 87;
    pub const TH1520_RESET_ID_PERI2PERI1_APB: i32 = 88;
    pub const TH1520_RESET_ID_VPSYS_APB: i32 = 89;
    pub const TH1520_RESET_ID_PERISYS_APB4: i32 = 90;
    pub const TH1520_RESET_ID_GMAC1_APB: i32 = 91;
    pub const TH1520_RESET_ID_GMAC1_AHB: i32 = 92;
    pub const TH1520_RESET_ID_GMAC1_CLKGEN: i32 = 93;
    pub const TH1520_RESET_ID_GMAC1_AXI: i32 = 94;
    pub const TH1520_RESET_ID_GMAC_AXI: i32 = 95;
    pub const TH1520_RESET_ID_GMAC_AXI_APB: i32 = 96;
    pub const TH1520_RESET_ID_PADCTRL1_APB: i32 = 97;
    pub const TH1520_RESET_ID_VOSYS_AXI: i32 = 98;
    pub const TH1520_RESET_ID_VOSYS_AXI_APB: i32 = 99;
    pub const TH1520_RESET_ID_VOSYS_AXI_X2X: i32 = 100;
    pub const TH1520_RESET_ID_MISC2VP_X2X: i32 = 101;
    pub const TH1520_RESET_ID_DSPSYS: i32 = 102;
    pub const TH1520_RESET_ID_VISYS: i32 = 103;
    pub const TH1520_RESET_ID_VOSYS: i32 = 104;
    pub const TH1520_RESET_ID_VPSYS: i32 = 105;
}

/* DSP Subsystem */
pub mod dsp {
    pub const TH1520_RESET_ID_X2X_DSP1: i32 = 0;
    pub const TH1520_RESET_ID_X2X_DSP0: i32 = 1;
    pub const TH1520_RESET_ID_X2X_SLAVE_DSP1: i32 = 2;
    pub const TH1520_RESET_ID_X2X_SLAVE_DSP0: i32 = 3;
    pub const TH1520_RESET_ID_DSP0_CORE: i32 = 4;
    pub const TH1520_RESET_ID_DSP0_DEBUG: i32 = 5;
    pub const TH1520_RESET_ID_DSP0_APB: i32 = 6;
    pub const TH1520_RESET_ID_DSP1_CORE: i32 = 7;
    pub const TH1520_RESET_ID_DSP1_DEBUG: i32 = 8;
    pub const TH1520_RESET_ID_DSP1_APB: i32 = 9;
    pub const TH1520_RESET_ID_DSPSYS_APB: i32 = 10;
    pub const TH1520_RESET_ID_AXI4_DSPSYS_SLV: i32 = 11;
    pub const TH1520_RESET_ID_AXI4_DSPSYS: i32 = 12;
    pub const TH1520_RESET_ID_AXI4_DSP_RS: i32 = 13;
}

/* MISC Subsystem */
pub mod misc {
    pub const TH1520_RESET_ID_EMMC_SDIO_CLKGEN: i32 = 0;
    pub const TH1520_RESET_ID_EMMC: i32 = 1;
    pub const TH1520_RESET_ID_MISCSYS_AXI: i32 = 2;
    pub const TH1520_RESET_ID_MISCSYS_AXI_APB: i32 = 3;
    pub const TH1520_RESET_ID_SDIO0: i32 = 4;
    pub const TH1520_RESET_ID_SDIO1: i32 = 5;
    pub const TH1520_RESET_ID_USB3_APB: i32 = 6;
    pub const TH1520_RESET_ID_USB3_PHY: i32 = 7;
    pub const TH1520_RESET_ID_USB3_VCC: i32 = 8;
}

/* VI Subsystem */
pub mod vi {
    pub const TH1520_RESET_ID_ISP0: i32 = 0;
    pub const TH1520_RESET_ID_ISP1: i32 = 1;
    pub const TH1520_RESET_ID_CSI0_APB: i32 = 2;
    pub const TH1520_RESET_ID_CSI1_APB: i32 = 3;
    pub const TH1520_RESET_ID_CSI2_APB: i32 = 4;
    pub const TH1520_RESET_ID_MIPI_FIFO: i32 = 5;
    pub const TH1520_RESET_ID_ISP_VENC_APB: i32 = 6;
    pub const TH1520_RESET_ID_VIPRE_APB: i32 = 7;
    pub const TH1520_RESET_ID_VIPRE_AXI: i32 = 8;
    pub const TH1520_RESET_ID_DW200_APB: i32 = 9;
    pub const TH1520_RESET_ID_VISYS3_AXI: i32 = 10;
    pub const TH1520_RESET_ID_VISYS2_AXI: i32 = 11;
    pub const TH1520_RESET_ID_VISYS1_AXI: i32 = 12;
    pub const TH1520_RESET_ID_VISYS_AXI: i32 = 13;
    pub const TH1520_RESET_ID_VISYS_APB: i32 = 14;
    pub const TH1520_RESET_ID_ISP_VENC_AXI: i32 = 15;
}

/* VO Subsystem */
pub mod vo {
    pub const TH1520_RESET_ID_GPU: i32 = 0;
    pub const TH1520_RESET_ID_GPU_CLKGEN: i32 = 1;
    pub const TH1520_RESET_ID_DPU_AHB: i32 = 5;
    pub const TH1520_RESET_ID_DPU_AXI: i32 = 6;
    pub const TH1520_RESET_ID_DPU_CORE: i32 = 7;
    pub const TH1520_RESET_ID_DSI0_APB: i32 = 8;
    pub const TH1520_RESET_ID_DSI1_APB: i32 = 9;
    pub const TH1520_RESET_ID_HDMI: i32 = 10;
    pub const TH1520_RESET_ID_HDMI_APB: i32 = 11;
    pub const TH1520_RESET_ID_VOAXI: i32 = 12;
    pub const TH1520_RESET_ID_VOAXI_APB: i32 = 13;
    pub const TH1520_RESET_ID_X2H_DPU_AXI: i32 = 14;
    pub const TH1520_RESET_ID_X2H_DPU_AHB: i32 = 15;
    pub const TH1520_RESET_ID_X2H_DPU1_AXI: i32 = 16;
    pub const TH1520_RESET_ID_X2H_DPU1_AHB: i32 = 17;
}

/* VP Subsystem */
pub mod vp {
    pub const TH1520_RESET_ID_VPSYS_AXI_APB: i32 = 0;
    pub const TH1520_RESET_ID_VPSYS_AXI: i32 = 1;
    pub const TH1520_RESET_ID_FCE_APB: i32 = 2;
    pub const TH1520_RESET_ID_FCE_CORE: i32 = 3;
    pub const TH1520_RESET_ID_FCE_X2X_MASTER: i32 = 4;
    pub const TH1520_RESET_ID_FCE_X2X_SLAVE: i32 = 5;
    pub const TH1520_RESET_ID_G2D_APB: i32 = 6;
    pub const TH1520_RESET_ID_G2D_ACLK: i32 = 7;
    pub const TH1520_RESET_ID_G2D_CORE: i32 = 8;
    pub const TH1520_RESET_ID_VDEC_APB: i32 = 9;
    pub const TH1520_RESET_ID_VDEC_ACLK: i32 = 10;
    pub const TH1520_RESET_ID_VDEC_CORE: i32 = 11;
    pub const TH1520_RESET_ID_VENC_APB: i32 = 12;
    pub const TH1520_RESET_ID_VENC_CORE: i32 = 13;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
