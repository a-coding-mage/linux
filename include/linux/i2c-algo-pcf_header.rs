/* SPDX-License-Identifier: GPL-2.0-or-later */
/* ------------------------------------------------------------------------- */
/* adap-pcf.h i2c driver algorithms for PCF8584 adapters                     */
/* ------------------------------------------------------------------------- */
/*   Copyright (C) 1995-97 Simon G. Vogl
                   1998-99 Hans Berglund

 */
/* ------------------------------------------------------------------------- */

/* With some changes from Kyösti Mälkki <kmalkki@cc.hut.fi> and even
   Frodo Looijaard <frodol@dds.nl> */

use core::ffi::c_void;

#[repr(C)]
pub struct i2c_algo_pcf_data {
    pub data: *mut c_void, /* private data for lolevel routines */
    pub setpcf: Option<unsafe extern "C" fn(data: *mut c_void, ctl: core::ffi::c_int, val: core::ffi::c_int)>,
    pub getpcf: Option<unsafe extern "C" fn(data: *mut c_void, ctl: core::ffi::c_int) -> core::ffi::c_int>,
    pub getown: Option<unsafe extern "C" fn(data: *mut c_void) -> core::ffi::c_int>,
    pub getclock: Option<unsafe extern "C" fn(data: *mut c_void) -> core::ffi::c_int>,
    pub waitforpin: Option<unsafe extern "C" fn(data: *mut c_void)>,

    pub xfer_begin: Option<unsafe extern "C" fn(data: *mut c_void)>,
    pub xfer_end: Option<unsafe extern "C" fn(data: *mut c_void)>,

    /* Multi-master lost arbitration back-off delay (msecs)
     * This should be set by the bus adapter or knowledgable client
     * if bus is multi-mastered, else zero
     */
    pub lab_mdelay: core::ffi::c_ulong,
}

unsafe extern "C" {
    pub fn i2c_pcf_add_bus(adapter: *mut i2c_adapter) -> core::ffi::c_int;
}

/* External dependency supplied by the surrounding repository. */
#[repr(C)]
pub struct i2c_adapter {
    _private: [u8; 0],
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
