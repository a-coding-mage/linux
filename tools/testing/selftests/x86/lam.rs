// SPDX-License-Identifier: GPL-2.0
// Translated from testing/selftests/x86/lam.c.
// C dependency intent: _GNU_SOURCE, kselftest.h, linux/io_uring.h, and x86_64 only.

#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(dead_code)]
#![allow(improper_ctypes)]

use core::arch::x86_64::__cpuid_count;
use core::ffi::{c_char, c_int, c_long, c_uint, c_ulong, c_void};
use core::mem::{size_of, zeroed};
use core::ptr::{null, null_mut};

type pid_t = c_int;
type off_t = i64;
type size_t = usize;
type uint = c_uint;
type uint8_t = u8;
type uint64_t = u64;

const LAM_NONE: c_ulong = 0;
const LAM_U57_BITS: c_ulong = 6;

const LAM_U57_MASK: uint64_t = 0x3f_u64 << 57;
/* arch prctl for LAM */
const ARCH_GET_UNTAG_MASK: c_ulong = 0x4001;
const ARCH_ENABLE_TAGGED_ADDR: c_ulong = 0x4002;
const ARCH_GET_MAX_TAG_BITS: c_ulong = 0x4003;
const ARCH_FORCE_TAGGED_SVA: c_ulong = 0x4004;

/* Specified test function bits */
const FUNC_MALLOC: c_uint = 0x1;
const FUNC_BITS: c_uint = 0x2;
const FUNC_MMAP: c_uint = 0x4;
const FUNC_SYSCALL: c_uint = 0x8;
const FUNC_URING: c_uint = 0x10;
const FUNC_INHERITE: c_uint = 0x20;
const FUNC_PASID: c_uint = 0x40;

/* get_user() pointer test cases */
const GET_USER_USER: c_uint = 0;
const GET_USER_KERNEL_TOP: c_uint = 1;
const GET_USER_KERNEL_BOT: c_uint = 2;
const GET_USER_KERNEL: c_uint = 3;

const TEST_MASK: c_uint = 0x7f;
const L5_SIGN_EXT_MASK: uint64_t = 0xff_u64 << 56;
const L4_SIGN_EXT_MASK: uint64_t = 0x1ffff_u64 << 47;

const LOW_ADDR: uint64_t = 0x1_u64 << 30;
const HIGH_ADDR: uint64_t = 0x3_u64 << 48;

const MALLOC_LEN: usize = 32;
const PAGE_SIZE: usize = 4 << 10;
const STACK_SIZE: usize = 65536;

const URING_QUEUE_SZ: c_uint = 1;
const URING_BLOCK_SZ: off_t = 2048;

/* Pasid test define */
const LAM_CMD_BIT: c_uint = 0x1;
const PAS_CMD_BIT: c_uint = 0x2;
const SVA_CMD_BIT: c_uint = 0x4;

const fn PAS_CMD(cmd1: c_uint, cmd2: c_uint, cmd3: c_uint) -> uint64_t {
    ((cmd3 as uint64_t) << 8) | ((cmd2 as uint64_t) << 4) | ((cmd1 as uint64_t) << 0)
}

const PROT_READ: c_int = 0x1;
const PROT_WRITE: c_int = 0x2;
const MAP_SHARED: c_int = 0x01;
const MAP_PRIVATE: c_int = 0x02;
const MAP_FIXED: c_int = 0x10;
const MAP_ANONYMOUS: c_int = 0x20;
const MAP_POPULATE: c_int = 0x8000;
const O_RDONLY: c_int = 0;
const O_RDWR: c_int = 2;
const SIGSEGV: c_int = 11;
const SIGCHLD: c_int = 17;
const CLONE_VM: c_int = 0x00000100;
const CLONE_FS: c_int = 0x00000200;
const CLONE_FILES: c_int = 0x00000400;
const EXIT_FAILURE: c_int = 1;
const PATH_MAX: usize = 4096;
const FIOASYNC: c_ulong = 0x5452;
const KSFT_SKIP: c_int = 4;
const SYS_arch_prctl: c_long = 158;
const __NR_io_uring_setup: c_long = 425;
const __NR_io_uring_enter: c_long = 426;
const IORING_FEAT_SINGLE_MMAP: u32 = 1 << 0;
const IORING_OFF_SQ_RING: off_t = 0;
const IORING_OFF_CQ_RING: off_t = 0x8000000;
const IORING_OFF_SQES: off_t = 0x10000000;
const IORING_ENTER_GETEVENTS: c_uint = 1;
const IORING_OP_READV: u8 = 1;

#[repr(C)]
struct testcases {
    later: c_uint,
    expected: c_int, /* 2: SIGSEGV Error; 1: other errors */
    lam: c_ulong,
    addr: uint64_t,
    cmd: uint64_t,
    test_func: Option<unsafe extern "C" fn(*mut testcases) -> c_int>,
    msg: *const c_char,
}

/* Used by CQ of uring, source file handler and file's size */
#[repr(C)]
struct file_io {
    file_fd: c_int,
    file_sz: off_t,
    iovecs: [iovec; 0],
}

#[repr(C)]
#[derive(Copy, Clone)]
struct iovec {
    iov_base: *mut c_void,
    iov_len: size_t,
}

#[repr(C)]
struct io_uring_sqring_offsets {
    head: u32,
    tail: u32,
    ring_mask: u32,
    ring_entries: u32,
    flags: u32,
    dropped: u32,
    array: u32,
    resv1: u32,
    user_addr: u64,
}

#[repr(C)]
struct io_uring_cqring_offsets {
    head: u32,
    tail: u32,
    ring_mask: u32,
    ring_entries: u32,
    overflow: u32,
    cqes: u32,
    flags: u32,
    resv1: u32,
    user_addr: u64,
}

#[repr(C)]
struct io_uring_params {
    sq_entries: u32,
    cq_entries: u32,
    flags: u32,
    sq_thread_cpu: u32,
    sq_thread_idle: u32,
    features: u32,
    wq_fd: u32,
    resv: [u32; 3],
    sq_off: io_uring_sqring_offsets,
    cq_off: io_uring_cqring_offsets,
}

#[repr(C)]
struct io_uring_cqe {
    user_data: u64,
    res: i32,
    flags: u32,
}

#[repr(C)]
struct io_uring_sqe {
    opcode: u8,
    flags: u8,
    ioprio: u16,
    fd: i32,
    off: u64,
    addr: u64,
    len: u32,
    rw_flags: u32,
    user_data: u64,
    rest: [u64; 6],
}

#[repr(C)]
union io_uring_queue_ptr {
    cqes: *mut io_uring_cqe,
    sqes: *mut io_uring_sqe,
}

#[repr(C)]
struct io_uring_queue {
    head: *mut c_uint,
    tail: *mut c_uint,
    ring_mask: *mut c_uint,
    ring_entries: *mut c_uint,
    flags: *mut c_uint,
    array: *mut c_uint,
    queue: io_uring_queue_ptr,
    ring_sz: size_t,
}

#[repr(C)]
struct io_ring {
    ring_fd: c_int,
    sq_ring: io_uring_queue,
    cq_ring: io_uring_queue,
}

#[repr(C)]
struct utsname {
    sysname: [c_char; 65],
    nodename: [c_char; 65],
    release: [c_char; 65],
    version: [c_char; 65],
    machine: [c_char; 65],
    domainname: [c_char; 65],
}

#[repr(C)]
struct stat {
    data: [u8; 256],
}

unsafe extern "C" {
    static mut optarg: *mut c_char;
    static mut errno: c_int;

    fn syscall(number: c_long, ...) -> c_long;
    fn mmap(addr: *mut c_void, len: size_t, prot: c_int, flags: c_int, fd: c_int, off: off_t) -> *mut c_void;
    fn munmap(addr: *mut c_void, len: size_t) -> c_int;
    fn malloc(size: size_t) -> *mut c_void;
    fn free(ptr: *mut c_void);
    fn strcpy(dest: *mut c_char, src: *const c_char) -> *mut c_char;
    fn strcmp(s1: *const c_char, s2: *const c_char) -> c_int;
    fn srand(seed: c_uint);
    fn rand() -> c_int;
    fn time(tloc: *mut c_long) -> c_long;
    fn fork() -> pid_t;
    fn wait(status: *mut c_int) -> pid_t;
    fn waitpid(pid: pid_t, status: *mut c_int, options: c_int) -> pid_t;
    fn exit(status: c_int) -> !;
    fn perror(s: *const c_char);
    fn signal(sig: c_int, handler: unsafe extern "C" fn(c_int)) -> usize;
    fn sigsetjmp(env: *mut c_long, savesigs: c_int) -> c_int;
    fn siglongjmp(env: *mut c_long, val: c_int) -> !;
    fn uname(buf: *mut utsname) -> c_int;
    fn memfd_create(name: *const c_char, flags: c_uint) -> c_int;
    fn ioctl(fd: c_int, request: c_ulong, ...) -> c_int;
    fn close(fd: c_int) -> c_int;
    fn memset(s: *mut c_void, c: c_int, n: size_t) -> *mut c_void;
    fn posix_memalign(memptr: *mut *mut c_void, alignment: size_t, size: size_t) -> c_int;
    fn readlink(pathname: *const c_char, buf: *mut c_char, bufsiz: size_t) -> isize;
    fn open(pathname: *const c_char, flags: c_int, ...) -> c_int;
    fn fstat(fd: c_int, statbuf: *mut stat) -> c_int;
    fn execlp(file: *const c_char, arg: *const c_char, ...) -> c_int;
    fn clone(fn_: unsafe extern "C" fn(*mut c_void) -> c_int, child_stack: *mut c_void, flags: c_int, arg: *mut c_void) -> c_int;
    fn getopt(argc: c_int, argv: *mut *mut c_char, optstring: *const c_char) -> c_int;
    fn strtoul(nptr: *const c_char, endptr: *mut *mut c_char, base: c_int) -> c_ulong;
    fn printf(format: *const c_char, ...) -> c_int;
    fn stat(pathname: *const c_char, statbuf: *mut stat) -> c_int;
    fn snprintf(str_: *mut c_char, size: size_t, format: *const c_char, ...) -> c_int;
    fn popen(command: *const c_char, type_: *const c_char) -> *mut c_void;
    fn fgets(s: *mut c_char, size: c_int, stream: *mut c_void) -> *mut c_char;
    fn pclose(stream: *mut c_void) -> c_int;
    fn strtol(nptr: *const c_char, endptr: *mut *mut c_char, base: c_int) -> c_long;
    fn system(command: *const c_char) -> c_int;

    fn ksft_print_msg(format: *const c_char, ...);
    fn ksft_test_result_skip(format: *const c_char, ...);
    fn ksft_test_result(result: c_int, format: *const c_char, ...);
    fn ksft_set_plan(plan: c_uint);
    fn ksft_exit_pass() -> !;
}

static mut tests_cnt: c_int = 0;
static mut segv_env: [c_long; 32] = [0; 32];

unsafe extern "C" fn segv_handler(sig: c_int) {
    ksft_print_msg(c"Get segmentation fault(%d).".as_ptr(), sig);
    siglongjmp((&raw mut segv_env) as *mut c_long, 1);
}

unsafe fn lam_is_available() -> c_int {
    let cpuinfo = __cpuid_count(0x7, 1);
    let mut bits: c_ulong = 0;

    /* Check if cpu supports LAM */
    if (cpuinfo.eax & (1 << 26)) == 0 {
        ksft_print_msg(c"LAM is not supported!\n".as_ptr());
        return 0;
    }

    /* Return 0 if CONFIG_ADDRESS_MASKING is not set */
    let ret = syscall(SYS_arch_prctl, ARCH_GET_MAX_TAG_BITS, &mut bits as *mut c_ulong);
    if ret != 0 {
        ksft_print_msg(c"LAM is disabled in the kernel!\n".as_ptr());
        return 0;
    }

    1
}

unsafe fn la57_enabled() -> c_int {
    let p = mmap(HIGH_ADDR as *mut c_void, PAGE_SIZE, PROT_READ | PROT_WRITE,
                 MAP_PRIVATE | MAP_ANONYMOUS | MAP_FIXED, -1, 0);
    let ret = if p == MAP_FAILED() { 0 } else { 1 };
    munmap(p, PAGE_SIZE);
    ret
}

/*
 * Set tagged address and read back untag mask.
 * check if the untagged mask is expected.
 *
 * @return:
 * 0: Set LAM mode successfully
 * others: failed to set LAM
 */
unsafe fn set_lam(lam: c_ulong) -> c_int {
    let mut ret = 0;
    let mut ptr: uint64_t = 0;

    if lam != LAM_U57_BITS && lam != LAM_NONE {
        return -1;
    }

    /* Skip check return */
    syscall(SYS_arch_prctl, ARCH_ENABLE_TAGGED_ADDR, lam);

    /* Get untagged mask */
    syscall(SYS_arch_prctl, ARCH_GET_UNTAG_MASK, &mut ptr as *mut uint64_t);

    /* Check mask returned is expected */
    if lam == LAM_U57_BITS {
        ret = (ptr != !LAM_U57_MASK) as c_int;
    } else if lam == LAM_NONE {
        ret = (ptr != !0_u64) as c_int;
    }

    ret
}

unsafe fn get_default_tag_bits() -> c_ulong {
    let pid = fork();
    let mut lam: c_int = LAM_NONE as c_int;
    let mut ret: c_int = 0;

    if pid < 0 {
        perror(c"Fork failed.".as_ptr());
    } else if pid == 0 {
        /* Set LAM mode in child process */
        if set_lam(LAM_U57_BITS) == 0 {
            lam = LAM_U57_BITS as c_int;
        } else {
            lam = LAM_NONE as c_int;
        }
        exit(lam);
    } else {
        wait(&mut ret);
        lam = WEXITSTATUS(ret);
    }

    lam as c_ulong
}

/*
 * Set tagged address and read back untag mask.
 * check if the untag mask is expected.
 */
unsafe fn get_lam() -> c_int {
    let mut ptr: uint64_t = 0;
    let mut ret = -1;
    /* Get untagged mask */
    if syscall(SYS_arch_prctl, ARCH_GET_UNTAG_MASK, &mut ptr as *mut uint64_t) == -1 {
        return -1;
    }

    /* Check mask returned is expected */
    if ptr == !LAM_U57_MASK {
        ret = LAM_U57_BITS as c_int;
    } else if ptr == !0_u64 {
        ret = LAM_NONE as c_int;
    }

    ret
}

/* According to LAM mode, set metadata in high bits */
unsafe fn set_metadata(src: uint64_t, lam: c_ulong) -> uint64_t {
    let metadata: uint64_t;

    srand(time(null_mut()) as c_uint);

    match lam {
        LAM_U57_BITS => {
            /* Set metadata in bits 62:57 */
            /* Get a random non-zero value as metadata */
            metadata = (((rand() as c_ulong % ((1_u64 << LAM_U57_BITS) as c_ulong - 1) + 1) as uint64_t) << 57)
                | (src & !LAM_U57_MASK);
        }
        _ => {
            metadata = src;
        }
    }

    metadata
}

/*
 * Set metadata in user pointer, compare new pointer with original pointer.
 * both pointers should point to the same address.
 *
 * @return:
 * 0: value on the pointer with metadata and value on original are same
 * 1: not same.
 */
unsafe fn handle_lam_test(src: *mut c_void, lam: c_uint) -> c_int {
    let ptr: *mut c_char;

    strcpy(src as *mut c_char, c"USER POINTER".as_ptr());

    ptr = set_metadata(src as uint64_t, lam as c_ulong) as *mut c_char;
    if src == ptr as *mut c_void {
        return 0;
    }

    /* Copy a string into the pointer with metadata */
    strcpy(ptr, c"METADATA POINTER".as_ptr());

    (strcmp(src as *mut c_char, ptr) != 0) as c_int
}

unsafe extern "C" fn handle_max_bits(_test: *mut testcases) -> c_int {
    let mut exp_bits = get_default_tag_bits();
    let mut bits: c_ulong = 0;

    if exp_bits != LAM_NONE {
        exp_bits = LAM_U57_BITS;
    }

    /* Get LAM max tag bits */
    if syscall(SYS_arch_prctl, ARCH_GET_MAX_TAG_BITS, &mut bits as *mut c_ulong) == -1 {
        return 1;
    }

    (exp_bits != bits) as c_int
}

/*
 * Test lam feature through dereference pointer get from malloc.
 * @return 0: Pass test. 1: Get failure during test 2: Get SIGSEGV
 */
unsafe extern "C" fn handle_malloc(test: *mut testcases) -> c_int {
    let mut ptr: *mut c_char = null_mut();
    let mut ret = 0;

    if (*test).later == 0 && (*test).lam != 0 {
        if set_lam((*test).lam) == -1 {
            return 1;
        }
    }

    ptr = malloc(MALLOC_LEN) as *mut c_char;
    if ptr.is_null() {
        perror(c"malloc() failure\n".as_ptr());
        return 1;
    }

    /* Set signal handler */
    if sigsetjmp((&raw mut segv_env) as *mut c_long, 1) == 0 {
        signal(SIGSEGV, segv_handler);
        ret = handle_lam_test(ptr as *mut c_void, (*test).lam as c_uint);
    } else {
        ret = 2;
    }

    if (*test).later != 0 && (*test).lam != 0 {
        if set_lam((*test).lam) == -1 && ret == 0 {
            ret = 1;
        }
    }

    free(ptr as *mut c_void);
    ret
}

unsafe extern "C" fn handle_mmap(test: *mut testcases) -> c_int {
    let flags: c_uint = (MAP_PRIVATE | MAP_ANONYMOUS | MAP_FIXED) as c_uint;
    let mut ret = 0;

    if (*test).later == 0 && (*test).lam != 0 {
        if set_lam((*test).lam) != 0 {
            return 1;
        }
    }

    let ptr = mmap((*test).addr as *mut c_void, PAGE_SIZE, PROT_READ | PROT_WRITE,
                   flags as c_int, -1, 0);
    if ptr == MAP_FAILED() {
        if (*test).addr == HIGH_ADDR {
            if la57_enabled() == 0 {
                return 3; /* unsupport LA57 */
            }
        }
        return 1;
    }

    if (*test).later != 0 && (*test).lam != 0 {
        if set_lam((*test).lam) != 0 {
            ret = 1;
        }
    }

    if ret == 0 {
        if sigsetjmp((&raw mut segv_env) as *mut c_long, 1) == 0 {
            signal(SIGSEGV, segv_handler);
            ret = handle_lam_test(ptr, (*test).lam as c_uint);
        } else {
            ret = 2;
        }
    }

    munmap(ptr, PAGE_SIZE);
    ret
}

unsafe extern "C" fn handle_syscall(test: *mut testcases) -> c_int {
    let mut unme: utsname = zeroed();
    let pu: *mut utsname;
    let mut ret = 0;

    if (*test).later == 0 && (*test).lam != 0 {
        if set_lam((*test).lam) != 0 {
            return 1;
        }
    }

    if sigsetjmp((&raw mut segv_env) as *mut c_long, 1) == 0 {
        signal(SIGSEGV, segv_handler);
        pu = set_metadata((&mut unme as *mut utsname) as uint64_t, (*test).lam) as *mut utsname;
        ret = uname(pu);
        if ret < 0 {
            ret = 1;
        }
    } else {
        ret = 2;
    }

    if (*test).later != 0 && (*test).lam != 0 {
        if set_lam((*test).lam) != -1 && ret == 0 {
            ret = 1;
        }
    }

    ret
}

unsafe extern "C" fn get_user_syscall(test: *mut testcases) -> c_int {
    let mut ptr_address: uint64_t;
    let mut bitmask: uint64_t;
    let fd: c_int;
    let mut ret = 0;
    let mut ptr: *mut c_void;

    if la57_enabled() != 0 {
        bitmask = L5_SIGN_EXT_MASK;
        ptr_address = HIGH_ADDR;
    } else {
        bitmask = L4_SIGN_EXT_MASK;
        ptr_address = LOW_ADDR;
    }

    ptr = mmap(ptr_address as *mut c_void, PAGE_SIZE, PROT_READ | PROT_WRITE,
               MAP_PRIVATE | MAP_ANONYMOUS | MAP_FIXED, -1, 0);

    if ptr == MAP_FAILED() {
        perror(c"failed to map byte to pass into get_user".as_ptr());
        return 1;
    }

    if set_lam((*test).lam) != 0 {
        ret = 2;
        munmap(ptr, PAGE_SIZE);
        return ret;
    }

    fd = memfd_create(c"lam_ioctl".as_ptr(), 0);
    if fd == -1 {
        munmap(ptr, PAGE_SIZE);
        exit(EXIT_FAILURE);
    }

    match (*test).later {
        GET_USER_USER => {
            /* Control group - properly tagged user pointer */
            ptr = set_metadata(ptr as uint64_t, (*test).lam) as *mut c_void;
        }
        GET_USER_KERNEL_TOP => {
            /* Kernel address with top bit cleared */
            bitmask &= bitmask >> 1;
            ptr = ((ptr as uint64_t) | bitmask) as *mut c_void;
        }
        GET_USER_KERNEL_BOT => {
            /* Kernel address with bottom sign-extension bit cleared */
            bitmask &= bitmask << 1;
            ptr = ((ptr as uint64_t) | bitmask) as *mut c_void;
        }
        GET_USER_KERNEL => {
            /* Try to pass a kernel address */
            ptr = ((ptr as uint64_t) | bitmask) as *mut c_void;
        }
        _ => {
            printf(c"Invalid test case value passed!\n".as_ptr());
        }
    }

    /*
     * Use FIOASYNC ioctl because it utilizes get_user() internally and is
     * very non-invasive to the system. Pass differently tagged pointers to
     * get_user() in order to verify that valid user pointers are going
     * through and invalid kernel/non-canonical pointers are not.
     */
    if ioctl(fd, FIOASYNC, ptr) != 0 {
        ret = 1;
    }

    close(fd);
    munmap(ptr, PAGE_SIZE);
    ret
}

unsafe extern "C" fn sys_uring_setup(entries: c_uint, p: *mut io_uring_params) -> c_int {
    syscall(__NR_io_uring_setup, entries, p) as c_int
}

unsafe extern "C" fn sys_uring_enter(fd: c_int, to: c_uint, min: c_uint, flags: c_uint) -> c_int {
    syscall(__NR_io_uring_enter, fd, to, min, flags, null::<c_void>(), 0) as c_int
}

/* Init submission queue and completion queue */
unsafe extern "C" fn mmap_io_uring(p: io_uring_params, s: *mut io_ring) -> c_int {
    let sring = &mut (*s).sq_ring as *mut io_uring_queue;
    let cring = &mut (*s).cq_ring as *mut io_uring_queue;

    (*sring).ring_sz = p.sq_off.array as usize + p.sq_entries as usize * size_of::<c_uint>();
    (*cring).ring_sz = p.cq_off.cqes as usize + p.cq_entries as usize * size_of::<io_uring_cqe>();

    if (p.features & IORING_FEAT_SINGLE_MMAP) != 0 {
        if (*cring).ring_sz > (*sring).ring_sz {
            (*sring).ring_sz = (*cring).ring_sz;
        }
        (*cring).ring_sz = (*sring).ring_sz;
    }

    let sq_ptr = mmap(null_mut(), (*sring).ring_sz, PROT_READ | PROT_WRITE,
                      MAP_SHARED | MAP_POPULATE, (*s).ring_fd, IORING_OFF_SQ_RING);

    if sq_ptr == MAP_FAILED() {
        perror(c"sub-queue!".as_ptr());
        return 1;
    }

    let mut cq_ptr = sq_ptr;

    if (p.features & IORING_FEAT_SINGLE_MMAP) == 0 {
        cq_ptr = mmap(null_mut(), (*cring).ring_sz, PROT_READ | PROT_WRITE,
                      MAP_SHARED | MAP_POPULATE, (*s).ring_fd, IORING_OFF_CQ_RING);
        if cq_ptr == MAP_FAILED() {
            perror(c"cpl-queue!".as_ptr());
            munmap(sq_ptr, (*sring).ring_sz);
            return 1;
        }
    }

    (*sring).head = (sq_ptr as *mut u8).add(p.sq_off.head as usize) as *mut c_uint;
    (*sring).tail = (sq_ptr as *mut u8).add(p.sq_off.tail as usize) as *mut c_uint;
    (*sring).ring_mask = (sq_ptr as *mut u8).add(p.sq_off.ring_mask as usize) as *mut c_uint;
    (*sring).ring_entries = (sq_ptr as *mut u8).add(p.sq_off.ring_entries as usize) as *mut c_uint;
    (*sring).flags = (sq_ptr as *mut u8).add(p.sq_off.flags as usize) as *mut c_uint;
    (*sring).array = (sq_ptr as *mut u8).add(p.sq_off.array as usize) as *mut c_uint;

    /* Map a queue as mem map */
    (*s).sq_ring.queue.sqes = mmap(null_mut(), p.sq_entries as usize * size_of::<io_uring_sqe>(),
                                   PROT_READ | PROT_WRITE, MAP_SHARED | MAP_POPULATE,
                                   (*s).ring_fd, IORING_OFF_SQES) as *mut io_uring_sqe;
    if (*s).sq_ring.queue.sqes as *mut c_void == MAP_FAILED() {
        munmap(sq_ptr, (*sring).ring_sz);
        if sq_ptr != cq_ptr {
            ksft_print_msg(c"failed to mmap uring queue!".as_ptr());
            munmap(cq_ptr, (*cring).ring_sz);
            return 1;
        }
    }

    (*cring).head = (cq_ptr as *mut u8).add(p.cq_off.head as usize) as *mut c_uint;
    (*cring).tail = (cq_ptr as *mut u8).add(p.cq_off.tail as usize) as *mut c_uint;
    (*cring).ring_mask = (cq_ptr as *mut u8).add(p.cq_off.ring_mask as usize) as *mut c_uint;
    (*cring).ring_entries = (cq_ptr as *mut u8).add(p.cq_off.ring_entries as usize) as *mut c_uint;
    (*cring).queue.cqes = (cq_ptr as *mut u8).add(p.cq_off.cqes as usize) as *mut io_uring_cqe;

    0
}

/* Init io_uring queues */
unsafe extern "C" fn setup_io_uring(s: *mut io_ring) -> c_int {
    let mut para: io_uring_params = zeroed();

    memset(&mut para as *mut _ as *mut c_void, 0, size_of::<io_uring_params>());
    (*s).ring_fd = sys_uring_setup(URING_QUEUE_SZ, &mut para);
    if (*s).ring_fd < 0 {
        return 1;
    }

    mmap_io_uring(para, s)
}

unsafe fn barrier() {
    core::sync::atomic::compiler_fence(core::sync::atomic::Ordering::SeqCst);
}

unsafe fn file_iovec(fi: *mut file_io, index: usize) -> *mut iovec {
    ((*fi).iovecs.as_ptr() as *mut iovec).add(index)
}

/*
 * Get data from completion queue. the data buffer saved the file data
 * return 0: success; others: error;
 */
unsafe extern "C" fn handle_uring_cq(s: *mut io_ring) -> c_int {
    let mut fi: *mut file_io = null_mut();
    let cring = &mut (*s).cq_ring as *mut io_uring_queue;
    let mut cqe: *mut io_uring_cqe;
    let mut head: c_uint;
    let mut len: off_t = 0;

    head = *(*cring).head;

    loop {
        barrier();
        if head == *(*cring).tail {
            break;
        }
        /* Get the entry */
        cqe = (*cring).queue.cqes.add((head & *(*s).cq_ring.ring_mask) as usize);
        fi = (*cqe).user_data as *mut file_io;
        if (*cqe).res < 0 {
            break;
        }

        let blocks: c_int = (((*fi).file_sz + URING_BLOCK_SZ - 1) / URING_BLOCK_SZ) as c_int;

        for i in 0..blocks {
            len += (*file_iovec(fi, i as usize)).iov_len as off_t;
        }

        head = head.wrapping_add(1);
    }

    *(*cring).head = head;
    barrier();

    (len != (*fi).file_sz) as c_int
}

/*
 * Submit squeue. specify via IORING_OP_READV.
 * the buffer need to be set metadata according to LAM mode
 */
unsafe extern "C" fn handle_uring_sq(ring: *mut io_ring, fi: *mut file_io, lam: c_ulong) -> c_int {
    let file_fd = (*fi).file_fd;
    let sring = &mut (*ring).sq_ring as *mut io_uring_queue;
    let mut cur_block: c_uint = 0;
    let mut tail: c_uint;
    let mut next_tail: c_uint;
    let sqe: *mut io_uring_sqe;

    let mut remain: off_t = (*fi).file_sz;
    let blocks: c_int = ((remain + URING_BLOCK_SZ - 1) / URING_BLOCK_SZ) as c_int;

    while remain != 0 {
        let mut bytes = remain;
        let mut buf: *mut c_void = null_mut();

        if bytes > URING_BLOCK_SZ {
            bytes = URING_BLOCK_SZ;
        }

        (*file_iovec(fi, cur_block as usize)).iov_len = bytes as size_t;

        if posix_memalign(&mut buf, URING_BLOCK_SZ as size_t, URING_BLOCK_SZ as size_t) != 0 {
            return 1;
        }

        (*file_iovec(fi, cur_block as usize)).iov_base = set_metadata(buf as uint64_t, lam) as *mut c_void;
        remain -= bytes;
        cur_block += 1;
    }

    next_tail = *(*sring).tail;
    tail = next_tail;
    next_tail = next_tail.wrapping_add(1);

    barrier();

    let index = tail & *(*ring).sq_ring.ring_mask;

    sqe = (*ring).sq_ring.queue.sqes.add(index as usize);
    (*sqe).fd = file_fd;
    (*sqe).flags = 0;
    (*sqe).opcode = IORING_OP_READV;
    (*sqe).addr = (*fi).iovecs.as_ptr() as c_ulong as u64;
    (*sqe).len = blocks as u32;
    (*sqe).off = 0;
    (*sqe).user_data = fi as uint64_t;

    *(*sring).array.add(index as usize) = index;
    tail = next_tail;

    if *(*sring).tail != tail {
        *(*sring).tail = tail;
        barrier();
    }

    if sys_uring_enter((*ring).ring_fd, 1, 1, IORING_ENTER_GETEVENTS) < 0 {
        return 1;
    }

    0
}

/*
 * Test LAM in async I/O and io_uring, read current binery through io_uring
 * Set metadata in pointers to iovecs buffer.
 */
unsafe extern "C" fn do_uring(lam: c_ulong) -> c_int {
    let mut ring: *mut io_ring;
    let fi: *mut file_io;
    let mut st: stat = zeroed();
    let mut ret = 1;
    let mut path: [c_char; PATH_MAX] = [0; PATH_MAX];

    /* get current process path */
    if readlink(c"/proc/self/exe".as_ptr(), path.as_mut_ptr(), PATH_MAX - 1) <= 0 {
        return 1;
    }

    let file_fd = open(path.as_ptr(), O_RDONLY);

    if file_fd < 0 {
        return 1;
    }

    if fstat(file_fd, &mut st) < 0 {
        close(file_fd);
        return ret;
    }

    let file_sz: off_t = *(core::ptr::addr_of!(st) as *const off_t).add(6);
    let blocks: c_int = ((file_sz + URING_BLOCK_SZ - 1) / URING_BLOCK_SZ) as c_int;

    fi = malloc(size_of::<file_io>() + size_of::<iovec>() * blocks as usize) as *mut file_io;
    if fi.is_null() {
        close(file_fd);
        return ret;
    }

    (*fi).file_sz = file_sz;
    (*fi).file_fd = file_fd;

    ring = malloc(size_of::<io_ring>()) as *mut io_ring;
    if ring.is_null() {
        free(fi as *mut c_void);
        close(file_fd);
        return ret;
    }

    memset(ring as *mut c_void, 0, size_of::<io_ring>());

    if setup_io_uring(ring) == 0 {
        if handle_uring_sq(ring, fi, lam) == 0 {
            ret = handle_uring_cq(ring);
        }
    }

    free(ring as *mut c_void);

    for i in 0..blocks {
        let iov = file_iovec(fi, i as usize);
        if !(*iov).iov_base.is_null() {
            let mut addr = (*iov).iov_base as uint64_t;

            match lam {
                LAM_U57_BITS => {
                    /* Clear bits 62:57 */
                    addr &= !LAM_U57_MASK;
                }
                _ => {}
            }
            free(addr as *mut c_void);
            (*iov).iov_base = null_mut();
        }
    }

    free(fi as *mut c_void);
    close(file_fd);

    ret
}

unsafe extern "C" fn handle_uring(test: *mut testcases) -> c_int {
    let mut ret = 0;

    if (*test).later == 0 && (*test).lam != 0 {
        if set_lam((*test).lam) != 0 {
            return 1;
        }
    }

    if sigsetjmp((&raw mut segv_env) as *mut c_long, 1) == 0 {
        signal(SIGSEGV, segv_handler);
        ret = do_uring((*test).lam);
    } else {
        ret = 2;
    }

    ret
}

unsafe fn fork_test(test: *mut testcases) -> c_int {
    let ret: c_int;
    let mut child_ret: c_int = 0;
    let pid = fork();

    if pid < 0 {
        perror(c"Fork failed.".as_ptr());
        ret = 1;
    } else if pid == 0 {
        ret = ((*test).test_func.unwrap())(test);
        exit(ret);
    } else {
        wait(&mut child_ret);
        ret = WEXITSTATUS(child_ret);
    }

    ret
}

unsafe extern "C" fn handle_execve(test: *mut testcases) -> c_int {
    let ret: c_int;
    let mut child_ret: c_int = 0;
    let lam = (*test).lam;
    let pid = fork();

    if pid < 0 {
        perror(c"Fork failed.".as_ptr());
        ret = 1;
    } else if pid == 0 {
        let mut path: [c_char; PATH_MAX] = [0; PATH_MAX];

        /* Set LAM mode in parent process */
        if set_lam(lam) != 0 {
            return 1;
        }

        /* Get current binary's path and the binary was run by execve */
        if readlink(c"/proc/self/exe".as_ptr(), path.as_mut_ptr(), PATH_MAX - 1) <= 0 {
            exit(-1);
        }

        /* run binary to get LAM mode and return to parent process */
        if execlp(path.as_ptr(), path.as_ptr(), c"-t 0x0".as_ptr(), null::<c_char>()) < 0 {
            perror(c"error on exec".as_ptr());
            exit(-1);
        }
        ret = 0;
    } else {
        wait(&mut child_ret);
        ret = WEXITSTATUS(child_ret);
        if ret != LAM_NONE as c_int {
            return 1;
        }
    }

    0
}

unsafe extern "C" fn handle_inheritance(test: *mut testcases) -> c_int {
    let ret: c_int;
    let mut child_ret: c_int = 0;
    let lam = (*test).lam as c_int;
    let pid: pid_t;

    /* Set LAM mode in parent process */
    if set_lam(lam as c_ulong) != 0 {
        return 1;
    }

    pid = fork();
    if pid < 0 {
        perror(c"Fork failed.".as_ptr());
        return 1;
    } else if pid == 0 {
        /* Set LAM mode in parent process */
        let child_lam = get_lam();
        exit(child_lam);
    } else {
        wait(&mut child_ret);
        ret = WEXITSTATUS(child_ret);

        if lam != ret {
            return 1;
        }
    }

    0
}

unsafe extern "C" fn thread_fn_get_lam(_arg: *mut c_void) -> c_int {
    get_lam()
}

unsafe extern "C" fn thread_fn_set_lam(arg: *mut c_void) -> c_int {
    let test = arg as *mut testcases;
    set_lam((*test).lam)
}

unsafe extern "C" fn handle_thread(test: *mut testcases) -> c_int {
    let mut stack: [c_char; STACK_SIZE] = [0; STACK_SIZE];
    let ret: c_int;
    let mut child_ret: c_int = 0;
    let mut lam = 0;
    let pid: pid_t;

    /* Set LAM mode in parent process */
    if (*test).later == 0 {
        lam = (*test).lam as c_int;
        if set_lam(lam as c_ulong) != 0 {
            return 1;
        }
    }

    pid = clone(thread_fn_get_lam,
                stack.as_mut_ptr().add(STACK_SIZE) as *mut c_void,
                SIGCHLD | CLONE_FILES | CLONE_FS | CLONE_VM, null_mut());
    if pid < 0 {
        perror(c"Clone failed.".as_ptr());
        return 1;
    }

    waitpid(pid, &mut child_ret, 0);
    ret = WEXITSTATUS(child_ret);

    if lam != ret {
        return 1;
    }

    if (*test).later != 0 {
        if set_lam((*test).lam) != 0 {
            return 1;
        }
    }

    0
}

unsafe extern "C" fn handle_thread_enable(test: *mut testcases) -> c_int {
    let mut stack: [c_char; STACK_SIZE] = [0; STACK_SIZE];
    let ret: c_int;
    let mut child_ret: c_int = 0;
    let lam = (*test).lam as c_int;
    let pid: pid_t;

    pid = clone(thread_fn_set_lam,
                stack.as_mut_ptr().add(STACK_SIZE) as *mut c_void,
                SIGCHLD | CLONE_FILES | CLONE_FS | CLONE_VM, test as *mut c_void);
    if pid < 0 {
        perror(c"Clone failed.".as_ptr());
        return 1;
    }

    waitpid(pid, &mut child_ret, 0);
    ret = WEXITSTATUS(child_ret);

    if lam != ret {
        return 1;
    }

    0
}

unsafe fn run_test(test: *mut testcases, count: usize) {
    let mut ret: c_int;

    for i in 0..count {
        let t = test.add(i);

        /* fork a process to run test case */
        tests_cnt += 1;
        ret = fork_test(t);

        /* return 3 is not support LA57, the case should be skipped */
        if ret == 3 {
            ksft_test_result_skip(c"%s".as_ptr(), (*t).msg);
            continue;
        }

        if ret != 0 {
            ret = ((*t).expected == ret) as c_int;
        } else {
            ret = ((*t).expected == 0) as c_int;
        }

        ksft_test_result(ret, c"%s".as_ptr(), (*t).msg);
    }
}

static mut uring_cases: [testcases; 2] = [
    testcases { later: 0, expected: 0, lam: LAM_U57_BITS, addr: 0, cmd: 0, test_func: Some(handle_uring), msg: c"URING: LAM_U57. Dereferencing pointer with metadata\n".as_ptr() },
    testcases { later: 1, expected: 1, lam: LAM_U57_BITS, addr: 0, cmd: 0, test_func: Some(handle_uring), msg: c"URING:[Negative] Disable LAM. Dereferencing pointer with metadata.\n".as_ptr() },
];

static mut malloc_cases: [testcases; 2] = [
    testcases { later: 0, expected: 0, lam: LAM_U57_BITS, addr: 0, cmd: 0, test_func: Some(handle_malloc), msg: c"MALLOC: LAM_U57. Dereferencing pointer with metadata\n".as_ptr() },
    testcases { later: 1, expected: 2, lam: LAM_U57_BITS, addr: 0, cmd: 0, test_func: Some(handle_malloc), msg: c"MALLOC:[Negative] Disable LAM. Dereferencing pointer with metadata.\n".as_ptr() },
];

static mut bits_cases: [testcases; 1] = [
    testcases { later: 0, expected: 0, lam: 0, addr: 0, cmd: 0, test_func: Some(handle_max_bits), msg: c"BITS: Check default tag bits\n".as_ptr() },
];

static mut syscall_cases: [testcases; 6] = [
    testcases { later: 0, expected: 0, lam: LAM_U57_BITS, addr: 0, cmd: 0, test_func: Some(handle_syscall), msg: c"SYSCALL: LAM_U57. syscall with metadata\n".as_ptr() },
    testcases { later: 1, expected: 1, lam: LAM_U57_BITS, addr: 0, cmd: 0, test_func: Some(handle_syscall), msg: c"SYSCALL:[Negative] Disable LAM. Dereferencing pointer with metadata.\n".as_ptr() },
    testcases { later: GET_USER_USER, expected: 0, lam: LAM_U57_BITS, addr: 0, cmd: 0, test_func: Some(get_user_syscall), msg: c"GET_USER: get_user() and pass a properly tagged user pointer.\n".as_ptr() },
    testcases { later: GET_USER_KERNEL_TOP, expected: 1, lam: LAM_U57_BITS, addr: 0, cmd: 0, test_func: Some(get_user_syscall), msg: c"GET_USER:[Negative] get_user() with a kernel pointer and the top bit cleared.\n".as_ptr() },
    testcases { later: GET_USER_KERNEL_BOT, expected: 1, lam: LAM_U57_BITS, addr: 0, cmd: 0, test_func: Some(get_user_syscall), msg: c"GET_USER:[Negative] get_user() with a kernel pointer and the bottom sign-extension bit cleared.\n".as_ptr() },
    testcases { later: GET_USER_KERNEL, expected: 1, lam: LAM_U57_BITS, addr: 0, cmd: 0, test_func: Some(get_user_syscall), msg: c"GET_USER:[Negative] get_user() and pass a kernel pointer.\n".as_ptr() },
];

static mut mmap_cases: [testcases; 3] = [
    testcases { later: 1, expected: 0, lam: LAM_U57_BITS, addr: HIGH_ADDR, cmd: 0, test_func: Some(handle_mmap), msg: c"MMAP: First mmap high address, then set LAM_U57.\n".as_ptr() },
    testcases { later: 0, expected: 0, lam: LAM_U57_BITS, addr: HIGH_ADDR, cmd: 0, test_func: Some(handle_mmap), msg: c"MMAP: First LAM_U57, then High address.\n".as_ptr() },
    testcases { later: 0, expected: 0, lam: LAM_U57_BITS, addr: LOW_ADDR, cmd: 0, test_func: Some(handle_mmap), msg: c"MMAP: First LAM_U57, then Low address.\n".as_ptr() },
];

static mut inheritance_cases: [testcases; 5] = [
    testcases { later: 0, expected: 0, lam: LAM_U57_BITS, addr: 0, cmd: 0, test_func: Some(handle_inheritance), msg: c"FORK: LAM_U57, child process should get LAM mode same as parent\n".as_ptr() },
    testcases { later: 0, expected: 0, lam: LAM_U57_BITS, addr: 0, cmd: 0, test_func: Some(handle_thread), msg: c"THREAD: LAM_U57, child thread should get LAM mode same as parent\n".as_ptr() },
    testcases { later: 0, expected: 1, lam: LAM_U57_BITS, addr: 0, cmd: 0, test_func: Some(handle_thread_enable), msg: c"THREAD: [NEGATIVE] Enable LAM in child.\n".as_ptr() },
    testcases { later: 1, expected: 1, lam: LAM_U57_BITS, addr: 0, cmd: 0, test_func: Some(handle_thread), msg: c"THREAD: [NEGATIVE] Enable LAM in parent after thread created.\n".as_ptr() },
    testcases { later: 0, expected: 0, lam: LAM_U57_BITS, addr: 0, cmd: 0, test_func: Some(handle_execve), msg: c"EXECVE: LAM_U57, child process should get disabled LAM mode\n".as_ptr() },
];

unsafe fn cmd_help() {
    printf(c"usage: lam [-h] [-t test list]\n".as_ptr());
    printf(c"\t-t test list: run tests specified in the test list, default:0x%x\n".as_ptr(), TEST_MASK);
    printf(c"\t\t0x1:malloc; 0x2:max_bits; 0x4:mmap; 0x8:syscall; 0x10:io_uring; 0x20:inherit;\n".as_ptr());
    printf(c"\t-h: help\n".as_ptr());
}

/* Check for file existence */
unsafe extern "C" fn file_Exists(fileName: *const c_char) -> uint8_t {
    let mut buffer: stat = zeroed();
    let ret: uint8_t = (stat(fileName, &mut buffer) == 0) as uint8_t;
    ret
}

/* Sysfs idxd files */
static dsa_configs: [*const c_char; 10] = [
    c"echo 1 > /sys/bus/dsa/devices/dsa0/wq0.1/group_id".as_ptr(),
    c"echo shared > /sys/bus/dsa/devices/dsa0/wq0.1/mode".as_ptr(),
    c"echo 10 > /sys/bus/dsa/devices/dsa0/wq0.1/priority".as_ptr(),
    c"echo 16 > /sys/bus/dsa/devices/dsa0/wq0.1/size".as_ptr(),
    c"echo 15 > /sys/bus/dsa/devices/dsa0/wq0.1/threshold".as_ptr(),
    c"echo user > /sys/bus/dsa/devices/dsa0/wq0.1/type".as_ptr(),
    c"echo MyApp1 > /sys/bus/dsa/devices/dsa0/wq0.1/name".as_ptr(),
    c"echo 1 > /sys/bus/dsa/devices/dsa0/engine0.1/group_id".as_ptr(),
    c"echo dsa0 > /sys/bus/dsa/drivers/idxd/bind".as_ptr(),
    /* bind files and devices, generated a device file in /dev */
    c"echo wq0.1 > /sys/bus/dsa/drivers/user/bind".as_ptr(),
];

/* DSA device file */
static dsaDeviceFile: *const c_char = c"/dev/dsa/wq0.1".as_ptr();
/* file for io*/
static dsaPasidEnable: *const c_char = c"/sys/bus/dsa/devices/dsa0/pasid_enabled".as_ptr();

/*
 * DSA depends on kernel cmdline "intel_iommu=on,sm_on"
 * return pasid_enabled (0: disable 1:enable)
 */
unsafe extern "C" fn Check_DSA_Kernel_Setting() -> c_int {
    let mut command: [c_char; 256] = [0; 256];
    let mut buf: [c_char; 256] = [0; 256];
    let mut ptr: *mut c_char = null_mut();
    let mut rv = -1;

    snprintf(command.as_mut_ptr(), command.len() - 1, c"cat %s".as_ptr(), dsaPasidEnable);

    let cmd = popen(command.as_ptr(), c"r".as_ptr());

    if !cmd.is_null() {
        while !fgets(buf.as_mut_ptr(), (buf.len() - 1) as c_int, cmd).is_null() {}
        pclose(cmd);
        rv = strtol(buf.as_ptr(), &mut ptr, 16) as c_int;
    }

    rv
}

/*
 * Config DSA's sysfs files as shared DSA's WQ.
 * Generated a device file /dev/dsa/wq0.1
 * Return:  0 OK; 1 Failed; 3 Skip(SVA disabled).
 */
unsafe extern "C" fn Dsa_Init_Sysfs() -> c_int {
    let len: uint = dsa_configs.len() as uint;
    let p = dsa_configs.as_ptr();

    if file_Exists(dsaDeviceFile) == 1 {
        return 0;
    }

    /* check the idxd driver */
    if file_Exists(dsaPasidEnable) != 1 {
        printf(c"Please make sure idxd driver was loaded\n".as_ptr());
        return 3;
    }

    /* Check SVA feature */
    if Check_DSA_Kernel_Setting() != 1 {
        printf(c"Please enable SVA.(Add intel_iommu=on,sm_on in kernel cmdline)\n".as_ptr());
        return 3;
    }

    /* Check the idxd device file on /dev/dsa/ */
    for i in 0..len {
        if system(*p.add(i as usize)) != 0 {
            return 1;
        }
    }

    /* After config, /dev/dsa/wq0.1 should be generated */
    (file_Exists(dsaDeviceFile) != 1) as c_int
}

/*
 * Open DSA device file, triger API: iommu_sva_alloc_pasid
 */
unsafe extern "C" fn allocate_dsa_pasid() -> *mut c_void {
    let fd: c_int;
    let wq: *mut c_void;

    fd = open(dsaDeviceFile, O_RDWR);
    if fd < 0 {
        perror(c"open".as_ptr());
        return MAP_FAILED();
    }

    wq = mmap(null_mut(), 0x1000, PROT_WRITE, MAP_SHARED | MAP_POPULATE, fd, 0);
    close(fd);
    if wq == MAP_FAILED() {
        perror(c"mmap".as_ptr());
    }

    wq
}

unsafe extern "C" fn set_force_svm() -> c_int {
    let ret: c_int = syscall(SYS_arch_prctl, ARCH_FORCE_TAGGED_SVA) as c_int;
    ret
}

unsafe extern "C" fn handle_pasid(test: *mut testcases) -> c_int {
    let mut tmp: uint = (*test).cmd as uint;
    let mut runed: uint = 0x0;
    let mut ret: c_int = 0;
    let mut wq: *mut c_void = null_mut();

    ret = Dsa_Init_Sysfs();
    if ret != 0 {
        return ret;
    }

    for _i in 0..3 {
        let mut err = 0;

        if (tmp & 0x1) != 0 {
            /* run set lam mode*/
            if (runed & 0x1) == 0 {
                err = set_lam(LAM_U57_BITS);
                runed |= 0x1;
            } else {
                err = 1;
            }
        } else if (tmp & 0x4) != 0 {
            /* run force svm */
            if (runed & 0x4) == 0 {
                err = set_force_svm();
                runed |= 0x4;
            } else {
                err = 1;
            }
        } else if (tmp & 0x2) != 0 {
            /* run allocate pasid */
            if (runed & 0x2) == 0 {
                runed |= 0x2;
                wq = allocate_dsa_pasid();
                if wq == MAP_FAILED() {
                    err = 1;
                }
            } else {
                err = 1;
            }
        }

        ret += err;
        if ret > 0 {
            break;
        }

        tmp >>= 4;
    }

    if wq != MAP_FAILED() && !wq.is_null() {
        if munmap(wq, 0x1000) != 0 {
            printf(c"munmap failed %d\n".as_ptr(), errno);
        }
    }

    if runed != 0x7 {
        ret = 1;
    }

    (ret != 0) as c_int
}

/*
 * Pasid test depends on idxd and SVA, kernel should enable iommu and sm.
 * command line(intel_iommu=on,sm_on)
 */
static mut pasid_cases: [testcases; 6] = [
    testcases { later: 0, expected: 1, lam: 0, addr: 0, cmd: PAS_CMD(LAM_CMD_BIT, PAS_CMD_BIT, SVA_CMD_BIT), test_func: Some(handle_pasid), msg: c"PASID: [Negative] Execute LAM, PASID, SVA in sequence\n".as_ptr() },
    testcases { later: 0, expected: 0, lam: 0, addr: 0, cmd: PAS_CMD(LAM_CMD_BIT, SVA_CMD_BIT, PAS_CMD_BIT), test_func: Some(handle_pasid), msg: c"PASID: Execute LAM, SVA, PASID in sequence\n".as_ptr() },
    testcases { later: 0, expected: 1, lam: 0, addr: 0, cmd: PAS_CMD(PAS_CMD_BIT, LAM_CMD_BIT, SVA_CMD_BIT), test_func: Some(handle_pasid), msg: c"PASID: [Negative] Execute PASID, LAM, SVA in sequence\n".as_ptr() },
    testcases { later: 0, expected: 0, lam: 0, addr: 0, cmd: PAS_CMD(PAS_CMD_BIT, SVA_CMD_BIT, LAM_CMD_BIT), test_func: Some(handle_pasid), msg: c"PASID: Execute PASID, SVA, LAM in sequence\n".as_ptr() },
    testcases { later: 0, expected: 0, lam: 0, addr: 0, cmd: PAS_CMD(SVA_CMD_BIT, LAM_CMD_BIT, PAS_CMD_BIT), test_func: Some(handle_pasid), msg: c"PASID: Execute SVA, LAM, PASID in sequence\n".as_ptr() },
    testcases { later: 0, expected: 0, lam: 0, addr: 0, cmd: PAS_CMD(SVA_CMD_BIT, PAS_CMD_BIT, LAM_CMD_BIT), test_func: Some(handle_pasid), msg: c"PASID: Execute SVA, PASID, LAM in sequence\n".as_ptr() },
];

unsafe fn MAP_FAILED() -> *mut c_void {
    (-1_isize) as *mut c_void
}

fn WEXITSTATUS(status: c_int) -> c_int {
    (status >> 8) & 0xff
}

unsafe fn stat_st_size(st: *const stat) -> off_t {
    *(st as *const off_t).add(6)
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn main(argc: c_int, argv: *mut *mut c_char) -> c_int {
    let mut c: c_int;
    let mut tests: c_uint = TEST_MASK;

    tests_cnt = 0;

    if lam_is_available() == 0 {
        return KSFT_SKIP;
    }

    loop {
        c = getopt(argc, argv, c"ht:".as_ptr());
        if c == -1 {
            break;
        }
        match c as u8 as char {
            't' => {
                tests = strtoul(optarg, null_mut(), 16) as c_uint;
                if tests != 0 && (tests & TEST_MASK) == 0 {
                    ksft_print_msg(c"Invalid argument!\n".as_ptr());
                    return -1;
                }
            }
            'h' => {
                cmd_help();
                return 0;
            }
            _ => {
                ksft_print_msg(c"Invalid argument\n".as_ptr());
                return -1;
            }
        }
    }

    /*
     * When tests is 0, it is not a real test case;
     * the option used by test case(execve) to check the lam mode in
     * process generated by execve, the process read back lam mode and
     * check with lam mode in parent process.
     */
    if tests == 0 {
        return get_lam();
    }

    /* Run test cases */
    if (tests & FUNC_MALLOC) != 0 {
        run_test((&raw mut malloc_cases) as *mut testcases, malloc_cases.len());
    }

    if (tests & FUNC_BITS) != 0 {
        run_test((&raw mut bits_cases) as *mut testcases, bits_cases.len());
    }

    if (tests & FUNC_MMAP) != 0 {
        run_test((&raw mut mmap_cases) as *mut testcases, mmap_cases.len());
    }

    if (tests & FUNC_SYSCALL) != 0 {
        run_test((&raw mut syscall_cases) as *mut testcases, syscall_cases.len());
    }

    if (tests & FUNC_URING) != 0 {
        run_test((&raw mut uring_cases) as *mut testcases, uring_cases.len());
    }

    if (tests & FUNC_INHERITE) != 0 {
        run_test((&raw mut inheritance_cases) as *mut testcases, inheritance_cases.len());
    }

    if (tests & FUNC_PASID) != 0 {
        run_test((&raw mut pasid_cases) as *mut testcases, pasid_cases.len());
    }

    ksft_set_plan(tests_cnt as c_uint);

    ksft_exit_pass();
}
