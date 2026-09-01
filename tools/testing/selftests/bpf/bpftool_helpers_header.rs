// SPDX-License-Identifier: GPL-2.0-only

// C dependencies in the original header:
// <stdlib.h>, <stdio.h>, <stdbool.h>

pub const MAX_BPFTOOL_CMD_LEN: usize = 256;

unsafe extern "C" {
    pub fn run_bpftool_command(args: *mut ::std::os::raw::c_char) -> ::std::os::raw::c_int;
    pub fn get_bpftool_command_output(
        args: *mut ::std::os::raw::c_char,
        output_buf: *mut ::std::os::raw::c_char,
        output_max_len: usize,
    ) -> ::std::os::raw::c_int;
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
