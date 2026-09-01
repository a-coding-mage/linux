// SPDX-License-Identifier: GPL-2.0-or-later
/***************************************************************************
 *            au88x0_a3ddata.c
 *
 *  Wed Nov 19 21:11:32 2003
 *  Copyright  2003  mjander
 *  mjander@users.sourceforge.org
 ****************************************************************************/

/*
 */

/* Constant initializer values. */

static A3dHrirZeros: a3d_Hrtf_t = [
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0,
    0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0,
    0, 0, 0,
];

#[allow(dead_code)]
static A3dHrirImpulse: a3d_Hrtf_t = [
    0x7fff, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0,
    0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0,
    0, 0, 0,
];

#[allow(dead_code)]
static A3dHrirOnes: a3d_Hrtf_t = [
    0x7fff, 0x7fff, 0x7fff, 0x7fff, 0x7fff, 0x7fff, 0x7fff, 0x7fff,
    0x7fff,
    0x7fff,
    0x7fff, 0x7fff, 0x7fff, 0x7fff, 0x7fff, 0x7fff, 0x7fff, 0x7fff,
    0x7fff,
    0x7fff,
    0x7fff, 0x7fff, 0x7fff, 0x7fff, 0x7fff, 0x7fff, 0x7fff, 0x7fff,
    0x7fff, 0x7fff, 0x7fff, 0x7fff, 0x7fff, 0x7fff, 0x7fff, 0x7fff,
    0x7fff,
    0x7fff,
    0x7fff, 0x7fff, 0x7fff, 0x7fff, 0x7fff, 0x7fff, 0x7fff, 0x7fff,
    0x7fff,
    0x7fff,
    0x7fff, 0x7fff, 0x7fff, 0x7fff, 0x7fff, 0x7fff, 0x7fff, 0x7fff,
];

#[allow(dead_code)]
static A3dHrirSatTest: a3d_Hrtf_t = [
    0x7fff, 0x7fff, 0x7fff, 0x7fff, 0x7fff, 0x7fff, 0x7fff, 0x7fff,
    0x7fff,
    0x7fff,
    0x8001u16 as i16, 0x8001u16 as i16, 0x8001u16 as i16, 0x8001u16 as i16,
    0x8001u16 as i16, 0x8001u16 as i16, 0x8001u16 as i16, 0x8001u16 as i16,
    0x8001u16 as i16,
    0x8001u16 as i16,
    0x7fff, 0x0000, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
];

#[allow(dead_code)]
static A3dHrirDImpulse: a3d_Hrtf_t = [
    0, 0x7fff, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0,
    0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0,
    0, 0, 0,
];

static A3dItdDlineZeros: a3d_ItdDline_t = [
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
];

static GainTCDefault: i16 = 0x300;
static ItdTCDefault: i16 = 0x0C8;
static HrtfTCDefault: i16 = 0x147;
static CoefTCDefault: i16 = 0x300;

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
