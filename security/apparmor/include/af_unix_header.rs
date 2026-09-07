// SPDX-License-Identifier: GPL-2.0-only
//
// AppArmor security module
//
// This file contains AppArmor af_unix fine grained mediation
//
// Copyright 2023 Canonical Ltd.
//
// This program is free software; you can redistribute it and/or
// modify it under the terms of the GNU General Public License as
// published by the Free Software Foundation, version 2 of the
// License.

// External dependencies: <net/af_unix.h>, "label.h"

use core::ffi::c_int;
use core::ffi::c_uint;

// External types from linux kernel and AppArmor
// struct unix_sock
// struct sockaddr_un
// struct aa_label
// struct cred
// struct sock
// struct socket
// struct msghdr
// struct file
// type sa_family_t
// const SS_CONNECTED

// unix_addr(A) - cast to sockaddr_un pointer
#[inline]
fn unix_addr(a: *const u8) -> *const sockaddr_un {
    a as *const sockaddr_un
}

// unix_addr_len(L) - calculate length minus sa_family_t size
#[inline]
fn unix_addr_len(l: usize) -> isize {
    (l as isize) - (core::mem::size_of::<sa_family_t>() as isize)
}

// unix_peer(sk) - access peer field from unix_sk(sk)
// Note: unix_sk is external macro/function that extracts unix_sock from sock
#[inline]
fn unix_peer(sk: *const sock) -> *const sock {
    // TODO: unix_sk is an external macro/function to extract unix_sock from sock
    unsafe { (*unix_sk(sk)).peer }
}

// is_unix_addr_abstract_name(B) - check if first byte is 0
#[inline]
fn is_unix_addr_abstract_name(b: *const u8) -> bool {
    unsafe { *b == 0 }
}

// is_unix_addr_anon(A, L) - check if anonymous
#[inline]
fn is_unix_addr_anon(a: *const u8, l: usize) -> bool {
    !a.is_null() && unix_addr_len(l) <= 0
}

// is_unix_addr_fs(A, L) - check if filesystem address
#[inline]
fn is_unix_addr_fs(a: *const u8, l: usize) -> bool {
    !is_unix_addr_anon(a, l) && !is_unix_addr_abstract_name(unsafe { (*unix_addr(a)).sun_path.as_ptr() })
}

// is_unix_anonymous(U) - check if unix socket is anonymous
#[inline]
fn is_unix_anonymous(u: *const unix_sock) -> bool {
    unsafe { (*u).addr.is_null() }
}

// is_unix_fs(U) - check if unix socket is filesystem socket
#[inline]
fn is_unix_fs(u: *const unix_sock) -> bool {
    !is_unix_anonymous(u) && unsafe { (*(*u).addr).name.sun_path[0] } != 0
}

// is_unix_connected(S) - check if socket state is connected
#[inline]
fn is_unix_connected(s: *const socket) -> bool {
    unsafe { (*s).state == SS_CONNECTED }
}

// External function declarations
extern "C" {
    pub fn aa_sunaddr(u: *const unix_sock, addrlen: *mut c_int) -> *mut sockaddr_un;

    pub fn aa_unix_peer_perm(
        subj_cred: *const cred,
        label: *mut aa_label,
        op: *const u8,
        request: u32,
        sk: *mut sock,
        peer_sk: *mut sock,
        peer_label: *mut aa_label,
    ) -> c_int;

    pub fn aa_unix_sock_perm(
        op: *const u8,
        request: u32,
        sock: *mut socket,
    ) -> c_int;

    pub fn aa_unix_create_perm(
        label: *mut aa_label,
        family: c_int,
        type_: c_int,
        protocol: c_int,
    ) -> c_int;

    pub fn aa_unix_bind_perm(
        sock: *mut socket,
        address: *mut sockaddr,
        addrlen: c_int,
    ) -> c_int;

    pub fn aa_unix_connect_perm(
        sock: *mut socket,
        address: *mut sockaddr,
        addrlen: c_int,
    ) -> c_int;

    pub fn aa_unix_listen_perm(
        sock: *mut socket,
        backlog: c_int,
    ) -> c_int;

    pub fn aa_unix_accept_perm(
        sock: *mut socket,
        newsock: *mut socket,
    ) -> c_int;

    pub fn aa_unix_msg_perm(
        op: *const u8,
        request: u32,
        sock: *mut socket,
        msg: *mut msghdr,
        size: c_int,
    ) -> c_int;

    pub fn aa_unix_opt_perm(
        op: *const u8,
        request: u32,
        sock: *mut socket,
        level: c_int,
        optname: c_int,
    ) -> c_int;

    pub fn aa_unix_file_perm(
        subj_cred: *const cred,
        label: *mut aa_label,
        op: *const u8,
        request: u32,
        file: *mut file,
    ) -> c_int;
}

// Opaque external types
#[repr(C)]
pub struct unix_sock;

#[repr(C)]
pub struct sockaddr_un;

#[repr(C)]
pub struct sockaddr;

#[repr(C)]
pub struct aa_label;

#[repr(C)]
pub struct cred;

#[repr(C)]
pub struct sock;

#[repr(C)]
pub struct socket;

#[repr(C)]
pub struct msghdr;

#[repr(C)]
pub struct file;

pub type sa_family_t = u16;

// Note: SS_CONNECTED and unix_sk would be defined in external dependencies
// Placeholder for external constant
// const SS_CONNECTED: u32 = /* value from net/af_unix.h */;
// Placeholder for external macro
// fn unix_sk(sk: *const sock) -> *const unix_sock { /* implementation from net/af_unix.h */ }


// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
