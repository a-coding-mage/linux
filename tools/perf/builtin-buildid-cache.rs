// SPDX-License-Identifier: GPL-2.0
/*
 * builtin-buildid-cache.rs
 *
 * Builtin buildid-cache command: Manages build-id cache
 *
 * Copyright (C) 2010, Red Hat Inc.
 * Copyright (C) 2010, Arnaldo Carvalho de Melo <acme@redhat.com>
 */

use core::ffi::{c_char, c_int, c_uint, c_ulong, c_void};

type size_t = usize;
type u64 = u64;

const PATH_MAX: usize = 4096;
const SBUILD_ID_SIZE: usize = 128;
const STRERR_BUFSIZE: usize = 128;
const DT_DIR: u8 = 4;
const EEXIST: c_int = 17;
const ENOENT: c_int = 2;
const EINVAL: c_int = 22;
const ENOMEM: c_int = 12;
const PERF_DATA_MODE_READ: c_int = 0;

#[repr(C)]
pub struct DIR {
    _private: [u8; 0],
}

#[repr(C)]
pub struct FILE {
    _private: [u8; 0],
}

#[repr(C)]
pub struct dirent {
    pub d_ino: c_ulong,
    pub d_off: c_ulong,
    pub d_reclen: u16,
    pub d_type: u8,
    pub d_name: [c_char; 256],
}

#[repr(C)]
pub struct nsinfo {
    _private: [u8; 0],
}

#[repr(C)]
pub struct nscookie {
    _private: [u8; 0],
}

#[repr(C)]
pub struct strlist {
    _private: [u8; 0],
}

#[repr(C)]
pub struct str_node {
    pub s: *mut c_char,
}

#[repr(C)]
pub struct dso {
    _private: [u8; 0],
}

#[repr(C)]
pub struct perf_session {
    _private: [u8; 0],
}

#[repr(C)]
pub struct perf_env {
    _private: [u8; 0],
}

#[repr(C)]
pub struct build_id {
    pub size: c_int,
    pub data: [u8; SBUILD_ID_SIZE],
}

#[repr(C)]
pub struct perf_debuginfod {
    pub urls: *mut c_char,
    pub set: bool,
}

#[repr(C)]
pub struct perf_data {
    pub mode: c_int,
    pub path: *const c_char,
    pub force: bool,
}

#[repr(C)]
pub struct option {
    _private: [u8; 0],
}

unsafe extern "C" {
    static mut errno: c_int;
    static mut stdout: *mut FILE;
    static mut verbose: c_int;
    static buildid_dir: *const c_char;
    static DSO__NAME_KCORE: *const c_char;
    static ref_reloc_sym_names: [*const c_char; 0];

    fn strlcpy(dst: *mut c_char, src: *const c_char, size: size_t) -> size_t;
    fn strrchr(s: *const c_char, c: c_int) -> *mut c_char;
    fn strcmp(s1: *const c_char, s2: *const c_char) -> c_int;
    fn strdup(s: *const c_char) -> *mut c_char;
    fn memcmp(s1: *const c_void, s2: *const c_void, n: size_t) -> c_int;
    fn free(ptr: *mut c_void);
    fn fprintf(stream: *mut FILE, format: *const c_char, ...) -> c_int;
    fn opendir(name: *const c_char) -> *mut DIR;
    fn readdir(dirp: *mut DIR) -> *mut dirent;
    fn closedir(dirp: *mut DIR) -> c_int;
    fn rmdir(pathname: *const c_char) -> c_int;

    fn scnprintf(buf: *mut c_char, size: size_t, fmt: *const c_char, ...) -> c_int;
    fn sysfs__snprintf_build_id(root_dir: *const c_char, sbuildid: *mut c_char, size: size_t) -> c_int;
    fn fetch_current_timestamp(buf: *mut c_char, sz: size_t) -> c_int;
    fn kallsyms__get_function_start(file: *const c_char, name: *const c_char, addr: *mut u64) -> c_int;
    fn compare_proc_modules(from: *const c_char, to: *const c_char) -> c_int;
    fn mkdir_p(path: *const c_char, mode: c_int) -> c_int;
    fn kcore_copy(from_dir: *const c_char, to_dir: *const c_char) -> c_int;
    fn pr_debug(fmt: *const c_char, ...);
    fn pr_warning(fmt: *const c_char, ...);
    fn nsinfo__mountns_enter(nsi: *mut nsinfo, nsc: *mut nscookie);
    fn nsinfo__mountns_exit(nsc: *mut nscookie);
    fn filename__read_build_id(filename: *const c_char, bid: *mut build_id) -> c_int;
    fn build_id__snprintf(bid: *const build_id, buf: *mut c_char, size: size_t) -> c_int;
    fn build_id_cache__add_s(sbuild_id: *const c_char, filename: *const c_char, nsi: *mut nsinfo, is_kallsyms: bool, is_vdso: bool) -> c_int;
    fn build_id_cache__remove_s(sbuild_id: *const c_char) -> c_int;
    fn build_id_cache__list_build_ids(pathname: *const c_char, nsi: *mut nsinfo, list: *mut *mut strlist) -> c_int;
    fn build_id_cache__list_all(validonly: bool) -> *mut strlist;
    fn build_id_cache__origname(sbuild_id: *const c_char) -> *mut c_char;
    fn build_id_cache__cached(sbuild_id: *const c_char) -> bool;
    fn dso__build_id_filename(dso: *mut dso, filename: *mut c_char, size: size_t, is_debug: bool) -> bool;
    fn dso__bid(dso: *mut dso) -> *mut build_id;
    fn perf_session__fprintf_dsos_buildid(session: *mut perf_session, fp: *mut FILE, cb: unsafe extern "C" fn(*mut dso, c_int) -> bool, parm: c_int) -> c_int;
    fn perf_config(cb: unsafe extern "C" fn(*const c_char, *const c_char, *mut c_void) -> c_int, data: *mut c_void) -> c_int;
    fn parse_options(argc: c_int, argv: *const *const c_char, options: *const option, usagestr: *const *const c_char, flags: c_uint) -> c_int;
    fn usage_with_options(usagestr: *const *const c_char, options: *const option) -> !;
    fn usage_with_options_msg(usagestr: *const *const c_char, options: *const option, fmt: *const c_char, ...) -> !;
    fn perf_debuginfod_setup(debuginfod: *mut perf_debuginfod);
    fn nsinfo__new(pid: c_int) -> *mut nsinfo;
    fn perf_session__new(data: *mut perf_data, repipe: *mut c_void) -> *mut perf_session;
    fn IS_ERR(ptr: *const c_void) -> bool;
    fn PTR_ERR(ptr: *const c_void) -> c_int;
    fn perf_session__env(session: *mut perf_session) -> *mut perf_env;
    fn symbol__init(env: *mut perf_env) -> c_int;
    fn setup_pager();
    fn strlist__new(slist: *const c_char, dupstr: *mut c_void) -> *mut strlist;
    fn strlist__delete(slist: *mut strlist);
    fn strlist__first(slist: *mut strlist) -> *mut str_node;
    fn strlist__next(pos: *mut str_node) -> *mut str_node;
    fn str_error_r(errnum: c_int, buf: *mut c_char, buflen: size_t) -> *mut c_char;
    fn perf_session__delete(session: *mut perf_session);
    fn nsinfo__zput(nsi: *mut nsinfo);
}

unsafe fn cstr(bytes: &'static [u8]) -> *const c_char {
    bytes.as_ptr() as *const c_char
}

unsafe fn build_id_cache__kcore_buildid(proc_dir: *const c_char, sbuildid: *mut c_char, sbuildid_size: size_t) -> c_int {
    let mut root_dir = [0 as c_char; PATH_MAX];
    let mut p: *mut c_char;

    strlcpy(root_dir.as_mut_ptr(), proc_dir, root_dir.len());

    p = strrchr(root_dir.as_ptr(), '/' as c_int);
    if p.is_null() {
        return -1;
    }
    *p = 0;
    sysfs__snprintf_build_id(root_dir.as_ptr(), sbuildid, sbuildid_size)
}

unsafe fn build_id_cache__kcore_dir(dir: *mut c_char, sz: size_t) -> c_int {
    fetch_current_timestamp(dir, sz)
}

unsafe fn same_kallsyms_reloc(from_dir: *const c_char, to_dir: *mut c_char) -> bool {
    let mut from = [0 as c_char; PATH_MAX];
    let mut to = [0 as c_char; PATH_MAX];
    let mut name: *const c_char;
    let mut addr1: u64 = 0;
    let mut addr2: u64 = 0;
    let mut i: usize = 0;
    let mut err: c_int = -1;

    scnprintf(from.as_mut_ptr(), from.len(), cstr(b"%s/kallsyms\0"), from_dir);
    scnprintf(to.as_mut_ptr(), to.len(), cstr(b"%s/kallsyms\0"), to_dir);

    loop {
        name = ref_reloc_sym_names[i];
        if name.is_null() {
            break;
        }
        err = kallsyms__get_function_start(from.as_ptr(), name, &mut addr1);
        if err == 0 {
            break;
        }
        i += 1;
    }

    if err != 0 {
        return false;
    }

    if kallsyms__get_function_start(to.as_ptr(), name, &mut addr2) != 0 {
        return false;
    }

    addr1 == addr2
}

unsafe fn build_id_cache__kcore_existing(from_dir: *const c_char, to_dir: *mut c_char, to_dir_sz: size_t) -> c_int {
    let mut from = [0 as c_char; PATH_MAX];
    let mut to = [0 as c_char; PATH_MAX];
    let mut to_subdir = [0 as c_char; PATH_MAX];
    let mut dent: *mut dirent;
    let mut ret: c_int = -1;
    let d: *mut DIR;

    d = opendir(to_dir);
    if d.is_null() {
        return -1;
    }

    scnprintf(from.as_mut_ptr(), from.len(), cstr(b"%s/modules\0"), from_dir);

    loop {
        dent = readdir(d);
        if dent.is_null() {
            break;
        }
        if (*dent).d_type != DT_DIR {
            continue;
        }
        scnprintf(to.as_mut_ptr(), to.len(), cstr(b"%s/%s/modules\0"), to_dir, (*dent).d_name.as_ptr());
        scnprintf(to_subdir.as_mut_ptr(), to_subdir.len(), cstr(b"%s/%s\0"), to_dir, (*dent).d_name.as_ptr());
        if compare_proc_modules(from.as_ptr(), to.as_ptr()) == 0 &&
            same_kallsyms_reloc(from_dir, to_subdir.as_mut_ptr()) {
            strlcpy(to_dir, to_subdir.as_ptr(), to_dir_sz);
            ret = 0;
            break;
        }
    }

    closedir(d);

    ret
}

unsafe fn build_id_cache__add_kcore(filename: *const c_char, force: bool) -> c_int {
    let mut dir = [0 as c_char; 32];
    let mut sbuildid = [0 as c_char; SBUILD_ID_SIZE];
    let mut from_dir = [0 as c_char; PATH_MAX];
    let mut to_dir = [0 as c_char; PATH_MAX];
    let mut p: *mut c_char;

    strlcpy(from_dir.as_mut_ptr(), filename, from_dir.len());

    p = strrchr(from_dir.as_ptr(), '/' as c_int);
    if p.is_null() || strcmp(p.add(1), cstr(b"kcore\0")) != 0 {
        return -1;
    }
    *p = 0;

    if build_id_cache__kcore_buildid(from_dir.as_ptr(), sbuildid.as_mut_ptr(), sbuildid.len()) < 0 {
        return -1;
    }

    scnprintf(to_dir.as_mut_ptr(), to_dir.len(), cstr(b"%s/%s/%s\0"), buildid_dir, DSO__NAME_KCORE, sbuildid.as_ptr());

    if !force && build_id_cache__kcore_existing(from_dir.as_ptr(), to_dir.as_mut_ptr(), to_dir.len()) == 0 {
        pr_debug(cstr(b"same kcore found in %s\n\0"), to_dir.as_ptr());
        return 0;
    }

    if build_id_cache__kcore_dir(dir.as_mut_ptr(), dir.len()) != 0 {
        return -1;
    }

    scnprintf(to_dir.as_mut_ptr(), to_dir.len(), cstr(b"%s/%s/%s/%s\0"), buildid_dir, DSO__NAME_KCORE, sbuildid.as_ptr(), dir.as_ptr());

    if mkdir_p(to_dir.as_ptr(), 0o755) != 0 {
        return -1;
    }

    if kcore_copy(from_dir.as_ptr(), to_dir.as_ptr()) != 0 {
        /* Remove YYYYmmddHHMMSShh directory */
        if rmdir(to_dir.as_ptr()) == 0 {
            p = strrchr(to_dir.as_ptr(), '/' as c_int);
            if !p.is_null() {
                *p = 0;
            }
            /* Try to remove buildid directory */
            if rmdir(to_dir.as_ptr()) == 0 {
                p = strrchr(to_dir.as_ptr(), '/' as c_int);
                if !p.is_null() {
                    *p = 0;
                }
                /* Try to remove [kernel.kcore] directory */
                rmdir(to_dir.as_ptr());
            }
        }
        return -1;
    }

    pr_debug(cstr(b"kcore added to build-id cache directory %s\n\0"), to_dir.as_ptr());

    0
}

unsafe fn build_id_cache__add_file(filename: *const c_char, nsi: *mut nsinfo) -> c_int {
    let mut sbuild_id = [0 as c_char; SBUILD_ID_SIZE];
    let mut bid = build_id { size: 0, data: [0; SBUILD_ID_SIZE] };
    let mut err: c_int;
    let mut nsc = core::mem::MaybeUninit::<nscookie>::uninit();

    nsinfo__mountns_enter(nsi, nsc.as_mut_ptr());
    err = filename__read_build_id(filename, &mut bid);
    nsinfo__mountns_exit(nsc.as_mut_ptr());
    if err < 0 {
        pr_debug(cstr(b"Couldn't read a build-id in %s\n\0"), filename);
        return -1;
    }

    build_id__snprintf(&bid, sbuild_id.as_mut_ptr(), sbuild_id.len());
    err = build_id_cache__add_s(sbuild_id.as_ptr(), filename, nsi, false, false);
    pr_debug(cstr(b"Adding %s %s: %s\n\0"), sbuild_id.as_ptr(), filename, if err != 0 { cstr(b"FAIL\0") } else { cstr(b"Ok\0") });
    err
}

unsafe fn build_id_cache__remove_file(filename: *const c_char, nsi: *mut nsinfo) -> c_int {
    let mut sbuild_id = [0 as c_char; SBUILD_ID_SIZE];
    let mut bid = build_id { size: 0, data: [0; SBUILD_ID_SIZE] };
    let mut nsc = core::mem::MaybeUninit::<nscookie>::uninit();
    let mut err: c_int;

    nsinfo__mountns_enter(nsi, nsc.as_mut_ptr());
    err = filename__read_build_id(filename, &mut bid);
    nsinfo__mountns_exit(nsc.as_mut_ptr());
    if err < 0 {
        pr_debug(cstr(b"Couldn't read a build-id in %s\n\0"), filename);
        return -1;
    }

    build_id__snprintf(&bid, sbuild_id.as_mut_ptr(), sbuild_id.len());
    err = build_id_cache__remove_s(sbuild_id.as_ptr());
    pr_debug(cstr(b"Removing %s %s: %s\n\0"), sbuild_id.as_ptr(), filename, if err != 0 { cstr(b"FAIL\0") } else { cstr(b"Ok\0") });

    err
}

unsafe fn build_id_cache__purge_path(pathname: *const c_char, nsi: *mut nsinfo) -> c_int {
    let mut list: *mut strlist = core::ptr::null_mut();
    let mut pos: *mut str_node;
    let mut err: c_int;

    err = build_id_cache__list_build_ids(pathname, nsi, &mut list);
    if err == 0 {
        pos = strlist__first(list);
        while !pos.is_null() {
            err = build_id_cache__remove_s((*pos).s);
            pr_debug(cstr(b"Removing %s %s: %s\n\0"), (*pos).s, pathname, if err != 0 { cstr(b"FAIL\0") } else { cstr(b"Ok\0") });
            if err != 0 {
                break;
            }
            pos = strlist__next(pos);
        }
        strlist__delete(list);
    }

    pr_debug(cstr(b"Purging %s: %s\n\0"), pathname, if err != 0 { cstr(b"FAIL\0") } else { cstr(b"Ok\0") });

    err
}

unsafe fn build_id_cache__purge_all() -> c_int {
    let mut list: *mut strlist;
    let mut pos: *mut str_node;
    let mut err: c_int = 0;
    let mut buf: *mut c_char;

    list = build_id_cache__list_all(false);
    if list.is_null() {
        pr_debug(cstr(b"Failed to get buildids: -%d\n\0"), errno);
        return -EINVAL;
    }

    pos = strlist__first(list);
    while !pos.is_null() {
        buf = build_id_cache__origname((*pos).s);
        err = build_id_cache__remove_s((*pos).s);
        pr_debug(cstr(b"Removing %s (%s): %s\n\0"), buf, (*pos).s, if err != 0 { cstr(b"FAIL\0") } else { cstr(b"Ok\0") });
        free(buf as *mut c_void);
        if err != 0 {
            break;
        }
        pos = strlist__next(pos);
    }
    strlist__delete(list);

    pr_debug(cstr(b"Purged all: %s\n\0"), if err != 0 { cstr(b"FAIL\0") } else { cstr(b"Ok\0") });
    err
}

unsafe extern "C" fn dso__missing_buildid_cache(dso: *mut dso, _parm: c_int) -> bool {
    let mut filename = [0 as c_char; PATH_MAX];
    let mut bid = build_id { size: 0, data: [0; SBUILD_ID_SIZE] };
    let err: c_int;

    if !dso__build_id_filename(dso, filename.as_mut_ptr(), filename.len(), false) {
        return true;
    }

    err = filename__read_build_id(filename.as_ptr(), &mut bid);
    if err < 0 {
        if err == -ENOENT {
            return false;
        }

        pr_warning(cstr(b"Problems with %s file, consider removing it from the cache\n\0"), filename.as_ptr());
    } else if memcmp((*dso__bid(dso)).data.as_ptr() as *const c_void, bid.data.as_ptr() as *const c_void, bid.size as size_t) != 0 {
        pr_warning(cstr(b"Problems with %s file, consider removing it from the cache\n\0"), filename.as_ptr());
    }

    true
}

unsafe fn build_id_cache__fprintf_missing(session: *mut perf_session, fp: *mut FILE) -> c_int {
    perf_session__fprintf_dsos_buildid(session, fp, dso__missing_buildid_cache, 0);
    0
}

unsafe fn build_id_cache__update_file(filename: *const c_char, nsi: *mut nsinfo) -> c_int {
    let mut sbuild_id = [0 as c_char; SBUILD_ID_SIZE];
    let mut bid = build_id { size: 0, data: [0; SBUILD_ID_SIZE] };
    let mut nsc = core::mem::MaybeUninit::<nscookie>::uninit();
    let mut err: c_int;

    nsinfo__mountns_enter(nsi, nsc.as_mut_ptr());
    err = filename__read_build_id(filename, &mut bid);
    nsinfo__mountns_exit(nsc.as_mut_ptr());
    if err < 0 {
        pr_debug(cstr(b"Couldn't read a build-id in %s\n\0"), filename);
        return -1;
    }
    err = 0;

    build_id__snprintf(&bid, sbuild_id.as_mut_ptr(), sbuild_id.len());
    if build_id_cache__cached(sbuild_id.as_ptr()) {
        err = build_id_cache__remove_s(sbuild_id.as_ptr());
    }

    if err == 0 {
        err = build_id_cache__add_s(sbuild_id.as_ptr(), filename, nsi, false, false);
    }

    pr_debug(cstr(b"Updating %s %s: %s\n\0"), sbuild_id.as_ptr(), filename, if err != 0 { cstr(b"FAIL\0") } else { cstr(b"Ok\0") });

    err
}

unsafe fn build_id_cache__show_all() -> c_int {
    let mut bidlist: *mut strlist;
    let mut nd: *mut str_node;
    let mut buf: *mut c_char;

    bidlist = build_id_cache__list_all(true);
    if bidlist.is_null() {
        pr_debug(cstr(b"Failed to get buildids: -%d\n\0"), errno);
        return -1;
    }
    nd = strlist__first(bidlist);
    while !nd.is_null() {
        buf = build_id_cache__origname((*nd).s);
        fprintf(stdout, cstr(b"%s %s\n\0"), (*nd).s, buf);
        free(buf as *mut c_void);
        nd = strlist__next(nd);
    }
    strlist__delete(bidlist);
    0
}

unsafe extern "C" fn perf_buildid_cache_config(var: *const c_char, value: *const c_char, cb: *mut c_void) -> c_int {
    let di = cb as *mut perf_debuginfod;

    if strcmp(var, cstr(b"buildid-cache.debuginfod\0")) == 0 {
        (*di).urls = strdup(value);
        if (*di).urls.is_null() {
            return -ENOMEM;
        }
        (*di).set = true;
    }

    0
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn cmd_buildid_cache(mut argc: c_int, argv: *const *const c_char) -> c_int {
    let mut list: *mut strlist;
    let mut pos: *mut str_node;
    let mut ret: c_int;
    let mut ns_id: c_int = -1;
    let mut force = false;
    let mut list_files = false;
    let mut opts_flag: bool;
    let mut purge_all = false;
    let mut add_name_list_str: *const c_char = core::ptr::null();
    let mut remove_name_list_str: *const c_char = core::ptr::null();
    let mut purge_name_list_str: *const c_char = core::ptr::null();
    let mut missing_filename: *const c_char = core::ptr::null();
    let mut update_name_list_str: *const c_char = core::ptr::null();
    let mut kcore_filename: *const c_char = core::ptr::null();
    let mut debuginfod = perf_debuginfod { urls: core::ptr::null_mut(), set: false };
    let mut sbuf = [0 as c_char; STRERR_BUFSIZE];

    let mut data = perf_data {
        mode: PERF_DATA_MODE_READ,
        path: core::ptr::null(),
        force: false,
    };
    let mut session: *mut perf_session = core::ptr::null_mut();
    let mut nsi: *mut nsinfo = core::ptr::null_mut();

    /*
     * The C source declares buildid_cache_options with OPT_STRING,
     * OPT_BOOLEAN, OPT_STRING_OPTARG_SET, OPT_INCR, OPT_INTEGER, and OPT_END
     * macros. Those option-construction macros are external dependency surface
     * for this isolated translation, so the array storage is left as an opaque
     * option block while preserving the parse_options/usage call structure.
     */
    let buildid_cache_options: [option; 1] = [option { _private: [] }];
    let buildid_cache_usage: [*const c_char; 2] = [
        cstr(b"perf buildid-cache [<options>]\0"),
        core::ptr::null(),
    ];

    ret = perf_config(perf_buildid_cache_config, &mut debuginfod as *mut _ as *mut c_void);
    if ret != 0 {
        return ret;
    }

    argc = parse_options(argc, argv, buildid_cache_options.as_ptr(), buildid_cache_usage.as_ptr(), 0);

    opts_flag = !add_name_list_str.is_null() || !kcore_filename.is_null() ||
        !remove_name_list_str.is_null() || !purge_name_list_str.is_null() ||
        !missing_filename.is_null() || !update_name_list_str.is_null() ||
        purge_all;

    if argc != 0 || !(list_files || opts_flag) {
        usage_with_options(buildid_cache_usage.as_ptr(), buildid_cache_options.as_ptr());
    }

    perf_debuginfod_setup(&mut debuginfod);

    /* -l is exclusive. It can not be used with other options. */
    if list_files && opts_flag {
        usage_with_options_msg(buildid_cache_usage.as_ptr(), buildid_cache_options.as_ptr(), cstr(b"-l is exclusive.\n\0"));
    }

    if ns_id > 0 {
        nsi = nsinfo__new(ns_id);
    }

    if !missing_filename.is_null() {
        data.path = missing_filename;
        data.force = force;

        session = perf_session__new(&mut data, core::ptr::null_mut());
        if IS_ERR(session as *const c_void) {
            return PTR_ERR(session as *const c_void);
        }
    }

    if symbol__init(if !session.is_null() { perf_session__env(session) } else { core::ptr::null_mut() }) < 0 {
        goto_out(session, nsi, ret)
    } else {
        setup_pager();

        if list_files {
            ret = build_id_cache__show_all();
            return goto_out(session, nsi, ret);
        }

        if !add_name_list_str.is_null() {
            list = strlist__new(add_name_list_str, core::ptr::null_mut());
            if !list.is_null() {
                pos = strlist__first(list);
                while !pos.is_null() {
                    if build_id_cache__add_file((*pos).s, nsi) != 0 {
                        if errno == EEXIST {
                            pr_debug(cstr(b"%s already in the cache\n\0"), (*pos).s);
                            pos = strlist__next(pos);
                            continue;
                        }
                        pr_warning(cstr(b"Couldn't add %s: %s\n\0"), (*pos).s, str_error_r(errno, sbuf.as_mut_ptr(), sbuf.len()));
                    }
                    pos = strlist__next(pos);
                }

                strlist__delete(list);
            }
        }

        if !remove_name_list_str.is_null() {
            list = strlist__new(remove_name_list_str, core::ptr::null_mut());
            if !list.is_null() {
                pos = strlist__first(list);
                while !pos.is_null() {
                    if build_id_cache__remove_file((*pos).s, nsi) != 0 {
                        if errno == ENOENT {
                            pr_debug(cstr(b"%s wasn't in the cache\n\0"), (*pos).s);
                            pos = strlist__next(pos);
                            continue;
                        }
                        pr_warning(cstr(b"Couldn't remove %s: %s\n\0"), (*pos).s, str_error_r(errno, sbuf.as_mut_ptr(), sbuf.len()));
                    }
                    pos = strlist__next(pos);
                }

                strlist__delete(list);
            }
        }

        if !purge_name_list_str.is_null() {
            list = strlist__new(purge_name_list_str, core::ptr::null_mut());
            if !list.is_null() {
                pos = strlist__first(list);
                while !pos.is_null() {
                    if build_id_cache__purge_path((*pos).s, nsi) != 0 {
                        if errno == ENOENT {
                            pr_debug(cstr(b"%s wasn't in the cache\n\0"), (*pos).s);
                            pos = strlist__next(pos);
                            continue;
                        }
                        pr_warning(cstr(b"Couldn't remove %s: %s\n\0"), (*pos).s, str_error_r(errno, sbuf.as_mut_ptr(), sbuf.len()));
                    }
                    pos = strlist__next(pos);
                }

                strlist__delete(list);
            }
        }

        if purge_all {
            if build_id_cache__purge_all() != 0 {
                pr_warning(cstr(b"Couldn't remove some caches. Error: %s.\n\0"), str_error_r(errno, sbuf.as_mut_ptr(), sbuf.len()));
            }
        }

        if !missing_filename.is_null() {
            ret = build_id_cache__fprintf_missing(session, stdout);
        }

        if !update_name_list_str.is_null() {
            list = strlist__new(update_name_list_str, core::ptr::null_mut());
            if !list.is_null() {
                pos = strlist__first(list);
                while !pos.is_null() {
                    if build_id_cache__update_file((*pos).s, nsi) != 0 {
                        if errno == ENOENT {
                            pr_debug(cstr(b"%s wasn't in the cache\n\0"), (*pos).s);
                            pos = strlist__next(pos);
                            continue;
                        }
                        pr_warning(cstr(b"Couldn't update %s: %s\n\0"), (*pos).s, str_error_r(errno, sbuf.as_mut_ptr(), sbuf.len()));
                    }
                    pos = strlist__next(pos);
                }

                strlist__delete(list);
            }
        }

        if !kcore_filename.is_null() && build_id_cache__add_kcore(kcore_filename, force) != 0 {
            pr_warning(cstr(b"Couldn't add %s\n\0"), kcore_filename);
        }

        goto_out(session, nsi, ret)
    }
}

unsafe fn goto_out(session: *mut perf_session, nsi: *mut nsinfo, ret: c_int) -> c_int {
    perf_session__delete(session);
    nsinfo__zput(nsi);

    ret
}
