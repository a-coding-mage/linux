// SPDX-License-Identifier: GPL-2.0
/*
* Watchdog Driver Test Program
* - Tests all ioctls
* - Tests Magic Close - CONFIG_WATCHDOG_NOWAYOUT
* - Could be tested against softdog driver on systems that
*   don't have watchdog hardware.
* - TODO:
* - Enhance test to add coverage for WDIOC_GETTEMP.
*
* Reference: Documentation/watchdog/watchdog-api.rst
 */

use core::ffi::{c_char, c_int, c_ulong, c_void};

const DEFAULT_PING_RATE: u32 = 1;

static mut fd: c_int = 0;
const v: c_char = b'V' as c_char;
static sopts: &[u8] = b"bdehp:st:Tn:NLf:i\0";

#[repr(C)]
struct option {
    name: *const c_char,
    has_arg: c_int,
    flag: *mut c_int,
    val: c_int,
}

#[repr(C)]
struct watchdog_info {
    options: u32,
    firmware_version: u32,
    identity: [c_char; 32],
}

struct wdiof_status {
    flag: c_int,
    status_str: *const c_char,
}

unsafe impl Sync for wdiof_status {}
unsafe impl Sync for option {}

const no_argument: c_int = 0;
const required_argument: c_int = 1;

const O_WRONLY: c_int = 1;
const ENOENT: c_int = 2;
const EACCES: c_int = 13;
const SIGINT: c_int = 2;
const SIGQUIT: c_int = 3;
const SIGKILL: c_int = 9;
const SIGTERM: c_int = 15;

/* Constants from <linux/watchdog.h> and ioctl encoding from system headers. */
const WDIOS_DISABLECARD: c_int = 0x0001;
const WDIOS_ENABLECARD: c_int = 0x0002;
const WDIOS_TEMPPANIC: c_int = 0x0004;
const WDIOS_UNKNOWN: c_int = -1;

const WDIOF_OVERHEAT: c_int = 0x0001;
const WDIOF_FANFAULT: c_int = 0x0002;
const WDIOF_EXTERN1: c_int = 0x0004;
const WDIOF_EXTERN2: c_int = 0x0008;
const WDIOF_POWERUNDER: c_int = 0x0010;
const WDIOF_CARDRESET: c_int = 0x0020;
const WDIOF_POWEROVER: c_int = 0x0040;
const WDIOF_SETTIMEOUT: c_int = 0x0080;
const WDIOF_MAGICCLOSE: c_int = 0x0100;
const WDIOF_PRETIMEOUT: c_int = 0x0200;
const WDIOF_ALARMONLY: c_int = 0x0400;
const WDIOF_KEEPALIVEPING: c_int = 0x8000;
const WDIOF_UNKNOWN: c_int = -1;

const WDIOC_GETSUPPORT: c_ulong = 0x80285700;
const WDIOC_GETSTATUS: c_ulong = 0x80045701;
const WDIOC_GETBOOTSTATUS: c_ulong = 0x80045702;
const WDIOC_GETTEMP: c_ulong = 0x80045703;
const WDIOC_SETOPTIONS: c_ulong = 0x80045704;
const WDIOC_KEEPALIVE: c_ulong = 0x80045705;
const WDIOC_SETTIMEOUT: c_ulong = 0xc0045706;
const WDIOC_GETTIMEOUT: c_ulong = 0x80045707;
const WDIOC_SETPRETIMEOUT: c_ulong = 0xc0045708;
const WDIOC_GETPRETIMEOUT: c_ulong = 0x80045709;
const WDIOC_GETTIMELEFT: c_ulong = 0x8004570a;

unsafe extern "C" {
    static mut optarg: *mut c_char;
    static mut optind: c_int;

    fn __errno_location() -> *mut c_int;
    fn setbuf(stream: *mut FILE, buf: *mut c_char);
    static mut stdout: *mut FILE;
    fn printf(format: *const c_char, ...) -> c_int;
    fn exit(status: c_int) -> !;
    fn strerror(errnum: c_int) -> *mut c_char;
    fn open(pathname: *const c_char, flags: c_int, ...) -> c_int;
    fn close(fd: c_int) -> c_int;
    fn write(fd: c_int, buf: *const c_void, count: usize) -> isize;
    fn ioctl(fd: c_int, request: c_ulong, ...) -> c_int;
    fn getopt_long(
        argc: c_int,
        argv: *const *mut c_char,
        optstring: *const c_char,
        longopts: *const option,
        longindex: *mut c_int,
    ) -> c_int;
    fn strtoul(nptr: *const c_char, endptr: *mut *mut c_char, base: c_int) -> c_ulong;
    fn signal(signum: c_int, handler: extern "C" fn(c_int)) -> extern "C" fn(c_int);
    fn sleep(seconds: u32) -> u32;
}

#[repr(C)]
struct FILE {
    _private: [u8; 0],
}

static lopts: [option; 14] = [
    option { name: b"bootstatus\0".as_ptr() as *const c_char, has_arg: no_argument, flag: core::ptr::null_mut(), val: b'b' as c_int },
    option { name: b"disable\0".as_ptr() as *const c_char, has_arg: no_argument, flag: core::ptr::null_mut(), val: b'd' as c_int },
    option { name: b"enable\0".as_ptr() as *const c_char, has_arg: no_argument, flag: core::ptr::null_mut(), val: b'e' as c_int },
    option { name: b"help\0".as_ptr() as *const c_char, has_arg: no_argument, flag: core::ptr::null_mut(), val: b'h' as c_int },
    option { name: b"pingrate\0".as_ptr() as *const c_char, has_arg: required_argument, flag: core::ptr::null_mut(), val: b'p' as c_int },
    option { name: b"status\0".as_ptr() as *const c_char, has_arg: no_argument, flag: core::ptr::null_mut(), val: b's' as c_int },
    option { name: b"timeout\0".as_ptr() as *const c_char, has_arg: required_argument, flag: core::ptr::null_mut(), val: b't' as c_int },
    option { name: b"gettimeout\0".as_ptr() as *const c_char, has_arg: no_argument, flag: core::ptr::null_mut(), val: b'T' as c_int },
    option { name: b"pretimeout\0".as_ptr() as *const c_char, has_arg: required_argument, flag: core::ptr::null_mut(), val: b'n' as c_int },
    option { name: b"getpretimeout\0".as_ptr() as *const c_char, has_arg: no_argument, flag: core::ptr::null_mut(), val: b'N' as c_int },
    option { name: b"gettimeleft\0".as_ptr() as *const c_char, has_arg: no_argument, flag: core::ptr::null_mut(), val: b'L' as c_int },
    option { name: b"file\0".as_ptr() as *const c_char, has_arg: required_argument, flag: core::ptr::null_mut(), val: b'f' as c_int },
    option { name: b"info\0".as_ptr() as *const c_char, has_arg: no_argument, flag: core::ptr::null_mut(), val: b'i' as c_int },
    option { name: core::ptr::null(), has_arg: no_argument, flag: core::ptr::null_mut(), val: 0x0 },
];

/*
 * This function simply sends an IOCTL to the driver, which in turn ticks
 * the PC Watchdog card to reset its internal timer so it doesn't trigger
 * a computer reset.
 */
unsafe fn keep_alive() {
    let mut dummy: c_int = 0;
    let ret: c_int;

    ret = ioctl(fd, WDIOC_KEEPALIVE, &mut dummy as *mut c_int);
    if ret == 0 {
        printf(c".\n".as_ptr());
    }
}

/*
 * The main program.  Run the program with "-d" to disable the card,
 * or "-e" to enable the card.
 */

extern "C" fn term(_sig: c_int) {
    unsafe {
        let ret = write(fd, &v as *const c_char as *const c_void, 1);

        close(fd);
        if ret < 0 {
            printf(c"\nStopping watchdog ticks failed (%d)...\n".as_ptr(), *__errno_location());
        } else {
            printf(c"\nStopping watchdog ticks...\n".as_ptr());
        }
        exit(0);
    }
}

unsafe fn usage(progname: *mut c_char) {
    printf(c"Usage: %s [options]\n".as_ptr(), progname);
    printf(c" -f, --file\t\tOpen watchdog device file\n".as_ptr());
    printf(c"\t\t\tDefault is /dev/watchdog\n".as_ptr());
    printf(c" -i, --info\t\tShow watchdog_info\n".as_ptr());
    printf(c" -s, --status\t\tGet status & supported features\n".as_ptr());
    printf(c" -b, --bootstatus\tGet last boot status (Watchdog/POR)\n".as_ptr());
    printf(c" -d, --disable\t\tTurn off the watchdog timer\n".as_ptr());
    printf(c" -e, --enable\t\tTurn on the watchdog timer\n".as_ptr());
    printf(c" -h, --help\t\tPrint the help message\n".as_ptr());
    printf(c" -p, --pingrate=P\tSet ping rate to P seconds (default %d)\n".as_ptr(), DEFAULT_PING_RATE);
    printf(c" -t, --timeout=T\tSet timeout to T seconds\n".as_ptr());
    printf(c" -T, --gettimeout\tGet the timeout\n".as_ptr());
    printf(c" -n, --pretimeout=T\tSet the pretimeout to T seconds\n".as_ptr());
    printf(c" -N, --getpretimeout\tGet the pretimeout\n".as_ptr());
    printf(c" -L, --gettimeleft\tGet the time left until timer expires\n".as_ptr());
    printf(c"\n".as_ptr());
    printf(c"Parameters are parsed left-to-right in real-time.\n".as_ptr());
    printf(c"Example: %s -d -t 10 -p 5 -e\n".as_ptr(), progname);
    printf(c"Example: %s -t 12 -T -n 7 -N\n".as_ptr(), progname);
}

const WDIOF_NUM_STATUS: usize = 8;

static wdiof_status: [wdiof_status; WDIOF_NUM_STATUS] = [
    wdiof_status { flag: WDIOF_SETTIMEOUT, status_str: c"Set timeout (in seconds)".as_ptr() },
    wdiof_status { flag: WDIOF_MAGICCLOSE, status_str: c"Supports magic close char".as_ptr() },
    wdiof_status { flag: WDIOF_PRETIMEOUT, status_str: c"Pretimeout (in seconds), get/set".as_ptr() },
    wdiof_status { flag: WDIOF_ALARMONLY, status_str: c"Watchdog triggers a management or other external alarm not a reboot".as_ptr() },
    wdiof_status { flag: WDIOF_KEEPALIVEPING, status_str: c"Keep alive ping reply".as_ptr() },
    wdiof_status { flag: WDIOS_DISABLECARD, status_str: c"Turn off the watchdog timer".as_ptr() },
    wdiof_status { flag: WDIOS_ENABLECARD, status_str: c"Turn on the watchdog timer".as_ptr() },
    wdiof_status { flag: WDIOS_TEMPPANIC, status_str: c"Kernel panic on temperature trip".as_ptr() },
];

unsafe fn print_status(flags: c_int) {
    let mut wdiof: c_int = 0;

    if flags == WDIOS_UNKNOWN {
        printf(c"Unknown status error from WDIOC_GETSTATUS\n".as_ptr());
        return;
    }

    while wdiof < WDIOF_NUM_STATUS as c_int {
        if flags & wdiof_status[wdiof as usize].flag != 0 {
            printf(c"Support/Status: %s\n".as_ptr(), wdiof_status[wdiof as usize].status_str);
        }
        wdiof += 1;
    }
}

const WDIOF_NUM_BOOTSTATUS: usize = 7;

static wdiof_bootstatus: [wdiof_status; WDIOF_NUM_BOOTSTATUS] = [
    wdiof_status { flag: WDIOF_OVERHEAT, status_str: c"Reset due to CPU overheat".as_ptr() },
    wdiof_status { flag: WDIOF_FANFAULT, status_str: c"Fan failed".as_ptr() },
    wdiof_status { flag: WDIOF_EXTERN1, status_str: c"External relay 1".as_ptr() },
    wdiof_status { flag: WDIOF_EXTERN2, status_str: c"External relay 2".as_ptr() },
    wdiof_status { flag: WDIOF_POWERUNDER, status_str: c"Power bad/power fault".as_ptr() },
    wdiof_status { flag: WDIOF_CARDRESET, status_str: c"Card previously reset the CPU".as_ptr() },
    wdiof_status { flag: WDIOF_POWEROVER, status_str: c"Power over voltage".as_ptr() },
];

unsafe fn print_boot_status(flags: c_int) {
    let mut wdiof: c_int = 0;

    if flags == WDIOF_UNKNOWN {
        printf(c"Unknown flag error from WDIOC_GETBOOTSTATUS\n".as_ptr());
        return;
    }

    if flags == 0 {
        printf(c"Last boot is caused by: Power-On-Reset\n".as_ptr());
        return;
    }

    while wdiof < WDIOF_NUM_BOOTSTATUS as c_int {
        if flags & wdiof_bootstatus[wdiof as usize].flag != 0 {
            printf(c"Last boot is caused by: %s\n".as_ptr(), wdiof_bootstatus[wdiof as usize].status_str);
        }
        wdiof += 1;
    }
}

unsafe fn main_0(argc: c_int, argv: *mut *mut c_char) -> c_int {
    let mut flags: c_int;
    let mut ping_rate: u32 = DEFAULT_PING_RATE;
    let mut ret: c_int;
    let mut c: c_int;
    let mut oneshot: c_int = 0;
    let mut file: *mut c_char = c"/dev/watchdog".as_ptr() as *mut c_char;
    let mut info: watchdog_info = core::mem::zeroed();
    let mut temperature: c_int = 0;

    setbuf(stdout, core::ptr::null_mut());

    loop {
        c = getopt_long(argc, argv as *const *mut c_char, sopts.as_ptr() as *const c_char, lopts.as_ptr(), core::ptr::null_mut());
        if c == -1 {
            break;
        }
        if c == b'f' as c_int {
            file = optarg;
        }
    }

    fd = open(file, O_WRONLY);

    if fd == -1 {
        if *__errno_location() == ENOENT {
            printf(c"Watchdog device (%s) not found.\n".as_ptr(), file);
        } else if *__errno_location() == EACCES {
            printf(c"Run watchdog as root.\n".as_ptr());
        } else {
            printf(c"Watchdog device open failed %s\n".as_ptr(), strerror(*__errno_location()));
        }
        exit(-1);
    }

    /*
     * Validate that `file` is a watchdog device
     */
    ret = ioctl(fd, WDIOC_GETSUPPORT, &mut info as *mut watchdog_info);
    if ret != 0 {
        printf(c"WDIOC_GETSUPPORT error '%s'\n".as_ptr(), strerror(*__errno_location()));
        close(fd);
        exit(ret);
    }

    optind = 0;

    loop {
        c = getopt_long(argc, argv as *const *mut c_char, sopts.as_ptr() as *const c_char, lopts.as_ptr(), core::ptr::null_mut());
        if c == -1 {
            break;
        }
        match c {
            x if x == b'b' as c_int => {
                flags = 0;
                oneshot = 1;
                ret = ioctl(fd, WDIOC_GETBOOTSTATUS, &mut flags as *mut c_int);
                if ret == 0 {
                    print_boot_status(flags);
                } else {
                    printf(c"WDIOC_GETBOOTSTATUS error '%s'\n".as_ptr(), strerror(*__errno_location()));
                }
            }
            x if x == b'd' as c_int => {
                flags = WDIOS_DISABLECARD;
                ret = ioctl(fd, WDIOC_SETOPTIONS, &mut flags as *mut c_int);
                if ret == 0 {
                    printf(c"Watchdog card disabled.\n".as_ptr());
                } else {
                    printf(c"WDIOS_DISABLECARD error '%s'\n".as_ptr(), strerror(*__errno_location()));
                    oneshot = 1;
                }
            }
            x if x == b'e' as c_int => {
                flags = WDIOS_ENABLECARD;
                ret = ioctl(fd, WDIOC_SETOPTIONS, &mut flags as *mut c_int);
                if ret == 0 {
                    printf(c"Watchdog card enabled.\n".as_ptr());
                } else {
                    printf(c"WDIOS_ENABLECARD error '%s'\n".as_ptr(), strerror(*__errno_location()));
                    oneshot = 1;
                }
            }
            x if x == b'p' as c_int => {
                ping_rate = strtoul(optarg, core::ptr::null_mut(), 0) as u32;
                if ping_rate == 0 {
                    ping_rate = DEFAULT_PING_RATE;
                }
                printf(c"Watchdog ping rate set to %u seconds.\n".as_ptr(), ping_rate);
            }
            x if x == b's' as c_int => {
                flags = 0;
                oneshot = 1;
                ret = ioctl(fd, WDIOC_GETSTATUS, &mut flags as *mut c_int);
                if ret == 0 {
                    print_status(flags);
                } else {
                    printf(c"WDIOC_GETSTATUS error '%s'\n".as_ptr(), strerror(*__errno_location()));
                }
                ret = ioctl(fd, WDIOC_GETTEMP, &mut temperature as *mut c_int);
                if ret != 0 {
                    printf(c"WDIOC_GETTEMP: '%s'\n".as_ptr(), strerror(*__errno_location()));
                } else {
                    printf(c"Temperature %d\n".as_ptr(), temperature);
                }
            }
            x if x == b't' as c_int => {
                flags = strtoul(optarg, core::ptr::null_mut(), 0) as c_int;
                ret = ioctl(fd, WDIOC_SETTIMEOUT, &mut flags as *mut c_int);
                if ret == 0 {
                    printf(c"Watchdog timeout set to %u seconds.\n".as_ptr(), flags);
                } else {
                    printf(c"WDIOC_SETTIMEOUT error '%s'\n".as_ptr(), strerror(*__errno_location()));
                    oneshot = 1;
                }
            }
            x if x == b'T' as c_int => {
                oneshot = 1;
                ret = ioctl(fd, WDIOC_GETTIMEOUT, &mut flags as *mut c_int);
                if ret == 0 {
                    printf(c"WDIOC_GETTIMEOUT returns %u seconds.\n".as_ptr(), flags);
                } else {
                    printf(c"WDIOC_GETTIMEOUT error '%s'\n".as_ptr(), strerror(*__errno_location()));
                }
            }
            x if x == b'n' as c_int => {
                flags = strtoul(optarg, core::ptr::null_mut(), 0) as c_int;
                ret = ioctl(fd, WDIOC_SETPRETIMEOUT, &mut flags as *mut c_int);
                if ret == 0 {
                    printf(c"Watchdog pretimeout set to %u seconds.\n".as_ptr(), flags);
                } else {
                    printf(c"WDIOC_SETPRETIMEOUT error '%s'\n".as_ptr(), strerror(*__errno_location()));
                    oneshot = 1;
                }
            }
            x if x == b'N' as c_int => {
                oneshot = 1;
                ret = ioctl(fd, WDIOC_GETPRETIMEOUT, &mut flags as *mut c_int);
                if ret == 0 {
                    printf(c"WDIOC_GETPRETIMEOUT returns %u seconds.\n".as_ptr(), flags);
                } else {
                    printf(c"WDIOC_GETPRETIMEOUT error '%s'\n".as_ptr(), strerror(*__errno_location()));
                }
            }
            x if x == b'L' as c_int => {
                oneshot = 1;
                ret = ioctl(fd, WDIOC_GETTIMELEFT, &mut flags as *mut c_int);
                if ret == 0 {
                    printf(c"WDIOC_GETTIMELEFT returns %u seconds.\n".as_ptr(), flags);
                } else {
                    printf(c"WDIOC_GETTIMELEFT error '%s'\n".as_ptr(), strerror(*__errno_location()));
                }
            }
            x if x == b'f' as c_int => {
                /* Handled above */
            }
            x if x == b'i' as c_int => {
                /*
                 * watchdog_info was obtained as part of file open
                 * validation. So we just show it here.
                 */
                oneshot = 1;
                printf(c"watchdog_info:\n".as_ptr());
                printf(c" identity:\t\t%s\n".as_ptr(), info.identity.as_ptr());
                printf(c" firmware_version:\t%u\n".as_ptr(), info.firmware_version);
                print_status(info.options as c_int);
            }
            _ => {
                usage(*argv);
                break;
            }
        }
    }

    if oneshot != 0 {
        /*
         * Send specific magic character 'V' just in case Magic Close is
         * enabled to ensure watchdog gets disabled on close.
         */
        ret = write(fd, &v as *const c_char as *const c_void, 1) as c_int;
        if ret < 0 {
            printf(c"Stopping watchdog ticks failed (%d)...\n".as_ptr(), *__errno_location());
        }
        close(fd);
        return 0;
    }

    /* Check if WDIOF_KEEPALIVEPING is supported */
    if info.options & WDIOF_KEEPALIVEPING as u32 == 0 {
        printf(c"WDIOC_KEEPALIVE not supported by this device\n".as_ptr());
        /*
         * Send specific magic character 'V' just in case Magic Close is
         * enabled to ensure watchdog gets disabled on close.
         */
        ret = write(fd, &v as *const c_char as *const c_void, 1) as c_int;
        if ret < 0 {
            printf(c"Stopping watchdog ticks failed (%d)...\n".as_ptr(), *__errno_location());
        }
        close(fd);
        return 0;
    }

    printf(c"Watchdog Ticking Away!\n".as_ptr());

    /*
     * Register the signals
     */
    signal(SIGINT, term);
    signal(SIGTERM, term);
    signal(SIGKILL, term);
    signal(SIGQUIT, term);

    loop {
        keep_alive();
        sleep(ping_rate);
    }
}

fn main() {
    unsafe {
        let mut args: Vec<*mut c_char> = std::env::args()
            .map(|arg| std::ffi::CString::new(arg).unwrap().into_raw())
            .collect();
        args.push(core::ptr::null_mut());
        main_0((args.len() - 1) as c_int, args.as_mut_ptr());
    }
}
