/* SPDX-License-Identifier: GPL-2.0 */

// C dependencies represented here:
// #include <dirent.h>
// #include <string.h>
// #include <stdbool.h>

unsafe extern "C" {
    pub fn tp_event_has_id(
        dir_path: *const ::std::os::raw::c_char,
        evt_dir: *mut ::libc::dirent,
    ) -> ::std::os::raw::c_int;

    pub fn is_valid_tracepoint(event_string: *const ::std::os::raw::c_char) -> bool;
}

#[macro_export]
macro_rules! for_each_event {
    ($dir_path:expr, $evt_dir:expr, $evt_dirent:ident, $body:block) => {
        while {
            $evt_dirent = unsafe { ::libc::readdir($evt_dir) };
            !$evt_dirent.is_null()
        } {
            if unsafe {
                (*$evt_dirent).d_type == ::libc::DT_DIR
                    && ::libc::strcmp((*$evt_dirent).d_name.as_ptr(), c".")
                    && ::libc::strcmp((*$evt_dirent).d_name.as_ptr(), c"..")
                    && !tp_event_has_id($dir_path, $evt_dirent)
            } {
                $body
            }
        }
    };
}

#[macro_export]
macro_rules! for_each_subsystem {
    ($sys_dir:expr, $sys_dirent:ident, $body:block) => {
        while {
            $sys_dirent = unsafe { ::libc::readdir($sys_dir) };
            !$sys_dirent.is_null()
        } {
            if unsafe {
                (*$sys_dirent).d_type == ::libc::DT_DIR
                    && ::libc::strcmp((*$sys_dirent).d_name.as_ptr(), c".")
                    && ::libc::strcmp((*$sys_dirent).d_name.as_ptr(), c"..")
            } {
                $body
            }
        }
    };
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
