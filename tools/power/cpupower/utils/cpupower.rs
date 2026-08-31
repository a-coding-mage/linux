// SPDX-License-Identifier: GPL-2.0-only
/*
 *  (C) 2010,2011       Thomas Renninger <trenn@suse.de>, Novell Inc.
 *
 *  Ideas taken over from the perf userspace tool (included in the Linus
 *  kernel git repo): subcommand builtins and param parsing.
 */

#![allow(non_camel_case_types)]
#![allow(non_upper_case_globals)]
#![allow(non_snake_case)]
#![allow(improper_ctypes)]

use core::ffi::{c_char, c_int, c_long, c_uint, c_void};

const EXIT_SUCCESS: c_int = 0;
const EXIT_FAILURE: c_int = 1;
const ENOMEM: c_int = 12;
const EINVAL: c_int = 22;
const LC_ALL: c_int = 6;
const _SC_NPROCESSORS_CONF: c_int = 83;

const PACKAGE: &[u8] = b"cpupower\0";
const VERSION: &[u8] = b"\0";
const PACKAGE_BUGREPORT: &[u8] = b"\0";

#[repr(C)]
pub struct cpupower_cpu_info {
    _private: [u8; 0],
}

#[repr(C)]
pub struct bitmask {
    _private: [u8; 0],
}

#[repr(C)]
pub struct stat {
    _private: [u8; 0],
}

#[repr(C)]
pub struct utsname {
    pub sysname: [c_char; 65],
    pub nodename: [c_char; 65],
    pub release: [c_char; 65],
    pub version: [c_char; 65],
    pub machine: [c_char; 65],
}

unsafe extern "C" {
    fn printf(format: *const c_char, ...) -> c_int;
    fn fprintf(stream: *mut c_void, format: *const c_char, ...) -> c_int;
    static mut stderr: *mut c_void;

    fn malloc(size: usize) -> *mut c_void;
    fn exit(status: c_int) -> !;
    fn system(command: *const c_char) -> c_int;

    fn strlen(s: *const c_char) -> usize;
    fn strcmp(s1: *const c_char, s2: *const c_char) -> c_int;
    fn sprintf(s: *mut c_char, format: *const c_char, ...) -> c_int;
    fn strcat(dest: *mut c_char, src: *const c_char) -> *mut c_char;

    fn execlp(file: *const c_char, arg: *const c_char, ...) -> c_int;
    fn geteuid() -> c_uint;
    fn sysconf(name: c_int) -> c_long;

    fn sched_getcpu() -> c_int;
    fn stat(pathname: *const c_char, statbuf: *mut stat) -> c_int;
    fn uname(buf: *mut utsname) -> c_int;

    fn setlocale(category: c_int, locale: *const c_char) -> *mut c_char;
    fn textdomain(domainname: *const c_char) -> *mut c_char;
    fn r#_(msgid: *const c_char) -> *const c_char;

    fn cmd_freq_info(argc: c_int, argv: *mut *const c_char) -> c_int;
    fn cmd_freq_set(argc: c_int, argv: *mut *const c_char) -> c_int;
    fn cmd_idle_info(argc: c_int, argv: *mut *const c_char) -> c_int;
    fn cmd_idle_set(argc: c_int, argv: *mut *const c_char) -> c_int;
    fn cmd_cap_info(argc: c_int, argv: *mut *const c_char) -> c_int;
    fn cmd_set(argc: c_int, argv: *mut *const c_char) -> c_int;
    fn cmd_info(argc: c_int, argv: *mut *const c_char) -> c_int;
    fn cmd_monitor(argc: c_int, argv: *mut *const c_char) -> c_int;

    fn bitmask_alloc(nbits: c_long) -> *mut bitmask;
    fn bitmask_setall(mask: *mut bitmask);
    fn bitmask_parselist(buf: *const c_char, mask: *mut bitmask) -> c_int;
    fn bitmask_free(mask: *mut bitmask);
    fn get_cpu_info(cpu_info: *mut cpupower_cpu_info);
}

static mut cpupower_cpu_info: cpupower_cpu_info = cpupower_cpu_info { _private: [] };
static mut run_as_root: c_int = 0;
static mut base_cpu: c_int = 0;
/* Affected cpus chosen by -c/--cpu param */
static mut cpus_chosen: *mut bitmask = core::ptr::null_mut();
static mut online_cpus: *mut bitmask = core::ptr::null_mut();
static mut offline_cpus: *mut bitmask = core::ptr::null_mut();

/* #ifdef DEBUG */
static mut be_verbose: c_int = 0;
/* #endif */

unsafe fn print_help();

#[repr(C)]
struct cmd_struct {
    cmd: *const c_char,
    main: Option<unsafe extern "C" fn(c_int, *mut *const c_char) -> c_int>,
    needs_root: c_int,
}

unsafe extern "C" fn cmd_help(argc: c_int, argv: *mut *const c_char) -> c_int {
    if argc > 1 {
        print_man_page(*argv.add(1)); /* exits within execlp() */
        return EXIT_FAILURE;
    }

    print_help();
    EXIT_SUCCESS
}

static mut commands: [cmd_struct; 9] = [
    cmd_struct { cmd: b"frequency-info\0".as_ptr() as *const c_char, main: Some(cmd_freq_info), needs_root: 0 },
    cmd_struct { cmd: b"frequency-set\0".as_ptr() as *const c_char, main: Some(cmd_freq_set), needs_root: 1 },
    cmd_struct { cmd: b"idle-info\0".as_ptr() as *const c_char, main: Some(cmd_idle_info), needs_root: 0 },
    cmd_struct { cmd: b"idle-set\0".as_ptr() as *const c_char, main: Some(cmd_idle_set), needs_root: 1 },
    cmd_struct { cmd: b"powercap-info\0".as_ptr() as *const c_char, main: Some(cmd_cap_info), needs_root: 0 },
    cmd_struct { cmd: b"set\0".as_ptr() as *const c_char, main: Some(cmd_set), needs_root: 1 },
    cmd_struct { cmd: b"info\0".as_ptr() as *const c_char, main: Some(cmd_info), needs_root: 0 },
    cmd_struct { cmd: b"monitor\0".as_ptr() as *const c_char, main: Some(cmd_monitor), needs_root: 0 },
    cmd_struct { cmd: b"help\0".as_ptr() as *const c_char, main: Some(cmd_help), needs_root: 0 },
    /*  { "bench", cmd_bench, 1 }, */
];

unsafe fn print_help() {
    let mut i: c_uint;

    /* #ifdef DEBUG */
    printf(r#_(b"Usage:\tcpupower [-d|--debug] [-c|--cpu cpulist ] <command> [<args>]\n\0".as_ptr() as *const c_char));
    /* #else
     * printf(_("Usage:\tcpupower [-c|--cpu cpulist ] <command> [<args>]\n"));
     * #endif
     */
    printf(r#_(b"Supported commands are:\n\0".as_ptr() as *const c_char));
    i = 0;
    while (i as usize) < commands.len() {
        printf(b"\t%s\n\0".as_ptr() as *const c_char, commands[i as usize].cmd);
        i += 1;
    }
    printf(r#_(b"\nNot all commands can make use of the -c cpulist option.\n\0".as_ptr() as *const c_char));
    printf(r#_(b"\nUse 'cpupower help <command>' for getting help for above commands.\n\0".as_ptr() as *const c_char));
}

unsafe fn print_man_page(subpage: *const c_char) -> c_int {
    let mut len: c_int;
    let page: *mut c_char;

    len = 10; /* enough for "cpupower-" */
    if !subpage.is_null() {
        len += strlen(subpage) as c_int;
    }

    page = malloc(len as usize) as *mut c_char;
    if page.is_null() {
        return -ENOMEM;
    }

    sprintf(page, b"cpupower\0".as_ptr() as *const c_char);
    if !subpage.is_null() && strcmp(subpage, b"help\0".as_ptr() as *const c_char) != 0 {
        strcat(page, b"-\0".as_ptr() as *const c_char);
        strcat(page, subpage);
    }

    execlp(
        b"man\0".as_ptr() as *const c_char,
        b"man\0".as_ptr() as *const c_char,
        page,
        core::ptr::null::<c_char>(),
    );

    /* should not be reached */
    -EINVAL
}

unsafe fn print_version() {
    printf(
        b"%s %s\n\0".as_ptr() as *const c_char,
        PACKAGE.as_ptr() as *const c_char,
        VERSION.as_ptr() as *const c_char,
    );
    printf(
        r#_(b"Report errors and bugs to %s, please.\n\0".as_ptr() as *const c_char),
        PACKAGE_BUGREPORT.as_ptr() as *const c_char,
    );
}

unsafe fn handle_options(argc: *mut c_int, argv: *mut *mut *const c_char) {
    let mut ret: c_int;
    let mut x: c_int;
    let mut new_argc: c_int = 0;

    if *argc < 1 {
        return;
    }

    x = 0;
    while x < *argc && (**argv).add(x as usize).read().read() == b'-' as c_char {
        let param: *const c_char = *(**argv).add(x as usize);
        if strcmp(param, b"-h\0".as_ptr() as *const c_char) == 0
            || strcmp(param, b"--help\0".as_ptr() as *const c_char) == 0
        {
            print_help();
            exit(EXIT_SUCCESS);
        } else if strcmp(param, b"-c\0".as_ptr() as *const c_char) == 0
            || strcmp(param, b"--cpu\0".as_ptr() as *const c_char) == 0
        {
            if *argc < 2 {
                print_help();
                exit(EXIT_FAILURE);
            }
            if strcmp(*(**argv).add((x + 1) as usize), b"all\0".as_ptr() as *const c_char) == 0 {
                bitmask_setall(cpus_chosen);
            } else {
                ret = bitmask_parselist(*(**argv).add((x + 1) as usize), cpus_chosen);
                if ret < 0 {
                    fprintf(stderr, r#_(b"Error parsing cpu list\n\0".as_ptr() as *const c_char));
                    exit(EXIT_FAILURE);
                }
            }
            x += 1;
            /* Cut out param: cpupower -c 1 info -> cpupower info */
            new_argc += 2;
        } else if strcmp(param, b"-v\0".as_ptr() as *const c_char) == 0
            || strcmp(param, b"--version\0".as_ptr() as *const c_char) == 0
        {
            print_version();
            exit(EXIT_SUCCESS);
        /* #ifdef DEBUG */
        } else if strcmp(param, b"-d\0".as_ptr() as *const c_char) == 0
            || strcmp(param, b"--debug\0".as_ptr() as *const c_char) == 0
        {
            be_verbose = 1;
            new_argc += 1;
        /* #endif */
        } else {
            fprintf(stderr, b"Unknown option: %s\n\0".as_ptr() as *const c_char, param);
            print_help();
            exit(EXIT_FAILURE);
        }
        x += 1;
    }
    *argc -= new_argc;
    *argv = (*argv).add(new_argc as usize);
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn main(mut argc: c_int, mut argv: *mut *const c_char) -> c_int {
    let mut cmd: *const c_char;
    let mut i: c_uint;
    let mut ret: c_uint;
    let mut statbuf: stat = core::mem::zeroed();
    let mut uts: utsname = core::mem::zeroed();
    let mut pathname: [c_char; 32] = [0; 32];

    cpus_chosen = bitmask_alloc(sysconf(_SC_NPROCESSORS_CONF));
    online_cpus = bitmask_alloc(sysconf(_SC_NPROCESSORS_CONF));
    offline_cpus = bitmask_alloc(sysconf(_SC_NPROCESSORS_CONF));

    argc -= 1;
    argv = argv.add(1);

    handle_options(&mut argc, &mut argv);

    cmd = *argv.add(0);

    if argc < 1 {
        print_help();
        return EXIT_FAILURE;
    }

    setlocale(LC_ALL, b"\0".as_ptr() as *const c_char);
    textdomain(PACKAGE.as_ptr() as *const c_char);

    /* Turn "perf cmd --help" into "perf help cmd" */
    if argc > 1 && strcmp(*argv.add(1), b"--help\0".as_ptr() as *const c_char) == 0 {
        *argv.add(1) = *argv.add(0);
        *argv.add(0) = b"help\0".as_ptr() as *const c_char;
        cmd = *argv.add(0);
    }

    base_cpu = sched_getcpu();
    if base_cpu < 0 {
        fprintf(stderr, r#_(b"No valid cpus found.\n\0".as_ptr() as *const c_char));
        return EXIT_FAILURE;
    }

    get_cpu_info(&mut cpupower_cpu_info);
    run_as_root = (geteuid() == 0) as c_int;
    if run_as_root != 0 {
        ret = uname(&mut uts) as c_uint;
        sprintf(
            pathname.as_mut_ptr(),
            b"/dev/cpu/%d/msr\0".as_ptr() as *const c_char,
            base_cpu,
        );
        if ret == 0
            && strcmp(uts.machine.as_ptr(), b"x86_64\0".as_ptr() as *const c_char) == 0
            && stat(pathname.as_ptr(), &mut statbuf) != 0
        {
            if system(b"modprobe msr\0".as_ptr() as *const c_char) == -1 {
                fprintf(stderr, r#_(b"MSR access not available.\n\0".as_ptr() as *const c_char));
            }
        }
    }

    i = 0;
    while (i as usize) < commands.len() {
        let p: *mut cmd_struct = commands.as_mut_ptr().add(i as usize);
        if strcmp((*p).cmd, cmd) != 0 {
            i += 1;
            continue;
        }
        if run_as_root == 0 && (*p).needs_root != 0 {
            fprintf(
                stderr,
                r#_(b"Subcommand %s needs root privileges\n\0".as_ptr() as *const c_char),
                cmd,
            );
            return EXIT_FAILURE;
        }
        ret = ((*p).main.unwrap())(argc, argv) as c_uint;
        if !cpus_chosen.is_null() {
            bitmask_free(cpus_chosen);
        }
        if !online_cpus.is_null() {
            bitmask_free(online_cpus);
        }
        if !offline_cpus.is_null() {
            bitmask_free(offline_cpus);
        }
        return ret as c_int;
    }
    print_help();
    EXIT_FAILURE
}
