/* SPDX-License-Identifier: (GPL-2.0 OR MIT) */
/*
 * Copyright (c) 2024, SaluteDevices. All Rights Reserved.
 *
 * Author: Jan Dakinevich <jan.dakinevich@salutedevices.com>
 */

pub const AUD_RESET_DDRARB: u32 = 0;
pub const AUD_RESET_TDMIN_A: u32 = 1;
pub const AUD_RESET_TDMIN_B: u32 = 2;
pub const AUD_RESET_TDMIN_LB: u32 = 3;
pub const AUD_RESET_LOOPBACK: u32 = 4;
pub const AUD_RESET_TDMOUT_A: u32 = 5;
pub const AUD_RESET_TDMOUT_B: u32 = 6;
pub const AUD_RESET_FRDDR_A: u32 = 7;
pub const AUD_RESET_FRDDR_B: u32 = 8;
pub const AUD_RESET_TODDR_A: u32 = 9;
pub const AUD_RESET_TODDR_B: u32 = 10;
pub const AUD_RESET_SPDIFIN: u32 = 11;
pub const AUD_RESET_RESAMPLE: u32 = 12;
pub const AUD_RESET_EQDRC: u32 = 13;
pub const AUD_RESET_LOCKER: u32 = 14;
pub const AUD_RESET_TOACODEC: u32 = 30;
pub const AUD_RESET_CLKTREE: u32 = 31;

pub const AUD_VAD_RESET_DDRARB: u32 = 0;
pub const AUD_VAD_RESET_PDM: u32 = 1;
pub const AUD_VAD_RESET_TDMIN_VAD: u32 = 2;
pub const AUD_VAD_RESET_TODDR_VAD: u32 = 3;
pub const AUD_VAD_RESET_TOVAD: u32 = 4;
pub const AUD_VAD_RESET_CLKTREE: u32 = 5;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
