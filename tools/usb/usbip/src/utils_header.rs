// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * Copyright (C) 2011 matt mooney <mfm@muteddisk.com>
 *               2005-2007 Takahiro Hirofuchi
 */

unsafe extern "C" {
    pub fn modify_match_busid(busid: *mut ::std::os::raw::c_char, add: ::std::os::raw::c_int) -> ::std::os::raw::c_int;
}
