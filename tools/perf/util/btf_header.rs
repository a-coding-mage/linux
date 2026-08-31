/* SPDX-License-Identifier: GPL-2.0 */

#[repr(C)]
pub struct btf {
    _private: [u8; 0],
}

#[repr(C)]
pub struct btf_member {
    _private: [u8; 0],
}

unsafe extern "C" {
    pub fn __btf_type__find_member_by_name(
        btf: *mut btf,
        type_id: ::std::os::raw::c_int,
        member_name: *const ::std::os::raw::c_char,
    ) -> *const btf_member;
}
