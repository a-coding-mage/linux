// SPDX-License-Identifier: GPL-2.0-only
/* Copyright (c) 2022 Meta Platforms, Inc. and affiliates. */

use core::ffi::{c_char, c_int, c_long, c_uint, c_ulong, c_void};
use core::mem;
use core::ptr;

type size_t = usize;
type bool_ = bool;
type __u8 = u8;
type __u32 = u32;
type __u64 = u64;

const NULL: *mut c_void = ptr::null_mut();
const ENOMEM: c_int = 12;
const EINVAL: c_int = 22;
const EOPNOTSUPP: c_int = 95;
const INT_MIN: c_int = c_int::MIN;
const REG_EXTENDED: c_int = 1;
const REG_NEWLINE: c_int = 4;
const BTF_KIND_FUNC: c_int = 12;
const CAP_SYS_ADMIN: c_int = 21;
const CAP_NET_ADMIN: c_int = 12;
const CAP_PERFMON: c_int = 38;
const CAP_BPF: c_int = 39;

/* include dependencies preserved as external declarations:
 * linux/capability.h, linux/err.h, stdlib.h, test_progs.h, bpf/btf.h,
 * autoconf_helper.h, disasm_helpers.h, unpriv_helpers.h, cap_helpers.h,
 * jit_disasm_helpers.h
 */

#[repr(C)]
pub struct regex_t {
    _private: [usize; 0],
}

#[repr(C)]
pub struct regmatch_t {
    pub rm_so: isize,
    pub rm_eo: isize,
}

#[repr(C)]
pub struct FILE {
    _private: [u8; 0],
}

#[repr(C)]
pub struct btf {
    _private: [u8; 0],
}

#[repr(C)]
pub struct btf_type {
    pub name_off: __u32,
    pub info: __u32,
    pub size: __u32,
    pub type_: __u32,
}

#[repr(C)]
pub struct btf_param {
    pub name_off: __u32,
    pub type_: __u32,
}

#[repr(C)]
pub struct btf_decl_tag {
    pub component_idx: c_int,
}

#[repr(C)]
pub struct bpf_object {
    _private: [u8; 0],
}

#[repr(C)]
pub struct bpf_program {
    _private: [u8; 0],
}

#[repr(C)]
pub struct bpf_map {
    _private: [u8; 0],
}

#[repr(C)]
pub struct bpf_link {
    _private: [u8; 0],
}

#[repr(C)]
pub struct bpf_insn {
    _private: [u8; 0],
}

#[repr(C)]
pub struct bpf_object_open_opts {
    pub sz: size_t,
    pub object_name: *const c_char,
    pub btf_custom_path: *const c_char,
}

#[repr(C)]
pub struct bpf_prog_stream_read_opts {
    pub sz: size_t,
}

#[repr(C)]
pub struct bpf_test_run_opts {
    pub sz: size_t,
    pub data_in: *mut c_void,
    pub data_size_in: __u32,
    pub data_out: *mut c_void,
    pub data_size_out: __u32,
    pub repeat: __u32,
    pub retval: __u32,
    pub ctx_in: *mut c_void,
    pub ctx_size_in: __u32,
}

#[repr(C)]
pub struct __sk_buff {
    pub data_end: __u32,
}

#[repr(C)]
pub struct expect_msg {
    pub on_next_line: bool_,
    pub substr: *const c_char,
    pub negative: bool_,
    pub is_regex: bool_,
    pub regex: regex_t,
}

#[repr(C)]
pub struct expected_msgs {
    pub patterns: *mut expect_msg,
    pub cnt: c_int,
}

#[repr(C)]
pub struct test_loader {
    pub log_buf: *mut c_char,
    pub log_buf_sz: size_t,
    pub pre_execution_cb: Option<unsafe extern "C" fn(*mut bpf_object) -> c_int>,
}

#[repr(C)]
pub struct env_t {
    pub verbosity: c_int,
}

type skel_elf_bytes_fn = Option<unsafe extern "C" fn(*mut size_t) -> *const c_void>;

unsafe extern "C" {
    static mut env: env_t;
    static mut errno: c_int;
    static mut stdout: *mut FILE;
    static mut stderr: *mut FILE;

    fn strlen(s: *const c_char) -> size_t;
    fn strncmp(a: *const c_char, b: *const c_char, n: size_t) -> c_int;
    fn strcmp(a: *const c_char, b: *const c_char) -> c_int;
    fn strstr(haystack: *const c_char, needle: *const c_char) -> *mut c_char;
    fn strchr(s: *const c_char, c: c_int) -> *mut c_char;
    fn strcpy(dst: *mut c_char, src: *const c_char) -> *mut c_char;
    fn strdup(s: *const c_char) -> *mut c_char;
    fn strtok_r(s: *mut c_char, delim: *const c_char, saveptr: *mut *mut c_char) -> *mut c_char;
    fn strtol(nptr: *const c_char, endptr: *mut *mut c_char, base: c_int) -> c_long;
    fn strverscmp(a: *const c_char, b: *const c_char) -> c_int;
    fn sscanf(s: *const c_char, format: *const c_char, ...) -> c_int;
    fn calloc(nmemb: size_t, size: size_t) -> *mut c_void;
    fn malloc(size: size_t) -> *mut c_void;
    fn realloc(ptr: *mut c_void, size: size_t) -> *mut c_void;
    fn free(ptr: *mut c_void);
    fn memset(s: *mut c_void, c: c_int, n: size_t) -> *mut c_void;
    fn qsort(base: *mut c_void, nmemb: size_t, size: size_t, compar: Option<unsafe extern "C" fn(*const c_void, *const c_void) -> c_int>);
    fn regcomp(preg: *mut regex_t, regex: *const c_char, cflags: c_int) -> c_int;
    fn regexec(preg: *const regex_t, string: *const c_char, nmatch: size_t, pmatch: *mut regmatch_t, eflags: c_int) -> c_int;
    fn regerror(errcode: c_int, preg: *const regex_t, errbuf: *mut c_char, errbuf_size: size_t) -> size_t;
    fn regfree(preg: *mut regex_t);
    fn strerror(errnum: c_int) -> *mut c_char;
    fn fprintf(stream: *mut FILE, format: *const c_char, ...) -> c_int;
    fn printf(format: *const c_char, ...) -> c_int;
    fn fmemopen(buf: *mut c_void, size: size_t, mode: *const c_char) -> *mut FILE;
    fn fflush(stream: *mut FILE) -> c_int;
    fn fclose(stream: *mut FILE) -> c_int;

    fn ASSERT_OK_PTR(ptr: *const c_void, name: *const c_char) -> bool_;
    fn ASSERT_OK(err: c_int, name: *const c_char) -> bool_;
    fn ASSERT_ERR(err: c_int, name: *const c_char) -> bool_;
    fn ASSERT_NEQ(ptr: *const c_void, null: *const c_void, name: *const c_char) -> bool_;
    fn ASSERT_GT(ret: c_int, val: c_int, name: *const c_char) -> bool_;
    fn ASSERT_EQ(a: c_int, b: c_int, name: *const c_char) -> bool_;
    fn ASSERT_FAIL(format: *const c_char, ...) -> c_int;
    fn PRINT_FAIL(format: *const c_char, ...) -> c_int;
    fn test__start_subtest_with_desc(name: *const c_char, description: *const c_char) -> bool_;
    fn test__skip();
    fn test__end_subtest();

    fn testing_prog_flags() -> c_int;
    fn bpf_program__name(prog: *mut bpf_program) -> *const c_char;
    fn bpf_program__type(prog: *mut bpf_program) -> c_int;
    fn bpf_program__fd(prog: *mut bpf_program) -> c_int;
    fn bpf_program__flags(prog: *mut bpf_program) -> c_int;
    fn bpf_program__set_log_buf(prog: *mut bpf_program, log_buf: *mut c_char, log_buf_sz: size_t);
    fn bpf_program__set_log_level(prog: *mut bpf_program, log_level: c_int);
    fn bpf_program__set_flags(prog: *mut bpf_program, flags: c_int);
    fn bpf_program__set_autoload(prog: *mut bpf_program, autoload: bool_);
    fn bpf_object__btf(obj: *mut bpf_object) -> *mut btf;
    fn bpf_object__open_mem(obj_bytes: *const c_void, obj_byte_cnt: size_t, opts: *mut bpf_object_open_opts) -> *mut bpf_object;
    fn bpf_object__load(obj: *mut bpf_object) -> c_int;
    fn bpf_object__close(obj: *mut bpf_object);
    fn bpf_object__next_program(obj: *mut bpf_object, prog: *mut bpf_program) -> *mut bpf_program;
    fn bpf_object__next_map(obj: *mut bpf_object, map: *mut bpf_map) -> *mut bpf_map;
    fn bpf_map__type(map: *mut bpf_map) -> c_int;
    fn bpf_map__map_flags(map: *mut bpf_map) -> __u32;
    fn bpf_map__set_autocreate(map: *mut bpf_map, autocreate: bool_);
    fn bpf_map__autocreate(map: *mut bpf_map) -> bool_;
    fn bpf_map__attach_struct_ops(map: *mut bpf_map) -> *mut bpf_link;
    fn bpf_map__name(map: *mut bpf_map) -> *const c_char;
    fn bpf_link__destroy(link: *mut bpf_link);
    fn bpf_prog_test_run_opts(fd: c_int, opts: *mut bpf_test_run_opts) -> c_int;
    fn bpf_prog_stream_read(prog_fd: c_int, stream_id: c_int, text: *mut c_char, text_sz: size_t, opts: *mut bpf_prog_stream_read_opts) -> c_int;
    fn btf__type_cnt(btf: *mut btf) -> c_int;
    fn btf__type_by_id(btf: *mut btf, id: c_int) -> *const btf_type;
    fn btf__str_by_offset(btf: *mut btf, offset: __u32) -> *const c_char;
    fn btf__find_by_name_kind(btf: *mut btf, name: *const c_char, kind: c_int) -> c_int;
    fn btf__parse(path: *const c_char, opts: *mut c_void) -> *mut btf;
    fn btf__free(btf: *mut btf);
    fn btf__name_by_offset(btf: *mut btf, offset: __u32) -> *const c_char;
    fn btf__add_func_proto(btf: *mut btf, ret_type_id: __u32) -> c_int;
    fn btf__add_func_param(btf: *mut btf, name: *const c_char, type_: __u32) -> c_int;
    fn btf_is_decl_tag(t: *const btf_type) -> bool_;
    fn btf_decl_tag(t: *const btf_type) -> *const btf_decl_tag;
    fn btf_is_func(t: *const btf_type) -> bool_;
    fn btf_is_func_proto(t: *const btf_type) -> bool_;
    fn btf_vlen(t: *const btf_type) -> __u32;
    fn btf_params(t: *const btf_type) -> *const btf_param;
    fn IS_ERR(ptr: *const c_void) -> bool_;
    fn ERR_PTR(err: c_long) -> *mut c_void;
    fn PTR_ERR(ptr: *const c_void) -> c_int;
    fn get_unpriv_disabled() -> bool_;
    fn cap_disable_effective(caps_to_drop: __u64, old_caps: *mut __u64) -> c_int;
    fn cap_enable_effective(caps: __u64, old_caps: *mut __u64) -> c_int;
    fn is_jit_enabled() -> bool_;
    fn get_xlated_program(prog_fd: c_int, insn_start: *mut *mut bpf_insn, insns_cnt: *mut __u32) -> c_int;
    fn disasm_insn(insn: *mut bpf_insn, buf: *mut c_char, buf_sz: size_t) -> *mut bpf_insn;
    fn get_jited_program_text(prog_fd: c_int, text: *mut c_char, text_sz: size_t) -> c_int;
}

const VERBOSE_NONE: c_int = 0;
const VERBOSE_VERY: c_int = 1;
const BPF_F_STRICT_ALIGNMENT: c_int = 1 << 0;
const BPF_F_ANY_ALIGNMENT: c_int = 1 << 1;
const BPF_F_TEST_RND_HI32: c_int = 1 << 2;
const BPF_F_TEST_STATE_FREQ: c_int = 1 << 3;
const BPF_F_SLEEPABLE: c_int = 1 << 4;
const BPF_F_XDP_HAS_FRAGS: c_int = 1 << 5;
const BPF_F_TEST_REG_INVARIANTS: c_int = 1 << 6;
const BPF_F_ZERO_SEED: __u32 = 1 << 6;
const BPF_PROG_TYPE_SCHED_ACT: c_int = 3;
const BPF_PROG_TYPE_SCHED_CLS: c_int = 4;
const BPF_PROG_TYPE_CGROUP_SKB: c_int = 8;
const BPF_PROG_TYPE_SYSCALL: c_int = 31;
const BPF_MAP_TYPE_HASH: c_int = 1;
const BPF_MAP_TYPE_ARRAY: c_int = 2;
const BPF_MAP_TYPE_PROG_ARRAY: c_int = 3;
const BPF_MAP_TYPE_PERF_EVENT_ARRAY: c_int = 4;
const BPF_MAP_TYPE_PERCPU_HASH: c_int = 5;
const BPF_MAP_TYPE_PERCPU_ARRAY: c_int = 6;
const BPF_MAP_TYPE_CGROUP_ARRAY: c_int = 8;
const BPF_MAP_TYPE_ARRAY_OF_MAPS: c_int = 12;
const BPF_MAP_TYPE_HASH_OF_MAPS: c_int = 13;
const BPF_MAP_TYPE_CGROUP_STORAGE: c_int = 19;
const BPF_MAP_TYPE_PERCPU_CGROUP_STORAGE: c_int = 21;
const BPF_MAP_TYPE_RINGBUF: c_int = 27;
const BPF_MAP_TYPE_STRUCT_OPS: c_int = 26;
const BPF_MAP_TYPE_USER_RINGBUF: c_int = 33;

unsafe fn str_has_pfx(str_: *const c_char, pfx: *const c_char) -> *const c_char {
    let len = strlen(pfx);
    if strncmp(str_, pfx, len) == 0 {
        str_.add(len)
    } else {
        ptr::null()
    }
}

const TEST_LOADER_LOG_BUF_SZ: size_t = 2097152;

/* Warning: duplicated in bpf_misc.h */
const POINTER_VALUE: c_int = 0xbadcafe;
const TEST_DATA_LEN: usize = 64;

/* CONFIG_HAVE_EFFICIENT_UNALIGNED_ACCESS controls this in C. */
const EFFICIENT_UNALIGNED_ACCESS: c_int = 0;

static mut SYSCTL_UNPRIV_DISABLED: c_int = -1;

#[repr(C)]
enum mode {
    PRIV = 1,
    UNPRIV = 2,
}

#[repr(C)]
enum load_mode {
    JITED = 1 << 0,
    NO_JITED = 1 << 1,
}

#[repr(C)]
pub struct test_subspec {
    pub name: *mut c_char,
    pub description: *mut c_char,
    pub expect_failure: bool_,
    pub expect_msgs: expected_msgs,
    pub expect_xlated: expected_msgs,
    pub jited: expected_msgs,
    pub stderr: expected_msgs,
    pub stdout: expected_msgs,
    pub retval: c_int,
    pub execute: bool_,
    pub caps: __u64,
}

#[repr(C)]
pub struct test_spec {
    pub prog_name: *const c_char,
    pub priv_: test_subspec,
    pub unpriv: test_subspec,
    pub btf_custom_path: *const c_char,
    pub btf_custom_func_path: *const c_char,
    pub log_level: c_int,
    pub prog_flags: c_int,
    pub mode_mask: c_int,
    pub arch_mask: c_int,
    pub load_mask: c_int,
    pub linear_sz: c_int,
    pub auxiliary: bool_,
    pub valid: bool_,
}

unsafe fn tester_init(tester: *mut test_loader) -> c_int {
    if (*tester).log_buf.is_null() {
        (*tester).log_buf_sz = TEST_LOADER_LOG_BUF_SZ;
        (*tester).log_buf = calloc((*tester).log_buf_sz, 1) as *mut c_char;
        if !ASSERT_OK_PTR((*tester).log_buf as *const c_void, c"tester_log_buf".as_ptr()) {
            return -ENOMEM;
        }
    }
    0
}

#[no_mangle]
pub unsafe extern "C" fn test_loader_fini(tester: *mut test_loader) {
    if tester.is_null() {
        return;
    }
    free((*tester).log_buf as *mut c_void);
}

#[no_mangle]
pub unsafe extern "C" fn free_msgs(msgs: *mut expected_msgs) {
    let mut i = 0;
    while i < (*msgs).cnt {
        if (*(*msgs).patterns.add(i as usize)).is_regex {
            regfree(&mut (*(*msgs).patterns.add(i as usize)).regex);
        }
        i += 1;
    }
    free((*msgs).patterns as *mut c_void);
    (*msgs).patterns = ptr::null_mut();
    (*msgs).cnt = 0;
}

unsafe fn free_test_spec(spec: *mut test_spec) {
    /* Deallocate expect_msgs arrays. */
    free_msgs(&mut (*spec).priv_.expect_msgs);
    free_msgs(&mut (*spec).unpriv.expect_msgs);
    free_msgs(&mut (*spec).priv_.expect_xlated);
    free_msgs(&mut (*spec).unpriv.expect_xlated);
    free_msgs(&mut (*spec).priv_.jited);
    free_msgs(&mut (*spec).unpriv.jited);
    free_msgs(&mut (*spec).unpriv.stderr);
    free_msgs(&mut (*spec).priv_.stderr);
    free_msgs(&mut (*spec).unpriv.stdout);
    free_msgs(&mut (*spec).priv_.stdout);

    free((*spec).priv_.name as *mut c_void);
    free((*spec).priv_.description as *mut c_void);
    free((*spec).unpriv.name as *mut c_void);
    free((*spec).unpriv.description as *mut c_void);
    (*spec).priv_.name = ptr::null_mut();
    (*spec).priv_.description = ptr::null_mut();
    (*spec).unpriv.name = ptr::null_mut();
    (*spec).unpriv.description = ptr::null_mut();
}

/* Compiles regular expression matching pattern.
 * Pattern has a special syntax:
 *
 *   pattern := (<verbatim text> | regex)*
 *   regex := "{{" <posix extended regular expression> "}}"
 *
 * In other words, pattern is a verbatim text with inclusion
 * of regular expressions enclosed in "{{" "}}" pairs.
 * For example, pattern "foo{{[0-9]+}}" matches strings like
 * "foo0", "foo007", etc.
 */
unsafe fn compile_regex(mut pattern: *const c_char, regex: *mut regex_t) -> c_int {
    let mut err_buf = [0 as c_char; 256];
    let mut buf = [0 as c_char; 256];
    let original_pattern = pattern;
    let buf_end = buf.as_mut_ptr().add(buf.len());
    let mut ptr_ = buf.as_mut_ptr();
    let mut in_regex = false;
    while *pattern != 0 && ptr_ < buf_end.sub(2) {
        let next = str_has_pfx(pattern, c"{{".as_ptr());
        if !in_regex && !next.is_null() {
            in_regex = true;
            pattern = next;
            continue;
        }
        let next = str_has_pfx(pattern, c"}}".as_ptr());
        if in_regex && !next.is_null() {
            in_regex = false;
            pattern = next;
            continue;
        }
        if in_regex {
            *ptr_ = *pattern;
            ptr_ = ptr_.add(1);
            pattern = pattern.add(1);
            continue;
        }
        /* list of characters that need escaping for extended posix regex */
        if !strchr(c".[]\\()*+?{}|^$".as_ptr(), *pattern as c_int).is_null() {
            *ptr_ = b'\\' as c_char;
            ptr_ = ptr_.add(1);
            *ptr_ = *pattern;
            ptr_ = ptr_.add(1);
            pattern = pattern.add(1);
            continue;
        }
        *ptr_ = *pattern;
        ptr_ = ptr_.add(1);
        pattern = pattern.add(1);
    }
    if *pattern != 0 {
        PRINT_FAIL(c"Regexp too long: '%s'\n".as_ptr(), original_pattern);
        return -EINVAL;
    }
    if in_regex {
        PRINT_FAIL(c"Regexp has open '{{' but no closing '}}': '%s'\n".as_ptr(), original_pattern);
        return -EINVAL;
    }
    let err = regcomp(regex, buf.as_ptr(), REG_EXTENDED | REG_NEWLINE);
    if err != 0 {
        regerror(err, regex, err_buf.as_mut_ptr(), err_buf.len());
        PRINT_FAIL(c"Regexp compilation error in '%s': '%s'\n".as_ptr(), buf.as_ptr(), err_buf.as_ptr());
        return -EINVAL;
    }
    0
}

unsafe fn __push_msg(pattern: *const c_char, on_next_line: bool_, negative: bool_, msgs: *mut expected_msgs) -> c_int {
    let tmp = realloc((*msgs).patterns as *mut c_void, (1 + (*msgs).cnt as usize) * mem::size_of::<expect_msg>());
    if tmp.is_null() {
        ASSERT_FAIL(c"failed to realloc memory for messages\n".as_ptr());
        return -ENOMEM;
    }
    (*msgs).patterns = tmp as *mut expect_msg;
    let msg = (*msgs).patterns.add((*msgs).cnt as usize);
    (*msg).on_next_line = on_next_line;
    (*msg).substr = pattern;
    (*msg).negative = negative;
    (*msg).is_regex = false;
    if !strstr(pattern, c"{{".as_ptr()).is_null() {
        let err = compile_regex(pattern, &mut (*msg).regex);
        if err != 0 {
            return err;
        }
        (*msg).is_regex = true;
    }
    (*msgs).cnt += 1;
    0
}

unsafe fn clone_msgs(from: *mut expected_msgs, to: *mut expected_msgs) -> c_int {
    let mut i = 0;
    while i < (*from).cnt {
        let msg = (*from).patterns.add(i as usize);
        let err = __push_msg((*msg).substr, (*msg).on_next_line, (*msg).negative, to);
        if err != 0 {
            return err;
        }
        i += 1;
    }
    0
}

unsafe fn push_msg(substr: *const c_char, negative: bool_, msgs: *mut expected_msgs) -> c_int {
    __push_msg(substr, false, negative, msgs)
}

unsafe fn push_disasm_msg(regex_str: *const c_char, on_next_line: *mut bool_, msgs: *mut expected_msgs) -> c_int {
    if strcmp(regex_str, c"...".as_ptr()) == 0 {
        *on_next_line = false;
        return 0;
    }
    let err = __push_msg(regex_str, *on_next_line, false, msgs);
    if err != 0 {
        return err;
    }
    *on_next_line = true;
    0
}

unsafe fn parse_int(str_: *const c_char, val: *mut c_int, name: *const c_char) -> c_int {
    let mut end: *mut c_char = ptr::null_mut();
    errno = 0;
    let tmp = if !str_has_pfx(str_, c"0x".as_ptr()).is_null() {
        strtol(str_.add(2), &mut end, 16)
    } else {
        strtol(str_, &mut end, 10)
    };
    if errno != 0 || *end != 0 {
        PRINT_FAIL(c"failed to parse %s from '%s'\n".as_ptr(), name, str_);
        return -EINVAL;
    }
    *val = tmp as c_int;
    0
}

unsafe fn parse_caps(str_: *const c_char, val: *mut __u64, name: *const c_char) -> c_int {
    let mut token: *mut c_char;
    let mut saveptr: *mut c_char = ptr::null_mut();
    let str_cpy = strdup(str_);
    if str_cpy.is_null() {
        PRINT_FAIL(c"Memory allocation failed\n".as_ptr());
        return -EINVAL;
    }
    token = strtok_r(str_cpy, c"|".as_ptr(), &mut saveptr);
    while !token.is_null() {
        errno = 0;
        if strncmp(c"CAP_".as_ptr(), token, mem::size_of_val(b"CAP_") - 1) == 0 {
            PRINT_FAIL(c"define %s constant in bpf_misc.h, failed to parse caps\n".as_ptr(), token);
            return -EINVAL;
        }
        let cap_flag = strtol(token, ptr::null_mut(), 10) as c_int;
        if cap_flag == 0 || errno != 0 {
            PRINT_FAIL(c"failed to parse caps %s\n".as_ptr(), name);
            return -EINVAL;
        }
        *val |= 1u64 << cap_flag;
        token = strtok_r(ptr::null_mut(), c"|".as_ptr(), &mut saveptr);
    }
    free(str_cpy as *mut c_void);
    0
}

unsafe fn parse_retval(str_: *const c_char, val: *mut c_int, name: *const c_char) -> c_int {
    /*
     * INT_MIN is defined as (-INT_MAX -1), i.e. it doesn't expand to a
     * single int and cannot be parsed with strtol, so we handle it
     * separately here. In addition, it expands to different expressions in
     * different compilers so we use a prefixed _INT_MIN instead.
     */
    if strcmp(str_, c"_INT_MIN".as_ptr()) == 0 {
        *val = INT_MIN;
        return 0;
    }
    parse_int(str_, val, name)
}

unsafe fn update_flags(flags: *mut c_int, flag: c_int, clear: bool_) {
    if clear {
        *flags &= !flag;
    } else {
        *flags |= flag;
    }
}

unsafe fn skip_decl_tag_pfx(s: *const c_char) -> *const c_char {
    let mut n: c_int = 0;
    if sscanf(s, c"comment:%*d:%n".as_ptr(), &mut n) < 0 || n == 0 {
        return ptr::null();
    }
    s.add(n as usize)
}

unsafe extern "C" fn compare_decl_tags(a: *const c_void, b: *const c_void) -> c_int {
    strverscmp(*(a as *const *const c_char), *(b as *const *const c_char))
}

/*
 * Compilers don't guarantee order in which BTF attributes would be generated,
 * while order is important for test tags like __msg.
 * Each test tag has the following prefix: "comment:" __COUNTER__,
 * when sorted using strverscmp this gives same order as in the original C code.
 */
unsafe fn collect_decl_tags(btf: *mut btf, id: c_int, cnt: *mut c_int) -> *mut *const c_char {
    let mut tags: *mut *const c_char = ptr::null_mut();
    *cnt = 0;
    let mut i = 1;
    while i < btf__type_cnt(btf) {
        let t = btf__type_by_id(btf, i);
        if !btf_is_decl_tag(t) || (*t).type_ as c_int != id || (*btf_decl_tag(t)).component_idx != -1 {
            i += 1;
            continue;
        }
        let tmp = realloc(tags as *mut c_void, ((*cnt + 1) as usize) * mem::size_of::<*const c_char>());
        if tmp.is_null() {
            free(tags as *mut c_void);
            return ERR_PTR(-(ENOMEM as c_long)) as *mut *const c_char;
        }
        tags = tmp as *mut *const c_char;
        *tags.add(*cnt as usize) = btf__str_by_offset(btf, (*t).name_off);
        *cnt += 1;
        i += 1;
    }
    if *cnt != 0 {
        qsort(tags as *mut c_void, *cnt as size_t, mem::size_of::<*const c_char>(), Some(compare_decl_tags));
    }
    tags
}

#[repr(C)]
enum arch {
    ARCH_UNKNOWN = 0x1,
    ARCH_X86_64 = 0x2,
    ARCH_ARM64 = 0x4,
    ARCH_RISCV64 = 0x8,
    ARCH_S390X = 0x10,
    ARCH_LOONGARCH = 0x20,
}

unsafe fn get_current_arch() -> c_int {
    #[cfg(target_arch = "x86_64")]
    {
        return arch::ARCH_X86_64 as c_int;
    }
    #[cfg(target_arch = "aarch64")]
    {
        return arch::ARCH_ARM64 as c_int;
    }
    #[cfg(all(target_arch = "riscv64"))]
    {
        return arch::ARCH_RISCV64 as c_int;
    }
    #[cfg(target_arch = "s390x")]
    {
        return arch::ARCH_S390X as c_int;
    }
    #[cfg(target_arch = "loongarch64")]
    {
        return arch::ARCH_LOONGARCH as c_int;
    }
    arch::ARCH_UNKNOWN as c_int
}

/* Uses btf_decl_tag attributes to describe the expected test
 * behavior, see bpf_misc.h for detailed description of each attribute
 * and attribute combinations.
 */
unsafe fn parse_test_spec(
    _tester: *mut test_loader,
    obj: *mut bpf_object,
    prog: *mut bpf_program,
    spec: *mut test_spec,
) -> c_int {
    let mut description: *const c_char = ptr::null();
    let mut has_unpriv_result = false;
    let mut has_unpriv_retval = false;
    let mut unpriv_xlated_on_next_line = true;
    let mut xlated_on_next_line = true;
    let mut unpriv_jit_on_next_line = false;
    let mut jit_on_next_line = false;
    let mut stderr_on_next_line = true;
    let mut unpriv_stderr_on_next_line = true;
    let mut stdout_on_next_line = true;
    let mut unpriv_stdout_on_next_line = true;
    let mut collect_jit = false;
    let mut nr_tags: c_int = 0;
    let mut err: c_int = 0;
    let mut arch_mask: __u32 = 0;
    let mut load_mask: __u32 = 0;

    memset(spec as *mut c_void, 0, mem::size_of::<test_spec>());
    (*spec).prog_name = bpf_program__name(prog);
    (*spec).prog_flags = testing_prog_flags();

    let btf = bpf_object__btf(obj);
    if btf.is_null() {
        ASSERT_FAIL(c"BPF object has no BTF".as_ptr());
        return -EINVAL;
    }

    let func_id = btf__find_by_name_kind(btf, (*spec).prog_name, BTF_KIND_FUNC);
    if func_id < 0 {
        ASSERT_FAIL(c"failed to find FUNC BTF type for '%s'".as_ptr(), (*spec).prog_name);
        return -EINVAL;
    }

    let tags = collect_decl_tags(btf, func_id, &mut nr_tags);
    if IS_ERR(tags as *const c_void) {
        return PTR_ERR(tags as *const c_void);
    }

    let mut i = 0;
    while i < nr_tags {
        let s0 = skip_decl_tag_pfx(*tags.add(i as usize));
        if s0.is_null() {
            i += 1;
            continue;
        }
        let s = s0;
        let mut val = str_has_pfx(s, c"test_description=".as_ptr());
        if !val.is_null() {
            description = val;
        } else if strcmp(s, c"test_expect_failure".as_ptr()) == 0 {
            (*spec).priv_.expect_failure = true;
            (*spec).mode_mask |= mode::PRIV as c_int;
        } else if strcmp(s, c"test_expect_success".as_ptr()) == 0 {
            (*spec).priv_.expect_failure = false;
            (*spec).mode_mask |= mode::PRIV as c_int;
        } else if strcmp(s, c"test_expect_failure_unpriv".as_ptr()) == 0 {
            (*spec).unpriv.expect_failure = true;
            (*spec).mode_mask |= mode::UNPRIV as c_int;
            has_unpriv_result = true;
        } else if strcmp(s, c"test_expect_success_unpriv".as_ptr()) == 0 {
            (*spec).unpriv.expect_failure = false;
            (*spec).mode_mask |= mode::UNPRIV as c_int;
            has_unpriv_result = true;
        } else if strcmp(s, c"test_auxiliary".as_ptr()) == 0 {
            (*spec).auxiliary = true;
            (*spec).mode_mask |= mode::PRIV as c_int;
        } else if strcmp(s, c"test_auxiliary_unpriv".as_ptr()) == 0 {
            (*spec).auxiliary = true;
            (*spec).mode_mask |= mode::UNPRIV as c_int;
        } else {
            let mut msg = str_has_pfx(s, c"test_expect_msg=".as_ptr());
            if !msg.is_null() {
                err = push_msg(msg, false, &mut (*spec).priv_.expect_msgs);
                if err != 0 { break; }
                (*spec).mode_mask |= mode::PRIV as c_int;
            } else {
                msg = str_has_pfx(s, c"test_expect_not_msg=".as_ptr());
                if !msg.is_null() {
                    err = push_msg(msg, true, &mut (*spec).priv_.expect_msgs);
                    if err != 0 { break; }
                    (*spec).mode_mask |= mode::PRIV as c_int;
                } else {
                    msg = str_has_pfx(s, c"test_expect_msg_unpriv=".as_ptr());
                    if !msg.is_null() {
                        err = push_msg(msg, false, &mut (*spec).unpriv.expect_msgs);
                        if err != 0 { break; }
                        (*spec).mode_mask |= mode::UNPRIV as c_int;
                    } else {
                        msg = str_has_pfx(s, c"test_expect_not_msg_unpriv=".as_ptr());
                        if !msg.is_null() {
                            err = push_msg(msg, true, &mut (*spec).unpriv.expect_msgs);
                            if err != 0 { break; }
                            (*spec).mode_mask |= mode::UNPRIV as c_int;
                        } else {
                            msg = str_has_pfx(s, c"test_jited=".as_ptr());
                            if !msg.is_null() {
                                if arch_mask == 0 {
                                    PRINT_FAIL(c"__jited used before __arch_*".as_ptr());
                                    break;
                                }
                                if collect_jit {
                                    err = push_disasm_msg(msg, &mut jit_on_next_line, &mut (*spec).priv_.jited);
                                    if err != 0 { break; }
                                    (*spec).mode_mask |= mode::PRIV as c_int;
                                }
                            } else {
                                msg = str_has_pfx(s, c"test_jited_unpriv=".as_ptr());
                                if !msg.is_null() {
                                    if arch_mask == 0 {
                                        PRINT_FAIL(c"__unpriv_jited used before __arch_*".as_ptr());
                                        break;
                                    }
                                    if collect_jit {
                                        err = push_disasm_msg(msg, &mut unpriv_jit_on_next_line, &mut (*spec).unpriv.jited);
                                        if err != 0 { break; }
                                        (*spec).mode_mask |= mode::UNPRIV as c_int;
                                    }
                                } else {
                                    msg = str_has_pfx(s, c"test_expect_xlated=".as_ptr());
                                    if !msg.is_null() {
                                        err = push_disasm_msg(msg, &mut xlated_on_next_line, &mut (*spec).priv_.expect_xlated);
                                        if err != 0 { break; }
                                        (*spec).mode_mask |= mode::PRIV as c_int;
                                    } else {
                                        msg = str_has_pfx(s, c"test_expect_xlated_unpriv=".as_ptr());
                                        if !msg.is_null() {
                                            err = push_disasm_msg(msg, &mut unpriv_xlated_on_next_line, &mut (*spec).unpriv.expect_xlated);
                                            if err != 0 { break; }
                                            (*spec).mode_mask |= mode::UNPRIV as c_int;
                                        } else {
                                            val = str_has_pfx(s, c"test_retval=".as_ptr());
                                            if !val.is_null() {
                                                err = parse_retval(val, &mut (*spec).priv_.retval, c"__retval".as_ptr());
                                                if err != 0 { break; }
                                                (*spec).priv_.execute = true;
                                                (*spec).mode_mask |= mode::PRIV as c_int;
                                            } else {
                                                val = str_has_pfx(s, c"test_retval_unpriv=".as_ptr());
                                                if !val.is_null() {
                                                    err = parse_retval(val, &mut (*spec).unpriv.retval, c"__retval_unpriv".as_ptr());
                                                    if err != 0 { break; }
                                                    (*spec).mode_mask |= mode::UNPRIV as c_int;
                                                    (*spec).unpriv.execute = true;
                                                    has_unpriv_retval = true;
                                                } else {
                                                    val = str_has_pfx(s, c"test_log_level=".as_ptr());
                                                    if !val.is_null() {
                                                        err = parse_int(val, &mut (*spec).log_level, c"test log level".as_ptr());
                                                        if err != 0 { break; }
                                                    } else {
                                                        val = str_has_pfx(s, c"test_prog_flags=".as_ptr());
                                                        if !val.is_null() {
                                                            let clear = *val == b'!' as c_char;
                                                            if clear { val = val.add(1); }
                                                            let mut flags: c_int = 0;
                                                            if strcmp(val, c"BPF_F_STRICT_ALIGNMENT".as_ptr()) == 0 {
                                                                update_flags(&mut (*spec).prog_flags, BPF_F_STRICT_ALIGNMENT, clear);
                                                            } else if strcmp(val, c"BPF_F_ANY_ALIGNMENT".as_ptr()) == 0 {
                                                                update_flags(&mut (*spec).prog_flags, BPF_F_ANY_ALIGNMENT, clear);
                                                            } else if strcmp(val, c"BPF_F_TEST_RND_HI32".as_ptr()) == 0 {
                                                                update_flags(&mut (*spec).prog_flags, BPF_F_TEST_RND_HI32, clear);
                                                            } else if strcmp(val, c"BPF_F_TEST_STATE_FREQ".as_ptr()) == 0 {
                                                                update_flags(&mut (*spec).prog_flags, BPF_F_TEST_STATE_FREQ, clear);
                                                            } else if strcmp(val, c"BPF_F_SLEEPABLE".as_ptr()) == 0 {
                                                                update_flags(&mut (*spec).prog_flags, BPF_F_SLEEPABLE, clear);
                                                            } else if strcmp(val, c"BPF_F_XDP_HAS_FRAGS".as_ptr()) == 0 {
                                                                update_flags(&mut (*spec).prog_flags, BPF_F_XDP_HAS_FRAGS, clear);
                                                            } else if strcmp(val, c"BPF_F_TEST_REG_INVARIANTS".as_ptr()) == 0 {
                                                                update_flags(&mut (*spec).prog_flags, BPF_F_TEST_REG_INVARIANTS, clear);
                                                            } else {
                                                                err = parse_int(val, &mut flags, c"test prog flags".as_ptr());
                                                                if err != 0 { break; }
                                                                update_flags(&mut (*spec).prog_flags, flags, clear);
                                                            }
                                                        } else {
                                                            val = str_has_pfx(s, c"test_arch=".as_ptr());
                                                            if !val.is_null() {
                                                                let arch_val: c_int;
                                                                if strcmp(val, c"X86_64".as_ptr()) == 0 {
                                                                    arch_val = arch::ARCH_X86_64 as c_int;
                                                                } else if strcmp(val, c"ARM64".as_ptr()) == 0 {
                                                                    arch_val = arch::ARCH_ARM64 as c_int;
                                                                } else if strcmp(val, c"RISCV64".as_ptr()) == 0 {
                                                                    arch_val = arch::ARCH_RISCV64 as c_int;
                                                                } else if strcmp(val, c"s390x".as_ptr()) == 0 {
                                                                    arch_val = arch::ARCH_S390X as c_int;
                                                                } else if strcmp(val, c"LOONGARCH".as_ptr()) == 0 {
                                                                    arch_val = arch::ARCH_LOONGARCH as c_int;
                                                                } else {
                                                                    PRINT_FAIL(c"bad arch spec: '%s'\n".as_ptr(), val);
                                                                    err = -EINVAL;
                                                                    break;
                                                                }
                                                                arch_mask |= arch_val as __u32;
                                                                collect_jit = get_current_arch() == arch_val;
                                                                unpriv_jit_on_next_line = true;
                                                                jit_on_next_line = true;
                                                            } else {
                                                                val = str_has_pfx(s, c"test_btf_path=".as_ptr());
                                                                if !val.is_null() {
                                                                    (*spec).btf_custom_path = val;
                                                                } else {
                                                                    val = str_has_pfx(s, c"test_btf_func_path=".as_ptr());
                                                                    if !val.is_null() {
                                                                        (*spec).btf_custom_func_path = val;
                                                                    } else {
                                                                        val = str_has_pfx(s, c"test_caps_unpriv=".as_ptr());
                                                                        if !val.is_null() {
                                                                            err = parse_caps(val, &mut (*spec).unpriv.caps, c"test caps".as_ptr());
                                                                            if err != 0 { break; }
                                                                            (*spec).mode_mask |= mode::UNPRIV as c_int;
                                                                        } else {
                                                                            val = str_has_pfx(s, c"load_mode=".as_ptr());
                                                                            if !val.is_null() {
                                                                                if strcmp(val, c"jited".as_ptr()) == 0 {
                                                                                    load_mask = load_mode::JITED as __u32;
                                                                                } else if strcmp(val, c"no_jited".as_ptr()) == 0 {
                                                                                    load_mask = load_mode::NO_JITED as __u32;
                                                                                } else {
                                                                                    PRINT_FAIL(c"bad load spec: '%s'".as_ptr(), val);
                                                                                    err = -EINVAL;
                                                                                    break;
                                                                                }
                                                                            } else {
                                                                                msg = str_has_pfx(s, c"test_expect_stderr=".as_ptr());
                                                                                if !msg.is_null() {
                                                                                    err = push_disasm_msg(msg, &mut stderr_on_next_line, &mut (*spec).priv_.stderr);
                                                                                    if err != 0 { break; }
                                                                                } else {
                                                                                    msg = str_has_pfx(s, c"test_expect_stderr_unpriv=".as_ptr());
                                                                                    if !msg.is_null() {
                                                                                        err = push_disasm_msg(msg, &mut unpriv_stderr_on_next_line, &mut (*spec).unpriv.stderr);
                                                                                        if err != 0 { break; }
                                                                                    } else {
                                                                                        msg = str_has_pfx(s, c"test_expect_stdout=".as_ptr());
                                                                                        if !msg.is_null() {
                                                                                            err = push_disasm_msg(msg, &mut stdout_on_next_line, &mut (*spec).priv_.stdout);
                                                                                            if err != 0 { break; }
                                                                                        } else {
                                                                                            msg = str_has_pfx(s, c"test_expect_stdout_unpriv=".as_ptr());
                                                                                            if !msg.is_null() {
                                                                                                err = push_disasm_msg(msg, &mut unpriv_stdout_on_next_line, &mut (*spec).unpriv.stdout);
                                                                                                if err != 0 { break; }
                                                                                            } else {
                                                                                                val = str_has_pfx(s, c"test_linear_size=".as_ptr());
                                                                                                if !val.is_null() {
                                                                                                    match bpf_program__type(prog) {
                                                                                                        BPF_PROG_TYPE_SCHED_ACT | BPF_PROG_TYPE_SCHED_CLS | BPF_PROG_TYPE_CGROUP_SKB => {
                                                                                                            err = parse_int(val, &mut (*spec).linear_sz, c"test linear size".as_ptr());
                                                                                                            if err != 0 { break; }
                                                                                                        }
                                                                                                        _ => {
                                                                                                            PRINT_FAIL(c"__linear_size for unsupported program type".as_ptr());
                                                                                                            err = -EINVAL;
                                                                                                            break;
                                                                                                        }
                                                                                                    }
                                                                                                }
                                                                                            }
                                                                                        }
                                                                                    }
                                                                                }
                                                                            }
                                                                        }
                                                                    }
                                                                }
                                                            }
                                                        }
                                                    }
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
        i += 1;
    }
    if err != 0 {
        free(tags as *mut c_void);
        free_test_spec(spec);
        return err;
    }

    (*spec).arch_mask = if arch_mask != 0 { arch_mask as c_int } else { -1 };
    (*spec).load_mask = if load_mask != 0 { load_mask as c_int } else { (load_mode::JITED as c_int) | (load_mode::NO_JITED as c_int) };

    if (*spec).mode_mask == 0 {
        (*spec).mode_mask = mode::PRIV as c_int;
    }

    if ((*spec).mode_mask & mode::PRIV as c_int) != 0 {
        (*spec).priv_.name = strdup((*spec).prog_name);
        if (*spec).priv_.name.is_null() {
            PRINT_FAIL(c"failed to allocate memory for priv.name\n".as_ptr());
            err = -ENOMEM;
            free(tags as *mut c_void);
            free_test_spec(spec);
            return err;
        }
        if !description.is_null() {
            (*spec).priv_.description = strdup(description);
            if (*spec).priv_.description.is_null() {
                PRINT_FAIL(c"failed to allocate memory for priv.description\n".as_ptr());
                err = -ENOMEM;
                free(tags as *mut c_void);
                free_test_spec(spec);
                return err;
            }
        }
    }

    if ((*spec).mode_mask & mode::UNPRIV as c_int) != 0 {
        let name_len = strlen((*spec).prog_name);
        let suffix = c" @unpriv".as_ptr();
        let suffix_len = strlen(suffix);
        let name = malloc(name_len + suffix_len + 1) as *mut c_char;
        if name.is_null() {
            PRINT_FAIL(c"failed to allocate memory for unpriv.name\n".as_ptr());
            err = -ENOMEM;
            free(tags as *mut c_void);
            free_test_spec(spec);
            return err;
        }
        strcpy(name, (*spec).prog_name);
        strcpy(name.add(name_len), suffix);
        (*spec).unpriv.name = name;

        if !description.is_null() {
            let descr_len = strlen(description);
            let descr = malloc(descr_len + suffix_len + 1) as *mut c_char;
            if descr.is_null() {
                PRINT_FAIL(c"failed to allocate memory for unpriv.description\n".as_ptr());
                err = -ENOMEM;
                free(tags as *mut c_void);
                free_test_spec(spec);
                return err;
            }
            strcpy(descr, description);
            strcpy(descr.add(descr_len), suffix);
            (*spec).unpriv.description = descr;
        }
    }

    if ((*spec).mode_mask & ((mode::PRIV as c_int) | (mode::UNPRIV as c_int))) != 0 {
        if !has_unpriv_result {
            (*spec).unpriv.expect_failure = (*spec).priv_.expect_failure;
        }
        if !has_unpriv_retval {
            (*spec).unpriv.retval = (*spec).priv_.retval;
            (*spec).unpriv.execute = (*spec).priv_.execute;
        }
        if (*spec).unpriv.expect_msgs.cnt == 0 {
            clone_msgs(&mut (*spec).priv_.expect_msgs, &mut (*spec).unpriv.expect_msgs);
        }
        if (*spec).unpriv.expect_xlated.cnt == 0 {
            clone_msgs(&mut (*spec).priv_.expect_xlated, &mut (*spec).unpriv.expect_xlated);
        }
        if (*spec).unpriv.jited.cnt == 0 {
            clone_msgs(&mut (*spec).priv_.jited, &mut (*spec).unpriv.jited);
        }
        if (*spec).unpriv.stderr.cnt == 0 {
            clone_msgs(&mut (*spec).priv_.stderr, &mut (*spec).unpriv.stderr);
        }
        if (*spec).unpriv.stdout.cnt == 0 {
            clone_msgs(&mut (*spec).priv_.stdout, &mut (*spec).unpriv.stdout);
        }
    }

    (*spec).valid = true;
    free(tags as *mut c_void);
    0
}

unsafe fn prepare_case(tester: *mut test_loader, spec: *mut test_spec, _obj: *mut bpf_object, prog: *mut bpf_program) {
    let mut min_log_level = 0;
    if env.verbosity > VERBOSE_NONE {
        min_log_level = 1;
    }
    if env.verbosity > VERBOSE_VERY {
        min_log_level = 2;
    }
    bpf_program__set_log_buf(prog, (*tester).log_buf, (*tester).log_buf_sz);
    /* Make sure we set at least minimal log level, unless test requires
     * even higher level already. Make sure to preserve independent log
     * level 4 (verifier stats), though.
     */
    if ((*spec).log_level & 3) < min_log_level {
        bpf_program__set_log_level(prog, ((*spec).log_level & 4) | min_log_level);
    } else {
        bpf_program__set_log_level(prog, (*spec).log_level);
    }
    let prog_flags = bpf_program__flags(prog);
    bpf_program__set_flags(prog, prog_flags | (*spec).prog_flags);
    *(*tester).log_buf = 0;
}

unsafe fn emit_verifier_log(log_buf: *const c_char, force: bool_) {
    if !force && env.verbosity == VERBOSE_NONE { return; }
    fprintf(stdout, c"VERIFIER LOG:\n=============\n%s=============\n".as_ptr(), log_buf);
}

unsafe fn emit_xlated(xlated: *const c_char, force: bool_) {
    if !force && env.verbosity == VERBOSE_NONE { return; }
    fprintf(stdout, c"XLATED:\n=============\n%s=============\n".as_ptr(), xlated);
}

unsafe fn emit_jited(jited: *const c_char, force: bool_) {
    if !force && env.verbosity == VERBOSE_NONE { return; }
    fprintf(stdout, c"JITED:\n=============\n%s=============\n".as_ptr(), jited);
}

unsafe fn emit_stderr(stderr_: *const c_char, force: bool_) {
    if !force && env.verbosity == VERBOSE_NONE { return; }
    fprintf(stdout, c"STDERR:\n=============\n%s=============\n".as_ptr(), stderr_);
}

unsafe fn verify_stderr(prog_fd: c_int, msgs: *mut expected_msgs) {
    let mut ropts: bpf_prog_stream_read_opts = mem::zeroed();
    ropts.sz = mem::size_of::<bpf_prog_stream_read_opts>();
    if (*msgs).cnt == 0 { return; }
    let buf = malloc(TEST_LOADER_LOG_BUF_SZ) as *mut c_char;
    if !ASSERT_NEQ(buf as *const c_void, ptr::null(), c"malloc".as_ptr()) { return; }
    let ret = bpf_prog_stream_read(prog_fd, 2, buf, TEST_LOADER_LOG_BUF_SZ - 1, &mut ropts);
    if ret > 0 {
        *buf.add(ret as usize) = 0;
        emit_stderr(buf, false);
        validate_msgs(buf, msgs, Some(emit_stderr));
    } else {
        ASSERT_GT(ret, 0, c"stderr stream read".as_ptr());
    }
    free(buf as *mut c_void);
}

#[no_mangle]
pub unsafe extern "C" fn verify_test_stderr(obj: *mut bpf_object, prog: *mut bpf_program) {
    let mut spec: test_spec = mem::zeroed();
    if parse_test_spec(ptr::null_mut(), obj, prog, &mut spec) != 0 {
        return;
    }
    verify_stderr(bpf_program__fd(prog), &mut spec.priv_.stderr);
    free_test_spec(&mut spec);
}

unsafe fn emit_stdout(bpf_stdout: *const c_char, force: bool_) {
    if !force && env.verbosity == VERBOSE_NONE { return; }
    fprintf(stdout, c"STDOUT:\n=============\n%s=============\n".as_ptr(), bpf_stdout);
}

unsafe fn match_msg(msg: *mut expect_msg, log: *mut *const c_char) -> *const c_char {
    let mut match_: *const c_char = ptr::null();
    let mut reg_match = [regmatch_t { rm_so: 0, rm_eo: 0 }];
    if !(*msg).is_regex {
        match_ = strstr(*log, (*msg).substr) as *const c_char;
        if !match_.is_null() {
            *log = match_.add(strlen((*msg).substr));
        }
    } else {
        let err = regexec(&(*msg).regex, *log, 1, reg_match.as_mut_ptr(), 0);
        if err == 0 {
            match_ = (*log).add(reg_match[0].rm_so as usize);
            *log = (*log).add(reg_match[0].rm_eo as usize);
        }
    }
    match_
}

unsafe fn count_lines(start: *const c_char, end: *const c_char) -> c_int {
    let mut tmp = start;
    let mut n = 0;
    while tmp < end {
        if *tmp == b'\n' as c_char { n += 1; }
        tmp = tmp.add(1);
    }
    n
}

#[repr(C)]
struct match_ {
    start: *const c_char,
    end: *const c_char,
    line: c_int,
}

/*
 * Positive messages are matched sequentially, each next message
 * is looked for starting from the end of a previous matched one.
 */
unsafe fn match_positive_msgs(mut log: *const c_char, msgs: *mut expected_msgs, matches: *mut match_) {
    let mut prev_match = log;
    let mut line = 0;
    let mut i = 0;
    while i < (*msgs).cnt {
        let msg = (*msgs).patterns.add(i as usize);
        if (*msg).negative {
            i += 1;
            continue;
        }
        let matchp = match_msg(msg, &mut log);
        if !matchp.is_null() {
            line += count_lines(prev_match, matchp);
            (*matches.add(i as usize)).start = matchp;
            (*matches.add(i as usize)).end = log;
            (*matches.add(i as usize)).line = line;
            prev_match = matchp;
        }
        i += 1;
    }
}

/*
 * Each negative messages N located between positive messages P1 and P2
 * is matched in the span P1.end .. P2.start. Consequently, negative messages
 * are unordered within the span.
 */
unsafe fn match_negative_msgs(log: *const c_char, msgs: *mut expected_msgs, matches: *mut match_) {
    let mut start = log;
    let log_end = log.add(strlen(log));
    let mut i = 0;
    while i < (*msgs).cnt {
        let msg = (*msgs).patterns.add(i as usize);
        /* positive message bumps span start */
        if !(*msg).negative {
            let end = (*matches.add(i as usize)).end;
            if !end.is_null() {
                start = end;
            }
            i += 1;
            continue;
        }
        /* count stride of negative patterns and adjust span end */
        let mut end = log_end;
        let mut next_positive = i + 1;
        while next_positive < (*msgs).cnt {
            if !(*(*msgs).patterns.add(next_positive as usize)).negative {
                end = (*matches.add(next_positive as usize)).start;
                break;
            }
            next_positive += 1;
        }
        /* try matching negative messages within identified span */
        let mut j = i;
        while j < next_positive {
            let mut next = start;
            let matchp = match_msg(msg, &mut next);
            if !matchp.is_null() && next <= end {
                (*matches.add(j as usize)).start = matchp;
                (*matches.add(j as usize)).end = next;
            }
            j += 1;
        }
        /* -1 to account for i++ */
        i = next_positive;
    }
}

type emit_fn_t = Option<unsafe fn(*const c_char, bool_)>;

#[no_mangle]
pub unsafe extern "C" fn validate_msgs(log_buf: *const c_char, msgs: *mut expected_msgs, emit_fn: emit_fn_t) {
    let cnt = (*msgs).cnt as usize;
    let layout_sz = mem::size_of::<match_>() * cnt;
    let matches = calloc(cnt, mem::size_of::<match_>()) as *mut match_;
    let mut prev_match: *mut match_ = ptr::null_mut();
    if layout_sz == 0 {
        return;
    }
    match_positive_msgs(log_buf, msgs, matches);
    match_negative_msgs(log_buf, msgs, matches);

    let mut i = 0;
    while i < (*msgs).cnt {
        let mut msg = (*msgs).patterns.add(i as usize);
        let matchp = matches.add(i as usize);
        let no_match = !(*msg).negative && (*matchp).start.is_null();
        let wrong_line = !(*msg).negative
            && (*msg).on_next_line
            && !prev_match.is_null()
            && (*prev_match).line + 1 != (*matchp).line;
        let unexpected = (*msg).negative && !(*matchp).start.is_null();
        if no_match || wrong_line || unexpected {
            PRINT_FAIL(c"expect_msg\n".as_ptr());
            if env.verbosity == VERBOSE_NONE {
                if let Some(f) = emit_fn { f(log_buf, true); }
            }
            let mut j = 0;
            while j <= i {
                msg = (*msgs).patterns.add(j as usize);
                let pat_status = if j < i {
                    c"MATCHED   ".as_ptr()
                } else if wrong_line {
                    c"WRONG LINE".as_ptr()
                } else if no_match {
                    c"EXPECTED  ".as_ptr()
                } else {
                    c"UNEXPECTED".as_ptr()
                };
                fprintf(stderr, c"%s %s: '%s'\n".as_ptr(), pat_status, if (*msg).is_regex { c" REGEX".as_ptr() } else { c"SUBSTR".as_ptr() }, (*msg).substr);
                j += 1;
            }
            if wrong_line {
                fprintf(stderr, c"expecting match at line %d, actual match is at line %d\n".as_ptr(), (*prev_match).line + 1, (*matchp).line);
            }
            break;
        }
        if !(*msg).negative {
            prev_match = matchp;
        }
        i += 1;
    }
    free(matches as *mut c_void);
}

#[repr(C)]
struct cap_state {
    old_caps: __u64,
    initialized: bool_,
}

unsafe fn drop_capabilities(caps: *mut cap_state) -> c_int {
    let caps_to_drop: __u64 =
        (1u64 << CAP_SYS_ADMIN) | (1u64 << CAP_NET_ADMIN) | (1u64 << CAP_PERFMON) | (1u64 << CAP_BPF);
    let err = cap_disable_effective(caps_to_drop, &mut (*caps).old_caps);
    if err != 0 {
        PRINT_FAIL(c"failed to drop capabilities: %i, %s\n".as_ptr(), err, strerror(-err));
        return err;
    }
    (*caps).initialized = true;
    0
}

unsafe fn restore_capabilities(caps: *mut cap_state) -> c_int {
    if !(*caps).initialized {
        return 0;
    }
    let err = cap_enable_effective((*caps).old_caps, ptr::null_mut());
    if err != 0 {
        PRINT_FAIL(c"failed to restore capabilities: %i, %s\n".as_ptr(), err, strerror(-err));
    }
    (*caps).initialized = false;
    err
}

unsafe fn can_execute_unpriv(_tester: *mut test_loader, spec: *mut test_spec) -> bool_ {
    if SYSCTL_UNPRIV_DISABLED < 0 {
        SYSCTL_UNPRIV_DISABLED = if get_unpriv_disabled() { 1 } else { 0 };
    }
    if SYSCTL_UNPRIV_DISABLED != 0 {
        return false;
    }
    if ((*spec).prog_flags & BPF_F_ANY_ALIGNMENT) != 0 && EFFICIENT_UNALIGNED_ACCESS == 0 {
        return false;
    }
    true
}

unsafe fn is_unpriv_capable_map(map: *mut bpf_map) -> bool_ {
    let type_ = bpf_map__type(map);
    match type_ {
        BPF_MAP_TYPE_HASH | BPF_MAP_TYPE_PERCPU_HASH | BPF_MAP_TYPE_HASH_OF_MAPS => {
            let flags = bpf_map__map_flags(map);
            (flags & BPF_F_ZERO_SEED) == 0
        }
        BPF_MAP_TYPE_PERCPU_CGROUP_STORAGE
        | BPF_MAP_TYPE_ARRAY
        | BPF_MAP_TYPE_RINGBUF
        | BPF_MAP_TYPE_PROG_ARRAY
        | BPF_MAP_TYPE_CGROUP_ARRAY
        | BPF_MAP_TYPE_PERCPU_ARRAY
        | BPF_MAP_TYPE_USER_RINGBUF
        | BPF_MAP_TYPE_ARRAY_OF_MAPS
        | BPF_MAP_TYPE_CGROUP_STORAGE
        | BPF_MAP_TYPE_PERF_EVENT_ARRAY => true,
        _ => false,
    }
}

unsafe fn do_prog_test_run(fd_prog: c_int, retval: *mut c_int, empty_opts: bool_, linear_sz: c_int) -> c_int {
    let mut tmp_out = [0u8; TEST_DATA_LEN << 2];
    let mut tmp_in = [0u8; TEST_DATA_LEN];
    let mut ctx: __sk_buff = mem::zeroed();
    let mut topts: bpf_test_run_opts = mem::zeroed();
    topts.sz = mem::size_of::<bpf_test_run_opts>();
    topts.data_in = tmp_in.as_mut_ptr() as *mut c_void;
    topts.data_size_in = tmp_in.len() as __u32;
    topts.data_out = tmp_out.as_mut_ptr() as *mut c_void;
    topts.data_size_out = tmp_out.len() as __u32;
    topts.repeat = 1;

    if linear_sz != 0 {
        ctx.data_end = linear_sz as __u32;
        topts.ctx_in = &mut ctx as *mut _ as *mut c_void;
        topts.ctx_size_in = mem::size_of::<__sk_buff>() as __u32;
    }
    if empty_opts {
        memset(&mut topts as *mut _ as *mut c_void, 0, mem::size_of::<bpf_test_run_opts>());
        topts.sz = mem::size_of::<bpf_test_run_opts>();
    }
    let err = bpf_prog_test_run_opts(fd_prog, &mut topts);
    let saved_errno = errno;
    if err != 0 {
        PRINT_FAIL(c"FAIL: Unexpected bpf_prog_test_run error: %d (%s) ".as_ptr(), saved_errno, strerror(saved_errno));
        return err;
    }
    ASSERT_OK(0, c"bpf_prog_test_run".as_ptr());
    *retval = topts.retval as c_int;
    0
}

unsafe fn should_do_test_run(spec: *mut test_spec, subspec: *mut test_subspec) -> bool_ {
    if !(*subspec).execute { return false; }
    if (*subspec).expect_failure { return false; }
    if ((*spec).prog_flags & BPF_F_ANY_ALIGNMENT) != 0 && EFFICIENT_UNALIGNED_ACCESS == 0 {
        if env.verbosity != VERBOSE_NONE {
            printf(c"alignment prevents execution\n".as_ptr());
        }
        return false;
    }
    true
}

/* Get a disassembly of BPF program after verifier applies all rewrites */
unsafe fn get_xlated_program_text(prog_fd: c_int, text: *mut c_char, text_sz: size_t) -> c_int {
    let mut insn_start: *mut bpf_insn = ptr::null_mut();
    let mut insns_cnt: __u32 = 0;
    let mut buf = [0 as c_char; 64];
    let mut out: *mut FILE = ptr::null_mut();
    let mut err = get_xlated_program(prog_fd, &mut insn_start, &mut insns_cnt);
    if !ASSERT_OK(err, c"get_xlated_program".as_ptr()) {
        free(insn_start as *mut c_void);
        return err;
    }
    out = fmemopen(text as *mut c_void, text_sz, c"w".as_ptr());
    if !ASSERT_OK_PTR(out as *const c_void, c"open_memstream".as_ptr()) {
        free(insn_start as *mut c_void);
        return err;
    }
    let insn_end = insn_start.add(insns_cnt as usize);
    let mut insn = insn_start;
    while insn < insn_end {
        let i = insn.offset_from(insn_start) as __u32;
        insn = disasm_insn(insn, buf.as_mut_ptr(), buf.len());
        fprintf(out, c"%d: %s\n".as_ptr(), i, buf.as_ptr());
    }
    fflush(out);
    free(insn_start as *mut c_void);
    if !out.is_null() {
        fclose(out);
    }
    err
}

/* Read the bpf stream corresponding to the stream_id */
unsafe fn get_stream(stream_id: c_int, prog_fd: c_int, text: *mut c_char, text_sz: size_t) -> c_int {
    let mut ropts: bpf_prog_stream_read_opts = mem::zeroed();
    ropts.sz = mem::size_of::<bpf_prog_stream_read_opts>();
    let ret = bpf_prog_stream_read(prog_fd, stream_id, text, text_sz, &mut ropts);
    ASSERT_GT(ret, 0, c"stream read".as_ptr());
    *text.add(ret as usize) = 0;
    ret
}

/*
 * Fix up the program's BTF using BTF from a separate file.
 *
 * For __naked subprogs, clang drops parameter names from BTF. Find FUNC
 * entries with anonymous parameters and replace their FUNC_PROTO with the
 * properly-named version from the custom file.
 */
unsafe fn fixup_btf_from_path(obj: *mut bpf_object, path: *const c_char) -> c_int {
    let prog_btf = bpf_object__btf(obj);
    if prog_btf.is_null() { return 0; }
    let custom_btf = btf__parse(path, ptr::null_mut());
    if !ASSERT_OK_PTR(custom_btf as *const c_void, c"parse_custom_btf".as_ptr()) {
        return -EINVAL;
    }
    let cnt = btf__type_cnt(prog_btf) as __u32;
    let custom_cnt = btf__type_cnt(custom_btf) as __u32;
    let mut err = 0;
    /* Fix up FUNC entries with anonymous params.
     * Save all data from prog_btf BEFORE calling btf__add_*,
     * since those calls may reallocate the BTF data buffer
     * and invalidate any pointers obtained from btf__type_by_id.
     */
    let mut i: __u32 = 1;
    while i < cnt {
        let t = btf__type_by_id(prog_btf, i as c_int);
        if !btf_is_func(t) {
            i += 1;
            continue;
        }
        let fp = btf__type_by_id(prog_btf, (*t).type_ as c_int);
        if fp.is_null() || !btf_is_func_proto(fp) || btf_vlen(fp) == 0 {
            i += 1;
            continue;
        }
        /* Check if any param is anonymous */
        let params = btf_params(fp);
        if (*params).name_off != 0 {
            i += 1;
            continue;
        }
        /* Find matching FUNC by name in custom BTF */
        let name = btf__name_by_offset(prog_btf, (*t).name_off);
        if name.is_null() {
            i += 1;
            continue;
        }
        let mut j: __u32 = 1;
        let mut custom_t: *const btf_type = ptr::null();
        while j < custom_cnt {
            custom_t = btf__type_by_id(custom_btf, j as c_int);
            if btf_is_func(custom_t) {
                let cname = btf__name_by_offset(custom_btf, (*custom_t).name_off);
                if !cname.is_null() && strcmp(name, cname) == 0 {
                    break;
                }
            }
            j += 1;
        }
        if j >= custom_cnt {
            i += 1;
            continue;
        }
        let custom_fp = btf__type_by_id(custom_btf, (*custom_t).type_ as c_int);
        if custom_fp.is_null() || !btf_is_func_proto(custom_fp) {
            i += 1;
            continue;
        }
        let vlen = btf_vlen(fp);
        if vlen != btf_vlen(custom_fp) {
            i += 1;
            continue;
        }
        /* Save data before btf__add_* calls invalidate pointers */
        let ret_type_id = (*fp).type_;
        let prog_param_types = malloc(vlen as usize * mem::size_of::<__u32>()) as *mut __u32;
        if prog_param_types.is_null() {
            err = -ENOMEM;
            break;
        }
        j = 0;
        while j < vlen {
            *prog_param_types.add(j as usize) = (*params.add(j as usize)).type_;
            j += 1;
        }
        /* Add a new FUNC_PROTO: param names from custom, types from prog */
        let new_proto_id = btf__add_func_proto(prog_btf, ret_type_id);
        if new_proto_id < 0 {
            err = new_proto_id;
            free(prog_param_types as *mut c_void);
            break;
        }
        let custom_params = btf_params(custom_fp);
        j = 0;
        while j < vlen {
            let pname0 = btf__name_by_offset(custom_btf, (*custom_params.add(j as usize)).name_off);
            let pname = if pname0.is_null() { c"".as_ptr() } else { pname0 };
            err = btf__add_func_param(prog_btf, pname, *prog_param_types.add(j as usize));
            if err != 0 { break; }
            j += 1;
        }
        free(prog_param_types as *mut c_void);
        if err != 0 { break; }
        /* Update the FUNC to point to the new FUNC_PROTO (re-fetch
         * since btf__add_* may have reallocated the data buffer).
         */
        (*(btf__type_by_id(prog_btf, i as c_int) as *mut btf_type)).type_ = new_proto_id as __u32;
        i += 1;
    }
    btf__free(custom_btf);
    err
}

/* this function is forced noinline and has short generic name to look better
 * in test_progs output (in case of a failure)
 */
#[inline(never)]
unsafe fn run_subtest(
    tester: *mut test_loader,
    open_opts: *mut bpf_object_open_opts,
    obj_bytes: *const c_void,
    obj_byte_cnt: size_t,
    specs: *mut test_spec,
    spec: *mut test_spec,
    unpriv: bool_,
) {
    let subspec = if unpriv { &mut (*spec).unpriv as *mut test_subspec } else { &mut (*spec).priv_ as *mut test_subspec };
    let current_runtime = if is_jit_enabled() { load_mode::JITED as c_int } else { load_mode::NO_JITED as c_int };
    let mut tprog: *mut bpf_program = ptr::null_mut();
    let mut links: [*mut bpf_link; 32] = [ptr::null_mut(); 32];
    let mut caps: cap_state = mem::zeroed();
    let mut retval: c_int = 0;
    let mut links_cnt: c_int = 0;

    if !test__start_subtest_with_desc((*subspec).name, (*subspec).description) {
        return;
    }
    if (get_current_arch() & (*spec).arch_mask) == 0 {
        test__skip();
        return;
    }
    if (current_runtime & (*spec).load_mask) == 0 {
        test__skip();
        return;
    }
    if unpriv {
        if !can_execute_unpriv(tester, spec) {
            test__skip();
            test__end_subtest();
            return;
        }
        if drop_capabilities(&mut caps) != 0 {
            test__end_subtest();
            return;
        }
        if (*subspec).caps != 0 {
            let err = cap_enable_effective((*subspec).caps, ptr::null_mut());
            if err != 0 {
                PRINT_FAIL(c"failed to set capabilities: %i, %s\n".as_ptr(), err, strerror(-err));
                test__end_subtest();
                restore_capabilities(&mut caps);
                return;
            }
        }
    }

    /* Implicitly reset to NULL if next test case doesn't specify.
     * btf_custom_func_path also serves as btf_custom_path for kfunc resolution.
     */
    (*open_opts).btf_custom_path = (*spec).btf_custom_path;
    if (*open_opts).btf_custom_path.is_null() {
        (*open_opts).btf_custom_path = (*spec).btf_custom_func_path;
    }
    let tobj = bpf_object__open_mem(obj_bytes, obj_byte_cnt, open_opts);
    if !ASSERT_OK_PTR(tobj as *const c_void, c"obj_open_mem".as_ptr()) {
        test__end_subtest();
        restore_capabilities(&mut caps);
        return;
    }
    if !(*spec).btf_custom_func_path.is_null() {
        let err = fixup_btf_from_path(tobj, (*spec).btf_custom_func_path);
        if err != 0 {
            PRINT_FAIL(c"failed to fixup BTF from %s: %d\n".as_ptr(), (*spec).btf_custom_func_path, err);
            bpf_object__close(tobj);
            test__end_subtest();
            restore_capabilities(&mut caps);
            return;
        }
    }

    let mut i: c_int = 0;
    let mut tprog_iter: *mut bpf_program = ptr::null_mut();
    loop {
        tprog_iter = bpf_object__next_program(tobj, tprog_iter);
        if tprog_iter.is_null() { break; }
        let spec_iter = specs.add(i as usize);
        i += 1;
        let mut should_load = false;
        if (*spec_iter).valid {
            if strcmp(bpf_program__name(tprog_iter), (*spec).prog_name) == 0 {
                tprog = tprog_iter;
                should_load = true;
            }
            if (*spec_iter).auxiliary && ((*spec_iter).mode_mask & if unpriv { mode::UNPRIV as c_int } else { mode::PRIV as c_int }) != 0 {
                should_load = true;
            }
        }
        bpf_program__set_autoload(tprog_iter, should_load);
    }

    prepare_case(tester, spec, tobj, tprog);

    /* By default bpf_object__load() automatically creates all
     * maps declared in the skeleton. Some map types are only
     * allowed in priv mode. Disable autoload for such maps in
     * unpriv mode.
     */
    let mut map: *mut bpf_map = ptr::null_mut();
    loop {
        map = bpf_object__next_map(tobj, map);
        if map.is_null() { break; }
        bpf_map__set_autocreate(map, !unpriv || is_unpriv_capable_map(map));
    }

    let mut err = bpf_object__load(tobj);
    if (*subspec).expect_failure {
        if !ASSERT_ERR(err, c"unexpected_load_success".as_ptr()) {
            emit_verifier_log((*tester).log_buf, false);
            bpf_object__close(tobj);
            test__end_subtest();
            restore_capabilities(&mut caps);
            return;
        }
    } else if !ASSERT_OK(err, c"unexpected_load_failure".as_ptr()) {
        emit_verifier_log((*tester).log_buf, true);
        bpf_object__close(tobj);
        test__end_subtest();
        restore_capabilities(&mut caps);
        return;
    }
    emit_verifier_log((*tester).log_buf, false);
    validate_msgs((*tester).log_buf, &mut (*subspec).expect_msgs, Some(emit_verifier_log));

    /* Restore capabilities because the kernel will silently ignore requests
     * for program info (such as xlated program text) if we are not
     * bpf-capable. Also, for some reason test_verifier executes programs
     * with all capabilities restored. Do the same here.
     */
    if restore_capabilities(&mut caps) != 0 {
        bpf_object__close(tobj);
        test__end_subtest();
        return;
    }

    if (*subspec).expect_xlated.cnt != 0 {
        err = get_xlated_program_text(bpf_program__fd(tprog), (*tester).log_buf, (*tester).log_buf_sz);
        if err != 0 {
            bpf_object__close(tobj);
            test__end_subtest();
            return;
        }
        emit_xlated((*tester).log_buf, false);
        validate_msgs((*tester).log_buf, &mut (*subspec).expect_xlated, Some(emit_xlated));
    }

    if (*subspec).jited.cnt != 0 {
        err = get_jited_program_text(bpf_program__fd(tprog), (*tester).log_buf, (*tester).log_buf_sz);
        if err == -EOPNOTSUPP {
            printf(c"%s:SKIP: jited programs disassembly is not supported,\n".as_ptr(), c"run_subtest".as_ptr());
            printf(c"%s:SKIP: tests are built w/o LLVM development libs\n".as_ptr(), c"run_subtest".as_ptr());
            test__skip();
            bpf_object__close(tobj);
            test__end_subtest();
            return;
        }
        if !ASSERT_EQ(err, 0, c"get_jited_program_text".as_ptr()) {
            bpf_object__close(tobj);
            test__end_subtest();
            return;
        }
        emit_jited((*tester).log_buf, false);
        validate_msgs((*tester).log_buf, &mut (*subspec).jited, Some(emit_jited));
    }

    if should_do_test_run(spec, subspec) {
        /* Do bpf_map__attach_struct_ops() for each struct_ops map.
         * This should trigger bpf_struct_ops->reg callback on kernel side.
         */
        map = ptr::null_mut();
        loop {
            map = bpf_object__next_map(tobj, map);
            if map.is_null() { break; }
            if !bpf_map__autocreate(map) || bpf_map__type(map) != BPF_MAP_TYPE_STRUCT_OPS {
                continue;
            }
            if links_cnt as usize >= links.len() {
                PRINT_FAIL(c"too many struct_ops maps".as_ptr());
                break;
            }
            let link = bpf_map__attach_struct_ops(map);
            if link.is_null() {
                PRINT_FAIL(c"bpf_map__attach_struct_ops failed for map %s: err=%d\n".as_ptr(), bpf_map__name(map), -errno);
                break;
            }
            links[links_cnt as usize] = link;
            links_cnt += 1;
        }

        if let Some(cb) = (*tester).pre_execution_cb {
            err = cb(tobj);
            if err != 0 {
                PRINT_FAIL(c"pre_execution_cb failed: %d\n".as_ptr(), err);
            }
        }

        err = do_prog_test_run(
            bpf_program__fd(tprog),
            &mut retval,
            bpf_program__type(tprog) == BPF_PROG_TYPE_SYSCALL,
            (*spec).linear_sz,
        );
        if err == 0 && retval != (*subspec).retval && (*subspec).retval != POINTER_VALUE {
            PRINT_FAIL(c"Unexpected retval: %d != %d\n".as_ptr(), retval, (*subspec).retval);
        }

        verify_stderr(bpf_program__fd(tprog), &mut (*subspec).stderr);

        if (*subspec).stdout.cnt != 0 {
            err = get_stream(1, bpf_program__fd(tprog), (*tester).log_buf, (*tester).log_buf_sz);
            if err <= 0 {
                PRINT_FAIL(c"Unexpected retval from get_stream(): %d, errno = %d\n".as_ptr(), err, errno);
            } else {
                emit_stdout((*tester).log_buf, false);
                validate_msgs((*tester).log_buf, &mut (*subspec).stdout, Some(emit_stdout));
            }
        }

        /* redo bpf_map__attach_struct_ops for each test */
        while links_cnt > 0 {
            links_cnt -= 1;
            bpf_link__destroy(links[links_cnt as usize]);
        }
    }

    while links_cnt > 0 {
        links_cnt -= 1;
        bpf_link__destroy(links[links_cnt as usize]);
    }
    bpf_object__close(tobj);
    test__end_subtest();
    restore_capabilities(&mut caps);
}

unsafe fn process_subtest(tester: *mut test_loader, skel_name: *const c_char, elf_bytes_factory: skel_elf_bytes_fn) {
    let mut open_opts: bpf_object_open_opts = mem::zeroed();
    open_opts.sz = mem::size_of::<bpf_object_open_opts>();
    open_opts.object_name = skel_name;
    if tester_init(tester) < 0 {
        return; /* failed to initialize tester */
    }
    let mut obj_byte_cnt: size_t = 0;
    let obj_bytes = elf_bytes_factory.unwrap()(&mut obj_byte_cnt);
    let obj = bpf_object__open_mem(obj_bytes, obj_byte_cnt, &mut open_opts);
    if !ASSERT_OK_PTR(obj as *const c_void, c"obj_open_mem".as_ptr()) {
        return;
    }
    let mut nr_progs = 0;
    let mut prog: *mut bpf_program = ptr::null_mut();
    loop {
        prog = bpf_object__next_program(obj, prog);
        if prog.is_null() { break; }
        nr_progs += 1;
    }
    let specs = calloc(nr_progs as size_t, mem::size_of::<test_spec>()) as *mut test_spec;
    if !ASSERT_OK_PTR(specs as *const c_void, c"specs_alloc".as_ptr()) {
        return;
    }
    let mut i = 0;
    prog = ptr::null_mut();
    loop {
        prog = bpf_object__next_program(obj, prog);
        if prog.is_null() { break; }
        /* ignore tests for which  we can't derive test specification */
        let err = parse_test_spec(tester, obj, prog, specs.add(i as usize));
        i += 1;
        if err != 0 {
            PRINT_FAIL(c"Can't parse test spec for program '%s'\n".as_ptr(), bpf_program__name(prog));
        }
    }
    i = 0;
    prog = ptr::null_mut();
    loop {
        prog = bpf_object__next_program(obj, prog);
        if prog.is_null() { break; }
        let spec = specs.add(i as usize);
        i += 1;
        if !(*spec).valid || (*spec).auxiliary {
            continue;
        }
        if ((*spec).mode_mask & mode::PRIV as c_int) != 0 {
            run_subtest(tester, &mut open_opts, obj_bytes, obj_byte_cnt, specs, spec, false);
        }
        if ((*spec).mode_mask & mode::UNPRIV as c_int) != 0 {
            run_subtest(tester, &mut open_opts, obj_bytes, obj_byte_cnt, specs, spec, true);
        }
    }
    i = 0;
    while i < nr_progs {
        free_test_spec(specs.add(i as usize));
        i += 1;
    }
    free(specs as *mut c_void);
    bpf_object__close(obj);
}

#[no_mangle]
pub unsafe extern "C" fn test_loader__run_subtests(
    tester: *mut test_loader,
    skel_name: *const c_char,
    elf_bytes_factory: skel_elf_bytes_fn,
) {
    /* see comment in run_subtest() for why we do this function nesting */
    process_subtest(tester, skel_name, elf_bytes_factory);
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
