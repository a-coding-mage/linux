// SPDX-License-Identifier: GPL-2.0-only

use core::ffi::{c_char, c_int, c_ulong, c_void};

const DEVPATH: &[u8] = b"/dev/papr-sysparm\0";

const ENOENT: c_int = 2;
const EPERM: c_int = 1;
const EBADF: c_int = 9;
const EFAULT: c_int = 14;
const EOPNOTSUPP: c_int = 95;

const O_RDONLY: c_int = 0;
const O_RDWR: c_int = 2;

const EXIT_SUCCESS: c_int = 0;
const EXIT_FAILURE: c_int = 1;

const UINT32_MAX: u32 = u32::MAX;

// From <asm/papr-sysparm.h>. Kept local because the C source initializes and
// inspects this ioctl block directly.
#[repr(C)]
struct papr_sysparm_io_block {
    parameter: u32,
    length: u16,
    data: [u8; 4096],
}

impl Default for papr_sysparm_io_block {
    fn default() -> Self {
        Self {
            parameter: 0,
            length: 0,
            data: [0; 4096],
        }
    }
}

struct sysparm_test {
    function: fn() -> c_int,
    description: *const c_char,
}

extern "C" {
    fn open(pathname: *const c_char, flags: c_int, ...) -> c_int;
    fn close(fd: c_int) -> c_int;
    fn ioctl(fd: c_int, request: c_ulong, ...) -> c_int;
    fn test_harness(function: fn() -> c_int, description: *const c_char) -> c_int;

    fn __errno_location() -> *mut c_int;

    // Macro constants supplied by <asm/papr-sysparm.h> in the original C file.
    static PAPR_SYSPARM_IOC_GET: c_ulong;
    static PAPR_SYSPARM_IOC_SET: c_ulong;
}

unsafe fn errno_value() -> c_int {
    *__errno_location()
}

macro_rules! FAIL_IF {
    ($cond:expr) => {
        if $cond {
            return 1;
        }
    };
}

macro_rules! SKIP_IF_MSG {
    ($cond:expr, $msg:expr) => {
        if $cond {
            let _ = $msg;
            return 0;
        }
    };
}

fn open_close() -> c_int {
    let devfd: c_int = unsafe { open(DEVPATH.as_ptr() as *const c_char, O_RDONLY) };

    SKIP_IF_MSG!(
        devfd < 0 && unsafe { errno_value() } == ENOENT,
        b"/dev/papr-sysparm not present\0".as_ptr() as *const c_char
    );

    FAIL_IF!(devfd < 0);
    FAIL_IF!(unsafe { close(devfd) } != 0);

    0
}

fn get_splpar() -> c_int {
    let mut sp = papr_sysparm_io_block {
        parameter: 20, // SPLPAR characteristics
        ..Default::default()
    };
    let devfd: c_int = unsafe { open(DEVPATH.as_ptr() as *const c_char, O_RDONLY) };

    SKIP_IF_MSG!(
        devfd < 0 && unsafe { errno_value() } == ENOENT,
        b"/dev/papr-sysparm not present\0".as_ptr() as *const c_char
    );

    FAIL_IF!(devfd < 0);
    FAIL_IF!(unsafe { ioctl(devfd, PAPR_SYSPARM_IOC_GET, &mut sp as *mut _) } != 0);
    FAIL_IF!(sp.length == 0);
    FAIL_IF!((sp.length as usize) > sp.data.len());
    FAIL_IF!(unsafe { close(devfd) } != 0);

    0
}

fn get_bad_parameter() -> c_int {
    let mut sp = papr_sysparm_io_block {
        parameter: UINT32_MAX, // there are only ~60 specified parameters
        ..Default::default()
    };
    let devfd: c_int = unsafe { open(DEVPATH.as_ptr() as *const c_char, O_RDONLY) };

    SKIP_IF_MSG!(
        devfd < 0 && unsafe { errno_value() } == ENOENT,
        b"/dev/papr-sysparm not present\0".as_ptr() as *const c_char
    );

    FAIL_IF!(devfd < 0);

    // Ensure expected error
    FAIL_IF!(unsafe { ioctl(devfd, PAPR_SYSPARM_IOC_GET, &mut sp as *mut _) } != -1);
    FAIL_IF!(unsafe { errno_value() } != EOPNOTSUPP);

    // Ensure the buffer is unchanged
    FAIL_IF!(sp.length != 0);
    for i in 0..sp.data.len() {
        FAIL_IF!(sp.data[i] != 0);
    }

    FAIL_IF!(unsafe { close(devfd) } != 0);

    0
}

fn check_efault_common(cmd: c_ulong) -> c_int {
    let devfd: c_int = unsafe { open(DEVPATH.as_ptr() as *const c_char, O_RDWR) };

    SKIP_IF_MSG!(
        devfd < 0 && unsafe { errno_value() } == ENOENT,
        b"/dev/papr-sysparm not present\0".as_ptr() as *const c_char
    );

    FAIL_IF!(devfd < 0);

    // Ensure expected error
    FAIL_IF!(unsafe { ioctl(devfd, cmd, core::ptr::null_mut::<c_void>()) } != -1);
    FAIL_IF!(unsafe { errno_value() } != EFAULT);

    FAIL_IF!(unsafe { close(devfd) } != 0);

    0
}

fn check_efault_get() -> c_int {
    unsafe { check_efault_common(PAPR_SYSPARM_IOC_GET) }
}

fn check_efault_set() -> c_int {
    unsafe { check_efault_common(PAPR_SYSPARM_IOC_SET) }
}

fn set_hmc0() -> c_int {
    let mut sp = papr_sysparm_io_block {
        parameter: 0, // HMC0, not a settable parameter
        ..Default::default()
    };
    let devfd: c_int = unsafe { open(DEVPATH.as_ptr() as *const c_char, O_RDWR) };

    SKIP_IF_MSG!(
        devfd < 0 && unsafe { errno_value() } == ENOENT,
        b"/dev/papr-sysparm not present\0".as_ptr() as *const c_char
    );

    FAIL_IF!(devfd < 0);

    // Ensure expected error
    FAIL_IF!(unsafe { ioctl(devfd, PAPR_SYSPARM_IOC_SET, &mut sp as *mut _) } != -1);
    SKIP_IF_MSG!(
        unsafe { errno_value() } == EOPNOTSUPP,
        b"operation not supported\0".as_ptr() as *const c_char
    );
    FAIL_IF!(unsafe { errno_value() } != EPERM);

    FAIL_IF!(unsafe { close(devfd) } != 0);

    0
}

fn set_with_ro_fd() -> c_int {
    let mut sp = papr_sysparm_io_block {
        parameter: 0, // HMC0, not a settable parameter.
        ..Default::default()
    };
    let devfd: c_int = unsafe { open(DEVPATH.as_ptr() as *const c_char, O_RDONLY) };

    SKIP_IF_MSG!(
        devfd < 0 && unsafe { errno_value() } == ENOENT,
        b"/dev/papr-sysparm not present\0".as_ptr() as *const c_char
    );

    FAIL_IF!(devfd < 0);

    // Ensure expected error
    FAIL_IF!(unsafe { ioctl(devfd, PAPR_SYSPARM_IOC_SET, &mut sp as *mut _) } != -1);
    SKIP_IF_MSG!(
        unsafe { errno_value() } == EOPNOTSUPP,
        b"operation not supported\0".as_ptr() as *const c_char
    );

    // HMC0 isn't a settable parameter and we would normally
    // expect to get EPERM on attempts to modify it. However, when
    // the file is open read-only, we expect the driver to prevent
    // the attempt with a distinct error.
    FAIL_IF!(unsafe { errno_value() } != EBADF);

    FAIL_IF!(unsafe { close(devfd) } != 0);

    0
}

static SYSPARM_TESTS: [sysparm_test; 7] = [
    sysparm_test {
        function: open_close,
        description: b"open and close /dev/papr-sysparm without issuing commands\0".as_ptr()
            as *const c_char,
    },
    sysparm_test {
        function: get_splpar,
        description: b"retrieve SPLPAR characteristics\0".as_ptr() as *const c_char,
    },
    sysparm_test {
        function: get_bad_parameter,
        description: b"verify EOPNOTSUPP for known-bad parameter\0".as_ptr() as *const c_char,
    },
    sysparm_test {
        function: check_efault_get,
        description: b"PAPR_SYSPARM_IOC_GET returns EFAULT on bad address\0".as_ptr()
            as *const c_char,
    },
    sysparm_test {
        function: check_efault_set,
        description: b"PAPR_SYSPARM_IOC_SET returns EFAULT on bad address\0".as_ptr()
            as *const c_char,
    },
    sysparm_test {
        function: set_hmc0,
        description: b"ensure EPERM on attempt to update HMC0\0".as_ptr() as *const c_char,
    },
    sysparm_test {
        function: set_with_ro_fd,
        description: b"PAPR_IOC_SYSPARM_SET returns EACCES on read-only fd\0".as_ptr()
            as *const c_char,
    },
];

fn main() -> c_int {
    let mut fails: usize = 0;

    for i in 0..SYSPARM_TESTS.len() {
        let t: &sysparm_test = &SYSPARM_TESTS[i];

        if unsafe { test_harness(t.function, t.description) } != 0 {
            fails += 1;
        }
    }

    if fails == 0 {
        EXIT_SUCCESS
    } else {
        EXIT_FAILURE
    }
}
