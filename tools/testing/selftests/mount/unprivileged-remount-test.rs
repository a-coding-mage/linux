// SPDX-License-Identifier: GPL-2.0
// Translated from testing/selftests/mount/unprivileged-remount-test.c

extern crate libc;

use libc::{
    c_char, c_int, c_ulong, c_void, gid_t, pid_t, ssize_t, uid_t, va_list, EXIT_FAILURE,
    EXIT_SUCCESS,
};
use std::ffi::{CStr, CString};
use std::mem;
use std::ptr;

const CLONE_NEWNS: c_int = 0x00020000;
const CLONE_NEWUTS: c_int = 0x04000000;
const CLONE_NEWIPC: c_int = 0x08000000;
const CLONE_NEWNET: c_int = 0x40000000;
const CLONE_NEWUSER: c_int = 0x10000000;
const CLONE_NEWPID: c_int = 0x20000000;

const MS_REC: c_ulong = 16384;
const MS_RELATIME: c_ulong = 1 << 21;
const MS_STRICTATIME: c_ulong = 1 << 24;

const O_WRONLY: c_int = libc::O_WRONLY;
const ENOENT: c_int = libc::ENOENT;

unsafe fn errno() -> c_int {
    *libc::__errno_location()
}

unsafe fn errno_string() -> String {
    CStr::from_ptr(libc::strerror(errno()))
        .to_string_lossy()
        .into_owned()
}

fn die(message: String) -> ! {
    unsafe {
        let bytes = message.as_bytes();
        libc::fwrite(
            bytes.as_ptr() as *const c_void,
            1,
            bytes.len(),
            libc::stderr,
        );
        libc::exit(EXIT_FAILURE);
    }
}

unsafe fn vmaybe_write_file(enoent_ok: bool, filename: &str, contents: String) {
    let filename_c = CString::new(filename).unwrap();
    let buf = contents.into_bytes();
    let buf_len = buf.len();

    let fd = libc::open(filename_c.as_ptr(), O_WRONLY);
    if fd < 0 {
        if errno() == ENOENT && enoent_ok {
            return;
        }
        die(format!(
            "open of {} failed: {}\n",
            filename,
            errno_string()
        ));
    }

    let written: ssize_t = libc::write(fd, buf.as_ptr() as *const c_void, buf_len);
    if written != buf_len as ssize_t {
        if written >= 0 {
            die(format!("short write to {}\n", filename));
        } else {
            die(format!(
                "write to {} failed: {}\n",
                filename,
                errno_string()
            ));
        }
    }
    if libc::close(fd) != 0 {
        die(format!(
            "close of {} failed: {}\n",
            filename,
            errno_string()
        ));
    }
}

macro_rules! maybe_write_file {
    ($filename:expr, $fmt:literal $(, $arg:expr)* $(,)?) => {{
        unsafe {
            vmaybe_write_file(true, $filename, format!($fmt $(, $arg)*));
        }
    }};
}

macro_rules! write_file {
    ($filename:expr, $fmt:literal $(, $arg:expr)* $(,)?) => {{
        unsafe {
            vmaybe_write_file(false, $filename, format!($fmt $(, $arg)*));
        }
    }};
}

unsafe fn read_mnt_flags(path: *const c_char) -> c_int {
    let mut stat: libc::statvfs = mem::zeroed();
    let ret = libc::statvfs(path, &mut stat);
    if ret != 0 {
        die(format!(
            "statvfs of {} failed: {}\n",
            CStr::from_ptr(path).to_string_lossy(),
            errno_string()
        ));
    }
    if stat.f_flag
        & !(libc::ST_RDONLY
            | libc::ST_NOSUID
            | libc::ST_NODEV
            | libc::ST_NOEXEC
            | libc::ST_NOATIME
            | libc::ST_NODIRATIME
            | libc::ST_RELATIME
            | libc::ST_SYNCHRONOUS
            | libc::ST_MANDLOCK)
        != 0
    {
        die("Unrecognized mount flags\n".to_string());
    }
    let mut mnt_flags: c_int = 0;
    if stat.f_flag & libc::ST_RDONLY != 0 {
        mnt_flags |= libc::MS_RDONLY as c_int;
    }
    if stat.f_flag & libc::ST_NOSUID != 0 {
        mnt_flags |= libc::MS_NOSUID as c_int;
    }
    if stat.f_flag & libc::ST_NODEV != 0 {
        mnt_flags |= libc::MS_NODEV as c_int;
    }
    if stat.f_flag & libc::ST_NOEXEC != 0 {
        mnt_flags |= libc::MS_NOEXEC as c_int;
    }
    if stat.f_flag & libc::ST_NOATIME != 0 {
        mnt_flags |= libc::MS_NOATIME as c_int;
    }
    if stat.f_flag & libc::ST_NODIRATIME != 0 {
        mnt_flags |= libc::MS_NODIRATIME as c_int;
    }
    if stat.f_flag & libc::ST_RELATIME != 0 {
        mnt_flags |= MS_RELATIME as c_int;
    }
    if stat.f_flag & libc::ST_SYNCHRONOUS != 0 {
        mnt_flags |= libc::MS_SYNCHRONOUS as c_int;
    }
    if stat.f_flag & libc::ST_MANDLOCK != 0 {
        mnt_flags |= libc::ST_MANDLOCK as c_int;
    }

    mnt_flags
}

unsafe fn create_and_enter_userns() {
    let uid: uid_t = libc::getuid();
    let gid: gid_t = libc::getgid();

    if libc::unshare(CLONE_NEWUSER) != 0 {
        die(format!(
            "unshare(CLONE_NEWUSER) failed: {}\n",
            errno_string()
        ));
    }

    maybe_write_file!("/proc/self/setgroups", "deny");
    write_file!("/proc/self/uid_map", "0 {} 1", uid);
    write_file!("/proc/self/gid_map", "0 {} 1", gid);

    if libc::setgid(0) != 0 {
        die(format!("setgid(0) failed {}\n", errno_string()));
    }
    if libc::setuid(0) != 0 {
        die(format!("setuid(0) failed {}\n", errno_string()));
    }
}

unsafe fn mount_call(
    source: *const c_char,
    target: *const c_char,
    filesystemtype: *const c_char,
    mountflags: c_ulong,
    data: *const c_void,
) -> c_int {
    libc::mount(source, target, filesystemtype, mountflags, data)
}

unsafe fn child_status_success(status: c_int) -> bool {
    libc::WIFEXITED(status) && libc::WEXITSTATUS(status) == EXIT_SUCCESS
}

unsafe fn test_unpriv_remount(
    fstype: *const c_char,
    mount_options: *const c_char,
    mount_flags: c_int,
    remount_flags: c_int,
    invalid_flags: c_int,
) -> bool {
    let child: pid_t = libc::fork();
    if child == -1 {
        die(format!("fork failed: {}\n", errno_string()));
    }
    if child != 0 {
        /* parent */
        let mut status: c_int = 0;
        let pid: pid_t = libc::waitpid(child, &mut status, 0);
        if pid == -1 {
            die(format!("waitpid failed: {}\n", errno_string()));
        }
        if pid != child {
            die(format!("waited for {} got {}\n", child, pid));
        }
        if !libc::WIFEXITED(status) {
            die("child did not terminate cleanly\n".to_string());
        }
        return child_status_success(status);
    }

    create_and_enter_userns();
    if libc::unshare(CLONE_NEWNS) != 0 {
        die(format!(
            "unshare(CLONE_NEWNS) failed: {}\n",
            errno_string()
        ));
    }

    let testing = CString::new("testing").unwrap();
    let tmp = CString::new("/tmp").unwrap();
    if mount_call(
        testing.as_ptr(),
        tmp.as_ptr(),
        fstype,
        mount_flags as c_ulong,
        mount_options as *const c_void,
    ) != 0
    {
        let options = if mount_options.is_null() {
            ""
        } else {
            CStr::from_ptr(mount_options).to_str().unwrap()
        };
        die(format!(
            "mount of {} with options '{}' on /tmp failed: {}\n",
            CStr::from_ptr(fstype).to_string_lossy(),
            options,
            errno_string()
        ));
    }

    create_and_enter_userns();

    if libc::unshare(CLONE_NEWNS) != 0 {
        die(format!(
            "unshare(CLONE_NEWNS) failed: {}\n",
            errno_string()
        ));
    }

    let none = CString::new("none").unwrap();
    if mount_call(
        tmp.as_ptr(),
        tmp.as_ptr(),
        none.as_ptr(),
        (libc::MS_REMOUNT | libc::MS_BIND | remount_flags as c_ulong) as c_ulong,
        ptr::null(),
    ) != 0
    {
        /* system("cat /proc/self/mounts"); */
        die(format!("remount of /tmp failed: {}\n", errno_string()));
    }

    if mount_call(
        tmp.as_ptr(),
        tmp.as_ptr(),
        none.as_ptr(),
        (libc::MS_REMOUNT | libc::MS_BIND | invalid_flags as c_ulong) as c_ulong,
        ptr::null(),
    ) == 0
    {
        /* system("cat /proc/self/mounts"); */
        die("remount of /tmp with invalid flags succeeded unexpectedly\n".to_string());
    }
    libc::exit(EXIT_SUCCESS);
}

unsafe fn test_unpriv_remount_simple(mount_flags: c_int) -> bool {
    let ramfs = CString::new("ramfs").unwrap();
    test_unpriv_remount(ramfs.as_ptr(), ptr::null(), mount_flags, mount_flags, 0)
}

unsafe fn test_unpriv_remount_atime(mount_flags: c_int, invalid_flags: c_int) -> bool {
    let ramfs = CString::new("ramfs").unwrap();
    test_unpriv_remount(
        ramfs.as_ptr(),
        ptr::null(),
        mount_flags,
        mount_flags,
        invalid_flags,
    )
}

unsafe fn test_priv_mount_unpriv_remount() -> bool {
    let child: pid_t;
    let mut ret: c_int;
    let orig_path = CString::new("/dev").unwrap();
    let dest_path = CString::new("/tmp").unwrap();
    let bind = CString::new("bind").unwrap();
    let none = CString::new("none").unwrap();
    let orig_mnt_flags: c_int;
    let remount_mnt_flags: c_int;

    child = libc::fork();
    if child == -1 {
        die(format!("fork failed: {}\n", errno_string()));
    }
    if child != 0 {
        /* parent */
        let mut status: c_int = 0;
        let pid: pid_t = libc::waitpid(child, &mut status, 0);
        if pid == -1 {
            die(format!("waitpid failed: {}\n", errno_string()));
        }
        if pid != child {
            die(format!("waited for {} got {}\n", child, pid));
        }
        if !libc::WIFEXITED(status) {
            die("child did not terminate cleanly\n".to_string());
        }
        return child_status_success(status);
    }

    orig_mnt_flags = read_mnt_flags(orig_path.as_ptr());

    create_and_enter_userns();
    ret = libc::unshare(CLONE_NEWNS);
    if ret != 0 {
        die(format!(
            "unshare(CLONE_NEWNS) failed: {}\n",
            errno_string()
        ));
    }

    ret = mount_call(
        orig_path.as_ptr(),
        dest_path.as_ptr(),
        bind.as_ptr(),
        libc::MS_BIND | MS_REC,
        ptr::null(),
    );
    if ret != 0 {
        die(format!(
            "recursive bind mount of {} onto {} failed: {}\n",
            orig_path.to_string_lossy(),
            dest_path.to_string_lossy(),
            errno_string()
        ));
    }

    ret = mount_call(
        dest_path.as_ptr(),
        dest_path.as_ptr(),
        none.as_ptr(),
        libc::MS_REMOUNT | libc::MS_BIND | orig_mnt_flags as c_ulong,
        ptr::null(),
    );
    if ret != 0 {
        /* system("cat /proc/self/mounts"); */
        die(format!("remount of /tmp failed: {}\n", errno_string()));
    }

    remount_mnt_flags = read_mnt_flags(dest_path.as_ptr());
    if orig_mnt_flags != remount_mnt_flags {
        die(format!(
            "Mount flags unexpectedly changed during remount of {} originally mounted on {}\n",
            dest_path.to_string_lossy(),
            orig_path.to_string_lossy()
        ));
    }
    libc::exit(EXIT_SUCCESS);
}

fn main() {
    unsafe {
        if !test_unpriv_remount_simple(libc::MS_RDONLY as c_int) {
            die("MS_RDONLY malfunctions\n".to_string());
        }
        let devpts = CString::new("devpts").unwrap();
        let newinstance = CString::new("newinstance").unwrap();
        if !test_unpriv_remount(
            devpts.as_ptr(),
            newinstance.as_ptr(),
            libc::MS_NODEV as c_int,
            libc::MS_NODEV as c_int,
            0,
        ) {
            die("MS_NODEV malfunctions\n".to_string());
        }
        if !test_unpriv_remount_simple(libc::MS_NOSUID as c_int) {
            die("MS_NOSUID malfunctions\n".to_string());
        }
        if !test_unpriv_remount_simple(libc::MS_NOEXEC as c_int) {
            die("MS_NOEXEC malfunctions\n".to_string());
        }
        if !test_unpriv_remount_atime(MS_RELATIME as c_int, libc::MS_NOATIME as c_int) {
            die("MS_RELATIME malfunctions\n".to_string());
        }
        if !test_unpriv_remount_atime(MS_STRICTATIME as c_int, libc::MS_NOATIME as c_int) {
            die("MS_STRICTATIME malfunctions\n".to_string());
        }
        if !test_unpriv_remount_atime(libc::MS_NOATIME as c_int, MS_STRICTATIME as c_int) {
            die("MS_NOATIME malfunctions\n".to_string());
        }
        if !test_unpriv_remount_atime(
            (MS_RELATIME | libc::MS_NODIRATIME) as c_int,
            libc::MS_NOATIME as c_int,
        ) {
            die("MS_RELATIME|MS_NODIRATIME malfunctions\n".to_string());
        }
        if !test_unpriv_remount_atime(
            (MS_STRICTATIME | libc::MS_NODIRATIME) as c_int,
            libc::MS_NOATIME as c_int,
        ) {
            die("MS_STRICTATIME|MS_NODIRATIME malfunctions\n".to_string());
        }
        if !test_unpriv_remount_atime(
            (libc::MS_NOATIME | libc::MS_NODIRATIME) as c_int,
            MS_STRICTATIME as c_int,
        ) {
            die("MS_NOATIME|MS_DIRATIME malfunctions\n".to_string());
        }
        let ramfs = CString::new("ramfs").unwrap();
        if !test_unpriv_remount(
            ramfs.as_ptr(),
            ptr::null(),
            MS_STRICTATIME as c_int,
            0,
            libc::MS_NOATIME as c_int,
        ) {
            die("Default atime malfunctions\n".to_string());
        }
        if !test_priv_mount_unpriv_remount() {
            die("Mount flags unexpectedly changed after remount\n".to_string());
        }
        libc::exit(EXIT_SUCCESS);
    }
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
