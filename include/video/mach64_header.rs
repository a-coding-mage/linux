/* SPDX-License-Identifier: GPL-2.0-or-later */
/* Faithful Rust translation of the ATI Mach64 register definitions. */
/* SPDX-License-Identifier: GPL-2.0-or-later */
/*
 * ATI Mach64 Register Definitions
 *
 * Copyright (C) 1997 Michael AK Tesch
 *  written with much help from Jon Howell
 *
 * Updated for 3D RAGE PRO and 3D RAGE Mobility by Geert Uytterhoeven
 */

/*
 * most of the rest of this file comes from ATI sample code
 */

/* NON-GUI MEMORY MAPPED Registers - expressed in BYTE offsets */

/* Accelerator CRTC */
pub const CRTC_H_TOTAL_DISP: u32 = 0x0000	/* Dword offset 0_00 */;
pub const CRTC2_H_TOTAL_DISP: u32 = 0x0000	/* Dword offset 0_00 */;
pub const CRTC_H_SYNC_STRT_WID: u32 = 0x0004	/* Dword offset 0_01 */;
pub const CRTC2_H_SYNC_STRT_WID: u32 = 0x0004	/* Dword offset 0_01 */;
pub const CRTC_H_SYNC_STRT: u32 = 0x0004;
pub const CRTC2_H_SYNC_STRT: u32 = 0x0004;
pub const CRTC_H_SYNC_DLY: u32 = 0x0005;
pub const CRTC2_H_SYNC_DLY: u32 = 0x0005;
pub const CRTC_H_SYNC_WID: u32 = 0x0006;
pub const CRTC2_H_SYNC_WID: u32 = 0x0006;
pub const CRTC_V_TOTAL_DISP: u32 = 0x0008	/* Dword offset 0_02 */;
pub const CRTC2_V_TOTAL_DISP: u32 = 0x0008	/* Dword offset 0_02 */;
pub const CRTC_V_TOTAL: u32 = 0x0008;
pub const CRTC2_V_TOTAL: u32 = 0x0008;
pub const CRTC_V_DISP: u32 = 0x000A;
pub const CRTC2_V_DISP: u32 = 0x000A;
pub const CRTC_V_SYNC_STRT_WID: u32 = 0x000C	/* Dword offset 0_03 */;
pub const CRTC2_V_SYNC_STRT_WID: u32 = 0x000C	/* Dword offset 0_03 */;
pub const CRTC_V_SYNC_STRT: u32 = 0x000C;
pub const CRTC2_V_SYNC_STRT: u32 = 0x000C;
pub const CRTC_V_SYNC_WID: u32 = 0x000E;
pub const CRTC2_V_SYNC_WID: u32 = 0x000E;
pub const CRTC_VLINE_CRNT_VLINE: u32 = 0x0010	/* Dword offset 0_04 */;
pub const CRTC2_VLINE_CRNT_VLINE: u32 = 0x0010	/* Dword offset 0_04 */;
pub const CRTC_OFF_PITCH: u32 = 0x0014	/* Dword offset 0_05 */;
pub const CRTC_OFFSET: u32 = 0x0014;
pub const CRTC_PITCH: u32 = 0x0016;
pub const CRTC_INT_CNTL: u32 = 0x0018	/* Dword offset 0_06 */;
pub const CRTC_GEN_CNTL: u32 = 0x001C	/* Dword offset 0_07 */;
pub const CRTC_PIX_WIDTH: u32 = 0x001D;
pub const CRTC_FIFO: u32 = 0x001E;
pub const CRTC_EXT_DISP: u32 = 0x001F;

/* Memory Buffer Control */
pub const DSP_CONFIG: u32 = 0x0020	/* Dword offset 0_08 */;
pub const PM_DSP_CONFIG: u32 = 0x0020	/* Dword offset 0_08 (Mobility Only) */;
pub const DSP_ON_OFF: u32 = 0x0024	/* Dword offset 0_09 */;
pub const PM_DSP_ON_OFF: u32 = 0x0024	/* Dword offset 0_09 (Mobility Only) */;
pub const TIMER_CONFIG: u32 = 0x0028	/* Dword offset 0_0A */;
pub const MEM_BUF_CNTL: u32 = 0x002C	/* Dword offset 0_0B */;
pub const MEM_ADDR_CONFIG: u32 = 0x0034	/* Dword offset 0_0D */;

/* Accelerator CRTC */
pub const CRT_TRAP: u32 = 0x0038	/* Dword offset 0_0E */;

pub const I2C_CNTL_0: u32 = 0x003C	/* Dword offset 0_0F */;

pub const DSTN_CONTROL_LG: u32 = 0x003C	/* Dword offset 0_0F (LG) */;

/* Overscan */
pub const OVR_CLR: u32 = 0x0040	/* Dword offset 0_10 */;
pub const OVR2_CLR: u32 = 0x0040	/* Dword offset 0_10 */;
pub const OVR_WID_LEFT_RIGHT: u32 = 0x0044	/* Dword offset 0_11 */;
pub const OVR2_WID_LEFT_RIGHT: u32 = 0x0044	/* Dword offset 0_11 */;
pub const OVR_WID_TOP_BOTTOM: u32 = 0x0048	/* Dword offset 0_12 */;
pub const OVR2_WID_TOP_BOTTOM: u32 = 0x0048	/* Dword offset 0_12 */;

/* Memory Buffer Control */
pub const VGA_DSP_CONFIG: u32 = 0x004C	/* Dword offset 0_13 */;
pub const PM_VGA_DSP_CONFIG: u32 = 0x004C	/* Dword offset 0_13 (Mobility Only) */;
pub const VGA_DSP_ON_OFF: u32 = 0x0050	/* Dword offset 0_14 */;
pub const PM_VGA_DSP_ON_OFF: u32 = 0x0050	/* Dword offset 0_14 (Mobility Only) */;
pub const DSP2_CONFIG: u32 = 0x0054	/* Dword offset 0_15 */;
pub const PM_DSP2_CONFIG: u32 = 0x0054	/* Dword offset 0_15 (Mobility Only) */;
pub const DSP2_ON_OFF: u32 = 0x0058	/* Dword offset 0_16 */;
pub const PM_DSP2_ON_OFF: u32 = 0x0058	/* Dword offset 0_16 (Mobility Only) */;

/* Accelerator CRTC */
pub const CRTC2_OFF_PITCH: u32 = 0x005C	/* Dword offset 0_17 */;

/* Hardware Cursor */
pub const CUR_CLR0: u32 = 0x0060	/* Dword offset 0_18 */;
pub const CUR2_CLR0: u32 = 0x0060	/* Dword offset 0_18 */;
pub const CUR_CLR1: u32 = 0x0064	/* Dword offset 0_19 */;
pub const CUR2_CLR1: u32 = 0x0064	/* Dword offset 0_19 */;
pub const CUR_OFFSET: u32 = 0x0068	/* Dword offset 0_1A */;
pub const CUR2_OFFSET: u32 = 0x0068	/* Dword offset 0_1A */;
pub const CUR_HORZ_VERT_POSN: u32 = 0x006C	/* Dword offset 0_1B */;
pub const CUR2_HORZ_VERT_POSN: u32 = 0x006C	/* Dword offset 0_1B */;
pub const CUR_HORZ_VERT_OFF: u32 = 0x0070	/* Dword offset 0_1C */;
pub const CUR2_HORZ_VERT_OFF: u32 = 0x0070	/* Dword offset 0_1C */;

pub const CNFG_PANEL_LG: u32 = 0x0074	/* Dword offset 0_1D (LG) */;

/* General I/O Control */
pub const GP_IO: u32 = 0x0078	/* Dword offset 0_1E */;

/* Test and Debug */
pub const HW_DEBUG: u32 = 0x007C	/* Dword offset 0_1F */;

/* Scratch Pad and Test */
pub const SCRATCH_REG0: u32 = 0x0080	/* Dword offset 0_20 */;
pub const SCRATCH_REG1: u32 = 0x0084	/* Dword offset 0_21 */;
pub const SCRATCH_REG2: u32 = 0x0088	/* Dword offset 0_22 */;
pub const SCRATCH_REG3: u32 = 0x008C	/* Dword offset 0_23 */;

/* Clock Control */
pub const CLOCK_CNTL: u32 = 0x0090	/* Dword offset 0_24 */;
/* CLOCK_CNTL register constants CT LAYOUT */
pub const CLOCK_SEL: u32 = 0x0f;
pub const CLOCK_SEL_INTERNAL: u32 = 0x03;
pub const CLOCK_SEL_EXTERNAL: u32 = 0x0c;
pub const CLOCK_DIV: u32 = 0x30;
pub const CLOCK_DIV1: u32 = 0x00;
pub const CLOCK_DIV2: u32 = 0x10;
pub const CLOCK_DIV4: u32 = 0x20;
pub const CLOCK_STROBE: u32 = 0x40;
/*  ?					0x80 */
/* CLOCK_CNTL register constants GX LAYOUT */
pub const CLOCK_BIT: u32 = 0x04	/* For ICS2595 */;
pub const CLOCK_PULSE: u32 = 0x08	/* For ICS2595 */;
/*#define CLOCK_STROBE			0x40 dito as CT */
pub const CLOCK_DATA: u32 = 0x80;

/* For internal PLL(CT) start */
pub const CLOCK_CNTL_ADDR: u32 = CLOCK_CNTL + 1;
pub const PLL_WR_EN: u32 = 0x02;
pub const PLL_ADDR: u32 = 0xfc;
pub const CLOCK_CNTL_DATA: u32 = CLOCK_CNTL + 2;
pub const PLL_DATA: u32 = 0xff;
/* For internal PLL(CT) end */

pub const CLOCK_SEL_CNTL: u32 = 0x0090	/* Dword offset 0_24 */;

/* Configuration */
pub const CNFG_STAT1: u32 = 0x0094	/* Dword offset 0_25 */;
pub const CNFG_STAT2: u32 = 0x0098	/* Dword offset 0_26 */;

/* Bus Control */
pub const BUS_CNTL: u32 = 0x00A0	/* Dword offset 0_28 */;

pub const LCD_INDEX: u32 = 0x00A4	/* Dword offset 0_29 */;
pub const LCD_DATA: u32 = 0x00A8	/* Dword offset 0_2A */;

pub const HFB_PITCH_ADDR_LG: u32 = 0x00A8	/* Dword offset 0_2A (LG) */;

/* Memory Control */
pub const EXT_MEM_CNTL: u32 = 0x00AC	/* Dword offset 0_2B */;
pub const MEM_CNTL: u32 = 0x00B0	/* Dword offset 0_2C */;
pub const MEM_VGA_WP_SEL: u32 = 0x00B4	/* Dword offset 0_2D */;
pub const MEM_VGA_RP_SEL: u32 = 0x00B8	/* Dword offset 0_2E */;

pub const I2C_CNTL_1: u32 = 0x00BC	/* Dword offset 0_2F */;

pub const LT_GIO_LG: u32 = 0x00BC	/* Dword offset 0_2F (LG) */;

/* DAC Control */
pub const DAC_REGS: u32 = 0x00C0	/* Dword offset 0_30 */;
pub const DAC_W_INDEX: u32 = 0x00C0	/* Dword offset 0_30 */;
pub const DAC_DATA: u32 = 0x00C1	/* Dword offset 0_30 */;
pub const DAC_MASK: u32 = 0x00C2	/* Dword offset 0_30 */;
pub const DAC_R_INDEX: u32 = 0x00C3	/* Dword offset 0_30 */;
pub const DAC_CNTL: u32 = 0x00C4	/* Dword offset 0_31 */;

pub const EXT_DAC_REGS: u32 = 0x00C8	/* Dword offset 0_32 */;

pub const HORZ_STRETCHING_LG: u32 = 0x00C8	/* Dword offset 0_32 (LG) */;
pub const VERT_STRETCHING_LG: u32 = 0x00CC	/* Dword offset 0_33 (LG) */;

/* Test and Debug */
pub const GEN_TEST_CNTL: u32 = 0x00D0	/* Dword offset 0_34 */;

/* Custom Macros */
pub const CUSTOM_MACRO_CNTL: u32 = 0x00D4	/* Dword offset 0_35 */;

pub const LCD_GEN_CNTL_LG: u32 = 0x00D4	/* Dword offset 0_35 (LG) */;
pub const POWER_MANAGEMENT_LG: u32 = 0x00D8	/* Dword offset 0_36 (LG) */;

/* Configuration */
pub const CNFG_CNTL: u32 = 0x00DC	/* Dword offset 0_37 (CT, ET, VT) */;
pub const CNFG_CHIP_ID: u32 = 0x00E0	/* Dword offset 0_38 */;
pub const CNFG_STAT0: u32 = 0x00E4	/* Dword offset 0_39 */;

/* Test and Debug */
pub const CRC_SIG: u32 = 0x00E8	/* Dword offset 0_3A */;
pub const CRC2_SIG: u32 = 0x00E8	/* Dword offset 0_3A */;


/* GUI MEMORY MAPPED Registers */

/* Draw Engine Destination Trajectory */
pub const DST_OFF_PITCH: u32 = 0x0100	/* Dword offset 0_40 */;
pub const DST_X: u32 = 0x0104	/* Dword offset 0_41 */;
pub const DST_Y: u32 = 0x0108	/* Dword offset 0_42 */;
pub const DST_Y_X: u32 = 0x010C	/* Dword offset 0_43 */;
pub const DST_WIDTH: u32 = 0x0110	/* Dword offset 0_44 */;
pub const DST_HEIGHT: u32 = 0x0114	/* Dword offset 0_45 */;
pub const DST_HEIGHT_WIDTH: u32 = 0x0118	/* Dword offset 0_46 */;
pub const DST_X_WIDTH: u32 = 0x011C	/* Dword offset 0_47 */;
pub const DST_BRES_LNTH: u32 = 0x0120	/* Dword offset 0_48 */;
pub const DST_BRES_ERR: u32 = 0x0124	/* Dword offset 0_49 */;
pub const DST_BRES_INC: u32 = 0x0128	/* Dword offset 0_4A */;
pub const DST_BRES_DEC: u32 = 0x012C	/* Dword offset 0_4B */;
pub const DST_CNTL: u32 = 0x0130	/* Dword offset 0_4C */;
pub const DST_Y_X__ALIAS__: u32 = 0x0134	/* Dword offset 0_4D */;
pub const TRAIL_BRES_ERR: u32 = 0x0138	/* Dword offset 0_4E */;
pub const TRAIL_BRES_INC: u32 = 0x013C	/* Dword offset 0_4F */;
pub const TRAIL_BRES_DEC: u32 = 0x0140	/* Dword offset 0_50 */;
pub const LEAD_BRES_LNTH: u32 = 0x0144	/* Dword offset 0_51 */;
pub const Z_OFF_PITCH: u32 = 0x0148	/* Dword offset 0_52 */;
pub const Z_CNTL: u32 = 0x014C	/* Dword offset 0_53 */;
pub const ALPHA_TST_CNTL: u32 = 0x0150	/* Dword offset 0_54 */;
pub const SECONDARY_STW_EXP: u32 = 0x0158	/* Dword offset 0_56 */;
pub const SECONDARY_S_X_INC: u32 = 0x015C	/* Dword offset 0_57 */;
pub const SECONDARY_S_Y_INC: u32 = 0x0160	/* Dword offset 0_58 */;
pub const SECONDARY_S_START: u32 = 0x0164	/* Dword offset 0_59 */;
pub const SECONDARY_W_X_INC: u32 = 0x0168	/* Dword offset 0_5A */;
pub const SECONDARY_W_Y_INC: u32 = 0x016C	/* Dword offset 0_5B */;
pub const SECONDARY_W_START: u32 = 0x0170	/* Dword offset 0_5C */;
pub const SECONDARY_T_X_INC: u32 = 0x0174	/* Dword offset 0_5D */;
pub const SECONDARY_T_Y_INC: u32 = 0x0178	/* Dword offset 0_5E */;
pub const SECONDARY_T_START: u32 = 0x017C	/* Dword offset 0_5F */;

/* Draw Engine Source Trajectory */
pub const SRC_OFF_PITCH: u32 = 0x0180	/* Dword offset 0_60 */;
pub const SRC_X: u32 = 0x0184	/* Dword offset 0_61 */;
pub const SRC_Y: u32 = 0x0188	/* Dword offset 0_62 */;
pub const SRC_Y_X: u32 = 0x018C	/* Dword offset 0_63 */;
pub const SRC_WIDTH1: u32 = 0x0190	/* Dword offset 0_64 */;
pub const SRC_HEIGHT1: u32 = 0x0194	/* Dword offset 0_65 */;
pub const SRC_HEIGHT1_WIDTH1: u32 = 0x0198	/* Dword offset 0_66 */;
pub const SRC_X_START: u32 = 0x019C	/* Dword offset 0_67 */;
pub const SRC_Y_START: u32 = 0x01A0	/* Dword offset 0_68 */;
pub const SRC_Y_X_START: u32 = 0x01A4	/* Dword offset 0_69 */;
pub const SRC_WIDTH2: u32 = 0x01A8	/* Dword offset 0_6A */;
pub const SRC_HEIGHT2: u32 = 0x01AC	/* Dword offset 0_6B */;
pub const SRC_HEIGHT2_WIDTH2: u32 = 0x01B0	/* Dword offset 0_6C */;
pub const SRC_CNTL: u32 = 0x01B4	/* Dword offset 0_6D */;

pub const SCALE_OFF: u32 = 0x01C0	/* Dword offset 0_70 */;
pub const SECONDARY_SCALE_OFF: u32 = 0x01C4	/* Dword offset 0_71 */;

pub const TEX_0_OFF: u32 = 0x01C0	/* Dword offset 0_70 */;
pub const TEX_1_OFF: u32 = 0x01C4	/* Dword offset 0_71 */;
pub const TEX_2_OFF: u32 = 0x01C8	/* Dword offset 0_72 */;
pub const TEX_3_OFF: u32 = 0x01CC	/* Dword offset 0_73 */;
pub const TEX_4_OFF: u32 = 0x01D0	/* Dword offset 0_74 */;
pub const TEX_5_OFF: u32 = 0x01D4	/* Dword offset 0_75 */;
pub const TEX_6_OFF: u32 = 0x01D8	/* Dword offset 0_76 */;
pub const TEX_7_OFF: u32 = 0x01DC	/* Dword offset 0_77 */;

pub const SCALE_WIDTH: u32 = 0x01DC	/* Dword offset 0_77 */;
pub const SCALE_HEIGHT: u32 = 0x01E0	/* Dword offset 0_78 */;

pub const TEX_8_OFF: u32 = 0x01E0	/* Dword offset 0_78 */;
pub const TEX_9_OFF: u32 = 0x01E4	/* Dword offset 0_79 */;
pub const TEX_10_OFF: u32 = 0x01E8	/* Dword offset 0_7A */;
pub const S_Y_INC: u32 = 0x01EC	/* Dword offset 0_7B */;

pub const SCALE_PITCH: u32 = 0x01EC	/* Dword offset 0_7B */;
pub const SCALE_X_INC: u32 = 0x01F0	/* Dword offset 0_7C */;

pub const RED_X_INC: u32 = 0x01F0	/* Dword offset 0_7C */;
pub const GREEN_X_INC: u32 = 0x01F4	/* Dword offset 0_7D */;

pub const SCALE_Y_INC: u32 = 0x01F4	/* Dword offset 0_7D */;
pub const SCALE_VACC: u32 = 0x01F8	/* Dword offset 0_7E */;
pub const SCALE_3D_CNTL: u32 = 0x01FC	/* Dword offset 0_7F */;

/* Host Data */
pub const HOST_DATA0: u32 = 0x0200	/* Dword offset 0_80 */;
pub const HOST_DATA1: u32 = 0x0204	/* Dword offset 0_81 */;
pub const HOST_DATA2: u32 = 0x0208	/* Dword offset 0_82 */;
pub const HOST_DATA3: u32 = 0x020C	/* Dword offset 0_83 */;
pub const HOST_DATA4: u32 = 0x0210	/* Dword offset 0_84 */;
pub const HOST_DATA5: u32 = 0x0214	/* Dword offset 0_85 */;
pub const HOST_DATA6: u32 = 0x0218	/* Dword offset 0_86 */;
pub const HOST_DATA7: u32 = 0x021C	/* Dword offset 0_87 */;
pub const HOST_DATA8: u32 = 0x0220	/* Dword offset 0_88 */;
pub const HOST_DATA9: u32 = 0x0224	/* Dword offset 0_89 */;
pub const HOST_DATAA: u32 = 0x0228	/* Dword offset 0_8A */;
pub const HOST_DATAB: u32 = 0x022C	/* Dword offset 0_8B */;
pub const HOST_DATAC: u32 = 0x0230	/* Dword offset 0_8C */;
pub const HOST_DATAD: u32 = 0x0234	/* Dword offset 0_8D */;
pub const HOST_DATAE: u32 = 0x0238	/* Dword offset 0_8E */;
pub const HOST_DATAF: u32 = 0x023C	/* Dword offset 0_8F */;
pub const HOST_CNTL: u32 = 0x0240	/* Dword offset 0_90 */;

/* GUI Bus Mastering */
pub const BM_HOSTDATA: u32 = 0x0244	/* Dword offset 0_91 */;
pub const BM_ADDR: u32 = 0x0248	/* Dword offset 0_92 */;
pub const BM_DATA: u32 = 0x0248	/* Dword offset 0_92 */;
pub const BM_GUI_TABLE_CMD: u32 = 0x024C	/* Dword offset 0_93 */;

/* Pattern */
pub const PAT_REG0: u32 = 0x0280	/* Dword offset 0_A0 */;
pub const PAT_REG1: u32 = 0x0284	/* Dword offset 0_A1 */;
pub const PAT_CNTL: u32 = 0x0288	/* Dword offset 0_A2 */;

/* Scissors */
pub const SC_LEFT: u32 = 0x02A0	/* Dword offset 0_A8 */;
pub const SC_RIGHT: u32 = 0x02A4	/* Dword offset 0_A9 */;
pub const SC_LEFT_RIGHT: u32 = 0x02A8	/* Dword offset 0_AA */;
pub const SC_TOP: u32 = 0x02AC	/* Dword offset 0_AB */;
pub const SC_BOTTOM: u32 = 0x02B0	/* Dword offset 0_AC */;
pub const SC_TOP_BOTTOM: u32 = 0x02B4	/* Dword offset 0_AD */;

/* Data Path */
pub const USR1_DST_OFF_PITCH: u32 = 0x02B8	/* Dword offset 0_AE */;
pub const USR2_DST_OFF_PITCH: u32 = 0x02BC	/* Dword offset 0_AF */;
pub const DP_BKGD_CLR: u32 = 0x02C0	/* Dword offset 0_B0 */;
pub const DP_FOG_CLR: u32 = 0x02C4	/* Dword offset 0_B1 */;
pub const DP_FRGD_CLR: u32 = 0x02C4	/* Dword offset 0_B1 */;
pub const DP_WRITE_MASK: u32 = 0x02C8	/* Dword offset 0_B2 */;
pub const DP_CHAIN_MASK: u32 = 0x02CC	/* Dword offset 0_B3 */;
pub const DP_PIX_WIDTH: u32 = 0x02D0	/* Dword offset 0_B4 */;
pub const DP_MIX: u32 = 0x02D4	/* Dword offset 0_B5 */;
pub const DP_SRC: u32 = 0x02D8	/* Dword offset 0_B6 */;
pub const DP_FRGD_CLR_MIX: u32 = 0x02DC	/* Dword offset 0_B7 */;
pub const DP_FRGD_BKGD_CLR: u32 = 0x02E0	/* Dword offset 0_B8 */;

/* Draw Engine Destination Trajectory */
pub const DST_X_Y: u32 = 0x02E8	/* Dword offset 0_BA */;
pub const DST_WIDTH_HEIGHT: u32 = 0x02EC	/* Dword offset 0_BB */;

/* Data Path */
pub const USR_DST_PICTH: u32 = 0x02F0	/* Dword offset 0_BC */;
pub const DP_SET_GUI_ENGINE2: u32 = 0x02F8	/* Dword offset 0_BE */;
pub const DP_SET_GUI_ENGINE: u32 = 0x02FC	/* Dword offset 0_BF */;

/* Color Compare */
pub const CLR_CMP_CLR: u32 = 0x0300	/* Dword offset 0_C0 */;
pub const CLR_CMP_MASK: u32 = 0x0304	/* Dword offset 0_C1 */;
pub const CLR_CMP_CNTL: u32 = 0x0308	/* Dword offset 0_C2 */;

/* Command FIFO */
pub const FIFO_STAT: u32 = 0x0310	/* Dword offset 0_C4 */;

pub const CONTEXT_MASK: u32 = 0x0320	/* Dword offset 0_C8 */;
pub const CONTEXT_LOAD_CNTL: u32 = 0x032C	/* Dword offset 0_CB */;

/* Engine Control */
pub const GUI_TRAJ_CNTL: u32 = 0x0330	/* Dword offset 0_CC */;

/* Engine Status/FIFO */
pub const GUI_STAT: u32 = 0x0338	/* Dword offset 0_CE */;

pub const TEX_PALETTE_INDEX: u32 = 0x0340	/* Dword offset 0_D0 */;
pub const STW_EXP: u32 = 0x0344	/* Dword offset 0_D1 */;
pub const LOG_MAX_INC: u32 = 0x0348	/* Dword offset 0_D2 */;
pub const S_X_INC: u32 = 0x034C	/* Dword offset 0_D3 */;
pub const S_Y_INC__ALIAS__: u32 = 0x0350	/* Dword offset 0_D4 */;

pub const SCALE_PITCH__ALIAS__: u32 = 0x0350	/* Dword offset 0_D4 */;

pub const S_START: u32 = 0x0354	/* Dword offset 0_D5 */;
pub const W_X_INC: u32 = 0x0358	/* Dword offset 0_D6 */;
pub const W_Y_INC: u32 = 0x035C	/* Dword offset 0_D7 */;
pub const W_START: u32 = 0x0360	/* Dword offset 0_D8 */;
pub const T_X_INC: u32 = 0x0364	/* Dword offset 0_D9 */;
pub const T_Y_INC: u32 = 0x0368	/* Dword offset 0_DA */;

pub const SECONDARY_SCALE_PITCH: u32 = 0x0368	/* Dword offset 0_DA */;

pub const T_START: u32 = 0x036C	/* Dword offset 0_DB */;
pub const TEX_SIZE_PITCH: u32 = 0x0370	/* Dword offset 0_DC */;
pub const TEX_CNTL: u32 = 0x0374	/* Dword offset 0_DD */;
pub const SECONDARY_TEX_OFFSET: u32 = 0x0378	/* Dword offset 0_DE */;
pub const TEX_PALETTE: u32 = 0x037C	/* Dword offset 0_DF */;

pub const SCALE_PITCH_BOTH: u32 = 0x0380	/* Dword offset 0_E0 */;
pub const SECONDARY_SCALE_OFF_ACC: u32 = 0x0384	/* Dword offset 0_E1 */;
pub const SCALE_OFF_ACC: u32 = 0x0388	/* Dword offset 0_E2 */;
pub const SCALE_DST_Y_X: u32 = 0x038C	/* Dword offset 0_E3 */;

/* Draw Engine Destination Trajectory */
pub const COMPOSITE_SHADOW_ID: u32 = 0x0398	/* Dword offset 0_E6 */;

pub const SECONDARY_SCALE_X_INC: u32 = 0x039C	/* Dword offset 0_E7 */;

pub const SPECULAR_RED_X_INC: u32 = 0x039C	/* Dword offset 0_E7 */;
pub const SPECULAR_RED_Y_INC: u32 = 0x03A0	/* Dword offset 0_E8 */;
pub const SPECULAR_RED_START: u32 = 0x03A4	/* Dword offset 0_E9 */;

pub const SECONDARY_SCALE_HACC: u32 = 0x03A4	/* Dword offset 0_E9 */;

pub const SPECULAR_GREEN_X_INC: u32 = 0x03A8	/* Dword offset 0_EA */;
pub const SPECULAR_GREEN_Y_INC: u32 = 0x03AC	/* Dword offset 0_EB */;
pub const SPECULAR_GREEN_START: u32 = 0x03B0	/* Dword offset 0_EC */;
pub const SPECULAR_BLUE_X_INC: u32 = 0x03B4	/* Dword offset 0_ED */;
pub const SPECULAR_BLUE_Y_INC: u32 = 0x03B8	/* Dword offset 0_EE */;
pub const SPECULAR_BLUE_START: u32 = 0x03BC	/* Dword offset 0_EF */;

pub const SCALE_X_INC__ALIAS__: u32 = 0x03C0	/* Dword offset 0_F0 */;

pub const RED_X_INC__ALIAS__: u32 = 0x03C0	/* Dword offset 0_F0 */;
pub const RED_Y_INC: u32 = 0x03C4	/* Dword offset 0_F1 */;
pub const RED_START: u32 = 0x03C8	/* Dword offset 0_F2 */;

pub const SCALE_HACC: u32 = 0x03C8	/* Dword offset 0_F2 */;
pub const SCALE_Y_INC__ALIAS__: u32 = 0x03CC	/* Dword offset 0_F3 */;

pub const GREEN_X_INC__ALIAS__: u32 = 0x03CC	/* Dword offset 0_F3 */;
pub const GREEN_Y_INC: u32 = 0x03D0	/* Dword offset 0_F4 */;

pub const SECONDARY_SCALE_Y_INC: u32 = 0x03D0	/* Dword offset 0_F4 */;
pub const SECONDARY_SCALE_VACC: u32 = 0x03D4	/* Dword offset 0_F5 */;

pub const GREEN_START: u32 = 0x03D4	/* Dword offset 0_F5 */;
pub const BLUE_X_INC: u32 = 0x03D8	/* Dword offset 0_F6 */;
pub const BLUE_Y_INC: u32 = 0x03DC	/* Dword offset 0_F7 */;
pub const BLUE_START: u32 = 0x03E0	/* Dword offset 0_F8 */;
pub const Z_X_INC: u32 = 0x03E4	/* Dword offset 0_F9 */;
pub const Z_Y_INC: u32 = 0x03E8	/* Dword offset 0_FA */;
pub const Z_START: u32 = 0x03EC	/* Dword offset 0_FB */;
pub const ALPHA_X_INC: u32 = 0x03F0	/* Dword offset 0_FC */;
pub const FOG_X_INC: u32 = 0x03F0	/* Dword offset 0_FC */;
pub const ALPHA_Y_INC: u32 = 0x03F4	/* Dword offset 0_FD */;
pub const FOG_Y_INC: u32 = 0x03F4	/* Dword offset 0_FD */;
pub const ALPHA_START: u32 = 0x03F8	/* Dword offset 0_FE */;
pub const FOG_START: u32 = 0x03F8	/* Dword offset 0_FE */;

pub const OVERLAY_Y_X_START: u32 = 0x0400	/* Dword offset 1_00 */;
pub const OVERLAY_Y_X_END: u32 = 0x0404	/* Dword offset 1_01 */;
pub const OVERLAY_VIDEO_KEY_CLR: u32 = 0x0408	/* Dword offset 1_02 */;
pub const OVERLAY_VIDEO_KEY_MSK: u32 = 0x040C	/* Dword offset 1_03 */;
pub const OVERLAY_GRAPHICS_KEY_CLR: u32 = 0x0410	/* Dword offset 1_04 */;
pub const OVERLAY_GRAPHICS_KEY_MSK: u32 = 0x0414	/* Dword offset 1_05 */;
pub const OVERLAY_KEY_CNTL: u32 = 0x0418	/* Dword offset 1_06 */;

pub const OVERLAY_SCALE_INC: u32 = 0x0420	/* Dword offset 1_08 */;
pub const OVERLAY_SCALE_CNTL: u32 = 0x0424	/* Dword offset 1_09 */;
pub const SCALER_HEIGHT_WIDTH: u32 = 0x0428	/* Dword offset 1_0A */;
pub const SCALER_TEST: u32 = 0x042C	/* Dword offset 1_0B */;
pub const SCALER_BUF0_OFFSET: u32 = 0x0434	/* Dword offset 1_0D */;
pub const SCALER_BUF1_OFFSET: u32 = 0x0438	/* Dword offset 1_0E */;
pub const SCALE_BUF_PITCH: u32 = 0x043C	/* Dword offset 1_0F */;

pub const CAPTURE_START_END: u32 = 0x0440	/* Dword offset 1_10 */;
pub const CAPTURE_X_WIDTH: u32 = 0x0444	/* Dword offset 1_11 */;
pub const VIDEO_FORMAT: u32 = 0x0448	/* Dword offset 1_12 */;
pub const VBI_START_END: u32 = 0x044C	/* Dword offset 1_13 */;
pub const CAPTURE_CONFIG: u32 = 0x0450	/* Dword offset 1_14 */;
pub const TRIG_CNTL: u32 = 0x0454	/* Dword offset 1_15 */;

pub const OVERLAY_EXCLUSIVE_HORZ: u32 = 0x0458	/* Dword offset 1_16 */;
pub const OVERLAY_EXCLUSIVE_VERT: u32 = 0x045C	/* Dword offset 1_17 */;

pub const VAL_WIDTH: u32 = 0x0460	/* Dword offset 1_18 */;
pub const CAPTURE_DEBUG: u32 = 0x0464	/* Dword offset 1_19 */;
pub const VIDEO_SYNC_TEST: u32 = 0x0468	/* Dword offset 1_1A */;

/* GenLocking */
pub const SNAPSHOT_VH_COUNTS: u32 = 0x0470	/* Dword offset 1_1C */;
pub const SNAPSHOT_F_COUNT: u32 = 0x0474	/* Dword offset 1_1D */;
pub const N_VIF_COUNT: u32 = 0x0478	/* Dword offset 1_1E */;
pub const SNAPSHOT_VIF_COUNT: u32 = 0x047C	/* Dword offset 1_1F */;

pub const CAPTURE_BUF0_OFFSET: u32 = 0x0480	/* Dword offset 1_20 */;
pub const CAPTURE_BUF1_OFFSET: u32 = 0x0484	/* Dword offset 1_21 */;
pub const CAPTURE_BUF_PITCH: u32 = 0x0488	/* Dword offset 1_22 */;

/* GenLocking */
pub const SNAPSHOT2_VH_COUNTS: u32 = 0x04B0	/* Dword offset 1_2C */;
pub const SNAPSHOT2_F_COUNT: u32 = 0x04B4	/* Dword offset 1_2D */;
pub const N_VIF2_COUNT: u32 = 0x04B8	/* Dword offset 1_2E */;
pub const SNAPSHOT2_VIF_COUNT: u32 = 0x04BC	/* Dword offset 1_2F */;

pub const MPP_CONFIG: u32 = 0x04C0	/* Dword offset 1_30 */;
pub const MPP_STROBE_SEQ: u32 = 0x04C4	/* Dword offset 1_31 */;
pub const MPP_ADDR: u32 = 0x04C8	/* Dword offset 1_32 */;
pub const MPP_DATA: u32 = 0x04CC	/* Dword offset 1_33 */;
pub const TVO_CNTL: u32 = 0x0500	/* Dword offset 1_40 */;

/* Test and Debug */
pub const CRT_HORZ_VERT_LOAD: u32 = 0x0544	/* Dword offset 1_51 */;

/* AGP */
pub const AGP_BASE: u32 = 0x0548	/* Dword offset 1_52 */;
pub const AGP_CNTL: u32 = 0x054C	/* Dword offset 1_53 */;

pub const SCALER_COLOUR_CNTL: u32 = 0x0550	/* Dword offset 1_54 */;
pub const SCALER_H_COEFF0: u32 = 0x0554	/* Dword offset 1_55 */;
pub const SCALER_H_COEFF1: u32 = 0x0558	/* Dword offset 1_56 */;
pub const SCALER_H_COEFF2: u32 = 0x055C	/* Dword offset 1_57 */;
pub const SCALER_H_COEFF3: u32 = 0x0560	/* Dword offset 1_58 */;
pub const SCALER_H_COEFF4: u32 = 0x0564	/* Dword offset 1_59 */;

/* Command FIFO */
pub const GUI_CMDFIFO_DEBUG: u32 = 0x0570	/* Dword offset 1_5C */;
pub const GUI_CMDFIFO_DATA: u32 = 0x0574	/* Dword offset 1_5D */;
pub const GUI_CNTL: u32 = 0x0578	/* Dword offset 1_5E */;

/* Bus Mastering */
pub const BM_FRAME_BUF_OFFSET: u32 = 0x0580	/* Dword offset 1_60 */;
pub const BM_SYSTEM_MEM_ADDR: u32 = 0x0584	/* Dword offset 1_61 */;
pub const BM_COMMAND: u32 = 0x0588	/* Dword offset 1_62 */;
pub const BM_STATUS: u32 = 0x058C	/* Dword offset 1_63 */;
pub const BM_GUI_TABLE: u32 = 0x05B8	/* Dword offset 1_6E */;
pub const BM_SYSTEM_TABLE: u32 = 0x05BC	/* Dword offset 1_6F */;

pub const SCALER_BUF0_OFFSET_U: u32 = 0x05D4	/* Dword offset 1_75 */;
pub const SCALER_BUF0_OFFSET_V: u32 = 0x05D8	/* Dword offset 1_76 */;
pub const SCALER_BUF1_OFFSET_U: u32 = 0x05DC	/* Dword offset 1_77 */;
pub const SCALER_BUF1_OFFSET_V: u32 = 0x05E0	/* Dword offset 1_78 */;

/* Setup Engine */
pub const VERTEX_1_S: u32 = 0x0640	/* Dword offset 1_90 */;
pub const VERTEX_1_T: u32 = 0x0644	/* Dword offset 1_91 */;
pub const VERTEX_1_W: u32 = 0x0648	/* Dword offset 1_92 */;
pub const VERTEX_1_SPEC_ARGB: u32 = 0x064C	/* Dword offset 1_93 */;
pub const VERTEX_1_Z: u32 = 0x0650	/* Dword offset 1_94 */;
pub const VERTEX_1_ARGB: u32 = 0x0654	/* Dword offset 1_95 */;
pub const VERTEX_1_X_Y: u32 = 0x0658	/* Dword offset 1_96 */;
pub const ONE_OVER_AREA: u32 = 0x065C	/* Dword offset 1_97 */;
pub const VERTEX_2_S: u32 = 0x0660	/* Dword offset 1_98 */;
pub const VERTEX_2_T: u32 = 0x0664	/* Dword offset 1_99 */;
pub const VERTEX_2_W: u32 = 0x0668	/* Dword offset 1_9A */;
pub const VERTEX_2_SPEC_ARGB: u32 = 0x066C	/* Dword offset 1_9B */;
pub const VERTEX_2_Z: u32 = 0x0670	/* Dword offset 1_9C */;
pub const VERTEX_2_ARGB: u32 = 0x0674	/* Dword offset 1_9D */;
pub const VERTEX_2_X_Y: u32 = 0x0678	/* Dword offset 1_9E */;
pub const ONE_OVER_AREA: u32 = 0x065C	/* Dword offset 1_9F */;
pub const VERTEX_3_S: u32 = 0x0680	/* Dword offset 1_A0 */;
pub const VERTEX_3_T: u32 = 0x0684	/* Dword offset 1_A1 */;
pub const VERTEX_3_W: u32 = 0x0688	/* Dword offset 1_A2 */;
pub const VERTEX_3_SPEC_ARGB: u32 = 0x068C	/* Dword offset 1_A3 */;
pub const VERTEX_3_Z: u32 = 0x0690	/* Dword offset 1_A4 */;
pub const VERTEX_3_ARGB: u32 = 0x0694	/* Dword offset 1_A5 */;
pub const VERTEX_3_X_Y: u32 = 0x0698	/* Dword offset 1_A6 */;
pub const ONE_OVER_AREA: u32 = 0x065C	/* Dword offset 1_A7 */;
pub const VERTEX_1_S: u32 = 0x0640	/* Dword offset 1_AB */;
pub const VERTEX_1_T: u32 = 0x0644	/* Dword offset 1_AC */;
pub const VERTEX_1_W: u32 = 0x0648	/* Dword offset 1_AD */;
pub const VERTEX_2_S: u32 = 0x0660	/* Dword offset 1_AE */;
pub const VERTEX_2_T: u32 = 0x0664	/* Dword offset 1_AF */;
pub const VERTEX_2_W: u32 = 0x0668	/* Dword offset 1_B0 */;
pub const VERTEX_3_SECONDARY_S: u32 = 0x06C0	/* Dword offset 1_B0 */;
pub const VERTEX_3_S: u32 = 0x0680	/* Dword offset 1_B1 */;
pub const VERTEX_3_SECONDARY_T: u32 = 0x06C4	/* Dword offset 1_B1 */;
pub const VERTEX_3_T: u32 = 0x0684	/* Dword offset 1_B2 */;
pub const VERTEX_3_SECONDARY_W: u32 = 0x06C8	/* Dword offset 1_B2 */;
pub const VERTEX_3_W: u32 = 0x0688	/* Dword offset 1_B3 */;
pub const VERTEX_1_SPEC_ARGB: u32 = 0x064C	/* Dword offset 1_B4 */;
pub const VERTEX_2_SPEC_ARGB: u32 = 0x066C	/* Dword offset 1_B5 */;
pub const VERTEX_3_SPEC_ARGB: u32 = 0x068C	/* Dword offset 1_B6 */;
pub const VERTEX_1_Z: u32 = 0x0650	/* Dword offset 1_B7 */;
pub const VERTEX_2_Z: u32 = 0x0670	/* Dword offset 1_B8 */;
pub const VERTEX_3_Z: u32 = 0x0690	/* Dword offset 1_B9 */;
pub const VERTEX_1_ARGB: u32 = 0x0654	/* Dword offset 1_BA */;
pub const VERTEX_2_ARGB: u32 = 0x0674	/* Dword offset 1_BB */;
pub const VERTEX_3_ARGB: u32 = 0x0694	/* Dword offset 1_BC */;
pub const VERTEX_1_X_Y: u32 = 0x0658	/* Dword offset 1_BD */;
pub const VERTEX_2_X_Y: u32 = 0x0678	/* Dword offset 1_BE */;
pub const VERTEX_3_X_Y: u32 = 0x0698	/* Dword offset 1_BF */;
pub const ONE_OVER_AREA_UC: u32 = 0x0700	/* Dword offset 1_C0 */;
pub const SETUP_CNTL: u32 = 0x0704	/* Dword offset 1_C1 */;
pub const VERTEX_1_SECONDARY_S: u32 = 0x0728	/* Dword offset 1_CA */;
pub const VERTEX_1_SECONDARY_T: u32 = 0x072C	/* Dword offset 1_CB */;
pub const VERTEX_1_SECONDARY_W: u32 = 0x0730	/* Dword offset 1_CC */;
pub const VERTEX_2_SECONDARY_S: u32 = 0x0734	/* Dword offset 1_CD */;
pub const VERTEX_2_SECONDARY_T: u32 = 0x0738	/* Dword offset 1_CE */;
pub const VERTEX_2_SECONDARY_W: u32 = 0x073C	/* Dword offset 1_CF */;


pub const GTC_3D_RESET_DELAY: u32 = 3	/* 3D engine reset delay in ms */;

/* CRTC control values (mostly CRTC_GEN_CNTL) */

pub const CRTC_H_SYNC_NEG: u32 = 0x00200000;
pub const CRTC_V_SYNC_NEG: u32 = 0x00200000;

pub const CRTC_DBL_SCAN_EN: u32 = 0x00000001;
pub const CRTC_INTERLACE_EN: u32 = 0x00000002;
pub const CRTC_HSYNC_DIS: u32 = 0x00000004;
pub const CRTC_VSYNC_DIS: u32 = 0x00000008;
pub const CRTC_CSYNC_EN: u32 = 0x00000010;
pub const CRTC_PIX_BY_2_EN: u32 = 0x00000020	/* unused on RAGE */;
pub const CRTC_DISPLAY_DIS: u32 = 0x00000040;
pub const CRTC_VGA_XOVERSCAN: u32 = 0x00000080;

pub const CRTC_PIX_WIDTH_MASK: u32 = 0x00000700;
pub const CRTC_PIX_WIDTH_4BPP: u32 = 0x00000100;
pub const CRTC_PIX_WIDTH_8BPP: u32 = 0x00000200;
pub const CRTC_PIX_WIDTH_15BPP: u32 = 0x00000300;
pub const CRTC_PIX_WIDTH_16BPP: u32 = 0x00000400;
pub const CRTC_PIX_WIDTH_24BPP: u32 = 0x00000500;
pub const CRTC_PIX_WIDTH_32BPP: u32 = 0x00000600;

pub const CRTC_BYTE_PIX_ORDER: u32 = 0x00000800;
pub const CRTC_PIX_ORDER_MSN_LSN: u32 = 0x00000000;
pub const CRTC_PIX_ORDER_LSN_MSN: u32 = 0x00000800;

pub const CRTC_VSYNC_INT_EN: u32 = 0x00001000ul	/* XC/XL */;
pub const CRTC_VSYNC_INT: u32 = 0x00002000ul	/* XC/XL */;
pub const CRTC_FIFO_OVERFILL: u32 = 0x0000c000ul	/* VT/GT */;
pub const CRTC2_VSYNC_INT_EN: u32 = 0x00004000ul	/* XC/XL */;
pub const CRTC2_VSYNC_INT: u32 = 0x00008000ul	/* XC/XL */;

pub const CRTC_FIFO_LWM: u32 = 0x000f0000;
pub const CRTC_HVSYNC_IO_DRIVE: u32 = 0x00010000	/* XC/XL */;
pub const CRTC2_PIX_WIDTH: u32 = 0x000e0000	/* LTPro */;

pub const CRTC_VGA_128KAP_PAGING: u32 = 0x00100000;
pub const CRTC_VFC_SYNC_TRISTATE: u32 = 0x00200000	/* VTB/GTB/LT */;
pub const CRTC2_EN: u32 = 0x00200000	/* LTPro */;
pub const CRTC_LOCK_REGS: u32 = 0x00400000;
pub const CRTC_SYNC_TRISTATE: u32 = 0x00800000;

pub const CRTC_EXT_DISP_EN: u32 = 0x01000000;
pub const CRTC_EN: u32 = 0x02000000;
pub const CRTC_DISP_REQ_EN: u32 = 0x04000000;
pub const CRTC_VGA_LINEAR: u32 = 0x08000000;
pub const CRTC_VSYNC_FALL_EDGE: u32 = 0x10000000;
pub const CRTC_VGA_TEXT_132: u32 = 0x20000000;
pub const CRTC_CNT_EN: u32 = 0x40000000;
pub const CRTC_CUR_B_TEST: u32 = 0x80000000;

pub const CRTC_CRNT_VLINE: u32 = 0x07f00000;

pub const CRTC_PRESERVED_MASK: u32 = 0x0001f000;

pub const CRTC_VBLANK: u32 = 0x00000001;
pub const CRTC_VBLANK_INT_EN: u32 = 0x00000002;
pub const CRTC_VBLANK_INT: u32 = 0x00000004;
pub const CRTC_VBLANK_INT_AK: u32 = CRTC_VBLANK_INT;
pub const CRTC_VLINE_INT_EN: u32 = 0x00000008;
pub const CRTC_VLINE_INT: u32 = 0x00000010;
pub const CRTC_VLINE_INT_AK: u32 = CRTC_VLINE_INT;
pub const CRTC_VLINE_SYNC: u32 = 0x00000020;
pub const CRTC_FRAME: u32 = 0x00000040;
pub const SNAPSHOT_INT_EN: u32 = 0x00000080;
pub const SNAPSHOT_INT: u32 = 0x00000100;
pub const SNAPSHOT_INT_AK: u32 = SNAPSHOT_INT;
pub const I2C_INT_EN: u32 = 0x00000200;
pub const I2C_INT: u32 = 0x00000400;
pub const I2C_INT_AK: u32 = I2C_INT;
pub const CRTC2_VBLANK: u32 = 0x00000800;
pub const CRTC2_VBLANK_INT_EN: u32 = 0x00001000;
pub const CRTC2_VBLANK_INT: u32 = 0x00002000;
pub const CRTC2_VBLANK_INT_AK: u32 = CRTC2_VBLANK_INT;
pub const CRTC2_VLINE_INT_EN: u32 = 0x00004000;
pub const CRTC2_VLINE_INT: u32 = 0x00008000;
pub const CRTC2_VLINE_INT_AK: u32 = CRTC2_VLINE_INT;
pub const CAPBUF0_INT_EN: u32 = 0x00010000;
pub const CAPBUF0_INT: u32 = 0x00020000;
pub const CAPBUF0_INT_AK: u32 = CAPBUF0_INT;
pub const CAPBUF1_INT_EN: u32 = 0x00040000;
pub const CAPBUF1_INT: u32 = 0x00080000;
pub const CAPBUF1_INT_AK: u32 = CAPBUF1_INT;
pub const OVERLAY_EOF_INT_EN: u32 = 0x00100000;
pub const OVERLAY_EOF_INT: u32 = 0x00200000;
pub const OVERLAY_EOF_INT_AK: u32 = OVERLAY_EOF_INT;
pub const ONESHOT_CAP_INT_EN: u32 = 0x00400000;
pub const ONESHOT_CAP_INT: u32 = 0x00800000;
pub const ONESHOT_CAP_INT_AK: u32 = ONESHOT_CAP_INT;
pub const BUSMASTER_EOL_INT_EN: u32 = 0x01000000;
pub const BUSMASTER_EOL_INT: u32 = 0x02000000;
pub const BUSMASTER_EOL_INT_AK: u32 = BUSMASTER_EOL_INT;
pub const GP_INT_EN: u32 = 0x04000000;
pub const GP_INT: u32 = 0x08000000;
pub const GP_INT_AK: u32 = GP_INT;
pub const CRTC2_VLINE_SYNC: u32 = 0x10000000;
pub const SNAPSHOT2_INT_EN: u32 = 0x20000000;
pub const SNAPSHOT2_INT: u32 = 0x40000000;
pub const SNAPSHOT2_INT_AK: u32 = SNAPSHOT2_INT;
pub const VBLANK_BIT2_INT: u32 = 0x80000000;
pub const VBLANK_BIT2_INT_AK: u32 = VBLANK_BIT2_INT;

pub const CRTC_INT_EN_MASK: u32 = (CRTC_VBLANK_INT_EN |					 CRTC_VLINE_INT_EN |					 SNAPSHOT_INT_EN |					 I2C_INT_EN |						 CRTC2_VBLANK_INT_EN |					 CRTC2_VLINE_INT_EN |					 CAPBUF0_INT_EN |					 CAPBUF1_INT_EN |					 OVERLAY_EOF_INT_EN |					 ONESHOT_CAP_INT_EN |					 BUSMASTER_EOL_INT_EN |					 GP_INT_EN |						 SNAPSHOT2_INT_EN);

/* DAC control values */

pub const DAC_EXT_SEL_RS2: u32 = 0x01;
pub const DAC_EXT_SEL_RS3: u32 = 0x02;
pub const DAC_8BIT_EN: u32 = 0x00000100;
pub const DAC_PIX_DLY_MASK: u32 = 0x00000600;
pub const DAC_PIX_DLY_0NS: u32 = 0x00000000;
pub const DAC_PIX_DLY_2NS: u32 = 0x00000200;
pub const DAC_PIX_DLY_4NS: u32 = 0x00000400;
pub const DAC_BLANK_ADJ_MASK: u32 = 0x00001800;
pub const DAC_BLANK_ADJ_0: u32 = 0x00000000;
pub const DAC_BLANK_ADJ_1: u32 = 0x00000800;
pub const DAC_BLANK_ADJ_2: u32 = 0x00001000;

/* DAC control values (my source XL/XC Register reference) */
pub const DAC_OUTPUT_MASK: u32 = 0x00000001  /* 0 - PAL, 1 - NTSC */;
pub const DAC_MISTERY_BIT: u32 = 0x00000002  /* PS2 ? RS343 ?, EXTRA_BRIGHT for GT */;
pub const DAC_BLANKING: u32 = 0x00000004;
pub const DAC_CMP_DISABLE: u32 = 0x00000008;
pub const DAC1_CLK_SEL: u32 = 0x00000010;
pub const PALETTE_ACCESS_CNTL: u32 = 0x00000020;
pub const PALETTE2_SNOOP_EN: u32 = 0x00000040;
pub const DAC_CMP_OUTPUT: u32 = 0x00000080 /* read only */;
/* #define DAC_8BIT_EN is ok */
pub const CRT_SENSE: u32 = 0x00000800 /* read only */;
pub const CRT_DETECTION_ON: u32 = 0x00001000;
pub const DAC_VGA_ADR_EN: u32 = 0x00002000;
pub const DAC_FEA_CON_EN: u32 = 0x00004000;
pub const DAC_PDWN: u32 = 0x00008000;
pub const DAC_TYPE_MASK: u32 = 0x00070000 /* read only */;



/* Mix control values */

pub const MIX_NOT_DST: u32 = 0x0000;
pub const MIX_0: u32 = 0x0001;
pub const MIX_1: u32 = 0x0002;
pub const MIX_DST: u32 = 0x0003;
pub const MIX_NOT_SRC: u32 = 0x0004;
pub const MIX_XOR: u32 = 0x0005;
pub const MIX_XNOR: u32 = 0x0006;
pub const MIX_SRC: u32 = 0x0007;
pub const MIX_NAND: u32 = 0x0008;
pub const MIX_NOT_SRC_OR_DST: u32 = 0x0009;
pub const MIX_SRC_OR_NOT_DST: u32 = 0x000a;
pub const MIX_OR: u32 = 0x000b;
pub const MIX_AND: u32 = 0x000c;
pub const MIX_SRC_AND_NOT_DST: u32 = 0x000d;
pub const MIX_NOT_SRC_AND_DST: u32 = 0x000e;
pub const MIX_NOR: u32 = 0x000f;

/* Maximum engine dimensions */
pub const ENGINE_MIN_X: u32 = 0;
pub const ENGINE_MIN_Y: u32 = 0;
pub const ENGINE_MAX_X: u32 = 4095;
pub const ENGINE_MAX_Y: u32 = 16383;

/* Mach64 engine bit constants - these are typically ORed together */

/* BUS_CNTL register constants */
pub const BUS_APER_REG_DIS: u32 = 0x00000010;
pub const BUS_FIFO_ERR_ACK: u32 = 0x00200000;
pub const BUS_HOST_ERR_ACK: u32 = 0x00800000;

/* GEN_TEST_CNTL register constants */
pub const GEN_OVR_OUTPUT_EN: u32 = 0x20;
pub const HWCURSOR_ENABLE: u32 = 0x80;
pub const GUI_ENGINE_ENABLE: u32 = 0x100;
pub const BLOCK_WRITE_ENABLE: u32 = 0x200;

/* DSP_CONFIG register constants */
pub const DSP_XCLKS_PER_QW: u32 = 0x00003fff;
pub const DSP_LOOP_LATENCY: u32 = 0x000f0000;
pub const DSP_PRECISION: u32 = 0x00700000;

/* DSP_ON_OFF register constants */
pub const DSP_OFF: u32 = 0x000007ff;
pub const DSP_ON: u32 = 0x07ff0000;
pub const VGA_DSP_OFF: u32 = DSP_OFF;
pub const VGA_DSP_ON: u32 = DSP_ON;
pub const VGA_DSP_XCLKS_PER_QW: u32 = DSP_XCLKS_PER_QW;

/* PLL register indices and fields */
pub const MPLL_CNTL: u32 = 0x00;
pub const PLL_PC_GAIN: u32 = 0x07;
pub const PLL_VC_GAIN: u32 = 0x18;
pub const PLL_DUTY_CYC: u32 = 0xE0;
pub const VPLL_CNTL: u32 = 0x01;
pub const PLL_REF_DIV: u32 = 0x02;
pub const PLL_GEN_CNTL: u32 = 0x03;
pub const PLL_OVERRIDE: u32 = 0x01	/* PLL_SLEEP */;
pub const PLL_MCLK_RST: u32 = 0x02	/* PLL_MRESET */;
pub const OSC_EN: u32 = 0x04;
pub const EXT_CLK_EN: u32 = 0x08;
pub const FORCE_DCLK_TRI_STATE: u32 = 0x08    /* VT4 -> */;
pub const MCLK_SRC_SEL: u32 = 0x70;
pub const EXT_CLK_CNTL: u32 = 0x80;
pub const DLL_PWDN: u32 = 0x80    /* VT4 -> */;
pub const MCLK_FB_DIV: u32 = 0x04;
pub const PLL_VCLK_CNTL: u32 = 0x05;
pub const PLL_VCLK_SRC_SEL: u32 = 0x03;
pub const PLL_VCLK_RST: u32 = 0x04;
pub const PLL_VCLK_INVERT: u32 = 0x08;
pub const VCLK_POST_DIV: u32 = 0x06;
pub const VCLK0_POST: u32 = 0x03;
pub const VCLK1_POST: u32 = 0x0C;
pub const VCLK2_POST: u32 = 0x30;
pub const VCLK3_POST: u32 = 0xC0;
pub const VCLK0_FB_DIV: u32 = 0x07;
pub const VCLK1_FB_DIV: u32 = 0x08;
pub const VCLK2_FB_DIV: u32 = 0x09;
pub const VCLK3_FB_DIV: u32 = 0x0A;
pub const PLL_EXT_CNTL: u32 = 0x0B;
pub const PLL_XCLK_MCLK_RATIO: u32 = 0x03;
pub const PLL_XCLK_SRC_SEL: u32 = 0x07;
pub const PLL_MFB_TIMES_4_2B: u32 = 0x08;
pub const PLL_VCLK0_XDIV: u32 = 0x10;
pub const PLL_VCLK1_XDIV: u32 = 0x20;
pub const PLL_VCLK2_XDIV: u32 = 0x40;
pub const PLL_VCLK3_XDIV: u32 = 0x80;
pub const DLL_CNTL: u32 = 0x0C;
pub const DLL1_CNTL: u32 = 0x0C;
pub const VFC_CNTL: u32 = 0x0D;
pub const PLL_TEST_CNTL: u32 = 0x0E;
pub const PLL_TEST_COUNT: u32 = 0x0F;
pub const LVDS_CNTL0: u32 = 0x10;
pub const LVDS_CNTL1: u32 = 0x11;
pub const AGP1_CNTL: u32 = 0x12;
pub const AGP2_CNTL: u32 = 0x13;
pub const DLL2_CNTL: u32 = 0x14;
pub const SCLK_FB_DIV: u32 = 0x15;
pub const SPLL_CNTL1: u32 = 0x16;
pub const SPLL_CNTL2: u32 = 0x17;
pub const APLL_STRAPS: u32 = 0x18;
pub const EXT_VPLL_CNTL: u32 = 0x19;
pub const EXT_VPLL_EN: u32 = 0x04;
pub const EXT_VPLL_VGA_EN: u32 = 0x08;
pub const EXT_VPLL_INSYNC: u32 = 0x10;
pub const EXT_VPLL_REF_DIV: u32 = 0x1A;
pub const EXT_VPLL_FB_DIV: u32 = 0x1B;
pub const EXT_VPLL_MSB: u32 = 0x1C;
pub const HTOTAL_CNTL: u32 = 0x1D;
pub const BYTE_CLK_CNTL: u32 = 0x1E;
pub const TV_PLL_CNTL1: u32 = 0x1F;
pub const TV_PLL_CNTL2: u32 = 0x20;
pub const TV_PLL_CNTL: u32 = 0x21;
pub const EXT_TV_PLL: u32 = 0x22;
pub const V2PLL_CNTL: u32 = 0x23;
pub const PLL_V2CLK_CNTL: u32 = 0x24;
pub const EXT_V2PLL_REF_DIV: u32 = 0x25;
pub const EXT_V2PLL_FB_DIV: u32 = 0x26;
pub const EXT_V2PLL_MSB: u32 = 0x27;
pub const HTOTAL2_CNTL: u32 = 0x28;
pub const PLL_YCLK_CNTL: u32 = 0x29;
pub const PM_DYN_CLK_CNTL: u32 = 0x2A;

/* CNFG_CNTL register constants */
pub const APERTURE_4M_ENABLE: u32 = 1;
pub const APERTURE_8M_ENABLE: u32 = 2;
pub const VGA_APERTURE_ENABLE: u32 = 4;

/* CNFG_STAT0 register constants (GX, CX) */
pub const CFG_BUS_TYPE: u32 = 0x00000007;
pub const CFG_MEM_TYPE: u32 = 0x00000038;
pub const CFG_INIT_DAC_TYPE: u32 = 0x00000e00;

/* CNFG_STAT0 register constants (CT, ET, VT) */
pub const CFG_MEM_TYPE_xT: u32 = 0x00000007;

pub const ISA: u32 = 0;
pub const EISA: u32 = 1;
pub const LOCAL_BUS: u32 = 6;
pub const PCI: u32 = 7;

/* Memory types for GX, CX */
pub const DRAMx4: u32 = 0;
pub const VRAMx16: u32 = 1;
pub const VRAMx16ssr: u32 = 2;
pub const DRAMx16: u32 = 3;
pub const GraphicsDRAMx16: u32 = 4;
pub const EnhancedVRAMx16: u32 = 5;
pub const EnhancedVRAMx16ssr: u32 = 6;

/* Memory types for CT, ET, VT, GT */
pub const DRAM: u32 = 1;
pub const EDO: u32 = 2;
pub const PSEUDO_EDO: u32 = 3;
pub const SDRAM: u32 = 4;
pub const SGRAM: u32 = 5;
pub const WRAM: u32 = 6;
pub const SDRAM32: u32 = 6;

pub const DAC_INTERNAL: u32 = 0x00;
pub const DAC_IBMRGB514: u32 = 0x01;
pub const DAC_ATI68875: u32 = 0x02;
pub const DAC_TVP3026_A: u32 = 0x72;
pub const DAC_BT476: u32 = 0x03;
pub const DAC_BT481: u32 = 0x04;
pub const DAC_ATT20C491: u32 = 0x14;
pub const DAC_SC15026: u32 = 0x24;
pub const DAC_MU9C1880: u32 = 0x34;
pub const DAC_IMSG174: u32 = 0x44;
pub const DAC_ATI68860_B: u32 = 0x05;
pub const DAC_ATI68860_C: u32 = 0x15;
pub const DAC_TVP3026_B: u32 = 0x75;
pub const DAC_STG1700: u32 = 0x06;
pub const DAC_ATT498: u32 = 0x16;
pub const DAC_STG1702: u32 = 0x07;
pub const DAC_SC15021: u32 = 0x17;
pub const DAC_ATT21C498: u32 = 0x27;
pub const DAC_STG1703: u32 = 0x37;
pub const DAC_CH8398: u32 = 0x47;
pub const DAC_ATT20C408: u32 = 0x57;

pub const CLK_ATI18818_0: u32 = 0;
pub const CLK_ATI18818_1: u32 = 1;
pub const CLK_STG1703: u32 = 2;
pub const CLK_CH8398: u32 = 3;
pub const CLK_INTERNAL: u32 = 4;
pub const CLK_ATT20C408: u32 = 5;
pub const CLK_IBMRGB514: u32 = 6;

/* MEM_CNTL register constants */
pub const MEM_SIZE_ALIAS: u32 = 0x00000007;
pub const MEM_SIZE_512K: u32 = 0x00000000;
pub const MEM_SIZE_1M: u32 = 0x00000001;
pub const MEM_SIZE_2M: u32 = 0x00000002;
pub const MEM_SIZE_4M: u32 = 0x00000003;
pub const MEM_SIZE_6M: u32 = 0x00000004;
pub const MEM_SIZE_8M: u32 = 0x00000005;
pub const MEM_SIZE_ALIAS_GTB: u32 = 0x0000000F;
pub const MEM_SIZE_2M_GTB: u32 = 0x00000003;
pub const MEM_SIZE_4M_GTB: u32 = 0x00000007;
pub const MEM_SIZE_6M_GTB: u32 = 0x00000009;
pub const MEM_SIZE_8M_GTB: u32 = 0x0000000B;
pub const MEM_BNDRY: u32 = 0x00030000;
pub const MEM_BNDRY_0K: u32 = 0x00000000;
pub const MEM_BNDRY_256K: u32 = 0x00010000;
pub const MEM_BNDRY_512K: u32 = 0x00020000;
pub const MEM_BNDRY_1M: u32 = 0x00030000;
pub const MEM_BNDRY_EN: u32 = 0x00040000;

pub const ONE_MB: u32 = 0x100000;

/* CNFG_CHIP_ID register constants */
pub const CFG_CHIP_TYPE: u32 = 0x0000FFFF;
pub const CFG_CHIP_CLASS: u32 = 0x00FF0000;
pub const CFG_CHIP_REV: u32 = 0xFF000000;
pub const CFG_CHIP_MAJOR: u32 = 0x07000000;
pub const CFG_CHIP_FND_ID: u32 = 0x38000000;
pub const CFG_CHIP_MINOR: u32 = 0xC0000000;


/* Chip IDs read from CNFG_CHIP_ID */

/* mach64GX family */
pub const GX_CHIP_ID: u32 = 0xD7	/* mach64GX (ATI888GX00) */;
pub const CX_CHIP_ID: u32 = 0x57	/* mach64CX (ATI888CX00) */;

pub const GX_PCI_ID: u32 = 0x4758	/* mach64GX (ATI888GX00) */;
pub const CX_PCI_ID: u32 = 0x4358	/* mach64CX (ATI888CX00) */;

/* mach64CT family */
pub const CT_CHIP_ID: u32 = 0x4354	/* mach64CT (ATI264CT) */;
pub const ET_CHIP_ID: u32 = 0x4554	/* mach64ET (ATI264ET) */;

/* mach64CT family / mach64VT class */
pub const VT_CHIP_ID: u32 = 0x5654	/* mach64VT (ATI264VT) */;
pub const VU_CHIP_ID: u32 = 0x5655	/* mach64VTB (ATI264VTB) */;
pub const VV_CHIP_ID: u32 = 0x5656	/* mach64VT4 (ATI264VT4) */;

/* mach64CT family / mach64GT (3D RAGE) class */
pub const LB_CHIP_ID: u32 = 0x4c42	/* RAGE LT PRO, AGP */;
pub const LD_CHIP_ID: u32 = 0x4c44	/* RAGE LT PRO */;
pub const LG_CHIP_ID: u32 = 0x4c47	/* RAGE LT */;
pub const LI_CHIP_ID: u32 = 0x4c49	/* RAGE LT PRO */;
pub const LP_CHIP_ID: u32 = 0x4c50	/* RAGE LT PRO */;
pub const LT_CHIP_ID: u32 = 0x4c54	/* RAGE LT */;

/* mach64CT family / (Rage XL) class */
pub const GR_CHIP_ID: u32 = 0x4752	/* RAGE XL, BGA, PCI33 */;
pub const GS_CHIP_ID: u32 = 0x4753	/* RAGE XL, PQFP, PCI33 */;
pub const GM_CHIP_ID: u32 = 0x474d	/* RAGE XL, BGA, AGP 1x,2x */;
pub const GN_CHIP_ID: u32 = 0x474e	/* RAGE XL, PQFP,AGP 1x,2x */;
pub const GO_CHIP_ID: u32 = 0x474f	/* RAGE XL, BGA, PCI66 */;
pub const GL_CHIP_ID: u32 = 0x474c	/* RAGE XL, PQFP, PCI66 */;

// C function-like macro retained as a Rust function below: IS_XL(id) ((id)==GR_CHIP_ID || (id)==GS_CHIP_ID || 		   (id)==GM_CHIP_ID || (id)==GN_CHIP_ID || 		   (id)==GO_CHIP_ID || (id)==GL_CHIP_ID)

pub const GT_CHIP_ID: u32 = 0x4754	/* RAGE (GT) */;
pub const GU_CHIP_ID: u32 = 0x4755	/* RAGE II/II+ (GTB) */;
pub const GV_CHIP_ID: u32 = 0x4756	/* RAGE IIC, PCI */;
pub const GW_CHIP_ID: u32 = 0x4757	/* RAGE IIC, AGP */;
pub const GZ_CHIP_ID: u32 = 0x475a	/* RAGE IIC, AGP */;
pub const GB_CHIP_ID: u32 = 0x4742	/* RAGE PRO, BGA, AGP 1x and 2x */;
pub const GD_CHIP_ID: u32 = 0x4744	/* RAGE PRO, BGA, AGP 1x only */;
pub const GI_CHIP_ID: u32 = 0x4749	/* RAGE PRO, BGA, PCI33 only */;
pub const GP_CHIP_ID: u32 = 0x4750	/* RAGE PRO, PQFP, PCI33, full 3D */;
pub const GQ_CHIP_ID: u32 = 0x4751	/* RAGE PRO, PQFP, PCI33, limited 3D */;

pub const LM_CHIP_ID: u32 = 0x4c4d	/* RAGE Mobility AGP, full function */;
pub const LN_CHIP_ID: u32 = 0x4c4e	/* RAGE Mobility AGP */;
pub const LR_CHIP_ID: u32 = 0x4c52	/* RAGE Mobility PCI, full function */;
pub const LS_CHIP_ID: u32 = 0x4c53	/* RAGE Mobility PCI */;

// C function-like macro retained as a Rust function below: IS_MOBILITY(id) ((id)==LM_CHIP_ID || (id)==LN_CHIP_ID || 			(id)==LR_CHIP_ID || (id)==LS_CHIP_ID)
/* Mach64 major ASIC revisions */
pub const MACH64_ASIC_NEC_VT_A3: u32 = 0x08;
pub const MACH64_ASIC_NEC_VT_A4: u32 = 0x48;
pub const MACH64_ASIC_SGS_VT_A4: u32 = 0x40;
pub const MACH64_ASIC_SGS_VT_B1S1: u32 = 0x01;
pub const MACH64_ASIC_SGS_GT_B1S1: u32 = 0x01;
pub const MACH64_ASIC_SGS_GT_B1S2: u32 = 0x41;
pub const MACH64_ASIC_UMC_GT_B2U1: u32 = 0x1a;
pub const MACH64_ASIC_UMC_GT_B2U2: u32 = 0x5a;
pub const MACH64_ASIC_UMC_VT_B2U3: u32 = 0x9a;
pub const MACH64_ASIC_UMC_GT_B2U3: u32 = 0x9a;
pub const MACH64_ASIC_UMC_R3B_D_P_A1: u32 = 0x1b;
pub const MACH64_ASIC_UMC_R3B_D_P_A2: u32 = 0x5b;
pub const MACH64_ASIC_UMC_R3B_D_P_A3: u32 = 0x1c;
pub const MACH64_ASIC_UMC_R3B_D_P_A4: u32 = 0x5c;

/* Mach64 foundries */
pub const MACH64_FND_SGS: u32 = 0;
pub const MACH64_FND_NEC: u32 = 1;
pub const MACH64_FND_UMC: u32 = 3;

/* Mach64 chip types */
pub const MACH64_UNKNOWN: u32 = 0;
pub const MACH64_GX: u32 = 1;
pub const MACH64_CX: u32 = 2;
pub const MACH64_CT: u32 = 3Restore;
pub const MACH64_ET: u32 = 4;
pub const MACH64_VT: u32 = 5;
pub const MACH64_GT: u32 = 6;

/* DST_CNTL register constants */
pub const DST_X_RIGHT_TO_LEFT: u32 = 0;
pub const DST_X_LEFT_TO_RIGHT: u32 = 1;
pub const DST_Y_BOTTOM_TO_TOP: u32 = 0;
pub const DST_Y_TOP_TO_BOTTOM: u32 = 2;
pub const DST_X_MAJOR: u32 = 0;
pub const DST_Y_MAJOR: u32 = 4;
pub const DST_X_TILE: u32 = 8;
pub const DST_Y_TILE: u32 = 0x10;
pub const DST_LAST_PEL: u32 = 0x20;
pub const DST_POLYGON_ENABLE: u32 = 0x40;
pub const DST_24_ROTATION_ENABLE: u32 = 0x80;

/* SRC_CNTL register constants */
pub const SRC_PATTERN_ENABLE: u32 = 1;
pub const SRC_ROTATION_ENABLE: u32 = 2;
pub const SRC_LINEAR_ENABLE: u32 = 4;
pub const SRC_BYTE_ALIGN: u32 = 8;
pub const SRC_LINE_X_RIGHT_TO_LEFT: u32 = 0;
pub const SRC_LINE_X_LEFT_TO_RIGHT: u32 = 0x10;

/* HOST_CNTL register constants */
pub const HOST_BYTE_ALIGN: u32 = 1;

/* GUI_TRAJ_CNTL register constants */
pub const PAT_MONO_8x8_ENABLE: u32 = 0x01000000;
pub const PAT_CLR_4x2_ENABLE: u32 = 0x02000000;
pub const PAT_CLR_8x1_ENABLE: u32 = 0x04000000;

/* DP_CHAIN_MASK register constants */
pub const DP_CHAIN_4BPP: u32 = 0x8888;
pub const DP_CHAIN_7BPP: u32 = 0xD2D2;
pub const DP_CHAIN_8BPP: u32 = 0x8080;
pub const DP_CHAIN_8BPP_RGB: u32 = 0x9292;
pub const DP_CHAIN_15BPP: u32 = 0x4210;
pub const DP_CHAIN_16BPP: u32 = 0x8410;
pub const DP_CHAIN_24BPP: u32 = 0x8080;
pub const DP_CHAIN_32BPP: u32 = 0x8080;

/* DP_PIX_WIDTH register constants */
pub const DST_1BPP: u32 = 0x0;
pub const DST_4BPP: u32 = 0x1;
pub const DST_8BPP: u32 = 0x2;
pub const DST_15BPP: u32 = 0x3;
pub const DST_16BPP: u32 = 0x4;
pub const DST_24BPP: u32 = 0x5;
pub const DST_32BPP: u32 = 0x6;
pub const DST_MASK: u32 = 0xF;
pub const SRC_1BPP: u32 = 0x000;
pub const SRC_4BPP: u32 = 0x100;
pub const SRC_8BPP: u32 = 0x200;
pub const SRC_15BPP: u32 = 0x300;
pub const SRC_16BPP: u32 = 0x400;
pub const SRC_24BPP: u32 = 0x500;
pub const SRC_32BPP: u32 = 0x600;
pub const SRC_MASK: u32 = 0xF00;
pub const DP_HOST_TRIPLE_EN: u32 = 0x2000;
pub const HOST_1BPP: u32 = 0x00000;
pub const HOST_4BPP: u32 = 0x10000;
pub const HOST_8BPP: u32 = 0x20000;
pub const HOST_15BPP: u32 = 0x30000;
pub const HOST_16BPP: u32 = 0x40000;
pub const HOST_24BPP: u32 = 0x50000;
pub const HOST_32BPP: u32 = 0x60000;
pub const HOST_MASK: u32 = 0xF0000;
pub const BYTE_ORDER_MSB_TO_LSB: u32 = 0;
pub const BYTE_ORDER_LSB_TO_MSB: u32 = 0x1000000;
pub const BYTE_ORDER_MASK: u32 = 0x1000000;

/* DP_MIX register constants */
pub const BKGD_MIX_NOT_D: u32 = 0;
pub const BKGD_MIX_ZERO: u32 = 1;
pub const BKGD_MIX_ONE: u32 = 2;
pub const BKGD_MIX_D: u32 = 3;
pub const BKGD_MIX_NOT_S: u32 = 4;
pub const BKGD_MIX_D_XOR_S: u32 = 5;
pub const BKGD_MIX_NOT_D_XOR_S: u32 = 6;
pub const BKGD_MIX_S: u32 = 7;
pub const BKGD_MIX_NOT_D_OR_NOT_S: u32 = 8;
pub const BKGD_MIX_D_OR_NOT_S: u32 = 9;
pub const BKGD_MIX_NOT_D_OR_S: u32 = 10;
pub const BKGD_MIX_D_OR_S: u32 = 11;
pub const BKGD_MIX_D_AND_S: u32 = 12;
pub const BKGD_MIX_NOT_D_AND_S: u32 = 13;
pub const BKGD_MIX_D_AND_NOT_S: u32 = 14;
pub const BKGD_MIX_NOT_D_AND_NOT_S: u32 = 15;
pub const BKGD_MIX_D_PLUS_S_DIV2: u32 = 0x17;
pub const FRGD_MIX_NOT_D: u32 = 0;
pub const FRGD_MIX_ZERO: u32 = 0x10000;
pub const FRGD_MIX_ONE: u32 = 0x20000;
pub const FRGD_MIX_D: u32 = 0x30000;
pub const FRGD_MIX_NOT_S: u32 = 0x40000;
pub const FRGD_MIX_D_XOR_S: u32 = 0x50000;
pub const FRGD_MIX_NOT_D_XOR_S: u32 = 0x60000;
pub const FRGD_MIX_S: u32 = 0x70000;
pub const FRGD_MIX_NOT_D_OR_NOT_S: u32 = 0x80000;
pub const FRGD_MIX_D_OR_NOT_S: u32 = 0x90000;
pub const FRGD_MIX_NOT_D_OR_S: u32 = 0xa0000;
pub const FRGD_MIX_D_OR_S: u32 = 0xb0000;
pub const FRGD_MIX_D_AND_S: u32 = 0xc0000;
pub const FRGD_MIX_NOT_D_AND_S: u32 = 0xd0000;
pub const FRGD_MIX_D_AND_NOT_S: u32 = 0xe0000;
pub const FRGD_MIX_NOT_D_AND_NOT_S: u32 = 0xf0000;
pub const FRGD_MIX_D_PLUS_S_DIV2: u32 = 0x170000;

/* DP_SRC register constants */
pub const BKGD_SRC_BKGD_CLR: u32 = 0;
pub const BKGD_SRC_FRGD_CLR: u32 = 1;
pub const BKGD_SRC_HOST: u32 = 2;
pub const BKGD_SRC_BLIT: u32 = 3;
pub const BKGD_SRC_PATTERN: u32 = 4;
pub const FRGD_SRC_BKGD_CLR: u32 = 0;
pub const FRGD_SRC_FRGD_CLR: u32 = 0x100;
pub const FRGD_SRC_HOST: u32 = 0x200;
pub const FRGD_SRC_BLIT: u32 = 0x300;
pub const FRGD_SRC_PATTERN: u32 = 0x400;
pub const MONO_SRC_ONE: u32 = 0;
pub const MONO_SRC_PATTERN: u32 = 0x10000;
pub const MONO_SRC_HOST: u32 = 0x20000;
pub const MONO_SRC_BLIT: u32 = 0x30000;

/* CLR_CMP_CNTL register constants */
pub const COMPARE_FALSE: u32 = 0;
pub const COMPARE_TRUE: u32 = 1;
pub const COMPARE_NOT_EQUAL: u32 = 4;
pub const COMPARE_EQUAL: u32 = 5;
pub const COMPARE_DESTINATION: u32 = 0;
pub const COMPARE_SOURCE: u32 = 0x1000000;

/* FIFO_STAT register constants */
pub const FIFO_ERR: u32 = 0x80000000;

/* CONTEXT_LOAD_CNTL constants */
pub const CONTEXT_NO_LOAD: u32 = 0;
pub const CONTEXT_LOAD: u32 = 0x10000;
pub const CONTEXT_LOAD_AND_DO_FILL: u32 = 0x20000;
pub const CONTEXT_LOAD_AND_DO_LINE: u32 = 0x30000;
pub const CONTEXT_EXECUTE: u32 = 0;
pub const CONTEXT_CMD_DISABLE: u32 = 0x80000000;

/* GUI_STAT register constants */
pub const ENGINE_IDLE: u32 = 0;
pub const ENGINE_BUSY: u32 = 1;
pub const SCISSOR_LEFT_FLAG: u32 = 0x10;
pub const SCISSOR_RIGHT_FLAG: u32 = 0x20;
pub const SCISSOR_TOP_FLAG: u32 = 0x40;
pub const SCISSOR_BOTTOM_FLAG: u32 = 0x80;

/* ATI VGA Extended Regsiters */
pub const sioATIEXT: u32 = 0x1ce;
pub const bioATIEXT: u32 = 0x3ce;

pub const ATI2E: u32 = 0xae;
pub const ATI32: u32 = 0xb2;
pub const ATI36: u32 = 0xb6;

/* VGA Graphics Controller Registers */
pub const R_GENMO: u32 = 0x3cc;
pub const VGAGRA: u32 = 0x3ce;
pub const GRA06: u32 = 0x06;

/* VGA Seququencer Registers */
pub const VGASEQ: u32 = 0x3c4;
pub const SEQ02: u32 = 0x02;
pub const SEQ04: u32 = 0x04;

pub const MACH64_MAX_X: u32 = ENGINE_MAX_X;
pub const MACH64_MAX_Y: u32 = ENGINE_MAX_Y;

pub const INC_X: u32 = 0x0020;
pub const INC_Y: u32 = 0x0080;

pub const RGB16_555: u32 = 0x0000;
pub const RGB16_565: u32 = 0x0040;
pub const RGB16_655: u32 = 0x0080;
pub const RGB16_664: u32 = 0x00c0;

pub const POLY_TEXT_TYPE: u32 = 0x0001;
pub const IMAGE_TEXT_TYPE: u32 = 0x0002;
pub const TEXT_TYPE_8_BIT: u32 = 0x0004;
pub const TEXT_TYPE_16_BIT: u32 = 0x0008;
pub const POLY_TEXT_TYPE_8: u32 = (POLY_TEXT_TYPE | TEXT_TYPE_8_BIT);
pub const IMAGE_TEXT_TYPE_8: u32 = (IMAGE_TEXT_TYPE | TEXT_TYPE_8_BIT);
pub const POLY_TEXT_TYPE_16: u32 = (POLY_TEXT_TYPE | TEXT_TYPE_16_BIT);
pub const IMAGE_TEXT_TYPE_16: u32 = (IMAGE_TEXT_TYPE | TEXT_TYPE_16_BIT);

pub const MACH64_NUM_CLOCKS: u32 = 16;
pub const MACH64_NUM_FREQS: u32 = 50;

/* Power Management register constants (LT & LT Pro) */
pub const PWR_MGT_ON: u32 = 0x00000001;
pub const PWR_MGT_MODE_MASK: u32 = 0x00000006;
pub const AUTO_PWR_UP: u32 = 0x00000008;
pub const USE_F32KHZ: u32 = 0x00000400;
pub const TRISTATE_MEM_EN: u32 = 0x00000800;
pub const SELF_REFRESH: u32 = 0x00000080;
pub const PWR_BLON: u32 = 0x02000000;
pub const STANDBY_NOW: u32 = 0x10000000;
pub const SUSPEND_NOW: u32 = 0x20000000;
pub const PWR_MGT_STATUS_MASK: u32 = 0xC0000000;
pub const PWR_MGT_STATUS_SUSPEND: u32 = 0x80000000;

/* PM Mode constants  */
pub const PWR_MGT_MODE_PIN: u32 = 0x00000000;
pub const PWR_MGT_MODE_REG: u32 = 0x00000002;
pub const PWR_MGT_MODE_TIMER: u32 = 0x00000004;
pub const PWR_MGT_MODE_PCI: u32 = 0x00000006;

/* LCD registers (LT Pro) */

/* LCD Index register */
pub const LCD_INDEX_MASK: u32 = 0x0000003F;
pub const LCD_DISPLAY_DIS: u32 = 0x00000100;
pub const LCD_SRC_SEL: u32 = 0x00000200;
pub const CRTC2_DISPLAY_DIS: u32 = 0x00000400;

/* LCD register indices */
pub const CNFG_PANEL: u32 = 0x00;
pub const LCD_GEN_CNTL: u32 = 0x01;
pub const DSTN_CONTROL: u32 = 0x02;
pub const HFB_PITCH_ADDR: u32 = 0x03;
pub const HORZ_STRETCHING: u32 = 0x04;
pub const VERT_STRETCHING: u32 = 0x05;
pub const EXT_VERT_STRETCH: u32 = 0x06;
pub const LT_GIO: u32 = 0x07;
pub const POWER_MANAGEMENT: u32 = 0x08;
pub const ZVGPIO: u32 = 0x09;
pub const ICON_CLR0: u32 = 0x0A;
pub const ICON_CLR1: u32 = 0x0B;
pub const ICON_OFFSET: u32 = 0x0C;
pub const ICON_HORZ_VERT_POSN: u32 = 0x0D;
pub const ICON_HORZ_VERT_OFF: u32 = 0x0E;
pub const ICON2_CLR0: u32 = 0x0F;
pub const ICON2_CLR1: u32 = 0x10;
pub const ICON2_OFFSET: u32 = 0x11;
pub const ICON2_HORZ_VERT_POSN: u32 = 0x12;
pub const ICON2_HORZ_VERT_OFF: u32 = 0x13;
pub const LCD_MISC_CNTL: u32 = 0x14;
pub const APC_CNTL: u32 = 0x1C;
pub const POWER_MANAGEMENT_2: u32 = 0x1D;
pub const ALPHA_BLENDING: u32 = 0x25;
pub const PORTRAIT_GEN_CNTL: u32 = 0x26;
pub const APC_CTRL_IO: u32 = 0x27;
pub const TEST_IO: u32 = 0x28;
pub const TEST_OUTPUTS: u32 = 0x29;
pub const DP1_MEM_ACCESS: u32 = 0x2A;
pub const DP0_MEM_ACCESS: u32 = 0x2B;
pub const DP0_DEBUG_A: u32 = 0x2C;
pub const DP0_DEBUG_B: u32 = 0x2D;
pub const DP1_DEBUG_A: u32 = 0x2E;
pub const DP1_DEBUG_B: u32 = 0x2F;
pub const DPCTRL_DEBUG_A: u32 = 0x30;
pub const DPCTRL_DEBUG_B: u32 = 0x31;
pub const MEMBLK_DEBUG: u32 = 0x32;
pub const APC_LUT_AB: u32 = 0x33;
pub const APC_LUT_CD: u32 = 0x34;
pub const APC_LUT_EF: u32 = 0x35;
pub const APC_LUT_GH: u32 = 0x36;
pub const APC_LUT_IJ: u32 = 0x37;
pub const APC_LUT_KL: u32 = 0x38;
pub const APC_LUT_MN: u32 = 0x39;
pub const APC_LUT_OP: u32 = 0x3A;

/* Values in LCD_GEN_CTRL */
pub const CRT_ON: u32 = 0x00000001ul;
pub const LCD_ON: u32 = 0x00000002ul;
pub const HORZ_DIVBY2_EN: u32 = 0x00000004ul;
pub const DONT_DS_ICON: u32 = 0x00000008ul;
pub const LOCK_8DOT: u32 = 0x00000010ul;
pub const ICON_ENABLE: u32 = 0x00000020ul;
pub const DONT_SHADOW_VPAR: u32 = 0x00000040ul;
pub const V2CLK_PM_EN: u32 = 0x00000080ul;
pub const RST_FM: u32 = 0x00000100ul;
pub const DISABLE_PCLK_RESET: u32 = 0x00000200ul	/* XC/XL */;
pub const DIS_HOR_CRT_DIVBY2: u32 = 0x00000400ul;
pub const SCLK_SEL: u32 = 0x00000800ul;
pub const SCLK_DELAY: u32 = 0x0000f000ul;
pub const TVCLK_PM_EN: u32 = 0x00010000ul;
pub const VCLK_DAC_PM_EN: u32 = 0x00020000ul;
pub const VCLK_LCD_OFF: u32 = 0x00040000ul;
pub const SELECT_WAIT_4MS: u32 = 0x00080000ul;
pub const XTALIN_PM_EN: u32 = 0x00080000ul	/* XC/XL */;
pub const V2CLK_DAC_PM_EN: u32 = 0x00100000ul;
pub const LVDS_EN: u32 = 0x00200000ul;
pub const LVDS_PLL_EN: u32 = 0x00400000ul;
pub const LVDS_PLL_RESET: u32 = 0x00800000ul;
pub const LVDS_RESERVED_BITS: u32 = 0x07000000ul;
pub const CRTC_RW_SELECT: u32 = 0x08000000ul	/* LTPro */;
pub const USE_SHADOWED_VEND: u32 = 0x10000000ul;
pub const USE_SHADOWED_ROWCUR: u32 = 0x20000000ul;
pub const SHADOW_EN: u32 = 0x40000000ul;
pub const SHADOW_RW_EN: u32 = 0x80000000ul;

pub const LCD_SET_PRIMARY_MASK: u32 = 0x07FFFBFBul;

/* Values in HORZ_STRETCHING */
pub const HORZ_STRETCH_BLEND: u32 = 0x00000ffful;
pub const HORZ_STRETCH_RATIO: u32 = 0x0000fffful;
pub const HORZ_STRETCH_LOOP: u32 = 0x00070000ul;
pub const HORZ_STRETCH_LOOP09: u32 = 0x00000000ul;
pub const HORZ_STRETCH_LOOP11: u32 = 0x00010000ul;
pub const HORZ_STRETCH_LOOP12: u32 = 0x00020000ul;
pub const HORZ_STRETCH_LOOP14: u32 = 0x00030000ul;
pub const HORZ_STRETCH_LOOP15: u32 = 0x00040000ul;
/*	?				0x00050000ul */
/*	?				0x00060000ul */
/*	?				0x00070000ul */
/*	?				0x00080000ul */
pub const HORZ_PANEL_SIZE: u32 = 0x0ff00000ul	/* XC/XL */;
/*	?				0x10000000ul */
pub const AUTO_HORZ_RATIO: u32 = 0x20000000ul	/* XC/XL */;
pub const HORZ_STRETCH_MODE: u32 = 0x40000000ul;
pub const HORZ_STRETCH_EN: u32 = 0x80000000ul;

/* Values in VERT_STRETCHING */
pub const VERT_STRETCH_RATIO0: u32 = 0x000003fful;
pub const VERT_STRETCH_RATIO1: u32 = 0x000ffc00ul;
pub const VERT_STRETCH_RATIO2: u32 = 0x3ff00000ul;
pub const VERT_STRETCH_USE0: u32 = 0x40000000ul;
pub const VERT_STRETCH_EN: u32 = 0x80000000ul;

/* Values in EXT_VERT_STRETCH */
pub const VERT_STRETCH_RATIO3: u32 = 0x000003fful;
pub const FORCE_DAC_DATA: u32 = 0x000000fful;
pub const FORCE_DAC_DATA_SEL: u32 = 0x00000300ul;
pub const VERT_STRETCH_MODE: u32 = 0x00000400ul;
pub const VERT_PANEL_SIZE: u32 = 0x003ff800ul;
pub const AUTO_VERT_RATIO: u32 = 0x00400000ul;
pub const USE_AUTO_FP_POS: u32 = 0x00800000ul;
pub const USE_AUTO_LCD_VSYNC: u32 = 0x01000000ul;
/*	?				0xfe000000ul */

/* Values in LCD_MISC_CNTL */
pub const BIAS_MOD_LEVEL_MASK: u32 = 0x0000ff00;
pub const BIAS_MOD_LEVEL_SHIFT: u32 = 8;
pub const BLMOD_EN: u32 = 0x00010000;
pub const BIASMOD_EN: u32 = 0x00020000;


#[inline]
pub const fn is_xl(id: u32) -> bool { id == GR_CHIP_ID || id == GS_CHIP_ID || id == GM_CHIP_ID || id == GN_CHIP_ID || id == GO_CHIP_ID || id == GL_CHIP_ID }
#[inline]
pub const fn is_mobility(id: u32) -> bool { id == LM_CHIP_ID || id == LN_CHIP_ID || id == LR_CHIP_ID || id == LS_CHIP_ID }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
