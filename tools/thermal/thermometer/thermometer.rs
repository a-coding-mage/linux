// SPDX-License-Identifier: GPL-2.0-only
// Copyright (C) 2022, Linaro Ltd - Daniel Lezcano <daniel.lezcano@linaro.org>
// C dependencies: dirent.h, fcntl.h, getopt.h, regex.h, signal.h, stdio.h,
// stdlib.h, string.h, sys/stat.h, sys/signalfd.h, sys/timerfd.h, sys/types.h,
// sys/wait.h, time.h, unistd.h, linux/thermal.h, libconfig.h,
// thermal-tools.h.

use core::ffi::{c_char, c_int, c_long, c_void};
use core::mem;
use core::ptr;

const CLASS_THERMAL: &[u8] = b"/sys/class/thermal\0";

const THERMOMETER_SUCCESS: c_int = 0;
const THERMOMETER_OPTION_ERROR: c_int = 1;
const THERMOMETER_LOG_ERROR: c_int = 2;
const THERMOMETER_CONFIG_ERROR: c_int = 3;
const THERMOMETER_TIME_ERROR: c_int = 4;
const THERMOMETER_INIT_ERROR: c_int = 5;
const THERMOMETER_RUNTIME_ERROR: c_int = 6;

const PATH_MAX: usize = 4096;
const THERMAL_NAME_LENGTH: usize = 20;

const F_OK: c_int = 0;
const O_RDONLY: c_int = 0;
const REG_NOSUB: c_int = 8;
const REG_EXTENDED: c_int = 1;
const CLOCK_MONOTONIC: c_int = 1;
const SIGTERM: c_int = 15;
const SIGINT: c_int = 2;
const SIGQUIT: c_int = 3;
const SIGCHLD: c_int = 17;
const SIG_BLOCK: c_int = 0;
const LOG_DEBUG: c_int = 7;
const TO_STDOUT: c_int = 0;
const TO_SYSLOG: c_int = 1;
const no_argument: c_int = 0;
const required_argument: c_int = 1;

#[repr(C)]
struct options {
    loglvl: c_int,
    logopt: c_int,
    overwrite: c_int,
    duration: c_int,
    config: *const c_char,
    postfix: [c_char; PATH_MAX],
    output: [c_char; PATH_MAX],
}

#[repr(C)]
struct tz_regex {
    regex: regex_t,
    polling: c_int,
}

#[repr(C)]
struct configuration {
    tz_regex: *mut tz_regex,
    nr_tz_regex: c_int,
}

#[repr(C)]
struct tz {
    file_out: *mut FILE,
    fd_temp: c_int,
    fd_timer: c_int,
    polling: c_int,
    name: *const c_char,
}

#[repr(C)]
struct thermometer {
    tz: *mut tz,
    nr_tz: c_int,
}

#[repr(C)]
struct regex_t {
    _private: [usize; 8],
}

#[repr(C)]
struct FILE {
    _private: [u8; 0],
}

#[repr(C)]
struct DIR {
    _private: [u8; 0],
}

#[repr(C)]
struct dirent {
    d_ino: usize,
    d_off: isize,
    d_reclen: u16,
    d_type: u8,
    d_name: [c_char; 256],
}

#[repr(C)]
struct option {
    name: *const c_char,
    has_arg: c_int,
    flag: *mut c_int,
    val: c_int,
}

#[repr(C)]
struct timespec {
    tv_sec: c_long,
    tv_nsec: c_long,
}

#[repr(C)]
struct itimerspec {
    it_interval: timespec,
    it_value: timespec,
}

#[repr(C)]
struct sigset_t {
    _private: [usize; 16],
}

#[repr(C)]
struct config_t {
    _private: [u8; 0],
}

#[repr(C)]
struct config_setting_t {
    _private: [u8; 0],
}

type pid_t = c_int;
type size_t = usize;
type ssize_t = isize;
type mainloop_callback_t = Option<unsafe extern "C" fn(c_int, *mut c_void) -> c_int>;

unsafe extern "C" {
    static mut optarg: *mut c_char;
    static mut optind: c_int;
    static VERSION: *const c_char;

    fn access(pathname: *const c_char, mode: c_int) -> c_int;
    fn asprintf(strp: *mut *mut c_char, fmt: *const c_char, ...) -> c_int;
    fn atoi(nptr: *const c_char) -> c_int;
    fn basename(path: *mut c_char) -> *mut c_char;
    fn close(fd: c_int) -> c_int;
    fn closedir(dirp: *mut DIR) -> c_int;
    fn config_error_file(config: *mut config_t) -> *const c_char;
    fn config_error_line(config: *mut config_t) -> c_int;
    fn config_error_text(config: *mut config_t) -> *const c_char;
    fn config_init(config: *mut config_t);
    fn config_lookup(config: *mut config_t, path: *const c_char) -> *mut config_setting_t;
    fn config_read_file(config: *mut config_t, filename: *const c_char) -> c_int;
    fn config_setting_get_elem(setting: *mut config_setting_t, idx: c_int) -> *mut config_setting_t;
    fn config_setting_length(setting: *mut config_setting_t) -> c_int;
    fn config_setting_lookup_int(setting: *mut config_setting_t, name: *const c_char, value: *mut c_int) -> c_int;
    fn config_setting_lookup_string(setting: *mut config_setting_t, name: *const c_char, value: *mut *const c_char) -> c_int;
    fn execvpe(file: *const c_char, argv: *mut *mut c_char, envp: *mut *mut c_char) -> c_int;
    fn exit(status: c_int) -> !;
    fn fclose(stream: *mut FILE) -> c_int;
    fn fopen(pathname: *const c_char, mode: *const c_char) -> *mut FILE;
    fn fork() -> pid_t;
    fn fprintf(stream: *mut FILE, format: *const c_char, ...) -> c_int;
    fn fscanf(stream: *mut FILE, format: *const c_char, ...) -> c_int;
    fn getopt_long(argc: c_int, argv: *mut *mut c_char, shortopts: *const c_char, longopts: *const option, longindex: *mut c_int) -> c_int;
    fn gmtime(timep: *const c_long) -> *mut c_void;
    fn kill(pid: pid_t, sig: c_int) -> c_int;
    fn log_init(level: c_int, name: *const c_char, options: c_int) -> c_int;
    fn log_str2level(level: *const c_char) -> c_int;
    fn mainloop(timeout: c_int) -> c_int;
    fn mainloop_add(fd: c_int, cb: mainloop_callback_t, data: *mut c_void) -> c_int;
    fn mainloop_exit();
    fn mainloop_init() -> c_int;
    fn mkdir(pathname: *const c_char, mode: c_int) -> c_int;
    fn msec_to_timespec(msec: c_int) -> timespec;
    fn open(pathname: *const c_char, flags: c_int, ...) -> c_int;
    fn opendir(name: *const c_char) -> *mut DIR;
    fn pread(fd: c_int, buf: *mut c_void, count: size_t, offset: isize) -> ssize_t;
    fn printf(format: *const c_char, ...) -> c_int;
    fn read(fd: c_int, buf: *mut c_void, count: size_t) -> ssize_t;
    fn readdir(dirp: *mut DIR) -> *mut dirent;
    fn realloc(ptr: *mut c_void, size: size_t) -> *mut c_void;
    fn regcomp(preg: *mut regex_t, regex: *const c_char, cflags: c_int) -> c_int;
    fn regexec(preg: *const regex_t, string: *const c_char, nmatch: size_t, pmatch: *mut c_void, eflags: c_int) -> c_int;
    fn sigaddset(set: *mut sigset_t, signum: c_int) -> c_int;
    fn sigemptyset(set: *mut sigset_t) -> c_int;
    fn signalfd(fd: c_int, mask: *const sigset_t, flags: c_int) -> c_int;
    fn sigprocmask(how: c_int, set: *const sigset_t, oldset: *mut sigset_t) -> c_int;
    fn sprintf(str_: *mut c_char, format: *const c_char, ...) -> c_int;
    fn strcpy(dest: *mut c_char, src: *const c_char) -> *mut c_char;
    fn strdup(s: *const c_char) -> *mut c_char;
    fn strftime(s: *mut c_char, max: size_t, format: *const c_char, tm: *const c_void) -> size_t;
    fn strlen(s: *const c_char) -> size_t;
    fn strncmp(s1: *const c_char, s2: *const c_char, n: size_t) -> c_int;
    fn time(tloc: *mut c_long) -> c_long;
    fn timerfd_create(clockid: c_int, flags: c_int) -> c_int;
    fn timerfd_settime(fd: c_int, flags: c_int, new_value: *const itimerspec, old_value: *mut itimerspec) -> c_int;
    fn uptimeofday_init() -> c_int;
    fn getuptimeofday_ms() -> c_long;
    fn waitpid(pid: pid_t, status: *mut c_int, options: c_int) -> pid_t;
}

macro_rules! ERROR {
    ($fmt:expr $(, $arg:expr)* $(,)?) => {
        printf($fmt.as_ptr() as *const c_char $(, $arg)*)
    };
}

macro_rules! INFO {
    ($fmt:expr $(, $arg:expr)* $(,)?) => {
        printf($fmt.as_ptr() as *const c_char $(, $arg)*)
    };
}

macro_rules! DEBUG {
    ($fmt:expr $(, $arg:expr)* $(,)?) => {
        printf($fmt.as_ptr() as *const c_char $(, $arg)*)
    };
}

unsafe fn configuration_tz_match(expr: *const c_char, config: *mut configuration) -> *mut tz_regex {
    let mut i: c_int = 0;

    while i < (*config).nr_tz_regex {
        if regexec(&(*(*config).tz_regex.add(i as usize)).regex, expr, 0, ptr::null_mut(), 0) == 0 {
            return (*config).tz_regex.add(i as usize);
        }

        i += 1;
    }

    ptr::null_mut()
}

unsafe fn configuration_default_init(config: *mut configuration) -> c_int {
    (*config).tz_regex = realloc(
        (*config).tz_regex as *mut c_void,
        mem::size_of::<tz_regex>() * ((*config).nr_tz_regex as usize + 1),
    ) as *mut tz_regex;

    if regcomp(
        &mut (*(*config).tz_regex.add((*config).nr_tz_regex as usize)).regex,
        b".*\0".as_ptr() as *const c_char,
        REG_NOSUB | REG_EXTENDED,
    ) != 0
    {
        ERROR!(b"Invalid regular expression\n\0");
        return -1;
    }

    (*(*config).tz_regex.add((*config).nr_tz_regex as usize)).polling = 250;
    (*config).nr_tz_regex = 1;

    0
}

unsafe fn configuration_init(path: *const c_char, config: *mut configuration) -> c_int {
    let mut cfg: config_t = mem::zeroed();
    let mut tz_setting: *mut config_setting_t;
    let mut i: c_int;
    let length: c_int;

    if !path.is_null() && access(path, F_OK) != 0 {
        ERROR!(b"'%s' is not accessible\n\0", path);
        return -1;
    }

    if path.is_null() && (*config).nr_tz_regex == 0 {
        INFO!(b"No thermal zones configured, using wildcard for all of them\n\0");
        return configuration_default_init(config);
    }

    config_init(&mut cfg);

    if config_read_file(&mut cfg, path) == 0 {
        ERROR!(
            b"Failed to parse %s:%d - %s\n\0",
            config_error_file(&mut cfg),
            config_error_line(&mut cfg),
            config_error_text(&mut cfg),
        );

        return -1;
    }

    tz_setting = config_lookup(&mut cfg, b"thermal-zones\0".as_ptr() as *const c_char);
    if tz_setting.is_null() {
        ERROR!(b"No thermal zone configured to be monitored\n\0");
        return -1;
    }

    length = config_setting_length(tz_setting);

    INFO!(b"Found %d thermal zone(s) regular expression\n\0", length);

    i = 0;
    while i < length {
        let mut node: *mut config_setting_t;
        let mut name: *const c_char = ptr::null();
        let mut polling: c_int = 0;

        node = config_setting_get_elem(tz_setting, i);
        if node.is_null() {
            ERROR!(b"Missing node name '%d'\n\0", i);
            return -1;
        }

        if config_setting_lookup_string(node, b"name\0".as_ptr() as *const c_char, &mut name) == 0 {
            ERROR!(b"Thermal zone name not found\n\0");
            return -1;
        }

        if config_setting_lookup_int(node, b"polling\0".as_ptr() as *const c_char, &mut polling) == 0 {
            ERROR!(b"Polling value not found\0");
            return -1;
        }

        (*config).tz_regex = realloc(
            (*config).tz_regex as *mut c_void,
            mem::size_of::<tz_regex>() * ((*config).nr_tz_regex as usize + 1),
        ) as *mut tz_regex;

        if regcomp(
            &mut (*(*config).tz_regex.add((*config).nr_tz_regex as usize)).regex,
            name,
            REG_NOSUB | REG_EXTENDED,
        ) != 0
        {
            ERROR!(b"Invalid regular expression '%s'\n\0", name);
            i += 1;
            continue;
        }

        (*(*config).tz_regex.add((*config).nr_tz_regex as usize)).polling = polling;
        (*config).nr_tz_regex += 1;

        INFO!(
            b"Thermal zone regular expression '%s' with polling %d\n\0",
            name,
            polling,
        );

        i += 1;
    }

    0
}

unsafe fn usage(cmd: *const c_char) -> ! {
    printf(b"%s Version: %s\n\0".as_ptr() as *const c_char, cmd, VERSION);
    printf(b"Usage: %s [options]\n\0".as_ptr() as *const c_char, cmd);
    printf(b"\t-h, --help\t\tthis help\n\0".as_ptr() as *const c_char);
    printf(b"\t-o, --output <dir>\toutput directory for temperature capture\n\0".as_ptr() as *const c_char);
    printf(b"\t-c, --config <file>\tconfiguration file\n\0".as_ptr() as *const c_char);
    printf(b"\t-d, --duration <seconds>\tcapture duration\n\0".as_ptr() as *const c_char);
    printf(b"\t-l, --loglevel <level>\tlog level: \0".as_ptr() as *const c_char);
    printf(b"DEBUG, INFO, NOTICE, WARN, ERROR\n\0".as_ptr() as *const c_char);
    printf(b"\t-p, --postfix <string>\tpostfix to be happened at the end of the files\n\0".as_ptr() as *const c_char);
    printf(b"\t-s, --syslog\t\toutput to syslog\n\0".as_ptr() as *const c_char);
    printf(b"\t-w, --overwrite\t\toverwrite the temperature capture files if they exist\n\0".as_ptr() as *const c_char);
    printf(b"\n\0".as_ptr() as *const c_char);
    exit(0);
}

unsafe fn options_init(argc: c_int, argv: *mut *mut c_char, options: *mut options) -> c_int {
    let mut opt: c_int;
    let now: c_long = time(ptr::null_mut());

    let long_options = [
        option { name: b"help\0".as_ptr() as *const c_char, has_arg: no_argument, flag: ptr::null_mut(), val: 'h' as c_int },
        option { name: b"config\0".as_ptr() as *const c_char, has_arg: required_argument, flag: ptr::null_mut(), val: 'c' as c_int },
        option { name: b"duration\0".as_ptr() as *const c_char, has_arg: required_argument, flag: ptr::null_mut(), val: 'd' as c_int },
        option { name: b"loglevel\0".as_ptr() as *const c_char, has_arg: required_argument, flag: ptr::null_mut(), val: 'l' as c_int },
        option { name: b"postfix\0".as_ptr() as *const c_char, has_arg: required_argument, flag: ptr::null_mut(), val: 'p' as c_int },
        option { name: b"output\0".as_ptr() as *const c_char, has_arg: required_argument, flag: ptr::null_mut(), val: 'o' as c_int },
        option { name: b"syslog\0".as_ptr() as *const c_char, has_arg: required_argument, flag: ptr::null_mut(), val: 's' as c_int },
        option { name: b"overwrite\0".as_ptr() as *const c_char, has_arg: no_argument, flag: ptr::null_mut(), val: 'w' as c_int },
        option { name: ptr::null(), has_arg: 0, flag: ptr::null_mut(), val: 0 },
    ];

    strftime(
        (*options).postfix.as_mut_ptr(),
        (*options).postfix.len(),
        b"-%Y-%m-%d_%H:%M:%S\0".as_ptr() as *const c_char,
        gmtime(&now),
    );

    loop {
        let mut optindex: c_int = 0;

        opt = getopt_long(
            argc,
            argv,
            b"ho:c:d:l:p:sw\0".as_ptr() as *const c_char,
            long_options.as_ptr(),
            &mut optindex,
        );
        if opt == -1 {
            break;
        }

        match opt {
            x if x == 'c' as c_int => (*options).config = optarg,
            x if x == 'd' as c_int => (*options).duration = atoi(optarg) * 1000,
            x if x == 'l' as c_int => (*options).loglvl = log_str2level(optarg),
            x if x == 'h' as c_int => usage(basename(*argv.add(0))),
            x if x == 'p' as c_int => {
                strcpy((*options).postfix.as_mut_ptr(), optarg);
            }
            x if x == 'o' as c_int => {
                strcpy((*options).output.as_mut_ptr(), optarg);
            }
            x if x == 's' as c_int => (*options).logopt = TO_SYSLOG,
            x if x == 'w' as c_int => (*options).overwrite = 1,
            _ => {
                ERROR!(b"Usage: %s --help\n\0", *argv.add(0));
                return -1;
            }
        }
    }

    0
}

unsafe fn thermometer_add_tz(path: *const c_char, name: *const c_char, polling: c_int, thermometer: *mut thermometer) -> c_int {
    let mut fd: c_int;
    let mut tz_path = [0 as c_char; PATH_MAX];
    let mut tz_ptr: *mut tz;

    sprintf(tz_path.as_mut_ptr(), b"%s/%s/temp\0".as_ptr() as *const c_char, CLASS_THERMAL.as_ptr(), path);

    fd = open(tz_path.as_ptr(), O_RDONLY);
    if fd < 0 {
        ERROR!(b"Failed to open '%s': %m\n\0", tz_path.as_ptr());
        return -1;
    }

    tz_ptr = realloc((*thermometer).tz as *mut c_void, mem::size_of::<tz>() * ((*thermometer).nr_tz as usize + 1)) as *mut tz;
    if tz_ptr.is_null() {
        ERROR!(b"Failed to allocate thermometer->tz\n\0");
        close(fd);
        return -1;
    }

    (*thermometer).tz = tz_ptr;
    (*(*thermometer).tz.add((*thermometer).nr_tz as usize)).fd_temp = fd;
    (*(*thermometer).tz.add((*thermometer).nr_tz as usize)).name = strdup(name);
    (*(*thermometer).tz.add((*thermometer).nr_tz as usize)).polling = polling;
    (*thermometer).nr_tz += 1;

    INFO!(b"Added thermal zone '%s->%s (polling:%d)'\n\0", path, name, polling);

    0
}

unsafe fn thermometer_init(config: *mut configuration, thermometer: *mut thermometer) -> c_int {
    let mut dir: *mut DIR;
    let mut dirent_ptr: *mut dirent;
    let mut tz_regex_ptr: *mut tz_regex;
    let tz_dirname = b"thermal_zone\0";

    if mainloop_init() != 0 {
        ERROR!(b"Failed to start mainloop\n\0");
        return -1;
    }

    dir = opendir(CLASS_THERMAL.as_ptr() as *const c_char);
    if dir.is_null() {
        ERROR!(b"failed to open '%s'\n\0", CLASS_THERMAL.as_ptr());
        return -1;
    }

    loop {
        dirent_ptr = readdir(dir);
        if dirent_ptr.is_null() {
            break;
        }
        let mut tz_type = [0 as c_char; THERMAL_NAME_LENGTH];
        let mut tz_path = [0 as c_char; PATH_MAX];
        let mut tz_file: *mut FILE;

        if strncmp((*dirent_ptr).d_name.as_ptr(), tz_dirname.as_ptr() as *const c_char, strlen(tz_dirname.as_ptr() as *const c_char)) != 0 {
            continue;
        }

        sprintf(tz_path.as_mut_ptr(), b"%s/%s/type\0".as_ptr() as *const c_char, CLASS_THERMAL.as_ptr(), (*dirent_ptr).d_name.as_ptr());

        tz_file = fopen(tz_path.as_ptr(), b"r\0".as_ptr() as *const c_char);
        if tz_file.is_null() {
            ERROR!(b"Failed to open '%s': %m\0", tz_path.as_ptr());
            continue;
        }

        fscanf(tz_file, b"%s\0".as_ptr() as *const c_char, tz_type.as_mut_ptr());

        fclose(tz_file);

        tz_regex_ptr = configuration_tz_match(tz_type.as_ptr(), config);
        if tz_regex_ptr.is_null() {
            continue;
        }

        if thermometer_add_tz((*dirent_ptr).d_name.as_ptr(), tz_type.as_ptr(), (*tz_regex_ptr).polling, thermometer) != 0 {
            continue;
        }
    }

    closedir(dir);

    0
}

unsafe extern "C" fn timer_temperature_callback(fd: c_int, arg: *mut c_void) -> c_int {
    let tz_ptr = arg as *mut tz;
    let mut buf = [0 as c_char; 16];

    pread((*tz_ptr).fd_temp, buf.as_mut_ptr() as *mut c_void, buf.len(), 0);

    fprintf((*tz_ptr).file_out, b"%ld %s\0".as_ptr() as *const c_char, getuptimeofday_ms(), buf.as_ptr());

    read(fd, buf.as_mut_ptr() as *mut c_void, buf.len());

    0
}

unsafe fn thermometer_start(thermometer: *mut thermometer, options: *mut options) -> c_int {
    let mut timer_it: itimerspec = mem::zeroed();
    let mut path: *mut c_char = ptr::null_mut();
    let mut f: *mut FILE;
    let mut i: c_int;

    INFO!(b"Capturing %d thermal zone(s) temperature...\n\0", (*thermometer).nr_tz);

    if access((*options).output.as_ptr(), F_OK) != 0 && mkdir((*options).output.as_ptr(), 0o700) != 0 {
        ERROR!(b"Failed to create directory '%s'\n\0", (*options).output.as_ptr());
        return -1;
    }

    i = 0;
    while i < (*thermometer).nr_tz {
        asprintf(&mut path, b"%s/%s%s\0".as_ptr() as *const c_char, (*options).output.as_ptr(), (*(*thermometer).tz.add(i as usize)).name, (*options).postfix.as_ptr());

        if (*options).overwrite == 0 && access(path, F_OK) == 0 {
            ERROR!(b"'%s' already exists\n\0", path);
            return -1;
        }

        f = fopen(path, b"w\0".as_ptr() as *const c_char);
        if f.is_null() {
            ERROR!(b"Failed to create '%s':%m\n\0", path);
            return -1;
        }

        fprintf(f, b"timestamp(ms) %s(\xC2\xB0mC)\n\0".as_ptr() as *const c_char, (*(*thermometer).tz.add(i as usize)).name);

        (*(*thermometer).tz.add(i as usize)).file_out = f;

        DEBUG!(b"Created '%s' file for thermal zone '%s'\n\0", path, (*(*thermometer).tz.add(i as usize)).name);

        /*
         * Create polling timer
         */
        (*(*thermometer).tz.add(i as usize)).fd_timer = timerfd_create(CLOCK_MONOTONIC, 0);
        if (*(*thermometer).tz.add(i as usize)).fd_timer < 0 {
            ERROR!(b"Failed to create timer for '%s': %m\n\0", (*(*thermometer).tz.add(i as usize)).name);
            return -1;
        }

        DEBUG!(b"Watching '%s' every %d ms\n\0", (*(*thermometer).tz.add(i as usize)).name, (*(*thermometer).tz.add(i as usize)).polling);

        timer_it.it_value = msec_to_timespec((*(*thermometer).tz.add(i as usize)).polling);
        timer_it.it_interval = timer_it.it_value;

        if timerfd_settime((*(*thermometer).tz.add(i as usize)).fd_timer, 0, &timer_it, ptr::null_mut()) < 0 {
            return -1;
        }

        if mainloop_add((*(*thermometer).tz.add(i as usize)).fd_timer, Some(timer_temperature_callback), &mut *(*thermometer).tz.add(i as usize) as *mut tz as *mut c_void) != 0 {
            return -1;
        }

        i += 1;
    }

    0
}

unsafe fn thermometer_execute(argc: c_int, argv: *mut *mut c_char, envp: *mut *mut c_char, pid: *mut pid_t) -> c_int {
    if argc == 0 {
        return 0;
    }

    *pid = fork();
    if *pid < 0 {
        ERROR!(b"Failed to fork process: %m\0");
        return -1;
    }

    if *pid == 0 {
        execvpe(*argv.add(0), argv, envp);
        exit(1);
    }

    0
}

unsafe extern "C" fn kill_process(_fd: c_int, arg: *mut c_void) -> c_int {
    let pid: pid_t = *(arg as *mut pid_t);

    if kill(pid, SIGTERM) != 0 {
        ERROR!(b"Failed to send SIGTERM signal to '%d': %p\n\0", pid);
    } else if waitpid(pid, ptr::null_mut(), 0) != 0 {
        ERROR!(b"Failed to wait pid '%d': %p\n\0", pid);
    }

    mainloop_exit();

    0
}

unsafe extern "C" fn exit_mainloop(_fd: c_int, _arg: *mut c_void) -> c_int {
    mainloop_exit();
    0
}

unsafe fn thermometer_wait(options: *mut options, pid: pid_t) -> c_int {
    let mut fd: c_int;
    let mut mask: sigset_t = mem::zeroed();

    /*
     * If there is a duration specified, we will exit the mainloop
     * and gracefully close all the files which will flush the
     * file system cache
     */
    if (*options).duration != 0 {
        let mut timer_it: itimerspec = mem::zeroed();

        timer_it.it_value = msec_to_timespec((*options).duration);

        fd = timerfd_create(CLOCK_MONOTONIC, 0);
        if fd < 0 {
            ERROR!(b"Failed to create duration timer: %m\n\0");
            return -1;
        }

        if timerfd_settime(fd, 0, &timer_it, ptr::null_mut()) != 0 {
            ERROR!(b"Failed to set timer time: %m\n\0");
            return -1;
        }

        if mainloop_add(fd, if pid < 0 { Some(exit_mainloop) } else { Some(kill_process) }, &pid as *const pid_t as *mut c_void) != 0 {
            ERROR!(b"Failed to set timer exit mainloop callback\n\0");
            return -1;
        }
    }

    /*
     * We want to catch any keyboard interrupt, as well as child
     * signals if any in order to exit properly
     */
    sigemptyset(&mut mask);
    sigaddset(&mut mask, SIGINT);
    sigaddset(&mut mask, SIGQUIT);
    sigaddset(&mut mask, SIGCHLD);

    if sigprocmask(SIG_BLOCK, &mask, ptr::null_mut()) != 0 {
        ERROR!(b"Failed to set sigprocmask: %m\n\0");
        return -1;
    }

    fd = signalfd(-1, &mask, 0);
    if fd < 0 {
        ERROR!(b"Failed to set the signalfd: %m\n\0");
        return -1;
    }

    if mainloop_add(fd, Some(exit_mainloop), ptr::null_mut()) != 0 {
        ERROR!(b"Failed to set timer exit mainloop callback\n\0");
        return -1;
    }

    mainloop(-1)
}

unsafe fn thermometer_stop(thermometer: *mut thermometer) -> c_int {
    let mut i: c_int;

    INFO!(b"Closing/flushing output files\n\0");

    i = 0;
    while i < (*thermometer).nr_tz {
        fclose((*(*thermometer).tz.add(i as usize)).file_out);
        i += 1;
    }

    0
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn main(argc: c_int, argv: *mut *mut c_char, envp: *mut *mut c_char) -> c_int {
    let mut options = options {
        loglvl: LOG_DEBUG,
        logopt: TO_STDOUT,
        overwrite: 0,
        duration: 0,
        config: ptr::null(),
        postfix: [0; PATH_MAX],
        output: [0; PATH_MAX],
    };
    strcpy(options.output.as_mut_ptr(), b".\0".as_ptr() as *const c_char);

    let mut config: configuration = mem::zeroed();
    let mut thermometer: thermometer = mem::zeroed();

    let mut pid: pid_t = -1;

    if options_init(argc, argv, &mut options) != 0 {
        return THERMOMETER_OPTION_ERROR;
    }

    if log_init(options.loglvl, *argv.add(0), options.logopt) != 0 {
        return THERMOMETER_LOG_ERROR;
    }

    if configuration_init(options.config, &mut config) != 0 {
        return THERMOMETER_CONFIG_ERROR;
    }

    if uptimeofday_init() != 0 {
        return THERMOMETER_TIME_ERROR;
    }

    if thermometer_init(&mut config, &mut thermometer) != 0 {
        return THERMOMETER_INIT_ERROR;
    }

    if thermometer_start(&mut thermometer, &mut options) != 0 {
        return THERMOMETER_RUNTIME_ERROR;
    }

    if thermometer_execute(argc - optind, argv.add(optind as usize), envp, &mut pid) != 0 {
        return THERMOMETER_RUNTIME_ERROR;
    }

    if thermometer_wait(&mut options, pid) != 0 {
        return THERMOMETER_RUNTIME_ERROR;
    }

    if thermometer_stop(&mut thermometer) != 0 {
        return THERMOMETER_RUNTIME_ERROR;
    }

    THERMOMETER_SUCCESS
}
