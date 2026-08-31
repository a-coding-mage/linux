// Translated from testing/selftests/memfd/common.h.
// Header guard and C include syntax are omitted in Rust.

unsafe extern "C" {
    pub static mut hugetlbfs_test: ::std::os::raw::c_int;

    pub fn default_huge_page_size() -> ::std::os::raw::c_ulong;
    pub fn sys_memfd_create(
        name: *const ::std::os::raw::c_char,
        flags: ::std::os::raw::c_uint,
    ) -> ::std::os::raw::c_int;
}
