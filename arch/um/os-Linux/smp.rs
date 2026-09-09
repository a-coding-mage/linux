// SPDX-License-Identifier: GPL-2.0
/*
 * Copyright (C) 2025 Ant Group
 * Author: Tiwei Bie <tiwei.btw@antgroup.com>
 */

// C dependencies: errno, pthread, signal, kern_util, um_malloc, init, os,
// smp, and internal interfaces are supplied by other translation units.

#[repr(C)]
pub struct cpu_thread_data {
    pub cpu: libc::c_int,
    pub sigset: libc::sigset_t,
}

static mut __curr_cpu: libc::c_int = 0;

pub unsafe fn uml_curr_cpu() -> libc::c_int {
    __curr_cpu
}

static mut cpu_threads: [libc::pthread_t; CONFIG_NR_CPUS] = [0; CONFIG_NR_CPUS];

unsafe extern "C" {
    static mut signals_enabled: libc::c_int;

    fn uml_start_secondary(arg: *mut libc::c_void);
    fn uml_kmalloc(size: libc::size_t, flags: libc::c_uint) -> *mut libc::c_void;
    fn kfree(ptr: *mut libc::c_void);
    fn panic(fmt: *const libc::c_char, ...);
    fn printk(fmt: *const libc::c_char, ...);
    fn fatal_sigsegv();
    fn pthread_sigqueue(thread: libc::pthread_t, sig: libc::c_int, value: libc::sigval) -> libc::c_int;
    fn um_trace_signals_off();
    fn um_trace_signals_on();
    fn uml_ipi_handler(vector: libc::c_int);
}

// Build-time constants supplied by the surrounding UML build.
const CONFIG_NR_CPUS: usize = 1;
const UM_GFP_ATOMIC: libc::c_uint = 0;
const IPI_SIGNAL: libc::c_int = 0;
const UM_KERN_ERR: *const libc::c_char = b"\0".as_ptr() as *const libc::c_char;

unsafe extern "C" fn cpu_thread(arg: *mut libc::c_void) -> *mut libc::c_void {
    let data = arg as *mut cpu_thread_data;

    __curr_cpu = (*data).cpu;

    uml_start_secondary(data as *mut libc::c_void);

    core::ptr::null_mut()
}

pub unsafe fn os_start_cpu_thread(cpu: libc::c_int) -> libc::c_int {
    let mut data: *mut cpu_thread_data;
    let mut sigset: libc::sigset_t = core::mem::zeroed();
    let mut oset: libc::sigset_t = core::mem::zeroed();
    let mut err: libc::c_int;

    data = uml_kmalloc(core::mem::size_of::<cpu_thread_data>(), UM_GFP_ATOMIC)
        as *mut cpu_thread_data;
    if data.is_null() {
        return -libc::ENOMEM;
    }

    libc::sigfillset(&mut sigset);
    if libc::sigprocmask(libc::SIG_SETMASK, &sigset, &mut oset) < 0 {
        err = *libc::__errno_location();
        kfree(data as *mut libc::c_void);
        return -err;
    }

    (*data).cpu = cpu;
    (*data).sigset = oset;

    err = libc::pthread_create(
        &mut cpu_threads[cpu as usize],
        core::ptr::null(),
        cpu_thread,
        data as *mut libc::c_void,
    );
    if libc::sigprocmask(libc::SIG_SETMASK, &oset, core::ptr::null_mut()) < 0 {
        panic(b"Failed to restore the signal mask, errno = %d\0".as_ptr() as _, *libc::__errno_location());
    }
    if err != 0 {
        kfree(data as *mut libc::c_void);
        return -err;
    }

    0
}

pub unsafe fn os_start_secondary(arg: *mut libc::c_void, switch_buf: *mut libc::jmp_buf) {
    let data = arg as *mut cpu_thread_data;

    libc::sigaddset(&mut (*data).sigset, IPI_SIGNAL);
    libc::sigaddset(&mut (*data).sigset, libc::SIGIO);

    if libc::sigprocmask(libc::SIG_SETMASK, &(*data).sigset, core::ptr::null_mut()) < 0 {
        panic(b"Failed to restore the signal mask, errno = %d\0".as_ptr() as _, *libc::__errno_location());
    }

    kfree(data as *mut libc::c_void);
    libc::longjmp(*switch_buf, 1);

    // unreachable
    printk(b"impossible long jump!\0".as_ptr() as _);
    fatal_sigsegv();
}

pub unsafe fn os_send_ipi(cpu: libc::c_int, vector: libc::c_int) -> libc::c_int {
    let value = libc::sigval { sival_int: vector };

    pthread_sigqueue(cpu_threads[cpu as usize], IPI_SIGNAL, value)
}

unsafe fn __local_ipi_set(enable: libc::c_int) {
    let mut sigset: libc::sigset_t = core::mem::zeroed();

    libc::sigemptyset(&mut sigset);
    libc::sigaddset(&mut sigset, IPI_SIGNAL);

    if libc::sigprocmask(
        if enable != 0 { libc::SIG_UNBLOCK } else { libc::SIG_BLOCK },
        &sigset,
        core::ptr::null_mut(),
    ) < 0 {
        panic(b"__local_ipi_set: sigprocmask failed, errno = %d\0".as_ptr() as _, *libc::__errno_location());
    }
}

pub unsafe fn os_local_ipi_enable() {
    __local_ipi_set(1);
}

pub unsafe fn os_local_ipi_disable() {
    __local_ipi_set(0);
}

unsafe extern "C" fn ipi_sig_handler(sig: libc::c_int, si: *mut libc::siginfo_t, _uc: *mut libc::c_void) {
    let save_errno = *libc::__errno_location();

    signals_enabled = 0;
    um_trace_signals_off();

    uml_ipi_handler((*si).si_value().sival_int);

    um_trace_signals_on();
    signals_enabled = 1;

    *libc::__errno_location() = save_errno;
}

pub unsafe fn os_init_smp() {
    let mut action: libc::sigaction = core::mem::zeroed();
    action.sa_sigaction = ipi_sig_handler as usize;
    action.sa_flags = libc::SA_SIGINFO | libc::SA_ONSTACK | libc::SA_RESTART;

    libc::sigfillset(&mut action.sa_mask);

    if libc::sigaction(IPI_SIGNAL, &action, core::ptr::null_mut()) < 0 {
        panic(b"os_init_smp: sigaction failed, errno = %d\0".as_ptr() as _, *libc::__errno_location());
    }

    cpu_threads[0] = libc::pthread_self();
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
