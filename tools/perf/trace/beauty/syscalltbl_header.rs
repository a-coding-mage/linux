// SPDX-License-Identifier: GPL-2.0
// Translated from perf/trace/beauty/syscalltbl.h.
// Original C header guard: __PERF_SYSCALLTBL_H

use core::ffi::c_char;

unsafe extern "C" {
    pub fn syscalltbl__name(e_machine: i32, id: i32) -> *const c_char;
    pub fn syscalltbl__id(e_machine: i32, name: *const c_char) -> i32;
    pub fn syscalltbl__num_idx(e_machine: i32) -> i32;
    pub fn syscalltbl__id_at_idx(e_machine: i32, idx: i32) -> i32;

    pub fn syscalltbl__strglobmatch_first(
        e_machine: i32,
        syscall_glob: *const c_char,
        idx: *mut i32,
    ) -> i32;
    pub fn syscalltbl__strglobmatch_next(
        e_machine: i32,
        syscall_glob: *const c_char,
        idx: *mut i32,
    ) -> i32;
}
