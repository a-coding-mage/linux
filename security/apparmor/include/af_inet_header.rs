// SPDX-License-Identifier: GPL-2.0-only
/*
 * AppArmor security module
 *
 * This file contains AppArmor inet/inet6 fine grained mediation
 *
 * Copyright 2024 Canonical Ltd.
 *
 * This program is free software; you can redistribute it and/or
 * modify it under the terms of the GNU General Public License as
 * published by the Free Software Foundation, version 2 of the
 * License.
 */

// Dependency: label.h

use std::ffi::{c_int, c_char};

// Opaque types from external headers
#[repr(C)]
pub struct socket;

#[repr(C)]
pub struct sockaddr;

#[repr(C)]
pub struct msghdr;

#[repr(C)]
pub struct aa_label;

#[repr(C)]
pub struct cred;

extern "C" {
    pub fn aa_inet_sock_perm(op: *const c_char, request: u32, sock: *mut socket) -> c_int;

    pub fn aa_inet_create_perm(
        label: *mut aa_label,
        family: c_int,
        type_: c_int,
        protocol: c_int,
    ) -> c_int;

    pub fn aa_inet_bind_perm(
        sock: *mut socket,
        address: *mut sockaddr,
        addrlen: c_int,
    ) -> c_int;

    pub fn aa_inet_connect_perm(
        sock: *mut socket,
        address: *mut sockaddr,
        addrlen: c_int,
    ) -> c_int;

    pub fn aa_inet_listen_perm(sock: *mut socket, backlog: c_int) -> c_int;

    pub fn aa_inet_accept_perm(sock: *mut socket, newsock: *mut socket) -> c_int;

    pub fn aa_inet_msg_perm(
        op: *const c_char,
        request: u32,
        sock: *mut socket,
        msg: *mut msghdr,
        size: c_int,
    ) -> c_int;

    pub fn aa_inet_opt_perm(
        op: *const c_char,
        request: u32,
        sock: *mut socket,
        level: c_int,
        optname: c_int,
    ) -> c_int;

    pub fn aa_inet_file_perm(
        subj_cred: *const cred,
        label: *mut aa_label,
        op: *const c_char,
        request: u32,
        sock: *mut socket,
    ) -> c_int;
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
