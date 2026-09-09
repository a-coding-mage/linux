/* SPDX-License-Identifier: GPL-2.0-only */
/*
 *  longhaul.h
 *  (C) 2003 Dave Jones.
 *
 *  VIA-specific information
 */

#[repr(C)]
#[derive(Copy, Clone)]
pub struct msr_bcr2_bits {
    pub val: u32,
}

pub const MSR_BCR2_RESEVED_MASK: u32 = 0x0007ffff;
pub const MSR_BCR2_ESOFTBF_MASK: u32 = 0x00080000;
pub const MSR_BCR2_RESERVED2_MASK: u32 = 0x00700000;
pub const MSR_BCR2_CLOCKMUL_MASK: u32 = 0x07800000;
pub const MSR_BCR2_RESERVED3_MASK: u32 = 0xf8000000;

#[repr(C)]
pub union msr_bcr2 {
    pub bits: msr_bcr2_bits,
    pub val: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct msr_longhaul_bits {
    pub val: u64,
}

pub const MSR_LONGHAUL_REVISIONID_MASK: u64 = 0x000000000000000f;
pub const MSR_LONGHAUL_REVISIONKEY_MASK: u64 = 0x00000000000000f0;
pub const MSR_LONGHAUL_ENABLESOFTRATIO_MASK: u64 = 0x0000000000000100;
pub const MSR_LONGHAUL_ENABLESOFTVID_MASK: u64 = 0x0000000000000200;
pub const MSR_LONGHAUL_ENABLESOFTBSEL_MASK: u64 = 0x0000000000000400;
pub const MSR_LONGHAUL_RESERVED_MASK: u64 = 0x0000000000003800;
pub const MSR_LONGHAUL_SOFTBUSRATIO4_MASK: u64 = 0x0000000000004000;
pub const MSR_LONGHAUL_VRMREV_MASK: u64 = 0x0000000000008000;
pub const MSR_LONGHAUL_SOFTBUSRATIO_MASK: u64 = 0x00000000000f0000;
pub const MSR_LONGHAUL_SOFTVID_MASK: u64 = 0x0000000001f00000;
pub const MSR_LONGHAUL_RESERVED2_MASK: u64 = 0x000000000e000000;
pub const MSR_LONGHAUL_SOFTBSEL_MASK: u64 = 0x0000000030000000;
pub const MSR_LONGHAUL_RESERVED3_MASK: u64 = 0x00000000c0000000;
pub const MSR_LONGHAUL_MAXMHZBR_MASK: u64 = 0x0000000f00000000;
pub const MSR_LONGHAUL_MAXIMUMVID_MASK: u64 = 0x000001f000000000;
pub const MSR_LONGHAUL_MAXMHZFSB_MASK: u64 = 0x0000060000000000;
pub const MSR_LONGHAUL_MAXMHZBR4_MASK: u64 = 0x0000080000000000;
pub const MSR_LONGHAUL_RESERVED4_MASK: u64 = 0x0000f00000000000;
pub const MSR_LONGHAUL_MINMHZBR_MASK: u64 = 0x000f000000000000;
pub const MSR_LONGHAUL_MINIMUMVID_MASK: u64 = 0x01f0000000000000;
pub const MSR_LONGHAUL_MINMHZFSB_MASK: u64 = 0x0600000000000000;
pub const MSR_LONGHAUL_MINMHZBR4_MASK: u64 = 0x0800000000000000;
pub const MSR_LONGHAUL_RESERVED5_MASK: u64 = 0xf000000000000000;

#[repr(C)]
pub union msr_longhaul {
    pub bits: msr_longhaul_bits,
    pub val: u64,
}

/* Clock ratio tables. Div/Mod by 10 to get ratio. */
pub static samuel1_mults: [i32; 16] = [-1,30,40,-1,-1,35,45,55,60,70,80,50,65,75,-1,-1];
pub static samuel1_eblcr: [i32; 16] = [50,30,40,-1,55,35,45,-1,-1,70,80,60,-1,75,-1,65];
pub static samuel2_eblcr: [i32; 16] = [50,30,40,100,55,35,45,110,90,70,80,60,120,75,130,65];
pub static ezra_mults: [i32; 16] = [100,30,40,90,95,35,45,55,60,70,80,50,65,75,85,120];
pub static ezra_eblcr: [i32; 16] = [50,30,40,100,55,35,45,95,90,70,80,60,120,75,85,65];
pub static ezrat_mults: [i32; 32] = [100,30,40,90,95,35,45,55,60,70,80,50,65,75,85,120,-1,110,-1,-1,105,115,125,135,140,150,160,130,145,155,-1,-1];
pub static ezrat_eblcr: [i32; 32] = [50,30,40,100,55,35,45,95,90,70,80,60,120,75,85,65,-1,110,120,-1,135,115,125,105,130,150,160,140,-1,155,-1,145];
pub static nehemiah_mults: [i32; 32] = [100,-1,40,90,95,-1,45,55,60,70,80,50,65,75,85,120,-1,110,-1,-1,105,115,125,135,140,150,160,130,145,155,-1,-1];
pub static nehemiah_eblcr: [i32; 32] = [50,160,40,100,55,-1,45,95,90,70,80,60,120,75,85,65,90,110,120,100,135,115,125,105,130,150,160,140,120,155,-1,145];

/* Voltage scales. Div/Mod by 1000 to get actual voltage. */
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mV_pos { pub mV: u16, pub pos: u16 }

pub static vrm85_mV: [mV_pos; 32] = [
    mV_pos{mV:1250,pos:8},mV_pos{mV:1200,pos:6},mV_pos{mV:1150,pos:4},mV_pos{mV:1100,pos:2},
    mV_pos{mV:1050,pos:0},mV_pos{mV:1800,pos:30},mV_pos{mV:1750,pos:28},mV_pos{mV:1700,pos:26},
    mV_pos{mV:1650,pos:24},mV_pos{mV:1600,pos:22},mV_pos{mV:1550,pos:20},mV_pos{mV:1500,pos:18},
    mV_pos{mV:1450,pos:16},mV_pos{mV:1400,pos:14},mV_pos{mV:1350,pos:12},mV_pos{mV:1300,pos:10},
    mV_pos{mV:1275,pos:9},mV_pos{mV:1225,pos:7},mV_pos{mV:1175,pos:5},mV_pos{mV:1125,pos:3},
    mV_pos{mV:1075,pos:1},mV_pos{mV:1825,pos:31},mV_pos{mV:1775,pos:29},mV_pos{mV:1725,pos:27},
    mV_pos{mV:1675,pos:25},mV_pos{mV:1625,pos:23},mV_pos{mV:1575,pos:21},mV_pos{mV:1525,pos:19},
    mV_pos{mV:1475,pos:17},mV_pos{mV:1425,pos:15},mV_pos{mV:1375,pos:13},mV_pos{mV:1325,pos:11},
];
pub static mV_vrm85: [u8; 32] = [0x04,0x14,0x03,0x13,0x02,0x12,0x01,0x11,0x00,0x10,0x0f,0x1f,0x0e,0x1e,0x0d,0x1d,0x0c,0x1c,0x0b,0x1b,0x0a,0x1a,0x09,0x19,0x08,0x18,0x07,0x17,0x06,0x16,0x05,0x15];
pub static mobilevrm_mV: [mV_pos; 32] = [
    mV_pos{mV:1750,pos:31},mV_pos{mV:1700,pos:30},mV_pos{mV:1650,pos:29},mV_pos{mV:1600,pos:28},mV_pos{mV:1550,pos:27},mV_pos{mV:1500,pos:26},mV_pos{mV:1450,pos:25},mV_pos{mV:1400,pos:24},mV_pos{mV:1350,pos:23},mV_pos{mV:1300,pos:22},mV_pos{mV:1250,pos:21},mV_pos{mV:1200,pos:20},mV_pos{mV:1150,pos:19},mV_pos{mV:1100,pos:18},mV_pos{mV:1050,pos:17},mV_pos{mV:1000,pos:16},mV_pos{mV:975,pos:15},mV_pos{mV:950,pos:14},mV_pos{mV:925,pos:13},mV_pos{mV:900,pos:12},mV_pos{mV:875,pos:11},mV_pos{mV:850,pos:10},mV_pos{mV:825,pos:9},mV_pos{mV:800,pos:8},mV_pos{mV:775,pos:7},mV_pos{mV:750,pos:6},mV_pos{mV:725,pos:5},mV_pos{mV:700,pos:4},mV_pos{mV:675,pos:3},mV_pos{mV:650,pos:2},mV_pos{mV:625,pos:1},mV_pos{mV:600,pos:0},
];
pub static mV_mobilevrm: [u8; 32] = [0x1f,0x1e,0x1d,0x1c,0x1b,0x1a,0x19,0x18,0x17,0x16,0x15,0x14,0x13,0x12,0x11,0x10,0x0f,0x0e,0x0d,0x0c,0x0b,0x0a,0x09,0x08,0x07,0x06,0x05,0x04,0x03,0x02,0x01,0x00];

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
