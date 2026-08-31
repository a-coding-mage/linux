// SPDX-License-Identifier: GPL-2.0
/* For general debugging purposes */

#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(dead_code)]
#![feature(c_variadic)]

use core::ffi::{c_char, c_int, c_long, c_uint, c_ulong, c_void};
use std::ffi::VaListImpl;
use std::ptr;

type bool_ = bool;
type u64 = u64;
type size_t = usize;
type pid_t = c_int;
type uintptr_t = usize;

const NSEC_PER_SEC: u64 = 1_000_000_000;
const NSEC_PER_USEC: u64 = 1_000;
const PERF_RECORD_MISC_USER: c_uint = 1 << 13;
const ARRAY_SIZE_STACKDUMP: size_t = 32;
const SIG_DFL: sighandler_t = None;

/* LIBTRACEEVENT_VERSION is 0 when HAVE_LIBTRACEEVENT is not configured. */
const LIBTRACEEVENT_VERSION: c_int = 0;

#[repr(C)]
pub struct FILE {
    _private: [u8; 0],
}

#[repr(C)]
pub struct timeval {
    pub tv_sec: c_long,
    pub tv_usec: c_long,
}

#[repr(C)]
pub struct tm {
    _private: [u8; 0],
}

#[repr(C)]
pub struct perf_event_header {
    pub type_: u32,
    pub misc: u16,
    pub size: u16,
}

#[repr(C)]
pub union perf_event {
    pub header: perf_event_header,
}

#[repr(C)]
pub struct sublevel_option {
    pub name: *const c_char,
    pub value_ptr: *mut c_int,
}

#[repr(C)]
pub struct perf_env {
    _private: [u8; 0],
}

#[repr(C)]
pub struct machine {
    _private: [u8; 0],
}

#[repr(C)]
pub struct thread {
    _private: [u8; 0],
}

#[repr(C)]
pub struct map {
    _private: [u8; 0],
}

#[repr(C)]
pub struct symbol {
    pub name: *const c_char,
}

#[repr(C)]
pub struct addr_location {
    pub thread: *mut thread,
    pub maps: *mut c_void,
    pub map: *mut map,
    pub sym: *mut symbol,
    pub addr: u64,
    pub comm: *const c_char,
    pub filtered: c_int,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub enum binary_printer_ops {
    BINARY_PRINT_DATA_BEGIN = 0,
    BINARY_PRINT_LINE_BEGIN = 1,
    BINARY_PRINT_ADDR = 2,
    BINARY_PRINT_NUM_DATA = 3,
    BINARY_PRINT_NUM_PAD = 4,
    BINARY_PRINT_SEP = 5,
    BINARY_PRINT_CHAR_DATA = 6,
    BINARY_PRINT_CHAR_PAD = 7,
    BINARY_PRINT_LINE_END = 8,
    BINARY_PRINT_DATA_END = 9,
}

type binary_printer = Option<
    unsafe extern "C" fn(binary_printer_ops, c_uint, *mut c_void, *mut FILE) -> c_int,
>;
type print_fn = Option<unsafe extern "C" fn(*const c_char, ...) -> c_int>;
type sighandler_t = Option<unsafe extern "C" fn(c_int)>;

unsafe extern "C" {
    static mut stderr: *mut FILE;
    static mut stdout: *mut FILE;
    static mut use_browser: c_int;

    fn fprintf(stream: *mut FILE, format: *const c_char, ...) -> c_int;
    fn vfprintf(stream: *mut FILE, format: *const c_char, arg: VaListImpl<'_>) -> c_int;
    fn vprintf(format: *const c_char, arg: VaListImpl<'_>) -> c_int;
    fn gettimeofday(tv: *mut timeval, tz: *mut c_void) -> c_int;
    fn localtime_r(timep: *const c_long, result: *mut tm) -> *mut tm;
    fn strftime(s: *mut c_char, max: size_t, format: *const c_char, tm: *const tm) -> size_t;
    fn isprint(c: c_int) -> c_int;
    fn isascii(c: c_int) -> c_int;
    fn getpid() -> pid_t;
    fn fileno(stream: *mut FILE) -> c_int;
    fn psignal(sig: c_int, s: *const c_char);
    fn signal(sig: c_int, handler: sighandler_t) -> sighandler_t;
    fn raise(sig: c_int) -> c_int;

    fn ui_helpline__vshow(fmt: *const c_char, args: VaListImpl<'_>);
    fn color_fprintf(fp: *mut FILE, color: *const c_char, fmt: *const c_char, ...) -> c_int;
    fn print_binary(
        data: *mut u8,
        len: size_t,
        bytes_per_line: size_t,
        printer: binary_printer,
        extra: *mut c_void,
    );
    fn perf_parse_sublevel_options(
        str_: *const c_char,
        opts: *mut sublevel_option,
    ) -> c_int;
    fn libapi_set_print(warning: print_fn, info: print_fn, debug: print_fn);
    fn perf_env__init(env: *mut perf_env);
    fn perf_env__exit(env: *mut perf_env);
    fn machine__new_live(env: *mut perf_env, kernel_maps: bool_, pid: pid_t) -> *mut machine;
    fn machine__find_thread(machine: *mut machine, pid: pid_t, tid: pid_t) -> *mut thread;
    fn machine__delete(machine: *mut machine);
    fn thread__find_map(
        thread: *mut thread,
        cpumode: c_uint,
        addr: u64,
        al: *mut addr_location,
    ) -> bool_;
    fn thread__put(thread: *mut thread);
    fn addr_location__init(al: *mut addr_location);
    fn addr_location__exit(al: *mut addr_location);
    fn map__find_symbol(map: *mut map, addr: u64) -> *mut symbol;
    fn map__fprintf_srcline(
        map: *mut map,
        addr: u64,
        prefix: *const c_char,
        fp: *mut FILE,
    ) -> c_int;

    /* Available when HAVE_BACKTRACE_SUPPORT is configured. */
    fn backtrace(buffer: *mut *mut c_void, size: c_int) -> c_int;
    fn backtrace_symbols_fd(buffer: *mut *mut c_void, size: c_int, fd: c_int);
}

const PERF_COLOR_BLUE: *const c_char = b"blue\0".as_ptr() as *const c_char;

#[no_mangle]
pub static mut verbose: c_int = 0;
#[no_mangle]
pub static mut debug_kmaps: c_int = 0;
#[no_mangle]
pub static mut debug_peo_args: c_int = 0;
#[no_mangle]
pub static mut dump_trace: bool_ = false;
#[no_mangle]
pub static mut quiet: bool_ = false;
#[no_mangle]
pub static mut debug_ordered_events: c_int = 0;
static mut redirect_to_stderr: c_int = 0;
#[no_mangle]
pub static mut debug_data_convert: c_int = 0;
static mut _debug_file: *mut FILE = ptr::null_mut();
static mut debug_display_time: bool_ = false;
#[no_mangle]
pub static mut debug_type_profile: c_int = 0;

unsafe fn pr_warning_once_debug_file_not_set() {
    static mut warned: bool_ = false;
    if !warned {
        warned = true;
        pr_warning_wrapper(b"debug_file not set\0".as_ptr() as *const c_char);
    }
}

#[no_mangle]
pub unsafe extern "C" fn debug_file() -> *mut FILE {
    if _debug_file.is_null() {
        debug_set_file(stderr);
        pr_warning_once_debug_file_not_set();
    }
    _debug_file
}

#[no_mangle]
pub unsafe extern "C" fn debug_set_file(file: *mut FILE) {
    _debug_file = file;
}

#[no_mangle]
pub unsafe extern "C" fn debug_set_display_time(set: bool_) {
    debug_display_time = set;
}

unsafe fn fprintf_time(file: *mut FILE) -> c_int {
    let mut tod = timeval { tv_sec: 0, tv_usec: 0 };
    let mut ltime: tm = core::mem::zeroed();
    let mut date = [0 as c_char; 64];

    if !debug_display_time {
        return 0;
    }

    if gettimeofday(&mut tod, ptr::null_mut()) != 0 {
        return 0;
    }

    if localtime_r(&tod.tv_sec, &mut ltime).is_null() {
        return 0;
    }

    strftime(
        date.as_mut_ptr(),
        date.len(),
        b"%F %H:%M:%S\0".as_ptr() as *const c_char,
        &ltime,
    );
    fprintf(
        file,
        b"[%s.%06lu] \0".as_ptr() as *const c_char,
        date.as_ptr(),
        tod.tv_usec as c_ulong,
    )
}

#[no_mangle]
pub unsafe extern "C" fn veprintf(
    level: c_int,
    var: c_int,
    fmt: *const c_char,
    args: VaListImpl<'_>,
) -> c_int {
    let mut ret = 0;

    if var >= level {
        if use_browser >= 1 && redirect_to_stderr == 0 {
            ui_helpline__vshow(fmt, args);
        } else {
            ret = fprintf_time(debug_file());
            ret += vfprintf(debug_file(), fmt, args);
        }
    }

    ret
}

#[no_mangle]
pub unsafe extern "C" fn eprintf(
    level: c_int,
    var: c_int,
    fmt: *const c_char,
    mut args: ...
) -> c_int {
    veprintf(level, var, fmt, args.as_va_list())
}

unsafe fn veprintf_time(t: u64, fmt: *const c_char, args: VaListImpl<'_>) -> c_int {
    let mut nsecs = t;
    let secs = nsecs / NSEC_PER_SEC;
    nsecs -= secs * NSEC_PER_SEC;
    let usecs = nsecs / NSEC_PER_USEC;

    let mut ret = fprintf(
        debug_file(),
        b"[%13llu.%06llu] \0".as_ptr() as *const c_char,
        secs,
        usecs,
    );
    ret += vfprintf(debug_file(), fmt, args);
    ret
}

#[no_mangle]
pub unsafe extern "C" fn eprintf_time(
    level: c_int,
    var: c_int,
    t: u64,
    fmt: *const c_char,
    mut args: ...
) -> c_int {
    let mut ret = 0;

    if var >= level {
        ret = veprintf_time(t, fmt, args.as_va_list());
    }

    ret
}

/*
 * Overloading libtraceevent standard info print
 * function, display with -v in perf.
 */
#[no_mangle]
pub unsafe extern "C" fn pr_stat(fmt: *const c_char, mut args: ...) {
    veprintf(1, verbose, fmt, args.as_va_list());
    eprintf(1, verbose, b"\n\0".as_ptr() as *const c_char);
}

#[no_mangle]
pub unsafe extern "C" fn dump_printf(fmt: *const c_char, mut args: ...) -> c_int {
    let mut ret = 0;

    if dump_trace {
        ret = vprintf(fmt, args.as_va_list());
    }

    ret
}

unsafe extern "C" fn trace_event_printer(
    op: binary_printer_ops,
    val: c_uint,
    extra: *mut c_void,
    fp: *mut FILE,
) -> c_int {
    let color = PERF_COLOR_BLUE;
    let event = extra as *mut perf_event;
    let ch = val as u8;
    let mut printed = 0;

    match op {
        binary_printer_ops::BINARY_PRINT_DATA_BEGIN => {
            printed += fprintf(fp, b".\0".as_ptr() as *const c_char);
            printed += color_fprintf(
                fp,
                color,
                b"\n. ... raw event: size %d bytes\n\0".as_ptr() as *const c_char,
                (*event).header.size as c_int,
            );
        }
        binary_printer_ops::BINARY_PRINT_LINE_BEGIN => {
            printed += fprintf(fp, b".\0".as_ptr() as *const c_char);
        }
        binary_printer_ops::BINARY_PRINT_ADDR => {
            printed += color_fprintf(
                fp,
                color,
                b"  %04x: \0".as_ptr() as *const c_char,
                val,
            );
        }
        binary_printer_ops::BINARY_PRINT_NUM_DATA => {
            printed += color_fprintf(fp, color, b" %02x\0".as_ptr() as *const c_char, val);
        }
        binary_printer_ops::BINARY_PRINT_NUM_PAD => {
            printed += color_fprintf(fp, color, b"   \0".as_ptr() as *const c_char);
        }
        binary_printer_ops::BINARY_PRINT_SEP => {
            printed += color_fprintf(fp, color, b"  \0".as_ptr() as *const c_char);
        }
        binary_printer_ops::BINARY_PRINT_CHAR_DATA => {
            printed += color_fprintf(
                fp,
                color,
                b"%c\0".as_ptr() as *const c_char,
                if isprint(ch as c_int) != 0 && isascii(ch as c_int) != 0 {
                    ch as c_int
                } else {
                    b'.' as c_int
                },
            );
        }
        binary_printer_ops::BINARY_PRINT_CHAR_PAD => {
            printed += color_fprintf(fp, color, b" \0".as_ptr() as *const c_char);
        }
        binary_printer_ops::BINARY_PRINT_LINE_END => {
            printed += color_fprintf(fp, color, b"\n\0".as_ptr() as *const c_char);
        }
        binary_printer_ops::BINARY_PRINT_DATA_END => {
            printed += fprintf(fp, b"\n\0".as_ptr() as *const c_char);
        }
    }

    printed
}

#[no_mangle]
pub unsafe extern "C" fn trace_event(event: *mut perf_event) {
    let raw_event = event as *mut u8;

    if !dump_trace {
        return;
    }

    print_binary(
        raw_event,
        (*event).header.size as size_t,
        16,
        Some(trace_event_printer),
        event as *mut c_void,
    );
}

static mut debug_opts: [sublevel_option; 8] = [
    sublevel_option {
        name: b"verbose\0".as_ptr() as *const c_char,
        value_ptr: ptr::addr_of_mut!(verbose),
    },
    sublevel_option {
        name: b"ordered-events\0".as_ptr() as *const c_char,
        value_ptr: ptr::addr_of_mut!(debug_ordered_events),
    },
    sublevel_option {
        name: b"stderr\0".as_ptr() as *const c_char,
        value_ptr: ptr::addr_of_mut!(redirect_to_stderr),
    },
    sublevel_option {
        name: b"data-convert\0".as_ptr() as *const c_char,
        value_ptr: ptr::addr_of_mut!(debug_data_convert),
    },
    sublevel_option {
        name: b"perf-event-open\0".as_ptr() as *const c_char,
        value_ptr: ptr::addr_of_mut!(debug_peo_args),
    },
    sublevel_option {
        name: b"kmaps\0".as_ptr() as *const c_char,
        value_ptr: ptr::addr_of_mut!(debug_kmaps),
    },
    sublevel_option {
        name: b"type-profile\0".as_ptr() as *const c_char,
        value_ptr: ptr::addr_of_mut!(debug_type_profile),
    },
    sublevel_option {
        name: ptr::null(),
        value_ptr: ptr::null_mut(),
    },
];

#[no_mangle]
pub unsafe extern "C" fn perf_debug_option(str_: *const c_char) -> c_int {
    let ret = perf_parse_sublevel_options(str_, debug_opts.as_mut_ptr());
    if ret != 0 {
        return ret;
    }

    /* Allow only verbose value in range (0, 10), otherwise set 0. */
    verbose = if verbose < 0 || verbose > 10 { 0 } else { verbose };

    /*
     * If LIBTRACEEVENT_VERSION >= MAKE_LIBTRACEEVENT_VERSION(1, 3, 0):
     * verbose selects TEP_LOG_INFO, TEP_LOG_DEBUG, or TEP_LOG_ALL.
     */
    if LIBTRACEEVENT_VERSION != 0 {
        /* tep_set_loglevel mapping is supplied by the libtraceevent build. */
    }

    0
}

#[no_mangle]
pub unsafe extern "C" fn perf_quiet_option() -> c_int {
    let mut opt = debug_opts.as_mut_ptr();

    /* disable all debug messages */
    while !(*opt).name.is_null() {
        *(*opt).value_ptr = -1;
        opt = opt.add(1);
    }

    /* For debug variables that are used as bool types, set to 0. */
    redirect_to_stderr = 0;
    debug_peo_args = 0;
    debug_kmaps = 0;
    debug_type_profile = 0;

    0
}

unsafe extern "C" fn pr_warning_wrapper(fmt: *const c_char, mut args: ...) -> c_int {
    veprintf(0, verbose, fmt, args.as_va_list())
}

unsafe extern "C" fn pr_debug_wrapper(fmt: *const c_char, mut args: ...) -> c_int {
    veprintf(1, verbose, fmt, args.as_va_list())
}

#[no_mangle]
pub unsafe extern "C" fn perf_debug_setup() {
    debug_set_file(stderr);
    libapi_set_print(
        Some(pr_warning_wrapper),
        Some(pr_warning_wrapper),
        Some(pr_debug_wrapper),
    );
}

#[no_mangle]
pub unsafe extern "C" fn __dump_stack(
    file: *mut FILE,
    stackdump: *mut *mut c_void,
    stackdump_size: size_t,
) {
    /* TODO: async safety. printf, malloc, etc. aren't safe inside a signal handler. */
    let pid = getpid();
    let mut thread: *mut thread = ptr::null_mut();
    let mut host_env: perf_env = core::mem::zeroed();

    perf_env__init(&mut host_env);
    let machine = machine__new_live(&mut host_env, false, pid);

    if !machine.is_null() {
        thread = machine__find_thread(machine, pid, pid);
    }

    /*
     * When HAVE_BACKTRACE_SUPPORT is configured, backtrace_symbols_fd is the
     * fallback if machine/thread creation fails.
     */
    if machine.is_null() || thread.is_null() {
        backtrace_symbols_fd(stackdump, stackdump_size as c_int, fileno(file));
        machine__delete(machine);
        perf_env__exit(&mut host_env);
        return;
    }

    for i in 0..stackdump_size {
        let mut al: addr_location = core::mem::zeroed();
        let addr = *stackdump.add(i) as uintptr_t as u64;
        let mut printed = false;

        addr_location__init(&mut al);
        if !thread.is_null()
            && thread__find_map(thread, PERF_RECORD_MISC_USER, addr, &mut al)
        {
            al.sym = map__find_symbol(al.map, al.addr);
            if !al.sym.is_null() {
                fprintf(
                    file,
                    b"    #%zd %p in %s \0".as_ptr() as *const c_char,
                    i,
                    *stackdump.add(i),
                    (*al.sym).name,
                );
                printed = true;
            }
        }
        if !printed {
            fprintf(
                file,
                b"    #%zd %p \0".as_ptr() as *const c_char,
                i,
                *stackdump.add(i),
            );
        }

        map__fprintf_srcline(al.map, al.addr, b"\0".as_ptr() as *const c_char, file);
        fprintf(file, b"\n\0".as_ptr() as *const c_char);
        addr_location__exit(&mut al);
    }
    thread__put(thread);
    machine__delete(machine);
    perf_env__exit(&mut host_env);
}

/* Obtain a backtrace and print it to stdout. */
#[no_mangle]
pub unsafe extern "C" fn dump_stack() {
    let mut stackdump = [ptr::null_mut::<c_void>(); ARRAY_SIZE_STACKDUMP];
    let size = backtrace(stackdump.as_mut_ptr(), stackdump.len() as c_int) as size_t;

    __dump_stack(stdout, stackdump.as_mut_ptr(), size);
}

#[no_mangle]
pub unsafe extern "C" fn sighandler_dump_stack(sig: c_int) {
    psignal(sig, b"perf\0".as_ptr() as *const c_char);
    dump_stack();
    signal(sig, SIG_DFL);
    raise(sig);
}
