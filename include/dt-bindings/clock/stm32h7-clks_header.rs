/* SYS, CORE AND BUS CLOCKS */
pub const SYS_D1CPRE: u32 = 0;
pub const HCLK: u32 = 1;
pub const PCLK1: u32 = 2;
pub const PCLK2: u32 = 3;
pub const PCLK3: u32 = 4;
pub const PCLK4: u32 = 5;
pub const HSI_DIV: u32 = 6;
pub const HSE_1M: u32 = 7;
pub const I2S_CKIN: u32 = 8;
pub const CK_DSI_PHY: u32 = 9;
pub const HSE_CK: u32 = 10;
pub const LSE_CK: u32 = 11;
pub const CSI_KER_DIV122: u32 = 12;
pub const RTC_CK: u32 = 13;
pub const CPU_SYSTICK: u32 = 14;

/* OSCILLATOR BANK */
pub const OSC_BANK: u32 = 18;
pub const HSI_CK: u32 = 18;
pub const HSI_KER_CK: u32 = 19;
pub const CSI_CK: u32 = 20;
pub const CSI_KER_CK: u32 = 21;
pub const RC48_CK: u32 = 22;
pub const LSI_CK: u32 = 23;

/* MCLOCK BANK */
pub const MCLK_BANK: u32 = 28;
pub const PER_CK: u32 = 28;
pub const PLLSRC: u32 = 29;
pub const SYS_CK: u32 = 30;
pub const TRACEIN_CK: u32 = 31;

/* ODF BANK */
pub const ODF_BANK: u32 = 32;
pub const PLL1_P: u32 = 32;
pub const PLL1_Q: u32 = 33;
pub const PLL1_R: u32 = 34;
pub const PLL2_P: u32 = 35;
pub const PLL2_Q: u32 = 36;
pub const PLL2_R: u32 = 37;
pub const PLL3_P: u32 = 38;
pub const PLL3_Q: u32 = 39;
pub const PLL3_R: u32 = 40;

/* MCO BANK */
pub const MCO_BANK: u32 = 41;
pub const MCO1: u32 = 41;
pub const MCO2: u32 = 42;

/* PERIF BANK */
pub const PERIF_BANK: u32 = 50;
pub const D1SRAM1_CK: u32 = 50;
pub const ITCM_CK: u32 = 51;
pub const DTCM2_CK: u32 = 52;
pub const DTCM1_CK: u32 = 53;
pub const FLITF_CK: u32 = 54;
pub const JPGDEC_CK: u32 = 55;
pub const DMA2D_CK: u32 = 56;
pub const MDMA_CK: u32 = 57;
pub const USB2ULPI_CK: u32 = 58;
pub const USB1ULPI_CK: u32 = 59;
pub const ETH1RX_CK: u32 = 60;
pub const ETH1TX_CK: u32 = 61;
pub const ETH1MAC_CK: u32 = 62;
pub const ART_CK: u32 = 63;
pub const DMA2_CK: u32 = 64;
pub const DMA1_CK: u32 = 65;
pub const D2SRAM3_CK: u32 = 66;
pub const D2SRAM2_CK: u32 = 67;
pub const D2SRAM1_CK: u32 = 68;
pub const HASH_CK: u32 = 69;
pub const CRYPT_CK: u32 = 70;
pub const CAMITF_CK: u32 = 71;
pub const BKPRAM_CK: u32 = 72;
pub const HSEM_CK: u32 = 73;
pub const BDMA_CK: u32 = 74;
pub const CRC_CK: u32 = 75;
pub const GPIOK_CK: u32 = 76;
pub const GPIOJ_CK: u32 = 77;
pub const GPIOI_CK: u32 = 78;
pub const GPIOH_CK: u32 = 79;
pub const GPIOG_CK: u32 = 80;
pub const GPIOF_CK: u32 = 81;
pub const GPIOE_CK: u32 = 82;
pub const GPIOD_CK: u32 = 83;
pub const GPIOC_CK: u32 = 84;
pub const GPIOB_CK: u32 = 85;
pub const GPIOA_CK: u32 = 86;
pub const WWDG1_CK: u32 = 87;
pub const DAC12_CK: u32 = 88;
pub const WWDG2_CK: u32 = 89;
pub const TIM14_CK: u32 = 90;
pub const TIM13_CK: u32 = 91;
pub const TIM12_CK: u32 = 92;
pub const TIM7_CK: u32 = 93;
pub const TIM6_CK: u32 = 94;
pub const TIM5_CK: u32 = 95;
pub const TIM4_CK: u32 = 96;
pub const TIM3_CK: u32 = 97;
pub const TIM2_CK: u32 = 98;
pub const MDIOS_CK: u32 = 99;
pub const OPAMP_CK: u32 = 100;
pub const CRS_CK: u32 = 101;
pub const TIM17_CK: u32 = 102;
pub const TIM16_CK: u32 = 103;
pub const TIM15_CK: u32 = 104;
pub const TIM8_CK: u32 = 105;
pub const TIM1_CK: u32 = 106;
pub const TMPSENS_CK: u32 = 107;
pub const RTCAPB_CK: u32 = 108;
pub const VREF_CK: u32 = 109;
pub const COMP12_CK: u32 = 110;
pub const SYSCFG_CK: u32 = 111;

/* KERNEL BANK */
pub const KERN_BANK: u32 = 120;
pub const SDMMC1_CK: u32 = 120;
pub const QUADSPI_CK: u32 = 121;
pub const FMC_CK: u32 = 122;
pub const USB2OTG_CK: u32 = 123;
pub const USB1OTG_CK: u32 = 124;
pub const ADC12_CK: u32 = 125;
pub const SDMMC2_CK: u32 = 126;
pub const RNG_CK: u32 = 127;
pub const ADC3_CK: u32 = 128;
pub const DSI_CK: u32 = 129;
pub const LTDC_CK: u32 = 130;
pub const UART8_CK: u32 = 131;
pub const UART7_CK: u32 = 132;
pub const HDMICEC_CK: u32 = 133;
pub const I2C3_CK: u32 = 134;
pub const I2C2_CK: u32 = 135;
pub const I2C1_CK: u32 = 136;
pub const UART5_CK: u32 = 137;
pub const UART4_CK: u32 = 138;
pub const USART3_CK: u32 = 139;
pub const USART2_CK: u32 = 140;
pub const SPDIFRX_CK: u32 = 141;
pub const SPI3_CK: u32 = 142;
pub const SPI2_CK: u32 = 143;
pub const LPTIM1_CK: u32 = 144;
pub const FDCAN_CK: u32 = 145;
pub const SWP_CK: u32 = 146;
pub const HRTIM_CK: u32 = 147;
pub const DFSDM1_CK: u32 = 148;
pub const SAI3_CK: u32 = 149;
pub const SAI2_CK: u32 = 150;
pub const SAI1_CK: u32 = 151;
pub const SPI5_CK: u32 = 152;
pub const SPI4_CK: u32 = 153;
pub const SPI1_CK: u32 = 154;
pub const USART6_CK: u32 = 155;
pub const USART1_CK: u32 = 156;
pub const SAI4B_CK: u32 = 157;
pub const SAI4A_CK: u32 = 158;
pub const LPTIM5_CK: u32 = 159;
pub const LPTIM4_CK: u32 = 160;
pub const LPTIM3_CK: u32 = 161;
pub const LPTIM2_CK: u32 = 162;
pub const I2C4_CK: u32 = 163;
pub const SPI6_CK: u32 = 164;
pub const LPUART1_CK: u32 = 165;

pub const STM32H7_MAX_CLKS: u32 = 166;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
