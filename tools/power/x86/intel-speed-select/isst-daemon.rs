// SPDX-License-Identifier: GPL-2.0
/*
 * Intel Speed Select -- Allow speed select to daemonize
 * Copyright (c) 2022 Intel Corporation.
 */

use libc::{
    c_char, c_int, c_void, mode_t, pid_t, sigaction, sigemptyset, sigaddset, sigprocmask,
    sigset_t, stat, time_t,
};

// C dependencies: stdio.h, stdlib.h, stdarg.h, string.h, unistd.h, fcntl.h,
// sys/file.h, sys/types.h, sys/stat.h, errno.h, getopt.h, signal.h, time.h,
// and "isst.h".

extern "C" {
    static mut stderr: *mut libc::FILE;

    fn fprintf(stream: *mut libc::FILE, format: *const c_char, ...) -> c_int;
    fn snprintf(s: *mut c_char, n: usize, format: *const c_char, ...) -> c_int;
    fn strlen(s: *const c_char) -> usize;
    fn exit(status: c_int) -> !;
    fn getppid() -> pid_t;
    fn getpid() -> pid_t;
    fn fork() -> pid_t;
    fn umask(mask: mode_t) -> mode_t;
    fn setsid() -> pid_t;
    fn getdtablesize() -> c_int;
    fn close(fd: c_int) -> c_int;
    fn open(pathname: *const c_char, flags: c_int, ...) -> c_int;
    fn dup(oldfd: c_int) -> c_int;
    fn chdir(path: *const c_char) -> c_int;
    fn fstat(fd: c_int, statbuf: *mut stat) -> c_int;
    fn flock(fd: c_int, operation: c_int) -> c_int;
    fn write(fd: c_int, buf: *const c_void, count: usize) -> isize;
    fn sigaction(signum: c_int, act: *const sigaction, oldact: *mut sigaction) -> c_int;
    fn sigprocmask(how: c_int, set: *const sigset_t, oldset: *mut sigset_t) -> c_int;
    fn signal(signum: c_int, handler: extern "C" fn(c_int)) -> usize;
    fn time(tloc: *mut time_t) -> time_t;
    fn sleep(seconds: libc::c_uint) -> libc::c_uint;

    fn debug_printf(format: *const c_char, ...);
    fn isst_get_ctdp_levels(id: *mut isst_id, pkg_dev: *mut isst_pkg_ctdp) -> c_int;
    fn isst_get_coremask_info(
        id: *mut isst_id,
        level: c_int,
        ctdp_level: *mut isst_pkg_ctdp_level_info,
    ) -> c_int;
    fn alloc_cpu_set(cpu_set: *mut *mut libc::cpu_set_t) -> usize;
    fn free_cpu_set(cpu_set: *mut libc::cpu_set_t);
    fn use_cgroupv2() -> c_int;
    fn enable_cpuset_controller() -> c_int;
    fn isolate_cpus(
        id: *mut isst_id,
        core_cpumask_size: usize,
        core_cpumask: *mut libc::cpu_set_t,
        current_level: c_int,
        arg: c_int,
    );
    fn get_topo_max_cpus() -> c_int;
    fn is_cpu_in_power_domain(cpu: c_int, id: *mut isst_id) -> c_int;
    fn CPU_ISSET_S(cpu: c_int, setsize: usize, set: *mut libc::cpu_set_t) -> c_int;
    fn set_cpu_online_offline(cpu: c_int, state: c_int) -> c_int;
    fn for_each_online_power_domain_in_set(
        cb: extern "C" fn(*mut isst_id, *mut c_void, *mut c_void, *mut c_void, *mut c_void),
        arg1: *mut c_void,
        arg2: *mut c_void,
        arg3: *mut c_void,
        arg4: *mut c_void,
    );
    fn hfi_exit();
    fn hfi_main() -> c_int;
}

#[repr(C)]
pub struct isst_id {
    pub cpu: c_int,
    pub pkg: c_int,
    pub die: c_int,
    pub punit: c_int,
}

#[repr(C)]
pub struct isst_pkg_ctdp {
    pub current_level: c_int,
    pub locked: c_int,
}

#[repr(C)]
pub struct isst_pkg_ctdp_level_info {
    pub core_cpumask_size: usize,
    pub core_cpumask: *mut libc::cpu_set_t,
    pub cpu_count: c_int,
}

static mut per_package_levels_info: [[[c_int; MAX_PUNIT_PER_DIE]; MAX_DIE_PER_PACKAGE]; MAX_PACKAGE_COUNT] =
    [[[0; MAX_PUNIT_PER_DIE]; MAX_DIE_PER_PACKAGE]; MAX_PACKAGE_COUNT];
static mut per_package_levels_tm: [[[time_t; MAX_PUNIT_PER_DIE]; MAX_DIE_PER_PACKAGE]; MAX_PACKAGE_COUNT] =
    [[[0; MAX_PUNIT_PER_DIE]; MAX_DIE_PER_PACKAGE]; MAX_PACKAGE_COUNT];

unsafe fn init_levels() {
    let mut i: c_int;
    let mut j: c_int;
    let mut k: c_int;

    i = 0;
    while i < MAX_PACKAGE_COUNT as c_int {
        j = 0;
        while j < MAX_DIE_PER_PACKAGE as c_int {
            k = 0;
            while k < MAX_PUNIT_PER_DIE as c_int {
                per_package_levels_info[i as usize][j as usize][k as usize] = -1;
                k += 1;
            }
            j += 1;
        }
        i += 1;
    }
}

#[no_mangle]
pub unsafe extern "C" fn process_level_change(id: *mut isst_id) {
    let mut ctdp_level: isst_pkg_ctdp_level_info = std::mem::zeroed();
    let mut pkg_dev: isst_pkg_ctdp = std::mem::zeroed();
    let mut tm: time_t;
    let mut ret: c_int;

    if (*id).pkg < 0 || (*id).die < 0 || (*id).punit < 0 {
        debug_printf(
            b"Invalid package/die info for cpu:%d\n\0".as_ptr() as *const c_char,
            (*id).cpu,
        );
        return;
    }

    tm = time(std::ptr::null_mut());
    if tm - per_package_levels_tm[(*id).pkg as usize][(*id).die as usize][(*id).punit as usize] < 2 {
        return;
    }

    per_package_levels_tm[(*id).pkg as usize][(*id).die as usize][(*id).punit as usize] = tm;

    ret = isst_get_ctdp_levels(id, &mut pkg_dev);
    if ret != 0 {
        debug_printf(b"Can't get tdp levels for cpu:%d\n\0".as_ptr() as *const c_char, (*id).cpu);
        return;
    }

    debug_printf(
        b"Get Config level %d pkg:%d die:%d current_level:%d\n\0".as_ptr() as *const c_char,
        (*id).cpu,
        (*id).pkg,
        (*id).die,
        pkg_dev.current_level,
    );

    if pkg_dev.locked != 0 {
        debug_printf(b"config TDP s locked \n\0".as_ptr() as *const c_char);
        return;
    }

    if per_package_levels_info[(*id).pkg as usize][(*id).die as usize][(*id).punit as usize]
        == pkg_dev.current_level
    {
        return;
    }

    debug_printf(
        b"**Config level change for cpu:%d pkg:%d die:%d from %d to %d\n\0".as_ptr()
            as *const c_char,
        (*id).cpu,
        (*id).pkg,
        (*id).die,
        per_package_levels_info[(*id).pkg as usize][(*id).die as usize][(*id).punit as usize],
        pkg_dev.current_level,
    );

    per_package_levels_info[(*id).pkg as usize][(*id).die as usize][(*id).punit as usize] =
        pkg_dev.current_level;

    ctdp_level.core_cpumask_size = alloc_cpu_set(&mut ctdp_level.core_cpumask);
    ret = isst_get_coremask_info(id, pkg_dev.current_level, &mut ctdp_level);
    if ret != 0 {
        free_cpu_set(ctdp_level.core_cpumask);
        debug_printf(b"Can't get core_mask:%d\n\0".as_ptr() as *const c_char, (*id).cpu);
        return;
    }

    if use_cgroupv2() != 0 {
        let ret: c_int;

        ret = enable_cpuset_controller();
        if ret != 0 {
            // goto use_offline;
        } else {
            isolate_cpus(
                id,
                ctdp_level.core_cpumask_size,
                ctdp_level.core_cpumask,
                pkg_dev.current_level,
                0,
            );

            free_cpu_set(ctdp_level.core_cpumask);
            return;
        }
    }

    // use_offline:
    if ctdp_level.cpu_count != 0 {
        let mut i: c_int;
        let max_cpus: c_int = get_topo_max_cpus();
        i = 0;
        while i < max_cpus {
            if is_cpu_in_power_domain(i, id) == 0 {
                i += 1;
                continue;
            }
            if CPU_ISSET_S(i, ctdp_level.core_cpumask_size, ctdp_level.core_cpumask) != 0 {
                fprintf(stderr, b"online cpu %d\n\0".as_ptr() as *const c_char, i);
                set_cpu_online_offline(i, 1);
            } else {
                fprintf(stderr, b"offline cpu %d\n\0".as_ptr() as *const c_char, i);
                set_cpu_online_offline(i, 0);
            }
            i += 1;
        }
    }

    // free_mask:
    free_cpu_set(ctdp_level.core_cpumask);
}

extern "C" fn _poll_for_config_change(
    id: *mut isst_id,
    _arg1: *mut c_void,
    _arg2: *mut c_void,
    _arg3: *mut c_void,
    _arg4: *mut c_void,
) {
    unsafe {
        process_level_change(id);
    }
}

unsafe fn poll_for_config_change() {
    for_each_online_power_domain_in_set(
        _poll_for_config_change,
        std::ptr::null_mut(),
        std::ptr::null_mut(),
        std::ptr::null_mut(),
        std::ptr::null_mut(),
    );
}

static mut done: c_int = 0;
static mut pid_file_handle: c_int = 0;

extern "C" fn signal_handler(sig: c_int) {
    unsafe {
        match sig {
            SIGINT | SIGTERM => {
                done = 1;
                hfi_exit();
                exit(0);
            }
            _ => {}
        }
    }
}

unsafe fn daemonize(rundir: *mut c_char, pidfile: *mut c_char) {
    let mut pid: c_int;
    let mut sid: c_int;
    let mut i: c_int;
    let mut str_buf: [c_char; 10] = [0; 10];
    let mut st: stat = std::mem::zeroed();
    let mut sig_actions: sigaction = std::mem::zeroed();
    let mut sig_set: sigset_t = std::mem::zeroed();
    let mut ret: c_int;

    if getppid() == 1 {
        return;
    }

    sigemptyset(&mut sig_set);
    sigaddset(&mut sig_set, SIGCHLD);
    sigaddset(&mut sig_set, SIGTSTP);
    sigaddset(&mut sig_set, SIGTTOU);
    sigaddset(&mut sig_set, SIGTTIN);
    sigprocmask(SIG_BLOCK, &sig_set, std::ptr::null_mut());

    sig_actions.sa_sigaction = signal_handler as usize;
    sigemptyset(&mut sig_actions.sa_mask);
    sig_actions.sa_flags = 0;

    sigaction(SIGHUP, &sig_actions, std::ptr::null_mut());
    sigaction(SIGTERM, &sig_actions, std::ptr::null_mut());
    sigaction(SIGINT, &sig_actions, std::ptr::null_mut());

    pid = fork();
    if pid < 0 {
        /* Could not fork */
        exit(EXIT_FAILURE);
    }
    if pid > 0 {
        exit(EXIT_SUCCESS);
    }

    umask(0o27);

    sid = setsid();
    if sid < 0 {
        exit(EXIT_FAILURE);
    }

    /* close all descriptors */
    i = getdtablesize();
    while i >= 0 {
        close(i);
        i -= 1;
    }

    i = open(b"/dev/null\0".as_ptr() as *const c_char, O_RDWR);
    if i < 0 {
        exit(EXIT_FAILURE);
    }

    ret = dup(i);
    if ret == -1 {
        exit(EXIT_FAILURE);
    }

    ret = chdir(rundir);
    if ret == -1 {
        exit(EXIT_FAILURE);
    }

    pid_file_handle = open(pidfile, O_RDWR | O_CREAT | O_NOFOLLOW, 0o600);
    if pid_file_handle == -1 {
        /* Couldn't open lock file */
        exit(1);
    }

    if fstat(pid_file_handle, &mut st) == -1 {
        exit(1);
    }

    if !S_ISREG(st.st_mode) {
        exit(1);
    }
    /* Try to lock file */
    // Original C condition:
    // #ifdef LOCKF_SUPPORT
    //     if (lockf(pid_file_handle, F_TLOCK, 0) == -1) {
    // #else
    if flock(pid_file_handle, LOCK_EX | LOCK_NB) < 0 {
        // #endif
        /* Couldn't get lock on lock file */
        fprintf(
            stderr,
            b"Couldn't get lock file %d\n\0".as_ptr() as *const c_char,
            getpid(),
        );
        exit(1);
    }
    snprintf(
        str_buf.as_mut_ptr(),
        str_buf.len(),
        b"%d\n\0".as_ptr() as *const c_char,
        getpid(),
    );
    ret = write(
        pid_file_handle,
        str_buf.as_ptr() as *const c_void,
        strlen(str_buf.as_ptr()),
    ) as c_int;
    if ret == -1 {
        exit(EXIT_FAILURE);
    }

    close(i);
}

#[no_mangle]
pub unsafe extern "C" fn isst_daemon(
    debug_mode: c_int,
    poll_interval: c_int,
    no_daemon: c_int,
) -> c_int {
    let mut ret: c_int;

    if no_daemon == 0 && poll_interval < 0 && debug_mode == 0 {
        fprintf(
            stderr,
            b"OOB mode is enabled and will run as daemon\n\0".as_ptr() as *const c_char,
        );
        daemonize(
            b"/tmp/\0".as_ptr() as *mut c_char,
            b"/tmp/hfi-events.pid\0".as_ptr() as *mut c_char,
        );
    } else {
        signal(SIGINT, signal_handler);
    }

    init_levels();

    if poll_interval < 0 {
        ret = hfi_main();
        if ret != 0 {
            fprintf(stderr, b"HFI initialization failed\n\0".as_ptr() as *const c_char);
        }
        fprintf(stderr, b"Must specify poll-interval\n\0".as_ptr() as *const c_char);
        return ret;
    }

    debug_printf(b"Starting loop\n\0".as_ptr() as *const c_char);
    while done == 0 {
        sleep(poll_interval as libc::c_uint);
        poll_for_config_change();
    }

    return 0;
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
