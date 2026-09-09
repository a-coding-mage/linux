// SPDX-License-Identifier: GPL-2.0
/*
 * Copyright (C) 2001 Lennert Buytenhek (buytenh@gnu.org)
 * Copyright (C) 2001 - 2007 Jeff Dike (jdike@{addtoit,linux.intel}.com)
 */

// C dependencies: errno, string, unistd, sys/socket, sys/uio, sys/un, mconsole.

use core::ffi::{c_char, c_int, c_void};

extern "C" {
    fn mconsole_version(req: *mut mc_request);
    fn mconsole_halt(req: *mut mc_request);
    fn mconsole_reboot(req: *mut mc_request);
    fn mconsole_config(req: *mut mc_request);
    fn mconsole_remove(req: *mut mc_request);
    fn mconsole_sysrq(req: *mut mc_request);
    fn mconsole_help(req: *mut mc_request);
    fn mconsole_cad(req: *mut mc_request);
    fn mconsole_stop(req: *mut mc_request);
    fn mconsole_go(req: *mut mc_request);
    fn mconsole_log(req: *mut mc_request);
    fn mconsole_proc(req: *mut mc_request);
    fn mconsole_stack(req: *mut mc_request);
    fn lock_notify();
    fn unlock_notify();
    fn printk(fmt: *const c_char, ...);
    fn recvfrom(fd: c_int, buf: *mut c_void, len: usize, flags: c_int,
                addr: *mut sockaddr, addrlen: *mut socklen_t) -> isize;
    fn sendmsg(fd: c_int, msg: *const msghdr, flags: c_int) -> isize;
    fn sendto(fd: c_int, buf: *const c_void, len: usize, flags: c_int,
              addr: *const sockaddr, addrlen: socklen_t) -> isize;
    fn socket(domain: c_int, ty: c_int, protocol: c_int) -> c_int;
    fn unlink(path: *const c_char) -> c_int;
}

extern "C" {
    static mut errno: c_int;
}

const MCONSOLE_PROC: c_int = 0;
const MCONSOLE_INTR: c_int = 1;

static mut commands: [mconsole_command; 13] = [
    mconsole_command { command: b"version\0".as_ptr() as *const c_char, handler: mconsole_version, context: MCONSOLE_PROC },
    mconsole_command { command: b"halt\0".as_ptr() as *const c_char, handler: mconsole_halt, context: MCONSOLE_PROC },
    mconsole_command { command: b"reboot\0".as_ptr() as *const c_char, handler: mconsole_reboot, context: MCONSOLE_PROC },
    mconsole_command { command: b"config\0".as_ptr() as *const c_char, handler: mconsole_config, context: MCONSOLE_PROC },
    mconsole_command { command: b"remove\0".as_ptr() as *const c_char, handler: mconsole_remove, context: MCONSOLE_PROC },
    mconsole_command { command: b"sysrq\0".as_ptr() as *const c_char, handler: mconsole_sysrq, context: MCONSOLE_INTR },
    mconsole_command { command: b"help\0".as_ptr() as *const c_char, handler: mconsole_help, context: MCONSOLE_INTR },
    mconsole_command { command: b"cad\0".as_ptr() as *const c_char, handler: mconsole_cad, context: MCONSOLE_INTR },
    mconsole_command { command: b"stop\0".as_ptr() as *const c_char, handler: mconsole_stop, context: MCONSOLE_PROC },
    mconsole_command { command: b"go\0".as_ptr() as *const c_char, handler: mconsole_go, context: MCONSOLE_INTR },
    mconsole_command { command: b"log\0".as_ptr() as *const c_char, handler: mconsole_log, context: MCONSOLE_INTR },
    mconsole_command { command: b"proc\0".as_ptr() as *const c_char, handler: mconsole_proc, context: MCONSOLE_PROC },
    mconsole_command { command: b"stack\0".as_ptr() as *const c_char, handler: mconsole_stack, context: MCONSOLE_INTR },
];

// Initialized in mconsole_init, which is an initcall.
#[no_mangle]
pub static mut mconsole_socket_name: [c_char; 256] = [0; 256];

unsafe fn mconsole_reply_v0(req: *mut mc_request, reply: *const c_char) -> c_int {
    let mut iov = iovec { iov_base: reply as *mut c_void, iov_len: strlen(reply) };
    let msg = msghdr { msg_name: &mut (*req).origin as *mut _ as *mut c_void,
        msg_namelen: (*req).originlen, msg_iov: &mut iov, msg_iovlen: 1,
        msg_control: core::ptr::null_mut(), msg_controllen: 0, msg_flags: 0 };
    sendmsg((*req).originating_fd, &msg, 0) as c_int
}

unsafe fn mconsole_parse(req: *mut mc_request) -> *mut mconsole_command {
    for i in 0..commands.len() {
        let cmd = &mut commands[i];
        if strncmp((*req).request.data.as_ptr(), cmd.command, strlen(cmd.command)) == 0 { return cmd; }
    }
    core::ptr::null_mut()
}

const fn min(a: usize, b: usize) -> usize { if a < b { a } else { b } }

pub unsafe fn mconsole_get_request(fd: c_int, req: *mut mc_request) -> c_int {
    (*req).originlen = core::mem::size_of_val(&(*req).origin) as socklen_t;
    (*req).len = recvfrom(fd, &mut (*req).request as *mut _ as *mut c_void,
        core::mem::size_of_val(&(*req).request), 0, (*req).origin.as_mut_ptr() as *mut sockaddr,
        &mut (*req).originlen);
    if (*req).len < 0 { return 0; }
    (*req).originating_fd = fd;
    if (*req).request.magic != MCONSOLE_MAGIC {
        let len = min((*req).request.data.len() - 1, strlen(&(*req).request as *const _ as *const c_char));
        memmove((*req).request.data.as_mut_ptr() as *mut c_void, &(*req).request as *const _ as *const c_void, len);
        (*req).request.data[len] = 0;
        (*req).request.magic = MCONSOLE_MAGIC; (*req).request.version = 0; (*req).request.len = len as _;
        mconsole_reply_v0(req, b"ERR Version 0 mconsole clients are not supported by this driver\0".as_ptr() as *const c_char);
        return 0;
    }
    if (*req).request.len >= MCONSOLE_MAX_DATA { mconsole_reply(req, b"Request too large\0".as_ptr() as *const c_char, 1, 0); return 0; }
    if (*req).request.version != MCONSOLE_VERSION { mconsole_reply(req, b"This driver only supports version clients\0".as_ptr() as *const c_char, 1, 0); }
    (*req).request.data[(*req).request.len as usize] = 0;
    (*req).cmd = mconsole_parse(req);
    if (*req).cmd.is_null() { mconsole_reply(req, b"Unknown command\0".as_ptr() as *const c_char, 1, 0); return 0; }
    1
}

pub unsafe fn mconsole_reply_len(req: *mut mc_request, mut str_: *const c_char, mut total: c_int, mut err: c_int, more: c_int) -> c_int {
    let mut reply: mconsole_reply = core::mem::zeroed();
    loop {
        reply.err = err; err = 0;
        let len = min(total as usize, MCONSOLE_MAX_DATA as usize - 1);
        reply.more = if len == total as usize { more } else { 1 };
        memcpy(reply.data.as_mut_ptr() as *mut c_void, str_ as *const c_void, len);
        reply.data[len] = 0; total -= len as c_int; str_ = str_.add(len); reply.len = (len + 1) as _;
        let n = sendto((*req).originating_fd, &reply as *const _ as *const c_void,
            core::mem::size_of_val(&reply) + reply.len as usize - reply.data.len(), 0,
            (*req).origin.as_ptr() as *const sockaddr, (*req).originlen);
        if n < 0 { return -errno_value(); } if total <= 0 { return 0; }
    }
}

pub unsafe fn mconsole_reply(req: *mut mc_request, str_: *const c_char, err: c_int, more: c_int) -> c_int {
    mconsole_reply_len(req, str_, strlen(str_) as c_int, err, more)
}

pub unsafe fn mconsole_unlink_socket() -> c_int { unlink(mconsole_socket_name.as_ptr()); 0 }

static mut notify_sock: c_int = -1;

pub unsafe fn mconsole_notify(sock_name: *mut c_char, ty: c_int, data: *const c_void, mut len: c_int) -> c_int {
    let mut target: sockaddr_un = core::mem::zeroed(); let mut packet: mconsole_notify = core::mem::zeroed(); let mut err = 0;
    lock_notify(); if notify_sock < 0 { notify_sock = socket(PF_UNIX, SOCK_DGRAM, 0); if notify_sock < 0 { err = -errno_value(); } } unlock_notify();
    if err != 0 { return err; }
    target.sun_family = AF_UNIX as _; strcpy(target.sun_path.as_mut_ptr(), sock_name);
    packet.magic = MCONSOLE_MAGIC; packet.version = MCONSOLE_VERSION; packet.type_ = ty;
    len = if len > packet.data.len() as c_int { packet.data.len() as c_int } else { len }; packet.len = len;
    memcpy(packet.data.as_mut_ptr() as *mut c_void, data, len as usize);
    let n = sendto(notify_sock, &packet as *const _ as *const c_void, core::mem::size_of_val(&packet) + len as usize - packet.data.len(), 0, &target as *const _ as *const sockaddr, core::mem::size_of_val(&target) as _);
    if n < 0 { -errno_value() } else { 0 }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
