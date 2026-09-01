// SPDX-License-Identifier: GPL-2.0-or-later
/*
 *  Copyright (c) by Jaroslav Kysela <perex@perex.cz>
 *  Universal interface for Audio Codec '97
 *
 *  For more details look to AC '97 component specification revision 2.2
 *  by Intel Corporation (http://developer.intel.com).
 */

pub const AC97_ID_AK4540: u32 = 0x414b4d00;
pub const AC97_ID_AK4542: u32 = 0x414b4d01;
pub const AC97_ID_AD1819: u32 = 0x41445303;
pub const AC97_ID_AD1881: u32 = 0x41445340;
pub const AC97_ID_AD1881A: u32 = 0x41445348;
pub const AC97_ID_AD1885: u32 = 0x41445360;
pub const AC97_ID_AD1886: u32 = 0x41445361;
pub const AC97_ID_AD1887: u32 = 0x41445362;
pub const AC97_ID_AD1886A: u32 = 0x41445363;
pub const AC97_ID_AD1980: u32 = 0x41445370;
pub const AC97_ID_TR28028: u32 = 0x54524108;
pub const AC97_ID_STAC9700: u32 = 0x83847600;
pub const AC97_ID_STAC9704: u32 = 0x83847604;
pub const AC97_ID_STAC9705: u32 = 0x83847605;
pub const AC97_ID_STAC9708: u32 = 0x83847608;
pub const AC97_ID_STAC9721: u32 = 0x83847609;
pub const AC97_ID_STAC9744: u32 = 0x83847644;
pub const AC97_ID_STAC9756: u32 = 0x83847656;
pub const AC97_ID_CS4297A: u32 = 0x43525910;
pub const AC97_ID_CS4299: u32 = 0x43525930;
pub const AC97_ID_CS4201: u32 = 0x43525948;
pub const AC97_ID_CS4205: u32 = 0x43525958;
pub const AC97_ID_CS_MASK: u32 = 0xfffffff8; /* bit 0-2: rev */
pub const AC97_ID_ALC100: u32 = 0x414c4300;
pub const AC97_ID_ALC650: u32 = 0x414c4720;
pub const AC97_ID_ALC650D: u32 = 0x414c4721;
pub const AC97_ID_ALC650E: u32 = 0x414c4722;
pub const AC97_ID_ALC650F: u32 = 0x414c4723;
pub const AC97_ID_ALC655: u32 = 0x414c4760;
pub const AC97_ID_ALC658: u32 = 0x414c4780;
pub const AC97_ID_ALC658D: u32 = 0x414c4781;
pub const AC97_ID_ALC850: u32 = 0x414c4790;
pub const AC97_ID_YMF743: u32 = 0x594d4800;
pub const AC97_ID_YMF753: u32 = 0x594d4803;
pub const AC97_ID_VT1616: u32 = 0x49434551;
pub const AC97_ID_CM9738: u32 = 0x434d4941;
pub const AC97_ID_CM9739: u32 = 0x434d4961;
pub const AC97_ID_CM9761_78: u32 = 0x434d4978;
pub const AC97_ID_CM9761_82: u32 = 0x434d4982;
pub const AC97_ID_CM9761_83: u32 = 0x434d4983;
pub const AC97_ID_ST7597: u32 = 0x53544d02;
pub const AC97_ID_ST_AC97_ID4: u32 = 0x53544d04;

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
