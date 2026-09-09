/* SPDX-License-Identifier: GPL-2.0 WITH Linux-syscall-note */

// Support for features provided by this header has been removed.
// Please consider updating your code.

#[repr(C)]
pub struct cyclades_monitor {
    pub int_count: ::core::ffi::c_ulong,
    pub char_count: ::core::ffi::c_ulong,
    pub char_max: ::core::ffi::c_ulong,
    pub char_last: ::core::ffi::c_ulong,
}

pub const CYGETMON: u32 = 0x435901;
pub const CYGETTHRESH: u32 = 0x435902;
pub const CYSETTHRESH: u32 = 0x435903;
pub const CYGETDEFTHRESH: u32 = 0x435904;
pub const CYSETDEFTHRESH: u32 = 0x435905;
pub const CYGETTIMEOUT: u32 = 0x435906;
pub const CYSETTIMEOUT: u32 = 0x435907;
pub const CYGETDEFTIMEOUT: u32 = 0x435908;
pub const CYSETDEFTIMEOUT: u32 = 0x435909;
pub const CYSETRFLOW: u32 = 0x43590a;
pub const CYGETRFLOW: u32 = 0x43590b;
pub const CYSETRTSDTR_INV: u32 = 0x43590c;
pub const CYGETRTSDTR_INV: u32 = 0x43590d;
pub const CYZSETPOLLCYCLE: u32 = 0x43590e;
pub const CYZGETPOLLCYCLE: u32 = 0x43590f;
pub const CYGETCD1400VER: u32 = 0x435910;
pub const CYSETWAIT: u32 = 0x435912;
pub const CYGETWAIT: u32 = 0x435913;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
