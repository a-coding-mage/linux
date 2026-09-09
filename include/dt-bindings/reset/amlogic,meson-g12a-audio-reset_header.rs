/* SPDX-License-Identifier: GPL-2.0 */
/*
 * Copyright (c) 2019 BayLibre, SAS.
 * Author: Jerome Brunet <jbrunet@baylibre.com>
 *
 */

pub const AUD_RESET_PDM: i32 = 0;
pub const AUD_RESET_TDMIN_A: i32 = 1;
pub const AUD_RESET_TDMIN_B: i32 = 2;
pub const AUD_RESET_TDMIN_C: i32 = 3;
pub const AUD_RESET_TDMIN_LB: i32 = 4;
pub const AUD_RESET_LOOPBACK: i32 = 5;
pub const AUD_RESET_TODDR_A: i32 = 6;
pub const AUD_RESET_TODDR_B: i32 = 7;
pub const AUD_RESET_TODDR_C: i32 = 8;
pub const AUD_RESET_FRDDR_A: i32 = 9;
pub const AUD_RESET_FRDDR_B: i32 = 10;
pub const AUD_RESET_FRDDR_C: i32 = 11;
pub const AUD_RESET_TDMOUT_A: i32 = 12;
pub const AUD_RESET_TDMOUT_B: i32 = 13;
pub const AUD_RESET_TDMOUT_C: i32 = 14;
pub const AUD_RESET_SPDIFOUT: i32 = 15;
pub const AUD_RESET_SPDIFOUT_B: i32 = 16;
pub const AUD_RESET_SPDIFIN: i32 = 17;
pub const AUD_RESET_EQDRC: i32 = 18;
pub const AUD_RESET_RESAMPLE: i32 = 19;
pub const AUD_RESET_DDRARB: i32 = 20;
pub const AUD_RESET_POWDET: i32 = 21;
pub const AUD_RESET_TORAM: i32 = 22;
pub const AUD_RESET_TOACODEC: i32 = 23;
pub const AUD_RESET_TOHDMITX: i32 = 24;
pub const AUD_RESET_CLKTREE: i32 = 25;

/* SM1 added resets */
pub const AUD_RESET_RESAMPLE_B: i32 = 26;
pub const AUD_RESET_TOVAD: i32 = 27;
pub const AUD_RESET_LOCKER: i32 = 28;
pub const AUD_RESET_SPDIFIN_LB: i32 = 29;
pub const AUD_RESET_FRATV: i32 = 30;
pub const AUD_RESET_FRHDMIRX: i32 = 31;
pub const AUD_RESET_FRDDR_D: i32 = 32;
pub const AUD_RESET_TODDR_D: i32 = 33;
pub const AUD_RESET_LOOPBACK_B: i32 = 34;
pub const AUD_RESET_EARCTX: i32 = 35;
pub const AUD_RESET_EARCRX: i32 = 36;
pub const AUD_RESET_FRDDR_E: i32 = 37;
pub const AUD_RESET_TODDR_E: i32 = 38;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
