// SPDX-License-Identifier: GPL-2.0
// Translated from lib/subcmd/parse-options.c.
// Header-provided types, constants, and helpers are declared here only as
// external dependencies expected from the surrounding translated repository.

#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(dead_code)]
#![allow(improper_ctypes)]

use core::ffi::{c_char, c_int, c_long, c_uint, c_ulong, c_void};
use core::mem::{size_of, zeroed};
use core::ptr;

type size_t = usize;
type u64 = u64;

const OPT_SHORT: c_int = 1;
const OPT_UNSET: c_int = 2;

const USAGE_OPTS_WIDTH: c_int = 24;
const USAGE_GAP: c_int = 2;

const OPTION_END: c_int = 0;
const OPTION_ARGUMENT: c_int = 1;
const OPTION_GROUP: c_int = 2;
const OPTION_STRING: c_int = 3;
const OPTION_INTEGER: c_int = 4;
const OPTION_UINTEGER: c_int = 5;
const OPTION_LONG: c_int = 6;
const OPTION_ULONG: c_int = 7;
const OPTION_U64: c_int = 8;
const OPTION_CALLBACK: c_int = 9;
const OPTION_BOOLEAN: c_int = 10;
const OPTION_INCR: c_int = 11;
const OPTION_BIT: c_int = 12;
const OPTION_SET_UINT: c_int = 13;
const OPTION_SET_PTR: c_int = 14;

const PARSE_OPT_KEEP_ARGV0: c_int = 1 << 0;
const PARSE_OPT_KEEP_UNKNOWN: c_int = 1 << 1;
const PARSE_OPT_STOP_AT_NON_OPTION: c_int = 1 << 2;
const PARSE_OPT_KEEP_DASHDASH: c_int = 1 << 3;
const PARSE_OPT_NO_INTERNAL_HELP: c_int = 1 << 4;
const PARSE_OPT_OPTARG_ALLOW_NEXT: c_int = 1 << 5;

const PARSE_OPT_LASTARG_DEFAULT: c_int = 1 << 8;
const PARSE_OPT_NONEG: c_int = 1 << 9;
const PARSE_OPT_DISABLED: c_int = 1 << 10;
const PARSE_OPT_EXCLUSIVE: c_int = 1 << 11;
const PARSE_OPT_NOARG: c_int = 1 << 12;
const PARSE_OPT_OPTARG: c_int = 1 << 13;
const PARSE_OPT_NOBUILD: c_int = 1 << 14;
const PARSE_OPT_CANSKIP: c_int = 1 << 15;
const PARSE_OPT_NOEMPTY: c_int = 1 << 16;
const PARSE_OPT_NOAUTONEG: c_int = 1 << 17;
const PARSE_OPT_HIDDEN: c_int = 1 << 18;

const PARSE_OPT_HELP: c_int = -1;
const PARSE_OPT_DONE: c_int = 0;
const PARSE_OPT_LIST_OPTS: c_int = 1;
const PARSE_OPT_LIST_SUBCMDS: c_int = 2;
const PARSE_OPT_UNKNOWN: c_int = 3;

#[repr(C)]
pub struct option {
    pub type_: c_int,
    pub short_name: c_int,
    pub long_name: *const c_char,
    pub value: *mut c_void,
    pub argh: *const c_char,
    pub help: *const c_char,
    pub flags: c_int,
    pub callback: Option<unsafe extern "C" fn(*const option, *const c_char, c_int) -> c_int>,
    pub defval: isize,
    pub set: *mut bool,
    pub parent: *const option,
    pub build_opt: *const c_char,
}

#[repr(C)]
pub struct parse_opt_ctx_t {
    pub argc: c_int,
    pub argv: *mut *const c_char,
    pub out: *mut *const c_char,
    pub cpidx: c_int,
    pub opt: *const c_char,
    pub flags: c_int,
    pub excl_opt: *const option,
}

#[repr(C)]
pub struct subcmd_config_t {
    pub exec_name: *const c_char,
}

unsafe extern "C" {
    static mut stderr: *mut c_void;
    static mut subcmd_config: subcmd_config_t;

    fn fprintf(stream: *mut c_void, fmt: *const c_char, ...) -> c_int;
    fn printf(fmt: *const c_char, ...) -> c_int;
    fn snprintf(s: *mut c_char, n: size_t, fmt: *const c_char, ...) -> c_int;
    fn fputc(c: c_int, stream: *mut c_void) -> c_int;
    fn putchar(c: c_int) -> c_int;
    fn strlen(s: *const c_char) -> size_t;
    fn strncmp(s1: *const c_char, s2: *const c_char, n: size_t) -> c_int;
    fn strcmp(s1: *const c_char, s2: *const c_char) -> c_int;
    fn strchr(s: *const c_char, c: c_int) -> *const c_char;
    fn strtol(nptr: *const c_char, endptr: *mut *mut c_char, base: c_int) -> c_long;
    fn strtoul(nptr: *const c_char, endptr: *mut *mut c_char, base: c_int) -> c_ulong;
    fn strtoull(nptr: *const c_char, endptr: *mut *mut c_char, base: c_int) -> u64;
    fn memset(s: *mut c_void, c: c_int, n: size_t) -> *mut c_void;
    fn memmove(dest: *mut c_void, src: *const c_void, n: size_t) -> *mut c_void;
    fn memcpy(dest: *mut c_void, src: *const c_void, n: size_t) -> *mut c_void;
    fn realloc(ptr: *mut c_void, size: size_t) -> *mut c_void;
    fn free(ptr: *mut c_void);
    fn strdup(s: *const c_char) -> *mut c_char;
    fn qsort(base: *mut c_void, nmemb: size_t, size: size_t,
             compar: Option<unsafe extern "C" fn(*const c_void, *const c_void) -> c_int>);
    fn tolower(c: c_int) -> c_int;
    fn exit(status: c_int) -> !;
    fn strncpy(dest: *mut c_char, src: *const c_char, n: size_t) -> *mut c_char;
    fn vasprintf(strp: *mut *mut c_char, fmt: *const c_char, ap: *mut c_void) -> c_int;
    fn strcasestr(haystack: *const c_char, needle: *const c_char) -> *mut c_char;

    fn die(fmt: *const c_char, ...) -> !;
    fn astrcatf(buf: *mut *mut c_char, fmt: *const c_char, ...);
    fn astrcat(buf: *mut *mut c_char, str_: *const c_char);
    fn zfree(ptr: *mut *mut c_char);
    fn setup_pager();
}

static mut error_buf: *mut c_char = ptr::null_mut();

unsafe fn strstarts(str_: *const c_char, prefix: *const c_char) -> bool {
    strncmp(str_, prefix, strlen(prefix)) == 0
}

unsafe fn cstr_lit(bytes: &'static [u8]) -> *const c_char {
    bytes.as_ptr() as *const c_char
}

unsafe fn opterror(opt: *const option, reason: *const c_char, flags: c_int) -> c_int {
    if flags & OPT_SHORT != 0 {
        fprintf(stderr, cstr_lit(b" Error: switch `%c' %s\0"), (*opt).short_name, reason);
    } else if flags & OPT_UNSET != 0 {
        fprintf(stderr, cstr_lit(b" Error: option `no-%s' %s\0"), (*opt).long_name, reason);
    } else {
        fprintf(stderr, cstr_lit(b" Error: option `%s' %s\0"), (*opt).long_name, reason);
    }
    -1
}

unsafe fn skip_prefix(str_: *const c_char, prefix: *const c_char) -> *const c_char {
    let len = strlen(prefix);
    if strncmp(str_, prefix, len) != 0 { ptr::null() } else { str_.add(len) }
}

unsafe fn optwarning(opt: *const option, reason: *const c_char, flags: c_int) {
    if flags & OPT_SHORT != 0 {
        fprintf(stderr, cstr_lit(b" Warning: switch `%c' %s\0"), (*opt).short_name, reason);
    } else if flags & OPT_UNSET != 0 {
        fprintf(stderr, cstr_lit(b" Warning: option `no-%s' %s\0"), (*opt).long_name, reason);
    } else {
        fprintf(stderr, cstr_lit(b" Warning: option `%s' %s\0"), (*opt).long_name, reason);
    }
}

unsafe fn get_arg(p: *mut parse_opt_ctx_t, opt: *const option, flags: c_int,
                  arg: *mut *const c_char) -> c_int {
    let res: *const c_char;
    if !(*p).opt.is_null() {
        res = (*p).opt;
        (*p).opt = ptr::null();
    } else if ((*opt).flags & PARSE_OPT_LASTARG_DEFAULT) != 0
        && ((*p).argc == 1 || **(*p).argv.add(1) == b'-' as c_char)
    {
        res = (*opt).defval as *const c_char;
    } else if (*p).argc > 1 {
        (*p).argc -= 1;
        (*p).argv = (*p).argv.add(1);
        res = *(*p).argv;
    } else {
        return opterror(opt, cstr_lit(b"requires a value\0"), flags);
    }
    if !arg.is_null() {
        *arg = res;
    }
    0
}

unsafe fn get_value(p: *mut parse_opt_ctx_t, opt: *const option, flags: c_int) -> c_int {
    let mut s: *const c_char = ptr::null();
    let mut arg: *const c_char = ptr::null();
    let unset = flags & OPT_UNSET;
    let mut err: c_int;
    let mut force_defval = false;

    if unset != 0 && !(*p).opt.is_null() {
        return opterror(opt, cstr_lit(b"takes no value\0"), flags);
    }
    if unset != 0 && ((*opt).flags & PARSE_OPT_NONEG) != 0 {
        return opterror(opt, cstr_lit(b"isn't available\0"), flags);
    }
    if ((*opt).flags & PARSE_OPT_DISABLED) != 0 {
        return opterror(opt, cstr_lit(b"is not usable\0"), flags);
    }

    if ((*opt).flags & PARSE_OPT_EXCLUSIVE) != 0 {
        if !(*p).excl_opt.is_null() && (*p).excl_opt != opt {
            let mut msg = [0 as c_char; 128];
            if ((flags & OPT_SHORT) != 0 && (*(*p).excl_opt).short_name != 0)
                || (*(*p).excl_opt).long_name.is_null()
            {
                snprintf(msg.as_mut_ptr(), msg.len(), cstr_lit(b"cannot be used with switch `%c'\0"),
                         (*(*p).excl_opt).short_name);
            } else {
                snprintf(msg.as_mut_ptr(), msg.len(), cstr_lit(b"cannot be used with %s\0"),
                         (*(*p).excl_opt).long_name);
            }
            opterror(opt, msg.as_ptr(), flags);
            return -3;
        }
        (*p).excl_opt = opt;
    }

    if (flags & OPT_SHORT) == 0 && !(*p).opt.is_null() {
        match (*opt).type_ {
            OPTION_CALLBACK => {
                if ((*opt).flags & PARSE_OPT_NOARG) == 0 {
                } else {
                    return opterror(opt, cstr_lit(b"takes no value\0"), flags);
                }
            }
            OPTION_BOOLEAN | OPTION_INCR | OPTION_BIT | OPTION_SET_UINT | OPTION_SET_PTR => {
                return opterror(opt, cstr_lit(b"takes no value\0"), flags);
            }
            _ => {}
        }
    }

    if ((*opt).flags & PARSE_OPT_OPTARG) != 0 && (*p).opt.is_null() {
        if ((*p).flags & PARSE_OPT_OPTARG_ALLOW_NEXT) == 0 {
            /*
             * If the option has an optional argument, and the argument is not
             * provided in the option itself, do not attempt to get it from
             * the next argument, unless PARSE_OPT_OPTARG_ALLOW_NEXT is set.
             *
             * This prevents a non-option argument from being interpreted as an
             * optional argument of a preceding option, for example:
             *
             * $ cmd --opt val
             * -> is "val" argument of "--opt" or a separate non-option
             * argument?
             *
             * With PARSE_OPT_OPTARG_ALLOW_NEXT, "val" is interpreted as
             * the argument of "--opt", i.e. the same as "--opt=val".
             * Without PARSE_OPT_OPTARG_ALLOW_NEXT, --opt is interpreted
             * as having the default value, and "val" as a separate non-option
             * argument.
             *
             * PARSE_OPT_OPTARG_ALLOW_NEXT is useful for commands that take no
             * non-option arguments and want to allow more flexibility in
             * optional argument passing.
             */
            force_defval = true;
        }
        if (*p).argc <= 1 || *(*(*p).argv.add(1)) == b'-' as c_char {
            /*
             * If next argument is an option or does not exist,
             * use the default value.
             */
            force_defval = true;
        }
    }

    if ((*opt).flags & PARSE_OPT_NOBUILD) != 0 {
        let mut reason = [0 as c_char; 128];
        let mut noarg = false;
        err = snprintf(
            reason.as_mut_ptr(),
            reason.len(),
            if ((*opt).flags & PARSE_OPT_CANSKIP) != 0 {
                cstr_lit(b"is being ignored because %s \0")
            } else {
                cstr_lit(b"is not available because %s\0")
            },
            (*opt).build_opt,
        );
        reason[reason.len() - 1] = 0;
        if err < 0 {
            strncpy(
                reason.as_mut_ptr(),
                if ((*opt).flags & PARSE_OPT_CANSKIP) != 0 {
                    cstr_lit(b"is being ignored\0")
                } else {
                    cstr_lit(b"is not available\0")
                },
                reason.len(),
            );
        }
        if ((*opt).flags & PARSE_OPT_CANSKIP) == 0 {
            return opterror(opt, reason.as_ptr(), flags);
        }
        err = 0;
        if unset != 0 || ((*opt).flags & PARSE_OPT_NOARG) != 0 || force_defval {
            noarg = true;
        }
        match (*opt).type_ {
            OPTION_BOOLEAN | OPTION_INCR | OPTION_BIT | OPTION_SET_UINT | OPTION_SET_PTR
            | OPTION_END | OPTION_ARGUMENT | OPTION_GROUP => noarg = true,
            _ => {}
        }
        if !noarg {
            err = get_arg(p, opt, flags, ptr::null_mut());
        }
        if err != 0 {
            return err;
        }
        optwarning(opt, reason.as_ptr(), flags);
        return 0;
    }

    match (*opt).type_ {
        OPTION_BIT => {
            if unset != 0 {
                *((*opt).value as *mut c_int) &= !((*opt).defval as c_int);
            } else {
                *((*opt).value as *mut c_int) |= (*opt).defval as c_int;
            }
            0
        }
        OPTION_BOOLEAN => {
            *((*opt).value as *mut bool) = unset == 0;
            if !(*opt).set.is_null() { *(*opt).set = true; }
            0
        }
        OPTION_INCR => {
            *((*opt).value as *mut c_int) = if unset != 0 { 0 } else { *((*opt).value as *mut c_int) + 1 };
            0
        }
        OPTION_SET_UINT => {
            *((*opt).value as *mut c_uint) = if unset != 0 { 0 } else { (*opt).defval as c_uint };
            0
        }
        OPTION_SET_PTR => {
            *((*opt).value as *mut *mut c_void) = if unset != 0 { ptr::null_mut() } else { (*opt).defval as *mut c_void };
            0
        }
        OPTION_STRING => {
            err = 0;
            if unset != 0 {
                *((*opt).value as *mut *const c_char) = ptr::null();
            } else if force_defval {
                *((*opt).value as *mut *const c_char) = (*opt).defval as *const c_char;
            } else {
                err = get_arg(p, opt, flags, (*opt).value as *mut *const c_char);
            }
            if !(*opt).set.is_null() { *(*opt).set = true; }
            /* PARSE_OPT_NOEMPTY: Allow NULL but disallow empty string. */
            if ((*opt).flags & PARSE_OPT_NOEMPTY) != 0 {
                let val = *((*opt).value as *mut *const c_char);
                if val.is_null() { return err; }
                /* Similar to unset if we are given an empty string. */
                if *val == 0 {
                    *((*opt).value as *mut *const c_char) = ptr::null();
                    return 0;
                }
            }
            err
        }
        OPTION_CALLBACK => {
            if !(*opt).set.is_null() { *(*opt).set = true; }
            let cb = (*opt).callback.unwrap();
            if unset != 0 {
                return if cb(opt, ptr::null(), 1) != 0 { -1 } else { 0 };
            }
            if ((*opt).flags & PARSE_OPT_NOARG) != 0 {
                return if cb(opt, ptr::null(), 0) != 0 { -1 } else { 0 };
            }
            if force_defval {
                return if cb(opt, ptr::null(), 0) != 0 { -1 } else { 0 };
            }
            if get_arg(p, opt, flags, &mut arg) != 0 { return -1; }
            if cb(opt, arg, 0) != 0 { -1 } else { 0 }
        }
        OPTION_INTEGER => {
            if unset != 0 { *((*opt).value as *mut c_int) = 0; return 0; }
            if force_defval { *((*opt).value as *mut c_int) = (*opt).defval as c_int; return 0; }
            if get_arg(p, opt, flags, &mut arg) != 0 { return -1; }
            *((*opt).value as *mut c_int) = strtol(arg, &mut s as *mut *const c_char as *mut *mut c_char, 10) as c_int;
            if *s != 0 { return opterror(opt, cstr_lit(b"expects a numerical value\0"), flags); }
            0
        }
        OPTION_UINTEGER => {
            if unset != 0 { *((*opt).value as *mut c_uint) = 0; return 0; }
            if force_defval { *((*opt).value as *mut c_uint) = (*opt).defval as c_uint; return 0; }
            if get_arg(p, opt, flags, &mut arg) != 0 { return -1; }
            if *arg == b'-' as c_char { return opterror(opt, cstr_lit(b"expects an unsigned numerical value\0"), flags); }
            *((*opt).value as *mut c_uint) = strtol(arg, &mut s as *mut *const c_char as *mut *mut c_char, 10) as c_uint;
            if *s != 0 { return opterror(opt, cstr_lit(b"expects a numerical value\0"), flags); }
            0
        }
        OPTION_LONG => {
            if unset != 0 { *((*opt).value as *mut c_long) = 0; return 0; }
            if force_defval { *((*opt).value as *mut c_long) = (*opt).defval as c_long; return 0; }
            if get_arg(p, opt, flags, &mut arg) != 0 { return -1; }
            *((*opt).value as *mut c_long) = strtol(arg, &mut s as *mut *const c_char as *mut *mut c_char, 10);
            if *s != 0 { return opterror(opt, cstr_lit(b"expects a numerical value\0"), flags); }
            0
        }
        OPTION_ULONG => {
            if unset != 0 { *((*opt).value as *mut c_ulong) = 0; return 0; }
            if force_defval { *((*opt).value as *mut c_ulong) = (*opt).defval as c_ulong; return 0; }
            if get_arg(p, opt, flags, &mut arg) != 0 { return -1; }
            *((*opt).value as *mut c_ulong) = strtoul(arg, &mut s as *mut *const c_char as *mut *mut c_char, 10);
            if *s != 0 { return opterror(opt, cstr_lit(b"expects a numerical value\0"), flags); }
            0
        }
        OPTION_U64 => {
            if unset != 0 { *((*opt).value as *mut u64) = 0; return 0; }
            if force_defval { *((*opt).value as *mut u64) = (*opt).defval as u64; return 0; }
            if get_arg(p, opt, flags, &mut arg) != 0 { return -1; }
            if *arg == b'-' as c_char { return opterror(opt, cstr_lit(b"expects an unsigned numerical value\0"), flags); }
            *((*opt).value as *mut u64) = strtoull(arg, &mut s as *mut *const c_char as *mut *mut c_char, 10);
            if *s != 0 { return opterror(opt, cstr_lit(b"expects a numerical value\0"), flags); }
            0
        }
        _ => die(cstr_lit(b"should not happen, someone must be hit on the forehead\0")),
    }
}

unsafe fn parse_short_opt(p: *mut parse_opt_ctx_t, mut options: *const option) -> c_int {
    loop {
        while (*options).type_ != OPTION_END {
            if (*options).short_name == *(*p).opt as c_int {
                (*p).opt = if *(*p).opt.add(1) != 0 { (*p).opt.add(1) } else { ptr::null() };
                return get_value(p, options, OPT_SHORT);
            }
            options = options.add(1);
        }
        if !(*options).parent.is_null() {
            options = (*options).parent;
            continue;
        }
        return -2;
    }
}

unsafe fn parse_long_opt(p: *mut parse_opt_ctx_t, arg: *const c_char, mut options: *const option) -> c_int {
    let mut arg_end = strchr(arg, b'=' as c_int);
    let mut abbrev_option: *const option = ptr::null();
    let mut ambiguous_option: *const option = ptr::null();
    let mut abbrev_flags: c_int = 0;
    let mut ambiguous_flags: c_int = 0;
    if arg_end.is_null() { arg_end = arg.add(strlen(arg)); }
    loop {
        while (*options).type_ != OPTION_END {
            let mut flags = 0;
            let mut rest: *const c_char;
            if (*options).long_name.is_null() {
                options = options.add(1);
                continue;
            }
            rest = skip_prefix(arg, (*options).long_name);
            if (*options).type_ == OPTION_ARGUMENT {
                if rest.is_null() { options = options.add(1); continue; }
                if *rest == b'=' as c_char { return opterror(options, cstr_lit(b"takes no value\0"), flags); }
                if *rest != 0 { options = options.add(1); continue; }
                *(*p).out.add((*p).cpidx as usize) = arg.sub(2);
                (*p).cpidx += 1;
                return 0;
            }
            if rest.is_null() {
                if strstarts((*options).long_name, cstr_lit(b"no-\0")) && ((*options).flags & PARSE_OPT_NOAUTONEG) == 0 {
                    /*
                     * The long name itself starts with "no-", so
                     * accept the option without "no-" so that users
                     * do not have to enter "no-no-" to get the
                     * negation.
                     */
                    rest = skip_prefix(arg, (*options).long_name.add(3));
                    if !rest.is_null() {
                        flags |= OPT_UNSET;
                    } else if strstarts((*options).long_name.add(3), arg) {
                        flags |= OPT_UNSET;
                        if !abbrev_option.is_null() {
                            ambiguous_option = abbrev_option;
                            ambiguous_flags = abbrev_flags;
                        }
                        abbrev_option = options;
                        abbrev_flags = flags;
                        options = options.add(1);
                        continue;
                    } else {
                        rest = ptr::null();
                    }
                    if !rest.is_null() {
                        if *rest != 0 {
                            if *rest != b'=' as c_char { options = options.add(1); continue; }
                            (*p).opt = rest.add(1);
                        }
                        return get_value(p, options, flags);
                    }
                }
                /* abbreviated? */
                if strncmp((*options).long_name, arg, arg_end.offset_from(arg) as size_t) == 0 {
                    if !abbrev_option.is_null() {
                        /*
                         * If this is abbreviated, it is
                         * ambiguous. So when there is no
                         * exact match later, we need to
                         * error out.
                         */
                        ambiguous_option = abbrev_option;
                        ambiguous_flags = abbrev_flags;
                    }
                    if (flags & OPT_UNSET) == 0 && *arg_end != 0 { (*p).opt = arg_end.add(1); }
                    abbrev_option = options;
                    abbrev_flags = flags;
                    options = options.add(1);
                    continue;
                }
                /* negated and abbreviated very much? */
                if strstarts(cstr_lit(b"no-\0"), arg) && ((*options).flags & PARSE_OPT_NOAUTONEG) == 0 {
                    flags |= OPT_UNSET;
                    if !abbrev_option.is_null() {
                        ambiguous_option = abbrev_option;
                        ambiguous_flags = abbrev_flags;
                    }
                    abbrev_option = options;
                    abbrev_flags = flags;
                    options = options.add(1);
                    continue;
                }
                /* negated? */
                if strncmp(arg, cstr_lit(b"no-\0"), 3) != 0 || ((*options).flags & PARSE_OPT_NOAUTONEG) != 0 {
                    options = options.add(1);
                    continue;
                }
                flags |= OPT_UNSET;
                rest = skip_prefix(arg.add(3), (*options).long_name);
                /* abbreviated and negated? */
                if rest.is_null() && strstarts((*options).long_name, arg.add(3)) {
                    if !abbrev_option.is_null() {
                        ambiguous_option = abbrev_option;
                        ambiguous_flags = abbrev_flags;
                    }
                    abbrev_option = options;
                    abbrev_flags = flags;
                    options = options.add(1);
                    continue;
                }
                if rest.is_null() { options = options.add(1); continue; }
            }
            if *rest != 0 {
                if *rest != b'=' as c_char { options = options.add(1); continue; }
                (*p).opt = rest.add(1);
            }
            return get_value(p, options, flags);
        }
        if !ambiguous_option.is_null() {
            fprintf(stderr, cstr_lit(b" Error: Ambiguous option: %s (could be --%s%s or --%s%s)\n\0"),
                    arg,
                    if (ambiguous_flags & OPT_UNSET) != 0 { cstr_lit(b"no-\0") } else { cstr_lit(b"\0") },
                    (*ambiguous_option).long_name,
                    if (abbrev_flags & OPT_UNSET) != 0 { cstr_lit(b"no-\0") } else { cstr_lit(b"\0") },
                    (*abbrev_option).long_name);
            return -1;
        }
        if !abbrev_option.is_null() {
            return get_value(p, abbrev_option, abbrev_flags);
        }
        if !(*options).parent.is_null() {
            options = (*options).parent;
            continue;
        }
        return -2;
    }
}

unsafe fn check_typos(arg: *const c_char, mut options: *const option) {
    if strlen(arg) < 3 { return; }
    if strstarts(arg, cstr_lit(b"no-\0")) {
        fprintf(stderr, cstr_lit(b" Error: did you mean `--%s` (with two dashes ?)\n\0"), arg);
        exit(129);
    }
    while (*options).type_ != OPTION_END {
        if !(*options).long_name.is_null() && strstarts((*options).long_name, arg) {
            fprintf(stderr, cstr_lit(b" Error: did you mean `--%s` (with two dashes ?)\n\0"), arg);
            exit(129);
        }
        options = options.add(1);
    }
}

unsafe fn parse_options_start(ctx: *mut parse_opt_ctx_t, argc: c_int, argv: *mut *const c_char, flags: c_int) {
    memset(ctx as *mut c_void, 0, size_of::<parse_opt_ctx_t>());
    (*ctx).argc = argc - 1;
    (*ctx).argv = argv.add(1);
    (*ctx).out = argv;
    (*ctx).cpidx = if (flags & PARSE_OPT_KEEP_ARGV0) != 0 { 1 } else { 0 };
    (*ctx).flags = flags;
    if (flags & PARSE_OPT_KEEP_UNKNOWN) != 0 && (flags & PARSE_OPT_STOP_AT_NON_OPTION) != 0 {
        die(cstr_lit(b"STOP_AT_NON_OPTION and KEEP_UNKNOWN don't go together\0"));
    }
}

unsafe fn usage_with_options_internal(usagestr: *mut *const c_char, opts: *const option,
                                      full: c_int, ctx: *mut parse_opt_ctx_t) -> c_int;

unsafe fn parse_options_step(ctx: *mut parse_opt_ctx_t, options: *const option,
                             usagestr: *mut *const c_char) -> c_int {
    let internal_help = if ((*ctx).flags & PARSE_OPT_NO_INTERNAL_HELP) == 0 { 1 } else { 0 };
    let mut excl_short_opt = 1;
    let mut arg: *const c_char;
    /* we must reset ->opt, unknown short option leave it dangling */
    (*ctx).opt = ptr::null();
    while (*ctx).argc != 0 {
        arg = *(*ctx).argv;
        if *arg != b'-' as c_char || *arg.add(1) == 0 {
            if ((*ctx).flags & PARSE_OPT_STOP_AT_NON_OPTION) != 0 { break; }
            *(*ctx).out.add((*ctx).cpidx as usize) = *(*ctx).argv;
            (*ctx).cpidx += 1;
            (*ctx).argc -= 1;
            (*ctx).argv = (*ctx).argv.add(1);
            continue;
        }
        if *arg.add(1) != b'-' as c_char {
            arg = arg.add(1);
            (*ctx).opt = arg;
            if internal_help != 0 && *(*ctx).opt == b'h' as c_char {
                return usage_with_options_internal(usagestr, options, 0, ctx);
            }
            match parse_short_opt(ctx, options) {
                -1 => return parse_options_usage(usagestr, options, arg, true),
                -2 => {}
                -3 => { break; }
                _ => {
                    if !(*ctx).opt.is_null() { check_typos(arg, options); }
                    while !(*ctx).opt.is_null() {
                        if internal_help != 0 && *(*ctx).opt == b'h' as c_char {
                            return usage_with_options_internal(usagestr, options, 0, ctx);
                        }
                        arg = (*ctx).opt;
                        match parse_short_opt(ctx, options) {
                            -1 => return parse_options_usage(usagestr, options, arg, true),
                            -2 => {
                                /*
                                 * fake a short option thing to hide the fact that we may have
                                 * started to parse aggregated stuff
                                 *
                                 * This is leaky, too bad.
                                 */
                                *(*ctx).argv = strdup((*ctx).opt.sub(1));
                                *(*(*ctx).argv as *mut c_char) = b'-' as c_char;
                                break;
                            }
                            -3 => { break; }
                            _ => {}
                        }
                    }
                    (*ctx).argc -= 1;
                    (*ctx).argv = (*ctx).argv.add(1);
                    continue;
                }
            }
            if parse_short_opt(ctx, options) == -3 {
                break;
            }
            if ((*ctx).flags & PARSE_OPT_KEEP_UNKNOWN) == 0 { return PARSE_OPT_UNKNOWN; }
            *(*ctx).out.add((*ctx).cpidx as usize) = *(*ctx).argv;
            (*ctx).cpidx += 1;
            (*ctx).opt = ptr::null();
            (*ctx).argc -= 1;
            (*ctx).argv = (*ctx).argv.add(1);
            continue;
        }
        if *arg.add(2) == 0 {
            /* "--" */
            if ((*ctx).flags & PARSE_OPT_KEEP_DASHDASH) == 0 {
                (*ctx).argc -= 1;
                (*ctx).argv = (*ctx).argv.add(1);
            }
            break;
        }
        arg = arg.add(2);
        if internal_help != 0 && strcmp(arg, cstr_lit(b"help-all\0")) == 0 {
            return usage_with_options_internal(usagestr, options, 1, ctx);
        }
        if internal_help != 0 && strcmp(arg, cstr_lit(b"help\0")) == 0 {
            return usage_with_options_internal(usagestr, options, 0, ctx);
        }
        if strcmp(arg, cstr_lit(b"list-opts\0")) == 0 { return PARSE_OPT_LIST_OPTS; }
        if strcmp(arg, cstr_lit(b"list-cmds\0")) == 0 { return PARSE_OPT_LIST_SUBCMDS; }
        match parse_long_opt(ctx, arg, options) {
            -1 => return parse_options_usage(usagestr, options, arg, false),
            -2 => {}
            -3 => {
                excl_short_opt = 0;
                parse_options_usage(usagestr, options, arg, excl_short_opt != 0);
                if (excl_short_opt != 0 && (*(*ctx).excl_opt).short_name != 0)
                    || (*(*ctx).excl_opt).long_name.is_null()
                {
                    let mut opt = (*(*ctx).excl_opt).short_name as c_char;
                    parse_options_usage(ptr::null_mut(), options, &mut opt as *mut c_char, true);
                } else {
                    parse_options_usage(ptr::null_mut(), options, (*(*ctx).excl_opt).long_name, false);
                }
                return PARSE_OPT_HELP;
            }
            _ => {
                (*ctx).argc -= 1;
                (*ctx).argv = (*ctx).argv.add(1);
                continue;
            }
        }
        if ((*ctx).flags & PARSE_OPT_KEEP_UNKNOWN) == 0 { return PARSE_OPT_UNKNOWN; }
        *(*ctx).out.add((*ctx).cpidx as usize) = *(*ctx).argv;
        (*ctx).cpidx += 1;
        (*ctx).opt = ptr::null();
        (*ctx).argc -= 1;
        (*ctx).argv = (*ctx).argv.add(1);
    }
    PARSE_OPT_DONE
}

unsafe fn parse_options_end(ctx: *mut parse_opt_ctx_t) -> c_int {
    memmove((*ctx).out.add((*ctx).cpidx as usize) as *mut c_void,
            (*ctx).argv as *const c_void,
            ((*ctx).argc as usize) * size_of::<*const c_char>());
    *(*ctx).out.add(((*ctx).cpidx + (*ctx).argc) as usize) = ptr::null();
    (*ctx).cpidx + (*ctx).argc
}

#[no_mangle]
pub unsafe extern "C" fn parse_options_subcommand(argc: c_int, argv: *mut *const c_char,
    mut options: *const option, subcommands: *mut *const c_char,
    usagestr: *mut *const c_char, flags: c_int) -> c_int {
    let mut ctx: parse_opt_ctx_t = zeroed();
    /* build usage string if it's not provided */
    if !subcommands.is_null() && (*usagestr).is_null() {
        let mut buf: *mut c_char = ptr::null_mut();
        astrcatf(&mut buf, cstr_lit(b"%s %s [<options>] {\0"), subcmd_config.exec_name, *argv);
        let mut i = 0;
        while !(*subcommands.add(i)).is_null() {
            if i != 0 { astrcat(&mut buf, cstr_lit(b"|\0")); }
            astrcat(&mut buf, *subcommands.add(i));
            i += 1;
        }
        astrcat(&mut buf, cstr_lit(b"}\0"));
        *usagestr = buf;
    }
    parse_options_start(&mut ctx, argc, argv, flags);
    match parse_options_step(&mut ctx, options, usagestr) {
        PARSE_OPT_HELP => exit(129),
        PARSE_OPT_DONE => {}
        PARSE_OPT_LIST_OPTS => {
            while (*options).type_ != OPTION_END {
                if !(*options).long_name.is_null() {
                    printf(cstr_lit(b"--%s \0"), (*options).long_name);
                }
                options = options.add(1);
            }
            putchar(b'\n' as c_int);
            exit(130);
        }
        PARSE_OPT_LIST_SUBCMDS => {
            if !subcommands.is_null() {
                let mut i = 0;
                while !(*subcommands.add(i)).is_null() {
                    printf(cstr_lit(b"%s \0"), *subcommands.add(i));
                    i += 1;
                }
            }
            putchar(b'\n' as c_int);
            exit(130);
        }
        _ => {
            if *(*ctx.argv).add(1) == b'-' as c_char {
                astrcatf(&mut error_buf, cstr_lit(b"unknown option `%s'\0"), (*ctx.argv).add(2));
            } else {
                astrcatf(&mut error_buf, cstr_lit(b"unknown switch `%c'\0"), *ctx.opt as c_int);
            }
            usage_with_options(usagestr, options);
        }
    }
    parse_options_end(&mut ctx)
}

#[no_mangle]
pub unsafe extern "C" fn parse_options(argc: c_int, argv: *mut *const c_char,
    options: *const option, usagestr: *mut *const c_char, flags: c_int) -> c_int {
    parse_options_subcommand(argc, argv, options, ptr::null_mut(), usagestr, flags)
}

unsafe fn print_option_help(opts: *const option, full: c_int) {
    let mut pos: size_t;
    let pad: c_int;
    if (*opts).type_ == OPTION_GROUP {
        fputc(b'\n' as c_int, stderr);
        if *(*opts).help != 0 { fprintf(stderr, cstr_lit(b"%s\n\0"), (*opts).help); }
        return;
    }
    if full == 0 && ((*opts).flags & PARSE_OPT_HIDDEN) != 0 { return; }
    if ((*opts).flags & PARSE_OPT_DISABLED) != 0 { return; }
    pos = fprintf(stderr, cstr_lit(b"    \0")) as size_t;
    if (*opts).short_name != 0 {
        pos += fprintf(stderr, cstr_lit(b"-%c\0"), (*opts).short_name) as size_t;
    } else {
        pos += fprintf(stderr, cstr_lit(b"    \0")) as size_t;
    }
    if !(*opts).long_name.is_null() && (*opts).short_name != 0 {
        pos += fprintf(stderr, cstr_lit(b", \0")) as size_t;
    }
    if !(*opts).long_name.is_null() {
        pos += fprintf(stderr, cstr_lit(b"--%s\0"), (*opts).long_name) as size_t;
    }
    match (*opts).type_ {
        OPTION_ARGUMENT => {}
        OPTION_LONG | OPTION_ULONG | OPTION_U64 | OPTION_INTEGER | OPTION_UINTEGER => {
            if ((*opts).flags & PARSE_OPT_OPTARG) != 0 {
                if !(*opts).long_name.is_null() { pos += fprintf(stderr, cstr_lit(b"[=<n>]\0")) as size_t; }
                else { pos += fprintf(stderr, cstr_lit(b"[<n>]\0")) as size_t; }
            } else {
                pos += fprintf(stderr, cstr_lit(b" <n>\0")) as size_t;
            }
        }
        OPTION_CALLBACK => {
            if ((*opts).flags & PARSE_OPT_NOARG) == 0 {
                if !(*opts).argh.is_null() {
                    if ((*opts).flags & PARSE_OPT_OPTARG) != 0 {
                        if !(*opts).long_name.is_null() { pos += fprintf(stderr, cstr_lit(b"[=<%s>]\0"), (*opts).argh) as size_t; }
                        else { pos += fprintf(stderr, cstr_lit(b"[<%s>]\0"), (*opts).argh) as size_t; }
                    } else { pos += fprintf(stderr, cstr_lit(b" <%s>\0"), (*opts).argh) as size_t; }
                } else if ((*opts).flags & PARSE_OPT_OPTARG) != 0 {
                    if !(*opts).long_name.is_null() { pos += fprintf(stderr, cstr_lit(b"[=...]\0")) as size_t; }
                    else { pos += fprintf(stderr, cstr_lit(b"[...]\0")) as size_t; }
                } else { pos += fprintf(stderr, cstr_lit(b" ...\0")) as size_t; }
            }
        }
        OPTION_STRING => {
            if !(*opts).argh.is_null() {
                if ((*opts).flags & PARSE_OPT_OPTARG) != 0 {
                    if !(*opts).long_name.is_null() { pos += fprintf(stderr, cstr_lit(b"[=<%s>]\0"), (*opts).argh) as size_t; }
                    else { pos += fprintf(stderr, cstr_lit(b"[<%s>]\0"), (*opts).argh) as size_t; }
                } else { pos += fprintf(stderr, cstr_lit(b" <%s>\0"), (*opts).argh) as size_t; }
            } else if ((*opts).flags & PARSE_OPT_OPTARG) != 0 {
                if !(*opts).long_name.is_null() { pos += fprintf(stderr, cstr_lit(b"[=...]\0")) as size_t; }
                else { pos += fprintf(stderr, cstr_lit(b"[...]\0")) as size_t; }
            } else { pos += fprintf(stderr, cstr_lit(b" ...\0")) as size_t; }
        }
        _ => {}
    }
    if pos <= USAGE_OPTS_WIDTH as size_t {
        pad = USAGE_OPTS_WIDTH - pos as c_int;
    } else {
        fputc(b'\n' as c_int, stderr);
        pad = USAGE_OPTS_WIDTH;
    }
    fprintf(stderr, cstr_lit(b"%*s%s\n\0"), pad + USAGE_GAP, cstr_lit(b"\0"), (*opts).help);
    if ((*opts).flags & PARSE_OPT_NOBUILD) != 0 {
        fprintf(stderr, cstr_lit(b"%*s(not built-in because %s)\n\0"),
                USAGE_OPTS_WIDTH + USAGE_GAP, cstr_lit(b"\0"), (*opts).build_opt);
    }
}

unsafe extern "C" fn option__cmp(va: *const c_void, vb: *const c_void) -> c_int {
    let a = va as *const option;
    let b = vb as *const option;
    let mut sa = tolower((*a).short_name);
    let mut sb = tolower((*b).short_name);
    let mut ret: c_int;
    if sa == 0 { sa = b'z' as c_int + 1; }
    if sb == 0 { sb = b'z' as c_int + 1; }
    ret = sa - sb;
    if ret == 0 {
        let la = if !(*a).long_name.is_null() { (*a).long_name } else { cstr_lit(b"\0") };
        let lb = if !(*b).long_name.is_null() { (*b).long_name } else { cstr_lit(b"\0") };
        ret = strcmp(la, lb);
    }
    ret
}

unsafe fn options__order(opts: *const option) -> *mut option {
    let mut nr_opts: c_int = 0;
    let mut nr_group: c_int = 0;
    let mut nr_parent: c_int = 0;
    let mut len: c_int;
    let mut o: *const option = ptr::null();
    let mut p = opts;
    let mut opt: *mut option;
    let mut ordered: *mut option = ptr::null_mut();
    let mut group: *mut option;
    /* flatten the options that have parents */
    while !p.is_null() {
        o = p;
        while (*o).type_ != OPTION_END {
            nr_opts += 1;
            o = o.add(1);
        }
        /*
         * the length is given by the number of options plus a null
         * terminator for the last loop iteration.
         */
        len = (size_of::<option>() * (nr_opts + if (*o).parent.is_null() { 1 } else { 0 }) as usize) as c_int;
        group = realloc(ordered as *mut c_void, len as size_t) as *mut option;
        if group.is_null() { return ordered; }
        ordered = group;
        memcpy(ordered.add(nr_parent as usize) as *mut c_void, p as *const c_void,
               size_of::<option>() * (nr_opts - nr_parent) as usize);
        nr_parent = nr_opts;
        p = (*o).parent;
    }
    /* copy the last OPTION_END */
    memcpy(ordered.add(nr_opts as usize) as *mut c_void, o as *const c_void, size_of::<option>());
    /* sort each option group individually */
    group = ordered;
    opt = ordered;
    while (*opt).type_ != OPTION_END {
        if (*opt).type_ == OPTION_GROUP {
            qsort(group as *mut c_void, nr_group as size_t, size_of::<option>(), Some(option__cmp));
            group = opt.add(1);
            nr_group = 0;
            opt = opt.add(1);
            continue;
        }
        nr_group += 1;
        opt = opt.add(1);
    }
    qsort(group as *mut c_void, nr_group as size_t, size_of::<option>(), Some(option__cmp));
    ordered
}

unsafe fn option__in_argv(opt: *const option, ctx: *const parse_opt_ctx_t) -> bool {
    let mut i = 1;
    while i < (*ctx).argc {
        let arg = *(*ctx).argv.add(i as usize);
        if *arg != b'-' as c_char {
            if *arg.add(1) == 0 {
                if *arg as c_int == (*opt).short_name { return true; }
                i += 1;
                continue;
            }
            if !(*opt).long_name.is_null() && strcmp((*opt).long_name, arg) == 0 { return true; }
            if !(*opt).help.is_null() && !strcasestr((*opt).help, arg).is_null() { return true; }
            i += 1;
            continue;
        }
        if *arg.add(1) as c_int == (*opt).short_name
            || (*arg.add(1) == b'-' as c_char && !(*opt).long_name.is_null()
                && strcmp((*opt).long_name, arg.add(2)) == 0)
        {
            return true;
        }
        i += 1;
    }
    false
}

unsafe fn usage_with_options_internal(usagestr: *mut *const c_char, mut opts: *const option,
                                      full: c_int, ctx: *mut parse_opt_ctx_t) -> c_int {
    let ordered: *mut option;
    if usagestr.is_null() { return PARSE_OPT_HELP; }
    setup_pager();
    if !error_buf.is_null() {
        fprintf(stderr, cstr_lit(b"  Error: %s\n\0"), error_buf);
        zfree(&mut error_buf);
    }
    let mut u = usagestr;
    fprintf(stderr, cstr_lit(b"\n Usage: %s\n\0"), *u);
    u = u.add(1);
    while !(*u).is_null() && **u != 0 {
        fprintf(stderr, cstr_lit(b"    or: %s\n\0"), *u);
        u = u.add(1);
    }
    while !(*u).is_null() {
        fprintf(stderr, cstr_lit(b"%s%s\n\0"), if **u != 0 { cstr_lit(b"    \0") } else { cstr_lit(b"\0") }, *u);
        u = u.add(1);
    }
    if (*opts).type_ != OPTION_GROUP { fputc(b'\n' as c_int, stderr); }
    ordered = options__order(opts);
    if !ordered.is_null() { opts = ordered; }
    while (*opts).type_ != OPTION_END {
        if !ctx.is_null() && (*ctx).argc > 1 && !option__in_argv(opts, ctx) {
            opts = opts.add(1);
            continue;
        }
        print_option_help(opts, full);
        opts = opts.add(1);
    }
    fputc(b'\n' as c_int, stderr);
    free(ordered as *mut c_void);
    PARSE_OPT_HELP
}

#[no_mangle]
pub unsafe extern "C" fn usage_with_options(usagestr: *mut *const c_char, opts: *const option) -> ! {
    usage_with_options_internal(usagestr, opts, 0, ptr::null_mut());
    exit(129);
}

// C variadic function translated as a Rust variadic declaration-style function.
// The va_list mechanics depend on the surrounding FFI ABI.
#[no_mangle]
pub unsafe extern "C" fn usage_with_options_msg(usagestr: *mut *const c_char,
    opts: *const option, fmt: *const c_char, _args: ...) -> ! {
    let tmp = error_buf;
    let mut ap: *mut c_void = ptr::null_mut();
    if vasprintf(&mut error_buf, fmt, ap) == -1 {
        die(cstr_lit(b"vasprintf failed\0"));
    }
    free(tmp as *mut c_void);
    usage_with_options_internal(usagestr, opts, 0, ptr::null_mut());
    exit(129);
}

#[no_mangle]
pub unsafe extern "C" fn parse_options_usage(usagestr: *mut *const c_char, mut opts: *const option,
    optstr: *const c_char, short_opt: bool) -> c_int {
    if !usagestr.is_null() {
        let mut u = usagestr;
        fprintf(stderr, cstr_lit(b"\n Usage: %s\n\0"), *u);
        u = u.add(1);
        while !(*u).is_null() && **u != 0 {
            fprintf(stderr, cstr_lit(b"    or: %s\n\0"), *u);
            u = u.add(1);
        }
        while !(*u).is_null() {
            fprintf(stderr, cstr_lit(b"%s%s\n\0"), if **u != 0 { cstr_lit(b"    \0") } else { cstr_lit(b"\0") }, *u);
            u = u.add(1);
        }
        fputc(b'\n' as c_int, stderr);
    }
    while (*opts).type_ != OPTION_END {
        if short_opt {
            if (*opts).short_name == *optstr as c_int {
                print_option_help(opts, 0);
                break;
            }
            opts = opts.add(1);
            continue;
        }
        if (*opts).long_name.is_null() {
            opts = opts.add(1);
            continue;
        }
        if strstarts((*opts).long_name, optstr) { print_option_help(opts, 0); }
        if strstarts(cstr_lit(b"no-\0"), optstr)
            && strstarts((*opts).long_name, optstr.add(3))
            && ((*opts).flags & PARSE_OPT_NOAUTONEG) == 0
        {
            print_option_help(opts, 0);
        }
        opts = opts.add(1);
    }
    PARSE_OPT_HELP
}

#[no_mangle]
pub unsafe extern "C" fn parse_opt_verbosity_cb(opt: *const option,
    _arg: *const c_char, unset: c_int) -> c_int {
    let target = (*opt).value as *mut c_int;
    if unset != 0 {
        /* --no-quiet, --no-verbose */
        *target = 0;
    } else if (*opt).short_name == b'v' as c_int {
        if *target >= 0 { *target += 1; } else { *target = 1; }
    } else {
        if *target <= 0 { *target -= 1; } else { *target = -1; }
    }
    0
}

unsafe fn find_option(mut opts: *mut option, shortopt: c_int, longopt: *const c_char) -> *mut option {
    while (*opts).type_ != OPTION_END {
        if (shortopt != 0 && (*opts).short_name == shortopt)
            || (!(*opts).long_name.is_null() && !longopt.is_null()
                && strcmp((*opts).long_name, longopt) == 0)
        {
            return opts;
        }
        opts = opts.add(1);
    }
    ptr::null_mut()
}

#[no_mangle]
pub unsafe extern "C" fn set_option_flag(opts: *mut option, shortopt: c_int,
    longopt: *const c_char, flag: c_int) {
    let opt = find_option(opts, shortopt, longopt);
    if !opt.is_null() { (*opt).flags |= flag; }
}

#[no_mangle]
pub unsafe extern "C" fn set_option_nobuild(opts: *mut option, shortopt: c_int,
    longopt: *const c_char, build_opt: *const c_char, can_skip: bool) {
    let opt = find_option(opts, shortopt, longopt);
    if opt.is_null() { return; }
    (*opt).flags |= PARSE_OPT_NOBUILD;
    (*opt).flags |= if can_skip { PARSE_OPT_CANSKIP } else { 0 };
    (*opt).build_opt = build_opt;
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
