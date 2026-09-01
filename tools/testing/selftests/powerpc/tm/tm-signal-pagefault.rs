// SPDX-License-Identifier: GPL-2.0
/*
 * Copyright 2020, Gustavo Luiz Duarte, IBM Corp.
 *
 * This test starts a transaction and triggers a signal, forcing a pagefault to
 * happen when the kernel signal handling code touches the user signal stack.
 *
 * In order to avoid pre-faulting the signal stack memory and to force the
 * pagefault to happen precisely in the kernel signal handling code, the
 * pagefault handling is done in userspace using the userfaultfd facility.
 *
 * Further pagefaults are triggered by crafting the signal handler's ucontext
 * to point to additional memory regions managed by the userfaultfd, so using
 * the same mechanism used to avoid pre-faulting the signal stack memory.
 *
 * On failure (bug is present) kernel crashes or never returns control back to
 * userspace. If bug is not present, tests completes almost immediately.
 */

// C dependencies: stdio.h, stdlib.h, string.h, linux/userfaultfd.h, poll.h,
// unistd.h, sys/ioctl.h, sys/syscall.h, fcntl.h, sys/mman.h, pthread.h,
// signal.h, errno.h, and "tm.h".

use core::arch::asm;
use core::ffi::{c_char, c_int, c_long, c_ulong, c_void};
use core::mem::size_of;
use core::ptr;

const UF_MEM_SIZE: usize = 655360; /* 10 x 64k pages */

const EXIT_FAILURE: c_int = 1;
const EXIT_SUCCESS: c_int = 0;

const POLLIN: c_short = 0x0001;
const O_CLOEXEC: c_int = 0o2000000;
const O_NONBLOCK: c_int = 0o0004000;
const PROT_READ: c_int = 0x1;
const PROT_WRITE: c_int = 0x2;
const MAP_PRIVATE: c_int = 0x02;
const MAP_ANONYMOUS: c_int = 0x20;
const SA_SIGINFO: c_int = 4;
const SA_ONSTACK: c_int = 0x08000000;
const SIGTRAP: c_int = 5;
const SIGSTKSZ: usize = 8192;
const ENOSYS: c_int = 38;
const _SC_PAGE_SIZE: c_int = 30;
const __NR_USERFAULTFD: c_long = 323;
const UFFD_API: u64 = 0xAA;
const UFFD_EVENT_PAGEFAULT: u8 = 12;
const UFFDIO_REGISTER_MODE_MISSING: u64 = 1;

type c_short = i16;
type size_t = usize;
type ssize_t = isize;
type pthread_t = c_ulong;
type elf_vrreg_t = [u8; 16];

// ioctl request values come from linux/userfaultfd.h for the target C build.
const UFFDIO_API: c_ulong = 0;
const UFFDIO_REGISTER: c_ulong = 0;
const UFFDIO_COPY: c_ulong = 0;

#[repr(C)]
struct FILE {
    _private: [u8; 0],
}

#[repr(C)]
struct pollfd {
    fd: c_int,
    events: c_short,
    revents: c_short,
}

#[repr(C)]
struct uffd_msg_pagefault {
    flags: u64,
    address: u64,
    reserved: u32,
}

#[repr(C)]
union uffd_msg_arg {
    pagefault: uffd_msg_pagefault,
}

#[repr(C)]
struct uffd_msg {
    event: u8,
    reserved1: u8,
    reserved2: u16,
    reserved3: u32,
    arg: uffd_msg_arg,
}

#[repr(C)]
struct uffdio_range {
    start: u64,
    len: u64,
}

#[repr(C)]
struct uffdio_api {
    api: u64,
    features: u64,
    ioctls: u64,
}

#[repr(C)]
struct uffdio_register {
    range: uffdio_range,
    mode: u64,
    ioctls: u64,
}

#[repr(C)]
struct uffdio_copy {
    dst: u64,
    src: u64,
    len: u64,
    mode: u64,
    copy: i64,
}

#[repr(C)]
struct stack_t {
    ss_sp: *mut c_void,
    ss_flags: c_int,
    ss_size: size_t,
}

#[repr(C)]
struct siginfo_t {
    _private: [u8; 0],
}

#[repr(C)]
struct pt_regs {
    nip: c_ulong,
}

#[repr(C)]
struct mcontext_t {
    regs: *mut pt_regs,
    v_regs: *mut elf_vrreg_t,
}

#[repr(C)]
struct ucontext_t {
    uc_link: *mut ucontext_t,
    uc_mcontext: mcontext_t,
}

#[repr(C)]
struct sigaction {
    sa_sigaction: extern "C" fn(c_int, *mut siginfo_t, *mut c_void),
    sa_flags: c_int,
}

unsafe extern "C" {
    static mut stderr: *mut FILE;
    static mut errno: c_int;
    static MAP_FAILED: *mut c_void;

    fn fprintf(stream: *mut FILE, format: *const c_char, ...) -> c_int;
    fn perror(s: *const c_char);
    fn exit(status: c_int) -> !;
    fn memcpy(dest: *mut c_void, src: *const c_void, n: size_t) -> *mut c_void;
    fn poll(fds: *mut pollfd, nfds: c_ulong, timeout: c_int) -> c_int;
    fn read(fd: c_int, buf: *mut c_void, count: size_t) -> ssize_t;
    fn ioctl(fd: c_int, request: c_ulong, ...) -> c_int;
    fn sysconf(name: c_int) -> c_long;
    fn syscall(number: c_long, ...) -> c_long;
    fn mmap(
        addr: *mut c_void,
        length: size_t,
        prot: c_int,
        flags: c_int,
        fd: c_int,
        offset: c_long,
    ) -> *mut c_void;
    fn pthread_create(
        thread: *mut pthread_t,
        attr: *const c_void,
        start_routine: extern "C" fn(*mut c_void) -> *mut c_void,
        arg: *mut c_void,
    ) -> c_int;
    fn sigaltstack(ss: *const stack_t, old_ss: *mut stack_t) -> c_int;
    fn sigaction(signum: c_int, act: *const sigaction, oldact: *mut sigaction) -> c_int;

    fn have_htm() -> bool;
    fn htm_is_synthetic() -> bool;
    fn test_harness_set_timeout(timeout: c_int);
    fn test_harness(
        test_function: extern "C" fn() -> c_int,
        name: *const c_char,
    ) -> c_int;
}

macro_rules! c_str {
    ($s:literal) => {
        concat!($s, "\0").as_ptr() as *const c_char
    };
}

macro_rules! SKIP_IF {
    ($cond:expr) => {
        if $cond {
            return EXIT_SUCCESS;
        }
    };
}

/* Memory handled by userfaultfd */
static mut UF_MEM: *mut c_char = ptr::null_mut();
static mut UF_MEM_OFFSET: size_t = 0;

/*
 * Data that will be copied into the faulting pages (instead of zero-filled
 * pages). This is used to make the test more reliable and avoid segfaulting
 * when we return from the signal handler. Since we are making the signal
 * handler's ucontext point to newly allocated memory, when that memory is
 * paged-in it will contain the expected content.
 */
static mut BACKING_MEM: [c_char; UF_MEM_SIZE] = [0; UF_MEM_SIZE];

static mut PAGESIZE: size_t = 0;

/*
 * Return a chunk of at least 'size' bytes of memory that will be handled by
 * userfaultfd. If 'backing_data' is not NULL, its content will be save to
 * 'backing_mem' and then copied into the faulting pages when the page fault
 * is handled.
 */
unsafe extern "C" fn get_uf_mem(size: size_t, backing_data: *mut c_void) -> *mut c_void {
    let ret: *mut c_void;

    if UF_MEM_OFFSET + size > UF_MEM_SIZE {
        fprintf(stderr, c_str!("Requesting more uf_mem than expected!\n"));
        exit(EXIT_FAILURE);
    }

    ret = UF_MEM.add(UF_MEM_OFFSET) as *mut c_void;

    /* Save the data that will be copied into the faulting page */
    if !backing_data.is_null() {
        memcpy(
            BACKING_MEM.as_mut_ptr().add(UF_MEM_OFFSET) as *mut c_void,
            backing_data,
            size,
        );
    }

    /* Reserve the requested amount of uf_mem */
    UF_MEM_OFFSET += size;
    /* Keep uf_mem_offset aligned to the page size (round up) */
    UF_MEM_OFFSET = (UF_MEM_OFFSET + PAGESIZE - 1) & !(PAGESIZE - 1);

    ret
}

extern "C" fn fault_handler_thread(arg: *mut c_void) -> *mut c_void {
    unsafe {
        let mut msg: uffd_msg = core::mem::zeroed(); /* Data read from userfaultfd */
        let uffd: c_long; /* userfaultfd file descriptor */
        let mut uffdio_copy: uffdio_copy = core::mem::zeroed();
        let mut pollfd: pollfd = core::mem::zeroed();
        let mut nread: ssize_t;
        let offset: ssize_t;

        uffd = arg as c_long;

        loop {
            pollfd.fd = uffd as c_int;
            pollfd.events = POLLIN;
            if poll(&mut pollfd, 1, -1) == -1 {
                perror(c_str!("poll() failed"));
                exit(EXIT_FAILURE);
            }

            nread = read(uffd as c_int, &mut msg as *mut _ as *mut c_void, size_of::<uffd_msg>());
            if nread == 0 {
                fprintf(stderr, c_str!("read(): EOF on userfaultfd\n"));
                exit(EXIT_FAILURE);
            }

            if nread == -1 {
                perror(c_str!("read() failed"));
                exit(EXIT_FAILURE);
            }

            /* We expect only one kind of event */
            if msg.event != UFFD_EVENT_PAGEFAULT {
                fprintf(stderr, c_str!("Unexpected event on userfaultfd\n"));
                exit(EXIT_FAILURE);
            }

            /*
             * We need to handle page faults in units of pages(!).
             * So, round faulting address down to page boundary.
             */
            uffdio_copy.dst = msg.arg.pagefault.address & !((PAGESIZE - 1) as u64);

            offset = (uffdio_copy.dst as *mut c_char).offset_from(UF_MEM);
            uffdio_copy.src = BACKING_MEM.as_ptr().offset(offset) as u64;

            uffdio_copy.len = PAGESIZE as u64;
            uffdio_copy.mode = 0;
            uffdio_copy.copy = 0;
            if ioctl(uffd as c_int, UFFDIO_COPY, &mut uffdio_copy) == -1 {
                perror(c_str!("ioctl-UFFDIO_COPY failed"));
                exit(EXIT_FAILURE);
            }
        }
    }
}

unsafe extern "C" fn setup_uf_mem() {
    let uffd: c_long; /* userfaultfd file descriptor */
    let mut thr: pthread_t = 0;
    let mut uffdio_api: uffdio_api = core::mem::zeroed();
    let mut uffdio_register: uffdio_register = core::mem::zeroed();
    let ret: c_int;

    PAGESIZE = sysconf(_SC_PAGE_SIZE) as size_t;

    /* Create and enable userfaultfd object */
    uffd = syscall(__NR_USERFAULTFD, O_CLOEXEC | O_NONBLOCK);
    if uffd == -1 {
        perror(c_str!("userfaultfd() failed"));
        exit(EXIT_FAILURE);
    }
    uffdio_api.api = UFFD_API;
    uffdio_api.features = 0;
    if ioctl(uffd as c_int, UFFDIO_API, &mut uffdio_api) == -1 {
        perror(c_str!("ioctl-UFFDIO_API failed"));
        exit(EXIT_FAILURE);
    }

    /*
     * Create a private anonymous mapping. The memory will be demand-zero
     * paged, that is, not yet allocated. When we actually touch the memory
     * the related page will be allocated via the userfaultfd mechanism.
     */
    UF_MEM = mmap(
        ptr::null_mut(),
        UF_MEM_SIZE,
        PROT_READ | PROT_WRITE,
        MAP_PRIVATE | MAP_ANONYMOUS,
        -1,
        0,
    ) as *mut c_char;
    if UF_MEM as *mut c_void == MAP_FAILED {
        perror(c_str!("mmap() failed"));
        exit(EXIT_FAILURE);
    }

    /*
     * Register the memory range of the mapping we've just mapped to be
     * handled by the userfaultfd object. In 'mode' we request to track
     * missing pages (i.e. pages that have not yet been faulted-in).
     */
    uffdio_register.range.start = UF_MEM as u64;
    uffdio_register.range.len = UF_MEM_SIZE as u64;
    uffdio_register.mode = UFFDIO_REGISTER_MODE_MISSING;
    if ioctl(uffd as c_int, UFFDIO_REGISTER, &mut uffdio_register) == -1 {
        perror(c_str!("ioctl-UFFDIO_REGISTER"));
        exit(EXIT_FAILURE);
    }

    /* Create a thread that will process the userfaultfd events */
    ret = pthread_create(
        &mut thr,
        ptr::null(),
        fault_handler_thread,
        uffd as *mut c_void,
    );
    if ret != 0 {
        fprintf(
            stderr,
            c_str!("pthread_create(): Error. Returned %d\n"),
            ret,
        );
        exit(EXIT_FAILURE);
    }
}

/*
 * Assumption: the signal was delivered while userspace was in transactional or
 * suspended state, i.e. uc->uc_link != NULL.
 */
extern "C" fn signal_handler(_signo: c_int, _si: *mut siginfo_t, uc: *mut c_void) {
    unsafe {
        let ucp: *mut ucontext_t = uc as *mut ucontext_t;

        /* Skip 'trap' after returning, otherwise we get a SIGTRAP again */
        (*(*(*ucp).uc_link).uc_mcontext.regs).nip += 4;

        (*ucp).uc_mcontext.v_regs = get_uf_mem(
            size_of::<elf_vrreg_t>(),
            (*ucp).uc_mcontext.v_regs as *mut c_void,
        ) as *mut elf_vrreg_t;

        (*(*ucp).uc_link).uc_mcontext.v_regs = get_uf_mem(
            size_of::<elf_vrreg_t>(),
            (*(*ucp).uc_link).uc_mcontext.v_regs as *mut c_void,
        ) as *mut elf_vrreg_t;

        (*ucp).uc_link = get_uf_mem(size_of::<ucontext_t>(), (*ucp).uc_link as *mut c_void)
            as *mut ucontext_t;
    }
}

unsafe extern "C" fn have_userfaultfd() -> bool {
    let rc: c_long;

    errno = 0;
    rc = syscall(__NR_USERFAULTFD, -1);

    rc == 0 || errno != ENOSYS
}

extern "C" fn tm_signal_pagefault() -> c_int {
    unsafe {
        let mut sa: sigaction = core::mem::zeroed();
        let mut ss: stack_t = core::mem::zeroed();

        SKIP_IF!(!have_htm());
        SKIP_IF!(htm_is_synthetic());
        SKIP_IF!(!have_userfaultfd());

        setup_uf_mem();

        /*
         * Set an alternative stack that will generate a page fault when the
         * signal is raised. The page fault will be treated via userfaultfd,
         * i.e. via fault_handler_thread.
         */
        ss.ss_sp = get_uf_mem(SIGSTKSZ, ptr::null_mut());
        ss.ss_size = SIGSTKSZ;
        ss.ss_flags = 0;
        if sigaltstack(&ss, ptr::null_mut()) == -1 {
            perror(c_str!("sigaltstack() failed"));
            exit(EXIT_FAILURE);
        }

        sa.sa_flags = SA_SIGINFO | SA_ONSTACK;
        sa.sa_sigaction = signal_handler;
        if sigaction(SIGTRAP, &sa, ptr::null_mut()) == -1 {
            perror(c_str!("sigaction() failed"));
            exit(EXIT_FAILURE);
        }

        /* Trigger a SIGTRAP in transactional state */
        asm!(
            "tbegin.",
            "beq    1f",
            "trap",
            "1:",
            options(nostack, preserves_flags)
        );

        /* Trigger a SIGTRAP in suspended state */
        asm!(
            "tbegin.",
            "beq    1f",
            "tsuspend.",
            "trap",
            "tresume.",
            "1:",
            options(nostack, preserves_flags)
        );

        EXIT_SUCCESS
    }
}

#[unsafe(no_mangle)]
pub extern "C" fn main(_argc: c_int, _argv: *mut *mut c_char) -> c_int {
    unsafe {
        /*
         * Depending on kernel config, the TM Bad Thing might not result in a
         * crash, instead the kernel never returns control back to userspace, so
         * set a tight timeout. If the test passes it completes almost
         * immediately.
         */
        test_harness_set_timeout(2);
        test_harness(tm_signal_pagefault, c_str!("tm_signal_pagefault"))
    }
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
