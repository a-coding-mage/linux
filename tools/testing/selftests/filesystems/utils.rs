// SPDX-License-Identifier: GPL-2.0
// Translated from C. Original includes provided libc, Linux, kselftest,
// wrappers, and local utils declarations.

#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(dead_code)]
#![allow(improper_ctypes)]

use core::ffi::{c_char, c_int, c_long, c_uint, c_ulong, c_void};
use core::mem;
use core::ptr;

type size_t = usize;
type ssize_t = isize;
type pid_t = c_int;
type uid_t = c_uint;
type gid_t = c_uint;
type __u32 = u32;
type uint64_t = u64;
type cap_t = *mut c_void;
type cap_value_t = c_int;

const MAX_USERNS_LEVEL: c_uint = 32;
const __STACK_SIZE: usize = 8 * 1024 * 1024;

const EBADF: c_int = 9;
const EINTR: c_int = 4;
const ENOENT: c_int = 2;
const ENOMEM: c_int = 12;
const E2BIG: c_int = 7;
const EIO: c_int = 5;

const O_RDONLY: c_int = 0;
const O_WRONLY: c_int = 1;
const O_CLOEXEC: c_int = 0o2000000;
const O_NOCTTY: c_int = 0o400;

const SIGCHLD: c_int = 17;
const SIGKILL: c_int = 9;

const CLONE_VM: c_int = 0x00000100;
const CLONE_FILES: c_int = 0x00000400;
const CLONE_NEWNS: c_int = 0x00020000;
const CLONE_NEWUSER: c_int = 0x10000000;

const AF_LOCAL: c_int = 1;
const SOCK_STREAM: c_int = 1;
const SOCK_CLOEXEC: c_int = O_CLOEXEC;

const PR_SET_DUMPABLE: c_int = 4;
const MS_REC: c_ulong = 16384;
const MS_PRIVATE: c_ulong = 1 << 18;

const AT_FDCWD: c_int = -100;
const STATX_MNT_ID_UNIQUE: c_uint = 0x00004000;

const CAP_EFFECTIVE: c_int = 0;

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
enum idmap_type_t {
    ID_TYPE_UID,
    ID_TYPE_GID,
}

#[repr(C)]
struct id_map {
    map_type: idmap_type_t,
    nsid: __u32,
    hostid: __u32,
    range: __u32,
}

#[repr(C)]
struct list {
    elem: *mut c_void,
    next: *mut list,
    prev: *mut list,
}

#[repr(C)]
struct userns_hierarchy {
    fd_userns: c_int,
    fd_event: c_int,
    level: c_uint,
    id_map: list,
}

#[repr(C)]
struct statx {
    stx_mask: u32,
    stx_blksize: u32,
    stx_attributes: u64,
    stx_nlink: u32,
    stx_uid: u32,
    stx_gid: u32,
    stx_mode: u16,
    __spare0: [u16; 1],
    stx_ino: u64,
    stx_size: u64,
    stx_blocks: u64,
    stx_attributes_mask: u64,
    __time_and_device_fields: [u8; 80],
    stx_mnt_id: u64,
    __spare2: [u64; 13],
}

unsafe extern "C" {
    fn __errno_location() -> *mut c_int;
    fn fprintf(stream: *mut c_void, format: *const c_char, ...) -> c_int;
    static mut stderr: *mut c_void;
    fn strerror(errnum: c_int) -> *mut c_char;
    fn snprintf(s: *mut c_char, n: size_t, format: *const c_char, ...) -> c_int;
    fn sprintf(s: *mut c_char, format: *const c_char, ...) -> c_int;
    fn memset(s: *mut c_void, c: c_int, n: size_t) -> *mut c_void;
    fn strlen(s: *const c_char) -> size_t;
    fn malloc(size: size_t) -> *mut c_void;
    fn read(fd: c_int, buf: *mut c_void, count: size_t) -> ssize_t;
    fn write(fd: c_int, buf: *const c_void, count: size_t) -> ssize_t;
    fn close(fd: c_int) -> c_int;
    fn open(pathname: *const c_char, flags: c_int, ...) -> c_int;
    fn clone(
        fn_: unsafe extern "C" fn(*mut c_void) -> c_int,
        child_stack: *mut c_void,
        flags: c_int,
        arg: *mut c_void,
        ...
    ) -> pid_t;
    fn __clone2(
        fn_: unsafe extern "C" fn(*mut c_void) -> c_int,
        child_stack_base: *mut c_void,
        stack_size: size_t,
        flags: c_int,
        arg: *mut c_void,
        ...
    ) -> pid_t;
    fn pause() -> c_int;
    fn _exit(status: c_int) -> !;
    fn waitpid(pid: pid_t, stat_loc: *mut c_int, options: c_int) -> pid_t;
    fn geteuid() -> uid_t;
    fn getuid() -> uid_t;
    fn getgid() -> gid_t;
    fn setgroups(size: size_t, list: *const gid_t) -> c_int;
    fn setresgid(rgid: gid_t, egid: gid_t, sgid: gid_t) -> c_int;
    fn setresuid(ruid: uid_t, euid: uid_t, suid: uid_t) -> c_int;
    fn prctl(option: c_int, ...) -> c_int;
    fn kill(pid: pid_t, sig: c_int) -> c_int;
    fn socketpair(domain: c_int, type_: c_int, protocol: c_int, sv: *mut c_int) -> c_int;
    fn unshare(flags: c_int) -> c_int;
    fn mount(
        source: *const c_char,
        target: *const c_char,
        filesystemtype: *const c_char,
        mountflags: c_ulong,
        data: *const c_void,
    ) -> c_int;
    fn cap_get_proc() -> cap_t;
    fn cap_clear_flag(cap_p: cap_t, flag: c_int) -> c_int;
    fn cap_set_flag(cap_p: cap_t, flag: c_int, ncap: c_int, caps: *const cap_value_t, value: c_int) -> c_int;
    fn cap_set_proc(cap_p: cap_t) -> c_int;
    fn cap_free(obj_d: *mut c_void) -> c_int;
    fn statx(dirfd: c_int, pathname: *const c_char, flags: c_int, mask: c_uint, statxbuf: *mut statx) -> c_int;

    fn ksft_print_msg(format: *const c_char, ...);
    fn ksft_exit_fail_msg(format: *const c_char, ...);
}

macro_rules! c {
    ($s:literal) => {
        concat!($s, "\0").as_ptr() as *const c_char
    };
}

unsafe fn errno() -> c_int {
    unsafe { *__errno_location() }
}

unsafe fn set_errno(e: c_int) {
    unsafe {
        *__errno_location() = e;
    }
}

macro_rules! syserror {
    ($format:literal $(, $arg:expr)* $(,)?) => {{
        unsafe {
            fprintf(stderr, c!(concat!("%m - ", $format, "\n")) $(, $arg)*);
            -errno()
        }
    }};
}

macro_rules! syserror_set {
    ($ret:expr, $format:literal $(, $arg:expr)* $(,)?) => {{
        let internal_ret = $ret;
        unsafe {
            set_errno(if internal_ret < 0 { -internal_ret } else { internal_ret });
            fprintf(stderr, c!(concat!("%m - ", $format, "\n")) $(, $arg)*);
        }
        internal_ret
    }};
}

unsafe fn list_init(list: *mut list) {
    unsafe {
        (*list).elem = ptr::null_mut();
        (*list).next = list;
        (*list).prev = list;
    }
}

unsafe fn list_empty(list: *const list) -> c_int {
    unsafe { (list == (*list).next as *const list) as c_int }
}

unsafe fn __list_add(new: *mut list, prev: *mut list, next: *mut list) {
    unsafe {
        (*next).prev = new;
        (*new).next = next;
        (*new).prev = prev;
        (*prev).next = new;
    }
}

unsafe fn list_add_tail(head: *mut list, list: *mut list) {
    unsafe {
        __list_add(list, (*head).prev, head);
    }
}

unsafe fn list_del(list: *mut list) {
    unsafe {
        let next = (*list).next;
        let prev = (*list).prev;
        (*next).prev = prev;
        (*prev).next = next;
    }
}

unsafe fn read_nointr(fd: c_int, buf: *mut c_void, count: size_t) -> ssize_t {
    let mut ret: ssize_t;
    loop {
        ret = unsafe { read(fd, buf, count) };
        if !(ret < 0 && unsafe { errno() } == EINTR) {
            break;
        }
    }
    ret
}

unsafe fn write_nointr(fd: c_int, buf: *const c_void, count: size_t) -> ssize_t {
    let mut ret: ssize_t;
    loop {
        ret = unsafe { write(fd, buf, count) };
        if !(ret < 0 && unsafe { errno() } == EINTR) {
            break;
        }
    }
    ret
}

unsafe fn do_clone(fn_: unsafe extern "C" fn(*mut c_void) -> c_int, arg: *mut c_void, flags: c_int) -> pid_t {
    let stack = unsafe { malloc(__STACK_SIZE) };
    if stack.is_null() {
        return -ENOMEM;
    }

    // C used __clone2 on __ia64__; this Rust translation keeps that conditional intent.
    #[cfg(target_arch = "ia64")]
    unsafe {
        __clone2(fn_, stack, __STACK_SIZE, flags | SIGCHLD, arg, ptr::null_mut::<c_void>())
    }
    #[cfg(not(target_arch = "ia64"))]
    unsafe {
        clone(fn_, (stack as *mut u8).add(__STACK_SIZE) as *mut c_void, flags | SIGCHLD, arg, ptr::null_mut::<c_void>())
    }
}

unsafe extern "C" fn get_userns_fd_cb(_data: *mut c_void) -> c_int {
    loop {
        unsafe {
            pause();
        }
    }
    #[allow(unreachable_code)]
    unsafe {
        _exit(0);
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn wait_for_pid(pid: pid_t) -> c_int {
    let mut status: c_int = 0;
    let mut ret: c_int;

    loop {
        ret = unsafe { waitpid(pid, &mut status, 0) };
        if ret == -1 {
            if unsafe { errno() } == EINTR {
                continue;
            }
            return -1;
        }
        break;
    }

    if (status & 0x7f) != 0 {
        return -1;
    }

    (status & 0xff00) >> 8
}

unsafe fn write_id_mapping(map_type: idmap_type_t, pid: pid_t, buf: *const c_char, buf_size: size_t) -> c_int {
    let mut fd: c_int = -EBADF;
    let mut setgroups_fd: c_int = -EBADF;
    let mut fret: c_int = -1;
    let mut ret: c_int;
    let mut path = [0 as c_char; 64];

    'out: loop {
        if unsafe { geteuid() } != 0 && map_type == idmap_type_t::ID_TYPE_GID {
            ret = unsafe { snprintf(path.as_mut_ptr(), path.len(), c!("/proc/%d/setgroups"), pid) };
            if ret < 0 || ret as usize >= path.len() {
                break 'out;
            }

            setgroups_fd = unsafe { open(path.as_ptr(), O_WRONLY | O_CLOEXEC) };
            if setgroups_fd < 0 && unsafe { errno() } != ENOENT {
                syserror!("Failed to open \"%s\"", path.as_ptr());
                break 'out;
            }

            if setgroups_fd >= 0 {
                ret = unsafe { write_nointr(setgroups_fd, c!("deny\n") as *const c_void, 5) as c_int };
                if ret != 5 {
                    syserror!("Failed to write \"deny\" to \"/proc/%d/setgroups\"", pid);
                    break 'out;
                }
            }
        }

        ret = unsafe {
            snprintf(
                path.as_mut_ptr(),
                path.len(),
                c!("/proc/%d/%cid_map"),
                pid,
                if map_type == idmap_type_t::ID_TYPE_UID { b'u' as c_int } else { b'g' as c_int },
            )
        };
        if ret < 0 || ret as usize >= path.len() {
            break 'out;
        }

        fd = unsafe { open(path.as_ptr(), O_WRONLY | O_CLOEXEC) };
        if fd < 0 {
            syserror!("Failed to open \"%s\"", path.as_ptr());
            break 'out;
        }

        ret = unsafe { write_nointr(fd, buf as *const c_void, buf_size) as c_int };
        if ret as size_t != buf_size {
            syserror!(
                "Failed to write %cid mapping to \"%s\"",
                if map_type == idmap_type_t::ID_TYPE_UID { b'u' as c_int } else { b'g' as c_int },
                path.as_ptr()
            );
            break 'out;
        }

        fret = 0;
        break 'out;
    }
    unsafe {
        close(fd);
        close(setgroups_fd);
    }
    fret
}

unsafe fn map_ids_from_idmap(idmap: *mut list, pid: pid_t) -> c_int {
    let mut fill: c_int;
    let mut left: c_int;
    let mut mapbuf = [0 as c_char; 4096];
    let mut had_entry = false;
    let mut map_type = idmap_type_t::ID_TYPE_UID;
    let mut u_or_g = b'u' as c_int;

    if unsafe { list_empty(idmap) } != 0 {
        return 0;
    }

    loop {
        had_entry = false;
        let mut pos = mapbuf.as_mut_ptr();
        let mut iterator: *mut list;

        iterator = unsafe { (*idmap).next };
        while iterator != idmap {
            let map = unsafe { (*iterator).elem as *mut id_map };
            if unsafe { (*map).map_type } != map_type {
                iterator = unsafe { (*iterator).next };
                continue;
            }

            had_entry = true;

            left = 4096 - unsafe { pos.offset_from(mapbuf.as_ptr()) as c_int };
            fill = unsafe {
                snprintf(
                    pos,
                    left as size_t,
                    c!("%u %u %u\n"),
                    (*map).nsid,
                    (*map).hostid,
                    (*map).range,
                )
            };
            /*
             * The kernel only takes <= 4k for writes to
             * /proc/<pid>/{g,u}id_map
             */
            if fill <= 0 || fill >= left {
                return syserror_set!(-E2BIG, "Too many %cid mappings defined", u_or_g);
            }

            pos = unsafe { pos.add(fill as usize) };
            iterator = unsafe { (*iterator).next };
        }
        if had_entry {
            let ret = unsafe { write_id_mapping(map_type, pid, mapbuf.as_ptr(), pos.offset_from(mapbuf.as_ptr()) as size_t) };
            if ret < 0 {
                return syserror!("Failed to write mapping: %s", mapbuf.as_ptr());
            }

            unsafe {
                memset(mapbuf.as_mut_ptr() as *mut c_void, 0, mem::size_of_val(&mapbuf));
            }
        }

        if map_type == idmap_type_t::ID_TYPE_GID {
            break;
        }
        map_type = idmap_type_t::ID_TYPE_GID;
        u_or_g = b'g' as c_int;
    }

    0
}

unsafe fn get_userns_fd_from_idmap(idmap: *mut list) -> c_int {
    let mut ret: c_int;
    let pid: pid_t;
    let mut path_ns = [0 as c_char; 64];

    pid = unsafe { do_clone(get_userns_fd_cb, ptr::null_mut(), CLONE_NEWUSER | CLONE_NEWNS) };
    if pid < 0 {
        return -unsafe { errno() };
    }

    ret = unsafe { map_ids_from_idmap(idmap, pid) };
    if ret < 0 {
        return ret;
    }

    ret = unsafe { snprintf(path_ns.as_mut_ptr(), path_ns.len(), c!("/proc/%d/ns/user"), pid) };
    if ret < 0 || ret as size_t >= path_ns.len() {
        ret = -EIO;
    } else {
        ret = unsafe { open(path_ns.as_ptr(), O_RDONLY | O_CLOEXEC | O_NOCTTY) };
    }

    unsafe {
        kill(pid, SIGKILL);
        wait_for_pid(pid);
    }
    ret
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn get_userns_fd(nsid: c_ulong, hostid: c_ulong, range: c_ulong) -> c_int {
    let mut head: list = unsafe { mem::zeroed() };
    let mut uid_mapl: list = unsafe { mem::zeroed() };
    let mut gid_mapl: list = unsafe { mem::zeroed() };
    let mut uid_map = id_map {
        map_type: idmap_type_t::ID_TYPE_UID,
        nsid: nsid as __u32,
        hostid: hostid as __u32,
        range: range as __u32,
    };
    let mut gid_map = id_map {
        map_type: idmap_type_t::ID_TYPE_GID,
        nsid: nsid as __u32,
        hostid: hostid as __u32,
        range: range as __u32,
    };

    unsafe { list_init(&mut head) };
    uid_mapl.elem = &mut uid_map as *mut _ as *mut c_void;
    gid_mapl.elem = &mut gid_map as *mut _ as *mut c_void;
    unsafe {
        list_add_tail(&mut head, &mut uid_mapl);
        list_add_tail(&mut head, &mut gid_mapl);
        get_userns_fd_from_idmap(&mut head)
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn switch_ids(uid: uid_t, gid: gid_t) -> bool {
    if unsafe { setgroups(0, ptr::null()) } != 0 {
        return syserror!("failure: setgroups") != 0;
    }

    if unsafe { setresgid(gid, gid, gid) } != 0 {
        return syserror!("failure: setresgid") != 0;
    }

    if unsafe { setresuid(uid, uid, uid) } != 0 {
        return syserror!("failure: setresuid") != 0;
    }

    /* Ensure we can access proc files from processes we can ptrace. */
    if unsafe { prctl(PR_SET_DUMPABLE, 1, 0, 0, 0) } != 0 {
        return syserror!("failure: make dumpable") != 0;
    }

    true
}

unsafe fn create_userns_hierarchy(h: *mut userns_hierarchy) -> c_int {
    let mut fret: c_int = -1;
    let mut cbuf: c_char = 0;
    let mut fd_socket = [0 as c_int; 2];
    let mut fd_userns: c_int = -EBADF;
    let mut ret: c_int = -1;
    let mut bytes: ssize_t;
    let pid: pid_t;
    let mut path = [0 as c_char; 256];

    if unsafe { (*h).level } == MAX_USERNS_LEVEL {
        return 0;
    }

    ret = unsafe { socketpair(AF_LOCAL, SOCK_STREAM | SOCK_CLOEXEC, 0, fd_socket.as_mut_ptr()) };
    if ret < 0 {
        return syserror!("failure: create socketpair");
    }

    /* Note the CLONE_FILES | CLONE_VM when mucking with fds and memory. */
    unsafe {
        (*h).fd_event = fd_socket[1];
    }
    pid = unsafe { do_clone(userns_fd_cb, h as *mut c_void, CLONE_NEWUSER | CLONE_FILES | CLONE_VM) };
    if pid < 0 {
        syserror!("failure: userns level %d", unsafe { (*h).level });
        unsafe { close(fd_socket[0]); close(fd_socket[1]); }
        return fret;
    }

    ret = unsafe { map_ids_from_idmap(&mut (*h).id_map, pid) };
    if ret < 0 {
        unsafe { kill(pid, SIGKILL); }
        syserror!("failure: writing id mapping for userns level %d for %d", unsafe { (*h).level }, pid);
        unsafe { wait_for_pid(pid); close(fd_socket[0]); close(fd_socket[1]); }
        return fret;
    }

    if unsafe { list_empty(&mut (*h).id_map) } == 0 {
        bytes = unsafe { write_nointr(fd_socket[0], c!("1") as *const c_void, 1) }; /* Inform the child we wrote a mapping. */
    } else {
        bytes = unsafe { write_nointr(fd_socket[0], c!("0") as *const c_void, 1) }; /* Inform the child we didn't write a mapping. */
    }
    if bytes < 0 {
        unsafe { kill(pid, SIGKILL); }
        syserror!("failure: write to socketpair");
        unsafe { wait_for_pid(pid); close(fd_socket[0]); close(fd_socket[1]); }
        return fret;
    }

    /* Wait for child to set*id() and become dumpable. */
    bytes = unsafe { read_nointr(fd_socket[0], &mut cbuf as *mut _ as *mut c_void, 1) };
    if bytes < 0 {
        unsafe { kill(pid, SIGKILL); }
        syserror!("failure: read from socketpair");
        unsafe { wait_for_pid(pid); close(fd_socket[0]); close(fd_socket[1]); }
        return fret;
    }

    unsafe {
        snprintf(path.as_mut_ptr(), path.len(), c!("/proc/%d/ns/user"), pid);
    }
    fd_userns = unsafe { open(path.as_ptr(), O_RDONLY | O_CLOEXEC) };
    if fd_userns < 0 {
        unsafe { kill(pid, SIGKILL); }
        syserror!("failure: open userns level %d for %d", unsafe { (*h).level }, pid);
        unsafe { wait_for_pid(pid); close(fd_socket[0]); close(fd_socket[1]); }
        return fret;
    }

    fret = 0;

    if unsafe { wait_for_pid(pid) } == 0 && fret == 0 {
        unsafe {
            (*h).fd_userns = fd_userns;
        }
        fd_userns = -EBADF;
    }

    if fd_userns >= 0 {
        unsafe { close(fd_userns); }
    }
    unsafe {
        close(fd_socket[0]);
        close(fd_socket[1]);
    }
    fret
}

unsafe extern "C" fn userns_fd_cb(data: *mut c_void) -> c_int {
    let mut h = data as *mut userns_hierarchy;
    let mut cbuf: c_char = 0;
    let mut ret: c_int;

    ret = unsafe { read_nointr((*h).fd_event, &mut cbuf as *mut _ as *mut c_void, 1) as c_int };
    if ret < 0 {
        return syserror!("failure: read from socketpair");
    }

    /* Only switch ids if someone actually wrote a mapping for us. */
    if cbuf == b'1' as c_char {
        if !unsafe { switch_ids(0, 0) } {
            return syserror!("failure: switch ids to 0");
        }
    }

    ret = unsafe { write_nointr((*h).fd_event, c!("1") as *const c_void, 1) as c_int };
    if ret < 0 {
        return syserror!("failure: write to socketpair");
    }

    h = unsafe { h.add(1) };
    ret = unsafe { create_userns_hierarchy(h) };
    if ret < 0 {
        return syserror!("failure: userns level %d", unsafe { (*h).level });
    }

    0
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn write_file(path: *const c_char, val: *const c_char) -> c_int {
    let fd = unsafe { open(path, O_WRONLY) };
    let len = unsafe { strlen(val) };
    let mut ret: c_int;

    if fd == -1 {
        unsafe { ksft_print_msg(c!("opening %s for write: %s\n"), path, strerror(errno())) };
        return -1;
    }

    ret = unsafe { write(fd, val as *const c_void, len) as c_int };
    if ret == -1 {
        unsafe { ksft_print_msg(c!("writing to %s: %s\n"), path, strerror(errno())) };
        return -1;
    }
    if ret as size_t != len {
        unsafe { ksft_print_msg(c!("short write to %s\n"), path) };
        return -1;
    }

    ret = unsafe { close(fd) };
    if ret == -1 {
        unsafe { ksft_print_msg(c!("closing %s\n"), path) };
        return -1;
    }

    0
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn setup_userns() -> c_int {
    let mut ret: c_int;
    let mut buf = [0 as c_char; 32];
    let uid: uid_t = unsafe { getuid() };
    let gid: gid_t = unsafe { getgid() };

    ret = unsafe { unshare(CLONE_NEWNS | CLONE_NEWUSER) };
    if ret != 0 {
        unsafe { ksft_exit_fail_msg(c!("unsharing mountns and userns: %s\n"), strerror(errno())) };
        return ret;
    }

    unsafe { sprintf(buf.as_mut_ptr(), c!("0 %d 1"), uid) };
    ret = unsafe { write_file(c!("/proc/self/uid_map"), buf.as_ptr()) };
    if ret != 0 {
        return ret;
    }
    ret = unsafe { write_file(c!("/proc/self/setgroups"), c!("deny")) };
    if ret != 0 {
        return ret;
    }
    unsafe { sprintf(buf.as_mut_ptr(), c!("0 %d 1"), gid) };
    ret = unsafe { write_file(c!("/proc/self/gid_map"), buf.as_ptr()) };
    if ret != 0 {
        return ret;
    }

    ret = unsafe { mount(c!(""), c!("/"), ptr::null(), MS_REC | MS_PRIVATE, ptr::null()) };
    if ret != 0 {
        unsafe { ksft_print_msg(c!("making mount tree private: %s\n"), strerror(errno())) };
        return ret;
    }

    0
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn enter_userns() -> c_int {
    let mut ret: c_int;
    let mut buf = [0 as c_char; 32];
    let uid: uid_t = unsafe { getuid() };
    let gid: gid_t = unsafe { getgid() };

    ret = unsafe { unshare(CLONE_NEWUSER) };
    if ret != 0 {
        return ret;
    }

    unsafe { sprintf(buf.as_mut_ptr(), c!("0 %d 1"), uid) };
    ret = unsafe { write_file(c!("/proc/self/uid_map"), buf.as_ptr()) };
    if ret != 0 {
        return ret;
    }
    ret = unsafe { write_file(c!("/proc/self/setgroups"), c!("deny")) };
    if ret != 0 {
        return ret;
    }
    unsafe { sprintf(buf.as_mut_ptr(), c!("0 %d 1"), gid) };
    ret = unsafe { write_file(c!("/proc/self/gid_map"), buf.as_ptr()) };
    if ret != 0 {
        return ret;
    }

    0
}

/* caps_down - lower all effective caps */
#[unsafe(no_mangle)]
pub unsafe extern "C" fn caps_down() -> c_int {
    let mut fret = false;
    let mut caps: cap_t = ptr::null_mut();
    let mut ret: c_int = -1;

    caps = unsafe { cap_get_proc() };
    if caps.is_null() {
        unsafe { cap_free(caps) };
        return fret as c_int;
    }

    ret = unsafe { cap_clear_flag(caps, CAP_EFFECTIVE) };
    if ret != 0 {
        unsafe { cap_free(caps) };
        return fret as c_int;
    }

    ret = unsafe { cap_set_proc(caps) };
    if ret != 0 {
        unsafe { cap_free(caps) };
        return fret as c_int;
    }

    fret = true;

    unsafe { cap_free(caps) };
    fret as c_int
}

/* cap_down - lower an effective cap */
#[unsafe(no_mangle)]
pub unsafe extern "C" fn cap_down(down: cap_value_t) -> c_int {
    let mut fret = false;
    let mut caps: cap_t = ptr::null_mut();
    let mut cap: cap_value_t = down;
    let mut ret: c_int = -1;

    caps = unsafe { cap_get_proc() };
    if caps.is_null() {
        unsafe { cap_free(caps) };
        return fret as c_int;
    }

    ret = unsafe { cap_set_flag(caps, CAP_EFFECTIVE, 1, &mut cap, 0) };
    if ret != 0 {
        unsafe { cap_free(caps) };
        return fret as c_int;
    }

    ret = unsafe { cap_set_proc(caps) };
    if ret != 0 {
        unsafe { cap_free(caps) };
        return fret as c_int;
    }

    fret = true;

    unsafe { cap_free(caps) };
    fret as c_int
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn get_unique_mnt_id(path: *const c_char) -> uint64_t {
    let mut sx: statx = unsafe { mem::zeroed() };
    let ret: c_int;

    ret = unsafe { statx(AT_FDCWD, path, 0, STATX_MNT_ID_UNIQUE, &mut sx) };
    if ret == -1 {
        unsafe {
            ksft_print_msg(
                c!("retrieving unique mount ID for %s: %s\n"),
                path,
                strerror(errno()),
            )
        };
        return 0;
    }

    if (sx.stx_mask & STATX_MNT_ID_UNIQUE) == 0 {
        unsafe { ksft_print_msg(c!("no unique mount ID available for %s\n"), path) };
        return 0;
    }

    sx.stx_mnt_id
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
