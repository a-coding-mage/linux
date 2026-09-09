// SPDX-License-Identifier: GPL-2.0-only
/*
 * 32bit Socket syscall emulation. Based on arch/sparc64/kernel/sys_sparc32.c.
 *
 * Copyright (C) 2000        VA Linux Co
 * Copyright (C) 2000        Don Dugger <n0ano@valinux.com>
 * Copyright (C) 1999        Arun Sharma <arun.sharma@intel.com>
 * Copyright (C) 1997,1998  Jakub Jelinek (jj@sunsite.mff.cuni.cz)
 * Copyright (C) 1997        David S. Miller (davem@caip.rutgers.edu)
 * Copyright (C) 2000        Hewlett-Packard Co.
 * Copyright (C) 2000        David Mosberger-Tang <davidm@hpl.hp.com>
 * Copyright (C) 2000,2001   Andi Kleen, SuSE Labs
 */

// Types, constants, macros, and functions supplied by the Linux kernel headers
// are intentionally referenced as external dependencies.

pub unsafe fn __get_compat_msghdr(
    kmsg: *mut msghdr,
    msg: *mut compat_msghdr,
    save_addr: *mut *mut sockaddr,
) -> i32 {
    let mut err: isize;
    (*kmsg).msg_flags = (*msg).msg_flags;
    (*kmsg).msg_namelen = (*msg).msg_namelen;
    if (*msg).msg_name.is_null() { (*kmsg).msg_namelen = 0; }
    if (*kmsg).msg_namelen < 0 { return -EINVAL; }
    if (*kmsg).msg_namelen > core::mem::size_of::<sockaddr_storage>() as i32 {
        (*kmsg).msg_namelen = core::mem::size_of::<sockaddr_storage>() as i32;
    }
    (*kmsg).msg_control_is_user = true;
    (*kmsg).msg_get_inq = 0;
    (*kmsg).msg_control_user = compat_ptr((*msg).msg_control);
    (*kmsg).msg_controllen = (*msg).msg_controllen;
    if !save_addr.is_null() { *save_addr = compat_ptr((*msg).msg_name); }
    if !(*msg).msg_name.is_null() && (*kmsg).msg_namelen != 0 {
        if save_addr.is_null() {
            err = move_addr_to_kernel(compat_ptr((*msg).msg_name), (*kmsg).msg_namelen, (*kmsg).msg_name);
            if err < 0 { return err as i32; }
        }
    } else {
        (*kmsg).msg_name = core::ptr::null_mut();
        (*kmsg).msg_namelen = 0;
    }
    if (*msg).msg_iovlen > UIO_MAXIOV { return -EMSGSIZE; }
    (*kmsg).msg_ubuf = core::ptr::null_mut();
    0
}

pub unsafe fn get_compat_msghdr(kmsg: *mut msghdr, umsg: *mut compat_msghdr, save_addr: *mut *mut sockaddr, iov: *mut *mut iovec) -> i32 {
    let mut msg: compat_msghdr = core::mem::zeroed();
    if copy_from_user(&mut msg as *mut _ as *mut _, umsg as *const _, core::mem::size_of::<compat_msghdr>()) != 0 { return -EFAULT; }
    let err = __get_compat_msghdr(kmsg, &mut msg, save_addr);
    if err != 0 { return err; }
    let err = import_iovec(if !save_addr.is_null() { ITER_DEST } else { ITER_SOURCE }, compat_ptr(msg.msg_iov), msg.msg_iovlen, UIO_FASTIOV, iov, &mut (*kmsg).msg_iter);
    if err < 0 { err as i32 } else { 0 }
}

#[inline]
unsafe fn cmsg_compat_nxthdr(msg: *mut msghdr, cmsg: *mut compat_cmsghdr, cmsg_len: i32) -> *mut compat_cmsghdr {
    let ptr = (cmsg as *mut u8).add(CMSG_COMPAT_ALIGN(cmsg_len as usize));
    if (ptr.add(1).offset_from((*msg).msg_control_user as *mut u8) as usize) > (*msg).msg_controllen { return core::ptr::null_mut(); }
    ptr as *mut compat_cmsghdr
}

#[inline] unsafe fn cmsg_compat_align(len: usize) -> usize { (len + core::mem::size_of::<i32>() - 1) & !(core::mem::size_of::<i32>() - 1) }
#[inline] unsafe fn cmsg_compat_space(len: usize) -> usize { core::mem::size_of::<compat_cmsghdr>() + cmsg_compat_align(len) }
#[inline] unsafe fn cmsg_compat_len(len: usize) -> usize { core::mem::size_of::<compat_cmsghdr>() + len }

unsafe fn cmsg_compat_firsthdr(msg: *mut msghdr) -> *mut compat_cmsghdr {
    if (*msg).msg_controllen >= core::mem::size_of::<compat_cmsghdr>() { (*msg).msg_control_user as *mut compat_cmsghdr } else { core::ptr::null_mut() }
}

pub unsafe fn cmsghdr_from_user_compat_to_kern(kmsg: *mut msghdr, sk: *mut sock, stackbuf: *mut u8, stackbuf_size: i32) -> i32 {
    let mut kcmlen: usize = 0;
    let mut ucmsg = cmsg_compat_firsthdr(kmsg);
    while !ucmsg.is_null() {
        let mut ucmlen: compat_size_t = 0;
        if get_user(&mut ucmlen, &(*ucmsg).cmsg_len) != 0 { return -EFAULT; }
        if ucmlen as usize < core::mem::size_of::<compat_cmsghdr>() || ucmlen as usize > ((*kmsg).msg_controllen - ((*ucmsg as *mut u8).offset_from((*kmsg).msg_control_user as *mut u8) as usize)) { return -EINVAL; }
        kcmlen += cmsg_align((ucmlen as usize - core::mem::size_of::<compat_cmsghdr>()) + core::mem::size_of::<cmsghdr>());
        ucmsg = cmsg_compat_nxthdr(kmsg, ucmsg, ucmlen as i32);
    }
    if kcmlen == 0 { return -EINVAL; }
    let mut kcmsg_base = stackbuf as *mut cmsghdr;
    if kcmlen > stackbuf_size as usize { kcmsg_base = sock_kmalloc(sk, kcmlen, GFP_KERNEL) as *mut cmsghdr; }
    if kcmsg_base.is_null() { return -ENOMEM; }
    core::ptr::write_bytes(kcmsg_base as *mut u8, 0, kcmlen);
    let mut kcmsg = kcmsg_base;
    ucmsg = cmsg_compat_firsthdr(kmsg);
    while !ucmsg.is_null() {
        let mut cmsg: compat_cmsghdr = core::mem::zeroed();
        if copy_from_user(&mut cmsg as *mut _ as *mut _, ucmsg as *const _, core::mem::size_of::<compat_cmsghdr>()) != 0 { return compat_cmsg_error(sk, stackbuf, kcmsg_base, kcmlen, -EFAULT); }
        if cmsg.cmsg_len as usize < core::mem::size_of::<compat_cmsghdr>() { return compat_cmsg_error(sk, stackbuf, kcmsg_base, kcmlen, -EINVAL); }
        let tmp = (cmsg.cmsg_len as usize - core::mem::size_of::<compat_cmsghdr>()) + core::mem::size_of::<cmsghdr>();
        if (kcmsg_base as *mut u8).add(kcmlen).offset_from(kcmsg as *mut u8) as usize < cmsg_align(tmp) { return compat_cmsg_error(sk, stackbuf, kcmsg_base, kcmlen, -EINVAL); }
        (*kcmsg).cmsg_len = tmp;
        (*kcmsg).cmsg_level = cmsg.cmsg_level;
        (*kcmsg).cmsg_type = cmsg.cmsg_type;
        let aligned = cmsg_align(tmp);
        if copy_from_user(cmsg_data(kcmsg), cmsg_compat_data(ucmsg), cmsg.cmsg_len as usize - core::mem::size_of::<compat_cmsghdr>()) != 0 { return compat_cmsg_error(sk, stackbuf, kcmsg_base, kcmlen, -EFAULT); }
        kcmsg = (kcmsg as *mut u8).add(aligned) as *mut cmsghdr;
        ucmsg = cmsg_compat_nxthdr(kmsg, ucmsg, cmsg.cmsg_len as i32);
    }
    if kcmsg.offset_from(kcmsg_base) as usize != kcmlen { return compat_cmsg_error(sk, stackbuf, kcmsg_base, kcmlen, -EINVAL); }
    (*kmsg).msg_control_is_user = false;
    (*kmsg).msg_control = kcmsg_base;
    (*kmsg).msg_controllen = kcmlen;
    0
}

unsafe fn compat_cmsg_error(sk: *mut sock, stackbuf: *mut u8, base: *mut cmsghdr, len: usize, err: i32) -> i32 { if base != stackbuf as *mut cmsghdr { sock_kfree_s(sk, base as *mut _, len); } err }

pub unsafe fn put_cmsg_compat(kmsg: *mut msghdr, level: i32, typ: i32, mut len: i32, mut data: *mut core::ffi::c_void) -> i32 {
    let cm = (*kmsg).msg_control_user as *mut compat_cmsghdr;
    if cm.is_null() || (*kmsg).msg_controllen < core::mem::size_of::<compat_cmsghdr>() { (*kmsg).msg_flags |= MSG_CTRUNC; return 0; }
    let mut cmhdr: compat_cmsghdr = core::mem::zeroed();
    let cmlen = cmsg_compat_len(len as usize).min((*kmsg).msg_controllen);
    if (*kmsg).msg_controllen < cmsg_compat_len(len as usize) { (*kmsg).msg_flags |= MSG_CTRUNC; }
    cmhdr.cmsg_level = level; cmhdr.cmsg_type = typ; cmhdr.cmsg_len = cmlen as _;
    if copy_to_user(cm, &cmhdr, core::mem::size_of::<compat_cmsghdr>()) != 0 { return -EFAULT; }
    if copy_to_user(cmsg_compat_data(cm), data, cmlen - core::mem::size_of::<compat_cmsghdr>()) != 0 { return -EFAULT; }
    let used = cmsg_compat_space(len as usize).min((*kmsg).msg_controllen);
    (*kmsg).msg_control_user = ((*kmsg).msg_control_user as *mut u8).add(used) as *mut _;
    (*kmsg).msg_controllen -= used;
    0
}

static NAS: [u8; 21] = [0, 12, 12, 12, 8, 12, 12, 12, 16, 16, 16, 24, 24, 8, 20, 20, 12, 12, 16, 20, 16];

unsafe fn __compat_sys_sendmsg(fd: i32, msg: *mut compat_msghdr, flags: u32) -> isize { __sys_sendmsg(fd, msg as *mut user_msghdr, flags | MSG_CMSG_COMPAT, false) }
unsafe fn __compat_sys_sendmmsg(fd: i32, mmsg: *mut compat_mmsghdr, vlen: u32, flags: u32) -> isize { __sys_sendmmsg(fd, mmsg as *mut mmsghdr, vlen, flags | MSG_CMSG_COMPAT, false) }
unsafe fn __compat_sys_recvmsg(fd: i32, msg: *mut compat_msghdr, flags: u32) -> isize { __sys_recvmsg(fd, msg as *mut user_msghdr, flags | MSG_CMSG_COMPAT, false) }
unsafe fn __compat_sys_recvfrom(fd: i32, buf: *mut core::ffi::c_void, len: compat_size_t, flags: u32, addr: *mut sockaddr, addrlen: *mut i32) -> isize { __sys_recvfrom(fd, buf, len, flags | MSG_CMSG_COMPAT, addr, addrlen) }

pub unsafe fn compat_sys_sendmsg(fd: i32, msg: *mut compat_msghdr, flags: u32) -> isize { __compat_sys_sendmsg(fd, msg, flags) }
pub unsafe fn compat_sys_sendmmsg(fd: i32, mmsg: *mut compat_mmsghdr, vlen: u32, flags: u32) -> isize { __compat_sys_sendmmsg(fd, mmsg, vlen, flags) }
pub unsafe fn compat_sys_recvmsg(fd: i32, msg: *mut compat_msghdr, flags: u32) -> isize { __compat_sys_recvmsg(fd, msg, flags) }
pub unsafe fn compat_sys_recv(fd: i32, buf: *mut core::ffi::c_void, len: compat_size_t, flags: u32) -> isize { __compat_sys_recvfrom(fd, buf, len, flags, core::ptr::null_mut(), core::ptr::null_mut()) }
pub unsafe fn compat_sys_recvfrom(fd: i32, buf: *mut core::ffi::c_void, len: compat_size_t, flags: u32, addr: *mut sockaddr, addrlen: *mut i32) -> isize { __compat_sys_recvfrom(fd, buf, len, flags, addr, addrlen) }
pub unsafe fn compat_sys_recvmmsg_time64(fd: i32, mmsg: *mut compat_mmsghdr, vlen: u32, flags: u32, timeout: *mut kernel_timespec) -> isize { __sys_recvmmsg(fd, mmsg as *mut mmsghdr, vlen, flags | MSG_CMSG_COMPAT, timeout, core::ptr::null_mut()) }

#[cfg(CONFIG_COMPAT_32BIT_TIME)]
pub unsafe fn compat_sys_recvmmsg_time32(fd: i32, mmsg: *mut compat_mmsghdr, vlen: u32, flags: u32, timeout: *mut old_timespec32) -> isize { __sys_recvmmsg(fd, mmsg as *mut mmsghdr, vlen, flags | MSG_CMSG_COMPAT, core::ptr::null_mut(), timeout) }

pub unsafe fn compat_sys_socketcall(call: i32, args: *mut u32) -> isize {
    if call < SYS_SOCKET || call > SYS_SENDMMSG { return -EINVAL as isize; }
    let len = NAS[call as usize] as usize;
    let mut a = [0u32; AUDITSC_ARGS];
    if len > core::mem::size_of_val(&a) { return -EINVAL as isize; }
    if copy_from_user(a.as_mut_ptr(), args, len) != 0 { return -EFAULT as isize; }
    let ret = audit_socketcall_compat(len / core::mem::size_of::<u32>(), a.as_mut_ptr());
    if ret != 0 { return ret as isize; }
    let a0 = a[0]; let a1 = a[1];
    match call {
        SYS_SOCKET => __sys_socket(a0, a1, a[2]), SYS_BIND => __sys_bind(a0, compat_ptr(a1), a[2]), SYS_CONNECT => __sys_connect(a0, compat_ptr(a1), a[2]), SYS_LISTEN => __sys_listen(a0, a1),
        SYS_ACCEPT => __sys_accept4(a0, compat_ptr(a1), compat_ptr(a[2]), 0), SYS_GETSOCKNAME => __sys_getsockname(a0, compat_ptr(a1), compat_ptr(a[2]), 0), SYS_GETPEERNAME => __sys_getsockname(a0, compat_ptr(a1), compat_ptr(a[2]), 1),
        SYS_SOCKETPAIR => __sys_socketpair(a0, a1, a[2], compat_ptr(a[3])), SYS_SEND => __sys_sendto(a0, compat_ptr(a1), a[2], a[3], core::ptr::null_mut(), 0), SYS_SENDTO => __sys_sendto(a0, compat_ptr(a1), a[2], a[3], compat_ptr(a[4]), a[5]),
        SYS_RECV => __compat_sys_recvfrom(a0, compat_ptr(a1), a[2], a[3], core::ptr::null_mut(), core::ptr::null_mut()), SYS_RECVFROM => __compat_sys_recvfrom(a0, compat_ptr(a1), a[2], a[3], compat_ptr(a[4]), compat_ptr(a[5])), SYS_SHUTDOWN => __sys_shutdown(a0, a1),
        SYS_SETSOCKOPT => __sys_setsockopt(a0, a1, a[2], compat_ptr(a[3]), a[4]), SYS_GETSOCKOPT => __sys_getsockopt(a0, a1, a[2], compat_ptr(a[3]), compat_ptr(a[4])), SYS_SENDMSG => __compat_sys_sendmsg(a0, compat_ptr(a1), a[2]),
        SYS_SENDMMSG => __compat_sys_sendmmsg(a0, compat_ptr(a1), a[2], a[3]), SYS_RECVMSG => __compat_sys_recvmsg(a0, compat_ptr(a1), a[2]), SYS_RECVMMSG => __sys_recvmmsg(a0, compat_ptr(a1), a[2], a[3] | MSG_CMSG_COMPAT, core::ptr::null_mut(), compat_ptr(a[4])),
        SYS_ACCEPT4 => __sys_accept4(a0, compat_ptr(a1), compat_ptr(a[2]), a[3]), _ => -EINVAL as isize,
    }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
