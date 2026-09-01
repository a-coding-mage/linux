// SPDX-License-Identifier: GPL-2.0-only
// C dependencies: errno.h, fcntl.h, stdlib.h, string.h, sys/ioctl.h, unistd.h,
// asm/papr-vpd.h, and "utils.h".

use core::ffi::{c_char, c_int, c_ulong, c_void};
use core::mem;
use core::ptr;

const DEVPATH: &[u8] = b"/dev/papr-vpd\0";

extern "C" {
    static mut errno: c_int;

    fn open(pathname: *const c_char, flags: c_int, ...) -> c_int;
    fn close(fd: c_int) -> c_int;
    fn ioctl(fd: c_int, request: c_ulong, ...) -> c_int;
    fn lseek(fd: c_int, offset: off_t, whence: c_int) -> off_t;
    fn malloc(size: size_t) -> *mut c_void;
    fn free(ptr: *mut c_void);
    fn pread(fd: c_int, buf: *mut c_void, count: size_t, offset: off_t) -> ssize_t;
    fn read(fd: c_int, buf: *mut c_void, count: size_t) -> ssize_t;
    fn memset(s: *mut c_void, c: c_int, n: size_t) -> *mut c_void;
    fn memmem(
        haystack: *const c_void,
        haystacklen: size_t,
        needle: *const c_void,
        needlelen: size_t,
    ) -> *mut c_void;
    fn strlen(s: *const c_char) -> size_t;
    fn memcmp(s1: *const c_void, s2: *const c_void, n: size_t) -> c_int;
    fn strchr(s: *const c_char, c: c_int) -> *mut c_char;
    fn snprintf(s: *mut c_char, maxlen: size_t, format: *const c_char, ...) -> c_int;
    fn sscanf(s: *const c_char, format: *const c_char, ...) -> c_int;

    fn read_file_alloc(path: *const c_char, buf: *mut *mut c_char, size: *mut size_t) -> c_int;
    fn test_harness(function: Option<unsafe extern "C" fn() -> c_int>, description: *const c_char) -> c_int;
}

type size_t = usize;
type ssize_t = isize;
type off_t = i64;

const O_RDONLY: c_int = 0;
const SEEK_END: c_int = 2;
const ENOENT: c_int = 2;
const EINVAL: c_int = 22;
const EFAULT: c_int = 14;
const EXIT_SUCCESS: c_int = 0;
const EXIT_FAILURE: c_int = 1;

// From <asm/papr-vpd.h>.
const PAPR_VPD_IOC_CREATE_HANDLE: c_ulong = 0;

#[repr(C)]
struct papr_location_code {
    str_: [c_char; 80],
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
            return 0;
        }
    };
}

unsafe extern "C" fn dev_papr_vpd_open_close() -> c_int {
    let devfd: c_int = open(DEVPATH.as_ptr() as *const c_char, O_RDONLY);

    SKIP_IF_MSG!(
        devfd < 0 && errno == ENOENT,
        b"/dev/papr-vpd not present\0".as_ptr() as *const c_char
    );

    FAIL_IF!(devfd < 0);
    FAIL_IF!(close(devfd) != 0);

    0
}

unsafe extern "C" fn dev_papr_vpd_get_handle_all() -> c_int {
    let devfd: c_int = open(DEVPATH.as_ptr() as *const c_char, O_RDONLY);
    let mut lc = papr_location_code { str_: [0; 80] };
    let mut size: off_t;
    let fd: c_int;

    SKIP_IF_MSG!(
        devfd < 0 && errno == ENOENT,
        b"/dev/papr-vpd not present\0".as_ptr() as *const c_char
    );

    FAIL_IF!(devfd < 0);

    errno = 0;
    fd = ioctl(devfd, PAPR_VPD_IOC_CREATE_HANDLE, &mut lc as *mut papr_location_code);
    FAIL_IF!(errno != 0);
    FAIL_IF!(fd < 0);

    FAIL_IF!(close(devfd) != 0);

    size = lseek(fd, 0, SEEK_END);
    FAIL_IF!(size <= 0);

    let buf: *mut c_void = malloc(size as size_t);
    FAIL_IF!(buf.is_null());

    let consumed: ssize_t = pread(fd, buf, size as size_t, 0);
    FAIL_IF!(consumed != size as ssize_t);

    /* Ensure EOF */
    FAIL_IF!(read(fd, buf, size as size_t) != 0);
    FAIL_IF!(close(fd) != 0);

    /* Verify that the buffer looks like VPD */
    static NEEDLE: &[u8] = b"System VPD\0";
    FAIL_IF!(memmem(
        buf,
        size as size_t,
        NEEDLE.as_ptr() as *const c_void,
        strlen(NEEDLE.as_ptr() as *const c_char),
    )
    .is_null());

    0
}

unsafe extern "C" fn dev_papr_vpd_get_handle_byte_at_a_time() -> c_int {
    let devfd: c_int = open(DEVPATH.as_ptr() as *const c_char, O_RDONLY);
    let mut lc = papr_location_code { str_: [0; 80] };
    let fd: c_int;

    SKIP_IF_MSG!(
        devfd < 0 && errno == ENOENT,
        b"/dev/papr-vpd not present\0".as_ptr() as *const c_char
    );

    FAIL_IF!(devfd < 0);

    errno = 0;
    fd = ioctl(devfd, PAPR_VPD_IOC_CREATE_HANDLE, &mut lc as *mut papr_location_code);
    FAIL_IF!(errno != 0);
    FAIL_IF!(fd < 0);

    FAIL_IF!(close(devfd) != 0);

    let mut consumed: size_t = 0;
    loop {
        let res: ssize_t;
        let mut c: c_char = 0;

        errno = 0;
        res = read(
            fd,
            &mut c as *mut c_char as *mut c_void,
            mem::size_of_val(&c),
        );
        FAIL_IF!(res > mem::size_of_val(&c) as ssize_t);
        FAIL_IF!(res < 0);
        FAIL_IF!(errno != 0);
        consumed = consumed.wrapping_add(res as size_t);
        if res == 0 {
            break;
        }
    }

    FAIL_IF!(consumed != lseek(fd, 0, SEEK_END) as size_t);

    FAIL_IF!(close(fd) != 0);

    0
}

unsafe extern "C" fn dev_papr_vpd_unterm_loc_code() -> c_int {
    let devfd: c_int = open(DEVPATH.as_ptr() as *const c_char, O_RDONLY);
    let mut lc = papr_location_code { str_: [0; 80] };
    let fd: c_int;

    SKIP_IF_MSG!(
        devfd < 0 && errno == ENOENT,
        b"/dev/papr-vpd not present\0".as_ptr() as *const c_char
    );

    FAIL_IF!(devfd < 0);

    /*
     * Place a non-null byte in every element of loc_code; the
     * driver should reject this input.
     */
    memset(
        lc.str_.as_mut_ptr() as *mut c_void,
        b'x' as c_int,
        lc.str_.len(),
    );

    errno = 0;
    fd = ioctl(devfd, PAPR_VPD_IOC_CREATE_HANDLE, &mut lc as *mut papr_location_code);
    FAIL_IF!(fd != -1);
    FAIL_IF!(errno != EINVAL);

    FAIL_IF!(close(devfd) != 0);
    0
}

unsafe extern "C" fn dev_papr_vpd_null_handle() -> c_int {
    let devfd: c_int = open(DEVPATH.as_ptr() as *const c_char, O_RDONLY);
    let rc: c_int;

    SKIP_IF_MSG!(
        devfd < 0 && errno == ENOENT,
        b"/dev/papr-vpd not present\0".as_ptr() as *const c_char
    );

    FAIL_IF!(devfd < 0);

    errno = 0;
    rc = ioctl(devfd, PAPR_VPD_IOC_CREATE_HANDLE, ptr::null_mut::<c_void>());
    FAIL_IF!(rc != -1);
    FAIL_IF!(errno != EFAULT);

    FAIL_IF!(close(devfd) != 0);
    0
}

unsafe extern "C" fn papr_vpd_close_handle_without_reading() -> c_int {
    let devfd: c_int = open(DEVPATH.as_ptr() as *const c_char, O_RDONLY);
    let mut lc = papr_location_code { str_: [0; 80] };
    let fd: c_int;

    SKIP_IF_MSG!(
        devfd < 0 && errno == ENOENT,
        b"/dev/papr-vpd not present\0".as_ptr() as *const c_char
    );

    FAIL_IF!(devfd < 0);

    errno = 0;
    fd = ioctl(devfd, PAPR_VPD_IOC_CREATE_HANDLE, &mut lc as *mut papr_location_code);
    FAIL_IF!(errno != 0);
    FAIL_IF!(fd < 0);

    /* close the handle without reading it */
    FAIL_IF!(close(fd) != 0);

    FAIL_IF!(close(devfd) != 0);
    0
}

unsafe extern "C" fn papr_vpd_reread() -> c_int {
    let devfd: c_int = open(DEVPATH.as_ptr() as *const c_char, O_RDONLY);
    let mut lc = papr_location_code { str_: [0; 80] };
    let fd: c_int;

    SKIP_IF_MSG!(
        devfd < 0 && errno == ENOENT,
        b"/dev/papr-vpd not present\0".as_ptr() as *const c_char
    );

    FAIL_IF!(devfd < 0);

    errno = 0;
    fd = ioctl(devfd, PAPR_VPD_IOC_CREATE_HANDLE, &mut lc as *mut papr_location_code);
    FAIL_IF!(errno != 0);
    FAIL_IF!(fd < 0);

    FAIL_IF!(close(devfd) != 0);

    let size: off_t = lseek(fd, 0, SEEK_END);
    FAIL_IF!(size <= 0);

    let mut bufs: [*mut c_char; 2] = [ptr::null_mut(); 2];

    for i in 0..bufs.len() {
        bufs[i] = malloc(size as size_t) as *mut c_char;
        FAIL_IF!(bufs[i].is_null());
        let consumed: ssize_t = pread(fd, bufs[i] as *mut c_void, size as size_t, 0);
        FAIL_IF!(consumed != size as ssize_t);
    }

    FAIL_IF!(memcmp(
        bufs[0] as *const c_void,
        bufs[1] as *const c_void,
        size as size_t,
    ) != 0);

    FAIL_IF!(close(fd) != 0);

    0
}

unsafe extern "C" fn get_system_loc_code(lc: *mut papr_location_code) -> c_int {
    static SYSTEM_ID_PATH: &[u8] = b"/sys/firmware/devicetree/base/system-id\0";
    static MODEL_PATH: &[u8] = b"/sys/firmware/devicetree/base/model\0";
    let mut system_id: *mut c_char = ptr::null_mut();
    let mut model: *mut c_char = ptr::null_mut();
    let mut err: c_int = -1;

    if read_file_alloc(
        MODEL_PATH.as_ptr() as *const c_char,
        &mut model as *mut *mut c_char,
        ptr::null_mut(),
    ) != 0
    {
        return err;
    }

    if read_file_alloc(
        SYSTEM_ID_PATH.as_ptr() as *const c_char,
        &mut system_id as *mut *mut c_char,
        ptr::null_mut(),
    ) != 0
    {
        goto_free_model(model);
        return err;
    }

    let mut mtm: *mut c_char = ptr::null_mut();
    let sscanf_ret: c_int = sscanf(
        model as *const c_char,
        b"IBM,%ms\0".as_ptr() as *const c_char,
        &mut mtm as *mut *mut c_char,
    );
    if sscanf_ret != 1 {
        goto_free_system_id(system_id, model);
        return err;
    }

    let mut plant_and_seq: *mut c_char = ptr::null_mut();
    if sscanf(
        system_id as *const c_char,
        b"IBM,%*c%*c%ms\0".as_ptr() as *const c_char,
        &mut plant_and_seq as *mut *mut c_char,
    ) != 1
    {
        goto_free_mtm(mtm, system_id, model);
        return err;
    }
    /*
     * Replace - with . to build location code.
     */
    let sep: *mut c_char = strchr(mtm as *const c_char, b'-' as c_int);
    if sep.is_null() {
        goto_free_plant_and_seq(plant_and_seq, mtm, system_id, model);
        return err;
    } else {
        *sep = b'.' as c_char;
    }

    snprintf(
        (*lc).str_.as_mut_ptr(),
        (*lc).str_.len(),
        b"U%s.%s\0".as_ptr() as *const c_char,
        mtm,
        plant_and_seq,
    );
    err = 0;

    free(plant_and_seq as *mut c_void);
    free(mtm as *mut c_void);
    free(system_id as *mut c_void);
    free(model as *mut c_void);
    err
}

unsafe fn goto_free_plant_and_seq(
    plant_and_seq: *mut c_char,
    mtm: *mut c_char,
    system_id: *mut c_char,
    model: *mut c_char,
) {
    free(plant_and_seq as *mut c_void);
    goto_free_mtm(mtm, system_id, model);
}

unsafe fn goto_free_mtm(mtm: *mut c_char, system_id: *mut c_char, model: *mut c_char) {
    free(mtm as *mut c_void);
    goto_free_system_id(system_id, model);
}

unsafe fn goto_free_system_id(system_id: *mut c_char, model: *mut c_char) {
    free(system_id as *mut c_void);
    goto_free_model(model);
}

unsafe fn goto_free_model(model: *mut c_char) {
    free(model as *mut c_void);
}

unsafe extern "C" fn papr_vpd_system_loc_code() -> c_int {
    let mut lc = mem::MaybeUninit::<papr_location_code>::uninit();
    let devfd: c_int = open(DEVPATH.as_ptr() as *const c_char, O_RDONLY);
    let mut size: off_t;
    let fd: c_int;

    SKIP_IF_MSG!(
        devfd < 0 && errno == ENOENT,
        b"/dev/papr-vpd not present\0".as_ptr() as *const c_char
    );
    SKIP_IF_MSG!(
        get_system_loc_code(lc.as_mut_ptr()) != 0,
        b"Cannot determine system location code\0".as_ptr() as *const c_char
    );
    let mut lc = lc.assume_init();

    FAIL_IF!(devfd < 0);

    errno = 0;
    fd = ioctl(devfd, PAPR_VPD_IOC_CREATE_HANDLE, &mut lc as *mut papr_location_code);
    FAIL_IF!(errno != 0);
    FAIL_IF!(fd < 0);

    FAIL_IF!(close(devfd) != 0);

    size = lseek(fd, 0, SEEK_END);
    FAIL_IF!(size <= 0);

    let buf: *mut c_void = malloc(size as size_t);
    FAIL_IF!(buf.is_null());

    let consumed: ssize_t = pread(fd, buf, size as size_t, 0);
    FAIL_IF!(consumed != size as ssize_t);

    /* Ensure EOF */
    FAIL_IF!(read(fd, buf, size as size_t) != 0);
    FAIL_IF!(close(fd) != 0);

    /* Verify that the buffer looks like VPD */
    static NEEDLE: &[u8] = b"System VPD\0";
    FAIL_IF!(memmem(
        buf,
        size as size_t,
        NEEDLE.as_ptr() as *const c_void,
        strlen(NEEDLE.as_ptr() as *const c_char),
    )
    .is_null());

    0
}

#[repr(C)]
struct vpd_test {
    function: Option<unsafe extern "C" fn() -> c_int>,
    description: *const c_char,
}

static VPD_TESTS: &[vpd_test] = &[
    vpd_test {
        function: Some(dev_papr_vpd_open_close),
        description: b"open/close /dev/papr-vpd\0".as_ptr() as *const c_char,
    },
    vpd_test {
        function: Some(dev_papr_vpd_unterm_loc_code),
        description: b"ensure EINVAL on unterminated location code\0".as_ptr() as *const c_char,
    },
    vpd_test {
        function: Some(dev_papr_vpd_null_handle),
        description: b"ensure EFAULT on bad handle addr\0".as_ptr() as *const c_char,
    },
    vpd_test {
        function: Some(dev_papr_vpd_get_handle_all),
        description: b"get handle for all VPD\0".as_ptr() as *const c_char,
    },
    vpd_test {
        function: Some(papr_vpd_close_handle_without_reading),
        description: b"close handle without consuming VPD\0".as_ptr() as *const c_char,
    },
    vpd_test {
        function: Some(dev_papr_vpd_get_handle_byte_at_a_time),
        description: b"read all VPD one byte at a time\0".as_ptr() as *const c_char,
    },
    vpd_test {
        function: Some(papr_vpd_reread),
        description: b"ensure re-read yields same results\0".as_ptr() as *const c_char,
    },
    vpd_test {
        function: Some(papr_vpd_system_loc_code),
        description: b"get handle for system VPD\0".as_ptr() as *const c_char,
    },
];

fn main() {
    unsafe {
        let mut fails: size_t = 0;

        for i in 0..VPD_TESTS.len() {
            let t: *const vpd_test = &VPD_TESTS[i];

            if test_harness((*t).function, (*t).description) != 0 {
                fails = fails.wrapping_add(1);
            }
        }

        std::process::exit(if fails == 0 { EXIT_SUCCESS } else { EXIT_FAILURE });
    }
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
