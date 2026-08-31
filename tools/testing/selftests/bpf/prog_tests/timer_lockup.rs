// SPDX-License-Identifier: GPL-2.0

// C dependencies: _GNU_SOURCE, <sched.h>, <test_progs.h>, <pthread.h>,
// <network_helpers.h>, <sys/sysinfo.h>, and "timer_lockup.skel.h".

use core::ffi::{c_int, c_long, c_void};
use core::mem::size_of;
use core::ptr;

#[allow(non_camel_case_types)]
type pthread_t = c_ulong;

#[allow(non_camel_case_types)]
type c_ulong = usize;

#[repr(C)]
pub struct cpu_set_t {
    __bits: [c_ulong; 16],
}

#[repr(C)]
pub struct bpf_test_run_opts {
    pub sz: usize,
    pub data_in: *const c_void,
    pub data_out: *mut c_void,
    pub data_size_in: u32,
    pub data_size_out: u32,
    pub retval: u32,
    pub repeat: u32,
    pub duration: u32,
    pub flags: u32,
    pub cpu: u32,
    pub batch_size: u32,
}

#[repr(C)]
pub struct timer_lockup {
    pub progs: timer_lockup_progs,
    pub bss: *mut timer_lockup_bss,
}

#[repr(C)]
pub struct timer_lockup_progs {
    pub timer1_prog: *mut bpf_program,
    pub timer2_prog: *mut bpf_program,
}

#[repr(C)]
pub struct timer_lockup_bss {
    pub timer1_err: c_int,
    pub timer2_err: c_int,
}

#[repr(C)]
pub struct bpf_program {
    _private: [u8; 0],
}

unsafe extern "C" {
    static pkt_v4: [u8; 0];
    static mut errno: c_int;

    fn pthread_self() -> pthread_t;
    fn pthread_setaffinity_np(thread: pthread_t, cpusetsize: usize, cpuset: *const cpu_set_t) -> c_int;
    fn pthread_create(
        thread: *mut pthread_t,
        attr: *const c_void,
        start_routine: unsafe extern "C" fn(*mut c_void) -> *mut c_void,
        arg: *mut c_void,
    ) -> c_int;
    fn pthread_exit(value_ptr: *mut c_void) -> !;
    fn pthread_join(thread: pthread_t, retval: *mut *mut c_void) -> c_int;

    fn get_nprocs() -> c_int;
    fn bpf_prog_test_run_opts(prog_fd: c_int, opts: *mut bpf_test_run_opts) -> c_int;
    fn bpf_program__fd(prog: *mut bpf_program) -> c_int;
    fn timer_lockup__open_and_load() -> *mut timer_lockup;
    fn timer_lockup__destroy(obj: *mut timer_lockup);

    fn test__skip();
    fn ASSERT_OK(res: c_int, name: *const u8) -> bool;
    fn ASSERT_OK_PTR(ptr: *const c_void, name: *const u8) -> bool;
    fn ASSERT_FAIL(name: *const u8) -> bool;
}

const EOPNOTSUPP: c_int = 95;
const EDEADLK: c_int = 35;

static mut CPU: c_long = 0;
static mut TIMER1_ERR: *mut c_int = ptr::null_mut();
static mut TIMER2_ERR: *mut c_int = ptr::null_mut();
static mut SKIP: bool = false;

#[unsafe(no_mangle)]
pub static mut k: c_int = 0;

unsafe fn cpu_zero(cpuset: *mut cpu_set_t) {
    unsafe {
        (*cpuset).__bits = [0; 16];
    }
}

unsafe fn cpu_set(cpu: c_long, cpuset: *mut cpu_set_t) {
    let cpu = cpu as usize;
    let bits_per_word = 8 * size_of::<c_ulong>();

    unsafe {
        (*cpuset).__bits[cpu / bits_per_word] |= (1 as c_ulong) << (cpu % bits_per_word);
    }
}

unsafe fn sync_fetch_and_add(ptr: *mut c_long, val: c_long) -> c_long {
    unsafe { core::intrinsics::atomic_xadd_seqcst(ptr, val) }
}

unsafe fn read_once_int(ptr: *const c_int) -> c_int {
    unsafe { ptr::read_volatile(ptr) }
}

unsafe fn write_once_bool(ptr: *mut bool, val: bool) {
    unsafe {
        ptr::write_volatile(ptr, val);
    }
}

unsafe extern "C" fn timer_lockup_thread(arg: *mut c_void) -> *mut c_void {
    let mut opts = bpf_test_run_opts {
        sz: size_of::<bpf_test_run_opts>(),
        data_in: unsafe { (&raw const pkt_v4).cast::<c_void>() },
        data_out: ptr::null_mut(),
        data_size_in: size_of_val_pkt_v4() as u32,
        data_size_out: 0,
        retval: 0,
        repeat: 1000,
        duration: 0,
        flags: 0,
        cpu: 0,
        batch_size: 0,
    };
    let mut i: c_int;
    let prog_fd: c_int = unsafe { *(arg as *mut c_int) };
    let mut cpuset = cpu_set_t { __bits: [0; 16] };

    unsafe {
        cpu_zero(&mut cpuset);
        let cpu_id = sync_fetch_and_add(&raw mut CPU, 1);
        cpu_set(cpu_id, &mut cpuset);
        ASSERT_OK(
            pthread_setaffinity_np(pthread_self(), size_of::<cpu_set_t>(), &cpuset),
            c"cpu affinity".as_ptr().cast::<u8>(),
        );
    }

    i = 0;
    unsafe {
        while read_once_int(TIMER1_ERR) == 0 && read_once_int(TIMER2_ERR) == 0 {
            bpf_prog_test_run_opts(prog_fd, &mut opts);
            /* Skip the test if we can't reproduce the race in a reasonable
             * amount of time.
             */
            if i > 50 {
                write_once_bool(&raw mut SKIP, true);
                break;
            }
            i += 1;
        }
    }

    ptr::null_mut()
}

fn size_of_val_pkt_v4() -> usize {
    unsafe { size_of_val_raw(&raw const pkt_v4) }
}

unsafe fn size_of_val_raw<T>(_: *const T) -> usize {
    size_of::<T>()
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn test_timer_lockup() {
    let mut timer1_prog: c_int;
    let mut timer2_prog: c_int;
    let skel: *mut timer_lockup;
    let mut thrds: [pthread_t; 2] = [0; 2];
    let mut ret: *mut c_void = ptr::null_mut();

    unsafe {
        if get_nprocs() < 2 {
            test__skip();
            return;
        }

        skel = timer_lockup__open_and_load();
        if skel.is_null() && errno == EOPNOTSUPP {
            test__skip();
            return;
        }
        if !ASSERT_OK_PTR(skel.cast::<c_void>(), c"timer_lockup__open_and_load".as_ptr().cast::<u8>()) {
            return;
        }

        timer1_prog = bpf_program__fd((*skel).progs.timer1_prog);
        timer2_prog = bpf_program__fd((*skel).progs.timer2_prog);

        TIMER1_ERR = &mut (*(*skel).bss).timer1_err;
        TIMER2_ERR = &mut (*(*skel).bss).timer2_err;

        if !ASSERT_OK(
            pthread_create(
                &mut thrds[0],
                ptr::null(),
                timer_lockup_thread,
                (&mut timer1_prog as *mut c_int).cast::<c_void>(),
            ),
            c"pthread_create thread1".as_ptr().cast::<u8>(),
        ) {
            timer_lockup__destroy(skel);
            return;
        }
        if !ASSERT_OK(
            pthread_create(
                &mut thrds[1],
                ptr::null(),
                timer_lockup_thread,
                (&mut timer2_prog as *mut c_int).cast::<c_void>(),
            ),
            c"pthread_create thread2".as_ptr().cast::<u8>(),
        ) {
            pthread_exit((&mut thrds[0] as *mut pthread_t).cast::<c_void>());
        }

        pthread_join(thrds[1], &mut ret);
        pthread_join(thrds[0], &mut ret);

        if SKIP {
            test__skip();
            timer_lockup__destroy(skel);
            return;
        }

        if *TIMER1_ERR != -EDEADLK && *TIMER1_ERR != 0 {
            ASSERT_FAIL(c"timer1_err bad value".as_ptr().cast::<u8>());
        }
        if *TIMER2_ERR != -EDEADLK && *TIMER2_ERR != 0 {
            ASSERT_FAIL(c"timer2_err bad value".as_ptr().cast::<u8>());
        }

        timer_lockup__destroy(skel);
    }
}
