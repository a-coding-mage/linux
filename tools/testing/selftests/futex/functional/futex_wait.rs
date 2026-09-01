// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * Copyright Collabora Ltd., 2021
 *
 * futex cmp requeue test by Andre Almeida <andrealmeid@collabora.com>
 */

/* C dependencies:
 * <fcntl.h>, <pthread.h>, <stdlib.h>, <sys/shm.h>, <sys/mman.h>
 * "futextest.h", "kselftest_harness.h"
 */

const timeout_ns: libc::c_long = 30000000;
const WAKE_WAIT_US: libc::useconds_t = 10000;
const SHM_PATH: &[u8] = b"futex_shm_file\0";

static mut futex: *mut libc::c_void = core::ptr::null_mut();

#[repr(C)]
struct waiter_args {
    _metadata: *mut __test_metadata,
    flags: libc::c_uint,
}

extern "C" {
    static mut errno: libc::c_int;

    fn futex_wait(
        uaddr: *mut libc::c_void,
        val: libc::c_int,
        timeout: *const libc::timespec,
        flags: libc::c_uint,
    ) -> libc::c_int;
    fn futex_wake(
        uaddr: *mut libc::c_void,
        nr_wake: libc::c_int,
        flags: libc::c_uint,
    ) -> libc::c_int;

    fn strerror(errnum: libc::c_int) -> *mut libc::c_char;
    fn free(ptr: *mut libc::c_void);
    fn malloc(size: libc::size_t) -> *mut libc::c_void;
    fn pthread_create(
        thread: *mut libc::pthread_t,
        attr: *const libc::pthread_attr_t,
        start_routine: extern "C" fn(*mut libc::c_void) -> *mut libc::c_void,
        arg: *mut libc::c_void,
    ) -> libc::c_int;
    fn pthread_join(thread: libc::pthread_t, retval: *mut *mut libc::c_void) -> libc::c_int;
    fn usleep(usec: libc::useconds_t) -> libc::c_int;
    fn shmget(key: libc::key_t, size: libc::size_t, shmflg: libc::c_int) -> libc::c_int;
    fn shmat(
        shmid: libc::c_int,
        shmaddr: *const libc::c_void,
        shmflg: libc::c_int,
    ) -> *mut libc::c_void;
    fn shmdt(shmaddr: *const libc::c_void) -> libc::c_int;
    fn open(path: *const libc::c_char, oflag: libc::c_int, mode: libc::mode_t) -> libc::c_int;
    fn ftruncate(fd: libc::c_int, length: libc::off_t) -> libc::c_int;
    fn close(fd: libc::c_int) -> libc::c_int;
    fn mmap(
        addr: *mut libc::c_void,
        length: libc::size_t,
        prot: libc::c_int,
        flags: libc::c_int,
        fd: libc::c_int,
        offset: libc::off_t,
    ) -> *mut libc::c_void;
    fn munmap(addr: *mut libc::c_void, length: libc::size_t) -> libc::c_int;
    fn memcpy(
        dest: *mut libc::c_void,
        src: *const libc::c_void,
        n: libc::size_t,
    ) -> *mut libc::c_void;
    fn remove(path: *const libc::c_char) -> libc::c_int;
}

// External declarations supplied by kselftest_harness.h / futextest.h.
#[repr(C)]
struct __test_metadata {
    _private: [u8; 0],
}

extern "C" {
    static FUTEX_PRIVATE_FLAG: libc::c_uint;
}

extern "C" fn waiterfn(arg: *mut libc::c_void) -> *mut libc::c_void {
    unsafe {
        let args = arg as *mut waiter_args;
        let _metadata = (*args)._metadata;
        let mut to: libc::timespec = core::mem::zeroed();
        let res: libc::c_int;

        to.tv_sec = 0;
        to.tv_nsec = timeout_ns;

        res = futex_wait(futex, 0, &to, (*args).flags);
        if res != 0 {
            EXPECT_EQ!(res, 0);
            TH_LOG!(
                _metadata,
                "waiter failed errno %d: %s\0",
                errno,
                strerror(errno)
            );
        }

        free(args as *mut libc::c_void);
        core::ptr::null_mut()
    }
}

// TEST(private_futex)
unsafe fn private_futex(_metadata: *mut __test_metadata) {
    let args = malloc(core::mem::size_of::<waiter_args>()) as *mut waiter_args;
    let mut f_private: u32 = 0;
    let mut waiter: libc::pthread_t = core::mem::zeroed();
    let res: libc::c_int;

    (*args)._metadata = _metadata;
    (*args).flags = FUTEX_PRIVATE_FLAG;
    futex = &mut f_private as *mut u32 as *mut libc::c_void;

    /* Testing a private futex */
    TH_LOG!(_metadata, "Calling private futex_wait on futex: %p\0", futex);
    ASSERT_EQ!(
        pthread_create(
            &mut waiter,
            core::ptr::null(),
            waiterfn,
            args as *mut libc::c_void
        ),
        0
    );
    TH_LOG!(_metadata, "pthread_create failed\0");

    usleep(WAKE_WAIT_US);

    TH_LOG!(_metadata, "Calling private futex_wake on futex: %p\0", futex);
    res = futex_wake(futex, 1, FUTEX_PRIVATE_FLAG);
    EXPECT_EQ!(res, 1);
    TH_LOG!(
        _metadata,
        "futex_wake private returned: %d %s\0",
        res,
        if res < 0 {
            strerror(errno)
        } else {
            b"\0".as_ptr() as *const libc::c_char as *mut libc::c_char
        }
    );

    pthread_join(waiter, core::ptr::null_mut());
}

// TEST(anon_page)
unsafe fn anon_page(_metadata: *mut __test_metadata) {
    let args = malloc(core::mem::size_of::<waiter_args>()) as *mut waiter_args;
    let shared_data: *mut u32;
    let mut waiter: libc::pthread_t = core::mem::zeroed();
    let mut res: libc::c_int;
    let shm_id: libc::c_int;

    (*args)._metadata = _metadata;
    (*args).flags = 0;

    /* Testing an anon page shared memory */
    shm_id = shmget(libc::IPC_PRIVATE, 4096, libc::IPC_CREAT | 0o666);
    if shm_id < 0 {
        if errno == libc::ENOSYS {
            free(args as *mut libc::c_void);
            SKIP!(return, "shmget syscall not supported\0");
        }
        ASSERT_GE!(shm_id, 0);
        TH_LOG!(_metadata, "shmget failed: %s\0", strerror(errno));
    }

    shared_data = shmat(shm_id, core::ptr::null(), 0) as *mut u32;
    if shared_data == (-1isize) as *mut u32 {
        free(args as *mut libc::c_void);
        ASSERT_NE!(shared_data, (-1isize) as *mut u32);
        TH_LOG!(_metadata, "shmat failed: %s\0", strerror(errno));
    }

    *shared_data = 0;
    futex = shared_data as *mut libc::c_void;

    TH_LOG!(
        _metadata,
        "Calling shared (page anon) futex_wait on futex: %p\0",
        futex
    );
    ASSERT_EQ!(
        pthread_create(
            &mut waiter,
            core::ptr::null(),
            waiterfn,
            args as *mut libc::c_void
        ),
        0
    );
    TH_LOG!(_metadata, "pthread_create failed\0");

    usleep(WAKE_WAIT_US);

    TH_LOG!(
        _metadata,
        "Calling shared (page anon) futex_wake on futex: %p\0",
        futex
    );
    res = futex_wake(futex, 1, 0);
    EXPECT_EQ!(res, 1);
    TH_LOG!(
        _metadata,
        "futex_wake shared (page anon) returned: %d %s\0",
        res,
        if res < 0 {
            strerror(errno)
        } else {
            b"\0".as_ptr() as *const libc::c_char as *mut libc::c_char
        }
    );

    pthread_join(waiter, core::ptr::null_mut());
    shmdt(shared_data as *const libc::c_void);
}

// TEST(file_backed)
unsafe fn file_backed(_metadata: *mut __test_metadata) {
    let args = malloc(core::mem::size_of::<waiter_args>()) as *mut waiter_args;
    let f_private: u32 = 0;
    let mut waiter: libc::pthread_t = core::mem::zeroed();
    let mut res: libc::c_int;
    let fd: libc::c_int;
    let shm: *mut libc::c_void;

    (*args)._metadata = _metadata;
    (*args).flags = 0;

    /* Testing a file backed shared memory */
    fd = open(
        SHM_PATH.as_ptr() as *const libc::c_char,
        libc::O_RDWR | libc::O_CREAT,
        0o600,
    );
    if fd < 0 {
        free(args as *mut libc::c_void);
        ASSERT_GE!(fd, 0);
        TH_LOG!(_metadata, "open failed: %s\0", strerror(errno));
    }

    if ftruncate(fd, core::mem::size_of_val(&f_private) as libc::off_t) != 0 {
        free(args as *mut libc::c_void);
        close(fd);
        ASSERT_TRUE!(0);
        TH_LOG!(_metadata, "ftruncate failed: %s\0", strerror(errno));
    }

    shm = mmap(
        core::ptr::null_mut(),
        core::mem::size_of_val(&f_private),
        libc::PROT_READ | libc::PROT_WRITE,
        libc::MAP_SHARED,
        fd,
        0,
    );
    if shm == libc::MAP_FAILED {
        free(args as *mut libc::c_void);
        close(fd);
        ASSERT_NE!(shm, libc::MAP_FAILED);
        TH_LOG!(_metadata, "mmap failed: %s\0", strerror(errno));
    }

    memcpy(
        shm,
        &f_private as *const u32 as *const libc::c_void,
        core::mem::size_of_val(&f_private),
    );

    futex = shm;

    TH_LOG!(
        _metadata,
        "Calling shared (file backed) futex_wait on futex: %p\0",
        futex
    );
    ASSERT_EQ!(
        pthread_create(
            &mut waiter,
            core::ptr::null(),
            waiterfn,
            args as *mut libc::c_void
        ),
        0
    );
    TH_LOG!(_metadata, "pthread_create failed\0");

    usleep(WAKE_WAIT_US);

    TH_LOG!(
        _metadata,
        "Calling shared (file backed) futex_wake on futex: %p\0",
        futex
    );
    res = futex_wake(shm, 1, 0);
    EXPECT_EQ!(res, 1);
    TH_LOG!(
        _metadata,
        "futex_wake shared (file backed) returned: %d %s\0",
        res,
        if res < 0 {
            strerror(errno)
        } else {
            b"\0".as_ptr() as *const libc::c_char as *mut libc::c_char
        }
    );

    pthread_join(waiter, core::ptr::null_mut());
    munmap(shm, core::mem::size_of_val(&f_private));
    remove(SHM_PATH.as_ptr() as *const libc::c_char);
    close(fd);
}

// TEST_HARNESS_MAIN

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
