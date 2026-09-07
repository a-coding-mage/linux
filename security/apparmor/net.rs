// SPDX-License-Identifier: GPL-2.0-only
/*
 * AppArmor security module
 *
 * This file contains AppArmor network mediation
 *
 * Copyright (C) 1998-2008 Novell/SUSE
 * Copyright 2009-2017 Canonical Ltd.
 */

// External dependencies from:
// #include "include/af_unix.h"
// #include "include/af_inet.h"
// #include "include/apparmor.h"
// #include "include/audit.h"
// #include "include/cred.h"
// #include "include/label.h"
// #include "include/net.h"
// #include "include/policy.h"
// #include "include/secid.h"
// #include "net_names.h"

use std::ffi::c_char;
use std::os::raw::c_int;

// External types and constants (from included headers)
extern "C" {
    type audit_buffer;
    type sockaddr_un;
    type sock;
    type unix_sock;
    type aa_dfa;
    type aa_policydb;
    type aa_profile;
    type aa_perms;
    type common_audit_data;
    type apparmor_audit_data;
    type cred;
    type aa_label;
    type aa_ruleset;
    type aa_sk_ctx;
    type socket;
    type file;
    type aa_secmark;

    // External functions
    fn unix_addr_len(addrlen: c_int) -> c_int;
    fn audit_log_format(ab: *mut audit_buffer, fmt: *const c_char, ...);
    fn audit_log_untrustedstring(ab: *mut audit_buffer, str: *const c_char);
    fn audit_string_contains_control(str: *const c_char, len: c_int) -> bool;
    fn audit_log_n_hex(ab: *mut audit_buffer, str: *const c_char, len: c_int);
    fn aad(sa: *mut common_audit_data) -> *mut apparmor_audit_data;
    fn aa_audit_perm_mask(
        ab: *mut audit_buffer,
        mask: u32,
        arg3: *const c_char,
        arg4: c_int,
        mask_names: *const *const c_char,
        mask_bits: u32,
    );
    fn unix_sk(sk: *const sock) -> *const unix_sock;
    fn aa_sunaddr(u: *const unix_sock, addrlen: *mut c_int) -> *mut sockaddr_un;
    fn unix_addr(addr: *mut sockaddr_un) -> *mut sockaddr_un;
    fn aa_check_perms(
        profile: *mut aa_profile,
        perms: *const aa_perms,
        request: u32,
        ad: *mut apparmor_audit_data,
        cb: unsafe extern "C" fn(*mut audit_buffer, *mut c_char),
    ) -> c_int;
    fn aa_lookup_perms(policy: *mut aa_policydb, state: aa_state_t) -> *mut aa_perms;
    fn aa_apply_modes_to_perms(profile: *mut aa_profile, perms: *mut aa_perms);
    fn aa_label_xaudit(
        ab: *mut audit_buffer,
        ns: *mut c_char,
        label: *mut aa_label,
        flags: c_int,
        gfp: c_int,
    );
    fn labels_ns(label: *mut aa_label) -> *mut c_char;
    fn aa_dfa_match_len(
        dfa: *const aa_dfa,
        state: aa_state_t,
        data: *const c_char,
        len: c_int,
    ) -> aa_state_t;
    fn cpu_to_be16(val: u16) -> u16;
    fn rcu_access_pointer(ptr: *const c_char) -> *const c_char;
    fn unconfined(label: *const aa_label) -> bool;
    fn fn_for_each_confined(
        label: *const aa_label,
        profile: *mut *mut aa_profile,
        cb: unsafe extern "C" fn(*mut aa_profile) -> c_int,
    ) -> c_int;
    fn current_cred() -> *const cred;
    fn begin_current_label_crit_section(needput: *mut bool) -> *mut aa_label;
    fn end_current_label_crit_section(label: *mut aa_label, needput: bool);
    fn aa_sock(sk: *const sock) -> *mut aa_sk_ctx;
    fn aa_label_strn_parse(
        base: *const aa_label,
        str: *const c_char,
        strlen: usize,
        gfp: c_int,
        bool_arg1: bool,
        bool_arg2: bool,
    ) -> *mut aa_label;
    fn aa_put_label(label: *mut aa_label);
    fn aa_unix_file_perm(subj_cred: *const cred, label: *mut aa_label, op: *const c_char, request: u32, file: *mut file) -> c_int;
    fn aa_inet_file_perm(subj_cred: *const cred, label: *mut aa_label, op: *const c_char, request: u32, sock: *mut socket) -> c_int;
    fn IS_ERR(ptr: *const c_char) -> bool;
    fn PTR_ERR(ptr: *const c_char) -> c_int;
    fn strlen(s: *const c_char) -> usize;

    // External global variables
    static mut address_family_names: *const *const c_char;
    static mut sock_type_names: *const *const c_char;
    static root_ns: *mut c_char;
}

pub type aa_state_t = u32;

#[repr(C)]
pub struct aa_sfs_entry {
    pub name: *const c_char,
    pub mode: u32,
}

pub static AA_SFS_ENTRY_NETWORK: [aa_sfs_entry; 3] = [
    aa_sfs_entry {
        name: b"af_mask\0" as *const u8 as *const c_char,
        mode: 0, // AA_SFS_AF_MASK
    },
    aa_sfs_entry {
        name: b"tcp-fast-open\0" as *const u8 as *const c_char,
        mode: 1,
    },
    aa_sfs_entry {
        name: std::ptr::null(),
        mode: 0,
    },
];

pub static AA_SFS_ENTRY_NETWORKV9: [aa_sfs_entry; 4] = [
    aa_sfs_entry {
        name: b"af_mask\0" as *const u8 as *const c_char,
        mode: 0, // AA_SFS_AF_MASK
    },
    aa_sfs_entry {
        name: b"af_unix\0" as *const u8 as *const c_char,
        mode: 1,
    },
    aa_sfs_entry {
        name: b"tcp-fast-open\0" as *const u8 as *const c_char,
        mode: 1,
    },
    aa_sfs_entry {
        name: std::ptr::null(),
        mode: 0,
    },
];

static NET_MASK_NAMES: &[&str] = &[
    "unknown", "send", "receive", "unknown",
    "create", "shutdown", "connect", "unknown",
    "setattr", "getattr", "setcred", "getcred",
    "chmod", "chown", "chgrp", "lock",
    "mmap", "mprot", "unknown", "unknown",
    "accept", "bind", "listen", "unknown",
    "setopt", "getopt", "unknown", "unknown",
    "unknown", "unknown", "unknown", "unknown",
];

unsafe fn audit_unix_addr(
    ab: *mut audit_buffer,
    str: *const c_char,
    addr: *mut sockaddr_un,
    addrlen: c_int,
) {
    let len = unix_addr_len(addrlen);

    if addr.is_null() || len <= 0 {
        audit_log_format(ab, b" %s=none\0" as *const u8 as *const c_char, str);
    } else if (*addr).sun_path[0] != 0 {
        audit_log_format(ab, b" %s=\0" as *const u8 as *const c_char, str);
        audit_log_untrustedstring(ab, (*addr).sun_path.as_ptr());
    } else {
        audit_log_format(ab, b" %s=\"@\0" as *const u8 as *const c_char, str);
        if audit_string_contains_control(&(*addr).sun_path[1], len - 1) {
            audit_log_n_hex(ab, &(*addr).sun_path[1], len - 1);
        } else {
            audit_log_format(
                ab,
                b"%.*s\0" as *const u8 as *const c_char,
                len - 1,
                &(*addr).sun_path[1],
            );
        }
        audit_log_format(ab, b"\"\0" as *const u8 as *const c_char);
    }
}

unsafe fn audit_unix_sk_addr(ab: *mut audit_buffer, str: *const c_char, sk: *const sock) {
    let u = unix_sk(sk);

    if !u.is_null() && !(*u).addr.is_null() {
        let mut addrlen: c_int = 0;
        let addr = aa_sunaddr(u, &mut addrlen);

        audit_unix_addr(ab, str, addr, addrlen);
    } else {
        audit_unix_addr(ab, str, std::ptr::null_mut(), 0);
    }
}

pub unsafe extern "C" fn audit_net_cb(ab: *mut audit_buffer, va: *mut c_char) {
    let sa = va as *mut common_audit_data;
    let ad = aad(sa);

    if !(*address_family_names).is_null()
        && !(*address_family_names.add((*ad).common.u.net as usize)).is_null()
    {
        audit_log_format(
            ab,
            b" family=\"%s\"\0" as *const u8 as *const c_char,
            *address_family_names.add((*ad).common.u.net as usize),
        );
    } else {
        audit_log_format(
            ab,
            b" family=\"unknown(%d)\"\0" as *const u8 as *const c_char,
            (*ad).common.u.net as usize,
        );
    }
    if !(*sock_type_names).is_null()
        && !(*sock_type_names.add((*ad).net.type as usize)).is_null()
    {
        audit_log_format(
            ab,
            b" sock_type=\"%s\"\0" as *const u8 as *const c_char,
            *sock_type_names.add((*ad).net.type as usize),
        );
    } else {
        audit_log_format(
            ab,
            b" sock_type=\"unknown(%d)\"\0" as *const u8 as *const c_char,
            (*ad).net.type,
        );
    }
    audit_log_format(
        ab,
        b" protocol=%d\0" as *const u8 as *const c_char,
        (*ad).net.protocol,
    );

    let net_perms_mask = 0xffffffff;
    if (*ad).request & net_perms_mask != 0 {
        audit_log_format(ab, b" requested=\0" as *const u8 as *const c_char);
        aa_audit_perm_mask(
            ab,
            (*ad).request,
            std::ptr::null(),
            0,
            NET_MASK_NAMES.as_ptr() as *const *const c_char,
            net_perms_mask,
        );

        if (*ad).denied & net_perms_mask != 0 {
            audit_log_format(ab, b" denied=\0" as *const u8 as *const c_char);
            aa_audit_perm_mask(
                ab,
                (*ad).denied,
                std::ptr::null(),
                0,
                NET_MASK_NAMES.as_ptr() as *const *const c_char,
                net_perms_mask,
            );
        }
    }
    let pf_unix = 1;
    if (*ad).common.u.net as usize == pf_unix {
        if (*ad).net.addr as usize != 0 || (*ad).common.u.net.is_null() {
            audit_unix_addr(
                ab,
                b"addr\0" as *const u8 as *const c_char,
                unix_addr((*ad).net.addr),
                (*ad).net.addrlen,
            );
        } else {
            audit_unix_sk_addr(ab, b"addr\0" as *const u8 as *const c_char, (*ad).common.u.net);
        }
        let net_peer_mask = 0x0000f000;
        if (*ad).request & net_peer_mask != 0 {
            audit_unix_addr(
                ab,
                b"peer_addr\0" as *const u8 as *const c_char,
                unix_addr((*ad).net.peer.addr),
                (*ad).net.peer.addrlen,
            );
        }
    }
    if !(*ad).peer.is_null() {
        audit_log_format(ab, b" peer=\0" as *const u8 as *const c_char);
        aa_label_xaudit(
            ab,
            labels_ns((*ad).subj_label),
            (*ad).peer,
            0,
            0,
        );
    }
}

pub unsafe fn aa_do_perms(
    profile: *mut aa_profile,
    policy: *mut aa_policydb,
    state: aa_state_t,
    request: u32,
    p: *const aa_perms,
    ad: *mut apparmor_audit_data,
) -> c_int {
    let mut perms: aa_perms;

    assert!(!profile.is_null());
    assert!(!policy.is_null());

    let perms_ptr = if state != 0 || p.is_null() {
        aa_lookup_perms(policy, state)
    } else {
        p as *mut aa_perms
    };

    perms = *perms_ptr;
    aa_apply_modes_to_perms(profile, &mut perms);
    aa_check_perms(profile, &perms, request, ad, audit_net_cb)
}

unsafe fn early_match(policy: *mut aa_policydb, state: aa_state_t, request: u32) -> *mut aa_perms {
    let p = aa_lookup_perms(policy, state);

    if ((*p).allow & request) != request && ((*p).allow & 0x00008000) != 0 {
        return std::ptr::null_mut();
    }
    p
}

unsafe fn aa_dfa_match_be16(dfa: *const aa_dfa, state: aa_state_t, data: u16) -> aa_state_t {
    let buffer = cpu_to_be16(data);

    aa_dfa_match_len(
        dfa,
        state,
        &buffer as *const u16 as *const c_char,
        2,
    )
}

pub unsafe fn aa_match_to_prot(
    policy: *mut aa_policydb,
    mut state: aa_state_t,
    request: u32,
    af: u16,
    type_: c_int,
    protocol: c_int,
    p: *mut *mut aa_perms,
    info: *mut *const c_char,
) -> aa_state_t {
    state = aa_dfa_match_be16((*policy).dfa, state, af);
    if state == 0 {
        *info = b"failed af match\0" as *const u8 as *const c_char;
        return state;
    }
    state = aa_dfa_match_be16((*policy).dfa, state, type_ as u16);
    if state != 0 {
        if !p.is_null() {
            *p = early_match(policy, state, request);
        }
        if p.is_null() || (*p).is_null() {
            state = aa_dfa_match_be16((*policy).dfa, state, protocol as u16);
            if state == 0 {
                *info = b"failed protocol match\0" as *const u8 as *const c_char;
            }
        }
    } else {
        *info = b"failed type match\0" as *const u8 as *const c_char;
    }

    state
}

pub unsafe fn aa_profile_af_perm(
    profile: *mut aa_profile,
    ad: *mut apparmor_audit_data,
    request: u32,
    family: u16,
    type_: c_int,
    protocol: c_int,
) -> c_int {
    let rules = (*(*profile).label.rules[0]);
    let mut p: *mut aa_perms = std::ptr::null_mut();
    let mut state: aa_state_t;

    assert!(family as usize < 32);
    assert!(type_ >= 0 && type_ < 256);
    assert!(!profile.is_null());

    state = 0; // RULE_MEDIATES_NET(rules) - external macro
    if state != 0 {
        state = aa_match_to_prot((*rules).policy, state, request, family, type_, protocol, &mut p, &mut (*ad).info);
        return aa_do_perms(profile, (*rules).policy, state, request, p, ad);
    }

    0
}

pub unsafe fn aa_af_perm(
    subj_cred: *const cred,
    label: *mut aa_label,
    op: *const c_char,
    request: u32,
    family: u16,
    type_: c_int,
    protocol: c_int,
) -> c_int {
    let mut profile: *mut aa_profile = std::ptr::null_mut();
    // DEFINE_AUDIT_NET(ad, op, subj_cred, NULL, family, type, protocol);

    fn_for_each_confined(
        label as *const aa_label,
        &mut profile,
        |p: *mut aa_profile| {
            aa_profile_af_perm(p, std::ptr::null_mut(), request, family, type_, protocol)
        }
    )
}

pub unsafe fn aa_label_sk_perm(
    subj_cred: *const cred,
    label: *mut aa_label,
    op: *const c_char,
    request: u32,
    sk: *const sock,
) -> c_int {
    let ctx = aa_sock(sk);
    let mut error: c_int = 0;

    assert!(!label.is_null());
    assert!(!sk.is_null());

    let kernel_t = std::ptr::null();
    if rcu_access_pointer((*ctx).label as *const c_char) != kernel_t && !unconfined(label) {
        let mut profile: *mut aa_profile = std::ptr::null_mut();
        // DEFINE_AUDIT_SK(ad, op, subj_cred, sk);

        error = fn_for_each_confined(
            label as *const aa_label,
            &mut profile,
            |p: *mut aa_profile| {
                aa_profile_af_perm(
                    p,
                    std::ptr::null_mut(),
                    request,
                    (*sk).sk_family as u16,
                    (*sk).sk_type,
                    (*sk).sk_protocol,
                )
            }
        );
    }

    error
}

pub unsafe fn aa_sk_perm(op: *const c_char, request: u32, sk: *const sock) -> c_int {
    let mut label: *mut aa_label;
    let mut needput: bool = false;
    let error: c_int;

    assert!(!sk.is_null());

    label = begin_current_label_crit_section(&mut needput);
    error = aa_label_sk_perm(current_cred(), label, op, request, sk);
    end_current_label_crit_section(label, needput);

    error
}

pub unsafe fn aa_sock_file_perm(
    subj_cred: *const cred,
    label: *mut aa_label,
    op: *const c_char,
    request: u32,
    file: *mut file,
) -> c_int {
    let sock = (*file).private_data as *mut socket;

    assert!(!label.is_null());

    if sock.is_null() || (*sock).sk.is_null() {
        return 0;
    }

    let pf_unix = 1;
    let pf_inet = 2;
    let pf_inet6 = 10;

    match (*(*sock).sk).sk_family as c_int {
        1 => { // PF_UNIX
            aa_unix_file_perm(subj_cred, label, op, request, file)
        }
        2 | 10 => { // PF_INET or PF_INET6
            aa_inet_file_perm(subj_cred, label, op, request, sock)
        }
        _ => aa_label_sk_perm(subj_cred, label, op, request, (*sock).sk),
    }
}

// #ifdef CONFIG_NETWORK_SECMARK
unsafe fn apparmor_secmark_init(secmark: *mut aa_secmark) -> c_int {
    let mut label: *mut aa_label;

    if (*secmark).label[0] as u8 == b'*' {
        (*secmark).secid = 0xffffffff; // AA_SECID_WILDCARD
        return 0;
    }

    label = aa_label_strn_parse(
        &(*root_ns).label as *const c_char as *const aa_label,
        (*secmark).label.as_ptr(),
        strlen((*secmark).label.as_ptr()),
        0,
        false,
        false,
    );

    if IS_ERR(label as *const c_char) {
        return PTR_ERR(label as *const c_char);
    }

    (*secmark).secid = (*label).secid;
    aa_put_label(label);

    0
}

unsafe fn aa_secmark_perm(
    profile: *mut aa_profile,
    request: u32,
    secid: u32,
    ad: *mut apparmor_audit_data,
) -> c_int {
    let mut i: c_int;
    let mut ret: c_int;
    let mut perms = std::mem::zeroed::<aa_perms>();
    let rules = (*(*profile).label.rules[0]);

    if (*rules).secmark_count == 0 {
        return 0;
    }

    i = 0;
    while i < (*rules).secmark_count {
        if (*(*rules).secmark.add(i as usize)).secid == 0 {
            ret = apparmor_secmark_init(&mut *(*rules).secmark.add(i as usize));
            if ret != 0 {
                return ret;
            }
        }

        if (*(*rules).secmark.add(i as usize)).secid == secid
            || (*(*rules).secmark.add(i as usize)).secid == 0xffffffff
        {
            if (*(*rules).secmark.add(i as usize)).deny != 0 {
                (*perms).deny = 0xffffffff;
            } else {
                (*perms).allow = 0xffffffff;
            }

            if (*(*rules).secmark.add(i as usize)).audit != 0 {
                (*perms).audit = 0xffffffff;
            }
        }
        i += 1;
    }

    aa_apply_modes_to_perms(profile, &mut perms);

    aa_check_perms(profile, &perms, request, ad, audit_net_cb)
}

pub unsafe fn apparmor_secmark_check(
    label: *mut aa_label,
    op: *mut c_char,
    request: u32,
    secid: u32,
    sk: *const sock,
) -> c_int {
    let mut profile: *mut aa_profile = std::ptr::null_mut();
    // DEFINE_AUDIT_SK(ad, op, NULL, sk);

    fn_for_each_confined(
        label as *const aa_label,
        &mut profile,
        |p: *mut aa_profile| {
            aa_secmark_perm(p, request, secid, std::ptr::null_mut())
        }
    )
}
// #endif CONFIG_NETWORK_SECMARK


// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
