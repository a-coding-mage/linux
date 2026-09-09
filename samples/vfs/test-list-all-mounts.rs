// SPDX-License-Identifier: GPL-2.0-or-later
// Copyright (c) 2024 Christian Brauner <brauner@kernel.org>

// Dependencies supplied by the corresponding C headers are intentionally
// referenced here rather than reimplemented.

use std::ffi::{c_char, c_int, c_void};
use std::mem;
use std::ptr;

extern "C" {
    fn syscall(number: c_long, ...) -> c_long;
    fn realloc(ptr: *mut c_void, size: usize) -> *mut c_void;
    fn free(ptr: *mut c_void);
    fn ioctl(fd: c_int, request: c_ulong, ...) -> c_int;
    fn getpid() -> c_int;
    fn close(fd: c_int) -> c_int;
    fn strlen(s: *const c_char) -> usize;
    fn printf(format: *const c_char, ...) -> c_int;
    fn exit(status: c_int) -> !;
    fn die_errno(message: *const c_char) -> !;
}

type c_long = i64;
type c_ulong = u64;
type c_uint = u32;
type __u64 = u64;

extern "C" {
    fn pidfd_open(pid: c_int, flags: c_uint) -> c_int;
    fn __errno_location() -> *mut c_int;
}

unsafe fn __statmount(
    mnt_id: __u64,
    mnt_ns_id: __u64,
    mask: __u64,
    stmnt: *mut statmount,
    bufsize: usize,
    flags: c_uint,
) -> c_long {
    let req = mnt_id_req {
        size: MNT_ID_REQ_SIZE_VER1,
        mnt_id,
        param: mask,
        mnt_ns_id,
    };

    syscall(__NR_statmount, &req, stmnt, bufsize, flags)
}

unsafe fn sys_statmount(
    mnt_id: __u64,
    mnt_ns_id: __u64,
    mask: __u64,
    flags: c_uint,
) -> *mut statmount {
    let mut bufsize: usize = 1 << 15;
    let mut stmnt: *mut statmount = ptr::null_mut();

    loop {
        let tmp = realloc(stmnt.cast(), bufsize) as *mut statmount;
        if tmp.is_null() {
            break;
        }

        stmnt = tmp;
        let ret = __statmount(mnt_id, mnt_ns_id, mask, stmnt, bufsize, flags);
        if ret == 0 {
            return stmnt;
        }

        if *__errno_location() != EOVERFLOW {
            break;
        }

        bufsize <<= 1;
        if bufsize >= UINT_MAX / 2 {
            break;
        }
    }

    free(stmnt.cast());
    ptr::null_mut()
}

unsafe fn sys_listmount(
    mnt_id: __u64,
    last_mnt_id: __u64,
    mnt_ns_id: __u64,
    list: *mut __u64,
    num: usize,
    flags: c_uint,
) -> c_long {
    let req = mnt_id_req {
        size: MNT_ID_REQ_SIZE_VER1,
        mnt_id,
        param: last_mnt_id,
        mnt_ns_id,
    };

    syscall(__NR_listmount, &req, list, num, flags)
}

unsafe fn main() {
    const LISTMNT_BUFFER: usize = 10;
    let mut list: [__u64; LISTMNT_BUFFER] = [0; LISTMNT_BUFFER];
    let mut last_mnt_id: __u64 = 0;
    let pidfd = pidfd_open(getpid(), 0);
    if pidfd < 0 {
        die_errno(b"pidfd_open failed\0".as_ptr() as *const c_char);
    }

    let mut fd_mntns = ioctl(pidfd, PIDFD_GET_MNT_NAMESPACE, 0);
    if fd_mntns < 0 {
        die_errno(b"ioctl(PIDFD_GET_MNT_NAMESPACE) failed\0".as_ptr() as *const c_char);
    }

    let mut info: mnt_ns_info = mem::zeroed();
    let ret = ioctl(fd_mntns, NS_MNT_GET_INFO, &mut info);
    if ret < 0 {
        die_errno(b"ioctl(NS_GET_MNTNS_ID) failed\0".as_ptr() as *const c_char);
    }

    printf(b"Listing %u mounts for mount namespace %llu\n\0".as_ptr() as *const c_char, info.nr_mounts, info.mnt_ns_id);
    loop {
        let nr_mounts = sys_listmount(LSMT_ROOT, last_mnt_id, info.mnt_ns_id, list.as_mut_ptr(), LISTMNT_BUFFER, 0);
        if nr_mounts <= 0 {
            printf(b"Finished listing %u mounts for mount namespace %llu\n\n\0".as_ptr() as *const c_char, info.nr_mounts, info.mnt_ns_id);
            let fd_mntns_next = ioctl(fd_mntns, NS_MNT_GET_NEXT, &mut info);
            if fd_mntns_next < 0 {
                if *__errno_location() == ENOENT {
                    printf(b"Finished listing all mount namespaces\n\0".as_ptr() as *const c_char);
                    exit(0);
                }
                die_errno(b"ioctl(NS_MNT_GET_NEXT) failed\0".as_ptr() as *const c_char);
            }
            close(fd_mntns);
            fd_mntns = fd_mntns_next;
            last_mnt_id = 0;
            printf(b"Listing %u mounts for mount namespace %llu\n\0".as_ptr() as *const c_char, info.nr_mounts, info.mnt_ns_id);
            continue;
        }

        for cur in 0..(nr_mounts as usize) {
            last_mnt_id = list[cur];
            let stmnt = sys_statmount(last_mnt_id, info.mnt_ns_id, STATMOUNT_SB_BASIC | STATMOUNT_MNT_BASIC | STATMOUNT_MNT_ROOT | STATMOUNT_MNT_POINT | STATMOUNT_MNT_NS_ID | STATMOUNT_MNT_OPTS | STATMOUNT_FS_TYPE | STATMOUNT_MNT_UIDMAP | STATMOUNT_MNT_GIDMAP, 0);
            if stmnt.is_null() {
                printf(b"Failed to statmount(%llu) in mount namespace(%llu)\n\0".as_ptr() as *const c_char, last_mnt_id, info.mnt_ns_id);
                continue;
            }
            let str_base = (*stmnt).str as *const c_char;
            let fs_type = if (*stmnt).mask & STATMOUNT_FS_TYPE != 0 { str_base.add((*stmnt).fs_type as usize) } else { b"\0".as_ptr() as *const c_char };
            let mnt_root = if (*stmnt).mask & STATMOUNT_MNT_ROOT != 0 { str_base.add((*stmnt).mnt_root as usize) } else { b"\0".as_ptr() as *const c_char };
            let mnt_point = if (*stmnt).mask & STATMOUNT_MNT_POINT != 0 { str_base.add((*stmnt).mnt_point as usize) } else { b"\0".as_ptr() as *const c_char };
            let mnt_opts = if (*stmnt).mask & STATMOUNT_MNT_OPTS != 0 { str_base.add((*stmnt).mnt_opts as usize) } else { b"\0".as_ptr() as *const c_char };
            printf(b"mnt_id:\t\t%llu\nmnt_parent_id:\t%llu\nfs_type:\t%s\nmnt_root:\t%s\nmnt_point:\t%s\nmnt_opts:\t%s\n\0".as_ptr() as *const c_char, (*stmnt).mnt_id, (*stmnt).mnt_parent_id, fs_type, mnt_root, mnt_point, mnt_opts);

            if (*stmnt).mask & STATMOUNT_MNT_UIDMAP != 0 {
                let mut idmap = str_base.add((*stmnt).mnt_uidmap as usize);
                for idx in 0..(*stmnt).mnt_uidmap_num as usize {
                    printf(b"mnt_uidmap[%zu]:\t%s\n\0".as_ptr() as *const c_char, idx, idmap);
                    idmap = idmap.add(strlen(idmap) + 1);
                }
            }

            if (*stmnt).mask & STATMOUNT_MNT_GIDMAP != 0 {
                let mut idmap = str_base.add((*stmnt).mnt_gidmap as usize);
                for idx in 0..(*stmnt).mnt_gidmap_num as usize {
                    printf(b"mnt_gidmap[%zu]:\t%s\n\0".as_ptr() as *const c_char, idx, idmap);
                    idmap = idmap.add(strlen(idmap) + 1);
                }
            }

            printf(b"\n\0".as_ptr() as *const c_char);
            free(stmnt.cast());
        }
    }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
