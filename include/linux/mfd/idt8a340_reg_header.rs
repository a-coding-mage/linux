/* SPDX-License-Identifier: GPL-2.0+ */
/* Rust translation of idt8a340_reg.h. External BIT semantics are expanded locally. */

/* SPDX-License-Identifier: GPL-2.0+ */
/*
 * Based on 5.2.0, Family Programming Guide (Sept 30, 2020)
 *
 * Copyright (C) 2021 Integrated Device Technology, Inc., a Renesas Company.
 */

pub const PAGE_ADDR_BASE: u16 = 0x0000u16;
pub const PAGE_ADDR: u16 = 0x00fcu16;

pub const HW_REVISION: u16 = 0x8180u16;
pub const REV_ID: u16 = 0x007au16;

pub const HW_DPLL_0: u16 = (0x8a00u16);
pub const HW_DPLL_1: u16 = (0x8b00u16);
pub const HW_DPLL_2: u16 = (0x8c00u16);
pub const HW_DPLL_3: u16 = (0x8d00u16);
pub const HW_DPLL_4: u16 = (0x8e00u16);
pub const HW_DPLL_5: u16 = (0x8f00u16);
pub const HW_DPLL_6: u16 = (0x9000u16);
pub const HW_DPLL_7: u16 = (0x9100u16);

pub const HW_DPLL_TOD_SW_TRIG_ADDR__0: u16 = (0x080u16);
pub const HW_DPLL_TOD_CTRL_1: u16 = (0x089u16);
pub const HW_DPLL_TOD_CTRL_2: u16 = (0x08Au16);
pub const HW_DPLL_TOD_OVR__0: u16 = (0x098u16);
pub const HW_DPLL_TOD_OUT_0__0: u16 = (0x0B0u16);

pub const HW_Q0_Q1_CH_SYNC_CTRL_0: u16 = (0xa740u16);
pub const HW_Q0_Q1_CH_SYNC_CTRL_1: u16 = (0xa741u16);
pub const HW_Q2_Q3_CH_SYNC_CTRL_0: u16 = (0xa742u16);
pub const HW_Q2_Q3_CH_SYNC_CTRL_1: u16 = (0xa743u16);
pub const HW_Q4_Q5_CH_SYNC_CTRL_0: u16 = (0xa744u16);
pub const HW_Q4_Q5_CH_SYNC_CTRL_1: u16 = (0xa745u16);
pub const HW_Q6_Q7_CH_SYNC_CTRL_0: u16 = (0xa746u16);
pub const HW_Q6_Q7_CH_SYNC_CTRL_1: u16 = (0xa747u16);
pub const HW_Q8_CH_SYNC_CTRL_0: u16 = (0xa748u16);
pub const HW_Q8_CH_SYNC_CTRL_1: u16 = (0xa749u16);
pub const HW_Q9_CH_SYNC_CTRL_0: u16 = (0xa74au16);
pub const HW_Q9_CH_SYNC_CTRL_1: u16 = (0xa74bu16);
pub const HW_Q10_CH_SYNC_CTRL_0: u16 = (0xa74cu16);
pub const HW_Q10_CH_SYNC_CTRL_1: u16 = (0xa74du16);
pub const HW_Q11_CH_SYNC_CTRL_0: u16 = (0xa74eu16);
pub const HW_Q11_CH_SYNC_CTRL_1: u16 = (0xa74fu16);

pub const SYNC_SOURCE_DPLL0_TOD_PPS: u16 = 0x14u16;
pub const SYNC_SOURCE_DPLL1_TOD_PPS: u16 = 0x15u16;
pub const SYNC_SOURCE_DPLL2_TOD_PPS: u16 = 0x16u16;
pub const SYNC_SOURCE_DPLL3_TOD_PPS: u16 = 0x17u16;

pub const SYNCTRL1_MASTER_SYNC_RST: u16 = (1u16 << \1);
pub const SYNCTRL1_MASTER_SYNC_TRIG: u16 = (1u16 << \1);
pub const SYNCTRL1_TOD_SYNC_TRIG: u16 = (1u16 << \1);
pub const SYNCTRL1_FBDIV_FRAME_SYNC_TRIG: u16 = (1u16 << \1);
pub const SYNCTRL1_FBDIV_SYNC_TRIG: u16 = (1u16 << \1);
pub const SYNCTRL1_Q1_DIV_SYNC_TRIG: u16 = (1u16 << \1);
pub const SYNCTRL1_Q0_DIV_SYNC_TRIG: u16 = (1u16 << \1);

pub const HW_Q8_CTRL_SPARE: u16 = (0xa7d4u16);
pub const HW_Q11_CTRL_SPARE: u16 = (0xa7ecu16);

/*
 * Select FOD5 as sync_trigger for Q8 divider.
 * Transition from logic zero to one
 * sets trigger to sync Q8 divider.
 *
 * Unused when FOD4 is driving Q8 divider (normal operation).
 */
pub const Q9_TO_Q8_SYNC_TRIG: u16 = (1u16 << \1);

/*
 * Enable FOD5 as driver for clock and sync for Q8 divider.
 * Enable fanout buffer for FOD5.
 *
 * Unused when FOD4 is driving Q8 divider (normal operation).
 */
pub const Q9_TO_Q8_FANOUT_AND_CLOCK_SYNC_ENABLE_MASK: u16 = ((1u16 << \1) | (1u16 << \1));

/*
 * Select FOD6 as sync_trigger for Q11 divider.
 * Transition from logic zero to one
 * sets trigger to sync Q11 divider.
 *
 * Unused when FOD7 is driving Q11 divider (normal operation).
 */
pub const Q10_TO_Q11_SYNC_TRIG: u16 = (1u16 << \1);

/*
 * Enable FOD6 as driver for clock and sync for Q11 divider.
 * Enable fanout buffer for FOD6.
 *
 * Unused when FOD7 is driving Q11 divider (normal operation).
 */
pub const Q10_TO_Q11_FANOUT_AND_CLOCK_SYNC_ENABLE_MASK: u16 = ((1u16 << \1) | (1u16 << \1));

pub const RESET_CTRL: u16 = 0xc000u16;
pub const SM_RESET: u16 = 0x0012u16;
pub const SM_RESET_V520: u16 = 0x0013u16;
pub const SM_RESET_CMD: u16 = 0x5Au16;

pub const GENERAL_STATUS: u16 = 0xc014u16;
pub const BOOT_STATUS: u16 = 0x0000u16;
pub const HW_REV_ID: u16 = 0x000Au16;
pub const BOND_ID: u16 = 0x000Bu16;
pub const HW_CSR_ID: u16 = 0x000Cu16;
pub const HW_IRQ_ID: u16 = 0x000Eu16;
pub const MAJ_REL: u16 = 0x0010u16;
pub const MIN_REL: u16 = 0x0011u16;
pub const HOTFIX_REL: u16 = 0x0012u16;
pub const PIPELINE_ID: u16 = 0x0014u16;
pub const BUILD_ID: u16 = 0x0018u16;
pub const JTAG_DEVICE_ID: u16 = 0x001cu16;
pub const PRODUCT_ID: u16 = 0x001eu16;
pub const OTP_SCSR_CONFIG_SELECT: u16 = 0x0022u16;

pub const STATUS: u16 = 0xc03cu16;
pub const DPLL0_STATUS: u16 = 0x0018u16;
pub const DPLL1_STATUS: u16 = 0x0019u16;
pub const DPLL2_STATUS: u16 = 0x001au16;
pub const DPLL3_STATUS: u16 = 0x001bu16;
pub const DPLL4_STATUS: u16 = 0x001cu16;
pub const DPLL5_STATUS: u16 = 0x001du16;
pub const DPLL6_STATUS: u16 = 0x001eu16;
pub const DPLL7_STATUS: u16 = 0x001fu16;
pub const DPLL_SYS_STATUS: u16 = 0x0020u16;
pub const DPLL_SYS_APLL_STATUS: u16 = 0x0021u16;
pub const DPLL0_FILTER_STATUS: u16 = 0x0044u16;
pub const DPLL1_FILTER_STATUS: u16 = 0x004cu16;
pub const DPLL2_FILTER_STATUS: u16 = 0x0054u16;
pub const DPLL3_FILTER_STATUS: u16 = 0x005cu16;
pub const DPLL4_FILTER_STATUS: u16 = 0x0064u16;
pub const DPLL5_FILTER_STATUS: u16 = 0x006cu16;
pub const DPLL6_FILTER_STATUS: u16 = 0x0074u16;
pub const DPLL7_FILTER_STATUS: u16 = 0x007cu16;
pub const DPLLSYS_FILTER_STATUS: u16 = 0x0084u16;
pub const USER_GPIO0_TO_7_STATUS: u16 = 0x008au16;
pub const USER_GPIO8_TO_15_STATUS: u16 = 0x008bu16;

pub const GPIO_USER_CONTROL: u16 = 0xc160u16;
pub const GPIO0_TO_7_OUT: u16 = 0x0000u16;
pub const GPIO8_TO_15_OUT: u16 = 0x0001u16;
pub const GPIO0_TO_7_OUT_V520: u16 = 0x0002u16;
pub const GPIO8_TO_15_OUT_V520: u16 = 0x0003u16;

pub const STICKY_STATUS_CLEAR: u16 = 0xc164u16;

pub const GPIO_TOD_NOTIFICATION_CLEAR: u16 = 0xc16cu16;

pub const ALERT_CFG: u16 = 0xc188u16;

pub const SYS_DPLL_XO: u16 = 0xc194u16;

pub const SYS_APLL: u16 = 0xc19cu16;

pub const INPUT_0: u16 = 0xc1b0u16;
pub const INPUT_1: u16 = 0xc1c0u16;
pub const INPUT_2: u16 = 0xc1d0u16;
pub const INPUT_3: u16 = 0xc200u16;
pub const INPUT_4: u16 = 0xc210u16;
pub const INPUT_5: u16 = 0xc220u16;
pub const INPUT_6: u16 = 0xc230u16;
pub const INPUT_7: u16 = 0xc240u16;
pub const INPUT_8: u16 = 0xc250u16;
pub const INPUT_9: u16 = 0xc260u16;
pub const INPUT_10: u16 = 0xc280u16;
pub const INPUT_11: u16 = 0xc290u16;
pub const INPUT_12: u16 = 0xc2a0u16;
pub const INPUT_13: u16 = 0xc2b0u16;
pub const INPUT_14: u16 = 0xc2c0u16;
pub const INPUT_15: u16 = 0xc2d0u16;

pub const REF_MON_0: u16 = 0xc2e0u16;
pub const REF_MON_1: u16 = 0xc2ecu16;
pub const REF_MON_2: u16 = 0xc300u16;
pub const REF_MON_3: u16 = 0xc30cu16;
pub const REF_MON_4: u16 = 0xc318u16;
pub const REF_MON_5: u16 = 0xc324u16;
pub const REF_MON_6: u16 = 0xc330u16;
pub const REF_MON_7: u16 = 0xc33cu16;
pub const REF_MON_8: u16 = 0xc348u16;
pub const REF_MON_9: u16 = 0xc354u16;
pub const REF_MON_10: u16 = 0xc360u16;
pub const REF_MON_11: u16 = 0xc36cu16;
pub const REF_MON_12: u16 = 0xc380u16;
pub const REF_MON_13: u16 = 0xc38cu16;
pub const REF_MON_14: u16 = 0xc398u16;
pub const REF_MON_15: u16 = 0xc3a4u16;

pub const DPLL_0: u16 = 0xc3b0u16;
pub const DPLL_CTRL_REG_0: u16 = 0x0002u16;
pub const DPLL_CTRL_REG_1: u16 = 0x0003u16;
pub const DPLL_CTRL_REG_2: u16 = 0x0004u16;
pub const DPLL_TOD_SYNC_CFG: u16 = 0x0031u16;
pub const DPLL_COMBO_SLAVE_CFG_0: u16 = 0x0032u16;
pub const DPLL_COMBO_SLAVE_CFG_1: u16 = 0x0033u16;
pub const DPLL_SLAVE_REF_CFG: u16 = 0x0034u16;
pub const DPLL_REF_MODE: u16 = 0x0035u16;
pub const DPLL_PHASE_MEASUREMENT_CFG: u16 = 0x0036u16;
pub const DPLL_MODE: u16 = 0x0037u16;
pub const DPLL_MODE_V520: u16 = 0x003Bu16;
pub const DPLL_1: u16 = 0xc400u16;
pub const DPLL_2: u16 = 0xc438u16;
pub const DPLL_2_V520: u16 = 0xc43cu16;
pub const DPLL_3: u16 = 0xc480u16;
pub const DPLL_4: u16 = 0xc4b8u16;
pub const DPLL_4_V520: u16 = 0xc4bcu16;
pub const DPLL_5: u16 = 0xc500u16;
pub const DPLL_6: u16 = 0xc538u16;
pub const DPLL_6_V520: u16 = 0xc53cu16;
pub const DPLL_7: u16 = 0xc580u16;
pub const SYS_DPLL: u16 = 0xc5b8u16;
pub const SYS_DPLL_V520: u16 = 0xc5bcu16;

pub const DPLL_CTRL_0: u16 = 0xc600u16;
pub const DPLL_CTRL_DPLL_MANU_REF_CFG: u16 = 0x0001u16;
pub const DPLL_CTRL_DPLL_FOD_FREQ: u16 = 0x001cu16;
pub const DPLL_CTRL_COMBO_MASTER_CFG: u16 = 0x003au16;
pub const DPLL_CTRL_1: u16 = 0xc63cu16;
pub const DPLL_CTRL_2: u16 = 0xc680u16;
pub const DPLL_CTRL_3: u16 = 0xc6bcu16;
pub const DPLL_CTRL_4: u16 = 0xc700u16;
pub const DPLL_CTRL_5: u16 = 0xc73cu16;
pub const DPLL_CTRL_6: u16 = 0xc780u16;
pub const DPLL_CTRL_7: u16 = 0xc7bcu16;
pub const SYS_DPLL_CTRL: u16 = 0xc800u16;

pub const DPLL_PHASE_0: u16 = 0xc818u16;
/* Signed 42-bit FFO in units of 2^(-53) */
#define DPLL_WR_PHASE                     0x0000
#define DPLL_PHASE_1                      0xc81c
#define DPLL_PHASE_2                      0xc820
#define DPLL_PHASE_3                      0xc824
#define DPLL_PHASE_4                      0xc828
#define DPLL_PHASE_5                      0xc82c
#define DPLL_PHASE_6                      0xc830
#define DPLL_PHASE_7                      0xc834

#define DPLL_FREQ_0                       0xc838
/* Signed 42-bit FFO in units of 2^(-53) */
#define DPLL_WR_FREQ                      0x0000
#define DPLL_FREQ_1                       0xc840
#define DPLL_FREQ_2                       0xc848
#define DPLL_FREQ_3                       0xc850
#define DPLL_FREQ_4                       0xc858
#define DPLL_FREQ_5                       0xc860
#define DPLL_FREQ_6                       0xc868
#define DPLL_FREQ_7                       0xc870

#define DPLL_PHASE_PULL_IN_0              0xc880
#define PULL_IN_OFFSET                    0x0000 /* Signed 32 bit */
pub const PULL_IN_SLOPE_LIMIT: u16 = 0x0004u16 /* Unsigned 24 bit */;
pub const PULL_IN_CTRL: u16 = 0x0007u16;
pub const DPLL_PHASE_PULL_IN_1: u16 = 0xc888u16;
pub const DPLL_PHASE_PULL_IN_2: u16 = 0xc890u16;
pub const DPLL_PHASE_PULL_IN_3: u16 = 0xc898u16;
pub const DPLL_PHASE_PULL_IN_4: u16 = 0xc8a0u16;
pub const DPLL_PHASE_PULL_IN_5: u16 = 0xc8a8u16;
pub const DPLL_PHASE_PULL_IN_6: u16 = 0xc8b0u16;
pub const DPLL_PHASE_PULL_IN_7: u16 = 0xc8b8u16;

pub const GPIO_CFG: u16 = 0xc8c0u16;
pub const GPIO_CFG_GBL: u16 = 0x0000u16;
pub const GPIO_0: u16 = 0xc8c2u16;
pub const GPIO_DCO_INC_DEC: u16 = 0x0000u16;
pub const GPIO_OUT_CTRL_0: u16 = 0x0001u16;
pub const GPIO_OUT_CTRL_1: u16 = 0x0002u16;
pub const GPIO_TOD_TRIG: u16 = 0x0003u16;
pub const GPIO_DPLL_INDICATOR: u16 = 0x0004u16;
pub const GPIO_LOS_INDICATOR: u16 = 0x0005u16;
pub const GPIO_REF_INPUT_DSQ_0: u16 = 0x0006u16;
pub const GPIO_REF_INPUT_DSQ_1: u16 = 0x0007u16;
pub const GPIO_REF_INPUT_DSQ_2: u16 = 0x0008u16;
pub const GPIO_REF_INPUT_DSQ_3: u16 = 0x0009u16;
pub const GPIO_MAN_CLK_SEL_0: u16 = 0x000au16;
pub const GPIO_MAN_CLK_SEL_1: u16 = 0x000bu16;
pub const GPIO_MAN_CLK_SEL_2: u16 = 0x000cu16;
pub const GPIO_SLAVE: u16 = 0x000du16;
pub const GPIO_ALERT_OUT_CFG: u16 = 0x000eu16;
pub const GPIO_TOD_NOTIFICATION_CFG: u16 = 0x000fu16;
pub const GPIO_CTRL: u16 = 0x0010u16;
pub const GPIO_CTRL_V520: u16 = 0x0011u16;
pub const GPIO_1: u16 = 0xc8d4u16;
pub const GPIO_2: u16 = 0xc8e6u16;
pub const GPIO_3: u16 = 0xc900u16;
pub const GPIO_4: u16 = 0xc912u16;
pub const GPIO_5: u16 = 0xc924u16;
pub const GPIO_6: u16 = 0xc936u16;
pub const GPIO_7: u16 = 0xc948u16;
pub const GPIO_8: u16 = 0xc95au16;
pub const GPIO_9: u16 = 0xc980u16;
pub const GPIO_10: u16 = 0xc992u16;
pub const GPIO_11: u16 = 0xc9a4u16;
pub const GPIO_12: u16 = 0xc9b6u16;
pub const GPIO_13: u16 = 0xc9c8u16;
pub const GPIO_14: u16 = 0xc9dau16;
pub const GPIO_15: u16 = 0xca00u16;

pub const OUT_DIV_MUX: u16 = 0xca12u16;
pub const OUTPUT_0: u16 = 0xca14u16;
pub const OUTPUT_0_V520: u16 = 0xca20u16;
/* FOD frequency output divider value */
#define OUT_DIV                           0x0000
#define OUT_DUTY_CYCLE_HIGH               0x0004
#define OUT_CTRL_0                        0x0008
#define OUT_CTRL_1                        0x0009
/* Phase adjustment in FOD cycles */
#define OUT_PHASE_ADJ                     0x000c
#define OUTPUT_1                          0xca24
#define OUTPUT_1_V520                     0xca30
#define OUTPUT_2                          0xca34
#define OUTPUT_2_V520                     0xca40
#define OUTPUT_3                          0xca44
#define OUTPUT_3_V520                     0xca50
#define OUTPUT_4                          0xca54
#define OUTPUT_4_V520                     0xca60
#define OUTPUT_5                          0xca64
#define OUTPUT_5_V520                     0xca80
#define OUTPUT_6                          0xca80
#define OUTPUT_6_V520                     0xca90
#define OUTPUT_7                          0xca90
#define OUTPUT_7_V520                     0xcaa0
#define OUTPUT_8                          0xcaa0
#define OUTPUT_8_V520                     0xcab0
#define OUTPUT_9                          0xcab0
#define OUTPUT_9_V520                     0xcac0
#define OUTPUT_10                         0xcac0
#define OUTPUT_10_V520                     0xcad0
#define OUTPUT_11                         0xcad0
#define OUTPUT_11_V520                    0xcae0

#define SERIAL                            0xcae0
#define SERIAL_V520                       0xcaf0

#define PWM_ENCODER_0                     0xcb00
#define PWM_ENCODER_1                     0xcb08
#define PWM_ENCODER_2                     0xcb10
#define PWM_ENCODER_3                     0xcb18
#define PWM_ENCODER_4                     0xcb20
#define PWM_ENCODER_5                     0xcb28
#define PWM_ENCODER_6                     0xcb30
#define PWM_ENCODER_7                     0xcb38
#define PWM_DECODER_0                     0xcb40
#define PWM_DECODER_1                     0xcb48
#define PWM_DECODER_1_V520                0xcb4a
#define PWM_DECODER_2                     0xcb50
#define PWM_DECODER_2_V520                0xcb54
#define PWM_DECODER_3                     0xcb58
#define PWM_DECODER_3_V520                0xcb5e
#define PWM_DECODER_4                     0xcb60
#define PWM_DECODER_4_V520                0xcb68
#define PWM_DECODER_5                     0xcb68
#define PWM_DECODER_5_V520                0xcb80
#define PWM_DECODER_6                     0xcb70
#define PWM_DECODER_6_V520                0xcb8a
#define PWM_DECODER_7                     0xcb80
#define PWM_DECODER_7_V520                0xcb94
#define PWM_DECODER_8                     0xcb88
#define PWM_DECODER_8_V520                0xcb9e
#define PWM_DECODER_9                     0xcb90
#define PWM_DECODER_9_V520                0xcba8
#define PWM_DECODER_10                    0xcb98
#define PWM_DECODER_10_V520               0xcbb2
#define PWM_DECODER_11                    0xcba0
#define PWM_DECODER_11_V520               0xcbbc
#define PWM_DECODER_12                    0xcba8
#define PWM_DECODER_12_V520               0xcbc6
#define PWM_DECODER_13                    0xcbb0
#define PWM_DECODER_13_V520               0xcbd0
#define PWM_DECODER_14                    0xcbb8
#define PWM_DECODER_14_V520               0xcbda
#define PWM_DECODER_15                    0xcbc0
#define PWM_DECODER_15_V520               0xcbe4
#define PWM_USER_DATA                     0xcbc8
#define PWM_USER_DATA_V520                0xcbf0

#define TOD_0                             0xcbcc
#define TOD_0_V520                        0xcc00
/* Enable TOD counter, output channel sync and even-PPS mode */
#define TOD_CFG                           0x0000
#define TOD_CFG_V520                      0x0001
#define TOD_1                             0xcbce
#define TOD_1_V520                        0xcc02
#define TOD_2                             0xcbd0
#define TOD_2_V520                        0xcc04
#define TOD_3                             0xcbd2
#define TOD_3_V520                        0xcc06

#define TOD_WRITE_0                       0xcc00
#define TOD_WRITE_0_V520                  0xcc10
/* 8-bit subns, 32-bit ns, 48-bit seconds */
#define TOD_WRITE                         0x0000
/* Counter increments after TOD write is completed */
#define TOD_WRITE_COUNTER                 0x000c
/* TOD write trigger configuration */
#define TOD_WRITE_SELECT_CFG_0            0x000d
/* TOD write trigger selection */
#define TOD_WRITE_CMD                     0x000f
#define TOD_WRITE_1                       0xcc10
#define TOD_WRITE_1_V520                  0xcc20
#define TOD_WRITE_2                       0xcc20
#define TOD_WRITE_2_V520                  0xcc30
#define TOD_WRITE_3                       0xcc30
#define TOD_WRITE_3_V520                  0xcc40

#define TOD_READ_PRIMARY_0                0xcc40
#define TOD_READ_PRIMARY_0_V520           0xcc50
/* 8-bit subns, 32-bit ns, 48-bit seconds */
#define TOD_READ_PRIMARY_BASE             0x0000
/* Counter increments after TOD write is completed */
#define TOD_READ_PRIMARY_COUNTER          0x000b
/* Read trigger configuration */
#define TOD_READ_PRIMARY_SEL_CFG_0        0x000c
/* Read trigger selection */
#define TOD_READ_PRIMARY_CMD              0x000e
#define TOD_READ_PRIMARY_CMD_V520         0x000f
#define TOD_READ_PRIMARY_1                0xcc50
#define TOD_READ_PRIMARY_1_V520           0xcc60
#define TOD_READ_PRIMARY_2                0xcc60
#define TOD_READ_PRIMARY_2_V520           0xcc80
#define TOD_READ_PRIMARY_3                0xcc80
#define TOD_READ_PRIMARY_3_V520           0xcc90

#define TOD_READ_SECONDARY_0              0xcc90
#define TOD_READ_SECONDARY_0_V520         0xcca0
/* 8-bit subns, 32-bit ns, 48-bit seconds */
#define TOD_READ_SECONDARY_BASE           0x0000
/* Counter increments after TOD write is completed */
#define TOD_READ_SECONDARY_COUNTER        0x000b
/* Read trigger configuration */
#define TOD_READ_SECONDARY_SEL_CFG_0      0x000c
/* Read trigger selection */
#define TOD_READ_SECONDARY_CMD            0x000e
#define TOD_READ_SECONDARY_CMD_V520       0x000f

#define TOD_READ_SECONDARY_1              0xcca0
#define TOD_READ_SECONDARY_1_V520         0xccb0
#define TOD_READ_SECONDARY_2              0xccb0
#define TOD_READ_SECONDARY_2_V520         0xccc0
#define TOD_READ_SECONDARY_3              0xccc0
#define TOD_READ_SECONDARY_3_V520         0xccd0

#define OUTPUT_TDC_CFG                    0xccd0
#define OUTPUT_TDC_CFG_V520               0xcce0
#define OUTPUT_TDC_0                      0xcd00
#define OUTPUT_TDC_1                      0xcd08
#define OUTPUT_TDC_2                      0xcd10
#define OUTPUT_TDC_3                      0xcd18
#define INPUT_TDC                         0xcd20

#define SCRATCH                           0xcf50
#define SCRATCH_V520                      0xcf4c

#define EEPROM                            0xcf68
#define EEPROM_V520                       0xcf64

#define OTP                               0xcf70

#define BYTE                              0xcf80

/* Bit definitions for the MAJ_REL register */
#define MAJOR_SHIFT                       (1)
#define MAJOR_MASK                        (0x7f)
#define PR_BUILD                          BIT(0)

/* Bit definitions for the USER_GPIO0_TO_7_STATUS register */
#define GPIO0_LEVEL                       BIT(0)
#define GPIO1_LEVEL                       BIT(1)
#define GPIO2_LEVEL                       BIT(2)
#define GPIO3_LEVEL                       BIT(3)
#define GPIO4_LEVEL                       BIT(4)
#define GPIO5_LEVEL                       BIT(5)
#define GPIO6_LEVEL                       BIT(6)
#define GPIO7_LEVEL                       BIT(7)

/* Bit definitions for the USER_GPIO8_TO_15_STATUS register */
#define GPIO8_LEVEL                       BIT(0)
#define GPIO9_LEVEL                       BIT(1)
#define GPIO10_LEVEL                      BIT(2)
#define GPIO11_LEVEL                      BIT(3)
#define GPIO12_LEVEL                      BIT(4)
#define GPIO13_LEVEL                      BIT(5)
#define GPIO14_LEVEL                      BIT(6)
#define GPIO15_LEVEL                      BIT(7)

/* Bit definitions for the GPIO0_TO_7_OUT register */
#define GPIO0_DRIVE_LEVEL                 BIT(0)
#define GPIO1_DRIVE_LEVEL                 BIT(1)
#define GPIO2_DRIVE_LEVEL                 BIT(2)
#define GPIO3_DRIVE_LEVEL                 BIT(3)
#define GPIO4_DRIVE_LEVEL                 BIT(4)
#define GPIO5_DRIVE_LEVEL                 BIT(5)
#define GPIO6_DRIVE_LEVEL                 BIT(6)
#define GPIO7_DRIVE_LEVEL                 BIT(7)

/* Bit definitions for the GPIO8_TO_15_OUT register */
#define GPIO8_DRIVE_LEVEL                 BIT(0)
#define GPIO9_DRIVE_LEVEL                 BIT(1)
#define GPIO10_DRIVE_LEVEL                BIT(2)
#define GPIO11_DRIVE_LEVEL                BIT(3)
#define GPIO12_DRIVE_LEVEL                BIT(4)
#define GPIO13_DRIVE_LEVEL                BIT(5)
#define GPIO14_DRIVE_LEVEL                BIT(6)
#define GPIO15_DRIVE_LEVEL                BIT(7)

/* Bit definitions for the DPLL_TOD_SYNC_CFG register */
#define TOD_SYNC_SOURCE_SHIFT             (1)
#define TOD_SYNC_SOURCE_MASK              (0x3)
#define TOD_SYNC_EN                       BIT(0)

/* Bit definitions for the DPLL_MODE register */
#define WRITE_TIMER_MODE                  BIT(6)
#define PLL_MODE_SHIFT                    (3)
#define PLL_MODE_MASK                     (0x7)
#define STATE_MODE_SHIFT                  (0)
#define STATE_MODE_MASK                   (0x7)

/* Bit definitions for the DPLL_MANU_REF_CFG register */
#define MANUAL_REFERENCE_SHIFT            (0)
#define MANUAL_REFERENCE_MASK             (0x1f)

/* Bit definitions for the GPIO_CFG_GBL register */
#define SUPPLY_MODE_SHIFT                 (0)
#define SUPPLY_MODE_MASK                  (0x3)

/* Bit definitions for the GPIO_DCO_INC_DEC register */
#define INCDEC_DPLL_INDEX_SHIFT           (0)
#define INCDEC_DPLL_INDEX_MASK            (0x7)

/* Bit definitions for the GPIO_OUT_CTRL_0 register */
#define CTRL_OUT_0                        BIT(0)
#define CTRL_OUT_1                        BIT(1)
#define CTRL_OUT_2                        BIT(2)
#define CTRL_OUT_3                        BIT(3)
#define CTRL_OUT_4                        BIT(4)
#define CTRL_OUT_5                        BIT(5)
#define CTRL_OUT_6                        BIT(6)
#define CTRL_OUT_7                        BIT(7)

/* Bit definitions for the GPIO_OUT_CTRL_1 register */
#define CTRL_OUT_8                        BIT(0)
#define CTRL_OUT_9                        BIT(1)
#define CTRL_OUT_10                       BIT(2)
#define CTRL_OUT_11                       BIT(3)
#define CTRL_OUT_12                       BIT(4)
#define CTRL_OUT_13                       BIT(5)
#define CTRL_OUT_14                       BIT(6)
#define CTRL_OUT_15                       BIT(7)

/* Bit definitions for the GPIO_TOD_TRIG register */
#define TOD_TRIG_0                        BIT(0)
#define TOD_TRIG_1                        BIT(1)
#define TOD_TRIG_2                        BIT(2)
#define TOD_TRIG_3                        BIT(3)

/* Bit definitions for the GPIO_DPLL_INDICATOR register */
#define IND_DPLL_INDEX_SHIFT              (0)
#define IND_DPLL_INDEX_MASK               (0x7)

/* Bit definitions for the GPIO_LOS_INDICATOR register */
#define REFMON_INDEX_SHIFT                (0)
#define REFMON_INDEX_MASK                 (0xf)
/* Active level of LOS indicator, 0=low 1=high */
#define ACTIVE_LEVEL                      BIT(4)

/* Bit definitions for the GPIO_REF_INPUT_DSQ_0 register */
#define DSQ_INP_0                         BIT(0)
#define DSQ_INP_1                         BIT(1)
#define DSQ_INP_2                         BIT(2)
#define DSQ_INP_3                         BIT(3)
#define DSQ_INP_4                         BIT(4)
#define DSQ_INP_5                         BIT(5)
#define DSQ_INP_6                         BIT(6)
#define DSQ_INP_7                         BIT(7)

/* Bit definitions for the GPIO_REF_INPUT_DSQ_1 register */
#define DSQ_INP_8                         BIT(0)
#define DSQ_INP_9                         BIT(1)
#define DSQ_INP_10                        BIT(2)
#define DSQ_INP_11                        BIT(3)
#define DSQ_INP_12                        BIT(4)
#define DSQ_INP_13                        BIT(5)
#define DSQ_INP_14                        BIT(6)
#define DSQ_INP_15                        BIT(7)

/* Bit definitions for the GPIO_REF_INPUT_DSQ_2 register */
#define DSQ_DPLL_0                        BIT(0)
#define DSQ_DPLL_1                        BIT(1)
#define DSQ_DPLL_2                        BIT(2)
#define DSQ_DPLL_3                        BIT(3)
#define DSQ_DPLL_4                        BIT(4)
#define DSQ_DPLL_5                        BIT(5)
#define DSQ_DPLL_6                        BIT(6)
#define DSQ_DPLL_7                        BIT(7)

/* Bit definitions for the GPIO_REF_INPUT_DSQ_3 register */
#define DSQ_DPLL_SYS                      BIT(0)
#define GPIO_DSQ_LEVEL                    BIT(1)

/* Bit definitions for the GPIO_TOD_NOTIFICATION_CFG register */
#define DPLL_TOD_SHIFT                    (0)
#define DPLL_TOD_MASK                     (0x3)
#define TOD_READ_SECONDARY                BIT(2)
#define GPIO_ASSERT_LEVEL                 BIT(3)

/* Bit definitions for the GPIO_CTRL register */
#define GPIO_FUNCTION_EN                  BIT(0)
#define GPIO_CMOS_OD_MODE                 BIT(1)
#define GPIO_CONTROL_DIR                  BIT(2)
#define GPIO_PU_PD_MODE                   BIT(3)
#define GPIO_FUNCTION_SHIFT               (4)
#define GPIO_FUNCTION_MASK                (0xf)

/* Bit definitions for the OUT_CTRL_1 register */
#define OUT_SYNC_DISABLE                  BIT(7)
#define SQUELCH_VALUE                     BIT(6)
#define SQUELCH_DISABLE                   BIT(5)
#define PAD_VDDO_SHIFT                    (2)
#define PAD_VDDO_MASK                     (0x7)
#define PAD_CMOSDRV_SHIFT                 (0)
#define PAD_CMOSDRV_MASK                  (0x3)

/* Bit definitions for the TOD_CFG register */
#define TOD_EVEN_PPS_MODE                 BIT(2)
#define TOD_OUT_SYNC_ENABLE               BIT(1)
#define TOD_ENABLE                        BIT(0)

/* Bit definitions for the TOD_WRITE_SELECT_CFG_0 register */
#define WR_PWM_DECODER_INDEX_SHIFT        (4)
#define WR_PWM_DECODER_INDEX_MASK         (0xf)
#define WR_REF_INDEX_SHIFT                (0)
#define WR_REF_INDEX_MASK                 (0xf)

/* Bit definitions for the TOD_WRITE_CMD register */
#define TOD_WRITE_SELECTION_SHIFT         (0)
#define TOD_WRITE_SELECTION_MASK          (0xf)
/* 4.8.7 */
#define TOD_WRITE_TYPE_SHIFT              (4)
#define TOD_WRITE_TYPE_MASK               (0x3)

/* Bit definitions for the TOD_READ_PRIMARY_SEL_CFG_0 register */
#define RD_PWM_DECODER_INDEX_SHIFT        (4)
#define RD_PWM_DECODER_INDEX_MASK         (0xf)
#define RD_REF_INDEX_SHIFT                (0)
#define RD_REF_INDEX_MASK                 (0xf)

/* Bit definitions for the TOD_READ_PRIMARY_CMD register */
#define TOD_READ_TRIGGER_MODE             BIT(4)
#define TOD_READ_TRIGGER_SHIFT            (0)
#define TOD_READ_TRIGGER_MASK             (0xf)

/* Bit definitions for the DPLL_CTRL_COMBO_MASTER_CFG register */
#define COMBO_MASTER_HOLD                 BIT(0)

/* Bit definitions for DPLL_SYS_STATUS register */
#define DPLL_SYS_STATE_MASK               (0xf)

/* Bit definitions for SYS_APLL_STATUS register */
#define SYS_APLL_LOSS_LOCK_LIVE_MASK       BIT(0)
#define SYS_APLL_LOSS_LOCK_LIVE_LOCKED     0
#define SYS_APLL_LOSS_LOCK_LIVE_UNLOCKED   1

/* Bit definitions for the DPLL0_STATUS register */
#define DPLL_STATE_MASK                   (0xf)
#define DPLL_STATE_SHIFT                  (0x0)

/* Values of DPLL_N.DPLL_MODE.PLL_MODE */
enum pll_mode {
	PLL_MODE_MIN = 0,
	PLL_MODE_PLL = PLL_MODE_MIN,
	PLL_MODE_WRITE_PHASE = 1,
	PLL_MODE_WRITE_FREQUENCY = 2,
	PLL_MODE_GPIO_INC_DEC = 3,
	PLL_MODE_SYNTHESIS = 4,
	PLL_MODE_PHASE_MEASUREMENT = 5,
	PLL_MODE_DISABLED = 6,
	PLL_MODE_MAX = PLL_MODE_DISABLED,
};

/* Values of DPLL_CTRL_n.DPLL_MANU_REF_CFG.MANUAL_REFERENCE */
enum manual_reference {
	MANU_REF_MIN = 0,
	MANU_REF_CLK0 = MANU_REF_MIN,
	MANU_REF_CLK1,
	MANU_REF_CLK2,
	MANU_REF_CLK3,
	MANU_REF_CLK4,
	MANU_REF_CLK5,
	MANU_REF_CLK6,
	MANU_REF_CLK7,
	MANU_REF_CLK8,
	MANU_REF_CLK9,
	MANU_REF_CLK10,
	MANU_REF_CLK11,
	MANU_REF_CLK12,
	MANU_REF_CLK13,
	MANU_REF_CLK14,
	MANU_REF_CLK15,
	MANU_REF_WRITE_PHASE,
	MANU_REF_WRITE_FREQUENCY,
	MANU_REF_XO_DPLL,
	MANU_REF_MAX = MANU_REF_XO_DPLL,
};

enum hw_tod_write_trig_sel {
	HW_TOD_WR_TRIG_SEL_MIN = 0,
	HW_TOD_WR_TRIG_SEL_MSB = HW_TOD_WR_TRIG_SEL_MIN,
	HW_TOD_WR_TRIG_SEL_RESERVED = 1,
	HW_TOD_WR_TRIG_SEL_TOD_PPS = 2,
	HW_TOD_WR_TRIG_SEL_IRIGB_PPS = 3,
	HW_TOD_WR_TRIG_SEL_PWM_PPS = 4,
	HW_TOD_WR_TRIG_SEL_GPIO = 5,
	HW_TOD_WR_TRIG_SEL_FOD_SYNC = 6,
	WR_TRIG_SEL_MAX = HW_TOD_WR_TRIG_SEL_FOD_SYNC,
};

enum scsr_read_trig_sel {
	/* CANCEL CURRENT TOD READ; MODULE BECOMES IDLE - NO TRIGGER OCCURS */
	SCSR_TOD_READ_TRIG_SEL_DISABLE = 0,
	/* TRIGGER IMMEDIATELY */
	SCSR_TOD_READ_TRIG_SEL_IMMEDIATE = 1,
	/* TRIGGER ON RISING EDGE OF INTERNAL TOD PPS SIGNAL */
	SCSR_TOD_READ_TRIG_SEL_TODPPS = 2,
	/* TRGGER ON RISING EDGE OF SELECTED REFERENCE INPUT */
	SCSR_TOD_READ_TRIG_SEL_REFCLK = 3,
	/* TRIGGER ON RISING EDGE OF SELECTED PWM DECODER 1PPS OUTPUT */
	SCSR_TOD_READ_TRIG_SEL_PWMPPS = 4,
	SCSR_TOD_READ_TRIG_SEL_RESERVED = 5,
	/* TRIGGER WHEN WRITE FREQUENCY EVENT OCCURS  */
	SCSR_TOD_READ_TRIG_SEL_WRITEFREQUENCYEVENT = 6,
	/* TRIGGER ON SELECTED GPIO */
	SCSR_TOD_READ_TRIG_SEL_GPIO = 7,
	SCSR_TOD_READ_TRIG_SEL_MAX = SCSR_TOD_READ_TRIG_SEL_GPIO,
};

/* Values STATUS.DPLL_SYS_STATUS.DPLL_SYS_STATE */
enum dpll_state {
	DPLL_STATE_MIN = 0,
	DPLL_STATE_FREERUN = DPLL_STATE_MIN,
	DPLL_STATE_LOCKACQ = 1,
	DPLL_STATE_LOCKREC = 2,
	DPLL_STATE_LOCKED = 3,
	DPLL_STATE_HOLDOVER = 4,
	DPLL_STATE_OPEN_LOOP = 5,
	DPLL_STATE_MAX = DPLL_STATE_OPEN_LOOP,
};

/* 4.8.7 only */
enum scsr_tod_write_trig_sel {
	SCSR_TOD_WR_TRIG_SEL_DISABLE = 0,
	SCSR_TOD_WR_TRIG_SEL_IMMEDIATE = 1,
	SCSR_TOD_WR_TRIG_SEL_REFCLK = 2,
	SCSR_TOD_WR_TRIG_SEL_PWMPPS = 3,
	SCSR_TOD_WR_TRIG_SEL_TODPPS = 4,
	SCSR_TOD_WR_TRIG_SEL_SYNCFOD = 5,
	SCSR_TOD_WR_TRIG_SEL_GPIO = 6,
	SCSR_TOD_WR_TRIG_SEL_MAX = SCSR_TOD_WR_TRIG_SEL_GPIO,
};

/* 4.8.7 only */
enum scsr_tod_write_type_sel {
	SCSR_TOD_WR_TYPE_SEL_ABSOLUTE = 0,
	SCSR_TOD_WR_TYPE_SEL_DELTA_PLUS = 1,
	SCSR_TOD_WR_TYPE_SEL_DELTA_MINUS = 2,
	SCSR_TOD_WR_TYPE_SEL_MAX = SCSR_TOD_WR_TYPE_SEL_DELTA_MINUS,
};

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
