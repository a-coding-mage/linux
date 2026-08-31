// SPDX-License-Identifier: GPL-2.0-or-later
// C includes translated as external declarations:
// <subcmd/parse-options.h>, <string.h>, <stdlib.h>,
// <objtool/builtin.h>, <objtool/objtool.h>, <objtool/klp.h>

use std::ffi::{c_char, c_int, c_void};

#[repr(C)]
struct FILE {
    _private: [u8; 0],
}

#[repr(C)]
struct subcmd {
    name: *const c_char,
    description: *const c_char,
    fn_: Option<unsafe extern "C" fn(c_int, *const *const c_char) -> c_int>,
}

unsafe extern "C" {
    static mut stderr: *mut FILE;

    fn fprintf(stream: *mut FILE, format: *const c_char, ...) -> c_int;
    fn strcmp(s1: *const c_char, s2: *const c_char) -> c_int;
    fn exit(status: c_int) -> !;

    fn cmd_klp_checksum(argc: c_int, argv: *const *const c_char) -> c_int;
    fn cmd_klp_diff(argc: c_int, argv: *const *const c_char) -> c_int;
    fn cmd_klp_post_link(argc: c_int, argv: *const *const c_char) -> c_int;
}

static mut subcmds: [subcmd; 3] = [
    subcmd {
        name: b"checksum\0".as_ptr() as *const c_char,
        description: b"Generate per-function checksums\0".as_ptr() as *const c_char,
        fn_: Some(cmd_klp_checksum),
    },
    subcmd {
        name: b"diff\0".as_ptr() as *const c_char,
        description: b"Generate binary diff of two object files\0".as_ptr() as *const c_char,
        fn_: Some(cmd_klp_diff),
    },
    subcmd {
        name: b"post-link\0".as_ptr() as *const c_char,
        description: b"Finalize klp symbols/relocs after module linking\0".as_ptr() as *const c_char,
        fn_: Some(cmd_klp_post_link),
    },
];

unsafe fn cmd_klp_usage() {
    fprintf(
        stderr,
        b"usage: objtool klp <subcommand> [<options>]\n\n\0".as_ptr() as *const c_char,
    );
    fprintf(stderr, b"Subcommands:\n\0".as_ptr() as *const c_char);

    let mut i = 0usize;
    while i < subcmds.len() {
        let cmd: *mut subcmd = &raw mut subcmds[i];

        fprintf(
            stderr,
            b"  %s\t%s\n\0".as_ptr() as *const c_char,
            (*cmd).name,
            (*cmd).description,
        );

        i += 1;
    }

    exit(1);
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn cmd_klp(mut argc: c_int, mut argv: *const *const c_char) -> c_int {
    argc -= 1;
    argv = argv.add(1);

    if argc == 0 {
        cmd_klp_usage();
    }

    if argc != 0 {
        let mut i = 0usize;
        while i < subcmds.len() {
            let cmd: *mut subcmd = &raw mut subcmds[i];

            if strcmp((*cmd).name, *argv.add(0)) == 0 {
                return ((*cmd).fn_).unwrap()(argc, argv);
            }

            i += 1;
        }
    }

    cmd_klp_usage();
    0
}
