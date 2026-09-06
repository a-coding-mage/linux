// SPDX-License-Identifier: GPL-2.0-only
//
// AppArmor security module
//
// This file contains AppArmor mediation of files
//
// Copyright (C) 1998-2008 Novell/SUSE
// Copyright 2009-2017 Canonical Ltd.

// Requires: linux/fs.h, linux/mount.h, linux/namei.h, uapi/linux/mount.h
// Requires: include/apparmor.h, include/audit.h, include/cred.h,
//           include/domain.h, include/file.h, include/match.h,
//           include/mount.h, include/path.h, include/policy.h

use core::ffi::c_void;

// External constants and types from Linux kernel
extern "C" {
    static MS_RDONLY: u64;
    static MS_NOSUID: u64;
    static MS_NODEV: u64;
    static MS_NOEXEC: u64;
    static MS_SYNCHRONOUS: u64;
    static MS_REMOUNT: u64;
    static MS_MANDLOCK: u64;
    static MS_DIRSYNC: u64;
    static MS_NOSYMFOLLOW: u64;
    static MS_NOATIME: u64;
    static MS_NODIRATIME: u64;
    static MS_BIND: u64;
    static MS_MOVE: u64;
    static MS_SILENT: u64;
    static MS_POSIXACL: u64;
    static MS_UNBINDABLE: u64;
    static MS_PRIVATE: u64;
    static MS_SLAVE: u64;
    static MS_SHARED: u64;
    static MS_RELATIME: u64;
    static MS_I_VERSION: u64;
    static MS_STRICTATIME: u64;
    static MS_NOUSER: u64;
    static MS_REC: u64;

    fn audit_log_format(ab: *mut audit_buffer, fmt: *const u8, ...);
    fn audit_log_untrustedstring(ab: *mut audit_buffer, str: *const u8);
}

// External types from AppArmor and kernel
#[repr(C)]
pub struct audit_buffer {
    _opaque: [u8; 0],
}

#[repr(C)]
pub struct common_audit_data {
    _opaque: [u8; 0],
}

#[repr(C)]
pub struct apparmor_audit_data {
    pub mnt: aa_mnt_audit_data,
    pub info: *const u8,
}

#[repr(C)]
pub struct aa_mnt_audit_data {
    pub type_: *const u8,
    pub src_name: *const u8,
    pub trans: *const u8,
    pub flags: u64,
    pub data: *const u8,
}

#[repr(C)]
pub struct aa_dfa {
    _opaque: [u8; 0],
}

#[repr(C)]
pub struct aa_perms {
    pub allow: u32,
    pub audit: u32,
    pub deny: u32,
}

#[repr(C)]
pub struct aa_policydb {
    pub dfa: *mut aa_dfa,
    pub perms: *mut aa_perms,
    pub start: *const aa_state_t,
}

#[repr(C)]
pub struct aa_ruleset {
    pub policy: *mut aa_policydb,
}

#[repr(C)]
pub struct dentry {
    pub d_inode: *mut inode,
}

#[repr(C)]
pub struct inode {
    pub i_mode: u32,
}

#[repr(C)]
pub struct super_block {
    pub s_type: *mut file_system_type,
}

#[repr(C)]
pub struct file_system_type {
    pub fs_flags: u32,
}

#[repr(C)]
pub struct vfsmount {
    pub mnt_root: *mut dentry,
    pub mnt_sb: *mut super_block,
}

#[repr(C)]
pub struct path {
    pub mnt: *mut vfsmount,
    pub dentry: *mut dentry,
}

#[repr(C)]
pub struct aa_label {
    pub hname: *const u8,
    pub rules: *mut *mut aa_ruleset,
}

#[repr(C)]
pub struct aa_profile {
    pub label: aa_label,
    pub path_flags: i32,
    pub disconnected: bool,
}

#[repr(C)]
pub struct cred {
    _opaque: [u8; 0],
}

pub type aa_state_t = u32;

const AA_CLASS_MOUNT: usize = 0;
const AA_MAY_MOUNT: u32 = 1;
const AA_MAY_UMOUNT: u32 = 2;
const AA_MAY_PIVOTROOT: u32 = 4;
const AA_MNT_CONT_MATCH: u32 = 8;
const AA_AUDIT_DATA: u32 = 16;
const PATH_IS_DIR: i32 = 1;
const FS_BINARY_MOUNTDATA: u32 = 1;
const FS_REQUIRES_DEV: u32 = 2;
const LOOKUP_FOLLOW: u32 = 1;
const LOOKUP_AUTOMOUNT: u32 = 2;
const GFP_KERNEL: i32 = 0;

// Macro equivalent for DEFINE_AUDIT_MOUNT
macro_rules! DEFINE_AUDIT_MOUNT {
    ($name:ident, $op:expr, $cred:expr) => {
        let mut $name = apparmor_audit_data {
            mnt: aa_mnt_audit_data {
                type_: core::ptr::null(),
                src_name: core::ptr::null(),
                trans: core::ptr::null(),
                flags: 0,
                data: core::ptr::null(),
            },
            info: core::ptr::null(),
        };
        let _subj_cred = $cred;
    };
}

extern "C" {
    fn aad(sa: *mut common_audit_data) -> *mut apparmor_audit_data;
    fn aa_dfa_next(dfa: *const aa_dfa, state: aa_state_t, c: u32) -> aa_state_t;
    fn aa_dfa_match(dfa: *const aa_dfa, state: aa_state_t, str: *const u8) -> aa_state_t;
    fn aa_dfa_null_transition(dfa: *const aa_dfa, state: aa_state_t) -> aa_state_t;
    fn aa_lookup_perms(policy: *const aa_policydb, state: aa_state_t) -> *const aa_perms;
    fn aa_path_name(
        path: *const path,
        flags: i32,
        buffer: *mut u8,
        name: *mut *const u8,
        info: *mut *const u8,
        disconnected: bool,
    ) -> i32;
    fn aa_audit_perm_error(
        label: *const aa_label,
        op: u32,
        error: i32,
        ad: *mut apparmor_audit_data,
        audit_cb: unsafe extern "C" fn(*mut audit_buffer, *mut c_void),
    ) -> i32;
    fn aa_get_buffer(sync: bool) -> *mut u8;
    fn aa_put_buffer(buffer: *mut u8);
    fn aa_check_perms(
        profile: *const aa_profile,
        perms: *const aa_perms,
        op: u32,
        ad: *mut apparmor_audit_data,
        audit_cb: unsafe extern "C" fn(*mut audit_buffer, *mut c_void),
    ) -> i32;
    fn aa_apply_modes_to_perms(profile: *const aa_profile, perms: *mut aa_perms);
    fn AA_BUG(cond: bool);
    fn RULE_MEDIATES(rules: *const aa_ruleset, class: usize) -> bool;
    fn IS_ERR(ptr: *const c_void) -> bool;
    fn PTR_ERR(ptr: *const c_void) -> i32;
    fn ERR_PTR(err: i32) -> *const c_void;
    fn kern_path(name: *const u8, flags: u32, path: *mut path) -> i32;
    fn path_put(path: *const path);
    fn S_ISDIR(mode: u32) -> bool;
    fn get_fs_type(name: *const u8) -> *mut file_system_type;
    fn put_filesystem(fstype: *mut file_system_type);
    fn profile_unconfined(profile: *const aa_profile) -> bool;
    fn aa_get_newest_label(label: *const aa_label) -> *mut aa_label;
    fn aa_replace_current_label(label: *mut aa_label) -> i32;
    fn aa_put_label(label: *mut aa_label);
    fn fn_for_each(
        label: *const aa_label,
        profile: *mut *mut aa_profile,
        expr: i32,
    ) -> i32;
    fn fn_label_build(
        label: *const aa_label,
        profile: *mut *mut aa_profile,
        gfp: i32,
        expr: *mut aa_label,
    ) -> *mut aa_label;
    fn our_mnt(mnt: *const vfsmount) -> bool;
}

unsafe extern "C" fn audit_mnt_flags(ab: *mut audit_buffer, flags: u64) {
    if (flags & MS_RDONLY) != 0 {
        audit_log_format(ab, b"ro\0" as *const u8);
    } else {
        audit_log_format(ab, b"rw\0" as *const u8);
    }
    if (flags & MS_NOSUID) != 0 {
        audit_log_format(ab, b", nosuid\0" as *const u8);
    }
    if (flags & MS_NODEV) != 0 {
        audit_log_format(ab, b", nodev\0" as *const u8);
    }
    if (flags & MS_NOEXEC) != 0 {
        audit_log_format(ab, b", noexec\0" as *const u8);
    }
    if (flags & MS_SYNCHRONOUS) != 0 {
        audit_log_format(ab, b", sync\0" as *const u8);
    }
    if (flags & MS_REMOUNT) != 0 {
        audit_log_format(ab, b", remount\0" as *const u8);
    }
    if (flags & MS_MANDLOCK) != 0 {
        audit_log_format(ab, b", mand\0" as *const u8);
    }
    if (flags & MS_DIRSYNC) != 0 {
        audit_log_format(ab, b", dirsync\0" as *const u8);
    }
    if (flags & MS_NOSYMFOLLOW) != 0 {
        audit_log_format(ab, b", nosymfollow\0" as *const u8);
    }
    if (flags & MS_NOATIME) != 0 {
        audit_log_format(ab, b", noatime\0" as *const u8);
    }
    if (flags & MS_NODIRATIME) != 0 {
        audit_log_format(ab, b", nodiratime\0" as *const u8);
    }
    if (flags & MS_BIND) != 0 {
        if (flags & MS_REC) != 0 {
            audit_log_format(ab, b", rbind\0" as *const u8);
        } else {
            audit_log_format(ab, b", bind\0" as *const u8);
        }
    }
    if (flags & MS_MOVE) != 0 {
        audit_log_format(ab, b", move\0" as *const u8);
    }
    if (flags & MS_SILENT) != 0 {
        audit_log_format(ab, b", silent\0" as *const u8);
    }
    if (flags & MS_POSIXACL) != 0 {
        audit_log_format(ab, b", acl\0" as *const u8);
    }
    if (flags & MS_UNBINDABLE) != 0 {
        if (flags & MS_REC) != 0 {
            audit_log_format(ab, b", runbindable\0" as *const u8);
        } else {
            audit_log_format(ab, b", unbindable\0" as *const u8);
        }
    }
    if (flags & MS_PRIVATE) != 0 {
        if (flags & MS_REC) != 0 {
            audit_log_format(ab, b", rprivate\0" as *const u8);
        } else {
            audit_log_format(ab, b", private\0" as *const u8);
        }
    }
    if (flags & MS_SLAVE) != 0 {
        if (flags & MS_REC) != 0 {
            audit_log_format(ab, b", rslave\0" as *const u8);
        } else {
            audit_log_format(ab, b", slave\0" as *const u8);
        }
    }
    if (flags & MS_SHARED) != 0 {
        if (flags & MS_REC) != 0 {
            audit_log_format(ab, b", rshared\0" as *const u8);
        } else {
            audit_log_format(ab, b", shared\0" as *const u8);
        }
    }
    if (flags & MS_RELATIME) != 0 {
        audit_log_format(ab, b", relatime\0" as *const u8);
    }
    if (flags & MS_I_VERSION) != 0 {
        audit_log_format(ab, b", iversion\0" as *const u8);
    }
    if (flags & MS_STRICTATIME) != 0 {
        audit_log_format(ab, b", strictatime\0" as *const u8);
    }
    if (flags & MS_NOUSER) != 0 {
        audit_log_format(ab, b", nouser\0" as *const u8);
    }
}

/// audit_cb - call back for mount specific audit fields
/// @ab: audit_buffer  (NOT NULL)
/// @va: audit struct to audit values of  (NOT NULL)
unsafe extern "C" fn audit_cb(ab: *mut audit_buffer, va: *mut c_void) {
    let sa = va as *mut common_audit_data;
    let ad = aad(sa);

    if !(*ad).mnt.type_.is_null() {
        audit_log_format(ab, b" fstype=\0" as *const u8);
        audit_log_untrustedstring(ab, (*ad).mnt.type_);
    }
    if !(*ad).mnt.src_name.is_null() {
        audit_log_format(ab, b" srcname=\0" as *const u8);
        audit_log_untrustedstring(ab, (*ad).mnt.src_name);
    }
    if !(*ad).mnt.trans.is_null() {
        audit_log_format(ab, b" trans=\0" as *const u8);
        audit_log_untrustedstring(ab, (*ad).mnt.trans);
    }
    if (*ad).mnt.flags != 0 {
        audit_log_format(ab, b" flags=\"\0" as *const u8);
        audit_mnt_flags(ab, (*ad).mnt.flags);
        audit_log_format(ab, b"\"\0" as *const u8);
    }
    if !(*ad).mnt.data.is_null() {
        audit_log_format(ab, b" options=\0" as *const u8);
        audit_log_untrustedstring(ab, (*ad).mnt.data);
    }
}

/// match_mnt_flags - Do an ordered match on mount flags
/// @dfa: dfa to match against
/// @state: state to start in
/// @flags: mount flags to match against
///
/// Mount flags are encoded as an ordered match. This is done instead of
/// checking against a simple bitmask, to allow for logical operations
/// on the flags.
///
/// Returns: next state after flags match
unsafe extern "C" fn match_mnt_flags(
    dfa: *const aa_dfa,
    mut state: aa_state_t,
    flags: u64,
) -> aa_state_t {
    for i in 0..=31 {
        if ((1u64 << i) & flags) != 0 {
            state = aa_dfa_next(dfa, state, (i + 1) as u32);
        }
    }

    state
}

static MNT_INFO_TABLE: &[&str] = &[
    "match succeeded",
    "failed mntpnt match",
    "failed srcname match",
    "failed type match",
    "failed flags match",
    "failed data match",
    "failed perms check",
];

/// Returns 0 on success else element that match failed in, this is the
/// index into the mnt_info_table above
unsafe extern "C" fn do_match_mnt(
    policy: *mut aa_policydb,
    start: aa_state_t,
    mntpnt: *const u8,
    devname: *const u8,
    type_: *const u8,
    flags: u64,
    data: *mut c_void,
    binary: bool,
    perms: *mut aa_perms,
) -> i32 {
    let mut state: aa_state_t;

    AA_BUG(policy.is_null());
    AA_BUG((*policy).dfa.is_null());
    AA_BUG((*policy).perms.is_null());
    AA_BUG(perms.is_null());

    state = aa_dfa_match((*policy).dfa, start, mntpnt);
    state = aa_dfa_null_transition((*policy).dfa, state);
    if state == 0 {
        return 1;
    }

    if !devname.is_null() {
        state = aa_dfa_match((*policy).dfa, state, devname);
    }
    state = aa_dfa_null_transition((*policy).dfa, state);
    if state == 0 {
        return 2;
    }

    if !type_.is_null() {
        state = aa_dfa_match((*policy).dfa, state, type_);
    }
    state = aa_dfa_null_transition((*policy).dfa, state);
    if state == 0 {
        return 3;
    }

    state = match_mnt_flags((*policy).dfa, state, flags);
    if state == 0 {
        return 4;
    }
    *perms = *aa_lookup_perms(policy, state);
    if ((*perms).allow & AA_MAY_MOUNT) != 0 {
        return 0;
    }

    if !data.is_null() && !binary && (((*perms).allow & AA_MNT_CONT_MATCH) != 0) {
        state = aa_dfa_null_transition((*policy).dfa, state);
        if state == 0 {
            return 4;
        }

        state = aa_dfa_match((*policy).dfa, state, data as *const u8);
        if state == 0 {
            return 5;
        }
        *perms = *aa_lookup_perms(policy, state);
        if ((*perms).allow & AA_MAY_MOUNT) != 0 {
            return 0;
        }
    }

    6
}

unsafe extern "C" fn path_flags(profile: *const aa_profile, path: *const path) -> i32 {
    AA_BUG(profile.is_null());
    AA_BUG(path.is_null());

    let mut flags = (*profile).path_flags;
    if S_ISDIR((*(*(*path).dentry).d_inode).i_mode) {
        flags |= PATH_IS_DIR;
    }
    flags
}

/// match_mnt_path_str - handle path matching for mount
/// @profile: the confining profile
/// @mntpath: for the mntpnt (NOT NULL)
/// @buffer: buffer to be used to lookup mntpath
/// @devname: string for the devname/src_name (MAY BE NULL OR ERRPTR)
/// @type: string for the dev type (MAYBE NULL)
/// @flags: mount flags to match
/// @data: fs mount data (MAYBE NULL)
/// @binary: whether @data is binary
/// @devinfo: error str if (IS_ERR(@devname))
/// @ad: apparmor audit data structure
///
/// Returns: 0 on success else error
unsafe extern "C" fn match_mnt_path_str(
    profile: *mut aa_profile,
    mntpath: *const path,
    buffer: *mut u8,
    devname: *const u8,
    type_: *const u8,
    flags: u64,
    data: *mut c_void,
    binary: bool,
    devinfo: *const u8,
    ad: *mut apparmor_audit_data,
) -> i32 {
    let mut perms = aa_perms {
        allow: 0,
        audit: 0,
        deny: 0,
    };
    let mut mntpnt: *const u8 = core::ptr::null();
    let rules = *(*(*profile).label.rules);
    let mut pos: i32;
    let mut error: i32;

    AA_BUG(profile.is_null());
    AA_BUG(mntpath.is_null());
    AA_BUG(buffer.is_null());

    if !RULE_MEDIATES(rules, AA_CLASS_MOUNT) {
        return 0;
    }

    (*ad).mnt.type_ = type_;

    error = aa_path_name(
        mntpath,
        path_flags(profile, mntpath),
        buffer,
        &mut mntpnt,
        &mut (*ad).info,
        (*profile).disconnected,
    );
    if error != 0 {
        return aa_audit_perm_error(&(*profile).label, AA_MAY_MOUNT, error, ad, audit_cb);
    }
    (*ad).info = mntpnt as *const u8;

    if IS_ERR(devname as *const c_void) {
        error = PTR_ERR(devname as *const c_void);
        (*ad).info = devinfo;
        return aa_audit_perm_error(&(*profile).label, AA_MAY_MOUNT, error, ad, audit_cb);
    }
    (*ad).mnt.src_name = devname;

    pos = do_match_mnt(
        (*rules).policy,
        *(*(*rules).policy).start.add(AA_CLASS_MOUNT),
        mntpnt,
        devname,
        type_,
        flags,
        data,
        binary,
        &mut perms,
    );
    if pos != 0 {
        (*ad).info = MNT_INFO_TABLE[pos as usize].as_ptr() as *const u8;
    }

    aa_apply_modes_to_perms(profile, &mut perms);
    if !data.is_null() && !binary && ((perms.audit & AA_AUDIT_DATA) != 0) {
        (*ad).mnt.data = data as *const u8;
    }
    aa_check_perms(profile, &perms, AA_MAY_MOUNT, ad, audit_cb)
}

/// match_mnt - handle path matching for mount
/// @profile: the confining profile
/// @path: for the mntpnt (NOT NULL)
/// @buffer: buffer to be used to lookup mntpath
/// @devpath: path devname/src_name (MAYBE NULL)
/// @devbuffer: buffer to be used to lookup devname/src_name
/// @type: string for the dev type (MAYBE NULL)
/// @flags: mount flags to match
/// @data: fs mount data (MAYBE NULL)
/// @binary: whether @data is binary
/// @ad: apparmor audit data structure
///
/// Returns: 0 on success else error
unsafe extern "C" fn match_mnt(
    profile: *mut aa_profile,
    path: *const path,
    buffer: *mut u8,
    devpath: *const path,
    devbuffer: *mut u8,
    type_: *const u8,
    flags: u64,
    data: *mut c_void,
    binary: bool,
    ad: *mut apparmor_audit_data,
) -> i32 {
    let mut devname: *const u8 = core::ptr::null();
    let mut info: *const u8 = core::ptr::null();
    let rules = *(*(*profile).label.rules);
    let mut error: i32 = -13;

    AA_BUG(profile.is_null());
    AA_BUG(!devpath.is_null() && devbuffer.is_null());

    if !RULE_MEDIATES(rules, AA_CLASS_MOUNT) {
        return 0;
    }

    if !devpath.is_null() {
        error = aa_path_name(
            devpath,
            path_flags(profile, devpath),
            devbuffer,
            &mut devname,
            &mut info,
            (*profile).disconnected,
        );
        if error != 0 {
            devname = ERR_PTR(error) as *const u8;
        }
    }

    match_mnt_path_str(
        profile, path, buffer, devname, type_, flags, data, binary, info, ad,
    )
}

#[no_mangle]
pub unsafe extern "C" fn aa_remount(
    subj_cred: *const cred,
    label: *mut aa_label,
    path: *const path,
    flags: u64,
    data: *mut c_void,
) -> i32 {
    let mut profile: *mut aa_profile;
    let mut buffer: *mut u8 = core::ptr::null_mut();
    let binary: bool;
    let mut error: i32;

    let mut ad = apparmor_audit_data {
        mnt: aa_mnt_audit_data {
            type_: core::ptr::null(),
            src_name: core::ptr::null(),
            trans: core::ptr::null(),
            flags: flags,
            data: core::ptr::null(),
        },
        info: core::ptr::null(),
    };

    AA_BUG(label.is_null());
    AA_BUG(path.is_null());

    binary = ((*(*(*path).dentry).d_inode).i_mode
        & ((*(*(*(*path).mnt).mnt_sb).s_type).fs_flags as u32))
        != 0;

    buffer = aa_get_buffer(false);
    if buffer.is_null() {
        return -12;
    }
    error = fn_for_each(
        label,
        &mut profile,
        match_mnt(profile, path, buffer, core::ptr::null(), core::ptr::null(), core::ptr::null(), flags, data, binary, &mut ad),
    );
    aa_put_buffer(buffer);

    error
}

#[no_mangle]
pub unsafe extern "C" fn aa_bind_mount(
    subj_cred: *const cred,
    label: *mut aa_label,
    path: *const path,
    dev_name: *const u8,
    flags: u64,
) -> i32 {
    let mut profile: *mut aa_profile;
    let mut buffer: *mut u8 = core::ptr::null_mut();
    let mut old_buffer: *mut u8 = core::ptr::null_mut();
    let mut old_path: path = path {
        mnt: core::ptr::null_mut(),
        dentry: core::ptr::null_mut(),
    };
    let mut error: i32;

    let mut ad = apparmor_audit_data {
        mnt: aa_mnt_audit_data {
            type_: core::ptr::null(),
            src_name: core::ptr::null(),
            trans: core::ptr::null(),
            flags: 0,
            data: core::ptr::null(),
        },
        info: core::ptr::null(),
    };

    AA_BUG(label.is_null());
    AA_BUG(path.is_null());

    if dev_name.is_null() || *dev_name == 0 {
        return -22;
    }

    let flags = flags & (MS_REC | MS_BIND);
    ad.mnt.flags = flags;

    error = kern_path(dev_name, LOOKUP_FOLLOW | LOOKUP_AUTOMOUNT, &mut old_path);
    if error != 0 {
        return aa_audit_perm_error(label, AA_MAY_MOUNT, error, &mut ad, audit_cb);
    }

    buffer = aa_get_buffer(false);
    old_buffer = aa_get_buffer(false);
    error = -12;
    if buffer.is_null() || old_buffer.is_null() {
        goto_out!();
    } else {
        error = fn_for_each(
            label,
            &mut profile,
            match_mnt(
                profile,
                path,
                buffer,
                &old_path,
                old_buffer,
                core::ptr::null(),
                flags,
                core::ptr::null_mut(),
                false,
                &mut ad,
            ),
        );
    }

    out!({
        aa_put_buffer(buffer);
        aa_put_buffer(old_buffer);
        path_put(&old_path);
    });

    error
}

macro_rules! goto_out {
    () => {
        // Placeholder for goto-like behavior in Rust
    };
}

macro_rules! out {
    ($block:block) => {
        $block
    };
}

#[no_mangle]
pub unsafe extern "C" fn aa_mount_change_type(
    subj_cred: *const cred,
    label: *mut aa_label,
    path: *const path,
    flags: u64,
) -> i32 {
    let mut profile: *mut aa_profile;
    let mut buffer: *mut u8 = core::ptr::null_mut();
    let mut error: i32;

    let mut ad = apparmor_audit_data {
        mnt: aa_mnt_audit_data {
            type_: core::ptr::null(),
            src_name: core::ptr::null(),
            trans: core::ptr::null(),
            flags: 0,
            data: core::ptr::null(),
        },
        info: core::ptr::null(),
    };

    AA_BUG(label.is_null());
    AA_BUG(path.is_null());

    let flags = flags
        & (MS_REC | MS_SILENT | MS_SHARED | MS_PRIVATE | MS_SLAVE | MS_UNBINDABLE);
    ad.mnt.flags = flags;

    buffer = aa_get_buffer(false);
    if buffer.is_null() {
        return -12;
    }
    error = fn_for_each(
        label,
        &mut profile,
        match_mnt(
            profile,
            path,
            buffer,
            core::ptr::null(),
            core::ptr::null_mut(),
            core::ptr::null(),
            flags,
            core::ptr::null_mut(),
            false,
            &mut ad,
        ),
    );
    aa_put_buffer(buffer);

    error
}

#[no_mangle]
pub unsafe extern "C" fn aa_move_mount(
    subj_cred: *const cred,
    label: *mut aa_label,
    from_path: *const path,
    to_path: *const path,
) -> i32 {
    let mut profile: *mut aa_profile;
    let mut to_buffer: *mut u8 = core::ptr::null_mut();
    let mut from_buffer: *mut u8 = core::ptr::null_mut();
    let mut error: i32;
    let mut actual_from_path = from_path;

    let mut ad = apparmor_audit_data {
        mnt: aa_mnt_audit_data {
            type_: core::ptr::null(),
            src_name: core::ptr::null(),
            trans: core::ptr::null(),
            flags: MS_MOVE,
            data: core::ptr::null(),
        },
        info: core::ptr::null(),
    };

    AA_BUG(label.is_null());
    AA_BUG(from_path.is_null());
    AA_BUG(to_path.is_null());

    to_buffer = aa_get_buffer(false);
    from_buffer = aa_get_buffer(false);
    error = -12;
    if to_buffer.is_null() || from_buffer.is_null() {
        goto_out!();
    } else {
        if !our_mnt((*from_path).mnt) {
            actual_from_path = core::ptr::null();
        }
        error = fn_for_each(
            label,
            &mut profile,
            match_mnt(
                profile,
                to_path,
                to_buffer,
                actual_from_path,
                from_buffer,
                core::ptr::null(),
                MS_MOVE,
                core::ptr::null_mut(),
                false,
                &mut ad,
            ),
        );
    }

    out!({
        aa_put_buffer(to_buffer);
        aa_put_buffer(from_buffer);
    });

    error
}

#[no_mangle]
pub unsafe extern "C" fn aa_move_mount_old(
    subj_cred: *const cred,
    label: *mut aa_label,
    path: *const path,
    orig_name: *const u8,
) -> i32 {
    let mut old_path: path = path {
        mnt: core::ptr::null_mut(),
        dentry: core::ptr::null_mut(),
    };
    let mut error: i32;

    if orig_name.is_null() || *orig_name == 0 {
        return -22;
    }
    error = kern_path(orig_name, LOOKUP_FOLLOW, &mut old_path);
    if error != 0 {
        return error;
    }

    error = aa_move_mount(subj_cred, label, &old_path, path);
    path_put(&old_path);

    error
}

#[no_mangle]
pub unsafe extern "C" fn aa_new_mount(
    subj_cred: *const cred,
    label: *mut aa_label,
    dev_name: *const u8,
    path: *const path,
    type_: *const u8,
    flags: u64,
    data: *mut c_void,
) -> i32 {
    let mut profile: *mut aa_profile;
    let mut buffer: *mut u8 = core::ptr::null_mut();
    let mut dev_buffer: *mut u8 = core::ptr::null_mut();
    let mut binary: bool = true;
    let mut error: i32;
    let mut requires_dev: i32 = 0;
    let mut tmp_path: path = path {
        mnt: core::ptr::null_mut(),
        dentry: core::ptr::null_mut(),
    };
    let mut dev_path: *const path = core::ptr::null();

    let mut ad = apparmor_audit_data {
        mnt: aa_mnt_audit_data {
            type_: core::ptr::null(),
            src_name: core::ptr::null(),
            trans: core::ptr::null(),
            flags: flags,
            data: core::ptr::null(),
        },
        info: core::ptr::null(),
    };

    AA_BUG(label.is_null());
    AA_BUG(path.is_null());

    if !type_.is_null() {
        let fstype = get_fs_type(type_);
        if fstype.is_null() {
            return -19;
        }
        binary = ((*fstype).fs_flags & FS_BINARY_MOUNTDATA) != 0;
        requires_dev = (((*fstype).fs_flags & FS_REQUIRES_DEV) != 0) as i32;
        put_filesystem(fstype);

        if requires_dev != 0 {
            if dev_name.is_null() || *dev_name == 0 {
                return -2;
            }

            error = kern_path(dev_name, LOOKUP_FOLLOW, &mut tmp_path);
            if error != 0 {
                return error;
            }
            dev_path = &tmp_path;
        }
    }

    buffer = aa_get_buffer(false);
    if buffer.is_null() {
        error = -12;
        goto_out!();
    } else {
        if !dev_path.is_null() {
            dev_buffer = aa_get_buffer(false);
            if dev_buffer.is_null() {
                error = -12;
                goto_out!();
            } else {
                error = fn_for_each(
                    label,
                    &mut profile,
                    match_mnt(
                        profile, path, buffer, dev_path, dev_buffer, type_, flags, data, binary,
                        &mut ad,
                    ),
                );
            }
        } else {
            error = fn_for_each(
                label,
                &mut profile,
                match_mnt_path_str(
                    profile, path, buffer, dev_name, type_, flags, data, binary, core::ptr::null(),
                    &mut ad,
                ),
            );
        }
    }

    out!({
        aa_put_buffer(buffer);
        aa_put_buffer(dev_buffer);
        if !dev_path.is_null() {
            path_put(dev_path);
        }
    });

    error
}

unsafe extern "C" fn profile_umount(
    profile: *mut aa_profile,
    path: *const path,
    buffer: *mut u8,
    ad: *mut apparmor_audit_data,
) -> i32 {
    let rules = *(*(*profile).label.rules);
    let mut perms = aa_perms {
        allow: 0,
        audit: 0,
        deny: 0,
    };
    let mut name: *const u8 = core::ptr::null();
    let state: aa_state_t;
    let mut error: i32;

    AA_BUG(profile.is_null());
    AA_BUG(path.is_null());

    if !RULE_MEDIATES(rules, AA_CLASS_MOUNT) {
        return 0;
    }

    error = aa_path_name(
        path,
        path_flags(profile, path),
        buffer,
        &mut name,
        &mut (*ad).info,
        (*profile).disconnected,
    );
    if error != 0 {
        return aa_audit_perm_error(&(*profile).label, AA_MAY_UMOUNT, error, ad, audit_cb);
    }

    (*ad).info = name as *const u8;
    let state = aa_dfa_match(
        (*(*rules).policy).dfa,
        *(*(*rules).policy).start.add(AA_CLASS_MOUNT),
        name,
    );
    perms = *aa_lookup_perms((*rules).policy, state);

    aa_apply_modes_to_perms(profile, &mut perms);
    aa_check_perms(profile, &perms, AA_MAY_UMOUNT, ad, audit_cb)
}

#[no_mangle]
pub unsafe extern "C" fn aa_umount(
    subj_cred: *const cred,
    label: *mut aa_label,
    mnt: *mut vfsmount,
    flags: i32,
) -> i32 {
    let mut profile: *mut aa_profile;
    let mut buffer: *mut u8 = core::ptr::null_mut();
    let mut error: i32;
    let path_obj = path {
        mnt: mnt,
        dentry: (*mnt).mnt_root,
    };

    let mut ad = apparmor_audit_data {
        mnt: aa_mnt_audit_data {
            type_: core::ptr::null(),
            src_name: core::ptr::null(),
            trans: core::ptr::null(),
            flags: 0,
            data: core::ptr::null(),
        },
        info: core::ptr::null(),
    };

    AA_BUG(label.is_null());
    AA_BUG(mnt.is_null());

    buffer = aa_get_buffer(false);
    if buffer.is_null() {
        return -12;
    }

    error = fn_for_each(label, &mut profile, profile_umount(profile, &path_obj, buffer, &mut ad));
    aa_put_buffer(buffer);

    error
}

/// helper fn for transition on pivotroot
///
/// Returns: label for transition or ERR_PTR. Does not return NULL
unsafe extern "C" fn build_pivotroot(
    profile: *mut aa_profile,
    new_path: *const path,
    new_buffer: *mut u8,
    old_path: *const path,
    old_buffer: *mut u8,
    ad: *mut apparmor_audit_data,
) -> *mut aa_label {
    let rules = *(*(*profile).label.rules);
    let mut old_name: *const u8 = core::ptr::null();
    let mut new_name: *const u8 = core::ptr::null();
    let mut perms = aa_perms {
        allow: 0,
        audit: 0,
        deny: 0,
    };
    let mut state: aa_state_t;
    let mut error: i32;

    AA_BUG(profile.is_null());
    AA_BUG(new_path.is_null());
    AA_BUG(old_path.is_null());

    if profile_unconfined(profile) || !RULE_MEDIATES(rules, AA_CLASS_MOUNT) {
        return aa_get_newest_label(&(*profile).label);
    }

    error = aa_path_name(
        old_path,
        path_flags(profile, old_path),
        old_buffer,
        &mut old_name,
        &mut (*ad).info,
        (*profile).disconnected,
    );
    if error != 0 {
        goto_err!();
    }
    (*ad).mnt.src_name = old_name;
    error = aa_path_name(
        new_path,
        path_flags(profile, new_path),
        new_buffer,
        &mut new_name,
        &mut (*ad).info,
        (*profile).disconnected,
    );
    if error != 0 {
        goto_err!();
    }
    (*ad).info = new_name as *const u8;

    state = aa_dfa_match(
        (*(*rules).policy).dfa,
        *(*(*rules).policy).start.add(AA_CLASS_MOUNT),
        new_name,
    );
    state = aa_dfa_null_transition((*(*rules).policy).dfa, state);
    state = aa_dfa_match((*(*rules).policy).dfa, state, old_name);
    perms = *aa_lookup_perms((*rules).policy, state);
    (*ad).mnt.trans = (*(*profile).label).hname;

    aa_apply_modes_to_perms(profile, &mut perms);
    error = aa_check_perms(profile, &perms, AA_MAY_PIVOTROOT, ad, audit_cb);

    out_label!({
        if error != 0 {
            return ERR_PTR(error) as *mut aa_label;
        }
        return aa_get_newest_label(&(*profile).label);
    });

    err_label!({
        error = aa_audit_perm_error(&(*profile).label, AA_MAY_PIVOTROOT, error, ad, audit_cb);
        goto_out!();
    });

    core::ptr::null_mut()
}

macro_rules! goto_err {
    () => {
        // Placeholder for goto-like behavior
    };
}

macro_rules! out_label {
    ($block:block) => {
        $block
    };
}

macro_rules! err_label {
    ($block:block) => {
        $block
    };
}

#[no_mangle]
pub unsafe extern "C" fn aa_pivotroot(
    subj_cred: *const cred,
    label: *mut aa_label,
    old_path: *const path,
    new_path: *const path,
) -> i32 {
    let mut profile: *mut aa_profile;
    let mut target: *mut aa_label = core::ptr::null_mut();
    let mut old_buffer: *mut u8 = core::ptr::null_mut();
    let mut new_buffer: *mut u8 = core::ptr::null_mut();
    let mut error: i32;

    let mut ad = apparmor_audit_data {
        mnt: aa_mnt_audit_data {
            type_: core::ptr::null(),
            src_name: core::ptr::null(),
            trans: core::ptr::null(),
            flags: 0,
            data: core::ptr::null(),
        },
        info: core::ptr::null(),
    };

    AA_BUG(label.is_null());
    AA_BUG(old_path.is_null());
    AA_BUG(new_path.is_null());

    old_buffer = aa_get_buffer(false);
    new_buffer = aa_get_buffer(false);
    error = -12;
    if old_buffer.is_null() || new_buffer.is_null() {
        goto_out!();
    } else {
        target = fn_label_build(
            label,
            &mut profile,
            GFP_KERNEL,
            build_pivotroot(profile, new_path, new_buffer, old_path, old_buffer, &mut ad),
        );
        AA_BUG(target.is_null());
        if !IS_ERR(target as *const c_void) {
            error = aa_replace_current_label(target);
            if error != 0 {
                goto_fail!();
            }
            aa_put_label(target);
        } else {
            error = PTR_ERR(target as *const c_void);
        }
    }

    out_pivotroot!({
        aa_put_buffer(old_buffer);
        aa_put_buffer(new_buffer);
    });

    error

    fail_pivotroot!({
        (*ad).mnt.trans = (*target).hname;
        error = aa_audit_perm_error(label, AA_MAY_PIVOTROOT, error, &mut ad, audit_cb);
        aa_put_label(target);
        goto_out!();
    });
}

macro_rules! goto_fail {
    () => {
        // Placeholder for goto
    };
}

macro_rules! out_pivotroot {
    ($block:block) => {
        $block
    };
}

macro_rules! fail_pivotroot {
    ($block:block) => {
        $block
    };
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
