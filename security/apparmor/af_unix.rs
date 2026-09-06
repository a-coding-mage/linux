// SPDX-License-Identifier: GPL-2.0-only
/*
 * AppArmor security module
 *
 * This file contains AppArmor af_unix fine grained mediation
 *
 * Copyright 2023 Canonical Ltd.
 *
 * This program is free software; you can redistribute it and/or
 * modify it under the terms of the GNU General Public License as
 * published by the Free Software Foundation, version 2 of the
 * License.
 */

// Linux kernel headers
// #include <linux/fs.h>
// #include <net/tcp_states.h>
// #include "include/audit.h"
// #include "include/af_unix.h"
// #include "include/apparmor.h"
// #include "include/file.h"
// #include "include/label.h"
// #include "include/net.h"
// #include "include/path.h"
// #include "include/policy.h"
// #include "include/cred.h"

use core::mem;
use core::ptr;

// External types from linux kernel and apparmor
type u32 = u32;
type u16 = u16;
type i32 = i32;
type c_char = i8;

// Forward declarations for external kernel types
#[repr(C)]
pub struct sock {
    // sk_family, sk_type, sk_protocol fields
}

#[repr(C)]
pub struct unix_sock {
    sk: sock,
    // Additional unix socket fields
}

#[repr(C)]
pub struct sockaddr_un {
    sun_family: u16,
    sun_path: [c_char; 108],
}

#[repr(C)]
pub struct unix_address {
    len: i32,
    name: *mut sockaddr_un,
}

#[repr(C)]
pub struct vfsuid_t {
    val: u32,
}

#[repr(C)]
pub struct path {
    dentry: *mut core::ffi::c_void,
    mnt: *mut core::ffi::c_void,
}

#[repr(C)]
pub struct inode {
    i_mode: u32,
}

#[repr(C)]
pub struct cred;

#[repr(C)]
pub struct aa_label;

#[repr(C)]
pub struct aa_profile {
    base: ProfileBase,
    label: aa_label,
}

#[repr(C)]
pub struct ProfileBase {
    hname: *const c_char,
}

#[repr(C)]
pub struct aa_ruleset {
    policy: *mut aa_policydb,
}

#[repr(C)]
pub struct aa_policydb {
    dfa: *mut aa_dfa,
}

#[repr(C)]
pub struct aa_dfa;

#[repr(C)]
pub struct aa_perms;

#[repr(C)]
pub struct path_cond {
    uid: u32,
    mode: u32,
}

#[repr(C)]
pub struct apparmor_audit_data {
    op: *const c_char,
    subj_cred: *const cred,
    peer: *const aa_label,
    info: *const c_char,
    net: AuditNet,
}

#[repr(C)]
pub struct AuditNet {
    addr: *mut sockaddr_un,
    addrlen: i32,
    peer: PeerInfo,
}

#[repr(C)]
pub struct PeerInfo {
    addr: *mut sockaddr_un,
    addrlen: i32,
}

#[repr(C)]
pub struct aa_sk_ctx;

#[repr(C)]
pub struct file {
    private_data: *mut core::ffi::c_void,
    f_cred: *const cred,
}

#[repr(C)]
pub struct socket {
    sk: *mut sock,
}

// Type aliases for state machines
type aa_state_t = u32;

// External functions from kernel and apparmor
extern "C" {
    fn aa_dfa_match_len(dfa: *const aa_dfa, state: aa_state_t, data: *const c_char, len: i32) -> aa_state_t;
    fn aa_dfa_null_transition(dfa: *const aa_dfa, state: aa_state_t) -> aa_state_t;
    fn aa_dfa_match(dfa: *const aa_dfa, state: aa_state_t, data: *const c_char) -> aa_state_t;
    fn aa_match_to_prot(policy: *mut aa_policydb, state: aa_state_t, request: u32,
                        family: i32, socket_type: i32, protocol: i32,
                        p: *mut *mut aa_perms, info: *mut *const c_char) -> aa_state_t;
    fn aa_do_perms(profile: *mut aa_profile, policy: *mut aa_policydb, state: aa_state_t, request: u32,
                   p: *const aa_perms, ad: *mut apparmor_audit_data) -> i32;
    fn aa_path_perm(op: *const c_char, subj_cred: *const cred, label: *mut aa_label,
                    path: *const path, path_type: i32, mask: u32, cond: *const path_cond) -> i32;
    fn aa_profile_af_perm(profile: *mut aa_profile, ad: *mut apparmor_audit_data,
                          request: u32, family: i32, socket_type: i32, protocol: i32) -> i32;
    fn aa_profile_af_sk_perm(profile: *mut aa_profile, ad: *mut apparmor_audit_data,
                             request: u32, sk: *mut sock) -> i32;
    fn smp_load_acquire(p: *const *const unix_address) -> *const unix_address;
    fn unconfined(label: *const aa_label) -> i32;
    fn label_mediates(label: *const aa_label, class: i32) -> i32;
    fn i_uid_into_vfsuid(mnt_idmap: *mut core::ffi::c_void, inode: *const inode) -> vfsuid_t;
    fn mnt_idmap(mnt: *mut core::ffi::c_void) -> *mut core::ffi::c_void;
    fn vfsuid_into_kuid(vfsuid: vfsuid_t) -> u32;
    fn aa_get_label(label: *const aa_label) -> *mut aa_label;
    fn aa_put_label(label: *mut aa_label);
    fn aa_label_merge(old: *const aa_label, new: *const aa_label, flags: i32) -> *mut aa_label;
    fn aa_label_is_subset(subset: *const aa_label, superset: *const aa_label) -> i32;
    fn aa_get_newest_label(label: *const aa_label) -> *mut aa_label;
    fn __aa_subj_label_is_cached(label: *const aa_label, cache: *const aa_label) -> i32;
    fn begin_current_label_crit_section(needput: *mut bool) -> *mut aa_label;
    fn end_current_label_crit_section(label: *mut aa_label, needput: bool);
    fn current_cred() -> *const cred;
    fn unix_addr(addr: *const core::ffi::c_void) -> *mut sockaddr_un;
    fn unix_sk(sk: *mut sock) -> *mut unix_sock;
    fn unix_addr_len(addrlen: i32) -> i32;
    fn is_unix_fs(sk: *mut sock) -> bool;
    fn is_unix_addr_fs(addr: *mut sockaddr_un, addrlen: i32) -> bool;
    fn unix_state_lock(sk: *mut sock);
    fn unix_state_unlock(sk: *mut sock);
    fn unix_peer(sk: *mut sock) -> *mut sock;
    fn sock_hold(sk: *mut sock);
    fn sock_put(sk: *mut sock);
    fn aa_sock(sk: *mut sock) -> *mut aa_sk_ctx;
    fn spin_lock(lock: *mut core::ffi::c_void);
    fn spin_unlock(lock: *mut core::ffi::c_void);
    fn rcu_read_lock();
    fn rcu_read_unlock();
    fn rcu_dereference(p: *const core::ffi::c_void) -> *const core::ffi::c_void;
    fn rcu_access_pointer(p: *const core::ffi::c_void) -> *const core::ffi::c_void;
    fn rcu_dereference_protected(p: *const core::ffi::c_void, c: i32) -> *const core::ffi::c_void;
    fn rcu_assign_pointer(p: *mut *const core::ffi::c_void, v: *const core::ffi::c_void);
    fn lockdep_is_held(lock: *mut core::ffi::c_void) -> i32;
    fn path_get(path: *mut path);
    fn path_put(path: *mut path);
}

// Macros - converted to consts and inline functions

const ABSTRACT_ADDR: &[u8] = b"\x00";
const ANONYMOUS_ADDR: &[u8] = b"\x01";
const DISCONNECTED_ADDR: &[u8] = b"\x02";
const SHUTDOWN_ADDR: &[u8] = b"\x03";
const FS_ADDR: &[u8] = b"/";

const CMD_ADDR: c_char = 1;
const CMD_LISTEN: c_char = 2;
const CMD_OPT: c_char = 4;

const NET_FS_PERMS: u32 = 0;
const NET_PEER_MASK: u32 = 0;
const PATH_SOCK_COND: i32 = 0;
const AA_CLASS_FILE: i32 = 0;
const AA_MAY_CREATE: u32 = 0;
const AA_MAY_BIND: u32 = 0;
const AA_MAY_LISTEN: u32 = 0;
const AA_MAY_ACCEPT: u32 = 0;
const MAY_READ: u32 = 0;
const MAY_WRITE: u32 = 0;
const PF_UNIX: i32 = 0;
const GFP_ATOMIC: i32 = 0;
const EINVAL: i32 = 22;

// Macros for audit and operations
// DEFINE_AUDIT_NET, DEFINE_AUDIT_SK, RULE_MEDIATES_UNIX, etc.
// These are converted to inline functions or expanded as needed

macro_rules! AA_BUG {
    ($cond:expr) => {
        if $cond {
            // BUG macro - undefined behavior assertion in kernel
        }
    };
}

macro_rules! fn_for_each_confined {
    ($label:expr, $profile:ident, $body:expr) => {
        {
            let mut result = 0;
            // Iterate over confined profiles in label
            result
        }
    };
}

macro_rules! fn_for_each_in_scope {
    ($label:expr, $peerp:ident, $body:expr) => {
        {
            let mut result = 0;
            // Iterate over profiles in scope
            result
        }
    };
}

macro_rules! last_error {
    ($current:expr, $new:expr) => {
        if $current == 0 {
            $current = $new;
        }
    };
}

macro_rules! xcheck {
    ($a:expr, $b:expr) => {
        ($a).max($b)
    };
}

// Static helper functions

#[inline]
unsafe fn aa_unix_sk(u: *mut unix_sock) -> *mut sock {
    &mut (*u).sk as *mut sock
}

unsafe fn unix_fs_perm(op: *const c_char, mask: u32, subj_cred: *const cred,
                      label: *mut aa_label, path: *const path) -> i32 {
    AA_BUG!(label.is_null());
    AA_BUG!(path.is_null());

    if unconfined(label) != 0 || label_mediates(label, AA_CLASS_FILE) == 0 {
        return 0;
    }

    let mask = mask & NET_FS_PERMS;

    if !(*path).dentry.is_null() {
        let inode = (*(*path).dentry) as *mut inode;
        let vfsuid = i_uid_into_vfsuid(mnt_idmap((*path).mnt), inode as *const inode);
        let cond = path_cond {
            uid: vfsuid_into_kuid(vfsuid),
            mode: (*inode).i_mode,
        };

        return aa_path_perm(op, subj_cred, label, path,
                           PATH_SOCK_COND, mask, &cond);
    }

    0
}

unsafe fn match_addr(dfa: *const aa_dfa, state: aa_state_t,
                    addr: *mut sockaddr_un, addrlen: i32) -> aa_state_t {
    let mut state = state;
    if !addr.is_null() {
        state = aa_dfa_match_len(dfa, state, (*addr).sun_path.as_ptr(),
                                unix_addr_len(addrlen));
    } else {
        state = aa_dfa_match_len(dfa, state, ANONYMOUS_ADDR.as_ptr() as *const c_char, 1);
    }
    state = aa_dfa_null_transition(dfa, state);
    state
}

unsafe fn match_to_local(policy: *mut aa_policydb, state: aa_state_t, request: u32,
                        socket_type: i32, protocol: i32,
                        addr: *mut sockaddr_un, addrlen: i32,
                        p: *mut *mut aa_perms,
                        info: *mut *const c_char) -> aa_state_t {
    let mut state = aa_match_to_prot(policy, state, request, PF_UNIX, socket_type,
                                    protocol, ptr::null_mut(), info);
    if state != 0 {
        state = match_addr((*policy).dfa, state, addr, addrlen);
        if state != 0 {
            state = aa_dfa_null_transition((*policy).dfa, state);
            if state == 0 {
                *info = b"failed local label match\0".as_ptr() as *const c_char;
            }
        } else {
            *info = b"failed local address match\0".as_ptr() as *const c_char;
        }
    }
    state
}

pub unsafe fn aa_sunaddr(u: *const unix_sock, addrlen: *mut i32) -> *mut sockaddr_un {
    let addr = smp_load_acquire(&(*u).addr);
    if !addr.is_null() {
        *addrlen = (*addr).len;
        return (*addr).name;
    }
    *addrlen = 0;
    ptr::null_mut()
}

unsafe fn match_to_sk(policy: *mut aa_policydb, state: aa_state_t, request: u32,
                     u: *mut unix_sock, p: *mut *mut aa_perms,
                     info: *mut *const c_char) -> aa_state_t {
    let mut addrlen: i32 = 0;
    let addr = aa_sunaddr(u, &mut addrlen);

    match_to_local(policy, state, request, (*(*u).sk).sk_type,
                  (*(*u).sk).sk_protocol, addr, addrlen, p, info)
}

unsafe fn match_to_cmd(policy: *mut aa_policydb, state: aa_state_t, request: u32,
                      u: *mut unix_sock, cmd: c_char, p: *mut *mut aa_perms,
                      info: *mut *const c_char) -> aa_state_t {
    AA_BUG!(p.is_null());

    let mut state = match_to_sk(policy, state, request, u, p, info);
    if state != 0 && (*p).is_null() {
        state = aa_dfa_match_len((*policy).dfa, state, &cmd as *const c_char, 1);
        if state == 0 {
            *info = b"failed cmd selection match\0".as_ptr() as *const c_char;
        }
    }
    state
}

unsafe fn match_to_peer(policy: *mut aa_policydb, state: aa_state_t, request: u32,
                       u: *mut unix_sock, peer_addr: *mut sockaddr_un, peer_addrlen: i32,
                       p: *mut *mut aa_perms, info: *mut *const c_char) -> aa_state_t {
    AA_BUG!(p.is_null());

    let mut state = match_to_cmd(policy, state, request, u, CMD_ADDR, p, info);
    if state != 0 && (*p).is_null() {
        state = match_addr((*policy).dfa, state, peer_addr, peer_addrlen);
        if state == 0 {
            *info = b"failed peer address match\0".as_ptr() as *const c_char;
        }
    }
    state
}

unsafe fn match_label(profile: *mut aa_profile, rule: *mut aa_ruleset, state: aa_state_t,
                     request: u32, peer: *mut aa_profile, p: *const aa_perms,
                     ad: *mut apparmor_audit_data) -> i32 {
    AA_BUG!(profile.is_null());
    AA_BUG!(peer.is_null());

    (*ad).peer = &(*peer).label as *const aa_label;

    let mut state = state;
    if state != 0 && p.is_null() {
        state = aa_dfa_match((*(*rule).policy).dfa, state,
                            (*peer).base.hname);
        if state == 0 {
            (*ad).info = b"failed peer label match\0".as_ptr() as *const c_char;
        }
    }

    aa_do_perms(profile, (*rule).policy, state, request, p, ad)
}

unsafe fn profile_create_perm(profile: *mut aa_profile, family: i32,
                             socket_type: i32, protocol: i32,
                             ad: *mut apparmor_audit_data) -> i32 {
    AA_BUG!(profile.is_null());

    let rules = (*profile).label.rules[0];
    let state = 0; // RULE_MEDIATES_UNIX(rules)

    if state != 0 {
        let state = aa_match_to_prot((*rules).policy, state, AA_MAY_CREATE,
                                    PF_UNIX, socket_type, protocol, ptr::null_mut(),
                                    &mut (*ad).info);

        return aa_do_perms(profile, (*rules).policy, state, AA_MAY_CREATE,
                          ptr::null(), ad);
    }

    aa_profile_af_perm(profile, ad, AA_MAY_CREATE, family, socket_type,
                      protocol)
}

unsafe fn profile_sk_perm(profile: *mut aa_profile,
                         ad: *mut apparmor_audit_data, request: u32,
                         sk: *mut sock, path: *const path) -> i32 {
    AA_BUG!(profile.is_null());
    AA_BUG!(sk.is_null());

    let rules = (*profile).label.rules[0];
    let mut p: *mut aa_perms = ptr::null_mut();
    let state = 0; // RULE_MEDIATES_UNIX(rules)

    if state != 0 {
        if is_unix_fs(sk) != 0 {
            return unix_fs_perm((*ad).op, request, (*ad).subj_cred,
                               &mut (*profile).label,
                               &(*unix_sk(sk)).path as *const path);
        }

        let state = match_to_sk((*rules).policy, state, request, unix_sk(sk),
                               &mut p, &mut (*ad).info);

        return aa_do_perms(profile, (*rules).policy, state, request, p, ad);
    }

    aa_profile_af_sk_perm(profile, ad, request, sk)
}

unsafe fn profile_bind_perm(profile: *mut aa_profile, sk: *mut sock,
                           ad: *mut apparmor_audit_data) -> i32 {
    AA_BUG!(profile.is_null());
    AA_BUG!(sk.is_null());
    AA_BUG!(ad.is_null());

    let rules = (*profile).label.rules[0];
    let mut p: *mut aa_perms = ptr::null_mut();
    let state = 0; // RULE_MEDIATES_UNIX(rules)

    if state != 0 {
        if is_unix_addr_fs((*ad).net.addr, (*ad).net.addrlen) != 0 {
            return 0;
        }

        let state = match_to_local((*rules).policy, state, AA_MAY_BIND,
                                  (*sk).sk_type, (*sk).sk_protocol,
                                  unix_addr((*ad).net.addr as *const core::ffi::c_void),
                                  (*ad).net.addrlen,
                                  &mut p, &mut (*ad).info);

        return aa_do_perms(profile, (*rules).policy, state, AA_MAY_BIND,
                          p, ad);
    }

    aa_profile_af_sk_perm(profile, ad, AA_MAY_BIND, sk)
}

unsafe fn profile_listen_perm(profile: *mut aa_profile, sk: *mut sock,
                             backlog: i32, ad: *mut apparmor_audit_data) -> i32 {
    AA_BUG!(profile.is_null());
    AA_BUG!(sk.is_null());
    AA_BUG!(ad.is_null());

    let rules = (*profile).label.rules[0];
    let mut p: *mut aa_perms = ptr::null_mut();
    let state = 0; // RULE_MEDIATES_UNIX(rules)

    if state != 0 {
        let b = (backlog as u16).to_be();

        if is_unix_fs(sk) != 0 {
            return unix_fs_perm((*ad).op, AA_MAY_LISTEN,
                               (*ad).subj_cred, &mut (*profile).label,
                               &(*unix_sk(sk)).path as *const path);
        }

        let mut state = match_to_cmd((*rules).policy, state, AA_MAY_LISTEN,
                                    unix_sk(sk), CMD_LISTEN, &mut p, &mut (*ad).info);
        if state != 0 && p.is_null() {
            state = aa_dfa_match_len((*(*rules).policy).dfa, state,
                                    &b as *const u16 as *const c_char, 2);
            if state == 0 {
                (*ad).info = b"failed listen backlog match\0".as_ptr() as *const c_char;
            }
        }
        return aa_do_perms(profile, (*rules).policy, state, AA_MAY_LISTEN,
                          p, ad);
    }

    aa_profile_af_sk_perm(profile, ad, AA_MAY_LISTEN, sk)
}

unsafe fn profile_accept_perm(profile: *mut aa_profile,
                             sk: *mut sock,
                             ad: *mut apparmor_audit_data) -> i32 {
    AA_BUG!(profile.is_null());
    AA_BUG!(sk.is_null());
    AA_BUG!(ad.is_null());

    let rules = (*profile).label.rules[0];
    let mut p: *mut aa_perms = ptr::null_mut();
    let state = 0; // RULE_MEDIATES_UNIX(rules)

    if state != 0 {
        if is_unix_fs(sk) != 0 {
            return unix_fs_perm((*ad).op, AA_MAY_ACCEPT,
                               (*ad).subj_cred, &mut (*profile).label,
                               &(*unix_sk(sk)).path as *const path);
        }

        let state = match_to_sk((*rules).policy, state, AA_MAY_ACCEPT,
                               unix_sk(sk), &mut p, &mut (*ad).info);

        return aa_do_perms(profile, (*rules).policy, state, AA_MAY_ACCEPT,
                          p, ad);
    }

    aa_profile_af_sk_perm(profile, ad, AA_MAY_ACCEPT, sk)
}

unsafe fn profile_opt_perm(profile: *mut aa_profile, request: u32,
                          sk: *mut sock, optname: i32,
                          ad: *mut apparmor_audit_data) -> i32 {
    AA_BUG!(profile.is_null());
    AA_BUG!(sk.is_null());
    AA_BUG!(ad.is_null());

    let rules = (*profile).label.rules[0];
    let mut p: *mut aa_perms = ptr::null_mut();
    let state = 0; // RULE_MEDIATES_UNIX(rules)

    if state != 0 {
        let b = (optname as u16).to_be();
        if is_unix_fs(sk) != 0 {
            return unix_fs_perm((*ad).op, request,
                               (*ad).subj_cred, &mut (*profile).label,
                               &(*unix_sk(sk)).path as *const path);
        }

        let mut state = match_to_cmd((*rules).policy, state, request, unix_sk(sk),
                                    CMD_OPT, &mut p, &mut (*ad).info);
        if state != 0 && p.is_null() {
            state = aa_dfa_match_len((*(*rules).policy).dfa, state,
                                    &b as *const u16 as *const c_char, 2);
            if state == 0 {
                (*ad).info = b"failed sockopt match\0".as_ptr() as *const c_char;
            }
        }
        return aa_do_perms(profile, (*rules).policy, state, request, p, ad);
    }

    aa_profile_af_sk_perm(profile, ad, request, sk)
}

unsafe fn profile_peer_perm(profile: *mut aa_profile, request: u32,
                           sk: *mut sock, path: *const path,
                           peer_addr: *mut sockaddr_un,
                           peer_addrlen: i32, peer_path: *const path,
                           peer_label: *mut aa_label,
                           ad: *mut apparmor_audit_data) -> i32 {
    AA_BUG!(profile.is_null());
    AA_BUG!(sk.is_null());
    AA_BUG!(peer_label.is_null());
    AA_BUG!(ad.is_null());

    let rules = (*profile).label.rules[0];
    let mut p: *mut aa_perms = ptr::null_mut();
    let state = 0; // RULE_MEDIATES_UNIX(rules)

    if state != 0 {
        if !peer_path.is_null() {
            return unix_fs_perm((*ad).op, request, (*ad).subj_cred,
                               &mut (*profile).label, peer_path);
        } else if !path.is_null() {
            return unix_fs_perm((*ad).op, request, (*ad).subj_cred,
                               &mut (*profile).label, path);
        }
        let state = match_to_peer((*rules).policy, state, request,
                                 unix_sk(sk),
                                 peer_addr, peer_addrlen, &mut p, &mut (*ad).info);

        return fn_for_each_in_scope!(peer_label, peerp, {
            match_label(profile, rules, state, request, peerp, p, ad)
        });
    }

    aa_profile_af_sk_perm(profile, ad, request, sk)
}

// Public exported functions

pub unsafe fn aa_unix_create_perm(label: *mut aa_label, family: i32, socket_type: i32,
                                 protocol: i32) -> i32 {
    if unconfined(label) == 0 {
        return 0;
    }

    return fn_for_each_confined!(label, profile, {
        profile_create_perm(profile, family, socket_type, protocol, ptr::null_mut())
    });
}

unsafe fn aa_unix_label_sk_perm(subj_cred: *const cred,
                                label: *mut aa_label,
                                op: *const c_char, request: u32, sk: *mut sock,
                                path: *const path) -> i32 {
    if unconfined(label) == 0 {
        return 0;
    }

    return fn_for_each_confined!(label, profile, {
        profile_sk_perm(profile, ptr::null_mut(), request, sk, path)
    });
}

pub unsafe fn aa_unix_sock_perm(op: *const c_char, request: u32, sock: *mut socket) -> i32 {
    let mut needput = false;
    let label = begin_current_label_crit_section(&mut needput);
    let error = aa_unix_label_sk_perm(current_cred(), label, op,
                                     request, (*sock).sk,
                                     if is_unix_fs((*sock).sk) != 0 {
                                         &(*unix_sk((*sock).sk)).path as *const path
                                     } else {
                                         ptr::null()
                                     });
    end_current_label_crit_section(label, needput);

    error
}

unsafe fn valid_addr(addr: *mut sockaddr_un, addr_len: i32) -> i32 {
    if addr_len < mem::offset_of!(sockaddr_un, sun_path) as i32 ||
       addr_len > mem::size_of::<sockaddr_un>() as i32 {
        return -EINVAL;
    }
    0
}

pub unsafe fn aa_unix_bind_perm(sock: *mut socket, addr: *mut sockaddr_un,
                               addrlen: i32) -> i32 {
    let error = valid_addr(addr, addrlen);
    if error != 0 {
        return error;
    }

    let mut needput = false;
    let label = begin_current_label_crit_section(&mut needput);
    let mut error = 0;

    if unconfined(label) == 0 {
        return fn_for_each_confined!(label, profile, {
            profile_bind_perm(profile, (*sock).sk, ptr::null_mut())
        });
    }
    end_current_label_crit_section(label, needput);

    error
}

pub unsafe fn aa_unix_listen_perm(sock: *mut socket, backlog: i32) -> i32 {
    let mut needput = false;
    let label = begin_current_label_crit_section(&mut needput);
    let mut error = 0;

    if unconfined(label) == 0 {
        error = fn_for_each_confined!(label, profile, {
            profile_listen_perm(profile, (*sock).sk, backlog, ptr::null_mut())
        });
    }
    end_current_label_crit_section(label, needput);

    error
}

pub unsafe fn aa_unix_accept_perm(sock: *mut socket, newsock: *mut socket) -> i32 {
    let mut needput = false;
    let label = begin_current_label_crit_section(&mut needput);
    let mut error = 0;

    if unconfined(label) == 0 {
        error = fn_for_each_confined!(label, profile, {
            profile_accept_perm(profile, (*sock).sk, ptr::null_mut())
        });
    }
    end_current_label_crit_section(label, needput);

    error
}

pub unsafe fn aa_unix_opt_perm(op: *const c_char, request: u32, sock: *mut socket,
                              level: i32, optname: i32) -> i32 {
    let mut needput = false;
    let label = begin_current_label_crit_section(&mut needput);
    let mut error = 0;

    if unconfined(label) == 0 {
        error = fn_for_each_confined!(label, profile, {
            profile_opt_perm(profile, request, (*sock).sk, optname, ptr::null_mut())
        });
    }
    end_current_label_crit_section(label, needput);

    error
}

unsafe fn unix_peer_perm(subj_cred: *const cred,
                        label: *mut aa_label, op: *const c_char, request: u32,
                        sk: *mut sock, path: *const path,
                        peer_addr: *mut sockaddr_un, peer_addrlen: i32,
                        peer_path: *const path, peer_label: *mut aa_label) -> i32 {
    return fn_for_each_confined!(label, profile, {
        profile_peer_perm(profile, request, sk, path,
                         peer_addr, peer_addrlen, peer_path,
                         peer_label, ptr::null_mut())
    });
}

pub unsafe fn aa_unix_peer_perm(subj_cred: *const cred,
                               label: *mut aa_label, op: *const c_char, request: u32,
                               sk: *mut sock, peer_sk: *mut sock,
                               peer_label: *mut aa_label) -> i32 {
    AA_BUG!(label.is_null());
    AA_BUG!(sk.is_null());
    AA_BUG!(peer_sk.is_null());
    AA_BUG!(peer_label.is_null());

    let peeru = unix_sk(peer_sk);
    let u = unix_sk(sk);
    let mut plen: i32 = 0;
    let paddr = aa_sunaddr(unix_sk(peer_sk), &mut plen);

    unix_peer_perm(subj_cred, label, op, request, sk,
                  if is_unix_fs(sk) != 0 { &(*u).path as *const path } else { ptr::null() },
                  paddr, plen,
                  if is_unix_fs(peer_sk) != 0 { &(*peeru).path as *const path } else { ptr::null() },
                  peer_label)
}

unsafe fn update_sk_ctx(sk: *mut sock, label: *mut aa_label,
                       plabel: *mut aa_label) {
    let ctx = aa_sock(sk);

    rcu_read_lock();
    let update_sk = (
        !plabel.is_null() &&
        (plabel != rcu_access_pointer((*ctx).peer_lastupdate) as *mut aa_label ||
         aa_label_is_subset(plabel as *const aa_label,
                           rcu_dereference((*ctx).peer as *mut core::ffi::c_void) as *const aa_label) == 0)
    ) ||
    __aa_subj_label_is_cached(label,
                             rcu_dereference((*ctx).label as *mut core::ffi::c_void) as *const aa_label) == 0;
    rcu_read_unlock();

    if update_sk == 0 {
        return;
    }

    spin_lock(&(*unix_sk(sk)).lock);

    let old = rcu_dereference_protected((*ctx).label as *mut core::ffi::c_void,
                                        lockdep_is_held(&(*unix_sk(sk)).lock)) as *mut aa_label;
    let l = aa_label_merge(old as *const aa_label, label as *const aa_label, GFP_ATOMIC);

    if !l.is_null() {
        if l != old {
            rcu_assign_pointer(&mut (*ctx).label as *mut *mut aa_label as *mut *const core::ffi::c_void, l as *const core::ffi::c_void);
            aa_put_label(old);
        } else {
            aa_put_label(l);
        }
    }

    if !plabel.is_null() && rcu_access_pointer((*ctx).peer_lastupdate) != plabel as *const core::ffi::c_void {
        let old = rcu_dereference_protected((*ctx).peer as *mut core::ffi::c_void,
                                           lockdep_is_held(&(*unix_sk(sk)).lock)) as *mut aa_label;

        if old == plabel {
            rcu_assign_pointer(&mut (*ctx).peer_lastupdate as *mut *mut aa_label as *mut *const core::ffi::c_void,
                              aa_get_label(plabel as *const aa_label) as *const core::ffi::c_void);
        } else if aa_label_is_subset(plabel as *const aa_label, old as *const aa_label) != 0 {
            rcu_assign_pointer(&mut (*ctx).peer_lastupdate as *mut *mut aa_label as *mut *const core::ffi::c_void,
                              aa_get_label(plabel as *const aa_label) as *const core::ffi::c_void);
            rcu_assign_pointer(&mut (*ctx).peer as *mut *mut aa_label as *mut *const core::ffi::c_void,
                              aa_get_label(plabel as *const aa_label) as *const core::ffi::c_void);
            aa_put_label(old);
        }
    }
    spin_unlock(&(*unix_sk(sk)).lock);
}

unsafe fn update_peer_ctx(sk: *mut sock, ctx: *mut aa_sk_ctx,
                         label: *mut aa_label) {
    spin_lock(&(*unix_sk(sk)).lock);

    let old = rcu_dereference_protected((*ctx).peer as *mut core::ffi::c_void,
                                        lockdep_is_held(&(*unix_sk(sk)).lock)) as *mut aa_label;
    let l = aa_label_merge(old as *const aa_label, label as *const aa_label, GFP_ATOMIC);

    if !l.is_null() {
        if l != old {
            rcu_assign_pointer(&mut (*ctx).peer as *mut *mut aa_label as *mut *const core::ffi::c_void, l as *const core::ffi::c_void);
            aa_put_label(old);
        } else {
            aa_put_label(l);
        }
    }

    spin_unlock(&(*unix_sk(sk)).lock);
}

pub unsafe fn aa_unix_file_perm(subj_cred: *const cred, label: *mut aa_label,
                               op: *const c_char, request: u32, file: *mut file) -> i32 {
    AA_BUG!(label.is_null());
    AA_BUG!(file.is_null());

    let sock = (*file).private_data as *mut socket;
    AA_BUG!((*sock).sk.is_null());

    let mut addrlen: i32 = 0;
    let mut peer_addrlen: i32 = 0;
    let mut plabel: *mut aa_label = ptr::null_mut();
    let mut peer_sk: *mut sock = ptr::null_mut();
    let sk_req = request & !NET_PEER_MASK;
    let mut is_sk_fs: i32 = 0;
    let mut error = 0;
    let path: path;

    unix_state_lock((*sock).sk);
    peer_sk = unix_peer((*sock).sk);
    if !peer_sk.is_null() {
        sock_hold(peer_sk);
    }

    is_sk_fs = is_unix_fs((*sock).sk);
    let addr = aa_sunaddr(unix_sk((*sock).sk), &mut addrlen);
    path = (*unix_sk((*sock).sk)).path;
    unix_state_unlock((*sock).sk);

    if is_sk_fs != 0 && !peer_sk.is_null() {
        // sk_req = request
    }

    if sk_req != 0 {
        error = aa_unix_label_sk_perm(subj_cred, label, op,
                                     sk_req, (*sock).sk,
                                     if is_sk_fs != 0 { &path as *const path } else { ptr::null() });
    }

    if peer_sk.is_null() {
        aa_put_label(plabel);
        return error;
    }

    if is_sk_fs == 0 {
        let is_peer_fs = is_unix_fs(peer_sk);
        let peer_addr = aa_sunaddr(unix_sk(peer_sk), &mut peer_addrlen);

        if is_peer_fs != 0 {
            let mut peer_path: path;

            unix_state_lock(peer_sk);
            peer_path = (*unix_sk(peer_sk)).path;
            if !peer_path.dentry.is_null() {
                path_get(&mut peer_path);
            }
            unix_state_unlock(peer_sk);

            let err = unix_fs_perm(op, request, subj_cred, label, &peer_path);
            if error == 0 {
                error = err;
            }

            if !peer_path.dentry.is_null() {
                path_put(&mut peer_path);
            }
        } else {
            let pctx = aa_sock(peer_sk);

            rcu_read_lock();
            plabel = aa_get_newest_label((*pctx).label);
            rcu_read_unlock();

            let err1 = unix_peer_perm(subj_cred, label, op,
                                     MAY_READ | MAY_WRITE, (*sock).sk,
                                     if is_sk_fs != 0 { &path as *const path } else { ptr::null() },
                                     peer_addr, peer_addrlen,
                                     ptr::null(), plabel);
            let err2 = unix_peer_perm((*file).f_cred, plabel, op,
                                     MAY_READ | MAY_WRITE, peer_sk,
                                     ptr::null(),
                                     addr, addrlen,
                                     if is_sk_fs != 0 { &path as *const path } else { ptr::null() },
                                     label);

            if err1 != 0 {
                error = err1;
            } else if err2 != 0 {
                error = err2;
            }

            if error == 0 && __aa_subj_label_is_cached(plabel, label) == 0 {
                update_peer_ctx(peer_sk, pctx, label);
            }
        }
    }

    sock_put(peer_sk);

    if error == 0 {
        update_sk_ctx((*sock).sk, label, plabel);
    }
    aa_put_label(plabel);

    error
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
