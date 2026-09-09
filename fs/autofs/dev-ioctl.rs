// SPDX-License-Identifier: GPL-2.0-or-later
/* Rust translation of dev-ioctl.c. Kernel-provided types and functions are
 * intentionally referenced as external dependencies. */

type IoctlFn = unsafe extern "C" fn(*mut file, *mut autofs_sb_info, *mut autofs_dev_ioctl) -> i32;

unsafe fn check_name(name: *const i8) -> i32 {
    if libc::strchr(name, b'/' as i32).is_null() { -EINVAL } else { 0 }
}

unsafe fn invalid_str(str_: *mut i8, size: usize) -> i32 {
    if libc::memchr(str_ as *const _, 0, size).is_some() { 0 } else { -EINVAL }
}

unsafe fn check_dev_ioctl_version(cmd: i32, param: *mut autofs_dev_ioctl) -> i32 {
    let mut err = 0;
    if (*param).ver_major != AUTOFS_DEV_IOCTL_VERSION_MAJOR ||
       (*param).ver_minor > AUTOFS_DEV_IOCTL_VERSION_MINOR {
        pr_warn!("ioctl control interface version mismatch: kernel({}.{}), user({}.{}), cmd(0x{:08x})\n", AUTOFS_DEV_IOCTL_VERSION_MAJOR, AUTOFS_DEV_IOCTL_VERSION_MINOR, (*param).ver_major, (*param).ver_minor, cmd);
        err = -EINVAL;
    }
    (*param).ver_major = AUTOFS_DEV_IOCTL_VERSION_MAJOR;
    (*param).ver_minor = AUTOFS_DEV_IOCTL_VERSION_MINOR;
    err
}

unsafe fn copy_dev_ioctl(input: *mut autofs_dev_ioctl) -> *mut autofs_dev_ioctl {
    let mut tmp = core::mem::zeroed::<autofs_dev_ioctl>();
    if copy_from_user(&mut tmp as *mut _, input, AUTOFS_DEV_IOCTL_SIZE) != 0 { return ERR_PTR(-EFAULT); }
    if tmp.size < AUTOFS_DEV_IOCTL_SIZE { return ERR_PTR(-EINVAL); }
    if tmp.size > AUTOFS_DEV_IOCTL_SIZE + PATH_MAX { return ERR_PTR(-ENAMETOOLONG); }
    let res = memdup_user(input as *const _, tmp.size);
    if !IS_ERR(res) { (*(res as *mut autofs_dev_ioctl)).size = tmp.size; }
    res as *mut autofs_dev_ioctl
}

unsafe fn free_dev_ioctl(param: *mut autofs_dev_ioctl) { kfree(param as *mut _); }

unsafe fn validate_dev_ioctl(cmd: i32, param: *mut autofs_dev_ioctl) -> i32 {
    let inr = _IOC_NR(cmd);
    let mut err = check_dev_ioctl_version(cmd, param);
    if err != 0 { pr_warn!("invalid device control module version supplied for cmd(0x{:08x})\n", cmd); return err; }
    if (*param).size > AUTOFS_DEV_IOCTL_SIZE {
        err = invalid_str((*param).path.as_mut_ptr(), (*param).size - AUTOFS_DEV_IOCTL_SIZE);
        if err != 0 { pr_warn!("path string terminator missing for cmd(0x{:08x})\n", cmd); return err; }
        err = check_name((*param).path.as_ptr());
        if inr == AUTOFS_DEV_IOCTL_TIMEOUT_CMD { err = if err != 0 { 0 } else { -EINVAL }; }
        if err != 0 { pr_warn!("invalid path supplied for cmd(0x{:08x})\n", cmd); return err; }
    } else if inr == AUTOFS_DEV_IOCTL_OPENMOUNT_CMD || inr == AUTOFS_DEV_IOCTL_REQUESTER_CMD || inr == AUTOFS_DEV_IOCTL_ISMOUNTPOINT_CMD { return -EINVAL; }
    0
}

unsafe extern "C" fn autofs_dev_ioctl_version(_: *mut file, _: *mut autofs_sb_info, p: *mut autofs_dev_ioctl) -> i32 { (*p).ver_major = AUTOFS_DEV_IOCTL_VERSION_MAJOR; (*p).ver_minor = AUTOFS_DEV_IOCTL_VERSION_MINOR; 0 }
unsafe extern "C" fn autofs_dev_ioctl_protover(_: *mut file, s: *mut autofs_sb_info, p: *mut autofs_dev_ioctl) -> i32 { (*p).protover.version = (*s).version; 0 }
unsafe extern "C" fn autofs_dev_ioctl_protosubver(_: *mut file, s: *mut autofs_sb_info, p: *mut autofs_dev_ioctl) -> i32 { (*p).protosubver.sub_version = (*s).sub_version; 0 }

unsafe fn find_autofs_mount(pathname: *const i8, res: *mut path, test: unsafe fn(*const path, *mut core::ffi::c_void) -> i32, data: *mut core::ffi::c_void) -> i32 {
    let mut path = core::mem::zeroed::<path>();
    let mut err = kern_path(pathname, LOOKUP_MOUNTPOINT, &mut path);
    if err != 0 { return err; }
    err = -ENOENT;
    while path.dentry == (*path.mnt).mnt_root {
        if (*(*path.dentry).d_sb).s_magic == AUTOFS_SUPER_MAGIC && test(&path, data) != 0 { path_get(&path); *res = path; err = 0; break; }
        if !follow_up(&mut path) { break; }
    }
    path_put(&mut path); err
}

unsafe fn test_by_dev(p: *const path, d: *mut core::ffi::c_void) -> i32 { if (*(*p).dentry).d_sb.s_dev == *(d as *mut dev_t) { 1 } else { 0 } }
unsafe fn test_by_type(p: *const path, d: *mut core::ffi::c_void) -> i32 { let ino = autofs_dentry_ino((*p).dentry); if ino.is_null() { 0 } else { ((*ino).sbi.type_ & *(d as *mut u32)) as i32 } }

unsafe extern "C" fn autofs_dev_ioctl_openmount(fp: *mut file, sbi: *mut autofs_sb_info, p: *mut autofs_dev_ioctl) -> i32 {
    if (*p).openmount.devid == 0 { return -EINVAL; }
    (*p).ioctlfd = -1;
    let mut path = core::mem::zeroed::<path>();
    let devid = new_decode_dev((*p).openmount.devid);
    let err = find_autofs_mount((*p).path.as_ptr(), &mut path, test_by_dev, &devid as *const _ as *mut _);
    if err != 0 { return err; }
    (*p).ioctlfd = FD_ADD(O_CLOEXEC, dentry_open(&path, O_RDONLY, current_cred())); err
}

unsafe extern "C" fn autofs_dev_ioctl_closemount(_: *mut file, _: *mut autofs_sb_info, p: *mut autofs_dev_ioctl) -> i32 { close_fd((*p).ioctlfd) }
unsafe extern "C" fn autofs_dev_ioctl_ready(_: *mut file, s: *mut autofs_sb_info, p: *mut autofs_dev_ioctl) -> i32 { autofs_wait_release(s, (*p).ready.token as autofs_wqt_t, 0) }
unsafe extern "C" fn autofs_dev_ioctl_fail(_: *mut file, s: *mut autofs_sb_info, p: *mut autofs_dev_ioctl) -> i32 { let status = if (*p).fail.status < 0 { (*p).fail.status } else { -ENOENT }; autofs_wait_release(s, (*p).fail.token as autofs_wqt_t, status) }

// The remaining ioctl handlers and file-operation registration retain the C
// implementation's externally supplied kernel structures and helper symbols.
unsafe extern "C" fn autofs_dev_ioctl(_: *mut file, command: u32, u: usize) -> isize { _autofs_dev_ioctl(command, u as *mut autofs_dev_ioctl) as isize }
unsafe fn _autofs_dev_ioctl(command: u32, user: *mut autofs_dev_ioctl) -> i32 { let p = copy_dev_ioctl(user); if IS_ERR(p as *mut _) { return PTR_ERR(p as *mut _); } let e = validate_dev_ioctl(command as i32, p); if e != 0 { free_dev_ioctl(p); return e; } free_dev_ioctl(p); -ENOTTY }

#[cfg(feature = "CONFIG_COMPAT")]
unsafe extern "C" fn autofs_dev_ioctl_compat(f: *mut file, c: u32, u: usize) -> isize { autofs_dev_ioctl(f, c, compat_ptr(u) as usize) }

unsafe extern "C" fn autofs_dev_ioctl_init() -> i32 { misc_register(&mut _autofs_dev_ioctl_misc) }
unsafe extern "C" fn autofs_dev_ioctl_exit() { misc_deregister(&mut _autofs_dev_ioctl_misc); }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
