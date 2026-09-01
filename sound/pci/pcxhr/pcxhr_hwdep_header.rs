/* SPDX-License-Identifier: GPL-2.0-or-later */
/*
 * Driver for Digigram pcxhr compatible soundcards
 *
 * definitions and makros for basic card access
 *
 * Copyright (c) 2004 by Digigram <alsa@digigram.com>
 */

/* firmware status codes  */
pub const PCXHR_FIRMWARE_XLX_INT_INDEX: i32 = 0;
pub const PCXHR_FIRMWARE_XLX_COM_INDEX: i32 = 1;
pub const PCXHR_FIRMWARE_DSP_EPRM_INDEX: i32 = 2;
pub const PCXHR_FIRMWARE_DSP_BOOT_INDEX: i32 = 3;
pub const PCXHR_FIRMWARE_DSP_MAIN_INDEX: i32 = 4;
pub const PCXHR_FIRMWARE_FILES_MAX_INDEX: i32 = 5;

/* exported */
unsafe extern "C" {
    pub fn pcxhr_setup_firmware(mgr: *mut pcxhr_mgr) -> core::ffi::c_int;
    pub fn pcxhr_reset_board(mgr: *mut pcxhr_mgr);
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
