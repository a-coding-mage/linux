/*
 * Copyright (C) 2022  Advanced Micro Devices, Inc.
 *
 * Permission is hereby granted, free of charge, to any person obtaining a
 * copy of this software and associated documentation files (the "Software"),
 * to deal in the Software without restriction, including without limitation
 * the rights to use, copy, modify, merge, publish, distribute, sublicense,
 * and/or sell copies of the Software, and to permit persons to whom the
 * Software is furnished to do so, subject to the following conditions:
 *
 * The above copyright notice and this permission notice shall be included
 * in all copies or substantial portions of the Software.
 *
 * THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS
 * OR IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
 * FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT.  IN NO EVENT SHALL
 * THE COPYRIGHT HOLDER(S) BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER LIABILITY, WHETHER IN
 * AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM, OUT OF OR IN
 * CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE SOFTWARE.
 */
// #ifndef _dcn_3_1_5_OFFSET_HEADER
// #define _dcn_3_1_5_OFFSET_HEADER



// addressBlock: dce_dc_dccg_dccg_dfs_dispdec
// base address: 0x0
// #define regDENTIST_DISPCLK_CNTL                                                                         0x0064
// #define regDENTIST_DISPCLK_CNTL_BASE_IDX                                                                1


// addressBlock: dce_dc_dccg_dccg_dispdec
// base address: 0x0
// #define regPHYPLLA_PIXCLK_RESYNC_CNTL                                                                   0x0040
// #define regPHYPLLA_PIXCLK_RESYNC_CNTL_BASE_IDX                                                          1
// #define regPHYPLLB_PIXCLK_RESYNC_CNTL                                                                   0x0041
// #define regPHYPLLB_PIXCLK_RESYNC_CNTL_BASE_IDX                                                          1
// #define regPHYPLLC_PIXCLK_RESYNC_CNTL                                                                   0x0042
// #define regPHYPLLC_PIXCLK_RESYNC_CNTL_BASE_IDX                                                          1
// #define regPHYPLLD_PIXCLK_RESYNC_CNTL                                                                   0x0043
// #define regPHYPLLD_PIXCLK_RESYNC_CNTL_BASE_IDX                                                          1
// #define regDP_DTO_DBUF_EN                                                                               0x0044
// #define regDP_DTO_DBUF_EN_BASE_IDX                                                                      1
// #define regDPREFCLK_CGTT_BLK_CTRL_REG                                                                   0x0048
// #define regDPREFCLK_CGTT_BLK_CTRL_REG_BASE_IDX                                                          1
// #define regDCCG_GATE_DISABLE_CNTL4                                                                      0x0049
// #define regDCCG_GATE_DISABLE_CNTL4_BASE_IDX                                                             1
// #define regDPSTREAMCLK_CNTL                                                                             0x004a
// #define regDPSTREAMCLK_CNTL_BASE_IDX                                                                    1
// #define regREFCLK_CGTT_BLK_CTRL_REG                                                                     0x004b
// #define regREFCLK_CGTT_BLK_CTRL_REG_BASE_IDX                                                            1
// #define regPHYPLLE_PIXCLK_RESYNC_CNTL                                                                   0x004c
// #define regPHYPLLE_PIXCLK_RESYNC_CNTL_BASE_IDX                                                          1
// #define regDCCG_PERFMON_CNTL2                                                                           0x004e
// #define regDCCG_PERFMON_CNTL2_BASE_IDX                                                                  1
// #define regDCCG_DS_DTO_INCR                                                                             0x0053
// #define regDCCG_DS_DTO_INCR_BASE_IDX                                                                    1
// #define regDCCG_DS_DTO_MODULO                                                                           0x0054
// #define regDCCG_DS_DTO_MODULO_BASE_IDX                                                                  1
// #define regDCCG_DS_CNTL                                                                                 0x0055
// #define regDCCG_DS_CNTL_BASE_IDX                                                                        1
// #define regDCCG_DS_HW_CAL_INTERVAL                                                                      0x0056
// #define regDCCG_DS_HW_CAL_INTERVAL_BASE_IDX                                                             1
// #define regDPREFCLK_CNTL                                                                                0x0058
// #define regDPREFCLK_CNTL_BASE_IDX                                                                       1
// #define regDCE_VERSION                                                                                  0x005e
// #define regDCE_VERSION_BASE_IDX                                                                         1
// #define regDCCG_GTC_CNTL                                                                                0x0060
// #define regDCCG_GTC_CNTL_BASE_IDX                                                                       1
// #define regDCCG_GTC_DTO_INCR                                                                            0x0061
// #define regDCCG_GTC_DTO_INCR_BASE_IDX                                                                   1
// #define regDCCG_GTC_DTO_MODULO                                                                          0x0062
// #define regDCCG_GTC_DTO_MODULO_BASE_IDX                                                                 1
// #define regDCCG_GTC_CURRENT                                                                             0x0063
// #define regDCCG_GTC_CURRENT_BASE_IDX                                                                    1
// #define regSYMCLK32_SE_CNTL                                                                             0x0065
// #define regSYMCLK32_SE_CNTL_BASE_IDX                                                                    1
// #define regSYMCLK32_LE_CNTL                                                                             0x0066
// #define regSYMCLK32_LE_CNTL_BASE_IDX                                                                    1
// #define regDSCCLK0_DTO_PARAM                                                                            0x006c
// #define regDSCCLK0_DTO_PARAM_BASE_IDX                                                                   1
// #define regDSCCLK1_DTO_PARAM                                                                            0x006d
// #define regDSCCLK1_DTO_PARAM_BASE_IDX                                                                   1
// #define regDSCCLK2_DTO_PARAM                                                                            0x006e
// #define regDSCCLK2_DTO_PARAM_BASE_IDX                                                                   1
// #define regMILLISECOND_TIME_BASE_DIV                                                                    0x0070
// #define regMILLISECOND_TIME_BASE_DIV_BASE_IDX                                                           1
// #define regDISPCLK_FREQ_CHANGE_CNTL                                                                     0x0071
// #define regDISPCLK_FREQ_CHANGE_CNTL_BASE_IDX                                                            1
// #define regDC_MEM_GLOBAL_PWR_REQ_CNTL                                                                   0x0072
// #define regDC_MEM_GLOBAL_PWR_REQ_CNTL_BASE_IDX                                                          1
// #define regDCCG_PERFMON_CNTL                                                                            0x0073
// #define regDCCG_PERFMON_CNTL_BASE_IDX                                                                   1
// #define regDCCG_GATE_DISABLE_CNTL                                                                       0x0074
// #define regDCCG_GATE_DISABLE_CNTL_BASE_IDX                                                              1
// #define regDISPCLK_CGTT_BLK_CTRL_REG                                                                    0x0075
// #define regDISPCLK_CGTT_BLK_CTRL_REG_BASE_IDX                                                           1
// #define regSOCCLK_CGTT_BLK_CTRL_REG                                                                     0x0076
// #define regSOCCLK_CGTT_BLK_CTRL_REG_BASE_IDX                                                            1
// #define regDCCG_CAC_STATUS                                                                              0x0077
// #define regDCCG_CAC_STATUS_BASE_IDX                                                                     1
// #define regMICROSECOND_TIME_BASE_DIV                                                                    0x007b
// #define regMICROSECOND_TIME_BASE_DIV_BASE_IDX                                                           1
// #define regDCCG_GATE_DISABLE_CNTL2                                                                      0x007c
// #define regDCCG_GATE_DISABLE_CNTL2_BASE_IDX                                                             1
// #define regSYMCLK_CGTT_BLK_CTRL_REG                                                                     0x007d
// #define regSYMCLK_CGTT_BLK_CTRL_REG_BASE_IDX                                                            1
// #define regDCCG_DISP_CNTL_REG                                                                           0x007f
// #define regDCCG_DISP_CNTL_REG_BASE_IDX                                                                  1
// #define regOTG0_PIXEL_RATE_CNTL                                                                         0x0080
// #define regOTG0_PIXEL_RATE_CNTL_BASE_IDX                                                                1
// #define regDP_DTO0_PHASE                                                                                0x0081
// #define regDP_DTO0_PHASE_BASE_IDX                                                                       1
// #define regDP_DTO0_MODULO                                                                               0x0082
// #define regDP_DTO0_MODULO_BASE_IDX                                                                      1
// #define regOTG0_PHYPLL_PIXEL_RATE_CNTL                                                                  0x0083
// #define regOTG0_PHYPLL_PIXEL_RATE_CNTL_BASE_IDX                                                         1
// #define regOTG1_PIXEL_RATE_CNTL                                                                         0x0084
// #define regOTG1_PIXEL_RATE_CNTL_BASE_IDX                                                                1
// #define regDP_DTO1_PHASE                                                                                0x0085
// #define regDP_DTO1_PHASE_BASE_IDX                                                                       1
// #define regDP_DTO1_MODULO                                                                               0x0086
// #define regDP_DTO1_MODULO_BASE_IDX                                                                      1
// #define regOTG1_PHYPLL_PIXEL_RATE_CNTL                                                                  0x0087
// #define regOTG1_PHYPLL_PIXEL_RATE_CNTL_BASE_IDX                                                         1
// #define regOTG2_PIXEL_RATE_CNTL                                                                         0x0088
// #define regOTG2_PIXEL_RATE_CNTL_BASE_IDX                                                                1
// #define regDP_DTO2_PHASE                                                                                0x0089
// #define regDP_DTO2_PHASE_BASE_IDX                                                                       1
// #define regDP_DTO2_MODULO                                                                               0x008a
// #define regDP_DTO2_MODULO_BASE_IDX                                                                      1
// #define regOTG2_PHYPLL_PIXEL_RATE_CNTL                                                                  0x008b
// #define regOTG2_PHYPLL_PIXEL_RATE_CNTL_BASE_IDX                                                         1
// #define regOTG3_PIXEL_RATE_CNTL                                                                         0x008c
// #define regOTG3_PIXEL_RATE_CNTL_BASE_IDX                                                                1
// #define regDP_DTO3_PHASE                                                                                0x008d
// #define regDP_DTO3_PHASE_BASE_IDX                                                                       1
// #define regDP_DTO3_MODULO                                                                               0x008e
// #define regDP_DTO3_MODULO_BASE_IDX                                                                      1
// #define regOTG3_PHYPLL_PIXEL_RATE_CNTL                                                                  0x008f
// #define regOTG3_PHYPLL_PIXEL_RATE_CNTL_BASE_IDX                                                         1
// #define regDPPCLK_CGTT_BLK_CTRL_REG                                                                     0x0098
// #define regDPPCLK_CGTT_BLK_CTRL_REG_BASE_IDX                                                            1
// #define regDPPCLK0_DTO_PARAM                                                                            0x0099
// #define regDPPCLK0_DTO_PARAM_BASE_IDX                                                                   1
// #define regDPPCLK1_DTO_PARAM                                                                            0x009a
// #define regDPPCLK1_DTO_PARAM_BASE_IDX                                                                   1
// #define regDPPCLK2_DTO_PARAM                                                                            0x009b
// #define regDPPCLK2_DTO_PARAM_BASE_IDX                                                                   1
// #define regDPPCLK3_DTO_PARAM                                                                            0x009c
// #define regDPPCLK3_DTO_PARAM_BASE_IDX                                                                   1
// #define regDCCG_CAC_STATUS2                                                                             0x009f
// #define regDCCG_CAC_STATUS2_BASE_IDX                                                                    1
// #define regSYMCLKA_CLOCK_ENABLE                                                                         0x00a0
// #define regSYMCLKA_CLOCK_ENABLE_BASE_IDX                                                                1
// #define regSYMCLKB_CLOCK_ENABLE                                                                         0x00a1
// #define regSYMCLKB_CLOCK_ENABLE_BASE_IDX                                                                1
// #define regSYMCLKC_CLOCK_ENABLE                                                                         0x00a2
// #define regSYMCLKC_CLOCK_ENABLE_BASE_IDX                                                                1
// #define regSYMCLKD_CLOCK_ENABLE                                                                         0x00a3
// #define regSYMCLKD_CLOCK_ENABLE_BASE_IDX                                                                1
// #define regSYMCLKE_CLOCK_ENABLE                                                                         0x00a4
// #define regSYMCLKE_CLOCK_ENABLE_BASE_IDX                                                                1
// #define regDCCG_SOFT_RESET                                                                              0x00a6
// #define regDCCG_SOFT_RESET_BASE_IDX                                                                     1
// #define regDSCCLK_DTO_CTRL                                                                              0x00a7
// #define regDSCCLK_DTO_CTRL_BASE_IDX                                                                     1
// #define regDCCG_AUDIO_DTO_SOURCE                                                                        0x00ab
// #define regDCCG_AUDIO_DTO_SOURCE_BASE_IDX                                                               1
// #define regDCCG_AUDIO_DTO0_PHASE                                                                        0x00ac
// #define regDCCG_AUDIO_DTO0_PHASE_BASE_IDX                                                               1
// #define regDCCG_AUDIO_DTO0_MODULE                                                                       0x00ad
// #define regDCCG_AUDIO_DTO0_MODULE_BASE_IDX                                                              1
// #define regDCCG_AUDIO_DTO1_PHASE                                                                        0x00ae
// #define regDCCG_AUDIO_DTO1_PHASE_BASE_IDX                                                               1
// #define regDCCG_AUDIO_DTO1_MODULE                                                                       0x00af
// #define regDCCG_AUDIO_DTO1_MODULE_BASE_IDX                                                              1
// #define regDCCG_VSYNC_OTG0_LATCH_VALUE                                                                  0x00b0
// #define regDCCG_VSYNC_OTG0_LATCH_VALUE_BASE_IDX                                                         1
// #define regDCCG_VSYNC_OTG1_LATCH_VALUE                                                                  0x00b1
// #define regDCCG_VSYNC_OTG1_LATCH_VALUE_BASE_IDX                                                         1
// #define regDCCG_VSYNC_OTG2_LATCH_VALUE                                                                  0x00b2
// #define regDCCG_VSYNC_OTG2_LATCH_VALUE_BASE_IDX                                                         1
// #define regDCCG_VSYNC_OTG3_LATCH_VALUE                                                                  0x00b3
// #define regDCCG_VSYNC_OTG3_LATCH_VALUE_BASE_IDX                                                         1
// #define regDCCG_VSYNC_OTG4_LATCH_VALUE                                                                  0x00b4
// #define regDCCG_VSYNC_OTG4_LATCH_VALUE_BASE_IDX                                                         1
// #define regDCCG_VSYNC_OTG5_LATCH_VALUE                                                                  0x00b5
// #define regDCCG_VSYNC_OTG5_LATCH_VALUE_BASE_IDX                                                         1
// #define regDPPCLK_DTO_CTRL                                                                              0x00b6
// #define regDPPCLK_DTO_CTRL_BASE_IDX                                                                     1
// #define regDCCG_VSYNC_CNT_CTRL                                                                          0x00b8
// #define regDCCG_VSYNC_CNT_CTRL_BASE_IDX                                                                 1
// #define regDCCG_VSYNC_CNT_INT_CTRL                                                                      0x00b9
// #define regDCCG_VSYNC_CNT_INT_CTRL_BASE_IDX                                                             1
// #define regFORCE_SYMCLK_DISABLE                                                                         0x00ba
// #define regFORCE_SYMCLK_DISABLE_BASE_IDX                                                                1
// #define regDCCG_TEST_CLK_SEL                                                                            0x00be
// #define regDCCG_TEST_CLK_SEL_BASE_IDX                                                                   1
// #define regDTBCLK_DTO0_PHASE                                                                            0x0018
// #define regDTBCLK_DTO0_PHASE_BASE_IDX                                                                   2
// #define regDTBCLK_DTO1_PHASE                                                                            0x0019
// #define regDTBCLK_DTO1_PHASE_BASE_IDX                                                                   2
// #define regDTBCLK_DTO2_PHASE                                                                            0x001a
// #define regDTBCLK_DTO2_PHASE_BASE_IDX                                                                   2
// #define regDTBCLK_DTO3_PHASE                                                                            0x001b
// #define regDTBCLK_DTO3_PHASE_BASE_IDX                                                                   2
// #define regDTBCLK_DTO0_MODULO                                                                           0x001f
// #define regDTBCLK_DTO0_MODULO_BASE_IDX                                                                  2
// #define regDTBCLK_DTO1_MODULO                                                                           0x0020
// #define regDTBCLK_DTO1_MODULO_BASE_IDX                                                                  2
// #define regDTBCLK_DTO2_MODULO                                                                           0x0021
// #define regDTBCLK_DTO2_MODULO_BASE_IDX                                                                  2
// #define regDTBCLK_DTO3_MODULO                                                                           0x0022
// #define regDTBCLK_DTO3_MODULO_BASE_IDX                                                                  2
// #define regHDMICHARCLK0_CLOCK_CNTL                                                                      0x004a
// #define regHDMICHARCLK0_CLOCK_CNTL_BASE_IDX                                                             2
// #define regPHYASYMCLK_CLOCK_CNTL                                                                        0x0052
// #define regPHYASYMCLK_CLOCK_CNTL_BASE_IDX                                                               2
// #define regPHYBSYMCLK_CLOCK_CNTL                                                                        0x0053
// #define regPHYBSYMCLK_CLOCK_CNTL_BASE_IDX                                                               2
// #define regPHYCSYMCLK_CLOCK_CNTL                                                                        0x0054
// #define regPHYCSYMCLK_CLOCK_CNTL_BASE_IDX                                                               2
// #define regPHYDSYMCLK_CLOCK_CNTL                                                                        0x0055
// #define regPHYDSYMCLK_CLOCK_CNTL_BASE_IDX                                                               2
// #define regPHYESYMCLK_CLOCK_CNTL                                                                        0x0056
// #define regPHYESYMCLK_CLOCK_CNTL_BASE_IDX                                                               2
// #define regDCCG_GATE_DISABLE_CNTL3                                                                      0x005a
// #define regDCCG_GATE_DISABLE_CNTL3_BASE_IDX                                                             2
// #define regHDMISTREAMCLK0_DTO_PARAM                                                                     0x005b
// #define regHDMISTREAMCLK0_DTO_PARAM_BASE_IDX                                                            2
// #define regDCCG_AUDIO_DTBCLK_DTO_PHASE                                                                  0x0061
// #define regDCCG_AUDIO_DTBCLK_DTO_PHASE_BASE_IDX                                                         2
// #define regDCCG_AUDIO_DTBCLK_DTO_MODULO                                                                 0x0062
// #define regDCCG_AUDIO_DTBCLK_DTO_MODULO_BASE_IDX                                                        2
// #define regDTBCLK_DTO_DBUF_EN                                                                           0x0063
// #define regDTBCLK_DTO_DBUF_EN_BASE_IDX                                                                  2
// #define regHDMISTREAMCLK_CNTL                                                                           0x0059
// #define regHDMISTREAMCLK_CNTL_BASE_IDX                                                                  2


// addressBlock: dce_dc_dccg_dccg_dcperfmon0_dc_perfmon_dispdec
// base address: 0x0
// #define regDC_PERFMON0_PERFCOUNTER_CNTL                                                                 0x0000
// #define regDC_PERFMON0_PERFCOUNTER_CNTL_BASE_IDX                                                        2
// #define regDC_PERFMON0_PERFCOUNTER_CNTL2                                                                0x0001
// #define regDC_PERFMON0_PERFCOUNTER_CNTL2_BASE_IDX                                                       2
// #define regDC_PERFMON0_PERFCOUNTER_STATE                                                                0x0002
// #define regDC_PERFMON0_PERFCOUNTER_STATE_BASE_IDX                                                       2
// #define regDC_PERFMON0_PERFMON_CNTL                                                                     0x0003
// #define regDC_PERFMON0_PERFMON_CNTL_BASE_IDX                                                            2
// #define regDC_PERFMON0_PERFMON_CNTL2                                                                    0x0004
// #define regDC_PERFMON0_PERFMON_CNTL2_BASE_IDX                                                           2
// #define regDC_PERFMON0_PERFMON_CVALUE_INT_MISC                                                          0x0005
// #define regDC_PERFMON0_PERFMON_CVALUE_INT_MISC_BASE_IDX                                                 2
// #define regDC_PERFMON0_PERFMON_CVALUE_LOW                                                               0x0006
// #define regDC_PERFMON0_PERFMON_CVALUE_LOW_BASE_IDX                                                      2
// #define regDC_PERFMON0_PERFMON_HI                                                                       0x0007
// #define regDC_PERFMON0_PERFMON_HI_BASE_IDX                                                              2
// #define regDC_PERFMON0_PERFMON_LOW                                                                      0x0008
// #define regDC_PERFMON0_PERFMON_LOW_BASE_IDX                                                             2


// addressBlock: dce_dc_dccg_dccg_dcperfmon1_dc_perfmon_dispdec
// base address: 0x30
// #define regDC_PERFMON1_PERFCOUNTER_CNTL                                                                 0x000c
// #define regDC_PERFMON1_PERFCOUNTER_CNTL_BASE_IDX                                                        2
// #define regDC_PERFMON1_PERFCOUNTER_CNTL2                                                                0x000d
// #define regDC_PERFMON1_PERFCOUNTER_CNTL2_BASE_IDX                                                       2
// #define regDC_PERFMON1_PERFCOUNTER_STATE                                                                0x000e
// #define regDC_PERFMON1_PERFCOUNTER_STATE_BASE_IDX                                                       2
// #define regDC_PERFMON1_PERFMON_CNTL                                                                     0x000f
// #define regDC_PERFMON1_PERFMON_CNTL_BASE_IDX                                                            2
// #define regDC_PERFMON1_PERFMON_CNTL2                                                                    0x0010
// #define regDC_PERFMON1_PERFMON_CNTL2_BASE_IDX                                                           2
// #define regDC_PERFMON1_PERFMON_CVALUE_INT_MISC                                                          0x0011
// #define regDC_PERFMON1_PERFMON_CVALUE_INT_MISC_BASE_IDX                                                 2
// #define regDC_PERFMON1_PERFMON_CVALUE_LOW                                                               0x0012
// #define regDC_PERFMON1_PERFMON_CVALUE_LOW_BASE_IDX                                                      2
// #define regDC_PERFMON1_PERFMON_HI                                                                       0x0013
// #define regDC_PERFMON1_PERFMON_HI_BASE_IDX                                                              2
// #define regDC_PERFMON1_PERFMON_LOW                                                                      0x0014
// #define regDC_PERFMON1_PERFMON_LOW_BASE_IDX                                                             2


// addressBlock: dce_dc_dmu_dmcu_dispdec
// base address: 0x0
// #define regDMCU_CTRL                                                                                    0x00da
// #define regDMCU_CTRL_BASE_IDX                                                                           2
// #define regDMCU_STATUS                                                                                  0x00db
// #define regDMCU_STATUS_BASE_IDX                                                                         2
// #define regDMCU_PC_START_ADDR                                                                           0x00dc
// #define regDMCU_PC_START_ADDR_BASE_IDX                                                                  2
// #define regDMCU_FW_START_ADDR                                                                           0x00dd
// #define regDMCU_FW_START_ADDR_BASE_IDX                                                                  2
// #define regDMCU_FW_END_ADDR                                                                             0x00de
// #define regDMCU_FW_END_ADDR_BASE_IDX                                                                    2
// #define regDMCU_FW_ISR_START_ADDR                                                                       0x00df
// #define regDMCU_FW_ISR_START_ADDR_BASE_IDX                                                              2
// #define regDMCU_FW_CS_HI                                                                                0x00e0
// #define regDMCU_FW_CS_HI_BASE_IDX                                                                       2
// #define regDMCU_FW_CS_LO                                                                                0x00e1
// #define regDMCU_FW_CS_LO_BASE_IDX                                                                       2
// #define regDMCU_RAM_ACCESS_CTRL                                                                         0x00e2
// #define regDMCU_RAM_ACCESS_CTRL_BASE_IDX                                                                2
// #define regDMCU_ERAM_WR_CTRL                                                                            0x00e3
// #define regDMCU_ERAM_WR_CTRL_BASE_IDX                                                                   2
// #define regDMCU_ERAM_WR_DATA                                                                            0x00e4
// #define regDMCU_ERAM_WR_DATA_BASE_IDX                                                                   2
// #define regDMCU_ERAM_RD_CTRL                                                                            0x00e5
// #define regDMCU_ERAM_RD_CTRL_BASE_IDX                                                                   2
// #define regDMCU_ERAM_RD_DATA                                                                            0x00e6
// #define regDMCU_ERAM_RD_DATA_BASE_IDX                                                                   2
// #define regDMCU_IRAM_WR_CTRL                                                                            0x00e7
// #define regDMCU_IRAM_WR_CTRL_BASE_IDX                                                                   2
// #define regDMCU_IRAM_WR_DATA                                                                            0x00e8
// #define regDMCU_IRAM_WR_DATA_BASE_IDX                                                                   2
// #define regDMCU_IRAM_RD_CTRL                                                                            0x00e9
// #define regDMCU_IRAM_RD_CTRL_BASE_IDX                                                                   2
// #define regDMCU_IRAM_RD_DATA                                                                            0x00ea
// #define regDMCU_IRAM_RD_DATA_BASE_IDX                                                                   2
// #define regDMCU_EVENT_TRIGGER                                                                           0x00eb
// #define regDMCU_EVENT_TRIGGER_BASE_IDX                                                                  2
// #define regDMCU_UC_INTERNAL_INT_STATUS                                                                  0x00ec
// #define regDMCU_UC_INTERNAL_INT_STATUS_BASE_IDX                                                         2
// #define regDMCU_SS_INTERRUPT_CNTL_STATUS                                                                0x00ed
// #define regDMCU_SS_INTERRUPT_CNTL_STATUS_BASE_IDX                                                       2
// #define regDMCU_INTERRUPT_TO_HOST_EN_MASK                                                               0x00f0
// #define regDMCU_INTERRUPT_TO_HOST_EN_MASK_BASE_IDX                                                      2
// #define regDMCU_INTERRUPT_TO_UC_EN_MASK                                                                 0x00f1
// #define regDMCU_INTERRUPT_TO_UC_EN_MASK_BASE_IDX                                                        2
// #define regDMCU_INTERRUPT_TO_UC_EN_MASK_1                                                               0x00f2
// #define regDMCU_INTERRUPT_TO_UC_EN_MASK_1_BASE_IDX                                                      2
// #define regDMCU_INTERRUPT_TO_UC_XIRQ_IRQ_SEL                                                            0x00f3
// #define regDMCU_INTERRUPT_TO_UC_XIRQ_IRQ_SEL_BASE_IDX                                                   2
// #define regDMCU_INTERRUPT_TO_UC_XIRQ_IRQ_SEL_1                                                          0x00f4
// #define regDMCU_INTERRUPT_TO_UC_XIRQ_IRQ_SEL_1_BASE_IDX                                                 2
// #define regDC_DMCU_SCRATCH                                                                              0x00f5
// #define regDC_DMCU_SCRATCH_BASE_IDX                                                                     2
// #define regDMCU_INT_CNT                                                                                 0x00f6
// #define regDMCU_INT_CNT_BASE_IDX                                                                        2
// #define regDMCU_FW_CHECKSUM_SMPL_BYTE_POS                                                               0x00f7
// #define regDMCU_FW_CHECKSUM_SMPL_BYTE_POS_BASE_IDX                                                      2
// #define regDMCU_UC_CLK_GATING_CNTL                                                                      0x00f8
// #define regDMCU_UC_CLK_GATING_CNTL_BASE_IDX                                                             2
// #define regMASTER_COMM_DATA_REG1                                                                        0x00f9
// #define regMASTER_COMM_DATA_REG1_BASE_IDX                                                               2
// #define regMASTER_COMM_DATA_REG2                                                                        0x00fa
// #define regMASTER_COMM_DATA_REG2_BASE_IDX                                                               2
// #define regMASTER_COMM_DATA_REG3                                                                        0x00fb
// #define regMASTER_COMM_DATA_REG3_BASE_IDX                                                               2
// #define regMASTER_COMM_CMD_REG                                                                          0x00fc
// #define regMASTER_COMM_CMD_REG_BASE_IDX                                                                 2
// #define regMASTER_COMM_CNTL_REG                                                                         0x00fd
// #define regMASTER_COMM_CNTL_REG_BASE_IDX                                                                2
// #define regSLAVE_COMM_DATA_REG1                                                                         0x00fe
// #define regSLAVE_COMM_DATA_REG1_BASE_IDX                                                                2
// #define regSLAVE_COMM_DATA_REG2                                                                         0x00ff
// #define regSLAVE_COMM_DATA_REG2_BASE_IDX                                                                2
// #define regSLAVE_COMM_DATA_REG3                                                                         0x0100
// #define regSLAVE_COMM_DATA_REG3_BASE_IDX                                                                2
// #define regSLAVE_COMM_CMD_REG                                                                           0x0101
// #define regSLAVE_COMM_CMD_REG_BASE_IDX                                                                  2
// #define regSLAVE_COMM_CNTL_REG                                                                          0x0102
// #define regSLAVE_COMM_CNTL_REG_BASE_IDX                                                                 2
// #define regDMCU_PERFMON_INTERRUPT_TO_UC_EN_MASK1                                                        0x010a
// #define regDMCU_PERFMON_INTERRUPT_TO_UC_EN_MASK1_BASE_IDX                                               2
// #define regDMCU_PERFMON_INTERRUPT_TO_UC_EN_MASK2                                                        0x010b
// #define regDMCU_PERFMON_INTERRUPT_TO_UC_EN_MASK2_BASE_IDX                                               2
// #define regDMCU_PERFMON_INTERRUPT_TO_UC_EN_MASK3                                                        0x010c
// #define regDMCU_PERFMON_INTERRUPT_TO_UC_EN_MASK3_BASE_IDX                                               2
// #define regDMCU_PERFMON_INTERRUPT_TO_UC_EN_MASK4                                                        0x010d
// #define regDMCU_PERFMON_INTERRUPT_TO_UC_EN_MASK4_BASE_IDX                                               2
// #define regDMCU_PERFMON_INTERRUPT_TO_UC_EN_MASK5                                                        0x010e
// #define regDMCU_PERFMON_INTERRUPT_TO_UC_EN_MASK5_BASE_IDX                                               2
// #define regDMCU_PERFMON_INTERRUPT_TO_UC_XIRQ_IRQ_SEL1                                                   0x010f
// #define regDMCU_PERFMON_INTERRUPT_TO_UC_XIRQ_IRQ_SEL1_BASE_IDX                                          2
// #define regDMCU_PERFMON_INTERRUPT_TO_UC_XIRQ_IRQ_SEL2                                                   0x0110
// #define regDMCU_PERFMON_INTERRUPT_TO_UC_XIRQ_IRQ_SEL2_BASE_IDX                                          2
// #define regDMCU_PERFMON_INTERRUPT_TO_UC_XIRQ_IRQ_SEL3                                                   0x0111
// #define regDMCU_PERFMON_INTERRUPT_TO_UC_XIRQ_IRQ_SEL3_BASE_IDX                                          2
// #define regDMCU_PERFMON_INTERRUPT_TO_UC_XIRQ_IRQ_SEL4                                                   0x0112
// #define regDMCU_PERFMON_INTERRUPT_TO_UC_XIRQ_IRQ_SEL4_BASE_IDX                                          2
// #define regDMCU_PERFMON_INTERRUPT_TO_UC_XIRQ_IRQ_SEL5                                                   0x0113
// #define regDMCU_PERFMON_INTERRUPT_TO_UC_XIRQ_IRQ_SEL5_BASE_IDX                                          2
// #define regDMCU_DPRX_INTERRUPT_TO_UC_EN_MASK1                                                           0x0115
// #define regDMCU_DPRX_INTERRUPT_TO_UC_EN_MASK1_BASE_IDX                                                  2
// #define regDMCU_DPRX_INTERRUPT_TO_UC_XIRQ_IRQ_SEL1                                                      0x0116
// #define regDMCU_DPRX_INTERRUPT_TO_UC_XIRQ_IRQ_SEL1_BASE_IDX                                             2
// #define regDMCU_INTERRUPT_TO_UC_EN_MASK_CONTINUE                                                        0x011a
// #define regDMCU_INTERRUPT_TO_UC_EN_MASK_CONTINUE_BASE_IDX                                               2
// #define regDMCU_INTERRUPT_TO_UC_XIRQ_IRQ_SEL_CONTINUE                                                   0x011b
// #define regDMCU_INTERRUPT_TO_UC_XIRQ_IRQ_SEL_CONTINUE_BASE_IDX                                          2
// #define regDMCU_INT_CNT_CONTINUE                                                                        0x011c
// #define regDMCU_INT_CNT_CONTINUE_BASE_IDX                                                               2
// #define regDMCU_INTERRUPT_TO_UC_XIRQ_IRQ_SEL_CONT2                                                      0x011d
// #define regDMCU_INTERRUPT_TO_UC_XIRQ_IRQ_SEL_CONT2_BASE_IDX                                             2
// #define regDMCU_INTERRUPT_TO_UC_EN_MASK_2                                                               0x011f
// #define regDMCU_INTERRUPT_TO_UC_EN_MASK_2_BASE_IDX                                                      2
// #define regDMCU_INT_CNT_CONT2                                                                           0x0120
// #define regDMCU_INT_CNT_CONT2_BASE_IDX                                                                  2
// #define regDMCU_INT_CNT_CONT3                                                                           0x0121
// #define regDMCU_INT_CNT_CONT3_BASE_IDX                                                                  2


// addressBlock: dce_dc_dmu_fgsec_dispdec
// base address: 0x0
// #define regDMCUB_RBBMIF_SEC_CNTL                                                                        0x017a
// #define regDMCUB_RBBMIF_SEC_CNTL_BASE_IDX                                                               2


// addressBlock: dce_dc_dmu_rbbmif_dispdec
// base address: 0x0
// #define regRBBMIF_TIMEOUT                                                                               0x017f
// #define regRBBMIF_TIMEOUT_BASE_IDX                                                                      2
// #define regRBBMIF_STATUS                                                                                0x0180
// #define regRBBMIF_STATUS_BASE_IDX                                                                       2
// #define regRBBMIF_STATUS_2                                                                              0x0181
// #define regRBBMIF_STATUS_2_BASE_IDX                                                                     2
// #define regRBBMIF_INT_STATUS                                                                            0x0182
// #define regRBBMIF_INT_STATUS_BASE_IDX                                                                   2
// #define regRBBMIF_TIMEOUT_DIS                                                                           0x0183
// #define regRBBMIF_TIMEOUT_DIS_BASE_IDX                                                                  2
// #define regRBBMIF_TIMEOUT_DIS_2                                                                         0x0184
// #define regRBBMIF_TIMEOUT_DIS_2_BASE_IDX                                                                2
// #define regRBBMIF_STATUS_FLAG                                                                           0x0185
// #define regRBBMIF_STATUS_FLAG_BASE_IDX                                                                  2


// addressBlock: dce_dc_dmu_dmu_dcperfmon_dc_perfmon_dispdec
// base address: 0x2f8
// #define regDC_PERFMON2_PERFCOUNTER_CNTL                                                                 0x00be
// #define regDC_PERFMON2_PERFCOUNTER_CNTL_BASE_IDX                                                        2
// #define regDC_PERFMON2_PERFCOUNTER_CNTL2                                                                0x00bf
// #define regDC_PERFMON2_PERFCOUNTER_CNTL2_BASE_IDX                                                       2
// #define regDC_PERFMON2_PERFCOUNTER_STATE                                                                0x00c0
// #define regDC_PERFMON2_PERFCOUNTER_STATE_BASE_IDX                                                       2
// #define regDC_PERFMON2_PERFMON_CNTL                                                                     0x00c1
// #define regDC_PERFMON2_PERFMON_CNTL_BASE_IDX                                                            2
// #define regDC_PERFMON2_PERFMON_CNTL2                                                                    0x00c2
// #define regDC_PERFMON2_PERFMON_CNTL2_BASE_IDX                                                           2
// #define regDC_PERFMON2_PERFMON_CVALUE_INT_MISC                                                          0x00c3
// #define regDC_PERFMON2_PERFMON_CVALUE_INT_MISC_BASE_IDX                                                 2
// #define regDC_PERFMON2_PERFMON_CVALUE_LOW                                                               0x00c4
// #define regDC_PERFMON2_PERFMON_CVALUE_LOW_BASE_IDX                                                      2
// #define regDC_PERFMON2_PERFMON_HI                                                                       0x00c5
// #define regDC_PERFMON2_PERFMON_HI_BASE_IDX                                                              2
// #define regDC_PERFMON2_PERFMON_LOW                                                                      0x00c6
// #define regDC_PERFMON2_PERFMON_LOW_BASE_IDX                                                             2


// addressBlock: dce_dc_dmu_ihc_dispdec
// base address: 0x0
// #define regDC_GPU_TIMER_START_POSITION_V_UPDATE                                                         0x0126
// #define regDC_GPU_TIMER_START_POSITION_V_UPDATE_BASE_IDX                                                2
// #define regDC_GPU_TIMER_START_POSITION_VSTARTUP                                                         0x0127
// #define regDC_GPU_TIMER_START_POSITION_VSTARTUP_BASE_IDX                                                2
// #define regDC_GPU_TIMER_READ                                                                            0x0128
// #define regDC_GPU_TIMER_READ_BASE_IDX                                                                   2
// #define regDC_GPU_TIMER_READ_CNTL                                                                       0x0129
// #define regDC_GPU_TIMER_READ_CNTL_BASE_IDX                                                              2
// #define regDC_GPU_TIMER_START_POSITION_VREADY                                                           0x0141
// #define regDC_GPU_TIMER_START_POSITION_VREADY_BASE_IDX                                                  2
// #define regDC_GPU_TIMER_START_POSITION_FLIP                                                             0x0142
// #define regDC_GPU_TIMER_START_POSITION_FLIP_BASE_IDX                                                    2
// #define regDC_GPU_TIMER_START_POSITION_V_UPDATE_NO_LOCK                                                 0x0143
// #define regDC_GPU_TIMER_START_POSITION_V_UPDATE_NO_LOCK_BASE_IDX                                        2
// #define regDC_GPU_TIMER_START_POSITION_FLIP_AWAY                                                        0x0144
// #define regDC_GPU_TIMER_START_POSITION_FLIP_AWAY_BASE_IDX                                               2
// #define regDCCG_INTERRUPT_DEST                                                                          0x0148
// #define regDCCG_INTERRUPT_DEST_BASE_IDX                                                                 2
// #define regDMU_INTERRUPT_DEST                                                                           0x0149
// #define regDMU_INTERRUPT_DEST_BASE_IDX                                                                  2
// #define regDMU_INTERRUPT_DEST2                                                                          0x014a
// #define regDMU_INTERRUPT_DEST2_BASE_IDX                                                                 2
// #define regDCPG_INTERRUPT_DEST                                                                          0x014b
// #define regDCPG_INTERRUPT_DEST_BASE_IDX                                                                 2
// #define regDCPG_INTERRUPT_DEST2                                                                         0x014c
// #define regDCPG_INTERRUPT_DEST2_BASE_IDX                                                                2
// #define regMMHUBBUB_INTERRUPT_DEST                                                                      0x014d
// #define regMMHUBBUB_INTERRUPT_DEST_BASE_IDX                                                             2
// #define regWB_INTERRUPT_DEST                                                                            0x014e
// #define regWB_INTERRUPT_DEST_BASE_IDX                                                                   2
// #define regDCHUB_INTERRUPT_DEST                                                                         0x014f
// #define regDCHUB_INTERRUPT_DEST_BASE_IDX                                                                2
// #define regDCHUB_PERFCOUNTER_INTERRUPT_DEST                                                             0x0150
// #define regDCHUB_PERFCOUNTER_INTERRUPT_DEST_BASE_IDX                                                    2
// #define regDCHUB_INTERRUPT_DEST2                                                                        0x0151
// #define regDCHUB_INTERRUPT_DEST2_BASE_IDX                                                               2
// #define regDPP_PERFCOUNTER_INTERRUPT_DEST                                                               0x0152
// #define regDPP_PERFCOUNTER_INTERRUPT_DEST_BASE_IDX                                                      2
// #define regMPC_INTERRUPT_DEST                                                                           0x0153
// #define regMPC_INTERRUPT_DEST_BASE_IDX                                                                  2
// #define regOPP_INTERRUPT_DEST                                                                           0x0154
// #define regOPP_INTERRUPT_DEST_BASE_IDX                                                                  2
// #define regOPTC_INTERRUPT_DEST                                                                          0x0155
// #define regOPTC_INTERRUPT_DEST_BASE_IDX                                                                 2
// #define regOTG0_INTERRUPT_DEST                                                                          0x0156
// #define regOTG0_INTERRUPT_DEST_BASE_IDX                                                                 2
// #define regOTG1_INTERRUPT_DEST                                                                          0x0157
// #define regOTG1_INTERRUPT_DEST_BASE_IDX                                                                 2
// #define regOTG2_INTERRUPT_DEST                                                                          0x0158
// #define regOTG2_INTERRUPT_DEST_BASE_IDX                                                                 2
// #define regOTG3_INTERRUPT_DEST                                                                          0x0159
// #define regOTG3_INTERRUPT_DEST_BASE_IDX                                                                 2
// #define regOTG4_INTERRUPT_DEST                                                                          0x015a
// #define regOTG4_INTERRUPT_DEST_BASE_IDX                                                                 2
// #define regOTG5_INTERRUPT_DEST                                                                          0x015b
// #define regOTG5_INTERRUPT_DEST_BASE_IDX                                                                 2
// #define regDIG_INTERRUPT_DEST                                                                           0x015c
// #define regDIG_INTERRUPT_DEST_BASE_IDX                                                                  2
// #define regI2C_DDC_HPD_INTERRUPT_DEST                                                                   0x015d
// #define regI2C_DDC_HPD_INTERRUPT_DEST_BASE_IDX                                                          2
// #define regDIO_INTERRUPT_DEST                                                                           0x015f
// #define regDIO_INTERRUPT_DEST_BASE_IDX                                                                  2
// #define regDCIO_INTERRUPT_DEST                                                                          0x0160
// #define regDCIO_INTERRUPT_DEST_BASE_IDX                                                                 2
// #define regHPD_INTERRUPT_DEST                                                                           0x0161
// #define regHPD_INTERRUPT_DEST_BASE_IDX                                                                  2
// #define regAZ_INTERRUPT_DEST                                                                            0x0162
// #define regAZ_INTERRUPT_DEST_BASE_IDX                                                                   2
// #define regAUX_INTERRUPT_DEST                                                                           0x0163
// #define regAUX_INTERRUPT_DEST_BASE_IDX                                                                  2
// #define regDSC_INTERRUPT_DEST                                                                           0x0164
// #define regDSC_INTERRUPT_DEST_BASE_IDX                                                                  2
// #define regHPO_INTERRUPT_DEST                                                                           0x0165
// #define regHPO_INTERRUPT_DEST_BASE_IDX                                                                  2


// addressBlock: dce_dc_dmu_dmu_misc_dispdec
// base address: 0x0
// #define regCC_DC_PIPE_DIS                                                                               0x00ca
// #define regCC_DC_PIPE_DIS_BASE_IDX                                                                      2
// #define regDMU_CLK_CNTL                                                                                 0x00cb
// #define regDMU_CLK_CNTL_BASE_IDX                                                                        2
// #define regDMU_MEM_PWR_CNTL                                                                             0x00cc
// #define regDMU_MEM_PWR_CNTL_BASE_IDX                                                                    2
// #define regDMCU_SMU_INTERRUPT_CNTL                                                                      0x00cd
// #define regDMCU_SMU_INTERRUPT_CNTL_BASE_IDX                                                             2
// #define regZSC_CNTL                                                                                     0x00cf
// #define regZSC_CNTL_BASE_IDX                                                                            2
// #define regZSC_CNTL2                                                                                    0x00d0
// #define regZSC_CNTL2_BASE_IDX                                                                           2
// #define regDMU_MISC_ALLOW_DS_FORCE                                                                      0x00d6
// #define regDMU_MISC_ALLOW_DS_FORCE_BASE_IDX                                                             2
// #define regZSC_STATUS                                                                                   0x00d7
// #define regZSC_STATUS_BASE_IDX                                                                          2


// addressBlock: dce_dc_dmu_dc_pg_dispdec
// base address: 0x0
// #define regDOMAIN0_PG_CONFIG                                                                            0x0080
// #define regDOMAIN0_PG_CONFIG_BASE_IDX                                                                   2
// #define regDOMAIN0_PG_STATUS                                                                            0x0081
// #define regDOMAIN0_PG_STATUS_BASE_IDX                                                                   2
// #define regDOMAIN1_PG_CONFIG                                                                            0x0082
// #define regDOMAIN1_PG_CONFIG_BASE_IDX                                                                   2
// #define regDOMAIN1_PG_STATUS                                                                            0x0083
// #define regDOMAIN1_PG_STATUS_BASE_IDX                                                                   2
// #define regDOMAIN2_PG_CONFIG                                                                            0x0084
// #define regDOMAIN2_PG_CONFIG_BASE_IDX                                                                   2
// #define regDOMAIN2_PG_STATUS                                                                            0x0085
// #define regDOMAIN2_PG_STATUS_BASE_IDX                                                                   2
// #define regDOMAIN3_PG_CONFIG                                                                            0x0086
// #define regDOMAIN3_PG_CONFIG_BASE_IDX                                                                   2
// #define regDOMAIN3_PG_STATUS                                                                            0x0087
// #define regDOMAIN3_PG_STATUS_BASE_IDX                                                                   2
// #define regDOMAIN16_PG_CONFIG                                                                           0x0089
// #define regDOMAIN16_PG_CONFIG_BASE_IDX                                                                  2
// #define regDOMAIN16_PG_STATUS                                                                           0x008a
// #define regDOMAIN16_PG_STATUS_BASE_IDX                                                                  2
// #define regDOMAIN17_PG_CONFIG                                                                           0x008b
// #define regDOMAIN17_PG_CONFIG_BASE_IDX                                                                  2
// #define regDOMAIN17_PG_STATUS                                                                           0x008c
// #define regDOMAIN17_PG_STATUS_BASE_IDX                                                                  2
// #define regDOMAIN18_PG_CONFIG                                                                           0x008d
// #define regDOMAIN18_PG_CONFIG_BASE_IDX                                                                  2
// #define regDOMAIN18_PG_STATUS                                                                           0x008e
// #define regDOMAIN18_PG_STATUS_BASE_IDX                                                                  2
// #define regDC_IP_REQUEST_CNTL                                                                           0x0093
// #define regDC_IP_REQUEST_CNTL_BASE_IDX                                                                  2


// addressBlock: dce_dc_dmu_dmcub_dispdec
// base address: 0x0
// #define regDMCUB_REGION0_OFFSET                                                                         0x018e
// #define regDMCUB_REGION0_OFFSET_BASE_IDX                                                                2
// #define regDMCUB_REGION0_OFFSET_HIGH                                                                    0x018f
// #define regDMCUB_REGION0_OFFSET_HIGH_BASE_IDX                                                           2
// #define regDMCUB_REGION1_OFFSET                                                                         0x0190
// #define regDMCUB_REGION1_OFFSET_BASE_IDX                                                                2
// #define regDMCUB_REGION1_OFFSET_HIGH                                                                    0x0191
// #define regDMCUB_REGION1_OFFSET_HIGH_BASE_IDX                                                           2
// #define regDMCUB_REGION2_OFFSET                                                                         0x0192
// #define regDMCUB_REGION2_OFFSET_BASE_IDX                                                                2
// #define regDMCUB_REGION2_OFFSET_HIGH                                                                    0x0193
// #define regDMCUB_REGION2_OFFSET_HIGH_BASE_IDX                                                           2
// #define regDMCUB_REGION4_OFFSET                                                                         0x0196
// #define regDMCUB_REGION4_OFFSET_BASE_IDX                                                                2
// #define regDMCUB_REGION4_OFFSET_HIGH                                                                    0x0197
// #define regDMCUB_REGION4_OFFSET_HIGH_BASE_IDX                                                           2
// #define regDMCUB_REGION5_OFFSET                                                                         0x0198
// #define regDMCUB_REGION5_OFFSET_BASE_IDX                                                                2
// #define regDMCUB_REGION5_OFFSET_HIGH                                                                    0x0199
// #define regDMCUB_REGION5_OFFSET_HIGH_BASE_IDX                                                           2
// #define regDMCUB_REGION6_OFFSET                                                                         0x019a
// #define regDMCUB_REGION6_OFFSET_BASE_IDX                                                                2
// #define regDMCUB_REGION6_OFFSET_HIGH                                                                    0x019b
// #define regDMCUB_REGION6_OFFSET_HIGH_BASE_IDX                                                           2
// #define regDMCUB_REGION7_OFFSET                                                                         0x019c
// #define regDMCUB_REGION7_OFFSET_BASE_IDX                                                                2
// #define regDMCUB_REGION7_OFFSET_HIGH                                                                    0x019d
// #define regDMCUB_REGION7_OFFSET_HIGH_BASE_IDX                                                           2
// #define regDMCUB_REGION0_TOP_ADDRESS                                                                    0x019e
// #define regDMCUB_REGION0_TOP_ADDRESS_BASE_IDX                                                           2
// #define regDMCUB_REGION1_TOP_ADDRESS                                                                    0x019f
// #define regDMCUB_REGION1_TOP_ADDRESS_BASE_IDX                                                           2
// #define regDMCUB_REGION2_TOP_ADDRESS                                                                    0x01a0
// #define regDMCUB_REGION2_TOP_ADDRESS_BASE_IDX                                                           2
// #define regDMCUB_REGION4_TOP_ADDRESS                                                                    0x01a1
// #define regDMCUB_REGION4_TOP_ADDRESS_BASE_IDX                                                           2
// #define regDMCUB_REGION5_TOP_ADDRESS                                                                    0x01a2
// #define regDMCUB_REGION5_TOP_ADDRESS_BASE_IDX                                                           2
// #define regDMCUB_REGION6_TOP_ADDRESS                                                                    0x01a3
// #define regDMCUB_REGION6_TOP_ADDRESS_BASE_IDX                                                           2
// #define regDMCUB_REGION7_TOP_ADDRESS                                                                    0x01a4
// #define regDMCUB_REGION7_TOP_ADDRESS_BASE_IDX                                                           2
// #define regDMCUB_REGION3_CW0_BASE_ADDRESS                                                               0x01a5
// #define regDMCUB_REGION3_CW0_BASE_ADDRESS_BASE_IDX                                                      2
// #define regDMCUB_REGION3_CW1_BASE_ADDRESS                                                               0x01a6
// #define regDMCUB_REGION3_CW1_BASE_ADDRESS_BASE_IDX                                                      2
// #define regDMCUB_REGION3_CW2_BASE_ADDRESS                                                               0x01a7
// #define regDMCUB_REGION3_CW2_BASE_ADDRESS_BASE_IDX                                                      2
// #define regDMCUB_REGION3_CW3_BASE_ADDRESS                                                               0x01a8
// #define regDMCUB_REGION3_CW3_BASE_ADDRESS_BASE_IDX                                                      2
// #define regDMCUB_REGION3_CW4_BASE_ADDRESS                                                               0x01a9
// #define regDMCUB_REGION3_CW4_BASE_ADDRESS_BASE_IDX                                                      2
// #define regDMCUB_REGION3_CW5_BASE_ADDRESS                                                               0x01aa
// #define regDMCUB_REGION3_CW5_BASE_ADDRESS_BASE_IDX                                                      2
// #define regDMCUB_REGION3_CW6_BASE_ADDRESS                                                               0x01ab
// #define regDMCUB_REGION3_CW6_BASE_ADDRESS_BASE_IDX                                                      2
// #define regDMCUB_REGION3_CW7_BASE_ADDRESS                                                               0x01ac
// #define regDMCUB_REGION3_CW7_BASE_ADDRESS_BASE_IDX                                                      2
// #define regDMCUB_REGION3_CW0_TOP_ADDRESS                                                                0x01ad
// #define regDMCUB_REGION3_CW0_TOP_ADDRESS_BASE_IDX                                                       2
// #define regDMCUB_REGION3_CW1_TOP_ADDRESS                                                                0x01ae
// #define regDMCUB_REGION3_CW1_TOP_ADDRESS_BASE_IDX                                                       2
// #define regDMCUB_REGION3_CW2_TOP_ADDRESS                                                                0x01af
// #define regDMCUB_REGION3_CW2_TOP_ADDRESS_BASE_IDX                                                       2
// #define regDMCUB_REGION3_CW3_TOP_ADDRESS                                                                0x01b0
// #define regDMCUB_REGION3_CW3_TOP_ADDRESS_BASE_IDX                                                       2
// #define regDMCUB_REGION3_CW4_TOP_ADDRESS                                                                0x01b1
// #define regDMCUB_REGION3_CW4_TOP_ADDRESS_BASE_IDX                                                       2
// #define regDMCUB_REGION3_CW5_TOP_ADDRESS                                                                0x01b2
// #define regDMCUB_REGION3_CW5_TOP_ADDRESS_BASE_IDX                                                       2
// #define regDMCUB_REGION3_CW6_TOP_ADDRESS                                                                0x01b3
// #define regDMCUB_REGION3_CW6_TOP_ADDRESS_BASE_IDX                                                       2
// #define regDMCUB_REGION3_CW7_TOP_ADDRESS                                                                0x01b4
// #define regDMCUB_REGION3_CW7_TOP_ADDRESS_BASE_IDX                                                       2
// #define regDMCUB_REGION3_CW0_OFFSET                                                                     0x01b5
// #define regDMCUB_REGION3_CW0_OFFSET_BASE_IDX                                                            2
// #define regDMCUB_REGION3_CW0_OFFSET_HIGH                                                                0x01b6
// #define regDMCUB_REGION3_CW0_OFFSET_HIGH_BASE_IDX                                                       2
// #define regDMCUB_REGION3_CW1_OFFSET                                                                     0x01b7
// #define regDMCUB_REGION3_CW1_OFFSET_BASE_IDX                                                            2
// #define regDMCUB_REGION3_CW1_OFFSET_HIGH                                                                0x01b8
// #define regDMCUB_REGION3_CW1_OFFSET_HIGH_BASE_IDX                                                       2
// #define regDMCUB_REGION3_CW2_OFFSET                                                                     0x01b9
// #define regDMCUB_REGION3_CW2_OFFSET_BASE_IDX                                                            2
// #define regDMCUB_REGION3_CW2_OFFSET_HIGH                                                                0x01ba
// #define regDMCUB_REGION3_CW2_OFFSET_HIGH_BASE_IDX                                                       2
// #define regDMCUB_REGION3_CW3_OFFSET                                                                     0x01bb
// #define regDMCUB_REGION3_CW3_OFFSET_BASE_IDX                                                            2
// #define regDMCUB_REGION3_CW3_OFFSET_HIGH                                                                0x01bc
// #define regDMCUB_REGION3_CW3_OFFSET_HIGH_BASE_IDX                                                       2
// #define regDMCUB_REGION3_CW4_OFFSET                                                                     0x01bd
// #define regDMCUB_REGION3_CW4_OFFSET_BASE_IDX                                                            2
// #define regDMCUB_REGION3_CW4_OFFSET_HIGH                                                                0x01be
// #define regDMCUB_REGION3_CW4_OFFSET_HIGH_BASE_IDX                                                       2
// #define regDMCUB_REGION3_CW5_OFFSET                                                                     0x01bf
// #define regDMCUB_REGION3_CW5_OFFSET_BASE_IDX                                                            2
// #define regDMCUB_REGION3_CW5_OFFSET_HIGH                                                                0x01c0
// #define regDMCUB_REGION3_CW5_OFFSET_HIGH_BASE_IDX                                                       2
// #define regDMCUB_REGION3_CW6_OFFSET                                                                     0x01c1
// #define regDMCUB_REGION3_CW6_OFFSET_BASE_IDX                                                            2
// #define regDMCUB_REGION3_CW6_OFFSET_HIGH                                                                0x01c2
// #define regDMCUB_REGION3_CW6_OFFSET_HIGH_BASE_IDX                                                       2
// #define regDMCUB_REGION3_CW7_OFFSET                                                                     0x01c3
// #define regDMCUB_REGION3_CW7_OFFSET_BASE_IDX                                                            2
// #define regDMCUB_REGION3_CW7_OFFSET_HIGH                                                                0x01c4
// #define regDMCUB_REGION3_CW7_OFFSET_HIGH_BASE_IDX                                                       2
// #define regDMCUB_INTERRUPT_ENABLE                                                                       0x01c5
// #define regDMCUB_INTERRUPT_ENABLE_BASE_IDX                                                              2
// #define regDMCUB_INTERRUPT_ACK                                                                          0x01c6
// #define regDMCUB_INTERRUPT_ACK_BASE_IDX                                                                 2
// #define regDMCUB_INTERRUPT_TYPE                                                                         0x01c8
// #define regDMCUB_INTERRUPT_TYPE_BASE_IDX                                                                2
// #define regDMCUB_EXT_INTERRUPT_CTXID                                                                    0x01ca
// #define regDMCUB_EXT_INTERRUPT_CTXID_BASE_IDX                                                           2
// #define regDMCUB_EXT_INTERRUPT_ACK                                                                      0x01cb
// #define regDMCUB_EXT_INTERRUPT_ACK_BASE_IDX                                                             2
// #define regDMCUB_INST_FETCH_FAULT_ADDR                                                                  0x01cc
// #define regDMCUB_INST_FETCH_FAULT_ADDR_BASE_IDX                                                         2
// #define regDMCUB_DATA_WRITE_FAULT_ADDR                                                                  0x01cd
// #define regDMCUB_DATA_WRITE_FAULT_ADDR_BASE_IDX                                                         2
// #define regDMCUB_SEC_CNTL                                                                               0x01ce
// #define regDMCUB_SEC_CNTL_BASE_IDX                                                                      2
// #define regDMCUB_MEM_CNTL                                                                               0x01cf
// #define regDMCUB_MEM_CNTL_BASE_IDX                                                                      2
// #define regDMCUB_INBOX0_BASE_ADDRESS                                                                    0x01d0
// #define regDMCUB_INBOX0_BASE_ADDRESS_BASE_IDX                                                           2
// #define regDMCUB_INBOX0_SIZE                                                                            0x01d1
// #define regDMCUB_INBOX0_SIZE_BASE_IDX                                                                   2
// #define regDMCUB_INBOX0_WPTR                                                                            0x01d2
// #define regDMCUB_INBOX0_WPTR_BASE_IDX                                                                   2
// #define regDMCUB_INBOX0_RPTR                                                                            0x01d3
// #define regDMCUB_INBOX0_RPTR_BASE_IDX                                                                   2
// #define regDMCUB_INBOX1_BASE_ADDRESS                                                                    0x01d4
// #define regDMCUB_INBOX1_BASE_ADDRESS_BASE_IDX                                                           2
// #define regDMCUB_INBOX1_SIZE                                                                            0x01d5
// #define regDMCUB_INBOX1_SIZE_BASE_IDX                                                                   2
// #define regDMCUB_INBOX1_WPTR                                                                            0x01d6
// #define regDMCUB_INBOX1_WPTR_BASE_IDX                                                                   2
// #define regDMCUB_INBOX1_RPTR                                                                            0x01d7
// #define regDMCUB_INBOX1_RPTR_BASE_IDX                                                                   2
// #define regDMCUB_OUTBOX0_BASE_ADDRESS                                                                   0x01d8
// #define regDMCUB_OUTBOX0_BASE_ADDRESS_BASE_IDX                                                          2
// #define regDMCUB_OUTBOX0_SIZE                                                                           0x01d9
// #define regDMCUB_OUTBOX0_SIZE_BASE_IDX                                                                  2
// #define regDMCUB_OUTBOX0_WPTR                                                                           0x01da
// #define regDMCUB_OUTBOX0_WPTR_BASE_IDX                                                                  2
// #define regDMCUB_OUTBOX0_RPTR                                                                           0x01db
// #define regDMCUB_OUTBOX0_RPTR_BASE_IDX                                                                  2
// #define regDMCUB_OUTBOX1_BASE_ADDRESS                                                                   0x01dc
// #define regDMCUB_OUTBOX1_BASE_ADDRESS_BASE_IDX                                                          2
// #define regDMCUB_OUTBOX1_SIZE                                                                           0x01dd
// #define regDMCUB_OUTBOX1_SIZE_BASE_IDX                                                                  2
// #define regDMCUB_OUTBOX1_WPTR                                                                           0x01de
// #define regDMCUB_OUTBOX1_WPTR_BASE_IDX                                                                  2
// #define regDMCUB_OUTBOX1_RPTR                                                                           0x01df
// #define regDMCUB_OUTBOX1_RPTR_BASE_IDX                                                                  2
// #define regDMCUB_TIMER_TRIGGER0                                                                         0x01e0
// #define regDMCUB_TIMER_TRIGGER0_BASE_IDX                                                                2
// #define regDMCUB_TIMER_TRIGGER1                                                                         0x01e1
// #define regDMCUB_TIMER_TRIGGER1_BASE_IDX                                                                2
// #define regDMCUB_TIMER_WINDOW                                                                           0x01e2
// #define regDMCUB_TIMER_WINDOW_BASE_IDX                                                                  2
// #define regDMCUB_SCRATCH0                                                                               0x01e3
// #define regDMCUB_SCRATCH0_BASE_IDX                                                                      2
// #define regDMCUB_SCRATCH1                                                                               0x01e4
// #define regDMCUB_SCRATCH1_BASE_IDX                                                                      2
// #define regDMCUB_SCRATCH2                                                                               0x01e5
// #define regDMCUB_SCRATCH2_BASE_IDX                                                                      2
// #define regDMCUB_SCRATCH3                                                                               0x01e6
// #define regDMCUB_SCRATCH3_BASE_IDX                                                                      2
// #define regDMCUB_SCRATCH4                                                                               0x01e7
// #define regDMCUB_SCRATCH4_BASE_IDX                                                                      2
// #define regDMCUB_SCRATCH5                                                                               0x01e8
// #define regDMCUB_SCRATCH5_BASE_IDX                                                                      2
// #define regDMCUB_SCRATCH6                                                                               0x01e9
// #define regDMCUB_SCRATCH6_BASE_IDX                                                                      2
// #define regDMCUB_SCRATCH7                                                                               0x01ea
// #define regDMCUB_SCRATCH7_BASE_IDX                                                                      2
// #define regDMCUB_SCRATCH8                                                                               0x01eb
// #define regDMCUB_SCRATCH8_BASE_IDX                                                                      2
// #define regDMCUB_SCRATCH9                                                                               0x01ec
// #define regDMCUB_SCRATCH9_BASE_IDX                                                                      2
// #define regDMCUB_SCRATCH10                                                                              0x01ed
// #define regDMCUB_SCRATCH10_BASE_IDX                                                                     2
// #define regDMCUB_SCRATCH11                                                                              0x01ee
// #define regDMCUB_SCRATCH11_BASE_IDX                                                                     2
// #define regDMCUB_SCRATCH12                                                                              0x01ef
// #define regDMCUB_SCRATCH12_BASE_IDX                                                                     2
// #define regDMCUB_SCRATCH13                                                                              0x01f0
// #define regDMCUB_SCRATCH13_BASE_IDX                                                                     2
// #define regDMCUB_SCRATCH14                                                                              0x01f1
// #define regDMCUB_SCRATCH14_BASE_IDX                                                                     2
// #define regDMCUB_SCRATCH15                                                                              0x01f2
// #define regDMCUB_SCRATCH15_BASE_IDX                                                                     2
// #define regDMCUB_CNTL                                                                                   0x01f6
// #define regDMCUB_CNTL_BASE_IDX                                                                          2
// #define regDMCUB_GPINT_DATAIN0                                                                          0x01f7
// #define regDMCUB_GPINT_DATAIN0_BASE_IDX                                                                 2
// #define regDMCUB_GPINT_DATAIN1                                                                          0x01f8
// #define regDMCUB_GPINT_DATAIN1_BASE_IDX                                                                 2
// #define regDMCUB_GPINT_DATAOUT                                                                          0x01f9
// #define regDMCUB_GPINT_DATAOUT_BASE_IDX                                                                 2
// #define regDMCUB_UNDEFINED_ADDRESS_FAULT_ADDR                                                           0x01fa
// #define regDMCUB_UNDEFINED_ADDRESS_FAULT_ADDR_BASE_IDX                                                  2
// #define regDMCUB_LS_WAKE_INT_ENABLE                                                                     0x01fb
// #define regDMCUB_LS_WAKE_INT_ENABLE_BASE_IDX                                                            2
// #define regDMCUB_MEM_PWR_CNTL                                                                           0x01fc
// #define regDMCUB_MEM_PWR_CNTL_BASE_IDX                                                                  2
// #define regDMCUB_TIMER_CURRENT                                                                          0x01fd
// #define regDMCUB_TIMER_CURRENT_BASE_IDX                                                                 2
// #define regDMCUB_PROC_ID                                                                                0x01ff
// #define regDMCUB_PROC_ID_BASE_IDX                                                                       2
// #define regDMCUB_CNTL2                                                                                  0x0200
// #define regDMCUB_CNTL2_BASE_IDX                                                                         2


// addressBlock: dce_dc_wb0_dispdec_dwb_top_dispdec
// base address: 0x0
// #define regDWB_ENABLE_CLK_CTRL                                                                          0x3228
// #define regDWB_ENABLE_CLK_CTRL_BASE_IDX                                                                 2
// #define regDWB_MEM_PWR_CTRL                                                                             0x3229
// #define regDWB_MEM_PWR_CTRL_BASE_IDX                                                                    2
// #define regFC_MODE_CTRL                                                                                 0x322a
// #define regFC_MODE_CTRL_BASE_IDX                                                                        2
// #define regFC_FLOW_CTRL                                                                                 0x322b
// #define regFC_FLOW_CTRL_BASE_IDX                                                                        2
// #define regFC_WINDOW_START                                                                              0x322c
// #define regFC_WINDOW_START_BASE_IDX                                                                     2
// #define regFC_WINDOW_SIZE                                                                               0x322d
// #define regFC_WINDOW_SIZE_BASE_IDX                                                                      2
// #define regFC_SOURCE_SIZE                                                                               0x322e
// #define regFC_SOURCE_SIZE_BASE_IDX                                                                      2
// #define regDWB_UPDATE_CTRL                                                                              0x322f
// #define regDWB_UPDATE_CTRL_BASE_IDX                                                                     2
// #define regDWB_CRC_CTRL                                                                                 0x3230
// #define regDWB_CRC_CTRL_BASE_IDX                                                                        2
// #define regDWB_CRC_MASK_R_G                                                                             0x3231
// #define regDWB_CRC_MASK_R_G_BASE_IDX                                                                    2
// #define regDWB_CRC_MASK_B_A                                                                             0x3232
// #define regDWB_CRC_MASK_B_A_BASE_IDX                                                                    2
// #define regDWB_CRC_VAL_R_G                                                                              0x3233
// #define regDWB_CRC_VAL_R_G_BASE_IDX                                                                     2
// #define regDWB_CRC_VAL_B_A                                                                              0x3234
// #define regDWB_CRC_VAL_B_A_BASE_IDX                                                                     2
// #define regDWB_OUT_CTRL                                                                                 0x3235
// #define regDWB_OUT_CTRL_BASE_IDX                                                                        2
// #define regDWB_MMHUBBUB_BACKPRESSURE_CNT_EN                                                             0x3236
// #define regDWB_MMHUBBUB_BACKPRESSURE_CNT_EN_BASE_IDX                                                    2
// #define regDWB_MMHUBBUB_BACKPRESSURE_CNT                                                                0x3237
// #define regDWB_MMHUBBUB_BACKPRESSURE_CNT_BASE_IDX                                                       2
// #define regDWB_HOST_READ_CONTROL                                                                        0x3238
// #define regDWB_HOST_READ_CONTROL_BASE_IDX                                                               2
// #define regDWB_OVERFLOW_STATUS                                                                          0x3239
// #define regDWB_OVERFLOW_STATUS_BASE_IDX                                                                 2
// #define regDWB_OVERFLOW_COUNTER                                                                         0x323a
// #define regDWB_OVERFLOW_COUNTER_BASE_IDX                                                                2
// #define regDWB_SOFT_RESET                                                                               0x323b
// #define regDWB_SOFT_RESET_BASE_IDX                                                                      2
// #define regDWB_DEBUG_CTRL                                                                               0x323c
// #define regDWB_DEBUG_CTRL_BASE_IDX                                                                      2


// addressBlock: dce_dc_wb0_dispdec_dwbcp_dispdec
// base address: 0x0
// #define regDWB_HDR_MULT_COEF                                                                            0x3294
// #define regDWB_HDR_MULT_COEF_BASE_IDX                                                                   2
// #define regDWB_GAMUT_REMAP_MODE                                                                         0x3295
// #define regDWB_GAMUT_REMAP_MODE_BASE_IDX                                                                2
// #define regDWB_GAMUT_REMAP_COEF_FORMAT                                                                  0x3296
// #define regDWB_GAMUT_REMAP_COEF_FORMAT_BASE_IDX                                                         2
// #define regDWB_GAMUT_REMAPA_C11_C12                                                                     0x3297
// #define regDWB_GAMUT_REMAPA_C11_C12_BASE_IDX                                                            2
// #define regDWB_GAMUT_REMAPA_C13_C14                                                                     0x3298
// #define regDWB_GAMUT_REMAPA_C13_C14_BASE_IDX                                                            2
// #define regDWB_GAMUT_REMAPA_C21_C22                                                                     0x3299
// #define regDWB_GAMUT_REMAPA_C21_C22_BASE_IDX                                                            2
// #define regDWB_GAMUT_REMAPA_C23_C24                                                                     0x329a
// #define regDWB_GAMUT_REMAPA_C23_C24_BASE_IDX                                                            2
// #define regDWB_GAMUT_REMAPA_C31_C32                                                                     0x329b
// #define regDWB_GAMUT_REMAPA_C31_C32_BASE_IDX                                                            2
// #define regDWB_GAMUT_REMAPA_C33_C34                                                                     0x329c
// #define regDWB_GAMUT_REMAPA_C33_C34_BASE_IDX                                                            2
// #define regDWB_GAMUT_REMAPB_C11_C12                                                                     0x329d
// #define regDWB_GAMUT_REMAPB_C11_C12_BASE_IDX                                                            2
// #define regDWB_GAMUT_REMAPB_C13_C14                                                                     0x329e
// #define regDWB_GAMUT_REMAPB_C13_C14_BASE_IDX                                                            2
// #define regDWB_GAMUT_REMAPB_C21_C22                                                                     0x329f
// #define regDWB_GAMUT_REMAPB_C21_C22_BASE_IDX                                                            2
// #define regDWB_GAMUT_REMAPB_C23_C24                                                                     0x32a0
// #define regDWB_GAMUT_REMAPB_C23_C24_BASE_IDX                                                            2
// #define regDWB_GAMUT_REMAPB_C31_C32                                                                     0x32a1
// #define regDWB_GAMUT_REMAPB_C31_C32_BASE_IDX                                                            2
// #define regDWB_GAMUT_REMAPB_C33_C34                                                                     0x32a2
// #define regDWB_GAMUT_REMAPB_C33_C34_BASE_IDX                                                            2
// #define regDWB_OGAM_CONTROL                                                                             0x32a3
// #define regDWB_OGAM_CONTROL_BASE_IDX                                                                    2
// #define regDWB_OGAM_LUT_INDEX                                                                           0x32a4
// #define regDWB_OGAM_LUT_INDEX_BASE_IDX                                                                  2
// #define regDWB_OGAM_LUT_DATA                                                                            0x32a5
// #define regDWB_OGAM_LUT_DATA_BASE_IDX                                                                   2
// #define regDWB_OGAM_LUT_CONTROL                                                                         0x32a6
// #define regDWB_OGAM_LUT_CONTROL_BASE_IDX                                                                2
// #define regDWB_OGAM_RAMA_START_CNTL_B                                                                   0x32a7
// #define regDWB_OGAM_RAMA_START_CNTL_B_BASE_IDX                                                          2
// #define regDWB_OGAM_RAMA_START_CNTL_G                                                                   0x32a8
// #define regDWB_OGAM_RAMA_START_CNTL_G_BASE_IDX                                                          2
// #define regDWB_OGAM_RAMA_START_CNTL_R                                                                   0x32a9
// #define regDWB_OGAM_RAMA_START_CNTL_R_BASE_IDX                                                          2
// #define regDWB_OGAM_RAMA_START_BASE_CNTL_B                                                              0x32aa
// #define regDWB_OGAM_RAMA_START_BASE_CNTL_B_BASE_IDX                                                     2
// #define regDWB_OGAM_RAMA_START_SLOPE_CNTL_B                                                             0x32ab
// #define regDWB_OGAM_RAMA_START_SLOPE_CNTL_B_BASE_IDX                                                    2
// #define regDWB_OGAM_RAMA_START_BASE_CNTL_G                                                              0x32ac
// #define regDWB_OGAM_RAMA_START_BASE_CNTL_G_BASE_IDX                                                     2
// #define regDWB_OGAM_RAMA_START_SLOPE_CNTL_G                                                             0x32ad
// #define regDWB_OGAM_RAMA_START_SLOPE_CNTL_G_BASE_IDX                                                    2
// #define regDWB_OGAM_RAMA_START_BASE_CNTL_R                                                              0x32ae
// #define regDWB_OGAM_RAMA_START_BASE_CNTL_R_BASE_IDX                                                     2
// #define regDWB_OGAM_RAMA_START_SLOPE_CNTL_R                                                             0x32af
// #define regDWB_OGAM_RAMA_START_SLOPE_CNTL_R_BASE_IDX                                                    2
// #define regDWB_OGAM_RAMA_END_CNTL1_B                                                                    0x32b0
// #define regDWB_OGAM_RAMA_END_CNTL1_B_BASE_IDX                                                           2
// #define regDWB_OGAM_RAMA_END_CNTL2_B                                                                    0x32b1
// #define regDWB_OGAM_RAMA_END_CNTL2_B_BASE_IDX                                                           2
// #define regDWB_OGAM_RAMA_END_CNTL1_G                                                                    0x32b2
// #define regDWB_OGAM_RAMA_END_CNTL1_G_BASE_IDX                                                           2
// #define regDWB_OGAM_RAMA_END_CNTL2_G                                                                    0x32b3
// #define regDWB_OGAM_RAMA_END_CNTL2_G_BASE_IDX                                                           2
// #define regDWB_OGAM_RAMA_END_CNTL1_R                                                                    0x32b4
// #define regDWB_OGAM_RAMA_END_CNTL1_R_BASE_IDX                                                           2
// #define regDWB_OGAM_RAMA_END_CNTL2_R                                                                    0x32b5
// #define regDWB_OGAM_RAMA_END_CNTL2_R_BASE_IDX                                                           2
// #define regDWB_OGAM_RAMA_OFFSET_B                                                                       0x32b6
// #define regDWB_OGAM_RAMA_OFFSET_B_BASE_IDX                                                              2
// #define regDWB_OGAM_RAMA_OFFSET_G                                                                       0x32b7
// #define regDWB_OGAM_RAMA_OFFSET_G_BASE_IDX                                                              2
// #define regDWB_OGAM_RAMA_OFFSET_R                                                                       0x32b8
// #define regDWB_OGAM_RAMA_OFFSET_R_BASE_IDX                                                              2
// #define regDWB_OGAM_RAMA_REGION_0_1                                                                     0x32b9
// #define regDWB_OGAM_RAMA_REGION_0_1_BASE_IDX                                                            2
// #define regDWB_OGAM_RAMA_REGION_2_3                                                                     0x32ba
// #define regDWB_OGAM_RAMA_REGION_2_3_BASE_IDX                                                            2
// #define regDWB_OGAM_RAMA_REGION_4_5                                                                     0x32bb
// #define regDWB_OGAM_RAMA_REGION_4_5_BASE_IDX                                                            2
// #define regDWB_OGAM_RAMA_REGION_6_7                                                                     0x32bc
// #define regDWB_OGAM_RAMA_REGION_6_7_BASE_IDX                                                            2
// #define regDWB_OGAM_RAMA_REGION_8_9                                                                     0x32bd
// #define regDWB_OGAM_RAMA_REGION_8_9_BASE_IDX                                                            2
// #define regDWB_OGAM_RAMA_REGION_10_11                                                                   0x32be
// #define regDWB_OGAM_RAMA_REGION_10_11_BASE_IDX                                                          2
// #define regDWB_OGAM_RAMA_REGION_12_13                                                                   0x32bf
// #define regDWB_OGAM_RAMA_REGION_12_13_BASE_IDX                                                          2
// #define regDWB_OGAM_RAMA_REGION_14_15                                                                   0x32c0
// #define regDWB_OGAM_RAMA_REGION_14_15_BASE_IDX                                                          2
// #define regDWB_OGAM_RAMA_REGION_16_17                                                                   0x32c1
// #define regDWB_OGAM_RAMA_REGION_16_17_BASE_IDX                                                          2
// #define regDWB_OGAM_RAMA_REGION_18_19                                                                   0x32c2
// #define regDWB_OGAM_RAMA_REGION_18_19_BASE_IDX                                                          2
// #define regDWB_OGAM_RAMA_REGION_20_21                                                                   0x32c3
// #define regDWB_OGAM_RAMA_REGION_20_21_BASE_IDX                                                          2
// #define regDWB_OGAM_RAMA_REGION_22_23                                                                   0x32c4
// #define regDWB_OGAM_RAMA_REGION_22_23_BASE_IDX                                                          2
// #define regDWB_OGAM_RAMA_REGION_24_25                                                                   0x32c5
// #define regDWB_OGAM_RAMA_REGION_24_25_BASE_IDX                                                          2
// #define regDWB_OGAM_RAMA_REGION_26_27                                                                   0x32c6
// #define regDWB_OGAM_RAMA_REGION_26_27_BASE_IDX                                                          2
// #define regDWB_OGAM_RAMA_REGION_28_29                                                                   0x32c7
// #define regDWB_OGAM_RAMA_REGION_28_29_BASE_IDX                                                          2
// #define regDWB_OGAM_RAMA_REGION_30_31                                                                   0x32c8
// #define regDWB_OGAM_RAMA_REGION_30_31_BASE_IDX                                                          2
// #define regDWB_OGAM_RAMA_REGION_32_33                                                                   0x32c9
// #define regDWB_OGAM_RAMA_REGION_32_33_BASE_IDX                                                          2
// #define regDWB_OGAM_RAMB_START_CNTL_B                                                                   0x32ca
// #define regDWB_OGAM_RAMB_START_CNTL_B_BASE_IDX                                                          2
// #define regDWB_OGAM_RAMB_START_CNTL_G                                                                   0x32cb
// #define regDWB_OGAM_RAMB_START_CNTL_G_BASE_IDX                                                          2
// #define regDWB_OGAM_RAMB_START_CNTL_R                                                                   0x32cc
// #define regDWB_OGAM_RAMB_START_CNTL_R_BASE_IDX                                                          2
// #define regDWB_OGAM_RAMB_START_BASE_CNTL_B                                                              0x32cd
// #define regDWB_OGAM_RAMB_START_BASE_CNTL_B_BASE_IDX                                                     2
// #define regDWB_OGAM_RAMB_START_SLOPE_CNTL_B                                                             0x32ce
// #define regDWB_OGAM_RAMB_START_SLOPE_CNTL_B_BASE_IDX                                                    2
// #define regDWB_OGAM_RAMB_START_BASE_CNTL_G                                                              0x32cf
// #define regDWB_OGAM_RAMB_START_BASE_CNTL_G_BASE_IDX                                                     2
// #define regDWB_OGAM_RAMB_START_SLOPE_CNTL_G                                                             0x32d0
// #define regDWB_OGAM_RAMB_START_SLOPE_CNTL_G_BASE_IDX                                                    2
// #define regDWB_OGAM_RAMB_START_BASE_CNTL_R                                                              0x32d1
// #define regDWB_OGAM_RAMB_START_BASE_CNTL_R_BASE_IDX                                                     2
// #define regDWB_OGAM_RAMB_START_SLOPE_CNTL_R                                                             0x32d2
// #define regDWB_OGAM_RAMB_START_SLOPE_CNTL_R_BASE_IDX                                                    2
// #define regDWB_OGAM_RAMB_END_CNTL1_B                                                                    0x32d3
// #define regDWB_OGAM_RAMB_END_CNTL1_B_BASE_IDX                                                           2
// #define regDWB_OGAM_RAMB_END_CNTL2_B                                                                    0x32d4
// #define regDWB_OGAM_RAMB_END_CNTL2_B_BASE_IDX                                                           2
// #define regDWB_OGAM_RAMB_END_CNTL1_G                                                                    0x32d5
// #define regDWB_OGAM_RAMB_END_CNTL1_G_BASE_IDX                                                           2
// #define regDWB_OGAM_RAMB_END_CNTL2_G                                                                    0x32d6
// #define regDWB_OGAM_RAMB_END_CNTL2_G_BASE_IDX                                                           2
// #define regDWB_OGAM_RAMB_END_CNTL1_R                                                                    0x32d7
// #define regDWB_OGAM_RAMB_END_CNTL1_R_BASE_IDX                                                           2
// #define regDWB_OGAM_RAMB_END_CNTL2_R                                                                    0x32d8
// #define regDWB_OGAM_RAMB_END_CNTL2_R_BASE_IDX                                                           2
// #define regDWB_OGAM_RAMB_OFFSET_B                                                                       0x32d9
// #define regDWB_OGAM_RAMB_OFFSET_B_BASE_IDX                                                              2
// #define regDWB_OGAM_RAMB_OFFSET_G                                                                       0x32da
// #define regDWB_OGAM_RAMB_OFFSET_G_BASE_IDX                                                              2
// #define regDWB_OGAM_RAMB_OFFSET_R                                                                       0x32db
// #define regDWB_OGAM_RAMB_OFFSET_R_BASE_IDX                                                              2
// #define regDWB_OGAM_RAMB_REGION_0_1                                                                     0x32dc
// #define regDWB_OGAM_RAMB_REGION_0_1_BASE_IDX                                                            2
// #define regDWB_OGAM_RAMB_REGION_2_3                                                                     0x32dd
// #define regDWB_OGAM_RAMB_REGION_2_3_BASE_IDX                                                            2
// #define regDWB_OGAM_RAMB_REGION_4_5                                                                     0x32de
// #define regDWB_OGAM_RAMB_REGION_4_5_BASE_IDX                                                            2
// #define regDWB_OGAM_RAMB_REGION_6_7                                                                     0x32df
// #define regDWB_OGAM_RAMB_REGION_6_7_BASE_IDX                                                            2
// #define regDWB_OGAM_RAMB_REGION_8_9                                                                     0x32e0
// #define regDWB_OGAM_RAMB_REGION_8_9_BASE_IDX                                                            2
// #define regDWB_OGAM_RAMB_REGION_10_11                                                                   0x32e1
// #define regDWB_OGAM_RAMB_REGION_10_11_BASE_IDX                                                          2
// #define regDWB_OGAM_RAMB_REGION_12_13                                                                   0x32e2
// #define regDWB_OGAM_RAMB_REGION_12_13_BASE_IDX                                                          2
// #define regDWB_OGAM_RAMB_REGION_14_15                                                                   0x32e3
// #define regDWB_OGAM_RAMB_REGION_14_15_BASE_IDX                                                          2
// #define regDWB_OGAM_RAMB_REGION_16_17                                                                   0x32e4
// #define regDWB_OGAM_RAMB_REGION_16_17_BASE_IDX                                                          2
// #define regDWB_OGAM_RAMB_REGION_18_19                                                                   0x32e5
// #define regDWB_OGAM_RAMB_REGION_18_19_BASE_IDX                                                          2
// #define regDWB_OGAM_RAMB_REGION_20_21                                                                   0x32e6
// #define regDWB_OGAM_RAMB_REGION_20_21_BASE_IDX                                                          2
// #define regDWB_OGAM_RAMB_REGION_22_23                                                                   0x32e7
// #define regDWB_OGAM_RAMB_REGION_22_23_BASE_IDX                                                          2
// #define regDWB_OGAM_RAMB_REGION_24_25                                                                   0x32e8
// #define regDWB_OGAM_RAMB_REGION_24_25_BASE_IDX                                                          2
// #define regDWB_OGAM_RAMB_REGION_26_27                                                                   0x32e9
// #define regDWB_OGAM_RAMB_REGION_26_27_BASE_IDX                                                          2
// #define regDWB_OGAM_RAMB_REGION_28_29                                                                   0x32ea
// #define regDWB_OGAM_RAMB_REGION_28_29_BASE_IDX                                                          2
// #define regDWB_OGAM_RAMB_REGION_30_31                                                                   0x32eb
// #define regDWB_OGAM_RAMB_REGION_30_31_BASE_IDX                                                          2
// #define regDWB_OGAM_RAMB_REGION_32_33                                                                   0x32ec
// #define regDWB_OGAM_RAMB_REGION_32_33_BASE_IDX                                                          2


// addressBlock: dce_dc_wb0_dispdec_wb_dcperfmon_dc_perfmon_dispdec
// base address: 0xca20
// #define regDC_PERFMON3_PERFCOUNTER_CNTL                                                                 0x3288
// #define regDC_PERFMON3_PERFCOUNTER_CNTL_BASE_IDX                                                        2
// #define regDC_PERFMON3_PERFCOUNTER_CNTL2                                                                0x3289
// #define regDC_PERFMON3_PERFCOUNTER_CNTL2_BASE_IDX                                                       2
// #define regDC_PERFMON3_PERFCOUNTER_STATE                                                                0x328a
// #define regDC_PERFMON3_PERFCOUNTER_STATE_BASE_IDX                                                       2
// #define regDC_PERFMON3_PERFMON_CNTL                                                                     0x328b
// #define regDC_PERFMON3_PERFMON_CNTL_BASE_IDX                                                            2
// #define regDC_PERFMON3_PERFMON_CNTL2                                                                    0x328c
// #define regDC_PERFMON3_PERFMON_CNTL2_BASE_IDX                                                           2
// #define regDC_PERFMON3_PERFMON_CVALUE_INT_MISC                                                          0x328d
// #define regDC_PERFMON3_PERFMON_CVALUE_INT_MISC_BASE_IDX                                                 2
// #define regDC_PERFMON3_PERFMON_CVALUE_LOW                                                               0x328e
// #define regDC_PERFMON3_PERFMON_CVALUE_LOW_BASE_IDX                                                      2
// #define regDC_PERFMON3_PERFMON_HI                                                                       0x328f
// #define regDC_PERFMON3_PERFMON_HI_BASE_IDX                                                              2
// #define regDC_PERFMON3_PERFMON_LOW                                                                      0x3290
// #define regDC_PERFMON3_PERFMON_LOW_BASE_IDX                                                             2


// addressBlock: dce_dc_mmhubbub_vga_dispdec
// base address: 0x0
// #define regVGA_MEM_WRITE_PAGE_ADDR                                                                      0x0000
// #define regVGA_MEM_WRITE_PAGE_ADDR_BASE_IDX                                                             0
// #define regVGA_MEM_READ_PAGE_ADDR                                                                       0x0001
// #define regVGA_MEM_READ_PAGE_ADDR_BASE_IDX                                                              0
// #define regVGA_RENDER_CONTROL                                                                           0x0000
// #define regVGA_RENDER_CONTROL_BASE_IDX                                                                  1
// #define regVGA_SEQUENCER_RESET_CONTROL                                                                  0x0001
// #define regVGA_SEQUENCER_RESET_CONTROL_BASE_IDX                                                         1
// #define regVGA_MODE_CONTROL                                                                             0x0002
// #define regVGA_MODE_CONTROL_BASE_IDX                                                                    1
// #define regVGA_SURFACE_PITCH_SELECT                                                                     0x0003
// #define regVGA_SURFACE_PITCH_SELECT_BASE_IDX                                                            1
// #define regVGA_MEMORY_BASE_ADDRESS                                                                      0x0004
// #define regVGA_MEMORY_BASE_ADDRESS_BASE_IDX                                                             1
// #define regVGA_TEST_DEBUG_INDEX                                                                         0x0005
// #define regVGA_TEST_DEBUG_INDEX_BASE_IDX                                                                1
// #define regVGA_DISPBUF1_SURFACE_ADDR                                                                    0x0006
// #define regVGA_DISPBUF1_SURFACE_ADDR_BASE_IDX                                                           1
// #define regVGA_TEST_DEBUG_DATA                                                                          0x0007
// #define regVGA_TEST_DEBUG_DATA_BASE_IDX                                                                 1
// #define regVGA_DISPBUF2_SURFACE_ADDR                                                                    0x0008
// #define regVGA_DISPBUF2_SURFACE_ADDR_BASE_IDX                                                           1
// #define regVGA_MEMORY_BASE_ADDRESS_HIGH                                                                 0x0009
// #define regVGA_MEMORY_BASE_ADDRESS_HIGH_BASE_IDX                                                        1
// #define regVGA_HDP_CONTROL                                                                              0x000a
// #define regVGA_HDP_CONTROL_BASE_IDX                                                                     1
// #define regVGA_CACHE_CONTROL                                                                            0x000b
// #define regVGA_CACHE_CONTROL_BASE_IDX                                                                   1
// #define regD1VGA_CONTROL                                                                                0x000c
// #define regD1VGA_CONTROL_BASE_IDX                                                                       1
// #define regVGA_SECURITY_LEVEL                                                                           0x000d
// #define regVGA_SECURITY_LEVEL_BASE_IDX                                                                  1
// #define regD2VGA_CONTROL                                                                                0x000e
// #define regD2VGA_CONTROL_BASE_IDX                                                                       1
// #define regVGA_HW_DEBUG                                                                                 0x000f
// #define regVGA_HW_DEBUG_BASE_IDX                                                                        1
// #define regVGA_STATUS                                                                                   0x0010
// #define regVGA_STATUS_BASE_IDX                                                                          1
// #define regVGA_STATUS_CLEAR                                                                             0x0012
// #define regVGA_STATUS_CLEAR_BASE_IDX                                                                    1
// #define regVGA_MAIN_CONTROL                                                                             0x0014
// #define regVGA_MAIN_CONTROL_BASE_IDX                                                                    1
// #define regVGA_TEST_CONTROL                                                                             0x0015
// #define regVGA_TEST_CONTROL_BASE_IDX                                                                    1
// #define regVGA_DEBUG_READBACK_INDEX                                                                     0x0016
// #define regVGA_DEBUG_READBACK_INDEX_BASE_IDX                                                            1
// #define regVGA_DEBUG_READBACK_DATA                                                                      0x0017
// #define regVGA_DEBUG_READBACK_DATA_BASE_IDX                                                             1
// #define regVGA_QOS_CTRL                                                                                 0x0018
// #define regVGA_QOS_CTRL_BASE_IDX                                                                        1
// #define regCRTC8_IDX                                                                                    0x002d
// #define regCRTC8_IDX_BASE_IDX                                                                           1
// #define regCRTC8_DATA                                                                                   0x002d
// #define regCRTC8_DATA_BASE_IDX                                                                          1
// #define regGENFC_WT                                                                                     0x002e
// #define regGENFC_WT_BASE_IDX                                                                            1
// #define regGENS1                                                                                        0x002e
// #define regGENS1_BASE_IDX                                                                               1
// #define regATTRDW                                                                                       0x0030
// #define regATTRDW_BASE_IDX                                                                              1
// #define regATTRX                                                                                        0x0030
// #define regATTRX_BASE_IDX                                                                               1
// #define regATTRDR                                                                                       0x0030
// #define regATTRDR_BASE_IDX                                                                              1
// #define regGENMO_WT                                                                                     0x0030
// #define regGENMO_WT_BASE_IDX                                                                            1
// #define regGENS0                                                                                        0x0030
// #define regGENS0_BASE_IDX                                                                               1
// #define regGENENB                                                                                       0x0030
// #define regGENENB_BASE_IDX                                                                              1
// #define regSEQ8_IDX                                                                                     0x0031
// #define regSEQ8_IDX_BASE_IDX                                                                            1
// #define regSEQ8_DATA                                                                                    0x0031
// #define regSEQ8_DATA_BASE_IDX                                                                           1
// #define regDAC_MASK                                                                                     0x0031
// #define regDAC_MASK_BASE_IDX                                                                            1
// #define regDAC_R_INDEX                                                                                  0x0031
// #define regDAC_R_INDEX_BASE_IDX                                                                         1
// #define regDAC_W_INDEX                                                                                  0x0032
// #define regDAC_W_INDEX_BASE_IDX                                                                         1
// #define regDAC_DATA                                                                                     0x0032
// #define regDAC_DATA_BASE_IDX                                                                            1
// #define regGENFC_RD                                                                                     0x0032
// #define regGENFC_RD_BASE_IDX                                                                            1
// #define regGENMO_RD                                                                                     0x0033
// #define regGENMO_RD_BASE_IDX                                                                            1
// #define regGRPH8_IDX                                                                                    0x0033
// #define regGRPH8_IDX_BASE_IDX                                                                           1
// #define regGRPH8_DATA                                                                                   0x0033
// #define regGRPH8_DATA_BASE_IDX                                                                          1
// #define regCRTC8_IDX_1                                                                                  0x0035
// #define regCRTC8_IDX_1_BASE_IDX                                                                         1
// #define regCRTC8_DATA_1                                                                                 0x0035
// #define regCRTC8_DATA_1_BASE_IDX                                                                        1
// #define regGENFC_WT_1                                                                                   0x0036
// #define regGENFC_WT_1_BASE_IDX                                                                          1
// #define regGENS1_1                                                                                      0x0036
// #define regGENS1_1_BASE_IDX                                                                             1
// #define regD3VGA_CONTROL                                                                                0x0038
// #define regD3VGA_CONTROL_BASE_IDX                                                                       1
// #define regD4VGA_CONTROL                                                                                0x0039
// #define regD4VGA_CONTROL_BASE_IDX                                                                       1
// #define regD5VGA_CONTROL                                                                                0x003a
// #define regD5VGA_CONTROL_BASE_IDX                                                                       1
// #define regD6VGA_CONTROL                                                                                0x003b
// #define regD6VGA_CONTROL_BASE_IDX                                                                       1
// #define regVGA_SOURCE_SELECT                                                                            0x003c
// #define regVGA_SOURCE_SELECT_BASE_IDX                                                                   1


// addressBlock: dce_dc_mmhubbub_vgaif_dispdec
// base address: 0x0
// #define regMCIF_CONTROL                                                                                 0x034a
// #define regMCIF_CONTROL_BASE_IDX                                                                        2
// #define regMCIF_WRITE_COMBINE_CONTROL                                                                   0x034b
// #define regMCIF_WRITE_COMBINE_CONTROL_BASE_IDX                                                          2
// #define regMCIF_PHASE0_OUTSTANDING_COUNTER                                                              0x034e
// #define regMCIF_PHASE0_OUTSTANDING_COUNTER_BASE_IDX                                                     2
// #define regMCIF_PHASE1_OUTSTANDING_COUNTER                                                              0x034f
// #define regMCIF_PHASE1_OUTSTANDING_COUNTER_BASE_IDX                                                     2
// #define regMCIF_PHASE2_OUTSTANDING_COUNTER                                                              0x0350
// #define regMCIF_PHASE2_OUTSTANDING_COUNTER_BASE_IDX                                                     2


// addressBlock: dce_dc_mmhubbub_mcif_wb0_dispdec
// base address: 0x0
// #define regMCIF_WB_BUFMGR_SW_CONTROL                                                                    0x0272
// #define regMCIF_WB_BUFMGR_SW_CONTROL_BASE_IDX                                                           2
// #define regMCIF_WB_BUFMGR_STATUS                                                                        0x0274
// #define regMCIF_WB_BUFMGR_STATUS_BASE_IDX                                                               2
// #define regMCIF_WB_BUF_PITCH                                                                            0x0275
// #define regMCIF_WB_BUF_PITCH_BASE_IDX                                                                   2
// #define regMCIF_WB_BUF_1_STATUS                                                                         0x0276
// #define regMCIF_WB_BUF_1_STATUS_BASE_IDX                                                                2
// #define regMCIF_WB_BUF_1_STATUS2                                                                        0x0277
// #define regMCIF_WB_BUF_1_STATUS2_BASE_IDX                                                               2
// #define regMCIF_WB_BUF_2_STATUS                                                                         0x0278
// #define regMCIF_WB_BUF_2_STATUS_BASE_IDX                                                                2
// #define regMCIF_WB_BUF_2_STATUS2                                                                        0x0279
// #define regMCIF_WB_BUF_2_STATUS2_BASE_IDX                                                               2
// #define regMCIF_WB_BUF_3_STATUS                                                                         0x027a
// #define regMCIF_WB_BUF_3_STATUS_BASE_IDX                                                                2
// #define regMCIF_WB_BUF_3_STATUS2                                                                        0x027b
// #define regMCIF_WB_BUF_3_STATUS2_BASE_IDX                                                               2
// #define regMCIF_WB_BUF_4_STATUS                                                                         0x027c
// #define regMCIF_WB_BUF_4_STATUS_BASE_IDX                                                                2
// #define regMCIF_WB_BUF_4_STATUS2                                                                        0x027d
// #define regMCIF_WB_BUF_4_STATUS2_BASE_IDX                                                               2
// #define regMCIF_WB_ARBITRATION_CONTROL                                                                  0x027e
// #define regMCIF_WB_ARBITRATION_CONTROL_BASE_IDX                                                         2
// #define regMCIF_WB_SCLK_CHANGE                                                                          0x027f
// #define regMCIF_WB_SCLK_CHANGE_BASE_IDX                                                                 2
// #define regMCIF_WB_TEST_DEBUG_INDEX                                                                     0x0280
// #define regMCIF_WB_TEST_DEBUG_INDEX_BASE_IDX                                                            2
// #define regMCIF_WB_TEST_DEBUG_DATA                                                                      0x0281
// #define regMCIF_WB_TEST_DEBUG_DATA_BASE_IDX                                                             2
// #define regMCIF_WB_BUF_1_ADDR_Y                                                                         0x0282
// #define regMCIF_WB_BUF_1_ADDR_Y_BASE_IDX                                                                2
// #define regMCIF_WB_BUF_1_ADDR_C                                                                         0x0284
// #define regMCIF_WB_BUF_1_ADDR_C_BASE_IDX                                                                2
// #define regMCIF_WB_BUF_2_ADDR_Y                                                                         0x0286
// #define regMCIF_WB_BUF_2_ADDR_Y_BASE_IDX                                                                2
// #define regMCIF_WB_BUF_2_ADDR_C                                                                         0x0288
// #define regMCIF_WB_BUF_2_ADDR_C_BASE_IDX                                                                2
// #define regMCIF_WB_BUF_3_ADDR_Y                                                                         0x028a
// #define regMCIF_WB_BUF_3_ADDR_Y_BASE_IDX                                                                2
// #define regMCIF_WB_BUF_3_ADDR_C                                                                         0x028c
// #define regMCIF_WB_BUF_3_ADDR_C_BASE_IDX                                                                2
// #define regMCIF_WB_BUF_4_ADDR_Y                                                                         0x028e
// #define regMCIF_WB_BUF_4_ADDR_Y_BASE_IDX                                                                2
// #define regMCIF_WB_BUF_4_ADDR_C                                                                         0x0290
// #define regMCIF_WB_BUF_4_ADDR_C_BASE_IDX                                                                2
// #define regMCIF_WB_BUFMGR_VCE_CONTROL                                                                   0x0292
// #define regMCIF_WB_BUFMGR_VCE_CONTROL_BASE_IDX                                                          2
// #define regMCIF_WB_NB_PSTATE_CONTROL                                                                    0x0293
// #define regMCIF_WB_NB_PSTATE_CONTROL_BASE_IDX                                                           2
// #define regMCIF_WB_CLOCK_GATER_CONTROL                                                                  0x0294
// #define regMCIF_WB_CLOCK_GATER_CONTROL_BASE_IDX                                                         2
// #define regMCIF_WB_SELF_REFRESH_CONTROL                                                                 0x0296
// #define regMCIF_WB_SELF_REFRESH_CONTROL_BASE_IDX                                                        2
// #define regMULTI_LEVEL_QOS_CTRL                                                                         0x0297
// #define regMULTI_LEVEL_QOS_CTRL_BASE_IDX                                                                2
// #define regMCIF_WB_BUF_LUMA_SIZE                                                                        0x0299
// #define regMCIF_WB_BUF_LUMA_SIZE_BASE_IDX                                                               2
// #define regMCIF_WB_BUF_CHROMA_SIZE                                                                      0x029a
// #define regMCIF_WB_BUF_CHROMA_SIZE_BASE_IDX                                                             2
// #define regMCIF_WB_BUF_1_ADDR_Y_HIGH                                                                    0x029b
// #define regMCIF_WB_BUF_1_ADDR_Y_HIGH_BASE_IDX                                                           2
// #define regMCIF_WB_BUF_1_ADDR_C_HIGH                                                                    0x029c
// #define regMCIF_WB_BUF_1_ADDR_C_HIGH_BASE_IDX                                                           2
// #define regMCIF_WB_BUF_2_ADDR_Y_HIGH                                                                    0x029d
// #define regMCIF_WB_BUF_2_ADDR_Y_HIGH_BASE_IDX                                                           2
// #define regMCIF_WB_BUF_2_ADDR_C_HIGH                                                                    0x029e
// #define regMCIF_WB_BUF_2_ADDR_C_HIGH_BASE_IDX                                                           2
// #define regMCIF_WB_BUF_3_ADDR_Y_HIGH                                                                    0x029f
// #define regMCIF_WB_BUF_3_ADDR_Y_HIGH_BASE_IDX                                                           2
// #define regMCIF_WB_BUF_3_ADDR_C_HIGH                                                                    0x02a0
// #define regMCIF_WB_BUF_3_ADDR_C_HIGH_BASE_IDX                                                           2
// #define regMCIF_WB_BUF_4_ADDR_Y_HIGH                                                                    0x02a1
// #define regMCIF_WB_BUF_4_ADDR_Y_HIGH_BASE_IDX                                                           2
// #define regMCIF_WB_BUF_4_ADDR_C_HIGH                                                                    0x02a2
// #define regMCIF_WB_BUF_4_ADDR_C_HIGH_BASE_IDX                                                           2
// #define regMCIF_WB_BUF_1_RESOLUTION                                                                     0x02a3
// #define regMCIF_WB_BUF_1_RESOLUTION_BASE_IDX                                                            2
// #define regMCIF_WB_BUF_2_RESOLUTION                                                                     0x02a4
// #define regMCIF_WB_BUF_2_RESOLUTION_BASE_IDX                                                            2
// #define regMCIF_WB_BUF_3_RESOLUTION                                                                     0x02a5
// #define regMCIF_WB_BUF_3_RESOLUTION_BASE_IDX                                                            2
// #define regMCIF_WB_BUF_4_RESOLUTION                                                                     0x02a6
// #define regMCIF_WB_BUF_4_RESOLUTION_BASE_IDX                                                            2
// #define regMCIF_WB_DRAM_SPEED_CHANGE_DURATION_VBI                                                       0x02a7
// #define regMCIF_WB_DRAM_SPEED_CHANGE_DURATION_VBI_BASE_IDX                                              2
// #define regMCIF_WB_VMID_CONTROL                                                                         0x02a8
// #define regMCIF_WB_VMID_CONTROL_BASE_IDX                                                                2
// #define regMCIF_WB_MIN_TTO                                                                              0x02a9
// #define regMCIF_WB_MIN_TTO_BASE_IDX                                                                     2


// addressBlock: dce_dc_mmhubbub_mmhubbub_dcperfmon_dc_perfmon_dispdec
// base address: 0xd48
// #define regDC_PERFMON4_PERFCOUNTER_CNTL                                                                 0x0352
// #define regDC_PERFMON4_PERFCOUNTER_CNTL_BASE_IDX                                                        2
// #define regDC_PERFMON4_PERFCOUNTER_CNTL2                                                                0x0353
// #define regDC_PERFMON4_PERFCOUNTER_CNTL2_BASE_IDX                                                       2
// #define regDC_PERFMON4_PERFCOUNTER_STATE                                                                0x0354
// #define regDC_PERFMON4_PERFCOUNTER_STATE_BASE_IDX                                                       2
// #define regDC_PERFMON4_PERFMON_CNTL                                                                     0x0355
// #define regDC_PERFMON4_PERFMON_CNTL_BASE_IDX                                                            2
// #define regDC_PERFMON4_PERFMON_CNTL2                                                                    0x0356
// #define regDC_PERFMON4_PERFMON_CNTL2_BASE_IDX                                                           2
// #define regDC_PERFMON4_PERFMON_CVALUE_INT_MISC                                                          0x0357
// #define regDC_PERFMON4_PERFMON_CVALUE_INT_MISC_BASE_IDX                                                 2
// #define regDC_PERFMON4_PERFMON_CVALUE_LOW                                                               0x0358
// #define regDC_PERFMON4_PERFMON_CVALUE_LOW_BASE_IDX                                                      2
// #define regDC_PERFMON4_PERFMON_HI                                                                       0x0359
// #define regDC_PERFMON4_PERFMON_HI_BASE_IDX                                                              2
// #define regDC_PERFMON4_PERFMON_LOW                                                                      0x035a
// #define regDC_PERFMON4_PERFMON_LOW_BASE_IDX                                                             2


// addressBlock: dce_dc_mmhubbub_mmhubbub_dispdec
// base address: 0x0
// #define regMCIF_WB_NB_PSTATE_LATENCY_WATERMARK                                                          0x02aa
// #define regMCIF_WB_NB_PSTATE_LATENCY_WATERMARK_BASE_IDX                                                 2
// #define regMCIF_WB_WATERMARK                                                                            0x02ab
// #define regMCIF_WB_WATERMARK_BASE_IDX                                                                   2
// #define regMMHUBBUB_WARMUP_CONFIG                                                                       0x02ac
// #define regMMHUBBUB_WARMUP_CONFIG_BASE_IDX                                                              2
// #define regMMHUBBUB_WARMUP_CONTROL_STATUS                                                               0x02ad
// #define regMMHUBBUB_WARMUP_CONTROL_STATUS_BASE_IDX                                                      2
// #define regMMHUBBUB_WARMUP_BASE_ADDR_LOW                                                                0x02ae
// #define regMMHUBBUB_WARMUP_BASE_ADDR_LOW_BASE_IDX                                                       2
// #define regMMHUBBUB_WARMUP_BASE_ADDR_HIGH                                                               0x02af
// #define regMMHUBBUB_WARMUP_BASE_ADDR_HIGH_BASE_IDX                                                      2
// #define regMMHUBBUB_WARMUP_ADDR_REGION                                                                  0x02b0
// #define regMMHUBBUB_WARMUP_ADDR_REGION_BASE_IDX                                                         2
// #define regMMHUBBUB_MIN_TTO                                                                             0x02b1
// #define regMMHUBBUB_MIN_TTO_BASE_IDX                                                                    2
// #define regMMHUBBUB_CTRL                                                                                0x0333
// #define regMMHUBBUB_CTRL_BASE_IDX                                                                       2
// #define regWBIF_SMU_WM_CONTROL                                                                          0x0334
// #define regWBIF_SMU_WM_CONTROL_BASE_IDX                                                                 2
// #define regWBIF0_MISC_CTRL                                                                              0x0335
// #define regWBIF0_MISC_CTRL_BASE_IDX                                                                     2
// #define regWBIF0_PHASE0_OUTSTANDING_COUNTER                                                             0x0336
// #define regWBIF0_PHASE0_OUTSTANDING_COUNTER_BASE_IDX                                                    2
// #define regWBIF0_PHASE1_OUTSTANDING_COUNTER                                                             0x0337
// #define regWBIF0_PHASE1_OUTSTANDING_COUNTER_BASE_IDX                                                    2
// #define regVGA_SRC_SPLIT_CNTL                                                                           0x033e
// #define regVGA_SRC_SPLIT_CNTL_BASE_IDX                                                                  2
// #define regMMHUBBUB_MEM_PWR_STATUS                                                                      0x033f
// #define regMMHUBBUB_MEM_PWR_STATUS_BASE_IDX                                                             2
// #define regMMHUBBUB_MEM_PWR_CNTL                                                                        0x0340
// #define regMMHUBBUB_MEM_PWR_CNTL_BASE_IDX                                                               2
// #define regMMHUBBUB_CLOCK_CNTL                                                                          0x0341
// #define regMMHUBBUB_CLOCK_CNTL_BASE_IDX                                                                 2
// #define regMMHUBBUB_SOFT_RESET                                                                          0x0342
// #define regMMHUBBUB_SOFT_RESET_BASE_IDX                                                                 2
// #define regDMU_IF_ERR_STATUS                                                                            0x0346
// #define regDMU_IF_ERR_STATUS_BASE_IDX                                                                   2
// #define regMMHUBBUB_CLIENT_UNIT_ID                                                                      0x0347
// #define regMMHUBBUB_CLIENT_UNIT_ID_BASE_IDX                                                             2
// #define regMMHUBBUB_WARMUP_VMID_CONTROL                                                                 0x0349
// #define regMMHUBBUB_WARMUP_VMID_CONTROL_BASE_IDX                                                        2


// addressBlock: dce_dc_hda_azf0controller_dispdec
// base address: 0x0
// #define regAZALIA_CONTROLLER_CLOCK_GATING                                                               0x03c2
// #define regAZALIA_CONTROLLER_CLOCK_GATING_BASE_IDX                                                      2
// #define regAZALIA_AUDIO_DTO                                                                             0x03c3
// #define regAZALIA_AUDIO_DTO_BASE_IDX                                                                    2
// #define regAZALIA_AUDIO_DTO_CONTROL                                                                     0x03c4
// #define regAZALIA_AUDIO_DTO_CONTROL_BASE_IDX                                                            2
// #define regAZALIA_SOCCLK_CONTROL                                                                        0x03c5
// #define regAZALIA_SOCCLK_CONTROL_BASE_IDX                                                               2
// #define regAZALIA_UNDERFLOW_FILLER_SAMPLE                                                               0x03c6
// #define regAZALIA_UNDERFLOW_FILLER_SAMPLE_BASE_IDX                                                      2
// #define regAZALIA_DATA_DMA_CONTROL                                                                      0x03c7
// #define regAZALIA_DATA_DMA_CONTROL_BASE_IDX                                                             2
// #define regAZALIA_BDL_DMA_CONTROL                                                                       0x03c8
// #define regAZALIA_BDL_DMA_CONTROL_BASE_IDX                                                              2
// #define regAZALIA_RIRB_AND_DP_CONTROL                                                                   0x03c9
// #define regAZALIA_RIRB_AND_DP_CONTROL_BASE_IDX                                                          2
// #define regAZALIA_CORB_DMA_CONTROL                                                                      0x03ca
// #define regAZALIA_CORB_DMA_CONTROL_BASE_IDX                                                             2
// #define regAZALIA_APPLICATION_POSITION_IN_CYCLIC_BUFFER                                                 0x03d1
// #define regAZALIA_APPLICATION_POSITION_IN_CYCLIC_BUFFER_BASE_IDX                                        2
// #define regAZALIA_CYCLIC_BUFFER_SYNC                                                                    0x03d2
// #define regAZALIA_CYCLIC_BUFFER_SYNC_BASE_IDX                                                           2
// #define regAZALIA_OUTPUT_STREAM_ARBITER_CONTROL                                                         0x03d5
// #define regAZALIA_OUTPUT_STREAM_ARBITER_CONTROL_BASE_IDX                                                2
// #define regAZALIA_INPUT_CRC0_CONTROL0                                                                   0x03d9
// #define regAZALIA_INPUT_CRC0_CONTROL0_BASE_IDX                                                          2
// #define regAZALIA_INPUT_CRC0_CONTROL1                                                                   0x03da
// #define regAZALIA_INPUT_CRC0_CONTROL1_BASE_IDX                                                          2
// #define regAZALIA_INPUT_CRC0_CONTROL2                                                                   0x03db
// #define regAZALIA_INPUT_CRC0_CONTROL2_BASE_IDX                                                          2
// #define regAZALIA_INPUT_CRC0_CONTROL3                                                                   0x03dc
// #define regAZALIA_INPUT_CRC0_CONTROL3_BASE_IDX                                                          2
// #define regAZALIA_INPUT_CRC0_RESULT                                                                     0x03dd
// #define regAZALIA_INPUT_CRC0_RESULT_BASE_IDX                                                            2
// #define regAZALIA_INPUT_CRC1_CONTROL0                                                                   0x03de
// #define regAZALIA_INPUT_CRC1_CONTROL0_BASE_IDX                                                          2
// #define regAZALIA_INPUT_CRC1_CONTROL1                                                                   0x03df
// #define regAZALIA_INPUT_CRC1_CONTROL1_BASE_IDX                                                          2
// #define regAZALIA_INPUT_CRC1_CONTROL2                                                                   0x03e0
// #define regAZALIA_INPUT_CRC1_CONTROL2_BASE_IDX                                                          2
// #define regAZALIA_INPUT_CRC1_CONTROL3                                                                   0x03e1
// #define regAZALIA_INPUT_CRC1_CONTROL3_BASE_IDX                                                          2
// #define regAZALIA_INPUT_CRC1_RESULT                                                                     0x03e2
// #define regAZALIA_INPUT_CRC1_RESULT_BASE_IDX                                                            2
// #define regAZALIA_CRC0_CONTROL0                                                                         0x03e3
// #define regAZALIA_CRC0_CONTROL0_BASE_IDX                                                                2
// #define regAZALIA_CRC0_CONTROL1                                                                         0x03e4
// #define regAZALIA_CRC0_CONTROL1_BASE_IDX                                                                2
// #define regAZALIA_CRC0_CONTROL2                                                                         0x03e5
// #define regAZALIA_CRC0_CONTROL2_BASE_IDX                                                                2
// #define regAZALIA_CRC0_CONTROL3                                                                         0x03e6
// #define regAZALIA_CRC0_CONTROL3_BASE_IDX                                                                2
// #define regAZALIA_CRC0_RESULT                                                                           0x03e7
// #define regAZALIA_CRC0_RESULT_BASE_IDX                                                                  2
// #define regAZALIA_CRC1_CONTROL0                                                                         0x03e8
// #define regAZALIA_CRC1_CONTROL0_BASE_IDX                                                                2
// #define regAZALIA_CRC1_CONTROL1                                                                         0x03e9
// #define regAZALIA_CRC1_CONTROL1_BASE_IDX                                                                2
// #define regAZALIA_CRC1_CONTROL2                                                                         0x03ea
// #define regAZALIA_CRC1_CONTROL2_BASE_IDX                                                                2
// #define regAZALIA_CRC1_CONTROL3                                                                         0x03eb
// #define regAZALIA_CRC1_CONTROL3_BASE_IDX                                                                2
// #define regAZALIA_CRC1_RESULT                                                                           0x03ec
// #define regAZALIA_CRC1_RESULT_BASE_IDX                                                                  2
// #define regAZALIA_MEM_PWR_CTRL                                                                          0x03ee
// #define regAZALIA_MEM_PWR_CTRL_BASE_IDX                                                                 2
// #define regAZALIA_MEM_PWR_STATUS                                                                        0x03ef
// #define regAZALIA_MEM_PWR_STATUS_BASE_IDX                                                               2


// addressBlock: dce_dc_hda_azf0root_dispdec
// base address: 0x0
// #define regAZALIA_F0_CODEC_ROOT_PARAMETER_VENDOR_AND_DEVICE_ID                                          0x0406
// #define regAZALIA_F0_CODEC_ROOT_PARAMETER_VENDOR_AND_DEVICE_ID_BASE_IDX                                 2
// #define regAZALIA_F0_CODEC_ROOT_PARAMETER_REVISION_ID                                                   0x0407
// #define regAZALIA_F0_CODEC_ROOT_PARAMETER_REVISION_ID_BASE_IDX                                          2
// #define regAZALIA_F0_CODEC_CHANNEL_COUNT_CONTROL                                                        0x0408
// #define regAZALIA_F0_CODEC_CHANNEL_COUNT_CONTROL_BASE_IDX                                               2
// #define regAZALIA_F0_CODEC_RESYNC_FIFO_CONTROL                                                          0x0409
// #define regAZALIA_F0_CODEC_RESYNC_FIFO_CONTROL_BASE_IDX                                                 2
// #define regAZALIA_F0_CODEC_FUNCTION_PARAMETER_GROUP_TYPE                                                0x040a
// #define regAZALIA_F0_CODEC_FUNCTION_PARAMETER_GROUP_TYPE_BASE_IDX                                       2
// #define regAZALIA_F0_CODEC_FUNCTION_PARAMETER_SUPPORTED_SIZE_RATES                                      0x040b
// #define regAZALIA_F0_CODEC_FUNCTION_PARAMETER_SUPPORTED_SIZE_RATES_BASE_IDX                             2
// #define regAZALIA_F0_CODEC_FUNCTION_PARAMETER_STREAM_FORMATS                                            0x040c
// #define regAZALIA_F0_CODEC_FUNCTION_PARAMETER_STREAM_FORMATS_BASE_IDX                                   2
// #define regAZALIA_F0_CODEC_FUNCTION_PARAMETER_POWER_STATES                                              0x040d
// #define regAZALIA_F0_CODEC_FUNCTION_PARAMETER_POWER_STATES_BASE_IDX                                     2
// #define regAZALIA_F0_CODEC_FUNCTION_CONTROL_POWER_STATE                                                 0x040e
// #define regAZALIA_F0_CODEC_FUNCTION_CONTROL_POWER_STATE_BASE_IDX                                        2
// #define regAZALIA_F0_CODEC_FUNCTION_CONTROL_RESET                                                       0x040f
// #define regAZALIA_F0_CODEC_FUNCTION_CONTROL_RESET_BASE_IDX                                              2
// #define regAZALIA_F0_CODEC_FUNCTION_CONTROL_RESPONSE_SUBSYSTEM_ID                                       0x0410
// #define regAZALIA_F0_CODEC_FUNCTION_CONTROL_RESPONSE_SUBSYSTEM_ID_BASE_IDX                              2
// #define regAZALIA_F0_CODEC_FUNCTION_CONTROL_CONVERTER_SYNCHRONIZATION                                   0x0411
// #define regAZALIA_F0_CODEC_FUNCTION_CONTROL_CONVERTER_SYNCHRONIZATION_BASE_IDX                          2
// #define regCC_RCU_DC_AUDIO_PORT_CONNECTIVITY                                                            0x0412
// #define regCC_RCU_DC_AUDIO_PORT_CONNECTIVITY_BASE_IDX                                                   2
// #define regCC_RCU_DC_AUDIO_INPUT_PORT_CONNECTIVITY                                                      0x0413
// #define regCC_RCU_DC_AUDIO_INPUT_PORT_CONNECTIVITY_BASE_IDX                                             2
// #define regAZALIA_F0_GTC_GROUP_OFFSET0                                                                  0x0415
// #define regAZALIA_F0_GTC_GROUP_OFFSET0_BASE_IDX                                                         2
// #define regAZALIA_F0_GTC_GROUP_OFFSET1                                                                  0x0416
// #define regAZALIA_F0_GTC_GROUP_OFFSET1_BASE_IDX                                                         2
// #define regAZALIA_F0_GTC_GROUP_OFFSET2                                                                  0x0417
// #define regAZALIA_F0_GTC_GROUP_OFFSET2_BASE_IDX                                                         2
// #define regAZALIA_F0_GTC_GROUP_OFFSET3                                                                  0x0418
// #define regAZALIA_F0_GTC_GROUP_OFFSET3_BASE_IDX                                                         2
// #define regAZALIA_F0_GTC_GROUP_OFFSET4                                                                  0x0419
// #define regAZALIA_F0_GTC_GROUP_OFFSET4_BASE_IDX                                                         2
// #define regAZALIA_F0_GTC_GROUP_OFFSET5                                                                  0x041a
// #define regAZALIA_F0_GTC_GROUP_OFFSET5_BASE_IDX                                                         2
// #define regAZALIA_F0_GTC_GROUP_OFFSET6                                                                  0x041b
// #define regAZALIA_F0_GTC_GROUP_OFFSET6_BASE_IDX                                                         2
// #define regREG_DC_AUDIO_PORT_CONNECTIVITY                                                               0x041c
// #define regREG_DC_AUDIO_PORT_CONNECTIVITY_BASE_IDX                                                      2
// #define regREG_DC_AUDIO_INPUT_PORT_CONNECTIVITY                                                         0x041d
// #define regREG_DC_AUDIO_INPUT_PORT_CONNECTIVITY_BASE_IDX                                                2


// addressBlock: dce_dc_hda_az_misc_dispdec
// base address: 0x0
// #define regAZ_CLOCK_CNTL                                                                                0x0372
// #define regAZ_CLOCK_CNTL_BASE_IDX                                                                       2


// addressBlock: dce_dc_hda_az_dcperfmon_dc_perfmon_dispdec
// base address: 0xde8
// #define regDC_PERFMON5_PERFCOUNTER_CNTL                                                                 0x037a
// #define regDC_PERFMON5_PERFCOUNTER_CNTL_BASE_IDX                                                        2
// #define regDC_PERFMON5_PERFCOUNTER_CNTL2                                                                0x037b
// #define regDC_PERFMON5_PERFCOUNTER_CNTL2_BASE_IDX                                                       2
// #define regDC_PERFMON5_PERFCOUNTER_STATE                                                                0x037c
// #define regDC_PERFMON5_PERFCOUNTER_STATE_BASE_IDX                                                       2
// #define regDC_PERFMON5_PERFMON_CNTL                                                                     0x037d
// #define regDC_PERFMON5_PERFMON_CNTL_BASE_IDX                                                            2
// #define regDC_PERFMON5_PERFMON_CNTL2                                                                    0x037e
// #define regDC_PERFMON5_PERFMON_CNTL2_BASE_IDX                                                           2
// #define regDC_PERFMON5_PERFMON_CVALUE_INT_MISC                                                          0x037f
// #define regDC_PERFMON5_PERFMON_CVALUE_INT_MISC_BASE_IDX                                                 2
// #define regDC_PERFMON5_PERFMON_CVALUE_LOW                                                               0x0380
// #define regDC_PERFMON5_PERFMON_CVALUE_LOW_BASE_IDX                                                      2
// #define regDC_PERFMON5_PERFMON_HI                                                                       0x0381
// #define regDC_PERFMON5_PERFMON_HI_BASE_IDX                                                              2
// #define regDC_PERFMON5_PERFMON_LOW                                                                      0x0382
// #define regDC_PERFMON5_PERFMON_LOW_BASE_IDX                                                             2


// addressBlock: dce_dc_hda_azf0stream0_dispdec
// base address: 0x0
// #define regAZF0STREAM0_AZALIA_STREAM_INDEX                                                              0x035e
// #define regAZF0STREAM0_AZALIA_STREAM_INDEX_BASE_IDX                                                     2
// #define regAZF0STREAM0_AZALIA_STREAM_DATA                                                               0x035f
// #define regAZF0STREAM0_AZALIA_STREAM_DATA_BASE_IDX                                                      2


// addressBlock: dce_dc_hda_azf0stream1_dispdec
// base address: 0x8
// #define regAZF0STREAM1_AZALIA_STREAM_INDEX                                                              0x0360
// #define regAZF0STREAM1_AZALIA_STREAM_INDEX_BASE_IDX                                                     2
// #define regAZF0STREAM1_AZALIA_STREAM_DATA                                                               0x0361
// #define regAZF0STREAM1_AZALIA_STREAM_DATA_BASE_IDX                                                      2


// addressBlock: dce_dc_hda_azf0stream2_dispdec
// base address: 0x10
// #define regAZF0STREAM2_AZALIA_STREAM_INDEX                                                              0x0362
// #define regAZF0STREAM2_AZALIA_STREAM_INDEX_BASE_IDX                                                     2
// #define regAZF0STREAM2_AZALIA_STREAM_DATA                                                               0x0363
// #define regAZF0STREAM2_AZALIA_STREAM_DATA_BASE_IDX                                                      2


// addressBlock: dce_dc_hda_azf0stream3_dispdec
// base address: 0x18
// #define regAZF0STREAM3_AZALIA_STREAM_INDEX                                                              0x0364
// #define regAZF0STREAM3_AZALIA_STREAM_INDEX_BASE_IDX                                                     2
// #define regAZF0STREAM3_AZALIA_STREAM_DATA                                                               0x0365
// #define regAZF0STREAM3_AZALIA_STREAM_DATA_BASE_IDX                                                      2


// addressBlock: dce_dc_hda_azf0stream4_dispdec
// base address: 0x20
// #define regAZF0STREAM4_AZALIA_STREAM_INDEX                                                              0x0366
// #define regAZF0STREAM4_AZALIA_STREAM_INDEX_BASE_IDX                                                     2
// #define regAZF0STREAM4_AZALIA_STREAM_DATA                                                               0x0367
// #define regAZF0STREAM4_AZALIA_STREAM_DATA_BASE_IDX                                                      2


// addressBlock: dce_dc_hda_azf0stream5_dispdec
// base address: 0x28
// #define regAZF0STREAM5_AZALIA_STREAM_INDEX                                                              0x0368
// #define regAZF0STREAM5_AZALIA_STREAM_INDEX_BASE_IDX                                                     2
// #define regAZF0STREAM5_AZALIA_STREAM_DATA                                                               0x0369
// #define regAZF0STREAM5_AZALIA_STREAM_DATA_BASE_IDX                                                      2


// addressBlock: dce_dc_hda_azf0stream6_dispdec
// base address: 0x30
// #define regAZF0STREAM6_AZALIA_STREAM_INDEX                                                              0x036a
// #define regAZF0STREAM6_AZALIA_STREAM_INDEX_BASE_IDX                                                     2
// #define regAZF0STREAM6_AZALIA_STREAM_DATA                                                               0x036b
// #define regAZF0STREAM6_AZALIA_STREAM_DATA_BASE_IDX                                                      2


// addressBlock: dce_dc_hda_azf0stream7_dispdec
// base address: 0x38
// #define regAZF0STREAM7_AZALIA_STREAM_INDEX                                                              0x036c
// #define regAZF0STREAM7_AZALIA_STREAM_INDEX_BASE_IDX                                                     2
// #define regAZF0STREAM7_AZALIA_STREAM_DATA                                                               0x036d
// #define regAZF0STREAM7_AZALIA_STREAM_DATA_BASE_IDX                                                      2


// addressBlock: dce_dc_hda_azf0stream8_dispdec
// base address: 0x320
// #define regAZF0STREAM8_AZALIA_STREAM_INDEX                                                              0x0426
// #define regAZF0STREAM8_AZALIA_STREAM_INDEX_BASE_IDX                                                     2
// #define regAZF0STREAM8_AZALIA_STREAM_DATA                                                               0x0427
// #define regAZF0STREAM8_AZALIA_STREAM_DATA_BASE_IDX                                                      2


// addressBlock: dce_dc_hda_azf0stream9_dispdec
// base address: 0x328
// #define regAZF0STREAM9_AZALIA_STREAM_INDEX                                                              0x0428
// #define regAZF0STREAM9_AZALIA_STREAM_INDEX_BASE_IDX                                                     2
// #define regAZF0STREAM9_AZALIA_STREAM_DATA                                                               0x0429
// #define regAZF0STREAM9_AZALIA_STREAM_DATA_BASE_IDX                                                      2


// addressBlock: dce_dc_hda_azf0stream10_dispdec
// base address: 0x330
// #define regAZF0STREAM10_AZALIA_STREAM_INDEX                                                             0x042a
// #define regAZF0STREAM10_AZALIA_STREAM_INDEX_BASE_IDX                                                    2
// #define regAZF0STREAM10_AZALIA_STREAM_DATA                                                              0x042b
// #define regAZF0STREAM10_AZALIA_STREAM_DATA_BASE_IDX                                                     2


// addressBlock: dce_dc_hda_azf0stream11_dispdec
// base address: 0x338
// #define regAZF0STREAM11_AZALIA_STREAM_INDEX                                                             0x042c
// #define regAZF0STREAM11_AZALIA_STREAM_INDEX_BASE_IDX                                                    2
// #define regAZF0STREAM11_AZALIA_STREAM_DATA                                                              0x042d
// #define regAZF0STREAM11_AZALIA_STREAM_DATA_BASE_IDX                                                     2


// addressBlock: dce_dc_hda_azf0stream12_dispdec
// base address: 0x340
// #define regAZF0STREAM12_AZALIA_STREAM_INDEX                                                             0x042e
// #define regAZF0STREAM12_AZALIA_STREAM_INDEX_BASE_IDX                                                    2
// #define regAZF0STREAM12_AZALIA_STREAM_DATA                                                              0x042f
// #define regAZF0STREAM12_AZALIA_STREAM_DATA_BASE_IDX                                                     2


// addressBlock: dce_dc_hda_azf0stream13_dispdec
// base address: 0x348
// #define regAZF0STREAM13_AZALIA_STREAM_INDEX                                                             0x0430
// #define regAZF0STREAM13_AZALIA_STREAM_INDEX_BASE_IDX                                                    2
// #define regAZF0STREAM13_AZALIA_STREAM_DATA                                                              0x0431
// #define regAZF0STREAM13_AZALIA_STREAM_DATA_BASE_IDX                                                     2


// addressBlock: dce_dc_hda_azf0stream14_dispdec
// base address: 0x350
// #define regAZF0STREAM14_AZALIA_STREAM_INDEX                                                             0x0432
// #define regAZF0STREAM14_AZALIA_STREAM_INDEX_BASE_IDX                                                    2
// #define regAZF0STREAM14_AZALIA_STREAM_DATA                                                              0x0433
// #define regAZF0STREAM14_AZALIA_STREAM_DATA_BASE_IDX                                                     2


// addressBlock: dce_dc_hda_azf0stream15_dispdec
// base address: 0x358
// #define regAZF0STREAM15_AZALIA_STREAM_INDEX                                                             0x0434
// #define regAZF0STREAM15_AZALIA_STREAM_INDEX_BASE_IDX                                                    2
// #define regAZF0STREAM15_AZALIA_STREAM_DATA                                                              0x0435
// #define regAZF0STREAM15_AZALIA_STREAM_DATA_BASE_IDX                                                     2


// addressBlock: dce_dc_hda_azf0endpoint0_dispdec
// base address: 0x0
// #define regAZF0ENDPOINT0_AZALIA_F0_CODEC_ENDPOINT_INDEX                                                 0x0386
// #define regAZF0ENDPOINT0_AZALIA_F0_CODEC_ENDPOINT_INDEX_BASE_IDX                                        2
// #define regAZF0ENDPOINT0_AZALIA_F0_CODEC_ENDPOINT_DATA                                                  0x0387
// #define regAZF0ENDPOINT0_AZALIA_F0_CODEC_ENDPOINT_DATA_BASE_IDX                                         2


// addressBlock: dce_dc_hda_azf0endpoint1_dispdec
// base address: 0x18
// #define regAZF0ENDPOINT1_AZALIA_F0_CODEC_ENDPOINT_INDEX                                                 0x038c
// #define regAZF0ENDPOINT1_AZALIA_F0_CODEC_ENDPOINT_INDEX_BASE_IDX                                        2
// #define regAZF0ENDPOINT1_AZALIA_F0_CODEC_ENDPOINT_DATA                                                  0x038d
// #define regAZF0ENDPOINT1_AZALIA_F0_CODEC_ENDPOINT_DATA_BASE_IDX                                         2


// addressBlock: dce_dc_hda_azf0endpoint2_dispdec
// base address: 0x30
// #define regAZF0ENDPOINT2_AZALIA_F0_CODEC_ENDPOINT_INDEX                                                 0x0392
// #define regAZF0ENDPOINT2_AZALIA_F0_CODEC_ENDPOINT_INDEX_BASE_IDX                                        2
// #define regAZF0ENDPOINT2_AZALIA_F0_CODEC_ENDPOINT_DATA                                                  0x0393
// #define regAZF0ENDPOINT2_AZALIA_F0_CODEC_ENDPOINT_DATA_BASE_IDX                                         2


// addressBlock: dce_dc_hda_azf0endpoint3_dispdec
// base address: 0x48
// #define regAZF0ENDPOINT3_AZALIA_F0_CODEC_ENDPOINT_INDEX                                                 0x0398
// #define regAZF0ENDPOINT3_AZALIA_F0_CODEC_ENDPOINT_INDEX_BASE_IDX                                        2
// #define regAZF0ENDPOINT3_AZALIA_F0_CODEC_ENDPOINT_DATA                                                  0x0399
// #define regAZF0ENDPOINT3_AZALIA_F0_CODEC_ENDPOINT_DATA_BASE_IDX                                         2


// addressBlock: dce_dc_hda_azf0endpoint4_dispdec
// base address: 0x60
// #define regAZF0ENDPOINT4_AZALIA_F0_CODEC_ENDPOINT_INDEX                                                 0x039e
// #define regAZF0ENDPOINT4_AZALIA_F0_CODEC_ENDPOINT_INDEX_BASE_IDX                                        2
// #define regAZF0ENDPOINT4_AZALIA_F0_CODEC_ENDPOINT_DATA                                                  0x039f
// #define regAZF0ENDPOINT4_AZALIA_F0_CODEC_ENDPOINT_DATA_BASE_IDX                                         2


// addressBlock: dce_dc_hda_azf0endpoint5_dispdec
// base address: 0x78
// #define regAZF0ENDPOINT5_AZALIA_F0_CODEC_ENDPOINT_INDEX                                                 0x03a4
// #define regAZF0ENDPOINT5_AZALIA_F0_CODEC_ENDPOINT_INDEX_BASE_IDX                                        2
// #define regAZF0ENDPOINT5_AZALIA_F0_CODEC_ENDPOINT_DATA                                                  0x03a5
// #define regAZF0ENDPOINT5_AZALIA_F0_CODEC_ENDPOINT_DATA_BASE_IDX                                         2


// addressBlock: dce_dc_hda_azf0endpoint6_dispdec
// base address: 0x90
// #define regAZF0ENDPOINT6_AZALIA_F0_CODEC_ENDPOINT_INDEX                                                 0x03aa
// #define regAZF0ENDPOINT6_AZALIA_F0_CODEC_ENDPOINT_INDEX_BASE_IDX                                        2
// #define regAZF0ENDPOINT6_AZALIA_F0_CODEC_ENDPOINT_DATA                                                  0x03ab
// #define regAZF0ENDPOINT6_AZALIA_F0_CODEC_ENDPOINT_DATA_BASE_IDX                                         2


// addressBlock: dce_dc_hda_azf0endpoint7_dispdec
// base address: 0xa8
// #define regAZF0ENDPOINT7_AZALIA_F0_CODEC_ENDPOINT_INDEX                                                 0x03b0
// #define regAZF0ENDPOINT7_AZALIA_F0_CODEC_ENDPOINT_INDEX_BASE_IDX                                        2
// #define regAZF0ENDPOINT7_AZALIA_F0_CODEC_ENDPOINT_DATA                                                  0x03b1
// #define regAZF0ENDPOINT7_AZALIA_F0_CODEC_ENDPOINT_DATA_BASE_IDX                                         2


// addressBlock: dce_dc_hda_azf0inputendpoint0_dispdec
// base address: 0x0
// #define regAZF0INPUTENDPOINT0_AZALIA_F0_CODEC_INPUT_ENDPOINT_INDEX                                      0x043a
// #define regAZF0INPUTENDPOINT0_AZALIA_F0_CODEC_INPUT_ENDPOINT_INDEX_BASE_IDX                             2
// #define regAZF0INPUTENDPOINT0_AZALIA_F0_CODEC_INPUT_ENDPOINT_DATA                                       0x043b
// #define regAZF0INPUTENDPOINT0_AZALIA_F0_CODEC_INPUT_ENDPOINT_DATA_BASE_IDX                              2


// addressBlock: dce_dc_hda_azf0inputendpoint1_dispdec
// base address: 0x10
// #define regAZF0INPUTENDPOINT1_AZALIA_F0_CODEC_INPUT_ENDPOINT_INDEX                                      0x043e
// #define regAZF0INPUTENDPOINT1_AZALIA_F0_CODEC_INPUT_ENDPOINT_INDEX_BASE_IDX                             2
// #define regAZF0INPUTENDPOINT1_AZALIA_F0_CODEC_INPUT_ENDPOINT_DATA                                       0x043f
// #define regAZF0INPUTENDPOINT1_AZALIA_F0_CODEC_INPUT_ENDPOINT_DATA_BASE_IDX                              2


// addressBlock: dce_dc_hda_azf0inputendpoint2_dispdec
// base address: 0x20
// #define regAZF0INPUTENDPOINT2_AZALIA_F0_CODEC_INPUT_ENDPOINT_INDEX                                      0x0442
// #define regAZF0INPUTENDPOINT2_AZALIA_F0_CODEC_INPUT_ENDPOINT_INDEX_BASE_IDX                             2
// #define regAZF0INPUTENDPOINT2_AZALIA_F0_CODEC_INPUT_ENDPOINT_DATA                                       0x0443
// #define regAZF0INPUTENDPOINT2_AZALIA_F0_CODEC_INPUT_ENDPOINT_DATA_BASE_IDX                              2


// addressBlock: dce_dc_hda_azf0inputendpoint3_dispdec
// base address: 0x30
// #define regAZF0INPUTENDPOINT3_AZALIA_F0_CODEC_INPUT_ENDPOINT_INDEX                                      0x0446
// #define regAZF0INPUTENDPOINT3_AZALIA_F0_CODEC_INPUT_ENDPOINT_INDEX_BASE_IDX                             2
// #define regAZF0INPUTENDPOINT3_AZALIA_F0_CODEC_INPUT_ENDPOINT_DATA                                       0x0447
// #define regAZF0INPUTENDPOINT3_AZALIA_F0_CODEC_INPUT_ENDPOINT_DATA_BASE_IDX                              2


// addressBlock: dce_dc_hda_azf0inputendpoint4_dispdec
// base address: 0x40
// #define regAZF0INPUTENDPOINT4_AZALIA_F0_CODEC_INPUT_ENDPOINT_INDEX                                      0x044a
// #define regAZF0INPUTENDPOINT4_AZALIA_F0_CODEC_INPUT_ENDPOINT_INDEX_BASE_IDX                             2
// #define regAZF0INPUTENDPOINT4_AZALIA_F0_CODEC_INPUT_ENDPOINT_DATA                                       0x044b
// #define regAZF0INPUTENDPOINT4_AZALIA_F0_CODEC_INPUT_ENDPOINT_DATA_BASE_IDX                              2


// addressBlock: dce_dc_hda_azf0inputendpoint5_dispdec
// base address: 0x50
// #define regAZF0INPUTENDPOINT5_AZALIA_F0_CODEC_INPUT_ENDPOINT_INDEX                                      0x044e
// #define regAZF0INPUTENDPOINT5_AZALIA_F0_CODEC_INPUT_ENDPOINT_INDEX_BASE_IDX                             2
// #define regAZF0INPUTENDPOINT5_AZALIA_F0_CODEC_INPUT_ENDPOINT_DATA                                       0x044f
// #define regAZF0INPUTENDPOINT5_AZALIA_F0_CODEC_INPUT_ENDPOINT_DATA_BASE_IDX                              2


// addressBlock: dce_dc_hda_azf0inputendpoint6_dispdec
// base address: 0x60
// #define regAZF0INPUTENDPOINT6_AZALIA_F0_CODEC_INPUT_ENDPOINT_INDEX                                      0x0452
// #define regAZF0INPUTENDPOINT6_AZALIA_F0_CODEC_INPUT_ENDPOINT_INDEX_BASE_IDX                             2
// #define regAZF0INPUTENDPOINT6_AZALIA_F0_CODEC_INPUT_ENDPOINT_DATA                                       0x0453
// #define regAZF0INPUTENDPOINT6_AZALIA_F0_CODEC_INPUT_ENDPOINT_DATA_BASE_IDX                              2


// addressBlock: dce_dc_hda_azf0inputendpoint7_dispdec
// base address: 0x70
// #define regAZF0INPUTENDPOINT7_AZALIA_F0_CODEC_INPUT_ENDPOINT_INDEX                                      0x0456
// #define regAZF0INPUTENDPOINT7_AZALIA_F0_CODEC_INPUT_ENDPOINT_INDEX_BASE_IDX                             2
// #define regAZF0INPUTENDPOINT7_AZALIA_F0_CODEC_INPUT_ENDPOINT_DATA                                       0x0457
// #define regAZF0INPUTENDPOINT7_AZALIA_F0_CODEC_INPUT_ENDPOINT_DATA_BASE_IDX                              2


// addressBlock: dce_dc_dchubbubl_hubbub_dispdec
// base address: 0x0
// #define regDCHUBBUB_ARB_DF_REQ_OUTSTAND                                                                 0x04f9
// #define regDCHUBBUB_ARB_DF_REQ_OUTSTAND_BASE_IDX                                                        2
// #define regDCHUBBUB_ARB_SAT_LEVEL                                                                       0x04fa
// #define regDCHUBBUB_ARB_SAT_LEVEL_BASE_IDX                                                              2
// #define regDCHUBBUB_ARB_QOS_FORCE                                                                       0x04fb
// #define regDCHUBBUB_ARB_QOS_FORCE_BASE_IDX                                                              2
// #define regDCHUBBUB_ARB_DRAM_STATE_CNTL                                                                 0x04fc
// #define regDCHUBBUB_ARB_DRAM_STATE_CNTL_BASE_IDX                                                        2
// #define regDCHUBBUB_ARB_DATA_URGENCY_WATERMARK_A                                                        0x04fd
// #define regDCHUBBUB_ARB_DATA_URGENCY_WATERMARK_A_BASE_IDX                                               2
// #define regDCHUBBUB_ARB_REFCYC_PER_TRIP_TO_MEMORY_A                                                     0x04fe
// #define regDCHUBBUB_ARB_REFCYC_PER_TRIP_TO_MEMORY_A_BASE_IDX                                            2
// #define regDCHUBBUB_ARB_ALLOW_SR_ENTER_WATERMARK_A                                                      0x04ff
// #define regDCHUBBUB_ARB_ALLOW_SR_ENTER_WATERMARK_A_BASE_IDX                                             2
// #define regDCHUBBUB_ARB_ALLOW_SR_ENTER_WATERMARK_Z8_A                                                   0x0500
// #define regDCHUBBUB_ARB_ALLOW_SR_ENTER_WATERMARK_Z8_A_BASE_IDX                                          2
// #define regDCHUBBUB_ARB_ALLOW_SR_EXIT_WATERMARK_A                                                       0x0501
// #define regDCHUBBUB_ARB_ALLOW_SR_EXIT_WATERMARK_A_BASE_IDX                                              2
// #define regDCHUBBUB_ARB_ALLOW_SR_EXIT_WATERMARK_Z8_A                                                    0x0502
// #define regDCHUBBUB_ARB_ALLOW_SR_EXIT_WATERMARK_Z8_A_BASE_IDX                                           2
// #define regDCHUBBUB_ARB_ALLOW_DRAM_CLK_CHANGE_WATERMARK_A                                               0x0503
// #define regDCHUBBUB_ARB_ALLOW_DRAM_CLK_CHANGE_WATERMARK_A_BASE_IDX                                      2
// #define regDCHUBBUB_ARB_FRAC_URG_BW_NOM_A                                                               0x0504
// #define regDCHUBBUB_ARB_FRAC_URG_BW_NOM_A_BASE_IDX                                                      2
// #define regDCHUBBUB_ARB_FRAC_URG_BW_FLIP_A                                                              0x0505
// #define regDCHUBBUB_ARB_FRAC_URG_BW_FLIP_A_BASE_IDX                                                     2
// #define regDCHUBBUB_ARB_DATA_URGENCY_WATERMARK_B                                                        0x0506
// #define regDCHUBBUB_ARB_DATA_URGENCY_WATERMARK_B_BASE_IDX                                               2
// #define regDCHUBBUB_ARB_REFCYC_PER_TRIP_TO_MEMORY_B                                                     0x0507
// #define regDCHUBBUB_ARB_REFCYC_PER_TRIP_TO_MEMORY_B_BASE_IDX                                            2
// #define regDCHUBBUB_ARB_ALLOW_SR_ENTER_WATERMARK_B                                                      0x0508
// #define regDCHUBBUB_ARB_ALLOW_SR_ENTER_WATERMARK_B_BASE_IDX                                             2
// #define regDCHUBBUB_ARB_ALLOW_SR_ENTER_WATERMARK_Z8_B                                                   0x0509
// #define regDCHUBBUB_ARB_ALLOW_SR_ENTER_WATERMARK_Z8_B_BASE_IDX                                          2
// #define regDCHUBBUB_ARB_ALLOW_SR_EXIT_WATERMARK_B                                                       0x050a
// #define regDCHUBBUB_ARB_ALLOW_SR_EXIT_WATERMARK_B_BASE_IDX                                              2
// #define regDCHUBBUB_ARB_ALLOW_SR_EXIT_WATERMARK_Z8_B                                                    0x050b
// #define regDCHUBBUB_ARB_ALLOW_SR_EXIT_WATERMARK_Z8_B_BASE_IDX                                           2
// #define regDCHUBBUB_ARB_ALLOW_DRAM_CLK_CHANGE_WATERMARK_B                                               0x050c
// #define regDCHUBBUB_ARB_ALLOW_DRAM_CLK_CHANGE_WATERMARK_B_BASE_IDX                                      2
// #define regDCHUBBUB_ARB_FRAC_URG_BW_NOM_B                                                               0x050d
// #define regDCHUBBUB_ARB_FRAC_URG_BW_NOM_B_BASE_IDX                                                      2
// #define regDCHUBBUB_ARB_FRAC_URG_BW_FLIP_B                                                              0x050e
// #define regDCHUBBUB_ARB_FRAC_URG_BW_FLIP_B_BASE_IDX                                                     2
// #define regDCHUBBUB_ARB_DATA_URGENCY_WATERMARK_C                                                        0x050f
// #define regDCHUBBUB_ARB_DATA_URGENCY_WATERMARK_C_BASE_IDX                                               2
// #define regDCHUBBUB_ARB_REFCYC_PER_TRIP_TO_MEMORY_C                                                     0x0510
// #define regDCHUBBUB_ARB_REFCYC_PER_TRIP_TO_MEMORY_C_BASE_IDX                                            2
// #define regDCHUBBUB_ARB_ALLOW_SR_ENTER_WATERMARK_C                                                      0x0511
// #define regDCHUBBUB_ARB_ALLOW_SR_ENTER_WATERMARK_C_BASE_IDX                                             2
// #define regDCHUBBUB_ARB_ALLOW_SR_ENTER_WATERMARK_Z8_C                                                   0x0512
// #define regDCHUBBUB_ARB_ALLOW_SR_ENTER_WATERMARK_Z8_C_BASE_IDX                                          2
// #define regDCHUBBUB_ARB_ALLOW_SR_EXIT_WATERMARK_C                                                       0x0513
// #define regDCHUBBUB_ARB_ALLOW_SR_EXIT_WATERMARK_C_BASE_IDX                                              2
// #define regDCHUBBUB_ARB_ALLOW_SR_EXIT_WATERMARK_Z8_C                                                    0x0514
// #define regDCHUBBUB_ARB_ALLOW_SR_EXIT_WATERMARK_Z8_C_BASE_IDX                                           2
// #define regDCHUBBUB_ARB_ALLOW_DRAM_CLK_CHANGE_WATERMARK_C                                               0x0515
// #define regDCHUBBUB_ARB_ALLOW_DRAM_CLK_CHANGE_WATERMARK_C_BASE_IDX                                      2
// #define regDCHUBBUB_ARB_FRAC_URG_BW_NOM_C                                                               0x0516
// #define regDCHUBBUB_ARB_FRAC_URG_BW_NOM_C_BASE_IDX                                                      2
// #define regDCHUBBUB_ARB_FRAC_URG_BW_FLIP_C                                                              0x0517
// #define regDCHUBBUB_ARB_FRAC_URG_BW_FLIP_C_BASE_IDX                                                     2
// #define regDCHUBBUB_ARB_DATA_URGENCY_WATERMARK_D                                                        0x0518
// #define regDCHUBBUB_ARB_DATA_URGENCY_WATERMARK_D_BASE_IDX                                               2
// #define regDCHUBBUB_ARB_REFCYC_PER_TRIP_TO_MEMORY_D                                                     0x0519
// #define regDCHUBBUB_ARB_REFCYC_PER_TRIP_TO_MEMORY_D_BASE_IDX                                            2
// #define regDCHUBBUB_ARB_ALLOW_SR_ENTER_WATERMARK_D                                                      0x051a
// #define regDCHUBBUB_ARB_ALLOW_SR_ENTER_WATERMARK_D_BASE_IDX                                             2
// #define regDCHUBBUB_ARB_ALLOW_SR_ENTER_WATERMARK_Z8_D                                                   0x051b
// #define regDCHUBBUB_ARB_ALLOW_SR_ENTER_WATERMARK_Z8_D_BASE_IDX                                          2
// #define regDCHUBBUB_ARB_ALLOW_SR_EXIT_WATERMARK_D                                                       0x051c
// #define regDCHUBBUB_ARB_ALLOW_SR_EXIT_WATERMARK_D_BASE_IDX                                              2
// #define regDCHUBBUB_ARB_ALLOW_SR_EXIT_WATERMARK_Z8_D                                                    0x051d
// #define regDCHUBBUB_ARB_ALLOW_SR_EXIT_WATERMARK_Z8_D_BASE_IDX                                           2
// #define regDCHUBBUB_ARB_ALLOW_DRAM_CLK_CHANGE_WATERMARK_D                                               0x051e
// #define regDCHUBBUB_ARB_ALLOW_DRAM_CLK_CHANGE_WATERMARK_D_BASE_IDX                                      2
// #define regDCHUBBUB_ARB_FRAC_URG_BW_NOM_D                                                               0x051f
// #define regDCHUBBUB_ARB_FRAC_URG_BW_NOM_D_BASE_IDX                                                      2
// #define regDCHUBBUB_ARB_FRAC_URG_BW_FLIP_D                                                              0x0520
// #define regDCHUBBUB_ARB_FRAC_URG_BW_FLIP_D_BASE_IDX                                                     2
// #define regDCHUBBUB_ARB_HOSTVM_CNTL                                                                     0x0521
// #define regDCHUBBUB_ARB_HOSTVM_CNTL_BASE_IDX                                                            2
// #define regDCHUBBUB_ARB_WATERMARK_CHANGE_CNTL                                                           0x0522
// #define regDCHUBBUB_ARB_WATERMARK_CHANGE_CNTL_BASE_IDX                                                  2
// #define regDCHUBBUB_ARB_TIMEOUT_ENABLE                                                                  0x0523
// #define regDCHUBBUB_ARB_TIMEOUT_ENABLE_BASE_IDX                                                         2
// #define regDCHUBBUB_GLOBAL_TIMER_CNTL                                                                   0x0524
// #define regDCHUBBUB_GLOBAL_TIMER_CNTL_BASE_IDX                                                          2
// #define regSURFACE_CHECK0_ADDRESS_LSB                                                                   0x0525
// #define regSURFACE_CHECK0_ADDRESS_LSB_BASE_IDX                                                          2
// #define regSURFACE_CHECK0_ADDRESS_MSB                                                                   0x0526
// #define regSURFACE_CHECK0_ADDRESS_MSB_BASE_IDX                                                          2
// #define regSURFACE_CHECK1_ADDRESS_LSB                                                                   0x0527
// #define regSURFACE_CHECK1_ADDRESS_LSB_BASE_IDX                                                          2
// #define regSURFACE_CHECK1_ADDRESS_MSB                                                                   0x0528
// #define regSURFACE_CHECK1_ADDRESS_MSB_BASE_IDX                                                          2
// #define regSURFACE_CHECK2_ADDRESS_LSB                                                                   0x0529
// #define regSURFACE_CHECK2_ADDRESS_LSB_BASE_IDX                                                          2
// #define regSURFACE_CHECK2_ADDRESS_MSB                                                                   0x052a
// #define regSURFACE_CHECK2_ADDRESS_MSB_BASE_IDX                                                          2
// #define regSURFACE_CHECK3_ADDRESS_LSB                                                                   0x052b
// #define regSURFACE_CHECK3_ADDRESS_LSB_BASE_IDX                                                          2
// #define regSURFACE_CHECK3_ADDRESS_MSB                                                                   0x052c
// #define regSURFACE_CHECK3_ADDRESS_MSB_BASE_IDX                                                          2
// #define regVTG0_CONTROL                                                                                 0x052d
// #define regVTG0_CONTROL_BASE_IDX                                                                        2
// #define regVTG1_CONTROL                                                                                 0x052e
// #define regVTG1_CONTROL_BASE_IDX                                                                        2
// #define regVTG2_CONTROL                                                                                 0x052f
// #define regVTG2_CONTROL_BASE_IDX                                                                        2
// #define regVTG3_CONTROL                                                                                 0x0530
// #define regVTG3_CONTROL_BASE_IDX                                                                        2
// #define regDCHUBBUB_SOFT_RESET                                                                          0x0531
// #define regDCHUBBUB_SOFT_RESET_BASE_IDX                                                                 2
// #define regDCHUBBUB_CLOCK_CNTL                                                                          0x0532
// #define regDCHUBBUB_CLOCK_CNTL_BASE_IDX                                                                 2
// #define regDCFCLK_CNTL                                                                                  0x0533
// #define regDCFCLK_CNTL_BASE_IDX                                                                         2
// #define regDCHUBBUB_PERFORMANCE_MEASUREMENT_CNTL                                                        0x0534
// #define regDCHUBBUB_PERFORMANCE_MEASUREMENT_CNTL_BASE_IDX                                               2
// #define regDCHUBBUB_PERFORMANCE_MEASUREMENT_CNTL2                                                       0x0535
// #define regDCHUBBUB_PERFORMANCE_MEASUREMENT_CNTL2_BASE_IDX                                              2
// #define regDCHUBBUB_VLINE_SNAPSHOT                                                                      0x0536
// #define regDCHUBBUB_VLINE_SNAPSHOT_BASE_IDX                                                             2
// #define regDCHUBBUB_CTRL_STATUS                                                                         0x0537
// #define regDCHUBBUB_CTRL_STATUS_BASE_IDX                                                                2
// #define regDCHUBBUB_TIMEOUT_DETECTION_CTRL1                                                             0x053d
// #define regDCHUBBUB_TIMEOUT_DETECTION_CTRL1_BASE_IDX                                                    2
// #define regDCHUBBUB_TIMEOUT_DETECTION_CTRL2                                                             0x053e
// #define regDCHUBBUB_TIMEOUT_DETECTION_CTRL2_BASE_IDX                                                    2
// #define regFMON_CTRL                                                                                    0x0540
// #define regFMON_CTRL_BASE_IDX                                                                           2
// #define regDCHUBBUB_TEST_DEBUG_INDEX                                                                    0x0541
// #define regDCHUBBUB_TEST_DEBUG_INDEX_BASE_IDX                                                           2
// #define regDCHUBBUB_TEST_DEBUG_DATA                                                                     0x0542
// #define regDCHUBBUB_TEST_DEBUG_DATA_BASE_IDX                                                            2


// addressBlock: dce_dc_dchubbubl_hubbub_sdpif_dispdec
// base address: 0x0
// #define regDCHUBBUB_SDPIF_CFG0                                                                          0x046f
// #define regDCHUBBUB_SDPIF_CFG0_BASE_IDX                                                                 2
// #define regDCHUBBUB_SDPIF_CFG1                                                                          0x0470
// #define regDCHUBBUB_SDPIF_CFG1_BASE_IDX                                                                 2
// #define regDCHUBBUB_SDPIF_CFG2                                                                          0x0471
// #define regDCHUBBUB_SDPIF_CFG2_BASE_IDX                                                                 2
// #define regVM_REQUEST_PHYSICAL                                                                          0x0472
// #define regVM_REQUEST_PHYSICAL_BASE_IDX                                                                 2
// #define regDCHUBBUB_FORCE_IO_STATUS_0                                                                   0x0473
// #define regDCHUBBUB_FORCE_IO_STATUS_0_BASE_IDX                                                          2
// #define regDCHUBBUB_FORCE_IO_STATUS_1                                                                   0x0474
// #define regDCHUBBUB_FORCE_IO_STATUS_1_BASE_IDX                                                          2
// #define regDCN_VM_FB_LOCATION_BASE                                                                      0x0475
// #define regDCN_VM_FB_LOCATION_BASE_BASE_IDX                                                             2
// #define regDCN_VM_FB_LOCATION_TOP                                                                       0x0476
// #define regDCN_VM_FB_LOCATION_TOP_BASE_IDX                                                              2
// #define regDCN_VM_FB_OFFSET                                                                             0x0477
// #define regDCN_VM_FB_OFFSET_BASE_IDX                                                                    2
// #define regDCN_VM_AGP_BOT                                                                               0x0478
// #define regDCN_VM_AGP_BOT_BASE_IDX                                                                      2
// #define regDCN_VM_AGP_TOP                                                                               0x0479
// #define regDCN_VM_AGP_TOP_BASE_IDX                                                                      2
// #define regDCN_VM_AGP_BASE                                                                              0x047a
// #define regDCN_VM_AGP_BASE_BASE_IDX                                                                     2
// #define regDCN_VM_LOCAL_HBM_ADDRESS_START                                                               0x047b
// #define regDCN_VM_LOCAL_HBM_ADDRESS_START_BASE_IDX                                                      2
// #define regDCN_VM_LOCAL_HBM_ADDRESS_END                                                                 0x047c
// #define regDCN_VM_LOCAL_HBM_ADDRESS_END_BASE_IDX                                                        2
// #define regDCN_VM_LOCAL_HBM_ADDRESS_LOCK_CNTL                                                           0x047d
// #define regDCN_VM_LOCAL_HBM_ADDRESS_LOCK_CNTL_BASE_IDX                                                  2
// #define regDCHUBBUB_SDPIF_PIPE_SEC_LVL                                                                  0x047e
// #define regDCHUBBUB_SDPIF_PIPE_SEC_LVL_BASE_IDX                                                         2
// #define regDCHUBBUB_SDPIF_PIPE_DMDATA_SEC_LVL                                                           0x047f
// #define regDCHUBBUB_SDPIF_PIPE_DMDATA_SEC_LVL_BASE_IDX                                                  2
// #define regDCHUBBUB_SDPIF_MEM_PWR_CTRL                                                                  0x0483
// #define regDCHUBBUB_SDPIF_MEM_PWR_CTRL_BASE_IDX                                                         2
// #define regDCHUBBUB_SDPIF_MEM_PWR_STATUS                                                                0x0484
// #define regDCHUBBUB_SDPIF_MEM_PWR_STATUS_BASE_IDX                                                       2


// addressBlock: dce_dc_dchubbubl_hubbub_ret_path_dispdec
// base address: 0x0
// #define regDCHUBBUB_RET_PATH_DCC_CFG                                                                    0x04af
// #define regDCHUBBUB_RET_PATH_DCC_CFG_BASE_IDX                                                           2
// #define regDCHUBBUB_RET_PATH_DCC_CFG0_0                                                                 0x04b0
// #define regDCHUBBUB_RET_PATH_DCC_CFG0_0_BASE_IDX                                                        2
// #define regDCHUBBUB_RET_PATH_DCC_CFG0_1                                                                 0x04b1
// #define regDCHUBBUB_RET_PATH_DCC_CFG0_1_BASE_IDX                                                        2
// #define regDCHUBBUB_RET_PATH_DCC_CFG1_0                                                                 0x04b2
// #define regDCHUBBUB_RET_PATH_DCC_CFG1_0_BASE_IDX                                                        2
// #define regDCHUBBUB_RET_PATH_DCC_CFG1_1                                                                 0x04b3
// #define regDCHUBBUB_RET_PATH_DCC_CFG1_1_BASE_IDX                                                        2
// #define regDCHUBBUB_RET_PATH_DCC_CFG2_0                                                                 0x04b4
// #define regDCHUBBUB_RET_PATH_DCC_CFG2_0_BASE_IDX                                                        2
// #define regDCHUBBUB_RET_PATH_DCC_CFG2_1                                                                 0x04b5
// #define regDCHUBBUB_RET_PATH_DCC_CFG2_1_BASE_IDX                                                        2
// #define regDCHUBBUB_RET_PATH_DCC_CFG3_0                                                                 0x04b6
// #define regDCHUBBUB_RET_PATH_DCC_CFG3_0_BASE_IDX                                                        2
// #define regDCHUBBUB_RET_PATH_DCC_CFG3_1                                                                 0x04b7
// #define regDCHUBBUB_RET_PATH_DCC_CFG3_1_BASE_IDX                                                        2
// #define regDCHUBBUB_RET_PATH_DCC_CFG4_0                                                                 0x04b8
// #define regDCHUBBUB_RET_PATH_DCC_CFG4_0_BASE_IDX                                                        2
// #define regDCHUBBUB_RET_PATH_DCC_CFG4_1                                                                 0x04b9
// #define regDCHUBBUB_RET_PATH_DCC_CFG4_1_BASE_IDX                                                        2
// #define regDCHUBBUB_RET_PATH_DCC_CFG5_0                                                                 0x04ba
// #define regDCHUBBUB_RET_PATH_DCC_CFG5_0_BASE_IDX                                                        2
// #define regDCHUBBUB_RET_PATH_DCC_CFG5_1                                                                 0x04bb
// #define regDCHUBBUB_RET_PATH_DCC_CFG5_1_BASE_IDX                                                        2
// #define regDCHUBBUB_RET_PATH_DCC_CFG6_0                                                                 0x04bc
// #define regDCHUBBUB_RET_PATH_DCC_CFG6_0_BASE_IDX                                                        2
// #define regDCHUBBUB_RET_PATH_DCC_CFG6_1                                                                 0x04bd
// #define regDCHUBBUB_RET_PATH_DCC_CFG6_1_BASE_IDX                                                        2
// #define regDCHUBBUB_RET_PATH_DCC_CFG7_0                                                                 0x04be
// #define regDCHUBBUB_RET_PATH_DCC_CFG7_0_BASE_IDX                                                        2
// #define regDCHUBBUB_RET_PATH_DCC_CFG7_1                                                                 0x04bf
// #define regDCHUBBUB_RET_PATH_DCC_CFG7_1_BASE_IDX                                                        2
// #define regDCHUBBUB_RET_PATH_MEM_PWR_CTRL                                                               0x04c0
// #define regDCHUBBUB_RET_PATH_MEM_PWR_CTRL_BASE_IDX                                                      2
// #define regDCHUBBUB_RET_PATH_MEM_PWR_STATUS                                                             0x04c1
// #define regDCHUBBUB_RET_PATH_MEM_PWR_STATUS_BASE_IDX                                                    2
// #define regDCHUBBUB_CRC_CTRL                                                                            0x04c2
// #define regDCHUBBUB_CRC_CTRL_BASE_IDX                                                                   2
// #define regDCHUBBUB_CRC0_VAL_R_G                                                                        0x04c3
// #define regDCHUBBUB_CRC0_VAL_R_G_BASE_IDX                                                               2
// #define regDCHUBBUB_CRC0_VAL_B_A                                                                        0x04c4
// #define regDCHUBBUB_CRC0_VAL_B_A_BASE_IDX                                                               2
// #define regDCHUBBUB_CRC1_VAL_R_G                                                                        0x04c5
// #define regDCHUBBUB_CRC1_VAL_R_G_BASE_IDX                                                               2
// #define regDCHUBBUB_CRC1_VAL_B_A                                                                        0x04c6
// #define regDCHUBBUB_CRC1_VAL_B_A_BASE_IDX                                                               2
// #define regDCHUBBUB_DCC_STAT_CNTL                                                                       0x04c7
// #define regDCHUBBUB_DCC_STAT_CNTL_BASE_IDX                                                              2
// #define regDCHUBBUB_DCC_STAT0                                                                           0x04c8
// #define regDCHUBBUB_DCC_STAT0_BASE_IDX                                                                  2
// #define regDCHUBBUB_DCC_STAT1                                                                           0x04c9
// #define regDCHUBBUB_DCC_STAT1_BASE_IDX                                                                  2
// #define regDCHUBBUB_DCC_STAT2                                                                           0x04ca
// #define regDCHUBBUB_DCC_STAT2_BASE_IDX                                                                  2
// #define regDCHUBBUB_COMPBUF_CTRL                                                                        0x04cb
// #define regDCHUBBUB_COMPBUF_CTRL_BASE_IDX                                                               2
// #define regDCHUBBUB_DET0_CTRL                                                                           0x04cc
// #define regDCHUBBUB_DET0_CTRL_BASE_IDX                                                                  2
// #define regDCHUBBUB_DET1_CTRL                                                                           0x04cd
// #define regDCHUBBUB_DET1_CTRL_BASE_IDX                                                                  2
// #define regDCHUBBUB_DET2_CTRL                                                                           0x04ce
// #define regDCHUBBUB_DET2_CTRL_BASE_IDX                                                                  2
// #define regDCHUBBUB_DET3_CTRL                                                                           0x04cf
// #define regDCHUBBUB_DET3_CTRL_BASE_IDX                                                                  2
// #define regDCHUBBUB_MEM_PWR_MODE_CTRL                                                                   0x04d1
// #define regDCHUBBUB_MEM_PWR_MODE_CTRL_BASE_IDX                                                          2
// #define regCOMPBUF_MEM_PWR_CTRL_1                                                                       0x04d2
// #define regCOMPBUF_MEM_PWR_CTRL_1_BASE_IDX                                                              2
// #define regCOMPBUF_MEM_PWR_CTRL_2                                                                       0x04d3


// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
