// SPDX-License-Identifier: GPL-2.0-or-later
/* Filesystem parameter parser.
 *
 * Copyright (C) 2018 Red Hat, Inc. All Rights Reserved.
 * Written by David Howells (dhowells@redhat.com)
 */

use core::ffi::c_char;

extern "C" {
    fn strcmp(a: *const c_char, b: *const c_char) -> i32;
    fn warn_plog(log: *mut p_log, fmt: *const c_char, ...);
    fn inval_plog(log: *mut p_log, fmt: *const c_char, ...) -> i32;
    fn invalf(fc: *mut fs_context, fmt: *const c_char, ...) -> i32;
    fn errorf(fc: *mut fs_context, fmt: *const c_char, ...);
    fn getname_kernel(name: *const c_char) -> *mut filename;
    fn is_err(ptr: *const filename) -> bool;
    fn ptr_err(ptr: *const filename) -> i32;
    fn filename_lookup(dirfd: i32, f: *mut filename, flags: u32,
                       path: *mut path, audit: *mut core::ffi::c_void) -> i32;
    fn path_put(path: *mut path);
    fn putname(f: *mut filename);
    fn d_backing_inode(dentry: *mut dentry) -> *mut inode;
    fn s_isblk(mode: u16) -> bool;
    fn kstrtouint(s: *const c_char, base: u32, result: *mut u32) -> i32;
    fn kstrtoint(s: *const c_char, base: u32, result: *mut i32) -> i32;
    fn kstrtoull(s: *const c_char, base: u32, result: *mut u64) -> i32;
    fn current_user_ns() -> *mut user_namespace;
    fn make_kuid(ns: *mut user_namespace, value: u32) -> kuid_t;
    fn make_kgid(ns: *mut user_namespace, value: u32) -> kgid_t;
    fn uid_valid(uid: kuid_t) -> bool;
    fn gid_valid(gid: kgid_t) -> bool;
    fn pr_err(fmt: *const c_char, ...);
}

static BOOL_NAMES: [constant_table; 7] = [
    constant_table { name: b"0\0".as_ptr() as *const c_char, value: false as i32 },
    constant_table { name: b"1\0".as_ptr() as *const c_char, value: true as i32 },
    constant_table { name: b"false\0".as_ptr() as *const c_char, value: false as i32 },
    constant_table { name: b"no\0".as_ptr() as *const c_char, value: false as i32 },
    constant_table { name: b"true\0".as_ptr() as *const c_char, value: true as i32 },
    constant_table { name: b"yes\0".as_ptr() as *const c_char, value: true as i32 },
    constant_table { name: core::ptr::null(), value: 0 },
];

unsafe fn __lookup_constant(tbl: *const constant_table, name: *const c_char) -> *const constant_table {
    let mut tbl = tbl;
    while !(*tbl).name.is_null() {
        if strcmp(name, (*tbl).name) == 0 {
            return tbl;
        }
        tbl = tbl.add(1);
    }
    core::ptr::null()
}

pub unsafe fn lookup_constant(tbl: *const constant_table, name: *const c_char, not_found: i32) -> i32 {
    let p = __lookup_constant(tbl, name);
    if !p.is_null() { (*p).value } else { not_found }
}

#[inline]
unsafe fn is_flag(p: *const fs_parameter_spec) -> bool {
    (*p).type_.is_none()
}

unsafe fn fs_lookup_key(desc: *const fs_parameter_spec, param: *mut fs_parameter,
                        negated: *mut bool) -> *const fs_parameter_spec {
    let mut p = desc;
    let mut other: *const fs_parameter_spec = core::ptr::null();
    let name = (*param).key;
    let want_flag = (*param).type_ == fs_value_is_flag;
    *negated = false;
    while !(*p).name.is_null() {
        if strcmp((*p).name, name) == 0 {
            if is_flag(p) == want_flag { return p; }
            other = p;
        }
        p = p.add(1);
    }
    if want_flag && *name as u8 == b'n' && *name.add(1) as u8 == b'o' && !*name.add(2).is_null() {
        p = desc;
        while !(*p).name.is_null() {
            if strcmp((*p).name, name.add(2)) == 0 && ((*p).flags & fs_param_neg_with_no) != 0 {
                *negated = true;
                return p;
            }
            p = p.add(1);
        }
    }
    other
}

pub unsafe fn __fs_parse(log: *mut p_log, desc: *const fs_parameter_spec,
                         param: *mut fs_parameter, result: *mut fs_parse_result) -> i32 {
    (*result).uint_64 = 0;
    let p = fs_lookup_key(desc, param, &mut (*result).negated);
    if p.is_null() { return -ENOPARAM; }
    if ((*p).flags & fs_param_deprecated) != 0 {
        warn_plog(log, b"Deprecated parameter '%s'\0".as_ptr() as *const c_char, (*param).key);
    }
    if is_flag(p) {
        if (*param).type_ != fs_value_is_flag {
            return inval_plog(log, b"Unexpected value for '%s'\0".as_ptr() as *const c_char, (*param).key);
        }
        (*result).boolean = !(*result).negated;
    } else {
        let ret = ((*p).type_)(log, p, param, result);
        if ret != 0 { return ret; }
    }
    (*p).opt
}

pub unsafe fn fs_lookup_param(fc: *mut fs_context, param: *mut fs_parameter,
                              want_bdev: bool, flags: u32, out_path: *mut path) -> i32 {
    let (f, put_f): (*mut filename, bool);
    match (*param).type_ {
        fs_value_is_string => {
            f = getname_kernel((*param).string);
            if is_err(f) { return ptr_err(f); }
            (*param).dirfd = AT_FDCWD;
            put_f = true;
        }
        fs_value_is_filename => { f = (*param).name; put_f = false; }
        _ => return invalf(fc, b"%s: not usable as path\0".as_ptr() as *const c_char, (*param).key),
    }
    let mut ret = filename_lookup((*param).dirfd, f, flags, out_path, core::ptr::null_mut());
    if ret < 0 {
        errorf(fc, b"%s: Lookup failure for '%s'\0".as_ptr() as *const c_char, (*param).key, (*f).name);
    } else if want_bdev && !s_isblk((*d_backing_inode((*out_path).dentry)).i_mode) {
        path_put(out_path); (*out_path).dentry = core::ptr::null_mut(); (*out_path).mnt = core::ptr::null_mut();
        errorf(fc, b"%s: Non-blockdev passed as '%s'\0".as_ptr() as *const c_char, (*param).key, (*f).name);
        ret = -ENOTBLK;
    }
    if put_f { putname(f); }
    ret
}

unsafe fn fs_param_bad_value(log: *mut p_log, param: *mut fs_parameter) -> i32 {
    inval_plog(log, b"Bad value for '%s'\0".as_ptr() as *const c_char, (*param).key)
}

pub unsafe fn fs_param_is_bool(log: *mut p_log, p: *const fs_parameter_spec, param: *mut fs_parameter, result: *mut fs_parse_result) -> i32 {
    if (*param).type_ != fs_value_is_string { return fs_param_bad_value(log, param); }
    if (*param).string.read() == 0 && ((*p).flags & fs_param_can_be_empty) != 0 { return 0; }
    let b = lookup_constant(BOOL_NAMES.as_ptr(), (*param).string, -1);
    if b == -1 { return fs_param_bad_value(log, param); }
    (*result).boolean = b != 0; 0
}

pub unsafe fn fs_param_is_u32(log: *mut p_log, p: *const fs_parameter_spec, param: *mut fs_parameter, result: *mut fs_parse_result) -> i32 {
    if (*param).type_ != fs_value_is_string || ((*param).string.read() == 0 && ((*p).flags & fs_param_can_be_empty) == 0) || kstrtouint((*param).string, (*p).data as usize as u32, &mut (*result).uint_32) < 0 { return fs_param_bad_value(log, param); } 0
}
pub unsafe fn fs_param_is_s32(log: *mut p_log, p: *const fs_parameter_spec, param: *mut fs_parameter, result: *mut fs_parse_result) -> i32 {
    if (*param).type_ != fs_value_is_string || ((*param).string.read() == 0 && ((*p).flags & fs_param_can_be_empty) == 0) || kstrtoint((*param).string, 0, &mut (*result).int_32) < 0 { return fs_param_bad_value(log, param); } 0
}
pub unsafe fn fs_param_is_u64(log: *mut p_log, p: *const fs_parameter_spec, param: *mut fs_parameter, result: *mut fs_parse_result) -> i32 {
    if (*param).type_ != fs_value_is_string || ((*param).string.read() == 0 && ((*p).flags & fs_param_can_be_empty) == 0) || kstrtoull((*param).string, 0, &mut (*result).uint_64) < 0 { return fs_param_bad_value(log, param); } 0
}
pub unsafe fn fs_param_is_enum(log: *mut p_log, p: *const fs_parameter_spec, param: *mut fs_parameter, result: *mut fs_parse_result) -> i32 {
    if (*param).type_ != fs_value_is_string || ((*param).string.read() == 0 && ((*p).flags & fs_param_can_be_empty) == 0) { return fs_param_bad_value(log, param); }
    let c = __lookup_constant((*p).data as *const constant_table, (*param).string); if c.is_null() { return fs_param_bad_value(log, param); } (*result).uint_32 = (*c).value as u32; 0
}
pub unsafe fn fs_param_is_string(log: *mut p_log, p: *const fs_parameter_spec, param: *mut fs_parameter, _result: *mut fs_parse_result) -> i32 {
    if (*param).type_ != fs_value_is_string || ((*param).string.read() == 0 && ((*p).flags & fs_param_can_be_empty) == 0) { return fs_param_bad_value(log, param); } 0
}
pub unsafe fn fs_param_is_fd(log: *mut p_log, p: *const fs_parameter_spec, param: *mut fs_parameter, result: *mut fs_parse_result) -> i32 {
    match (*param).type_ { fs_value_is_string => { if ((*param).string.read() != 0 || ((*p).flags & fs_param_can_be_empty) != 0) && kstrtouint((*param).string, 0, &mut (*result).uint_32) >= 0 && (*result).uint_32 <= INT_MAX as u32 { return 0; } }, fs_value_is_file => { (*result).uint_32 = (*param).dirfd as u32; if (*result).uint_32 <= INT_MAX as u32 { return 0; } }, _ => {} } fs_param_bad_value(log, param)
}
pub unsafe fn fs_param_is_file_or_string(log: *mut p_log, p: *const fs_parameter_spec, param: *mut fs_parameter, result: *mut fs_parse_result) -> i32 {
    match (*param).type_ { fs_value_is_string => fs_param_is_string(log, p, param, result), fs_value_is_file => { (*result).uint_32 = (*param).dirfd as u32; if (*result).uint_32 <= INT_MAX as u32 { 0 } else { fs_param_bad_value(log, param) } }, _ => fs_param_bad_value(log, param) }
}
pub unsafe fn fs_param_is_uid(log: *mut p_log, p: *const fs_parameter_spec, param: *mut fs_parameter, result: *mut fs_parse_result) -> i32 { if fs_param_is_u32(log, p, param, result) != 0 { return fs_param_bad_value(log, param); } let uid = make_kuid(current_user_ns(), (*result).uint_32); if !uid_valid(uid) { return inval_plog(log, b"Invalid uid '%s'\0".as_ptr() as *const c_char, (*param).string); } (*result).uid = uid; 0 }
pub unsafe fn fs_param_is_gid(log: *mut p_log, p: *const fs_parameter_spec, param: *mut fs_parameter, result: *mut fs_parse_result) -> i32 { if fs_param_is_u32(log, p, param, result) != 0 { return fs_param_bad_value(log, param); } let gid = make_kgid(current_user_ns(), (*result).uint_32); if !gid_valid(gid) { return inval_plog(log, b"Invalid gid '%s'\0".as_ptr() as *const c_char, (*param).string); } (*result).gid = gid; 0 }
pub unsafe fn fs_param_is_blockdev(_log: *mut p_log, _p: *const fs_parameter_spec, _param: *mut fs_parameter, _result: *mut fs_parse_result) -> i32 { 0 }

// CONFIG_VALIDATE_FS_PARSER: retain this conditional when enabled by the build.
#[cfg(CONFIG_VALIDATE_FS_PARSER)]
pub unsafe fn fs_validate_description(name: *const c_char,
                                      desc: *const fs_parameter_spec) -> bool {
    let mut good = true;
    let mut param = desc;
    while !(*param).name.is_null() {
        let mut p2 = desc;
        while p2 < param {
            if strcmp((*param).name, (*p2).name) == 0 {
                if is_flag(param) != is_flag(p2) {
                    p2 = p2.add(1);
                    continue;
                }
                pr_err(b"VALIDATE %s: PARAM[%s]: Duplicate\0".as_ptr() as *const c_char,
                       name, (*param).name);
                good = false;
            }
            p2 = p2.add(1);
        }
        param = param.add(1);
    }
    good
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
