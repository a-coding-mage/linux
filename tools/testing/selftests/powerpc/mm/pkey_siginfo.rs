// SPDX-License-Identifier: GPL-2.0

/*
 * Copyright 2020, Sandipan Das, IBM Corp.
 *
 * Test if the signal information reports the correct memory protection
 * key upon getting a key access violation fault for a page that was
 * attempted to be protected by two different keys from two competing
 * threads at the same time.
 */

// C dependencies: stdio.h, stdlib.h, string.h, signal.h, unistd.h,
// pthread.h, sys/mman.h, and "pkeys.h".

use core::arch::asm;
use core::ffi::{c_char, c_int, c_long, c_uint, c_ulong, c_void};
use core::mem::size_of;
use core::ptr::{null, null_mut, read_volatile, write_volatile};

type size_t = usize;
type sig_atomic_t = c_int;
type pthread_t = c_ulong;

#[repr(C)]
pub struct pthread_attr_t {
    __private: [u8; 0],
}

#[repr(C)]
pub struct pthread_barrier_t {
    __private: [u8; 0],
}

#[repr(C)]
pub struct sigset_t {
    __private: [u8; 0],
}

#[repr(C)]
pub struct siginfo_t {
    pub si_signo: c_int,
    pub si_errno: c_int,
    pub si_code: c_int,
    pub si_addr: *mut c_void,
}

#[repr(C)]
pub struct sigaction {
    pub sa_handler: usize,
    pub sa_sigaction: unsafe extern "C" fn(c_int, *mut siginfo_t, *mut c_void),
    pub sa_mask: sigset_t,
    pub sa_flags: c_int,
    pub sa_restorer: usize,
}

#[repr(C)]
struct region {
    rights: c_ulong,
    base: *mut c_uint,
    size: size_t,
}

const PPC_INST_NOP: c_uint = 0x60000000;
const PPC_INST_BLR: c_uint = 0x4e800020;
const PROT_RWX: c_int = PROT_READ | PROT_WRITE | PROT_EXEC;

const NUM_ITERATIONS: c_int = 1000000;

// Constants supplied by C headers / pkeys.h.
extern "C" {
    static PROT_READ: c_int;
    static PROT_WRITE: c_int;
    static PROT_EXEC: c_int;
    static MAP_PRIVATE: c_int;
    static MAP_ANONYMOUS: c_int;
    static SIG_SETMASK: c_int;
    static SA_SIGINFO: c_int;
    static SIGSEGV: c_int;
    static SEGV_PKUERR: c_int;
    static PKEY_DISABLE_EXECUTE: c_ulong;
    static PKEY_DISABLE_WRITE: c_ulong;
    static PKEY_DISABLE_ACCESS: c_ulong;
    static PKEY_UNRESTRICTED: c_ulong;
    static NR_PKEYS: c_int;
    static mut MAP_FAILED: *mut c_void;
}

extern "C" {
    fn siginfo_pkey(sinfo: *mut siginfo_t) -> c_int;
    fn sigsafe_err(msg: *const c_char);
    fn _exit(status: c_int) -> !;
    fn getpagesize() -> c_int;
    fn mprotect(addr: *mut c_void, len: size_t, prot: c_int) -> c_int;
    fn pkey_set_rights(pkey: c_int, rights: c_ulong);
    fn gettid() -> c_int;
    fn printf(fmt: *const c_char, ...) -> c_int;
    fn sys_pkey_alloc(flags: c_uint, rights: c_ulong) -> c_int;
    fn sys_pkey_mprotect(addr: *mut c_void, len: size_t, prot: c_int, pkey: c_int) -> c_int;
    fn sys_pkey_free(pkey: c_int) -> c_int;
    fn pkey_rights(rights: c_ulong) -> *const c_char;
    fn pthread_barrier_wait(barrier: *mut pthread_barrier_t) -> c_int;
    fn rand() -> c_int;
    fn srand(seed: c_uint);
    fn time(tloc: *mut c_long) -> c_long;
    fn pkeys_unsupported() -> c_int;
    fn mmap(
        addr: *mut c_void,
        length: size_t,
        prot: c_int,
        flags: c_int,
        fd: c_int,
        offset: c_long,
    ) -> *mut c_void;
    fn sigprocmask(how: c_int, set: *const sigset_t, oldset: *mut sigset_t) -> c_int;
    fn sigaction(signum: c_int, act: *const sigaction, oldact: *mut sigaction) -> c_int;
    fn pthread_attr_init(attr: *mut pthread_attr_t) -> c_int;
    fn pthread_barrier_init(
        barrier: *mut pthread_barrier_t,
        attr: *const c_void,
        count: c_uint,
    ) -> c_int;
    fn puts(s: *const c_char) -> c_int;
    fn pthread_create(
        thread: *mut pthread_t,
        attr: *const pthread_attr_t,
        start_routine: unsafe extern "C" fn(*mut c_void) -> *mut c_void,
        arg: *mut c_void,
    ) -> c_int;
    fn pthread_join(thread: pthread_t, retval: *mut *mut c_void) -> c_int;
    fn pthread_attr_destroy(attr: *mut pthread_attr_t) -> c_int;
    fn pthread_barrier_destroy(barrier: *mut pthread_barrier_t) -> c_int;
    fn munmap(addr: *mut c_void, length: size_t) -> c_int;
    fn test_harness(test: unsafe extern "C" fn() -> c_int, name: *const c_char) -> c_int;
}

static mut perm_pkey: sig_atomic_t = 0;
static mut rest_pkey: sig_atomic_t = 0;
static mut rights: sig_atomic_t = 0;
static mut fault_count: sig_atomic_t = 0;
static mut fault_addr: *mut c_uint = null_mut();
static mut iteration_barrier: pthread_barrier_t = pthread_barrier_t { __private: [] };

unsafe extern "C" fn segv_handler(_signum: c_int, sinfo: *mut siginfo_t, _ctx: *mut c_void) {
    let pgstart: *mut c_void;
    let pgsize: size_t;
    let pkey: c_int;

    pkey = siginfo_pkey(sinfo);

    /* Check if this fault originated from a pkey access violation */
    if (*sinfo).si_code != SEGV_PKUERR {
        sigsafe_err(c"got a fault for an unexpected reason\n".as_ptr());
        _exit(1);
    }

    /* Check if this fault originated from the expected address */
    if (*sinfo).si_addr != read_volatile(&raw const fault_addr) as *mut c_void {
        sigsafe_err(c"got a fault for an unexpected address\n".as_ptr());
        _exit(1);
    }

    /* Check if this fault originated from the restrictive pkey */
    if pkey != read_volatile(&raw const rest_pkey) {
        sigsafe_err(c"got a fault for an unexpected pkey\n".as_ptr());
        _exit(1);
    }

    /* Check if too many faults have occurred for the same iteration */
    if read_volatile(&raw const fault_count) > 0 {
        sigsafe_err(c"got too many faults for the same address\n".as_ptr());
        _exit(1);
    }

    pgsize = getpagesize() as size_t;
    pgstart = ((read_volatile(&raw const fault_addr) as c_ulong) & !((pgsize - 1) as c_ulong))
        as *mut c_void;

    /*
     * If the current fault occurred due to lack of execute rights,
     * reassociate the page with the exec-only pkey since execute
     * rights cannot be changed directly for the faulting pkey as
     * IAMR is inaccessible from userspace.
     *
     * Otherwise, if the current fault occurred due to lack of
     * read-write rights, change the AMR permission bits for the
     * pkey.
     *
     * This will let the test continue.
     */
    if read_volatile(&raw const rights) as c_ulong == PKEY_DISABLE_EXECUTE
        && mprotect(pgstart, pgsize, PROT_EXEC) != 0
    {
        _exit(1);
    } else {
        pkey_set_rights(pkey, PKEY_UNRESTRICTED);
    }

    write_volatile(
        &raw mut fault_count,
        read_volatile(&raw const fault_count).wrapping_add(1),
    );
}

unsafe extern "C" fn protect(p: *mut c_void) -> *mut c_void {
    let mut rights_local: c_ulong;
    let base: *mut c_uint;
    let size: size_t;
    let tid: c_int;
    let mut i: c_int;

    tid = gettid();
    base = (*(p as *mut region)).base;
    size = (*(p as *mut region)).size;
    FAIL_IF_EXIT!(base.is_null());

    /* No read, write and execute restrictions */
    rights_local = 0;

    printf(
        c"tid %d, pkey permissions are %s\n".as_ptr(),
        tid,
        pkey_rights(rights_local),
    );

    /* Allocate the permissive pkey */
    write_volatile(&raw mut perm_pkey, sys_pkey_alloc(0, rights_local));
    FAIL_IF_EXIT!(read_volatile(&raw const perm_pkey) < 0);

    /*
     * Repeatedly try to protect the common region with a permissive
     * pkey
     */
    i = 0;
    while i < NUM_ITERATIONS {
        /*
         * Wait until the other thread has finished allocating the
         * restrictive pkey or until the next iteration has begun
         */
        pthread_barrier_wait(&raw mut iteration_barrier);

        /* Try to associate the permissive pkey with the region */
        FAIL_IF_EXIT!(sys_pkey_mprotect(
            base as *mut c_void,
            size,
            PROT_RWX,
            read_volatile(&raw const perm_pkey),
        ) != 0);
        i += 1;
    }

    /* Free the permissive pkey */
    sys_pkey_free(read_volatile(&raw const perm_pkey));

    null_mut()
}

unsafe extern "C" fn protect_access(p: *mut c_void) -> *mut c_void {
    let size: size_t;
    let numinsns: size_t;
    let base: *mut c_uint;
    let tid: c_int;
    let mut i: c_int;

    tid = gettid();
    base = (*(p as *mut region)).base;
    size = (*(p as *mut region)).size;
    write_volatile(&raw mut rights, (*(p as *mut region)).rights as sig_atomic_t);
    numinsns = size / size_of::<c_uint>();
    FAIL_IF_EXIT!(base.is_null());

    /* Allocate the restrictive pkey */
    write_volatile(
        &raw mut rest_pkey,
        sys_pkey_alloc(0, read_volatile(&raw const rights) as c_ulong),
    );
    FAIL_IF_EXIT!(read_volatile(&raw const rest_pkey) < 0);

    printf(
        c"tid %d, pkey permissions are %s\n".as_ptr(),
        tid,
        pkey_rights(read_volatile(&raw const rights) as c_ulong),
    );
    printf(
        c"tid %d, %s randomly in range [%p, %p]\n".as_ptr(),
        tid,
        if read_volatile(&raw const rights) as c_ulong == PKEY_DISABLE_EXECUTE {
            c"execute".as_ptr()
        } else if read_volatile(&raw const rights) as c_ulong == PKEY_DISABLE_WRITE {
            c"write".as_ptr()
        } else {
            c"read".as_ptr()
        },
        base,
        base.add(numinsns),
    );

    /*
     * Repeatedly try to protect the common region with a restrictive
     * pkey and read, write or execute from it
     */
    i = 0;
    while i < NUM_ITERATIONS {
        /*
         * Wait until the other thread has finished allocating the
         * permissive pkey or until the next iteration has begun
         */
        pthread_barrier_wait(&raw mut iteration_barrier);

        /* Try to associate the restrictive pkey with the region */
        FAIL_IF_EXIT!(sys_pkey_mprotect(
            base as *mut c_void,
            size,
            PROT_RWX,
            read_volatile(&raw const rest_pkey),
        ) != 0);

        /* Choose a random instruction word address from the region */
        write_volatile(
            &raw mut fault_addr,
            base.add((rand() as size_t) % numinsns),
        );
        write_volatile(&raw mut fault_count, 0);

        match read_volatile(&raw const rights) as c_ulong {
            /* Read protection test */
            x if x == PKEY_DISABLE_ACCESS => {
                /*
                 * Read an instruction word from the region and
                 * verify if it has not been overwritten to
                 * something unexpected
                 */
                let value = read_volatile(read_volatile(&raw const fault_addr));
                FAIL_IF_EXIT!(value != PPC_INST_NOP && value != PPC_INST_BLR);
            }

            /* Write protection test */
            x if x == PKEY_DISABLE_WRITE => {
                /*
                 * Write an instruction word to the region and
                 * verify if the overwrite has succeeded
                 */
                write_volatile(read_volatile(&raw const fault_addr), PPC_INST_BLR);
                FAIL_IF_EXIT!(read_volatile(read_volatile(&raw const fault_addr)) != PPC_INST_BLR);
            }

            /* Execute protection test */
            x if x == PKEY_DISABLE_EXECUTE => {
                /* Jump to the region and execute instructions */
                asm!(
                    "mtctr {0}; bctrl",
                    in(reg) read_volatile(&raw const fault_addr),
                    lateout("ctr") _,
                    lateout("lr") _,
                    options(nostack)
                );
            }

            _ => {}
        }

        /*
         * Restore the restrictions originally imposed by the
         * restrictive pkey as the signal handler would have
         * cleared out the corresponding AMR bits
         */
        pkey_set_rights(
            read_volatile(&raw const rest_pkey),
            read_volatile(&raw const rights) as c_ulong,
        );
        i += 1;
    }

    /* Free restrictive pkey */
    sys_pkey_free(read_volatile(&raw const rest_pkey));

    null_mut()
}

unsafe extern "C" fn reset_pkeys(rights: c_ulong) {
    let mut pkeys: [c_int; NR_PKEYS as usize] = [0; NR_PKEYS as usize];
    let mut i: c_int;

    /* Exhaustively allocate all available pkeys */
    i = 0;
    while i < NR_PKEYS {
        pkeys[i as usize] = sys_pkey_alloc(0, rights);
        i += 1;
    }

    /* Free all allocated pkeys */
    i = 0;
    while i < NR_PKEYS {
        sys_pkey_free(pkeys[i as usize]);
        i += 1;
    }
}

unsafe extern "C" fn test() -> c_int {
    let mut prot_thread: pthread_t = 0;
    let mut pacc_thread: pthread_t = 0;
    let mut act: sigaction = core::mem::zeroed();
    let mut attr: pthread_attr_t = pthread_attr_t { __private: [] };
    let numinsns: size_t;
    let mut r: region = core::mem::zeroed();
    let ret: c_int;
    let mut i: size_t;

    srand(time(null_mut()) as c_uint);
    ret = pkeys_unsupported();
    if ret != 0 {
        return ret;
    }

    /* Allocate the region */
    r.size = getpagesize() as size_t;
    r.base = mmap(
        null_mut(),
        r.size,
        PROT_RWX,
        MAP_PRIVATE | MAP_ANONYMOUS,
        -1,
        0,
    ) as *mut c_uint;
    FAIL_IF!(r.base as *mut c_void == MAP_FAILED);

    /*
     * Fill the region with no-ops with a branch at the end
     * for returning to the caller
     */
    numinsns = r.size / size_of::<c_uint>();
    i = 0;
    while i < numinsns - 1 {
        *r.base.add(i) = PPC_INST_NOP;
        i += 1;
    }
    *r.base.add(i) = PPC_INST_BLR;

    /* Setup SIGSEGV handler */
    act.sa_handler = 0;
    act.sa_sigaction = segv_handler;
    FAIL_IF!(sigprocmask(SIG_SETMASK, null(), &mut act.sa_mask) != 0);
    act.sa_flags = SA_SIGINFO;
    act.sa_restorer = 0;
    FAIL_IF!(sigaction(SIGSEGV, &act, null_mut()) != 0);

    /*
     * For these tests, the parent process should clear all bits of
     * AMR and IAMR, i.e. impose no restrictions, for all available
     * pkeys. This will be the base for the initial AMR and IAMR
     * values for all the test thread pairs.
     *
     * If the AMR and IAMR bits of all available pkeys are cleared
     * before running the tests and a fault is generated when
     * attempting to read, write or execute instructions from a
     * pkey protected region, the pkey responsible for this must be
     * the one from the protect-and-access thread since the other
     * one is fully permissive. Despite that, if the pkey reported
     * by siginfo is not the restrictive pkey, then there must be a
     * kernel bug.
     */
    reset_pkeys(0);

    /* Setup barrier for protect and protect-and-access threads */
    FAIL_IF!(pthread_attr_init(&mut attr) != 0);
    FAIL_IF!(pthread_barrier_init(&raw mut iteration_barrier, null(), 2) != 0);

    /* Setup and start protect and protect-and-read threads */
    puts(c"starting thread pair (protect, protect-and-read)".as_ptr());
    r.rights = PKEY_DISABLE_ACCESS;
    FAIL_IF!(pthread_create(&mut prot_thread, &attr, protect, &mut r as *mut _ as *mut c_void) != 0);
    FAIL_IF!(pthread_create(&mut pacc_thread, &attr, protect_access, &mut r as *mut _ as *mut c_void) != 0);
    FAIL_IF!(pthread_join(prot_thread, null_mut()) != 0);
    FAIL_IF!(pthread_join(pacc_thread, null_mut()) != 0);

    /* Setup and start protect and protect-and-write threads */
    puts(c"starting thread pair (protect, protect-and-write)".as_ptr());
    r.rights = PKEY_DISABLE_WRITE;
    FAIL_IF!(pthread_create(&mut prot_thread, &attr, protect, &mut r as *mut _ as *mut c_void) != 0);
    FAIL_IF!(pthread_create(&mut pacc_thread, &attr, protect_access, &mut r as *mut _ as *mut c_void) != 0);
    FAIL_IF!(pthread_join(prot_thread, null_mut()) != 0);
    FAIL_IF!(pthread_join(pacc_thread, null_mut()) != 0);

    /* Setup and start protect and protect-and-execute threads */
    puts(c"starting thread pair (protect, protect-and-execute)".as_ptr());
    r.rights = PKEY_DISABLE_EXECUTE;
    FAIL_IF!(pthread_create(&mut prot_thread, &attr, protect, &mut r as *mut _ as *mut c_void) != 0);
    FAIL_IF!(pthread_create(&mut pacc_thread, &attr, protect_access, &mut r as *mut _ as *mut c_void) != 0);
    FAIL_IF!(pthread_join(prot_thread, null_mut()) != 0);
    FAIL_IF!(pthread_join(pacc_thread, null_mut()) != 0);

    /* Cleanup */
    FAIL_IF!(pthread_attr_destroy(&mut attr) != 0);
    FAIL_IF!(pthread_barrier_destroy(&raw mut iteration_barrier) != 0);
    munmap(r.base as *mut c_void, r.size);

    0
}

pub unsafe fn main() -> c_int {
    test_harness(test, c"pkey_siginfo".as_ptr())
}
