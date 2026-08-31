/* SPDX-License-Identifier: GPL-2.0 */

use core::ffi::{c_char, c_int, c_long, c_uint, c_ulong, c_void};

pub type u64 = u64;

#[repr(C)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum parse_opt_type {
    /* special types */
    OPTION_END = 0,
    OPTION_ARGUMENT,
    OPTION_GROUP,
    /* options with no arguments */
    OPTION_BIT,
    OPTION_BOOLEAN,
    OPTION_INCR,
    OPTION_SET_UINT,
    OPTION_SET_PTR,
    /* options with arguments (usually) */
    OPTION_STRING,
    OPTION_INTEGER,
    OPTION_LONG,
    OPTION_ULONG,
    OPTION_CALLBACK,
    OPTION_U64,
    OPTION_UINTEGER,
}

pub const PARSE_OPT_KEEP_DASHDASH: c_int = 1;
pub const PARSE_OPT_STOP_AT_NON_OPTION: c_int = 2;
pub const PARSE_OPT_KEEP_ARGV0: c_int = 4;
pub const PARSE_OPT_KEEP_UNKNOWN: c_int = 8;
pub const PARSE_OPT_NO_INTERNAL_HELP: c_int = 16;
pub const PARSE_OPT_OPTARG_ALLOW_NEXT: c_int = 32;

pub const PARSE_OPT_OPTARG: c_int = 1;
pub const PARSE_OPT_NOARG: c_int = 2;
pub const PARSE_OPT_NONEG: c_int = 4;
pub const PARSE_OPT_HIDDEN: c_int = 8;
pub const PARSE_OPT_LASTARG_DEFAULT: c_int = 16;
pub const PARSE_OPT_DISABLED: c_int = 32;
pub const PARSE_OPT_EXCLUSIVE: c_int = 64;
pub const PARSE_OPT_NOEMPTY: c_int = 128;
pub const PARSE_OPT_NOBUILD: c_int = 256;
pub const PARSE_OPT_CANSKIP: c_int = 512;
pub const PARSE_OPT_NOAUTONEG: c_int = 1024;

pub type parse_opt_cb =
    Option<unsafe extern "C" fn(*const option, *const c_char, c_int) -> c_int>;

/*
 * `type`::
 *   holds the type of the option, you must have an OPTION_END last in your
 *   array.
 *
 * `short_name`::
 *   the character to use as a short option name, '\0' if none.
 *
 * `long_name`::
 *   the long option name, without the leading dashes, NULL if none.
 *
 * `value`::
 *   stores pointers to the values to be filled.
 *
 * `argh`::
 *   token to explain the kind of argument this option wants. Keep it
 *   homogeneous across the repository.
 *
 * `help`::
 *   the short help associated to what the option does.
 *   Must never be NULL (except for OPTION_END).
 *   OPTION_GROUP uses this pointer to store the group header.
 *
 * `flags`::
 *   mask of parse_opt_option_flags.
 *   PARSE_OPT_OPTARG: says that the argument is optional (not for BOOLEANs)
 *   PARSE_OPT_NOARG: says that this option takes no argument, for CALLBACKs
 *   PARSE_OPT_NONEG: says that this option cannot be negated
 *   PARSE_OPT_HIDDEN this option is skipped in the default usage, showed in
 *                    the long one.
 *
 * `callback`::
 *   pointer to the callback to use for OPTION_CALLBACK.
 *
 * `defval`::
 *   default value to fill (*->value) with for PARSE_OPT_OPTARG.
 *   OPTION_{BIT,SET_UINT,SET_PTR} store the {mask,integer,pointer} to put in
 *   the value when met.
 *   CALLBACKS can use it like they want.
 *
 * `set`::
 *   whether an option was set by the user
 */
#[repr(C)]
#[derive(Copy, Clone)]
pub struct option {
    pub type_: parse_opt_type,
    pub short_name: c_int,
    pub long_name: *const c_char,
    pub value: *mut c_void,
    pub argh: *const c_char,
    pub help: *const c_char,
    pub build_opt: *const c_char,

    pub flags: c_int,
    pub callback: parse_opt_cb,
    pub defval: isize,
    pub set: *mut bool,
    pub data: *mut c_void,
    pub parent: *const option,
}

impl option {
    pub const fn zeroed(type_: parse_opt_type) -> Self {
        Self {
            type_,
            short_name: 0,
            long_name: core::ptr::null(),
            value: core::ptr::null_mut(),
            argh: core::ptr::null(),
            help: core::ptr::null(),
            build_opt: core::ptr::null(),
            flags: 0,
            callback: None,
            defval: 0,
            set: core::ptr::null_mut(),
            data: core::ptr::null_mut(),
            parent: core::ptr::null(),
        }
    }
}

macro_rules! check_vtype {
    ($v:expr, $type:ty) => {
        $v as $type
    };
}

macro_rules! OPT_END {
    () => {
        option::zeroed(parse_opt_type::OPTION_END)
    };
}

macro_rules! OPT_PARENT {
    ($p:expr) => {{
        let mut opt = option::zeroed(parse_opt_type::OPTION_END);
        opt.parent = $p;
        opt
    }};
}

macro_rules! OPT_ARGUMENT {
    ($l:expr, $h:expr) => {{
        let mut opt = option::zeroed(parse_opt_type::OPTION_ARGUMENT);
        opt.long_name = $l;
        opt.help = $h;
        opt
    }};
}

macro_rules! OPT_GROUP {
    ($h:expr) => {{
        let mut opt = option::zeroed(parse_opt_type::OPTION_GROUP);
        opt.help = $h;
        opt
    }};
}

macro_rules! OPT_BIT {
    ($s:expr, $l:expr, $v:expr, $h:expr, $b:expr) => {{
        let mut opt = option::zeroed(parse_opt_type::OPTION_BIT);
        opt.short_name = $s;
        opt.long_name = $l;
        opt.value = check_vtype!($v, *mut c_int) as *mut c_void;
        opt.help = $h;
        opt.defval = $b as isize;
        opt
    }};
}

macro_rules! OPT_BOOLEAN {
    ($s:expr, $l:expr, $v:expr, $h:expr) => {{
        let mut opt = option::zeroed(parse_opt_type::OPTION_BOOLEAN);
        opt.short_name = $s;
        opt.long_name = $l;
        opt.value = check_vtype!($v, *mut bool) as *mut c_void;
        opt.help = $h;
        opt
    }};
}

macro_rules! OPT_BOOLEAN_FLAG {
    ($s:expr, $l:expr, $v:expr, $h:expr, $f:expr) => {{
        let mut opt = OPT_BOOLEAN!($s, $l, $v, $h);
        opt.flags = $f;
        opt
    }};
}

macro_rules! OPT_BOOLEAN_SET {
    ($s:expr, $l:expr, $v:expr, $os:expr, $h:expr) => {{
        let mut opt = OPT_BOOLEAN!($s, $l, $v, $h);
        opt.set = check_vtype!($os, *mut bool);
        opt
    }};
}

macro_rules! OPT_INCR {
    ($s:expr, $l:expr, $v:expr, $h:expr) => {{
        let mut opt = option::zeroed(parse_opt_type::OPTION_INCR);
        opt.short_name = $s;
        opt.long_name = $l;
        opt.value = check_vtype!($v, *mut c_int) as *mut c_void;
        opt.help = $h;
        opt
    }};
}

macro_rules! OPT_SET_UINT {
    ($s:expr, $l:expr, $v:expr, $h:expr, $i:expr) => {{
        let mut opt = option::zeroed(parse_opt_type::OPTION_SET_UINT);
        opt.short_name = $s;
        opt.long_name = $l;
        opt.value = check_vtype!($v, *mut c_uint) as *mut c_void;
        opt.help = $h;
        opt.defval = $i as isize;
        opt
    }};
}

macro_rules! OPT_SET_PTR {
    ($s:expr, $l:expr, $v:expr, $h:expr, $p:expr) => {{
        let mut opt = option::zeroed(parse_opt_type::OPTION_SET_PTR);
        opt.short_name = $s;
        opt.long_name = $l;
        opt.value = $v as *mut c_void;
        opt.help = $h;
        opt.defval = $p as isize;
        opt
    }};
}

macro_rules! OPT_INTEGER {
    ($s:expr, $l:expr, $v:expr, $h:expr) => {{
        let mut opt = option::zeroed(parse_opt_type::OPTION_INTEGER);
        opt.short_name = $s;
        opt.long_name = $l;
        opt.value = check_vtype!($v, *mut c_int) as *mut c_void;
        opt.help = $h;
        opt
    }};
}

macro_rules! OPT_UINTEGER {
    ($s:expr, $l:expr, $v:expr, $h:expr) => {{
        let mut opt = option::zeroed(parse_opt_type::OPTION_UINTEGER);
        opt.short_name = $s;
        opt.long_name = $l;
        opt.value = check_vtype!($v, *mut c_uint) as *mut c_void;
        opt.help = $h;
        opt
    }};
}

macro_rules! OPT_UINTEGER_OPTARG {
    ($s:expr, $l:expr, $v:expr, $d:expr, $h:expr) => {{
        let mut opt = OPT_UINTEGER!($s, $l, $v, $h);
        opt.flags = PARSE_OPT_OPTARG;
        opt.defval = $d as isize;
        opt
    }};
}

macro_rules! OPT_LONG {
    ($s:expr, $l:expr, $v:expr, $h:expr) => {{
        let mut opt = option::zeroed(parse_opt_type::OPTION_LONG);
        opt.short_name = $s;
        opt.long_name = $l;
        opt.value = check_vtype!($v, *mut c_long) as *mut c_void;
        opt.help = $h;
        opt
    }};
}

macro_rules! OPT_ULONG {
    ($s:expr, $l:expr, $v:expr, $h:expr) => {{
        let mut opt = option::zeroed(parse_opt_type::OPTION_ULONG);
        opt.short_name = $s;
        opt.long_name = $l;
        opt.value = check_vtype!($v, *mut c_ulong) as *mut c_void;
        opt.help = $h;
        opt
    }};
}

macro_rules! OPT_U64 {
    ($s:expr, $l:expr, $v:expr, $h:expr) => {{
        let mut opt = option::zeroed(parse_opt_type::OPTION_U64);
        opt.short_name = $s;
        opt.long_name = $l;
        opt.value = check_vtype!($v, *mut u64) as *mut c_void;
        opt.help = $h;
        opt
    }};
}

macro_rules! OPT_STRING {
    ($s:expr, $l:expr, $v:expr, $a:expr, $h:expr) => {{
        let mut opt = option::zeroed(parse_opt_type::OPTION_STRING);
        opt.short_name = $s;
        opt.long_name = $l;
        opt.value = check_vtype!($v, *mut *const c_char) as *mut c_void;
        opt.argh = $a;
        opt.help = $h;
        opt
    }};
}

macro_rules! OPT_STRING_OPTARG {
    ($s:expr, $l:expr, $v:expr, $a:expr, $h:expr, $d:expr) => {{
        let mut opt = OPT_STRING!($s, $l, $v, $a, $h);
        opt.flags = PARSE_OPT_OPTARG;
        opt.defval = $d as isize;
        opt
    }};
}

macro_rules! OPT_STRING_OPTARG_SET {
    ($s:expr, $l:expr, $v:expr, $os:expr, $a:expr, $h:expr, $d:expr) => {{
        let mut opt = OPT_STRING_OPTARG!($s, $l, $v, $a, $h, $d);
        opt.set = check_vtype!($os, *mut bool);
        opt
    }};
}

macro_rules! OPT_STRING_NOEMPTY {
    ($s:expr, $l:expr, $v:expr, $a:expr, $h:expr) => {{
        let mut opt = OPT_STRING!($s, $l, $v, $a, $h);
        opt.flags = PARSE_OPT_NOEMPTY;
        opt
    }};
}

macro_rules! OPT_DATE {
    ($s:expr, $l:expr, $v:expr, $h:expr) => {{
        let mut opt = option::zeroed(parse_opt_type::OPTION_CALLBACK);
        opt.short_name = $s;
        opt.long_name = $l;
        opt.value = $v as *mut c_void;
        opt.argh = b"time\0".as_ptr() as *const c_char;
        opt.help = $h;
        opt.callback = Some(parse_opt_approxidate_cb);
        opt
    }};
}

macro_rules! OPT_CALLBACK {
    ($s:expr, $l:expr, $v:expr, $a:expr, $h:expr, $f:expr) => {{
        let mut opt = option::zeroed(parse_opt_type::OPTION_CALLBACK);
        opt.short_name = $s;
        opt.long_name = $l;
        opt.value = $v as *mut c_void;
        opt.argh = $a;
        opt.help = $h;
        opt.callback = Some($f);
        opt
    }};
}

macro_rules! OPT_CALLBACK_FLAG {
    ($s:expr, $l:expr, $v:expr, $a:expr, $h:expr, $f:expr, $fl:expr) => {{
        let mut opt = OPT_CALLBACK!($s, $l, $v, $a, $h, $f);
        opt.flags = $fl;
        opt
    }};
}

macro_rules! OPT_CALLBACK_SET {
    ($s:expr, $l:expr, $v:expr, $os:expr, $a:expr, $h:expr, $f:expr) => {{
        let mut opt = OPT_CALLBACK!($s, $l, $v, $a, $h, $f);
        opt.set = check_vtype!($os, *mut bool);
        opt
    }};
}

macro_rules! OPT_CALLBACK_NOOPT {
    ($s:expr, $l:expr, $v:expr, $a:expr, $h:expr, $f:expr) => {{
        let mut opt = OPT_CALLBACK!($s, $l, $v, $a, $h, $f);
        opt.flags = PARSE_OPT_NOARG;
        opt
    }};
}

macro_rules! OPT_CALLBACK_DEFAULT {
    ($s:expr, $l:expr, $v:expr, $a:expr, $h:expr, $f:expr, $d:expr) => {{
        let mut opt = OPT_CALLBACK!($s, $l, $v, $a, $h, $f);
        opt.defval = $d as isize;
        opt.flags = PARSE_OPT_LASTARG_DEFAULT;
        opt
    }};
}

macro_rules! OPT_CALLBACK_DEFAULT_NOOPT {
    ($s:expr, $l:expr, $v:expr, $a:expr, $h:expr, $f:expr, $d:expr) => {{
        let mut opt = OPT_CALLBACK!($s, $l, $v, $a, $h, $f);
        opt.defval = $d as isize;
        opt.flags = PARSE_OPT_LASTARG_DEFAULT | PARSE_OPT_NOARG;
        opt
    }};
}

macro_rules! OPT_CALLBACK_OPTARG {
    ($s:expr, $l:expr, $v:expr, $d:expr, $a:expr, $h:expr, $f:expr) => {{
        let mut opt = OPT_CALLBACK!($s, $l, $v, $a, $h, $f);
        opt.flags = PARSE_OPT_OPTARG;
        opt.data = $d as *mut c_void;
        opt
    }};
}

/*
 * parse_options() will filter out the processed options and leave the
 * non-option argments in argv[].
 * Returns the number of arguments left in argv[].
 *
 * NOTE: parse_options() and parse_options_subcommand() may call exit() in the
 * case of an error (or for 'special' options like --list-cmds or --list-opts).
 */
extern "C" {
    pub fn parse_options(
        argc: c_int,
        argv: *mut *const c_char,
        options: *const option,
        usagestr: *const *const c_char,
        flags: c_int,
    ) -> c_int;

    pub fn parse_options_subcommand(
        argc: c_int,
        argv: *mut *const c_char,
        options: *const option,
        subcommands: *const *const c_char,
        usagestr: *mut *const c_char,
        flags: c_int,
    ) -> c_int;

    pub fn usage_with_options(usagestr: *const *const c_char, options: *const option) -> !;

    /* C declaration used __attribute__((format(printf,3,4))) and varargs. */
    pub fn usage_with_options_msg(
        usagestr: *const *const c_char,
        options: *const option,
        fmt: *const c_char,
        ...
    ) -> !;
}

/*----- incremantal advanced APIs -----*/

pub const PARSE_OPT_HELP: c_int = -1;
pub const PARSE_OPT_DONE: c_int = 0;
pub const PARSE_OPT_LIST_OPTS: c_int = 1;
pub const PARSE_OPT_LIST_SUBCMDS: c_int = 2;
pub const PARSE_OPT_UNKNOWN: c_int = 3;

/*
 * It's okay for the caller to consume argv/argc in the usual way.
 * Other fields of that structure are private to parse-options and should not
 * be modified in any way.
 */
#[repr(C)]
#[derive(Copy, Clone)]
pub struct parse_opt_ctx_t {
    pub argv: *mut *const c_char,
    pub out: *mut *const c_char,
    pub argc: c_int,
    pub cpidx: c_int,
    pub opt: *const c_char,
    pub excl_opt: *const option,
    pub flags: c_int,
}

extern "C" {
    pub fn parse_options_usage(
        usagestr: *const *const c_char,
        opts: *const option,
        optstr: *const c_char,
        short_opt: bool,
    ) -> c_int;

    /*----- some often used options -----*/
    pub fn parse_opt_abbrev_cb(_: *const option, _: *const c_char, _: c_int) -> c_int;
    pub fn parse_opt_approxidate_cb(_: *const option, _: *const c_char, _: c_int) -> c_int;
    pub fn parse_opt_verbosity_cb(_: *const option, _: *const c_char, _: c_int) -> c_int;
}

macro_rules! OPT__VERBOSE {
    ($var:expr) => {
        OPT_BOOLEAN!(
            b'v' as c_int,
            b"verbose\0".as_ptr() as *const c_char,
            $var,
            b"be verbose\0".as_ptr() as *const c_char
        )
    };
}

macro_rules! OPT__QUIET {
    ($var:expr) => {
        OPT_BOOLEAN!(
            b'q' as c_int,
            b"quiet\0".as_ptr() as *const c_char,
            $var,
            b"be quiet\0".as_ptr() as *const c_char
        )
    };
}

macro_rules! OPT__VERBOSITY {
    ($var:expr) => {
        {
            let mut opt = option::zeroed(parse_opt_type::OPTION_CALLBACK);
            opt.short_name = b'v' as c_int;
            opt.long_name = b"verbose\0".as_ptr() as *const c_char;
            opt.value = $var as *mut c_void;
            opt.help = b"be more verbose\0".as_ptr() as *const c_char;
            opt.flags = PARSE_OPT_NOARG;
            opt.callback = Some(parse_opt_verbosity_cb);
            opt
        },
        {
            let mut opt = option::zeroed(parse_opt_type::OPTION_CALLBACK);
            opt.short_name = b'q' as c_int;
            opt.long_name = b"quiet\0".as_ptr() as *const c_char;
            opt.value = $var as *mut c_void;
            opt.help = b"be more quiet\0".as_ptr() as *const c_char;
            opt.flags = PARSE_OPT_NOARG;
            opt.callback = Some(parse_opt_verbosity_cb);
            opt
        }
    };
}

macro_rules! OPT__DRY_RUN {
    ($var:expr) => {
        OPT_BOOLEAN!(
            b'n' as c_int,
            b"dry-run\0".as_ptr() as *const c_char,
            $var,
            b"dry run\0".as_ptr() as *const c_char
        )
    };
}

macro_rules! OPT__ABBREV {
    ($var:expr) => {{
        let mut opt = option::zeroed(parse_opt_type::OPTION_CALLBACK);
        opt.long_name = b"abbrev\0".as_ptr() as *const c_char;
        opt.value = $var as *mut c_void;
        opt.argh = b"n\0".as_ptr() as *const c_char;
        opt.help = b"use <n> digits to display SHA-1s\0".as_ptr() as *const c_char;
        opt.flags = PARSE_OPT_OPTARG;
        opt.callback = Some(parse_opt_abbrev_cb);
        opt
    }};
}

extern "C" {
    pub fn parse_options_fix_filename(
        prefix: *const c_char,
        file: *const c_char,
    ) -> *const c_char;

    pub fn set_option_flag(opts: *mut option, sopt: c_int, lopt: *const c_char, flag: c_int);

    pub fn set_option_nobuild(
        opts: *mut option,
        shortopt: c_int,
        longopt: *const c_char,
        build_opt: *const c_char,
        can_skip: bool,
    );
}
