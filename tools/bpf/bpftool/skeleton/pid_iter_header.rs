// SPDX-License-Identifier: (GPL-2.0-only OR BSD-2-Clause)
// Copyright (c) 2020 Facebook

#[repr(C)]
pub struct pid_iter_entry {
    pub id: __u32,
    pub pid: ::core::ffi::c_int,
    pub bpf_cookie: __u64,
    pub has_bpf_cookie: bool,
    pub comm: [::core::ffi::c_char; 16],
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
