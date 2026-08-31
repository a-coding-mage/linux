#![allow(non_camel_case_types)]

use std::os::raw::{c_char, c_int, c_uchar};

const AT_FDCWD: c_int = -100;

#[repr(C)]
pub struct file_handle {
    pub handle_bytes: u32,
    pub handle_type: c_int,
    pub f_handle: [c_uchar; 0],
}

unsafe extern "C" {
    pub fn name_to_handle_at(
        dirfd: c_int,
        pathname: *const c_char,
        handle: *mut file_handle,
        mount_id: *mut c_int,
        flags: c_int,
    ) -> c_int;
}

#[repr(C)]
struct Handle {
    fh: file_handle,
    cgroup_id: u64,
}

fn main() {
    let mut handle = Handle {
        fh: file_handle {
            handle_bytes: 0,
            handle_type: 0,
            f_handle: [],
        },
        cgroup_id: 0,
    };
    let mut mount_id: c_int = 0;

    unsafe {
        name_to_handle_at(
            AT_FDCWD,
            c"/".as_ptr(),
            &mut handle.fh,
            &mut mount_id,
            0,
        );
    }
}
