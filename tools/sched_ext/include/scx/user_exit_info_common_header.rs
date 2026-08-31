/* SPDX-License-Identifier: GPL-2.0 */
/*
 * Define struct user_exit_info which is shared between BPF and userspace parts
 * to communicate exit status and other information.
 *
 * Copyright (c) 2022 Meta Platforms, Inc. and affiliates.
 * Copyright (c) 2022 Tejun Heo <tj@kernel.org>
 * Copyright (c) 2022 David Vernet <dvernet@meta.com>
 */

/* C header guard and LSP-only vmlinux include omitted in Rust translation. */

#[repr(C)]
pub enum uei_sizes {
    UEI_REASON_LEN = 128,
    UEI_MSG_LEN = 1024,
    UEI_DUMP_DFL_LEN = 32768,
}

pub const UEI_REASON_LEN: usize = 128;
pub const UEI_MSG_LEN: usize = 1024;
pub const UEI_DUMP_DFL_LEN: usize = 32768;

#[repr(C)]
pub struct user_exit_info {
    pub kind: ::std::os::raw::c_int,
    /*
     * CPU that triggered the exit, or -1 if unset (e.g. running on an
     * older kernel that does not expose this field).
     */
    pub exit_cpu: s32,
    pub exit_code: s64,
    pub reason: [::std::os::raw::c_char; UEI_REASON_LEN],
    pub msg: [::std::os::raw::c_char; UEI_MSG_LEN],
}
