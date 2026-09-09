/* SPDX-License-Identifier: GPL-2.0 WITH Linux-syscall-note */

// Local Channel Commands
pub const TC_WRITE: u8 = 0x01; // Write
pub const TC_RDBUF: u8 = 0x02; // Read Buffer
pub const TC_EWRITE: u8 = 0x05; // Erase write
pub const TC_READMOD: u8 = 0x06; // Read modified
pub const TC_EWRITEA: u8 = 0x0d; // Erase write alternate
pub const TC_WRITESF: u8 = 0x11; // Write structured field

// Buffer Control Orders
pub const TO_GE: u8 = 0x08; // Graphics Escape
pub const TO_SF: u8 = 0x1d; // Start field
pub const TO_SBA: u8 = 0x11; // Set buffer address
pub const TO_IC: u8 = 0x13; // Insert cursor
pub const TO_PT: u8 = 0x05; // Program tab
pub const TO_RA: u8 = 0x3c; // Repeat to address
pub const TO_SFE: u8 = 0x29; // Start field extended
pub const TO_EUA: u8 = 0x12; // Erase unprotected to address
pub const TO_MF: u8 = 0x2c; // Modify field
pub const TO_SA: u8 = 0x28; // Set attribute

// Field Attribute Bytes
pub const TF_INPUT: u8 = 0x40; // Visible input
pub const TF_INPUTN: u8 = 0x4c; // Invisible input
pub const TF_INMDT: u8 = 0xc1; // Visible, Set-MDT
pub const TF_LOG: u8 = 0x60;

// Character Attribute Bytes
pub const TAT_RESET: u8 = 0x00;
pub const TAT_FIELD: u8 = 0xc0;
pub const TAT_EXTHI: u8 = 0x41;
pub const TAT_FGCOLOR: u8 = 0x42;
pub const TAT_CHARS: u8 = 0x43;
pub const TAT_BGCOLOR: u8 = 0x45;
pub const TAT_TRANS: u8 = 0x46;

// Extended-Highlighting Bytes
pub const TAX_RESET: u8 = 0x00;
pub const TAX_BLINK: u8 = 0xf1;
pub const TAX_REVER: u8 = 0xf2;
pub const TAX_UNDER: u8 = 0xf4;

// Reset value
pub const TAR_RESET: u8 = 0x00;

// Color values
pub const TAC_RESET: u8 = 0x00;
pub const TAC_BLUE: u8 = 0xf1;
pub const TAC_RED: u8 = 0xf2;
pub const TAC_PINK: u8 = 0xf3;
pub const TAC_GREEN: u8 = 0xf4;
pub const TAC_TURQ: u8 = 0xf5;
pub const TAC_YELLOW: u8 = 0xf6;
pub const TAC_WHITE: u8 = 0xf7;
pub const TAC_DEFAULT: u8 = 0x00;

// Write Control Characters
pub const TW_NONE: u8 = 0x40; // No particular action
pub const TW_KR: u8 = 0xc2; // Keyboard restore
pub const TW_PLUSALARM: u8 = 0x04; // Add this bit for alarm

pub const RAW3270_FIRSTMINOR: u8 = 1; // First minor number
pub const RAW3270_MAXDEVS: u8 = 255; // Max number of 3270 devices

pub const AID_CLEAR: u8 = 0x6d;
pub const AID_ENTER: u8 = 0x7d;
pub const AID_PF3: u8 = 0xf3;
pub const AID_PF7: u8 = 0xf7;
pub const AID_PF8: u8 = 0xf8;
pub const AID_READ_PARTITION: u8 = 0x88;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
