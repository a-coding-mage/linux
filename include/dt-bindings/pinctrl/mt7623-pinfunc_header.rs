/* SPDX-License-Identifier: GPL-2.0 */
#define __DTS_MT7623_PINFUNC_H

// External dependency: <dt-bindings/pinctrl/mt65xx.h>

pub const MT7623_PIN_0_PWRAP_SPI0_MI_FUNC_GPIO0: u32 = (mtk_pin_no!(0) | 0);
pub const MT7623_PIN_0_PWRAP_SPI0_MI_FUNC_PWRAP_SPIDO: u32 = (mtk_pin_no!(0) | 1);
pub const MT7623_PIN_0_PWRAP_SPI0_MI_FUNC_PWRAP_SPIDI: u32 = (mtk_pin_no!(0) | 2);

pub const MT7623_PIN_1_PWRAP_SPI0_MO_FUNC_GPIO1: u32 = (mtk_pin_no!(1) | 0);
pub const MT7623_PIN_1_PWRAP_SPI0_MO_FUNC_PWRAP_SPIDI: u32 = (mtk_pin_no!(1) | 1);
pub const MT7623_PIN_1_PWRAP_SPI0_MO_FUNC_PWRAP_SPIDO: u32 = (mtk_pin_no!(1) | 2);

pub const MT7623_PIN_2_PWRAP_INT_FUNC_GPIO2: u32 = (mtk_pin_no!(2) | 0);
pub const MT7623_PIN_2_PWRAP_INT_FUNC_PWRAP_INT: u32 = (mtk_pin_no!(2) | 1);

pub const MT7623_PIN_3_PWRAP_SPI0_CK_FUNC_GPIO3: u32 = (mtk_pin_no!(3) | 0);
pub const MT7623_PIN_3_PWRAP_SPI0_CK_FUNC_PWRAP_SPICK_I: u32 = (mtk_pin_no!(3) | 1);

pub const MT7623_PIN_4_PWRAP_SPI0_CSN_FUNC_GPIO4: u32 = (mtk_pin_no!(4) | 0);
pub const MT7623_PIN_4_PWRAP_SPI0_CSN_FUNC_PWRAP_SPICS_B_I: u32 = (mtk_pin_no!(4) | 1);

pub const MT7623_PIN_5_PWRAP_SPI0_CK2_FUNC_GPIO5: u32 = (mtk_pin_no!(5) | 0);
pub const MT7623_PIN_5_PWRAP_SPI0_CK2_FUNC_PWRAP_SPICK2_I: u32 = (mtk_pin_no!(5) | 1);
pub const MT7623_PIN_5_PWRAP_SPI0_CK2_FUNC_ANT_SEL1: u32 = (mtk_pin_no!(5) | 5);

pub const MT7623_PIN_6_PWRAP_SPI0_CSN2_FUNC_GPIO6: u32 = (mtk_pin_no!(6) | 0);
pub const MT7623_PIN_6_PWRAP_SPI0_CSN2_FUNC_PWRAP_SPICS2_B_I: u32 = (mtk_pin_no!(6) | 1);
pub const MT7623_PIN_6_PWRAP_SPI0_CSN2_FUNC_ANT_SEL0: u32 = (mtk_pin_no!(6) | 5);

pub const MT7623_PIN_7_SPI1_CSN_FUNC_GPIO7: u32 = (mtk_pin_no!(7) | 0);
pub const MT7623_PIN_7_SPI1_CSN_FUNC_SPI1_CS: u32 = (mtk_pin_no!(7) | 1);
pub const MT7623_PIN_7_SPI1_CSN_FUNC_KCOL0: u32 = (mtk_pin_no!(7) | 4);

pub const MT7623_PIN_8_SPI1_MI_FUNC_GPIO8: u32 = (mtk_pin_no!(8) | 0);
pub const MT7623_PIN_8_SPI1_MI_FUNC_SPI1_MI: u32 = (mtk_pin_no!(8) | 1);
pub const MT7623_PIN_8_SPI1_MI_FUNC_SPI1_MO: u32 = (mtk_pin_no!(8) | 2);
pub const MT7623_PIN_8_SPI1_MI_FUNC_KCOL1: u32 = (mtk_pin_no!(8) | 4);

pub const MT7623_PIN_9_SPI1_MO_FUNC_GPIO9: u32 = (mtk_pin_no!(9) | 0);
pub const MT7623_PIN_9_SPI1_MO_FUNC_SPI1_MO: u32 = (mtk_pin_no!(9) | 1);
pub const MT7623_PIN_9_SPI1_MO_FUNC_SPI1_MI: u32 = (mtk_pin_no!(9) | 2);
pub const MT7623_PIN_9_SPI1_MO_FUNC_EXT_FRAME_SYNC: u32 = (mtk_pin_no!(9) | 3);
pub const MT7623_PIN_9_SPI1_MO_FUNC_KCOL2: u32 = (mtk_pin_no!(9) | 4);

pub const MT7623_PIN_10_RTC32K_CK_FUNC_GPIO10: u32 = (mtk_pin_no!(10) | 0);
pub const MT7623_PIN_10_RTC32K_CK_FUNC_RTC32K_CK: u32 = (mtk_pin_no!(10) | 1);

pub const MT7623_PIN_11_WATCHDOG_FUNC_GPIO11: u32 = (mtk_pin_no!(11) | 0);
pub const MT7623_PIN_11_WATCHDOG_FUNC_WATCHDOG: u32 = (mtk_pin_no!(11) | 1);

pub const MT7623_PIN_12_SRCLKENA_FUNC_GPIO12: u32 = (mtk_pin_no!(12) | 0);
pub const MT7623_PIN_12_SRCLKENA_FUNC_SRCLKENA: u32 = (mtk_pin_no!(12) | 1);

pub const MT7623_PIN_13_SRCLKENAI_FUNC_GPIO13: u32 = (mtk_pin_no!(13) | 0);
pub const MT7623_PIN_13_SRCLKENAI_FUNC_SRCLKENAI: u32 = (mtk_pin_no!(13) | 1);

pub const MT7623_PIN_14_GPIO14_FUNC_GPIO14: u32 = (mtk_pin_no!(14) | 0);
pub const MT7623_PIN_14_GPIO14_FUNC_URXD2: u32 = (mtk_pin_no!(14) | 1);
pub const MT7623_PIN_14_GPIO14_FUNC_UTXD2: u32 = (mtk_pin_no!(14) | 2);
pub const MT7623_PIN_14_GPIO14_FUNC_SRCCLKENAI2: u32 = (mtk_pin_no!(14) | 5);

pub const MT7623_PIN_15_GPIO15_FUNC_GPIO15: u32 = (mtk_pin_no!(15) | 0);
pub const MT7623_PIN_15_GPIO15_FUNC_UTXD2: u32 = (mtk_pin_no!(15) | 1);
pub const MT7623_PIN_15_GPIO15_FUNC_URXD2: u32 = (mtk_pin_no!(15) | 2);

pub const MT7623_PIN_18_PCM_CLK_FUNC_GPIO18: u32 = (mtk_pin_no!(18) | 0);
pub const MT7623_PIN_18_PCM_CLK_FUNC_PCM_CLK0: u32 = (mtk_pin_no!(18) | 1);
pub const MT7623_PIN_18_PCM_CLK_FUNC_MRG_CLK: u32 = (mtk_pin_no!(18) | 2);
pub const MT7623_PIN_18_PCM_CLK_FUNC_MM_TEST_CK: u32 = (mtk_pin_no!(18) | 4);
pub const MT7623_PIN_18_PCM_CLK_FUNC_CONN_DSP_JCK: u32 = (mtk_pin_no!(18) | 5);
pub const MT7623_PIN_18_PCM_CLK_FUNC_AP_PCM_CLKO: u32 = (mtk_pin_no!(18) | 6);

pub const MT7623_PIN_19_PCM_SYNC_FUNC_GPIO19: u32 = (mtk_pin_no!(19) | 0);
pub const MT7623_PIN_19_PCM_SYNC_FUNC_PCM_SYNC: u32 = (mtk_pin_no!(19) | 1);
pub const MT7623_PIN_19_PCM_SYNC_FUNC_MRG_SYNC: u32 = (mtk_pin_no!(19) | 2);
pub const MT7623_PIN_19_PCM_SYNC_FUNC_CONN_DSP_JINTP: u32 = (mtk_pin_no!(19) | 5);
pub const MT7623_PIN_19_PCM_SYNC_FUNC_AP_PCM_SYNC: u32 = (mtk_pin_no!(19) | 6);

pub const MT7623_PIN_20_PCM_RX_FUNC_GPIO20: u32 = (mtk_pin_no!(20) | 0);
pub const MT7623_PIN_20_PCM_RX_FUNC_PCM_RX: u32 = (mtk_pin_no!(20) | 1);
pub const MT7623_PIN_20_PCM_RX_FUNC_MRG_RX: u32 = (mtk_pin_no!(20) | 2);
pub const MT7623_PIN_20_PCM_RX_FUNC_MRG_TX: u32 = (mtk_pin_no!(20) | 3);
pub const MT7623_PIN_20_PCM_RX_FUNC_PCM_TX: u32 = (mtk_pin_no!(20) | 4);
pub const MT7623_PIN_20_PCM_RX_FUNC_CONN_DSP_JDI: u32 = (mtk_pin_no!(20) | 5);
pub const MT7623_PIN_20_PCM_RX_FUNC_AP_PCM_RX: u32 = (mtk_pin_no!(20) | 6);

pub const MT7623_PIN_21_PCM_TX_FUNC_GPIO21: u32 = (mtk_pin_no!(21) | 0);
pub const MT7623_PIN_21_PCM_TX_FUNC_PCM_TX: u32 = (mtk_pin_no!(21) | 1);
pub const MT7623_PIN_21_PCM_TX_FUNC_MRG_TX: u32 = (mtk_pin_no!(21) | 2);
pub const MT7623_PIN_21_PCM_TX_FUNC_MRG_RX: u32 = (mtk_pin_no!(21) | 3);
pub const MT7623_PIN_21_PCM_TX_FUNC_PCM_RX: u32 = (mtk_pin_no!(21) | 4);
pub const MT7623_PIN_21_PCM_TX_FUNC_CONN_DSP_JMS: u32 = (mtk_pin_no!(21) | 5);
pub const MT7623_PIN_21_PCM_TX_FUNC_AP_PCM_TX: u32 = (mtk_pin_no!(21) | 6);

pub const MT7623_PIN_22_EINT0_FUNC_GPIO22: u32 = (mtk_pin_no!(22) | 0);
pub const MT7623_PIN_22_EINT0_FUNC_UCTS0: u32 = (mtk_pin_no!(22) | 1);
pub const MT7623_PIN_22_EINT0_FUNC_PCIE0_PERST_N: u32 = (mtk_pin_no!(22) | 2);
pub const MT7623_PIN_22_EINT0_FUNC_KCOL3: u32 = (mtk_pin_no!(22) | 3);
pub const MT7623_PIN_22_EINT0_FUNC_CONN_DSP_JDO: u32 = (mtk_pin_no!(22) | 4);
pub const MT7623_PIN_22_EINT0_FUNC_EXT_FRAME_SYNC: u32 = (mtk_pin_no!(22) | 5);

pub const MT7623_PIN_23_EINT1_FUNC_GPIO23: u32 = (mtk_pin_no!(23) | 0);
pub const MT7623_PIN_23_EINT1_FUNC_URTS0: u32 = (mtk_pin_no!(23) | 1);
pub const MT7623_PIN_23_EINT1_FUNC_PCIE1_PERST_N: u32 = (mtk_pin_no!(23) | 2);
pub const MT7623_PIN_23_EINT1_FUNC_KCOL2: u32 = (mtk_pin_no!(23) | 3);
pub const MT7623_PIN_23_EINT1_FUNC_CONN_MCU_TDO: u32 = (mtk_pin_no!(23) | 4);
pub const MT7623_PIN_23_EINT1_FUNC_EXT_FRAME_SYNC: u32 = (mtk_pin_no!(23) | 5);

pub const MT7623_PIN_24_EINT2_FUNC_GPIO24: u32 = (mtk_pin_no!(24) | 0);
pub const MT7623_PIN_24_EINT2_FUNC_UCTS1: u32 = (mtk_pin_no!(24) | 1);
pub const MT7623_PIN_24_EINT2_FUNC_PCIE2_PERST_N: u32 = (mtk_pin_no!(24) | 2);
pub const MT7623_PIN_24_EINT2_FUNC_KCOL1: u32 = (mtk_pin_no!(24) | 3);
pub const MT7623_PIN_24_EINT2_FUNC_CONN_MCU_DBGACK_N: u32 = (mtk_pin_no!(24) | 4);

pub const MT7623_PIN_25_EINT3_FUNC_GPIO25: u32 = (mtk_pin_no!(25) | 0);
pub const MT7623_PIN_25_EINT3_FUNC_URTS1: u32 = (mtk_pin_no!(25) | 1);
pub const MT7623_PIN_25_EINT3_FUNC_KCOL0: u32 = (mtk_pin_no!(25) | 3);
pub const MT7623_PIN_25_EINT3_FUNC_CONN_MCU_DBGI_N: u32 = (mtk_pin_no!(25) | 4);

pub const MT7623_PIN_26_EINT4_FUNC_GPIO26: u32 = (mtk_pin_no!(26) | 0);
pub const MT7623_PIN_26_EINT4_FUNC_UCTS3: u32 = (mtk_pin_no!(26) | 1);
pub const MT7623_PIN_26_EINT4_FUNC_DRV_VBUS_P1: u32 = (mtk_pin_no!(26) | 2);
pub const MT7623_PIN_26_EINT4_FUNC_KROW3: u32 = (mtk_pin_no!(26) | 3);
pub const MT7623_PIN_26_EINT4_FUNC_CONN_MCU_TCK0: u32 = (mtk_pin_no!(26) | 4);
pub const MT7623_PIN_26_EINT4_FUNC_CONN_MCU_AICE_JCKC: u32 = (mtk_pin_no!(26) | 5);
pub const MT7623_PIN_26_EINT4_FUNC_PCIE2_WAKE_N: u32 = (mtk_pin_no!(26) | 6);

pub const MT7623_PIN_27_EINT5_FUNC_GPIO27: u32 = (mtk_pin_no!(27) | 0);
pub const MT7623_PIN_27_EINT5_FUNC_URTS3: u32 = (mtk_pin_no!(27) | 1);
pub const MT7623_PIN_27_EINT5_FUNC_IDDIG_P1: u32 = (mtk_pin_no!(27) | 2);
pub const MT7623_PIN_27_EINT5_FUNC_KROW2: u32 = (mtk_pin_no!(27) | 3);
pub const MT7623_PIN_27_EINT5_FUNC_CONN_MCU_TDI: u32 = (mtk_pin_no!(27) | 4);
pub const MT7623_PIN_27_EINT5_FUNC_PCIE1_WAKE_N: u32 = (mtk_pin_no!(27) | 6);

pub const MT7623_PIN_28_EINT6_FUNC_GPIO28: u32 = (mtk_pin_no!(28) | 0);
pub const MT7623_PIN_28_EINT6_FUNC_DRV_VBUS: u32 = (mtk_pin_no!(28) | 1);
pub const MT7623_PIN_28_EINT6_FUNC_KROW1: u32 = (mtk_pin_no!(28) | 3);
pub const MT7623_PIN_28_EINT6_FUNC_CONN_MCU_TRST_B: u32 = (mtk_pin_no!(28) | 4);
pub const MT7623_PIN_28_EINT6_FUNC_PCIE0_WAKE_N: u32 = (mtk_pin_no!(28) | 6);

pub const MT7623_PIN_29_EINT7_FUNC_GPIO29: u32 = (mtk_pin_no!(29) | 0);
pub const MT7623_PIN_29_EINT7_FUNC_IDDIG: u32 = (mtk_pin_no!(29) | 1);
pub const MT7623_PIN_29_EINT7_FUNC_MSDC1_WP: u32 = (mtk_pin_no!(29) | 2);
pub const MT7623_PIN_29_EINT7_FUNC_KROW0: u32 = (mtk_pin_no!(29) | 3);
pub const MT7623_PIN_29_EINT7_FUNC_CONN_MCU_TMS: u32 = (mtk_pin_no!(29) | 4);
pub const MT7623_PIN_29_EINT7_FUNC_CONN_MCU_AICE_JMSC: u32 = (mtk_pin_no!(29) | 5);
pub const MT7623_PIN_29_EINT7_FUNC_PCIE2_PERST_N: u32 = (mtk_pin_no!(29) | 6);

pub const MT7623_PIN_33_I2S1_DATA_FUNC_GPIO33: u32 = (mtk_pin_no!(33) | 0);
pub const MT7623_PIN_33_I2S1_DATA_FUNC_I2S1_DATA: u32 = (mtk_pin_no!(33) | 1);
pub const MT7623_PIN_33_I2S1_DATA_FUNC_I2S1_DATA_BYPS: u32 = (mtk_pin_no!(33) | 2);
pub const MT7623_PIN_33_I2S1_DATA_FUNC_PCM_TX: u32 = (mtk_pin_no!(33) | 3);
pub const MT7623_PIN_33_I2S1_DATA_FUNC_IMG_TEST_CK: u32 = (mtk_pin_no!(33) | 4);
pub const MT7623_PIN_33_I2S1_DATA_FUNC_G1_RXD0: u32 = (mtk_pin_no!(33) | 5);
pub const MT7623_PIN_33_I2S1_DATA_FUNC_AP_PCM_TX: u32 = (mtk_pin_no!(33) | 6);

pub const MT7623_PIN_34_I2S1_DATA_IN_FUNC_GPIO34: u32 = (mtk_pin_no!(34) | 0);
pub const MT7623_PIN_34_I2S1_DATA_IN_FUNC_I2S1_DATA_IN: u32 = (mtk_pin_no!(34) | 1);
pub const MT7623_PIN_34_I2S1_DATA_IN_FUNC_PCM_RX: u32 = (mtk_pin_no!(34) | 3);
pub const MT7623_PIN_34_I2S1_DATA_IN_FUNC_VDEC_TEST_CK: u32 = (mtk_pin_no!(34) | 4);
pub const MT7623_PIN_34_I2S1_DATA_IN_FUNC_G1_RXD1: u32 = (mtk_pin_no!(34) | 5);
pub const MT7623_PIN_34_I2S1_DATA_IN_FUNC_AP_PCM_RX: u32 = (mtk_pin_no!(34) | 6);

pub const MT7623_PIN_35_I2S1_BCK_FUNC_GPIO35: u32 = (mtk_pin_no!(35) | 0);
pub const MT7623_PIN_35_I2S1_BCK_FUNC_I2S1_BCK: u32 = (mtk_pin_no!(35) | 1);
pub const MT7623_PIN_35_I2S1_BCK_FUNC_PCM_CLK0: u32 = (mtk_pin_no!(35) | 3);
pub const MT7623_PIN_35_I2S1_BCK_FUNC_G1_RXD2: u32 = (mtk_pin_no!(35) | 5);
pub const MT7623_PIN_35_I2S1_BCK_FUNC_AP_PCM_CLKO: u32 = (mtk_pin_no!(35) | 6);

pub const MT7623_PIN_36_I2S1_LRCK_FUNC_GPIO36: u32 = (mtk_pin_no!(36) | 0);
pub const MT7623_PIN_36_I2S1_LRCK_FUNC_I2S1_LRCK: u32 = (mtk_pin_no!(36) | 1);
pub const MT7623_PIN_36_I2S1_LRCK_FUNC_PCM_SYNC: u32 = (mtk_pin_no!(36) | 3);
pub const MT7623_PIN_36_I2S1_LRCK_FUNC_G1_RXD3: u32 = (mtk_pin_no!(36) | 5);
pub const MT7623_PIN_36_I2S1_LRCK_FUNC_AP_PCM_SYNC: u32 = (mtk_pin_no!(36) | 6);

pub const MT7623_PIN_37_I2S1_MCLK_FUNC_GPIO37: u32 = (mtk_pin_no!(37) | 0);
pub const MT7623_PIN_37_I2S1_MCLK_FUNC_I2S1_MCLK: u32 = (mtk_pin_no!(37) | 1);
pub const MT7623_PIN_37_I2S1_MCLK_FUNC_G1_RXDV: u32 = (mtk_pin_no!(37) | 5);

pub const MT7623_PIN_39_JTMS_FUNC_GPIO39: u32 = (mtk_pin_no!(39) | 0);
pub const MT7623_PIN_39_JTMS_FUNC_JTMS: u32 = (mtk_pin_no!(39) | 1);
pub const MT7623_PIN_39_JTMS_FUNC_CONN_MCU_TMS: u32 = (mtk_pin_no!(39) | 2);
pub const MT7623_PIN_39_JTMS_FUNC_CONN_MCU_AICE_JMSC: u32 = (mtk_pin_no!(39) | 3);
pub const MT7623_PIN_39_JTMS_FUNC_DFD_TMS_XI: u32 = (mtk_pin_no!(39) | 4);

pub const MT7623_PIN_40_JTCK_FUNC_GPIO40: u32 = (mtk_pin_no!(40) | 0);
pub const MT7623_PIN_40_JTCK_FUNC_JTCK: u32 = (mtk_pin_no!(40) | 1);
pub const MT7623_PIN_40_JTCK_FUNC_CONN_MCU_TCK1: u32 = (mtk_pin_no!(40) | 2);
pub const MT7623_PIN_40_JTCK_FUNC_CONN_MCU_AICE_JCKC: u32 = (mtk_pin_no!(40) | 3);
pub const MT7623_PIN_40_JTCK_FUNC_DFD_TCK_XI: u32 = (mtk_pin_no!(40) | 4);

pub const MT7623_PIN_41_JTDI_FUNC_GPIO41: u32 = (mtk_pin_no!(41) | 0);
pub const MT7623_PIN_41_JTDI_FUNC_JTDI: u32 = (mtk_pin_no!(41) | 1);
pub const MT7623_PIN_41_JTDI_FUNC_CONN_MCU_TDI: u32 = (mtk_pin_no!(41) | 2);
pub const MT7623_PIN_41_JTDI_FUNC_DFD_TDI_XI: u32 = (mtk_pin_no!(41) | 4);

pub const MT7623_PIN_42_JTDO_FUNC_GPIO42: u32 = (mtk_pin_no!(42) | 0);
pub const MT7623_PIN_42_JTDO_FUNC_JTDO: u32 = (mtk_pin_no!(42) | 1);
pub const MT7623_PIN_42_JTDO_FUNC_CONN_MCU_TDO: u32 = (mtk_pin_no!(42) | 2);
pub const MT7623_PIN_42_JTDO_FUNC_DFD_TDO: u32 = (mtk_pin_no!(42) | 4);

pub const MT7623_PIN_43_NCLE_FUNC_GPIO43: u32 = (mtk_pin_no!(43) | 0);
pub const MT7623_PIN_43_NCLE_FUNC_NCLE: u32 = (mtk_pin_no!(43) | 1);
pub const MT7623_PIN_43_NCLE_FUNC_EXT_XCS2: u32 = (mtk_pin_no!(43) | 2);

pub const MT7623_PIN_44_NCEB1_FUNC_GPIO44: u32 = (mtk_pin_no!(44) | 0);
pub const MT7623_PIN_44_NCEB1_FUNC_NCEB1: u32 = (mtk_pin_no!(44) | 1);
pub const MT7623_PIN_44_NCEB1_FUNC_IDDIG: u32 = (mtk_pin_no!(44) | 2);

pub const MT7623_PIN_45_NCEB0_FUNC_GPIO45: u32 = (mtk_pin_no!(45) | 0);
pub const MT7623_PIN_45_NCEB0_FUNC_NCEB0: u32 = (mtk_pin_no!(45) | 1);
pub const MT7623_PIN_45_NCEB0_FUNC_DRV_VBUS: u32 = (mtk_pin_no!(45) | 2);

pub const MT7623_PIN_46_IR_FUNC_GPIO46: u32 = (mtk_pin_no!(46) | 0);
pub const MT7623_PIN_46_IR_FUNC_IR: u32 = (mtk_pin_no!(46) | 1);

pub const MT7623_PIN_47_NREB_FUNC_GPIO47: u32 = (mtk_pin_no!(47) | 0);
pub const MT7623_PIN_47_NREB_FUNC_NREB: u32 = (mtk_pin_no!(47) | 1);
pub const MT7623_PIN_47_NREB_FUNC_IDDIG_P1: u32 = (mtk_pin_no!(47) | 2);

pub const MT7623_PIN_48_NRNB_FUNC_GPIO48: u32 = (mtk_pin_no!(48) | 0);
pub const MT7623_PIN_48_NRNB_FUNC_NRNB: u32 = (mtk_pin_no!(48) | 1);
pub const MT7623_PIN_48_NRNB_FUNC_DRV_VBUS_P1: u32 = (mtk_pin_no!(48) | 2);

pub const MT7623_PIN_49_I2S0_DATA_FUNC_GPIO49: u32 = (mtk_pin_no!(49) | 0);
pub const MT7623_PIN_49_I2S0_DATA_FUNC_I2S0_DATA: u32 = (mtk_pin_no!(49) | 1);
pub const MT7623_PIN_49_I2S0_DATA_FUNC_I2S0_DATA_BYPS: u32 = (mtk_pin_no!(49) | 2);
pub const MT7623_PIN_49_I2S0_DATA_FUNC_PCM_TX: u32 = (mtk_pin_no!(49) | 3);
pub const MT7623_PIN_49_I2S0_DATA_FUNC_AP_I2S_DO: u32 = (mtk_pin_no!(49) | 6);

pub const MT7623_PIN_53_SPI0_CSN_FUNC_GPIO53: u32 = (mtk_pin_no!(53) | 0);
pub const MT7623_PIN_53_SPI0_CSN_FUNC_SPI0_CS: u32 = (mtk_pin_no!(53) | 1);
pub const MT7623_PIN_53_SPI0_CSN_FUNC_SPDIF: u32 = (mtk_pin_no!(53) | 3);
pub const MT7623_PIN_53_SPI0_CSN_FUNC_ADC_CK: u32 = (mtk_pin_no!(53) | 4);
pub const MT7623_PIN_53_SPI0_CSN_FUNC_PWM1: u32 = (mtk_pin_no!(53) | 5);

pub const MT7623_PIN_54_SPI0_CK_FUNC_GPIO54: u32 = (mtk_pin_no!(54) | 0);
pub const MT7623_PIN_54_SPI0_CK_FUNC_SPI0_CK: u32 = (mtk_pin_no!(54) | 1);
pub const MT7623_PIN_54_SPI0_CK_FUNC_SPDIF_IN1: u32 = (mtk_pin_no!(54) | 3);
pub const MT7623_PIN_54_SPI0_CK_FUNC_ADC_DAT_IN: u32 = (mtk_pin_no!(54) | 4);

pub const MT7623_PIN_55_SPI0_MI_FUNC_GPIO55: u32 = (mtk_pin_no!(55) | 0);
pub const MT7623_PIN_55_SPI0_MI_FUNC_SPI0_MI: u32 = (mtk_pin_no!(55) | 1);
pub const MT7623_PIN_55_SPI0_MI_FUNC_SPI0_MO: u32 = (mtk_pin_no!(55) | 2);
pub const MT7623_PIN_55_SPI0_MI_FUNC_MSDC1_WP: u32 = (mtk_pin_no!(55) | 3);
pub const MT7623_PIN_55_SPI0_MI_FUNC_ADC_WS: u32 = (mtk_pin_no!(55) | 4);
pub const MT7623_PIN_55_SPI0_MI_FUNC_PWM2: u32 = (mtk_pin_no!(55) | 5);

pub const MT7623_PIN_56_SPI0_MO_FUNC_GPIO56: u32 = (mtk_pin_no!(56) | 0);
pub const MT7623_PIN_56_SPI0_MO_FUNC_SPI0_MO: u32 = (mtk_pin_no!(56) | 1);
pub const MT7623_PIN_56_SPI0_MO_FUNC_SPI0_MI: u32 = (mtk_pin_no!(56) | 2);
pub const MT7623_PIN_56_SPI0_MO_FUNC_SPDIF_IN0: u32 = (mtk_pin_no!(56) | 3);

pub const MT7623_PIN_57_SDA1_FUNC_GPIO57: u32 = (mtk_pin_no!(57) | 0);
pub const MT7623_PIN_57_SDA1_FUNC_SDA1: u32 = (mtk_pin_no!(57) | 1);

pub const MT7623_PIN_58_SCL1_FUNC_GPIO58: u32 = (mtk_pin_no!(58) | 0);
pub const MT7623_PIN_58_SCL1_FUNC_SCL1: u32 = (mtk_pin_no!(58) | 1);

pub const MT7623_PIN_60_WB_RSTB_FUNC_GPIO60: u32 = (mtk_pin_no!(60) | 0);
pub const MT7623_PIN_60_WB_RSTB_FUNC_WB_RSTB: u32 = (mtk_pin_no!(60) | 1);

pub const MT7623_PIN_61_GPIO61_FUNC_GPIO61: u32 = (mtk_pin_no!(61) | 0);
pub const MT7623_PIN_61_GPIO61_FUNC_TEST_FD: u32 = (mtk_pin_no!(61) | 1);

pub const MT7623_PIN_62_GPIO62_FUNC_GPIO62: u32 = (mtk_pin_no!(62) | 0);
pub const MT7623_PIN_62_GPIO62_FUNC_TEST_FC: u32 = (mtk_pin_no!(62) | 1);

pub const MT7623_PIN_63_WB_SCLK_FUNC_GPIO63: u32 = (mtk_pin_no!(63) | 0);
pub const MT7623_PIN_63_WB_SCLK_FUNC_WB_SCLK: u32 = (mtk_pin_no!(63) | 1);

pub const MT7623_PIN_64_WB_SDATA_FUNC_GPIO64: u32 = (mtk_pin_no!(64) | 0);
pub const MT7623_PIN_64_WB_SDATA_FUNC_WB_SDATA: u32 = (mtk_pin_no!(64) | 1);

pub const MT7623_PIN_65_WB_SEN_FUNC_GPIO65: u32 = (mtk_pin_no!(65) | 0);
pub const MT7623_PIN_65_WB_SEN_FUNC_WB_SEN: u32 = (mtk_pin_no!(65) | 1);

pub const MT7623_PIN_66_WB_CRTL0_FUNC_GPIO66: u32 = (mtk_pin_no!(66) | 0);
pub const MT7623_PIN_66_WB_CRTL0_FUNC_WB_CRTL0: u32 = (mtk_pin_no!(66) | 1);

pub const MT7623_PIN_67_WB_CRTL1_FUNC_GPIO67: u32 = (mtk_pin_no!(67) | 0);
pub const MT7623_PIN_67_WB_CRTL1_FUNC_WB_CRTL1: u32 = (mtk_pin_no!(67) | 1);

pub const MT7623_PIN_68_WB_CRTL2_FUNC_GPIO68: u32 = (mtk_pin_no!(68) | 0);
pub const MT7623_PIN_68_WB_CRTL2_FUNC_WB_CRTL2: u32 = (mtk_pin_no!(68) | 1);

pub const MT7623_PIN_69_WB_CRTL3_FUNC_GPIO69: u32 = (mtk_pin_no!(69) | 0);
pub const MT7623_PIN_69_WB_CRTL3_FUNC_WB_CRTL3: u32 = (mtk_pin_no!(69) | 1);

pub const MT7623_PIN_70_WB_CRTL4_FUNC_GPIO70: u32 = (mtk_pin_no!(70) | 0);
pub const MT7623_PIN_70_WB_CRTL4_FUNC_WB_CRTL4: u32 = (mtk_pin_no!(70) | 1);

pub const MT7623_PIN_71_WB_CRTL5_FUNC_GPIO71: u32 = (mtk_pin_no!(71) | 0);
pub const MT7623_PIN_71_WB_CRTL5_FUNC_WB_CRTL5: u32 = (mtk_pin_no!(71) | 1);

pub const MT7623_PIN_72_I2S0_DATA_IN_FUNC_GPIO72: u32 = (mtk_pin_no!(72) | 0);
pub const MT7623_PIN_72_I2S0_DATA_IN_FUNC_I2S0_DATA_IN: u32 = (mtk_pin_no!(72) | 1);
pub const MT7623_PIN_72_I2S0_DATA_IN_FUNC_PCM_RX: u32 = (mtk_pin_no!(72) | 3);
pub const MT7623_PIN_72_I2S0_DATA_IN_FUNC_PWM0: u32 = (mtk_pin_no!(72) | 4);
pub const MT7623_PIN_72_I2S0_DATA_IN_FUNC_DISP_PWM: u32 = (mtk_pin_no!(72) | 5);
pub const MT7623_PIN_72_I2S0_DATA_IN_FUNC_AP_I2S_DI: u32 = (mtk_pin_no!(72) | 6);

pub const MT7623_PIN_73_I2S0_LRCK_FUNC_GPIO73: u32 = (mtk_pin_no!(73) | 0);
pub const MT7623_PIN_73_I2S0_LRCK_FUNC_I2S0_LRCK: u32 = (mtk_pin_no!(73) | 1);
pub const MT7623_PIN_73_I2S0_LRCK_FUNC_PCM_SYNC: u32 = (mtk_pin_no!(73) | 3);
pub const MT7623_PIN_73_I2S0_LRCK_FUNC_AP_I2S_LRCK: u32 = (mtk_pin_no!(73) | 6);

pub const MT7623_PIN_74_I2S0_BCK_FUNC_GPIO74: u32 = (mtk_pin_no!(74) | 0);
pub const MT7623_PIN_74_I2S0_BCK_FUNC_I2S0_BCK: u32 = (mtk_pin_no!(74) | 1);
pub const MT7623_PIN_74_I2S0_BCK_FUNC_PCM_CLK0: u32 = (mtk_pin_no!(74) | 3);
pub const MT7623_PIN_74_I2S0_BCK_FUNC_AP_I2S_BCK: u32 = (mtk_pin_no!(74) | 6);

pub const MT7623_PIN_75_SDA0_FUNC_GPIO75: u32 = (mtk_pin_no!(75) | 0);
pub const MT7623_PIN_75_SDA0_FUNC_SDA0: u32 = (mtk_pin_no!(75) | 1);

pub const MT7623_PIN_76_SCL0_FUNC_GPIO76: u32 = (mtk_pin_no!(76) | 0);
pub const MT7623_PIN_76_SCL0_FUNC_SCL0: u32 = (mtk_pin_no!(76) | 1);

pub const MT7623_PIN_77_SDA2_FUNC_GPIO77: u32 = (mtk_pin_no!(77) | 0);
pub const MT7623_PIN_77_SDA2_FUNC_SDA2: u32 = (mtk_pin_no!(77) | 1);

pub const MT7623_PIN_78_SCL2_FUNC_GPIO78: u32 = (mtk_pin_no!(78) | 0);
pub const MT7623_PIN_78_SCL2_FUNC_SCL2: u32 = (mtk_pin_no!(78) | 1);

pub const MT7623_PIN_79_URXD0_FUNC_GPIO79: u32 = (mtk_pin_no!(79) | 0);
pub const MT7623_PIN_79_URXD0_FUNC_URXD0: u32 = (mtk_pin_no!(79) | 1);
pub const MT7623_PIN_79_URXD0_FUNC_UTXD0: u32 = (mtk_pin_no!(79) | 2);

pub const MT7623_PIN_80_UTXD0_FUNC_GPIO80: u32 = (mtk_pin_no!(80) | 0);
pub const MT7623_PIN_80_UTXD0_FUNC_UTXD0: u32 = (mtk_pin_no!(80) | 1);
pub const MT7623_PIN_80_UTXD0_FUNC_URXD0: u32 = (mtk_pin_no!(80) | 2);

pub const MT7623_PIN_81_URXD1_FUNC_GPIO81: u32 = (mtk_pin_no!(81) | 0);
pub const MT7623_PIN_81_URXD1_FUNC_URXD1: u32 = (mtk_pin_no!(81) | 1);
pub const MT7623_PIN_81_URXD1_FUNC_UTXD1: u32 = (mtk_pin_no!(81) | 2);

pub const MT7623_PIN_82_UTXD1_FUNC_GPIO82: u32 = (mtk_pin_no!(82) | 0);
pub const MT7623_PIN_82_UTXD1_FUNC_UTXD1: u32 = (mtk_pin_no!(82) | 1);
pub const MT7623_PIN_82_UTXD1_FUNC_URXD1: u32 = (mtk_pin_no!(82) | 2);

pub const MT7623_PIN_83_LCM_RST_FUNC_GPIO83: u32 = (mtk_pin_no!(83) | 0);
pub const MT7623_PIN_83_LCM_RST_FUNC_LCM_RST: u32 = (mtk_pin_no!(83) | 1);
pub const MT7623_PIN_83_LCM_RST_FUNC_VDAC_CK_XI: u32 = (mtk_pin_no!(83) | 2);

pub const MT7623_PIN_84_DSI_TE_FUNC_GPIO84: u32 = (mtk_pin_no!(84) | 0);
pub const MT7623_PIN_84_DSI_TE_FUNC_DSI_TE: u32 = (mtk_pin_no!(84) | 1);

pub const MT7623_PIN_91_MIPI_TDN3_FUNC_GPIO91: u32 = (mtk_pin_no!(91) | 0);
pub const MT7623_PIN_91_MIPI_TDN3_FUNC_TDN3: u32 = (mtk_pin_no!(91) | 1);

pub const MT7623_PIN_92_MIPI_TDP3_FUNC_GPIO92: u32 = (mtk_pin_no!(92) | 0);
pub const MT7623_PIN_92_MIPI_TDP3_FUNC_TDP3: u32 = (mtk_pin_no!(92) | 1);

pub const MT7623_PIN_93_MIPI_TDN2_FUNC_GPIO93: u32 = (mtk_pin_no!(93) | 0);
pub const MT7623_PIN_93_MIPI_TDN2_FUNC_TDN2: u32 = (mtk_pin_no!(93) | 1);

pub const MT7623_PIN_94_MIPI_TDP2_FUNC_GPIO94: u32 = (mtk_pin_no!(94) | 0);
pub const MT7623_PIN_94_MIPI_TDP2_FUNC_TDP2: u32 = (mtk_pin_no!(94) | 1);

pub const MT7623_PIN_95_MIPI_TCN_FUNC_GPIO95: u32 = (mtk_pin_no!(95) | 0);
pub const MT7623_PIN_95_MIPI_TCN_FUNC_TCN: u32 = (mtk_pin_no!(95) | 1);

pub const MT7623_PIN_96_MIPI_TCP_FUNC_GPIO96: u32 = (mtk_pin_no!(96) | 0);
pub const MT7623_PIN_96_MIPI_TCP_FUNC_TCP: u32 = (mtk_pin_no!(96) | 1);

pub const MT7623_PIN_97_MIPI_TDN1_FUNC_GPIO97: u32 = (mtk_pin_no!(97) | 0);
pub const MT7623_PIN_97_MIPI_TDN1_FUNC_TDN1: u32 = (mtk_pin_no!(97) | 1);

pub const MT7623_PIN_98_MIPI_TDP1_FUNC_GPIO98: u32 = (mtk_pin_no!(98) | 0);
pub const MT7623_PIN_98_MIPI_TDP1_FUNC_TDP1: u32 = (mtk_pin_no!(98) | 1);

pub const MT7623_PIN_99_MIPI_TDN0_FUNC_GPIO99: u32 = (mtk_pin_no!(99) | 0);
pub const MT7623_PIN_99_MIPI_TDN0_FUNC_TDN0: u32 = (mtk_pin_no!(99) | 1);

pub const MT7623_PIN_100_MIPI_TDP0_FUNC_GPIO100: u32 = (mtk_pin_no!(100) | 0);
pub const MT7623_PIN_100_MIPI_TDP0_FUNC_TDP0: u32 = (mtk_pin_no!(100) | 1);

pub const MT7623_PIN_101_SPI2_CSN_FUNC_GPIO101: u32 = (mtk_pin_no!(101) | 0);
pub const MT7623_PIN_101_SPI2_CSN_FUNC_SPI2_CS: u32 = (mtk_pin_no!(101) | 1);
pub const MT7623_PIN_101_SPI2_CSN_FUNC_SCL3: u32 = (mtk_pin_no!(101) | 3);
pub const MT7623_PIN_101_SPI2_CSN_FUNC_KROW0: u32 = (mtk_pin_no!(101) | 4);

pub const MT7623_PIN_102_SPI2_MI_FUNC_GPIO102: u32 = (mtk_pin_no!(102) | 0);
pub const MT7623_PIN_102_SPI2_MI_FUNC_SPI2_MI: u32 = (mtk_pin_no!(102) | 1);
pub const MT7623_PIN_102_SPI2_MI_FUNC_SPI2_MO: u32 = (mtk_pin_no!(102) | 2);
pub const MT7623_PIN_102_SPI2_MI_FUNC_SDA3: u32 = (mtk_pin_no!(102) | 3);
pub const MT7623_PIN_102_SPI2_MI_FUNC_KROW1: u32 = (mtk_pin_no!(102) | 4);

pub const MT7623_PIN_103_SPI2_MO_FUNC_GPIO103: u32 = (mtk_pin_no!(103) | 0);
pub const MT7623_PIN_103_SPI2_MO_FUNC_SPI2_MO: u32 = (mtk_pin_no!(103) | 1);
pub const MT7623_PIN_103_SPI2_MO_FUNC_SPI2_MI: u32 = (mtk_pin_no!(103) | 2);
pub const MT7623_PIN_103_SPI2_MO_FUNC_SCL3: u32 = (mtk_pin_no!(103) | 3);
pub const MT7623_PIN_103_SPI2_MO_FUNC_KROW2: u32 = (mtk_pin_no!(103) | 4);

pub const MT7623_PIN_104_SPI2_CK_FUNC_GPIO104: u32 = (mtk_pin_no!(104) | 0);
pub const MT7623_PIN_104_SPI2_CK_FUNC_SPI2_CK: u32 = (mtk_pin_no!(104) | 1);
pub const MT7623_PIN_104_SPI2_CK_FUNC_SDA3: u32 = (mtk_pin_no!(104) | 3);
pub const MT7623_PIN_104_SPI2_CK_FUNC_KROW3: u32 = (mtk_pin_no!(104) | 4);

pub const MT7623_PIN_105_MSDC1_CMD_FUNC_GPIO105: u32 = (mtk_pin_no!(105) | 0);
pub const MT7623_PIN_105_MSDC1_CMD_FUNC_MSDC1_CMD: u32 = (mtk_pin_no!(105) | 1);
pub const MT7623_PIN_105_MSDC1_CMD_FUNC_SDA1: u32 = (mtk_pin_no!(105) | 3);
pub const MT7623_PIN_105_MSDC1_CMD_FUNC_I2SOUT_BCK: u32 = (mtk_pin_no!(105) | 6);

pub const MT7623_PIN_106_MSDC1_CLK_FUNC_GPIO106: u32 = (mtk_pin_no!(106) | 0);
pub const MT7623_PIN_106_MSDC1_CLK_FUNC_MSDC1_CLK: u32 = (mtk_pin_no!(106) | 1);
pub const MT7623_PIN_106_MSDC1_CLK_FUNC_SCL1: u32 = (mtk_pin_no!(106) | 3);
pub const MT7623_PIN_106_MSDC1_CLK_FUNC_I2SOUT_LRCK: u32 = (mtk_pin_no!(106) | 6);

pub const MT7623_PIN_107_MSDC1_DAT0_FUNC_GPIO107: u32 = (mtk_pin_no!(107) | 0);
pub const MT7623_PIN_107_MSDC1_DAT0_FUNC_MSDC1_DAT0: u32 = (mtk_pin_no!(107) | 1);
pub const MT7623_PIN_107_MSDC1_DAT0_FUNC_UTXD0: u32 = (mtk_pin_no!(107) | 5);
pub const MT7623_PIN_107_MSDC1_DAT0_FUNC_I2SOUT_DATA_OUT: u32 = (mtk_pin_no!(107) | 6);

pub const MT7623_PIN_108_MSDC1_DAT1_FUNC_GPIO108: u32 = (mtk_pin_no!(108) | 0);
pub const MT7623_PIN_108_MSDC1_DAT1_FUNC_MSDC1_DAT1: u32 = (mtk_pin_no!(108) | 1);
pub const MT7623_PIN_108_MSDC1_DAT1_FUNC_PWM0: u32 = (mtk_pin_no!(108) | 3);
pub const MT7623_PIN_108_MSDC1_DAT1_FUNC_URXD0: u32 = (mtk_pin_no!(108) | 5);
pub const MT7623_PIN_108_MSDC1_DAT1_FUNC_PWM1: u32 = (mtk_pin_no!(108) | 6);

pub const MT7623_PIN_109_MSDC1_DAT2_FUNC_GPIO109: u32 = (mtk_pin_no!(109) | 0);
pub const MT7623_PIN_109_MSDC1_DAT2_FUNC_MSDC1_DAT2: u32 = (mtk_pin_no!(109) | 1);
pub const MT7623_PIN_109_MSDC1_DAT2_FUNC_SDA2: u32 = (mtk_pin_no!(109) | 3);
pub const MT7623_PIN_109_MSDC1_DAT2_FUNC_UTXD1: u32 = (mtk_pin_no!(109) | 5);
pub const MT7623_PIN_109_MSDC1_DAT2_FUNC_PWM2: u32 = (mtk_pin_no!(109) | 6);

pub const MT7623_PIN_110_MSDC1_DAT3_FUNC_GPIO110: u32 = (mtk_pin_no!(110) | 0);
pub const MT7623_PIN_110_MSDC1_DAT3_FUNC_MSDC1_DAT3: u32 = (mtk_pin_no!(110) | 1);
pub const MT7623_PIN_110_MSDC1_DAT3_FUNC_SCL2: u32 = (mtk_pin_no!(110) | 3);
pub const MT7623_PIN_110_MSDC1_DAT3_FUNC_URXD1: u32 = (mtk_pin_no!(110) | 5);
pub const MT7623_PIN_110_MSDC1_DAT3_FUNC_PWM3: u32 = (mtk_pin_no!(110) | 6);

pub const MT7623_PIN_111_MSDC0_DAT7_FUNC_GPIO111: u32 = (mtk_pin_no!(111) | 0);
pub const MT7623_PIN_111_MSDC0_DAT7_FUNC_MSDC0_DAT7: u32 = (mtk_pin_no!(111) | 1);
pub const MT7623_PIN_111_MSDC0_DAT7_FUNC_NLD7: u32 = (mtk_pin_no!(111) | 4);

pub const MT7623_PIN_112_MSDC0_DAT6_FUNC_GPIO112: u32 = (mtk_pin_no!(112) | 0);
pub const MT7623_PIN_112_MSDC0_DAT6_FUNC_MSDC0_DAT6: u32 = (mtk_pin_no!(112) | 1);
pub const MT7623_PIN_112_MSDC0_DAT6_FUNC_NLD6: u32 = (mtk_pin_no!(112) | 4);

pub const MT7623_PIN_113_MSDC0_DAT5_FUNC_GPIO113: u32 = (mtk_pin_no!(113) | 0);
pub const MT7623_PIN_113_MSDC0_DAT5_FUNC_MSDC0_DAT5: u32 = (mtk_pin_no!(113) | 1);
pub const MT7623_PIN_113_MSDC0_DAT5_FUNC_NLD5: u32 = (mtk_pin_no!(113) | 4);

pub const MT7623_PIN_114_MSDC0_DAT4_FUNC_GPIO114: u32 = (mtk_pin_no!(114) | 0);
pub const MT7623_PIN_114_MSDC0_DAT4_FUNC_MSDC0_DAT4: u32 = (mtk_pin_no!(114) | 1);
pub const MT7623_PIN_114_MSDC0_DAT4_FUNC_NLD4: u32 = (mtk_pin_no!(114) | 4);

pub const MT7623_PIN_115_MSDC0_RSTB_FUNC_GPIO115: u32 = (mtk_pin_no!(115) | 0);
pub const MT7623_PIN_115_MSDC0_RSTB_FUNC_MSDC0_RSTB: u32 = (mtk_pin_no!(115) | 1);
pub const MT7623_PIN_115_MSDC0_RSTB_FUNC_NLD8: u32 = (mtk_pin_no!(115) | 4);

pub const MT7623_PIN_116_MSDC0_CMD_FUNC_GPIO116: u32 = (mtk_pin_no!(116) | 0);
pub const MT7623_PIN_116_MSDC0_CMD_FUNC_MSDC0_CMD: u32 = (mtk_pin_no!(116) | 1);
pub const MT7623_PIN_116_MSDC0_CMD_FUNC_NALE: u32 = (mtk_pin_no!(116) | 4);

pub const MT7623_PIN_117_MSDC0_CLK_FUNC_GPIO117: u32 = (mtk_pin_no!(117) | 0);
pub const MT7623_PIN_117_MSDC0_CLK_FUNC_MSDC0_CLK: u32 = (mtk_pin_no!(117) | 1);
pub const MT7623_PIN_117_MSDC0_CLK_FUNC_NWEB: u32 = (mtk_pin_no!(117) | 4);

pub const MT7623_PIN_118_MSDC0_DAT3_FUNC_GPIO118: u32 = (mtk_pin_no!(118) | 0);
pub const MT7623_PIN_118_MSDC0_DAT3_FUNC_MSDC0_DAT3: u32 = (mtk_pin_no!(118) | 1);
pub const MT7623_PIN_118_MSDC0_DAT3_FUNC_NLD3: u32 = (mtk_pin_no!(118) | 4);

pub const MT7623_PIN_119_MSDC0_DAT2_FUNC_GPIO119: u32 = (mtk_pin_no!(119) | 0);
pub const MT7623_PIN_119_MSDC0_DAT2_FUNC_MSDC0_DAT2: u32 = (mtk_pin_no!(119) | 1);
pub const MT7623_PIN_119_MSDC0_DAT2_FUNC_NLD2: u32 = (mtk_pin_no!(119) | 4);

pub const MT7623_PIN_120_MSDC0_DAT1_FUNC_GPIO120: u32 = (mtk_pin_no!(120) | 0);
pub const MT7623_PIN_120_MSDC0_DAT1_FUNC_MSDC0_DAT1: u32 = (mtk_pin_no!(120) | 1);
pub const MT7623_PIN_120_MSDC0_DAT1_FUNC_NLD1: u32 = (mtk_pin_no!(120) | 4);

pub const MT7623_PIN_121_MSDC0_DAT0_FUNC_GPIO121: u32 = (mtk_pin_no!(121) | 0);
pub const MT7623_PIN_121_MSDC0_DAT0_FUNC_MSDC0_DAT0: u32 = (mtk_pin_no!(121) | 1);
pub const MT7623_PIN_121_MSDC0_DAT0_FUNC_NLD0: u32 = (mtk_pin_no!(121) | 4);
pub const MT7623_PIN_121_MSDC0_DAT0_FUNC_WATCHDOG: u32 = (mtk_pin_no!(121) | 5);

pub const MT7623_PIN_122_GPIO122_FUNC_GPIO122: u32 = (mtk_pin_no!(122) | 0);
pub const MT7623_PIN_122_GPIO122_FUNC_CEC: u32 = (mtk_pin_no!(122) | 1);
pub const MT7623_PIN_122_GPIO122_FUNC_SDA2: u32 = (mtk_pin_no!(122) | 4);
pub const MT7623_PIN_122_GPIO122_FUNC_URXD0: u32 = (mtk_pin_no!(122) | 5);

pub const MT7623_PIN_123_HTPLG_FUNC_GPIO123: u32 = (mtk_pin_no!(123) | 0);
pub const MT7623_PIN_123_HTPLG_FUNC_HTPLG: u32 = (mtk_pin_no!(123) | 1);
pub const MT7623_PIN_123_HTPLG_FUNC_SCL2: u32 = (mtk_pin_no!(123) | 4);
pub const MT7623_PIN_123_HTPLG_FUNC_UTXD0: u32 = (mtk_pin_no!(123) | 5);

pub const MT7623_PIN_124_GPIO124_FUNC_GPIO124: u32 = (mtk_pin_no!(124) | 0);
pub const MT7623_PIN_124_GPIO124_FUNC_HDMISCK: u32 = (mtk_pin_no!(124) | 1);
pub const MT7623_PIN_124_GPIO124_FUNC_SDA1: u32 = (mtk_pin_no!(124) | 4);
pub const MT7623_PIN_124_GPIO124_FUNC_PWM3: u32 = (mtk_pin_no!(124) | 5);

pub const MT7623_PIN_125_GPIO125_FUNC_GPIO125: u32 = (mtk_pin_no!(125) | 0);
pub const MT7623_PIN_125_GPIO125_FUNC_HDMISD: u32 = (mtk_pin_no!(125) | 1);
pub const MT7623_PIN_125_GPIO125_FUNC_SCL1: u32 = (mtk_pin_no!(125) | 4);
pub const MT7623_PIN_125_GPIO125_FUNC_PWM4: u32 = (mtk_pin_no!(125) | 5);

pub const MT7623_PIN_126_I2S0_MCLK_FUNC_GPIO126: u32 = (mtk_pin_no!(126) | 0);
pub const MT7623_PIN_126_I2S0_MCLK_FUNC_I2S0_MCLK: u32 = (mtk_pin_no!(126) | 1);
pub const MT7623_PIN_126_I2S0_MCLK_FUNC_AP_I2S_MCLK: u32 = (mtk_pin_no!(126) | 6);

pub const MT7623_PIN_199_SPI1_CK_FUNC_GPIO199: u32 = (mtk_pin_no!(199) | 0);
pub const MT7623_PIN_199_SPI1_CK_FUNC_SPI1_CK: u32 = (mtk_pin_no!(199) | 1);

pub const MT7623_PIN_200_URXD2_FUNC_GPIO200: u32 = (mtk_pin_no!(200) | 0);
pub const MT7623_PIN_200_URXD2_FUNC_URXD2: u32 = (mtk_pin_no!(200) | 6);

pub const MT7623_PIN_201_UTXD2_FUNC_GPIO201: u32 = (mtk_pin_no!(201) | 0);
pub const MT7623_PIN_201_UTXD2_FUNC_UTXD2: u32 = (mtk_pin_no!(201) | 6);

pub const MT7623_PIN_203_PWM0_FUNC_GPIO203: u32 = (mtk_pin_no!(203) | 0);
pub const MT7623_PIN_203_PWM0_FUNC_PWM0: u32 = (mtk_pin_no!(203) | 1);
pub const MT7623_PIN_203_PWM0_FUNC_DISP_PWM: u32 = (mtk_pin_no!(203) | 2);

pub const MT7623_PIN_204_PWM1_FUNC_GPIO204: u32 = (mtk_pin_no!(204) | 0);
pub const MT7623_PIN_204_PWM1_FUNC_PWM1: u32 = (mtk_pin_no!(204) | 1);

pub const MT7623_PIN_205_PWM2_FUNC_GPIO205: u32 = (mtk_pin_no!(205) | 0);
pub const MT7623_PIN_205_PWM2_FUNC_PWM2: u32 = (mtk_pin_no!(205) | 1);

pub const MT7623_PIN_206_PWM3_FUNC_GPIO206: u32 = (mtk_pin_no!(206) | 0);
pub const MT7623_PIN_206_PWM3_FUNC_PWM3: u32 = (mtk_pin_no!(206) | 1);

pub const MT7623_PIN_207_PWM4_FUNC_GPIO207: u32 = (mtk_pin_no!(207) | 0);
pub const MT7623_PIN_207_PWM4_FUNC_PWM4: u32 = (mtk_pin_no!(207) | 1);

pub const MT7623_PIN_208_AUD_EXT_CK1_FUNC_GPIO208: u32 = (mtk_pin_no!(208) | 0);
pub const MT7623_PIN_208_AUD_EXT_CK1_FUNC_AUD_EXT_CK1: u32 = (mtk_pin_no!(208) | 1);
pub const MT7623_PIN_208_AUD_EXT_CK1_FUNC_PWM0: u32 = (mtk_pin_no!(208) | 2);
pub const MT7623_PIN_208_AUD_EXT_CK1_FUNC_PCIE0_PERST_N: u32 = (mtk_pin_no!(208) | 3);
pub const MT7623_PIN_208_AUD_EXT_CK1_FUNC_DISP_PWM: u32 = (mtk_pin_no!(208) | 5);

pub const MT7623_PIN_209_AUD_EXT_CK2_FUNC_GPIO209: u32 = (mtk_pin_no!(209) | 0);
pub const MT7623_PIN_209_AUD_EXT_CK2_FUNC_AUD_EXT_CK2: u32 = (mtk_pin_no!(209) | 1);
pub const MT7623_PIN_209_AUD_EXT_CK2_FUNC_MSDC1_WP: u32 = (mtk_pin_no!(209) | 2);
pub const MT7623_PIN_209_AUD_EXT_CK2_FUNC_PCIE1_PERST_N: u32 = (mtk_pin_no!(209) | 3);
pub const MT7623_PIN_209_AUD_EXT_CK2_FUNC_PWM1: u32 = (mtk_pin_no!(209) | 5);

pub const MT7623_PIN_236_EXT_SDIO3_FUNC_GPIO236: u32 = (mtk_pin_no!(236) | 0);
pub const MT7623_PIN_236_EXT_SDIO3_FUNC_EXT_SDIO3: u32 = (mtk_pin_no!(236) | 1);
pub const MT7623_PIN_236_EXT_SDIO3_FUNC_IDDIG: u32 = (mtk_pin_no!(236) | 2);

pub const MT7623_PIN_237_EXT_SDIO2_FUNC_GPIO237: u32 = (mtk_pin_no!(237) | 0);
pub const MT7623_PIN_237_EXT_SDIO2_FUNC_EXT_SDIO2: u32 = (mtk_pin_no!(237) | 1);
pub const MT7623_PIN_237_EXT_SDIO2_FUNC_DRV_VBUS: u32 = (mtk_pin_no!(237) | 2);

pub const MT7623_PIN_238_EXT_SDIO1_FUNC_GPIO238: u32 = (mtk_pin_no!(238) | 0);
pub const MT7623_PIN_238_EXT_SDIO1_FUNC_EXT_SDIO1: u32 = (mtk_pin_no!(238) | 1);

pub const MT7623_PIN_239_EXT_SDIO0_FUNC_GPIO239: u32 = (mtk_pin_no!(239) | 0);
pub const MT7623_PIN_239_EXT_SDIO0_FUNC_EXT_SDIO0: u32 = (mtk_pin_no!(239) | 1);

pub const MT7623_PIN_240_EXT_XCS_FUNC_GPIO240: u32 = (mtk_pin_no!(240) | 0);
pub const MT7623_PIN_240_EXT_XCS_FUNC_EXT_XCS: u32 = (mtk_pin_no!(240) | 1);

pub const MT7623_PIN_241_EXT_SCK_FUNC_GPIO241: u32 = (mtk_pin_no!(241) | 0);
pub const MT7623_PIN_241_EXT_SCK_FUNC_EXT_SCK: u32 = (mtk_pin_no!(241) | 1);

pub const MT7623_PIN_242_URTS2_FUNC_GPIO242: u32 = (mtk_pin_no!(242) | 0);
pub const MT7623_PIN_242_URTS2_FUNC_URTS2: u32 = (mtk_pin_no!(242) | 1);
pub const MT7623_PIN_242_URTS2_FUNC_UTXD3: u32 = (mtk_pin_no!(242) | 2);
pub const MT7623_PIN_242_URTS2_FUNC_URXD3: u32 = (mtk_pin_no!(242) | 3);
pub const MT7623_PIN_242_URTS2_FUNC_SCL1: u32 = (mtk_pin_no!(242) | 4);

pub const MT7623_PIN_243_UCTS2_FUNC_GPIO243: u32 = (mtk_pin_no!(243) | 0);
pub const MT7623_PIN_243_UCTS2_FUNC_UCTS2: u32 = (mtk_pin_no!(243) | 1);
pub const MT7623_PIN_243_UCTS2_FUNC_URXD3: u32 = (mtk_pin_no!(243) | 2);
pub const MT7623_PIN_243_UCTS2_FUNC_UTXD3: u32 = (mtk_pin_no!(243) | 3);
pub const MT7623_PIN_243_UCTS2_FUNC_SDA1: u32 = (mtk_pin_no!(243) | 4);

pub const MT7623_PIN_250_GPIO250_FUNC_GPIO250: u32 = (mtk_pin_no!(250) | 0);
pub const MT7623_PIN_250_GPIO250_FUNC_TEST_MD7: u32 = (mtk_pin_no!(250) | 1);
pub const MT7623_PIN_250_GPIO250_FUNC_PCIE0_CLKREQ_N: u32 = (mtk_pin_no!(250) | 6);

pub const MT7623_PIN_251_GPIO251_FUNC_GPIO251: u32 = (mtk_pin_no!(251) | 0);
pub const MT7623_PIN_251_GPIO251_FUNC_TEST_MD6: u32 = (mtk_pin_no!(251) | 1);
pub const MT7623_PIN_251_GPIO251_FUNC_PCIE0_WAKE_N: u32 = (mtk_pin_no!(251) | 6);

pub const MT7623_PIN_252_GPIO252_FUNC_GPIO252: u32 = (mtk_pin_no!(252) | 0);
pub const MT7623_PIN_252_GPIO252_FUNC_TEST_MD5: u32 = (mtk_pin_no!(252) | 1);
pub const MT7623_PIN_252_GPIO252_FUNC_PCIE1_CLKREQ_N: u32 = (mtk_pin_no!(252) | 6);

pub const MT7623_PIN_253_GPIO253_FUNC_GPIO253: u32 = (mtk_pin_no!(253) | 0);
pub const MT7623_PIN_253_GPIO253_FUNC_TEST_MD4: u32 = (mtk_pin_no!(253) | 1);
pub const MT7623_PIN_253_GPIO253_FUNC_PCIE1_WAKE_N: u32 = (mtk_pin_no!(253) | 6);

pub const MT7623_PIN_254_GPIO254_FUNC_GPIO254: u32 = (mtk_pin_no!(254) | 0);
pub const MT7623_PIN_254_GPIO254_FUNC_TEST_MD3: u32 = (mtk_pin_no!(254) | 1);
pub const MT7623_PIN_254_GPIO254_FUNC_PCIE2_CLKREQ_N: u32 = (mtk_pin_no!(254) | 6);

pub const MT7623_PIN_255_GPIO255_FUNC_GPIO255: u32 = (mtk_pin_no!(255) | 0);
pub const MT7623_PIN_255_GPIO255_FUNC_TEST_MD2: u32 = (mtk_pin_no!(255) | 1);
pub const MT7623_PIN_255_GPIO255_FUNC_PCIE2_WAKE_N: u32 = (mtk_pin_no!(255) | 6);

pub const MT7623_PIN_256_GPIO256_FUNC_GPIO256: u32 = (mtk_pin_no!(256) | 0);
pub const MT7623_PIN_256_GPIO256_FUNC_TEST_MD1: u32 = (mtk_pin_no!(256) | 1);

pub const MT7623_PIN_257_GPIO257_FUNC_GPIO257: u32 = (mtk_pin_no!(257) | 0);
pub const MT7623_PIN_257_GPIO257_FUNC_TEST_MD0: u32 = (mtk_pin_no!(257) | 1);

pub const MT7623_PIN_261_MSDC1_INS_FUNC_GPIO261: u32 = (mtk_pin_no!(261) | 0);
pub const MT7623_PIN_261_MSDC1_INS_FUNC_MSDC1_INS: u32 = (mtk_pin_no!(261) | 1);

pub const MT7623_PIN_262_G2_TXEN_FUNC_GPIO262: u32 = (mtk_pin_no!(262) | 0);
pub const MT7623_PIN_262_G2_TXEN_FUNC_G2_TXEN: u32 = (mtk_pin_no!(262) | 1);

pub const MT7623_PIN_263_G2_TXD3_FUNC_GPIO263: u32 = (mtk_pin_no!(263) | 0);
pub const MT7623_PIN_263_G2_TXD3_FUNC_G2_TXD3: u32 = (mtk_pin_no!(263) | 1);

pub const MT7623_PIN_264_G2_TXD2_FUNC_GPIO264: u32 = (mtk_pin_no!(264) | 0);
pub const MT7623_PIN_264_G2_TXD2_FUNC_G2_TXD2: u32 = (mtk_pin_no!(264) | 1);

pub const MT7623_PIN_265_G2_TXD1_FUNC_GPIO265: u32 = (mtk_pin_no!(265) | 0);
pub const MT7623_PIN_265_G2_TXD1_FUNC_G2_TXD1: u32 = (mtk_pin_no!(265) | 1);

pub const MT7623_PIN_266_G2_TXD0_FUNC_GPIO266: u32 = (mtk_pin_no!(266) | 0);
pub const MT7623_PIN_266_G2_TXD0_FUNC_G2_TXD0: u32 = (mtk_pin_no!(266) | 1);

pub const MT7623_PIN_267_G2_TXCLK_FUNC_GPIO267: u32 = (mtk_pin_no!(267) | 0);
pub const MT7623_PIN_267_G2_TXCLK_FUNC_G2_TXC: u32 = (mtk_pin_no!(267) | 1);

pub const MT7623_PIN_268_G2_RXCLK_FUNC_GPIO268: u32 = (mtk_pin_no!(268) | 0);
pub const MT7623_PIN_268_G2_RXCLK_FUNC_G2_RXC: u32 = (mtk_pin_no!(268) | 1);

pub const MT7623_PIN_269_G2_RXD0_FUNC_GPIO269: u32 = (mtk_pin_no!(269) | 0);
pub const MT7623_PIN_269_G2_RXD0_FUNC_G2_RXD0: u32 = (mtk_pin_no!(269) | 1);

pub const MT7623_PIN_270_G2_RXD1_FUNC_GPIO270: u32 = (mtk_pin_no!(270) | 0);
pub const MT7623_PIN_270_G2_RXD1_FUNC_G2_RXD1: u32 = (mtk_pin_no!(270) | 1);

pub const MT7623_PIN_271_G2_RXD2_FUNC_GPIO271: u32 = (mtk_pin_no!(271) | 0);
pub const MT7623_PIN_271_G2_RXD2_FUNC_G2_RXD2: u32 = (mtk_pin_no!(271) | 1);

pub const MT7623_PIN_272_G2_RXD3_FUNC_GPIO272: u32 = (mtk_pin_no!(272) | 0);
pub const MT7623_PIN_272_G2_RXD3_FUNC_G2_RXD3: u32 = (mtk_pin_no!(272) | 1);

pub const MT7623_PIN_274_G2_RXDV_FUNC_GPIO274: u32 = (mtk_pin_no!(274) | 0);
pub const MT7623_PIN_274_G2_RXDV_FUNC_G2_RXDV: u32 = (mtk_pin_no!(274) | 1);

pub const MT7623_PIN_275_G2_MDC_FUNC_GPIO275: u32 = (mtk_pin_no!(275) | 0);
pub const MT7623_PIN_275_G2_MDC_FUNC_MDC: u32 = (mtk_pin_no!(275) | 1);

pub const MT7623_PIN_276_G2_MDIO_FUNC_GPIO276: u32 = (mtk_pin_no!(276) | 0);
pub const MT7623_PIN_276_G2_MDIO_FUNC_MDIO: u32 = (mtk_pin_no!(276) | 1);

pub const MT7623_PIN_278_JTAG_RESET_FUNC_GPIO278: u32 = (mtk_pin_no!(278) | 0);
pub const MT7623_PIN_278_JTAG_RESET_FUNC_JTAG_RESET: u32 = (mtk_pin_no!(278) | 1);



// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
