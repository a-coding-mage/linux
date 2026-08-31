// SPDX-License-Identifier: GPL-2.0

// C dependencies: "vmlinux.h", <bpf/bpf_helpers.h>, <bpf/bpf_tracing.h>

pub const MAX_PATH_LEN: usize = 128;
pub const MAX_FILES: usize = 7;

#[allow(non_camel_case_types)]
pub type pid_t = i32;
#[allow(non_camel_case_types)]
pub type __u32 = u32;
#[allow(non_camel_case_types)]
pub type loff_t = i64;

#[repr(C)]
pub struct path {
    _private: [u8; 0],
}

#[repr(C)]
pub struct kstat {
    _private: [u8; 0],
}

#[repr(C)]
pub struct file {
    pub f_path: path,
}

unsafe extern "C" {
    pub fn bpf_get_current_pid_tgid() -> u64;
    pub fn bpf_d_path(path: *mut path, buf: *mut i8, sz: __u32) -> i32;
}

#[unsafe(no_mangle)]
pub static mut my_pid: pid_t = 0;
#[unsafe(no_mangle)]
pub static mut cnt_stat: __u32 = 0;
#[unsafe(no_mangle)]
pub static mut cnt_close: __u32 = 0;
#[unsafe(no_mangle)]
pub static mut paths_stat: [[i8; MAX_PATH_LEN]; MAX_FILES] = [[0; MAX_PATH_LEN]; MAX_FILES];
#[unsafe(no_mangle)]
pub static mut paths_close: [[i8; MAX_PATH_LEN]; MAX_FILES] = [[0; MAX_PATH_LEN]; MAX_FILES];
#[unsafe(no_mangle)]
pub static mut rets_stat: [i32; MAX_FILES] = [0; MAX_FILES];
#[unsafe(no_mangle)]
pub static mut rets_close: [i32; MAX_FILES] = [0; MAX_FILES];

#[unsafe(no_mangle)]
pub static mut called_stat: i32 = 0;
#[unsafe(no_mangle)]
pub static mut called_close: i32 = 0;
#[unsafe(no_mangle)]
pub static mut path_match_fallocate: i32 = 0;

#[unsafe(link_section = "fentry/security_inode_getattr")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn prog_stat(
    path: *mut path,
    _stat: *mut kstat,
    _request_mask: __u32,
    _query_flags: u32,
) -> i32 {
    let pid: pid_t = (unsafe { bpf_get_current_pid_tgid() } >> 32) as pid_t;
    let cnt: __u32 = unsafe { cnt_stat };
    let ret: i32;

    unsafe {
        called_stat = 1;
    }

    if unsafe { pid != my_pid } {
        return 0;
    }

    if cnt >= MAX_FILES as __u32 {
        return 0;
    }
    ret = unsafe {
        bpf_d_path(
            path,
            paths_stat[cnt as usize].as_mut_ptr(),
            MAX_PATH_LEN as __u32,
        )
    };

    unsafe {
        rets_stat[cnt as usize] = ret;
        cnt_stat = cnt_stat.wrapping_add(1);
    }
    0
}

#[unsafe(link_section = "fentry/filp_close")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn prog_close(file: *mut file, _id: *mut core::ffi::c_void) -> i32 {
    let pid: pid_t = (unsafe { bpf_get_current_pid_tgid() } >> 32) as pid_t;
    let cnt: __u32 = unsafe { cnt_close };
    let ret: i32;

    unsafe {
        called_close = 1;
    }

    if unsafe { pid != my_pid } {
        return 0;
    }

    if cnt >= MAX_FILES as __u32 {
        return 0;
    }
    ret = unsafe {
        bpf_d_path(
            &mut (*file).f_path as *mut path,
            paths_close[cnt as usize].as_mut_ptr(),
            MAX_PATH_LEN as __u32,
        )
    };

    unsafe {
        rets_close[cnt as usize] = ret;
        cnt_close = cnt_close.wrapping_add(1);
    }
    0
}

#[unsafe(link_section = "fentry/vfs_fallocate")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn prog_fallocate(
    file: *mut file,
    _mode: i32,
    _offset: loff_t,
    _len: loff_t,
) -> i32 {
    let pid: pid_t = (unsafe { bpf_get_current_pid_tgid() } >> 32) as pid_t;
    let mut ret: i32 = 0;
    let mut path_fallocate: [i8; MAX_PATH_LEN] = [0; MAX_PATH_LEN];

    if unsafe { pid != my_pid } {
        return 0;
    }

    ret = unsafe {
        bpf_d_path(
            &mut (*file).f_path as *mut path,
            path_fallocate.as_mut_ptr(),
            MAX_PATH_LEN as __u32,
        )
    };
    if ret < 0 {
        return 0;
    }

    if path_fallocate[0] == 0 {
        return 0;
    }

    unsafe {
        path_match_fallocate = 1;
    }
    0
}

#[unsafe(link_section = "license")]
#[unsafe(no_mangle)]
pub static _license: [u8; 4] = *b"GPL\0";
