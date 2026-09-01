// SPDX-License-Identifier: GPL-2.0
/*
 * builtin-help.c
 *
 * Builtin help command
 */

use core::ffi::{c_char, c_int, c_uint, c_void};
use core::mem;
use core::ptr;

type size_t = usize;
type ssize_t = isize;
type mode_t = c_uint;

const HELP_FORMAT_NONE: help_format = 0;
const HELP_FORMAT_MAN: help_format = 1;
const HELP_FORMAT_INFO: help_format = 2;
const HELP_FORMAT_WEB: help_format = 3;

type help_format = c_uint;

const PATH_MAX: usize = 4096;
const STRERR_BUFSIZE: usize = 1024;
const PERF_MAN_PATH: *const c_char = b"man\0".as_ptr() as *const c_char;
const PERF_INFO_PATH: *const c_char = b"info\0".as_ptr() as *const c_char;
const PERF_HTML_PATH: *const c_char = b"html\0".as_ptr() as *const c_char;

#[repr(C)]
struct man_viewer_list {
    next: *mut man_viewer_list,
    name: [c_char; 0],
}

#[repr(C)]
struct man_viewer_info_list {
    next: *mut man_viewer_info_list,
    info: *const c_char,
    name: [c_char; 0],
}

#[repr(C)]
struct strbuf {
    alloc: size_t,
    len: size_t,
    buf: *mut c_char,
}

#[repr(C)]
struct child_process {
    argv: *const *const c_char,
    err: c_int,
    stdout_to_stderr: c_uint,
}

#[repr(C)]
struct cmdnames {
    alloc: c_int,
    cnt: c_int,
    names: *mut *mut c_void,
}

#[repr(C)]
struct option {
    type_: c_int,
    short_name: c_int,
    long_name: *const c_char,
    value: *mut c_void,
    precision: c_int,
    argh: *const c_char,
    help: *const c_char,
    flags: c_int,
    callback: Option<unsafe extern "C" fn(*const option, *const c_char, c_int) -> c_int>,
    defval: isize,
    ll_callback: Option<unsafe extern "C" fn(*const option, *const c_char, c_int) -> c_int>,
    extra: *const c_void,
    subcommand_fn: *const c_void,
}

#[repr(C)]
struct stat {
    st_dev: u64,
    st_ino: u64,
    st_nlink: u64,
    st_mode: mode_t,
}

#[repr(C)]
struct cmdname_help {
    name: *const c_char,
    help: *const c_char,
}

static mut man_viewer_list: *mut man_viewer_list = ptr::null_mut();
static mut man_viewer_info_list: *mut man_viewer_info_list = ptr::null_mut();
static mut main_cmds: cmdnames = cmdnames {
    alloc: 0,
    cnt: 0,
    names: ptr::null_mut(),
};
static mut other_cmds: cmdnames = cmdnames {
    alloc: 0,
    cnt: 0,
    names: ptr::null_mut(),
};

unsafe extern "C" {
    static mut errno: c_int;
    static perf_usage_string: *const c_char;
    static perf_more_info_string: *const c_char;

    fn strcmp(s1: *const c_char, s2: *const c_char) -> c_int;
    fn strncmp(s1: *const c_char, s2: *const c_char, n: size_t) -> c_int;
    fn strcasecmp(s1: *const c_char, s2: *const c_char) -> c_int;
    fn strncasecmp(s1: *const c_char, s2: *const c_char, n: size_t) -> c_int;
    fn strlen(s: *const c_char) -> size_t;
    fn strcpy(dest: *mut c_char, src: *const c_char) -> *mut c_char;
    fn strncpy(dest: *mut c_char, src: *const c_char, n: size_t) -> *mut c_char;
    fn strdup(s: *const c_char) -> *mut c_char;
    fn strrchr(s: *const c_char, c: c_int) -> *mut c_char;
    fn memset(s: *mut c_void, c: c_int, n: size_t) -> *mut c_void;
    fn atoi(nptr: *const c_char) -> c_int;
    fn getenv(name: *const c_char) -> *mut c_char;
    fn setenv(name: *const c_char, value: *const c_char, overwrite: c_int) -> c_int;
    fn asprintf(strp: *mut *mut c_char, fmt: *const c_char, ...) -> c_int;
    fn fprintf(stream: *mut c_void, fmt: *const c_char, ...) -> c_int;
    fn printf(fmt: *const c_char, ...) -> c_int;
    fn puts(s: *const c_char) -> c_int;
    fn free(ptr: *mut c_void);
    fn close(fd: c_int) -> c_int;
    fn execlp(file: *const c_char, arg: *const c_char, ...) -> c_int;
    fn execl(path: *const c_char, arg: *const c_char, ...) -> c_int;
    fn stat(path: *const c_char, buf: *mut stat) -> c_int;

    fn pr_err(fmt: *const c_char, ...);
    fn pr_warning(fmt: *const c_char, ...);
    fn str_error_r(errnum: c_int, buf: *mut c_char, buflen: size_t) -> *const c_char;
    fn zalloc(size: size_t) -> *mut c_void;
    fn strbuf_read(sb: *mut strbuf, fd: c_int, hint: size_t) -> ssize_t;
    fn strbuf_release(sb: *mut strbuf);
    fn start_command(cmd: *mut child_process) -> c_int;
    fn finish_command(cmd: *mut child_process) -> c_int;
    fn config_error_nonbool(var: *const c_char) -> c_int;
    fn perf_config(
        fn_: Option<unsafe extern "C" fn(*const c_char, *const c_char, *mut c_void) -> c_int>,
        data: *mut c_void,
    ) -> c_int;
    fn load_command_list(prefix: *const c_char, main: *mut cmdnames, other: *mut cmdnames);
    fn list_commands(title: *const c_char, main: *mut cmdnames, other: *mut cmdnames);
    fn parse_options_subcommand(
        argc: c_int,
        argv: *const *const c_char,
        options: *const option,
        subcommands: *const *const c_char,
        usagestr: *const *const c_char,
        flags: c_int,
    ) -> c_int;
    fn system_path(path: *const c_char) -> *const c_char;
    fn mkpath(buf: *mut c_char, size: size_t, fmt: *const c_char, ...) -> *const c_char;
    fn execl_cmd(cmd: *const c_char, ...) -> c_int;
}

unsafe fn strbuf_init() -> strbuf {
    strbuf {
        alloc: 0,
        len: 0,
        buf: b"\0".as_ptr() as *mut c_char,
    }
}

unsafe fn strstarts(str_: *const c_char, prefix: *const c_char) -> bool {
    strncmp(str_, prefix, strlen(prefix)) == 0
}

unsafe fn s_isreg(mode: mode_t) -> bool {
    (mode & 0o170000) == 0o100000
}

unsafe fn opt_boolean(
    short_name: c_int,
    long_name: *const c_char,
    value: *mut bool,
    help: *const c_char,
) -> option {
    option {
        type_: 1,
        short_name,
        long_name,
        value: value as *mut c_void,
        precision: 0,
        argh: ptr::null(),
        help,
        flags: 0,
        callback: None,
        defval: 0,
        ll_callback: None,
        extra: ptr::null(),
        subcommand_fn: ptr::null(),
    }
}

unsafe fn opt_set_uint(
    short_name: c_int,
    long_name: *const c_char,
    value: *mut help_format,
    help: *const c_char,
    defval: help_format,
) -> option {
    option {
        type_: 2,
        short_name,
        long_name,
        value: value as *mut c_void,
        precision: 0,
        argh: ptr::null(),
        help,
        flags: 0,
        callback: None,
        defval: defval as isize,
        ll_callback: None,
        extra: ptr::null(),
        subcommand_fn: ptr::null(),
    }
}

unsafe fn opt_end() -> option {
    option {
        type_: 0,
        short_name: 0,
        long_name: ptr::null(),
        value: ptr::null_mut(),
        precision: 0,
        argh: ptr::null(),
        help: ptr::null(),
        flags: 0,
        callback: None,
        defval: 0,
        ll_callback: None,
        extra: ptr::null(),
        subcommand_fn: ptr::null(),
    }
}

unsafe fn parse_help_format(format: *const c_char) -> help_format {
    if strcmp(format, b"man\0".as_ptr() as *const c_char) == 0 {
        return HELP_FORMAT_MAN;
    }
    if strcmp(format, b"info\0".as_ptr() as *const c_char) == 0 {
        return HELP_FORMAT_INFO;
    }
    if strcmp(format, b"web\0".as_ptr() as *const c_char) == 0
        || strcmp(format, b"html\0".as_ptr() as *const c_char) == 0
    {
        return HELP_FORMAT_WEB;
    }

    pr_err(
        b"unrecognized help format '%s'\0".as_ptr() as *const c_char,
        format,
    );
    HELP_FORMAT_NONE
}

unsafe fn get_man_viewer_info(name: *const c_char) -> *const c_char {
    let mut viewer = man_viewer_info_list;

    while !viewer.is_null() {
        if strcasecmp(name, (*viewer).name.as_ptr()) == 0 {
            return (*viewer).info;
        }
        viewer = (*viewer).next;
    }
    ptr::null()
}

unsafe fn check_emacsclient_version() -> c_int {
    let mut buffer = strbuf_init();
    let mut ec_process: child_process = mem::zeroed();
    let argv_ec = [
        b"emacsclient\0".as_ptr() as *const c_char,
        b"--version\0".as_ptr() as *const c_char,
        ptr::null(),
    ];
    let mut version: c_int;
    let mut ret: c_int = -1;

    /* emacsclient prints its version number on stderr */
    memset(
        &mut ec_process as *mut child_process as *mut c_void,
        0,
        mem::size_of::<child_process>(),
    );
    ec_process.argv = argv_ec.as_ptr();
    ec_process.err = -1;
    ec_process.stdout_to_stderr = 1;
    if start_command(&mut ec_process) != 0 {
        fprintf(
            stderr(),
            b"Failed to start emacsclient.\n\0".as_ptr() as *const c_char,
        );
        return -1;
    }
    if strbuf_read(&mut buffer, ec_process.err, 20) < 0 {
        fprintf(
            stderr(),
            b"Failed to read emacsclient version\n\0".as_ptr() as *const c_char,
        );
        strbuf_release(&mut buffer);
        return ret;
    }
    close(ec_process.err);

    /*
     * Don't bother checking return value, because "emacsclient --version"
     * seems to always exits with code 1.
     */
    finish_command(&mut ec_process);

    if !strstarts(buffer.buf, b"emacsclient\0".as_ptr() as *const c_char) {
        fprintf(
            stderr(),
            b"Failed to parse emacsclient version.\n\0".as_ptr() as *const c_char,
        );
        strbuf_release(&mut buffer);
        return ret;
    }

    version = atoi(buffer.buf.add(strlen(b"emacsclient\0".as_ptr() as *const c_char)));

    if version < 22 {
        fprintf(
            stderr(),
            b"emacsclient version '%d' too old (< 22).\n\0".as_ptr() as *const c_char,
            version,
        );
    } else {
        ret = 0;
    }
    strbuf_release(&mut buffer);
    ret
}

unsafe fn stderr() -> *mut c_void {
    unsafe extern "C" {
        static mut stderr: *mut c_void;
    }
    stderr
}

unsafe fn exec_failed(cmd: *const c_char) {
    let mut sbuf = [0 as c_char; STRERR_BUFSIZE];
    pr_warning(
        b"failed to exec '%s': %s\0".as_ptr() as *const c_char,
        cmd,
        str_error_r(errno, sbuf.as_mut_ptr(), sbuf.len()),
    );
}

unsafe fn exec_woman_emacs(mut path: *const c_char, page: *const c_char) {
    if check_emacsclient_version() == 0 {
        /* This works only with emacsclient version >= 22. */
        let mut man_page: *mut c_char = ptr::null_mut();

        if path.is_null() {
            path = b"emacsclient\0".as_ptr() as *const c_char;
        }
        if asprintf(
            &mut man_page,
            b"(woman \"%s\")\0".as_ptr() as *const c_char,
            page,
        ) > 0
        {
            execlp(
                path,
                b"emacsclient\0".as_ptr() as *const c_char,
                b"-e\0".as_ptr() as *const c_char,
                man_page,
                ptr::null::<c_char>(),
            );
            free(man_page as *mut c_void);
        }
        exec_failed(path);
    }
}

unsafe fn exec_man_konqueror(mut path: *const c_char, page: *const c_char) {
    let display = getenv(b"DISPLAY\0".as_ptr() as *const c_char);

    if !display.is_null() && *display != 0 {
        let mut man_page: *mut c_char = ptr::null_mut();
        let mut filename = b"kfmclient\0".as_ptr() as *const c_char;

        /* It's simpler to launch konqueror using kfmclient. */
        if !path.is_null() {
            let file = strrchr(path, '/' as c_int);
            if !file.is_null()
                && strcmp(file.add(1), b"konqueror\0".as_ptr() as *const c_char) == 0
            {
                let new = strdup(path);
                let dest = strrchr(new, '/' as c_int);

                /* strlen("konqueror") == strlen("kfmclient") */
                strcpy(dest.add(1), b"kfmclient\0".as_ptr() as *const c_char);
                path = new;
            }
            if !file.is_null() {
                filename = file;
            }
        } else {
            path = b"kfmclient\0".as_ptr() as *const c_char;
        }
        if asprintf(
            &mut man_page,
            b"man:%s(1)\0".as_ptr() as *const c_char,
            page,
        ) > 0
        {
            execlp(
                path,
                filename,
                b"newTab\0".as_ptr() as *const c_char,
                man_page,
                ptr::null::<c_char>(),
            );
            free(man_page as *mut c_void);
        }
        exec_failed(path);
    }
}

unsafe fn exec_man_man(mut path: *const c_char, page: *const c_char) {
    if path.is_null() {
        path = b"man\0".as_ptr() as *const c_char;
    }
    execlp(
        path,
        b"man\0".as_ptr() as *const c_char,
        page,
        ptr::null::<c_char>(),
    );
    exec_failed(path);
}

unsafe fn exec_man_cmd(cmd: *const c_char, page: *const c_char) {
    let mut shell_cmd: *mut c_char = ptr::null_mut();

    if asprintf(
        &mut shell_cmd,
        b"%s %s\0".as_ptr() as *const c_char,
        cmd,
        page,
    ) > 0
    {
        execl(
            b"/bin/sh\0".as_ptr() as *const c_char,
            b"sh\0".as_ptr() as *const c_char,
            b"-c\0".as_ptr() as *const c_char,
            shell_cmd,
            ptr::null::<c_char>(),
        );
        free(shell_cmd as *mut c_void);
    }
    exec_failed(cmd);
}

unsafe fn add_man_viewer(name: *const c_char) {
    let mut p: *mut *mut man_viewer_list = &mut man_viewer_list;
    let len = strlen(name);

    while !(*p).is_null() {
        p = &mut (**p).next;
    }
    *p = zalloc(mem::size_of::<man_viewer_list>() + len + 1) as *mut man_viewer_list;
    strcpy((**p).name.as_mut_ptr(), name);
}

unsafe fn supported_man_viewer(name: *const c_char, len: size_t) -> c_int {
    (strncasecmp(b"man\0".as_ptr() as *const c_char, name, len) == 0
        || strncasecmp(b"woman\0".as_ptr() as *const c_char, name, len) == 0
        || strncasecmp(b"konqueror\0".as_ptr() as *const c_char, name, len) == 0)
        as c_int
}

unsafe fn do_add_man_viewer_info(name: *const c_char, len: size_t, value: *const c_char) {
    let new = zalloc(mem::size_of::<man_viewer_info_list>() + len + 1) as *mut man_viewer_info_list;

    strncpy((*new).name.as_mut_ptr(), name, len);
    (*new).info = strdup(value);
    (*new).next = man_viewer_info_list;
    man_viewer_info_list = new;
}

unsafe fn unsupported_man_viewer(name: *const c_char, var: *const c_char) {
    pr_warning(
        b"'%s': path for unsupported man viewer.\nPlease consider using 'man.<tool>.%s' instead.\0"
            .as_ptr() as *const c_char,
        name,
        var,
    );
}

unsafe fn add_man_viewer_path(name: *const c_char, len: size_t, value: *const c_char) -> c_int {
    if supported_man_viewer(name, len) != 0 {
        do_add_man_viewer_info(name, len, value);
    } else {
        unsupported_man_viewer(name, b"cmd\0".as_ptr() as *const c_char);
    }

    0
}

unsafe fn add_man_viewer_cmd(name: *const c_char, len: size_t, value: *const c_char) -> c_int {
    if supported_man_viewer(name, len) != 0 {
        unsupported_man_viewer(name, b"path\0".as_ptr() as *const c_char);
    } else {
        do_add_man_viewer_info(name, len, value);
    }

    0
}

unsafe fn add_man_viewer_info(var: *const c_char, value: *const c_char) -> c_int {
    let name = var.add(4);
    let subkey = strrchr(name, '.' as c_int);

    if subkey.is_null() {
        pr_err(
            b"Config with no key for man viewer: %s\0".as_ptr() as *const c_char,
            name,
        );
        return -1;
    }

    if strcmp(subkey, b".path\0".as_ptr() as *const c_char) == 0 {
        if value.is_null() {
            return config_error_nonbool(var);
        }
        return add_man_viewer_path(name, subkey.offset_from(name) as size_t, value);
    }
    if strcmp(subkey, b".cmd\0".as_ptr() as *const c_char) == 0 {
        if value.is_null() {
            return config_error_nonbool(var);
        }
        return add_man_viewer_cmd(name, subkey.offset_from(name) as size_t, value);
    }

    pr_warning(
        b"'%s': unsupported man viewer sub key.\0".as_ptr() as *const c_char,
        subkey,
    );
    0
}

unsafe extern "C" fn perf_help_config(
    var: *const c_char,
    value: *const c_char,
    cb: *mut c_void,
) -> c_int {
    let help_formatp = cb as *mut help_format;

    if strcmp(var, b"help.format\0".as_ptr() as *const c_char) == 0 {
        if value.is_null() {
            return config_error_nonbool(var);
        }
        *help_formatp = parse_help_format(value);
        if *help_formatp == HELP_FORMAT_NONE {
            return -1;
        }
        return 0;
    }
    if strcmp(var, b"man.viewer\0".as_ptr() as *const c_char) == 0 {
        if value.is_null() {
            return config_error_nonbool(var);
        }
        add_man_viewer(value);
        return 0;
    }
    if strstarts(var, b"man.\0".as_ptr() as *const c_char) {
        return add_man_viewer_info(var, value);
    }

    0
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn list_common_cmds_help() {
    let common_cmds = [
        cmdname_help { name: b"annotate\0".as_ptr() as *const c_char, help: b"Read perf.data (created by perf record) and display annotated code\0".as_ptr() as *const c_char },
        cmdname_help { name: b"archive\0".as_ptr() as *const c_char, help: b"Create archive with object files with build-ids found in perf.data file\0".as_ptr() as *const c_char },
        cmdname_help { name: b"bench\0".as_ptr() as *const c_char, help: b"General framework for benchmark suites\0".as_ptr() as *const c_char },
        cmdname_help { name: b"buildid-cache\0".as_ptr() as *const c_char, help: b"Manage build-id cache.\0".as_ptr() as *const c_char },
        cmdname_help { name: b"buildid-list\0".as_ptr() as *const c_char, help: b"List the buildids in a perf.data file\0".as_ptr() as *const c_char },
        cmdname_help { name: b"c2c\0".as_ptr() as *const c_char, help: b"Shared Data C2C/HITM Analyzer.\0".as_ptr() as *const c_char },
        cmdname_help { name: b"config\0".as_ptr() as *const c_char, help: b"Get and set variables in a configuration file.\0".as_ptr() as *const c_char },
        cmdname_help { name: b"daemon\0".as_ptr() as *const c_char, help: b"Run record sessions on background\0".as_ptr() as *const c_char },
        cmdname_help { name: b"data\0".as_ptr() as *const c_char, help: b"Data file related processing\0".as_ptr() as *const c_char },
        cmdname_help { name: b"diff\0".as_ptr() as *const c_char, help: b"Read perf.data files and display the differential profile\0".as_ptr() as *const c_char },
        cmdname_help { name: b"evlist\0".as_ptr() as *const c_char, help: b"List the event names in a perf.data file\0".as_ptr() as *const c_char },
        cmdname_help { name: b"ftrace\0".as_ptr() as *const c_char, help: b"simple wrapper for kernel's ftrace functionality\0".as_ptr() as *const c_char },
        cmdname_help { name: b"inject\0".as_ptr() as *const c_char, help: b"Filter to augment the events stream with additional information\0".as_ptr() as *const c_char },
        cmdname_help { name: b"iostat\0".as_ptr() as *const c_char, help: b"Show I/O performance metrics\0".as_ptr() as *const c_char },
        cmdname_help { name: b"kallsyms\0".as_ptr() as *const c_char, help: b"Searches running kernel for symbols\0".as_ptr() as *const c_char },
        cmdname_help { name: b"kvm\0".as_ptr() as *const c_char, help: b"Tool to trace/measure kvm guest os\0".as_ptr() as *const c_char },
        cmdname_help { name: b"list\0".as_ptr() as *const c_char, help: b"List all symbolic event types\0".as_ptr() as *const c_char },
        cmdname_help { name: b"mem\0".as_ptr() as *const c_char, help: b"Profile memory accesses\0".as_ptr() as *const c_char },
        cmdname_help { name: b"record\0".as_ptr() as *const c_char, help: b"Run a command and record its profile into perf.data\0".as_ptr() as *const c_char },
        cmdname_help { name: b"report\0".as_ptr() as *const c_char, help: b"Read perf.data (created by perf record) and display the profile\0".as_ptr() as *const c_char },
        cmdname_help { name: b"script\0".as_ptr() as *const c_char, help: b"Read perf.data (created by perf record) and display trace output\0".as_ptr() as *const c_char },
        cmdname_help { name: b"stat\0".as_ptr() as *const c_char, help: b"Run a command and gather performance counter statistics\0".as_ptr() as *const c_char },
        cmdname_help { name: b"test\0".as_ptr() as *const c_char, help: b"Runs sanity tests.\0".as_ptr() as *const c_char },
        cmdname_help { name: b"top\0".as_ptr() as *const c_char, help: b"System profiling tool.\0".as_ptr() as *const c_char },
        cmdname_help { name: b"version\0".as_ptr() as *const c_char, help: b"display the version of perf binary\0".as_ptr() as *const c_char },
        /* HAVE_LIBELF_SUPPORT: {"probe", "Define new dynamic tracepoints"} */
        /* HAVE_LIBTRACEEVENT: trace, kmem, kwork, lock, sched, timechart entries */
    ];
    let mut longest: size_t = 0;

    for i in 0..common_cmds.len() {
        if longest < strlen(common_cmds[i].name) {
            longest = strlen(common_cmds[i].name);
        }
    }

    puts(b" The most commonly used perf commands are:\0".as_ptr() as *const c_char);
    for i in 0..common_cmds.len() {
        printf(
            b"   %-*s   \0".as_ptr() as *const c_char,
            longest as c_int,
            common_cmds[i].name,
        );
        puts(common_cmds[i].help);
    }
}

unsafe fn cmd_to_page(perf_cmd: *const c_char) -> *const c_char {
    let mut s: *mut c_char = ptr::null_mut();

    if perf_cmd.is_null() {
        return b"perf\0".as_ptr() as *const c_char;
    } else if strstarts(perf_cmd, b"perf\0".as_ptr() as *const c_char) {
        return perf_cmd;
    }

    if asprintf(
        &mut s,
        b"perf-%s\0".as_ptr() as *const c_char,
        perf_cmd,
    ) < 0
    {
        ptr::null()
    } else {
        s
    }
}

unsafe fn setup_man_path() {
    let mut new_path: *mut c_char = ptr::null_mut();
    let old_path = getenv(b"MANPATH\0".as_ptr() as *const c_char);
    let old_or_empty = if old_path.is_null() {
        b"\0".as_ptr() as *const c_char
    } else {
        old_path
    };

    /* We should always put ':' after our path. If there is no
     * old_path, the ':' at the end will let 'man' to try
     * system-wide paths after ours to find the manual page. If
     * there is old_path, we need ':' as delimiter. */
    if asprintf(
        &mut new_path,
        b"%s:%s\0".as_ptr() as *const c_char,
        system_path(PERF_MAN_PATH),
        old_or_empty,
    ) > 0
    {
        setenv(b"MANPATH\0".as_ptr() as *const c_char, new_path, 1);
        free(new_path as *mut c_void);
    } else {
        pr_err(b"Unable to setup man path\0".as_ptr() as *const c_char);
    }
}

unsafe fn exec_viewer(name: *const c_char, page: *const c_char) {
    let info = get_man_viewer_info(name);

    if strcasecmp(name, b"man\0".as_ptr() as *const c_char) == 0 {
        exec_man_man(info, page);
    } else if strcasecmp(name, b"woman\0".as_ptr() as *const c_char) == 0 {
        exec_woman_emacs(info, page);
    } else if strcasecmp(name, b"konqueror\0".as_ptr() as *const c_char) == 0 {
        exec_man_konqueror(info, page);
    } else if !info.is_null() {
        exec_man_cmd(info, page);
    } else {
        pr_warning(
            b"'%s': unknown man viewer.\0".as_ptr() as *const c_char,
            name,
        );
    }
}

unsafe fn show_man_page(perf_cmd: *const c_char) -> c_int {
    let mut viewer: *mut man_viewer_list;
    let page = cmd_to_page(perf_cmd);
    let fallback = getenv(b"PERF_MAN_VIEWER\0".as_ptr() as *const c_char);

    setup_man_path();
    viewer = man_viewer_list;
    while !viewer.is_null() {
        exec_viewer((*viewer).name.as_ptr(), page); /* will return when unable */
        viewer = (*viewer).next;
    }

    if !fallback.is_null() {
        exec_viewer(fallback, page);
    }
    exec_viewer(b"man\0".as_ptr() as *const c_char, page);

    pr_err(b"no man viewer handled the request\0".as_ptr() as *const c_char);
    -1
}

unsafe fn show_info_page(perf_cmd: *const c_char) -> c_int {
    let page = cmd_to_page(perf_cmd);
    setenv(
        b"INFOPATH\0".as_ptr() as *const c_char,
        system_path(PERF_INFO_PATH),
        1,
    );
    execlp(
        b"info\0".as_ptr() as *const c_char,
        b"info\0".as_ptr() as *const c_char,
        b"perfman\0".as_ptr() as *const c_char,
        page,
        ptr::null::<c_char>(),
    );
    -1
}

unsafe fn get_html_page_path(page_path: *mut *mut c_char, page: *const c_char) -> c_int {
    let mut st: stat = mem::zeroed();
    let html_path = system_path(PERF_HTML_PATH);
    let mut path = [0 as c_char; PATH_MAX];

    /* Check that we have a perf documentation directory. */
    if stat(
        mkpath(
            path.as_mut_ptr(),
            path.len(),
            b"%s/perf.html\0".as_ptr() as *const c_char,
            html_path,
        ),
        &mut st,
    ) != 0
        || !s_isreg(st.st_mode)
    {
        pr_err(
            b"'%s': not a documentation directory.\0".as_ptr() as *const c_char,
            html_path,
        );
        return -1;
    }

    asprintf(
        page_path,
        b"%s/%s.html\0".as_ptr() as *const c_char,
        html_path,
        page,
    )
}

/*
 * If open_html is not defined in a platform-specific way (see for
 * example compat/mingw.h), we use the script web--browse to display
 * HTML.
 */
unsafe fn open_html(path: *const c_char) {
    execl_cmd(
        b"web--browse\0".as_ptr() as *const c_char,
        b"-c\0".as_ptr() as *const c_char,
        b"help.browser\0".as_ptr() as *const c_char,
        path,
        ptr::null::<c_char>(),
    );
}

unsafe fn show_html_page(perf_cmd: *const c_char) -> c_int {
    let page = cmd_to_page(perf_cmd);
    let mut page_path: *mut c_char = ptr::null_mut(); /* it leaks but we exec below */

    if get_html_page_path(&mut page_path, page) < 0 {
        return -1;
    }

    open_html(page_path);

    0
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn cmd_help(mut argc: c_int, argv: *mut *const c_char) -> c_int {
    let mut show_all = false;
    let mut help_format: help_format = HELP_FORMAT_MAN;
    let builtin_help_options = [
        opt_boolean(
            'a' as c_int,
            b"all\0".as_ptr() as *const c_char,
            &mut show_all,
            b"print all available commands\0".as_ptr() as *const c_char,
        ),
        opt_set_uint(
            'm' as c_int,
            b"man\0".as_ptr() as *const c_char,
            &mut help_format,
            b"show man page\0".as_ptr() as *const c_char,
            HELP_FORMAT_MAN,
        ),
        opt_set_uint(
            'w' as c_int,
            b"web\0".as_ptr() as *const c_char,
            &mut help_format,
            b"show manual in web browser\0".as_ptr() as *const c_char,
            HELP_FORMAT_WEB,
        ),
        opt_set_uint(
            'i' as c_int,
            b"info\0".as_ptr() as *const c_char,
            &mut help_format,
            b"show info page\0".as_ptr() as *const c_char,
            HELP_FORMAT_INFO,
        ),
        opt_end(),
    ];
    let builtin_help_subcommands = [
        b"buildid-cache\0".as_ptr() as *const c_char,
        b"buildid-list\0".as_ptr() as *const c_char,
        b"diff\0".as_ptr() as *const c_char,
        b"evlist\0".as_ptr() as *const c_char,
        b"help\0".as_ptr() as *const c_char,
        b"list\0".as_ptr() as *const c_char,
        b"record\0".as_ptr() as *const c_char,
        b"report\0".as_ptr() as *const c_char,
        b"bench\0".as_ptr() as *const c_char,
        b"stat\0".as_ptr() as *const c_char,
        b"timechart\0".as_ptr() as *const c_char,
        b"top\0".as_ptr() as *const c_char,
        b"annotate\0".as_ptr() as *const c_char,
        b"script\0".as_ptr() as *const c_char,
        b"sched\0".as_ptr() as *const c_char,
        b"kallsyms\0".as_ptr() as *const c_char,
        b"kmem\0".as_ptr() as *const c_char,
        b"lock\0".as_ptr() as *const c_char,
        b"kvm\0".as_ptr() as *const c_char,
        b"test\0".as_ptr() as *const c_char,
        b"inject\0".as_ptr() as *const c_char,
        b"mem\0".as_ptr() as *const c_char,
        b"data\0".as_ptr() as *const c_char,
        /* HAVE_LIBELF_SUPPORT: "probe" */
        b"trace\0".as_ptr() as *const c_char,
        ptr::null(),
    ];
    let builtin_help_usage = [
        b"perf help [--all] [--man|--web|--info] [command]\0".as_ptr() as *const c_char,
        ptr::null(),
    ];
    let mut rc: c_int;

    load_command_list(
        b"perf-\0".as_ptr() as *const c_char,
        &mut main_cmds,
        &mut other_cmds,
    );

    rc = perf_config(
        Some(perf_help_config),
        &mut help_format as *mut help_format as *mut c_void,
    );
    if rc != 0 {
        return rc;
    }

    argc = parse_options_subcommand(
        argc,
        argv,
        builtin_help_options.as_ptr(),
        builtin_help_subcommands.as_ptr(),
        builtin_help_usage.as_ptr(),
        0,
    );

    if show_all {
        printf(
            b"\n Usage: %s\n\n\0".as_ptr() as *const c_char,
            perf_usage_string,
        );
        list_commands(
            b"perf commands\0".as_ptr() as *const c_char,
            &mut main_cmds,
            &mut other_cmds,
        );
        printf(
            b" %s\n\n\0".as_ptr() as *const c_char,
            perf_more_info_string,
        );
        return 0;
    }

    if (*argv).is_null() {
        printf(
            b"\n usage: %s\n\n\0".as_ptr() as *const c_char,
            perf_usage_string,
        );
        list_common_cmds_help();
        printf(
            b"\n %s\n\n\0".as_ptr() as *const c_char,
            perf_more_info_string,
        );
        return 0;
    }

    match help_format {
        HELP_FORMAT_MAN => {
            rc = show_man_page(*argv);
        }
        HELP_FORMAT_INFO => {
            rc = show_info_page(*argv);
        }
        HELP_FORMAT_WEB => {
            rc = show_html_page(*argv);
        }
        HELP_FORMAT_NONE => {
            /* fall-through */
            rc = -1;
        }
        _ => {
            rc = -1;
        }
    }

    rc
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
