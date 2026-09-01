// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * PTP 1588 clock support - User space test program
 *
 * Copyright (C) 2010 OMICRON electronics GmbH
 */

use std::env;
use std::ffi::{CStr, CString};
use std::mem::{size_of, zeroed};
use std::os::raw::{c_char, c_int, c_long, c_ulong, c_void};
use std::ptr;

const DEVICE: &str = "/dev/ptp0";

const ADJ_OFFSET: c_int = 0x0001;
const ADJ_FREQUENCY: c_int = 0x0002;
const ADJ_NANO: c_int = 0x2000;
const ADJ_SETOFFSET: c_int = 0x0100;

const CLOCK_REALTIME: clockid_t = 0;
const CLOCK_MONOTONIC: clockid_t = 1;
const CLOCK_INVALID: clockid_t = -1;
const CLOCK_MONOTONIC_RAW: clockid_t = 4;

const NSEC_PER_SEC: i64 = 1000000000;
const EOF: c_int = -1;
const O_RDONLY: c_int = 0;
const O_RDWR: c_int = 2;

const PTP_MAX_SAMPLES: usize = 25;
const PTP_ENABLE_FEATURE: u32 = 1 << 0;
const PTP_RISING_EDGE: u32 = 1 << 1;
const PTP_FALLING_EDGE: u32 = 1 << 2;
const PTP_EXTTS_VALID_FLAGS: u32 = PTP_ENABLE_FEATURE | PTP_RISING_EDGE | PTP_FALLING_EDGE;
const PTP_PEROUT_DUTY_CYCLE: u32 = 1 << 1;
const PTP_PEROUT_PHASE: u32 = 1 << 2;

const IOC_NRBITS: c_ulong = 8;
const IOC_TYPEBITS: c_ulong = 8;
const IOC_SIZEBITS: c_ulong = 14;
const IOC_NRSHIFT: c_ulong = 0;
const IOC_TYPESHIFT: c_ulong = IOC_NRSHIFT + IOC_NRBITS;
const IOC_SIZESHIFT: c_ulong = IOC_TYPESHIFT + IOC_TYPEBITS;
const IOC_DIRSHIFT: c_ulong = IOC_SIZESHIFT + IOC_SIZEBITS;
const IOC_WRITE: c_ulong = 1;
const IOC_READ: c_ulong = 2;

const fn ioc(dir: c_ulong, type_: c_ulong, nr: c_ulong, size: c_ulong) -> c_ulong {
    (dir << IOC_DIRSHIFT) | (type_ << IOC_TYPESHIFT) | (nr << IOC_NRSHIFT) | (size << IOC_SIZESHIFT)
}

const fn iowr<T>(type_: c_ulong, nr: c_ulong) -> c_ulong {
    ioc(IOC_READ | IOC_WRITE, type_, nr, size_of::<T>() as c_ulong)
}

const fn iow<T>(type_: c_ulong, nr: c_ulong) -> c_ulong {
    ioc(IOC_WRITE, type_, nr, size_of::<T>() as c_ulong)
}

const PTP_CLK_MAGIC: c_ulong = b'=' as c_ulong;
const PTP_CLOCK_GETCAPS: c_ulong = iowr::<ptp_clock_caps>(PTP_CLK_MAGIC, 1);
const PTP_EXTTS_REQUEST: c_ulong = iow::<ptp_extts_request>(PTP_CLK_MAGIC, 2);
const PTP_PEROUT_REQUEST2: c_ulong = iow::<ptp_perout_request>(PTP_CLK_MAGIC, 12);
const PTP_ENABLE_PPS: c_ulong = iow::<c_int>(PTP_CLK_MAGIC, 4);
const PTP_SYS_OFFSET: c_ulong = iowr::<ptp_sys_offset>(PTP_CLK_MAGIC, 5);
const PTP_PIN_GETFUNC: c_ulong = iowr::<ptp_pin_desc>(PTP_CLK_MAGIC, 6);
const PTP_PIN_SETFUNC: c_ulong = iow::<ptp_pin_desc>(PTP_CLK_MAGIC, 7);
const PTP_SYS_OFFSET_PRECISE: c_ulong = iowr::<ptp_sys_offset_precise>(PTP_CLK_MAGIC, 8);
const PTP_SYS_OFFSET_EXTENDED: c_ulong = iowr::<ptp_sys_offset_extended>(PTP_CLK_MAGIC, 9);
const PTP_EXTTS_REQUEST2: c_ulong = iow::<ptp_extts_request>(PTP_CLK_MAGIC, 11);
const PTP_MASK_CLEAR_ALL: c_ulong = iow::<c_uint>(PTP_CLK_MAGIC, 19);
const PTP_MASK_EN_SINGLE: c_ulong = iow::<c_uint>(PTP_CLK_MAGIC, 20);

#[allow(non_camel_case_types)]
type clockid_t = c_int;
#[allow(non_camel_case_types)]
type time_t = c_long;
#[allow(non_camel_case_types)]
type suseconds_t = c_long;
#[allow(non_camel_case_types)]
type c_uint = u32;

#[repr(C)]
#[derive(Copy, Clone)]
struct timeval {
    tv_sec: time_t,
    tv_usec: suseconds_t,
}

#[repr(C)]
#[derive(Copy, Clone)]
struct timespec {
    tv_sec: time_t,
    tv_nsec: c_long,
}

#[repr(C)]
#[derive(Copy, Clone)]
struct timex {
    modes: c_uint,
    offset: c_long,
    freq: c_long,
    maxerror: c_long,
    esterror: c_long,
    status: c_int,
    constant: c_long,
    precision: c_long,
    tolerance: c_long,
    time: timeval,
    tick: c_long,
    ppsfreq: c_long,
    jitter: c_long,
    shift: c_int,
    stabil: c_long,
    jitcnt: c_long,
    calcnt: c_long,
    errcnt: c_long,
    stbcnt: c_long,
    tai: c_int,
    __padding: [c_int; 11],
}

#[repr(C)]
#[derive(Copy, Clone)]
struct ptp_clock_time {
    sec: i64,
    nsec: u32,
    reserved: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
struct ptp_clock_caps {
    max_adj: c_int,
    n_alarm: c_int,
    n_ext_ts: c_int,
    n_per_out: c_int,
    pps: c_int,
    n_pins: c_int,
    cross_timestamping: c_int,
    adjust_phase: c_int,
    max_phase_adj: c_int,
    rsv: [c_int; 11],
}

#[repr(C)]
#[derive(Copy, Clone)]
struct ptp_extts_request {
    index: c_uint,
    flags: c_uint,
    rsv: [c_uint; 2],
}

#[repr(C)]
#[derive(Copy, Clone)]
struct ptp_extts_event {
    index: c_uint,
    t: ptp_clock_time,
}

#[repr(C)]
#[derive(Copy, Clone)]
struct ptp_perout_request {
    start: ptp_clock_time,
    period: ptp_clock_time,
    index: c_uint,
    flags: c_uint,
    on: ptp_clock_time,
    phase: ptp_clock_time,
}

#[repr(C)]
#[derive(Copy, Clone)]
struct ptp_pin_desc {
    name: [c_char; 64],
    index: c_uint,
    func: c_uint,
    chan: c_uint,
    rsv: [c_uint; 5],
}

#[repr(C)]
#[derive(Copy, Clone)]
struct ptp_sys_offset {
    n_samples: c_uint,
    rsv: [c_uint; 3],
    ts: [ptp_clock_time; 2 * PTP_MAX_SAMPLES + 1],
}

#[repr(C)]
#[derive(Copy, Clone)]
struct ptp_sys_offset_extended {
    n_samples: c_uint,
    clockid: clockid_t,
    rsv: [c_uint; 2],
    ts: [[ptp_clock_time; 3]; PTP_MAX_SAMPLES],
}

#[repr(C)]
#[derive(Copy, Clone)]
struct ptp_sys_offset_precise {
    device: ptp_clock_time,
    sys_realtime: ptp_clock_time,
    sys_monoraw: ptp_clock_time,
    rsv: [c_uint; 4],
}

unsafe extern "C" {
    static mut errno: c_int;

    fn open(pathname: *const c_char, flags: c_int, ...) -> c_int;
    fn close(fd: c_int) -> c_int;
    fn read(fd: c_int, buf: *mut c_void, count: usize) -> isize;
    fn ioctl(fd: c_int, request: c_ulong, ...) -> c_int;
    fn clock_gettime(clk_id: clockid_t, tp: *mut timespec) -> c_int;
    fn clock_settime(clk_id: clockid_t, tp: *const timespec) -> c_int;
    fn clock_adjtime(clk_id: clockid_t, tx: *mut timex) -> c_int;
    fn ctime(timep: *const time_t) -> *mut c_char;
    fn strerror(errnum: c_int) -> *mut c_char;
    fn perror(s: *const c_char);
    fn calloc(nmemb: usize, size: usize) -> *mut c_void;
    fn free(ptr: *mut c_void);
    fn fflush(stream: *mut c_void) -> c_int;
    fn getchar() -> c_int;
    fn getopt(argc: c_int, argv: *const *mut c_char, optstring: *const c_char) -> c_int;
    fn atoi(nptr: *const c_char) -> c_int;
    fn atoll(nptr: *const c_char) -> i64;
    fn strcasecmp(s1: *const c_char, s2: *const c_char) -> c_int;
    fn strrchr(s: *const c_char, c: c_int) -> *mut c_char;
    fn sscanf(s: *const c_char, format: *const c_char, ...) -> c_int;
}

unsafe extern "C" {
    static mut optarg: *mut c_char;
    static mut stdout: *mut c_void;
}

unsafe fn c_string(ptr: *const c_char) -> String {
    if ptr.is_null() {
        String::new()
    } else {
        CStr::from_ptr(ptr).to_string_lossy().into_owned()
    }
}

unsafe fn show_flag_test(rq_index: c_int, flags: c_uint, err: c_int) {
    let rq = if rq_index != 0 {
        (b'1' + rq_index as u8) as char
    } else {
        ' '
    };
    println!(
        "PTP_EXTTS_REQUEST{} flags 0x{:08x} : ({}) {}",
        rq,
        flags,
        err,
        c_string(strerror(errno))
    );
    /* sigh, uClibc ... */
    errno = 0;
}

unsafe fn do_flag_test(fd: c_int, index: c_uint) {
    let mut extts_request: ptp_extts_request = zeroed();
    let request: [c_ulong; 2] = [PTP_EXTTS_REQUEST, PTP_EXTTS_REQUEST2];
    let enable_flags: [c_uint; 5] = [
        PTP_ENABLE_FEATURE,
        PTP_ENABLE_FEATURE | PTP_RISING_EDGE,
        PTP_ENABLE_FEATURE | PTP_FALLING_EDGE,
        PTP_ENABLE_FEATURE | PTP_RISING_EDGE | PTP_FALLING_EDGE,
        PTP_ENABLE_FEATURE | (PTP_EXTTS_VALID_FLAGS + 1),
    ];
    let mut err: c_int;

    extts_request.index = index;

    for i in 0..2 {
        for j in 0..5 {
            extts_request.flags = enable_flags[j];
            err = ioctl(fd, request[i], &mut extts_request);
            show_flag_test(i as c_int, extts_request.flags, err);

            extts_request.flags = 0;
            err = ioctl(fd, request[i], &mut extts_request);
            let _ = err;
        }
    }
}

fn get_clockid(fd: c_int) -> clockid_t {
    const CLOCKFD: c_int = 3;
    ((!fd as c_uint) << 3) as clockid_t | CLOCKFD
}

fn ppb_to_scaled_ppm(ppb: c_int) -> c_long {
    /*
     * The 'freq' field in the 'struct timex' is in parts per
     * million, but with a 16 bit binary fractional field.
     * Instead of calculating either one of
     *
     *    scaled_ppm = (ppb / 1000) << 16  [1]
     *    scaled_ppm = (ppb << 16) / 1000  [2]
     *
     * we simply use double precision math, in order to avoid the
     * truncation in [1] and the possible overflow in [2].
     */
    (ppb as f64 * 65.536) as c_long
}

unsafe fn pctns(t: *mut ptp_clock_time) -> i64 {
    (*t).sec * NSEC_PER_SEC + (*t).nsec as i64
}

fn usage(progname: &str) {
    eprint!(
        "usage: {} [options]\n\
 -c         query the ptp clock's capabilities\n\
 -d name    device to open\n\
 -e val     read 'val' external time stamp events\n\
 -E val     enable rising (1), falling (2), or both (3) edges\n\
 -f val     adjust the ptp clock frequency by 'val' ppb\n\
 -F chan    Enable single channel mask and keep device open for debugfs verification.\n\
 -g         get the ptp clock time\n\
 -h         prints this message\n\
 -i val     index for event/trigger\n\
 -k val     measure the time offset between system and phc clock\n\
            for 'val' times (Maximum 25)\n\
 -l         list the current pin configuration\n\
 -L pin,val configure pin index 'pin' with function 'val'\n\
            the channel index is taken from the '-i' option\n\
            'val' specifies the auxiliary function:\n\
            0 - none\n\
            1 - external time stamp\n\
            2 - periodic output\n\
 -n val     shift the ptp clock time by 'val' nanoseconds\n\
 -o val     phase offset (in nanoseconds) to be provided to the PHC servo\n\
 -p val     enable output with a period of 'val' nanoseconds\n\
 -H val     set output phase to 'val' nanoseconds (requires -p)\n\
 -w val     set output pulse width to 'val' nanoseconds (requires -p)\n\
 -P val     enable or disable (val=1|0) the system clock PPS\n\
 -r         open the ptp clock in readonly mode\n\
 -s         set the ptp clock time from the system time\n\
 -S         set the system time from the ptp clock time\n\
 -t val     shift the ptp clock time by 'val' seconds\n\
 -T val     set the ptp clock time to 'val' seconds\n\
 -x val     get an extended ptp clock time with the desired number of samples (up to {})\n\
 -X         get a ptp clock cross timestamp\n\
 -y val     pre/post tstamp timebase to use {{realtime|monotonic|monotonic-raw}}\n\
 -z         test combinations of rising/falling external time stamp flags\n",
        progname, PTP_MAX_SAMPLES
    );
}

unsafe fn perror_str(s: &str) {
    let cs = CString::new(s).unwrap();
    perror(cs.as_ptr());
}

unsafe fn main_impl(argc: c_int, argv: *mut *mut c_char) -> c_int {
    let mut caps: ptp_clock_caps = zeroed();
    let mut event: ptp_extts_event = zeroed();
    let mut extts_request: ptp_extts_request = zeroed();
    let mut perout_request: ptp_perout_request = zeroed();
    let mut desc: ptp_pin_desc = zeroed();
    let mut ts: timespec = zeroed();
    let mut tx: timex = zeroed();
    let mut pct: *mut ptp_clock_time;
    let mut sysoff: *mut ptp_sys_offset;
    let mut soe: *mut ptp_sys_offset_extended;
    let mut xts: *mut ptp_sys_offset_precise;

    let argv0 = *argv;
    let slash = strrchr(argv0, b'/' as c_int);
    let progname_ptr = if !slash.is_null() { slash.add(1) } else { argv0 };
    let progname = c_string(progname_ptr);

    let mut device_storage = CString::new(DEVICE).unwrap();
    let mut device: *mut c_char = device_storage.as_ptr() as *mut c_char;
    let mut adjfreq: c_int = 0x7fffffff;
    let mut adjtime: c_int = 0;
    let mut adjns: c_int = 0;
    let mut adjphase: c_int = 0;
    let mut capabilities: c_int = 0;
    let mut extts: c_int = 0;
    let mut edge: c_int = 0;
    let mut flagtest: c_int = 0;
    let mut gettime: c_int = 0;
    let mut index: c_int = 0;
    let mut list_pins: c_int = 0;
    let mut pct_offset: c_int = 0;
    let mut getextended: c_int = 0;
    let mut getcross: c_int = 0;
    let mut n_samples: c_int = 0;
    let mut pin_index: c_int = -1;
    let mut pin_func: c_int = 0;
    let mut pps: c_int = -1;
    let mut seconds: c_int = 0;
    let mut readonly: c_int = 0;
    let mut settime: c_int = 0;
    let mut channel: c_int = -1;
    let mut ext_clockid: clockid_t = CLOCK_REALTIME;

    let mut t1: i64;
    let mut t2: i64;
    let mut tp: i64;
    let mut interval: i64;
    let mut offset: i64;
    let mut perout_phase: i64 = -1;
    let mut pulsewidth: i64 = -1;
    let mut perout: i64 = -1;

    let optstring = CString::new("cd:e:E:f:F:ghH:i:k:lL:n:o:p:P:rsSt:T:w:x:Xy:z").unwrap();
    loop {
        let c = getopt(argc, argv as *const *mut c_char, optstring.as_ptr());
        if c == EOF {
            break;
        }
        match c as u8 as char {
            'c' => capabilities = 1,
            'd' => device = optarg,
            'e' => extts = atoi(optarg),
            'E' => {
                edge = atoi(optarg);
                edge = (if edge & 1 != 0 { PTP_RISING_EDGE as c_int } else { 0 })
                    | (if edge & 2 != 0 { PTP_FALLING_EDGE as c_int } else { 0 });
            }
            'f' => adjfreq = atoi(optarg),
            'F' => channel = atoi(optarg),
            'g' => gettime = 1,
            'H' => perout_phase = atoll(optarg),
            'i' => index = atoi(optarg),
            'k' => {
                pct_offset = 1;
                n_samples = atoi(optarg);
            }
            'l' => list_pins = 1,
            'L' => {
                let fmt = CString::new("%d,%d").unwrap();
                let cnt = sscanf(optarg, fmt.as_ptr(), &mut pin_index, &mut pin_func);
                if cnt != 2 {
                    usage(&progname);
                    return -1;
                }
            }
            'n' => adjns = atoi(optarg),
            'o' => adjphase = atoi(optarg),
            'p' => perout = atoll(optarg),
            'P' => pps = atoi(optarg),
            'r' => readonly = 1,
            's' => settime = 1,
            'S' => settime = 2,
            't' => adjtime = atoi(optarg),
            'T' => {
                settime = 3;
                seconds = atoi(optarg);
            }
            'w' => pulsewidth = atoi(optarg) as i64,
            'x' => {
                getextended = atoi(optarg);
                if getextended < 1 || getextended > PTP_MAX_SAMPLES as c_int {
                    eprintln!(
                        "number of extended timestamp samples must be between 1 and {}; was asked for {}",
                        PTP_MAX_SAMPLES, getextended
                    );
                    return -1;
                }
            }
            'X' => getcross = 1,
            'y' => {
                let realtime = CString::new("realtime").unwrap();
                let monotonic = CString::new("monotonic").unwrap();
                let monotonic_raw = CString::new("monotonic-raw").unwrap();
                if strcasecmp(optarg, realtime.as_ptr()) == 0 {
                    ext_clockid = CLOCK_REALTIME;
                } else if strcasecmp(optarg, monotonic.as_ptr()) == 0 {
                    ext_clockid = CLOCK_MONOTONIC;
                } else if strcasecmp(optarg, monotonic_raw.as_ptr()) == 0 {
                    ext_clockid = CLOCK_MONOTONIC_RAW;
                } else {
                    eprintln!(
                        "type needs to be realtime, monotonic or monotonic-raw; was given {}",
                        c_string(optarg)
                    );
                    return -1;
                }
            }
            'z' => flagtest = 1,
            'h' => {
                usage(&progname);
                return 0;
            }
            '?' | _ => {
                usage(&progname);
                return -1;
            }
        }
    }

    let fd = open(device, if readonly != 0 { O_RDONLY } else { O_RDWR });
    if fd < 0 {
        eprintln!("opening {}: {}", c_string(device), c_string(strerror(errno)));
        return -1;
    }

    let clkid = get_clockid(fd);
    if CLOCK_INVALID == clkid {
        eprintln!("failed to read clock id");
        return -1;
    }

    if capabilities != 0 {
        if ioctl(fd, PTP_CLOCK_GETCAPS, &mut caps) != 0 {
            perror_str("PTP_CLOCK_GETCAPS");
        } else {
            print!(
                "capabilities:\n\
  {} maximum frequency adjustment (ppb)\n\
  {} programmable alarms\n\
  {} external time stamp channels\n\
  {} programmable periodic signals\n\
  {} pulse per second\n\
  {} programmable pins\n\
  {} cross timestamping\n\
  {} adjust_phase\n\
  {} maximum phase adjustment (ns)\n",
                caps.max_adj,
                caps.n_alarm,
                caps.n_ext_ts,
                caps.n_per_out,
                caps.pps,
                caps.n_pins,
                caps.cross_timestamping,
                caps.adjust_phase,
                caps.max_phase_adj
            );
        }
    }

    if 0x7fffffff != adjfreq {
        tx = zeroed();
        tx.modes = ADJ_FREQUENCY as c_uint;
        tx.freq = ppb_to_scaled_ppm(adjfreq);
        if clock_adjtime(clkid, &mut tx) != 0 {
            perror_str("clock_adjtime");
        } else {
            println!("frequency adjustment okay");
        }
    }

    if adjtime != 0 || adjns != 0 {
        tx = zeroed();
        tx.modes = (ADJ_SETOFFSET | ADJ_NANO) as c_uint;
        tx.time.tv_sec = adjtime as time_t;
        tx.time.tv_usec = adjns as suseconds_t;
        while tx.time.tv_usec < 0 {
            tx.time.tv_sec -= 1;
            tx.time.tv_usec += NSEC_PER_SEC as suseconds_t;
        }

        if clock_adjtime(clkid, &mut tx) < 0 {
            perror_str("clock_adjtime");
        } else {
            println!("time shift okay");
        }
    }

    if adjphase != 0 {
        tx = zeroed();
        tx.modes = (ADJ_OFFSET | ADJ_NANO) as c_uint;
        tx.offset = adjphase as c_long;

        if clock_adjtime(clkid, &mut tx) < 0 {
            perror_str("clock_adjtime");
        } else {
            println!("phase adjustment okay");
        }
    }

    if gettime != 0 {
        if clock_gettime(clkid, &mut ts) != 0 {
            perror_str("clock_gettime");
        } else {
            print!(
                "clock time: {}.{:09} or {}",
                ts.tv_sec,
                ts.tv_nsec,
                c_string(ctime(&ts.tv_sec))
            );
        }
    }

    if settime == 1 {
        clock_gettime(CLOCK_REALTIME, &mut ts);
        if clock_settime(clkid, &ts) != 0 {
            perror_str("clock_settime");
        } else {
            println!("set time okay");
        }
    }

    if settime == 2 {
        clock_gettime(clkid, &mut ts);
        if clock_settime(CLOCK_REALTIME, &ts) != 0 {
            perror_str("clock_settime");
        } else {
            println!("set time okay");
        }
    }

    if settime == 3 {
        ts.tv_sec = seconds as time_t;
        ts.tv_nsec = 0;
        if clock_settime(clkid, &ts) != 0 {
            perror_str("clock_settime");
        } else {
            println!("set time okay");
        }
    }

    if pin_index >= 0 {
        desc = zeroed();
        desc.index = pin_index as c_uint;
        desc.func = pin_func as c_uint;
        desc.chan = index as c_uint;
        if ioctl(fd, PTP_PIN_SETFUNC, &mut desc) != 0 {
            perror_str("PTP_PIN_SETFUNC");
        } else {
            println!("set pin function okay");
        }
    }

    if extts != 0 {
        if readonly == 0 {
            extts_request = zeroed();
            extts_request.index = index as c_uint;
            extts_request.flags = PTP_ENABLE_FEATURE | edge as c_uint;
            if ioctl(fd, PTP_EXTTS_REQUEST, &mut extts_request) != 0 {
                perror_str("PTP_EXTTS_REQUEST");
                extts = 0;
            } else {
                println!("external time stamp request okay");
            }
        }
        while extts != 0 {
            let cnt = read(
                fd,
                &mut event as *mut ptp_extts_event as *mut c_void,
                size_of::<ptp_extts_event>(),
            );
            if cnt != size_of::<ptp_extts_event>() as isize {
                perror_str("read");
                break;
            }
            println!("event index {} at {}.{:09}", event.index, event.t.sec, event.t.nsec);
            fflush(stdout);
            extts -= 1;
        }
        if readonly == 0 {
            /* Disable the feature again. */
            extts_request.flags = 0;
            if ioctl(fd, PTP_EXTTS_REQUEST, &mut extts_request) != 0 {
                perror_str("PTP_EXTTS_REQUEST");
            }
        }
    }

    if flagtest != 0 {
        do_flag_test(fd, index as c_uint);
    }

    if list_pins != 0 {
        let mut n_pins: c_int = 0;
        if ioctl(fd, PTP_CLOCK_GETCAPS, &mut caps) != 0 {
            perror_str("PTP_CLOCK_GETCAPS");
        } else {
            n_pins = caps.n_pins;
        }
        let mut i: c_uint = 0;
        while i < n_pins as c_uint {
            desc.index = i;
            if ioctl(fd, PTP_PIN_GETFUNC, &mut desc) != 0 {
                perror_str("PTP_PIN_GETFUNC");
                break;
            }
            println!(
                "name {} index {} func {} chan {}",
                c_string(desc.name.as_ptr()),
                desc.index,
                desc.func,
                desc.chan
            );
            i += 1;
        }
    }

    if pulsewidth >= 0 && perout < 0 {
        println!("-w can only be specified together with -p");
        return -1;
    }

    if perout_phase >= 0 && perout < 0 {
        println!("-H can only be specified together with -p");
        return -1;
    }

    if perout >= 0 {
        if clock_gettime(clkid, &mut ts) != 0 {
            perror_str("clock_gettime");
            return -1;
        }
        perout_request = zeroed();
        perout_request.index = index as c_uint;
        perout_request.period.sec = perout / NSEC_PER_SEC;
        perout_request.period.nsec = (perout % NSEC_PER_SEC) as u32;
        perout_request.flags = 0;
        if pulsewidth >= 0 {
            perout_request.flags |= PTP_PEROUT_DUTY_CYCLE;
            perout_request.on.sec = pulsewidth / NSEC_PER_SEC;
            perout_request.on.nsec = (pulsewidth % NSEC_PER_SEC) as u32;
        }
        if perout_phase >= 0 {
            perout_request.flags |= PTP_PEROUT_PHASE;
            perout_request.phase.sec = perout_phase / NSEC_PER_SEC;
            perout_request.phase.nsec = (perout_phase % NSEC_PER_SEC) as u32;
        } else {
            perout_request.start.sec = ts.tv_sec as i64 + 2;
            perout_request.start.nsec = 0;
        }

        if ioctl(fd, PTP_PEROUT_REQUEST2, &mut perout_request) != 0 {
            perror_str("PTP_PEROUT_REQUEST");
        } else {
            println!("periodic output request okay");
        }
    }

    if pps != -1 {
        let enable: c_int = if pps != 0 { 1 } else { 0 };
        if ioctl(fd, PTP_ENABLE_PPS, enable) != 0 {
            perror_str("PTP_ENABLE_PPS");
        } else {
            println!("pps for system time request okay");
        }
    }

    if pct_offset != 0 {
        if n_samples <= 0 || n_samples > 25 {
            println!("n_samples should be between 1 and 25");
            usage(&progname);
            return -1;
        }

        sysoff = calloc(1, size_of::<ptp_sys_offset>()) as *mut ptp_sys_offset;
        if sysoff.is_null() {
            perror_str("calloc");
            return -1;
        }
        (*sysoff).n_samples = n_samples as c_uint;

        if ioctl(fd, PTP_SYS_OFFSET, sysoff) != 0 {
            perror_str("PTP_SYS_OFFSET");
        } else {
            println!("system and phc clock time offset request okay");
        }

        pct = (*sysoff).ts.as_mut_ptr();
        let mut i: c_uint = 0;
        while i < (*sysoff).n_samples {
            t1 = pctns(pct.add(2 * i as usize));
            tp = pctns(pct.add(2 * i as usize + 1));
            t2 = pctns(pct.add(2 * i as usize + 2));
            interval = t2 - t1;
            offset = (t2 + t1) / 2 - tp;

            println!(
                "system time: {}.{:09}",
                (*pct.add(2 * i as usize)).sec,
                (*pct.add(2 * i as usize)).nsec
            );
            println!(
                "phc    time: {}.{:09}",
                (*pct.add(2 * i as usize + 1)).sec,
                (*pct.add(2 * i as usize + 1)).nsec
            );
            println!(
                "system time: {}.{:09}",
                (*pct.add(2 * i as usize + 2)).sec,
                (*pct.add(2 * i as usize + 2)).nsec
            );
            println!(
                "system/phc clock time offset is {} ns\nsystem     clock time delay  is {} ns",
                offset, interval
            );
            i += 1;
        }

        free(sysoff as *mut c_void);
    }

    if getextended != 0 {
        soe = calloc(1, size_of::<ptp_sys_offset_extended>()) as *mut ptp_sys_offset_extended;
        if soe.is_null() {
            perror_str("calloc");
            return -1;
        }

        (*soe).n_samples = getextended as c_uint;
        (*soe).clockid = ext_clockid;

        if ioctl(fd, PTP_SYS_OFFSET_EXTENDED, soe) != 0 {
            perror_str("PTP_SYS_OFFSET_EXTENDED");
        } else {
            println!("extended timestamp request returned {} samples", getextended);

            let mut i: c_uint = 0;
            while i < getextended as c_uint {
                match ext_clockid {
                    CLOCK_REALTIME => println!(
                        "sample #{:2}: real time before: {}.{:09}",
                        i, (*soe).ts[i as usize][0].sec, (*soe).ts[i as usize][0].nsec
                    ),
                    CLOCK_MONOTONIC => println!(
                        "sample #{:2}: monotonic time before: {}.{:09}",
                        i, (*soe).ts[i as usize][0].sec, (*soe).ts[i as usize][0].nsec
                    ),
                    CLOCK_MONOTONIC_RAW => println!(
                        "sample #{:2}: monotonic-raw time before: {}.{:09}",
                        i, (*soe).ts[i as usize][0].sec, (*soe).ts[i as usize][0].nsec
                    ),
                    _ => {}
                }
                println!(
                    "            phc time: {}.{:09}",
                    (*soe).ts[i as usize][1].sec, (*soe).ts[i as usize][1].nsec
                );
                match ext_clockid {
                    CLOCK_REALTIME => println!(
                        "            real time after: {}.{:09}",
                        (*soe).ts[i as usize][2].sec, (*soe).ts[i as usize][2].nsec
                    ),
                    CLOCK_MONOTONIC => println!(
                        "            monotonic time after: {}.{:09}",
                        (*soe).ts[i as usize][2].sec, (*soe).ts[i as usize][2].nsec
                    ),
                    CLOCK_MONOTONIC_RAW => println!(
                        "            monotonic-raw time after: {}.{:09}",
                        (*soe).ts[i as usize][2].sec, (*soe).ts[i as usize][2].nsec
                    ),
                    _ => {}
                }
                i += 1;
            }
        }

        free(soe as *mut c_void);
    }

    if getcross != 0 {
        xts = calloc(1, size_of::<ptp_sys_offset_precise>()) as *mut ptp_sys_offset_precise;
        if xts.is_null() {
            perror_str("calloc");
            return -1;
        }

        if ioctl(fd, PTP_SYS_OFFSET_PRECISE, xts) != 0 {
            perror_str("PTP_SYS_OFFSET_PRECISE");
        } else {
            println!("system and phc crosstimestamping request okay");

            println!("device time: {}.{:09}", (*xts).device.sec, (*xts).device.nsec);
            println!(
                "system time: {}.{:09}",
                (*xts).sys_realtime.sec, (*xts).sys_realtime.nsec
            );
            println!("monoraw time: {}.{:09}", (*xts).sys_monoraw.sec, (*xts).sys_monoraw.nsec);
        }

        free(xts as *mut c_void);
    }

    if channel >= 0 {
        if ioctl(fd, PTP_MASK_CLEAR_ALL) != 0 {
            perror_str("PTP_MASK_CLEAR_ALL");
        } else if ioctl(fd, PTP_MASK_EN_SINGLE, &mut channel as *mut c_int as *mut c_uint) != 0 {
            perror_str("PTP_MASK_EN_SINGLE");
        } else {
            println!("Channel {} exclusively enabled. Check on debugfs.", channel);
            print!("Press any key to continue\n.");
            getchar();
        }
    }

    close(fd);
    0
}

fn main() {
    let args: Vec<CString> = env::args()
        .map(|arg| CString::new(arg).unwrap())
        .collect();
    let mut argv: Vec<*mut c_char> = args.iter().map(|arg| arg.as_ptr() as *mut c_char).collect();
    argv.push(ptr::null_mut());
    unsafe {
        std::process::exit(main_impl(args.len() as c_int, argv.as_mut_ptr()));
    }
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
