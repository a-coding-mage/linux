// SPDX-License-Identifier: GPL-2.0
//
// kselftest configuration helpers for the hw specific configuration
//
// Original author: Jaroslav Kysela <perex@perex.cz>
// Copyright (c) 2022 Red Hat Inc.

use core::ffi::{c_char, c_int, c_long, c_uint, c_void};
use core::mem;
use core::ptr;

const SYSFS_ROOT: &[u8] = b"/sys\0";
const PATH_MAX: usize = 4096;
const O_RDONLY: c_int = 0;
const R_OK: c_int = 4;
const S_IRUSR: c_uint = 0o400;
const S_IFMT: c_uint = 0o170000;
const S_IFDIR: c_uint = 0o040000;
const S_IFLNK: c_uint = 0o120000;
const DT_DIR: u8 = 4;
const ENOENT: c_int = 2;
const REG_EXTENDED: c_int = 1;
const SND_CONFIG_TYPE_COMPOUND: c_int = 1024;

#[inline]
unsafe fn S_ISLNK(mode: c_uint) -> bool {
    mode & S_IFMT == S_IFLNK
}

#[inline]
unsafe fn S_ISDIR(mode: c_uint) -> bool {
    mode & S_IFMT == S_IFDIR
}

#[repr(C)]
pub struct snd_config_t {
    _private: [u8; 0],
}

#[repr(C)]
pub struct snd_input_t {
    _private: [u8; 0],
}

#[repr(C)]
pub struct snd_output_t {
    _private: [u8; 0],
}

pub type snd_config_iterator_t = *mut c_void;

#[repr(C)]
pub struct regex_t {
    _private: [u8; 0],
}

#[repr(C)]
pub struct regmatch_t {
    _private: [u8; 0],
}

#[repr(C)]
pub struct stat {
    pub st_mode: c_uint,
}

#[repr(C)]
pub struct dirent {
    pub d_ino: usize,
    pub d_off: isize,
    pub d_reclen: u16,
    pub d_type: u8,
    pub d_name: [c_char; 256],
}

#[repr(C)]
pub struct card_cfg_data {
    pub filename: *const c_char,
    pub config: *mut snd_config_t,
    pub config_id: *const c_char,
    pub card: c_int,
    pub next: *mut card_cfg_data,
}

pub static mut conf_cards: *mut card_cfg_data = ptr::null_mut();

static alsa_config: &[u8] = b"ctl.hw {\n\
\t@args [ CARD ]\n\
\t@args.CARD.type string\n\
\ttype hw\n\
\tcard $CARD\n\
}\n\
pcm.hw {\n\
\t@args [ CARD DEV SUBDEV ]\n\
\t@args.CARD.type string\n\
\t@args.DEV.type integer\n\
\t@args.SUBDEV.type integer\n\
\ttype hw\n\
\tcard $CARD\n\
\tdevice $DEV\n\
\tsubdevice $SUBDEV\n\
}\n\0";

unsafe extern "C" {
    static mut stdout: *mut c_void;
    static mut errno: c_int;

    fn strlen(s: *const c_char) -> usize;
    fn snprintf(s: *mut c_char, n: usize, format: *const c_char, ...) -> c_int;
    fn sprintf(s: *mut c_char, format: *const c_char, ...) -> c_int;
    fn strncmp(s1: *const c_char, s2: *const c_char, n: usize) -> c_int;
    fn strrchr(s: *const c_char, c: c_int) -> *mut c_char;
    fn strdup(s: *const c_char) -> *mut c_char;
    fn strerror(errnum: c_int) -> *mut c_char;
    fn malloc(size: usize) -> *mut c_void;
    fn free(ptr: *mut c_void);
    fn open(pathname: *const c_char, flags: c_int, ...) -> c_int;
    fn read(fd: c_int, buf: *mut c_void, count: usize) -> isize;
    fn close(fd: c_int) -> c_int;
    fn access(pathname: *const c_char, mode: c_int) -> c_int;
    fn lstat(pathname: *const c_char, statbuf: *mut stat) -> c_int;
    fn readlink(pathname: *const c_char, buf: *mut c_char, bufsiz: usize) -> isize;
    fn scandir(
        dirp: *const c_char,
        namelist: *mut *mut *mut dirent,
        filter: Option<unsafe extern "C" fn(*const dirent) -> c_int>,
        compar: Option<unsafe extern "C" fn(*const *const dirent, *const *const dirent) -> c_int>,
    ) -> c_int;
    fn alphasort(a: *const *const dirent, b: *const *const dirent) -> c_int;
    fn regcomp(preg: *mut regex_t, regex: *const c_char, cflags: c_int) -> c_int;
    fn regexec(
        preg: *const regex_t,
        string: *const c_char,
        nmatch: usize,
        pmatch: *mut regmatch_t,
        eflags: c_int,
    ) -> c_int;
    fn regfree(preg: *mut regex_t);

    fn ksft_print_msg(msg: *const c_char, ...);
    fn ksft_exit_fail();
    fn ksft_exit_fail_msg(msg: *const c_char, ...) -> !;

    fn snd_strerror(errnum: c_int) -> *const c_char;
    fn snd_input_buffer_open(
        inputp: *mut *mut snd_input_t,
        buffer: *const c_char,
        size: usize,
    ) -> c_int;
    fn snd_input_stdio_open(
        inputp: *mut *mut snd_input_t,
        file: *const c_char,
        mode: *const c_char,
    ) -> c_int;
    fn snd_input_close(input: *mut snd_input_t) -> c_int;
    fn snd_config_top(config: *mut *mut snd_config_t) -> c_int;
    fn snd_config_load(config: *mut snd_config_t, input: *mut snd_input_t) -> c_int;
    fn snd_config_delete(config: *mut snd_config_t) -> c_int;
    fn snd_config_save(config: *mut snd_config_t, output: *mut snd_output_t) -> c_int;
    fn snd_config_search(
        config: *mut snd_config_t,
        key: *const c_char,
        result: *mut *mut snd_config_t,
    ) -> c_int;
    fn snd_config_get_type(config: *mut snd_config_t) -> c_int;
    fn snd_config_get_string(
        config: *mut snd_config_t,
        ptr: *mut *const c_char,
    ) -> c_int;
    fn snd_config_get_integer(config: *mut snd_config_t, ptr: *mut c_long) -> c_int;
    fn snd_config_get_bool(config: *mut snd_config_t) -> c_int;
    fn snd_config_get_id(config: *mut snd_config_t, id: *mut *const c_char) -> c_int;
    fn snd_config_iterator_first(config: *mut snd_config_t) -> snd_config_iterator_t;
    fn snd_config_iterator_next(iterator: snd_config_iterator_t) -> snd_config_iterator_t;
    fn snd_config_iterator_end(config: *mut snd_config_t) -> snd_config_iterator_t;
    fn snd_config_iterator_entry(iterator: snd_config_iterator_t) -> *mut snd_config_t;
    fn snd_output_stdio_attach(
        outputp: *mut *mut snd_output_t,
        fp: *mut c_void,
        _close: c_int,
    ) -> c_int;
    fn snd_output_close(output: *mut snd_output_t) -> c_int;
}

// If alsa-lib does not provide snd_config_load_string, keep the local fallback.
unsafe fn snd_config_load_string(
    config: *mut *mut snd_config_t,
    s: *const c_char,
    mut size: usize,
) -> c_int {
    let mut input: *mut snd_input_t = ptr::null_mut();
    let mut dst: *mut snd_config_t = ptr::null_mut();
    let mut err: c_int;

    assert!(!config.is_null() && !s.is_null());
    if size == 0 {
        size = strlen(s);
    }
    err = snd_input_buffer_open(&mut input, s, size);
    if err < 0 {
        return err;
    }
    err = snd_config_top(&mut dst);
    if err < 0 {
        snd_input_close(input);
        return err;
    }
    err = snd_config_load(dst, input);
    snd_input_close(input);
    if err < 0 {
        snd_config_delete(dst);
        return err;
    }
    *config = dst;
    0
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn get_alsalib_config() -> *mut snd_config_t {
    let mut config: *mut snd_config_t = ptr::null_mut();
    let err: c_int;

    err = snd_config_load_string(
        &mut config,
        alsa_config.as_ptr() as *const c_char,
        strlen(alsa_config.as_ptr() as *const c_char),
    );
    if err < 0 {
        ksft_print_msg(
            c"Unable to parse custom alsa-lib configuration: %s\n".as_ptr(),
            snd_strerror(err),
        );
        ksft_exit_fail();
    }
    config
}

unsafe fn conf_data_by_card(card: c_int, msg: bool) -> *mut card_cfg_data {
    let mut conf: *mut card_cfg_data;

    conf = conf_cards;
    while !conf.is_null() {
        if (*conf).card == card {
            if msg {
                ksft_print_msg(
                    c"using hw card config %s for card %d\n".as_ptr(),
                    (*conf).filename,
                    card,
                );
            }
            return conf;
        }
        conf = (*conf).next;
    }
    ptr::null_mut()
}

unsafe fn dump_config_tree(top: *mut snd_config_t) {
    let mut out: *mut snd_output_t = ptr::null_mut();
    let err: c_int;

    err = snd_output_stdio_attach(&mut out, stdout, 0);
    if err < 0 {
        ksft_exit_fail_msg(c"stdout attach\n".as_ptr());
    }
    if snd_config_save(top, out) != 0 {
        ksft_exit_fail_msg(c"config save\n".as_ptr());
    }
    snd_output_close(out);
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn conf_load_from_file(filename: *const c_char) -> *mut snd_config_t {
    let mut dst: *mut snd_config_t = ptr::null_mut();
    let mut input: *mut snd_input_t = ptr::null_mut();
    let mut err: c_int;

    err = snd_input_stdio_open(&mut input, filename, c"r".as_ptr());
    if err < 0 {
        ksft_exit_fail_msg(c"Unable to parse filename %s\n".as_ptr(), filename);
    }
    err = snd_config_top(&mut dst);
    if err < 0 {
        ksft_exit_fail_msg(c"Out of memory\n".as_ptr());
    }
    err = snd_config_load(dst, input);
    snd_input_close(input);
    if err < 0 {
        ksft_exit_fail_msg(c"Unable to parse filename %s\n".as_ptr(), filename);
    }
    dst
}

unsafe fn sysfs_get(sysfs_root: *const c_char, mut id: *const c_char) -> *mut c_char {
    let mut path = [0 as c_char; PATH_MAX];
    let mut link = [0 as c_char; PATH_MAX + 1];
    let mut sb: stat = mem::zeroed();
    let mut len: isize;
    let mut e: *mut c_char;
    let fd: c_int;

    if *id == b'/' as c_char {
        id = id.add(1);
    }
    snprintf(
        path.as_mut_ptr(),
        path.len(),
        c"%s/%s".as_ptr(),
        sysfs_root,
        id,
    );
    if lstat(path.as_ptr(), &mut sb) != 0 {
        return ptr::null_mut();
    }
    if S_ISLNK(sb.st_mode) {
        len = readlink(path.as_ptr(), link.as_mut_ptr(), link.len() - 1);
        if len <= 0 {
            ksft_exit_fail_msg(
                c"sysfs: cannot read link '%s': %s\n".as_ptr(),
                path.as_ptr(),
                strerror(errno),
            );
        }
        link[len as usize] = 0;
        e = strrchr(link.as_ptr(), b'/' as c_int);
        if !e.is_null() {
            return strdup(e.add(1));
        }
        return ptr::null_mut();
    }
    if S_ISDIR(sb.st_mode) {
        return ptr::null_mut();
    }
    if (sb.st_mode & S_IRUSR) == 0 {
        return ptr::null_mut();
    }

    fd = open(path.as_ptr(), O_RDONLY);
    if fd < 0 {
        if errno == ENOENT {
            return ptr::null_mut();
        }
        ksft_exit_fail_msg(
            c"sysfs: open failed for '%s': %s\n".as_ptr(),
            path.as_ptr(),
            strerror(errno),
        );
    }
    len = read(fd, path.as_mut_ptr() as *mut c_void, path.len() - 1);
    close(fd);
    if len < 0 {
        ksft_exit_fail_msg(
            c"sysfs: unable to read value '%s': %s\n".as_ptr(),
            path.as_ptr(),
            strerror(errno),
        );
    }
    while len > 0 && path[len as usize - 1] == b'\n' as c_char {
        len -= 1;
    }
    path[len as usize] = 0;
    e = strdup(path.as_ptr());
    if e.is_null() {
        ksft_exit_fail_msg(c"Out of memory\n".as_ptr());
    }
    e
}

unsafe fn sysfs_match(sysfs_root: *const c_char, config: *mut snd_config_t) -> bool {
    let mut node: *mut snd_config_t;
    let mut path_config: *mut snd_config_t = ptr::null_mut();
    let mut regex_config: *mut snd_config_t = ptr::null_mut();
    let mut i: snd_config_iterator_t;
    let mut next: snd_config_iterator_t;
    let mut path_string: *const c_char = ptr::null();
    let mut regex_string: *const c_char = ptr::null();
    let mut v: *mut c_char;
    let mut re: regex_t = mem::zeroed();
    let mut match_: [regmatch_t; 1] = [mem::zeroed()];
    let mut iter: c_int = 0;
    let ret: c_int;

    i = snd_config_iterator_first(config);
    while i != snd_config_iterator_end(config) {
        next = snd_config_iterator_next(i);
        node = snd_config_iterator_entry(i);
        if snd_config_search(node, c"path".as_ptr(), &mut path_config) != 0 {
            ksft_exit_fail_msg(c"Missing path field in the sysfs block\n".as_ptr());
        }
        if snd_config_search(node, c"regex".as_ptr(), &mut regex_config) != 0 {
            ksft_exit_fail_msg(c"Missing regex field in the sysfs block\n".as_ptr());
        }
        if snd_config_get_string(path_config, &mut path_string) != 0 {
            ksft_exit_fail_msg(c"Path field in the sysfs block is not a string\n".as_ptr());
        }
        if snd_config_get_string(regex_config, &mut regex_string) != 0 {
            ksft_exit_fail_msg(c"Regex field in the sysfs block is not a string\n".as_ptr());
        }
        iter += 1;
        v = sysfs_get(sysfs_root, path_string);
        if v.is_null() {
            return false;
        }
        if regcomp(&mut re, regex_string, REG_EXTENDED) != 0 {
            ksft_exit_fail_msg(c"Wrong regex '%s'\n".as_ptr(), regex_string);
        }
        ret = regexec(&re, v, 1, match_.as_mut_ptr(), 0);
        regfree(&mut re);
        if ret != 0 {
            return false;
        }
        i = next;
    }
    iter > 0
}

unsafe fn assign_card_config(card: c_int, sysfs_card_root: *const c_char) {
    let mut data: *mut card_cfg_data;
    let mut sysfs_card_config: *mut snd_config_t = ptr::null_mut();

    data = conf_cards;
    while !data.is_null() {
        snd_config_search((*data).config, c"sysfs".as_ptr(), &mut sysfs_card_config);
        if !sysfs_match(sysfs_card_root, sysfs_card_config) {
            data = (*data).next;
            continue;
        }

        (*data).card = card;
        break;
    }
}

unsafe fn assign_card_configs() {
    let mut fn_ = [0 as c_char; 128];
    let mut card: c_int;

    card = 0;
    while card < 32 {
        snprintf(
            fn_.as_mut_ptr(),
            fn_.len(),
            c"%s/class/sound/card%d".as_ptr(),
            SYSFS_ROOT.as_ptr() as *const c_char,
            card,
        );
        if access(fn_.as_ptr(), R_OK) == 0 {
            assign_card_config(card, fn_.as_ptr());
        }
        card += 1;
    }
}

unsafe extern "C" fn filename_filter(dirent: *const dirent) -> c_int {
    let flen: usize;

    if dirent.is_null() {
        return 0;
    }
    if (*dirent).d_type == DT_DIR {
        return 0;
    }
    flen = strlen((*dirent).d_name.as_ptr());
    if flen <= 5 {
        return 0;
    }
    if strncmp((*dirent).d_name.as_ptr().add(flen - 5), c".conf".as_ptr(), 5) == 0 {
        return 1;
    }
    0
}

unsafe fn match_config(filename: *const c_char) -> bool {
    let mut data: *mut card_cfg_data;
    let config: *mut snd_config_t;
    let mut sysfs_config: *mut snd_config_t = ptr::null_mut();
    let mut card_config: *mut snd_config_t = ptr::null_mut();
    let mut sysfs_card_config: *mut snd_config_t = ptr::null_mut();
    let mut node: *mut snd_config_t;
    let mut i: snd_config_iterator_t;
    let mut next: snd_config_iterator_t;

    config = conf_load_from_file(filename);
    if snd_config_search(config, c"sysfs".as_ptr(), &mut sysfs_config) != 0
        || snd_config_get_type(sysfs_config) != SND_CONFIG_TYPE_COMPOUND
    {
        ksft_exit_fail_msg(c"Missing global sysfs block in filename %s\n".as_ptr(), filename);
    }
    if snd_config_search(config, c"card".as_ptr(), &mut card_config) != 0
        || snd_config_get_type(card_config) != SND_CONFIG_TYPE_COMPOUND
    {
        ksft_exit_fail_msg(c"Missing global card block in filename %s\n".as_ptr(), filename);
    }
    if !sysfs_match(SYSFS_ROOT.as_ptr() as *const c_char, sysfs_config) {
        return false;
    }
    i = snd_config_iterator_first(card_config);
    while i != snd_config_iterator_end(card_config) {
        next = snd_config_iterator_next(i);
        node = snd_config_iterator_entry(i);
        if snd_config_search(node, c"sysfs".as_ptr(), &mut sysfs_card_config) != 0
            || snd_config_get_type(sysfs_card_config) != SND_CONFIG_TYPE_COMPOUND
        {
            ksft_exit_fail_msg(c"Missing card sysfs block in filename %s\n".as_ptr(), filename);
        }

        data = malloc(mem::size_of::<card_cfg_data>()) as *mut card_cfg_data;
        if data.is_null() {
            ksft_exit_fail_msg(c"Out of memory\n".as_ptr());
        }
        (*data).filename = filename;
        (*data).config = node;
        (*data).card = -1;
        if snd_config_get_id(node, &mut (*data).config_id) != 0 {
            ksft_exit_fail_msg(c"snd_config_get_id failed for card\n".as_ptr());
        }
        (*data).next = conf_cards;
        conf_cards = data;
        i = next;
    }
    true
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn conf_load() {
    let fn_: *const c_char = c"conf.d".as_ptr();
    let mut namelist: *mut *mut dirent = ptr::null_mut();
    let n: c_int;
    let mut j: c_int;

    n = scandir(fn_, &mut namelist, Some(filename_filter), Some(alphasort));
    if n < 0 {
        ksft_exit_fail_msg(c"scandir: %s\n".as_ptr(), strerror(errno));
    }
    j = 0;
    while j < n {
        let sl = strlen(fn_) + strlen((**namelist.add(j as usize)).d_name.as_ptr()) + 2;
        let mut filename = malloc(sl) as *mut c_char;
        if filename.is_null() {
            ksft_exit_fail_msg(c"Out of memory\n".as_ptr());
        }
        sprintf(
            filename,
            c"%s/%s".as_ptr(),
            fn_,
            (**namelist.add(j as usize)).d_name.as_ptr(),
        );
        if match_config(filename) {
            filename = ptr::null_mut();
        }
        free(filename as *mut c_void);
        free(*namelist.add(j as usize) as *mut c_void);
        j += 1;
    }
    free(namelist as *mut c_void);

    assign_card_configs();
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn conf_free() {
    let mut conf: *mut card_cfg_data;

    while !conf_cards.is_null() {
        conf = conf_cards;
        conf_cards = (*conf).next;
        snd_config_delete((*conf).config);
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn conf_by_card(card: c_int) -> *mut snd_config_t {
    let conf: *mut card_cfg_data;

    conf = conf_data_by_card(card, true);
    if !conf.is_null() {
        return (*conf).config;
    }
    ptr::null_mut()
}

unsafe fn conf_get_by_keys(
    mut root: *mut snd_config_t,
    key1: *const c_char,
    key2: *const c_char,
    result: *mut *mut snd_config_t,
) -> c_int {
    let mut ret: c_int = 0;

    if !key1.is_null() {
        ret = snd_config_search(root, key1, &mut root);
        if ret != -ENOENT && ret < 0 {
            return ret;
        }
    }
    if !key2.is_null() {
        ret = snd_config_search(root, key2, &mut root);
    }
    if ret >= 0 {
        *result = root;
    }
    ret
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn conf_get_subtree(
    mut root: *mut snd_config_t,
    key1: *const c_char,
    key2: *const c_char,
) -> *mut snd_config_t {
    let ret: c_int;

    if root.is_null() {
        return ptr::null_mut();
    }
    ret = conf_get_by_keys(root, key1, key2, &mut root);
    if ret == -ENOENT {
        return ptr::null_mut();
    }
    if ret < 0 {
        ksft_exit_fail_msg(
            c"key '%s'.'%s' search error: %s\n".as_ptr(),
            key1,
            key2,
            snd_strerror(ret),
        );
    }
    root
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn conf_get_count(
    root: *mut snd_config_t,
    key1: *const c_char,
    key2: *const c_char,
) -> c_int {
    let mut cfg: *mut snd_config_t = ptr::null_mut();
    let mut i: snd_config_iterator_t;
    let mut next: snd_config_iterator_t;
    let mut count: c_int;
    let ret: c_int;

    if root.is_null() {
        return -1;
    }
    ret = conf_get_by_keys(root, key1, key2, &mut cfg);
    if ret == -ENOENT {
        return -1;
    }
    if ret < 0 {
        ksft_exit_fail_msg(
            c"key '%s'.'%s' search error: %s\n".as_ptr(),
            key1,
            key2,
            snd_strerror(ret),
        );
    }
    if snd_config_get_type(cfg) != SND_CONFIG_TYPE_COMPOUND {
        ksft_exit_fail_msg(c"key '%s'.'%s' is not a compound\n".as_ptr(), key1, key2);
    }
    count = 0;
    i = snd_config_iterator_first(cfg);
    while i != snd_config_iterator_end(cfg) {
        next = snd_config_iterator_next(i);
        count += 1;
        i = next;
    }
    count
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn conf_get_string(
    root: *mut snd_config_t,
    key1: *const c_char,
    key2: *const c_char,
    def: *const c_char,
) -> *const c_char {
    let mut cfg: *mut snd_config_t = ptr::null_mut();
    let mut s: *const c_char = ptr::null();
    let ret: c_int;

    if root.is_null() {
        return def;
    }
    ret = conf_get_by_keys(root, key1, key2, &mut cfg);
    if ret == -ENOENT {
        return def;
    }
    if ret < 0 {
        ksft_exit_fail_msg(
            c"key '%s'.'%s' search error: %s\n".as_ptr(),
            key1,
            key2,
            snd_strerror(ret),
        );
    }
    if snd_config_get_string(cfg, &mut s) != 0 {
        ksft_exit_fail_msg(c"key '%s'.'%s' is not a string\n".as_ptr(), key1, key2);
    }
    s
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn conf_get_long(
    root: *mut snd_config_t,
    key1: *const c_char,
    key2: *const c_char,
    def: c_long,
) -> c_long {
    let mut cfg: *mut snd_config_t = ptr::null_mut();
    let mut l: c_long = 0;
    let ret: c_int;

    if root.is_null() {
        return def;
    }
    ret = conf_get_by_keys(root, key1, key2, &mut cfg);
    if ret == -ENOENT {
        return def;
    }
    if ret < 0 {
        ksft_exit_fail_msg(
            c"key '%s'.'%s' search error: %s\n".as_ptr(),
            key1,
            key2,
            snd_strerror(ret),
        );
    }
    if snd_config_get_integer(cfg, &mut l) != 0 {
        ksft_exit_fail_msg(c"key '%s'.'%s' is not an integer\n".as_ptr(), key1, key2);
    }
    l
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn conf_get_bool(
    root: *mut snd_config_t,
    key1: *const c_char,
    key2: *const c_char,
    def: c_int,
) -> c_int {
    let mut cfg: *mut snd_config_t = ptr::null_mut();
    let mut ret: c_int;

    if root.is_null() {
        return def;
    }
    ret = conf_get_by_keys(root, key1, key2, &mut cfg);
    if ret == -ENOENT {
        return def;
    }
    if ret < 0 {
        ksft_exit_fail_msg(
            c"key '%s'.'%s' search error: %s\n".as_ptr(),
            key1,
            key2,
            snd_strerror(ret),
        );
    }
    ret = snd_config_get_bool(cfg);
    if ret < 0 {
        ksft_exit_fail_msg(c"key '%s'.'%s' is not a bool\n".as_ptr(), key1, key2);
    }
    (ret != 0) as c_int
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn conf_get_string_array(
    root: *mut snd_config_t,
    key1: *const c_char,
    key2: *const c_char,
    array: *mut *const c_char,
    array_size: c_int,
    def: *const c_char,
) {
    let mut cfg: *mut snd_config_t = ptr::null_mut();
    let mut buf = [0 as c_char; 16];
    let ret: c_int;
    let mut index: c_int;

    ret = conf_get_by_keys(root, key1, key2, &mut cfg);
    if ret == -ENOENT {
        cfg = ptr::null_mut();
    } else if ret < 0 {
        ksft_exit_fail_msg(
            c"key '%s'.'%s' search error: %s\n".as_ptr(),
            key1,
            key2,
            snd_strerror(ret),
        );
    }
    index = 0;
    while index < array_size {
        if cfg.is_null() {
            *array.add(index as usize) = def;
        } else {
            sprintf(buf.as_mut_ptr(), c"%i".as_ptr(), index);
            *array.add(index as usize) =
                conf_get_string(cfg, buf.as_ptr(), ptr::null(), def);
        }
        index += 1;
    }
}
