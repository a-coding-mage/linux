// SPDX-License-Identifier: GPL-2.0-or-later
/***************************************************************************
 *            au88x0_cxtalk.h
 *
 *  Wed Nov 19 19:07:17 2003
 *  Copyright  2003  mjander
 *  mjander@users.sourceforge.org
 ****************************************************************************/

/*
 */

/* The crosstalk canceler supports 5 stereo input channels. The result is
   available at one single output route pair (stereo). */

// C header dependency: #include "au88x0.h"

pub const XTDLINE_SZ: usize = 32;
pub const XTGAINS_SZ: usize = 10;
pub const XTINST_SZ: usize = 4;

pub const XT_HEADPHONE: u32 = 1;
pub const XT_SPEAKER0: u32 = 2;
pub const XT_SPEAKER1: u32 = 3;
pub const XT_DIAMOND: u32 = 4;

pub type xtalk_dline_t = [u32; XTDLINE_SZ];
pub type xtalk_gains_t = [u16; XTGAINS_SZ];
pub type xtalk_instate_t = [u16; XTINST_SZ];
pub type xtalk_coefs_t = [[u16; 5]; 5];
pub type xtalk_state_t = [[u16; 4]; 5];

// The following declarations were C `static` function prototypes in this
// header. They are represented as external declarations here; their internal
// linkage intent belongs to the translation unit that defines them.
unsafe extern "C" {
    fn vortex_XtalkHw_SetGains(vortex: *mut vortex_t, gains: *const u16);
    fn vortex_XtalkHw_SetGainsAllChan(vortex: *mut vortex_t);
    fn vortex_XtalkHw_SetSampleRate(vortex: *mut vortex_t, sr: u32);
    fn vortex_XtalkHw_ProgramPipe(vortex: *mut vortex_t);
    fn vortex_XtalkHw_ProgramPipe(vortex: *mut vortex_t);
    fn vortex_XtalkHw_ProgramXtalkWide(vortex: *mut vortex_t);
    fn vortex_XtalkHw_ProgramXtalkNarrow(vortex: *mut vortex_t);
    fn vortex_XtalkHw_ProgramDiamondXtalk(vortex: *mut vortex_t);
    fn vortex_XtalkHw_Enable(vortex: *mut vortex_t);
    fn vortex_XtalkHw_Disable(vortex: *mut vortex_t);
    fn vortex_XtalkHw_init(vortex: *mut vortex_t);
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
