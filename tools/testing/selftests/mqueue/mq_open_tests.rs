/*
 * This application is Copyright 2012 Red Hat, Inc.
 *	Doug Ledford <dledford@redhat.com>
 *
 * mq_open_tests is free software: you can redistribute it and/or modify
 * it under the terms of the GNU General Public License as published by
 * the Free Software Foundation, version 3.
 *
 * mq_open_tests is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 * GNU General Public License for more details.
 *
 * For the full text of the license, see <http://www.gnu.org/licenses/>.
 *
 * mq_open_tests.c
 *   Tests the various situations that should either succeed or fail to
 *   open a posix message queue and then reports whether or not they
 *   did as they were supposed to.
 *
 */

use libc::{
    c_char, c_int, c_long, c_void, FILE, mode_t, mq_attr, mqd_t, rlimit, size_t, O_CREAT, O_EXCL,
    O_RDWR, RLIMIT_MSGQUEUE,
};

unsafe extern "C" {
    fn fprintf(stream: *mut FILE, format: *const c_char, ...) -> c_int;
    fn fscanf(stream: *mut FILE, format: *const c_char, ...) -> c_int;
    fn printf(format: *const c_char, ...) -> c_int;
    fn perror(s: *const c_char);
    fn fopen(pathname: *const c_char, mode: *const c_char) -> *mut FILE;
    fn rewind(stream: *mut FILE);
    fn getrlimit(resource: c_int, rlim: *mut rlimit) -> c_int;
    fn setrlimit(resource: c_int, rlim: *const rlimit) -> c_int;
    fn getuid() -> libc::uid_t;
    fn seteuid(euid: libc::uid_t) -> c_int;
    fn malloc(size: size_t) -> *mut c_void;
    fn strdup(s: *const c_char) -> *mut c_char;
    fn strlen(s: *const c_char) -> size_t;
    fn strcat(dest: *mut c_char, src: *const c_char) -> *mut c_char;
    fn exit(status: c_int) -> !;
    fn mq_open(name: *const c_char, oflag: c_int, ...) -> mqd_t;
    fn mq_getattr(mqdes: mqd_t, mqstat: *mut mq_attr) -> c_int;
    fn mq_close(mqdes: mqd_t) -> c_int;
    fn mq_unlink(name: *const c_char) -> c_int;
    fn error(status: c_int, errnum: c_int, format: *const c_char, ...);
    fn __errno_location() -> *mut c_int;
    fn ksft_exit_skip(msg: *const c_char, ...) -> !;
}

static USAGE: *const c_char = b"Usage:\n  %s path\n\n\tpath\tPath name of the message queue to create\n\n\tNote: this program must be run as root in order to enable all tests\n\n\0".as_ptr() as *const c_char;

static DEF_MSGS: *const c_char = b"/proc/sys/fs/mqueue/msg_default\0".as_ptr() as *const c_char;
static DEF_MSGSIZE: *const c_char =
    b"/proc/sys/fs/mqueue/msgsize_default\0".as_ptr() as *const c_char;
static MAX_MSGS: *const c_char = b"/proc/sys/fs/mqueue/msg_max\0".as_ptr() as *const c_char;
static MAX_MSGSIZE: *const c_char = b"/proc/sys/fs/mqueue/msgsize_max\0".as_ptr() as *const c_char;

static mut DEFAULT_SETTINGS: c_int = 0;
static mut SAVED_LIMITS: rlimit = rlimit {
    rlim_cur: 0,
    rlim_max: 0,
};
static mut CUR_LIMITS: rlimit = rlimit {
    rlim_cur: 0,
    rlim_max: 0,
};
static mut SAVED_DEF_MSGS: c_int = 0;
static mut SAVED_DEF_MSGSIZE: c_int = 0;
static mut SAVED_MAX_MSGS: c_int = 0;
static mut SAVED_MAX_MSGSIZE: c_int = 0;
static mut CUR_DEF_MSGS: c_int = 0;
static mut CUR_DEF_MSGSIZE: c_int = 0;
static mut CUR_MAX_MSGS: c_int = 0;
static mut CUR_MAX_MSGSIZE: c_int = 0;
static mut DEF_MSGS_FILE: *mut FILE = core::ptr::null_mut();
static mut DEF_MSGSIZE_FILE: *mut FILE = core::ptr::null_mut();
static mut MAX_MSGS_FILE: *mut FILE = core::ptr::null_mut();
static mut MAX_MSGSIZE_FILE: *mut FILE = core::ptr::null_mut();
static mut QUEUE_PATH: *mut c_char = core::ptr::null_mut();
static DEFAULT_QUEUE_PATH: *const c_char = b"/test1\0".as_ptr() as *const c_char;
static mut QUEUE: mqd_t = -1;

const DEFFILEMODE: mode_t = 0o666;

unsafe fn __set(stream: *mut FILE, value: c_int, err_msg: *const c_char) {
    rewind(stream);
    if fprintf(stream, b"%d\0".as_ptr() as *const c_char, value) < 0 {
        perror(err_msg);
    }
}

unsafe fn shutdown(exit_val: c_int, err_cause: *const c_char, line_no: c_int) {
    static mut IN_SHUTDOWN: c_int = 0;

    /* In case we get called recursively by a set() call below */
    let old_in_shutdown = IN_SHUTDOWN;
    IN_SHUTDOWN += 1;
    if old_in_shutdown != 0 {
        return;
    }

    if seteuid(0) == -1 {
        perror(b"seteuid() failed\0".as_ptr() as *const c_char);
    }

    if QUEUE != -1 {
        if mq_close(QUEUE) != 0 {
            perror(b"mq_close() during shutdown\0".as_ptr() as *const c_char);
        }
    }
    if !QUEUE_PATH.is_null() {
        /*
         * Be silent if this fails, if we cleaned up already it's
         * expected to fail
         */
        mq_unlink(QUEUE_PATH);
    }
    if DEFAULT_SETTINGS != 0 {
        if SAVED_DEF_MSGS != 0 {
            __set(
                DEF_MSGS_FILE,
                SAVED_DEF_MSGS,
                b"failed to restore saved_def_msgs\0".as_ptr() as *const c_char,
            );
        }
        if SAVED_DEF_MSGSIZE != 0 {
            __set(
                DEF_MSGSIZE_FILE,
                SAVED_DEF_MSGSIZE,
                b"failed to restore saved_def_msgsize\0".as_ptr() as *const c_char,
            );
        }
    }
    if SAVED_MAX_MSGS != 0 {
        __set(
            MAX_MSGS_FILE,
            SAVED_MAX_MSGS,
            b"failed to restore saved_max_msgs\0".as_ptr() as *const c_char,
        );
    }
    if SAVED_MAX_MSGSIZE != 0 {
        __set(
            MAX_MSGSIZE_FILE,
            SAVED_MAX_MSGSIZE,
            b"failed to restore saved_max_msgsize\0".as_ptr() as *const c_char,
        );
    }
    if exit_val != 0 {
        error(
            exit_val,
            *__errno_location(),
            b"%s at %d\0".as_ptr() as *const c_char,
            err_cause,
            line_no,
        );
    }
    exit(0);
}

unsafe fn get(stream: *mut FILE) -> c_int {
    let mut value: c_int = 0;
    rewind(stream);
    if fscanf(
        stream,
        b"%d\0".as_ptr() as *const c_char,
        &mut value as *mut c_int,
    ) != 1
    {
        shutdown(
            4,
            b"Error reading /proc entry\0".as_ptr() as *const c_char,
            line!() as c_int - 1,
        );
    }
    value
}

unsafe fn set(stream: *mut FILE, value: c_int) {
    let new_value: c_int;

    rewind(stream);
    if fprintf(stream, b"%d\0".as_ptr() as *const c_char, value) < 0 {
        return shutdown(
            5,
            b"Failed writing to /proc file\0".as_ptr() as *const c_char,
            line!() as c_int - 1,
        );
    }
    new_value = get(stream);
    if new_value != value {
        return shutdown(
            5,
            b"We didn't get what we wrote to /proc back\0".as_ptr() as *const c_char,
            line!() as c_int - 1,
        );
    }
}

unsafe fn getr(type_: c_int, rlim: *mut rlimit) {
    if getrlimit(type_, rlim) != 0 {
        shutdown(6, b"getrlimit()\0".as_ptr() as *const c_char, line!() as c_int - 1);
    }
}

unsafe fn setr(type_: c_int, rlim: *mut rlimit) {
    if setrlimit(type_, rlim) != 0 {
        shutdown(7, b"setrlimit()\0".as_ptr() as *const c_char, line!() as c_int - 1);
    }
}

unsafe fn validate_current_settings() {
    let mut rlim_needed: c_int;

    if CUR_LIMITS.rlim_cur < 4096 {
        printf(
            b"Current rlimit value for POSIX message queue bytes is unreasonably low,\nincreasing.\n\n\0"
                .as_ptr() as *const c_char,
        );
        CUR_LIMITS.rlim_cur = 8192;
        CUR_LIMITS.rlim_max = 16384;
        setr(RLIMIT_MSGQUEUE as c_int, &mut CUR_LIMITS as *mut rlimit);
    }

    if DEFAULT_SETTINGS != 0 {
        rlim_needed =
            (CUR_DEF_MSGS + 1) * (CUR_DEF_MSGSIZE + 1 + 2 * core::mem::size_of::<*mut c_void>() as c_int);
        if (rlim_needed as libc::rlim_t) > CUR_LIMITS.rlim_cur {
            printf(
                b"Temporarily lowering default queue parameters to something that will work\nwith the current rlimit values.\n\n\0"
                    .as_ptr() as *const c_char,
            );
            set(DEF_MSGS_FILE, 10);
            CUR_DEF_MSGS = 10;
            set(DEF_MSGSIZE_FILE, 128);
            CUR_DEF_MSGSIZE = 128;
        }
    } else {
        rlim_needed =
            (CUR_MAX_MSGS + 1) * (CUR_MAX_MSGSIZE + 1 + 2 * core::mem::size_of::<*mut c_void>() as c_int);
        if (rlim_needed as libc::rlim_t) > CUR_LIMITS.rlim_cur {
            printf(
                b"Temporarily lowering maximum queue parameters to something that will work\nwith the current rlimit values in case this is a kernel that ties the default\nqueue parameters to the maximum queue parameters.\n\n\0"
                    .as_ptr() as *const c_char,
            );
            set(MAX_MSGS_FILE, 10);
            CUR_MAX_MSGS = 10;
            set(MAX_MSGSIZE_FILE, 128);
            CUR_MAX_MSGSIZE = 128;
        }
    }
}

/*
 * test_queue - Test opening a queue, shutdown if we fail.  This should
 * only be called in situations that should never fail.  We clean up
 * after ourselves and return the queue attributes in *result.
 */
unsafe fn test_queue(attr: *mut mq_attr, result: *mut mq_attr) {
    let flags: c_int = O_RDWR | O_EXCL | O_CREAT;
    let perms: c_int = DEFFILEMODE as c_int;

    QUEUE = mq_open(QUEUE_PATH, flags, perms, attr);
    if QUEUE == -1 {
        shutdown(1, b"mq_open()\0".as_ptr() as *const c_char, line!() as c_int);
    }
    if mq_getattr(QUEUE, result) != 0 {
        shutdown(1, b"mq_getattr()\0".as_ptr() as *const c_char, line!() as c_int);
    }
    if mq_close(QUEUE) != 0 {
        shutdown(1, b"mq_close()\0".as_ptr() as *const c_char, line!() as c_int);
    }
    QUEUE = -1;
    if mq_unlink(QUEUE_PATH) != 0 {
        shutdown(1, b"mq_unlink()\0".as_ptr() as *const c_char, line!() as c_int);
    }
}

/*
 * Same as test_queue above, but failure is not fatal.
 * Returns:
 * 0 - Failed to create a queue
 * 1 - Created a queue, attributes in *result
 */
unsafe fn test_queue_fail(attr: *mut mq_attr, result: *mut mq_attr) -> c_int {
    let flags: c_int = O_RDWR | O_EXCL | O_CREAT;
    let perms: c_int = DEFFILEMODE as c_int;

    QUEUE = mq_open(QUEUE_PATH, flags, perms, attr);
    if QUEUE == -1 {
        return 0;
    }
    if mq_getattr(QUEUE, result) != 0 {
        shutdown(1, b"mq_getattr()\0".as_ptr() as *const c_char, line!() as c_int);
    }
    if mq_close(QUEUE) != 0 {
        shutdown(1, b"mq_close()\0".as_ptr() as *const c_char, line!() as c_int);
    }
    QUEUE = -1;
    if mq_unlink(QUEUE_PATH) != 0 {
        shutdown(1, b"mq_unlink()\0".as_ptr() as *const c_char, line!() as c_int);
    }
    1
}

unsafe fn c_main(argc: c_int, argv: *mut *mut c_char) -> c_int {
    let mut attr: mq_attr = core::mem::zeroed();
    let mut result: mq_attr = core::mem::zeroed();

    if argc != 2 {
        printf(
            b"Using Default queue path - %s\n\0".as_ptr() as *const c_char,
            DEFAULT_QUEUE_PATH,
        );
        QUEUE_PATH = DEFAULT_QUEUE_PATH as *mut c_char;
    } else {
        /*
         * Although we can create a msg queue with a non-absolute path name,
         * unlink will fail.  So, if the name doesn't start with a /, add one
         * when we save it.
         */
        if *(*argv.add(1)) == b'/' as c_char {
            QUEUE_PATH = strdup(*argv.add(1));
        } else {
            QUEUE_PATH = malloc(strlen(*argv.add(1)) + 2) as *mut c_char;
            if QUEUE_PATH.is_null() {
                perror(b"malloc()\0".as_ptr() as *const c_char);
                exit(1);
            }
            *QUEUE_PATH.add(0) = b'/' as c_char;
            *QUEUE_PATH.add(1) = 0;
            strcat(QUEUE_PATH, *argv.add(1));
        }
    }

    if getuid() != 0 {
        ksft_exit_skip(
            b"Not running as root, but almost all tests require root in order to modify\nsystem settings.  Exiting.\n\0"
                .as_ptr() as *const c_char,
        );
    }

    /* Find out what files there are for us to make tweaks in */
    DEF_MSGS_FILE = fopen(DEF_MSGS, b"r+\0".as_ptr() as *const c_char);
    DEF_MSGSIZE_FILE = fopen(DEF_MSGSIZE, b"r+\0".as_ptr() as *const c_char);
    MAX_MSGS_FILE = fopen(MAX_MSGS, b"r+\0".as_ptr() as *const c_char);
    MAX_MSGSIZE_FILE = fopen(MAX_MSGSIZE, b"r+\0".as_ptr() as *const c_char);

    if MAX_MSGS_FILE.is_null() {
        shutdown(2, b"Failed to open msg_max\0".as_ptr() as *const c_char, line!() as c_int);
    }
    if MAX_MSGSIZE_FILE.is_null() {
        shutdown(2, b"Failed to open msgsize_max\0".as_ptr() as *const c_char, line!() as c_int);
    }
    if !DEF_MSGS_FILE.is_null() || !DEF_MSGSIZE_FILE.is_null() {
        DEFAULT_SETTINGS = 1;
    }

    /* Load up the current system values for everything we can */
    getr(RLIMIT_MSGQUEUE as c_int, &mut SAVED_LIMITS as *mut rlimit);
    CUR_LIMITS = SAVED_LIMITS;
    if DEFAULT_SETTINGS != 0 {
        CUR_DEF_MSGS = get(DEF_MSGS_FILE);
        SAVED_DEF_MSGS = CUR_DEF_MSGS;
        CUR_DEF_MSGSIZE = get(DEF_MSGSIZE_FILE);
        SAVED_DEF_MSGSIZE = CUR_DEF_MSGSIZE;
    }
    CUR_MAX_MSGS = get(MAX_MSGS_FILE);
    SAVED_MAX_MSGS = CUR_MAX_MSGS;
    CUR_MAX_MSGSIZE = get(MAX_MSGSIZE_FILE);
    SAVED_MAX_MSGSIZE = CUR_MAX_MSGSIZE;

    /* Tell the user our initial state */
    printf(b"\nInitial system state:\n\0".as_ptr() as *const c_char);
    printf(
        b"\tUsing queue path:\t\t%s\n\0".as_ptr() as *const c_char,
        QUEUE_PATH,
    );
    printf(
        b"\tRLIMIT_MSGQUEUE(soft):\t\t%ld\n\0".as_ptr() as *const c_char,
        SAVED_LIMITS.rlim_cur as c_long,
    );
    printf(
        b"\tRLIMIT_MSGQUEUE(hard):\t\t%ld\n\0".as_ptr() as *const c_char,
        SAVED_LIMITS.rlim_max as c_long,
    );
    printf(
        b"\tMaximum Message Size:\t\t%d\n\0".as_ptr() as *const c_char,
        SAVED_MAX_MSGSIZE,
    );
    printf(
        b"\tMaximum Queue Size:\t\t%d\n\0".as_ptr() as *const c_char,
        SAVED_MAX_MSGS,
    );
    if DEFAULT_SETTINGS != 0 {
        printf(
            b"\tDefault Message Size:\t\t%d\n\0".as_ptr() as *const c_char,
            SAVED_DEF_MSGSIZE,
        );
        printf(
            b"\tDefault Queue Size:\t\t%d\n\0".as_ptr() as *const c_char,
            SAVED_DEF_MSGS,
        );
    } else {
        printf(b"\tDefault Message Size:\t\tNot Supported\n\0".as_ptr() as *const c_char);
        printf(b"\tDefault Queue Size:\t\tNot Supported\n\0".as_ptr() as *const c_char);
    }
    printf(b"\n\0".as_ptr() as *const c_char);

    validate_current_settings();

    printf(b"Adjusted system state for testing:\n\0".as_ptr() as *const c_char);
    printf(
        b"\tRLIMIT_MSGQUEUE(soft):\t\t%ld\n\0".as_ptr() as *const c_char,
        CUR_LIMITS.rlim_cur as c_long,
    );
    printf(
        b"\tRLIMIT_MSGQUEUE(hard):\t\t%ld\n\0".as_ptr() as *const c_char,
        CUR_LIMITS.rlim_max as c_long,
    );
    printf(
        b"\tMaximum Message Size:\t\t%d\n\0".as_ptr() as *const c_char,
        CUR_MAX_MSGSIZE,
    );
    printf(
        b"\tMaximum Queue Size:\t\t%d\n\0".as_ptr() as *const c_char,
        CUR_MAX_MSGS,
    );
    if DEFAULT_SETTINGS != 0 {
        printf(
            b"\tDefault Message Size:\t\t%d\n\0".as_ptr() as *const c_char,
            CUR_DEF_MSGSIZE,
        );
        printf(
            b"\tDefault Queue Size:\t\t%d\n\0".as_ptr() as *const c_char,
            CUR_DEF_MSGS,
        );
    }

    printf(
        b"\n\nTest series 1, behavior when no attr struct passed to mq_open:\n\0".as_ptr()
            as *const c_char,
    );
    if DEFAULT_SETTINGS == 0 {
        test_queue(core::ptr::null_mut(), &mut result as *mut mq_attr);
        printf(
            b"Given sane system settings, mq_open without an attr struct succeeds:\tPASS\n\0"
                .as_ptr() as *const c_char,
        );
        if result.mq_maxmsg != CUR_MAX_MSGS as libc::c_long
            || result.mq_msgsize != CUR_MAX_MSGSIZE as libc::c_long
        {
            printf(
                b"Kernel does not support setting the default mq attributes,\nbut also doesn't tie the defaults to the maximums:\t\t\tPASS\n\0"
                    .as_ptr() as *const c_char,
            );
        } else {
            CUR_MAX_MSGS += 1;
            set(MAX_MSGS_FILE, CUR_MAX_MSGS);
            CUR_MAX_MSGSIZE += 1;
            set(MAX_MSGSIZE_FILE, CUR_MAX_MSGSIZE);
            test_queue(core::ptr::null_mut(), &mut result as *mut mq_attr);
            if result.mq_maxmsg == CUR_MAX_MSGS as libc::c_long
                && result.mq_msgsize == CUR_MAX_MSGSIZE as libc::c_long
            {
                printf(
                    b"Kernel does not support setting the default mq attributes and\nalso ties system wide defaults to the system wide maximums:\t\tFAIL\n\0"
                        .as_ptr() as *const c_char,
                );
            } else {
                printf(
                    b"Kernel does not support setting the default mq attributes,\nbut also doesn't tie the defaults to the maximums:\t\t\tPASS\n\0"
                        .as_ptr() as *const c_char,
                );
            }
        }
    } else {
        printf(
            b"Kernel supports setting defaults separately from maximums:\t\tPASS\n\0"
                .as_ptr() as *const c_char,
        );
        /*
         * While we are here, go ahead and test that the kernel
         * properly follows the default settings
         */
        test_queue(core::ptr::null_mut(), &mut result as *mut mq_attr);
        printf(
            b"Given sane values, mq_open without an attr struct succeeds:\t\tPASS\n\0"
                .as_ptr() as *const c_char,
        );
        if result.mq_maxmsg != CUR_DEF_MSGS as libc::c_long
            || result.mq_msgsize != CUR_DEF_MSGSIZE as libc::c_long
        {
            printf(
                b"Kernel supports setting defaults, but does not actually honor them:\tFAIL\n\n\0"
                    .as_ptr() as *const c_char,
            );
        } else {
            CUR_DEF_MSGS += 1;
            set(DEF_MSGS_FILE, CUR_DEF_MSGS);
            CUR_DEF_MSGSIZE += 1;
            set(DEF_MSGSIZE_FILE, CUR_DEF_MSGSIZE);
            /* In case max was the same as the default */
            CUR_MAX_MSGS += 1;
            set(MAX_MSGS_FILE, CUR_MAX_MSGS);
            CUR_MAX_MSGSIZE += 1;
            set(MAX_MSGSIZE_FILE, CUR_MAX_MSGSIZE);
            test_queue(core::ptr::null_mut(), &mut result as *mut mq_attr);
            if result.mq_maxmsg != CUR_DEF_MSGS as libc::c_long
                || result.mq_msgsize != CUR_DEF_MSGSIZE as libc::c_long
            {
                printf(
                    b"Kernel supports setting defaults, but does not actually honor them:\tFAIL\n\0"
                        .as_ptr() as *const c_char,
                );
            } else {
                printf(
                    b"Kernel properly honors default setting knobs:\t\t\t\tPASS\n\0"
                        .as_ptr() as *const c_char,
                );
            }
        }
        set(DEF_MSGS_FILE, CUR_MAX_MSGS + 1);
        CUR_DEF_MSGS = CUR_MAX_MSGS + 1;
        set(DEF_MSGSIZE_FILE, CUR_MAX_MSGSIZE + 1);
        CUR_DEF_MSGSIZE = CUR_MAX_MSGSIZE + 1;
        if (CUR_DEF_MSGS * (CUR_DEF_MSGSIZE + 2 * core::mem::size_of::<*mut c_void>() as c_int))
            as libc::rlim_t
            >= CUR_LIMITS.rlim_cur
        {
            CUR_LIMITS.rlim_cur = ((CUR_DEF_MSGS + 2)
                * (CUR_DEF_MSGSIZE + 2 * core::mem::size_of::<*mut c_void>() as c_int))
                as libc::rlim_t;
            CUR_LIMITS.rlim_max = 2 * CUR_LIMITS.rlim_cur;
            setr(RLIMIT_MSGQUEUE as c_int, &mut CUR_LIMITS as *mut rlimit);
        }
        if test_queue_fail(core::ptr::null_mut(), &mut result as *mut mq_attr) != 0 {
            if result.mq_maxmsg == CUR_MAX_MSGS as libc::c_long
                && result.mq_msgsize == CUR_MAX_MSGSIZE as libc::c_long
            {
                printf(
                    b"Kernel properly limits default values to lesser of default/max:\t\tPASS\n\0"
                        .as_ptr() as *const c_char,
                );
            } else {
                printf(
                    b"Kernel does not properly set default queue parameters when\ndefaults > max:\t\t\t\t\t\t\t\tFAIL\n\0"
                        .as_ptr() as *const c_char,
                );
            }
        } else {
            printf(
                b"Kernel fails to open mq because defaults are greater than maximums:\tFAIL\n\0"
                    .as_ptr() as *const c_char,
            );
        }
        CUR_DEF_MSGS -= 1;
        set(DEF_MSGS_FILE, CUR_DEF_MSGS);
        CUR_DEF_MSGSIZE -= 1;
        set(DEF_MSGSIZE_FILE, CUR_DEF_MSGSIZE);
        CUR_LIMITS.rlim_cur = (CUR_DEF_MSGS * CUR_DEF_MSGSIZE) as libc::rlim_t;
        CUR_LIMITS.rlim_max = CUR_LIMITS.rlim_cur;
        setr(RLIMIT_MSGQUEUE as c_int, &mut CUR_LIMITS as *mut rlimit);
        if test_queue_fail(core::ptr::null_mut(), &mut result as *mut mq_attr) != 0 {
            printf(
                b"Kernel creates queue even though defaults would exceed\nrlimit setting:\t\t\t\t\t\t\t\tFAIL\n\0"
                    .as_ptr() as *const c_char,
            );
        } else {
            printf(
                b"Kernel properly fails to create queue when defaults would\nexceed rlimit:\t\t\t\t\t\t\t\tPASS\n\0"
                    .as_ptr() as *const c_char,
            );
        }
    }

    /*
     * Test #2 - open with an attr struct that exceeds rlimit
     */
    printf(
        b"\n\nTest series 2, behavior when attr struct is passed to mq_open:\n\0".as_ptr()
            as *const c_char,
    );
    CUR_MAX_MSGS = 32;
    CUR_MAX_MSGSIZE = (CUR_LIMITS.rlim_max >> 4) as c_int;
    set(MAX_MSGS_FILE, CUR_MAX_MSGS);
    set(MAX_MSGSIZE_FILE, CUR_MAX_MSGSIZE);
    attr.mq_maxmsg = CUR_MAX_MSGS as libc::c_long;
    attr.mq_msgsize = CUR_MAX_MSGSIZE as libc::c_long;
    if test_queue_fail(&mut attr as *mut mq_attr, &mut result as *mut mq_attr) != 0 {
        printf(
            b"Queue open in excess of rlimit max when euid = 0 succeeded:\t\tFAIL\n\0"
                .as_ptr() as *const c_char,
        );
    } else {
        printf(
            b"Queue open in excess of rlimit max when euid = 0 failed:\t\tPASS\n\0"
                .as_ptr() as *const c_char,
        );
    }
    attr.mq_maxmsg = (CUR_MAX_MSGS + 1) as libc::c_long;
    attr.mq_msgsize = 10;
    if test_queue_fail(&mut attr as *mut mq_attr, &mut result as *mut mq_attr) != 0 {
        printf(
            b"Queue open with mq_maxmsg > limit when euid = 0 succeeded:\t\tPASS\n\0"
                .as_ptr() as *const c_char,
        );
    } else {
        printf(
            b"Queue open with mq_maxmsg > limit when euid = 0 failed:\t\tFAIL\n\0"
                .as_ptr() as *const c_char,
        );
    }
    attr.mq_maxmsg = 1;
    attr.mq_msgsize = (CUR_MAX_MSGSIZE + 1) as libc::c_long;
    if test_queue_fail(&mut attr as *mut mq_attr, &mut result as *mut mq_attr) != 0 {
        printf(
            b"Queue open with mq_msgsize > limit when euid = 0 succeeded:\t\tPASS\n\0"
                .as_ptr() as *const c_char,
        );
    } else {
        printf(
            b"Queue open with mq_msgsize > limit when euid = 0 failed:\t\tFAIL\n\0"
                .as_ptr() as *const c_char,
        );
    }
    attr.mq_maxmsg = 65536;
    attr.mq_msgsize = 65536;
    if test_queue_fail(&mut attr as *mut mq_attr, &mut result as *mut mq_attr) != 0 {
        printf(
            b"Queue open with total size > 2GB when euid = 0 succeeded:\t\tFAIL\n\0"
                .as_ptr() as *const c_char,
        );
    } else {
        printf(
            b"Queue open with total size > 2GB when euid = 0 failed:\t\t\tPASS\n\0"
                .as_ptr() as *const c_char,
        );
    }

    if seteuid(99) == -1 {
        perror(b"seteuid() failed\0".as_ptr() as *const c_char);
        exit(1);
    }

    attr.mq_maxmsg = CUR_MAX_MSGS as libc::c_long;
    attr.mq_msgsize = CUR_MAX_MSGSIZE as libc::c_long;
    if test_queue_fail(&mut attr as *mut mq_attr, &mut result as *mut mq_attr) != 0 {
        printf(
            b"Queue open in excess of rlimit max when euid = 99 succeeded:\t\tFAIL\n\0"
                .as_ptr() as *const c_char,
        );
    } else {
        printf(
            b"Queue open in excess of rlimit max when euid = 99 failed:\t\tPASS\n\0"
                .as_ptr() as *const c_char,
        );
    }
    attr.mq_maxmsg = (CUR_MAX_MSGS + 1) as libc::c_long;
    attr.mq_msgsize = 10;
    if test_queue_fail(&mut attr as *mut mq_attr, &mut result as *mut mq_attr) != 0 {
        printf(
            b"Queue open with mq_maxmsg > limit when euid = 99 succeeded:\t\tFAIL\n\0"
                .as_ptr() as *const c_char,
        );
    } else {
        printf(
            b"Queue open with mq_maxmsg > limit when euid = 99 failed:\t\tPASS\n\0"
                .as_ptr() as *const c_char,
        );
    }
    attr.mq_maxmsg = 1;
    attr.mq_msgsize = (CUR_MAX_MSGSIZE + 1) as libc::c_long;
    if test_queue_fail(&mut attr as *mut mq_attr, &mut result as *mut mq_attr) != 0 {
        printf(
            b"Queue open with mq_msgsize > limit when euid = 99 succeeded:\t\tFAIL\n\0"
                .as_ptr() as *const c_char,
        );
    } else {
        printf(
            b"Queue open with mq_msgsize > limit when euid = 99 failed:\t\tPASS\n\0"
                .as_ptr() as *const c_char,
        );
    }
    attr.mq_maxmsg = 65536;
    attr.mq_msgsize = 65536;
    if test_queue_fail(&mut attr as *mut mq_attr, &mut result as *mut mq_attr) != 0 {
        printf(
            b"Queue open with total size > 2GB when euid = 99 succeeded:\t\tFAIL\n\0"
                .as_ptr() as *const c_char,
        );
    } else {
        printf(
            b"Queue open with total size > 2GB when euid = 99 failed:\t\t\tPASS\n\0"
                .as_ptr() as *const c_char,
        );
    }

    shutdown(0, b"\0".as_ptr() as *const c_char, 0);
    0
}

fn main() {
    unsafe {
        let mut args: Vec<*mut c_char> = std::env::args()
            .map(|arg| std::ffi::CString::new(arg).unwrap().into_raw())
            .collect();
        args.push(core::ptr::null_mut());
        c_main((args.len() - 1) as c_int, args.as_mut_ptr());
    }
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
