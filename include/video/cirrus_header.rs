/*
 * drivers/video/clgenfb.h - Cirrus Logic chipset constants
 *
 * Copyright 1999 Jeff Garzik <jgarzik@pobox.com>
 *
 * Original clgenfb author:  Frank Neumann
 *
 * Based on retz3fb.c and clgen.c:
 *      Copyright (C) 1997 Jes Sorensen
 *      Copyright (C) 1996 Frank Neumann
 *
 ***************************************************************
 *
 * Format this code with GNU indent '-kr -i8 -pcs' options.
 *
 * This file is subject to the terms and conditions of the GNU General Public
 * License.  See the file COPYING in the main directory of this archive
 * for more details.
 *
 */

/* OLD COMMENT: definitions for Piccolo/SD64 VGA controller chip   */
/* OLD COMMENT: these definitions might most of the time also work */
/* OLD COMMENT: for other CL-GD542x/543x based boards..            */

/*** External/General Registers ***/
pub const CL_POS102: u32 = 0x102; // POS102 register
pub const CL_VSSM: u32 = 0x46e8; // Adapter Sleep
pub const CL_VSSM2: u32 = 0x3c3; // Motherboard Sleep

/*** VGA Sequencer Registers ***/
/* the following are from the "extension registers" group */
pub const CL_SEQR6: u32 = 0x6; // Unlock ALL Extensions
pub const CL_SEQR7: u32 = 0x7; // Extended Sequencer Mode
pub const CL_SEQR8: u32 = 0x8; // EEPROM Control
pub const CL_SEQR9: u32 = 0x9; // Scratch Pad 0 (do not access!)
pub const CL_SEQRA: u32 = 0xa; // Scratch Pad 1 (do not access!)
pub const CL_SEQRB: u32 = 0xb; // VCLK0 Numerator
pub const CL_SEQRC: u32 = 0xc; // VCLK1 Numerator
pub const CL_SEQRD: u32 = 0xd; // VCLK2 Numerator
pub const CL_SEQRE: u32 = 0xe; // VCLK3 Numerator
pub const CL_SEQRF: u32 = 0xf; // DRAM Control
pub const CL_SEQR10: u32 = 0x10; // Graphics Cursor X Position
pub const CL_SEQR11: u32 = 0x11; // Graphics Cursor Y Position
pub const CL_SEQR12: u32 = 0x12; // Graphics Cursor Attributes
pub const CL_SEQR13: u32 = 0x13; // Graphics Cursor Pattern Address Offset
pub const CL_SEQR14: u32 = 0x14; // Scratch Pad 2 (CL-GD5426/'28 Only) (do not access!)
pub const CL_SEQR15: u32 = 0x15; // Scratch Pad 3 (CL-GD5426/'28 Only) (do not access!)
pub const CL_SEQR16: u32 = 0x16; // Performance Tuning (CL-GD5424/'26/'28 Only)
pub const CL_SEQR17: u32 = 0x17; // Configuration ReadBack and Extended Control (CL-GF5428 Only)
pub const CL_SEQR18: u32 = 0x18; // Signature Generator Control (Not CL-GD5420)
pub const CL_SEQR19: u32 = 0x19; // Signature Generator Result Low Byte (Not CL-GD5420)
pub const CL_SEQR1A: u32 = 0x1a; // Signature Generator Result High Byte (Not CL-GD5420)
pub const CL_SEQR1B: u32 = 0x1b; // VCLK0 Denominator and Post-Scalar Value
pub const CL_SEQR1C: u32 = 0x1c; // VCLK1 Denominator and Post-Scalar Value
pub const CL_SEQR1D: u32 = 0x1d; // VCLK2 Denominator and Post-Scalar Value
pub const CL_SEQR1E: u32 = 0x1e; // VCLK3 Denominator and Post-Scalar Value
pub const CL_SEQR1F: u32 = 0x1f; // BIOS ROM write enable and MCLK Select

/*** CRT Controller Registers ***/
pub const CL_CRT22: u32 = 0x22; // Graphics Data Latches ReadBack
pub const CL_CRT24: u32 = 0x24; // Attribute Controller Toggle ReadBack
pub const CL_CRT26: u32 = 0x26; // Attribute Controller Index ReadBack
/* the following are from the "extension registers" group */
pub const CL_CRT19: u32 = 0x19; // Interlace End
pub const CL_CRT1A: u32 = 0x1a; // Interlace Control
pub const CL_CRT1B: u32 = 0x1b; // Extended Display Controls
pub const CL_CRT1C: u32 = 0x1c; // Sync adjust and genlock register
pub const CL_CRT1D: u32 = 0x1d; // Overlay Extended Control register
pub const CL_CRT1E: u32 = 0x1e; // Another overflow register
pub const CL_CRT25: u32 = 0x25; // Part Status Register
pub const CL_CRT27: u32 = 0x27; // ID Register
pub const CL_CRT51: u32 = 0x51; // P4 disable "flicker fixer"

/*** Graphics Controller Registers ***/
/* the following are from the "extension registers" group */
pub const CL_GR9: u32 = 0x9; // Offset Register 0
pub const CL_GRA: u32 = 0xa; // Offset Register 1
pub const CL_GRB: u32 = 0xb; // Graphics Controller Mode Extensions
pub const CL_GRC: u32 = 0xc; // Color Key (CL-GD5424/'26/'28 Only)
pub const CL_GRD: u32 = 0xd; // Color Key Mask (CL-GD5424/'26/'28 Only)
pub const CL_GRE: u32 = 0xe; // Miscellaneous Control (Cl-GD5428 Only)
pub const CL_GRF: u32 = 0xf; // Display Compression Control register
pub const CL_GR10: u32 = 0x10; // 16-bit Pixel BG Color High Byte (Not CL-GD5420)
pub const CL_GR11: u32 = 0x11; // 16-bit Pixel FG Color High Byte (Not CL-GD5420)
pub const CL_GR12: u32 = 0x12; // Background Color Byte 2 Register
pub const CL_GR13: u32 = 0x13; // Foreground Color Byte 2 Register
pub const CL_GR14: u32 = 0x14; // Background Color Byte 3 Register
pub const CL_GR15: u32 = 0x15; // Foreground Color Byte 3 Register
/* the following are CL-GD5426/'28 specific blitter registers */
pub const CL_GR20: u32 = 0x20; // BLT Width Low
pub const CL_GR21: u32 = 0x21; // BLT Width High
pub const CL_GR22: u32 = 0x22; // BLT Height Low
pub const CL_GR23: u32 = 0x23; // BLT Height High
pub const CL_GR24: u32 = 0x24; // BLT Destination Pitch Low
pub const CL_GR25: u32 = 0x25; // BLT Destination Pitch High
pub const CL_GR26: u32 = 0x26; // BLT Source Pitch Low
pub const CL_GR27: u32 = 0x27; // BLT Source Pitch High
pub const CL_GR28: u32 = 0x28; // BLT Destination Start Low
pub const CL_GR29: u32 = 0x29; // BLT Destination Start Mid
pub const CL_GR2A: u32 = 0x2a; // BLT Destination Start High
pub const CL_GR2C: u32 = 0x2c; // BLT Source Start Low
pub const CL_GR2D: u32 = 0x2d; // BLT Source Start Mid
pub const CL_GR2E: u32 = 0x2e; // BLT Source Start High
pub const CL_GR2F: u32 = 0x2f; // Picasso IV Blitter compat mode..?
pub const CL_GR30: u32 = 0x30; // BLT Mode
pub const CL_GR31: u32 = 0x31; // BLT Start/Status
pub const CL_GR32: u32 = 0x32; // BLT Raster Operation
pub const CL_GR33: u32 = 0x33; // another P4 "compat" register..
pub const CL_GR34: u32 = 0x34; // Transparent Color Select Low
pub const CL_GR35: u32 = 0x35; // Transparent Color Select High
pub const CL_GR38: u32 = 0x38; // Source Transparent Color Mask Low
pub const CL_GR39: u32 = 0x39; // Source Transparent Color Mask High

/*** Attribute Controller Registers ***/
pub const CL_AR33: u32 = 0x33; // The "real" Pixel Panning register (?)
pub const CL_AR34: u32 = 0x34; // TEST

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
