// SPDX-License-Identifier: GPL-2.0-or-later
/* scm.c - Socket level control messages processing. */

// Kernel declarations supplied by the surrounding translation unit.
use core::ptr;

#[inline]
unsafe fn scm_check_creds(creds: *mut ucred) -> i32 {
    let cred = current_cred();
    let uid = make_kuid((*cred).user_ns, (*creds).uid);
    let gid = make_kgid((*cred).user_ns, (*creds).gid);
    if !uid_valid(uid) || !gid_valid(gid) { return -EINVAL; }
    if (((*creds).pid == task_tgid_vnr(current())) ||
        ns_capable(task_active_pid_ns(current()).user_ns, CAP_SYS_ADMIN)) &&
       ((uid_eq(uid, (*cred).uid) || uid_eq(uid, (*cred).euid) ||
         uid_eq(uid, (*cred).suid)) || ns_capable((*cred).user_ns, CAP_SETUID)) &&
       ((gid_eq(gid, (*cred).gid) || gid_eq(gid, (*cred).egid) ||
         gid_eq(gid, (*cred).sgid)) || ns_capable((*cred).user_ns, CAP_SETGID)) { 0 }
    else { -EPERM }
}

unsafe fn scm_fp_copy(cmsg: *mut cmsghdr, fplp: *mut *mut scm_fp_list) -> i32 {
    let fdp = cmsg_data(cmsg) as *mut i32;
    let mut fpl = *fplp;
    let num = ((*cmsg).cmsg_len - core::mem::size_of::<cmsghdr>()) /
        core::mem::size_of::<i32>();
    if num <= 0 { return 0; }
    if num > SCM_MAX_FD { return -EINVAL; }
    if fpl.is_null() {
        fpl = kmalloc_obj::<scm_fp_list>(GFP_KERNEL_ACCOUNT);
        if fpl.is_null() { return -ENOMEM; }
        *fplp = fpl;
        (*fpl).count = 0; (*fpl).count_unix = 0; (*fpl).max = SCM_MAX_FD;
        (*fpl).user = ptr::null_mut();
        #[cfg(CONFIG_UNIX)] { (*fpl).inflight = false; (*fpl).dead = false;
            (*fpl).edges = ptr::null_mut(); INIT_LIST_HEAD(&mut (*fpl).vertices); }
    }
    if (*fpl).count + num > (*fpl).max { return -EINVAL; }
    let mut fpp = (*fpl).fp.as_mut_ptr().add((*fpl).count as usize);
    for i in 0..num {
        let fd = *fdp.add(i as usize);
        let file = fget_raw(fd);
        if fd < 0 || file.is_null() { return -EBADF; }
        if io_is_uring_fops(file) { fput(file); return -EINVAL; }
        if !unix_get_socket(file).is_null() { (*fpl).count_unix += 1; }
        *fpp = file; fpp = fpp.add(1); (*fpl).count += 1;
    }
    if (*fpl).user.is_null() { (*fpl).user = get_uid(current_user()); }
    num as i32
}

pub unsafe fn __scm_destroy(scm: *mut scm_cookie) {
    let fpl = (*scm).fp;
    if !fpl.is_null() {
        (*scm).fp = ptr::null_mut();
        for i in (0..(*fpl).count).rev() { fput(*fpl).fp[i as usize]; }
        free_uid((*fpl).user); kfree(fpl);
    }
}

unsafe fn scm_replace_pid(scm: *mut scm_cookie, pid: *mut pid) -> i32 {
    scm_destroy_cred(scm);
    let err = pidfs_register_pid(pid); if err != 0 { return err; }
    (*scm).pid = pid; (*scm).creds.pid = pid_vnr(pid); 0
}

pub unsafe fn __scm_send(sock: *mut socket, msg: *mut msghdr, p: *mut scm_cookie) -> i32 {
    let ops = read_once((*sock).ops);
    let mut cmsg = ptr::null_mut(); let mut err;
    for_each_cmsghdr!(cmsg, msg) {
        err = -EINVAL;
        if !cmsg_ok(msg, cmsg) { scm_destroy(p); return err; }
        if (*cmsg).cmsg_level != SOL_SOCKET { continue; }
        match (*cmsg).cmsg_type {
            SCM_RIGHTS => { if ops.is_null() || (*ops).family != PF_UNIX { scm_destroy(p); return err; }
                err = scm_fp_copy(cmsg, &mut (*p).fp); if err < 0 { scm_destroy(p); return err; } }
            SCM_CREDENTIALS => {
                if (*cmsg).cmsg_len != cmsg_len(core::mem::size_of::<ucred>()) { scm_destroy(p); return err; }
                let mut creds: ucred = core::mem::zeroed(); ptr::copy_nonoverlapping(cmsg_data(cmsg), &mut creds as *mut _ as *mut _, core::mem::size_of::<ucred>());
                err = scm_check_creds(&mut creds); if err != 0 { scm_destroy(p); return err; }
                if (*p).pid.is_null() || pid_vnr((*p).pid) != creds.pid {
                    let pid = find_get_pid(creds.pid); if pid.is_null() { scm_destroy(p); return -ESRCH; }
                    err = scm_replace_pid(p, pid); if err != 0 { put_pid(pid); scm_destroy(p); return err; }
                }
                let uid = make_kuid(current_user_ns(), creds.uid); let gid = make_kgid(current_user_ns(), creds.gid);
                if !uid_valid(uid) || !gid_valid(gid) { scm_destroy(p); return -EINVAL; }
                (*p).creds.uid = uid; (*p).creds.gid = gid;
            }
            _ => { scm_destroy(p); return err; }
        }
    }
    if !(*p).fp.is_null() && (*(*p).fp).count == 0 { kfree((*p).fp); (*p).fp = ptr::null_mut(); }
    0
}

pub unsafe fn put_cmsg(msg: *mut msghdr, level: i32, typ: i32, len: i32, data: *mut core::ffi::c_void) -> i32 {
    let mut cmlen = cmsg_len(len as usize) as usize;
    if (*msg).msg_flags & MSG_CMSG_COMPAT != 0 { return put_cmsg_compat(msg, level, typ, len, data); }
    if (*msg).msg_control.is_null() || (*msg).msg_controllen < core::mem::size_of::<cmsghdr>() { (*msg).msg_flags |= MSG_CTRUNC; return 0; }
    if (*msg).msg_controllen < cmlen { (*msg).msg_flags |= MSG_CTRUNC; cmlen = (*msg).msg_controllen; }
    let cm = (*msg).msg_control;
    (*cm).cmsg_level = level; (*cm).cmsg_type = typ; (*cm).cmsg_len = cmlen;
    ptr::copy_nonoverlapping(data as *const u8, cmsg_data(cm), cmlen - core::mem::size_of::<cmsghdr>());
    let used = core::cmp::min(cmsg_space(len as usize), (*msg).msg_controllen);
    if (*msg).msg_control_is_user { (*msg).msg_control_user = (*msg).msg_control_user.add(used); }
    else { (*msg).msg_control = (*msg).msg_control.add(used); }
    (*msg).msg_controllen -= used; 0
}

pub unsafe fn put_cmsg_notrunc(msg: *mut msghdr, level: i32, typ: i32, len: i32, data: *mut core::ffi::c_void) -> i32 {
    if (*msg).msg_control.is_null() || (*msg).msg_controllen < cmsg_len(len as usize) { return -ETOOSMALL; }
    put_cmsg(msg, level, typ, len, data)
}

pub unsafe fn put_cmsg_scm_timestamping64(msg: *mut msghdr, t: *mut scm_timestamping_internal) {
    let mut out: scm_timestamping64 = core::mem::zeroed();
    for i in 0..out.ts.len() { let tv = ktime_to_timespec64((*t).ts[i]); out.ts[i].tv_sec = tv.tv_sec; out.ts[i].tv_nsec = tv.tv_nsec; }
    put_cmsg(msg, SOL_SOCKET, SO_TIMESTAMPING_NEW, core::mem::size_of_val(&out) as i32, &mut out as *mut _ as *mut _);
}

pub unsafe fn put_cmsg_scm_timestamping(msg: *mut msghdr, t: *mut scm_timestamping_internal) {
    let mut out: scm_timestamping = core::mem::zeroed();
    for i in 0..out.ts.len() { let tv = ktime_to_timespec64((*t).ts[i]); out.ts[i].tv_sec = tv.tv_sec; out.ts[i].tv_nsec = tv.tv_nsec; }
    put_cmsg(msg, SOL_SOCKET, SO_TIMESTAMPING_OLD, core::mem::size_of_val(&out) as i32, &mut out as *mut _ as *mut _);
}

unsafe fn scm_max_fds(msg: *mut msghdr) -> i32 { if (*msg).msg_controllen <= core::mem::size_of::<cmsghdr>() { 0 } else { (( (*msg).msg_controllen - core::mem::size_of::<cmsghdr>()) / core::mem::size_of::<i32>()) as i32 } }

pub unsafe fn scm_recv_one_fd(f: *mut file, ufd: *mut i32, flags: u32, notrunc: bool) -> i32 {
    if ufd.is_null() { return -EFAULT; }
    let error = security_file_receive(f); if error != 0 { return if notrunc { put_user(error, ufd) } else { error }; }
    let fdf = fd_prepare(f, flags, get_file(f)); if fdf.err != 0 { return fdf.err; }
    let error = put_user(fd_prepare_fd(fdf), ufd); if error != 0 { return error; }
    receive_sock(fd_prepare_file(fdf)); fd_publish(fdf)
}

pub unsafe fn scm_detach_fds(msg: *mut msghdr, scm: *mut scm_cookie, notrunc: bool) {
    if !(*msg).msg_control_is_user { return; }
    if (*msg).msg_flags & MSG_CMSG_COMPAT != 0 { scm_detach_fds_compat(msg, scm, notrunc); return; }
    let fdmax = core::cmp::min(scm_max_fds(msg), (*(*scm).fp).count);
    let data = cmsg_user_data((*msg).msg_control_user) as *mut i32; let mut i = 0; let mut err = 0;
    while i < fdmax { err = scm_recv_one_fd((*(*scm).fp).fp[i as usize], data.add(i as usize), if (*msg).msg_flags & MSG_CMSG_CLOEXEC != 0 { O_CLOEXEC } else { 0 }, notrunc); if err < 0 { break; } i += 1; }
    if i > 0 { let cmlen = cmsg_len((i as usize) * core::mem::size_of::<i32>()); put_user(SOL_SOCKET, &mut (*(*msg).msg_control_user).cmsg_level); put_user(SCM_RIGHTS, &mut (*(*msg).msg_control_user).cmsg_type); put_user(cmlen, &mut (*(*msg).msg_control_user).cmsg_len); let used = core::cmp::min(cmsg_space((i as usize)*core::mem::size_of::<i32>()), (*msg).msg_controllen); (*msg).msg_control_user = (*msg).msg_control_user.add(used); (*msg).msg_controllen -= used; }
    if i < (*(*scm).fp).count || ((*(*scm).fp).count != 0 && fdmax <= 0) { (*msg).msg_flags |= MSG_CTRUNC; }
    __scm_destroy(scm);
}

pub unsafe fn scm_fp_dup(fpl: *mut scm_fp_list) -> *mut scm_fp_list {
    if fpl.is_null() { return ptr::null_mut(); }
    let n = kmemdup(fpl, core::mem::offset_of!(scm_fp_list, fp) + (*fpl).count as usize * core::mem::size_of::<*mut file>(), GFP_KERNEL_ACCOUNT);
    if !n.is_null() { for i in 0..(*fpl).count { get_file((*fpl).fp[i as usize]); } (*n).max = (*n).count; (*n).user = get_uid((*fpl).user); #[cfg(CONFIG_UNIX)] { (*n).inflight=false; (*n).edges=ptr::null_mut(); INIT_LIST_HEAD(&mut (*n).vertices); } } n
}

#[cfg(CONFIG_SECURITY_NETWORK)]
unsafe fn scm_passec(sk: *mut sock, msg: *mut msghdr, scm: *mut scm_cookie) {
    if (*sk).sk_scm_security {
        let mut ctx: lsm_context = core::mem::zeroed();
        let err = security_secid_to_secctx((*scm).secid, &mut ctx);
        if err >= 0 { put_cmsg(msg, SOL_SOCKET, SCM_SECURITY, ctx.len, ctx.context); security_release_secctx(&mut ctx); }
    }
}
#[cfg(CONFIG_SECURITY_NETWORK)]
unsafe fn scm_has_secdata(sk: *mut sock) -> bool { (*sk).sk_scm_security }

#[cfg(not(CONFIG_SECURITY_NETWORK))]
unsafe fn scm_passec(_sk: *mut sock, _msg: *mut msghdr, _scm: *mut scm_cookie) {}
#[cfg(not(CONFIG_SECURITY_NETWORK))]
unsafe fn scm_has_secdata(_sk: *mut sock) -> bool { false }

unsafe fn scm_pidfd_recv(msg: *mut msghdr, scm: *mut scm_cookie) {
    let mut pidfd_file = ptr::null_mut();
    let len = if (*msg).msg_flags & MSG_CMSG_COMPAT != 0 { core::mem::size_of::<compat_cmsghdr>() + core::mem::size_of::<i32>() } else { core::mem::size_of::<cmsghdr>() + core::mem::size_of::<i32>() };
    if (*msg).msg_controllen < len { (*msg).msg_flags |= MSG_CTRUNC; return; }
    if (*scm).pid.is_null() { return; }
    let pidfd = pidfd_prepare((*scm).pid, PIDFD_STALE, &mut pidfd_file);
    if put_cmsg(msg, SOL_SOCKET, SCM_PIDFD, core::mem::size_of::<i32>() as i32, &pidfd as *const _ as *mut _) != 0 {
        if !pidfd_file.is_null() { put_unused_fd(pidfd); fput(pidfd_file); } return;
    }
    if !pidfd_file.is_null() { fd_install(pidfd, pidfd_file); }
}

unsafe fn __scm_recv_common(sk: *mut sock, msg: *mut msghdr, scm: *mut scm_cookie, _flags: i32) -> bool {
    if (*msg).msg_control.is_null() {
        if (*sk).sk_scm_credentials || (*sk).sk_scm_pidfd || !(*scm).fp.is_null() || scm_has_secdata(sk) { (*msg).msg_flags |= MSG_CTRUNC; }
        scm_destroy(scm); return false;
    }
    if (*sk).sk_scm_credentials {
        let ns = current_user_ns();
        let mut creds = ucred { pid: (*scm).creds.pid, uid: from_kuid_munged(ns, (*scm).creds.uid), gid: from_kgid_munged(ns, (*scm).creds.gid) };
        put_cmsg(msg, SOL_SOCKET, SCM_CREDENTIALS, core::mem::size_of::<ucred>() as i32, &mut creds as *mut _ as *mut _);
    }
    scm_passec(sk, msg, scm); true
}

pub unsafe fn scm_recv(sock: *mut socket, msg: *mut msghdr, scm: *mut scm_cookie, flags: i32) {
    if !__scm_recv_common((*sock).sk, msg, scm, flags) { return; }
    scm_destroy_cred(scm);
}

pub unsafe fn scm_recv_unix(sock: *mut socket, msg: *mut msghdr, scm: *mut scm_cookie, flags: i32) {
    if !__scm_recv_common((*sock).sk, msg, scm, flags) { return; }
    if !(*scm).fp.is_null() { let u = unix_sk((*sock).sk); scm_detach_fds(msg, scm, read_once((*u).scm_rights_notrunc)); }
    if (*sock).sk.sk_scm_pidfd { scm_pidfd_recv(msg, scm); }
    scm_destroy_cred(scm);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
