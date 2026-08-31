// SPDX-License-Identifier: GPL-2.0-or-later

// C dependencies:
// errno.h, fcntl.h, signal.h, stdio.h, string.h, sys/ioctl.h, sys/mman.h,
// sys/stat.h, sys/types.h, unistd.h
// Local dependencies:
// vas-api.h, utils.h

use core::ffi::{c_char, c_int, c_ulong, c_void};

static mut faulted: bool = false;

unsafe extern "C" {
    fn access(pathname: *const c_char, mode: c_int) -> c_int;
    fn open(pathname: *const c_char, flags: c_int) -> c_int;
    fn ioctl(fd: c_int, request: c_ulong, ...) -> c_int;
    fn sigaction(
        signum: c_int,
        act: *const libc::sigaction,
        oldact: *mut libc::sigaction,
    ) -> c_int;
    fn mmap(
        addr: *mut c_void,
        length: usize,
        prot: c_int,
        flags: c_int,
        fd: c_int,
        offset: libc::off_t,
    ) -> *mut c_void;

    fn test_harness(
        test_function: unsafe extern "C" fn() -> c_int,
        name: *const c_char,
    ) -> c_int;
}

unsafe extern "C" fn sigbus_handler(_n: c_int, _info: *mut libc::siginfo_t, ctxt_v: *mut c_void) {
    let ctxt = ctxt_v as *mut libc::ucontext_t;
    let regs = (*ctxt).uc_mcontext.regs;

    faulted = true;
    (*regs).nip += 4;
}

unsafe extern "C" fn test_ra_error() -> c_int {
    let mut attr: vas_tx_win_open_attr = core::mem::zeroed();
    let fd: c_int;
    let paste_addr: *mut c_int;
    let devname = b"/dev/crypto/nx-gzip\0".as_ptr() as *const c_char;
    let mut act: libc::sigaction = core::mem::zeroed();

    act.sa_sigaction = sigbus_handler as usize;
    act.sa_flags = libc::SA_SIGINFO;

    attr.version = 1;
    attr.vas_id = 0;

    SKIP_IF!(access(devname, libc::F_OK));

    fd = open(devname, libc::O_RDWR);
    FAIL_IF!(fd < 0);
    FAIL_IF!(ioctl(fd, VAS_TX_WIN_OPEN, &mut attr) < 0);
    FAIL_IF!(sigaction(libc::SIGBUS, &act, core::ptr::null_mut()) != 0);

    paste_addr = mmap(
        core::ptr::null_mut(),
        4096,
        libc::PROT_READ | libc::PROT_WRITE,
        libc::MAP_SHARED,
        fd,
        0u64 as libc::off_t,
    ) as *mut c_int;

    /* The following assignment triggers exception */
    mb();
    *paste_addr = 1;
    mb();

    FAIL_IF!(!faulted);

    0
}

fn main() {
    unsafe {
        std::process::exit(test_harness(
            test_ra_error,
            b"inject-ra-err\0".as_ptr() as *const c_char,
        ));
    }
}
