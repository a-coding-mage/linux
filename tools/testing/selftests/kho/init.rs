// SPDX-License-Identifier: GPL-2.0

use core::ffi::{c_char, c_int, c_long, c_ulong, c_void};

/* C dependencies originally included:
 * <stdio.h>, <unistd.h>, <fcntl.h>, <sys/syscall.h>, <sys/mount.h>,
 * <sys/reboot.h>, <linux/kexec.h>
 */

/* from arch/x86/include/asm/setup.h */
const COMMAND_LINE_SIZE: usize = 2048;

const KERNEL_IMAGE: &[u8] = b"/kernel\0";

const O_RDONLY: c_int = 0;
const __NR_kexec_file_load: c_long = 320;
const KEXEC_FILE_NO_INITRAMFS: c_ulong = 0x00000004;
const RB_AUTOBOOT: c_int = 0x01234567;
const RB_KEXEC: c_int = 0x45584543;

unsafe extern "C" {
    fn mount(
        source: *const c_char,
        target: *const c_char,
        filesystemtype: *const c_char,
        mountflags: c_ulong,
        data: *const c_void,
    ) -> c_int;
    fn syscall(num: c_long, ...) -> c_long;
    fn open(pathname: *const c_char, flags: c_int, ...) -> c_int;
    fn read(fd: c_int, buf: *mut c_void, count: usize) -> isize;
    fn close(fd: c_int) -> c_int;
    fn reboot(cmd: c_int) -> c_int;
}

unsafe fn mount_filesystems() -> c_int {
    if unsafe {
        mount(
            c"debugfs".as_ptr(),
            c"/debugfs".as_ptr(),
            c"debugfs".as_ptr(),
            0,
            core::ptr::null(),
        )
    } < 0
    {
        return -1;
    }

    unsafe {
        mount(
            c"proc".as_ptr(),
            c"/proc".as_ptr(),
            c"proc".as_ptr(),
            0,
            core::ptr::null(),
        )
    }
}

unsafe fn kexec_file_load(
    kernel_fd: c_int,
    initrd_fd: c_int,
    cmdline_len: c_ulong,
    cmdline: *const c_char,
    flags: c_ulong,
) -> c_long {
    unsafe {
        syscall(
            __NR_kexec_file_load,
            kernel_fd,
            initrd_fd,
            cmdline_len,
            cmdline,
            flags,
        )
    }
}

unsafe fn kexec_load() -> c_int {
    let mut cmdline = [0 as c_char; COMMAND_LINE_SIZE];
    let len: isize;
    let mut fd: c_int;
    let err: c_int;

    fd = unsafe { open(c"/proc/cmdline".as_ptr(), O_RDONLY) };
    if fd < 0 {
        return -1;
    }

    len = unsafe { read(fd, cmdline.as_mut_ptr() as *mut c_void, core::mem::size_of_val(&cmdline)) };
    unsafe {
        close(fd);
    }
    if len < 0 {
        return -1;
    }

    /* replace \n with \0 */
    cmdline[(len - 1) as usize] = 0;
    fd = unsafe { open(KERNEL_IMAGE.as_ptr() as *const c_char, O_RDONLY) };
    if fd < 0 {
        return -1;
    }

    err = unsafe {
        kexec_file_load(
            fd,
            -1,
            len as c_ulong,
            cmdline.as_ptr(),
            KEXEC_FILE_NO_INITRAMFS,
        ) as c_int
    };
    unsafe {
        close(fd);
    }

    if err != 0 { err } else { 0 }
}

#[no_mangle]
pub unsafe extern "C" fn main(_argc: c_int, _argv: *mut *mut c_char) -> c_int {
    if unsafe { mount_filesystems() } != 0 {
        unsafe {
            reboot(RB_AUTOBOOT);
        }
        return -1;
    }

    if unsafe { kexec_load() } != 0 {
        unsafe {
            reboot(RB_AUTOBOOT);
        }
        return -1;
    }

    if unsafe { reboot(RB_KEXEC) } != 0 {
        unsafe {
            reboot(RB_AUTOBOOT);
        }
        return -1;
    }

    0
}
