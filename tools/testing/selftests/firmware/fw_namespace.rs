// SPDX-License-Identifier: GPL-2.0
/* Test triggering of loading of firmware from different mount
 * namespaces. Expect firmware to be always loaded from the mount
 * namespace of PID 1. */

use libc::{
    c_char, c_int, c_void, close, exit, fork, free, mount, open, pid_t, setvbuf, stderr, stdout,
    strerror, strlen, umount, unlink, unshare, waitpid, write, CLONE_NEWNS, EXIT_FAILURE,
    EXIT_SUCCESS, MS_RDONLY, MS_REC, MS_SLAVE, O_CREAT, O_WRONLY, _IONBF,
};
use std::ffi::CString;
use std::ptr;

static mut fw_path: *mut c_char = ptr::null_mut();

unsafe fn die_message(msg: &str) -> ! {
    let c_msg = CString::new(msg).unwrap();

    libc::fprintf(stderr, c_msg.as_ptr());
    if !fw_path.is_null() {
        unlink(fw_path);
    }
    umount(c"/lib/firmware".as_ptr());
    exit(EXIT_FAILURE);
}

unsafe fn die_message_errno(prefix: &str) -> ! {
    let err = strerror(*libc::__errno_location());
    let fmt = CString::new(format!("{}%s\n", prefix)).unwrap();

    libc::fprintf(stderr, fmt.as_ptr(), err);
    if !fw_path.is_null() {
        unlink(fw_path);
    }
    umount(c"/lib/firmware".as_ptr());
    exit(EXIT_FAILURE);
}

unsafe fn die_waited_for(child: pid_t, pid: pid_t) -> ! {
    libc::fprintf(stderr, c"waited for %d got %d\n".as_ptr(), child, pid);
    if !fw_path.is_null() {
        unlink(fw_path);
    }
    umount(c"/lib/firmware".as_ptr());
    exit(EXIT_FAILURE);
}

/* The C source defines a local variadic die(char *fmt, ...) helper using
 * va_start/vfprintf/va_end. Stable Rust cannot define an equivalent C-variadic
 * Rust function, so the translated call sites use the die_message,
 * die_message_errno, and die_waited_for helpers above to preserve the same
 * printing, cleanup, and exit behavior. */

unsafe fn trigger_fw(fw_name: *const c_char, sys_path: *const c_char) {
    let fd: c_int;

    fd = open(sys_path, O_WRONLY);
    if fd < 0 {
        die_message_errno("open failed: ");
    }
    if write(fd, fw_name as *const c_void, strlen(fw_name)) != strlen(fw_name) as isize {
        exit(EXIT_FAILURE);
    }
    close(fd);
}

unsafe fn setup_fw(fw_path_arg: *const c_char) {
    let fd: c_int;
    let fw = c"ABCD0123";

    fd = open(fw_path_arg, O_WRONLY | O_CREAT, 0o600);
    if fd < 0 {
        die_message_errno("open failed: ");
    }
    if write(fd, fw.as_ptr() as *const c_void, 8) != 8 {
        die_message_errno("write failed: ");
    }
    close(fd);
}

unsafe fn test_fw_in_ns(
    fw_name: *const c_char,
    sys_path: *const c_char,
    block_fw_in_parent_ns: bool,
) -> bool {
    let child: pid_t;

    if block_fw_in_parent_ns {
        if mount(
            c"test".as_ptr() as *const c_void,
            c"/lib/firmware".as_ptr(),
            c"tmpfs".as_ptr(),
            MS_RDONLY,
            ptr::null(),
        ) == -1
        {
            die_message("blocking firmware in parent ns failed\n");
        }
    }

    child = fork();
    if child == -1 {
        die_message_errno("fork failed: ");
    }
    if child != 0 {
        /* parent */
        let pid: pid_t;
        let mut status: c_int = 0;

        pid = waitpid(child, &mut status, 0);
        if pid == -1 {
            die_message_errno("waitpid failed: ");
        }
        if pid != child {
            die_waited_for(child, pid);
        }
        if !libc::WIFEXITED(status) {
            die_message("child did not terminate cleanly\n");
        }
        if block_fw_in_parent_ns {
            umount(c"/lib/firmware".as_ptr());
        }
        return libc::WEXITSTATUS(status) == EXIT_SUCCESS;
    }

    if unshare(CLONE_NEWNS) != 0 {
        die_message_errno("unshare(CLONE_NEWNS) failed: ");
    }
    if mount(
        ptr::null(),
        c"/".as_ptr(),
        ptr::null(),
        MS_SLAVE | MS_REC,
        ptr::null(),
    ) == -1
    {
        die_message("remount root in child ns failed\n");
    }

    if !block_fw_in_parent_ns {
        if mount(
            c"test".as_ptr() as *const c_void,
            c"/lib/firmware".as_ptr(),
            c"tmpfs".as_ptr(),
            MS_RDONLY,
            ptr::null(),
        ) == -1
        {
            die_message("blocking firmware in child ns failed\n");
        }
    } else {
        umount(c"/lib/firmware".as_ptr());
    }

    trigger_fw(fw_name, sys_path);

    exit(EXIT_SUCCESS);
}

#[no_mangle]
pub unsafe extern "C" fn main(argc: c_int, argv: *mut *mut c_char) -> c_int {
    let fw_name = c"test-firmware.bin";
    let sys_path: *mut c_char;

    if argc != 2 {
        let prog = if !argv.is_null() {
            *argv
        } else {
            c"fw_namespace".as_ptr() as *mut c_char
        };
        libc::fprintf(stderr, c"usage: %s sys_path\n".as_ptr(), prog);
        if !fw_path.is_null() {
            unlink(fw_path);
        }
        umount(c"/lib/firmware".as_ptr());
        exit(EXIT_FAILURE);
    }

    /* Mount tmpfs to /lib/firmware so we don't have to assume
       that it is writable for us.*/
    if mount(
        c"test".as_ptr() as *const c_void,
        c"/lib/firmware".as_ptr(),
        c"tmpfs".as_ptr(),
        0,
        ptr::null(),
    ) == -1
    {
        die_message("mounting tmpfs to /lib/firmware failed\n");
    }

    sys_path = *argv.add(1);
    let built_fw_path = CString::new(format!(
        "/lib/firmware/{}",
        fw_name.to_str().unwrap()
    ));
    if built_fw_path.is_err() {
        die_message("error: failed to build full fw_path\n");
    }
    fw_path = built_fw_path.unwrap().into_raw();

    setup_fw(fw_path);

    setvbuf(stdout, ptr::null_mut(), _IONBF, 0);
    /* Positive case: firmware in PID1 mount namespace */
    libc::printf(c"Testing with firmware in parent namespace (assumed to be same file system as PID1)\n".as_ptr());
    if !test_fw_in_ns(fw_name.as_ptr(), sys_path, false) {
        die_message("error: failed to access firmware\n");
    }

    /* Negative case: firmware in child mount namespace, expected to fail */
    libc::printf(c"Testing with firmware in child namespace\n".as_ptr());
    if test_fw_in_ns(fw_name.as_ptr(), sys_path, true) {
        die_message("error: firmware access did not fail\n");
    }

    unlink(fw_path);
    free(fw_path as *mut c_void);
    umount(c"/lib/firmware".as_ptr());
    exit(EXIT_SUCCESS);
}
