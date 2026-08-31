// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * tmon.c Thermal Monitor (TMON) main function and entry point
 *
 * Copyright (C) 2012 Intel Corporation. All rights reserved.
 *
 * Author: Jacob Pan <jacob.jun.pan@linux.intel.com>
 */

use core::ffi::c_void;
use libc::{
    c_char, c_double, c_int, c_long, c_short, c_uint, c_ulong, mode_t, pid_t, size_t, FILE,
};

const EXIT_FAILURE: c_int = 1;
const EXIT_SUCCESS: c_int = 0;
const FALSE: c_int = 0;
const TRUE: c_int = 1;
const LOG_INFO: c_int = 6;
const LOG_NOTICE: c_int = 5;
const LOG_ERR: c_int = 3;
const LOG_DEBUG: c_int = 7;
const LOG_CONS: c_int = 0x02;
const LOG_PID: c_int = 0x01;
const LOG_NDELAY: c_int = 0x08;
const LOG_LOCAL0: c_int = 16 << 3;
const SIGINT: c_int = 2;
const SIGTERM: c_int = 15;
const SIGKILL: c_int = 9;
const SIGWINCH: c_int = 28;
const SIG_ERR: usize = !0usize;
const STDIN_FILENO: c_int = 0;
const STDOUT_FILENO: c_int = 1;
const STDERR_FILENO: c_int = 2;
const S_IWGRP: mode_t = 0o020;
const S_IWOTH: mode_t = 0o002;
const CDEV_NAME_SIZE: usize = 32;
const VERSION: &[u8] = b"1.0\0";
const TMON_LOG_FILE: &[u8] = b"/var/tmp/tmon.log\0";

fn LOG_UPTO(pri: c_int) -> c_int {
    (1 << (pri + 1)) - 1
}

fn S_ISLNK(m: mode_t) -> bool {
    (m & libc::S_IFMT) == libc::S_IFLNK
}

#[repr(C)]
pub struct Option {
    pub name: *const c_char,
    pub has_arg: c_int,
    pub flag: *mut c_int,
    pub val: c_int,
}

#[repr(C)]
pub struct WINDOW {
    _private: [u8; 0],
}

#[repr(C)]
pub struct pthread_t {
    _private: c_ulong,
}

#[repr(C)]
pub struct pthread_mutex_t {
    _private: [u8; 40],
}

#[repr(C)]
pub struct stat {
    pub st_dev: libc::dev_t,
    pub st_ino: libc::ino_t,
    pub st_nlink: libc::nlink_t,
    pub st_mode: mode_t,
    pub st_uid: libc::uid_t,
    pub st_gid: libc::gid_t,
    pub __pad0: c_int,
    pub st_rdev: libc::dev_t,
    pub st_size: libc::off_t,
    pub st_blksize: libc::blksize_t,
    pub st_blocks: libc::blkcnt_t,
    pub st_atime: libc::time_t,
    pub st_atime_nsec: c_long,
    pub st_mtime: libc::time_t,
    pub st_mtime_nsec: c_long,
    pub st_ctime: libc::time_t,
    pub st_ctime_nsec: c_long,
    pub __glibc_reserved: [c_long; 3],
}

#[repr(C)]
pub struct thermal_trip_point {
    pub type_: c_int,
    pub temp: c_ulong,
}

#[repr(C)]
pub struct thermal_zone_info {
    pub type_: [c_char; 32],
    pub instance: c_int,
    pub cdev_binding: c_uint,
    pub nr_trip_pts: c_int,
    pub tp: [thermal_trip_point; 32],
}

#[repr(C)]
pub struct cooling_dev_info {
    pub type_: [c_char; 32],
    pub instance: c_int,
}

#[repr(C)]
pub struct thermal_data {
    pub nr_tz_sensor: c_int,
    pub nr_cooling_dev: c_int,
    pub tzi: *mut thermal_zone_info,
    pub cdi: *mut cooling_dev_info,
}

#[repr(C)]
pub struct thermal_record {
    pub temp: *mut c_ulong,
    pub pid_out_pct: c_double,
}

pub static mut ticktime: c_ulong = 1; /* seconds */
pub static mut no_control: c_ulong = 1; /* monitoring only or use cooling device for
                                         * temperature control.
                                         */
pub static mut time_elapsed: c_double = 0.0;
pub static mut target_temp_user: c_ulong = 65; /* can be select by tui later */
pub static mut dialogue_on: c_int = 0;
pub static mut tmon_exit: c_int = 0;
static mut daemon_mode: c_short = 0;
static mut logging: c_int = 0; /* for recording thermal data to a file */
static mut debug_on: c_int = 0;
pub static mut tmon_log: *mut FILE = core::ptr::null_mut();
/*cooling device used for the PID controller */
pub static mut ctrl_cdev: [c_char; CDEV_NAME_SIZE] = [
    b'N' as c_char,
    b'o' as c_char,
    b'n' as c_char,
    b'e' as c_char,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
];
pub static mut target_thermal_zone: c_int = 0; /* user selected target zone instance */

pub static mut event_tid: pthread_t = pthread_t { _private: 0 };
pub static mut input_lock: pthread_mutex_t = pthread_mutex_t { _private: [0; 40] };

unsafe extern "C" {
    static mut optarg: *mut c_char;
    static mut stderr: *mut FILE;
    static mut stdout: *mut FILE;
    static mut stdscr: *mut WINDOW;
    static mut ptdata: thermal_data;
    static mut trec: *mut thermal_record;
    static trip_type_name: *mut *const c_char;

    fn printf(format: *const c_char, ...) -> c_int;
    fn fprintf(stream: *mut FILE, format: *const c_char, ...) -> c_int;
    fn fflush(stream: *mut FILE) -> c_int;
    fn fclose(stream: *mut FILE) -> c_int;
    fn fopen(pathname: *const c_char, mode: *const c_char) -> *mut FILE;
    fn exit(status: c_int) -> !;
    fn strtod(nptr: *const c_char, endptr: *mut *mut c_char) -> c_double;
    fn strncpy(dest: *mut c_char, src: *const c_char, n: size_t) -> *mut c_char;
    fn strerror(errnum: c_int) -> *mut c_char;
    fn memset(s: *mut c_void, c: c_int, n: size_t) -> *mut c_void;
    fn getopt_long(
        argc: c_int,
        argv: *mut *mut c_char,
        optstring: *const c_char,
        longopts: *const Option,
        longindex: *mut c_int,
    ) -> c_int;
    fn getuid() -> libc::uid_t;
    fn geteuid() -> libc::uid_t;
    fn sleep(seconds: c_uint) -> c_uint;
    fn lstat(pathname: *const c_char, statbuf: *mut stat) -> c_int;
    fn setlogmask(maskpri: c_int) -> c_int;
    fn openlog(ident: *const c_char, option: c_int, facility: c_int);
    fn syslog(priority: c_int, format: *const c_char, ...);
    fn closelog();
    fn signal(signum: c_int, handler: unsafe extern "C" fn(c_int)) -> usize;
    fn refresh() -> c_int;
    fn keypad(win: *mut WINDOW, bf: c_int) -> c_int;
    fn echo() -> c_int;
    fn nocbreak() -> c_int;
    fn endwin() -> c_int;
    fn nodelay(win: *mut WINDOW, bf: c_int) -> c_int;
    fn pthread_mutex_init(mutex: *mut pthread_mutex_t, attr: *const c_void) -> c_int;
    fn pthread_mutex_lock(mutex: *mut pthread_mutex_t) -> c_int;
    fn pthread_mutex_unlock(mutex: *mut pthread_mutex_t) -> c_int;
    fn pthread_mutex_destroy(mutex: *mut pthread_mutex_t) -> c_int;
    fn pthread_cancel(thread: pthread_t) -> c_int;
    fn pthread_create(
        thread: *mut pthread_t,
        attr: *const c_void,
        start_routine: *const c_void,
        arg: *mut c_void,
    ) -> c_int;
    fn fork() -> pid_t;
    fn umask(mask: mode_t) -> mode_t;
    fn setsid() -> pid_t;
    fn chdir(path: *const c_char) -> c_int;
    fn close(fd: c_int) -> c_int;

    fn set_ctrl_state(state: c_int);
    fn close_windows();
    fn free_thermal_data();
    fn probe_thermal_sysfs() -> c_int;
    fn initialize_curses();
    fn setup_windows();
    fn resize_handler(sig: c_int);
    fn show_title_bar();
    fn show_sensors_w();
    fn show_cooling_device();
    fn update_thermal_data();
    fn show_data_w();
    fn init_thermal_controller();
    fn handle_tui_events(arg: *mut c_void) -> *mut c_void;
    fn zone_instance_to_index(zone: c_int) -> c_int;
    fn controller_handler(temp: c_ulong, yk: *mut c_double);
    fn show_control_w();
    fn disable_tui();
}

pub unsafe extern "C" fn usage() {
    unsafe {
        printf(c"Usage: tmon [OPTION...]\n".as_ptr());
        printf(c"  -c, --control         cooling device in control\n".as_ptr());
        printf(c"  -d, --daemon          run as daemon, no TUI\n".as_ptr());
        printf(c"  -g, --debug           debug message in syslog\n".as_ptr());
        printf(c"  -h, --help            show this help message\n".as_ptr());
        printf(c"  -l, --log             log data to /var/tmp/tmon.log\n".as_ptr());
        printf(c"  -t, --time-interval   sampling time interval, > 1 sec.\n".as_ptr());
        printf(c"  -T, --target-temp     initial target temperature\n".as_ptr());
        printf(c"  -v, --version         show version\n".as_ptr());
        printf(c"  -z, --zone            target thermal zone id\n".as_ptr());

        exit(0);
    }
}

pub unsafe extern "C" fn version() {
    unsafe {
        printf(c"TMON version %s\n".as_ptr(), VERSION.as_ptr());
        exit(EXIT_SUCCESS);
    }
}

unsafe extern "C" fn tmon_cleanup() {
    unsafe {
        syslog(LOG_INFO, c"TMON exit cleanup\n".as_ptr());
        fflush(stdout);
        refresh();
        if !tmon_log.is_null() {
            fclose(tmon_log);
        }
        if event_tid._private != 0 {
            pthread_mutex_lock(&raw mut input_lock);
            pthread_cancel(event_tid);
            pthread_mutex_unlock(&raw mut input_lock);
            pthread_mutex_destroy(&raw mut input_lock);
        }
        closelog();
        /* relax control knobs, undo throttling */
        set_ctrl_state(0);

        keypad(stdscr, FALSE);
        echo();
        nocbreak();
        close_windows();
        endwin();
        free_thermal_data();

        exit(1);
    }
}

unsafe extern "C" fn tmon_sig_handler(sig: c_int) {
    unsafe {
        syslog(LOG_INFO, c"TMON caught signal %d\n".as_ptr(), sig);
        refresh();
        match sig {
            SIGTERM => {
                printf(c"sigterm, exit and clean up\n".as_ptr());
                fflush(stdout);
            }
            SIGKILL => {
                printf(c"sigkill, exit and clean up\n".as_ptr());
                fflush(stdout);
            }
            SIGINT => {
                printf(c"ctrl-c, exit and clean up\n".as_ptr());
                fflush(stdout);
            }
            _ => {}
        }
        tmon_exit = true as c_int;
    }
}

unsafe extern "C" fn start_syslog() {
    unsafe {
        if debug_on != 0 {
            setlogmask(LOG_UPTO(LOG_DEBUG));
        } else {
            setlogmask(LOG_UPTO(LOG_ERR));
        }
        openlog(
            c"tmon.log".as_ptr(),
            LOG_CONS | LOG_PID | LOG_NDELAY,
            LOG_LOCAL0,
        );
        syslog(LOG_NOTICE, c"TMON started by User %d".as_ptr(), getuid());
    }
}

unsafe extern "C" fn prepare_logging() {
    unsafe {
        let mut i: c_int;
        let mut logstat: stat = core::mem::zeroed();

        if logging == 0 {
            return;
        }
        /* open local data log file */
        tmon_log = fopen(TMON_LOG_FILE.as_ptr(), c"w+".as_ptr());
        if tmon_log.is_null() {
            syslog(
                LOG_ERR,
                c"failed to open log file %s\n".as_ptr(),
                TMON_LOG_FILE.as_ptr(),
            );
            return;
        }

        if lstat(TMON_LOG_FILE.as_ptr(), &mut logstat) < 0 {
            syslog(
                LOG_ERR,
                c"Unable to stat log file %s\n".as_ptr(),
                TMON_LOG_FILE.as_ptr(),
            );
            fclose(tmon_log);
            tmon_log = core::ptr::null_mut();
            return;
        }

        /* The log file must be a regular file owned by us */
        if S_ISLNK(logstat.st_mode) {
            syslog(LOG_ERR, c"Log file is a symlink.  Will not log\n".as_ptr());
            fclose(tmon_log);
            tmon_log = core::ptr::null_mut();
            return;
        }

        if logstat.st_uid != getuid() {
            syslog(LOG_ERR, c"We don't own the log file.  Not logging\n".as_ptr());
            fclose(tmon_log);
            tmon_log = core::ptr::null_mut();
            return;
        }

        fprintf(
            tmon_log,
            c"#----------- THERMAL SYSTEM CONFIG -------------\n".as_ptr(),
        );
        i = 0;
        while i < ptdata.nr_tz_sensor {
            let mut binding_str: [c_char; 33] = [0; 33]; /* size of long + 1 */
            let mut j: c_int;

            memset(
                binding_str.as_mut_ptr() as *mut c_void,
                0,
                core::mem::size_of_val(&binding_str),
            );
            j = 0;
            while j < 32 {
                binding_str[j as usize] =
                    if ((*ptdata.tzi.add(i as usize)).cdev_binding & (1 << j)) != 0 {
                        b'1' as c_char
                    } else {
                        b'0' as c_char
                    };
                j += 1;
            }

            fprintf(
                tmon_log,
                c"#thermal zone %s%02d cdevs binding: %32s\n".as_ptr(),
                (*ptdata.tzi.add(i as usize)).type_.as_ptr(),
                (*ptdata.tzi.add(i as usize)).instance,
                binding_str.as_ptr(),
            );
            j = 0;
            while j < (*ptdata.tzi.add(i as usize)).nr_trip_pts {
                fprintf(
                    tmon_log,
                    c"#\tTP%02d type:%s, temp:%lu\n".as_ptr(),
                    j,
                    *trip_type_name.add((*ptdata.tzi.add(i as usize)).tp[j as usize].type_ as usize),
                    (*ptdata.tzi.add(i as usize)).tp[j as usize].temp,
                );
                j += 1;
            }
            i += 1;
        }

        i = 0;
        while i < ptdata.nr_cooling_dev {
            fprintf(
                tmon_log,
                c"#cooling devices%02d: %s\n".as_ptr(),
                i,
                (*ptdata.cdi.add(i as usize)).type_.as_ptr(),
            );
            i += 1;
        }

        fprintf(
            tmon_log,
            c"#---------- THERMAL DATA LOG STARTED -----------\n".as_ptr(),
        );
        fprintf(tmon_log, c"Samples TargetTemp ".as_ptr());
        i = 0;
        while i < ptdata.nr_tz_sensor {
            fprintf(
                tmon_log,
                c"%s%d    ".as_ptr(),
                (*ptdata.tzi.add(i as usize)).type_.as_ptr(),
                (*ptdata.tzi.add(i as usize)).instance,
            );
            i += 1;
        }
        i = 0;
        while i < ptdata.nr_cooling_dev {
            fprintf(
                tmon_log,
                c"%s%d ".as_ptr(),
                (*ptdata.cdi.add(i as usize)).type_.as_ptr(),
                (*ptdata.cdi.add(i as usize)).instance,
            );
            i += 1;
        }

        fprintf(tmon_log, c"\n".as_ptr());
    }
}

static mut opts: [Option; 9] = [
    Option {
        name: c"control".as_ptr(),
        has_arg: 1,
        flag: core::ptr::null_mut(),
        val: b'c' as c_int,
    },
    Option {
        name: c"daemon".as_ptr(),
        has_arg: 0,
        flag: core::ptr::null_mut(),
        val: b'd' as c_int,
    },
    Option {
        name: c"time-interval".as_ptr(),
        has_arg: 1,
        flag: core::ptr::null_mut(),
        val: b't' as c_int,
    },
    Option {
        name: c"target-temp".as_ptr(),
        has_arg: 1,
        flag: core::ptr::null_mut(),
        val: b'T' as c_int,
    },
    Option {
        name: c"log".as_ptr(),
        has_arg: 0,
        flag: core::ptr::null_mut(),
        val: b'l' as c_int,
    },
    Option {
        name: c"help".as_ptr(),
        has_arg: 0,
        flag: core::ptr::null_mut(),
        val: b'h' as c_int,
    },
    Option {
        name: c"version".as_ptr(),
        has_arg: 0,
        flag: core::ptr::null_mut(),
        val: b'v' as c_int,
    },
    Option {
        name: c"debug".as_ptr(),
        has_arg: 0,
        flag: core::ptr::null_mut(),
        val: b'g' as c_int,
    },
    Option {
        name: core::ptr::null(),
        has_arg: 0,
        flag: core::ptr::null_mut(),
        val: 0,
    },
];

#[unsafe(no_mangle)]
pub unsafe extern "C" fn main(argc: c_int, argv: *mut *mut c_char) -> c_int {
    unsafe {
        let mut err: c_int = 0;
        let mut id2: c_int = 0;
        let mut c: c_int;
        let mut yk: c_double = 0.0;
        let mut temp: c_double; /* controller output */
        let mut target_tz_index: c_int;

        if geteuid() != 0 {
            printf(c"TMON needs to be run as root\n".as_ptr());
            exit(EXIT_FAILURE);
        }

        loop {
            c = getopt_long(
                argc,
                argv,
                c"c:dlht:T:vgz:".as_ptr(),
                (&raw const opts) as *const Option,
                &mut id2,
            );
            if c == -1 {
                break;
            }
            match c {
                x if x == b'c' as c_int => {
                    no_control = 0;
                    strncpy(ctrl_cdev.as_mut_ptr(), optarg, CDEV_NAME_SIZE);
                }
                x if x == b'd' as c_int => {
                    start_daemon_mode();
                    printf(c"Run TMON in daemon mode\n".as_ptr());
                }
                x if x == b't' as c_int => {
                    ticktime = strtod(optarg, core::ptr::null_mut()) as c_ulong;
                    if ticktime < 1 {
                        ticktime = 1;
                    }
                }
                x if x == b'T' as c_int => {
                    temp = strtod(optarg, core::ptr::null_mut());
                    if temp < 0.0 {
                        fprintf(stderr, c"error: temperature must be positive\n".as_ptr());
                        return 1;
                    }
                    target_temp_user = temp as c_ulong;
                }
                x if x == b'l' as c_int => {
                    printf(c"Logging data to /var/tmp/tmon.log\n".as_ptr());
                    logging = 1;
                }
                x if x == b'h' as c_int => {
                    usage();
                }
                x if x == b'v' as c_int => {
                    version();
                }
                x if x == b'g' as c_int => {
                    debug_on = 1;
                }
                x if x == b'z' as c_int => {
                    target_thermal_zone = strtod(optarg, core::ptr::null_mut()) as c_int;
                }
                _ => {}
            }
        }
        if pthread_mutex_init(&raw mut input_lock, core::ptr::null()) != 0 {
            fprintf(stderr, c"\n mutex init failed, exit\n".as_ptr());
            return 1;
        }
        start_syslog();
        if signal(SIGINT, tmon_sig_handler) == SIG_ERR {
            syslog(LOG_DEBUG, c"Cannot handle SIGINT\n".as_ptr());
        }
        if signal(SIGTERM, tmon_sig_handler) == SIG_ERR {
            syslog(LOG_DEBUG, c"Cannot handle SIGTERM\n".as_ptr());
        }

        if probe_thermal_sysfs() != 0 {
            pthread_mutex_destroy(&raw mut input_lock);
            closelog();
            return -1;
        }
        initialize_curses();
        setup_windows();
        signal(SIGWINCH, resize_handler);
        show_title_bar();
        show_sensors_w();
        show_cooling_device();
        update_thermal_data();
        show_data_w();
        prepare_logging();
        init_thermal_controller();

        nodelay(stdscr, TRUE);
        err = pthread_create(
            &raw mut event_tid,
            core::ptr::null(),
            handle_tui_events as *const c_void,
            core::ptr::null_mut(),
        );
        if err != 0 {
            printf(c"\ncan't create thread :[%s]".as_ptr(), strerror(err));
            tmon_cleanup();
            exit(EXIT_FAILURE);
        }

        /* validate range of user selected target zone, default to the first
         * instance if out of range
         */
        target_tz_index = zone_instance_to_index(target_thermal_zone);
        if target_tz_index < 0 {
            target_thermal_zone = (*ptdata.tzi.add(0)).instance;
            syslog(
                LOG_ERR,
                c"target zone is not found, default to %d\n".as_ptr(),
                target_thermal_zone,
            );
        }
        loop {
            sleep(ticktime as c_uint);
            show_title_bar();
            show_sensors_w();
            update_thermal_data();
            if dialogue_on == 0 {
                show_data_w();
                show_cooling_device();
            }
            time_elapsed += ticktime as c_double;
            controller_handler(
                *(*trec.add(0)).temp.add(target_tz_index as usize) / 1000,
                &mut yk,
            );
            (*trec.add(0)).pid_out_pct = yk;
            if dialogue_on == 0 {
                show_control_w();
            }
            if tmon_exit != 0 {
                break;
            }
        }
        tmon_cleanup();
        0
    }
}

unsafe extern "C" fn start_daemon_mode() {
    unsafe {
        daemon_mode = 1;
        /* fork */
        let mut sid: pid_t;
        let pid: pid_t = fork();

        if pid < 0 {
            exit(EXIT_FAILURE);
        } else if pid > 0 {
            /* kill parent */
            exit(EXIT_SUCCESS);
        }

        /* disable TUI, it may not be necessary, but saves some resource */
        disable_tui();

        /* change the file mode mask */
        umask(S_IWGRP | S_IWOTH);

        /* new SID for the daemon process */
        sid = setsid();
        if sid < 0 {
            exit(EXIT_FAILURE);
        }

        /* change working directory */
        if chdir(c"/".as_ptr()) < 0 {
            exit(EXIT_FAILURE);
        }

        sleep(10);

        close(STDIN_FILENO);
        close(STDOUT_FILENO);
        close(STDERR_FILENO);
    }
}
