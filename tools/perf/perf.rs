/*
 * perf.c
 *
 * Performance analysis utility.
 *
 * This is the main hub from which the sub-commands (perf stat,
 * perf top, perf record, perf report, etc.) are started.
 */

#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(dead_code)]
#![allow(improper_ctypes)]

use core::ffi::{c_char, c_int, c_uint, c_void};

type FILE = c_void;
type va_list = *mut c_void;
type size_t = usize;
type time_t = isize;
type mode_t = c_uint;

#[repr(C)]
struct stat {
    st_mode: mode_t,
}

#[repr(C)]
struct option {
    long_name: *const c_char,
}

#[repr(C)]
struct cmdnames {
    alloc: c_uint,
    cnt: c_uint,
    names: *mut *mut c_void,
}

#[repr(C)]
struct cmd_struct {
    cmd: *const c_char,
    fn_: Option<unsafe extern "C" fn(c_int, *const *const c_char) -> c_int>,
    option: c_int,
}

#[repr(C)]
struct pager_config {
    cmd: *const c_char,
    val: c_int,
}

#[repr(C)]
enum libperf_print_level {
    LIBPERF_PRINT_NONE = 0,
}

const RUN_SETUP: c_int = 1 << 0;
const USE_PAGER: c_int = 1 << 1;
const ENOENT: c_int = 2;
const ERR_RUN_COMMAND_EXEC: c_int = 1;
const PERF_HTML_PATH: *const c_char = b"PERF_HTML_PATH\0".as_ptr() as *const c_char;
const PERF_PAGER_ENVIRONMENT: *const c_char = b"PERF_PAGER\0".as_ptr() as *const c_char;
const PERF_EXEC_PATH: *const c_char = b"PERF_EXEC_PATH\0".as_ptr() as *const c_char;
const EXEC_PATH_ENVIRONMENT: *const c_char = b"PERF_EXEC_PATH\0".as_ptr() as *const c_char;
const PREFIX: *const c_char = b"\0".as_ptr() as *const c_char;
const CMD_EXEC_PATH: *const c_char = b"--exec-path\0".as_ptr() as *const c_char;
const CMD_DEBUGFS_DIR: *const c_char = b"--debugfs-dir=\0".as_ptr() as *const c_char;

static mut use_pager: c_int = -1;
static mut debug_fp: *mut FILE = core::ptr::null_mut();

unsafe extern "C" {
    static mut stderr: *mut FILE;
    static mut stdout: *mut FILE;
    static mut errno: c_int;
    static mut verbose: c_int;
    static mut use_browser: c_int;
    static mut config_exclusive_filename: *const c_char;
    static perf_usage_string: *const c_char;
    static perf_more_info_string: *const c_char;

    fn cmd_buildid_cache(argc: c_int, argv: *const *const c_char) -> c_int;
    fn cmd_buildid_list(argc: c_int, argv: *const *const c_char) -> c_int;
    fn cmd_check(argc: c_int, argv: *const *const c_char) -> c_int;
    fn cmd_config(argc: c_int, argv: *const *const c_char) -> c_int;
    fn cmd_c2c(argc: c_int, argv: *const *const c_char) -> c_int;
    fn cmd_diff(argc: c_int, argv: *const *const c_char) -> c_int;
    fn cmd_evlist(argc: c_int, argv: *const *const c_char) -> c_int;
    fn cmd_help(argc: c_int, argv: *const *const c_char) -> c_int;
    fn cmd_kallsyms(argc: c_int, argv: *const *const c_char) -> c_int;
    fn cmd_list(argc: c_int, argv: *const *const c_char) -> c_int;
    fn cmd_record(argc: c_int, argv: *const *const c_char) -> c_int;
    fn cmd_report(argc: c_int, argv: *const *const c_char) -> c_int;
    fn cmd_bench(argc: c_int, argv: *const *const c_char) -> c_int;
    fn cmd_stat(argc: c_int, argv: *const *const c_char) -> c_int;
    fn cmd_timechart(argc: c_int, argv: *const *const c_char) -> c_int;
    fn cmd_top(argc: c_int, argv: *const *const c_char) -> c_int;
    fn cmd_annotate(argc: c_int, argv: *const *const c_char) -> c_int;
    fn cmd_version(argc: c_int, argv: *const *const c_char) -> c_int;
    fn cmd_script(argc: c_int, argv: *const *const c_char) -> c_int;
    fn cmd_sched(argc: c_int, argv: *const *const c_char) -> c_int;
    fn cmd_probe(argc: c_int, argv: *const *const c_char) -> c_int;
    fn cmd_kmem(argc: c_int, argv: *const *const c_char) -> c_int;
    fn cmd_lock(argc: c_int, argv: *const *const c_char) -> c_int;
    fn cmd_kvm(argc: c_int, argv: *const *const c_char) -> c_int;
    fn cmd_test(argc: c_int, argv: *const *const c_char) -> c_int;
    fn cmd_trace(argc: c_int, argv: *const *const c_char) -> c_int;
    fn cmd_inject(argc: c_int, argv: *const *const c_char) -> c_int;
    fn cmd_mem(argc: c_int, argv: *const *const c_char) -> c_int;
    fn cmd_data(argc: c_int, argv: *const *const c_char) -> c_int;
    fn cmd_ftrace(argc: c_int, argv: *const *const c_char) -> c_int;
    fn cmd_daemon(argc: c_int, argv: *const *const c_char) -> c_int;
    fn cmd_kwork(argc: c_int, argv: *const *const c_char) -> c_int;

    fn strcmp(s1: *const c_char, s2: *const c_char) -> c_int;
    fn strlen(s: *const c_char) -> size_t;
    fn strstarts(str_: *const c_char, prefix: *const c_char) -> bool;
    fn perf_config_bool(var: *const c_char, value: *const c_char) -> c_int;
    fn perf_config(
        fn_: Option<unsafe extern "C" fn(*const c_char, *const c_char, *mut c_void) -> c_int>,
        data: *mut c_void,
    ) -> c_int;
    fn setenv(name: *const c_char, value: *const c_char, overwrite: c_int) -> c_int;
    fn fopen(path: *const c_char, mode: *const c_char) -> *mut FILE;
    fn fprintf(stream: *mut FILE, fmt: *const c_char, ...) -> c_int;
    fn debug_set_file(fp: *mut FILE);
    fn puts(s: *const c_char) -> c_int;
    fn putchar(c: c_int) -> c_int;
    fn printf(fmt: *const c_char, ...) -> c_int;
    fn exit(status: c_int) -> !;
    fn get_argv_exec_path() -> *const c_char;
    fn set_argv_exec_path(path: *const c_char);
    fn system_path(path: *const c_char) -> *const c_char;
    fn usage(str_: *const c_char) -> !;
    fn tracing_path_set(path: *const c_char);
    fn tracing_path_mount() -> *const c_char;
    fn set_buildid_dir(dir: *const c_char);
    fn perf_debug_option(str_: *const c_char) -> c_int;
    fn fstat(fd: c_int, st: *mut stat) -> c_int;
    fn fileno(stream: *mut FILE) -> c_int;
    fn fflush(stream: *mut FILE) -> c_int;
    fn ferror(stream: *mut FILE) -> c_int;
    fn fclose(stream: *mut FILE) -> c_int;
    fn perf_config__exit();
    fn exit_browser(status: c_int);
    fn asprintf(strp: *mut *mut c_char, fmt: *const c_char, ...) -> c_int;
    fn run_command_v_opt(argv: *const *const c_char, opt: c_int) -> c_int;
    fn IS_RUN_COMMAND_ERR(status: c_int) -> bool;
    fn pr_err(fmt: *const c_char, ...);
    fn zfree(ptr: *mut *mut c_char);
    fn veprintf(level: libperf_print_level, var: c_int, fmt: *const c_char, ap: va_list) -> c_int;
    fn perf_debug_setup();
    fn exec_cmd_init(
        exec_name: *const c_char,
        prefix: *const c_char,
        exec_path: *const c_char,
        exec_path_env: *const c_char,
    );
    fn pager_init(pager_env: *const c_char);
    fn libperf_init(print_fn: Option<unsafe extern "C" fn(libperf_print_level, *const c_char, va_list) -> c_int>);
    fn extract_argv0_path(path: *const c_char) -> *const c_char;
    fn srandom(seed: c_uint);
    fn time(tloc: *mut time_t) -> time_t;
    fn getenv(name: *const c_char) -> *const c_char;
    fn perf_default_config(var: *const c_char, value: *const c_char, data: *mut c_void) -> c_int;
    fn setup_path();
    fn pthread__block_sigwinch();
    fn list_common_cmds_help();
    fn add_cmdname(cmds: *mut cmdnames, name: *const c_char, len: size_t);
    fn help_unknown_cmd(cmd: *const c_char, main_cmds: *mut cmdnames) -> *const c_char;
    fn clean_cmdnames(cmds: *mut cmdnames);
}

static commands: &[cmd_struct] = &[
    cmd_struct { cmd: b"archive\0".as_ptr() as *const c_char, fn_: None, option: 0 },
    cmd_struct { cmd: b"buildid-cache\0".as_ptr() as *const c_char, fn_: Some(cmd_buildid_cache), option: 0 },
    cmd_struct { cmd: b"buildid-list\0".as_ptr() as *const c_char, fn_: Some(cmd_buildid_list), option: 0 },
    cmd_struct { cmd: b"check\0".as_ptr() as *const c_char, fn_: Some(cmd_check), option: 0 },
    cmd_struct { cmd: b"config\0".as_ptr() as *const c_char, fn_: Some(cmd_config), option: 0 },
    cmd_struct { cmd: b"c2c\0".as_ptr() as *const c_char, fn_: Some(cmd_c2c), option: 0 },
    cmd_struct { cmd: b"diff\0".as_ptr() as *const c_char, fn_: Some(cmd_diff), option: 0 },
    cmd_struct { cmd: b"evlist\0".as_ptr() as *const c_char, fn_: Some(cmd_evlist), option: 0 },
    cmd_struct { cmd: b"help\0".as_ptr() as *const c_char, fn_: Some(cmd_help), option: 0 },
    cmd_struct { cmd: b"iostat\0".as_ptr() as *const c_char, fn_: None, option: 0 },
    cmd_struct { cmd: b"kallsyms\0".as_ptr() as *const c_char, fn_: Some(cmd_kallsyms), option: 0 },
    cmd_struct { cmd: b"list\0".as_ptr() as *const c_char, fn_: Some(cmd_list), option: 0 },
    cmd_struct { cmd: b"record\0".as_ptr() as *const c_char, fn_: Some(cmd_record), option: 0 },
    cmd_struct { cmd: b"report\0".as_ptr() as *const c_char, fn_: Some(cmd_report), option: 0 },
    cmd_struct { cmd: b"bench\0".as_ptr() as *const c_char, fn_: Some(cmd_bench), option: 0 },
    cmd_struct { cmd: b"stat\0".as_ptr() as *const c_char, fn_: Some(cmd_stat), option: 0 },
    // Present in C only when HAVE_LIBTRACEEVENT is defined.
    cmd_struct { cmd: b"timechart\0".as_ptr() as *const c_char, fn_: Some(cmd_timechart), option: 0 },
    cmd_struct { cmd: b"top\0".as_ptr() as *const c_char, fn_: Some(cmd_top), option: 0 },
    cmd_struct { cmd: b"annotate\0".as_ptr() as *const c_char, fn_: Some(cmd_annotate), option: 0 },
    cmd_struct { cmd: b"version\0".as_ptr() as *const c_char, fn_: Some(cmd_version), option: 0 },
    cmd_struct { cmd: b"script\0".as_ptr() as *const c_char, fn_: Some(cmd_script), option: 0 },
    // Present in C only when HAVE_LIBTRACEEVENT is defined.
    cmd_struct { cmd: b"sched\0".as_ptr() as *const c_char, fn_: Some(cmd_sched), option: 0 },
    // Present in C only when HAVE_LIBELF_SUPPORT is defined.
    cmd_struct { cmd: b"probe\0".as_ptr() as *const c_char, fn_: Some(cmd_probe), option: 0 },
    // Present in C only when HAVE_LIBTRACEEVENT is defined.
    cmd_struct { cmd: b"kmem\0".as_ptr() as *const c_char, fn_: Some(cmd_kmem), option: 0 },
    cmd_struct { cmd: b"lock\0".as_ptr() as *const c_char, fn_: Some(cmd_lock), option: 0 },
    cmd_struct { cmd: b"kvm\0".as_ptr() as *const c_char, fn_: Some(cmd_kvm), option: 0 },
    cmd_struct { cmd: b"test\0".as_ptr() as *const c_char, fn_: Some(cmd_test), option: 0 },
    // Present in C only when HAVE_LIBTRACEEVENT is defined.
    cmd_struct { cmd: b"trace\0".as_ptr() as *const c_char, fn_: Some(cmd_trace), option: 0 },
    cmd_struct { cmd: b"inject\0".as_ptr() as *const c_char, fn_: Some(cmd_inject), option: 0 },
    cmd_struct { cmd: b"mem\0".as_ptr() as *const c_char, fn_: Some(cmd_mem), option: 0 },
    cmd_struct { cmd: b"data\0".as_ptr() as *const c_char, fn_: Some(cmd_data), option: 0 },
    cmd_struct { cmd: b"ftrace\0".as_ptr() as *const c_char, fn_: Some(cmd_ftrace), option: 0 },
    cmd_struct { cmd: b"daemon\0".as_ptr() as *const c_char, fn_: Some(cmd_daemon), option: 0 },
    // Present in C only when HAVE_LIBTRACEEVENT is defined.
    cmd_struct { cmd: b"kwork\0".as_ptr() as *const c_char, fn_: Some(cmd_kwork), option: 0 },
];

static options: &[option] = &[
    option { long_name: b"help\0".as_ptr() as *const c_char },
    option { long_name: b"version\0".as_ptr() as *const c_char },
    option { long_name: b"exec-path\0".as_ptr() as *const c_char },
    option { long_name: b"html-path\0".as_ptr() as *const c_char },
    option { long_name: b"paginate\0".as_ptr() as *const c_char },
    option { long_name: b"no-pager\0".as_ptr() as *const c_char },
    option { long_name: b"debugfs-dir\0".as_ptr() as *const c_char },
    option { long_name: b"buildid-dir\0".as_ptr() as *const c_char },
    option { long_name: b"list-cmds\0".as_ptr() as *const c_char },
    option { long_name: b"list-opts\0".as_ptr() as *const c_char },
    option { long_name: b"debug\0".as_ptr() as *const c_char },
    option { long_name: b"debug-file\0".as_ptr() as *const c_char },
    option { long_name: core::ptr::null() },
];

unsafe fn same_cmd_with_prefix(var: *const c_char, c: *mut pager_config, header: *const c_char) -> bool {
    strstarts(var, header) && strcmp(var.add(strlen(header)), (*c).cmd) == 0
}

unsafe extern "C" fn pager_command_config(var: *const c_char, value: *const c_char, data: *mut c_void) -> c_int {
    let c = data as *mut pager_config;
    if same_cmd_with_prefix(var, c, b"pager.\0".as_ptr() as *const c_char) {
        (*c).val = perf_config_bool(var, value);
    }
    0
}

/* returns 0 for "no pager", 1 for "use pager", and -1 for "not specified" */
unsafe fn check_pager_config(cmd: *const c_char) -> c_int {
    let mut c = pager_config { cmd, val: -1 };
    let err = perf_config(Some(pager_command_config), &mut c as *mut _ as *mut c_void);
    if err != 0 { err } else { c.val }
}

unsafe extern "C" fn browser_command_config(var: *const c_char, value: *const c_char, data: *mut c_void) -> c_int {
    let c = data as *mut pager_config;
    if same_cmd_with_prefix(var, c, b"tui.\0".as_ptr() as *const c_char) {
        (*c).val = perf_config_bool(var, value);
    }
    if same_cmd_with_prefix(var, c, b"gtk.\0".as_ptr() as *const c_char) {
        (*c).val = if perf_config_bool(var, value) != 0 { 2 } else { 0 };
    }
    0
}

/*
 * returns 0 for "no tui", 1 for "use tui", 2 for "use gtk",
 * and -1 for "not specified"
 */
unsafe fn check_browser_config(cmd: *const c_char) -> c_int {
    let mut c = pager_config { cmd, val: -1 };
    let err = perf_config(Some(browser_command_config), &mut c as *mut _ as *mut c_void);
    if err != 0 { err } else { c.val }
}

unsafe fn commit_pager_choice() {
    match use_pager {
        0 => {
            setenv(PERF_PAGER_ENVIRONMENT, b"cat\0".as_ptr() as *const c_char, 1);
        }
        1 => {
            /* setup_pager(); */
        }
        _ => {}
    }
}

unsafe fn set_debug_file(path: *const c_char) -> c_int {
    debug_fp = fopen(path, b"w\0".as_ptr() as *const c_char);
    if debug_fp.is_null() {
        fprintf(stderr, b"Open debug file '%s' failed: %m\n\0".as_ptr() as *const c_char, path);
        return -1;
    }

    debug_set_file(debug_fp);
    0
}

unsafe fn handle_options(argv: *mut *const *const c_char, argc: *mut c_int, envchanged: *mut c_int) -> c_int {
    let mut handled = 0;

    while *argc > 0 {
        let mut cmd = **argv;
        if *cmd != b'-' as c_char {
            break;
        }

        /*
         * For legacy reasons, the "version" and "help"
         * commands can be written with "--" prepended
         * to make them look like flags.
         */
        if strcmp(cmd, b"--help\0".as_ptr() as *const c_char) == 0
            || strcmp(cmd, b"--version\0".as_ptr() as *const c_char) == 0
        {
            break;
        }

        /*
         * Shortcut for '-h' and '-v' options to invoke help
         * and version command.
         */
        if strcmp(cmd, b"-h\0".as_ptr() as *const c_char) == 0 {
            *(*argv as *mut *const c_char) = b"--help\0".as_ptr() as *const c_char;
            break;
        }

        if strcmp(cmd, b"-v\0".as_ptr() as *const c_char) == 0 {
            *(*argv as *mut *const c_char) = b"--version\0".as_ptr() as *const c_char;
            break;
        }

        if strcmp(cmd, b"-vv\0".as_ptr() as *const c_char) == 0 {
            *(*argv as *mut *const c_char) = b"version\0".as_ptr() as *const c_char;
            verbose = 1;
            break;
        }

        /*
         * Check remaining flags.
         */
        if strstarts(cmd, CMD_EXEC_PATH) {
            cmd = cmd.add(strlen(CMD_EXEC_PATH));
            if *cmd == b'=' as c_char {
                set_argv_exec_path(cmd.add(1));
            } else {
                puts(get_argv_exec_path());
                exit(0);
            }
        } else if strcmp(cmd, b"--html-path\0".as_ptr() as *const c_char) == 0 {
            puts(system_path(PERF_HTML_PATH));
            exit(0);
        } else if strcmp(cmd, b"-p\0".as_ptr() as *const c_char) == 0
            || strcmp(cmd, b"--paginate\0".as_ptr() as *const c_char) == 0
        {
            use_pager = 1;
        } else if strcmp(cmd, b"--no-pager\0".as_ptr() as *const c_char) == 0 {
            use_pager = 0;
            if !envchanged.is_null() {
                *envchanged = 1;
            }
        } else if strcmp(cmd, b"--debugfs-dir\0".as_ptr() as *const c_char) == 0 {
            if *argc < 2 {
                fprintf(stderr, b"No directory given for --debugfs-dir.\n\0".as_ptr() as *const c_char);
                usage(perf_usage_string);
            }
            tracing_path_set(*(*argv).add(1));
            if !envchanged.is_null() {
                *envchanged = 1;
            }
            *argv = (*argv).add(1);
            *argc -= 1;
        } else if strcmp(cmd, b"--buildid-dir\0".as_ptr() as *const c_char) == 0 {
            if *argc < 2 {
                fprintf(stderr, b"No directory given for --buildid-dir.\n\0".as_ptr() as *const c_char);
                usage(perf_usage_string);
            }
            set_buildid_dir(*(*argv).add(1));
            if !envchanged.is_null() {
                *envchanged = 1;
            }
            *argv = (*argv).add(1);
            *argc -= 1;
        } else if strstarts(cmd, CMD_DEBUGFS_DIR) {
            tracing_path_set(cmd.add(strlen(CMD_DEBUGFS_DIR)));
            fprintf(stderr, b"dir: %s\n\0".as_ptr() as *const c_char, tracing_path_mount());
            if !envchanged.is_null() {
                *envchanged = 1;
            }
        } else if strcmp(cmd, b"--list-cmds\0".as_ptr() as *const c_char) == 0 {
            let mut i: c_uint = 0;
            while (i as usize) < commands.len() {
                let p = &commands[i as usize] as *const cmd_struct;
                printf(b"%s \0".as_ptr() as *const c_char, (*p).cmd);
                i += 1;
            }
            putchar(b'\n' as c_int);
            exit(0);
        } else if strcmp(cmd, b"--list-opts\0".as_ptr() as *const c_char) == 0 {
            let mut i: c_uint = 0;
            while (i as usize) < options.len() - 1 {
                let p = &options[i as usize] as *const option;
                printf(b"--%s \0".as_ptr() as *const c_char, (*p).long_name);
                i += 1;
            }
            putchar(b'\n' as c_int);
            exit(0);
        } else if strcmp(cmd, b"--debug\0".as_ptr() as *const c_char) == 0 {
            if *argc < 2 {
                fprintf(stderr, b"No variable specified for --debug.\n\0".as_ptr() as *const c_char);
                usage(perf_usage_string);
            }
            if perf_debug_option(*(*argv).add(1)) != 0 {
                usage(perf_usage_string);
            }

            *argv = (*argv).add(1);
            *argc -= 1;
        } else if strcmp(cmd, b"--debug-file\0".as_ptr() as *const c_char) == 0 {
            if *argc < 2 {
                fprintf(stderr, b"No path given for --debug-file.\n\0".as_ptr() as *const c_char);
                usage(perf_usage_string);
            }

            if set_debug_file(*(*argv).add(1)) != 0 {
                usage(perf_usage_string);
            }

            *argv = (*argv).add(1);
            *argc -= 1;
        } else {
            fprintf(stderr, b"Unknown option: %s\n\0".as_ptr() as *const c_char, cmd);
            usage(perf_usage_string);
        }

        *argv = (*argv).add(1);
        *argc -= 1;
        handled += 1;
    }
    handled
}

fn S_ISFIFO(mode: mode_t) -> bool {
    (mode & 0o170000) == 0o010000
}

fn S_ISSOCK(mode: mode_t) -> bool {
    (mode & 0o170000) == 0o140000
}

unsafe fn run_builtin(p: *const cmd_struct, argc: c_int, argv: *const *const c_char) -> c_int {
    let mut status: c_int;
    let mut st: stat = core::mem::zeroed();

    if use_browser == -1 {
        use_browser = check_browser_config((*p).cmd);
    }

    if use_pager == -1 && ((*p).option & RUN_SETUP) != 0 {
        use_pager = check_pager_config((*p).cmd);
    }
    if use_pager == -1 && ((*p).option & USE_PAGER) != 0 {
        use_pager = 1;
    }
    commit_pager_choice();

    status = ((*p).fn_.unwrap())(argc, argv);
    perf_config__exit();
    exit_browser(status);

    if status != 0 {
        return status & 0xff;
    }

    /* Somebody closed stdout? */
    if fstat(fileno(stdout), &mut st) != 0 {
        return 0;
    }
    /* Ignore write errors for pipes and sockets.. */
    if S_ISFIFO(st.st_mode) || S_ISSOCK(st.st_mode) {
        return 0;
    }

    status = 1;
    /* Check for ENOSPC and EIO errors.. */
    if fflush(stdout) != 0 {
        fprintf(stderr, b"write failure on standard output: %m\n\0".as_ptr() as *const c_char);
        return status;
    }
    if ferror(stdout) != 0 {
        fprintf(stderr, b"unknown write failure on standard output\n\0".as_ptr() as *const c_char);
        return status;
    }
    if fclose(stdout) != 0 {
        fprintf(stderr, b"close failed on standard output: %m\n\0".as_ptr() as *const c_char);
        return status;
    }
    status = 0;
    status
}

unsafe fn handle_internal_command(argc: c_int, argv: *mut *const c_char) {
    let mut cmd = *argv;
    let mut i: c_uint = 0;

    /* Turn "perf cmd --help" into "perf help cmd" */
    if argc > 1 && strcmp(*argv.add(1), b"--help\0".as_ptr() as *const c_char) == 0 {
        *argv.add(1) = *argv;
        *argv = b"help\0".as_ptr() as *const c_char;
        cmd = *argv;
    }

    while (i as usize) < commands.len() {
        let p = &commands[i as usize] as *const cmd_struct;
        if (*p).fn_.is_none() {
            i += 1;
            continue;
        }
        if strcmp((*p).cmd, cmd) != 0 {
            i += 1;
            continue;
        }
        exit(run_builtin(p, argc, argv as *const *const c_char));
    }
}

unsafe fn execv_dashed_external(argv: *mut *const c_char) {
    let mut cmd: *mut c_char = core::ptr::null_mut();
    let tmp: *const c_char;
    let mut status: c_int;

    if asprintf(&mut cmd, b"perf-%s\0".as_ptr() as *const c_char, *argv) < 0 {
        pr_err(b"FATAL: unable to run '%s'\0".as_ptr() as *const c_char, *argv);
        status = -128;
        exit(-status);
    }

    /*
     * argv[0] must be the perf command, but the argv array
     * belongs to the caller, and may be reused in
     * subsequent loop iterations. Save argv[0] and
     * restore it on error.
     */
    tmp = *argv;
    *argv = cmd;

    /*
     * if we fail because the command is not found, it is
     * OK to return. Otherwise, we just pass along the status code.
     */
    status = run_command_v_opt(argv as *const *const c_char, 0);
    if status != -ERR_RUN_COMMAND_EXEC {
        if IS_RUN_COMMAND_ERR(status) {
            pr_err(b"FATAL: unable to run '%s'\0".as_ptr() as *const c_char, *argv);
            status = -128;
        }
        exit(-status);
    }
    errno = ENOENT; /* as if we called execvp */

    *argv = tmp;
    zfree(&mut cmd);
}

unsafe fn run_argv(argcp: *mut c_int, argv: *mut *mut *const c_char) -> c_int {
    /* See if it's an internal command */
    handle_internal_command(*argcp, *argv);

    /* .. then try the external ones */
    execv_dashed_external(*argv);
    0
}

unsafe extern "C" fn libperf_print(level: libperf_print_level, fmt: *const c_char, ap: va_list) -> c_int {
    veprintf(level, verbose, fmt, ap)
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn main(mut argc: c_int, mut argv: *mut *const c_char) -> c_int {
    let mut err: c_int;
    let mut done_help: c_int = 0;
    let mut cmd: *const c_char;

    perf_debug_setup();

    /* libsubcmd init */
    exec_cmd_init(b"perf\0".as_ptr() as *const c_char, PREFIX, PERF_EXEC_PATH, EXEC_PATH_ENVIRONMENT);
    pager_init(PERF_PAGER_ENVIRONMENT);

    libperf_init(Some(libperf_print));

    cmd = extract_argv0_path(*argv);
    if cmd.is_null() {
        cmd = b"perf-help\0".as_ptr() as *const c_char;
    }

    srandom(time(core::ptr::null_mut()) as c_uint);

    /* Setting $PERF_CONFIG makes perf read _only_ the given config file. */
    config_exclusive_filename = getenv(b"PERF_CONFIG\0".as_ptr() as *const c_char);

    err = perf_config(Some(perf_default_config), core::ptr::null_mut());
    if err != 0 {
        return err;
    }
    set_buildid_dir(core::ptr::null());

    /*
     * "perf-xxxx" is the same as "perf xxxx", but we obviously:
     *
     *  - cannot take flags in between the "perf" and the "xxxx".
     *  - cannot execute it externally (since it would just do
     *    the same thing over again)
     *
     * So we just directly call the internal command handler. If that one
     * fails to handle this, then maybe we just run a renamed perf binary
     * that contains a dash in its name. To handle this scenario, we just
     * fall through and ignore the "xxxx" part of the command string.
     */
    if strstarts(cmd, b"perf-\0".as_ptr() as *const c_char) {
        cmd = cmd.add(5);
        *argv = cmd;
        handle_internal_command(argc, argv);
        /*
         * If the command is handled, the above function does not
         * return undo changes and fall through in such a case.
         */
        cmd = cmd.sub(5);
        *argv = cmd;
    }
    if strstarts(cmd, b"trace\0".as_ptr() as *const c_char) {
        // C condition: when HAVE_LIBTRACEEVENT is not defined, print this error and go to out.
        // When HAVE_LIBTRACEEVENT is defined: setup_path(); argv[0] = "trace"; return cmd_trace(argc, argv);
        fprintf(
            stderr,
            b"trace command not available: missing libtraceevent devel package at build time.\n\0".as_ptr()
                as *const c_char,
        );
        if !debug_fp.is_null() {
            fclose(debug_fp);
        }
        return 1;
    }
    /* Look for flags.. */
    argv = argv.add(1);
    argc -= 1;
    let mut argv_for_options = argv as *const *const c_char;
    handle_options(&mut argv_for_options, &mut argc, core::ptr::null_mut());
    argv = argv_for_options as *mut *const c_char;
    commit_pager_choice();

    if argc > 0 {
        if strstarts(*argv, b"--\0".as_ptr() as *const c_char) {
            *argv = (*argv).add(2);
        }
    } else {
        /* The user didn't specify a command; give them help */
        printf(b"\n usage: %s\n\n\0".as_ptr() as *const c_char, perf_usage_string);
        list_common_cmds_help();
        printf(b"\n %s\n\n\0".as_ptr() as *const c_char, perf_more_info_string);
        if !debug_fp.is_null() {
            fclose(debug_fp);
        }
        return 1;
    }
    cmd = *argv;

    /*
     * We use PATH to find perf commands, but we prepend some higher
     * precedence paths: the "--exec-path" option, the PERF_EXEC_PATH
     * environment, and the $(perfexecdir) from the Makefile at build
     * time.
     */
    setup_path();
    /*
     * Block SIGWINCH notifications so that the thread that wants it can
     * unblock and get syscalls like select interrupted instead of waiting
     * forever while the signal goes to some other non interested thread.
     */
    pthread__block_sigwinch();

    loop {
        run_argv(&mut argc, &mut argv);

        if errno != ENOENT {
            break;
        }

        if done_help == 0 {
            let mut main_cmds: cmdnames = core::mem::zeroed();

            let mut i: c_uint = 0;
            while (i as usize) < commands.len() {
                add_cmdname(&mut main_cmds, commands[i as usize].cmd, strlen(commands[i as usize].cmd));
                i += 1;
            }
            cmd = help_unknown_cmd(cmd, &mut main_cmds);
            *argv = cmd;
            clean_cmdnames(&mut main_cmds);
            done_help = 1;
            if cmd.is_null() {
                break;
            }
        } else {
            break;
        }
    }

    if !cmd.is_null() {
        fprintf(stderr, b"Failed to run command '%s': %m\n\0".as_ptr() as *const c_char, cmd);
    }

    if !debug_fp.is_null() {
        fclose(debug_fp);
    }

    1
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
