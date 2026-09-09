// SPDX-License-Identifier: GPL-2.0
/* SPDX-License-Identifier: GPL-2.0 */


pub const RADEON_REGSIZE: u32 = 0x4000;


pub const MM_INDEX: u32 = 0x0000;
pub const MM_DATA: u32 = 0x0004;
pub const BUS_CNTL: u32 = 0x0030;
pub const HI_STAT: u32 = 0x004C;
pub const BUS_CNTL1: u32 = 0x0034;
pub const I2C_CNTL_1: u32 = 0x0094;
pub const CNFG_CNTL: u32 = 0x00E0;
pub const CNFG_MEMSIZE: u32 = 0x00F8;
pub const CNFG_APER_0_BASE: u32 = 0x0100;
pub const CNFG_APER_1_BASE: u32 = 0x0104;
pub const CNFG_APER_SIZE: u32 = 0x0108;
pub const CNFG_REG_1_BASE: u32 = 0x010C;
pub const CNFG_REG_APER_SIZE: u32 = 0x0110;
pub const PAD_AGPINPUT_DELAY: u32 = 0x0164;
pub const PAD_CTLR_STRENGTH: u32 = 0x0168;
pub const PAD_CTLR_UPDATE: u32 = 0x016C;
pub const PAD_CTLR_MISC: u32 = 0x0aa0;
pub const AGP_CNTL: u32 = 0x0174;
pub const BM_STATUS: u32 = 0x0160;
pub const CAP0_TRIG_CNTL: u32 = 0x0950;
pub const CAP1_TRIG_CNTL: u32 = 0x09c0;
pub const VIPH_CONTROL: u32 = 0x0C40;
pub const VENDOR_ID: u32 = 0x0F00;
pub const DEVICE_ID: u32 = 0x0F02;
pub const COMMAND: u32 = 0x0F04;
pub const STATUS: u32 = 0x0F06;
pub const REVISION_ID: u32 = 0x0F08;
pub const REGPROG_INF: u32 = 0x0F09;
pub const SUB_CLASS: u32 = 0x0F0A;
pub const BASE_CODE: u32 = 0x0F0B;
pub const CACHE_LINE: u32 = 0x0F0C;
pub const LATENCY: u32 = 0x0F0D;
pub const HEADER: u32 = 0x0F0E;
pub const BIST: u32 = 0x0F0F;
pub const REG_MEM_BASE: u32 = 0x0F10;
pub const REG_IO_BASE: u32 = 0x0F14;
pub const REG_REG_BASE: u32 = 0x0F18;
pub const ADAPTER_ID: u32 = 0x0F2C;
pub const BIOS_ROM: u32 = 0x0F30;
pub const CAPABILITIES_PTR: u32 = 0x0F34;
pub const INTERRUPT_LINE: u32 = 0x0F3C;
pub const INTERRUPT_PIN: u32 = 0x0F3D;
pub const MIN_GRANT: u32 = 0x0F3E;
pub const MAX_LATENCY: u32 = 0x0F3F;
pub const ADAPTER_ID_W: u32 = 0x0F4C;
pub const PMI_CAP_ID: u32 = 0x0F50;
pub const PMI_NXT_CAP_PTR: u32 = 0x0F51;
pub const PMI_PMC_REG: u32 = 0x0F52;
pub const PM_STATUS: u32 = 0x0F54;
pub const PMI_DATA: u32 = 0x0F57;
pub const AGP_CAP_ID: u32 = 0x0F58;
pub const AGP_STATUS: u32 = 0x0F5C;
pub const AGP_COMMAND: u32 = 0x0F60;
pub const AIC_CTRL: u32 = 0x01D0;
pub const AIC_STAT: u32 = 0x01D4;
pub const AIC_PT_BASE: u32 = 0x01D8;
pub const AIC_LO_ADDR: u32 = 0x01DC;
pub const AIC_HI_ADDR: u32 = 0x01E0;
pub const AIC_TLB_ADDR: u32 = 0x01E4;
pub const AIC_TLB_DATA: u32 = 0x01E8;
pub const DAC_CNTL: u32 = 0x0058;
pub const DAC_CNTL2: u32 = 0x007c;
pub const CRTC_GEN_CNTL: u32 = 0x0050;
pub const MEM_CNTL: u32 = 0x0140;
pub const MC_CNTL: u32 = 0x0140;
pub const EXT_MEM_CNTL: u32 = 0x0144;
pub const MC_TIMING_CNTL: u32 = 0x0144;
pub const MC_AGP_LOCATION: u32 = 0x014C;
pub const MEM_IO_CNTL_A0: u32 = 0x0178;
pub const MEM_REFRESH_CNTL: u32 = 0x0178;
pub const MEM_INIT_LATENCY_TIMER: u32 = 0x0154;
pub const MC_INIT_GFX_LAT_TIMER: u32 = 0x0154;
pub const MEM_SDRAM_MODE_REG: u32 = 0x0158;
pub const AGP_BASE: u32 = 0x0170;
pub const MEM_IO_CNTL_A1: u32 = 0x017C;
pub const MC_READ_CNTL_AB: u32 = 0x017C;
pub const MEM_IO_CNTL_B0: u32 = 0x0180;
pub const MC_INIT_MISC_LAT_TIMER: u32 = 0x0180;
pub const MEM_IO_CNTL_B1: u32 = 0x0184;
pub const MC_IOPAD_CNTL: u32 = 0x0184;
pub const MC_DEBUG: u32 = 0x0188;
pub const MC_STATUS: u32 = 0x0150;
pub const MEM_IO_OE_CNTL: u32 = 0x018C;
pub const MC_CHIP_IO_OE_CNTL_AB: u32 = 0x018C;
pub const MC_FB_LOCATION: u32 = 0x0148;
pub const HOST_PATH_CNTL: u32 = 0x0130;
pub const MEM_VGA_WP_SEL: u32 = 0x0038;
pub const MEM_VGA_RP_SEL: u32 = 0x003C;
pub const HDP_DEBUG: u32 = 0x0138;
pub const SW_SEMAPHORE: u32 = 0x013C;
pub const CRTC2_GEN_CNTL: u32 = 0x03f8;
pub const CRTC2_DISPLAY_BASE_ADDR: u32 = 0x033c;
pub const SURFACE_CNTL: u32 = 0x0B00;
pub const SURFACE0_LOWER_BOUND: u32 = 0x0B04;
pub const SURFACE1_LOWER_BOUND: u32 = 0x0B14;
pub const SURFACE2_LOWER_BOUND: u32 = 0x0B24;
pub const SURFACE3_LOWER_BOUND: u32 = 0x0B34;
pub const SURFACE4_LOWER_BOUND: u32 = 0x0B44;
pub const SURFACE5_LOWER_BOUND: u32 = 0x0B54;
pub const SURFACE6_LOWER_BOUND: u32 = 0x0B64;
pub const SURFACE7_LOWER_BOUND: u32 = 0x0B74;
pub const SURFACE0_UPPER_BOUND: u32 = 0x0B08;
pub const SURFACE1_UPPER_BOUND: u32 = 0x0B18;
pub const SURFACE2_UPPER_BOUND: u32 = 0x0B28;
pub const SURFACE3_UPPER_BOUND: u32 = 0x0B38;
pub const SURFACE4_UPPER_BOUND: u32 = 0x0B48;
pub const SURFACE5_UPPER_BOUND: u32 = 0x0B58;
pub const SURFACE6_UPPER_BOUND: u32 = 0x0B68;
pub const SURFACE7_UPPER_BOUND: u32 = 0x0B78;
pub const SURFACE0_INFO: u32 = 0x0B0C;
pub const SURFACE1_INFO: u32 = 0x0B1C;
pub const SURFACE2_INFO: u32 = 0x0B2C;
pub const SURFACE3_INFO: u32 = 0x0B3C;
pub const SURFACE4_INFO: u32 = 0x0B4C;
pub const SURFACE5_INFO: u32 = 0x0B5C;
pub const SURFACE6_INFO: u32 = 0x0B6C;
pub const SURFACE7_INFO: u32 = 0x0B7C;
pub const SURFACE_ACCESS_FLAGS: u32 = 0x0BF8;
pub const SURFACE_ACCESS_CLR: u32 = 0x0BFC;
pub const GEN_INT_CNTL: u32 = 0x0040;
pub const GEN_INT_STATUS: u32 = 0x0044;
pub const CRTC_EXT_CNTL: u32 = 0x0054;
pub const RB3D_CNTL: u32 = 0x1C3C;
pub const WAIT_UNTIL: u32 = 0x1720;
pub const ISYNC_CNTL: u32 = 0x1724;
pub const RBBM_GUICNTL: u32 = 0x172C;
pub const RBBM_STATUS: u32 = 0x0E40;
pub const RBBM_STATUS_alt_1: u32 = 0x1740;
pub const RBBM_CNTL: u32 = 0x00EC;
pub const RBBM_CNTL_alt_1: u32 = 0x0E44;
pub const RBBM_SOFT_RESET: u32 = 0x00F0;
pub const RBBM_SOFT_RESET_alt_1: u32 = 0x0E48;
pub const NQWAIT_UNTIL: u32 = 0x0E50;
pub const RBBM_DEBUG: u32 = 0x0E6C;
pub const RBBM_CMDFIFO_ADDR: u32 = 0x0E70;
pub const RBBM_CMDFIFO_DATAL: u32 = 0x0E74;
pub const RBBM_CMDFIFO_DATAH: u32 = 0x0E78;
pub const RBBM_CMDFIFO_STAT: u32 = 0x0E7C;
pub const CRTC_STATUS: u32 = 0x005C;
pub const GPIO_VGA_DDC: u32 = 0x0060;
pub const GPIO_DVI_DDC: u32 = 0x0064;
pub const GPIO_MONID: u32 = 0x0068;
pub const GPIO_CRT2_DDC: u32 = 0x006c;
pub const PALETTE_INDEX: u32 = 0x00B0;
pub const PALETTE_DATA: u32 = 0x00B4;
pub const PALETTE_30_DATA: u32 = 0x00B8;
pub const CRTC_H_TOTAL_DISP: u32 = 0x0200;
pub const CRTC_H_SYNC_STRT_WID: u32 = 0x0204;
pub const CRTC_V_TOTAL_DISP: u32 = 0x0208;
pub const CRTC_V_SYNC_STRT_WID: u32 = 0x020C;
pub const CRTC_VLINE_CRNT_VLINE: u32 = 0x0210;
pub const CRTC_CRNT_FRAME: u32 = 0x0214;
pub const CRTC_GUI_TRIG_VLINE: u32 = 0x0218;
pub const CRTC_DEBUG: u32 = 0x021C;
pub const CRTC_OFFSET_RIGHT: u32 = 0x0220;
pub const CRTC_OFFSET: u32 = 0x0224;
pub const CRTC_OFFSET_CNTL: u32 = 0x0228;
pub const CRTC_PITCH: u32 = 0x022C;
pub const OVR_CLR: u32 = 0x0230;
pub const OVR_WID_LEFT_RIGHT: u32 = 0x0234;
pub const OVR_WID_TOP_BOTTOM: u32 = 0x0238;
pub const DISPLAY_BASE_ADDR: u32 = 0x023C;
pub const SNAPSHOT_VH_COUNTS: u32 = 0x0240;
pub const SNAPSHOT_F_COUNT: u32 = 0x0244;
pub const N_VIF_COUNT: u32 = 0x0248;
pub const SNAPSHOT_VIF_COUNT: u32 = 0x024C;
pub const FP_CRTC_H_TOTAL_DISP: u32 = 0x0250;
pub const FP_CRTC_V_TOTAL_DISP: u32 = 0x0254;
pub const CRT_CRTC_H_SYNC_STRT_WID: u32 = 0x0258;
pub const CRT_CRTC_V_SYNC_STRT_WID: u32 = 0x025C;
pub const CUR_OFFSET: u32 = 0x0260;
pub const CUR_HORZ_VERT_POSN: u32 = 0x0264;
pub const CUR_HORZ_VERT_OFF: u32 = 0x0268;
pub const CUR_CLR0: u32 = 0x026C;
pub const CUR_CLR1: u32 = 0x0270;
pub const FP_HORZ_VERT_ACTIVE: u32 = 0x0278;
pub const CRTC_MORE_CNTL: u32 = 0x027C;
pub const CRTC_H_CUTOFF_ACTIVE_EN: u32 = (1<<4);
pub const CRTC_V_CUTOFF_ACTIVE_EN: u32 = (1<<5);
pub const DAC_EXT_CNTL: u32 = 0x0280;
pub const FP_GEN_CNTL: u32 = 0x0284;
pub const FP_HORZ_STRETCH: u32 = 0x028C;
pub const FP_VERT_STRETCH: u32 = 0x0290;
pub const FP_H_SYNC_STRT_WID: u32 = 0x02C4;
pub const FP_V_SYNC_STRT_WID: u32 = 0x02C8;
pub const AUX_WINDOW_HORZ_CNTL: u32 = 0x02D8;
pub const AUX_WINDOW_VERT_CNTL: u32 = 0x02DC;
//#define DDA_CONFIG			       0x02e0
//#define DDA_ON_OFF			       0x02e4
pub const DVI_I2C_CNTL_1: u32 = 0x02e4;
pub const GRPH_BUFFER_CNTL: u32 = 0x02F0;
pub const GRPH2_BUFFER_CNTL: u32 = 0x03F0;
pub const VGA_BUFFER_CNTL: u32 = 0x02F4;
pub const OV0_Y_X_START: u32 = 0x0400;
pub const OV0_Y_X_END: u32 = 0x0404;
pub const OV0_PIPELINE_CNTL: u32 = 0x0408;
pub const OV0_REG_LOAD_CNTL: u32 = 0x0410;
pub const OV0_SCALE_CNTL: u32 = 0x0420;
pub const OV0_V_INC: u32 = 0x0424;
pub const OV0_P1_V_ACCUM_INIT: u32 = 0x0428;
pub const OV0_P23_V_ACCUM_INIT: u32 = 0x042C;
pub const OV0_P1_BLANK_LINES_AT_TOP: u32 = 0x0430;
pub const OV0_P23_BLANK_LINES_AT_TOP: u32 = 0x0434;
pub const OV0_BASE_ADDR: u32 = 0x043C;
pub const OV0_VID_BUF0_BASE_ADRS: u32 = 0x0440;
pub const OV0_VID_BUF1_BASE_ADRS: u32 = 0x0444;
pub const OV0_VID_BUF2_BASE_ADRS: u32 = 0x0448;
pub const OV0_VID_BUF3_BASE_ADRS: u32 = 0x044C;
pub const OV0_VID_BUF4_BASE_ADRS: u32 = 0x0450;
pub const OV0_VID_BUF5_BASE_ADRS: u32 = 0x0454;
pub const OV0_VID_BUF_PITCH0_VALUE: u32 = 0x0460;
pub const OV0_VID_BUF_PITCH1_VALUE: u32 = 0x0464;
pub const OV0_AUTO_FLIP_CNTRL: u32 = 0x0470;
pub const OV0_DEINTERLACE_PATTERN: u32 = 0x0474;
pub const OV0_SUBMIT_HISTORY: u32 = 0x0478;
pub const OV0_H_INC: u32 = 0x0480;
pub const OV0_STEP_BY: u32 = 0x0484;
pub const OV0_P1_H_ACCUM_INIT: u32 = 0x0488;
pub const OV0_P23_H_ACCUM_INIT: u32 = 0x048C;
pub const OV0_P1_X_START_END: u32 = 0x0494;
pub const OV0_P2_X_START_END: u32 = 0x0498;
pub const OV0_P3_X_START_END: u32 = 0x049C;
pub const OV0_FILTER_CNTL: u32 = 0x04A0;
pub const OV0_FOUR_TAP_COEF_0: u32 = 0x04B0;
pub const OV0_FOUR_TAP_COEF_1: u32 = 0x04B4;
pub const OV0_FOUR_TAP_COEF_2: u32 = 0x04B8;
pub const OV0_FOUR_TAP_COEF_3: u32 = 0x04BC;
pub const OV0_FOUR_TAP_COEF_4: u32 = 0x04C0;
pub const OV0_FLAG_CNTRL: u32 = 0x04DC;
pub const OV0_SLICE_CNTL: u32 = 0x04E0;
pub const OV0_VID_KEY_CLR_LOW: u32 = 0x04E4;
pub const OV0_VID_KEY_CLR_HIGH: u32 = 0x04E8;
pub const OV0_GRPH_KEY_CLR_LOW: u32 = 0x04EC;
pub const OV0_GRPH_KEY_CLR_HIGH: u32 = 0x04F0;
pub const OV0_KEY_CNTL: u32 = 0x04F4;
pub const OV0_TEST: u32 = 0x04F8;
pub const SUBPIC_CNTL: u32 = 0x0540;
pub const SUBPIC_DEFCOLCON: u32 = 0x0544;
pub const SUBPIC_Y_X_START: u32 = 0x054C;
pub const SUBPIC_Y_X_END: u32 = 0x0550;
pub const SUBPIC_V_INC: u32 = 0x0554;
pub const SUBPIC_H_INC: u32 = 0x0558;
pub const SUBPIC_BUF0_OFFSET: u32 = 0x055C;
pub const SUBPIC_BUF1_OFFSET: u32 = 0x0560;
pub const SUBPIC_LC0_OFFSET: u32 = 0x0564;
pub const SUBPIC_LC1_OFFSET: u32 = 0x0568;
pub const SUBPIC_PITCH: u32 = 0x056C;
pub const SUBPIC_BTN_HLI_COLCON: u32 = 0x0570;
pub const SUBPIC_BTN_HLI_Y_X_START: u32 = 0x0574;
pub const SUBPIC_BTN_HLI_Y_X_END: u32 = 0x0578;
pub const SUBPIC_PALETTE_INDEX: u32 = 0x057C;
pub const SUBPIC_PALETTE_DATA: u32 = 0x0580;
pub const SUBPIC_H_ACCUM_INIT: u32 = 0x0584;
pub const SUBPIC_V_ACCUM_INIT: u32 = 0x0588;
pub const DISP_MISC_CNTL: u32 = 0x0D00;
pub const DAC_MACRO_CNTL: u32 = 0x0D04;
pub const DISP_PWR_MAN: u32 = 0x0D08;
pub const DISP_TEST_DEBUG_CNTL: u32 = 0x0D10;
pub const DISP_HW_DEBUG: u32 = 0x0D14;
pub const DAC_CRC_SIG1: u32 = 0x0D18;
pub const DAC_CRC_SIG2: u32 = 0x0D1C;
pub const OV0_LIN_TRANS_A: u32 = 0x0D20;
pub const OV0_LIN_TRANS_B: u32 = 0x0D24;
pub const OV0_LIN_TRANS_C: u32 = 0x0D28;
pub const OV0_LIN_TRANS_D: u32 = 0x0D2C;
pub const OV0_LIN_TRANS_E: u32 = 0x0D30;
pub const OV0_LIN_TRANS_F: u32 = 0x0D34;
pub const OV0_GAMMA_0_F: u32 = 0x0D40;
pub const OV0_GAMMA_10_1F: u32 = 0x0D44;
pub const OV0_GAMMA_20_3F: u32 = 0x0D48;
pub const OV0_GAMMA_40_7F: u32 = 0x0D4C;
pub const OV0_GAMMA_380_3BF: u32 = 0x0D50;
pub const OV0_GAMMA_3C0_3FF: u32 = 0x0D54;
pub const DISP_MERGE_CNTL: u32 = 0x0D60;
pub const DISP_OUTPUT_CNTL: u32 = 0x0D64;
pub const DISP_LIN_TRANS_GRPH_A: u32 = 0x0D80;
pub const DISP_LIN_TRANS_GRPH_B: u32 = 0x0D84;
pub const DISP_LIN_TRANS_GRPH_C: u32 = 0x0D88;
pub const DISP_LIN_TRANS_GRPH_D: u32 = 0x0D8C;
pub const DISP_LIN_TRANS_GRPH_E: u32 = 0x0D90;
pub const DISP_LIN_TRANS_GRPH_F: u32 = 0x0D94;
pub const DISP_LIN_TRANS_VID_A: u32 = 0x0D98;
pub const DISP_LIN_TRANS_VID_B: u32 = 0x0D9C;
pub const DISP_LIN_TRANS_VID_C: u32 = 0x0DA0;
pub const DISP_LIN_TRANS_VID_D: u32 = 0x0DA4;
pub const DISP_LIN_TRANS_VID_E: u32 = 0x0DA8;
pub const DISP_LIN_TRANS_VID_F: u32 = 0x0DAC;
pub const RMX_HORZ_FILTER_0TAP_COEF: u32 = 0x0DB0;
pub const RMX_HORZ_FILTER_1TAP_COEF: u32 = 0x0DB4;
pub const RMX_HORZ_FILTER_2TAP_COEF: u32 = 0x0DB8;
pub const RMX_HORZ_PHASE: u32 = 0x0DBC;
pub const DAC_EMBEDDED_SYNC_CNTL: u32 = 0x0DC0;
pub const DAC_BROAD_PULSE: u32 = 0x0DC4;
pub const DAC_SKEW_CLKS: u32 = 0x0DC8;
pub const DAC_INCR: u32 = 0x0DCC;
pub const DAC_NEG_SYNC_LEVEL: u32 = 0x0DD0;
pub const DAC_POS_SYNC_LEVEL: u32 = 0x0DD4;
pub const DAC_BLANK_LEVEL: u32 = 0x0DD8;
pub const CLOCK_CNTL_INDEX: u32 = 0x0008;
pub const CLOCK_CNTL_DATA: u32 = 0x000C;
pub const CP_RB_CNTL: u32 = 0x0704;
pub const CP_RB_BASE: u32 = 0x0700;
pub const CP_RB_RPTR_ADDR: u32 = 0x070C;
pub const CP_RB_RPTR: u32 = 0x0710;
pub const CP_RB_WPTR: u32 = 0x0714;
pub const CP_RB_WPTR_DELAY: u32 = 0x0718;
pub const CP_IB_BASE: u32 = 0x0738;
pub const CP_IB_BUFSZ: u32 = 0x073C;
pub const SCRATCH_REG0: u32 = 0x15E0;
pub const GUI_SCRATCH_REG0: u32 = 0x15E0;
pub const SCRATCH_REG1: u32 = 0x15E4;
pub const GUI_SCRATCH_REG1: u32 = 0x15E4;
pub const SCRATCH_REG2: u32 = 0x15E8;
pub const GUI_SCRATCH_REG2: u32 = 0x15E8;
pub const SCRATCH_REG3: u32 = 0x15EC;
pub const GUI_SCRATCH_REG3: u32 = 0x15EC;
pub const SCRATCH_REG4: u32 = 0x15F0;
pub const GUI_SCRATCH_REG4: u32 = 0x15F0;
pub const SCRATCH_REG5: u32 = 0x15F4;
pub const GUI_SCRATCH_REG5: u32 = 0x15F4;
pub const SCRATCH_UMSK: u32 = 0x0770;
pub const SCRATCH_ADDR: u32 = 0x0774;
pub const DP_BRUSH_FRGD_CLR: u32 = 0x147C;
pub const DP_BRUSH_BKGD_CLR: u32 = 0x1478;
pub const DST_LINE_START: u32 = 0x1600;
pub const DST_LINE_END: u32 = 0x1604;
pub const SRC_OFFSET: u32 = 0x15AC;
pub const SRC_PITCH: u32 = 0x15B0;
pub const SRC_TILE: u32 = 0x1704;
pub const SRC_PITCH_OFFSET: u32 = 0x1428;
pub const SRC_X: u32 = 0x1414;
pub const SRC_Y: u32 = 0x1418;
pub const SRC_X_Y: u32 = 0x1590;
pub const SRC_Y_X: u32 = 0x1434;
pub const DST_Y_X: u32 = 0x1438;
pub const DST_WIDTH_HEIGHT: u32 = 0x1598;
pub const DST_HEIGHT_WIDTH: u32 = 0x143c;
pub const DST_OFFSET: u32 = 0x1404;
pub const SRC_CLUT_ADDRESS: u32 = 0x1780;
pub const SRC_CLUT_DATA: u32 = 0x1784;
pub const SRC_CLUT_DATA_RD: u32 = 0x1788;
pub const HOST_DATA0: u32 = 0x17C0;
pub const HOST_DATA1: u32 = 0x17C4;
pub const HOST_DATA2: u32 = 0x17C8;
pub const HOST_DATA3: u32 = 0x17CC;
pub const HOST_DATA4: u32 = 0x17D0;
pub const HOST_DATA5: u32 = 0x17D4;
pub const HOST_DATA6: u32 = 0x17D8;
pub const HOST_DATA7: u32 = 0x17DC;
pub const HOST_DATA_LAST: u32 = 0x17E0;
pub const DP_SRC_ENDIAN: u32 = 0x15D4;
pub const DP_SRC_FRGD_CLR: u32 = 0x15D8;
pub const DP_SRC_BKGD_CLR: u32 = 0x15DC;
pub const SC_LEFT: u32 = 0x1640;
pub const SC_RIGHT: u32 = 0x1644;
pub const SC_TOP: u32 = 0x1648;
pub const SC_BOTTOM: u32 = 0x164C;
pub const SRC_SC_RIGHT: u32 = 0x1654;
pub const SRC_SC_BOTTOM: u32 = 0x165C;
pub const DP_CNTL: u32 = 0x16C0;
pub const DP_CNTL_XDIR_YDIR_YMAJOR: u32 = 0x16D0;
pub const DP_DATATYPE: u32 = 0x16C4;
pub const DP_MIX: u32 = 0x16C8;
pub const DP_WRITE_MSK: u32 = 0x16CC;
pub const DP_XOP: u32 = 0x17F8;
pub const CLR_CMP_CLR_SRC: u32 = 0x15C4;
pub const CLR_CMP_CLR_DST: u32 = 0x15C8;
pub const CLR_CMP_CNTL: u32 = 0x15C0;
pub const CLR_CMP_MSK: u32 = 0x15CC;
pub const DSTCACHE_MODE: u32 = 0x1710;
pub const DSTCACHE_CTLSTAT: u32 = 0x1714;
pub const DEFAULT_PITCH_OFFSET: u32 = 0x16E0;
pub const DEFAULT_SC_BOTTOM_RIGHT: u32 = 0x16E8;
pub const DEFAULT_SC_TOP_LEFT: u32 = 0x16EC;
pub const SRC_PITCH_OFFSET: u32 = 0x1428;
pub const DST_PITCH_OFFSET: u32 = 0x142C;
pub const DP_GUI_MASTER_CNTL: u32 = 0x146C;
pub const SC_TOP_LEFT: u32 = 0x16EC;
pub const SC_BOTTOM_RIGHT: u32 = 0x16F0;
pub const SRC_SC_BOTTOM_RIGHT: u32 = 0x16F4;
pub const RB2D_DSTCACHE_MODE: u32 = 0x3428;
pub const RB2D_DSTCACHE_CTLSTAT_broken: u32 = 0x342C /* do not use */;
pub const LVDS_GEN_CNTL: u32 = 0x02d0;
pub const LVDS_PLL_CNTL: u32 = 0x02d4;
pub const FP2_GEN_CNTL: u32 = 0x0288;
pub const TMDS_CNTL: u32 = 0x0294;
pub const TMDS_CRC: u32 = 0x02a0;
pub const TMDS_TRANSMITTER_CNTL: u32 = 0x02a4;
pub const MPP_TB_CONFIG: u32 = 0x01c0;
pub const PAMAC0_DLY_CNTL: u32 = 0x0a94;
pub const PAMAC1_DLY_CNTL: u32 = 0x0a98;
pub const PAMAC2_DLY_CNTL: u32 = 0x0a9c;
pub const FW_CNTL: u32 = 0x0118;
pub const FCP_CNTL: u32 = 0x0910;
pub const VGA_DDA_ON_OFF: u32 = 0x02ec;
pub const TV_MASTER_CNTL: u32 = 0x0800;

//#define BASE_CODE			       0x0f0b
pub const BIOS_0_SCRATCH: u32 = 0x0010;
pub const BIOS_1_SCRATCH: u32 = 0x0014;
pub const BIOS_2_SCRATCH: u32 = 0x0018;
pub const BIOS_3_SCRATCH: u32 = 0x001c;
pub const BIOS_4_SCRATCH: u32 = 0x0020;
pub const BIOS_5_SCRATCH: u32 = 0x0024;
pub const BIOS_6_SCRATCH: u32 = 0x0028;
pub const BIOS_7_SCRATCH: u32 = 0x002c;

pub const HDP_SOFT_RESET: u32 = (1 << 26);

pub const TV_DAC_CNTL: u32 = 0x088c;
pub const GPIOPAD_MASK: u32 = 0x0198;
pub const GPIOPAD_A: u32 = 0x019c;
pub const GPIOPAD_EN: u32 = 0x01a0;
pub const GPIOPAD_Y: u32 = 0x01a4;
pub const ZV_LCDPAD_MASK: u32 = 0x01a8;
pub const ZV_LCDPAD_A: u32 = 0x01ac;
pub const ZV_LCDPAD_EN: u32 = 0x01b0;
pub const ZV_LCDPAD_Y: u32 = 0x01b4;

/* PLL Registers */
pub const CLK_PIN_CNTL: u32 = 0x0001;
pub const PPLL_CNTL: u32 = 0x0002;
pub const PPLL_REF_DIV: u32 = 0x0003;
pub const PPLL_DIV_0: u32 = 0x0004;
pub const PPLL_DIV_1: u32 = 0x0005;
pub const PPLL_DIV_2: u32 = 0x0006;
pub const PPLL_DIV_3: u32 = 0x0007;
pub const VCLK_ECP_CNTL: u32 = 0x0008;
pub const HTOTAL_CNTL: u32 = 0x0009;
pub const M_SPLL_REF_FB_DIV: u32 = 0x000a;
pub const AGP_PLL_CNTL: u32 = 0x000b;
pub const SPLL_CNTL: u32 = 0x000c;
pub const SCLK_CNTL: u32 = 0x000d;
pub const MPLL_CNTL: u32 = 0x000e;
pub const MDLL_CKO: u32 = 0x000f;
pub const MDLL_RDCKA: u32 = 0x0010;
pub const MCLK_CNTL: u32 = 0x0012;
pub const AGP_PLL_CNTL: u32 = 0x000b;
pub const PLL_TEST_CNTL: u32 = 0x0013;
pub const CLK_PWRMGT_CNTL: u32 = 0x0014;
pub const PLL_PWRMGT_CNTL: u32 = 0x0015;
pub const MCLK_MISC: u32 = 0x001f;
pub const P2PLL_CNTL: u32 = 0x002a;
pub const P2PLL_REF_DIV: u32 = 0x002b;
pub const PIXCLKS_CNTL: u32 = 0x002d;
pub const SCLK_MORE_CNTL: u32 = 0x0035;

/* MCLK_CNTL bit constants */
pub const FORCEON_MCLKA: u32 = (1 << 16);
pub const FORCEON_MCLKB: u32 = (1 << 17);
pub const FORCEON_YCLKA: u32 = (1 << 18);
pub const FORCEON_YCLKB: u32 = (1 << 19);
pub const FORCEON_MC: u32 = (1 << 20);
pub const FORCEON_AIC: u32 = (1 << 21);

/* SCLK_CNTL bit constants */
pub const DYN_STOP_LAT_MASK: u32 = 0x00007ff8;
pub const CP_MAX_DYN_STOP_LAT: u32 = 0x0008;
pub const SCLK_FORCEON_MASK: u32 = 0xffff8000;

/* SCLK_MORE_CNTL bit constants */
pub const SCLK_MORE_FORCEON: u32 = 0x0700;

/* BUS_CNTL bit constants */
pub const BUS_DBL_RESYNC: u32 = 0x00000001;
pub const BUS_MSTR_RESET: u32 = 0x00000002;
pub const BUS_FLUSH_BUF: u32 = 0x00000004;
pub const BUS_STOP_REQ_DIS: u32 = 0x00000008;
pub const BUS_ROTATION_DIS: u32 = 0x00000010;
pub const BUS_MASTER_DIS: u32 = 0x00000040;
pub const BUS_ROM_WRT_EN: u32 = 0x00000080;
pub const BUS_DIS_ROM: u32 = 0x00001000;
pub const BUS_PCI_READ_RETRY_EN: u32 = 0x00002000;
pub const BUS_AGP_AD_STEPPING_EN: u32 = 0x00004000;
pub const BUS_PCI_WRT_RETRY_EN: u32 = 0x00008000;
pub const BUS_MSTR_RD_MULT: u32 = 0x00100000;
pub const BUS_MSTR_RD_LINE: u32 = 0x00200000;
pub const BUS_SUSPEND: u32 = 0x00400000;
pub const LAT_16X: u32 = 0x00800000;
pub const BUS_RD_DISCARD_EN: u32 = 0x01000000;
pub const BUS_RD_ABORT_EN: u32 = 0x02000000;
pub const BUS_MSTR_WS: u32 = 0x04000000;
pub const BUS_PARKING_DIS: u32 = 0x08000000;
pub const BUS_MSTR_DISCONNECT_EN: u32 = 0x10000000;
pub const BUS_WRT_BURST: u32 = 0x20000000;
pub const BUS_READ_BURST: u32 = 0x40000000;
pub const BUS_RDY_READ_DLY: u32 = 0x80000000;

/* PIXCLKS_CNTL */
pub const PIX2CLK_SRC_SEL_MASK: u32 = 0x03;
pub const PIX2CLK_SRC_SEL_CPUCLK: u32 = 0x00;
pub const PIX2CLK_SRC_SEL_PSCANCLK: u32 = 0x01;
pub const PIX2CLK_SRC_SEL_BYTECLK: u32 = 0x02;
pub const PIX2CLK_SRC_SEL_P2PLLCLK: u32 = 0x03;
pub const PIX2CLK_ALWAYS_ONb: u32 = (1<<6);
pub const PIX2CLK_DAC_ALWAYS_ONb: u32 = (1<<7);
pub const PIXCLK_TV_SRC_SEL: u32 = (1 << 8);
pub const PIXCLK_LVDS_ALWAYS_ONb: u32 = (1 << 14);
pub const PIXCLK_TMDS_ALWAYS_ONb: u32 = (1 << 15);


/* CLOCK_CNTL_INDEX bit constants */
pub const PLL_WR_EN: u32 = 0x00000080;

/* CNFG_CNTL bit constants */
pub const CFG_VGA_RAM_EN: u32 = 0x00000100;
pub const CFG_ATI_REV_ID_MASK: u32 = (0xf << 16);
pub const CFG_ATI_REV_A11: u32 = (0 << 16);
pub const CFG_ATI_REV_A12: u32 = (1 << 16);
pub const CFG_ATI_REV_A13: u32 = (2 << 16);

/* CRTC_EXT_CNTL bit constants */
pub const VGA_ATI_LINEAR: u32 = 0x00000008;
pub const VGA_128KAP_PAGING: u32 = 0x00000010;
pub const XCRT_CNT_EN: u32 = (1 << 6);
pub const CRTC_HSYNC_DIS: u32 = (1 << 8);
pub const CRTC_VSYNC_DIS: u32 = (1 << 9);
pub const CRTC_DISPLAY_DIS: u32 = (1 << 10);
pub const CRTC_CRT_ON: u32 = (1 << 15);


/* DSTCACHE_CTLSTAT bit constants */
pub const RB2D_DC_FLUSH_2D: u32 = (1 << 0);
pub const RB2D_DC_FREE_2D: u32 = (1 << 2);
pub const RB2D_DC_FLUSH_ALL: u32 = (RB2D_DC_FUSH_2D | RB2D_DC_FREE_2D);
pub const RB2D_DC_BUSY: u32 = (1 << 31);

/* DSTCACHE_MODE bits constants */
pub const RB2D_DC_AUTOFLUSH_ENABLE: u32 = (1 << 8);
pub const RB2D_DC_DC_DISABLE_IGNORE_PE: u32 = (1 << 17);

/* CRTC_GEN_CNTL bit constants */
pub const CRTC_DBL_SCAN_EN: u32 = 0x00000001;
pub const CRTC_CUR_EN: u32 = 0x00010000;
pub const CRTC_INTERLACE_EN: u32 = (1 << 1);
pub const CRTC_BYPASS_LUT_EN: u32 = (1 << 14);
pub const CRTC_EXT_DISP_EN: u32 = (1 << 24);
pub const CRTC_EN: u32 = (1 << 25);
pub const CRTC_DISP_REQ_EN_B: u32 = (1 << 26);

/* CRTC_STATUS bit constants */
pub const CRTC_VBLANK: u32 = 0x00000001;

/* CRTC2_GEN_CNTL bit constants */
pub const CRT2_ON: u32 = (1 << 7);
pub const CRTC2_DISPLAY_DIS: u32 = (1 << 23);
pub const CRTC2_EN: u32 = (1 << 25);
pub const CRTC2_DISP_REQ_EN_B: u32 = (1 << 26);

/* CUR_OFFSET, CUR_HORZ_VERT_POSN, CUR_HORZ_VERT_OFF bit constants */
pub const CUR_LOCK: u32 = 0x80000000;

/* GPIO bit constants */
pub const GPIO_A_0: u32 = (1 <<  0);
pub const GPIO_A_1: u32 = (1 <<  1);
pub const GPIO_Y_0: u32 = (1 <<  8);
pub const GPIO_Y_1: u32 = (1 <<  9);
pub const GPIO_EN_0: u32 = (1 << 16);
pub const GPIO_EN_1: u32 = (1 << 17);
pub const GPIO_MASK_0: u32 = (1 << 24);
pub const GPIO_MASK_1: u32 = (1 << 25);
pub const VGA_DDC_DATA_OUTPUT: u32 = GPIO_A_0;
pub const VGA_DDC_CLK_OUTPUT: u32 = GPIO_A_1;
pub const VGA_DDC_DATA_INPUT: u32 = GPIO_Y_0;
pub const VGA_DDC_CLK_INPUT: u32 = GPIO_Y_1;
pub const VGA_DDC_DATA_OUT_EN: u32 = GPIO_EN_0;
pub const VGA_DDC_CLK_OUT_EN: u32 = GPIO_EN_1;


/* FP bit constants */
pub const FP_CRTC_H_TOTAL_MASK: u32 = 0x000003ff;
pub const FP_CRTC_H_DISP_MASK: u32 = 0x01ff0000;
pub const FP_CRTC_V_TOTAL_MASK: u32 = 0x00000fff;
pub const FP_CRTC_V_DISP_MASK: u32 = 0x0fff0000;
pub const FP_H_SYNC_STRT_CHAR_MASK: u32 = 0x00001ff8;
pub const FP_H_SYNC_WID_MASK: u32 = 0x003f0000;
pub const FP_V_SYNC_STRT_MASK: u32 = 0x00000fff;
pub const FP_V_SYNC_WID_MASK: u32 = 0x001f0000;
pub const FP_CRTC_H_TOTAL_SHIFT: u32 = 0x00000000;
pub const FP_CRTC_H_DISP_SHIFT: u32 = 0x00000010;
pub const FP_CRTC_V_TOTAL_SHIFT: u32 = 0x00000000;
pub const FP_CRTC_V_DISP_SHIFT: u32 = 0x00000010;
pub const FP_H_SYNC_STRT_CHAR_SHIFT: u32 = 0x00000003;
pub const FP_H_SYNC_WID_SHIFT: u32 = 0x00000010;
pub const FP_V_SYNC_STRT_SHIFT: u32 = 0x00000000;
pub const FP_V_SYNC_WID_SHIFT: u32 = 0x00000010;

/* FP_GEN_CNTL bit constants */
pub const FP_FPON: u32 = (1 << 0);
pub const FP_TMDS_EN: u32 = (1 << 2);
pub const FP_PANEL_FORMAT: u32 = (1 << 3);
pub const FP_EN_TMDS: u32 = (1 << 7);
pub const FP_DETECT_SENSE: u32 = (1 << 8);
pub const R200_FP_SOURCE_SEL_MASK: u32 = (3 << 10);
pub const R200_FP_SOURCE_SEL_CRTC1: u32 = (0 << 10);
pub const R200_FP_SOURCE_SEL_CRTC2: u32 = (1 << 10);
pub const R200_FP_SOURCE_SEL_RMX: u32 = (2 << 10);
pub const R200_FP_SOURCE_SEL_TRANS: u32 = (3 << 10);
pub const FP_SEL_CRTC1: u32 = (0 << 13);
pub const FP_SEL_CRTC2: u32 = (1 << 13);
pub const FP_USE_VGA_HSYNC: u32 = (1 << 14);
pub const FP_CRTC_DONT_SHADOW_HPAR: u32 = (1 << 15);
pub const FP_CRTC_DONT_SHADOW_VPAR: u32 = (1 << 16);
pub const FP_CRTC_DONT_SHADOW_HEND: u32 = (1 << 17);
pub const FP_CRTC_USE_SHADOW_VEND: u32 = (1 << 18);
pub const FP_RMX_HVSYNC_CONTROL_EN: u32 = (1 << 20);
pub const FP_DFP_SYNC_SEL: u32 = (1 << 21);
pub const FP_CRTC_LOCK_8DOT: u32 = (1 << 22);
pub const FP_CRT_SYNC_SEL: u32 = (1 << 23);
pub const FP_USE_SHADOW_EN: u32 = (1 << 24);
pub const FP_CRT_SYNC_ALT: u32 = (1 << 26);

/* FP2_GEN_CNTL bit constants */
pub const FP2_BLANK_EN: u32 = (1 <<  1);
pub const FP2_ON: u32 = (1 <<  2);
pub const FP2_PANEL_FORMAT: u32 = (1 <<  3);
pub const FP2_SOURCE_SEL_MASK: u32 = (3 << 10);
pub const FP2_SOURCE_SEL_CRTC2: u32 = (1 << 10);
pub const FP2_SRC_SEL_MASK: u32 = (3 << 13);
pub const FP2_SRC_SEL_CRTC2: u32 = (1 << 13);
pub const FP2_FP_POL: u32 = (1 << 16);
pub const FP2_LP_POL: u32 = (1 << 17);
pub const FP2_SCK_POL: u32 = (1 << 18);
pub const FP2_LCD_CNTL_MASK: u32 = (7 << 19);
pub const FP2_PAD_FLOP_EN: u32 = (1 << 22);
pub const FP2_CRC_EN: u32 = (1 << 23);
pub const FP2_CRC_READ_EN: u32 = (1 << 24);
pub const FP2_DV0_EN: u32 = (1 << 25);
pub const FP2_DV0_RATE_SEL_SDR: u32 = (1 << 26);


/* LVDS_GEN_CNTL bit constants */
pub const LVDS_ON: u32 = (1 << 0);
pub const LVDS_DISPLAY_DIS: u32 = (1 << 1);
pub const LVDS_PANEL_TYPE: u32 = (1 << 2);
pub const LVDS_PANEL_FORMAT: u32 = (1 << 3);
pub const LVDS_EN: u32 = (1 << 7);
pub const LVDS_BL_MOD_LEVEL_MASK: u32 = 0x0000ff00;
pub const LVDS_BL_MOD_LEVEL_SHIFT: u32 = 8;
pub const LVDS_BL_MOD_EN: u32 = (1 << 16);
pub const LVDS_DIGON: u32 = (1 << 18);
pub const LVDS_BLON: u32 = (1 << 19);
pub const LVDS_SEL_CRTC2: u32 = (1 << 23);
pub const LVDS_STATE_MASK: u32 = (VDS_ON | VDS_DISPAY_DIS | VDS_B_MOD_EVE_MASK | VDS_BON);

/* LVDS_PLL_CNTL bit constatns */
pub const HSYNC_DELAY_SHIFT: u32 = 0x1c;
pub const HSYNC_DELAY_MASK: u32 = (0xf << 0x1c);

/* TMDS_TRANSMITTER_CNTL bit constants */
pub const TMDS_PLL_EN: u32 = (1 << 0);
pub const TMDS_PLLRST: u32 = (1 << 1);
pub const TMDS_RAN_PAT_RST: u32 = (1 << 7);
pub const TMDS_ICHCSEL: u32 = (1 << 28);

/* FP_HORZ_STRETCH bit constants */
pub const HORZ_STRETCH_RATIO_MASK: u32 = 0xffff;
pub const HORZ_STRETCH_RATIO_MAX: u32 = 4096;
pub const HORZ_PANEL_SIZE: u32 = (0x1ff << 16);
pub const HORZ_PANEL_SHIFT: u32 = 16;
pub const HORZ_STRETCH_PIXREP: u32 = (0 << 25);
pub const HORZ_STRETCH_BLEND: u32 = (1 << 26);
pub const HORZ_STRETCH_ENABLE: u32 = (1 << 25);
pub const HORZ_AUTO_RATIO: u32 = (1 << 27);
pub const HORZ_FP_LOOP_STRETCH: u32 = (0x7 << 28);
pub const HORZ_AUTO_RATIO_INC: u32 = (1 << 31);


/* FP_VERT_STRETCH bit constants */
pub const VERT_STRETCH_RATIO_MASK: u32 = 0xfff;
pub const VERT_STRETCH_RATIO_MAX: u32 = 4096;
pub const VERT_PANEL_SIZE: u32 = (0xfff << 12);
pub const VERT_PANEL_SHIFT: u32 = 12;
pub const VERT_STRETCH_LINREP: u32 = (0 << 26);
pub const VERT_STRETCH_BLEND: u32 = (1 << 26);
pub const VERT_STRETCH_ENABLE: u32 = (1 << 25);
pub const VERT_AUTO_RATIO_EN: u32 = (1 << 27);
pub const VERT_FP_LOOP_STRETCH: u32 = (0x7 << 28);
pub const VERT_STRETCH_RESERVED: u32 = 0xf1000000;

/* DAC_CNTL bit constants */
pub const DAC_8BIT_EN: u32 = 0x00000100;
pub const DAC_4BPP_PIX_ORDER: u32 = 0x00000200;
pub const DAC_CRC_EN: u32 = 0x00080000;
pub const DAC_MASK_ALL: u32 = (0xff << 24);
pub const DAC_PDWN: u32 = (1 << 15);
pub const DAC_EXPAND_MODE: u32 = (1 << 14);
pub const DAC_VGA_ADR_EN: u32 = (1 << 13);
pub const DAC_RANGE_CNTL: u32 = (3 <<  0);
pub const DAC_RANGE_CNTL_MASK: u32 = 0x03;
pub const DAC_BLANKING: u32 = (1 <<  2);
pub const DAC_CMP_EN: u32 = (1 <<  3);
pub const DAC_CMP_OUTPUT: u32 = (1 <<  7);

/* DAC_CNTL2 bit constants */
pub const DAC2_EXPAND_MODE: u32 = (1 << 14);
pub const DAC2_CMP_EN: u32 = (1 << 7);
pub const DAC2_PALETTE_ACCESS_CNTL: u32 = (1 << 5);

/* DAC_EXT_CNTL bit constants */
pub const DAC_FORCE_BLANK_OFF_EN: u32 = (1 << 4);
pub const DAC_FORCE_DATA_EN: u32 = (1 << 5);
pub const DAC_FORCE_DATA_SEL_MASK: u32 = (3 << 6);
pub const DAC_FORCE_DATA_MASK: u32 = 0x0003ff00;
pub const DAC_FORCE_DATA_SHIFT: u32 = 8;

/* GEN_RESET_CNTL bit constants */
pub const SOFT_RESET_GUI: u32 = 0x00000001;
pub const SOFT_RESET_VCLK: u32 = 0x00000100;
pub const SOFT_RESET_PCLK: u32 = 0x00000200;
pub const SOFT_RESET_ECP: u32 = 0x00000400;
pub const SOFT_RESET_DISPENG_XCLK: u32 = 0x00000800;

/* MEM_CNTL bit constants */
pub const MEM_CTLR_STATUS_IDLE: u32 = 0x00000000;
pub const MEM_CTLR_STATUS_BUSY: u32 = 0x00100000;
pub const MEM_SEQNCR_STATUS_IDLE: u32 = 0x00000000;
pub const MEM_SEQNCR_STATUS_BUSY: u32 = 0x00200000;
pub const MEM_ARBITER_STATUS_IDLE: u32 = 0x00000000;
pub const MEM_ARBITER_STATUS_BUSY: u32 = 0x00400000;
pub const MEM_REQ_UNLOCK: u32 = 0x00000000;
pub const MEM_REQ_LOCK: u32 = 0x00800000;
pub const MEM_NUM_CHANNELS_MASK: u32 = 0x00000001;
pub const MEM_USE_B_CH_ONLY: u32 = 0x00000002;
pub const RV100_MEM_HALF_MODE: u32 = 0x00000008;
pub const R300_MEM_NUM_CHANNELS_MASK: u32 = 0x00000003;
pub const R300_MEM_USE_CD_CH_ONLY: u32 = 0x00000004;


/* RBBM_SOFT_RESET bit constants */
pub const SOFT_RESET_CP: u32 = (1 <<  0);
pub const SOFT_RESET_HI: u32 = (1 <<  1);
pub const SOFT_RESET_SE: u32 = (1 <<  2);
pub const SOFT_RESET_RE: u32 = (1 <<  3);
pub const SOFT_RESET_PP: u32 = (1 <<  4);
pub const SOFT_RESET_E2: u32 = (1 <<  5);
pub const SOFT_RESET_RB: u32 = (1 <<  6);
pub const SOFT_RESET_HDP: u32 = (1 <<  7);

/* WAIT_UNTIL bit constants */
pub const WAIT_DMA_GUI_IDLE: u32 = (1 << 9);
pub const WAIT_2D_IDLECLEAN: u32 = (1 << 16);

/* SURFACE_CNTL bit constants */
pub const SURF_TRANSLATION_DIS: u32 = (1 << 8);
pub const NONSURF_AP0_SWP_16BPP: u32 = (1 << 20);
pub const NONSURF_AP0_SWP_32BPP: u32 = (1 << 21);
pub const NONSURF_AP1_SWP_16BPP: u32 = (1 << 22);
pub const NONSURF_AP1_SWP_32BPP: u32 = (1 << 23);

/* DEFAULT_SC_BOTTOM_RIGHT bit constants */
pub const DEFAULT_SC_RIGHT_MAX: u32 = (0x1fff << 0);
pub const DEFAULT_SC_BOTTOM_MAX: u32 = (0x1fff << 16);

/* MM_INDEX bit constants */
pub const MM_APER: u32 = 0x80000000;

/* CLR_CMP_CNTL bit constants */
pub const COMPARE_SRC_FALSE: u32 = 0x00000000;
pub const COMPARE_SRC_TRUE: u32 = 0x00000001;
pub const COMPARE_SRC_NOT_EQUAL: u32 = 0x00000004;
pub const COMPARE_SRC_EQUAL: u32 = 0x00000005;
pub const COMPARE_SRC_EQUAL_FLIP: u32 = 0x00000007;
pub const COMPARE_DST_FALSE: u32 = 0x00000000;
pub const COMPARE_DST_TRUE: u32 = 0x00000100;
pub const COMPARE_DST_NOT_EQUAL: u32 = 0x00000400;
pub const COMPARE_DST_EQUAL: u32 = 0x00000500;
pub const COMPARE_DESTINATION: u32 = 0x00000000;
pub const COMPARE_SOURCE: u32 = 0x01000000;
pub const COMPARE_SRC_AND_DST: u32 = 0x02000000;


/* DP_CNTL bit constants */
pub const DST_X_RIGHT_TO_LEFT: u32 = 0x00000000;
pub const DST_X_LEFT_TO_RIGHT: u32 = 0x00000001;
pub const DST_Y_BOTTOM_TO_TOP: u32 = 0x00000000;
pub const DST_Y_TOP_TO_BOTTOM: u32 = 0x00000002;
pub const DST_X_MAJOR: u32 = 0x00000000;
pub const DST_Y_MAJOR: u32 = 0x00000004;
pub const DST_X_TILE: u32 = 0x00000008;
pub const DST_Y_TILE: u32 = 0x00000010;
pub const DST_LAST_PEL: u32 = 0x00000020;
pub const DST_TRAIL_X_RIGHT_TO_LEFT: u32 = 0x00000000;
pub const DST_TRAIL_X_LEFT_TO_RIGHT: u32 = 0x00000040;
pub const DST_TRAP_FILL_RIGHT_TO_LEFT: u32 = 0x00000000;
pub const DST_TRAP_FILL_LEFT_TO_RIGHT: u32 = 0x00000080;
pub const DST_BRES_SIGN: u32 = 0x00000100;
pub const DST_HOST_BIG_ENDIAN_EN: u32 = 0x00000200;
pub const DST_POLYLINE_NONLAST: u32 = 0x00008000;
pub const DST_RASTER_STALL: u32 = 0x00010000;
pub const DST_POLY_EDGE: u32 = 0x00040000;


/* DP_CNTL_YDIR_XDIR_YMAJOR bit constants (short version of DP_CNTL) */
pub const DST_X_MAJOR_S: u32 = 0x00000000;
pub const DST_Y_MAJOR_S: u32 = 0x00000001;
pub const DST_Y_BOTTOM_TO_TOP_S: u32 = 0x00000000;
pub const DST_Y_TOP_TO_BOTTOM_S: u32 = 0x00008000;
pub const DST_X_RIGHT_TO_LEFT_S: u32 = 0x00000000;
pub const DST_X_LEFT_TO_RIGHT_S: u32 = 0x80000000;


/* DP_DATATYPE bit constants */
pub const DST_8BPP: u32 = 0x00000002;
pub const DST_15BPP: u32 = 0x00000003;
pub const DST_16BPP: u32 = 0x00000004;
pub const DST_24BPP: u32 = 0x00000005;
pub const DST_32BPP: u32 = 0x00000006;
pub const DST_8BPP_RGB332: u32 = 0x00000007;
pub const DST_8BPP_Y8: u32 = 0x00000008;
pub const DST_8BPP_RGB8: u32 = 0x00000009;
pub const DST_16BPP_VYUY422: u32 = 0x0000000b;
pub const DST_16BPP_YVYU422: u32 = 0x0000000c;
pub const DST_32BPP_AYUV444: u32 = 0x0000000e;
pub const DST_16BPP_ARGB4444: u32 = 0x0000000f;
pub const BRUSH_SOLIDCOLOR: u32 = 0x00000d00;
pub const SRC_MONO: u32 = 0x00000000;
pub const SRC_MONO_LBKGD: u32 = 0x00010000;
pub const SRC_DSTCOLOR: u32 = 0x00030000;
pub const BYTE_ORDER_MSB_TO_LSB: u32 = 0x00000000;
pub const BYTE_ORDER_LSB_TO_MSB: u32 = 0x40000000;
pub const DP_CONVERSION_TEMP: u32 = 0x80000000;
pub const HOST_BIG_ENDIAN_EN: u32 = (1 << 29);


/* DP_GUI_MASTER_CNTL bit constants */
pub const GMC_SRC_PITCH_OFFSET_DEFAULT: u32 = 0x00000000;
pub const GMC_SRC_PITCH_OFFSET_LEAVE: u32 = 0x00000001;
pub const GMC_DST_PITCH_OFFSET_DEFAULT: u32 = 0x00000000;
pub const GMC_DST_PITCH_OFFSET_LEAVE: u32 = 0x00000002;
pub const GMC_SRC_CLIP_DEFAULT: u32 = 0x00000000;
pub const GMC_SRC_CLIP_LEAVE: u32 = 0x00000004;
pub const GMC_DST_CLIP_DEFAULT: u32 = 0x00000000;
pub const GMC_DST_CLIP_LEAVE: u32 = 0x00000008;
pub const GMC_BRUSH_8x8MONO: u32 = 0x00000000;
pub const GMC_BRUSH_8x8MONO_LBKGD: u32 = 0x00000010;
pub const GMC_BRUSH_8x1MONO: u32 = 0x00000020;
pub const GMC_BRUSH_8x1MONO_LBKGD: u32 = 0x00000030;
pub const GMC_BRUSH_1x8MONO: u32 = 0x00000040;
pub const GMC_BRUSH_1x8MONO_LBKGD: u32 = 0x00000050;
pub const GMC_BRUSH_32x1MONO: u32 = 0x00000060;
pub const GMC_BRUSH_32x1MONO_LBKGD: u32 = 0x00000070;
pub const GMC_BRUSH_32x32MONO: u32 = 0x00000080;
pub const GMC_BRUSH_32x32MONO_LBKGD: u32 = 0x00000090;
pub const GMC_BRUSH_8x8COLOR: u32 = 0x000000a0;
pub const GMC_BRUSH_8x1COLOR: u32 = 0x000000b0;
pub const GMC_BRUSH_1x8COLOR: u32 = 0x000000c0;
pub const GMC_BRUSH_SOLID_COLOR: u32 = 0x000000d0;
pub const GMC_DST_8BPP: u32 = 0x00000200;
pub const GMC_DST_15BPP: u32 = 0x00000300;
pub const GMC_DST_16BPP: u32 = 0x00000400;
pub const GMC_DST_24BPP: u32 = 0x00000500;
pub const GMC_DST_32BPP: u32 = 0x00000600;
pub const GMC_DST_8BPP_RGB332: u32 = 0x00000700;
pub const GMC_DST_8BPP_Y8: u32 = 0x00000800;
pub const GMC_DST_8BPP_RGB8: u32 = 0x00000900;
pub const GMC_DST_16BPP_VYUY422: u32 = 0x00000b00;
pub const GMC_DST_16BPP_YVYU422: u32 = 0x00000c00;
pub const GMC_DST_32BPP_AYUV444: u32 = 0x00000e00;
pub const GMC_DST_16BPP_ARGB4444: u32 = 0x00000f00;
pub const GMC_SRC_MONO: u32 = 0x00000000;
pub const GMC_SRC_MONO_LBKGD: u32 = 0x00001000;
pub const GMC_SRC_DSTCOLOR: u32 = 0x00003000;
pub const GMC_BYTE_ORDER_MSB_TO_LSB: u32 = 0x00000000;
pub const GMC_BYTE_ORDER_LSB_TO_MSB: u32 = 0x00004000;
pub const GMC_DP_CONVERSION_TEMP_9300: u32 = 0x00008000;
pub const GMC_DP_CONVERSION_TEMP_6500: u32 = 0x00000000;
pub const GMC_DP_SRC_RECT: u32 = 0x02000000;
pub const GMC_DP_SRC_HOST: u32 = 0x03000000;
pub const GMC_DP_SRC_HOST_BYTEALIGN: u32 = 0x04000000;
pub const GMC_3D_FCN_EN_CLR: u32 = 0x00000000;
pub const GMC_3D_FCN_EN_SET: u32 = 0x08000000;
pub const GMC_DST_CLR_CMP_FCN_LEAVE: u32 = 0x00000000;
pub const GMC_DST_CLR_CMP_FCN_CLEAR: u32 = 0x10000000;
pub const GMC_AUX_CLIP_LEAVE: u32 = 0x00000000;
pub const GMC_AUX_CLIP_CLEAR: u32 = 0x20000000;
pub const GMC_WRITE_MASK_LEAVE: u32 = 0x00000000;
pub const GMC_WRITE_MASK_SET: u32 = 0x40000000;
pub const GMC_CLR_CMP_CNTL_DIS: u32 = (1 << 28);
pub const GMC_SRC_DATATYPE_COLOR: u32 = (3 << 12);
pub const ROP3_S: u32 = 0x00cc0000;
pub const ROP3_SRCCOPY: u32 = 0x00cc0000;
pub const ROP3_P: u32 = 0x00f00000;
pub const ROP3_PATCOPY: u32 = 0x00f00000;
pub const DP_SRC_SOURCE_MASK: u32 = (7    << 24);
pub const GMC_BRUSH_NONE: u32 = (15   <<  4);
pub const DP_SRC_SOURCE_MEMORY: u32 = (2    << 24);
pub const GMC_BRUSH_SOLIDCOLOR: u32 = 0x000000d0;

/* DP_MIX bit constants */
pub const DP_SRC_RECT: u32 = 0x00000200;
pub const DP_SRC_HOST: u32 = 0x00000300;
pub const DP_SRC_HOST_BYTEALIGN: u32 = 0x00000400;

/* MPLL_CNTL bit constants */
pub const MPLL_RESET: u32 = 0x00000001;

/* MDLL_CKO bit constants */
pub const MCKOA_SLEEP: u32 = 0x00000001;
pub const MCKOA_RESET: u32 = 0x00000002;
pub const MCKOA_REF_SKEW_MASK: u32 = 0x00000700;
pub const MCKOA_FB_SKEW_MASK: u32 = 0x00007000;

/* MDLL_RDCKA bit constants */
pub const MRDCKA0_SLEEP: u32 = 0x00000001;
pub const MRDCKA0_RESET: u32 = 0x00000002;
pub const MRDCKA1_SLEEP: u32 = 0x00010000;
pub const MRDCKA1_RESET: u32 = 0x00020000;

/* VCLK_ECP_CNTL constants */
pub const VCLK_SRC_SEL_MASK: u32 = 0x03;
pub const VCLK_SRC_SEL_CPUCLK: u32 = 0x00;
pub const VCLK_SRC_SEL_PSCANCLK: u32 = 0x01;
pub const VCLK_SRC_SEL_BYTECLK: u32 = 0x02;
pub const VCLK_SRC_SEL_PPLLCLK: u32 = 0x03;
pub const PIXCLK_ALWAYS_ONb: u32 = 0x00000040;
pub const PIXCLK_DAC_ALWAYS_ONb: u32 = 0x00000080;

/* BUS_CNTL1 constants */
pub const BUS_CNTL1_MOBILE_PLATFORM_SEL_MASK: u32 = 0x0c000000;
pub const BUS_CNTL1_MOBILE_PLATFORM_SEL_SHIFT: u32 = 26;
pub const BUS_CNTL1_AGPCLK_VALID: u32 = 0x80000000;

/* PLL_PWRMGT_CNTL constants */
pub const PLL_PWRMGT_CNTL_SPLL_TURNOFF: u32 = 0x00000002;
pub const PLL_PWRMGT_CNTL_PPLL_TURNOFF: u32 = 0x00000004;
pub const PLL_PWRMGT_CNTL_P2PLL_TURNOFF: u32 = 0x00000008;
pub const PLL_PWRMGT_CNTL_TVPLL_TURNOFF: u32 = 0x00000010;
pub const PLL_PWRMGT_CNTL_MOBILE_SU: u32 = 0x00010000;
pub const PLL_PWRMGT_CNTL_SU_SCLK_USE_BCLK: u32 = 0x00020000;
pub const PLL_PWRMGT_CNTL_SU_MCLK_USE_BCLK: u32 = 0x00040000;

/* TV_DAC_CNTL constants */
pub const TV_DAC_CNTL_BGSLEEP: u32 = 0x00000040;
pub const TV_DAC_CNTL_DETECT: u32 = 0x00000010;
pub const TV_DAC_CNTL_BGADJ_MASK: u32 = 0x000f0000;
pub const TV_DAC_CNTL_DACADJ_MASK: u32 = 0x00f00000;
pub const TV_DAC_CNTL_BGADJ__SHIFT: u32 = 16;
pub const TV_DAC_CNTL_DACADJ__SHIFT: u32 = 20;
pub const TV_DAC_CNTL_RDACPD: u32 = 0x01000000;
pub const TV_DAC_CNTL_GDACPD: u32 = 0x02000000;
pub const TV_DAC_CNTL_BDACPD: u32 = 0x04000000;

/* DISP_MISC_CNTL constants */
pub const DISP_MISC_CNTL_SOFT_RESET_GRPH_PP: u32 = (1 << 0);
pub const DISP_MISC_CNTL_SOFT_RESET_SUBPIC_PP: u32 = (1 << 1);
pub const DISP_MISC_CNTL_SOFT_RESET_OV0_PP: u32 = (1 << 2);
pub const DISP_MISC_CNTL_SOFT_RESET_GRPH_SCLK: u32 = (1 << 4);
pub const DISP_MISC_CNTL_SOFT_RESET_SUBPIC_SCLK: u32 = (1 << 5);
pub const DISP_MISC_CNTL_SOFT_RESET_OV0_SCLK: u32 = (1 << 6);
pub const DISP_MISC_CNTL_SOFT_RESET_GRPH2_PP: u32 = (1 << 12);
pub const DISP_MISC_CNTL_SOFT_RESET_GRPH2_SCLK: u32 = (1 << 15);
pub const DISP_MISC_CNTL_SOFT_RESET_LVDS: u32 = (1 << 16);
pub const DISP_MISC_CNTL_SOFT_RESET_TMDS: u32 = (1 << 17);
pub const DISP_MISC_CNTL_SOFT_RESET_DIG_TMDS: u32 = (1 << 18);
pub const DISP_MISC_CNTL_SOFT_RESET_TV: u32 = (1 << 19);

/* DISP_PWR_MAN constants */
pub const DISP_PWR_MAN_DISP_PWR_MAN_D3_CRTC_EN: u32 = (1 << 0);
pub const DISP_PWR_MAN_DISP2_PWR_MAN_D3_CRTC2_EN: u32 = (1 << 4);
pub const DISP_PWR_MAN_DISP_D3_RST: u32 = (1 << 16);
pub const DISP_PWR_MAN_DISP_D3_REG_RST: u32 = (1 << 17);
pub const DISP_PWR_MAN_DISP_D3_GRPH_RST: u32 = (1 << 18);
pub const DISP_PWR_MAN_DISP_D3_SUBPIC_RST: u32 = (1 << 19);
pub const DISP_PWR_MAN_DISP_D3_OV0_RST: u32 = (1 << 20);
pub const DISP_PWR_MAN_DISP_D1D2_GRPH_RST: u32 = (1 << 21);
pub const DISP_PWR_MAN_DISP_D1D2_SUBPIC_RST: u32 = (1 << 22);
pub const DISP_PWR_MAN_DISP_D1D2_OV0_RST: u32 = (1 << 23);
pub const DISP_PWR_MAN_DIG_TMDS_ENABLE_RST: u32 = (1 << 24);
pub const DISP_PWR_MAN_TV_ENABLE_RST: u32 = (1 << 25);
pub const DISP_PWR_MAN_AUTO_PWRUP_EN: u32 = (1 << 26);

/* masks */

pub const CNFG_MEMSIZE_MASK: u32 = 0x1f000000;
pub const MEM_CFG_TYPE: u32 = 0x40000000;
pub const DST_OFFSET_MASK: u32 = 0x003fffff;
pub const DST_PITCH_MASK: u32 = 0x3fc00000;
pub const DEFAULT_TILE_MASK: u32 = 0xc0000000;
pub const PPLL_DIV_SEL_MASK: u32 = 0x00000300;
pub const PPLL_RESET: u32 = 0x00000001;
pub const PPLL_SLEEP: u32 = 0x00000002;
pub const PPLL_ATOMIC_UPDATE_EN: u32 = 0x00010000;
pub const PPLL_REF_DIV_MASK: u32 = 0x000003ff;
pub const PPLL_FB3_DIV_MASK: u32 = 0x000007ff;
pub const PPLL_POST3_DIV_MASK: u32 = 0x00070000;
pub const PPLL_ATOMIC_UPDATE_R: u32 = 0x00008000;
pub const PPLL_ATOMIC_UPDATE_W: u32 = 0x00008000;
pub const PPLL_VGA_ATOMIC_UPDATE_EN: u32 = 0x00020000;
pub const R300_PPLL_REF_DIV_ACC_MASK: u32 = (0x3ff << 18);
pub const R300_PPLL_REF_DIV_ACC_SHIFT: u32 = 18;

pub const GUI_ACTIVE: u32 = 0x80000000;


pub const MC_IND_INDEX: u32 = 0x01F8;
pub const MC_IND_DATA: u32 = 0x01FC;

/* PAD_CTLR_STRENGTH */
pub const PAD_MANUAL_OVERRIDE: u32 = 0x80000000;

// pllCLK_PIN_CNTL
pub const CLK_PIN_CNTL__OSC_EN_MASK: u32 = 0x00000001;
pub const CLK_PIN_CNTL__OSC_EN: u32 = 0x00000001;
pub const CLK_PIN_CNTL__XTL_LOW_GAIN_MASK: u32 = 0x00000004;
pub const CLK_PIN_CNTL__XTL_LOW_GAIN: u32 = 0x00000004;
pub const CLK_PIN_CNTL__DONT_USE_XTALIN_MASK: u32 = 0x00000010;
pub const CLK_PIN_CNTL__DONT_USE_XTALIN: u32 = 0x00000010;
pub const CLK_PIN_CNTL__SLOW_CLOCK_SOURCE_MASK: u32 = 0x00000020;
pub const CLK_PIN_CNTL__SLOW_CLOCK_SOURCE: u32 = 0x00000020;
pub const CLK_PIN_CNTL__CG_CLK_TO_OUTPIN_MASK: u32 = 0x00000800;
pub const CLK_PIN_CNTL__CG_CLK_TO_OUTPIN: u32 = 0x00000800;
pub const CLK_PIN_CNTL__CG_COUNT_UP_TO_OUTPIN_MASK: u32 = 0x00001000;
pub const CLK_PIN_CNTL__CG_COUNT_UP_TO_OUTPIN: u32 = 0x00001000;
pub const CLK_PIN_CNTL__ACCESS_REGS_IN_SUSPEND_MASK: u32 = 0x00002000;
pub const CLK_PIN_CNTL__ACCESS_REGS_IN_SUSPEND: u32 = 0x00002000;
pub const CLK_PIN_CNTL__CG_SPARE_MASK: u32 = 0x00004000;
pub const CLK_PIN_CNTL__CG_SPARE: u32 = 0x00004000;
pub const CLK_PIN_CNTL__SCLK_DYN_START_CNTL_MASK: u32 = 0x00008000;
pub const CLK_PIN_CNTL__SCLK_DYN_START_CNTL: u32 = 0x00008000;
pub const CLK_PIN_CNTL__CP_CLK_RUNNING_MASK: u32 = 0x00010000;
pub const CLK_PIN_CNTL__CP_CLK_RUNNING: u32 = 0x00010000;
pub const CLK_PIN_CNTL__CG_SPARE_RD_MASK: u32 = 0x00060000;
pub const CLK_PIN_CNTL__XTALIN_ALWAYS_ONb_MASK: u32 = 0x00080000;
pub const CLK_PIN_CNTL__XTALIN_ALWAYS_ONb: u32 = 0x00080000;
pub const CLK_PIN_CNTL__PWRSEQ_DELAY_MASK: u32 = 0xff000000;

// pllCLK_PWRMGT_CNTL
pub const CLK_PWRMGT_CNTL__MPLL_PWRMGT_OFF__SHIFT: u32 = 0x00000000;
pub const CLK_PWRMGT_CNTL__SPLL_PWRMGT_OFF__SHIFT: u32 = 0x00000001;
pub const CLK_PWRMGT_CNTL__PPLL_PWRMGT_OFF__SHIFT: u32 = 0x00000002;
pub const CLK_PWRMGT_CNTL__P2PLL_PWRMGT_OFF__SHIFT: u32 = 0x00000003;
pub const CLK_PWRMGT_CNTL__MCLK_TURNOFF__SHIFT: u32 = 0x00000004;
pub const CLK_PWRMGT_CNTL__SCLK_TURNOFF__SHIFT: u32 = 0x00000005;
pub const CLK_PWRMGT_CNTL__PCLK_TURNOFF__SHIFT: u32 = 0x00000006;
pub const CLK_PWRMGT_CNTL__P2CLK_TURNOFF__SHIFT: u32 = 0x00000007;
pub const CLK_PWRMGT_CNTL__MC_CH_MODE__SHIFT: u32 = 0x00000008;
pub const CLK_PWRMGT_CNTL__TEST_MODE__SHIFT: u32 = 0x00000009;
pub const CLK_PWRMGT_CNTL__GLOBAL_PMAN_EN__SHIFT: u32 = 0x0000000a;
pub const CLK_PWRMGT_CNTL__ENGINE_DYNCLK_MODE__SHIFT: u32 = 0x0000000c;
pub const CLK_PWRMGT_CNTL__ACTIVE_HILO_LAT__SHIFT: u32 = 0x0000000d;
pub const CLK_PWRMGT_CNTL__DISP_DYN_STOP_LAT__SHIFT: u32 = 0x0000000f;
pub const CLK_PWRMGT_CNTL__MC_BUSY__SHIFT: u32 = 0x00000010;
pub const CLK_PWRMGT_CNTL__MC_INT_CNTL__SHIFT: u32 = 0x00000011;
pub const CLK_PWRMGT_CNTL__MC_SWITCH__SHIFT: u32 = 0x00000012;
pub const CLK_PWRMGT_CNTL__DLL_READY__SHIFT: u32 = 0x00000013;
pub const CLK_PWRMGT_CNTL__DISP_PM__SHIFT: u32 = 0x00000014;
pub const CLK_PWRMGT_CNTL__DYN_STOP_MODE__SHIFT: u32 = 0x00000015;
pub const CLK_PWRMGT_CNTL__CG_NO1_DEBUG__SHIFT: u32 = 0x00000018;
pub const CLK_PWRMGT_CNTL__TVPLL_PWRMGT_OFF__SHIFT: u32 = 0x0000001e;
pub const CLK_PWRMGT_CNTL__TVCLK_TURNOFF__SHIFT: u32 = 0x0000001f;

// pllP2PLL_CNTL
pub const P2PLL_CNTL__P2PLL_RESET_MASK: u32 = 0x00000001;
pub const P2PLL_CNTL__P2PLL_RESET: u32 = 0x00000001;
pub const P2PLL_CNTL__P2PLL_SLEEP_MASK: u32 = 0x00000002;
pub const P2PLL_CNTL__P2PLL_SLEEP: u32 = 0x00000002;
pub const P2PLL_CNTL__P2PLL_TST_EN_MASK: u32 = 0x00000004;
pub const P2PLL_CNTL__P2PLL_TST_EN: u32 = 0x00000004;
pub const P2PLL_CNTL__P2PLL_REFCLK_SEL_MASK: u32 = 0x00000010;
pub const P2PLL_CNTL__P2PLL_REFCLK_SEL: u32 = 0x00000010;
pub const P2PLL_CNTL__P2PLL_FBCLK_SEL_MASK: u32 = 0x00000020;
pub const P2PLL_CNTL__P2PLL_FBCLK_SEL: u32 = 0x00000020;
pub const P2PLL_CNTL__P2PLL_TCPOFF_MASK: u32 = 0x00000040;
pub const P2PLL_CNTL__P2PLL_TCPOFF: u32 = 0x00000040;
pub const P2PLL_CNTL__P2PLL_TVCOMAX_MASK: u32 = 0x00000080;
pub const P2PLL_CNTL__P2PLL_TVCOMAX: u32 = 0x00000080;
pub const P2PLL_CNTL__P2PLL_PCP_MASK: u32 = 0x00000700;
pub const P2PLL_CNTL__P2PLL_PVG_MASK: u32 = 0x00003800;
pub const P2PLL_CNTL__P2PLL_PDC_MASK: u32 = 0x0000c000;
pub const P2PLL_CNTL__P2PLL_ATOMIC_UPDATE_EN_MASK: u32 = 0x00010000;
pub const P2PLL_CNTL__P2PLL_ATOMIC_UPDATE_EN: u32 = 0x00010000;
pub const P2PLL_CNTL__P2PLL_ATOMIC_UPDATE_SYNC_MASK: u32 = 0x00040000;
pub const P2PLL_CNTL__P2PLL_ATOMIC_UPDATE_SYNC: u32 = 0x00040000;
pub const P2PLL_CNTL__P2PLL_DISABLE_AUTO_RESET_MASK: u32 = 0x00080000;
pub const P2PLL_CNTL__P2PLL_DISABLE_AUTO_RESET: u32 = 0x00080000;

// pllPIXCLKS_CNTL
pub const PIXCLKS_CNTL__PIX2CLK_SRC_SEL__SHIFT: u32 = 0x00000000;
pub const PIXCLKS_CNTL__PIX2CLK_INVERT__SHIFT: u32 = 0x00000004;
pub const PIXCLKS_CNTL__PIX2CLK_SRC_INVERT__SHIFT: u32 = 0x00000005;
pub const PIXCLKS_CNTL__PIX2CLK_ALWAYS_ONb__SHIFT: u32 = 0x00000006;
pub const PIXCLKS_CNTL__PIX2CLK_DAC_ALWAYS_ONb__SHIFT: u32 = 0x00000007;
pub const PIXCLKS_CNTL__PIXCLK_TV_SRC_SEL__SHIFT: u32 = 0x00000008;
pub const PIXCLKS_CNTL__PIXCLK_BLEND_ALWAYS_ONb__SHIFT: u32 = 0x0000000b;
pub const PIXCLKS_CNTL__PIXCLK_GV_ALWAYS_ONb__SHIFT: u32 = 0x0000000c;
pub const PIXCLKS_CNTL__PIXCLK_DIG_TMDS_ALWAYS_ONb__SHIFT: u32 = 0x0000000d;
pub const PIXCLKS_CNTL__PIXCLK_LVDS_ALWAYS_ONb__SHIFT: u32 = 0x0000000e;
pub const PIXCLKS_CNTL__PIXCLK_TMDS_ALWAYS_ONb__SHIFT: u32 = 0x0000000f;


// pllPIXCLKS_CNTL
pub const PIXCLKS_CNTL__PIX2CLK_SRC_SEL_MASK: u32 = 0x00000003;
pub const PIXCLKS_CNTL__PIX2CLK_INVERT: u32 = 0x00000010;
pub const PIXCLKS_CNTL__PIX2CLK_SRC_INVERT: u32 = 0x00000020;
pub const PIXCLKS_CNTL__PIX2CLK_ALWAYS_ONb: u32 = 0x00000040;
pub const PIXCLKS_CNTL__PIX2CLK_DAC_ALWAYS_ONb: u32 = 0x00000080;
pub const PIXCLKS_CNTL__PIXCLK_TV_SRC_SEL: u32 = 0x00000100;
pub const PIXCLKS_CNTL__PIXCLK_BLEND_ALWAYS_ONb: u32 = 0x00000800;
pub const PIXCLKS_CNTL__PIXCLK_GV_ALWAYS_ONb: u32 = 0x00001000;
pub const PIXCLKS_CNTL__PIXCLK_DIG_TMDS_ALWAYS_ONb: u32 = 0x00002000;
pub const PIXCLKS_CNTL__PIXCLK_LVDS_ALWAYS_ONb: u32 = 0x00004000;
pub const PIXCLKS_CNTL__PIXCLK_TMDS_ALWAYS_ONb: u32 = 0x00008000;
pub const PIXCLKS_CNTL__DISP_TVOUT_PIXCLK_TV_ALWAYS_ONb: u32 = (1 << 9);
pub const PIXCLKS_CNTL__R300_DVOCLK_ALWAYS_ONb: u32 = (1 << 10);
pub const PIXCLKS_CNTL__R300_PIXCLK_DVO_ALWAYS_ONb: u32 = (1 << 13);
pub const PIXCLKS_CNTL__R300_PIXCLK_TRANS_ALWAYS_ONb: u32 = (1 << 16);
pub const PIXCLKS_CNTL__R300_PIXCLK_TVO_ALWAYS_ONb: u32 = (1 << 17);
pub const PIXCLKS_CNTL__R300_P2G2CLK_ALWAYS_ONb: u32 = (1 << 18);
pub const PIXCLKS_CNTL__R300_P2G2CLK_DAC_ALWAYS_ONb: u32 = (1 << 19);
pub const PIXCLKS_CNTL__R300_DISP_DAC_PIXCLK_DAC2_BLANK_OFF: u32 = (1 << 23);


// pllP2PLL_DIV_0
pub const P2PLL_DIV_0__P2PLL_FB_DIV_MASK: u32 = 0x000007ff;
pub const P2PLL_DIV_0__P2PLL_ATOMIC_UPDATE_W_MASK: u32 = 0x00008000;
pub const P2PLL_DIV_0__P2PLL_ATOMIC_UPDATE_W: u32 = 0x00008000;
pub const P2PLL_DIV_0__P2PLL_ATOMIC_UPDATE_R_MASK: u32 = 0x00008000;
pub const P2PLL_DIV_0__P2PLL_ATOMIC_UPDATE_R: u32 = 0x00008000;
pub const P2PLL_DIV_0__P2PLL_POST_DIV_MASK: u32 = 0x00070000;

// pllSCLK_CNTL
pub const SCLK_CNTL__SCLK_SRC_SEL_MASK: u32 = 0x00000007;
pub const SCLK_CNTL__CP_MAX_DYN_STOP_LAT: u32 = 0x00000008;
pub const SCLK_CNTL__HDP_MAX_DYN_STOP_LAT: u32 = 0x00000010;
pub const SCLK_CNTL__TV_MAX_DYN_STOP_LAT: u32 = 0x00000020;
pub const SCLK_CNTL__E2_MAX_DYN_STOP_LAT: u32 = 0x00000040;
pub const SCLK_CNTL__SE_MAX_DYN_STOP_LAT: u32 = 0x00000080;
pub const SCLK_CNTL__IDCT_MAX_DYN_STOP_LAT: u32 = 0x00000100;
pub const SCLK_CNTL__VIP_MAX_DYN_STOP_LAT: u32 = 0x00000200;
pub const SCLK_CNTL__RE_MAX_DYN_STOP_LAT: u32 = 0x00000400;
pub const SCLK_CNTL__PB_MAX_DYN_STOP_LAT: u32 = 0x00000800;
pub const SCLK_CNTL__TAM_MAX_DYN_STOP_LAT: u32 = 0x00001000;
pub const SCLK_CNTL__TDM_MAX_DYN_STOP_LAT: u32 = 0x00002000;
pub const SCLK_CNTL__RB_MAX_DYN_STOP_LAT: u32 = 0x00004000;
pub const SCLK_CNTL__DYN_STOP_LAT_MASK: u32 = 0x00007ff8;
pub const SCLK_CNTL__FORCE_DISP2: u32 = 0x00008000;
pub const SCLK_CNTL__FORCE_CP: u32 = 0x00010000;
pub const SCLK_CNTL__FORCE_HDP: u32 = 0x00020000;
pub const SCLK_CNTL__FORCE_DISP1: u32 = 0x00040000;
pub const SCLK_CNTL__FORCE_TOP: u32 = 0x00080000;
pub const SCLK_CNTL__FORCE_E2: u32 = 0x00100000;
pub const SCLK_CNTL__FORCE_SE: u32 = 0x00200000;
pub const SCLK_CNTL__FORCE_IDCT: u32 = 0x00400000;
pub const SCLK_CNTL__FORCE_VIP: u32 = 0x00800000;
pub const SCLK_CNTL__FORCE_RE: u32 = 0x01000000;
pub const SCLK_CNTL__FORCE_PB: u32 = 0x02000000;
pub const SCLK_CNTL__FORCE_TAM: u32 = 0x04000000;
pub const SCLK_CNTL__FORCE_TDM: u32 = 0x08000000;
pub const SCLK_CNTL__FORCE_RB: u32 = 0x10000000;
pub const SCLK_CNTL__FORCE_TV_SCLK: u32 = 0x20000000;
pub const SCLK_CNTL__FORCE_SUBPIC: u32 = 0x40000000;
pub const SCLK_CNTL__FORCE_OV0: u32 = 0x80000000;
pub const SCLK_CNTL__R300_FORCE_VAP: u32 = (1<<21);
pub const SCLK_CNTL__R300_FORCE_SR: u32 = (1<<25);
pub const SCLK_CNTL__R300_FORCE_PX: u32 = (1<<26);
pub const SCLK_CNTL__R300_FORCE_TX: u32 = (1<<27);
pub const SCLK_CNTL__R300_FORCE_US: u32 = (1<<28);
pub const SCLK_CNTL__R300_FORCE_SU: u32 = (1<<30);
pub const SCLK_CNTL__FORCEON_MASK: u32 = 0xffff8000;

// pllSCLK_CNTL2
pub const SCLK_CNTL2__R300_TCL_MAX_DYN_STOP_LAT: u32 = (1<<10);
pub const SCLK_CNTL2__R300_GA_MAX_DYN_STOP_LAT: u32 = (1<<11);
pub const SCLK_CNTL2__R300_CBA_MAX_DYN_STOP_LAT: u32 = (1<<12);
pub const SCLK_CNTL2__R300_FORCE_TCL: u32 = (1<<13);
pub const SCLK_CNTL2__R300_FORCE_CBA: u32 = (1<<14);
pub const SCLK_CNTL2__R300_FORCE_GA: u32 = (1<<15);

// SCLK_MORE_CNTL
pub const SCLK_MORE_CNTL__DISPREGS_MAX_DYN_STOP_LAT: u32 = 0x00000001;
pub const SCLK_MORE_CNTL__MC_GUI_MAX_DYN_STOP_LAT: u32 = 0x00000002;
pub const SCLK_MORE_CNTL__MC_HOST_MAX_DYN_STOP_LAT: u32 = 0x00000004;
pub const SCLK_MORE_CNTL__FORCE_DISPREGS: u32 = 0x00000100;
pub const SCLK_MORE_CNTL__FORCE_MC_GUI: u32 = 0x00000200;
pub const SCLK_MORE_CNTL__FORCE_MC_HOST: u32 = 0x00000400;
pub const SCLK_MORE_CNTL__STOP_SCLK_EN: u32 = 0x00001000;
pub const SCLK_MORE_CNTL__STOP_SCLK_A: u32 = 0x00002000;
pub const SCLK_MORE_CNTL__STOP_SCLK_B: u32 = 0x00004000;
pub const SCLK_MORE_CNTL__STOP_SCLK_C: u32 = 0x00008000;
pub const SCLK_MORE_CNTL__HALF_SPEED_SCLK: u32 = 0x00010000;
pub const SCLK_MORE_CNTL__IO_CG_VOLTAGE_DROP: u32 = 0x00020000;
pub const SCLK_MORE_CNTL__TVFB_SOFT_RESET: u32 = 0x00040000;
pub const SCLK_MORE_CNTL__VOLTAGE_DROP_SYNC: u32 = 0x00080000;
pub const SCLK_MORE_CNTL__IDLE_DELAY_HALF_SCLK: u32 = 0x00400000;
pub const SCLK_MORE_CNTL__AGP_BUSY_HALF_SCLK: u32 = 0x00800000;
pub const SCLK_MORE_CNTL__CG_SPARE_RD_C_MASK: u32 = 0xff000000;
pub const SCLK_MORE_CNTL__FORCEON: u32 = 0x00000700;

// MCLK_CNTL
pub const MCLK_CNTL__MCLKA_SRC_SEL_MASK: u32 = 0x00000007;
pub const MCLK_CNTL__YCLKA_SRC_SEL_MASK: u32 = 0x00000070;
pub const MCLK_CNTL__MCLKB_SRC_SEL_MASK: u32 = 0x00000700;
pub const MCLK_CNTL__YCLKB_SRC_SEL_MASK: u32 = 0x00007000;
pub const MCLK_CNTL__FORCE_MCLKA_MASK: u32 = 0x00010000;
pub const MCLK_CNTL__FORCE_MCLKA: u32 = 0x00010000;
pub const MCLK_CNTL__FORCE_MCLKB_MASK: u32 = 0x00020000;
pub const MCLK_CNTL__FORCE_MCLKB: u32 = 0x00020000;
pub const MCLK_CNTL__FORCE_YCLKA_MASK: u32 = 0x00040000;
pub const MCLK_CNTL__FORCE_YCLKA: u32 = 0x00040000;
pub const MCLK_CNTL__FORCE_YCLKB_MASK: u32 = 0x00080000;
pub const MCLK_CNTL__FORCE_YCLKB: u32 = 0x00080000;
pub const MCLK_CNTL__FORCE_MC_MASK: u32 = 0x00100000;
pub const MCLK_CNTL__FORCE_MC: u32 = 0x00100000;
pub const MCLK_CNTL__FORCE_AIC_MASK: u32 = 0x00200000;
pub const MCLK_CNTL__FORCE_AIC: u32 = 0x00200000;
pub const MCLK_CNTL__MRDCKA0_SOUTSEL_MASK: u32 = 0x03000000;
pub const MCLK_CNTL__MRDCKA1_SOUTSEL_MASK: u32 = 0x0c000000;
pub const MCLK_CNTL__MRDCKB0_SOUTSEL_MASK: u32 = 0x30000000;
pub const MCLK_CNTL__MRDCKB1_SOUTSEL_MASK: u32 = 0xc0000000;
pub const MCLK_CNTL__R300_DISABLE_MC_MCLKA: u32 = (1 << 21);
pub const MCLK_CNTL__R300_DISABLE_MC_MCLKB: u32 = (1 << 21);

// MCLK_MISC
pub const MCLK_MISC__SCLK_SOURCED_FROM_MPLL_SEL_MASK: u32 = 0x00000003;
pub const MCLK_MISC__MCLK_FROM_SPLL_DIV_SEL_MASK: u32 = 0x00000004;
pub const MCLK_MISC__MCLK_FROM_SPLL_DIV_SEL: u32 = 0x00000004;
pub const MCLK_MISC__ENABLE_SCLK_FROM_MPLL_MASK: u32 = 0x00000008;
pub const MCLK_MISC__ENABLE_SCLK_FROM_MPLL: u32 = 0x00000008;
pub const MCLK_MISC__MPLL_MODEA_MODEC_HW_SEL_EN_MASK: u32 = 0x00000010;
pub const MCLK_MISC__MPLL_MODEA_MODEC_HW_SEL_EN: u32 = 0x00000010;
pub const MCLK_MISC__DLL_READY_LAT_MASK: u32 = 0x00000100;
pub const MCLK_MISC__DLL_READY_LAT: u32 = 0x00000100;
pub const MCLK_MISC__MC_MCLK_MAX_DYN_STOP_LAT_MASK: u32 = 0x00001000;
pub const MCLK_MISC__MC_MCLK_MAX_DYN_STOP_LAT: u32 = 0x00001000;
pub const MCLK_MISC__IO_MCLK_MAX_DYN_STOP_LAT_MASK: u32 = 0x00002000;
pub const MCLK_MISC__IO_MCLK_MAX_DYN_STOP_LAT: u32 = 0x00002000;
pub const MCLK_MISC__MC_MCLK_DYN_ENABLE_MASK: u32 = 0x00004000;
pub const MCLK_MISC__MC_MCLK_DYN_ENABLE: u32 = 0x00004000;
pub const MCLK_MISC__IO_MCLK_DYN_ENABLE_MASK: u32 = 0x00008000;
pub const MCLK_MISC__IO_MCLK_DYN_ENABLE: u32 = 0x00008000;
pub const MCLK_MISC__CGM_CLK_TO_OUTPIN_MASK: u32 = 0x00010000;
pub const MCLK_MISC__CGM_CLK_TO_OUTPIN: u32 = 0x00010000;
pub const MCLK_MISC__CLK_OR_COUNT_SEL_MASK: u32 = 0x00020000;
pub const MCLK_MISC__CLK_OR_COUNT_SEL: u32 = 0x00020000;
pub const MCLK_MISC__EN_MCLK_TRISTATE_IN_SUSPEND_MASK: u32 = 0x00040000;
pub const MCLK_MISC__EN_MCLK_TRISTATE_IN_SUSPEND: u32 = 0x00040000;
pub const MCLK_MISC__CGM_SPARE_RD_MASK: u32 = 0x00300000;
pub const MCLK_MISC__CGM_SPARE_A_RD_MASK: u32 = 0x00c00000;
pub const MCLK_MISC__TCLK_TO_YCLKB_EN_MASK: u32 = 0x01000000;
pub const MCLK_MISC__TCLK_TO_YCLKB_EN: u32 = 0x01000000;
pub const MCLK_MISC__CGM_SPARE_A_MASK: u32 = 0x0e000000;

// VCLK_ECP_CNTL
pub const VCLK_ECP_CNTL__VCLK_SRC_SEL_MASK: u32 = 0x00000003;
pub const VCLK_ECP_CNTL__VCLK_INVERT: u32 = 0x00000010;
pub const VCLK_ECP_CNTL__PIXCLK_SRC_INVERT: u32 = 0x00000020;
pub const VCLK_ECP_CNTL__PIXCLK_ALWAYS_ONb: u32 = 0x00000040;
pub const VCLK_ECP_CNTL__PIXCLK_DAC_ALWAYS_ONb: u32 = 0x00000080;
pub const VCLK_ECP_CNTL__ECP_DIV_MASK: u32 = 0x00000300;
pub const VCLK_ECP_CNTL__ECP_FORCE_ON: u32 = 0x00040000;
pub const VCLK_ECP_CNTL__SUBCLK_FORCE_ON: u32 = 0x00080000;
pub const VCLK_ECP_CNTL__R300_DISP_DAC_PIXCLK_DAC_BLANK_OFF: u32 = (1<<23);

// PLL_PWRMGT_CNTL
pub const PLL_PWRMGT_CNTL__MPLL_TURNOFF_MASK: u32 = 0x00000001;
pub const PLL_PWRMGT_CNTL__MPLL_TURNOFF: u32 = 0x00000001;
pub const PLL_PWRMGT_CNTL__SPLL_TURNOFF_MASK: u32 = 0x00000002;
pub const PLL_PWRMGT_CNTL__SPLL_TURNOFF: u32 = 0x00000002;
pub const PLL_PWRMGT_CNTL__PPLL_TURNOFF_MASK: u32 = 0x00000004;
pub const PLL_PWRMGT_CNTL__PPLL_TURNOFF: u32 = 0x00000004;
pub const PLL_PWRMGT_CNTL__P2PLL_TURNOFF_MASK: u32 = 0x00000008;
pub const PLL_PWRMGT_CNTL__P2PLL_TURNOFF: u32 = 0x00000008;
pub const PLL_PWRMGT_CNTL__TVPLL_TURNOFF_MASK: u32 = 0x00000010;
pub const PLL_PWRMGT_CNTL__TVPLL_TURNOFF: u32 = 0x00000010;
pub const PLL_PWRMGT_CNTL__AGPCLK_DYN_STOP_LAT_MASK: u32 = 0x000001e0;
pub const PLL_PWRMGT_CNTL__APM_POWER_STATE_MASK: u32 = 0x00000600;
pub const PLL_PWRMGT_CNTL__APM_PWRSTATE_RD_MASK: u32 = 0x00001800;
pub const PLL_PWRMGT_CNTL__PM_MODE_SEL_MASK: u32 = 0x00002000;
pub const PLL_PWRMGT_CNTL__PM_MODE_SEL: u32 = 0x00002000;
pub const PLL_PWRMGT_CNTL__EN_PWRSEQ_DONE_COND_MASK: u32 = 0x00004000;
pub const PLL_PWRMGT_CNTL__EN_PWRSEQ_DONE_COND: u32 = 0x00004000;
pub const PLL_PWRMGT_CNTL__EN_DISP_PARKED_COND_MASK: u32 = 0x00008000;
pub const PLL_PWRMGT_CNTL__EN_DISP_PARKED_COND: u32 = 0x00008000;
pub const PLL_PWRMGT_CNTL__MOBILE_SU_MASK: u32 = 0x00010000;
pub const PLL_PWRMGT_CNTL__MOBILE_SU: u32 = 0x00010000;
pub const PLL_PWRMGT_CNTL__SU_SCLK_USE_BCLK_MASK: u32 = 0x00020000;
pub const PLL_PWRMGT_CNTL__SU_SCLK_USE_BCLK: u32 = 0x00020000;
pub const PLL_PWRMGT_CNTL__SU_MCLK_USE_BCLK_MASK: u32 = 0x00040000;
pub const PLL_PWRMGT_CNTL__SU_MCLK_USE_BCLK: u32 = 0x00040000;
pub const PLL_PWRMGT_CNTL__SU_SUSTAIN_DISABLE_MASK: u32 = 0x00080000;
pub const PLL_PWRMGT_CNTL__SU_SUSTAIN_DISABLE: u32 = 0x00080000;
pub const PLL_PWRMGT_CNTL__TCL_BYPASS_DISABLE_MASK: u32 = 0x00100000;
pub const PLL_PWRMGT_CNTL__TCL_BYPASS_DISABLE: u32 = 0x00100000;
pub const PLL_PWRMGT_CNTL__TCL_CLOCK_CTIVE_RD_MASK: u32 = 0x00200000;
pub const PLL_PWRMGT_CNTL__TCL_CLOCK_ACTIVE_RD: u32 = 0x00200000;
pub const PLL_PWRMGT_CNTL__CG_NO2_DEBUG_MASK: u32 = 0xff000000;

// CLK_PWRMGT_CNTL
pub const CLK_PWRMGT_CNTL__MPLL_PWRMGT_OFF_MASK: u32 = 0x00000001;
pub const CLK_PWRMGT_CNTL__MPLL_PWRMGT_OFF: u32 = 0x00000001;
pub const CLK_PWRMGT_CNTL__SPLL_PWRMGT_OFF_MASK: u32 = 0x00000002;
pub const CLK_PWRMGT_CNTL__SPLL_PWRMGT_OFF: u32 = 0x00000002;
pub const CLK_PWRMGT_CNTL__PPLL_PWRMGT_OFF_MASK: u32 = 0x00000004;
pub const CLK_PWRMGT_CNTL__PPLL_PWRMGT_OFF: u32 = 0x00000004;
pub const CLK_PWRMGT_CNTL__P2PLL_PWRMGT_OFF_MASK: u32 = 0x00000008;
pub const CLK_PWRMGT_CNTL__P2PLL_PWRMGT_OFF: u32 = 0x00000008;
pub const CLK_PWRMGT_CNTL__MCLK_TURNOFF_MASK: u32 = 0x00000010;
pub const CLK_PWRMGT_CNTL__MCLK_TURNOFF: u32 = 0x00000010;
pub const CLK_PWRMGT_CNTL__SCLK_TURNOFF_MASK: u32 = 0x00000020;
pub const CLK_PWRMGT_CNTL__SCLK_TURNOFF: u32 = 0x00000020;
pub const CLK_PWRMGT_CNTL__PCLK_TURNOFF_MASK: u32 = 0x00000040;
pub const CLK_PWRMGT_CNTL__PCLK_TURNOFF: u32 = 0x00000040;
pub const CLK_PWRMGT_CNTL__P2CLK_TURNOFF_MASK: u32 = 0x00000080;
pub const CLK_PWRMGT_CNTL__P2CLK_TURNOFF: u32 = 0x00000080;
pub const CLK_PWRMGT_CNTL__MC_CH_MODE_MASK: u32 = 0x00000100;
pub const CLK_PWRMGT_CNTL__MC_CH_MODE: u32 = 0x00000100;
pub const CLK_PWRMGT_CNTL__TEST_MODE_MASK: u32 = 0x00000200;
pub const CLK_PWRMGT_CNTL__TEST_MODE: u32 = 0x00000200;
pub const CLK_PWRMGT_CNTL__GLOBAL_PMAN_EN_MASK: u32 = 0x00000400;
pub const CLK_PWRMGT_CNTL__GLOBAL_PMAN_EN: u32 = 0x00000400;
pub const CLK_PWRMGT_CNTL__ENGINE_DYNCLK_MODE_MASK: u32 = 0x00001000;
pub const CLK_PWRMGT_CNTL__ENGINE_DYNCLK_MODE: u32 = 0x00001000;
pub const CLK_PWRMGT_CNTL__ACTIVE_HILO_LAT_MASK: u32 = 0x00006000;
pub const CLK_PWRMGT_CNTL__DISP_DYN_STOP_LAT_MASK: u32 = 0x00008000;
pub const CLK_PWRMGT_CNTL__DISP_DYN_STOP_LAT: u32 = 0x00008000;
pub const CLK_PWRMGT_CNTL__MC_BUSY_MASK: u32 = 0x00010000;
pub const CLK_PWRMGT_CNTL__MC_BUSY: u32 = 0x00010000;
pub const CLK_PWRMGT_CNTL__MC_INT_CNTL_MASK: u32 = 0x00020000;
pub const CLK_PWRMGT_CNTL__MC_INT_CNTL: u32 = 0x00020000;
pub const CLK_PWRMGT_CNTL__MC_SWITCH_MASK: u32 = 0x00040000;
pub const CLK_PWRMGT_CNTL__MC_SWITCH: u32 = 0x00040000;
pub const CLK_PWRMGT_CNTL__DLL_READY_MASK: u32 = 0x00080000;
pub const CLK_PWRMGT_CNTL__DLL_READY: u32 = 0x00080000;
pub const CLK_PWRMGT_CNTL__DISP_PM_MASK: u32 = 0x00100000;
pub const CLK_PWRMGT_CNTL__DISP_PM: u32 = 0x00100000;
pub const CLK_PWRMGT_CNTL__DYN_STOP_MODE_MASK: u32 = 0x00e00000;
pub const CLK_PWRMGT_CNTL__CG_NO1_DEBUG_MASK: u32 = 0x3f000000;
pub const CLK_PWRMGT_CNTL__TVPLL_PWRMGT_OFF_MASK: u32 = 0x40000000;
pub const CLK_PWRMGT_CNTL__TVPLL_PWRMGT_OFF: u32 = 0x40000000;
pub const CLK_PWRMGT_CNTL__TVCLK_TURNOFF_MASK: u32 = 0x80000000;
pub const CLK_PWRMGT_CNTL__TVCLK_TURNOFF: u32 = 0x80000000;

// BUS_CNTL1
pub const BUS_CNTL1__PMI_IO_DISABLE_MASK: u32 = 0x00000001;
pub const BUS_CNTL1__PMI_IO_DISABLE: u32 = 0x00000001;
pub const BUS_CNTL1__PMI_MEM_DISABLE_MASK: u32 = 0x00000002;
pub const BUS_CNTL1__PMI_MEM_DISABLE: u32 = 0x00000002;
pub const BUS_CNTL1__PMI_BM_DISABLE_MASK: u32 = 0x00000004;
pub const BUS_CNTL1__PMI_BM_DISABLE: u32 = 0x00000004;
pub const BUS_CNTL1__PMI_INT_DISABLE_MASK: u32 = 0x00000008;
pub const BUS_CNTL1__PMI_INT_DISABLE: u32 = 0x00000008;
pub const BUS_CNTL1__BUS2_IMMEDIATE_PMI_DISABLE_MASK: u32 = 0x00000020;
pub const BUS_CNTL1__BUS2_IMMEDIATE_PMI_DISABLE: u32 = 0x00000020;
pub const BUS_CNTL1__BUS2_VGA_REG_COHERENCY_DIS_MASK: u32 = 0x00000100;
pub const BUS_CNTL1__BUS2_VGA_REG_COHERENCY_DIS: u32 = 0x00000100;
pub const BUS_CNTL1__BUS2_VGA_MEM_COHERENCY_DIS_MASK: u32 = 0x00000200;
pub const BUS_CNTL1__BUS2_VGA_MEM_COHERENCY_DIS: u32 = 0x00000200;
pub const BUS_CNTL1__BUS2_HDP_REG_COHERENCY_DIS_MASK: u32 = 0x00000400;
pub const BUS_CNTL1__BUS2_HDP_REG_COHERENCY_DIS: u32 = 0x00000400;
pub const BUS_CNTL1__BUS2_GUI_INITIATOR_COHERENCY_DIS_MASK: u32 = 0x00000800;
pub const BUS_CNTL1__BUS2_GUI_INITIATOR_COHERENCY_DIS: u32 = 0x00000800;
pub const BUS_CNTL1__MOBILE_PLATFORM_SEL_MASK: u32 = 0x0c000000;
pub const BUS_CNTL1__SEND_SBA_LATENCY_MASK: u32 = 0x70000000;
pub const BUS_CNTL1__AGPCLK_VALID_MASK: u32 = 0x80000000;
pub const BUS_CNTL1__AGPCLK_VALID: u32 = 0x80000000;

// BUS_CNTL1
pub const BUS_CNTL1__PMI_IO_DISABLE__SHIFT: u32 = 0x00000000;
pub const BUS_CNTL1__PMI_MEM_DISABLE__SHIFT: u32 = 0x00000001;
pub const BUS_CNTL1__PMI_BM_DISABLE__SHIFT: u32 = 0x00000002;
pub const BUS_CNTL1__PMI_INT_DISABLE__SHIFT: u32 = 0x00000003;
pub const BUS_CNTL1__BUS2_IMMEDIATE_PMI_DISABLE__SHIFT: u32 = 0x00000005;
pub const BUS_CNTL1__BUS2_VGA_REG_COHERENCY_DIS__SHIFT: u32 = 0x00000008;
pub const BUS_CNTL1__BUS2_VGA_MEM_COHERENCY_DIS__SHIFT: u32 = 0x00000009;
pub const BUS_CNTL1__BUS2_HDP_REG_COHERENCY_DIS__SHIFT: u32 = 0x0000000a;
pub const BUS_CNTL1__BUS2_GUI_INITIATOR_COHERENCY_DIS__SHIFT: u32 = 0x0000000b;
pub const BUS_CNTL1__MOBILE_PLATFORM_SEL__SHIFT: u32 = 0x0000001a;
pub const BUS_CNTL1__SEND_SBA_LATENCY__SHIFT: u32 = 0x0000001c;
pub const BUS_CNTL1__AGPCLK_VALID__SHIFT: u32 = 0x0000001f;

// CRTC_OFFSET_CNTL
pub const CRTC_OFFSET_CNTL__CRTC_TILE_LINE_MASK: u32 = 0x0000000f;
pub const CRTC_OFFSET_CNTL__CRTC_TILE_LINE_RIGHT_MASK: u32 = 0x000000f0;
pub const CRTC_OFFSET_CNTL__CRTC_TILE_EN_RIGHT_MASK: u32 = 0x00004000;
pub const CRTC_OFFSET_CNTL__CRTC_TILE_EN_RIGHT: u32 = 0x00004000;
pub const CRTC_OFFSET_CNTL__CRTC_TILE_EN_MASK: u32 = 0x00008000;
pub const CRTC_OFFSET_CNTL__CRTC_TILE_EN: u32 = 0x00008000;
pub const CRTC_OFFSET_CNTL__CRTC_OFFSET_FLIP_CNTL_MASK: u32 = 0x00010000;
pub const CRTC_OFFSET_CNTL__CRTC_OFFSET_FLIP_CNTL: u32 = 0x00010000;
pub const CRTC_OFFSET_CNTL__CRTC_STEREO_OFFSET_EN_MASK: u32 = 0x00020000;
pub const CRTC_OFFSET_CNTL__CRTC_STEREO_OFFSET_EN: u32 = 0x00020000;
pub const CRTC_OFFSET_CNTL__CRTC_STEREO_SYNC_EN_MASK: u32 = 0x000c0000;
pub const CRTC_OFFSET_CNTL__CRTC_STEREO_SYNC_OUT_EN_MASK: u32 = 0x00100000;
pub const CRTC_OFFSET_CNTL__CRTC_STEREO_SYNC_OUT_EN: u32 = 0x00100000;
pub const CRTC_OFFSET_CNTL__CRTC_STEREO_SYNC_MASK: u32 = 0x00200000;
pub const CRTC_OFFSET_CNTL__CRTC_STEREO_SYNC: u32 = 0x00200000;
pub const CRTC_OFFSET_CNTL__CRTC_GUI_TRIG_OFFSET_LEFT_EN_MASK: u32 = 0x10000000;
pub const CRTC_OFFSET_CNTL__CRTC_GUI_TRIG_OFFSET_LEFT_EN: u32 = 0x10000000;
pub const CRTC_OFFSET_CNTL__CRTC_GUI_TRIG_OFFSET_RIGHT_EN_MASK: u32 = 0x20000000;
pub const CRTC_OFFSET_CNTL__CRTC_GUI_TRIG_OFFSET_RIGHT_EN: u32 = 0x20000000;
pub const CRTC_OFFSET_CNTL__CRTC_GUI_TRIG_OFFSET_MASK: u32 = 0x40000000;
pub const CRTC_OFFSET_CNTL__CRTC_GUI_TRIG_OFFSET: u32 = 0x40000000;
pub const CRTC_OFFSET_CNTL__CRTC_OFFSET_LOCK_MASK: u32 = 0x80000000;
pub const CRTC_OFFSET_CNTL__CRTC_OFFSET_LOCK: u32 = 0x80000000;

// CRTC_GEN_CNTL
pub const CRTC_GEN_CNTL__CRTC_DBL_SCAN_EN_MASK: u32 = 0x00000001;
pub const CRTC_GEN_CNTL__CRTC_DBL_SCAN_EN: u32 = 0x00000001;
pub const CRTC_GEN_CNTL__CRTC_INTERLACE_EN_MASK: u32 = 0x00000002;
pub const CRTC_GEN_CNTL__CRTC_INTERLACE_EN: u32 = 0x00000002;
pub const CRTC_GEN_CNTL__CRTC_C_SYNC_EN_MASK: u32 = 0x00000010;
pub const CRTC_GEN_CNTL__CRTC_C_SYNC_EN: u32 = 0x00000010;
pub const CRTC_GEN_CNTL__CRTC_PIX_WIDTH_MASK: u32 = 0x00000f00;
pub const CRTC_GEN_CNTL__CRTC_ICON_EN_MASK: u32 = 0x00008000;
pub const CRTC_GEN_CNTL__CRTC_ICON_EN: u32 = 0x00008000;
pub const CRTC_GEN_CNTL__CRTC_CUR_EN_MASK: u32 = 0x00010000;
pub const CRTC_GEN_CNTL__CRTC_CUR_EN: u32 = 0x00010000;
pub const CRTC_GEN_CNTL__CRTC_VSTAT_MODE_MASK: u32 = 0x00060000;
pub const CRTC_GEN_CNTL__CRTC_CUR_MODE_MASK: u32 = 0x00700000;
pub const CRTC_GEN_CNTL__CRTC_EXT_DISP_EN_MASK: u32 = 0x01000000;
pub const CRTC_GEN_CNTL__CRTC_EXT_DISP_EN: u32 = 0x01000000;
pub const CRTC_GEN_CNTL__CRTC_EN_MASK: u32 = 0x02000000;
pub const CRTC_GEN_CNTL__CRTC_EN: u32 = 0x02000000;
pub const CRTC_GEN_CNTL__CRTC_DISP_REQ_EN_B_MASK: u32 = 0x04000000;
pub const CRTC_GEN_CNTL__CRTC_DISP_REQ_EN_B: u32 = 0x04000000;

// CRTC2_GEN_CNTL
pub const CRTC2_GEN_CNTL__CRTC2_DBL_SCAN_EN_MASK: u32 = 0x00000001;
pub const CRTC2_GEN_CNTL__CRTC2_DBL_SCAN_EN: u32 = 0x00000001;
pub const CRTC2_GEN_CNTL__CRTC2_INTERLACE_EN_MASK: u32 = 0x00000002;
pub const CRTC2_GEN_CNTL__CRTC2_INTERLACE_EN: u32 = 0x00000002;
pub const CRTC2_GEN_CNTL__CRTC2_SYNC_TRISTATE_MASK: u32 = 0x00000010;
pub const CRTC2_GEN_CNTL__CRTC2_SYNC_TRISTATE: u32 = 0x00000010;
pub const CRTC2_GEN_CNTL__CRTC2_HSYNC_TRISTATE_MASK: u32 = 0x00000020;
pub const CRTC2_GEN_CNTL__CRTC2_HSYNC_TRISTATE: u32 = 0x00000020;
pub const CRTC2_GEN_CNTL__CRTC2_VSYNC_TRISTATE_MASK: u32 = 0x00000040;
pub const CRTC2_GEN_CNTL__CRTC2_VSYNC_TRISTATE: u32 = 0x00000040;
pub const CRTC2_GEN_CNTL__CRT2_ON_MASK: u32 = 0x00000080;
pub const CRTC2_GEN_CNTL__CRT2_ON: u32 = 0x00000080;
pub const CRTC2_GEN_CNTL__CRTC2_PIX_WIDTH_MASK: u32 = 0x00000f00;
pub const CRTC2_GEN_CNTL__CRTC2_ICON_EN_MASK: u32 = 0x00008000;
pub const CRTC2_GEN_CNTL__CRTC2_ICON_EN: u32 = 0x00008000;
pub const CRTC2_GEN_CNTL__CRTC2_CUR_EN_MASK: u32 = 0x00010000;
pub const CRTC2_GEN_CNTL__CRTC2_CUR_EN: u32 = 0x00010000;
pub const CRTC2_GEN_CNTL__CRTC2_CUR_MODE_MASK: u32 = 0x00700000;
pub const CRTC2_GEN_CNTL__CRTC2_DISPLAY_DIS_MASK: u32 = 0x00800000;
pub const CRTC2_GEN_CNTL__CRTC2_DISPLAY_DIS: u32 = 0x00800000;
pub const CRTC2_GEN_CNTL__CRTC2_EN_MASK: u32 = 0x02000000;
pub const CRTC2_GEN_CNTL__CRTC2_EN: u32 = 0x02000000;
pub const CRTC2_GEN_CNTL__CRTC2_DISP_REQ_EN_B_MASK: u32 = 0x04000000;
pub const CRTC2_GEN_CNTL__CRTC2_DISP_REQ_EN_B: u32 = 0x04000000;
pub const CRTC2_GEN_CNTL__CRTC2_C_SYNC_EN_MASK: u32 = 0x08000000;
pub const CRTC2_GEN_CNTL__CRTC2_C_SYNC_EN: u32 = 0x08000000;
pub const CRTC2_GEN_CNTL__CRTC2_HSYNC_DIS_MASK: u32 = 0x10000000;
pub const CRTC2_GEN_CNTL__CRTC2_HSYNC_DIS: u32 = 0x10000000;
pub const CRTC2_GEN_CNTL__CRTC2_VSYNC_DIS_MASK: u32 = 0x20000000;
pub const CRTC2_GEN_CNTL__CRTC2_VSYNC_DIS: u32 = 0x20000000;

// AGP_CNTL
pub const AGP_CNTL__MAX_IDLE_CLK_MASK: u32 = 0x000000ff;
pub const AGP_CNTL__HOLD_RD_FIFO_MASK: u32 = 0x00000100;
pub const AGP_CNTL__HOLD_RD_FIFO: u32 = 0x00000100;
pub const AGP_CNTL__HOLD_RQ_FIFO_MASK: u32 = 0x00000200;
pub const AGP_CNTL__HOLD_RQ_FIFO: u32 = 0x00000200;
pub const AGP_CNTL__EN_2X_STBB_MASK: u32 = 0x00000400;
pub const AGP_CNTL__EN_2X_STBB: u32 = 0x00000400;
pub const AGP_CNTL__FORCE_FULL_SBA_MASK: u32 = 0x00000800;
pub const AGP_CNTL__FORCE_FULL_SBA: u32 = 0x00000800;
pub const AGP_CNTL__SBA_DIS_MASK: u32 = 0x00001000;
pub const AGP_CNTL__SBA_DIS: u32 = 0x00001000;
pub const AGP_CNTL__AGP_REV_ID_MASK: u32 = 0x00002000;
pub const AGP_CNTL__AGP_REV_ID: u32 = 0x00002000;
pub const AGP_CNTL__REG_CRIPPLE_AGP4X_MASK: u32 = 0x00004000;
pub const AGP_CNTL__REG_CRIPPLE_AGP4X: u32 = 0x00004000;
pub const AGP_CNTL__REG_CRIPPLE_AGP2X4X_MASK: u32 = 0x00008000;
pub const AGP_CNTL__REG_CRIPPLE_AGP2X4X: u32 = 0x00008000;
pub const AGP_CNTL__FORCE_INT_VREF_MASK: u32 = 0x00010000;
pub const AGP_CNTL__FORCE_INT_VREF: u32 = 0x00010000;
pub const AGP_CNTL__PENDING_SLOTS_VAL_MASK: u32 = 0x00060000;
pub const AGP_CNTL__PENDING_SLOTS_SEL_MASK: u32 = 0x00080000;
pub const AGP_CNTL__PENDING_SLOTS_SEL: u32 = 0x00080000;
pub const AGP_CNTL__EN_EXTENDED_AD_STB_2X_MASK: u32 = 0x00100000;
pub const AGP_CNTL__EN_EXTENDED_AD_STB_2X: u32 = 0x00100000;
pub const AGP_CNTL__DIS_QUEUED_GNT_FIX_MASK: u32 = 0x00200000;
pub const AGP_CNTL__DIS_QUEUED_GNT_FIX: u32 = 0x00200000;
pub const AGP_CNTL__EN_RDATA2X4X_MULTIRESET_MASK: u32 = 0x00400000;
pub const AGP_CNTL__EN_RDATA2X4X_MULTIRESET: u32 = 0x00400000;
pub const AGP_CNTL__EN_RBFCALM_MASK: u32 = 0x00800000;
pub const AGP_CNTL__EN_RBFCALM: u32 = 0x00800000;
pub const AGP_CNTL__FORCE_EXT_VREF_MASK: u32 = 0x01000000;
pub const AGP_CNTL__FORCE_EXT_VREF: u32 = 0x01000000;
pub const AGP_CNTL__DIS_RBF_MASK: u32 = 0x02000000;
pub const AGP_CNTL__DIS_RBF: u32 = 0x02000000;
pub const AGP_CNTL__DELAY_FIRST_SBA_EN_MASK: u32 = 0x04000000;
pub const AGP_CNTL__DELAY_FIRST_SBA_EN: u32 = 0x04000000;
pub const AGP_CNTL__DELAY_FIRST_SBA_VAL_MASK: u32 = 0x38000000;
pub const AGP_CNTL__AGP_MISC_MASK: u32 = 0xc0000000;

// AGP_CNTL
pub const AGP_CNTL__MAX_IDLE_CLK__SHIFT: u32 = 0x00000000;
pub const AGP_CNTL__HOLD_RD_FIFO__SHIFT: u32 = 0x00000008;
pub const AGP_CNTL__HOLD_RQ_FIFO__SHIFT: u32 = 0x00000009;
pub const AGP_CNTL__EN_2X_STBB__SHIFT: u32 = 0x0000000a;
pub const AGP_CNTL__FORCE_FULL_SBA__SHIFT: u32 = 0x0000000b;
pub const AGP_CNTL__SBA_DIS__SHIFT: u32 = 0x0000000c;
pub const AGP_CNTL__AGP_REV_ID__SHIFT: u32 = 0x0000000d;
pub const AGP_CNTL__REG_CRIPPLE_AGP4X__SHIFT: u32 = 0x0000000e;
pub const AGP_CNTL__REG_CRIPPLE_AGP2X4X__SHIFT: u32 = 0x0000000f;
pub const AGP_CNTL__FORCE_INT_VREF__SHIFT: u32 = 0x00000010;
pub const AGP_CNTL__PENDING_SLOTS_VAL__SHIFT: u32 = 0x00000011;
pub const AGP_CNTL__PENDING_SLOTS_SEL__SHIFT: u32 = 0x00000013;
pub const AGP_CNTL__EN_EXTENDED_AD_STB_2X__SHIFT: u32 = 0x00000014;
pub const AGP_CNTL__DIS_QUEUED_GNT_FIX__SHIFT: u32 = 0x00000015;
pub const AGP_CNTL__EN_RDATA2X4X_MULTIRESET__SHIFT: u32 = 0x00000016;
pub const AGP_CNTL__EN_RBFCALM__SHIFT: u32 = 0x00000017;
pub const AGP_CNTL__FORCE_EXT_VREF__SHIFT: u32 = 0x00000018;
pub const AGP_CNTL__DIS_RBF__SHIFT: u32 = 0x00000019;
pub const AGP_CNTL__DELAY_FIRST_SBA_EN__SHIFT: u32 = 0x0000001a;
pub const AGP_CNTL__DELAY_FIRST_SBA_VAL__SHIFT: u32 = 0x0000001b;
pub const AGP_CNTL__AGP_MISC__SHIFT: u32 = 0x0000001e;

// DISP_MISC_CNTL
pub const DISP_MISC_CNTL__SOFT_RESET_GRPH_PP_MASK: u32 = 0x00000001;
pub const DISP_MISC_CNTL__SOFT_RESET_GRPH_PP: u32 = 0x00000001;
pub const DISP_MISC_CNTL__SOFT_RESET_SUBPIC_PP_MASK: u32 = 0x00000002;
pub const DISP_MISC_CNTL__SOFT_RESET_SUBPIC_PP: u32 = 0x00000002;
pub const DISP_MISC_CNTL__SOFT_RESET_OV0_PP_MASK: u32 = 0x00000004;
pub const DISP_MISC_CNTL__SOFT_RESET_OV0_PP: u32 = 0x00000004;
pub const DISP_MISC_CNTL__SOFT_RESET_GRPH_SCLK_MASK: u32 = 0x00000010;
pub const DISP_MISC_CNTL__SOFT_RESET_GRPH_SCLK: u32 = 0x00000010;
pub const DISP_MISC_CNTL__SOFT_RESET_SUBPIC_SCLK_MASK: u32 = 0x00000020;
pub const DISP_MISC_CNTL__SOFT_RESET_SUBPIC_SCLK: u32 = 0x00000020;
pub const DISP_MISC_CNTL__SOFT_RESET_OV0_SCLK_MASK: u32 = 0x00000040;
pub const DISP_MISC_CNTL__SOFT_RESET_OV0_SCLK: u32 = 0x00000040;
pub const DISP_MISC_CNTL__SYNC_STRENGTH_MASK: u32 = 0x00000300;
pub const DISP_MISC_CNTL__SYNC_PAD_FLOP_EN_MASK: u32 = 0x00000400;
pub const DISP_MISC_CNTL__SYNC_PAD_FLOP_EN: u32 = 0x00000400;
pub const DISP_MISC_CNTL__SOFT_RESET_GRPH2_PP_MASK: u32 = 0x00001000;
pub const DISP_MISC_CNTL__SOFT_RESET_GRPH2_PP: u32 = 0x00001000;
pub const DISP_MISC_CNTL__SOFT_RESET_GRPH2_SCLK_MASK: u32 = 0x00008000;
pub const DISP_MISC_CNTL__SOFT_RESET_GRPH2_SCLK: u32 = 0x00008000;
pub const DISP_MISC_CNTL__SOFT_RESET_LVDS_MASK: u32 = 0x00010000;
pub const DISP_MISC_CNTL__SOFT_RESET_LVDS: u32 = 0x00010000;
pub const DISP_MISC_CNTL__SOFT_RESET_TMDS_MASK: u32 = 0x00020000;
pub const DISP_MISC_CNTL__SOFT_RESET_TMDS: u32 = 0x00020000;
pub const DISP_MISC_CNTL__SOFT_RESET_DIG_TMDS_MASK: u32 = 0x00040000;
pub const DISP_MISC_CNTL__SOFT_RESET_DIG_TMDS: u32 = 0x00040000;
pub const DISP_MISC_CNTL__SOFT_RESET_TV_MASK: u32 = 0x00080000;
pub const DISP_MISC_CNTL__SOFT_RESET_TV: u32 = 0x00080000;
pub const DISP_MISC_CNTL__PALETTE2_MEM_RD_MARGIN_MASK: u32 = 0x00f00000;
pub const DISP_MISC_CNTL__PALETTE_MEM_RD_MARGIN_MASK: u32 = 0x0f000000;
pub const DISP_MISC_CNTL__RMX_BUF_MEM_RD_MARGIN_MASK: u32 = 0xf0000000;

// DISP_PWR_MAN
pub const DISP_PWR_MAN__DISP_PWR_MAN_D3_CRTC_EN_MASK: u32 = 0x00000001;
pub const DISP_PWR_MAN__DISP_PWR_MAN_D3_CRTC_EN: u32 = 0x00000001;
pub const DISP_PWR_MAN__DISP2_PWR_MAN_D3_CRTC2_EN_MASK: u32 = 0x00000010;
pub const DISP_PWR_MAN__DISP2_PWR_MAN_D3_CRTC2_EN: u32 = 0x00000010;
pub const DISP_PWR_MAN__DISP_PWR_MAN_DPMS_MASK: u32 = 0x00000300;
pub const DISP_PWR_MAN__DISP_D3_RST_MASK: u32 = 0x00010000;
pub const DISP_PWR_MAN__DISP_D3_RST: u32 = 0x00010000;
pub const DISP_PWR_MAN__DISP_D3_REG_RST_MASK: u32 = 0x00020000;
pub const DISP_PWR_MAN__DISP_D3_REG_RST: u32 = 0x00020000;
pub const DISP_PWR_MAN__DISP_D3_GRPH_RST_MASK: u32 = 0x00040000;
pub const DISP_PWR_MAN__DISP_D3_GRPH_RST: u32 = 0x00040000;
pub const DISP_PWR_MAN__DISP_D3_SUBPIC_RST_MASK: u32 = 0x00080000;
pub const DISP_PWR_MAN__DISP_D3_SUBPIC_RST: u32 = 0x00080000;
pub const DISP_PWR_MAN__DISP_D3_OV0_RST_MASK: u32 = 0x00100000;
pub const DISP_PWR_MAN__DISP_D3_OV0_RST: u32 = 0x00100000;
pub const DISP_PWR_MAN__DISP_D1D2_GRPH_RST_MASK: u32 = 0x00200000;
pub const DISP_PWR_MAN__DISP_D1D2_GRPH_RST: u32 = 0x00200000;
pub const DISP_PWR_MAN__DISP_D1D2_SUBPIC_RST_MASK: u32 = 0x00400000;
pub const DISP_PWR_MAN__DISP_D1D2_SUBPIC_RST: u32 = 0x00400000;
pub const DISP_PWR_MAN__DISP_D1D2_OV0_RST_MASK: u32 = 0x00800000;
pub const DISP_PWR_MAN__DISP_D1D2_OV0_RST: u32 = 0x00800000;
pub const DISP_PWR_MAN__DIG_TMDS_ENABLE_RST_MASK: u32 = 0x01000000;
pub const DISP_PWR_MAN__DIG_TMDS_ENABLE_RST: u32 = 0x01000000;
pub const DISP_PWR_MAN__TV_ENABLE_RST_MASK: u32 = 0x02000000;
pub const DISP_PWR_MAN__TV_ENABLE_RST: u32 = 0x02000000;
pub const DISP_PWR_MAN__AUTO_PWRUP_EN_MASK: u32 = 0x04000000;
pub const DISP_PWR_MAN__AUTO_PWRUP_EN: u32 = 0x04000000;

// MC_IND_INDEX
pub const MC_IND_INDEX__MC_IND_ADDR_MASK: u32 = 0x0000001f;
pub const MC_IND_INDEX__MC_IND_WR_EN_MASK: u32 = 0x00000100;
pub const MC_IND_INDEX__MC_IND_WR_EN: u32 = 0x00000100;

// MC_IND_DATA
pub const MC_IND_DATA__MC_IND_DATA_MASK: u32 = 0xffffffff;

// MC_CHP_IO_CNTL_A1
pub const MC_CHP_IO_CNTL_A1__MEM_SLEWN_CKA__SHIFT: u32 = 0x00000000;
pub const MC_CHP_IO_CNTL_A1__MEM_SLEWN_AA__SHIFT: u32 = 0x00000001;
pub const MC_CHP_IO_CNTL_A1__MEM_SLEWN_DQMA__SHIFT: u32 = 0x00000002;
pub const MC_CHP_IO_CNTL_A1__MEM_SLEWN_DQSA__SHIFT: u32 = 0x00000003;
pub const MC_CHP_IO_CNTL_A1__MEM_SLEWP_CKA__SHIFT: u32 = 0x00000004;
pub const MC_CHP_IO_CNTL_A1__MEM_SLEWP_AA__SHIFT: u32 = 0x00000005;
pub const MC_CHP_IO_CNTL_A1__MEM_SLEWP_DQMA__SHIFT: u32 = 0x00000006;
pub const MC_CHP_IO_CNTL_A1__MEM_SLEWP_DQSA__SHIFT: u32 = 0x00000007;
pub const MC_CHP_IO_CNTL_A1__MEM_PREAMP_AA__SHIFT: u32 = 0x00000008;
pub const MC_CHP_IO_CNTL_A1__MEM_PREAMP_DQMA__SHIFT: u32 = 0x00000009;
pub const MC_CHP_IO_CNTL_A1__MEM_PREAMP_DQSA__SHIFT: u32 = 0x0000000a;
pub const MC_CHP_IO_CNTL_A1__MEM_IO_MODEA__SHIFT: u32 = 0x0000000c;
pub const MC_CHP_IO_CNTL_A1__MEM_REC_CKA__SHIFT: u32 = 0x0000000e;
pub const MC_CHP_IO_CNTL_A1__MEM_REC_AA__SHIFT: u32 = 0x00000010;
pub const MC_CHP_IO_CNTL_A1__MEM_REC_DQMA__SHIFT: u32 = 0x00000012;
pub const MC_CHP_IO_CNTL_A1__MEM_REC_DQSA__SHIFT: u32 = 0x00000014;
pub const MC_CHP_IO_CNTL_A1__MEM_SYNC_PHASEA__SHIFT: u32 = 0x00000016;
pub const MC_CHP_IO_CNTL_A1__MEM_SYNC_CENTERA__SHIFT: u32 = 0x00000017;
pub const MC_CHP_IO_CNTL_A1__MEM_SYNC_ENA__SHIFT: u32 = 0x00000018;
pub const MC_CHP_IO_CNTL_A1__MEM_CLK_SELA__SHIFT: u32 = 0x0000001a;
pub const MC_CHP_IO_CNTL_A1__MEM_CLK_INVA__SHIFT: u32 = 0x0000001c;
pub const MC_CHP_IO_CNTL_A1__MEM_DATA_ENIMP_A__SHIFT: u32 = 0x0000001e;
pub const MC_CHP_IO_CNTL_A1__MEM_CNTL_ENIMP_A__SHIFT: u32 = 0x0000001f;

// MC_CHP_IO_CNTL_B1
pub const MC_CHP_IO_CNTL_B1__MEM_SLEWN_CKB__SHIFT: u32 = 0x00000000;
pub const MC_CHP_IO_CNTL_B1__MEM_SLEWN_AB__SHIFT: u32 = 0x00000001;
pub const MC_CHP_IO_CNTL_B1__MEM_SLEWN_DQMB__SHIFT: u32 = 0x00000002;
pub const MC_CHP_IO_CNTL_B1__MEM_SLEWN_DQSB__SHIFT: u32 = 0x00000003;
pub const MC_CHP_IO_CNTL_B1__MEM_SLEWP_CKB__SHIFT: u32 = 0x00000004;
pub const MC_CHP_IO_CNTL_B1__MEM_SLEWP_AB__SHIFT: u32 = 0x00000005;
pub const MC_CHP_IO_CNTL_B1__MEM_SLEWP_DQMB__SHIFT: u32 = 0x00000006;
pub const MC_CHP_IO_CNTL_B1__MEM_SLEWP_DQSB__SHIFT: u32 = 0x00000007;
pub const MC_CHP_IO_CNTL_B1__MEM_PREAMP_AB__SHIFT: u32 = 0x00000008;
pub const MC_CHP_IO_CNTL_B1__MEM_PREAMP_DQMB__SHIFT: u32 = 0x00000009;
pub const MC_CHP_IO_CNTL_B1__MEM_PREAMP_DQSB__SHIFT: u32 = 0x0000000a;
pub const MC_CHP_IO_CNTL_B1__MEM_IO_MODEB__SHIFT: u32 = 0x0000000c;
pub const MC_CHP_IO_CNTL_B1__MEM_REC_CKB__SHIFT: u32 = 0x0000000e;
pub const MC_CHP_IO_CNTL_B1__MEM_REC_AB__SHIFT: u32 = 0x00000010;
pub const MC_CHP_IO_CNTL_B1__MEM_REC_DQMB__SHIFT: u32 = 0x00000012;
pub const MC_CHP_IO_CNTL_B1__MEM_REC_DQSB__SHIFT: u32 = 0x00000014;
pub const MC_CHP_IO_CNTL_B1__MEM_SYNC_PHASEB__SHIFT: u32 = 0x00000016;
pub const MC_CHP_IO_CNTL_B1__MEM_SYNC_CENTERB__SHIFT: u32 = 0x00000017;
pub const MC_CHP_IO_CNTL_B1__MEM_SYNC_ENB__SHIFT: u32 = 0x00000018;
pub const MC_CHP_IO_CNTL_B1__MEM_CLK_SELB__SHIFT: u32 = 0x0000001a;
pub const MC_CHP_IO_CNTL_B1__MEM_CLK_INVB__SHIFT: u32 = 0x0000001c;
pub const MC_CHP_IO_CNTL_B1__MEM_DATA_ENIMP_B__SHIFT: u32 = 0x0000001e;
pub const MC_CHP_IO_CNTL_B1__MEM_CNTL_ENIMP_B__SHIFT: u32 = 0x0000001f;

// MC_CHP_IO_CNTL_A1
pub const MC_CHP_IO_CNTL_A1__MEM_SLEWN_CKA_MASK: u32 = 0x00000001;
pub const MC_CHP_IO_CNTL_A1__MEM_SLEWN_CKA: u32 = 0x00000001;
pub const MC_CHP_IO_CNTL_A1__MEM_SLEWN_AA_MASK: u32 = 0x00000002;
pub const MC_CHP_IO_CNTL_A1__MEM_SLEWN_AA: u32 = 0x00000002;
pub const MC_CHP_IO_CNTL_A1__MEM_SLEWN_DQMA_MASK: u32 = 0x00000004;
pub const MC_CHP_IO_CNTL_A1__MEM_SLEWN_DQMA: u32 = 0x00000004;
pub const MC_CHP_IO_CNTL_A1__MEM_SLEWN_DQSA_MASK: u32 = 0x00000008;
pub const MC_CHP_IO_CNTL_A1__MEM_SLEWN_DQSA: u32 = 0x00000008;
pub const MC_CHP_IO_CNTL_A1__MEM_SLEWP_CKA_MASK: u32 = 0x00000010;
pub const MC_CHP_IO_CNTL_A1__MEM_SLEWP_CKA: u32 = 0x00000010;
pub const MC_CHP_IO_CNTL_A1__MEM_SLEWP_AA_MASK: u32 = 0x00000020;
pub const MC_CHP_IO_CNTL_A1__MEM_SLEWP_AA: u32 = 0x00000020;
pub const MC_CHP_IO_CNTL_A1__MEM_SLEWP_DQMA_MASK: u32 = 0x00000040;
pub const MC_CHP_IO_CNTL_A1__MEM_SLEWP_DQMA: u32 = 0x00000040;
pub const MC_CHP_IO_CNTL_A1__MEM_SLEWP_DQSA_MASK: u32 = 0x00000080;
pub const MC_CHP_IO_CNTL_A1__MEM_SLEWP_DQSA: u32 = 0x00000080;
pub const MC_CHP_IO_CNTL_A1__MEM_PREAMP_AA_MASK: u32 = 0x00000100;
pub const MC_CHP_IO_CNTL_A1__MEM_PREAMP_AA: u32 = 0x00000100;
pub const MC_CHP_IO_CNTL_A1__MEM_PREAMP_DQMA_MASK: u32 = 0x00000200;
pub const MC_CHP_IO_CNTL_A1__MEM_PREAMP_DQMA: u32 = 0x00000200;
pub const MC_CHP_IO_CNTL_A1__MEM_PREAMP_DQSA_MASK: u32 = 0x00000400;
pub const MC_CHP_IO_CNTL_A1__MEM_PREAMP_DQSA: u32 = 0x00000400;
pub const MC_CHP_IO_CNTL_A1__MEM_IO_MODEA_MASK: u32 = 0x00003000;
pub const MC_CHP_IO_CNTL_A1__MEM_REC_CKA_MASK: u32 = 0x0000c000;
pub const MC_CHP_IO_CNTL_A1__MEM_REC_AA_MASK: u32 = 0x00030000;
pub const MC_CHP_IO_CNTL_A1__MEM_REC_DQMA_MASK: u32 = 0x000c0000;
pub const MC_CHP_IO_CNTL_A1__MEM_REC_DQSA_MASK: u32 = 0x00300000;
pub const MC_CHP_IO_CNTL_A1__MEM_SYNC_PHASEA_MASK: u32 = 0x00400000;
pub const MC_CHP_IO_CNTL_A1__MEM_SYNC_PHASEA: u32 = 0x00400000;
pub const MC_CHP_IO_CNTL_A1__MEM_SYNC_CENTERA_MASK: u32 = 0x00800000;
pub const MC_CHP_IO_CNTL_A1__MEM_SYNC_CENTERA: u32 = 0x00800000;
pub const MC_CHP_IO_CNTL_A1__MEM_SYNC_ENA_MASK: u32 = 0x03000000;
pub const MC_CHP_IO_CNTL_A1__MEM_CLK_SELA_MASK: u32 = 0x0c000000;
pub const MC_CHP_IO_CNTL_A1__MEM_CLK_INVA_MASK: u32 = 0x10000000;
pub const MC_CHP_IO_CNTL_A1__MEM_CLK_INVA: u32 = 0x10000000;
pub const MC_CHP_IO_CNTL_A1__MEM_DATA_ENIMP_A_MASK: u32 = 0x40000000;
pub const MC_CHP_IO_CNTL_A1__MEM_DATA_ENIMP_A: u32 = 0x40000000;
pub const MC_CHP_IO_CNTL_A1__MEM_CNTL_ENIMP_A_MASK: u32 = 0x80000000;
pub const MC_CHP_IO_CNTL_A1__MEM_CNTL_ENIMP_A: u32 = 0x80000000;

// MC_CHP_IO_CNTL_B1
pub const MC_CHP_IO_CNTL_B1__MEM_SLEWN_CKB_MASK: u32 = 0x00000001;
pub const MC_CHP_IO_CNTL_B1__MEM_SLEWN_CKB: u32 = 0x00000001;
pub const MC_CHP_IO_CNTL_B1__MEM_SLEWN_AB_MASK: u32 = 0x00000002;
pub const MC_CHP_IO_CNTL_B1__MEM_SLEWN_AB: u32 = 0x00000002;
pub const MC_CHP_IO_CNTL_B1__MEM_SLEWN_DQMB_MASK: u32 = 0x00000004;
pub const MC_CHP_IO_CNTL_B1__MEM_SLEWN_DQMB: u32 = 0x00000004;
pub const MC_CHP_IO_CNTL_B1__MEM_SLEWN_DQSB_MASK: u32 = 0x00000008;
pub const MC_CHP_IO_CNTL_B1__MEM_SLEWN_DQSB: u32 = 0x00000008;
pub const MC_CHP_IO_CNTL_B1__MEM_SLEWP_CKB_MASK: u32 = 0x00000010;
pub const MC_CHP_IO_CNTL_B1__MEM_SLEWP_CKB: u32 = 0x00000010;
pub const MC_CHP_IO_CNTL_B1__MEM_SLEWP_AB_MASK: u32 = 0x00000020;
pub const MC_CHP_IO_CNTL_B1__MEM_SLEWP_AB: u32 = 0x00000020;
pub const MC_CHP_IO_CNTL_B1__MEM_SLEWP_DQMB_MASK: u32 = 0x00000040;
pub const MC_CHP_IO_CNTL_B1__MEM_SLEWP_DQMB: u32 = 0x00000040;
pub const MC_CHP_IO_CNTL_B1__MEM_SLEWP_DQSB_MASK: u32 = 0x00000080;
pub const MC_CHP_IO_CNTL_B1__MEM_SLEWP_DQSB: u32 = 0x00000080;
pub const MC_CHP_IO_CNTL_B1__MEM_PREAMP_AB_MASK: u32 = 0x00000100;
pub const MC_CHP_IO_CNTL_B1__MEM_PREAMP_AB: u32 = 0x00000100;
pub const MC_CHP_IO_CNTL_B1__MEM_PREAMP_DQMB_MASK: u32 = 0x00000200;
pub const MC_CHP_IO_CNTL_B1__MEM_PREAMP_DQMB: u32 = 0x00000200;
pub const MC_CHP_IO_CNTL_B1__MEM_PREAMP_DQSB_MASK: u32 = 0x00000400;
pub const MC_CHP_IO_CNTL_B1__MEM_PREAMP_DQSB: u32 = 0x00000400;
pub const MC_CHP_IO_CNTL_B1__MEM_IO_MODEB_MASK: u32 = 0x00003000;
pub const MC_CHP_IO_CNTL_B1__MEM_REC_CKB_MASK: u32 = 0x0000c000;
pub const MC_CHP_IO_CNTL_B1__MEM_REC_AB_MASK: u32 = 0x00030000;
pub const MC_CHP_IO_CNTL_B1__MEM_REC_DQMB_MASK: u32 = 0x000c0000;
pub const MC_CHP_IO_CNTL_B1__MEM_REC_DQSB_MASK: u32 = 0x00300000;
pub const MC_CHP_IO_CNTL_B1__MEM_SYNC_PHASEB_MASK: u32 = 0x00400000;
pub const MC_CHP_IO_CNTL_B1__MEM_SYNC_PHASEB: u32 = 0x00400000;
pub const MC_CHP_IO_CNTL_B1__MEM_SYNC_CENTERB_MASK: u32 = 0x00800000;
pub const MC_CHP_IO_CNTL_B1__MEM_SYNC_CENTERB: u32 = 0x00800000;
pub const MC_CHP_IO_CNTL_B1__MEM_SYNC_ENB_MASK: u32 = 0x03000000;
pub const MC_CHP_IO_CNTL_B1__MEM_CLK_SELB_MASK: u32 = 0x0c000000;
pub const MC_CHP_IO_CNTL_B1__MEM_CLK_INVB_MASK: u32 = 0x10000000;
pub const MC_CHP_IO_CNTL_B1__MEM_CLK_INVB: u32 = 0x10000000;
pub const MC_CHP_IO_CNTL_B1__MEM_DATA_ENIMP_B_MASK: u32 = 0x40000000;
pub const MC_CHP_IO_CNTL_B1__MEM_DATA_ENIMP_B: u32 = 0x40000000;
pub const MC_CHP_IO_CNTL_B1__MEM_CNTL_ENIMP_B_MASK: u32 = 0x80000000;
pub const MC_CHP_IO_CNTL_B1__MEM_CNTL_ENIMP_B: u32 = 0x80000000;

// MEM_SDRAM_MODE_REG
pub const MEM_SDRAM_MODE_REG__MEM_MODE_REG_MASK: u32 = 0x00007fff;
pub const MEM_SDRAM_MODE_REG__MEM_WR_LATENCY_MASK: u32 = 0x000f0000;
pub const MEM_SDRAM_MODE_REG__MEM_CAS_LATENCY_MASK: u32 = 0x00700000;
pub const MEM_SDRAM_MODE_REG__MEM_CMD_LATENCY_MASK: u32 = 0x00800000;
pub const MEM_SDRAM_MODE_REG__MEM_CMD_LATENCY: u32 = 0x00800000;
pub const MEM_SDRAM_MODE_REG__MEM_STR_LATENCY_MASK: u32 = 0x01000000;
pub const MEM_SDRAM_MODE_REG__MEM_STR_LATENCY: u32 = 0x01000000;
pub const MEM_SDRAM_MODE_REG__MEM_FALL_OUT_CMD_MASK: u32 = 0x02000000;
pub const MEM_SDRAM_MODE_REG__MEM_FALL_OUT_CMD: u32 = 0x02000000;
pub const MEM_SDRAM_MODE_REG__MEM_FALL_OUT_DATA_MASK: u32 = 0x04000000;
pub const MEM_SDRAM_MODE_REG__MEM_FALL_OUT_DATA: u32 = 0x04000000;
pub const MEM_SDRAM_MODE_REG__MEM_FALL_OUT_STR_MASK: u32 = 0x08000000;
pub const MEM_SDRAM_MODE_REG__MEM_FALL_OUT_STR: u32 = 0x08000000;
pub const MEM_SDRAM_MODE_REG__MC_INIT_COMPLETE_MASK: u32 = 0x10000000;
pub const MEM_SDRAM_MODE_REG__MC_INIT_COMPLETE: u32 = 0x10000000;
pub const MEM_SDRAM_MODE_REG__MEM_DDR_DLL_MASK: u32 = 0x20000000;
pub const MEM_SDRAM_MODE_REG__MEM_DDR_DLL: u32 = 0x20000000;
pub const MEM_SDRAM_MODE_REG__MEM_CFG_TYPE_MASK: u32 = 0x40000000;
pub const MEM_SDRAM_MODE_REG__MEM_CFG_TYPE: u32 = 0x40000000;
pub const MEM_SDRAM_MODE_REG__MEM_SDRAM_RESET_MASK: u32 = 0x80000000;
pub const MEM_SDRAM_MODE_REG__MEM_SDRAM_RESET: u32 = 0x80000000;

// MEM_SDRAM_MODE_REG
pub const MEM_SDRAM_MODE_REG__MEM_MODE_REG__SHIFT: u32 = 0x00000000;
pub const MEM_SDRAM_MODE_REG__MEM_WR_LATENCY__SHIFT: u32 = 0x00000010;
pub const MEM_SDRAM_MODE_REG__MEM_CAS_LATENCY__SHIFT: u32 = 0x00000014;
pub const MEM_SDRAM_MODE_REG__MEM_CMD_LATENCY__SHIFT: u32 = 0x00000017;
pub const MEM_SDRAM_MODE_REG__MEM_STR_LATENCY__SHIFT: u32 = 0x00000018;
pub const MEM_SDRAM_MODE_REG__MEM_FALL_OUT_CMD__SHIFT: u32 = 0x00000019;
pub const MEM_SDRAM_MODE_REG__MEM_FALL_OUT_DATA__SHIFT: u32 = 0x0000001a;
pub const MEM_SDRAM_MODE_REG__MEM_FALL_OUT_STR__SHIFT: u32 = 0x0000001b;
pub const MEM_SDRAM_MODE_REG__MC_INIT_COMPLETE__SHIFT: u32 = 0x0000001c;
pub const MEM_SDRAM_MODE_REG__MEM_DDR_DLL__SHIFT: u32 = 0x0000001d;
pub const MEM_SDRAM_MODE_REG__MEM_CFG_TYPE__SHIFT: u32 = 0x0000001e;
pub const MEM_SDRAM_MODE_REG__MEM_SDRAM_RESET__SHIFT: u32 = 0x0000001f;

// MEM_REFRESH_CNTL
pub const MEM_REFRESH_CNTL__MEM_REFRESH_RATE_MASK: u32 = 0x000000ff;
pub const MEM_REFRESH_CNTL__MEM_REFRESH_DIS_MASK: u32 = 0x00000100;
pub const MEM_REFRESH_CNTL__MEM_REFRESH_DIS: u32 = 0x00000100;
pub const MEM_REFRESH_CNTL__MEM_DYNAMIC_CKE_MASK: u32 = 0x00000200;
pub const MEM_REFRESH_CNTL__MEM_DYNAMIC_CKE: u32 = 0x00000200;
pub const MEM_REFRESH_CNTL__MEM_TRFC_MASK: u32 = 0x0000f000;
pub const MEM_REFRESH_CNTL__MEM_CLKA0_ENABLE_MASK: u32 = 0x00010000;
pub const MEM_REFRESH_CNTL__MEM_CLKA0_ENABLE: u32 = 0x00010000;
pub const MEM_REFRESH_CNTL__MEM_CLKA0b_ENABLE_MASK: u32 = 0x00020000;
pub const MEM_REFRESH_CNTL__MEM_CLKA0b_ENABLE: u32 = 0x00020000;
pub const MEM_REFRESH_CNTL__MEM_CLKA1_ENABLE_MASK: u32 = 0x00040000;
pub const MEM_REFRESH_CNTL__MEM_CLKA1_ENABLE: u32 = 0x00040000;
pub const MEM_REFRESH_CNTL__MEM_CLKA1b_ENABLE_MASK: u32 = 0x00080000;
pub const MEM_REFRESH_CNTL__MEM_CLKA1b_ENABLE: u32 = 0x00080000;
pub const MEM_REFRESH_CNTL__MEM_CLKAFB_ENABLE_MASK: u32 = 0x00100000;
pub const MEM_REFRESH_CNTL__MEM_CLKAFB_ENABLE: u32 = 0x00100000;
pub const MEM_REFRESH_CNTL__DLL_FB_SLCT_CKA_MASK: u32 = 0x00c00000;
pub const MEM_REFRESH_CNTL__MEM_CLKB0_ENABLE_MASK: u32 = 0x01000000;
pub const MEM_REFRESH_CNTL__MEM_CLKB0_ENABLE: u32 = 0x01000000;
pub const MEM_REFRESH_CNTL__MEM_CLKB0b_ENABLE_MASK: u32 = 0x02000000;
pub const MEM_REFRESH_CNTL__MEM_CLKB0b_ENABLE: u32 = 0x02000000;
pub const MEM_REFRESH_CNTL__MEM_CLKB1_ENABLE_MASK: u32 = 0x04000000;
pub const MEM_REFRESH_CNTL__MEM_CLKB1_ENABLE: u32 = 0x04000000;
pub const MEM_REFRESH_CNTL__MEM_CLKB1b_ENABLE_MASK: u32 = 0x08000000;
pub const MEM_REFRESH_CNTL__MEM_CLKB1b_ENABLE: u32 = 0x08000000;
pub const MEM_REFRESH_CNTL__MEM_CLKBFB_ENABLE_MASK: u32 = 0x10000000;
pub const MEM_REFRESH_CNTL__MEM_CLKBFB_ENABLE: u32 = 0x10000000;
pub const MEM_REFRESH_CNTL__DLL_FB_SLCT_CKB_MASK: u32 = 0xc0000000;

// MC_STATUS
pub const MC_STATUS__MEM_PWRUP_COMPL_A_MASK: u32 = 0x00000001;
pub const MC_STATUS__MEM_PWRUP_COMPL_A: u32 = 0x00000001;
pub const MC_STATUS__MEM_PWRUP_COMPL_B_MASK: u32 = 0x00000002;
pub const MC_STATUS__MEM_PWRUP_COMPL_B: u32 = 0x00000002;
pub const MC_STATUS__MC_IDLE_MASK: u32 = 0x00000004;
pub const MC_STATUS__MC_IDLE: u32 = 0x00000004;
pub const MC_STATUS__IMP_N_VALUE_R_BACK_MASK: u32 = 0x00000078;
pub const MC_STATUS__IMP_P_VALUE_R_BACK_MASK: u32 = 0x00000780;
pub const MC_STATUS__TEST_OUT_R_BACK_MASK: u32 = 0x00000800;
pub const MC_STATUS__TEST_OUT_R_BACK: u32 = 0x00000800;
pub const MC_STATUS__DUMMY_OUT_R_BACK_MASK: u32 = 0x00001000;
pub const MC_STATUS__DUMMY_OUT_R_BACK: u32 = 0x00001000;
pub const MC_STATUS__IMP_N_VALUE_A_R_BACK_MASK: u32 = 0x0001e000;
pub const MC_STATUS__IMP_P_VALUE_A_R_BACK_MASK: u32 = 0x001e0000;
pub const MC_STATUS__IMP_N_VALUE_CK_R_BACK_MASK: u32 = 0x01e00000;
pub const MC_STATUS__IMP_P_VALUE_CK_R_BACK_MASK: u32 = 0x1e000000;

// MDLL_CKO
pub const MDLL_CKO__MCKOA_SLEEP_MASK: u32 = 0x00000001;
pub const MDLL_CKO__MCKOA_SLEEP: u32 = 0x00000001;
pub const MDLL_CKO__MCKOA_RESET_MASK: u32 = 0x00000002;
pub const MDLL_CKO__MCKOA_RESET: u32 = 0x00000002;
pub const MDLL_CKO__MCKOA_RANGE_MASK: u32 = 0x0000000c;
pub const MDLL_CKO__ERSTA_SOUTSEL_MASK: u32 = 0x00000030;
pub const MDLL_CKO__MCKOA_FB_SEL_MASK: u32 = 0x000000c0;
pub const MDLL_CKO__MCKOA_REF_SKEW_MASK: u32 = 0x00000700;
pub const MDLL_CKO__MCKOA_FB_SKEW_MASK: u32 = 0x00007000;
pub const MDLL_CKO__MCKOA_BP_SEL_MASK: u32 = 0x00008000;
pub const MDLL_CKO__MCKOA_BP_SEL: u32 = 0x00008000;
pub const MDLL_CKO__MCKOB_SLEEP_MASK: u32 = 0x00010000;
pub const MDLL_CKO__MCKOB_SLEEP: u32 = 0x00010000;
pub const MDLL_CKO__MCKOB_RESET_MASK: u32 = 0x00020000;
pub const MDLL_CKO__MCKOB_RESET: u32 = 0x00020000;
pub const MDLL_CKO__MCKOB_RANGE_MASK: u32 = 0x000c0000;
pub const MDLL_CKO__ERSTB_SOUTSEL_MASK: u32 = 0x00300000;
pub const MDLL_CKO__MCKOB_FB_SEL_MASK: u32 = 0x00c00000;
pub const MDLL_CKO__MCKOB_REF_SKEW_MASK: u32 = 0x07000000;
pub const MDLL_CKO__MCKOB_FB_SKEW_MASK: u32 = 0x70000000;
pub const MDLL_CKO__MCKOB_BP_SEL_MASK: u32 = 0x80000000;
pub const MDLL_CKO__MCKOB_BP_SEL: u32 = 0x80000000;

// MDLL_RDCKA
pub const MDLL_RDCKA__MRDCKA0_SLEEP_MASK: u32 = 0x00000001;
pub const MDLL_RDCKA__MRDCKA0_SLEEP: u32 = 0x00000001;
pub const MDLL_RDCKA__MRDCKA0_RESET_MASK: u32 = 0x00000002;
pub const MDLL_RDCKA__MRDCKA0_RESET: u32 = 0x00000002;
pub const MDLL_RDCKA__MRDCKA0_RANGE_MASK: u32 = 0x0000000c;
pub const MDLL_RDCKA__MRDCKA0_REF_SEL_MASK: u32 = 0x00000030;
pub const MDLL_RDCKA__MRDCKA0_FB_SEL_MASK: u32 = 0x000000c0;
pub const MDLL_RDCKA__MRDCKA0_REF_SKEW_MASK: u32 = 0x00000700;
pub const MDLL_RDCKA__MRDCKA0_SINSEL_MASK: u32 = 0x00000800;
pub const MDLL_RDCKA__MRDCKA0_SINSEL: u32 = 0x00000800;
pub const MDLL_RDCKA__MRDCKA0_FB_SKEW_MASK: u32 = 0x00007000;
pub const MDLL_RDCKA__MRDCKA0_BP_SEL_MASK: u32 = 0x00008000;
pub const MDLL_RDCKA__MRDCKA0_BP_SEL: u32 = 0x00008000;
pub const MDLL_RDCKA__MRDCKA1_SLEEP_MASK: u32 = 0x00010000;
pub const MDLL_RDCKA__MRDCKA1_SLEEP: u32 = 0x00010000;
pub const MDLL_RDCKA__MRDCKA1_RESET_MASK: u32 = 0x00020000;
pub const MDLL_RDCKA__MRDCKA1_RESET: u32 = 0x00020000;
pub const MDLL_RDCKA__MRDCKA1_RANGE_MASK: u32 = 0x000c0000;
pub const MDLL_RDCKA__MRDCKA1_REF_SEL_MASK: u32 = 0x00300000;
pub const MDLL_RDCKA__MRDCKA1_FB_SEL_MASK: u32 = 0x00c00000;
pub const MDLL_RDCKA__MRDCKA1_REF_SKEW_MASK: u32 = 0x07000000;
pub const MDLL_RDCKA__MRDCKA1_SINSEL_MASK: u32 = 0x08000000;
pub const MDLL_RDCKA__MRDCKA1_SINSEL: u32 = 0x08000000;
pub const MDLL_RDCKA__MRDCKA1_FB_SKEW_MASK: u32 = 0x70000000;
pub const MDLL_RDCKA__MRDCKA1_BP_SEL_MASK: u32 = 0x80000000;
pub const MDLL_RDCKA__MRDCKA1_BP_SEL: u32 = 0x80000000;

// MDLL_RDCKB
pub const MDLL_RDCKB__MRDCKB0_SLEEP_MASK: u32 = 0x00000001;
pub const MDLL_RDCKB__MRDCKB0_SLEEP: u32 = 0x00000001;
pub const MDLL_RDCKB__MRDCKB0_RESET_MASK: u32 = 0x00000002;
pub const MDLL_RDCKB__MRDCKB0_RESET: u32 = 0x00000002;
pub const MDLL_RDCKB__MRDCKB0_RANGE_MASK: u32 = 0x0000000c;
pub const MDLL_RDCKB__MRDCKB0_REF_SEL_MASK: u32 = 0x00000030;
pub const MDLL_RDCKB__MRDCKB0_FB_SEL_MASK: u32 = 0x000000c0;
pub const MDLL_RDCKB__MRDCKB0_REF_SKEW_MASK: u32 = 0x00000700;
pub const MDLL_RDCKB__MRDCKB0_SINSEL_MASK: u32 = 0x00000800;
pub const MDLL_RDCKB__MRDCKB0_SINSEL: u32 = 0x00000800;
pub const MDLL_RDCKB__MRDCKB0_FB_SKEW_MASK: u32 = 0x00007000;
pub const MDLL_RDCKB__MRDCKB0_BP_SEL_MASK: u32 = 0x00008000;
pub const MDLL_RDCKB__MRDCKB0_BP_SEL: u32 = 0x00008000;
pub const MDLL_RDCKB__MRDCKB1_SLEEP_MASK: u32 = 0x00010000;
pub const MDLL_RDCKB__MRDCKB1_SLEEP: u32 = 0x00010000;
pub const MDLL_RDCKB__MRDCKB1_RESET_MASK: u32 = 0x00020000;
pub const MDLL_RDCKB__MRDCKB1_RESET: u32 = 0x00020000;
pub const MDLL_RDCKB__MRDCKB1_RANGE_MASK: u32 = 0x000c0000;
pub const MDLL_RDCKB__MRDCKB1_REF_SEL_MASK: u32 = 0x00300000;
pub const MDLL_RDCKB__MRDCKB1_FB_SEL_MASK: u32 = 0x00c00000;
pub const MDLL_RDCKB__MRDCKB1_REF_SKEW_MASK: u32 = 0x07000000;
pub const MDLL_RDCKB__MRDCKB1_SINSEL_MASK: u32 = 0x08000000;
pub const MDLL_RDCKB__MRDCKB1_SINSEL: u32 = 0x08000000;
pub const MDLL_RDCKB__MRDCKB1_FB_SKEW_MASK: u32 = 0x70000000;
pub const MDLL_RDCKB__MRDCKB1_BP_SEL_MASK: u32 = 0x80000000;
pub const MDLL_RDCKB__MRDCKB1_BP_SEL: u32 = 0x80000000;

pub const MDLL_R300_RDCK__MRDCKA_SLEEP: u32 = 0x00000001;
pub const MDLL_R300_RDCK__MRDCKA_RESET: u32 = 0x00000002;
pub const MDLL_R300_RDCK__MRDCKB_SLEEP: u32 = 0x00000004;
pub const MDLL_R300_RDCK__MRDCKB_RESET: u32 = 0x00000008;
pub const MDLL_R300_RDCK__MRDCKC_SLEEP: u32 = 0x00000010;
pub const MDLL_R300_RDCK__MRDCKC_RESET: u32 = 0x00000020;
pub const MDLL_R300_RDCK__MRDCKD_SLEEP: u32 = 0x00000040;
pub const MDLL_R300_RDCK__MRDCKD_RESET: u32 = 0x00000080;

pub const pllCLK_PIN_CNTL: u32 = 0x0001;
pub const pllPPLL_CNTL: u32 = 0x0002;
pub const pllPPLL_REF_DIV: u32 = 0x0003;
pub const pllPPLL_DIV_0: u32 = 0x0004;
pub const pllPPLL_DIV_1: u32 = 0x0005;
pub const pllPPLL_DIV_2: u32 = 0x0006;
pub const pllPPLL_DIV_3: u32 = 0x0007;
pub const pllVCLK_ECP_CNTL: u32 = 0x0008;
pub const pllHTOTAL_CNTL: u32 = 0x0009;
pub const pllM_SPLL_REF_FB_DIV: u32 = 0x000A;
pub const pllAGP_PLL_CNTL: u32 = 0x000B;
pub const pllSPLL_CNTL: u32 = 0x000C;
pub const pllSCLK_CNTL: u32 = 0x000D;
pub const pllMPLL_CNTL: u32 = 0x000E;
pub const pllMDLL_CKO: u32 = 0x000F;
pub const pllMDLL_RDCKA: u32 = 0x0010;
pub const pllMDLL_RDCKB: u32 = 0x0011;
pub const pllMCLK_CNTL: u32 = 0x0012;
pub const pllPLL_TEST_CNTL: u32 = 0x0013;
pub const pllCLK_PWRMGT_CNTL: u32 = 0x0014;
pub const pllPLL_PWRMGT_CNTL: u32 = 0x0015;
pub const pllCG_TEST_MACRO_RW_WRITE: u32 = 0x0016;
pub const pllCG_TEST_MACRO_RW_READ: u32 = 0x0017;
pub const pllCG_TEST_MACRO_RW_DATA: u32 = 0x0018;
pub const pllCG_TEST_MACRO_RW_CNTL: u32 = 0x0019;
pub const pllDISP_TEST_MACRO_RW_WRITE: u32 = 0x001A;
pub const pllDISP_TEST_MACRO_RW_READ: u32 = 0x001B;
pub const pllDISP_TEST_MACRO_RW_DATA: u32 = 0x001C;
pub const pllDISP_TEST_MACRO_RW_CNTL: u32 = 0x001D;
pub const pllSCLK_CNTL2: u32 = 0x001E;
pub const pllMCLK_MISC: u32 = 0x001F;
pub const pllTV_PLL_FINE_CNTL: u32 = 0x0020;
pub const pllTV_PLL_CNTL: u32 = 0x0021;
pub const pllTV_PLL_CNTL1: u32 = 0x0022;
pub const pllTV_DTO_INCREMENTS: u32 = 0x0023;
pub const pllSPLL_AUX_CNTL: u32 = 0x0024;
pub const pllMPLL_AUX_CNTL: u32 = 0x0025;
pub const pllP2PLL_CNTL: u32 = 0x002A;
pub const pllP2PLL_REF_DIV: u32 = 0x002B;
pub const pllP2PLL_DIV_0: u32 = 0x002C;
pub const pllPIXCLKS_CNTL: u32 = 0x002D;
pub const pllHTOTAL2_CNTL: u32 = 0x002E;
pub const pllSSPLL_CNTL: u32 = 0x0030;
pub const pllSSPLL_REF_DIV: u32 = 0x0031;
pub const pllSSPLL_DIV_0: u32 = 0x0032;
pub const pllSS_INT_CNTL: u32 = 0x0033;
pub const pllSS_TST_CNTL: u32 = 0x0034;
pub const pllSCLK_MORE_CNTL: u32 = 0x0035;

pub const ixMC_PERF_CNTL: u32 = 0x0000;
pub const ixMC_PERF_SEL: u32 = 0x0001;
pub const ixMC_PERF_REGION_0: u32 = 0x0002;
pub const ixMC_PERF_REGION_1: u32 = 0x0003;
pub const ixMC_PERF_COUNT_0: u32 = 0x0004;
pub const ixMC_PERF_COUNT_1: u32 = 0x0005;
pub const ixMC_PERF_COUNT_2: u32 = 0x0006;
pub const ixMC_PERF_COUNT_3: u32 = 0x0007;
pub const ixMC_PERF_COUNT_MEMCH_A: u32 = 0x0008;
pub const ixMC_PERF_COUNT_MEMCH_B: u32 = 0x0009;
pub const ixMC_IMP_CNTL: u32 = 0x000A;
pub const ixMC_CHP_IO_CNTL_A0: u32 = 0x000B;
pub const ixMC_CHP_IO_CNTL_A1: u32 = 0x000C;
pub const ixMC_CHP_IO_CNTL_B0: u32 = 0x000D;
pub const ixMC_CHP_IO_CNTL_B1: u32 = 0x000E;
pub const ixMC_IMP_CNTL_0: u32 = 0x000F;
pub const ixTC_MISMATCH_1: u32 = 0x0010;
pub const ixTC_MISMATCH_2: u32 = 0x0011;
pub const ixMC_BIST_CTRL: u32 = 0x0012;
pub const ixREG_COLLAR_WRITE: u32 = 0x0013;
pub const ixREG_COLLAR_READ: u32 = 0x0014;
pub const ixR300_MC_IMP_CNTL: u32 = 0x0018;
pub const ixR300_MC_CHP_IO_CNTL_A0: u32 = 0x0019;
pub const ixR300_MC_CHP_IO_CNTL_A1: u32 = 0x001a;
pub const ixR300_MC_CHP_IO_CNTL_B0: u32 = 0x001b;
pub const ixR300_MC_CHP_IO_CNTL_B1: u32 = 0x001c;
pub const ixR300_MC_CHP_IO_CNTL_C0: u32 = 0x001d;
pub const ixR300_MC_CHP_IO_CNTL_C1: u32 = 0x001e;
pub const ixR300_MC_CHP_IO_CNTL_D0: u32 = 0x001f;
pub const ixR300_MC_CHP_IO_CNTL_D1: u32 = 0x0020;
pub const ixR300_MC_IMP_CNTL_0: u32 = 0x0021;
pub const ixR300_MC_ELPIDA_CNTL: u32 = 0x0022;
pub const ixR300_MC_CHP_IO_OE_CNTL_CD: u32 = 0x0023;
pub const ixR300_MC_READ_CNTL_CD: u32 = 0x0024;
pub const ixR300_MC_MC_INIT_WR_LAT_TIMER: u32 = 0x0025;
pub const ixR300_MC_DEBUG_CNTL: u32 = 0x0026;
pub const ixR300_MC_BIST_CNTL_0: u32 = 0x0028;
pub const ixR300_MC_BIST_CNTL_1: u32 = 0x0029;
pub const ixR300_MC_BIST_CNTL_2: u32 = 0x002a;
pub const ixR300_MC_BIST_CNTL_3: u32 = 0x002b;
pub const ixR300_MC_BIST_CNTL_4: u32 = 0x002c;
pub const ixR300_MC_BIST_CNTL_5: u32 = 0x002d;
pub const ixR300_MC_IMP_STATUS: u32 = 0x002e;
pub const ixR300_MC_DLL_CNTL: u32 = 0x002f;
pub const NB_TOM: u32 = 0x15C;




// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
