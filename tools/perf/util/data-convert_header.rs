/* SPDX-License-Identifier: GPL-2.0 */

/* Translated from perf/util/data-convert.h. */

#[repr(C)]
pub struct perf_data_convert_opts {
    pub force: bool,
    pub all: bool,
    pub tod: bool,
    pub time_str: *const ::std::os::raw::c_char,
}

/* Original C condition: #ifdef HAVE_BABELTRACE2_CTF_WRITER_SUPPORT */
#[cfg(HAVE_BABELTRACE2_CTF_WRITER_SUPPORT)]
unsafe extern "C" {
    pub fn bt_convert__perf2ctf(
        input_name: *const ::std::os::raw::c_char,
        to_ctf: *const ::std::os::raw::c_char,
        opts: *mut perf_data_convert_opts,
    ) -> ::std::os::raw::c_int;
}

unsafe extern "C" {
    pub fn bt_convert__perf2json(
        input_name: *const ::std::os::raw::c_char,
        to_ctf: *const ::std::os::raw::c_char,
        opts: *mut perf_data_convert_opts,
    ) -> ::std::os::raw::c_int;
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
