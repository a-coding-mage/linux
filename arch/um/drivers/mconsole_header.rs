/* SPDX-License-Identifier: GPL-2.0 */
/*
 * Copyright (C) 2001 Lennert Buytenhek (buytenh@gnu.org)
 * Copyright (C) 2001 - 2007 Jeff Dike (jdike@{addtoit,linux.intel}.com)
 */

use core::ffi::{c_char, c_void};

// C dependency: <sysdep/ptrace.h> supplies `uml_pt_regs`.

pub const MCONSOLE_MAGIC: u32 = 0xcafebabe;
pub const MCONSOLE_MAX_DATA: usize = 512;
pub const MCONSOLE_VERSION: u32 = 2;

#[repr(C)]
pub struct mconsole_request {
    pub magic: u32,
    pub version: u32,
    pub len: u32,
    pub data: [c_char; MCONSOLE_MAX_DATA],
}

#[repr(C)]
pub struct mconsole_reply {
    pub err: u32,
    pub more: u32,
    pub len: u32,
    pub data: [c_char; MCONSOLE_MAX_DATA],
}

pub const MCONSOLE_SOCKET: u32 = 0;
pub const MCONSOLE_PANIC: u32 = 1;
pub const MCONSOLE_HANG: u32 = 2;
pub const MCONSOLE_USER_NOTIFY: u32 = 3;

#[repr(C)]
pub struct mconsole_notify {
    pub magic: u32,
    pub version: u32,
    pub type_: u32,
    pub len: u32,
    pub data: [c_char; MCONSOLE_MAX_DATA],
}

pub struct mc_request;

#[repr(C)]
#[derive(Copy, Clone)]
pub enum mc_context {
    MCONSOLE_INTR = 0,
    MCONSOLE_PROC = 1,
}

#[repr(C)]
pub struct mconsole_command {
    pub command: *mut c_char,
    pub handler: Option<unsafe extern "C" fn(req: *mut mc_request)>,
    pub context: mc_context,
}

#[repr(C)]
pub struct mc_request {
    pub len: libc::c_int,
    pub as_interrupt: libc::c_int,

    pub originating_fd: libc::c_int,
    pub originlen: libc::c_uint,
    pub origin: [u8; 128], /* sockaddr_un */

    pub request: mconsole_request,
    pub cmd: *mut mconsole_command,
    pub regs: crate::uml_pt_regs,
}

extern "C" {
    pub static mut mconsole_socket_name: [c_char; 0];

    pub fn mconsole_unlink_socket() -> libc::c_int;
    pub fn mconsole_reply_len(
        req: *mut mc_request,
        reply: *const c_char,
        len: libc::c_int,
        err: libc::c_int,
        more: libc::c_int,
    ) -> libc::c_int;
    pub fn mconsole_reply(
        req: *mut mc_request,
        str_: *const c_char,
        err: libc::c_int,
        more: libc::c_int,
    ) -> libc::c_int;

    pub fn mconsole_version(req: *mut mc_request);
    pub fn mconsole_help(req: *mut mc_request);
    pub fn mconsole_halt(req: *mut mc_request);
    pub fn mconsole_reboot(req: *mut mc_request);
    pub fn mconsole_config(req: *mut mc_request);
    pub fn mconsole_remove(req: *mut mc_request);
    pub fn mconsole_sysrq(req: *mut mc_request);
    pub fn mconsole_cad(req: *mut mc_request);
    pub fn mconsole_stop(req: *mut mc_request);
    pub fn mconsole_go(req: *mut mc_request);
    pub fn mconsole_log(req: *mut mc_request);
    pub fn mconsole_proc(req: *mut mc_request);
    pub fn mconsole_stack(req: *mut mc_request);

    pub fn mconsole_get_request(fd: libc::c_int, req: *mut mc_request) -> libc::c_int;
    pub fn mconsole_notify(
        sock_name: *mut c_char,
        type_: libc::c_int,
        data: *const c_void,
        len: libc::c_int,
    ) -> libc::c_int;
    pub fn mconsole_notify_socket() -> *mut c_char;
    pub fn lock_notify();
    pub fn unlock_notify();
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
