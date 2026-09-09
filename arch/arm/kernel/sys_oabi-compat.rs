// SPDX-License-Identifier: GPL-2.0-only
/* Compatibility wrappers for old ARM ABI syscall structures. */

// Kernel headers and configuration-provided symbols are external dependencies.

#[repr(C, packed(4))]
pub struct oldabi_stat64 {
    pub st_dev: u64,
    pub __pad1: u32,
    pub __st_ino: libc::c_ulong,
    pub st_mode: u32,
    pub st_nlink: u32,
    pub st_uid: libc::c_ulong,
    pub st_gid: libc::c_ulong,
    pub st_rdev: u64,
    pub __pad2: u32,
    pub st_size: i64,
    pub st_blksize: libc::c_ulong,
    pub st_blocks: u64,
    pub st_atime: libc::c_ulong,
    pub st_atime_nsec: libc::c_ulong,
    pub st_mtime: libc::c_ulong,
    pub st_mtime_nsec: libc::c_ulong,
    pub st_ctime: libc::c_ulong,
    pub st_ctime_nsec: libc::c_ulong,
    pub st_ino: u64,
}

unsafe fn cp_oldabi_stat64(stat: *mut kstat, statbuf: *mut oldabi_stat64) -> libc::c_long {
    let mut tmp: oldabi_stat64 = core::mem::zeroed();
    tmp.st_dev = huge_encode_dev((*stat).dev);
    tmp.__pad1 = 0;
    tmp.__st_ino = (*stat).ino;
    tmp.st_mode = (*stat).mode;
    tmp.st_nlink = (*stat).nlink;
    tmp.st_uid = from_kuid_munged(current_user_ns(), (*stat).uid);
    tmp.st_gid = from_kgid_munged(current_user_ns(), (*stat).gid);
    tmp.st_rdev = huge_encode_dev((*stat).rdev);
    tmp.st_size = (*stat).size;
    tmp.st_blocks = (*stat).blocks;
    tmp.__pad2 = 0;
    tmp.st_blksize = (*stat).blksize;
    tmp.st_atime = (*stat).atime.tv_sec;
    tmp.st_atime_nsec = (*stat).atime.tv_nsec;
    tmp.st_mtime = (*stat).mtime.tv_sec;
    tmp.st_mtime_nsec = (*stat).mtime.tv_nsec;
    tmp.st_ctime = (*stat).ctime.tv_sec;
    tmp.st_ctime_nsec = (*stat).ctime.tv_nsec;
    tmp.st_ino = (*stat).ino;
    if copy_to_user(statbuf as *mut libc::c_void, &tmp as *const _ as *const libc::c_void,
                    core::mem::size_of::<oldabi_stat64>()) != 0 { -EFAULT } else { 0 }
}

pub unsafe fn sys_oabi_stat64(filename: *const libc::c_char, statbuf: *mut oldabi_stat64) -> libc::c_long {
    let mut stat = core::mem::zeroed::<kstat>();
    let mut error = vfs_stat(filename, &mut stat);
    if error == 0 { error = cp_oldabi_stat64(&mut stat, statbuf); }
    error
}

pub unsafe fn sys_oabi_lstat64(filename: *const libc::c_char, statbuf: *mut oldabi_stat64) -> libc::c_long {
    let mut stat = core::mem::zeroed::<kstat>();
    let mut error = vfs_lstat(filename, &mut stat);
    if error == 0 { error = cp_oldabi_stat64(&mut stat, statbuf); }
    error
}

pub unsafe fn sys_oabi_fstat64(fd: libc::c_ulong, statbuf: *mut oldabi_stat64) -> libc::c_long {
    let mut stat = core::mem::zeroed::<kstat>();
    let mut error = vfs_fstat(fd, &mut stat);
    if error == 0 { error = cp_oldabi_stat64(&mut stat, statbuf); }
    error
}

pub unsafe fn sys_oabi_fstatat64(dfd: libc::c_int, filename: *const libc::c_char,
                                 statbuf: *mut oldabi_stat64, flag: libc::c_int) -> libc::c_long {
    let mut stat = core::mem::zeroed::<kstat>();
    let error = vfs_fstatat(dfd, filename, &mut stat, flag);
    if error != 0 { return error; }
    cp_oldabi_stat64(&mut stat, statbuf)
}

#[repr(C, packed(4))]
pub struct oabi_flock64 { pub l_type: i16, pub l_whence: i16, pub l_start: loff_t, pub l_len: loff_t, pub l_pid: pid_t }

unsafe fn get_oabi_flock(kernel: *mut flock64, arg: *mut oabi_flock64) -> libc::c_int {
    let mut user = core::mem::zeroed::<oabi_flock64>();
    if copy_from_user(&mut user as *mut _ as *mut libc::c_void, arg as *const libc::c_void,
                      core::mem::size_of::<oabi_flock64>()) != 0 { return -EFAULT; }
    (*kernel).l_type = user.l_type; (*kernel).l_whence = user.l_whence;
    (*kernel).l_start = user.l_start; (*kernel).l_len = user.l_len; (*kernel).l_pid = user.l_pid; 0
}

unsafe fn put_oabi_flock(kernel: *mut flock64, arg: *mut oabi_flock64) -> libc::c_int {
    let user = oabi_flock64 { l_type: (*kernel).l_type, l_whence: (*kernel).l_whence,
        l_start: (*kernel).l_start, l_len: (*kernel).l_len, l_pid: (*kernel).l_pid };
    if copy_to_user(arg as *mut libc::c_void, &user as *const _ as *const libc::c_void,
                    core::mem::size_of::<oabi_flock64>()) != 0 { -EFAULT } else { 0 }
}

pub unsafe fn sys_oabi_fcntl64(fd: libc::c_uint, cmd: libc::c_uint, arg: libc::c_ulong) -> libc::c_long {
    let argp = arg as *mut libc::c_void;
    let f = fd_raw(fd);
    if fd_empty(f) { return -EBADF; }
    let mut flock = core::mem::zeroed::<flock64>();
    match cmd {
        F_GETLK64 | F_OFD_GETLK => {
            let mut err = security_file_fcntl(fd_file(f), cmd, arg);
            if err == 0 { err = get_oabi_flock(&mut flock, argp as *mut _); }
            if err == 0 { err = fcntl_getlk64(fd_file(f), cmd, &mut flock); }
            if err == 0 { err = put_oabi_flock(&mut flock, argp as *mut _); } err
        },
        F_SETLK64 | F_SETLKW64 | F_OFD_SETLK | F_OFD_SETLKW => {
            let mut err = security_file_fcntl(fd_file(f), cmd, arg);
            if err == 0 { err = get_oabi_flock(&mut flock, argp as *mut _); }
            if err == 0 { err = fcntl_setlk64(fd, fd_file(f), cmd, &mut flock); } err
        },
        _ => sys_fcntl64(fd, cmd, arg),
    }
}

#[repr(C, packed(4))]
pub struct oabi_epoll_event { pub events: __poll_t, pub data: u64 }

#[cfg(CONFIG_EPOLL)]
pub unsafe fn sys_oabi_epoll_ctl(epfd: libc::c_int, op: libc::c_int, fd: libc::c_int, event: *mut oabi_epoll_event) -> libc::c_long {
    let mut user = core::mem::zeroed::<oabi_epoll_event>();
    if ep_op_has_event(op) && copy_from_user(&mut user as *mut _ as *mut _, event as *const _, core::mem::size_of::<oabi_epoll_event>()) != 0 { return -EFAULT; }
    let kernel = epoll_event { events: user.events, data: user.data };
    do_epoll_ctl(epfd, op, fd, &kernel, false)
}
#[cfg(not(CONFIG_EPOLL))]
pub unsafe fn sys_oabi_epoll_ctl(_: libc::c_int, _: libc::c_int, _: libc::c_int, _: *mut oabi_epoll_event) -> libc::c_long { -EINVAL }

pub unsafe fn epoll_put_uevent(revents: __poll_t, data: u64, uevent: *mut epoll_event) -> *mut epoll_event {
    if in_oabi_syscall() {
        let oevent = uevent as *mut oabi_epoll_event;
        if __put_user(revents, &mut (*oevent).events) != 0 || __put_user(data, &mut (*oevent).data) != 0 { return core::ptr::null_mut(); }
        return oevent.add(1) as *mut epoll_event;
    }
    if __put_user(revents, &mut (*uevent).events) != 0 || __put_user(data, &mut (*uevent).data) != 0 { core::ptr::null_mut() } else { uevent.add(1) }
}

#[repr(C)] pub struct oabi_sembuf { pub sem_num: u16, pub sem_op: i16, pub sem_flg: i16, pub __pad: u16 }

#[cfg(CONFIG_SYSVIPC)]
pub unsafe fn sys_oabi_semtimedop(semid: libc::c_int, tsops: *mut oabi_sembuf, nsops: libc::c_uint, timeout: *const old_timespec32) -> libc::c_long {
    let ns = (*current()).nsproxy.ipc_ns;
    if nsops > (*ns).sc_semopm() { return -E2BIG; }
    if nsops < 1 || nsops > SEMOPM { return -EINVAL; }
    let sops = kvmalloc_objs::<sembuf>(nsops);
    if sops.is_null() { return -ENOMEM; }
    let mut err = 0;
    for i in 0..nsops as usize { let mut osb = core::mem::zeroed::<oabi_sembuf>(); err |= copy_from_user(&mut osb as *mut _ as *mut _, tsops.add(i) as *const _, core::mem::size_of::<oabi_sembuf>()) as libc::c_long; (*sops.add(i)).sem_num = osb.sem_num; (*sops.add(i)).sem_op = osb.sem_op; (*sops.add(i)).sem_flg = osb.sem_flg; }
    if err != 0 { kvfree(sops as *mut _); return -EFAULT; }
    if !timeout.is_null() { let mut ts = core::mem::zeroed::<timespec64>(); err = get_old_timespec32(&mut ts, timeout); if err == 0 { err = __do_semtimedop(semid, sops, nsops, &ts, ns); } } else { err = __do_semtimedop(semid, sops, nsops, core::ptr::null(), ns); }
    kvfree(sops as *mut _); err
}
#[cfg(not(CONFIG_SYSVIPC))]
pub unsafe fn sys_oabi_semtimedop(_: libc::c_int, _: *mut oabi_sembuf, _: libc::c_uint, _: *const old_timespec32) -> libc::c_long { -ENOSYS }
pub unsafe fn sys_oabi_semop(semid: libc::c_int, tsops: *mut oabi_sembuf, nsops: libc::c_uint) -> libc::c_long { sys_oabi_semtimedop(semid, tsops, nsops, core::ptr::null()) }

#[cfg(CONFIG_SYSVIPC)]
pub unsafe fn sys_oabi_ipc(call: libc::c_uint, first: libc::c_int, second: libc::c_int, third: libc::c_int, ptr: *mut libc::c_void, fifth: libc::c_long) -> libc::c_int {
    match call & 0xffff {
        SEMOP => sys_oabi_semtimedop(first, ptr as *mut oabi_sembuf, second as _, core::ptr::null()) as _,
        SEMTIMEDOP => sys_oabi_semtimedop(first, ptr as *mut oabi_sembuf, second as _, fifth as *const old_timespec32) as _,
        _ => sys_ipc(call, first, second, third, ptr, fifth),
    }
}
#[cfg(not(CONFIG_SYSVIPC))]
pub unsafe fn sys_oabi_ipc(_: libc::c_uint, _: libc::c_int, _: libc::c_int, _: libc::c_int, _: *mut libc::c_void, _: libc::c_long) -> libc::c_int { -ENOSYS }

pub unsafe fn sys_oabi_bind(fd: libc::c_int, addr: *mut sockaddr, mut addrlen: libc::c_int) -> libc::c_long { let mut family = 0; if addrlen == 112 && get_user(&mut family, &(*addr).sa_family) == 0 && family == AF_UNIX { addrlen = 110; } sys_bind(fd, addr, addrlen) }
pub unsafe fn sys_oabi_connect(fd: libc::c_int, addr: *mut sockaddr, mut addrlen: libc::c_int) -> libc::c_long { let mut family = 0; if addrlen == 112 && get_user(&mut family, &(*addr).sa_family) == 0 && family == AF_UNIX { addrlen = 110; } sys_connect(fd, addr, addrlen) }
pub unsafe fn sys_oabi_sendto(fd: libc::c_int, buff: *mut libc::c_void, len: usize, flags: libc::c_uint, addr: *mut sockaddr, mut addrlen: libc::c_int) -> libc::c_long { let mut family = 0; if addrlen == 112 && get_user(&mut family, &(*addr).sa_family) == 0 && family == AF_UNIX { addrlen = 110; } sys_sendto(fd, buff, len, flags, addr, addrlen) }

pub unsafe fn sys_oabi_sendmsg(fd: libc::c_int, msg: *mut user_msghdr, flags: libc::c_uint) -> libc::c_long { let mut addr: *mut sockaddr = core::ptr::null_mut(); let mut namelen = 0; let mut family = 0; if !msg.is_null() && get_user(&mut namelen, &(*msg).msg_namelen) == 0 && namelen == 112 && get_user(&mut addr, &(*msg).msg_name) == 0 && get_user(&mut family, &(*addr).sa_family) == 0 && family == AF_UNIX { put_user(110, &mut (*msg).msg_namelen); } sys_sendmsg(fd, msg, flags) }

pub unsafe fn sys_oabi_socketcall(call: libc::c_int, args: *mut libc::c_ulong) -> libc::c_ulong { let mut r = -EFAULT as libc::c_ulong; let mut a = [0; 6]; match call { SYS_BIND => if copy_from_user(a.as_mut_ptr() as *mut _, args as *const _, 3 * core::mem::size_of::<libc::c_ulong>()) == 0 { r = sys_oabi_bind(a[0] as _, a[1] as *mut _, a[2] as _) as _ }, SYS_CONNECT => if copy_from_user(a.as_mut_ptr() as *mut _, args as *const _, 3 * core::mem::size_of::<libc::c_ulong>()) == 0 { r = sys_oabi_connect(a[0] as _, a[1] as *mut _, a[2] as _) as _ }, SYS_SENDTO => if copy_from_user(a.as_mut_ptr() as *mut _, args as *const _, 6 * core::mem::size_of::<libc::c_ulong>()) == 0 { r = sys_oabi_sendto(a[0] as _, a[1] as *mut _, a[2] as _, a[3] as _, a[4] as *mut _, a[5] as _) as _ }, SYS_SENDMSG => if copy_from_user(a.as_mut_ptr() as *mut _, args as *const _, 3 * core::mem::size_of::<libc::c_ulong>()) == 0 { r = sys_oabi_sendmsg(a[0] as _, a[1] as *mut _, a[2] as _) as _ }, _ => r = sys_socketcall(call, args), } r }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
