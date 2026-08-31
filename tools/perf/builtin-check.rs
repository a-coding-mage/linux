// SPDX-License-Identifier: GPL-2.0
//
// Translated from perf/builtin-check.c.
// C dependencies: builtin.h, color.h, util/bpf-utils.h, util/debug.h,
// util/header.h, tools/config.h, stdbool.h, stdio.h, string.h,
// subcmd/parse-options.h.

#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(dead_code)]

use core::ffi::{c_char, c_int, c_void};

#[repr(C)]
pub struct option {
    _private: [u8; 0],
}

#[repr(C)]
pub struct feature_status {
    pub name: *const c_char,
    pub macro_: *const c_char,
    pub tip: *const c_char,
    pub is_builtin: c_int,
}

unsafe extern "C" {
    static mut quiet: c_int;
    static mut stdout: *mut c_void;

    static PERF_COLOR_RED: *const c_char;
    static PERF_COLOR_GREEN: *const c_char;

    fn printf(fmt: *const c_char, ...) -> c_int;
    fn color_fprintf(fp: *mut c_void, color: *const c_char, fmt: *const c_char, ...) -> c_int;
    fn putchar(c: c_int) -> c_int;
    fn pr_err(fmt: *const c_char, ...) -> c_int;

    fn strcmp(s1: *const c_char, s2: *const c_char) -> c_int;
    fn strcasecmp(s1: *const c_char, s2: *const c_char) -> c_int;
    fn strdup(s: *const c_char) -> *mut c_char;
    fn strtok(s: *mut c_char, delim: *const c_char) -> *mut c_char;
    fn free(ptr: *mut c_void);

    fn parse_options(
        argc: c_int,
        argv: *const *const c_char,
        options: *mut option,
        usagestr: *const *const c_char,
        flags: c_int,
    ) -> c_int;
    fn parse_options_subcommand(
        argc: c_int,
        argv: *const *const c_char,
        options: *mut option,
        subcommands: *const *const c_char,
        usagestr: *mut *const c_char,
        flags: c_int,
    ) -> c_int;
    fn usage_with_options(usagestr: *const *const c_char, options: *mut option) -> !;
}

// Build-time feature macros from tools/config.h. These are supplied by the
// surrounding perf build in C; Rust-side declarations preserve the dependency.
unsafe extern "C" {
    static HAVE_AIO_SUPPORT: c_int;
    static HAVE_LIBBPF_SUPPORT: c_int;
    static HAVE_BPF_SKEL: c_int;
    static HAVE_DEBUGINFOD_SUPPORT: c_int;
    static HAVE_LIBDW_SUPPORT: c_int;
    static HAVE_DWARF_UNWIND_SUPPORT: c_int;
    static HAVE_LIBBFD_SUPPORT: c_int;
    static HAVE_BABELTRACE2_CTF_WRITER_SUPPORT: c_int;
    static HAVE_LIBBPF_STRINGS_SUPPORT: c_int;
    static HAVE_LIBCAPSTONE_SUPPORT: c_int;
    static HAVE_LIBELF_SUPPORT: c_int;
    static HAVE_LIBLLVM_SUPPORT: c_int;
    static HAVE_LIBNUMA_SUPPORT: c_int;
    static HAVE_CSTRACE_SUPPORT: c_int;
    static HAVE_LIBPERL_SUPPORT: c_int;
    static HAVE_LIBPFM: c_int;
    static HAVE_LIBPYTHON_SUPPORT: c_int;
    static HAVE_SLANG_SUPPORT: c_int;
    static HAVE_LIBTRACEEVENT: c_int;
    static HAVE_LIBUNWIND_SUPPORT: c_int;
    static HAVE_LZMA_SUPPORT: c_int;
    static HAVE_ZLIB_SUPPORT: c_int;
    static HAVE_ZSTD_SUPPORT: c_int;
    static HAVE_RUST_SUPPORT: c_int;
}

unsafe fn IS_BUILTIN(macro_: c_int) -> c_int {
    macro_
}

const NULL: *const c_char = core::ptr::null();
const NULL_MUT_CHAR: *mut c_char = core::ptr::null_mut();

static check_subcommands: [*const c_char; 2] = [c"feature".as_ptr(), NULL];

// Original C initializers:
//   OPT_BOOLEAN('q', "quiet", &quiet, "do not show any warnings or messages"),
//   OPT_END()
// and:
//   OPT_PARENT(check_options)
// These parser option macros are provided by subcmd/parse-options.h and have no
// file-local Rust equivalent without the external option layout.
static mut check_options: [option; 0] = [];
static mut check_feature_options: [option; 0] = [];

static mut check_usage: [*const c_char; 2] = [NULL, NULL];
static check_feature_usage: [*const c_char; 2] = [
    c"perf check feature <feature_list>".as_ptr(),
    NULL,
];

macro_rules! FEATURE_STATUS {
    ($name_:expr, $macro_:ident) => {
        feature_status {
            name: $name_.as_ptr(),
            macro_: stringify!($macro_).as_ptr() as *const c_char,
            tip: core::ptr::null(),
            is_builtin: unsafe { IS_BUILTIN($macro_) },
        }
    };
}

macro_rules! FEATURE_STATUS_TIP {
    ($name_:expr, $macro_:ident, $tip_:expr) => {
        feature_status {
            name: $name_.as_ptr(),
            macro_: stringify!($macro_).as_ptr() as *const c_char,
            tip: $tip_.as_ptr(),
            is_builtin: unsafe { IS_BUILTIN($macro_) },
        }
    };
}

static mut supported_features: [feature_status; 28] = [
    FEATURE_STATUS!(c"aio", HAVE_AIO_SUPPORT),
    FEATURE_STATUS!(c"bpf", HAVE_LIBBPF_SUPPORT),
    FEATURE_STATUS!(c"bpf_skeletons", HAVE_BPF_SKEL),
    FEATURE_STATUS!(c"debuginfod", HAVE_DEBUGINFOD_SUPPORT),
    FEATURE_STATUS!(c"dwarf", HAVE_LIBDW_SUPPORT),
    FEATURE_STATUS!(c"dwarf_getlocations", HAVE_LIBDW_SUPPORT),
    FEATURE_STATUS!(c"dwarf-unwind", HAVE_DWARF_UNWIND_SUPPORT),
    FEATURE_STATUS_TIP!(
        c"libbfd",
        HAVE_LIBBFD_SUPPORT,
        c"Deprecated, license incompatibility, use BUILD_NONDISTRO=1 and install binutils-dev[el]"
    ),
    FEATURE_STATUS!(
        c"babeltrace2-ctf-writer",
        HAVE_BABELTRACE2_CTF_WRITER_SUPPORT
    ),
    FEATURE_STATUS!(c"libbpf-strings", HAVE_LIBBPF_STRINGS_SUPPORT),
    FEATURE_STATUS!(c"libcapstone", HAVE_LIBCAPSTONE_SUPPORT),
    FEATURE_STATUS!(c"libdw-dwarf-unwind", HAVE_LIBDW_SUPPORT),
    FEATURE_STATUS!(c"libelf", HAVE_LIBELF_SUPPORT),
    FEATURE_STATUS!(c"libLLVM", HAVE_LIBLLVM_SUPPORT),
    FEATURE_STATUS!(c"libnuma", HAVE_LIBNUMA_SUPPORT),
    FEATURE_STATUS!(c"libopencsd", HAVE_CSTRACE_SUPPORT),
    FEATURE_STATUS_TIP!(
        c"libperl",
        HAVE_LIBPERL_SUPPORT,
        c"Deprecated, use LIBPERL=1 and install perl-ExtUtils-Embed/libperl-dev to build with it"
    ),
    FEATURE_STATUS!(c"libpfm4", HAVE_LIBPFM),
    FEATURE_STATUS!(c"libpython", HAVE_LIBPYTHON_SUPPORT),
    FEATURE_STATUS!(c"libslang", HAVE_SLANG_SUPPORT),
    FEATURE_STATUS!(c"libtraceevent", HAVE_LIBTRACEEVENT),
    FEATURE_STATUS_TIP!(
        c"libunwind",
        HAVE_LIBUNWIND_SUPPORT,
        c"Deprecated, use LIBUNWIND=1 and install libunwind-dev[el] to build with it"
    ),
    FEATURE_STATUS!(c"lzma", HAVE_LZMA_SUPPORT),
    FEATURE_STATUS!(c"numa_num_possible_cpus", HAVE_LIBNUMA_SUPPORT),
    FEATURE_STATUS!(c"zlib", HAVE_ZLIB_SUPPORT),
    FEATURE_STATUS!(c"zstd", HAVE_ZSTD_SUPPORT),
    FEATURE_STATUS!(c"rust", HAVE_RUST_SUPPORT),

    // this should remain at end, to know the array end
    feature_status {
        name: core::ptr::null(),
        macro_: c"_".as_ptr(),
        tip: core::ptr::null(),
        is_builtin: 0,
    },
];

unsafe fn on_off_print(status: *const c_char) {
    printf(c"[ ".as_ptr());

    if strcmp(status, c"OFF".as_ptr()) == 0 {
        color_fprintf(stdout, PERF_COLOR_RED, c"%-3s".as_ptr(), status);
    } else {
        color_fprintf(stdout, PERF_COLOR_GREEN, c"%-3s".as_ptr(), status);
    }

    printf(c" ]".as_ptr());
}

/* Helper function to print status of a feature along with name/macro */
#[unsafe(no_mangle)]
pub unsafe extern "C" fn feature_status__printf(feature: *const feature_status) {
    let name = (*feature).name;
    let macro_ = (*feature).macro_;
    let status = if (*feature).is_builtin != 0 {
        c"on".as_ptr()
    } else {
        c"OFF".as_ptr()
    };

    printf(c"%22s: ".as_ptr(), name);
    on_off_print(status);
    printf(c"  # %s".as_ptr(), macro_);

    if (*feature).is_builtin == 0 && !(*feature).tip.is_null() {
        printf(c" ( tip: %s )".as_ptr(), (*feature).tip);
    }

    putchar('\n' as c_int);
}

/**
 * check whether "feature" is built-in with perf
 *
 * returns:
 *    0: NOT built-in or Feature not known
 *    1: Built-in
 */
unsafe fn has_support(feature: *const c_char) -> c_int {
    let mut i: usize = 0;

    while !supported_features[i].name.is_null() {
        if strcasecmp(feature, supported_features[i].name) == 0
            || strcasecmp(feature, supported_features[i].macro_) == 0
        {
            if quiet == 0 {
                feature_status__printf(&supported_features[i] as *const feature_status);
            }
            return supported_features[i].is_builtin;
        }

        i += 1;
    }

    if quiet == 0 {
        pr_err(
            c"Unknown feature '%s', please use 'perf version --build-options' to see which ones are available.\n"
                .as_ptr(),
            feature,
        );
    }

    0
}

/**
 * Usage: 'perf check feature <feature_list>'
 *
 * <feature_list> can be a single feature name/macro, or a comma-separated list
 * of feature names/macros
 * eg. argument can be "libtraceevent" or "libtraceevent,bpf" etc
 *
 * In case of a comma-separated list, feature_enabled will be 1, only if
 * all features passed in the string are supported
 *
 * Note that argv will get modified
 */
unsafe fn subcommand_feature(mut argc: c_int, argv: *const *const c_char) -> c_int {
    let feature_list: *mut c_char;
    let mut feature_name: *mut c_char;
    let mut feature_enabled: c_int;

    argc = parse_options(
        argc,
        argv,
        check_feature_options.as_mut_ptr(),
        check_feature_usage.as_ptr(),
        0,
    );

    if argc == 0 {
        usage_with_options(check_feature_usage.as_ptr(), check_feature_options.as_mut_ptr());
    }

    if argc > 1 {
        pr_err(c"Too many arguments passed to 'perf check feature'\n".as_ptr());
        return -1;
    }

    feature_enabled = 1;
    /* feature_list is a non-const copy of 'argv[0]' */
    feature_list = strdup(*argv.add(0));
    if feature_list.is_null() {
        pr_err(c"ERROR: failed to allocate memory for feature list\n".as_ptr());
        return -1;
    }

    feature_name = strtok(feature_list, c",".as_ptr());

    while !feature_name.is_null() {
        feature_enabled &= has_support(feature_name as *const c_char);
        feature_name = strtok(NULL_MUT_CHAR, c",".as_ptr());
    }

    free(feature_list as *mut c_void);

    (feature_enabled == 0) as c_int
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn cmd_check(mut argc: c_int, argv: *const *const c_char) -> c_int {
    argc = parse_options_subcommand(
        argc,
        argv,
        check_options.as_mut_ptr(),
        check_subcommands.as_ptr(),
        check_usage.as_mut_ptr(),
        0,
    );

    if argc == 0 {
        usage_with_options(check_usage.as_ptr(), check_options.as_mut_ptr());
    }

    if strcmp(*argv.add(0), c"feature".as_ptr()) == 0 {
        return subcommand_feature(argc, argv);
    }

    /* If no subcommand matched above, print usage help */
    pr_err(c"Unknown subcommand: %s\n".as_ptr(), *argv.add(0));
    usage_with_options(check_usage.as_ptr(), check_options.as_mut_ptr());

    /* free usage string allocated by parse_options_subcommand */
    free(check_usage[0] as *mut c_void);

    0
}
