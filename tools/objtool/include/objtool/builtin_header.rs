/* SPDX-License-Identifier: GPL-2.0-or-later */
/*
 * Copyright (C) 2015 Josh Poimboeuf <jpoimboe@redhat.com>
 */

// C dependency: <subcmd/parse-options.h>

#[repr(C)]
pub struct opts {
    /* actions: */
    pub checksum: bool,
    pub disas: *const ::std::os::raw::c_char,
    pub dump_orc: bool,
    pub hack_jump_label: bool,
    pub hack_noinstr: bool,
    pub hack_skylake: bool,
    pub ibt: bool,
    pub klp_symids: bool,
    pub mcount: bool,
    pub noabs: bool,
    pub noinstr: bool,
    pub orc: bool,
    pub prefix: ::std::os::raw::c_int,
    pub retpoline: bool,
    pub rethunk: bool,
    pub unret: bool,
    pub sls: bool,
    pub stackval: bool,
    pub static_call: bool,
    pub uaccess: bool,

    /* options: */
    pub backtrace: bool,
    pub backup: bool,
    pub cfi: bool,
    pub debug_checksum: *const ::std::os::raw::c_char,
    pub dryrun: bool,
    pub fineibt: bool,
    pub link: bool,
    pub mnop: bool,
    pub module: bool,
    pub no_unreachable: bool,
    pub output: *const ::std::os::raw::c_char,
    pub sec_address: bool,
    pub stats: bool,
    pub trace: *const ::std::os::raw::c_char,
    pub verbose: bool,
    pub werror: bool,
    pub wide: bool,
}

extern "C" {
    pub static mut opts: opts;

    pub fn cmd_parse_options(
        argc: ::std::os::raw::c_int,
        argv: *const *const ::std::os::raw::c_char,
        usage: *const *const ::std::os::raw::c_char,
    ) -> ::std::os::raw::c_int;

    pub fn objtool_run(
        argc: ::std::os::raw::c_int,
        argv: *const *const ::std::os::raw::c_char,
    ) -> ::std::os::raw::c_int;

    pub fn make_backup() -> ::std::os::raw::c_int;

    pub fn cmd_klp(
        argc: ::std::os::raw::c_int,
        argv: *const *const ::std::os::raw::c_char,
    ) -> ::std::os::raw::c_int;
}
