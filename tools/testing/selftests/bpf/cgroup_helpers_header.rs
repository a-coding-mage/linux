/* SPDX-License-Identifier: GPL-2.0 */

use core::ffi::{c_char, c_int, c_ulonglong};

// C dependencies originally provided through <errno.h>, <string.h>, and stdio users
// of log_err().
#[repr(C)]
pub struct FILE {
    _private: [u8; 0],
}

unsafe extern "C" {
    pub static mut stderr: *mut FILE;

    pub fn __errno_location() -> *mut c_int;
    pub fn strerror(errnum: c_int) -> *mut c_char;
    pub fn fprintf(stream: *mut FILE, format: *const c_char, ...) -> c_int;
}

#[macro_export]
macro_rules! clean_errno {
    () => {{
        unsafe {
            if *$crate::__errno_location() == 0 {
                b"None\0".as_ptr() as *const ::core::ffi::c_char
            } else {
                $crate::strerror(*$crate::__errno_location())
            }
        }
    }};
}

#[macro_export]
macro_rules! log_err {
    ($msg:literal $(, $arg:expr)* $(,)?) => {{
        unsafe {
            $crate::fprintf(
                $crate::stderr,
                concat!("(%s:%d: errno: %s) ", $msg, "\n\0").as_ptr()
                    as *const ::core::ffi::c_char,
                concat!(file!(), "\0").as_ptr() as *const ::core::ffi::c_char,
                line!() as ::core::ffi::c_int,
                $crate::clean_errno!()
                $(, $arg)*
            )
        }
    }};
}

unsafe extern "C" {
    /* cgroupv2 related */
    pub fn enable_controllers(relative_path: *const c_char, controllers: *const c_char) -> c_int;
    pub fn write_cgroup_file(
        relative_path: *const c_char,
        file: *const c_char,
        buf: *const c_char,
    ) -> c_int;
    pub fn write_cgroup_file_parent(
        relative_path: *const c_char,
        file: *const c_char,
        buf: *const c_char,
    ) -> c_int;
    pub fn cgroup_setup_and_join(relative_path: *const c_char) -> c_int;
    pub fn get_root_cgroup() -> c_int;
    pub fn create_and_get_cgroup(relative_path: *const c_char) -> c_int;
    pub fn remove_cgroup(relative_path: *const c_char);
    pub fn remove_cgroup_pid(relative_path: *const c_char, pid: c_int);
    pub fn get_cgroup_id(relative_path: *const c_char) -> c_ulonglong;
    pub fn get_cgroup1_hierarchy_id(subsys_name: *const c_char) -> c_int;

    pub fn join_cgroup(relative_path: *const c_char) -> c_int;
    pub fn join_root_cgroup() -> c_int;
    pub fn join_parent_cgroup(relative_path: *const c_char) -> c_int;

    pub fn set_cgroup_xattr(
        relative_path: *const c_char,
        name: *const c_char,
        value: *const c_char,
    ) -> c_int;

    pub fn setup_cgroup_environment() -> c_int;
    pub fn cleanup_cgroup_environment();

    /* cgroupv1 related */
    pub fn set_classid() -> c_int;
    pub fn join_classid() -> c_int;
    pub fn get_classid_cgroup_id() -> c_ulonglong;
    pub fn open_classid() -> c_int;

    pub fn setup_classid_environment() -> c_int;
    pub fn cleanup_classid_environment();
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
