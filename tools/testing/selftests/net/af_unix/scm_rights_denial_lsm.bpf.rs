// SPDX-License-Identifier: GPL-2.0
// C includes translated as external dependencies:
// linux/bpf.h, linux/errno.h, bpf/bpf_helpers.h, bpf/bpf_tracing.h

#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(improper_ctypes)]

pub type __u32 = u32;
pub type __u64 = u64;

pub const BPF_MAP_TYPE_HASH: u32 = 1;
pub const EPERM: i32 = 1;

#[used]
#[link_section = "license"]
pub static mut _license: [u8; 4] = *b"GPL\0";

#[repr(C)]
pub struct inode {
    pub i_ino: libc::c_ulong,
}

#[repr(C)]
pub struct file {
    pub f_inode: *mut inode,
}

#[repr(C)]
pub struct denied_inodes_def {
    pub type_: u32,
    pub max_entries: u32,
    pub key_size: u32,
    pub value_size: u32,
}

#[used]
#[link_section = ".maps"]
pub static mut denied_inodes: denied_inodes_def = denied_inodes_def {
    type_: BPF_MAP_TYPE_HASH,
    max_entries: 16,
    key_size: core::mem::size_of::<__u64>() as u32, /* inode number */
    value_size: core::mem::size_of::<__u32>() as u32, /* tgid of the receiver being tested */
};

unsafe extern "C" {
    pub fn bpf_get_current_pid_tgid() -> __u64;
    pub fn bpf_map_lookup_elem(map: *mut denied_inodes_def, key: *const core::ffi::c_void) -> *mut core::ffi::c_void;
}

#[unsafe(no_mangle)]
#[link_section = "lsm/file_receive"]
pub unsafe extern "C" fn scm_rights_deny(file: *mut file) -> i32 {
    let tgid: __u32 = (unsafe { bpf_get_current_pid_tgid() } >> 32) as __u32;
    let ino: __u64 = unsafe { (*(*file).f_inode).i_ino as __u64 };
    let mut owner: *mut __u32;

    owner = unsafe {
        bpf_map_lookup_elem(
            &raw mut denied_inodes,
            &ino as *const __u64 as *const core::ffi::c_void,
        ) as *mut __u32
    };
    if !owner.is_null() && unsafe { *owner } == tgid {
        return -EPERM;
    }

    return 0;
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
