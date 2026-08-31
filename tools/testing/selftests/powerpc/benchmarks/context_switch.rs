// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * Context switch microbenchmark.
 *
 * Copyright (C) 2015 Anton Blanchard <anton@au.ibm.com>, IBM
 */

// C dependencies: errno.h, sched.h, string.h, stdio.h, unistd.h, stdlib.h,
// getopt.h, signal.h, assert.h, pthread.h, limits.h, sys/time.h, sys/syscall.h,
// sys/sysinfo.h, sys/types.h, sys/shm.h, linux/futex.h, and "utils.h".

use core::arch::asm;
use core::ffi::{c_char, c_int, c_long, c_ulong, c_void};
use core::ptr;

static mut timeout: u32 = 30;

static mut touch_vdso: c_int = 0;
static mut tv: libc::timeval = libc::timeval {
    tv_sec: 0,
    tv_usec: 0,
};

static mut touch_fp: c_int = 1;
static mut fp: f64 = 0.0;

static mut touch_vector: c_int = 1;
type VectorInt = [i32; 4];
static mut a: VectorInt = [0; 4];
static mut b: VectorInt = [0; 4];
static mut c: VectorInt = [0; 4];

#[cfg(target_arch = "powerpc")]
static mut touch_altivec: c_int = 1;
#[cfg(target_arch = "powerpc64")]
static mut touch_altivec: c_int = 1;

/*
 * Note: LTO (Link Time Optimisation) doesn't play well with this function
 * attribute. Be very careful enabling LTO for this test.
 */
#[cfg(any(target_arch = "powerpc", target_arch = "powerpc64"))]
unsafe fn altivec_touch_fn() {
    c = [
        a[0].wrapping_add(b[0]),
        a[1].wrapping_add(b[1]),
        a[2].wrapping_add(b[2]),
        a[3].wrapping_add(b[3]),
    ];
}

unsafe fn vector_add_touch() {
    c = [
        a[0].wrapping_add(b[0]),
        a[1].wrapping_add(b[1]),
        a[2].wrapping_add(b[2]),
        a[3].wrapping_add(b[3]),
    ];
}

unsafe fn touch() {
    if touch_vdso != 0 {
        libc::gettimeofday(core::ptr::addr_of_mut!(tv), ptr::null_mut());
    }

    if touch_fp != 0 {
        fp += 0.1;
    }

    #[cfg(any(target_arch = "powerpc", target_arch = "powerpc64"))]
    {
        if touch_altivec != 0 {
            altivec_touch_fn();
        }
    }

    if touch_vector != 0 {
        vector_add_touch();
    }

    asm!(
        "# {0} {1} {2}",
        in(reg) core::ptr::addr_of!(tv),
        in(reg) core::ptr::addr_of!(fp),
        in(reg) core::ptr::addr_of!(c),
        options(nostack, preserves_flags)
    );
}

type ThreadFn = unsafe extern "C" fn(*mut c_void) -> *mut c_void;
type StartFn = unsafe fn(ThreadFn, *mut c_void, c_ulong);

unsafe fn start_thread_on(
    function: ThreadFn,
    arg: *mut c_void,
    cpu: c_ulong,
) {
    let mut rc: c_int;
    let mut tid: libc::pthread_t = core::mem::zeroed();
    let mut cpuset: libc::cpu_set_t = core::mem::zeroed();
    let mut attr: libc::pthread_attr_t = core::mem::zeroed();

    libc::CPU_ZERO(&mut cpuset);
    libc::CPU_SET(cpu as usize, &mut cpuset);

    rc = libc::pthread_attr_init(&mut attr);
    if rc != 0 {
        *libc::__errno_location() = rc;
        libc::perror(c"pthread_attr_init".as_ptr());
        libc::exit(1);
    }

    rc = libc::pthread_attr_setaffinity_np(
        &mut attr,
        core::mem::size_of::<libc::cpu_set_t>(),
        &cpuset,
    );
    if rc != 0 {
        *libc::__errno_location() = rc;
        libc::perror(c"pthread_attr_setaffinity_np".as_ptr());
        libc::exit(1);
    }

    rc = libc::pthread_create(&mut tid, &attr, Some(function), arg);
    if rc != 0 {
        *libc::__errno_location() = rc;
        libc::perror(c"pthread_create".as_ptr());
        libc::exit(1);
    }
}

unsafe fn start_process_on(
    function: ThreadFn,
    arg: *mut c_void,
    cpu: c_ulong,
) {
    let pid: c_int;
    let ncpus: c_int;
    let cpuset: *mut libc::cpu_set_t;
    let size: usize;

    pid = libc::fork();
    if pid == -1 {
        libc::perror(c"fork".as_ptr());
        libc::exit(1);
    }

    if pid != 0 {
        return;
    }

    ncpus = libc::get_nprocs();
    size = libc::CPU_ALLOC_SIZE(ncpus as usize);
    cpuset = libc::CPU_ALLOC(ncpus as usize);
    if cpuset.is_null() {
        libc::perror(c"malloc".as_ptr());
        libc::exit(1);
    }
    libc::CPU_ZERO_S(size, cpuset);
    libc::CPU_SET_S(cpu as usize, size, cpuset);

    if libc::sched_setaffinity(0, size, cpuset) != 0 {
        libc::perror(c"sched_setaffinity".as_ptr());
        libc::CPU_FREE(cpuset);
        libc::exit(1);
    }

    libc::CPU_FREE(cpuset);
    function(arg);

    libc::exit(0);
}

static mut iterations: c_ulong = 0;
static mut iterations_prev: c_ulong = 0;

unsafe extern "C" fn sigalrm_handler(_junk: c_int) {
    let i: c_ulong = iterations;

    libc::printf(c"%ld\n".as_ptr(), i.wrapping_sub(iterations_prev));
    iterations_prev = i;

    timeout = timeout.wrapping_sub(1);
    if timeout == 0 {
        libc::kill(0, libc::SIGUSR1);
    }

    libc::alarm(1);
}

unsafe extern "C" fn sigusr1_handler(_junk: c_int) {
    libc::exit(0);
}

#[repr(C)]
struct actions {
    setup: unsafe fn(c_int, c_int),
    thread1: ThreadFn,
    thread2: ThreadFn,
}

const READ: usize = 0;
const WRITE: usize = 1;

static mut pipe_fd1: [c_int; 2] = [0; 2];
static mut pipe_fd2: [c_int; 2] = [0; 2];

unsafe fn pipe_setup(_cpu1: c_int, _cpu2: c_int) {
    if libc::pipe(core::ptr::addr_of_mut!(pipe_fd1) as *mut c_int) != 0
        || libc::pipe(core::ptr::addr_of_mut!(pipe_fd2) as *mut c_int) != 0
    {
        libc::exit(1);
    }
}

unsafe extern "C" fn pipe_thread1(_arg: *mut c_void) -> *mut c_void {
    libc::signal(libc::SIGALRM, sigalrm_handler as usize);
    libc::alarm(1);

    loop {
        assert!(
            libc::read(pipe_fd1[READ], core::ptr::addr_of_mut!(c) as *mut c_void, 1) == 1
        );
        touch();

        assert!(
            libc::write(pipe_fd2[WRITE], core::ptr::addr_of!(c) as *const c_void, 1) == 1
        );
        touch();

        iterations = iterations.wrapping_add(2);
    }
}

unsafe extern "C" fn pipe_thread2(_arg: *mut c_void) -> *mut c_void {
    loop {
        assert!(
            libc::write(pipe_fd1[WRITE], core::ptr::addr_of!(c) as *const c_void, 1) == 1
        );
        touch();

        assert!(
            libc::read(pipe_fd2[READ], core::ptr::addr_of_mut!(c) as *mut c_void, 1) == 1
        );
        touch();
    }
}

static mut pipe_actions: actions = actions {
    setup: pipe_setup,
    thread1: pipe_thread1,
    thread2: pipe_thread2,
};

unsafe fn yield_setup(cpu1: c_int, cpu2: c_int) {
    if cpu1 != cpu2 {
        libc::fprintf(
            libc::stderr,
            c"Both threads must be on the same CPU for yield test\n".as_ptr(),
        );
        libc::exit(1);
    }
}

unsafe extern "C" fn yield_thread1(_arg: *mut c_void) -> *mut c_void {
    libc::signal(libc::SIGALRM, sigalrm_handler as usize);
    libc::alarm(1);

    loop {
        libc::sched_yield();
        touch();

        iterations = iterations.wrapping_add(2);
    }
}

unsafe extern "C" fn yield_thread2(_arg: *mut c_void) -> *mut c_void {
    loop {
        libc::sched_yield();
        touch();
    }
}

static mut yield_actions: actions = actions {
    setup: yield_setup,
    thread1: yield_thread1,
    thread2: yield_thread2,
};

unsafe fn sys_futex(
    addr1: *mut c_void,
    op: c_int,
    val1: c_int,
    timeout_arg: *mut libc::timespec,
    addr2: *mut c_void,
    val3: c_int,
) -> c_long {
    libc::syscall(
        libc::SYS_futex,
        addr1,
        op,
        val1,
        timeout_arg,
        addr2,
        val3,
    )
}

unsafe fn cmpxchg(p: *mut c_ulong, expected: c_ulong, desired: c_ulong) -> c_ulong {
    let mut exp = expected;
    let _ = core::sync::atomic::AtomicU64::from_ptr(p as *mut u64).compare_exchange(
        exp as u64,
        desired as u64,
        core::sync::atomic::Ordering::SeqCst,
        core::sync::atomic::Ordering::SeqCst,
    ).map_err(|old| exp = old as c_ulong);
    exp
}

unsafe fn xchg(p: *mut c_ulong, val: c_ulong) -> c_ulong {
    core::sync::atomic::AtomicU64::from_ptr(p as *mut u64)
        .swap(val as u64, core::sync::atomic::Ordering::SeqCst) as c_ulong
}

static mut processes: c_int = 0;

unsafe fn mutex_lock(m: *mut c_ulong) -> c_int {
    let mut c_val: c_int;
    let mut flags: c_int = libc::FUTEX_WAIT;
    if processes == 0 {
        flags |= libc::FUTEX_PRIVATE_FLAG;
    }

    c_val = cmpxchg(m, 0, 1) as c_int;
    if c_val == 0 {
        return 0;
    }

    if c_val == 1 {
        c_val = xchg(m, 2) as c_int;
    }

    while c_val != 0 {
        sys_futex(
            m as *mut c_void,
            flags,
            2,
            ptr::null_mut(),
            ptr::null_mut(),
            0,
        );
        c_val = xchg(m, 2) as c_int;
    }

    0
}

unsafe fn mutex_unlock(m: *mut c_ulong) -> c_int {
    let mut flags: c_int = libc::FUTEX_WAKE;
    if processes == 0 {
        flags |= libc::FUTEX_PRIVATE_FLAG;
    }

    if *m == 2 {
        *m = 0;
    } else if xchg(m, 0) == 1 {
        return 0;
    }

    sys_futex(
        m as *mut c_void,
        flags,
        1,
        ptr::null_mut(),
        ptr::null_mut(),
        0,
    );

    0
}

static mut m1: *mut c_ulong = ptr::null_mut();
static mut m2: *mut c_ulong = ptr::null_mut();

unsafe fn futex_setup(_cpu1: c_int, _cpu2: c_int) {
    if processes == 0 {
        static mut _m1: c_ulong = 0;
        static mut _m2: c_ulong = 0;
        m1 = core::ptr::addr_of_mut!(_m1);
        m2 = core::ptr::addr_of_mut!(_m2);
    } else {
        let shmid: c_int;
        let shmaddr: *mut c_void;

        shmid = libc::shmget(libc::IPC_PRIVATE, libc::getpagesize() as usize, libc::SHM_R | libc::SHM_W);
        if shmid < 0 {
            libc::perror(c"shmget".as_ptr());
            libc::exit(1);
        }

        shmaddr = libc::shmat(shmid, ptr::null(), 0);
        if shmaddr == (-1isize) as *mut c_void {
            libc::perror(c"shmat".as_ptr());
            libc::shmctl(shmid, libc::IPC_RMID, ptr::null_mut());
            libc::exit(1);
        }

        libc::shmctl(shmid, libc::IPC_RMID, ptr::null_mut());

        m1 = shmaddr as *mut c_ulong;
        m2 = (shmaddr as *mut u8).add(core::mem::size_of_val(&*m1)) as *mut c_ulong;
    }

    *m1 = 0;
    *m2 = 0;

    mutex_lock(m1);
    mutex_lock(m2);
}

unsafe extern "C" fn futex_thread1(_arg: *mut c_void) -> *mut c_void {
    libc::signal(libc::SIGALRM, sigalrm_handler as usize);
    libc::alarm(1);

    loop {
        mutex_lock(m2);
        mutex_unlock(m1);

        iterations = iterations.wrapping_add(2);
    }
}

unsafe extern "C" fn futex_thread2(_arg: *mut c_void) -> *mut c_void {
    loop {
        mutex_unlock(m2);
        mutex_lock(m1);
    }
}

static mut futex_actions: actions = actions {
    setup: futex_setup,
    thread1: futex_thread1,
    thread2: futex_thread2,
};

#[cfg(any(target_arch = "powerpc", target_arch = "powerpc64"))]
static mut options: [libc::option; 8] = [
    libc::option { name: c"test".as_ptr(), has_arg: libc::required_argument, flag: ptr::null_mut(), val: 't' as c_int },
    libc::option { name: c"process".as_ptr(), has_arg: libc::no_argument, flag: core::ptr::addr_of_mut!(processes), val: 1 },
    libc::option { name: c"timeout".as_ptr(), has_arg: libc::required_argument, flag: ptr::null_mut(), val: 's' as c_int },
    libc::option { name: c"vdso".as_ptr(), has_arg: libc::no_argument, flag: core::ptr::addr_of_mut!(touch_vdso), val: 1 },
    libc::option { name: c"no-fp".as_ptr(), has_arg: libc::no_argument, flag: core::ptr::addr_of_mut!(touch_fp), val: 0 },
    libc::option { name: c"no-altivec".as_ptr(), has_arg: libc::no_argument, flag: core::ptr::addr_of_mut!(touch_altivec), val: 0 },
    libc::option { name: c"no-vector".as_ptr(), has_arg: libc::no_argument, flag: core::ptr::addr_of_mut!(touch_vector), val: 0 },
    libc::option { name: ptr::null(), has_arg: 0, flag: ptr::null_mut(), val: 0 },
];

#[cfg(not(any(target_arch = "powerpc", target_arch = "powerpc64")))]
static mut options: [libc::option; 7] = [
    libc::option { name: c"test".as_ptr(), has_arg: libc::required_argument, flag: ptr::null_mut(), val: 't' as c_int },
    libc::option { name: c"process".as_ptr(), has_arg: libc::no_argument, flag: core::ptr::addr_of_mut!(processes), val: 1 },
    libc::option { name: c"timeout".as_ptr(), has_arg: libc::required_argument, flag: ptr::null_mut(), val: 's' as c_int },
    libc::option { name: c"vdso".as_ptr(), has_arg: libc::no_argument, flag: core::ptr::addr_of_mut!(touch_vdso), val: 1 },
    libc::option { name: c"no-fp".as_ptr(), has_arg: libc::no_argument, flag: core::ptr::addr_of_mut!(touch_fp), val: 0 },
    libc::option { name: c"no-vector".as_ptr(), has_arg: libc::no_argument, flag: core::ptr::addr_of_mut!(touch_vector), val: 0 },
    libc::option { name: ptr::null(), has_arg: 0, flag: ptr::null_mut(), val: 0 },
];

unsafe fn usage() {
    libc::fprintf(
        libc::stderr,
        c"Usage: context_switch2 <options> CPU1 CPU2\n\n".as_ptr(),
    );
    libc::fprintf(
        libc::stderr,
        c"\t\t--test=X\tpipe, futex or yield (default)\n".as_ptr(),
    );
    libc::fprintf(
        libc::stderr,
        c"\t\t--process\tUse processes (default threads)\n".as_ptr(),
    );
    libc::fprintf(
        libc::stderr,
        c"\t\t--timeout=X\tDuration in seconds to run (default 30)\n".as_ptr(),
    );
    libc::fprintf(libc::stderr, c"\t\t--vdso\t\ttouch VDSO\n".as_ptr());
    libc::fprintf(libc::stderr, c"\t\t--no-fp\t\tDon't touch FP\n".as_ptr());
    #[cfg(any(target_arch = "powerpc", target_arch = "powerpc64"))]
    libc::fprintf(
        libc::stderr,
        c"\t\t--no-altivec\tDon't touch altivec\n".as_ptr(),
    );
    libc::fprintf(
        libc::stderr,
        c"\t\t--no-vector\tDon't touch vector\n".as_ptr(),
    );
}

unsafe extern "C" {
    fn pick_online_cpu() -> c_int;
    fn have_hwcap(feature: c_ulong) -> c_int;
    static PPC_FEATURE_HAS_ALTIVEC: c_ulong;
    static PPC_FEATURE_HAS_VSX: c_ulong;
}

unsafe fn main_impl(argc: c_int, argv: *mut *mut c_char) -> c_int {
    let mut c_opt: i8;
    let mut actions_ptr: *mut actions = core::ptr::addr_of_mut!(yield_actions);
    let mut cpu1: c_int;
    let mut cpu2: c_int;
    let start_fn: StartFn;

    loop {
        let mut option_index: c_int = 0;

        c_opt = libc::getopt_long(
            argc,
            argv,
            c"".as_ptr(),
            core::ptr::addr_of_mut!(options) as *const libc::option,
            &mut option_index,
        ) as i8;

        if c_opt == -1 {
            break;
        }

        match c_opt as c_int {
            0 => {
                if options[option_index as usize].flag != ptr::null_mut() {
                    continue;
                }

                usage();
                libc::exit(1);
            }

            x if x == 't' as c_int => {
                if libc::strcmp(libc::optarg, c"pipe".as_ptr()) == 0 {
                    actions_ptr = core::ptr::addr_of_mut!(pipe_actions);
                } else if libc::strcmp(libc::optarg, c"yield".as_ptr()) == 0 {
                    actions_ptr = core::ptr::addr_of_mut!(yield_actions);
                } else if libc::strcmp(libc::optarg, c"futex".as_ptr()) == 0 {
                    actions_ptr = core::ptr::addr_of_mut!(futex_actions);
                } else {
                    usage();
                    libc::exit(1);
                }
            }

            x if x == 's' as c_int => {
                timeout = libc::atoi(libc::optarg) as u32;
            }

            _ => {
                usage();
                libc::exit(1);
            }
        }
    }

    if processes != 0 {
        start_fn = start_process_on;
    } else {
        start_fn = start_thread_on;
    }

    if argc - libc::optind != 2 {
        cpu1 = pick_online_cpu();
        cpu2 = cpu1;
    } else {
        cpu1 = libc::atoi(*argv.add(libc::optind as usize));
        libc::optind += 1;
        cpu2 = libc::atoi(*argv.add(libc::optind as usize));
        libc::optind += 1;
    }

    libc::printf(
        c"Using %s with ".as_ptr(),
        if processes != 0 {
            c"processes".as_ptr()
        } else {
            c"threads".as_ptr()
        },
    );

    if actions_ptr == core::ptr::addr_of_mut!(pipe_actions) {
        libc::printf(c"pipe".as_ptr());
    } else if actions_ptr == core::ptr::addr_of_mut!(yield_actions) {
        libc::printf(c"yield".as_ptr());
    } else {
        libc::printf(c"futex".as_ptr());
    }

    #[cfg(any(target_arch = "powerpc", target_arch = "powerpc64"))]
    {
        if have_hwcap(PPC_FEATURE_HAS_ALTIVEC) == 0 {
            touch_altivec = 0;
        }
    }

    if have_hwcap(PPC_FEATURE_HAS_VSX) == 0 {
        touch_vector = 0;
    }

    libc::printf(
        c" on cpus %d/%d touching FP:%s altivec:%s vector:%s vdso:%s\n".as_ptr(),
        cpu1,
        cpu2,
        if touch_fp != 0 { c"yes".as_ptr() } else { c"no".as_ptr() },
        #[cfg(any(target_arch = "powerpc", target_arch = "powerpc64"))]
        if touch_altivec != 0 { c"yes".as_ptr() } else { c"no".as_ptr() },
        #[cfg(not(any(target_arch = "powerpc", target_arch = "powerpc64")))]
        c"no".as_ptr(),
        if touch_vector != 0 { c"yes".as_ptr() } else { c"no".as_ptr() },
        if touch_vdso != 0 { c"yes".as_ptr() } else { c"no".as_ptr() },
    );

    /* Create a new process group so we can signal everyone for exit */
    libc::setpgid(libc::getpid(), libc::getpid());

    libc::signal(libc::SIGUSR1, sigusr1_handler as usize);

    ((*actions_ptr).setup)(cpu1, cpu2);

    start_fn((*actions_ptr).thread1, ptr::null_mut(), cpu1 as c_ulong);
    start_fn((*actions_ptr).thread2, ptr::null_mut(), cpu2 as c_ulong);

    loop {
        libc::sleep(3600);
    }
}

fn main() {
    unsafe {
        let mut args: Vec<*mut c_char> = std::env::args()
            .map(|arg| std::ffi::CString::new(arg).unwrap().into_raw())
            .collect();
        args.push(ptr::null_mut());
        main_impl((args.len() - 1) as c_int, args.as_mut_ptr());
    }
}
