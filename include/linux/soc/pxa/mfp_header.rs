/* SPDX-License-Identifier: GPL-2.0-only */
/* Common Multi-Function Pin Definitions (translated from C header). */

#[inline]
pub const fn mfp_to_gpio(m: i32) -> i32 { m % 256 }

/* list of all the configurable MFP pins */
pub const MFP_PIN_INVALID: i32 = -1;
pub const MFP_PIN_GPIO0: i32 = 0;
pub const MFP_PIN_GPIO1: i32 = 1;
pub const MFP_PIN_GPIO2: i32 = 2;
pub const MFP_PIN_GPIO3: i32 = 3;
pub const MFP_PIN_GPIO4: i32 = 4;
pub const MFP_PIN_GPIO5: i32 = 5;
pub const MFP_PIN_GPIO6: i32 = 6;
pub const MFP_PIN_GPIO7: i32 = 7;
pub const MFP_PIN_GPIO8: i32 = 8;
pub const MFP_PIN_GPIO9: i32 = 9;
pub const MFP_PIN_GPIO10: i32 = 10;
pub const MFP_PIN_GPIO11: i32 = 11;
pub const MFP_PIN_GPIO12: i32 = 12;
pub const MFP_PIN_GPIO13: i32 = 13;
pub const MFP_PIN_GPIO14: i32 = 14;
pub const MFP_PIN_GPIO15: i32 = 15;
pub const MFP_PIN_GPIO16: i32 = 16;
pub const MFP_PIN_GPIO17: i32 = 17;
pub const MFP_PIN_GPIO18: i32 = 18;
pub const MFP_PIN_GPIO19: i32 = 19;
pub const MFP_PIN_GPIO20: i32 = 20;
pub const MFP_PIN_GPIO21: i32 = 21;
pub const MFP_PIN_GPIO22: i32 = 22;
pub const MFP_PIN_GPIO23: i32 = 23;
pub const MFP_PIN_GPIO24: i32 = 24;
pub const MFP_PIN_GPIO25: i32 = 25;
pub const MFP_PIN_GPIO26: i32 = 26;
pub const MFP_PIN_GPIO27: i32 = 27;
pub const MFP_PIN_GPIO28: i32 = 28;
pub const MFP_PIN_GPIO29: i32 = 29;
pub const MFP_PIN_GPIO30: i32 = 30;
pub const MFP_PIN_GPIO31: i32 = 31;
pub const MFP_PIN_GPIO32: i32 = 32;
pub const MFP_PIN_GPIO33: i32 = 33;
pub const MFP_PIN_GPIO34: i32 = 34;
pub const MFP_PIN_GPIO35: i32 = 35;
pub const MFP_PIN_GPIO36: i32 = 36;
pub const MFP_PIN_GPIO37: i32 = 37;
pub const MFP_PIN_GPIO38: i32 = 38;
pub const MFP_PIN_GPIO39: i32 = 39;
pub const MFP_PIN_GPIO40: i32 = 40;
pub const MFP_PIN_GPIO41: i32 = 41;
pub const MFP_PIN_GPIO42: i32 = 42;
pub const MFP_PIN_GPIO43: i32 = 43;
pub const MFP_PIN_GPIO44: i32 = 44;
pub const MFP_PIN_GPIO45: i32 = 45;
pub const MFP_PIN_GPIO46: i32 = 46;
pub const MFP_PIN_GPIO47: i32 = 47;
pub const MFP_PIN_GPIO48: i32 = 48;
pub const MFP_PIN_GPIO49: i32 = 49;
pub const MFP_PIN_GPIO50: i32 = 50;
pub const MFP_PIN_GPIO51: i32 = 51;
pub const MFP_PIN_GPIO52: i32 = 52;
pub const MFP_PIN_GPIO53: i32 = 53;
pub const MFP_PIN_GPIO54: i32 = 54;
pub const MFP_PIN_GPIO55: i32 = 55;
pub const MFP_PIN_GPIO56: i32 = 56;
pub const MFP_PIN_GPIO57: i32 = 57;
pub const MFP_PIN_GPIO58: i32 = 58;
pub const MFP_PIN_GPIO59: i32 = 59;
pub const MFP_PIN_GPIO60: i32 = 60;
pub const MFP_PIN_GPIO61: i32 = 61;
pub const MFP_PIN_GPIO62: i32 = 62;
pub const MFP_PIN_GPIO63: i32 = 63;
pub const MFP_PIN_GPIO64: i32 = 64;
pub const MFP_PIN_GPIO65: i32 = 65;
pub const MFP_PIN_GPIO66: i32 = 66;
pub const MFP_PIN_GPIO67: i32 = 67;
pub const MFP_PIN_GPIO68: i32 = 68;
pub const MFP_PIN_GPIO69: i32 = 69;
pub const MFP_PIN_GPIO70: i32 = 70;
pub const MFP_PIN_GPIO71: i32 = 71;
pub const MFP_PIN_GPIO72: i32 = 72;
pub const MFP_PIN_GPIO73: i32 = 73;
pub const MFP_PIN_GPIO74: i32 = 74;
pub const MFP_PIN_GPIO75: i32 = 75;
pub const MFP_PIN_GPIO76: i32 = 76;
pub const MFP_PIN_GPIO77: i32 = 77;
pub const MFP_PIN_GPIO78: i32 = 78;
pub const MFP_PIN_GPIO79: i32 = 79;
pub const MFP_PIN_GPIO80: i32 = 80;
pub const MFP_PIN_GPIO81: i32 = 81;
pub const MFP_PIN_GPIO82: i32 = 82;
pub const MFP_PIN_GPIO83: i32 = 83;
pub const MFP_PIN_GPIO84: i32 = 84;
pub const MFP_PIN_GPIO85: i32 = 85;
pub const MFP_PIN_GPIO86: i32 = 86;
pub const MFP_PIN_GPIO87: i32 = 87;
pub const MFP_PIN_GPIO88: i32 = 88;
pub const MFP_PIN_GPIO89: i32 = 89;
pub const MFP_PIN_GPIO90: i32 = 90;
pub const MFP_PIN_GPIO91: i32 = 91;
pub const MFP_PIN_GPIO92: i32 = 92;
pub const MFP_PIN_GPIO93: i32 = 93;
pub const MFP_PIN_GPIO94: i32 = 94;
pub const MFP_PIN_GPIO95: i32 = 95;
pub const MFP_PIN_GPIO96: i32 = 96;
pub const MFP_PIN_GPIO97: i32 = 97;
pub const MFP_PIN_GPIO98: i32 = 98;
pub const MFP_PIN_GPIO99: i32 = 99;
pub const MFP_PIN_GPIO100: i32 = 100;
pub const MFP_PIN_GPIO101: i32 = 101;
pub const MFP_PIN_GPIO102: i32 = 102;
pub const MFP_PIN_GPIO103: i32 = 103;
pub const MFP_PIN_GPIO104: i32 = 104;
pub const MFP_PIN_GPIO105: i32 = 105;
pub const MFP_PIN_GPIO106: i32 = 106;
pub const MFP_PIN_GPIO107: i32 = 107;
pub const MFP_PIN_GPIO108: i32 = 108;
pub const MFP_PIN_GPIO109: i32 = 109;
pub const MFP_PIN_GPIO110: i32 = 110;
pub const MFP_PIN_GPIO111: i32 = 111;
pub const MFP_PIN_GPIO112: i32 = 112;
pub const MFP_PIN_GPIO113: i32 = 113;
pub const MFP_PIN_GPIO114: i32 = 114;
pub const MFP_PIN_GPIO115: i32 = 115;
pub const MFP_PIN_GPIO116: i32 = 116;
pub const MFP_PIN_GPIO117: i32 = 117;
pub const MFP_PIN_GPIO118: i32 = 118;
pub const MFP_PIN_GPIO119: i32 = 119;
pub const MFP_PIN_GPIO120: i32 = 120;
pub const MFP_PIN_GPIO121: i32 = 121;
pub const MFP_PIN_GPIO122: i32 = 122;
pub const MFP_PIN_GPIO123: i32 = 123;
pub const MFP_PIN_GPIO124: i32 = 124;
pub const MFP_PIN_GPIO125: i32 = 125;
pub const MFP_PIN_GPIO126: i32 = 126;
pub const MFP_PIN_GPIO127: i32 = 127;
pub const MFP_PIN_GPIO128: i32 = 128;
pub const MFP_PIN_GPIO129: i32 = 129;
pub const MFP_PIN_GPIO130: i32 = 130;
pub const MFP_PIN_GPIO131: i32 = 131;
pub const MFP_PIN_GPIO132: i32 = 132;
pub const MFP_PIN_GPIO133: i32 = 133;
pub const MFP_PIN_GPIO134: i32 = 134;
pub const MFP_PIN_GPIO135: i32 = 135;
pub const MFP_PIN_GPIO136: i32 = 136;
pub const MFP_PIN_GPIO137: i32 = 137;
pub const MFP_PIN_GPIO138: i32 = 138;
pub const MFP_PIN_GPIO139: i32 = 139;
pub const MFP_PIN_GPIO140: i32 = 140;
pub const MFP_PIN_GPIO141: i32 = 141;
pub const MFP_PIN_GPIO142: i32 = 142;
pub const MFP_PIN_GPIO143: i32 = 143;
pub const MFP_PIN_GPIO144: i32 = 144;
pub const MFP_PIN_GPIO145: i32 = 145;
pub const MFP_PIN_GPIO146: i32 = 146;
pub const MFP_PIN_GPIO147: i32 = 147;
pub const MFP_PIN_GPIO148: i32 = 148;
pub const MFP_PIN_GPIO149: i32 = 149;
pub const MFP_PIN_GPIO150: i32 = 150;
pub const MFP_PIN_GPIO151: i32 = 151;
pub const MFP_PIN_GPIO152: i32 = 152;
pub const MFP_PIN_GPIO153: i32 = 153;
pub const MFP_PIN_GPIO154: i32 = 154;
pub const MFP_PIN_GPIO155: i32 = 155;
pub const MFP_PIN_GPIO156: i32 = 156;
pub const MFP_PIN_GPIO157: i32 = 157;
pub const MFP_PIN_GPIO158: i32 = 158;
pub const MFP_PIN_GPIO159: i32 = 159;
pub const MFP_PIN_GPIO160: i32 = 160;
pub const MFP_PIN_GPIO161: i32 = 161;
pub const MFP_PIN_GPIO162: i32 = 162;
pub const MFP_PIN_GPIO163: i32 = 163;
pub const MFP_PIN_GPIO164: i32 = 164;
pub const MFP_PIN_GPIO165: i32 = 165;
pub const MFP_PIN_GPIO166: i32 = 166;
pub const MFP_PIN_GPIO167: i32 = 167;
pub const MFP_PIN_GPIO168: i32 = 168;
pub const MFP_PIN_GPIO169: i32 = 169;
pub const MFP_PIN_GPIO170: i32 = 170;
pub const MFP_PIN_GPIO171: i32 = 171;
pub const MFP_PIN_GPIO172: i32 = 172;
pub const MFP_PIN_GPIO173: i32 = 173;
pub const MFP_PIN_GPIO174: i32 = 174;
pub const MFP_PIN_GPIO175: i32 = 175;
pub const MFP_PIN_GPIO176: i32 = 176;
pub const MFP_PIN_GPIO177: i32 = 177;
pub const MFP_PIN_GPIO178: i32 = 178;
pub const MFP_PIN_GPIO179: i32 = 179;
pub const MFP_PIN_GPIO180: i32 = 180;
pub const MFP_PIN_GPIO181: i32 = 181;
pub const MFP_PIN_GPIO182: i32 = 182;
pub const MFP_PIN_GPIO183: i32 = 183;
pub const MFP_PIN_GPIO184: i32 = 184;
pub const MFP_PIN_GPIO185: i32 = 185;
pub const MFP_PIN_GPIO186: i32 = 186;
pub const MFP_PIN_GPIO187: i32 = 187;
pub const MFP_PIN_GPIO188: i32 = 188;
pub const MFP_PIN_GPIO189: i32 = 189;
pub const MFP_PIN_GPIO190: i32 = 190;
pub const MFP_PIN_GPIO191: i32 = 191;
pub const MFP_PIN_GPIO255: i32 = 255;
pub const MFP_PIN_GPIO0_2: i32 = 256;
pub const MFP_PIN_GPIO1_2: i32 = 257;
pub const MFP_PIN_GPIO2_2: i32 = 258;
pub const MFP_PIN_GPIO3_2: i32 = 259;
pub const MFP_PIN_GPIO4_2: i32 = 260;
pub const MFP_PIN_GPIO5_2: i32 = 261;
pub const MFP_PIN_GPIO6_2: i32 = 262;
pub const MFP_PIN_GPIO7_2: i32 = 263;
pub const MFP_PIN_GPIO8_2: i32 = 264;
pub const MFP_PIN_GPIO9_2: i32 = 265;
pub const MFP_PIN_GPIO10_2: i32 = 266;
pub const MFP_PIN_GPIO11_2: i32 = 267;
pub const MFP_PIN_GPIO12_2: i32 = 268;
pub const MFP_PIN_GPIO13_2: i32 = 269;
pub const MFP_PIN_GPIO14_2: i32 = 270;
pub const MFP_PIN_GPIO15_2: i32 = 271;
pub const MFP_PIN_GPIO16_2: i32 = 272;
pub const MFP_PIN_ULPI_STP: i32 = 273;
pub const MFP_PIN_ULPI_NXT: i32 = 274;
pub const MFP_PIN_ULPI_DIR: i32 = 275;
pub const MFP_PIN_nXCVREN: i32 = 276;
pub const MFP_PIN_DF_CLE_nOE: i32 = 277;
pub const MFP_PIN_DF_nADV1_ALE: i32 = 278;
pub const MFP_PIN_DF_SCLK_E: i32 = 279;
pub const MFP_PIN_DF_SCLK_S: i32 = 280;
pub const MFP_PIN_nBE0: i32 = 281;
pub const MFP_PIN_nBE1: i32 = 282;
pub const MFP_PIN_DF_nADV2_ALE: i32 = 283;
pub const MFP_PIN_DF_INT_RnB: i32 = 284;
pub const MFP_PIN_DF_nCS0: i32 = 285;
pub const MFP_PIN_DF_nCS1: i32 = 286;
pub const MFP_PIN_nLUA: i32 = 287;
pub const MFP_PIN_nLLA: i32 = 288;
pub const MFP_PIN_DF_nWE: i32 = 289;
pub const MFP_PIN_DF_ALE_nWE: i32 = 290;
pub const MFP_PIN_DF_nRE_nOE: i32 = 291;
pub const MFP_PIN_DF_ADDR0: i32 = 292;
pub const MFP_PIN_DF_ADDR1: i32 = 293;
pub const MFP_PIN_DF_ADDR2: i32 = 294;
pub const MFP_PIN_DF_ADDR3: i32 = 295;
pub const MFP_PIN_DF_IO0: i32 = 296;
pub const MFP_PIN_DF_IO1: i32 = 297;
pub const MFP_PIN_DF_IO2: i32 = 298;
pub const MFP_PIN_DF_IO3: i32 = 299;
pub const MFP_PIN_DF_IO4: i32 = 300;
pub const MFP_PIN_DF_IO5: i32 = 301;
pub const MFP_PIN_DF_IO6: i32 = 302;
pub const MFP_PIN_DF_IO7: i32 = 303;
pub const MFP_PIN_DF_IO8: i32 = 304;
pub const MFP_PIN_DF_IO9: i32 = 305;
pub const MFP_PIN_DF_IO10: i32 = 306;
pub const MFP_PIN_DF_IO11: i32 = 307;
pub const MFP_PIN_DF_IO12: i32 = 308;
pub const MFP_PIN_DF_IO13: i32 = 309;
pub const MFP_PIN_DF_IO14: i32 = 310;
pub const MFP_PIN_DF_IO15: i32 = 311;
pub const MFP_PIN_DF_nCS0_SM_nCS2: i32 = 312;
pub const MFP_PIN_DF_nCS1_SM_nCS3: i32 = 313;
pub const MFP_PIN_SM_nCS0: i32 = 314;
pub const MFP_PIN_SM_nCS1: i32 = 315;
pub const MFP_PIN_DF_WEn: i32 = 316;
pub const MFP_PIN_DF_REn: i32 = 317;
pub const MFP_PIN_DF_CLE_SM_OEn: i32 = 318;
pub const MFP_PIN_DF_ALE_SM_WEn: i32 = 319;
pub const MFP_PIN_DF_RDY0: i32 = 320;
pub const MFP_PIN_DF_RDY1: i32 = 321;
pub const MFP_PIN_SM_SCLK: i32 = 322;
pub const MFP_PIN_SM_BE0: i32 = 323;
pub const MFP_PIN_SM_BE1: i32 = 324;
pub const MFP_PIN_SM_ADV: i32 = 325;
pub const MFP_PIN_SM_ADVMUX: i32 = 326;
pub const MFP_PIN_SM_RDY: i32 = 327;
pub const MFP_PIN_MMC1_DAT7: i32 = 328;
pub const MFP_PIN_MMC1_DAT6: i32 = 329;
pub const MFP_PIN_MMC1_DAT5: i32 = 330;
pub const MFP_PIN_MMC1_DAT4: i32 = 331;
pub const MFP_PIN_MMC1_DAT3: i32 = 332;
pub const MFP_PIN_MMC1_DAT2: i32 = 333;
pub const MFP_PIN_MMC1_DAT1: i32 = 334;
pub const MFP_PIN_MMC1_DAT0: i32 = 335;
pub const MFP_PIN_MMC1_CMD: i32 = 336;
pub const MFP_PIN_MMC1_CLK: i32 = 337;
pub const MFP_PIN_MMC1_CD: i32 = 338;
pub const MFP_PIN_MMC1_WP: i32 = 339;
pub const MFP_PIN_GSIM_UIO: i32 = 340;
pub const MFP_PIN_GSIM_UCLK: i32 = 341;
pub const MFP_PIN_GSIM_UDET: i32 = 342;
pub const MFP_PIN_GSIM_nURST: i32 = 343;
pub const MFP_PIN_PMIC_INT: i32 = 344;
pub const MFP_PIN_RDY: i32 = 345;
pub const MFP_PIN_TWSI1_SCL: i32 = 346;
pub const MFP_PIN_TWSI1_SDA: i32 = 347;
pub const MFP_PIN_TWSI4_SCL: i32 = 348;
pub const MFP_PIN_TWSI4_SDA: i32 = 349;
pub const MFP_PIN_CLK_REQ: i32 = 350;
pub const MFP_PIN_MAX: i32 = 351;

pub type mfp_cfg_t = libc::c_ulong;

#[inline] pub const fn MFP_PIN(x: u32) -> u32 { x & 0x3ff }

pub const MFP_AF0: u32 = 0x0 << 10;
pub const MFP_AF1: u32 = 0x1 << 10;
pub const MFP_AF2: u32 = 0x2 << 10;
pub const MFP_AF3: u32 = 0x3 << 10;
pub const MFP_AF4: u32 = 0x4 << 10;
pub const MFP_AF5: u32 = 0x5 << 10;
pub const MFP_AF6: u32 = 0x6 << 10;
pub const MFP_AF7: u32 = 0x7 << 10;
pub const MFP_AF_MASK: u32 = 0x7 << 10;
#[inline] pub const fn MFP_AF(x: u32) -> u32 { (x >> 10) & 0x7 }

pub const MFP_DS01X: u32 = 0x0 << 13;
pub const MFP_DS02X: u32 = 0x1 << 13;
pub const MFP_DS03X: u32 = 0x2 << 13;
pub const MFP_DS04X: u32 = 0x3 << 13;
pub const MFP_DS06X: u32 = 0x4 << 13;
pub const MFP_DS08X: u32 = 0x5 << 13;
pub const MFP_DS10X: u32 = 0x6 << 13;
pub const MFP_DS13X: u32 = 0x7 << 13;
pub const MFP_DS_MASK: u32 = 0x7 << 13;
#[inline] pub const fn MFP_DS(x: u32) -> u32 { (x >> 13) & 0x7 }

pub const MFP_LPM_DEFAULT: u32 = 0x0 << 16;
pub const MFP_LPM_DRIVE_LOW: u32 = 0x1 << 16;
pub const MFP_LPM_DRIVE_HIGH: u32 = 0x2 << 16;
pub const MFP_LPM_PULL_LOW: u32 = 0x3 << 16;
pub const MFP_LPM_PULL_HIGH: u32 = 0x4 << 16;
pub const MFP_LPM_FLOAT: u32 = 0x5 << 16;
pub const MFP_LPM_INPUT: u32 = 0x6 << 16;
pub const MFP_LPM_STATE_MASK: u32 = 0x7 << 16;
#[inline] pub const fn MFP_LPM_STATE(x: u32) -> u32 { (x >> 16) & 0x7 }

pub const MFP_LPM_EDGE_NONE: u32 = 0x0 << 19;
pub const MFP_LPM_EDGE_RISE: u32 = 0x1 << 19;
pub const MFP_LPM_EDGE_FALL: u32 = 0x2 << 19;
pub const MFP_LPM_EDGE_BOTH: u32 = 0x3 << 19;
pub const MFP_LPM_EDGE_MASK: u32 = 0x3 << 19;
#[inline] pub const fn MFP_LPM_EDGE(x: u32) -> u32 { (x >> 19) & 0x3 }

pub const MFP_PULL_NONE: u32 = 0x0 << 21;
pub const MFP_PULL_LOW: u32 = 0x1 << 21;
pub const MFP_PULL_HIGH: u32 = 0x2 << 21;
pub const MFP_PULL_BOTH: u32 = 0x3 << 21;
pub const MFP_PULL_FLOAT: u32 = 0x4 << 21;
pub const MFP_PULL_MASK: u32 = 0x7 << 21;
#[inline] pub const fn MFP_PULL(x: u32) -> u32 { (x >> 21) & 0x7 }

pub const MFP_CFG_DEFAULT: u32 = MFP_AF0 | MFP_DS03X | MFP_LPM_DEFAULT | MFP_LPM_EDGE_NONE | MFP_PULL_NONE;
#[macro_export] macro_rules! MFP_CFG { ($pin:expr, $af:expr) => { (MFP_CFG_DEFAULT & !MFP_AF_MASK) | (MFP_PIN($pin) | $af) }; }
#[macro_export] macro_rules! MFP_CFG_DRV { ($pin:expr, $af:expr, $drv:expr) => { (MFP_CFG_DEFAULT & !(MFP_AF_MASK | MFP_DS_MASK)) | (MFP_PIN($pin) | $af | $drv) }; }
#[macro_export] macro_rules! MFP_CFG_LPM { ($pin:expr, $af:expr, $lpm:expr) => { (MFP_CFG_DEFAULT & !(MFP_AF_MASK | MFP_LPM_STATE_MASK)) | (MFP_PIN($pin) | $af | $lpm) }; }
#[macro_export] macro_rules! MFP_CFG_X { ($pin:expr, $af:expr, $drv:expr, $lpm:expr) => { (MFP_CFG_DEFAULT & !(MFP_AF_MASK | MFP_DS_MASK | MFP_LPM_STATE_MASK)) | (MFP_PIN($pin) | $af | $drv | $lpm) }; }

/* CONFIG_PXA3xx || CONFIG_ARCH_MMP declarations. */
#[repr(C)] pub struct mfp_addr_map { pub start: u32, pub end: u32, pub offset: libc::c_ulong }
#[macro_export] macro_rules! MFP_ADDR_X { ($start:expr, $end:expr, $offset:expr) => { mfp_addr_map { start: $start, end: $end, offset: $offset } }; }
#[macro_export] macro_rules! MFP_ADDR { ($pin:expr, $offset:expr) => { mfp_addr_map { start: $pin, end: u32::MAX, offset: $offset } }; }
#[macro_export] macro_rules! MFP_ADDR_END { () => { mfp_addr_map { start: MFP_PIN_INVALID as u32, end: 0, offset: 0 } }; }

extern "C" {
    pub fn mfp_init_base(mfpr_base: *mut core::ffi::c_void);
    pub fn mfp_init_addr(map: *mut mfp_addr_map);
    pub fn mfp_read(mfp: i32) -> libc::c_ulong;
    pub fn mfp_write(mfp: i32, mfpr_val: libc::c_ulong);
    pub fn mfp_config(mfp_cfgs: *mut libc::c_ulong, num: i32);
    pub fn mfp_config_run();
    pub fn mfp_config_lpm();
}


// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
