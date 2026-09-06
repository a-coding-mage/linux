// SPDX-License-Identifier: GPL-2.0-only
//
// AppArmor security module
//
// This file contains AppArmor /proc/<pid>/attr/ interface function definitions.
//
// Copyright (C) 1998-2008 Novell/SUSE
// Copyright 2009-2010 Canonical Ltd.

// Dependency: label.h

use std::ffi::c_char;
use std::os::raw::c_int;

#[repr(C)]
pub struct aa_label {
    _opaque: [u8; 0],
}

extern "C" {
    pub fn aa_getprocattr(
        label: *mut aa_label,
        string: *mut *mut c_char,
        newline: bool,
    ) -> c_int;

    pub fn aa_setprocattr_changehat(
        args: *mut c_char,
        size: usize,
        flags: c_int,
    ) -> c_int;
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
