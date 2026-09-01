/* SPDX-License-Identifier: GPL-2.0-only */
/******************************************************************************

    AudioScience HPI driver
    Copyright (C) 1997-2011  AudioScience Inc. <support@audioscience.com>


 HPI Extended Message Handler Functions

(C) Copyright AudioScience Inc. 1997-2003
******************************************************************************/

/* Dependency from the original C header: "hpi_internal.h". */

use core::ffi::c_void;

pub const HPIMSGX_ALLADAPTERS: u32 = 0xFFFF;

unsafe extern "C" {
    pub fn hpi_send_recv_ex(
        phm: *mut hpi_message,
        phr: *mut hpi_response,
        h_owner: *mut c_void,
    );
}

pub use hpi_send_recv_ex as HPI_MESSAGE_LOWER_LAYER;

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
