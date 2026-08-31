/* SPDX-License-Identifier: GPL-2.0 */

#[repr(C)]
pub struct option {
    _private: [u8; 0],
}

unsafe extern "C" {
    pub fn parse_user_regs(opt: *const option, str: *const ::std::os::raw::c_char, unset: ::std::os::raw::c_int) -> ::std::os::raw::c_int;
    pub fn parse_intr_regs(opt: *const option, str: *const ::std::os::raw::c_char, unset: ::std::os::raw::c_int) -> ::std::os::raw::c_int;
}
