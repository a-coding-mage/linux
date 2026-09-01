// SPDX-License-Identifier: GPL-2.0-only
/*
 *  (C) 2010,2011       Thomas Renninger <trenn@suse.de>, Novell Inc.
 *
 *  Output format inspired by Len Brown's <lenb@kernel.org> turbostat tool.
 */

use core::ffi::{c_char, c_double, c_int, c_long, c_uint, c_ulonglong, c_void};

const EXIT_FAILURE: c_int = 1;
const EXIT_SUCCESS: c_int = 0;
const MONITORS_MAX: usize = 32;
const MONITOR_NAME_LEN: usize = 32;
const RANGE_MAX: usize = 4;
const CLOCK_REALTIME: c_int = 0;
const SIGINT: c_int = 2;
const SIGQUIT: c_int = 3;
const SIG_IGN: usize = 1;

const MAX_COL_WIDTH: c_int = 6;
const TOPOLOGY_DEPTH_PKG: c_int = 3;
const TOPOLOGY_DEPTH_CORE: c_int = 2;
const TOPOLOGY_DEPTH_CPU: c_int = 1;

type PidT = c_int;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct timespec {
    pub tv_sec: c_long,
    pub tv_nsec: c_long,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct cstate_t {
    pub name: *const c_char,
    pub desc: *const c_char,
    pub id: c_uint,
    pub range: c_uint,
    pub get_count: Option<unsafe extern "C" fn(c_uint, *mut c_ulonglong, c_uint) -> c_int>,
    pub get_count_percent: Option<unsafe extern "C" fn(c_uint, *mut c_double, c_uint) -> c_int>,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct cpuidle_monitor_flags {
    pub needs_root: c_uint,
}

#[repr(C)]
pub struct cpuidle_monitor {
    pub name: *const c_char,
    pub hw_states_num: c_int,
    pub hw_states: *mut cstate_t,
    pub overflow_s: c_uint,
    pub flags: cpuidle_monitor_flags,
    pub do_register: Option<unsafe extern "C" fn() -> *mut cpuidle_monitor>,
    pub start: Option<unsafe extern "C" fn()>,
    pub stop: Option<unsafe extern "C" fn()>,
    pub unregister: Option<unsafe extern "C" fn()>,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct cpupower_topology_core_info {
    pub cpu: c_int,
    pub core: c_int,
    pub pkg: c_int,
    pub is_online: c_int,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct cpupower_topology {
    pub pkgs: c_int,
    pub cores: c_int,
    pub core_info: *mut cpupower_topology_core_info,
}

unsafe extern "C" {
    static mut errno: c_int;
    static mut optarg: *mut c_char;
    static mut optind: c_int;
    static mut cpus_chosen: *mut c_void;
    static mut run_as_root: c_int;

    fn printf(fmt: *const c_char, ...) -> c_int;
    fn fprintf(stream: *mut c_void, fmt: *const c_char, ...) -> c_int;
    static mut stderr: *mut c_void;
    fn perror(s: *const c_char);
    fn exit(status: c_int) -> !;
    fn strlen(s: *const c_char) -> usize;
    fn strcmp(s1: *const c_char, s2: *const c_char) -> c_int;
    fn strcpy(dest: *mut c_char, src: *const c_char) -> *mut c_char;
    fn snprintf(s: *mut c_char, n: usize, fmt: *const c_char, ...) -> c_int;
    fn sprintf(s: *mut c_char, fmt: *const c_char, ...) -> c_int;
    fn malloc(size: usize) -> *mut c_void;
    fn free(ptr: *mut c_void);
    fn memcpy(dest: *mut c_void, src: *const c_void, n: usize) -> *mut c_void;
    fn strtok(str: *mut c_char, delim: *const c_char) -> *mut c_char;
    fn atoi(nptr: *const c_char) -> c_int;
    fn basename(path: *mut c_char) -> *mut c_char;
    fn getopt(argc: c_int, argv: *const *mut c_char, optstring: *const c_char) -> c_int;
    fn fork() -> PidT;
    fn execvp(file: *const c_char, argv: *const *mut c_char) -> c_int;
    fn waitpid(pid: PidT, stat_loc: *mut c_int, options: c_int) -> PidT;
    fn signal(signum: c_int, handler: usize) -> usize;
    fn sleep(seconds: c_uint) -> c_uint;
    fn clock_gettime(clk_id: c_int, tp: *mut timespec) -> c_int;
    fn gettext(msgid: *const c_char) -> *mut c_char;

    fn bitmask_isbitset(mask: *mut c_void, bit: c_uint) -> c_int;
    fn bitmask_isallclear(mask: *mut c_void) -> c_int;
    fn bitmask_setall(mask: *mut c_void) -> c_int;
    fn bind_cpu(cpu: c_int) -> c_int;
    fn get_cpu_topology(cpu_top: *mut cpupower_topology) -> c_int;
    fn cpu_topology_release(cpu_top: cpupower_topology);
    fn dprint(fmt: *const c_char, ...);
}

macro_rules! c_str {
    ($s:literal) => {
        concat!($s, "\0").as_ptr() as *const c_char
    };
}

unsafe fn _(msgid: *const c_char) -> *mut c_char {
    gettext(msgid)
}

unsafe fn wifexited(status: c_int) -> bool {
    (status & 0x7f) == 0
}

unsafe fn wexitstatus(status: c_int) -> c_int {
    (status & 0xff00) >> 8
}

/*
 * Define pointers to all monitors.
 * C used idle_monitors.def to initialize this array:
 * #define DEF(x) & x ## _monitor ,
 * struct cpuidle_monitor *all_monitors[] = {
 * #include "idle_monitors.def"
 * 0
 * };
 */
unsafe extern "C" {
    static mut all_monitors: [*mut cpuidle_monitor; 1];
}

#[unsafe(no_mangle)]
pub static mut cpu_count: c_int = 0;

static mut monitors: [*mut cpuidle_monitor; MONITORS_MAX] = [core::ptr::null_mut(); MONITORS_MAX];
static mut avail_monitors: c_uint = 0;

static mut progname: *mut c_char = core::ptr::null_mut();

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
enum operation_mode_e {
    list = 1,
    show = 2,
    show_all = 3,
}

static mut mode: c_int = 0;
static mut interval: c_int = 1;
static mut show_monitors_param: *mut c_char = core::ptr::null_mut();
static mut cpu_top: cpupower_topology = cpupower_topology {
    pkgs: 0,
    cores: 0,
    core_info: core::ptr::null_mut(),
};
static mut wake_cpus: c_uint = 0;

/* ToDo: Document this in the manpage */
static mut range_abbr: [c_char; RANGE_MAX] = [b'T' as c_char, b'C' as c_char, b'P' as c_char, b'M' as c_char];

unsafe fn print_wrong_arg_exit() {
    printf(_(c_str!("invalid or unknown argument\n")));
    exit(EXIT_FAILURE);
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn timespec_diff_us(start: timespec, end: timespec) -> i64 {
    let mut temp: timespec = core::mem::zeroed();
    if end.tv_nsec - start.tv_nsec < 0 {
        temp.tv_sec = end.tv_sec - start.tv_sec - 1;
        temp.tv_nsec = 1000000000 + end.tv_nsec - start.tv_nsec;
    } else {
        temp.tv_sec = end.tv_sec - start.tv_sec;
        temp.tv_nsec = end.tv_nsec - start.tv_nsec;
    }
    (temp.tv_sec * 1000000 + temp.tv_nsec / 1000) as i64
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn print_n_spaces(n: c_int) {
    let mut x: c_int = 0;
    while x < n {
        printf(c_str!(" "));
        x += 1;
    }
}

/*s is filled with left and right spaces
 *to make its length atleast n+1
 */
#[unsafe(no_mangle)]
pub unsafe extern "C" fn fill_string_with_spaces(s: *mut c_char, n: c_int) -> c_int {
    let temp: *mut c_char;
    let mut len = strlen(s) as c_int;

    if len >= n {
        return -1;
    }

    temp = malloc(core::mem::size_of::<c_char>() * (n as usize + 1)) as *mut c_char;
    while len < n {
        *s.offset(len as isize) = b' ' as c_char;
        len += 1;
    }
    *s.offset(len as isize) = 0;
    snprintf(temp, n as usize + 1, c_str!(" %s"), s);
    strcpy(s, temp);
    free(temp as *mut c_void);
    0
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn print_header(topology_depth: c_int) {
    let mut mon: c_uint;
    let mut state: c_int;
    let mut need_len: c_int;
    let mut s: cstate_t;
    let mut buf: [c_char; 128] = [0; 128];

    fill_string_with_spaces(buf.as_mut_ptr(), topology_depth * 5 - 1);
    printf(c_str!("%s|"), buf.as_ptr());

    mon = 0;
    while mon < avail_monitors {
        need_len = (*monitors[mon as usize]).hw_states_num * (MAX_COL_WIDTH + 1) - 1;
        if mon != 0 {
            printf(c_str!("||"));
        }
        sprintf(buf.as_mut_ptr(), c_str!("%s"), (*monitors[mon as usize]).name);
        fill_string_with_spaces(buf.as_mut_ptr(), need_len);
        printf(c_str!("%s"), buf.as_ptr());
        mon += 1;
    }
    printf(c_str!("\n"));

    match topology_depth {
        TOPOLOGY_DEPTH_PKG => {
            printf(c_str!(" PKG|"));
            printf(c_str!("CORE|"));
            printf(c_str!(" CPU|"));
        }
        TOPOLOGY_DEPTH_CORE => {
            printf(c_str!("CORE|"));
            printf(c_str!(" CPU|"));
        }
        TOPOLOGY_DEPTH_CPU => {
            printf(c_str!(" CPU|"));
        }
        _ => return,
    }

    mon = 0;
    while mon < avail_monitors {
        if mon != 0 {
            printf(c_str!("||"));
        }
        state = 0;
        while state < (*monitors[mon as usize]).hw_states_num {
            if state != 0 {
                printf(c_str!("|"));
            }
            s = *(*monitors[mon as usize]).hw_states.offset(state as isize);
            sprintf(buf.as_mut_ptr(), c_str!("%s"), s.name);
            fill_string_with_spaces(buf.as_mut_ptr(), MAX_COL_WIDTH);
            printf(c_str!("%s"), buf.as_ptr());
            state += 1;
        }
        printf(c_str!(" "));
        mon += 1;
    }
    printf(c_str!("\n"));
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn print_results(topology_depth: c_int, cpu: c_int) {
    let mut mon: c_uint;
    let mut state: c_int;
    let mut ret: c_int;
    let mut percent: c_double = 0.0;
    let mut result: c_ulonglong = 0;
    let mut s: cstate_t;
    let info = cpu_top.core_info.offset(cpu as isize);

    /* Be careful CPUs may got resorted for pkg value do not just use cpu */
    if bitmask_isbitset(cpus_chosen, (*info).cpu as c_uint) == 0 {
        return;
    }
    if (*info).is_online == 0 && (*info).pkg == -1 {
        return;
    }

    match topology_depth {
        TOPOLOGY_DEPTH_PKG => {
            printf(c_str!("%4d|"), (*info).pkg);
            printf(c_str!("%4d|"), (*info).core);
            printf(c_str!("%4d|"), (*info).cpu);
        }
        TOPOLOGY_DEPTH_CORE => {
            printf(c_str!("%4d|"), (*info).core);
            printf(c_str!("%4d|"), (*info).cpu);
        }
        TOPOLOGY_DEPTH_CPU => {
            printf(c_str!("%4d|"), (*info).cpu);
        }
        _ => return,
    }

    mon = 0;
    while mon < avail_monitors {
        if mon != 0 {
            printf(c_str!("||"));
        }

        state = 0;
        while state < (*monitors[mon as usize]).hw_states_num {
            if state != 0 {
                printf(c_str!("|"));
            }

            s = *(*monitors[mon as usize]).hw_states.offset(state as isize);

            if let Some(get_count_percent) = s.get_count_percent {
                ret = get_count_percent(s.id, &mut percent, (*info).cpu as c_uint);
                if ret != 0 {
                    printf(c_str!("******"));
                } else if percent >= 100.0 {
                    printf(c_str!("%6.1f"), percent);
                } else {
                    printf(c_str!("%6.2f"), percent);
                }
            } else if let Some(get_count) = s.get_count {
                ret = get_count(s.id, &mut result, (*info).cpu as c_uint);
                if ret != 0 {
                    printf(c_str!("******"));
                } else {
                    printf(c_str!("%6llu"), result);
                }
            } else {
                printf(
                    _(c_str!("Monitor %s, Counter %s has no count function. Implementation error\n")),
                    (*monitors[mon as usize]).name,
                    s.name,
                );
                exit(EXIT_FAILURE);
            }
            state += 1;
        }
        mon += 1;
    }
    /*
     * The monitor could still provide useful data, for example
     * AMD HW counters partly sit in PCI config space.
     * It's up to the monitor plug-in to check .is_online, this one
     * is just for additional info.
     */
    if (*info).is_online == 0 && (*info).pkg != -1 {
        printf(_(c_str!(" *is offline\n")));
        return;
    } else {
        printf(c_str!("\n"));
    }
}

/* param: string passed by -m param (The list of monitors to show)
 *
 * Monitors must have been registered already, matching monitors
 * are picked out and available monitors array is overridden
 * with matching ones
 *
 * Monitors get sorted in the same order the user passes them
*/
unsafe fn parse_monitor_param(param: *mut c_char) {
    let mut num: c_uint;
    let mut mon: c_int;
    let mut hits: c_int = 0;
    let mut tmp = param;
    let mut token: *mut c_char;
    let mut tmp_mons: [*mut cpuidle_monitor; MONITORS_MAX] = [core::ptr::null_mut(); MONITORS_MAX];

    mon = 0;
    while mon < MONITORS_MAX as c_int {
        token = strtok(tmp, c_str!(","));
        tmp = core::ptr::null_mut();
        if token.is_null() {
            break;
        }
        if strlen(token) >= MONITOR_NAME_LEN {
            printf(_(c_str!("%s: max monitor name length (%d) exceeded\n")), token, MONITOR_NAME_LEN as c_int);
            mon += 1;
            continue;
        }

        num = 0;
        while num < avail_monitors {
            if strcmp((*monitors[num as usize]).name, token) == 0 {
                dprint(c_str!("Found requested monitor: %s\n"), token);
                tmp_mons[hits as usize] = monitors[num as usize];
                hits += 1;
            }
            num += 1;
        }
        mon += 1;
    }
    if hits == 0 {
        printf(_(c_str!("No matching monitor found in %s, try -l option\n")), param);
        exit(EXIT_FAILURE);
    }
    /* Override detected/registerd monitors array with requested one */
    memcpy(
        monitors.as_mut_ptr() as *mut c_void,
        tmp_mons.as_ptr() as *const c_void,
        core::mem::size_of::<*mut cpuidle_monitor>() * MONITORS_MAX,
    );
    avail_monitors = hits as c_uint;
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn list_monitors() {
    let mut mon: c_uint;
    let mut state: c_int;
    let mut s: cstate_t;

    mon = 0;
    while mon < avail_monitors {
        printf(
            _(c_str!("Monitor \"%s\" (%d states) - Might overflow after %u s\n")),
            (*monitors[mon as usize]).name,
            (*monitors[mon as usize]).hw_states_num,
            (*monitors[mon as usize]).overflow_s,
        );

        state = 0;
        while state < (*monitors[mon as usize]).hw_states_num {
            s = *(*monitors[mon as usize]).hw_states.offset(state as isize);
            /*
             * ToDo show more state capabilities:
             * percent, time (granlarity)
             */
            printf(
                c_str!("%s\t[%c] -> %s\n"),
                s.name,
                range_abbr[s.range as usize] as c_int,
                gettext(s.desc),
            );
            state += 1;
        }
        mon += 1;
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn fork_it(argv: *mut *mut c_char) -> c_int {
    let mut status: c_int = 0;
    let mut num: c_uint;
    let timediff: c_ulonglong;
    let child_pid: PidT;
    let mut start: timespec = core::mem::zeroed();
    let mut end: timespec = core::mem::zeroed();

    child_pid = fork();
    clock_gettime(CLOCK_REALTIME, &mut start);

    num = 0;
    while num < avail_monitors {
        if let Some(start_fn) = (*monitors[num as usize]).start {
            start_fn();
        }
        num += 1;
    }

    if child_pid == 0 {
        /* child */
        if execvp(*argv.offset(0), argv) == -1 {
            printf(c_str!("Invalid monitor command %s\n"), *argv.offset(0));
            exit(errno);
        }
    } else {
        /* parent */
        if child_pid == -1 {
            perror(c_str!("fork"));
            exit(1);
        }

        signal(SIGINT, SIG_IGN);
        signal(SIGQUIT, SIG_IGN);
        if waitpid(child_pid, &mut status, 0) == -1 {
            perror(c_str!("wait"));
            exit(1);
        }
    }
    clock_gettime(CLOCK_REALTIME, &mut end);
    num = 0;
    while num < avail_monitors {
        if let Some(stop_fn) = (*monitors[num as usize]).stop {
            stop_fn();
        }
        num += 1;
    }

    timediff = timespec_diff_us(start, end) as c_ulonglong;
    if wifexited(status) {
        printf(
            _(c_str!("%s took %.5f seconds and exited with status %d\n")),
            *argv.offset(0),
            timediff as c_double / (1000.0 * 1000.0),
            wexitstatus(status),
        );
    }
    0
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn do_interval_measure(i: c_int) -> c_int {
    let mut num: c_uint;
    let mut cpu: c_int;

    if wake_cpus != 0 {
        cpu = 0;
        while cpu < cpu_count {
            bind_cpu(cpu);
            cpu += 1;
        }
    }

    num = 0;
    while num < avail_monitors {
        dprint(
            c_str!("HW C-state residency monitor: %s - States: %d\n"),
            (*monitors[num as usize]).name,
            (*monitors[num as usize]).hw_states_num,
        );
        if let Some(start_fn) = (*monitors[num as usize]).start {
            start_fn();
        }
        num += 1;
    }

    sleep(i as c_uint);

    if wake_cpus != 0 {
        cpu = 0;
        while cpu < cpu_count {
            bind_cpu(cpu);
            cpu += 1;
        }
    }

    num = 0;
    while num < avail_monitors {
        if let Some(stop_fn) = (*monitors[num as usize]).stop {
            stop_fn();
        }
        num += 1;
    }

    0
}

unsafe fn cmdline(argc: c_int, argv: *mut *mut c_char) {
    let mut opt: c_int;
    progname = basename(*argv.offset(0));

    loop {
        opt = getopt(argc, argv, c_str!("+lci:m:"));
        if opt == -1 {
            break;
        }
        match opt as u8 as char {
            'l' => {
                if mode != 0 {
                    print_wrong_arg_exit();
                }
                mode = operation_mode_e::list as c_int;
            }
            'i' => {
                /* only allow -i with -m or no option */
                if mode != 0 && mode != operation_mode_e::show as c_int {
                    print_wrong_arg_exit();
                }
                interval = atoi(optarg);
            }
            'm' => {
                if mode != 0 {
                    print_wrong_arg_exit();
                }
                mode = operation_mode_e::show as c_int;
                show_monitors_param = optarg;
            }
            'c' => {
                wake_cpus = 1;
            }
            _ => {
                print_wrong_arg_exit();
            }
        }
    }
    if mode == 0 {
        mode = operation_mode_e::show_all as c_int;
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn cmd_monitor(argc: c_int, argv: *mut *mut c_char) -> c_int {
    let mut num: c_uint;
    let mut test_mon: *mut cpuidle_monitor;
    let mut cpu: c_int;

    cmdline(argc, argv);
    cpu_count = get_cpu_topology(&mut cpu_top);
    if cpu_count < 0 {
        printf(_(c_str!("Cannot read number of available processors\n")));
        return EXIT_FAILURE;
    }

    if (*cpu_top.core_info.offset(0)).is_online == 0 {
        printf(c_str!("WARNING: at least one cpu is offline\n"));
    }

    /* Default is: monitor all CPUs */
    if bitmask_isallclear(cpus_chosen) != 0 {
        bitmask_setall(cpus_chosen);
    }

    dprint(c_str!("System has up to %d CPU cores\n"), cpu_count);

    num = 0;
    while !all_monitors[num as usize].is_null() {
        dprint(c_str!("Try to register: %s\n"), (*all_monitors[num as usize]).name);
        test_mon = if let Some(do_register) = (*all_monitors[num as usize]).do_register {
            do_register()
        } else {
            core::ptr::null_mut()
        };
        if !test_mon.is_null() {
            if (*test_mon).flags.needs_root != 0 && run_as_root == 0 {
                fprintf(
                    stderr,
                    _(c_str!("Available monitor %s needs root access\n")),
                    (*test_mon).name,
                );
                num += 1;
                continue;
            }
            monitors[avail_monitors as usize] = test_mon;
            dprint(c_str!("%s registered\n"), (*all_monitors[num as usize]).name);
            avail_monitors += 1;
        }
        num += 1;
    }

    if avail_monitors == 0 {
        printf(_(c_str!("No HW Cstate monitors found\n")));
        cpu_topology_release(cpu_top);
        return 1;
    }

    if mode == operation_mode_e::list as c_int {
        list_monitors();
        cpu_topology_release(cpu_top);
        exit(EXIT_SUCCESS);
    }

    if mode == operation_mode_e::show as c_int {
        parse_monitor_param(show_monitors_param);
    }

    dprint(
        c_str!("Packages: %d - Cores: %d - CPUs: %d\n"),
        cpu_top.pkgs,
        cpu_top.cores,
        cpu_count,
    );

    /*
     * if any params left, it must be a command to fork
     */
    if argc - optind != 0 {
        fork_it(argv.offset(optind as isize));
    } else {
        do_interval_measure(interval);
    }

    /* ToDo: Topology parsing needs fixing first to do
       this more generically */
    if cpu_top.pkgs > 1 {
        print_header(TOPOLOGY_DEPTH_PKG);
    } else {
        print_header(TOPOLOGY_DEPTH_CPU);
    }

    cpu = 0;
    while cpu < cpu_count {
        if cpu_top.pkgs > 1 {
            print_results(TOPOLOGY_DEPTH_PKG, cpu);
        } else {
            print_results(TOPOLOGY_DEPTH_CPU, cpu);
        }
        cpu += 1;
    }

    num = 0;
    while num < avail_monitors {
        if let Some(unregister) = (*monitors[num as usize]).unregister {
            unregister();
        }
        num += 1;
    }
    cpu_topology_release(cpu_top);
    0
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
