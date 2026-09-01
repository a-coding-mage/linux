// SPDX-License-Identifier: GPL-2.0-only
/*
 * An implementation of the host initiated guest snapshot for Hyper-V.
 *
 * Copyright (C) 2013, Microsoft, Inc.
 * Author : K. Y. Srinivasan <kys@microsoft.com>
 */

// C includes translated as external libc/kernel dependencies:
// sys/types.h, sys/poll.h, sys/ioctl.h, sys/stat.h, sys/sysmacros.h,
// fcntl.h, stdio.h, mntent.h, stdlib.h, unistd.h, string.h, ctype.h,
// errno.h, linux/fs.h, linux/major.h, linux/hyperv.h, syslog.h,
// getopt.h, stdbool.h, dirent.h.

use libc::{
    access, c_char, c_int, c_long, c_uint, c_void, close, daemon, exit, fclose, fprintf, free,
    getpid, ioctl, malloc, open, opendir, poll, pollfd, read, readdir, snprintf, sprintf, stat,
    strcmp, strerror, strlen, strncmp, strncpy, syslog, write, DIR, FILE, O_RDONLY, O_RDWR, PATH_MAX,
    POLLIN, R_OK, X_OK,
};

unsafe extern "C" {
    static mut errno: c_int;
    static mut stderr: *mut FILE;

    fn setmntent(filename: *const c_char, ty: *const c_char) -> *mut FILE;
    fn getmntent(stream: *mut FILE) -> *mut mntent;
    fn endmntent(stream: *mut FILE) -> c_int;
    fn hasmntopt(mnt: *const mntent, opt: *const c_char) -> *mut c_char;
    fn getopt_long(
        argc: c_int,
        argv: *mut *mut c_char,
        shortopts: *const c_char,
        longopts: *const option,
        longindex: *mut c_int,
    ) -> c_int;
    fn openlog(ident: *const c_char, option: c_int, facility: c_int);
}

#[repr(C)]
pub struct mntent {
    pub mnt_fsname: *mut c_char,
    pub mnt_dir: *mut c_char,
    pub mnt_type: *mut c_char,
    pub mnt_opts: *mut c_char,
    pub mnt_freq: c_int,
    pub mnt_passno: c_int,
}

#[repr(C)]
pub struct option {
    pub name: *const c_char,
    pub has_arg: c_int,
    pub flag: *mut c_int,
    pub val: c_int,
}

#[repr(C)]
pub struct dirent {
    pub d_ino: libc::ino_t,
    pub d_off: libc::off_t,
    pub d_reclen: libc::c_ushort,
    pub d_type: libc::c_uchar,
    pub d_name: [c_char; 256],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hv_vss_hdr {
    pub operation: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hv_vss_msg {
    pub vss_hdr: hv_vss_hdr,
    pub error: i32,
    pub body: [u8; 48],
}

const LOG_ERR: c_int = 3;
const LOG_INFO: c_int = 6;
const LOG_USER: c_int = 1 << 3;
const EXIT_FAILURE: c_int = 1;
const ENOENT: c_int = 2;
const EINVAL: c_int = 22;
const EBUSY: c_int = 16;
const MNTOPT_RO: *const c_char = b"ro\0".as_ptr() as *const c_char;

const NO_ARGUMENT: c_int = 0;

const VSS_OP_FREEZE: c_int = 0;
const VSS_OP_THAW: c_int = 1;
const VSS_OP_HOT_BACKUP: c_int = 2;
const VSS_OP_REGISTER1: u32 = 4;

const HV_S_OK: c_int = 0;
const HV_E_FAIL: c_int = -1;

const FIFREEZE: c_uint = 0xc0045877;
const FITHAW: c_uint = 0xc0045878;

static mut fs_frozen: bool = false;

unsafe fn major(dev: libc::dev_t) -> c_uint {
    (((dev >> 8) & 0xfff) | ((dev >> 32) & !0xfff)) as c_uint
}

unsafe fn minor(dev: libc::dev_t) -> c_uint {
    ((dev & 0xff) | ((dev >> 12) & !0xff)) as c_uint
}

/* Don't use syslog() in the function since that can cause write to disk */
unsafe fn vss_do_freeze(dir: *mut c_char, cmd: c_uint) -> c_int {
    let mut ret: c_int;
    let fd: c_int = open(dir, O_RDONLY);

    if fd < 0 {
        return 1;
    }

    ret = ioctl(fd, cmd as c_long, 0);

    /*
     * If a partition is mounted more than once, only the first
     * FREEZE/THAW can succeed and the later ones will get
     * EBUSY/EINVAL respectively: there could be 2 cases:
     * 1) a user may mount the same partition to different directories
     *  by mistake or on purpose;
     * 2) The subvolume of btrfs appears to have the same partition
     * mounted more than once.
     */
    if ret != 0 {
        if (cmd == FIFREEZE && errno == EBUSY) || (cmd == FITHAW && errno == EINVAL) {
            close(fd);
            return 0;
        }
    }

    close(fd);
    if ret != 0 { 1 } else { 0 }
}

unsafe fn is_dev_loop(blkname: *const c_char) -> bool {
    let mut buffer: *mut c_char;
    let mut dir: *mut DIR;
    let mut entry: *mut dirent;
    let mut ret: bool = false;

    buffer = malloc(PATH_MAX as usize) as *mut c_char;
    if buffer.is_null() {
        syslog(LOG_ERR, c"Can't allocate memory!".as_ptr());
        exit(1);
    }

    snprintf(buffer, PATH_MAX as usize, c"%s/loop".as_ptr(), blkname);
    if access(buffer, R_OK | X_OK) == 0 {
        ret = true;
        goto_free_buffer(buffer, ret)
    } else {
        if errno != ENOENT {
            syslog(
                LOG_ERR,
                c"Can't access: %s; error:%d %s!".as_ptr(),
                buffer,
                errno,
                strerror(errno),
            );
        }

        snprintf(buffer, PATH_MAX as usize, c"%s/slaves".as_ptr(), blkname);
        dir = opendir(buffer);
        if dir.is_null() {
            if errno != ENOENT {
                syslog(
                    LOG_ERR,
                    c"Can't opendir: %s; error:%d %s!".as_ptr(),
                    buffer,
                    errno,
                    strerror(errno),
                );
            }
            goto_free_buffer(buffer, ret)
        } else {
            loop {
                entry = readdir(dir) as *mut dirent;
                if entry.is_null() {
                    break;
                }

                if strcmp((*entry).d_name.as_ptr(), c".".as_ptr()) == 0
                    || strcmp((*entry).d_name.as_ptr(), c"..".as_ptr()) == 0
                {
                    continue;
                }

                snprintf(
                    buffer,
                    PATH_MAX as usize,
                    c"%s/slaves/%s".as_ptr(),
                    blkname,
                    (*entry).d_name.as_ptr(),
                );
                if is_dev_loop(buffer) {
                    ret = true;
                    break;
                }
            }
            libc::closedir(dir);
            goto_free_buffer(buffer, ret)
        }
    }
}

unsafe fn goto_free_buffer(buffer: *mut c_char, ret: bool) -> bool {
    free(buffer as *mut c_void);
    ret
}

unsafe fn vss_operate(operation: c_int) -> c_int {
    let match_: [c_char; 6] = *b"/dev/\0".as_ptr().cast::<[c_char; 6]>();
    let mut mounts: *mut FILE;
    let mut ent: *mut mntent;
    let mut sb: stat = std::mem::zeroed();
    let mut errdir: [c_char; 1024] = [0; 1024];
    let mut blkdir: [c_char; 23] = [0; 23]; /* /sys/dev/block/XXX:XXX */
    let cmd: c_uint;
    let mut error: c_int = 0;
    let mut root_seen: c_int = 0;
    let mut save_errno: c_int = 0;

    match operation {
        VSS_OP_FREEZE => {
            cmd = FIFREEZE;
        }
        VSS_OP_THAW => {
            cmd = FITHAW;
        }
        _ => {
            return -1;
        }
    }

    mounts = setmntent(c"/proc/mounts".as_ptr(), c"r".as_ptr());
    if mounts.is_null() {
        return -1;
    }

    loop {
        ent = getmntent(mounts);
        if ent.is_null() {
            break;
        }

        if strncmp((*ent).mnt_fsname, match_.as_ptr(), strlen(match_.as_ptr())) != 0 {
            continue;
        }
        if stat((*ent).mnt_fsname, &mut sb) != 0 {
            syslog(
                LOG_ERR,
                c"Can't stat: %s; error:%d %s!".as_ptr(),
                (*ent).mnt_fsname,
                errno,
                strerror(errno),
            );
        } else {
            sprintf(
                blkdir.as_mut_ptr(),
                c"/sys/dev/block/%d:%d".as_ptr(),
                major(sb.st_rdev),
                minor(sb.st_rdev),
            );
            if is_dev_loop(blkdir.as_ptr()) {
                continue;
            }
        }
        if !hasmntopt(ent, MNTOPT_RO).is_null() {
            continue;
        }
        if strcmp((*ent).mnt_type, c"vfat".as_ptr()) == 0 {
            continue;
        }
        if strcmp((*ent).mnt_dir, c"/".as_ptr()) == 0 {
            root_seen = 1;
            continue;
        }
        error |= vss_do_freeze((*ent).mnt_dir, cmd);
        if operation == VSS_OP_FREEZE {
            if error != 0 {
                goto_err(&mut error, &mut save_errno, ent, mounts, &mut errdir);
                return error;
            }
            fs_frozen = true;
        }
    }

    endmntent(mounts);

    if root_seen != 0 {
        error |= vss_do_freeze(c"/".as_ptr() as *mut c_char, cmd);
        if operation == VSS_OP_FREEZE {
            if error != 0 {
                ent = std::ptr::null_mut();
                goto_err(&mut error, &mut save_errno, ent, mounts, &mut errdir);
                return error;
            }
            fs_frozen = true;
        }
    }

    if operation == VSS_OP_THAW && error == 0 {
        fs_frozen = false;
    }

    error
}

unsafe fn goto_err(
    error: *mut c_int,
    save_errno: *mut c_int,
    ent: *mut mntent,
    mounts: *mut FILE,
    errdir: *mut [c_char; 1024],
) {
    *save_errno = errno;
    if !ent.is_null() {
        strncpy(
            (*errdir).as_mut_ptr(),
            (*ent).mnt_dir,
            std::mem::size_of::<[c_char; 1024]>() - 1,
        );
        endmntent(mounts);
    }
    vss_operate(VSS_OP_THAW);
    fs_frozen = false;
    /* Call syslog after we thaw all filesystems */
    if !ent.is_null() {
        syslog(
            LOG_ERR,
            c"FREEZE of %s failed; error:%d %s".as_ptr(),
            (*errdir).as_ptr(),
            *save_errno,
            strerror(*save_errno),
        );
    } else {
        syslog(
            LOG_ERR,
            c"FREEZE of / failed; error:%d %s".as_ptr(),
            *save_errno,
            strerror(*save_errno),
        );
    }
    let _ = error;
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn print_usage(argv: *mut *mut c_char) {
    fprintf(
        stderr,
        c"Usage: %s [options]\nOptions are:\n  -n, --no-daemon        stay in foreground, don't daemonize\n  -h, --help             print this help\n"
            .as_ptr(),
        *argv,
    );
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn main(argc: c_int, argv: *mut *mut c_char) -> c_int {
    let mut vss_fd: c_int = -1;
    let mut len: c_int;
    let mut error: c_int;
    let mut pfd: pollfd = std::mem::zeroed();
    let mut op: c_int;
    let mut vss_msg: [hv_vss_msg; 1] = [std::mem::zeroed(); 1];
    let mut daemonize: c_int = 1;
    let mut long_index: c_int = 0;
    let mut opt: c_int;
    let mut in_handshake: c_int;
    let mut kernel_modver: u32;

    let long_options: [option; 3] = [
        option {
            name: c"help".as_ptr(),
            has_arg: NO_ARGUMENT,
            flag: std::ptr::null_mut(),
            val: 'h' as c_int,
        },
        option {
            name: c"no-daemon".as_ptr(),
            has_arg: NO_ARGUMENT,
            flag: std::ptr::null_mut(),
            val: 'n' as c_int,
        },
        option {
            name: std::ptr::null(),
            has_arg: 0,
            flag: std::ptr::null_mut(),
            val: 0,
        },
    ];

    loop {
        opt = getopt_long(
            argc,
            argv,
            c"hn".as_ptr(),
            long_options.as_ptr(),
            &mut long_index,
        );
        if opt == -1 {
            break;
        }
        match opt {
            x if x == 'n' as c_int => {
                daemonize = 0;
            }
            x if x == 'h' as c_int => {
                print_usage(argv);
                exit(0);
            }
            _ => {
                print_usage(argv);
                exit(EXIT_FAILURE);
            }
        }
    }

    if daemonize != 0 && daemon(1, 0) != 0 {
        return 1;
    }

    openlog(c"Hyper-V VSS".as_ptr(), 0, LOG_USER);
    syslog(LOG_INFO, c"VSS starting; pid is:%d".as_ptr(), getpid());

    loop {
        if vss_fd != -1 {
            close(vss_fd);
        }
        if fs_frozen {
            if vss_operate(VSS_OP_THAW) != 0 || fs_frozen {
                syslog(
                    LOG_ERR,
                    c"failed to thaw file system: err=%d".as_ptr(),
                    errno,
                );
                exit(EXIT_FAILURE);
            }
        }

        in_handshake = 1;
        vss_fd = open(c"/dev/vmbus/hv_vss".as_ptr(), O_RDWR);
        if vss_fd < 0 {
            syslog(
                LOG_ERR,
                c"open /dev/vmbus/hv_vss failed; error: %d %s".as_ptr(),
                errno,
                strerror(errno),
            );
            exit(EXIT_FAILURE);
        }
        /*
         * Register ourselves with the kernel.
         */
        vss_msg[0].vss_hdr.operation = VSS_OP_REGISTER1;

        len = write(
            vss_fd,
            vss_msg.as_mut_ptr() as *mut c_void,
            std::mem::size_of::<hv_vss_msg>(),
        ) as c_int;
        if len < 0 {
            syslog(
                LOG_ERR,
                c"registration to kernel failed; error: %d %s".as_ptr(),
                errno,
                strerror(errno),
            );
            close(vss_fd);
            exit(EXIT_FAILURE);
        }

        pfd.fd = vss_fd;

        loop {
            pfd.events = POLLIN;
            pfd.revents = 0;

            if poll(&mut pfd, 1, -1) < 0 {
                syslog(
                    LOG_ERR,
                    c"poll failed; error:%d %s".as_ptr(),
                    errno,
                    strerror(errno),
                );
                if errno == EINVAL {
                    close(vss_fd);
                    exit(EXIT_FAILURE);
                } else {
                    continue;
                }
            }

            len = read(
                vss_fd,
                vss_msg.as_mut_ptr() as *mut c_void,
                std::mem::size_of::<hv_vss_msg>(),
            ) as c_int;

            if in_handshake != 0 {
                if len as usize != std::mem::size_of::<u32>() {
                    syslog(LOG_ERR, c"invalid version negotiation".as_ptr());
                    exit(EXIT_FAILURE);
                }
                kernel_modver = *(vss_msg.as_ptr() as *const u32);
                in_handshake = 0;
                syslog(
                    LOG_INFO,
                    c"VSS: kernel module version: %d".as_ptr(),
                    kernel_modver,
                );
                continue;
            }

            if len as usize != std::mem::size_of::<hv_vss_msg>() {
                syslog(
                    LOG_ERR,
                    c"read failed; error:%d %s".as_ptr(),
                    errno,
                    strerror(errno),
                );
                break;
            }

            op = vss_msg[0].vss_hdr.operation as c_int;
            error = HV_S_OK;

            match op {
                VSS_OP_FREEZE | VSS_OP_THAW => {
                    error = vss_operate(op);
                    syslog(
                        LOG_INFO,
                        c"VSS: op=%s: %s\n".as_ptr(),
                        if op == VSS_OP_FREEZE {
                            c"FREEZE".as_ptr()
                        } else {
                            c"THAW".as_ptr()
                        },
                        if error != 0 {
                            c"failed".as_ptr()
                        } else {
                            c"succeeded".as_ptr()
                        },
                    );

                    if error != 0 {
                        error = HV_E_FAIL;
                        syslog(LOG_ERR, c"op=%d failed!".as_ptr(), op);
                        syslog(LOG_ERR, c"report it with these files:".as_ptr());
                        syslog(LOG_ERR, c"/etc/fstab and /proc/mounts".as_ptr());
                    }
                }
                VSS_OP_HOT_BACKUP => {
                    syslog(LOG_INFO, c"VSS: op=CHECK HOT BACKUP\n".as_ptr());
                }
                _ => {
                    syslog(LOG_ERR, c"Illegal op:%d\n".as_ptr(), op);
                }
            }

            /*
             * The write() may return an error due to the faked VSS_OP_THAW
             * message upon hibernation. Ignore the error by resetting the
             * dev file, i.e. closing and re-opening it.
             */
            vss_msg[0].error = error;
            len = write(
                vss_fd,
                vss_msg.as_mut_ptr() as *mut c_void,
                std::mem::size_of::<hv_vss_msg>(),
            ) as c_int;
            if len as usize != std::mem::size_of::<hv_vss_msg>() {
                syslog(
                    LOG_ERR,
                    c"write failed; error: %d %s".as_ptr(),
                    errno,
                    strerror(errno),
                );
                break;
            }
        }
    }

    close(vss_fd);
    exit(0);
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
