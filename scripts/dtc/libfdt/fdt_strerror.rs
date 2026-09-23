// SPDX-License-Identifier: (GPL-2.0-or-later OR BSD-2-Clause)
//! Stable names for the original libfdt result codes.
// Copyright (C) 2006 David Gibson, IBM Corporation.
pub(crate) fn strerror(code: i32) -> &'static str {
    const NAMES: [&str; 20] = [
        "<no error>",
        "FDT_ERR_NOTFOUND",
        "FDT_ERR_EXISTS",
        "FDT_ERR_NOSPACE",
        "FDT_ERR_BADOFFSET",
        "FDT_ERR_BADPATH",
        "FDT_ERR_BADPHANDLE",
        "FDT_ERR_BADSTATE",
        "FDT_ERR_TRUNCATED",
        "FDT_ERR_BADMAGIC",
        "FDT_ERR_BADVERSION",
        "FDT_ERR_BADSTRUCTURE",
        "FDT_ERR_BADLAYOUT",
        "FDT_ERR_INTERNAL",
        "FDT_ERR_BADNCELLS",
        "FDT_ERR_BADVALUE",
        "FDT_ERR_BADOVERLAY",
        "FDT_ERR_NOPHANDLES",
        "FDT_ERR_BADFLAGS",
        "FDT_ERR_ALIGNMENT",
    ];
    if code > 0 {
        "<valid offset/length>"
    } else {
        NAMES
            .get(code.unsigned_abs() as usize)
            .copied()
            .unwrap_or("<unknown error>")
    }
}
