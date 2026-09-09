/*
 * Copyright (c) 2016 - Savoir-faire Linux
 * Author: Sebastien Bourdelin <sebastien.bourdelin@savoirfairelinux.com>
 *
 * This file is licensed under the terms of the GNU General Public
 * License version 2. This program is licensed "as is" without any
 * warranty of any kind, whether express or implied.
 */

#[repr(C)]
pub struct ts_nbus {
    _private: [u8; 0],
}

extern "C" {
    pub fn ts_nbus_read(ts_nbus: *mut ts_nbus, adr: u8, val: *mut u16) -> i32;
    pub fn ts_nbus_write(ts_nbus: *mut ts_nbus, adr: u8, val: u16) -> i32;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
