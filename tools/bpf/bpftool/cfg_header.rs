/* SPDX-License-Identifier: (GPL-2.0-only OR BSD-2-Clause) */
/* Copyright (C) 2018 Netronome Systems, Inc. */

/* C dependency: "xlated_dumper.h" provides struct dump_data. */

unsafe extern "C" {
    pub fn dump_xlated_cfg(
        dd: *mut dump_data,
        buf: *mut core::ffi::c_void,
        len: core::ffi::c_uint,
        opcodes: bool,
        linum: bool,
    );
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
