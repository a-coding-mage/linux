// SPDX-License-Identifier: GPL-2.0-or-later

/*
 * Use pidfds, nsfds, listmount() and statmount() mimic the
 * contents of /proc/self/mountinfo.
 */

use std::ffi::{c_char, c_int, c_uint, c_void};

/* Dependencies supplied by the surrounding repository. */
use crate::*;

/* max mounts per listmount call */
const MAXMOUNTS: usize = 1024;

/* size of struct statmount (including trailing string buffer) */
const STATMOUNT_BUFSIZE: usize = 4096;

static mut EXT_FORMAT: bool = false;

#[cfg(not(any()))]
const __NR_PIDFD_OPEN: c_long = -1;

extern "C" {
    fn syscall(number: c_long, ...) -> c_long;
    fn printf(format: *const c_char, ...) -> c_int;
    fn perror(s: *const c_char);
    fn getopt(argc: c_int, argv: *const *mut c_char, optstring: *const c_char) -> c_int;
    static mut optarg: *mut c_char;
    fn atoi(s: *const c_char) -> c_int;
    fn getpid() -> pid_t;
    fn ioctl(fd: c_int, request: c_ulong, ...) -> c_int;
    fn close(fd: c_int) -> c_int;
}

type c_long = isize;
type c_ulong = usize;
type ssize_t = isize;
type pid_t = i32;

/*
 * There are no bindings in glibc for listmount() and statmount() (yet),
 * make our own here.
 */
unsafe fn statmount(
    mnt_id: __u64,
    mnt_ns_id: __u64,
    mask: __u64,
    buf: *mut statmount,
    bufsize: usize,
    flags: c_uint,
) -> c_int {
    let mut req = mnt_id_req {
        size: MNT_ID_REQ_SIZE_VER0,
        mnt_id,
        param: mask,
        ..std::mem::zeroed()
    };

    if mnt_ns_id != 0 {
        req.size = MNT_ID_REQ_SIZE_VER1;
        req.mnt_ns_id = mnt_ns_id;
    }

    syscall(__NR_statmount, &req, buf, bufsize, flags) as c_int
}

unsafe fn listmount(
    mnt_id: __u64,
    mnt_ns_id: __u64,
    last_mnt_id: __u64,
    list: *mut __u64,
    num: usize,
    flags: c_uint,
) -> ssize_t {
    let mut req = mnt_id_req {
        size: MNT_ID_REQ_SIZE_VER0,
        mnt_id,
        param: last_mnt_id,
        ..std::mem::zeroed()
    };

    if mnt_ns_id != 0 {
        req.size = MNT_ID_REQ_SIZE_VER1;
        req.mnt_ns_id = mnt_ns_id;
    }

    syscall(__NR_listmount, &req, list, num, flags) as ssize_t
}

unsafe fn show_mnt_attrs(flags: __u64) {
    printf(if flags & MOUNT_ATTR_RDONLY != 0 { b"ro\0".as_ptr() } else { b"rw\0".as_ptr() } as *const c_char);

    if flags & MOUNT_ATTR_NOSUID != 0 { printf(b",nosuid\0".as_ptr() as *const c_char); }
    if flags & MOUNT_ATTR_NODEV != 0 { printf(b",nodev\0".as_ptr() as *const c_char); }
    if flags & MOUNT_ATTR_NOEXEC != 0 { printf(b",noexec\0".as_ptr() as *const c_char); }

    match flags & MOUNT_ATTR__ATIME {
        MOUNT_ATTR_RELATIME => printf(b",relatime\0".as_ptr() as *const c_char),
        MOUNT_ATTR_NOATIME => printf(b",noatime\0".as_ptr() as *const c_char),
        MOUNT_ATTR_STRICTATIME => {},
        _ => {},
    }

    if flags & MOUNT_ATTR_NODIRATIME != 0 { printf(b",nodiratime\0".as_ptr() as *const c_char); }
    if flags & MOUNT_ATTR_NOSYMFOLLOW != 0 { printf(b",nosymfollow\0".as_ptr() as *const c_char); }
    if flags & MOUNT_ATTR_IDMAP != 0 { printf(b",idmapped\0".as_ptr() as *const c_char); }
}

unsafe fn show_propagation(sm: *const statmount) {
    if (*sm).mnt_propagation & MS_SHARED != 0 { printf(b" shared:%llu\0".as_ptr() as *const c_char, (*sm).mnt_peer_group); }
    if (*sm).mnt_propagation & MS_SLAVE != 0 {
        printf(b" master:%llu\0".as_ptr() as *const c_char, (*sm).mnt_master);
        if (*sm).propagate_from != 0 && (*sm).propagate_from != (*sm).mnt_master { printf(b" propagate_from:%llu\0".as_ptr() as *const c_char, (*sm).propagate_from); }
    }
    if (*sm).mnt_propagation & MS_UNBINDABLE != 0 { printf(b" unbindable\0".as_ptr() as *const c_char); }
}

unsafe fn show_sb_flags(flags: __u64) {
    printf(if flags & MS_RDONLY != 0 { b"ro\0".as_ptr() } else { b"rw\0".as_ptr() } as *const c_char);
    if flags & MS_SYNCHRONOUS != 0 { printf(b",sync\0".as_ptr() as *const c_char); }
    if flags & MS_DIRSYNC != 0 { printf(b",dirsync\0".as_ptr() as *const c_char); }
    if flags & MS_MANDLOCK != 0 { printf(b",mand\0".as_ptr() as *const c_char); }
    if flags & MS_LAZYTIME != 0 { printf(b",lazytime\0".as_ptr() as *const c_char); }
}

unsafe fn dump_mountinfo(mnt_id: __u64, mnt_ns_id: __u64) -> c_int {
    let mut storage = [0u8; STATMOUNT_BUFSIZE];
    let buf = storage.as_mut_ptr() as *mut statmount;
    let mask = STATMOUNT_SB_BASIC | STATMOUNT_MNT_BASIC | STATMOUNT_PROPAGATE_FROM |
        STATMOUNT_FS_TYPE | STATMOUNT_MNT_ROOT | STATMOUNT_MNT_POINT |
        STATMOUNT_MNT_OPTS | STATMOUNT_FS_SUBTYPE | STATMOUNT_SB_SOURCE;

    if statmount(mnt_id, mnt_ns_id, mask, buf, STATMOUNT_BUFSIZE, 0) < 0 {
        perror(b"statmount\0".as_ptr() as *const c_char);
        return 1;
    }

    if EXT_FORMAT { printf(b"0x%llx 0x%llx 0x%llx \0".as_ptr() as *const c_char, mnt_ns_id, mnt_id, (*buf).mnt_parent_id); }
    printf(b"%u %u %u:%u %s %s \0".as_ptr() as *const c_char, (*buf).mnt_id_old, (*buf).mnt_parent_id_old, (*buf).sb_dev_major, (*buf).sb_dev_minor, (*buf).str.as_ptr().add((*buf).mnt_root as usize), (*buf).str.as_ptr().add((*buf).mnt_point as usize));
    show_mnt_attrs((*buf).mnt_attr);
    show_propagation(buf);
    printf(b" - %s\0".as_ptr() as *const c_char, (*buf).str.as_ptr().add((*buf).fs_type as usize));
    if (*buf).mask & STATMOUNT_FS_SUBTYPE != 0 { printf(b".%s\0".as_ptr() as *const c_char, (*buf).str.as_ptr().add((*buf).fs_subtype as usize)); }
    if (*buf).mask & STATMOUNT_SB_SOURCE != 0 { printf(b" %s \0".as_ptr() as *const c_char, (*buf).str.as_ptr().add((*buf).sb_source as usize)); } else { printf(b" :none \0".as_ptr() as *const c_char); }
    show_sb_flags((*buf).sb_flags);
    if (*buf).mask & STATMOUNT_MNT_OPTS != 0 { printf(b",%s\0".as_ptr() as *const c_char, (*buf).str.as_ptr().add((*buf).mnt_opts as usize)); }
    printf(b"\n\0".as_ptr() as *const c_char);
    0
}

unsafe fn dump_mounts(mnt_ns_id: __u64) -> c_int {
    let mut mntid = [0u64; MAXMOUNTS];
    let mut last_mnt_id = 0u64;
    let mut count: ssize_t;
    loop {
        count = listmount(LSMT_ROOT, mnt_ns_id, last_mnt_id, mntid.as_mut_ptr(), MAXMOUNTS, 0);
        if count < 0 || count > MAXMOUNTS as ssize_t { perror(b"listmount\0".as_ptr() as *const c_char); return 1; }
        for i in 0..count as usize { let ret = dump_mountinfo(mntid[i], mnt_ns_id); if ret != 0 { return ret; } }
        last_mnt_id = mntid[count as usize - 1];
        if count != MAXMOUNTS as ssize_t { break; }
    }
    0
}

unsafe fn usage(prog: *const c_char) {
    printf(b"Usage:\n\0".as_ptr() as *const c_char);
    printf(b"%s [-e] [-p pid] [-r] [-h]\n\0".as_ptr() as *const c_char, prog);
    printf(b"    -e: extended format\n    -h: print usage message\n    -p: get mount namespace from given pid\n    -r: recursively print all mounts in all child namespaces\n\0".as_ptr() as *const c_char);
}

pub unsafe fn main(argc: c_int, argv: *const *mut c_char) -> c_int {
    let mut mni: mnt_ns_info = std::mem::zeroed();
    mni.size = MNT_NS_INFO_SIZE_VER0;
    let mut pid = getpid();
    let mut recursive = false;
    let mut opt;
    loop {
        opt = getopt(argc, argv, b"ehp:r\0".as_ptr() as *const c_char);
        if opt == -1 { break; }
        match opt as u8 as char {
            'e' => EXT_FORMAT = true,
            'h' => { usage(*argv); return 0; },
            'p' => pid = atoi(optarg),
            'r' => recursive = true,
            _ => {},
        }
    }
    let pidfd = syscall(__NR_pidfd_open, pid, 0) as c_int;
    if pidfd < 0 { perror(b"pidfd_open\0".as_ptr() as *const c_char); return 1; }
    let mut mntns = ioctl(pidfd, PIDFD_GET_MNT_NAMESPACE, std::ptr::null_mut::<c_void>());
    if mntns < 0 { perror(b"PIDFD_GET_MNT_NAMESPACE\0".as_ptr() as *const c_char); return 1; }
    close(pidfd);
    if ioctl(mntns, NS_MNT_GET_INFO, &mut mni) < 0 { perror(b"NS_MNT_GET_INFO\0".as_ptr() as *const c_char); return 1; }
    loop {
        let ret = dump_mounts(mni.mnt_ns_id);
        if ret != 0 { return ret; }
        if !recursive { break; }
        let next = ioctl(mntns, NS_MNT_GET_NEXT, &mut mni);
        close(mntns);
        mntns = next;
        if mntns < 0 { break; }
    }
    0
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
