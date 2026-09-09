// Translated from tegra186-clock.h. C header guards are omitted.
/* SPDX-License-Identifier: GPL-2.0 */
/** @file */

/**
 * @defgroup clock_ids Clock Identifiers
 * @{
 *   @defgroup extern_input external input clocks
 *   @{
 *     @def TEGRA186_CLK_OSC
 *     @def TEGRA186_CLK_CLK_32K
 *     @def TEGRA186_CLK_DTV_INPUT
 *     @def TEGRA186_CLK_SOR0_PAD_CLKOUT
 *     @def TEGRA186_CLK_SOR1_PAD_CLKOUT
 *     @def TEGRA186_CLK_I2S1_SYNC_INPUT
 *     @def TEGRA186_CLK_I2S2_SYNC_INPUT
 *     @def TEGRA186_CLK_I2S3_SYNC_INPUT
 *     @def TEGRA186_CLK_I2S4_SYNC_INPUT
 *     @def TEGRA186_CLK_I2S5_SYNC_INPUT
 *     @def TEGRA186_CLK_I2S6_SYNC_INPUT
 *     @def TEGRA186_CLK_SPDIFIN_SYNC_INPUT
 *   @}
 *
 *   @defgroup extern_output external output clocks
 *   @{
 *     @def TEGRA186_CLK_EXTPERIPH1
 *     @def TEGRA186_CLK_EXTPERIPH2
 *     @def TEGRA186_CLK_EXTPERIPH3
 *     @def TEGRA186_CLK_EXTPERIPH4
 *   @}
 *
 *   @defgroup display_clks display related clocks
 *   @{
 *     @def TEGRA186_CLK_CEC
 *     @def TEGRA186_CLK_DSIC
 *     @def TEGRA186_CLK_DSIC_LP
 *     @def TEGRA186_CLK_DSID
 *     @def TEGRA186_CLK_DSID_LP
 *     @def TEGRA186_CLK_DPAUX1
 *     @def TEGRA186_CLK_DPAUX
 *     @def TEGRA186_CLK_HDA2HDMICODEC
 *     @def TEGRA186_CLK_NVDISPLAY_DISP
 *     @def TEGRA186_CLK_NVDISPLAY_DSC
 *     @def TEGRA186_CLK_NVDISPLAY_P0
 *     @def TEGRA186_CLK_NVDISPLAY_P1
 *     @def TEGRA186_CLK_NVDISPLAY_P2
 *     @def TEGRA186_CLK_NVDISPLAYHUB
 *     @def TEGRA186_CLK_SOR_SAFE
 *     @def TEGRA186_CLK_SOR0
 *     @def TEGRA186_CLK_SOR0_OUT
 *     @def TEGRA186_CLK_SOR1
 *     @def TEGRA186_CLK_SOR1_OUT
 *     @def TEGRA186_CLK_DSI
 *     @def TEGRA186_CLK_MIPI_CAL
 *     @def TEGRA186_CLK_DSIA_LP
 *     @def TEGRA186_CLK_DSIB
 *     @def TEGRA186_CLK_DSIB_LP
 *   @}
 *
 *   @defgroup camera_clks camera related clocks
 *   @{
 *     @def TEGRA186_CLK_NVCSI
 *     @def TEGRA186_CLK_NVCSILP
 *     @def TEGRA186_CLK_VI
 *   @}
 *
 *   @defgroup audio_clks audio related clocks
 *   @{
 *     @def TEGRA186_CLK_ACLK
 *     @def TEGRA186_CLK_ADSP
 *     @def TEGRA186_CLK_ADSPNEON
 *     @def TEGRA186_CLK_AHUB
 *     @def TEGRA186_CLK_APE
 *     @def TEGRA186_CLK_APB2APE
 *     @def TEGRA186_CLK_AUD_MCLK
 *     @def TEGRA186_CLK_DMIC1
 *     @def TEGRA186_CLK_DMIC2
 *     @def TEGRA186_CLK_DMIC3
 *     @def TEGRA186_CLK_DMIC4
 *     @def TEGRA186_CLK_DSPK1
 *     @def TEGRA186_CLK_DSPK2
 *     @def TEGRA186_CLK_HDA
 *     @def TEGRA186_CLK_HDA2CODEC_2X
 *     @def TEGRA186_CLK_I2S1
 *     @def TEGRA186_CLK_I2S2
 *     @def TEGRA186_CLK_I2S3
 *     @def TEGRA186_CLK_I2S4
 *     @def TEGRA186_CLK_I2S5
 *     @def TEGRA186_CLK_I2S6
 *     @def TEGRA186_CLK_MAUD
 *     @def TEGRA186_CLK_PLL_A_OUT0
 *     @def TEGRA186_CLK_SPDIF_DOUBLER
 *     @def TEGRA186_CLK_SPDIF_IN
 *     @def TEGRA186_CLK_SPDIF_OUT
 *     @def TEGRA186_CLK_SYNC_DMIC1
 *     @def TEGRA186_CLK_SYNC_DMIC2
 *     @def TEGRA186_CLK_SYNC_DMIC3
 *     @def TEGRA186_CLK_SYNC_DMIC4
 *     @def TEGRA186_CLK_SYNC_DMIC5
 *     @def TEGRA186_CLK_SYNC_DSPK1
 *     @def TEGRA186_CLK_SYNC_DSPK2
 *     @def TEGRA186_CLK_SYNC_I2S1
 *     @def TEGRA186_CLK_SYNC_I2S2
 *     @def TEGRA186_CLK_SYNC_I2S3
 *     @def TEGRA186_CLK_SYNC_I2S4
 *     @def TEGRA186_CLK_SYNC_I2S5
 *     @def TEGRA186_CLK_SYNC_I2S6
 *     @def TEGRA186_CLK_SYNC_SPDIF
 *   @}
 *
 *   @defgroup uart_clks UART clocks
 *   @{
 *     @def TEGRA186_CLK_AON_UART_FST_MIPI_CAL
 *     @def TEGRA186_CLK_UARTA
 *     @def TEGRA186_CLK_UARTB
 *     @def TEGRA186_CLK_UARTC
 *     @def TEGRA186_CLK_UARTD
 *     @def TEGRA186_CLK_UARTE
 *     @def TEGRA186_CLK_UARTF
 *     @def TEGRA186_CLK_UARTG
 *     @def TEGRA186_CLK_UART_FST_MIPI_CAL
 *   @}
 *
 *   @defgroup i2c_clks I2C clocks
 *   @{
 *     @def TEGRA186_CLK_AON_I2C_SLOW
 *     @def TEGRA186_CLK_I2C1
 *     @def TEGRA186_CLK_I2C2
 *     @def TEGRA186_CLK_I2C3
 *     @def TEGRA186_CLK_I2C4
 *     @def TEGRA186_CLK_I2C5
 *     @def TEGRA186_CLK_I2C6
 *     @def TEGRA186_CLK_I2C8
 *     @def TEGRA186_CLK_I2C9
 *     @def TEGRA186_CLK_I2C1
 *     @def TEGRA186_CLK_I2C12
 *     @def TEGRA186_CLK_I2C13
 *     @def TEGRA186_CLK_I2C14
 *     @def TEGRA186_CLK_I2C_SLOW
 *     @def TEGRA186_CLK_VI_I2C
 *   @}
 *
 *   @defgroup spi_clks SPI clocks
 *   @{
 *     @def TEGRA186_CLK_SPI1
 *     @def TEGRA186_CLK_SPI2
 *     @def TEGRA186_CLK_SPI3
 *     @def TEGRA186_CLK_SPI4
 *   @}
 *
 *   @defgroup storage storage related clocks
 *   @{
 *     @def TEGRA186_CLK_SATA
 *     @def TEGRA186_CLK_SATA_OOB
 *     @def TEGRA186_CLK_SATA_IOBIST
 *     @def TEGRA186_CLK_SDMMC_LEGACY_TM
 *     @def TEGRA186_CLK_SDMMC1
 *     @def TEGRA186_CLK_SDMMC2
 *     @def TEGRA186_CLK_SDMMC3
 *     @def TEGRA186_CLK_SDMMC4
 *     @def TEGRA186_CLK_QSPI
 *     @def TEGRA186_CLK_QSPI_OUT
 *     @def TEGRA186_CLK_UFSDEV_REF
 *     @def TEGRA186_CLK_UFSHC
 *   @}
 *
 *   @defgroup pwm_clks PWM clocks
 *   @{
 *     @def TEGRA186_CLK_PWM1
 *     @def TEGRA186_CLK_PWM2
 *     @def TEGRA186_CLK_PWM3
 *     @def TEGRA186_CLK_PWM4
 *     @def TEGRA186_CLK_PWM5
 *     @def TEGRA186_CLK_PWM6
 *     @def TEGRA186_CLK_PWM7
 *     @def TEGRA186_CLK_PWM8
 *   @}
 *
 *   @defgroup plls PLLs and related clocks
 *   @{
 *     @def TEGRA186_CLK_PLLREFE_OUT_GATED
 *     @def TEGRA186_CLK_PLLREFE_OUT1
 *     @def TEGRA186_CLK_PLLD_OUT1
 *     @def TEGRA186_CLK_PLLP_OUT0
 *     @def TEGRA186_CLK_PLLP_OUT5
 *     @def TEGRA186_CLK_PLLA
 *     @def TEGRA186_CLK_PLLE_PWRSEQ
 *     @def TEGRA186_CLK_PLLA_OUT1
 *     @def TEGRA186_CLK_PLLREFE_REF
 *     @def TEGRA186_CLK_UPHY_PLL0_PWRSEQ
 *     @def TEGRA186_CLK_UPHY_PLL1_PWRSEQ
 *     @def TEGRA186_CLK_PLLREFE_PLLE_PASSTHROUGH
 *     @def TEGRA186_CLK_PLLREFE_PEX
 *     @def TEGRA186_CLK_PLLREFE_IDDQ
 *     @def TEGRA186_CLK_PLLC_OUT_AON
 *     @def TEGRA186_CLK_PLLC_OUT_ISP
 *     @def TEGRA186_CLK_PLLC_OUT_VE
 *     @def TEGRA186_CLK_PLLC4_OUT
 *     @def TEGRA186_CLK_PLLREFE_OUT
 *     @def TEGRA186_CLK_PLLREFE_PLL_REF
 *     @def TEGRA186_CLK_PLLE
 *     @def TEGRA186_CLK_PLLC
 *     @def TEGRA186_CLK_PLLP
 *     @def TEGRA186_CLK_PLLD
 *     @def TEGRA186_CLK_PLLD2
 *     @def TEGRA186_CLK_PLLREFE_VCO
 *     @def TEGRA186_CLK_PLLC2
 *     @def TEGRA186_CLK_PLLC3
 *     @def TEGRA186_CLK_PLLDP
 *     @def TEGRA186_CLK_PLLC4_VCO
 *     @def TEGRA186_CLK_PLLA1
 *     @def TEGRA186_CLK_PLLNVCSI
 *     @def TEGRA186_CLK_PLLDISPHUB
 *     @def TEGRA186_CLK_PLLD3
 *     @def TEGRA186_CLK_PLLBPMPCAM
 *     @def TEGRA186_CLK_PLLAON
 *     @def TEGRA186_CLK_PLLU
 *     @def TEGRA186_CLK_PLLC4_VCO_DIV2
 *     @def TEGRA186_CLK_PLL_REF
 *     @def TEGRA186_CLK_PLLREFE_OUT1_DIV5
 *     @def TEGRA186_CLK_UTMIP_PLL_PWRSEQ
 *     @def TEGRA186_CLK_PLL_U_48M
 *     @def TEGRA186_CLK_PLL_U_480M
 *     @def TEGRA186_CLK_PLLC4_OUT0
 *     @def TEGRA186_CLK_PLLC4_OUT1
 *     @def TEGRA186_CLK_PLLC4_OUT2
 *     @def TEGRA186_CLK_PLLC4_OUT_MUX
 *     @def TEGRA186_CLK_DFLLDISP_DIV
 *     @def TEGRA186_CLK_PLLDISPHUB_DIV
 *     @def TEGRA186_CLK_PLLP_DIV8
 *   @}
 *
 *   @defgroup nafll_clks NAFLL clock sources
 *   @{
 *     @def TEGRA186_CLK_NAFLL_AXI_CBB
 *     @def TEGRA186_CLK_NAFLL_BCPU
 *     @def TEGRA186_CLK_NAFLL_BPMP
 *     @def TEGRA186_CLK_NAFLL_DISP
 *     @def TEGRA186_CLK_NAFLL_GPU
 *     @def TEGRA186_CLK_NAFLL_ISP
 *     @def TEGRA186_CLK_NAFLL_MCPU
 *     @def TEGRA186_CLK_NAFLL_NVDEC
 *     @def TEGRA186_CLK_NAFLL_NVENC
 *     @def TEGRA186_CLK_NAFLL_NVJPG
 *     @def TEGRA186_CLK_NAFLL_SCE
 *     @def TEGRA186_CLK_NAFLL_SE
 *     @def TEGRA186_CLK_NAFLL_TSEC
 *     @def TEGRA186_CLK_NAFLL_TSECB
 *     @def TEGRA186_CLK_NAFLL_VI
 *     @def TEGRA186_CLK_NAFLL_VIC
 *   @}
 *
 *   @defgroup mphy MPHY related clocks
 *   @{
 *     @def TEGRA186_CLK_MPHY_L0_RX_SYMB
 *     @def TEGRA186_CLK_MPHY_L0_RX_LS_BIT
 *     @def TEGRA186_CLK_MPHY_L0_TX_SYMB
 *     @def TEGRA186_CLK_MPHY_L0_TX_LS_3XBIT
 *     @def TEGRA186_CLK_MPHY_L0_RX_ANA
 *     @def TEGRA186_CLK_MPHY_L1_RX_ANA
 *     @def TEGRA186_CLK_MPHY_IOBIST
 *     @def TEGRA186_CLK_MPHY_TX_1MHZ_REF
 *     @def TEGRA186_CLK_MPHY_CORE_PLL_FIXED
 *   @}
 *
 *   @defgroup eavb EAVB related clocks
 *   @{
 *     @def TEGRA186_CLK_EQOS_AXI
 *     @def TEGRA186_CLK_EQOS_PTP_REF
 *     @def TEGRA186_CLK_EQOS_RX
 *     @def TEGRA186_CLK_EQOS_RX_INPUT
 *     @def TEGRA186_CLK_EQOS_TX
 *   @}
 *
 *   @defgroup usb USB related clocks
 *   @{
 *     @def TEGRA186_CLK_PEX_USB_PAD0_MGMT
 *     @def TEGRA186_CLK_PEX_USB_PAD1_MGMT
 *     @def TEGRA186_CLK_HSIC_TRK
 *     @def TEGRA186_CLK_USB2_TRK
 *     @def TEGRA186_CLK_USB2_HSIC_TRK
 *     @def TEGRA186_CLK_XUSB_CORE_SS
 *     @def TEGRA186_CLK_XUSB_CORE_DEV
 *     @def TEGRA186_CLK_XUSB_FALCON
 *     @def TEGRA186_CLK_XUSB_FS
 *     @def TEGRA186_CLK_XUSB
 *     @def TEGRA186_CLK_XUSB_DEV
 *     @def TEGRA186_CLK_XUSB_HOST
 *     @def TEGRA186_CLK_XUSB_SS
 *   @}
 *
 *   @defgroup bigblock compute block related clocks
 *   @{
 *     @def TEGRA186_CLK_GPCCLK
 *     @def TEGRA186_CLK_GPC2CLK
 *     @def TEGRA186_CLK_GPU
 *     @def TEGRA186_CLK_HOST1X
 *     @def TEGRA186_CLK_ISP
 *     @def TEGRA186_CLK_NVDEC
 *     @def TEGRA186_CLK_NVENC
 *     @def TEGRA186_CLK_NVJPG
 *     @def TEGRA186_CLK_SE
 *     @def TEGRA186_CLK_TSEC
 *     @def TEGRA186_CLK_TSECB
 *     @def TEGRA186_CLK_VIC
 *   @}
 *
 *   @defgroup can CAN bus related clocks
 *   @{
 *     @def TEGRA186_CLK_CAN1
 *     @def TEGRA186_CLK_CAN1_HOST
 *     @def TEGRA186_CLK_CAN2
 *     @def TEGRA186_CLK_CAN2_HOST
 *   @}
 *
 *   @defgroup system basic system clocks
 *   @{
 *     @def TEGRA186_CLK_ACTMON
 *     @def TEGRA186_CLK_AON_APB
 *     @def TEGRA186_CLK_AON_CPU_NIC
 *     @def TEGRA186_CLK_AON_NIC
 *     @def TEGRA186_CLK_AXI_CBB
 *     @def TEGRA186_CLK_BPMP_APB
 *     @def TEGRA186_CLK_BPMP_CPU_NIC
 *     @def TEGRA186_CLK_BPMP_NIC_RATE
 *     @def TEGRA186_CLK_CLK_M
 *     @def TEGRA186_CLK_EMC
 *     @def TEGRA186_CLK_MSS_ENCRYPT
 *     @def TEGRA186_CLK_SCE_APB
 *     @def TEGRA186_CLK_SCE_CPU_NIC
 *     @def TEGRA186_CLK_SCE_NIC
 *     @def TEGRA186_CLK_TSC
 *   @}
 *
 *   @defgroup pcie_clks PCIe related clocks
 *   @{
 *     @def TEGRA186_CLK_AFI
 *     @def TEGRA186_CLK_PCIE
 *     @def TEGRA186_CLK_PCIE2_IOBIST
 *     @def TEGRA186_CLK_PCIERX0
 *     @def TEGRA186_CLK_PCIERX1
 *     @def TEGRA186_CLK_PCIERX2
 *     @def TEGRA186_CLK_PCIERX3
 *     @def TEGRA186_CLK_PCIERX4
 *   @}
 */

/** @brief output of gate CLK_ENB_FUSE */
pub const TEGRA186_CLK_FUSE: u32 = 0;
/**
 * @brief It's not what you think
 * @details output of gate CLK_ENB_GPU. This output connects to the GPU
 * pwrclk. @warning: This is almost certainly not the clock you think
 * it is. If you're looking for the clock of the graphics engine, see
 * TEGRA186_GPCCLK
 */
pub const TEGRA186_CLK_GPU: u32 = 1;
/** @brief output of gate CLK_ENB_PCIE */
pub const TEGRA186_CLK_PCIE: u32 = 3;
/** @brief output of the divider IPFS_CLK_DIVISOR */
pub const TEGRA186_CLK_AFI: u32 = 4;
/** @brief output of gate CLK_ENB_PCIE2_IOBIST */
pub const TEGRA186_CLK_PCIE2_IOBIST: u32 = 5;
/** @brief output of gate CLK_ENB_PCIERX0*/
pub const TEGRA186_CLK_PCIERX0: u32 = 6;
/** @brief output of gate CLK_ENB_PCIERX1*/
pub const TEGRA186_CLK_PCIERX1: u32 = 7;
/** @brief output of gate CLK_ENB_PCIERX2*/
pub const TEGRA186_CLK_PCIERX2: u32 = 8;
/** @brief output of gate CLK_ENB_PCIERX3*/
pub const TEGRA186_CLK_PCIERX3: u32 = 9;
/** @brief output of gate CLK_ENB_PCIERX4*/
pub const TEGRA186_CLK_PCIERX4: u32 = 10;
/** @brief output branch of PLL_C for ISP, controlled by gate CLK_ENB_PLLC_OUT_ISP */
pub const TEGRA186_CLK_PLLC_OUT_ISP: u32 = 11;
/** @brief output branch of PLL_C for VI, controlled by gate CLK_ENB_PLLC_OUT_VE */
pub const TEGRA186_CLK_PLLC_OUT_VE: u32 = 12;
/** @brief output branch of PLL_C for AON domain, controlled by gate CLK_ENB_PLLC_OUT_AON */
pub const TEGRA186_CLK_PLLC_OUT_AON: u32 = 13;
/** @brief output of gate CLK_ENB_SOR_SAFE */
pub const TEGRA186_CLK_SOR_SAFE: u32 = 39;
/** @brief output of mux controlled by CLK_RST_CONTROLLER_CLK_SOURCE_I2S2 */
pub const TEGRA186_CLK_I2S2: u32 = 42;
/** @brief output of mux controlled by CLK_RST_CONTROLLER_CLK_SOURCE_I2S3 */
pub const TEGRA186_CLK_I2S3: u32 = 43;
/** @brief output of mux controlled by CLK_RST_CONTROLLER_CLK_SOURCE_SPDF_IN */
pub const TEGRA186_CLK_SPDIF_IN: u32 = 44;
/** @brief output of gate CLK_ENB_SPDIF_DOUBLER */
pub const TEGRA186_CLK_SPDIF_DOUBLER: u32 = 45;
/**  @clkdesc{spi_clks, out, mux, CLK_RST_CONTROLLER_CLK_SOURCE_SPI3} */
pub const TEGRA186_CLK_SPI3: u32 = 46;
/** @clkdesc{i2c_clks, out, mux, CLK_RST_CONTROLLER_CLK_SOURCE_I2C1} */
pub const TEGRA186_CLK_I2C1: u32 = 47;
/** @clkdesc{i2c_clks, out, mux, CLK_RST_CONTROLLER_CLK_SOURCE_I2C5} */
pub const TEGRA186_CLK_I2C5: u32 = 48;
/** @brief output of mux controlled by CLK_RST_CONTROLLER_CLK_SOURCE_SPI1 */
pub const TEGRA186_CLK_SPI1: u32 = 49;
/** @brief output of mux controlled by CLK_RST_CONTROLLER_CLK_SOURCE_ISP */
pub const TEGRA186_CLK_ISP: u32 = 50;
/** @brief output of mux controlled by CLK_RST_CONTROLLER_CLK_SOURCE_VI */
pub const TEGRA186_CLK_VI: u32 = 51;
/** @brief output of mux controlled by CLK_RST_CONTROLLER_CLK_SOURCE_SDMMC1 */
pub const TEGRA186_CLK_SDMMC1: u32 = 52;
/** @brief output of mux controlled by CLK_RST_CONTROLLER_CLK_SOURCE_SDMMC2 */
pub const TEGRA186_CLK_SDMMC2: u32 = 53;
/** @brief output of mux controlled by CLK_RST_CONTROLLER_CLK_SOURCE_SDMMC4 */
pub const TEGRA186_CLK_SDMMC4: u32 = 54;
/** @brief output of mux controlled by CLK_RST_CONTROLLER_CLK_SOURCE_UARTA */
pub const TEGRA186_CLK_UARTA: u32 = 55;
/** @brief output of mux controlled by CLK_RST_CONTROLLER_CLK_SOURCE_UARTB */
pub const TEGRA186_CLK_UARTB: u32 = 56;
/** @brief output of mux controlled by CLK_RST_CONTROLLER_CLK_SOURCE_HOST1X */
pub const TEGRA186_CLK_HOST1X: u32 = 57;
/**
 * @brief controls the EMC clock frequency.
 * @details Doing a clk_set_rate on this clock will select the
 * appropriate clock source, program the source rate and execute a
 * specific sequence to switch to the new clock source for both memory
 * controllers. This can be used to control the balance between memory
 * throughput and memory controller power.
 */
pub const TEGRA186_CLK_EMC: u32 = 58;
/* @brief output of mux controlled by CLK_RST_CONTROLLER_CLK_SOURCE_EXTPERIPH4 */
pub const TEGRA186_CLK_EXTPERIPH4: u32 = 73;
/** @brief output of mux controlled by CLK_RST_CONTROLLER_CLK_SOURCE_SPI4 */
pub const TEGRA186_CLK_SPI4: u32 = 74;
/** @brief output of mux controlled by CLK_RST_CONTROLLER_CLK_SOURCE_I2C3 */
pub const TEGRA186_CLK_I2C3: u32 = 75;
/** @brief output of mux controlled by CLK_RST_CONTROLLER_CLK_SOURCE_SDMMC3 */
pub const TEGRA186_CLK_SDMMC3: u32 = 76;
/** @brief output of mux controlled by CLK_RST_CONTROLLER_CLK_SOURCE_UARTD */
pub const TEGRA186_CLK_UARTD: u32 = 77;
/** output of mux controlled by CLK_RST_CONTROLLER_CLK_SOURCE_I2S1 */
pub const TEGRA186_CLK_I2S1: u32 = 79;
/** output of gate CLK_ENB_DTV */
pub const TEGRA186_CLK_DTV: u32 = 80;
/** output of mux controlled by CLK_RST_CONTROLLER_CLK_SOURCE_TSEC */
pub const TEGRA186_CLK_TSEC: u32 = 81;
/** @brief output of gate CLK_ENB_DP2 */
pub const TEGRA186_CLK_DP2: u32 = 82;
/** output of mux controlled by CLK_RST_CONTROLLER_CLK_SOURCE_I2S4 */
pub const TEGRA186_CLK_I2S4: u32 = 84;
/** output of mux controlled by CLK_RST_CONTROLLER_CLK_SOURCE_I2S5 */
pub const TEGRA186_CLK_I2S5: u32 = 85;
/** output of mux controlled by CLK_RST_CONTROLLER_CLK_SOURCE_I2C4 */
pub const TEGRA186_CLK_I2C4: u32 = 86;
/** output of mux controlled by CLK_RST_CONTROLLER_CLK_SOURCE_AHUB */
pub const TEGRA186_CLK_AHUB: u32 = 87;
/** output of mux controlled by CLK_RST_CONTROLLER_CLK_SOURCE_HDA2CODEC_2X */
pub const TEGRA186_CLK_HDA2CODEC_2X: u32 = 88;
/** @brief output of mux controlled by CLK_RST_CONTROLLER_CLK_SOURCE_EXTPERIPH1 */
pub const TEGRA186_CLK_EXTPERIPH1: u32 = 89;
/** @brief output of mux controlled by CLK_RST_CONTROLLER_CLK_SOURCE_EXTPERIPH2 */
pub const TEGRA186_CLK_EXTPERIPH2: u32 = 90;
/** @brief output of mux controlled by CLK_RST_CONTROLLER_CLK_SOURCE_EXTPERIPH3 */
pub const TEGRA186_CLK_EXTPERIPH3: u32 = 91;
/** @brief output of mux controlled by CLK_RST_CONTROLLER_CLK_SOURCE_I2C_SLOW */
pub const TEGRA186_CLK_I2C_SLOW: u32 = 92;
/** @brief output of the SOR1_CLK_SRC mux in CLK_RST_CONTROLLER_CLK_SOURCE_SOR1 */
pub const TEGRA186_CLK_SOR1: u32 = 93;
/** @brief output of gate CLK_ENB_CEC */
pub const TEGRA186_CLK_CEC: u32 = 94;
/** @brief output of gate CLK_ENB_DPAUX1 */
pub const TEGRA186_CLK_DPAUX1: u32 = 95;
/** @brief output of gate CLK_ENB_DPAUX */
pub const TEGRA186_CLK_DPAUX: u32 = 96;
/** @brief output of the SOR0_CLK_SRC mux in CLK_RST_CONTROLLER_CLK_SOURCE_SOR0 */
pub const TEGRA186_CLK_SOR0: u32 = 97;
/** @brief output of gate CLK_ENB_HDA2HDMICODEC */
pub const TEGRA186_CLK_HDA2HDMICODEC: u32 = 98;
/** @brief output of mux controlled by CLK_RST_CONTROLLER_CLK_SOURCE_SATA */
pub const TEGRA186_CLK_SATA: u32 = 99;
/** @brief output of gate CLK_ENB_SATA_OOB */
pub const TEGRA186_CLK_SATA_OOB: u32 = 100;
/** @brief output of gate CLK_ENB_SATA_IOBIST */
pub const TEGRA186_CLK_SATA_IOBIST: u32 = 101;
/** output of mux controlled by CLK_RST_CONTROLLER_CLK_SOURCE_HDA */
pub const TEGRA186_CLK_HDA: u32 = 102;
/** @brief output of mux controlled by CLK_RST_CONTROLLER_CLK_SOURCE_SE */
pub const TEGRA186_CLK_SE: u32 = 103;
/** @brief output of gate CLK_ENB_APB2APE */
pub const TEGRA186_CLK_APB2APE: u32 = 104;
/** @brief output of mux controlled by CLK_RST_CONTROLLER_CLK_SOURCE_APE */
pub const TEGRA186_CLK_APE: u32 = 105;
/** @brief output of gate CLK_ENB_IQC1 */
pub const TEGRA186_CLK_IQC1: u32 = 106;
/** @brief output of gate CLK_ENB_IQC2 */
pub const TEGRA186_CLK_IQC2: u32 = 107;
/** divide by 2 version of TEGRA186_CLK_PLLREFE_VCO */
pub const TEGRA186_CLK_PLLREFE_OUT: u32 = 108;
/** @brief output of gate CLK_ENB_PLLREFE_PLL_REF */
pub const TEGRA186_CLK_PLLREFE_PLL_REF: u32 = 109;
/** @brief output of gate CLK_ENB_PLLC4_OUT */
pub const TEGRA186_CLK_PLLC4_OUT: u32 = 110;
/** @brief output of mux xusb_core_clk_switch on page 67 of T186_Clocks_IAS.doc */
pub const TEGRA186_CLK_XUSB: u32 = 111;
/** controls xusb_dev_ce signal on page 66 and 67 of T186_Clocks_IAS.doc */
pub const TEGRA186_CLK_XUSB_DEV: u32 = 112;
/** controls xusb_host_ce signal on page 67 of T186_Clocks_IAS.doc */
pub const TEGRA186_CLK_XUSB_HOST: u32 = 113;
/** controls xusb_ss_ce signal on page 67 of T186_Clocks_IAS.doc */
pub const TEGRA186_CLK_XUSB_SS: u32 = 114;
/** @brief output of gate CLK_ENB_DSI */
pub const TEGRA186_CLK_DSI: u32 = 115;
/** @brief output of gate CLK_ENB_MIPI_CAL */
pub const TEGRA186_CLK_MIPI_CAL: u32 = 116;
/** @brief output of mux controlled by CLK_RST_CONTROLLER_CLK_SOURCE_DSIA_LP */
pub const TEGRA186_CLK_DSIA_LP: u32 = 117;
/** @brief output of gate CLK_ENB_DSIB */
pub const TEGRA186_CLK_DSIB: u32 = 118;
/** @brief output of mux controlled by CLK_RST_CONTROLLER_CLK_SOURCE_DSIB_LP */
pub const TEGRA186_CLK_DSIB_LP: u32 = 119;
/** @brief output of mux controlled by CLK_RST_CONTROLLER_CLK_SOURCE_DMIC1 */
pub const TEGRA186_CLK_DMIC1: u32 = 122;
/** @brief output of mux controlled by CLK_RST_CONTROLLER_CLK_SOURCE_DMIC2 */
pub const TEGRA186_CLK_DMIC2: u32 = 123;
/** @brief output of mux controlled by CLK_RST_CONTROLLER_CLK_SOURCE_AUD_MCLK */
pub const TEGRA186_CLK_AUD_MCLK: u32 = 124;
/** @brief output of mux controlled by CLK_RST_CONTROLLER_CLK_SOURCE_I2C6 */
pub const TEGRA186_CLK_I2C6: u32 = 125;
/**output of mux controlled by CLK_RST_CONTROLLER_CLK_SOURCE_UART_FST_MIPI_CAL */
pub const TEGRA186_CLK_UART_FST_MIPI_CAL: u32 = 126;
/** @brief output of mux controlled by CLK_RST_CONTROLLER_CLK_SOURCE_VIC */
pub const TEGRA186_CLK_VIC: u32 = 127;
/** @brief output of mux controlled by CLK_RST_CONTROLLER_CLK_SOURCE_SDMMC_LEGACY_TM */
pub const TEGRA186_CLK_SDMMC_LEGACY_TM: u32 = 128;
/** @brief output of mux controlled by CLK_RST_CONTROLLER_CLK_SOURCE_NVDEC */
pub const TEGRA186_CLK_NVDEC: u32 = 129;
/** @brief output of mux controlled by CLK_RST_CONTROLLER_CLK_SOURCE_NVJPG */
pub const TEGRA186_CLK_NVJPG: u32 = 130;
/** @brief output of mux controlled by CLK_RST_CONTROLLER_CLK_SOURCE_NVENC */
pub const TEGRA186_CLK_NVENC: u32 = 131;
/** @brief output of the QSPI_CLK_SRC mux in CLK_RST_CONTROLLER_CLK_SOURCE_QSPI */
pub const TEGRA186_CLK_QSPI: u32 = 132;
/** @brief output of mux controlled by CLK_RST_CONTROLLER_CLK_SOURCE_VI_I2C */
pub const TEGRA186_CLK_VI_I2C: u32 = 133;
/** @brief output of gate CLK_ENB_HSIC_TRK */
pub const TEGRA186_CLK_HSIC_TRK: u32 = 134;
/** @brief output of gate CLK_ENB_USB2_TRK */
pub const TEGRA186_CLK_USB2_TRK: u32 = 135;
/** output of mux controlled by CLK_RST_CONTROLLER_CLK_SOURCE_MAUD */
pub const TEGRA186_CLK_MAUD: u32 = 136;
/** @brief output of mux controlled by CLK_RST_CONTROLLER_CLK_SOURCE_TSECB */
pub const TEGRA186_CLK_TSECB: u32 = 137;
/** @brief output of gate CLK_ENB_ADSP */
pub const TEGRA186_CLK_ADSP: u32 = 138;
/** @brief output of gate CLK_ENB_ADSPNEON */
pub const TEGRA186_CLK_ADSPNEON: u32 = 139;
/** @brief output of the divider CLK_RST_CONTROLLER_CLK_SOURCE_MPHY_L0_RX_LS_SYMB */
pub const TEGRA186_CLK_MPHY_L0_RX_SYMB: u32 = 140;
/** @brief output of gate CLK_ENB_MPHY_L0_RX_LS_BIT */
pub const TEGRA186_CLK_MPHY_L0_RX_LS_BIT: u32 = 141;
/** @brief output of the divider CLK_RST_CONTROLLER_CLK_SOURCE_MPHY_L0_TX_LS_SYMB */
pub const TEGRA186_CLK_MPHY_L0_TX_SYMB: u32 = 142;
/** @brief output of gate CLK_ENB_MPHY_L0_TX_LS_3XBIT */
pub const TEGRA186_CLK_MPHY_L0_TX_LS_3XBIT: u32 = 143;
/** @brief output of gate CLK_ENB_MPHY_L0_RX_ANA */
pub const TEGRA186_CLK_MPHY_L0_RX_ANA: u32 = 144;
/** @brief output of gate CLK_ENB_MPHY_L1_RX_ANA */
pub const TEGRA186_CLK_MPHY_L1_RX_ANA: u32 = 145;
/** @brief output of the divider CLK_RST_CONTROLLER_CLK_SOURCE_MPHY_IOBIST */
pub const TEGRA186_CLK_MPHY_IOBIST: u32 = 146;
/** @brief output of the divider CLK_RST_CONTROLLER_CLK_SOURCE_MPHY_TX_1MHZ_REF */
pub const TEGRA186_CLK_MPHY_TX_1MHZ_REF: u32 = 147;
/** @brief output of the divider CLK_RST_CONTROLLER_CLK_SOURCE_MPHY_CORE_PLL_FIXED */
pub const TEGRA186_CLK_MPHY_CORE_PLL_FIXED: u32 = 148;
/** @brief output of mux controlled by CLK_RST_CONTROLLER_CLK_SOURCE_AXI_CBB */
pub const TEGRA186_CLK_AXI_CBB: u32 = 149;
/** @brief output of mux controlled by CLK_RST_CONTROLLER_CLK_SOURCE_DMIC3 */
pub const TEGRA186_CLK_DMIC3: u32 = 150;
/** @brief output of mux controlled by CLK_RST_CONTROLLER_CLK_SOURCE_DMIC4 */
pub const TEGRA186_CLK_DMIC4: u32 = 151;
/** @brief output of mux controlled by CLK_RST_CONTROLLER_CLK_SOURCE_DSPK1 */
pub const TEGRA186_CLK_DSPK1: u32 = 152;
/** @brief output of mux controlled by CLK_RST_CONTROLLER_CLK_SOURCE_DSPK2 */
pub const TEGRA186_CLK_DSPK2: u32 = 153;
/** @brief output of mux controlled by CLK_RST_CONTROLLER_CLK_SOURCE_I2C6 */
pub const TEGRA186_CLK_I2S6: u32 = 154;
/** @brief output of mux controlled by CLK_RST_CONTROLLER_CLK_SOURCE_NVDISPLAY_P0 */
pub const TEGRA186_CLK_NVDISPLAY_P0: u32 = 155;
/** @brief output of the NVDISPLAY_DISP_CLK_SRC mux in CLK_RST_CONTROLLER_CLK_SOURCE_NVDISPLAY_DISP */
pub const TEGRA186_CLK_NVDISPLAY_DISP: u32 = 156;
/** @brief output of gate CLK_ENB_NVDISPLAY_DSC */
pub const TEGRA186_CLK_NVDISPLAY_DSC: u32 = 157;
/** @brief output of mux controlled by CLK_RST_CONTROLLER_CLK_SOURCE_NVDISPLAYHUB */
pub const TEGRA186_CLK_NVDISPLAYHUB: u32 = 158;
/** @brief output of mux controlled by CLK_RST_CONTROLLER_CLK_SOURCE_NVDISPLAY_P1 */
pub const TEGRA186_CLK_NVDISPLAY_P1: u32 = 159;
/** @brief output of mux controlled by CLK_RST_CONTROLLER_CLK_SOURCE_NVDISPLAY_P2 */
pub const TEGRA186_CLK_NVDISPLAY_P2: u32 = 160;
/** @brief output of mux controlled by CLK_RST_CONTROLLER_CLK_SOURCE_TACH */
pub const TEGRA186_CLK_TACH: u32 = 166;
/** @brief output of gate CLK_ENB_EQOS */
pub const TEGRA186_CLK_EQOS_AXI: u32 = 167;
/** @brief output of gate CLK_ENB_EQOS_RX */
pub const TEGRA186_CLK_EQOS_RX: u32 = 168;
/** @brief output of mux controlled by CLK_RST_CONTROLLER_CLK_SOURCE_UFSHC_CG_SYS */
pub const TEGRA186_CLK_UFSHC: u32 = 178;
/** @brief output of mux controlled by CLK_RST_CONTROLLER_CLK_SOURCE_UFSDEV_REF */
pub const TEGRA186_CLK_UFSDEV_REF: u32 = 179;
/** @brief output of mux controlled by CLK_RST_CONTROLLER_CLK_SOURCE_NVCSI */
pub const TEGRA186_CLK_NVCSI: u32 = 180;
/** @brief output of mux controlled by CLK_RST_CONTROLLER_CLK_SOURCE_NVCSILP */
pub const TEGRA186_CLK_NVCSILP: u32 = 181;
/** @brief output of mux controlled by CLK_RST_CONTROLLER_CLK_SOURCE_I2C7 */
pub const TEGRA186_CLK_I2C7: u32 = 182;
/** @brief output of mux controlled by CLK_RST_CONTROLLER_CLK_SOURCE_I2C9 */
pub const TEGRA186_CLK_I2C9: u32 = 183;
/** @brief output of mux controlled by CLK_RST_CONTROLLER_CLK_SOURCE_I2C12 */
pub const TEGRA186_CLK_I2C12: u32 = 184;
/** @brief output of mux controlled by CLK_RST_CONTROLLER_CLK_SOURCE_I2C13 */
pub const TEGRA186_CLK_I2C13: u32 = 185;
/** @brief output of mux controlled by CLK_RST_CONTROLLER_CLK_SOURCE_I2C14 */
pub const TEGRA186_CLK_I2C14: u32 = 186;
/** @brief output of mux controlled by CLK_RST_CONTROLLER_CLK_SOURCE_PWM1 */
pub const TEGRA186_CLK_PWM1: u32 = 187;
/** @brief output of mux controlled by CLK_RST_CONTROLLER_CLK_SOURCE_PWM2 */
pub const TEGRA186_CLK_PWM2: u32 = 188;
/** @brief output of mux controlled by CLK_RST_CONTROLLER_CLK_SOURCE_PWM3 */
pub const TEGRA186_CLK_PWM3: u32 = 189;
/** @brief output of mux controlled by CLK_RST_CONTROLLER_CLK_SOURCE_PWM5 */
pub const TEGRA186_CLK_PWM5: u32 = 190;
/** @brief output of mux controlled by CLK_RST_CONTROLLER_CLK_SOURCE_PWM6 */
pub const TEGRA186_CLK_PWM6: u32 = 191;
/** @brief output of mux controlled by CLK_RST_CONTROLLER_CLK_SOURCE_PWM7 */
pub const TEGRA186_CLK_PWM7: u32 = 192;
/** @brief output of mux controlled by CLK_RST_CONTROLLER_CLK_SOURCE_PWM8 */
pub const TEGRA186_CLK_PWM8: u32 = 193;
/** @brief output of mux controlled by CLK_RST_CONTROLLER_CLK_SOURCE_UARTE */
pub const TEGRA186_CLK_UARTE: u32 = 194;
/** @brief output of mux controlled by CLK_RST_CONTROLLER_CLK_SOURCE_UARTF */
pub const TEGRA186_CLK_UARTF: u32 = 195;
/** @deprecated */
pub const TEGRA186_CLK_DBGAPB: u32 = 196;
/** @brief output of mux controlled by CLK_RST_CONTROLLER_CLK_SOURCE_BPMP_CPU_NIC */
pub const TEGRA186_CLK_BPMP_CPU_NIC: u32 = 197;
/** @brief output of mux controlled by CLK_RST_CONTROLLER_CLK_SOURCE_BPMP_APB */
pub const TEGRA186_CLK_BPMP_APB: u32 = 199;
/** @brief output of mux controlled by TEGRA186_CLK_SOC_ACTMON */
pub const TEGRA186_CLK_ACTMON: u32 = 201;
/** @brief output of mux controlled by CLK_RST_CONTROLLER_CLK_SOURCE_AON_CPU_NIC */
pub const TEGRA186_CLK_AON_CPU_NIC: u32 = 208;
/** @brief output of mux controlled by CLK_RST_CONTROLLER_CLK_SOURCE_CAN1 */
pub const TEGRA186_CLK_CAN1: u32 = 210;
/** @brief output of gate CLK_ENB_CAN1_HOST */
pub const TEGRA186_CLK_CAN1_HOST: u32 = 211;
/** @brief output of mux controlled by CLK_RST_CONTROLLER_CLK_SOURCE_CAN2 */
pub const TEGRA186_CLK_CAN2: u32 = 212;
/** @brief output of gate CLK_ENB_CAN2_HOST */
pub const TEGRA186_CLK_CAN2_HOST: u32 = 213;
/** @brief output of mux controlled by CLK_RST_CONTROLLER_CLK_SOURCE_AON_APB */
pub const TEGRA186_CLK_AON_APB: u32 = 214;
/** @brief output of mux controlled by CLK_RST_CONTROLLER_CLK_SOURCE_UARTC */
pub const TEGRA186_CLK_UARTC: u32 = 215;
/** @brief output of mux controlled by CLK_RST_CONTROLLER_CLK_SOURCE_UARTG */
pub const TEGRA186_CLK_UARTG: u32 = 216;
/** @brief output of mux controlled by CLK_RST_CONTROLLER_CLK_SOURCE_AON_UART_FST_MIPI_CAL */
pub const TEGRA186_CLK_AON_UART_FST_MIPI_CAL: u32 = 217;
/** @brief output of mux controlled by CLK_RST_CONTROLLER_CLK_SOURCE_I2C2 */
pub const TEGRA186_CLK_I2C2: u32 = 218;
/** @brief output of mux controlled by CLK_RST_CONTROLLER_CLK_SOURCE_I2C8 */
pub const TEGRA186_CLK_I2C8: u32 = 219;
/** @brief output of mux controlled by CLK_RST_CONTROLLER_CLK_SOURCE_I2C10 */
pub const TEGRA186_CLK_I2C10: u32 = 220;
/** @brief output of mux controlled by CLK_RST_CONTROLLER_CLK_SOURCE_AON_I2C_SLOW */
pub const TEGRA186_CLK_AON_I2C_SLOW: u32 = 221;
/** @brief output of mux controlled by CLK_RST_CONTROLLER_CLK_SOURCE_SPI2 */
pub const TEGRA186_CLK_SPI2: u32 = 222;
/** @brief output of mux controlled by CLK_RST_CONTROLLER_CLK_SOURCE_DMIC5 */
pub const TEGRA186_CLK_DMIC5: u32 = 223;
/** @brief output of mux controlled by CLK_RST_CONTROLLER_CLK_SOURCE_AON_TOUCH */
pub const TEGRA186_CLK_AON_TOUCH: u32 = 224;
/** @brief output of mux controlled by CLK_RST_CONTROLLER_CLK_SOURCE_PWM4 */
pub const TEGRA186_CLK_PWM4: u32 = 225;
/** @brief output of mux controlled by CLK_RST_CONTROLLER_CLK_SOURCE_TSC. This clock object is read only and is used for all timers in the system. */
pub const TEGRA186_CLK_TSC: u32 = 226;
/** @brief output of mux controlled by CLK_RST_CONTROLLER_CLK_SOURCE_MSS_ENCRYPT */
pub const TEGRA186_CLK_MSS_ENCRYPT: u32 = 227;
/** @brief output of mux controlled by CLK_RST_CONTROLLER_CLK_SOURCE_SCE_CPU_NIC */
pub const TEGRA186_CLK_SCE_CPU_NIC: u32 = 228;
/** @brief output of mux controlled by CLK_RST_CONTROLLER_CLK_SOURCE_SCE_APB */
pub const TEGRA186_CLK_SCE_APB: u32 = 230;
/** @brief output of gate CLK_ENB_DSIC */
pub const TEGRA186_CLK_DSIC: u32 = 231;
/** @brief output of mux controlled by CLK_RST_CONTROLLER_CLK_SOURCE_DSIC_LP */
pub const TEGRA186_CLK_DSIC_LP: u32 = 232;
/** @brief output of gate CLK_ENB_DSID */
pub const TEGRA186_CLK_DSID: u32 = 233;
/** @brief output of mux controlled by CLK_RST_CONTROLLER_CLK_SOURCE_DSID_LP */
pub const TEGRA186_CLK_DSID_LP: u32 = 234;
/** @brief output of the divider CLK_RST_CONTROLLER_CLK_SOURCE_PEX_SATA_USB_RX_BYP */
pub const TEGRA186_CLK_PEX_SATA_USB_RX_BYP: u32 = 236;
/** @brief output of mux controlled by CLK_RST_CONTROLLER_CLK_SOURCE_SPDIF_OUT */
pub const TEGRA186_CLK_SPDIF_OUT: u32 = 238;
/** @brief output of the divider CLK_RST_CONTROLLER_CLK_SOURCE_EQOS_PTP_REF_CLK_0 */
pub const TEGRA186_CLK_EQOS_PTP_REF: u32 = 239;
/** @brief output of the divider CLK_RST_CONTROLLER_CLK_SOURCE_EQOS_TX_CLK */
pub const TEGRA186_CLK_EQOS_TX: u32 = 240;
/** @brief output of the divider CLK_RST_CONTROLLER_CLK_SOURCE_USB2_HSIC_TRK */
pub const TEGRA186_CLK_USB2_HSIC_TRK: u32 = 241;
/** @brief output of mux xusb_ss_clk_switch on page 66 of T186_Clocks_IAS.doc */
pub const TEGRA186_CLK_XUSB_CORE_SS: u32 = 242;
/** @brief output of mux xusb_core_dev_clk_switch on page 67 of T186_Clocks_IAS.doc */
pub const TEGRA186_CLK_XUSB_CORE_DEV: u32 = 243;
/** @brief output of mux xusb_core_falcon_clk_switch on page 67 of T186_Clocks_IAS.doc */
pub const TEGRA186_CLK_XUSB_FALCON: u32 = 244;
/** @brief output of mux xusb_fs_clk_switch on page 66 of T186_Clocks_IAS.doc */
pub const TEGRA186_CLK_XUSB_FS: u32 = 245;
/** @brief output of the divider CLK_RST_CONTROLLER_PLLA_OUT */
pub const TEGRA186_CLK_PLL_A_OUT0: u32 = 246;
/** @brief output of mux controlled by CLK_RST_CONTROLLER_AUDIO_SYNC_CLK_I2S1 */
pub const TEGRA186_CLK_SYNC_I2S1: u32 = 247;
/** @brief output of mux controlled by CLK_RST_CONTROLLER_AUDIO_SYNC_CLK_I2S2 */
pub const TEGRA186_CLK_SYNC_I2S2: u32 = 248;
/** @brief output of mux controlled by CLK_RST_CONTROLLER_AUDIO_SYNC_CLK_I2S3 */
pub const TEGRA186_CLK_SYNC_I2S3: u32 = 249;
/** @brief output of mux controlled by CLK_RST_CONTROLLER_AUDIO_SYNC_CLK_I2S4 */
pub const TEGRA186_CLK_SYNC_I2S4: u32 = 250;
/** @brief output of mux controlled by CLK_RST_CONTROLLER_AUDIO_SYNC_CLK_I2S5 */
pub const TEGRA186_CLK_SYNC_I2S5: u32 = 251;
/** @brief output of mux controlled by CLK_RST_CONTROLLER_AUDIO_SYNC_CLK_I2S6 */
pub const TEGRA186_CLK_SYNC_I2S6: u32 = 252;
/** @brief output of mux controlled by CLK_RST_CONTROLLER_AUDIO_SYNC_CLK_DSPK1 */
pub const TEGRA186_CLK_SYNC_DSPK1: u32 = 253;
/** @brief output of mux controlled by CLK_RST_CONTROLLER_AUDIO_SYNC_CLK_DSPK2 */
pub const TEGRA186_CLK_SYNC_DSPK2: u32 = 254;
/** @brief output of mux controlled by CLK_RST_CONTROLLER_AUDIO_SYNC_CLK_DMIC1 */
pub const TEGRA186_CLK_SYNC_DMIC1: u32 = 255;
/** @brief output of mux controlled by CLK_RST_CONTROLLER_AUDIO_SYNC_CLK_DMIC2 */
pub const TEGRA186_CLK_SYNC_DMIC2: u32 = 256;
/** @brief output of mux controlled by CLK_RST_CONTROLLER_AUDIO_SYNC_CLK_DMIC3 */
pub const TEGRA186_CLK_SYNC_DMIC3: u32 = 257;
/** @brief output of mux controlled by CLK_RST_CONTROLLER_AUDIO_SYNC_CLK_DMIC4 */
pub const TEGRA186_CLK_SYNC_DMIC4: u32 = 259;
/** @brief output of mux controlled by CLK_RST_CONTROLLER_AUDIO_SYNC_CLK_SPDIF */
pub const TEGRA186_CLK_SYNC_SPDIF: u32 = 260;
/** @brief output of gate CLK_ENB_PLLREFE_OUT */
pub const TEGRA186_CLK_PLLREFE_OUT_GATED: u32 = 261;
/** @brief output of the divider PLLREFE_DIVP in CLK_RST_CONTROLLER_PLLREFE_BASE. PLLREFE has 2 outputs:
  *      * VCO/pdiv defined by this clock object
  *      * VCO/2 defined by TEGRA186_CLK_PLLREFE_OUT
  */
pub const TEGRA186_CLK_PLLREFE_OUT1: u32 = 262;
pub const TEGRA186_CLK_PLLD_OUT1: u32 = 267;
/** @brief output of the divider PLLP_DIVP in CLK_RST_CONTROLLER_PLLP_BASE */
pub const TEGRA186_CLK_PLLP_OUT0: u32 = 269;
/** @brief output of the divider CLK_RST_CONTROLLER_PLLP_OUTC */
pub const TEGRA186_CLK_PLLP_OUT5: u32 = 270;
/** PLL controlled by CLK_RST_CONTROLLER_PLLA_BASE for use by audio clocks */
pub const TEGRA186_CLK_PLLA: u32 = 271;
/** @brief output of mux controlled by CLK_RST_CONTROLLER_ACLK_BURST_POLICY divided by the divider controlled by ACLK_CLK_DIVISOR in CLK_RST_CONTROLLER_SUPER_ACLK_DIVIDER */
pub const TEGRA186_CLK_ACLK: u32 = 273;
/** fixed 48MHz clock divided down from TEGRA186_CLK_PLL_U */
pub const TEGRA186_CLK_PLL_U_48M: u32 = 274;
/** fixed 480MHz clock divided down from TEGRA186_CLK_PLL_U */
pub const TEGRA186_CLK_PLL_U_480M: u32 = 275;
/** @brief output of the divider PLLC4_DIVP in CLK_RST_CONTROLLER_PLLC4_BASE. Output frequency is TEGRA186_CLK_PLLC4_VCO/PLLC4_DIVP */
pub const TEGRA186_CLK_PLLC4_OUT0: u32 = 276;
/** fixed /3 divider. Output frequency of this clock is TEGRA186_CLK_PLLC4_VCO/3 */
pub const TEGRA186_CLK_PLLC4_OUT1: u32 = 277;
/** fixed /5 divider. Output frequency of this clock is TEGRA186_CLK_PLLC4_VCO/5 */
pub const TEGRA186_CLK_PLLC4_OUT2: u32 = 278;
/** @brief output of mux controlled by PLLC4_CLK_SEL in CLK_RST_CONTROLLER_PLLC4_MISC1 */
pub const TEGRA186_CLK_PLLC4_OUT_MUX: u32 = 279;
/** @brief output of divider NVDISPLAY_DISP_CLK_DIVISOR in CLK_RST_CONTROLLER_CLK_SOURCE_NVDISPLAY_DISP when DFLLDISP_DIV is selected in NVDISPLAY_DISP_CLK_SRC */
pub const TEGRA186_CLK_DFLLDISP_DIV: u32 = 284;
/** @brief output of divider NVDISPLAY_DISP_CLK_DIVISOR in CLK_RST_CONTROLLER_CLK_SOURCE_NVDISPLAY_DISP when PLLDISPHUB_DIV is selected in NVDISPLAY_DISP_CLK_SRC */
pub const TEGRA186_CLK_PLLDISPHUB_DIV: u32 = 285;
/** fixed /8 divider which is used as the input for TEGRA186_CLK_SOR_SAFE */
pub const TEGRA186_CLK_PLLP_DIV8: u32 = 286;
/** @brief output of divider CLK_RST_CONTROLLER_BPMP_NIC_RATE */
pub const TEGRA186_CLK_BPMP_NIC: u32 = 287;
/** @brief output of the divider CLK_RST_CONTROLLER_PLLA1_OUT1 */
pub const TEGRA186_CLK_PLL_A_OUT1: u32 = 288;
/** @deprecated */
pub const TEGRA186_CLK_GPC2CLK: u32 = 289;
/** A fake clock which must be enabled during KFUSE read operations to ensure adequate VDD_CORE voltage. */
pub const TEGRA186_CLK_KFUSE: u32 = 293;
/**
 * @brief controls the PLLE hardware sequencer.
 * @details This clock only has enable and disable methods. When the
 * PLLE hw sequencer is enabled, PLLE, will be enabled or disabled by
 * hw based on the control signals from the PCIe, SATA and XUSB
 * clocks. When the PLLE hw sequencer is disabled, the state of PLLE
 * is controlled by sw using clk_enable/clk_disable on
 * TEGRA186_CLK_PLLE.
 */
pub const TEGRA186_CLK_PLLE_PWRSEQ: u32 = 294;
/** fixed 60MHz clock divided down from, TEGRA186_CLK_PLL_U */
pub const TEGRA186_CLK_PLLREFE_REF: u32 = 295;
/** @brief output of mux controlled by SOR0_CLK_SEL0 and SOR0_CLK_SEL1 in CLK_RST_CONTROLLER_CLK_SOURCE_SOR0 */
pub const TEGRA186_CLK_SOR0_OUT: u32 = 296;
/** @brief output of mux controlled by SOR1_CLK_SEL0 and SOR1_CLK_SEL1 in CLK_RST_CONTROLLER_CLK_SOURCE_SOR1 */
pub const TEGRA186_CLK_SOR1_OUT: u32 = 297;
/** @brief fixed /5 divider.  Output frequency of this clock is TEGRA186_CLK_PLLREFE_OUT1/5. Used as input for TEGRA186_CLK_EQOS_AXI */
pub const TEGRA186_CLK_PLLREFE_OUT1_DIV5: u32 = 298;
/** @brief controls the UTMIP_PLL (aka PLLU) hardware sqeuencer */
pub const TEGRA186_CLK_UTMIP_PLL_PWRSEQ: u32 = 301;
/** @brief output of the divider CLK_RST_CONTROLLER_CLK_SOURCE_PEX_USB_PAD_PLL0_MGMT */
pub const TEGRA186_CLK_PEX_USB_PAD0_MGMT: u32 = 302;
/** @brief output of the divider CLK_RST_CONTROLLER_CLK_SOURCE_PEX_USB_PAD_PLL1_MGMT */
pub const TEGRA186_CLK_PEX_USB_PAD1_MGMT: u32 = 303;
/** @brief controls the UPHY_PLL0 hardware sqeuencer */
pub const TEGRA186_CLK_UPHY_PLL0_PWRSEQ: u32 = 304;
/** @brief controls the UPHY_PLL1 hardware sqeuencer */
pub const TEGRA186_CLK_UPHY_PLL1_PWRSEQ: u32 = 305;
/** @brief control for PLLREFE_IDDQ in CLK_RST_CONTROLLER_PLLREFE_MISC so the bypass output even be used when the PLL is disabled */
pub const TEGRA186_CLK_PLLREFE_PLLE_PASSTHROUGH: u32 = 306;
/** @brief output of the mux controlled by PLLREFE_SEL_CLKIN_PEX in CLK_RST_CONTROLLER_PLLREFE_MISC */
pub const TEGRA186_CLK_PLLREFE_PEX: u32 = 307;
/** @brief control for PLLREFE_IDDQ in CLK_RST_CONTROLLER_PLLREFE_MISC to turn on the PLL when enabled */
pub const TEGRA186_CLK_PLLREFE_IDDQ: u32 = 308;
/** @brief output of the divider QSPI_CLK_DIV2_SEL in CLK_RST_CONTROLLER_CLK_SOURCE_QSPI */
pub const TEGRA186_CLK_QSPI_OUT: u32 = 309;
/**
 * @brief GPC2CLK-div-2
 * @details fixed /2 divider. Output frequency is
 * TEGRA186_CLK_GPC2CLK/2. The frequency of this clock is the
 * frequency at which the GPU graphics engine runs. */
pub const TEGRA186_CLK_GPCCLK: u32 = 310;
/** @brief output of divider CLK_RST_CONTROLLER_AON_NIC_RATE */
pub const TEGRA186_CLK_AON_NIC: u32 = 450;
/** @brief output of divider CLK_RST_CONTROLLER_SCE_NIC_RATE */
pub const TEGRA186_CLK_SCE_NIC: u32 = 451;
/** Fixed 100MHz PLL for PCIe, SATA and superspeed USB */
pub const TEGRA186_CLK_PLLE: u32 = 512;
/** @brief PLL controlled by CLK_RST_CONTROLLER_PLLC_BASE */
pub const TEGRA186_CLK_PLLC: u32 = 513;
/** Fixed 408MHz PLL for use by peripheral clocks */
pub const TEGRA186_CLK_PLLP: u32 = 516;
/** @deprecated */
pub const TEGRA186_CLK_PLL_P: u32 = TEGRA186_CLK_PLLP;
/** @brief PLL controlled by CLK_RST_CONTROLLER_PLLD_BASE for use by DSI */
pub const TEGRA186_CLK_PLLD: u32 = 518;
/** @brief PLL controlled by CLK_RST_CONTROLLER_PLLD2_BASE for use by HDMI or DP */
pub const TEGRA186_CLK_PLLD2: u32 = 519;
/**
 * @brief PLL controlled by CLK_RST_CONTROLLER_PLLREFE_BASE.
 * @details Note that this clock only controls the VCO output, before
 * the post-divider. See TEGRA186_CLK_PLLREFE_OUT1 for more
 * information.
 */
pub const TEGRA186_CLK_PLLREFE_VCO: u32 = 520;
/** @brief PLL controlled by CLK_RST_CONTROLLER_PLLC2_BASE */
pub const TEGRA186_CLK_PLLC2: u32 = 521;
/** @brief PLL controlled by CLK_RST_CONTROLLER_PLLC3_BASE */
pub const TEGRA186_CLK_PLLC3: u32 = 522;
/** @brief PLL controlled by CLK_RST_CONTROLLER_PLLDP_BASE for use as the DP link clock */
pub const TEGRA186_CLK_PLLDP: u32 = 523;
/** @brief PLL controlled by CLK_RST_CONTROLLER_PLLC4_BASE */
pub const TEGRA186_CLK_PLLC4_VCO: u32 = 524;
/** @brief PLL controlled by CLK_RST_CONTROLLER_PLLA1_BASE for use by audio clocks */
pub const TEGRA186_CLK_PLLA1: u32 = 525;
/** @brief PLL controlled by CLK_RST_CONTROLLER_PLLNVCSI_BASE */
pub const TEGRA186_CLK_PLLNVCSI: u32 = 526;
/** @brief PLL controlled by CLK_RST_CONTROLLER_PLLDISPHUB_BASE */
pub const TEGRA186_CLK_PLLDISPHUB: u32 = 527;
/** @brief PLL controlled by CLK_RST_CONTROLLER_PLLD3_BASE for use by HDMI or DP */
pub const TEGRA186_CLK_PLLD3: u32 = 528;
/** @brief PLL controlled by CLK_RST_CONTROLLER_PLLBPMPCAM_BASE */
pub const TEGRA186_CLK_PLLBPMPCAM: u32 = 531;
/** @brief PLL controlled by CLK_RST_CONTROLLER_PLLAON_BASE for use by IP blocks in the AON domain */
pub const TEGRA186_CLK_PLLAON: u32 = 532;
/** Fixed frequency 960MHz PLL for USB and EAVB */
pub const TEGRA186_CLK_PLLU: u32 = 533;
/** fixed /2 divider. Output frequency is TEGRA186_CLK_PLLC4_VCO/2 */
pub const TEGRA186_CLK_PLLC4_VCO_DIV2: u32 = 535;
/** @brief NAFLL clock source for AXI_CBB */
pub const TEGRA186_CLK_NAFLL_AXI_CBB: u32 = 564;
/** @brief NAFLL clock source for BPMP */
pub const TEGRA186_CLK_NAFLL_BPMP: u32 = 565;
/** @brief NAFLL clock source for ISP */
pub const TEGRA186_CLK_NAFLL_ISP: u32 = 566;
/** @brief NAFLL clock source for NVDEC */
pub const TEGRA186_CLK_NAFLL_NVDEC: u32 = 567;
/** @brief NAFLL clock source for NVENC */
pub const TEGRA186_CLK_NAFLL_NVENC: u32 = 568;
/** @brief NAFLL clock source for NVJPG */
pub const TEGRA186_CLK_NAFLL_NVJPG: u32 = 569;
/** @brief NAFLL clock source for SCE */
pub const TEGRA186_CLK_NAFLL_SCE: u32 = 570;
/** @brief NAFLL clock source for SE */
pub const TEGRA186_CLK_NAFLL_SE: u32 = 571;
/** @brief NAFLL clock source for TSEC */
pub const TEGRA186_CLK_NAFLL_TSEC: u32 = 572;
/** @brief NAFLL clock source for TSECB */
pub const TEGRA186_CLK_NAFLL_TSECB: u32 = 573;
/** @brief NAFLL clock source for VI */
pub const TEGRA186_CLK_NAFLL_VI: u32 = 574;
/** @brief NAFLL clock source for VIC */
pub const TEGRA186_CLK_NAFLL_VIC: u32 = 575;
/** @brief NAFLL clock source for DISP */
pub const TEGRA186_CLK_NAFLL_DISP: u32 = 576;
/** @brief NAFLL clock source for GPU */
pub const TEGRA186_CLK_NAFLL_GPU: u32 = 577;
/** @brief NAFLL clock source for M-CPU cluster */
pub const TEGRA186_CLK_NAFLL_MCPU: u32 = 578;
/** @brief NAFLL clock source for B-CPU cluster */
pub const TEGRA186_CLK_NAFLL_BCPU: u32 = 579;
/** @brief input from Tegra's CLK_32K_IN pad */
pub const TEGRA186_CLK_CLK_32K: u32 = 608;
/** @brief output of divider CLK_RST_CONTROLLER_CLK_M_DIVIDE */
pub const TEGRA186_CLK_CLK_M: u32 = 609;
/** @brief output of divider PLL_REF_DIV in CLK_RST_CONTROLLER_OSC_CTRL */
pub const TEGRA186_CLK_PLL_REF: u32 = 610;
/** @brief input from Tegra's XTAL_IN */
pub const TEGRA186_CLK_OSC: u32 = 612;
/** @brief clock recovered from EAVB input */
pub const TEGRA186_CLK_EQOS_RX_INPUT: u32 = 613;
/** @brief clock recovered from DTV input */
pub const TEGRA186_CLK_DTV_INPUT: u32 = 614;
/** @brief SOR0 brick output which feeds into SOR0_CLK_SEL mux in CLK_RST_CONTROLLER_CLK_SOURCE_SOR0*/
pub const TEGRA186_CLK_SOR0_PAD_CLKOUT: u32 = 615;
/** @brief SOR1 brick output which feeds into SOR1_CLK_SEL mux in CLK_RST_CONTROLLER_CLK_SOURCE_SOR1*/
pub const TEGRA186_CLK_SOR1_PAD_CLKOUT: u32 = 616;
/** @brief clock recovered from I2S1 input */
pub const TEGRA186_CLK_I2S1_SYNC_INPUT: u32 = 617;
/** @brief clock recovered from I2S2 input */
pub const TEGRA186_CLK_I2S2_SYNC_INPUT: u32 = 618;
/** @brief clock recovered from I2S3 input */
pub const TEGRA186_CLK_I2S3_SYNC_INPUT: u32 = 619;
/** @brief clock recovered from I2S4 input */
pub const TEGRA186_CLK_I2S4_SYNC_INPUT: u32 = 620;
/** @brief clock recovered from I2S5 input */
pub const TEGRA186_CLK_I2S5_SYNC_INPUT: u32 = 621;
/** @brief clock recovered from I2S6 input */
pub const TEGRA186_CLK_I2S6_SYNC_INPUT: u32 = 622;
/** @brief clock recovered from SPDIFIN input */
pub const TEGRA186_CLK_SPDIFIN_SYNC_INPUT: u32 = 623;

/**
 * @brief subject to change
 * @details maximum clock identifier value plus one.
 */
pub const TEGRA186_CLK_CLK_MAX: u32 = 624;

/** @} */



// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
