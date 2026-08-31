// SPDX-License-Identifier: GPL-2.0+

/*
 * Context switch microbenchmark.
 *
 * Copyright 2018, Anton Blanchard, IBM Corp.
 */

// C dependency intent: _GNU_SOURCE plus libc, linux futex, pthread, sched,
// signal, sys/shm, syscall, time, types, wait, and unistd interfaces.

use core::ffi::c_void;
use core::mem;
use core::ptr;

static mut TIMEOUT: libc::c_uint = 30;

unsafe fn set_cpu(cpu: libc::c_int) {
    let mut cpuset: libc::cpu_set_t = mem::zeroed();

    if cpu == -1 {
        return;
    }

    libc::CPU_ZERO(&mut cpuset);
    libc::CPU_SET(cpu as usize, &mut cpuset);

    if libc::sched_setaffinity(0, mem::size_of_val(&cpuset), &cpuset) != 0 {
        libc::perror(c"sched_setaffinity".as_ptr());
        libc::exit(1);
    }
}

unsafe fn start_process_on(
    func: unsafe extern "C" fn(*mut c_void) -> *mut c_void,
    arg: *mut c_void,
    cpu: libc::c_int,
) {
    let pid: libc::c_int;

    pid = libc::fork();
    if pid == -1 {
        libc::perror(c"fork".as_ptr());
        libc::exit(1);
    }

    if pid != 0 {
        return;
    }

    set_cpu(cpu);

    func(arg);

    libc::exit(0);
}

static mut CPU: libc::c_int = 0;
static mut DO_FORK: libc::c_int = 0;
static mut DO_VFORK: libc::c_int = 0;
static mut DO_EXEC: libc::c_int = 0;
static mut EXEC_FILE: *mut libc::c_char = ptr::null_mut();
static mut EXEC_TARGET: libc::c_int = 0;
static mut ITERATIONS: libc::c_ulong = 0;
static mut ITERATIONS_PREV: libc::c_ulong = 0;

unsafe fn run_exec() {
    let argv: [*mut libc::c_char; 2] = [
        c"./exec_target".as_ptr() as *mut libc::c_char,
        ptr::null_mut(),
    ];

    if libc::execve(
        c"./exec_target".as_ptr(),
        argv.as_ptr(),
        ptr::null(),
    ) == -1
    {
        libc::perror(c"execve".as_ptr());
        libc::exit(1);
    }
}

unsafe fn bench_fork() {
    loop {
        let mut pid: libc::pid_t = libc::fork();
        if pid == -1 {
            libc::perror(c"fork".as_ptr());
            libc::exit(1);
        }
        if pid == 0 {
            if DO_EXEC != 0 {
                run_exec();
            }
            libc::_exit(0);
        }
        pid = libc::waitpid(pid, ptr::null_mut(), 0);
        if pid == -1 {
            libc::perror(c"waitpid".as_ptr());
            libc::exit(1);
        }
        ITERATIONS = ITERATIONS.wrapping_add(1);
    }
}

unsafe fn bench_vfork() {
    loop {
        let mut pid: libc::pid_t = libc::vfork();
        if pid == -1 {
            libc::perror(c"fork".as_ptr());
            libc::exit(1);
        }
        if pid == 0 {
            if DO_EXEC != 0 {
                run_exec();
            }
            libc::_exit(0);
        }
        pid = libc::waitpid(pid, ptr::null_mut(), 0);
        if pid == -1 {
            libc::perror(c"waitpid".as_ptr());
            libc::exit(1);
        }
        ITERATIONS = ITERATIONS.wrapping_add(1);
    }
}

unsafe extern "C" fn null_fn(_arg: *mut c_void) -> *mut c_void {
    libc::pthread_exit(ptr::null_mut());
}

unsafe fn bench_thread() {
    let mut tid: libc::pthread_t = mem::zeroed();
    let mut cpuset: libc::cpu_set_t = mem::zeroed();
    let mut attr: libc::pthread_attr_t = mem::zeroed();
    let mut rc: libc::c_int;

    rc = libc::pthread_attr_init(&mut attr);
    if rc != 0 {
        *libc::__errno_location() = rc;
        libc::perror(c"pthread_attr_init".as_ptr());
        libc::exit(1);
    }

    if CPU != -1 {
        libc::CPU_ZERO(&mut cpuset);
        libc::CPU_SET(CPU as usize, &mut cpuset);

        rc = libc::pthread_attr_setaffinity_np(
            &mut attr,
            mem::size_of::<libc::cpu_set_t>(),
            &cpuset,
        );
        if rc != 0 {
            *libc::__errno_location() = rc;
            libc::perror(c"pthread_attr_setaffinity_np".as_ptr());
            libc::exit(1);
        }
    }

    loop {
        rc = libc::pthread_create(&mut tid, &attr, null_fn, ptr::null_mut());
        if rc != 0 {
            *libc::__errno_location() = rc;
            libc::perror(c"pthread_create".as_ptr());
            libc::exit(1);
        }
        rc = libc::pthread_join(tid, ptr::null_mut());
        if rc != 0 {
            *libc::__errno_location() = rc;
            libc::perror(c"pthread_join".as_ptr());
            libc::exit(1);
        }
        ITERATIONS = ITERATIONS.wrapping_add(1);
    }
}

unsafe extern "C" fn sigalrm_handler(_junk: libc::c_int) {
    let i: libc::c_ulong = ITERATIONS;

    libc::printf(c"%ld\n".as_ptr(), i.wrapping_sub(ITERATIONS_PREV));
    ITERATIONS_PREV = i;

    TIMEOUT = TIMEOUT.wrapping_sub(1);
    if TIMEOUT == 0 {
        libc::kill(0, libc::SIGUSR1);
    }

    libc::alarm(1);
}

unsafe extern "C" fn sigusr1_handler(_junk: libc::c_int) {
    libc::exit(0);
}

unsafe extern "C" fn bench_proc(_arg: *mut c_void) -> *mut c_void {
    libc::signal(libc::SIGALRM, sigalrm_handler as libc::sighandler_t);
    libc::alarm(1);

    if DO_FORK != 0 {
        bench_fork();
    } else if DO_VFORK != 0 {
        bench_vfork();
    } else {
        bench_thread();
    }

    ptr::null_mut()
}

static mut OPTIONS: [libc::option; 6] = [
    libc::option {
        name: c"fork".as_ptr(),
        has_arg: libc::no_argument,
        flag: &raw mut DO_FORK,
        val: 1,
    },
    libc::option {
        name: c"vfork".as_ptr(),
        has_arg: libc::no_argument,
        flag: &raw mut DO_VFORK,
        val: 1,
    },
    libc::option {
        name: c"exec".as_ptr(),
        has_arg: libc::no_argument,
        flag: &raw mut DO_EXEC,
        val: 1,
    },
    libc::option {
        name: c"timeout".as_ptr(),
        has_arg: libc::required_argument,
        flag: ptr::null_mut(),
        val: b's' as libc::c_int,
    },
    libc::option {
        name: c"exec-target".as_ptr(),
        has_arg: libc::no_argument,
        flag: &raw mut EXEC_TARGET,
        val: 1,
    },
    libc::option {
        name: ptr::null(),
        has_arg: 0,
        flag: ptr::null_mut(),
        val: 0,
    },
];

unsafe fn usage() {
    libc::fprintf(libc::stderr, c"Usage: fork <options> CPU\n\n".as_ptr());
    libc::fprintf(
        libc::stderr,
        c"\t\t--fork\tUse fork() (default threads)\n".as_ptr(),
    );
    libc::fprintf(
        libc::stderr,
        c"\t\t--vfork\tUse vfork() (default threads)\n".as_ptr(),
    );
    libc::fprintf(
        libc::stderr,
        c"\t\t--exec\tAlso exec() (default no exec)\n".as_ptr(),
    );
    libc::fprintf(
        libc::stderr,
        c"\t\t--timeout=X\tDuration in seconds to run (default 30)\n".as_ptr(),
    );
    libc::fprintf(
        libc::stderr,
        c"\t\t--exec-target\tInternal option for exec workload\n".as_ptr(),
    );
}

unsafe fn main_impl(argc: libc::c_int, argv: *mut *mut libc::c_char) -> libc::c_int {
    let mut c: libc::c_schar;

    loop {
        let mut option_index: libc::c_int = 0;

        c = libc::getopt_long(
            argc,
            argv,
            c"".as_ptr(),
            OPTIONS.as_ptr(),
            &mut option_index,
        ) as libc::c_schar;

        if c == -1 {
            break;
        }

        match c as libc::c_int {
            0 => {
                if OPTIONS[option_index as usize].flag != ptr::null_mut() {
                    continue;
                }

                usage();
                libc::exit(1);
            }

            x if x == b's' as libc::c_int => {
                TIMEOUT = libc::atoi(libc::optarg) as libc::c_uint;
            }

            _ => {
                usage();
                libc::exit(1);
            }
        }
    }

    if DO_FORK != 0 && DO_VFORK != 0 {
        usage();
        libc::exit(1);
    }
    if DO_EXEC != 0 && DO_FORK == 0 && DO_VFORK == 0 {
        usage();
        libc::exit(1);
    }

    if DO_EXEC != 0 {
        let dirname: *mut libc::c_char = libc::strdup(*argv.offset(0));
        let mut i: libc::c_int;
        i = (libc::strlen(dirname) - 1) as libc::c_int;
        while i != 0 {
            if *dirname.offset(i as isize) == b'/' as libc::c_char {
                *dirname.offset(i as isize) = b'\0' as libc::c_char;
                if libc::chdir(dirname) == -1 {
                    libc::perror(c"chdir".as_ptr());
                    libc::exit(1);
                }
                break;
            }
            i -= 1;
        }
    }

    if EXEC_TARGET != 0 {
        libc::exit(0);
    }

    if argc - libc::optind != 1 {
        CPU = -1;
    } else {
        CPU = libc::atoi(*argv.offset(libc::optind as isize));
        libc::optind += 1;
    }

    if DO_EXEC != 0 {
        EXEC_FILE = *argv.offset(0);
    }

    set_cpu(CPU);

    libc::printf(c"Using ".as_ptr());
    if DO_FORK != 0 {
        libc::printf(c"fork".as_ptr());
    } else if DO_VFORK != 0 {
        libc::printf(c"vfork".as_ptr());
    } else {
        libc::printf(c"clone".as_ptr());
    }

    if DO_EXEC != 0 {
        libc::printf(c" + exec".as_ptr());
    }

    libc::printf(c" on cpu %d\n".as_ptr(), CPU);

    /* Create a new process group so we can signal everyone for exit */
    libc::setpgid(libc::getpid(), libc::getpid());

    libc::signal(libc::SIGUSR1, sigusr1_handler as libc::sighandler_t);

    start_process_on(bench_proc, ptr::null_mut(), CPU);

    loop {
        libc::sleep(3600);
    }
}

fn main() {
    unsafe {
        let mut args: Vec<*mut libc::c_char> = std::env::args()
            .map(|arg| std::ffi::CString::new(arg).unwrap().into_raw())
            .collect();
        args.push(ptr::null_mut());

        let argc = (args.len() - 1) as libc::c_int;
        main_impl(argc, args.as_mut_ptr());
    }
}
