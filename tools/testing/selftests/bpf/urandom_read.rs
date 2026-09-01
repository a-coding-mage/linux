// C includes removed; external C/libc and project symbols are declared below.

pub type c_char = i8;
pub type c_int = i32;
pub type FILE = core::ffi::c_void;

pub const _SDT_HAS_SEMAPHORES: c_int = 1;
pub const SHARED: c_int = 1;
pub const BUF_SIZE: usize = 256;
pub const O_RDONLY: c_int = 0;
pub const SIGPIPE: c_int = 13;

/* defined in urandom_read_aux.c */
unsafe extern "C" {
    pub fn urand_read_without_sema(iter_num: c_int, iter_cnt: c_int, read_sz: c_int);
    /* these are coming from urandom_read_lib{1,2}.c */
    pub fn urandlib_read_with_sema(iter_num: c_int, iter_cnt: c_int, read_sz: c_int);
    pub fn urandlib_read_without_sema(iter_num: c_int, iter_cnt: c_int, read_sz: c_int);

    pub fn urandlib_api() -> c_int;
    // COMPAT_VERSION(urandlib_api_old, urandlib_api, LIBURANDOM_READ_1.0.0)
    pub fn urandlib_api_old() -> c_int;
    pub fn urandlib_api_sameoffset() -> c_int;

    pub static mut stdout: *mut FILE;

    pub fn open(pathname: *const c_char, flags: c_int, ...) -> c_int;
    pub fn atoi(nptr: *const c_char) -> c_int;
    pub fn signal(signum: c_int, handler: extern "C" fn(c_int)) -> extern "C" fn(c_int);
    pub fn fprintf(stream: *mut FILE, format: *const c_char, ...) -> c_int;
    pub fn fflush(stream: *mut FILE) -> c_int;
    pub fn getpid() -> c_int;
    pub fn read(fd: c_int, buf: *mut core::ffi::c_void, count: usize) -> isize;
    pub fn close(fd: c_int) -> c_int;
}

#[unsafe(link_section = ".probes")]
#[used]
pub static mut urand_read_with_sema_semaphore: u16 = 0;

#[inline(never)]
unsafe fn urandom_read(fd: c_int, count: c_int) {
    let mut buf = [0 as c_char; BUF_SIZE];
    let mut i: c_int;

    i = 0;
    while i < count {
        unsafe {
            read(fd, buf.as_mut_ptr() as *mut core::ffi::c_void, BUF_SIZE);

            /* trigger USDTs defined in executable itself */
            urand_read_without_sema(i, count, BUF_SIZE as c_int);
            // STAP_PROBE3(urand, read_with_sema, i, count, BUF_SIZE);

            /* trigger USDTs defined in shared lib */
            urandlib_read_without_sema(i, count, BUF_SIZE as c_int);
            urandlib_read_with_sema(i, count, BUF_SIZE as c_int);
        }
        i += 1;
    }
}

static mut parent_ready: bool = false;

extern "C" fn handle_sigpipe(_sig: c_int) {
    unsafe {
        core::ptr::write_volatile(core::ptr::addr_of_mut!(parent_ready), true);
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn main(argc: c_int, argv: *mut *mut c_char) -> c_int {
    let fd = unsafe { open(c"/dev/urandom".as_ptr() as *const c_char, O_RDONLY) };
    let mut count: c_int = 4;
    let mut report_pid = false;

    if fd < 0 {
        return 1;
    }

    if argc >= 2 {
        count = unsafe { atoi(*argv.add(1)) };
    }
    if argc >= 3 {
        report_pid = true;
        /* install SIGPIPE handler to catch when parent closes their
         * end of the pipe (on the other side of our stdout)
         */
        unsafe {
            signal(SIGPIPE, handle_sigpipe);
        }
    }

    /* report PID and wait for parent process to send us "signal" by
     * closing stdout
     */
    if report_pid {
        while unsafe { !core::ptr::read_volatile(core::ptr::addr_of!(parent_ready)) } {
            unsafe {
                fprintf(stdout, c"%d\n".as_ptr() as *const c_char, getpid());
                fflush(stdout);
            }
        }
        /* at this point stdout is closed, parent process knows our
         * PID and is ready to trace us
         */
    }

    unsafe {
        urandom_read(fd, count);

        urandlib_api();
        urandlib_api_old();
        urandlib_api_sameoffset();

        close(fd);
    }
    0
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
