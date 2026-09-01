/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * Copyright(c) 2021-2022 Intel Corporation
 *
 * Author: Cezary Rojewski <cezary.rojewski@intel.com>
 */

// C dependency intent: #include <linux/sizes.h>

use core::ffi::{c_int, c_uint, c_ulong, c_void};
use core::marker::{PhantomData, PhantomPinned};

pub const AVS_CL_DEFAULT_BUFFER_SIZE: usize = 128 * 1024;

#[repr(C)]
pub struct hda_cldma {
    _data: [u8; 0],
    _marker: PhantomData<(*mut u8, PhantomPinned)>,
}

#[repr(C)]
pub struct hdac_bus {
    _data: [u8; 0],
    _marker: PhantomData<(*mut u8, PhantomPinned)>,
}

unsafe extern "C" {
    pub static mut code_loader: hda_cldma;

    pub fn hda_cldma_fill(cl: *mut hda_cldma);
    pub fn hda_cldma_transfer(cl: *mut hda_cldma, start_delay: c_ulong);

    pub fn hda_cldma_start(cl: *mut hda_cldma) -> c_int;
    pub fn hda_cldma_stop(cl: *mut hda_cldma) -> c_int;
    pub fn hda_cldma_reset(cl: *mut hda_cldma) -> c_int;

    pub fn hda_cldma_set_data(cl: *mut hda_cldma, data: *mut c_void, size: c_uint);
    pub fn hda_cldma_setup(cl: *mut hda_cldma);
    pub fn hda_cldma_interrupt(cl: *mut hda_cldma);
    pub fn hda_cldma_init(
        cl: *mut hda_cldma,
        bus: *mut hdac_bus,
        dsp_ba: *mut c_void,
        buffer_size: c_uint,
    ) -> c_int;
    pub fn hda_cldma_free(cl: *mut hda_cldma);
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
