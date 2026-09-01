// SPDX-License-Identifier: GPL-2.0

// C dependencies: pthread.h, sys/shm.h, sys/mman.h, fcntl.h, stdbool.h,
// stdio.h, stdlib.h, time.h, assert.h, "futextest.h", "futex2test.h".

use std::ffi::{c_char, c_int, c_long, c_uint, c_void};
use std::ptr;
use std::sync::atomic::{AtomicU32, AtomicU64, Ordering};

type u32 = std::ffi::c_uint;
type s32 = i32;
type u64 = std::ffi::c_ulonglong;
type pthread_t = libc_pthread_t;

type libc_pthread_t = usize;

const FUTEX2_SIZE_U32: c_uint = 0x02;
const FUTEX2_PRIVATE: c_uint = 0x80;
const FUTEX2_NUMA: c_uint = 0x04;
const FUTEX_NO_NODE: c_int = -1;

static mut fflags: c_uint = FUTEX2_SIZE_U32 | FUTEX2_PRIVATE;
static mut fnode: c_int = FUTEX_NO_NODE;

/* fairly stupid test-and-set lock with a waiter flag */

const N_LOCK: u32 = 0x0000001;
const N_WAITERS: u32 = 0x0001000;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct futex_numa_32_parts {
    pub val: u32,
    pub node: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub union futex_numa_32_union {
    pub full: u64,
    pub parts: futex_numa_32_parts,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct futex_numa_32 {
    pub u: futex_numa_32_union,
}

#[repr(C)]
pub struct timespec {
    pub tv_sec: c_long,
    pub tv_nsec: c_long,
}

unsafe extern "C" {
    static mut optarg: *mut c_char;

    fn getopt(argc: c_int, argv: *mut *mut c_char, optstring: *const c_char) -> c_int;
    fn atoi(nptr: *const c_char) -> c_int;
    fn calloc(nmemb: usize, size: usize) -> *mut c_void;
    fn perror(s: *const c_char);
    fn exit(status: c_int) -> !;
    fn printf(format: *const c_char, ...) -> c_int;
    fn sleep(seconds: c_uint) -> c_uint;
    fn nanosleep(req: *const timespec, rem: *mut timespec) -> c_int;
    fn pthread_create(
        thread: *mut pthread_t,
        attr: *const c_void,
        start_routine: unsafe extern "C" fn(*mut c_void) -> *mut c_void,
        arg: *mut c_void,
    ) -> c_int;
    fn pthread_join(thread: pthread_t, retval: *mut *mut c_void) -> c_int;

    fn futex2_wait(
        uaddr: *mut c_void,
        val: u32,
        flags: c_uint,
        timeout: *mut c_void,
        clockid: c_int,
    ) -> c_int;
    fn futex2_wake(uaddr: *mut c_void, nr_wake: c_int, flags: c_uint) -> c_int;
}

#[inline]
unsafe fn full_ptr(lock: *mut futex_numa_32) -> *mut AtomicU64 {
    unsafe { &mut (*lock).u.full as *mut u64 as *mut AtomicU64 }
}

#[inline]
unsafe fn val_ptr(lock: *mut futex_numa_32) -> *mut AtomicU32 {
    unsafe { &mut (*lock).u.parts.val as *mut u32 as *mut AtomicU32 }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn futex_numa_32_lock(lock: *mut futex_numa_32) {
    loop {
        let mut new: futex_numa_32;
        let mut old = futex_numa_32 {
            u: futex_numa_32_union {
                full: unsafe { (*full_ptr(lock)).load(Ordering::Relaxed) },
            },
        };

        loop {
            new = old;
            if unsafe { old.u.parts.val } == 0 {
                /* no waiter, no lock -> first lock, set no-node */
                unsafe {
                    new.u.parts.node = fnode as u32;
                }
            }
            if unsafe { old.u.parts.val } & N_LOCK != 0 {
                /* contention, set waiter */
                unsafe {
                    new.u.parts.val |= N_WAITERS;
                }
            }
            unsafe {
                new.u.parts.val |= N_LOCK;
            }

            /* nothing changed, ready to block */
            if unsafe { old.u.full == new.u.full } {
                break;
            }

            /*
             * Use u64 cmpxchg to set the futex value and node in a
             * consistent manner.
             */
            let mut expected = unsafe { old.u.full };
            match unsafe {
                (*full_ptr(lock)).compare_exchange(
                    expected,
                    new.u.full,
                    Ordering::Acquire,
                    Ordering::Relaxed,
                )
            } {
                Ok(_) => {
                    /* if we just set N_LOCK, we own it */
                    if unsafe { old.u.parts.val } & N_LOCK == 0 {
                        return;
                    }

                    /* go block */
                    break;
                }
                Err(actual) => {
                    expected = actual;
                    old.u.full = expected;
                }
            }
        }

        unsafe {
            futex2_wait(
                lock as *mut c_void,
                new.u.parts.val,
                fflags,
                ptr::null_mut(),
                0,
            );
        }
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn futex_numa_32_unlock(lock: *mut futex_numa_32) {
    let mut val: u32 = unsafe { (*val_ptr(lock)).fetch_sub(N_LOCK, Ordering::Release) - N_LOCK };
    assert!((val as s32) >= 0);
    if val & N_WAITERS != 0 {
        let woken = unsafe { futex2_wake(lock as *mut c_void, 1, fflags) };
        assert!(val == N_WAITERS);
        if woken == 0 {
            let _ = unsafe {
                (*val_ptr(lock)).compare_exchange(val, 0_u32, Ordering::Relaxed, Ordering::Relaxed)
            };
        }
    }
}

static mut nanos: c_long = 50000;

#[repr(C)]
pub struct thread_args {
    pub tid: pthread_t,
    pub done: *mut c_int,
    pub lock: *mut futex_numa_32,
    pub val: c_int,
    pub val1: *mut c_int,
    pub val2: *mut c_int,
    pub node: c_int,
}

unsafe extern "C" fn threadfn(_arg: *mut c_void) -> *mut c_void {
    let args = _arg as *mut thread_args;
    let ts = timespec {
        tv_sec: 0,
        tv_nsec: unsafe { nanos },
    };
    let mut node: c_int;

    while unsafe { *(*args).done } == 0 {
        unsafe {
            futex_numa_32_lock((*args).lock);
            (*args).val += 1;

            assert!(*(*args).val1 == *(*args).val2);
            *(*args).val1 += 1;
            nanosleep(&ts, ptr::null_mut());
            *(*args).val2 += 1;

            node = (*(*args).lock).u.parts.node as c_int;
            futex_numa_32_unlock((*args).lock);

            if node != (*args).node {
                (*args).node = node;
                printf(c"node: %d\n".as_ptr(), node);
            }

            nanosleep(&ts, ptr::null_mut());
        }
    }

    ptr::null_mut()
}

unsafe extern "C" fn contendfn(_arg: *mut c_void) -> *mut c_void {
    let args = _arg as *mut thread_args;

    while unsafe { *(*args).done } == 0 {
        /*
         * futex2_wait() will take hb-lock, verify *var == val and
         * queue/abort.  By knowingly setting val 'wrong' this will
         * abort and thereby generate hb-lock contention.
         */
        unsafe {
            futex2_wait(
                &mut (*(*args).lock).u.parts.val as *mut u32 as *mut c_void,
                !0_u32,
                fflags,
                ptr::null_mut(),
                0,
            );
            (*args).val += 1;
        }
    }

    ptr::null_mut()
}

static mut done: c_int = 0;
static mut lock: futex_numa_32 = futex_numa_32 {
    u: futex_numa_32_union {
        parts: futex_numa_32_parts { val: 0, node: 0 },
    },
};
static mut val1: c_int = 0;
static mut val2: c_int = 0;

#[unsafe(no_mangle)]
pub unsafe extern "C" fn main(argc: c_int, argv: *mut *mut c_char) -> c_int {
    let mut tas: [*mut thread_args; 512] = [ptr::null_mut(); 512];
    let mut cas: [*mut thread_args; 512] = [ptr::null_mut(); 512];
    let mut c: c_int;
    let mut t: c_int;
    let mut threads: c_int = 2;
    let mut contenders: c_int = 0;
    let mut sleeps: c_int = 10;
    let mut total: c_int = 0;

    loop {
        c = unsafe { getopt(argc, argv, c"c:t:s:n:N::".as_ptr()) };
        if c == -1 {
            break;
        }
        match c as u8 as char {
            'c' => {
                contenders = unsafe { atoi(optarg) };
            }
            't' => {
                threads = unsafe { atoi(optarg) };
            }
            's' => {
                sleeps = unsafe { atoi(optarg) };
            }
            'n' => {
                unsafe {
                    nanos = atoi(optarg) as c_long;
                }
            }
            'N' => unsafe {
                fflags |= FUTEX2_NUMA;
                if !optarg.is_null() {
                    fnode = atoi(optarg);
                }
            },
            _ => unsafe {
                exit(1);
            },
        }
    }

    t = 0;
    while t < contenders {
        let args = unsafe { calloc(1, std::mem::size_of::<thread_args>()) as *mut thread_args };
        if args.is_null() {
            unsafe {
                perror(c"thread_args".as_ptr());
                exit(-1);
            }
        }

        unsafe {
            (*args).done = &raw mut done;
            (*args).lock = &raw mut lock;
            (*args).val1 = &raw mut val1;
            (*args).val2 = &raw mut val2;
            (*args).node = -1;

            if pthread_create(&mut (*args).tid, ptr::null(), contendfn, args as *mut c_void) != 0 {
                perror(c"pthread_create".as_ptr());
                exit(-1);
            }
        }

        cas[t as usize] = args;
        t += 1;
    }

    t = 0;
    while t < threads {
        let args = unsafe { calloc(1, std::mem::size_of::<thread_args>()) as *mut thread_args };
        if args.is_null() {
            unsafe {
                perror(c"thread_args".as_ptr());
                exit(-1);
            }
        }

        unsafe {
            (*args).done = &raw mut done;
            (*args).lock = &raw mut lock;
            (*args).val1 = &raw mut val1;
            (*args).val2 = &raw mut val2;
            (*args).node = -1;

            if pthread_create(&mut (*args).tid, ptr::null(), threadfn, args as *mut c_void) != 0 {
                perror(c"pthread_create".as_ptr());
                exit(-1);
            }
        }

        tas[t as usize] = args;
        t += 1;
    }

    unsafe {
        sleep(sleeps as c_uint);

        done = true as c_int;
    }

    t = 0;
    while t < threads {
        let args = tas[t as usize];

        unsafe {
            pthread_join((*args).tid, ptr::null_mut());
            total += (*args).val;
        }
        //		printf("tval: %d\n", args->val);
        t += 1;
    }
    unsafe {
        printf(c"total: %d\n".as_ptr(), total);
    }

    if contenders != 0 {
        total = 0;
        t = 0;
        while t < contenders {
            let args = cas[t as usize];

            unsafe {
                pthread_join((*args).tid, ptr::null_mut());
                total += (*args).val;
            }
            //		printf("tval: %d\n", args->val);
            t += 1;
        }
        unsafe {
            printf(c"contenders: %d\n".as_ptr(), total);
        }
    }

    0
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
