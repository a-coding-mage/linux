/* SPDX-License-Identifier: GPL-2.0 */

/***************************************************************************
 *            au88x0_eq.h
 *
 *  Definitions and constant data for the Aureal Hardware EQ.
 *
 *  Sun Jun  8 18:23:38 2003
 *  Author: Manuel Jander (mjander@users.sourceforge.net)
 ****************************************************************************/

#[repr(C)]
pub struct auxxEqCoeffSet_t {
    pub LeftCoefs: [u16; 50],  //0x4
    pub RightCoefs: [u16; 50], // 0x68
    pub LeftGains: [u16; 10],  //0xd0
    pub RightGains: [u16; 10], //0xe4
}

#[repr(C)]
pub struct eqhw_t {
    pub this04: i32, /* How many filters for each side (default = 10) */
    pub this08: i32, /* inited to cero. Stereo flag? */
}

#[repr(C)]
pub struct eqlzr_t {
    pub this04: eqhw_t,          /* CHwEq */
    pub this08: u16,             /* Bad codec flag ? SetBypassGain: bypass gain */
    pub this0a: u16,
    pub this0c: u16,             /* SetBypassGain: bypass gain when this28 is not set. */
    pub this0e: u16,
    pub this10: i32,             /* How many gains are used for each side (right or left). */
    pub this14_array: [u16; 10], /* SetLeftGainsTarget: Left (and right?) EQ gains  */
    pub this28: i32,             /* flag related to EQ enabled or not. Gang flag ? */
    pub this54: i32,             /* SetBypass */
    pub this58: i32,
    pub this5c: i32,
    /*0x60 */ pub coefset: auxxEqCoeffSet_t,
    /* 50 u16 word each channel. */
    pub this130: [u16; 20], /* Left and Right gains */
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
