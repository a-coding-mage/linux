// SPDX-License-Identifier: LGPL-2.1

#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum rlimit_action {
    NO_CHANGE,
    SET_TO_MAX,
    INCREASED_MAX,
}

unsafe extern "C" {
    pub fn rlimit__bump_memlock();

    pub fn rlimit__increase_nofile(set_rlimit: *mut rlimit_action) -> bool;
}
