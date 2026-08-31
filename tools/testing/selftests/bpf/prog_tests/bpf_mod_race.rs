// SPDX-License-Identifier: GPL-2.0
// C dependencies: unistd.h, pthread.h, sys/mman.h, stdatomic.h,
// test_progs.h, sys/syscall.h, linux/module.h, linux/userfaultfd.h,
// ksym_race.skel.h, bpf_mod_race.skel.h, kfunc_call_race.skel.h,
// testing_helpers.h

use core::ffi::{c_char, c_int, c_long, c_ulong, c_void};
use core::mem::size_of;
use core::ptr;
use core::sync::atomic::{AtomicI32, Ordering};

type PthreadT = c_ulong;

const O_CLOEXEC: c_int = 0o2000000;
const PROT_READ: c_int = 0x1;
const MAP_PRIVATE: c_int = 0x02;
const MAP_ANONYMOUS: c_int = 0x20;
const SIGKILL: c_int = 9;
const ENXIO: c_int = 6;
const __NR_USERFAULTFD: c_long = 323;
const UFFD_API: u64 = 0xAA;
const UFFDIO_REGISTER_MODE_MISSING: u64 = 1 << 0;
const UFFD_EVENT_PAGEFAULT: u8 = 12;

// ioctl request values are provided by linux/userfaultfd.h in C.
const UFFDIO_API: c_ulong = 0xC018_AA3F;
const UFFDIO_REGISTER: c_ulong = 0xC020_AA00;

#[repr(C)]
struct test_config {
    str_open: *const c_char,
    bpf_open_and_load: unsafe extern "C" fn() -> *mut c_void,
    bpf_destroy: unsafe extern "C" fn(*mut c_void),
}

#[repr(i32)]
#[derive(Copy, Clone, PartialEq, Eq)]
enum bpf_test_state {
    _TS_INVALID = 0,
    TS_MODULE_LOAD = 1,
    TS_MODULE_LOAD_FAIL = 2,
}

static STATE: AtomicI32 = AtomicI32::new(bpf_test_state::_TS_INVALID as i32);

#[repr(C)]
struct uffdio_range {
    start: u64,
    len: u64,
}

#[repr(C)]
struct uffdio_register {
    range: uffdio_range,
    mode: u64,
    ioctls: u64,
}

#[repr(C)]
struct uffdio_api {
    api: u64,
    features: u64,
    ioctls: u64,
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
    reserved: [u8; 32],
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
struct bpf_mod_race_config {
    tgid: c_int,
    inject_error: c_int,
    fault_addr: *mut c_void,
}

#[repr(C)]
struct bpf_mod_race_rodata {
    bpf_mod_race_config: bpf_mod_race_config,
}

#[repr(C)]
struct bpf_mod_race_bss {
    bpf_blocking: c_int,
}

#[repr(C)]
struct bpf_mod_race_data {
    res_try_get_module: bool,
}

#[repr(C)]
struct bpf_mod_race {
    rodata: *mut bpf_mod_race_rodata,
    bss: *mut bpf_mod_race_bss,
    data: *mut bpf_mod_race_data,
}

extern "C" {
    static mut errno: c_int;
    static mut MAP_FAILED: *mut c_void;

    fn syscall(num: c_long, ...) -> c_long;
    fn ioctl(fd: c_int, request: c_ulong, ...) -> c_int;
    fn close(fd: c_int) -> c_int;
    fn read(fd: c_int, buf: *mut c_void, count: usize) -> isize;
    fn getpagesize() -> c_int;
    fn getpid() -> c_int;
    fn mmap(
        addr: *mut c_void,
        length: usize,
        prot: c_int,
        flags: c_int,
        fd: c_int,
        offset: c_long,
    ) -> *mut c_void;
    fn munmap(addr: *mut c_void, length: usize) -> c_int;
    fn pthread_create(
        thread: *mut PthreadT,
        attr: *const c_void,
        start_routine: unsafe extern "C" fn(*mut c_void) -> *mut c_void,
        arg: *mut c_void,
    ) -> c_int;
    fn pthread_join(thread: PthreadT, retval: *mut *mut c_void) -> c_int;
    fn pthread_kill(thread: PthreadT, sig: c_int) -> c_int;

    fn ASSERT_NEQ(a: c_long, b: c_long, name: *const c_char) -> bool;
    fn ASSERT_OK(res: c_int, name: *const c_char) -> bool;
    fn ASSERT_OK_PTR(ptr: *mut c_void, name: *const c_char) -> bool;
    fn ASSERT_GE(a: c_int, b: c_int, name: *const c_char) -> bool;
    fn ASSERT_EQ(a: c_long, b: c_long, name: *const c_char) -> bool;

    fn load_bpf_testmod(verbose: bool) -> c_int;
    fn unload_bpf_testmod(verbose: bool) -> c_int;
    fn kern_sync_rcu() -> c_int;
    fn test__start_subtest(name: *const c_char) -> bool;

    fn bpf_mod_race__open() -> *mut bpf_mod_race;
    fn bpf_mod_race__load(skel: *mut bpf_mod_race) -> c_int;
    fn bpf_mod_race__attach(skel: *mut bpf_mod_race) -> c_int;
    fn bpf_mod_race__destroy(skel: *mut bpf_mod_race);

    fn ksym_race__open_and_load() -> *mut c_void;
    fn ksym_race__destroy(skel: *mut c_void);
    fn kfunc_call_race__open_and_load() -> *mut c_void;
    fn kfunc_call_race__destroy(skel: *mut c_void);
}

unsafe extern "C" fn load_module_thread(p: *mut c_void) -> *mut c_void {
    if !ASSERT_NEQ(
        load_bpf_testmod(false) as c_long,
        0,
        c"load_module_thread must fail".as_ptr(),
    ) {
        STATE.store(bpf_test_state::TS_MODULE_LOAD as i32, Ordering::SeqCst);
    } else {
        STATE.store(
            bpf_test_state::TS_MODULE_LOAD_FAIL as i32,
            Ordering::SeqCst,
        );
    }
    p
}

unsafe fn sys_userfaultfd(flags: c_int) -> c_int {
    syscall(__NR_USERFAULTFD, flags) as c_int
}

unsafe fn test_setup_uffd(fault_addr: *mut c_void) -> c_int {
    let mut uffd_register: uffdio_register = core::mem::zeroed();
    let mut uffd_api: uffdio_api = core::mem::zeroed();
    let uffd: c_int;

    uffd = sys_userfaultfd(O_CLOEXEC);
    if uffd < 0 {
        return -errno;
    }

    uffd_api.api = UFFD_API;
    uffd_api.features = 0;
    if ioctl(uffd, UFFDIO_API, &mut uffd_api) != 0 {
        close(uffd);
        return -1;
    }

    uffd_register.range.start = fault_addr as c_ulong as u64;
    uffd_register.range.len = getpagesize() as u64;
    uffd_register.mode = UFFDIO_REGISTER_MODE_MISSING;
    if ioctl(uffd, UFFDIO_REGISTER, &mut uffd_register) != 0 {
        close(uffd);
        return -1;
    }
    uffd
}

unsafe fn test_bpf_mod_race_config(config: *const test_config) {
    let fault_addr: *mut c_void;
    let mut skel_fail: *mut c_void;
    let skel: *mut bpf_mod_race;
    let mut uffd_msg: uffd_msg = core::mem::zeroed();
    let mut load_mod_thrd: PthreadT = 0;
    let blockingp: *mut AtomicI32;
    let mut uffd: c_int;
    let mut ret: c_int;

    fault_addr = mmap(
        ptr::null_mut(),
        4096,
        PROT_READ,
        MAP_PRIVATE | MAP_ANONYMOUS,
        -1,
        0,
    );
    if !ASSERT_NEQ(
        fault_addr as c_long,
        MAP_FAILED as c_long,
        c"mmap for uffd registration".as_ptr(),
    ) {
        return;
    }

    if !ASSERT_OK(unload_bpf_testmod(false), c"unload bpf_testmod".as_ptr()) {
        munmap(fault_addr, 4096);
        STATE.store(bpf_test_state::_TS_INVALID as i32, Ordering::SeqCst);
        return;
    }

    skel = bpf_mod_race__open();
    if !ASSERT_OK_PTR(skel as *mut c_void, c"bpf_mod_kfunc_race__open".as_ptr()) {
        unload_bpf_testmod(false);
        ASSERT_OK(load_bpf_testmod(false), c"restore bpf_testmod".as_ptr());
        munmap(fault_addr, 4096);
        STATE.store(bpf_test_state::_TS_INVALID as i32, Ordering::SeqCst);
        return;
    }

    (*(*skel).rodata).bpf_mod_race_config.tgid = getpid();
    (*(*skel).rodata).bpf_mod_race_config.inject_error = -4242;
    (*(*skel).rodata).bpf_mod_race_config.fault_addr = fault_addr;
    if !ASSERT_OK(bpf_mod_race__load(skel), c"bpf_mod___load".as_ptr()) {
        bpf_mod_race__destroy(skel);
        ASSERT_OK(kern_sync_rcu(), c"kern_sync_rcu".as_ptr());
        unload_bpf_testmod(false);
        ASSERT_OK(load_bpf_testmod(false), c"restore bpf_testmod".as_ptr());
        munmap(fault_addr, 4096);
        STATE.store(bpf_test_state::_TS_INVALID as i32, Ordering::SeqCst);
        return;
    }
    blockingp = &mut (*(*skel).bss).bpf_blocking as *mut c_int as *mut AtomicI32;

    if !ASSERT_OK(
        bpf_mod_race__attach(skel),
        c"bpf_mod_kfunc_race__attach".as_ptr(),
    ) {
        bpf_mod_race__destroy(skel);
        ASSERT_OK(kern_sync_rcu(), c"kern_sync_rcu".as_ptr());
        unload_bpf_testmod(false);
        ASSERT_OK(load_bpf_testmod(false), c"restore bpf_testmod".as_ptr());
        munmap(fault_addr, 4096);
        STATE.store(bpf_test_state::_TS_INVALID as i32, Ordering::SeqCst);
        return;
    }

    uffd = test_setup_uffd(fault_addr);
    if !ASSERT_GE(uffd, 0, c"userfaultfd open + register address".as_ptr()) {
        bpf_mod_race__destroy(skel);
        ASSERT_OK(kern_sync_rcu(), c"kern_sync_rcu".as_ptr());
        unload_bpf_testmod(false);
        ASSERT_OK(load_bpf_testmod(false), c"restore bpf_testmod".as_ptr());
        munmap(fault_addr, 4096);
        STATE.store(bpf_test_state::_TS_INVALID as i32, Ordering::SeqCst);
        return;
    }

    if !ASSERT_OK(
        pthread_create(
            &mut load_mod_thrd,
            ptr::null(),
            load_module_thread,
            ptr::null_mut(),
        ),
        c"load module thread".as_ptr(),
    ) {
        close(uffd);
        bpf_mod_race__destroy(skel);
        ASSERT_OK(kern_sync_rcu(), c"kern_sync_rcu".as_ptr());
        unload_bpf_testmod(false);
        ASSERT_OK(load_bpf_testmod(false), c"restore bpf_testmod".as_ptr());
        munmap(fault_addr, 4096);
        STATE.store(bpf_test_state::_TS_INVALID as i32, Ordering::SeqCst);
        return;
    }

    /* Now, we either fail loading module, or block in bpf prog, spin to find out */
    while STATE.load(Ordering::SeqCst) == 0 && (*blockingp).load(Ordering::SeqCst) == 0 {}
    if !ASSERT_EQ(
        STATE.load(Ordering::SeqCst) as c_long,
        bpf_test_state::_TS_INVALID as c_long,
        c"module load should block".as_ptr(),
    ) {
        pthread_join(load_mod_thrd, ptr::null_mut());
        close(uffd);
        bpf_mod_race__destroy(skel);
        ASSERT_OK(kern_sync_rcu(), c"kern_sync_rcu".as_ptr());
        unload_bpf_testmod(false);
        ASSERT_OK(load_bpf_testmod(false), c"restore bpf_testmod".as_ptr());
        munmap(fault_addr, 4096);
        STATE.store(bpf_test_state::_TS_INVALID as i32, Ordering::SeqCst);
        return;
    }
    if !ASSERT_EQ(
        (*blockingp).load(Ordering::SeqCst) as c_long,
        1,
        c"module load blocked".as_ptr(),
    ) {
        pthread_kill(load_mod_thrd, SIGKILL);
        close(uffd);
        bpf_mod_race__destroy(skel);
        ASSERT_OK(kern_sync_rcu(), c"kern_sync_rcu".as_ptr());
        unload_bpf_testmod(false);
        ASSERT_OK(load_bpf_testmod(false), c"restore bpf_testmod".as_ptr());
        munmap(fault_addr, 4096);
        STATE.store(bpf_test_state::_TS_INVALID as i32, Ordering::SeqCst);
        return;
    }

    /* We might have set bpf_blocking to 1, but may have not blocked in
     * bpf_copy_from_user. Read userfaultfd descriptor to verify that.
     */
    if !ASSERT_EQ(
        read(
            uffd,
            &mut uffd_msg as *mut uffd_msg as *mut c_void,
            size_of::<uffd_msg>(),
        ) as c_long,
        size_of::<uffd_msg>() as c_long,
        c"read uffd block event".as_ptr(),
    ) {
        pthread_join(load_mod_thrd, ptr::null_mut());
        close(uffd);
        bpf_mod_race__destroy(skel);
        ASSERT_OK(kern_sync_rcu(), c"kern_sync_rcu".as_ptr());
        unload_bpf_testmod(false);
        ASSERT_OK(load_bpf_testmod(false), c"restore bpf_testmod".as_ptr());
        munmap(fault_addr, 4096);
        STATE.store(bpf_test_state::_TS_INVALID as i32, Ordering::SeqCst);
        return;
    }
    if !ASSERT_EQ(
        uffd_msg.event as c_long,
        UFFD_EVENT_PAGEFAULT as c_long,
        c"read uffd event is pagefault".as_ptr(),
    ) {
        pthread_join(load_mod_thrd, ptr::null_mut());
        close(uffd);
        bpf_mod_race__destroy(skel);
        ASSERT_OK(kern_sync_rcu(), c"kern_sync_rcu".as_ptr());
        unload_bpf_testmod(false);
        ASSERT_OK(load_bpf_testmod(false), c"restore bpf_testmod".as_ptr());
        munmap(fault_addr, 4096);
        STATE.store(bpf_test_state::_TS_INVALID as i32, Ordering::SeqCst);
        return;
    }

    /* We know that load_mod_thrd is blocked in the fmod_ret program, the
     * module state is still MODULE_STATE_COMING because mod->init hasn't
     * returned. This is the time we try to load a program calling kfunc and
     * check if we get ENXIO from verifier.
     */
    skel_fail = ((*config).bpf_open_and_load)();
    ret = errno;
    if !ASSERT_EQ(
        skel_fail as c_long,
        ptr::null_mut::<c_void>() as c_long,
        (*config).str_open,
    ) {
        /* Close uffd to unblock load_mod_thrd */
        close(uffd);
        uffd = -1;
        while (*blockingp).load(Ordering::SeqCst) != 2 {}
        ASSERT_OK(kern_sync_rcu(), c"kern_sync_rcu".as_ptr());
        ((*config).bpf_destroy)(skel_fail);
        pthread_join(load_mod_thrd, ptr::null_mut());
        bpf_mod_race__destroy(skel);
        ASSERT_OK(kern_sync_rcu(), c"kern_sync_rcu".as_ptr());
        unload_bpf_testmod(false);
        ASSERT_OK(load_bpf_testmod(false), c"restore bpf_testmod".as_ptr());
        munmap(fault_addr, 4096);
        STATE.store(bpf_test_state::_TS_INVALID as i32, Ordering::SeqCst);
        return;
    }
    ASSERT_EQ(ret as c_long, ENXIO as c_long, c"verifier returns ENXIO".as_ptr());
    ASSERT_EQ(
        (*(*skel).data).res_try_get_module as c_long,
        false as c_long,
        c"btf_try_get_module == false".as_ptr(),
    );

    close(uffd);
    uffd = -1;
    pthread_join(load_mod_thrd, ptr::null_mut());
    if uffd < 0 {
        ASSERT_EQ(
            STATE.load(Ordering::SeqCst) as c_long,
            bpf_test_state::TS_MODULE_LOAD_FAIL as c_long,
            c"load_mod_thrd success".as_ptr(),
        );
    }
    bpf_mod_race__destroy(skel);
    ASSERT_OK(kern_sync_rcu(), c"kern_sync_rcu".as_ptr());
    unload_bpf_testmod(false);
    ASSERT_OK(load_bpf_testmod(false), c"restore bpf_testmod".as_ptr());
    munmap(fault_addr, 4096);
    STATE.store(bpf_test_state::_TS_INVALID as i32, Ordering::SeqCst);
}

static KSYM_STR_OPEN: &[u8] = b"ksym_race__open_and_load\0";
static KFUNC_STR_OPEN: &[u8] = b"kfunc_call_race__open_and_load\0";

static KSYM_CONFIG: test_config = test_config {
    str_open: KSYM_STR_OPEN.as_ptr() as *const c_char,
    bpf_open_and_load: ksym_race__open_and_load,
    bpf_destroy: ksym_race__destroy,
};

static KFUNC_CONFIG: test_config = test_config {
    str_open: KFUNC_STR_OPEN.as_ptr() as *const c_char,
    bpf_open_and_load: kfunc_call_race__open_and_load,
    bpf_destroy: kfunc_call_race__destroy,
};

#[no_mangle]
pub unsafe extern "C" fn serial_test_bpf_mod_race() {
    if test__start_subtest(c"ksym (used_btfs UAF)".as_ptr()) {
        test_bpf_mod_race_config(&KSYM_CONFIG);
    }
    if test__start_subtest(c"kfunc (kfunc_btf_tab UAF)".as_ptr()) {
        test_bpf_mod_race_config(&KFUNC_CONFIG);
    }
}
